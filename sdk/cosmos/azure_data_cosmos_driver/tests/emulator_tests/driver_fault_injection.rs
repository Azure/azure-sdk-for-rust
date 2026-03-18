// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration tests for fault injection functionality.

#![cfg(feature = "fault_injection")]

use crate::framework::DriverTestClient;
use azure_data_cosmos_driver::fault_injection::*;
use std::error::Error;
use std::sync::Arc;

/// Tests that a rule with probability 0.0 never injects faults.
///
/// A read operation should succeed because the fault never fires.
#[tokio::test]
pub async fn fault_injection_probability_zero_never_fails() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(0.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("zero-probability", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        let item_json = br#"{"id": "item1", "pk": "pk1", "value": "test"}"#;
        context.create_item(&container, "pk1", item_json).await?;

        // With probability 0.0, the read should succeed
        let read_result = context.read_item(&container, "item1", "pk1").await;
        assert!(
            read_result.is_ok(),
            "Read should succeed with probability 0.0, got: {:?}",
            read_result.err()
        );

        // Verify the rule was never hit
        assert_eq!(rule.hit_count(), 0, "Rule should not have been hit");

        Ok(())
    })
    .await
}

/// Tests that a ServiceUnavailable fault with probability 1.0 causes read failures.
#[tokio::test]
pub async fn fault_injection_service_unavailable_causes_failure() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("always-503", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        let item_json = br#"{"id": "item1", "pk": "pk1", "value": "test"}"#;
        context.create_item(&container, "pk1", item_json).await?;

        // With probability 1.0, the read should fail
        let read_result = context.read_item(&container, "item1", "pk1").await;
        assert!(
            read_result.is_err(),
            "Read should fail with ServiceUnavailable injected"
        );

        let err_msg = read_result.unwrap_err().to_string();
        assert!(
            err_msg.contains("503")
                || err_msg.contains("Service Unavailable")
                || err_msg.contains("ServiceUnavailable"),
            "Error should indicate 503 Service Unavailable, got: {err_msg}"
        );

        // Verify the rule was hit
        assert!(rule.hit_count() > 0, "Rule should have been hit");

        Ok(())
    })
    .await
}

/// Tests that fault injection respects the operation type filter.
///
/// A rule targeting only ReadItem should not affect CreateItem operations.
#[tokio::test]
pub async fn fault_injection_operation_type_filter() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("read-only-fault", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        // CreateItem should succeed (rule only targets ReadItem)
        let item_json = br#"{"id": "item1", "pk": "pk1", "value": "test"}"#;
        let create_result = context.create_item(&container, "pk1", item_json).await;
        assert!(
            create_result.is_ok(),
            "CreateItem should succeed when rule targets ReadItem, got: {:?}",
            create_result.err()
        );

        // ReadItem should fail (matches the rule)
        let read_result = context.read_item(&container, "item1", "pk1").await;
        assert!(
            read_result.is_err(),
            "ReadItem should fail when targeted by fault injection"
        );

        Ok(())
    })
    .await
}

/// Tests that fault injection stops after the hit limit is reached.
///
/// A rule with a hit limit should only inject faults up to that limit,
/// then allow operations to succeed normally.
#[tokio::test]
pub async fn fault_injection_hit_limit_stops_after_n_faults() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("hit-limit-test", result)
            .with_condition(condition)
            .with_hit_limit(2)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(
        rules,
        async move |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            let item_json = br#"{"id": "item1", "pk": "pk1", "value": "test"}"#;
            context.create_item(&container, "pk1", item_json).await?;

            // Execute reads to consume the hit limit.
            // Due to internal retries, the limit may be exhausted within fewer
            // top-level calls than the limit value.
            for _ in 0..5 {
                let _ = context.read_item(&container, "item1", "pk1").await;
            }

            // Verify the rule was hit exactly the limit number of times
            assert_eq!(
                rule.hit_count(),
                2,
                "Rule should have been hit exactly the hit limit"
            );

            // After hitting the limit, reads should succeed
            let final_read = context.read_item(&container, "item1", "pk1").await;
            assert!(
                final_read.is_ok(),
                "Reads should succeed after hit limit is exhausted, got: {:?}",
                final_read.err()
            );

            Ok(())
        },
    )
    .await
}

/// Tests that a ConnectionError fault causes read failures.
#[tokio::test]
pub async fn fault_injection_connection_error() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ConnectionError)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("connection-error", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        let item_json = br#"{"id": "item1", "pk": "pk1", "value": "test"}"#;
        context.create_item(&container, "pk1", item_json).await?;

        // With a connection error injected, the read should fail
        let read_result = context.read_item(&container, "item1", "pk1").await;
        assert!(
            read_result.is_err(),
            "Read should fail with ConnectionError injected"
        );

        let err_msg = read_result.unwrap_err().to_string();
        assert!(
            err_msg.to_lowercase().contains("connection"),
            "Error should indicate a connection error, got: {err_msg}"
        );

        // Verify the rule was hit
        assert!(rule.hit_count() > 0, "Rule should have been hit");

        Ok(())
    })
    .await
}
