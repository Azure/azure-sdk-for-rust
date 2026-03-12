// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration tests for the fault injection framework running against the Cosmos DB emulator.
//!
//! These tests verify that fault injection rules are correctly applied at the HTTP transport layer,
//! producing the expected errors and allowing normal operations when rules are disabled or don't match.
//!
//! Each test uses a **single runtime** with fault rules that start disabled. Rules are enabled
//! after setup operations (database/container/item creation) are complete.

use crate::framework::DriverTestClient;
use azure_core::http::StatusCode;
use azure_data_cosmos_driver::driver::fault_injection::{
    FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
    FaultInjectionRuleBuilder, FaultOperationType,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// A simple test item for fault injection CRUD operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestItem {
    id: String,
    pk: String,
    value: i32,
}

/// With probability 0.0, the fault should never be applied. All reads should succeed.
#[tokio::test]
pub async fn fault_injection_probability_zero_never_fails() -> Result<(), Box<dyn Error>> {
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(0.0)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("probability-zero", result)
            .with_condition(condition)
            .disabled()
            .build(),
    );

    let rule_handle = Arc::clone(&rule);

    DriverTestClient::run_with_fault_injection_and_unique_db(
        vec![rule],
        async move |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            let item = TestItem {
                id: "prob-zero-item".to_string(),
                pk: "pk1".to_string(),
                value: 42,
            };
            let item_json = serde_json::to_vec(&item)?;
            context
                .create_item(&container, &item.id, item.pk.clone(), &item_json)
                .await?;

            // Enable the rule — but probability 0.0 means it never fires
            rule_handle.enable();

            for _ in 0..5 {
                let read_result = context
                    .read_item(&container, &item.id, item.pk.clone())
                    .await;
                assert!(
                    read_result.is_ok(),
                    "read should succeed with probability 0.0: {:?}",
                    read_result.err()
                );
            }

            Ok(())
        },
    )
    .await
}

/// With probability 1.0, every read should fail with 503 ServiceUnavailable.
#[tokio::test]
pub async fn fault_injection_probability_one_always_fails() -> Result<(), Box<dyn Error>> {
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("probability-one", result)
            .with_condition(condition)
            .disabled()
            .build(),
    );

    let rule_handle = Arc::clone(&rule);

    DriverTestClient::run_with_fault_injection_and_unique_db(
        vec![rule],
        async move |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            let item = TestItem {
                id: "prob-one-item".to_string(),
                pk: "pk1".to_string(),
                value: 42,
            };
            let item_json = serde_json::to_vec(&item)?;
            context
                .create_item(&container, &item.id, item.pk.clone(), &item_json)
                .await?;

            // Enable the rule — probability 1.0 means every read fails
            rule_handle.enable();

            for _ in 0..5 {
                let read_result = context
                    .read_item(&container, &item.id, item.pk.clone())
                    .await;
                let err = read_result.expect_err("read should fail with probability 1.0");
                assert_eq!(
                    err.http_status(),
                    Some(StatusCode::ServiceUnavailable),
                    "expected 503 ServiceUnavailable"
                );
            }

            Ok(())
        },
    )
    .await
}

/// Injects 429 with hit_limit=2. The first read should exhaust the fault budget via retries,
/// and the read should eventually succeed.
#[tokio::test]
pub async fn fault_injection_429_retry_with_hit_limit() -> Result<(), Box<dyn Error>> {
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::TooManyRequests)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("throttle-with-limit", result)
            .with_condition(condition)
            .with_hit_limit(2)
            .disabled()
            .build(),
    );

    let rule_handle = Arc::clone(&rule);

    DriverTestClient::run_with_fault_injection_and_unique_db(
        vec![rule],
        async move |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            let item = TestItem {
                id: "throttle-item".to_string(),
                pk: "pk1".to_string(),
                value: 42,
            };
            let item_json = serde_json::to_vec(&item)?;
            context
                .create_item(&container, &item.id, item.pk.clone(), &item_json)
                .await?;

            // Enable the rule
            rule_handle.enable();

            // The first read triggers retries that exhaust the hit_limit, then succeeds
            let read_result = context
                .read_item(&container, &item.id, item.pk.clone())
                .await;
            assert!(
                read_result.is_ok(),
                "read should succeed after hit_limit exhausted: {:?}",
                read_result.err()
            );

            Ok(())
        },
    )
    .await
}

/// Two rules target ReadItem. The first rule (429) should win over the second (503)
/// because rules are evaluated in order.
#[tokio::test]
pub async fn fault_injection_multiple_rules_priority() -> Result<(), Box<dyn Error>> {
    let error1 = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::TooManyRequests)
        .build();
    let condition1 = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();
    let rule1 = Arc::new(
        FaultInjectionRuleBuilder::new("first-rule-429", error1)
            .with_condition(condition1)
            .disabled()
            .build(),
    );

    let error2 = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();
    let condition2 = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();
    let rule2 = Arc::new(
        FaultInjectionRuleBuilder::new("second-rule-503", error2)
            .with_condition(condition2)
            .disabled()
            .build(),
    );

    let rule1_handle = Arc::clone(&rule1);
    let rule2_handle = Arc::clone(&rule2);

    DriverTestClient::run_with_fault_injection_and_unique_db(
        vec![rule1, rule2],
        async move |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            let item = TestItem {
                id: "priority-item".to_string(),
                pk: "pk1".to_string(),
                value: 42,
            };
            let item_json = serde_json::to_vec(&item)?;
            context
                .create_item(&container, &item.id, item.pk.clone(), &item_json)
                .await?;

            // Enable both rules
            rule1_handle.enable();
            rule2_handle.enable();

            let read_result = context
                .read_item(&container, &item.id, item.pk.clone())
                .await;
            let err = read_result.expect_err("expected first rule (429) to apply");
            assert_eq!(
                err.http_status(),
                Some(StatusCode::TooManyRequests),
                "first matching rule should win (429, not 503)"
            );

            Ok(())
        },
    )
    .await
}

/// First rule has a future start_time, so it's skipped. Second rule (503) applies.
#[tokio::test]
pub async fn fault_injection_first_rule_inactive_due_to_start_time() -> Result<(), Box<dyn Error>> {
    let error1 = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::TooManyRequests)
        .build();
    let condition1 = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();
    let rule1 = Arc::new(
        FaultInjectionRuleBuilder::new("first-rule-429-future", error1)
            .with_condition(condition1)
            .with_start_time(Instant::now() + Duration::from_secs(300))
            .disabled()
            .build(),
    );

    let error2 = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();
    let condition2 = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();
    let rule2 = Arc::new(
        FaultInjectionRuleBuilder::new("second-rule-503", error2)
            .with_condition(condition2)
            .disabled()
            .build(),
    );

    let rule1_handle = Arc::clone(&rule1);
    let rule2_handle = Arc::clone(&rule2);

    DriverTestClient::run_with_fault_injection_and_unique_db(
        vec![rule1, rule2],
        async move |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            let item = TestItem {
                id: "start-time-item".to_string(),
                pk: "pk1".to_string(),
                value: 42,
            };
            let item_json = serde_json::to_vec(&item)?;
            context
                .create_item(&container, &item.id, item.pk.clone(), &item_json)
                .await?;

            // Enable both rules
            rule1_handle.enable();
            rule2_handle.enable();

            let read_result = context
                .read_item(&container, &item.id, item.pk.clone())
                .await;
            let err = read_result.expect_err("expected second rule (503) to apply");
            assert_eq!(
                err.http_status(),
                Some(StatusCode::ServiceUnavailable),
                "second rule should apply (503) since first rule has not started"
            );

            Ok(())
        },
    )
    .await
}

/// First rule has an end_time in the past (already expired). Second rule (503) applies.
#[tokio::test]
pub async fn fault_injection_first_rule_expired_due_to_end_time() -> Result<(), Box<dyn Error>> {
    let error1 = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::TooManyRequests)
        .build();
    let condition1 = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();
    let rule1 = Arc::new(
        FaultInjectionRuleBuilder::new("first-rule-429-expired", error1)
            .with_condition(condition1)
            .with_end_time(Instant::now()) // Already expired
            .disabled()
            .build(),
    );

    let error2 = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();
    let condition2 = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();
    let rule2 = Arc::new(
        FaultInjectionRuleBuilder::new("second-rule-503", error2)
            .with_condition(condition2)
            .disabled()
            .build(),
    );

    let rule1_handle = Arc::clone(&rule1);
    let rule2_handle = Arc::clone(&rule2);

    DriverTestClient::run_with_fault_injection_and_unique_db(
        vec![rule1, rule2],
        async move |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            let item = TestItem {
                id: "end-time-item".to_string(),
                pk: "pk1".to_string(),
                value: 42,
            };
            let item_json = serde_json::to_vec(&item)?;
            context
                .create_item(&container, &item.id, item.pk.clone(), &item_json)
                .await?;

            // Enable both rules
            rule1_handle.enable();
            rule2_handle.enable();

            // Small delay to ensure the expired rule's end_time is firmly in the past
            tokio::time::sleep(Duration::from_millis(100)).await;

            let read_result = context
                .read_item(&container, &item.id, item.pk.clone())
                .await;
            let err = read_result.expect_err("expected second rule (503) to apply");
            assert_eq!(
                err.http_status(),
                Some(StatusCode::ServiceUnavailable),
                "second rule should apply (503) since first rule's end_time has passed"
            );

            Ok(())
        },
    )
    .await
}

/// Fault stops applying after hit_limit is exhausted.
/// With hit_limit=4, the first few reads fail (consuming the budget via retries),
/// and subsequent reads succeed.
#[tokio::test]
pub async fn fault_injection_hit_limit_behavior() -> Result<(), Box<dyn Error>> {
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::InternalServerError)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("hit-limit-test", result)
            .with_condition(condition)
            .with_hit_limit(4)
            .disabled()
            .build(),
    );

    let rule_handle = Arc::clone(&rule);

    DriverTestClient::run_with_fault_injection_and_unique_db(
        vec![rule],
        async move |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            let item = TestItem {
                id: "hit-limit-item".to_string(),
                pk: "pk1".to_string(),
                value: 42,
            };
            let item_json = serde_json::to_vec(&item)?;
            context
                .create_item(&container, &item.id, item.pk.clone(), &item_json)
                .await?;

            // Enable the rule
            rule_handle.enable();

            // The first reads should fail (500 InternalServerError), consuming the hit budget.
            // Retries also count against hit_limit, so depending on retry behavior,
            // the budget may be exhausted within a few logical reads.
            for i in 1..=2 {
                let read_result = context
                    .read_item(&container, &item.id, item.pk.clone())
                    .await;
                assert!(
                    read_result.is_err(),
                    "read {} should fail (within hit_limit)",
                    i
                );
                assert_eq!(
                    read_result.unwrap_err().http_status(),
                    Some(StatusCode::InternalServerError),
                    "read {} should return 500",
                    i
                );
            }

            // After the hit_limit is exhausted, reads should succeed
            let read_result = context
                .read_item(&container, &item.id, item.pk.clone())
                .await;
            assert!(
                read_result.is_ok(),
                "read should succeed after hit_limit exhausted: {:?}",
                read_result.err()
            );

            Ok(())
        },
    )
    .await
}

/// With no fault rules, operations should succeed normally.
#[tokio::test]
pub async fn fault_injection_empty_rules() -> Result<(), Box<dyn Error>> {
    DriverTestClient::run_with_unique_db(async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        let item = TestItem {
            id: "empty-rules-item".to_string(),
            pk: "pk1".to_string(),
            value: 42,
        };
        let item_json = serde_json::to_vec(&item)?;
        context
            .create_item(&container, &item.id, item.pk.clone(), &item_json)
            .await?;

        // Read should succeed with no fault rules
        let read_result = context
            .read_item(&container, &item.id, item.pk.clone())
            .await;
        assert!(
            read_result.is_ok(),
            "read should succeed with no fault rules: {:?}",
            read_result.err()
        );

        Ok(())
    })
    .await
}

/// Disabling a rule at runtime prevents fault injection; re-enabling resumes injection.
#[tokio::test]
pub async fn fault_injection_enable_disable_rule() -> Result<(), Box<dyn Error>> {
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("enable-disable-test", result)
            .with_condition(condition)
            .disabled()
            .build(),
    );

    let rule_handle = Arc::clone(&rule);

    DriverTestClient::run_with_fault_injection_and_unique_db(
        vec![rule],
        async move |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            let item = TestItem {
                id: "toggle-item".to_string(),
                pk: "pk1".to_string(),
                value: 42,
            };
            let item_json = serde_json::to_vec(&item)?;
            context
                .create_item(&container, &item.id, item.pk.clone(), &item_json)
                .await?;

            // Rule starts disabled — read should succeed
            assert!(!rule_handle.is_enabled());
            let read_result = context
                .read_item(&container, &item.id, item.pk.clone())
                .await;
            assert!(
                read_result.is_ok(),
                "read should succeed while rule is disabled: {:?}",
                read_result.err()
            );

            // Enable the rule — read should fail
            rule_handle.enable();
            assert!(rule_handle.is_enabled());
            let read_result = context
                .read_item(&container, &item.id, item.pk.clone())
                .await;
            let err = read_result.expect_err("read should fail while rule is enabled");
            assert_eq!(
                err.http_status(),
                Some(StatusCode::ServiceUnavailable),
                "expected 503 while rule is enabled"
            );

            // Disable the rule — read should succeed again
            rule_handle.disable();
            assert!(!rule_handle.is_enabled());
            let read_result = context
                .read_item(&container, &item.id, item.pk.clone())
                .await;
            assert!(
                read_result.is_ok(),
                "read should succeed after disabling rule: {:?}",
                read_result.err()
            );

            // Re-enable the rule — read should fail again
            rule_handle.enable();
            assert!(rule_handle.is_enabled());
            let read_result = context
                .read_item(&container, &item.id, item.pk.clone())
                .await;
            assert!(
                read_result.is_err(),
                "read should fail after re-enabling rule"
            );

            Ok(())
        },
    )
    .await
}
