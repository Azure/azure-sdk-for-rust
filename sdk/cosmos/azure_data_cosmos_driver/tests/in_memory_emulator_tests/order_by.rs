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
//! Scoped to *fresh* (non-resumed) scenarios; resume/split behavior is
//! covered by the mock-pipeline and driver-level integration tests.

use std::sync::Arc;

use azure_core::http::Url;

use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, ContainerConfig, InMemoryEmulatorHttpClient, VirtualAccountConfig,
    VirtualRegion,
};
use azure_data_cosmos_driver::models::{
    CosmosOperation, FeedRange, ItemReference, MaxItemCountHint, PartitionKey,
    PartitionKeyDefinition,
};
use azure_data_cosmos_driver::options::{DriverOptions, OperationOptions};

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
        .resolve_container("testdb", "testcoll")
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

    let mut plan = driver
        .plan_operation(operation, &OperationOptions::default(), None)
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
        let body: serde_json::Value =
            serde_json::from_slice(&response.into_body().single().unwrap()).unwrap();
        for item in body["Documents"].as_array().unwrap() {
            ranks.push(item["rank"].as_i64().unwrap());
        }
    }

    assert_eq!(ranks, vec![0, 1, 2, 3, 4, 5]);
}

#[tokio::test]
async fn cross_partition_order_by_paginates_with_small_page_size() {
    let (_emulator, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll")
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

    let mut plan = driver
        .plan_operation(operation, &OperationOptions::default(), None)
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
        let body: serde_json::Value =
            serde_json::from_slice(&response.into_body().single().unwrap()).unwrap();
        let documents = body["Documents"].as_array().unwrap();
        page_sizes.push(documents.len());
        for item in documents {
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
    let mut plan = driver
        .plan_operation(operation, &OperationOptions::default(), None)
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
        let body: serde_json::Value =
            serde_json::from_slice(&response.into_body().single().unwrap()).unwrap();
        for item in body["Documents"].as_array().unwrap() {
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
        .resolve_container("testdb", "testcoll")
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

    let asc_ids =
        run_query_collecting_ids(&driver, &container, "SELECT * FROM c ORDER BY c.rank ASC").await;
    assert_eq!(
        asc_ids,
        vec!["bbb", "aaa", "ccc"],
        "ASC ties must follow creation (rid) order, not the store's alphabetical-by-id \
         iteration order: {asc_ids:?}"
    );

    let desc_ids =
        run_query_collecting_ids(&driver, &container, "SELECT * FROM c ORDER BY c.rank DESC").await;
    assert_eq!(
        desc_ids,
        vec!["ccc", "aaa", "bbb"],
        "DESC ties must follow reverse creation (rid) order: {desc_ids:?}"
    );
}

/// Regression: a full-key tie spanning *different* physical partitions
/// must also be broken by document `_rid` in the driver's cross-partition
/// merge — not by whichever partition happens to be polled/arrive first.
/// Seeded out of both alphabetical-id and partition-key order so neither
/// coincidentally matches rid (creation) order.
#[tokio::test]
async fn tied_order_by_key_across_partitions_returns_rid_order() {
    let (_emulator, driver) = setup().await;
    let container = driver
        .resolve_container("testdb", "testcoll")
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
    let creation_order: Vec<String> = seeds.iter().map(|(id, _)| id.to_string()).collect();

    let asc_ids =
        run_query_collecting_ids(&driver, &container, "SELECT * FROM c ORDER BY c.rank ASC").await;
    assert_eq!(
        asc_ids, creation_order,
        "cross-partition ASC ties must follow creation (rid) order: {asc_ids:?}"
    );

    let desc_ids =
        run_query_collecting_ids(&driver, &container, "SELECT * FROM c ORDER BY c.rank DESC").await;
    let mut expected_desc = creation_order;
    expected_desc.reverse();
    assert_eq!(
        desc_ids, expected_desc,
        "cross-partition DESC ties must follow reverse creation (rid) order: {desc_ids:?}"
    );
}
