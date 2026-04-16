// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! End-to-end tests that exercise the full driver pipeline through the emulator
//! **and** (optionally) a real Cosmos DB account.
//!
//! Each test:
//! 1. Runs the same sequence of operations against the in-memory emulator.
//! 2. When `AZURE_COSMOS_CONNECTION_STRING` is set, repeats the operations
//!    against a real account.
//! 3. Compares status codes, Cosmos headers, and response payloads between
//!    the two backends (real → emulator) using per-header validation rules.
//!
//! See [`super::validation`] for the header/body comparison rules.

use azure_data_cosmos_driver::models::{
    CosmosOperation, DatabaseReference, ItemReference, PartitionKey,
};
use azure_data_cosmos_driver::options::{OperationOptions, OperationOptionsBuilder};

use super::dual_backend::DualBackend;
use super::validation::{
    compare_responses, BodyValidationSpec, HeaderValidationSpec, ResponseSnapshot,
};

/// Sets up both backends with a shared database and container.
///
/// Returns `(backend, db_name, emulator_container, Option<real_container>)`.
async fn setup_with_container() -> (
    DualBackend,
    String,
    azure_data_cosmos_driver::models::ContainerReference,
    Option<azure_data_cosmos_driver::models::ContainerReference>,
) {
    let backend = DualBackend::setup().await.unwrap();
    let db_name = backend.unique_db_name();
    let container_name = "testcoll";
    let pk_path = "/pk";

    // Provision emulator
    backend.provision_emulator(&db_name, container_name, pk_path);

    // Provision real account (if available)
    if backend.has_real_backend() {
        backend.create_real_database(&db_name).await.unwrap();
        backend
            .create_real_container(&db_name, container_name, pk_path)
            .await
            .unwrap();
    }

    // Resolve containers
    let emu_container = backend
        .emulator_driver
        .resolve_container(&db_name, container_name)
        .await
        .unwrap();

    let real_container = if let Some(ref real_driver) = backend.real_driver {
        Some(
            real_driver
                .resolve_container(&db_name, container_name)
                .await
                .unwrap(),
        )
    } else {
        None
    };

    (backend, db_name, emu_container, real_container)
}

#[tokio::test]
async fn create_and_read_item_through_driver() {
    let (backend, db_name, emu_container, real_container) = setup_with_container().await;

    // ── Create item ──────────────────────────────────────────────
    let item_body = serde_json::json!({
        "id": "driver-item-1",
        "pk": "pk1",
        "value": 42
    });
    let body_bytes = serde_json::to_vec(&item_body).unwrap();

    let (emu_create, real_create) = backend
        .execute_and_compare(
            &emu_container,
            real_container.as_ref(),
            |container| {
                let op = CosmosOperation::create_item(container.clone(), PartitionKey::from("pk1"))
                    .with_body(body_bytes.clone());
                (op, OperationOptions::default())
            },
            &HeaderValidationSpec::for_point_operation(),
            BodyValidationSpec::DocumentMatch,
        )
        .await
        .unwrap();

    assert_eq!(
        u16::from(emu_create.status().status_code()),
        201,
        "Emulator create should return 201 Created",
    );
    if let Some(ref real) = real_create {
        assert_eq!(
            u16::from(real.status().status_code()),
            201,
            "Real create should return 201 Created",
        );
    }

    // ── Read item back ───────────────────────────────────────────
    let (emu_read, real_read) = backend
        .execute_and_compare(
            &emu_container,
            real_container.as_ref(),
            |container| {
                let item =
                    ItemReference::from_name(container, PartitionKey::from("pk1"), "driver-item-1");
                (
                    CosmosOperation::read_item(item),
                    OperationOptions::default(),
                )
            },
            &HeaderValidationSpec::for_point_operation(),
            BodyValidationSpec::DocumentMatch,
        )
        .await
        .unwrap();

    assert_eq!(
        u16::from(emu_read.status().status_code()),
        200,
        "Emulator read should return 200 OK",
    );

    // Verify emulator body structure
    let doc: serde_json::Value = serde_json::from_slice(emu_read.body()).unwrap();
    assert_eq!(doc["id"], "driver-item-1");
    assert_eq!(doc["value"], 42);
    assert!(
        doc.get("_rid").is_some(),
        "Should have _rid system property"
    );
    assert!(
        doc.get("_etag").is_some(),
        "Should have _etag system property",
    );

    if let Some(ref real) = real_read {
        let real_doc: serde_json::Value = serde_json::from_slice(real.body()).unwrap();
        assert_eq!(real_doc["id"], "driver-item-1");
        assert_eq!(real_doc["value"], 42);
    }

    // Cleanup
    backend.cleanup_real_database(&db_name).await;
}

#[tokio::test]
async fn create_database_and_container_through_driver() {
    let backend = DualBackend::setup().await.unwrap();
    let db_name = format!("dual-cp-{}", &backend.run_id);
    let container_name = "drivercoll";

    // ── Create database ──────────────────────────────────────────
    let db_body = serde_json::to_vec(&serde_json::json!({"id": &db_name})).unwrap();

    let (emu_create_db, real_create_db) = backend
        .execute_account_op_and_compare(
            |account| {
                let op =
                    CosmosOperation::create_database(account.clone()).with_body(db_body.clone());
                (op, OperationOptions::default())
            },
            &HeaderValidationSpec::for_control_plane(),
            BodyValidationSpec::StructuralMatch,
        )
        .await
        .unwrap();

    assert_eq!(
        u16::from(emu_create_db.status().status_code()),
        201,
        "Emulator create DB should return 201",
    );
    if let Some(ref real) = real_create_db {
        assert_eq!(
            u16::from(real.status().status_code()),
            201,
            "Real create DB should return 201",
        );
    }

    // ── Create container ─────────────────────────────────────────
    let coll_body = serde_json::to_vec(&serde_json::json!({
        "id": container_name,
        "partitionKey": {"paths": ["/pk"], "kind": "Hash", "version": 2}
    }))
    .unwrap();

    // Build operations separately for each backend since DatabaseReference
    // needs backend-specific account references.
    let emu_db_ref =
        DatabaseReference::from_name(backend.emulator_account.clone(), db_name.clone());
    let emu_create_coll_op =
        CosmosOperation::create_container(emu_db_ref).with_body(coll_body.clone());
    let emu_create_coll = backend
        .emulator_driver
        .execute_operation(emu_create_coll_op, OperationOptions::default())
        .await
        .unwrap();

    let real_create_coll = if let (Some(ref driver), Some(ref account)) =
        (&backend.real_driver, &backend.real_account)
    {
        let real_db_ref = DatabaseReference::from_name(account.clone(), db_name.clone());
        let real_op = CosmosOperation::create_container(real_db_ref).with_body(coll_body.clone());
        let resp = driver
            .execute_operation(real_op, OperationOptions::default())
            .await
            .unwrap();
        Some(resp)
    } else {
        None
    };

    assert_eq!(
        u16::from(emu_create_coll.status().status_code()),
        201,
        "Emulator create container should return 201",
    );

    // Compare create-container responses
    if let Some(ref real_resp) = real_create_coll {
        assert_eq!(
            u16::from(real_resp.status().status_code()),
            201,
            "Real create container should return 201",
        );
        let real_snap = ResponseSnapshot::capture(real_resp, "real");
        let emu_snap = ResponseSnapshot::capture(&emu_create_coll, "emulator");
        compare_responses(
            &real_snap,
            &emu_snap,
            &HeaderValidationSpec::for_control_plane(),
            BodyValidationSpec::StructuralMatch,
        );
    }

    // Verify container is resolvable on emulator
    let _emu_coll = backend
        .emulator_driver
        .resolve_container(&db_name, container_name)
        .await
        .unwrap();

    // Cleanup
    backend.cleanup_real_database(&db_name).await;
}

#[tokio::test]
async fn delete_item_through_driver() {
    let (backend, db_name, emu_container, real_container) = setup_with_container().await;

    // ── Create item ──────────────────────────────────────────────
    let item_body = serde_json::json!({
        "id": "delete-me",
        "pk": "pk1",
        "value": 1
    });
    let body_bytes = serde_json::to_vec(&item_body).unwrap();

    backend
        .execute_and_compare(
            &emu_container,
            real_container.as_ref(),
            |container| {
                let op = CosmosOperation::create_item(container.clone(), PartitionKey::from("pk1"))
                    .with_body(body_bytes.clone());
                (op, OperationOptions::default())
            },
            &HeaderValidationSpec::for_point_operation(),
            BodyValidationSpec::DocumentMatch,
        )
        .await
        .unwrap();

    // ── Delete item ──────────────────────────────────────────────
    let (emu_delete, real_delete) = backend
        .execute_and_compare(
            &emu_container,
            real_container.as_ref(),
            |container| {
                let item =
                    ItemReference::from_name(container, PartitionKey::from("pk1"), "delete-me");
                (
                    CosmosOperation::delete_item(item),
                    OperationOptions::default(),
                )
            },
            &HeaderValidationSpec::for_delete_operation(),
            BodyValidationSpec::Ignore,
        )
        .await
        .unwrap();

    assert_eq!(
        u16::from(emu_delete.status().status_code()),
        204,
        "Emulator delete should return 204 No Content",
    );
    if let Some(ref real) = real_delete {
        assert_eq!(
            u16::from(real.status().status_code()),
            204,
            "Real delete should return 204 No Content",
        );
    }

    // ── Verify item is gone (emulator) ───────────────────────────
    let emu_read_deleted = backend
        .emulator_driver
        .execute_operation(
            CosmosOperation::read_item(ItemReference::from_name(
                &emu_container,
                PartitionKey::from("pk1"),
                "delete-me",
            )),
            OperationOptions::default(),
        )
        .await;
    assert!(
        emu_read_deleted.is_err(),
        "Emulator: reading deleted item should fail",
    );

    // ── Verify item is gone (real) ───────────────────────────────
    if let (Some(ref driver), Some(ref real_ctr)) = (&backend.real_driver, &real_container) {
        let real_read_deleted = driver
            .execute_operation(
                CosmosOperation::read_item(ItemReference::from_name(
                    real_ctr,
                    PartitionKey::from("pk1"),
                    "delete-me",
                )),
                OperationOptions::default(),
            )
            .await;
        assert!(
            real_read_deleted.is_err(),
            "Real: reading deleted item should fail",
        );
    }

    // Cleanup
    backend.cleanup_real_database(&db_name).await;
}

#[tokio::test]
async fn replace_item_through_driver() {
    let (backend, db_name, emu_container, real_container) = setup_with_container().await;

    // ── Create item ──────────────────────────────────────────────
    let create_body = serde_json::to_vec(&serde_json::json!({
        "id": "replace-me",
        "pk": "pk1",
        "value": 1
    }))
    .unwrap();

    backend
        .execute_and_compare(
            &emu_container,
            real_container.as_ref(),
            |container| {
                let op = CosmosOperation::create_item(container.clone(), PartitionKey::from("pk1"))
                    .with_body(create_body.clone());
                (op, OperationOptions::default())
            },
            &HeaderValidationSpec::for_point_operation(),
            BodyValidationSpec::DocumentMatch,
        )
        .await
        .unwrap();

    // ── Replace item ─────────────────────────────────────────────
    let replace_body = serde_json::to_vec(&serde_json::json!({
        "id": "replace-me",
        "pk": "pk1",
        "value": 99
    }))
    .unwrap();

    let (emu_replace, real_replace) = backend
        .execute_and_compare(
            &emu_container,
            real_container.as_ref(),
            |container| {
                let item =
                    ItemReference::from_name(container, PartitionKey::from("pk1"), "replace-me");
                let op = CosmosOperation::replace_item(item).with_body(replace_body.clone());
                (op, OperationOptions::default())
            },
            &HeaderValidationSpec::for_point_operation(),
            BodyValidationSpec::DocumentMatch,
        )
        .await
        .unwrap();

    assert_eq!(
        u16::from(emu_replace.status().status_code()),
        200,
        "Emulator replace should return 200",
    );

    // Verify updated value via read
    let (emu_read, _) = backend
        .execute_and_compare(
            &emu_container,
            real_container.as_ref(),
            |container| {
                let item =
                    ItemReference::from_name(container, PartitionKey::from("pk1"), "replace-me");
                (
                    CosmosOperation::read_item(item),
                    OperationOptions::default(),
                )
            },
            &HeaderValidationSpec::for_point_operation(),
            BodyValidationSpec::DocumentMatch,
        )
        .await
        .unwrap();

    let doc: serde_json::Value = serde_json::from_slice(emu_read.body()).unwrap();
    assert_eq!(doc["value"], 99, "value should be updated to 99");

    if let Some(ref real) = real_replace {
        assert_eq!(u16::from(real.status().status_code()), 200);
    }

    // Cleanup
    backend.cleanup_real_database(&db_name).await;
}

#[tokio::test]
async fn read_with_stale_session_token_returns_404_1002() {
    let (backend, db_name, emu_container, real_container) = setup_with_container().await;

    // Read a non-existent item with a session token whose LSN is far ahead
    // of the partition's actual LSN. This forces a 404/1002
    // ReadSessionNotAvailable on both backends.
    //
    // Disable session retries so the error propagates immediately.
    let stale_token = "0:-1#999999";
    let opts = OperationOptionsBuilder::new()
        .with_max_session_retry_count(0)
        .build();

    // ── Emulator ─────────────────────────────────────────────────
    let emu_err = backend
        .emulator_driver
        .execute_operation(
            CosmosOperation::read_item(ItemReference::from_name(
                &emu_container,
                PartitionKey::from("pk1"),
                "no-such-item",
            ))
            .with_session_token(stale_token),
            opts.clone(),
        )
        .await;

    let emu_err = emu_err.expect_err("Emulator should return an error for stale session read");
    assert_eq!(
        emu_err.http_status(),
        Some(azure_core::http::StatusCode::NotFound),
        "Emulator error should be HTTP 404",
    );
    match emu_err.kind() {
        azure_core::error::ErrorKind::HttpResponse { error_code, .. } => {
            assert_eq!(
                error_code.as_deref(),
                Some("1002"),
                "Emulator error should have substatus 1002",
            );
        }
        other => panic!("Expected HttpResponse error, got: {other}"),
    }

    // ── Real account (if available) ──────────────────────────────
    if let (Some(ref driver), Some(ref real_ctr)) = (&backend.real_driver, &real_container) {
        let real_err = driver
            .execute_operation(
                CosmosOperation::read_item(ItemReference::from_name(
                    real_ctr,
                    PartitionKey::from("pk1"),
                    "no-such-item",
                ))
                .with_session_token(stale_token),
                opts.clone(),
            )
            .await;

        let real_err = real_err.expect_err("Real should return an error for stale session read");
        assert_eq!(
            real_err.http_status(),
            Some(azure_core::http::StatusCode::NotFound),
            "Real error should be HTTP 404",
        );
        match real_err.kind() {
            azure_core::error::ErrorKind::HttpResponse { error_code, .. } => {
                assert_eq!(
                    error_code.as_deref(),
                    Some("1002"),
                    "Real error should have substatus 1002",
                );
            }
            other => panic!("Expected HttpResponse error, got: {other}"),
        }
    }

    // Cleanup
    backend.cleanup_real_database(&db_name).await;
}

// TODO: upsert_item_through_driver is disabled because `CosmosOperation::upsert_item`
// currently sends POST to /dbs/{db}/colls/{coll}/docs/{doc_id} (the item-level path),
// but the Cosmos REST API and the emulator expect POST to the collection feed endpoint
// /dbs/{db}/colls/{coll}/docs with the `x-ms-documentdb-is-upsert: True` header.
// Re-enable once the driver or emulator upsert routing is fixed.
#[tokio::test]
#[ignore = "upsert routing mismatch between driver and emulator — see TODO above"]
async fn upsert_item_through_driver() {
    let (backend, db_name, emu_container, real_container) = setup_with_container().await;

    let upsert_body = serde_json::to_vec(&serde_json::json!({
        "id": "upsert-item",
        "pk": "pk1",
        "value": 10
    }))
    .unwrap();

    // ── Upsert (insert) ─────────────────────────────────────────
    let (emu_upsert1, real_upsert1) = backend
        .execute_and_compare(
            &emu_container,
            real_container.as_ref(),
            |container| {
                let item =
                    ItemReference::from_name(container, PartitionKey::from("pk1"), "upsert-item");
                let op = CosmosOperation::upsert_item(item).with_body(upsert_body.clone());
                (op, OperationOptions::default())
            },
            &HeaderValidationSpec::for_point_operation(),
            BodyValidationSpec::DocumentMatch,
        )
        .await
        .unwrap();

    assert_eq!(
        u16::from(emu_upsert1.status().status_code()),
        201,
        "Emulator upsert-as-insert should return 201",
    );
    if let Some(ref real) = real_upsert1 {
        assert_eq!(u16::from(real.status().status_code()), 201);
    }

    // ── Upsert (update) ─────────────────────────────────────────
    let upsert_body2 = serde_json::to_vec(&serde_json::json!({
        "id": "upsert-item",
        "pk": "pk1",
        "value": 20
    }))
    .unwrap();

    let (emu_upsert2, real_upsert2) = backend
        .execute_and_compare(
            &emu_container,
            real_container.as_ref(),
            |container| {
                let item =
                    ItemReference::from_name(container, PartitionKey::from("pk1"), "upsert-item");
                let op = CosmosOperation::upsert_item(item).with_body(upsert_body2.clone());
                (op, OperationOptions::default())
            },
            &HeaderValidationSpec::for_point_operation(),
            BodyValidationSpec::DocumentMatch,
        )
        .await
        .unwrap();

    assert_eq!(
        u16::from(emu_upsert2.status().status_code()),
        200,
        "Emulator upsert-as-update should return 200",
    );
    if let Some(ref real) = real_upsert2 {
        assert_eq!(u16::from(real.status().status_code()), 200);
    }

    // Verify final state
    let (emu_read, _) = backend
        .execute_and_compare(
            &emu_container,
            real_container.as_ref(),
            |container| {
                let item =
                    ItemReference::from_name(container, PartitionKey::from("pk1"), "upsert-item");
                (
                    CosmosOperation::read_item(item),
                    OperationOptions::default(),
                )
            },
            &HeaderValidationSpec::for_point_operation(),
            BodyValidationSpec::DocumentMatch,
        )
        .await
        .unwrap();

    let doc: serde_json::Value = serde_json::from_slice(emu_read.body()).unwrap();
    assert_eq!(doc["value"], 20, "value should reflect second upsert");

    // Cleanup
    backend.cleanup_real_database(&db_name).await;
}
