// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-level integration test for the periodic background account-metadata
//! refresh loop.
//!
//! These tests exercise the full driver pipeline (`CosmosDriver::execute_operation`
//! and `LocationStateStore::start_account_refresh_loop`) against the in-memory
//! emulator and count how many times the account-read endpoint (`GET /`) is
//! hit. They guard against two regressions:
//!
//! 1. **No periodic refresh** — the original bug. A long-running client must
//!    re-fetch database-account metadata on a timer rather than only when an
//!    operation triggers a lazy refresh path.
//! 2. **Refresh on every operation** — the over-correction. Per-operation
//!    lookups hit `AccountMetadataCache::get_or_fetch`, which is a cheap
//!    fast path; freshness is owned by the timer, so back-to-back operations
//!    must NOT each issue a `GET /`.
//!
//! The tests use [`tokio::time::pause()`] so virtual time advances under the
//! test's control. This makes them deterministic regardless of CI load and
//! avoids waiting through real wall-clock intervals (the production refresh
//! period is 5 minutes — far too long to wait for in unit-style tests).

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::time::Duration;

use azure_core::http::{Method, Request, Url};

use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, InMemoryEmulatorHttpClient, RequestObserver, VirtualAccountConfig,
    VirtualRegion,
};
use azure_data_cosmos_driver::models::{AccountReference, CosmosOperation, DatabaseReference};
use azure_data_cosmos_driver::options::DriverOptions;
use azure_data_cosmos_driver::options::OperationOptions;

const GATEWAY_URL: &str = "https://eastus.emulator.local";

/// Production refresh interval, kept in sync with
/// `LocationStateStore::BACKGROUND_REFRESH_INTERVAL`.
const REFRESH_INTERVAL: Duration = Duration::from_secs(300);

/// Counts requests whose URL path is `/` and method is `GET` — the database
/// account read endpoint that the driver hits on every refresh.
#[derive(Debug, Default)]
struct AccountReadCounter {
    count: AtomicUsize,
}

impl AccountReadCounter {
    fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    fn count(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }
}

impl RequestObserver for AccountReadCounter {
    fn on_request(&self, request: &Request) {
        if request.method() == Method::Get && request.url().path() == "/" {
            self.count.fetch_add(1, Ordering::SeqCst);
        }
    }
}

/// Builds an in-memory emulator with a single region, a pre-provisioned
/// `testdb` database, and the supplied [`RequestObserver`] attached.
fn build_emulator_with_observer(
    counter: Arc<AccountReadCounter>,
) -> Arc<InMemoryEmulatorHttpClient> {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);

    let emulator = InMemoryEmulatorHttpClient::new(config).with_request_observer(counter);
    emulator.store().create_database("testdb");
    Arc::new(emulator)
}

fn account() -> AccountReference {
    AccountReference::with_master_key(Url::parse(GATEWAY_URL).unwrap(), "ZW11bGF0b3Ita2V5")
}

/// Regression test for the long-running-workload bug: even with NO incoming
/// operations, the driver must periodically re-fetch database-account
/// metadata once the background refresh loop has spawned.
///
/// Uses `start_paused = true` so virtual time advances deterministically.
/// Because the production refresh interval is 5 minutes, only a paused-time
/// runtime can validate this scenario in test-suite-friendly time. The test
/// fast-forwards through several refresh windows and asserts the
/// `RequestObserver` saw at least that many additional `GET /` requests
/// purely from the timer.
///
/// The original bug had no timer at all, so a client with zero operations
/// would have observed exactly one `GET /` (the init fetch) for its
/// lifetime; this test fails on that behavior.
#[tokio::test(start_paused = true)]
async fn background_loop_refetches_database_account_with_no_traffic() {
    let counter = AccountReadCounter::new();
    let emulator = build_emulator_with_observer(counter.clone());

    let runtime = emulator
        .runtime_builder()
        .build()
        .await
        .expect("runtime should build");

    let _driver = runtime
        .create_driver(DriverOptions::builder(account()).build())
        .await
        .expect("driver should initialize against the in-memory emulator");

    let after_init = counter.count();
    assert!(
        after_init >= 1,
        "expected at least one account-read during driver init, got {after_init}"
    );

    // Fast-forward through several refresh intervals. With paused time,
    // `tokio::time::sleep` here auto-advances virtual time and wakes the
    // background loop's sleep at each tick. We add an extra interval of
    // headroom so the final refresh has time to complete before the
    // assertion runs.
    const TICKS: u32 = 3;
    tokio::time::sleep(REFRESH_INTERVAL * (TICKS + 1)).await;

    let post_init_account_reads = counter.count() - after_init;
    assert!(
        post_init_account_reads >= TICKS as usize,
        "expected at least {TICKS} additional account-read calls from the background timer with no traffic, \
         got {post_init_account_reads}; total observed = {}, after_init = {after_init}",
        counter.count()
    );
}

/// Companion regression test: per-operation lookups must NOT issue a
/// `GET /` on every operation, because freshness is owned by the
/// background refresh loop. Issues several back-to-back operations under
/// paused time (so the timer cannot fire) and asserts exactly zero
/// additional account reads happen on top of the init fetch.
#[tokio::test(start_paused = true)]
async fn back_to_back_operations_do_not_trigger_per_request_account_reads() {
    let counter = AccountReadCounter::new();
    let emulator = build_emulator_with_observer(counter.clone());

    let runtime = emulator
        .runtime_builder()
        .build()
        .await
        .expect("runtime should build");

    let driver = runtime
        .create_driver(DriverOptions::builder(account()).build())
        .await
        .expect("driver should initialize against the in-memory emulator");

    let after_init = counter.count();

    let db_ref = DatabaseReference::from_name(driver.account().clone(), "testdb".to_string());

    // Issue several back-to-back operations. Under paused time the
    // background timer cannot fire because we never await a sleep that
    // would advance virtual time. Per-operation lookup hits
    // AccountMetadataCache::get_or_fetch (cheap fast path) — the
    // background timer is the freshness owner.
    for _ in 0..10 {
        driver
            .execute_operation(
                CosmosOperation::read_database(db_ref.clone()),
                OperationOptions::default(),
            )
            .await
            .expect("read_database should succeed against the in-memory emulator");
    }

    let post_init_account_reads = counter.count() - after_init;
    assert_eq!(
        post_init_account_reads, 0,
        "expected the per-operation hot path to issue 0 account-read calls during \
         back-to-back operations under paused virtual time; got {post_init_account_reads}"
    );
}
