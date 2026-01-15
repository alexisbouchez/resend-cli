use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWebhookRequest {
    pub endpoint: String,
    pub events: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Webhook {
    pub id: String,
    pub endpoint: Option<String>,
    pub created_at: Option<String>,
    pub signing_secret: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct ListWebhooksResponse {
    pub data: Vec<Webhook>,
}
