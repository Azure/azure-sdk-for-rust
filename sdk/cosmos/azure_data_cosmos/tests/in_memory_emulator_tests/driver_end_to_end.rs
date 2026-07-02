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
    CosmosOperation, DatabaseReference, ItemReference, PartitionKey, ResponseBody,
};
use azure_data_cosmos_driver::options::{DriverOptions, OperationOptions, OperationOptionsBuilder};
use azure_data_cosmos_driver::CosmosResponse;

#[cfg(feature = "fault_injection")]
use azure_data_cosmos_driver::options::Region;

use super::dual_backend::DualBackend;
use super::validation::{
    compare_responses, BodyValidationSpec, HeaderValidationSpec, ResponseSnapshot,
};
use uuid::Uuid;

/// Parses a single-payload response body as JSON without consuming the response.
fn body_json(response: &CosmosResponse) -> serde_json::Value {
    match response.body() {
        ResponseBody::Bytes(b) => serde_json::from_slice(b).unwrap(),
        ResponseBody::NoPayload => panic!("expected single Bytes body, got no payload"),
        ResponseBody::Items(_) => panic!("expected single Bytes body, got feed response"),
    }
}

/// Sets up both backends with a shared database and container.
///
/// Returns `(backend, db_name, emulator_container, Option<real_container>)`.
fn make_stale_session_token(token: &str) -> String {
    let mut parts = token.split('#');
    let prefix = parts.next().unwrap_or(token);
    let Some(_) = parts.next() else {
        return format!("{prefix}#9999999999");
    };

    let region_progress: Vec<String> = parts
        .map(|segment| match segment.split_once('=') {
            Some((region_id, _)) => format!("{region_id}=9999999999"),
            None => segment.to_string(),
        })
        .collect();

    if region_progress.is_empty() {
        format!("{prefix}#9999999999")
    } else {
        format!("{prefix}#9999999999#{}", region_progress.join("#"))
    }
}

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
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: dual-backend test fails against vnext gateway"
)]
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
                let item =
                    ItemReference::from_name(container, PartitionKey::from("pk1"), "driver-item-1");
                let op = CosmosOperation::create_item(item).with_body(body_bytes.clone());
                (op, OperationOptions::default())
            },
            &HeaderValidationSpec::for_point_operation(),
            BodyValidationSpec::DocumentMatch,
        )
        .await
        .unwrap();

    assert_eq!(
        u16::from(emu_create.status()),
        201,
        "Emulator create should return 201 Created",
    );
    if let Some(ref real) = real_create {
        assert_eq!(
            u16::from(real.status()),
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
        u16::from(emu_read.status()),
        200,
        "Emulator read should return 200 OK",
    );

    // Verify emulator body structure
    let doc: serde_json::Value = body_json(&emu_read);
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
        let real_doc: serde_json::Value = body_json(real);
        assert_eq!(real_doc["id"], "driver-item-1");
        assert_eq!(real_doc["value"], 42);
    }

    // Cleanup
    backend.cleanup_real_database(&db_name).await;
}

#[tokio::test]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: dual-backend test fails against vnext gateway"
)]
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
        u16::from(emu_create_db.status()),
        201,
        "Emulator create DB should return 201",
    );
    if let Some(ref real) = real_create_db {
        assert_eq!(
            u16::from(real.status()),
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
        .execute_singleton_operation(emu_create_coll_op, OperationOptions::default())
        .await
        .unwrap();

    let real_create_coll = if let (Some(ref driver), Some(ref account)) =
        (&backend.real_driver, &backend.real_account)
    {
        let real_db_ref = DatabaseReference::from_name(account.clone(), db_name.clone());
        let real_op = CosmosOperation::create_container(real_db_ref).with_body(coll_body.clone());
        let resp = driver
            .execute_singleton_operation(real_op, OperationOptions::default())
            .await
            .unwrap();
        Some(resp)
    } else {
        None
    };

    assert_eq!(
        u16::from(emu_create_coll.status()),
        201,
        "Emulator create container should return 201",
    );

    // Compare create-container responses
    if let Some(ref real_resp) = real_create_coll {
        assert_eq!(
            u16::from(real_resp.status()),
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
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: dual-backend test fails against vnext gateway"
)]
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
                let item =
                    ItemReference::from_name(container, PartitionKey::from("pk1"), "delete-me");
                let op = CosmosOperation::create_item(item).with_body(body_bytes.clone());
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
        u16::from(emu_delete.status()),
        204,
        "Emulator delete should return 204 No Content",
    );
    if let Some(ref real) = real_delete {
        assert_eq!(
            u16::from(real.status()),
            204,
            "Real delete should return 204 No Content",
        );
    }

    // ── Verify item is gone (emulator) ───────────────────────────
    let emu_read_deleted = backend
        .emulator_driver
        .execute_singleton_operation(
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
            .execute_singleton_operation(
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
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: dual-backend test fails against vnext gateway"
)]
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
                let item =
                    ItemReference::from_name(container, PartitionKey::from("pk1"), "replace-me");
                let op = CosmosOperation::create_item(item).with_body(create_body.clone());
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
        u16::from(emu_replace.status()),
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

    let doc: serde_json::Value = body_json(&emu_read);
    assert_eq!(doc["value"], 99, "value should be updated to 99");

    if let Some(ref real) = real_replace {
        assert_eq!(u16::from(real.status()), 200);
    }

    // Cleanup
    backend.cleanup_real_database(&db_name).await;
}

#[tokio::test]
async fn read_with_stale_session_token_returns_404_1002() {
    let (backend, db_name, emu_container, real_container) = setup_with_container().await;

    // We first do a write to get a real session token (with the correct PKRange
    // ID), then bump its LSN far beyond the partition's actual LSN. This makes
    // the service match the token to the correct partition and detect that it
    // cannot be satisfied, reported as 404 / sub-status 1002
    // (ReadSessionNotAvailable) on both classic gateway and the Gateway 2.0
    // thin-client path.
    //
    // We derive the token (and pkrange id) from a seed create on each backend
    // — `pk1` does not always hash to pkrange 0 under V2 hashing, so a
    // hardcoded `"0:..."` token is silently ignored against the real
    // partition and the read returns plain 404 instead of 404/1002.

    // Create a seed item on both backends to get a valid session token from real.
    let seed_body = serde_json::to_vec(&serde_json::json!({
        "id": "seed-for-session",
        "pk": "pk1",
        "value": 0
    }))
    .unwrap();

    let real_stale_token =
        if let (Some(ref driver), Some(ref real_ctr)) = (&backend.real_driver, &real_container) {
            let seed_result = driver
                .execute_singleton_operation(
                    CosmosOperation::create_item(ItemReference::from_name(
                        real_ctr,
                        PartitionKey::from("pk1"),
                        "seed-for-session",
                    ))
                    .with_body(seed_body.clone()),
                    OperationOptions::default(),
                )
                .await
                .expect("Real seed create should succeed");
            let token = seed_result
                .headers()
                .session_token
                .as_ref()
                .expect("Real create should return session token")
                .as_str()
                .to_string();
            Some(make_stale_session_token(&token))
        } else {
            None
        };

    // Seed the emulator and derive a stale token using the same pkrange id
    // the emulator routed the seed write to.
    let emu_seed_result = backend
        .emulator_driver
        .execute_singleton_operation(
            CosmosOperation::create_item(ItemReference::from_name(
                &emu_container,
                PartitionKey::from("pk1"),
                "seed-for-session",
            ))
            .with_body(seed_body),
            OperationOptions::default(),
        )
        .await
        .expect("Emulator seed create should succeed");
    let emu_seed_token = emu_seed_result
        .headers()
        .session_token
        .as_ref()
        .expect("Emulator create should return a session token")
        .as_str()
        .to_string();
    let emu_stale_token = make_stale_session_token(&emu_seed_token);

    // Disable session retries so the error propagates immediately.
    let opts = OperationOptionsBuilder::new()
        .with_max_session_retry_count(0)
        .build();

    // ── Emulator ─────────────────────────────────────────────────
    let emu_err = backend
        .emulator_driver
        .execute_singleton_operation(
            CosmosOperation::read_item(ItemReference::from_name(
                &emu_container,
                PartitionKey::from("pk1"),
                "no-such-item",
            ))
            .with_session_token(emu_stale_token.clone()),
            opts.clone(),
        )
        .await;

    let emu_err = emu_err.expect_err("Emulator should return an error for stale session read");
    assert_eq!(
        Some(emu_err.status().status_code()),
        Some(azure_core::http::StatusCode::NotFound),
        "Emulator error should be HTTP 404",
    );
    let error_code = emu_err.status().sub_status().map(|s| s.value().to_string());
    assert_eq!(
        error_code.as_deref(),
        Some("1002"),
        "Emulator error should have substatus 1002",
    );

    // ── Real account (if available) ──────────────────────────────
    if let (Some(ref driver), Some(ref real_ctr)) = (&backend.real_driver, &real_container) {
        let stale_token: String = real_stale_token
            .clone()
            .expect("real_stale_token should be set when real driver is available");

        // On a multi-region account the read regions may not have replicated
        // the freshly created database/container/item yet; a lagging region
        // returns a plain resource 404 (no sub-status) instead of the session
        // 404/1002 we assert below. Confirm the seed item is point-readable from
        // every advertised read region before issuing the stale read.
        backend
            .wait_for_sentinel_readable_from_all_regions(
                &db_name,
                "testcoll",
                "pk1",
                "seed-for-session",
            )
            .await
            .expect("seed item should become readable from all regions");

        let real_err = driver
            .execute_singleton_operation(
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
        // The read targets a nonexistent item, so it returns HTTP 404 on every
        // consistency level. Under Session the seed-derived token's bumped LSN
        // additionally trips the soft 404 / sub-status 1002 (ReadSessionNotAvailable)
        // path (asserted below); Eventual/Strong ignore the token entirely.
        assert_eq!(
            real_err.status().status_code(),
            azure_core::http::StatusCode::NotFound,
            "Real stale session read should return HTTP 404",
        );
        // Substatus 1002 is only produced under Session consistency; on
        // Eventual/Strong accounts the stale token is ignored and the missing
        // item surfaces as a plain 404/0. Only assert 1002 on Session accounts.
        if DualBackend::real_account_uses_session_consistency() {
            assert_eq!(
                real_err.status().sub_status().map(|s| s.value()),
                Some(1002),
                "Real 404 stale session read should surface substatus 1002",
            );
        }
    }

    // Cleanup
    backend.cleanup_real_database(&db_name).await;
}

#[tokio::test]
async fn read_after_split_refreshes_driver_routing_map() {
    let (backend, db_name, emu_container, _) = setup_with_container().await;

    let create = backend
        .emulator_driver
        .execute_singleton_operation(
            CosmosOperation::create_item(ItemReference::from_name(
                &emu_container,
                PartitionKey::from("pk1"),
                "split-item",
            ))
            .with_body(
                serde_json::to_vec(&serde_json::json!({
                    "id": "split-item",
                    "pk": "pk1",
                    "value": 42
                }))
                .unwrap(),
            ),
            OperationOptions::default(),
        )
        .await
        .unwrap();

    let routed_partition_id: u32 = create
        .headers()
        .session_token
        .as_ref()
        .and_then(|token| token.as_str().split(':').next())
        .and_then(|prefix| prefix.parse().ok())
        .expect("create should return a session token with a numeric partition id");

    backend.emulator_store.split_partition(
        &db_name,
        "testcoll",
        routed_partition_id,
        std::time::Duration::ZERO,
    );
    backend.emulator_store.drain_pending_control_plane().await;

    let read = backend
        .emulator_driver
        .execute_singleton_operation(
            CosmosOperation::read_item(ItemReference::from_name(
                &emu_container,
                PartitionKey::from("pk1"),
                "split-item",
            )),
            OperationOptions::default(),
        )
        .await
        .unwrap();

    assert_eq!(
        u16::from(read.status()),
        200,
        "driver should refresh the routing map after a split",
    );

    let doc: serde_json::Value = body_json(&read);
    assert_eq!(doc["id"], "split-item");
    assert_eq!(doc["value"], 42);

    backend.cleanup_real_database(&db_name).await;
}
#[tokio::test]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: dual-backend test fails against vnext gateway"
)]
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
        u16::from(emu_upsert1.status()),
        201,
        "Emulator upsert-as-insert should return 201",
    );
    if let Some(ref real) = real_upsert1 {
        assert_eq!(u16::from(real.status()), 201);
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
        u16::from(emu_upsert2.status()),
        200,
        "Emulator upsert-as-update should return 200",
    );
    if let Some(ref real) = real_upsert2 {
        assert_eq!(u16::from(real.status()), 200);
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

    let doc: serde_json::Value = body_json(&emu_read);
    assert_eq!(doc["value"], 20, "value should reflect second upsert");

    // Cleanup
    backend.cleanup_real_database(&db_name).await;
}

#[tokio::test]
async fn paused_satellite_converges_to_latest_hub_write() {
    use azure_core::http::Url;
    use azure_data_cosmos_driver::in_memory_emulator::{
        ConsistencyLevel, InMemoryEmulatorHttpClient, ReplicationConfig, VirtualAccountConfig,
        VirtualRegion, WriteMode,
    };
    use azure_data_cosmos_driver::models::AccountReference;
    use azure_data_cosmos_driver::options::{DriverOptionsBuilder, Region};

    let run_id = Uuid::new_v4().to_string()[..8].to_string();
    let east_url = "https://eastus.emulator.local";
    let west_url = "https://westus.emulator.local";

    let config = VirtualAccountConfig::new(vec![
        VirtualRegion::new("East US", Url::parse(east_url).unwrap()),
        VirtualRegion::new("West US", Url::parse(west_url).unwrap()),
    ])
    .unwrap()
    .with_write_mode(WriteMode::Single)
    .with_consistency(ConsistencyLevel::Session)
    .with_replication_config(ReplicationConfig::immediate());

    let emulator = std::sync::Arc::new(InMemoryEmulatorHttpClient::new(config));
    let emulator_store = emulator.store();
    let emulator_runtime = emulator.runtime_builder().build().await.unwrap();

    let db_name = format!("hub-sync-{run_id}");
    emulator_store.create_database(&db_name);
    emulator_store.create_container(
        &db_name,
        "hub-testcoll",
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))
        .unwrap(),
    );
    emulator_store.pause_replication("West US");

    let account = AccountReference::with_master_key(Url::parse(east_url).unwrap(), "dGVzdGtleQ==");
    let driver = emulator_runtime
        .create_driver(
            DriverOptionsBuilder::new(account)
                .with_preferred_regions(vec![Region::WEST_US, Region::EAST_US])
                .build(),
        )
        .await
        .unwrap();

    let container = driver
        .resolve_container(&db_name, "hub-testcoll")
        .await
        .unwrap();

    driver
        .execute_singleton_operation(
            CosmosOperation::create_item(ItemReference::from_name(
                &container,
                PartitionKey::from("pk1"),
                "hub-item",
            ))
            .with_body(
                serde_json::to_vec(&serde_json::json!({
                    "id": "hub-item",
                    "pk": "pk1",
                    "value": 1
                }))
                .unwrap(),
            ),
            OperationOptions::default(),
        )
        .await
        .unwrap();

    driver
        .execute_singleton_operation(
            CosmosOperation::replace_item(ItemReference::from_name(
                &container,
                PartitionKey::from("pk1"),
                "hub-item",
            ))
            .with_body(
                serde_json::to_vec(&serde_json::json!({
                    "id": "hub-item",
                    "pk": "pk1",
                    "value": 2
                }))
                .unwrap(),
            ),
            OperationOptions::default(),
        )
        .await
        .unwrap();

    // Disable cross-region hedging on this read so the §5.2 driver-default
    // (≥2 preferred regions) does not race the West US 404/1002 against an
    // East US hedge. This test exercises the session-retry path on a paused
    // satellite; with hedging enabled the secondary leg in East US would
    // succeed (West US is intentionally stale) and mask the per-region
    // failure the test is asserting.
    let no_session_retry = OperationOptionsBuilder::new()
        .with_max_session_retry_count(0)
        .with_availability_strategy(
            azure_data_cosmos_driver::options::AvailabilityStrategy::Disabled,
        )
        .build();

    let west_read_before_resume = driver
        .execute_singleton_operation(
            CosmosOperation::read_item(ItemReference::from_name(
                &container,
                PartitionKey::from("pk1"),
                "hub-item",
            )),
            no_session_retry,
        )
        .await
        .expect_err("paused satellite should not observe the hub write yet");
    assert_eq!(
        Some(west_read_before_resume.status().status_code()),
        Some(azure_core::http::StatusCode::NotFound),
        "read should fail while West US replication is paused",
    );

    emulator_store.resume_replication("West US");

    let west_read_after_resume = driver
        .execute_singleton_operation(
            CosmosOperation::read_item(ItemReference::from_name(
                &container,
                PartitionKey::from("pk1"),
                "hub-item",
            )),
            OperationOptions::default(),
        )
        .await
        .unwrap();

    assert_eq!(
        u16::from(west_read_after_resume.status()),
        200,
        "satellite read should succeed once hub writes replicate",
    );

    let doc: serde_json::Value = body_json(&west_read_after_resume);
    assert_eq!(doc["value"], 2);
}

#[tokio::test]
async fn create_retries_after_429_throttling() {
    use azure_core::http::Url;
    use azure_data_cosmos_driver::in_memory_emulator::{
        ConsistencyLevel, ContainerConfig, InMemoryEmulatorHttpClient, VirtualAccountConfig,
        VirtualRegion,
    };
    use azure_data_cosmos_driver::models::AccountReference;

    let run_id = Uuid::new_v4().to_string()[..8].to_string();
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse("https://eastus.emulator.local").unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session)
    .with_throttling_enabled(true);

    let emulator = std::sync::Arc::new(InMemoryEmulatorHttpClient::new(config));
    let emulator_store = emulator.store();
    let emulator_runtime = emulator.runtime_builder().build().await.unwrap();

    let db_name = format!("driver-throttle-{run_id}");
    emulator_store.create_database(&db_name);
    emulator_store.create_container_with_config(
        &db_name,
        "throttle_coll",
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))
        .unwrap(),
        ContainerConfig::new()
            .with_partition_count(1)
            .with_throughput(400)
            .build()
            .unwrap(),
    );

    let account = AccountReference::with_master_key(
        Url::parse("https://eastus.emulator.local").unwrap(),
        "dGVzdGtleQ==",
    );
    let driver = emulator_runtime
        .create_driver(DriverOptions::builder(account.clone()).build())
        .await
        .unwrap();
    let container = driver
        .resolve_container(&db_name, "throttle_coll")
        .await
        .unwrap();

    let seed_body = serde_json::to_vec(&serde_json::json!({
        "id": "seed-throttle",
        "pk": "pk1",
        "value": 1,
        "padding": "x".repeat(40 * 1024)
    }))
    .unwrap();
    driver
        .execute_singleton_operation(
            CosmosOperation::create_item(ItemReference::from_name(
                &container,
                PartitionKey::from("pk1"),
                "seed-throttle",
            ))
            .with_body(seed_body),
            OperationOptions::default(),
        )
        .await
        .unwrap();

    let throttled_body = serde_json::to_vec(&serde_json::json!({
        "id": "throttled-item",
        "pk": "pk1",
        "value": 42,
        "padding": "x".repeat(8 * 1024)
    }))
    .unwrap();

    let start = std::time::Instant::now();
    let create = driver
        .execute_singleton_operation(
            CosmosOperation::create_item(ItemReference::from_name(
                &container,
                PartitionKey::from("pk1"),
                "throttled-item",
            ))
            .with_body(throttled_body),
            OperationOptions::default(),
        )
        .await
        .unwrap();
    let elapsed = start.elapsed();

    assert!(
        elapsed >= std::time::Duration::from_millis(200),
        "create should have retried after a 429 throttling response (elapsed: {:?})",
        elapsed,
    );
    assert_eq!(u16::from(create.status()), 201);

    let read = driver
        .execute_singleton_operation(
            CosmosOperation::read_item(ItemReference::from_name(
                &container,
                PartitionKey::from("pk1"),
                "throttled-item",
            )),
            OperationOptions::default(),
        )
        .await
        .unwrap();

    let doc: serde_json::Value = body_json(&read);
    assert_eq!(doc["value"], 42);
    assert_eq!(doc["padding"].as_str().map(str::len), Some(8 * 1024));
}

// ─── Multi-region fault injection ────────────────────────────────────────────

/// Tests that fault injection on the primary (preferred) read region causes
/// the driver to failover to the secondary region and return a successful
/// response. Validates status code, headers, and body on the failover path.
///
/// Setup:
/// - Multi-region emulator: East US (write region) + West US (read-only)
/// - Preferred regions: [East US, West US]
/// - Fault rule: 503 ServiceUnavailable on ReadItem in East US (hit limit = 4)
///
/// Flow:
/// 1. Create an item in East US (no fault — rule targets ReadItem only)
/// 2. Read the item — driver hits 503 in East US, retries, fails over to West US
/// 3. Verify the read succeeds with 200, correct body, and proper headers
///
/// When `AZURE_COSMOS_CONNECTION_STRING` is set, the same fault injection
/// scenario runs against a real account and responses are compared.
#[cfg(feature = "fault_injection")]
#[tokio::test]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: dual-backend test fails against vnext gateway"
)]
async fn read_failover_on_503_via_fault_injection() {
    use azure_core::http::Url;
    use azure_data_cosmos_driver::fault_injection::{
        FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
        FaultInjectionRuleBuilder, FaultOperationType,
    };
    use azure_data_cosmos_driver::in_memory_emulator::{
        ConsistencyLevel, InMemoryEmulatorHttpClient, ReplicationConfig, VirtualAccountConfig,
        VirtualRegion, WriteMode,
    };
    use azure_data_cosmos_driver::models::AccountReference;
    use azure_data_cosmos_driver::options::DriverOptionsBuilder;
    use std::sync::Arc;

    let _ = tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();

    // ── Fault injection rule: 503 on ReadItem in East US ─────────
    let fault_result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();

    let fault_condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(Region::EAST_US)
        .build();

    // Use a shared rule so both emulator and real backends share the same hit
    // count state and the same `enabled` flag.
    let shared_enabled = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let shared_hit_count = Arc::new(std::sync::atomic::AtomicU32::new(0));

    let emu_rule = Arc::new(
        FaultInjectionRuleBuilder::new("read-503-east-us", fault_result.clone())
            .with_condition(fault_condition.clone())
            .with_hit_limit(4) // enough for local retries then failover
            .with_shared_state(Arc::clone(&shared_enabled), Arc::clone(&shared_hit_count))
            .build(),
    );

    // ── Multi-region emulator setup ──────────────────────────────
    let east_url = "https://eastus.emulator.local";
    let west_url = "https://westus.emulator.local";

    let config = VirtualAccountConfig::new(vec![
        VirtualRegion::new("East US", Url::parse(east_url).unwrap()),
        VirtualRegion::new("West US", Url::parse(west_url).unwrap()),
    ])
    .unwrap()
    .with_write_mode(WriteMode::Single)
    .with_consistency(ConsistencyLevel::Session)
    .with_replication_config(ReplicationConfig::immediate());

    let emulator = std::sync::Arc::new(InMemoryEmulatorHttpClient::new(config));
    let emulator_store = emulator.store();

    // Build runtime; fault injection is configured per driver (not on the runtime).
    let emulator_runtime = emulator.runtime_builder().build().await.unwrap();

    // Provision database and container.
    emulator_store.create_database("fi-testdb");
    emulator_store.create_container(
        "fi-testdb",
        "fi-testcoll",
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))
        .unwrap(),
    );

    let emu_account =
        AccountReference::with_master_key(Url::parse(east_url).unwrap(), "dGVzdGtleQ==");
    let emu_driver_opts = DriverOptionsBuilder::new(emu_account.clone())
        .with_preferred_regions(vec![Region::EAST_US, Region::WEST_US])
        .with_fault_injection_rules(vec![Arc::clone(&emu_rule)])
        .expect("distinct fault injection rule id")
        .build();
    let emu_driver = emulator_runtime
        .create_driver(emu_driver_opts)
        .await
        .unwrap();

    let emu_container = emu_driver
        .resolve_container("fi-testdb", "fi-testcoll")
        .await
        .unwrap();

    // ── Create item (no fault — rule targets ReadItem only) ──────
    let item_body = serde_json::to_vec(&serde_json::json!({
        "id": "failover-item",
        "pk": "pk1",
        "value": 42
    }))
    .unwrap();

    let emu_create = emu_driver
        .execute_singleton_operation(
            CosmosOperation::create_item(ItemReference::from_name(
                &emu_container,
                PartitionKey::from("pk1"),
                "failover-item",
            ))
            .with_body(item_body.clone()),
            OperationOptions::default(),
        )
        .await
        .unwrap();

    assert_eq!(
        u16::from(emu_create.status()),
        201,
        "Emulator create should return 201",
    );

    // ── Read item — should failover from East US → West US ───────
    let emu_read = emu_driver
        .execute_singleton_operation(
            CosmosOperation::read_item(ItemReference::from_name(
                &emu_container,
                PartitionKey::from("pk1"),
                "failover-item",
            )),
            OperationOptions::default(),
        )
        .await
        .unwrap();

    assert_eq!(
        u16::from(emu_read.status()),
        200,
        "Emulator read should succeed via failover to West US",
    );

    // Verify fault rule was hit (confirms failover actually occurred).
    assert!(
        emu_rule.hit_count() > 0,
        "Fault rule should have been hit at least once (was hit {} times)",
        emu_rule.hit_count(),
    );

    // Verify response body.
    let doc: serde_json::Value = body_json(&emu_read);
    assert_eq!(doc["id"], "failover-item");
    assert_eq!(doc["value"], 42);

    // Verify key headers on the successful response.
    let emu_headers = emu_read.headers();
    assert!(
        emu_headers.activity_id.is_some(),
        "activity_id should be present",
    );
    assert!(
        emu_headers.request_charge.is_some(),
        "request_charge should be present",
    );
    assert!(
        emu_headers.session_token.is_some(),
        "session_token should be present",
    );
    assert!(
        emu_headers.etag.is_some(),
        "etag should be present on successful read",
    );
    assert!(
        emu_headers.server_duration_ms.is_some(),
        "server_duration_ms should be present",
    );
    assert!(
        emu_read.status().sub_status().is_none(),
        "successful read should have no substatus",
    );

    // Verify system properties in body.
    assert!(doc.get("_rid").is_some(), "should have _rid");
    assert!(doc.get("_etag").is_some(), "should have _etag");

    // ── Real account comparison (if available) ───────────────────
    let real_result = try_real_failover_comparison(
        &item_body,
        fault_condition,
        fault_result,
        shared_enabled,
        shared_hit_count,
    )
    .await;

    if let Some(real_read) = real_result {
        let real_snap = ResponseSnapshot::capture(&real_read, "real");
        let emu_snap = ResponseSnapshot::capture(&emu_read, "emulator");
        compare_responses(
            &real_snap,
            &emu_snap,
            &HeaderValidationSpec::for_point_operation(),
            BodyValidationSpec::DocumentMatch,
        );
    }
}

/// Runs the same fault-injection failover scenario against a real Cosmos DB
/// account when `AZURE_COSMOS_CONNECTION_STRING` is set.
///
/// Returns `Some(CosmosResponse)` with the successful read, or `None` when
/// no real account is configured.
#[cfg(feature = "fault_injection")]
async fn try_real_failover_comparison(
    item_body: &[u8],
    fault_condition: azure_data_cosmos_driver::fault_injection::FaultInjectionCondition,
    fault_result: azure_data_cosmos_driver::fault_injection::FaultInjectionResult,
    shared_enabled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    shared_hit_count: std::sync::Arc<std::sync::atomic::AtomicU32>,
) -> Option<azure_data_cosmos_driver::models::CosmosResponse> {
    use azure_core::http::Url;
    use azure_data_cosmos_driver::driver::CosmosDriverRuntime;
    use azure_data_cosmos_driver::fault_injection::FaultInjectionRuleBuilder;
    use azure_data_cosmos_driver::models::{AccountReference, ConnectionString};
    use azure_data_cosmos_driver::options::{
        ConnectionPoolOptions, DriverOptionsBuilder, ServerCertificateValidation,
    };
    use std::sync::Arc;

    let conn_str_raw = std::env::var("AZURE_COSMOS_CONNECTION_STRING").ok()?;
    let mode = std::env::var("AZURE_COSMOS_TEST_MODE")
        .unwrap_or_default()
        .to_lowercase();
    if mode == "skipped" {
        return None;
    }

    let conn_str: ConnectionString = conn_str_raw.parse().ok()?;
    let endpoint: Url = conn_str.account_endpoint().parse().ok()?;
    let key = conn_str.account_key().secret().to_string();
    let account = AccountReference::with_master_key(endpoint, key);

    // Reset shared hit count for the real leg.
    shared_hit_count.store(0, std::sync::atomic::Ordering::SeqCst);

    let real_rule = Arc::new(
        FaultInjectionRuleBuilder::new("read-503-east-us-real", fault_result)
            .with_condition(fault_condition)
            .with_hit_limit(4)
            .with_shared_state(shared_enabled, shared_hit_count)
            .build(),
    );

    let mut pool_builder = ConnectionPoolOptions::builder();
    if conn_str.account_endpoint().contains("localhost") {
        pool_builder = pool_builder.with_server_certificate_validation(
            ServerCertificateValidation::RequiredUnlessEmulator,
        );
    }
    let pool = pool_builder.build().ok()?;

    let runtime = CosmosDriverRuntime::builder()
        .with_connection_pool(pool)
        .build()
        .await
        .ok()?;

    let driver_opts = DriverOptionsBuilder::new(account.clone())
        .with_preferred_regions(vec![Region::EAST_US, Region::WEST_US])
        .with_fault_injection_rules(vec![Arc::clone(&real_rule)])
        .ok()?
        .build();

    let driver = runtime.create_driver(driver_opts).await.ok()?;

    // Create a unique database for this test run.
    let run_id = uuid::Uuid::new_v4().to_string()[..8].to_string();
    let db_name = format!("fi-failover-{run_id}");

    let db_body = serde_json::to_vec(&serde_json::json!({"id": &db_name})).ok()?;
    let db_ref = azure_data_cosmos_driver::models::DatabaseReference::from_name(
        account.clone(),
        db_name.clone(),
    );
    driver
        .execute_singleton_operation(
            CosmosOperation::create_database(account.clone()).with_body(db_body),
            OperationOptions::default(),
        )
        .await
        .ok()?;

    let coll_body = serde_json::to_vec(&serde_json::json!({
        "id": "fi-testcoll",
        "partitionKey": {"paths": ["/pk"], "kind": "Hash", "version": 2}
    }))
    .ok()?;
    driver
        .execute_singleton_operation(
            CosmosOperation::create_container(db_ref.clone()).with_body(coll_body),
            OperationOptions::default(),
        )
        .await
        .ok()?;

    let container = driver
        .resolve_container(&db_name, "fi-testcoll")
        .await
        .ok()?;

    // Create item.
    driver
        .execute_singleton_operation(
            CosmosOperation::create_item(ItemReference::from_name(
                &container,
                PartitionKey::from("pk1"),
                "failover-item",
            ))
            .with_body(item_body.to_vec()),
            OperationOptions::default(),
        )
        .await
        .ok()?;

    // Read item — should failover.
    let read_result = driver
        .execute_singleton_operation(
            CosmosOperation::read_item(ItemReference::from_name(
                &container,
                PartitionKey::from("pk1"),
                "failover-item",
            )),
            OperationOptions::default(),
        )
        .await;

    // Cleanup.
    let _ = driver
        .execute_singleton_operation(
            CosmosOperation::delete_database(db_ref),
            OperationOptions::default(),
        )
        .await;

    read_result.ok()
}

// ─── V1 partition-key coverage ─────────────────────────────────────────────
//
// Real Cosmos DB no longer creates V1 `Hash` containers by default, but
// pre-existing V1 containers in customer accounts are still routed to. The
// emulator must therefore correctly distribute V1 EPKs across physical
// partitions and round-trip point operations against them. Without the
// version-aware boundary scheme in [`in_memory_emulator::store`], every V1
// EPK lex-compared below `boundary[0]` (a V2-style 32-char hex) and landed
// in partition 0, defeating partitioning entirely.

async fn setup_with_v1_container() -> (
    DualBackend,
    String,
    azure_data_cosmos_driver::models::ContainerReference,
    Option<azure_data_cosmos_driver::models::ContainerReference>,
) {
    let backend = DualBackend::setup().await.unwrap();
    let db_name = backend.unique_db_name();
    let container_name = "testcoll-v1";
    let pk_path = "/pk";

    backend.provision_emulator_v1(&db_name, container_name, pk_path);

    if backend.has_real_backend() {
        backend.create_real_database(&db_name).await.unwrap();
        backend
            .create_real_container_v1(&db_name, container_name, pk_path)
            .await
            .unwrap();
    }

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
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: dual-backend test fails against vnext gateway"
)]
async fn v1_create_read_replace_delete_through_driver() {
    let (backend, db_name, emu_container, real_container) = setup_with_v1_container().await;

    // Create
    let create_body = serde_json::json!({"id": "v1-item-1", "pk": "v1-pk-A", "value": 1});
    let create_bytes = serde_json::to_vec(&create_body).unwrap();
    let (emu_create, real_create) = backend
        .execute_and_compare(
            &emu_container,
            real_container.as_ref(),
            |container| {
                let item =
                    ItemReference::from_name(container, PartitionKey::from("v1-pk-A"), "v1-item-1");
                let op = CosmosOperation::create_item(item).with_body(create_bytes.clone());
                (op, OperationOptions::default())
            },
            &HeaderValidationSpec::for_point_operation(),
            BodyValidationSpec::DocumentMatch,
        )
        .await
        .unwrap();
    assert_eq!(u16::from(emu_create.status()), 201);
    if let Some(ref real) = real_create {
        assert_eq!(u16::from(real.status()), 201);
    }

    // Read
    let (emu_read, _) = backend
        .execute_and_compare(
            &emu_container,
            real_container.as_ref(),
            |container| {
                let item =
                    ItemReference::from_name(container, PartitionKey::from("v1-pk-A"), "v1-item-1");
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
    assert_eq!(u16::from(emu_read.status()), 200);
    let doc: serde_json::Value = body_json(&emu_read);
    assert_eq!(doc["id"], "v1-item-1");
    assert_eq!(doc["pk"], "v1-pk-A");
    assert_eq!(doc["value"], 1);

    // Replace
    let replace_body = serde_json::json!({"id": "v1-item-1", "pk": "v1-pk-A", "value": 99});
    let replace_bytes = serde_json::to_vec(&replace_body).unwrap();
    let (emu_replace, _) = backend
        .execute_and_compare(
            &emu_container,
            real_container.as_ref(),
            |container| {
                let item =
                    ItemReference::from_name(container, PartitionKey::from("v1-pk-A"), "v1-item-1");
                let op = CosmosOperation::replace_item(item).with_body(replace_bytes.clone());
                (op, OperationOptions::default())
            },
            &HeaderValidationSpec::for_point_operation(),
            BodyValidationSpec::DocumentMatch,
        )
        .await
        .unwrap();
    assert_eq!(u16::from(emu_replace.status()), 200);

    // Delete
    let (emu_delete, _) = backend
        .execute_and_compare(
            &emu_container,
            real_container.as_ref(),
            |container| {
                let item =
                    ItemReference::from_name(container, PartitionKey::from("v1-pk-A"), "v1-item-1");
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
    assert_eq!(u16::from(emu_delete.status()), 204);

    backend.cleanup_real_database(&db_name).await;
}

/// Drives ~200 V1 writes across distinct partition-key values to confirm that
/// V1 routing distributes load across physical partitions in the emulator —
/// the regression bait for the "all V1 EPKs land in partition 0" bug.
#[tokio::test]
async fn v1_writes_distribute_across_partitions() {
    let (backend, db_name, emu_container, _) = setup_with_v1_container().await;

    let mut written = 0usize;
    for i in 0..200 {
        let pk = format!("v1-tenant-{}", i);
        let id = format!("v1-doc-{}", i);
        let body = serde_json::json!({"id": id, "pk": pk, "value": i as i64});
        let body_bytes = serde_json::to_vec(&body).unwrap();
        let resp = backend
            .emulator_driver
            .execute_singleton_operation(
                CosmosOperation::create_item(ItemReference::from_name(
                    &emu_container,
                    PartitionKey::from(pk.clone()),
                    id.clone(),
                ))
                .with_body(body_bytes),
                OperationOptions::default(),
            )
            .await
            .unwrap();
        assert_eq!(u16::from(resp.status()), 201);
        written += 1;
    }

    // Tally partition-key range ids reported on subsequent reads. We don't
    // assert exact distribution here (the per-partition tally test in the
    // `store` unit tests covers that with controlled fixtures); we only
    // require that more than one distinct pkrange handles the workload,
    // which is enough to catch the all-in-one-partition regression even at
    // a small `N=200` sample.
    let mut distinct_pkranges = std::collections::HashSet::new();
    for i in 0..200 {
        let pk = format!("v1-tenant-{}", i);
        let id = format!("v1-doc-{}", i);
        let resp = backend
            .emulator_driver
            .execute_singleton_operation(
                CosmosOperation::read_item(ItemReference::from_name(
                    &emu_container,
                    PartitionKey::from(pk),
                    id,
                )),
                OperationOptions::default(),
            )
            .await
            .unwrap();
        if let Some(token) = resp.headers().session_token.as_ref() {
            if let Some(prefix) = token.as_str().split(':').next() {
                if let Ok(pkrange_id) = prefix.parse::<u32>() {
                    distinct_pkranges.insert(pkrange_id);
                }
            }
        }
    }
    assert_eq!(written, 200);
    assert!(
        distinct_pkranges.len() > 1,
        "V1 writes routed to only {:?} distinct pkrange(s) — distribution is broken",
        distinct_pkranges,
    );

    backend.cleanup_real_database(&db_name).await;
}

/// Asserts that a failed operation's `CosmosError` carries the rich
/// per-operation `DiagnosticsContext` so callers can recover ActivityId,
/// region, transport shard, and per-attempt event history on the error path.
#[tokio::test]
async fn error_carries_extractable_diagnostics() {
    let (backend, db_name, emu_container, _real_container) = setup_with_container().await;

    let read_missing = backend
        .emulator_driver
        .execute_operation(
            CosmosOperation::read_item(ItemReference::from_name(
                &emu_container,
                PartitionKey::from("pk-not-here"),
                "id-that-does-not-exist",
            )),
            OperationOptions::default(),
        )
        .await;

    let err = read_missing.expect_err("read of missing item must fail");
    assert_eq!(
        u16::from(err.status().status_code()),
        404,
        "missing-item read should surface as 404",
    );

    let diagnostics = err
        .diagnostics()
        .expect("error must carry diagnostics context attached by the pipeline");

    let json = diagnostics.to_json_string(None);
    assert!(
        json.contains("\"activity_id\""),
        "diagnostics JSON should include activity_id, got: {json}",
    );

    let final_status = diagnostics
        .status()
        .expect("diagnostics must record the final operation status");
    assert_eq!(
        u16::from(final_status.status_code()),
        404,
        "diagnostics status_code should be 404 for missing item, got: {:?}",
        final_status.status_code(),
    );

    backend.cleanup_real_database(&db_name).await;
}
