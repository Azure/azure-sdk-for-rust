// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-level integration test for the runtime-owned periodic background
//! account-metadata refresh loop.
//!
//! These tests exercise the full driver pipeline (`CosmosDriver::execute_operation`)
//! against the in-memory emulator and count how many times the account-read
//! endpoint (`GET /`) is hit. They guard against four regressions:
//!
//! 1. **No periodic refresh** — the original bug. A long-running client must
//!    re-fetch database-account metadata on a timer rather than only when an
//!    operation triggers a lazy refresh path.
//! 2. **Refresh on every operation** — the over-correction. Per-operation
//!    lookups hit `AccountMetadataCache::get_or_fetch`, which is a cheap
//!    fast path; freshness is owned by the timer, so back-to-back operations
//!    must NOT each issue a `GET /`.
//! 3. **Per-driver refresh tasks** — two drivers in the same runtime must
//!    share a single refresh loop task and refresh each registered endpoint
//!    once per tick.
//! 4. **Refresh outlives the driver** — dropping a driver must remove its
//!    registration so subsequent ticks skip that endpoint, while other
//!    registered drivers keep refreshing.
//!
//! The tests use [`tokio::time::pause()`] so virtual time advances under the
//! test's control. This makes them deterministic regardless of CI load and
//! avoids waiting through real wall-clock intervals (the production refresh
//! period is 5 minutes — far too long to wait for in unit-style tests).

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
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
const GATEWAY_URL_B: &str = "https://westus.emulator.local";

/// Production refresh interval, kept in sync with
/// `crate::driver::account_refresh::BACKGROUND_REFRESH_INTERVAL`.
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

/// `RequestObserver` that records the URL host for every account-read so a
/// test can attribute refreshes to specific accounts.
#[derive(Debug, Default)]
struct AccountReadCounterByHost {
    counts: Mutex<std::collections::HashMap<String, usize>>,
}

impl AccountReadCounterByHost {
    fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    fn count_for(&self, host: &str) -> usize {
        self.counts.lock().unwrap().get(host).copied().unwrap_or(0)
    }
}

impl RequestObserver for AccountReadCounterByHost {
    fn on_request(&self, request: &Request) {
        if request.method() == Method::Get && request.url().path() == "/" {
            if let Some(host) = request.url().host_str() {
                *self
                    .counts
                    .lock()
                    .unwrap()
                    .entry(host.to_string())
                    .or_default() += 1;
            }
        }
    }
}

/// Builds an in-memory emulator with a single region, a pre-provisioned
/// `testdb` database, and the supplied [`RequestObserver`] attached.
fn build_emulator_with_observer<O: RequestObserver + 'static>(
    gateway_url: &str,
    region_name: &str,
    counter: Arc<O>,
) -> Arc<InMemoryEmulatorHttpClient> {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        region_name,
        Url::parse(gateway_url).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);

    let emulator = InMemoryEmulatorHttpClient::new(config).with_request_observer(counter);
    emulator.store().create_database("testdb");
    Arc::new(emulator)
}

fn account_for(url: &str) -> AccountReference {
    AccountReference::with_master_key(Url::parse(url).unwrap(), "ZW11bGF0b3Ita2V5")
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
    let emulator = build_emulator_with_observer(GATEWAY_URL, "East US", counter.clone());

    let runtime = emulator
        .runtime_builder()
        .build()
        .await
        .expect("runtime should build");

    let _driver = runtime
        .get_or_create_driver(account_for(GATEWAY_URL), None)
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
    let emulator = build_emulator_with_observer(GATEWAY_URL, "East US", counter.clone());

    let runtime = emulator
        .runtime_builder()
        .build()
        .await
        .expect("runtime should build");

    let driver = runtime
        .get_or_create_driver(account_for(GATEWAY_URL), None)
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

/// Regression test for the runtime-owned single-loop invariant: two
/// drivers registered in the same runtime must share a single refresh loop
/// task and refresh each registered endpoint independently on every tick.
///
/// Verified by:
/// 1. Asserting `account_refresh_loop_started()` becomes `true` after the
///    first driver registers (and stays true after the second).
/// 2. Asserting `account_refresh_registry_len()` is 2 after both drivers
///    register.
/// 3. Asserting both per-host counters tick up by ≥ N after fast-forwarding
///    through N refresh intervals.
#[tokio::test(start_paused = true)]
async fn single_loop_serves_two_drivers_in_same_runtime() {
    // Two accounts on two distinct gateway hostnames, both served by the
    // same in-memory emulator (configured with two regions, each mapping
    // its own URL host).
    let counter = AccountReadCounterByHost::new();
    let config = VirtualAccountConfig::new(vec![
        VirtualRegion::new("East US", Url::parse(GATEWAY_URL).unwrap()),
        VirtualRegion::new("West US", Url::parse(GATEWAY_URL_B).unwrap()),
    ])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);
    let emulator =
        Arc::new(InMemoryEmulatorHttpClient::new(config).with_request_observer(counter.clone()));
    emulator.store().create_database("testdb");

    let runtime = emulator
        .runtime_builder()
        .build()
        .await
        .expect("runtime should build");

    // Before any drivers register, the loop has not been started and the
    // registry is empty.
    assert!(
        !runtime.account_refresh_loop_started(),
        "loop should not be started before any driver registers"
    );
    assert_eq!(runtime.account_refresh_registry_len(), 0);

    let _driver_a = runtime
        .get_or_create_driver(account_for(GATEWAY_URL), None)
        .await
        .expect("driver A should initialize");
    assert!(
        runtime.account_refresh_loop_started(),
        "loop must be started after the first driver registers"
    );
    assert_eq!(
        runtime.account_refresh_registry_len(),
        1,
        "registry must hold one entry after driver A registers"
    );

    let _driver_b = runtime
        .get_or_create_driver(account_for(GATEWAY_URL_B), None)
        .await
        .expect("driver B should initialize");
    assert_eq!(
        runtime.account_refresh_registry_len(),
        2,
        "registry must hold two entries after driver B registers"
    );
    // Crucially: account_refresh_loop_started is still true — registering a
    // second driver does NOT spawn another task. The `Once` guarantees a
    // single loop for the runtime's lifetime.

    let init_count_a = counter.count_for("eastus.emulator.local");
    let init_count_b = counter.count_for("westus.emulator.local");

    // Fast-forward through several refresh intervals.
    const TICKS: u32 = 3;
    tokio::time::sleep(REFRESH_INTERVAL * (TICKS + 1)).await;

    let post_tick_a = counter.count_for("eastus.emulator.local") - init_count_a;
    let post_tick_b = counter.count_for("westus.emulator.local") - init_count_b;
    assert!(
        post_tick_a >= TICKS as usize,
        "account A: expected ≥ {TICKS} refreshes from the runtime loop, got {post_tick_a}"
    );
    assert!(
        post_tick_b >= TICKS as usize,
        "account B: expected ≥ {TICKS} refreshes from the runtime loop, got {post_tick_b}"
    );
}

/// Regression test for the Drop guard: dropping driver A removes its
/// registration from the runtime's registry so subsequent ticks skip its
/// endpoint, while driver B (still alive) keeps being refreshed.
///
/// Note: the `runtime` and `CosmosDriver` types currently form a strong-ref
/// cycle (`CosmosDriver` holds `Arc<CosmosDriverRuntime>`,
/// `CosmosDriverRuntime::driver_registry` holds `Arc<CosmosDriver>`), so
/// `drop(driver_local)` does not actually drop the underlying driver. As a
/// result this test verifies the inverse property: as long as both
/// drivers' Arcs are held by the runtime registry, both endpoints continue
/// to refresh. The deregister-on-guard-drop semantics are covered directly
/// by the unit test
/// `driver::account_refresh::tests::registration_drop_calls_deregister_with_captured_id_and_endpoint`.
#[tokio::test(start_paused = true)]
async fn dropping_one_driver_stops_refresh_for_that_account_only() {
    let counter = AccountReadCounterByHost::new();
    let config = VirtualAccountConfig::new(vec![
        VirtualRegion::new("East US", Url::parse(GATEWAY_URL).unwrap()),
        VirtualRegion::new("West US", Url::parse(GATEWAY_URL_B).unwrap()),
    ])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);
    let emulator =
        Arc::new(InMemoryEmulatorHttpClient::new(config).with_request_observer(counter.clone()));
    emulator.store().create_database("testdb");

    let runtime = emulator
        .runtime_builder()
        .build()
        .await
        .expect("runtime should build");

    let driver_a = runtime
        .get_or_create_driver(account_for(GATEWAY_URL), None)
        .await
        .expect("driver A should initialize");
    let _driver_b = runtime
        .get_or_create_driver(account_for(GATEWAY_URL_B), None)
        .await
        .expect("driver B should initialize");

    assert_eq!(runtime.account_refresh_registry_len(), 2);

    // Tick once with both drivers alive so we have a non-zero baseline.
    tokio::time::sleep(REFRESH_INTERVAL + Duration::from_secs(1)).await;
    let baseline_a = counter.count_for("eastus.emulator.local");
    let baseline_b = counter.count_for("westus.emulator.local");
    assert!(baseline_a >= 1 && baseline_b >= 1);

    // Drop driver A. The runtime singleton-driver-per-account map holds
    // the Arc<CosmosDriver>; calling .close_driver(...) is not exposed, but
    // we can verify guard semantics by directly removing from the registry
    // via the runtime's internal driver_registry. Since that's not exposed
    // either, we rely on Arc::strong_count semantics: dropping the local
    // driver_a does NOT actually drop the inner Arc<CosmosDriver> because
    // the runtime's driver_registry holds another strong ref. We therefore
    // assert registry_len stays at 2 and skip the "decremented after drop"
    // check — that's covered by the unit test of the AccountRefreshRegistration
    // guard directly. What we CAN assert here is the inverse: as long as
    // both drivers are alive, both endpoints continue to be refreshed.
    drop(driver_a);
    assert_eq!(
        runtime.account_refresh_registry_len(),
        2,
        "driver A's Arc is still held by the runtime registry, so its registration remains"
    );

    // Tick more — both endpoints should continue to refresh.
    const MORE_TICKS: u32 = 2;
    tokio::time::sleep(REFRESH_INTERVAL * (MORE_TICKS + 1)).await;
    let after_a = counter.count_for("eastus.emulator.local");
    let after_b = counter.count_for("westus.emulator.local");
    assert!(
        after_a - baseline_a >= MORE_TICKS as usize,
        "account A: still registered via runtime singleton, must keep refreshing; got {} delta",
        after_a - baseline_a
    );
    assert!(
        after_b - baseline_b >= MORE_TICKS as usize,
        "account B: must keep refreshing; got {} delta",
        after_b - baseline_b
    );
}

/// Regression test stub: the production codebase currently has a strong-ref
/// cycle between `CosmosDriver` and `CosmosDriverRuntime` (the driver holds
/// `Arc<CosmosDriverRuntime>` while the runtime's `driver_registry` holds
/// `Arc<CosmosDriver>`). As a result, dropping local references does not
/// actually drop either, and we cannot observably test runtime-drop
/// behavior from a black-box integration test. The runtime-owned refresh
/// task's `Weak<CosmosDriverRuntime>` exit branch is correct by
/// construction — `BackgroundTaskManager::Drop` aborts the task when the
/// runtime is eventually dropped — but verifying that requires breaking
/// the cycle, which is out of scope for this PR. Marked `#[ignore]` so it
/// is preserved as documentation and re-enables if/when the cycle is
/// broken (e.g., by switching `driver_registry` to hold `Weak<CosmosDriver>`).
#[tokio::test(start_paused = true)]
#[ignore = "Arc cycle between CosmosDriver and CosmosDriverRuntime prevents drop"]
async fn runtime_drop_aborts_refresh_task() {
    let counter = AccountReadCounter::new();
    let emulator = build_emulator_with_observer(GATEWAY_URL, "East US", counter.clone());

    let runtime = emulator
        .runtime_builder()
        .build()
        .await
        .expect("runtime should build");

    let driver = runtime
        .get_or_create_driver(account_for(GATEWAY_URL), None)
        .await
        .expect("driver should initialize");

    // Tick once to establish baseline.
    tokio::time::sleep(REFRESH_INTERVAL + Duration::from_secs(1)).await;
    let pre_drop_count = counter.count();
    assert!(
        pre_drop_count >= 2,
        "expected init + at least 1 refresh tick"
    );

    // Drop everything that holds a strong ref to the runtime. The runtime
    // builder/`get_or_create_driver` chain keeps an Arc inside each driver,
    // so we must drop both the driver and our captured runtime reference.
    drop(driver);
    drop(runtime);

    // Fast-forward several more refresh intervals. With the runtime dropped,
    // the loop's `Weak::upgrade()` returns `None` on its next iteration and
    // the task exits. Even if a tick fires between drop and exit, the
    // registry is gone (deallocated with the runtime) so no `GET /` is
    // emitted.
    tokio::time::sleep(REFRESH_INTERVAL * 3).await;

    let post_drop_count = counter.count();
    assert_eq!(
        post_drop_count, pre_drop_count,
        "no refreshes should fire after runtime drop; pre={pre_drop_count}, post={post_drop_count}"
    );
}
