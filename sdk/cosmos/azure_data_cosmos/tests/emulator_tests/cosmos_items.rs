// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Use the shared test framework declared in `tests/emulator/mod.rs`.
use super::framework;

use azure_core::{
    http::{Etag, StatusCode},
    Uuid,
};
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::models::ItemResponse;
use azure_data_cosmos::models::{ContainerProperties, PartitionKeyVersion};
use azure_data_cosmos::options::{
    ContentResponseOnWrite, ItemWriteOptions, OperationOptions, Precondition,
};
use azure_data_cosmos::PartitionKey;
use azure_data_cosmos_driver::{
    models::{AccountReference as DriverAccountReference, CosmosOperation, DatabaseReference},
    options::{
        ConnectionPoolOptions, OperationOptions as DriverOperationOptions,
        ServerCertificateValidation,
    },
    CosmosDriverRuntime, DriverOptions,
};
use framework::get_effective_hub_endpoint;
use framework::TestRunContext;
use framework::{TestClient, TestOptions};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use std::{borrow::Cow, error::Error};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct NestedItem {
    nested_value: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct TestItem {
    id: Cow<'static, str>,
    partition_key: Option<Cow<'static, str>>,
    value: usize,
    nested: NestedItem,
    bool_value: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Utf8PartitionKeyItem {
    id: Cow<'static, str>,
    partition_key: Cow<'static, str>,
    message: Cow<'static, str>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct InvalidUtf8BodyItem {
    id: Cow<'static, str>,
    partition_key: Cow<'static, str>,
    raw_payload: ByteBuf,
}

/// Helper function to assert common response properties.
/// Verifies status code, that request charge is present and positive, endpoint is correct,
/// and that session token, activity ID, and server duration are present.
fn assert_response(
    response: &ItemResponse,
    expected_status: StatusCode,
    _expected_endpoint: &str,
    read_operation: bool,
) {
    assert_eq!(response.status(), expected_status, "unexpected status code");
    let request_charge = response.headers().request_charge();
    assert!(
        request_charge.is_some(),
        "expected request charge to be present"
    );
    assert!(
        request_charge.unwrap().value() > 0.0,
        "expected request charge to be positive"
    );
    if read_operation {
        // ETag is only returned on read operations
        let etag = response.headers().etag();
        assert!(etag.is_some(), "expected etag to be present");
        assert!(
            !etag.unwrap().to_string().is_empty(),
            "expected etag to be non-empty"
        );
    }

    assert!(
        response.headers().session_token().is_some(),
        "expected session token to be present"
    );
    let diagnostics = response.diagnostics();
    assert!(
        !diagnostics.activity_id().as_str().is_empty(),
        "expected activity ID to be non-empty"
    );
    // The driver tracks at least one request per operation and finalizes its
    // status on completion. Validate the richer DiagnosticsContext fields.
    assert!(
        diagnostics.request_count() >= 1,
        "expected at least one request to be tracked"
    );
    let op_status = diagnostics
        .status()
        .expect("operation status should be set on completed diagnostics");
    assert_eq!(
        op_status.status_code(),
        expected_status,
        "operation-level diagnostics status should match HTTP response status"
    );
    assert!(
        f64::from(diagnostics.total_request_charge()) > 0.0,
        "expected positive total request charge in diagnostics"
    );
    // Server duration is returned by the Cosmos DB service on all operations
    let requests = diagnostics.requests();
    let server_duration = requests
        .iter()
        .filter_map(|r| r.server_duration_ms())
        .next();
    assert!(
        server_duration.is_some(),
        "expected at least one tracked request to report server_duration_ms"
    );
    assert!(
        server_duration.unwrap() >= 0.0,
        "expected server_duration_ms to be non-negative"
    );
}

async fn create_container(
    run_context: &TestRunContext,
) -> azure_data_cosmos::Result<ContainerClient> {
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

/// Like [`create_container`] but provisions a legacy **partition key version
/// 1** container by intentionally omitting `version`; the service interprets
/// a version-less create payload as V1. This exercises the classic-gateway
/// point-operation path for legacy V1 containers.
async fn create_v1_container(
    run_context: &TestRunContext,
) -> Result<ContainerClient, Box<dyn Error>> {
    let db_client = run_context.create_db().await?;
    let container_id = format!("Container-V1-{}", Uuid::new_v4());
    let connection_string = framework::resolve_connection_string()
        .ok_or("Cosmos connection string is not configured")?;
    let account = DriverAccountReference::with_master_key(
        connection_string.account_endpoint().parse::<url::Url>()?,
        connection_string.account_key().clone(),
    );
    let runtime = CosmosDriverRuntime::builder()
        .with_connection_pool(
            ConnectionPoolOptions::builder()
                .with_server_certificate_validation(
                    ServerCertificateValidation::RequiredUnlessEmulator,
                )
                .build()?,
        )
        .build()
        .await?;
    let driver = runtime
        .create_driver(DriverOptions::builder(account.clone()).build())
        .await?;
    let database = DatabaseReference::from_name(account, db_client.id().to_string());
    let body = format!(
        r#"{{"id":"{container_id}","partitionKey":{{"paths":["/partition_key"],"kind":"Hash"}}}}"#
    );
    let response = driver
        .execute_singleton_operation(
            CosmosOperation::create_container(database).with_body(body.into_bytes()),
            DriverOperationOptions::default(),
        )
        .await?;
    if !response.status().is_success() {
        return Err(format!(
            "failed to create version-less V1 container: {}",
            response.status()
        )
        .into());
    }
    let container_client = db_client.container_client(&container_id).await?;

    let body = container_client.read(None).await?.into_body().single()?;
    let raw: serde_json::Value = serde_json::from_slice(&body)?;
    assert!(
        raw["partitionKey"].get("version").is_none(),
        "the service must omit partitionKey.version when reading a V1 container"
    );
    let properties: ContainerProperties = serde_json::from_slice(&body)?;
    assert_eq!(
        properties.partition_key.version(),
        PartitionKeyVersion::V1,
        "a version-less service response must deserialize as V1"
    );

    Ok(container_client)
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: behavioral divergence"
)]
pub async fn item_crud() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();

            // Create an item with @ in both ID and partition key
            let mut item = TestItem {
                id: format!("Item@1-{}", unique_id).into(),
                partition_key: Some(format!("Partition@1-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };

            let pk = format!("Partition@1-{}", unique_id);
            let item_id = format!("Item@1-{}", unique_id);

            let response = container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;
            assert_response(
                &response,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );
            assert!(response.into_body().is_empty());

            // Try to read the item
            let read_response = run_context
                .read_item(&container_client, &pk, &item_id, None)
                .await?;
            assert_response(
                &read_response,
                StatusCode::Ok,
                &get_effective_hub_endpoint(),
                true,
            );
            let read_item: TestItem = read_response.into_model()?;
            assert_eq!(item, read_item);

            // Replace the item
            item.value = 24;
            item.nested.nested_value = "Updated".into();

            let response = container_client
                .replace_item(&pk, &item_id, &item, None)
                .await?;
            assert_response(
                &response,
                StatusCode::Ok,
                &get_effective_hub_endpoint(),
                false,
            );
            assert!(response.into_body().is_empty());

            // Update again, but this time ask for the response
            item.value = 12;
            item.nested.nested_value = "UpdatedAgain".into();
            let response = container_client
                .replace_item(&pk, &item_id, &item, {
                    let mut operation = OperationOptions::default();
                    operation.content_response_on_write = Some(ContentResponseOnWrite::Enabled);
                    Some(ItemWriteOptions::default().with_operation_options(operation))
                })
                .await?;
            assert_response(
                &response,
                StatusCode::Ok,
                &get_effective_hub_endpoint(),
                false,
            );
            let updated_item: TestItem = response.into_body().into_single()?;
            assert_eq!(item, updated_item);

            // Delete the item
            let response = container_client.delete_item(&pk, &item_id, None).await?;
            assert_response(
                &response,
                StatusCode::NoContent,
                &get_effective_hub_endpoint(),
                false,
            );
            assert!(response.into_body().is_empty());

            // Try to read the item again, expecting a 404
            // loop with backoff to avoid test flakes due to eventual consistency
            loop {
                match container_client.read_item(&pk, &item_id, None).await {
                    Ok(_) => {
                        println!("expected a 404 error when reading the deleted item, retrying...");
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    }
                    Err(err) => {
                        assert_eq!(
                            azure_core::http::StatusCode::NotFound,
                            err.status().status_code()
                        );
                        break;
                    }
                }
            }

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// Point CRUD (create → read → replace → delete) against a legacy **partition
/// key version 1** container over the classic gateway. Companion to the
/// Gateway 2.0 V1 test: classic gateway hashes the partition key server-side,
/// so it was never affected by the client-side EPK version-default bug, but we
/// pin the V1 point-op path here so both transports stay covered.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: behavioral divergence"
)]
pub async fn v1_container_item_crud() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_v1_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();

            let mut item = TestItem {
                id: format!("v1-item-{unique_id}").into(),
                partition_key: Some(format!("v1-pk-{unique_id}").into()),
                value: 1,
                nested: NestedItem {
                    nested_value: "initial".into(),
                },
                bool_value: true,
            };
            let pk = format!("v1-pk-{unique_id}");
            let item_id = format!("v1-item-{unique_id}");

            let create_resp = container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;
            assert_response(
                &create_resp,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );

            let read_resp = container_client.read_item(&pk, &item_id, None).await?;
            assert_response(
                &read_resp,
                StatusCode::Ok,
                &get_effective_hub_endpoint(),
                true,
            );
            let read_item: TestItem = read_resp.into_model()?;
            assert_eq!(item, read_item, "V1 container read must round-trip");

            item.value = 2;
            item.nested.nested_value = "updated".into();
            let replace_resp = container_client
                .replace_item(&pk, &item_id, &item, None)
                .await?;
            assert_response(
                &replace_resp,
                StatusCode::Ok,
                &get_effective_hub_endpoint(),
                false,
            );

            let reread: TestItem = container_client
                .read_item(&pk, &item_id, None)
                .await?
                .into_model()?;
            assert_eq!(item, reread, "replace must be reflected on re-read");

            let delete_resp = container_client.delete_item(&pk, &item_id, None).await?;
            assert_response(
                &delete_resp,
                StatusCode::NoContent,
                &get_effective_hub_endpoint(),
                false,
            );

            const MAX_DELETE_POLLS: u32 = 20;
            for attempt in 0..MAX_DELETE_POLLS {
                match container_client.read_item(&pk, &item_id, None).await {
                    Ok(_) if attempt + 1 < MAX_DELETE_POLLS => {
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    }
                    Ok(_) => {
                        return Err(format!(
                            "V1 item remained readable after {MAX_DELETE_POLLS} delete polls"
                        )
                        .into());
                    }
                    Err(err) if err.status().status_code() == StatusCode::NotFound => return Ok(()),
                    Err(err) => {
                        return Err(format!(
                            "expected NotFound after deleting V1 item, got {}",
                            err.status()
                        )
                        .into());
                    }
                }
            }
            unreachable!("delete polling loop always returns")
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: behavioral divergence"
)]
pub async fn item_read_system_properties() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();

            // Create an item
            let item = TestItem {
                id: format!("Item1-{}", unique_id).into(),
                partition_key: Some(format!("Partition1-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };

            let pk = format!("Partition1-{}", unique_id);
            let item_id = format!("Item1-{}", unique_id);

            let create_response = container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;
            assert_response(
                &create_response,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );

            let read_response = run_context
                .read_item(&container_client, &pk, &item_id, None)
                .await?;
            assert_response(
                &read_response,
                StatusCode::Ok,
                &get_effective_hub_endpoint(),
                true,
            );
            let read_item: serde_json::Value = read_response.into_model()?;
            assert!(
                read_item.get("_rid").is_some(),
                "expected _rid to be present"
            );

            assert!(
                read_item.get("_etag").is_some(),
                "expected _etag to be present"
            );

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: behavioral divergence"
)]
pub async fn item_upsert_new() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();

            let item = TestItem {
                id: format!("Item1-{}", unique_id).into(),
                partition_key: Some(format!("Partition1-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };

            let pk = format!("Partition1-{}", unique_id);
            let item_id = format!("Item1-{}", unique_id);

            let upsert_response = container_client
                .upsert_item(&pk, &item_id, &item, None)
                .await?;
            assert_response(
                &upsert_response,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );

            let read_response = run_context
                .read_item(&container_client, &pk, &item_id, None)
                .await?;
            assert_response(
                &read_response,
                StatusCode::Ok,
                &get_effective_hub_endpoint(),
                true,
            );
            let read_item: TestItem = read_response.into_model()?;
            assert_eq!(item, read_item);

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: behavioral divergence"
)]
pub async fn item_upsert_existing() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();

            let mut item = TestItem {
                id: format!("Item1-{}", unique_id).into(),
                partition_key: Some(format!("Partition1-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };

            let pk = format!("Partition1-{}", unique_id);
            let item_id = format!("Item1-{}", unique_id);

            let create_response = container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;
            assert_response(
                &create_response,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );

            item.value = 24;
            item.nested.nested_value = "Updated".into();

            let upsert_response = container_client
                .upsert_item(&pk, &item_id, &item, {
                    let mut operation = OperationOptions::default();
                    operation.content_response_on_write = Some(ContentResponseOnWrite::Enabled);
                    Some(ItemWriteOptions::default().with_operation_options(operation))
                })
                .await?;
            assert_response(
                &upsert_response,
                StatusCode::Ok,
                &get_effective_hub_endpoint(),
                false,
            );
            let updated_item: TestItem = upsert_response.into_body().into_single()?;
            assert_eq!(item, updated_item);

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: behavioral divergence"
)]
pub async fn item_null_partition_key() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();

            let mut item = TestItem {
                id: format!("Item1-{}", unique_id).into(),
                partition_key: None,
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };

            let item_id = format!("Item1-{}", unique_id);

            let create_response = container_client
                .create_item(PartitionKey::NULL, &item_id, &item, None)
                .await?;
            assert_response(
                &create_response,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );

            item.value = 24;
            item.nested.nested_value = "Updated".into();

            let upsert_response = container_client
                .upsert_item(PartitionKey::NULL, &item_id, &item, None)
                .await?;
            assert_response(
                &upsert_response,
                StatusCode::Ok,
                &get_effective_hub_endpoint(),
                false,
            );

            let read_response = run_context
                .read_item(&container_client, PartitionKey::NULL, &item_id, None)
                .await?;
            assert_response(
                &read_response,
                StatusCode::Ok,
                &get_effective_hub_endpoint(),
                true,
            );
            let read_item: TestItem = read_response.into_model()?;
            assert_eq!(item, read_item);

            let delete_response = container_client
                .delete_item(PartitionKey::NULL, &item_id, None)
                .await?;
            assert_response(
                &delete_response,
                StatusCode::NoContent,
                &get_effective_hub_endpoint(),
                false,
            );

            // loop with backoff to avoid test flakes due to eventual consistency
            loop {
                match container_client
                    .read_item(PartitionKey::NULL, &item_id, None)
                    .await
                {
                    Ok(_) => {
                        println!("expected a 404 error when reading the deleted item, retrying...");
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    }
                    Err(err) => {
                        assert_eq!(
                            azure_core::http::StatusCode::NotFound,
                            err.status().status_code()
                        );
                        break;
                    }
                }
            }

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: behavioral divergence"
)]
pub async fn item_replace_if_match_etag() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();

            //Create an item
            let mut item = TestItem {
                id: format!("Item1-{}", unique_id).into(),
                partition_key: Some(format!("Partition1-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };

            let pk = format!("Partition1-{}", unique_id);
            let item_id = format!("Item1-{}", unique_id);

            let response = container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;
            assert_response(
                &response,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );

            //Store Etag from response
            let etag: Etag = response
                .headers()
                .etag()
                .expect("expected the etag to be returned")
                .clone();

            //Replace item with correct Etag
            item.value = 24;
            item.nested.nested_value = "Updated".into();

            let replace_response = container_client
                .replace_item(
                    &pk,
                    &item_id,
                    &item,
                    Some(
                        ItemWriteOptions::default()
                            .with_precondition(Precondition::IfMatch(Etag::from(etag.to_string()))),
                    ),
                )
                .await?;
            assert_response(
                &replace_response,
                StatusCode::Ok,
                &get_effective_hub_endpoint(),
                false,
            );

            //Replace item with incorrect Etag
            item.value = 52;
            item.nested.nested_value = "UpdatedAgain".into();

            let response = container_client
                .replace_item(
                    &pk,
                    &item_id,
                    &item,
                    Some(
                        ItemWriteOptions::default()
                            .with_precondition(Precondition::IfMatch(Etag::from("incorrectEtag"))),
                    ),
                )
                .await;

            assert_eq!(
                azure_core::http::StatusCode::PreconditionFailed,
                response
                    .expect_err("expected the server to return an error")
                    .status()
                    .status_code()
            );

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: behavioral divergence"
)]
pub async fn item_upsert_if_match_etag() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();

            //Create an item
            let mut item = TestItem {
                id: format!("Item1-{}", unique_id).into(),
                partition_key: Some(format!("Partition1-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };

            let pk = format!("Partition1-{}", unique_id);
            let item_id = format!("Item1-{}", unique_id);

            let response = container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;
            assert_response(
                &response,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );

            //Store Etag from response
            let etag: Etag = response
                .headers()
                .etag()
                .expect("expected the etag to be returned")
                .clone();

            //Upsert item with correct Etag
            item.value = 24;
            item.nested.nested_value = "Updated".into();

            let upsert_response = container_client
                .upsert_item(
                    &pk,
                    &item_id,
                    &item,
                    Some(
                        ItemWriteOptions::default()
                            .with_precondition(Precondition::IfMatch(Etag::from(etag.to_string()))),
                    ),
                )
                .await?;
            assert_response(
                &upsert_response,
                StatusCode::Ok,
                &get_effective_hub_endpoint(),
                false,
            );

            //Upsert item with incorrect Etag
            item.value = 52;
            item.nested.nested_value = "UpdatedAgain".into();

            let response = container_client
                .upsert_item(
                    &pk,
                    &item_id,
                    &item,
                    Some(
                        ItemWriteOptions::default()
                            .with_precondition(Precondition::IfMatch(Etag::from("incorrectEtag"))),
                    ),
                )
                .await;

            assert_eq!(
                azure_core::http::StatusCode::PreconditionFailed,
                response
                    .expect_err("expected the server to return an error")
                    .status()
                    .status_code()
            );

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: behavioral divergence"
)]
pub async fn item_delete_if_match_etag() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();

            //Create an item
            let item = TestItem {
                id: format!("Item1-{}", unique_id).into(),
                partition_key: Some(format!("Partition1-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };

            let pk = format!("Partition1-{}", unique_id);
            let item_id = format!("Item1-{}", unique_id);

            let response = container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;
            assert_response(
                &response,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );

            //Store Etag from response
            let etag: Etag = response
                .headers()
                .etag()
                .expect("expected the etag to be returned")
                .clone();

            //Delete item with correct Etag
            let delete_response = container_client
                .delete_item(
                    &pk,
                    &item_id,
                    Some(
                        ItemWriteOptions::default()
                            .with_precondition(Precondition::IfMatch(Etag::from(etag.to_string()))),
                    ),
                )
                .await?;
            assert_response(
                &delete_response,
                StatusCode::NoContent,
                &get_effective_hub_endpoint(),
                false,
            );

            //Add item again for second delete test
            let create_response = container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;
            assert_response(
                &create_response,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );

            //Delete item with incorrect Etag
            let response = container_client
                .delete_item(
                    &pk,
                    &item_id,
                    Some(
                        ItemWriteOptions::default()
                            .with_precondition(Precondition::IfMatch(Etag::from("incorrectEtag"))),
                    ),
                )
                .await;

            assert_eq!(
                azure_core::http::StatusCode::PreconditionFailed,
                response
                    .expect_err("expected the server to return an error")
                    .status()
                    .status_code()
            );

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// An item type without a partition key property, for testing undefined partition keys.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct UndefinedPkItem {
    id: Cow<'static, str>,
    value: usize,
}

/// An item type with an explicit partition key property.
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct ExplicitPkItem {
    id: Cow<'static, str>,
    partition_key: Cow<'static, str>,
    value: usize,
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: behavioral divergence"
)]
pub async fn item_undefined_partition_key() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();

            // Create an item WITHOUT the partition_key property (undefined PK).
            let item_no_pk = UndefinedPkItem {
                id: format!("Item-NoPK-{}", unique_id).into(),
                value: 100,
            };
            let item_no_pk_id = format!("Item-NoPK-{}", unique_id);

            let response = container_client
                .create_item(PartitionKey::UNDEFINED, &item_no_pk_id, &item_no_pk, None)
                .await?;
            assert_response(
                &response,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );

            // Create an item WITH a null partition_key property.
            let item_null_pk = TestItem {
                id: format!("Item-NullPK-{}", unique_id).into(),
                partition_key: None,
                value: 200,
                nested: NestedItem {
                    nested_value: "NullPK".into(),
                },
                bool_value: false,
            };
            let item_null_pk_id = format!("Item-NullPK-{}", unique_id);

            let response = container_client
                .create_item(PartitionKey::NULL, &item_null_pk_id, &item_null_pk, None)
                .await?;
            assert_response(
                &response,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );

            // Create an item WITH an explicit partition_key value.
            let pk_value = format!("PK-{}", unique_id);
            let item_with_pk = ExplicitPkItem {
                id: format!("Item-WithPK-{}", unique_id).into(),
                partition_key: pk_value.clone().into(),
                value: 300,
            };
            let item_with_pk_id = format!("Item-WithPK-{}", unique_id);

            let response = container_client
                .create_item(&pk_value, &item_with_pk_id, &item_with_pk, None)
                .await?;
            assert_response(
                &response,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );

            // Read the undefined-PK item using UNDEFINED - should succeed.
            let read_response = run_context
                .read_item(
                    &container_client,
                    PartitionKey::UNDEFINED,
                    &item_no_pk_id,
                    None,
                )
                .await?;
            assert_response(
                &read_response,
                StatusCode::Ok,
                &get_effective_hub_endpoint(),
                true,
            );
            let read_item: UndefinedPkItem = read_response.into_model()?;
            assert_eq!(item_no_pk, read_item);

            // Reading the undefined-PK item with NULL should fail (wrong partition).
            let result = container_client
                .read_item(PartitionKey::NULL, &item_no_pk_id, None)
                .await;
            assert_eq!(
                azure_core::http::StatusCode::NotFound,
                result
                    .expect_err("expected a 404 for undefined-PK item read with NULL")
                    .status()
                    .status_code()
            );

            // Read the null-PK item using NULL - should succeed.
            let read_response = run_context
                .read_item(
                    &container_client,
                    PartitionKey::NULL,
                    &item_null_pk_id,
                    None,
                )
                .await?;
            assert_response(
                &read_response,
                StatusCode::Ok,
                &get_effective_hub_endpoint(),
                true,
            );
            let read_item: TestItem = read_response.into_model()?;
            assert_eq!(item_null_pk, read_item);

            // Reading the null-PK item with UNDEFINED should fail (wrong partition).
            let result = container_client
                .read_item(PartitionKey::UNDEFINED, &item_null_pk_id, None)
                .await;
            assert_eq!(
                azure_core::http::StatusCode::NotFound,
                result
                    .expect_err("expected a 404 for null-PK item read with UNDEFINED")
                    .status()
                    .status_code()
            );

            // Delete the undefined-PK item using UNDEFINED.
            let response = container_client
                .delete_item(PartitionKey::UNDEFINED, &item_no_pk_id, None)
                .await?;
            assert_response(
                &response,
                StatusCode::NoContent,
                &get_effective_hub_endpoint(),
                false,
            );

            // Suppress unused variable warning
            let _ = item_with_pk_id;

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// Validates that `create_item` (driver-routed) returns 409 Conflict when the
/// item already exists. This exercises the driver's error-path bridging.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: behavioral divergence"
)]
pub async fn create_item_duplicate_returns_conflict() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();

            let item = TestItem {
                id: format!("dup-{}", unique_id).into(),
                partition_key: Some(format!("pk-{}", unique_id).into()),
                value: 1,
                nested: NestedItem {
                    nested_value: "first".into(),
                },
                bool_value: true,
            };
            let pk = format!("pk-{}", unique_id);
            let item_id = format!("dup-{}", unique_id);

            // First create should succeed.
            let response = container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;
            assert_response(
                &response,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );

            // Second create of the same item should fail with 409 Conflict.
            let result = container_client
                .create_item(&pk, &item_id, &item, None)
                .await;
            assert_eq!(
                StatusCode::Conflict,
                result
                    .expect_err("expected conflict on duplicate create")
                    .status()
                    .status_code(),
            );

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// Validates that `create_item` (driver-routed) returns the created item body
/// when `ContentResponseOnWrite::Enabled` is set.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: behavioral divergence"
)]
pub async fn create_item_with_content_response() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();

            let item = TestItem {
                id: format!("cr-{}", unique_id).into(),
                partition_key: Some(format!("pk-{}", unique_id).into()),
                value: 99,
                nested: NestedItem {
                    nested_value: "content-response".into(),
                },
                bool_value: false,
            };
            let pk = format!("pk-{}", unique_id);
            let item_id = format!("cr-{}", unique_id);

            let mut operation = OperationOptions::default();
            operation.content_response_on_write = Some(ContentResponseOnWrite::Enabled);
            let options = ItemWriteOptions::default().with_operation_options(operation);

            let response = container_client
                .create_item(&pk, &item_id, &item, Some(options))
                .await?;
            assert_response(
                &response,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );

            // Deserialize the body and verify it matches the original item.
            let created: TestItem = response.into_body().into_single()?;
            assert_eq!(item, created);

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

/// Validates that driver-routed `create_item` returns all expected response
/// metadata: session token, activity ID, request charge, and server duration.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: behavioral divergence"
)]
pub async fn create_item_response_metadata() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();

            let item = TestItem {
                id: format!("meta-{}", unique_id).into(),
                partition_key: Some(format!("pk-{}", unique_id).into()),
                value: 7,
                nested: NestedItem {
                    nested_value: "metadata-check".into(),
                },
                bool_value: true,
            };
            let pk = format!("pk-{}", unique_id);
            let item_id = format!("meta-{}", unique_id);

            let response = container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;
            assert_eq!(response.status(), StatusCode::Created);

            // Session token must be present for session consistency.
            assert!(
                response.headers().session_token().is_some(),
                "expected session token on create_item response"
            );

            // Diagnostics from the driver pipeline must surface a populated
            // activity ID and at least one tracked request with timing data.
            let diagnostics = response.diagnostics();
            assert!(
                !diagnostics.activity_id().as_str().is_empty(),
                "activity ID must be non-empty"
            );
            assert!(
                diagnostics.request_count() >= 1,
                "expected at least one request to be tracked in diagnostics"
            );
            let op_status = diagnostics
                .status()
                .expect("operation status should be set on completed diagnostics");
            assert_eq!(
                op_status.status_code(),
                StatusCode::Created,
                "operation-level diagnostics status should match HTTP response status"
            );

            // Request charge must be positive.
            let charge = response.headers().request_charge();
            assert!(charge.is_some(), "expected request charge");
            assert!(
                charge.unwrap().value() > 0.0,
                "request charge must be positive"
            );
            assert!(
                f64::from(diagnostics.total_request_charge()) >= charge.unwrap().value(),
                "diagnostics total request charge should aggregate response request charge"
            );

            // Server duration must be present and non-negative on the tracked request.
            let requests = diagnostics.requests();
            let duration = requests
                .iter()
                .filter_map(|r| r.server_duration_ms())
                .next();
            assert!(duration.is_some(), "expected server_duration_ms");
            assert!(
                duration.unwrap() >= 0.0,
                "server_duration_ms must be non-negative"
            );

            // Response body should be empty when ContentResponseOnWrite is not enabled.
            assert!(response.into_body().is_empty());

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn item_partition_key_non_ascii_utf8() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();

            // cspell:disable-next-line
            let partition_key_value = format!("分区-κλειδί-مفتاح-🙂-{}", unique_id);
            let item_id = format!("utf8-pk-{}", unique_id);
            let item = Utf8PartitionKeyItem {
                id: item_id.clone().into(),
                partition_key: partition_key_value.clone().into(),
                message: "こんにちは Cosmos - UTF-8 ✅".into(),
            };

            let create_response = container_client
                .create_item(&partition_key_value, &item_id, &item, None)
                .await?;
            assert_response(
                &create_response,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );

            let read_response = run_context
                .read_item(&container_client, &partition_key_value, &item_id, None)
                .await?;
            assert_response(
                &read_response,
                StatusCode::Ok,
                &get_effective_hub_endpoint(),
                true,
            );
            let read_item: Utf8PartitionKeyItem = read_response.into_model()?;
            assert_eq!(item, read_item);

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn item_body_invalid_utf8_bytes_roundtrip() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();

            let partition_key_value = format!("raw-bytes-{}", unique_id);
            let item_id = format!("invalid-utf8-body-{}", unique_id);

            // Include bytes that are not valid UTF-8 sequences.
            let invalid_utf8_bytes = vec![0xed, 0xa0, 0x80, 0xf0, 0x28, 0x8c, 0x28];
            assert!(
                std::str::from_utf8(&invalid_utf8_bytes).is_err(),
                "test fixture should contain invalid UTF-8"
            );

            let item = InvalidUtf8BodyItem {
                id: item_id.clone().into(),
                partition_key: partition_key_value.clone().into(),
                raw_payload: ByteBuf::from(invalid_utf8_bytes.clone()),
            };

            let create_response = container_client
                .create_item(&partition_key_value, &item_id, &item, None)
                .await?;
            assert_response(
                &create_response,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );

            let read_response = run_context
                .read_item(&container_client, &partition_key_value, &item_id, None)
                .await?;
            assert_response(
                &read_response,
                StatusCode::Ok,
                &get_effective_hub_endpoint(),
                true,
            );
            let read_item: InvalidUtf8BodyItem = read_response.into_model()?;
            assert_eq!(
                read_item.raw_payload,
                ByteBuf::from(invalid_utf8_bytes),
                "invalid UTF-8 bytes should round-trip in document body"
            );

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}
