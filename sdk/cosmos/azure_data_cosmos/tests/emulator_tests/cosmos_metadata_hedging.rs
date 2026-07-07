// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Emulator integration tests for metadata hedging.
//!
//! Metadata hedging only *fires* a cross-region hedge when the account exposes at least
//! two applicable regions. The Cosmos DB emulator is single-region, so these tests verify
//! the equally-important **safety property** that the emulator can prove deterministically:
//! enabling hedging must never break the two hot-path metadata reads (Collection Read and
//! the PartitionKeyRange ReadFeed), and must be a transparent no-op when there is no second
//! region to hedge to. The multi-region *behavioral* acceptance criteria (a hedge fires,
//! wins, or is arbitrated against the primary) require a real multi-region account — the
//! same limitation the `multi_write` failover tests carry — and the decision logic itself
//! is covered deterministically by the unit tests in
//! `src/routing/metadata_hedging.rs`.

#![cfg(feature = "key_auth")]

use super::framework;

use azure_core::Uuid;
use azure_data_cosmos::models::ContainerProperties;
use framework::{TestClient, TestOptions, TestRunContext};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
struct TestItem {
    id: String,
    partition_key: String,
    payload: usize,
}

fn make_item(unique_id: &str, partition: &str, payload: usize) -> TestItem {
    TestItem {
        id: format!("Item-{}", unique_id),
        partition_key: partition.to_string(),
        payload,
    }
}

/// Exercises both metadata caches (Collection Read via container properties, and the
/// PartitionKeyRange ReadFeed via a cross-partition query) plus item CRUD, asserting that
/// every operation returns correct data. Used by both the hedging-enabled and the
/// hedging-disabled tests so they assert identical behavior.
async fn exercise_metadata_reads(run_context: &TestRunContext) -> Result<(), Box<dyn Error>> {
    let db_client = run_context.create_db().await?;
    let container_id = format!("Container-{}", Uuid::new_v4());
    run_context
        .create_container(
            &db_client,
            ContainerProperties::new(container_id.clone(), "/partition_key".into()),
            None,
        )
        .await?;
    let container_client = db_client.container_client(&container_id).await;

    // Collection Read: reading container properties routes through ContainerCache.
    let props = container_client.read(None).await?.into_model()?;
    assert_eq!(props.id, container_id, "container id round-trips");

    // Item CRUD across two partitions: forces the routing map (PartitionKeyRangeCache)
    // and the partition-key definition (ContainerCache) to be resolved.
    let unique_id = Uuid::new_v4().to_string();
    let item_a = make_item(&format!("{unique_id}-a"), "partition-a", 1);
    let item_b = make_item(&format!("{unique_id}-b"), "partition-b", 2);
    let pk_a = item_a.partition_key.clone();
    let pk_b = item_b.partition_key.clone();
    let item_a_id = item_a.id.clone();

    container_client.create_item(&pk_a, &item_a, None).await?;
    container_client.create_item(&pk_b, &item_b, None).await?;

    let read_a = run_context
        .read_item::<TestItem>(&container_client, &pk_a, &item_a_id, None)
        .await?
        .into_model()?;
    assert_eq!(
        read_a, item_a,
        "item round-trips through the hedged read path"
    );

    // Cross-partition query: exercises the PartitionKeyRange ReadFeed (first page) path.
    let mut results: Vec<TestItem> = run_context
        .query_items(
            &container_client,
            "SELECT * FROM c",
            azure_data_cosmos::PartitionKey::EMPTY,
        )
        .await?;
    results.sort_by_key(|item| item.payload);
    assert_eq!(results.len(), 2, "cross-partition query returns both items");
    assert_eq!(results[0], item_a);
    assert_eq!(results[1], item_b);

    Ok(())
}

/// With metadata hedging ENABLED on a single-region (emulator) account, all metadata reads
/// and data operations succeed and return correct data. The hedged code path runs but stays
/// primary-only because there is no second applicable region — proving the feature is a safe
/// no-op in the single-region case (the common real-world scenario when enabling it).
#[tokio::test]
pub async fn metadata_reads_succeed_with_hedging_enabled() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_options(
        async |run_context| exercise_metadata_reads(run_context).await,
        TestOptions::new().with_client_metadata_hedging(true),
    )
    .await
}

/// Control: with metadata hedging DISABLED (the default), the same lifecycle succeeds
/// identically. Ensures the hedging-enabled test above is asserting parity, not masking a
/// regression in the unchanged path.
#[tokio::test]
pub async fn metadata_reads_succeed_with_hedging_disabled() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_options(
        async |run_context| exercise_metadata_reads(run_context).await,
        TestOptions::new().with_client_metadata_hedging(false),
    )
    .await
}
