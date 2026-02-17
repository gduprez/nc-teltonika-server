use reqwest::Client;
use serde_json::json;
use tracing::error;
use crate::config::get_settings;

pub struct TeamsNotificationService;

impl TeamsNotificationService {
    // We could also move this URL to config, but for now leaving as constant or using config if available
    
    pub async fn note(title: &str, message: &str) {
        let settings = get_settings();
        let env = &settings.env;
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
        // Use config for URL if set, otherwise fallback to hardcoded (or just use config which has default)
        let url = &settings.webhook.teams_url;
        
        let _ = client.post(url)
            .json(&payload)
            .send()
            .await;
    }

    pub async fn sql_error(sql: &str, error: &str) {
        let settings = get_settings();
        let env = &settings.env;
        
        // Update to env contains dev
        if env.contains("dev") {
             error!("SQL Error: {}", error);
             return; 
        }

        let message = format!(
            "<b>Error message</b>:<br/>{}<br/><br/><b>Sql:</b><br/>{}<br/>",
            error, sql
        );
        
        Self::note("SQL ERROR", &message).await;
    }
}
