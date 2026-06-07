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
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
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
        context.create_item(&container, "item1", "pk1", item_json).await?;

        // With probability 0.0, the read should succeed
        let read_response = context
            .read_item(&container, "item1", "pk1")
            .await
            .expect("Read should succeed with probability 0.0");

        // Verify the rule was never hit
        assert_eq!(rule.hit_count(), 0, "Rule should not have been hit");

        // Verify evaluations are in diagnostics
        let diagnostics = read_response.diagnostics();
        let requests = diagnostics.requests();
        assert!(!requests.is_empty(), "Should have at least one request");

        // At least one request should have evaluations showing the probability miss
        let has_probability_miss = requests.iter().any(|r| {
            r.fault_injection_evaluations().iter().any(|e| {
                matches!(e, FaultInjectionEvaluation::ProbabilityMiss { rule_id, .. } if rule_id == "zero-probability")
            })
        });
        assert!(
            has_probability_miss,
            "Diagnostics should contain ProbabilityMiss evaluation for the zero-probability rule"
        );

        Ok(())
    })
    .await
}

/// Tests that a ServiceUnavailable fault with probability 1.0 causes read failures.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
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
        context
            .create_item(&container, "item1", "pk1", item_json)
            .await?;

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
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
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

    Box::pin(DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        // CreateItem should succeed (rule only targets ReadItem)
        let item_json = br#"{"id": "item1", "pk": "pk1", "value": "test"}"#;
        let create_response = context
            .create_item(&container, "item1", "pk1", item_json)
            .await
            .expect("CreateItem should succeed when rule targets ReadItem");

        // CreateItem should show OperationMismatch for the read-only rule
        let create_diagnostics = create_response.diagnostics();
        let create_requests = create_diagnostics.requests();

        let has_op_mismatch = create_requests.iter().any(|r| {
            r.fault_injection_evaluations().iter().any(|e| {
                matches!(e, FaultInjectionEvaluation::OperationMismatch { rule_id } if rule_id == "read-only-fault")
            })
        });
        assert!(
            has_op_mismatch,
            "CreateItem diagnostics should contain OperationMismatch evaluation"
        );

        // ReadItem should fail (matches the rule)
        let read_result = context.read_item(&container, "item1", "pk1").await;
        assert!(
            read_result.is_err(),
            "ReadItem should fail when targeted by fault injection"
        );

        Ok(())
    }))
    .await
}

/// Tests that fault injection stops after the hit limit is reached.
///
/// A rule with a hit limit should only inject faults up to that limit,
/// then allow operations to succeed normally.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
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
            context.create_item(&container, "item1", "pk1", item_json).await?;

            // Execute reads to consume the hit limit.
            // Due to internal retries, the limit may be exhausted within fewer
            // top-level calls than the limit value.
            for _ in 0..5 {
                let result = context.read_item(&container, "item1", "pk1").await;
                if result.is_ok() {
                    // Hit limit exhausted — reads succeed now
                    break;
                }
            }

            // Verify the rule was hit exactly the limit number of times
            assert_eq!(
                rule.hit_count(),
                2,
                "Rule should have been hit exactly the hit limit"
            );

            // After hitting the limit, reads should succeed
            let final_response = context
                .read_item(&container, "item1", "pk1")
                .await
                .expect("Reads should succeed after hit limit is exhausted");

            // Verify diagnostics contain HitLimitExhausted evaluation
            let final_diagnostics = final_response.diagnostics();
            let final_requests = final_diagnostics.requests();

            let has_hit_limit = final_requests.iter().any(|r| {
                r.fault_injection_evaluations().iter().any(|e| {
                    matches!(e, FaultInjectionEvaluation::HitLimitExhausted { rule_id, .. } if rule_id == "hit-limit-test")
                })
            });
            assert!(
                has_hit_limit,
                "Diagnostics should contain HitLimitExhausted evaluation after limit reached"
            );

            Ok(())
        },
    )
    .await
}

/// Tests that a ConnectionError fault causes read failures.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
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
        context
            .create_item(&container, "item1", "pk1", item_json)
            .await?;

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

/// Verifies that a transient failure on a force-refresh of the partition-key
/// range cache does NOT regress the cached routing map to empty.
///
/// Scenario:
/// 1. Install (but disable) a one-shot `ServiceUnavailable` fault on
///    `MetadataPartitionKeyRanges`.
/// 2. Warm the routing-map cache successfully (fault disabled).
/// 3. Enable the fault, then force-refresh the cache. The fetch fails.
/// 4. Assert the post-refresh routing map is still populated, proving the
///    cache kept the previously cached map rather than replacing it with an
///    empty placeholder that would break routing until the next explicit
///    invalidation.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn pkrange_refresh_transient_failure_preserves_cached_routing_map(
) -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::MetadataPartitionKeyRanges)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("pkrange-refresh-transient", result)
            .with_condition(condition)
            .with_hit_limit(1)
            .build(),
    );
    // Start disabled so the warmup below isn't intercepted; we enable the
    // rule immediately before force-refreshing so the failure is guaranteed
    // to land on the refresh path under test.
    rule.disable();
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        // Warmup: fault is disabled, this populates the cache with real ranges.
        let warmed = context
            .resolve_all_partition_key_ranges(&container, false)
            .await?;
        assert!(
            warmed.is_some_and(|r| !r.is_empty()),
            "warmup resolve must populate the routing map"
        );
        assert_eq!(
            rule.hit_count(),
            0,
            "warmup must not have triggered the disabled fault"
        );

        // Arm the fault and force-refresh. With the fix in place, the refresh
        // sees the transient failure but preserves the previously cached map.
        rule.enable();
        let refreshed = context
            .resolve_all_partition_key_ranges(&container, true)
            .await?;

        assert!(
            refreshed.is_some(),
            "force-refresh on transient failure must not return None"
        );
        let ranges = refreshed.unwrap();
        assert!(
            !ranges.is_empty(),
            "force-refresh on transient failure must preserve the previously cached \
             routing map -- empty ranges indicate the cache regressed to empty"
        );

        assert_eq!(
            rule.hit_count(),
            1,
            "force-refresh must have triggered the fault exactly once"
        );

        // A subsequent non-refresh lookup must still see the populated cache.
        let after = context
            .resolve_all_partition_key_ranges(&container, false)
            .await?;
        assert!(
            after.is_some_and(|r| !r.is_empty()),
            "subsequent non-refresh lookup must observe the preserved routing map"
        );

        Ok(())
    })
    .await
}
