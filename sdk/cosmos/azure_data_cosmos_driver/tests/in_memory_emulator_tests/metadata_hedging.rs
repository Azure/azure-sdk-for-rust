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
    AvailabilityStrategy, DriverOptions, HedgeThreshold, HedgingStrategy, OperationOptions,
    OperationOptionsBuilder, PartitionFailoverOptions, Region,
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
/// hedge threshold, so the threshold reliably elapses and the (undelayed)
/// alternate region wins the race.
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
/// container metadata read is hedged and the undelayed alternate region wins.
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
        "the undelayed alternate region should win the metadata hedge race; \
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
