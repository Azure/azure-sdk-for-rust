# Gateway Endpoint Health Checks Specification

**Issue**: [Azure/azure-sdk-for-rust#3846](https://github.com/Azure/azure-sdk-for-rust/issues/3846)
**Branch**: `tvaron3/healthChecksSpec`
**Status**: Draft

---

## 1. Goals & Motivation

### Why

The driver currently has no mechanism to proactively determine whether a regional gateway
endpoint is reachable. Endpoint unavailability is only discovered reactively — when a real
user request fails and triggers a retry or failover. This leads to:

- **Avoidable first-request failures**: The driver may route requests to a region that has
  been down for minutes.
- **Slower failover**: Without proactive probing, the driver must wait for a real request
  to fail before marking an endpoint unavailable.
- **Stale availability state**: Once an endpoint is marked unavailable, the driver has no
  way to detect when it recovers except via expiration timers.

Health checks address all three by probing regional endpoints in the background and updating
the `AccountEndpointState` (§4.4 of the Transport Pipeline Spec).

### Design Constraint: Swappable Probe API

The Cosmos DB gateway team is building a dedicated health endpoint. Until that ships, we use
`GetDatabaseAccount` (a lightweight metadata read) as the health probe. The design **must**
make it trivial to swap the underlying probe API without changing the health check
orchestration, scheduling, or state management.

### Non-Goals

- **Partition-level health / circuit breaker**: Covered separately in Transport Pipeline
  Spec §4.5. Health checks here are **endpoint-level** (regional gateway availability).
- **Connection-pool shard health**: Covered in Transport Pipeline Spec §6.5–6.6 (per-shard
  eviction). That is a transport-layer concern; this spec is about service-level reachability.
- **User-facing health API**: Health checks are internal. No public API is exposed.

---

## 2. Architecture Overview

```text
┌─────────────────────────────────────────────────────────────┐
│                    CosmosDriverRuntime                       │
│                                                             │
│  ┌──────────────────┐   ┌─────────────────────────────────┐ │
│  │AccountMetadata   │   │  EndpointHealthMonitor           │ │
│  │Cache             │   │                                  │ │
│  │  (account props) │──>│  • owns background task          │ │
│  └──────────────────┘   │  • schedules probes              │ │
│                         │  • updates endpoint state        │ │
│                         │                                  │ │
│                         │  ┌────────────────────────────┐  │ │
│                         │  │HealthProbe (trait object)   │  │ │
│                         │  │  • get_database_account()   │  │ │
│                         │  │  • (future: /health API)    │  │ │
│                         │  └────────────────────────────┘  │ │
│                         └─────────────────────────────────┘ │
│                                      │                      │
│                                      ▼                      │
│                         ┌─────────────────────────────────┐ │
│                         │ AccountEndpointStateStore        │ │
│                         │   mark_endpoint_available()      │ │
│                         │   mark_endpoint_unavailable()    │ │
│                         └─────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

### Relationship to Transport Pipeline Spec

This spec extends the Transport Pipeline Spec (§4.4) by adding:
1. **Proactive probing** — a background task that calls a health probe on each endpoint
2. **Availability recovery** — marking endpoints available again after successful probes
3. **Startup probing** — verifying endpoint reachability during client initialization

The `AccountEndpointState`, `AccountEndpointStateStore`, `mark_endpoint_unavailable`, and
`expire_unavailable_endpoints` systems defined in §4.4 are consumed as-is. This spec adds
`mark_endpoint_available` and the orchestration layer.

---

## 3. Component Design

### 3.1 `HealthProbe` Trait — The Swappable Probe API

The probe abstraction isolates the health check orchestration from the specific API used to
test endpoint reachability:

```rust
// driver/health/probe.rs

/// Result of a single endpoint health probe.
#[derive(Clone, Debug)]
pub(crate) enum ProbeResult {
    /// The endpoint responded successfully.
    Healthy {
        /// How long the probe took (for diagnostics/logging).
        latency: Duration,
    },
    /// The endpoint did not respond or returned an error.
    Unhealthy {
        /// Diagnostic reason for the failure.
        reason: String,
        /// How long before the probe failed/timed out.
        latency: Duration,
    },
}

/// Trait for probing a single Cosmos DB gateway endpoint.
///
/// Implementations must be `Send + Sync` (shared across the background task
/// and potentially multiple concurrent probes).
///
/// The trait is intentionally minimal: one method, one endpoint, one result.
/// The orchestration layer handles scheduling, concurrency, retries, and
/// state updates.
#[async_trait]
pub(crate) trait HealthProbe: Send + Sync {
    /// Probe the given endpoint and return whether it is healthy.
    ///
    /// Implementations should use a short timeout (e.g. 5s) to avoid blocking
    /// the health sweep for too long.
    async fn probe(&self, endpoint: &CosmosEndpoint) -> ProbeResult;
}
```

### 3.2 `GetDatabaseAccountProbe` — Current Implementation

Uses the existing `fetch_account_properties` path to issue a `GET /` (GetDatabaseAccount)
against a specific regional endpoint:

```rust
// driver/health/get_database_account_probe.rs

/// Health probe that calls GetDatabaseAccount on the target endpoint.
///
/// This is the initial implementation. When the gateway health API ships,
/// a new `GatewayHealthProbe` will replace this — only this struct changes,
/// not the orchestration.
pub(crate) struct GetDatabaseAccountProbe {
    /// The transport layer — reuses the existing metadata pipeline for
    /// authenticated requests.
    transport: Arc<CosmosTransport>,
    /// Auth context for the account (key or AAD).
    auth: Arc<dyn Authorization>,
    /// Timeout for individual probe requests.
    probe_timeout: Duration,
}

impl GetDatabaseAccountProbe {
    pub fn new(
        transport: Arc<CosmosTransport>,
        auth: Arc<dyn Authorization>,
        probe_timeout: Duration,
    ) -> Self {
        Self {
            transport,
            auth,
            probe_timeout,
        }
    }
}

#[async_trait]
impl HealthProbe for GetDatabaseAccountProbe {
    async fn probe(&self, endpoint: &CosmosEndpoint) -> ProbeResult {
        let start = Instant::now();
        let account_endpoint = endpoint.to_account_endpoint();
        let pipeline = self
            .transport
            .create_metadata_pipeline(&account_endpoint, &*self.auth);

        let mut request = Request::new(
            account_endpoint.join_path("/"),
            azure_core::http::Method::Get,
        );
        let mut context = Context::default();
        context.insert(AuthorizationContext::new(
            azure_core::http::Method::Get,
            ResourceType::DatabaseAccount,
            "",
        ));

        // Apply probe-specific timeout.
        let result = tokio::time::timeout(
            self.probe_timeout,
            pipeline.send(&context, &mut request),
        )
        .await;

        let latency = start.elapsed();
        match result {
            Ok(Ok(response)) if response.status().is_success() => {
                ProbeResult::Healthy { latency }
            }
            Ok(Ok(response)) => ProbeResult::Unhealthy {
                reason: format!("HTTP {}", response.status()),
                latency,
            },
            Ok(Err(e)) => ProbeResult::Unhealthy {
                reason: format!("transport error: {e}"),
                latency,
            },
            Err(_) => ProbeResult::Unhealthy {
                reason: "probe timed out".to_string(),
                latency,
            },
        }
    }
}
```

### 3.3 `EndpointHealthMonitor` — Orchestration

The monitor is the central coordinator. It owns the background task, determines which
endpoints to probe, executes probes concurrently, and applies results to the endpoint
state store.

```rust
// driver/health/monitor.rs

/// Configuration for the endpoint health monitor.
#[derive(Clone, Debug)]
pub(crate) struct HealthMonitorConfig {
    /// How often the background sweep runs.
    /// Default: 300s (5 minutes), matching the Python SDK.
    pub refresh_interval: Duration,

    /// Maximum number of retries per probe attempt.
    /// Default: 3, matching the Python SDK.
    pub max_probe_retries: u32,

    /// Initial retry delay for failed probes (exponential backoff).
    /// Default: 500ms.
    pub initial_retry_delay: Duration,

    /// Maximum retry delay cap.
    /// Default: 180s (3 minutes).
    pub max_retry_delay: Duration,

    /// Timeout for individual probe requests.
    /// Default: 5s.
    pub probe_timeout: Duration,

    /// Whether to run health checks on startup.
    /// Default: true.
    pub probe_on_startup: bool,
}

impl Default for HealthMonitorConfig {
    fn default() -> Self {
        Self {
            refresh_interval: Duration::from_secs(300),
            max_probe_retries: 3,
            initial_retry_delay: Duration::from_millis(500),
            max_retry_delay: Duration::from_secs(180),
            probe_timeout: Duration::from_secs(5),
            probe_on_startup: true,
        }
    }
}

/// Manages background health checking of gateway endpoints.
///
/// Spawned by `CosmosDriver` during initialization. Runs a periodic
/// sweep that probes all relevant endpoints and updates the
/// `AccountEndpointStateStore`.
pub(crate) struct EndpointHealthMonitor {
    config: HealthMonitorConfig,
    probe: Arc<dyn HealthProbe>,
    endpoint_state_store: Arc<AccountEndpointStateStore>,
    account_metadata_cache: Arc<AccountMetadataCache>,
    /// Handle to the background sweep task. Dropped on shutdown.
    _sweep_handle: JoinHandle<()>,
}
```

### 3.4 Endpoint Selection — Which Endpoints to Probe

Following the Python SDK pattern, the monitor probes:

1. All **read endpoints** in the preferred locations list.
2. The **first write endpoint** (unless it's already in the read set).
3. Deduplicates so no endpoint is probed twice per sweep.

```rust
// driver/health/monitor.rs (pure function)

/// Determines which endpoints should be health-checked.
///
/// Returns a deduplicated list of endpoints from the current
/// `AccountEndpointState`.
fn endpoints_to_probe(state: &AccountEndpointState) -> Vec<CosmosEndpoint> {
    let mut endpoints = Vec::new();
    let mut seen = HashSet::new();

    // All preferred read endpoints.
    for ep in &state.preferred_read_endpoints {
        if seen.insert(ep.clone()) {
            endpoints.push(ep.clone());
        }
    }

    // First preferred write endpoint (if not already included).
    if let Some(write_ep) = state.preferred_write_endpoints.first() {
        if seen.insert(write_ep.clone()) {
            endpoints.push(write_ep.clone());
        }
    }

    endpoints
}
```

### 3.5 Probe Execution with Retry

Each endpoint probe has its own retry loop with exponential backoff:

```rust
// driver/health/monitor.rs

/// Probes a single endpoint with retries.
///
/// Returns the final `ProbeResult` after exhausting retries or
/// getting a healthy response.
async fn probe_with_retry(
    probe: &dyn HealthProbe,
    endpoint: &CosmosEndpoint,
    config: &HealthMonitorConfig,
) -> ProbeResult {
    let mut delay = config.initial_retry_delay;

    for attempt in 0..=config.max_probe_retries {
        let result = probe.probe(endpoint).await;
        match &result {
            ProbeResult::Healthy { .. } => return result,
            ProbeResult::Unhealthy { .. } if attempt < config.max_probe_retries => {
                // Exponential backoff: delay + 2^attempt, capped.
                let backoff = delay + Duration::from_millis(1 << attempt);
                let capped = backoff.min(config.max_retry_delay);
                tokio::time::sleep(capped).await;
                delay = capped;
            }
            _ => return result,
        }
    }

    // Unreachable, but satisfy compiler.
    ProbeResult::Unhealthy {
        reason: "max retries exhausted".to_string(),
        latency: Duration::ZERO,
    }
}
```

### 3.6 Background Sweep Loop

```rust
// driver/health/monitor.rs

impl EndpointHealthMonitor {
    /// The main background loop. Runs until the monitor is dropped.
    async fn sweep_loop(
        config: HealthMonitorConfig,
        probe: Arc<dyn HealthProbe>,
        endpoint_state_store: Arc<AccountEndpointStateStore>,
        account_metadata_cache: Arc<AccountMetadataCache>,
        shutdown: CancellationToken,
    ) {
        loop {
            tokio::select! {
                _ = tokio::time::sleep(config.refresh_interval) => {},
                _ = shutdown.cancelled() => break,
            }

            Self::run_sweep(
                &config,
                &*probe,
                &endpoint_state_store,
                &account_metadata_cache,
            )
            .await;
        }
    }

    /// Executes a single health sweep across all relevant endpoints.
    ///
    /// Steps:
    /// 1. Snapshot current endpoint state.
    /// 2. Determine which endpoints to probe.
    /// 3. Probe all endpoints concurrently.
    /// 4. Apply results: mark healthy endpoints available, unhealthy unavailable.
    async fn run_sweep(
        config: &HealthMonitorConfig,
        probe: &dyn HealthProbe,
        endpoint_state_store: &AccountEndpointStateStore,
        account_metadata_cache: &AccountMetadataCache,
    ) {
        let state = endpoint_state_store.snapshot();
        let endpoints = endpoints_to_probe(&state);

        if endpoints.is_empty() {
            return;
        }

        // Probe all endpoints concurrently.
        let probe_futures: Vec<_> = endpoints
            .iter()
            .map(|ep| {
                let ep = ep.clone();
                let probe = probe; // borrow, not move
                async move {
                    let result = probe_with_retry(probe, &ep, config).await;
                    (ep, result)
                }
            })
            .collect();

        let results = futures::future::join_all(probe_futures).await;

        // Apply results to endpoint state.
        // We build a single new state snapshot with all changes applied atomically.
        let mut new_state = (*state).clone();
        for (endpoint, result) in &results {
            match result {
                ProbeResult::Healthy { .. } => {
                    // Remove from unavailable set if present.
                    new_state.unavailable_endpoints.remove(endpoint);
                }
                ProbeResult::Unhealthy { reason, .. } => {
                    // Mark unavailable (or keep unavailable if already marked).
                    new_state.unavailable_endpoints.entry(endpoint.clone())
                        .or_insert_with(|| {
                            (Instant::now(), UnavailableReason::ServiceUnavailable)
                        });
                    tracing::warn!(
                        endpoint = %endpoint,
                        reason = %reason,
                        "health probe failed, marking endpoint unavailable"
                    );
                }
            }
        }

        endpoint_state_store.swap(new_state);
    }
}
```

### 3.7 Startup Probing

On `CosmosDriver` initialization, after the initial `GetDatabaseAccount` succeeds and
populates the `AccountMetadataCache`, the health monitor runs an initial sweep to verify
all endpoints before the driver begins accepting requests:

```rust
// driver/health/monitor.rs

impl EndpointHealthMonitor {
    /// Creates and starts the health monitor.
    ///
    /// If `config.probe_on_startup` is true, runs an initial sweep
    /// synchronously before returning. The background loop then takes over.
    pub async fn start(
        config: HealthMonitorConfig,
        probe: Arc<dyn HealthProbe>,
        endpoint_state_store: Arc<AccountEndpointStateStore>,
        account_metadata_cache: Arc<AccountMetadataCache>,
    ) -> Self {
        // Run initial sweep if configured.
        if config.probe_on_startup {
            Self::run_sweep(
                &config,
                &*probe,
                &endpoint_state_store,
                &account_metadata_cache,
            )
            .await;
        }

        let shutdown = CancellationToken::new();
        let sweep_handle = tokio::spawn(Self::sweep_loop(
            config.clone(),
            Arc::clone(&probe),
            Arc::clone(&endpoint_state_store),
            Arc::clone(&account_metadata_cache),
            shutdown.clone(),
        ));

        Self {
            config,
            probe,
            endpoint_state_store,
            account_metadata_cache,
            _sweep_handle: sweep_handle,
        }
    }
}
```

### 3.8 `mark_endpoint_available` System Function

This is a new system function complementing the existing `mark_endpoint_unavailable` from
the Transport Pipeline Spec §4.4:

```rust
// driver/routing/account_endpoint_state.rs (addition to existing systems)

/// Produce a new state with an endpoint marked as available (removed from
/// the unavailable set).
///
/// If the endpoint is not in the unavailable set, the state is returned
/// unchanged.
fn mark_endpoint_available(
    state: &AccountEndpointState,
    endpoint: &CosmosEndpoint,
) -> AccountEndpointState {
    if !state.unavailable_endpoints.contains_key(endpoint) {
        return state.clone();
    }
    let mut new_state = state.clone();
    new_state.unavailable_endpoints.remove(endpoint);
    new_state
}
```

---

## 4. Integration Points

### 4.1 `CosmosDriver` Initialization

The `EndpointHealthMonitor` is created during `CosmosDriver` construction, after the
initial `AccountProperties` fetch succeeds:

```rust
// Pseudocode in CosmosDriver::new() or get_or_create_driver():

let account_props = self.fetch_account_properties(&account).await?;
let endpoint_state = build_account_endpoint_state(
    &account_props, &preferred_locations, default_endpoint, None,
);
let endpoint_state_store = Arc::new(AccountEndpointStateStore::new(endpoint_state));

// Create health probe (swappable).
let health_probe: Arc<dyn HealthProbe> = Arc::new(GetDatabaseAccountProbe::new(
    Arc::clone(&self.runtime.transport()),
    account.auth().clone(),
    health_config.probe_timeout,
));

// Start health monitor (runs initial sweep + background loop).
let health_monitor = EndpointHealthMonitor::start(
    health_config,
    health_probe,
    Arc::clone(&endpoint_state_store),
    Arc::clone(&self.runtime.account_metadata_cache()),
)
.await;
```

### 4.2 `execute_operation` Integration

The operation loop in `execute_operation` already reads from `AccountEndpointStateStore`
(via the `LocationSnapshot` in the Transport Pipeline Spec). Health check results
automatically influence routing because the sweep updates the same
`unavailable_endpoints` map that `resolve_endpoint` reads.

No changes to `execute_operation` are required. The integration is purely through shared
state.

### 4.3 Reactive + Proactive Synergy

The health monitor (proactive) and the operation retry loop (reactive) both write to the
same `AccountEndpointStateStore`:

| Source | Marks Unavailable | Marks Available |
|--------|-------------------|-----------------|
| `execute_operation` retry (reactive) | Yes — on 503, transport error | No — relies on expiration |
| Health sweep (proactive) | Yes — on probe failure | Yes — on probe success |

This means:
- Reactive failures are detected instantly (no waiting for next sweep).
- Proactive sweeps detect recovery (no waiting for expiration timer).
- Both write atomically via `swap()` on the same store.

### 4.4 Diagnostics Integration

Health probe activity is recorded in the `DiagnosticsContext` system. The existing
`ExecutionContext::CircuitBreakerProbe` variant (already defined in the Transport Pipeline
Spec) is used for health check probe requests:

```rust
// When logging probe attempts:
tracing::debug!(
    endpoint = %endpoint,
    execution_context = "CircuitBreakerProbe",
    latency_ms = latency.as_millis(),
    "health probe completed"
);
```

---

## 5. Configuration Surface

### 5.1 `HealthCheckOptions`

New option group nested within `DriverOptions`:

```rust
// driver/options/health.rs

/// Options controlling background endpoint health checking.
///
/// Health checks probe regional gateway endpoints to detect unavailability
/// and recovery. Results feed into the endpoint routing system.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct HealthCheckOptions {
    /// Whether health checks are enabled. Default: `true`.
    enabled: bool,
    /// Interval between background health sweeps. Default: 300s (5 minutes).
    refresh_interval: Duration,
    /// Whether to probe all endpoints on startup before accepting requests.
    /// Default: `true`.
    probe_on_startup: bool,
    /// Maximum retries per probe attempt. Default: 3.
    max_probe_retries: u32,
    /// Initial delay between probe retries. Default: 500ms.
    initial_retry_delay: Duration,
    /// Maximum delay between probe retries. Default: 180s.
    max_retry_delay: Duration,
    /// Timeout for individual probe requests. Default: 5s.
    probe_timeout: Duration,
}

impl Default for HealthCheckOptions {
    fn default() -> Self {
        Self {
            enabled: true,
            refresh_interval: Duration::from_secs(300),
            probe_on_startup: true,
            max_probe_retries: 3,
            initial_retry_delay: Duration::from_millis(500),
            max_retry_delay: Duration::from_secs(180),
            probe_timeout: Duration::from_secs(5),
        }
    }
}

impl HealthCheckOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn with_refresh_interval(mut self, interval: Duration) -> Self {
        self.refresh_interval = interval;
        self
    }

    pub fn with_probe_on_startup(mut self, probe: bool) -> Self {
        self.probe_on_startup = probe;
        self
    }

    pub fn with_max_probe_retries(mut self, retries: u32) -> Self {
        self.max_probe_retries = retries;
        self
    }

    pub fn with_probe_timeout(mut self, timeout: Duration) -> Self {
        self.probe_timeout = timeout;
        self
    }
}
```

### 5.2 Integration with `DriverOptions`

```rust
// In options/mod.rs — addition to DriverOptions:

pub struct DriverOptions {
    // ... existing fields ...

    /// Options for background endpoint health checking.
    pub health_check: HealthCheckOptions,
}
```

### 5.3 Environment Variable Overrides

Following the Python SDK pattern, key thresholds can be overridden via environment
variables for debugging and tuning without code changes:

| Env Var | Field | Default |
|---------|-------|---------|
| `AZURE_COSMOS_HEALTH_CHECK_INTERVAL_SECS` | `refresh_interval` | 300 |
| `AZURE_COSMOS_HEALTH_CHECK_MAX_RETRIES` | `max_probe_retries` | 3 |
| `AZURE_COSMOS_HEALTH_CHECK_RETRY_AFTER_MS` | `initial_retry_delay` | 500 |
| `AZURE_COSMOS_HEALTH_CHECK_PROBE_TIMEOUT_SECS` | `probe_timeout` | 5 |
| `AZURE_COSMOS_HEALTH_CHECK_ENABLED` | `enabled` | true |

Environment variables take precedence over programmatic options. This is resolved at
`HealthMonitorConfig` construction time (not in the public options struct).

---

## 6. File Layout

```text
sdk/cosmos/azure_data_cosmos_driver/src/
  driver/
    health/
      mod.rs                          # Public exports
      probe.rs                        # HealthProbe trait + ProbeResult
      get_database_account_probe.rs   # GetDatabaseAccount implementation
      monitor.rs                      # EndpointHealthMonitor + sweep loop
      config.rs                       # HealthMonitorConfig (internal)
  options/
    health.rs                         # HealthCheckOptions (public)
```

---

## 7. Swapping to Gateway Health API

When the dedicated gateway health endpoint ships, the migration is:

1. Create `GatewayHealthProbe` implementing `HealthProbe`:
   ```rust
   pub(crate) struct GatewayHealthProbe { /* ... */ }

   #[async_trait]
   impl HealthProbe for GatewayHealthProbe {
       async fn probe(&self, endpoint: &CosmosEndpoint) -> ProbeResult {
           // GET {endpoint}/_health (or whatever the API is)
           // Parse response for per-region health details
       }
   }
   ```

2. Update the probe construction in `CosmosDriver` initialization to use
   `GatewayHealthProbe` instead of `GetDatabaseAccountProbe`.

3. **Nothing else changes.** The monitor, sweep loop, retry logic, state management,
   configuration, and diagnostics all remain identical.

This is the key design benefit of the `HealthProbe` trait abstraction.

---

## 8. Shutdown & Resource Cleanup

The `EndpointHealthMonitor` uses a `CancellationToken` to signal the background task to
stop. When the `CosmosDriver` is dropped:

1. The `CancellationToken` is cancelled.
2. The `JoinHandle` is dropped (the sweep loop exits on next `tokio::select!`).
3. Any in-flight probe requests complete naturally (they have a timeout).

No explicit `shutdown()` method is needed — the RAII pattern via `Drop` handles cleanup.

---

## 9. Testing Strategy

### 9.1 Unit Tests

| Test | What it validates |
|------|-------------------|
| `endpoints_to_probe_deduplicates` | Read + write overlap → no duplicate probes |
| `endpoints_to_probe_empty_state` | No endpoints → empty probe list |
| `probe_with_retry_succeeds_first_try` | Healthy response → no retries |
| `probe_with_retry_succeeds_after_failures` | Retries until success |
| `probe_with_retry_exhausts_retries` | All retries fail → returns Unhealthy |
| `sweep_marks_healthy_available` | Previously unavailable endpoint passes probe → removed from unavailable set |
| `sweep_marks_unhealthy_unavailable` | Failed probe → added to unavailable set |
| `sweep_atomic_state_update` | Multiple probe results applied in single state swap |
| `mark_endpoint_available_no_op` | Already-available endpoint → state unchanged |
| `config_env_var_overrides` | Env vars override defaults |

### 9.2 Integration Tests

| Test | What it validates |
|------|-------------------|
| `startup_probe_populates_state` | After driver init, unavailable endpoints reflect actual reachability |
| `background_sweep_detects_recovery` | Mark endpoint unavailable → probe succeeds → endpoint marked available |
| `health_check_with_fault_injection` | Inject transport error → probe fails → endpoint unavailable |

### 9.3 Mock Probe for Testing

```rust
/// Test-only health probe that returns preconfigured results per endpoint.
#[cfg(test)]
pub(crate) struct MockHealthProbe {
    results: HashMap<CosmosEndpoint, ProbeResult>,
}

#[cfg(test)]
#[async_trait]
impl HealthProbe for MockHealthProbe {
    async fn probe(&self, endpoint: &CosmosEndpoint) -> ProbeResult {
        self.results
            .get(endpoint)
            .cloned()
            .unwrap_or(ProbeResult::Healthy {
                latency: Duration::from_millis(1),
            })
    }
}
```

---

## 10. Migration Plan

This feature is part of the Transport Pipeline Spec migration. It can be implemented as
a standalone step that depends on Step 2 (multi-region failover & endpoint management)
since it requires `AccountEndpointState` and `AccountEndpointStateStore`.

| Sub-step | Work Item | Files |
|----------|-----------|-------|
| HC.1 | **`HealthProbe` trait + `ProbeResult`** — Define the swappable probe interface. | `driver/health/probe.rs` |
| HC.2 | **`GetDatabaseAccountProbe`** — Implement using existing metadata pipeline. | `driver/health/get_database_account_probe.rs` |
| HC.3 | **`HealthCheckOptions`** — Public configuration surface. | `options/health.rs` |
| HC.4 | **`EndpointHealthMonitor`** — Sweep loop, concurrent probing, retry logic. | `driver/health/monitor.rs`, `driver/health/config.rs` |
| HC.5 | **`mark_endpoint_available`** — New system function for endpoint recovery. | `driver/routing/account_endpoint_state.rs` |
| HC.6 | **Wire into `CosmosDriver`** — Create probe + monitor during driver init. | `driver/cosmos_driver.rs` |
| HC.7 | **Startup probing** — Run initial sweep before driver accepts requests. | `driver/health/monitor.rs` |
| HC.8 | **Env var overrides** — Read `AZURE_COSMOS_HEALTH_CHECK_*` vars at config construction. | `driver/health/config.rs` |
| HC.9 | **Unit tests** — Probe, retry, sweep, state update, config override tests. | `driver/health/tests/` |
| HC.10 | **Integration tests** — End-to-end with fault injection. | `tests/` |

### Dependency

- **Requires**: Transport Pipeline Spec Step 2 (`AccountEndpointState`,
  `AccountEndpointStateStore`, `CosmosEndpoint`, `UnavailableReason`).
- **Enables**: Faster failover and recovery in Steps 3+ (circuit breaker, hedging).

---

## 11. Open Questions

| # | Question | Status |
|---|----------|--------|
| 1 | Should the startup probe block driver creation or run in the background? The Python SDK runs it in a background daemon. Blocking gives better first-request guarantees but adds latency to client construction. | **Proposed: blocking** (matches user expectation that client is ready after construction) |
| 2 | Should health checks be disabled for emulator mode? The emulator is single-region and always localhost. | **Proposed: yes** (skip when connection string is `"emulator"`) |
| 3 | Should the health monitor also trigger an `AccountMetadataCache` refresh on sweep? The Python SDK re-fetches account properties alongside health checks. | **Proposed: yes** (piggyback on the `GetDatabaseAccount` response to refresh topology) |
| 4 | What is the gateway health API contract? Need to coordinate with the gateway team for the `GatewayHealthProbe` implementation. | **Deferred** until API is available |
| 5 | Should `HealthCheckOptions` be `pub` on `DriverOptions` or only exposed via builder? | **Proposed: both** (field on options + builder setter for ergonomics) |
