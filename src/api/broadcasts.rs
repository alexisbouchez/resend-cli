use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBroadcastRequest {
    pub name: String,
    pub segment_id: String,
    pub from: String,
    pub subject: String,
    pub html: Option<String>,
    pub text: Option<String>,
    pub reply_to: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBroadcastRequest {
    pub name: Option<String>,
    pub segment_id: Option<String>,
    pub from: Option<String>,
    pub subject: Option<String>,
    pub html: Option<String>,
    pub text: Option<String>,
    pub reply_to: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Broadcast {
    pub id: String,
    pub name: Option<String>,
    pub status: String,
    pub created_at: String,
    pub segment_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListBroadcastsResponse {
    pub data: Vec<Broadcast>,
}
