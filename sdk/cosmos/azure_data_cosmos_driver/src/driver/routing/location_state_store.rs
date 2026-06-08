// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Unified lock-free location state store.

use std::{
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc, Weak,
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use crossbeam_epoch::{self as epoch, Atomic, Owned};
use futures::future::BoxFuture;
use url::Url;

#[cfg(feature = "tokio")]
use crate::driver::transport::background_task_manager::BackgroundTaskManager;
use crate::{
    driver::{
        cache::{AccountMetadataCache, AccountProperties},
        transport::connectivity_probe::{ConnectivityProbe, ProbeOutcome, ProbeRole},
    },
    models::AccountEndpoint,
    options::Region,
};

use super::{
    build_account_endpoint_state, expire_partition_overrides, expire_unavailable_endpoints,
    mark_endpoint_unavailable, mark_partition_unavailable,
    partition_endpoint_state::{PartitionEndpointState, PartitionFailoverConfig},
    AccountEndpointState, CosmosEndpoint, LocationEffect,
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
        ) -> BoxFuture<'static, crate::error::Result<AccountProperties>>
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
    /// Probe used to validate Gateway 2.0 (thin-client) proxy endpoints
    /// after every account-metadata refresh. When `None`, Gateway 2.0 is
    /// gated only by `gateway20_enabled` and the presence of advertised
    /// thin-client endpoints (today's behavior). When `Some`, a failing
    /// probe flips `gateway20_runtime_blocked` and suppresses Gateway 2.0
    /// from the routing snapshot until a subsequent probe succeeds.
    connectivity_probe: Option<Arc<dyn ConnectivityProbe>>,
    /// Runtime gate set by `run_connectivity_probe`. When `true`, the
    /// rebuilt snapshot drops Gateway 2.0 URLs even though
    /// `gateway20_enabled` is on and the service advertises thin-client
    /// endpoints. Defaults to `false` (fail-open) so behavior is unchanged
    /// when no probe is wired.
    gateway20_runtime_blocked: AtomicBool,
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
        connectivity_probe: Option<Arc<dyn ConnectivityProbe>>,
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
            connectivity_probe,
            gateway20_runtime_blocked: AtomicBool::new(false),
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

        // Probe Gateway 2.0 proxy endpoints BEFORE syncing into the routing
        // snapshot, so the subsequent rebuild reflects the probe outcome
        // (via `effective_gateway20_enabled`). A transition in the probe
        // gate clears `last_synced_etag` inside the helper so the same-etag
        // fast path in `sync_account_properties` does not skip the rebuild.
        self.run_connectivity_probe(&properties).await;

        let default_endpoint = self.default_endpoint.clone();
        self.sync_account_properties(properties, &default_endpoint);
    }

    /// Returns `gateway20_enabled && !gateway20_runtime_blocked`. The
    /// snapshot builder uses this in place of the static configured flag so
    /// a failed connectivity probe transparently disables Gateway 2.0
    /// routing without changing the operator-facing toggle.
    fn effective_gateway20_enabled(&self) -> bool {
        self.gateway20_enabled && !self.gateway20_runtime_blocked.load(Ordering::Acquire)
    }

    /// Runs the wired connectivity probe against the Gateway 2.0 endpoints
    /// advertised in `properties` and updates `gateway20_runtime_blocked`.
    ///
    /// No-ops when:
    /// * no probe is wired (constructor passed `None`),
    /// * `gateway20_enabled` is false (operator-disabled), or
    /// * the account metadata contains no thin-client endpoints (then
    ///   `effective_gateway20_enabled` is moot — the snapshot rebuild
    ///   cannot pick up Gateway 2.0 URLs that were never returned).
    ///
    /// On a probe state transition (blocked ↔ unblocked) the
    /// `last_synced_etag` is cleared so the immediately-following
    /// `sync_account_properties` call rebuilds the snapshot even when the
    /// server returned an unchanged etag.
    async fn run_connectivity_probe(&self, properties: &AccountProperties) {
        let Some(probe) = self.connectivity_probe.as_ref() else {
            return;
        };
        if !self.gateway20_enabled {
            return;
        }

        let mut endpoints: Vec<(Region, ProbeRole, Url)> = Vec::with_capacity(
            properties.thin_client_writable_locations.len()
                + properties.thin_client_readable_locations.len(),
        );
        for loc in &properties.thin_client_writable_locations {
            endpoints.push((
                loc.name.clone(),
                ProbeRole::Write,
                loc.database_account_endpoint.url().clone(),
            ));
        }
        for loc in &properties.thin_client_readable_locations {
            endpoints.push((
                loc.name.clone(),
                ProbeRole::Read,
                loc.database_account_endpoint.url().clone(),
            ));
        }

        if endpoints.is_empty() {
            // No advertised Gateway 2.0 endpoints. Reset the gate so a
            // future iteration that DOES advertise endpoints starts from
            // unblocked, and the snapshot naturally falls back to Gateway
            // V1 because `build_account_endpoint_state` has no thin-client
            // URLs to pick up.
            let was_blocked = self.gateway20_runtime_blocked.swap(false, Ordering::AcqRel);
            if was_blocked {
                self.last_synced_etag.lock().unwrap().clear();
            }
            return;
        }

        let outcome = probe.probe_endpoints(endpoints).await;
        let now_blocked = !outcome.is_healthy();
        let was_blocked = self
            .gateway20_runtime_blocked
            .swap(now_blocked, Ordering::AcqRel);

        if was_blocked != now_blocked {
            // Clear the etag so the same-etag fast path in
            // `sync_account_properties` does not skip the rebuild that has
            // to flip Gateway 2.0 routing on or off.
            self.last_synced_etag.lock().unwrap().clear();
            tracing::info!(
                endpoint = %self.account_endpoint,
                gateway20_runtime_blocked = now_blocked,
                "LocationStateStore: Gateway 2.0 connectivity probe state changed",
            );
        }

        if let ProbeOutcome::Failed { failures } = &outcome {
            for (region, failure) in failures {
                tracing::warn!(
                    endpoint = %self.account_endpoint,
                    region = %region,
                    failure = %failure,
                    "Gateway 2.0 connectivity probe failed",
                );
            }
        }
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
                self.effective_gateway20_enabled(),
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
            let fut: BoxFuture<'static, crate::error::Result<AccountProperties>> =
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
            None,
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
            let fut: BoxFuture<'static, crate::error::Result<AccountProperties>> =
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
            None,
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
            let fut: BoxFuture<'static, crate::error::Result<AccountProperties>> =
                Box::pin(async move {
                    let n = total.fetch_add(1, Ordering::SeqCst);
                    if n == 0 {
                        Err(crate::error::CosmosError::builder()
                            .with_status(crate::error::CosmosStatus::new(
                                azure_core::http::StatusCode::BadRequest,
                            ))
                            .with_message("simulated network failure")
                            .build())
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
            None,
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
            let fut: BoxFuture<'static, crate::error::Result<AccountProperties>> =
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
            None,
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

    /// End-to-end coverage for the "service stops advertising Gateway 2.0"
    /// fallback. The store is initialized with `gateway20_enabled=true`,
    /// then fed two successive `AccountProperties` payloads via
    /// `sync_account_properties` (the same path exercised by both the
    /// event-driven refresh and the background 5-minute refresh loop):
    ///
    /// 1. First payload includes `thinClient*Locations`. The rebuilt
    ///    snapshot must carry `gateway20_url` on every preferred endpoint
    ///    and `uses_gateway20(true)` must report `true`.
    /// 2. Second payload omits `thinClient*Locations` entirely, simulating
    ///    the database-account call no longer returning thin-client
    ///    endpoints. The rebuilt snapshot must drop the `gateway20_url`,
    ///    causing subsequent operations to route through the standard
    ///    compute gateway even though the operator-level toggle is still
    ///    on.
    ///
    /// This test mocks the refresh function (no live infra required) and
    /// verifies the dynamic transport switch end-to-end through the same
    /// state machine that runs in production.
    #[test]
    fn sync_account_properties_drops_gateway20_when_thin_client_locations_disappear() {
        let default_endpoint = CosmosEndpoint::global(test_endpoint().url().clone());
        // The refresh fn is unused in this test — sync_account_properties
        // is called directly with explicit payloads.
        let refresh = Arc::new(|_previous: Option<Arc<AccountProperties>>| {
            let payload = test_refresh_payload();
            let fut: BoxFuture<'static, crate::error::Result<AccountProperties>> =
                Box::pin(async move { Ok(payload) });
            fut
        });

        let store = LocationStateStore::new(
            Arc::new(AccountMetadataCache::new()),
            test_endpoint(),
            default_endpoint.clone(),
            refresh,
            // gateway20_enabled — operator has Gateway 2.0 on.
            true,
            Duration::from_secs(60),
            PartitionFailoverConfig::default(),
            Vec::new(),
            None,
        );

        // ── First refresh: service advertises Gateway 2.0 endpoints. ─────
        let with_g2: AccountProperties = serde_json::from_value(serde_json::json!({
            "_self": "",
            "id": "test",
            "_rid": "test.documents.azure.com",
            "_etag": "etag-with-g2",
            "media": "//media/",
            "addresses": "//addresses/",
            "_dbs": "//dbs/",
            "writableLocations": [{ "name": "eastus", "databaseAccountEndpoint": "https://test-eastus.documents.azure.com:443/" }],
            "readableLocations": [{ "name": "eastus", "databaseAccountEndpoint": "https://test-eastus.documents.azure.com:443/" }],
            "thinClientWritableLocations": [{ "name": "eastus", "databaseAccountEndpoint": "https://test-eastus-thin.documents.azure.com:444/" }],
            "thinClientReadableLocations": [{ "name": "eastus", "databaseAccountEndpoint": "https://test-eastus-thin.documents.azure.com:444/" }],
            "enableMultipleWriteLocations": false,
            "userReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "userConsistencyPolicy": { "defaultConsistencyLevel": "Session" },
            "systemReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "readPolicy": { "primaryReadCoefficient": 1, "secondaryReadCoefficient": 1 },
            "queryEngineConfiguration": "{}"
        })).unwrap();
        store.sync_account_properties(Arc::new(with_g2), &default_endpoint);

        let snap_g2 = store.snapshot();
        assert!(
            snap_g2.account.preferred_read_endpoints[0]
                .gateway20_url()
                .is_some(),
            "after first refresh the read endpoint must carry a Gateway 2.0 URL"
        );
        assert!(
            snap_g2.account.preferred_write_endpoints[0]
                .gateway20_url()
                .is_some(),
            "after first refresh the write endpoint must carry a Gateway 2.0 URL"
        );
        assert!(
            snap_g2.account.preferred_read_endpoints[0].uses_gateway20(true),
            "request pipeline must route reads via Gateway 2.0 while the service advertises thin-client endpoints"
        );
        assert!(
            snap_g2.account.preferred_write_endpoints[0].uses_gateway20(true),
            "request pipeline must route writes via Gateway 2.0 while the service advertises thin-client endpoints"
        );

        // ── Second refresh: service stops advertising Gateway 2.0. ───────
        // Same standard locations + a different etag (otherwise the
        // etag-equality fast path would skip the rebuild). No
        // `thinClient*Locations`.
        let without_g2: AccountProperties = serde_json::from_value(serde_json::json!({
            "_self": "",
            "id": "test",
            "_rid": "test.documents.azure.com",
            "_etag": "etag-without-g2",
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
        })).unwrap();
        store.sync_account_properties(Arc::new(without_g2), &default_endpoint);

        let snap_fallback = store.snapshot();
        assert!(
            snap_fallback.account.generation > snap_g2.account.generation,
            "second sync must bump generation so callers re-resolve endpoints"
        );
        assert!(
            snap_fallback.account.preferred_read_endpoints[0]
                .gateway20_url()
                .is_none(),
            "read endpoint must lose its Gateway 2.0 URL after the service stops advertising thinClientReadableLocations"
        );
        assert!(
            snap_fallback.account.preferred_write_endpoints[0]
                .gateway20_url()
                .is_none(),
            "write endpoint must lose its Gateway 2.0 URL after the service stops advertising thinClientWritableLocations"
        );
        assert!(
            !snap_fallback.account.preferred_read_endpoints[0].uses_gateway20(true),
            "request pipeline must fall back to the compute gateway for reads even though the operator toggle is still on"
        );
        assert!(
            !snap_fallback.account.preferred_write_endpoints[0].uses_gateway20(true),
            "request pipeline must fall back to the compute gateway for writes even though the operator toggle is still on"
        );
        // The compute gateway URL itself must be unchanged — only the
        // Gateway 2.0 overlay is removed.
        assert_eq!(
            snap_fallback.account.preferred_read_endpoints[0]
                .selected_url(true)
                .as_str(),
            "https://test-eastus.documents.azure.com/",
            "fallback URL must be the standard compute-gateway URL from writable/readableLocations"
        );
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
            let fut: BoxFuture<'static, crate::error::Result<AccountProperties>> =
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
            None,
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

    /// Mock probe used by the runtime-gating tests below. Returns a canned
    /// [`ProbeOutcome`] on every call and tracks how many times it was
    /// invoked so the tests can assert refresh-driven probing.
    #[derive(Debug)]
    struct MockProbe {
        outcome: std::sync::Mutex<ProbeOutcome>,
        calls: AtomicUsize,
    }

    impl MockProbe {
        fn new(outcome: ProbeOutcome) -> Self {
            Self {
                outcome: std::sync::Mutex::new(outcome),
                calls: AtomicUsize::new(0),
            }
        }

        fn set_outcome(&self, outcome: ProbeOutcome) {
            *self.outcome.lock().unwrap() = outcome;
        }
    }

    #[async_trait::async_trait]
    impl ConnectivityProbe for MockProbe {
        async fn probe_endpoints(&self, _: Vec<(Region, ProbeRole, Url)>) -> ProbeOutcome {
            self.calls.fetch_add(1, Ordering::SeqCst);
            self.outcome.lock().unwrap().clone()
        }
    }

    fn refresh_payload_with_g2() -> AccountProperties {
        serde_json::from_value(serde_json::json!({
            "_self": "",
            "id": "test",
            "_rid": "test.documents.azure.com",
            "_etag": "etag-with-g2",
            "media": "//media/",
            "addresses": "//addresses/",
            "_dbs": "//dbs/",
            "writableLocations": [{ "name": "eastus", "databaseAccountEndpoint": "https://test-eastus.documents.azure.com:443/" }],
            "readableLocations": [{ "name": "eastus", "databaseAccountEndpoint": "https://test-eastus.documents.azure.com:443/" }],
            "thinClientWritableLocations": [{ "name": "eastus", "databaseAccountEndpoint": "https://test-eastus-thin.documents.azure.com:10250/" }],
            "thinClientReadableLocations": [{ "name": "eastus", "databaseAccountEndpoint": "https://test-eastus-thin.documents.azure.com:10250/" }],
            "enableMultipleWriteLocations": false,
            "userReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "userConsistencyPolicy": { "defaultConsistencyLevel": "Session" },
            "systemReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "readPolicy": { "primaryReadCoefficient": 1, "secondaryReadCoefficient": 1 },
            "queryEngineConfiguration": "{}"
        }))
        .unwrap()
    }

    /// A failing connectivity probe must suppress Gateway 2.0 from the
    /// rebuilt routing snapshot even though `gateway20_enabled = true` and
    /// the account metadata still advertises thin-client endpoints. After
    /// a subsequent successful probe (with the SAME etag — exercises the
    /// transition-clears-etag path), the next refresh must restore
    /// Gateway 2.0 routing without a server-side metadata change.
    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn connectivity_probe_failure_suppresses_gateway20_then_recovers() {
        let default_endpoint = CosmosEndpoint::global(test_endpoint().url().clone());

        let refresh = Arc::new(|_previous: Option<Arc<AccountProperties>>| {
            let payload = refresh_payload_with_g2();
            let fut: BoxFuture<'static, crate::error::Result<AccountProperties>> =
                Box::pin(async move { Ok(payload) });
            fut
        });

        let probe = Arc::new(MockProbe::new(ProbeOutcome::Failed {
            failures: vec![(
                "eastus".into(),
                crate::driver::transport::connectivity_probe::ProbeFailure::Status(503),
            )],
        }));

        let store = Arc::new(LocationStateStore::new(
            Arc::new(AccountMetadataCache::new()),
            test_endpoint(),
            default_endpoint.clone(),
            refresh,
            true,
            Duration::from_secs(60),
            PartitionFailoverConfig::default(),
            Vec::new(),
            Some(probe.clone() as Arc<dyn ConnectivityProbe>),
        ));

        // First refresh: probe fails — snapshot must NOT carry Gateway 2.0 URLs.
        store.force_refresh_account_properties().await;
        assert_eq!(probe.calls.load(Ordering::SeqCst), 1);
        let snap_failed = store.snapshot();
        assert!(
            snap_failed.account.preferred_read_endpoints[0]
                .gateway20_url()
                .is_none(),
            "failed probe must suppress Gateway 2.0 read URL even when advertised"
        );
        assert!(
            snap_failed.account.preferred_write_endpoints[0]
                .gateway20_url()
                .is_none(),
            "failed probe must suppress Gateway 2.0 write URL even when advertised"
        );

        // Flip the probe to healthy. Server-side metadata (and etag) are
        // unchanged — the transition path in `run_connectivity_probe` must
        // clear the etag so the rebuild happens despite the fast-path skip.
        probe.set_outcome(ProbeOutcome::AllHealthy);
        store.force_refresh_account_properties().await;
        assert_eq!(probe.calls.load(Ordering::SeqCst), 2);
        let snap_recovered = store.snapshot();
        assert!(
            snap_recovered.account.preferred_read_endpoints[0]
                .gateway20_url()
                .is_some(),
            "successful probe must restore Gateway 2.0 read URL without a metadata change"
        );
        assert!(
            snap_recovered.account.preferred_write_endpoints[0]
                .gateway20_url()
                .is_some(),
            "successful probe must restore Gateway 2.0 write URL without a metadata change"
        );
    }

    /// When no probe is wired the constructor stays fail-open, matching
    /// today's behavior: `effective_gateway20_enabled` equals
    /// `gateway20_enabled` and Gateway 2.0 routing is governed purely by
    /// whether the account metadata advertises thin-client endpoints.
    #[test]
    fn no_probe_wired_preserves_existing_gateway20_behavior() {
        let default_endpoint = CosmosEndpoint::global(test_endpoint().url().clone());

        let refresh = Arc::new(|_previous: Option<Arc<AccountProperties>>| {
            let payload = refresh_payload_with_g2();
            let fut: BoxFuture<'static, crate::error::Result<AccountProperties>> =
                Box::pin(async move { Ok(payload) });
            fut
        });

        let store = LocationStateStore::new(
            Arc::new(AccountMetadataCache::new()),
            test_endpoint(),
            default_endpoint.clone(),
            refresh,
            true,
            Duration::from_secs(60),
            PartitionFailoverConfig::default(),
            Vec::new(),
            None,
        );

        assert!(store.effective_gateway20_enabled());
        store.sync_account_properties(Arc::new(refresh_payload_with_g2()), &default_endpoint);
        let snap = store.snapshot();
        assert!(
            snap.account.preferred_read_endpoints[0]
                .gateway20_url()
                .is_some(),
            "with no probe wired the snapshot must keep Gateway 2.0 routing as before"
        );
    }
}
