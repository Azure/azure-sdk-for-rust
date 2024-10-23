use std::error::Error;

use azure_core::StatusCode;
use azure_data_cosmos::CosmosClient;
use clap::{Args, Subcommand};

/// Deletes an item, database, or container.
#[derive(Clone, Args)]
pub struct DeleteCommand {
    #[command(subcommand)]
    subcommand: Subcommands,
}

#[derive(Clone, Subcommand)]
pub enum Subcommands {
    /// Delete an item in a container.
    Item {
        /// The database containing the item.
        database: String,

        /// The container containing the item.
        container: String,

        /// The ID of the item.
        #[clap(long, short)]
        item_id: String,

        /// The partition key of the item.
        #[clap(long, short)]
        partition_key: String,
    },

    /// Create a database (does not support Entra ID).
    Database {
        /// The ID of the database to delete.
        id: String,
    },

    /// Create a container (does not support Entra ID).
    Container {
        /// The ID of the database the container is in.
        database: String,

        /// The ID of the container to delete
        id: String,
    },
}

impl DeleteCommand {
    pub async fn run(self, client: CosmosClient) -> Result<(), Box<dyn Error>> {
        match self.subcommand {
            Subcommands::Item {
                database,
                container,
                item_id,
                partition_key,
            } => {
                let db_client = client.database_client(database);
                let container_client = db_client.container_client(container);

                let response = container_client
                    .delete_item(partition_key, item_id, None)
                    .await;
                match response {
                    Err(e) if e.http_status() == Some(StatusCode::NotFound) => {
                        println!("Item not found!")
                    }
                    Ok(_) => println!("Item deleted"),
                    Err(e) => return Err(e.into()),
                };
                Ok(())
            }

            Subcommands::Database { id } => {
                let db_client = client.database_client(id);
                db_client.delete(None).await?;
                Ok(())
            }

            Subcommands::Container { database, id } => {
                let db_client = client.database_client(database);
                let container_client = db_client.container_client(id);
                container_client.delete(None).await?;
                Ok(())
            }
        }
    }
}
