// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Hierarchical-partition-key (HPK / MultiHash) query coverage driven through
//! the public SDK against the in-memory emulator.
//!
//! These tests exercise the fixes for:
//! - #4680 — prefix scopes (`FeedScope::partition` with fewer components than
//!   the container's partition-key hierarchy) must filter to the prefix instead
//!   of returning every item in the physical partition.
//! - #4681 — a full-container cross-partition query over an HPK container must
//!   fan out successfully instead of failing.
//!
//! The in-memory emulator honors `x-ms-start-epk`/`x-ms-end-epk` filtering for
//! MultiHash containers, so the prefix counts below are deterministic without a
//! live account.

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
struct GeoItem {
    id: String,
    country: String,
    state: String,
    city: String,
}

impl GeoItem {
    fn new(country: &str, state: &str, city: &str) -> Self {
        Self {
            id: format!("{country}-{state}-{city}"),
            country: country.to_string(),
            state: state.to_string(),
            city: city.to_string(),
        }
    }
}

/// The 9-row dataset shared by every HPK test:
/// - USA/CA: 5 rows
/// - USA/WA: 2 rows
/// - USA/NY: 1 row  → USA total = 8
/// - CANADA/ON: 1 row → grand total = 9
fn dataset() -> Vec<GeoItem> {
    vec![
        GeoItem::new("USA", "CA", "SanFrancisco"),
        GeoItem::new("USA", "CA", "LosAngeles"),
        GeoItem::new("USA", "CA", "SanDiego"),
        GeoItem::new("USA", "CA", "SanJose"),
        GeoItem::new("USA", "CA", "Oakland"),
        GeoItem::new("USA", "WA", "Seattle"),
        GeoItem::new("USA", "WA", "Tacoma"),
        GeoItem::new("USA", "NY", "NewYork"),
        GeoItem::new("CANADA", "ON", "Toronto"),
    ]
}

/// Provisions a `/country/state/city` MultiHash container (4 physical
/// partitions) seeded with [`dataset`], and returns a ready SDK
/// [`ContainerClient`] wired to the in-memory emulator.
async fn setup_hpk_container() -> ContainerClient {
    let run_id = Uuid::new_v4().to_string()[..8].to_string();

    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(EMULATOR_GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);

    let emulator = std::sync::Arc::new(InMemoryEmulatorHttpClient::new(config));
    let emulator_store = emulator.store();

    let db_name = format!("hpk-{run_id}");
    emulator_store.create_database(&db_name);
    emulator_store.create_container_with_config(
        &db_name,
        "geo",
        serde_json::from_value(serde_json::json!({
            "paths": ["/country", "/state", "/city"],
            "kind": "MultiHash",
            "version": 2
        }))
        .unwrap(),
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
        .container_client("geo")
        .await
        .unwrap();

    for item in dataset() {
        container
            .create_item(
                PartitionKey::from((item.country.clone(), item.state.clone(), item.city.clone())),
                &item.id,
                &item,
                None,
            )
            .await
            .unwrap();
    }

    container
}

async fn query_scope(container: &ContainerClient, scope: FeedScope) -> Vec<GeoItem> {
    // Box the query pipeline futures so callers awaiting `query_scope` don't trip
    // `clippy::large-futures` (the `query_items` future is ~16 KB on the stack).
    let iter = Box::pin(container.query_items(Query::from("SELECT * FROM c"), scope, None))
        .await
        .unwrap();
    Box::pin(iter.try_collect()).await.unwrap()
}

/// #4680 — a one-level prefix `(USA,)` returns only the 8 USA rows, not CANADA.
#[tokio::test]
async fn hpk_query_prefix_level1() {
    let container = setup_hpk_container().await;
    let items = query_scope(&container, FeedScope::partition(PartitionKey::from("USA"))).await;

    assert_eq!(items.len(), 8, "prefix (USA,) should return 8 items");
    assert!(
        items.iter().all(|i| i.country == "USA"),
        "prefix (USA,) leaked non-USA rows: {items:?}"
    );
}

/// #4680 — a two-level prefix `(USA, CA)` returns only the 5 CA rows.
#[tokio::test]
async fn hpk_query_prefix_level2() {
    let container = setup_hpk_container().await;
    let items = query_scope(
        &container,
        FeedScope::partition(PartitionKey::from(("USA", "CA"))),
    )
    .await;

    assert_eq!(items.len(), 5, "prefix (USA, CA) should return 5 items");
    assert!(
        items.iter().all(|i| i.country == "USA" && i.state == "CA"),
        "prefix (USA, CA) leaked non-CA rows: {items:?}"
    );
}

/// #4680 — a prefix with no matching rows `(USA, TX)` returns nothing.
#[tokio::test]
async fn hpk_query_prefix_no_match_returns_empty() {
    let container = setup_hpk_container().await;
    let items = query_scope(
        &container,
        FeedScope::partition(PartitionKey::from(("USA", "TX"))),
    )
    .await;

    assert!(items.is_empty(), "prefix (USA, TX) should return 0 items");
}

/// #4680 — explicit anti-leak guard: the `(USA, CA)` result must exclude every
/// WA, NY, and CANADA row.
#[tokio::test]
async fn hpk_query_prefix_correctness_guard() {
    let container = setup_hpk_container().await;
    let items = query_scope(
        &container,
        FeedScope::partition(PartitionKey::from(("USA", "CA"))),
    )
    .await;

    // A green result on an *empty* set would silently pass the loop-based
    // anti-leak checks below, so pin the expected count first (issue #4680).
    assert_eq!(
        items.len(),
        5,
        "prefix (USA, CA) should return exactly 5 items"
    );
    for item in &items {
        assert_eq!(item.country, "USA", "leaked foreign country: {item:?}");
        assert_eq!(item.state, "CA", "leaked foreign state: {item:?}");
    }
    assert!(
        !items
            .iter()
            .any(|i| i.state == "WA" || i.state == "NY" || i.country == "CANADA"),
        "prefix (USA, CA) leaked WA/NY/CANADA rows: {items:?}"
    );
}

/// #4681 — a full-container cross-partition query over an HPK container fans out
/// successfully and returns every row.
#[tokio::test]
async fn hpk_query_cross_partition_full_container() {
    let container = setup_hpk_container().await;
    let items = query_scope(&container, FeedScope::full_container()).await;

    assert_eq!(
        items.len(),
        9,
        "full-container query over an HPK container should return all 9 items"
    );
}

/// Sanity: a complete key still targets a single logical partition and returns
/// exactly its one row.
#[tokio::test]
async fn hpk_query_full_key_single_partition() {
    let container = setup_hpk_container().await;
    let items = query_scope(
        &container,
        FeedScope::partition(PartitionKey::from(("USA", "CA", "SanFrancisco"))),
    )
    .await;

    assert_eq!(items.len(), 1, "complete key should return exactly 1 item");
    assert_eq!(items[0].city, "SanFrancisco");
}
