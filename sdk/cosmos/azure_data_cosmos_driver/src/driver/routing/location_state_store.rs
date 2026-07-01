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
use url::Url;

#[cfg(feature = "tokio")]
use crate::driver::transport::background_task_manager::BackgroundTaskManager;
use crate::{
    driver::cache::{AccountMetadataCache, AccountProperties},
    models::AccountEndpoint,
    options::{PartitionFailoverOptions, Region},
};

use super::{
    advance_hub_region_discovery, build_account_endpoint_state, cache_hub_region,
    expire_partition_overrides, mark_endpoint_unavailable, mark_partition_unavailable,
    partition_endpoint_state::PartitionEndpointState, partition_key_range_id::PartitionKeyRangeId,
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
        ) -> BoxFuture<'static, crate::error::Result<AccountProperties>>
        + Send
        + Sync,
>;

/// Connectivity probe for a single endpoint URL.
///
/// Returns `true` if the endpoint is reachable (a request to it completed),
/// `false` if it could not be connected to. Used to gate failback of an
/// endpoint that was previously marked unavailable: an endpoint only rejoins
/// the routing rotation after a probe confirms it is reachable, rather than
/// purely because an unavailability cooldown elapsed.
#[cfg(feature = "tokio")]
pub(crate) type EndpointProbeFn = Arc<dyn Fn(Url) -> BoxFuture<'static, bool> + Send + Sync>;

/// Interval between iterations of the background endpoint-probe loop, which
/// probes endpoints whose unavailability cooldown has elapsed and fails back
/// only those that are reachable.
#[cfg(feature = "tokio")]
pub(crate) const ENDPOINT_PROBE_INTERVAL: Duration = Duration::from_secs(60);

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
        partition_failover_options: PartitionFailoverOptions,
        preferred_regions: Vec<Region>,
    ) -> Self {
        let account_state = AccountEndpointState::single(default_endpoint.clone());
        let partition_state = PartitionEndpointState::new(partition_failover_options);

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
                LocationEffect::CacheHubRegion {
                    partition_key_range_id,
                    hub_endpoint,
                } => {
                    let pk_range_id = partition_key_range_id.clone();
                    let hub_endpoint = hub_endpoint.clone();
                    self.apply_partition(|current_partitions| {
                        cache_hub_region(current_partitions, &pk_range_id, &hub_endpoint)
                    });
                }
                LocationEffect::AdvanceHubRegionDiscovery {
                    partition_key_range_id,
                    failed_endpoint,
                } => {
                    let pk_range_id = partition_key_range_id.clone();
                    let failed_endpoint = failed_endpoint.clone();
                    self.apply_partition(|current_partitions| {
                        let account = self.account_snapshot();
                        advance_hub_region_discovery(
                            current_partitions,
                            &account,
                            &pk_range_id,
                            &failed_endpoint,
                        )
                    });
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
    /// this timer-driven refresh fails (network error, service 5xx, ...), the
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
        self.apply_account(|current| {
            let mut next = build_account_endpoint_state(
                &properties,
                default_endpoint.clone(),
                Some(current.generation),
                self.gateway20_enabled,
                &self.preferred_regions,
            );
            // Carry forward all unavailability marks from the current state.
            // The background endpoint-probe loop is the sole owner of
            // account-level failback: a marked endpoint is only cleared once a
            // connectivity probe confirms it is reachable, never on a time
            // basis. See `probe_and_failback_unavailable_endpoints`.
            next.unavailable_endpoints = current.unavailable_endpoints.clone();
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
        self.apply_partition(|previous| {
            let mut next = previous.clone();
            next.per_partition_automatic_failover_enabled =
                per_partition_automatic_failover_enabled;
            // The incident kill switch (`AZURE_COSMOS_PPCB_ENABLED_OVERRIDE`) is
            // authoritative: when set it wins over both the account property and
            // the base option. Only when it is unset does PPCB fall back to
            // `account property || option`.
            next.per_partition_circuit_breaker_enabled = previous
                .config
                .circuit_breaker_enabled_override()
                .unwrap_or_else(|| {
                    per_partition_automatic_failover_enabled
                        || previous.config.circuit_breaker_enabled()
                });

            // Drop per-partition routing overrides on any edge of either
            // enablement flag. The eligibility gate in `is_eligible_for_ppaf` /
            // `is_eligible_for_ppcb` already prevents stale entries from being
            // *applied* while the corresponding feature is off, but leaving
            // them in place would let an outdated override silently re-apply
            // if the operator (or background flip) re-enabled the feature
            // later. Clearing on every edge -- not just disable -- keeps the
            // override maps strictly in sync with their owning feature flag
            // and removes a class of "stale entry survives across a flip"
            // bugs without any cost on the steady-state path.
            if previous.per_partition_automatic_failover_enabled
                != next.per_partition_automatic_failover_enabled
            {
                // Clearing to prevent stale PPAF entries from silently
                // re-applying after a later re-enable. (Disable-time
                // correctness is already guaranteed by `is_eligible_for_ppaf`
                // gating every read of this map.)
                next.failover_overrides.clear();
            }
            if previous.per_partition_circuit_breaker_enabled
                != next.per_partition_circuit_breaker_enabled
            {
                // Same rationale as the PPAF clear above, applied to the PPCB
                // override map. Note that PPCB tracks PPAF here (PPCB is
                // implicitly on whenever PPAF is on), so an isolated PPAF
                // flip also flips PPCB and clears both maps -- exactly the
                // intended invariant.
                next.circuit_breaker_overrides.clear();
            }

            next
        });
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

    /// Starts the background endpoint-probe loop.
    ///
    /// This loop is the sole owner of account-level endpoint failback. An
    /// endpoint marked unavailable is never cleared simply because its cooldown
    /// elapsed; once the cooldown has elapsed the loop probes the endpoint for
    /// connectivity (via `probe_fn`) and only fails back (clears the
    /// unavailability mark) endpoints that are reachable. Unreachable endpoints
    /// have their cooldown reset and stay out of the rotation. This prevents
    /// repeatedly routing real traffic to an endpoint that is still unreachable
    /// (e.g. firewall-blocked), which otherwise causes sustained low throughput.
    ///
    /// Mirrors `start_failback_loop` (`Weak<Self>` for self-termination,
    /// `BackgroundTaskManager` for abort-on-drop).
    #[cfg(feature = "tokio")]
    pub fn start_endpoint_probe_loop(self: &Arc<Self>, probe_fn: EndpointProbeFn) {
        let weak_store: Weak<LocationStateStore> = Arc::downgrade(self);
        self.background_task_manager.spawn(async move {
            endpoint_probe_loop(weak_store, probe_fn, ENDPOINT_PROBE_INTERVAL).await;
        });
    }

    /// Probes every account-level unavailable endpoint whose cooldown has
    /// elapsed and fails back only those that are reachable.
    ///
    /// For each due endpoint the probe is awaited out of band (never under a
    /// state lock — `apply_account` is only entered to record the result). A
    /// reachable endpoint has its unavailability mark removed (rejoining the
    /// rotation); an unreachable endpoint has its cooldown reset so it is
    /// re-probed after the next interval rather than thrashing.
    #[cfg(feature = "tokio")]
    pub(crate) async fn probe_and_failback_unavailable_endpoints(
        &self,
        probe_fn: &EndpointProbeFn,
    ) {
        let now = Instant::now();
        let ttl = self.endpoint_unavailability_ttl;
        let snapshot = self.account_snapshot();

        // Capture each due endpoint's observed `marked_at` so we can detect a
        // concurrent re-mark that lands while the (potentially slow) probe is in
        // flight. The URLs are borrowed from `snapshot` (held for the whole
        // call) — we clone only when handing one to `probe_fn`.
        let due: Vec<(&Url, Instant)> = snapshot
            .unavailable_endpoints
            .iter()
            .filter(|(_, (marked_at, _))| now.saturating_duration_since(*marked_at) >= ttl)
            .map(|(url, (marked_at, _))| (url, *marked_at))
            .collect();

        if due.is_empty() {
            return;
        }

        for (url, observed_marked_at) in due {
            let reachable = probe_fn(url.clone()).await;
            self.apply_account(|current| {
                let mut next = current.clone();
                if let Some((marked_at, _)) = next.unavailable_endpoints.get_mut(url) {
                    if reachable {
                        // Only fail back if the mark is the same one we probed.
                        // A newer `marked_at` means a concurrent transport
                        // failure re-marked the endpoint while the probe was in
                        // flight, so keep it out of rotation and re-probe later.
                        if *marked_at == observed_marked_at {
                            next.unavailable_endpoints.remove(url);
                        }
                    } else {
                        // Reset the cooldown so the endpoint is re-probed later.
                        *marked_at = Instant::now();
                    }
                }
                next
            });

            if reachable {
                if !self
                    .account_snapshot()
                    .unavailable_endpoints
                    .contains_key(url)
                {
                    tracing::info!(
                        endpoint = %url,
                        "endpoint passed connectivity probe; failing back",
                    );
                }
            } else {
                tracing::warn!(
                    endpoint = %url,
                    "endpoint failed connectivity probe; keeping it out of rotation \
                     and resetting its cooldown for a later re-probe",
                );
            }
        }
    }

    /// Records that the alternate (secondary) hedge attempt completed before
    /// the primary in
    /// [`execute_hedged`](crate::driver::pipeline::operation_pipeline).
    ///
    /// Increments a per-`(partition, primary_region)` counter atomically
    /// via [`Self::apply_partition`]. When the counter reaches
    /// [`PartitionFailoverOptions::consecutive_hedge_win_threshold`] the
    /// partition is tripped by installing an `Unhealthy` entry in
    /// `circuit_breaker_overrides` (the same shape PPCB uses for hard
    /// failures), so subsequent requests route away from the degraded
    /// primary region. The trip is recovered by the existing PPCB
    /// failback sweep.
    ///
    /// `primary_region` is `None` for default-endpoint accounts whose
    /// snapshot does not carry a named region.
    pub fn record_consecutive_hedge_win(
        &self,
        partition: &PartitionKeyRangeId,
        primary_region: Option<&Region>,
    ) {
        tracing::debug!(
            partition = %partition,
            primary_region = ?primary_region.map(Region::as_str),
            "recorded alternate-region hedge win",
        );
        let account = self.account_snapshot();
        self.apply_partition(|current| {
            record_hedge_alternate_win(current, &account, partition, primary_region)
        });
    }

    /// Records that the primary attempt won in
    /// [`execute_hedged`](crate::driver::pipeline::operation_pipeline).
    ///
    /// Clears the per-`(partition, primary_region)` consecutive-hedge-win
    /// counter atomically via [`Self::apply_partition`] so transient
    /// cross-region latency spikes do not accumulate into a trip over
    /// arbitrarily long timescales. Does not touch
    /// `circuit_breaker_overrides` — an existing trip recovers via the
    /// failback sweep, not via primary wins.
    pub fn record_primary_win(
        &self,
        partition: &PartitionKeyRangeId,
        primary_region: Option<&Region>,
    ) {
        tracing::debug!(
            partition = %partition,
            primary_region = ?primary_region.map(Region::as_str),
            "recorded primary-region hedge win",
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
async fn failback_loop(weak_store: Weak<LocationStateStore>, config: PartitionFailoverOptions) {
    loop {
        tokio::time::sleep(config.failback_sweep_interval()).await;

        let Some(store) = weak_store.upgrade() else {
            // LocationStateStore was dropped — exit the loop.
            break;
        };

        store.apply_partition(|current_partitions| {
            expire_partition_overrides(
                current_partitions,
                Instant::now(),
                config.partition_unavailability_duration(),
            )
        });
    }
}

/// Background endpoint-probe loop. Periodically probes account-level endpoints
/// whose unavailability cooldown has elapsed and fails back only those that are
/// reachable. Exits when the `LocationStateStore` is dropped (`Weak::upgrade()`
/// returns `None`).
#[cfg(feature = "tokio")]
async fn endpoint_probe_loop(
    weak_store: Weak<LocationStateStore>,
    probe_fn: EndpointProbeFn,
    interval: Duration,
) {
    loop {
        tokio::time::sleep(interval).await;

        let Some(store) = weak_store.upgrade() else {
            // LocationStateStore was dropped — exit the loop.
            break;
        };

        store
            .probe_and_failback_unavailable_endpoints(&probe_fn)
            .await;
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
        driver::{
            cache::{AccountRegion, ConsistencyPolicy, ReadPolicy, ReplicationPolicy},
            routing::{CosmosEndpoint, LocationEffect, UnavailableReason},
        },
        models::{AccountEndpoint, DefaultConsistencyLevel},
    };

    use super::*;

    fn test_endpoint() -> AccountEndpoint {
        AccountEndpoint::from(url::Url::parse("https://test.documents.azure.com:443/").unwrap())
    }

    /// Single canonical fake account-properties payload used by every test in
    /// this module. Override only the fields that matter for a specific test
    /// using struct-update syntax (`..default_account_properties()`):
    ///
    /// ```ignore
    /// AccountProperties {
    ///     enable_per_partition_failover_behavior: true,
    ///     etag: "etag-on".into(),
    ///     ..default_account_properties()
    /// }
    /// ```
    ///
    /// Wire-format round-trip is exercised in
    /// `driver::cache::account_metadata_cache::tests::deserialize_full_account_payload`;
    /// these helpers exist to test `sync_account_properties` flag-edge logic,
    /// which has nothing to gain from re-running serde on every call.
    fn default_account_properties() -> AccountProperties {
        let eastus_endpoint = AccountEndpoint::from(
            url::Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let eastus_region = AccountRegion {
            name: Region::from("eastus"),
            database_account_endpoint: eastus_endpoint,
        };
        AccountProperties {
            self_link: String::new(),
            id: "test".into(),
            rid: "test.documents.azure.com".into(),
            media: "//media/".into(),
            addresses: "//addresses/".into(),
            dbs: "//dbs/".into(),
            writable_locations: vec![eastus_region.clone()],
            readable_locations: vec![eastus_region],
            enable_multiple_write_locations: false,
            continuous_backup_enabled: false,
            enable_n_region_synchronous_commit: false,
            enable_per_partition_failover_behavior: false,
            user_replication_policy: ReplicationPolicy {
                min_replica_set_size: 3,
                max_replica_set_size: 4,
            },
            user_consistency_policy: ConsistencyPolicy {
                default_consistency_level: DefaultConsistencyLevel::Session,
            },
            system_replication_policy: ReplicationPolicy {
                min_replica_set_size: 3,
                max_replica_set_size: 4,
            },
            read_policy: ReadPolicy {
                primary_read_coefficient: 1,
                secondary_read_coefficient: 1,
            },
            query_engine_configuration: "{}".into(),
            thin_client_writable_locations: Vec::new(),
            thin_client_readable_locations: Vec::new(),
            etag: "etag-1".into(),
        }
    }

    fn test_refresh_payload() -> AccountProperties {
        default_account_properties()
    }

    /// Builds an `AccountProperties` payload with explicit
    /// `enable_per_partition_failover_behavior` and caller-supplied etag --
    /// successive `sync_account_properties` calls with the same etag would be
    /// short-circuited by the unchanged-etag fast path, so tests that walk a
    /// state machine across multiple syncs need distinct etags.
    fn test_payload_with_ppaf(enabled: bool, etag: &str) -> AccountProperties {
        AccountProperties {
            enable_per_partition_failover_behavior: enabled,
            etag: etag.into(),
            ..default_account_properties()
        }
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
            PartitionFailoverOptions::default(),
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

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn unavailable_endpoint_fails_back_only_after_successful_probe() {
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
            // Cooldown of zero so the marked endpoint is immediately probe-due.
            Duration::ZERO,
            PartitionFailoverOptions::default(),
            Vec::new(),
        );

        store
            .apply(&[LocationEffect::MarkEndpointUnavailable {
                endpoint: default_endpoint.clone(),
                reason: UnavailableReason::TransportError,
            }])
            .await;
        assert_eq!(store.snapshot().account.unavailable_endpoints.len(), 1);

        // A failing probe keeps the endpoint unavailable (cooldown is reset,
        // not cleared) — this is the core #4597 behavior: no time-based failback.
        let unreachable: EndpointProbeFn =
            Arc::new(|_url: Url| Box::pin(async move { false }) as BoxFuture<'static, bool>);
        store
            .probe_and_failback_unavailable_endpoints(&unreachable)
            .await;
        assert_eq!(
            store.snapshot().account.unavailable_endpoints.len(),
            1,
            "a failed probe must keep the endpoint unavailable"
        );

        // A successful probe fails the endpoint back (clears the mark).
        let reachable: EndpointProbeFn =
            Arc::new(|_url: Url| Box::pin(async move { true }) as BoxFuture<'static, bool>);
        store
            .probe_and_failback_unavailable_endpoints(&reachable)
            .await;
        assert_eq!(
            store.snapshot().account.unavailable_endpoints.len(),
            0,
            "a successful probe must fail the endpoint back"
        );
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn failed_probe_resets_cooldown_keeping_endpoint_out_of_rotation() {
        let default_endpoint = CosmosEndpoint::global(test_endpoint().url().clone());
        let refresh = Arc::new(|_previous: Option<Arc<AccountProperties>>| {
            let payload = test_refresh_payload();
            let fut: BoxFuture<'static, crate::error::Result<AccountProperties>> =
                Box::pin(async move { Ok(payload) });
            fut
        });

        let make_store = || {
            LocationStateStore::new(
                Arc::new(AccountMetadataCache::new()),
                test_endpoint(),
                default_endpoint.clone(),
                refresh.clone(),
                false,
                // Non-zero cooldown so the reset-on-failure is observable.
                Duration::from_secs(60),
                PartitionFailoverOptions::default(),
                Vec::new(),
            )
        };

        let endpoint = default_endpoint.url().clone();
        let insert_due_mark = |store: &LocationStateStore| {
            store.apply_account(|current| {
                let mut next = current.clone();
                next.unavailable_endpoints.insert(
                    endpoint.clone(),
                    (
                        // Old enough that the endpoint is already probe-due.
                        Instant::now() - Duration::from_secs(120),
                        UnavailableReason::TransportError,
                    ),
                );
                next
            });
        };

        let reachable: EndpointProbeFn =
            Arc::new(|_url: Url| Box::pin(async move { true }) as BoxFuture<'static, bool>);
        let unreachable: EndpointProbeFn =
            Arc::new(|_url: Url| Box::pin(async move { false }) as BoxFuture<'static, bool>);

        // Baseline: a due endpoint is failed back by a successful probe.
        let store = make_store();
        insert_due_mark(&store);
        store
            .probe_and_failback_unavailable_endpoints(&reachable)
            .await;
        assert_eq!(
            store.snapshot().account.unavailable_endpoints.len(),
            0,
            "a due endpoint must fail back on a successful probe",
        );

        // A failed probe resets the cooldown, so the endpoint is no longer due;
        // an immediately-following successful probe must NOT fail it back
        // (proving the reset is real, not a no-op as it would be with a zero
        // cooldown).
        let store = make_store();
        insert_due_mark(&store);
        store
            .probe_and_failback_unavailable_endpoints(&unreachable)
            .await;
        store
            .probe_and_failback_unavailable_endpoints(&reachable)
            .await;
        assert_eq!(
            store.snapshot().account.unavailable_endpoints.len(),
            1,
            "a failed probe must reset the cooldown so the endpoint is not \
             immediately re-probed and failed back",
        );
    }

    #[test]
    fn account_sync_preserves_unavailable_marks_for_probe_loop() {
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
            PartitionFailoverOptions::default(),
            Vec::new(),
        );

        let properties = Arc::new(test_refresh_payload());
        store.sync_account_properties(Arc::clone(&properties), &default_endpoint);

        let stale_endpoint = CosmosEndpoint::regional(
            "eastus".into(),
            url::Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        store.apply_account(|current| {
            let mut next = current.clone();
            next.unavailable_endpoints.insert(
                stale_endpoint.url().clone(),
                (
                    Instant::now() - Duration::from_secs(120),
                    UnavailableReason::TransportError,
                ),
            );
            next
        });

        // Re-sync with a different Arc (same data, different pointer). The
        // background endpoint-probe loop is the sole owner of failback, so an
        // unavailable mark must survive an account sync regardless of how long
        // ago it was set — only a successful probe may clear it. There is no
        // longer any time-based pruning on sync.
        let properties2 = Arc::new(test_refresh_payload());
        store.sync_account_properties(properties2, &default_endpoint);

        assert_eq!(
            store.snapshot().account.unavailable_endpoints.len(),
            1,
            "account sync must NOT clear unavailable marks; only a probe may",
        );
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
            PartitionFailoverOptions::default(),
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
            PartitionFailoverOptions::default(),
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

    #[tokio::test]
    async fn location_state_store_preserves_cache_on_refresh_failure() {
        let endpoint = test_endpoint();
        let default_endpoint = CosmosEndpoint::global(endpoint.url().clone());
        let total_refreshes = Arc::new(AtomicUsize::new(0));
        let total_refreshes_clone = Arc::clone(&total_refreshes);

        // Call 1 succeeds (bootstrap seeds the cache); calls 2+ surface a
        // typed 503 mirroring what `fetch_account_properties_with_transport`
        // now produces on a 5xx account-metadata response.
        let refresh = Arc::new(move |_previous: Option<Arc<AccountProperties>>| {
            let total = Arc::clone(&total_refreshes_clone);
            let payload = test_refresh_payload();
            let fut: BoxFuture<'static, crate::error::Result<AccountProperties>> =
                Box::pin(async move {
                    let n = total.fetch_add(1, Ordering::SeqCst);
                    if n == 0 {
                        Ok(payload)
                    } else {
                        Err(crate::error::CosmosError::builder()
                            .with_status(crate::error::CosmosStatus::new(
                                azure_core::http::StatusCode::ServiceUnavailable,
                            ))
                            .with_message("simulated 5xx on periodic account-metadata refresh")
                            .build())
                    }
                });
            fut
        });

        let cache = Arc::new(AccountMetadataCache::new());
        let store = LocationStateStore::new(
            Arc::clone(&cache),
            endpoint.clone(),
            default_endpoint,
            refresh,
            false,
            Duration::from_secs(60),
            PartitionFailoverOptions::default(),
            Vec::new(),
        );

        // Bootstrap: succeeds, seeds the cache.
        store.force_refresh_account_properties().await;
        assert_eq!(total_refreshes.load(Ordering::SeqCst), 1);
        let after_bootstrap = cache
            .get(&endpoint)
            .await
            .expect("bootstrap should have populated the cache");
        assert_eq!(
            after_bootstrap.etag, "etag-1",
            "bootstrap should install the v1 payload"
        );

        // Subsequent timer-driven refresh: fails. Cache must NOT be replaced
        // or evicted — concurrent operations should keep serving the v1 value.
        store.force_refresh_account_properties().await;
        assert_eq!(total_refreshes.load(Ordering::SeqCst), 2);

        let after_failed_refresh = cache
            .get(&endpoint)
            .await
            .expect("failed refresh must NOT evict the cached entry");
        assert_eq!(
            after_failed_refresh.etag, "etag-1",
            "failed refresh must NOT overwrite the previously-cached v1 payload",
        );
        assert!(
            Arc::ptr_eq(&after_bootstrap, &after_failed_refresh),
            "failed refresh must leave the exact same Arc in the cache",
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
            PartitionFailoverOptions::default(),
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

    // ── Hub region cache apply tests ──────────────────────────────────

    fn test_multi_region_refresh_payload() -> AccountProperties {
        serde_json::from_value(serde_json::json!({
            "_self": "",
            "id": "test",
            "_rid": "test.documents.azure.com",
            "_etag": "etag-multi",
            "media": "//media/",
            "addresses": "//addresses/",
            "_dbs": "//dbs/",
            "writableLocations": [{ "name": "eastus", "databaseAccountEndpoint": "https://test-eastus.documents.azure.com:443/" }],
            "readableLocations": [
                { "name": "eastus", "databaseAccountEndpoint": "https://test-eastus.documents.azure.com:443/" },
                { "name": "westus", "databaseAccountEndpoint": "https://test-westus.documents.azure.com:443/" }
            ],
            "enableMultipleWriteLocations": false,
            // Hub-region caching is PPAF-gated, so the apply-tests below
            // need an account that opts into PPAF.
            "enablePerPartitionFailoverBehavior": true,
            "userReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "userConsistencyPolicy": { "defaultConsistencyLevel": "Session" },
            "systemReplicationPolicy": { "minReplicaSetSize": 3, "maxReplicasetSize": 4 },
            "readPolicy": { "primaryReadCoefficient": 1, "secondaryReadCoefficient": 1 },
            "queryEngineConfiguration": "{}"
        }))
        .unwrap()
    }

    fn build_store_with_two_regions() -> LocationStateStore {
        let default_endpoint = CosmosEndpoint::global(test_endpoint().url().clone());
        let refresh = Arc::new(|_previous: Option<Arc<AccountProperties>>| {
            let payload = test_multi_region_refresh_payload();
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
            PartitionFailoverOptions::default(),
            Vec::new(),
        );

        let properties = Arc::new(test_multi_region_refresh_payload());
        store.sync_account_properties(properties, &default_endpoint);

        store
    }

    /// Integration test for the apply pipeline + hub-region cache effects:
    /// exercises CacheHubRegion → AdvanceHubRegionDiscovery → CacheHubRegion
    /// in sequence and verifies the SetCurrent-only update semantics
    /// preserve `failed_endpoints`
    #[tokio::test]
    async fn apply_cache_hub_region_then_advance_then_cache_again() {
        let store = build_store_with_two_regions();
        let eastus = CosmosEndpoint::regional(
            "eastus".into(),
            url::Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let westus = CosmosEndpoint::regional(
            "westus".into(),
            url::Url::parse("https://test-westus.documents.azure.com:443/").unwrap(),
        );

        store
            .apply(&[LocationEffect::CacheHubRegion {
                partition_key_range_id: "0".parse().unwrap(),
                hub_endpoint: eastus.clone(),
            }])
            .await;
        store
            .apply(&[LocationEffect::AdvanceHubRegionDiscovery {
                partition_key_range_id: "0".parse().unwrap(),
                failed_endpoint: eastus.clone(),
            }])
            .await;
        store
            .apply(&[LocationEffect::CacheHubRegion {
                partition_key_range_id: "0".parse().unwrap(),
                hub_endpoint: westus.clone(),
            }])
            .await;

        let snapshot = store.snapshot();
        let entry = snapshot.partitions.failover_overrides.get("0").unwrap();
        assert_eq!(entry.current_endpoint, westus);
        assert!(
            entry.failed_endpoints.contains(&eastus),
            "re-caching on success must preserve failed_endpoints so a future 403/3 rotation does not re-try eastus, got failed_endpoints={:?}",
            entry.failed_endpoints,
        );
    }

    // -- PPAF dynamic enablement (issue #4325) ----------------------------
    //
    // The driver consumes `AccountProperties.enable_per_partition_failover_behavior`
    // as the single source of truth for PPAF enablement. These tests verify
    // that `sync_account_properties` propagates the server flag into the live
    // `PartitionEndpointState.per_partition_automatic_failover_enabled` on
    // every refresh, and that stale `failover_overrides` entries are dropped
    // when the server flag flips off so they cannot silently re-apply if the
    // operator re-enables PPAF later.

    fn build_store_for_ppaf_tests() -> LocationStateStore {
        let default_endpoint = CosmosEndpoint::global(test_endpoint().url().clone());
        let refresh = Arc::new(|_previous: Option<Arc<AccountProperties>>| {
            let payload = test_refresh_payload();
            let fut: BoxFuture<'static, crate::error::Result<AccountProperties>> =
                Box::pin(async move { Ok(payload) });
            fut
        });

        LocationStateStore::new(
            Arc::new(AccountMetadataCache::new()),
            test_endpoint(),
            default_endpoint,
            refresh,
            false,
            Duration::from_secs(60),
            PartitionFailoverOptions::default(),
            Vec::new(),
        )
    }

    #[test]
    fn sync_propagates_ppaf_flag_from_account_properties() {
        let store = build_store_for_ppaf_tests();
        let default_endpoint = store.default_endpoint().clone();

        // Initial state -- no sync yet, flag is false from `PartitionEndpointState::default()`.
        assert!(
            !store
                .snapshot()
                .partitions
                .per_partition_automatic_failover_enabled
        );

        // First sync with PPAF=true must flip the flag on.
        store.sync_account_properties(
            Arc::new(test_payload_with_ppaf(true, "etag-on")),
            &default_endpoint,
        );
        assert!(
            store
                .snapshot()
                .partitions
                .per_partition_automatic_failover_enabled,
            "PPAF flag should be true after a sync with enablePerPartitionFailoverBehavior=true"
        );

        // Second sync with PPAF=false must flip it back off.
        store.sync_account_properties(
            Arc::new(test_payload_with_ppaf(false, "etag-off")),
            &default_endpoint,
        );
        assert!(
            !store
                .snapshot()
                .partitions
                .per_partition_automatic_failover_enabled,
            "PPAF flag should be false after a sync with enablePerPartitionFailoverBehavior=false"
        );
    }

    #[test]
    fn disabling_ppaf_clears_failover_overrides() {
        use crate::driver::routing::partition_endpoint_state::{
            HealthStatus, PartitionFailoverEntry,
        };
        use crate::driver::routing::partition_key_range_id::PartitionKeyRangeId;
        use std::collections::HashSet;

        let store = build_store_for_ppaf_tests();
        let default_endpoint = store.default_endpoint().clone();

        // Enable PPAF first so the override would actually be honored by the
        // routing eligibility gate (the test is about transition cleanup, not
        // about whether the override applies while PPAF is on).
        store.sync_account_properties(
            Arc::new(test_payload_with_ppaf(true, "etag-on")),
            &default_endpoint,
        );

        // Seed a fake PPAF override entry. This mirrors what
        // `mark_partition_unavailable` would install during a real
        // single-master write failover.
        let endpoint_a = CosmosEndpoint::regional(
            "eastus".into(),
            url::Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let endpoint_b = CosmosEndpoint::regional(
            "westus".into(),
            url::Url::parse("https://test-westus.documents.azure.com:443/").unwrap(),
        );
        let pkrange_id: PartitionKeyRangeId = "0".to_string().into();
        let entry = PartitionFailoverEntry {
            current_endpoint: endpoint_b.clone(),
            first_failed_endpoint: endpoint_a.clone(),
            failed_endpoints: HashSet::from([endpoint_a]),
            read_failure_count: 0,
            write_failure_count: 1,
            first_failure_time: Instant::now(),
            last_failure_time: Instant::now(),
            health_status: HealthStatus::Unhealthy,
            failback_jitter: Duration::ZERO,
        };

        store.apply_partition(|current| {
            let mut next = current.clone();
            next.failover_overrides
                .insert(pkrange_id.clone(), entry.clone());
            next
        });
        assert_eq!(
            store.snapshot().partitions.failover_overrides.len(),
            1,
            "test setup: failover override was not installed"
        );

        // Server flips PPAF off -- the next refresh must drop the stale override
        // entry so it cannot silently re-apply when PPAF is re-enabled.
        store.sync_account_properties(
            Arc::new(test_payload_with_ppaf(false, "etag-off")),
            &default_endpoint,
        );

        let snapshot = store.snapshot();
        assert!(
            !snapshot.partitions.per_partition_automatic_failover_enabled,
            "PPAF should be off after sync_account_properties with false"
        );
        assert!(
            snapshot.partitions.failover_overrides.is_empty(),
            "failover_overrides should be cleared when PPAF flips from true to false; \
             leaving stale entries lets old failovers re-apply silently on re-enable"
        );
    }

    #[test]
    fn re_enabling_ppaf_does_not_clear_overrides_that_were_installed_after_re_enable() {
        // The clear path fires on every flag-flip *edge*, not on every refresh.
        // A same-state sync (true -> true with a new etag) is not an edge, so
        // overrides installed during a stable enabled window must survive.
        // (Whether the clear fires on false->true is exercised separately by
        // `enabling_ppaf_clears_stale_failover_overrides_from_prior_window`.)
        use crate::driver::routing::partition_endpoint_state::{
            HealthStatus, PartitionFailoverEntry,
        };
        use crate::driver::routing::partition_key_range_id::PartitionKeyRangeId;
        use std::collections::HashSet;

        let store = build_store_for_ppaf_tests();
        let default_endpoint = store.default_endpoint().clone();

        // false -> true: clear path must not fire (no prior overrides anyway).
        store.sync_account_properties(
            Arc::new(test_payload_with_ppaf(true, "etag-on-1")),
            &default_endpoint,
        );

        let endpoint_a = CosmosEndpoint::regional(
            "eastus".into(),
            url::Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let endpoint_b = CosmosEndpoint::regional(
            "westus".into(),
            url::Url::parse("https://test-westus.documents.azure.com:443/").unwrap(),
        );
        let pkrange_id: PartitionKeyRangeId = "0".to_string().into();
        let entry = PartitionFailoverEntry {
            current_endpoint: endpoint_b.clone(),
            first_failed_endpoint: endpoint_a.clone(),
            failed_endpoints: HashSet::from([endpoint_a]),
            read_failure_count: 0,
            write_failure_count: 1,
            first_failure_time: Instant::now(),
            last_failure_time: Instant::now(),
            health_status: HealthStatus::Unhealthy,
            failback_jitter: Duration::ZERO,
        };
        store.apply_partition(|current| {
            let mut next = current.clone();
            next.failover_overrides
                .insert(pkrange_id.clone(), entry.clone());
            next
        });

        // true -> true with a new etag (still enabled): clear path must not fire.
        store.sync_account_properties(
            Arc::new(test_payload_with_ppaf(true, "etag-on-2")),
            &default_endpoint,
        );

        assert_eq!(
            store.snapshot().partitions.failover_overrides.len(),
            1,
            "overrides installed while PPAF is on must not be cleared by a same-state sync"
        );
    }

    #[test]
    fn enabling_ppaf_clears_stale_failover_overrides_from_prior_window() {
        // Covers the false->true edge: an entry left over from a *prior*
        // enabled window must be dropped on re-enable so it cannot silently
        // re-apply against a routing topology the operator never re-confirmed.
        use crate::driver::routing::partition_endpoint_state::{
            HealthStatus, PartitionFailoverEntry,
        };
        use crate::driver::routing::partition_key_range_id::PartitionKeyRangeId;
        use std::collections::HashSet;

        let store = build_store_for_ppaf_tests();
        let default_endpoint = store.default_endpoint().clone();

        // Land in a known PPAF-off baseline first.
        store.sync_account_properties(
            Arc::new(test_payload_with_ppaf(false, "etag-off")),
            &default_endpoint,
        );

        // Simulate a stale entry that survived from a prior enabled window
        // (e.g. before the clear-on-disable fix shipped, or because state was
        // mutated through a different code path).
        let endpoint_a = CosmosEndpoint::regional(
            "eastus".into(),
            url::Url::parse("https://test-eastus.documents.azure.com:443/").unwrap(),
        );
        let endpoint_b = CosmosEndpoint::regional(
            "westus".into(),
            url::Url::parse("https://test-westus.documents.azure.com:443/").unwrap(),
        );
        let pkrange_id: PartitionKeyRangeId = "0".to_string().into();
        let stale_entry = PartitionFailoverEntry {
            current_endpoint: endpoint_b.clone(),
            first_failed_endpoint: endpoint_a.clone(),
            failed_endpoints: HashSet::from([endpoint_a]),
            read_failure_count: 0,
            write_failure_count: 1,
            first_failure_time: Instant::now(),
            last_failure_time: Instant::now(),
            health_status: HealthStatus::Unhealthy,
            failback_jitter: Duration::ZERO,
        };
        store.apply_partition(|current| {
            let mut next = current.clone();
            next.failover_overrides
                .insert(pkrange_id.clone(), stale_entry.clone());
            next
        });
        assert_eq!(
            store.snapshot().partitions.failover_overrides.len(),
            1,
            "test setup: stale override was not installed"
        );

        // Server flips PPAF on -- false->true edge must drop the stale entry.
        store.sync_account_properties(
            Arc::new(test_payload_with_ppaf(true, "etag-on")),
            &default_endpoint,
        );

        let snapshot = store.snapshot();
        assert!(
            snapshot.partitions.per_partition_automatic_failover_enabled,
            "PPAF should be on after sync with enablePerPartitionFailoverBehavior=true"
        );
        assert!(
            snapshot.partitions.failover_overrides.is_empty(),
            "stale failover_overrides from a prior enabled window must be cleared \
             on re-enable so they cannot silently re-apply against a routing \
             topology the operator never re-confirmed"
        );
    }
}
