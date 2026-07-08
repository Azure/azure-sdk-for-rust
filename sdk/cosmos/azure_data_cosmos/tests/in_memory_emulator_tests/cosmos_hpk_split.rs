// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Class C — Hierarchical Partition Key (HPK) coverage over a **multi–physical
//! partition ("split") topology**, driven through the public `azure_data_cosmos`
//! client against the in-memory emulator.
//!
//! The emulator-backed suite in `tests/emulator_tests/cosmos_hpk.rs` runs every
//! HPK scenario against a container that (from the SDK's point of view) has a
//! single physical partition, because the Cosmos DB emulator does not let a test
//! deterministically provision extra physical partitions or trigger a split. The
//! in-memory emulator does: it exposes `EmulatorStore::create_container_with_config`
//! (start with N physical partitions) and `EmulatorStore::split_partition` (split
//! an existing partition), and it runs entirely in-process, so these tests execute
//! in normal CI with no live account and no emulator process.
//!
//! What this module locks in:
//! * **Topology** — a MultiHash `/country/state/city` container provisioned with 2
//!   physical partitions reports exactly 2 feed ranges to the client.
//! * **Full-key targeting** — a complete 3-level key always resolves to exactly one
//!   physical partition, even across a split topology.
//! * **Distribution / routing correctness** — a multi-country dataset spreads
//!   deterministically across both partitions (proven via the SDK's
//!   `feed_range_from_partition_key` routing), and a cross-partition query fans out
//!   to both partitions and returns the whole dataset. The in-memory emulator's
//!   query engine does not honor `FeedScope::range` (EPK sub-range) filtering, so
//!   range-scoped result filtering is validated against the real emulator elsewhere.
//! * **Split preservation** — splitting a populated partition grows the topology to
//!   2 ranges while every item stays readable by full key, a full-container query
//!   still returns everything, and each item now routes to one of the two children.
//!
//! Two Class C behaviors are intentionally covered elsewhere rather than here:
//! * **Cross-partition prefix fan-out** — a partition-key *prefix* whose EPK band
//!   spans multiple physical partitions. The in-memory emulator only ever splits at
//!   the geometric EPK midpoint (`compute_epk_midpoint`), never inside a single
//!   prefix's hash band, so a shared-prefix dataset always lands wholly on one side
//!   and a prefix resolves to exactly one range (verified below in
//!   [`hpk_split_prefix_resolves_to_single_partition`]). Real prefix-query behavior
//!   against a genuine emulator is covered by the `hpk_query_prefix_*` tests in
//!   `cosmos_hpk.rs` (tracked by #4680).
//! * **Full-key-equality cross-partition query** — covered by the `#[ignore]`d
//!   `hpk_query_full_key_equality_cross_partition` in `cosmos_hpk.rs`, pending the
//!   planner fix in #4638.

use azure_data_cosmos::{
    options::Region, AccountEndpoint, AccountReference, ContainerClient, CosmosClient,
    CosmosClientBuilder, CosmosRuntimeBuilder, FeedScope, PartitionKey, Query, RoutingStrategy,
};
use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, ContainerConfig, EmulatorStore, InMemoryEmulatorHttpClient,
    VirtualAccountConfig, VirtualRegion,
};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

const GATEWAY_URL: &str = "https://eastus.emulator.local";
const DB: &str = "hpk_split_db";
const COLL: &str = "geo";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct GeoItem {
    id: String,
    country: String,
    state: String,
    city: String,
}

impl GeoItem {
    fn new(id: &str, country: &str, state: &str, city: &str) -> Self {
        Self {
            id: id.into(),
            country: country.into(),
            state: state.into(),
            city: city.into(),
        }
    }

    fn partition_key(&self) -> PartitionKey {
        PartitionKey::from((self.country.clone(), self.state.clone(), self.city.clone()))
    }
}

/// Deterministic dataset with distinct full keys chosen so the two physical
/// partitions are both exercised (verified empirically against the emulator's
/// hash distribution).
fn geo_dataset() -> Vec<GeoItem> {
    vec![
        GeoItem::new("usa", "USA", "CA", "LosAngeles"),
        GeoItem::new("canada", "CANADA", "ON", "Toronto"),
        GeoItem::new("mexico", "MEXICO", "JAL", "Guadalajara"),
        GeoItem::new("brazil", "BRAZIL", "SP", "SaoPaulo"),
        GeoItem::new("france", "FRANCE", "IDF", "Paris"),
        GeoItem::new("germany", "GERMANY", "BE", "Berlin"),
        GeoItem::new("japan", "JAPAN", "13", "Tokyo"),
        GeoItem::new("india", "INDIA", "MH", "Mumbai"),
    ]
}

fn emulator() -> Arc<InMemoryEmulatorHttpClient> {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        azure_core::http::Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);
    Arc::new(InMemoryEmulatorHttpClient::new(config))
}

async fn build_client(
    emulator: &Arc<InMemoryEmulatorHttpClient>,
) -> Result<CosmosClient, Box<dyn Error>> {
    let account = AccountReference::with_authentication_key(
        GATEWAY_URL.parse::<AccountEndpoint>().unwrap(),
        azure_core::credentials::Secret::new("dGVzdGtleQ=="),
    );
    let client = CosmosClientBuilder::new()
        .with_runtime(
            CosmosRuntimeBuilder::from(emulator.runtime_builder())
                .build()
                .await?,
        )
        .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
        .await?;
    Ok(client)
}

/// Creates the MultiHash `/country/state/city` container with `partition_count`
/// physical partitions.
fn create_geo_container(store: &Arc<EmulatorStore>, partition_count: u32) {
    store.create_database(DB);
    store.create_container_with_config(
        DB,
        COLL,
        serde_json::from_value(serde_json::json!({
            "paths": ["/country", "/state", "/city"],
            "kind": "MultiHash",
            "version": 2
        }))
        .unwrap(),
        ContainerConfig::new()
            .with_partition_count(partition_count)
            .build()
            .unwrap(),
    );
}

async fn container_client(client: &CosmosClient) -> Result<ContainerClient, Box<dyn Error>> {
    Ok(client.database_client(DB).container_client(COLL).await?)
}

async fn seed(container: &ContainerClient, items: &[GeoItem]) -> Result<(), Box<dyn Error>> {
    for item in items {
        container
            .create_item(item.partition_key(), &item.id, item, None)
            .await?;
    }
    Ok(())
}

/// Runs a cross-partition `SELECT * FROM c` (full-container fan-out) and returns
/// the sorted ids found.
async fn query_all_ids(container: &ContainerClient) -> Result<Vec<String>, Box<dyn Error>> {
    let items: Vec<GeoItem> = container
        .query_items(
            Query::from("SELECT * FROM c"),
            FeedScope::full_container(),
            None,
        )
        .await?
        .try_collect()
        .await?;
    let mut ids: Vec<String> = items.into_iter().map(|i| i.id).collect();
    ids.sort();
    Ok(ids)
}

/// c01 — A container provisioned with 2 physical partitions reports exactly 2
/// feed ranges to the client.
#[tokio::test]
async fn hpk_split_topology_reports_two_ranges() -> Result<(), Box<dyn Error>> {
    let emulator = emulator();
    create_geo_container(&emulator.store(), 2);
    let client = build_client(&emulator).await?;
    let container = container_client(&client).await?;

    let ranges = container.read_feed_ranges(None).await?;
    assert_eq!(
        ranges.len(),
        2,
        "a 2-partition MultiHash container must expose 2 feed ranges"
    );
    Ok(())
}

/// c03 — Every full 3-level key resolves to exactly one physical partition.
#[tokio::test]
async fn hpk_split_full_key_targets_single_partition() -> Result<(), Box<dyn Error>> {
    let emulator = emulator();
    create_geo_container(&emulator.store(), 2);
    let client = build_client(&emulator).await?;
    let container = container_client(&client).await?;

    let ranges = container.read_feed_ranges(None).await?;
    for item in geo_dataset() {
        let resolved = container
            .feed_range_from_partition_key(item.partition_key(), None)
            .await?;
        assert_eq!(
            resolved.len(),
            1,
            "full key {:?} must map to exactly one physical partition",
            (&item.country, &item.state, &item.city)
        );
        assert!(
            ranges.contains(&resolved[0]),
            "resolved range for {:?} must be one of the container's feed ranges",
            item.id
        );
    }
    Ok(())
}

/// c04 — A multi-country dataset spreads deterministically across both physical
/// partitions (proven via the SDK's `feed_range_from_partition_key` routing), and
/// a cross-partition `SELECT * FROM c` fans out to both partitions and returns the
/// whole dataset.
///
/// Note: distribution is asserted through feed-range *resolution* rather than
/// `FeedScope::range`-scoped queries, because the in-memory emulator's query engine
/// does not filter documents by EPK sub-range (a range-scoped `SELECT *` returns the
/// full container). Range-scoped query filtering is validated against the real
/// emulator in `cosmos_feed_ranges.rs` / `cosmos_hpk.rs`.
#[tokio::test]
async fn hpk_split_dataset_distributes_across_partitions() -> Result<(), Box<dyn Error>> {
    let emulator = emulator();
    create_geo_container(&emulator.store(), 2);
    let client = build_client(&emulator).await?;
    let container = container_client(&client).await?;

    let dataset = geo_dataset();
    seed(&container, &dataset).await?;

    let ranges = container.read_feed_ranges(None).await?;
    assert_eq!(ranges.len(), 2);

    // Resolve every item to a physical partition and record which one.
    let mut per_partition = [0usize; 2];
    for item in &dataset {
        let resolved = container
            .feed_range_from_partition_key(item.partition_key(), None)
            .await?;
        assert_eq!(
            resolved.len(),
            1,
            "full key {:?} must resolve to exactly one physical partition",
            item.id
        );
        let idx = ranges
            .iter()
            .position(|r| r == &resolved[0])
            .expect("resolved range must be one of the container's feed ranges");
        per_partition[idx] += 1;

        // Resolution is deterministic: resolving the same key again is identical.
        let again = container
            .feed_range_from_partition_key(item.partition_key(), None)
            .await?;
        assert_eq!(
            resolved, again,
            "feed-range resolution must be deterministic"
        );
    }

    // Both physical partitions are exercised by the dataset.
    assert!(
        per_partition[0] > 0 && per_partition[1] > 0,
        "dataset must populate both physical partitions (distribution={per_partition:?})"
    );
    assert_eq!(
        per_partition[0] + per_partition[1],
        dataset.len(),
        "every item must map to exactly one partition"
    );

    // A cross-partition query fans out to both partitions and returns everything.
    let got = query_all_ids(&container).await?;
    let mut expected: Vec<String> = dataset.iter().map(|i| i.id.clone()).collect();
    expected.sort();
    assert_eq!(
        got, expected,
        "cross-partition query must return the full dataset across both partitions"
    );
    Ok(())
}

/// c05 — Splitting a populated partition grows the topology to 2 ranges while
/// preserving data and routing: every item stays readable by full key, a
/// full-container query still returns everything, and each item now routes to
/// one of the two children.
#[tokio::test]
async fn hpk_split_preserves_data_and_routing() -> Result<(), Box<dyn Error>> {
    let emulator = emulator();
    let store = emulator.store();
    create_geo_container(&store, 1);

    let dataset = geo_dataset();
    {
        let client = build_client(&emulator).await?;
        let container = container_client(&client).await?;
        assert_eq!(
            container.read_feed_ranges(None).await?.len(),
            1,
            "container must start with a single physical partition"
        );
        seed(&container, &dataset).await?;
    }

    // Split the only partition and let the control-plane task complete.
    store.split_partition(DB, COLL, 0, Duration::from_secs(0));
    store.drain_pending_control_plane().await;

    // A fresh client observes the post-split topology (routing cache is per-client).
    let client = build_client(&emulator).await?;
    let container = container_client(&client).await?;

    let ranges = container.read_feed_ranges(None).await?;
    assert_eq!(ranges.len(), 2, "split must yield two child feed ranges");

    // Every item is still readable by full key after the split.
    for item in &dataset {
        let read: GeoItem = container
            .read_item(item.partition_key(), &item.id, None)
            .await?
            .into_body()
            .into_single()
            .unwrap();
        assert_eq!(&read, item, "item {} changed across the split", item.id);

        // And still routes to exactly one of the two children.
        let resolved = container
            .feed_range_from_partition_key(item.partition_key(), None)
            .await?;
        assert_eq!(resolved.len(), 1);
        assert!(
            ranges.contains(&resolved[0]),
            "item {} must route to one of the child ranges after the split",
            item.id
        );
    }

    // A full-container (cross-partition) query still returns the whole dataset.
    let got = query_all_ids(&container).await?;
    let mut expected: Vec<String> = dataset.iter().map(|i| i.id.clone()).collect();
    expected.sort();
    assert_eq!(
        got, expected,
        "cross-partition query after split must return the full dataset"
    );
    Ok(())
}

/// c02 (documented limitation) — With the in-memory emulator's geometric-midpoint
/// split, a shared partition-key *prefix* never fans out across physical
/// partitions: its EPK band lies wholly within one partition, so the client
/// returns exactly one feed range. This guards against over-fanning (returning
/// more ranges than necessary). Genuine cross-partition prefix fan-out is a real
/// account / real emulator behavior tracked separately (see the module docs).
#[tokio::test]
async fn hpk_split_prefix_resolves_to_single_partition() -> Result<(), Box<dyn Error>> {
    let emulator = emulator();
    create_geo_container(&emulator.store(), 2);
    let client = build_client(&emulator).await?;
    let container = container_client(&client).await?;

    let ranges = container.read_feed_ranges(None).await?;

    for (country, state) in [("USA", "CA"), ("CANADA", "ON"), ("INDIA", "MH")] {
        let level1 = container
            .feed_range_from_partition_key(PartitionKey::from(country), None)
            .await?;
        assert_eq!(
            level1.len(),
            1,
            "level-1 prefix ({country}) must resolve to a single physical partition"
        );
        assert!(ranges.contains(&level1[0]));

        let level2 = container
            .feed_range_from_partition_key(PartitionKey::from((country, state)), None)
            .await?;
        assert_eq!(
            level2.len(),
            1,
            "level-2 prefix ({country},{state}) must resolve to a single physical partition"
        );
        assert!(ranges.contains(&level2[0]));
    }
    Ok(())
}
