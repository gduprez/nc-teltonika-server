use reqwest::Client;
use serde_json::json;
use std::env;

pub struct TeamsNotificationService;

impl TeamsNotificationService {
    const TELTONIKA_SERVER_TEAMS_WEBHOOK: &'static str = "https://nauticoncept.webhook.office.com/webhookb2/3e85c63b-47b3-40b5-aed1-38cd7782b8e3@0b54a401-b5bc-4c03-b505-f690c8c5de4b/IncomingWebhook/41a874a7910a4f09b71baadd3afdb46a/752cea04-da22-4bd2-bed1-a81f4b884fef/V2CTD5qoUoRXUQyO6fvocB37SikQRGLVmkdpXP2YO1SZs1";

    pub async fn note(title: &str, message: &str) {
        let env = env::var("APP_ENV").unwrap_or_default();
        let title_with_env = format!("({}) {}", env, title);
        
        let payload = json!({
            "@type": "MessageCard",
            "@context": "https://schema.org/extensions",
            "summary": title_with_env,
            "title": title_with_env,
            "sections": [{
                "text": message
            }]
        });

        let client = Client::new();
        let _ = client.post(Self::TELTONIKA_SERVER_TEAMS_WEBHOOK)
            .json(&payload)
            .send()
            .await;
    }

    pub async fn sql_error(sql: &str, error: &str) {
        let env = env::var("APP_ENV").unwrap_or_default();
        // Update to env contains dev
        if env.contains("dev") {
             println!("SQL Error: {}", error);
             return; 
        }

        let message = format!(
            "<b>Error message</b>:<br/>{}<br/><br/><b>Sql:</b><br/>{}<br/>",
            error, sql
        );
        
        Self::note("SQL ERROR", &message).await;
    }
}
