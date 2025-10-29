// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! Example of using the Event Hubs SDK to get partition properties.

use azure_identity::DeveloperToolsCredential;
use azure_messaging_eventhubs::ProducerClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize tracing subscriber from environment.
    tracing_subscriber::fmt().init();

    let host = env::var("EVENTHUBS_HOST").unwrap();
    let eventhub = env::var("EVENTHUB_NAME").unwrap();

    let credential = DeveloperToolsCredential::new(None)?;

    let result = ProducerClient::builder()
        .with_application_id("test_get_properties".to_string())
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await;

    if let Err(err) = result {
        println!("Error opening client: {:?}", err);
        return Ok(());
    }
    let client = result?;

    let properties = client.get_eventhub_properties().await.unwrap();
    println!("Eventhub Properties for: {eventhub} {:?}", properties);

    for partition in properties.partition_ids.iter() {
        let partition_properties = client.get_partition_properties(partition).await.unwrap();
        println!(
            "Partition Properties for: {partition} {:?}",
            partition_properties
        );
    }
    Ok(())
}
