// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! End-to-end pipeline test for PPAF (Per-Partition Automatic Failover)
//! **dynamic enablement** -- issue
//! <https://github.com/Azure/azure-sdk-for-rust/issues/4325>.
//!
//! The full contract -- CAS swap correctness, the `true -> false` stale-
//! override clearing in `sync_account_properties`, and the per-op sync hot
//! path -- is covered by unit tests in
//! `src/driver/routing/location_state_store.rs`.
//!
//! The one piece those unit tests **cannot** cover is the *wiring*:
//! that the periodic background account-refresh loop
//! (`start_account_refresh_loop` -> gateway `read_database_account` ->
//! `LocationStateStore::sync_account_properties`) actually fires and
//! propagates a server-side flag flip into the live
//! `PartitionEndpointState`, with no driver restart and no client-side
//! override. This file is the regression guard for that wiring.
//!
//! It uses `tokio::time::pause()` so virtual time advances under the test's
//! control, mirroring the existing pattern in `account_metadata_refresh.rs`
//! (the production refresh interval is 5 minutes -- far too long to wait
//! for in a unit-style test).

use std::sync::Arc;
use std::time::Duration;

use azure_core::http::Url;

use azure_data_cosmos_driver::models::{AccountReference, CosmosOperation, DatabaseReference};
use azure_data_cosmos_driver::options::OperationOptions;
use azure_data_cosmos_driver::{
    in_memory_emulator::{
        ConsistencyLevel, InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion,
    },
    DriverOptions,
};

const GATEWAY_URL: &str = "https://eastus.emulator.local";

/// Production refresh interval, kept in sync with
/// `LocationStateStore::BACKGROUND_REFRESH_INTERVAL` (300 s).
const REFRESH_INTERVAL: Duration = Duration::from_secs(300);

fn build_emulator_with_initial_ppaf(ppaf_enabled: bool) -> Arc<InMemoryEmulatorHttpClient> {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session)
    .with_per_partition_failover(ppaf_enabled);

    let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
    emulator.store().create_database("testdb");
    emulator
}

fn account() -> AccountReference {
    AccountReference::with_master_key(Url::parse(GATEWAY_URL).unwrap(), "ZW11bGF0b3Ita2V5")
}

/// Drives one trivial operation through the full pipeline so the snapshot
/// we observe after the background refresh is the post-refresh one, and so
/// we also exercise the consumer hot path that reads the live flag.
async fn read_database(driver: &azure_data_cosmos_driver::CosmosDriver) {
    let db_ref = DatabaseReference::from_name(driver.account().clone(), "testdb".to_string());
    driver
        .execute_operation(
            CosmosOperation::read_database(db_ref),
            OperationOptions::default(),
        )
        .await
        .expect("read_database should succeed against the in-memory emulator");
}

/// **The dynamic-enablement regression test for issue #4325.**
///
/// Starts with PPAF off, has the "operator" flip the server flag to true,
/// and asserts that the driver picks up the change on the next periodic
/// account-refresh tick -- no driver restart, no per-operation refresh, no
/// client-side override.
///
/// This guards the wiring `start_account_refresh_loop` ->
/// `read_database_account` -> `LocationStateStore::sync_account_properties`
/// that the unit tests cannot exercise (they call `sync_account_properties`
/// directly).
#[tokio::test(start_paused = true)]
async fn ppaf_picks_up_runtime_enable_after_background_refresh() {
    let emulator = build_emulator_with_initial_ppaf(false);

    let runtime = emulator
        .runtime_builder()
        .build()
        .await
        .expect("runtime should build");

    let driver = runtime
        .create_driver(DriverOptions::builder(account()).build())
        .await
        .expect("driver should initialize against the in-memory emulator");

    assert!(
        !driver.is_per_partition_automatic_failover_enabled_for_testing(),
        "test setup: PPAF should start disabled"
    );

    // Operator turns PPAF on at the service.
    emulator.store().config().set_per_partition_failover(true);

    // Without a refresh, the driver MUST NOT have observed the change.
    // (No virtual time has advanced and we have issued no operations.)
    assert!(
        !driver.is_per_partition_automatic_failover_enabled_for_testing(),
        "PPAF should still be disabled before the background refresh fires"
    );

    // Advance two refresh intervals to give the bg loop a chance to fire
    // even if the first tick races with the test scheduler under paused
    // time.
    tokio::time::sleep(REFRESH_INTERVAL * 2).await;

    read_database(&driver).await;

    assert!(
        driver.is_per_partition_automatic_failover_enabled_for_testing(),
        "PPAF should be enabled after the background account-refresh loop \
         picks up the server-side flag flip"
    );
}
