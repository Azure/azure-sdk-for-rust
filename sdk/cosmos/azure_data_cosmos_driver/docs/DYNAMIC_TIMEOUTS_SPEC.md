# Dynamic Timeout Spec for `azure_data_cosmos_driver`

**Status**: Draft
**Date**: 2026-03-05
**Authors**: (team)

---

## Table of Contents

1. [Goals & Motivation](#1-goals--motivation)
2. [Current Behavior](#2-current-behavior)
3. [Design: Dynamic Timeout Escalation](#3-design-dynamic-timeout-escalation)
4. [Timeout Ladders](#4-timeout-ladders)
5. [Integration with Retry Loop](#5-integration-with-retry-loop)
6. [Interaction with End-to-End Deadline](#6-interaction-with-end-to-end-deadline)
7. [Interaction with ConnectionPoolOptions Bounds](#7-interaction-with-connectionpooloptions-bounds)
8. [Implementation Details](#8-implementation-details)
9. [Adaptive Connection Timeout](#9-adaptive-connection-timeout)
10. [Cross-SDK Reference](#10-cross-sdk-reference)
11. [Open Questions](#11-open-questions)

---

## 1. Goals & Motivation

### Why

When a Cosmos DB request times out and is retried, using the same timeout value for the retry is
often suboptimal. Transient network issues or momentary server load spikes may cause an initial
request to fail with a timeout, but a retry with the same timeout will likely fail the same way.
Progressively increasing the timeout on each retry attempt gives the operation a better chance of
succeeding without requiring the first attempt to use an unnecessarily long timeout — which would
add latency to the common case.

### Goals

1. **Reduce unnecessary failures**: When a request times out, increase the timeout on the next
   retry attempt to give the backend more time to respond. This avoids repeated timeout failures
   that could have succeeded with a slightly longer timeout.
2. **Preserve fast-path latency**: The first attempt uses a short, aggressive timeout. Only retries
   use longer timeouts. This keeps the common (non-retry) path fast.
3. **Align with other Cosmos SDKs**: The Java SDK already implements timeout escalation for
   metadata requests. This brings the same pattern to the Rust driver for both data plane and
   metadata requests.
4. **Leverage existing retry budget**: The operation pipeline already supports up to 3 failover
   retries by default, which naturally maps to the 3 tiers of the escalation ladder.

### Non-Goals

- **User-configurable ladders**: The escalation ladder is a fixed internal default. Users cannot
  override the step durations. The existing `ConnectionPoolOptions` min/max bounds still act as
  clamping limits (see [§7](#7-interaction-with-connectionpooloptions-bounds)).
- **Connection timeout escalation per retry**: Connection timeouts are NOT escalated per retry
  attempt like request timeouts are. Instead, connection timeouts use an adaptive model described
  in [§9](#9-adaptive-connection-timeout).
- **Query plan timeout escalation**: Deferred until query plan execution is implemented.

---

## 2. Current Behavior

### Static Timeouts

Today, timeouts are configured once via `ConnectionPoolOptions` and applied uniformly to every
request attempt:

| Timeout Type                | Default | Min   | Max   |
|-----------------------------|---------|-------|-------|
| Connection timeout (min)    | 100ms   | 100ms | 6s    |
| Connection timeout (max)    | 5s      | 100ms | 6s    |
| Data plane request (min)    | 100ms   | 100ms | 65s   |
| Data plane request (max)    | 6s      | 100ms | ∞     |
| Metadata request (min)      | 100ms   | 100ms | 6s    |
| Metadata request (max)      | 65s     | 100ms | 65s   |

**Note on `min_*` values**: The `min_*` timeout fields are defined in `ConnectionPoolOptions` with
getters and builder setters, but are **not yet consumed** outside `connection_pool.rs`. Only the
`max_*` values are wired into the transport (e.g., `max_connect_timeout()` is passed to
`reqwest::ClientBuilder::connect_timeout()`). The `min_*` fields exist as placeholders for the
dynamic timeout clamping described in [§7](#7-interaction-with-connectionpooloptions-bounds).

### Transport Retry

The retry loop lives in `execute_operation_pipeline()` (in `operation_pipeline.rs`), which runs a
7-stage loop. When a transport failure occurs, the pipeline evaluates the result and produces one
of:

- `FailoverRetry`: Retry in a different region/endpoint (budget: `max_failover_retries`, default 3)
- `SessionRetry`: Retry for session consistency (budget: `max_session_retries`, default 1)

The retry state is tracked in `OperationRetryState`:

```rust
pub(crate) struct OperationRetryState {
    pub failover_retry_count: u32,
    pub max_failover_retries: u32,
    pub session_token_retry_count: u32,
    pub max_session_retries: u32,
    // ... location, excluded regions, etc.
}
```

Currently, timeouts are static — every attempt uses the same timeout regardless of retry count.

---

## 3. Design: Dynamic Timeout Escalation

### Core Concept

Instead of using a single fixed timeout for all attempts, the driver uses an **escalation ladder**:
a fixed sequence of increasing timeout durations indexed by the attempt number. On each retry, the
next (longer) timeout in the ladder is used. The ladder values are not user-configurable — they are
internal defaults chosen to balance latency and reliability.

### Separate Mechanisms

- **Request timeouts**: Escalated per retry attempt via a fixed ladder (see [§4](#4-timeout-ladders)).
  Applied per-attempt via the existing `azure_core::sleep()` deadline-race mechanism.
- **Connection timeouts**: Adaptively tuned based on observed failure rate (see
  [§9](#9-adaptive-connection-timeout)). Applied at the `HttpClient` level by the
  `ShardedHttpTransport`.

---

## 4. Timeout Ladders

### Data Plane Request Timeout Ladder

| Attempt | Timeout |
|---------|---------|
| 0       | 6s      |
| 1       | 10s     |
| 2       | 65s     |

The jump from 10s to 65s is intentionally large. Tiers 0 and 1 cover transient spikes where a
slightly longer timeout resolves the issue. If 10s was not enough, the problem is likely
backend-side (e.g., cross-partition query fan-out, throttled partition) and needs the full timeout
budget. An intermediate step would just delay the inevitable success or failure without adding
signal. The 65s value matches `max_dataplane_request_timeout`.

### Metadata Request Timeout Ladder

Following the Java SDK pattern for `DatabaseAccount` metadata calls:

| Attempt | Timeout |
|---------|---------|
| 0       | 5s      |
| 1       | 10s     |
| 2       | 20s     |

### Ladder Behavior

- If `attempt >= ladder.len()`, use the last value in the ladder (the ladder "saturates" at its
  final tier).
- The ladder values are **not configurable** by users. They are internal constants.

---

## 5. Integration with Retry Loop

### Retry Model

Timeout escalation operates at the **transport pipeline level**, not the operation pipeline level.
The operation pipeline handles region failover (`FailoverRetry`) and session consistency
(`SessionRetry`) — these retries address routing and consistency concerns, not timeout failures.
Timeout escalation should only trigger when a request **actually times out**.

The transport pipeline (`execute_transport_pipeline()` in `transport_pipeline.rs`) already runs
its own retry loop for 429 throttling and local connectivity retries. Timeout escalation is
indexed by a **transport-level timeout retry counter** that increments only when the previous
attempt failed due to a timeout. Non-timeout failures (e.g., 429 throttle, connection refused)
do not advance the ladder.

### Timeout Selection per Attempt

Before each transport attempt, the pipeline selects the request timeout from the ladder based on
the timeout retry count:

```rust
fn timeout_for_attempt(timeout_retry_count: usize, ladder: &[Duration]) -> Duration {
    ladder[timeout_retry_count.min(ladder.len() - 1)]
}
```

The ladder is selected based on the existing `PipelineType` parameter passed to the transport
pipeline:

- `PipelineType::DataPlane` → `DATAPLANE_REQUEST_TIMEOUT_LADDER`
- `PipelineType::Metadata` → `METADATA_REQUEST_TIMEOUT_LADDER`

Attempts beyond the ladder length saturate at the final tier (see
[§4 Ladder Behavior](#4-timeout-ladders)).

### Hedging and Timeout Ladder Position

When hedging is enabled (see `TRANSPORT_PIPELINE_SPEC.md` §4.2), the hedged attempt runs in a
secondary region as a speculative execution. **Hedged attempts always start at tier 0 of the
timeout ladder**, independent of the primary attempt's ladder position.

Rationale: The hedged attempt targets a different regional endpoint. Network conditions (latency,
connectivity) are independent between regions. A short initial timeout that was insufficient for
the primary region may be perfectly adequate for the secondary region. Starting the hedged attempt
at the same escalated tier as the primary would waste time if the secondary region is responsive.

### Pseudocode

The timeout selection integrates into the transport pipeline's retry loop in
`execute_transport_pipeline()`. The ladder computes a per-attempt deadline that is clamped
against the end-to-end deadline and fed into the existing `remaining_request_timeout()` mechanism:

```rust
const DATAPLANE_REQUEST_TIMEOUT_LADDER: &[Duration] = &[
    Duration::from_secs(6),
    Duration::from_secs(10),
    Duration::from_secs(65),
];

const METADATA_REQUEST_TIMEOUT_LADDER: &[Duration] = &[
    Duration::from_secs(5),
    Duration::from_secs(10),
    Duration::from_secs(20),
];

// Inside execute_transport_pipeline():
let ladder = match pipeline_type {
    PipelineType::DataPlane => DATAPLANE_REQUEST_TIMEOUT_LADDER,
    PipelineType::Metadata => METADATA_REQUEST_TIMEOUT_LADDER,
};
let mut timeout_retry_count = 0_usize;

loop {
    // Compute per-attempt deadline from ladder
    let attempt_timeout = timeout_for_attempt(timeout_retry_count, ladder);
    let attempt_deadline = Instant::now() + attempt_timeout;

    // Clamp against e2e deadline (e2e always wins)
    let effective_deadline = match request.deadline {
        Some(e2e) => Some(attempt_deadline.min(e2e)),
        None => Some(attempt_deadline),
    };

    let per_request_timeout = remaining_request_timeout(effective_deadline);

    // Execute HTTP attempt with the computed timeout
    let result = execute_http_attempt(
        &http_request, transport, per_request_timeout, ...
    ).await;

    // On timeout: escalate and retry (if budget allows)
    if is_timeout_error(&result) {
        timeout_retry_count += 1;
        continue;
    }

    // On 429 throttle: existing throttle retry logic (no ladder advance)
    // On success or non-timeout error: return result
    return result;
}
```

---

## 6. Interaction with End-to-End Deadline

The end-to-end operation deadline (from `EndToEndOperationLatencyPolicy`) takes precedence over the
per-attempt timeout ladder. The per-attempt deadline computed from the ladder is clamped against
the e2e deadline — see [§7](#7-interaction-with-connectionpooloptions-bounds) for the full
clamping order.

If the remaining deadline is zero (or already exceeded), the retry loop exits immediately with a
timeout error — no attempt is made. This is existing behavior and is preserved.

---

## 7. Interaction with ConnectionPoolOptions Bounds

The `ConnectionPoolOptions` min/max bounds act as clamping limits on the ladder value **before**
the deadline clamp is applied. The clamping order is critical — pool bounds first, deadline last:

```rust
// Step 1: Clamp ladder value to pool bounds
let pool_clamped = ladder_value
    .max(connection_pool.min_dataplane_request_timeout())
    .min(connection_pool.max_dataplane_request_timeout());

// Step 2: Compute attempt deadline
let attempt_deadline = Instant::now() + pool_clamped;

// Step 3: Clamp against e2e deadline (e2e always wins)
let effective_deadline = match e2e_deadline {
    Some(d) => Some(attempt_deadline.min(d)),
    None => Some(attempt_deadline),
};
```

**The deadline must always be the final clamp.** If pool bounds were applied after the deadline,
`min_dataplane_request_timeout` could produce a timeout longer than the remaining deadline
(e.g., `min = 2s` with only `500ms` remaining).

This ensures that:
- If a user sets `max_dataplane_request_timeout` to 10s, the 3rd tier (65s) is clamped to 10s.
- If a user sets `min_dataplane_request_timeout` to 2s, the 1st tier (6s) stays at 6s (already
  above the minimum).
- The e2e deadline always takes precedence over both ladder values and pool bounds.
- The existing validation bounds in `ConnectionPoolOptionsBuilder::build()` remain unchanged.

---

## 8. Implementation Details

### Per-Attempt Deadline via Existing `TransportRequest.deadline`

The codebase consistently uses deadlines (`Option<Instant>`) for timeout propagation, not
durations. The transport pipeline's `remaining_request_timeout()` function computes the remaining
budget lazily from the deadline. The timeout ladder integrates into this pattern without adding
new fields to `TransportRequest`.

The ladder computes a per-attempt `Duration`, converts it to an `Instant` deadline, clamps it
against the e2e deadline, and passes the result as the `TransportRequest.deadline`. The existing
`remaining_request_timeout()` function continues to work unchanged.

#### No New Fields on `TransportRequest`

The existing `deadline: Option<Instant>` field already serves as the single source of truth for
timeout enforcement. The ladder's per-attempt timeout is folded into this field at the point where
the `TransportRequest` is constructed — no `request_timeout: Duration` field is needed, and there
is no risk of two competing timeout sources on the same struct.

#### Required Changes

**In `typespec_client_core`** — Add an optional per-request timeout to `Request`:

The `Request` struct (`typespec_client_core::http::request::Request`) currently has no timeout
field. Per-request timeouts are needed so that callers (like the Cosmos driver) can override the
client-level timeout on individual requests. The change is minimal and backwards-compatible:

- The new field is `Option<Duration>` defaulting to `None` — existing callers are unaffected.
- `Request` already derives `Clone`; `Option<Duration>` is `Clone + Copy`, so no breakage.
- `Debug` is manually implemented for `Request` and would include the new field.

**Cross-crate PR strategy**: This change touches `typespec_client_core`, which is shared across
all Azure SDK crates. It requires a separate PR with cross-team review before the Cosmos driver
can depend on it. The PR should add the field + getter/setter to `Request` only — the driver
applies the timeout via the existing `azure_core::sleep()` race mechanism.

```rust
// In typespec_client_core::http::request::Request
impl Request {
    /// Returns the per-request timeout override, if set.
    pub fn timeout(&self) -> Option<Duration> {
        self.timeout
    }

    /// Sets a per-request timeout that overrides the client-level timeout.
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = Some(timeout);
    }
}
```

**Timeout enforcement**: The transport pipeline currently enforces timeouts by racing
`azure_core::sleep()` against the HTTP future. This remains the enforcement mechanism. When
`Request::set_timeout()` becomes available in `typespec_client_core`, the sleep-race can be
removed in favor of native per-request timeouts in the `HttpClient` implementation (e.g.,
`reqwest::RequestBuilder::timeout()`). Until then, both mechanisms should not be active
simultaneously to avoid double-timeout enforcement with different error types.

**In `azure_data_cosmos_driver`**:

1. **Compute per-attempt deadline in the transport pipeline** (`transport_pipeline.rs`):

```rust
// Ladder constants
const DATAPLANE_REQUEST_TIMEOUT_LADDER: &[Duration] = &[
    Duration::from_secs(6),
    Duration::from_secs(10),
    Duration::from_secs(65),
];

const METADATA_REQUEST_TIMEOUT_LADDER: &[Duration] = &[
    Duration::from_secs(5),
    Duration::from_secs(10),
    Duration::from_secs(20),
];

fn timeout_for_attempt(timeout_retry_count: usize, ladder: &[Duration]) -> Duration {
    ladder[timeout_retry_count.min(ladder.len() - 1)]
}
```

2. **Integrate into the transport retry loop**: Before each attempt, compute the effective
   deadline from the ladder and clamp against the e2e deadline:

```rust
let attempt_timeout = timeout_for_attempt(timeout_retry_count, ladder);
let attempt_deadline = Instant::now() + attempt_timeout;
let effective_deadline = match request.deadline {
    Some(e2e) => Some(attempt_deadline.min(e2e)),
    None => Some(attempt_deadline),
};
let per_request_timeout = remaining_request_timeout(effective_deadline);
```

3. **Integration with `ShardedHttpTransport`**: The sharded transport sends requests via
   per-endpoint shard pools. The transport pipeline already enforces `per_request_timeout` via
   `azure_core::sleep()` racing the HTTP future. The ladder-computed deadline feeds directly into
   this existing mechanism — no changes to the sharded transport are needed.

### Diagnostic Recording

The effective per-attempt request timeout is recorded in `DiagnosticsContext` for observability.
This helps users understand why a retry succeeded when the initial attempt failed (e.g., "attempt 0
timed out at 6s, attempt 1 succeeded with a 10s timeout").

The effective request timeout is recorded as a flat field directly on `RequestDiagnostics`,
matching the established pattern for per-attempt metadata (`duration_ms`, `timed_out`, etc.):

```rust
// In RequestDiagnostics (existing struct):
pub effective_request_timeout: Option<Duration>,
```

This is recorded via `DiagnosticsContextBuilder::update_request()` alongside existing per-request
metadata (charge, activity ID, session token). The timeout value recorded is the **effective**
value after clamping by both `ConnectionPoolOptions` bounds and the end-to-end deadline.

### Where It Lives

- The ladder constants and `timeout_for_attempt()` free function should be defined in
  `src/driver/transport/transport_pipeline.rs` alongside the existing transport retry logic.
- The transport pipeline's retry loop is the integration point — it computes the per-attempt
  deadline and passes it to `remaining_request_timeout()`.

---

## 9. Adaptive Connection Timeout

### Motivation

Connection timeouts operate at a different level than request timeouts. The `reqwest::Client` sets
`connect_timeout` at construction time — it cannot be overridden per-request. This means the
per-retry escalation ladder pattern used for request timeouts does not apply.

However, a static connection timeout is also suboptimal. A 1s connection timeout is correct for
virtually all cloud/datacenter network environments and keeps the fast path aggressive. But
developers working from machines with higher-latency connections (e.g., VPN, poor Wi-Fi, remote
networks) may see persistent connection failures at 1s.

### Design: Failure-Rate Adaptive Tuning

The connection timeout starts at **1s** (the aggressive default) and is **adaptively increased to
5s** if the `ShardedHttpTransport` observes a sustained connection failure rate above a threshold.
This is a **one-time, persistent transition** — not a per-retry escalation.

**Reconciliation with existing config**: The existing `ConnectionPoolOptions::max_connect_timeout()`
defaults to 5s and is currently used directly as the `reqwest::Client` connect timeout. This spec
introduces a new internal **initial** connect timeout of 1s that starts below the pool max. The
adaptive mechanism transitions from this 1s initial value to the configured
`max_connect_timeout()` (5s) on sustained connection failures. The pool's `max_connect_timeout`
remains the upper bound — the adaptive mechanism never exceeds it.

```text
Normal state: connect_timeout = 1s
                    │
                    ▼
        ┌───────────────────────┐
        │ Connection failure    │
        │ rate exceeds          │──── YES ───▶ connect_timeout = 5s
        │ threshold?            │             (create new HttpClient)
        └───────────────────────┘
                    │ NO
                    ▼
              Keep 1s
```

### Key Properties

- **Start at 1s**: Sufficient for any well-connected cloud environment. Keeps the common case fast.
- **Fall back to 5s**: Only triggered when connection failures are persistent, indicating the
  client is on a slow or unreliable network. >1s is essentially only needed for developers on
  poor connections — production workloads running in Azure should never hit this.
- **Exactly-once transition**: This is not a per-attempt ladder. Once the `ShardedHttpTransport`
  decides to increase the connection timeout for an endpoint, it creates new `HttpClient` instances
  with the higher timeout and marks old shards as unhealthy for immediate reclamation.
- **No fallback to 1s**: Once elevated to 5s, the connection timeout stays at 5s for the lifetime
  of the driver. A restart resets to 1s. Reverting risks oscillation (1s→fail→5s→succeed→1s→fail
  cycle). Since >1s only matters for unusual environments, the permanent elevation is acceptable —
  the 4s overhead per connection is negligible compared to the reliability gain.
- **Idempotent transition**: Multiple concurrent connection failures may simultaneously observe
  the threshold exceeded. The transition uses lock-free atomics to match the sharded transport's
  hot-path pattern — an `AtomicBool` tracks the elevated state and an `AtomicU32` tracks
  consecutive failures. No Mutex is needed for the state transition itself:

```rust
// Per-endpoint state (lock-free hot path):
struct EndpointShardPool {
    connect_timeout_elevated: AtomicBool,         // false = 1s, true = 5s
    consecutive_connect_failures: AtomicU32,       // reset on success
    // ... existing shard management fields ...
}
```

### Implementation in `ShardedHttpTransport`

The `ShardedHttpTransport` (introduced in Step 6, PR #3957, in `sharded_transport.rs`) manages a
pool of `HttpClient` shards per endpoint. It tracks per-shard health metrics (consecutive
failures, inflight count, last success time) and runs a periodic background health sweep that
evicts unhealthy or idle shards.

Connection timeout adaptation fits naturally into this model:

1. **Track connection failures per endpoint**: The `ShardedHttpTransport` monitors connection
   failures (errors where `is_connect()` returns `true`) for each endpoint independently,
   aggregated across **all shards** for that endpoint. If any shard's connection attempt fails,
   the per-endpoint consecutive failure counter increments. A successful connection on **any**
   shard for that endpoint resets the counter. This ensures that a single consistently failing
   shard cannot be masked by healthy peers — the endpoint-level view captures systemic issues
   like DNS resolution failures or firewall rules that affect all connections to that host.
2. **Consecutive failure threshold**: When an endpoint accumulates **>3 consecutive connection
   failures**, the transport transitions that endpoint to the elevated timeout. Consecutive
   failures (rather than a failure rate) avoid false positives from transient blips during pool
   expansion or momentary network hiccups. The counter resets on any successful connection. A
   threshold of 3 provides enough signal that the issue is persistent while reacting before the
   application sees widespread failures.
3. **Create new clients and replace old shards**: New `HttpClient` instances are created with
   `connect_timeout = 5s` via the `HttpClientFactory`. Existing shards with the 1s timeout are
   **immediately marked unhealthy** so the health sweep reclaims them on its next pass, rather
   than waiting for natural drain. This ensures inflight requests on old shards do not continue
   failing at 1s while new shards are available with 5s.
4. **No `azure_core` changes needed**: This is entirely internal to the `ShardedHttpTransport`
   (introduced in Step 6). The `HttpClientFactory::build()` method reads `connect_timeout` from
   `ConnectionPoolOptions` (via `max_connect_timeout()`). To support adaptive connection timeouts,
   the `ShardedHttpTransport` overrides the connect timeout in its `base_client_config` before
   creating new shards.

```rust
// In ShardedHttpTransport, per-endpoint connection failure tracking:
// (called after each connection attempt)
fn on_connect_result(pool: &EndpointShardPool, success: bool) {
    if success {
        pool.consecutive_connect_failures.store(0, Ordering::Relaxed);
    } else {
        let prev = pool.consecutive_connect_failures.fetch_add(1, Ordering::Relaxed);
        if prev >= 3 && !pool.connect_timeout_elevated.swap(true, Ordering::Relaxed) {
            // First thread to flip the flag — mark old shards for eviction.
            // New shards will be created with connect_timeout = 5s.
            pool.mark_old_shards_for_eviction();
        }
    }
}
```

---

## 10. Cross-SDK Reference

### Java SDK (`azure-cosmos`)

From `sdk/cosmos/azure-cosmos/docs/TimeoutAndRetriesConfig.md`:

**Gateway mode (HTTP):**

| Operation Type   | Request Timeout Ladder | Connection Timeout |
|------------------|------------------------|--------------------|
| QueryPlan        | 0.5s, 5s, 10s         | 45s                |
| AddressRefresh   | 0.5s, 5s, 10s         | 45s                |
| Database Account | 5s, 10s, 20s          | 45s                |
| Other HTTP calls | 60s, 60s, 60s         | 45s                |

**Direct mode (TCP):**

| Operation Type | Request Timeout | Connection Timeout |
|----------------|-----------------|---------------------|
| All TCP calls  | 5s (fixed)      | 5s (fixed)          |

### This Spec (Rust Driver)

| Request Type | Request Timeout Ladder |
|--------------|------------------------|
| Data plane   | 6s, 10s, 65s           |
| Metadata     | 5s, 10s, 20s           |

**Differences from Java SDK:**
- Rust data plane uses 6s/10s/65s; Java direct mode uses a flat 5s.
- Java gateway "Other HTTP calls" use a flat 60s; Rust data plane starts lower (6s).
- Connection timeouts: Java uses a flat 45s (gateway) / 5s (direct). Rust uses an adaptive
  model starting at 1s and escalating to 5s on sustained connection failures.

---

## 11. Open Questions

1. **Metadata timeout ladder scope**: Should the metadata ladder (5s → 10s → 20s) apply to all
   metadata operations uniformly, or should specific metadata operations (e.g., database account
   reads) have their own ladders? For now, a single metadata ladder is proposed. Query plan and
   address refresh are deferred.
