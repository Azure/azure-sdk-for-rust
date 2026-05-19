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
//! ## Tests included
//!
//! All three rows below come from spec §15.2 ("Integration Tests — Fault
//! Injection"). They cover the three primary outcomes the timer-driven
//! race can produce:
//!
//! * [`hedging_read_primary_fast`] — no fault injection, primary returns
//!   200 immediately, threshold 500 ms. The primary wins the
//!   primary-vs-threshold race in `execute_hedged` (§6.5 #3 zero-overhead
//!   happy path), and `HedgeDiagnostics::primary_only` is attached per the
//!   §10.1 attachment contract — `was_hedge=false`,
//!   `total_requests_launched=1`, `response_region=EAST_US`. This test
//!   distinguishes *"hedging was active and the primary won"* from
//!   *"hedging was not selected for this operation"*; both cases would
//!   produce a successful read without it.
//! * [`hedging_read_primary_slow`] — 800 ms delay (no error) on East US
//!   `ReadItem`, threshold 100 ms. The primary attempt is still in flight
//!   when the threshold elapses; `execute_hedged` spawns the alternate
//!   against West US and the alternate wins by virtue of having no delay
//!   injected. Diagnostics show `was_hedge=true`,
//!   `total_requests_launched=2`, `response_region=WEST_US` — the spec's
//!   canonical "tail-latency cut" outcome.
//! * [`hedging_read_primary_503`] — 500 ms delay + 503 injected on East US
//!   `ReadItem`, threshold 100 ms. Same primary-vs-alternate race shape as
//!   the slow case, but the primary's eventual result is a transient
//!   failure; the alternate still wins. Diagnostics identical to the slow
//!   case. The 500 ms delay is load-bearing: without it the primary's
//!   instant 503 would win pre-threshold per §6.5 #3 (a final-classified
//!   error counts as a primary win for race purposes).
//!
//! ## Deferred follow-ups
//!
//! * `hedging_read_both_regions_slow`, `hedging_write_not_hedged`,
//!   `hedging_hub_region_header_propagates`,
//!   `hedging_disabled_per_operation`, `hedging_respects_deadline`, the
//!   remaining §15.1 backfills (`app_cancel_preserves_hedge_diagnostics`,
//!   etc.) and the §15.3 live multi-region tests land in subsequent
//!   commits.

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

/// Spec §15.2 row 2 — *primary wins pre-threshold, primary-only
/// diagnostics attached*.
///
/// No faults. Primary returns immediately; the primary-vs-threshold race
/// inside `execute_hedged` resolves on the primary side before the
/// secondary is ever constructed (§6.5 #3 zero-overhead happy path).
/// Per the §10.1 attachment contract, `HedgeDiagnostics::primary_only` is
/// still attached so callers can distinguish *"hedging was active and
/// the primary won"* from *"hedging was not selected"*.
#[tokio::test]
async fn hedging_read_primary_fast() {
    let ctx = setup_multi_region(WriteMode::Single).await;
    seed_item(&ctx, "fast-item", "pk1").await;

    // Threshold deliberately well above the in-memory emulator's response
    // latency so the primary always wins pre-threshold.
    let (driver, op_options) =
        make_hedging_driver(&ctx, Duration::from_millis(500), Vec::new()).await;

    let hedge_diag = read_item_hedge_diagnostics(&driver, op_options, "fast-item", "pk1")
        .await
        .expect(
            "hedging strategy was resolved and `execute_hedged` was entered; \
             `HedgeDiagnostics::primary_only` must be attached per the §10.1 \
             attachment contract",
        );

    assert!(
        !hedge_diag.was_hedge,
        "primary should win pre-threshold; diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.total_requests_launched, 1,
        "only the primary should have been launched (§6.5 #3 zero-overhead \
         happy path); diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.response_region,
        Region::EAST_US,
        "primary (East US) should be the winning region; diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.regions_contacted,
        vec![Region::EAST_US],
        "only the primary region should appear in the contacted-regions \
         trail; diag={hedge_diag:?}",
    );
}

/// Spec §15.2 row 1 — *primary slow (no error), alternate wins*.
///
/// 800 ms delay (no error) injected on East US `ReadItem`, threshold
/// 100 ms. The primary is still in flight when the threshold elapses;
/// `execute_hedged` spawns the West US alternate, which has no delay
/// and wins. This is the canonical tail-latency-cut scenario the spec
/// was designed for.
#[tokio::test]
async fn hedging_read_primary_slow() {
    let ctx = setup_multi_region(WriteMode::Single).await;
    seed_item(&ctx, "slow-item", "pk1").await;

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(Region::EAST_US)
        .build();
    // Delay only — no error. The primary will eventually succeed with a
    // 200, but only after the alternate has long since won the race.
    let result = FaultInjectionResultBuilder::new()
        .with_delay(Duration::from_millis(800))
        .with_probability(1.0)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("hedging-east-us-delay", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    let (driver, op_options) = make_hedging_driver(&ctx, Duration::from_millis(100), rules).await;

    let hedge_diag = read_item_hedge_diagnostics(&driver, op_options, "slow-item", "pk1")
        .await
        .expect(
            "slow primary should cause the threshold to elapse and \
             `execute_hedged` to spawn the alternate — `HedgeDiagnostics` \
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
        "both regions should appear in the contacted-regions trail; \
         diag={hedge_diag:?}",
    );
    assert!(
        rule.hit_count() >= 1,
        "the East US delay rule should have been applied at least once",
    );
}

/// Spec §15.2 row 3 — *primary returns 503 (delayed), alternate wins*.
///
/// 500 ms delay + 503 injected on East US `ReadItem`, threshold 100 ms.
/// The delay is load-bearing: an instant 503 would win the primary side
/// of the race pre-threshold per §6.5 #3 (a final-classified error
/// counts as a primary completion for race purposes), so the alternate
/// would never spawn. With the delay, the threshold elapses while the
/// primary is still in flight, the alternate spawns against West US, and
/// the alternate's success wins over the primary's transient 503.
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
