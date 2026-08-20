// Copyright (C) Microsoft Corporation. All rights reserved.
// Licensed under the MIT license.

//! # Event Hubs SDK for Rust
//!
//!

mod common;
use azure_core::error::ErrorKind as AzureErrorKind;
use azure_core::time::{Duration, OffsetDateTime};
use azure_messaging_eventhubs::CheckpointStore;
use std::sync::Arc;

use azure_messaging_eventhubs::{
    models::{Checkpoint, Ownership},
    InMemoryCheckpointStore,
};
use tracing::info;

#[test]
fn test_update_ownership() {
    common::setup();
    let store = InMemoryCheckpointStore::new();
    let ownership = Ownership {
        fully_qualified_namespace: "namespace".to_string(),
        event_hub_name: "event_hub".to_string(),
        consumer_group: "consumer_group".to_string(),
        partition_id: "partition_id".to_string(),
        owner_id: Some("owner_id".to_string()),
        etag: Some("etag".into()),
        ..Default::default()
    };
    let result = store.update_ownership(&ownership);
    assert!(result.is_ok());
}

#[test]
fn test_update_ownership_invalid() {
    common::setup();
    let store = InMemoryCheckpointStore::new();
    let ownership = Ownership {
        fully_qualified_namespace: "fqdn.servicebus.windows.net".to_string(),
        partition_id: "partition_id".to_string(),
        owner_id: Some("owner_id".to_string()),
        etag: Some("etag".into()),
        ..Default::default()
    };
    let result = store.update_ownership(&ownership);
    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().kind(), AzureErrorKind::Other);
}

/// A renewed ownership gets a new ETag and a fresh `last_modified_time`, and the
/// record it replaces no longer claims the partition.
#[tokio::test]
async fn test_claim_ownership_renewal_rotates_etag_and_timestamp() {
    common::setup();
    let store = InMemoryCheckpointStore::new();
    let ownership = Ownership {
        fully_qualified_namespace: "ns.servicebus.windows.net".to_string(),
        event_hub_name: "event_hub".to_string(),
        consumer_group: "consumer_group".to_string(),
        partition_id: "partition_id".to_string(),
        owner_id: Some("owner_id".to_string()),
        ..Default::default()
    };

    let first = store
        .claim_ownership(&[ownership])
        .await
        .unwrap()
        .pop()
        .expect("the first claim returns an ownership");
    assert!(first.etag.is_some());
    assert!(first.last_modified_time.is_some());

    // Renew with a deliberately old timestamp. A store that keeps the caller's
    // value gives the old time back, so the test tells a refresh from a copy
    // without a dependency on the clock resolution.
    let stale_time = OffsetDateTime::now_utc() - Duration::seconds(3600);
    let mut renewal = first.clone();
    renewal.last_modified_time = Some(stale_time);

    let second = store
        .claim_ownership(std::slice::from_ref(&renewal))
        .await
        .unwrap()
        .pop()
        .expect("the renewal returns an ownership");

    // The renewal must rotate the ETag, the same way a fresh claim does.
    assert!(second.etag.is_some());
    assert_ne!(first.etag, second.etag);

    // The renewal must stamp the current time, not keep the caller's value.
    assert!(second.last_modified_time.expect("the renewal sets a time") > stale_time);

    // The first record is stale now, so a claim that carries its ETag fails.
    let stale = store.update_ownership(&first);
    assert!(stale.is_err());
    assert_eq!(*stale.unwrap_err().kind(), AzureErrorKind::Other);
}

#[tokio::test]
async fn test_update_checkpoint() {
    common::setup();
    let store = InMemoryCheckpointStore::new();
    let checkpoint = Checkpoint {
        fully_qualified_namespace: "namespace".to_string(),
        event_hub_name: "event_hub".to_string(),
        consumer_group: "consumer_group".to_string(),
        partition_id: "partition_id".to_string(),
        ..Default::default()
    };
    let result = store.update_checkpoint(checkpoint).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_checkpoints() {
    common::setup();
    let store = InMemoryCheckpointStore::new();
    let checkpoint = Checkpoint {
        fully_qualified_namespace: "namespace".to_string(),
        event_hub_name: "event_hub".to_string(),
        consumer_group: "consumer_group".to_string(),
        partition_id: "partition_id".to_string(),
        ..Default::default()
    };
    info!("Adding checkpoint: {checkpoint:?}");
    store.update_checkpoint(checkpoint).await.unwrap();

    let checkpoints = store
        .list_checkpoints("namespace", "event_hub", "consumer_group")
        .await
        .unwrap();

    info!("List checkpoints: {checkpoints:?}");
    assert_eq!(checkpoints.len(), 1);
}

fn get_random_name(prefix: &str) -> String {
    format!("{}{}", prefix, azure_core::Uuid::new_v4())
}

#[tokio::test]
async fn checkpoints() -> azure_core::Result<()> {
    common::setup();
    let test_name = get_random_name("checkpoint");

    let checkpoint_store = Arc::new(InMemoryCheckpointStore::new());
    let checkpoints = checkpoint_store
        .list_checkpoints(
            "fully-qualified-namespace",
            "event-hub-name",
            "consumer-group",
        )
        .await
        .unwrap();
    assert_eq!(checkpoints.len(), 0);

    let checkpoint = Checkpoint {
        fully_qualified_namespace: "ns.servicebus.windows.net".to_string(),
        event_hub_name: "event-hub-name".to_string(),
        consumer_group: "consumer-group".to_string(),
        partition_id: test_name.clone(),
        offset: Some("offset".to_string()),
        sequence_number: Some(0),
    };

    // Even though we added a checkpoint in one namespace, it doesn't change the older namespace.
    checkpoint_store
        .update_checkpoint(checkpoint.clone())
        .await
        .unwrap();
    let checkpoints = checkpoint_store
        .list_checkpoints(
            "fully-qualified-namespace",
            "event-hub-name",
            "consumer-group",
        )
        .await
        .unwrap();
    assert_eq!(checkpoints.len(), 0);

    let checkpoints = checkpoint_store
        .list_checkpoints(
            "ns.servicebus.windows.net",
            "event-hub-name",
            "consumer-group",
        )
        .await;
    assert!(checkpoints.is_ok());
    let checkpoints = checkpoints.unwrap();
    assert_eq!(checkpoints.len(), 1);
    assert_eq!(checkpoints[0].partition_id, test_name.as_str());
    assert_eq!(checkpoints[0].offset, Some("offset".to_string()));
    assert_eq!(checkpoints[0].sequence_number, Some(0));
    assert_eq!(checkpoints[0].event_hub_name, "event-hub-name");
    assert_eq!(checkpoints[0].consumer_group, "consumer-group");

    Ok(())
}

/// A checkpoint that one caller stores with a mixed case consumer group must
/// come back to a caller that lists with a lowercase consumer group.
#[tokio::test]
async fn test_checkpoint_key_survives_consumer_group_case_change() {
    common::setup();
    let store = InMemoryCheckpointStore::new();
    let checkpoint = Checkpoint {
        fully_qualified_namespace: "NS-Test.ServiceBus.Windows.Net".to_string(),
        event_hub_name: "My-EventHub".to_string(),
        consumer_group: "$Default".to_string(),
        partition_id: "Partition-A".to_string(),
        ..Default::default()
    };
    store.update_checkpoint(checkpoint).await.unwrap();

    let checkpoints = store
        .list_checkpoints("ns-test.servicebus.windows.net", "my-eventhub", "$default")
        .await
        .unwrap();
    assert_eq!(
        checkpoints.len(),
        1,
        "the lowercase listing did not find the mixed case checkpoint"
    );

    // The store returns a clone of the stored record, so the fields keep the
    // case of the caller that stored them. No folded value leaks into a field.
    assert_eq!(checkpoints[0].partition_id, "Partition-A");
    assert_eq!(checkpoints[0].consumer_group, "$Default");
    assert_eq!(checkpoints[0].event_hub_name, "My-EventHub");
    assert_eq!(
        checkpoints[0].fully_qualified_namespace,
        "NS-Test.ServiceBus.Windows.Net"
    );
}

/// The load balancer drives the ownership path, so it needs the same
/// stability across the case of the consumer group.
#[tokio::test]
async fn test_ownership_key_survives_consumer_group_case_change() {
    common::setup();
    let store = InMemoryCheckpointStore::new();
    let ownership = Ownership {
        fully_qualified_namespace: "NS-Test.ServiceBus.Windows.Net".to_string(),
        event_hub_name: "My-EventHub".to_string(),
        consumer_group: "$Default".to_string(),
        partition_id: "Partition-A".to_string(),
        owner_id: Some("owner_id".to_string()),
        ..Default::default()
    };
    store.claim_ownership(&[ownership]).await.unwrap();

    let ownerships = store
        .list_ownerships("ns-test.servicebus.windows.net", "my-eventhub", "$default")
        .await
        .unwrap();
    assert_eq!(
        ownerships.len(),
        1,
        "the lowercase listing did not find the mixed case ownership"
    );
    assert_eq!(ownerships[0].partition_id, "Partition-A");
}
