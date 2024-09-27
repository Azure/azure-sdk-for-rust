// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_data_cosmos::{
    clients::{ContainerClientMethods, DatabaseClientMethods},
    CosmosClient, CosmosClientMethods,
};
use clap::Parser;

/// A simple example to show connecting to a Cosmos DB Account and retrieving the properties of a database.
#[derive(Parser)]
pub struct Args {
    /// The Cosmos DB endpoint to connect to.
    endpoint: String,

    /// The database to fetch information for.
    database: String,

    /// Optionally, the container to fetch information for.
    #[clap(long, short)]
    container: Option<String>,

    /// An authentication key to use when connecting to the Cosmos DB account. If omitted, the connection will use Entra ID.
    #[clap(long)]
    #[cfg(feature = "key_auth")]
    key: Option<String>,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    let client = create_client(&args)?;

    let db_client = client.database_client(&args.database);
    if let Some(container_name) = args.container {
        let container_client = db_client.container_client(container_name);
        let response = container_client
            .read(None)
            .await?
            .deserialize_body()
            .await?;
        println!("{:?}", response);
        return Ok(());
    } else {
        let response = db_client.read(None).await?.deserialize_body().await?;
        println!("{:?}", response);
    }
    Ok(())
}

#[cfg(feature = "key_auth")]
fn create_client(args: &Args) -> azure_core::Result<CosmosClient> {
    if let Some(key) = args.key.as_ref() {
        CosmosClient::with_key(&args.endpoint, key.clone(), None)
    } else {
        let cred = azure_identity::DefaultAzureCredential::new()?;
        CosmosClient::new(&args.endpoint, cred, None)
    }
}

#[cfg(not(feature = "key_auth"))]
fn create_client(args: &Args) -> azure_core::Result<CosmosClient> {
    let cred = azure_identity::DefaultAzureCredential::new()?;
    CosmosClient::new(&args.endpoint, cred, None)
}
