// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![cfg(feature = "key_auth")]

// Use the shared test framework declared in `tests/emulator/mod.rs`.
use super::framework;

use azure_core::{http::StatusCode, Uuid};
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::models::{ContainerProperties, ItemResponse};
use azure_data_cosmos::{PatchItemOptions, PatchOp, PatchSpec};
use framework::TestClient;
use framework::TestRunContext;
use serde::{Deserialize, Serialize};
use std::error::Error;

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

            let patch_response: ItemResponse<PatchTestItem> = container_client
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
                .read_item::<PatchTestItem>(&pk, &item_id, None)
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
                .patch_item::<serde_json::Value>(&pk, &missing_id, patch, None)
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
/// We can't reliably force a 412 from the emulator here, so this test
/// only asserts the option does not break a normal patch. The retry-loop
/// behavior is covered exhaustively by the dispatcher-driven unit tests
/// (`rmw_recovers_from_412_on_first_replace` and
/// `rmw_propagates_412_after_exhausting_max_attempts`).
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
            let response: ItemResponse<PatchTestItem> = container_client
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
