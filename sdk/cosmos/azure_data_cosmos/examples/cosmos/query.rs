use std::error::Error;

use azure_data_cosmos::{CosmosClient, PartitionKey};
use clap::{Args, Subcommand};
use futures::StreamExt;

/// Run a single-partition query against a container.
#[derive(Clone, Args)]
pub struct QueryCommand {
    #[command(subcommand)]
    subcommand: Subcommands,
}

#[derive(Clone, Subcommand)]
enum Subcommands {
    Items {
        /// The database to query.
        database: String,

        /// The container to query.
        container: String,

        /// The query to execute.
        query: String,

        /// The partition key to use when querying the container. Currently this only supports a single string partition key.
        #[clap(long, short)]
        partition_key: String,
    },
    Databases {
        /// The query to execute.
        query: String,
    },
    Containers {
        /// The database to query.
        database: String,

        /// The query to execute.
        query: String,
    },
}

impl QueryCommand {
    pub async fn run(self, client: CosmosClient) -> Result<(), Box<dyn Error>> {
        match self.subcommand {
            Subcommands::Items {
                database,
                container,
                query,
                partition_key,
            } => {
                let db_client = client.database_client(&database);
                let container_client = db_client.container_client(&container);

                let pk = PartitionKey::from(&partition_key);
                let mut items =
                    container_client.query_items::<serde_json::Value>(&query, pk, None)?;

                while let Some(page) = items.next().await {
                    let page = page?.deserialize_body().await?;
                    println!("Results Page");
                    println!("  Items:");
                    for item in page.items {
                        println!("    * {:#?}", item);
                    }
                }
                Ok(())
            }
            Subcommands::Databases { query } => {
                let mut dbs = client.query_databases(query, None)?;

                while let Some(page) = dbs.next().await {
                    let page = page?.deserialize_body().await?;
                    println!("Results Page");
                    println!("  Databases:");
                    for item in page.databases {
                        println!("    * {:#?}", item);
                    }
                }
                Ok(())
            }
            Subcommands::Containers { database, query } => {
                let db_client = client.database_client(&database);
                let mut dbs = db_client.query_containers(query, None)?;

                while let Some(page) = dbs.next().await {
                    let page = page?.deserialize_body().await?;
                    println!("Results Page");
                    println!("  Containers:");
                    for item in page.containers {
                        println!("    * {:#?}", item);
                    }
                }
                Ok(())
            }
        }
    }
}
