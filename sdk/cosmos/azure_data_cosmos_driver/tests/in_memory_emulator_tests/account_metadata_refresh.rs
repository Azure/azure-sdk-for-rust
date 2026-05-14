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
//!    re-fetch database-account metadata on a timer (5-minute default,
//!    matching Java/.NET) rather than only when an operation triggers a
//!    lazy refresh path.
//! 2. **Refresh on every operation** — the over-correction. Per-operation
//!    lookups hit `AccountMetadataCache::get_or_fetch`, which is a cheap
//!    fast path; freshness is owned by the timer, so back-to-back operations
//!    must NOT each issue a `GET /`.

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
use azure_data_cosmos_driver::options::{OperationOptions, OperationOptionsBuilder};

const GATEWAY_URL: &str = "https://eastus.emulator.local";

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
/// This exercises the timer path end-to-end —
/// `CosmosDriver::new` constructs the `LocationStateStore` with the
/// configured `account_metadata_refresh_interval_in_seconds` and
/// `start_account_refresh_loop` spawns the loop via `BackgroundTaskManager`.
/// The original bug had no timer at all, so a client with zero operations
/// would have observed exactly one `GET /` (the init fetch) for its
/// lifetime; this test fails on that behavior.
#[tokio::test]
async fn background_loop_refetches_database_account_with_no_traffic() {
    // OperationOptions only accept whole-second granularity, so 1 second is
    // the smallest interval we can configure through the public API.
    let interval_secs: u32 = 1;
    let counter = AccountReadCounter::new();
    let emulator = build_emulator_with_observer(counter.clone());

    let runtime = emulator
        .runtime_builder()
        .with_operation_options(
            OperationOptionsBuilder::new()
                .with_account_metadata_refresh_interval_in_seconds(interval_secs)
                .build(),
        )
        .build()
        .await
        .expect("runtime should build");

    let _driver = runtime
        .get_or_create_driver(account(), None)
        .await
        .expect("driver should initialize against the in-memory emulator");

    let after_init = counter.count();
    assert!(
        after_init >= 1,
        "expected at least one account-read during driver init, got {after_init}"
    );

    // Wait long enough for several timer ticks. Issue NO operations.
    // We pad generously beyond the minimum expected ticks (`MIN_TICKS`) so
    // the assertion is not flaky under load on slow CI runners; the test's
    // purpose is to prove the timer fires AT ALL (the original bug had no
    // timer), not to land on an exact tick count.
    const MIN_TICKS: u32 = 2;
    let wait_secs: u64 = (MIN_TICKS as u64 + 4) * interval_secs as u64;
    tokio::time::sleep(Duration::from_secs(wait_secs)).await;

    let post_init_account_reads = counter.count() - after_init;
    assert!(
        post_init_account_reads >= MIN_TICKS as usize,
        "expected at least {MIN_TICKS} additional account-read calls from the background timer with no traffic, \
         got {post_init_account_reads}; total observed = {}, after_init = {after_init}",
        counter.count()
    );
}

/// Companion regression test: per-operation lookups must NOT issue a
/// `GET /` on every operation, because freshness is owned by the
/// background refresh loop. Configures the timer with a long interval
/// (so it does not fire during the test) and confirms the per-operation
/// path adds zero account reads on top of the init fetch.
#[tokio::test]
async fn back_to_back_operations_do_not_trigger_per_request_account_reads() {
    let counter = AccountReadCounter::new();
    let emulator = build_emulator_with_observer(counter.clone());

    let runtime = emulator
        .runtime_builder()
        .with_operation_options(
            OperationOptionsBuilder::new()
                // Long enough that the background loop does not fire during
                // the back-to-back operations below.
                .with_account_metadata_refresh_interval_in_seconds(3600)
                .build(),
        )
        .build()
        .await
        .expect("runtime should build");

    let driver = runtime
        .get_or_create_driver(account(), None)
        .await
        .expect("driver should initialize against the in-memory emulator");

    let after_init = counter.count();

    let db_ref = DatabaseReference::from_name(driver.account().clone(), "testdb".to_string());

    // Issue several back-to-back operations with no sleep between them.
    // Per-operation lookup hits AccountMetadataCache::get_or_fetch (cheap
    // fast path) — the background timer is the freshness owner and is
    // configured not to fire during this test.
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
    // We assert `<= 1` rather than `== 0` to remain robust if the very-long
    // (1-hour) timer somehow fires once (e.g., wall-clock jump). The purpose
    // of the assertion is to prove that the per-operation hot path itself
    // is NOT issuing GET / per request — even one extra account-read across
    // 10 back-to-back operations would still demonstrate that property.
    assert!(
        post_init_account_reads <= 1,
        "expected the per-operation hot path to issue 0 account-read calls during \
         back-to-back operations (background timer was configured for 1 hour); \
         got {post_init_account_reads}"
    );
}
