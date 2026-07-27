// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! In-memory-emulator integration tests for the cross-partition
//! `OFFSET`/`LIMIT`/`TOP` pipeline (`SkipTake`).
//!
//! These exercise the real path end to end: planner -> query-plan fetch ->
//! per-partition rewritten-query execution -> cross-partition merge ->
//! global `SkipTake`. Because the driver rejects cross-partition `ORDER BY`
//! today, the merge order across physical partitions is unspecified, so
//! cross-partition assertions check the deterministic *count* (plus subset and
//! uniqueness); exact ordered windows are asserted only for single-physical-
//! partition scenarios, where the emulator returns documents in creation
//! (`_rid`) order.
//!
//! The count is deterministic regardless of merge order:
//! `count = max(0, min(limit, total - offset))`. Verifying it end to end
//! guards the correctness of the emulator's Gateway-style `OFFSET x LIMIT y`
//! -> `OFFSET 0 LIMIT (x + y)` rewrite: without it, each partition would skip
//! `offset` locally and the client `SkipTake` would skip again, collapsing the
//! result.

use std::sync::Arc;

use azure_core::http::Url;

use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, ContainerConfig, InMemoryEmulatorHttpClient, VirtualAccountConfig,
    VirtualRegion,
};
use azure_data_cosmos_driver::models::{
    ContainerReference, CosmosOperation, FeedRange, ItemReference, PartitionKey,
    PartitionKeyDefinition,
};
use azure_data_cosmos_driver::options::{DriverOptions, OperationOptions};

const GATEWAY_URL: &str = "https://eastus.emulator.local";

/// Builds an in-memory emulator container with `partition_count` physical
/// partitions and a driver wired to it.
async fn setup(
    partition_count: u32,
) -> (
    Arc<InMemoryEmulatorHttpClient>,
    Arc<azure_data_cosmos_driver::driver::CosmosDriver>,
) {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);

    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
    let store = emulator.store();
    store.create_database("testdb");
    let container_config = ContainerConfig::new()
        .with_partition_count(partition_count)
        .build()
        .unwrap();
    store.create_container_with_config(
        "testdb",
        "testcoll",
        PartitionKeyDefinition::new(vec![std::borrow::Cow::Borrowed("/pk")]),
        container_config,
    );

    let runtime = emulator
        .runtime_builder()
        .build()
        .await
        .expect("runtime builds against the in-memory emulator");
    let account = azure_data_cosmos_driver::models::AccountReference::with_master_key(
        Url::parse(GATEWAY_URL).unwrap(),
        "ZW11bGF0b3Ita2V5",
    );
    let driver = runtime
        .create_driver(DriverOptions::builder(account).build())
        .await
        .expect("driver initializes against the emulator");
    (emulator, driver)
}

/// Seeds `(id, pk)` documents in order, each carrying an incrementing `seq`
/// field, and returns the ids in creation order.
async fn seed<S: AsRef<str>>(
    driver: &azure_data_cosmos_driver::driver::CosmosDriver,
    container: &ContainerReference,
    docs: &[(S, S)],
) -> Vec<String> {
    let mut ids = Vec::with_capacity(docs.len());
    for (seq, (id, pk)) in docs.iter().enumerate() {
        let id = id.as_ref();
        let pk = pk.as_ref();
        let item_ref = ItemReference::from_name(
            container,
            PartitionKey::from(pk.to_string()),
            id.to_string(),
        );
        let body = serde_json::json!({"id": id, "pk": pk, "seq": seq});
        driver
            .execute_singleton_operation(
                CosmosOperation::create_item(item_ref)
                    .with_body(serde_json::to_vec(&body).unwrap()),
                OperationOptions::default(),
            )
            .await
            .expect("seed item created");
        ids.push(id.to_owned());
    }
    ids
}

/// Runs `query` to completion and returns every result's `id` in emitted order.
async fn run_query_collecting_ids(
    driver: &azure_data_cosmos_driver::driver::CosmosDriver,
    container: &ContainerReference,
    query: &str,
) -> Vec<String> {
    let body = serde_json::to_vec(&serde_json::json!({"query": query, "parameters": []})).unwrap();
    let operation =
        CosmosOperation::query_items(container.clone(), Some(FeedRange::full())).with_body(body);
    let mut plan = driver
        .plan_operation(operation, &OperationOptions::default(), None)
        .await
        .expect("plan builds a cross-partition pipeline");

    let mut ids = Vec::new();
    while let Some(response) = driver
        .execute_plan(
            &mut plan,
            Some(container.clone()),
            OperationOptions::default(),
        )
        .await
        .expect("page executes")
    {
        let body: serde_json::Value =
            serde_json::from_slice(&response.into_body().single().unwrap()).unwrap();
        for item in body["Documents"].as_array().unwrap() {
            ids.push(item["id"].as_str().unwrap().to_owned());
        }
    }
    ids
}

/// Asserts `ids` are a duplicate-free subset of `universe`.
fn assert_subset_no_dups(ids: &[String], universe: &[String]) {
    let mut seen = std::collections::HashSet::new();
    for id in ids {
        assert!(seen.insert(id), "duplicate id in result: {id}");
        assert!(
            universe.contains(id),
            "result id {id} is not one of the seeded documents"
        );
    }
}

/// The load-bearing regression: a cross-partition `OFFSET x LIMIT y` must skip
/// `x` exactly once, globally. With six documents spread across two physical
/// partitions, `OFFSET 2 LIMIT 3` must return three documents. Under the
/// pre-rewrite (double-skip) bug each partition would skip two locally and the
/// client would skip two more, yielding zero — so the count alone discriminates
/// the fix.
#[tokio::test]
async fn cross_partition_offset_limit_returns_correct_count() {
    let (_emulator, driver) = setup(2).await;
    let container = driver
        .resolve_container("testdb", "testcoll")
        .await
        .expect("container resolves");

    let seeds = [
        ("d1", "pk-a"),
        ("d2", "pk-b"),
        ("d3", "pk-c"),
        ("d4", "pk-d"),
        ("d5", "pk-e"),
        ("d6", "pk-f"),
    ];
    let universe = seed(&driver, &container, &seeds).await;

    let ids =
        run_query_collecting_ids(&driver, &container, "SELECT c.id FROM c OFFSET 2 LIMIT 3").await;

    assert_eq!(
        ids.len(),
        3,
        "OFFSET 2 LIMIT 3 over 6 docs must return 3 (a double-skip bug would return fewer): {ids:?}"
    );
    assert_subset_no_dups(&ids, &universe);
}

/// When every document lands in a single physical partition, the emitted order
/// is deterministic (`_rid`/creation order), so the exact window is asserted.
#[tokio::test]
async fn single_partition_offset_limit_returns_exact_window() {
    let (_emulator, driver) = setup(2).await;
    let container = driver
        .resolve_container("testdb", "testcoll")
        .await
        .expect("container resolves");

    // All docs share one partition key, so they occupy one physical partition.
    let seeds = [
        ("d0", "solo"),
        ("d1", "solo"),
        ("d2", "solo"),
        ("d3", "solo"),
        ("d4", "solo"),
        ("d5", "solo"),
    ];
    seed(&driver, &container, &seeds).await;

    let ids =
        run_query_collecting_ids(&driver, &container, "SELECT c.id FROM c OFFSET 2 LIMIT 3").await;

    assert_eq!(
        ids,
        vec!["d2", "d3", "d4"],
        "single-partition OFFSET 2 LIMIT 3 must return the exact creation-order window"
    );
}

/// `OFFSET` at or beyond the total document count yields an empty result,
/// deterministically, even across partitions.
#[tokio::test]
async fn offset_beyond_total_returns_empty() {
    let (_emulator, driver) = setup(2).await;
    let container = driver
        .resolve_container("testdb", "testcoll")
        .await
        .expect("container resolves");

    let seeds = [
        ("d1", "pk-a"),
        ("d2", "pk-b"),
        ("d3", "pk-c"),
        ("d4", "pk-d"),
    ];
    seed(&driver, &container, &seeds).await;

    let ids =
        run_query_collecting_ids(&driver, &container, "SELECT c.id FROM c OFFSET 10 LIMIT 5").await;

    assert!(
        ids.is_empty(),
        "OFFSET past the total must be empty: {ids:?}"
    );
}

/// `LIMIT 0` yields an empty result regardless of offset or partitioning.
#[tokio::test]
async fn limit_zero_returns_empty() {
    let (_emulator, driver) = setup(2).await;
    let container = driver
        .resolve_container("testdb", "testcoll")
        .await
        .expect("container resolves");

    let seeds = [
        ("d1", "pk-a"),
        ("d2", "pk-b"),
        ("d3", "pk-c"),
        ("d4", "pk-d"),
    ];
    seed(&driver, &container, &seeds).await;

    let ids =
        run_query_collecting_ids(&driver, &container, "SELECT c.id FROM c OFFSET 1 LIMIT 0").await;

    assert!(ids.is_empty(), "LIMIT 0 must be empty: {ids:?}");
}

/// Cross-partition `TOP n` returns exactly `min(n, total)` documents. `TOP`
/// needs no rewrite (per-partition `TOP n` plus a global `TOP n` is already
/// correct); this guards that the `SkipTake` take-bound is applied once.
#[tokio::test]
async fn cross_partition_top_returns_correct_count() {
    let (_emulator, driver) = setup(2).await;
    let container = driver
        .resolve_container("testdb", "testcoll")
        .await
        .expect("container resolves");

    let seeds = [
        ("d1", "pk-a"),
        ("d2", "pk-b"),
        ("d3", "pk-c"),
        ("d4", "pk-d"),
        ("d5", "pk-e"),
        ("d6", "pk-f"),
    ];
    let universe = seed(&driver, &container, &seeds).await;

    let ids = run_query_collecting_ids(&driver, &container, "SELECT TOP 4 c.id FROM c").await;

    assert_eq!(ids.len(), 4, "TOP 4 over 6 docs must return 4: {ids:?}");
    assert_subset_no_dups(&ids, &universe);
}

/// `TOP n` with `n >= total` returns every document exactly once.
#[tokio::test]
async fn cross_partition_top_larger_than_total_returns_all() {
    let (_emulator, driver) = setup(2).await;
    let container = driver
        .resolve_container("testdb", "testcoll")
        .await
        .expect("container resolves");

    let seeds = [("d1", "pk-a"), ("d2", "pk-b"), ("d3", "pk-c")];
    let mut universe = seed(&driver, &container, &seeds).await;

    let mut ids = run_query_collecting_ids(&driver, &container, "SELECT TOP 50 c.id FROM c").await;

    assert_subset_no_dups(&ids, &universe);
    ids.sort();
    universe.sort();
    assert_eq!(ids, universe, "TOP >= total must return all documents once");
}

// ---------------------------------------------------------------------------
// Catalog-driven layer
//
// The hand-written cases above are the human-readable, regression-proof core.
// This section additionally drives every scenario tagged `inMemoryEmulator` in
// the shared JSON catalog (`tests/fixtures/skip_take_scenarios.json`) through
// the same real emulator path, so new scenarios can be added as data. The
// catalog's structural invariants (known assertion modes, source attribution,
// single-partition determinism for `exactOrdered`, etc.) are enforced
// separately by `tests/skip_take_scenario_catalog.rs`.
// ---------------------------------------------------------------------------

const CATALOG_JSON: &str = include_str!("../fixtures/skip_take_scenarios.json");

#[derive(serde::Deserialize)]
struct Catalog {
    scenarios: Vec<CatalogScenario>,
}

#[derive(serde::Deserialize)]
struct CatalogScenario {
    id: String,
    layers: Vec<String>,
    query: CatalogQuery,
    documents: Vec<CatalogDocument>,
    #[serde(rename = "partitionCount")]
    partition_count: u32,
    assertion: String,
    #[serde(rename = "expectedIds", default)]
    expected_ids: Vec<String>,
    #[serde(rename = "expectedCount")]
    expected_count: Option<usize>,
}

#[derive(serde::Deserialize)]
struct CatalogQuery {
    text: String,
}

#[derive(serde::Deserialize)]
struct CatalogDocument {
    id: String,
    pk: String,
}

/// Runs every `inMemoryEmulator`-tagged catalog scenario end to end against a
/// freshly built emulator, asserting the outcome according to its declared
/// `assertion` mode:
/// - `exactOrdered`: the emitted ids equal `expectedIds` (single logical
///   partition => deterministic creation order).
/// - `unorderedSubsetCount`: the emitted ids are a duplicate-free subset of the
///   seeds with cardinality `expectedCount` (cross-partition order is
///   unspecified without `ORDER BY`).
/// - `empty`: no ids are emitted.
#[tokio::test]
async fn catalog_scenarios_match_expectations() {
    let catalog: Catalog =
        serde_json::from_str(CATALOG_JSON).expect("skip_take catalog is valid JSON");

    for scenario in &catalog.scenarios {
        if !scenario.layers.iter().any(|l| l == "inMemoryEmulator") {
            continue;
        }

        let (_emulator, driver) = setup(scenario.partition_count).await;
        let container = driver
            .resolve_container("testdb", "testcoll")
            .await
            .expect("container resolves");

        let docs: Vec<(&str, &str)> = scenario
            .documents
            .iter()
            .map(|d| (d.id.as_str(), d.pk.as_str()))
            .collect();
        let universe = seed(&driver, &container, &docs).await;

        let ids = run_query_collecting_ids(&driver, &container, &scenario.query.text).await;

        match scenario.assertion.as_str() {
            "exactOrdered" => assert_eq!(
                ids, scenario.expected_ids,
                "scenario {}: expected exact ordered window",
                scenario.id
            ),
            "unorderedSubsetCount" => {
                let expected = scenario
                    .expected_count
                    .expect("unorderedSubsetCount scenario has expectedCount");
                assert_eq!(
                    ids.len(),
                    expected,
                    "scenario {}: expected {expected} results, got {ids:?}",
                    scenario.id
                );
                assert_subset_no_dups(&ids, &universe);
            }
            "empty" => assert!(
                ids.is_empty(),
                "scenario {}: expected an empty result, got {ids:?}",
                scenario.id
            ),
            other => panic!("scenario {}: unknown assertion {other:?}", scenario.id),
        }
    }
}
