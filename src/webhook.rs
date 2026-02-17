use reqwest::Client;
use std::env;
use crate::notifications::TeamsNotificationService;

pub async fn send_webhook_to_nauticoncept_api() {
     let base_url = env::var("NAUTICONCEPT_API_URL").unwrap_or_default();
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
         Ok(_) => println!("✅ Sent webhook to Nauticoncept API"),
         Err(e) => {
             TeamsNotificationService::note("❌ Error sending webhook to Nauticoncept", &e.to_string()).await;
         }
     }
}
