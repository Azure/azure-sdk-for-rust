// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Use the shared test framework declared in `tests/emulator_tests/mod.rs`.
use super::framework;

use azure_core::{http::StatusCode, Uuid};
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::diagnostics::TransportKind;
use azure_data_cosmos::fault_injection::{
    CustomResponseBuilder, FaultInjectionConditionBuilder, FaultInjectionResultBuilder,
    FaultInjectionRuleBuilder, FaultOperationType,
};
use azure_data_cosmos::models::{ContainerProperties, ItemResponse, PATCH_TRACKING_PROPERTY};
use azure_data_cosmos::models::{PatchInstructions, PatchOperation};
use azure_data_cosmos::options::{
    ContentResponseOnWrite, OperationOptions, PatchItemOptions, PatchStrategy, Precondition,
};
use framework::TestClient;
use framework::TestOptions;
use framework::TestRunContext;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
struct PatchTestItem {
    id: String,
    partition_key: String,
    display_name: String,
    visits: i64,
    deleted: bool,
}

fn strip_system_properties(mut value: serde_json::Value) -> serde_json::Value {
    if let Some(object) = value.as_object_mut() {
        object.retain(|name, _| !name.starts_with('_'));
    }
    value
}

async fn create_container(
    run_context: &TestRunContext,
) -> azure_data_cosmos::Result<ContainerClient> {
    let db_client = run_context.create_db().await?;
    create_container_in_database(run_context, &db_client).await
}

async fn create_container_in_database(
    run_context: &TestRunContext,
    db_client: &azure_data_cosmos::clients::DatabaseClient,
) -> azure_data_cosmos::Result<ContainerClient> {
    let container_id = format!("Container-{}", Uuid::new_v4());
    run_context
        .create_container(
            db_client,
            ContainerProperties::new(container_id.clone(), "/partition_key".into()),
            None,
        )
        .await?;
    let container_client = db_client.container_client(&container_id, None).await?;
    Ok(container_client)
}

/// SDK-level happy path through [`ContainerClient::patch_item`].
///
/// Exercises the public `azure_data_cosmos` API end-to-end: it creates an
/// item, issues a [`PatchInstructions`] mixing `Set`, `Increment`, and `Replace`,
/// then verifies that:
///
/// * the response is HTTP 200 with diagnostics populated,
/// * the default response body is the locally-merged post-image, and
/// * a fresh `read_item` observes the same merged state — i.e. the
///   RMW Replace actually landed on the service.
///
/// This pins the public surface in addition to the driver-level unit
/// tests in `azure_data_cosmos_driver::driver::pipeline::patch_handler`.
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "emulator",
        test_category = "emulator_vnext",
        test_category = "emulator_inmemory"
    )),
    ignore = "requires test_category 'emulator', 'emulator_vnext', or 'emulator_inmemory'"
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

            let patch = PatchInstructions::from(vec![
                PatchOperation::set("/deleted", serde_json::json!(true)),
                PatchOperation::increment("/visits", 3i64),
                PatchOperation::replace("/display_name", serde_json::json!("after")),
            ]);

            let patch_response: ItemResponse = container_client
                .patch_item(&pk, &item_id, patch, None)
                .await?;
            assert_eq!(patch_response.status(), StatusCode::Ok);
            let effective_tracking_id = patch_response
                .patch_tracking_id()
                .expect("unsafe PATCH exposes its generated tracking ID");

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
            assert_eq!(
                diagnostics.patch_tracking_id().map(|id| id.as_uuid()),
                Some(effective_tracking_id.as_uuid())
            );

            // PATCH defaults to returning the locally-merged post-image.
            let post_image: PatchTestItem = patch_response.into_model()?;
            assert_eq!(post_image.id, item_id);
            assert_eq!(post_image.partition_key, pk);
            assert_eq!(post_image.display_name, "after");
            assert_eq!(post_image.visits, 3);
            assert!(post_image.deleted);

            // Round-trip: a fresh read sees the same merged state, which
            // means the RMW Replace actually persisted.
            let read_response = container_client.read_item(&pk, &item_id, None).await?;
            assert_eq!(read_response.status(), StatusCode::Ok);
            let read_item: PatchTestItem = read_response.into_model()?;
            assert_eq!(read_item, post_image);

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// `content_response_on_write = Disabled` suppresses the public SDK response
/// body for client-side RMW while still committing the mutation.
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "emulator",
        test_category = "emulator_vnext",
        test_category = "emulator_inmemory"
    )),
    ignore = "requires test_category 'emulator', 'emulator_vnext', or 'emulator_inmemory'"
)]
pub async fn patch_item_honors_disabled_content_response() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();
            let item_id = format!("patch-no-content-{unique_id}");
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

            let mut operation = OperationOptions::default();
            operation.content_response_on_write = Some(ContentResponseOnWrite::Disabled);
            let options = PatchItemOptions::default()
                .with_strategy(PatchStrategy::ClientSide)
                .with_operation_options(operation);
            let response = container_client
                .patch_item(
                    &pk,
                    &item_id,
                    PatchInstructions::from(vec![PatchOperation::set(
                        "/display_name",
                        serde_json::json!("after"),
                    )]),
                    Some(options),
                )
                .await?;

            assert!(response.into_body().is_empty());
            let stored: PatchTestItem = container_client
                .read_item(&pk, &item_id, None)
                .await?
                .into_model()?;
            assert_eq!(stored.display_name, "after");

            Ok(())
        },
        Some(TestOptions::for_emulator()),
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
    not(any(
        test_category = "emulator",
        test_category = "emulator_vnext",
        test_category = "emulator_inmemory"
    )),
    ignore = "requires test_category 'emulator', 'emulator_vnext', or 'emulator_inmemory'"
)]
pub async fn patch_item_missing_returns_not_found() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();
            let missing_id = format!("missing-{unique_id}");
            let pk = format!("pk-{unique_id}");

            let patch = PatchInstructions::from(vec![PatchOperation::set(
                "/deleted",
                serde_json::json!(true),
            )]);
            let err = container_client
                .patch_item(&pk, &missing_id, patch, None)
                .await
                .expect_err("expected NotFound, got Ok");
            assert_eq!(
                err.status().status_code(),
                StatusCode::NotFound,
                "expected 404 NotFound from the read leg; got: {err}",
            );

            Ok(())
        },
        Some(TestOptions::for_emulator()),
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
    not(any(
        test_category = "emulator",
        test_category = "emulator_vnext",
        test_category = "emulator_inmemory"
    )),
    ignore = "requires test_category 'emulator', 'emulator_vnext', or 'emulator_inmemory'"
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
            let patch = PatchInstructions::from(vec![PatchOperation::increment("/visits", 1i64)]);
            let response: ItemResponse = container_client
                .patch_item(&pk, &item_id, patch, Some(options))
                .await?;
            assert_eq!(response.status(), StatusCode::Ok);
            let merged: PatchTestItem = response.into_model()?;
            assert_eq!(merged.visits, 1);

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "emulator",
        test_category = "emulator_vnext",
        test_category = "emulator_inmemory"
    )),
    ignore = "requires test_category 'emulator', 'emulator_vnext', or 'emulator_inmemory'"
)]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: behavioral divergence"
)]
pub async fn patch_strategy_obeys_service_instruction_limit() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();
            let item_id = format!("patch-limit-{unique_id}");
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

            let instructions = PatchInstructions::from(
                (0..11)
                    .map(|index| {
                        PatchOperation::set(format!("/field{index}"), serde_json::json!(index))
                    })
                    .collect::<Vec<_>>(),
            );

            let auto_response = container_client
                .patch_item(&pk, &item_id, instructions.clone(), None)
                .await?;
            assert_eq!(auto_response.status(), StatusCode::Ok);
            assert_eq!(
                auto_response.diagnostics().request_count(),
                2,
                "Auto must use client-side Read+Replace for 11 instructions"
            );

            let server_options =
                PatchItemOptions::default().with_strategy(PatchStrategy::ServerSide);
            let error = container_client
                .patch_item(&pk, &item_id, instructions, Some(server_options))
                .await
                .expect_err("explicit ServerSide must surface the service limit");
            assert_eq!(error.status().status_code(), StatusCode::BadRequest);

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "emulator",
        test_category = "emulator_vnext",
        test_category = "emulator_inmemory"
    )),
    ignore = "requires test_category 'emulator', 'emulator_vnext', or 'emulator_inmemory'"
)]
pub async fn patch_item_auto_commits_safe_list_server_side() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();
            let item_id = format!("patch-auto-server-{unique_id}");
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

            let response = container_client
                .patch_item(
                    &pk,
                    &item_id,
                    PatchInstructions::from(vec![
                        PatchOperation::set("/deleted", serde_json::json!(true)),
                        PatchOperation::replace("/display_name", serde_json::json!("after")),
                    ]),
                    None,
                )
                .await?;

            assert_eq!(response.status(), StatusCode::Ok);
            assert_eq!(
                response.diagnostics().request_count(),
                1,
                "safe Auto PATCH must use one server-side request"
            );
            let post_image: PatchTestItem = response.into_model()?;
            assert_eq!(post_image.display_name, "after");
            assert!(post_image.deleted);

            let stored: PatchTestItem = container_client
                .read_item(&pk, &item_id, None)
                .await?
                .into_model()?;
            assert_eq!(stored, post_image);

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator_inmemory_gateway_v2"),
    ignore = "requires hosted in-memory emulator with Gateway V2 enabled"
)]
pub async fn patch_item_server_side_round_trips_over_gateway_v2() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;

            for (content_response, expect_body) in [
                (ContentResponseOnWrite::Enabled, true),
                (ContentResponseOnWrite::Disabled, false),
            ] {
                let unique_id = Uuid::new_v4().to_string();
                let item_id = format!("patch-gateway-v2-{expect_body}-{unique_id}");
                let pk = format!("pk-{unique_id}");
                let initial = PatchTestItem {
                    id: item_id.clone(),
                    partition_key: pk.clone(),
                    display_name: "before".into(),
                    visits: 0,
                    deleted: false,
                };
                let create_response = container_client
                    .create_item(&pk, &item_id, &initial, None)
                    .await?;
                for request in create_response.diagnostics().requests().iter() {
                    assert_eq!(
                        request.transport_kind(),
                        TransportKind::GatewayV2,
                        "hosted Gateway V2 fixture must route the seed Create over Gateway V2"
                    );
                }

                let mut operation_options = OperationOptions::default();
                operation_options.content_response_on_write = Some(content_response);
                let options = PatchItemOptions::default()
                    .with_strategy(PatchStrategy::ServerSide)
                    .with_operation_options(operation_options);
                let response = container_client
                    .patch_item(
                        &pk,
                        &item_id,
                        PatchInstructions::from(vec![PatchOperation::set(
                            "/display_name",
                            serde_json::json!("after"),
                        )]),
                        Some(options),
                    )
                    .await?;

                assert_eq!(response.diagnostics().request_count(), 1);
                for request in response.diagnostics().requests().iter() {
                    assert_eq!(request.transport_kind(), TransportKind::GatewayV2);
                }
                assert_eq!(response.into_body().is_empty(), !expect_body);

                let stored: PatchTestItem = container_client
                    .read_item(&pk, &item_id, None)
                    .await?
                    .into_model()?;
                assert_eq!(stored.display_name, "after");
            }

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator_inmemory_gateway_v2"),
    ignore = "requires hosted in-memory emulator with Gateway V2 enabled"
)]
pub async fn patch_item_client_side_round_trips_over_gateway_v2() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();
            let item_id = format!("patch-client-gateway-v2-{unique_id}");
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

            let options = PatchItemOptions::default().with_strategy(PatchStrategy::ClientSide);
            let response = container_client
                .patch_item(
                    &pk,
                    &item_id,
                    PatchInstructions::from(vec![PatchOperation::set(
                        "/display_name",
                        serde_json::json!("after"),
                    )]),
                    Some(options),
                )
                .await?;

            let requests = response.diagnostics().requests();
            assert_eq!(requests.len(), 2);
            assert_eq!(requests[0].operation_name(), Some("patch_read_item"));
            assert_eq!(requests[1].operation_name(), Some("patch_replace_item"));
            assert!(requests
                .iter()
                .all(|request| request.transport_kind() == TransportKind::GatewayV2));

            let post_image: PatchTestItem = response.into_model()?;
            assert_eq!(post_image.display_name, "after");
            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "emulator",
        test_category = "emulator_vnext",
        test_category = "emulator_inmemory"
    )),
    ignore = "requires test_category 'emulator', 'emulator_vnext', or 'emulator_inmemory'"
)]
pub async fn patch_item_server_and_client_strategies_match_service_results(
) -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, db_client| {
            let client_container = create_container_in_database(run_context, db_client).await?;
            let server_container = create_container_in_database(run_context, db_client).await?;
            let client_options =
                PatchItemOptions::default().with_strategy(PatchStrategy::ClientSide);
            let server_options =
                PatchItemOptions::default().with_strategy(PatchStrategy::ServerSide);

            let cases = [
                (
                    "set",
                    PatchOperation::set("/deleted", serde_json::json!(true)),
                ),
                (
                    "replace",
                    PatchOperation::replace("/display_name", serde_json::json!("after")),
                ),
                (
                    "add",
                    PatchOperation::add("/tags/-", serde_json::json!("beta")),
                ),
                ("remove", PatchOperation::remove("/deleted")),
                ("increment", PatchOperation::increment("/visits", 2i64)),
                (
                    "move",
                    PatchOperation::move_value("/source", "/destination"),
                ),
            ];

            for (case_name, operation) in cases {
                let unique_id = Uuid::new_v4().to_string();
                let item_id = format!("patch-equivalence-{case_name}-{unique_id}");
                let pk = format!("pk-{unique_id}");
                let initial = serde_json::json!({
                    "id": item_id,
                    "partition_key": pk,
                    "display_name": "before",
                    "visits": 1,
                    "deleted": false,
                    "tags": ["alpha"],
                    "source": "move-me"
                });

                client_container
                    .create_item(&pk, &item_id, &initial, None)
                    .await?;
                server_container
                    .create_item(&pk, &item_id, &initial, None)
                    .await?;

                let instructions = PatchInstructions::from(vec![operation]);
                let client_should_track = !instructions.is_retry_safe();
                let client_response: serde_json::Value = client_container
                    .patch_item(
                        &pk,
                        &item_id,
                        instructions.clone(),
                        Some(client_options.clone()),
                    )
                    .await?
                    .into_model()?;
                let server_response: serde_json::Value = server_container
                    .patch_item(&pk, &item_id, instructions, Some(server_options.clone()))
                    .await?
                    .into_model()?;

                let client_stored: serde_json::Value = client_container
                    .read_item(&pk, &item_id, None)
                    .await?
                    .into_model()?;
                let server_stored: serde_json::Value = server_container
                    .read_item(&pk, &item_id, None)
                    .await?
                    .into_model()?;
                assert_eq!(
                    client_response.get(PATCH_TRACKING_PROPERTY).is_some(),
                    client_should_track,
                    "client response marker mismatch for {case_name}"
                );
                assert_eq!(
                    client_stored.get(PATCH_TRACKING_PROPERTY).is_some(),
                    client_should_track,
                    "client stored marker mismatch for {case_name}"
                );
                assert!(
                    server_response.get(PATCH_TRACKING_PROPERTY).is_none(),
                    "server response must not contain a marker for {case_name}"
                );
                assert!(
                    server_stored.get(PATCH_TRACKING_PROPERTY).is_none(),
                    "server item must not contain a marker for {case_name}"
                );
                assert_eq!(
                    strip_system_properties(client_stored),
                    strip_system_properties(server_stored),
                    "client-side and server-side PATCH differ for {case_name}"
                );
            }

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "emulator",
        test_category = "emulator_vnext",
        test_category = "emulator_inmemory"
    )),
    ignore = "requires test_category 'emulator', 'emulator_vnext', or 'emulator_inmemory'"
)]
pub async fn patch_item_server_and_client_strategies_match_service_errors(
) -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, db_client| {
            let client_container = create_container_in_database(run_context, db_client).await?;
            let server_container = create_container_in_database(run_context, db_client).await?;
            let client_options =
                PatchItemOptions::default().with_strategy(PatchStrategy::ClientSide);
            let server_options =
                PatchItemOptions::default().with_strategy(PatchStrategy::ServerSide);

            let cases = [
                ("empty list", PatchInstructions::from(Vec::new())),
                (
                    "missing replace path",
                    PatchInstructions::from(vec![PatchOperation::replace(
                        "/missing/leaf",
                        serde_json::json!(1),
                    )]),
                ),
                (
                    "non-number increment",
                    PatchInstructions::from(vec![PatchOperation::increment("/display_name", 1i64)]),
                ),
                (
                    "partition key",
                    PatchInstructions::from(vec![PatchOperation::set(
                        "/partition_key",
                        serde_json::json!("moved"),
                    )]),
                ),
            ];

            for (case_name, instructions) in cases {
                let unique_id = Uuid::new_v4().to_string();
                let item_id = format!("patch-error-equivalence-{case_name}-{unique_id}");
                let pk = format!("pk-{unique_id}");
                let initial = PatchTestItem {
                    id: item_id.clone(),
                    partition_key: pk.clone(),
                    display_name: "before".into(),
                    visits: 1,
                    deleted: false,
                };

                client_container
                    .create_item(&pk, &item_id, &initial, None)
                    .await?;
                server_container
                    .create_item(&pk, &item_id, &initial, None)
                    .await?;

                let client_error = match client_container
                    .patch_item(
                        &pk,
                        &item_id,
                        instructions.clone(),
                        Some(client_options.clone()),
                    )
                    .await
                {
                    Err(error) => error,
                    Ok(response) => {
                        return Err(format!(
                            "client-side PATCH should reject {case_name}; response={response:?}"
                        )
                        .into());
                    }
                };
                let server_error = match server_container
                    .patch_item(&pk, &item_id, instructions, Some(server_options.clone()))
                    .await
                {
                    Err(error) => error,
                    Ok(response) => {
                        return Err(format!(
                            "server-side PATCH should reject {case_name}; response={response:?}"
                        )
                        .into());
                    }
                };

                assert_eq!(
                    client_error.status().status_code(),
                    server_error.status().status_code(),
                    "status mismatch for {case_name}"
                );
                assert_eq!(
                    client_error.status().sub_status(),
                    server_error.status().sub_status(),
                    "substatus mismatch for {case_name}"
                );
            }

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "emulator",
        test_category = "emulator_vnext",
        test_category = "emulator_inmemory"
    )),
    ignore = "requires test_category 'emulator', 'emulator_vnext', or 'emulator_inmemory'"
)]
pub async fn patch_preconditions_match_across_strategies() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;

            for strategy in [PatchStrategy::ClientSide, PatchStrategy::ServerSide] {
                let unique_id = Uuid::new_v4().to_string();
                let item_id = format!("patch-precondition-{strategy}-{unique_id}");
                let pk = format!("pk-{unique_id}");
                let initial = PatchTestItem {
                    id: item_id.clone(),
                    partition_key: pk.clone(),
                    display_name: "before".into(),
                    visits: 0,
                    deleted: false,
                };
                let create_response = container_client
                    .create_item(&pk, &item_id, &initial, None)
                    .await?;
                let initial_etag = create_response
                    .headers()
                    .etag()
                    .expect("create must return an ETag")
                    .clone();

                let matching = PatchItemOptions::default()
                    .with_strategy(strategy)
                    .with_precondition(Precondition::if_match(initial_etag.clone()));
                container_client
                    .patch_item(
                        &pk,
                        &item_id,
                        PatchInstructions::from(vec![PatchOperation::set(
                            "/display_name",
                            serde_json::json!("matched"),
                        )]),
                        Some(matching),
                    )
                    .await?;

                let stale = PatchItemOptions::default()
                    .with_strategy(strategy)
                    .with_precondition(Precondition::if_match(initial_etag));
                let error = container_client
                    .patch_item(
                        &pk,
                        &item_id,
                        PatchInstructions::from(vec![PatchOperation::set(
                            "/deleted",
                            serde_json::json!(true),
                        )]),
                        Some(stale),
                    )
                    .await
                    .expect_err("stale If-Match must reject PATCH");
                assert_eq!(
                    error.status().status_code(),
                    StatusCode::PreconditionFailed,
                    "{strategy} stale If-Match"
                );
            }

            Ok(())
        },
        Some(TestOptions::for_emulator()),
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

    let regular = db_client.container_client(&container_id, None).await?;
    regular
        .create_item(&initial.partition_key, &initial.id, initial, None)
        .await?;

    let fault_client = run_context
        .fault_client()
        .expect("fault client should be configured");
    let fault_db_client = fault_client.database_client(db_client.id());
    let fault_container = fault_db_client
        .container_client(&container_id, None)
        .await?;

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
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "emulator",
        test_category = "emulator_vnext",
        test_category = "emulator_inmemory"
    )),
    ignore = "requires test_category 'emulator', 'emulator_vnext', or 'emulator_inmemory'"
)]
pub async fn patch_item_412_retry_succeeds() -> Result<(), Box<dyn Error>> {
    let rule = build_replace_412_rule("sdk-patch-412-once", Some(1));
    let options = TestOptions::for_emulator().with_fault_injection_rules(vec![Arc::clone(&rule)]);

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

            let patch = PatchInstructions::from(vec![PatchOperation::increment("/visits", 1i64)]);
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
            let read_response = regular.read_item(&pk, &item_id, None).await?;
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
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "emulator",
        test_category = "emulator_vnext",
        test_category = "emulator_inmemory"
    )),
    ignore = "requires test_category 'emulator', 'emulator_vnext', or 'emulator_inmemory'"
)]
pub async fn patch_item_412_exhaustion_surfaces_precondition_failed() -> Result<(), Box<dyn Error>>
{
    let rule = build_replace_412_rule("sdk-patch-412-always", None);
    let options = TestOptions::for_emulator().with_fault_injection_rules(vec![Arc::clone(&rule)]);

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
            let patch = PatchInstructions::from(vec![PatchOperation::increment("/visits", 1i64)]);

            let err = fault_container
                .patch_item(&pk, &item_id, patch, Some(patch_options))
                .await
                .expect_err("PATCH should fail after exhausting max_attempts");
            assert_eq!(
                err.status().status_code(),
                StatusCode::PreconditionFailed,
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
