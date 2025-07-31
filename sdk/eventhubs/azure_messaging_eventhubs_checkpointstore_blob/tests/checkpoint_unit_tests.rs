// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Unit tests for the blob checkpoint store models and utilities.

use azure_core::Result;
use azure_core_test::{recorded, Recording, TestContext};
use azure_messaging_eventhubs::CheckpointStore;
use azure_messaging_eventhubs_checkpointstore_blob::BlobCheckpointStore;
use azure_storage_blob::BlobContainerClient;
use std::sync::Arc;

fn create_test_checkpoint_store(recording: &Recording) -> Result<Arc<BlobCheckpointStore>> {
    let credential = recording.credential();
    let blob_container_client = BlobContainerClient::new(
        &recording.var("AZURE_STORAGE_BLOB_ENDPOINT", None),
        recording.var("AZURE_STORAGE_BLOB_CONTAINER", None),
        credential.clone(),
        None,
    )?;

    Ok(BlobCheckpointStore::new(blob_container_client))
}

#[recorded::test]
async fn list_checkpoints(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let checkpoint_store = create_test_checkpoint_store(recording)?;

    let namespace = recording.var("EVENTHUBS_HOST", None);
    let consumer_group = recording
        .var_opt("EVENTHUBS_CONSUMER_GROUP", None)
        .unwrap_or("$Default".to_string());
    let eventhub_name = recording.var("EVENTHUB_NAME", None);

    let checkpoints = checkpoint_store
        .list_checkpoints(&namespace, &eventhub_name, &consumer_group)
        .await?;
    assert_eq!(checkpoints.len(), 10);
    for (i, checkpoint) in checkpoints.iter().enumerate() {
        assert!(
            !checkpoint.partition_id.is_empty(),
            "Checkpoint {i} has an empty partition_id",
        );
        assert_eq!(checkpoint.partition_id, i.to_string());
        assert_eq!(checkpoint.fully_qualified_namespace, namespace);
        assert_eq!(checkpoint.consumer_group, consumer_group);
    }

    Ok(())
}

#[recorded::test]
async fn list_ownerships(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let checkpoint_store = create_test_checkpoint_store(recording)?;

    let namespace = recording.var("EVENTHUBS_HOST", None);
    let consumer_group = recording
        .var_opt("EVENTHUBS_CONSUMER_GROUP", None)
        .unwrap_or("$Default".to_string());
    let eventhub_name = recording.var("EVENTHUB_NAME", None);

    let ownerships = checkpoint_store
        .list_ownerships(&namespace, &eventhub_name, &consumer_group)
        .await?;
    assert_eq!(ownerships.len(), 10);
    for (i, ownership) in ownerships.iter().enumerate() {
        assert!(
            !ownership.partition_id.is_empty(),
            "Ownership {i} has an empty partition_id",
        );
        assert_eq!(ownership.partition_id, i.to_string());
        assert_eq!(ownership.fully_qualified_namespace, namespace);
        assert_eq!(ownership.consumer_group, consumer_group);
    }

    Ok(())
}
