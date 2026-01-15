use crate::api::broadcasts::{CreateBroadcastRequest, UpdateBroadcastRequest};
use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct BroadcastsCommand {
    #[command(subcommand)]
    pub command: BroadcastsSubcommand,
}

#[derive(Subcommand)]
pub enum BroadcastsSubcommand {
    /// Create a new broadcast
    Create {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        segment_id: String,
        #[arg(short, long)]
        from: String,
        #[arg(short, long)]
        subject: String,
        #[arg(long)]
        html: Option<String>,
        #[arg(long)]
        text: Option<String>,
    },
    /// List broadcasts
    List(crate::api::PaginationOptions),
    /// Get a single broadcast
    Get { id: String },
    /// Update a broadcast
    Update {
        id: String,
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        segment_id: Option<String>,
        #[arg(short, long)]
        from: Option<String>,
        #[arg(short, long)]
        subject: Option<String>,
        #[arg(long)]
        html: Option<String>,
        #[arg(long)]
        text: Option<String>,
        #[arg(long)]
        reply_to: Option<Vec<String>>,
    },
    /// Delete a broadcast
    Delete { id: String },
    /// Send a broadcast
    Send { id: String },
}

use crate::api::ResendApi;

impl BroadcastsCommand {
    pub async fn execute<T: ResendApi + Send + Sync>(self, client: T) -> Result<()> {
        match self.command {
            BroadcastsSubcommand::Create {
                name,
                segment_id,
                from,
                subject,
                html,
                text,
            } => {
                let request = CreateBroadcastRequest {
                    name,
                    segment_id,
                    from,
                    subject,
                    html,
                    text,
                    reply_to: None,
                };
                let broadcast = client.create_broadcast(request).await?;
                println!("Broadcast created successfully!");
                println!("{:#?}", broadcast);
            }
            BroadcastsSubcommand::List(pagination) => {
                let response = client.list_broadcasts(pagination).await?;
                println!("{:#?}", response.data);
            }
            BroadcastsSubcommand::Get { id } => {
                let broadcast = client.get_broadcast(&id).await?;
                println!("{:#?}", broadcast);
            }
            BroadcastsSubcommand::Update {
                id,
                name,
                segment_id,
                from,
                subject,
                html,
                text,
                reply_to,
            } => {
                let request = UpdateBroadcastRequest {
                    name,
                    segment_id,
                    from,
                    subject,
                    html,
                    text,
                    reply_to,
                };
                let broadcast = client.update_broadcast(&id, request).await?;
                println!("Broadcast updated successfully!");
                println!("{:#?}", broadcast);
            }
            BroadcastsSubcommand::Delete { id } => {
                client.delete_broadcast(&id).await?;
                println!("Broadcast {} deleted successfully!", id);
            }
            BroadcastsSubcommand::Send { id } => {
                client.send_broadcast(&id).await?;
                println!("Broadcast {} sent successfully!", id);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{MockResendApi, PaginationOptions};
    use crate::api::broadcasts::{Broadcast, ListBroadcastsResponse};

    #[tokio::test]
    async fn test_create_broadcast() {
        let mut mock = MockResendApi::new();
        mock.expect_create_broadcast().returning(|_| Ok(Broadcast {
            id: "b_123".to_string(),
            name: Some("Test".to_string()),
            status: "draft".to_string(),
            created_at: "now".to_string(),
            segment_id: Some("s_123".to_string()),
        }));

        let cmd = BroadcastsCommand {
            command: BroadcastsSubcommand::Create {
                name: "Test".to_string(),
                segment_id: "s_123".to_string(),
                from: "me@example.com".to_string(),
                subject: "Sub".to_string(),
                html: None,
                text: None,
            },
        };
        assert!(cmd.execute(mock).await.is_ok());
    }

    #[tokio::test]
    async fn test_list_broadcasts() {
        let mut mock = MockResendApi::new();
        mock.expect_list_broadcasts().returning(|_| Ok(ListBroadcastsResponse { data: vec![] }));
        let cmd = BroadcastsCommand { command: BroadcastsSubcommand::List(PaginationOptions::default()) };
        assert!(cmd.execute(mock).await.is_ok());
    }

    #[tokio::test]
    async fn test_get_broadcast() {
        let mut mock = MockResendApi::new();
        mock.expect_get_broadcast().returning(|_| Ok(Broadcast {
            id: "b_123".to_string(),
            name: None,
            status: "sent".to_string(),
            created_at: "now".to_string(),
            segment_id: None,
        }));
        let cmd = BroadcastsCommand { command: BroadcastsSubcommand::Get { id: "b_123".to_string() } };
        assert!(cmd.execute(mock).await.is_ok());
    }

    #[tokio::test]
    async fn test_delete_broadcast() {
        let mut mock = MockResendApi::new();
        mock.expect_delete_broadcast().returning(|_| Ok(()));
        let cmd = BroadcastsCommand { command: BroadcastsSubcommand::Delete { id: "b_123".to_string() } };
        assert!(cmd.execute(mock).await.is_ok());
    }

    #[tokio::test]
    async fn test_send_broadcast() {
        let mut mock = MockResendApi::new();
        mock.expect_send_broadcast().returning(|_| Ok(()));
        let cmd = BroadcastsCommand { command: BroadcastsSubcommand::Send { id: "b_123".to_string() } };
        assert!(cmd.execute(mock).await.is_ok());
    }
}
