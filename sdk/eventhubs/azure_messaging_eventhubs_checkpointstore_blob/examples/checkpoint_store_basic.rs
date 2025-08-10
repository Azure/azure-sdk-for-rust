// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Basic example showing how to create and use a blob checkpoint store.

use azure_identity::DeveloperToolsCredential;
use azure_messaging_eventhubs::{
    models::{Checkpoint, Ownership},
    CheckpointStore,
};
use azure_messaging_eventhubs_checkpointstore_blob::BlobCheckpointStore;
use azure_storage_blob::BlobContainerClient;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Configuration - replace with your actual values
    let storage_account_url = std::env::var("AZURE_STORAGE_BLOB_ENDPOINT")
        .unwrap_or_else(|_| "https://yourstorageaccount.blob.core.windows.net".to_string());

    let container =
        std::env::var("AZURE_STORAGE_CONTAINER").expect("Missing AZURE_STORAGE_CONTAINER");

    info!("Creating blob checkpoint store...");

    // Create Azure credential and blob service client
    let credential = DeveloperToolsCredential::new(None)?;

    let blob_container_client =
        BlobContainerClient::new(&storage_account_url, container, credential, None)?;

    // Create the checkpoint store
    let checkpoint_store = BlobCheckpointStore::new(blob_container_client);

    // Example Event Hub configuration
    let fully_qualified_namespace = "your-eventhubs-namespace.servicebus.windows.net";
    let eventhub_name = "my-eventhub";
    let consumer_group = "$Default";
    let partition_id = "0";
    let owner_id = "example-processor-1";

    info!("Testing ownership operations...");

    // Create and claim ownership
    let ownership = Ownership {
        fully_qualified_namespace: fully_qualified_namespace.to_string(),
        event_hub_name: eventhub_name.to_string(),
        consumer_group: consumer_group.to_string(),
        partition_id: partition_id.to_string(),
        owner_id: Some(owner_id.to_string()),
        ..Default::default()
    };

    let claimed_ownerships = checkpoint_store.claim_ownership(&[ownership]).await?;

    info!("Claimed {} ownerships", claimed_ownerships.len());

    // List all ownerships
    let ownerships = checkpoint_store
        .list_ownerships(fully_qualified_namespace, eventhub_name, consumer_group)
        .await?;

    info!("Found {} existing ownerships", ownerships.len());

    info!("Testing checkpoint operations...");

    // Create and update a checkpoint
    let mut checkpoint = Checkpoint {
        fully_qualified_namespace: fully_qualified_namespace.to_string(),
        event_hub_name: eventhub_name.to_string(),
        consumer_group: consumer_group.to_string(),
        partition_id: partition_id.to_string(),
        ..Default::default()
    };

    // Simulate processing some events
    // This is where you would normally process events from the Event Hub
    // For this example, we'll just update the checkpoint with some dummy data
    checkpoint.sequence_number = Some(42);
    checkpoint.offset = Some("100".to_string());

    // Save the checkpoint
    checkpoint_store.update_checkpoint(checkpoint).await?;

    // List all checkpoints
    let checkpoints = checkpoint_store
        .list_checkpoints(fully_qualified_namespace, eventhub_name, consumer_group)
        .await?;

    info!("Found {} existing checkpoints", checkpoints.len());

    for checkpoint in &checkpoints {
        info!(
            "Checkpoint for partition {}: sequence={:?}, offset={:?}",
            checkpoint.partition_id, checkpoint.sequence_number, checkpoint.offset,
        );
    }

    info!("Example completed successfully!");

    Ok(())
}
