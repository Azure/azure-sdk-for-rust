// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! End-to-end tests that exercise the full driver pipeline through the emulator.
//!
//! These tests use `CosmosDriver::execute_operation()` — the same entry point
//! that real application code uses — with the emulator replacing HTTP I/O.
//! This verifies the entire stack: endpoint resolution, session routing, retry
//! logic, authorization headers, transport pipeline, and the emulator itself.

use azure_core::http::Url;
use azure_data_cosmos_driver::{
    in_memory_emulator::{
        ConsistencyLevel, InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion,
    },
    models::{AccountReference, CosmosOperation, DatabaseReference, ItemReference, PartitionKey},
    options::OperationOptions,
};

const GATEWAY_URL: &str = "https://eastus.emulator.local";

/// Sets up the emulator, builds a real `CosmosDriverRuntime` wired to it,
/// and returns a driver ready for `execute_operation` calls.
async fn setup_driver() -> (
    std::sync::Arc<azure_data_cosmos_driver::CosmosDriver>,
    std::sync::Arc<azure_data_cosmos_driver::in_memory_emulator::EmulatorStore>,
) {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .with_consistency(ConsistencyLevel::Session);

    let emulator = InMemoryEmulatorHttpClient::new(config);
    let store = emulator.store();

    // Pre-provision database and container in the emulator store
    store.create_database("testdb");
    store.create_container(
        "testdb",
        "testcoll",
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))
        .unwrap(),
    );

    // Build a real runtime wired to the emulator
    let runtime = emulator.runtime_builder().build().await.unwrap();

    // Create a driver for the emulator endpoint
    let account = AccountReference::with_master_key(
        Url::parse(GATEWAY_URL).unwrap(),
        // The emulator ignores auth, but the driver requires a key
        "dGVzdGtleQ==",
    );
    let driver = runtime.get_or_create_driver(account, None).await.unwrap();

    (driver, store)
}

#[tokio::test]
async fn create_and_read_item_through_driver() {
    let (driver, _store) = setup_driver().await;

    // Resolve the container through the driver (goes through the emulator's
    // ReadContainer handler)
    let container = driver
        .resolve_container("testdb", "testcoll")
        .await
        .unwrap();

    // Create an item using the full driver pipeline
    let pk = PartitionKey::from("pk1");
    let body = serde_json::to_vec(&serde_json::json!({
        "id": "driver-item-1",
        "pk": "pk1",
        "value": 42
    }))
    .unwrap();

    let create_result = driver
        .execute_operation(
            CosmosOperation::create_item(container.clone(), pk.clone()).with_body(body),
            OperationOptions::new(),
        )
        .await
        .unwrap();

    assert_eq!(
        u16::from(create_result.status().status_code()),
        201,
        "Create should return 201 Created"
    );

    // Read the item back through the driver
    let item = ItemReference::from_name(&container, PartitionKey::from("pk1"), "driver-item-1");
    let read_result = driver
        .execute_operation(CosmosOperation::read_item(item), OperationOptions::new())
        .await
        .unwrap();

    assert_eq!(
        u16::from(read_result.status().status_code()),
        200,
        "Read should return 200 OK"
    );

    // Verify the body contains our data
    let doc: serde_json::Value = serde_json::from_slice(read_result.body()).unwrap();
    assert_eq!(doc["id"], "driver-item-1");
    assert_eq!(doc["value"], 42);
    assert!(
        doc.get("_rid").is_some(),
        "Should have _rid system property"
    );
    assert!(
        doc.get("_etag").is_some(),
        "Should have _etag system property"
    );
}

#[tokio::test]
async fn create_database_and_container_through_driver() {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .with_consistency(ConsistencyLevel::Session);

    let emulator = InMemoryEmulatorHttpClient::new(config);
    let runtime = emulator.runtime_builder().build().await.unwrap();
    let account =
        AccountReference::with_master_key(Url::parse(GATEWAY_URL).unwrap(), "dGVzdGtleQ==");
    let driver = runtime
        .get_or_create_driver(account.clone(), None)
        .await
        .unwrap();

    // Create database through the driver pipeline
    let db_body = serde_json::to_vec(&serde_json::json!({"id": "driverdb"})).unwrap();
    let create_db = driver
        .execute_operation(
            CosmosOperation::create_database(account.clone()).with_body(db_body),
            OperationOptions::new(),
        )
        .await
        .unwrap();

    assert_eq!(
        u16::from(create_db.status().status_code()),
        201,
        "Create database should return 201"
    );

    // Create container through the driver pipeline
    let db_ref = DatabaseReference::from_name(account, "driverdb");
    let coll_body = serde_json::to_vec(&serde_json::json!({
        "id": "drivercoll",
        "partitionKey": {"paths": ["/pk"], "kind": "Hash", "version": 2}
    }))
    .unwrap();

    let create_coll = driver
        .execute_operation(
            CosmosOperation::create_container(db_ref).with_body(coll_body),
            OperationOptions::new(),
        )
        .await
        .unwrap();

    assert_eq!(
        u16::from(create_coll.status().status_code()),
        201,
        "Create container should return 201"
    );

    // Verify we can resolve the container
    let _container = driver
        .resolve_container("driverdb", "drivercoll")
        .await
        .unwrap();
}

#[tokio::test]
async fn delete_item_through_driver() {
    let (driver, _store) = setup_driver().await;
    let container = driver
        .resolve_container("testdb", "testcoll")
        .await
        .unwrap();

    // Create
    let pk = PartitionKey::from("pk1");
    let body = serde_json::to_vec(&serde_json::json!({
        "id": "delete-me",
        "pk": "pk1",
        "value": 1
    }))
    .unwrap();

    driver
        .execute_operation(
            CosmosOperation::create_item(container.clone(), pk.clone()).with_body(body),
            OperationOptions::new(),
        )
        .await
        .unwrap();

    // Delete
    let item = ItemReference::from_name(&container, pk, "delete-me");
    let delete_result = driver
        .execute_operation(CosmosOperation::delete_item(item), OperationOptions::new())
        .await
        .unwrap();

    assert_eq!(
        u16::from(delete_result.status().status_code()),
        204,
        "Delete should return 204 No Content"
    );

    // Verify item is gone
    let item = ItemReference::from_name(&container, PartitionKey::from("pk1"), "delete-me");
    let read_result = driver
        .execute_operation(CosmosOperation::read_item(item), OperationOptions::new())
        .await;

    assert!(read_result.is_err(), "Reading deleted item should fail");
}
