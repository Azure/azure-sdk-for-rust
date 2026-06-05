// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Multi-region live-account integration tests for cross-region request
//! hedging (`docs/HEDGING_SPEC.md`).
//!
//! **Prereqs**: ≥2-region read account. The `test-resources.bicep` template
//! provisions East US 2 + West US 3 when `enableMultipleRegions=true`. A
//! third region is required to exercise §15.3 `hedging_read_cross_region`
//! and the §6.7 fallback path; the two tests below run on the 2-region
//! topology.
//!
//! These tests use fault injection to introduce a deterministic delay on
//! reads in the primary region. The alternate region — selected by the
//! hedging eligibility evaluator from the driver's `preferred_regions` —
//! produces the winning response, and the returned `HedgeDiagnostics`
//! must classify the race per the §10.1 / §10.1.1 terminal-state taxonomy.
//!
//! Gated by `test_category = "multi_region"` — requires a live multi-region
//! Cosmos DB account, not the local emulator.

#![cfg(feature = "fault_injection")]

use crate::framework::DriverTestClient;
use azure_data_cosmos_driver::diagnostics::HedgeTerminalState;
use azure_data_cosmos_driver::fault_injection::{
    FaultInjectionConditionBuilder, FaultInjectionResultBuilder, FaultInjectionRuleBuilder,
    FaultOperationType,
};
use azure_data_cosmos_driver::options::{
    AvailabilityStrategy, HedgeThreshold, HedgingStrategy, OperationOptions,
    OperationOptionsBuilder, Region,
};
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

/// Primary region — the first entry in the `preferred_regions` list passed
/// to the driver. The hedging evaluator targets this region for the primary
/// leg and selects the next eligible entry as the alternate.
const PRIMARY_REGION: Region = Region::NORTH_CENTRAL_US;

/// Alternate region — second entry in `preferred_regions`. Spec §6.2:
/// `regions[1]` is the deterministic alternate-region target.
const ALTERNATE_REGION: Region = Region::EAST_US_2;

/// Threshold short enough that any added delay on the primary leg fires
/// the alternate-region hedge well before the operation deadline. Per
/// §5.2 the driver default is `min(1000ms, request_timeout / 2)`; we use
/// 200 ms explicitly so the test does not depend on `request_timeout`.
const HEDGE_THRESHOLD: Duration = Duration::from_millis(200);

/// Delay injected on the primary leg's read — comfortably above
/// `HEDGE_THRESHOLD` and below the default end-to-end timeout, so the
/// alternate always wins the race and the operation completes within
/// the deadline.
const PRIMARY_READ_DELAY: Duration = Duration::from_millis(1500);

/// Builds a runtime-level [`OperationOptions`] that enables hedging with a
/// custom threshold. Tests pass this into the harness so the
/// driver-runtime layer of the option-resolution view
/// (per `docs/HierarchicalConfigModel.md`) carries the strategy for every
/// operation issued by the test.
fn hedging_operation_options(threshold: Duration) -> OperationOptions {
    let threshold = HedgeThreshold::new(threshold)
        .expect("threshold must be non-zero per HedgeThreshold::new contract");
    let strategy = AvailabilityStrategy::Hedging(HedgingStrategy::new(threshold));
    OperationOptionsBuilder::new()
        .with_availability_strategy(strategy)
        .build()
}

// ────────────────────────────────────────────────────────────────────────────
// Test 1: primary slow → alternate wins the race.
// ────────────────────────────────────────────────────────────────────────────

/// Read latency on the primary region exceeds the hedge threshold → the
/// alternate region must produce the winning response.
///
/// Validates the end-to-end §6.1 race + §10.1 diagnostics shape against a
/// real account: the diagnostics returned to the caller must record
/// `terminal_state == AlternateWon` and
/// `response_region == ALTERNATE_REGION`.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_region"),
    ignore = "requires test_category 'multi_region'"
)]
pub async fn hedging_primary_slow_alternate_wins() -> Result<(), Box<dyn Error>> {
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new(
            "hedging-primary-slow",
            FaultInjectionResultBuilder::new()
                .with_delay(PRIMARY_READ_DELAY)
                .with_probability(1.0)
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::ReadItem)
                .with_region(PRIMARY_REGION)
                .build(),
        )
        .build(),
    );

    DriverTestClient::run_with_unique_db_and_hedging(
        vec![Arc::clone(&rule)],
        hedging_operation_options(HEDGE_THRESHOLD),
        vec![PRIMARY_REGION, ALTERNATE_REGION],
        async |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            // Seed an item to read. The seeding write itself is not hedged
            // (writes are §5.1-excluded) and is unaffected by the
            // read-only fault rule above.
            let item_id = "hedge-item-1";
            let body = format!(r#"{{"id": "{item_id}", "pk": "pk1", "value": "hedge-test"}}"#);
            context
                .create_item_with_pk(&container, "pk1", body.as_bytes())
                .await?;

            let response = context
                .read_item(&container, item_id, "pk1")
                .await
                .expect("read should succeed via the hedged alternate region");

            let diagnostics = response.diagnostics();
            let hedge_diag = diagnostics.hedge_diagnostics().expect(
                "hedge diagnostics must be attached when hedging is configured \
                 and at least one leg returns a Final response (spec §10.1)",
            );

            assert_eq!(
                hedge_diag.terminal_state(),
                HedgeTerminalState::AlternateWon,
                "alternate must win when the primary is delayed past the threshold; \
                 hedge_diag = {hedge_diag:?}",
            );
            assert!(
                matches!(
                    hedge_diag.terminal_state(),
                    HedgeTerminalState::AlternateWon
                ),
                "alternate-won invariant: terminal_state must be AlternateWon \
                 (spec §10.1); hedge_diag = {hedge_diag:?}",
            );
            assert_eq!(
                hedge_diag.response_region(),
                Some(&ALTERNATE_REGION),
                "winning response must come from the alternate region; \
                 hedge_diag = {hedge_diag:?}",
            );
            assert_eq!(
                hedge_diag.primary_region(),
                &PRIMARY_REGION,
                "primary_region must record the region that lost the race; \
                 hedge_diag = {hedge_diag:?}",
            );
            assert_eq!(
                hedge_diag.alternate_region(),
                Some(&ALTERNATE_REGION),
                "alternate_region must be populated once the threshold elapses \
                 and a hedge is spawned; hedge_diag = {hedge_diag:?}",
            );

            assert!(
                rule.hit_count() > 0,
                "fault rule should have fired at least once on the primary region's read",
            );

            Ok(())
        },
    )
    .await
}

// ────────────────────────────────────────────────────────────────────────────
// Test 2: primary fast → zero-overhead happy path, no alternate spawned.
// ────────────────────────────────────────────────────────────────────────────

/// No fault injection → the primary returns within the threshold window and
/// the alternate is never spawned.
///
/// `HedgeDiagnostics` is still attached on the zero-overhead happy path
/// (spec §10.1: a synthetic "primary-only" record lets callers distinguish
/// *"hedging was selected but never fanned out"* from *"hedging was not
/// selected"*). The terminal state must classify as
/// `PrimaryWonPreThreshold` and only the primary leg launched.
#[tokio::test]
#[cfg_attr(
    not(test_category = "multi_region"),
    ignore = "requires test_category 'multi_region'"
)]
pub async fn hedging_primary_fast_no_alternate() -> Result<(), Box<dyn Error>> {
    DriverTestClient::run_with_unique_db_and_hedging(
        vec![],
        hedging_operation_options(HEDGE_THRESHOLD),
        vec![PRIMARY_REGION, ALTERNATE_REGION],
        async |context, database| {
            let container_name = context.unique_container_name();
            let container = context
                .create_container(&database, &container_name, "/pk")
                .await?;

            let item_id = "hedge-item-fast";
            let body = format!(r#"{{"id": "{item_id}", "pk": "pk1", "value": "fast"}}"#);
            context
                .create_item_with_pk(&container, "pk1", body.as_bytes())
                .await?;

            let response = context.read_item(&container, item_id, "pk1").await?;
            let diagnostics = response.diagnostics();
            let hedge_diag = diagnostics.hedge_diagnostics().expect(
                "hedge diagnostics must be attached even on the zero-overhead happy \
                 path so consumers can distinguish hedge-selected-but-not-fanned-out \
                 from hedge-not-selected (spec §10.1)",
            );

            assert_eq!(
                hedge_diag.terminal_state(),
                HedgeTerminalState::PrimaryWonPreThreshold,
                "healthy account → primary must win before the threshold fires; \
                 hedge_diag = {hedge_diag:?}",
            );
            assert!(
                !matches!(
                    hedge_diag.terminal_state(),
                    HedgeTerminalState::AlternateWon
                ),
                "terminal_state must NOT be AlternateWon when no alternate produced the response; \
                 hedge_diag = {hedge_diag:?}",
            );
            assert_eq!(
                hedge_diag.alternate_region(),
                None,
                "only the primary leg should be launched on the zero-overhead happy \
                 path (spec §6.5 invariant #3); hedge_diag = {hedge_diag:?}",
            );
            assert_eq!(
                hedge_diag.response_region(),
                Some(&PRIMARY_REGION),
                "primary owns the response on the zero-overhead happy path; \
                 hedge_diag = {hedge_diag:?}",
            );

            Ok(())
        },
    )
    .await
}
