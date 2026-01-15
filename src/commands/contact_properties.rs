use crate::api::contact_properties::{CreateContactPropertyRequest, UpdateContactPropertyRequest};
use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct ContactPropertiesCommand {
    #[command(subcommand)]
    pub command: ContactPropertiesSubcommand,
}

#[derive(Subcommand)]
pub enum ContactPropertiesSubcommand {
    /// Create a new contact property
    Create {
        #[arg(short, long)]
        key: String,
        #[arg(short, long, name = "type")]
        property_type: String,
        #[arg(long)]
        fallback_value: Option<String>,
    },
    /// List contact properties
    List(crate::api::PaginationOptions),
    /// Get a single contact property
    Get { id: String },
    /// Update a contact property
    Update {
        id: String,
        #[arg(long)]
        fallback_value: Option<String>,
    },
    /// Delete a contact property
    Delete { id: String },
}

use crate::api::ResendApi;

impl ContactPropertiesCommand {
    pub async fn execute<T: ResendApi + Send + Sync>(self, client: T) -> Result<()> {
        match self.command {
            ContactPropertiesSubcommand::Create {
                key,
                property_type,
                fallback_value,
            } => {
                let fallback = fallback_value.map(serde_json::Value::String);
                let request = CreateContactPropertyRequest {
                    key,
                    property_type,
                    fallback_value: fallback,
                };
                let property = client.create_contact_property(request).await?;
                println!("Contact property created successfully!");
                println!("{:#?}", property);
            }
            ContactPropertiesSubcommand::List(pagination) => {
                let response = client.list_contact_properties(pagination).await?;
                println!("{:#?}", response.data);
            }
            ContactPropertiesSubcommand::Get { id } => {
                let property = client.get_contact_property(&id).await?;
                println!("{:#?}", property);
            }
            ContactPropertiesSubcommand::Update { id, fallback_value } => {
                let fallback = fallback_value.map(serde_json::Value::String);
                let request = UpdateContactPropertyRequest {
                    fallback_value: fallback,
                };
                let property = client.update_contact_property(&id, request).await?;
                println!("Contact property updated successfully!");
                println!("{:#?}", property);
            }
            ContactPropertiesSubcommand::Delete { id } => {
                client.delete_contact_property(&id).await?;
                println!("Contact property {} deleted successfully!", id);
            }
        }
        Ok(())
    }
}
