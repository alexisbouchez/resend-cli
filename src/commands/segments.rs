use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct SegmentsCommand {
    #[command(subcommand)]
    pub command: SegmentsSubcommand,
}

#[derive(Subcommand)]
pub enum SegmentsSubcommand {
    /// Create a new segment
    Create {
        #[arg(short, long)]
        name: String,
    },
    /// List segments
    List(crate::api::PaginationOptions),
    /// Get a single segment
    Get { id: String },
    /// Delete a segment
    Delete { id: String },
}

use crate::api::ResendApi;

impl SegmentsCommand {
    pub async fn execute<T: ResendApi + Send + Sync>(self, client: T) -> Result<()> {
        match self.command {
            SegmentsSubcommand::Create { name } => {
                let segment = client.create_segment(&name).await?;
                println!("Segment created successfully!");
                println!("{:#?}", segment);
            }
            SegmentsSubcommand::List(pagination) => {
                let response = client.list_segments(pagination).await?;
                println!("{:#?}", response.data);
            }
            SegmentsSubcommand::Get { id } => {
                let segment = client.get_segment(&id).await?;
                println!("{:#?}", segment);
            }
            SegmentsSubcommand::Delete { id } => {
                client.delete_segment(&id).await?;
                println!("Segment {} deleted successfully!", id);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{MockResendApi, PaginationOptions};
    use crate::api::segments::{Segment, ListSegmentsResponse};

    #[tokio::test]
    async fn test_list_segments() {
        let mut mock = MockResendApi::new();
        
        mock.expect_list_segments()
            .returning(|_| Ok(ListSegmentsResponse {
                data: vec![Segment {
                    id: "seg_1".to_string(),
                    name: "Test Segment".to_string(),
                    created_at: "2023-01-01".to_string(),
                }]
            }));

        let cmd = SegmentsCommand {
            command: SegmentsSubcommand::List(PaginationOptions::default()),
        };

        let result = cmd.execute(mock).await;
        assert!(result.is_ok());
    }
}
