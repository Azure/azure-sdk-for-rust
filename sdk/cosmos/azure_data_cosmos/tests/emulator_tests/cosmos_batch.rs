// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![cfg(feature = "key_auth")]

// Use the shared test framework declared in `tests/emulator/mod.rs`.
use super::framework;

use azure_core::http::StatusCode;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::models::{ContainerProperties, PatchDocument};
use azure_data_cosmos::TransactionalBatch;
use framework::TestClient;
use framework::TestRunContext;
use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
struct BatchTestItem {
    id: String,
    partition_key: String,
    value: i32,
    name: String,
}

async fn create_container(run_context: &TestRunContext) -> azure_core::Result<ContainerClient> {
    let db_client = run_context.create_db().await?;
    let container_id = format!("BatchContainer-{}", Uuid::new_v4());
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
pub async fn batch_create_and_read() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(async |run_context, _db_client| {
        let container_client = create_container(run_context).await?;
        let partition_key = format!("pk-{}", Uuid::new_v4());

        let item1 = BatchTestItem {
            id: "item1".to_string(),
            partition_key: partition_key.clone(),
            value: 100,
            name: "First Item".to_string(),
        };

        let item2 = BatchTestItem {
            id: "item2".to_string(),
            partition_key: partition_key.clone(),
            value: 200,
            name: "Second Item".to_string(),
        };

        // Create a batch with create and read operations
        let batch = TransactionalBatch::new(&partition_key)
            .create_item(&item1)?
            .create_item(&item2)?
            .read_item("item1");

        let response = container_client
            .execute_transactional_batch(batch, None)
            .await?;

        assert_eq!(response.status(), StatusCode::Ok);

        let batch_response = response.into_model()?;
        assert_eq!(batch_response.results.len(), 3);

        // Verify all operations succeeded
        for result in &batch_response.results {
            assert!(
                result.is_success(),
                "Operation failed with status code: {}",
                result.status_code
            );
        }

        // The first two operations are creates (status 201)
        assert_eq!(batch_response.results[0].status_code, 201);
        assert_eq!(batch_response.results[1].status_code, 201);

        // The third operation is a read (status 200)
        assert_eq!(batch_response.results[2].status_code, 200);
        let read_item: BatchTestItem = batch_response.results[2]
            .deserialize_body()?
            .expect("Read operation should return an item");
        assert_eq!(read_item.id, "item1");
        assert_eq!(read_item.value, 100);

        Ok(())
    }, None)
    .await
}

#[tokio::test]
pub async fn batch_mixed_operations() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(async |run_context, _db_client| {
        let container_client = create_container(run_context).await?;
        let partition_key = format!("pk-{}", Uuid::new_v4());

        // First create some items directly
        let item1 = BatchTestItem {
            id: "item1".to_string(),
            partition_key: partition_key.clone(),
            value: 100,
            name: "First Item".to_string(),
        };

        let item2 = BatchTestItem {
            id: "item2".to_string(),
            partition_key: partition_key.clone(),
            value: 200,
            name: "Second Item".to_string(),
        };

        container_client
            .create_item(&partition_key, &item1, None)
            .await?;
        container_client
            .create_item(&partition_key, &item2, None)
            .await?;

        // Now execute a batch with mixed operations
        let updated_item1 = BatchTestItem {
            id: "item1".to_string(),
            partition_key: partition_key.clone(),
            value: 150,
            name: "Updated First Item".to_string(),
        };

        let item3 = BatchTestItem {
            id: "item3".to_string(),
            partition_key: partition_key.clone(),
            value: 300,
            name: "Third Item".to_string(),
        };

        let batch = TransactionalBatch::new(&partition_key)
            .replace_item("item1", &updated_item1)?
            .create_item(&item3)?
            .delete_item("item2");

        let response = container_client
            .execute_transactional_batch(batch, None)
            .await?;

        assert_eq!(response.status(), StatusCode::Ok);

        let batch_response = response.into_model()?;
        assert_eq!(batch_response.results.len(), 3);

        // Verify all operations succeeded
        for result in &batch_response.results {
            assert!(
                result.is_success(),
                "Operation failed with status code: {}",
                result.status_code
            );
        }

        // Verify item1 was replaced
        let read_item1 = run_context
            .read_item::<BatchTestItem>(&container_client, &partition_key, "item1", None)
            .await?
            .into_model()?;
        assert_eq!(read_item1.value, 150);
        assert_eq!(read_item1.name, "Updated First Item");

        // Verify item3 was created
        let read_item3 = run_context
            .read_item::<BatchTestItem>(&container_client, &partition_key, "item3", None)
            .await?
            .into_model()?;
        assert_eq!(read_item3.value, 300);

        // Verify item2 was deleted (should return 404)
        let read_result = run_context
            .read_item::<BatchTestItem>(&container_client, &partition_key, "item2", None)
            .await;
        assert!(read_result.is_err());

        Ok(())
    }, None)
    .await
}

#[tokio::test]
pub async fn batch_with_patch() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(async |run_context, _db_client| {
        let container_client = create_container(run_context).await?;
        let partition_key = format!("pk-{}", Uuid::new_v4());

        // Create an item first
        let item1 = BatchTestItem {
            id: "item1".to_string(),
            partition_key: partition_key.clone(),
            value: 100,
            name: "First Item".to_string(),
        };

        container_client
            .create_item(&partition_key, &item1, None)
            .await?;

        // Execute a batch with patch operation
        let patch = PatchDocument::default()
            .with_set("/value", 999)?
            .with_set("/name", "Patched Item")?;

        let item2 = BatchTestItem {
            id: "item2".to_string(),
            partition_key: partition_key.clone(),
            value: 200,
            name: "Second Item".to_string(),
        };

        let batch = TransactionalBatch::new(&partition_key)
            .patch_item("item1", patch)
            .create_item(&item2)?;

        let response = container_client
            .execute_transactional_batch(batch, None)
            .await?;

        assert_eq!(response.status(), StatusCode::Ok);

        let batch_response = response.into_model()?;
        assert_eq!(batch_response.results.len(), 2);

        // Verify all operations succeeded
        for result in &batch_response.results {
            assert!(
                result.is_success(),
                "Operation failed with status code: {}",
                result.status_code
            );
        }

        // Verify item1 was patched
        let read_item1 = run_context
            .read_item::<BatchTestItem>(&container_client, &partition_key, "item1", None)
            .await?
            .into_model()?;
        assert_eq!(read_item1.value, 999);
        assert_eq!(read_item1.name, "Patched Item");

        Ok(())
    }, None)
    .await
}

#[tokio::test]
pub async fn batch_atomicity_on_failure() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(async |run_context, _db_client| {
        let container_client = create_container(run_context).await?;
        let partition_key = format!("pk-{}", Uuid::new_v4());

        // Create an item first
        let item1 = BatchTestItem {
            id: "item1".to_string(),
            partition_key: partition_key.clone(),
            value: 100,
            name: "First Item".to_string(),
        };

        container_client
            .create_item(&partition_key, &item1, None)
            .await?;

        // Try to create a batch that will fail (trying to delete a non-existent item)
        // This should cause the entire batch to fail and roll back
        let item2 = BatchTestItem {
            id: "item2".to_string(),
            partition_key: partition_key.clone(),
            value: 200,
            name: "Second Item".to_string(),
        };

        let batch = TransactionalBatch::new(&partition_key)
            .create_item(&item2)?
            .delete_item("nonexistent_item"); // This will fail

        let response = container_client
            .execute_transactional_batch(batch, None)
            .await;

        // The batch should fail
        assert!(response.is_err(), "Expected batch to fail");

        // Verify item2 was NOT created (due to rollback)
        let read_result = run_context
            .read_item::<BatchTestItem>(&container_client, &partition_key, "item2", None)
            .await;
        assert!(
            read_result.is_err(),
            "item2 should not exist due to batch rollback"
        );

        Ok(())
    }, None)
    .await
}
