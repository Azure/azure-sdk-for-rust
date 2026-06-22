# Cross-Region Hedging Availability Strategy Spec

**Status:** Draft
**Date:** 2026-05-14
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
13. [Write Hedging (Removed)](#13-write-hedging-removed)
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
is exceeded, and returns whichever **final** (non-transient) response arrives first.
This bounds tail latency
at roughly `threshold + cross-region-RTT` instead of waiting for the slow region to
respond.

### Goals

1. **Reduce tail latency** — p99/p99.9 point-read latencies bounded
   by a user-configured threshold plus cross-region RTT.
2. **Transparent to application** — the caller sees a single `CosmosResponse`; the
   hedging mechanism is invisible unless inspected via diagnostics.
3. **Configurable** — a single `threshold` knob is user-controlled at both
   client and per-operation levels; opt-out via `AvailabilityStrategy::Disabled`.
4. **Complementary to failover** — hedging handles *latency*; PPAF/PPCB handle
   *failures*. They compose without interference, and a repeated
   alternate-region win feeds back into PPCB to mark the primary partition
   degraded (see §9.5).
5. **Resource-safe** — at most one alternate-region request is in flight at
   any time (max two concurrent requests per logical operation), and the
   loser is cancelled cooperatively (best-effort — in-flight HTTP
   requests cannot be aborted mid-stream; see §14.2) to bound RU and
   transport waste over time.
6. **Zero-overhead happy path** — when the primary returns before the
   threshold elapses, `execute_hedged()` must not allocate the hedge
   task, the `FuturesUnordered`, or any per-hedge state (see §6.5).

### Non-Goals

- Hedging within a single region (e.g., across gateway nodes).
- Hedging writes of any kind. Write hedging on multi-master amplifies 409 /
  412 surface area and has near-zero adoption in the Java SDK; the Rust
  driver does not hedge writes in any phase. (If service-side idempotency
  keys land later, a separate proposal can revisit this.)
- Fanning out to more than one alternate region. The .NET / Java
  N-region progressive fan-out is dropped in favor of a single-alternate
  region model (max two concurrent requests).
- Automatic threshold tuning based on observed latency histograms (future work).
- Coupling hedging activation to PPAF. Hedging is independent of PPAF
  in this driver — see §5.2.

All Cosmos DB operation types are addressed by the phased rollout below.
Nothing is permanently excluded — stored procedure execution and
adaptive-tuning are deferred to the Future bucket pending a separate
design review.

### Operation-type scope (phased)

| Operation type | Phase 1 | Phase 2 | Future |
|---|:---:|:---:|:---:|
| Document point reads (GetItem) | ✅ | ✅ | ✅ |
| Queries (`QueryItems`) — page-level | ❌ | ✅ | ✅ |
| `ReadMany` — page-level | ❌ | ✅ | ✅ |
| Change feed — page-level | ❌ | ✅ | ✅ |
| Metadata operations (Database / Container / Offer / Throughput) | ❌ | ✅ | ✅ |
| Document writes (Create/Replace/Upsert/Delete/Patch) — any topology | ❌ | ❌ | ❌ |
| Stored procedure execution (`ExecuteJavaScript`) | ❌ | ❌ | 🟡 candidate |

> **Triggers and UDFs** are not standalone operations — they ride along
> as request headers on document operations and are therefore hedged
> automatically with the document op they decorate. Only stored
> procedure execution is a standalone server-side execution and
> deferred to Future.

Phase 1 ships document point reads only — the smallest correct surface
that exercises `execute_hedged()`, region pinning, cancellation, and the
PPCB feedback loop end-to-end. Phase 2 widens to feed-style operations
(Query / ReadMany / ChangeFeed), each hedged **per page**, plus
metadata operations. The exact integration with the `FeedRange`
abstraction is being co-designed with the feed-operation spec (see
§16). Writes are not in scope for any phase.

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

Hedging applies **only** to document-level operations:

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
([source](https://github.com/Azure/azure-cosmos-dotnet-v3/blob/0830090ca4dc47b71398ea0871cbe00b591aa8fc/docs/Cross%20Region%20Request%20Hedging.md)):

- **Threshold:** `min(1000ms, RequestTimeout / 2)` (falls back to `1000ms` if
  `RequestTimeout == 0`)
- **Threshold step:** `500ms`
- **Write hedging:** disabled

PPCB (Per-Partition Circuit Breaker) on its own does **not** auto-enable
hedging in .NET. However, .NET implicitly turns PPCB on whenever PPAF is
enabled
([CosmosClientOptions.cs `EnablePartitionLevelCircuitBreaker`](https://github.com/Azure/azure-cosmos-dotnet-v3/blob/71d317ffc6b6d62199b35c06a372825849e406fc/Microsoft.Azure.Cosmos/src/CosmosClientOptions.cs)),
so a PPAF-enabled deployment ends up running with all three (PPAF + PPCB +
hedging) active simultaneously.

**The Rust driver diverges from .NET here.** Hedging in this driver is
activated **independently of PPAF and PPCB** — whenever the account
has ≥ 2 applicable preferred regions and the user has not opted out,
the driver-default `HedgingStrategy` is used (§5.2). Rationale: the
Rust driver is greenfield and has no backward-compatibility constraint
that forced .NET v3 / Java v4 to gate hedging on PPAF. PPCB is fed by
hedging via `record_consecutive_hedge_win` (§9.5) but does not gate
the hedge decision. Opt-out is via `AvailabilityStrategy::Disabled`.
See §5.2 for the full activation rules.

### 2.7 Diagnostics

The .NET SDK attaches three diagnostic data points to the winning response:
- **Hedge Config** — threshold/step/write-enabled settings string.
- **Hedge Context** — list of regions that were contacted (up to and including the
  winning request's ordinal).
- **Response Region** — the target region name of the winning request.

---

## 3. Architectural Overview

### 3.1 Where Hedging Sits in the Driver

Hedging is an **in-pipeline decision** taken by
`evaluate_transport_result` (TPS §3.4) and dispatched by STAGE 7 of
the operation pipeline (TPS §4.1). When the evaluator decides a
hedged attempt is warranted, it returns
`OperationAction::Hedge { secondary_routing }`; STAGE 7 then calls
`execute_hedged()` (TPS §4.2), which races the primary attempt
against a single secondary attempt in `secondary_routing.region`.

```
CosmosDriver.execute_operation()
    │
    ▼
 execute_operation_pipeline()         [TPS §4.1, unchanged]
    │   STAGE 1 snapshot → STAGE 2 routing → STAGE 3 build request
    │
    ▼
 STAGE 4 transport attempt (primary)
    │
    ▼
 STAGE 5 evaluate_transport_result    [TPS §3.4]
    │   returns (OperationAction, Vec<LocationEffect>)
    │   └─ may produce OperationAction::Hedge { secondary_routing }
    │
    ▼
 STAGE 6 apply LocationEffects        [unchanged]
    │
    ▼
 STAGE 7 dispatch on OperationAction
    │   └─ OperationAction::Hedge → execute_hedged()        [TPS §4.2]
    │                                  │
    │                                  ▼
    │                            tokio::select!
    │                            primary_fut vs sleep(threshold)
    │                            then race primary_fut vs secondary_fut
    │                                  │
    │                                  ▼
    │                            first non-transient response wins;
    │                            loser is dropped (cancelled by Drop)
    │                                  │
    ◄─────────────────────────────────┘
    │
    ▼
 CosmosResponse + HedgeDiagnostics (attached on the winning response)
```

**Rationale.** Hedging composes with the existing per-attempt retry
layer rather than wrapping it. The `evaluate_transport_result` decision
function is the single place every routing/retry/hedge decision is
made, so hedging eligibility and secondary-region selection sit
exactly where they belong. There is no parallel orchestrator above the
pipeline and no `cosmos_driver.rs`-level wrapper.

### 3.2 Core Data Flow

```
                                STAGE 5
                  evaluate_transport_result(...)
                  returns OperationAction::Hedge {
                      secondary_routing: RoutingDecision {
                          region: regions[1],
                          excluded_regions: user ∪ (all_regions \ regions[1]),
                      }
                  }
                                  │
                                  ▼
                            STAGE 7
                       execute_hedged(
                           primary_routing,        // STAGE 2 result
                           secondary_routing,      // from the action
                           threshold,
                           …
                       )
                                  │
                  ┌───────────────────────────────┐
                  │   tokio::pin!(primary_fut);   │
                  │   tokio::select! biased; {    │
                  │     primary_fut → return,    │   ← zero-overhead
                  │     sleep(threshold) → hedge │     happy path
                  │   }                            │
                  └─────────────┬─────────────────┘
                                  ▼
                  ┌─────────────────────────────────┐
                  │  Build secondary transport     │
                  │  with ExecutionContext::        │
                  │  Hedging marker (§10).          │
                  │  Race primary vs secondary      │
                  │  via tokio::select!.            │
                  └─────────────┬───────────────────┘
                                  ▼
               First final result wins; the loser's future is
               dropped (its in-flight transport `Drop`s emit a
               cancellation signal at the next await point).
                                  │
                                  ▼
                  Return CosmosResponse + HedgeDiagnostics
```

### 3.3 Design Principles

1. **Single decision enum.** Hedging is dispatched via
   `OperationAction::Hedge { secondary_routing }` returned by
   `evaluate_transport_result`. There is no separate eligibility
   gate above the pipeline and no parallel orchestrator.
2. **Structural cancellation.** The secondary's future is owned by
   `tokio::select!` inside `execute_hedged()`; dropping it cancels
   the in-flight transport via the standard `Drop` chain. No
   `CancellationToken` is required in the hedge path.
3. **Immutable request cloning.** The `CosmosOperation` (which
   contains an `Arc`-backed body, headers, partition key) is cheap
   to clone. The secondary uses a different `RoutingDecision` (and
   therefore a different `ExcludeRegions` set) but shares the same
   `CosmosOperation`.
4. **Respect existing systems.** Hedging composes with PPAF / PPCB,
   session consistency, and throughput control because each attempt
   re-enters the per-attempt pipeline (TPS §4). A repeated
   alternate-region win is fed back into PPCB via
   `record_hedge_win()` (§9.5).

### 3.4 Reconciliation with `TRANSPORT_PIPELINE_SPEC.md` §4.2 — **Resolved**

This spec adopts the TPS §4.2 in-pipeline shape verbatim:
hedging is selected by `evaluate_transport_result` returning
`OperationAction::Hedge { secondary_routing }` (TPS §3.4) and is
executed by `execute_hedged()` from the STAGE 7 dispatch (TPS §4.1).

The pseudocode in §6 is the **normative semantics** of the
`OperationAction::Hedge` arm: function signature, race shape,
zero-overhead happy path, and the diagnostics / PPCB-feedback
callsites are all load-bearing on the TPS-side implementation.

**Invariants the merged TPS-side implementation MUST preserve:**

- At most one alternate-region request in flight at any time
  (§6.5 #1).
- `ExcludeRegions` is the region-pinning mechanism for the secondary
  (§6.3, §8.4); the evaluator computes the set when it produces
  `secondary_routing`.
- Zero-overhead happy path when the primary wins before the
  threshold timer fires — no `Arc<>`, no clones, no
  `CancellationToken` in this branch (§6.5 #3).
- Hedging-win feedback into PPCB via `record_hedge_win()` (§9.5)
  is invoked by `execute_hedged()` immediately on a secondary win.
- App-cancellation re-raises with hedge diagnostics preserved
  (§6.5 #7, §14.2).

**Cross-spec follow-ups outside this spec:**

- TPS §4.2 currently states *"For write operations, hedging is only
  enabled on multi-write-region (MWR) accounts"*. This spec drops
  write hedging entirely (§1 Non-Goals, per Fabian F5). The TPS
  §4.2 wording must be updated to match — tracked as an open
  follow-up against the TPS owner.
- TPS §4.2 specifies a **dynamic P99-based threshold clamped to
  50–4000 ms**; this spec specifies a **static
  `min(1000ms, request_timeout / 2)` driver default** (§5.2). The
  threshold-policy disagreement is tracked as a new open question
  (§17 Q11) pending cross-team alignment.

---

## 4. Configuration Surface

### 4.1 HedgingStrategy Type

```rust
/// A validated, non-zero hedging threshold.
///
/// Newtype around `Duration` whose only constructor rejects zero —
/// trades a fallible-construct ceremony per call site for a single
/// `Option`-returning constructor that compiles away when the input is
/// known at compile time.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HedgeThreshold(Duration);

impl HedgeThreshold {
    /// Returns `None` if `duration` is zero.
    pub const fn new(duration: Duration) -> Option<Self> {
        if duration.is_zero() { None } else { Some(Self(duration)) }
    }

    pub const fn get(self) -> Duration { self.0 }
}

/// Cross-region hedging availability strategy.
///
/// When the primary request does not complete within `threshold`, the driver
/// sends a single speculative request to the next preferred region.
/// The first non-transient response wins; the loser is cancelled.
///
/// At most one alternate-region request is in flight at any time — the
/// driver does not fan out to a third region.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct HedgingStrategy {
    threshold: HedgeThreshold,
}

impl HedgingStrategy {
    /// Creates a new hedging strategy with the given threshold.
    pub const fn new(threshold: HedgeThreshold) -> Self {
        Self { threshold }
    }

    /// Returns the threshold before the alternate-region hedge fires.
    pub const fn threshold(self) -> HedgeThreshold { self.threshold }
}
```

There is no separate "write hedging" flag, no `threshold_step`, no
fallible constructor, and no SDK-default factory. The single knob is
`threshold`; everything else (eligibility, region selection, cancellation)
is driven by spec rules rather than configuration.

> **Divergence from .NET and Java.**
>
> - **No N-region fan-out / no `threshold_step`.** .NET's
>   `thresholdStep` and Java's `DEFAULT_THRESHOLD_STEP = 100ms` exist
>   only to schedule the second, third, …, Nth hedge. The Rust driver
>   caps fan-out at one alternate region (max two concurrent
>   requests), so a step is unnecessary.
> - **No write hedging knob.** .NET's `enableMultiWriteRegionHedge`
>   and Java's implicit write-hedging path are intentionally dropped
>   (see §1 Non-Goals).
> - **No fallible constructor on `HedgingStrategy`.** Validation moves
>   into the `HedgeThreshold` newtype (`Option<Self>` on zero), so the
>   only error surface is at the boundary where the duration is
>   constructed.
> - **No `HedgingStrategy::default()` / no-arg constructor.** Users
>   always pick a threshold explicitly. The driver-wide default-on
>   activation (§5.2) uses an internal default that is not part of the
>   public API.

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

> **Interaction with default-on activation (§5.2).** Because hedging
> is on by default for accounts that satisfy §5.1, setting
> `AvailabilityStrategy::Disabled` at the **client** level is the
> code-level kill switch: it suppresses the §5.2 driver default for
> every operation on that client. Setting `Disabled` on a single
> operation suppresses only that operation; sibling operations continue
> to use the client-level strategy or the §5.2 default. The full
> precedence chain is in §11.3.1.

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

The driver exposes a single boolean master switch for hedging via the
`AZURE_COSMOS_HEDGING_ENABLED` environment variable (mapped to
`OperationOptions::hedging_enabled`). It is **enabled by default**: leaving
the variable unset preserves the code-driven behavior described in §11. When
set it becomes the source of truth and takes precedence over the programmatic
`AvailabilityStrategy` in **both** directions — `false` disables hedging even
when an explicit `AvailabilityStrategy::Hedging(..)` is configured, and `true`
enables hedging even when an explicit `AvailabilityStrategy::Disabled` is
configured (a programmatic `Hedging(..)` still supplies its custom threshold).

A second `AZURE_COSMOS_HEDGING_ENABLED_OVERRIDE` variant acts as a
process-wide incident kill switch that wins over **every** configuration
layer, including a per-request value. Both variables are read once at
runtime-build time (not per request), so flipping one mid-incident requires a
process restart, and both are inert unless set. Boolean values are parsed
leniently — `true`/`false`, `1`/`0`, `yes`/`no`, `on`/`off`, case-insensitive
— so a kill switch does not silently fail open on a common spelling.

There is still no env var for the hedge threshold or write hedging because
those features do not exist (see §4.1 divergence note).

---

## 5. Eligibility Rules

### 5.1 `should_hedge()` — Pure Function

`should_hedge()` is a pure helper consulted by
`evaluate_transport_result` (TPS §3.4) when deciding whether to
return `OperationAction::Hedge`. It is called once per per-attempt
pipeline iteration; there is no orchestrator above the pipeline that
calls it separately.

```rust
/// Determines whether the given operation should use hedging.
///
/// Returns `false` if:
/// - No hedging strategy is resolved (or explicitly `Disabled`)
/// - Application preferred-region list empty or has < 2 entries after
///   `ExcludeRegions` filtering
/// - Operation is not in the phase-allowed ResourceType set
/// - Operation is a write (writes are never hedged — see §1 Non-Goals)
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
| 2 | Application preferred-region list empty | No |
| 3 | `ResourceType` not in the **phase-allowed set** † | No |
| 4 | Operation is a write (any topology) | No |
| 5 | Applicable `preferred_read_endpoints` (after `ExcludeRegions`) has < 2 entries | No |
| 6 | Read with ≥ 2 applicable read endpoints | **Yes** |

The "≥ 2 applicable endpoints" check is computed against the
post-`ExcludeRegions` list, not the raw account region count — a user
who excludes all-but-one region at the operation level will (correctly)
skip hedging even on a multi-region account.

> **† Phase-allowed `ResourceType` set.** Row 3's allowed set evolves
> with the implementation phases in §16:
>
> | Phase | Allowed `ResourceType` set |
> |---|---|
> | 1 (MVP)  | `{Document}` for point reads only — enforced by an additional `OperationType` guard inside the predicate (reads only, no writes). |
> | 2        | Phase 1 set ∪ feed-style operations (Query / ReadMany / ChangeFeed — still `ResourceType.Document` but `OperationType` differs) ∪ `{Database, Container, Offer, Throughput}` (metadata reads). |
> | Future   | Phase 2 set ∪ `{StoredProcedure}` (sprocs only — triggers / UDFs are not standalone operations). |
>
> Phase 1 implementations should hard-code the allowed `OperationType`
> set to `{Read}`. Each subsequent phase widens the constants in one
> place; no other change to §5.1 is required.

### 5.2 Default activation

Hedging is **on by default** for accounts that satisfy the §5.1
eligibility rules — it is independent of PPAF and PPCB. Rationale: the
Rust driver is greenfield, so we do not need the .NET / Java
PPAF-coupled opt-in to preserve backward compatibility. Tail-latency
protection is a generally useful default; users who do not want it can
opt out at any layer via `AvailabilityStrategy::Disabled` (§4.2).

**Driver default values** (used when no user strategy is configured):

- **Threshold:** `min(1000ms, request_timeout / 2)`, with `1000ms` as
  the fallback when `request_timeout` is unset or zero. Matches the
  .NET v3 default-threshold formula; chosen because it is conservative
  on accounts with sub-second p50 latency and degrades gracefully
  under high configured request timeouts.

**Activation preconditions** — even with hedging on by default, the
`OperationAction::Hedge` is not produced at runtime for operations that fail any of:

1. **At least two applicable preferred regions.** Single-region
   accounts and operations whose `ExcludeRegions` filter leaves only
   one applicable region skip hedging.
2. **Application-level region configuration is required.** The driver
   must have a non-empty preferred-region list (set via
   `DriverOptions::preferred_regions` or
   `OperationOptions::application_preferred_regions`) so the
   evaluator has a deterministic alternate-region target. Without
   it, hedging is silently skipped.

Both checks are enforced inside `should_hedge()`; failure of either
short-circuits the hedge decision before the primary request is sent.

> **Cross-SDK comparison.** .NET v3 couples hedging activation to
> PPAF (auto-enable only when PPAF is on); Java v4 does the same and
> additionally exposes
> `COSMOS.IS_READ_AVAILABILITY_STRATEGY_ENABLED_WITH_PPAF=false` as a
> runtime opt-out. Both coupling decisions exist because PPAF in
> those SDKs is opt-in and signals an "availability-oriented"
> deployment. The Rust driver is greenfield and treats hedging as a
> stand-alone tail-latency tool, so this coupling is dropped —
> hedging is on by default and PPAF / PPCB activation is unrelated.
> Java's threshold (`500ms`) and .NET's (`min(1000ms, RequestTimeout/2)`)
> bracket the Rust default; users wanting tighter latency bounds should configure
> `500ms` explicitly.

---

## 6. Hedging Algorithm

> **Reading guide.** This section is the **normative semantics** of
> the `OperationAction::Hedge` arm dispatched by STAGE 7 of the
> operation pipeline (TPS §4.1). The signatures below match the TPS
> shape so that anyone implementing TPS §4.2 can lift the body
> directly. The race shape, zero-overhead happy path, app-cancel
> harvesting, and PPCB-feedback callsite are load-bearing.

### 6.1 Overview

Hedging is dispatched in two places:

1. **`evaluate_transport_result` (TPS §3.4)** decides whether the
   per-attempt result warrants a hedge. When `should_hedge()` (§5.1)
   says yes and the primary has not yet produced a final result, the
   evaluator returns
   `OperationAction::Hedge { secondary_routing: RoutingDecision }`.
   The secondary `RoutingDecision` is computed by the evaluator
   (§6.2 / §6.3) and carries the secondary region plus the
   `ExcludeRegions` set that pins the hedge to that region.
2. **`execute_hedged()` (called from STAGE 7)** consumes both
   `RoutingDecision`s, races the primary attempt against a single
   secondary attempt, and returns the first non-transient response.

Both functions live in `operation_pipeline.rs`; there is no separate
orchestrator and no `cosmos_driver.rs`-level wrapper.

```rust
// Selected by TPS §3.4.
OperationAction::Hedge {
    secondary_routing: RoutingDecision,
}

// Dispatched by TPS §4.1 STAGE 7.
async fn execute_hedged(
    operation: &CosmosOperation,
    options: &OperationOptions,
    primary_routing: &RoutingDecision,
    secondary_routing: &RoutingDecision,
    threshold: HedgeThreshold,
    session: &SessionState,
    transport: &AdaptiveTransport,
    credential: &Credential,
    diagnostics: &mut DiagnosticsContextBuilder,
    deadline: Option<Instant>,
) -> azure_core::Result<CosmosResponse>;
```

`execute_hedged()` fires **at most two** concurrent transport
attempts: the primary at `t=0`, and a single secondary attempt at
`t=threshold` if the primary has not yet produced a final result.
There is no third hedge, no `threshold_step`, and no N-region
fan-out.

### 6.2 Region Selection (computed by `evaluate_transport_result`)

The primary uses normal routing — STAGE 2's `resolve_endpoint()`
result. The secondary is `applicable_read_endpoints[1]` after
`ExcludeRegions` filtering — i.e. the second region in the user's
preferred-region list that is not currently excluded. If no such
region exists, the evaluator does **not** return
`OperationAction::Hedge` (§5.1 row 5); the operation pipeline takes
some other action (`Complete`, `FailoverRetry`, `SessionRetry`, or
`Abort`).

```
regions = applicable_read_endpoints(excluded_regions)
// regions[0] = primary    (normal routing — primary_routing.region)
// regions[1] = secondary  (the hedge target — secondary_routing.region)
// regions[2..] = unused by hedging in this driver
```

### 6.3 Request Routing via ExcludeRegions (also evaluator-side)

The secondary is pinned to its target region by setting
`secondary_routing.excluded_regions = user ∪ (all_regions \ regions[1])`.
This is computed by the evaluator when it builds the
`secondary_routing: RoutingDecision`; `execute_hedged()` itself does
no routing math.

| Request | ExcludeRegions | Target |
|---|---|---|
| Primary | (the user's original exclusion set, if any) | regions[0] (normal routing) |
| Secondary | user-original ∪ `(all_regions \ regions[1])` | regions[1] |

This piggybacks on the existing `ExcludeRegions` mechanism in
`resolve_endpoint()` (TPS §4.1 STAGE 2), requiring no changes to the
endpoint resolution logic, and composes with any user-specified
`ExcludeRegions` (the secondary's exclusion set is the *union* of the
user's and the per-hedge pin).

### 6.4 Execution Flow (Pseudocode)

`execute_hedged()` races the primary attempt against a single
threshold timer. If the timer wins, it builds the secondary transport
request from `secondary_routing` and races primary vs secondary via
`tokio::select!`. There is no `CancellationToken`: the loser's future
is dropped, which `Drop`s the in-flight transport (TPS §5.1) and emits
the cancellation signal at the next transport await point.

```rust
async fn execute_hedged(
    operation: &CosmosOperation,
    options: &OperationOptions,
    primary_routing: &RoutingDecision,
    secondary_routing: &RoutingDecision,
    threshold: HedgeThreshold,
    session: &SessionState,
    transport: &AdaptiveTransport,
    credential: &Credential,
    diagnostics: &mut DiagnosticsContextBuilder,
    deadline: Option<Instant>,
) -> Result<CosmosResponse> {
    // ── Build the primary transport request. The secondary is NOT built
    //    yet — every allocation below is gated on the threshold timer
    //    firing (§6.5 invariant #3). ──
    let primary_req = build_transport_request(
        operation, primary_routing, session, options, deadline,
    );
    let primary_fut = execute_transport_pipeline(
        primary_req, transport, credential, diagnostics,
    );
    tokio::pin!(primary_fut);

    // ── Happy path: just await the primary against the threshold timer. ──
    tokio::select! {
        biased;

        // Primary returned before the threshold. Attach a "no hedge fired"
        // HedgeDiagnostics and return — zero-overhead happy path (§6.5 #3).
        result = &mut primary_fut => {
            diagnostics.record_attempt(&result);
            return evaluate_and_return(
                operation, result, diagnostics,
                HedgeDiagnostics::primary_only(threshold, &primary_routing.region),
            );
        }

        // Threshold elapsed → fall through to the hedged race below.
        _ = sleep(threshold.get()) => {}
    }

    // ── Spawn the secondary. From here on, both futures are pinned to
    //    this stack frame. The shared hub-region latch (§9.6) is
    //    constructed here too, after the threshold fires. ──
    let shared_hub_region_latch = if hub_region_latch_eligible(operation, options) {
        Some(Arc::new(AtomicBool::new(false)))
    } else {
        None
    };

    let secondary_req = build_transport_request(
        operation, secondary_routing, session, options, deadline,
    )
    .with_execution_context(ExecutionContext::Hedging)
    .with_shared_hub_region_latch(shared_hub_region_latch.clone());

    let secondary_fut = execute_transport_pipeline(
        secondary_req, transport, credential, diagnostics,
    );
    tokio::pin!(secondary_fut);

    // ── Race primary vs secondary. First final result wins; the loser's
    //    future is dropped (Drop chain cancels the in-flight transport).
    //    A transient result on either side keeps the *other* side racing.
    //    Application cancellation is observed by the surrounding
    //    `select!` arms via the deadline — no CancellationToken tree. ──
    let mut last_transient: Option<(Side, azure_core::Error)> = None;
    let mut primary_done = false;
    let mut secondary_done = false;

    while !(primary_done && secondary_done) {
        tokio::select! {
            biased;

            // App-cancel observed via deadline → harvest the most-advanced
            // pipeline within HARVEST_WINDOW for diagnostics, then re-raise.
            _ = app_cancel_signal(deadline) => {
                let err = harvest_app_cancel_error(
                    &mut primary_fut, primary_done,
                    &mut secondary_fut, secondary_done,
                    threshold, primary_routing, secondary_routing,
                ).await;
                return Err(err);
            }

            r = &mut primary_fut, if !primary_done => {
                primary_done = true;
                diagnostics.record_primary_attempt(&r);
                match classify(r) {
                    Outcome::Final(resp) => {
                        return Ok(decorate(
                            resp, threshold,
                            primary_routing, secondary_routing,
                            Side::Primary,
                        ));
                    }
                    Outcome::Transient(err) => {
                        last_transient = Some((Side::Primary, err));
                    }
                }
            }

            r = &mut secondary_fut, if !secondary_done => {
                secondary_done = true;
                diagnostics.record_hedged_attempt(&r);
                match classify(r) {
                    Outcome::Final(resp) => {
                        // Repeated secondary wins feed back into PPCB — see §9.5.
                        record_hedge_win(&primary_routing.partition_key_range_id,
                                         &primary_routing.region);
                        return Ok(decorate(
                            resp, threshold,
                            primary_routing, secondary_routing,
                            Side::Secondary,
                        ));
                    }
                    Outcome::Transient(err) => {
                        last_transient = Some((Side::Secondary, err));
                    }
                }
            }
        }
    }

    // ── Both sides terminated transient — surface the most recent error. ──
    Err(last_transient.map(|(_, e)| e).unwrap_or_else(|| {
        azure_core::Error::message(
            azure_core::error::ErrorKind::Other,
            "hedging completed without producing a response",
        )
    }))
}
```

#### 6.4.1 Helper sketches

```rust
enum Side { Primary, Secondary }
enum Outcome { Final(CosmosResponse), Transient(azure_core::Error) }

fn classify(r: Result<CosmosResponse, azure_core::Error>) -> Outcome {
    match r {
        Ok(resp) if is_final_result(resp.status()) => Outcome::Final(resp),
        Ok(resp) => Outcome::Transient(transient_from_response(resp)),
        Err(err) => Outcome::Transient(err),
    }
}

/// Build the secondary `RoutingDecision` inside `evaluate_transport_result`.
/// This is what populates `OperationAction::Hedge { secondary_routing }`;
/// `execute_hedged()` does NOT compute routing.
///
/// **Region+endpoint distinctness contract (Phase 1).** The chosen
/// secondary MUST differ from the resolved primary on BOTH
/// `region` AND `endpoint_key`. Single-field distinctness is
/// insufficient: a regional endpoint and its global-account alias may
/// resolve to the same physical replica even when their `Region`
/// strings differ, and two different region strings can also alias to
/// the same TLS endpoint after applicable-region resolution. If no
/// applicable endpoint satisfies BOTH conditions, this function returns
/// `None` and the caller falls back to its non-hedged decision (i.e.
/// `OperationAction::Hedge` is never produced when no distinct
/// alternate exists). See `hedging_eligibility.rs` for the post-fix
/// implementation introduced in commit `afb7baeb`.
fn build_secondary_routing(
    primary: &RoutingDecision,
    user_excluded: &[Region],
    regions: &[Region],
) -> Option<RoutingDecision> {
    let mut excluded: Vec<Region> = user_excluded.to_vec();
    for (i, r) in regions.iter().enumerate() {
        if i != /* secondary index */ 1 && !excluded.contains(r) {
            excluded.push(r.clone());
        }
    }
    // Caller has already verified `regions[1]` differs from
    // `primary.region` AND its endpoint_key differs from
    // `primary.endpoint_key`; otherwise it returns None upstream.
    Some(RoutingDecision {
        region: regions[1].clone(),
        excluded_regions: excluded,
        partition_key_range_id: primary.partition_key_range_id.clone(),
        // ... other RoutingDecision fields inherited from primary ...
    })
}

fn decorate(
    mut resp: CosmosResponse,
    threshold: HedgeThreshold,
    primary: &RoutingDecision,
    secondary: &RoutingDecision,
    winner: Side,
) -> CosmosResponse {
    let (primary_region, alternate_region) = match winner {
        Side::Primary   => (primary.region.clone(), None),
        Side::Secondary => (primary.region.clone(), Some(secondary.region.clone())),
    };
    let response_region = match winner {
        Side::Primary   => primary.region.clone(),
        Side::Secondary => secondary.region.clone(),
    };
    resp.attach_hedge_diagnostics(HedgeDiagnostics {
        strategy_config: HedgingStrategyConfig { threshold },
        primary_region,
        alternate_region,
        response_region,
        was_hedge: matches!(winner, Side::Secondary),
    });
    resp
}
```

#### 6.4.2 Ownership & Sharing

`execute_hedged()` avoids `tokio::spawn` and `FuturesUnordered`
entirely in both the happy path and the hedged race — both futures
are pinned on the stack and polled by `tokio::select!`. This keeps
the allocator out of the hot path and removes the `'static + Send`
constraint that a `JoinHandle` model would impose.

**Cancellation is structural, not signalled.** When the primary wins,
the secondary's pinned future is dropped — its in-flight transport
`Drop`s and emits the cancellation signal at the next transport
`await` point (TPS §5.1). There is no `CancellationToken` in the
hedge path. The same property holds for the surrounding code: if the
caller drops `execute_hedged()`'s future (e.g. via
`tokio::time::timeout` one layer up), both the primary and secondary
futures are dropped together by the standard structured-concurrency
chain.

### 6.5 Key Invariants

1. **Max two concurrent transport attempts.** Primary + at most one
   secondary. The driver does not fan out to a third region under any
   circumstance.
2. **Primary fires immediately.** Zero additional latency on the
   happy path — `execute_hedged()`'s entry is a single
   `tokio::select!` with two arms (primary future, threshold timer).
3. **Zero-overhead happy path.** If the primary returns a final
   result before the threshold timer elapses, `execute_hedged()`
   MUST NOT:
   - Build the secondary `transport_request`.
   - Construct the `Arc<AtomicBool>` shared hub-region latch (§9.6).
   - Allocate any `Vec` per-hedge.

   This is a load-bearing performance constraint and is gated by the
   `hedging_zero_overhead_happy_path_no_allocs` benchmark in §15.
4. **Region pinning is hard.** The secondary's per-attempt retry
   layer honors its `ExcludeRegions` set for every retry trigger
   class — it cannot fall back to the primary's region (§8.4).
5. **Cancellation is structural.** Dropping the loser's future via
   `tokio::select!` cancels its in-flight transport via the standard
   `Drop` chain. No `CancellationToken` is required in the hedge
   path; the per-attempt deadline check inside the transport
   pipeline (TPS §5.1) handles deadline-based cancellation.
6. **Single writer to diagnostics.** Only the winning response gets
   `HedgeDiagnostics` attached; when the primary wins before the
   threshold elapses, a synthetic "primary-only" diagnostics record
   is used so consumers can tell *"hedging was selected but never
   fanned out"* apart from *"hedging was not selected"*. The on-wire
   marker for a hedged secondary attempt is
   `ExecutionContext::Hedging` (TPS §3.4) on the secondary's
   `transport_request`.
7. **App-cancel preserves hedge trace.** When the application's
   cancellation (observed via the deadline) fires while both
   attempts are racing, `execute_hedged()` harvests the in-flight
   futures within a bounded `HARVEST_WINDOW = 50ms` and attaches the
   most-advanced result's diagnostics to the returned
   `application_cancelled_error()`. Mirrors .NET v3's behavior of
   awaiting the most-advanced task before re-raising.
8. **Secondary wins feed back into PPCB.** A win by the secondary is
   an out-of-band signal that the primary partition is degraded —
   `execute_hedged()` records it via `record_hedge_win()` (§9.5) so
   PPCB can mark the partition `Unhealthy` after the configured
   number of consecutive secondary wins.
9. **Single decision enum, two entry points in the pipeline.** Hedging
   is selected either by a first-attempt eligibility check (STAGE 2b
   in `execute_operation_pipeline`, before any transport request has
   gone out — uses `evaluate_hedge_eligibility` directly) or by
   `evaluate_transport_result` returning
   `OperationAction::Hedge { secondary_routing }` on a post-attempt
   upgrade (STAGE 5b → STAGE 7). There is no parallel orchestrator and
   no separate cancellation tree above the pipeline. Both entry points
   call into the same `execute_hedged()` body; the two pipeline call
   sites differ only in *when* the eligibility check fires
   (pre-request vs. post-classification) and are functionally
   equivalent. Operator-visible behavior — the hedge race itself, PPCB
   feedback, and diagnostics attachment — is identical.
10. **Hedge legs apply LocationEffects symmetrically to the non-hedged
    path.** Each leg's `TransportResult` is passed through a
    non-consuming sibling of `evaluate_transport_result` —
    `retry_evaluation::evaluate_hedge_leg_effects` — and the resulting
    `Vec<LocationEffect>` is applied via
    `location_state_store.apply(&effects).await` at each of the four
    `classify_hedge_result` sites before `execute_hedged()` returns.
    This restores parity with the non-hedged path: a 503 observed on
    either leg marks the responsible (partition, endpoint) pair
    unavailable, a 403/`WriteForbidden` triggers
    `RefreshAccountProperties`, etc. Without this step, a hedge that
    races against the non-hedged path could leak transient cluster
    state by failing to mark a known-bad endpoint. The evaluator is
    side-effect-free w.r.t. `OperationRetryState` — it only emits
    effects and returns a separate `observed_session_unavailable`
    signal consumed by §9.6's cross-hedge latch propagation.
11. **Cross-hedge 1002 propagation is symmetric.** When either hedge
    leg observes the four trigger conditions of §9.6.1, the
    `BothTransient` finalizer surfaces an `observed_session_unavailable: bool`
    flag back to the pipeline. The pipeline's STAGE 2b / STAGE 7
    `BothTransient` handlers call `propagate_hedge_session_unavailable`,
    which idempotently flips the parent `OperationRetryState`'s
    `hub_region_processing_only` field AND `Release`-stores the same
    bit on the shared `Arc<AtomicBool>` (§9.6.2). Without this fan-in,
    the parent operation would re-issue its first post-hedge retry
    without the hub-region header, paying the discovery cost a second
    time even though one of the legs already discovered it. See §14.1
    for the full both-transient state-mutation contract.

---

## 7. Final Result Classification

### 7.1 `is_final_result()` — Pure Function

```rust
/// Determines whether a response status code is a final (non-transient) result.
///
/// Final results terminate hedging immediately. Transient results allow other
/// in-flight hedges to continue racing for a better outcome.
///
/// Note: 403 (with or without sub-status `3` WriteForbidden) is **final** for
/// hedging — it is an authorization/ownership decision that another region
/// cannot change in parallel. When a 403 (e.g. `WriteForbidden`) needs to be
/// retried against another region, the dedicated retry pipeline (PPAF) handles
/// it through the normal retry loop rather than via a parallel hedge race.
///
/// Note: a 429 is normally transient, but the account-/partition-wide
/// throttles `3200` (RU_BUDGET_EXCEEDED), `3210`
/// (RU_BUDGET_EXCEEDED_FOR_MASTER), and `3214` (HOT_PARTITION_KEY_THROTTLED)
/// are **final** — racing a second region cannot relieve them (§7.2.1).
///
/// Note: protocol-/policy-level errors (`422`, `451`, `501`, `505`) are
/// **final** because no alternate region can resolve them.
fn is_final_result(status: &CosmosStatus) -> bool {
    let code = status.http_status_code;
    let sub = status.sub_status_code;

    // All 1xx, 2xx, 3xx → final
    if code < 400 {
        return true;
    }

    // Specific client / server errors that are definitively non-transient
    matches!(code,
        400  // Bad Request
        | 401  // Unauthorized
        | 403  // Forbidden / WriteForbidden — authorization decision
        | 405  // Method Not Allowed
        | 409  // Conflict
        | 412  // Precondition Failed
        | 413  // Request Entity Too Large
        | 422  // Unprocessable Entity
        | 451  // Unavailable For Legal Reasons
        | 501  // Not Implemented
        | 505  // HTTP Version Not Supported
    ) || (code == 404 && sub == 0)  // Not Found with no sub-status
        || (code == 429 && matches!(sub,
            3200  // RU_BUDGET_EXCEEDED
            | 3210  // RU_BUDGET_EXCEEDED_FOR_MASTER
            | 3214  // HOT_PARTITION_KEY_THROTTLED
        ))
}
```

### 7.2 Transient vs. Non-Transient Responses

| Status | Sub-Status | Transient? | Rationale |
|--------|------------|------------|-----------|
| 200 | * | No (final) | Success |
| 304 | * | No (final) | Not Modified |
| 400 | * | No (final) | Client error — won't succeed in another region |
| 401 | * | No (final) | Auth failure — same credentials everywhere |
| 403 | * | No (final) | Forbidden / authorization decision (RBAC, WriteForbidden, account ownership) — another region cannot change the answer; retriable sub-statuses go through the dedicated retry pipeline (e.g. PPAF) instead of a parallel hedge race |
| 404 | 0 | No (final) | Resource genuinely not found |
| 404 | 1002 | **Yes** | ReadSessionNotAvailable — session lag |
| 405 | * | No (final) | Wrong HTTP method |
| 408 | * | **Yes** | Timeout — another region may be faster |
| 409 | * | No (final) | Conflict — deterministic |
| 410 | * | **Yes** | Gone — partition may have moved |
| 412 | * | No (final) | Precondition — deterministic |
| 413 | * | No (final) | Payload too large — same everywhere |
| 422 | * | No (final) | Unprocessable Entity — payload-level error; same in every region |
| 429 | 3092 (or none) | **Yes** | Transient capacity pressure — another region may have capacity |
| 429 | 3200 / 3210 / 3214 | No (final) | RU-budget / hot-partition throttle — account-/partition-wide; racing another region only doubles the load (§7.2.1) |
| 451 | * | No (final) | Unavailable For Legal Reasons — policy-level; not regional |
| 500 | * | **Yes** | Internal error — may be region-specific |
| 501 | * | No (final) | Not Implemented — protocol-level; no alternate region can serve |
| 503 | * | **Yes** | Unavailable — another region may be healthy |
| 505 | * | No (final) | HTTP Version Not Supported — protocol-level; no alternate region can serve |

> **Note on 403 sub-statuses.** The driver classifies any 403 (with or
> without `WriteForbidden` sub-status `3`) as **final** for hedging
> purposes — a 403 is an authorization/ownership decision (RBAC,
> account ownership, write-region restriction) that another region
> cannot change in parallel. When a 403 *can* be retried — e.g.
> `WriteForbidden` on a single-master account — the dedicated retry
> path (PPAF write retry) handles the redirect through the normal
> retry loop rather than via a parallel hedge race. Racing the same
> request against another region would, at best, duplicate the denial
> and, at worst, double a security-sensitive signal that may itself
> be rate-limited.

### 7.2.1 429 sub-status sensitivity (hedge-spawn eligibility)

The §7.2 table classifies `429 | * | transient` for the purpose of the
*in-race* outcome decision (`classify_hedge_result`) — i.e. once two legs
are already running, a 429 from one leg does not terminate the race. That
blanket rule is **not** the same as the decision to *spawn* a hedge in the
first place, which is deliberately narrower and sub-status-aware.

A cross-region hedge is only ever started when the primary attempt
produces a region-changing retry action (`FailoverRetry` / `SessionRetry`)
that `maybe_upgrade_to_hedge` then upgrades (§8). For a 429 this happens
**only** for sub-status `3092` (`SystemResourceUnavailable`), which
`try_handle_retry_trigger_group` admits to the retry-trigger group. Every
other 429 sub-status falls through `evaluate_http_outcome` to `Abort` and
is therefore **never** upgraded into a cross-region hedge:

| 429 sub-status | Constant | Hedge-spawn eligible? | Rationale |
|----------------|----------|-----------------------|-----------|
| 3092 | `SYSTEM_RESOURCE_UNAVAILABLE` | **Yes** | Transient backend capacity pressure (e.g. high CPU / transient hop saturation); another replica/region can serve immediately. |
| 3200 | `RU_BUDGET_EXCEEDED` | No (Abort) | Provisioned throughput exhausted. Failing over to another region cannot conjure RU/s and only spreads the throttle; the correct response is in-region backoff, not hedging. |
| 3210 | `RU_BUDGET_EXCEEDED_FOR_MASTER` | No (Abort) | Control-plane / master-partition RU exhaustion — same reasoning as 3200. |
| 3214 | `HOT_PARTITION_KEY_THROTTLED` | No (Abort) | A hot logical partition is throttled identically in every region (the same partition key maps to the same logical partition everywhere), so hedging cannot route around it and merely doubles the load on an already-saturated partition. |
| other | — | No (Abort) | Conservative default: only sub-statuses with a known cross-region benefit may spawn a hedge. |

This means the "examine the 429 sub-status before hedging" guard already
exists implicitly via the retry-trigger group — there is no separate
hedging-specific check to add. The regression test
`throttle_substatus_gates_hedge_eligibility` in `retry_evaluation.rs`
pins this behavior so a future widening of the trigger group cannot
silently begin hedging RU-exhaustion or hot-partition throttles.

> **In-race finality.** The *in-race* classifier (`is_final_result`, the
> backing predicate for `classify_hedge_result` / `result_is_final`) is
> also sub-status-aware for 429: `3200` (`RU_BUDGET_EXCEEDED`), `3210`
> (`RU_BUDGET_EXCEEDED_FOR_MASTER`), and `3214`
> (`HOT_PARTITION_KEY_THROTTLED`) are treated as **final**, so when a
> proactively-hedged leg lands on one of these the race short-circuits
> instead of spending the partner region's RU on a throttle it cannot
> relieve. A generic 429 and the transient-capacity `3092`
> (`SYSTEM_RESOURCE_UNAVAILABLE`) remain transient. The lockstep test
> `result_is_final_agrees_with_classify_hedge_result` pins these two
> predicates together.

---

## 8. Operation Pipeline Integration

### 8.1 Entry Point Changes

**`cosmos_driver.rs::execute_operation()` does not change.** Hedging
is selected entirely inside the per-attempt pipeline by
`evaluate_transport_result` (TPS §3.4) returning
`OperationAction::Hedge { secondary_routing }`, and dispatched by
STAGE 7 of `execute_operation_pipeline()` (TPS §4.1) calling
`execute_hedged()` (§6.1, TPS §4.2).

The only changes outside the transport pipeline are:

- `OperationOptions` gains an `availability_strategy:
  Option<AvailabilityStrategy>` field (§4.3).
- `evaluate_transport_result` consults the resolved
  `HedgingStrategy` (§4) and `should_hedge()` (§5.1) when deciding
  whether to return `OperationAction::Hedge`.

The driver does not need a top-level wrapper: `execute_operation()`
calls `execute_operation_pipeline()` exactly as it does today.

### 8.2 Operation Cloning

Each hedged invocation needs its own:
- `OperationRetryState` — independent retry counters per region
- `DiagnosticsContextBuilder` — separate diagnostics chain per attempt
- `OperationOptions` — different `ExcludedRegions` per hedge

Items shared (via `Arc` or reference):
- `CosmosOperation` — immutable; body is `Bytes` (cheaply cloneable)
- `LocationStateStore` — lock-free; multiple readers are safe; **both
  legs apply LocationEffects to the same store** via
  `evaluate_hedge_leg_effects` (§6.5 invariant #10)
- `SessionManager` — designed for concurrent access
- `Credential` — `Arc`-wrapped
- **Hub-region-processing-only latch** — a single `Arc<AtomicBool>`
  is shared across the primary and the alternate hedge for the
  lifetime of the outer operation. See §9.6 for the full rationale;
  the short version is that the per-`OperationRetryState`
  `hub_region_processing_only` field added by [PR #4389][pr-4389] is
  otherwise per-hedge, which would force the alternate hedge to
  independently re-discover the hub region via its own 404/1002
  cycle. .NET v3 hit and fixed this in
  [azure-cosmos-dotnet-v3 PR #5815][dotnet-pr-5815] via the
  `CrossRegionAvailabilityContext` shared object; the Rust driver
  adopts the equivalent shared signal.

**Alternate-region distinctness.** Per §6.4.1's
`build_secondary_routing` contract, the alternate must differ from the
resolved primary on BOTH `region` AND `endpoint_key`. If no applicable
endpoint satisfies BOTH conditions, the upstream returns no
`OperationAction::Hedge` and the operation runs through the non-hedged
path. This guards against the case where a regional endpoint and its
global-account alias resolve to the same physical replica even though
their `Region` strings differ.

[pr-4389]: https://github.com/Azure/azure-sdk-for-rust/pull/4389
[dotnet-pr-5815]: https://github.com/Azure/azure-cosmos-dotnet-v3/pull/5815

### 8.3 Cancellation Propagation

Cancellation in the hedge path is **structural**, not signalled.
`execute_hedged()` owns both futures via `tokio::select!`; the loser
is dropped, which `Drop`s its in-flight transport (TPS §5.1) and
emits the cancellation signal at the next transport `await` point.
No `CancellationToken` is constructed.

```
                execute_hedged()
                       │
        ┌────────────────────────────┐
        │  tokio::select! biased; {  │
        │    primary_fut,            │
        │    secondary_fut,          │
        │    deadline_signal(),      │
        │  }                         │
        └──────────────┬─────────────┘
                       ▼
        winner returned → loser future dropped
                       │
                       ▼
        Drop chain runs through the transport pipeline
        → in-flight HTTP/AMQP request is cancelled at the
          next await point (TPS §5.1).
```

Application-cancellation enters via the per-attempt deadline (the
same mechanism used everywhere else in the transport pipeline). When
the deadline fires while both attempts are racing,
`execute_hedged()` harvests the most-advanced future for
diagnostics within `HARVEST_WINDOW = 50ms` (§6.5 #7) before
returning the cancellation error.

### 8.4 Local-Only Retries Inside a Hedge (Contract)

> **Contract:** Each hedged pipeline invocation runs the **full operation
> pipeline including the retry layer**, but the retry layer must perform
> **local-only retries** — it is forbidden from re-routing the request to
> a different region than the one targeted by that hedge.

This matches the .NET v3 behavior documented in
[Cross Region Request Hedging.md](https://github.com/Azure/azure-cosmos-dotnet-v3/blob/0830090ca4dc47b71398ea0871cbe00b591aa8fc/docs/Cross%20Region%20Request%20Hedging.md):
*"hedged requests are restricted to the region they are sent out in so no
cross region retries will be made, only local retries."*

**Mechanism.** `execute_hedged()` enforces this implicitly via
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

**Relationship to §6.5 invariant #10 (LocationEffects).** Local-only
*retries* and applying *LocationEffects* are independent contracts:

- A hedge leg MUST NOT re-route to a different region on retry (this
  section's contract — enforced via `ExcludeRegions`).
- A hedge leg's `TransportResult` IS applied to the shared
  `LocationStateStore` via `evaluate_hedge_leg_effects` so that the
  parent operation (and any sibling operations sharing the same store)
  observes endpoint-marks and `RefreshAccountProperties` requests
  identically to the non-hedged path (§6.5 invariant #10).

Effects are applied at the four `classify_hedge_result` sites *after*
the race resolves, so they neither interfere with the in-flight
`tokio::select!` nor delay a winning response. The two contracts
together produce the .NET v3 behavior: *region-pinned legs that
nonetheless feed the shared availability map.*

### 8.5 File Layout

New files:
```
src/driver/pipeline/
    hedging_diagnostics.rs  # HedgeDiagnostics struct
```

Modified files:
```
src/driver/pipeline/operation_pipeline.rs
    # - evaluate_transport_result() may return OperationAction::Hedge
    # - STAGE 7 dispatch arm calls execute_hedged()
    # - execute_hedged() implementation (§6.4)
src/driver/pipeline/transport_pipeline.rs
    # - ExecutionContext::Hedging marker on the secondary's transport_request
src/options/operation_options.rs    # Add availability_strategy field
src/options/mod.rs                  # Export AvailabilityStrategy, HedgingStrategy
src/diagnostics/mod.rs              # HedgeDiagnostics type
```

There is no `hedging.rs` module: the hedge code is a peer of the
rest of the pipeline dispatch in `operation_pipeline.rs`.

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

**Concurrent capture from competing hedges.** Even after `execute_hedged()`
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
moves to a last-write-wins model, `execute_hedged()` must instead
suppress session capture on hedges that observed cancellation before
STAGE 4 completed (or capture into a per-hedge buffer that only the winner
flushes). This invariant must be covered by the unit tests listed in §15.1.

### 9.3 Throughput Control

Each hedged request independently checks the throughput control group
budget. Hedging **does** increase RU consumption when the alternate
hedge actually executes transport. With the single-alternate model
(§6), the maximum RU multiplier introduced by hedging is **2×** — one
primary + one alternate.

The throughput control snapshot is acquired per-attempt in the
operation pipeline (STAGE 3), so the alternate sees the latest budget.

**Pathological interaction under TC saturation.** When the throughput
control group is saturated, both the primary and the alternate will be
throttled to 429 by the local TC gate before reaching the network.
`execute_hedged()` classifies 429 as transient (see §7.2), drains both
responses, and returns the most recent 429 — i.e., under TC saturation
hedging is at best a no-op and at worst adds 2× TC pressure plus the
threshold-timer latency on top.

Because hedging is on by default (§5.2), operators on TC-saturated
accounts should explicitly opt out via `AvailabilityStrategy::Disabled`
on the driver.

**Mitigations the implementation must adopt:**

1. **Sizing guidance (operator-facing docs).** State explicitly that
   the maximum RU multiplier introduced by hedging is **2×**, and TC
   group budgets should be sized with that headroom in mind.
2. **Short-circuit on local TC 429.** If the primary returns a
   TC-gate 429 *before* reaching transport (i.e., the throttle is
   local rather than service-side), `execute_hedged()` SHOULD treat
   that as a "do not fan out" signal — the alternate will hit the
   same gate. Distinguish this from a service-side 429 (which is
   genuinely region-local and benefits from hedging) via the response
   source field.
3. **Optional: exempt the alternate from TC accounting.**
   Speculative-hedge RU is not user-attributable; a losing alternate's
   RU is wasted by definition. A future option MAY skip TC accounting
   for the alternate hedge. Out of scope for Phase 1; tracked as a
   follow-up.

### 9.4 End-to-End Deadline

If `EndToEndOperationLatencyPolicy` is configured, **both the primary
and the alternate share the same deadline**. The deadline is computed
once at the start of `execute_hedged()` (after the threshold timer
fires — see §6.5 #3 zero-overhead-happy-path) and threaded into
both the primary's and the secondary's `OperationRetryState` via
pipeline invocation.

Implication: the alternate has less time budget than the primary. If
the deadline is 5s and the threshold is 3s, the alternate has only ~2s
to complete.

### 9.5 Hedging-win feedback into PPCB

A repeated win by the **alternate region** is signal: the primary
partition is consistently slow on this access pattern. The
`execute_hedged()` records each alternate-win via a callback into PPCB so
the circuit breaker can transition the primary partition to a degraded
state after a configurable number of consecutive wins.

**Mechanism (sketch — pending PPCB owner sign-off):**

```rust
// Called by `execute_hedged()` immediately after an alternate-region win,
// before returning the response to the caller.
fn record_hedge_win(
    location_store: &LocationStateStore,
    partition: PartitionKeyRangeId,
    primary_region: &Region,
) {
    location_store.record_consecutive_hedge_win(partition, primary_region);
    // PPCB's internal threshold (e.g., 5 consecutive wins) triggers
    // a transition to `Unhealthy` and the existing PPCB probe machinery
    // takes over. A primary-region win on the same partition resets
    // the counter.
}
```

**Invariants:**

1. **Counter is per (partition, primary_region) pair.** A hedge-win
   on partition P with primary region A does not affect partition Q.
2. **Primary-region wins reset the counter — *for any `Final`
   outcome*.** Any direct primary win on the same partition clears
   the consecutive-hedge-win counter so transient cross-region
   latency spikes do not accumulate. **A primary `Final` outcome
   includes application-classified HTTP failures** (404, 409, 412,
   `Pre/PostconditionFailed`, etc.) in addition to 2xx successes —
   what matters is that the primary leg produced a definitive
   response in time, not whether that response was a 2xx. Recording
   a reset on a primary 4xx is essential: such responses are
   structurally faster than transient retries (no backoff, no
   alternate region launch), and gating the reset on `result.is_ok()`
   would allow a sticky 4xx workload to retain stale alternate-win
   credit and oscillate partition routing.
3. **PPCB owns the threshold and the state transition.** The hedging
   `execute_hedged()` only emits the signal; whether N hedge-wins trip the
   breaker, and what state the partition transitions to, lives in the
   PPCB module. The threshold itself is operator-tunable via
   [`OperationOptions::consecutive_hedge_win_threshold`] (or the
   `AZURE_COSMOS_CONSECUTIVE_HEDGE_WIN_THRESHOLD` environment variable),
   with a default of `5` matching the .NET v3 SDK convention. Lower
   values trip the partition faster when the primary region is
   chronically slow; higher values are more tolerant of occasional
   latency spikes the hedge happens to win.
4. **Updates are lock-free / CAS-based.** Matches the existing
   `LocationStateStore` contract (§9.1).
5. **PK-range-id is captured even when both legs return transient.**
   `execute_hedged()` peeks at each `TransportResult`'s
   `CosmosResponseHeaders.partition_key_range_id` via the
   non-consuming `pk_range_id_from_result` helper at all four
   `classify_hedge_result` sites and accumulates the first non-`None`
   id observed across either leg into `captured_pk_range_id`. When
   the race finalizes as `HedgedRaceResult::BothTransient`, the
   captured id is threaded through `finalize_both_transient` and
   written back to `retry_state.partition_key_range_id` at the
   `BothTransient` handlers (STAGE 2b and STAGE 7) **only when
   currently `None`**, before `try_advance_after_both_transient`
   runs. Without this capture, a transient hedge that nonetheless
   surfaced a real PK-range-id (e.g. a `Gone/PartitionKeyRangeGone`
   with the partition's actual id in the headers) would lose that
   identity at the upgrade boundary, forcing a subsequent attempt to
   re-resolve from scratch. Symmetric to how `evaluate_transport_result`
   already captures `partition_key_range_id` on the non-hedged path —
   the hedge race must not regress this signal. Introduced by
   commit `827abc74d`.
6. **Hedging-win feedback is in addition to, not in place of,
   per-leg LocationEffects.** A secondary win still calls
   `apply_hedge_leg_effects` for each leg's response (per §6.5
   invariant #10) before invoking `record_hedge_win`. The PPCB
   counter complements — it does not replace — the immediate
   per-endpoint marks emitted by `evaluate_hedge_leg_effects`.

**Status:** This contract crosses module boundaries; the exact PPCB
state transition and threshold constant are out of scope for this
spec and will be co-designed with the PPCB owner before Phase 1 ships.
The `execute_hedged()`-side callsite is the load-bearing commitment here —
any PPCB-side implementation that consumes `record_consecutive_hedge_win()`
satisfies this contract.

### 9.6 Hub-Region-Processing-Only Header

The driver emits the `x-ms-cosmos-hub-region-processing-only: True`
request header on retries triggered by a `404 / 1002
(READ_SESSION_NOT_AVAILABLE)` response, scoped to **single-master
data-plane** operations. The header is specified in
[`HUB_REGION_PROCESSING_HEADER_SPEC.md`](https://github.com/Azure/azure-sdk-for-rust/blob/75c9c291528652355d62d2cd70af04b59a61e6d3/sdk/cosmos/azure_data_cosmos/docs/HUB_REGION_PROCESSING_HEADER_SPEC.md)
and implemented in [Rust PR #4389][pr-4389] (parity baseline with
[.NET PR #5447](https://github.com/Azure/azure-cosmos-dotnet-v3/pull/5447)).

#### 9.6.1 The hedging-specific correctness gap

The Rust latch lives on `OperationRetryState`
(`components.rs::OperationRetryState::hub_region_processing_only`)
and is set in `retry_evaluation.rs::build_session_retry_state` when
all four conditions hold:

1. `is_dataplane`
2. `!can_use_multiple_write_locations` (single-master account)
3. `session_token_retry_count == 0` (first 1002 within the operation)
4. `!hub_region_processing_only` (idempotency)

It is consumed in `operation_pipeline.rs::apply_hub_region_header` on
every subsequent transport attempt of the same operation.

Per §8.2, **the alternate hedge has its own `OperationRetryState`**.
Without additional coordination, this means the primary and the
alternate would independently observe their own first 1002, each
pay the full hub-discovery latency, and re-issue the next attempt
with the header set. The header's purpose — *bound the discovery
cycle to a single 1002 round-trip per operation* — is defeated for
whichever side has not yet observed 1002.

This is the same gap .NET v3 had after its first hub-region header
PR ([#5447](https://github.com/Azure/azure-cosmos-dotnet-v3/pull/5447))
and **explicitly fixed** in
[PR #5815 — *Read Consistency Strategy: Adds hub region header for
LastCommittedWriteRegion strategy*][dotnet-pr-5815], in the section
*"Hedging request with hub region header"*:

> When `CrossRegionHedgingAvailabilityStrategy` is active, the primary
> request may discover the hub region mid-flight … Hedged requests are
> clones of the original and run with their own `ClientRetryPolicy`
> instance, so they would normally repeat the entire hub discovery
> cycle independently. To avoid this redundant retry overhead, we
> introduce a `CrossRegionAvailabilityContext` — a lightweight shared
> object with a volatile `bool ShouldAddHubRegionProcessingOnlyHeader`
> flag. This context is injected into `RequestMessage.Properties`
> before the clone loop in `CrossRegionHedgingAvailabilityStrategy`.
> Since `Clone()` performs a shallow dictionary copy, all clones
> (primary + hedges) share the same `CrossRegionAvailabilityContext`
> reference. When the primary's `ClientRetryPolicy` sets the hub flag
> after 2× 404/1002, it also sets the flag on the shared context.
> Each hedge's `ClientRetryPolicy.OnBeforeSendRequest` reads this
> shared flag on every attempt and attaches the
> `x-ms-cosmos-hub-region-processing-only` header immediately —
> without needing to go through its own 404/1002 discovery.

The Rust hedge path MUST adopt the equivalent design.

#### 9.6.2 Required design — `Arc<AtomicBool>` shared latch

Construct a single `Arc<AtomicBool>` in `execute_hedged()` after
the threshold elapses (so the happy path stays alloc-free — §6.5 #3)
**before the alternate hedge is spawned** (i.e. after the threshold
timer fires — keeps the §6.5 invariant #3 zero-overhead happy path
intact), and thread it into the primary's pipeline params as well.
Concretely:

```rust
// In execute_hedged(), right after the threshold timer fires and
// before building the secondary transport_request:
let shared_hub_region_latch: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

// Both primary and alternate carry the same Arc.
let primary_retry_state = OperationRetryState::initial(/* … */)
    .with_shared_hub_region_latch(shared_hub_region_latch.clone());
let alternate_retry_state = OperationRetryState::initial(/* … */)
    .with_shared_hub_region_latch(shared_hub_region_latch.clone());
```

> **Happy-path note.** When the primary returns a final result before
> the threshold timer fires, no `Arc<AtomicBool>` is constructed and
> no latch threading happens — the per-state field added by
> [#4389][pr-4389] remains the only latch, exactly as today. This is
> required by §6.5 invariant #3 (zero-overhead happy path).

This requires a small extension to `OperationRetryState`:

```rust
pub struct OperationRetryState {
    // … existing fields …

    /// Per-operation hub-region-processing-only latch.
    /// Sticky for the lifetime of this `OperationRetryState`.
    pub hub_region_processing_only: bool,

    /// Cross-hedge shared latch. `Some(_)` only when this operation is
    /// running inside `execute_hedged()` past the threshold —
    /// `None` on the non-hedged code path and on the zero-overhead
    /// happy path, so today's allocator behavior is preserved.
    ///
    /// Mirrors .NET v3's `CrossRegionAvailabilityContext` injected
    /// into `RequestMessage.Properties` before the clone loop
    /// (azure-cosmos-dotnet-v3 PR #5815).
    pub shared_hub_region_latch: Option<Arc<AtomicBool>>,
}
```

The two existing call sites are then extended:

- **`build_session_retry_state` (latch-set side).** When the four
  trigger conditions fire and the new state sets
  `hub_region_processing_only = true`, also publish the result on
  the shared latch if present:

  ```rust
  if let Some(shared) = &retry_state.shared_hub_region_latch {
      shared.store(true, Ordering::Release);
  }
  ```

  `Release` is sufficient — the only thing being published is the
  bool itself; no further state hangs off it.

- **`apply_hub_region_header` (header-emission side).** Emit the
  header when *either* the per-state latch is set or the shared
  latch is set:

  ```rust
  let emit = retry_state.hub_region_processing_only
      || retry_state
          .shared_hub_region_latch
          .as_ref()
          .map(|shared| shared.load(Ordering::Acquire))
          .unwrap_or(false);
  if emit {
      transport_request.headers.insert(
          HeaderName::from_static(request_header_names::HUB_REGION_PROCESSING_ONLY),
          HeaderValue::from_static("True"),
      );
  }
  ```

This preserves the §5/§7/§8 invariants of
`HUB_REGION_PROCESSING_HEADER_SPEC.md` (account-level scope,
data-plane scope, idempotency / sticky semantics) on a per-hedge
basis while also propagating the discovery from one side of the
race to the other as soon as it happens.

#### 9.6.3 Eligibility — when the shared latch is actually wired

The shared latch is populated only when all of the following are true
at the point the alternate hedge is about to spawn inside
`execute_hedged()`:

| Condition | Why |
|---|---|
| Operation is data-plane (`is_dataplane`) | Mirrors the §1.5 scope of `HUB_REGION_PROCESSING_HEADER_SPEC.md`. |
| Account is single-master (`!can_use_multiple_write_locations`) | Mirrors AC-4 of `HUB_REGION_PROCESSING_HEADER_SPEC.md`; multi-master accounts have a separate recovery path and the header is never emitted. |
| Hedging actually fans out (threshold elapsed → secondary spawned) | When `execute_hedged()` returns from the happy path (§6.4 — primary wins before the threshold), there is no second pipeline to propagate to. |

When any condition fails, `shared_hub_region_latch` is `None` and the
existing per-state behavior from [#4389][pr-4389] is preserved
bit-for-bit.

#### 9.6.4 Interaction with §8.4 (Local-only retries inside a hedge)

The §8.4 local-only-retry contract is unaffected by the shared latch:
the latch governs only which request header is emitted, not the
endpoint resolution. `ExcludeRegions` continues to pin each hedge to
its own region across retries; the shared latch merely ensures every
hedge's retries — within their pinned region — also carry the
hub-region hint once any hedge has observed 1002. No new retry
trigger paths or region-fallback edges are introduced.

#### 9.6.5 Concurrency notes

- `AtomicBool` with `Release` / `Acquire` ordering is sufficient —
  the bool is the only thing being shared and there is no dependent
  state. `Relaxed` would also be functionally correct (single-flag
  race with a monotonic 0 → 1 transition) but `Release` / `Acquire`
  is preferred for reader/code-author clarity and costs nothing on
  every architecture the Rust SDK targets.
- The latch is monotonic 0 → 1 and never reset within an operation —
  matches the "sticky" semantics of the per-state latch in
  `components.rs`.
- The `Arc` is scoped to one outer `execute_hedged()` call, so
  it is dropped when `execute_hedged()` returns (no global state, no
  leak across operations).
- A losing hedge whose transport already responded after
  cancellation (cf. §14.2) may still observe and CAS-set the shared
  latch — this is benign: `execute_hedged()` has already returned a
  winner, and the next observer of the dropped `Arc` is no one.

#### 9.6.6 Forwarding the upgrade-path retry-state transition

The `OperationAction::Hedge` variant carries a `new_state: OperationRetryState`
field (added by commit `2bb080d15`) that mirrors the same field on
`FailoverRetry` and `SessionRetry`. In
`maybe_upgrade_to_hedge`, when an underlying `FailoverRetry` /
`SessionRetry` action is rewritten into `Hedge`, the new_state from the
original action is destructured and forwarded into the rewritten
`Hedge` variant.

In the STAGE 7 `Hedge` dispatch arm, `new_state` is applied to the
live `retry_state` via `advance_to_next_attempt` (same shape that
`FailoverRetry` / `SessionRetry` use today) **before constructing
`AttemptContext` and calling `execute_hedged`**. Without this step:

- `hub_region_processing_only_initial` (the snapshot read by
  `apply_hub_region_header` on the next attempt) would lag the latch
  set by the just-classified 1002, defeating the §9.6.1 cycle bound.
- `partition_key_range_id` captured by `evaluate_transport_result` on
  the upgrade-triggering attempt would be discarded, forcing the
  hedge legs to re-resolve from scratch.
- The session-token retry counter would not advance, breaking the
  AC-3 budget exhaustion check in
  `HUB_REGION_PROCESSING_HEADER_SPEC.md` §7.1.

The STAGE 2b pre-attempt path is unaffected — it never goes through
`maybe_upgrade_to_hedge`, so its `retry_state` is the unmodified
initial state.

#### 9.6.7 Cross-hedge propagation on `BothTransient`

The shared `Arc<AtomicBool>` (§9.6.2) handles the *forward*
propagation: as soon as either leg observes the four-condition 1002
trigger, both legs' subsequent in-leg retries see the latch via
`apply_hub_region_header`. But when **both** legs return transient
(`HedgedRaceResult::BothTransient`) and the operation falls back to the
non-hedged retry path on the parent `retry_state`, the parent did NOT
run any of the four-condition checks itself — it merely orchestrated
the race. Without an explicit fan-in, the parent's first post-hedge
retry attempt would re-issue WITHOUT the hub-region header, paying the
discovery cost a second time.

**Mechanism (commit pending — implements S2).**
`evaluate_hedge_leg_effects` returns a `HedgeLegEvaluation { effects,
observed_session_unavailable }` struct whose second field is set when
the four-condition trigger of §9.6.1 would have fired for that leg's
result and state snapshot. `execute_hedged` accumulates this across
the four `classify_hedge_result` sites into
`race_observed_session_unavailable: bool` and surfaces it via the new
`HedgedRaceResult::BothTransient { observed_session_unavailable, .. }`
field. The STAGE 2b and STAGE 7 `BothTransient` handlers then call
`propagate_hedge_session_unavailable(&mut retry_state, observed_session_unavailable)`,
which idempotently:

1. Sets `retry_state.hub_region_processing_only = true` if it isn't
   already.
2. `Release`-stores `true` on `retry_state.shared_hub_region_latch`
   (the same `Arc<AtomicBool>` the legs were sharing), so any
   surviving observer (e.g. a sibling operation that obtained a clone)
   also picks up the signal.

The function is a no-op when `observed_session_unavailable == false`
or when the latch was already flipped — matching the monotonic 0 → 1
semantics of the per-state field.

**Why a separate evaluator instead of reusing `evaluate_transport_result`.**
The non-hedged evaluator consumes the `TransportResult`/`CosmosError`
when building the `Abort` path. The hedge race cannot consume the
result — it needs to keep classifying for `Outcome::Final` /
`Outcome::Transient` and threading the response to the winner. The
non-consuming `evaluate_hedge_leg_effects` is the minimal mirror:
same `(operation, endpoint, retry_state, result)` tuple, same
LocationEffects, plus the new `observed_session_unavailable` signal —
but no `OperationAction` and no consumption of the underlying error.

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
`execute_hedged()` was entered. It is `None` in all of the following cases:

- No `AvailabilityStrategy` was resolved (no client/operation/driver-default
  strategy, or the user set `AvailabilityStrategy::Disabled`).
- A strategy was resolved but `should_hedge()` returned `false` (e.g.
  fewer than 2 applicable preferred regions, or the operation's
  `ResourceType` is outside the phase-allowed set — see §5.1).
- The strategy resolved but `execute_hedged()` short-circuited before
  spawning the primary (e.g. cancellation observed at entry).

**Field semantics when the primary wins before the first hedge fires:**

| Field | Value |
|---|---|
| `strategy_config` | The active strategy config (always populated) |
| `primary_region` | `regions[0]` |
| `alternate_region` | `None` |
| `response_region` | `Some(regions[0])` |
| `terminal_state` | `HedgeTerminalState::PrimaryWonPreThreshold` |

This lets callers distinguish *"hedging was active and the primary won
amongst the launched requests"* from *"hedging was active but no hedge
ever fired because the primary returned within the threshold window"*.

**Terminal-state taxonomy (authoritative for observability).** Because
the race can end in six structurally distinct ways — including states
where **no leg produced a final response** — `HedgeDiagnostics` carries
an explicit `terminal_state: HedgeTerminalState` field. Downstream
consumers (hedge win-rate metrics, alerting, dashboards) **must**
consult `terminal_state` rather than inferring intent from `was_hedge`
or the regions list alone.

| `HedgeTerminalState`              | `was_hedge` | Total reqs | Race outcome                                                           | Operation result                                       |
|-----------------------------------|-------------|------------|------------------------------------------------------------------------|--------------------------------------------------------|
| `PrimaryWonPreThreshold`          | `false`     | 1          | Primary finished before the threshold timer; alternate never spawned. | Success / final HTTP response                          |
| `DeadlineExceededPreThreshold`    | `false`     | 1          | Deadline fired pre-threshold; primary harvested for diagnostics.       | `application_cancelled_error`                          |
| `PrimaryWonAfterHedge`            | `false`     | 2          | Threshold elapsed, alternate spawned, primary still won the race.      | Success / final HTTP response                          |
| `AlternateWon`                    | `true`      | 2          | Alternate produced a `Final` outcome before the primary completed.     | Success / final HTTP response                          |
| `BothTransient { deadline_elapsed: true }`  | `false` | 2 | Both legs returned transient outcomes while the deadline had elapsed.  | `application_cancelled_error`                          |
| `BothTransient { deadline_elapsed: false }` | `false` | 2 | Both legs returned transient outcomes without the deadline firing.     | `transient_outcome_error`                              |
| `CancelledAwaitingPartner`        | `false`     | 2          | One leg transient, deadline fired while awaiting the partner leg.      | `application_cancelled_error`                          |

**Hedge win-rate invariant.** `was_hedge` is `true` **only** when
`terminal_state == AlternateWon`. Metrics computed as
`count(was_hedge=true) / count(*)` therefore correctly reflect the
fraction of operations where the alternate produced the response that
was returned to the caller — terminal-error states do **not** inflate
the numerator.

```rust
/// Diagnostic information about a hedging execution.
///
/// For successful responses this is attached to the winning response.
/// For terminal-error outcomes (deadline exceeded, both legs transient)
/// this is recorded for observability but the operation still returns
/// an `Err` to the caller.
///
/// All fields are private; read them through the accessors below.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct HedgeDiagnostics {
    /// The hedging strategy configuration that was active.
    strategy_config: HedgingStrategyConfig,
    /// The primary region the operation was initially dispatched to.
    primary_region: Region,
    /// The alternate region the hedge was dispatched to, if any.
    ///
    /// `None` when only the primary leg ran (terminal states
    /// `PrimaryWonPreThreshold` and `DeadlineExceededPreThreshold`);
    /// `Some(_)` otherwise.
    alternate_region: Option<Region>,
    /// The region whose response was returned to the caller.
    ///
    /// `Some(_)` only for the winning terminal states
    /// (`PrimaryWonPreThreshold`, `PrimaryWonAfterHedge`, `AlternateWon`);
    /// `None` for the terminal-error states where no leg produced a
    /// final response (`BothTransient`, `CancelledAwaitingPartner`,
    /// `DeadlineExceededPreThreshold`).
    response_region: Option<Region>,
    /// Structured classification of how the hedge race terminated.
    /// See the terminal-state taxonomy table above for semantics.
    terminal_state: HedgeTerminalState,
}

impl HedgeDiagnostics {
    pub fn strategy_config(&self) -> HedgingStrategyConfig { /* ... */ }
    pub fn primary_region(&self) -> &Region { /* ... */ }
    pub fn alternate_region(&self) -> Option<&Region> { /* ... */ }
    pub fn response_region(&self) -> Option<&Region> { /* ... */ }
    /// Authoritative classification of how the race ended. Replaced the
    /// legacy `was_hedge` boolean — derive that via
    /// `matches!(state, HedgeTerminalState::AlternateWon)`.
    pub fn terminal_state(&self) -> HedgeTerminalState { /* ... */ }
}

/// Structured classification of how a hedge race terminated. Used by
/// observability consumers to distinguish "alternate genuinely won"
/// from "race ended in a terminal error" — the latter must not be
/// counted in hedge win-rate metrics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum HedgeTerminalState {
    /// Primary returned `Final` before the threshold timer fired.
    PrimaryWonPreThreshold,
    /// Deadline fired before the primary or any hedge produced an
    /// outcome. Only the primary was launched.
    DeadlineExceededPreThreshold,
    /// Threshold elapsed and alternate was spawned, but the primary
    /// won the race anyway.
    PrimaryWonAfterHedge,
    /// Alternate produced the winning `Final` outcome.
    AlternateWon,
    /// Both legs returned transient outcomes. `deadline_elapsed`
    /// distinguishes the deadline-fired vs. fast-fail variants.
    BothTransient { deadline_elapsed: bool },
    /// One leg returned transient and the deadline fired while
    /// awaiting the partner leg's completion.
    CancelledAwaitingPartner,
}

#[derive(Clone, Debug)]
pub struct HedgingStrategyConfig {
    /// The configured threshold before the alternate-region hedge fires.
    pub threshold: HedgeThreshold,
}
```

### 10.2 DiagnosticsContext Integration

The secondary's `transport_request` is built with
`ExecutionContext::Hedging` (TPS §3.4) so that the on-wire request
carries an explicit "this is a hedged secondary attempt" marker.
This marker:

- Lets the transport layer attribute logs / spans / metric labels
  to the correct attempt class.
- Is the input that distinguishes a secondary attempt from a normal
  retry inside the per-attempt `transport_pipeline.rs` (TPS §5).
- Is set by `execute_hedged()` exactly once per operation (§6.4),
  on the secondary only.

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
    threshold = ?strategy.threshold().get(),
    regions = ?regions,
    "hedging enabled, launching primary request"
);

// When the threshold elapses and the alternate is spawned
tracing::debug!(
    target_region = %regions[1],
    elapsed = ?start.elapsed(),
    "launching alternate-region hedge"
);

// When a winner is found
tracing::debug!(
    region = %winner_region,
    elapsed = ?start.elapsed(),
    was_hedge,
    "hedging winner selected, cancelling loser"
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
| `cosmos.hedge.enabled_for_operation` | DEBUG | `threshold_ms`, `region_count` | `evaluate_transport_result` decides to hedge a specific operation |
| `cosmos.hedge.alternate_spawned` | DEBUG | `target_region`, `elapsed_ms` | The threshold elapsed and the alternate hedge was spawned |
| `cosmos.hedge.canceled` | DEBUG | `which` (`primary` / `alternate`), `target_region`, `reason` (`winner_found` / `deadline` / `app_canceled`) | A losing pipeline is canceled |
| `cosmos.hedge.winner_selected` | DEBUG | `winner_region`, `elapsed_ms`, `was_hedge` | A response is selected as final. DEBUG (not INFO) because the §5.2 driver default enables hedging on every multi-region read; emitting at INFO would amplify log volume by 1 event/op in production. Win-rate metrics should be derived from the `HedgeDiagnostics` chain (§10.3) rather than from log scrapes. |
| `cosmos.hedge.both_transient` | WARN | `last_status_code` | Both primary and alternate returned transient responses |
| `cosmos.hedge.recorded_alternate_win` | DEBUG | `primary_region`, `partition` | `execute_hedged()` recorded an alternate-region win for PPCB feedback (§9.5) |

**Reserved metric names** (intentionally namespaced; not emitted in
Phase 1, awaiting an `azure_core` metrics surface):

| Metric | Type | Labels | Description |
|---|---|---|---|
| `cosmos.hedge.operations_total` | counter | `result` (`primary_won` / `alternate_won` / `both_transient` / `disabled`) | Hedging-eligible operations grouped by outcome |
| `cosmos.hedge.alternate_spawned_total` | counter |  | Total alternate hedges spawned (i.e., operations where the threshold elapsed) |
| `cosmos.hedge.first_response_latency_ms` | histogram | `was_hedge` (bool) | Latency from `execute_hedged()` entry to the winning response |
| `cosmos.hedge.canceled_total` | counter | `reason` (`winner_found` / `deadline` / `app_canceled`) | Pipelines canceled before completion |
| `cosmos.hedge.ru_charge_winner` | histogram | `was_hedge` | RU of the winning response; this is the caller-visible RU charge |
| `cosmos.hedge.ru_charge_total` | histogram | `winner_region` | Total RU consumed across primary + alternate, including the loser; operator-facing only |
| `cosmos.hedge.consecutive_alternate_wins` | gauge | `partition`, `primary_region` | Current PPCB-feedback counter value for a (partition, primary-region) pair (§9.5) |

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
let threshold = HedgeThreshold::new(Duration::from_millis(500))
    .expect("500ms is non-zero");
let driver = CosmosDriverRuntimeBuilder::new()
    .build(endpoint, credential, DriverOptionsBuilder::new()
        .with_availability_strategy(AvailabilityStrategy::Hedging(
            HedgingStrategy::new(threshold),
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

// Or use a tighter threshold for a single read
let tight = HedgeThreshold::new(Duration::from_millis(200))
    .expect("200ms is non-zero");
let options = OperationOptionsBuilder::new()
    .with_availability_strategy(AvailabilityStrategy::Hedging(
        HedgingStrategy::new(tight),
    ))
    .build();
```

### 11.3 Layered Resolution

The `OperationOptionsView` layered resolution applies. For most fields it is:

```
operation > account > runtime > environment
```

The `hedging_enabled` master switch additionally recognizes a top-priority
`{ENV}_OVERRIDE` layer, making its full chain:

```
{ENV}_OVERRIDE > operation > account > runtime > {ENV}
```

If the operation sets `AvailabilityStrategy::Disabled`, it overrides a
client-level hedging strategy — unless a `hedging_enabled` value (from the
env switch or its `_OVERRIDE`) forces hedging on; see §11.3.1.

#### 11.3.1 Availability-strategy resolution priority

The driver picks the effective strategy in the following priority order
(highest first):

| Priority | Source | Notes |
|:---:|---|---|
| 0 | `hedging_enabled` master switch (`AZURE_COSMOS_HEDGING_ENABLED` and its `_OVERRIDE`, resolved through the layered view) | When set it is the source of truth: `Some(false)` forces hedging **off** even over an explicit `Hedging(..)`; `Some(true)` forces hedging **on** even over an explicit `Disabled` (a programmatic `Hedging(..)` still supplies its threshold). `None` (the default) defers to the layers below. |
| 1 | Operation `availability_strategy` (incl. `Disabled`) | Per-request override |
| 2 | Client / runtime `availability_strategy` | Applies to all requests |
| 3 | **Driver default** (§5.2) | Default-on for accounts with ≥ 2 applicable preferred regions; threshold = `min(1000ms, request_timeout / 2)`; independent of PPAF/PPCB |
| 4 | None | Hedging off (single-region account or insufficient region config) |

The resolved strategy is consumed by `evaluate_transport_result`
(TPS §3.4), which calls `should_hedge()` (§5.1) and (when eligible)
returns `OperationAction::Hedge { secondary_routing }` carrying the
chosen threshold and the secondary `RoutingDecision`. The pipeline
does the resolution lookup once per per-attempt iteration; there is
no separate orchestrator-side resolution step.

A user-configured `AvailabilityStrategy::Disabled` suppresses every lower
layer (the client/runtime strategy and the driver default) — **unless** the
`hedging_enabled` master switch (Priority 0) forces hedging on. Conversely,
`hedging_enabled = Some(false)` disables hedging even when an explicit
`Hedging(..)` is configured at any layer.

Operators who want to globally enable or disable hedging without a code
change can set `AZURE_COSMOS_HEDGING_ENABLED`; for an incident kill switch
that overrides even per-request code, set
`AZURE_COSMOS_HEDGING_ENABLED_OVERRIDE`. Leaving both unset preserves the
code-driven `AvailabilityStrategy` behavior described in §11.1.

---

## 12. Cancellation & Resource Cleanup

### 12.1 Structural Cancellation (No Token)

Cancellation in the hedge path is structural: `execute_hedged()`
owns both attempts via `tokio::select!`, and the loser's future is
dropped when the winner returns. Dropping the future runs the
standard `Drop` chain through the transport pipeline and cancels the
in-flight HTTP/AMQP request at the next `await` point (TPS §5.1).

```rust
// No CancellationToken in the hedge path.
tokio::select! {
    biased;
    r = &mut primary_fut, if !primary_done   => { /* primary won */ }
    r = &mut secondary_fut, if !secondary_done => { /* secondary won */ }
    _ = app_cancel_signal(deadline)          => { /* deadline tripped */ }
}
// On return, the un-polled future is dropped → in-flight request cancelled.
```

This is the same model TPS §4.2 uses for non-hedged retries and is
the reason the spec does not require `tokio_util` as a dependency
(see §17 Q10).

### 12.2 Pipeline Cooperation

The transport pipeline (TPS §5) already honors:

1. **Per-attempt deadline** — every request carries an `Instant`
   deadline, checked before send and at every layered `await`.
2. **Future-drop cancellation** — every `await` inside the transport
   stack is cancellation-safe; dropping the surrounding future at
   any point releases all owned resources (connection, body buffer,
   diagnostics row).

`execute_hedged()` relies on both: the per-attempt deadline lets it
observe application-level cancellation (the deadline is set from the
caller's overall budget), and the drop-the-future model lets it
cancel the loser without any additional plumbing.

### 12.3 No Task Spawning

`execute_hedged()` does **not** use `tokio::spawn` or `FuturesUnordered`
(see §6.4.2). The primary and alternate futures are pinned to the
`execute_hedged()`'s stack frame and polled directly by `tokio::select!`.
This means:

- No `'static + Send` constraint on captured values.
- No `JoinHandle` to drain, no orphaned-task risk.
- Dropping `execute_hedged()`'s future (e.g., on `tokio::time::timeout`
  expiry one layer up) drops both pipelines via standard structured
  concurrency \u2014 the in-flight transport `Drop`s emit the same
  cancellation signal a `CancellationToken` would.

Panics inside a pipeline propagate naturally back through the awaited
future rather than being caught by a `JoinHandle`. This matches the
behavior of every other awaited operation in the driver.

---

## 13. Write Hedging (Removed)

> **Removed from scope.** Earlier drafts of this spec described a
> multi-master write-hedging mode gated by an
> `enable_multi_write_region_hedge` flag, mirroring Java v4's behavior.
> That mode is now an explicit Non-Goal (see §1). Rationale:
>
> - Write hedging on multi-master amplifies 409 Conflict / 412
>   Precondition Failed surface area because the same write can be in
>   flight to multiple regions simultaneously via async replication.
> - Adoption in Java v4 has been near-zero; the operational surprise
>   of a "successful" hedge that produces a 409 on the loser-region
>   replay is consistently called out as a frequent customer issue.
> - Single-master writes have never been hedged in any SDK; PPAF
>   handles write failover for single-master.
>
> If service-side idempotency keys are added later, a separate
> proposal can revisit write hedging on that primitive. Until then,
> writes go through the existing operation pipeline unchanged — no
> hedging fan-out, no MM-write configuration knob.

This section is retained as a numbered placeholder so cross-references
to §13 in earlier reviews still resolve; the prior subsection
numbering (§13.1 / §13.2 / §13.3) is intentionally not preserved.

---

## 14. Error Handling & Edge Cases

### 14.1 Both Pipelines Return Transient Errors

If both the primary and the alternate return transient errors (e.g., 503
on both regions), `execute_hedged()` returns the **last** response
received. The retry logic within each pipeline invocation will have
already attempted retries before surfacing the error.

**State-mutation contract.** The `HedgedRaceResult::BothTransient`
finalizer is the seam where the parent operation's `retry_state`
absorbs everything the hedge race learned, so the next retry attempt
on the parent `retry_state` behaves identically to a non-hedged
attempt that observed the same conditions. Specifically:

1. **LocationEffects (§6.5 #10).** Both legs' results have already
   been routed through `evaluate_hedge_leg_effects` and applied via
   `location_state_store.apply()` at their respective
   `classify_hedge_result` sites. No additional apply happens at the
   `BothTransient` boundary — the marks are already live in the
   shared store by the time the race terminates.
2. **Hub-region 1002 latch (§9.6.7).** The
   `observed_session_unavailable` bool surfaced through
   `BothTransient` is passed to `propagate_hedge_session_unavailable`,
   which flips `retry_state.hub_region_processing_only` and
   `Release`-stores `true` on the shared `Arc<AtomicBool>` so the
   parent's next attempt emits the header.
3. **PK-range-id capture (§9.5 invariant #5).** The
   `partition_key_range_id` captured non-consumingly from either
   leg's `CosmosResponseHeaders` is written into
   `retry_state.partition_key_range_id` **only when currently
   `None`**, before `try_advance_after_both_transient` runs. This
   matches the capture the non-hedged path performs in
   `evaluate_transport_result`.
4. **Retry-counter accounting.** The retry counters on the parent
   `retry_state` advance via `try_advance_after_both_transient`
   exactly as they would for a single failed attempt. The race does
   not double-charge; each leg's own per-leg retries (§8.4) consumed
   that leg's budget, and the parent counts the race as one logical
   attempt.

After this fan-in, control returns to the normal pipeline loop in
`execute_operation_pipeline` and the next attempt re-evaluates eligibility
(hedge again, plain attempt, or abort) based on the now-updated
`retry_state`.

### 14.2 Primary Succeeds After the Alternate Hedge Launched

If the primary returns a final result 1ms after the alternate hedge is
launched, the alternate is cancelled. The alternate's transport request
may or may not have been sent (depends on timing). Cancellation is
best-effort — an in-flight HTTP request cannot be aborted mid-stream,
but the response will be discarded.

> **Divergence from .NET on application-cancel diagnostics.** When the
> *application* cancellation token fires mid-race, .NET awaits the
> most-recently-completed task with no timeout (relying on it being
> already completed) before re-raising. The Rust hedge path instead
> harvests the alternate pipeline within a bounded `HARVEST_WINDOW = 50ms`
> window (see §6.4 / §6.5 invariant #7). This bounds user-visible
> cancel latency under a stuck transport future at the cost of
> occasionally returning slightly less-rich diagnostics than .NET would.
> Documented as best-effort: "diagnostics-on-cancel are attached when
> the alternate has produced a result within 50ms of cancellation."

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

The applicable-region snapshot is captured at the start of
`execute_hedged()`. If account metadata refreshes during
execution (e.g., a `RefreshAccountProperties` effect), the
`ExcludeRegions` set on the already-launched alternate hedge is
unchanged.

Per the §8.4 contract, `ExcludeRegions` is a **hard constraint** inside
the alternate hedge: `resolve_endpoint()` does *not* fall back to an
excluded region even if the metadata refresh has marked the only allowed
region unavailable. If the alternate ends up with no eligible endpoint,
the retry layer returns the terminal "all eligible regions excluded"
condition (§8.4 item 2) as the result.

**How `execute_hedged()` handles that terminal condition.** The "all
eligible regions excluded" result is classified as transient (§7.2) and
does *not* short-circuit `execute_hedged()`. The primary continues racing,
and if the primary produces a final result it wins. If the primary is
also transient, §14.1 applies.

---

## 15. Test Plan

### 15.1 Unit Tests

| Test | Validates |
|------|-----------|
| `should_hedge_read_multi_region` | Reads eligible on multi-region account with ≥ 2 applicable preferred regions |
| `should_hedge_read_single_region` | Reads NOT eligible on single-region account |
| `should_hedge_excluded_to_one_region` | Reads NOT eligible when `ExcludeRegions` leaves < 2 applicable read endpoints |
| `should_hedge_no_preferred_regions` | NOT eligible when application-preferred-region list is empty |
| `should_hedge_write_never` | Writes (Create / Replace / Upsert / Delete / Patch) NEVER hedged regardless of topology |
| `should_hedge_non_document` | Non-Document `ResourceType`s excluded in Phase 1 |
| `should_hedge_disabled_override` | Per-operation `AvailabilityStrategy::Disabled` overrides client-level hedging |
| `is_final_result_success` | 200 → final |
| `is_final_result_conflict` | 409 → final |
| `is_final_result_503` | 503 → transient |
| `is_final_result_404_0` | 404/0 → final |
| `is_final_result_404_1002` | 404/1002 → transient |
| `is_final_result_429` | 429 → transient |
| `hedge_threshold_rejects_zero` | `HedgeThreshold::new(Duration::ZERO)` returns `None` (matches the §4.1 newtype contract) |
| `hedge_threshold_accepts_positive` | `HedgeThreshold::new(Duration::from_millis(1))` is `Some(_)` |
| `alternate_region_pin_excludes_primary` | Alternate hedge's `ExcludeRegions` contains the primary region |
| `alternate_region_pin_unions_user_excludes` | When the user supplied `ExcludeRegions = {X}`, the alternate hedge's set is `{X} ∪ (all_regions \ regions[1])` |
| `exclude_regions_honored_by_every_retry_trigger` | For each retry trigger class — PPAF write retry, PPCB markdown failback, transport-layer 503, throttling 429, session-token 1002 — fault-inject the trigger inside the alternate hedge and assert the retry attempt does **not** route to a region listed in the hedge's `ExcludeRegions`. Encodes the §8.4 cross-cutting invariant. |
| `app_cancel_preserves_hedge_diagnostics` | Cancel the application token while both pipelines are racing; assert the returned error carries `HedgeDiagnostics` from the most-advanced pipeline (covers §6.5 invariant #7). |
| `record_hedge_win_increments_ppcb_counter` | An alternate-region win calls `record_consecutive_hedge_win` exactly once on the primary partition (§9.5). |
| `primary_win_resets_hedge_win_counter` | A direct primary-region win clears the consecutive-hedge-win counter on that partition. |
| `zero_overhead_happy_path_no_allocs` | When the primary returns before the threshold timer fires, `execute_hedged()` allocates no per-hedge state (no `CancellationToken`, no cloned `OperationOptions`, no `ExcludeRegions` recompute). Backed by `dhat-rs` allocation count. |
| `shared_hub_region_latch_initialized_when_eligible` | `execute_hedged()` invoked on a data-plane / single-master operation; the threshold elapses and a secondary is spawned. Assert both the primary's and the secondary's `OperationRetryState.shared_hub_region_latch` are `Some(_)` and point to the same `Arc<AtomicBool>` instance (encodes §9.6.2 / §9.6.3). |
| `shared_hub_region_latch_none_on_zero_overhead_happy_path` | Primary returns before the threshold; assert no `Arc<AtomicBool>` was ever constructed and the per-state latch remains the only mechanism — preserves §6.5 invariant #3 and the [#4389][pr-4389] baseline allocator behavior (§9.6.2). |
| `shared_hub_region_latch_none_on_multi_master_or_metadata` | Multi-master *or* metadata pipeline; assert `shared_hub_region_latch` is `None` even when the alternate spawns, matching `HUB_REGION_PROCESSING_HEADER_SPEC.md` §5 account-level / §1.5 data-plane gates (§9.6.3). |
| `shared_hub_region_latch_propagates_first_1002_across_hedges` | Drive 1002 through `build_session_retry_state` on the primary; assert (a) the primary's per-state `hub_region_processing_only` is `true`, (b) the shared `Arc<AtomicBool>` is `true`, (c) on the next transport attempt the alternate — whose per-state latch is still `false` — has `apply_hub_region_header` emit the header. Rust counterpart of .NET PR #5815's `CrossRegionAvailabilityContext_PropagatesHubHeaderFlagToHedgedRequests` test. |
| `shared_hub_region_latch_no_1002_emits_no_header` | Neither side observes 1002; assert no transport attempt calls `apply_hub_region_header` with the header set, regardless of `shared_hub_region_latch` presence. |

### 15.2 Integration Tests (Fault Injection)

| Test | Setup | Validates |
|------|-------|-----------|
| `hedging_read_primary_slow` | 2s delay on Region A reads, threshold 200ms | Alternate Region B wins; diagnostics show `was_hedge=true`, `alternate_region=Some(B)` |
| `hedging_read_primary_fast` | No faults | Primary wins before threshold; `hedge_diagnostics=Some(_)` with `was_hedge=false` and `alternate_region=None` |
| `hedging_read_primary_503` | 503 on Region A reads | Alternate Region B wins with success |
| `hedging_read_both_regions_slow` | 2s delay on both regions | Whichever responds first wins (graceful degradation) |
| `hedging_write_not_hedged` | 2s delay on writes on a multi-master account | NO alternate hedge fires; write returns after the delay |
| `hedging_disabled_per_operation` | Client hedging on; operation `Disabled` | No alternate hedge; normal path |
| `hedging_respects_deadline` | threshold > deadline | No alternate fires; deadline error |
| `hedging_with_ppcb_existing_failures` | Region A primary has prior PPCB failures | Hedging still fires; PPCB and hedging compose without interference |
| `hedging_cancels_loser` | Delay on Region A | Region B wins; verify Region A transport task observed cancellation (hit_count ≤ 1) |
| `hedging_failback_to_primary` | Region A initially slow, then fast | First few reads hedged; subsequent reads complete on primary before the threshold |
| `hedging_exclude_regions_under_503_retry` | Alternate hedge gets a 503 (triggers transport retry) while a third region is healthy and excluded by that hedge's `ExcludeRegions` | Alternate hedge's retry stays pinned to its region (does NOT fall back to the third region) — fault-injection counterpart to the §8.4 invariant unit test. |
| `hedging_alternate_wins_trip_ppcb` | Force N consecutive alternate-region wins on the same partition | PPCB transitions the primary partition to `Unhealthy` after the configured threshold (§9.5). |
| `hedging_hub_region_header_propagates_across_hedges` | 2-region single-master data-plane account; fault-inject `404/1002` on the primary's first attempt against Region A, healthy 200 on the alternate against Region B after the threshold | Primary's retry against Region A emits `x-ms-cosmos-hub-region-processing-only: True` (per-state latch) **and** the alternate against Region B emits the same header on every attempt — without itself ever observing a 1002 (per the shared `Arc<AtomicBool>` from §9.6). Encodes the cross-hedge propagation invariant under fault injection; counterpart of .NET PR #5815's emulator-level coverage. |

### 15.3 Multi-Region Live Tests

Gated by `test_category = "multi_region"`:

| Test | Account Type | Validates |
|------|-------------|-----------|
| `hedging_read_cross_region` | 2-region SM | Read hedged to satellite when primary slow |
| `hedging_ppcb_feedback_cross_region` | 2-region SM with primary partition under load | Repeated alternate wins trip PPCB; subsequent reads route directly to the alternate without hedging until PPCB probes the primary back to `Healthy` |

---

## 16. Implementation Phases

The phased rollout introduced in §1 ("Operation-type scope (phased)")
maps onto the implementation milestones below. Each phase is auditable
against the §1 Goals.

| §1 Goal | Phase that closes it |
|---|---|
| **G1. Reduce tail latency** (p99/p99.9 bounded by `threshold + RTT`) | Phase 1 (point reads). Phase 2 widens to feed-style operations + metadata. |
| **G2. Transparent to application** (single `CosmosResponse`; opt-in diagnostics) | Phase 1 (`HedgeDiagnostics`, `DiagnosticsContext` integration). |
| **G3. Configurable** (single `threshold` knob at client and per-operation levels; explicit opt-out) | Phase 1. |
| **G4. Complementary to failover** (composes with PPAF/PPCB; feeds PPCB) | Phase 1 (lock-free `LocationStateStore` interaction §9.1 + PPCB feedback callsite §9.5). |
| **G5. Resource-safe** (≤ 2 concurrent pipelines, loser cancelled promptly) | Phase 1 (single-`select!` `execute_hedged()` §6.4 + structural drop-the-future cancellation §12). |
| **G6. Zero-overhead happy path** (no per-hedge state when primary wins early) | Phase 1 (gated by `zero_overhead_happy_path_no_allocs` test §15.1). |

§1 Non-Goals (single-region hedging, write hedging, multi-region
fan-out > 1 alternate, automatic threshold tuning, PPAF coupling)
remain out of scope for every phase below.

### Phase 1: Point-read Hedging + PPCB Feedback (MVP)

**Operation rows from §1 covered (Phase 1 column):**
- Document point reads (`GetItem`).

Writes are excluded by spec rule (§1 Non-Goals, §5.1 row 4). Feed-style
operations (Query / ReadMany / ChangeFeed) and metadata operations are
deferred to Phase 2 because they require additional coordination — see
that section.

**Scope:**

- `HedgeThreshold`, `HedgingStrategy`, `AvailabilityStrategy` types (§4).
- `should_hedge()` covering point reads (§5.1; phase-allowed
  `ResourceType` = `{Document}` with `OperationType = Read`).
- `is_final_result()` (§7.1).
- `execute_hedged()` (§6.4) extending the `OperationAction::Hedge`
  arm of `operation_pipeline.rs` STAGE 7, with:
  - zero-overhead happy path (§6.5 invariant #3),
  - single alternate-region hedge (§6.5 invariant #1),
  - app-cancel diagnostics preservation (§6.5 invariant #7).
- `HedgeDiagnostics` + `DiagnosticsContext` integration (§10).
- Integration into `cosmos_driver.rs` (§8.1) — final integration point
  to be coordinated with the in-pipeline shape in
  `TRANSPORT_PIPELINE_SPEC.md` §4.2 (§3.4).
- Cooperative cancellation via `CancellationToken` (§12); the loser
  pipeline observes cancellation at every pipeline `select!` point
  → satisfies **G5**.
- **Default-on activation** (§5.2) independent of PPAF/PPCB — driver
  default threshold `min(1000ms, request_timeout / 2)`, applied when
  the account has ≥ 2 applicable preferred regions and no user
  strategy is set.
- **Per-operation override surface** (satisfies **G3**):
  `OperationOptions::availability_strategy` accepting
  `Some(AvailabilityStrategy::Hedging(..))`,
  `Some(AvailabilityStrategy::Disabled)`, or `None`; layered
  resolution per §11.3 / §11.3.1.
- **PPCB feedback callsite** (§9.5): `record_consecutive_hedge_win`
  invoked on every alternate-region win; the PPCB-side state
  transition is co-designed with the PPCB module owner before
  Phase 1 ships (tracked as an in-flight dependency, not blocking the
  `execute_hedged()` design).
- **Hub-region-processing-only header cross-hedge propagation**
  (§9.6): extend `OperationRetryState` with
  `shared_hub_region_latch: Option<Arc<AtomicBool>>`; construct the
  `Arc<AtomicBool>` in `execute_hedged()` after the threshold
  elapses (preserves §6.5 invariant #3); update
  `build_session_retry_state` to publish to the shared latch on first
  1002 (`Release`) and `apply_hub_region_header` to read-OR
  per-state and shared latches (`Acquire`). Mirrors
  [.NET v3 PR #5815][dotnet-pr-5815]'s `CrossRegionAvailabilityContext`
  fix.
- Unit + fault-injection tests per §15 including the `§8.4`
  cross-cutting `ExcludeRegions` invariant, the `§6.5` invariant #3
  zero-overhead allocation test, the `§9.5` PPCB-feedback test, and
  the five `§9.6` shared-latch unit tests plus the
  `hedging_hub_region_header_propagates_across_hedges` fault-injection
  test.

**§1 Goals closed at end of Phase 1:** G2, G3, G4, G5, G6 in full;
G1 for point reads only.

**Out of scope this phase (deferred to Phase 2 / Future per §1 table):**
Feed-style operations (Query / ReadMany / ChangeFeed), metadata
operations, stored procedure execution, adaptive threshold tuning.

**Deliverables:**

- New files: `hedging.rs`, `hedging_diagnostics.rs` (see §8.5).
- Modified: `cosmos_driver.rs`, `operation_options.rs`, `mod.rs`,
  `diagnostics/mod.rs`, plus a `LocationStateStore`-side
  `record_consecutive_hedge_win` API contributed under the PPCB module.
- Extended: `components.rs::OperationRetryState` (new
  `shared_hub_region_latch: Option<Arc<AtomicBool>>` field),
  `retry_evaluation.rs::build_session_retry_state` (shared-latch
  publish), `operation_pipeline.rs::apply_hub_region_header`
  (per-state OR shared-latch emit) — per §9.6.

### Phase 2: Feed-style operations + Metadata

**Operation rows from §1 covered (Phase 2 column):**

- `QueryItems` — hedged **per page**.
- `ReadMany` — hedged **per page**.
- Change feed (`ReadFeed`) — hedged **per page**.
- Metadata operations: Database / Container / Offer / Throughput
  **reads only**.

**Scope (deferred — design pass required before scheduling):**

- Extend `should_hedge()`'s phase-allowed set to add the feed-style
  operations and the metadata read `ResourceType`s.
- **Per-page semantics for feeds.** Each page request is an
  independent hedge fan-out; the winning page's continuation token is
  forwarded to the next page. Final integration must align with the
  `FeedRange` abstraction being designed by the feed-operations spec
  (cross-coordination required — see also Ashley's spec); the
  per-page hedge boundary is conditional on that spec landing.
- **Metadata cache invalidation.** Hedged metadata reads must not
  produce stale-cache races when one region returns an older view
  than another; decide whether to prefer the latest `_etag` /
  resource id or the fastest response.
- **Diagnostics caveat for multi-stage operations.** Query / ReadMany /
  ChangeFeed contact regions *before* the hedge dispatch starts
  (query plan fetches, partition-key-range cache loads,
  identity-batching pre-flights, metadata-cache priming).
  `HedgeDiagnostics::alternate_region` covers only the regions the
  hedge path itself fanned out to; pre-hedge contacts show up in
  the surrounding `DiagnosticsContext` (existing per-attempt region
  trail). Operators must consult both surfaces to distinguish
  hedge-driven contacts from setup-driven contacts — keeps **G2**
  intact under multi-stage operations.

**§1 Goals advanced:** G1 widens to feed-style operations and
metadata reads. G2 extended with pre-hedge / hedge contact
disambiguation.

### Future: Stored Procedure Execution + Adaptive Thresholds

**Operation rows from §1 covered (Future column):**

- Stored procedure execution (`ExecuteJavaScript`) — **🟡 candidate**
  pending a separate design proposal (server-side state mutation,
  idempotency, body cloning of script payloads interact with hedging
  in non-obvious ways).

**Out-of-spec extensions** (explicit §1 Non-Goals — recorded here so
the future-work landing zone is well known):

- Latency histogram tracking per-region.
- Auto-tuning threshold based on observed p50 / p90 latency.
- Exponential backoff on the threshold after repeated alternate-wins.
- Write hedging on idempotency-key-aware service paths (if those
  primitives ever exist on the wire).

These items intentionally do **not** advance any §1 Goal; adding any
of them constitutes a new goal and requires a spec amendment.

---

## 17. Open Questions

1. **Should hedging be enabled by default?** — **Resolved.** Yes, on
   by default for accounts with ≥ 2 applicable preferred regions,
   independent of PPAF and PPCB (§5.2). Rationale: the Rust driver is
   greenfield and has no backward-compatibility constraint that
   forced .NET v3 and Java v4 to gate hedging on PPAF. Opt-out is via
   `AvailabilityStrategy::Disabled` (per-op or per-client).

2. **Interaction with `EndToEndOperationLatencyPolicy`** —
   **Resolved.** Primary and alternate share the deadline (§9.4).
   The alternate inherits the *remaining* budget after the threshold,
   not its own full timeout.

3. **RU accounting** — **Resolved.** Caller-visible per-operation RU
   charge is the **winning** response's RU only. Aggregate hedge cost
   (winner + alternate) is surfaced via the operator-facing
   `cosmos.hedge.ru_charge_total` metric (§10.4). Intentional
   divergence from .NET v3, which folds sub-request RU into
   `RequestCharge`.

4. **Composition with the `x-ms-cosmos-hub-region-processing-only`
   header from [PR #4389][pr-4389]** — **Resolved.** The per-state
   `hub_region_processing_only: bool` latch on `OperationRetryState`
   is per-hedge by construction (§8.2), which would defeat the
   header's *one round-trip per operation* guarantee under hedging.
   `execute_hedged()` constructs a single `Arc<AtomicBool>` shared
   between primary and alternate when the threshold elapses; both
   sides read-OR the per-state and shared latches at
   `apply_hub_region_header`. Full design in §9.6. Mirrors
   [.NET v3 PR #5815][dotnet-pr-5815]'s `CrossRegionAvailabilityContext`
   fix.

5. **Race with background PPCB failback** — **Resolved.** Primary and
   alternate are independent pipeline invocations with independent
   retry states; the shared `LocationStateStore` uses CAS-based
   updates that are safe under concurrency (§9.1).

6. **Max concurrent hedges cap** — **Resolved.** Capped at one
   alternate region (≤ 2 concurrent pipelines) by spec rule (§6.5
   invariant #1). This is a stronger guarantee than .NET v3 and
   Java v4, which fan out across all preferred regions; the cap is
   chosen to bound RU multiplier at 2× and avoid the diminishing
   returns of progressive fan-out.

7. **PPCB threshold for consecutive-secondary-win trip** — **Open.**
   `execute_hedged()` emits the `record_consecutive_hedge_win`
   signal on every secondary win (§9.5), but the threshold ("after
   N consecutive wins, mark partition `Unhealthy`") and the state
   transition itself live in the PPCB module. Pending PPCB-owner
   sign-off.

8. **Reconciliation with `TRANSPORT_PIPELINE_SPEC.md` §4.2** —
   **Resolved.** This spec adopts the TPS in-pipeline shape:
   `OperationAction::Hedge { secondary_routing }` returned by
   `evaluate_transport_result` and dispatched by STAGE 7 calling
   `execute_hedged()`. See §3.4 for the adoption statement and the
   five preserved invariants. The `OperationAction::Hedge` arm of
   `operation_pipeline.rs` is the only entry into `execute_hedged()`.

9. **FeedRange integration for Phase 2** — **Open.** Per-page hedging
   for Query / ReadMany / ChangeFeed must align with the `FeedRange`
   abstraction being designed by the feed-operation spec. Not a
   Phase 1 blocker; tracked in §16 Phase 2 scope.

10. **`tokio_util` dependency** — **Resolved.** Not needed under the
    TPS shape. Cancellation in the hedge path is structural:
    `tokio::select!` owns both attempts and dropping the loser's
    future cancels the in-flight transport via the standard `Drop`
    chain (§8.3, §12.1). No `CancellationToken` and therefore no
    `tokio_util` dependency is required.

11. **Threshold-policy disagreement with TPS §4.2** — **Open.** This
    spec specifies a static driver default of
    `min(1000ms, request_timeout / 2)` (§5.2), while TPS §4.2
    specifies a dynamic P99-based threshold clamped to 50–4000 ms.
    Both shapes are compatible with the
    `OperationAction::Hedge { secondary_routing }` enum (the
    threshold value is computed by the evaluator and passed into
    `execute_hedged()`), so the disagreement is a pure policy
    question, not a structural one. Pending cross-team alignment on
    which policy ships in v1.

---

## Appendix A: .NET SDK Source References

- [`AvailabilityStrategy.cs`](https://github.com/Azure/azure-cosmos-dotnet-v3/blob/master/Microsoft.Azure.Cosmos/src/Routing/AvailabilityStrategy/AvailabilityStrategy.cs) — Public factory methods
- [`CrossRegionHedgingAvailabilityStrategy.cs`](https://github.com/Azure/azure-cosmos-dotnet-v3/blob/master/Microsoft.Azure.Cosmos/src/Routing/AvailabilityStrategy/CrossRegionHedgingAvailabilityStrategy.cs) — Core implementation (410 lines)
- [`DisabledAvailabilityStrategy.cs`](https://github.com/Azure/azure-cosmos-dotnet-v3/blob/master/Microsoft.Azure.Cosmos/src/Routing/AvailabilityStrategy/DisabledAvailabilityStrategy.cs) — Sentinel for per-request disable
- [`AvailabilityStrategyInternal.cs`](https://github.com/Azure/azure-cosmos-dotnet-v3/blob/master/Microsoft.Azure.Cosmos/src/Routing/AvailabilityStrategy/AvailabilityStrategyInternal.cs) — Internal abstract base

## Appendix B: Glossary

| Term | Definition |
|------|-----------|
| Hedging | Sending the same request to a primary region and (after a threshold) one alternate region; first non-transient response wins |
| Threshold | Time before the alternate-region hedge fires |
| Alternate region | The single fallback region targeted by the hedge — `applicable_read_endpoints[1]` after `ExcludeRegions` filtering |
| Final result | A response that is definitively non-transient (success or permanent error) — see §7.1 |
| Transient result | A response that might succeed in another region (5xx except 501/505, timeout, 404/1002, 429, 410) — see §7.2 |
| PPAF | Per-Partition Automatic Failover (write failover on single-master). Independent of hedging in this driver. |
| PPCB | Per-Partition Circuit Breaker (read/write failover on failure threshold). Receives signal from hedging on repeated alternate-region wins (§9.5). |
| MM | Multi-master (multi-write-region) account |
| SM | Single-master account |
