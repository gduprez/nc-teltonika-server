mod parser;
mod db;
mod notifications;
mod utils;
mod webhook;
mod monitor;
pub mod config;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{timeout, Duration};
use tokio::sync::Semaphore;
use parser::{TeltonikaParser};
use db::{init_db, TeltonikaDataRepo};
use notifications::TeamsNotificationService;
use utils::format_record;
use webhook::send_webhook_to_nauticoncept_api;
use bytes::Bytes;
use std::env;
use std::sync::Arc;
use sqlx::PgPool;
use tracing::{info, error, debug};
use rlimit::{setrlimit, getrlimit, Resource};
use config::get_settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load env
    dotenvy::from_path(".env").ok();

    // Initialize config (will panic if fails, which is fine for startup)
    let settings = get_settings();

    // Initialize tracing with JSON support if requested
    if env::var("RUST_LOG_FORMAT").unwrap_or_default() == "json" {
        tracing_subscriber::fmt()
            .json()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .init();
    } else {
        tracing_subscriber::fmt::init();
    }
    
    let port = settings.server.port;
    
    // JS: 1 * 60 * 1000 = 60000 ms = 1 minute.
    let inactive_timeout_ms = 60000; 
    
    let (pool, _tunnel) = init_db().await?;
    let pool = Arc::new(pool);
    
    // Start Monitor Server (Health + Metrics)
    let monitor_port = settings.server.monitor_port;
    let monitor_pool = (*pool).clone();
    tokio::spawn(async move {
        monitor::start(monitor_port, monitor_pool).await;
    });

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    info!("Server started on port {}", port);

    // Set file descriptor limit
    let fd_limit: u64 = env::var("FILE_DESCRIPTOR_LIMIT")
        .unwrap_or("10000".to_string())
        .parse()
        .unwrap_or(10000);

    match setrlimit(Resource::NOFILE, fd_limit, fd_limit) {
        Ok(_) => info!("Successfully set file descriptor limit to {}", fd_limit),
        Err(e) => error!("Failed to set file descriptor limit to {}: {}", fd_limit, e),
    }

    // Log current file descriptor limit
    match getrlimit(Resource::NOFILE) {
        Ok((soft, hard)) => info!("File descriptor limit: soft={}, hard={}", soft, hard),
        Err(e) => error!("Failed to get file descriptor limit: {}", e),
    }
    
    let version = env!("CARGO_PKG_VERSION");
    TeamsNotificationService::note(&format!("nc-teltonika-server v{} started", version), "").await;
    
    // Connection Limit
    let max_connections = 5000;
    let connection_semaphore = Arc::new(Semaphore::new(max_connections));

    loop {
        // Wait for a permit if we are at limit (backpressure)
        let permit = match connection_semaphore.clone().acquire_owned().await {
            Ok(p) => p,
            Err(e) => {
                error!("Semaphore acquire error: {}", e);
                break;
            }
        };

        tokio::select! {
            res = listener.accept() => {
                match res {
                    Ok((socket, addr)) => {
                        debug!("client connected: {:?}", addr);
                        metrics::gauge!("tcp_connections_active").increment(1.0);
                        
                        let pool = pool.clone();
                        tokio::spawn(async move {
                            // Hold permit until task finishes
                            let _permit = permit;
                            handle_client(socket, addr, pool, inactive_timeout_ms).await;
                            metrics::gauge!("tcp_connections_active").decrement(1.0);
                        });
                    }
                    Err(e) => {
                         error!("Accept error: {}", e);
                         // Don't hold permit if accept failed
                        drop(permit);
                    }
                }
            }
            _ = tokio::signal::ctrl_c() => {
                info!("Received Ctrl+C, shutting down...");
                break;
            }
        }
    }

    Ok(())
}

async fn handle_client(mut socket: TcpStream, addr: std::net::SocketAddr, pool: Arc<PgPool>, timeout_ms: u64) {
    let mut imei = String::new();
    let mut buf = [0u8; 8192];
    let timeout_duration = Duration::from_millis(timeout_ms);

    loop {
        let read_res = timeout(timeout_duration, socket.read(&mut buf)).await;
        
        match read_res {
             Ok(Ok(n)) if n == 0 => {
                 debug!("client disconnected");
                 return;
             },
             Ok(Ok(n)) => {
                 metrics::counter!("packets_received_total").increment(1);
                 debug!("Received data from {}, length: {} bytes", addr, n);
                 debug!("{}", hex::encode(&buf[0..n]));
                 
                 let data = Bytes::copy_from_slice(&buf[0..n]);
                 
                 // Create parser
                 let parser = TeltonikaParser::new(data.clone());
                 
                 if parser.invalid {
                     debug!("❌ Invalid data received, closing connection");
                     return;
                 }
                 
                 if parser.is_imei {
                     if let Some(i) = parser.imei {
                         imei = i;
                         // Send ACK (0x01)
                         if let Err(_) = socket.write_all(&[1]).await {
                             return;
                         }
                     }
                 } else if let Some(avl) = parser.avl_data {
                     if avl.records.is_empty() {
                         return; // Close if no records? JS: `if (!avl || !avl.number_of_data) c.end()`
                     }
                     
                     for record in &avl.records {
                         debug!("{}", format_record(record));
                     }
                     
                     // DB Save
                     let start = std::time::Instant::now();
                     TeltonikaDataRepo::save_avl_data(&pool, &imei, &avl.records, &hex::encode(&data), "new").await;
                     metrics::histogram!("db_query_duration_seconds").record(start.elapsed().as_secs_f64());
                     
                     // Webhook
                     send_webhook_to_nauticoncept_api().await;
                     
                     // Send ACK: 4 bytes (Number of Data as Big Endian int32)
                     let count = avl.number_of_data as u32;
                     let ack = count.to_be_bytes();
                     if let Err(_) = socket.write_all(&ack).await {
                          return;
                     }
                     info!("✅ Sent ACK: {} record(s) to {}", count, addr);
                 }
             },
             Err(_) => {
                 debug!("Client timed out due to inactivity");
                 return;
             },
             Ok(Err(e)) => {
                 error!("Error reading from socket: {}", e);
                 return;
             }
        }
    }
}
