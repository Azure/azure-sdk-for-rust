// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration tests for fault injection framework.
//! These tests run against the Cosmos DB emulator.

use super::framework;

use azure_core::{http::StatusCode, Uuid};
use azure_data_cosmos::fault_injection::{
    FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
    FaultInjectionRule, FaultInjectionRuleBuilder, FaultOperationType, TransportKind,
};
use azure_data_cosmos::models::{ContainerProperties, ThroughputProperties};
use azure_data_cosmos::options::{
    ItemReadOptions, OperationOptionsBuilder, ThrottlingRetryOptionsBuilder,
};
use framework::{TestClient, TestOptions};
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
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
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

    let fault_builder = vec![Arc::new(rule)];

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

            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            // With probability 0.0, all reads should succeed
            for i in 1..=5 {
                let result = fault_container_client.read_item(&pk, &item_id, None).await;
                assert!(
                    result.is_ok(),
                    "read {} should succeed with probability 0.0: {:?}",
                    i,
                    result.err()
                );
            }

            Ok(())
        },
        Some(TestOptions::for_emulator().with_fault_injection_rules(fault_builder)),
    )
    .await
}

/// Test probability fault injection - with probability 1.0, fault should always apply.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
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

    let fault_builder = vec![Arc::new(rule)];

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

            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            // With probability 1.0, all reads should fail
            for i in 1..=5 {
                let result = fault_container_client.read_item(&pk, &item_id, None).await;
                let err =
                    result.expect_err(&format!("read {} should fail with probability 1.0", i));
                assert_eq!(
                    StatusCode::ServiceUnavailable,
                    err.status().status_code(),
                    "read {} should return 503",
                    i
                );
            }

            Ok(())
        },
        Some(TestOptions::for_emulator().with_fault_injection_rules(fault_builder)),
    )
    .await
}

/// Test retry on transient errors with hit_limit.
/// Injects 429 for the first 2 requests, verifies 3rd succeeds.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
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

    let fault_builder = vec![Arc::new(rule)];

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

            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            // First request - should succeed after retries
            let result = fault_container_client.read_item(&pk, &item_id, None).await;
            // Verify the read succeeded
            assert!(
                result.is_ok(),
                "Read should succeed after hit_limit exhausted, but got error: {:?}",
                result.err()
            );

            let response = result.unwrap();
            assert_eq!(response.status(), StatusCode::Ok);

            Ok(())
        },
        Some(TestOptions::for_emulator().with_fault_injection_rules(fault_builder)),
    )
    .await
}

/// Test DeleteItem fault - verify CRUD operations unaffected.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
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

    let fault_builder = vec![Arc::new(rule)];

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
            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            // Read should succeed
            let read_result = fault_container_client.read_item(&pk, &item_id, None).await;
            assert!(
                read_result.is_ok(),
                "read should succeed: {:?}",
                read_result.err()
            );

            // Upsert should succeed
            let mut updated_item = item.clone();
            updated_item.value = 100;
            let upsert_result = fault_container_client
                .upsert_item(&pk, &item_id, &updated_item, None)
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
                StatusCode::ServiceUnavailable,
                err.status().status_code(),
                "delete should return 503 ServiceUnavailable"
            );

            Ok(())
        },
        Some(TestOptions::for_emulator().with_fault_injection_rules(fault_builder)),
    )
    .await
}

/// Test container-specific fault - verify other containers unaffected.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
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

    let fault_builder = vec![Arc::new(rule)];

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

            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            // Read should succeed since container name doesn't match "FaultyContainer"
            let result = fault_container_client.read_item(&pk, &item_id, None).await;

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
            let faulty_fault_container_client = fault_db_client
                .container_client(faulty_container_id)
                .await?;
            let faulty_result = faulty_fault_container_client
                .read_item(&pk, &item_id, None)
                .await;

            let err = faulty_result
                .expect_err("read should fail for container matching 'FaultyContainer'");
            assert_eq!(
                StatusCode::ServiceUnavailable,
                err.status().status_code(),
                "expected 503 ServiceUnavailable for FaultyContainer"
            );

            Ok(())
        },
        Some(TestOptions::for_emulator().with_fault_injection_rules(fault_builder)),
    )
    .await
}

/// Test multiple rules priority - first matching rule wins.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
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

    let fault_builder = vec![Arc::new(rule1), Arc::new(rule2)];

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

            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            let read_options = ItemReadOptions::default().with_operation_options(
                OperationOptionsBuilder::new()
                    .with_throttling_retry_options(
                        ThrottlingRetryOptionsBuilder::new()
                            .with_max_retry_count(0)
                            .build(),
                    )
                    .build(),
            );
            let result = fault_container_client
                .read_item(&pk, &item_id, Some(read_options))
                .await;

            // Should get 429 (first rule), not 503 (second rule)
            let err = result.expect_err("expected first rule (429) to apply");
            assert_eq!(
                StatusCode::TooManyRequests,
                err.status().status_code(),
                "first matching rule should win (429, not 503)"
            );

            Ok(())
        },
        Some(TestOptions::for_emulator().with_fault_injection_rules(fault_builder)),
    )
    .await
}

/// Test that first rule is skipped because its start_time is in the future.
/// Second rule applies immediately and should win.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
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

    let fault_builder = vec![Arc::new(rule1), Arc::new(rule2)];

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

            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            let result = fault_container_client.read_item(&pk, &item_id, None).await;

            // Should get 503 (second rule) because first rule hasn't started yet
            let err = result.expect_err("expected second rule (503) to apply");
            assert_eq!(
                StatusCode::ServiceUnavailable,
                err.status().status_code(),
                "second rule should apply (503) since first rule has not started"
            );

            Ok(())
        },
        Some(TestOptions::for_emulator().with_fault_injection_rules(fault_builder)),
    )
    .await
}

/// Test that first rule is expired because its end_time is in the past.
/// Second rule should win.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
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

    let fault_builder = vec![Arc::new(rule1), Arc::new(rule2)];

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

            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            // Small delay to ensure duration has passed
            tokio::time::sleep(Duration::from_millis(100)).await;

            let result = fault_container_client.read_item(&pk, &item_id, None).await;

            // Should get 503 (second rule) because first rule's duration has expired
            let err = result.expect_err("expected second rule (503) to apply");
            assert_eq!(
                StatusCode::ServiceUnavailable,
                err.status().status_code(),
                "second rule should apply (503) since first rule's end_time has passed"
            );

            Ok(())
        },
        Some(TestOptions::for_emulator().with_fault_injection_rules(fault_builder)),
    )
    .await
}

/// Test hit_limit behavior - fault stops after N applications.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn fault_injection_hit_limit_behavior() -> Result<(), Box<dyn Error>> {
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::InternalServerError)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    // The driver retries 500 errors internally (up to 3 failover retries per call),
    // so each read_item call consumes up to 4 fault injection hits. Setting
    // hit_limit to 8 ensures 2 calls fail completely before the limit is exhausted.
    let rule = FaultInjectionRuleBuilder::new("hit-limit-test", server_error)
        .with_condition(condition)
        .with_hit_limit(8)
        .build();

    let fault_builder = vec![Arc::new(rule)];

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

            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            // First 2 requests should fail with one in region retry
            for i in 1..=2 {
                let result = fault_container_client.read_item(&pk, &item_id, None).await;
                assert!(
                    result.is_err(),
                    "request {} should fail (within hit_limit)",
                    i
                );
                assert_eq!(
                    StatusCode::InternalServerError,
                    result.unwrap_err().status().status_code()
                );
            }

            // After hit_limit is exhausted by retries, the next read should succeed
            let result = run_context
                .read_item(&fault_container_client, &pk, &item_id, None)
                .await;
            assert!(
                result.is_ok(),
                "request 3 should succeed after hit_limit exhausted: {:?}",
                result.err()
            );

            Ok(())
        },
        Some(TestOptions::for_emulator().with_fault_injection_rules(fault_builder)),
    )
    .await
}

/// Test empty rules - no fault injection, operations should succeed.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn fault_injection_empty_rules() -> Result<(), Box<dyn Error>> {
    let fault_builder: Vec<Arc<FaultInjectionRule>> = Vec::new();

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

            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            // Read should succeed with no fault rules
            let result = fault_container_client.read_item(&pk, &item_id, None).await;

            assert!(
                result.is_ok(),
                "read should succeed with empty fault rules: {:?}",
                result.err()
            );

            Ok(())
        },
        Some(TestOptions::for_emulator().with_fault_injection_rules(fault_builder)),
    )
    .await
}

/// Test that item operations succeed when metadata operations are faulted.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
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
    let fault_builder = vec![rule];

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
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

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

            let create_result = fault_container_client
                .create_item(&pk, &item_id, &item, None)
                .await;
            assert!(
                create_result.is_ok(),
                "create item should succeed: {:?}",
                create_result.err()
            );

            // Read item should succeed (use run_context.read_item for replication retry)
            let read_result = run_context
                .read_item(&fault_container_client, &pk, &item_id, None)
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
                .upsert_item(&pk, &item_id, &updated_item, None)
                .await;
            assert!(
                upsert_result.is_ok(),
                "upsert item should succeed: {:?}",
                upsert_result.err()
            );

            Ok(())
        },
        Some(TestOptions::for_emulator().with_fault_injection_rules(fault_builder)),
    )
    .await
}

/// Test that disabling a rule at runtime prevents fault injection,
/// and re-enabling it resumes injection.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
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

    assert_eq!(rule.id(), "enable-disable-test");
    assert!(rule.is_enabled());

    let rule_handle = Arc::clone(&rule);

    let fault_builder = vec![rule];

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

            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            // Rule is enabled — read should fail
            let result = fault_container_client.read_item(&pk, &item_id, None).await;
            assert!(result.is_err(), "read should fail while rule is enabled");

            // Disable the rule at runtime
            rule_handle.disable();
            assert!(!rule_handle.is_enabled());

            // Read should now succeed
            let result = fault_container_client.read_item(&pk, &item_id, None).await;
            assert!(
                result.is_ok(),
                "read should succeed after disabling rule: {:?}",
                result.err()
            );

            // Re-enable the rule
            rule_handle.enable();
            assert!(rule_handle.is_enabled());

            // Read should fail again
            let result = fault_container_client.read_item(&pk, &item_id, None).await;
            assert!(result.is_err(), "read should fail after re-enabling rule");

            Ok(())
        },
        Some(TestOptions::for_emulator().with_fault_injection_rules(fault_builder)),
    )
    .await
}

// ----------------------------------------------------------------------------
// Gateway 2.0 fault injection coverage
// ----------------------------------------------------------------------------

/// Gateway 2.0 ConnectionError on every region must surface a connection
/// failure to the caller — Rust does **not** silently fall back to the
/// standard gateway. `ConnectionError` models any failure to establish the
/// connection (DNS, TLS, a blocked port, or network-unreachable all look the
/// same to the client), so the failure stays visible to the operator instead
/// of being masked. The fault-injection rule matches every `ReadItem`
/// regardless of transport (a prior `with_transport_kind` filter caused
/// false-negatives on some accounts), while the Gateway 2.0-specific routing
/// behavior is asserted at the unit level in the driver's gateway_v2 pipeline
/// tests.
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "gateway_v2",
        test_category = "gateway_v2_multi_region"
    )),
    ignore = "requires test_category 'gateway_v2'"
)]
pub async fn gateway_v2_connection_error_fails_fast_after_all_regions_attempted(
) -> Result<(), Box<dyn Error>> {
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ConnectionError)
        .with_probability(1.0)
        .build();

    // No transport_kind or region filter: the rule injects a connection error
    // on every region (probability 1.0) to exercise the "all regions attempted,
    // then fail fast" contract. The condition builder does support
    // `with_region(...)` scoping, but a single-region variant would instead test
    // failover (covered separately). The asserted contract here is that
    // connection errors on every region surface to the caller rather than
    // silently falling back to the standard gateway.
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let rule = FaultInjectionRuleBuilder::new("gateway_v2-conn-error-fail-fast", server_error)
        .with_condition(condition)
        .build();

    let fault_builder = vec![Arc::new(rule)];

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

            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            // The rule fires on every Gateway 2.0 attempt across every
            // region. With fail-fast semantics, the read must surface the
            // connection error rather than silently retry on the standard
            // gateway.
            let result = fault_container_client.read_item(&pk, &item_id, None).await;
            let err = result.expect_err(
                "read must fail fast after Gateway 2.0 connection errors on every region",
            );
            let status = err.status();
            assert!(
                status.sub_status()
                    == Some(azure_data_cosmos_driver::SubStatusCode::TRANSPORT_IO_FAILED)
                    || status.sub_status()
                        == Some(
                            azure_data_cosmos_driver::SubStatusCode::TRANSPORT_CONNECTION_FAILED
                        )
                    || err.to_string().to_lowercase().contains("connection"),
                "expected a connection-failure error, got: {err:?}"
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_injection_rules(fault_builder)),
    )
    .await
}

// =========================================================================
// Diagnostics-on-error-path coverage
// =========================================================================
//
// These tests use fault injection to deterministically drive operations
// into the error path, then assert that `CosmosError::diagnostics()`
// surfaces the rich per-operation context the SDK promises (ActivityId,
// retry / per-attempt history, transport shard, region, final status,
// fault-injection evaluations). They guard the contract that callers
// can do post-mortem analysis on a failed operation without re-running it.

/// A read forced to fail with 503 must surface diagnostics whose
/// per-attempt request history records the SDK's cross-region retries
/// before giving up. Validates `CosmosError::diagnostics()` is `Some`,
/// `request_count() >= 2`, every request has a real ActivityId, and the
/// final operation status matches the injected fault.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn error_diagnostics_records_retry_history() -> Result<(), Box<dyn Error>> {
    let always_503 = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("retry-history-503", always_503)
            .with_condition(condition)
            .build(),
    );
    let fault_builder = vec![Arc::clone(&rule)];

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
            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            let err = fault_container_client
                .read_item(&pk, &item_id, None)
                .await
                .expect_err("503 fault must drive the read into the error path");

            // CosmosError exposes the operation's final HTTP status via the
            // attached CosmosStatus (which is sourced from diagnostics).
            assert_eq!(
                u16::from(err.status().status_code()),
                503,
                "CosmosError::status() should reflect the injected 503"
            );
            assert_eq!(
                err.status().status_code(),
                StatusCode::ServiceUnavailable,
                "CosmosError::status() should be StatusCode::ServiceUnavailable"
            );

            // Diagnostics carrier survives end-to-end.
            let diag = err
                .diagnostics()
                .expect("CosmosError must carry diagnostics on the fault-injected error path");

            // The retry pipeline must have made multiple attempts before
            // exhausting its budget. We don't pin an exact number — the
            // pipeline's cross-region failover count is configuration-
            // dependent — but at minimum the initial attempt plus one
            // retry should be recorded.
            assert!(
                diag.request_count() >= 2,
                "diagnostics must record at least 2 request attempts on the retry path, got {}",
                diag.request_count()
            );

            // Final operation status is the 503 we injected.
            let final_status = diag
                .status()
                .expect("diagnostics must record the final operation status");
            assert_eq!(
                u16::from(final_status.status_code()),
                503,
                "diagnostics final status_code should be 503, got {:?}",
                final_status.status_code()
            );

            // Top-level ActivityId is populated.
            assert!(
                !diag.activity_id().as_str().is_empty(),
                "diagnostics activity_id must be non-empty"
            );

            // Per-attempt records carry their own per-request metadata.
            let requests = diag.requests();
            assert!(
                !requests.is_empty(),
                "diagnostics.requests() must be non-empty"
            );
            let mut injected_503_attempts = 0usize;
            for (i, req) in requests.iter().enumerate() {
                let status = req.status();
                let code = u16::from(status.status_code());
                if code == 503 {
                    injected_503_attempts += 1;
                }
                // Every recorded attempt either reflects the injected 503
                // or a transient transport-layer probe (status 0). Other
                // status codes here would mean we're recording attempts
                // that don't match what fault injection actually did.
                assert!(
                    code == 503 || code == 0,
                    "attempt {i} status should reflect injected 503 or be a transport probe, got {code}"
                );
            }
            // The whole point of this test is to prove the diagnostics
            // history captured the injected failures — not just a stream
            // of transport probes. At least one attempt MUST carry the
            // injected 503 status.
            assert!(
                injected_503_attempts > 0,
                "at least one recorded attempt must reflect the injected 503; \
                 recorded statuses: {:?}",
                requests
                    .iter()
                    .map(|r| u16::from(r.status().status_code()))
                    .collect::<Vec<_>>()
            );

            // Duration is captured. Compare against `Duration::ZERO` rather
            // than `.as_millis() > 0` so sub-millisecond retry loops still
            // assert correctly.
            assert!(
                diag.duration() > Duration::ZERO,
                "diagnostics.duration() should be > 0 after retries"
            );

            // JSON projection includes the activity_id — guards the
            // serialize-for-ADX path callers rely on for post-mortem.
            let json = diag.to_json_string(None);
            assert!(
                json.contains("\"activity_id\""),
                "diagnostics JSON must include activity_id; got: {json}"
            );

            // The rule must have actually fired (defensive sanity check).
            assert!(
                rule.hit_count() >= 1,
                "fault injection rule should have been hit at least once, got {}",
                rule.hit_count()
            );

            Ok(())
        },
        Some(TestOptions::for_emulator().with_fault_injection_rules(fault_builder)),
    )
    .await
}

// ── 449 RetryWith policy — live SDK coverage ─────────────────────────────────
//
// 449 RetryWith signals a transient same-region concurrency conflict that the
// driver retries internally (in `try_handle_retry_with`). These tests exercise
// the policy from the SDK surface (`ContainerClient::read_item`) so a regression
// that disables the driver-side retry surfaces as a user-visible 449.

/// Test that 449 RetryWith faults are transparently retried by the driver, so
/// the SDK caller sees `Ok` once the rule's hit-limit is exhausted. Mirrors
/// `fault_injection_429_retry_with_hit_limit`, differing only in the error type
/// since both 429 and 449 trigger a driver-side retry without reaching the
/// caller.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn fault_injection_449_retry_with_hit_limit() -> Result<(), Box<dyn Error>> {
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::RetryWith)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let rule = FaultInjectionRuleBuilder::new("449-retry-with-limit", server_error)
        .with_condition(condition)
        .with_hit_limit(2)
        .build();

    let fault_builder = vec![Arc::new(rule)];

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

            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            // The driver retries 449 in-region; with hit_limit=2 the rule
            // fires twice and the third attempt succeeds because the rule
            // is then disabled.
            let result = fault_container_client.read_item(&pk, &item_id, None).await;
            assert!(
                result.is_ok(),
                "read_item should succeed after the 449 hit_limit is exhausted; got error: {:?}",
                result.err()
            );
            assert_eq!(result.unwrap().status(), StatusCode::Ok);

            Ok(())
        },
        Some(TestOptions::new().with_fault_injection_rules(fault_builder)),
    )
    .await
}

/// Validates that fault-injection metadata propagates into the error-path
/// diagnostics: `fault_injection_enabled()` is true on the operation
/// context, and at least one attempt's `fault_injection_evaluations()`
/// records the `Applied` evaluation for the configured rule.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn error_diagnostics_includes_fault_injection_evaluations() -> Result<(), Box<dyn Error>>
{
    use azure_data_cosmos_driver::fault_injection::FaultInjectionEvaluation;

    let always_503 = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("fi-eval-applied", always_503)
            .with_condition(condition)
            .build(),
    );
    let fault_builder = vec![Arc::clone(&rule)];

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
            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            let err = fault_container_client
                .read_item(&pk, &item_id, None)
                .await
                .expect_err("503 fault must drive the read into the error path");

            let diag = err
                .diagnostics()
                .expect("CosmosError must carry diagnostics");

            assert!(
                diag.fault_injection_enabled(),
                "diagnostics must mark the operation as fault-injection-enabled"
            );

            let has_applied = diag.requests().iter().any(|req| {
                req.fault_injection_evaluations().iter().any(|e| {
                    matches!(
                        e,
                        FaultInjectionEvaluation::Applied { rule_id }
                            if rule_id == "fi-eval-applied"
                    )
                })
            });
            assert!(
                has_applied,
                "at least one request attempt must record the `Applied` evaluation for rule 'fi-eval-applied'"
            );

            // Negative control: the recorded `Applied` evaluations must
            // be consistent with the rule's hit counter. Without this
            // cross-check the assertion above passes for any non-zero
            // hit_count, even if attempts and evaluations are mis-aligned.
            let applied_in_diagnostics = diag
                .requests()
                .iter()
                .flat_map(|req| req.fault_injection_evaluations().iter())
                .filter(|e| {
                    matches!(
                        e,
                        FaultInjectionEvaluation::Applied { rule_id }
                            if rule_id == "fi-eval-applied"
                    )
                })
                .count();
            assert_eq!(
                applied_in_diagnostics as u32,
                rule.hit_count(),
                "diagnostics-recorded `Applied` count ({}) must match the rule's hit_count ({}); \
                 a mismatch means some applications fired without being recorded in per-attempt diagnostics",
                applied_in_diagnostics,
                rule.hit_count()
            );

            Ok(())
        },
        Some(TestOptions::for_emulator().with_fault_injection_rules(fault_builder)),
    )
    .await
}

/// Gateway 2.0 449 RetryWith — like `fault_injection_449_retry_with_hit_limit`,
/// but scoped to the Gateway 2.0 transport.
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "gateway_v2",
        test_category = "gateway_v2_multi_region"
    )),
    ignore = "requires test_category 'gateway_v2'"
)]
pub async fn gateway_v2_449_retry_with_hit_limit() -> Result<(), Box<dyn Error>> {
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::RetryWith)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_transport_kind(TransportKind::GatewayV2)
        .build();

    let rule = FaultInjectionRuleBuilder::new("gateway_v2-449-retry-with-limit", server_error)
        .with_condition(condition)
        .with_hit_limit(2)
        .build();

    let fault_builder = vec![Arc::new(rule)];

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

            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            // The driver retries 449 in-region; with hit_limit=2 the rule
            // fires twice on Gateway 2.0 traffic and the third attempt
            // succeeds because the rule is then disabled.
            let result = fault_container_client.read_item(&pk, &item_id, None).await;
            assert!(
                result.is_ok(),
                "read_item should succeed after Gateway 2.0 449 hit_limit is exhausted; got error: {:?}",
                result.err()
            );
            assert_eq!(result.unwrap().status(), StatusCode::Ok);

            Ok(())
        },
        Some(TestOptions::new().with_fault_injection_rules(fault_builder)),
    )
    .await
}

/// Cross-checks that the `CosmosError` accessors (`status`, `response`,
/// `diagnostics`) agree with the inner `DiagnosticsContext` on the
/// fault-injected error path. Guards against accidental divergence between
/// the two surfaces.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn cosmos_error_accessors_match_diagnostics_after_fault() -> Result<(), Box<dyn Error>> {
    let always_503 = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("accessor-consistency", always_503)
            .with_condition(condition)
            .build(),
    );
    let fault_builder = vec![Arc::clone(&rule)];

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
            container_client
                .create_item(&pk, &item_id, &item, None)
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id).await?;

            let err = fault_container_client
                .read_item(&pk, &item_id, None)
                .await
                .expect_err("read must fail under fault injection");

            // diagnostics() is Some on the fault-injected error path.
            let diag = err
                .diagnostics()
                .expect("CosmosError must carry diagnostics on the fault-injected error path");

            // CosmosError::status() agrees with diagnostics.status() — the
            // two surfaces are derived from the same final-attempt record
            // and must not diverge.
            let diag_status = diag
                .status()
                .expect("diagnostics must have a final status after a failure");
            assert_eq!(
                u16::from(err.status().status_code()),
                u16::from(diag_status.status_code()),
                "CosmosError::status() must equal diagnostics.status().status_code()"
            );

            // The error must reflect the wire response (a real 503 came
            // back from the service-side fault), not a synthetic
            // client-side failure.
            assert!(
                err.response().is_some(),
                "CosmosError must carry a CosmosResponse for a service-side 503"
            );

            Ok(())
        },
        Some(TestOptions::for_emulator().with_fault_injection_rules(fault_builder)),
    )
    .await
}
