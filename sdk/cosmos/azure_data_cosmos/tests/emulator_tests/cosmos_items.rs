// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![cfg(feature = "key_auth")]
#[cfg(feature = "fault_injection")]

// Use the shared test framework declared in `tests/emulator/mod.rs`.
use super::framework;

use azure_core::http::Etag;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::models::ContainerProperties;
use azure_data_cosmos::{models::PatchDocument, ItemOptions, PartitionKey};
use azure_data_cosmos::fault_injection::{
    FaultInjectionClientBuilder, FaultInjectionConditionBuilder, FaultInjectionRuleBuilder,
    FaultInjectionServerError, FaultInjectionServerErrorType,
};
use framework::TestClient;
use framework::TestRunContext;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, error::Error};
use uuid::Uuid;

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

async fn create_container(run_context: &TestRunContext) -> azure_core::Result<ContainerClient> {
    let db_client = run_context.create_db().await?;
    let container_id = format!("Container-{}", Uuid::new_v4());
    run_context
        .create_container(
            &db_client,
            ContainerProperties {
                id: container_id.clone().into(),
                partition_key: "/partition_key".into(),
                ..Default::default()
            },
            None,
        )
        .await?;
    let container_client = db_client.container_client(&container_id);

    Ok(container_client)
}

// TODO: add asserts on status code (and other headers/diagnostics) for all the tests

#[tokio::test]
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

            let response = container_client.create_item(&pk, &item, None).await?;
            let body = response.into_body().into_string()?;
            assert_eq!("", body);

            // Try to read the item
            let read_item: TestItem = run_context
                .read_item::<TestItem>(&container_client, &pk, &item_id)
                .await?;
            assert_eq!(item, read_item);

            // Replace the item
            item.value = 24;
            item.nested.nested_value = "Updated".into();

            let response = container_client
                .replace_item(&pk, &item_id, &item, None)
                .await?;
            let body = response.into_body().into_string()?;
            assert_eq!("", body);

            // Update again, but this time ask for the response
            item.value = 12;
            item.nested.nested_value = "UpdatedAgain".into();
            let updated_item: TestItem = container_client
                .replace_item(
                    &pk,
                    &item_id,
                    &item,
                    Some(ItemOptions {
                        enable_content_response_on_write: true,
                        ..Default::default()
                    }),
                )
                .await?
                .into_body()
                .json()?;
            assert_eq!(item, updated_item);

            // Delete the item
            let response = container_client.delete_item(&pk, &item_id, None).await?;
            let body = response.into_body().into_string()?;
            assert_eq!("", body);

            // Try to read the item again, expecting a 404
            // loop with backoff to avoid test flakes due to eventual consistency
            loop {
                match container_client
                    .read_item::<TestItem>(&pk, &item_id, None)
                    .await
                {
                    Ok(_) => {
                        println!("expected a 404 error when reading the deleted item, retrying...");
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    }
                    Err(err) => {
                        assert_eq!(
                            Some(azure_core::http::StatusCode::NotFound),
                            err.http_status()
                        );
                        break;
                    }
                }
            }

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
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

            container_client.create_item(&pk, &item, None).await?;

            let read_item: serde_json::Value = run_context
                .read_item::<serde_json::Value>(&container_client, &pk, &item_id)
                .await?;
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
        None,
    )
    .await
}

#[tokio::test]
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

            container_client.upsert_item(&pk, &item, None).await?;

            let read_item: TestItem = run_context
                .read_item::<TestItem>(&container_client, &pk, &item_id)
                .await?;
            assert_eq!(item, read_item);

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
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

            container_client.create_item(&pk, &item, None).await?;

            item.value = 24;
            item.nested.nested_value = "Updated".into();

            let updated_item: TestItem = container_client
                .upsert_item(
                    &pk,
                    &item,
                    Some(ItemOptions {
                        enable_content_response_on_write: true,
                        ..Default::default()
                    }),
                )
                .await?
                .into_body()
                .json()?;
            assert_eq!(item, updated_item);

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
pub async fn item_patch() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();

            let item = TestItem {
                id: format!("Item3-{}", unique_id).into(),
                partition_key: Some(format!("Partition1-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };

            let pk = format!("Partition1-{}", unique_id);
            let item_id = format!("Item3-{}", unique_id);

            container_client.create_item(&pk, &item, None).await?;

            let patch = PatchDocument::default()
                .with_replace("/nested/nested_value", "Patched")?
                .with_increment("/value", 10)?;
            container_client
                .patch_item(&pk, &item_id, patch, None)
                .await?;

            let patched_item: TestItem = run_context
                .read_item::<TestItem>(&container_client, &pk, &item_id)
                .await?;
            assert_eq!("Patched", patched_item.nested.nested_value);
            assert_eq!(52, patched_item.value);

            let patch = PatchDocument::default().with_replace("/bool_value", false)?;
            let response_item: TestItem = container_client
                .patch_item(
                    &pk,
                    &item_id,
                    patch,
                    Some(ItemOptions {
                        enable_content_response_on_write: true,
                        ..Default::default()
                    }),
                )
                .await?
                .into_body()
                .json()?;
            assert!(!response_item.bool_value);

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
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

            container_client
                .create_item(PartitionKey::NULL, &item, None)
                .await?;

            item.value = 24;
            item.nested.nested_value = "Updated".into();

            container_client
                .upsert_item(PartitionKey::NULL, &item, None)
                .await?;

            let read_item: TestItem = run_context
                .read_item::<TestItem>(&container_client, PartitionKey::NULL, &item_id)
                .await?;
            assert_eq!(item, read_item);

            container_client
                .patch_item(
                    PartitionKey::NULL,
                    &item_id,
                    PatchDocument::default().with_set("/value", 10)?,
                    None,
                )
                .await?;

            let read_item: TestItem = run_context
                .read_item::<TestItem>(&container_client, PartitionKey::NULL, &item_id)
                .await?;
            assert_eq!(10, read_item.value);

            container_client
                .delete_item(PartitionKey::NULL, &item_id, None)
                .await?;

            // loop with backoff to avoid test flakes due to eventual consistency
            loop {
                match container_client
                    .read_item::<()>(PartitionKey::NULL, &item_id, None)
                    .await
                {
                    Ok(_) => {
                        println!("expected a 404 error when reading the deleted item, retrying...");
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    }
                    Err(err) => {
                        assert_eq!(
                            Some(azure_core::http::StatusCode::NotFound),
                            err.http_status()
                        );
                        break;
                    }
                }
            }

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
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

            let response = container_client.create_item(&pk, &item, None).await?;

            //Store Etag from response
            let etag: Etag = response
                .headers()
                .get_str(&azure_core::http::headers::ETAG)
                .expect("expected the etag to be returned")
                .into();

            //Replace item with correct Etag
            item.value = 24;
            item.nested.nested_value = "Updated".into();

            container_client
                .replace_item(
                    &pk,
                    &item_id,
                    &item,
                    Some(ItemOptions {
                        if_match_etag: Some(etag),
                        ..Default::default()
                    }),
                )
                .await?;

            //Replace item with incorrect Etag
            item.value = 52;
            item.nested.nested_value = "UpdatedAgain".into();

            let response = container_client
                .replace_item(
                    &pk,
                    &item_id,
                    &item,
                    Some(ItemOptions {
                        if_match_etag: Some("incorrectEtag".into()),
                        ..Default::default()
                    }),
                )
                .await;

            assert_eq!(
                Some(azure_core::http::StatusCode::PreconditionFailed),
                response
                    .expect_err("expected the server to return an error")
                    .http_status()
            );

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
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

            let response = container_client.create_item(&pk, &item, None).await?;

            //Store Etag from response
            let etag: Etag = response
                .headers()
                .get_str(&azure_core::http::headers::ETAG)
                .expect("expected the etag to be returned")
                .into();

            //Upsert item with correct Etag
            item.value = 24;
            item.nested.nested_value = "Updated".into();

            container_client
                .upsert_item(
                    &pk,
                    &item,
                    Some(ItemOptions {
                        if_match_etag: Some(etag),
                        ..Default::default()
                    }),
                )
                .await?;

            //Upsert item with incorrect Etag
            item.value = 52;
            item.nested.nested_value = "UpdatedAgain".into();

            let response = container_client
                .upsert_item(
                    &pk,
                    &item,
                    Some(ItemOptions {
                        if_match_etag: Some("incorrectEtag".into()),
                        ..Default::default()
                    }),
                )
                .await;

            assert_eq!(
                Some(azure_core::http::StatusCode::PreconditionFailed),
                response
                    .expect_err("expected the server to return an error")
                    .http_status()
            );

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
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

            let response = container_client.create_item(&pk, &item, None).await?;

            //Store Etag from response
            let etag: Etag = response
                .headers()
                .get_str(&azure_core::http::headers::ETAG)
                .expect("expected the etag to be returned")
                .into();

            //Delete item with correct Etag
            container_client
                .delete_item(
                    &pk,
                    &item_id,
                    Some(ItemOptions {
                        if_match_etag: Some(etag),
                        ..Default::default()
                    }),
                )
                .await?;

            //Add item again for second delete test
            container_client.create_item(&pk, &item, None).await?;

            //Delete item with incorrect Etag
            let response = container_client
                .delete_item(
                    &pk,
                    &item_id,
                    Some(ItemOptions {
                        if_match_etag: Some("incorrectEtag".into()),
                        ..Default::default()
                    }),
                )
                .await;

            assert_eq!(
                Some(azure_core::http::StatusCode::PreconditionFailed),
                response
                    .expect_err("expected the server to return an error")
                    .http_status()
            );

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
pub async fn item_patch_if_match_etag() -> Result<(), Box<dyn Error>> {
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

            let response = container_client.create_item(&pk, &item, None).await?;

            //Store Etag from response
            let etag: Etag = response
                .headers()
                .get_str(&azure_core::http::headers::ETAG)
                .expect("expected the etag to be returned")
                .into();

            //Patch item with correct Etag
            let patch = PatchDocument::default()
                .with_replace("/nested/nested_value", "Patched")?
                .with_increment("/value", 10)?;

            container_client
                .patch_item(
                    &pk,
                    &item_id,
                    patch,
                    Some(ItemOptions {
                        if_match_etag: Some(etag),
                        ..Default::default()
                    }),
                )
                .await?;

            let patched_item: TestItem = container_client
                .read_item(&pk, &item_id, None)
                .await?
                .into_model()?;

            assert_eq!("Patched", patched_item.nested.nested_value);
            assert_eq!(52, patched_item.value);

            //Patch item with incorrect Etag
            let patch = PatchDocument::default()
                .with_replace("/nested/nested_value", "PatchedIncorrect")?
                .with_increment("/value", 15)?;

            let response = container_client
                .patch_item(
                    &pk,
                    &item_id,
                    patch,
                    Some(ItemOptions {
                        if_match_etag: Some("incorrectEtag".into()),
                        ..Default::default()
                    }),
                )
                .await;

            assert_eq!(
                Some(azure_core::http::StatusCode::PreconditionFailed),
                response
                    .expect_err("expected the server to return an error")
                    .http_status()
            );

            Ok(())
        },
        None,
    )
    .await
}

/// Test that verifies fault injection with repeated 503 Service Unavailable errors.
/// This test expects the operation to fail with a 503 error since no retries are configured
/// for this error type.
#[tokio::test]
pub async fn item_read_with_503_fault_injection() -> Result<(), Box<dyn Error>> {
    use azure_data_cosmos::CosmosClientOptions;
    use std::time::Duration;

    // Create a fault injection rule that always returns 503 Service Unavailable
    let server_error = FaultInjectionServerError::builder(FaultInjectionServerErrorType::ServiceUnavailable)
        .probability(1.0) // Always inject the fault
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(azure_data_cosmos::fault_injection::FaultOperationType::ReadItem)
        .build();

    let rule = FaultInjectionRuleBuilder::new("503-always", server_error)
        .with_condition(condition)
        .build();

    let mut fault_builder = FaultInjectionClientBuilder::new();
    fault_builder.with_rule(rule);

    // Inject the fault into client options
    let options = fault_builder.inject(CosmosClientOptions::default());

    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let unique_id = Uuid::new_v4().to_string();

            // Create an item (this should succeed since fault is only on reads)
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

            container_client.create_item(&pk, &item, None).await?;

            // Try to read the item - this should fail with 503 due to fault injection
            let result = container_client
                .read_item::<TestItem>(&pk, &item_id, None)
                .await;

            // Verify we got a 503 error
            let err = result.expect_err("expected the read to fail with 503");
            assert_eq!(
                Some(azure_core::http::StatusCode::ServiceUnavailable),
                err.http_status(),
                "expected 503 Service Unavailable, got {:?}",
                err.http_status()
            );

            Ok(())
        },
        Some(framework::TestOptions::new().with_client_options(options)),
    )
    .await
}

