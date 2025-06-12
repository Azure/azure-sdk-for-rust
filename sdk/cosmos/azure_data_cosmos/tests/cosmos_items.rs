#![cfg(feature = "key_auth")]

mod framework;

use azure_core::http::{headers::HeaderName, response};
use azure_core_test::{recorded, TestContext};
use azure_data_cosmos::{
    clients::ContainerClient,
    models::{ContainerProperties, PatchDocument},
    CosmosClient, ItemOptions, PartitionKey,
};
use framework::{test_data, TestAccount};
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

async fn create_container(
    account: &TestAccount,
    cosmos_client: &CosmosClient,
) -> azure_core::Result<ContainerClient> {
    // Create a database and a container
    let db_client = test_data::create_database(account, cosmos_client).await?;
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

#[recorded::test]
pub async fn item_create_read_replace_delete(context: TestContext) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let container_client = create_container(&account, &cosmos_client).await?;

    // Create an item
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
    let body = response.into_raw_body().collect_string().await?;
    assert_eq!("", body);

    // Try to read the item
    let read_item: TestItem = container_client
        .read_item("Partition1", "Item1", None)
        .await?
        .into_body()
        .await?;
    assert_eq!(item, read_item);

    // Replace the item
    item.value = 24;
    item.nested.nested_value = "Updated".into();

    let response = container_client
        .replace_item("Partition1", "Item1", &item, None)
        .await?;
    let body = response.into_raw_body().collect_string().await?;
    assert_eq!("", body);

    // Update again, but this time ask for the response
    item.value = 12;
    item.nested.nested_value = "UpdatedAgain".into();
    let updated_item: TestItem = container_client
        .replace_item(
            "Partition1",
            "Item1",
            &item,
            Some(ItemOptions {
                enable_content_response_on_write: true,
                ..Default::default()
            }),
        )
        .await?
        .into_raw_body()
        .json()
        .await?;
    assert_eq!(item, updated_item);

    // Delete the item
    let response = container_client
        .delete_item("Partition1", "Item1", None)
        .await?;
    let body = response.into_raw_body().collect_string().await?;
    assert_eq!("", body);

    // Try to read the item again, expecting a 404
    let result = container_client
        .read_item::<TestItem>("Partition1", "Item1", None)
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

    account.cleanup().await?;
    Ok(())
}

#[recorded::test]
pub async fn item_create_content_response_on_write(
    context: TestContext,
) -> Result<(), Box<dyn Error>> {
    use azure_data_cosmos::ItemOptions;

    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let container_client = create_container(&account, &cosmos_client).await?;

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

    let response_item: TestItem = container_client
        .create_item(
            "Partition1",
            &item,
            Some(ItemOptions {
                enable_content_response_on_write: true,
                ..Default::default()
            }),
        )
        .await?
        .into_raw_body()
        .json()
        .await?;
    assert_eq!(item, response_item);

    account.cleanup().await?;
    Ok(())
}

#[recorded::test]
pub async fn item_read_system_properties(context: TestContext) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let container_client = create_container(&account, &cosmos_client).await?;

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
        .into_body()
        .await?;
    assert!(
        read_item.get("_rid").is_some(),
        "expected _rid to be present"
    );

    assert!(
        read_item.get("_etag").is_some(),
        "expected _etag to be present"
    );

    account.cleanup().await?;
    Ok(())
}

#[recorded::test]
pub async fn item_upsert_new(context: TestContext) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let container_client = create_container(&account, &cosmos_client).await?;

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
        .into_body()
        .await?;
    assert_eq!(item, read_item);

    account.cleanup().await?;
    Ok(())
}

#[recorded::test]
pub async fn item_upsert_existing(context: TestContext) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let container_client = create_container(&account, &cosmos_client).await?;

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
        .into_raw_body()
        .json()
        .await?;
    assert_eq!(item, updated_item);

    account.cleanup().await?;
    Ok(())
}

#[recorded::test]
pub async fn item_patch(context: TestContext) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let container_client = create_container(&account, &cosmos_client).await?;

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
        .into_body()
        .await?;
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
        .into_raw_body()
        .json()
        .await?;
    assert!(!response_item.bool_value);

    account.cleanup().await?;
    Ok(())
}

#[recorded::test]
pub async fn item_null_partition_key(context: TestContext) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let container_client = create_container(&account, &cosmos_client).await?;

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
        .into_body()
        .await?;
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
        .into_body()
        .await?;
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

    account.cleanup().await?;
    Ok(())
}

#[recorded::test]
pub async fn item_replace_if_match_etag(context: TestContext) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let container_client = create_container(&account, &cosmos_client).await?;

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
    const ETAG: HeaderName = HeaderName::from_static("etag");
    let etag = response
        .headers()
        .get_str(&ETAG)
        .ok()
        .map(|s| s.to_string());

    //Replace item with correct Etag
    item.value = 24;
    item.nested.nested_value = "Updated".into();

    //could change response name to replaced item for better understanding
    let response = container_client
        .replace_item(
            "Partition1",
            "Item1",
            &item,
            Some(ItemOptions {
                if_match_etag: etag,
                enable_content_response_on_write: false,
                ..Default::default()
            }),
        )
        .await?;

    let body = response.into_raw_body().collect_string().await?;
    assert_eq!("", body);

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
                enable_content_response_on_write: true,
                ..Default::default()
            }),
        )
        .await;

    match response {
        Ok(_) => {
            return Err(
                "expected a 412 Precondition Failed error when using an incorrect ETag".into(),
            );
        }
        Err(err) => {
            assert_eq!(
                Some(azure_core::http::StatusCode::PreconditionFailed),
                err.http_status()
            );
        }
    }

    account.cleanup().await?;
    Ok(())
}

#[recorded::test]
pub async fn item_upsert_if_match_etag(context: TestContext) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let container_client = create_container(&account, &cosmos_client).await?;

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
    const ETAG: HeaderName = HeaderName::from_static("etag");
    let etag = response
        .headers()
        .get_str(&ETAG)
        .ok()
        .map(|s| s.to_string());

    //Upsert item with correct Etag
    item.value = 24;
    item.nested.nested_value = "Updated".into();

    let response = container_client
        .upsert_item(
            "Partition1",
            &item,
            Some(ItemOptions {
                if_match_etag: etag,
                enable_content_response_on_write: false,
                ..Default::default()
            }),
        )
        .await?;

    let body = response.into_raw_body().collect_string().await?;
    assert_eq!("", body);

    //Upsert item with incorrect Etag
    item.value = 52;
    item.nested.nested_value = "UpdatedAgain".into();

    let response = container_client
        .upsert_item(
            "Partition1",
            &item,
            Some(ItemOptions {
                if_match_etag: Some("incorrectEtag".into()),
                enable_content_response_on_write: false,
                ..Default::default()
            }),
        )
        .await;

    match response {
        Ok(_) => {
            return Err(
                "expected a 412 Precondition Failed error when using an incorrect ETag".into(),
            );
        }
        Err(err) => {
            assert_eq!(
                Some(azure_core::http::StatusCode::PreconditionFailed),
                err.http_status()
            );
        }
    }

    account.cleanup().await?;
    Ok(())
}

#[recorded::test]
pub async fn item_delete_if_match_etag(context: TestContext) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let container_client = create_container(&account, &cosmos_client).await?;

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
    const ETAG: HeaderName = HeaderName::from_static("etag");
    let etag = response
        .headers()
        .get_str(&ETAG)
        .ok()
        .map(|s| s.to_string());

    //Delete item with correct Etag
    let response = container_client
        .delete_item(
            "Partition1",
            "Item1",
            Some(ItemOptions {
                if_match_etag: etag,
                enable_content_response_on_write: false,
                ..Default::default()
            }),
        )
        .await?;

    let body = response.into_raw_body().collect_string().await?;
    assert_eq!("", body);

    //Add item again for second delete test
    let response = container_client
        .create_item("Partition1", &item, None)
        .await?;

    //Delete item with incorrect Etag
    let response = container_client
        .delete_item(
            "Partition1",
            "Item1",
            Some(ItemOptions {
                if_match_etag: Some("incorrectEtag".into()),
                enable_content_response_on_write: false,
                ..Default::default()
            }),
        )
        .await;
    match response {
        Ok(_) => {
            return Err(
                "expected a 412 Precondition Failed error when using an incorrect ETag".into(),
            );
        }
        Err(err) => {
            assert_eq!(
                Some(azure_core::http::StatusCode::PreconditionFailed),
                err.http_status()
            );
        }
    }

    account.cleanup().await?;
    Ok(())
}

#[recorded::test]
pub async fn item_patch_if_match_etag(context: TestContext) -> Result<(), Box<dyn Error>> {
    let account = TestAccount::from_env(context, None).await?;
    let cosmos_client = account.connect_with_key(None)?;
    let container_client = create_container(&account, &cosmos_client).await?;

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
    const ETAG: HeaderName = HeaderName::from_static("etag");
    let etag = response
        .headers()
        .get_str(&ETAG)
        .ok()
        .map(|s| s.to_string());

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
                if_match_etag: etag,
                enable_content_response_on_write: false,
                ..Default::default()
            }),
        )
        .await?;

    let patched_item: TestItem = container_client
        .read_item("Partition1", "Item1", None)
        .await?
        .into_body()
        .await?;
    assert_eq!("Patched", patched_item.nested.nested_value);
    assert_eq!(52, patched_item.value);

    //Patch item with incorrect Etag
    let patch = PatchDocument::default()
        .with_replace("/nested/nested_value", "Patched_Incorrect")?
        .with_increment("/value", 15)?;

    let response = container_client
        .patch_item(
            "Partition1",
            "Item1",
            patch,
            Some(ItemOptions {
                if_match_etag: Some("incorrectEtag".into()),
                enable_content_response_on_write: false,
                ..Default::default()
            }),
        )
        .await;

    match response {
        Ok(_) => {
            return Err(
                "expected a 412 Precondition Failed error when using an incorrect ETag".into(),
            );
        }
        Err(err) => {
            assert_eq!(
                Some(azure_core::http::StatusCode::PreconditionFailed),
                err.http_status()
            );
        }
    }

    account.cleanup().await?;
    Ok(())
}
