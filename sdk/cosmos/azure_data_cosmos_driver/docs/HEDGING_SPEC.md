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

- Hedging within a single region (e.g., across gateway nodes).
- Automatic threshold tuning based on observed latency histograms (future work).

All Cosmos DB operation types are addressed by the phased rollout below.
Nothing is permanently excluded — stored procedures / triggers / UDFs and
adaptive-tuning are deferred to the Future bucket pending a separate
design review.

### Operation-type scope (phased)

| Operation type | Phase 1 | Phase 2 | Phase 3 | Future |
|---|:---:|:---:|:---:|:---:|
| Document point reads (GetItem) | ✅ | ✅ | ✅ | ✅ |
| Document point writes on multi-master (Create/Replace/Upsert/Delete/Patch) | ✅ | ✅ | ✅ | ✅ |
| Queries (`QueryItems`) | ❌ | ✅ | ✅ | ✅ |
| `ReadMany` | ❌ | ✅ | ✅ | ✅ |
| Change feed | ❌ | ❌ | ✅ | ✅ |
| Metadata operations (Database / Container / Offer / Throughput) | ❌ | ❌ | ✅ | ✅ |
| Stored procedures / triggers / UDFs execution | ❌ | ❌ | ❌ | 🟡 candidate |

.NET v3 documents Query / ReadMany / ChangeFeed as supported by
`CrossRegionHedgingAvailabilityStrategy`
([source](https://github.com/Azure/azure-cosmos-dotnet-v3/blob/main/docs/Cross%20Region%20Request%20Hedging.md));
Java v4 ships read + write + query + ReadMany hedging today. The Rust
driver phases these in sequentially because Query / ReadMany / ChangeFeed
each carry pagination or checkpoint semantics (continuation tokens,
multi-page state, change-feed lease state) that need their own design
pass before they can be safely fanned out across regions. Metadata
operations are control-plane and rarely latency-critical, but are
included in Phase 3 to provide complete operation coverage where it is
safe and cheap. Sprocs / triggers / UDFs are deferred to Future because
their server-side execution model interacts with hedging in non-obvious
ways (server-side state, idempotency). See §16 for the full rollout plan.

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

### 2.6 PPAF / PPCB Integration

When PPAF (Per-Partition Automatic Failover) is enabled on the account and the
user has not specified a custom availability strategy, the .NET SDK
**automatically enables** a default cross-region hedging strategy
(`IsSDKDefaultStrategyForPPAF = true`). This provides latency protection
alongside PPAF's failover protection.

**Default values used by .NET (PPAF-driven)**
([source](https://github.com/Azure/azure-cosmos-dotnet-v3/blob/main/docs/Cross%20Region%20Request%20Hedging.md)):

- **Threshold:** `min(1000ms, RequestTimeout / 2)` (falls back to `1000ms` if
  `RequestTimeout == 0`)
- **Threshold step:** `500ms`
- **Write hedging:** disabled

PPCB (Per-Partition Circuit Breaker) on its own does **not** auto-enable
hedging in .NET. However, .NET implicitly turns PPCB on whenever PPAF is
enabled
([CosmosClientOptions.cs `EnablePartitionLevelCircuitBreaker`](https://github.com/Azure/azure-cosmos-dotnet-v3/blob/main/Microsoft.Azure.Cosmos/src/CosmosClientOptions.cs)),
so a PPAF-enabled deployment ends up running with all three (PPAF + PPCB +
hedging) active simultaneously.

**The Rust driver matches .NET exactly:** the SDK-default hedging strategy
is auto-enabled **only when PPAF is enabled** on the account and the user
has not configured a custom strategy. Enabling PPCB on its own (without
PPAF) does **not** auto-enable hedging — PPCB is a failure-driven circuit
breaker and does not by itself signal that the application wants latency
hedging. Users who want hedging without PPAF must configure an
`AvailabilityStrategy` explicitly. See §5.2 for the full activation rules
and lifecycle.

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

> **Divergence from .NET:** the .NET v3 constructor accepts a nullable
> `thresholdStep` and silently coerces `null` to a `-1ms` sentinel via `??`
> ([source](https://github.com/Azure/azure-cosmos-dotnet-v3/blob/master/Microsoft.Azure.Cosmos/src/Routing/AvailabilityStrategy/CrossRegionHedgingAvailabilityStrategy.cs#L60))
> — a likely latent bug, since the same parameter is also checked against
> `<= TimeSpan.Zero` immediately above (`null` slips through that comparison).
> The Rust API requires both `threshold` and `threshold_step` to be explicit
> non-zero `Duration` values.

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

**Decision matrix** — evaluated in order; first matching row wins:

| # | Condition | Hedge? |
|---:|-----------|--------|
| 1 | No strategy resolved (or `AvailabilityStrategy::Disabled`) | No |
| 2 | Application preferred-region list empty (no fan-out targets) | No |
| 3 | `ResourceType != Document` | No |
| 4 | Read: applicable `preferred_read_endpoints` (after `ExcludeRegions`) has < 2 entries | No |
| 5 | Write + single-master | No |
| 6 | Write + multi-master + `enable_multi_write_region_hedge = false` | No |
| 7 | Write + multi-master + applicable `preferred_write_endpoints` (after `ExcludeRegions`) has < 2 entries | No |
| 8 | Read with ≥ 2 applicable read endpoints | Yes |
| 9 | Write + multi-master + `enable_multi_write_region_hedge = true` + ≥ 2 applicable write endpoints | Yes |

The "≥ 2 applicable endpoints" check is computed against the
operation-appropriate list **after** `ExcludeRegions` filtering, not the
raw account region count — a user who excludes all-but-one region at the
operation level will (correctly) skip hedging even on a multi-region
account.

> **Divergence from .NET:** .NET's single-region bypass test in
> `ExecuteAvailabilityStrategyAsync` checks `ReadEndpoints.Count == 1` for
> *all* operations, including writes
> ([source](https://github.com/Azure/azure-cosmos-dotnet-v3/blob/master/Microsoft.Azure.Cosmos/src/Routing/AvailabilityStrategy/CrossRegionHedgingAvailabilityStrategy.cs#L108)).
> The Rust impl uses the operation-appropriate region list
> (`preferred_read_endpoints` for reads, `preferred_write_endpoints` for
> multi-master writes), which is intentionally more precise: a multi-region
> read account that has only one *write* region should still hedge writes
> only when the write list has ≥ 2 entries.

### 5.2 Default Hedging Enablement Driven by PPAF

When the user has not configured a custom `AvailabilityStrategy` (via
client-level options or per-operation options), the driver auto-enables a
default hedging strategy whenever **PPAF** is enabled on the account. This
matches .NET v3 exactly. **Enabling PPCB alone does not auto-enable
hedging** — see §2.6 for the rationale.

"Enable" here means the hedging orchestrator becomes active for eligible
operations — no separate type or factory needs to be constructed by the user.

**Activation rules** (evaluated against the latest account properties):

| PPAF | PPCB | User strategy | Effective strategy |
|:---:|:---:|---|---|
| off | off | none | none (hedging off) |
| off | off | `Hedging(…)` | user strategy |
| off | off | `Disabled` | none (hedging off) |
| off | **on** | none | none (hedging off — PPCB alone does not enable hedging) |
| **on** | off | none | **SDK default** (PPAF-driven) |
| **on** | **on** | none | **SDK default** (PPAF-driven) |
| on/off | on/off | `Hedging(…)` | user strategy (always wins) |
| on/off | on/off | `Disabled` | none (user opt-out wins) |

**Default values used when auto-enabled** (see §5.2.1 below for the cross-SDK
comparison and the rationale for these specific values):

- **Threshold:** `min(1000ms, request_timeout / 2)`, with `1000ms` as fallback
  when `request_timeout` is unset or zero.
- **Threshold step:** `500ms`.
- **Write hedging:** **enabled in Phase 1 only when the account is
  multi-master**. Single-master accounts never hedge writes (PPAF
  handles write redirection there). On multi-master accounts the
  PPAF-driven default applies to writes as well as reads (see §16
  Phase 1 scope).

**Lifecycle** — the SDK-default strategy is dynamic with the account:

- It is enabled the first time an account-properties refresh reports PPAF
  on, and there is no user strategy.
- It is removed if a subsequent refresh reports PPAF off again.
- It is **never** activated when the user has explicitly configured a
  strategy, even `AvailabilityStrategy::Disabled` — the user-level value
  always wins over the SDK default.

**Preconditions (mandatory)** — even when PPAF is enabled and the SDK
default would otherwise activate, hedging is **skipped at runtime** for
operations that fail any of the following checks:

1. **At least two applicable regions.** The operation's preferred-endpoint
   list (`preferred_read_endpoints` for reads, `preferred_write_endpoints`
   for multi-master writes) — after `ExcludeRegions` filtering — must
   contain ≥ 2 entries. Single-region accounts and accounts where the
   user has excluded all but one region skip hedging (see §5.1).
2. **Application-level region configuration is required.** The driver
   must have a non-empty preferred-region list to derive the fan-out
   order from. This is set via `ApplicationRegion` /
   `ApplicationPreferredRegions` in .NET, or via
   `DriverOptions::preferred_regions` /
   `OperationOptions::application_preferred_regions` in Rust. Without it,
   the driver has no ordered hedge-target list and falls back to
   single-region routing — even when PPAF is enabled.

Both checks are enforced inside `should_hedge()` (§5.1); failure of either
short-circuits the orchestrator before the primary request is sent.

#### 5.2.1 Cross-SDK comparison of default thresholds

> **.NET v3 SDK** — PPAF auto-enables a default hedging strategy via
> `SDKDefaultCrossRegionHedgingStrategyForPPAF`
> ([source](https://github.com/Azure/azure-cosmos-dotnet-v3/blob/main/Microsoft.Azure.Cosmos/src/Routing/AvailabilityStrategy/AvailabilityStrategy.cs)):
> threshold = `min(1000ms, RequestTimeout / 2)`, step = `500ms`, write
> hedging disabled. PPCB on its own does not auto-enable hedging, but PPAF
> implicitly enables PPCB.
>
> **Java v4 SDK** — `ThresholdBasedAvailabilityStrategy` ships with
> `DEFAULT_THRESHOLD = 500ms` and `DEFAULT_THRESHOLD_STEP = 100ms`
> ([source](https://github.com/Azure/azure-sdk-for-java/blob/main/sdk/cosmos/azure-cosmos/src/main/java/com/azure/cosmos/ThresholdBasedAvailabilityStrategy.java#L15-L16));
> PPAF auto-enables this default (gated by
> `COSMOS.IS_PER_PARTITION_AUTOMATIC_FAILOVER_ENABLED`, opt-out via
> `COSMOS.IS_READ_AVAILABILITY_STRATEGY_ENABLED_WITH_PPAF`).
>
> **Rust driver** — follows .NET's threshold formula
> (`min(1000ms, request_timeout / 2)` / `500ms`) and .NET's activation
> trigger (PPAF only, not PPCB) so behavior matches the dominant SDK.
> Users targeting cross-SDK latency parity with Java should explicitly
> configure `500ms / 100ms`.

```rust
// In cosmos_driver.rs, during account properties sync
let user_strategy = resolved_options.availability_strategy();
let ppaf_enabled = account_properties.enable_per_partition_failover_behavior;

// PPCB on its own does NOT auto-enable hedging (matches .NET).
if user_strategy.is_none() && ppaf_enabled {
    // Compute the .NET-compatible threshold from request_timeout.
    let request_timeout = resolved_options.request_timeout();
    let threshold = match request_timeout {
        Some(t) if !t.is_zero() => Duration::from_millis(1000).min(t / 2),
        _ => Duration::from_millis(1000),
    };
    let default_strategy = HedgingStrategy::sdk_default(
        threshold,
        Duration::from_millis(500),
        // Phase 1: enable write hedging by default when the account is
        // multi-master; it is a no-op on single-master accounts because
        // should_hedge() rejects writes there (see §5.1).
        /* enable_multi_write_region_hedge */
        account_properties.can_use_multiple_write_locations(),
    );
    // Mark as SDK-default so diagnostics can distinguish it from a
    // user-configured strategy (mirrors .NET's IsSDKDefaultStrategyForPPAF).
    resolved_options.set_sdk_default_strategy(default_strategy);
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

> **Important orchestration note:** The pseudocode below is presented as a
> spawn loop followed by a drain loop for readability, but a faithful
> implementation must **interleave result observation with the spawn timer**
> the way .NET does. .NET uses a single `do { Task.WhenAny(requestTasks) }
> while (winner == hedgeTimer && !winner.IsCompleted)` per iteration, so a
> final result observed mid-fan-out short-circuits the loop and prevents
> launching unnecessary hedges. The two-phase form below would launch every
> hedge regardless of when a winner appears, defeating the cancellation
> guarantee. The Rust impl SHOULD use a single `tokio::select!` per spawn
> step that races (a) the spawn-delay timer, (b) the next completion from
> the in-flight `FuturesUnordered`, and (c) the parent cancellation token,
> so a final result during the fan-out cancels remaining spawns immediately.

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
///
/// Note: 403 (with or without sub-status `3` WriteForbidden) is **transient**
/// for hedging — it typically indicates the targeted region cannot serve the
/// request right now (account-level failover, write-region change). The retry
/// pipeline running *inside* each hedge may treat `403/3` as a redirect
/// trigger; that is independent of this classification.
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
| 403 | 0 (no sub) | **Yes** | Forbidden — may indicate a regional failover in progress; another region may serve |
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

> **Note on 403 sub-statuses.** The driver classifies any 403 (with or
> without `WriteForbidden` sub-status `3`) as **transient** for hedging
> purposes — a 403 typically signals that the targeted region cannot
> currently serve the request (account-level failover in progress, write
> region change, etc.), and the *correct* action is to keep racing other
> in-flight hedges. This matches .NET v3's `IsFinalResult` behavior. Note
> that the *retry layer* may treat `403/3` differently (PPAF write retry
> consumes it as a write-redirect signal); the hedging classification
> here governs only whether hedging itself terminates early, not the
> retry pipeline that runs *inside* each hedge.

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

### 8.4 Local-Only Retries Inside a Hedge (Contract)

> **Contract:** Each hedged pipeline invocation runs the **full operation
> pipeline including the retry layer**, but the retry layer must perform
> **local-only retries** — it is forbidden from re-routing the request to
> a different region than the one targeted by that hedge.

This matches the .NET v3 behavior documented in
[Cross Region Request Hedging.md](https://github.com/Azure/azure-cosmos-dotnet-v3/blob/main/docs/Cross%20Region%20Request%20Hedging.md):
*"hedged requests are restricted to the region they are sent out in so no
cross region retries will be made, only local retries."*

**Mechanism.** The orchestrator enforces this implicitly via
`ExcludeRegions` (see §6.3): hedge `N` is sent with
`ExcludeRegions = all_regions \ { regions[N] }`. The retry layer's
region-fallback logic (PPAF / PPCB / 503 retry) consults
`ExcludeRegions` when picking the next endpoint and therefore has no
alternate region to fall back to — every retry attempt re-resolves to the
same target region.

**Why this matters.** Without this property, two hedges launched against
different regions could converge onto the *same* fallback region during a
regional outage, defeating the hedge's purpose and inflating RU.

**Implementation requirement.** The retry layer must:

1. Always honor `ExcludeRegions` from the operation options when computing
   the next endpoint after a retry trigger.
2. Treat "all eligible regions excluded" as a terminal condition — return
   the last seen response/error rather than looping infinitely.
3. Not bypass `ExcludeRegions` for any retry trigger class (PPAF write
   retry, PPCB markdown failback, transport-layer 503, throttling, etc.).

This is a **cross-cutting invariant** that any new retry trigger added
after Phase 1 must respect.

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

> **Divergence from .NET diagnostic shape:** .NET attaches diagnostics in two
> different shapes depending on which loop produces the winner.
> - **Fast path** (winner emerges during the spawn-and-race loop): only
>   `HedgeConfig` is attached when the **primary** wins (`requestNumber == 0`);
>   when a hedge wins, `HedgeContext = hedgeRegions.Take(requestNumber + 1)`
>   and `ResponseRegion` are also attached
>   ([source](https://github.com/Azure/azure-cosmos-dotnet-v3/blob/master/Microsoft.Azure.Cosmos/src/Routing/AvailabilityStrategy/CrossRegionHedgingAvailabilityStrategy.cs#L198-L224)).
> - **Drain path** (winner emerges from the post-loop drain after all hedges
>   were launched): all three fields are always attached, with
>   `HedgeContext = hedgeRegions` (the full list).
>
> The Rust impl simplifies this by **always** attaching the full
> `HedgeDiagnostics` (config, regions launched, winner region, was_hedge flag)
> whenever a hedging strategy was active for the operation — even when the
> primary wins immediately. This is strictly more informative and avoids the
> bookkeeping required to mirror .NET's two-shape behavior.

**Attachment contract.** `DiagnosticsContext::hedge_diagnostics` is
`Some(_)` if and only if a hedging strategy was **resolved and active**
for the operation — i.e. `should_hedge()` returned `true` and the
orchestrator was entered. It is `None` in all of the following cases:

- No `AvailabilityStrategy` was resolved (no client/operation/SDK-default
  strategy, or the user set `AvailabilityStrategy::Disabled`).
- A strategy was resolved but `should_hedge()` returned `false` (e.g.
  fewer than 2 applicable preferred regions, or non-Document operation).
- The strategy resolved but the orchestrator short-circuited before
  spawning the primary (e.g. cancellation observed at entry).

**Field semantics when the primary wins before the first hedge fires:**

| Field | Value |
|---|---|
| `strategy_config` | The active strategy config (always populated) |
| `regions_contacted` | `vec![regions[0]]` (just the primary) |
| `response_region` | `regions[0]` |
| `total_requests_launched` | `1` |
| `was_hedge` | `false` |

This lets callers distinguish *"hedging was active and the primary won
amongst the launched requests"* from *"hedging was active but no hedge
ever fired because the primary returned within the threshold window"*.

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

### 10.4 Reserved Telemetry / Metrics Surface

Neither .NET v3 nor the current Rust driver emits quantitative metrics for
hedging beyond the structured per-response `HedgeDiagnostics`. Phase 1 will
ship without metrics, but the spec **reserves** the following surface so
that later phases (or a separate observability PR) can add them without
breaking changes.

**Reserved `tracing` event names** (under target `cosmos.hedge`):

| Event | Level | Fields | Emitted when |
|---|---|---|---|
| `cosmos.hedge.enabled_for_operation` | DEBUG | `threshold_ms`, `step_ms`, `region_count`, `is_sdk_default` | Orchestrator decides to hedge a specific operation |
| `cosmos.hedge.spawned` | DEBUG | `request_number`, `target_region`, `elapsed_ms` | A hedge request task is spawned |
| `cosmos.hedge.canceled` | DEBUG | `request_number`, `target_region`, `reason` (`winner_found` / `deadline` / `app_canceled`) | A non-winning hedge is canceled |
| `cosmos.hedge.won` | INFO | `request_number`, `winner_region`, `elapsed_ms`, `was_primary` | A response is selected as final |
| `cosmos.hedge.all_transient` | WARN | `regions_attempted`, `last_status_code` | Drain loop returned the last transient response |

**Reserved metric names** (intentionally namespaced; not emitted in
Phase 1, awaiting an `azure_core` metrics surface):

| Metric | Type | Labels | Description |
|---|---|---|---|
| `cosmos.hedge.operations_total` | counter | `result` (`primary_won` / `hedge_won` / `all_transient` / `disabled`) | Hedging-eligible operations grouped by outcome |
| `cosmos.hedge.requests_spawned_total` | counter | `request_number` | Total hedge requests spawned (primary = 0) |
| `cosmos.hedge.first_response_latency_ms` | histogram | `was_primary` (bool) | Latency from orchestrator entry to the winning response |
| `cosmos.hedge.canceled_total` | counter | `reason` (`winner_found` / `deadline` / `app_canceled`) | Hedges that were canceled before completion |
| `cosmos.hedge.ru_charge_winner` | histogram | `was_primary` | RU of the winning response (matches what is reported to the caller) |

Notes:

- RU consumed by losing hedges is **not** reported to the caller (see §17
  Q3); a separate metric should be introduced if operators need to track
  hedge cost separately from per-operation cost.
- Histogram bucket layout intentionally unspecified — defer to whichever
  metrics provider `azure_core` settles on.
- Event/metric names follow OpenTelemetry conventions: dot-separated,
  lower-snake-case, namespaced under `cosmos.hedge`.

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

#### 11.3.1 Availability-strategy resolution priority

The driver picks the effective strategy in the following priority order
(highest first), mirroring the .NET resolution model:

| Priority | Source | Notes |
|:---:|---|---|
| 1 | Operation `availability_strategy` (incl. `Disabled`) | Per-request override |
| 2 | Client / runtime `availability_strategy` | Applies to all requests |
| 3 | **SDK default** (PPAF-driven, see §5.2) | Auto-enabled when account has PPAF on and no user strategy is set. PPCB alone does not auto-enable hedging. |
| 4 | None | Hedging off |

A user-configured `AvailabilityStrategy::Disabled` at any layer suppresses the
SDK default — explicit opt-out always wins.

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

### Phase 1: Read + Write Hedging + PPAF Default Enablement (MVP)

**Scope:**
- `HedgingStrategy` and `AvailabilityStrategy` types
- `should_hedge()` covering document point reads and document point
  writes on multi-master accounts
- `is_final_result()`
- `execute_with_hedging()` orchestrator
- `HedgeDiagnostics`
- Integration into `cosmos_driver.rs`
- Cancellation via `CancellationToken`
- `enable_multi_write_region_hedge` configuration knob (with
  documentation of 409 / 412 risk for non-idempotent upserts; see §13)
- **Auto-enable the SDK-default hedging strategy when PPAF is enabled
  on the account and the user has not configured a strategy** (see
  §5.2). The SDK default covers reads always, and writes when the
  account is multi-master. **Enabling PPCB alone does not auto-enable
  hedging** (matches .NET).
- Lifecycle handling: enable on account-properties refresh when PPAF
  appears, remove when PPAF goes away again. Re-evaluate the
  multi-master flag on each refresh so write coverage tracks the
  account state.
- Unit tests + fault injection tests for both reads and writes
  (including PPAF-on, PPCB-on without PPAF (must NOT activate), and
  PPAF+PPCB activation matrix coverage; single-master vs. multi-master
  write coverage)
- Environment variable support

**Deliverables:**
- New files: `hedging.rs`, `hedging_diagnostics.rs`
- Modified: `cosmos_driver.rs`, `operation_options.rs`, `mod.rs`

### Phase 2: Query + ReadMany Hedging

**Scope (deferred — design pass required before scheduling):**
- Extend `should_hedge()` to allow `OperationType::Query` and
  `OperationType::ReadMany`.
- Continuation-token semantics: per-page hedging vs. per-operation
  hedging; how to compose hedge-winner selection with continuation-token
  forwarding so callers see a consistent paging experience.
- ReadMany batching: each underlying point read is independently
  hedge-able today via Phase 1; whole-call hedging needs coordination
  so that one slow identity does not block the entire batch.
- Query plan caching interaction: query-plan fetches issued before the
  orchestrator starts must not skew the hedge fan-out (see Phase 3
  diagnostics caveat below).
- Aligns Rust scope with Java v4, which already ships Query / ReadMany
  hedging.

### Phase 3: Change Feed + Metadata Hedging

**Scope (deferred — design pass required before scheduling):**
- Extend `should_hedge()` to allow `OperationType::ReadFeed`
  (change feed) and metadata operations (Database / Container / Offer /
  Throughput reads and updates).
- Change-feed checkpointing: ensure hedged change-feed reads do not
  produce divergent continuation tokens; lease-state interactions for
  change-feed processor scenarios must be documented.
- Metadata cache invalidation: hedged metadata reads must not produce
  stale-cache races when one region returns an older view than another;
  decide whether to prefer the latest `_etag` / resource id or the
  fastest response.
- **Diagnostics caveat for multi-phase operations** (applies to Phase 2
  Query and Phase 3 ChangeFeed alike): Query, ReadMany, and ChangeFeed
  may contact regions *before* the hedge orchestrator starts — query
  plan fetches, partition-key-range cache loads, and identity-batching
  pre-flights all hit the gateway/region in the normal pipeline.
  `HedgeDiagnostics::regions_contacted` covers only the regions the
  orchestrator itself fanned out to; pre-hedge contacts show up in the
  surrounding `DiagnosticsContext` (existing per-attempt region trail).
  Phase 2 / Phase 3 designs must specify how the two surfaces compose
  so operators can tell hedge-driven contacts apart from setup-driven
  contacts.

### Future: Stored Procedures / Triggers / UDFs + Adaptive Thresholds

**Scope:**
- Stored procedure, trigger, and UDF execution hedging — candidate
  only. Server-side execution interacts with hedging in non-obvious
  ways (state mutation, idempotency, body cloning of script payloads)
  and needs a separate design proposal.
- Latency histogram tracking per-region
- Auto-tuning threshold based on p50 / p90 latency
- Exponential backoff on hedge threshold after repeated hedges

---

## 17. Open Questions

1. **Should hedging be enabled by default?** — **Resolved.** Off by default
   when PPAF is not enabled on the account. When the user opts into PPAF,
   the driver auto-enables the SDK-default hedging strategy (see §5.2).
   This matches .NET v3 exactly. Enabling PPCB alone does **not**
   auto-enable hedging — PPCB is failure-driven and does not by itself
   signal a desire for latency hedging. Phase 1 covers reads and writes
   on multi-master accounts; Phase 2 extends to Query and ReadMany;
   Phase 3 covers Change Feed and metadata (see §16).

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
