use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReceivedEmail {
    pub id: String,
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListReceivedEmailsResponse {
    pub data: Vec<ReceivedEmail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReceivedAttachment {
    pub id: String,
    pub filename: String,
    pub size: u64,
    pub content_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListReceivedAttachmentsResponse {
    pub data: Vec<ReceivedAttachment>,
}
