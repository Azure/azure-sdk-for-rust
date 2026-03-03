// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::error::Error;

use azure_data_cosmos::{CosmosClient, PartitionKey};
use clap::{Args, Subcommand};
use futures::TryStreamExt;

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

        /// The partition key to use when querying the container. Currently, this only supports a single string partition key.
        #[arg(long, short)]
        partition_key: Option<String>,
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
                let container_client = db_client.container_client(&container).await;

                let pk = match partition_key {
                    Some(pk) => PartitionKey::from(pk),
                    None => PartitionKey::EMPTY,
                };

                let mut items =
                    container_client.query_items::<serde_json::Value>(&query, pk, None)?;

                println!("Items:");
                while let Some(item) = items.try_next().await? {
                    println!("  * {:#?}", item);
                }
                Ok(())
            }
            Subcommands::Databases { query } => {
                let mut dbs = client.query_databases(query, None)?;

                println!("Databases:");
                while let Some(item) = dbs.try_next().await? {
                    println!("  * {:#?}", item);
                }
                Ok(())
            }
            Subcommands::Containers { database, query } => {
                let db_client = client.database_client(&database);
                let mut dbs = db_client.query_containers(query, None)?;

                println!("Containers:");
                while let Some(item) = dbs.try_next().await? {
                    println!("  * {:#?}", item);
                }
                Ok(())
            }
        }
    }
}
