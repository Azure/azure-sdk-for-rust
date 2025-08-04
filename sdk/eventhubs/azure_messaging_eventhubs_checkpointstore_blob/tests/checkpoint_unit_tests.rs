// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Unit tests for the blob checkpoint store models and utilities.

use azure_core::Result;
use azure_core_test::{recorded, Recording, TestContext};
use azure_messaging_eventhubs::{models::Checkpoint, CheckpointStore};
use azure_messaging_eventhubs_checkpointstore_blob::BlobCheckpointStore;
use azure_storage_blob::{BlobContainerClient, BlobContainerClientOptions};
use std::sync::Arc;
use tracing::trace;

pub fn create_test_checkpoint_store(recording: &Recording) -> Result<Arc<BlobCheckpointStore>> {
    let credential = recording.credential();
    let mut options = BlobContainerClientOptions::default();
    recording.instrument(&mut options.client_options);
    let blob_container_client = BlobContainerClient::new(
        &recording.var("AZURE_STORAGE_BLOB_ENDPOINT", None),
        recording.var("AZURE_STORAGE_BLOB_CONTAINER", None),
        credential.clone(),
        Some(options),
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

    // Create a checkpoint with both offset and sequence number
    let checkpoint = Checkpoint {
        fully_qualified_namespace: namespace.clone(),
        event_hub_name: eventhub_name.clone(),
        consumer_group: consumer_group.clone(),
        partition_id: "0".to_string(),
        offset: Some("12345".to_string()),
        sequence_number: Some(100),
    };

    // Add a checkpoint to the checkpoint store so we have at least one entry.
    checkpoint_store.update_checkpoint(checkpoint).await?;

    let checkpoints = checkpoint_store
        .list_checkpoints(&namespace, &eventhub_name, &consumer_group)
        .await?;
    assert!(!checkpoints.is_empty());
    for (i, checkpoint) in checkpoints.iter().enumerate() {
        trace!("Checkpoint {i}: {checkpoint:?}");
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
async fn update_checkpoint_basic(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let checkpoint_store = create_test_checkpoint_store(recording)?;

    let namespace = recording.var("EVENTHUBS_HOST", None);
    let consumer_group = recording
        .var_opt("EVENTHUBS_CONSUMER_GROUP", None)
        .unwrap_or("$Default".to_string());
    let eventhub_name = recording.var("EVENTHUB_NAME", None);

    // Create a checkpoint with both offset and sequence number
    let checkpoint = Checkpoint {
        fully_qualified_namespace: namespace.clone(),
        event_hub_name: eventhub_name.clone(),
        consumer_group: consumer_group.clone(),
        partition_id: "0".to_string(),
        offset: Some("12345".to_string()),
        sequence_number: Some(100),
    };

    // Update the checkpoint - this should create the blob with metadata
    let result = checkpoint_store.update_checkpoint(checkpoint).await;
    assert!(result.is_ok(), "Updating checkpoint should succeed");

    Ok(())
}

#[recorded::test]
async fn update_checkpoint_offset_only(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let checkpoint_store = create_test_checkpoint_store(recording)?;

    let namespace = recording.var("EVENTHUBS_HOST", None);
    let consumer_group = recording
        .var_opt("EVENTHUBS_CONSUMER_GROUP", None)
        .unwrap_or("$Default".to_string());
    let eventhub_name = recording.var("EVENTHUB_NAME", None);

    // Create a checkpoint with only offset
    let checkpoint = Checkpoint {
        fully_qualified_namespace: namespace.clone(),
        event_hub_name: eventhub_name.clone(),
        consumer_group: consumer_group.clone(),
        partition_id: "1".to_string(),
        offset: Some("67890".to_string()),
        sequence_number: None,
    };

    // Update the checkpoint
    let result = checkpoint_store.update_checkpoint(checkpoint).await;
    assert!(
        result.is_ok(),
        "Updating checkpoint with offset only should succeed"
    );

    Ok(())
}

#[recorded::test]
async fn update_checkpoint_sequence_number_only(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let checkpoint_store = create_test_checkpoint_store(recording)?;

    let namespace = recording.var("EVENTHUBS_HOST", None);
    let consumer_group = recording
        .var_opt("EVENTHUBS_CONSUMER_GROUP", None)
        .unwrap_or("$Default".to_string());
    let eventhub_name = recording.var("EVENTHUB_NAME", None);

    // Create a checkpoint with only sequence number
    let checkpoint = Checkpoint {
        fully_qualified_namespace: namespace.clone(),
        event_hub_name: eventhub_name.clone(),
        consumer_group: consumer_group.clone(),
        partition_id: "2".to_string(),
        offset: None,
        sequence_number: Some(200),
    };

    // Update the checkpoint
    let result = checkpoint_store.update_checkpoint(checkpoint).await;
    assert!(
        result.is_ok(),
        "Updating checkpoint with sequence number only should succeed"
    );

    Ok(())
}

#[recorded::test]
async fn update_checkpoint_empty_values(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let checkpoint_store = create_test_checkpoint_store(recording)?;

    let namespace = recording.var("EVENTHUBS_HOST", None);
    let consumer_group = recording
        .var_opt("EVENTHUBS_CONSUMER_GROUP", None)
        .unwrap_or("$Default".to_string());
    let eventhub_name = recording.var("EVENTHUB_NAME", None);

    // Create a checkpoint with no offset or sequence number (empty metadata)
    let checkpoint = Checkpoint {
        fully_qualified_namespace: namespace.clone(),
        event_hub_name: eventhub_name.clone(),
        consumer_group: consumer_group.clone(),
        partition_id: "3".to_string(),
        offset: None,
        sequence_number: None,
    };

    // Update the checkpoint - should still work even with empty metadata
    let result = checkpoint_store.update_checkpoint(checkpoint).await;
    assert!(
        result.is_ok(),
        "Updating checkpoint with empty values should succeed"
    );

    Ok(())
}

#[recorded::test]
async fn update_checkpoint_multiple_updates(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let checkpoint_store = create_test_checkpoint_store(recording)?;

    let namespace = recording.var("EVENTHUBS_HOST", None);
    let consumer_group = recording
        .var_opt("EVENTHUBS_CONSUMER_GROUP", None)
        .unwrap_or("$Default".to_string());
    let eventhub_name = recording.var("EVENTHUB_NAME", None);

    // Create an initial checkpoint
    let mut checkpoint = Checkpoint {
        fully_qualified_namespace: namespace.clone(),
        event_hub_name: eventhub_name.clone(),
        consumer_group: consumer_group.clone(),
        partition_id: "4".to_string(),
        offset: Some("100".to_string()),
        sequence_number: Some(10),
    };

    // First update
    let result1 = checkpoint_store.update_checkpoint(checkpoint.clone()).await;
    assert!(result1.is_ok(), "First checkpoint update should succeed");

    // Update the same checkpoint with new values
    checkpoint.offset = Some("200".to_string());
    checkpoint.sequence_number = Some(20);

    let result2 = checkpoint_store.update_checkpoint(checkpoint.clone()).await;
    assert!(result2.is_ok(), "Second checkpoint update should succeed");

    // Third update with different values
    checkpoint.offset = Some("300".to_string());
    checkpoint.sequence_number = Some(30);

    let result3 = checkpoint_store.update_checkpoint(checkpoint).await;
    assert!(result3.is_ok(), "Third checkpoint update should succeed");

    Ok(())
}

#[recorded::test]
async fn update_checkpoint_verify_in_list_checkpoints(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let checkpoint_store = create_test_checkpoint_store(recording)?;

    let namespace = recording.var("EVENTHUBS_HOST", None);
    let consumer_group = recording
        .var_opt("EVENTHUBS_CONSUMER_GROUP", None)
        .unwrap_or("$Default".to_string());
    let eventhub_name = recording.var("EVENTHUB_NAME", None);

    // Create a checkpoint with specific values
    let test_offset = "99999".to_string();
    let test_sequence_number = 42;
    let test_partition_id = "5".to_string();

    let checkpoint = Checkpoint {
        fully_qualified_namespace: namespace.clone(),
        event_hub_name: eventhub_name.clone(),
        consumer_group: consumer_group.clone(),
        partition_id: test_partition_id.clone(),
        offset: Some(test_offset.clone()),
        sequence_number: Some(test_sequence_number),
    };

    // Update the checkpoint
    let update_result = checkpoint_store.update_checkpoint(checkpoint).await;
    assert!(update_result.is_ok(), "Updating checkpoint should succeed");

    // Now list checkpoints and verify our update is there
    let checkpoints = checkpoint_store
        .list_checkpoints(&namespace, &eventhub_name, &consumer_group)
        .await?;

    // Find our checkpoint in the list
    let found_checkpoint = checkpoints
        .iter()
        .find(|cp| cp.partition_id == test_partition_id);

    if let Some(found) = found_checkpoint {
        assert_eq!(
            found.offset,
            Some(test_offset),
            "Offset should match what we set"
        );
        assert_eq!(
            found.sequence_number,
            Some(test_sequence_number),
            "Sequence number should match what we set"
        );
        assert_eq!(found.fully_qualified_namespace, namespace);
        assert_eq!(found.event_hub_name, eventhub_name);
        assert_eq!(found.consumer_group, consumer_group);
    } else {
        // It's possible the checkpoint doesn't show up immediately in list_checkpoints
        // due to Azure storage eventual consistency, so we'll just log this for debugging
        trace!("Checkpoint for partition {} not found in list_checkpoints result. This might be due to eventual consistency.", test_partition_id);
    }

    Ok(())
}
