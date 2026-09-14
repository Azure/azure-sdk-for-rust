// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-level integration tests for the `PartitionKeyRangeCache` lifecycle.
//!
//! These tests exercise the full driver pipeline (`CosmosDriver::resolve_all_partition_key_ranges`
//! / `resolve_partition_key_ranges_for_key`) against the in-memory emulator, using
//! [`RequestObserver`] to count real `/pkranges` requests. They cover cache lifecycle
//! behavior that unit tests with mocked fetch closures cannot: real HTTP-layer ETag
//! round trips, real single-flight coalescing under genuine task concurrency, real
//! split/merge topology changes via the emulator's control plane, and isolation across
//! multiple containers sharing one driver instance.

use std::future::{poll_fn, Future};
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};
use std::time::Duration;

use azure_core::http::{Method, Request, Url};
use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, ContainerConfig, Epk, InMemoryEmulatorHttpClient, RequestGate,
    RequestObserver, VirtualAccountConfig, VirtualRegion,
};
use azure_data_cosmos_driver::models::{AccountReference, FeedRange, PartitionKey};
use azure_data_cosmos_driver::options::DriverOptions;
use azure_data_cosmos_driver::CosmosDriver;

const GATEWAY_URL: &str = "https://eastus.emulator.local";
const DATABASE_NAME: &str = "pkcache-db";

fn account() -> AccountReference {
    AccountReference::with_master_key(Url::parse(GATEWAY_URL).unwrap(), "ZW11bGF0b3Ita2V5")
}

/// Number of real `/pkranges` GET requests a single converged cold fetch (or
/// forced refresh) issues against the in-memory emulator. The driver's
/// change-feed-style pagination (see `pk_range_page_fetcher` /
/// `fetch_pk_ranges_from_service`) always issues one request that returns the
/// page(s) of ranges plus an ETag, followed by exactly one more
/// `If-None-Match` request that comes back `304 Not Modified` to confirm the
/// feed is fully drained — even when the whole topology fits on a single
/// page. Two requests per fetch is therefore the correct baseline, not an
/// artifact under test; these tests assert against `REQUESTS_PER_FETCH`
/// rather than a bare `1` so the real invariant (coalescing / cache reuse)
/// stays intact regardless of how many requests one fetch happens to need.
const REQUESTS_PER_FETCH: usize = 2;

/// Counts requests whose path matches `/dbs/{db}/colls/{coll}/pkranges` — the
/// endpoint the `PartitionKeyRangeCache` hits on every cold fetch or refresh.
#[derive(Debug, Default)]
struct PkRangesRequestCounter {
    count: AtomicUsize,
}

impl PkRangesRequestCounter {
    fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    fn count(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }
}

impl RequestObserver for PkRangesRequestCounter {
    fn on_request(&self, request: &Request) {
        if request.method() == Method::Get && request.url().path().ends_with("/pkranges") {
            self.count.fetch_add(1, Ordering::SeqCst);
        }
    }
}

#[derive(Debug)]
struct PkRangesRequestGate {
    open: AtomicBool,
    opened: tokio::sync::Notify,
}

impl PkRangesRequestGate {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            open: AtomicBool::new(false),
            opened: tokio::sync::Notify::new(),
        })
    }

    fn open(&self) {
        self.open.store(true, Ordering::Release);
        self.opened.notify_waiters();
    }
}

#[async_trait::async_trait]
impl RequestGate for PkRangesRequestGate {
    async fn wait(&self, request: &Request) {
        if request.method() != Method::Get || !request.url().path().ends_with("/pkranges") {
            return;
        }

        while !self.open.load(Ordering::Acquire) {
            let opened = self.opened.notified();
            if self.open.load(Ordering::Acquire) {
                return;
            }
            opened.await;
        }
    }
}

#[derive(Debug)]
struct FirstPollTracker {
    expected: usize,
    polled: AtomicUsize,
    all_polled: tokio::sync::Notify,
}

impl FirstPollTracker {
    fn new(expected: usize) -> Arc<Self> {
        Arc::new(Self {
            expected,
            polled: AtomicUsize::new(0),
            all_polled: tokio::sync::Notify::new(),
        })
    }

    fn record(&self) {
        if self.polled.fetch_add(1, Ordering::SeqCst) + 1 == self.expected {
            self.all_polled.notify_one();
        }
    }

    async fn wait_for_all(&self) {
        while self.polled.load(Ordering::SeqCst) < self.expected {
            let all_polled = self.all_polled.notified();
            if self.polled.load(Ordering::SeqCst) >= self.expected {
                return;
            }
            all_polled.await;
        }
    }
}

/// Builds an in-memory emulator with a single region, a pre-provisioned
/// database, and the supplied [`RequestObserver`] attached (if any).
fn build_emulator(observer: Option<Arc<dyn RequestObserver>>) -> Arc<InMemoryEmulatorHttpClient> {
    build_emulator_with_gate(observer, None)
}

fn build_emulator_with_gate(
    observer: Option<Arc<dyn RequestObserver>>,
    gate: Option<Arc<dyn RequestGate>>,
) -> Arc<InMemoryEmulatorHttpClient> {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);

    let emulator = InMemoryEmulatorHttpClient::new(config);
    let emulator = match observer {
        Some(observer) => emulator.with_request_observer(observer),
        None => emulator,
    };
    let emulator = match gate {
        Some(gate) => emulator.with_request_gate(gate),
        None => emulator,
    };
    Arc::new(emulator)
}

async fn create_driver(emulator: &Arc<InMemoryEmulatorHttpClient>) -> Arc<CosmosDriver> {
    let runtime = emulator
        .runtime_builder()
        .build()
        .await
        .expect("runtime should build");
    runtime
        .create_driver(DriverOptions::builder(account()).build())
        .await
        .expect("driver should initialize against the in-memory emulator")
}

fn hash_pk_def() -> serde_json::Value {
    serde_json::json!({"paths": ["/pk"], "kind": "Hash", "version": 2})
}

fn multi_hash_pk_def() -> serde_json::Value {
    serde_json::json!({"paths": ["/tenant", "/user"], "kind": "MultiHash", "version": 2})
}

// =============================================================================
// Concurrent cold requests share one /pkranges fetch (single-flight coalescing)
// =============================================================================

#[tokio::test]
async fn concurrent_cold_requests_share_one_pkranges_fetch() {
    const CONCURRENT_CALLERS: usize = 8;
    let counter = PkRangesRequestCounter::new();
    let gate = PkRangesRequestGate::new();
    let emulator = build_emulator_with_gate(Some(counter.clone()), Some(gate.clone()));
    emulator.store().create_database(DATABASE_NAME);
    emulator.store().create_container(
        DATABASE_NAME,
        "coll",
        serde_json::from_value(hash_pk_def()).unwrap(),
    );
    let driver = create_driver(&emulator).await;
    let container = driver
        .resolve_container(DATABASE_NAME, "coll", Default::default())
        .await
        .expect("container resolves");

    // Hold /pkranges requests while every lookup future is polled once. This
    // proves the callers overlap inside the cache instead of merely reusing a
    // value that an earlier task already warmed.
    let first_polls = FirstPollTracker::new(CONCURRENT_CALLERS);
    let mut handles = Vec::with_capacity(CONCURRENT_CALLERS);
    for _ in 0..CONCURRENT_CALLERS {
        let driver = driver.clone();
        let container = container.clone();
        let first_polls = first_polls.clone();
        handles.push(tokio::spawn(async move {
            let lookup = driver.resolve_all_partition_key_ranges(&container, false);
            tokio::pin!(lookup);
            let mut first_poll = true;
            poll_fn(|cx| {
                let result = lookup.as_mut().poll(cx);
                if first_poll {
                    first_poll = false;
                    first_polls.record();
                }
                result
            })
            .await
            .expect("cold fetch succeeds")
            .expect("cold fetch returns topology")
        }));
    }

    tokio::time::timeout(Duration::from_secs(5), first_polls.wait_for_all())
        .await
        .expect("all concurrent lookup futures should be polled");
    assert_eq!(
        counter.count(),
        1,
        "all pending cold callers must share one in-flight /pkranges request"
    );
    gate.open();

    let mut results = Vec::with_capacity(CONCURRENT_CALLERS);
    for handle in handles {
        results.push(handle.await.expect("task should not panic"));
    }

    // Every caller must observe the same converged topology.
    let first_len = results[0].len();
    assert!(results.iter().all(|r| r.len() == first_len));
    assert_eq!(
        counter.count(),
        REQUESTS_PER_FETCH,
        "8 concurrent cold callers must coalesce onto exactly one /pkranges fetch \
         (one page request + one confirming 304, not one pair per caller)"
    );
}

// =============================================================================
// Invalidation / refetch
// =============================================================================

#[tokio::test]
async fn force_refresh_refetches_and_invalidation_is_observable() {
    let counter = PkRangesRequestCounter::new();
    let emulator = build_emulator(Some(counter.clone()));
    let store = emulator.store();
    store.create_database(DATABASE_NAME);
    store.create_container(
        DATABASE_NAME,
        "coll",
        serde_json::from_value(hash_pk_def()).unwrap(),
    );
    let driver = create_driver(&emulator).await;
    let container = driver
        .resolve_container(DATABASE_NAME, "coll", Default::default())
        .await
        .expect("container resolves");

    // Cold fetch populates the cache.
    let initial = driver
        .resolve_all_partition_key_ranges(&container, false)
        .await
        .expect("cold fetch succeeds")
        .expect("cold fetch returns topology");
    assert_eq!(initial.len(), 4); // default partition count
    let after_cold = counter.count();
    assert_eq!(after_cold, REQUESTS_PER_FETCH);

    // A second non-forced lookup must hit the warm cache — no new request.
    driver
        .resolve_all_partition_key_ranges(&container, false)
        .await
        .expect("warm lookup succeeds")
        .expect("warm lookup returns topology");
    assert_eq!(
        counter.count(),
        after_cold,
        "a non-forced lookup must reuse the cached routing map"
    );

    // A forced refresh must always refetch, even with no topology change.
    driver
        .resolve_all_partition_key_ranges(&container, true)
        .await
        .expect("forced refresh succeeds")
        .expect("forced refresh returns topology");
    assert!(
        counter.count() > after_cold,
        "force_refresh=true must invalidate the cache and refetch from the service"
    );
}

// =============================================================================
// ETag continuation / incremental replacement
// =============================================================================

#[tokio::test]
async fn repeated_force_refresh_round_trips_etag_without_corrupting_the_map() {
    let emulator = build_emulator(None);
    let store = emulator.store();
    store.create_database(DATABASE_NAME);
    store.create_container(
        DATABASE_NAME,
        "coll",
        serde_json::from_value(hash_pk_def()).unwrap(),
    );
    let driver = create_driver(&emulator).await;
    let container = driver
        .resolve_container(DATABASE_NAME, "coll", Default::default())
        .await
        .expect("container resolves");

    let initial = driver
        .resolve_all_partition_key_ranges(&container, false)
        .await
        .expect("cold fetch succeeds")
        .expect("cold fetch returns topology");

    // Repeated forced refreshes with no topology change round-trip through
    // the real `If-None-Match` / 304 path (see `handle_read_pkranges`): the
    // service returns `NotModified`, and the driver must preserve the
    // existing map rather than caching an empty/incomplete one.
    for _ in 0..3 {
        let refreshed = driver
            .resolve_all_partition_key_ranges(&container, true)
            .await
            .expect("refresh succeeds")
            .expect("refresh must not degrade to an empty/missing map on 304 Not Modified");
        assert_eq!(refreshed.len(), initial.len());
        let mut refreshed_ids: Vec<_> = refreshed.iter().map(|r| r.id.to_string()).collect();
        let mut initial_ids: Vec<_> = initial.iter().map(|r| r.id.to_string()).collect();
        refreshed_ids.sort();
        initial_ids.sort();
        assert_eq!(refreshed_ids, initial_ids);
    }
}

#[tokio::test]
async fn split_produces_incremental_replacement_on_next_force_refresh() {
    let emulator = build_emulator(None);
    let store = emulator.store();
    store.create_database(DATABASE_NAME);
    store.create_container(
        DATABASE_NAME,
        "coll",
        serde_json::from_value(hash_pk_def()).unwrap(),
    );
    let driver = create_driver(&emulator).await;
    let container = driver
        .resolve_container(DATABASE_NAME, "coll", Default::default())
        .await
        .expect("container resolves");

    let initial = driver
        .resolve_all_partition_key_ranges(&container, false)
        .await
        .expect("cold fetch succeeds")
        .expect("cold fetch returns topology");
    assert_eq!(initial.len(), 4);
    let initial_ids: std::collections::HashSet<_> =
        initial.iter().map(|r| r.id.to_string()).collect();

    // Split partition 0 with zero min_lock_duration, then drain the control
    // plane so the split is fully applied before we look again.
    store.split_partition(DATABASE_NAME, "coll", 0, Duration::ZERO);
    store.drain_pending_control_plane().await;

    let after_split = driver
        .resolve_all_partition_key_ranges(&container, true)
        .await
        .expect("post-split refresh succeeds")
        .expect("post-split refresh converges on the new topology");

    // One parent replaced by two children: net +1 range.
    assert_eq!(after_split.len(), 5);
    let after_split_ids: std::collections::HashSet<_> =
        after_split.iter().map(|r| r.id.to_string()).collect();
    assert!(
        !after_split_ids.is_subset(&initial_ids),
        "post-split topology must contain at least one new child range id"
    );

    // Point lookups against the new topology must resolve to exactly one of
    // the surviving/child ranges (never the retired parent, never zero).
    let pk = PartitionKey::from("some-partition-value");
    let owning = driver
        .resolve_partition_key_ranges_for_key(&container, &pk, false)
        .await
        .expect("point lookup succeeds")
        .expect("point lookup returns an owning range");
    assert_eq!(owning.len(), 1);
    assert!(after_split_ids.contains(&owning[0].id));
}

// =============================================================================
// Multi-container isolation
// =============================================================================

#[tokio::test]
async fn multi_container_caches_do_not_leak_between_containers() {
    let counter = PkRangesRequestCounter::new();
    let emulator = build_emulator(Some(counter.clone()));
    let store = emulator.store();
    store.create_database(DATABASE_NAME);
    store.create_container_with_config(
        DATABASE_NAME,
        "coll-a",
        serde_json::from_value(hash_pk_def()).unwrap(),
        ContainerConfig::new()
            .with_partition_count(3)
            .build()
            .unwrap(),
    );
    store.create_container_with_config(
        DATABASE_NAME,
        "coll-b",
        serde_json::from_value(hash_pk_def()).unwrap(),
        ContainerConfig::new()
            .with_partition_count(6)
            .build()
            .unwrap(),
    );
    let driver = create_driver(&emulator).await;
    let container_a = driver
        .resolve_container(DATABASE_NAME, "coll-a", Default::default())
        .await
        .expect("container-a resolves");
    let container_b = driver
        .resolve_container(DATABASE_NAME, "coll-b", Default::default())
        .await
        .expect("container-b resolves");

    let ranges_a = driver
        .resolve_all_partition_key_ranges(&container_a, false)
        .await
        .expect("container-a fetch succeeds")
        .expect("container-a fetch returns topology");
    let ranges_b = driver
        .resolve_all_partition_key_ranges(&container_b, false)
        .await
        .expect("container-b fetch succeeds")
        .expect("container-b fetch returns topology");

    assert_eq!(ranges_a.len(), 3, "container A's cache entry is isolated");
    assert_eq!(ranges_b.len(), 6, "container B's cache entry is isolated");
    assert_eq!(
        counter.count(),
        2 * REQUESTS_PER_FETCH,
        "one cold /pkranges fetch per distinct container"
    );

    // Warm lookups for both containers must stay warm independently.
    driver
        .resolve_all_partition_key_ranges(&container_a, false)
        .await
        .unwrap();
    driver
        .resolve_all_partition_key_ranges(&container_b, false)
        .await
        .unwrap();
    assert_eq!(
        counter.count(),
        2 * REQUESTS_PER_FETCH,
        "warm lookups on either container must not trigger new fetches"
    );

    // Force-refreshing one container's cache must not disturb the other's.
    driver
        .resolve_all_partition_key_ranges(&container_a, true)
        .await
        .unwrap();
    assert_eq!(
        counter.count(),
        2 * REQUESTS_PER_FETCH + 1,
        "force-refreshing container A must issue only its incremental 304 request, \
         without refetching container B"
    );
}

// =============================================================================
// resolve_all_partition_key_ranges / resolve_partition_key_ranges_for_key —
// full key, MultiHash prefix, force-refresh, empty-map failure
// (Goal 3: driver-level coverage that cannot be exercised without a real
// fetch pipeline, since `pk_range_page_fetcher` is private.)
// =============================================================================

#[tokio::test]
async fn resolve_partition_key_ranges_for_key_full_key_returns_single_owning_range() {
    let emulator = build_emulator(None);
    let store = emulator.store();
    store.create_database(DATABASE_NAME);
    store.create_container(
        DATABASE_NAME,
        "coll",
        serde_json::from_value(hash_pk_def()).unwrap(),
    );
    let driver = create_driver(&emulator).await;
    let container = driver
        .resolve_container(DATABASE_NAME, "coll", Default::default())
        .await
        .expect("container resolves");

    let pk = PartitionKey::from("customer-42");
    let ranges = driver
        .resolve_partition_key_ranges_for_key(&container, &pk, false)
        .await
        .expect("full-key lookup succeeds")
        .expect("full-key lookup returns an owning range");

    // A full (non-prefix) key is a point lookup: exactly one owning range.
    assert_eq!(ranges.len(), 1);
}

#[tokio::test]
async fn resolve_partition_key_ranges_for_key_multihash_prefix_returns_multiple_ranges() {
    let emulator = build_emulator(None);
    let store = emulator.store();
    store.create_database(DATABASE_NAME);
    store.create_container_with_config(
        DATABASE_NAME,
        "coll",
        serde_json::from_value(multi_hash_pk_def()).unwrap(),
        ContainerConfig::new()
            .with_partition_count(8)
            .build()
            .unwrap(),
    );
    let driver = create_driver(&emulator).await;
    let container = driver
        .resolve_container(DATABASE_NAME, "coll", Default::default())
        .await
        .expect("container resolves");

    // Split the physical owner inside this prefix's EPK interval so the prefix
    // deterministically spans both children. A point-lookup implementation
    // would incorrectly return only one of them.
    let prefix_pk = PartitionKey::from("tenant-1");
    let prefix = FeedRange::for_partition(prefix_pk.clone(), container.partition_key_definition());
    let before = driver
        .resolve_partition_key_ranges_for_key(&container, &prefix_pk, false)
        .await
        .expect("prefix lookup succeeds")
        .expect("prefix lookup returns its initial owner");
    assert_eq!(before.len(), 1);
    let owner_id = before[0].id.parse::<u32>().expect("range id is numeric");
    let split_epk = Epk::from(format!("{}80", prefix.min_inclusive().to_hex()));
    assert!(split_epk > *prefix.min_inclusive());
    assert!(split_epk < *prefix.max_exclusive());
    store.split_partition_at_epk(DATABASE_NAME, "coll", owner_id, split_epk, Duration::ZERO);
    store.drain_pending_control_plane().await;

    let ranges = driver
        .resolve_partition_key_ranges_for_key(&container, &prefix_pk, true)
        .await
        .expect("refreshed prefix lookup succeeds")
        .expect("refreshed prefix lookup returns overlapping children");
    assert_eq!(
        ranges.len(),
        2,
        "a prefix split within its EPK interval must resolve to both child ranges"
    );

    // A full (both-component) key against the same container must still
    // resolve through the point-lookup path to exactly one range.
    let full_pk = PartitionKey::from(("tenant-1", "user-7"));
    let full_ranges = driver
        .resolve_partition_key_ranges_for_key(&container, &full_pk, false)
        .await
        .expect("full-key lookup succeeds")
        .expect("full-key lookup returns an owning range");
    assert_eq!(full_ranges.len(), 1);
}

#[tokio::test]
async fn resolve_partition_key_ranges_for_key_force_refresh_reflects_split() {
    let emulator = build_emulator(None);
    let store = emulator.store();
    store.create_database(DATABASE_NAME);
    store.create_container(
        DATABASE_NAME,
        "coll",
        serde_json::from_value(hash_pk_def()).unwrap(),
    );
    let driver = create_driver(&emulator).await;
    let container = driver
        .resolve_container(DATABASE_NAME, "coll", Default::default())
        .await
        .expect("container resolves");

    let pk = PartitionKey::from("stable-key");
    let before = driver
        .resolve_partition_key_ranges_for_key(&container, &pk, false)
        .await
        .expect("lookup succeeds")
        .expect("lookup returns an owning range");
    assert_eq!(before.len(), 1);
    let before_id = before[0].id.to_string();

    let split_partition_id = before_id.parse::<u32>().expect("range id is numeric");
    store.split_partition(DATABASE_NAME, "coll", split_partition_id, Duration::ZERO);
    store.drain_pending_control_plane().await;

    // Without force_refresh, the stale cached map still returns the retired
    // parent that owned the key before the split.
    let without_refresh = driver
        .resolve_partition_key_ranges_for_key(&container, &pk, false)
        .await
        .expect("lookup succeeds")
        .expect("lookup still returns exactly one range from whichever map is cached");
    assert_eq!(without_refresh.len(), 1);
    assert_eq!(
        without_refresh[0].id, before_id,
        "without force_refresh the cached parent remains visible"
    );

    // With force_refresh, the cache must reflect the post-split topology.
    let with_refresh = driver
        .resolve_partition_key_ranges_for_key(&container, &pk, true)
        .await
        .expect("forced lookup succeeds")
        .expect("forced lookup returns an owning range from the converged post-split map");
    assert_eq!(with_refresh.len(), 1);
    assert_ne!(
        with_refresh[0].id, before_id,
        "force_refresh must replace the retired parent with the owning child"
    );
}

#[tokio::test]
async fn resolve_all_partition_key_ranges_returns_none_on_persistent_fetch_failure() {
    // Resolve a real container, then delete it out from under the cached
    // reference (without recreating it, unlike the name-cache-refresh
    // scenarios in `container_recreation.rs`). Every subsequent `/pkranges`
    // request now hits a genuinely nonexistent container and fails with a
    // plain 404 that the driver cannot recover from. This exercises the
    // fail-open contract of `resolve_all_partition_key_ranges`: a persistent
    // fetch failure must be reported as `Ok(None)`, never as a
    // successful-but-empty `Vec` and never as a propagated error.
    let emulator = build_emulator(None);
    let store = emulator.store();
    store.create_database(DATABASE_NAME);
    store.create_container(
        DATABASE_NAME,
        "flaky-coll",
        serde_json::from_value(hash_pk_def()).unwrap(),
    );
    let driver = create_driver(&emulator).await;
    let container = driver
        .resolve_container(DATABASE_NAME, "flaky-coll", Default::default())
        .await
        .expect("container resolves");

    let delete_url = Url::parse(&format!(
        "{GATEWAY_URL}/dbs/{DATABASE_NAME}/colls/flaky-coll"
    ))
    .unwrap();
    let response = emulator
        .execute_request(&Request::new(delete_url, Method::Delete))
        .await
        .unwrap();
    assert_eq!(
        response.status(),
        azure_core::http::StatusCode::NoContent,
        "container must actually be deleted for the fetch to fail"
    );

    let result = driver
        .resolve_all_partition_key_ranges(&container, false)
        .await
        .expect("a fetch failure must not surface as a driver error");

    assert!(
        result.is_none(),
        "an unresolvable container must yield Ok(None), never an empty-but-successful Vec"
    );
}
