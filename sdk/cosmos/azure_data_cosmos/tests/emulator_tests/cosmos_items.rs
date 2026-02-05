// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![cfg(feature = "key_auth")]

// Use the shared test framework declared in `tests/emulator/mod.rs`.
use super::framework;

use azure_core::http::{Etag, StatusCode};
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::models::{ContainerProperties, CosmosResponse};
use azure_data_cosmos::{models::PatchDocument, ItemOptions, PartitionKey};
use framework::TestClient;
use framework::TestRunContext;
use framework::{get_effective_hub_endpoint, get_global_endpoint};
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

/// Helper function to assert common response properties.
/// Verifies status code, that request charge is present and positive, endpoint is correct,
/// and that session token and activity ID are present.
fn assert_response<T>(
    response: &CosmosResponse<T>,
    expected_status: StatusCode,
    expected_endpoint: &str,
    read_operation: bool,
) {
    assert_eq!(response.status(), expected_status, "unexpected status code");
    let request_charge = response.request_charge();
    assert!(
        request_charge.is_some(),
        "expected request charge to be present"
    );
    assert!(
        request_charge.unwrap() > 0.0,
        "expected request charge to be positive"
    );
    if read_operation {
        // ETag is only returned on read operations
        let etag = response.etag();
        assert!(etag.is_some(), "expected etag to be present");
        assert!(etag.unwrap() != "", "expected etag to be non-empty");
    }

    assert_eq!(
        response
            .request()
            .clone()
            .into_raw_request()
            .url()
            .host_str()
            .unwrap(),
        expected_endpoint,
        "unexpected endpoint"
    );
    assert!(
        response.session_token().is_some(),
        "expected session token to be present"
    );
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
            assert_response(
                &response,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );
            let body = response.into_body().into_string()?;
            assert_eq!("", body);

            // Try to read the item
            let read_response = run_context
                .read_item::<TestItem>(&container_client, &pk, &item_id, None)
                .await?;
            assert_response(&read_response, StatusCode::Ok, &get_global_endpoint(), true);
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
            let body = response.into_body().into_string()?;
            assert_eq!("", body);

            // Update again, but this time ask for the response
            item.value = 12;
            item.nested.nested_value = "UpdatedAgain".into();
            let response = container_client
                .replace_item(
                    &pk,
                    &item_id,
                    &item,
                    Some(ItemOptions {
                        enable_content_response_on_write: true,
                        ..Default::default()
                    }),
                )
                .await?;
            assert_response(
                &response,
                StatusCode::Ok,
                &get_effective_hub_endpoint(),
                false,
            );
            let updated_item: TestItem = response.into_body().json()?;
            assert_eq!(item, updated_item);

            // Delete the item
            let response = container_client.delete_item(&pk, &item_id, None).await?;
            assert_response(
                &response,
                StatusCode::NoContent,
                &get_effective_hub_endpoint(),
                false,
            );
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

            let upsert_response = container_client.upsert_item(&pk, &item, None).await?;
            assert_response(
                &upsert_response,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );

            let read_response = run_context
                .read_item::<TestItem>(&container_client, &pk, &item_id, None)
                .await?;
            assert_response(&read_response, StatusCode::Ok, &get_global_endpoint(), true);
            let read_item: TestItem = read_response.into_model()?;
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

            let create_response = container_client.create_item(&pk, &item, None).await?;
            assert_response(
                &create_response,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );

            item.value = 24;
            item.nested.nested_value = "Updated".into();

            let upsert_response = container_client
                .upsert_item(
                    &pk,
                    &item,
                    Some(ItemOptions {
                        enable_content_response_on_write: true,
                        ..Default::default()
                    }),
                )
                .await?;
            assert_response(
                &upsert_response,
                StatusCode::Ok,
                &get_effective_hub_endpoint(),
                false,
            );
            let updated_item: TestItem = upsert_response.into_body().json()?;
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

            let create_response = container_client.create_item(&pk, &item, None).await?;
            assert_response(
                &create_response,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );

            let patch = PatchDocument::default()
                .with_replace("/nested/nested_value", "Patched")?
                .with_increment("/value", 10)?;
            let patch_response = container_client
                .patch_item(&pk, &item_id, patch, None)
                .await?;
            assert_response(
                &patch_response,
                StatusCode::Ok,
                &get_effective_hub_endpoint(),
                false,
            );

            let read_response = run_context
                .read_item::<TestItem>(&container_client, &pk, &item_id, None)
                .await?;
            assert_response(&read_response, StatusCode::Ok, &get_global_endpoint(), true);
            let patched_item: TestItem = read_response.into_model()?;
            assert_eq!("Patched", patched_item.nested.nested_value);
            assert_eq!(52, patched_item.value);

            let patch = PatchDocument::default().with_replace("/bool_value", false)?;
            let patch_response = container_client
                .patch_item(
                    &pk,
                    &item_id,
                    patch,
                    Some(ItemOptions {
                        enable_content_response_on_write: true,
                        ..Default::default()
                    }),
                )
                .await?;
            assert_response(
                &patch_response,
                StatusCode::Ok,
                &get_effective_hub_endpoint(),
                false,
            );
            let response_item: TestItem = patch_response.into_body().json()?;
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

            let create_response = container_client
                .create_item(PartitionKey::NULL, &item, None)
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
                .upsert_item(PartitionKey::NULL, &item, None)
                .await?;
            assert_response(
                &upsert_response,
                StatusCode::Ok,
                &get_effective_hub_endpoint(),
                false,
            );

            let read_response = run_context
                .read_item::<TestItem>(&container_client, PartitionKey::NULL, &item_id, None)
                .await?;
            assert_response(&read_response, StatusCode::Ok, &get_global_endpoint(), true);
            let read_item: TestItem = read_response.into_model()?;
            assert_eq!(item, read_item);

            let patch_response = container_client
                .patch_item(
                    PartitionKey::NULL,
                    &item_id,
                    PatchDocument::default().with_set("/value", 10)?,
                    None,
                )
                .await?;
            assert_response(
                &patch_response,
                StatusCode::Ok,
                &get_effective_hub_endpoint(),
                false,
            );

            let read_response = run_context
                .read_item::<TestItem>(&container_client, PartitionKey::NULL, &item_id, None)
                .await?;
            assert_response(&read_response, StatusCode::Ok, &get_global_endpoint(), true);
            let read_item: TestItem = read_response.into_model()?;
            assert_eq!(10, read_item.value);

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
            assert_response(
                &response,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );

            //Store Etag from response
            let etag: Etag = response
                .headers()
                .get_str(&azure_core::http::headers::ETAG)
                .expect("expected the etag to be returned")
                .into();

            //Replace item with correct Etag
            item.value = 24;
            item.nested.nested_value = "Updated".into();

            let replace_response = container_client
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
            assert_response(
                &response,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );

            //Store Etag from response
            let etag: Etag = response
                .headers()
                .get_str(&azure_core::http::headers::ETAG)
                .expect("expected the etag to be returned")
                .into();

            //Upsert item with correct Etag
            item.value = 24;
            item.nested.nested_value = "Updated".into();

            let upsert_response = container_client
                .upsert_item(
                    &pk,
                    &item,
                    Some(ItemOptions {
                        if_match_etag: Some(etag),
                        ..Default::default()
                    }),
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
            assert_response(
                &response,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );

            //Store Etag from response
            let etag: Etag = response
                .headers()
                .get_str(&azure_core::http::headers::ETAG)
                .expect("expected the etag to be returned")
                .into();

            //Delete item with correct Etag
            let delete_response = container_client
                .delete_item(
                    &pk,
                    &item_id,
                    Some(ItemOptions {
                        if_match_etag: Some(etag),
                        ..Default::default()
                    }),
                )
                .await?;
            assert_response(
                &delete_response,
                StatusCode::NoContent,
                &get_effective_hub_endpoint(),
                false,
            );

            //Add item again for second delete test
            let create_response = container_client.create_item(&pk, &item, None).await?;
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
            assert_response(
                &response,
                StatusCode::Created,
                &get_effective_hub_endpoint(),
                false,
            );

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

            let patch_response = container_client
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
            assert_response(
                &patch_response,
                StatusCode::Ok,
                &get_effective_hub_endpoint(),
                false,
            );

            let read_response = run_context
                .read_item::<TestItem>(&container_client, &pk, &item_id, None)
                .await?;
            assert_response(&read_response, StatusCode::Ok, &get_global_endpoint(), true);
            let patched_item: TestItem = read_response.into_model()?;

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
