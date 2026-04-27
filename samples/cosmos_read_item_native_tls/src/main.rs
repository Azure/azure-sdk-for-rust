// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_data_cosmos::{
    CosmosAccountEndpoint, CosmosAccountReference, CosmosClient, RoutingStrategy,
};
use azure_identity::DeveloperToolsCredential;
use clap::Parser;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let credential: Arc<dyn azure_core::credentials::TokenCredential> =
        DeveloperToolsCredential::new(None)?;
    let endpoint: CosmosAccountEndpoint = args.endpoint.parse()?;
    let account = CosmosAccountReference::with_credential(endpoint, credential);

    // No explicit TLS configuration is needed. Because we compiled reqwest
    // with the `native-tls` feature (and without `rustls`), the default
    // TLS backend is the platform's native TLS stack (Schannel on Windows,
    // Security Framework on macOS, OpenSSL on Linux).
    let client = CosmosClient::builder()
        .build(account, RoutingStrategy::ProximityTo(args.region.into()))
        .await?;

    let db_client = client.database_client(&args.database);
    let container_client = db_client.container_client(&args.container).await?;

    let response = container_client
        .read_item::<serde_json::Value>(&args.partition_key, &args.item_id, None)
        .await?;
    let item = response.into_model()?;
    println!("{}", serde_json::to_string_pretty(&item)?);

    Ok(())
}

#[derive(Parser)]
struct Args {
    /// Cosmos DB account endpoint (e.g. "https://myaccount.documents.azure.com/").
    #[arg(long, env = "AZURE_COSMOS_ENDPOINT")]
    endpoint: String,

    /// Azure region where the application is running (e.g. "East US").
    #[arg(long, env = "AZURE_COSMOS_REGION", default_value = "East US")]
    region: String,

    /// Database name.
    #[arg(long, env = "AZURE_COSMOS_DATABASE", default_value = "SampleDB")]
    database: String,

    /// Container name.
    #[arg(
        long,
        env = "AZURE_COSMOS_CONTAINER",
        default_value = "SampleContainer"
    )]
    container: String,

    /// ID of the item to read.
    #[arg(long, default_value = "1")]
    item_id: String,

    /// Partition key value of the item.
    #[arg(long, default_value = "sample-partition")]
    partition_key: String,
}
