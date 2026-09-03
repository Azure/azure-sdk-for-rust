// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cross-region metadata hedging integration tests using the in-memory emulator.
//!
//! Companion to `hedging.rs` (data-plane point-read hedging). These tests cover
//! the metadata cache reads that were widened into the hedging path — here, the
//! **Collection `Read`** (container metadata) read, which is directly observable
//! because it can be executed as a singleton operation and its `CosmosResponse`
//! carries the `HedgeDiagnostics`.
//!
//! ## Harness model
//!
//! Each test stands up a 2-region (East US primary + West US) single-master
//! emulator account via [`super::setup_multi_region`], wraps the transport in
//! fault injection, and executes `CosmosOperation::read_container_by_name`
//! directly against the driver so the container metadata read's own
//! `HedgeDiagnostics` can be inspected.
//!
//! ## Metadata-specific threshold
//!
//! Unlike data-plane reads (which use the configured / driver-default
//! threshold), the two metadata reads use a **fixed 1.5 s** hedge threshold
//! (matching .NET), regardless of the configured `AvailabilityStrategy`
//! threshold. The injected primary-region delay is therefore chosen well above
//! 1.5 s so the threshold reliably elapses and the alternate wins.

#![cfg(feature = "fault_injection")]

use std::sync::Arc;
use std::time::Duration;

use azure_core::http::Url;
use azure_data_cosmos_driver::diagnostics::{HedgeDiagnostics, HedgeTerminalState};
use azure_data_cosmos_driver::driver::CosmosDriver;
use azure_data_cosmos_driver::fault_injection::{
    FaultInjectionConditionBuilder, FaultInjectionResultBuilder, FaultInjectionRule,
    FaultInjectionRuleBuilder, FaultOperationType,
};
use azure_data_cosmos_driver::in_memory_emulator::WriteMode;
use azure_data_cosmos_driver::models::{AccountReference, CosmosOperation, DatabaseReference};
use azure_data_cosmos_driver::options::{
    AvailabilityStrategy, DriverOptions, HedgeThreshold, HedgingOptions, HedgingStrategy,
    OperationOptions, OperationOptionsBuilder, PartitionFailoverOptions, Region,
};

use super::{setup_multi_region, MultiRegionTestContext};

/// Endpoint used as the account's primary endpoint. Must match the East US
/// gateway URL synthesized by [`super::setup_multi_region`].
const ACCOUNT_ENDPOINT: &str = "https://eastus.emulator.local";
const ACCOUNT_KEY: &str = "ZW11bGF0b3JrZXk=";

/// Test database/container provisioned by `setup_multi_region`.
const DB_NAME: &str = "testdb";
const COLL_NAME: &str = "testcoll";

/// A primary-region metadata delay comfortably above the fixed 1.5 s metadata
/// hedge threshold, so the threshold reliably elapses and the alternate region
/// (which has no delay) wins the race.
const PRIMARY_METADATA_DELAY: Duration = Duration::from_millis(2500);

fn account() -> AccountReference {
    AccountReference::with_master_key(
        Url::parse(ACCOUNT_ENDPOINT).expect("valid endpoint"),
        // The emulator does not validate the signature; any base64 works.
        ACCOUNT_KEY,
    )
}

/// Builds a driver wired to the multi-region emulator with the supplied
/// fault-injection rules.
async fn make_driver(
    ctx: &MultiRegionTestContext,
    rules: Vec<Arc<FaultInjectionRule>>,
) -> Arc<CosmosDriver> {
    make_driver_with_hedging(ctx, rules, HedgingOptions::default()).await
}

/// Same as [`make_driver`], but with an explicit driver-wide hedging budget, so
/// a test can drive the budget-exhausted branch deterministically.
async fn make_driver_with_hedging(
    ctx: &MultiRegionTestContext,
    rules: Vec<Arc<FaultInjectionRule>>,
    hedging: HedgingOptions,
) -> Arc<CosmosDriver> {
    let runtime = ctx
        .emulator
        .runtime_builder_with_fault_rules(rules)
        .build()
        .await
        .expect("runtime builds");

    let driver_options = DriverOptions::builder(account())
        .with_preferred_regions(vec![Region::EAST_US, Region::WEST_US])
        .with_partition_failover_options(
            PartitionFailoverOptions::builder()
                .with_circuit_breaker_enabled(false)
                .build()
                .expect("partition failover options build"),
        )
        .with_hedging_options(hedging)
        .build();

    runtime
        .create_driver(driver_options)
        .await
        .expect("driver initializes against emulator metadata")
}

/// `AvailabilityStrategy::Hedging` op-options. The threshold value here is
/// irrelevant for metadata reads (they use the fixed 1.5 s metadata threshold);
/// only the enablement matters.
fn hedging_options() -> OperationOptions {
    let strategy = HedgingStrategy::new(
        HedgeThreshold::new(Duration::from_millis(500)).expect("non-zero threshold"),
    );
    OperationOptionsBuilder::new()
        .with_availability_strategy(AvailabilityStrategy::Hedging(strategy))
        .build()
}

fn disabled_options() -> OperationOptions {
    OperationOptionsBuilder::new()
        .with_availability_strategy(AvailabilityStrategy::Disabled)
        .build()
}

/// Builds a region-targeted response-delay rule on the Collection `Read`
/// metadata operation.
fn container_read_delay_rule(region: Region, delay: Duration) -> Arc<FaultInjectionRule> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::MetadataReadContainer)
        .with_region(region)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_delay(delay)
        .with_probability(1.0)
        .build();
    Arc::new(
        FaultInjectionRuleBuilder::new("metadata-container-read-delay", result)
            .with_condition(condition)
            .build(),
    )
}

/// Delays the partition-key-range ReadFeed in `region` past the metadata hedge
/// threshold. Doubles as a request counter for that region.
fn pk_range_delay_rule(region: Region, delay: Duration) -> Arc<FaultInjectionRule> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::MetadataPartitionKeyRanges)
        .with_region(region)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_delay(delay)
        .with_probability(1.0)
        .build();
    Arc::new(
        FaultInjectionRuleBuilder::new("metadata-pk-range-delay", result)
            .with_condition(condition)
            .build(),
    )
}

/// Zero-effect rule counting partition-key-range requests to `region`.
///
/// Probability 1.0 with no delay, error, or custom response: it increments its
/// hit count and forwards the request untouched, making it an exact per-region
/// request counter for an operation that surfaces no diagnostics to the caller.
fn pk_range_counter_rule(region: Region) -> Arc<FaultInjectionRule> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::MetadataPartitionKeyRanges)
        .with_region(region)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_probability(1.0)
        .build();
    Arc::new(
        FaultInjectionRuleBuilder::new("metadata-pk-range-counter", result)
            .with_condition(condition)
            .build(),
    )
}

/// Executes the container metadata read (`Collection` `Read`) directly and
/// returns the response's optional [`HedgeDiagnostics`].
async fn container_read_hedge_diagnostics(
    driver: &Arc<CosmosDriver>,
    op_options: OperationOptions,
) -> Option<HedgeDiagnostics> {
    let db_ref = DatabaseReference::from_name(account(), DB_NAME.to_owned());
    let operation = CosmosOperation::read_container_by_name(db_ref, COLL_NAME.to_owned());

    let response = driver
        .execute_singleton_operation(operation, op_options)
        .await
        .expect("container metadata read succeeds");

    response.diagnostics().hedge_diagnostics().cloned()
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

/// Enabled + primary metadata read slow past the (fixed 1.5 s) threshold ⇒ the
/// container metadata read is hedged and the alternate region, which has no
/// delay, wins.
#[tokio::test]
async fn metadata_container_read_hedges_when_primary_slow() {
    let ctx = setup_multi_region(WriteMode::Single).await;

    let rule = container_read_delay_rule(Region::EAST_US, PRIMARY_METADATA_DELAY);
    let driver = make_driver(&ctx, vec![Arc::clone(&rule)]).await;

    let hedge_diag = container_read_hedge_diagnostics(&driver, hedging_options())
        .await
        .expect(
            "a metadata Collection Read slow past the 1.5s threshold must enter \
             execute_hedged and attach HedgeDiagnostics",
        );

    assert_eq!(
        hedge_diag.terminal_state(),
        HedgeTerminalState::AlternateWon,
        "the alternate region, which has no delay, should win the metadata hedge race; \
         diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.alternate_region(),
        Some(&Region::WEST_US),
        "the alternate must be West US; diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.response_region(),
        Some(&Region::WEST_US),
        "the winning region must be the alternate (West US); diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.primary_region(),
        &Region::EAST_US,
        "primary_region must record the (slow, losing) East US; diag={hedge_diag:?}",
    );
    assert!(
        rule.hit_count() >= 1,
        "the East US metadata-read delay rule should have been applied at least once",
    );
}

/// Enabled + no fault ⇒ the primary metadata read wins pre-threshold and no
/// alternate is ever spawned (zero-overhead happy path).
#[tokio::test]
async fn metadata_container_read_primary_wins_when_fast() {
    let ctx = setup_multi_region(WriteMode::Single).await;
    let driver = make_driver(&ctx, Vec::new()).await;

    let hedge_diag = container_read_hedge_diagnostics(&driver, hedging_options())
        .await
        .expect(
            "hedging is enabled, so HedgeDiagnostics is attached even when the \
             primary wins pre-threshold",
        );

    assert_ne!(
        hedge_diag.terminal_state(),
        HedgeTerminalState::AlternateWon,
        "with no injected delay the primary must win; diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.alternate_region(),
        None,
        "no alternate should be launched on the zero-overhead happy path; \
         diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.response_region(),
        Some(&Region::EAST_US),
        "the primary (East US) should be the winning region; diag={hedge_diag:?}",
    );
}

/// Disabled + primary metadata read slow ⇒ the read is NOT hedged (no
/// HedgeDiagnostics attached) and still succeeds via the primary region.
#[tokio::test]
async fn metadata_container_read_not_hedged_when_disabled() {
    let ctx = setup_multi_region(WriteMode::Single).await;

    let rule = container_read_delay_rule(Region::EAST_US, PRIMARY_METADATA_DELAY);
    let driver = make_driver(&ctx, vec![Arc::clone(&rule)]).await;

    let hedge_diag = container_read_hedge_diagnostics(&driver, disabled_options()).await;

    assert!(
        hedge_diag.is_none(),
        "with AvailabilityStrategy::Disabled the metadata read must not hedge, so \
         no HedgeDiagnostics should be attached; diag={hedge_diag:?}",
    );
}

/// Enabled + primary slow, but the client's concurrent-metadata-hedge budget is
/// exhausted ⇒ the read is NOT upgraded into a hedge race. It falls through to
/// the ordinary sequential path and still succeeds against the slow primary.
///
/// This is the amplification guardrail: during a region brownout every metadata
/// read crosses the threshold at once, and without a ceiling each one would
/// spawn an extra request into the alternate region — exactly when that region
/// is least able to absorb it.
#[tokio::test]
async fn metadata_container_read_not_hedged_when_budget_exhausted() {
    let ctx = setup_multi_region(WriteMode::Single).await;

    let rule = container_read_delay_rule(Region::EAST_US, PRIMARY_METADATA_DELAY);
    // A budget of zero refuses every metadata hedge, as an exhausted one would.
    let driver = make_driver_with_hedging(
        &ctx,
        vec![Arc::clone(&rule)],
        HedgingOptions::builder()
            .with_max_concurrent_metadata_attempts(0)
            .build(),
    )
    .await;

    let hedge_diag = container_read_hedge_diagnostics(&driver, hedging_options()).await;

    assert!(
        hedge_diag.is_none(),
        "an exhausted hedge budget must skip the hedge upgrade entirely, so the \
         operation never enters execute_hedged and carries no HedgeDiagnostics; \
         diag={hedge_diag:?}",
    );
    assert!(
        rule.hit_count() >= 1,
        "the read must still have been served (slowly) by the primary region",
    );
}

/// A finished race returns its slot: with a budget of exactly one, two
/// *sequential* reads both hedge. Proves the permit is dropped when the race
/// ends rather than leaking for the lifetime of the driver — a leak here would
/// silently disable hedging after the first N operations.
#[tokio::test]
async fn metadata_hedge_budget_slot_is_released_after_each_race() {
    let ctx = setup_multi_region(WriteMode::Single).await;

    let rule = container_read_delay_rule(Region::EAST_US, PRIMARY_METADATA_DELAY);
    let driver = make_driver_with_hedging(
        &ctx,
        vec![Arc::clone(&rule)],
        HedgingOptions::builder()
            .with_max_concurrent_metadata_attempts(1)
            .build(),
    )
    .await;

    for round in 1..=2 {
        let hedge_diag = container_read_hedge_diagnostics(&driver, hedging_options())
            .await
            .unwrap_or_else(|| {
                panic!(
                    "round {round}: the single budget slot must be free again, so this \
                     read should hedge",
                )
            });
        assert_eq!(
            hedge_diag.terminal_state(),
            HedgeTerminalState::AlternateWon,
            "round {round}: the alternate region, which has no delay, should win; \
             diag={hedge_diag:?}",
        );
    }
}

/// Partition-key-range ReadFeed: a **cold** chain hedges, and every page after
/// it stays pinned to the region that won.
///
/// PK-range fetches are driven internally by the cache and surface no
/// diagnostics to the caller, so hedge activity is observed with per-region
/// zero-effect counter rules (probability 1.0, no delay / error / body — they
/// count the request and forward it untouched).
///
/// The second half is the important one. The ETag a PK-range page returns is
/// only meaningful to the region that issued it, so once West wins the cold
/// chain the continuation is region-affine to West. A forced refresh must
/// therefore go back to West *and not hedge* — a hedge leg would carry West's
/// ETag into East, where it means nothing.
#[tokio::test]
async fn metadata_pk_range_read_hedges_cold_then_pins_to_the_winner() {
    let ctx = setup_multi_region(WriteMode::Single).await;

    let east_pk = pk_range_delay_rule(Region::EAST_US, PRIMARY_METADATA_DELAY);
    let west_pk = pk_range_counter_rule(Region::WEST_US);
    let driver = make_driver(&ctx, vec![Arc::clone(&east_pk), Arc::clone(&west_pk)]).await;

    let container = driver
        .resolve_container_by_name(DB_NAME, COLL_NAME, OperationOptions::default())
        .await
        .expect("container resolves");

    // Cold chain: the PK-range cache is empty, so this carries no continuation
    // and is hedge-eligible. East stalls past the 1.5 s metadata threshold.
    driver
        .resolve_all_partition_key_ranges(&container, false)
        .await
        .expect("cold partition key range fetch succeeds")
        .expect("cold partition key range fetch returns topology");

    assert!(
        east_pk.hit_count() >= 1,
        "the cold chain must have been attempted against the primary (East US) first",
    );
    assert!(
        west_pk.hit_count() >= 1,
        "the cold chain stalled past the 1.5s metadata threshold, so it must have \
         hedged into the alternate region (West US)",
    );

    let east_after_cold = east_pk.hit_count();
    let west_after_cold = west_pk.hit_count();

    // Forced refresh: resumes from West's region-affine continuation, so it must
    // be pinned to West with hedging suppressed.
    driver
        .resolve_all_partition_key_ranges(&container, true)
        .await
        .expect("forced partition key range refresh succeeds")
        .expect("forced partition key range refresh returns topology");

    assert_eq!(
        east_pk.hit_count(),
        east_after_cold,
        "a refresh resuming West's continuation must not reach East US: it is \
         pinned to the region that issued the ETag, and pinning also suppresses \
         hedging so no alternate leg is spawned",
    );
    assert!(
        west_pk.hit_count() > west_after_cold,
        "the refresh must still have run — against West US, the pinned region",
    );
}
