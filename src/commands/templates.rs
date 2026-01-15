use crate::api::templates::{CreateTemplateRequest, UpdateTemplateRequest};
use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Args)]
pub struct TemplatesCommand {
    #[command(subcommand)]
    pub command: TemplatesSubcommand,
}

#[derive(Subcommand)]
pub enum TemplatesSubcommand {
    /// Create a new template
    Create {
        #[arg(short, long)]
        name: String,
        #[arg(long)]
        html: String,
    },
    /// List templates
    List(crate::api::PaginationOptions),
    /// Get a single template
    Get { id: String },
    /// Update a template
    Update {
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        html: Option<String>,
    },
    /// Delete a template
    Delete { id: String },
}

use crate::api::ResendApi;

impl TemplatesCommand {
    pub async fn execute<T: ResendApi + Send + Sync>(self, client: T) -> Result<()> {
        match self.command {
            TemplatesSubcommand::Create { name, html } => {
                let request = CreateTemplateRequest { name, html };
                let template = client.create_template(request).await?;
                println!("Template created successfully!");
                println!("{:#?}", template);
            }
            TemplatesSubcommand::List(pagination) => {
                let response = client.list_templates(pagination).await?;
                println!("{:#?}", response.data);
            }
            TemplatesSubcommand::Get { id } => {
                let template = client.get_template(&id).await?;
                println!("{:#?}", template);
            }
            TemplatesSubcommand::Update { id, name, html } => {
                let request = UpdateTemplateRequest { name, html };
                let template = client.update_template(&id, request).await?;
                println!("Template updated successfully!");
                println!("{:#?}", template);
            }
            TemplatesSubcommand::Delete { id } => {
                client.delete_template(&id).await?;
                println!("Template {} deleted successfully!", id);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::templates::{ListTemplatesResponse, Template};
    use crate::api::{MockResendApi, PaginationOptions};

    #[tokio::test]
    async fn test_list_templates() {
        let mut mock = MockResendApi::new();

        mock.expect_list_templates().returning(|_| {
            Ok(ListTemplatesResponse {
                data: vec![Template {
                    id: "tpl_1".to_string(),
                    name: "Test Template".to_string(),
                    created_at: "2023-01-01".to_string(),
                }],
            })
        });

        let cmd = TemplatesCommand {
            command: TemplatesSubcommand::List(PaginationOptions::default()),
        };

        let result = cmd.execute(mock).await;
        assert!(result.is_ok());
    }
}
