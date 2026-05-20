// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Unified lock-free location state store.

use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Weak,
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use crossbeam_epoch::{self as epoch, Atomic, Owned};
use futures::future::BoxFuture;

#[cfg(feature = "tokio")]
use crate::driver::transport::background_task_manager::BackgroundTaskManager;
use crate::{
    driver::cache::{AccountMetadataCache, AccountProperties},
    models::AccountEndpoint,
    options::Region,
};

use super::{
    build_account_endpoint_state, expire_partition_overrides, expire_unavailable_endpoints,
    mark_endpoint_unavailable, mark_partition_unavailable,
    partition_endpoint_state::{PartitionEndpointState, PartitionFailoverConfig},
    partition_key_range_id::PartitionKeyRangeId,
    record_hedge_alternate_win, record_hedge_primary_win, AccountEndpointState, CosmosEndpoint,
    LocationEffect,
};

/// Immutable location snapshot consumed by one operation-loop iteration.
#[derive(Clone, Debug)]
pub(crate) struct LocationSnapshot {
    pub account: Arc<AccountEndpointState>,
    pub partitions: Arc<PartitionEndpointState>,
}

#[cfg(test)]
impl LocationSnapshot {
    pub(crate) fn for_tests(account: Arc<AccountEndpointState>) -> Self {
        Self {
            account,
            partitions: Arc::new(PartitionEndpointState::default()),
        }
    }

    pub(crate) fn for_tests_with_partitions(
        account: Arc<AccountEndpointState>,
        partitions: Arc<PartitionEndpointState>,
    ) -> Self {
        Self {
            account,
            partitions,
        }
    }
}

type AccountRefreshFn = Arc<
    dyn Fn(
            Option<Arc<AccountProperties>>,
        ) -> BoxFuture<'static, azure_core::Result<AccountProperties>>
        + Send
        + Sync,
>;

/// Interval between iterations of the background account-metadata refresh
/// loop. Independent of `LocationStateStore::refresh_interval` (which
/// rate-limits the event-driven refresh emitted by
/// `LocationEffect::RefreshAccountProperties`).
#[cfg(feature = "tokio")]
pub(crate) const BACKGROUND_REFRESH_INTERVAL: Duration = Duration::from_secs(300);

/// Unified location state store with lock-free reads and CAS-loop writes.
pub(crate) struct LocationStateStore {
    account: Atomic<AccountEndpointState>,
    partitions: Atomic<PartitionEndpointState>,
    account_metadata_cache: Arc<AccountMetadataCache>,
    account_endpoint: AccountEndpoint,
    account_refresh_fn: AccountRefreshFn,
    default_endpoint: CosmosEndpoint,
    preferred_regions: Vec<Region>,
    gateway20_enabled: bool,
    endpoint_unavailability_ttl: Duration,
    refresh_interval: Duration,
    last_refresh_epoch_ms: AtomicU64,
    /// The etag of the last `AccountProperties` that was synced.
    /// Used to skip the CAS loop when the account metadata hasn't changed.
    last_synced_etag: std::sync::Mutex<String>,
    /// Pointer identity of the last synced `AccountProperties` arc.
    /// When `sync_account_properties` is called with the same `Arc` (i.e.,
    /// the account metadata cache returned the same cached value), the entire
    /// sync is skipped without acquiring any other locks or rebuilding endpoint
    /// lists.
    last_synced_properties: std::sync::Mutex<Option<Arc<AccountProperties>>>,
    /// Monotonic version counter bumped on every successful CAS write.
    account_version: AtomicU64,
    /// Cached snapshot: (version, snapshot). When the version matches
    /// `account_version`, `snapshot()` returns `Arc::clone()` of the cached
    /// arcs (refcount increment only) instead of a full clone.
    cached_snapshot: std::sync::Mutex<(u64, LocationSnapshot)>,
    /// Manages the background failback loop task.
    /// Dropping this manager aborts the failback task.
    #[cfg(feature = "tokio")]
    background_task_manager: BackgroundTaskManager,
}

impl std::fmt::Debug for LocationStateStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocationStateStore")
            .field("account_endpoint", &self.account_endpoint)
            .field("default_endpoint", &self.default_endpoint)
            .field(
                "endpoint_unavailability_ttl",
                &self.endpoint_unavailability_ttl,
            )
            .field("refresh_interval", &self.refresh_interval)
            .finish_non_exhaustive()
    }
}

impl Drop for LocationStateStore {
    fn drop(&mut self) {
        // crossbeam_epoch::Atomic<T> intentionally has no destructor.
        // Detach each field and convert to Owned for immediate deallocation.
        // Safety: &mut self in Drop guarantees exclusive access.
        let account = std::mem::replace(&mut self.account, Atomic::null());
        let partitions = std::mem::replace(&mut self.partitions, Atomic::null());

        unsafe {
            if let Some(account) = account.try_into_owned() {
                drop(account);
            }

            if let Some(partitions) = partitions.try_into_owned() {
                drop(partitions);
            }
        }
    }
}

impl LocationStateStore {
    /// Creates a new location store with a single-endpoint account snapshot.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        account_metadata_cache: Arc<AccountMetadataCache>,
        account_endpoint: AccountEndpoint,
        default_endpoint: CosmosEndpoint,
        account_refresh_fn: AccountRefreshFn,
        gateway20_enabled: bool,
        endpoint_unavailability_ttl: Duration,
        partition_failover_config: PartitionFailoverConfig,
        preferred_regions: Vec<Region>,
    ) -> Self {
        let account_state = AccountEndpointState::single(default_endpoint.clone());
        let partition_state = PartitionEndpointState::new(partition_failover_config);

        let initial_snapshot = LocationSnapshot {
            account: Arc::new(account_state.clone()),
            partitions: Arc::new(partition_state.clone()),
        };

        Self {
            account: Atomic::new(account_state),
            partitions: Atomic::new(partition_state),
            account_metadata_cache,
            account_endpoint,
            account_refresh_fn,
            default_endpoint,
            preferred_regions,
            gateway20_enabled,
            endpoint_unavailability_ttl,
            // Rate limit for event-driven refreshes emitted by
            // `LocationEffect::RefreshAccountProperties` (e.g. retry policies
            // hitting WriteForbidden). Kept at 5 s — urgent recovery cadence,
            // independent of the periodic background loop driven by
            // `BACKGROUND_REFRESH_INTERVAL`.
            refresh_interval: Duration::from_secs(5),
            last_refresh_epoch_ms: AtomicU64::new(0),
            last_synced_etag: std::sync::Mutex::new(String::new()),
            last_synced_properties: std::sync::Mutex::new(None),
            account_version: AtomicU64::new(0),
            cached_snapshot: std::sync::Mutex::new((0, initial_snapshot)),
            #[cfg(feature = "tokio")]
            background_task_manager: BackgroundTaskManager::new(),
        }
    }

    /// Returns the default endpoint.
    pub fn default_endpoint(&self) -> &CosmosEndpoint {
        &self.default_endpoint
    }

    /// Returns a snapshot of account and partition state.
    ///
    /// Uses a fast path when the version hasn't changed since the last
    /// snapshot: the cached `Arc` is cloned without touching the
    /// epoch-protected pointers. On a version mismatch (slow path),
    /// the current state is loaded under an epoch guard, cloned into
    /// fresh `Arc`s, and cached for subsequent callers.
    pub fn snapshot(&self) -> LocationSnapshot {
        let current_version = self.account_version.load(Ordering::Acquire);

        {
            let cached = self.cached_snapshot.lock().unwrap();
            if cached.0 == current_version {
                return cached.1.clone();
            }
        }

        let guard = epoch::pin();

        let account = {
            // SAFETY: pointer comes from `Atomic` and stays valid while guard is pinned.
            let current = unsafe { self.account.load(Ordering::Acquire, &guard).deref() };
            Arc::new(current.clone())
        };

        let partitions = {
            // SAFETY: pointer comes from `Atomic` and stays valid while guard is pinned.
            let current = unsafe { self.partitions.load(Ordering::Acquire, &guard).deref() };
            Arc::new(current.clone())
        };

        let snapshot = LocationSnapshot {
            account,
            partitions,
        };

        let mut cached = self.cached_snapshot.lock().unwrap();
        if cached.0 < current_version {
            *cached = (current_version, snapshot.clone());
        }

        snapshot
    }

    /// Returns the configured endpoint unavailability TTL.
    pub fn endpoint_unavailability_ttl(&self) -> Duration {
        self.endpoint_unavailability_ttl
    }

    /// Returns the latest account snapshot.
    #[allow(dead_code)]
    pub fn account_snapshot(&self) -> Arc<AccountEndpointState> {
        let guard = epoch::pin();
        // SAFETY: pointer comes from `Atomic` and stays valid while guard is pinned.
        let current = unsafe { self.account.load(Ordering::Acquire, &guard).deref() };
        Arc::new(current.clone())
    }

    /// Applies location effects (endpoint unavailability and account refresh).
    pub async fn apply(&self, effects: &[LocationEffect]) {
        for effect in effects {
            match effect {
                LocationEffect::MarkEndpointUnavailable { endpoint, reason } => {
                    let endpoint = endpoint.clone();
                    let reason = reason.clone();
                    self.apply_account(|current| {
                        mark_endpoint_unavailable(current, &endpoint, reason.clone())
                    });
                }
                LocationEffect::MarkPartitionUnavailable(partition) => {
                    if partition.partition_key_range_id.is_none() {
                        // No partition key range ID available (first attempt);
                        // skip partition-level marking.
                        continue;
                    }
                    let is_partitioned = partition.is_partitioned_resource;
                    self.apply_partition(|current_partitions| {
                        let account = self.account_snapshot();
                        mark_partition_unavailable(
                            current_partitions,
                            &account,
                            partition,
                            is_partitioned,
                        )
                    });
                }
                LocationEffect::RefreshAccountProperties => {
                    self.refresh_account_properties_if_due().await;
                }
            }
        }
    }

    fn apply_account(&self, mut f: impl FnMut(&AccountEndpointState) -> AccountEndpointState) {
        let guard = epoch::pin();

        loop {
            let current = self.account.load(Ordering::Acquire, &guard);
            // SAFETY: pointer comes from `Atomic` and stays valid while guard is pinned.
            let current_ref = unsafe { current.deref() };
            let next_state = f(current_ref);

            match self.account.compare_exchange(
                current,
                Owned::new(next_state),
                Ordering::AcqRel,
                Ordering::Acquire,
                &guard,
            ) {
                Ok(_) => {
                    // `current` is the old value that was just replaced.
                    unsafe { guard.defer_destroy(current) };
                    self.account_version.fetch_add(1, Ordering::Release);
                    return;
                }
                Err(_) => continue,
            }
        }
    }

    /// CAS loop on partition-level state.
    pub(crate) fn apply_partition(
        &self,
        mut f: impl FnMut(&PartitionEndpointState) -> PartitionEndpointState,
    ) {
        let guard = epoch::pin();

        loop {
            let current = self.partitions.load(Ordering::Acquire, &guard);
            // SAFETY: pointer comes from `Atomic` and stays valid while guard is pinned.
            let current_ref = unsafe { current.deref() };
            let next_state = f(current_ref);

            match self.partitions.compare_exchange(
                current,
                Owned::new(next_state),
                Ordering::AcqRel,
                Ordering::Acquire,
                &guard,
            ) {
                Ok(_) => {
                    // current is the pointer that was just replaced and is no
                    // longer reachable from self.partitions. compare_exchange
                    // returns the newly-installed pointer on success, not the
                    // replaced one, so we must defer-destroy current (matching
                    // the pattern in apply_account above).
                    unsafe { guard.defer_destroy(current) };
                    self.account_version.fetch_add(1, Ordering::Release);
                    return;
                }
                Err(_) => continue,
            }
        }
    }

    async fn refresh_account_properties_if_due(&self) {
        let now_ms = epoch_millis();
        let refresh_after_ms = self.refresh_interval.as_millis() as u64;
        let last = self.last_refresh_epoch_ms.load(Ordering::Acquire);

        if now_ms.saturating_sub(last) < refresh_after_ms {
            return;
        }

        if self
            .last_refresh_epoch_ms
            .compare_exchange(last, now_ms, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return;
        }

        self.refresh_account_properties_inner().await;
    }

    /// Force-refresh account properties without consulting the
    /// `refresh_interval` rate limit. Intended for the periodic background
    /// timer loop (the timer interval IS the rate limit). The event-driven
    /// path from retry policies must continue to use
    /// [`refresh_account_properties_if_due`] to throttle bursts.
    ///
    /// The `last_refresh_epoch_ms` clock is updated by
    /// [`refresh_account_properties_inner`] only on a successful fetch — if
    /// this timer-driven refresh fails (network error, service 5xx, …), the
    /// event-driven path is NOT throttled and is free to retry recovery
    /// immediately.
    async fn force_refresh_account_properties(&self) {
        self.refresh_account_properties_inner().await;
    }

    /// Shared implementation of both `refresh_account_properties_if_due`
    /// (rate-limited, event-driven) and `force_refresh_account_properties`
    /// (timer-driven).
    ///
    /// Performs the fallible network fetch first (outside any cache lock),
    /// then atomically replaces the cached value via
    /// [`AccountMetadataCache::replace`] / `get_or_refresh_with`. This avoids
    /// the race where a concurrent `execute_operation` thread could hit the
    /// cache during the gap between an `invalidate` and a follow-up
    /// `get_or_fetch` — a bug pattern present in earlier revisions of this
    /// code. On success the routing snapshot is CAS-updated and the
    /// rate-limit clock advances; on failure the previous snapshot and
    /// rate-limit timestamp are left intact so the event-driven path can
    /// retry recovery immediately.
    async fn refresh_account_properties_inner(&self) {
        // Capture the previous properties so the refresh callback can use
        // them for regional fallback if the primary endpoint fails. We
        // intentionally do NOT invalidate the cache here — concurrent
        // `execute_operation` callers should keep getting the prior value
        // until the new one is ready, rather than seeing a hole and racing
        // to refill it themselves.
        let previous_props = self
            .account_metadata_cache
            .get(&self.account_endpoint)
            .await;

        let refresh_fn = Arc::clone(&self.account_refresh_fn);
        let fetched = (refresh_fn)(previous_props).await;

        let new_properties = match fetched {
            Ok(props) => props,
            Err(e) => {
                tracing::warn!(
                    endpoint = %self.account_endpoint,
                    error = %e,
                    "LocationStateStore: account metadata refresh failed; routing snapshot not updated",
                );
                return;
            }
        };

        // Atomically replace the cache entry under the cache's internal
        // single-pending-I/O lock. `_existing` is provided for the predicate
        // but we always replace because we just produced fresh data.
        let cached_arc = self
            .account_metadata_cache
            .get_or_refresh_with(
                self.account_endpoint.clone(),
                |_existing| true,
                || async { new_properties },
            )
            .await;

        // Only advance the rate-limit clock once a fresh value is in the
        // cache — if the cache somehow returned None (logically impossible
        // here since the factory always produces a value) we don't want to
        // throttle the event-driven path either.
        let Some(properties) = cached_arc else {
            tracing::warn!(
                endpoint = %self.account_endpoint,
                "LocationStateStore: account metadata cache produced no value after refresh; routing snapshot not updated",
            );
            return;
        };

        self.last_refresh_epoch_ms
            .store(epoch_millis(), Ordering::Release);

        let default_endpoint = self.default_endpoint.clone();
        self.sync_account_properties(properties, &default_endpoint);
    }

    /// Updates account state from properties using a CAS loop that preserves
    /// existing `unavailable_endpoints` marks set by concurrent operations.
    ///
    /// Skips the CAS loop when the `AccountProperties` etag matches
    /// the last synced value (same server version, properties unchanged).
    pub fn sync_account_properties(
        &self,
        properties: Arc<AccountProperties>,
        default_endpoint: &CosmosEndpoint,
    ) {
        // Fast path: same Arc pointer means identical data — nothing changed.
        {
            let last = self.last_synced_properties.lock().unwrap();
            if last.as_ref().is_some_and(|p| Arc::ptr_eq(p, &properties)) {
                return;
            }
        }

        self.prune_expired_unavailable_endpoints();

        if !properties.etag.is_empty() {
            let last_etag = self.last_synced_etag.lock().unwrap();
            if *last_etag == properties.etag {
                // Etag matches: update the pointer so future calls hit the fast path.
                drop(last_etag);
                *self.last_synced_properties.lock().unwrap() = Some(properties);
                return;
            }
        }

        let default_endpoint = default_endpoint.clone();
        let ttl = self.endpoint_unavailability_ttl;
        self.apply_account(|current| {
            let mut next = build_account_endpoint_state(
                &properties,
                default_endpoint.clone(),
                Some(current.generation),
                self.gateway20_enabled,
                &self.preferred_regions,
            );
            // Carry forward unavailability marks from the current state,
            // filtering out entries that have expired past the configured TTL.
            let now = Instant::now();
            let mut unavailable = current.unavailable_endpoints.clone();
            unavailable.retain(|_, (marked_at, _)| now.saturating_duration_since(*marked_at) < ttl);
            next.unavailable_endpoints = unavailable;
            next
        });

        if !properties.etag.is_empty() {
            let mut last_etag = self.last_synced_etag.lock().unwrap();
            *last_etag = properties.etag.clone();
        }

        // Update partition-level PPAF/PPCB flags from account properties.
        let per_partition_automatic_failover_enabled =
            properties.enable_per_partition_failover_behavior;

        *self.last_synced_properties.lock().unwrap() = Some(properties);
        self.apply_partition(|current| {
            let mut next = current.clone();
            next.per_partition_automatic_failover_enabled =
                per_partition_automatic_failover_enabled;
            next.per_partition_circuit_breaker_enabled = per_partition_automatic_failover_enabled
                || current.config.circuit_breaker_option_enabled;
            next
        });
    }

    fn prune_expired_unavailable_endpoints(&self) {
        let now = Instant::now();
        let ttl = self.endpoint_unavailability_ttl;
        let snapshot = self.account_snapshot();

        let has_expired = snapshot
            .unavailable_endpoints
            .values()
            .any(|(marked_at, _)| now.saturating_duration_since(*marked_at) >= ttl);

        if !has_expired {
            return;
        }

        self.apply_account(|current| expire_unavailable_endpoints(current, now, ttl));
    }

    /// Starts the background failback loop that periodically sweeps expired
    /// partition override entries.
    ///
    /// The loop holds a `Weak` reference to `self` so it self-terminates when
    /// the store is dropped. The `BackgroundTaskManager` provides abort-on-drop
    /// as an additional safety layer.
    #[cfg(feature = "tokio")]
    pub fn start_failback_loop(self: &Arc<Self>) {
        let weak_store: Weak<LocationStateStore> = Arc::downgrade(self);
        let config = self.snapshot().partitions.config.clone();
        self.background_task_manager.spawn(async move {
            failback_loop(weak_store, config).await;
        });
    }

    /// Starts the background account-metadata refresh loop.
    ///
    /// Mirrors `start_failback_loop` (`Weak<Self>` for self-termination,
    /// `BackgroundTaskManager` for abort-on-drop). The loop sleeps for
    /// [`BACKGROUND_REFRESH_INTERVAL`], then unconditionally refreshes the
    /// database account metadata via `force_refresh_account_properties`
    /// (the timer interval IS the rate limit, so the event-driven
    /// `refresh_interval` check is bypassed).
    #[cfg(feature = "tokio")]
    pub fn start_account_refresh_loop(self: &Arc<Self>) {
        let weak_store: Weak<LocationStateStore> = Arc::downgrade(self);
        self.background_task_manager.spawn(async move {
            account_refresh_loop(weak_store, BACKGROUND_REFRESH_INTERVAL).await;
        });
    }

    /// Records that the alternate (secondary) hedge attempt completed before
    /// the primary in
    /// [`execute_hedged`](crate::driver::pipeline::operation_pipeline).
    ///
    /// Per [`docs/HEDGING_SPEC.md`] §9.5: increments a per-`(partition,
    /// primary_region)` counter atomically via [`Self::apply_partition`]. When
    /// the counter reaches
    /// [`PartitionFailoverConfig::consecutive_hedge_win_threshold`] the
    /// partition is tripped by installing an `Unhealthy` entry in
    /// `circuit_breaker_overrides` (same shape PPCB uses for hard failures),
    /// so subsequent requests route away from the degraded primary region.
    /// The trip is recovered by the existing PPCB failback sweep.
    ///
    /// `primary_region` is `None` for default-endpoint accounts whose snapshot
    /// does not carry a named region (matches the spec invariant that the
    /// counter key is `(partition, primary_region)`).
    pub fn record_consecutive_hedge_win(
        &self,
        partition: &PartitionKeyRangeId,
        primary_region: Option<&Region>,
    ) {
        tracing::debug!(
            partition = %partition,
            primary_region = ?primary_region.map(Region::as_str),
            "cosmos.hedge.recorded_alternate_win",
        );
        let account = self.account_snapshot();
        self.apply_partition(|current| {
            record_hedge_alternate_win(current, &account, partition, primary_region)
        });
    }

    /// Records that the primary attempt won in
    /// [`execute_hedged`](crate::driver::pipeline::operation_pipeline).
    ///
    /// Per [`docs/HEDGING_SPEC.md`] §9.5 invariant #2: clears the
    /// per-`(partition, primary_region)` consecutive-hedge-win counter
    /// atomically via [`Self::apply_partition`] so transient cross-region
    /// latency spikes do not accumulate into a trip over arbitrarily long
    /// timescales. Does not touch `circuit_breaker_overrides` — an existing
    /// trip recovers via the failback sweep, not via primary wins.
    pub fn record_primary_win(
        &self,
        partition: &PartitionKeyRangeId,
        primary_region: Option<&Region>,
    ) {
        tracing::debug!(
            partition = %partition,
            primary_region = ?primary_region.map(Region::as_str),
            "cosmos.hedge.recorded_primary_win",
        );
        self.apply_partition(|current| {
            record_hedge_primary_win(current, partition, primary_region)
        });
    }
}

/// Background account-metadata refresh loop. Periodically calls
/// `force_refresh_account_properties` on the store. Exits when the
/// `LocationStateStore` is dropped (`Weak::upgrade()` returns `None`).
///
/// Each successful iteration emits `tracing::debug!` so operators can confirm
/// the timer is alive without flooding logs (debug, not info, since this
/// fires every 5 minutes per driver in steady state).
#[cfg(feature = "tokio")]
async fn account_refresh_loop(weak_store: Weak<LocationStateStore>, interval: Duration) {
    loop {
        tokio::time::sleep(interval).await;

        let Some(store) = weak_store.upgrade() else {
            // LocationStateStore was dropped — exit the loop.
            break;
        };

        store.force_refresh_account_properties().await;
        tracing::debug!(
            endpoint = %store.account_endpoint,
            interval_secs = interval.as_secs(),
            "LocationStateStore: background account metadata refresh tick complete",
        );
    }
}

/// Background failback loop that periodically sweeps expired partition overrides.
///
/// Exits when the `LocationStateStore` is dropped (`Weak::upgrade()` returns `None`).
#[cfg(feature = "tokio")]
async fn failback_loop(weak_store: Weak<LocationStateStore>, config: PartitionFailoverConfig) {
    loop {
        tokio::time::sleep(config.failback_sweep_interval).await;

        let Some(store) = weak_store.upgrade() else {
            // LocationStateStore was dropped — exit the loop.
            break;
        };

        store.apply_partition(|current_partitions| {
            expire_partition_overrides(
                current_partitions,
                Instant::now(),
                config.partition_unavailability_duration,
            )
        });
    }
}

fn epoch_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use crate::{
        driver::routing::{CosmosEndpoint, LocationEffect, UnavailableReason},
        models::AccountEndpoint,
    };

    use super::*;

    fn test_endpoint() -> AccountEndpoint {
        AccountEndpoint::from(url::Url::parse("https://test.documents.azure.com:443/").unwrap())
    }

    fn test_refresh_payload() -> AccountProperties {
        serde_json::from_value(serde_json::json!({
            "_self": "",
            "id": "test",
            "_rid": "test.documents.azure.com",
            "_etag": "etag-1",
            "media": "//media/",
            "addresses": "//addresses/",
            "_dbs": "//dbs/",
            "writableLocations": [{ "name": "eastus", "databaseAccountEndpoint": "https://test-eastus.documents.azure.com:443/" }],
            "readableLocations": [{ "name": "eastus", "databaseAccountEndpoint": "https://test-eastus.documents.azure.com:443/" }],
            "enableMultipleWriteLocations": false,
            "userReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "userConsistencyPolicy": { "defaultConsistencyLevel": "Session" },
            "systemReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "readPolicy": { "primaryReadCoefficient": 1, "secondaryReadCoefficient": 1 },
            "queryEngineConfiguration": "{}"
        }))
        .unwrap()
    }

    #[tokio::test]
    async fn apply_marks_endpoint_unavailable() {
        let default_endpoint = CosmosEndpoint::global(test_endpoint().url().clone());
        let refresh = Arc::new(|_previous: Option<Arc<AccountProperties>>| {
            let payload = test_refresh_payload();
            let fut: BoxFuture<'static, azure_core::Result<AccountProperties>> =
                Box::pin(async move { Ok(payload) });
            fut
        });

        let store = LocationStateStore::new(
            Arc::new(AccountMetadataCache::new()),
            test_endpoint(),
            default_endpoint.clone(),
            refresh,
            false,
            Duration::from_secs(60),
            PartitionFailoverConfig::default(),
            Vec::new(),
        );

        store
            .apply(&[LocationEffect::MarkEndpointUnavailable {
                endpoint: default_endpoint.clone(),
                reason: UnavailableReason::TransportError,
            }])
            .await;

        let snapshot = store.snapshot();
        assert_eq!(snapshot.account.unavailable_endpoints.len(), 1);
    }

    #[tokio::test]
    async fn refresh_effect_is_rate_limited() {
        let default_endpoint = CosmosEndpoint::global(test_endpoint().url().clone());
        let refresh_calls = Arc::new(AtomicUsize::new(0));
        let refresh_calls_clone = Arc::clone(&refresh_calls);
        let refresh = Arc::new(move |_previous: Option<Arc<AccountProperties>>| {
            let refresh_calls = Arc::clone(&refresh_calls_clone);
            let payload = test_refresh_payload();
            let fut: BoxFuture<'static, azure_core::Result<AccountProperties>> =
                Box::pin(async move {
                    refresh_calls.fetch_add(1, Ordering::SeqCst);
                    Ok(payload)
                });
            fut
        });

        let store = LocationStateStore::new(
            Arc::new(AccountMetadataCache::new()),
            test_endpoint(),
            default_endpoint,
            refresh,
            false,
            Duration::from_secs(60),
            PartitionFailoverConfig::default(),
            Vec::new(),
        );

        store
            .apply(&[LocationEffect::RefreshAccountProperties])
            .await;
        store
            .apply(&[LocationEffect::RefreshAccountProperties])
            .await;

        // The second call should be throttled by refresh_interval.
        assert_eq!(refresh_calls.load(Ordering::SeqCst), 1);
    }

    /// Regression guard: a failed timer-driven refresh must NOT advance the
    /// rate-limit clock, so the event-driven path (`LocationEffect::RefreshAccountProperties`)
    /// can attempt recovery immediately rather than waiting out the
    /// 5-second `refresh_interval` window.
    #[tokio::test]
    async fn failed_refresh_does_not_throttle_event_driven_path() {
        let default_endpoint = CosmosEndpoint::global(test_endpoint().url().clone());
        let success_refreshes = Arc::new(AtomicUsize::new(0));
        let total_refreshes = Arc::new(AtomicUsize::new(0));
        let success_refreshes_clone = Arc::clone(&success_refreshes);
        let total_refreshes_clone = Arc::clone(&total_refreshes);
        // First call fails; subsequent calls succeed.
        let refresh = Arc::new(move |_previous: Option<Arc<AccountProperties>>| {
            let total = Arc::clone(&total_refreshes_clone);
            let success = Arc::clone(&success_refreshes_clone);
            let payload = test_refresh_payload();
            let fut: BoxFuture<'static, azure_core::Result<AccountProperties>> =
                Box::pin(async move {
                    let n = total.fetch_add(1, Ordering::SeqCst);
                    if n == 0 {
                        Err(azure_core::Error::with_message(
                            azure_core::error::ErrorKind::Other,
                            "simulated network failure",
                        ))
                    } else {
                        success.fetch_add(1, Ordering::SeqCst);
                        Ok(payload)
                    }
                });
            fut
        });

        let store = LocationStateStore::new(
            Arc::new(AccountMetadataCache::new()),
            test_endpoint(),
            default_endpoint,
            refresh,
            false,
            Duration::from_secs(60),
            PartitionFailoverConfig::default(),
            Vec::new(),
        );

        // First refresh: fails. Should NOT advance last_refresh_epoch_ms,
        // so the next event-driven refresh is free to retry immediately.
        store.force_refresh_account_properties().await;
        assert_eq!(total_refreshes.load(Ordering::SeqCst), 1);
        assert_eq!(success_refreshes.load(Ordering::SeqCst), 0);

        // Immediate event-driven refresh — must NOT be throttled by the
        // failed timer-driven attempt above.
        store
            .apply(&[LocationEffect::RefreshAccountProperties])
            .await;
        assert_eq!(
            total_refreshes.load(Ordering::SeqCst),
            2,
            "event-driven refresh was incorrectly throttled by a previously-failed timer-driven refresh"
        );
        assert_eq!(success_refreshes.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn sync_account_properties_prunes_expired_marks_even_when_etag_is_unchanged() {
        let default_endpoint = CosmosEndpoint::global(test_endpoint().url().clone());
        let refresh = Arc::new(|_previous: Option<Arc<AccountProperties>>| {
            let payload = test_refresh_payload();
            let fut: BoxFuture<'static, azure_core::Result<AccountProperties>> =
                Box::pin(async move { Ok(payload) });
            fut
        });

        let store = LocationStateStore::new(
            Arc::new(AccountMetadataCache::new()),
            test_endpoint(),
            default_endpoint.clone(),
            refresh,
            false,
            Duration::from_secs(60),
            PartitionFailoverConfig::default(),
            Vec::new(),
        );

        let properties = Arc::new(test_refresh_payload());
        store.sync_account_properties(Arc::clone(&properties), &default_endpoint);

        let expired_endpoint = CosmosEndpoint::regional(
            "eastus".into(),
            url::Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        store.apply_account(|current| {
            let mut next = current.clone();
            next.unavailable_endpoints.insert(
                expired_endpoint.url().clone(),
                (
                    Instant::now() - Duration::from_secs(120),
                    UnavailableReason::TransportError,
                ),
            );
            next
        });

        // Use a different Arc to force a re-sync (same data, different pointer).
        let properties2 = Arc::new(test_refresh_payload());
        store.sync_account_properties(properties2, &default_endpoint);

        let snapshot = store.snapshot();
        assert!(snapshot.account.unavailable_endpoints.is_empty());
    }

    #[test]
    fn apply_partition_keeps_installed_pointer_live_until_store_drop() {
        // Regression test for a use-after-free in `apply_partition`. Earlier
        // versions called `defer_destroy(installed)` instead of
        // `defer_destroy(replaced)` (because crossbeam_epoch's
        // `compare_exchange` returns the *newly installed* pointer in `Ok`,
        // not the replaced one). With that bug, the freshly installed
        // `PartitionEndpointState` is reclaimed once the epoch advances even
        // though it is still reachable through `self.partitions`.
        //
        // We detect this by stamping a `Weak` canary onto the new state via
        // `apply_partition`'s closure. If the new state is incorrectly
        // destroyed, the canary's only strong reference (held inside the
        // destroyed state) drops and `weak.upgrade()` returns `None`. With the
        // fix, the strong ref stays alive until the store itself is dropped.

        let default_endpoint = CosmosEndpoint::global(test_endpoint().url().clone());
        let refresh = Arc::new(|_previous: Option<Arc<AccountProperties>>| {
            let payload = test_refresh_payload();
            let fut: BoxFuture<'static, azure_core::Result<AccountProperties>> =
                Box::pin(async move { Ok(payload) });
            fut
        });

        let store = LocationStateStore::new(
            Arc::new(AccountMetadataCache::new()),
            test_endpoint(),
            default_endpoint,
            refresh,
            false,
            Duration::from_secs(60),
            PartitionFailoverConfig::default(),
            Vec::new(),
        );

        let canary = Arc::new(());
        let weak = Arc::downgrade(&canary);

        // Move the canary into the closure. `apply_partition` may call the
        // closure more than once under contention; in this single-threaded
        // test there is no contention, but we still write it defensively so
        // the closure is callable repeatedly.
        let canary_for_closure = canary.clone();
        store.apply_partition(move |current| {
            let mut next = current.clone();
            next.per_partition_automatic_failover_enabled =
                !current.per_partition_automatic_failover_enabled;
            next._test_canary = Some(canary_for_closure.clone());
            next
        });

        // Drop our outer strong reference. The only remaining strong ref must
        // now live inside the newly installed `PartitionEndpointState` held by
        // `self.partitions`.
        drop(canary);

        // Force epoch advancement on multiple participants so any incorrectly
        // deferred destroy would actually fire. Without this, the buggy free
        // can be observed only at process exit.
        let collector = epoch::default_collector();
        let helper_a = collector.register();
        let helper_b = collector.register();
        for _ in 0..1024 {
            let mut g = epoch::pin();
            g.flush();
            g.repin();
            drop(g);

            let mut ga = helper_a.pin();
            ga.flush();
            ga.repin();
            drop(ga);

            let mut gb = helper_b.pin();
            gb.flush();
            gb.repin();
            drop(gb);

            if weak.upgrade().is_none() {
                break;
            }
        }

        assert!(
            weak.upgrade().is_some(),
            "newly installed PartitionEndpointState was reclaimed by apply_partition \
             while still reachable from self.partitions (use-after-free regression)"
        );

        // After dropping the store the canary must be released, confirming
        // the install path eventually frees the state via the normal Drop
        // path rather than leaking it.
        drop(store);
        assert!(
            weak.upgrade().is_none(),
            "PartitionEndpointState was leaked: not dropped after LocationStateStore drop"
        );
    }
}
