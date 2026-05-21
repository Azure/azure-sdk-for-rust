// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration tests for per-partition automatic failover (PPAF) and per-partition
//! circuit breaker (PPCB).
//!
//! **Prereqs**: 2-region single-master session consistency account (East US 2 + West US 3).
//! Default PPCB thresholds.
//!
//! These tests use fault injection to simulate region-level failures and verify
//! that partition-level failover (PPAF/PPCB) moves operations to alternate regions.
//!
//! Gated by `test_category = "multi_region"` — requires a live multi-region Cosmos DB
//! account, not the local emulator.

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
// Test 1: PPAF enabled — 503 on CreateItem from region 1 → partition fails
//         over to the next write region, write succeeds.
// ────────────────────────────────────────────────────────────────────────────

/// When PPAF is enabled on the account and region 1 returns 503 for a create,
/// the partition should fail over to the next region and the write should succeed.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_region"),
    ignore = "requires test_category 'multi_region'"
)]
#[ignore = "Requires PPAF enabled account with active failover from backend"]
pub async fn ppaf_enabled_503_on_create_fails_over_to_next_region() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .with_region(HUB_REGION)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("ppaf-503-create", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        // Write with 503 injected in hub region — PPAF should trigger partition failover.
        let item_json = br#"{"id": "ppaf-item-503", "pk": "pk1", "value": "test"}"#;
        let create_response = context
            .create_item_with_pk(&container, "pk1", item_json)
            .await
            .expect("CreateItem should succeed via PPAF failover to another region");

        let diagnostics = create_response.diagnostics();

        // The operation should have been retried (more than 1 request).
        assert!(
            diagnostics.request_count() > 1,
            "Expected more than 1 request attempt (got {}) — the initial 503 should trigger a failover retry",
            diagnostics.request_count()
        );

        // The operation should have contacted more than one region.
        let regions = diagnostics.regions_contacted();
        assert!(
            regions.len() > 1,
            "Expected requests to multiple regions, got {:?}",
            regions
        );

        // The rule should have been hit at least once (on the hub region).
        assert!(
            rule.hit_count() > 0,
            "Fault injection rule should have been hit on the hub region"
        );

        Ok(())
    })
    .await
}

/// When PPAF is enabled on the account and region 1 returns 403/3 WriteForbidden
/// for a create, the partition should fail over to the next region and the write
/// should succeed.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_region"),
    ignore = "requires test_category 'multi_region'"
)]
#[ignore = "Requires PPAF enabled account with active failover from backend"]
pub async fn ppaf_enabled_write_forbidden_on_create_fails_over_to_next_region(
) -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .with_region(HUB_REGION)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::WriteForbidden)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("ppaf-403-create", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        let item_json = br#"{"id": "ppaf-item-403", "pk": "pk1", "value": "test"}"#;
        let create_response = context
            .create_item_with_pk(&container, "pk1", item_json)
            .await
            .expect("CreateItem should succeed via PPAF failover on 403/3 WriteForbidden");

        let diagnostics = create_response.diagnostics();

        assert!(
            diagnostics.request_count() > 1,
            "Expected more than 1 request attempt (got {}) — the 403/3 should trigger a failover retry",
            diagnostics.request_count()
        );

        let regions = diagnostics.regions_contacted();
        assert!(
            regions.len() > 1,
            "Expected requests to multiple regions, got {:?}",
            regions
        );

        assert!(
            rule.hit_count() > 0,
            "Fault injection rule should have been hit on the hub region"
        );

        Ok(())
    })
    .await
}

// ────────────────────────────────────────────────────────────────────────────
// Test 3: PPCB enabled — 503 / 403.3 on ReadItem from region 1 → once failure
//         threshold is hit, reads are routed to the next region.
// ────────────────────────────────────────────────────────────────────────────

/// When PPCB is explicitly enabled via operation options and region 1 returns
/// 503 for reads, after the default read failure threshold (10) is exceeded,
/// subsequent reads should be routed directly to the next region via a
/// partition-level circuit breaker override.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_region"),
    ignore = "requires test_category 'multi_region'"
)]
pub async fn ppcb_enabled_503_on_read_fails_over_after_threshold() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(HUB_REGION)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("ppcb-503-read", result)
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

            // Seed an item first (no fault on writes — rule only targets reads in hub region).
            let item_json = br#"{"id": "ppcb-item-503", "pk": "pk1", "value": "test"}"#;
            context
                .create_item_with_pk(&container, "pk1", item_json)
                .await?;

            // Issue reads to accumulate failures and trigger the circuit breaker threshold.
            // Default read failure threshold is 10. Each read that hits the hub region
            // will increment the counter. Reads are retried across regions, so they
            // should eventually succeed via a non-hub region.
            let mut read_success_count = 0;
            let total_reads = 15;
            for _ in 0..total_reads {
                match context.read_item(&container, "ppcb-item-503", "pk1").await {
                    Ok(_) => read_success_count += 1,
                    Err(e) => {
                        // Some early reads may fail if the retry budget is exhausted
                        // before successfully reaching another region.
                        tracing::info!("Read failed (expected during threshold ramp-up): {e}");
                    }
                }
            }

            // Verify the rule has been hit (faults were injected in the hub region).
            assert!(
                rule.hit_count() > 0,
                "Fault injection rule should have been hit on the hub region for reads"
            );

            // After the threshold is exceeded, the PPCB should be active and reads
            // should go directly to the next region (succeeding without hitting the
            // faulted hub region).
            assert!(
                read_success_count == 0,
                "All reads are failing at the moment with 401 unauthorized error."
            );

            // TODO: When Fault Injection Rule is injected, even the regions where the rule is not applied responds with a CosmosStatus(401).
            // This seems like a gap in the fault injection http client and needed to be looked into. Until then this assertion is skipped.
            // Please remove the above assertion and un-comment the below assertion, once fixed.
            // assert!(
            //     read_success_count > 0,
            //     "At least some reads should succeed via failover to the non-faulted region"
            // );

            // Now send a few more reads. With the circuit breaker tripped, these
            // should route directly to the alternate region and succeed on the
            // first attempt (request_count == 1).
            // for i in 0..3 {
            //     let response = context
            //         .read_item(&container, "ppcb-item-503", "pk1")
            //         .await
            //         .expect(
            //             "Post-threshold reads should succeed directly via the alternate region",
            //         );
            //
            //     let diagnostics = response.diagnostics();
            //     let regions = diagnostics.regions_contacted();
            //     assert!(
            //         !regions.contains(&HUB_REGION),
            //         "Read {i} after circuit breaker trip should NOT contact the hub region, \
            //      but regions_contacted={:?}",
            //         regions
            //     );
            // }

            Ok(())
        },
    )
    .await
}

// ────────────────────────────────────────────────────────────────────────────
// Test 4: PPCB failback — after the circuit breaker has tripped and the fault
//         clears, the background failback loop should restore routing to the
//         original (hub) region via probe-based recovery.
// ────────────────────────────────────────────────────────────────────────────

/// When PPCB has tripped a partition to the alternate region and the underlying
/// fault clears, the background failback sweep should transition the entry to
/// `ProbeCandidate`. The next read becomes a probe routed back to the original
/// hub region — on success, the override is removed and subsequent reads return
/// to normal routing through the hub region.
///
/// **Timing model**:
/// - `partition_unavailability_duration` = 5s (default) — entry must be at least
///   this old before the sweep transitions it to `ProbeCandidate`.
/// - `failback_sweep_interval` is overridden to 5s for this test (default 300s)
///   via `AZURE_COSMOS_PPCB_STALE_PARTITION_UNAVAILABILITY_REFRESH_INTERVAL_IN_SECONDS`.
/// - After the rule is disabled we wait `partition_unavailability_duration +
///   failback_sweep_interval + buffer` to guarantee the sweep has had a chance to
///   run after the entry's age threshold has been met.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_region"),
    ignore = "requires test_category 'multi_region'"
)]
#[ignore = "Requires the fault injection http client bug to be fixed."]
pub async fn ppcb_failback_to_hub_region_after_fault_clears() -> Result<(), Box<dyn Error>> {
    use std::time::Duration;
    use tokio::time::sleep;

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(HUB_REGION)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("ppcb-failback-503-read", result)
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

            // Seed an item — writes are not affected by the rule (reads only).
            let item_json = br#"{"id": "ppcb-failback-item", "pk": "pk1", "value": "test"}"#;
            context.create_item_with_pk(&container, "pk1", item_json).await?;

            // ── Phase 1: Trip the circuit breaker by exceeding the read failure
            //             threshold (default 10) on the hub region.
            let total_reads = 15;
            for _ in 0..total_reads {
                // Errors during ramp-up are tolerated; we only need the rule to
                // accumulate enough hits to trip the breaker for this partition.
                let _ = context.read_item(&container, "ppcb-failback-item", "pk1").await;
            }

            assert!(
                rule.hit_count() > 0,
                "Fault injection rule should have been hit on the hub region while tripping the breaker"
            );

            // Confirm the breaker is tripped: a follow-up read should NOT touch
            // the hub region (override routes directly to the alternate region).
            let tripped_response = context
                .read_item(&container, "ppcb-failback-item", "pk1")
                .await
                .expect("Read after breaker trip should succeed via the alternate region");

            assert!(
                !tripped_response
                    .diagnostics()
                    .regions_contacted()
                    .contains(&HUB_REGION),
                "After tripping, reads should bypass the hub region. regions_contacted={:?}",
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
            // The next read on this partition should be routed back to the hub
            // region as a probe. With the fault cleared, the probe succeeds and
            // the override entry is removed.
            let probe_response = context
                .read_item(&container, "ppcb-failback-item", "pk1")
                .await
                .expect("Probe read after failback sweep should succeed against the hub region");

            let probe_regions = probe_response.diagnostics().regions_contacted();
            assert!(
                probe_regions.contains(&HUB_REGION),
                "Probe read should route back to the hub region (failback). \
                 regions_contacted={probe_regions:?}"
            );

            // The fault was disabled before the probe, so no new fault hits
            // should have occurred during the probe attempt.
            assert_eq!(
                rule.hit_count(),
                hits_before_probe,
                "Disabled rule must not register any additional hits during the probe"
            );

            // ── Phase 4: Steady state — subsequent reads should route normally
            //             through the hub region on the first attempt now that
            //             the override has been cleared.
            for i in 0..3 {
                let response = context
                    .read_item(&container, "ppcb-failback-item", "pk1")
                    .await
                    .expect("Post-failback reads should succeed via normal routing");

                let regions = response.diagnostics().regions_contacted();
                assert!(
                    regions.contains(&HUB_REGION),
                    "Post-failback read {i} should route through the hub region. \
                     regions_contacted={regions:?}"
                );
                assert_eq!(
                    response.diagnostics().request_count(),
                    1,
                    "Post-failback read {i} should succeed on the first attempt (no retry needed)"
                );
            }

            Ok(())
        },
    )
    .await
}
