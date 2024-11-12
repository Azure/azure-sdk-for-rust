use std::error::Error;

use azure_core::StatusCode;
use azure_data_cosmos::{CosmosClient, PartitionKey};
use clap::{Args, Subcommand};

use crate::utils::ThroughputOptions;

/// Creates a new item.
#[derive(Clone, Args)]
pub struct ReplaceCommand {
    #[command(subcommand)]
    subcommand: Subcommands,
}

#[derive(Clone, Subcommand)]
pub enum Subcommands {
    Item {
        /// The database in which to create the item.
        database: String,

        /// The container in which to create the item.
        container: String,

        /// The ID of the item.
        #[clap(long, short)]
        item_id: String,

        /// The partition key of the new item.
        #[clap(long, short)]
        partition_key: String,

        /// The JSON of the new item.
        #[clap(long, short)]
        json: String,
    },
    DatabaseThroughput {
        /// The database to update throughput for.
        database: String,

        #[clap(flatten)]
        throughput_options: ThroughputOptions,
    },
    ContainerThroughput {
        /// The database containing the container.
        database: String,

        /// The container to update throughput for.
        container: String,

        #[clap(flatten)]
        throughput_options: ThroughputOptions,
    },
}

impl ReplaceCommand {
    pub async fn run(self, client: CosmosClient) -> Result<(), Box<dyn Error>> {
        match self.subcommand {
            Subcommands::Item {
                database,
                container,
                item_id,
                partition_key,
                json,
            } => {
                let db_client = client.database_client(&database);
                let container_client = db_client.container_client(&container);

                let pk = PartitionKey::from(&partition_key);
                let item: serde_json::Value = serde_json::from_str(&json)?;

                let response = container_client
                    .replace_item(pk, &item_id, item, None)
                    .await;
                match response {
                    Err(e) if e.http_status() == Some(StatusCode::NotFound) => {
                        println!("Item not found!")
                    }
                    Ok(r) => {
                        let item: serde_json::Value = r.deserialize_body().await?.unwrap();
                        println!("Replaced item:");
                        println!("{:#?}", item);
                    }
                    Err(e) => return Err(e.into()),
                };
                Ok(())
            }
            Subcommands::DatabaseThroughput {
                database,
                throughput_options,
            } => {
                let throughput_properties = throughput_options.try_into()?;
                let db_client = client.database_client(&database);
                let new_throughput = db_client
                    .replace_throughput(throughput_properties, None)
                    .await?
                    .deserialize_body()
                    .await?;
                println!("New Throughput: {:#?}", new_throughput);
                Ok(())
            }
            Subcommands::ContainerThroughput {
                database,
                container,
                throughput_options,
            } => {
                let throughput_properties = throughput_options.try_into()?;
                let db_client = client.database_client(&database);
                let container_client = db_client.container_client(&container);
                let new_throughput = container_client
                    .replace_throughput(throughput_properties, None)
                    .await?
                    .deserialize_body()
                    .await?;
                println!("New Throughput: {:#?}", new_throughput);
                Ok(())
            }
        }
    }
}
