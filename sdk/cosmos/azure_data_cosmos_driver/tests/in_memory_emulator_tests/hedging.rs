// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cross-region hedging integration tests using the in-memory emulator.
//!
//! Implements the initial subset of spec §15.2 ("Fault-injection / multi-region
//! tests") from `docs/HEDGING_SPEC.md`. This is the first driver-level test
//! coverage of the hedging stack: prior phases were covered by unit tests
//! against synthetic eligibility/race fixtures (§15.1).
//!
//! ## Harness model
//!
//! Each test:
//!
//! 1. Stands up a 2-region (East US + West US) single-master emulator account
//!    via the shared [`super::setup_multi_region`] helper.
//! 2. Wraps the emulator transport in `FaultInjectingHttpClientFactory` via
//!    `InMemoryEmulatorHttpClient::runtime_builder_with_fault_rules` so
//!    region-targeted errors can be injected without a real network.
//! 3. Configures the driver with `preferred_regions = [East US, West US]`
//!    and a per-operation `AvailabilityStrategy::Hedging(...)` threshold.
//! 4. Pre-seeds a known item via the gateway (which the emulator replicates
//!    synchronously across regions) and issues a `ReadItem` against the
//!    driver, inspecting the returned [`HedgeDiagnostics`].
//!
//! ## Tests included in this initial cut
//!
//! * [`hedging_no_failure_no_hedge_diagnostics`] — no fault injection,
//!   primary read returns 200 immediately. The current implementation only
//!   upgrades a *transient-failed* primary action to `OperationAction::Hedge`
//!   (see `maybe_upgrade_to_hedge` in `operation_pipeline.rs`); a successful
//!   primary therefore completes without ever entering `execute_hedged`,
//!   leaving `DiagnosticsContext::hedge_diagnostics() == None`. This test
//!   pins that behavior so the divergence from spec §15.2's
//!   `hedging_read_primary_fast` row (which expects `Some(_)` with
//!   `was_hedge=false`) is detected the moment a future change adds
//!   pre-failure timer-driven hedge dispatch.
//! * [`hedging_read_primary_503`] — 500 ms delay + 503 injected on East US
//!   `ReadItem` requests, threshold 100 ms. Path:
//!   1. The primary attempt eventually returns 503 → `FailoverRetry`
//!      classification.
//!   2. `maybe_upgrade_to_hedge` rewrites the action to `Hedge` and
//!      `execute_hedged` re-enters with both routings.
//!   3. The fresh primary attempt is delayed by the same fault rule and
//!      the alternate (West US) fires after the threshold and wins.
//!
//!   Diagnostics must show `was_hedge=true`, `total_requests_launched=2`,
//!   `response_region=Region::WEST_US`. The delay is required because the
//!   `execute_hedged` race honors §6.5 #3 — an instant primary error
//!   "wins" pre-threshold and is returned as the operation result without
//!   the alternate ever firing.
//!
//! ## Deferred follow-ups
//!
//! * `hedging_read_primary_slow` — needs pre-failure timer-driven hedge
//!   dispatch (spec §6.1 / §15.2 row 1). Not yet wired in the operation
//!   pipeline; see `hedging_no_failure_no_hedge_diagnostics` for the
//!   tracking assertion.
//! * `hedging_write_not_hedged`, `hedging_hub_region_header_propagates`,
//!   `hedging_disabled_per_operation`, `hedging_respects_deadline`, the
//!   §15.1 backfills (`app_cancel_preserves_hedge_diagnostics`, etc.) and
//!   the §15.3 live multi-region tests land in subsequent commits.

#![cfg(feature = "fault_injection")]

use std::sync::Arc;
use std::time::Duration;

use azure_core::http::{HttpClient, Url};
use azure_data_cosmos_driver::diagnostics::HedgeDiagnostics;
use azure_data_cosmos_driver::driver::CosmosDriver;
use azure_data_cosmos_driver::fault_injection::{
    FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
    FaultInjectionRule, FaultInjectionRuleBuilder, FaultOperationType,
};
use azure_data_cosmos_driver::in_memory_emulator::WriteMode;
use azure_data_cosmos_driver::models::{
    AccountReference, CosmosOperation, ItemReference, PartitionKey,
};
use azure_data_cosmos_driver::options::{
    AvailabilityStrategy, DriverOptions, HedgeThreshold, HedgingStrategy, OperationOptions,
    OperationOptionsBuilder, Region,
};

use super::{create_item_request, setup_multi_region, MultiRegionTestContext};

/// Endpoint used as the account's primary endpoint. Must match the East US
/// gateway URL synthesized by [`super::setup_multi_region`] so the metadata
/// probe finds the account.
const ACCOUNT_ENDPOINT: &str = "https://eastus.emulator.local";

/// Test database/container provisioned by `setup_multi_region`.
const DB_NAME: &str = "testdb";
const COLL_NAME: &str = "testcoll";

/// Seeds a known item into the emulator's store by issuing a raw POST to the
/// East US gateway URL — the same path the existing point-operations tests
/// use. The in-memory emulator's `ReplicationConfig::immediate()` replicates
/// the write synchronously across regions, so a read against either region's
/// gateway returns the same item.
async fn seed_item(ctx: &MultiRegionTestContext, item_id: &str, pk: &str) {
    let body = serde_json::json!({
        "id": item_id,
        "pk": pk,
        "value": 42,
    });
    let pk_header = format!(r#"["{pk}"]"#);
    let req = create_item_request(&ctx.east_url, DB_NAME, COLL_NAME, &body, &pk_header, false);
    let response = ctx
        .emulator
        .execute_request(&req)
        .await
        .expect("seed create returns a response");
    let status = u16::from(response.status());
    assert!(
        (200..300).contains(&status),
        "seed create should succeed; got HTTP {status}",
    );
}

/// Builds a driver wired to the multi-region emulator with the supplied
/// fault-injection rules and a hedging strategy at `threshold`.
async fn make_hedging_driver(
    ctx: &MultiRegionTestContext,
    threshold: Duration,
    rules: Vec<Arc<FaultInjectionRule>>,
) -> (Arc<CosmosDriver>, OperationOptions) {
    let runtime = ctx
        .emulator
        .runtime_builder_with_fault_rules(rules)
        .build()
        .await
        .expect("runtime builds");

    let account = AccountReference::with_master_key(
        Url::parse(ACCOUNT_ENDPOINT).expect("valid endpoint"),
        // Cosmos master keys are base64; the emulator does not actually
        // validate the signature so any well-formed base64 value works.
        "ZW11bGF0b3JrZXk=",
    );

    let driver_options = DriverOptions::builder(account.clone())
        .with_preferred_regions(vec![Region::EAST_US, Region::WEST_US])
        .build();

    let driver = runtime
        .get_or_create_driver(account, Some(driver_options))
        .await
        .expect("driver initializes against emulator metadata");

    let hedging_strategy = HedgingStrategy::new(
        HedgeThreshold::new(threshold).expect("hedge threshold must be non-zero"),
    );
    let op_options = OperationOptionsBuilder::new()
        .with_availability_strategy(AvailabilityStrategy::Hedging(hedging_strategy))
        .build();

    (driver, op_options)
}

/// Issues a `ReadItem` for `(item_id, pk)` against `driver` with the supplied
/// `OperationOptions`. Returns the response's optional `HedgeDiagnostics` —
/// callers assert on whether it should be `Some(_)` (hedge race ran) or
/// `None` (primary completed without hedging being entered).
async fn read_item_hedge_diagnostics(
    driver: &Arc<CosmosDriver>,
    op_options: OperationOptions,
    item_id: &str,
    pk: &str,
) -> Option<HedgeDiagnostics> {
    let container = driver
        .resolve_container_by_name(DB_NAME, COLL_NAME)
        .await
        .expect("container resolves");

    let item_ref = ItemReference::from_name(
        &container,
        PartitionKey::from(pk.to_owned()),
        item_id.to_owned(),
    );
    let operation = CosmosOperation::read_item(item_ref);

    let response = driver
        .execute_operation(operation, op_options)
        .await
        .expect("read_item succeeds");

    response.diagnostics().hedge_diagnostics().cloned()
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

/// Pins the current "primary succeeds → no hedging entered" behavior.
///
/// See module doc above for why this diverges from spec §15.2's
/// `hedging_read_primary_fast` row.
#[tokio::test]
async fn hedging_no_failure_no_hedge_diagnostics() {
    let ctx = setup_multi_region(WriteMode::Single).await;
    seed_item(&ctx, "fast-item", "pk1").await;

    let (driver, op_options) =
        make_hedging_driver(&ctx, Duration::from_millis(500), Vec::new()).await;

    let hedge_diag = read_item_hedge_diagnostics(&driver, op_options, "fast-item", "pk1").await;

    assert!(
        hedge_diag.is_none(),
        "current pipeline only attaches HedgeDiagnostics when `execute_hedged` \
         is entered (which requires a transient primary failure); got {hedge_diag:?}",
    );
}

/// Spec §15.2 — *alternate region wins after a primary 503*.
///
/// The 503 trips the primary's FailoverRetry classification;
/// `maybe_upgrade_to_hedge` rewrites the action to `Hedge`, and
/// `execute_hedged` races a fresh primary attempt (still failing) against
/// the West US alternate, which wins.
#[tokio::test]
async fn hedging_read_primary_503() {
    let ctx = setup_multi_region(WriteMode::Single).await;
    seed_item(&ctx, "item-503", "pk1").await;

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(Region::EAST_US)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        // The hedge race is between the primary and a timer; without a
        // delay the primary's 503 returns essentially instantly and
        // "wins" pre-threshold (§6.5 #3) — the alternate never fires.
        // The delay makes the primary slower than the threshold so the
        // alternate-region race actually runs.
        .with_delay(Duration::from_millis(500))
        .with_probability(1.0)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("hedging-east-us-503", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    let (driver, op_options) = make_hedging_driver(&ctx, Duration::from_millis(100), rules).await;

    let hedge_diag = read_item_hedge_diagnostics(&driver, op_options, "item-503", "pk1")
        .await
        .expect(
            "primary 503 should trigger the hedge upgrade — HedgeDiagnostics \
             must be attached (spec §10.1)",
        );

    assert!(
        hedge_diag.was_hedge,
        "alternate region should win the hedge race; diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.total_requests_launched, 2,
        "primary + alternate should both have been launched; diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.response_region,
        Region::WEST_US,
        "alternate (West US) should be the winning region; diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.regions_contacted,
        vec![Region::EAST_US, Region::WEST_US],
        "both regions should appear in the contacted-regions trail; diag={hedge_diag:?}",
    );
    assert!(
        rule.hit_count() >= 1,
        "the East US 503 rule should have been applied at least once",
    );
}
