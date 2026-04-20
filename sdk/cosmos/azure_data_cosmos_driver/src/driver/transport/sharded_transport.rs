// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cSpell:ignore pointee

//! HTTP/2 transport sharding for gateway endpoints.

use std::{
    collections::HashMap,
    fmt,
    sync::{
        atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::Instant,
};

use arc_swap::ArcSwap;

use azure_core::error::ErrorKind;

use super::cosmos_transport_client::{HttpRequest, HttpResponse, TransportClient, TransportError};
#[cfg(any(feature = "tokio", test))]
use std::time::Duration;
#[cfg(any(feature = "tokio", test))]
use tracing::debug;
use tracing::trace;
use url::Url;

use crate::diagnostics::TransportShardDiagnostics;
use crate::options::ConnectionPoolOptions;

#[cfg(feature = "tokio")]
use super::background_task_manager::BackgroundTaskManager;
use super::http_client_factory::{HttpClientConfig, HttpClientFactory};

pub(crate) struct TransportDispatch {
    pub(crate) result: Result<HttpResponse, TransportError>,
    pub(crate) shard_id: Option<u64>,
    pub(crate) shard_diagnostics: Option<TransportShardDiagnostics>,
}

#[derive(Clone)]
pub(crate) struct ShardedHttpTransport {
    pools: Arc<Mutex<HashMap<EndpointKey, Arc<EndpointShardPool>>>>,
    client_factory: Arc<dyn HttpClientFactory>,
    connection_pool: ConnectionPoolOptions,
    client_config: HttpClientConfig,
    #[cfg(feature = "tokio")]
    background_tasks: Arc<BackgroundTaskManager>,
}

impl ShardedHttpTransport {
    pub(crate) fn new(
        connection_pool: ConnectionPoolOptions,
        client_factory: Arc<dyn HttpClientFactory>,
        client_config: HttpClientConfig,
    ) -> Self {
        let transport = Self {
            pools: Arc::new(Mutex::new(HashMap::new())),
            client_factory,
            connection_pool,
            client_config,
            #[cfg(feature = "tokio")]
            background_tasks: Arc::new(BackgroundTaskManager::new()),
        };

        #[cfg(feature = "tokio")]
        transport.spawn_health_sweep();

        transport
    }

    pub(crate) async fn send(
        &self,
        request: &HttpRequest,
        excluded_shard_id: Option<u64>,
        endpoint_key: &EndpointKey,
        preferred_shard_id: Option<u64>,
    ) -> TransportDispatch {
        let pool = match self.get_or_create_pool(endpoint_key.clone()) {
            Ok(pool) => pool,
            Err(error) => {
                return TransportDispatch {
                    result: Err(TransportError::new(
                        error,
                        crate::diagnostics::RequestSentStatus::NotSent,
                    )),
                    shard_id: None,
                    shard_diagnostics: None,
                };
            }
        };

        let shard = match pool.select_shard(excluded_shard_id, preferred_shard_id) {
            Ok(shard) => shard,
            Err(error) => {
                return TransportDispatch {
                    result: Err(TransportError::new(
                        error,
                        crate::diagnostics::RequestSentStatus::NotSent,
                    )),
                    shard_id: None,
                    shard_diagnostics: None,
                };
            }
        };

        let shard_id = shard.id;
        let guard = shard.start_request();
        let result = shard.client.send(request).await;
        guard.finish(&result);
        let shard_diagnostics = Some(shard.transport_diagnostics());

        TransportDispatch {
            result,
            shard_id: Some(shard_id),
            shard_diagnostics,
        }
    }

    pub(crate) fn can_retry_on_different_shard(
        &self,
        excluded_shard_id: u64,
        endpoint_key: &EndpointKey,
    ) -> bool {
        let pool = {
            // Safe to ignore poisoning: the critical section only performs
            // a HashMap::get + Arc::clone which cannot panic.
            let pools = self.pools.lock().unwrap_or_else(|e| e.into_inner());
            pools.get(endpoint_key).cloned()
        };
        pool.is_some_and(|pool| pool.can_select_different_shard(excluded_shard_id))
    }

    /// Returns the ID of the shard that would be selected for the given request,
    /// without dispatching the request. Returns `None` if the pool does not exist
    /// or shard selection fails.
    pub(crate) fn pre_select_shard_id(
        &self,
        excluded_shard_id: Option<u64>,
        endpoint_key: &EndpointKey,
    ) -> Option<u64> {
        let pool = {
            // Safe to ignore poisoning: the critical section only performs
            // a HashMap::get + Arc::clone which cannot panic.
            let pools = self.pools.lock().unwrap_or_else(|e| e.into_inner());
            pools.get(endpoint_key).cloned()
        };
        // The outer pools mutex is released before calling select_shard,
        // which acquires the inner shards RwLock. This avoids blocking
        // get_or_create_pool for other endpoints during shard selection.
        pool.and_then(|pool| pool.select_shard(excluded_shard_id, None).ok())
            .map(|shard| shard.id)
    }

    fn get_or_create_pool(
        &self,
        endpoint_key: EndpointKey,
    ) -> azure_core::Result<Arc<EndpointShardPool>> {
        // Safe to ignore poisoning: the critical section only performs
        // HashMap::get/insert + Arc::clone which cannot panic.
        let mut pools = self.pools.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(pool) = pools.get(&endpoint_key) {
            return Ok(pool.clone());
        }

        let pool = Arc::new(EndpointShardPool::new(
            endpoint_key.clone(),
            self.connection_pool.clone(),
            self.client_factory.clone(),
            self.client_config,
        )?);
        pools.insert(endpoint_key, pool.clone());
        Ok(pool)
    }

    #[cfg(feature = "tokio")]
    fn spawn_health_sweep(&self) {
        if tokio::runtime::Handle::try_current().is_err() {
            return;
        }

        let interval = self.connection_pool.http2_health_check_interval();
        let pools = Arc::clone(&self.pools);

        self.background_tasks.spawn(async move {
            let mut ticker = tokio::time::interval(interval);
            ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            loop {
                ticker.tick().await;

                let snapshot = pools
                    .lock()
                    // Safe to ignore poisoning: the critical section only
                    // clones Arc values from the HashMap.
                    .unwrap_or_else(|e| e.into_inner())
                    .values()
                    .cloned()
                    .collect::<Vec<_>>();

                for pool in snapshot {
                    if let Err(error) = pool.run_health_sweep() {
                        debug!(endpoint = %pool.endpoint.0, %error, "http2 shard health sweep failed");
                    }
                }
            }
        });
    }
}

impl fmt::Debug for ShardedHttpTransport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pool_count = self.pools.lock().map(|p| p.len()).unwrap_or_default();
        f.debug_struct("ShardedHttpTransport")
            .field("pool_count", &pool_count)
            .field(
                "max_streams_per_client",
                &self.connection_pool.max_http2_streams_per_client(),
            )
            .field(
                "max_connections_per_endpoint",
                &self.connection_pool.max_http2_connections_per_endpoint(),
            )
            .finish_non_exhaustive()
    }
}

/// Key used to look up the connection shard pool for an endpoint.
///
/// The inner `Arc<str>` makes cloning cheap — it's an atomic reference count
/// increment with no heap allocation. Endpoints are created once at startup or
/// when the account metadata changes, so the underlying string is shared across
/// all operations routed to the same host:port.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct EndpointKey(Arc<str>);

impl TryFrom<&Url> for EndpointKey {
    type Error = azure_core::Error;

    fn try_from(url: &Url) -> azure_core::Result<Self> {
        let host = url.host_str().ok_or_else(|| {
            azure_core::Error::with_message(
                ErrorKind::DataConversion,
                format!("request URL is missing a host: {url}"),
            )
        })?;
        let port = url.port_or_known_default().ok_or_else(|| {
            azure_core::Error::with_message(
                ErrorKind::DataConversion,
                format!("request URL is missing a known port: {url}"),
            )
        })?;
        Ok(Self(Arc::from(format!("{host}:{port}").as_str())))
    }
}

struct EndpointShardPool {
    endpoint: EndpointKey,
    connection_pool: ConnectionPoolOptions,
    client_factory: Arc<dyn HttpClientFactory>,
    base_client_config: HttpClientConfig,
    /// Lock-free shard list. Reads via `ArcSwap::load()` incur no
    /// reader-counter contention. Writes build a new `Vec` and swap
    /// the pointer atomically.
    shards: ArcSwap<Vec<Arc<ClientShard>>>,
    /// Serializes write operations (try_create_shard, health sweep)
    /// to prevent concurrent mutations from racing. Readers never
    /// acquire this lock — they use `shards.load()` directly.
    write_lock: Mutex<()>,
    next_shard_id: AtomicU64,
}

impl EndpointShardPool {
    fn new(
        endpoint: EndpointKey,
        connection_pool: ConnectionPoolOptions,
        client_factory: Arc<dyn HttpClientFactory>,
        base_client_config: HttpClientConfig,
    ) -> azure_core::Result<Self> {
        let pool = Self {
            endpoint,
            connection_pool,
            client_factory,
            base_client_config,
            shards: ArcSwap::from_pointee(Vec::new()),
            write_lock: Mutex::new(()),
            next_shard_id: AtomicU64::new(1),
        };

        // Best-effort eager shard creation. If a transient TLS/DNS issue
        // prevents building the initial shard(s), the pool starts empty and
        // `select_shard` → `try_create_shard` will retry on the next request.
        // The background health sweep also backfills to min_clients.
        {
            let mut initial = Vec::new();
            while initial.len() < pool.connection_pool.min_http2_connections_per_endpoint() {
                match pool.build_shard() {
                    Ok(shard) => initial.push(Arc::new(shard)),
                    Err(error) => {
                        tracing::debug!(
                            endpoint = %pool.endpoint.0,
                            error = %error,
                            created = initial.len(),
                            target = pool.connection_pool.min_http2_connections_per_endpoint(),
                            "Initial shard creation failed; pool will backfill lazily"
                        );
                        break;
                    }
                }
            }
            pool.shards.store(Arc::new(initial));
        }

        Ok(pool)
    }

    fn select_shard(
        &self,
        excluded_shard_id: Option<u64>,
        preferred_shard_id: Option<u64>,
    ) -> azure_core::Result<Arc<ClientShard>> {
        let max_streams = self.connection_pool.max_http2_streams_per_client();
        let min_connections = self.connection_pool.min_http2_connections_per_endpoint();

        // Fast path: lock-free read via ArcSwap::load().
        {
            let shards = self.shards.load();
            if let Some(shard) = select_from_shards(
                &shards,
                excluded_shard_id,
                preferred_shard_id,
                max_streams,
                min_connections,
            ) {
                return Ok(shard);
            }
        }

        // All active shards at capacity (or pool is empty) — try creating a new one.
        if let Some(shard) = self.try_create_shard()? {
            return Ok(shard);
        }

        // Fallback: least-loaded of all selectable shards (over-capacity).
        let shards = self.shards.load();
        shards
            .iter()
            .filter(|s| s.is_selectable(excluded_shard_id))
            .min_by_key(|s| s.inflight())
            .cloned()
            .ok_or_else(|| {
                azure_core::Error::with_message(
                    ErrorKind::Other,
                    format!(
                        "endpoint shard pool {} has no available shards",
                        self.endpoint.0
                    ),
                )
            })
    }

    fn can_select_different_shard(&self, excluded_shard_id: u64) -> bool {
        let shards = self.shards.load();
        shards
            .iter()
            .any(|shard| shard.is_selectable(Some(excluded_shard_id)))
            || shards.len() < self.connection_pool.max_http2_connections_per_endpoint()
    }

    /// Creates a new shard if below the max limit. Serialized via `write_lock`
    /// to prevent concurrent scale-up from exceeding `max_connections`.
    fn try_create_shard(&self) -> azure_core::Result<Option<Arc<ClientShard>>> {
        // Safe to ignore poisoning: the critical section only reads
        // ArcSwap, builds a shard, and stores a new Vec — none of
        // which panic.
        let _guard = self.write_lock.lock().unwrap_or_else(|e| e.into_inner());
        let current = self.shards.load();
        if current.len() >= self.connection_pool.max_http2_connections_per_endpoint() {
            return Ok(None);
        }

        let shard = Arc::new(self.build_shard()?);
        trace!(
            endpoint = %self.endpoint.0,
            shard_id = shard.id,
            pool_size = current.len() + 1,
            "Created new shard (scale-up from request path)"
        );
        let mut new_vec = (**current).clone();
        new_vec.push(shard.clone());
        self.shards.store(Arc::new(new_vec));
        Ok(Some(shard))
    }

    fn build_shard(&self) -> azure_core::Result<ClientShard> {
        let client_config = self.base_client_config;

        let client = self
            .client_factory
            .build(&self.connection_pool, client_config)?;

        Ok(ClientShard::new(
            self.next_shard_id.fetch_add(1, Ordering::Relaxed),
            client,
        ))
    }
}

#[cfg(any(feature = "tokio", test))]
impl EndpointShardPool {
    fn run_health_sweep(&self) -> azure_core::Result<()> {
        let now = Instant::now();
        let threshold = self.connection_pool.http2_consecutive_failure_threshold();
        let grace = self.connection_pool.http2_eviction_grace_period();
        let idle_timeout = self.connection_pool.idle_http2_client_timeout();
        let min_clients = self.connection_pool.min_http2_connections_per_endpoint();
        let max_clients = self.connection_pool.max_http2_connections_per_endpoint();

        // Phase 1: evaluate, mark, and compute a new shard list.
        // Serialized via write_lock to prevent concurrent mutations.
        let shards_needed = {
            // Safe to ignore poisoning: the critical section only reads
            // snapshots and swaps the ArcSwap — no panicking operations.
            let _guard = self.write_lock.lock().unwrap_or_else(|e| e.into_inner());
            let current = self.shards.load();

            if current.is_empty() {
                min_clients
            } else {
                let snapshots = current
                    .iter()
                    .map(|shard| shard.snapshot())
                    .collect::<Vec<_>>();
                let probe_candidate = pick_probe_candidate(&snapshots, threshold, grace, now);
                let has_healthy_peer = snapshots.iter().any(|snapshot| {
                    !snapshot.marked_for_eviction
                        && (snapshot.consecutive_failures < threshold
                            || snapshot.has_recent_success(now, grace))
                });

                let mut needs_probe_replacement = false;
                for snapshot in &snapshots {
                    let should_mark = match probe_candidate {
                        Some(probe_id) => snapshot.id == probe_id,
                        None => {
                            has_healthy_peer
                                && snapshot.consecutive_failures >= threshold
                                && !snapshot.marked_for_eviction
                                && !snapshot.has_recent_success(now, grace)
                        }
                    };

                    if should_mark {
                        if probe_candidate == Some(snapshot.id) {
                            needs_probe_replacement = true;
                        }
                        if let Some(shard) = current.iter().find(|s| s.id == snapshot.id) {
                            trace!(
                                endpoint = %self.endpoint.0,
                                shard_id = snapshot.id,
                                consecutive_failures = snapshot.consecutive_failures,
                                is_probe_candidate = probe_candidate == Some(snapshot.id),
                                "Marking shard for eviction"
                            );
                            shard.mark_for_eviction();
                        }
                    }
                }

                // Build the new shard list, removing evicted idle shards.
                let mut new_shards: Vec<Arc<ClientShard>> = current
                    .iter()
                    .filter(|shard| !(shard.is_marked_for_eviction() && shard.inflight() == 0))
                    .cloned()
                    .collect();

                // Reclaim idle overflow shards from the tail.
                while new_shards.len() > min_clients {
                    let should_remove = new_shards.last().is_some_and(|shard| {
                        !shard.is_marked_for_eviction() && shard.is_idle_for(now, idle_timeout)
                    });
                    if !should_remove {
                        break;
                    }
                    new_shards.pop();
                }

                // Calculate how many shards we need to build outside the lock.
                let mut needed = 0;
                if needs_probe_replacement && new_shards.len() < max_clients {
                    needed += 1;
                }
                needed += min_clients.saturating_sub(new_shards.len() + needed);

                // Swap the shard list atomically.
                if new_shards.len() != current.len() {
                    trace!(
                        endpoint = %self.endpoint.0,
                        previous_count = current.len(),
                        new_count = new_shards.len(),
                        backfill_needed = needed,
                        "Health sweep updated shard pool"
                    );
                }
                self.shards.store(Arc::new(new_shards));
                needed
            }
            // write_lock dropped here.
        };

        if shards_needed == 0 {
            return Ok(());
        }

        // Phase 2: build replacement shards outside the lock. This is the
        // expensive part (TCP connect, TLS handshake) and does not block
        // concurrent `select_shard` readers.
        let mut new_shards = Vec::with_capacity(shards_needed);
        for _ in 0..shards_needed {
            match self.build_shard() {
                Ok(shard) => new_shards.push(Arc::new(shard)),
                Err(error) => {
                    debug!(endpoint = %self.endpoint.0, %error, "shard build failed during health sweep");
                }
            }
        }

        if !new_shards.is_empty() {
            // Phase 3: re-acquire write lock and insert the new shards.
            let _guard = self.write_lock.lock().unwrap_or_else(|e| e.into_inner());
            let current = self.shards.load();
            let mut updated = (**current).clone();
            for new_shard in new_shards {
                if updated.len() < max_clients {
                    updated.push(new_shard);
                }
            }
            self.shards.store(Arc::new(updated));
        }

        Ok(())
    }
}

impl fmt::Debug for EndpointShardPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let shard_count = self.shards.load().len();
        f.debug_struct("EndpointShardPool")
            .field("endpoint", &self.endpoint)
            .field("shard_count", &shard_count)
            .finish_non_exhaustive()
    }
}

/// Sentinel value for `AtomicU64` timestamp fields meaning "no value" (like `None`).
const TIMESTAMP_NONE: u64 = u64::MAX;

/// Offset added to nanos values so that times before `creation_time` can be
/// represented as positive integers. With a 30-second pre-creation window,
/// `Instant::now() - 30s` still fits comfortably in a `u64`.
const TIMESTAMP_BIAS_NANOS: u64 = 30_000_000_000; // 30 seconds

struct ClientShard {
    id: u64,
    client: Arc<dyn TransportClient>,
    /// Monotonic base used for all timestamp offsets on this shard.
    creation_time: Instant,
    // -- Hot-path atomic counters (no Mutex needed) --
    inflight: AtomicU32,
    /// Lock-free eviction flag checked on the hot path (`select_shard`).
    marked_for_eviction: AtomicBool,
    /// Nanos since `creation_time` of the most recent request start.
    last_request_at_nanos: AtomicU64,
    /// Nanos since `creation_time` of the most recent successful response,
    /// or `TIMESTAMP_NONE` if no success has been recorded yet.
    last_success_at_nanos: AtomicU64,
    consecutive_failures: AtomicU32,
    total_requests: AtomicU64,
    total_failures: AtomicU64,
    /// Requests started but never finished (e.g., cancelled by a timeout race).
    total_cancellations: AtomicU64,
}

impl ClientShard {
    fn new(id: u64, client: Arc<dyn TransportClient>) -> Self {
        Self {
            id,
            client,
            creation_time: Instant::now(),
            inflight: AtomicU32::new(0),
            marked_for_eviction: AtomicBool::new(false),
            last_request_at_nanos: AtomicU64::new(TIMESTAMP_BIAS_NANOS),
            last_success_at_nanos: AtomicU64::new(TIMESTAMP_NONE),
            consecutive_failures: AtomicU32::new(0),
            total_requests: AtomicU64::new(0),
            total_failures: AtomicU64::new(0),
            total_cancellations: AtomicU64::new(0),
        }
    }

    /// Converts an `Instant` to a biased nanos offset from this shard's creation time.
    fn instant_to_nanos(&self, instant: Instant) -> u64 {
        if let Some(d) = instant.checked_duration_since(self.creation_time) {
            TIMESTAMP_BIAS_NANOS.saturating_add(d.as_nanos() as u64)
        } else {
            // instant is before creation_time — subtract the deficit from the bias.
            let deficit = self.creation_time.duration_since(instant).as_nanos() as u64;
            TIMESTAMP_BIAS_NANOS.saturating_sub(deficit)
        }
    }

    fn inflight(&self) -> u32 {
        self.inflight.load(Ordering::Relaxed)
    }

    /// Begins tracking an inflight request and returns an RAII guard.
    ///
    /// Fully lock-free: only atomic increments on `inflight` and `total_requests`,
    /// plus an atomic store for `last_request_at_nanos`.
    fn start_request(&self) -> InflightGuard<'_> {
        self.inflight.fetch_add(1, Ordering::Relaxed);
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        self.last_request_at_nanos
            .store(self.instant_to_nanos(Instant::now()), Ordering::Relaxed);
        InflightGuard {
            shard: self,
            finished: false,
        }
    }

    fn record_request_outcome(&self, result: &Result<HttpResponse, TransportError>) {
        self.inflight.fetch_sub(1, Ordering::Relaxed);
        let now_nanos = self.instant_to_nanos(Instant::now());
        self.last_request_at_nanos
            .store(now_nanos, Ordering::Relaxed);
        if result.is_ok() {
            self.last_success_at_nanos
                .store(now_nanos, Ordering::Relaxed);
            self.consecutive_failures.store(0, Ordering::Relaxed);
        } else {
            self.consecutive_failures.fetch_add(1, Ordering::Relaxed);
            self.total_failures.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn decrement_inflight(&self) {
        self.inflight.fetch_sub(1, Ordering::Relaxed);
        self.total_cancellations.fetch_add(1, Ordering::Relaxed);
    }
}

/// RAII guard that ensures the inflight counter on a [`ClientShard`] is always
/// decremented, even when an async future is cancelled mid-flight.
///
/// Call [`finish`](Self::finish) to record the request outcome (success/failure
/// state). If the guard is dropped without calling `finish` (e.g., the future
/// was cancelled by a timeout race), only the inflight counter is decremented —
/// no success/failure state change is recorded, which is the safest default.
struct InflightGuard<'a> {
    shard: &'a ClientShard,
    finished: bool,
}

impl<'a> InflightGuard<'a> {
    /// Records the request outcome and consumes the guard.
    ///
    /// This decrements the inflight counter and updates success/failure state.
    fn finish(mut self, result: &Result<HttpResponse, TransportError>) {
        self.finished = true;
        self.shard.record_request_outcome(result);
    }
}

impl Drop for InflightGuard<'_> {
    fn drop(&mut self) {
        if !self.finished {
            self.shard.decrement_inflight();
        }
    }
}

impl ClientShard {
    fn is_marked_for_eviction(&self) -> bool {
        self.marked_for_eviction.load(Ordering::Relaxed)
    }

    fn is_selectable(&self, excluded_shard_id: Option<u64>) -> bool {
        excluded_shard_id != Some(self.id) && !self.is_marked_for_eviction()
    }

    fn transport_diagnostics(&self) -> TransportShardDiagnostics {
        TransportShardDiagnostics::new(
            self.id,
            self.inflight(),
            self.consecutive_failures.load(Ordering::Relaxed),
            self.total_requests.load(Ordering::Relaxed),
            self.total_failures.load(Ordering::Relaxed),
            self.total_cancellations.load(Ordering::Relaxed),
            self.is_marked_for_eviction(),
        )
    }

    /// Bumps inflight for test setup (not cancellation-safe; use `start_request` in production).
    #[cfg(test)]
    fn record_request_start(&self) {
        self.inflight.fetch_add(1, Ordering::Relaxed);
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        self.last_request_at_nanos
            .store(self.instant_to_nanos(Instant::now()), Ordering::Relaxed);
    }

    /// Records outcome for test setup (not cancellation-safe; use `InflightGuard::finish` in production).
    #[cfg(test)]
    fn record_request_finish(&self, result: &Result<HttpResponse, TransportError>) {
        self.record_request_outcome(result);
    }

    /// Sets the `last_request_at` timestamp for testing.
    #[cfg(test)]
    fn set_last_request_at(&self, instant: Instant) {
        self.last_request_at_nanos
            .store(self.instant_to_nanos(instant), Ordering::Relaxed);
    }

    /// Sets the `last_success_at` timestamp for testing.
    #[cfg(test)]
    fn set_last_success_at(&self, instant: Option<Instant>) {
        match instant {
            Some(t) => self
                .last_success_at_nanos
                .store(self.instant_to_nanos(t), Ordering::Relaxed),
            None => self
                .last_success_at_nanos
                .store(TIMESTAMP_NONE, Ordering::Relaxed),
        }
    }

    /// Sets the consecutive failure counter for testing.
    #[cfg(test)]
    fn set_consecutive_failures(&self, count: u32) {
        self.consecutive_failures.store(count, Ordering::Relaxed);
    }
}

#[cfg(any(feature = "tokio", test))]
impl ClientShard {
    /// Converts a biased nanos offset to an `Instant` relative to this shard's creation time.
    fn nanos_to_instant(&self, biased_nanos: u64) -> Instant {
        if biased_nanos >= TIMESTAMP_BIAS_NANOS {
            self.creation_time + Duration::from_nanos(biased_nanos - TIMESTAMP_BIAS_NANOS)
        } else {
            // Time before creation_time: subtract the deficit.
            self.creation_time - Duration::from_nanos(TIMESTAMP_BIAS_NANOS - biased_nanos)
        }
    }

    fn snapshot(&self) -> ClientShardHealthSnapshot {
        let last_success_nanos = self.last_success_at_nanos.load(Ordering::Relaxed);
        ClientShardHealthSnapshot {
            id: self.id,
            inflight: self.inflight(),
            last_request_at: self
                .nanos_to_instant(self.last_request_at_nanos.load(Ordering::Relaxed)),
            last_success_at: if last_success_nanos == TIMESTAMP_NONE {
                None
            } else {
                Some(self.nanos_to_instant(last_success_nanos))
            },
            consecutive_failures: self.consecutive_failures.load(Ordering::Relaxed),
            total_requests: self.total_requests.load(Ordering::Relaxed),
            total_failures: self.total_failures.load(Ordering::Relaxed),
            marked_for_eviction: self.is_marked_for_eviction(),
        }
    }

    fn mark_for_eviction(&self) {
        self.marked_for_eviction.store(true, Ordering::Relaxed);
    }

    fn is_idle_for(&self, now: Instant, idle_timeout: Duration) -> bool {
        if self.inflight() != 0 {
            return false;
        }

        let last_request_at =
            self.nanos_to_instant(self.last_request_at_nanos.load(Ordering::Relaxed));
        now.duration_since(last_request_at) >= idle_timeout
    }
}

#[cfg(any(feature = "tokio", test))]
#[derive(Clone, Copy, Debug)]
struct ClientShardHealthSnapshot {
    id: u64,
    inflight: u32,
    last_request_at: Instant,
    last_success_at: Option<Instant>,
    consecutive_failures: u32,
    total_requests: u64,
    total_failures: u64,
    marked_for_eviction: bool,
}

#[cfg(any(feature = "tokio", test))]
impl ClientShardHealthSnapshot {
    fn has_recent_success(self, now: Instant, grace_period: Duration) -> bool {
        self.last_success_at
            .is_some_and(|last_success_at| now.duration_since(last_success_at) <= grace_period)
    }
}

/// Pure selection logic operating on a shard slice — no side effects.
///
/// Returns the best shard from `shards` that is selectable and under the
/// stream limit, preferring `preferred_shard_id` when available. Returns
/// `None` when all shards are at capacity (caller should try creating a
/// new shard or fall back to over-capacity selection).
fn select_from_shards(
    shards: &[Arc<ClientShard>],
    excluded_shard_id: Option<u64>,
    preferred_shard_id: Option<u64>,
    max_streams: u32,
    min_connections: usize,
) -> Option<Arc<ClientShard>> {
    if shards.is_empty() {
        return None;
    }

    // If a preferred shard was pre-selected (e.g. for timeout diagnostics
    // accuracy), reuse it when still selectable and under the stream limit.
    if let Some(preferred_id) = preferred_shard_id {
        if let Some(shard) = shards
            .iter()
            .find(|s| s.id == preferred_id && s.is_selectable(excluded_shard_id))
        {
            if shard.inflight() < max_streams {
                return Some(Arc::clone(shard));
            }
        }
    }

    let active_count = active_shard_count(shards, excluded_shard_id, max_streams, min_connections);

    // Try least-loaded with capacity among active shards
    shards
        .iter()
        .filter(|s| s.is_selectable(excluded_shard_id))
        .take(active_count)
        .filter(|s| s.inflight() < max_streams)
        .min_by_key(|s| s.inflight())
        .cloned()
}

/// Computes the number of active shards that should participate in selection.
///
/// Based on current inflight load, returns a count between `min_connections`
/// and the number of selectable shards.
fn active_shard_count(
    shards: &[Arc<ClientShard>],
    excluded_shard_id: Option<u64>,
    max_streams: u32,
    min_connections: usize,
) -> usize {
    let mut selectable_count = 0usize;
    let mut total_inflight = 0u32;

    for shard in shards {
        if shard.is_selectable(excluded_shard_id) {
            selectable_count += 1;
            total_inflight += shard.inflight();
        }
    }

    if selectable_count == 0 {
        return 0;
    }

    let needed = (total_inflight as usize + 1).div_ceil(max_streams as usize);
    needed.max(min_connections).min(selectable_count).max(1)
}

#[cfg(any(feature = "tokio", test))]
fn pick_probe_candidate(
    snapshots: &[ClientShardHealthSnapshot],
    threshold: u32,
    grace_period: Duration,
    now: Instant,
) -> Option<u64> {
    // If any eligible shard is healthy (below threshold or has a recent
    // success), there's no all-failing condition — no probe needed.
    let any_healthy = snapshots
        .iter()
        .filter(|s| !s.marked_for_eviction)
        .any(|snapshot| {
            snapshot.consecutive_failures < threshold
                || snapshot.has_recent_success(now, grace_period)
        });

    if any_healthy || !snapshots.iter().any(|s| !s.marked_for_eviction) {
        return None;
    }

    snapshots
        .iter()
        .filter(|s| !s.marked_for_eviction)
        .max_by_key(|snapshot| {
            (
                snapshot.consecutive_failures,
                std::cmp::Reverse(snapshot.last_success_at.unwrap_or(snapshot.last_request_at)),
                snapshot.total_failures,
                std::cmp::Reverse(snapshot.total_requests),
                snapshot.inflight,
            )
        })
        .map(|snapshot| snapshot.id)
}

impl fmt::Debug for ClientShard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClientShard")
            .field("id", &self.id)
            .field("inflight", &self.inflight())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::transport::cosmos_transport_client::{
        HttpRequest, HttpResponse, TransportError,
    };
    use async_trait::async_trait;

    #[derive(Debug, Default)]
    struct TrackingFactory {
        idle_ping_flags: Mutex<Vec<bool>>,
    }

    impl TrackingFactory {
        fn idle_ping_flags(&self) -> Vec<bool> {
            self.idle_ping_flags
                .lock()
                .expect("tracking lock poisoned")
                .clone()
        }
    }

    impl HttpClientFactory for TrackingFactory {
        fn build(
            &self,
            _connection_pool: &ConnectionPoolOptions,
            config: HttpClientConfig,
        ) -> azure_core::Result<Arc<dyn TransportClient>> {
            self.idle_ping_flags
                .lock()
                .expect("tracking lock poisoned")
                .push(config.http2_keep_alive_while_idle);
            Ok(Arc::new(NoopTransportClient))
        }
    }

    #[derive(Debug)]
    struct NoopTransportClient;

    #[async_trait]
    impl TransportClient for NoopTransportClient {
        async fn send(&self, _request: &HttpRequest) -> Result<HttpResponse, TransportError> {
            Err(TransportError::new(
                azure_core::Error::with_message(
                    ErrorKind::Other,
                    "noop client should not execute requests in shard unit tests",
                ),
                crate::diagnostics::RequestSentStatus::NotSent,
            ))
        }
    }

    fn connection_pool() -> ConnectionPoolOptions {
        ConnectionPoolOptions::builder()
            .with_max_http2_streams_per_client(2)
            .with_min_http2_connections_per_endpoint(1)
            .with_max_http2_connections_per_endpoint(4)
            .with_http2_consecutive_failure_threshold(2)
            .with_http2_eviction_grace_period(Duration::from_millis(100))
            .with_idle_http2_client_timeout(Duration::from_millis(1_000))
            .build()
            .unwrap()
    }

    fn client_config() -> HttpClientConfig {
        HttpClientConfig::dataplane_gateway(
            &connection_pool(),
            crate::diagnostics::TransportHttpVersion::Http2,
        )
    }

    #[test]
    fn endpoint_pool_scales_up_when_active_shards_are_full() {
        let factory = Arc::new(TrackingFactory::default());
        let pool = EndpointShardPool::new(
            EndpointKey(Arc::from("test.documents.azure.com:443")),
            connection_pool(),
            factory,
            client_config(),
        )
        .unwrap();

        let first = pool.select_shard(None, None).unwrap();
        first.record_request_start();
        first.record_request_start();

        let second = pool.select_shard(None, None).unwrap();

        assert_ne!(first.id, second.id);
        assert_eq!(pool.shards.load().len(), 2);
    }

    #[test]
    fn background_sweep_reclaims_idle_overflow_shards() {
        let factory = Arc::new(TrackingFactory::default());
        let pool = EndpointShardPool::new(
            EndpointKey(Arc::from("test.documents.azure.com:443")),
            connection_pool(),
            factory,
            client_config(),
        )
        .unwrap();

        let first = pool.select_shard(None, None).unwrap();
        first.record_request_start();
        first.record_request_start();
        let overflow = pool.select_shard(None, None).unwrap();
        overflow.record_request_start();
        overflow.record_request_finish(&Err(TransportError::new(
            azure_core::Error::with_message(ErrorKind::Other, "synthetic"),
            crate::diagnostics::RequestSentStatus::NotSent,
        )));

        overflow.set_last_request_at(Instant::now() - Duration::from_secs(5));

        first.record_request_finish(&Err(TransportError::new(
            azure_core::Error::with_message(ErrorKind::Other, "synthetic"),
            crate::diagnostics::RequestSentStatus::NotSent,
        )));
        first.record_request_finish(&Err(TransportError::new(
            azure_core::Error::with_message(ErrorKind::Other, "synthetic"),
            crate::diagnostics::RequestSentStatus::NotSent,
        )));

        first.set_consecutive_failures(0);
        first.set_last_success_at(Some(Instant::now()));

        pool.run_health_sweep().unwrap();

        let selected = pool.select_shard(None, None).unwrap();

        assert_eq!(selected.id, first.id);
        assert_eq!(pool.shards.load().len(), 1);
    }

    #[test]
    fn all_http2_shards_keep_idle_pings_enabled() {
        let factory = Arc::new(TrackingFactory::default());
        let pool = EndpointShardPool::new(
            EndpointKey(Arc::from("test.documents.azure.com:443")),
            connection_pool(),
            factory.clone(),
            client_config(),
        )
        .unwrap();

        let first = pool.select_shard(None, None).unwrap();
        first.record_request_start();
        first.record_request_start();
        let second = pool.select_shard(None, None).unwrap();
        second.record_request_start();
        second.record_request_start();
        let third = pool.select_shard(None, None).unwrap();

        assert_ne!(first.id, second.id);
        assert_ne!(second.id, third.id);
        assert_eq!(factory.idle_ping_flags(), vec![true, true, true]);
    }

    #[test]
    fn health_sweep_evicts_failed_shard_when_healthy_peer_exists() {
        let factory = Arc::new(TrackingFactory::default());
        let pool = EndpointShardPool::new(
            EndpointKey(Arc::from("test.documents.azure.com:443")),
            connection_pool(),
            factory,
            client_config(),
        )
        .unwrap();

        let first = pool.select_shard(None, None).unwrap();
        first.record_request_start();
        first.record_request_start();
        let second = pool.select_shard(None, None).unwrap();

        first.record_request_finish(&Err(TransportError::new(
            azure_core::Error::with_message(ErrorKind::Other, "synthetic"),
            crate::diagnostics::RequestSentStatus::NotSent,
        )));
        first.record_request_finish(&Err(TransportError::new(
            azure_core::Error::with_message(ErrorKind::Other, "synthetic"),
            crate::diagnostics::RequestSentStatus::NotSent,
        )));

        second.record_request_start();
        second.record_request_finish(&Err(TransportError::new(
            azure_core::Error::with_message(ErrorKind::Other, "synthetic"),
            crate::diagnostics::RequestSentStatus::NotSent,
        )));
        second.record_request_start();
        second.record_request_finish(&Err(TransportError::new(
            azure_core::Error::with_message(ErrorKind::Other, "synthetic"),
            crate::diagnostics::RequestSentStatus::NotSent,
        )));

        {
            first.set_consecutive_failures(0);
            first.set_last_success_at(Some(Instant::now()));
            first.set_last_request_at(Instant::now());
        }

        {
            second.set_last_success_at(Some(Instant::now() - Duration::from_secs(5)));
            second.set_last_request_at(Instant::now() - Duration::from_secs(5));
        }

        pool.run_health_sweep().unwrap();

        let snapshots = pool
            .shards
            .load()
            .iter()
            .map(|shard| shard.id)
            .collect::<Vec<_>>();
        assert_eq!(snapshots, vec![first.id]);
    }

    #[test]
    fn health_sweep_replaces_only_one_probe_when_all_shards_are_failing() {
        let factory = Arc::new(TrackingFactory::default());
        let pool = EndpointShardPool::new(
            EndpointKey(Arc::from("test.documents.azure.com:443")),
            connection_pool(),
            factory.clone(),
            client_config(),
        )
        .unwrap();

        let first = pool.select_shard(None, None).unwrap();
        first.record_request_start();
        first.record_request_start();
        let second = pool.select_shard(None, None).unwrap();

        first.record_request_finish(&Err(TransportError::new(
            azure_core::Error::with_message(ErrorKind::Other, "synthetic"),
            crate::diagnostics::RequestSentStatus::NotSent,
        )));
        first.record_request_finish(&Err(TransportError::new(
            azure_core::Error::with_message(ErrorKind::Other, "synthetic"),
            crate::diagnostics::RequestSentStatus::NotSent,
        )));

        for shard in [&first, &second] {
            shard.set_last_success_at(None);
            shard.set_last_request_at(Instant::now() - Duration::from_secs(5));
            shard.set_consecutive_failures(2);
        }

        first.inflight.store(0, Ordering::Relaxed);

        pool.run_health_sweep().unwrap();

        let shard_ids = pool
            .shards
            .load()
            .iter()
            .map(|shard| shard.id)
            .collect::<Vec<_>>();
        assert_eq!(shard_ids.len(), 2);
        assert!(shard_ids
            .iter()
            .any(|id| *id == first.id || *id == second.id));
        assert!(shard_ids.iter().any(|id| *id > second.id));
        assert_eq!(factory.idle_ping_flags(), vec![true, true, true]);
    }

    #[tokio::test(start_paused = true)]
    async fn background_health_sweep_fires_and_evicts_failed_shards() {
        // Create a transport with a short health check interval so the
        // background sweep fires quickly in paused-time mode.
        let health_interval = Duration::from_millis(100);
        let pool_opts = ConnectionPoolOptions::builder()
            .with_max_http2_streams_per_client(2)
            .with_min_http2_connections_per_endpoint(1)
            .with_max_http2_connections_per_endpoint(4)
            .with_http2_consecutive_failure_threshold(2)
            .with_http2_eviction_grace_period(Duration::from_millis(100))
            .with_idle_http2_client_timeout(Duration::from_millis(1_000))
            .with_http2_health_check_interval(health_interval)
            .build()
            .unwrap();

        let config = HttpClientConfig::dataplane_gateway(
            &pool_opts,
            crate::diagnostics::TransportHttpVersion::Http2,
        );
        let factory = Arc::new(TrackingFactory::default());

        let transport = ShardedHttpTransport::new(pool_opts.clone(), factory.clone(), config);

        // Create a pool and force a shard above the failure threshold.
        let endpoint_key = EndpointKey(Arc::from("sweep-test.documents.azure.com:443"));
        let pool = transport.get_or_create_pool(endpoint_key.clone()).unwrap();

        // Fill the first shard so a second shard is created.
        let first = pool.select_shard(None, None).unwrap();
        first.record_request_start();
        first.record_request_start();
        let second = pool.select_shard(None, None).unwrap();

        // Mark the second shard with consecutive failures above threshold.
        for _ in 0..3 {
            second.record_request_start();
            second.record_request_finish(&Err(TransportError::new(
                azure_core::Error::with_message(ErrorKind::Other, "synthetic"),
                crate::diagnostics::RequestSentStatus::NotSent,
            )));
        }

        // Make second's last success old enough that it passes the grace period.
        second.set_last_success_at(None);
        second.set_last_request_at(Instant::now() - Duration::from_secs(5));

        // Ensure the first shard is healthy so eviction can proceed.
        first.record_request_finish(&Ok(HttpResponse {
            status: 200,
            headers: azure_core::http::headers::Headers::new(),
            body: Vec::new(),
        }));
        first.record_request_finish(&Ok(HttpResponse {
            status: 200,
            headers: azure_core::http::headers::Headers::new(),
            body: Vec::new(),
        }));

        let second_id = second.id;

        // Advance time past the health check interval so the background
        // sweep fires and evicts the failed shard.
        tokio::time::advance(health_interval * 3).await;
        tokio::task::yield_now().await;

        // Give the spawned task a chance to run.
        tokio::time::advance(health_interval).await;
        tokio::task::yield_now().await;

        let shard_ids: Vec<u64> = pool.shards.load().iter().map(|s| s.id).collect();

        // The failed shard should have been evicted and replaced.
        assert!(
            !shard_ids.contains(&second_id),
            "failed shard {second_id} should have been evicted by background sweep, remaining: {shard_ids:?}"
        );
    }
}
