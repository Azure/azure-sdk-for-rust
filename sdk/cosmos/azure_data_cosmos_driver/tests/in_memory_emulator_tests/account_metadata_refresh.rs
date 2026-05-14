// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-level integration test for periodic database-account metadata
//! refresh.
//!
//! This test exercises the full driver pipeline (`CosmosDriver::execute_operation`)
//! against the in-memory emulator and counts how many times the account-read
//! endpoint (`GET /`) is hit. It exists as a regression guard against the bug
//! where the per-operation cache lookup used a non-staleness-aware method
//! (`get_or_fetch`) and therefore re-fetched account metadata exactly once
//! per process lifetime, regardless of how long the workload ran.
//!
//! See `account_metadata_cache::tests::refresh_if_stale_refreshes_repeatedly_for_long_running_workload`
//! for the cache-level companion test.

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
use azure_data_cosmos_driver::options::OperationOptions;

const GATEWAY_URL: &str = "https://eastus.emulator.local";

/// Counts requests whose URL path is `/` and method is `GET` — the database
/// account read endpoint that the driver hits to refresh account metadata.
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

fn build_emulator() -> Arc<InMemoryEmulatorHttpClient> {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);

    let emulator = InMemoryEmulatorHttpClient::new(config);
    let store = emulator.store();
    store.create_database("testdb");
    Arc::new(emulator)
}

fn account() -> AccountReference {
    AccountReference::with_master_key(Url::parse(GATEWAY_URL).unwrap(), "ZW11bGF0b3Ita2V5")
}

/// Regression test for the long-running workload bug: the per-operation
/// account metadata lookup must trigger a fresh `GET /` once the staleness
/// threshold elapses, instead of reusing the very first cached response
/// for the lifetime of the driver.
#[tokio::test]
async fn database_account_is_refetched_after_staleness_threshold_via_full_pipeline() {
    // Use a tiny threshold so the test can simulate multiple "stale" windows
    // in a fraction of a second instead of 10 real minutes.
    let staleness_threshold = Duration::from_millis(50);

    let emulator = build_emulator();
    let counter = AccountReadCounter::new();
    let emulator_with_observer = Arc::new(
        InMemoryEmulatorHttpClient::new(emulator.store().config().clone())
            .with_request_observer(counter.clone()),
    );
    // Carry the pre-provisioned database into the observed emulator instance
    // by reusing the same store.
    emulator_with_observer.store().create_database("testdb");

    let runtime = emulator_with_observer
        .runtime_builder()
        .with_account_metadata_staleness_threshold(staleness_threshold)
        .build()
        .await
        .expect("runtime should build");

    // Driver creation triggers the initial account-properties fetch.
    let driver = runtime
        .get_or_create_driver(account(), None)
        .await
        .expect("driver should initialize against the in-memory emulator");

    let after_init = counter.count();
    assert!(
        after_init >= 1,
        "expected at least one account-read during driver init, got {after_init}"
    );

    let db_ref = DatabaseReference::from_name(driver.account().clone(), "testdb".to_string());

    // Issue several operations spaced beyond the staleness threshold.
    // Each operation must trigger a fresh `GET /` because the per-operation
    // metadata lookup honors the staleness threshold.
    const POST_INIT_ITERATIONS: usize = 3;
    for _ in 0..POST_INIT_ITERATIONS {
        // Sleep first so the cache is stale by the time the next operation
        // fires its per-operation refresh check.
        tokio::time::sleep(staleness_threshold * 2).await;
        driver
            .execute_operation(
                CosmosOperation::read_database(db_ref.clone()),
                OperationOptions::default(),
            )
            .await
            .expect("read_database should succeed against the in-memory emulator");
    }

    let after_iterations = counter.count();
    let post_init_account_reads = after_iterations - after_init;

    assert_eq!(
        post_init_account_reads, POST_INIT_ITERATIONS,
        "expected {POST_INIT_ITERATIONS} additional account-read calls (one per stale operation), \
         got {post_init_account_reads}; total observed = {after_iterations}, after_init = {after_init}"
    );
}

/// Companion test: within a single staleness window the driver MUST reuse
/// the cached account metadata. This guards against an over-correction
/// where the fix accidentally hammers `GET /` on every operation.
#[tokio::test]
async fn database_account_is_not_refetched_within_staleness_window_via_full_pipeline() {
    // A long threshold so several back-to-back operations stay within the
    // same fresh window.
    let staleness_threshold = Duration::from_secs(60);

    let counter = AccountReadCounter::new();
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);
    let emulator =
        Arc::new(InMemoryEmulatorHttpClient::new(config).with_request_observer(counter.clone()));
    emulator.store().create_database("testdb");

    let runtime = emulator
        .runtime_builder()
        .with_account_metadata_staleness_threshold(staleness_threshold)
        .build()
        .await
        .expect("runtime should build");

    let driver = runtime
        .get_or_create_driver(account(), None)
        .await
        .expect("driver should initialize against the in-memory emulator");

    let after_init = counter.count();

    let db_ref = DatabaseReference::from_name(driver.account().clone(), "testdb".to_string());

    // Issue several back-to-back operations with no sleep between them —
    // all should fall within the same fresh window.
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
        "expected 0 additional account-read calls within the fresh window, \
         got {post_init_account_reads}"
    );
}
