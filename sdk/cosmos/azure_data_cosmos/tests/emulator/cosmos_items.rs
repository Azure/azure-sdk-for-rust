#![cfg(all(feature = "key_auth"))]

mod framework;

use azure_core::http::Etag;
use azure_data_cosmos::{
    clients::ContainerClient,
    models::{ContainerProperties, PatchDocument},
    ItemOptions, PartitionKey,
};
use framework::{TestClient, TestRunContext};
use serde::{Deserialize, Serialize};
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

async fn create_container(run_context: &TestRunContext) -> azure_core::Result<ContainerClient> {
    let db_client = run_context.create_db().await?;
    db_client
        .create_container(
            ContainerProperties {
                id: "Container".into(),
                partition_key: "/partition_key".into(),
                ..Default::default()
            },
            None,
        )
        .await?;
    let container_client = db_client.container_client("Container");

    Ok(container_client)
}

#[tokio::test]
pub async fn item_crud() -> Result<(), Box<dyn Error>> {
    TestClient::run(
        async |run_context| {
            let container_client = create_container(run_context).await?;

            // Create an item with @ in both ID and partition key
            let mut item = TestItem {
                id: "Item@1".into(),
                partition_key: Some("Partition@1".into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };

            let response = container_client
                .create_item("Partition@1", &item, None)
                .await?;
            let body = response.into_body().into_string()?;
            assert_eq!("", body);

            // Try to read the item
            let read_item: TestItem = container_client
                .read_item("Partition@1", "Item@1", None)
                .await?
                .into_model()?;
            assert_eq!(item, read_item);

            // Replace the item
            item.value = 24;
            item.nested.nested_value = "Updated".into();

            let response = container_client
                .replace_item("Partition@1", "Item@1", &item, None)
                .await?;
            let body = response.into_body().into_string()?;
            assert_eq!("", body);

            // Update again, but this time ask for the response
            item.value = 12;
            item.nested.nested_value = "UpdatedAgain".into();
            let updated_item: TestItem = container_client
                .replace_item(
                    "Partition@1",
                    "Item@1",
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
            let response = container_client
                .delete_item("Partition@1", "Item@1", None)
                .await?;
            let body = response.into_body().into_string()?;
            assert_eq!("", body);

            // Try to read the item again, expecting a 404
            let result = container_client
                .read_item::<TestItem>("Partition@1", "Item@1", None)
                .await;
            match result {
                Ok(_) => return Err("expected a 404 error when reading the deleted item".into()),
                Err(err) => {
                    assert_eq!(
                        Some(azure_core::http::StatusCode::NotFound),
                        err.http_status()
                    );
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
    TestClient::run(
        async |run_context| {
            let container_client = create_container(run_context).await?;

            // Create an item
            let item = TestItem {
                id: "Item1".into(),
                partition_key: Some("Partition1".into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };
            container_client
                .create_item("Partition1", &item, None)
                .await?;

            let read_item: serde_json::Value = container_client
                .read_item("Partition1", "Item1", None)
                .await?
                .into_model()?;
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
    TestClient::run(
        async |run_context| {
            let container_client = create_container(run_context).await?;

            let item = TestItem {
                id: "Item1".into(),
                partition_key: Some("Partition1".into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };
            container_client
                .upsert_item("Partition1", &item, None)
                .await?;

            let read_item: TestItem = container_client
                .read_item("Partition1", "Item1", None)
                .await?
                .into_model()?;
            assert_eq!(item, read_item);

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
pub async fn item_upsert_existing() -> Result<(), Box<dyn Error>> {
    TestClient::run(
        async |run_context| {
            let container_client = create_container(run_context).await?;

            let mut item = TestItem {
                id: "Item1".into(),
                partition_key: Some("Partition1".into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };
            container_client
                .create_item("Partition1", &item, None)
                .await?;

            item.value = 24;
            item.nested.nested_value = "Updated".into();

            let updated_item: TestItem = container_client
                .upsert_item(
                    "Partition1",
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
    TestClient::run(
        async |run_context| {
            let container_client = create_container(run_context).await?;

            let item = TestItem {
                id: "Item1".into(),
                partition_key: Some("Partition1".into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };
            container_client
                .create_item("Partition1", &item, None)
                .await?;

            let patch = PatchDocument::default()
                .with_replace("/nested/nested_value", "Patched")?
                .with_increment("/value", 10)?;
            container_client
                .patch_item("Partition1", "Item1", patch, None)
                .await?;

            let patched_item: TestItem = container_client
                .read_item("Partition1", "Item1", None)
                .await?
                .into_model()?;
            assert_eq!("Patched", patched_item.nested.nested_value);
            assert_eq!(52, patched_item.value);

            let patch = PatchDocument::default().with_replace("/bool_value", false)?;
            let response_item: TestItem = container_client
                .patch_item(
                    "Partition1",
                    "Item1",
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
    TestClient::run(
        async |run_context| {
            let container_client = create_container(run_context).await?;

            let mut item = TestItem {
                id: "Item1".into(),
                partition_key: None,
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };
            container_client
                .create_item(PartitionKey::NULL, &item, None)
                .await?;

            item.value = 24;
            item.nested.nested_value = "Updated".into();

            container_client
                .upsert_item(PartitionKey::NULL, &item, None)
                .await?;

            let read_item: TestItem = container_client
                .read_item(PartitionKey::NULL, "Item1", None)
                .await?
                .into_model()?;
            assert_eq!(item, read_item);

            container_client
                .patch_item(
                    PartitionKey::NULL,
                    "Item1",
                    PatchDocument::default().with_set("/value", 10)?,
                    None,
                )
                .await?;

            let read_item: TestItem = container_client
                .read_item(PartitionKey::NULL, "Item1", None)
                .await?
                .into_model()?;
            assert_eq!(10, read_item.value);

            container_client
                .delete_item(PartitionKey::NULL, "Item1", None)
                .await?;

            let result = container_client
                .read_item::<()>(PartitionKey::NULL, "Item1", None)
                .await;
            match result {
                Ok(_) => return Err("expected a 404 error when reading the deleted item".into()),
                Err(err) => {
                    assert_eq!(
                        Some(azure_core::http::StatusCode::NotFound),
                        err.http_status()
                    );
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
    TestClient::run(
        async |run_context| {
            let container_client = create_container(run_context).await?;

            //Create an item
            let mut item = TestItem {
                id: "Item1".into(),
                partition_key: Some("Partition1".into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };

            let response = container_client
                .create_item("Partition1", &item, None)
                .await?;

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
                    "Partition1",
                    "Item1",
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
                    "Partition1",
                    "Item1",
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
    TestClient::run(
        async |run_context| {
            let container_client = create_container(run_context).await?;

            //Create an item
            let mut item = TestItem {
                id: "Item1".into(),
                partition_key: Some("Partition1".into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };

            let response = container_client
                .create_item("Partition1", &item, None)
                .await?;

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
                    "Partition1",
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
                    "Partition1",
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
    TestClient::run(
        async |run_context| {
            let container_client = create_container(run_context).await?;

            //Create an item
            let item = TestItem {
                id: "Item1".into(),
                partition_key: Some("Partition1".into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };

            let response = container_client
                .create_item("Partition1", &item, None)
                .await?;

            //Store Etag from response
            let etag: Etag = response
                .headers()
                .get_str(&azure_core::http::headers::ETAG)
                .expect("expected the etag to be returned")
                .into();

            //Delete item with correct Etag
            container_client
                .delete_item(
                    "Partition1",
                    "Item1",
                    Some(ItemOptions {
                        if_match_etag: Some(etag),
                        ..Default::default()
                    }),
                )
                .await?;

            //Add item again for second delete test
            container_client
                .create_item("Partition1", &item, None)
                .await?;

            //Delete item with incorrect Etag
            let response = container_client
                .delete_item(
                    "Partition1",
                    "Item1",
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
    TestClient::run(
        async |run_context| {
            let container_client = create_container(run_context).await?;

            //Create an item
            let item = TestItem {
                id: "Item1".into(),
                partition_key: Some("Partition1".into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };

            let response = container_client
                .create_item("Partition1", &item, None)
                .await?;

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
                    "Partition1",
                    "Item1",
                    patch,
                    Some(ItemOptions {
                        if_match_etag: Some(etag),
                        ..Default::default()
                    }),
                )
                .await?;

            let patched_item: TestItem = container_client
                .read_item("Partition1", "Item1", None)
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
                    "Partition1",
                    "Item1",
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
