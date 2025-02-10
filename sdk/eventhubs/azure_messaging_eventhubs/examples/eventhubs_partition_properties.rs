// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
use azure_core::error::Result;
use azure_identity::DefaultAzureCredential;
use azure_messaging_eventhubs::{ProducerClient, ProducerClientOptions};

use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing subscriber from environment.
    tracing_subscriber::fmt().init();

    let host = env::var("EVENTHUBS_HOST").unwrap();
    let eventhub = env::var("EVENTHUB_NAME").unwrap();

    let credential = DefaultAzureCredential::new()?;

    let client = ProducerClient::new(
        host,
        eventhub.clone(),
        credential,
        Some(ProducerClientOptions {
            application_id: Some("test_get_properties".to_string()),
            ..Default::default()
        }),
    );
    let result = client.open().await;
    info!("Open result: {:?}", result);
    if result.is_err() {
        println!("Error opening client: {:?}", result.err());
        return Ok(());
    }
    let properties = client.get_eventhub_properties().await.unwrap();
    println!("Eventhub Properties for: {eventhub} {:?}", properties);

    for partition in properties.partition_ids.iter() {
        let partition_properties = client
            .get_partition_properties(partition.clone())
            .await
            .unwrap();
        println!(
            "Partition Properties for: {partition} {:?}",
            partition_properties
        );
    }
    Ok(())
}
