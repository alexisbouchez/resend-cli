//! # API Keys API Module
//!
//! This module defines the data structures used for API key operations in the Resend API.
//! It includes request and response types for creating, retrieving, and managing API keys.

use serde::{Deserialize, Serialize};

/// Request structure for creating an API key
///
/// This struct contains the parameters needed to create an API key through the Resend API.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApiKeyRequest {
    /// Name for the API key
    pub name: String,
    /// Optional permission level for the API key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    /// Optional domain ID to restrict the API key to a specific domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
}

use tabled::Tabled;

/// Data structure representing an API key
///
/// This struct contains information about an API key in the Resend API.
#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct ApiKey {
    /// Unique identifier for the API key
    pub id: String,
    /// Name of the API key
    pub name: String,
    /// Creation timestamp of the API key
    pub created_at: String,
    /// Token value for the API key (excluded from table display)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[tabled(skip)]
    pub token: Option<String>,
}

/// Response structure for listing API keys
///
/// This struct contains a list of API keys from the Resend API.
#[derive(Debug, Serialize, Deserialize)]
pub struct ListApiKeysResponse {
    /// Array of API key objects
    pub data: Vec<ApiKey>,
}
