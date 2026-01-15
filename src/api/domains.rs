//! # Domain API Module
//!
//! This module defines the data structures used for domain operations in the Resend API.
//! It includes request and response types for creating, retrieving, and managing domains.

use serde::{Deserialize, Serialize};

/// Request structure for creating a domain
///
/// This struct contains the parameters needed to create a domain through the Resend API.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDomainRequest {
    /// Name of the domain to create
    pub name: String,
    /// Optional region for the domain (defaults to us-east-1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

use tabled::Tabled;

/// Data structure representing a domain
///
/// This struct contains information about a domain in the Resend API.
#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct Domain {
    /// Unique identifier for the domain
    pub id: String,
    /// Name of the domain
    pub name: String,
    /// Creation timestamp of the domain
    pub created_at: String,
    /// Current status of the domain (e.g., "not_verified", "verified")
    pub status: String,
    /// Region where the domain is hosted
    pub region: String,
}

/// Response structure for listing domains
///
/// This struct contains a list of domains from the Resend API.
#[derive(Debug, Serialize, Deserialize)]
pub struct ListDomainsResponse {
    /// Array of domain objects
    pub data: Vec<Domain>,
}
