mod parser;
mod db;
mod notifications;
mod utils;
mod webhook;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{timeout, Duration};
use parser::{TeltonikaParser};
use db::{init_db, TeltonikaDataRepo};
use notifications::TeamsNotificationService;
use utils::format_record;
use webhook::send_webhook_to_nauticoncept_api;
use bytes::Bytes;
use std::env;
use std::sync::Arc;
use sqlx::PgPool;
use rlimit::{getrlimit, setrlimit, Resource};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load env
    dotenvy::from_path(".env").ok();
    // Also try to load from ../.env as usually source is in src/
    // But working dir is rust_version/
    // The user's env file is mostly in root.
    // I'll assume they copy .env or run from root.
    
    let port = env::var("HTTP_SERVER_PORT").unwrap_or("6000".to_string());
    let debug = env::var("DEBUG").unwrap_or("false".to_string()) == "1";
    
    // JS: 1 * 60 * 1000 = 60000 ms = 1 minute.
    let inactive_timeout_ms = 60000; 
    
    let (pool, _tunnel) = init_db().await?;
    let pool = Arc::new(pool);
    
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    println!("Server started on port {}", port);

    // Set file descriptor limit
    let fd_limit: u64 = env::var("FILE_DESCRIPTOR_LIMIT")
        .unwrap_or("10000".to_string())
        .parse()
        .unwrap_or(10000);

    match setrlimit(Resource::NOFILE, fd_limit, fd_limit) {
        Ok(_) => println!("Successfully set file descriptor limit to {}", fd_limit),
        Err(e) => println!("Failed to set file descriptor limit to {}: {}", fd_limit, e),
    }

    // Log current file descriptor limit
    match getrlimit(Resource::NOFILE) {
        Ok((soft, hard)) => println!("File descriptor limit: soft={}, hard={}", soft, hard),
        Err(e) => println!("Failed to get file descriptor limit: {}", e),
    }
    
    let version = env!("CARGO_PKG_VERSION");
    TeamsNotificationService::note(&format!("nc-teltonika-server v{} started", version), "").await;
    
    loop {
        tokio::select! {
            res = listener.accept() => {
                match res {
                    Ok((socket, addr)) => {
                        if debug { println!("client connected: {:?}", addr); }
                        
                        let pool = pool.clone();
                        tokio::spawn(async move {
                            handle_client(socket, addr, pool, debug, inactive_timeout_ms).await;
                        });
                    }
                    Err(e) => println!("Accept error: {}", e),
                }
            }
            _ = tokio::signal::ctrl_c() => {
                println!("Received Ctrl+C, shutting down...");
                break;
            }
        }
    }

    Ok(())
}

async fn handle_client(mut socket: TcpStream, addr: std::net::SocketAddr, pool: Arc<PgPool>, debug: bool, timeout_ms: u64) {
    let mut imei = String::new();
    let mut buf = [0u8; 8192];
    let timeout_duration = Duration::from_millis(timeout_ms);

    loop {
        let read_res = timeout(timeout_duration, socket.read(&mut buf)).await;
        
        match read_res {
             Ok(Ok(n)) if n == 0 => {
                 println!("client disconnected");
                 return;
             },
             Ok(Ok(n)) => {
                 if debug {
                    println!("Received data from {}, length: {} bytes", addr, n);
                    println!("{}", hex::encode(&buf[0..n]));
                 }
                 
                 let data = Bytes::copy_from_slice(&buf[0..n]);
                 
                 // Create parser
                 let parser = TeltonikaParser::new(data.clone());
                 
                 if parser.invalid {
                     if debug { println!("❌ Invalid data received, closing connection"); }
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
                     
                     if debug {
                        for record in &avl.records {
                            println!("{}", format_record(record));
                        }
                     }
                     
                     // DB Save
                     TeltonikaDataRepo::save_avl_data(&pool, &imei, &avl.records, &hex::encode(&data), "new").await;
                     
                     // Webhook
                     send_webhook_to_nauticoncept_api().await;
                     
                     // Send ACK: 4 bytes (Number of Data as Big Endian int32)
                     let count = avl.number_of_data as u32;
                     let ack = count.to_be_bytes();
                     if let Err(_) = socket.write_all(&ack).await {
                          return;
                     }
                     println!("✅ Sent ACK: {} record(s) to {}", count, addr);
                 }
             },
             Err(_) => {
                 println!("Client timed out due to inactivity");
                 return;
             },
             Ok(Err(e)) => {
                 println!("Error reading from socket: {}", e);
                 return;
             }
        }
    }
}
