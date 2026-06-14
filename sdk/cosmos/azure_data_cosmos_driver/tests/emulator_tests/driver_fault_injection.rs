// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration tests for fault injection functionality.

#![cfg(feature = "fault_injection")]

use crate::framework::DriverTestClient;
use azure_data_cosmos_driver::fault_injection::*;
use azure_data_cosmos_driver::options::{
    OperationOptions, OperationOptionsBuilder, Region, ThrottlingRetryOptionsBuilder,
};
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

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

/// End-to-end validation that the configurable 429 (throttle) retry budget —
/// [`ThrottlingRetryOptions::max_retry_count`](azure_data_cosmos_driver::options::ThrottlingRetryOptions::max_retry_count),
/// exposed on the nested `OperationOptions::throttling_retry_options` group —
/// is honored across the full driver stack against a live emulator account.
///
/// A fault rule injects an HTTP 429 (`TooManyRequests`) on **every**
/// `ReadItem` transport attempt with probability `1.0`. Because the injected
/// 429 carries no sub-status it is a *generic* throttle: the operation
/// pipeline classifies it as region-confirming (no cross-region failover), so
/// the entire retry budget is spent inside a single transport-pipeline
/// invocation. Counting `rule.hit_count()` therefore yields the exact number
/// of read attempts that reached the (faulted) transport client.
///
/// Wire-attempt accounting for `max_retry_count = N`:
///
/// * **Total = `N + 1`** attempts on the wire (1 initial + N retries) for
///   any N, including `N == 0`. The one-shot forced-final-retry safety net
///   in `execute_transport_pipeline` is gated on `attempt_count <
///   max_attempts`, so once the count budget is exhausted the safety net is
///   suppressed too — matching the .NET-parity
///   `MaxRetryAttemptsOnRateLimitedRequests` semantic.
/// * The forced-final retry still fires when the *cumulative-wait* budget
///   (rather than the count) is the limiter; this test uses a generous
///   300-second wait budget so the count is always the sole limiter.
///
/// This mirrors the unit-level coverage in
/// `transport_pipeline::tests::execute_transport_pipeline_honors_configured_max_throttle_attempts`
/// but exercises the real option-resolution → fault-injection → transport
/// retry path instead of driving the loop directly.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn fault_injection_429_honors_configurable_throttle_retry_count(
) -> Result<(), Box<dyn Error>> {
    // (configured throttle-retry budget, expected total ReadItem attempts).
    // Total = 1 initial + N retries (the forced-final retry is suppressed
    // once the count budget is exhausted; it only fires when the
    // cumulative-wait budget is the limiter).
    for (max_throttle_retry_count, expected_hits) in [(0_u32, 1_u32), (1, 2), (3, 4), (5, 6)] {
        let rule = Arc::new(
            FaultInjectionRuleBuilder::new(
                "always-429",
                FaultInjectionResultBuilder::new()
                    .with_error(FaultInjectionErrorType::TooManyRequests)
                    .with_probability(1.0)
                    .build(),
            )
            .with_condition(
                FaultInjectionConditionBuilder::new()
                    .with_operation_type(FaultOperationType::ReadItem)
                    .build(),
            )
            .build(),
        );

        // Pin the throttle-retry budget at the runtime layer of the option
        // view. A generous cumulative-wait budget keeps the attempt count the
        // sole limiter for these small retry counts. No end-to-end latency
        // policy is set, so the transport request carries no deadline and the
        // forced-final retry is immediate.
        let operation_options = OperationOptionsBuilder::new()
            .with_throttling_retry_options(
                ThrottlingRetryOptionsBuilder::new()
                    .with_max_retry_count(max_throttle_retry_count)
                    .with_max_retry_wait_time(Duration::from_secs(300))
                    .build(),
            )
            .build();

        let rule_for_assert = Arc::clone(&rule);
        Box::pin(
            DriverTestClient::run_with_unique_db_and_fault_injection_options(
                vec![rule],
                operation_options,
                async move |context, database| {
                    let container_name = context.unique_container_name();
                    let container = context
                        .create_container(&database, &container_name, "/pk")
                        .await?;

                    // Seed the item with a write. The fault rule targets only
                    // ReadItem, so the seeding write is unaffected.
                    let item_json = br#"{"id": "item1", "pk": "pk1", "value": "test"}"#;
                    context
                        .create_item(&container, "item1", "pk1", item_json)
                        .await?;

                    // The read always observes 429 and ultimately fails once
                    // the throttle budget is exhausted.
                    let read_result = context.read_item(&container, "item1", "pk1").await;
                    assert!(
                        read_result.is_err(),
                        "read must fail once the throttle budget is exhausted \
                         (max_throttle_retry_count={max_throttle_retry_count})",
                    );

                    assert_eq!(
                        rule_for_assert.hit_count(),
                        expected_hits,
                        "max_throttle_retry_count={max_throttle_retry_count} must yield \
                         {expected_hits} ReadItem attempts on the wire, but the 429 fault \
                         rule fired {} time(s)",
                        rule_for_assert.hit_count(),
                    );

                    Ok(())
                },
            ),
        )
        .await?;
    }

    Ok(())
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

// Live-account reproductions for topology-related Forbidden sub-status codes.

/// Pins live 403/1008 handling: refresh topology and retry to another region.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_region"),
    ignore = "requires test_category 'multi_region'"
)]
pub async fn live_403_1008_create_item_triggers_refresh_and_retry() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .with_region(Region::EAST_US_2)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::DatabaseAccountNotFound)
        .with_probability(1.0)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("live-403-1008-create-east2", result)
            .with_condition(condition)
            .with_hit_limit(1)
            .build(),
    );
    // Start disabled so the warmup phase can prime both regions without
    // tripping the fault.
    rule.disable();
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_hedging(
        rules,
        OperationOptions::default(),
        vec![Region::EAST_US_2, Region::WEST_US_3],
        async |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            // Warm both regions to avoid a failover racing control-plane replication.
            let warmup_json = br#"{"id": "warmup", "pk": "pk1", "value": "warmup"}"#;
            context
                .create_item(&container, "warmup", "pk1", warmup_json)
                .await?;
            context.read_item(&container, "warmup", "pk1").await?;
            tokio::time::sleep(Duration::from_secs(5)).await;

            rule.enable();

            let item_json = br#"{"id": "live-1008-item", "pk": "pk1", "value": "test"}"#;
            let result = context
                .create_item(&container, "live-1008-item", "pk1", item_json)
                .await;

            assert!(
                rule.hit_count() >= 1,
                "fault rule must have fired at least once against East US 2",
            );

            match result {
                // Post-fix path: refresh + retry succeeded.
                Ok(response) => {
                    let diagnostics = response.diagnostics();
                    let requests = diagnostics.requests();
                    assert!(
                        requests.len() >= 2,
                        "post-fix: SDK must retry after 403/1008; observed request_count={}, \
                         requests={:?}",
                        requests.len(),
                        requests,
                    );
                }
                // Pre-fix path: 1008 bubbled up.
                Err(err) => {
                    panic!(
                        "403/1008 repro confirmed against a real account: 403/1008 bubbled up \
                         to the caller (no refresh + retry). Error: {err}"
                    );
                }
            }

            Ok(())
        },
    )
    .await
}

/// Pins live 403/3 handling: existing failover to West US 3 must keep working.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
pub async fn live_403_3_create_item_triggers_failover() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .with_region(Region::EAST_US_2)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::WriteForbidden)
        .with_probability(1.0)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("live-403-3-create-east2", result)
            .with_condition(condition)
            .with_hit_limit(1)
            .build(),
    );
    // Start disabled so the warmup phase can prime both regions without
    // tripping the fault.
    rule.disable();
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_hedging(
        rules,
        OperationOptions::default(),
        vec![Region::EAST_US_2, Region::WEST_US_3],
        async |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            // Warm both regions to avoid a failover racing control-plane replication.
            let warmup_json = br#"{"id": "warmup", "pk": "pk1", "value": "warmup"}"#;
            context
                .create_item(&container, "warmup", "pk1", warmup_json)
                .await?;
            context.read_item(&container, "warmup", "pk1").await?;
            tokio::time::sleep(Duration::from_secs(5)).await;

            rule.enable();

            let item_json = br#"{"id": "live-403-3-item", "pk": "pk1", "value": "test"}"#;
            let response = context
                .create_item(&container, "live-403-3-item", "pk1", item_json)
                .await
                .expect(
                    "regression guard: with both regions warmed up, 403/3 on East US 2 must \
                     fail over to West US 3 and succeed",
                );

            assert!(
                rule.hit_count() >= 1,
                "fault rule must have fired at least once against East US 2",
            );

            let diagnostics = response.diagnostics();
            let requests = diagnostics.requests();
            assert!(
                requests.len() >= 2,
                "SDK must retry CreateItem after 403/3; observed request_count={}, \
                 requests={:?}",
                requests.len(),
                requests,
            );

            // The failover attempt must have actually targeted a different
            // region than the faulted attempt.
            let first_region = requests[0].region().cloned();
            let last_region = requests.last().and_then(|r| r.region()).cloned();
            assert_ne!(
                first_region, last_region,
                "post-403/3 retry must target a different region; first={:?}, last={:?}",
                first_region, last_region,
            );

            Ok(())
        },
    )
    .await
}
