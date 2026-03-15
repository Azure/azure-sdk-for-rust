// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! HTTP/2 transport sharding for gateway endpoints.

use std::{
    collections::HashMap,
    fmt,
    sync::{
        atomic::{AtomicU32, AtomicU64, Ordering},
        Arc, Mutex, RwLock,
    },
    time::{Duration, Instant},
};

use azure_core::{
    error::ErrorKind,
    http::{AsyncRawResponse, HttpClient, Request},
};
#[cfg(feature = "tokio")]
use tracing::debug;
use url::Url;

use crate::diagnostics::TransportShardDiagnostics;
use crate::options::ConnectionPoolOptions;

#[cfg(feature = "tokio")]
use super::background_task_manager::BackgroundTaskManager;
use super::http_client_factory::{HttpClientConfig, HttpClientFactory};

pub(crate) struct TransportDispatch {
    pub(crate) result: azure_core::Result<AsyncRawResponse>,
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
        request: &Request,
        excluded_shard_id: Option<u64>,
    ) -> TransportDispatch {
        let endpoint_key = match EndpointKey::from_url(request.url()) {
            Ok(endpoint_key) => endpoint_key,
            Err(error) => {
                return TransportDispatch {
                    result: Err(error),
                    shard_id: None,
                    shard_diagnostics: None,
                };
            }
        };

        let pool = match self.get_or_create_pool(endpoint_key) {
            Ok(pool) => pool,
            Err(error) => {
                return TransportDispatch {
                    result: Err(error),
                    shard_id: None,
                    shard_diagnostics: None,
                };
            }
        };

        let shard = match pool.select_shard(excluded_shard_id) {
            Ok(shard) => shard,
            Err(error) => {
                return TransportDispatch {
                    result: Err(error),
                    shard_id: None,
                    shard_diagnostics: None,
                };
            }
        };

        let shard_id = shard.id;
        let guard = shard.start_request();
        let result = shard.client.execute_request(request).await;
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
        request: &Request,
        excluded_shard_id: u64,
    ) -> bool {
        let endpoint_key = match EndpointKey::from_url(request.url()) {
            Ok(endpoint_key) => endpoint_key,
            Err(_) => return false,
        };
        let pools = self.pools.lock().expect("endpoint pool lock poisoned");
        pools
            .get(&endpoint_key)
            .is_some_and(|pool| pool.can_select_different_shard(excluded_shard_id))
    }

    fn get_or_create_pool(
        &self,
        endpoint_key: EndpointKey,
    ) -> azure_core::Result<Arc<EndpointShardPool>> {
        let mut pools = self.pools.lock().expect("endpoint pool lock poisoned");
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
                    .expect("endpoint pool lock poisoned")
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct EndpointKey(String);

impl EndpointKey {
    fn from_url(url: &Url) -> azure_core::Result<Self> {
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
        Ok(Self(format!("{host}:{port}")))
    }
}

struct EndpointShardPool {
    endpoint: EndpointKey,
    connection_pool: ConnectionPoolOptions,
    client_factory: Arc<dyn HttpClientFactory>,
    base_client_config: HttpClientConfig,
    shards: RwLock<Vec<Arc<ClientShard>>>,
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
            shards: RwLock::new(Vec::new()),
            next_shard_id: AtomicU64::new(1),
        };

        {
            let mut shards = pool.shards.write().expect("shard lock poisoned");
            while shards.len() < pool.connection_pool.min_http2_connections_per_endpoint() {
                let shard = pool.build_shard(shards.len())?;
                shards.push(Arc::new(shard));
            }
        }

        Ok(pool)
    }

    fn select_shard(&self, excluded_shard_id: Option<u64>) -> azure_core::Result<Arc<ClientShard>> {
        let max_streams = self.connection_pool.max_http2_streams_per_client();

        // Fast path: select in-place under read lock, clone only the winner.
        {
            let shards = self.shards.read().expect("shard lock poisoned");

            if !shards.is_empty() {
                let active_count = self.active_shard_count_from_slice(&shards, excluded_shard_id);

                // Try least-loaded with capacity among active shards
                if let Some(shard) = shards
                    .iter()
                    .filter(|s| s.is_selectable(excluded_shard_id))
                    .take(active_count)
                    .filter(|s| s.inflight() < max_streams)
                    .min_by_key(|s| s.inflight())
                {
                    return Ok(Arc::clone(shard));
                }
            }
            // Read lock dropped here before potential write lock in try_create_shard.
        }

        // All active shards at capacity (or pool is empty) — try creating a new one.
        if let Some(shard) = self.try_create_shard()? {
            return Ok(shard);
        }

        // Fallback: least-loaded of all selectable shards (over-capacity).
        let shards = self.shards.read().expect("shard lock poisoned");
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

    fn active_shard_count_from_slice(
        &self,
        shards: &[Arc<ClientShard>],
        excluded_shard_id: Option<u64>,
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

        let max_streams = self.connection_pool.max_http2_streams_per_client() as usize;
        let needed = (total_inflight as usize + 1).div_ceil(max_streams);
        needed
            .max(self.connection_pool.min_http2_connections_per_endpoint())
            .min(selectable_count)
            .max(1)
    }

    fn can_select_different_shard(&self, excluded_shard_id: u64) -> bool {
        let shards = self.shards.read().expect("shard lock poisoned");
        shards
            .iter()
            .any(|shard| shard.is_selectable(Some(excluded_shard_id)))
            || shards.len() < self.connection_pool.max_http2_connections_per_endpoint()
    }

    fn try_create_shard(&self) -> azure_core::Result<Option<Arc<ClientShard>>> {
        let mut shards = self.shards.write().expect("shard lock poisoned");
        if shards.len() >= self.connection_pool.max_http2_connections_per_endpoint() {
            return Ok(None);
        }

        let shard = Arc::new(self.build_shard(shards.len())?);
        shards.push(shard.clone());
        Ok(Some(shard))
    }

    fn build_shard(&self, shard_ordinal: usize) -> azure_core::Result<ClientShard> {
        let mut client_config = self.base_client_config;
        client_config.http2_keep_alive_while_idle =
            shard_ordinal < self.connection_pool.http2_keep_alive_idle_client_count();

        let client = self
            .client_factory
            .build(&self.connection_pool, client_config)?;

        Ok(ClientShard::new(
            self.next_shard_id.fetch_add(1, Ordering::Relaxed),
            client,
        ))
    }

    fn run_health_sweep(&self) -> azure_core::Result<()> {
        let now = Instant::now();
        let threshold = self.connection_pool.http2_consecutive_failure_threshold();
        let grace = self.connection_pool.http2_eviction_grace_period();
        let idle_timeout = self.connection_pool.idle_http2_client_timeout();
        let min_clients = self.connection_pool.min_http2_connections_per_endpoint();
        let max_clients = self.connection_pool.max_http2_connections_per_endpoint();

        // Phase 1: evaluate, mark, and remove under write lock.
        // Determine how many replacement shards are needed but do NOT build
        // them here — `build_shard` performs TCP/TLS which would block
        // concurrent `select_shard` readers.
        let shards_needed = {
            let mut shards = self.shards.write().expect("shard lock poisoned");

            if shards.is_empty() {
                min_clients.saturating_sub(shards.len())
            } else {
                let snapshots = shards
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
                for shard in shards.iter() {
                    let snapshot = shard.snapshot();
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
                        shard.mark_for_eviction();
                    }
                }

                shards.retain(|shard| !(shard.is_marked_for_eviction() && shard.inflight() == 0));

                // Reclaim idle overflow shards
                while shards.len() > min_clients {
                    let should_remove = shards.last().is_some_and(|shard| {
                        !shard.is_marked_for_eviction() && shard.is_idle_for(now, idle_timeout)
                    });
                    if !should_remove {
                        break;
                    }
                    shards.pop();
                }

                // Calculate how many shards we need to build outside the lock.
                let mut needed = 0;
                if needs_probe_replacement && shards.len() < max_clients {
                    needed += 1;
                }
                needed += min_clients.saturating_sub(shards.len() + needed);
                needed
            }
            // Write lock dropped here.
        };

        if shards_needed == 0 {
            return Ok(());
        }

        // Phase 2: build replacement shards outside the lock. This is the
        // expensive part (TCP connect, TLS handshake) and must not block
        // concurrent `select_shard` readers.
        let mut new_shards = Vec::with_capacity(shards_needed);
        for _ in 0..shards_needed {
            // Use 0 as ordinal placeholder — the actual ordinal is set when inserting.
            let shard = self.build_shard(0)?;
            new_shards.push(Arc::new(shard));
        }

        // Phase 3: re-acquire write lock and insert the new shards.
        {
            let mut shards = self.shards.write().expect("shard lock poisoned");
            for new_shard in new_shards {
                if shards.len() < max_clients {
                    shards.push(new_shard);
                }
            }
        }

        Ok(())
    }
}

impl fmt::Debug for EndpointShardPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let shard_count = self.shards.read().map(|s| s.len()).unwrap_or_default();
        f.debug_struct("EndpointShardPool")
            .field("endpoint", &self.endpoint)
            .field("shard_count", &shard_count)
            .finish_non_exhaustive()
    }
}

struct ClientShard {
    id: u64,
    client: Arc<dyn HttpClient>,
    inflight: AtomicU32,
    state: Mutex<ClientShardState>,
}

impl ClientShard {
    fn new(id: u64, client: Arc<dyn HttpClient>) -> Self {
        Self {
            id,
            client,
            inflight: AtomicU32::new(0),
            state: Mutex::new(ClientShardState {
                last_request_at: Instant::now(),
                last_success_at: None,
                consecutive_failures: 0,
                total_requests: 0,
                total_failures: 0,
                marked_for_eviction: false,
            }),
        }
    }

    fn inflight(&self) -> u32 {
        self.inflight.load(Ordering::Relaxed)
    }

    /// Begins tracking an inflight request and returns an RAII guard.
    ///
    /// The guard ensures the inflight counter is always decremented, even if
    /// the async transport future is cancelled (e.g., by a timeout race in
    /// `execute_http_attempt`). Call [`InflightGuard::finish`] with the result
    /// to record success/failure state. If the guard is dropped without
    /// `finish`, only the inflight counter is decremented.
    fn start_request(&self) -> InflightGuard<'_> {
        self.inflight.fetch_add(1, Ordering::Relaxed);
        {
            let mut state = self.state.lock().expect("shard state lock poisoned");
            state.last_request_at = Instant::now();
            state.total_requests = state.total_requests.saturating_add(1);
        }
        InflightGuard {
            shard: self,
            finished: false,
        }
    }

    fn record_request_outcome(&self, result: &azure_core::Result<AsyncRawResponse>) {
        self.inflight.fetch_sub(1, Ordering::Relaxed);
        let mut state = self.state.lock().expect("shard state lock poisoned");
        state.last_request_at = Instant::now();
        if result.is_ok() {
            state.last_success_at = Some(state.last_request_at);
            state.consecutive_failures = 0;
        } else {
            state.consecutive_failures = state.consecutive_failures.saturating_add(1);
            state.total_failures = state.total_failures.saturating_add(1);
        }
    }

    fn decrement_inflight(&self) {
        self.inflight.fetch_sub(1, Ordering::Relaxed);
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
    fn finish(mut self, result: &azure_core::Result<AsyncRawResponse>) {
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
    fn snapshot(&self) -> ClientShardHealthSnapshot {
        let state = self.state.lock().expect("shard state lock poisoned");
        ClientShardHealthSnapshot {
            id: self.id,
            inflight: self.inflight(),
            last_request_at: state.last_request_at,
            last_success_at: state.last_success_at,
            consecutive_failures: state.consecutive_failures,
            total_requests: state.total_requests,
            total_failures: state.total_failures,
            marked_for_eviction: state.marked_for_eviction,
        }
    }

    fn mark_for_eviction(&self) {
        self.state
            .lock()
            .expect("shard state lock poisoned")
            .marked_for_eviction = true;
    }

    fn is_marked_for_eviction(&self) -> bool {
        self.state
            .lock()
            .expect("shard state lock poisoned")
            .marked_for_eviction
    }

    fn is_selectable(&self, excluded_shard_id: Option<u64>) -> bool {
        excluded_shard_id != Some(self.id) && !self.is_marked_for_eviction()
    }

    fn is_idle_for(&self, now: Instant, idle_timeout: Duration) -> bool {
        if self.inflight() != 0 {
            return false;
        }

        let state = self.state.lock().expect("shard state lock poisoned");
        now.duration_since(state.last_request_at) >= idle_timeout
    }

    fn transport_diagnostics(&self) -> TransportShardDiagnostics {
        let state = self.state.lock().expect("shard state lock poisoned");
        TransportShardDiagnostics::new(
            self.id,
            self.inflight(),
            state.consecutive_failures,
            state.total_requests,
            state.total_failures,
            state.marked_for_eviction,
        )
    }

    /// Bumps inflight for test setup (not cancellation-safe; use `start_request` in production).
    #[cfg(test)]
    fn record_request_start(&self) {
        self.inflight.fetch_add(1, Ordering::Relaxed);
        let mut state = self.state.lock().expect("shard state lock poisoned");
        state.last_request_at = Instant::now();
        state.total_requests = state.total_requests.saturating_add(1);
    }

    /// Records outcome for test setup (not cancellation-safe; use `InflightGuard::finish` in production).
    #[cfg(test)]
    fn record_request_finish(&self, result: &azure_core::Result<AsyncRawResponse>) {
        self.record_request_outcome(result);
    }
}

struct ClientShardState {
    last_request_at: Instant,
    last_success_at: Option<Instant>,
    consecutive_failures: u32,
    total_requests: u64,
    total_failures: u64,
    marked_for_eviction: bool,
}

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

impl ClientShardHealthSnapshot {
    fn has_recent_success(self, now: Instant, grace_period: Duration) -> bool {
        self.last_success_at
            .is_some_and(|last_success_at| now.duration_since(last_success_at) <= grace_period)
    }
}

fn pick_probe_candidate(
    snapshots: &[ClientShardHealthSnapshot],
    threshold: u32,
    grace_period: Duration,
    now: Instant,
) -> Option<u64> {
    let eligible = snapshots
        .iter()
        .copied()
        .filter(|snapshot| !snapshot.marked_for_eviction)
        .collect::<Vec<_>>();

    if eligible.is_empty()
        || eligible.iter().any(|snapshot| {
            snapshot.consecutive_failures < threshold
                || snapshot.has_recent_success(now, grace_period)
        })
    {
        return None;
    }

    eligible
        .into_iter()
        .max_by_key(|snapshot| {
            (
                snapshot.consecutive_failures,
                snapshot.total_failures,
                std::cmp::Reverse(snapshot.last_success_at.unwrap_or(snapshot.last_request_at)),
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
        ) -> azure_core::Result<Arc<dyn HttpClient>> {
            self.idle_ping_flags
                .lock()
                .expect("tracking lock poisoned")
                .push(config.http2_keep_alive_while_idle);
            Ok(Arc::new(NoopHttpClient))
        }
    }

    #[derive(Debug)]
    struct NoopHttpClient;

    #[async_trait]
    impl HttpClient for NoopHttpClient {
        async fn execute_request(
            &self,
            _request: &Request,
        ) -> azure_core::Result<AsyncRawResponse> {
            Err(azure_core::Error::with_message(
                ErrorKind::Other,
                "noop client should not execute requests in shard unit tests",
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
            .with_http2_keep_alive_idle_client_count(2)
            .with_idle_http2_client_timeout(Duration::from_millis(1_000))
            .build()
            .unwrap()
    }

    fn client_config() -> HttpClientConfig {
        HttpClientConfig::dataplane_gateway(
            &connection_pool(),
            crate::driver::transport::http_client_factory::NegotiatedHttpVersion::Http2,
        )
    }

    #[test]
    fn endpoint_pool_scales_up_when_active_shards_are_full() {
        let factory = Arc::new(TrackingFactory::default());
        let pool = EndpointShardPool::new(
            EndpointKey("test.documents.azure.com:443".to_owned()),
            connection_pool(),
            factory,
            client_config(),
        )
        .unwrap();

        let first = pool.select_shard(None).unwrap();
        first.record_request_start();
        first.record_request_start();

        let second = pool.select_shard(None).unwrap();

        assert_ne!(first.id, second.id);
        assert_eq!(pool.shards.read().unwrap().len(), 2);
    }

    #[test]
    fn background_sweep_reclaims_idle_overflow_shards() {
        let factory = Arc::new(TrackingFactory::default());
        let pool = EndpointShardPool::new(
            EndpointKey("test.documents.azure.com:443".to_owned()),
            connection_pool(),
            factory,
            client_config(),
        )
        .unwrap();

        let first = pool.select_shard(None).unwrap();
        first.record_request_start();
        first.record_request_start();
        let overflow = pool.select_shard(None).unwrap();
        overflow.record_request_start();
        overflow.record_request_finish(&Err(azure_core::Error::with_message(
            ErrorKind::Other,
            "synthetic",
        )));

        {
            let mut state = overflow.state.lock().unwrap();
            state.last_request_at = Instant::now() - Duration::from_secs(5);
        }

        first.record_request_finish(&Err(azure_core::Error::with_message(
            ErrorKind::Other,
            "synthetic",
        )));
        first.record_request_finish(&Err(azure_core::Error::with_message(
            ErrorKind::Other,
            "synthetic",
        )));

        {
            let mut state = first.state.lock().unwrap();
            state.consecutive_failures = 0;
            state.last_success_at = Some(Instant::now());
        }

        pool.run_health_sweep().unwrap();

        let selected = pool.select_shard(None).unwrap();

        assert_eq!(selected.id, first.id);
        assert_eq!(pool.shards.read().unwrap().len(), 1);
    }

    #[test]
    fn first_idle_ping_clients_keep_http2_idle_pings_enabled() {
        let factory = Arc::new(TrackingFactory::default());
        let pool = EndpointShardPool::new(
            EndpointKey("test.documents.azure.com:443".to_owned()),
            connection_pool(),
            factory.clone(),
            client_config(),
        )
        .unwrap();

        let first = pool.select_shard(None).unwrap();
        first.record_request_start();
        first.record_request_start();
        let second = pool.select_shard(None).unwrap();
        second.record_request_start();
        second.record_request_start();
        let third = pool.select_shard(None).unwrap();

        assert_ne!(first.id, second.id);
        assert_ne!(second.id, third.id);
        assert_eq!(factory.idle_ping_flags(), vec![true, true, false]);
    }

    #[test]
    fn health_sweep_evicts_failed_shard_when_healthy_peer_exists() {
        let factory = Arc::new(TrackingFactory::default());
        let pool = EndpointShardPool::new(
            EndpointKey("test.documents.azure.com:443".to_owned()),
            connection_pool(),
            factory,
            client_config(),
        )
        .unwrap();

        let first = pool.select_shard(None).unwrap();
        first.record_request_start();
        first.record_request_start();
        let second = pool.select_shard(None).unwrap();

        first.record_request_finish(&Err(azure_core::Error::with_message(
            ErrorKind::Other,
            "synthetic",
        )));
        first.record_request_finish(&Err(azure_core::Error::with_message(
            ErrorKind::Other,
            "synthetic",
        )));

        second.record_request_start();
        second.record_request_finish(&Err(azure_core::Error::with_message(
            ErrorKind::Other,
            "synthetic",
        )));
        second.record_request_start();
        second.record_request_finish(&Err(azure_core::Error::with_message(
            ErrorKind::Other,
            "synthetic",
        )));

        {
            let mut state = first.state.lock().unwrap();
            state.consecutive_failures = 0;
            state.last_success_at = Some(Instant::now());
            state.last_request_at = Instant::now();
        }

        {
            let mut state = second.state.lock().unwrap();
            state.last_success_at = Some(Instant::now() - Duration::from_secs(5));
            state.last_request_at = Instant::now() - Duration::from_secs(5);
        }

        pool.run_health_sweep().unwrap();

        let snapshots = pool
            .shards
            .read()
            .unwrap()
            .iter()
            .map(|shard| shard.id)
            .collect::<Vec<_>>();
        assert_eq!(snapshots, vec![first.id]);
    }

    #[test]
    fn health_sweep_replaces_only_one_probe_when_all_shards_are_failing() {
        let factory = Arc::new(TrackingFactory::default());
        let pool = EndpointShardPool::new(
            EndpointKey("test.documents.azure.com:443".to_owned()),
            connection_pool(),
            factory,
            client_config(),
        )
        .unwrap();

        let first = pool.select_shard(None).unwrap();
        first.record_request_start();
        first.record_request_start();
        let second = pool.select_shard(None).unwrap();

        first.record_request_finish(&Err(azure_core::Error::with_message(
            ErrorKind::Other,
            "synthetic",
        )));
        first.record_request_finish(&Err(azure_core::Error::with_message(
            ErrorKind::Other,
            "synthetic",
        )));

        for shard in [&first, &second] {
            let mut state = shard.state.lock().unwrap();
            state.last_success_at = None;
            state.last_request_at = Instant::now() - Duration::from_secs(5);
            state.consecutive_failures = 2;
            state.total_failures = 2;
        }

        first.inflight.store(0, Ordering::Relaxed);

        pool.run_health_sweep().unwrap();

        let shard_ids = pool
            .shards
            .read()
            .unwrap()
            .iter()
            .map(|shard| shard.id)
            .collect::<Vec<_>>();
        assert_eq!(shard_ids.len(), 2);
        assert!(shard_ids
            .iter()
            .any(|id| *id == first.id || *id == second.id));
        assert!(shard_ids.iter().any(|id| *id > second.id));
    }
}
