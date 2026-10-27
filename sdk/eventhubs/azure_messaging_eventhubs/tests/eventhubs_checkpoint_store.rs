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

/// A claim that lost the race is a normal outcome. The store must report the
/// loss with an empty result and keep the winner's record.
#[tokio::test]
async fn claim_ownership_lost_claim_is_not_an_error() {
    common::setup();
    let store = InMemoryCheckpointStore::new();
    let ownership = Ownership {
        fully_qualified_namespace: "ns.servicebus.windows.net".to_string(),
        event_hub_name: "event_hub".to_string(),
        consumer_group: "consumer_group".to_string(),
        partition_id: "0".to_string(),
        owner_id: Some("owner-a".to_string()),
        ..Default::default()
    };

    let first = store.claim_ownership(&[ownership]).await.unwrap();
    assert_eq!(first.len(), 1);

    // A competing renewal rotates the ETag, which makes the first record stale.
    let second = store.claim_ownership(&[first[0].clone()]).await.unwrap();
    assert_eq!(second.len(), 1);

    let lost = store.claim_ownership(&[first[0].clone()]).await;
    assert!(
        lost.is_ok(),
        "a lost claim must not be an error, got: {:?}",
        lost.as_ref().err()
    );
    assert!(
        lost.unwrap().is_empty(),
        "the losing claim must return no ownership"
    );

    let ownerships = store
        .list_ownerships("ns.servicebus.windows.net", "event_hub", "consumer_group")
        .await
        .unwrap();
    assert_eq!(ownerships.len(), 1);
    assert_eq!(
        ownerships[0].etag, second[0].etag,
        "a lost claim must not mutate the record"
    );
}

/// One lost claim must not cancel the partitions behind it in the batch. The
/// stale partition sits in the middle of the batch, so a store that stops at
/// the first conflict fails this test.
#[tokio::test]
async fn claim_ownership_continues_past_a_lost_claim() {
    common::setup();
    let store = InMemoryCheckpointStore::new();
    let new_ownership = |partition_id: &str, owner_id: &str| Ownership {
        fully_qualified_namespace: "ns.servicebus.windows.net".to_string(),
        event_hub_name: "event_hub".to_string(),
        consumer_group: "consumer_group".to_string(),
        partition_id: partition_id.to_string(),
        owner_id: Some(owner_id.to_string()),
        ..Default::default()
    };

    let claimed = store
        .claim_ownership(&[
            new_ownership("0", "owner-a"),
            new_ownership("1", "owner-a"),
            new_ownership("2", "owner-a"),
        ])
        .await
        .unwrap();
    assert_eq!(claimed.len(), 3);

    // A second instance takes partition 1 behind the caller's back.
    let mut rotated = claimed[1].clone();
    rotated.owner_id = Some("owner-b".to_string());
    let winner_b = store.claim_ownership(&[rotated]).await.unwrap();
    assert_eq!(winner_b.len(), 1);

    let result = store
        .claim_ownership(&[claimed[0].clone(), claimed[1].clone(), claimed[2].clone()])
        .await;
    assert!(
        result.is_ok(),
        "one lost claim must not cancel the batch, got: {:?}",
        result.as_ref().err()
    );

    let kept = result.unwrap();
    let mut partition_ids = kept
        .iter()
        .map(|o| o.partition_id.clone())
        .collect::<Vec<_>>();
    partition_ids.sort();
    assert_eq!(partition_ids, vec!["0".to_string(), "2".to_string()]);

    let kept_zero = kept
        .iter()
        .find(|o| o.partition_id == "0")
        .expect("partition 0 stays with the caller");
    assert_ne!(
        kept_zero.etag, claimed[0].etag,
        "the winner's rotated ETag must reach the caller"
    );
    let kept_two = kept
        .iter()
        .find(|o| o.partition_id == "2")
        .expect("partition 2 stays with the caller");
    assert_ne!(
        kept_two.etag, claimed[2].etag,
        "the winner's rotated ETag must reach the caller"
    );

    let ownerships = store
        .list_ownerships("ns.servicebus.windows.net", "event_hub", "consumer_group")
        .await
        .unwrap();
    let lost_partition = ownerships
        .iter()
        .find(|o| o.partition_id == "1")
        .expect("partition 1 stays in the store");
    assert_eq!(
        lost_partition.etag, winner_b[0].etag,
        "the loser must not overwrite the winner"
    );
    assert_eq!(lost_partition.owner_id, Some("owner-b".to_string()));
}

/// A validation failure is a different outcome from a lost claim, so
/// `claim_ownership` must still return an error for a record it cannot use.
#[tokio::test]
async fn claim_ownership_invalid_ownership_still_errors() {
    common::setup();
    let store = InMemoryCheckpointStore::new();
    let ownership = Ownership {
        fully_qualified_namespace: "fqdn.servicebus.windows.net".to_string(),
        partition_id: "partition_id".to_string(),
        owner_id: Some("owner_id".to_string()),
        etag: Some("etag".into()),
        ..Default::default()
    };
    let result = store.claim_ownership(&[ownership]).await;
    assert!(result.is_err(), "a validation failure is not a lost claim");
    assert_eq!(*result.unwrap_err().kind(), AzureErrorKind::Other);
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
