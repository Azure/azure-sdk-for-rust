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
use azure_data_cosmos_driver::options::Region;
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
            .create_item(&container, "pk1", item_json)
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
            .create_item(&container, "pk1", item_json)
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

/// When PPCB is enabled (default when PPAF is enabled) and region 1 returns
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

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        // Seed an item first (no fault on writes — rule only targets reads in hub region).
        let item_json = br#"{"id": "ppcb-item-503", "pk": "pk1", "value": "test"}"#;
        context.create_item(&container, "pk1", item_json).await?;

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

        // After the threshold is exceeded, the PPCB should be active and reads
        // should go directly to the next region (succeeding without hitting the
        // faulted hub region).
        assert!(
            read_success_count > 0,
            "At least some reads should succeed via failover to the non-faulted region"
        );

        // Verify the rule has been hit (faults were injected in the hub region).
        assert!(
            rule.hit_count() > 0,
            "Fault injection rule should have been hit on the hub region for reads"
        );

        // Now send a few more reads. With the circuit breaker tripped, these
        // should route directly to the alternate region and succeed on the
        // first attempt (request_count == 1).
        for i in 0..3 {
            let response = context
                .read_item(&container, "ppcb-item-503", "pk1")
                .await
                .expect("Post-threshold reads should succeed directly via the alternate region");

            let diagnostics = response.diagnostics();
            let regions = diagnostics.regions_contacted();
            assert!(
                !regions.contains(&HUB_REGION),
                "Read {i} after circuit breaker trip should NOT contact the hub region, \
                 but regions_contacted={:?}",
                regions
            );
        }

        Ok(())
    })
    .await
}
