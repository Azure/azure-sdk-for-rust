# Gateway Endpoint Health Checks Specification

**Issue**: [Azure/azure-sdk-for-rust#3846](https://github.com/Azure/azure-sdk-for-rust/issues/3846)
**Status**: Draft

> **Step 2 Dependency**: This spec depends on Transport Pipeline Spec Step 2
> (`AccountEndpointState`, `AccountEndpointStateStore`, `AccountEndpoint`-based
> routing with `preferred_read_endpoints` / `preferred_write_endpoints`).
> Code examples referencing these types are **pseudocode** until Step 2 lands.
> Step 1 (PR #3875) established the pipeline architecture; this spec aligns with
> the new pipeline design (plain async functions, no azure_core policy pipeline).

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
3. **Startup probing** — verifying endpoint reachability in the background during client initialization

The `AccountEndpointState`, `AccountEndpointStateStore`, and
`expire_unavailable_endpoints` systems defined in §4.4 are consumed as-is (once
implemented in Step 2). This spec adds `mark_endpoint_available`,
`mark_endpoint_unavailable`, and the orchestration layer.

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
        /// Structured failure details for diagnostics integration.
        failure: ProbeFailure,
        /// How long before the probe failed/timed out.
        latency: Duration,
    },
}

/// Structured details about a probe failure, providing the information
/// needed by the `DiagnosticsContext` system without string parsing.
#[derive(Clone, Debug)]
pub(crate) struct ProbeFailure {
    /// Cosmos status (HTTP status code + optional sub-status code),
    /// if the probe received an HTTP response.
    pub status: Option<CosmosStatus>,
    /// Human-readable description of the failure.
    pub message: String,
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
    async fn probe(&self, endpoint: &AccountEndpoint) -> ProbeResult;
}
```

### 3.2 `GetDatabaseAccountProbe` — Current Implementation

Uses `execute_transport_pipeline` from the new pipeline architecture (Step 1, PR #3875) to
issue a `GET /` (GetDatabaseAccount) against a specific regional endpoint:

```rust
// driver/health/get_database_account_probe.rs

/// Health probe that calls GetDatabaseAccount on the target endpoint.
///
/// This is the initial implementation. When the gateway health API ships,
/// a new `GatewayHealthProbe` will replace this — only this struct changes,
/// not the orchestration.
pub(crate) struct GetDatabaseAccountProbe {
    /// The transport layer — provides raw HttpClient for metadata requests.
    transport: Arc<CosmosTransport>,
    /// Credential for the account (master key or AAD token).
    credential: Credential,
    /// User agent header value.
    user_agent: HeaderValue,
}

impl GetDatabaseAccountProbe {
    pub fn new(
        transport: Arc<CosmosTransport>,
        credential: Credential,
        user_agent: HeaderValue,
    ) -> Self {
        Self {
            transport,
            credential,
            user_agent,
        }
    }
}

#[async_trait]
impl HealthProbe for GetDatabaseAccountProbe {
    async fn probe(&self, endpoint: &AccountEndpoint) -> ProbeResult {
        let start = Instant::now();
        let http_client = self.transport.get_metadata_http_client(endpoint);

        // Build a TransportRequest for GetDatabaseAccount (GET /).
        let auth_context = AuthorizationContext::new(
            Method::Get,
            ResourceType::DatabaseAccount,
            "",
        );
        let transport_request = TransportRequest {
            method: Method::Get,
            url: endpoint.url().clone(),
            region: None,
            headers: Headers::new(),
            body: None,
            auth_context,
            execution_context: ExecutionContext::HealthCheckProbe,
            // Per-request timeout will be wired when PR #3871 lands.
            // Until then, uses client-level metadata timeout.
            deadline: None,
        };

        // Use a lightweight DiagnosticsContextBuilder for the probe.
        let mut diagnostics = DiagnosticsContextBuilder::new(
            ActivityId::new(),
            PipelineType::Metadata,
            TransportSecurity::Tls,
        );

        let result = execute_transport_pipeline(
            transport_request,
            http_client.as_ref(),
            &self.credential,
            &self.user_agent,
            PipelineType::Metadata,
            TransportSecurity::Tls,
            &mut diagnostics,
        )
        .await;

        let latency = start.elapsed();
        match result.outcome {
            TransportOutcome::Success { status, .. } => {
                ProbeResult::Healthy { latency }
            }
            TransportOutcome::HttpError { status, .. } => {
                ProbeResult::Unhealthy {
                    failure: ProbeFailure {
                        status: Some(status),
                        message: format!("HTTP {}", status.status_code()),
                    },
                    latency,
                }
            }
            TransportOutcome::TransportError { error, .. } => {
                ProbeResult::Unhealthy {
                    failure: ProbeFailure {
                        status: None,
                        message: format!("transport error: {error}"),
                    },
                    latency,
                }
            }
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

    /// Maximum number of probe attempts per endpoint per sweep.
    /// Default: 3 (3 total attempts), matching the Python SDK.
    /// Used by the probe's own retry loop; the transport pipeline handles
    /// 429 throttle retries independently.
    pub max_probe_attempts: u32,
}

impl Default for HealthMonitorConfig {
    fn default() -> Self {
        Self {
            refresh_interval: Duration::from_secs(300),
            max_probe_attempts: 3,
        }
    }
}

/// Manages background health checking of gateway endpoints.
///
/// Spawned by `CosmosDriver` during initialization. Runs a periodic
/// sweep that probes all relevant endpoints and updates the
/// `AccountEndpointStateStore`.
///
/// On drop, the `CancellationToken` is cancelled and the background
/// task exits (see §8).
pub(crate) struct EndpointHealthMonitor {
    config: HealthMonitorConfig,
    probe: Arc<dyn HealthProbe>,
    endpoint_state_store: Arc<AccountEndpointStateStore>,
    account_metadata_cache: Arc<AccountMetadataCache>,
    /// Cancellation token to signal the background task to stop.
    shutdown: CancellationToken,
    /// Handle to the background sweep task. Dropped on shutdown.
    _sweep_handle: JoinHandle<()>,
}

impl Drop for EndpointHealthMonitor {
    fn drop(&mut self) {
        self.shutdown.cancel();
    }
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
fn endpoints_to_probe(state: &AccountEndpointState) -> Vec<AccountEndpoint> {
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

### 3.5 Probe Retry Strategy

Health probes use a two-layer retry approach matching the pipeline architecture:

1. **Transport-level (429 throttling)**: Handled automatically by
   `execute_transport_pipeline()` — the probe benefits from this for free.
2. **Probe-level (transient failures)**: The health monitor retries failed probes
   up to `max_probe_retries` times with exponential backoff. This is a simple retry
   loop in the sweep, not a shared policy object — the pipeline's operation-level
   retry (`OperationRetryState`) is designed for user-facing operations and not
   suitable for background probes.

```rust
// Probe retry is a simple loop in run_sweep():
for attempt in 0..config.max_probe_attempts {
    let result = probe.probe(&ep).await;
    match &result {
        ProbeResult::Healthy { .. } => return result,
        ProbeResult::Unhealthy { .. } if attempt + 1 < config.max_probe_attempts => {
            tokio::time::sleep(backoff_delay(attempt)).await;
        }
        _ => return result,
    }
}
```

### 3.6 Background Sweep Loop

```rust
// driver/health/monitor.rs

impl EndpointHealthMonitor {
    /// The main background loop. Runs an initial sweep immediately,
    /// then periodically until the monitor is dropped.
    ///
    /// Uses `azure_core` async primitives (not tokio directly) to remain
    /// async-runtime agnostic.
    async fn sweep_loop(
        config: HealthMonitorConfig,
        probe: Arc<dyn HealthProbe>,
        endpoint_state_store: Arc<AccountEndpointStateStore>,
        account_metadata_cache: Arc<AccountMetadataCache>,
        shutdown: CancellationToken,
    ) {
        // Run initial sweep immediately on startup.
        Self::run_sweep(
            &config,
            &*probe,
            &endpoint_state_store,
            &account_metadata_cache,
        )
        .await;

        loop {
            // Wait for either the refresh interval to elapse or shutdown,
            // using azure_core async primitives (runtime-agnostic).
            let sleep = azure_core::task::sleep(config.refresh_interval);
            let cancelled = shutdown.cancelled();
            futures::pin_mut!(sleep, cancelled);

            match futures::future::select(sleep, cancelled).await {
                futures::future::Either::Left(_) => {},
                futures::future::Either::Right(_) => break,
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
        // 429 throttle retries are handled by execute_transport_pipeline().
        // Probe-level retries are handled by the sweep (see §3.5).
        let probe_futures: Vec<_> = endpoints
            .iter()
            .map(|ep| {
                let ep = ep.clone();
                let probe = probe; // borrow, not move
                async move {
                    let result = probe.probe(&ep).await;
                    (ep, result)
                }
            })
            .collect();

        let results = futures::future::join_all(probe_futures).await;

        // Apply results per-endpoint against the latest state to avoid
        // overwriting concurrent reactive updates (no snapshot+swap).
        for (endpoint, result) in &results {
            match result {
                ProbeResult::Healthy { .. } => {
                    endpoint_state_store.mark_available(endpoint);
                }
                ProbeResult::Unhealthy { failure, .. } => {
                    endpoint_state_store.mark_unavailable(
                        endpoint,
                        UnavailableReason::ServiceUnavailable,
                    );
                    tracing::warn!(
                        endpoint = %endpoint,
                        reason = %failure.message,
                        status_code = ?failure.status.as_ref().map(|s| s.status_code()),
                        sub_status_code = ?failure.status.as_ref().and_then(|s| s.sub_status()),
                        "health probe failed, marking endpoint unavailable"
                    );
                }
            }
        }
    }
}
```

### 3.7 Startup Probing

On `CosmosDriver` initialization, after the initial `GetDatabaseAccount` succeeds and
populates the `AccountMetadataCache`, the health monitor spawns the background sweep loop
which immediately runs the first sweep. The driver does **not** block on the initial sweep
— the client is usable immediately after construction. The reactive retry/failover path
covers the window before the first sweep completes.

```rust
// driver/health/monitor.rs

impl EndpointHealthMonitor {
    /// Creates and starts the health monitor.
    ///
    /// Spawns a background task that runs an initial sweep immediately,
    /// then continues with periodic sweeps. Does not block — the driver
    /// is ready to accept requests before the first sweep completes.
    pub fn start(
        config: HealthMonitorConfig,
        probe: Arc<dyn HealthProbe>,
        endpoint_state_store: Arc<AccountEndpointStateStore>,
        account_metadata_cache: Arc<AccountMetadataCache>,
    ) -> Self {
        let shutdown = CancellationToken::new();
        let sweep_handle = azure_core::task::spawn(Self::sweep_loop(
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
            shutdown,
            _sweep_handle: sweep_handle,
        }
    }
}
```

### 3.8 `mark_endpoint_available` System Function

New system function for endpoint recovery, part of the Step 2 endpoint state system
(alongside `mark_endpoint_unavailable` which is also new in Step 2):

```rust
// driver/routing/account_endpoint_state.rs (new in Step 2)

/// Produce a new state with an endpoint marked as available (removed from
/// the unavailable set).
///
/// If the endpoint is not in the unavailable set, the state is returned
/// unchanged.
fn mark_endpoint_available(
    state: &AccountEndpointState,
    endpoint: &AccountEndpoint,
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
    account.credential().clone(),
    user_agent.clone(),
));

// Start health monitor (non-blocking — spawns background sweep immediately).
let health_monitor = EndpointHealthMonitor::start(
    health_config,
    health_probe,
    Arc::clone(&endpoint_state_store),
    Arc::clone(&self.runtime.account_metadata_cache()),
);
```

### 4.2 `execute_operation_pipeline` Integration

The operation pipeline (`execute_operation_pipeline`) already reads from
`AccountEndpointStateStore` (via `resolve_endpoint` in the Transport Pipeline Spec).
Health check results automatically influence routing because the sweep updates the same
`unavailable_endpoints` map that `resolve_endpoint` reads.

No changes to `execute_operation_pipeline` are required. The integration is purely through
shared state.

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
- Both apply per-endpoint mutations to the same store (no lost updates).

### 4.4 Diagnostics Integration

Health probe activity is recorded in the `DiagnosticsContext` system using a dedicated
`ExecutionContext::HealthCheckProbe` variant. This is distinct from
`ExecutionContext::CircuitBreakerProbe` (used by the partition-level circuit breaker /
PPCB), since health checks and PPCB are independent features.

Each probe result is recorded as a diagnostic event containing:
- **Endpoint URI**: The regional gateway endpoint that was probed.
- **HTTP status code**: When the probe received an HTTP response.
- **Sub-status code**: When present in response headers (`x-ms-substatus`).
- **Latency**: How long the probe took.
- **Outcome**: Healthy or Unhealthy with the `ProbeFailure` details.

```rust
// When logging probe attempts:
tracing::debug!(
    endpoint = %endpoint,
    execution_context = "HealthCheckProbe",
    latency_ms = latency.as_millis(),
    status_code = ?failure.as_ref().and_then(|f| f.status.as_ref().map(|s| s.status_code())),
    sub_status_code = ?failure.as_ref().and_then(|f| f.status.as_ref().and_then(|s| s.sub_status())),
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
///
/// Health checks are always enabled and always run on startup (non-blocking).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct HealthCheckOptions {
    /// Interval between background health sweeps. Default: 300s (5 minutes).
    refresh_interval: Duration,
    /// Maximum probe attempts per endpoint per sweep (probe-level retry loop).
    /// Default: 3.
    max_probe_attempts: u32,
}

impl Default for HealthCheckOptions {
    fn default() -> Self {
        Self {
            refresh_interval: Duration::from_secs(300),
            max_probe_attempts: 3,
        }
    }
}

impl HealthCheckOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_refresh_interval(mut self, interval: Duration) -> Self {
        self.refresh_interval = interval;
        self
    }

    pub fn with_max_probe_attempts(mut self, attempts: u32) -> Self {
        self.max_probe_attempts = attempts;
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

Configuration is programmatic-only via `HealthCheckOptions`. There are no environment
variable overrides — all tuning is done in code.

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
       async fn probe(&self, endpoint: &AccountEndpoint) -> ProbeResult {
           // GET {endpoint}/_health (or whatever the API is)
           // Parse response for per-region health details
       }
   }
   ```

2. Update the probe construction in `CosmosDriver` initialization to use
   `GatewayHealthProbe` instead of `GetDatabaseAccountProbe`.

3. **Nothing else changes.** The monitor, sweep loop, state management,
   configuration, and diagnostics all remain identical.

This is the key design benefit of the `HealthProbe` trait abstraction.

---

## 8. Shutdown & Resource Cleanup

The `EndpointHealthMonitor` implements `Drop` to cancel the background task.
When the monitor is dropped:

1. `Drop::drop` cancels the `CancellationToken`.
2. The sweep loop detects cancellation and exits.
3. Any in-flight probe requests complete naturally (they have a client-level timeout).

No explicit `shutdown()` method is needed — the RAII pattern via `Drop` handles cleanup.

---

## 9. Testing Strategy

### 9.1 Unit Tests

| Test | What it validates |
|------|-------------------|
| `endpoints_to_probe_deduplicates` | Read + write overlap → no duplicate probes |
| `endpoints_to_probe_empty_state` | No endpoints → empty probe list |
| `sweep_marks_healthy_available` | Previously unavailable endpoint passes probe → removed from unavailable set |
| `sweep_marks_unhealthy_unavailable` | Failed probe → added to unavailable set |
| `sweep_atomic_state_update` | Multiple probe results applied in single state swap |
| `mark_endpoint_available_no_op` | Already-available endpoint → state unchanged |
| `probe_failure_captures_status_codes` | `ProbeFailure` contains HTTP status and sub-status from response |
| `startup_sweep_runs_immediately` | Background sweep loop executes first sweep without waiting for interval |

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
    results: HashMap<AccountEndpoint, ProbeResult>,
}

#[cfg(test)]
#[async_trait]
impl HealthProbe for MockHealthProbe {
    async fn probe(&self, endpoint: &AccountEndpoint) -> ProbeResult {
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
| HC.1 | **`HealthProbe` trait + `ProbeResult` + `ProbeFailure`** — Define the swappable probe interface with structured failure details. | `driver/health/probe.rs` |
| HC.2 | **`GetDatabaseAccountProbe`** — Implement using `execute_transport_pipeline()` and `get_metadata_http_client()`. | `driver/health/get_database_account_probe.rs` |
| HC.3 | **`HealthCheckOptions`** — Public configuration surface (no env vars, no enabled toggle). | `options/health.rs` |
| HC.4 | **`EndpointHealthMonitor`** — Sweep loop with concurrent probing and probe-level retry. 429 retries handled by transport pipeline. | `driver/health/monitor.rs`, `driver/health/config.rs` |
| HC.5 | **`mark_endpoint_available`** — New system function for endpoint recovery. | `driver/routing/account_endpoint_state.rs` |
| HC.6 | **Wire into `CosmosDriver`** — Create probe + monitor during driver init (non-blocking). | `driver/cosmos_driver.rs` |
| HC.7 | **`ExecutionContext::HealthCheckProbe`** — New diagnostics variant, separate from PPCB. | `driver/diagnostics/` |
| HC.8 | **Unit tests** — Probe, sweep, state update, structured failure tests. | `driver/health/tests/` |
| HC.9 | **Integration tests** — End-to-end with fault injection. | `tests/` |

### Dependency

- **Requires**: Transport Pipeline Spec Step 2 (`AccountEndpointState`,
  `AccountEndpointStateStore`, `AccountEndpoint`-based routing, `UnavailableReason`).
  Step 1 (PR #3875) provides the pipeline architecture used by probes.
- **Enables**: Faster failover and recovery in Steps 3+ (circuit breaker, hedging).

---

## 11. Open Questions

| # | Question | Status |
|---|----------|--------|
| 1 | Should the startup probe block driver creation or run in the background? The Python SDK runs it in a background daemon. Blocking gives better first-request guarantees but adds latency to client construction. | **Resolved: non-blocking.** Startup probe runs in the background. The reactive retry/failover path covers the window before the first sweep completes. |
| 2 | Should health checks be disabled for emulator mode? The emulator is single-region and always localhost. | **Resolved: keep enabled.** Health checks run against the emulator too. This provides limited value (single-region, localhost) but avoids code-path divergence and lets emulator tests exercise the health check path. |
| 3 | Should the health monitor also trigger an `AccountMetadataCache` refresh on sweep? The Python SDK re-fetches account properties alongside health checks. | **Open.** Leaning toward decoupled (health sweep only updates endpoint availability, metadata refresh stays on its own schedule) but subject to further discussion. |
| 4 | What is the gateway health API contract? Need to coordinate with the gateway team for the `GatewayHealthProbe` implementation. | **Deferred** until API is available |
| 5 | Should `HealthCheckOptions` be `pub` on `DriverOptions` or only exposed via builder? | **Proposed: both** (field on options + builder setter for ergonomics) |
