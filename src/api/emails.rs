//! # Email API Module
//!
//! This module defines the data structures used for email operations in the Resend API.
//! It includes request and response types for sending, retrieving, and managing emails.

use serde::{Deserialize, Serialize};
use tabled::Tabled;

/// Request structure for sending an email
///
/// This struct contains all the parameters needed to send an email through the Resend API.
#[derive(Debug, Serialize, Deserialize)]
pub struct SendEmailRequest {
    /// Sender's email address
    pub from: String,
    /// Recipient email addresses
    pub to: Vec<String>,
    /// Email subject line
    pub subject: String,
    /// HTML content of the email (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// Plain text content of the email (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Carbon copy recipients (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    /// Blind carbon copy recipients (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<String>>,
    /// Reply-to addresses (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<Vec<String>>,
    /// Scheduled delivery time for the email (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
}

/// Response structure for sending an email
///
/// This struct contains the response from the Resend API after sending an email.
#[derive(Debug, Serialize, Deserialize)]
pub struct SendEmailResponse {
    /// Unique identifier for the sent email
    pub id: String,
}

/// Data structure representing an email
///
/// This struct contains information about an email in the Resend API.
#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct Email {
    /// Unique identifier for the email
    pub id: String,
    /// Sender's email address
    pub from: String,
    /// Recipient email addresses
    #[tabled(display_with = "display_vec")]
    pub to: Vec<String>,
    /// Email subject line
    pub subject: String,
    /// Creation timestamp of the email
    pub created_at: String,
    /// Status of the last event for the email
    pub last_event: String,
}

/// Helper function to display vector values in tables
fn display_vec(v: &Vec<String>) -> String {
    v.join(", ")
}

/// Response structure for listing emails
///
/// This struct contains a list of emails from the Resend API.
#[derive(Debug, Serialize, Deserialize)]
pub struct ListEmailsResponse {
    /// Array of email objects
    pub data: Vec<Email>,
}

/// Request structure for updating an email
///
/// This struct contains parameters for updating a scheduled email.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEmailRequest {
    /// New scheduled delivery time for the email
    pub scheduled_at: String,
}

/// Data structure representing an email attachment
///
/// This struct contains information about an email attachment in the Resend API.
#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct Attachment {
    /// Unique identifier for the attachment
    pub id: String,
    /// Filename of the attachment
    pub filename: String,
    /// Size of the attachment in bytes
    pub size: u64,
    /// MIME type of the attachment
    pub content_type: String,
}

/// Response structure for listing email attachments
///
/// This struct contains a list of attachments for an email.
#[derive(Debug, Serialize, Deserialize)]
pub struct ListAttachmentsResponse {
    /// Array of attachment objects
    pub data: Vec<Attachment>,
}
