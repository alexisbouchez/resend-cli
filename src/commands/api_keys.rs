use crate::api::api_keys::CreateApiKeyRequest;
use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct ApiKeysCommand {
    #[command(subcommand)]
    pub command: ApiKeysSubcommand,
}

#[derive(Subcommand)]
pub enum ApiKeysSubcommand {
    /// Create a new API key
    Create {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        permission: Option<String>,
        #[arg(short, long)]
        domain_id: Option<String>,
    },
    /// List API keys
    List(crate::api::PaginationOptions),
    /// Delete an API key
    Delete { id: String },
}

use crate::api::ResendApi;

impl ApiKeysCommand {
    pub async fn execute<T: ResendApi + Send + Sync>(self, client: T) -> Result<()> {
        match self.command {
            ApiKeysSubcommand::Create {
                name,
                permission,
                domain_id,
            } => {
                let request = CreateApiKeyRequest {
                    name,
                    permission,
                    domain_id,
                };
                let response = client.create_api_key(request).await?;
                println!("API Key created successfully!");
                println!("ID: {}", response.id);
                if let Some(token) = response.token {
                    println!("Token: {}", token);
                    println!("WARNING: This token is only shown once!");
                }
            }
            ApiKeysSubcommand::List(pagination) => {
                let response = client.list_api_keys(pagination).await?;
                crate::output::print_table(response.data);
            }
            ApiKeysSubcommand::Delete { id } => {
                client.delete_api_key(&id).await?;
                println!("API Key {} deleted successfully!", id);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::api_keys::{ApiKey, ListApiKeysResponse};
    use crate::api::{MockResendApi, PaginationOptions};

    #[tokio::test]
    async fn test_list_api_keys() {
        let mut mock = MockResendApi::new();

        mock.expect_list_api_keys().returning(|_| {
            Ok(ListApiKeysResponse {
                data: vec![ApiKey {
                    id: "key_1".to_string(),
                    name: "Test Key".to_string(),
                    created_at: "2023-01-01".to_string(),
                    token: None,
                }],
            })
        });

        let cmd = ApiKeysCommand {
            command: ApiKeysSubcommand::List(PaginationOptions::default()),
        };

        let result = cmd.execute(mock).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_api_key() {
        let mut mock = MockResendApi::new();

        mock.expect_create_api_key().returning(|_| {
            Ok(ApiKey {
                id: "new_key_id".to_string(),
                name: "New Test Key".to_string(),
                created_at: "2023-01-01".to_string(),
                token: Some("test_token_value".to_string()),
            })
        });

        let cmd = ApiKeysCommand {
            command: ApiKeysSubcommand::Create {
                name: "New Test Key".to_string(),
                permission: Some("full_access".to_string()),
                domain_id: Some("domain_123".to_string()),
            },
        };

        let result = cmd.execute(mock).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_api_key() {
        let mut mock = MockResendApi::new();

        mock.expect_delete_api_key().returning(|_| Ok(()));

        let cmd = ApiKeysCommand {
            command: ApiKeysSubcommand::Delete {
                id: "key_to_delete".to_string(),
            },
        };

        let result = cmd.execute(mock).await;
        assert!(result.is_ok());
    }
}
