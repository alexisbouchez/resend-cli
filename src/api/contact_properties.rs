use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateContactPropertyRequest {
    pub key: String,
    #[serde(rename = "type")]
    pub property_type: String,
    pub fallback_value: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateContactPropertyRequest {
    pub fallback_value: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactProperty {
    pub id: String,
    pub key: String,
    #[serde(rename = "type")]
    pub property_type: String,
    pub fallback_value: Option<serde_json::Value>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListContactPropertiesResponse {
    pub data: Vec<ContactProperty>,
}
