// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![cfg(feature = "key_auth")]

// Use the shared test framework declared in `tests/emulator/mod.rs`.
use super::framework;

use azure_core::http::StatusCode;
use azure_core::Uuid;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::models::ContainerProperties;
use azure_data_cosmos::options::BatchOptions;
use azure_data_cosmos::TransactionalBatch;
use framework::TestClient;
use framework::TestRunContext;
use serde::{Deserialize, Serialize};
use std::error::Error;

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
            ContainerProperties::new(container_id.clone(), "/partition_key".into()),
            None,
        )
        .await?;
    let container_client = db_client.container_client(&container_id).await;

    Ok(container_client)
}

#[tokio::test]
pub async fn batch_create_and_read() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
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
                .read_item("item1", None);

            let options = BatchOptions::default().with_content_response_on_write_enabled(true);

            let response = container_client
                .execute_transactional_batch(batch, Some(options))
                .await?;

            assert_eq!(response.status(), StatusCode::Ok);

            let batch_response = response.into_model()?;

            // Verify status codes: two creates (201) and one read (200)
            let status_codes: Vec<u16> = batch_response
                .results()
                .iter()
                .map(|r| r.status_code())
                .collect();
            assert_eq!(status_codes, vec![201, 201, 200]);

            // Verify the read operation returned the correct item
            let read_item: BatchTestItem = batch_response.results()[2]
                .deserialize_body()?
                .expect("Read operation should return an item");
            assert_eq!(read_item.id, "item1");
            assert_eq!(read_item.value, 100);

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
pub async fn batch_mixed_operations() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
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
                .replace_item("item1", &updated_item1, None)?
                .create_item(&item3)?
                .delete_item("item2", None);

            let response = container_client
                .execute_transactional_batch(batch, None)
                .await?;

            assert_eq!(response.status(), StatusCode::Ok);

            let batch_response = response.into_model()?;

            // Verify all operations succeeded: replace (200), create (201), delete (204)
            let status_codes: Vec<u16> = batch_response
                .results()
                .iter()
                .map(|r| r.status_code())
                .collect();
            assert!(
                status_codes.iter().all(|&c| c >= 200 && c < 300),
                "Expected all success status codes, got: {:?}",
                status_codes
            );
            assert_eq!(status_codes.len(), 3);

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
pub async fn batch_atomicity_on_failure() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
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
                .delete_item("nonexistent_item", None); // This will fail with 404

            let response = container_client
                .execute_transactional_batch(batch, None)
                .await?;

            // When one operation in a batch fails, the response is still successful (207 Multi-Status)
            // but individual operation results contain their status codes
            let batch_response = response.into_model()?;

            // First operation (create item2) gets 424 Failed Dependency because a subsequent operation failed
            // Second operation (delete nonexistent) gets 404 Not Found
            let status_codes: Vec<u16> = batch_response
                .results()
                .iter()
                .map(|r| r.status_code())
                .collect();
            assert_eq!(status_codes, vec![424, 404]);

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
pub async fn batch_fails_when_exceeding_max_operations() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let partition_key = format!("pk-{}", Uuid::new_v4());

            // Create a batch with 101 operations (exceeds the 100 operation limit)
            let mut batch = TransactionalBatch::new(&partition_key);
            for i in 0..101 {
                let item = BatchTestItem {
                    id: format!("item{}", i),
                    partition_key: partition_key.clone(),
                    value: i,
                    name: format!("Item #{}", i),
                };
                batch = batch.create_item(&item)?;
            }

            let response = container_client
                .execute_transactional_batch(batch, None)
                .await;

            // The batch should fail with BadRequest (400) for exceeding max operations
            assert!(
                response.is_err(),
                "Expected batch to fail when exceeding 100 operations"
            );
            let err = response.unwrap_err();
            assert_eq!(
                err.http_status(),
                Some(StatusCode::BadRequest),
                "Expected BadRequest (400) status code"
            );

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
pub async fn batch_fails_when_exceeding_max_payload_size() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_shared_db(
        async |run_context, _db_client| {
            let container_client = create_container(run_context).await?;
            let partition_key = format!("pk-{}", Uuid::new_v4());

            // Create a batch with a large payload (> 2MB)
            // Each item will have a large string field to exceed the limit
            let large_string = "x".repeat(500_000); // 500KB per item
            let mut batch = TransactionalBatch::new(&partition_key);

            // 5 items × 500KB = ~2.5MB, which exceeds the 2MB limit
            for i in 0..5 {
                let item = serde_json::json!({
                    "id": format!("large_item_{}", i),
                    "partition_key": partition_key.clone(),
                    "large_data": large_string.clone(),
                });
                batch = batch.create_item(&item)?;
            }

            let response = container_client
                .execute_transactional_batch(batch, None)
                .await;

            // The batch should fail with RequestEntityTooLarge (413) for exceeding max payload size
            assert!(
                response.is_err(),
                "Expected batch to fail when exceeding 2MB payload size"
            );
            let err = response.unwrap_err();
            assert_eq!(
                err.http_status(),
                Some(StatusCode::PayloadTooLarge),
                "Expected RequestEntityTooLarge (413) status code"
            );

            Ok(())
        },
        None,
    )
    .await
}
