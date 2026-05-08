// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration tests for per-partition circuit breaker (PPCB) on **writes**
//! against multi-master (multiple write regions enabled) accounts.
//!
//! **Prereqs**: Multi-master Cosmos DB account with at least 2 write regions
//! (e.g. North Central US + South Central US). Default PPCB thresholds.
//!
//! These tests use fault injection to simulate region-level write failures and
//! verify that PPCB:
//!   1. Trips the circuit for a partition after the write failure threshold is
//!      exceeded, routing subsequent writes to an alternate write region.
//!   2. Fails back to the original (hub) region via probe-based recovery once
//!      the underlying fault clears.
//!
//! Gated by `test_category = "multi_write"` — requires a live multi-master
//! Cosmos DB account, not the local emulator.

#![cfg(feature = "fault_injection")]

use crate::framework::DriverTestClient;
use azure_data_cosmos_driver::fault_injection::{
    FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
    FaultInjectionRuleBuilder, FaultOperationType,
};
use azure_data_cosmos_driver::options::{OperationOptionsBuilder, Region};
use std::error::Error;
use std::sync::Arc;

/// The primary region where the account's hub is located.
/// Must match the first preferred write region of the test account.
const HUB_REGION: Region = Region::EAST_US_2;

// ────────────────────────────────────────────────────────────────────────────
// Test 1: PPCB enabled — 503 on CreateItem from the hub region trips the
//         circuit breaker after the write failure threshold is exceeded;
//         subsequent writes route directly to an alternate write region.
// ────────────────────────────────────────────────────────────────────────────

/// On a multi-master account with PPCB enabled, when the hub region returns 503
/// for `CreateItem`, the circuit breaker for the affected partition should trip
/// once the write failure threshold (default 5) is exceeded. Subsequent writes
/// must route directly to an alternate write region without contacting the hub.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
#[ignore = "Requires the fault injection http client bug to be fixed."]
pub async fn ppcb_enabled_503_on_create_fails_over_after_threshold() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .with_region(HUB_REGION)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("ppcb-503-create-multi-write", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    // PPCB is disabled by default; explicitly enable it via operation options.
    let operation_options = OperationOptionsBuilder::new()
        .with_per_partition_circuit_breaker_enabled(true)
        .build();

    DriverTestClient::run_with_unique_db_and_fault_injection_options(
        rules,
        operation_options,
        async |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            // Issue creates to accumulate failures and trigger the circuit breaker
            // threshold. Default write failure threshold is 5 on multi-master
            // accounts. Each create that hits the hub region will increment the
            // counter. Writes are retried across write regions, so they should
            // eventually succeed via a non-hub region.
            let mut create_success_count = 0;
            let total_creates = 10;
            for i in 0..total_creates {
                let body = format!(
                    r#"{{"id": "ppcb-create-503-{i}", "pk": "pk1", "value": "test"}}"#
                );
                match context.create_item_with_pk(&container, "pk1", body.as_bytes()).await {
                    Ok(_) => create_success_count += 1,
                    Err(e) => {
                        // Some early creates may fail if the retry budget is exhausted
                        // before successfully reaching another write region.
                        tracing::info!("Create failed (expected during threshold ramp-up): {e}");
                    }
                }
            }

            // After the threshold is exceeded, the PPCB should be active and writes
            // should go directly to the next write region (succeeding without hitting
            // the faulted hub region).
            assert!(
                create_success_count > 0,
                "At least some creates should succeed via failover to the non-faulted write region"
            );

            // Verify the rule has been hit (faults were injected in the hub region).
            assert!(
                rule.hit_count() > 0,
                "Fault injection rule should have been hit on the hub region for creates"
            );

            // Now send a few more creates. With the circuit breaker tripped, these
            // should route directly to the alternate write region and succeed on
            // the first attempt (request_count == 1).
            for i in 0..3 {
                let body = format!(
                    r#"{{"id": "ppcb-create-503-post-{i}", "pk": "pk1", "value": "test"}}"#
                );
                let response = context
                    .create_item_with_pk(&container, "pk1", body.as_bytes())
                    .await
                    .expect(
                        "Post-threshold creates should succeed directly via the alternate write region",
                    );

                let diagnostics = response.diagnostics();
                let regions = diagnostics.regions_contacted();
                assert!(
                    !regions.contains(&HUB_REGION),
                    "Create {i} after circuit breaker trip should NOT contact the hub region, \
                 but regions_contacted={:?}",
                    regions
                );
            }

            Ok(())
        },
    )
    .await
}

// ────────────────────────────────────────────────────────────────────────────
// Test 2: PPCB write failback — after the circuit breaker has tripped on
//         writes and the fault clears, the background failback loop should
//         restore routing to the original (hub) write region via probe-based
//         recovery.
// ────────────────────────────────────────────────────────────────────────────

/// When PPCB has tripped a partition to an alternate write region (due to write
/// failures in the hub) and the underlying fault clears, the background failback
/// sweep should transition the entry to `ProbeCandidate`. The next write becomes
/// a probe routed back to the original hub region — on success, the override is
/// removed and subsequent writes return to normal routing through the hub region.
///
/// **Timing model**:
/// - `partition_unavailability_duration` = 5s (default) — entry must be at least
///   this old before the sweep transitions it to `ProbeCandidate`.
/// - `failback_sweep_interval` is overridden to 5s for this test (default 300s)
///   via `with_ppcb_stale_partition_unavailability_refresh_interval_in_seconds`.
/// - After the rule is disabled we wait `partition_unavailability_duration +
///   failback_sweep_interval + buffer` to guarantee the sweep has had a chance
///   to run after the entry's age threshold has been met.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_write"),
    ignore = "requires test_category 'multi_write'"
)]
#[ignore = "Requires the fault injection http client bug to be fixed."]
pub async fn ppcb_failback_to_hub_region_after_write_fault_clears() -> Result<(), Box<dyn Error>> {
    use std::time::Duration;
    use tokio::time::sleep;

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .with_region(HUB_REGION)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("ppcb-failback-503-create", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    // Tighten the failback sweep interval so the test does not have to wait the
    // 300s production default. `partition_unavailability_duration` is left at
    // its 5s default (which is also the minimum permitted value).
    const FAILBACK_SWEEP_SECS: u32 = 5;
    const PARTITION_UNAVAILABILITY_SECS: u32 = 5;

    let operation_options = OperationOptionsBuilder::new()
        .with_per_partition_circuit_breaker_enabled(true)
        .with_ppcb_stale_partition_unavailability_refresh_interval_in_seconds(FAILBACK_SWEEP_SECS)
        .with_allowed_partition_unavailability_duration_in_seconds(PARTITION_UNAVAILABILITY_SECS)
        .build();

    DriverTestClient::run_with_unique_db_and_fault_injection_options(
        rules,
        operation_options,
        async |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            // ── Phase 1: Trip the circuit breaker by exceeding the write failure
            //             threshold (default 5 on multi-master accounts) on the
            //             hub region.
            let total_creates = 10;
            for i in 0..total_creates {
                let body = format!(
                    r#"{{"id": "ppcb-write-failback-trip-{i}", "pk": "pk1", "value": "test"}}"#
                );
                // Errors during ramp-up are tolerated; we only need the rule to
                // accumulate enough hits to trip the breaker for this partition.
                let _ = context.create_item_with_pk(&container, "pk1", body.as_bytes()).await;
            }

            assert!(
                rule.hit_count() > 0,
                "Fault injection rule should have been hit on the hub region while tripping the breaker"
            );

            // Confirm the breaker is tripped: a follow-up create should NOT touch
            // the hub region (override routes directly to the alternate write region).
            let tripped_body =
                br#"{"id": "ppcb-write-failback-tripped", "pk": "pk1", "value": "test"}"#;
            let tripped_response = context
                .create_item_with_pk(&container, "pk1", tripped_body)
                .await
                .expect("Create after breaker trip should succeed via the alternate write region");

            assert!(
                !tripped_response
                    .diagnostics()
                    .regions_contacted()
                    .contains(&HUB_REGION),
                "After tripping, writes should bypass the hub region. regions_contacted={:?}",
                tripped_response.diagnostics().regions_contacted()
            );

            // ── Phase 2: Clear the fault and wait for failback sweep.
            //
            // The sweep transitions `Unhealthy` entries older than
            // `partition_unavailability_duration` to `ProbeCandidate`. We wait
            // long enough to cover both the unavailability age threshold AND a
            // full sweep cycle, plus a small buffer for scheduling jitter.
            rule.disable();

            let wait =
                Duration::from_secs((PARTITION_UNAVAILABILITY_SECS + FAILBACK_SWEEP_SECS + 5) as u64);
            tracing::info!(
                "Disabled fault rule; waiting {wait:?} for failback sweep to mark entry as ProbeCandidate"
            );
            sleep(wait).await;

            let hits_before_probe = rule.hit_count();

            // ── Phase 3: Probe attempt.
            //
            // The next write on this partition should be routed back to the hub
            // region as a probe. With the fault cleared, the probe succeeds and
            // the override entry is removed.
            let probe_body =
                br#"{"id": "ppcb-write-failback-probe", "pk": "pk1", "value": "test"}"#;
            let probe_response = context
                .create_item_with_pk(&container, "pk1", probe_body)
                .await
                .expect("Probe write after failback sweep should succeed against the hub region");

            let probe_regions = probe_response.diagnostics().regions_contacted();
            assert!(
                probe_regions.contains(&HUB_REGION),
                "Probe write should route back to the hub region (failback). \
                 regions_contacted={probe_regions:?}"
            );

            // The fault was disabled before the probe, so no new fault hits
            // should have occurred during the probe attempt.
            assert_eq!(
                rule.hit_count(),
                hits_before_probe,
                "Disabled rule must not register any additional hits during the probe"
            );

            // ── Phase 4: Steady state — subsequent writes should route normally
            //             through the hub region on the first attempt now that
            //             the override has been cleared.
            for i in 0..3 {
                let body = format!(
                    r#"{{"id": "ppcb-write-failback-steady-{i}", "pk": "pk1", "value": "test"}}"#
                );
                let response = context
                    .create_item_with_pk(&container, "pk1", body.as_bytes())
                    .await
                    .expect("Post-failback writes should succeed via normal routing");

                let regions = response.diagnostics().regions_contacted();
                assert!(
                    regions.contains(&HUB_REGION),
                    "Post-failback write {i} should route through the hub region. \
                     regions_contacted={regions:?}"
                );
                assert_eq!(
                    response.diagnostics().request_count(),
                    1,
                    "Post-failback write {i} should succeed on the first attempt (no retry needed)"
                );
            }

            Ok(())
        },
    )
    .await
}
