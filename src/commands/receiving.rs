use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct ReceivingCommand {
    #[command(subcommand)]
    pub command: ReceivingSubcommand,
}

#[derive(Subcommand)]
pub enum ReceivingSubcommand {
    /// List received emails
    List(crate::api::PaginationOptions),
    /// Get a single received email
    Get { id: String },
    /// List attachments for a received email
    Attachments { id: String },
}

use crate::api::ResendApi;

impl ReceivingCommand {
    pub async fn execute<T: ResendApi + Send + Sync>(self, client: T) -> Result<()> {
        match self.command {
            ReceivingSubcommand::List(pagination) => {
                let response = client.list_received_emails(pagination).await?;
                println!("{:#?}", response.data);
            }
            ReceivingSubcommand::Get { id } => {
                let email = client.get_received_email(&id).await?;
                println!("{:#?}", email);
            }
            ReceivingSubcommand::Attachments { id } => {
                let response = client.list_received_attachments(&id).await?;
                println!("{:#?}", response.data);
            }
        }
        Ok(())
    }
}
