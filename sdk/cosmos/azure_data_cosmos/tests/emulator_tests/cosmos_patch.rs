// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![cfg(feature = "key_auth")]

// Use the shared test framework declared in `tests/emulator/mod.rs`.
use super::framework;

use azure_core::{http::StatusCode, Uuid};
use azure_data_cosmos::clients::ContainerClient;
#[cfg(feature = "fault_injection")]
use azure_data_cosmos::fault_injection::{
    CustomResponseBuilder, FaultInjectionClientBuilder, FaultInjectionConditionBuilder,
    FaultInjectionResultBuilder, FaultInjectionRuleBuilder, FaultOperationType,
};
use azure_data_cosmos::models::{ContainerProperties, ItemResponse};
use azure_data_cosmos::{PatchItemOptions, PatchOp, PatchSpec};
use framework::TestClient;
#[cfg(feature = "fault_injection")]
use framework::TestOptions;
use framework::TestRunContext;
use serde::{Deserialize, Serialize};
use std::error::Error;
#[cfg(feature = "fault_injection")]
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
struct PatchTestItem {
    id: String,
    partition_key: String,
    display_name: String,
    visits: i64,
    deleted: bool,
}

async fn create_container(run_context: &TestRunContext) -> azure_core::Result<ContainerClient> {
    let db_client = run_context.create_db().await?;
    let container_id = format!("Container-{}", Uuid::new_v4());
    run_context
        .create_container(
            &db_client,
            ContainerProperties::new(container_id.clone(), "/partition_key".into()),
            None,
        )
        .await?;
    let container_client = db_client.container_client(&container_id).await?;
    Ok(container_client)
}

/// SDK-level happy path through [`ContainerClient::patch_item`].
///
/// Exercises the public `azure_data_cosmos` API end-to-end: it creates an
/// item, issues a [`PatchSpec`] mixing `Set`, `Increment`, and `Replace`,
/// then verifies that:
///
/// * the response is HTTP 200 with diagnostics populated,
/// * the response body is the locally-merged post-image (the driver
///   synthesizes it regardless of `content_response_on_write`), and
/// * a fresh `read_item` observes the same merged state — i.e. the
///   RMW Replace actually landed on the service.
///
/// This pins the public surface in addition to the driver-level unit
/// tests in `azure_data_cosmos_driver::driver::pipeline::patch_handler`.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn patch_item_round_trip() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();
            let item_id = format!("patch-item-{unique_id}");
            let pk = format!("pk-{unique_id}");

            let initial = PatchTestItem {
                id: item_id.clone(),
                partition_key: pk.clone(),
                display_name: "before".into(),
                visits: 0,
                deleted: false,
            };

            container_client
                .create_item(&pk, &item_id, &initial, None)
                .await?;

            let patch = PatchSpec::new(vec![
                PatchOp::set("/deleted", serde_json::json!(true)),
                PatchOp::increment("/visits", 3i64),
                PatchOp::replace("/display_name", serde_json::json!("after")),
            ]);

            let patch_response: ItemResponse = container_client
                .patch_item(&pk, &item_id, patch, None)
                .await?;
            assert_eq!(patch_response.status(), StatusCode::Ok);

            // Diagnostics must be populated — the handler tracks the
            // sub-requests (Read + Replace) under one operation.
            let diagnostics = patch_response.diagnostics();
            assert!(
                !diagnostics.activity_id().as_str().is_empty(),
                "expected activity ID to be non-empty"
            );
            assert!(
                diagnostics.request_count() >= 1,
                "expected at least one tracked sub-request, got {}",
                diagnostics.request_count(),
            );

            // The driver always returns the locally-merged post-image —
            // even though `content_response_on_write` was not enabled.
            let post_image: PatchTestItem = patch_response.into_model()?;
            assert_eq!(post_image.id, item_id);
            assert_eq!(post_image.partition_key, pk);
            assert_eq!(post_image.display_name, "after");
            assert_eq!(post_image.visits, 3);
            assert!(post_image.deleted);

            // Round-trip: a fresh read sees the same merged state, which
            // means the RMW Replace actually persisted.
            let read_response = container_client
                .read_item(&pk, &item_id, None)
                .await?;
            assert_eq!(read_response.status(), StatusCode::Ok);
            let read_item: PatchTestItem = read_response.into_model()?;
            assert_eq!(read_item, post_image);

            Ok(())
        },
        None,
    )
    .await
}

/// PATCH against a never-created item id surfaces a typed `NotFound`
/// error without retries or replace attempts.
///
/// This is the SDK-surface mirror of the driver-level emulator test
/// `cosmos_patch_read_missing_item_returns_not_found` and the unit test
/// `rmw_propagates_read_error_immediately`.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn patch_item_missing_returns_not_found() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();
            let missing_id = format!("missing-{unique_id}");
            let pk = format!("pk-{unique_id}");

            let patch = PatchSpec::new(vec![PatchOp::set("/deleted", serde_json::json!(true))]);
            let err = container_client
                .patch_item(&pk, &missing_id, patch, None)
                .await
                .expect_err("expected NotFound, got Ok");
            assert_eq!(
                err.http_status(),
                Some(StatusCode::NotFound),
                "expected 404 NotFound from the read leg; got: {err}",
            );

            Ok(())
        },
        None,
    )
    .await
}

/// `PatchItemOptions::with_max_attempts(1)` reaches the service: pinning
/// that the option survives the SDK → driver translation for the
/// happy-path (single-attempt) flow.
///
/// The retry-loop behavior itself is covered end-to-end against a forced
/// 412 by [`patch_item_412_retry_succeeds`] (single 412 → retries and
/// succeeds) and [`patch_item_412_exhaustion_surfaces_precondition_failed`]
/// (persistent 412 → surfaces a typed `PreconditionFailed` error after
/// exhausting `max_attempts`). The dispatcher-driven unit tests
/// `rmw_recovers_from_412_on_first_replace` and
/// `rmw_propagates_412_after_exhausting_max_attempts` in
/// `azure_data_cosmos_driver::driver::pipeline::patch_handler` cover the
/// underlying loop semantics.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn patch_item_honors_max_attempts_option() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();
            let item_id = format!("patch-max-attempts-{unique_id}");
            let pk = format!("pk-{unique_id}");

            let initial = PatchTestItem {
                id: item_id.clone(),
                partition_key: pk.clone(),
                display_name: "x".into(),
                visits: 0,
                deleted: false,
            };

            container_client
                .create_item(&pk, &item_id, &initial, None)
                .await?;

            let options =
                PatchItemOptions::default().with_max_attempts(std::num::NonZeroU8::new(1).unwrap());
            let patch = PatchSpec::new(vec![PatchOp::increment("/visits", 1i64)]);
            let response: ItemResponse = container_client
                .patch_item(&pk, &item_id, patch, Some(options))
                .await?;
            assert_eq!(response.status(), StatusCode::Ok);
            let merged: PatchTestItem = response.into_model()?;
            assert_eq!(merged.visits, 1);

            Ok(())
        },
        None,
    )
    .await
}

// ---------------------------------------------------------------------------
// Fault-injected 412 retry + exhaustion at the SDK surface.
//
// Walks the same SDK ContainerClient::patch_item path as the happy-path
// tests above, but routes calls through a fault-injection-aware client so
// the internal ReplaceItem sub-op of the driver RMW loop returns a
// synthetic 412. These mirror the driver-level emulator tests
// `cosmos_patch_412_retry` and `cosmos_patch_412_exhaustion`.
// ---------------------------------------------------------------------------

/// Build a [`FaultInjectionRule`] that returns a synthetic 412 for every
/// `ReplaceItem` request, with an optional `hit_limit` to cap how many
/// times it fires.
#[cfg(feature = "fault_injection")]
fn build_replace_412_rule(
    name: &str,
    hit_limit: Option<u32>,
) -> Arc<azure_data_cosmos::fault_injection::FaultInjectionRule> {
    let custom_412 = CustomResponseBuilder::new(StatusCode::PreconditionFailed)
        .with_body(br#"{"code":"PreconditionFailed","message":"injected 412"}"#.to_vec())
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_custom_response(custom_412)
        .build();
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReplaceItem)
        .build();
    let mut rule = FaultInjectionRuleBuilder::new(name, result).with_condition(condition);
    if let Some(limit) = hit_limit {
        rule = rule.with_hit_limit(limit);
    }
    Arc::new(rule.build())
}

/// Create a fresh container under `db_client`, seed it with `initial`, and
/// return `(regular_container, fault_container, item_id, pk)`. The fault
/// container is bound to the fault-injection-aware `CosmosClient` exposed
/// by `run_context.fault_client()`, so calls through it are subject to the
/// fault rules registered on `TestOptions`.
#[cfg(feature = "fault_injection")]
async fn setup_fault_injected_container(
    run_context: &TestRunContext,
    db_client: &azure_data_cosmos::clients::DatabaseClient,
    initial: &PatchTestItem,
) -> Result<(ContainerClient, ContainerClient, String, String), Box<dyn Error>> {
    let container_id = format!("Container-{}", Uuid::new_v4());
    run_context
        .create_container(
            db_client,
            ContainerProperties::new(container_id.clone(), "/partition_key".into()),
            None,
        )
        .await?;

    let regular = db_client.container_client(&container_id).await?;
    regular
        .create_item(&initial.partition_key, &initial.id, initial, None)
        .await?;

    let fault_client = run_context
        .fault_client()
        .expect("fault client should be configured");
    let fault_db_client = fault_client.database_client(db_client.id());
    let fault_container = fault_db_client.container_client(&container_id).await?;

    Ok((
        regular,
        fault_container,
        initial.id.clone(),
        initial.partition_key.clone(),
    ))
}

/// Driver RMW retries on a single fault-injected 412 on the internal
/// `ReplaceItem` and the overall PATCH eventually succeeds at the SDK
/// surface.
///
/// Mirrors the driver-level emulator test `cosmos_patch_412_retry`.
#[cfg(feature = "fault_injection")]
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn patch_item_412_retry_succeeds() -> Result<(), Box<dyn Error>> {
    let rule = build_replace_412_rule("sdk-patch-412-once", Some(1));
    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::clone(&rule));
    let options = TestOptions::new().with_fault_injection_builder(fault_builder);

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let unique_id = Uuid::new_v4().to_string();
            let initial = PatchTestItem {
                id: format!("patch-412-retry-{unique_id}"),
                partition_key: format!("pk-{unique_id}"),
                display_name: "before".into(),
                visits: 0,
                deleted: false,
            };
            let (regular, fault_container, item_id, pk) =
                setup_fault_injected_container(run_context, db_client, &initial).await?;

            let patch = PatchSpec::new(vec![PatchOp::increment("/visits", 1i64)]);
            let response: ItemResponse = fault_container
                .patch_item(&pk, &item_id, patch, None)
                .await?;
            assert_eq!(
                response.status(),
                StatusCode::Ok,
                "PATCH should succeed after one retried 412"
            );

            let merged: PatchTestItem = response.into_model()?;
            assert_eq!(
                merged.visits, 1,
                "post-image should reflect the locally-merged Increment"
            );

            // The fault rule fired exactly once — the first Replace hit
            // it; the retry's Replace went to the live emulator.
            assert_eq!(
                rule.hit_count(),
                1,
                "fault rule should fire exactly once on the first attempt; got {}",
                rule.hit_count()
            );

            // A fresh read sees the same merged state — the retry's
            // Replace actually persisted on the service.
            let read_response = regular
                .read_item(&pk, &item_id, None)
                .await?;
            let read_item: PatchTestItem = read_response.into_model()?;
            assert_eq!(read_item, merged);

            Ok(())
        },
        Some(options),
    )
    .await
}

/// Persistent fault-injected 412 on every internal `ReplaceItem` exhausts
/// `PatchItemOptions::max_attempts(2)` and the SDK surfaces a typed
/// `PreconditionFailed` error.
///
/// Mirrors the driver-level emulator test `cosmos_patch_412_exhaustion`.
#[cfg(feature = "fault_injection")]
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn patch_item_412_exhaustion_surfaces_precondition_failed() -> Result<(), Box<dyn Error>>
{
    let rule = build_replace_412_rule("sdk-patch-412-always", None);
    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::clone(&rule));
    let options = TestOptions::new().with_fault_injection_builder(fault_builder);

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let unique_id = Uuid::new_v4().to_string();
            let initial = PatchTestItem {
                id: format!("patch-412-exhaust-{unique_id}"),
                partition_key: format!("pk-{unique_id}"),
                display_name: "before".into(),
                visits: 0,
                deleted: false,
            };
            let (_regular, fault_container, item_id, pk) =
                setup_fault_injected_container(run_context, db_client, &initial).await?;

            let max_attempts = std::num::NonZeroU8::new(2).unwrap();
            let patch_options = PatchItemOptions::default().with_max_attempts(max_attempts);
            let patch = PatchSpec::new(vec![PatchOp::increment("/visits", 1i64)]);

            let err = fault_container
                .patch_item(&pk, &item_id, patch, Some(patch_options))
                .await
                .expect_err("PATCH should fail after exhausting max_attempts");
            assert_eq!(
                err.http_status(),
                Some(StatusCode::PreconditionFailed),
                "exhausted PATCH should surface 412 PreconditionFailed; got: {err}"
            );

            // One injection per attempt — max_attempts total.
            assert_eq!(
                rule.hit_count(),
                u32::from(max_attempts.get()),
                "fault rule should fire once per attempt; hit_count={} max_attempts={}",
                rule.hit_count(),
                max_attempts.get()
            );

            Ok(())
        },
        Some(options),
    )
    .await
}
