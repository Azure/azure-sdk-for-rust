// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration tests for fault injection functionality.

#![cfg(feature = "fault_injection")]

use crate::framework::DriverTestClient;
use azure_data_cosmos_driver::diagnostics::TransportKind;
use azure_data_cosmos_driver::fault_injection::*;
use std::error::Error;
use std::sync::Arc;

/// Tests that a rule with probability 0.0 never injects faults.
///
/// A read operation should succeed because the fault never fires.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
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
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
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
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
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
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
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
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
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

// ----------------------------------------------------------------------------
// Gateway 2.0 fault injection coverage (Phase 6)
// ----------------------------------------------------------------------------
//
// The following three tests lock in the retry/failover behavior the Gateway
// 2.0 transport must exhibit when the underlying thin-client connection fails.
// Each test exercises a distinct failure shape:
//
//   - 503 Service Unavailable → regional failover
//   - 408 Request Timeout     → cross-region for reads / local-only for writes
//   - 404/1002 Read Session   → remote-preferred + no PKRange refresh
//
// **Limitation**: `FaultInjectionCondition` does not yet expose a per-transport-
// kind filter — there is no `with_transport_kind(TransportKind::Gateway20)`
// today. As a result, faults injected here apply to whichever transport happens
// to be selected at dispatch time. To reliably exercise these against Gateway
// 2.0, the Phase 6 CI matrix must run them on a live thin-client account
// (`testCategory = 'gateway20'`); the emulator does not yet expose Gateway
// 2.0 endpoints. See `docs/GATEWAY_20_SPEC.md` (Phase 6) for the harness gap.

/// Gateway 2.0 503 Service Unavailable should trigger regional failover.
///
/// The rule is scoped to [`TransportKind::Gateway20`] so it does not also
/// fire on standard-gateway requests issued during account discovery. The
/// emulator does not yet expose Gateway 2.0 endpoints, so this test is
/// gated behind the `gateway20` test category until CI gains a thin-client
/// account; see `docs/GATEWAY_20_SPEC.md` (Phase 6).
#[tokio::test]
#[cfg_attr(
    not(test_category = "gateway20"),
    ignore = "requires test_category 'gateway20'"
)]
pub async fn gateway20_service_unavailable_triggers_regional_failover() -> Result<(), Box<dyn Error>>
{
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_transport_kind(TransportKind::Gateway20)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("gateway20-503-failover", result)
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

        // The read should fail (single region, fault always fires) but the
        // failover machinery must have been invoked. Once `RequestDiagnostics`
        // exposes per-attempt endpoint selection, assert that the diagnostics
        // record at least one regional failover attempt.
        let read_result = context.read_item(&container, "item1", "pk1").await;
        assert!(
            read_result.is_err(),
            "Read should fail when 503 fires on every attempt"
        );

        assert!(rule.hit_count() > 0, "Rule should have been hit");

        Ok(())
    })
    .await
}

/// Gateway 2.0 408 Request Timeout should retry across regions for reads,
/// but stay local-only for writes (single-region writes can't safely retry
/// across regions without risking duplicates).
///
/// The rule is scoped to [`TransportKind::Gateway20`] so it does not affect
/// standard-gateway traffic. The emulator does not yet expose Gateway 2.0
/// endpoints, so this test is gated behind the `gateway20` test category.
#[tokio::test]
#[cfg_attr(
    not(test_category = "gateway20"),
    ignore = "requires test_category 'gateway20'"
)]
pub async fn gateway20_request_timeout_cross_region_for_reads() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_transport_kind(TransportKind::Gateway20)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::Timeout)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("gateway20-408-cross-region", result)
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

        let read_result = context.read_item(&container, "item1", "pk1").await;
        assert!(
            read_result.is_err(),
            "Read should ultimately fail when 408 fires on every attempt"
        );

        // TODO(Phase 6): once diagnostics expose retry attempts, assert that
        // a single-region account exhausts local-only retries while a
        // multi-region account performs at least one cross-region attempt.
        assert!(rule.hit_count() > 0, "Rule should have been hit");

        Ok(())
    })
    .await
}

/// Gateway 2.0 404/1002 ReadSessionNotAvailable must trigger a
/// remote-preferred retry path **without** invalidating the partition-key
/// range (PKRange) cache. The 404/1002 substatus indicates a session-token
/// mismatch, which is unrelated to the routing topology — refreshing PKRange
/// would be a wasted metadata round-trip.
///
/// The rule is scoped to [`TransportKind::Gateway20`] so it does not also
/// fire on standard-gateway requests. The emulator does not yet expose
/// Gateway 2.0 endpoints, so this test is gated behind the `gateway20`
/// test category until CI gains a thin-client account.
#[tokio::test]
#[cfg_attr(
    not(test_category = "gateway20"),
    ignore = "requires test_category 'gateway20'"
)]
pub async fn gateway20_read_session_not_available_remote_preferred() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_transport_kind(TransportKind::Gateway20)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ReadSessionNotAvailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("gateway20-1002-remote-preferred", result)
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

        let read_result = context.read_item(&container, "item1", "pk1").await;
        assert!(
            read_result.is_err(),
            "Read should fail when 404/1002 fires on every attempt"
        );

        // TODO(Phase 6): once diagnostics record metadata-cache hits, assert
        // that the PKRange cache was NOT refreshed during these retries (a
        // 404/1002 is a session-token issue, not a routing-topology issue).
        assert!(rule.hit_count() > 0, "Rule should have been hit");

        Ok(())
    })
    .await
}
