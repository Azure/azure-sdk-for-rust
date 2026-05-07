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

| Operation type | Phase 1 | Phase 2 | Future |
|---|:---:|:---:|:---:|
| Document point reads (GetItem) | ✅ | ✅ | ✅ |
| Document point writes on multi-master (Create/Replace/Upsert/Delete/Patch) | ✅ | ✅ | ✅ |
| Queries (`QueryItems`) | ✅ | ✅ | ✅ |
| `ReadMany` | ✅ | ✅ | ✅ |
| Change feed | ✅ | ✅ | ✅ |
| Metadata operations (Database / Container / Offer / Throughput) | ❌ | ✅ | ✅ |
| Stored procedures / triggers / UDFs execution | ❌ | ❌ | 🟡 candidate |

.NET v3 documents Query / ReadMany / ChangeFeed as supported by
`CrossRegionHedgingAvailabilityStrategy`
([source](https://github.com/Azure/azure-cosmos-dotnet-v3/blob/main/docs/Cross%20Region%20Request%20Hedging.md));
Java v4 ships read + write + query + ReadMany hedging today. The Rust
driver matches the dominant SDKs by covering the same document-scoped
operation surface in Phase 1 — Query, ReadMany, and ChangeFeed all
travel as `ResourceType.Document` over the wire and are hedged by the
same orchestrator. Metadata operations are control-plane and rarely
latency-critical; they are deferred to Phase 2 to provide complete
operation coverage where it is safe and cheap. Sprocs / triggers / UDFs
are deferred to Future because their server-side execution model
interacts with hedging in non-obvious ways (server-side state,
idempotency). See §16 for the full rollout plan.

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

/// Configuration error returned by fallible `HedgingStrategy` constructors.
#[derive(Debug, thiserror::Error)]
pub enum HedgingConfigError {
    #[error("hedging threshold must be > 0, got {0:?}")]
    ZeroThreshold(Duration),
    #[error("hedging threshold_step must be > 0, got {0:?}")]
    ZeroThresholdStep(Duration),
}

impl HedgingStrategy {
    /// Creates a new hedging strategy.
    ///
    /// Returns `HedgingConfigError` if `threshold` or `threshold_step` is
    /// zero. Use this constructor whenever the values originate from a
    /// runtime source (env var, remote config, JSON file, user input) so a
    /// malformed value can be surfaced without aborting the process.
    pub fn new(
        threshold: Duration,
        threshold_step: Duration,
    ) -> Result<Self, HedgingConfigError> { ... }

    /// Infallible constructor for callers that have compile-time-validated
    /// values (e.g., `Duration::from_millis(500)` literals in tests or
    /// hand-tuned defaults).
    ///
    /// # Panics
    /// Panics if `threshold` or `threshold_step` is zero.
    pub fn new_unchecked(threshold: Duration, threshold_step: Duration) -> Self { ... }

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

> **Divergence from .NET and Java:** the .NET v3 constructor accepts a
> nullable `thresholdStep` and silently coerces `null` to a `-1ms`
> sentinel via `??`
> ([source](https://github.com/Azure/azure-cosmos-dotnet-v3/blob/master/Microsoft.Azure.Cosmos/src/Routing/AvailabilityStrategy/CrossRegionHedgingAvailabilityStrategy.cs#L60))
> — a likely latent bug, since the same parameter is also checked
> against `<= TimeSpan.Zero` immediately above (`null` slips through
> that comparison). Java v4's `ThresholdBasedAvailabilityStrategy`
> rejects `null` or `isNegative()` durations but **accepts
> `Duration.ZERO`**
> ([source](https://github.com/Azure/azure-sdk-for-java/blob/main/sdk/cosmos/azure-cosmos/src/main/java/com/azure/cosmos/ThresholdBasedAvailabilityStrategy.java)),
> and additionally exposes a no-arg constructor that fills in built-in
> defaults (`500ms / 100ms`). The Rust API requires both `threshold`
> and `threshold_step` to be explicit non-zero `Duration` values —
> stricter than .NET (which rejects `<= 0` but lets `null` slip), and
> stricter than Java (which accepts zero). There is no no-arg
> constructor: the user must always supply both values (or rely on the
> PPAF SDK default in §5.2).

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
| `AZURE_COSMOS_HEDGING_THRESHOLD_STEP_MS` | Step interval in milliseconds | `500` |
| `AZURE_COSMOS_HEDGING_ENABLE_MULTI_WRITE_REGION` | `true`/`false` | `false` |
| `AZURE_COSMOS_HEDGING_DISABLE_SDK_DEFAULT` | `true`/`false` — suppresses the PPAF-driven SDK default (§5.2) without forcing every call site to set `AvailabilityStrategy::Disabled`. Equivalent to Java v4's `COSMOS.IS_READ_AVAILABILITY_STRATEGY_ENABLED_WITH_PPAF=false`. | `false` |

Environment variables provide the lowest-priority configuration above the
SDK default (see §11.3.1). Setting the threshold env var implicitly enables
hedging at the runtime level. Setting `AZURE_COSMOS_HEDGING_DISABLE_SDK_DEFAULT=true`
is equivalent to setting `DriverOptions::disable_sdk_default_hedging(true)`
in code: it suppresses *only* the SDK-default strategy, leaving any
user-configured `AvailabilityStrategy` (client-level or per-operation)
untouched.

> **Default-step alignment.** The env-var step default (`500ms`) intentionally
> matches the PPAF-driven SDK-default step in §5.2 / §2.6, so a user who flips
> hedging on via env var sees the same fan-out cadence as a user who let PPAF
> auto-enable it. The two paths share the `500ms` constant; if either ever
> changes, both must move together.

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
| 3 | `ResourceType` not in the **phase-allowed set** † | No |
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

> **† Phase-allowed `ResourceType` set.** Row 3's allowed set evolves with
> the implementation phases in §16, so the eligibility rule is encoded in
> one place rather than rewritten each phase:
>
> | Phase | Allowed `ResourceType` set |
> |---|---|
> | 1 (MVP)  | `{Document}` (covers point reads/writes, Query, ReadMany, ChangeFeed — all travel as `ResourceType.Document`) |
> | 2        | Phase 1 set ∪ `{Database, Container, Offer, Throughput}` |
> | Future   | Phase 2 set ∪ `{StoredProcedure, Trigger, UDF}` (see §16 "Future") |
>
> Phase 1 implementations should hard-code the set to `{Document}`,
> matching .NET's `ShouldHedge` exactly. Each subsequent phase widens
> the constant in one place; no other change to §5.1 is required.

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
- **Write hedging:** **disabled** — matches .NET v3 exactly. The
  SDK-default strategy never hedges writes, even on multi-master
  accounts. Users who want write hedging must construct an explicit
  `HedgingStrategy` and call
  `with_multi_write_region_hedge()` (see §13). Rationale: write hedging
  on multi-master can amplify 409 / 412 surface area (see §13.1) and is
  non-obvious to operators — it must be a deliberate opt-in, not an
  SDK-default behavior that activates implicitly the moment the account
  becomes MM.

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
> implicitly enables PPCB. There is no runtime opt-out for the SDK
> default — users must set a `RequestOptions.AvailabilityStrategy =
> AvailabilityStrategy.DisabledStrategy()` per request.
>
> **Java v4 SDK** — `ThresholdBasedAvailabilityStrategy` ships with
> `DEFAULT_THRESHOLD = 500ms` and `DEFAULT_THRESHOLD_STEP = 100ms`
> ([source](https://github.com/Azure/azure-sdk-for-java/blob/main/sdk/cosmos/azure-cosmos/src/main/java/com/azure/cosmos/ThresholdBasedAvailabilityStrategy.java#L15-L16));
> PPAF auto-enables this default (gated by
> `COSMOS.IS_PER_PARTITION_AUTOMATIC_FAILOVER_ENABLED`, opt-out via
> `COSMOS.IS_READ_AVAILABILITY_STRATEGY_ENABLED_WITH_PPAF`). Java
> notably has **no write-hedging flag at all** —
> `ThresholdBasedAvailabilityStrategy` is read-only by design; write
> hedging in Java is gated by separate system properties outside the
> strategy type itself. Java also exposes a no-arg constructor that
> uses the built-in defaults.
>
> **Rust driver** — follows .NET's threshold formula
> (`min(1000ms, request_timeout / 2)` / `500ms`) and .NET's activation
> trigger (PPAF only, not PPCB) so behavior matches the dominant SDK.
> Unlike .NET, Rust **does** expose a runtime opt-out for the SDK
> default (see §4.4 `AZURE_COSMOS_HEDGING_DISABLE_SDK_DEFAULT` and the
> `DriverOptions::disable_sdk_default_hedging` builder knob), aligning
> with Java's opt-out ergonomics. Write hedging is exposed on the same
> `HedgingStrategy` type via `with_multi_write_region_hedge()` rather
> than a separate strategy class. Users targeting cross-SDK latency
> parity with Java should explicitly configure `500ms / 100ms`.

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
        // Match .NET exactly: write hedging is OFF in the SDK-default
        // strategy even on multi-master accounts. Users who want write
        // hedging must opt in explicitly via
        // `HedgingStrategy::with_multi_write_region_hedge()`.
        // Rationale: write hedging on multi-master can amplify 409/412
        // surface area (see §13.1) and is non-obvious to operators —
        // it must be a deliberate opt-in, not an SDK-default behavior
        // that activates implicitly the moment the account becomes MM.
        /* enable_multi_write_region_hedge */ false,
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

The orchestrator interleaves spawn-timer waits with completion observation in
a single loop, so a final result observed mid-fan-out short-circuits the spawn
schedule. This faithfully translates the .NET pattern
(`do { Task.WhenAny(requestTasks) } while (winner == hedgeTimer && !winner.IsCompleted)`)
into Tokio idioms: `tokio::select!` races the spawn timer, the next completion
from a shared `FuturesUnordered`, and the parent cancellation token.

> **Why this shape (not a sequential spawn loop followed by a drain loop):**
> a two-phase form would launch every hedge regardless of when a winner
> appears — the spawn loop's `cancel.cancelled()` arm only fires *after* the
> drain loop sets the token, by which point all hedges have already been
> spawned. The single-loop form below ensures `cancel.cancel()` runs the
> instant a final result is observed, even while later spawn timers are
> still pending.

```rust
async fn execute_with_hedging(
    strategy: &HedgingStrategy,
    options: &OperationOptions,
    parent_cancel: &CancellationToken,
    /* ... all other pipeline params ... */
) -> Result<CosmosResponse> {
    let regions = get_applicable_regions(&account_state, &options.excluded_regions, is_read);
    if regions.len() <= 1 {
        // No alternate region — fall through to non-hedged path
        return execute_operation_pipeline(options, /* ... */).await;
    }

    // `cancel` is a child of the application's cancellation token: cancelling
    // it tears down all in-flight hedges; cancelling the parent also cancels
    // every hedge (and is re-raised to the caller below).
    let cancel = parent_cancel.child_token();
    let mut remaining: FuturesUnordered<JoinHandle<HedgeOutcome>> = FuturesUnordered::new();
    let mut last_transient: Option<HedgeOutcome> = None;

    for request_number in 0..regions.len() {
        // ── Compute the wait until *this* hedge should be spawned ──
        let wait = if request_number == 0 {
            Duration::ZERO  // Primary fires immediately
        } else if request_number == 1 {
            strategy.threshold()
        } else {
            strategy.threshold_step()
        };
        let spawn_at = tokio::time::Instant::now() + wait;

        // ── Single-select loop: race spawn timer ↔ next completion ↔ parent cancel ──
        // Skip the inner loop entirely on `request_number == 0` (wait == 0)
        // so the primary spawns without an extra scheduler round-trip.
        if !wait.is_zero() {
            loop {
                tokio::select! {
                    biased;

                    // Application cancellation: tear down hedges, but
                    // first attempt to harvest the most recent in-flight
                    // hedge's result so its trace/diagnostics survive into
                    // the returned error (mirrors .NET's
                    // `if (applicationProvidedCancellationToken.IsCancellationRequested)
                    //  { await (Task<HedgingResponse>)completedTask; }`).
                    _ = parent_cancel.cancelled() => {
                        cancel.cancel();
                        return Err(harvest_app_cancel_error(
                            &mut remaining,
                            last_transient.take(),
                            strategy,
                            &regions,
                        ).await);
                    }

                    // A previously-spawned hedge completed. If it's a final
                    // result we cancel the rest and return immediately —
                    // never spawning the hedges that were still pending.
                    Some(join_result) = remaining.next(), if !remaining.is_empty() => {
                        if let Some(response) = handle_completion(
                            join_result,
                            &cancel,
                            &mut last_transient,
                            &regions,
                            strategy,
                        ) {
                            return Ok(response);
                        }
                        // Transient → keep waiting for the same spawn deadline
                        // (note `spawn_at` is unchanged, so the timer naturally
                        //  shrinks as elapsed time accumulates).
                    }

                    // Spawn timer fires → break inner loop and launch the hedge.
                    _ = tokio::time::sleep_until(spawn_at) => break,
                }
            }
        }

        // ── Build per-hedge OperationOptions ──
        let hedge_options = if request_number == 0 {
            options.clone()  // Primary: no region override
        } else {
            // Compose the user's *original* exclusion set with the per-hedge
            // pin so the hedge can never widen routing back into a region the
            // user explicitly opted out of. `regions` is already post-filter,
            // so we must add the originals back in explicitly.
            let mut exclude: Vec<Region> = options
                .excluded_regions
                .as_ref()
                .map(|er| er.regions().to_vec())
                .unwrap_or_default();
            for (i, r) in regions.iter().enumerate() {
                if i != request_number && !exclude.contains(r) {
                    exclude.push(r.clone());
                }
            }
            let mut opts = options.clone();
            opts.excluded_regions = Some(ExcludedRegions::new(exclude));
            opts
        };

        // ── Spawn the hedge ──
        let cancel_child = cancel.child_token();
        let target_region = regions[request_number].clone();
        let task = tokio::spawn(async move {
            tokio::select! {
                result = execute_operation_pipeline(&hedge_options, /* ... */) => {
                    HedgeOutcome { request_number, region: target_region, result }
                }
                _ = cancel_child.cancelled() => {
                    HedgeOutcome {
                        request_number,
                        region: target_region,
                        result: Err(cancelled_error()),
                    }
                }
            }
        });
        remaining.push(task);
    }

    // ── All hedges spawned — drain remaining completions ──
    while let Some(join_result) = remaining.next().await {
        if let Some(response) = handle_completion(
            join_result,
            &cancel,
            &mut last_transient,
            &regions,
            strategy,
        ) {
            return Ok(response);
        }
        if parent_cancel.is_cancelled() {
            cancel.cancel();
            return Err(harvest_app_cancel_error(
                &mut remaining,
                last_transient.take(),
                strategy,
                &regions,
            ).await);
        }
    }

    // ── No hedge produced a final result ──
    cancel.cancel();
    match last_transient {
        Some(outcome) => outcome.result,
        None => Err(azure_core::Error::message(
            azure_core::error::ErrorKind::Other,
            "hedging completed without producing a response",
        )),
    }
}

/// Inspects a completed hedge. Returns `Some(response)` when this is a final
/// winner (caller must return it); returns `None` when the outcome was
/// transient or a panic (caller keeps racing).
fn handle_completion(
    join_result: Result<HedgeOutcome, JoinError>,
    cancel: &CancellationToken,
    last_transient: &mut Option<HedgeOutcome>,
    regions: &[Region],
    strategy: &HedgingStrategy,
) -> Option<CosmosResponse> {
    let outcome = match join_result {
        Ok(o) => o,
        Err(join_err) => {
            tracing::error!("hedged task panicked: {join_err}");
            return None;
        }
    };
    match &outcome.result {
        Ok(response) if is_final_result(response.status()) => {
            // Cancel the rest *immediately* — this is the load-bearing call
            // that the single-loop spawn shape exists to make timely.
            cancel.cancel();
            let mut response = outcome.result.unwrap();
            response.attach_hedge_diagnostics(HedgeDiagnostics {
                strategy_config: strategy.clone(),
                regions_contacted: regions[..=outcome.request_number].to_vec(),
                response_region: outcome.region,
            });
            Some(response)
        }
        _ => {
            *last_transient = Some(outcome);
            None
        }
    }
}

/// Builds the application-cancellation error after harvesting whatever
/// in-flight hedges can complete within a short bounded window. Any
/// outcome (success or transient) collected in the window contributes
/// its diagnostics to the returned error so the user-visible
/// `OperationCanceledException` carries the trace from the hedge that
/// was furthest along — mirroring the .NET v3 behavior where the
/// orchestrator awaits the faulted task before re-raising.
async fn harvest_app_cancel_error(
    remaining: &mut FuturesUnordered<JoinHandle<HedgeOutcome>>,
    mut last: Option<HedgeOutcome>,
    strategy: &HedgingStrategy,
    regions: &[Region],
) -> azure_core::Error {
    // Bounded so a stuck hedge can't extend user-visible cancel latency.
    const HARVEST_WINDOW: Duration = Duration::from_millis(50);
    let deadline = tokio::time::Instant::now() + HARVEST_WINDOW;

    while let Ok(Some(join_result)) = tokio::time::timeout_at(
        deadline,
        remaining.next(),
    ).await {
        if let Ok(outcome) = join_result {
            last = Some(outcome);
        }
    }

    let mut err = application_cancelled_error();
    if let Some(outcome) = last {
        err.attach_hedge_diagnostics(HedgeDiagnostics {
            strategy_config: strategy.clone(),
            regions_contacted: regions[..=outcome.request_number].to_vec(),
            response_region: outcome.region,
        });
    }
    err
}
```

#### 6.4.1 Ownership & Sharing

Because each hedge is spawned via `tokio::spawn`, every captured value must
be `'static + Send`. The orchestrator achieves this without per-iteration
heap allocation as follows:

| Value | Sharing strategy |
|---|---|
| `regions: Arc<Vec<Region>>` | Cloned (cheap `Arc::clone`) into each spawn for diagnostics; the per-hedge target is moved as an owned `Region`. |
| Pipeline parameters (`Arc<HedgeContext>`) | Bundled into a single `Arc` shared across spawns; each spawn clones the `Arc`. |
| `OperationOptions` (per hedge) | Cloned per spawn — the body is `Bytes` (cheap) and the rest is small `Copy`/`Arc` data. |
| `cancel_child: CancellationToken` | Constructed per spawn via `cancel.child_token()` and moved in. |
| `parent_cancel: &CancellationToken` | Borrowed only by the orchestrator itself — never crosses a spawn boundary. |
| `remaining: FuturesUnordered<JoinHandle<_>>` | Owned by the orchestrator; never shared. |

This keeps the orchestrator lock-free (no `Mutex`/`RwLock` introduced for
hedging state), matching the design principles in §3.3.

### 6.5 Key Invariants

1. **At most `regions.len()` concurrent requests** — one per region.
2. **Primary request fires immediately** — zero additional latency on the happy path.
3. **Hedge timers can be preempted** — if a winner arrives during a timer wait,
   no further hedges are launched.
4. **Cancellation is cooperative** — `CancellationToken` is checked at `select!`
   points inside `execute_operation_pipeline()` and at the transport layer via
   deadline enforcement.
5. **Single writer to diagnostics** — only the winning response gets hedge
   diagnostics attached.
6. **App-cancel preserves hedge trace** — when the application's cancellation
   token fires, the orchestrator harvests in-flight hedges within a bounded
   window (`HARVEST_WINDOW = 50ms`) and attaches the most-advanced hedge's
   diagnostics to the returned `application_cancelled_error()`. This mirrors
   .NET v3's `await (Task<HedgingResponse>)completedTask;` behavior so users
   see the trace from the request that was actually in flight when they
   cancelled.

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

### 8.5 File Layout

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

**Concurrent capture from competing hedges.** Even after the orchestrator
cancels losing hedges and returns the winner, a losing hedge's transport
future may already have received a response in flight (cancellation is
best-effort — see §14.2). If both the winner and a losing hedge call back
into `SessionManager` to record their captured tokens, the order in which
the writes land is non-deterministic. The hedging design relies on
**`SessionManager` updates being commutative under max-LSN merge**: each
write merges the incoming token with the stored token by taking the higher
LSN per-partition. Under that contract, late writes from a losing hedge are
safe (they can only advance the stored LSN, never regress it) and the next
operation always observes a token at least as fresh as the winner's.

This is a precondition on the existing `SessionManager` API — if it ever
moves to a last-write-wins model, the hedging orchestrator must instead
suppress session capture on hedges that observed cancellation before
STAGE 4 completed (or capture into a per-hedge buffer that only the winner
flushes). This invariant must be covered by the unit tests listed in §15.1.

### 9.3 Throughput Control

Each hedged request independently checks the throughput control group budget.
Hedging **does** increase RU consumption when hedge requests actually execute
transport. Users should account for this when setting throughput control limits.

The throughput control snapshot is acquired per-attempt in the operation pipeline
(STAGE 3), so concurrent hedges will see the latest budget.

**Pathological interaction under TC saturation.** When the throughput control
group is saturated, every hedge will be throttled to 429 by the local TC gate
before reaching the network. The orchestrator classifies 429 as transient
(see §7.2), drains every hedge, and returns the last 429 — i.e., under TC
saturation hedging actively makes the experience **worse** by multiplying
TC pressure and adding orchestration latency on top of the throttle.

This is amplified by the SDK-default auto-enable on PPAF accounts (§5.2):
a PPAF-enabled account under load gets hedging by default. Users can opt
out of that SDK-default behavior via
`DriverOptions::disable_sdk_default_hedging` or
`AZURE_COSMOS_HEDGING_DISABLE_SDK_DEFAULT` (§4.4 / §11.3.1), which
suppresses *only* the PPAF-driven default and leaves any user-configured
`AvailabilityStrategy` untouched. Setting `AvailabilityStrategy::Disabled`
at the operation or client level remains the broader way to disable
hedging entirely (and also blocks any per-operation `Hedging(..)`
override at lower layers).

**Mitigations the implementation must adopt:**

1. **Sizing guidance (operator-facing docs).** State explicitly that the
   maximum RU multiplier introduced by hedging is `regions.len()`, and TC
   group budgets should be sized with that headroom in mind when hedging
   is enabled.
2. **Short-circuit on local TC 429.** If the primary returns a TC-gate 429
   *before* reaching transport (i.e., the throttle is local rather than
   service-side), the orchestrator SHOULD treat that as a "do not fan out"
   signal — every hedge will hit the same gate. Distinguish this from a
   service-side 429 (which is genuinely region-local and benefits from
   hedging) via the response source field.
3. **Optional: exempt hedges from TC accounting.** Speculative-hedge RU is
   not user-attributable; the loser's RU is wasted by definition. A future
   option (`HedgingStrategy::with_throughput_control_exemption`) MAY skip
   TC accounting for hedge requests. Out of scope for Phase 1; tracked as
   a follow-up.

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
| `cosmos.hedge.ru_charge_winner` | histogram | `was_primary` | RU of the winning response; this is the caller-visible RU charge |
| `cosmos.hedge.ru_charge_total` | histogram | `winner_region`, `attempt_count` | Total RU consumed across all hedge attempts for the operation, including losing hedges; operator-facing only |

Notes:

- RU consumed by losing hedges is **not** reported to the caller. The
  external per-operation RU contract remains the winning response's RU
  charge, while any aggregate hedge cost is surfaced via the separate
  operator metric `cosmos.hedge.ru_charge_total`. See §17 Q3 for the
  resolved external-contract decision.
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
            // `HedgingStrategy::new` is fallible (see §4.1) — it returns
            // `Result<Self, HedgingConfigError>` for zero / non-positive
            // threshold or step. Propagate with `?` (or `expect` in
            // examples / tests where the inputs are static).
            HedgingStrategy::new(
                Duration::from_millis(500),
                Duration::from_millis(500),
            )?,
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
        )?,                              // see §4.1 — fallible constructor
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
(highest first), mirroring the .NET resolution model and extending it with
explicit env-var support:

| Priority | Source | Notes |
|:---:|---|---|
| 1 | Operation `availability_strategy` (incl. `Disabled`) | Per-request override |
| 2 | Client / runtime `availability_strategy` | Applies to all requests |
| 3 | Environment variables (§4.4) | Deploy-time intent; overrides SDK default but not code-level config |
| 4 | **SDK default** (PPAF-driven, see §5.2) | Auto-enabled when account has PPAF on and no user strategy is set. PPCB alone does not auto-enable hedging. |
| 5 | None | Hedging off |

A user-configured `AvailabilityStrategy::Disabled` at any layer suppresses every
lower layer (including the SDK default and env-var-derived strategy) — explicit
opt-out always wins.

The **SDK-default opt-out** (`DriverOptions::disable_sdk_default_hedging`
or `AZURE_COSMOS_HEDGING_DISABLE_SDK_DEFAULT=true`, see §4.4) is a
narrower kill-switch: it suppresses **only** priority 4 (the PPAF-driven
SDK default) without affecting priorities 1–3. Users who want PPAF
without the auto-enabled hedging strategy should use this flag rather
than setting `AvailabilityStrategy::Disabled` at the client level (which
would also block per-operation `Hedging(..)` overrides). Mirrors Java
v4's `COSMOS.IS_READ_AVAILABILITY_STRATEGY_ENABLED_WITH_PPAF=false`.

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
        // Use the post-`ExcludeRegions` applicable list (matches §5.1
        // row 7) — a user who has excluded all-but-one write region at
        // the operation level should correctly skip hedging even on a
        // multi-region MM account.
        && applicable_write_endpoints(account_state, &operation.excluded_regions).len() > 1
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

> **Divergence from .NET on application-cancel diagnostics.** When the
> *application* cancellation token fires mid-fan-out, .NET awaits the
> most-recently-completed task with no timeout (relying on it being
> already completed) before re-raising. The Rust orchestrator instead
> harvests in-flight hedges within a bounded `HARVEST_WINDOW = 50ms`
> window (see §6.4 / §6.5 invariant #6). This bounds user-visible
> cancel latency under a stuck transport future at the cost of
> occasionally returning slightly less-rich diagnostics than .NET would.
> Documented as best-effort: "diagnostics-on-cancel are attached when a
> hedge has produced a result within 50ms of cancellation."

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

The region list is captured at the start of `execute_with_hedging()`. If
account metadata refreshes during execution (e.g., a `RefreshAccountProperties`
effect), the region list for **already-launched** hedges is unchanged — each
hedge keeps the `ExcludeRegions` set it was spawned with.

Per the §8.4 contract, `ExcludeRegions` is a **hard constraint** inside each
hedge: `resolve_endpoint()` does *not* fall back to an excluded region even
if the metadata refresh has marked the only allowed region unavailable. If
an in-flight hedge ends up with no eligible endpoint, the retry layer
returns the terminal "all eligible regions excluded" condition (§8.4 item
2) as the result of that hedge.

**How the orchestrator handles that terminal condition.** The "all
eligible regions excluded" result from a single hedge is classified as
transient (§7.2) — it does *not* short-circuit the orchestrator. The
remaining hedges (each pinned to a different region) continue racing, and
whichever one produces a final result wins. If *every* hedge terminates
this way, §14.1 applies (return the last-seen transient response). This
preserves the per-hedge region-pinning invariant under metadata churn
without sacrificing availability.

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
| `hedging_config_validation` | Zero / non-positive `threshold` returns `HedgingConfigError` (matches the fallible `HedgingStrategy::new` contract in §4.1) |
| `hedging_config_requires_explicit_step` | `threshold_step` must be provided explicitly; constructor does not default it from `threshold` |
| `region_exclusion_for_hedge_n` | Correct ExcludeRegions per hedge |
| `exclude_regions_honored_by_every_retry_trigger` | For each retry trigger class — PPAF write retry, PPCB markdown failback, transport-layer 503, throttling 429, session-token 1002 — fault-inject the trigger inside a hedge and assert the retry attempt does **not** route to a region listed in the hedge's `ExcludeRegions`. Encodes the §8.4 cross-cutting invariant; new retry triggers added in later phases must extend this test. |
| `app_cancel_preserves_hedge_diagnostics` | Cancel the application token mid-fan-out; assert the returned error carries `HedgeDiagnostics` from the most-advanced in-flight hedge (covers §6.5 invariant #6). |

### 15.2 Integration Tests (Fault Injection)

| Test | Setup | Validates |
|------|-------|-----------|
| `hedging_read_primary_slow` | 2s delay on Region A reads | Hedge to Region B wins; diagnostics show `was_hedge=true` |
| `hedging_read_primary_fast` | No faults | Primary wins; no hedge launched; `hedge_diagnostics=Some(_)` with `was_hedge=false` and `total_requests_launched=1` (matches the §10.1 always-attached contract when the orchestrator is entered) |
| `hedging_read_primary_503` | 503 on Region A reads | Hedge to Region B wins with success |
| `hedging_read_all_regions_slow` | 2s delay on all regions | Last region to respond wins (graceful degradation) |
| `hedging_write_multi_master` | 2s delay on Region A creates | Hedge to Region B succeeds |
| `hedging_write_single_master_not_hedged` | 2s delay on writes | No hedge — write returns after delay |
| `hedging_disabled_per_operation` | Client hedging on; operation disabled | No hedge — normal path |
| `hedging_respects_deadline` | threshold > deadline | No hedge fires; deadline error |
| `hedging_with_ppcb` | 503 on Region A reads; PPCB enabled | PPCB and hedging both apply; circuit breaker tripped AND hedge succeeds |
| `hedging_cancels_losers` | Delay on Region A | Region B wins; verify Region A task cancelled (hit_count ≤ expected) |
| `hedging_failback_to_primary` | Region A initially slow, then fast | First few reads hedged; after threshold tightened, primary wins again |
| `hedging_exclude_regions_under_503_retry` | Region B inside hedge returns 503 (triggers transport retry) while Region C is healthy and excluded by that hedge's `ExcludeRegions` | Hedge B's retry stays pinned to Region B (does NOT fall back to Region C) — fault-injection counterpart to the §8.4 invariant unit test. |

### 15.3 Multi-Region Live Tests

Gated by `test_category = "multi_region"`:

| Test | Account Type | Validates |
|------|-------------|-----------|
| `hedging_read_cross_region` | 2-region SM | Read hedged to satellite when primary slow |
| `hedging_write_cross_region` | 2-region MM | Write hedged; handle 409 if applicable |
| `hedging_with_ppaf` | PPAF-enabled SM | Hedging + PPAF work together on write failure |

---

## 16. Implementation Phases

This section is the execution plan for the phased rollout introduced in §1
("Operation-type scope (phased)"). Each phase below explicitly maps to the
§1 operation-scope table rows it lights up and the §1 Goals it closes, so
the phases are auditable against the spec's own goal list rather than
drifting into a separate scope.

| §1 Goal | Phase that closes it |
|---|---|
| **G1. Reduce tail latency** (p99/p99.9 bounded by `threshold + RTT`) | Phase 1 (point reads + opt-in writes + Query + ReadMany + ChangeFeed — the full document-scoped surface that .NET v3 and Java v4 already hedge). Phase 2 widens to metadata. |
| **G2. Transparent to application** (single `CosmosResponse`; opt-in diagnostics) | Phase 1 (`HedgeDiagnostics`, `DiagnosticsContext` integration). Phase 2 extends the diagnostics surface to cover pre-hedge region contacts (see Phase 2 caveat). |
| **G3. Configurable** (threshold, step, write-hedging opt-in at client AND per-operation levels) | Phase 1 (client-level via `DriverOptions::availability_strategy`; per-operation override via `OperationOptions::availability_strategy`; `AvailabilityStrategy::Disabled` sentinel; env-var fallback; SDK-default opt-out). |
| **G4. Complementary to failover** (composes with PPAF/PPCB without interference) | Phase 1 (PPAF auto-enable, lock-free `LocationStateStore` interaction, §9.1). |
| **G5. Resource-safe** (losing hedges cancelled promptly to bound RU/transport waste) | Phase 1 (single-loop `tokio::select!` orchestrator + `CancellationToken` child tree, §6.4 / §12). |

§1 Non-Goals (single-region hedging, automatic threshold tuning) remain
out of scope for every phase below; adaptive-threshold work is tracked in
**Future** but is explicitly labelled as a *non-goal* of the current spec.

### Phase 1: Document-scoped Hedging + PPAF Default Enablement (MVP)

**Operation rows from §1 covered (Phase 1 column) — matches .NET v3 and Java v4:**
- Document point reads (`GetItem`)
- Document point writes on **multi-master** accounts: `CreateItem`,
  `ReplaceItem`, `UpsertItem`, `DeleteItem`, `PatchItem` — eligible only
  when the user has explicitly opted in via
  `HedgingStrategy::with_multi_write_region_hedge()` (matches .NET; see
  §5.2 / §13).
- `QueryItems` (single- and cross-partition)
- `ReadMany`
- Change feed (`ReadFeed`)

All five categories travel as `ResourceType.Document` over the wire and
are therefore covered by the same `should_hedge()` predicate (§5.1
row 3, Phase-1 allowed set = `{Document}`). This matches .NET v3's
`ShouldHedge` exactly and aligns with the Java v4 hedging surface.

**Scope:**
- `HedgingStrategy` and `AvailabilityStrategy` types (§4.1, §4.2).
- `should_hedge()` covering the operation set above (§5.1; phase-allowed
  set = `{Document}` per §5.1 row 3 footnote).
- `is_final_result()` (§7.1).
- `execute_with_hedging()` orchestrator with single-loop spawn+observe
  shape (§6.4) and `harvest_app_cancel_error` for trace-preserving
  application cancellation (§6.4 / §6.5 invariant #6).
- `HedgeDiagnostics` and `DiagnosticsContext` integration (§10).
- Integration into `cosmos_driver.rs` (§8.1).
- Cooperative cancellation via `CancellationToken` child tree (§12);
  loser hedges observe cancellation at every pipeline `select!` point
  → satisfies **G5**.
- `enable_multi_write_region_hedge` configuration knob with explicit
  documentation of 409 / 412 amplification risk for non-idempotent
  upserts (§13).
- **Per-operation override surface** (satisfies **G3**): `OperationOptions::availability_strategy`
  (§4.3) accepting `Some(AvailabilityStrategy::Hedging(..))`,
  `Some(AvailabilityStrategy::Disabled)`, or `None`; layered resolution
  per §11.3 / §11.3.1.
- **Auto-enable the SDK-default hedging strategy when PPAF is enabled
  on the account and the user has not configured a strategy** (§5.2).
  The SDK default covers reads on multi-region accounts; writes are NOT
  covered by the SDK default — write hedging requires explicit opt-in
  on a user-constructed `HedgingStrategy` (matches .NET exactly).
  PPCB alone does **not** auto-enable hedging.
- **SDK-default opt-out** at the runtime layer:
  `DriverOptions::disable_sdk_default_hedging(true)` and the
  `AZURE_COSMOS_HEDGING_DISABLE_SDK_DEFAULT` env var (§4.4) — mirrors
  Java v4's `IS_READ_AVAILABILITY_STRATEGY_ENABLED_WITH_PPAF` opt-out.
- Lifecycle handling: enable on account-properties refresh when PPAF
  appears, remove when PPAF goes away again. The SDK-default strategy
  itself does not change shape based on the multi-master flag.
- Environment variable support (§4.4) at the runtime level (priority 3
  in §11.3.1).
- **Continuation-token semantics for Query / ReadMany / ChangeFeed:**
  hedging operates per-page — each page request is an independent
  hedge fan-out, and the winning page's continuation token is forwarded
  to the next page. This matches the .NET behavior, where the
  `AvailabilityStrategy` operates at the `RequestInvokerHandler` level
  and each request (including paged ones) goes through its own handler
  pipeline.
- Unit + fault-injection tests per §15 covering: PPAF-on, PPCB-on
  without PPAF (must NOT activate), PPAF+PPCB activation matrix,
  single-master vs. multi-master write coverage, the §8.4 cross-cutting
  `ExcludeRegions` invariant, and the §6.5 invariant #6 app-cancel
  diagnostics preservation.

**§1 Goals closed at end of Phase 1:** G2, G3, G4, G5 in full; G1 for
the full document-scoped operation surface (parity with .NET v3 / Java v4).

**Out of scope this phase (deferred to Phase 2 / Future per §1 table):**
Metadata operations (Database / Container / Offer / Throughput), stored
procedures / triggers / UDFs, adaptive threshold tuning.

**Deliverables:**
- New files: `hedging.rs`, `hedging_diagnostics.rs` (see §8.5).
- Modified: `cosmos_driver.rs`, `operation_options.rs`, `mod.rs`,
  `diagnostics/mod.rs`.

### Phase 2: Metadata Hedging

**Operation rows from §1 covered (Phase 2 column):**
- Metadata operations: Database / Container / Offer / Throughput reads
  and updates

**Scope (deferred — design pass required before scheduling):**
- Extend `should_hedge()`'s phase-allowed `ResourceType` set to add
  `Database`, `Container`, `Offer`, `Throughput`.
- **Metadata cache invalidation:** hedged metadata reads must not
  produce stale-cache races when one region returns an older view than
  another; decide whether to prefer the latest `_etag` / resource id or
  the fastest response.
- **Diagnostics caveat for multi-phase operations** (already applies to
  Phase 1 Query / ReadMany / ChangeFeed and extends to Phase 2 metadata
  operations): these may contact regions *before* the hedge
  orchestrator starts — query plan fetches, partition-key-range cache
  loads, identity-batching pre-flights, and metadata-cache priming all
  hit the gateway/region in the normal pipeline.
  `HedgeDiagnostics::regions_contacted` covers only the regions the
  orchestrator itself fanned out to; pre-hedge contacts show up in the
  surrounding `DiagnosticsContext` (existing per-attempt region trail).
  Operators must consult both surfaces to tell hedge-driven contacts
  apart from setup-driven contacts — keeps **G2** intact under
  multi-stage operations.

**§1 Goals advanced:** G1 widens to metadata. G2 extended with
pre-hedge / hedge contact disambiguation.

### Future: Stored Procedures / Triggers / UDFs + Adaptive Thresholds

**Operation rows from §1 covered (Future column):**
- Stored procedures / triggers / UDFs execution — **🟡 candidate**
  pending a separate design proposal (server-side state mutation,
  idempotency, body cloning of script payloads interact with hedging
  in non-obvious ways).

**Out-of-spec extensions** (explicit §1 Non-Goals — included here only
to record where the work would land if priorities change):
- Latency histogram tracking per-region.
- Auto-tuning threshold based on observed p50 / p90 latency.
- Exponential backoff on hedge threshold after repeated hedges.

These three items intentionally do **not** advance any §1 Goal — they
are quality-of-life follow-ups whose addition would constitute a new
goal and require a spec amendment.

---

## 17. Open Questions

1. **Should hedging be enabled by default?** — **Resolved.** Off by default
   when PPAF is not enabled on the account. When the user opts into PPAF,
   the driver auto-enables the SDK-default hedging strategy (see §5.2).
   This matches .NET v3 exactly. Enabling PPCB alone does **not**
   auto-enable hedging — PPCB is failure-driven and does not by itself
   signal a desire for latency hedging. A runtime opt-out
   (`DriverOptions::disable_sdk_default_hedging` /
   `AZURE_COSMOS_HEDGING_DISABLE_SDK_DEFAULT`) is provided for users who
   want PPAF without the auto-default, mirroring Java v4's
   `IS_READ_AVAILABILITY_STRATEGY_ENABLED_WITH_PPAF` flag. Phase 1
   covers the full document-scoped surface (point reads/writes, Query,
   ReadMany, ChangeFeed) matching .NET v3 / Java v4; Phase 2 extends
   to metadata (see §16).

2. **Interaction with `EndToEndOperationLatencyPolicy`** — **Resolved.**
   All hedges share the same end-to-end deadline. The deadline is computed
   once at the start of `execute_with_hedging()` and passed to each pipeline
   invocation (see §9.4). This matches .NET v3 behavior: late hedges
   inherit the *remaining* deadline budget rather than getting their own
   full timeout, which prevents a slow primary from extending the
   user-visible latency past the configured cap.

3. **RU accounting** — **Resolved.** The caller-visible per-operation RU
   charge is the **winning** response's RU only. Aggregate hedge cost
   (winner + losers) is surfaced separately via the operator-facing
   `cosmos.hedge.ru_charge_total` metric (see §10.4). Rationale: the
   public `CosmosResponse.charge` contract should remain a stable proxy
   for "what this logical operation cost the user against their RU
   budget"; loser-hedge RU is speculative work the user did not request
   and would inflate per-operation accounting in a non-deterministic
   way. Operators who care about total cluster RU draw still get the
   total via the separate metric. This is an intentional divergence
   from .NET v3, which folds sub-request RU into `RequestCharge`.

4. **Race with background failback** — **Resolved.** PPCB transitions to
   `ProbeCandidate` and a concurrent hedge against the original region are
   independent pipeline invocations with independent retry states; the
   shared `LocationStateStore` uses CAS-based updates that are safe under
   concurrency (see §9.1). No additional coordination is required.

5. **Max concurrent hedges cap** — **Resolved.** No cap in Phase 1 (matches
   .NET SDK). The natural ceiling is `regions.len()`. A configurable cap
   (e.g., `HedgingStrategy::with_max_concurrent_hedges`) MAY be added in
   Phase 2 if operational data shows the unbounded fan-out is problematic
   on accounts with many regions; tracked as a follow-up rather than an
   open design question.

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
