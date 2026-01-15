use crate::api::webhooks::CreateWebhookRequest;
use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct WebhooksCommand {
    #[command(subcommand)]
    pub command: WebhooksSubcommand,
}

#[derive(Subcommand)]
pub enum WebhooksSubcommand {
    /// Create a new webhook
    Create {
        #[arg(short, long)]
        endpoint: String,
        #[arg(short, long)]
        events: Vec<String>,
    },
    /// List webhooks
    List(crate::api::PaginationOptions),
    /// Get a single webhook
    Get { id: String },
    /// Delete a webhook
    Delete { id: String },
}

use crate::api::ResendApi;

impl WebhooksCommand {
    pub async fn execute<T: ResendApi + Send + Sync>(self, client: T) -> Result<()> {
        match self.command {
            WebhooksSubcommand::Create { endpoint, events } => {
                let request = CreateWebhookRequest { endpoint, events };
                let webhook = client.create_webhook(request).await?;
                println!("Webhook created successfully!");
                println!("{:#?}", webhook);
            }
            WebhooksSubcommand::List(pagination) => {
                let response = client.list_webhooks(pagination).await?;
                println!("{:#?}", response.data);
            }
            WebhooksSubcommand::Get { id } => {
                let webhook = client.get_webhook(&id).await?;
                println!("{:#?}", webhook);
            }
            WebhooksSubcommand::Delete { id } => {
                client.delete_webhook(&id).await?;
                println!("Webhook {} deleted successfully!", id);
            }
        }
        Ok(())
    }
}
