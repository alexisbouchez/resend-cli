//! # Domain Commands Module
//!
//! This module provides command-line interface functionality for managing domains
//! through the Resend API. It includes commands for creating, listing, getting,
//! deleting, and verifying domains.

use crate::api::domains::CreateDomainRequest;
use anyhow::Result;
use clap::{Args, Subcommand};

/// Command structure for domain-related operations
///
/// This struct represents the top-level domain command that contains
/// various subcommands for different domain operations.
#[derive(Args)]
pub struct DomainsCommand {
    #[command(subcommand)]
    pub command: DomainsSubcommand,
}

/// Subcommands for domain operations
///
/// This enum defines all the available domain-related subcommands,
/// including creating, listing, getting, deleting, and verifying domains.
#[derive(Subcommand)]
pub enum DomainsSubcommand {
    /// Create a new domain with the specified name and optional region
    Create {
        /// Name of the domain to create
        #[arg(short, long)]
        name: String,
        /// Optional region for the domain
        #[arg(short, long)]
        region: Option<String>,
    },
    /// List domains with optional pagination
    List(crate::api::PaginationOptions),
    /// Get a single domain by its ID
    Get {
        /// ID of the domain to retrieve
        id: String
    },
    /// Delete a domain by its ID
    Delete {
        /// ID of the domain to delete
        id: String
    },
    /// Verify a domain by its ID
    Verify {
        /// ID of the domain to verify
        id: String
    },
}

use crate::api::ResendApi;

impl DomainsCommand {
    /// Executes the domain command based on the selected subcommand
    ///
    /// This method handles the execution of different domain-related operations
    /// by dispatching to the appropriate API method based on the selected subcommand.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The API client type that implements the ResendApi trait
    ///
    /// # Arguments
    ///
    /// * `self` - The domain command with its selected subcommand
    /// * `client` - The API client to use for executing the command
    ///
    /// # Returns
    ///
    /// Ok(()) if the command executed successfully, or an error if the operation failed
    pub async fn execute<T: ResendApi + Send + Sync>(self, client: T) -> Result<()> {
        match self.command {
            DomainsSubcommand::Create { name, region } => {
                let request = CreateDomainRequest { name, region };
                let domain = client.create_domain(request).await?;
                println!("Domain created successfully!");
                println!("{:#?}", domain);
            }
            DomainsSubcommand::List(pagination) => {
                let response = client.list_domains(pagination).await?;
                crate::output::print_table(response.data);
            }
            DomainsSubcommand::Get { id } => {
                let domain = client.get_domain(&id).await?;
                println!("{:#?}", domain);
            }
            DomainsSubcommand::Delete { id } => {
                client.delete_domain(&id).await?;
                println!("Domain {} deleted successfully!", id);
            }
            DomainsSubcommand::Verify { id } => {
                client.verify_domain(&id).await?;
                println!("Verification process initiated for domain {}!", id);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{MockResendApi, PaginationOptions};
    use crate::api::domains::{Domain, ListDomainsResponse};

    #[tokio::test]
    async fn test_list_domains() {
        let mut mock = MockResendApi::new();

        mock.expect_list_domains()
            .returning(|_| Ok(ListDomainsResponse {
                data: vec![Domain {
                    id: "dom_1".to_string(),
                    name: "example.com".to_string(),
                    created_at: "2023-01-01".to_string(),
                    status: "verified".to_string(),
                    region: "us-east-1".to_string(),
                }]
            }));

        let cmd = DomainsCommand {
            command: DomainsSubcommand::List(PaginationOptions::default()),
        };

        let result = cmd.execute(mock).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_domain() {
        let mut mock = MockResendApi::new();

        mock.expect_create_domain()
            .returning(|_| Ok(Domain {
                id: "new_dom_id".to_string(),
                name: "newdomain.com".to_string(),
                created_at: "2023-01-01".to_string(),
                status: "not_verified".to_string(),
                region: "us-east-1".to_string(),
            }));

        let cmd = DomainsCommand {
            command: DomainsSubcommand::Create {
                name: "newdomain.com".to_string(),
                region: Some("us-west-2".to_string()),
            },
        };

        let result = cmd.execute(mock).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_domain() {
        let mut mock = MockResendApi::new();

        mock.expect_get_domain()
            .returning(|_| Ok(Domain {
                id: "dom_get_id".to_string(),
                name: "getdomain.com".to_string(),
                created_at: "2023-01-01".to_string(),
                status: "verified".to_string(),
                region: "eu-west-1".to_string(),
            }));

        let cmd = DomainsCommand {
            command: DomainsSubcommand::Get {
                id: "dom_get_id".to_string(),
            },
        };

        let result = cmd.execute(mock).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_domain() {
        let mut mock = MockResendApi::new();

        mock.expect_delete_domain()
            .returning(|_| Ok(()));

        let cmd = DomainsCommand {
            command: DomainsSubcommand::Delete {
                id: "dom_delete_id".to_string(),
            },
        };

        let result = cmd.execute(mock).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_verify_domain() {
        let mut mock = MockResendApi::new();

        mock.expect_verify_domain()
            .returning(|_| Ok(()));

        let cmd = DomainsCommand {
            command: DomainsSubcommand::Verify {
                id: "dom_verify_id".to_string(),
            },
        };

        let result = cmd.execute(mock).await;
        assert!(result.is_ok());
    }
}
