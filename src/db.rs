use sqlx::postgres::{PgPoolOptions, PgPool};
use std::env;
use std::process::{Command, Stdio};
use std::time::Duration;
use crate::parser::models::AvlRecord;
use crate::notifications::TeamsNotificationService;
use serde_json::json;

pub struct SshTunnel {
    child: std::process::Child,
}

impl Drop for SshTunnel {
    fn drop(&mut self) {
        println!("Stopping SSH tunnel...");
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

pub async fn init_db() -> Result<(PgPool, Option<SshTunnel>), sqlx::Error> {
    let mut tunnel = None;

    if let Ok(user) = env::var("SSH_USER") {
        if !user.is_empty() {
            let host = env::var("SSH_HOST").unwrap_or_default();
            let key_path = env::var("SSH_PRIVATE_KEY_PATH").unwrap_or_default();
            let local_port = env::var("DB_PORT").unwrap_or("5432".to_string());
            
            println!("Starting SSH tunnel to {}@{} on port {}...", user, host, local_port);
            
            // ssh -N -L local_port:127.0.0.1:5432 user@host -i key_path
            // Note: We use -o ExitOnForwardFailure=yes to fail if port is taken
            let child = Command::new("ssh")
                .arg("-N")
                .arg("-L")
                .arg(format!("{}:127.0.0.1:5432", local_port))
                .arg(format!("{}@{}", user, host))
                .arg("-i")
                .arg(&key_path)
                .arg("-o").arg("StrictHostKeyChecking=no") 
                .arg("-o").arg("ExitOnForwardFailure=yes")
                .stdout(Stdio::null())
                .stderr(Stdio::inherit()) 
                .spawn();

            if let Ok(c) = child {
                tunnel = Some(SshTunnel { child: c });
            }
            
            // Give it a moment to establish
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }

    let host = env::var("DB_HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("DB_PORT").unwrap_or("5432".to_string()).parse::<u16>().unwrap_or(5432);
    let user = env::var("DB_USER").unwrap_or_default();
    let password = env::var("DB_PASSWORD").unwrap_or_default();
    let db_name = env::var("DB_NAME").unwrap_or_default();

    println!("Connecting to DB at {}:{}/{} as {}...", host, port, db_name, user);

    let options = sqlx::postgres::PgConnectOptions::new()
        .host(&host)
        .port(port)
        .username(&user)
        .password(&password)
        .database(&db_name);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(options).await?;
        
    Ok((pool, tunnel))
}

pub struct TeltonikaDataRepo;

impl TeltonikaDataRepo {
    pub async fn save_avl_data(pool: &PgPool, imei: &str, data: &Vec<AvlRecord>, raw: &str, status: &str) {
        let sql = "INSERT INTO teltonika_data (imei, data, raw, created_at, status) VALUES ($1, $2, $3, $4, $5)";
        
        let json_data = json!(data);
        
        let res = sqlx::query(sql)
            .bind(imei)
            .bind(&json_data)
            .bind(raw)
            .bind(chrono::Utc::now())
            .bind(status)
            .execute(pool).await;
            
        if let Err(e) = res {
             let msg = format!("{:?}", e);
             TeamsNotificationService::sql_error(sql, &msg).await;
        }
    }
}
