use config::{Config, ConfigError, Environment};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub port: u16,
    pub http_port: u16,
    pub monitor_port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SshSettings {
    pub user: Option<String>,
    pub host: Option<String>,
    pub key_path: Option<String>,
    pub tunnel_port: Option<u16>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WebhookSettings {
    pub nauticoncept_url: String,
    pub teams_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub ssh: SshSettings,
    pub webhook: WebhookSettings,
    pub env: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env_run = env::var("APP_ENV").unwrap_or_else(|_| "development".into());

        let builder = Config::builder()
            // Start with default values (could be in a default.toml, but here we set via set_default if needed)
            // .add_source(File::with_name("config/default")) 
            // .add_source(File::with_name(&format!("config/{}", env_run)).required(false))
            // Load from Environment Variables
            // APP_SERVER__PORT=6000 maps to settings.server.port
            .add_source(Environment::default().separator("__").prefix("APP"))
            // Compatibility with existing flat Env Vars (Manual mapping)
            .set_default("server.port", env::var("HTTP_SERVER_PORT").unwrap_or("6000".into()))?
            .set_default("server.http_port", env::var("HTTP_SERVER_PORT").unwrap_or("6000".into()))? // Assuming same unless specified
            .set_default("server.monitor_port", env::var("MONITOR_PORT").unwrap_or("9090".into()))?
            
            .set_default("database.host", env::var("DB_HOST").unwrap_or("127.0.0.1".into()))?
            .set_default("database.port", env::var("DB_PORT").unwrap_or("5432".into()))?
            .set_default("database.user", env::var("DB_USER").unwrap_or("".into()))?
            .set_default("database.password", env::var("DB_PASSWORD").unwrap_or("".into()))?
            .set_default("database.name", env::var("DB_NAME").unwrap_or("".into()))?
            
            .set_default("ssh.user", env::var("SSH_USER").ok())?
            .set_default("ssh.host", env::var("SSH_HOST").ok())?
            .set_default("ssh.key_path", env::var("SSH_PRIVATE_KEY_PATH").ok())?
            // .set_default("ssh.tunnel_port", ...)?
            
             .set_default("webhook.nauticoncept_url", env::var("NAUTICONCEPT_API_URL").unwrap_or("".into()))?
             // Hardcoded fallback for now, but should be env var
             .set_default("webhook.teams_url", "https://nauticoncept.webhook.office.com/webhookb2/3e85c63b-47b3-40b5-aed1-38cd7782b8e3@0b54a401-b5bc-4c03-b505-f690c8c5de4b/IncomingWebhook/41a874a7910a4f09b71baadd3afdb46a/752cea04-da22-4bd2-bed1-a81f4b884fef/V2CTD5qoUoRXUQyO6fvocB37SikQRGLVmkdpXP2YO1SZs1")?
             
            .set_default("env", env_run)?;

        builder.build()?.try_deserialize()
    }
}

pub static CONFIG: std::sync::OnceLock<Settings> = std::sync::OnceLock::new();

pub fn get_settings() -> &'static Settings {
    CONFIG.get_or_init(|| Settings::new().expect("Failed to load configuration"))
}
