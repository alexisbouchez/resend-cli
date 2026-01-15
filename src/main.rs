//! # Resend CLI
//!
//! A command-line interface for interacting with the Resend API.
//! This tool allows users to manage emails, domains, API keys, and other resources
//! through the Resend service from the command line.
//!
//! ## Features
//!
//! - Send and manage emails
//! - Manage domains and verify ownership
//! - Create and manage API keys
//! - Handle contacts and segments
//! - Manage templates, topics, and webhooks
//! - Send and receive broadcast messages
//! - Manage contact properties

mod api;
mod commands;
mod config;
mod output;

use crate::api::ResendClient;
use crate::commands::api_keys::ApiKeysCommand;
use crate::commands::broadcasts::BroadcastsCommand;
use crate::commands::contact_properties::ContactPropertiesCommand;
use crate::commands::contacts::ContactsCommand;
use crate::commands::domains::DomainsCommand;
use crate::commands::emails::EmailsCommand;
use crate::commands::receiving::ReceivingCommand;
use crate::commands::segments::SegmentsCommand;
use crate::commands::templates::TemplatesCommand;
use crate::commands::topics::TopicsCommand;
use crate::commands::webhooks::WebhooksCommand;
use crate::config::Config;
use anyhow::Result;
use clap::{Parser, Subcommand};

/// Command-line interface parser for the Resend CLI
///
/// This struct defines the main CLI structure and handles parsing of command-line arguments.
/// It supports various subcommands for managing different aspects of the Resend service.
#[derive(Parser)]
#[command(name = "resend")]
#[command(about = "Resend CLI - Manage your emails, domains, and more", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Enum representing all available commands in the Resend CLI
///
/// Each variant corresponds to a specific functionality of the Resend API,
/// allowing users to perform various operations like sending emails, managing domains,
/// handling contacts, and more.
#[derive(Subcommand)]
enum Commands {
    /// Configure the Resend CLI with an API key
    Config {
        /// API key for authenticating with the Resend API
        #[arg(long)]
        api_key: String,
    },
    /// Manage emails - send, retrieve, list, cancel, and update emails
    Emails(EmailsCommand),
    /// Manage API keys - create, list, and delete API keys
    #[command(name = "api-keys")]
    ApiKeys(ApiKeysCommand),
    /// Manage domains - create, list, get, delete, and verify domains
    Domains(DomainsCommand),
    /// Manage segments - create, list, get, and delete segments
    Segments(SegmentsCommand),
    /// Manage contacts - create, list, get, update, and delete contacts
    Contacts(ContactsCommand),
    /// Manage templates - create, list, get, update, and delete email templates
    Templates(TemplatesCommand),
    /// Manage topics - create, list, get, update, and delete topics
    Topics(TopicsCommand),
    /// Manage webhooks - create, list, get, and delete webhooks
    Webhooks(WebhooksCommand),
    /// Manage broadcasts - create, list, get, update, delete, and send broadcasts
    Broadcasts(BroadcastsCommand),
    /// Manage contact properties - create, list, get, update, and delete contact properties
    #[command(name = "contact-properties")]
    ContactProperties(ContactPropertiesCommand),
    /// Manage received emails - list and retrieve received emails
    Receiving(ReceivingCommand),
}

/// Main entry point for the Resend CLI application
///
/// This function handles command-line argument parsing and routes the request
/// to the appropriate command handler. It manages configuration loading,
/// client initialization, and command execution.
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Config { api_key } => {
            let config = Config { api_key };
            config.save()?;
            println!("Configuration saved successfully!");
            return Ok(());
        }
        _ => {}
    }

    let config = Config::load()?;
    let client = ResendClient::new(config);

    match cli.command {
        Commands::Emails(cmd) => cmd.execute(client).await?,
        Commands::ApiKeys(cmd) => cmd.execute(client).await?,
        Commands::Domains(cmd) => cmd.execute(client).await?,
        Commands::Segments(cmd) => cmd.execute(client).await?,
        Commands::Contacts(cmd) => cmd.execute(client).await?,
        Commands::Templates(cmd) => cmd.execute(client).await?,
        Commands::Topics(cmd) => cmd.execute(client).await?,
        Commands::Webhooks(cmd) => cmd.execute(client).await?,
        Commands::Broadcasts(cmd) => cmd.execute(client).await?,
        Commands::ContactProperties(cmd) => cmd.execute(client).await?,
        Commands::Receiving(cmd) => cmd.execute(client).await?,
        Commands::Config { .. } => unreachable!(),
    }

    Ok(())
}
