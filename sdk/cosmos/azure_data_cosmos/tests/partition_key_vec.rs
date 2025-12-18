#![cfg(feature = "key_auth")]

//! Integration tests for using Vec<PartitionKeyValue> to build partition keys at runtime.
//! Demonstrates the use case of generic code that works across containers with different
//! partition key schemas.

mod framework;

use azure_data_cosmos::{
    clients::ContainerClient, models::ContainerProperties, PartitionKey, PartitionKeyValue, Query,
};
use framework::{TestClient, TestRunContext};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, error::Error};

/// Generic item that can be used with different partition key schemas
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
struct GenericItem {
    id: Cow<'static, str>,
    tenant_id: Cow<'static, str>,
    region: Cow<'static, str>,
    user_id: Cow<'static, str>,
    data: Cow<'static, str>,
}

async fn create_container(run_context: &TestRunContext) -> azure_core::Result<ContainerClient> {
    let db_client = run_context.create_db().await?;
    db_client
        .create_container(
            ContainerProperties {
                id: "SingleLevelContainer".into(),
                partition_key: "/tenant_id".into(),
                ..Default::default()
            },
            None,
        )
        .await?;
    let container_client = db_client.container_client("SingleLevelContainer");

    Ok(container_client)
}

/// Test demonstrating Vec<PartitionKeyValue> for runtime partition key construction
#[tokio::test]
async fn vec_partition_key_dynamic_construction() -> Result<(), Box<dyn Error>> {
    TestClient::run(async |run_context| {
        let container = create_container(run_context).await?;

        // Create test items
        let item1 = GenericItem {
            id: "item1".into(),
            tenant_id: "tenant-a".into(),
            region: "us-west".into(),
            user_id: "user-123".into(),
            data: "test data 1".into(),
        };

        let item2 = GenericItem {
            id: "item2".into(),
            tenant_id: "tenant-b".into(),
            region: "us-east".into(),
            user_id: "user-456".into(),
            data: "test data 2".into(),
        };

        // Create items using dynamically constructed partition keys from Vec
        // This demonstrates the use case where partition key values are built at runtime
        let pk1 = PartitionKey::from(vec![PartitionKeyValue::from(item1.tenant_id.to_string())]);
        container.create_item(pk1.clone(), &item1, None).await?;

        let pk2 = PartitionKey::from(vec![PartitionKeyValue::from(item2.tenant_id.to_string())]);
        container.create_item(pk2.clone(), &item2, None).await?;

        // Query using dynamically constructed partition keys
        let items1: Vec<GenericItem> = container
            .query_items(
                Query::from("SELECT * FROM c WHERE c.id = @id").with_parameter("@id", "item1")?,
                pk1.clone(),
                None,
            )?
            .try_collect()
            .await?;
        assert_eq!(items1.len(), 1);
        assert_eq!(items1[0].id, "item1");
        assert_eq!(items1[0].data, "test data 1");
        assert_eq!(items1[0].tenant_id, "tenant-a");

        let items2: Vec<GenericItem> = container
            .query_items(
                Query::from("SELECT * FROM c WHERE c.id = @id").with_parameter("@id", "item2")?,
                pk2.clone(),
                None,
            )?
            .try_collect()
            .await?;
        assert_eq!(items2.len(), 1);
        assert_eq!(items2[0].id, "item2");
        assert_eq!(items2[0].data, "test data 2");
        assert_eq!(items2[0].tenant_id, "tenant-b");

        Ok(())
    })
    .await
}
