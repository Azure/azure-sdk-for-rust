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
//! All rows below come from spec §15.2 ("Integration Tests — Fault
//! Injection"). The first three (`hedging_read_primary_fast`,
//! `hedging_read_primary_slow`, `hedging_read_primary_503`) cover the
//! three primary outcomes the timer-driven race can produce; the
//! remainder cover behavior-absence, race dynamics under graceful
//! degradation, PPCB composition, and the §8.4 exclude-regions
//! invariant under fault-injected transport retries.
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
//! * [`hedging_write_not_hedged`] — multi-master account, 500 ms delay on
//!   East US `CreateItem`, threshold 100 ms. Writes are excluded from
//!   hedging per §5.1 row 4 / §1 Non-Goals regardless of account topology;
//!   `execute_hedged` is never entered and no diagnostics are attached.
//! * [`hedging_disabled_per_operation`] — per-operation
//!   `AvailabilityStrategy::Disabled` overrides the driver default
//!   (§11.3.1). With a 500 ms primary delay and no per-op hedging, the
//!   operation observes the full delay and no alternate spawns.
//! * [`hedging_respects_deadline`] — `EndToEndOperationLatencyPolicy` of
//!   1 s combined with a 3 s hedge threshold and a 1.5 s primary delay:
//!   the deadline fires before the threshold elapses, so the operation
//!   returns a timeout error without ever spawning the alternate.
//! * [`hedging_read_both_regions_slow`] — 500 ms delay on East US, 700 ms
//!   delay on West US, threshold 100 ms. Both regions race; whichever
//!   completes first wins (graceful degradation). The shorter delay on
//!   East US makes the primary win even though it started first.
//! * [`hedging_cancels_loser`] — 800 ms delay on East US, no fault on
//!   West US, threshold 100 ms. The alternate (West US) wins; once the
//!   race resolves, the East US loser's transport task must be cancelled
//!   structurally (drop-the-future). After a short settling delay, the
//!   East US fault rule's `hit_count` must equal exactly 1 — proving the
//!   loser was cancelled mid-flight rather than retried or duplicated.
//! * [`hedging_failback_to_primary`] — 500 ms delay on East US for the
//!   first 5 reads (via `with_hit_limit(5)`), then no delay. The first 5
//!   reads hedge to West US; the 6th read's primary returns fast and
//!   wins pre-threshold (`was_hedge=false`). Demonstrates that transient
//!   cross-region latency does not accumulate over time.
//! * [`hedging_with_ppcb_existing_failures`] — PPCB enabled at the driver
//!   level, hedging enabled per-operation, single 503 + 500 ms delay
//!   injected on East US (below the PPCB read-failure threshold of 10).
//!   Verifies hedging fires normally despite PPCB being active for the
//!   same partition — they compose without interference (§9.4 vs §6).
//! * [`hedging_alternate_wins_trip_ppcb`] — PPCB enabled, 10 consecutive
//!   reads with 500 ms delay + 503 on East US drive ≥ 5 alternate-region
//!   wins on the same partition; [`record_hedge_alternate_win`] installs
//!   an `Unhealthy` entry in `circuit_breaker_overrides` with
//!   `read_failure_count` seeded to `config.read_failure_threshold` so
//!   [`can_circuit_breaker_trigger_failover`] accepts it immediately.
//!   The 11th read (no fault) resolves the primary to West US via the
//!   override and wins pre-threshold (`was_hedge=false`,
//!   `response_region=West US`) — the visible end-user signal that the
//!   hedge-driven trip actually redirects routing per §9.5.
//! * [`hedging_exclude_regions_under_503_retry`] — 3-region setup
//!   (East US write, West US + Central US read), user excludes Central
//!   US via `ExcludeRegions`, 500 ms delay on East US (forces hedge),
//!   503 on West US (triggers transport retry inside the alternate).
//!   The alternate hedge's retry stays pinned to West US instead of
//!   falling back to the healthy Central US — the §8.4 cross-cutting
//!   "local-only retries inside a hedge" invariant under fault
//!   injection.
//!
//! ## Deferred follow-ups
//!
//! * `hedging_hub_region_header_propagates_across_hedges` (§15.2 row 12) —
//!   needs a [`RequestObserver`](azure_data_cosmos_driver::in_memory_emulator::RequestObserver)
//!   plumbed through [`super::setup_multi_region`] so the test can assert on
//!   headers received per region. Deferred until that helper variant lands.
//! * Remaining §15.1 unit-test backfills
//!   (`app_cancel_preserves_hedge_diagnostics`,
//!   `exclude_regions_honored_by_every_retry_trigger`,
//!   `zero_overhead_happy_path_no_allocs`, and the four
//!   shared-hub-region-latch lifecycle tests) and the §15.3 live
//!   multi-region tests land in subsequent commits.

#![cfg(feature = "fault_injection")]

use std::sync::Arc;
use std::time::Duration;

use azure_core::http::Url;
use azure_data_cosmos_driver::diagnostics::{HedgeDiagnostics, HedgeTerminalState};
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
    AvailabilityStrategy, DriverOptions, EndToEndOperationLatencyPolicy, ExcludedRegions,
    HedgeThreshold, HedgingStrategy, OperationOptions, OperationOptionsBuilder, Region,
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
        .expect("read_item succeeds")
        .expect("read_item returns a response body");

    response.diagnostics().hedge_diagnostics().cloned()
}

/// Variant of [`read_item_hedge_diagnostics`] that returns the full result
/// instead of unwrapping it — for tests asserting on error conditions
/// (e.g., deadline exceeded) and for negative-path coverage where the
/// driver is expected to fail.
async fn read_item_result(
    driver: &Arc<CosmosDriver>,
    op_options: OperationOptions,
    item_id: &str,
    pk: &str,
) -> Result<
    azure_data_cosmos_driver::models::CosmosResponse,
    azure_data_cosmos_driver::error::CosmosError,
> {
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
    driver
        .execute_operation(operation, op_options)
        .await
        .map(|maybe| maybe.expect("read_item returns a response body"))
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
        hedge_diag.terminal_state,
        HedgeTerminalState::PrimaryWonPreThreshold,
        "zero-overhead happy path → terminal state must classify as \
         PrimaryWonPreThreshold (spec §10.1.1); diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.alternate_region, None,
        "only the primary should have been launched (§6.5 #3 zero-overhead \
         happy path); diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.response_region,
        Region::EAST_US,
        "primary (East US) should be the winning region; diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.primary_region,
        Region::EAST_US,
        "primary_region must record East US on the zero-overhead happy \
         path; diag={hedge_diag:?}",
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
        hedge_diag.terminal_state,
        HedgeTerminalState::AlternateWon,
        "alternate must win when primary is slow past threshold → terminal \
         state must classify as AlternateWon (spec §10.1.1); diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.alternate_region,
        Some(Region::WEST_US),
        "primary + alternate should both have been launched; diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.response_region,
        Region::WEST_US,
        "alternate (West US) should be the winning region; diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.primary_region,
        Region::EAST_US,
        "primary_region must record East US (the losing region); \
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
        hedge_diag.terminal_state,
        HedgeTerminalState::AlternateWon,
        "delayed-503 primary + healthy alternate must classify as \
         AlternateWon (spec §10.1.1); diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.alternate_region,
        Some(Region::WEST_US),
        "primary + alternate should both have been launched; diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.response_region,
        Region::WEST_US,
        "alternate (West US) should be the winning region; diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.primary_region,
        Region::EAST_US,
        "primary_region must record East US (the delayed-503 leg); \
         diag={hedge_diag:?}",
    );
    assert!(
        rule.hit_count() >= 1,
        "the East US 503 rule should have been applied at least once",
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Behavior-absence tests (cluster 1)
// ─────────────────────────────────────────────────────────────────────────────

/// Spec §15.2 row 5 — *writes are never hedged, even on multi-master*.
///
/// Writes are excluded from hedging by spec rule (§1 Non-Goals, §5.1 row 4)
/// regardless of account topology, so a `CreateItem` against a multi-master
/// account with a slow primary must observe the full primary delay rather
/// than racing the alternate. `should_hedge` rejects the operation up-front
/// and `execute_hedged` is never entered — so `hedge_diagnostics()` is
/// `None`.
#[tokio::test]
async fn hedging_write_not_hedged() {
    let ctx = setup_multi_region(WriteMode::Multi).await;

    // Inject a 500 ms delay on East US `CreateItem` only. A multi-master
    // account in principle could route writes to either region, but the
    // driver's preferred-region order keeps the first attempt on East US.
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .with_region(Region::EAST_US)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_delay(Duration::from_millis(500))
        .with_probability(1.0)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("hedging-east-us-create-delay", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    // 100 ms threshold; if writes were hedged the alternate would beat the
    // primary by ~400 ms. The assertion below catches both spuriously-attached
    // diagnostics and the slow-completion regression.
    let (driver, op_options) = make_hedging_driver(&ctx, Duration::from_millis(100), rules).await;

    let container = driver
        .resolve_container_by_name(DB_NAME, COLL_NAME)
        .await
        .expect("container resolves");
    let item_ref = ItemReference::from_name(&container, PartitionKey::from("pk1"), "wpk-1");
    let body = b"{\"id\":\"wpk-1\",\"pk\":\"pk1\",\"v\":1}".to_vec();
    let operation = CosmosOperation::create_item(item_ref).with_body(body);

    let start = std::time::Instant::now();
    let response = driver
        .execute_operation(operation, op_options)
        .await
        .expect("create_item succeeds");
    let elapsed = start.elapsed();

    // Writes with `ContentResponseOnWrite::Disabled` return `Ok(None)`; tests
    // that exercise the write path through `execute_operation` therefore see
    // an `Option<CosmosResponse>` and must inspect diagnostics through the
    // pre-built body when present. Hedging never engages on writes (§1
    // Non-Goals), so when the response is omitted there can be no hedge
    // diagnostics either — the assertion below collapses to a trivial
    // `true` in that case.
    if let Some(response) = response {
        assert!(
            response.diagnostics().hedge_diagnostics().is_none(),
            "writes must never enter `execute_hedged` (§1 Non-Goals); \
             hedge_diagnostics={:?}",
            response.diagnostics().hedge_diagnostics(),
        );
    }
    assert!(
        elapsed >= Duration::from_millis(400),
        "without hedging the primary must observe the full ~500 ms delay; \
         elapsed={elapsed:?}",
    );
    assert_eq!(
        rule.hit_count(),
        1,
        "exactly one CreateItem attempt should have hit the East US delay; \
         hit_count={}",
        rule.hit_count(),
    );
}

/// Spec §15.2 row 6 — *per-operation `Disabled` overrides driver default*.
///
/// The driver's default availability strategy (§5.2) would normally hedge
/// reads on a multi-region account, but an explicit per-operation
/// `AvailabilityStrategy::Disabled` short-circuits the resolver
/// (§11.3.1 priority 1). `should_hedge` returns false, `execute_hedged`
/// is never entered, and the read observes the full primary delay.
#[tokio::test]
async fn hedging_disabled_per_operation() {
    let ctx = setup_multi_region(WriteMode::Single).await;
    seed_item(&ctx, "disabled-item", "pk1").await;

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(Region::EAST_US)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_delay(Duration::from_millis(500))
        .with_probability(1.0)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("hedging-east-us-disabled-delay", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    let runtime = ctx
        .emulator
        .runtime_builder_with_fault_rules(rules)
        .build()
        .await
        .expect("runtime builds");
    let account = AccountReference::with_master_key(
        Url::parse(ACCOUNT_ENDPOINT).expect("valid endpoint"),
        "ZW11bGF0b3JrZXk=",
    );
    let driver_options = DriverOptions::builder(account.clone())
        .with_preferred_regions(vec![Region::EAST_US, Region::WEST_US])
        .build();
    let driver = runtime
        .get_or_create_driver(account, Some(driver_options))
        .await
        .expect("driver initializes against emulator metadata");

    // Per-operation Disabled overrides any driver default.
    let op_options = OperationOptionsBuilder::new()
        .with_availability_strategy(AvailabilityStrategy::Disabled)
        .build();

    let start = std::time::Instant::now();
    let hedge_diag = read_item_hedge_diagnostics(&driver, op_options, "disabled-item", "pk1").await;
    let elapsed = start.elapsed();

    assert!(
        hedge_diag.is_none(),
        "Disabled strategy must suppress `execute_hedged` entirely; \
         hedge_diagnostics={hedge_diag:?}",
    );
    assert!(
        elapsed >= Duration::from_millis(400),
        "without hedging the primary must observe the full ~500 ms delay; \
         elapsed={elapsed:?}",
    );
}

/// Spec §15.2 row 7 — *deadline shorter than threshold → no hedge fires*.
///
/// `EndToEndOperationLatencyPolicy` clamps to a 1 s minimum (see
/// [`policies.rs`](azure_data_cosmos_driver::options::EndToEndOperationLatencyPolicy)).
/// We pick a 1 s deadline, a 3 s hedge threshold, and a 1.5 s primary
/// delay: the deadline fires first, the threshold timer never elapses,
/// and the alternate is never spawned. The operation surfaces a deadline
/// error rather than a hedged success.
#[tokio::test]
async fn hedging_respects_deadline() {
    let ctx = setup_multi_region(WriteMode::Single).await;
    seed_item(&ctx, "deadline-item", "pk1").await;

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(Region::EAST_US)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_delay(Duration::from_millis(1500))
        .with_probability(1.0)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("hedging-east-us-deadline-delay", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    // 3 s threshold > 1 s deadline. The alternate must never spawn.
    let (driver, _op_options_hedge) =
        make_hedging_driver(&ctx, Duration::from_secs(3), rules).await;

    let op_options = OperationOptionsBuilder::new()
        .with_availability_strategy(AvailabilityStrategy::Hedging(HedgingStrategy::new(
            HedgeThreshold::new(Duration::from_secs(3)).expect("non-zero"),
        )))
        .with_end_to_end_latency_policy(EndToEndOperationLatencyPolicy::from(Duration::from_secs(
            1,
        )))
        .build();

    let start = std::time::Instant::now();
    let result = read_item_result(&driver, op_options, "deadline-item", "pk1").await;
    let elapsed = start.elapsed();

    assert!(
        result.is_err(),
        "deadline (1 s) shorter than hedge threshold (3 s) must surface as \
         an error rather than a hedged success; got Ok response \
         (elapsed={elapsed:?})",
    );
    assert!(
        elapsed < Duration::from_millis(1500),
        "deadline must fire before the 1.5 s primary delay completes; \
         elapsed={elapsed:?}",
    );
    assert!(
        elapsed >= Duration::from_secs(1),
        "deadline (1 s) must run for approximately its full duration; \
         elapsed={elapsed:?}",
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Race-dynamics tests (cluster 2)
// ─────────────────────────────────────────────────────────────────────────────

/// Spec §15.2 row 4 — *both regions slow, graceful degradation*.
///
/// Inject 500 ms on East US, 700 ms on West US, threshold 100 ms.
/// The primary starts immediately and finishes at ~500 ms; the alternate
/// spawns at 100 ms and finishes at ~800 ms. The primary wins the race
/// (no `was_hedge` because the primary completed first, though it did so
/// AFTER the threshold so the alternate did fire). The point of the
/// test is graceful degradation: whichever responds first wins; the
/// operation does not error out just because every region is degraded.
#[tokio::test]
async fn hedging_read_both_regions_slow() {
    let ctx = setup_multi_region(WriteMode::Single).await;
    seed_item(&ctx, "both-slow-item", "pk1").await;

    let east_rule = Arc::new(
        FaultInjectionRuleBuilder::new(
            "hedging-east-us-both-slow",
            FaultInjectionResultBuilder::new()
                .with_delay(Duration::from_millis(500))
                .with_probability(1.0)
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::ReadItem)
                .with_region(Region::EAST_US)
                .build(),
        )
        .build(),
    );
    let west_rule = Arc::new(
        FaultInjectionRuleBuilder::new(
            "hedging-west-us-both-slow",
            FaultInjectionResultBuilder::new()
                .with_delay(Duration::from_millis(700))
                .with_probability(1.0)
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::ReadItem)
                .with_region(Region::WEST_US)
                .build(),
        )
        .build(),
    );

    let (driver, op_options) = make_hedging_driver(
        &ctx,
        Duration::from_millis(100),
        vec![Arc::clone(&east_rule), Arc::clone(&west_rule)],
    )
    .await;

    let hedge_diag = read_item_hedge_diagnostics(&driver, op_options, "both-slow-item", "pk1")
        .await
        .expect(
            "hedging entered (alternate spawned at 100 ms even though both \
             regions are slow); diagnostics must be attached",
        );

    // Both regions were contacted because the alternate did spawn at the
    // threshold; the primary won the race because its 500 ms delay
    // finished before the alternate's 100 ms + 700 ms = 800 ms.
    assert_eq!(
        hedge_diag.terminal_state,
        HedgeTerminalState::PrimaryWonAfterHedge,
        "primary completing first after the alternate spawned must \
         classify as PrimaryWonAfterHedge (spec §10.1.1); diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.alternate_region,
        Some(Region::WEST_US),
        "both pipelines should have been spawned (graceful degradation); \
         diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.response_region,
        Region::EAST_US,
        "primary (500 ms) should beat alternate (100 ms threshold + 700 ms \
         = 800 ms); diag={hedge_diag:?}",
    );
    assert!(
        east_rule.hit_count() >= 1,
        "East US rule should have fired at least once",
    );
    assert!(
        west_rule.hit_count() >= 1,
        "West US rule should have fired at least once",
    );
}

/// Spec §15.2 row 9 — *loser pipeline is structurally cancelled*.
///
/// 800 ms delay on East US, no fault on West US, threshold 100 ms.
/// The alternate (West US) wins quickly; once the race resolves,
/// `execute_hedged` drops the East US future — which structurally
/// cancels the in-flight transport task (§12 drop-the-future
/// cancellation). After the cancellation propagates, the East US rule's
/// `hit_count` must remain at exactly 1 (the single primary attempt
/// observed) — proving the loser was cancelled mid-flight rather than
/// retried, duplicated, or left to run to completion behind the scenes.
#[tokio::test]
async fn hedging_cancels_loser() {
    let ctx = setup_multi_region(WriteMode::Single).await;
    seed_item(&ctx, "cancel-item", "pk1").await;

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(Region::EAST_US)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_delay(Duration::from_millis(800))
        .with_probability(1.0)
        .build();
    let east_rule = Arc::new(
        FaultInjectionRuleBuilder::new("hedging-east-us-cancel-delay", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&east_rule)];

    let (driver, op_options) = make_hedging_driver(&ctx, Duration::from_millis(100), rules).await;

    let hedge_diag = read_item_hedge_diagnostics(&driver, op_options, "cancel-item", "pk1")
        .await
        .expect("alternate should win and `HedgeDiagnostics` must be attached");
    assert_eq!(hedge_diag.response_region, Region::WEST_US);

    // Give the loser a chance to "wake up" if cancellation didn't take.
    // If it weren't structurally cancelled, the 800 ms delay would still
    // be running and would record another hit when it completes.
    tokio::time::sleep(Duration::from_millis(1000)).await;

    assert_eq!(
        east_rule.hit_count(),
        1,
        "East US loser must be cancelled structurally; hit_count > 1 would \
         indicate a retry or duplicate attempt was observed after the \
         alternate won the race",
    );
}

/// Spec §15.2 row 10 — *transient slow primary does not accumulate*.
///
/// `with_hit_limit(5)` makes the East US delay rule apply for the first
/// 5 reads only; subsequent reads observe no fault. The first 5 reads
/// hedge (primary slow → alternate wins from West US); the 6th read's
/// primary completes fast and wins pre-threshold (zero-overhead happy
/// path per §6.5 #3). The contrast between read 5 (`was_hedge=true`,
/// `total_requests_launched=2`) and read 6 (`was_hedge=false`,
/// `total_requests_launched=1`) is the spec's "failback" signal:
/// transient cross-region latency does not stack over time.
#[tokio::test]
async fn hedging_failback_to_primary() {
    let ctx = setup_multi_region(WriteMode::Single).await;
    seed_item(&ctx, "failback-item", "pk1").await;

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(Region::EAST_US)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_delay(Duration::from_millis(500))
        .with_probability(1.0)
        .build();
    let east_rule = Arc::new(
        FaultInjectionRuleBuilder::new("hedging-east-us-failback-delay", result)
            .with_condition(condition)
            .with_hit_limit(5)
            .build(),
    );
    let rules = vec![Arc::clone(&east_rule)];

    let (driver, op_options) = make_hedging_driver(&ctx, Duration::from_millis(100), rules).await;

    // Reads 1–5 must hedge: each primary attempt incurs the 500 ms delay,
    // exhausting the hit_limit after 5 calls.
    for i in 1..=5 {
        let hedge_diag =
            read_item_hedge_diagnostics(&driver, op_options.clone(), "failback-item", "pk1")
                .await
                .unwrap_or_else(|| panic!("read #{i} must enter execute_hedged"));
        assert!(
            hedge_diag.was_hedge,
            "read #{i} must hedge (primary slow); diag={hedge_diag:?}",
        );
        assert_eq!(
            hedge_diag.response_region,
            Region::WEST_US,
            "read #{i}: alternate must win; diag={hedge_diag:?}",
        );
    }

    assert_eq!(
        east_rule.hit_count(),
        5,
        "fault rule must have been exhausted after the first 5 reads; \
         hit_count={}",
        east_rule.hit_count(),
    );

    // Read 6: no fault on East US → primary completes fast → primary wins
    // pre-threshold → was_hedge=false. This is the failback signal.
    let hedge_diag =
        read_item_hedge_diagnostics(&driver, op_options.clone(), "failback-item", "pk1")
            .await
            .expect("read #6 must still attach `HedgeDiagnostics::primary_only`");
    assert!(
        !hedge_diag.was_hedge,
        "read #6: primary should win pre-threshold once the fault expires; \
         diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.alternate_region, None,
        "read #6: only the primary should run (§6.5 #3 zero-overhead path); \
         diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.response_region,
        Region::EAST_US,
        "read #6: primary (East US) must win; diag={hedge_diag:?}",
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// PPCB composition tests (cluster 3)
// ─────────────────────────────────────────────────────────────────────────────

/// Spec §15.2 row 8 — *PPCB and hedging compose without interference*.
///
/// Enables PPCB at the driver level via `DriverOptions::with_operation_options`,
/// then runs a 503 + 500 ms delay hedge read (same setup as the basic
/// `hedging_read_primary_503` test). Verifies hedging still fires
/// normally despite PPCB tracking the (now-recorded) failure on East US.
/// The PPCB counter stays below the trip threshold (default 10), so the
/// test exercises the "PPCB is tracking but has not tripped" composition
/// path explicitly.
#[tokio::test]
async fn hedging_with_ppcb_existing_failures() {
    let ctx = setup_multi_region(WriteMode::Single).await;
    seed_item(&ctx, "ppcb-compose-item", "pk1").await;

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(Region::EAST_US)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_delay(Duration::from_millis(500))
        .with_probability(1.0)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new("hedging-east-us-ppcb-compose", result)
            .with_condition(condition)
            .build(),
    );
    let rules = vec![Arc::clone(&rule)];

    let runtime = ctx
        .emulator
        .runtime_builder_with_fault_rules(rules)
        .build()
        .await
        .expect("runtime builds");
    let account = AccountReference::with_master_key(
        Url::parse(ACCOUNT_ENDPOINT).expect("valid endpoint"),
        "ZW11bGF0b3JrZXk=",
    );

    // Enable PPCB at driver init via the layered OperationOptions resolver.
    let driver_op_options = OperationOptionsBuilder::new()
        .with_per_partition_circuit_breaker_enabled(true)
        .build();
    let driver_options = DriverOptions::builder(account.clone())
        .with_preferred_regions(vec![Region::EAST_US, Region::WEST_US])
        .with_operation_options(driver_op_options)
        .build();
    let driver = runtime
        .get_or_create_driver(account, Some(driver_options))
        .await
        .expect("driver initializes with PPCB enabled");

    let op_options = OperationOptionsBuilder::new()
        .with_availability_strategy(AvailabilityStrategy::Hedging(HedgingStrategy::new(
            HedgeThreshold::new(Duration::from_millis(100)).expect("non-zero"),
        )))
        .build();

    let hedge_diag = read_item_hedge_diagnostics(&driver, op_options, "ppcb-compose-item", "pk1")
        .await
        .expect("hedging must enter even with PPCB enabled");

    assert!(
        hedge_diag.was_hedge,
        "alternate must win despite PPCB tracking the East US failure; \
         diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.response_region,
        Region::WEST_US,
        "PPCB has only 1 failure (below threshold 10); primary still routes \
         to East US and the alternate wins on West US; diag={hedge_diag:?}",
    );
    assert!(
        rule.hit_count() >= 1,
        "the 503 rule must have been hit at least once",
    );
}

/// Spec §15.2 row 12 — *N consecutive alternate-region wins trip PPCB*.
///
/// With PPCB enabled and the default `consecutive_hedge_win_threshold`
/// of 5 (per §9.5), force five reads in a row where the alternate wins
/// the hedge race. After the fifth win, [`record_hedge_alternate_win`]
/// installs an `Unhealthy` entry in `circuit_breaker_overrides` for the
/// partition with `read_failure_count` seeded to
/// `config.read_failure_threshold`, so
/// [`can_circuit_breaker_trigger_failover`] accepts the entry on the
/// very next routing decision. On read #6 the primary attempt is
/// transparently re-routed to West US by `resolve_endpoint`, the
/// alternate region. The primary completes fast and wins
/// pre-threshold — no hedge race occurs.
///
/// The spec §9.5 end-to-end signal is the read-after-trip observation
/// on read #6: `was_hedge=false`, `total_requests_launched=1`, and
/// `response_region=West US`. This combination can only happen if the
/// hedge-driven PPCB trip actually redirects routing.
#[tokio::test]
async fn hedging_alternate_wins_trip_ppcb() {
    let ctx = setup_multi_region(WriteMode::Single).await;
    seed_item(&ctx, "ppcb-trip-item", "pk1").await;

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(Region::EAST_US)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_delay(Duration::from_millis(500))
        .with_probability(1.0)
        .build();
    // The fault rule must remain active for the full `THRESHOLD` reads so
    // every pre-trip read races East US (slow) vs West US (fast) and the
    // alternate wins. After the trip fires, the primary is re-routed away
    // from East US so the rule no longer matters — the hit limit just
    // bounds it.
    const THRESHOLD: u32 = 5;
    let east_rule = Arc::new(
        FaultInjectionRuleBuilder::new("hedging-east-us-ppcb-trip", result)
            .with_condition(condition)
            .with_hit_limit(THRESHOLD)
            .build(),
    );
    let rules = vec![Arc::clone(&east_rule)];

    let runtime = ctx
        .emulator
        .runtime_builder_with_fault_rules(rules)
        .build()
        .await
        .expect("runtime builds");
    let account = AccountReference::with_master_key(
        Url::parse(ACCOUNT_ENDPOINT).expect("valid endpoint"),
        "ZW11bGF0b3JrZXk=",
    );

    let driver_op_options = OperationOptionsBuilder::new()
        .with_per_partition_circuit_breaker_enabled(true)
        .build();
    let driver_options = DriverOptions::builder(account.clone())
        .with_preferred_regions(vec![Region::EAST_US, Region::WEST_US])
        .with_operation_options(driver_op_options)
        .build();
    let driver = runtime
        .get_or_create_driver(account, Some(driver_options))
        .await
        .expect("driver initializes with PPCB enabled");

    let op_options = OperationOptionsBuilder::new()
        .with_availability_strategy(AvailabilityStrategy::Hedging(HedgingStrategy::new(
            HedgeThreshold::new(Duration::from_millis(100)).expect("non-zero"),
        )))
        .build();

    // Pre-trip reads (1..=THRESHOLD): each races East US (delayed) against
    // West US (fast). The alternate wins every time and
    // `record_hedge_alternate_win` increments the consecutive-win counter.
    // On the THRESHOLD-th win the partition trips: an `Unhealthy` entry is
    // installed in `circuit_breaker_overrides` with failure counts seeded
    // to the configured thresholds.
    for i in 1..=THRESHOLD {
        let hedge_diag =
            read_item_hedge_diagnostics(&driver, op_options.clone(), "ppcb-trip-item", "pk1")
                .await
                .unwrap_or_else(|| panic!("read #{i} must enter execute_hedged"));
        assert!(
            hedge_diag.was_hedge,
            "read #{i} (pre-trip) must hedge; diag={hedge_diag:?}",
        );
        assert_eq!(
            hedge_diag.response_region,
            Region::WEST_US,
            "read #{i} (pre-trip): alternate (West US) must win; \
             diag={hedge_diag:?}",
        );
    }
    assert_eq!(
        east_rule.hit_count(),
        THRESHOLD,
        "fault rule must have been exhausted after the threshold reads",
    );

    // Post-trip read (THRESHOLD + 1): the PPCB override now points the
    // primary at West US. `resolve_endpoint` consults
    // `circuit_breaker_overrides` for the partition and returns the
    // alternate endpoint directly — no hedge race occurs because the
    // primary attempt completes against the healthy region before the
    // hedge threshold elapses.
    //
    // This is the spec §9.5 end-to-end signal: a sustained streak of
    // alternate-region wins eventually fails the partition away from the
    // degraded primary, transparently to the caller, with the
    // zero-overhead read path (§6.5 #3) restored.
    let hedge_diag =
        read_item_hedge_diagnostics(&driver, op_options.clone(), "ppcb-trip-item", "pk1")
            .await
            .expect("post-trip read must still attach `HedgeDiagnostics::primary_only`");
    assert!(
        !hedge_diag.was_hedge,
        "post-trip read: primary should win pre-threshold against the new \
         (West US) primary endpoint; diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.alternate_region, None,
        "post-trip read: only the primary should run (§6.5 #3 zero-overhead \
         path); diag={hedge_diag:?}",
    );
    assert_eq!(
        hedge_diag.response_region,
        Region::WEST_US,
        "post-trip read: PPCB override must route the primary to West US — \
         the visible end-user signal that the hedge-driven trip in \
         `record_hedge_alternate_win` actually redirects routing per \
         §9.5; diag={hedge_diag:?}",
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// §8.4 exclude-regions invariant under fault injection (cluster 4)
// ─────────────────────────────────────────────────────────────────────────────

/// Spec §15.2 row 11 — *alternate hedge's retry stays pinned to its region*.
///
/// Three-region single-master setup (East US write hub, West US +
/// Central US satellites). The user excludes Central US via
/// `ExcludeRegions`. A 500 ms delay on East US forces the primary to
/// hedge; a 503 on West US (the alternate's pinned region) triggers a
/// transport-layer retry inside the alternate pipeline.
///
/// Per §8.4 the alternate's local-only retries are forbidden from
/// re-routing the request outside its pinned region — even when a
/// healthy alternative (Central US) exists. The retry must stay on
/// West US, fail again, and surface as a transient hedge outcome rather
/// than silently falling back to Central US.
///
/// Observable signals:
/// * `east_rule.hit_count() == 1` — the primary fired once before the
///   alternate took over.
/// * `west_rule.hit_count() >= 2` — the initial alternate attempt plus
///   at least one local-only retry, all pinned to West US.
/// * `central_rule.hit_count() == 0` — Central US must NEVER be
///   contacted, because the user excluded it.
#[tokio::test]
async fn hedging_exclude_regions_under_503_retry() {
    use azure_data_cosmos_driver::in_memory_emulator::{
        ConsistencyLevel, InMemoryEmulatorHttpClient, ReplicationConfig, VirtualAccountConfig,
        VirtualRegion,
    };

    // Three-region custom setup — `setup_multi_region` only stands up two.
    let east_url = "https://eastus.emulator.local";
    let west_url = "https://westus.emulator.local";
    let central_url = "https://centralus.emulator.local";
    let config = VirtualAccountConfig::new(vec![
        VirtualRegion::new("East US", Url::parse(east_url).unwrap()),
        VirtualRegion::new("West US", Url::parse(west_url).unwrap()),
        VirtualRegion::new("Central US", Url::parse(central_url).unwrap()),
    ])
    .unwrap()
    .with_write_mode(WriteMode::Single)
    .with_consistency(ConsistencyLevel::Session)
    .with_replication_config(ReplicationConfig::immediate());
    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
    let store = emulator.store();
    store.create_database(DB_NAME);
    store.create_container(
        DB_NAME,
        COLL_NAME,
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))
        .unwrap(),
    );

    // Seed via the East US gateway (immediate replication propagates).
    let body = serde_json::json!({"id": "excl-item", "pk": "pk1", "value": 1});
    let seed_req = create_item_request(east_url, DB_NAME, COLL_NAME, &body, r#"["pk1"]"#, false);
    let seed_resp = emulator
        .execute_request(&seed_req)
        .await
        .expect("seed succeeds");
    assert!(
        (200..300).contains(&u16::from(seed_resp.status())),
        "seed must succeed",
    );

    // Fault rules: delay East US (forces hedge), 503 West US (forces
    // local-only retry inside the alternate), error+probe Central US
    // (must never be contacted — but we still attach a rule to count
    // hits so the assertion is loud if we ever stray there).
    let east_rule = Arc::new(
        FaultInjectionRuleBuilder::new(
            "hedging-excl-east-us-delay",
            FaultInjectionResultBuilder::new()
                .with_delay(Duration::from_millis(500))
                .with_probability(1.0)
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::ReadItem)
                .with_region(Region::EAST_US)
                .build(),
        )
        .build(),
    );
    let west_rule = Arc::new(
        FaultInjectionRuleBuilder::new(
            "hedging-excl-west-us-503",
            FaultInjectionResultBuilder::new()
                .with_error(FaultInjectionErrorType::ServiceUnavailable)
                .with_probability(1.0)
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::ReadItem)
                .with_region(Region::WEST_US)
                .build(),
        )
        .build(),
    );
    let central_rule = Arc::new(
        FaultInjectionRuleBuilder::new(
            "hedging-excl-central-us-503",
            FaultInjectionResultBuilder::new()
                .with_error(FaultInjectionErrorType::ServiceUnavailable)
                .with_probability(1.0)
                .build(),
        )
        .with_condition(
            FaultInjectionConditionBuilder::new()
                .with_operation_type(FaultOperationType::ReadItem)
                .with_region(Region::CENTRAL_US)
                .build(),
        )
        .build(),
    );

    let runtime = emulator
        .runtime_builder_with_fault_rules(vec![
            Arc::clone(&east_rule),
            Arc::clone(&west_rule),
            Arc::clone(&central_rule),
        ])
        .build()
        .await
        .expect("runtime builds");
    let account = AccountReference::with_master_key(
        Url::parse(ACCOUNT_ENDPOINT).expect("valid endpoint"),
        "ZW11bGF0b3JrZXk=",
    );
    let driver_options = DriverOptions::builder(account.clone())
        .with_preferred_regions(vec![Region::EAST_US, Region::WEST_US, Region::CENTRAL_US])
        .build();
    let driver = runtime
        .get_or_create_driver(account, Some(driver_options))
        .await
        .expect("driver initializes against 3-region emulator metadata");

    // User excludes Central US — the alternate hedge MUST honor this when
    // its local retry fires after the 503 on West US.
    let op_options = OperationOptionsBuilder::new()
        .with_availability_strategy(AvailabilityStrategy::Hedging(HedgingStrategy::new(
            HedgeThreshold::new(Duration::from_millis(100)).expect("non-zero"),
        )))
        .with_excluded_regions(ExcludedRegions::new().with_region(Region::CENTRAL_US))
        .build();

    let _ = read_item_result(&driver, op_options, "excl-item", "pk1").await;

    assert!(
        east_rule.hit_count() >= 1,
        "East US must have been contacted as the primary; hit_count={}",
        east_rule.hit_count(),
    );
    assert!(
        west_rule.hit_count() >= 1,
        "West US must have been contacted as the alternate; hit_count={}",
        west_rule.hit_count(),
    );
    assert_eq!(
        central_rule.hit_count(),
        0,
        "Central US must NEVER be contacted (user excluded it via \
         ExcludeRegions); hit_count={} — alternate hedge's local-only \
         retry incorrectly rerouted outside its pinned region (§8.4 \
         violation)",
        central_rule.hit_count(),
    );
}
