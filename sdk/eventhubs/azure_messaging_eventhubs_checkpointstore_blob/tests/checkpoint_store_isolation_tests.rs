// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Isolation and error tests for the blob checkpoint store.

use azure_core::{
    error::ErrorKind,
    http::{StatusCode, Url},
    Result,
};
use azure_core_test::{recorded, Recording, TestContext};
use azure_messaging_eventhubs::{
    models::{Checkpoint, Ownership},
    CheckpointStore,
};
use azure_messaging_eventhubs_checkpointstore_blob::BlobCheckpointStore;
use azure_storage_blob::{
    models::StorageErrorCode, BlobContainerClient, BlobContainerClientOptions, StorageError,
};
use std::sync::Arc;

// The checkpoint store never resolves the namespace or the event hub against a
// service, so a random name for each key part keeps the blob prefix unique to
// one test run in a shared container.
fn unique(recording: &Recording, prefix: &str) -> String {
    recording.random_string::<24>(Some(prefix))
}

fn missing_container_name(recording: &Recording) -> String {
    recording
        .random_string::<32>(Some("nonexistent-"))
        .to_ascii_lowercase()
}

fn create_checkpoint_store(
    recording: &Recording,
    container_name: &str,
) -> Result<Arc<BlobCheckpointStore>> {
    let credential = recording.credential();
    let mut options = BlobContainerClientOptions::default();
    recording.instrument(&mut options.client_options);
    let endpoint = recording.var("AZURE_STORAGE_BLOB_ENDPOINT", None);
    let mut container_url = Url::parse(&endpoint)?;
    container_url
        .path_segments_mut()
        .expect("endpoint must be a valid base URL")
        .push(container_name);
    let blob_container_client =
        BlobContainerClient::new(container_url, Some(credential), Some(options))?;

    Ok(BlobCheckpointStore::new(blob_container_client))
}

#[recorded::test(live)]
async fn checkpoint_isolation_by_event_hub(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let container = recording.var("AZURE_STORAGE_BLOB_CONTAINER", None);
    let checkpoint_store = create_checkpoint_store(recording, &container)?;

    let namespace = unique(recording, "ns-");
    let consumer_group = unique(recording, "cg-");
    let event_hub_a = unique(recording, "eh-");
    let event_hub_b = unique(recording, "eh-");

    checkpoint_store
        .update_checkpoint(Checkpoint {
            fully_qualified_namespace: namespace.clone(),
            event_hub_name: event_hub_a.clone(),
            consumer_group: consumer_group.clone(),
            partition_id: "isolation-a".to_string(),
            offset: Some("1000".to_string()),
            sequence_number: Some(1),
        })
        .await?;

    checkpoint_store
        .update_checkpoint(Checkpoint {
            fully_qualified_namespace: namespace.clone(),
            event_hub_name: event_hub_b.clone(),
            consumer_group: consumer_group.clone(),
            partition_id: "isolation-b".to_string(),
            offset: Some("2000".to_string()),
            sequence_number: Some(2),
        })
        .await?;

    let listed = checkpoint_store
        .list_checkpoints(&namespace, &event_hub_a, &consumer_group)
        .await?;

    assert!(
        !listed.iter().any(|c| c.partition_id == "isolation-b"),
        "a checkpoint from the other event hub leaked into the listing: {listed:?}"
    );
    assert_eq!(
        listed.len(),
        1,
        "expected only the checkpoint written under the first event hub: {listed:?}"
    );
    assert_eq!(listed[0].partition_id, "isolation-a");
    assert_eq!(
        listed[0].offset,
        Some("1000".to_string()),
        "listed metadata must come from the first event hub's blob"
    );

    Ok(())
}

#[recorded::test(live)]
async fn checkpoint_isolation_by_consumer_group(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let container = recording.var("AZURE_STORAGE_BLOB_CONTAINER", None);
    let checkpoint_store = create_checkpoint_store(recording, &container)?;

    let namespace = unique(recording, "ns-");
    let event_hub = unique(recording, "eh-");
    let consumer_group_a = unique(recording, "cg-");
    let consumer_group_b = unique(recording, "cg-");

    checkpoint_store
        .update_checkpoint(Checkpoint {
            fully_qualified_namespace: namespace.clone(),
            event_hub_name: event_hub.clone(),
            consumer_group: consumer_group_a.clone(),
            partition_id: "isolation-a".to_string(),
            offset: Some("1000".to_string()),
            sequence_number: Some(1),
        })
        .await?;

    checkpoint_store
        .update_checkpoint(Checkpoint {
            fully_qualified_namespace: namespace.clone(),
            event_hub_name: event_hub.clone(),
            consumer_group: consumer_group_b.clone(),
            partition_id: "isolation-b".to_string(),
            offset: Some("2000".to_string()),
            sequence_number: Some(2),
        })
        .await?;

    let listed = checkpoint_store
        .list_checkpoints(&namespace, &event_hub, &consumer_group_a)
        .await?;

    assert!(
        !listed.iter().any(|c| c.partition_id == "isolation-b"),
        "a checkpoint from the other consumer group leaked into the listing: {listed:?}"
    );
    assert_eq!(
        listed.len(),
        1,
        "expected only the checkpoint written under the first consumer group: {listed:?}"
    );
    assert_eq!(listed[0].partition_id, "isolation-a");
    assert_eq!(
        listed[0].offset,
        Some("1000".to_string()),
        "listed metadata must come from the first consumer group's blob"
    );

    Ok(())
}

#[recorded::test(live)]
async fn checkpoint_isolation_by_namespace(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let container = recording.var("AZURE_STORAGE_BLOB_CONTAINER", None);
    let checkpoint_store = create_checkpoint_store(recording, &container)?;

    let event_hub = unique(recording, "eh-");
    let consumer_group = unique(recording, "cg-");
    let namespace_a = unique(recording, "ns-");
    let namespace_b = unique(recording, "ns-");

    checkpoint_store
        .update_checkpoint(Checkpoint {
            fully_qualified_namespace: namespace_a.clone(),
            event_hub_name: event_hub.clone(),
            consumer_group: consumer_group.clone(),
            partition_id: "isolation-a".to_string(),
            offset: Some("1000".to_string()),
            sequence_number: Some(1),
        })
        .await?;

    checkpoint_store
        .update_checkpoint(Checkpoint {
            fully_qualified_namespace: namespace_b.clone(),
            event_hub_name: event_hub.clone(),
            consumer_group: consumer_group.clone(),
            partition_id: "isolation-b".to_string(),
            offset: Some("2000".to_string()),
            sequence_number: Some(2),
        })
        .await?;

    let listed = checkpoint_store
        .list_checkpoints(&namespace_a, &event_hub, &consumer_group)
        .await?;

    assert!(
        !listed.iter().any(|c| c.partition_id == "isolation-b"),
        "a checkpoint from the other namespace leaked into the listing: {listed:?}"
    );
    assert_eq!(
        listed.len(),
        1,
        "expected only the checkpoint written under the first namespace: {listed:?}"
    );
    assert_eq!(listed[0].partition_id, "isolation-a");
    assert_eq!(
        listed[0].offset,
        Some("1000".to_string()),
        "listed metadata must come from the first namespace's blob"
    );

    Ok(())
}

#[recorded::test(live)]
async fn ownership_isolation_by_event_hub(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let container = recording.var("AZURE_STORAGE_BLOB_CONTAINER", None);
    let checkpoint_store = create_checkpoint_store(recording, &container)?;

    let namespace = unique(recording, "ns-");
    let consumer_group = unique(recording, "cg-");
    let event_hub_a = unique(recording, "eh-");
    let event_hub_b = unique(recording, "eh-");

    let ownership_a = Ownership {
        fully_qualified_namespace: namespace.clone(),
        event_hub_name: event_hub_a.clone(),
        consumer_group: consumer_group.clone(),
        partition_id: "isolation-a".to_string(),
        owner_id: Some("owner-a".to_string()),
        etag: None,
        last_modified_time: None,
    };
    checkpoint_store.claim_ownership(&[ownership_a]).await?;

    let ownership_b = Ownership {
        fully_qualified_namespace: namespace.clone(),
        event_hub_name: event_hub_b.clone(),
        consumer_group: consumer_group.clone(),
        partition_id: "isolation-b".to_string(),
        owner_id: Some("owner-b".to_string()),
        etag: None,
        last_modified_time: None,
    };
    checkpoint_store.claim_ownership(&[ownership_b]).await?;

    let listed = checkpoint_store
        .list_ownerships(&namespace, &event_hub_a, &consumer_group)
        .await?;

    assert!(
        !listed.iter().any(|o| o.partition_id == "isolation-b"),
        "an ownership from the other event hub leaked into the listing: {listed:?}"
    );
    assert_eq!(
        listed.len(),
        1,
        "expected only the ownership written under the first event hub: {listed:?}"
    );
    assert_eq!(listed[0].partition_id, "isolation-a");
    assert_eq!(
        listed[0].owner_id,
        Some("owner-a".to_string()),
        "listed owner must come from the first event hub's blob"
    );

    Ok(())
}

#[recorded::test(live)]
async fn ownership_isolation_by_consumer_group(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let container = recording.var("AZURE_STORAGE_BLOB_CONTAINER", None);
    let checkpoint_store = create_checkpoint_store(recording, &container)?;

    let namespace = unique(recording, "ns-");
    let event_hub = unique(recording, "eh-");
    let consumer_group_a = unique(recording, "cg-");
    let consumer_group_b = unique(recording, "cg-");

    let ownership_a = Ownership {
        fully_qualified_namespace: namespace.clone(),
        event_hub_name: event_hub.clone(),
        consumer_group: consumer_group_a.clone(),
        partition_id: "isolation-a".to_string(),
        owner_id: Some("owner-a".to_string()),
        etag: None,
        last_modified_time: None,
    };
    checkpoint_store.claim_ownership(&[ownership_a]).await?;

    let ownership_b = Ownership {
        fully_qualified_namespace: namespace.clone(),
        event_hub_name: event_hub.clone(),
        consumer_group: consumer_group_b.clone(),
        partition_id: "isolation-b".to_string(),
        owner_id: Some("owner-b".to_string()),
        etag: None,
        last_modified_time: None,
    };
    checkpoint_store.claim_ownership(&[ownership_b]).await?;

    let listed = checkpoint_store
        .list_ownerships(&namespace, &event_hub, &consumer_group_a)
        .await?;

    assert!(
        !listed.iter().any(|o| o.partition_id == "isolation-b"),
        "an ownership from the other consumer group leaked into the listing: {listed:?}"
    );
    assert_eq!(
        listed.len(),
        1,
        "expected only the ownership written under the first consumer group: {listed:?}"
    );
    assert_eq!(listed[0].partition_id, "isolation-a");
    assert_eq!(
        listed[0].owner_id,
        Some("owner-a".to_string()),
        "listed owner must come from the first consumer group's blob"
    );

    Ok(())
}

#[recorded::test(live)]
async fn ownership_isolation_by_namespace(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let container = recording.var("AZURE_STORAGE_BLOB_CONTAINER", None);
    let checkpoint_store = create_checkpoint_store(recording, &container)?;

    let event_hub = unique(recording, "eh-");
    let consumer_group = unique(recording, "cg-");
    let namespace_a = unique(recording, "ns-");
    let namespace_b = unique(recording, "ns-");

    let ownership_a = Ownership {
        fully_qualified_namespace: namespace_a.clone(),
        event_hub_name: event_hub.clone(),
        consumer_group: consumer_group.clone(),
        partition_id: "isolation-a".to_string(),
        owner_id: Some("owner-a".to_string()),
        etag: None,
        last_modified_time: None,
    };
    checkpoint_store.claim_ownership(&[ownership_a]).await?;

    let ownership_b = Ownership {
        fully_qualified_namespace: namespace_b.clone(),
        event_hub_name: event_hub.clone(),
        consumer_group: consumer_group.clone(),
        partition_id: "isolation-b".to_string(),
        owner_id: Some("owner-b".to_string()),
        etag: None,
        last_modified_time: None,
    };
    checkpoint_store.claim_ownership(&[ownership_b]).await?;

    let listed = checkpoint_store
        .list_ownerships(&namespace_a, &event_hub, &consumer_group)
        .await?;

    assert!(
        !listed.iter().any(|o| o.partition_id == "isolation-b"),
        "an ownership from the other namespace leaked into the listing: {listed:?}"
    );
    assert_eq!(
        listed.len(),
        1,
        "expected only the ownership written under the first namespace: {listed:?}"
    );
    assert_eq!(listed[0].partition_id, "isolation-a");
    assert_eq!(
        listed[0].owner_id,
        Some("owner-a".to_string()),
        "listed owner must come from the first namespace's blob"
    );

    Ok(())
}

#[recorded::test(live)]
async fn list_checkpoints_missing_container_reports_container_not_found(
    ctx: TestContext,
) -> Result<()> {
    let recording = ctx.recording();
    let container = missing_container_name(recording);
    let checkpoint_store = create_checkpoint_store(recording, &container)?;

    let namespace = unique(recording, "ns-");
    let event_hub = unique(recording, "eh-");
    let consumer_group = unique(recording, "cg-");

    let err = checkpoint_store
        .list_checkpoints(&namespace, &event_hub, &consumer_group)
        .await
        .expect_err("list_checkpoints on a missing container must fail");

    let storage_error = StorageError::try_from(err).expect("expected an HTTP response error");
    assert_eq!(
        storage_error.status_code,
        StatusCode::NotFound,
        "expected HTTP 404 from list_checkpoints on a missing container"
    );
    assert_eq!(
        storage_error.error_code.as_ref(),
        Some(&StorageErrorCode::ContainerNotFound),
        "expected the ContainerNotFound error code from list_checkpoints"
    );

    Ok(())
}

#[recorded::test(live)]
async fn list_ownerships_missing_container_reports_container_not_found(
    ctx: TestContext,
) -> Result<()> {
    let recording = ctx.recording();
    let container = missing_container_name(recording);
    let checkpoint_store = create_checkpoint_store(recording, &container)?;

    let namespace = unique(recording, "ns-");
    let event_hub = unique(recording, "eh-");
    let consumer_group = unique(recording, "cg-");

    let err = checkpoint_store
        .list_ownerships(&namespace, &event_hub, &consumer_group)
        .await
        .expect_err("list_ownerships on a missing container must fail");

    let storage_error = StorageError::try_from(err).expect("expected an HTTP response error");
    assert_eq!(
        storage_error.status_code,
        StatusCode::NotFound,
        "expected HTTP 404 from list_ownerships on a missing container"
    );
    assert_eq!(
        storage_error.error_code.as_ref(),
        Some(&StorageErrorCode::ContainerNotFound),
        "expected the ContainerNotFound error code from list_ownerships"
    );

    Ok(())
}

#[recorded::test(live)]
async fn update_checkpoint_missing_container_reports_container_not_found(
    ctx: TestContext,
) -> Result<()> {
    let recording = ctx.recording();
    let container = missing_container_name(recording);
    let checkpoint_store = create_checkpoint_store(recording, &container)?;

    let namespace = unique(recording, "ns-");
    let event_hub = unique(recording, "eh-");
    let consumer_group = unique(recording, "cg-");

    let err = checkpoint_store
        .update_checkpoint(Checkpoint {
            fully_qualified_namespace: namespace,
            event_hub_name: event_hub,
            consumer_group,
            partition_id: "isolation-a".to_string(),
            offset: Some("1000".to_string()),
            sequence_number: Some(1),
        })
        .await
        .expect_err("update_checkpoint on a missing container must fail");

    let storage_error = StorageError::try_from(err).expect("expected an HTTP response error");
    assert_eq!(
        storage_error.status_code,
        StatusCode::NotFound,
        "expected HTTP 404 from update_checkpoint on a missing container"
    );
    assert_eq!(
        storage_error.error_code.as_ref(),
        Some(&StorageErrorCode::ContainerNotFound),
        "expected the ContainerNotFound error code from update_checkpoint"
    );

    Ok(())
}

#[recorded::test(live)]
async fn claim_ownership_missing_container_reports_container_not_found(
    ctx: TestContext,
) -> Result<()> {
    let recording = ctx.recording();
    let container = missing_container_name(recording);
    let checkpoint_store = create_checkpoint_store(recording, &container)?;

    let namespace = unique(recording, "ns-");
    let event_hub = unique(recording, "eh-");
    let consumer_group = unique(recording, "cg-");

    let ownership = Ownership {
        fully_qualified_namespace: namespace,
        event_hub_name: event_hub,
        consumer_group,
        partition_id: "isolation-a".to_string(),
        owner_id: Some("owner-a".to_string()),
        etag: None,
        last_modified_time: None,
    };

    let err = checkpoint_store
        .claim_ownership(&[ownership])
        .await
        .expect_err("claim_ownership on a missing container must fail");

    let storage_error = StorageError::try_from(err).expect("expected an HTTP response error");
    assert_eq!(
        storage_error.status_code,
        StatusCode::NotFound,
        "expected HTTP 404 from claim_ownership on a missing container"
    );
    assert_eq!(
        storage_error.error_code.as_ref(),
        Some(&StorageErrorCode::ContainerNotFound),
        "expected the ContainerNotFound error code from claim_ownership"
    );

    Ok(())
}

#[test]
fn checkpoint_blob_prefix_name_layout() -> Result<()> {
    assert_eq!(
        Checkpoint::get_checkpoint_blob_prefix_name("ns1", "eh1", "cg1")?,
        "ns1/eh1/cg1/checkpoint/"
    );
    Ok(())
}

#[test]
fn checkpoint_blob_name_appends_partition_id() -> Result<()> {
    assert_eq!(
        Checkpoint::get_checkpoint_blob_name("ns1", "eh1", "cg1", "7")?,
        "ns1/eh1/cg1/checkpoint/7"
    );
    Ok(())
}

#[test]
fn ownership_prefix_name_layout() -> Result<()> {
    assert_eq!(
        Ownership::get_ownership_prefix_name("ns1", "eh1", "cg1")?,
        "ns1/eh1/cg1/ownership/"
    );
    Ok(())
}

#[test]
fn ownership_name_appends_partition_id() -> Result<()> {
    assert_eq!(
        Ownership::get_ownership_name("ns1", "eh1", "cg1", "7")?,
        "ns1/eh1/cg1/ownership/7"
    );
    Ok(())
}

#[test]
fn checkpoint_blob_name_differs_for_each_key_part() -> Result<()> {
    let base = Checkpoint::get_checkpoint_blob_name("ns1", "eh1", "cg1", "7")?;
    assert_ne!(
        base,
        Checkpoint::get_checkpoint_blob_name("ns2", "eh1", "cg1", "7")?,
        "the namespace must change the checkpoint blob name"
    );
    assert_ne!(
        base,
        Checkpoint::get_checkpoint_blob_name("ns1", "eh2", "cg1", "7")?,
        "the event hub must change the checkpoint blob name"
    );
    assert_ne!(
        base,
        Checkpoint::get_checkpoint_blob_name("ns1", "eh1", "cg2", "7")?,
        "the consumer group must change the checkpoint blob name"
    );
    Ok(())
}

#[test]
fn ownership_name_differs_for_each_key_part() -> Result<()> {
    let base = Ownership::get_ownership_name("ns1", "eh1", "cg1", "7")?;
    assert_ne!(
        base,
        Ownership::get_ownership_name("ns2", "eh1", "cg1", "7")?,
        "the namespace must change the ownership blob name"
    );
    assert_ne!(
        base,
        Ownership::get_ownership_name("ns1", "eh2", "cg1", "7")?,
        "the event hub must change the ownership blob name"
    );
    assert_ne!(
        base,
        Ownership::get_ownership_name("ns1", "eh1", "cg2", "7")?,
        "the consumer group must change the ownership blob name"
    );
    Ok(())
}

#[test]
fn key_builders_reject_empty_key_parts() {
    let cases: Vec<(Result<String>, &str)> = vec![
        (
            Checkpoint::get_checkpoint_blob_prefix_name("", "eh1", "cg1"),
            "fully_qualified_namespace",
        ),
        (
            Checkpoint::get_checkpoint_blob_prefix_name("ns1", "", "cg1"),
            "event_hub_name",
        ),
        (
            Checkpoint::get_checkpoint_blob_prefix_name("ns1", "eh1", ""),
            "consumer_group",
        ),
        (
            Ownership::get_ownership_prefix_name("", "eh1", "cg1"),
            "fully_qualified_namespace",
        ),
        (
            Ownership::get_ownership_prefix_name("ns1", "", "cg1"),
            "event_hub_name",
        ),
        (
            Ownership::get_ownership_prefix_name("ns1", "eh1", ""),
            "consumer_group",
        ),
        (
            Checkpoint::get_checkpoint_blob_name("ns1", "eh1", "cg1", ""),
            "partition_id",
        ),
        (
            Ownership::get_ownership_name("ns1", "eh1", "cg1", ""),
            "partition_id",
        ),
    ];

    for (result, field) in cases {
        let Err(err) = result else {
            panic!("an empty {field} must be rejected");
        };
        assert!(
            matches!(err.kind(), ErrorKind::Other),
            "expected ErrorKind::Other for the empty {field} case, got {:?}",
            err.kind()
        );
        assert_eq!(
            err.to_string(),
            format!("Required field {field} is empty"),
            "unexpected message for the empty {field} case"
        );
    }
}
