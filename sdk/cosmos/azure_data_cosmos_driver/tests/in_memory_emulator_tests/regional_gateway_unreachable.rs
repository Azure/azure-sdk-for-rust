// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-level integration tests for the regional gateway unreachable
//! scenario — the failure mode the production perf workload exposed when
//! a regional Cosmos DB gateway becomes TCP-unreachable.
//!
//! # Behaviors under test
//!
//! ## B1 — Reads must failover
//!
//! When the regional gateway for the application's primary read region is
//! unreachable at the TCP layer (connect refused, RST, packet drop — *not*
//! a 5xx/449/1002 application-layer response), and another preferred
//! read region exists, the next read attempt must land on that next
//! region's regional endpoint and succeed.
//!
//! ## B2 — No silent global fallback for writes
//!
//! When the writable region's regional gateway is unreachable at the TCP
//! layer, and `excluded_regions` blocks the only other write region, the
//! operation must terminate as a failure on the regional endpoint. It
//! must NOT silently shift to the account's global endpoint URL and
//! succeed there — that hides the regional outage from monitoring.
//!
//! # Test matrix (13 tests; ★ = encodes the production-observed gap)
//!
//! - R1–R4: reads (Multi/Single, healthy/faulted SECONDARY, excluded sets).
//! - W1–W6: writes (the same matrix, plus the W3/W4 ★ cases for B2).
//! - PPCB-1..3 (★ on 1 and 2): exercise the PPCB override path —
//!   Phase 1 trips PPCB into a `Failover`/`ProbeCandidate` state via a
//!   503 storm; Phase 2 swaps the rule to `ConnectionError`; the
//!   override must NOT bypass `excluded_regions` or resurrect a region
//!   that is now transport-unreachable.
//! - G1, G2: cold-start (topology fetched while healthy, then the
//!   regional endpoint goes unreachable before the first data-plane op).
//!
//! # Assertion strategy
//!
//! The in-memory emulator wraps the fault injector around the emulator
//! transport. A fault-injected request never reaches the emulator's
//! `execute_request`, so the [`HostRecorder`] only observes requests
//! that pass through to a healthy region. The assertion model is
//! therefore:
//!
//! - **Positive**: `recorder.data_plane_hosts()` lists every request
//!   that successfully reached a region. For tests that expect failover
//!   to SECONDARY, the recorder must contain SECONDARY (and only
//!   SECONDARY) data-plane hosts.
//! - **Negative**: tests that expect terminal failure also assert
//!   `recorder.data_plane_hosts()` contains no host outside the
//!   allowed set — silent fallback to SECONDARY (despite exclusion)
//!   would show up here.
//! - **Global guard**: `debug_assert!(!selected.is_global())` at
//!   `operation_pipeline.rs:1040` panics in debug builds (cargo test
//!   runs debug) on any pipeline path that returns the global endpoint
//!   — a hard guardrail every test inherits for free.
//! - **Outcome guard**: every test asserts the operation outcome
//!   (success on the expected region, or terminal transport error).
//! - **Time guard**: every test sets a 5 s
//!   [`EndToEndOperationLatencyPolicy`] so a buggy infinite-retry SDK
//!   fails CI on timeout, not by hanging.

#![cfg(feature = "fault_injection")]

use std::sync::Arc;
use std::time::Duration;

use azure_core::http::Url;

use azure_data_cosmos_driver::driver::CosmosDriver;
use azure_data_cosmos_driver::error::CosmosError;
use azure_data_cosmos_driver::fault_injection::{
    FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
    FaultInjectionRule, FaultInjectionRuleBuilder, FaultOperationType,
};
use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, InMemoryEmulatorHttpClient, ReplicationConfig, VirtualAccountConfig,
    VirtualRegion, WriteMode,
};
use azure_data_cosmos_driver::models::{
    AccountReference, ContainerReference, CosmosOperation, ItemReference, PartitionKey,
};
use azure_data_cosmos_driver::options::{
    DriverOptions, EndToEndOperationLatencyPolicy, ExcludedRegions, OperationOptions, Region,
};

use super::host_recorder::HostRecorder;

// ─────────────────────────────────────────────────────────────────────
// Shared fixture constants
// ─────────────────────────────────────────────────────────────────────

/// Primary region URL — stand-in for the production "centralus" region.
const PRIMARY_URL: &str = "https://eastus.emulator.local";
const PRIMARY_HOST: &str = "eastus.emulator.local";

/// Secondary region URL — stand-in for the production "eastus2" region.
const SECONDARY_URL: &str = "https://westus.emulator.local";
const SECONDARY_HOST: &str = "westus.emulator.local";

const DB_NAME: &str = "testdb";
const COLL_NAME: &str = "testcoll";

/// End-to-end deadline used by every test. Long enough for a normal
/// in-memory operation (which completes in low single-digit ms), short
/// enough that a buggy infinite-retry SDK fails the test on timeout
/// rather than hanging CI.
const E2E_DEADLINE: Duration = Duration::from_secs(5);

// ─────────────────────────────────────────────────────────────────────
// Fixture
// ─────────────────────────────────────────────────────────────────────

/// Owns the emulator, recorder, driver, and resolved container handle
/// for a single test. The fields are all `Arc`/`Clone` so tests can hold
/// them across `await` boundaries without lifetime gymnastics.
struct Fixture {
    /// Captures every request that reaches the emulator (post fault
    /// injection). Tests filter `GET /` topology fetches via
    /// [`HostRecorder::data_plane_hosts`].
    recorder: Arc<HostRecorder>,
    /// Driver under test, already bootstrapped against the two-region
    /// topology and wired through the fault-injecting transport stack.
    driver: Arc<CosmosDriver>,
    /// Resolved container handle reused across all operations in the
    /// test.
    container: ContainerReference,
}

/// Builds the two-region in-memory emulator with the supplied fault
/// rules and bootstraps a driver against it. The driver's preferred
/// regions are `[PRIMARY, SECONDARY]` so PRIMARY is the application's
/// primary read/write target.
///
/// `enable_ppcb` toggles per-partition circuit breaker and a 1-failure
/// trip threshold via env vars; PPCB is read at driver init (not from
/// per-op `OperationOptions`), so tests that exercise the override path
/// must set this to `true` BEFORE the driver bootstraps.
async fn build_fixture(
    write_mode: WriteMode,
    rules: Vec<Arc<FaultInjectionRule>>,
    enable_ppcb: bool,
) -> Fixture {
    if enable_ppcb {
        std::env::set_var("AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED", "true");
        std::env::set_var("AZURE_COSMOS_CIRCUIT_BREAKER_FAILURE_COUNT_FOR_READS", "1");
        std::env::set_var("AZURE_COSMOS_CIRCUIT_BREAKER_FAILURE_COUNT_FOR_WRITES", "1");
    } else {
        std::env::remove_var("AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED");
        std::env::remove_var("AZURE_COSMOS_CIRCUIT_BREAKER_FAILURE_COUNT_FOR_READS");
        std::env::remove_var("AZURE_COSMOS_CIRCUIT_BREAKER_FAILURE_COUNT_FOR_WRITES");
    }
    let recorder = HostRecorder::new();

    let config = VirtualAccountConfig::new(vec![
        VirtualRegion::new("East US", Url::parse(PRIMARY_URL).unwrap()),
        VirtualRegion::new("West US", Url::parse(SECONDARY_URL).unwrap()),
    ])
    .unwrap()
    .with_write_mode(write_mode)
    .with_consistency(ConsistencyLevel::Session)
    .with_replication_config(ReplicationConfig::immediate());

    let emulator = Arc::new(
        InMemoryEmulatorHttpClient::new(config).with_request_observer(Arc::clone(&recorder)
            as Arc<dyn azure_data_cosmos_driver::in_memory_emulator::RequestObserver>),
    );
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
        .expect("runtime builds against in-memory emulator");

    let account = AccountReference::with_master_key(
        Url::parse(PRIMARY_URL).unwrap(),
        // Master keys are base64; the emulator does not validate the
        // signature so any well-formed base64 value works.
        "ZW11bGF0b3Ita2V5",
    );

    let driver_options = DriverOptions::builder(account.clone())
        .with_preferred_regions(vec![Region::EAST_US, Region::WEST_US])
        .build();

    let driver = runtime
        .get_or_create_driver(account, Some(driver_options))
        .await
        .expect("driver initializes against emulator metadata");

    let container = driver
        .resolve_container_by_name(DB_NAME, COLL_NAME)
        .await
        .expect("container resolves");

    Fixture {
        recorder,
        driver,
        container,
    }
}

// ─────────────────────────────────────────────────────────────────────
// Fault rule helpers
// ─────────────────────────────────────────────────────────────────────

/// Builds a `ConnectionError` fault rule for `operation_type` scoped to
/// `region`. The rule fires unconditionally (`probability = 1.0`) so the
/// unreachable simulation is total.
fn connection_error_rule(
    id: &str,
    operation_type: FaultOperationType,
    region: Region,
) -> Arc<FaultInjectionRule> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(operation_type)
        .with_region(region)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ConnectionError)
        .with_probability(1.0)
        .build();
    Arc::new(
        FaultInjectionRuleBuilder::new(id, result)
            .with_condition(condition)
            .build(),
    )
}

/// Builds a `ServiceUnavailable` (503) fault rule. Unlike
/// `ConnectionError`, this synthesizes an HTTP 503 response — the
/// pipeline classifies it as a service-side failure rather than a
/// transport-layer failure. Used for the PPCB Phase-1 "503 storm" that
/// trips the circuit breaker into the override state.
fn service_unavailable_rule(
    id: &str,
    operation_type: FaultOperationType,
    region: Region,
) -> Arc<FaultInjectionRule> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(operation_type)
        .with_region(region)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_probability(1.0)
        .build();
    Arc::new(
        FaultInjectionRuleBuilder::new(id, result)
            .with_condition(condition)
            .build(),
    )
}

// ─────────────────────────────────────────────────────────────────────
// OperationOptions helpers
// ─────────────────────────────────────────────────────────────────────

/// Returns base `OperationOptions` with the standard E2E deadline.
/// Every test uses this as the foundation; assertion-specific fields
/// are layered on top.
fn base_options() -> OperationOptions {
    let mut opts = OperationOptions::default();
    opts.end_to_end_latency_policy = Some(EndToEndOperationLatencyPolicy::new(E2E_DEADLINE));
    opts
}

/// `base_options()` with the supplied regions excluded.
fn options_with_excluded(regions: impl IntoIterator<Item = Region>) -> OperationOptions {
    let mut opts = base_options();
    opts.excluded_regions = Some(ExcludedRegions::from_iter(regions));
    opts
}

/// `base_options()` with PPCB enabled at the lowest possible failure
/// thresholds (1 failure trips the circuit). Used by the PPCB tests so
/// a single Phase-1 fault is sufficient to populate
/// `circuit_breaker_overrides`.
fn ppcb_options(extra_excluded: Option<Vec<Region>>) -> OperationOptions {
    let mut opts = base_options();
    opts.per_partition_circuit_breaker_enabled = Some(true);
    opts.circuit_breaker_failure_count_for_reads = Some(1);
    opts.circuit_breaker_failure_count_for_writes = Some(1);
    if let Some(regions) = extra_excluded {
        opts.excluded_regions = Some(ExcludedRegions::from_iter(regions));
    }
    opts
}

// ─────────────────────────────────────────────────────────────────────
// Operation helpers
// ─────────────────────────────────────────────────────────────────────

/// Seeds an item via the driver (no exclusions) so subsequent reads can
/// observe it. With `ReplicationConfig::immediate()` both regions hold
/// the item synchronously.
async fn seed_item(fixture: &Fixture, item_id: &str, pk: &str) {
    let body = serde_json::json!({"id": item_id, "pk": pk, "value": 1}).to_string();
    let item_ref = ItemReference::from_name(
        &fixture.container,
        PartitionKey::from(pk.to_owned()),
        item_id.to_string(),
    );
    fixture
        .driver
        .execute_operation(
            CosmosOperation::create_item(item_ref).with_body(body.into_bytes()),
            base_options(),
        )
        .await
        .expect("seed write should succeed before faults are activated");
}

/// Issues a `read_item` for `(item_id, pk)` using `opts`.
async fn read_item(
    fixture: &Fixture,
    item_id: &str,
    pk: &str,
    opts: OperationOptions,
) -> Result<Option<azure_data_cosmos_driver::models::CosmosResponse>, CosmosError> {
    let item_ref = ItemReference::from_name(
        &fixture.container,
        PartitionKey::from(pk.to_owned()),
        item_id.to_string(),
    );
    fixture
        .driver
        .execute_operation(CosmosOperation::read_item(item_ref), opts)
        .await
}

/// Issues a `create_item` with a freshly synthesized body for
/// `(item_id, pk)` using `opts`.
async fn create_item(
    fixture: &Fixture,
    item_id: &str,
    pk: &str,
    opts: OperationOptions,
) -> Result<Option<azure_data_cosmos_driver::models::CosmosResponse>, CosmosError> {
    let body = serde_json::json!({"id": item_id, "pk": pk, "value": 1}).to_string();
    let item_ref = ItemReference::from_name(
        &fixture.container,
        PartitionKey::from(pk.to_owned()),
        item_id.to_string(),
    );
    fixture
        .driver
        .execute_operation(
            CosmosOperation::create_item(item_ref).with_body(body.into_bytes()),
            opts,
        )
        .await
}

/// Issues an `upsert_item` for `(item_id, pk)` using `opts`.
async fn upsert_item(
    fixture: &Fixture,
    item_id: &str,
    pk: &str,
    opts: OperationOptions,
) -> Result<Option<azure_data_cosmos_driver::models::CosmosResponse>, CosmosError> {
    let body = serde_json::json!({"id": item_id, "pk": pk, "value": 1}).to_string();
    let item_ref = ItemReference::from_name(
        &fixture.container,
        PartitionKey::from(pk.to_owned()),
        item_id.to_string(),
    );
    fixture
        .driver
        .execute_operation(
            CosmosOperation::upsert_item(item_ref).with_body(body.into_bytes()),
            opts,
        )
        .await
}

// ─────────────────────────────────────────────────────────────────────
// Assertion helpers
// ─────────────────────────────────────────────────────────────────────

/// Asserts every data-plane host in `recorder` (i.e., excluding `GET /`
/// topology fetches) belongs to `allowed`. Useful for tests that pin
/// "every observed regional landing is one of these hosts".
fn assert_all_hosts_in(recorder: &HostRecorder, allowed: &[&str]) {
    let hosts = recorder.data_plane_hosts();
    for host in &hosts {
        assert!(
            allowed.contains(&host.as_str()),
            "data-plane request landed on unexpected host {host:?}; \
             allowed={allowed:?}; full host list={hosts:?}"
        );
    }
}

/// Asserts NO data-plane request landed on the supplied `forbidden`
/// host. This is the primary B2 guard: silent fallback to a region the
/// user excluded — or to the global endpoint — manifests as that host
/// appearing in `data_plane_hosts()`.
fn assert_no_data_plane_host(recorder: &HostRecorder, forbidden: &str) {
    let hosts = recorder.data_plane_hosts();
    assert!(
        !hosts.iter().any(|h| h == forbidden),
        "no data-plane request should have landed on {forbidden:?}; \
         observed hosts={hosts:?}"
    );
}

/// Asserts `result` is a terminal error caused by the
/// `ConnectionError` fault. The SDK wraps a transport-layer connection
/// failure into a `TransportGenerated503(20003)` outer error whose
/// `Details` carry the inner `20010 / "connection error"` cause — the
/// assertion accepts that shape and rejects any other terminal error
/// (most importantly, an HTTP-classified error from a host that should
/// never have been reached).
fn assert_terminal_transport_failure<T: std::fmt::Debug>(result: &Result<T, CosmosError>) {
    let err = result.as_ref().err().unwrap_or_else(|| {
        panic!(
            "operation should fail terminally; got Ok({:?})",
            result.as_ref().ok().unwrap()
        )
    });
    let text = format!("{err}");
    assert!(
        text.contains("connection error") || text.contains("20010"),
        "terminal error must carry the ConnectionError fault signature \
         (inner 20010 / 'connection error'); got error: {err:?}"
    );
}

/// Asserts the fault rule was hit at least `min_count` times. A rule
/// that never fires is a silent test bug — either the rule conditions
/// don't match, or the SDK skipped the region entirely.
fn assert_rule_fired(rule: &FaultInjectionRule, min_count: u32) {
    let hits = rule.hit_count();
    assert!(
        hits >= min_count,
        "fault rule {id:?} fired {hits} times, expected at least {min_count}",
        id = rule.id()
    );
}

// ─────────────────────────────────────────────────────────────────────
// R1–R4: reads
// ─────────────────────────────────────────────────────────────────────

/// **R1** — Multi-master read: PRIMARY unreachable, SECONDARY healthy, no
/// exclusions → read failover lands on SECONDARY and succeeds.
#[tokio::test]
async fn r1_read_multi_primary_unreachable_failover_to_secondary() {
    let fault = connection_error_rule(
        "r1-east-unreachable",
        FaultOperationType::ReadItem,
        Region::EAST_US,
    );
    let fixture = build_fixture(WriteMode::Multi, vec![Arc::clone(&fault)], false).await;
    seed_item(&fixture, "r1-item", "pk1").await;
    fixture.recorder.clear();

    let result = read_item(&fixture, "r1-item", "pk1", base_options()).await;
    assert!(
        result.as_ref().is_ok(),
        "R1: read must succeed via SECONDARY failover; got {result:?}"
    );

    assert_rule_fired(&fault, 1);
    let hosts = fixture.recorder.data_plane_hosts();
    assert!(
        !hosts.is_empty(),
        "R1: SECONDARY should have served the read; recorder data-plane is empty"
    );
    assert_all_hosts_in(&fixture.recorder, &[SECONDARY_HOST]);
    assert_no_data_plane_host(&fixture.recorder, PRIMARY_HOST);
}

/// **R2** — Single-master read: same as R1 but `WriteMode::Single`.
/// Reads still failover even when only one region is writable.
#[tokio::test]
async fn r2_read_single_primary_unreachable_failover_to_secondary() {
    let fault = connection_error_rule(
        "r2-east-unreachable",
        FaultOperationType::ReadItem,
        Region::EAST_US,
    );
    let fixture = build_fixture(WriteMode::Single, vec![Arc::clone(&fault)], false).await;
    seed_item(&fixture, "r2-item", "pk1").await;
    fixture.recorder.clear();

    let result = read_item(&fixture, "r2-item", "pk1", base_options()).await;
    assert!(
        result.as_ref().is_ok(),
        "R2: read must succeed via SECONDARY failover; got {result:?}"
    );

    assert_rule_fired(&fault, 1);
    assert_all_hosts_in(&fixture.recorder, &[SECONDARY_HOST]);
    assert_no_data_plane_host(&fixture.recorder, PRIMARY_HOST);
}

/// **R3** — Both regions unreachable, no exclusions → terminal transport
/// error. The pipeline exhausts both preferred read regions and must
/// surface the failure rather than silently routing elsewhere (e.g.,
/// global).
#[tokio::test]
async fn r3_read_multi_both_regions_unreachable_terminal_failure() {
    let east = connection_error_rule("r3-east", FaultOperationType::ReadItem, Region::EAST_US);
    let west = connection_error_rule("r3-west", FaultOperationType::ReadItem, Region::WEST_US);
    let fixture = build_fixture(
        WriteMode::Multi,
        vec![Arc::clone(&east), Arc::clone(&west)],
        false,
    )
    .await;
    seed_item(&fixture, "r3-item", "pk1").await;
    fixture.recorder.clear();

    let result = read_item(&fixture, "r3-item", "pk1", base_options()).await;
    assert_terminal_transport_failure(&result);

    // Both faults must have fired — if one region was silently skipped
    // entirely, the test isn't exercising the both-regions-down path.
    assert_rule_fired(&east, 1);
    assert_rule_fired(&west, 1);

    // Fault-injected requests never reach the emulator, so no
    // data-plane host should appear.
    let hosts = fixture.recorder.data_plane_hosts();
    assert!(
        hosts.is_empty(),
        "R3: every attempt should have been intercepted by a fault; \
         no data-plane host should appear in the recorder. got={hosts:?}"
    );
}

/// **R4** — PRIMARY unreachable, SECONDARY healthy, but caller excludes
/// SECONDARY. The pipeline has no eligible region and must surface a
/// terminal transport error — it must NOT silently route the read to
/// SECONDARY (despite exclusion) or to the global endpoint.
#[tokio::test]
async fn r4_read_primary_unreachable_secondary_excluded_terminal_failure() {
    let fault = connection_error_rule("r4-east", FaultOperationType::ReadItem, Region::EAST_US);
    let fixture = build_fixture(WriteMode::Multi, vec![Arc::clone(&fault)], false).await;
    seed_item(&fixture, "r4-item", "pk1").await;
    fixture.recorder.clear();

    let result = read_item(
        &fixture,
        "r4-item",
        "pk1",
        options_with_excluded([Region::WEST_US]),
    )
    .await;
    assert_terminal_transport_failure(&result);

    assert_rule_fired(&fault, 1);
    assert_no_data_plane_host(&fixture.recorder, SECONDARY_HOST);
    assert_all_hosts_in(&fixture.recorder, &[/* nothing should pass through */]);
}

// ─────────────────────────────────────────────────────────────────────
// W1–W6: writes
// ─────────────────────────────────────────────────────────────────────

/// **W1** — Healthy baseline. Writes land on PRIMARY only.
#[tokio::test]
async fn w1_write_multi_no_faults_lands_on_primary() {
    let fixture = build_fixture(WriteMode::Multi, Vec::new(), false).await;
    let result = create_item(&fixture, "w1-item", "pk1", base_options()).await;
    assert!(
        result.as_ref().is_ok(),
        "W1: baseline write must succeed; got {result:?}"
    );
    assert_all_hosts_in(&fixture.recorder, &[PRIMARY_HOST]);
}

/// **W2** — Multi-master write: PRIMARY unreachable, SECONDARY healthy,
/// no exclusions → failover to SECONDARY succeeds.
#[tokio::test]
async fn w2_write_multi_primary_unreachable_failover_to_secondary() {
    let fault = connection_error_rule("w2-east", FaultOperationType::CreateItem, Region::EAST_US);
    let fixture = build_fixture(WriteMode::Multi, vec![Arc::clone(&fault)], false).await;
    fixture.recorder.clear();

    let result = create_item(&fixture, "w2-item", "pk1", base_options()).await;
    assert!(
        result.as_ref().is_ok(),
        "W2: multi-write write must failover to SECONDARY; got {result:?}"
    );

    assert_rule_fired(&fault, 1);
    assert_all_hosts_in(&fixture.recorder, &[SECONDARY_HOST]);
    assert_no_data_plane_host(&fixture.recorder, PRIMARY_HOST);
}

/// **W3 ★** — Multi-master write: PRIMARY unreachable, SECONDARY healthy,
/// caller excludes SECONDARY → MUST surface terminal transport error.
/// The B2 scenario: write must NOT silently land on SECONDARY despite
/// the user's exclusion, and must NOT silently shift to the global
/// endpoint.
#[tokio::test]
async fn w3_write_multi_primary_unreachable_secondary_excluded_terminal_failure() {
    let fault = connection_error_rule("w3-east", FaultOperationType::CreateItem, Region::EAST_US);
    let fixture = build_fixture(WriteMode::Multi, vec![Arc::clone(&fault)], false).await;
    fixture.recorder.clear();

    let result = create_item(
        &fixture,
        "w3-item",
        "pk1",
        options_with_excluded([Region::WEST_US]),
    )
    .await;
    assert_terminal_transport_failure(&result);

    assert_rule_fired(&fault, 1);
    assert_no_data_plane_host(&fixture.recorder, SECONDARY_HOST);
    assert_all_hosts_in(
        &fixture.recorder,
        &[/* SECONDARY excluded, PRIMARY faulted */],
    );
}

/// **W4 ★** — Same as W3 but `upsert_item` instead of `create_item`.
/// The override-bypass gap is per partition-routing path, not per
/// operation type — covering upsert pins that the fix isn't accidentally
/// scoped to one verb.
#[tokio::test]
async fn w4_upsert_multi_primary_unreachable_secondary_excluded_terminal_failure() {
    let fault = connection_error_rule("w4-east", FaultOperationType::UpsertItem, Region::EAST_US);
    let fixture = build_fixture(WriteMode::Multi, vec![Arc::clone(&fault)], false).await;
    fixture.recorder.clear();

    let result = upsert_item(
        &fixture,
        "w4-item",
        "pk1",
        options_with_excluded([Region::WEST_US]),
    )
    .await;
    assert_terminal_transport_failure(&result);

    assert_rule_fired(&fault, 1);
    assert_no_data_plane_host(&fixture.recorder, SECONDARY_HOST);
}

/// **W5** — Both regions unreachable, no exclusions → terminal transport
/// error. Write-side analogue of R3.
#[tokio::test]
async fn w5_write_multi_both_regions_unreachable_terminal_failure() {
    let east = connection_error_rule("w5-east", FaultOperationType::CreateItem, Region::EAST_US);
    let west = connection_error_rule("w5-west", FaultOperationType::CreateItem, Region::WEST_US);
    let fixture = build_fixture(
        WriteMode::Multi,
        vec![Arc::clone(&east), Arc::clone(&west)],
        false,
    )
    .await;
    fixture.recorder.clear();

    let result = create_item(&fixture, "w5-item", "pk1", base_options()).await;
    assert_terminal_transport_failure(&result);

    assert_rule_fired(&east, 1);
    assert_rule_fired(&west, 1);
    let hosts = fixture.recorder.data_plane_hosts();
    assert!(
        hosts.is_empty(),
        "W5: every attempt intercepted by a fault; expected empty \
         data-plane host list, got {hosts:?}"
    );
}

/// **W6** — Single-master write: PRIMARY unreachable, SECONDARY read-only,
/// caller excludes SECONDARY. With `WriteMode::Single` SECONDARY can't
/// accept writes anyway; the operation must terminate as transport
/// failure on PRIMARY.
#[tokio::test]
async fn w6_write_single_primary_unreachable_secondary_excluded_terminal_failure() {
    let fault = connection_error_rule("w6-east", FaultOperationType::CreateItem, Region::EAST_US);
    let fixture = build_fixture(WriteMode::Single, vec![Arc::clone(&fault)], false).await;
    fixture.recorder.clear();

    let result = create_item(
        &fixture,
        "w6-item",
        "pk1",
        options_with_excluded([Region::WEST_US]),
    )
    .await;
    assert_terminal_transport_failure(&result);

    assert_rule_fired(&fault, 1);
    assert_no_data_plane_host(&fixture.recorder, SECONDARY_HOST);
}

// ─────────────────────────────────────────────────────────────────────
// PPCB-1..3: per-partition circuit breaker override path
// ─────────────────────────────────────────────────────────────────────

/// Drives `op` `repeat` times against a phase-1 503-storm rule so the
/// PPCB circuit-breaker counters trip. Each invocation is expected to
/// fail (the rule returns 503 for every attempt); the goal is to
/// populate `circuit_breaker_overrides` for the operation's partition.
async fn trip_ppcb_reads(fixture: &Fixture, item_id: &str, pk: &str, repeat: usize) {
    for _ in 0..repeat {
        let _ = read_item(fixture, item_id, pk, ppcb_options(None)).await;
    }
}

async fn trip_ppcb_writes(fixture: &Fixture, item_id: &str, pk: &str, repeat: usize) {
    for _ in 0..repeat {
        let _ = create_item(fixture, item_id, pk, ppcb_options(None)).await;
    }
}

/// **PPCB-1 ★** — Read scenario. Phase-1 503 storm on PRIMARY trips
/// PPCB; Phase-2 swaps PRIMARY to `ConnectionError` and excludes
/// SECONDARY. The PPCB override entry must NOT resurrect SECONDARY
/// (the user excluded it), and PRIMARY's transport failure must NOT
/// silently fall through to global.
#[tokio::test]
async fn ppcb1_read_override_must_respect_excluded_regions() {
    let storm = service_unavailable_rule(
        "ppcb1-east-503",
        FaultOperationType::ReadItem,
        Region::EAST_US,
    );
    let unreachable = connection_error_rule(
        "ppcb1-east-unreachable",
        FaultOperationType::ReadItem,
        Region::EAST_US,
    );
    // Start with unreachable disabled — only the 503 storm fires during
    // Phase 1.
    unreachable.disable();

    let fixture = build_fixture(
        WriteMode::Multi,
        vec![Arc::clone(&storm), Arc::clone(&unreachable)],
        true,
    )
    .await;
    seed_item(&fixture, "ppcb1-item", "pk1").await;

    // Phase 1: 503 storm trips PPCB. Threshold is 1 (set via
    // `ppcb_options`) so one failure populates the override entry.
    trip_ppcb_reads(&fixture, "ppcb1-item", "pk1", 2).await;
    assert_rule_fired(&storm, 1);

    // Phase 2: swap the rule. Disable the 503 storm, enable the
    // unreachable. PPCB state from Phase 1 should persist.
    storm.disable();
    unreachable.enable();
    fixture.recorder.clear();

    // Phase 3: read with SECONDARY excluded.
    let result = read_item(
        &fixture,
        "ppcb1-item",
        "pk1",
        ppcb_options(Some(vec![Region::WEST_US])),
    )
    .await;
    assert_terminal_transport_failure(&result);

    assert_rule_fired(&unreachable, 1);
    assert_no_data_plane_host(&fixture.recorder, SECONDARY_HOST);
}

/// **PPCB-2 ★** — Write scenario. Same setup as PPCB-1 but for
/// `create_item`. Multi-master + PPCB-eligible means the override path
/// at `operation_pipeline.rs:1086-1115` is the most likely site for the
/// silent-substitution bug.
#[tokio::test]
async fn ppcb2_write_override_must_respect_excluded_regions() {
    let storm = service_unavailable_rule(
        "ppcb2-east-503",
        FaultOperationType::CreateItem,
        Region::EAST_US,
    );
    let unreachable = connection_error_rule(
        "ppcb2-east-unreachable",
        FaultOperationType::CreateItem,
        Region::EAST_US,
    );
    unreachable.disable();

    let fixture = build_fixture(
        WriteMode::Multi,
        vec![Arc::clone(&storm), Arc::clone(&unreachable)],
        true,
    )
    .await;

    trip_ppcb_writes(&fixture, "ppcb2-item", "pk1", 2).await;
    assert_rule_fired(&storm, 1);

    storm.disable();
    unreachable.enable();
    fixture.recorder.clear();

    let result = create_item(
        &fixture,
        "ppcb2-item-final",
        "pk1",
        ppcb_options(Some(vec![Region::WEST_US])),
    )
    .await;
    assert_terminal_transport_failure(&result);

    assert_rule_fired(&unreachable, 1);
    assert_no_data_plane_host(&fixture.recorder, SECONDARY_HOST);
}

/// **PPCB-3** — Same setup as PPCB-1 but no exclusion. The override
/// SHOULD succeed via SECONDARY because SECONDARY is healthy and the
/// user did not exclude it. Pins that the PPCB-respects-exclusions fix
/// is scoped — it must not break the happy path where the override is
/// the intended healing mechanism.
#[tokio::test]
async fn ppcb3_read_override_routes_to_healthy_secondary() {
    let storm = service_unavailable_rule(
        "ppcb3-east-503",
        FaultOperationType::ReadItem,
        Region::EAST_US,
    );
    let unreachable = connection_error_rule(
        "ppcb3-east-unreachable",
        FaultOperationType::ReadItem,
        Region::EAST_US,
    );
    unreachable.disable();

    let fixture = build_fixture(
        WriteMode::Multi,
        vec![Arc::clone(&storm), Arc::clone(&unreachable)],
        true,
    )
    .await;
    seed_item(&fixture, "ppcb3-item", "pk1").await;

    trip_ppcb_reads(&fixture, "ppcb3-item", "pk1", 2).await;
    assert_rule_fired(&storm, 1);

    storm.disable();
    unreachable.enable();
    fixture.recorder.clear();

    let result = read_item(&fixture, "ppcb3-item", "pk1", ppcb_options(None)).await;
    assert!(
        result.as_ref().is_ok(),
        "PPCB-3: post-trip read should succeed via SECONDARY (the override's \
         intended healing mechanism); got {result:?}"
    );

    assert_all_hosts_in(&fixture.recorder, &[SECONDARY_HOST]);
    assert_no_data_plane_host(&fixture.recorder, PRIMARY_HOST);
}

// ─────────────────────────────────────────────────────────────────────
// G1, G2: cold-start (topology discovered healthy, then unreachable)
// ─────────────────────────────────────────────────────────────────────

/// **G1** — Cold-start read failover. Driver bootstrap succeeded
/// (topology discovered while PRIMARY was healthy). Before the first
/// data-plane op, PRIMARY goes unreachable. The first read must failover
/// to SECONDARY rather than re-pinning to the just-discovered (now
/// dead) PRIMARY.
#[tokio::test]
async fn g1_cold_start_read_failover_to_secondary() {
    let fault = connection_error_rule("g1-east", FaultOperationType::ReadItem, Region::EAST_US);
    // Critically: disable the fault during bootstrap so topology fetch
    // succeeds, then enable it before the first data-plane op.
    fault.disable();

    let fixture = build_fixture(WriteMode::Multi, vec![Arc::clone(&fault)], false).await;
    seed_item(&fixture, "g1-item", "pk1").await;
    fixture.recorder.clear();

    // Activate the unreachable just before the first read.
    fault.enable();
    let result = read_item(&fixture, "g1-item", "pk1", base_options()).await;
    assert!(
        result.as_ref().is_ok(),
        "G1: cold-start read must failover to SECONDARY; got {result:?}"
    );

    assert_rule_fired(&fault, 1);
    assert_all_hosts_in(&fixture.recorder, &[SECONDARY_HOST]);
    assert_no_data_plane_host(&fixture.recorder, PRIMARY_HOST);
}

/// **G2** — Cold-start write with SECONDARY excluded → terminal
/// transport error. Write-side analogue of G1 + W3.
#[tokio::test]
async fn g2_cold_start_write_secondary_excluded_terminal_failure() {
    let fault = connection_error_rule("g2-east", FaultOperationType::CreateItem, Region::EAST_US);
    fault.disable();

    let fixture = build_fixture(WriteMode::Multi, vec![Arc::clone(&fault)], false).await;
    fixture.recorder.clear();

    fault.enable();
    let result = create_item(
        &fixture,
        "g2-item",
        "pk1",
        options_with_excluded([Region::WEST_US]),
    )
    .await;
    assert_terminal_transport_failure(&result);

    assert_rule_fired(&fault, 1);
    assert_no_data_plane_host(&fixture.recorder, SECONDARY_HOST);
}
