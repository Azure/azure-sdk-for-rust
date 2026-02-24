use std::error::Error;

use azure_data_cosmos::{
    models::{ContainerProperties, PartitionKeyDefinition, ThroughputProperties},
    CosmosClient, CreateContainerOptions, CreateDatabaseOptions, ItemOptions, PartitionKey,
};
use clap::{Args, Subcommand};

use crate::utils::ThroughputOptions;

/// Creates a new item, database, or container.
#[derive(Clone, Args)]
pub struct CreateCommand {
    #[command(subcommand)]
    subcommand: Subcommands,
}

#[derive(Clone, Subcommand)]
pub enum Subcommands {
    /// Create an item in a container.
    Item {
        /// The database in which to create the item.
        database: String,

        /// The container in which to create the item.
        container: String,

        /// The partition key of the new item.
        #[arg(long, short)]
        partition_key: String,

        /// The JSON of the new item.
        #[arg(long, short)]
        json: String,

        /// If set, the updated item will be included in the response.
        #[arg(long)]
        show_updated: bool,
    },

    /// Create a database (does not support Entra ID).
    Database {
        /// The ID of the new database to create.
        id: String,

        #[command(flatten)]
        throughput_options: ThroughputOptions,
    },

    /// Create a container (does not support Entra ID).
    Container {
        /// The ID of the database to create the container in.
        database: String,

        #[command(flatten)]
        throughput_options: ThroughputOptions,

        /// The ID of the new container to create.
        #[arg(long, short)]
        id: Option<String>,

        /// The path to the partition key properties (supports up to 3).
        #[arg(long, short)]
        partition_key: Vec<String>,

        /// The JSON for a ContainerProperties value. The 'id' and 'partition key' options are ignored if this is set.
        #[arg(long)]
        json: Option<String>,
    },
}

impl CreateCommand {
    pub async fn run(self, client: CosmosClient) -> Result<(), Box<dyn Error>> {
        match self.subcommand {
            Subcommands::Item {
                database,
                container,
                partition_key,
                json,
                show_updated,
            } => {
                let db_client = client.database_client(&database);
                let container_client = db_client.container_client(&container).await;

                let pk = PartitionKey::from(&partition_key);
                let item: serde_json::Value = serde_json::from_str(&json)?;

                let options =
                    ItemOptions::default().with_content_response_on_write_enabled(show_updated);

                let response = container_client
                    .create_item(pk, item, Some(options))
                    .await?;

                println!("Created item successfully");

                if show_updated {
                    let created: serde_json::Value = response.into_body().json()?;
                    println!("Newly created item:");
                    println!("{:#?}", created);
                }
                Ok(())
            }

            Subcommands::Database {
                id,
                throughput_options,
            } => {
                let throughput_properties: Option<ThroughputProperties> =
                    throughput_options.try_into()?;
                let options = throughput_properties
                    .map(|p| CreateDatabaseOptions::default().with_throughput(p));

                let db = client.create_database(&id, options).await?.into_model()?;
                println!("Created database:");
                println!("{:#?}", db);
                Ok(())
            }

            Subcommands::Container {
                database,
                throughput_options,
                id,
                partition_key,
                json,
            } => {
                let throughput_properties: Option<ThroughputProperties> =
                    throughput_options.try_into()?;
                let options = throughput_properties
                    .map(|p| CreateContainerOptions::default().with_throughput(p));

                let properties = match json {
                    Some(j) => serde_json::from_str(&j).unwrap(),
                    None => {
                        if partition_key.is_empty() {
                            panic!("the partition key is required when not using '--json'");
                        }

                        if partition_key.len() > 3 {
                            panic!("only up to 3 partition key paths are supported");
                        }

                        ContainerProperties::new(
                            id.expect("the ID is required when not using '--json'"),
                            PartitionKeyDefinition::new(partition_key),
                        )
                    }
                };
                let container = client
                    .database_client(&database)
                    .create_container(properties, options)
                    .await?
                    .into_model()?;
                println!("Created container:");
                println!("{:#?}", container);
                Ok(())
            }
        }
    }
}
