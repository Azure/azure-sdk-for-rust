// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_data_cosmos::{
    clients::{ContainerClientMethods, DatabaseClientMethods},
    CosmosClient, CosmosClientMethods, PartitionKey,
};
use clap::Parser;
use futures::StreamExt;

/// An example to show querying a Cosmos DB container.
#[derive(Parser)]
pub struct Args {
    /// The Cosmos DB endpoint to connect to.
    endpoint: String,

    /// The database to query.
    database: String,

    /// The container to query.
    container: String,

    /// The query to execute.
    #[clap(long, short)]
    query: String,

    /// The partition key to use when querying the container. Currently this only supports a single string partition key.
    #[clap(long, short)]
    partition_key: String,

    /// An authentication key to use when connecting to the Cosmos DB account. If omitted, the connection will use Entra ID.
    #[clap(long)]
    #[cfg(feature = "key_auth")]
    key: Option<String>,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    let client = create_client(&args);

    let db_client = client.database_client(&args.database);
    let container_client = db_client.container_client(&args.container);

    let pk = PartitionKey::from(args.partition_key);
    let mut items_pager =
        container_client.query_items::<serde_json::Value>(&args.query, pk, None)?;

    while let Some(page) = items_pager.next().await {
        let response = page?;
        println!("Results Page");
        println!("  Query Metrics: {:?}", response.query_metrics);
        println!("  Index Metrics: {:?}", response.index_metrics);
        println!("  Items:");
        for item in response.items {
            println!("    * {:?}", item);
        }
    }
    Ok(())
}

#[cfg(feature = "key_auth")]
fn create_client(args: &Args) -> CosmosClient {
    if let Some(key) = args.key.as_ref() {
        CosmosClient::with_key(&args.endpoint, key.clone(), None).unwrap()
    } else {
        let cred = azure_identity::create_default_credential().unwrap();
        CosmosClient::new(&args.endpoint, cred, None).unwrap()
    }
}

#[cfg(not(feature = "key_auth"))]
fn create_client(args: &Args) -> CosmosClient {
    let cred = azure_identity::create_default_credential().unwrap();
    CosmosClient::new(&args.endpoint, cred, None).unwrap()
}
