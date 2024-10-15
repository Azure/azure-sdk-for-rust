use std::error::Error;

use azure_data_cosmos::{
    clients::{ContainerClientMethods, DatabaseClientMethods},
    CosmosClient, CosmosClientMethods, PartitionKey,
};
use clap::{Args, Subcommand};

#[cfg(feature = "control_plane")]
use azure_data_cosmos::models::{ContainerDefinition, PartitionKeyDefinition};

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
    #[cfg(feature = "control_plane")]
    Database {
        /// The ID of the new database to create.
        id: String,
    },

    /// Create a container (does not support Entra ID).
    #[cfg(feature = "control_plane")]
    Container {
        /// The ID of the database to create the container in.
        database: String,

        /// The ID of the new container to create.
        id: String,

        /// The path to the partition key properties (supports up to 3).
        #[clap(long, short)]
        partition_key: Vec<String>,

        /// Optional indexing policy JSON to apply to the container.
        ///
        /// See https://learn.microsoft.com/en-us/azure/cosmos-db/index-policy for more.
        #[clap(long, short)]
        indexing_policy: Option<String>,
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

            #[cfg(feature = "control_plane")]
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

            #[cfg(feature = "control_plane")]
            Subcommands::Container {
                database,
                id,
                partition_key,
                indexing_policy,
            } => {
                let indexing_policy = indexing_policy.map(|s| serde_json::from_str(&s).unwrap());
                let partition_key = PartitionKeyDefinition::new(partition_key);
                let container_definition = ContainerDefinition {
                    id,
                    partition_key,
                    indexing_policy,
                };
                let container = client
                    .database_client(database)
                    .create_container(container_definition, None)
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
