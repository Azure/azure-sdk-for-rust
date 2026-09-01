// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! In-memory-emulator integration tests for the cross-partition streaming
//! `ORDER BY` pipeline (`StreamingOrderedMerge`).
//!
//! Unlike the mock-pipeline tests, these exercise the real planner →
//! query-plan fetch → per-partition rewritten-query execution → envelope
//! parsing → merge path against the in-memory emulator's actual query
//! evaluator.
//!
//! Fresh (non-resumed) scenarios exercise the pure streaming ORDER BY merge;
//! the combined ORDER BY + `OFFSET`/`LIMIT`/`TOP` section additionally covers
//! pagination-resume and a mid-stream partition split for the composed
//! `SkipTake { child: StreamingOrderedMerge }` pipeline.

use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;

use azure_core::http::Url;

use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, ContainerConfig, InMemoryEmulatorHttpClient, VirtualAccountConfig,
    VirtualRegion,
};
use azure_data_cosmos_driver::models::{
    CosmosOperation, FeedRange, ItemReference, MaxItemCountHint, PartitionKey,
    PartitionKeyDefinition,
};
use azure_data_cosmos_driver::options::{DriverOptions, OperationOptions, PlanOptions};

const GATEWAY_URL: &str = "https://eastus.emulator.local";

/// Builds a two-physical-partition in-memory emulator container and a
/// driver wired to it.
async fn setup() -> (
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
        .with_partition_count(2)
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

#[tokio::test]
async fn cross_partition_order_by_returns_globally_sorted_results() {
    let (_emulator, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .expect("container resolves");

    // Shuffled ranks across partitions so an unsorted merge would fail.
    let seeds = [
        ("d1", "pk-a", 5),
        ("d2", "pk-b", 1),
        ("d3", "pk-c", 4),
        ("d4", "pk-d", 2),
        ("d5", "pk-e", 3),
        ("d6", "pk-f", 0),
    ];
    for (id, pk, rank) in seeds {
        let item_ref = ItemReference::from_name(&container, PartitionKey::from(pk), id);
        let body = serde_json::json!({"id": id, "pk": pk, "rank": rank});
        driver
            .execute_singleton_operation(
                CosmosOperation::create_item(item_ref)
                    .with_body(serde_json::to_vec(&body).unwrap()),
                OperationOptions::default(),
            )
            .await
            .expect("seed item created");
    }

    let operation = CosmosOperation::query_items(container.clone(), Some(FeedRange::full()))
        .with_body(br#"{"query":"SELECT * FROM c ORDER BY c.rank ASC","parameters":[]}"#.to_vec());

    let mut plan = Box::pin(driver.plan_operation(
        operation,
        &OperationOptions::default(),
        None,
        &PlanOptions::default(),
    ))
    .await
    .expect("plan builds a StreamingOrderedMerge pipeline");

    let mut ranks: Vec<i64> = Vec::new();
    while let Some(response) = driver
        .execute_plan(
            &mut plan,
            Some(container.clone()),
            OperationOptions::default(),
        )
        .await
        .expect("page executes")
    {
        for item in &super::page_document_values(response) {
            ranks.push(item["rank"].as_i64().unwrap());
        }
    }

    assert_eq!(ranks, vec![0, 1, 2, 3, 4, 5]);
}

#[tokio::test]
async fn cross_partition_order_by_paginates_with_small_page_size() {
    let (_emulator, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .expect("container resolves");

    for i in 0..8 {
        let id = format!("item-{i}");
        let pk = format!("pk-{i}");
        // Descending seed order so a passthrough merge would fail.
        let rank = 8 - i;
        let item_ref =
            ItemReference::from_name(&container, PartitionKey::from(pk.clone()), id.clone());
        let body = serde_json::json!({"id": id, "pk": pk, "rank": rank});
        driver
            .execute_singleton_operation(
                CosmosOperation::create_item(item_ref)
                    .with_body(serde_json::to_vec(&body).unwrap()),
                OperationOptions::default(),
            )
            .await
            .expect("seed item created");
    }

    let operation = CosmosOperation::query_items(container.clone(), Some(FeedRange::full()))
        .with_body(br#"{"query":"SELECT * FROM c ORDER BY c.rank ASC","parameters":[]}"#.to_vec())
        .with_max_item_count(MaxItemCountHint::Limit(
            std::num::NonZeroU32::new(3).unwrap(),
        ));

    let mut plan = Box::pin(driver.plan_operation(
        operation,
        &OperationOptions::default(),
        None,
        &PlanOptions::default(),
    ))
    .await
    .unwrap();

    let mut ranks: Vec<i64> = Vec::new();
    let mut page_sizes: Vec<usize> = Vec::new();
    while let Some(response) = driver
        .execute_plan(
            &mut plan,
            Some(container.clone()),
            OperationOptions::default(),
        )
        .await
        .unwrap()
    {
        let documents = super::page_document_values(response);
        page_sizes.push(documents.len());
        for item in &documents {
            ranks.push(item["rank"].as_i64().unwrap());
        }
    }

    assert_eq!(ranks, (1..=8).collect::<Vec<_>>());
    assert!(
        page_sizes.iter().all(|&n| n <= 3),
        "no page should exceed the requested max_item_count: {page_sizes:?}"
    );
    assert!(
        page_sizes.len() >= 3,
        "expected multiple pages: {page_sizes:?}"
    );
}

/// Runs `query` (already-valid Cosmos SQL) to completion and returns every
/// result's `id`, in emitted order.
async fn run_query_collecting_ids(
    driver: &azure_data_cosmos_driver::driver::CosmosDriver,
    container: &azure_data_cosmos_driver::models::ContainerReference,
    query: &str,
) -> Vec<String> {
    let body = serde_json::to_vec(&serde_json::json!({"query": query, "parameters": []})).unwrap();
    let operation =
        CosmosOperation::query_items(container.clone(), Some(FeedRange::full())).with_body(body);
    let mut plan = Box::pin(driver.plan_operation(
        operation,
        &OperationOptions::default(),
        None,
        &PlanOptions::default(),
    ))
    .await
    .expect("plan builds a StreamingOrderedMerge pipeline");

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
        for item in &super::page_document_values(response) {
            ids.push(item["id"].as_str().unwrap().to_owned());
        }
    }
    ids
}

/// Regression: within a single logical partition, a full-key tie must be
/// broken by document `_rid` (creation order) — never by the store's
/// internal iteration order. The in-memory emulator stores a logical
/// partition's documents in a `BTreeMap` keyed by id (alphabetical
/// iteration), so seeding out of alphabetical order ("bbb" first, then
/// "aaa", then "ccc", all under the same partition key) makes the two
/// orders disagree: a correct rid-based tie-break must reorder them.
#[tokio::test]
async fn tied_order_by_key_within_one_partition_returns_rid_order_not_storage_order() {
    let (_emulator, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .expect("container resolves");

    let pk = "tied-pk";
    for id in ["bbb", "aaa", "ccc"] {
        let item_ref = ItemReference::from_name(&container, PartitionKey::from(pk), id);
        let body = serde_json::json!({"id": id, "pk": pk, "rank": 5});
        driver
            .execute_singleton_operation(
                CosmosOperation::create_item(item_ref)
                    .with_body(serde_json::to_vec(&body).unwrap()),
                OperationOptions::default(),
            )
            .await
            .expect("seed item created");
    }

    let asc_ids = Box::pin(run_query_collecting_ids(
        &driver,
        &container,
        "SELECT * FROM c ORDER BY c.rank ASC",
    ))
    .await;
    assert_eq!(
        asc_ids,
        vec!["bbb", "aaa", "ccc"],
        "ASC ties must follow creation (rid) order, not the store's alphabetical-by-id \
         iteration order: {asc_ids:?}"
    );

    let desc_ids = Box::pin(run_query_collecting_ids(
        &driver,
        &container,
        "SELECT * FROM c ORDER BY c.rank DESC",
    ))
    .await;
    assert_eq!(
        desc_ids,
        vec!["ccc", "aaa", "bbb"],
        "DESC ties must follow reverse creation (rid) order: {desc_ids:?}"
    );
}

/// Regression: a full-key tie spanning physical partitions is deterministic
/// and complete. Cross-stream ties use leftmost EPK range; RID ordering remains
/// local to each backend range.
#[tokio::test]
async fn tied_order_by_key_across_partitions_is_deterministic_and_complete() {
    let (_emulator, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .expect("container resolves");

    // Same partition keys as `cross_partition_order_by_returns_globally_sorted_results`,
    // known to spread across both of this container's physical partitions.
    let seeds = [
        ("s6", "pk-f"),
        ("s1", "pk-a"),
        ("s5", "pk-e"),
        ("s2", "pk-b"),
        ("s4", "pk-d"),
        ("s3", "pk-c"),
    ];
    for (id, pk) in seeds {
        let item_ref = ItemReference::from_name(&container, PartitionKey::from(pk), id);
        let body = serde_json::json!({"id": id, "pk": pk, "rank": 7});
        driver
            .execute_singleton_operation(
                CosmosOperation::create_item(item_ref)
                    .with_body(serde_json::to_vec(&body).unwrap()),
                OperationOptions::default(),
            )
            .await
            .expect("seed item created");
    }
    let mut expected_ids: Vec<String> = seeds.iter().map(|(id, _)| id.to_string()).collect();
    expected_ids.sort();

    let asc_ids = Box::pin(run_query_collecting_ids(
        &driver,
        &container,
        "SELECT * FROM c ORDER BY c.rank ASC",
    ))
    .await;
    let asc_ids_again = Box::pin(run_query_collecting_ids(
        &driver,
        &container,
        "SELECT * FROM c ORDER BY c.rank ASC",
    ))
    .await;
    assert_eq!(asc_ids, asc_ids_again, "ASC ties must be deterministic");
    let mut sorted_asc = asc_ids.clone();
    sorted_asc.sort();
    assert_eq!(
        sorted_asc, expected_ids,
        "cross-partition ASC ties must return every row exactly once: {asc_ids:?}"
    );

    let desc_ids = Box::pin(run_query_collecting_ids(
        &driver,
        &container,
        "SELECT * FROM c ORDER BY c.rank DESC",
    ))
    .await;
    let desc_ids_again = Box::pin(run_query_collecting_ids(
        &driver,
        &container,
        "SELECT * FROM c ORDER BY c.rank DESC",
    ))
    .await;
    assert_eq!(desc_ids, desc_ids_again, "DESC ties must be deterministic");
    let mut sorted_desc = desc_ids.clone();
    sorted_desc.sort();
    assert_eq!(
        sorted_desc, expected_ids,
        "cross-partition DESC ties must return every row exactly once: {desc_ids:?}"
    );
}

/// A fresh cross-partition streaming `ORDER BY` plan is subject to the same
/// `max_fan_out` ceiling as every other plan shape. The merge holds all of
/// its children open concurrently, so leaking past the limit here is worse
/// than on the sequential drain.
#[tokio::test]
async fn cross_partition_order_by_enforces_max_fan_out() {
    let (_emulator, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .expect("container resolves");

    let operation = CosmosOperation::query_items(container.clone(), Some(FeedRange::full()))
        .with_body(br#"{"query":"SELECT * FROM c ORDER BY c.rank ASC","parameters":[]}"#.to_vec());

    // The container has two physical partitions, so a ceiling of one must trip.
    let result = Box::pin(driver.plan_operation(
        operation,
        &OperationOptions::default(),
        None,
        &PlanOptions::default().with_max_fan_out(1),
    ))
    .await;

    let err = match result {
        Ok(_) => panic!("a two-partition ORDER BY plan must exceed a max_fan_out of 1"),
        Err(err) => err,
    };

    assert_eq!(
        err.status().sub_status(),
        Some(
            azure_data_cosmos_driver::error::SubStatusCode::CLIENT_CROSS_PARTITION_FAN_OUT_EXCEEDED
        ),
        "unexpected error: {err}",
    );
}

// ---------------------------------------------------------------------------
// Combined ORDER BY + OFFSET / LIMIT / TOP
//
// These exercise the composition added in #4750: the streaming ORDER BY merge
// wrapped in a global `SkipTake`. Unlike the pure OFFSET/LIMIT/TOP tests
// (`skip_take.rs`), which can only assert a deterministic *count* across
// partitions because natural-order fan-out has no defined cross-partition
// order, ORDER BY makes the merged stream globally sorted — so the window is
// deterministic *across partitions* and we assert the exact ordered result.
// ---------------------------------------------------------------------------

/// Seeds `(id, pk, rank)` documents in the given order.
async fn seed_ranked(
    driver: &azure_data_cosmos_driver::driver::CosmosDriver,
    container: &azure_data_cosmos_driver::models::ContainerReference,
    seeds: &[(&str, &str, i64)],
) {
    for (id, pk, rank) in seeds {
        let item_ref = ItemReference::from_name(
            container,
            PartitionKey::from(pk.to_string()),
            id.to_string(),
        );
        let body = serde_json::json!({"id": id, "pk": pk, "rank": rank});
        driver
            .execute_singleton_operation(
                CosmosOperation::create_item(item_ref)
                    .with_body(serde_json::to_vec(&body).unwrap()),
                OperationOptions::default(),
            )
            .await
            .expect("seed item created");
    }
}

/// Runs `query` to completion and returns every result's `rank`, in emitted
/// order, honoring an optional `max_item_count` page-size hint.
async fn run_query_collecting_ranks(
    driver: &azure_data_cosmos_driver::driver::CosmosDriver,
    container: &azure_data_cosmos_driver::models::ContainerReference,
    query: &str,
    page_size: Option<u32>,
) -> Vec<i64> {
    let body = serde_json::to_vec(&serde_json::json!({"query": query, "parameters": []})).unwrap();
    let mut operation =
        CosmosOperation::query_items(container.clone(), Some(FeedRange::full())).with_body(body);
    if let Some(n) = page_size {
        operation =
            operation.with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(n).unwrap()));
    }
    let mut plan = Box::pin(driver.plan_operation(
        operation,
        &OperationOptions::default(),
        None,
        &PlanOptions::default(),
    ))
    .await
    .expect("plan builds a combined ORDER BY + window pipeline");

    let mut ranks = Vec::new();
    while let Some(response) = driver
        .execute_plan(
            &mut plan,
            Some(container.clone()),
            OperationOptions::default(),
        )
        .await
        .expect("page executes")
    {
        for item in &super::page_document_values(response) {
            ranks.push(item["rank"].as_i64().unwrap());
        }
    }
    ranks
}

/// Ranks shuffled across two physical partitions, so a window over the merged
/// stream is only correct if global ordering is applied *before* the skip/take.
const SHUFFLED_SIX: &[(&str, &str, i64)] = &[
    ("d1", "pk-a", 5),
    ("d2", "pk-b", 1),
    ("d3", "pk-c", 4),
    ("d4", "pk-d", 2),
    ("d5", "pk-e", 3),
    ("d6", "pk-f", 0),
];

#[tokio::test]
async fn cross_partition_order_by_offset_limit_returns_exact_window() {
    let (_emulator, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .expect("container resolves");
    seed_ranked(&driver, &container, SHUFFLED_SIX).await;

    let ranks = run_query_collecting_ranks(
        &driver,
        &container,
        "SELECT * FROM c ORDER BY c.rank ASC OFFSET 2 LIMIT 3",
        None,
    )
    .await;

    assert_eq!(
        ranks,
        vec![2, 3, 4],
        "ORDER BY ASC OFFSET 2 LIMIT 3 must return the exact global window across partitions"
    );
}

#[tokio::test]
async fn cross_partition_order_by_top_returns_exact_prefix() {
    let (_emulator, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .expect("container resolves");
    seed_ranked(&driver, &container, SHUFFLED_SIX).await;

    let ranks = run_query_collecting_ranks(
        &driver,
        &container,
        "SELECT TOP 3 * FROM c ORDER BY c.rank ASC",
        None,
    )
    .await;

    assert_eq!(
        ranks,
        vec![0, 1, 2],
        "TOP 3 with ORDER BY ASC must return the three globally-smallest ranks"
    );
}

#[tokio::test]
async fn cross_partition_order_by_desc_offset_limit_returns_exact_window() {
    let (_emulator, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .expect("container resolves");
    seed_ranked(&driver, &container, SHUFFLED_SIX).await;

    let ranks = run_query_collecting_ranks(
        &driver,
        &container,
        "SELECT * FROM c ORDER BY c.rank DESC OFFSET 1 LIMIT 2",
        None,
    )
    .await;

    assert_eq!(
        ranks,
        vec![4, 3],
        "ORDER BY DESC OFFSET 1 LIMIT 2 must skip the largest then take the next two"
    );
}

#[tokio::test]
async fn cross_partition_order_by_offset_beyond_total_is_empty() {
    let (_emulator, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .expect("container resolves");
    seed_ranked(&driver, &container, SHUFFLED_SIX).await;

    let ranks = run_query_collecting_ranks(
        &driver,
        &container,
        "SELECT * FROM c ORDER BY c.rank ASC OFFSET 10 LIMIT 5",
        None,
    )
    .await;

    assert!(
        ranks.is_empty(),
        "an OFFSET past the total must be empty even with ORDER BY: {ranks:?}"
    );
}

/// The window must be preserved and globally ordered even when the ordered
/// merge paginates: a small page size forces the `SkipTake` to consume the
/// `StreamingOrderedMerge` across several pages, round-tripping a nested
/// `SkipTake { child: StreamingOrderedMerge }` continuation between pages.
#[tokio::test]
async fn cross_partition_order_by_offset_limit_paginates_preserving_window() {
    let (_emulator, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .expect("container resolves");

    // Eight ranks 1..=8, shuffled across partitions.
    let seeds = [
        ("d1", "pk-a", 8),
        ("d2", "pk-b", 3),
        ("d3", "pk-c", 6),
        ("d4", "pk-d", 1),
        ("d5", "pk-e", 5),
        ("d6", "pk-f", 2),
        ("d7", "pk-g", 7),
        ("d8", "pk-h", 4),
    ];
    seed_ranked(&driver, &container, &seeds).await;

    let ranks = run_query_collecting_ranks(
        &driver,
        &container,
        "SELECT * FROM c ORDER BY c.rank ASC OFFSET 2 LIMIT 4",
        Some(2),
    )
    .await;

    assert_eq!(
        ranks,
        vec![3, 4, 5, 6],
        "a paginated combined query must yield the exact global window in order"
    );
}

/// A physical partition split *mid-query* must not corrupt a combined
/// ORDER BY + window result. The streaming merge absorbs the split internally
/// (re-fanning to the child ranges via `split_for_topology_change`), and the
/// wrapping `SkipTake` keeps applying the single global window over the
/// re-merged stream. With a page size of 1 the split lands after the first
/// emitted document, so the resume path crosses the topology change.
#[tokio::test]
async fn cross_partition_order_by_offset_limit_survives_split_mid_stream() {
    let (emulator, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .expect("container resolves");

    // Eight ranks 1..=8, shuffled across partitions.
    let seeds = [
        ("d1", "pk-a", 8),
        ("d2", "pk-b", 3),
        ("d3", "pk-c", 6),
        ("d4", "pk-d", 1),
        ("d5", "pk-e", 5),
        ("d6", "pk-f", 2),
        ("d7", "pk-g", 7),
        ("d8", "pk-h", 4),
    ];
    seed_ranked(&driver, &container, &seeds).await;

    let body = serde_json::to_vec(&serde_json::json!({
        "query": "SELECT * FROM c ORDER BY c.rank ASC OFFSET 2 LIMIT 4",
        "parameters": [],
    }))
    .unwrap();
    let operation = CosmosOperation::query_items(container.clone(), Some(FeedRange::full()))
        .with_body(body)
        .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(1).unwrap()));

    let mut plan = Box::pin(driver.plan_operation(
        operation,
        &OperationOptions::default(),
        None,
        &PlanOptions::default(),
    ))
    .await
    .expect("plan builds a combined ORDER BY + window pipeline");

    let mut ranks = Vec::new();
    let mut split_injected = false;
    while let Some(response) = driver
        .execute_plan(
            &mut plan,
            Some(container.clone()),
            OperationOptions::default(),
        )
        .await
        .expect("page executes")
    {
        for item in &super::page_document_values(response) {
            ranks.push(item["rank"].as_i64().unwrap());
        }

        // After the very first emitted page, split a physical partition so the
        // remaining pages must resume across the topology change.
        if !split_injected {
            let store = emulator.store();
            store.split_partition("testdb", "testcoll", 0, Duration::ZERO);
            store.drain_pending_control_plane().await;
            assert_eq!(
                store.child_partition_ids("testdb", "testcoll", &[0]).len(),
                2,
                "the split must materialize two child partitions before we resume"
            );
            split_injected = true;
        }
    }

    assert!(
        split_injected,
        "the query must page at least once so the split lands mid-stream"
    );
    assert_eq!(
        ranks,
        vec![3, 4, 5, 6],
        "a combined ORDER BY + window query must preserve the exact global window \
         across a mid-stream partition split"
    );
}
