// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration tests for fault injection framework.
//! These tests run against the Cosmos DB emulator.

#![cfg(feature = "key_auth")]
#![cfg(feature = "fault_injection")]

use super::framework;

use azure_core::{http::StatusCode, Uuid};
use azure_data_cosmos::fault_injection::{
    FaultInjectionClientBuilder, FaultInjectionConditionBuilder, FaultInjectionErrorType,
    FaultInjectionResultBuilder, FaultInjectionRuleBuilder, FaultOperationType,
};
use azure_data_cosmos::models::{ContainerProperties, ThroughputProperties};
use framework::{get_effective_hub_endpoint, TestClient, TestOptions};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::error::Error;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
struct NestedItem {
    nested_value: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
struct TestItem {
    id: Cow<'static, str>,
    partition_key: Option<Cow<'static, str>>,
    value: usize,
    nested: NestedItem,
    bool_value: bool,
}

fn create_test_item(unique_id: &str) -> TestItem {
    TestItem {
        id: format!("Item-{}", unique_id).into(),
        partition_key: Some(format!("Partition-{}", unique_id).into()),
        value: 42,
        nested: NestedItem {
            nested_value: "Nested".into(),
        },
        bool_value: true,
    }
}

/// Test probability fault injection - fault should only apply based on probability.
/// With probability 0.0, the fault should never be applied.
#[tokio::test]
pub async fn fault_injection_probability_zero_never_fails() -> Result<(), Box<dyn Error>> {
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(0.0) // Never inject
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let rule = FaultInjectionRuleBuilder::new("probability-zero", server_error)
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
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await;

            // With probability 0.0, all reads should succeed
            for i in 1..=5 {
                let result = fault_container_client
                    .read_item::<TestItem>(&pk, &item_id, None)
                    .await;
                assert!(
                    result.is_ok(),
                    "read {} should succeed with probability 0.0: {:?}",
                    i,
                    result.err()
                );
            }

            Ok(())
        },
        Some(TestOptions::new().with_fault_injection_builder(fault_builder)),
    )
    .await
}

/// Test probability fault injection - with probability 1.0, fault should always apply.
#[tokio::test]
pub async fn fault_injection_probability_one_always_fails() -> Result<(), Box<dyn Error>> {
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0) // Always inject
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let rule = FaultInjectionRuleBuilder::new("probability-one", server_error)
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
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await;

            // With probability 1.0, all reads should fail
            for i in 1..=5 {
                let result = fault_container_client
                    .read_item::<TestItem>(&pk, &item_id, None)
                    .await;
                let err =
                    result.expect_err(&format!("read {} should fail with probability 1.0", i));
                assert_eq!(
                    Some(StatusCode::ServiceUnavailable),
                    err.http_status(),
                    "read {} should return 503",
                    i
                );
            }

            Ok(())
        },
        Some(TestOptions::new().with_fault_injection_builder(fault_builder)),
    )
    .await
}

/// Test retry on transient errors with hit_limit.
/// Injects 429 for the first 2 requests, verifies 3rd succeeds.
#[tokio::test]
pub async fn fault_injection_429_retry_with_hit_limit() -> Result<(), Box<dyn Error>> {
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::TooManyRequests)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let rule = FaultInjectionRuleBuilder::new("throttle-with-limit", server_error)
        .with_condition(condition)
        .with_hit_limit(2)
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
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await;

            // First request - should succeed after retries
            let result = fault_container_client
                .read_item::<TestItem>(&pk, &item_id, None)
                .await;
            // Verify the read succeeded
            assert!(
                result.is_ok(),
                "Read should succeed after hit_limit exhausted, but got error: {:?}",
                result.err()
            );

            let response = result.unwrap();
            assert_eq!(response.status(), StatusCode::Ok);
            assert_eq!(
                response.request_url().host_str().unwrap(),
                get_effective_hub_endpoint()
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_injection_builder(fault_builder)),
    )
    .await
}

/// Test DeleteItem fault - verify CRUD operations unaffected.
#[tokio::test]
pub async fn fault_injection_delete_item_fault_crud_succeeds() -> Result<(), Box<dyn Error>> {
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::DeleteItem)
        .build();

    let rule = FaultInjectionRuleBuilder::new("delete-fails", server_error)
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
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            // Create using normal client
            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await;

            // Read should succeed
            let read_result = fault_container_client
                .read_item::<TestItem>(&pk, &item_id, None)
                .await;
            assert!(
                read_result.is_ok(),
                "read should succeed: {:?}",
                read_result.err()
            );

            // Upsert should succeed
            let mut updated_item = item.clone();
            updated_item.value = 100;
            let upsert_result = fault_container_client
                .upsert_item(&pk, &updated_item, None)
                .await;
            assert!(
                upsert_result.is_ok(),
                "upsert should succeed: {:?}",
                upsert_result.err()
            );

            // Delete should fail with 503 because the fault targets DeleteItem
            let delete_result = fault_container_client
                .delete_item(&pk, &item_id, None)
                .await;
            let err = delete_result.expect_err("delete should fail due to fault injection");
            assert_eq!(
                Some(StatusCode::ServiceUnavailable),
                err.http_status(),
                "delete should return 503 ServiceUnavailable"
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_injection_builder(fault_builder)),
    )
    .await
}

/// Test container-specific fault - verify other containers unaffected.
#[tokio::test]
pub async fn fault_injection_container_specific() -> Result<(), Box<dyn Error>> {
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();

    // Create a condition that targets a specific container name pattern
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_container_id("FaultyContainer")
        .build();

    let rule = FaultInjectionRuleBuilder::new("container-specific", server_error)
        .with_condition(condition)
        .build();

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::new(rule));

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            // Create a container that doesn't match the fault condition
            let container_id = format!("SafeContainer-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            let unique_id = Uuid::new_v4().to_string();
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await;

            // Read should succeed since container name doesn't match "FaultyContainer"
            let result = fault_container_client
                .read_item::<TestItem>(&pk, &item_id, None)
                .await;

            assert!(
                result.is_ok(),
                "read should succeed for non-matching container: {:?}",
                result.err()
            );

            // Create a container with name "FaultyContainer" and verify read fails
            let faulty_container_id = "FaultyContainer";
            run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties::new(faulty_container_id, "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            // Now try to read using the fault client - should fail because container name contains "FaultyContainer"
            let faulty_fault_container_client =
                fault_db_client.container_client(faulty_container_id).await;
            let faulty_result = faulty_fault_container_client
                .read_item::<TestItem>(&pk, &item_id, None)
                .await;

            let err = faulty_result
                .expect_err("read should fail for container matching 'FaultyContainer'");
            assert_eq!(
                Some(StatusCode::ServiceUnavailable),
                err.http_status(),
                "expected 503 ServiceUnavailable for FaultyContainer"
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_injection_builder(fault_builder)),
    )
    .await
}

/// Test multiple rules priority - first matching rule wins.
#[tokio::test]
pub async fn fault_injection_multiple_rules_priority() -> Result<(), Box<dyn Error>> {
    // First rule: 429 for ReadItem
    let error1 = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::TooManyRequests)
        .build();
    let condition1 = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();
    let rule1 = FaultInjectionRuleBuilder::new("first-rule-429", error1)
        .with_condition(condition1)
        .build();

    // Second rule: 503 for ReadItem (should not be applied since first rule matches)
    let error2 = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();
    let condition2 = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();
    let rule2 = FaultInjectionRuleBuilder::new("second-rule-503", error2)
        .with_condition(condition2)
        .build();

    let fault_builder = FaultInjectionClientBuilder::new()
        .with_rule(Arc::new(rule1))
        .with_rule(Arc::new(rule2));

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
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await;

            let result = fault_container_client
                .read_item::<TestItem>(&pk, &item_id, None)
                .await;

            // Should get 429 (first rule), not 503 (second rule)
            let err = result.expect_err("expected first rule (429) to apply");
            assert_eq!(
                Some(StatusCode::TooManyRequests),
                err.http_status(),
                "first matching rule should win (429, not 503)"
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_injection_builder(fault_builder)),
    )
    .await
}

/// Test that first rule is skipped because its start_time is in the future.
/// Second rule applies immediately and should win.
#[tokio::test]
pub async fn fault_injection_first_rule_inactive_due_to_start_time() -> Result<(), Box<dyn Error>> {
    // First rule: 429 for ReadItem, but with a future start_time (won't be active yet)
    let error1 = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::TooManyRequests)
        .build();
    let condition1 = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();
    let rule1 = FaultInjectionRuleBuilder::new("first-rule-429-future", error1)
        .with_condition(condition1)
        .with_start_time(Instant::now() + Duration::from_secs(300))
        .build();

    // Second rule: 503 for ReadItem (should be applied since first rule hasn't started)
    let error2 = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();
    let condition2 = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();
    let rule2 = FaultInjectionRuleBuilder::new("second-rule-503", error2)
        .with_condition(condition2)
        .build();

    let fault_builder = FaultInjectionClientBuilder::new()
        .with_rule(Arc::new(rule1))
        .with_rule(Arc::new(rule2));

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
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await;

            let result = fault_container_client
                .read_item::<TestItem>(&pk, &item_id, None)
                .await;

            // Should get 503 (second rule) because first rule hasn't started yet
            let err = result.expect_err("expected second rule (503) to apply");
            assert_eq!(
                Some(StatusCode::ServiceUnavailable),
                err.http_status(),
                "second rule should apply (503) since first rule has not started"
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_injection_builder(fault_builder)),
    )
    .await
}

/// Test that first rule is expired because its end_time is in the past.
/// Second rule should win.
#[tokio::test]
pub async fn fault_injection_first_rule_expired_due_to_end_time() -> Result<(), Box<dyn Error>> {
    // First rule: 429 for ReadItem, but with an end_time in the past (already expired)
    let error1 = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::TooManyRequests)
        .build();
    let condition1 = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();
    let rule1 = FaultInjectionRuleBuilder::new("first-rule-429-expired", error1)
        .with_condition(condition1)
        .with_end_time(Instant::now()) // Already expired
        .build();

    // Second rule: 503 for ReadItem (should be applied since first rule is expired)
    let error2 = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();
    let condition2 = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();
    let rule2 = FaultInjectionRuleBuilder::new("second-rule-503", error2)
        .with_condition(condition2)
        .build();

    let fault_builder = FaultInjectionClientBuilder::new()
        .with_rule(Arc::new(rule1))
        .with_rule(Arc::new(rule2));

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
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await;

            // Small delay to ensure duration has passed
            tokio::time::sleep(Duration::from_millis(100)).await;

            let result = fault_container_client
                .read_item::<TestItem>(&pk, &item_id, None)
                .await;

            // Should get 503 (second rule) because first rule's duration has expired
            let err = result.expect_err("expected second rule (503) to apply");
            assert_eq!(
                Some(StatusCode::ServiceUnavailable),
                err.http_status(),
                "second rule should apply (503) since first rule's end_time has passed"
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_injection_builder(fault_builder)),
    )
    .await
}

/// Test hit_limit behavior - fault stops after N applications.
#[tokio::test]
pub async fn fault_injection_hit_limit_behavior() -> Result<(), Box<dyn Error>> {
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::InternalServerError)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let rule = FaultInjectionRuleBuilder::new("hit-limit-test", server_error)
        .with_condition(condition)
        .with_hit_limit(4)
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
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await;

            // First 2 requests should fail with one in region retry
            for i in 1..=2 {
                let result = fault_container_client
                    .read_item::<TestItem>(&pk, &item_id, None)
                    .await;
                assert!(
                    result.is_err(),
                    "request {} should fail (within hit_limit)",
                    i
                );
                assert_eq!(
                    Some(StatusCode::InternalServerError),
                    result.unwrap_err().http_status()
                );
            }

            // After hit_limit is exhausted by retries, the next read should succeed
            let result = run_context
                .read_item::<TestItem>(&fault_container_client, &pk, &item_id, None)
                .await;
            assert!(
                result.is_ok(),
                "request 4 should succeed after hit_limit exhausted: {:?}",
                result.err()
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_injection_builder(fault_builder)),
    )
    .await
}

/// Test empty rules - no fault injection, operations should succeed.
#[tokio::test]
pub async fn fault_injection_empty_rules() -> Result<(), Box<dyn Error>> {
    let fault_builder = FaultInjectionClientBuilder::new();

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
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await;

            // Read should succeed with no fault rules
            let result = fault_container_client
                .read_item::<TestItem>(&pk, &item_id, None)
                .await;

            assert!(
                result.is_ok(),
                "read should succeed with empty fault rules: {:?}",
                result.err()
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_injection_builder(fault_builder)),
    )
    .await
}

/// Test that item operations succeed when metadata operations are faulted.
#[tokio::test]
pub async fn fault_injection_metadata_fault_item_ops_succeed() -> Result<(), Box<dyn Error>> {
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::InternalServerError)
        .build();

    // Fault all metadata operations
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::MetadataReadContainer)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("metadata-fails", server_error)
            .with_condition(condition)
            .build(),
    );

    // Start the rule disabled so we can warm the container cache first.
    // With partition-level circuit breaker enabled by default, item operations
    // trigger metadata reads (MetadataReadContainer) to resolve container
    // properties on a cold cache. We disable the rule during the warmup call
    // so the cache populates, then enable it to verify that subsequent item
    // operations succeed even when metadata reads are faulted.
    rule.disable();

    let rule_handle = Arc::clone(&rule);
    let fault_builder = FaultInjectionClientBuilder::new().with_rule(rule);

    TestClient::run_with_unique_db(
        async move |run_context, db_client| {
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
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await;

            // Cache warmup: read the container with the rule disabled so that
            // ContainerClient::read() populates the internal container cache.
            // Subsequent item operations (which resolve container properties
            // for partition-level routing) will find the cache warm.
            let warmup_result = fault_container_client.read(None).await;
            assert!(
                warmup_result.is_ok(),
                "warmup container read should succeed: {:?}",
                warmup_result.err()
            );

            // Enable the metadata fault rule now that the cache is warm.
            rule_handle.enable();

            // Create item should succeed
            let unique_id = Uuid::new_v4().to_string();
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            let create_result = fault_container_client.create_item(&pk, &item, None).await;
            assert!(
                create_result.is_ok(),
                "create item should succeed: {:?}",
                create_result.err()
            );

            // Read item should succeed (use run_context.read_item for replication retry)
            let read_result = run_context
                .read_item::<TestItem>(&fault_container_client, &pk, &item_id, None)
                .await;
            assert!(
                read_result.is_ok(),
                "read item should succeed: {:?}",
                read_result.err()
            );

            // Upsert should succeed
            let mut updated_item = item.clone();
            updated_item.value = 999;
            let upsert_result = fault_container_client
                .upsert_item(&pk, &updated_item, None)
                .await;
            assert!(
                upsert_result.is_ok(),
                "upsert item should succeed: {:?}",
                upsert_result.err()
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_injection_builder(fault_builder)),
    )
    .await
}

/// Test that disabling a rule at runtime prevents fault injection,
/// and re-enabling it resumes injection.
#[tokio::test]
pub async fn fault_injection_enable_disable_rule() -> Result<(), Box<dyn Error>> {
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("enable-disable-test", server_error)
            .with_condition(condition)
            .build(),
    );

    assert_eq!(rule.id, "enable-disable-test");
    assert!(rule.is_enabled());

    let rule_handle = Arc::clone(&rule);

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(rule);

    TestClient::run_with_unique_db(
        async move |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties::new(container_id.clone(), "/partition_key".into()),
                    ThroughputProperties::manual(400),
                )
                .await?;

            let unique_id = Uuid::new_v4().to_string();
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await;

            // Rule is enabled — read should fail
            let result = fault_container_client
                .read_item::<TestItem>(&pk, &item_id, None)
                .await;
            assert!(result.is_err(), "read should fail while rule is enabled");

            // Disable the rule at runtime
            rule_handle.disable();
            assert!(!rule_handle.is_enabled());

            // Read should now succeed
            let result = fault_container_client
                .read_item::<TestItem>(&pk, &item_id, None)
                .await;
            assert!(
                result.is_ok(),
                "read should succeed after disabling rule: {:?}",
                result.err()
            );

            // Re-enable the rule
            rule_handle.enable();
            assert!(rule_handle.is_enabled());

            // Read should fail again
            let result = fault_container_client
                .read_item::<TestItem>(&pk, &item_id, None)
                .await;
            assert!(result.is_err(), "read should fail after re-enabling rule");

            Ok(())
        },
        Some(TestOptions::new().with_fault_injection_builder(fault_builder)),
    )
    .await
}
