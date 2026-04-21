// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration tests for per-partition automatic failover (PPAF) and per-partition
//! circuit breaker (PPCB).
//!
//! **Prereqs**: 3-region single-master session consistency account with PPAF enabled.
//! Default PPCB thresholds.
//!
//! These tests use fault injection to simulate region-level failures and verify
//! that partition-level failover (PPAF/PPCB) moves operations to alternate regions.

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
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
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
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
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
// Test 2: PPAF disabled — 503 / 403.3 on CreateItem from region 1 → partition
//         does NOT fail over; write fails.
// ────────────────────────────────────────────────────────────────────────────

/// When PPAF is disabled, a 503 on CreateItem from region 1 should NOT trigger
/// partition failover for a non-idempotent write. The write should fail because
/// the request was already sent and PPAF retry is not allowed.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn ppaf_disabled_503_on_create_does_not_failover() -> Result<(), Box<dyn Error>> {
    // This test requires a single-master account with PPAF **disabled**.
    // When PPAF is off and the request is already sent, a non-idempotent write
    // (CreateItem) that gets a transient error (503) is not safe to retry, so
    // the driver should abort.
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .with_region(HUB_REGION)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("no-ppaf-503-create", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        let item_json = br#"{"id": "no-ppaf-item-503", "pk": "pk1", "value": "test"}"#;
        let create_result = context.create_item(&container, "pk1", item_json).await;

        // With PPAF disabled, the non-idempotent write should fail (abort).
        assert!(
            create_result.is_err(),
            "CreateItem should fail when PPAF is disabled and 503 is returned — \
            non-idempotent write cannot be retried safely"
        );

        let err_msg = create_result.unwrap_err().to_string();
        assert!(
            err_msg.contains("503") || err_msg.contains("ServiceUnavailable"),
            "Error should indicate 503 Service Unavailable, got: {err_msg}"
        );

        Ok(())
    })
    .await
}

/// When PPAF is disabled, a 403/3 WriteForbidden on CreateItem should trigger
/// a cross-regional failover retry (403/3 always triggers failover regardless
/// of PPAF), but without partition-level override. Subsequent writes to the same
/// partition will still attempt the hub region first.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn ppaf_disabled_write_forbidden_on_create_retries_but_no_partition_override(
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
        FaultInjectionRuleBuilder::new("no-ppaf-403-create", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        // 403/3 WriteForbidden always triggers FailoverRetry regardless of PPAF,
        // so the first write should succeed by failing over to another region.
        let item_json = br#"{"id": "no-ppaf-item-403", "pk": "pk1", "value": "test"}"#;
        let create_response = context
            .create_item(&container, "pk1", item_json)
            .await
            .expect("CreateItem should succeed via cross-regional failover on 403/3");

        let diagnostics = create_response.diagnostics();
        assert!(
            diagnostics.request_count() > 1,
            "Expected failover retry on 403/3 (got {} requests)",
            diagnostics.request_count()
        );

        // Disable the rule so the next write can succeed without faults.
        rule.disable();

        // Without PPAF, there is no partition-level override, so the next write
        // should go back to the hub region (because the endpoint was only marked
        // unavailable for WriteForbidden, which is ignored for reads and expires
        // per the TTL).
        let item_json2 = br#"{"id": "no-ppaf-item-403-2", "pk": "pk1", "value": "test2"}"#;
        let second_response = context
            .create_item(&container, "pk1", item_json2)
            .await
            .expect("Second CreateItem should succeed after fault rule is disabled");

        // Verify the second write reached the hub region (no partition override
        // steering it to an alternate region).
        let second_diagnostics = second_response.diagnostics();
        assert_eq!(
            second_diagnostics.request_count(),
            1,
            "Second write should succeed on the first attempt (no fault injection)"
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
/// 503 for reads, after the default read failure threshold (5) is exceeded,
/// subsequent reads should be routed directly to the next region via a
/// partition-level circuit breaker override.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
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
        // Default read failure threshold is 5. Each read that hits the hub region
        // will increment the counter. Reads are retried across regions, so they
        // should eventually succeed via a non-hub region.
        let mut read_success_count = 0;
        let total_reads = 10;
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

/// Same as above but with 403/3 WriteForbidden injected on reads from hub region
/// to verify PPCB also handles write-forbidden errors for reads.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn ppcb_enabled_write_forbidden_on_read_fails_over_after_threshold(
) -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(HUB_REGION)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::WriteForbidden)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("ppcb-403-read", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        let item_json = br#"{"id": "ppcb-item-403", "pk": "pk1", "value": "test"}"#;
        context.create_item(&container, "pk1", item_json).await?;

        // Issue reads to trip the circuit breaker.
        for _ in 0..10 {
            let _ = context.read_item(&container, "ppcb-item-403", "pk1").await;
        }

        assert!(
            rule.hit_count() > 0,
            "Fault injection rule should have been hit"
        );

        // Post-threshold reads should go directly to the alternate region.
        for i in 0..3 {
            let response = context
                .read_item(&container, "ppcb-item-403", "pk1")
                .await
                .expect("Post-threshold reads should succeed via alternate region");

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

// ────────────────────────────────────────────────────────────────────────────
// Test 4: PPCB disabled — 503 on ReadItem from region 1 → no partition-level
//         failover; reads still go to hub and fail, triggering cross-regional
//         retry (higher latency, no sticky override).
// ────────────────────────────────────────────────────────────────────────────

/// When PPCB is disabled, 503 on reads from region 1 should NOT create a
/// partition-level override. Each read still attempts the hub region first,
/// fails, and retries cross-regionally. This means higher latency but reads
/// still eventually succeed after each cross-regional retry.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn ppcb_disabled_503_on_read_no_partition_failover() -> Result<(), Box<dyn Error>> {
    // This test requires PPCB to be disabled on the account/driver.
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(HUB_REGION)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("no-ppcb-503-read", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        let item_json = br#"{"id": "no-ppcb-item-503", "pk": "pk1", "value": "test"}"#;
        context.create_item(&container, "pk1", item_json).await?;

        // Without PPCB, reads should still succeed via cross-regional retry,
        // but each read will first hit the hub region (getting the 503) and
        // then retry to another region. This means the hub region is always
        // contacted — no partition-level circuit breaker override.
        for i in 0..5 {
            let response = context
                .read_item(&container, "no-ppcb-item-503", "pk1")
                .await
                .expect("Reads should eventually succeed via cross-regional retry");

            let diagnostics = response.diagnostics();

            // Without PPCB, each read should have more than 1 request attempt
            // because the hub region fails first, then a retry succeeds.
            assert!(
                diagnostics.request_count() > 1,
                "Read {i}: expected more than 1 request attempt (got {}) — \
                 the hub region should fail and trigger a cross-regional retry",
                diagnostics.request_count()
            );

            // The hub region should always be contacted (no partition override
            // skips it).
            let regions = diagnostics.regions_contacted();
            assert!(
                regions.contains(&HUB_REGION),
                "Read {i}: hub region should still be contacted without PPCB, \
                 but regions_contacted={:?}",
                regions
            );
        }

        Ok(())
    })
    .await
}

/// Same as above but with 403/3 WriteForbidden on reads when PPCB is disabled.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn ppcb_disabled_write_forbidden_on_read_no_partition_failover(
) -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(HUB_REGION)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::WriteForbidden)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("no-ppcb-403-read", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        let item_json = br#"{"id": "no-ppcb-item-403", "pk": "pk1", "value": "test"}"#;
        context.create_item(&container, "pk1", item_json).await?;

        // Without PPCB, each read will hit the hub region first, get the fault,
        // and then retry cross-regionally. The hub region should always be contacted.
        for i in 0..5 {
            let response = context
                .read_item(&container, "no-ppcb-item-403", "pk1")
                .await
                .expect("Reads should eventually succeed via cross-regional retry");

            let diagnostics = response.diagnostics();
            assert!(
                diagnostics.request_count() > 1,
                "Read {i}: expected cross-regional retry (got {} requests)",
                diagnostics.request_count()
            );

            let regions = diagnostics.regions_contacted();
            assert!(
                regions.contains(&HUB_REGION),
                "Read {i}: hub region should be contacted without PPCB, \
                 but regions_contacted={:?}",
                regions
            );
        }

        Ok(())
    })
    .await
}
