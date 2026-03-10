// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Tests for retry policies with 408 (Timeout) and 500 (Internal Server Error)
//! errors in multi-write scenarios.
//!
//! Verifies that:
//! - Read and query operations retry across regions on 408 and 500 errors.
//! - Write operations (create, upsert, replace, delete) do NOT retry across
//!   regions on 408 errors.

#![cfg(feature = "key_auth")]
#![cfg(feature = "fault_injection")]

use super::framework;

use azure_core::http::StatusCode;
use azure_core::Uuid;
use azure_data_cosmos::fault_injection::{
    FaultInjectionClientBuilder, FaultInjectionConditionBuilder, FaultInjectionErrorType,
    FaultInjectionResultBuilder, FaultInjectionRuleBuilder, FaultOperationType,
};
use azure_data_cosmos::models::{ContainerProperties, ThroughputProperties};
use azure_data_cosmos::Query;
use framework::{TestClient, TestOptions, HUB_REGION, SATELLITE_REGION};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
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

/// Test that reads retry across regions on 408 (Timeout) errors.
///
/// Injects a 408 on the hub region for ReadItem with a hit limit of 1.
/// The read should fail on the hub region and succeed via cross-region
/// retry on the satellite region.
#[tokio::test]
pub async fn read_cross_region_retry_on_408() -> Result<(), Box<dyn Error>> {
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::Timeout)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(HUB_REGION)
        .build();

    let rule = FaultInjectionRuleBuilder::new("read-408-hub", server_error)
        .with_condition(condition)
        .with_hit_limit(1)
        .build();

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::new(rule));

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            let unique_id = Uuid::new_v4().to_string();
            let item = TestItem {
                id: format!("Item-{}", unique_id).into(),
                partition_key: Some(format!("Partition-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };

            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await;

            // Read should succeed via cross-region retry after hub returns 408
            let result = run_context
                .read_item::<TestItem>(&fault_container_client, &pk, &item_id, None)
                .await;

            assert!(
                result.is_ok(),
                "Read should succeed via cross-region retry on 408, but got error: {:?}",
                result.err()
            );

            let response = result.unwrap();
            let request_url = response.request_url().to_string();
            assert!(
                request_url.contains(&SATELLITE_REGION.as_str()),
                "read should have failed over to satellite region, but URL was: {}",
                request_url
            );

            Ok(())
        },
        Some(
            TestOptions::new()
                .with_fault_injection_builder(fault_builder)
                .with_fault_client_application_region(HUB_REGION),
        ),
    )
    .await
}

/// Test that writes do NOT retry across regions on 408 (Timeout) errors.
///
/// Injects a 408 on all regions for CreateItem. The write should fail
/// with a 408 status code rather than retrying on a different region.
#[tokio::test]
pub async fn write_no_cross_region_retry_on_408() -> Result<(), Box<dyn Error>> {
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::Timeout)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .build();

    let rule = FaultInjectionRuleBuilder::new("write-408-all-regions", server_error)
        .with_condition(condition)
        .build();

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::new(rule));

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await;

            let unique_id = Uuid::new_v4().to_string();
            let item = TestItem {
                id: format!("Item-{}", unique_id).into(),
                partition_key: Some(format!("Partition-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };
            let pk = format!("Partition-{}", unique_id);

            // Write should fail with 408 — no cross-region retry for writes
            let result = fault_container_client.create_item(&pk, &item, None).await;

            let err = result.expect_err("write should fail with 408 and not retry across regions");
            assert_eq!(
                Some(StatusCode::RequestTimeout),
                err.http_status(),
                "expected RequestTimeout (408), got {:?}",
                err.http_status()
            );

            Ok(())
        },
        Some(
            TestOptions::new()
                .with_fault_injection_builder(fault_builder)
                .with_fault_client_application_region(HUB_REGION),
        ),
    )
    .await
}

/// Test that upsert operations do NOT retry across regions on 408 (Timeout) errors.
///
/// Same as the create test above but for upserts, verifying that the no-cross-region
/// retry policy applies consistently to all write operations.
#[tokio::test]
pub async fn upsert_no_cross_region_retry_on_408() -> Result<(), Box<dyn Error>> {
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::Timeout)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::UpsertItem)
        .build();

    let rule = FaultInjectionRuleBuilder::new("upsert-408-all-regions", server_error)
        .with_condition(condition)
        .build();

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::new(rule));

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await;

            let unique_id = Uuid::new_v4().to_string();
            let item = TestItem {
                id: format!("Item-{}", unique_id).into(),
                partition_key: Some(format!("Partition-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };
            let pk = format!("Partition-{}", unique_id);

            // Upsert should fail with 408 — no cross-region retry for writes
            let result = fault_container_client.upsert_item(&pk, &item, None).await;

            let err = result.expect_err("upsert should fail with 408 and not retry across regions");
            assert_eq!(
                Some(StatusCode::RequestTimeout),
                err.http_status(),
                "expected RequestTimeout (408), got {:?}",
                err.http_status()
            );

            Ok(())
        },
        Some(
            TestOptions::new()
                .with_fault_injection_builder(fault_builder)
                .with_fault_client_application_region(HUB_REGION),
        ),
    )
    .await
}

/// Test that query operations retry across regions on 408 (Timeout) errors.
///
/// Queries are read-only operations and should cross-region retry just like
/// point reads. Injects a 408 on the hub region for QueryItem with a hit
/// limit of 1, then verifies the query succeeds via the satellite region.
#[tokio::test]
pub async fn query_cross_region_retry_on_408() -> Result<(), Box<dyn Error>> {
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::Timeout)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::QueryItem)
        .with_region(HUB_REGION)
        .build();

    let rule = FaultInjectionRuleBuilder::new("query-408-hub", server_error)
        .with_condition(condition)
        .with_hit_limit(1)
        .build();

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::new(rule));

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            let unique_id = Uuid::new_v4().to_string();
            let item = TestItem {
                id: format!("Item-{}", unique_id).into(),
                partition_key: Some(format!("Partition-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };
            let pk = format!("Partition-{}", unique_id);

            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await;

            let query = Query::from(format!("SELECT * FROM c WHERE c.partition_key = '{}'", pk));

            // Query should succeed via cross-region retry after hub returns 408
            let items: Vec<TestItem> = run_context
                .query_items(&fault_container_client, query, &pk)
                .await?;

            assert!(
                !items.is_empty(),
                "query should return items after cross-region retry on 408"
            );
            assert_eq!(items[0].value, 42);

            Ok(())
        },
        Some(
            TestOptions::new()
                .with_fault_injection_builder(fault_builder)
                .with_fault_client_application_region(HUB_REGION),
        ),
    )
    .await
}

/// Test that reads retry across regions on 500 (Internal Server Error).
///
/// Injects a 500 on the hub region for ReadItem with a hit limit of 1.
/// The read should fail on the hub region and succeed via cross-region
/// retry on the satellite region.
#[tokio::test]
pub async fn read_cross_region_retry_on_500() -> Result<(), Box<dyn Error>> {
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::InternalServerError)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(HUB_REGION)
        .build();

    let rule = FaultInjectionRuleBuilder::new("read-500-hub", server_error)
        .with_condition(condition)
        .with_hit_limit(1)
        .build();

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::new(rule));

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            let unique_id = Uuid::new_v4().to_string();
            let item = TestItem {
                id: format!("Item-{}", unique_id).into(),
                partition_key: Some(format!("Partition-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await;

            // Read should succeed via cross-region retry after hub returns 500
            let result = run_context
                .read_item::<TestItem>(&fault_container_client, &pk, &item_id, None)
                .await;

            assert!(
                result.is_ok(),
                "Read should succeed via cross-region retry on 500, but got error: {:?}",
                result.err()
            );

            let response = result.unwrap();
            let request_url = response.request_url().to_string();
            assert!(
                request_url.contains(&SATELLITE_REGION.as_str()),
                "read should have failed over to satellite region, but URL was: {}",
                request_url
            );

            Ok(())
        },
        Some(
            TestOptions::new()
                .with_fault_injection_builder(fault_builder)
                .with_fault_client_application_region(HUB_REGION),
        ),
    )
    .await
}

/// Test that replace operations do NOT retry across regions on 408 (Timeout) errors.
///
/// Injects a 408 on all regions for ReplaceItem. The replace should fail
/// with a 408 status code rather than retrying on a different region.
#[tokio::test]
pub async fn replace_no_cross_region_retry_on_408() -> Result<(), Box<dyn Error>> {
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::Timeout)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReplaceItem)
        .build();

    let rule = FaultInjectionRuleBuilder::new("replace-408-all-regions", server_error)
        .with_condition(condition)
        .build();

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::new(rule));

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            let unique_id = Uuid::new_v4().to_string();
            let item = TestItem {
                id: format!("Item-{}", unique_id).into(),
                partition_key: Some(format!("Partition-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            // Create the item first via the non-fault client
            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await;

            let updated_item = TestItem {
                id: item_id.clone().into(),
                partition_key: Some(pk.clone().into()),
                value: 99,
                nested: NestedItem {
                    nested_value: "Updated".into(),
                },
                bool_value: false,
            };

            // Replace should fail with 408 — no cross-region retry for writes
            let result = fault_container_client
                .replace_item(&pk, &item_id, &updated_item, None)
                .await;

            let err =
                result.expect_err("replace should fail with 408 and not retry across regions");
            assert_eq!(
                Some(StatusCode::RequestTimeout),
                err.http_status(),
                "expected RequestTimeout (408), got {:?}",
                err.http_status()
            );

            Ok(())
        },
        Some(
            TestOptions::new()
                .with_fault_injection_builder(fault_builder)
                .with_fault_client_application_region(HUB_REGION),
        ),
    )
    .await
}

/// Test that delete operations do NOT retry across regions on 408 (Timeout) errors.
///
/// Injects a 408 on all regions for DeleteItem. The delete should fail
/// with a 408 status code rather than retrying on a different region.
#[tokio::test]
pub async fn delete_no_cross_region_retry_on_408() -> Result<(), Box<dyn Error>> {
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::Timeout)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::DeleteItem)
        .build();

    let rule = FaultInjectionRuleBuilder::new("delete-408-all-regions", server_error)
        .with_condition(condition)
        .build();

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::new(rule));

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            let unique_id = Uuid::new_v4().to_string();
            let item = TestItem {
                id: format!("Item-{}", unique_id).into(),
                partition_key: Some(format!("Partition-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            // Create the item first via the non-fault client
            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await;

            // Delete should fail with 408 — no cross-region retry for writes
            let result = fault_container_client
                .delete_item(&pk, &item_id, None)
                .await;

            let err = result.expect_err("delete should fail with 408 and not retry across regions");
            assert_eq!(
                Some(StatusCode::RequestTimeout),
                err.http_status(),
                "expected RequestTimeout (408), got {:?}",
                err.http_status()
            );

            Ok(())
        },
        Some(
            TestOptions::new()
                .with_fault_injection_builder(fault_builder)
                .with_fault_client_application_region(HUB_REGION),
        ),
    )
    .await
}
