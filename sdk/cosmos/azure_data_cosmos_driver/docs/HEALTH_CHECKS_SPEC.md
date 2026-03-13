# Gateway Endpoint Health Checks Specification

**Issue**: [Azure/azure-sdk-for-rust#3846](https://github.com/Azure/azure-sdk-for-rust/issues/3846)
**Status**: Draft

> **Step 2 Dependency**: This spec depends on Transport Pipeline Spec Step 2
> (`AccountEndpointState`, `AccountEndpointStateStore`, `CosmosEndpoint`,
> `LocationEffect`, `LocationStateStore`).
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

### Design Principles

- **Healthy by default**: Endpoints are always eligible for routing unless explicitly
  excluded by the customer via excluded regions. Transient unavailability may deprioritize
  an endpoint but never permanently removes it from the routing pool. This avoids issues
  seen in the Python SDK where transient failures could remove an endpoint for the lifetime
  of the process.
- **No public API**: Health check configuration is fully internal with hardcoded defaults.
  No `HealthCheckOptions` is exposed to users.
- **Runtime-agnostic**: Uses `azure_core` async primitives and the existing
  `BackgroundTaskManager` for task lifecycle, not tokio directly.

### Non-Goals

- **Partition-level health / circuit breaker**: Covered separately in Transport Pipeline
  Spec §4.5. Health checks here are **endpoint-level** (regional gateway availability).
- **Connection-pool shard health**: Covered in Transport Pipeline Spec §6.5–6.6 (per-shard
  eviction). That is a transport-layer concern; this spec is about service-level reachability.
- **User-facing health API**: Health checks are internal. No public API is exposed.

### Differences from Python SDK

The Python SDK does **not** have an independent background health sweep. Instead:
- `_refresh_endpoint_list_private()` is triggered every 5 minutes on the request path
- Health checks are piggybacked on metadata refresh, not run independently

The Rust driver uses an **independent background sweep** that runs regardless of request
patterns. This is an intentional architectural improvement — it provides more reliable
probing for applications with bursty or idle request patterns, where piggybacking on
request-triggered refresh would leave long gaps without health information.

---

## 2. Architecture Overview

```text
┌─────────────────────────────────────────────────────────────┐
│                    CosmosDriverRuntime                       │
│                                                             │
│  ┌─────────────────┐  ┌──────────────────────────────────┐  │
│  │ AccountMetadata  │  │ EndpointHealthMonitor            │  │
│  │ Cache            │  │                                  │  │
│  │ (account props)  │  │  • owns background task          │  │
│  └─────────────────┘  │  • schedules probes              │  │
│                        │  • emits LocationEffects          │  │
│                        │                                  │  │
│                        │  ┌────────────────────────────┐  │  │
│                        │  │ HealthProbe (enum)          │  │  │
│                        │  │  • GetAccountMetadata       │  │  │
│                        │  │  • (future: HealthEndpoint) │  │  │
│                        │  └────────────────────────────┘  │  │
│                        └──────────────────────────────────┘  │
│                                     │                        │
│                                     ▼                        │
│                        ┌──────────────────────────────────┐  │
│                        │ LocationStateStore                │  │
│                        │   apply(LocationEffect)           │  │
│                        └──────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### Relationship to Transport Pipeline Spec

This spec extends the Transport Pipeline Spec (§4.4) by adding:
1. **Proactive probing** — a background task that calls a health probe on each endpoint
2. **Availability recovery** — marking endpoints available again after successful probes
3. **Startup probing** — verifying endpoint reachability in the background during client
   initialization

This spec requires amendments to the Transport Pipeline Spec:
- **New `LocationEffect` variants**: `LocationEffect::MarkEndpointAvailable` and
  `LocationEffect::MarkEndpointUnavailable` (routed through `LocationStateStore::apply()`
  for consistency with the single-writer model)
- **New `ExecutionContext` variant**: `ExecutionContext::HealthCheckProbe`
- **Expiration replaced**: `expire_unavailable_endpoints` is no longer needed as a
  recovery mechanism — health probes handle recovery directly. Endpoint availability
  is determined solely by health checks.

---

## 3. Component Design

### 3.1 `HealthProbe` Enum

The probe is an enum (not a trait) following [DOP guidance on `dyn Trait`](https://analogrelay.github.io/dop-in-rust/dyn_trait/01-dyn-trait.html#when-not-to-use-dyn-trait-continued).
This isn't user-extensible — the set of probe strategies is known and closed:

```rust
// driver/health/probe.rs

/// Health probe strategy for testing endpoint reachability.
///
/// Uses an enum rather than a trait because the set of probe strategies
/// is closed and not user-extensible.
pub(crate) enum HealthProbe {
    /// Probe via GetDatabaseAccount (GET /) — the initial implementation.
    GetAccountMetadata {
        transport: Arc<CosmosTransport>,
        credential: Credential,
        user_agent: HeaderValue,
    },
    // Future: HealthEndpoint { url_pattern: ... } when the gateway
    // health API ships. Discovery mechanism TBD (see Open Question #4).
}

impl HealthProbe {
    /// Probe the given endpoint and return whether it is healthy.
    ///
    /// Uses `execute_operation` with all other regions excluded to reuse
    /// the full pipeline (retry, diagnostics, signing).
    async fn probe(&self, endpoint: &CosmosEndpoint) -> ProbeResult {
        match self {
            HealthProbe::GetAccountMetadata { .. } => {
                self.probe_get_account_metadata(endpoint).await
            }
        }
    }
}
```

### 3.2 `ProbeResult`

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

/// Structured details about a probe failure.
///
/// Carries the actual error for deferred formatting — the error is only
/// converted to a string when needed for logging or display.
#[derive(Clone, Debug)]
pub(crate) struct ProbeFailure {
    /// Cosmos status (HTTP status code + optional sub-status code),
    /// if the probe received an HTTP response.
    pub status: Option<CosmosStatus>,
    /// The underlying error. Formatting is deferred until display time.
    pub error: azure_core::Error,
    /// Number of consecutive failures for this endpoint (across sweeps).
    pub consecutive_failures: u32,
}
```

### 3.3 Probe Implementation — `GetAccountMetadata`

Uses `execute_operation` with all regions excluded except the target endpoint. This
reuses the full pipeline (retry, diagnostics, signing) and avoids duplicating
transport-level code:

```rust
// driver/health/probe.rs

impl HealthProbe {
    async fn probe_get_account_metadata(
        &self,
        endpoint: &CosmosEndpoint,
    ) -> ProbeResult {
        let start = Instant::now();

        // Execute a GetDatabaseAccount operation scoped to this endpoint
        // by excluding all other regions. This reuses the full pipeline:
        // signing, headers, transport retry (429), diagnostics.
        let result = execute_operation(
            GetDatabaseAccountOperation::new(),
            &OperationOptions {
                excluded_regions: all_regions_except(endpoint.region()),
                execution_context: ExecutionContext::HealthCheckProbe,
                ..Default::default()
            },
        )
        .await;

        let latency = start.elapsed();
        match result {
            Ok(_) => ProbeResult::Healthy { latency },
            Err(error) => {
                let status = extract_cosmos_status(&error);
                ProbeResult::Unhealthy {
                    failure: ProbeFailure {
                        status,
                        error,
                        consecutive_failures: 0, // Updated by the monitor
                    },
                    latency,
                }
            }
        }
    }
}
```

### 3.4 `EndpointHealthMonitor` — Orchestration

The monitor is the central coordinator. It owns the background task via
`BackgroundTaskManager` (the same abstraction used by `GlobalPartitionEndpointManager`
and `GlobalEndpointManager`). Dropping the monitor drops all stored futures, which
cancels the background task via Rust's drop semantics — no `CancellationToken` needed.

```rust
// driver/health/monitor.rs

/// Manages background health checking of gateway endpoints.
///
/// Spawned by `CosmosDriver` during initialization. Runs a periodic
/// sweep that probes all relevant endpoints and applies results as
/// `LocationEffect`s through the `LocationStateStore`.
///
/// Dropping the monitor cancels the background task automatically
/// via `BackgroundTaskManager`'s drop-based cancellation (see §8).
pub(crate) struct EndpointHealthMonitor {
    probe: HealthProbe,
    location_state_store: Arc<LocationStateStore>,
    /// Tracks consecutive failures per endpoint across sweeps.
    failure_counts: Arc<Mutex<HashMap<CosmosEndpoint, u32>>>,
    /// Background task manager — dropping this cancels the sweep task.
    task_manager: BackgroundTaskManager,
}
```

### 3.5 Endpoint Selection — Which Endpoints to Probe

Following the Python SDK pattern, the monitor probes:

1. All **read endpoints** in the preferred locations list.
2. The **first write endpoint** (unless it's already in the read set).
3. Deduplicates so no endpoint is probed twice per sweep.

The **global endpoint** is never probed — it is used only for initial account topology
discovery (`GetDatabaseAccount` during driver init).

```rust
// driver/health/monitor.rs (pure function)

/// Determines which endpoints should be health-checked.
///
/// Returns a deduplicated list of regional endpoints. The global endpoint
/// is excluded — it is only used for topology discovery, not health probing.
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

### 3.6 Probe Retry Strategy

Health probes use a two-layer retry approach:

1. **Transport-level (429 throttling)**: Handled automatically by the transport pipeline
   within `execute_operation` — the probe benefits from this for free.
2. **Probe-level (transient failures)**: The sweep retries failed probes up to
   `MAX_PROBE_ATTEMPTS` times with exponential backoff.

Retry parameters (matching Python SDK):
- **Max attempts**: 3 (total, not additional)
- **Initial delay**: 500ms
- **Backoff factor**: 2x exponential
- **Max delay cap**: 3 minutes (180s)
- **Jitter**: ±25% via the existing `jitter::with_jitter` helper

> **Note**: When PR #3871 lands per-request timeout support, consider scaling the probe
> timeout on each retry attempt (as the Python SDK does with `timeout ** 2`), giving
> transient issues more time to resolve on later attempts.

```rust
/// Hardcoded probe retry defaults (matching Python SDK).
const MAX_PROBE_ATTEMPTS: u32 = 3;
const INITIAL_RETRY_DELAY: Duration = Duration::from_millis(500);
const BACKOFF_FACTOR: f64 = 2.0;
const MAX_RETRY_DELAY: Duration = Duration::from_secs(180);
const BACKOFF_JITTER_RATIO: f64 = 0.25;

/// Probes a single endpoint with retries.
async fn probe_with_retry(
    probe: &HealthProbe,
    endpoint: &CosmosEndpoint,
) -> ProbeResult {
    let mut delay = INITIAL_RETRY_DELAY;

    for attempt in 0..MAX_PROBE_ATTEMPTS {
        let result = probe.probe(endpoint).await;
        match &result {
            ProbeResult::Healthy { .. } => return result,
            ProbeResult::Unhealthy { .. } if attempt + 1 < MAX_PROBE_ATTEMPTS => {
                let jittered = Duration::from_secs_f64(
                    with_jitter(delay.as_secs_f64(), BACKOFF_JITTER_RATIO),
                );
                azure_core::task::sleep(jittered).await;
                delay = (delay.mul_f64(BACKOFF_FACTOR)).min(MAX_RETRY_DELAY);
            }
            _ => return result,
        }
    }

    unreachable!("loop always returns")
}
```

### 3.7 Background Sweep Loop

```rust
// driver/health/monitor.rs

impl EndpointHealthMonitor {
    /// The main background loop. Runs an initial sweep immediately,
    /// then periodically (with jitter) until dropped.
    ///
    /// Uses `azure_core` async primitives and `futures` for
    /// runtime-agnostic execution.
    async fn sweep_loop(
        probe: HealthProbe,
        location_state_store: Arc<LocationStateStore>,
        failure_counts: Arc<Mutex<HashMap<CosmosEndpoint, u32>>>,
    ) {
        // Run initial sweep immediately on startup.
        Self::run_sweep(&probe, &location_state_store, &failure_counts).await;

        loop {
            // Jitter ±10% to avoid thundering herd across driver instances.
            let interval = Duration::from_secs_f64(
                with_jitter(REFRESH_INTERVAL.as_secs_f64(), 0.1),
            );
            azure_core::task::sleep(interval).await;

            Self::run_sweep(&probe, &location_state_store, &failure_counts).await;
        }
    }

    /// Executes a single health sweep across all relevant endpoints.
    ///
    /// Steps:
    /// 1. Snapshot current endpoint state.
    /// 2. Determine which endpoints to probe.
    /// 3. Probe all endpoints concurrently, processing results as they arrive.
    /// 4. Apply results as `LocationEffect`s through `LocationStateStore`.
    ///
    /// Note: If a metadata refresh adds/removes regions mid-sweep, probe
    /// results may target stale endpoints. This is safe — `mark_available`
    /// on a removed region is a no-op (only removes from unavailable set).
    async fn run_sweep(
        probe: &HealthProbe,
        location_state_store: &LocationStateStore,
        failure_counts: &Mutex<HashMap<CosmosEndpoint, u32>>,
    ) {
        let state = location_state_store.snapshot();
        let endpoints = endpoints_to_probe(&state);

        if endpoints.is_empty() {
            return;
        }

        // Probe all endpoints concurrently, processing results as they arrive
        // via FuturesUnordered (runtime-agnostic alternative to tokio::JoinSet).
        let mut probes = FuturesUnordered::new();
        for ep in &endpoints {
            let ep = ep.clone();
            let probe = &probe;
            probes.push(async move {
                let result = probe_with_retry(probe, &ep).await;
                (ep, result)
            });
        }

        let mut counts = failure_counts.lock().unwrap();
        while let Some((endpoint, result)) = probes.next().await {
            match &result {
                ProbeResult::Healthy { latency } => {
                    let was_unhealthy = counts.remove(&endpoint).unwrap_or(0) > 0;
                    location_state_store.apply(
                        LocationEffect::MarkEndpointAvailable(endpoint.clone()),
                    );
                    if was_unhealthy {
                        tracing::info!(
                            endpoint = %endpoint,
                            latency_ms = latency.as_millis(),
                            "health probe recovered, marking endpoint available"
                        );
                    }
                }
                ProbeResult::Unhealthy { failure, latency } => {
                    let count = counts.entry(endpoint.clone()).or_insert(0);
                    *count += 1;
                    location_state_store.apply(
                        LocationEffect::MarkEndpointUnavailable(
                            endpoint.clone(),
                            UnavailableReason::ServiceUnavailable,
                        ),
                    );
                    tracing::warn!(
                        endpoint = %endpoint,
                        consecutive_failures = *count,
                        latency_ms = latency.as_millis(),
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

### 3.8 Startup Probing

On `CosmosDriver` initialization, after the initial `GetDatabaseAccount` succeeds, the
health monitor spawns the background sweep loop which immediately runs the first sweep.
The driver does **not** block on the initial sweep — the client is usable immediately
after construction. The reactive retry/failover path covers the window before the first
sweep completes.

```rust
// driver/health/monitor.rs

/// Hardcoded sweep interval (matching Python SDK).
const REFRESH_INTERVAL: Duration = Duration::from_secs(300);

impl EndpointHealthMonitor {
    /// Creates and starts the health monitor.
    ///
    /// Spawns a background task via `BackgroundTaskManager` that runs an
    /// initial sweep immediately, then continues with periodic sweeps.
    /// Does not block — the driver is ready to accept requests before the
    /// first sweep completes.
    ///
    /// Dropping the returned monitor cancels the background task.
    pub fn start(
        probe: HealthProbe,
        location_state_store: Arc<LocationStateStore>,
    ) -> Self {
        let failure_counts = Arc::new(Mutex::new(HashMap::new()));
        let task_manager = BackgroundTaskManager::new();

        task_manager.spawn(Box::pin(Self::sweep_loop(
            probe.clone(),
            Arc::clone(&location_state_store),
            Arc::clone(&failure_counts),
        )));

        Self {
            probe,
            location_state_store,
            failure_counts,
            task_manager,
        }
    }
}
```

---

## 4. Integration Points

### 4.1 `CosmosDriver` Initialization

The `EndpointHealthMonitor` is created during `CosmosDriver` construction, after the
initial `AccountProperties` fetch succeeds:

```rust
// Pseudocode in CosmosDriver::new() or get_or_create_driver():

let health_probe = HealthProbe::GetAccountMetadata {
    transport: Arc::clone(&self.runtime.transport()),
    credential: account.credential().clone(),
    user_agent: user_agent.clone(),
};

// Start health monitor (non-blocking — spawns background sweep immediately).
let health_monitor = EndpointHealthMonitor::start(
    health_probe,
    Arc::clone(&location_state_store),
);
```

### 4.2 `execute_operation_pipeline` Integration

The operation pipeline (`execute_operation_pipeline`) already reads from
`LocationStateStore` (via `resolve_endpoint` in the Transport Pipeline Spec).
Health check results automatically influence routing because the sweep applies
`LocationEffect`s through the same store that `resolve_endpoint` reads.

No changes to `execute_operation_pipeline` are required. The integration is purely
through shared state.

### 4.3 Reactive + Proactive Synergy

The health monitor (proactive) and the operation retry loop (reactive) both apply
effects through the same `LocationStateStore`:

| Source | Marks Unavailable | Marks Available |
|--------|-------------------|-----------------|
| `execute_operation` retry (reactive) | Yes — on 503, transport error | No |
| Health sweep (proactive) | Yes — on probe failure | Yes — on probe success |

This means:
- Reactive failures are detected instantly (no waiting for next sweep).
- Proactive sweeps detect recovery — this is the **only** recovery mechanism.
  Expiration-based recovery (`expire_unavailable_endpoints`) is not used; endpoint
  availability is determined solely by health checks.
- Both route through `LocationStateStore::apply()` for consistency, observability,
  and single-writer semantics (no lost updates).

### 4.4 Diagnostics Integration

Health probe activity is recorded in the `DiagnosticsContext` system using a dedicated
`ExecutionContext::HealthCheckProbe` variant. This is distinct from
`ExecutionContext::CircuitBreakerProbe` (used by the partition-level circuit breaker /
PPCB), since health checks and PPCB are independent features.

Logging levels:
- **Failed probes**: `warn!` with endpoint, status code, sub-status, consecutive failure
  count, and latency.
- **First successful probe after failure (recovery)**: `info!` with endpoint and latency.
- **Routine successful probes**: Not logged (avoids noise; "everything is OK" alarms
  get ignored).

---

## 5. Configuration

Health check configuration is fully internal — there is no public `HealthCheckOptions`.
All parameters are hardcoded constants matching the Python SDK:

| Parameter | Value | Notes |
|-----------|-------|-------|
| Sweep interval | 300s (5 min) | ±10% jitter applied per sweep |
| Max probe attempts | 3 | Total attempts per endpoint per sweep |
| Initial retry delay | 500ms | Exponential backoff on probe failure |
| Backoff factor | 2x | Doubles each retry |
| Max retry delay | 180s (3 min) | Cap on backoff |
| Jitter ratio | ±25% | Applied to backoff delays |

These values can be adjusted in future releases if needed, without breaking public API.

---

## 6. File Layout

```text
sdk/cosmos/azure_data_cosmos_driver/src/
  driver/
    health/
      mod.rs              # Public exports
      probe.rs            # HealthProbe enum + ProbeResult + ProbeFailure
      monitor.rs          # EndpointHealthMonitor + sweep loop + retry
```

---

## 7. Swapping to Gateway Health API

When the dedicated gateway health endpoint ships, the migration is:

1. Add a `HealthEndpoint` variant to the `HealthProbe` enum:
   ```rust
   pub(crate) enum HealthProbe {
       GetAccountMetadata { /* ... */ },
       HealthEndpoint { url_pattern: String, /* ... */ },
   }
   ```

2. Implement `probe()` for the new variant.

3. Update the probe construction in `CosmosDriver` initialization to select the
   appropriate variant (potentially based on information from `GetAccountProperties`).

4. **Nothing else changes.** The monitor, sweep loop, state management, and diagnostics
   all remain identical.

---

## 8. Shutdown & Resource Cleanup

The `EndpointHealthMonitor` uses `BackgroundTaskManager` (the same abstraction used by
`GlobalPartitionEndpointManager` and `GlobalEndpointManager`) for background task
lifecycle. When the monitor is dropped:

1. `BackgroundTaskManager` is dropped.
2. All stored futures are dropped, which cancels the sweep task — `drop(future)` is how
   Rust communicates cancellation to futures.
3. Any in-flight HTTP requests are cancelled by the runtime when their futures are dropped.

No explicit `shutdown()` method or `CancellationToken` is needed — Rust's ownership
model handles cleanup via `Drop`.

---

## 9. Testing Strategy

### 9.1 Unit Tests

| Test | What it validates |
|------|-------------------|
| `endpoints_to_probe_deduplicates` | Read + write overlap → no duplicate probes |
| `endpoints_to_probe_empty_state` | No endpoints → empty probe list |
| `endpoints_to_probe_excludes_global` | Global endpoint is never included |
| `sweep_marks_healthy_available` | Previously unavailable endpoint passes probe → `LocationEffect::MarkEndpointAvailable` emitted |
| `sweep_marks_unhealthy_unavailable` | Failed probe → `LocationEffect::MarkEndpointUnavailable` emitted |
| `probe_failure_tracks_consecutive_count` | `consecutive_failures` increments across sweeps and resets on success |
| `startup_sweep_runs_immediately` | Background sweep loop executes first sweep without waiting for interval |
| `sweep_interval_has_jitter` | Successive sweep intervals vary (not exactly 300s) |

### 9.2 Integration Tests

Tests use `FaultInjection` to simulate transport failures, exercising the real probe
code path:

| Test | What it validates |
|------|-------------------|
| `startup_probe_populates_state` | After driver init, unavailable endpoints reflect actual reachability |
| `background_sweep_detects_recovery` | Inject error → endpoint unavailable → clear fault → probe succeeds → endpoint available |
| `health_check_with_fault_injection` | Inject transport error → probe fails → `LocationEffect::MarkEndpointUnavailable` applied |
| `recovery_logs_info_after_failure` | First success after failure logs at `info!` level |

---

## 10. Migration Plan

This feature is part of the Transport Pipeline Spec migration. It can be implemented as
a standalone step that depends on Step 2 (multi-region failover & endpoint management)
since it requires `AccountEndpointState`, `LocationStateStore`, and `LocationEffect`.

| Sub-step | Work Item | Files |
|----------|-----------|-------|
| HC.1 | **`HealthProbe` enum + `ProbeResult` + `ProbeFailure`** — Enum-based probe with structured failure details. | `driver/health/probe.rs` |
| HC.2 | **`GetAccountMetadata` variant** — Implement via `execute_operation` with excluded regions. | `driver/health/probe.rs` |
| HC.3 | **`LocationEffect` variants** — Add `MarkEndpointAvailable` and `MarkEndpointUnavailable`. Amend Transport Pipeline Spec. | `driver/routing/` |
| HC.4 | **`EndpointHealthMonitor`** — Sweep loop with `FuturesUnordered`, probe-level retry, jitter, `BackgroundTaskManager`. | `driver/health/monitor.rs` |
| HC.5 | **`ExecutionContext::HealthCheckProbe`** — New diagnostics variant, separate from PPCB. | `driver/diagnostics/` |
| HC.6 | **Wire into `CosmosDriver`** — Create probe + monitor during driver init (non-blocking). | `driver/cosmos_driver.rs` |
| HC.7 | **Unit tests** — Probe, sweep, state update, consecutive failure tracking. | `driver/health/tests/` |
| HC.8 | **Integration tests** — End-to-end with FaultInjection. | `tests/` |

### Dependency

- **Requires**: Transport Pipeline Spec Step 2 (`AccountEndpointState`,
  `LocationStateStore`, `LocationEffect`, `CosmosEndpoint`, `UnavailableReason`).
  Step 1 (PR #3875) provides the pipeline architecture used by probes.
- **Requires amendment**: Transport Pipeline Spec must add `LocationEffect::MarkEndpointAvailable`
  and `LocationEffect::MarkEndpointUnavailable` variants.
- **Enables**: Faster failover and recovery in Steps 3+ (circuit breaker, hedging).

---

## 11. Open Questions

| # | Question | Status |
|---|----------|--------|
| 1 | Should the health monitor also trigger an `AccountMetadataCache` refresh on sweep? The Python SDK re-fetches account properties alongside health checks. | **Open.** Leaning toward decoupled (health sweep only updates endpoint availability, metadata refresh stays on its own schedule) but subject to further discussion. |
| 2 | What is the gateway health API contract? Need to coordinate with the gateway team for the `HealthEndpoint` variant. Can the health probe URL be discovered via `GetAccountProperties`? | **Deferred** until API is available |
| 3 | Should the global endpoint ever be used for anything beyond topology discovery in the Rust SDK? | **Proposed: no** — global endpoint is for topology only, all data operations and health probes target regional endpoints. |
