// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Example showing how to use the blob checkpoint store with an Event Hubs processor.

use azure_core::http::Url;
use azure_identity::DeveloperToolsCredential;
use azure_messaging_eventhubs::{ConsumerClient, EventProcessor};
use azure_messaging_eventhubs_checkpointstore_blob::BlobCheckpointStore;
use azure_storage_blob::BlobServiceClient;
use std::env;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Configuration - replace with your actual values
    let storage_account_url = env::var("AZURE_STORAGE_ACCOUNT_URL")
        .expect("AZURE_STORAGE_ACCOUNT_URL environment variable must be set");
    let container_name = env::var("AZURE_STORAGE_BLOB_CONTAINER")
        .unwrap_or_else(|_| "eventhubs-checkpoints".to_string());
    let consumer_group =
        env::var("EVENTHUBS_CONSUMER_GROUP").unwrap_or_else(|_| "$Default".to_string());

    let eventhubs_connection_string =
        env::var("EVENTHUBS_CONNECTION_STRING").expect("EVENTHUBS_CONNECTION_STRING must be set");
    let eventhub_name = env::var("EVENTHUB_NAME").ok();

    info!("Setting up Event Hubs processor with blob checkpoint store...");

    // Create Azure credential and blob service client, then derive a container
    // client by name.
    let credential = DeveloperToolsCredential::new(None)?;
    let service_url = Url::parse(&storage_account_url)?;
    let service_client = BlobServiceClient::new(service_url, Some(credential.clone()), None)?;
    let blob_container_client = service_client.blob_container_client(&container_name);
    let consumer = ConsumerClient::builder()
        .with_application_id("ProcessorExample".to_string())
        .with_consumer_group(consumer_group)
        .open_with_connection_string(&eventhubs_connection_string, eventhub_name.as_deref())
        .await?;

    // Create the checkpoint store
    let checkpoint_store = BlobCheckpointStore::new(blob_container_client);

    // Create the event processor
    let processor = EventProcessor::builder()
        .build(consumer, checkpoint_store)
        .await?;

    info!("Starting event processor...");

    const TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);

    tokio::select! {
        result = processor.run() => {
            info!("Event processor finished: {:?}", result);
            if let Err(e) = result {
                info!("Event processor failed: {:?}", e);
            } else {
                info!("Event processor finished successfully");
            }
        }
        _ = tokio::time::sleep(TIMEOUT) => {
            info!("Timeout reached.");
        }
    }

    info!("Shutdown signal sent to event processor");
    let r = processor.shutdown().await;
    if let Err(e) = r {
        info!("Failed to shutdown event processor: {:?}", e);
    } else {
        info!("Event processor shutdown sent successfully");
    }

    info!("Event processor stopped.");

    Ok(())
}
