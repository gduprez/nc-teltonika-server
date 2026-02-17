use reqwest::Client;
use crate::notifications::TeamsNotificationService;
use crate::config::get_settings;
use tracing::info;

pub async fn send_webhook_to_nauticoncept_api() {
     let settings = get_settings();
     let base_url = &settings.webhook.nauticoncept_url;
     
     if base_url.is_empty() {
         return;
     }
     
     let url = format!("{}/modmessage-ttk/message-webhook", base_url);
     let client = Client::new();
     
     match client.post(&url)
         .header("Content-Type", "application/json")
         .json(&serde_json::json!({}))
         .send()
         .await 
     {
         Ok(_) => info!("✅ Sent webhook to Nauticoncept API"),
         Err(e) => {
             TeamsNotificationService::note("❌ Error sending webhook to Nauticoncept", &e.to_string()).await;
         }
     }
}
