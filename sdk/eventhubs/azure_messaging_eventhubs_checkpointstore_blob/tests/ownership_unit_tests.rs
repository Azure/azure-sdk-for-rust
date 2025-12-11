// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Unit tests for the blob checkpoint store models and utilities.

use azure_core::{http::Etag, time::OffsetDateTime, Result};
use azure_core_test::{recorded, Recording, TestContext};
use azure_messaging_eventhubs::{models::Ownership, CheckpointStore};
mod checkpoint_unit_tests;
use checkpoint_unit_tests::create_test_checkpoint_store;
use tracing::trace;

fn create_test_namespace(recording: &Recording) -> String {
    let namespace = recording.var("EVENTHUBS_HOST", None);
    recording.random_string::<55>(Some(namespace.as_str()))
}

#[recorded::test]
async fn list_ownerships(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    recording.set_matcher(Matcher::BodilessMatcher).await?;
    const TEST_PARTITION_ID: &str = "list_ownerships";
    let checkpoint_store = create_test_checkpoint_store(recording)?;

    let namespace = create_test_namespace(recording);
    let consumer_group = recording
        .var_opt("EVENTHUBS_CONSUMER_GROUP", None)
        .unwrap_or("$Default".to_string());
    let eventhub_name = recording.var("EVENTHUB_NAME", None);

    let ownership = Ownership {
        fully_qualified_namespace: namespace.clone(),
        event_hub_name: eventhub_name.clone(),
        consumer_group: consumer_group.clone(),
        partition_id: TEST_PARTITION_ID.to_string(),
        owner_id: Some("test-owner-1".to_string()),
        etag: None, // No etag for new ownership
        last_modified_time: None,
    };
    checkpoint_store.claim_ownership(&[ownership]).await?;

    let ownerships = checkpoint_store
        .list_ownerships(&namespace, &eventhub_name, &consumer_group)
        .await?;
    assert!(!ownerships.is_empty());
    let mut found_ownership = false;
    for (i, ownership) in ownerships.iter().enumerate() {
        trace!("Ownership: {ownership:?}");
        assert!(
            !ownership.partition_id.is_empty(),
            "Ownership {i} has an empty partition_id",
        );
        assert_eq!(ownership.fully_qualified_namespace, namespace);
        assert_eq!(ownership.consumer_group, consumer_group);
        if ownership.partition_id == TEST_PARTITION_ID {
            assert!(
                !found_ownership,
                "There should only be one ownership with the test partition ID"
            );
            found_ownership = true;
        }
    }
    assert!(found_ownership, "Expected ownership not found.");

    Ok(())
}

#[recorded::test]
async fn claim_ownership_empty_list(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let checkpoint_store = create_test_checkpoint_store(recording)?;

    // Test claiming ownership with an empty list
    let ownerships = checkpoint_store.claim_ownership(&[]).await?;
    assert!(
        ownerships.is_empty(),
        "Should return empty list for empty input"
    );

    Ok(())
}

#[recorded::test]
async fn claim_ownership_single_partition(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let checkpoint_store = create_test_checkpoint_store(recording)?;

    const TEST_PARTITION_ID: &str = "claim_ownership_single_partition";

    let namespace = create_test_namespace(recording);
    let consumer_group = recording
        .var_opt("EVENTHUBS_CONSUMER_GROUP", None)
        .unwrap_or("$Default".to_string());
    let eventhub_name = recording.var("EVENTHUB_NAME", None);

    // Create a single ownership to claim
    let ownership = Ownership {
        fully_qualified_namespace: namespace.clone(),
        event_hub_name: eventhub_name.clone(),
        consumer_group: consumer_group.clone(),
        partition_id: TEST_PARTITION_ID.to_string(),
        owner_id: Some("test-owner-1".to_string()),
        etag: None, // No etag for new ownership
        last_modified_time: None,
    };

    let claimed_ownerships = checkpoint_store.claim_ownership(&[ownership]).await?;

    assert_eq!(
        claimed_ownerships.len(),
        1,
        "Should return one claimed ownership"
    );

    let claimed = &claimed_ownerships[0];
    assert_eq!(claimed.fully_qualified_namespace, namespace);
    assert_eq!(claimed.event_hub_name, eventhub_name);
    assert_eq!(claimed.consumer_group, consumer_group);
    assert_eq!(claimed.partition_id, TEST_PARTITION_ID);
    assert_eq!(claimed.owner_id, Some("test-owner-1".to_string()));
    assert!(
        claimed.etag.is_some(),
        "Claimed ownership should have an etag"
    );
    assert!(
        claimed.last_modified_time.is_some(),
        "Claimed ownership should have last modified time"
    );

    Ok(())
}

#[recorded::test]
async fn claim_ownership_multiple_partitions(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let checkpoint_store = create_test_checkpoint_store(recording)?;

    let namespace = create_test_namespace(recording);
    let consumer_group = recording
        .var_opt("EVENTHUBS_CONSUMER_GROUP", None)
        .unwrap_or("$Default".to_string());
    let eventhub_name = recording.var("EVENTHUB_NAME", None);

    // Create multiple ownerships to claim
    let ownerships = vec![
        Ownership {
            fully_qualified_namespace: namespace.clone(),
            event_hub_name: eventhub_name.clone(),
            consumer_group: consumer_group.clone(),
            partition_id: "0".to_string(),
            owner_id: Some("test-owner-1".to_string()),
            etag: None,
            last_modified_time: None,
        },
        Ownership {
            fully_qualified_namespace: namespace.clone(),
            event_hub_name: eventhub_name.clone(),
            consumer_group: consumer_group.clone(),
            partition_id: "1".to_string(),
            owner_id: Some("test-owner-1".to_string()),
            etag: None,
            last_modified_time: None,
        },
        Ownership {
            fully_qualified_namespace: namespace.clone(),
            event_hub_name: eventhub_name.clone(),
            consumer_group: consumer_group.clone(),
            partition_id: "2".to_string(),
            owner_id: Some("test-owner-2".to_string()),
            etag: None,
            last_modified_time: None,
        },
    ];

    let claimed_ownerships = checkpoint_store.claim_ownership(&ownerships).await?;

    assert_eq!(
        claimed_ownerships.len(),
        3,
        "Should claim all three ownerships"
    );

    // Verify each claimed ownership
    for (i, claimed) in claimed_ownerships.iter().enumerate() {
        assert_eq!(claimed.fully_qualified_namespace, namespace);
        assert_eq!(claimed.event_hub_name, eventhub_name);
        assert_eq!(claimed.consumer_group, consumer_group);
        assert_eq!(claimed.partition_id, i.to_string());
        assert!(
            claimed.etag.is_some(),
            "Claimed ownership should have an etag"
        );
        assert!(
            claimed.last_modified_time.is_some(),
            "Claimed ownership should have last modified time"
        );
    }

    // Verify the owner IDs are correct
    assert_eq!(
        claimed_ownerships[0].owner_id,
        Some("test-owner-1".to_string())
    );
    assert_eq!(
        claimed_ownerships[1].owner_id,
        Some("test-owner-1".to_string())
    );
    assert_eq!(
        claimed_ownerships[2].owner_id,
        Some("test-owner-2".to_string())
    );

    Ok(())
}

#[recorded::test]
async fn claim_ownership_update_existing(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    const TEST_PARTITION_ID: &str = "ownership_update_existing";
    let checkpoint_store = create_test_checkpoint_store(recording)?;

    let namespace = create_test_namespace(recording);
    let consumer_group = recording
        .var_opt("EVENTHUBS_CONSUMER_GROUP", None)
        .unwrap_or("$Default".to_string());
    let eventhub_name = recording.var("EVENTHUB_NAME", None);

    // First, claim an ownership
    let initial_ownership = Ownership {
        fully_qualified_namespace: namespace.clone(),
        event_hub_name: eventhub_name.clone(),
        consumer_group: consumer_group.clone(),
        partition_id: TEST_PARTITION_ID.to_string(),
        owner_id: Some("initial-owner".to_string()),
        etag: None,
        last_modified_time: None,
    };

    let initial_claimed = checkpoint_store
        .claim_ownership(std::slice::from_ref(&initial_ownership))
        .await?;
    assert_eq!(initial_claimed.len(), 1);
    let initial_etag = initial_claimed[0].etag.clone();
    assert!(initial_etag.is_some());

    // Now try to update the ownership with the correct etag
    let updated_ownership = Ownership {
        owner_id: Some("updated-owner".to_string()),
        ..initial_claimed[0].clone()
    };

    let updated_claimed = checkpoint_store
        .claim_ownership(&[updated_ownership])
        .await?;
    assert_eq!(updated_claimed.len(), 1);

    let updated = &updated_claimed[0];
    assert_eq!(updated.owner_id, Some("updated-owner".to_string()));
    assert!(updated.etag.is_some());
    assert_ne!(
        updated.etag, initial_etag,
        "ETag should be different after update"
    );

    Ok(())
}

#[recorded::test]
async fn claim_ownership_concurrent_update_should_fail(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let checkpoint_store = create_test_checkpoint_store(recording)?;

    let namespace = create_test_namespace(recording);
    let consumer_group = recording
        .var_opt("EVENTHUBS_CONSUMER_GROUP", None)
        .unwrap_or("$Default".to_string());
    let eventhub_name = recording.var("EVENTHUB_NAME", None);

    // First, claim an ownership
    let initial_ownership = Ownership {
        fully_qualified_namespace: namespace.clone(),
        event_hub_name: eventhub_name.clone(),
        consumer_group: consumer_group.clone(),
        partition_id: "0".to_string(),
        owner_id: Some("initial-owner".to_string()),
        etag: None,
        last_modified_time: None,
    };

    let initial_claimed = checkpoint_store
        .claim_ownership(&[initial_ownership])
        .await?;
    assert_eq!(initial_claimed.len(), 1);

    // Simulate a concurrent update by using a stale/invalid etag
    let stale_ownership = Ownership {
        fully_qualified_namespace: namespace.clone(),
        event_hub_name: eventhub_name.clone(),
        consumer_group: consumer_group.clone(),
        partition_id: "0".to_string(),
        owner_id: Some("concurrent-owner".to_string()),
        etag: Some(Etag::from("stale-etag-value")),
        last_modified_time: Some(OffsetDateTime::now_utc()),
    };

    // This should fail due to etag mismatch
    let ownerships = checkpoint_store.claim_ownership(&[stale_ownership]).await?;
    assert!(ownerships.is_empty());

    Ok(())
}

#[recorded::test]
async fn claim_ownership_no_owner_id(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let checkpoint_store = create_test_checkpoint_store(recording)?;

    let namespace = create_test_namespace(recording);
    let consumer_group = recording
        .var_opt("EVENTHUBS_CONSUMER_GROUP", None)
        .unwrap_or("$Default".to_string());
    let eventhub_name = recording.var("EVENTHUB_NAME", None);

    // Create ownership without owner_id (releasing ownership)
    let ownership = Ownership {
        fully_qualified_namespace: namespace.clone(),
        event_hub_name: eventhub_name.clone(),
        consumer_group: consumer_group.clone(),
        partition_id: "0".to_string(),
        owner_id: None, // No owner - releasing ownership
        etag: None,
        last_modified_time: None,
    };

    let claimed_ownerships = checkpoint_store.claim_ownership(&[ownership]).await?;

    // The behavior when owner_id is None may vary - it might still create a blob
    // or handle it differently. We just verify it doesn't crash and returns a result.
    trace!(
        "Claimed ownerships with no owner_id: {:?}",
        claimed_ownerships
    );

    Ok(())
}
