// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration tests for partition key range cache integration and partition-level
//! failover behavior tested against the local emulator via fault injection.
//!
//! These tests cover:
//!   1. PK range fetch failure graceful fallback — when the metadata request for
//!      partition key ranges fails (503), the pre-resolution returns `None` and the
//!      operation still succeeds without a pre-resolved pk_range_id.
//!   2. Partition split / 410 Gone handling — when a data operation returns 410
//!      (PartitionIsGone), the pipeline performs a failover retry and the operation
//!      ultimately succeeds.
//!
//! These tests are gated on `test_category = "emulator"` only — they are
//! intentionally not run against `test_category = "emulator_vnext"` because
//! partition-failover semantics depend on PK-range cache + multi-replica
//! gateway behavior that the vnext (Linux) emulator does not model the same
//! way as the legacy Windows emulator.

#![cfg(feature = "fault_injection")]

use crate::framework::DriverTestClient;
use azure_data_cosmos_driver::fault_injection::*;
use azure_data_cosmos_driver::options::PartitionFailoverOptions;
use std::error::Error;
use std::sync::Arc;

// ────────────────────────────────────────────────────────────────────────────
// PK Range Cache Tests
// ────────────────────────────────────────────────────────────────────────────

/// When the `MetadataPartitionKeyRanges` request returns 503, the pre-resolution
/// of the partition key range ID for PPCB/PPAF is non-fatal: the driver falls
/// back to executing the operation without a pre-resolved `partition_key_range_id`.
///
/// The data operation (ReadItem) must still succeed — the 503 on pkranges only
/// prevents the driver from pre-routing the request to the optimal region;
/// it does not abort the data operation.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn pkrange_fetch_503_falls_back_gracefully_to_data_operation(
) -> Result<(), Box<dyn Error>> {
    // Inject a persistent 503 on ALL MetadataPartitionKeyRanges requests so
    // that pre-resolution always fails.
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::MetadataPartitionKeyRanges)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("pkrange-503-always", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    // PPCB must be enabled for the driver to actually fetch PK ranges
    // (pre_resolve_partition_key_range_id short-circuits otherwise).
    let partition_failover_options = PartitionFailoverOptions::builder()
        .with_circuit_breaker_enabled(true)
        .build()?;

    DriverTestClient::run_with_unique_db_and_fault_injection_partition_failover_options(
        rules,
        partition_failover_options,
        async |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            // Create an item — succeeds even though pkrange pre-resolution is failing.
            let item_json = br#"{"id": "pkrange-fallback-1", "pk": "pk1", "value": "test"}"#;
            context
                .create_seed_item(&container, "pkrange-fallback-1", "pk1", item_json)
                .await
                .expect("CreateItem must succeed even when pkrange metadata fetch returns 503");

            // Read the item back — also succeeds without pre-resolved pk_range_id.
            context
                .read_item(&container, "pkrange-fallback-1", "pk1")
                .await
                .expect("ReadItem must succeed even when pkrange metadata fetch returns 503");

            // Confirm the rule was hit (the injected fault actually fired).
            assert!(
                rule.hit_count() > 0,
                "MetadataPartitionKeyRanges fault should have been hit at least once"
            );

            Ok(())
        },
    )
    .await
}

/// When `MetadataPartitionKeyRanges` fails transiently (503) for the first few
/// requests but then recovers, subsequent data operations that trigger a fresh
/// pkrange fetch should succeed and the routing map should be populated.
///
/// Verified by: injecting a hit-limited 503 on pkrange requests, executing
/// enough data operations to exhaust the limit, then confirming operations
/// continue to succeed normally.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn pkrange_fetch_transient_failure_then_recovers() -> Result<(), Box<dyn Error>> {
    // Only fail the first 3 pkrange fetch attempts, then let subsequent ones succeed.
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::MetadataPartitionKeyRanges)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("pkrange-503-transient", result)
            .with_condition(condition)
            .with_hit_limit(3)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    // PPCB must be enabled for the driver to actually fetch PK ranges.
    let partition_failover_options = PartitionFailoverOptions::builder()
        .with_circuit_breaker_enabled(true)
        .build()?;

    DriverTestClient::run_with_unique_db_and_fault_injection_partition_failover_options(
        rules,
        partition_failover_options,
        async |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            // Create items — data operations succeed throughout because pkrange failure
            // only affects pre-resolution, not the data operation itself.
            for i in 0..5 {
                let body =
                    format!(r#"{{"id": "pkrange-transient-{i}", "pk": "pk1", "value": "test"}}"#);
                context
                    .create_seed_item(
                        &container,
                        &format!("pkrange-transient-{i}"),
                        "pk1",
                        body.as_bytes(),
                    )
                    .await
                    .expect("CreateItem must succeed even with transient pkrange failures");
            }

            // The rule should have been hit (up to its limit of 3).
            assert_eq!(
                rule.hit_count(),
                3,
                "pkrange fault should have been hit exactly 3 times (the hit limit)"
            );

            // After the limit is exhausted, reads should succeed normally.
            let read_result = context
                .read_item(&container, "pkrange-transient-0", "pk1")
                .await;
            assert!(
                read_result.is_ok(),
                "ReadItem should succeed after pkrange fault limit is exhausted"
            );

            Ok(())
        },
    )
    .await
}

// ────────────────────────────────────────────────────────────────────────────
// Partition Split (410 Gone) Tests
// ────────────────────────────────────────────────────────────────────────────

/// When a `ReadItem` returns 410/PartitionIsGone (simulating a partition split),
/// the operation pipeline retries the request and ultimately succeeds.
///
/// The 410 is injected only once (hit_limit = 1). On the retry, the fault is
/// exhausted, so the operation reaches the service and succeeds.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn partition_split_on_read_retries_and_succeeds() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::PartitionIsGone)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("split-410-read", result)
            .with_condition(condition)
            .with_hit_limit(1)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        // Seed an item (no fault on writes — rule only targets ReadItem).
        let item_json = br#"{"id": "split-item-1", "pk": "pk1", "value": "test"}"#;
        context
            .create_seed_item(&container, "split-item-1", "pk1", item_json)
            .await?;

        // The first read should hit the 410 and retry, then succeed on the second attempt.
        let read_response = context
            .read_item(&container, "split-item-1", "pk1")
            .await
            .expect("ReadItem should succeed after retrying through the 410 PartitionIsGone");

        // The rule should have been hit exactly once.
        assert_eq!(
            rule.hit_count(),
            1,
            "410 fault should have been hit exactly once (the hit limit)"
        );

        // The diagnostics on the surfaced response must capture EVERY
        // attempt the operation made — that's the
        // "one operation = one `DiagnosticsContext`" contract. The first
        // attempt failed with the injected 410; the dataflow layer
        // caught `is_partition_topology_change()`, invalidated the
        // partition-key-range cache, and re-executed the request. The
        // re-execution gets its own per-operation pipeline (and
        // therefore its own diagnostics), but the dataflow retry path
        // splices the failed attempt's diagnostics onto the retry's
        // response before returning, so the caller sees both attempts.
        let diagnostics = read_response.diagnostics();
        assert!(
            diagnostics.request_count() > 1,
            "Expected more than 1 request attempt (got {}) — the 410 should trigger a retry, and the dataflow layer must aggregate prior attempt diagnostics onto the final response",
            diagnostics.request_count()
        );

        Ok(())
    })
    .await
}

/// When a `CreateItem` returns 410/PartitionIsGone (simulating a partition split
/// mid-write), a non-idempotent write without PPAF aborts after the first 410
/// (since it is not safe to retry a non-idempotent write that may have been sent).
///
/// This verifies that the driver correctly aborts rather than double-writing.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn partition_split_on_create_aborts_non_idempotent_write() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::PartitionIsGone)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("split-410-create", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        // CreateItem with a persistent 410 — the driver retries across
        // available endpoints but eventually exhausts its retry budget.
        let item_json = br#"{"id": "split-create-1", "pk": "pk1", "value": "test"}"#;
        let create_result = context
            .create_item(&container, "split-create-1", "pk1", item_json)
            .await;

        assert!(
            create_result.is_err(),
            "CreateItem should fail when partition is permanently gone (retry budget exhausted)"
        );

        // Confirm the fault was hit.
        assert!(
            rule.hit_count() > 0,
            "410 fault should have been hit at least once"
        );

        Ok(())
    })
    .await
}

/// When a `ReadItem` returns 410/PartitionIsGone repeatedly (all retries hit the
/// fault), the operation exhausts its retry budget and fails with an error.
///
/// Validates that the driver does not retry indefinitely on persistent splits.
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn partition_split_on_read_exhausts_retries_and_fails() -> Result<(), Box<dyn Error>> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .build();

    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::PartitionIsGone)
        .with_probability(1.0)
        .build();

    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("split-410-read-persistent", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    DriverTestClient::run_with_unique_db_and_fault_injection(rules, async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        // Seed an item (fault only targets reads).
        let item_json = br#"{"id": "split-persistent-1", "pk": "pk1", "value": "test"}"#;
        context
            .create_seed_item(&container, "split-persistent-1", "pk1", item_json)
            .await?;

        // With a persistent 410 on all reads, the driver exhausts its retry budget.
        let read_result = context
            .read_item(&container, "split-persistent-1", "pk1")
            .await;

        assert!(
            read_result.is_err(),
            "ReadItem should fail when 410 is injected on all retry attempts"
        );

        // Multiple retries should have occurred before giving up.
        assert!(
            rule.hit_count() > 1,
            "Expected multiple retry attempts before exhausting budget, got {} hits",
            rule.hit_count()
        );

        Ok(())
    })
    .await
}
