# Cross-Region Hedging Availability Strategy Spec

**Status:** Draft  
**Date:** 2026-04-23  
**Authors:** (team)  
**Crate:** `azure_data_cosmos_driver`

---

## Table of Contents

1. [Goals & Motivation](#1-goals--motivation)
2. [Background: .NET SDK Implementation](#2-background-net-sdk-implementation)
3. [Architectural Overview](#3-architectural-overview)
4. [Configuration Surface](#4-configuration-surface)
5. [Eligibility Rules](#5-eligibility-rules)
6. [Hedging Algorithm](#6-hedging-algorithm)
7. [Final Result Classification](#7-final-result-classification)
8. [Operation Pipeline Integration](#8-operation-pipeline-integration)
9. [Interaction with Existing Systems](#9-interaction-with-existing-systems)
10. [Diagnostics & Observability](#10-diagnostics--observability)
11. [Options API Design](#11-options-api-design)
12. [Cancellation & Resource Cleanup](#12-cancellation--resource-cleanup)
13. [Multi-Write Region Write Hedging](#13-multi-write-region-write-hedging)
14. [Error Handling & Edge Cases](#14-error-handling--edge-cases)
15. [Test Plan](#15-test-plan)
16. [Implementation Phases](#16-implementation-phases)
17. [Open Questions](#17-open-questions)

---

## 1. Goals & Motivation

### Problem

When a Cosmos DB region experiences elevated latency (but is not fully down), the
existing failover/circuit-breaker mechanisms (PPAF/PPCB) do **not** trigger because
the region eventually returns successful responses. Users see tail-latency spikes
(p99, p99.9) on operations routed to the slow region.

### Solution: Speculative Hedging

**Hedging** sends the same request to an alternate region after a latency threshold
is exceeded, and returns whichever response arrives first. This bounds tail latency
at roughly `threshold + cross-region-RTT` instead of waiting for the slow region to
respond.

### Goals

1. **Reduce tail latency** — p99/p99.9 point-read and point-write latencies bounded
   by a user-configured threshold plus cross-region RTT.
2. **Transparent to application** — the caller sees a single `CosmosResponse`; the
   hedging mechanism is invisible unless inspected via diagnostics.
3. **Configurable** — threshold, step interval, and write-hedging opt-in are
   user-controlled at both client and per-operation levels.
4. **Complementary to failover** — hedging handles *latency*; PPAF/PPCB handle
   *failures*. They compose without interference.
5. **Resource-safe** — hedged requests that lose the race are cancelled promptly to
   avoid wasted RU/s and transport resources.

### Non-Goals

- Hedging for non-document operations (metadata, sprocs, queries, change feed).
- Hedging within a single region (e.g., across gateway nodes).
- Automatic threshold tuning based on observed latency histograms (future work).

---

## 2. Background: .NET SDK Implementation

The .NET Cosmos DB SDK (v3) implements hedging via the
`CrossRegionHedgingAvailabilityStrategy` class. This section documents its design
to inform the Rust implementation.

### 2.1 Public API

```csharp
// Factory method on AvailabilityStrategy (public abstract class)
public static AvailabilityStrategy CrossRegionHedgingStrategy(
    TimeSpan threshold,        // Time before first hedge fires
    TimeSpan? thresholdStep,   // Time between subsequent hedges
    bool enableMultiWriteRegionHedge = false);  // Opt-in for writes on MM

// Per-request override
requestOptions.AvailabilityStrategy = AvailabilityStrategy.DisabledStrategy();
```

### 2.2 Configuration Model

| Parameter | Description | Default | Constraints |
|-----------|-------------|---------|-------------|
| `threshold` | Delay before firing the first hedge request | (required) | `> 0` |
| `thresholdStep` | Delay between subsequent hedge requests | (required) | `> 0` |
| `enableMultiWriteRegionHedge` | Allow hedging for writes on multi-write accounts | `false` | Opt-in; increases 409/412 risk |

### 2.3 Eligibility — `ShouldHedge()`

Hedging applies **only** to document-level point operations:

1. `ResourceType == Document` — metadata (Database, Container, etc.) is excluded.
2. **Reads**: Always eligible.
3. **Writes**: Only if `enableMultiWriteRegionHedge == true` AND the account
   supports multi-region writes for this resource/operation type.
4. **Single-region accounts**: Bypassed (no alternate region to hedge to).

### 2.4 Execution — `ExecuteAvailabilityStrategyAsync()`

```
┌─────────────────────────────────────────────────────────────────────┐
│  ExecuteAvailabilityStrategyAsync                                   │
│                                                                     │
│  1. Clone request body (CloneableStream)                            │
│  2. Get applicable regions via GlobalEndpointManager                │
│     (respects ExcludeRegions, read vs. write list)                  │
│  3. For requestNumber = 0..regions.len():                           │
│     a. awaitTime = (requestNumber == 0) ? threshold : thresholdStep │
│     b. Start hedge timer (Task.Delay(awaitTime))                    │
│     c. Clone request, set ExcludeRegions to exclude all regions     │
│        except regions[requestNumber] (primary request keeps all)    │
│     d. Fire CloneAndSendAsync → RequestSenderAndResultCheckAsync    │
│     e. Race: Task.WhenAny(requestTask, hedgeTimer, ...)             │
│        ├─ Timer wins → continue to next iteration (launch next      │
│        │               hedge), keep running tasks alive              │
│        ├─ Request wins + IsFinalResult → cancel all others, return  │
│        └─ Request wins + transient → remove task, continue racing   │
│  4. After all regions attempted, drain remaining tasks:             │
│     a. Wait for each remaining task via WhenAny                     │
│     b. First IsFinalResult or last remaining task → return          │
│  5. If all tasks fail/cancel, throw last exception                  │
└─────────────────────────────────────────────────────────────────────┘
```

**Key observations:**

- **Primary request is request #0** — it uses the same region the SDK would normally
  pick. `ExcludeRegions` is NOT set for it, so it follows normal routing.
- **Hedge requests (1..N)** — each excludes all regions except one target region,
  forcing routing to that specific region.
- **Concurrent fan-out** — all fired requests run in parallel. Timers gate when
  new hedges are launched, but previously launched requests continue.
- **Early termination** — the first `IsFinalResult` response cancels all other
  in-flight requests via a linked `CancellationTokenSource`.

### 2.5 Final Result Classification — `IsFinalResult()`

A response is "final" (non-transient) if:

| Condition | Final? |
|-----------|--------|
| Any 1xx, 2xx, 3xx | Yes |
| 400 Bad Request | Yes |
| 401 Unauthorized | Yes |
| 405 Method Not Allowed | Yes |
| 409 Conflict | Yes |
| 412 Precondition Failed | Yes |
| 413 Request Entity Too Large | Yes |
| 404 with sub-status 0 (Unknown) | Yes |
| All other 4xx/5xx | **No** (transient) |

Non-final (transient) responses do NOT terminate hedging — the SDK keeps waiting
for other in-flight requests that might succeed.

### 2.6 PPAF Integration

When PPAF is enabled on the account and the user has not specified a custom
availability strategy, the SDK **automatically** installs a default hedging
strategy (`IsSDKDefaultStrategyForPPAF = true`). This provides latency protection
alongside PPAF's failover protection.

### 2.7 Diagnostics

The .NET SDK attaches three diagnostic data points to the winning response:
- **Hedge Config** — threshold/step/write-enabled settings string.
- **Hedge Context** — list of regions that were contacted (up to and including the
  winning request's ordinal).
- **Response Region** — the target region name of the winning request.

---

## 3. Architectural Overview

### 3.1 Where Hedging Sits in the Driver

Hedging operates **above** the existing 7-stage operation pipeline loop. It wraps
the `execute_operation_pipeline()` call, launching parallel invocations against
different regions:

```
CosmosDriver.execute_operation()
    │
    ▼
┌──────────────────────────────────────────┐
│  HedgingOrchestrator (new)               │
│                                          │
│  • Decides if hedging applies            │
│  • Clones operation context              │
│  • Spawns primary + hedge tasks          │
│  • Races results, applies IsFinalResult  │
│  • Cancels losers, returns winner        │
└──────────────────────────────────────────┘
    │
    ▼ (one or more parallel invocations)
execute_operation_pipeline()  [existing 7-stage loop]
    │
    ▼
CosmosResponse
```

**Rationale:** Hedging must operate above the retry loop because each hedged
request needs its own independent retry state, session tokens, and endpoint
resolution. The operation pipeline already handles per-region retries; hedging
adds cross-region parallelism on top.

### 3.2 Core Data Flow

```
                              ┌──────────────┐
                              │  Hedge Timer  │
                              │  (threshold)  │
                              └──────┬───────┘
                                     │ fires
    ┌────────────────────────────────┼────────────────────────────────┐
    │                                │                                │
    ▼                                ▼                                ▼
┌────────────┐               ┌────────────┐               ┌────────────┐
│ Primary    │               │  Hedge #1  │               │  Hedge #2  │
│ Region A   │               │  Region B  │               │  Region C  │
│ (request 0)│               │ (request 1)│               │ (request 2)│
└─────┬──────┘               └─────┬──────┘               └─────┬──────┘
      │                            │                            │
      ▼                            ▼                            ▼
   execute_operation_            execute_operation_           execute_operation_
   pipeline()                    pipeline()                   pipeline()
      │                            │                            │
      └────────────┬───────────────┘                            │
                   ▼                                            │
           tokio::select! / race                                │
           first IsFinalResult wins ◄───────────────────────────┘
                   │
                   ▼
           Cancel losers (CancellationToken)
                   │
                   ▼
           Return CosmosResponse + HedgeDiagnostics
```

### 3.3 Design Principles

1. **Pure orchestration** — the hedging layer does NOT modify the operation pipeline.
   It composes multiple independent pipeline invocations.
2. **Cooperative cancellation** — each hedged pipeline invocation receives a
   `CancellationToken` (via `tokio_util::sync::CancellationToken`) that is
   cancelled when a winner is found.
3. **Immutable request cloning** — the `CosmosOperation` (which contains `&[u8]`
   body, headers, partition key) is cheap to clone (bytes are `Arc`-backed).
4. **Respect existing systems** — hedging does not interfere with PPAF/PPCB,
   session consistency, or throughput control. Each pipeline invocation operates
   independently with its own retry state.

---

## 4. Configuration Surface

### 4.1 HedgingStrategy Type

```rust
/// Cross-region hedging availability strategy.
///
/// When the primary request does not complete within `threshold`, the driver
/// sends a speculative (hedged) request to the next preferred region. If that
/// does not complete within `threshold_step`, another hedge is launched, until
/// all available regions have a request in flight.
///
/// The first non-transient response wins; all other in-flight requests are
/// cancelled.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct HedgingStrategy {
    /// Delay before the first hedge request is fired.
    threshold: Duration,

    /// Delay between subsequent hedge requests.
    threshold_step: Duration,

    /// Whether to hedge write operations on multi-write accounts.
    ///
    /// **Caveat:** enabling this may increase 409 Conflict / 412 Precondition
    /// Failed errors, because the same write can be inflight to multiple regions
    /// simultaneously via async replication.
    enable_multi_write_region_hedge: bool,
}

impl HedgingStrategy {
    /// Creates a new hedging strategy.
    ///
    /// # Panics
    /// Panics if `threshold` or `threshold_step` is zero.
    pub fn new(threshold: Duration, threshold_step: Duration) -> Self { ... }

    /// Enables hedging for write operations on multi-master accounts.
    pub fn with_multi_write_region_hedge(mut self) -> Self { ... }

    /// Returns the threshold before the first hedge fires.
    pub fn threshold(&self) -> Duration { ... }

    /// Returns the interval between subsequent hedge requests.
    pub fn threshold_step(&self) -> Duration { ... }

    /// Returns whether write hedging is enabled on multi-write accounts.
    pub fn multi_write_region_hedge_enabled(&self) -> bool { ... }
}
```

### 4.2 Disabled Strategy

```rust
/// Sentinel value used to disable hedging for a specific operation when a
/// client-level strategy is configured.
///
/// ```rust
/// let options = ItemReadOptionsBuilder::new()
///     .with_availability_strategy(AvailabilityStrategy::Disabled)
///     .build();
/// ```
#[derive(Clone, Debug)]
pub enum AvailabilityStrategy {
    /// Cross-region hedging strategy.
    Hedging(HedgingStrategy),
    /// Explicitly disable the client-level strategy for this request.
    Disabled,
}
```

### 4.3 Integration with OperationOptions

```rust
// In OperationOptions (layered resolution: operation > account > runtime > env)
pub struct OperationOptions {
    // ... existing fields ...

    /// Availability strategy controlling cross-region hedging.
    ///
    /// When set to `Some(AvailabilityStrategy::Hedging(..))`, the driver will
    /// speculatively send requests to alternate regions after the configured
    /// threshold. Set to `Some(AvailabilityStrategy::Disabled)` to suppress a
    /// client-level strategy for this request.
    pub availability_strategy: Option<AvailabilityStrategy>,
}
```

### 4.4 Environment Variable Support

| Variable | Description | Default |
|----------|-------------|---------|
| `AZURE_COSMOS_HEDGING_THRESHOLD_MS` | Threshold in milliseconds | (none — off) |
| `AZURE_COSMOS_HEDGING_THRESHOLD_STEP_MS` | Step interval in milliseconds | Same as threshold |
| `AZURE_COSMOS_HEDGING_ENABLE_MULTI_WRITE_REGION` | `true`/`false` | `false` |

Environment variables provide the lowest-priority configuration. Setting the
threshold env var implicitly enables hedging at the runtime level.

---

## 5. Eligibility Rules

### 5.1 `should_hedge()` — Pure Function

```rust
/// Determines whether the given operation should use hedging.
///
/// Returns `false` if:
/// - No hedging strategy is configured (or explicitly disabled)
/// - Only one region is available (no alternate to hedge to)
/// - Operation is not a document point operation
/// - Operation is a write AND multi-write hedging is not enabled
/// - Operation is a write on a single-master account
fn should_hedge(
    strategy: &HedgingStrategy,
    operation: &CosmosOperation,
    account_state: &AccountEndpointState,
) -> bool
```

**Decision matrix:**

| Condition | Hedge? |
|-----------|--------|
| `ResourceType != Document` | No |
| Single region account | No |
| Read operation | Yes |
| Write + single-master | No |
| Write + multi-master + `enable_multi_write_region_hedge = false` | No |
| Write + multi-master + `enable_multi_write_region_hedge = true` | Yes |

### 5.2 Interaction with PPAF Default Strategy

When PPAF is enabled on the account and the user has NOT configured a custom
availability strategy, the driver SHOULD install a default hedging strategy
automatically (matching .NET SDK behavior):

```rust
// In cosmos_driver.rs, during account properties sync
if account_properties.enable_per_partition_failover_behavior
    && resolved_options.availability_strategy().is_none()
{
    // Install SDK-default hedging: 500ms threshold, 500ms step
    let default_strategy = HedgingStrategy::new(
        Duration::from_millis(500),
        Duration::from_millis(500),
    );
    // Mark as SDK-default (distinguishable in diagnostics)
    ...
}
```

---

## 6. Hedging Algorithm

### 6.1 Overview

The hedging orchestrator is an async function that wraps
`execute_operation_pipeline()`:

```rust
async fn execute_with_hedging(
    strategy: &HedgingStrategy,
    // ... all parameters for execute_operation_pipeline ...
) -> azure_core::Result<CosmosResponse>
```

### 6.2 Region Selection

Regions are ordered from the applicable preferred endpoint list:

- **Reads:** `preferred_read_endpoints`
- **Writes (multi-master):** `preferred_write_endpoints`

The region list is filtered by `ExcludeRegions` from the operation options.

```
regions = get_applicable_regions(excluded_regions, is_read)
// regions[0] = primary (normal routing)
// regions[1] = first hedge candidate
// regions[N] = Nth hedge candidate
```

### 6.3 Request Routing via ExcludeRegions

Each hedged request is directed to a specific region by **excluding all other
regions**:

| Request # | ExcludeRegions | Target |
|-----------|----------------|--------|
| 0 (primary) | (none — normal routing) | regions[0] |
| 1 (hedge 1) | all except regions[1] | regions[1] |
| 2 (hedge 2) | all except regions[2] | regions[2] |

This piggybacks on the existing `ExcludeRegions` mechanism in `resolve_endpoint()`,
requiring no changes to the endpoint resolution logic.

### 6.4 Execution Flow (Pseudocode)

```rust
async fn execute_with_hedging(...) -> Result<CosmosResponse> {
    let regions = get_applicable_regions(&account_state, &excluded_regions, is_read);
    if regions.len() <= 1 {
        // No alternate region — fall through to non-hedged path
        return execute_operation_pipeline(...).await;
    }

    let cancel = CancellationToken::new();
    let (winner_tx, winner_rx) = tokio::sync::oneshot::channel();

    // Shared state: collects JoinHandles for cleanup
    let mut tasks: Vec<JoinHandle<HedgeOutcome>> = Vec::with_capacity(regions.len());

    for request_number in 0..regions.len() {
        // ── Gate: wait for threshold/step before launching ──
        let wait = if request_number == 0 {
            Duration::ZERO  // Primary fires immediately
        } else if request_number == 1 {
            strategy.threshold()
        } else {
            strategy.threshold_step()
        };

        tokio::select! {
            // Timer fires → launch this hedge
            _ = tokio::time::sleep(wait) => {}
            // A previous request already won → stop launching
            _ = cancel.cancelled() => break,
        }

        // ── Clone operation context for this hedge ──
        let hedge_options = if request_number == 0 {
            options.clone()  // Primary: no region override
        } else {
            let mut opts = options.clone();
            // Exclude all regions except the target
            let exclude: Vec<Region> = regions.iter()
                .enumerate()
                .filter(|(i, _)| *i != request_number)
                .map(|(_, r)| r.clone())
                .collect();
            opts.excluded_regions = Some(ExcludedRegions::new(exclude));
            opts
        };

        let cancel_child = cancel.child_token();
        let task = tokio::spawn(async move {
            tokio::select! {
                result = execute_operation_pipeline(
                    &hedge_options,
                    /* ... all other params ... */
                ) => {
                    HedgeOutcome {
                        request_number,
                        region: regions[request_number].clone(),
                        result,
                    }
                }
                _ = cancel_child.cancelled() => {
                    HedgeOutcome {
                        request_number,
                        region: regions[request_number].clone(),
                        result: Err(cancelled_error()),
                    }
                }
            }
        });
        tasks.push(task);
    }

    // ── Race: collect results as they complete ──
    let mut remaining: FuturesUnordered<_> = tasks.into_iter().collect();
    let mut last_transient: Option<HedgeOutcome> = None;

    while let Some(join_result) = remaining.next().await {
        let outcome = match join_result {
            Ok(o) => o,
            Err(join_err) => {
                // Task panicked — treat as transient failure
                tracing::error!("hedged task panicked: {join_err}");
                continue;
            }
        };

        match &outcome.result {
            Ok(response) if is_final_result(response.status()) => {
                // ── Winner: cancel all others, return ──
                cancel.cancel();
                let mut response = outcome.result.unwrap();
                response.attach_hedge_diagnostics(HedgeDiagnostics {
                    strategy_config: strategy.clone(),
                    regions_contacted: regions[..=outcome.request_number].to_vec(),
                    response_region: outcome.region,
                });
                return Ok(response);
            }
            Ok(_) => {
                // Transient response — keep waiting for other hedges
                last_transient = Some(outcome);
            }
            Err(_) => {
                // Error — keep waiting
                last_transient = Some(outcome);
            }
        }
    }

    // ── All tasks completed without a final result ──
    cancel.cancel();
    match last_transient {
        Some(outcome) => outcome.result,
        None => Err(azure_core::Error::message(
            azure_core::error::ErrorKind::Other,
            "hedging completed without producing a response",
        )),
    }
}
```

### 6.5 Key Invariants

1. **At most `regions.len()` concurrent requests** — one per region.
2. **Primary request fires immediately** — zero additional latency on the happy path.
3. **Hedge timers are interruptible** — if a winner arrives during a timer wait,
   no further hedges are launched.
4. **Cancellation is cooperative** — `CancellationToken` is checked at `select!`
   points inside `execute_operation_pipeline()` and at the transport layer via
   deadline enforcement.
5. **Single writer to diagnostics** — only the winning response gets hedge
   diagnostics attached.

---

## 7. Final Result Classification

### 7.1 `is_final_result()` — Pure Function

```rust
/// Determines whether a response status code is a final (non-transient) result.
///
/// Final results terminate hedging immediately. Transient results allow other
/// in-flight hedges to continue racing for a better outcome.
fn is_final_result(status: &CosmosStatus) -> bool {
    let code = status.http_status_code;
    let sub = status.sub_status_code;

    // All 1xx, 2xx, 3xx → final
    if code < 400 {
        return true;
    }

    // Specific client errors that are definitively non-transient
    matches!(code,
        400  // Bad Request
        | 401  // Unauthorized
        | 405  // Method Not Allowed
        | 409  // Conflict
        | 412  // Precondition Failed
        | 413  // Request Entity Too Large
    ) || (code == 404 && sub == 0)  // Not Found with no sub-status
}
```

### 7.2 Transient vs. Non-Transient Responses

| Status | Sub-Status | Transient? | Rationale |
|--------|------------|------------|-----------|
| 200 | * | No (final) | Success |
| 304 | * | No (final) | Not Modified |
| 400 | * | No (final) | Client error — won't succeed in another region |
| 401 | * | No (final) | Auth failure — same credentials everywhere |
| 403 | 3 | **Yes** | WriteForbidden — region may be failing over |
| 404 | 0 | No (final) | Resource genuinely not found |
| 404 | 1002 | **Yes** | ReadSessionNotAvailable — session lag |
| 405 | * | No (final) | Wrong HTTP method |
| 408 | * | **Yes** | Timeout — another region may be faster |
| 409 | * | No (final) | Conflict — deterministic |
| 410 | * | **Yes** | Gone — partition may have moved |
| 412 | * | No (final) | Precondition — deterministic |
| 413 | * | No (final) | Payload too large — same everywhere |
| 429 | * | **Yes** | Throttled — another region may have capacity |
| 500 | * | **Yes** | Internal error — may be region-specific |
| 503 | * | **Yes** | Unavailable — another region may be healthy |

---

## 8. Operation Pipeline Integration

### 8.1 Entry Point Changes

The integration point is `cosmos_driver.rs` → `execute_operation()`:

```rust
// Current flow (simplified):
pub async fn execute_operation(&self, operation: CosmosOperation) -> Result<CosmosResponse> {
    let options = self.resolve_options(&operation);
    execute_operation_pipeline(options, ...).await
}

// New flow with hedging:
pub async fn execute_operation(&self, operation: CosmosOperation) -> Result<CosmosResponse> {
    let options = self.resolve_options(&operation);

    if let Some(strategy) = self.resolve_hedging_strategy(&options, &operation) {
        execute_with_hedging(&strategy, options, ...).await
    } else {
        execute_operation_pipeline(options, ...).await
    }
}
```

### 8.2 Operation Cloning

Each hedged invocation needs its own:
- `OperationRetryState` — independent retry counters per region
- `DiagnosticsContextBuilder` — separate diagnostics chain per attempt
- `OperationOptions` — different `ExcludedRegions` per hedge

Items shared (via `Arc` or reference):
- `CosmosOperation` — immutable; body is `Bytes` (cheaply cloneable)
- `LocationStateStore` — lock-free; multiple readers are safe
- `SessionManager` — designed for concurrent access
- `Credential` — `Arc`-wrapped

### 8.3 Cancellation Propagation

The operation pipeline already has deadline-based cancellation. Hedging adds a
second cancellation signal:

```
User CancellationToken
    │
    ├─► Hedging CancellationToken (cancel.cancel() when winner found)
    │       │
    │       ├─► Primary pipeline: deadline + hedging_cancel
    │       ├─► Hedge #1 pipeline: deadline + hedging_cancel
    │       └─► Hedge #2 pipeline: deadline + hedging_cancel
    │
    └─► End-to-end deadline (existing)
```

The pipeline's deadline check (`if deadline_exceeded { return DeadlineExceeded }`)
naturally cooperates with hedging cancellation — a cancelled hedge will observe the
token at its next `select!` point and exit.

### 8.4 File Layout

New files:
```
src/driver/pipeline/
    hedging.rs          # HedgingOrchestrator, execute_with_hedging(), is_final_result()
    hedging_diagnostics.rs  # HedgeDiagnostics struct
```

Modified files:
```
src/driver/cosmos_driver.rs       # Route through hedging when strategy enabled
src/options/operation_options.rs   # Add availability_strategy field
src/options/mod.rs                 # Export AvailabilityStrategy, HedgingStrategy
src/diagnostics/mod.rs            # HedgeDiagnostics type
```

---

## 9. Interaction with Existing Systems

### 9.1 PPAF / PPCB

Hedging and partition-level failover are **complementary**:

| System | Handles | Trigger |
|--------|---------|---------|
| Hedging | Latency | Timer (threshold exceeded) |
| PPAF | Write failures (single-master) | 403/3 from service |
| PPCB | Read/write failures | Failure count threshold |

**No interference:** Each hedged pipeline invocation has its own
`OperationRetryState`. Partition-level effects (`LocationEffect::MarkPartitionUnavailable`)
are applied via the shared `LocationStateStore`, which is lock-free and handles
concurrent CAS operations correctly. Multiple hedges marking the same partition
unavailable is idempotent (the CAS loop merges failure counts).

### 9.2 Session Consistency

**Challenge:** Session tokens are per-region. A hedged request to Region B may not
have the session token captured from a prior request that went to Region A.

**Resolution:** The `SessionManager` is shared across all hedges. Each pipeline
invocation:
1. Reads the latest session token before sending (STAGE 3).
2. Captures the response session token after receiving (post-STAGE 4).

Because hedges run in parallel, a hedge to Region B may use a session token from
Region A. If that fails with 404/1002, the pipeline's session retry logic handles
it internally — this is indistinguishable from normal session retry behavior.

### 9.3 Throughput Control

Each hedged request independently checks the throughput control group budget.
Hedging **does** increase RU consumption when hedge requests actually execute
transport. Users should account for this when setting throughput control limits.

The throughput control snapshot is acquired per-attempt in the operation pipeline
(STAGE 3), so concurrent hedges will see the latest budget.

### 9.4 End-to-End Deadline

If `EndToEndOperationLatencyPolicy` is configured, **all hedges share the same
deadline**. The deadline is computed once at the start of `execute_with_hedging()`
and passed to each pipeline invocation.

Implication: late hedges have less time budget. If the deadline is 5s and the
threshold is 3s, the hedge has only ~2s to complete.

---

## 10. Diagnostics & Observability

### 10.1 HedgeDiagnostics

```rust
/// Diagnostic information about a hedging execution, attached to the winning
/// response.
#[derive(Clone, Debug)]
pub struct HedgeDiagnostics {
    /// The hedging strategy configuration that was active.
    pub strategy_config: HedgingStrategyConfig,
    /// Regions that had requests launched (up to and including the winner).
    pub regions_contacted: Vec<Region>,
    /// The target region of the winning response.
    pub response_region: Region,
    /// How many hedge requests were launched (including primary).
    pub total_requests_launched: usize,
    /// Whether the primary or a hedge won.
    pub was_hedge: bool,
}

#[derive(Clone, Debug)]
pub struct HedgingStrategyConfig {
    pub threshold: Duration,
    pub threshold_step: Duration,
    pub multi_write_region_hedge_enabled: bool,
}
```

### 10.2 DiagnosticsContext Integration

```rust
// In DiagnosticsContext (existing)
pub struct DiagnosticsContext {
    // ... existing fields ...

    /// Hedging diagnostics, present only if hedging was active.
    pub hedge_diagnostics: Option<HedgeDiagnostics>,
}
```

### 10.3 Tracing

```rust
// At hedging entry
tracing::debug!(
    threshold = ?strategy.threshold(),
    step = ?strategy.threshold_step(),
    regions = ?regions,
    "hedging enabled, launching primary request"
);

// When launching a hedge
tracing::debug!(
    request_number,
    target_region = %region,
    elapsed = ?start.elapsed(),
    "launching hedge request"
);

// When a winner is found
tracing::debug!(
    request_number,
    region = %winner_region,
    elapsed = ?start.elapsed(),
    was_hedge = request_number > 0,
    "hedging winner selected, cancelling remaining requests"
);
```

---

## 11. Options API Design

### 11.1 Client-Level Configuration

```rust
// DriverOptions (client-level)
let driver = CosmosDriverRuntimeBuilder::new()
    .build(endpoint, credential, DriverOptionsBuilder::new()
        .with_availability_strategy(AvailabilityStrategy::Hedging(
            HedgingStrategy::new(
                Duration::from_millis(500),
                Duration::from_millis(500),
            ),
        ))
        .build()
    ).await?;
```

### 11.2 Per-Operation Override

```rust
// Disable hedging for a specific operation
let options = OperationOptionsBuilder::new()
    .with_availability_strategy(AvailabilityStrategy::Disabled)
    .build();

// Or use a different threshold for this operation
let options = OperationOptionsBuilder::new()
    .with_availability_strategy(AvailabilityStrategy::Hedging(
        HedgingStrategy::new(
            Duration::from_millis(200),  // Tighter threshold for this read
            Duration::from_millis(200),
        ),
    ))
    .build();
```

### 11.3 Layered Resolution

The existing `OperationOptionsView` layered resolution applies:

```
operation > account > runtime > environment
```

If the operation sets `AvailabilityStrategy::Disabled`, it overrides a client-level
hedging strategy.

---

## 12. Cancellation & Resource Cleanup

### 12.1 Cancellation Token Design

```rust
use tokio_util::sync::CancellationToken;

// Hedging orchestrator creates a parent token
let hedge_cancel = CancellationToken::new();

// Each pipeline invocation receives a child token
let child = hedge_cancel.child_token();

// On winner found:
hedge_cancel.cancel();  // All children see cancellation
```

### 12.2 Pipeline Cooperation

The operation pipeline must check cancellation at strategic points:

1. **Before STAGE 4 (transport)** — avoid sending a request if already cancelled.
2. **During transport** — the HTTP client's timeout mechanism cooperates with
   `tokio::select!` on cancellation.
3. **Before STAGE 7 (retry decision)** — exit early if cancelled.

**Implementation:** The pipeline's existing deadline check provides the natural
integration point. We extend the deadline concept:

```rust
// A request is "done" if either:
// 1. The end-to-end deadline has passed
// 2. The hedging cancellation token is triggered
fn should_exit(deadline: Option<Instant>, cancel: &CancellationToken) -> bool {
    cancel.is_cancelled() || deadline.is_some_and(|d| Instant::now() > d)
}
```

### 12.3 Spawn Safety

Hedged tasks are spawned via `tokio::spawn`. The orchestrator **must** ensure
all spawned tasks are either:
- Completed (winner)
- Cancelled (losers — via token)
- Collected (via `JoinHandle`, not abandoned)

The `FuturesUnordered` drain loop ensures all handles are awaited. Tasks that
panic are caught by `JoinHandle` without crashing the runtime.

---

## 13. Multi-Write Region Write Hedging

### 13.1 Semantics

When `enable_multi_write_region_hedge` is `true`, writes (Create, Replace, Upsert,
Delete, Patch) to multi-master accounts are hedged. This can cause:

- **409 Conflict** — two creates with the same ID racing to different regions.
- **412 Precondition Failed** — two conditional writes racing.
- **Non-deterministic Upsert** — an upsert may create in one region while it
  replaces in another.

These are **expected** when write hedging is enabled. The application MUST handle
409/412 responses.

### 13.2 Safety

- **Single-master writes are NEVER hedged** — PPAF handles write failover for
  single-master. Hedging a non-idempotent write to a single-master account could
  cause data corruption (write sent to non-writable region).
- **IdempotencyKey (future)** — if the service supports idempotency keys, write
  hedging becomes safer. This is a future enhancement.

### 13.3 Configuration Gate

```rust
fn should_hedge_write(
    strategy: &HedgingStrategy,
    account_state: &AccountEndpointState,
    operation: &CosmosOperation,
) -> bool {
    strategy.multi_write_region_hedge_enabled()
        && account_state.multiple_write_locations_enabled
        && account_state.preferred_write_endpoints.len() > 1
}
```

---

## 14. Error Handling & Edge Cases

### 14.1 All Hedges Return Transient Errors

If all regions return transient errors (e.g., 503 everywhere), the orchestrator
returns the **last** response received. The retry logic within each pipeline
invocation will have already attempted retries before surfacing the error.

### 14.2 Primary Succeeds After Hedge Launched

If the primary returns a final result 1ms after a hedge is launched, the hedge is
cancelled. The hedge's transport request may or may not have been sent (depends on
timing). Cancellation is best-effort — an in-flight HTTP request cannot be aborted
mid-stream, but the response will be discarded.

### 14.3 Deadline Interplay

If the end-to-end deadline is shorter than the hedging threshold, hedging has no
effect — the primary request will hit the deadline before any hedge fires.

```
deadline = 200ms, threshold = 500ms
→ Primary fires at t=0
→ Hedge would fire at t=500ms, but deadline hit at t=200ms
→ Only primary result returned (or deadline error)
```

**Guideline:** `threshold` should be significantly less than `end_to_end_timeout / 2`
to leave time for the hedge to complete.

### 14.4 Region List Changes During Hedging

The region list is captured at the start of `execute_with_hedging()`. If account
metadata refreshes during execution (e.g., a `RefreshAccountProperties` effect),
the region list for **already-launched** hedges is unchanged. The pipeline's
`resolve_endpoint()` within each hedge will use the latest `LocationSnapshot`,
which may reflect updated regions. This is safe because `ExcludeRegions` only
*hints* at routing — `resolve_endpoint()` always falls back to available endpoints.

---

## 15. Test Plan

### 15.1 Unit Tests

| Test | Validates |
|------|-----------|
| `should_hedge_read_multi_region` | Reads eligible on multi-region account |
| `should_hedge_read_single_region` | Reads NOT eligible on single-region |
| `should_hedge_write_disabled` | Writes NOT eligible by default |
| `should_hedge_write_multi_master_enabled` | Writes eligible when opted-in on MM |
| `should_hedge_write_single_master` | Writes NEVER eligible on SM |
| `should_hedge_non_document` | Non-document ops excluded |
| `should_hedge_disabled_override` | Per-operation disable overrides client |
| `is_final_result_success` | 200 → final |
| `is_final_result_conflict` | 409 → final |
| `is_final_result_503` | 503 → transient |
| `is_final_result_404_0` | 404/0 → final |
| `is_final_result_404_1002` | 404/1002 → transient |
| `is_final_result_429` | 429 → transient |
| `hedging_config_validation` | Zero threshold panics |
| `hedging_config_step_defaults` | Step inherits threshold |
| `region_exclusion_for_hedge_n` | Correct ExcludeRegions per hedge |

### 15.2 Integration Tests (Fault Injection)

| Test | Setup | Validates |
|------|-------|-----------|
| `hedging_read_primary_slow` | 2s delay on Region A reads | Hedge to Region B wins; diagnostics show `was_hedge=true` |
| `hedging_read_primary_fast` | No faults | Primary wins; no hedge launched; `hedge_diagnostics=None` or `was_hedge=false` |
| `hedging_read_primary_503` | 503 on Region A reads | Hedge to Region B wins with success |
| `hedging_read_all_regions_slow` | 2s delay on all regions | Last region to respond wins (graceful degradation) |
| `hedging_write_multi_master` | 2s delay on Region A creates | Hedge to Region B succeeds |
| `hedging_write_single_master_not_hedged` | 2s delay on writes | No hedge — write returns after delay |
| `hedging_disabled_per_operation` | Client hedging on; operation disabled | No hedge — normal path |
| `hedging_respects_deadline` | threshold > deadline | No hedge fires; deadline error |
| `hedging_with_ppcb` | 503 on Region A reads; PPCB enabled | PPCB and hedging both apply; circuit breaker tripped AND hedge succeeds |
| `hedging_cancels_losers` | Delay on Region A | Region B wins; verify Region A task cancelled (hit_count ≤ expected) |
| `hedging_failback_to_primary` | Region A initially slow, then fast | First few reads hedged; after threshold tightened, primary wins again |

### 15.3 Multi-Region Live Tests

Gated by `test_category = "multi_region"`:

| Test | Account Type | Validates |
|------|-------------|-----------|
| `hedging_read_cross_region` | 2-region SM | Read hedged to satellite when primary slow |
| `hedging_write_cross_region` | 2-region MM | Write hedged; handle 409 if applicable |
| `hedging_with_ppaf` | PPAF-enabled SM | Hedging + PPAF work together on write failure |

---

## 16. Implementation Phases

### Phase 1: Read Hedging (MVP)

**Scope:**
- `HedgingStrategy` and `AvailabilityStrategy` types
- `should_hedge()` (reads only)
- `is_final_result()`
- `execute_with_hedging()` orchestrator
- `HedgeDiagnostics`
- Integration into `cosmos_driver.rs`
- Cancellation via `CancellationToken`
- Unit tests + fault injection tests for reads
- Environment variable support

**Deliverables:**
- New files: `hedging.rs`, `hedging_diagnostics.rs`
- Modified: `cosmos_driver.rs`, `operation_options.rs`, `mod.rs`

### Phase 2: Write Hedging + PPAF Default

**Scope:**
- `enable_multi_write_region_hedge` support
- Write eligibility rules
- Auto-enable hedging when PPAF is detected
- Write hedging integration tests
- Documentation for 409/412 behavior

### Phase 3: Adaptive Thresholds (Future)

**Scope:**
- Latency histogram tracking per-region
- Auto-tuning threshold based on p50/p90 latency
- Exponential backoff on hedge threshold after repeated hedges

---

## 17. Open Questions

1. **Should hedging be enabled by default?** — The .NET SDK does NOT enable it by
   default (except as PPAF companion). Recommendation: off by default, opt-in via
   configuration.

2. **Interaction with `EndToEndOperationLatencyPolicy`** — Should the hedge's
   deadline be the remaining time from the shared deadline, or should each hedge get
   its own full timeout? Recommendation: shared deadline (matching .NET behavior).

3. **RU accounting** — Hedged requests consume RU/s. Should diagnostics report the
   total RU across all hedges, or only the winner's RU? Recommendation: total RU
   (matching .NET SDK's `RequestCharge` which includes all sub-requests).

4. **Race with background failback** — If PPCB transitions a partition to
   `ProbeCandidate` while a hedging orchestrator is running, the probe request
   and the hedge may both target the original region. Is this safe?
   Recommendation: yes — they are independent pipeline invocations with independent
   retry states. The CAS-based state update is safe under concurrency.

5. **Max concurrent hedges cap** — Should there be a maximum number of concurrent
   hedges (e.g., 3) even if the account has 5 regions? Recommendation: no cap
   initially (match .NET SDK). Add a cap option in Phase 3 if needed.

6. **`tokio_util` dependency** — `CancellationToken` requires `tokio_util`. Is this
   acceptable for the driver crate? Alternative: implement a lightweight cancel
   signal using `Arc<AtomicBool>` + `tokio::sync::Notify`.

---

## Appendix A: .NET SDK Source References

- [`AvailabilityStrategy.cs`](https://github.com/Azure/azure-cosmos-dotnet-v3/blob/master/Microsoft.Azure.Cosmos/src/Routing/AvailabilityStrategy/AvailabilityStrategy.cs) — Public factory methods
- [`CrossRegionHedgingAvailabilityStrategy.cs`](https://github.com/Azure/azure-cosmos-dotnet-v3/blob/master/Microsoft.Azure.Cosmos/src/Routing/AvailabilityStrategy/CrossRegionHedgingAvailabilityStrategy.cs) — Core implementation (410 lines)
- [`DisabledAvailabilityStrategy.cs`](https://github.com/Azure/azure-cosmos-dotnet-v3/blob/master/Microsoft.Azure.Cosmos/src/Routing/AvailabilityStrategy/DisabledAvailabilityStrategy.cs) — Sentinel for per-request disable
- [`AvailabilityStrategyInternal.cs`](https://github.com/Azure/azure-cosmos-dotnet-v3/blob/master/Microsoft.Azure.Cosmos/src/Routing/AvailabilityStrategy/AvailabilityStrategyInternal.cs) — Internal abstract base

## Appendix B: Glossary

| Term | Definition |
|------|-----------|
| Hedging | Sending the same request to multiple regions; first response wins |
| Threshold | Time before the first hedge request fires |
| ThresholdStep | Time between subsequent hedge requests |
| Final result | A response that is definitively non-transient (success or permanent error) |
| Transient result | A response that might succeed in another region (5xx, timeout, etc.) |
| PPAF | Per-Partition Automatic Failover (write failover on single-master) |
| PPCB | Per-Partition Circuit Breaker (read/write failover on failure threshold) |
| MM | Multi-master (multi-write-region) account |
| SM | Single-master account |
