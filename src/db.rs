use sqlx::postgres::{PgPoolOptions, PgPool};
use std::process::{Command, Stdio};
use std::time::Duration;
use crate::parser::models::AvlRecord;
use crate::notifications::TeamsNotificationService;
use crate::config::get_settings;
use serde_json::json;
use tracing::info;

pub struct SshTunnel {
    child: std::process::Child,
}

impl Drop for SshTunnel {
    fn drop(&mut self) {
        info!("Stopping SSH tunnel...");
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

pub async fn init_db() -> Result<(PgPool, Option<SshTunnel>), sqlx::Error> {
    let mut tunnel = None;
    let settings = get_settings();

    if let Some(user) = &settings.ssh.user {
        if !user.is_empty() {
            let host = settings.ssh.host.as_deref().unwrap_or("");
            let key_path = settings.ssh.key_path.as_deref().unwrap_or("");
            let local_port = settings.database.port;
            
            info!("Starting SSH tunnel to {}@{} on port {}...", user, host, local_port);
            
            // ssh -N -L local_port:127.0.0.1:5432 user@host -i key_path
            let mut cmd = Command::new("ssh");
            cmd.arg("-N")
                .arg("-L")
                .arg(format!("{}:127.0.0.1:5432", local_port))
                .arg(format!("{}@{}", user, host));
            
            if !key_path.is_empty() {
                cmd.arg("-i").arg(key_path);
            }
                
            let child = cmd.arg("-o").arg("StrictHostKeyChecking=no") 
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

    let host = &settings.database.host;
    let port = settings.database.port;
    let user = &settings.database.user;
    let password = &settings.database.password;
    let db_name = &settings.database.name;

    info!("Connecting to DB at {}:{}/{} as {}...", host, port, db_name, user);

    let options = sqlx::postgres::PgConnectOptions::new()
        .host(host)
        .port(port)
        .username(user)
        .password(password)
        .database(db_name);

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

    pub async fn check_health(pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT 1").execute(pool).await.map(|_| ())
    }
}
