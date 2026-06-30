// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-level integration test for probe-gated account endpoint failback.
//!
//! Background (PR #4604, issue #4622): account-level endpoint failback is
//! gated on a connectivity probe. An endpoint marked unavailable rejoins the
//! routing rotation **only** after a background probe confirms it is
//! reachable; the previous time-based auto-clear was removed. The unit tests
//! in `location_state_store.rs` cover the failback state machine with
//! *injected fake* probe closures. This test exercises the **real** probe
//! closure (the one the driver builds from its transport) end to end against
//! the in-memory emulator, covering the three behaviors issue #4622 calls out:
//!
//! 1. **Marked unavailable** — a regional endpoint is unavailable and stays
//!    out of rotation.
//! 2. **Probe-gated failback** — while the region is connection-blocked the
//!    endpoint is *not* failed back (the probe keeps failing); once
//!    connectivity is restored a successful probe fails it back.
//! 3. **Cooldown / re-probe** — a failed probe resets the cooldown so the
//!    endpoint is not immediately re-probed and prematurely failed back.
//!
//! Scope note: the endpoint is *seeded* as unavailable via an internal test
//! hook rather than driven through an operation. Operation-driven marking
//! (connection error -> failover -> mark) is already covered by
//! `regional_gateway_unreachable.rs`; this test isolates the new
//! probe-gated *failback* behavior and drives the **real** connectivity probe
//! against the emulator (blocked vs. healthy) to decide failback.
//!
//! Determinism: instead of waiting for the 60-second background probe loop,
//! the test drives a single probe-and-failback iteration via the internal
//! `run_endpoint_probe_once_for_testing` hook and configures a short
//! `endpoint_unavailability_ttl` so the endpoint becomes probe-eligible
//! quickly.

#![cfg(feature = "fault_injection")]

use std::sync::Arc;
use std::time::Duration;

use azure_core::http::Url;

use azure_data_cosmos_driver::driver::CosmosDriver;
use azure_data_cosmos_driver::fault_injection::{
    FaultInjectionConditionBuilder, FaultInjectionErrorType, FaultInjectionResultBuilder,
    FaultInjectionRule, FaultInjectionRuleBuilder,
};
use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, InMemoryEmulatorHttpClient, ReplicationConfig, VirtualAccountConfig,
    VirtualRegion, WriteMode,
};
use azure_data_cosmos_driver::models::AccountReference;
use azure_data_cosmos_driver::options::{DriverOptions, OperationOptions, Region};

// Shared fixture constants

/// Primary region URL — the endpoint that is taken offline and recovered.
const PRIMARY_URL: &str = "https://eastus.emulator.local";
const PRIMARY_HOST: &str = "eastus.emulator.local";

/// Secondary region URL — keeps the account multi-region.
const SECONDARY_URL: &str = "https://westus.emulator.local";

const DB_NAME: &str = "testdb";
const COLL_NAME: &str = "testcoll";

/// Short unavailability cooldown so a marked endpoint becomes probe-eligible
/// quickly. Kept non-zero so the cooldown-reset behavior is observable.
const UNAVAILABILITY_TTL: Duration = Duration::from_millis(50);

/// Owns the emulator and driver for a single test. The emulator is retained so
/// its transport stays alive for the duration of the test.
struct Fixture {
    _emulator: Arc<InMemoryEmulatorHttpClient>,
    driver: Arc<CosmosDriver>,
}

/// Builds a two-region in-memory emulator with the supplied fault rules and
/// bootstraps a driver against it. The driver is configured with a short
/// endpoint-unavailability cooldown so the probe hook can drive failback
/// without waiting for production-scale timers.
async fn build_fixture(rules: Vec<Arc<FaultInjectionRule>>) -> Fixture {
    let config = VirtualAccountConfig::new(vec![
        VirtualRegion::new("East US", Url::parse(PRIMARY_URL).unwrap()),
        VirtualRegion::new("West US", Url::parse(SECONDARY_URL).unwrap()),
    ])
    .unwrap()
    .with_write_mode(WriteMode::Multi)
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

    let mut operation_options = OperationOptions::default();
    operation_options.endpoint_unavailability_ttl = Some(UNAVAILABILITY_TTL);

    let driver_options = DriverOptions::builder(account)
        .with_preferred_regions(vec![Region::EAST_US, Region::WEST_US])
        .with_operation_options(operation_options)
        .build();

    let driver = runtime
        .create_driver(driver_options)
        .await
        .expect("driver initializes against emulator metadata");

    Fixture {
        _emulator: emulator,
        driver,
    }
}

/// Builds a region-scoped `ConnectionError` rule covering **every** operation
/// to `region` (no operation-type filter), so it also blocks the connectivity
/// probe's `GET /probe`. Created disabled so it does not interfere with driver
/// bootstrap.
fn region_connection_error_rule(id: &str, region: Region) -> Arc<FaultInjectionRule> {
    let condition = FaultInjectionConditionBuilder::new()
        .with_region(region)
        .build();
    let result = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ConnectionError)
        .with_probability(1.0)
        .build();
    let rule = Arc::new(
        FaultInjectionRuleBuilder::new(id, result)
            .with_condition(condition)
            .build(),
    );
    rule.disable();
    rule
}

/// Returns whether the primary endpoint currently carries an account-level
/// unavailability mark.
fn primary_marked_unavailable(fixture: &Fixture) -> bool {
    fixture
        .driver
        .is_endpoint_host_marked_unavailable_for_testing(PRIMARY_HOST)
}

/// Sleeps just past the unavailability cooldown so the next probe sweep treats
/// the endpoint as due.
async fn wait_past_cooldown() {
    tokio::time::sleep(UNAVAILABILITY_TTL + Duration::from_millis(20)).await;
}

/// End-to-end probe-gated failback: a failed probe keeps a marked endpoint out
/// of rotation and resets its cooldown, and only a successful probe (after
/// connectivity is restored) fails it back.
#[tokio::test]
async fn primary_endpoint_fails_back_only_after_successful_probe() {
    let outage = region_connection_error_rule("probe-failback-east-outage", Region::EAST_US);
    let fixture = build_fixture(vec![Arc::clone(&outage)]).await;

    // The primary should be healthy before the outage begins.
    assert!(
        !primary_marked_unavailable(&fixture),
        "primary must start healthy (not marked unavailable)"
    );

    // Phase 1 — outage: block the primary region entirely and mark its
    // endpoint unavailable (standing in for the operation-driven failover that
    // is covered by `regional_gateway_unreachable.rs`).
    outage.enable();
    assert!(
        fixture
            .driver
            .mark_region_endpoint_unavailable_for_testing(&Region::EAST_US),
        "the primary region endpoint must exist in the routing snapshot"
    );
    assert!(
        primary_marked_unavailable(&fixture),
        "the primary endpoint must be marked unavailable after seeding"
    );

    // Phase 2 — probe-gated failback is withheld while the outage persists.
    // Wait past the cooldown so the endpoint is probe-eligible, then run one
    // probe sweep: the probe to the primary fails (connection blocked), so the
    // unavailability mark is retained.
    wait_past_cooldown().await;
    fixture.driver.run_endpoint_probe_once_for_testing().await;
    assert!(
        primary_marked_unavailable(&fixture),
        "a failed probe must NOT fail the endpoint back; it stays unavailable"
    );
    assert!(
        outage.hit_count() >= 1,
        "the probe must have been routed to the blocked primary endpoint \
         (the outage fault should have intercepted it)"
    );

    // Phase 3 — cooldown reset: the failed probe just reset the cooldown, so an
    // immediate second sweep (before the cooldown elapses again) must skip the
    // endpoint rather than re-probe and prematurely fail it back.
    let hits_before_immediate_sweep = outage.hit_count();
    fixture.driver.run_endpoint_probe_once_for_testing().await;
    assert!(
        primary_marked_unavailable(&fixture),
        "a failed probe must reset the cooldown so the endpoint is not \
         immediately re-probed and failed back"
    );
    assert_eq!(
        outage.hit_count(),
        hits_before_immediate_sweep,
        "the endpoint must NOT be re-probed before its cooldown elapses again"
    );

    // Phase 4 — recovery: restore connectivity, wait past the cooldown, and run
    // one more probe sweep. The probe now succeeds, so the endpoint is failed
    // back into rotation.
    outage.disable();
    wait_past_cooldown().await;
    fixture.driver.run_endpoint_probe_once_for_testing().await;
    assert!(
        !primary_marked_unavailable(&fixture),
        "a successful probe (after connectivity is restored) must fail the \
         endpoint back into rotation"
    );
}
