use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSegmentRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Segment {
    pub id: String,
    pub name: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListSegmentsResponse {
    pub data: Vec<Segment>,
}
