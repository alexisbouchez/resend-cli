use crate::api::contacts::{CreateContactRequest, UpdateContactRequest};
use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct ContactsCommand {
    #[command(subcommand)]
    pub command: ContactsSubcommand,
}

#[derive(Subcommand)]
pub enum ContactsSubcommand {
    /// Create a new contact
    Create {
        #[arg(short, long)]
        email: String,
        #[arg(long)]
        first_name: Option<String>,
        #[arg(long)]
        last_name: Option<String>,
        #[arg(long)]
        unsubscribed: Option<bool>,
    },
    /// List contacts
    List(crate::api::PaginationOptions),
    /// Get a single contact
    Get { id: String },
    /// Update a contact
    Update {
        id: String,
        #[arg(long)]
        first_name: Option<String>,
        #[arg(long)]
        last_name: Option<String>,
        #[arg(long)]
        unsubscribed: Option<bool>,
    },
    /// Delete a contact
    Delete { id: String },
    /// Add contact to segment
    AddToSegment {
        contact_id: String,
        segment_id: String,
    },
    /// Remove contact from segment
    RemoveFromSegment {
        contact_id: String,
        segment_id: String,
    },
}

use crate::api::ResendApi;

impl ContactsCommand {
    pub async fn execute<T: ResendApi + Send + Sync>(self, client: T) -> Result<()> {
        match self.command {
            ContactsSubcommand::Create {
                email,
                first_name,
                last_name,
                unsubscribed,
            } => {
                let request = CreateContactRequest {
                    email,
                    first_name,
                    last_name,
                    unsubscribed,
                    properties: None,
                };
                let contact = client.create_contact(request).await?;
                println!("Contact created successfully!");
                println!("{:#?}", contact);
            }
            ContactsSubcommand::List(pagination) => {
                let response = client.list_contacts(pagination).await?;
                println!("{:#?}", response.data);
            }
            ContactsSubcommand::Get { id } => {
                let contact = client.get_contact(&id).await?;
                println!("{:#?}", contact);
            }
            ContactsSubcommand::Update {
                id,
                first_name,
                last_name,
                unsubscribed,
            } => {
                let request = UpdateContactRequest {
                    first_name,
                    last_name,
                    unsubscribed,
                };
                let contact = client.update_contact(&id, request).await?;
                println!("Contact updated successfully!");
                println!("{:#?}", contact);
            }
            ContactsSubcommand::Delete { id } => {
                client.delete_contact(&id).await?;
                println!("Contact {} deleted successfully!", id);
            }
            ContactsSubcommand::AddToSegment {
                contact_id,
                segment_id,
            } => {
                client
                    .add_contact_to_segment(&contact_id, &segment_id)
                    .await?;
                println!(
                    "Contact {} added to segment {} successfully!",
                    contact_id, segment_id
                );
            }
            ContactsSubcommand::RemoveFromSegment {
                contact_id,
                segment_id,
            } => {
                client
                    .delete_contact_from_segment(&contact_id, &segment_id)
                    .await?;
                println!(
                    "Contact {} removed from segment {} successfully!",
                    contact_id, segment_id
                );
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::contacts::{Contact, ListContactsResponse};
    use crate::api::{MockResendApi, PaginationOptions};

    #[tokio::test]
    async fn test_list_contacts() {
        let mut mock = MockResendApi::new();

        mock.expect_list_contacts().returning(|_| {
            Ok(ListContactsResponse {
                data: vec![Contact {
                    id: "con_1".to_string(),
                    email: "test@example.com".to_string(),
                    first_name: Some("Test".to_string()),
                    last_name: Some("User".to_string()),
                    created_at: "2023-01-01".to_string(),
                    unsubscribed: false,
                }],
            })
        });

        let cmd = ContactsCommand {
            command: ContactsSubcommand::List(PaginationOptions::default()),
        };

        let result = cmd.execute(mock).await;
        assert!(result.is_ok());
    }
}
