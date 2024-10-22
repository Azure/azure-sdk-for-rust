use std::error::Error;

use azure_data_cosmos::{
    models::{ContainerProperties, PartitionKeyDefinition},
    CosmosClient, PartitionKey,
};
use clap::{Args, Subcommand};

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
        #[clap(long, short)]
        partition_key: String,

        /// The JSON of the new item.
        #[clap(long, short)]
        json: String,
    },

    /// Create a database (does not support Entra ID).
    Database {
        /// The ID of the new database to create.
        id: String,
    },

    /// Create a container (does not support Entra ID).
    Container {
        /// The ID of the database to create the container in.
        database: String,

        /// The ID of the new container to create.
        #[clap(long, short)]
        id: Option<String>,

        /// The path to the partition key properties (supports up to 3).
        #[clap(long, short)]
        partition_key: Vec<String>,

        /// The JSON for a ContainerProperties value. The 'id' and 'partition key' options are ignored if this is set.
        #[clap(long)]
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
            } => {
                let db_client = client.database_client(database);
                let container_client = db_client.container_client(container);

                let pk = PartitionKey::from(&partition_key);
                let item: serde_json::Value = serde_json::from_str(&json)?;

                let created = container_client
                    .create_item(pk, item, None)
                    .await?
                    .deserialize_body()
                    .await?
                    .unwrap();
                println!("Created item:");
                println!("{:#?}", created);
                Ok(())
            }

            Subcommands::Database { id } => {
                let db = client
                    .create_database(id, None)
                    .await?
                    .deserialize_body()
                    .await?
                    .unwrap();
                println!("Created database:");
                println!("{:#?}", db);
                Ok(())
            }

            Subcommands::Container {
                database,
                id,
                partition_key,
                json,
            } => {
                let properties = match json {
                    Some(j) => serde_json::from_str(&j).unwrap(),
                    None => ContainerProperties {
                        id: id.expect("the ID is required when not using '--json'"),
                        partition_key: PartitionKeyDefinition::new(partition_key),
                        ..Default::default()
                    },
                };
                let container = client
                    .database_client(database)
                    .create_container(properties, None)
                    .await?
                    .deserialize_body()
                    .await?
                    .unwrap();
                println!("Created container:");
                println!("{:#?}", container);
                Ok(())
            }
        }
    }
}
