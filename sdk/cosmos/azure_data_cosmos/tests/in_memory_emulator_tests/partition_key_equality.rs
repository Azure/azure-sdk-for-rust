// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! End-to-end coverage for cross-partition queries whose `WHERE` clause pins
//! the partition key with an equality (`c.pk = @v`) or `IN` (`c.pk IN (@a, @b)`)
//! predicate, driven through the public SDK against the in-memory emulator.
//!
//! These exercise the fix for issues #4574 / #4638 (Option B): the gateway
//! query plan returns a *closed* point EPK range `[X, X]` per pinned value, and
//! the planner normalizes each to the half-open window `[X, successor(X))`
//! (zero-extended to full partition-key width) and routes it as an
//! `x-ms-start-epk`/`x-ms-end-epk` pair with
//! `x-ms-read-key-type: EffectivePartitionKeyRange`. Before the fix an equality
//! point collapsed to an empty intersection and panicked the planner.
//!
//! The in-memory emulator honors `x-ms-start-epk`/`x-ms-end-epk` filtering and
//! rejects windowed reads that declare the point key type, so a green result
//! here proves the emitted window and key-type header are both correct.

use azure_core::credentials::Secret;
use azure_core::http::Url;
use azure_data_cosmos::{
    options::Region, AccountEndpoint, AccountReference, ContainerClient, CosmosClientBuilder,
    CosmosRuntimeBuilder, FeedScope, PartitionKey, Query, RoutingStrategy,
};
use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, ContainerConfig, InMemoryEmulatorHttpClient, VirtualAccountConfig,
    VirtualRegion,
};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const EMULATOR_GATEWAY_URL: &str = "https://eastus.emulator.local";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct Doc {
    id: String,
    pk: String,
    tag: String,
}

/// Eight rows, each with a distinct `/pk` value so they spread across the four
/// physical partitions of the container.
fn dataset() -> Vec<Doc> {
    (0..8)
        .map(|i| Doc {
            id: format!("id-{i}"),
            pk: format!("pk-{i}"),
            tag: format!("tag-{i}"),
        })
        .collect()
}

/// Provisions a single-hash `/pk` container (4 physical partitions) seeded with
/// [`dataset`], and returns a ready SDK [`ContainerClient`] wired to the
/// in-memory emulator.
async fn setup_container() -> ContainerClient {
    let run_id = Uuid::new_v4().to_string()[..8].to_string();

    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(EMULATOR_GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);

    let emulator = std::sync::Arc::new(InMemoryEmulatorHttpClient::new(config));
    let emulator_store = emulator.store();

    let db_name = format!("partition-key-equality-{run_id}");
    emulator_store.create_database(&db_name);
    emulator_store.create_container_with_config(
        &db_name,
        "docs",
        "/pk".into(),
        ContainerConfig::new()
            .with_partition_count(4)
            .build()
            .unwrap(),
    );

    let emulator_account = AccountReference::with_authentication_key(
        EMULATOR_GATEWAY_URL.parse::<AccountEndpoint>().unwrap(),
        Secret::new("dGVzdGtleQ=="),
    );
    let client = CosmosClientBuilder::new()
        .with_runtime(
            CosmosRuntimeBuilder::from(emulator.runtime_builder())
                .build()
                .await
                .unwrap(),
        )
        .build(
            emulator_account,
            RoutingStrategy::ProximityTo(Region::EAST_US),
        )
        .await
        .unwrap();

    let container = client
        .database_client(&db_name)
        .container_client("docs")
        .await
        .unwrap();

    for item in dataset() {
        container
            .create_item(PartitionKey::from(item.pk.clone()), &item.id, &item, None)
            .await
            .unwrap();
    }

    container
}

/// Runs `query` as a cross-partition (`FeedScope::full_container`) request.
async fn query_full_container(container: &ContainerClient, query: Query) -> Vec<Doc> {
    let iter = Box::pin(container.query_items(query, FeedScope::full_container(), None))
        .await
        .unwrap();
    Box::pin(iter.try_collect()).await.unwrap()
}

/// #4574 / #4638 — a cross-partition query with a single equality predicate on
/// the partition key returns exactly the one matching row (the query plan point
/// `[X, X]` is normalized to a `[X, successor(X))` window). Regression for the
/// empty-intersection planner panic.
#[tokio::test]
async fn equality_predicate_cross_partition_returns_single_row() {
    let container = setup_container().await;
    let query = Query::from("SELECT * FROM c WHERE c.pk = @pk")
        .with_parameter("@pk", "pk-3")
        .unwrap();

    let items = query_full_container(&container, query).await;

    assert_eq!(
        items.len(),
        1,
        "equality should return exactly 1 row, got {items:?}"
    );
    assert_eq!(items[0].pk, "pk-3");
    assert_eq!(items[0].id, "id-3");
}

/// #4574 / #4638 — the headline scenario: `c.pk IN (@a, @b)` on a cross-partition
/// query returns exactly the two matching rows. Each value becomes its own
/// disjoint `[X, successor(X))` window (no whole-partition widening, no
/// de-duplication needed).
#[tokio::test]
async fn in_predicate_cross_partition_returns_matching_rows() {
    let container = setup_container().await;
    let query = Query::from("SELECT * FROM c WHERE c.pk IN (@a, @b)")
        .with_parameter("@a", "pk-1")
        .unwrap()
        .with_parameter("@b", "pk-6")
        .unwrap();

    let mut items = query_full_container(&container, query).await;
    items.sort_by(|a, b| a.pk.cmp(&b.pk));

    assert_eq!(
        items.len(),
        2,
        "IN(2) should return exactly 2 rows, got {items:?}"
    );
    assert_eq!(items[0].pk, "pk-1");
    assert_eq!(items[1].pk, "pk-6");
}

/// #4574 / #4638 — an equality predicate for an absent value returns no rows
/// (the window is honored and simply matches nothing) rather than panicking or
/// falling back to a full scan.
#[tokio::test]
async fn equality_predicate_no_match_returns_empty() {
    let container = setup_container().await;
    let query = Query::from("SELECT * FROM c WHERE c.pk = @pk")
        .with_parameter("@pk", "pk-absent")
        .unwrap();

    let items = query_full_container(&container, query).await;

    assert!(
        items.is_empty(),
        "absent equality should return 0 rows, got {items:?}"
    );
}
