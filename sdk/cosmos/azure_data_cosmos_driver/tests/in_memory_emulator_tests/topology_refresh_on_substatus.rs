// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration tests that pin the desired SDK behavior for two
//! topology-related Forbidden sub-status codes:
//!
//! * **403 / 1008 `DatabaseAccountNotFound`** — the gateway in a given
//!   region no longer owns the account (typically because the customer
//!   shrank the account's region list). The first 1008 must trigger an
//!   account-topology refresh + retry against the refreshed endpoint
//!   set; bubble up only when every healthy current region also fails.
//!
//! * **403 / 3 `WriteForbidden`** — write-only. The gateway tells the
//!   client that its idea of "write region" is stale. The handler
//!   ([`try_handle_write_forbidden`] in `retry_evaluation.rs`) emits
//!   `RefreshAccountProperties` + `MarkEndpointUnavailable` +
//!   `FailoverRetry`; these tests pin the desired end-to-end behavior
//!   so future changes can be regression-tested.
//!
//! ## Test-harness model
//!
//! Each test follows the same shape, adapted from the patterns in
//! [`super::excluded_regions_fallback`] and [`super::hedging`]:
//!
//! 1. Build a 2-region in-memory emulator (East US hub + West US
//!    satellite) with `ReplicationConfig::immediate()` so a write that
//!    succeeds against any region is immediately readable from the
//!    other. `WriteMode::Multi` is used for the 1008 tests so reads can
//!    succeed cross-region; `WriteMode::Multi` is also used for the
//!    403/3 tests because the fault rule is what flips a region into
//!    "not a write region", not the underlying account topology.
//! 2. Wrap the emulator transport in [`HostRecorder`] (`RequestObserver`)
//!    **and** [`runtime_builder_with_fault_rules`] so:
//!    * the observer captures every request that actually reaches the
//!      emulator (post-fault retries + the emulator's own `GET /`
//!      account-topology fetches);
//!    * the fault factory short-circuits faulted requests before the
//!      emulator, so [`FaultInjectionRule::hit_count`] is the authority
//!      for the faulted side and the observer is the authority for the
//!      non-faulted side. See
//!      [`super::hedging`] for the same composition.
//! 3. Pre-seed an item via an unfaulted write so the post-fault read /
//!    write has something to land on.
//! 4. Clear the recorder, enable the fault rule, issue the operation
//!    through `driver.execute_operation`, and inspect:
//!    * the rule's [`hit_count`](FaultInjectionRule::hit_count),
//!    * the recorder's data-plane hosts (filtering out `GET /` for
//!      retry assertions, or counting `GET /` for refresh assertions),
//!    * the response's diagnostics — [`request_count`] and
//!      [`regions_contacted`] in particular.
//!
//! ## Test-harness limits
//!
//! * The in-memory emulator always returns its configured regions on
//!   `GET /`. Tests cannot simulate "topology shrank from 2 regions to
//!   1" via the emulator alone. The 1008 tests therefore assert
//!   *refresh-was-triggered* (count of `GET /` goes up after the fault)
//!   and *retry-targeted-a-different-region* (via the in-flight skip
//!   set) rather than a full topology-shrink scenario.
//! * Cross-partition query (`QueryItems`) and dedicated feed-range
//!   query (`FeedRangeQuery`) paths are not covered here. They route
//!   through the same operation-type-agnostic `evaluate_http_outcome`
//!   classifier in `retry_evaluation.rs`, so coverage for `ReadItem` /
//!   `CreateItem` / `UpsertItem` carries transitively.
//! * The hedging-only path (`evaluate_hedge_leg_effects`) mirrors the
//!   1008 handler from the main pipeline so a hedge race that observes
//!   1008 also emits `RefreshAccountProperties` — covered structurally
//!   by the existing `hedging` test module.

#![cfg(feature = "fault_injection")]

use std::sync::Arc;
use std::time::Duration;

use azure_core::http::Url;

use azure_data_cosmos_driver::driver::CosmosDriver;
use azure_data_cosmos_driver::fault_injection::{
    FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
    FaultInjectionRule, FaultInjectionRuleBuilder, FaultOperationType,
};
use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, InMemoryEmulatorHttpClient, ReplicationConfig, VirtualAccountConfig,
    VirtualRegion, WriteMode,
};
use azure_data_cosmos_driver::models::{
    AccountReference, CosmosOperation, CosmosStatus, ItemReference, PartitionKey,
};
use azure_data_cosmos_driver::options::{DriverOptions, ExcludedRegions, OperationOptions, Region};

use super::host_recorder::HostRecorder;

const EAST_URL: &str = "https://eastus.emulator.local";
const WEST_URL: &str = "https://westus.emulator.local";
const EAST_HOST: &str = "eastus.emulator.local";
const WEST_HOST: &str = "westus.emulator.local";

const DB_NAME: &str = "testdb";
const COLL_NAME: &str = "testcoll";
const PK_VALUE: &str = "pk1";

/// Builds a two-region in-memory emulator with the given write mode and
/// observer, then wires it through `runtime_builder_with_fault_rules`
/// to compose fault injection with the emulator transport. Returns a
/// ready-to-use driver + the master-key account reference.
async fn build_driver_with_faults(
    write_mode: WriteMode,
    observer: Arc<HostRecorder>,
    rules: Vec<Arc<FaultInjectionRule>>,
) -> (Arc<CosmosDriver>, AccountReference) {
    let config = VirtualAccountConfig::new(vec![
        VirtualRegion::new("East US", Url::parse(EAST_URL).unwrap()),
        VirtualRegion::new("West US", Url::parse(WEST_URL).unwrap()),
    ])
    .unwrap()
    .with_write_mode(write_mode)
    .with_consistency(ConsistencyLevel::Session)
    .with_replication_config(ReplicationConfig::immediate());

    let emulator =
        Arc::new(InMemoryEmulatorHttpClient::new(config).with_request_observer(observer));
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

    let runtime = emulator
        .runtime_builder_with_fault_rules(rules)
        .build()
        .await
        .expect("runtime builds against the in-memory emulator");

    let account =
        AccountReference::with_master_key(Url::parse(EAST_URL).unwrap(), "ZW11bGF0b3Ita2V5");

    let driver_options = DriverOptions::builder(account.clone())
        .with_preferred_regions(vec![Region::EAST_US, Region::WEST_US])
        .build();

    let driver = runtime
        .get_or_create_driver(account.clone(), Some(driver_options))
        .await
        .expect("driver initializes against emulator metadata");

    (driver, account)
}

/// Seeds a known item via the driver under default options. Done before
/// the test-under-fault phase, then the recorder is cleared so only
/// post-fault traffic counts.
async fn seed_item_via_driver(driver: &CosmosDriver, item_id: &str) {
    let container = driver
        .resolve_container_by_name(DB_NAME, COLL_NAME)
        .await
        .expect("container resolves for seeding");
    let item_ref = ItemReference::from_name(
        &container,
        PartitionKey::from(PK_VALUE),
        item_id.to_string(),
    );
    let body = serde_json::json!({
        "id": item_id,
        "pk": PK_VALUE,
        "value": 1,
    });
    let op = CosmosOperation::create_item(item_ref).with_body(serde_json::to_vec(&body).unwrap());
    driver
        .execute_operation(op, OperationOptions::default())
        .await
        .expect("seeding write succeeds before faults are installed");
}

/// Builds a fault rule that returns `error_type` for a single region's
/// `op` requests. `hit_limit` of `None` means "always", `Some(n)` means
/// "first n hits then heal" (the latter is for tests that want the
/// retry to eventually succeed against the same region).
fn region_fault_rule(
    id: &'static str,
    op: FaultOperationType,
    region: Region,
    error_type: FaultInjectionErrorType,
    hit_limit: Option<u32>,
) -> Arc<FaultInjectionRule> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(op)
        .with_region(region)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_error(error_type)
        .with_probability(1.0)
        .build();
    let mut builder = FaultInjectionRuleBuilder::new(id, result).with_condition(condition);
    if let Some(limit) = hit_limit {
        builder = builder.with_hit_limit(limit);
    }
    Arc::new(builder.build())
}

/// Convenience: reads an item by id under `pk1`.
async fn read_item(
    driver: &CosmosDriver,
    item_id: &str,
) -> Result<
    Option<azure_data_cosmos_driver::models::CosmosResponse>,
    azure_data_cosmos_driver::error::CosmosError,
> {
    let container = driver
        .resolve_container_by_name(DB_NAME, COLL_NAME)
        .await
        .expect("container resolves");
    let item_ref = ItemReference::from_name(
        &container,
        PartitionKey::from(PK_VALUE),
        item_id.to_string(),
    );
    driver
        .execute_operation(
            CosmosOperation::read_item(item_ref),
            OperationOptions::default(),
        )
        .await
}

/// Convenience: creates a new item.
async fn create_item(
    driver: &CosmosDriver,
    item_id: &str,
) -> Result<
    Option<azure_data_cosmos_driver::models::CosmosResponse>,
    azure_data_cosmos_driver::error::CosmosError,
> {
    let container = driver
        .resolve_container_by_name(DB_NAME, COLL_NAME)
        .await
        .expect("container resolves");
    let item_ref = ItemReference::from_name(
        &container,
        PartitionKey::from(PK_VALUE),
        item_id.to_string(),
    );
    let body = serde_json::json!({
        "id": item_id,
        "pk": PK_VALUE,
        "value": 2,
    });
    let op = CosmosOperation::create_item(item_ref).with_body(serde_json::to_vec(&body).unwrap());
    driver
        .execute_operation(op, OperationOptions::default())
        .await
}

/// Convenience: upserts an item.
async fn upsert_item(
    driver: &CosmosDriver,
    item_id: &str,
) -> Result<
    Option<azure_data_cosmos_driver::models::CosmosResponse>,
    azure_data_cosmos_driver::error::CosmosError,
> {
    let container = driver
        .resolve_container_by_name(DB_NAME, COLL_NAME)
        .await
        .expect("container resolves");
    let item_ref = ItemReference::from_name(
        &container,
        PartitionKey::from(PK_VALUE),
        item_id.to_string(),
    );
    let body = serde_json::json!({
        "id": item_id,
        "pk": PK_VALUE,
        "value": 3,
    });
    let op = CosmosOperation::upsert_item(item_ref).with_body(serde_json::to_vec(&body).unwrap());
    driver
        .execute_operation(op, OperationOptions::default())
        .await
}

// ─────────────────────────────────────────────────────────────────────
// 1008 / DatabaseAccountNotFound
// ─────────────────────────────────────────────────────────────────────

/// **403/1008 — ReadItem.** With East US returning 403/1008 on `ReadItem`,
/// the SDK must (post-fix) refresh the account topology and retry
/// against West US, which has the item replicated and returns it.
///
/// Pre-fix expectation: FAIL — no 1008 handler, single attempt
/// abort, `request_count == 1`, no West US data-plane request, no
/// extra `GET /` refresh, final result is `Err(403/1008)`.
#[tokio::test]
async fn read_item_403_1008_triggers_refresh_and_cross_region_retry() {
    let recorder = HostRecorder::new();
    let rule = region_fault_rule(
        "1008-read-east",
        FaultOperationType::ReadItem,
        Region::EAST_US,
        FaultInjectionErrorType::DatabaseAccountNotFound,
        Some(1),
    );
    let (driver, _account) =
        build_driver_with_faults(WriteMode::Multi, recorder.clone(), vec![rule.clone()]).await;

    seed_item_via_driver(&driver, "read-1008-item").await;
    recorder.clear();

    let response = read_item(&driver, "read-1008-item")
        .await
        .expect("post-fix: refresh + cross-region retry must succeed")
        .expect("read returns a response body");

    assert!(
        rule.hit_count() >= 1,
        "the East US 1008 fault rule should have been applied at least once",
    );
    assert!(
        response.diagnostics().request_count() >= 2,
        "post-fix: the SDK must retry at least once after 1008; \
         observed request_count={}",
        response.diagnostics().request_count(),
    );
    let hosts = recorder.data_plane_hosts();
    assert!(
        hosts.iter().any(|h| h == WEST_HOST),
        "post-fix: at least one data-plane request must land on West US \
         after East US returns 1008; observed hosts={hosts:?}",
    );
    let regions = response.diagnostics().regions_contacted();
    assert!(
        regions.contains(&Region::WEST_US),
        "regions_contacted must include West US post-fix; observed={regions:?}",
    );
}

/// **403/1008 — CreateItem.** Same shape as the read test but for a
/// write. Multi-master account means West US accepts the write
/// natively, so the post-fix retry succeeds against West US.
#[tokio::test]
async fn create_item_403_1008_triggers_refresh_and_cross_region_retry() {
    let recorder = HostRecorder::new();
    let rule = region_fault_rule(
        "1008-create-east",
        FaultOperationType::CreateItem,
        Region::EAST_US,
        FaultInjectionErrorType::DatabaseAccountNotFound,
        Some(1),
    );
    let (driver, _account) =
        build_driver_with_faults(WriteMode::Multi, recorder.clone(), vec![rule.clone()]).await;

    // No seed needed for create — but build the driver first, then
    // clear the recorder so the initial topology fetch doesn't count.
    recorder.clear();

    let response = create_item(&driver, "create-1008-item")
        .await
        .expect("post-fix: refresh + cross-region retry must succeed")
        .expect("create returns a response body");

    assert!(rule.hit_count() >= 1);
    assert!(
        response.diagnostics().request_count() >= 2,
        "post-fix: the SDK must retry CreateItem at least once after \
         1008; observed request_count={}",
        response.diagnostics().request_count(),
    );
    let hosts = recorder.data_plane_hosts();
    assert!(
        hosts.iter().any(|h| h == WEST_HOST),
        "post-fix: CreateItem retry must reach West US; \
         observed hosts={hosts:?}",
    );
}

/// **403/1008 — UpsertItem.** Same shape; rounds out the write coverage so
/// the test matrix mirrors the five operation types observed in production
/// (ReadItem, CreateItem, UpsertItem, QueryItems, FeedRangeQuery).
/// which observed `request_count=1` for all 5 op types.
#[tokio::test]
async fn upsert_item_403_1008_triggers_refresh_and_cross_region_retry() {
    let recorder = HostRecorder::new();
    let rule = region_fault_rule(
        "1008-upsert-east",
        FaultOperationType::UpsertItem,
        Region::EAST_US,
        FaultInjectionErrorType::DatabaseAccountNotFound,
        Some(1),
    );
    let (driver, _account) =
        build_driver_with_faults(WriteMode::Multi, recorder.clone(), vec![rule.clone()]).await;

    recorder.clear();

    let response = upsert_item(&driver, "upsert-1008-item")
        .await
        .expect("post-fix: refresh + cross-region retry must succeed")
        .expect("upsert returns a response body");

    assert!(rule.hit_count() >= 1);
    assert!(
        response.diagnostics().request_count() >= 2,
        "post-fix: the SDK must retry UpsertItem at least once after \
         1008; observed request_count={}",
        response.diagnostics().request_count(),
    );
    let hosts = recorder.data_plane_hosts();
    assert!(
        hosts.iter().any(|h| h == WEST_HOST),
        "post-fix: UpsertItem retry must reach West US; \
         observed hosts={hosts:?}",
    );
}

/// **403/1008 — bounded bubble-up.** When *every* preferred region returns
/// 1008, the SDK must still attempt at least one retry (after a
/// refresh) before giving up, and must not loop forever. The test
/// asserts a bounded request count and a final 403/1008 error.
///
/// Pre-fix expectation: FAIL — the single-attempt abort means the SDK
/// reaches `request_count == 1` rather than the `>= 2` minimum the
/// retry-after-refresh contract demands.
#[tokio::test]
async fn all_regions_403_1008_bounded_retries_then_bubble_up() {
    let recorder = HostRecorder::new();
    let east_rule = region_fault_rule(
        "1008-read-east-all",
        FaultOperationType::ReadItem,
        Region::EAST_US,
        FaultInjectionErrorType::DatabaseAccountNotFound,
        None,
    );
    let west_rule = region_fault_rule(
        "1008-read-west-all",
        FaultOperationType::ReadItem,
        Region::WEST_US,
        FaultInjectionErrorType::DatabaseAccountNotFound,
        None,
    );
    let (driver, _account) = build_driver_with_faults(
        WriteMode::Multi,
        recorder.clone(),
        vec![east_rule.clone(), west_rule.clone()],
    )
    .await;

    seed_item_via_driver(&driver, "all-stale-item").await;
    recorder.clear();

    // Bubble-up takes up to 120 attempts × 1s BACKEND_FAILOVER_RETRY_INTERVAL
    // ≈ 120s on a multi-write account; the timeout is the runaway-loop guard,
    // not the convergence SLO.
    let result = tokio::time::timeout(
        Duration::from_secs(180),
        read_item(&driver, "all-stale-item"),
    )
    .await
    .expect("post-fix retry budget must be bounded — operation hung past 180s");

    let err = result.expect_err("with both regions returning 1008, the read must fail");
    let status = err.status();
    assert_eq!(
        status,
        CosmosStatus::DATABASE_ACCOUNT_NOT_FOUND,
        "1008 exhausted-budget bubble-up must surface the original status unchanged; \
         observed status={status:?}",
    );

    let diagnostics = err
        .diagnostics()
        .expect("wire-response error must carry diagnostics");
    assert!(
        diagnostics.request_count() >= 2,
        "post-fix: SDK must attempt ≥ 2 requests (one per region) \
         before bubbling up 1008; observed request_count={}",
        diagnostics.request_count(),
    );
    // Sanity guardrail against an unbounded retry loop. The exact cap
    // is a fix-time decision; this number is intentionally generous so
    // the test stays robust if the fix tunes the budget. A truly
    // unbounded loop would have hit the 10s timeout above; this
    // assertion just pins the order of magnitude.
    assert!(
        diagnostics.request_count() <= 250,
        "retry budget for 1008 must be bounded; observed \
         request_count={} which suggests an infinite-retry regression",
        diagnostics.request_count(),
    );
}

/// **403/1008 — refresh is what changes.** This test isolates the
/// refresh-was-triggered signal: it counts the emulator's `GET /`
/// account-topology fetches before vs. after the faulted read. With
/// the fix, the SDK calls `refresh_account_properties_if_due` on
/// receipt of 1008, which translates into at least one extra `GET /`
/// arriving at the emulator beyond the baseline driver-init traffic.
///
/// Pre-fix expectation: FAIL — no refresh is triggered, so the
/// post-clear `account_read_count()` stays at zero.
#[tokio::test]
async fn read_item_403_1008_triggers_topology_refresh() {
    let recorder = HostRecorder::new();
    let rule = region_fault_rule(
        "1008-read-east-refresh",
        FaultOperationType::ReadItem,
        Region::EAST_US,
        FaultInjectionErrorType::DatabaseAccountNotFound,
        Some(1),
    );
    let (driver, _account) =
        build_driver_with_faults(WriteMode::Multi, recorder.clone(), vec![rule.clone()]).await;

    seed_item_via_driver(&driver, "refresh-signal-item").await;
    recorder.clear();

    // Ignore the result; this test only cares about whether the
    // emulator saw an additional account-topology fetch as a side
    // effect of the 1008.
    let _ = read_item(&driver, "refresh-signal-item").await;

    assert!(rule.hit_count() >= 1);
    let refresh_count = recorder.account_read_count();
    assert!(
        refresh_count >= 1,
        "post-fix: receipt of 1008 must trigger at least one \
         account-topology refresh (`GET /`); observed refresh_count={refresh_count}",
    );
}

// ─────────────────────────────────────────────────────────────────────
// 403 / 3 — WriteForbidden
// ─────────────────────────────────────────────────────────────────────

/// **403/3 — stale write region.** A first attempt against East US
/// returns 403/3; the SDK must refresh the account topology, then
/// retry against West US (which the multi-write emulator accepts).
/// Final result is success.
///
/// Pre-fix expectation: likely PASS for the basic retry, since
/// `try_handle_write_forbidden` exists in the SDK and emits
/// `FailoverRetry` + `RefreshAccountProperties`. This test is the
/// regression guard.
#[tokio::test]
async fn write_403_3_triggers_refresh_and_cross_region_retry() {
    let recorder = HostRecorder::new();
    let rule = region_fault_rule(
        "403-3-create-east",
        FaultOperationType::CreateItem,
        Region::EAST_US,
        FaultInjectionErrorType::WriteForbidden,
        Some(1),
    );
    let (driver, _account) =
        build_driver_with_faults(WriteMode::Multi, recorder.clone(), vec![rule.clone()]).await;

    recorder.clear();

    let response = create_item(&driver, "403-3-item")
        .await
        .expect("post-fix: retry against the refreshed write region must succeed")
        .expect("create returns a response body");

    assert!(rule.hit_count() >= 1);
    assert!(
        response.diagnostics().request_count() >= 2,
        "the SDK must retry CreateItem after 403/3; \
         observed request_count={}",
        response.diagnostics().request_count(),
    );
    let hosts = recorder.data_plane_hosts();
    assert!(
        hosts.iter().any(|h| h == WEST_HOST),
        "the post-403/3 retry must reach West US; observed hosts={hosts:?}",
    );
}

/// **403/3 — refresh is what changes.** The 403/3 handler in
/// `retry_evaluation.rs` already emits `RefreshAccountProperties`; this
/// test pins that wire-up so a fix that consolidates the refresh path
/// for 1008 + 403/3 cannot accidentally drop the 403/3 side.
#[tokio::test]
async fn write_403_3_triggers_topology_refresh() {
    let recorder = HostRecorder::new();
    let rule = region_fault_rule(
        "403-3-create-east-refresh",
        FaultOperationType::CreateItem,
        Region::EAST_US,
        FaultInjectionErrorType::WriteForbidden,
        Some(1),
    );
    let (driver, _account) =
        build_driver_with_faults(WriteMode::Multi, recorder.clone(), vec![rule.clone()]).await;

    recorder.clear();
    let _ = create_item(&driver, "403-3-refresh-item").await;

    assert!(rule.hit_count() >= 1);
    let refresh_count = recorder.account_read_count();
    assert!(
        refresh_count >= 1,
        "receipt of 403/3 must trigger at least one account-topology \
         refresh; observed refresh_count={refresh_count}",
    );
}

/// **403/3 — bounded bubble-up.** Both regions return 403/3 on
/// CreateItem: the SDK must retry within a bounded budget and then
/// surface the original 403/3 to the caller. Guards against any
/// future change that would turn `FailoverRetry` into an unbounded
/// loop when no region is healthy.
#[tokio::test]
async fn all_regions_403_3_bounded_retries_then_bubble_up() {
    let recorder = HostRecorder::new();
    let east = region_fault_rule(
        "403-3-create-east-all",
        FaultOperationType::CreateItem,
        Region::EAST_US,
        FaultInjectionErrorType::WriteForbidden,
        None,
    );
    let west = region_fault_rule(
        "403-3-create-west-all",
        FaultOperationType::CreateItem,
        Region::WEST_US,
        FaultInjectionErrorType::WriteForbidden,
        None,
    );
    let (driver, _account) = build_driver_with_faults(
        WriteMode::Multi,
        recorder.clone(),
        vec![east.clone(), west.clone()],
    )
    .await;

    recorder.clear();

    // Bubble-up takes up to 120 attempts × 1s BACKEND_FAILOVER_RETRY_INTERVAL
    // ≈ 120s on a multi-write account; the timeout is the runaway-loop guard,
    // not the convergence SLO.
    let result = tokio::time::timeout(
        Duration::from_secs(180),
        create_item(&driver, "403-3-all-forbidden-item"),
    )
    .await
    .expect("retry budget must be bounded — operation hung past 180s");

    let err = result.expect_err("with both regions returning 403/3, the create must fail");
    let status = err.status();
    assert_eq!(
        status,
        CosmosStatus::WRITE_FORBIDDEN,
        "403/3 exhausted-budget bubble-up must surface the original status unchanged; \
         observed status={status:?}",
    );

    let diagnostics = err
        .diagnostics()
        .expect("wire-response error must carry diagnostics");
    assert!(
        diagnostics.request_count() <= 250,
        "retry budget for 403/3 must be bounded; observed \
         request_count={} which suggests an infinite-retry regression",
        diagnostics.request_count(),
    );
}

/// **403/3 — second attempt must land in a different region.** A
/// production-observed pattern was 4× retries pinned to the same
/// stale region. This in-memory test exercises only the basic
/// single-failure failover path: a fault rule with `hit_limit=Some(1)`
/// cannot drive the `write_failure_threshold` (5+) needed to populate
/// the PPCB override cache, so this test does NOT reproduce the
/// production pin — it only verifies that the existing
/// `try_handle_write_forbidden` handler reroutes attempt 2 to a
/// different region in the simple case.
///
/// **Pre-fix expectation: PASS** — the basic failover wiring already
/// works. The actual production pin (override-cache pinning to a stale
/// region) is covered by:
/// * `resolve_endpoint_ppcb_override_pinned_to_unavailable_endpoint_reproduces_prod_403_3_loop`
///   (unit test in `operation_pipeline::tests`, the production-shape
///   repro);
/// * PPCB-1 / PPCB-2 in `regional_gateway_unreachable.rs` (override
///   skip-set coverage).
#[tokio::test]
async fn write_403_3_second_attempt_targets_different_region() {
    let recorder = HostRecorder::new();
    let rule = region_fault_rule(
        "403-3-create-east-region-honesty",
        FaultOperationType::CreateItem,
        Region::EAST_US,
        FaultInjectionErrorType::WriteForbidden,
        Some(1),
    );
    let (driver, _account) =
        build_driver_with_faults(WriteMode::Multi, recorder.clone(), vec![rule.clone()]).await;

    recorder.clear();

    let response = create_item(&driver, "403-3-region-honesty-item")
        .await
        .expect("post-fix: the retry succeeds against a different region")
        .expect("create returns a response body");

    assert!(rule.hit_count() >= 1);

    let diagnostics = response.diagnostics();
    let requests = diagnostics.requests();
    assert!(
        requests.len() >= 2,
        "expected at least 2 attempts to compare regions; \
         observed request_count={}",
        requests.len(),
    );

    // The first attempt is the faulted one against East US (the
    // fault factory still records the attempt in diagnostics even
    // though the emulator never saw it). The second attempt is the
    // retry, which MUST target a region different from the first.
    let first_region = requests[0].region().cloned();
    let second_region = requests[1].region().cloned();
    assert_ne!(
        first_region, second_region,
        "post-fix: the second attempt must target a different region \
         than the first (the 403/3 retry must actually fail over). \
         attempts[0]={first_region:?}, attempts[1]={second_region:?}, \
         full={requests:?}",
    );

    // Pin the assertion stronger via the emulator's view: at least one
    // post-fault data-plane request must reach West. We do NOT assert
    // "East never appears" here — the emulator's request observer also
    // sees per-partition metadata lookups (e.g. `/pkranges`) which the
    // driver legitimately issues against East as part of routing
    // preparation. Those metadata hops are not the retry under test;
    // the `assert_ne!(first_region, second_region)` above is the
    // behavioral contract.
    let hosts = recorder.data_plane_hosts();
    assert!(
        hosts.iter().any(|h| h == WEST_HOST),
        "the 403/3 retry must reach West US; observed hosts={hosts:?}",
    );
}

/// **403/3 — does persistent 403/3 against one region pin retries
/// to that region?** A production-observed trace had `request_count=4`
/// against the same region with the retry never moving to a healthy
/// region. This test asks the in-memory emulator the same question:
///
/// * Persistent fault (`hit_limit = None`) on CreateItem against East
///   US, so the SDK cannot accidentally succeed on attempt 2 by way of
///   the rule being exhausted; and
/// * West US completely healthy.
///
/// **Empirical finding (pre-fix):** the in-memory emulator does **not**
/// reproduce the production pin. With persistent 403/3 on East and a
/// healthy West, the existing `try_handle_write_forbidden` handler
/// successfully fails the second attempt over to West. This means the
/// production pin is rooted somewhere the emulator does not currently
/// exercise — most likely the PPCB/PPAF per-partition override cache
/// in `operation_pipeline::resolve_endpoint`. The test is kept as a
/// regression guard for the basic 403/3 failover path — it must
/// continue to pass through any future fix.
#[tokio::test]
async fn write_403_3_persistent_fault_pins_retries_to_same_region() {
    let recorder = HostRecorder::new();
    let rule = region_fault_rule(
        "403-3-create-east-persistent",
        FaultOperationType::CreateItem,
        Region::EAST_US,
        FaultInjectionErrorType::WriteForbidden,
        None,
    );
    let (driver, _account) =
        build_driver_with_faults(WriteMode::Multi, recorder.clone(), vec![rule.clone()]).await;

    recorder.clear();

    let result = tokio::time::timeout(
        Duration::from_secs(10),
        create_item(&driver, "403-3-pin-item"),
    )
    .await
    .expect("create must not hang");

    assert!(
        rule.hit_count() >= 1,
        "fault rule must fire at least once on East US",
    );

    match result {
        // Post-fix success path: failover actually rerouted to West US.
        Ok(Some(response)) => {
            let diagnostics = response.diagnostics();
            let requests = diagnostics.requests();
            let east_attempts = requests
                .iter()
                .filter(|r| r.region().map(|r| r == &Region::EAST_US).unwrap_or(false))
                .count();
            let west_attempts = requests
                .iter()
                .filter(|r| r.region().map(|r| r == &Region::WEST_US).unwrap_or(false))
                .count();
            assert!(
                west_attempts >= 1,
                "post-fix: at least one attempt must reach West US once East US is \
                 known to be persistently 403/3; attempts={requests:?}",
            );
            // Surface the production-observed pin shape on the success
            // branch so the assertion message is informative.
            assert!(
                east_attempts < requests.len(),
                "all attempts targeted East US even after persistent 403/3 — this is \
                 the production retry-pin shape. attempts={requests:?}",
            );
        }
        // Pre-fix failure path: persistent 403/3, all attempts pinned to
        // East US, bubble-up after N attempts. This branch is what the
        // assertion below will trip on against the unchanged SDK and is
        // the precise production-pin shape.
        Err(err) => {
            let diagnostics = err
                .diagnostics()
                .expect("a wire-level 403/3 must carry diagnostics");
            let requests = diagnostics.requests();
            let east_attempts = requests
                .iter()
                .filter(|r| r.region().map(|r| r == &Region::EAST_US).unwrap_or(false))
                .count();
            let west_attempts = requests
                .iter()
                .filter(|r| r.region().map(|r| r == &Region::WEST_US).unwrap_or(false))
                .count();
            panic!(
                "Basic 403/3 failover regression: persistent 403/3 on East with a healthy \
                 West must reroute. Observed request_count={} with east_attempts={} / \
                 west_attempts={}. NOTE: this is NOT the production pin shape — that pin needs \
                 the PPCB override cache populated (5+ consecutive failures across many \
                 partition keys); see the `_reproduces_prod_403_3_loop` unit test in \
                 `operation_pipeline::tests` for the actual production pinning shape. \
                 attempts={requests:?}",
                diagnostics.request_count(),
                east_attempts,
                west_attempts,
            );
        }
        Ok(None) => panic!("create must return a response body on the post-fix path"),
    }
}

/// **403/3 — backend-driven failover honors caller's `excluded_regions`.**
///
/// 403/3 (WriteForbidden) is the backend signaling write-region rotation,
/// not an account-topology change. Customer-supplied `excluded_regions`
/// remain meaningful, so the SDK must not cross into an excluded region
/// when looking for a fallback.
///
/// Setup: multi-write account [East US, West US], caller sets
/// `excluded_regions = [West US]` (pin to East), East returns 403/3
/// persistently. With West excluded the SDK has no fallback target, so
/// the operation must bubble up rather than silently routing into West.
#[tokio::test]
async fn write_403_3_retry_honors_excluded_region() {
    let recorder = HostRecorder::new();
    let rule = region_fault_rule(
        "403-3-create-east-honors-excluded",
        FaultOperationType::CreateItem,
        Region::EAST_US,
        FaultInjectionErrorType::WriteForbidden,
        None,
    );
    let (driver, _account) =
        build_driver_with_faults(WriteMode::Multi, recorder.clone(), vec![rule.clone()]).await;

    recorder.clear();

    let container = driver
        .resolve_container_by_name(DB_NAME, COLL_NAME)
        .await
        .expect("container resolves");
    let item_ref = ItemReference::from_name(
        &container,
        PartitionKey::from(PK_VALUE),
        "403-3-honors-excluded-item".to_string(),
    );
    let body = serde_json::json!({
        "id": "403-3-honors-excluded-item",
        "pk": PK_VALUE,
        "value": 4,
    });
    let op = CosmosOperation::create_item(item_ref).with_body(serde_json::to_vec(&body).unwrap());

    let mut opts = OperationOptions::default();
    opts.excluded_regions = Some(ExcludedRegions::from_iter([Region::WEST_US]));

    // West is excluded, East persistently 403/3 → SDK exhausts the 120-attempt
    // backend-failover budget with 1s spacing before bubbling up; 180s leaves
    // headroom for CI scheduling jitter on the ~120s nominal cost.
    let result = tokio::time::timeout(
        std::time::Duration::from_secs(180),
        driver.execute_operation(op, opts),
    )
    .await
    .expect("operation must not hang when its only fallback region is excluded");

    assert!(
        result.is_err(),
        "with West excluded and East persistently returning 403/3, the operation \
         must bubble up rather than crossing into the excluded region; got {result:?}",
    );
    assert!(rule.hit_count() >= 1, "fault rule must fire on East US");

    let hosts = recorder.data_plane_hosts();
    assert!(
        hosts.iter().all(|h| h != WEST_HOST),
        "no data-plane request may reach West US (the excluded region) \
         during 403/3 retries; observed hosts={hosts:?}",
    );
}

/// **403/1008 — backend-driven failover honors caller's
/// `excluded_regions`.** Same rationale as the 403/3 sibling test:
/// the caller's regional exclusion is sacred. Even though 1008 means
/// the cached topology is stale, the SDK must not unilaterally route
/// into a region the customer explicitly opted out of. With the only
/// fallback region excluded and the primary persistently returning
/// 1008, the operation must bubble up rather than cross into West US.
#[tokio::test]
async fn create_item_403_1008_retry_honors_excluded_region() {
    let recorder = HostRecorder::new();
    let rule = region_fault_rule(
        "403-1008-create-east-honors-excluded",
        FaultOperationType::CreateItem,
        Region::EAST_US,
        FaultInjectionErrorType::DatabaseAccountNotFound,
        None,
    );
    let (driver, _account) =
        build_driver_with_faults(WriteMode::Multi, recorder.clone(), vec![rule.clone()]).await;

    recorder.clear();

    let container = driver
        .resolve_container_by_name(DB_NAME, COLL_NAME)
        .await
        .expect("container resolves");
    let item_ref = ItemReference::from_name(
        &container,
        PartitionKey::from(PK_VALUE),
        "403-1008-honors-excluded-item".to_string(),
    );
    let body = serde_json::json!({
        "id": "403-1008-honors-excluded-item",
        "pk": PK_VALUE,
        "value": 5,
    });
    let op = CosmosOperation::create_item(item_ref).with_body(serde_json::to_vec(&body).unwrap());

    let mut opts = OperationOptions::default();
    opts.excluded_regions = Some(ExcludedRegions::from_iter([Region::WEST_US]));

    // 120-attempt budget x 1000ms BACKEND_FAILOVER_RETRY_INTERVAL = up to ~120s wall time.
    let result = tokio::time::timeout(
        std::time::Duration::from_secs(180),
        driver.execute_operation(op, opts),
    )
    .await
    .expect("operation must not hang when its only fallback region is excluded");

    assert!(
        result.is_err(),
        "with West excluded and East persistently returning 403/1008, the operation \
         must bubble up rather than crossing into the excluded region; got {result:?}",
    );
    assert!(rule.hit_count() >= 1, "fault rule must fire on East US");

    let hosts = recorder.data_plane_hosts();
    assert!(
        hosts.iter().all(|h| h != WEST_HOST),
        "no data-plane request may reach West US (the excluded region) \
         during 403/1008 retries; observed hosts={hosts:?}",
    );
}

/// **GetDatabaseAccount metadata refresh is independent of
/// `excluded_regions`.**
///
/// `excluded_regions` is a per-operation, data-plane-routing preference.
/// It must NOT filter the SDK's account-topology probe (`GET /`):
/// excluding a region from data traffic should not blind the client to
/// topology changes happening at that region's endpoint, and on
/// accounts whose current write region is in the caller's exclusion
/// list, honoring the exclusion for metadata would leave the client
/// unable to learn the new topology at all.
///
/// Setup: multi-write account [East US, West US] (East US is also the
/// global bootstrap endpoint, so `GET /` naturally targets East).
/// Caller sets `excluded_regions = [East US]` (the global!) and West
/// returns 403/1008 persistently on `CreateItem`. The 1008 handler
/// requests `RefreshAccountProperties` on every retry; the metadata
/// probe must continue to hit East despite East being excluded.
///
/// Assertions:
/// - At least one `GET /` request was observed against East after the
///   recorder was cleared (proves the SDK reached the excluded region
///   for metadata).
/// - No `GET /` was sent to West — `GET /` is bound to the global
///   endpoint, not the exclusion-filtered data-plane list.
#[tokio::test]
async fn metadata_refresh_ignores_excluded_regions() {
    let recorder = HostRecorder::new();
    let rule = region_fault_rule(
        "403-1008-west-metadata-refresh-ignores-excluded",
        FaultOperationType::CreateItem,
        Region::WEST_US,
        FaultInjectionErrorType::DatabaseAccountNotFound,
        None,
    );
    let (driver, _account) =
        build_driver_with_faults(WriteMode::Multi, recorder.clone(), vec![rule.clone()]).await;

    // Drop bootstrap-time observations so the assertions below only
    // reflect activity triggered by the post-clear operation.
    recorder.clear();

    let container = driver
        .resolve_container_by_name(DB_NAME, COLL_NAME)
        .await
        .expect("container resolves");
    let item_ref = ItemReference::from_name(
        &container,
        PartitionKey::from(PK_VALUE),
        "metadata-refresh-ignores-excluded-item".to_string(),
    );
    let body = serde_json::json!({
        "id": "metadata-refresh-ignores-excluded-item",
        "pk": PK_VALUE,
        "value": 6,
    });
    let op = CosmosOperation::create_item(item_ref).with_body(serde_json::to_vec(&body).unwrap());

    let mut opts = OperationOptions::default();
    // Exclude East — the same region that hosts the global bootstrap
    // endpoint. If excluded_regions incorrectly filtered metadata
    // requests, the SDK would have nowhere to send `GET /`.
    opts.excluded_regions = Some(ExcludedRegions::from_iter([Region::EAST_US]));

    // 120-attempt budget x 1000ms BACKEND_FAILOVER_RETRY_INTERVAL = up
    // to ~120s wall time on bubble-up; 180s leaves CI headroom.
    let result = tokio::time::timeout(
        std::time::Duration::from_secs(180),
        driver.execute_operation(op, opts),
    )
    .await
    .expect("operation must not hang while the SDK retries on the only non-excluded region");

    assert!(
        result.is_err(),
        "with East excluded and West persistently 403/1008, the operation must \
         bubble up rather than cross into the excluded region; got {result:?}",
    );
    assert!(
        rule.hit_count() >= 1,
        "fault rule must fire on West US (the only non-excluded region)",
    );

    let topology_hosts = recorder.topology_hosts();
    assert!(
        topology_hosts.iter().any(|h| h == EAST_HOST),
        "metadata refresh must hit East US (the global bootstrap endpoint) \
         even though East is in the caller's excluded_regions list; \
         observed topology_hosts={topology_hosts:?}",
    );
    assert!(
        topology_hosts.iter().all(|h| h != WEST_HOST),
        "`GET /` is always routed via the global endpoint, not the \
         excluded-region-filtered data-plane list; observed \
         topology_hosts={topology_hosts:?}",
    );
}
