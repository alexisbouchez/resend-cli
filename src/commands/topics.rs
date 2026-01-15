use crate::api::topics::{CreateTopicRequest, UpdateTopicRequest};
use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct TopicsCommand {
    #[command(subcommand)]
    pub command: TopicsSubcommand,
}

#[derive(Subcommand)]
pub enum TopicsSubcommand {
    /// Create a new topic
    Create {
        #[arg(short, long)]
        name: String,
        #[arg(long, default_value = "opt_in")]
        default_subscription: String,
    },
    /// List topics
    List(crate::api::PaginationOptions),
    /// Get a single topic
    Get { id: String },
    /// Update a topic
    Update {
        id: String,
        #[arg(long)]
        name: Option<String>,
    },
    /// Delete a topic
    Delete { id: String },
}

use crate::api::ResendApi;

impl TopicsCommand {
    pub async fn execute<T: ResendApi + Send + Sync>(self, client: T) -> Result<()> {
        match self.command {
            TopicsSubcommand::Create {
                name,
                default_subscription,
            } => {
                let request = CreateTopicRequest {
                    name,
                    default_subscription,
                };
                let topic = client.create_topic(request).await?;
                println!("Topic created successfully!");
                println!("{:#?}", topic);
            }
            TopicsSubcommand::List(pagination) => {
                let response = client.list_topics(pagination).await?;
                println!("{:#?}", response.data);
            }
            TopicsSubcommand::Get { id } => {
                let topic = client.get_topic(&id).await?;
                println!("{:#?}", topic);
            }
            TopicsSubcommand::Update { id, name } => {
                let request = UpdateTopicRequest { name };
                let topic = client.update_topic(&id, request).await?;
                println!("Topic updated successfully!");
                println!("{:#?}", topic);
            }
            TopicsSubcommand::Delete { id } => {
                client.delete_topic(&id).await?;
                println!("Topic {} deleted successfully!", id);
            }
        }
        Ok(())
    }
}
