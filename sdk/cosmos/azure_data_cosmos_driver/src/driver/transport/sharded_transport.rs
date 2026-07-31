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

        let reservation = match pool.select_shard(excluded_shard_id, preferred_shard_id) {
            Ok(reservation) => reservation,
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

        let shard_id = reservation.shard.id;
        let result = reservation.shard.client.send(request).await;
        let shard_diagnostics = Some(reservation.finish(&result));

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

    /// Returns the best-effort ID of the shard that would be selected.
    ///
    /// Preselection may create pool capacity but does not reserve a stream. The
    /// eventual dispatch may use a different shard if load changes first.
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
        // Release the outer pools mutex first. Pool-level preselection may
        // acquire its write lock and create capacity before returning an ID.
        pool.and_then(|pool| pool.pre_select_shard_id(excluded_shard_id).ok())
    }

    fn get_or_create_pool(
        &self,
        endpoint_key: EndpointKey,
    ) -> crate::error::Result<Arc<EndpointShardPool>> {
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
                "target_streams_per_client",
                &self.connection_pool.target_http2_streams_per_client(),
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
    type Error = crate::error::CosmosError;

    fn try_from(url: &Url) -> crate::error::Result<Self> {
        let host = url.host_str().ok_or_else(|| {
            crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_REQUEST_URL_MISSING_HOST)
                .with_message(format!("request URL is missing a host: {url}"))
                .build()
        })?;
        let port = url.port_or_known_default().ok_or_else(|| {
            crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_REQUEST_URL_MISSING_KNOWN_PORT)
                .with_message(format!("request URL is missing a known port: {url}"))
                .build()
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
    /// Serializes request-path scale-up and health-sweep mutations
    /// to prevent concurrent mutations from racing. Readers never
    /// acquire this lock — they use `shards.load()` directly.
    write_lock: Mutex<()>,
    next_shard_id: AtomicU64,
    #[cfg(test)]
    preselection_slow_path_barrier: Mutex<Option<Arc<std::sync::Barrier>>>,
}

impl EndpointShardPool {
    fn new(
        endpoint: EndpointKey,
        connection_pool: ConnectionPoolOptions,
        client_factory: Arc<dyn HttpClientFactory>,
        base_client_config: HttpClientConfig,
    ) -> crate::error::Result<Self> {
        let pool = Self {
            endpoint,
            connection_pool,
            client_factory,
            base_client_config,
            shards: ArcSwap::from_pointee(Vec::new()),
            write_lock: Mutex::new(()),
            next_shard_id: AtomicU64::new(1),
            #[cfg(test)]
            preselection_slow_path_barrier: Mutex::new(None),
        };

        // Best-effort eager shard creation. If a transient TLS/DNS issue
        // prevents building the initial shard(s), the pool starts empty and
        // request-path scale-up will retry on the next request.
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
    ) -> crate::error::Result<InflightGuard> {
        let max_streams = self.connection_pool.max_http2_streams_per_client();
        let target_streams = self.connection_pool.target_http2_streams_per_client();
        let min_connections = self.connection_pool.min_http2_connections_per_endpoint();

        // Fast path: make a bounded number of lock-free reservation attempts.
        {
            let shards = self.shards.load();
            if let Some(reservation) = try_reserve_preferred(
                &shards,
                excluded_shard_id,
                preferred_shard_id,
                target_streams,
            ) {
                return Ok(reservation);
            }

            // Below the connection cap, normal selection is bounded by the
            // desired `target_streams` occupancy (not the hard `max_streams`
            // cap) so load fans out across shards early instead of filling a
            // single shard all the way up before another is used.
            if let Some(reservation) =
                reserve_from_shards(&shards, excluded_shard_id, target_streams, min_connections)
            {
                return Ok(reservation);
            }

            if shards.len() >= self.connection_pool.max_http2_connections_per_endpoint() {
                return self.reserve_at_max_connections(&shards, excluded_shard_id, max_streams);
            }
        }

        // Slow path serializes scale-up, rechecks capacity after waiting for
        // the lock, and reserves a new shard before publishing it.
        let guard = self.write_lock.lock().unwrap_or_else(|e| e.into_inner());
        let current = self.shards.load();
        if let Some(reservation) = try_reserve_preferred(
            &current,
            excluded_shard_id,
            preferred_shard_id,
            target_streams,
        ) {
            return Ok(reservation);
        }
        if let Some(reservation) =
            reserve_from_shards(&current, excluded_shard_id, target_streams, min_connections)
        {
            return Ok(reservation);
        }

        if current.len() < self.connection_pool.max_http2_connections_per_endpoint() {
            // Runs under write_lock on the request path. HttpClientFactory::build
            // must remain non-blocking and must not perform network I/O.
            let shard = match self.build_shard() {
                Ok(shard) => Arc::new(shard),
                Err(error) => {
                    tracing::debug!(
                        endpoint = %self.endpoint.0,
                        %error,
                        "Shard scale-up failed; using existing hard-cap headroom"
                    );
                    drop(guard);
                    return match self.reserve_at_max_connections(
                        &current,
                        excluded_shard_id,
                        max_streams,
                    ) {
                        Ok(reservation) => Ok(reservation),
                        Err(_) => Err(error),
                    };
                }
            };
            let reservation = shard.reserve_initial();
            trace!(
                endpoint = %self.endpoint.0,
                shard_id = shard.id,
                pool_size = current.len() + 1,
                "Created and reserved new shard (scale-up from request path)"
            );
            let mut updated = (**current).clone();
            updated.push(shard);
            self.shards.store(Arc::new(updated));
            return Ok(reservation);
        }

        drop(guard);
        self.reserve_at_max_connections(&current, excluded_shard_id, max_streams)
    }

    fn reserve_at_max_connections(
        &self,
        shards: &[Arc<ClientShard>],
        excluded_shard_id: Option<u64>,
        max_streams: u32,
    ) -> crate::error::Result<InflightGuard> {
        // Max-connections fallback: no more shards can be created. Prefer an
        // existing selectable shard with room under the hard `max_streams`
        // cap — this may push a shard above `target_streams` but must never
        // exceed `max_streams`.
        let selectable_count = shards
            .iter()
            .filter(|shard| shard.is_selectable(excluded_shard_id))
            .count();
        if let Some(reservation) =
            reserve_least_loaded_shard(shards, excluded_shard_id, selectable_count, max_streams)
        {
            return Ok(reservation);
        }

        // Every selectable shard is already at the hard cap — total capacity
        // is genuinely exhausted. As a last-resort safety valve, dispatch
        // anyway rather than fail the request outright; this uncapped path
        // is not expected to trigger under normal target/max configurations.
        let shard = select_least_loaded_shard(shards, excluded_shard_id, selectable_count, None)
            .ok_or_else(|| {
                crate::error::CosmosError::builder()
                    .with_status(crate::models::CosmosStatus::TRANSPORT_GENERATED_503)
                    .with_message(format!(
                        "endpoint shard pool {} has no available shards",
                        self.endpoint.0
                    ))
                    .build()
            })?;
        Ok(shard.reserve_over_capacity())
    }

    fn pre_select_shard_id(&self, excluded_shard_id: Option<u64>) -> crate::error::Result<u64> {
        let max_streams = self.connection_pool.max_http2_streams_per_client();
        let target_streams = self.connection_pool.target_http2_streams_per_client();
        let min_connections = self.connection_pool.min_http2_connections_per_endpoint();

        {
            let shards = self.shards.load();
            if let Some(shard) = select_from_shards(
                &shards,
                excluded_shard_id,
                None,
                target_streams,
                min_connections,
            ) {
                return Ok(shard.id);
            }

            if shards.len() >= self.connection_pool.max_http2_connections_per_endpoint() {
                return self.preselect_at_max_connections(&shards, excluded_shard_id, max_streams);
            }
        }

        // Preselection is diagnostic-only. Unlike dispatch, a scale-up build
        // failure returns no hint rather than consuming existing hard-cap headroom.
        if let Some(shard) =
            self.select_or_create_shard(excluded_shard_id, target_streams, min_connections)?
        {
            return Ok(shard.id);
        }

        // Max-connections fallback mirrors `select_shard`: prefer a shard
        // with room under the hard `max_streams` cap before falling back to
        // the least-loaded shard unconditionally.
        let shards = self.shards.load();
        let selectable_count = shards
            .iter()
            .filter(|shard| shard.is_selectable(excluded_shard_id))
            .count();
        select_least_loaded_shard(
            &shards,
            excluded_shard_id,
            selectable_count,
            Some(max_streams),
        )
        .or_else(|| select_least_loaded_shard(&shards, excluded_shard_id, selectable_count, None))
        .map(|shard| shard.id)
        .ok_or_else(|| {
            crate::error::CosmosError::builder()
                .with_status(crate::models::CosmosStatus::TRANSPORT_GENERATED_503)
                .with_message(format!(
                    "endpoint shard pool {} has no available shards",
                    self.endpoint.0
                ))
                .build()
        })
    }

    fn preselect_at_max_connections(
        &self,
        shards: &[Arc<ClientShard>],
        excluded_shard_id: Option<u64>,
        max_streams: u32,
    ) -> crate::error::Result<u64> {
        let selectable_count = shards
            .iter()
            .filter(|shard| shard.is_selectable(excluded_shard_id))
            .count();
        select_least_loaded_shard(
            shards,
            excluded_shard_id,
            selectable_count,
            Some(max_streams),
        )
        .or_else(|| select_least_loaded_shard(shards, excluded_shard_id, selectable_count, None))
        .map(|shard| shard.id)
        .ok_or_else(|| {
            crate::error::CosmosError::builder()
                .with_status(crate::models::CosmosStatus::TRANSPORT_GENERATED_503)
                .with_message(format!(
                    "endpoint shard pool {} has no available shards",
                    self.endpoint.0
                ))
                .build()
        })
    }

    fn can_select_different_shard(&self, excluded_shard_id: u64) -> bool {
        let shards = self.shards.load();
        shards
            .iter()
            .any(|shard| shard.is_selectable(Some(excluded_shard_id)))
            || shards.len() < self.connection_pool.max_http2_connections_per_endpoint()
    }

    /// Rechecks selection under `write_lock` before creating a shard below the max limit.
    ///
    /// `desired_streams` is the occupancy threshold used to decide whether an
    /// existing shard still has room (typically `target_streams`, mirroring
    /// `select_shard`'s normal-path fan-out behavior).
    fn select_or_create_shard(
        &self,
        excluded_shard_id: Option<u64>,
        desired_streams: u32,
        min_connections: usize,
    ) -> crate::error::Result<Option<Arc<ClientShard>>> {
        #[cfg(test)]
        self.wait_for_preselection_slow_path();

        // Safe to ignore poisoning: the critical section only reads
        // ArcSwap, builds a shard, and stores a new Vec — none of
        // which panic.
        let _guard = self.write_lock.lock().unwrap_or_else(|e| e.into_inner());
        let current = self.shards.load();
        if let Some(shard) = select_from_shards(
            &current,
            excluded_shard_id,
            None,
            desired_streams,
            min_connections,
        ) {
            return Ok(Some(shard));
        }
        if current.len() >= self.connection_pool.max_http2_connections_per_endpoint() {
            return Ok(None);
        }

        // Runs under write_lock on the request path. HttpClientFactory::build
        // must remain non-blocking and must not perform network I/O.
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

    #[cfg(test)]
    fn wait_for_preselection_slow_path(&self) {
        let barrier = self
            .preselection_slow_path_barrier
            .lock()
            .expect("preselection barrier lock poisoned")
            .clone();
        if let Some(barrier) = barrier {
            barrier.wait();
        }
    }

    #[cfg(test)]
    fn set_preselection_slow_path_barrier(&self, barrier: Arc<std::sync::Barrier>) {
        *self
            .preselection_slow_path_barrier
            .lock()
            .expect("preselection barrier lock poisoned") = Some(barrier);
    }

    fn build_shard(&self) -> crate::error::Result<ClientShard> {
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
    fn run_health_sweep(&self) -> crate::error::Result<()> {
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

        // Phase 2: build replacement shards outside the lock defensively. The
        // current factory is non-blocking, but future implementations must not
        // extend the health-sweep critical section.
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

    fn try_reserve(self: &Arc<Self>, max_streams: u32) -> Option<InflightGuard> {
        let inflight = self.inflight.load(Ordering::Relaxed);
        if inflight >= max_streams {
            return None;
        }
        self.inflight
            .compare_exchange(inflight, inflight + 1, Ordering::Relaxed, Ordering::Relaxed)
            .ok()?;
        self.record_reservation();
        Some(InflightGuard::new(Arc::clone(self)))
    }

    fn reserve_initial(self: &Arc<Self>) -> InflightGuard {
        let previous = self.inflight.fetch_add(1, Ordering::Relaxed);
        debug_assert_eq!(previous, 0, "new shards must be unpublished and idle");
        self.record_reservation();
        InflightGuard::new(Arc::clone(self))
    }

    fn reserve_over_capacity(self: &Arc<Self>) -> InflightGuard {
        self.inflight.fetch_add(1, Ordering::Relaxed);
        self.record_reservation();
        InflightGuard::new(Arc::clone(self))
    }

    fn record_reservation(&self) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        self.last_request_at_nanos
            .store(self.instant_to_nanos(Instant::now()), Ordering::Relaxed);
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
struct InflightGuard {
    shard: Arc<ClientShard>,
    finished: bool,
}

impl InflightGuard {
    fn new(shard: Arc<ClientShard>) -> Self {
        Self {
            shard,
            finished: false,
        }
    }

    /// Records the request outcome and consumes the guard.
    ///
    /// This decrements the inflight counter and updates success/failure state.
    fn finish(
        mut self,
        result: &Result<HttpResponse, TransportError>,
    ) -> TransportShardDiagnostics {
        self.finished = true;
        self.shard.record_request_outcome(result);
        self.shard.transport_diagnostics()
    }
}

impl Drop for InflightGuard {
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

    /// Bumps inflight for test setup (not cancellation-safe; reserve through the pool in production).
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
/// Returns the best shard from `shards` that is selectable and under
/// `desired_streams` occupancy, preferring `preferred_shard_id` when
/// available. Returns `None` when all active shards are at or above
/// `desired_streams` (caller should try creating a new shard or fall back to
/// over-capacity selection).
fn select_from_shards(
    shards: &[Arc<ClientShard>],
    excluded_shard_id: Option<u64>,
    preferred_shard_id: Option<u64>,
    desired_streams: u32,
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
            if shard.inflight() < desired_streams {
                return Some(Arc::clone(shard));
            }
        }
    }

    let active_count =
        active_shard_count(shards, excluded_shard_id, desired_streams, min_connections);

    select_least_loaded_shard(
        shards,
        excluded_shard_id,
        active_count,
        Some(desired_streams),
    )
}

fn reserve_from_shards(
    shards: &[Arc<ClientShard>],
    excluded_shard_id: Option<u64>,
    desired_streams: u32,
    min_connections: usize,
) -> Option<InflightGuard> {
    let active_count =
        active_shard_count(shards, excluded_shard_id, desired_streams, min_connections);
    reserve_least_loaded_shard(shards, excluded_shard_id, active_count, desired_streams)
}

fn try_reserve_preferred(
    shards: &[Arc<ClientShard>],
    excluded_shard_id: Option<u64>,
    preferred_shard_id: Option<u64>,
    stream_limit: u32,
) -> Option<InflightGuard> {
    selectable_preferred_shard(shards, excluded_shard_id, preferred_shard_id)
        .and_then(|shard| shard.try_reserve(stream_limit))
}

fn selectable_preferred_shard(
    shards: &[Arc<ClientShard>],
    excluded_shard_id: Option<u64>,
    preferred_shard_id: Option<u64>,
) -> Option<&Arc<ClientShard>> {
    let preferred_id = preferred_shard_id?;
    shards
        .iter()
        .find(|shard| shard.id == preferred_id && shard.is_selectable(excluded_shard_id))
}

fn select_least_loaded_shard(
    shards: &[Arc<ClientShard>],
    excluded_shard_id: Option<u64>,
    candidate_count: usize,
    max_streams: Option<u32>,
) -> Option<Arc<ClientShard>> {
    let mut selected: Option<(Arc<ClientShard>, u32)> = None;
    for shard in shards
        .iter()
        .filter(|shard| shard.is_selectable(excluded_shard_id))
        .take(candidate_count)
    {
        let inflight = shard.inflight();
        if max_streams.is_some_and(|max_streams| inflight >= max_streams) {
            continue;
        }
        if selected
            .as_ref()
            .is_none_or(|(_, selected_inflight)| inflight < *selected_inflight)
        {
            selected = Some((Arc::clone(shard), inflight));
        }
    }

    selected.map(|(shard, _)| shard)
}

fn reserve_least_loaded_shard(
    shards: &[Arc<ClientShard>],
    excluded_shard_id: Option<u64>,
    candidate_count: usize,
    max_streams: u32,
) -> Option<InflightGuard> {
    let selected = select_least_loaded_shard(
        shards,
        excluded_shard_id,
        candidate_count,
        Some(max_streams),
    )?;
    if let Some(reservation) = selected.try_reserve(max_streams) {
        return Some(reservation);
    }

    for shard in shards
        .iter()
        .filter(|shard| shard.is_selectable(excluded_shard_id))
        .take(candidate_count)
    {
        if shard.id == selected.id {
            continue;
        }
        if let Some(reservation) = shard.try_reserve(max_streams) {
            return Some(reservation);
        }
    }

    None
}

/// Computes the number of active shards that should participate in selection.
///
/// Based on current inflight load relative to the `desired_streams` occupancy
/// target, returns a count between `min_connections` and the number of
/// selectable shards. Using `desired_streams` (rather than the hard
/// `max_streams` cap) as the divisor here is what drives early fan-out:
/// the active window grows once shards approach their desired occupancy,
/// rather than only once they hit the hard cap.
fn active_shard_count(
    shards: &[Arc<ClientShard>],
    excluded_shard_id: Option<u64>,
    desired_streams: u32,
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

    let needed = (total_inflight as usize + 1).div_ceil(desired_streams as usize);
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
    use std::sync::{mpsc, Barrier};

    fn synthetic_transport_error() -> TransportError {
        TransportError::new(
            crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message("synthetic")
                .build(),
            crate::diagnostics::RequestSentStatus::NotSent,
        )
    }

    fn successful_response() -> HttpResponse {
        HttpResponse {
            status: 200,
            headers: azure_core::http::headers::Headers::new(),
            body: Vec::new(),
        }
    }

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
        ) -> crate::error::Result<Arc<dyn TransportClient>> {
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
                crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::new(
                        azure_core::http::StatusCode::BadRequest,
                    ))
                    .with_message("noop client should not execute requests in shard unit tests")
                    .build(),
                crate::diagnostics::RequestSentStatus::NotSent,
            ))
        }
    }

    #[derive(Debug)]
    struct StaticClientFactory {
        client: Arc<dyn TransportClient>,
    }

    impl HttpClientFactory for StaticClientFactory {
        fn build(
            &self,
            _connection_pool: &ConnectionPoolOptions,
            _config: HttpClientConfig,
        ) -> crate::error::Result<Arc<dyn TransportClient>> {
            Ok(Arc::clone(&self.client))
        }
    }

    #[derive(Debug)]
    struct FailAfterFactory {
        successful_builds: usize,
        build_calls: std::sync::atomic::AtomicUsize,
    }

    impl HttpClientFactory for FailAfterFactory {
        fn build(
            &self,
            _connection_pool: &ConnectionPoolOptions,
            _config: HttpClientConfig,
        ) -> crate::error::Result<Arc<dyn TransportClient>> {
            let call = self.build_calls.fetch_add(1, Ordering::Relaxed);
            if call < self.successful_builds {
                Ok(Arc::new(NoopTransportClient))
            } else {
                Err(crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::CLIENT_HTTP_CLIENT_CONSTRUCTION_FAILED)
                    .with_message("synthetic client construction failure")
                    .build())
            }
        }
    }

    #[derive(Debug, Default)]
    struct CompletingTransportClient {
        active: AtomicU32,
        peak_active: AtomicU32,
        completions: AtomicU64,
        overlap_barrier: Mutex<Option<Arc<tokio::sync::Barrier>>>,
    }

    impl CompletingTransportClient {
        fn set_overlap_barrier(&self, barrier: Option<Arc<tokio::sync::Barrier>>) {
            *self
                .overlap_barrier
                .lock()
                .expect("overlap barrier lock poisoned") = barrier;
        }
    }

    #[async_trait]
    impl TransportClient for CompletingTransportClient {
        async fn send(&self, _request: &HttpRequest) -> Result<HttpResponse, TransportError> {
            let active = self.active.fetch_add(1, Ordering::Relaxed) + 1;
            self.peak_active.fetch_max(active, Ordering::Relaxed);
            let overlap_barrier = self
                .overlap_barrier
                .lock()
                .expect("overlap barrier lock poisoned")
                .clone();
            if let Some(overlap_barrier) = overlap_barrier {
                overlap_barrier.wait().await;
            }
            self.active.fetch_sub(1, Ordering::Relaxed);
            self.completions.fetch_add(1, Ordering::Relaxed);
            Ok(successful_response())
        }
    }

    #[derive(Debug)]
    struct PendingTransportClient;

    #[async_trait]
    impl TransportClient for PendingTransportClient {
        async fn send(&self, _request: &HttpRequest) -> Result<HttpResponse, TransportError> {
            futures::future::pending().await
        }
    }

    fn connection_pool() -> ConnectionPoolOptions {
        ConnectionPoolOptions::builder()
            .with_max_http2_streams_per_client(2)
            .with_target_http2_streams_per_client(2)
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

    fn test_request(endpoint: &str) -> HttpRequest {
        HttpRequest {
            url: Url::parse(endpoint).unwrap(),
            method: azure_core::http::Method::Post,
            headers: azure_core::http::headers::Headers::new(),
            body: None,
            timeout: None,
            #[cfg(feature = "fault_injection")]
            evaluation_collector: None,
        }
    }

    fn shard_pool(
        min_shard_count: usize,
        max_shard_count: usize,
        max_streams: u32,
    ) -> EndpointShardPool {
        // Preserves prior test semantics for callers written before the
        // target/max split: target == max, so normal-path selection is
        // bounded by the same value as the hard reservation cap.
        shard_pool_with_target(min_shard_count, max_shard_count, max_streams, max_streams)
    }

    fn shard_pool_with_target(
        min_shard_count: usize,
        max_shard_count: usize,
        max_streams: u32,
        target_streams: u32,
    ) -> EndpointShardPool {
        let connection_pool = ConnectionPoolOptions::builder()
            .with_max_http2_streams_per_client(max_streams)
            .with_target_http2_streams_per_client(target_streams)
            .with_min_http2_connections_per_endpoint(min_shard_count)
            .with_max_http2_connections_per_endpoint(max_shard_count)
            .build()
            .unwrap();
        let client_config = HttpClientConfig::dataplane_gateway(
            &connection_pool,
            crate::diagnostics::TransportHttpVersion::Http2,
        );
        EndpointShardPool::new(
            EndpointKey(Arc::from("shard-pool.documents.azure.com:443")),
            connection_pool,
            Arc::new(TrackingFactory::default()),
            client_config,
        )
        .unwrap()
    }

    #[test]
    fn concurrent_preselection_reuses_new_capacity_before_scaling_again() {
        const PRESELECT_COUNT: usize = 16;

        let pool = Arc::new(shard_pool(1, 4, 1));
        let full_reservation = pool.select_shard(None, None).unwrap();
        let full_shard_id = full_reservation.shard.id;
        pool.set_preselection_slow_path_barrier(Arc::new(Barrier::new(PRESELECT_COUNT)));

        let selections = std::thread::scope(|scope| {
            let (sender, receiver) = mpsc::channel();
            for _ in 0..PRESELECT_COUNT {
                let pool = Arc::clone(&pool);
                let sender = sender.clone();
                scope.spawn(move || {
                    sender
                        .send(pool.pre_select_shard_id(None).unwrap())
                        .unwrap();
                });
            }
            drop(sender);
            receiver.into_iter().collect::<Vec<_>>()
        });

        assert_eq!(selections.len(), PRESELECT_COUNT);
        assert!(selections.iter().all(|shard_id| *shard_id != full_shard_id));
        assert!(selections.iter().all(|shard_id| *shard_id == selections[0]));
        assert_eq!(pool.shards.load().len(), 2);
    }

    #[test]
    fn concurrent_selection_reserves_without_exceeding_stream_limit() {
        const MAX_STREAMS: u32 = 2;
        const REQUEST_COUNT: usize = 32;

        let connection_pool = ConnectionPoolOptions::builder()
            .with_max_http2_streams_per_client(MAX_STREAMS)
            .with_target_http2_streams_per_client(MAX_STREAMS)
            .with_min_http2_connections_per_endpoint(1)
            .with_max_http2_connections_per_endpoint(REQUEST_COUNT / MAX_STREAMS as usize)
            .build()
            .unwrap();
        let client_config = HttpClientConfig::dataplane_gateway(
            &connection_pool,
            crate::diagnostics::TransportHttpVersion::Http2,
        );
        let pool = Arc::new(
            EndpointShardPool::new(
                EndpointKey(Arc::from("concurrent.documents.azure.com:443")),
                connection_pool,
                Arc::new(TrackingFactory::default()),
                client_config,
            )
            .unwrap(),
        );
        let start = Arc::new(Barrier::new(REQUEST_COUNT + 1));

        let reservations = std::thread::scope(|scope| {
            let (sender, receiver) = mpsc::channel();
            for _ in 0..REQUEST_COUNT {
                let pool = Arc::clone(&pool);
                let start = Arc::clone(&start);
                let sender = sender.clone();
                scope.spawn(move || {
                    start.wait();
                    let reservation = pool.select_shard(None, None).unwrap();
                    assert!(sender.send(reservation).is_ok());
                });
            }
            drop(sender);
            start.wait();
            receiver.into_iter().collect::<Vec<_>>()
        });

        assert_eq!(reservations.len(), REQUEST_COUNT);
        let inflight = pool
            .shards
            .load()
            .iter()
            .map(|shard| shard.inflight())
            .collect::<Vec<_>>();
        assert_eq!(inflight.iter().sum::<u32>(), REQUEST_COUNT as u32);
        assert!(inflight.iter().all(|count| *count <= MAX_STREAMS));
        assert!(inflight.contains(&MAX_STREAMS));
    }

    #[test]
    fn successful_reservation_finishes_once() {
        let pool = EndpointShardPool::new(
            EndpointKey(Arc::from("success.documents.azure.com:443")),
            connection_pool(),
            Arc::new(TrackingFactory::default()),
            client_config(),
        )
        .unwrap();

        let reservation = pool.select_shard(None, None).unwrap();
        let shard = Arc::clone(&reservation.shard);

        assert_eq!(shard.inflight(), 1);
        assert_eq!(shard.total_requests.load(Ordering::Relaxed), 1);

        reservation.finish(&Ok(successful_response()));

        assert_eq!(shard.inflight(), 0);
        assert_eq!(shard.total_requests.load(Ordering::Relaxed), 1);
        assert_eq!(shard.total_cancellations.load(Ordering::Relaxed), 0);
        assert_ne!(
            shard.last_success_at_nanos.load(Ordering::Relaxed),
            TIMESTAMP_NONE
        );
    }

    #[test]
    fn dropped_reservation_records_cancellation_once() {
        let pool = EndpointShardPool::new(
            EndpointKey(Arc::from("cancel.documents.azure.com:443")),
            connection_pool(),
            Arc::new(TrackingFactory::default()),
            client_config(),
        )
        .unwrap();

        let reservation = pool.select_shard(None, None).unwrap();
        let shard = Arc::clone(&reservation.shard);
        drop(reservation);

        assert_eq!(shard.inflight(), 0);
        assert_eq!(shard.total_requests.load(Ordering::Relaxed), 1);
        assert_eq!(shard.total_cancellations.load(Ordering::Relaxed), 1);
        assert_eq!(shard.total_failures.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn repeated_concurrent_sends_complete_and_release_reservations() {
        const CONCURRENCY: usize = 20;
        const ROUNDS: usize = 50;
        const SEED_COUNT: usize = 100;

        let expected = (SEED_COUNT + CONCURRENCY * ROUNDS) as u64;
        let (sender, receiver) = mpsc::sync_channel(1);
        let worker = std::thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(4)
                .enable_time()
                .build()
                .unwrap();
            runtime.block_on(async move {
                let connection_pool = ConnectionPoolOptions::builder()
                    .with_max_http2_streams_per_client(16)
                    .with_min_http2_connections_per_endpoint(1)
                    .with_max_http2_connections_per_endpoint(4)
                    .build()
                    .unwrap();
                let client = Arc::new(CompletingTransportClient::default());
                let transport_client: Arc<dyn TransportClient> = client.clone();
                let transport = Arc::new(ShardedHttpTransport::new(
                    connection_pool.clone(),
                    Arc::new(StaticClientFactory {
                        client: transport_client,
                    }),
                    HttpClientConfig::dataplane_gateway_v2(&connection_pool),
                ));
                let endpoint =
                    Arc::new(EndpointKey(Arc::from("send-test.documents.azure.com:443")));
                let request = Arc::new(test_request("https://send-test.documents.azure.com/dbs"));

                for _ in 0..SEED_COUNT {
                    let dispatch = transport.send(&request, None, &endpoint, None).await;
                    assert!(dispatch.result.is_ok());
                }

                for _ in 0..ROUNDS {
                    client.set_overlap_barrier(Some(Arc::new(tokio::sync::Barrier::new(
                        CONCURRENCY,
                    ))));
                    let start = Arc::new(tokio::sync::Barrier::new(CONCURRENCY + 1));
                    let mut tasks = tokio::task::JoinSet::new();
                    for _ in 0..CONCURRENCY {
                        let transport = Arc::clone(&transport);
                        let endpoint = Arc::clone(&endpoint);
                        let request = Arc::clone(&request);
                        let start = Arc::clone(&start);
                        tasks.spawn(async move {
                            start.wait().await;
                            let preferred = transport.pre_select_shard_id(None, &endpoint);
                            transport.send(&request, None, &endpoint, preferred).await
                        });
                    }
                    start.wait().await;
                    while let Some(result) = tasks.join_next().await {
                        assert!(result.unwrap().result.is_ok());
                    }
                    client.set_overlap_barrier(None);
                }

                let pool = transport
                    .get_or_create_pool(endpoint.as_ref().clone())
                    .unwrap();
                let shards = pool.shards.load();
                let inflight = shards.iter().map(|shard| shard.inflight()).sum::<u32>();
                let total_requests = shards
                    .iter()
                    .map(|shard| shard.total_requests.load(Ordering::Relaxed))
                    .sum::<u64>();
                let total_cancellations = shards
                    .iter()
                    .map(|shard| shard.total_cancellations.load(Ordering::Relaxed))
                    .sum::<u64>();
                sender
                    .send((
                        client.completions.load(Ordering::Relaxed),
                        client.peak_active.load(Ordering::Relaxed),
                        inflight,
                        total_requests,
                        total_cancellations,
                    ))
                    .unwrap();
            });
        });

        let summary = match receiver.recv_timeout(Duration::from_secs(5)) {
            Ok(summary) => summary,
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                worker.join().expect("concurrent send worker panicked");
                panic!("concurrent send worker exited without a summary");
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                panic!("concurrent sends should complete before the timeout");
            }
        };
        worker.join().expect("concurrent send worker panicked");

        let (completions, peak_active, inflight, total_requests, total_cancellations) = summary;
        assert_eq!(
            (completions, inflight, total_requests, total_cancellations),
            (expected, 0, expected, 0)
        );
        assert_eq!(peak_active, CONCURRENCY as u32);
    }

    #[tokio::test]
    async fn cancelled_send_releases_reservation_once() {
        let connection_pool = connection_pool();
        let transport = ShardedHttpTransport::new(
            connection_pool.clone(),
            Arc::new(StaticClientFactory {
                client: Arc::new(PendingTransportClient),
            }),
            HttpClientConfig::dataplane_gateway_v2(&connection_pool),
        );
        let endpoint = EndpointKey(Arc::from("cancel-send.documents.azure.com:443"));
        let request = test_request("https://cancel-send.documents.azure.com/dbs");
        let pool = transport.get_or_create_pool(endpoint.clone()).unwrap();
        let preferred = transport.pre_select_shard_id(None, &endpoint);

        let result = tokio::time::timeout(
            Duration::from_millis(25),
            transport.send(&request, None, &endpoint, preferred),
        )
        .await;

        assert!(result.is_err());
        let shards = pool.shards.load();
        assert_eq!(shards.iter().map(|shard| shard.inflight()).sum::<u32>(), 0);
        assert_eq!(
            shards
                .iter()
                .map(|shard| shard.total_requests.load(Ordering::Relaxed))
                .sum::<u64>(),
            1
        );
        assert_eq!(
            shards
                .iter()
                .map(|shard| shard.total_cancellations.load(Ordering::Relaxed))
                .sum::<u64>(),
            1
        );
        assert_eq!(
            shards
                .iter()
                .map(|shard| shard.total_failures.load(Ordering::Relaxed))
                .sum::<u64>(),
            0
        );
    }

    #[test]
    fn preselection_is_non_reserving_and_preferred_when_selectable() {
        let pool = EndpointShardPool::new(
            EndpointKey(Arc::from("preferred.documents.azure.com:443")),
            connection_pool(),
            Arc::new(TrackingFactory::default()),
            client_config(),
        )
        .unwrap();

        let preferred_id = pool.pre_select_shard_id(None).unwrap();
        let preferred = Arc::clone(&pool.shards.load()[0]);

        assert_eq!(preferred.id, preferred_id);
        assert_eq!(preferred.inflight(), 0);
        assert_eq!(preferred.total_requests.load(Ordering::Relaxed), 0);

        let reservation = pool.select_shard(None, Some(preferred_id)).unwrap();

        assert_eq!(reservation.shard.id, preferred_id);
        assert_eq!(preferred.inflight(), 1);
        assert_eq!(preferred.total_requests.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn preferred_at_target_uses_less_loaded_peer() {
        let pool = shard_pool_with_target(2, 2, 4, 1);
        let shards = pool.shards.load();
        let preferred = Arc::clone(&shards[0]);
        let peer = Arc::clone(&shards[1]);
        drop(shards);

        let preferred_reservation = preferred.try_reserve(4).unwrap();
        let selected = pool.select_shard(None, Some(preferred.id)).unwrap();

        assert_eq!(preferred.inflight(), 1);
        assert_eq!(selected.shard.id, peer.id);
        assert_eq!(peer.inflight(), 1);

        selected.finish(&Ok(successful_response()));
        preferred_reservation.finish(&Ok(successful_response()));
    }

    #[test]
    fn shard_build_failure_uses_existing_hard_cap_headroom() {
        let connection_pool = ConnectionPoolOptions::builder()
            .with_max_http2_streams_per_client(4)
            .with_target_http2_streams_per_client(1)
            .with_min_http2_connections_per_endpoint(1)
            .with_max_http2_connections_per_endpoint(2)
            .build()
            .unwrap();
        let client_config = HttpClientConfig::dataplane_gateway(
            &connection_pool,
            crate::diagnostics::TransportHttpVersion::Http2,
        );
        let factory = Arc::new(FailAfterFactory {
            successful_builds: 1,
            build_calls: std::sync::atomic::AtomicUsize::new(0),
        });
        let pool = EndpointShardPool::new(
            EndpointKey(Arc::from("build-failure.documents.azure.com:443")),
            connection_pool,
            factory.clone(),
            client_config,
        )
        .unwrap();

        let target_reservation = pool.select_shard(None, None).unwrap();
        let fallback = pool.select_shard(None, None).unwrap();

        assert_eq!(factory.build_calls.load(Ordering::Relaxed), 2);
        assert_eq!(pool.shards.load().len(), 1);
        assert_eq!(fallback.shard.id, target_reservation.shard.id);
        assert_eq!(fallback.shard.inflight(), 2);

        fallback.finish(&Ok(successful_response()));
        target_reservation.finish(&Ok(successful_response()));
    }

    #[test]
    fn shard_build_failure_without_fallback_preserves_original_error() {
        let connection_pool = ConnectionPoolOptions::builder()
            .with_max_http2_streams_per_client(4)
            .with_target_http2_streams_per_client(1)
            .with_min_http2_connections_per_endpoint(1)
            .with_max_http2_connections_per_endpoint(2)
            .build()
            .unwrap();
        let client_config = HttpClientConfig::dataplane_gateway(
            &connection_pool,
            crate::diagnostics::TransportHttpVersion::Http2,
        );
        let pool = EndpointShardPool::new(
            EndpointKey(Arc::from("empty-build-failure.documents.azure.com:443")),
            connection_pool,
            Arc::new(FailAfterFactory {
                successful_builds: 0,
                build_calls: std::sync::atomic::AtomicUsize::new(0),
            }),
            client_config,
        )
        .unwrap();

        let error = match pool.select_shard(None, None) {
            Ok(_) => panic!("selection should preserve the construction failure"),
            Err(error) => error,
        };

        assert_eq!(
            error.status(),
            crate::error::CosmosStatus::CLIENT_HTTP_CLIENT_CONSTRUCTION_FAILED
        );
        assert_eq!(pool.shards.load().len(), 0);
    }

    #[test]
    fn max_connections_fallback_does_not_wait_for_write_lock() {
        let pool = Arc::new(shard_pool_with_target(1, 1, 4, 1));
        let target_reservation = pool.select_shard(None, None).unwrap();
        let write_guard = pool
            .write_lock
            .lock()
            .expect("write lock should not be poisoned");
        let (sender, receiver) = mpsc::sync_channel(1);
        let pool_for_thread = Arc::clone(&pool);
        let handle = std::thread::spawn(move || {
            sender
                .send(pool_for_thread.select_shard(None, None))
                .unwrap();
        });

        let result = receiver.recv_timeout(Duration::from_secs(1));
        drop(write_guard);
        handle.join().expect("selection thread should not panic");
        let fallback = result
            .expect("max-connections fallback should not wait for write_lock")
            .unwrap();

        assert_eq!(fallback.shard.id, target_reservation.shard.id);
        assert_eq!(fallback.shard.inflight(), 2);

        fallback.finish(&Ok(successful_response()));
        target_reservation.finish(&Ok(successful_response()));
    }

    #[test]
    fn hard_limit_fallback_uses_least_loaded_shard_not_preferred() {
        let pool = shard_pool_with_target(2, 2, 4, 1);
        let shards = pool.shards.load();
        let preferred = Arc::clone(&shards[0]);
        let peer = Arc::clone(&shards[1]);
        drop(shards);

        let preferred_reservations = (0..3)
            .map(|_| preferred.try_reserve(4).unwrap())
            .collect::<Vec<_>>();
        let peer_reservation = peer.try_reserve(4).unwrap();

        let selected = pool.select_shard(None, Some(preferred.id)).unwrap();

        assert_eq!(selected.shard.id, peer.id);
        assert_eq!(preferred.inflight(), 3);
        assert_eq!(peer.inflight(), 2);

        selected.finish(&Ok(successful_response()));
        peer_reservation.finish(&Ok(successful_response()));
        for reservation in preferred_reservations {
            reservation.finish(&Ok(successful_response()));
        }
    }

    #[test]
    fn selection_excludes_preferred_shard_when_requested() {
        let pool = EndpointShardPool::new(
            EndpointKey(Arc::from("excluded.documents.azure.com:443")),
            connection_pool(),
            Arc::new(TrackingFactory::default()),
            client_config(),
        )
        .unwrap();
        let excluded_id = pool.pre_select_shard_id(None).unwrap();

        let reservation = pool
            .select_shard(Some(excluded_id), Some(excluded_id))
            .unwrap();

        assert_ne!(reservation.shard.id, excluded_id);
        assert_eq!(pool.shards.load().len(), 2);
    }

    #[test]
    fn selection_preserves_over_capacity_fallback_at_max_connections() {
        let connection_pool = ConnectionPoolOptions::builder()
            .with_max_http2_streams_per_client(1)
            .with_target_http2_streams_per_client(1)
            .with_min_http2_connections_per_endpoint(1)
            .with_max_http2_connections_per_endpoint(1)
            .build()
            .unwrap();
        let client_config = HttpClientConfig::dataplane_gateway(
            &connection_pool,
            crate::diagnostics::TransportHttpVersion::Http2,
        );
        let pool = EndpointShardPool::new(
            EndpointKey(Arc::from("fallback.documents.azure.com:443")),
            connection_pool,
            Arc::new(TrackingFactory::default()),
            client_config,
        )
        .unwrap();

        let first = pool.select_shard(None, None).unwrap();
        let fallback = pool.select_shard(None, None).unwrap();

        assert_eq!(fallback.shard.id, first.shard.id);
        assert_eq!(first.shard.inflight(), 2);
        assert_eq!(pool.shards.load().len(), 1);
    }

    #[test]
    fn preselection_returns_shard_at_max_connections_and_hard_capacity() {
        let pool = shard_pool_with_target(1, 1, 1, 1);
        let reservation = pool.select_shard(None, None).unwrap();

        let preselected_id = pool.pre_select_shard_id(None).unwrap();

        assert_eq!(preselected_id, reservation.shard.id);
        assert_eq!(reservation.shard.inflight(), 1);
        assert_eq!(reservation.shard.total_requests.load(Ordering::Relaxed), 1);

        reservation.finish(&Ok(successful_response()));
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
        let first_id = first.shard.id;
        let first_second_stream = pool.select_shard(None, None).unwrap();
        let second = pool.select_shard(None, None).unwrap();

        assert_eq!(first_second_stream.shard.id, first_id);
        assert_ne!(first_id, second.shard.id);
        assert_eq!(second.shard.inflight(), 1);
        assert_eq!(second.shard.total_requests.load(Ordering::Relaxed), 1);
        assert_eq!(second.shard.total_cancellations.load(Ordering::Relaxed), 0);
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

        let first_request = pool.select_shard(None, None).unwrap();
        let first = Arc::clone(&first_request.shard);
        let first_second_request = pool.select_shard(None, None).unwrap();
        let overflow_request = pool.select_shard(None, None).unwrap();
        let overflow = Arc::clone(&overflow_request.shard);
        overflow_request.finish(&Err(synthetic_transport_error()));

        overflow.set_last_request_at(Instant::now() - Duration::from_secs(5));

        first_request.finish(&Err(synthetic_transport_error()));
        first_second_request.finish(&Err(synthetic_transport_error()));

        first.set_consecutive_failures(0);
        first.set_last_success_at(Some(Instant::now()));

        pool.run_health_sweep().unwrap();

        let selected = pool.select_shard(None, None).unwrap();

        assert_eq!(selected.shard.id, first.id);
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
        let first_id = first.shard.id;
        let first_second_stream = pool.select_shard(None, None).unwrap();
        let second = pool.select_shard(None, None).unwrap();
        let second_id = second.shard.id;
        let second_second_stream = pool.select_shard(None, None).unwrap();
        let third = pool.select_shard(None, None).unwrap();

        assert_eq!(first_second_stream.shard.id, first_id);
        assert_eq!(second_second_stream.shard.id, second_id);
        assert_ne!(first_id, second_id);
        assert_ne!(second_id, third.shard.id);
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

        let first_request = pool.select_shard(None, None).unwrap();
        let first = Arc::clone(&first_request.shard);
        let first_second_request = pool.select_shard(None, None).unwrap();
        let second_request = pool.select_shard(None, None).unwrap();
        let second = Arc::clone(&second_request.shard);

        first_request.finish(&Err(synthetic_transport_error()));
        first_second_request.finish(&Err(synthetic_transport_error()));
        second_request.finish(&Err(synthetic_transport_error()));
        second.record_request_start();
        second.record_request_finish(&Err(synthetic_transport_error()));

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

        let first_request = pool.select_shard(None, None).unwrap();
        let first = Arc::clone(&first_request.shard);
        let first_second_request = pool.select_shard(None, None).unwrap();
        let second_request = pool.select_shard(None, None).unwrap();
        let second = Arc::clone(&second_request.shard);

        first_request.finish(&Err(synthetic_transport_error()));
        first_second_request.finish(&Err(synthetic_transport_error()));
        second_request.finish(&Err(synthetic_transport_error()));

        for shard in [&first, &second] {
            shard.set_last_success_at(None);
            shard.set_last_request_at(Instant::now() - Duration::from_secs(5));
            shard.set_consecutive_failures(2);
        }

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
            .with_target_http2_streams_per_client(2)
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
        let first_request = pool.select_shard(None, None).unwrap();
        let first_second_request = pool.select_shard(None, None).unwrap();
        let second_request = pool.select_shard(None, None).unwrap();
        let second = Arc::clone(&second_request.shard);

        // Mark the second shard with consecutive failures above threshold.
        second_request.finish(&Err(synthetic_transport_error()));
        for _ in 0..2 {
            second.record_request_start();
            second.record_request_finish(&Err(synthetic_transport_error()));
        }

        // Make second's last success old enough that it passes the grace period.
        second.set_last_success_at(None);
        second.set_last_request_at(Instant::now() - Duration::from_secs(5));

        // Ensure the first shard is healthy so eviction can proceed.
        first_request.finish(&Ok(successful_response()));
        first_second_request.finish(&Ok(successful_response()));

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

    #[test]
    fn fifth_reservation_with_target_four_creates_second_shard() {
        // target=4 (desired occupancy), max=16 (hard cap), enough max
        // connections to scale up. The first 4 reservations should all land
        // on the initial shard; the 5th should trigger a second shard.
        let pool = shard_pool_with_target(1, 4, 16, 4);

        let mut reservations = Vec::new();
        for _ in 0..4 {
            reservations.push(pool.select_shard(None, None).unwrap());
        }
        assert_eq!(pool.shards.load().len(), 1);
        let first_shard_id = reservations[0].shard.id;
        assert!(reservations.iter().all(|r| r.shard.id == first_shard_id));

        let fifth = pool.select_shard(None, None).unwrap();
        assert_eq!(pool.shards.load().len(), 2);
        assert_ne!(fifth.shard.id, first_shard_id);
    }

    #[test]
    fn concurrent_selection_with_target_four_distributes_over_five_shards() {
        const REQUEST_COUNT: usize = 20;
        const TARGET_STREAMS: u32 = 4;
        const MAX_STREAMS: u32 = 16;

        // With sufficient max connections available, 20 concurrent
        // reservations at target=4 should fan out over at least 5 shards,
        // and no shard should exceed the target occupancy.
        let pool = Arc::new(shard_pool_with_target(1, 10, MAX_STREAMS, TARGET_STREAMS));
        let start = Arc::new(Barrier::new(REQUEST_COUNT + 1));

        let reservations = std::thread::scope(|scope| {
            let (sender, receiver) = mpsc::channel();
            for _ in 0..REQUEST_COUNT {
                let pool = Arc::clone(&pool);
                let start = Arc::clone(&start);
                let sender = sender.clone();
                scope.spawn(move || {
                    start.wait();
                    let reservation = pool.select_shard(None, None).unwrap();
                    assert!(sender.send(reservation).is_ok());
                });
            }
            drop(sender);
            start.wait();
            receiver.into_iter().collect::<Vec<_>>()
        });

        assert_eq!(reservations.len(), REQUEST_COUNT);
        let shards = pool.shards.load();
        assert!(
            shards.len() >= 5,
            "expected at least 5 shards for {REQUEST_COUNT} requests at target {TARGET_STREAMS}, got {}",
            shards.len()
        );
        let inflight = shards
            .iter()
            .map(|shard| shard.inflight())
            .collect::<Vec<_>>();
        assert_eq!(inflight.iter().sum::<u32>(), REQUEST_COUNT as u32);
        assert!(
            inflight.iter().all(|count| *count <= TARGET_STREAMS),
            "no shard should exceed target occupancy of {TARGET_STREAMS}, got {inflight:?}"
        );
    }

    #[test]
    fn normal_path_does_not_exceed_target_when_scale_up_available() {
        // 9 sequential reservations at target=4 with 3 connections
        // available should fan out to 3 shards (4/4/1) rather than filling
        // a single shard toward the hard max.
        let pool = shard_pool_with_target(1, 3, 16, 4);

        let mut reservations = Vec::new();
        for _ in 0..9 {
            reservations.push(pool.select_shard(None, None).unwrap());
        }

        let shards = pool.shards.load();
        assert_eq!(
            shards.len(),
            3,
            "9 reservations at target 4 with scale-up available should use 3 shards"
        );
        let inflight = shards
            .iter()
            .map(|shard| shard.inflight())
            .collect::<Vec<_>>();
        assert_eq!(inflight.iter().sum::<u32>(), 9);
        assert!(
            inflight.iter().all(|count| *count <= 4),
            "no shard should exceed the target occupancy of 4, got {inflight:?}"
        );
    }

    #[test]
    fn max_connections_fallback_permits_exceeding_target_but_not_hard_max() {
        // A single allowed connection means no new shard can ever be
        // created, so once the target (4) is reached the bounded
        // max-connections fallback must let the shard keep growing up to
        // (but never past) the hard max (16).
        let pool = shard_pool_with_target(1, 1, 16, 4);

        let mut reservations = Vec::new();
        for _ in 0..16 {
            reservations.push(pool.select_shard(None, None).unwrap());
        }

        let shards = pool.shards.load();
        assert_eq!(
            shards.len(),
            1,
            "max_http2_connections_per_endpoint=1 must not create a second shard"
        );
        assert_eq!(
            shards[0].inflight(),
            16,
            "single shard should grow beyond target(4) up to hard max(16)"
        );
    }
}
