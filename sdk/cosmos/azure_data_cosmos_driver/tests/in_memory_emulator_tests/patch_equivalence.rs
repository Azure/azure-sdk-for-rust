// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Equivalence tests for the two patch execution paths.
//!
//! A patch runs either server-side (one `PATCH` request) or client-side (read,
//! merge locally, conditional replace). Callers pick between them for latency
//! and conflict-resolution reasons, not for semantics — so the two paths must
//! produce the same document and the same success/error signal for the same
//! input. Anything else turns [`PatchStrategy`] into a behavioral switch and
//! makes [`PatchStrategy::Auto`], which chooses on the caller's behalf,
//! unsafe.
//!
//! Each test runs one patch through both strategies against the same seeded
//! item and compares the outcome. Divergences that are inherent to the two
//! designs are asserted explicitly rather than quietly skipped:
//!
//! - **Request charge** differs — the client-side loop pays for a read plus a
//!   replace, so it is strictly more expensive.
//! - **System properties** (`_rid`, `_self`, `_etag`, `_ts`) differ because the
//!   item is re-created between the two runs.
//!
//! [`PatchStrategy`]: azure_data_cosmos_driver::options::PatchStrategy
//! [`PatchStrategy::Auto`]: azure_data_cosmos_driver::options::PatchStrategy::Auto

use std::sync::Arc;

use azure_core::http::Url;

use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion,
};
use azure_data_cosmos_driver::models::{
    AccountReference, ContainerReference, CosmosOperation, CosmosResponse, ItemReference,
    PartitionKey, PatchInstructions, PatchOperation,
};
use azure_data_cosmos_driver::options::{
    ContentResponseOnWrite, DriverOptions, OperationOptions, OperationOptionsBuilder, PatchStrategy,
};
use azure_data_cosmos_driver::CosmosDriver;

use super::host_recorder::HostRecorder;

const GATEWAY_URL: &str = "https://eastus.emulator.local";
const PK: &str = "pk1";
const ITEM_ID: &str = "equivalence-item";

fn account() -> AccountReference {
    AccountReference::with_master_key(Url::parse(GATEWAY_URL).unwrap(), "ZW11bGF0b3Ita2V5")
}

async fn build_driver() -> (Arc<InMemoryEmulatorHttpClient>, Arc<CosmosDriver>) {
    build_driver_with_recorder(None).await
}

async fn build_driver_with_recorder(
    recorder: Option<Arc<HostRecorder>>,
) -> (Arc<InMemoryEmulatorHttpClient>, Arc<CosmosDriver>) {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);

    let emulator = InMemoryEmulatorHttpClient::new(config);
    let emulator = match recorder {
        Some(recorder) => emulator.with_request_observer(recorder),
        None => emulator,
    };
    let store = emulator.store();
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
    let emulator = Arc::new(emulator);

    let runtime = emulator
        .runtime_builder()
        .build()
        .await
        .expect("runtime should build against the in-memory emulator");
    let driver = runtime
        .create_driver(DriverOptions::builder(account()).build())
        .await
        .expect("driver should initialize");

    (emulator, driver)
}

/// Everything a caller can observe about one patch attempt.
#[derive(Debug)]
struct Outcome {
    /// `Ok` carries the post-patch document with system properties stripped;
    /// `Err` carries the `(status, sub_status)` pair.
    result: Result<serde_json::Value, (u16, Option<u32>)>,
    etag: Option<String>,
    session_token: Option<String>,
}

fn strip_system_properties(mut doc: serde_json::Value) -> serde_json::Value {
    if let Some(map) = doc.as_object_mut() {
        map.retain(|key, _| !key.starts_with('_'));
    }
    doc
}

fn body_json(response: CosmosResponse) -> serde_json::Value {
    let bytes = response.into_body().single().unwrap_or_default();
    serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null)
}

fn outcome_from(response: CosmosResponse) -> Outcome {
    let headers = response.headers();
    let etag = headers.etag.as_ref().map(|e| e.to_string());
    let session_token = headers.session_token.as_ref().map(|t| t.to_string());
    let body = body_json(response);
    Outcome {
        result: Ok(strip_system_properties(body)),
        etag,
        session_token,
    }
}

fn outcome_from_error(error: azure_data_cosmos_driver::error::CosmosError) -> Outcome {
    let status = error.status();
    Outcome {
        result: Err((
            u16::from(status.status_code()),
            status.sub_status().map(|s| s.value() as u32),
        )),
        etag: None,
        session_token: None,
    }
}

async fn seed_item(
    driver: &CosmosDriver,
    container: &ContainerReference,
    seed: &serde_json::Value,
) {
    let item = ItemReference::from_name(container, PartitionKey::from(PK), ITEM_ID.to_string());
    driver
        .execute_operation(
            CosmosOperation::create_item(item).with_body(seed.to_string().into_bytes()),
            OperationOptions::default(),
        )
        .await
        .expect("seeding the item must succeed");
}

async fn delete_item_if_present(driver: &CosmosDriver, container: &ContainerReference) {
    let item = ItemReference::from_name(container, PartitionKey::from(PK), ITEM_ID.to_string());
    let _ = driver
        .execute_operation(
            CosmosOperation::delete_item(item),
            OperationOptions::default(),
        )
        .await;
}

/// Reads the item back, so the assertion covers what actually landed in the
/// store rather than only what the response echoed.
async fn read_item(
    driver: &CosmosDriver,
    container: &ContainerReference,
) -> Option<serde_json::Value> {
    let item = ItemReference::from_name(container, PartitionKey::from(PK), ITEM_ID.to_string());
    let response = driver
        .execute_operation(
            CosmosOperation::read_item(item),
            OperationOptions::default(),
        )
        .await
        .ok()??;
    Some(strip_system_properties(body_json(response)))
}

async fn run_with_strategy(
    driver: &CosmosDriver,
    container: &ContainerReference,
    seed: &serde_json::Value,
    patch: &PatchInstructions,
    strategy: PatchStrategy,
) -> (Outcome, Option<serde_json::Value>) {
    delete_item_if_present(driver, container).await;
    seed_item(driver, container, seed).await;

    let item = ItemReference::from_name(container, PartitionKey::from(PK), ITEM_ID.to_string());
    let operation = CosmosOperation::patch_item(item).with_body(serde_json::to_vec(patch).unwrap());
    let options = OperationOptionsBuilder::new()
        .with_patch_strategy(strategy)
        .with_content_response_on_write(ContentResponseOnWrite::Enabled)
        .build();

    let outcome = match driver.execute_operation(operation, options).await {
        Ok(Some(response)) => outcome_from(response),
        Ok(None) => panic!("patch must produce a response"),
        Err(error) => outcome_from_error(error),
    };

    let stored = read_item(driver, container).await;
    (outcome, stored)
}

/// Runs `patch` against a freshly seeded `seed` under both strategies and
/// asserts the two are indistinguishable to the caller.
async fn assert_equivalent(seed: serde_json::Value, patch: PatchInstructions, scenario: &str) {
    let (_emulator, driver) = build_driver().await;
    let container = driver
        .resolve_container("testdb", "testcoll")
        .await
        .expect("container should resolve");

    let (client_side, client_stored) = run_with_strategy(
        &driver,
        &container,
        &seed,
        &patch,
        PatchStrategy::ClientSide,
    )
    .await;
    let (server_side, server_stored) = run_with_strategy(
        &driver,
        &container,
        &seed,
        &patch,
        PatchStrategy::ServerSide,
    )
    .await;

    match (&client_side.result, &server_side.result) {
        (Ok(client_doc), Ok(server_doc)) => {
            assert_eq!(
                client_doc, server_doc,
                "[{scenario}] response bodies diverge between strategies"
            );
            assert_eq!(
                client_stored, server_stored,
                "[{scenario}] stored documents diverge between strategies"
            );
            assert_eq!(
                client_stored.as_ref(),
                Some(client_doc),
                "[{scenario}] client-side response does not match what was stored"
            );
            assert!(
                client_side.etag.is_some() && server_side.etag.is_some(),
                "[{scenario}] both strategies must return an etag on success"
            );
            assert!(
                client_side.session_token.is_some() && server_side.session_token.is_some(),
                "[{scenario}] both strategies must advance the session token"
            );
        }
        (Err(client_status), Err(server_status)) => {
            assert_eq!(
                client_status, server_status,
                "[{scenario}] error status/sub-status diverge between strategies"
            );
            assert_eq!(
                client_stored, server_stored,
                "[{scenario}] a failed patch must leave the same document under both strategies"
            );
        }
        (client, server) => panic!(
            "[{scenario}] strategies disagree on success vs failure: \
             client-side = {client:?}, server-side = {server:?}"
        ),
    }
}

fn seed_doc() -> serde_json::Value {
    serde_json::json!({
        "id": ITEM_ID,
        "pk": PK,
        "name": "original",
        "visits": 1,
        "ratio": 1.5,
        "nested": { "kept": true, "replaced": "before" },
        "tags": ["a", "b"],
    })
}

#[tokio::test]
async fn set_is_equivalent() {
    assert_equivalent(
        seed_doc(),
        PatchInstructions::from(vec![PatchOperation::set(
            "/name",
            serde_json::json!("updated"),
        )]),
        "set",
    )
    .await;
}

#[tokio::test]
async fn set_on_a_new_member_is_equivalent() {
    assert_equivalent(
        seed_doc(),
        PatchInstructions::from(vec![PatchOperation::set(
            "/nested/added",
            serde_json::json!(7),
        )]),
        "set new member",
    )
    .await;
}

#[tokio::test]
async fn replace_is_equivalent() {
    assert_equivalent(
        seed_doc(),
        PatchInstructions::from(vec![PatchOperation::replace(
            "/nested/replaced",
            serde_json::json!("after"),
        )]),
        "replace",
    )
    .await;
}

#[tokio::test]
async fn add_object_member_is_equivalent() {
    assert_equivalent(
        seed_doc(),
        PatchInstructions::from(vec![PatchOperation::add(
            "/nested/fresh",
            serde_json::json!("v"),
        )]),
        "add object member",
    )
    .await;
}

#[tokio::test]
async fn array_append_is_equivalent() {
    assert_equivalent(
        seed_doc(),
        PatchInstructions::from(vec![PatchOperation::add("/tags/-", serde_json::json!("c"))]),
        "array append",
    )
    .await;
}

#[tokio::test]
async fn remove_is_equivalent() {
    assert_equivalent(
        seed_doc(),
        PatchInstructions::from(vec![PatchOperation::remove("/nested/replaced")]),
        "remove",
    )
    .await;
}

#[tokio::test]
async fn integer_increment_is_equivalent() {
    assert_equivalent(
        seed_doc(),
        PatchInstructions::from(vec![PatchOperation::increment("/visits", 5i64)]),
        "integer increment",
    )
    .await;
}

/// Guards `CosmosNumber`'s int/float split across both evaluators: a float
/// delta must not be silently truncated by one path and not the other.
#[tokio::test]
async fn float_increment_is_equivalent() {
    assert_equivalent(
        seed_doc(),
        PatchInstructions::from(vec![PatchOperation::increment("/ratio", 0.25f64)]),
        "float increment",
    )
    .await;
}

#[tokio::test]
async fn move_is_equivalent() {
    assert_equivalent(
        seed_doc(),
        PatchInstructions::from(vec![PatchOperation::move_value("/name", "/renamed")]),
        "move",
    )
    .await;
}

#[tokio::test]
async fn multiple_operations_apply_in_order_under_both_strategies() {
    assert_equivalent(
        seed_doc(),
        PatchInstructions::from(vec![
            PatchOperation::set("/name", serde_json::json!("first")),
            PatchOperation::set("/name", serde_json::json!("second")),
            PatchOperation::increment("/visits", 2i64),
            PatchOperation::remove("/nested/replaced"),
        ]),
        "multiple operations",
    )
    .await;
}

// ── Error-shape equivalence ───────────────────────────────────────────
//
// The failure signal matters as much as the success one: an application that
// branches on a status code must not have to know which path ran.

#[tokio::test]
async fn missing_path_fails_identically() {
    assert_equivalent(
        seed_doc(),
        PatchInstructions::from(vec![PatchOperation::replace(
            "/does/not/exist",
            serde_json::json!(1),
        )]),
        "replace missing path",
    )
    .await;
}

#[tokio::test]
async fn removing_a_missing_path_fails_identically() {
    assert_equivalent(
        seed_doc(),
        PatchInstructions::from(vec![PatchOperation::remove("/absent")]),
        "remove missing path",
    )
    .await;
}

#[tokio::test]
async fn incrementing_a_non_number_fails_identically() {
    assert_equivalent(
        seed_doc(),
        PatchInstructions::from(vec![PatchOperation::increment("/name", 1i64)]),
        "increment non-number",
    )
    .await;
}

#[tokio::test]
async fn array_index_out_of_range_fails_identically() {
    assert_equivalent(
        seed_doc(),
        PatchInstructions::from(vec![PatchOperation::replace(
            "/tags/99",
            serde_json::json!("x"),
        )]),
        "array index out of range",
    )
    .await;
}

#[tokio::test]
async fn moving_from_a_missing_path_fails_identically() {
    assert_equivalent(
        seed_doc(),
        PatchInstructions::from(vec![PatchOperation::move_value("/absent", "/target")]),
        "move from missing path",
    )
    .await;
}

/// Rewriting the partition key would relocate the item, so both paths must
/// refuse it rather than one silently succeeding.
#[tokio::test]
async fn patching_the_partition_key_fails_identically() {
    assert_equivalent(
        seed_doc(),
        PatchInstructions::from(vec![PatchOperation::set("/pk", serde_json::json!("moved"))]),
        "patch partition key",
    )
    .await;
}

// ── Path selection ────────────────────────────────────────────────────

/// Counts the data-plane requests a single patch costs under `strategy`.
async fn data_plane_requests_for(strategy: PatchStrategy, patch: &PatchInstructions) -> usize {
    let recorder = HostRecorder::new();
    let (_emulator, driver) = build_driver_with_recorder(Some(recorder.clone())).await;
    let container = driver
        .resolve_container("testdb", "testcoll")
        .await
        .expect("container should resolve");

    seed_item(&driver, &container, &seed_doc()).await;
    recorder.clear();

    let item = ItemReference::from_name(&container, PartitionKey::from(PK), ITEM_ID.to_string());
    let operation = CosmosOperation::patch_item(item).with_body(serde_json::to_vec(patch).unwrap());
    let options = OperationOptionsBuilder::new()
        .with_patch_strategy(strategy)
        .build();
    driver
        .execute_operation(operation, options)
        .await
        .expect("patch should succeed");

    recorder.data_plane_hosts().len()
}

/// Without this, the equivalence suite above could pass vacuously — if the
/// server-side path silently fell back to the read-modify-write loop, every
/// comparison would trivially hold while the feature did nothing.
///
/// The request count is the observable difference: a server-side patch is one
/// request, the client-side loop is a read followed by a replace.
#[tokio::test]
async fn each_strategy_takes_the_path_it_names() {
    let patch = PatchInstructions::from(vec![PatchOperation::set("/name", serde_json::json!("x"))]);

    assert_eq!(
        data_plane_requests_for(PatchStrategy::ServerSide, &patch).await,
        1,
        "a server-side patch must reach the service in a single request"
    );
    assert_eq!(
        data_plane_requests_for(PatchStrategy::ClientSide, &patch).await,
        2,
        "the client-side loop must issue a read and a replace"
    );
}

/// `Auto` is the default, and its whole value is picking the cheap path when
/// that is safe and stepping back when it is not.
#[tokio::test]
async fn auto_uses_the_service_for_safe_operations_and_falls_back_otherwise() {
    let safe = PatchInstructions::from(vec![PatchOperation::set("/name", serde_json::json!("x"))]);
    assert_eq!(
        data_plane_requests_for(PatchStrategy::Auto, &safe).await,
        1,
        "a retry-safe patch should go straight to the service"
    );

    let unsafe_to_resend =
        PatchInstructions::from(vec![PatchOperation::increment("/visits", 1i64)]);
    assert_eq!(
        data_plane_requests_for(PatchStrategy::Auto, &unsafe_to_resend).await,
        2,
        "an increment could be applied twice on resend, so Auto must use the loop"
    );

    let too_many = PatchInstructions::from(
        (0..11)
            .map(|i| PatchOperation::set(format!("/f{i}"), serde_json::json!(i)))
            .collect::<Vec<_>>(),
    );
    assert_eq!(
        data_plane_requests_for(PatchStrategy::Auto, &too_many).await,
        2,
        "a list over the service's operation limit must not be sent"
    );
}

/// `ServerSide` is documented as honored without fallback, so an over-long list
/// must surface the service's own rejection rather than quietly switching to
/// the loop.
#[tokio::test]
async fn server_side_surfaces_the_service_operation_limit() {
    let (_emulator, driver) = build_driver().await;
    let container = driver
        .resolve_container("testdb", "testcoll")
        .await
        .expect("container should resolve");
    seed_item(&driver, &container, &seed_doc()).await;

    let patch = PatchInstructions::from(
        (0..11)
            .map(|i| PatchOperation::set(format!("/f{i}"), serde_json::json!(i)))
            .collect::<Vec<_>>(),
    );
    let item = ItemReference::from_name(&container, PartitionKey::from(PK), ITEM_ID.to_string());
    let operation =
        CosmosOperation::patch_item(item).with_body(serde_json::to_vec(&patch).unwrap());
    let options = OperationOptionsBuilder::new()
        .with_patch_strategy(PatchStrategy::ServerSide)
        .build();

    let error = driver
        .execute_operation(operation, options)
        .await
        .expect_err("an over-long list must be rejected");
    assert_eq!(
        u16::from(error.status().status_code()),
        400,
        "expected the service's 400, got {error}"
    );
}

// ── Response body ─────────────────────────────────────────────────────

/// The post-image must not depend on which path ran. Before server-side patch
/// existed, `patch_item` always returned it; leaving `content_response_on_write`
/// unset must not silently turn that off for a `set`-only list.
#[tokio::test]
async fn the_post_image_is_returned_without_configuring_content_response() {
    for strategy in [
        None,
        Some(PatchStrategy::Auto),
        Some(PatchStrategy::ServerSide),
        Some(PatchStrategy::ClientSide),
    ] {
        let (_emulator, driver) = build_driver().await;
        let container = driver
            .resolve_container("testdb", "testcoll")
            .await
            .expect("container should resolve");
        seed_item(&driver, &container, &seed_doc()).await;

        let patch = PatchInstructions::from(vec![PatchOperation::set(
            "/name",
            serde_json::json!("after"),
        )]);
        let item =
            ItemReference::from_name(&container, PartitionKey::from(PK), ITEM_ID.to_string());
        let operation =
            CosmosOperation::patch_item(item).with_body(serde_json::to_vec(&patch).unwrap());
        let mut builder = OperationOptionsBuilder::new();
        if let Some(strategy) = strategy {
            builder = builder.with_patch_strategy(strategy);
        }

        let response = driver
            .execute_operation(operation, builder.build())
            .await
            .expect("patch should succeed")
            .expect("patch must produce a response");
        assert_eq!(
            body_json(response)["name"],
            serde_json::json!("after"),
            "{strategy:?} returned no post-image with content_response_on_write unset"
        );
    }
}

/// An explicit `Disabled` still opts out, so the default above is a default and
/// not an override.
#[tokio::test]
async fn an_explicit_disable_still_suppresses_the_post_image() {
    let (_emulator, driver) = build_driver().await;
    let container = driver
        .resolve_container("testdb", "testcoll")
        .await
        .expect("container should resolve");
    seed_item(&driver, &container, &seed_doc()).await;

    let patch = PatchInstructions::from(vec![PatchOperation::set(
        "/name",
        serde_json::json!("after"),
    )]);
    let item = ItemReference::from_name(&container, PartitionKey::from(PK), ITEM_ID.to_string());
    let operation =
        CosmosOperation::patch_item(item).with_body(serde_json::to_vec(&patch).unwrap());
    let options = OperationOptionsBuilder::new()
        .with_patch_strategy(PatchStrategy::ServerSide)
        .with_content_response_on_write(ContentResponseOnWrite::Disabled)
        .build();

    let response = driver
        .execute_operation(operation, options)
        .await
        .expect("patch should succeed")
        .expect("patch must produce a response");
    assert_eq!(body_json(response), serde_json::Value::Null);
}
