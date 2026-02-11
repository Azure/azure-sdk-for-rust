// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration tests for fault injection framework.
//! These tests run against the Cosmos DB emulator.

#![cfg(feature = "key_auth")]
#![cfg(feature = "fault_injection")]

use super::framework;

use azure_core::http::StatusCode;
use azure_data_cosmos::fault_injection::{
    FaultInjectionClientBuilder, FaultInjectionConditionBuilder, FaultInjectionErrorType,
    FaultInjectionResultBuilder, FaultInjectionRuleBuilder, FaultOperationType,
};
use azure_data_cosmos::models::{ContainerProperties, ThroughputProperties};
use azure_data_cosmos::CosmosClientOptions;
use framework::{
    get_effective_hub_endpoint, TestClient, TestOptions, HUB_REGION, SATELLITE_REGION,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::error::Error;
use std::sync::Arc;
use std::time::{Duration, Instant};
use uuid::Uuid;

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
    let fault_options = fault_builder.inject(CosmosClientOptions::default());

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
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
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

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
        Some(TestOptions::new().with_fault_client_options(fault_options)),
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
    let fault_options = fault_builder.inject(CosmosClientOptions::default());

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
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
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

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
        Some(TestOptions::new().with_fault_client_options(fault_options)),
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

    let client_options = CosmosClientOptions {
        application_preferred_regions: vec![HUB_REGION, SATELLITE_REGION],
        ..Default::default()
    };
    let fault_options = fault_builder.inject(client_options);

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
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
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

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
                response
                    .request()
                    .clone()
                    .into_raw_request()
                    .url()
                    .host_str()
                    .unwrap(),
                get_effective_hub_endpoint()
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_client_options(fault_options)),
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
    let fault_options = fault_builder.inject(CosmosClientOptions::default());

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
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
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

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
        Some(TestOptions::new().with_fault_client_options(fault_options)),
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
    let fault_options = fault_builder.inject(CosmosClientOptions::default());

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            // Create a container that doesn't match the fault condition
            let container_id = format!("SafeContainer-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
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
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

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
                    ContainerProperties {
                        id: faulty_container_id.into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
                    ThroughputProperties::manual(400),
                )
                .await?;

            // Now try to read using the fault client - should fail because container name contains "FaultyContainer"
            let faulty_fault_container_client =
                fault_db_client.container_client(&faulty_container_id);
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
        Some(TestOptions::new().with_fault_client_options(fault_options)),
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
    let fault_options = fault_builder.inject(CosmosClientOptions::default());

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
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
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

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
        Some(TestOptions::new().with_fault_client_options(fault_options)),
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
    let fault_options = fault_builder.inject(CosmosClientOptions::default());

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
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
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

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
        Some(TestOptions::new().with_fault_client_options(fault_options)),
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
    let fault_options = fault_builder.inject(CosmosClientOptions::default());

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
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
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

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
        Some(TestOptions::new().with_fault_client_options(fault_options)),
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
        .with_hit_limit(3)
        .build();

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::new(rule));
    let fault_options = fault_builder.inject(CosmosClientOptions::default());

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
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
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

            // First 3 requests should fail
            for i in 1..=3 {
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
        Some(TestOptions::new().with_fault_client_options(fault_options)),
    )
    .await
}

/// Test empty rules - no fault injection, operations should succeed.
#[tokio::test]
pub async fn fault_injection_empty_rules() -> Result<(), Box<dyn Error>> {
    let fault_builder = FaultInjectionClientBuilder::new();
    let fault_options = fault_builder.inject(CosmosClientOptions::default());

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
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
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

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
        Some(TestOptions::new().with_fault_client_options(fault_options)),
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

    let rule = FaultInjectionRuleBuilder::new("metadata-fails", server_error)
        .with_condition(condition)
        .build();

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::new(rule));
    let fault_options = fault_builder.inject(CosmosClientOptions::default());

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
                    ThroughputProperties::manual(400),
                )
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

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
        Some(TestOptions::new().with_fault_client_options(fault_options)),
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
    let fault_options = fault_builder.inject(CosmosClientOptions::default());

    TestClient::run_with_unique_db(
        async move |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
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
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

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
        Some(TestOptions::new().with_fault_client_options(fault_options)),
    )
    .await
}

// =============================================================================
// PPAF (Per-Partition Automatic Failover) Tests
// =============================================================================

/// PPAF: WriteForbidden (403/3) on write operations triggers retry and eventually succeeds.
///
/// This test validates the PPAF retry path: when a write request receives a 403/3
/// (WriteForbidden) response, the retry policy detects this as a partition-level
/// write failure and attempts to fail the write over to a different endpoint.
/// With a hit_limit, the fault stops after N applications, allowing the write to succeed.
#[tokio::test]
pub async fn ppaf_write_forbidden_on_create_retries_and_succeeds() -> Result<(), Box<dyn Error>> {
    let write_forbidden_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::WriteForbidden)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .build();

    // hit_limit=2: first 2 create attempts get 403/3, third succeeds
    let rule = FaultInjectionRuleBuilder::new("ppaf-write-forbidden-create", write_forbidden_error)
        .with_condition(condition)
        .with_hit_limit(2)
        .build();

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::new(rule));

    let client_options = CosmosClientOptions {
        application_preferred_regions: vec![HUB_REGION, SATELLITE_REGION],
        ..Default::default()
    };
    let fault_options = fault_builder.inject(client_options);

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
                    ThroughputProperties::manual(400),
                )
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

            // Create item using fault client - should succeed after retries exhaust the hit_limit
            let unique_id = Uuid::new_v4().to_string();
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            let create_result = fault_container_client.create_item(&pk, &item, None).await;
            assert!(
                create_result.is_ok(),
                "create should succeed after WriteForbidden retries exhaust hit_limit: {:?}",
                create_result.err()
            );

            // Verify the item was actually created by reading it back
            let read_result = run_context
                .read_item::<TestItem>(&container_client, &pk, &item_id, None)
                .await;
            assert!(
                read_result.is_ok(),
                "read should succeed for the created item: {:?}",
                read_result.err()
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_client_options(fault_options)),
    )
    .await
}

/// PPAF: WriteForbidden on writes does not affect read operations.
///
/// Validates that a WriteForbidden fault rule scoped to CreateItem operations
/// does not interfere with ReadItem operations on the same container.
#[tokio::test]
pub async fn ppaf_write_forbidden_does_not_affect_reads() -> Result<(), Box<dyn Error>> {
    let write_forbidden_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::WriteForbidden)
        .build();

    // Only target CreateItem — reads should be unaffected
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .build();

    let rule = FaultInjectionRuleBuilder::new("ppaf-write-only-fault", write_forbidden_error)
        .with_condition(condition)
        .build();

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::new(rule));

    let client_options = CosmosClientOptions {
        application_preferred_regions: vec![HUB_REGION, SATELLITE_REGION],
        ..Default::default()
    };
    let fault_options = fault_builder.inject(client_options);

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
                    ThroughputProperties::manual(400),
                )
                .await?;

            // Create an item using the normal client (no fault injection)
            let unique_id = Uuid::new_v4().to_string();
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

            // Reads through the fault client should succeed — WriteForbidden only targets writes
            let read_result = run_context
                .read_item::<TestItem>(&fault_container_client, &pk, &item_id, None)
                .await;
            assert!(
                read_result.is_ok(),
                "read should succeed despite WriteForbidden rule on writes: {:?}",
                read_result.err()
            );

            let response = read_result.unwrap();
            assert_eq!(response.status(), StatusCode::Ok);

            Ok(())
        },
        Some(TestOptions::new().with_fault_client_options(fault_options)),
    )
    .await
}

/// PPAF: WriteForbidden on upsert triggers retry and eventually succeeds.
///
/// Similar to the create test, this validates that UpsertItem operations
/// also trigger the PPAF retry path on WriteForbidden errors.
#[tokio::test]
pub async fn ppaf_write_forbidden_on_upsert_retries_and_succeeds() -> Result<(), Box<dyn Error>> {
    let write_forbidden_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::WriteForbidden)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::UpsertItem)
        .build();

    let rule = FaultInjectionRuleBuilder::new("ppaf-write-forbidden-upsert", write_forbidden_error)
        .with_condition(condition)
        .with_hit_limit(2)
        .build();

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::new(rule));

    let client_options = CosmosClientOptions {
        application_preferred_regions: vec![HUB_REGION, SATELLITE_REGION],
        ..Default::default()
    };
    let fault_options = fault_builder.inject(client_options);

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
                    ThroughputProperties::manual(400),
                )
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

            // Upsert item using fault client — should succeed after retries exhaust the hit_limit
            let unique_id = Uuid::new_v4().to_string();
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            let upsert_result = fault_container_client.upsert_item(&pk, &item, None).await;
            assert!(
                upsert_result.is_ok(),
                "upsert should succeed after WriteForbidden retries exhaust hit_limit: {:?}",
                upsert_result.err()
            );

            // Verify item was persisted by reading it back via normal client
            let read_result = run_context
                .read_item::<TestItem>(&container_client, &pk, &item_id, None)
                .await;
            assert!(
                read_result.is_ok(),
                "read should succeed for the upserted item: {:?}",
                read_result.err()
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_client_options(fault_options)),
    )
    .await
}

/// PPAF: WriteForbidden failback — disabling the fault rule at runtime
/// simulates the original region recovering, and subsequent writes succeed immediately.
#[tokio::test]
pub async fn ppaf_write_forbidden_failback_after_rule_disabled() -> Result<(), Box<dyn Error>> {
    let write_forbidden_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::WriteForbidden)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("ppaf-failback", write_forbidden_error)
            .with_condition(condition)
            .build(),
    );

    let rule_handle = Arc::clone(&rule);

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(rule);

    let client_options = CosmosClientOptions {
        application_preferred_regions: vec![HUB_REGION, SATELLITE_REGION],
        ..Default::default()
    };
    let fault_options = fault_builder.inject(client_options);

    TestClient::run_with_unique_db(
        async move |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
                    ThroughputProperties::manual(400),
                )
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

            // Phase 1: Fault active — create should fail with WriteForbidden
            let unique_id1 = Uuid::new_v4().to_string();
            let item1 = create_test_item(&unique_id1);
            let pk1 = format!("Partition-{}", unique_id1);

            let result = fault_container_client.create_item(&pk1, &item1, None).await;
            let err = result.expect_err("create should fail while WriteForbidden rule is active");
            assert_eq!(
                Some(StatusCode::Forbidden),
                err.http_status(),
                "expected 403 Forbidden from WriteForbidden fault"
            );

            // Phase 2: Simulate failback — disable the fault rule (region recovered)
            rule_handle.disable();
            assert!(!rule_handle.is_enabled());

            // Create should now succeed (failback to original region)
            let unique_id2 = Uuid::new_v4().to_string();
            let item2 = create_test_item(&unique_id2);
            let pk2 = format!("Partition-{}", unique_id2);
            let item_id2 = format!("Item-{}", unique_id2);

            let create_result = fault_container_client.create_item(&pk2, &item2, None).await;
            assert!(
                create_result.is_ok(),
                "create should succeed after disabling WriteForbidden rule (failback): {:?}",
                create_result.err()
            );

            // Verify item was actually created
            let read_result = run_context
                .read_item::<TestItem>(&container_client, &pk2, &item_id2, None)
                .await;
            assert!(
                read_result.is_ok(),
                "read should confirm the item exists: {:?}",
                read_result.err()
            );

            // Phase 3: Re-enable fault (simulate another failover event)
            rule_handle.enable();
            assert!(rule_handle.is_enabled());

            let unique_id3 = Uuid::new_v4().to_string();
            let item3 = create_test_item(&unique_id3);
            let pk3 = format!("Partition-{}", unique_id3);

            let result = fault_container_client.create_item(&pk3, &item3, None).await;
            assert!(
                result.is_err(),
                "create should fail again after re-enabling WriteForbidden rule"
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_client_options(fault_options)),
    )
    .await
}

// =============================================================================
// PPCB (Per-Partition Circuit Breaker) Tests
// =============================================================================

/// PPCB: ServiceUnavailable (503) on reads with circuit breaker enabled triggers
/// retry and eventually succeeds after hit_limit is exhausted.
///
/// When `enable_partition_level_circuit_breaker` is set, the retry policy tracks
/// consecutive failures per partition key range. After exceeding the threshold,
/// it marks the endpoint unavailable for that partition and fails over to
/// an alternate read endpoint. With a hit_limit, the fault expires and reads succeed.
#[tokio::test]
pub async fn ppcb_service_unavailable_read_retries_and_succeeds() -> Result<(), Box<dyn Error>> {
    let service_unavailable = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    // hit_limit=3: first 3 read attempts get 503, subsequent succeed
    let rule = FaultInjectionRuleBuilder::new("ppcb-503-read", service_unavailable)
        .with_condition(condition)
        .with_hit_limit(3)
        .build();

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::new(rule));

    let client_options = CosmosClientOptions {
        application_preferred_regions: vec![HUB_REGION, SATELLITE_REGION],
        enable_partition_level_circuit_breaker: true,
        ..Default::default()
    };
    let fault_options = fault_builder.inject(client_options);

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
                    ThroughputProperties::manual(400),
                )
                .await?;

            // Create item using normal client
            let unique_id = Uuid::new_v4().to_string();
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

            // Read should eventually succeed — the retry policy will exhaust the
            // hit_limit and the circuit breaker path will be exercised
            let read_result = run_context
                .read_item::<TestItem>(&fault_container_client, &pk, &item_id, None)
                .await;
            assert!(
                read_result.is_ok(),
                "read should succeed after ServiceUnavailable hit_limit exhausted with PPCB: {:?}",
                read_result.err()
            );

            let response = read_result.unwrap();
            assert_eq!(response.status(), StatusCode::Ok);

            Ok(())
        },
        Some(TestOptions::new().with_fault_client_options(fault_options)),
    )
    .await
}

/// PPCB: ServiceUnavailable (503) on writes with circuit breaker enabled triggers
/// retry and eventually succeeds.
///
/// For write operations, the circuit breaker has a higher consecutive failure threshold
/// (typically >5 for writes vs >2 for reads). This test validates that the retry path
/// handles ServiceUnavailable on writes correctly when PPCB is enabled.
#[tokio::test]
pub async fn ppcb_service_unavailable_write_retries_and_succeeds() -> Result<(), Box<dyn Error>> {
    let service_unavailable = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .build();

    // hit_limit=2: first 2 create attempts get 503, subsequent succeed
    let rule = FaultInjectionRuleBuilder::new("ppcb-503-write", service_unavailable)
        .with_condition(condition)
        .with_hit_limit(2)
        .build();

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::new(rule));

    let client_options = CosmosClientOptions {
        application_preferred_regions: vec![HUB_REGION, SATELLITE_REGION],
        enable_partition_level_circuit_breaker: true,
        ..Default::default()
    };
    let fault_options = fault_builder.inject(client_options);

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
                    ThroughputProperties::manual(400),
                )
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

            // Create item using fault client — should succeed after retries
            let unique_id = Uuid::new_v4().to_string();
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            let create_result = fault_container_client.create_item(&pk, &item, None).await;
            assert!(
                create_result.is_ok(),
                "create should succeed after ServiceUnavailable hit_limit exhausted with PPCB: {:?}",
                create_result.err()
            );

            // Verify item was persisted
            let read_result = run_context
                .read_item::<TestItem>(&container_client, &pk, &item_id, None)
                .await;
            assert!(
                read_result.is_ok(),
                "read should confirm the created item exists: {:?}",
                read_result.err()
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_client_options(fault_options)),
    )
    .await
}

/// PPCB: Failback scenario — after disabling the ServiceUnavailable fault rule,
/// reads recover immediately (simulating the partition becoming healthy again).
#[tokio::test]
pub async fn ppcb_service_unavailable_failback_after_rule_disabled() -> Result<(), Box<dyn Error>> {
    let service_unavailable = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("ppcb-failback", service_unavailable)
            .with_condition(condition)
            .build(),
    );

    let rule_handle = Arc::clone(&rule);

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(rule);

    let client_options = CosmosClientOptions {
        application_preferred_regions: vec![HUB_REGION, SATELLITE_REGION],
        enable_partition_level_circuit_breaker: true,
        ..Default::default()
    };
    let fault_options = fault_builder.inject(client_options);

    TestClient::run_with_unique_db(
        async move |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
                    ThroughputProperties::manual(400),
                )
                .await?;

            // Create item using normal client
            let unique_id = Uuid::new_v4().to_string();
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

            // Phase 1: Fault active — read should fail with 503
            let result = fault_container_client
                .read_item::<TestItem>(&pk, &item_id, None)
                .await;
            let err = result.expect_err("read should fail while ServiceUnavailable rule is active");
            assert_eq!(
                Some(StatusCode::ServiceUnavailable),
                err.http_status(),
                "expected 503 ServiceUnavailable"
            );

            // Phase 2: Disable fault rule (simulate partition recovery / failback)
            rule_handle.disable();

            let read_result = fault_container_client
                .read_item::<TestItem>(&pk, &item_id, None)
                .await;
            assert!(
                read_result.is_ok(),
                "read should succeed after disabling ServiceUnavailable rule (failback): {:?}",
                read_result.err()
            );

            // Phase 3: Re-enable fault — reads should fail again
            rule_handle.enable();

            let result = fault_container_client
                .read_item::<TestItem>(&pk, &item_id, None)
                .await;
            assert!(
                result.is_err(),
                "read should fail again after re-enabling ServiceUnavailable rule"
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_client_options(fault_options)),
    )
    .await
}

/// PPCB: Circuit breaker disabled — ServiceUnavailable still fails without
/// partition-level failover routing.
///
/// With `enable_partition_level_circuit_breaker: false`, the circuit breaker
/// does not track per-partition failures. ServiceUnavailable errors are still
/// retried through the standard retry policy, but without partition-level routing.
#[tokio::test]
pub async fn ppcb_disabled_service_unavailable_no_circuit_breaker() -> Result<(), Box<dyn Error>> {
    let service_unavailable = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    // No hit_limit — fault applies indefinitely
    let rule = FaultInjectionRuleBuilder::new("ppcb-disabled-503", service_unavailable)
        .with_condition(condition)
        .build();

    let fault_builder = FaultInjectionClientBuilder::new().with_rule(Arc::new(rule));

    let client_options = CosmosClientOptions {
        application_preferred_regions: vec![HUB_REGION, SATELLITE_REGION],
        enable_partition_level_circuit_breaker: false, // explicitly disabled
        ..Default::default()
    };
    let fault_options = fault_builder.inject(client_options);

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
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
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

            // With circuit breaker disabled and no hit_limit, reads should fail with 503
            let result = fault_container_client
                .read_item::<TestItem>(&pk, &item_id, None)
                .await;
            let err = result
                .expect_err("read should fail with circuit breaker disabled and persistent 503");
            assert_eq!(
                Some(StatusCode::ServiceUnavailable),
                err.http_status(),
                "expected 503 without circuit breaker intervention"
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_client_options(fault_options)),
    )
    .await
}

// =============================================================================
// Combined PPAF + PPCB Tests
// =============================================================================

/// Combined: WriteForbidden on writes and ServiceUnavailable on reads
/// with both PPAF retry path and PPCB enabled. Both operation types should
/// recover after their respective fault hit_limits are exhausted.
#[tokio::test]
pub async fn ppaf_ppcb_combined_write_forbidden_and_service_unavailable(
) -> Result<(), Box<dyn Error>> {
    // Rule 1: WriteForbidden (403/3) on CreateItem — PPAF path
    let write_forbidden = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::WriteForbidden)
        .build();
    let write_condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .build();
    let write_rule = FaultInjectionRuleBuilder::new("ppaf-combined-write", write_forbidden)
        .with_condition(write_condition)
        .with_hit_limit(2)
        .build();

    // Rule 2: ServiceUnavailable (503) on ReadItem — PPCB path
    let service_unavailable = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();
    let read_condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();
    let read_rule = FaultInjectionRuleBuilder::new("ppcb-combined-read", service_unavailable)
        .with_condition(read_condition)
        .with_hit_limit(3)
        .build();

    let fault_builder = FaultInjectionClientBuilder::new()
        .with_rule(Arc::new(write_rule))
        .with_rule(Arc::new(read_rule));

    let client_options = CosmosClientOptions {
        application_preferred_regions: vec![HUB_REGION, SATELLITE_REGION],
        enable_partition_level_circuit_breaker: true,
        ..Default::default()
    };
    let fault_options = fault_builder.inject(client_options);

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
                    ThroughputProperties::manual(400),
                )
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

            // Write: should succeed after WriteForbidden retries exhaust hit_limit
            let unique_id = Uuid::new_v4().to_string();
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            let create_result = fault_container_client.create_item(&pk, &item, None).await;
            assert!(
                create_result.is_ok(),
                "create should succeed after WriteForbidden retries: {:?}",
                create_result.err()
            );

            // Read: should succeed after ServiceUnavailable retries exhaust hit_limit
            let read_result = run_context
                .read_item::<TestItem>(&fault_container_client, &pk, &item_id, None)
                .await;
            assert!(
                read_result.is_ok(),
                "read should succeed after ServiceUnavailable retries with PPCB: {:?}",
                read_result.err()
            );

            let response = read_result.unwrap();
            assert_eq!(response.status(), StatusCode::Ok);

            Ok(())
        },
        Some(TestOptions::new().with_fault_client_options(fault_options)),
    )
    .await
}

/// Combined: Full failover-failback lifecycle using enable/disable on both
/// WriteForbidden and ServiceUnavailable rules simultaneously.
///
/// Lifecycle:
/// 1. Both faults active → writes and reads fail
/// 2. Disable both faults (failback) → writes and reads succeed
/// 3. Re-enable only write fault → writes fail, reads still succeed
#[tokio::test]
pub async fn ppaf_ppcb_full_failover_failback_lifecycle() -> Result<(), Box<dyn Error>> {
    let write_forbidden = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::WriteForbidden)
        .build();
    let write_condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .build();
    let write_rule = Arc::new(
        FaultInjectionRuleBuilder::new("lifecycle-write", write_forbidden)
            .with_condition(write_condition)
            .build(),
    );
    let write_handle = Arc::clone(&write_rule);

    let service_unavailable = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();
    let read_condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();
    let read_rule = Arc::new(
        FaultInjectionRuleBuilder::new("lifecycle-read", service_unavailable)
            .with_condition(read_condition)
            .build(),
    );
    let read_handle = Arc::clone(&read_rule);

    let fault_builder = FaultInjectionClientBuilder::new()
        .with_rule(write_rule)
        .with_rule(read_rule);

    let client_options = CosmosClientOptions {
        application_preferred_regions: vec![HUB_REGION, SATELLITE_REGION],
        enable_partition_level_circuit_breaker: true,
        ..Default::default()
    };
    let fault_options = fault_builder.inject(client_options);

    TestClient::run_with_unique_db(
        async move |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
                    ThroughputProperties::manual(400),
                )
                .await?;

            // Create a baseline item using normal client
            let unique_id = Uuid::new_v4().to_string();
            let item = create_test_item(&unique_id);
            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

            // Phase 1: Both faults active — both writes and reads fail
            let write_result = fault_container_client
                .create_item(
                    &pk,
                    &TestItem {
                        id: format!("Item-phase1-{}", Uuid::new_v4()).into(),
                        partition_key: Some(pk.clone().into()),
                        value: 1,
                        nested: NestedItem {
                            nested_value: "phase1".into(),
                        },
                        bool_value: false,
                    },
                    None,
                )
                .await;
            assert!(
                write_result.is_err(),
                "write should fail in phase 1 (WriteForbidden active)"
            );

            let read_result = fault_container_client
                .read_item::<TestItem>(&pk, &item_id, None)
                .await;
            assert!(
                read_result.is_err(),
                "read should fail in phase 1 (ServiceUnavailable active)"
            );

            // Phase 2: Disable both faults (failback) — both succeed
            write_handle.disable();
            read_handle.disable();

            let write_result = fault_container_client
                .create_item(
                    &pk,
                    &TestItem {
                        id: format!("Item-phase2-{}", Uuid::new_v4()).into(),
                        partition_key: Some(pk.clone().into()),
                        value: 2,
                        nested: NestedItem {
                            nested_value: "phase2".into(),
                        },
                        bool_value: true,
                    },
                    None,
                )
                .await;
            assert!(
                write_result.is_ok(),
                "write should succeed in phase 2 (faults disabled): {:?}",
                write_result.err()
            );

            let read_result = fault_container_client
                .read_item::<TestItem>(&pk, &item_id, None)
                .await;
            assert!(
                read_result.is_ok(),
                "read should succeed in phase 2 (faults disabled): {:?}",
                read_result.err()
            );

            // Phase 3: Re-enable only write fault — writes fail, reads succeed
            write_handle.enable();

            let write_result = fault_container_client
                .create_item(
                    &pk,
                    &TestItem {
                        id: format!("Item-phase3-{}", Uuid::new_v4()).into(),
                        partition_key: Some(pk.clone().into()),
                        value: 3,
                        nested: NestedItem {
                            nested_value: "phase3".into(),
                        },
                        bool_value: false,
                    },
                    None,
                )
                .await;
            assert!(
                write_result.is_err(),
                "write should fail in phase 3 (only WriteForbidden re-enabled)"
            );

            let read_result = fault_container_client
                .read_item::<TestItem>(&pk, &item_id, None)
                .await;
            assert!(
                read_result.is_ok(),
                "read should still succeed in phase 3 (ServiceUnavailable still disabled): {:?}",
                read_result.err()
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_client_options(fault_options)),
    )
    .await
}
