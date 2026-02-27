// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::error::Error;

use azure_core::http::StatusCode;
use azure_data_cosmos::CosmosClient;
use clap::{Args, Subcommand};

/// Reads a specific item.
#[derive(Clone, Args)]
pub struct ReadCommand {
    #[command(subcommand)]
    subcommands: Subcommands,
}

#[derive(Clone, Subcommand)]
enum Subcommands {
    Database {
        /// The database to read metadata for.
        database: String,
    },
    Container {
        /// The database containing the container.
        database: String,

        /// The container to read metadata for.
        container: String,
    },
    Item {
        /// The database containing the item.
        database: String,

        /// The container containing the item.
        container: String,

        /// The ID of the item.
        #[arg(long, short)]
        item_id: String,

        /// The partition key of the item.
        #[arg(long, short)]
        partition_key: String,
    },
}

impl ReadCommand {
    pub async fn run(self, client: CosmosClient) -> Result<(), Box<dyn Error>> {
        match self.subcommands {
            Subcommands::Item {
                database,
                container,
                item_id,
                partition_key,
            } => {
                let db_client = client.database_client(&database);
                let container_client = db_client.container_client(&container).await;

                let response = container_client
                    .read_item(&partition_key, &item_id, None)
                    .await;
                match response {
                    Err(e) if e.http_status() == Some(StatusCode::NotFound) => {
                        println!("Item not found!")
                    }
                    Ok(r) => {
                        let item: serde_json::Value = r.into_model()?;
                        println!("Found item:");
                        println!("{:#?}", item);
                    }
                    Err(e) => return Err(e.into()),
                };
                Ok(())
            }
            Subcommands::Database { database } => {
                let db_client = client.database_client(&database);
                let response = db_client.read(None).await?.into_model()?;
                println!("Database:");
                println!(" {:#?}", response);

                let resp = db_client.read_throughput(None).await?;

                match resp {
                    None => println!("Database does not have provisioned throughput"),
                    Some(throughput) => {
                        println!("Throughput:");
                        crate::utils::print_throughput(throughput);
                    }
                }
                Ok(())
            }
            Subcommands::Container {
                database,
                container,
            } => {
                let db_client = client.database_client(&database);
                let container_client = db_client.container_client(&container).await;
                let response = container_client.read(None).await?.into_model()?;
                println!("Container:");
                println!("  {:#?}", response);

                let resp = container_client.read_throughput(None).await?;

                match resp {
                    None => println!("Container does not have provisioned throughput"),
                    Some(throughput) => {
                        println!("Throughput:");
                        crate::utils::print_throughput(throughput);
                    }
                }
                Ok(())
            }
        }
    }
}
