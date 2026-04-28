# Dynamic Timeout Spec for `azure_data_cosmos_driver`

**Status**: Draft
**Date**: 2026-04-21
**Authors**: (team)

---

## Table of Contents

1. [Goals & Motivation](#1-goals--motivation)
2. [Design: Dynamic Timeout Escalation](#2-design-dynamic-timeout-escalation)
3. [Timeout Ladders](#3-timeout-ladders)
4. [Integration with Retry Loop](#4-integration-with-retry-loop)
5. [Interaction with End-to-End Deadline](#5-interaction-with-end-to-end-deadline)
6. [Interaction with ConnectionPoolOptions Bounds](#6-interaction-with-connectionpooloptions-bounds)
7. [Implementation Details](#7-implementation-details)
8. [Adaptive Connection Timeout](#8-adaptive-connection-timeout)
9. [Cross-SDK Reference](#9-cross-sdk-reference)
10. [Open Questions](#10-open-questions)

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
4. **Bounded transport-level retry budget**: Timeout escalation introduces a new transport-level
   timeout-retry counter (distinct from the operation pipeline's `max_failover_retries` budget,
   which governs region failover on routing failures). The counter is bounded by the ladder length
   so a saturated ladder cannot loop indefinitely. See [§4](#4-integration-with-retry-loop) for
   ordering relative to the existing throttle and local-connectivity counters.

### Non-Goals

- **User-configurable ladders**: The escalation ladder is a fixed internal default. Users cannot
  override the step durations. The existing `ConnectionPoolOptions` min/max bounds still act as
  clamping limits (see [§6](#6-interaction-with-connectionpooloptions-bounds)).
- **Connection timeout escalation per retry**: Connection timeouts are NOT escalated per retry
  attempt like request timeouts are. Instead, connection timeouts use an adaptive model described
  in [§8](#8-adaptive-connection-timeout).
- **User-facing per-attempt knobs**: Users do not configure individual ladder tiers, the
  per-attempt timeout, or the timeout-retry budget directly. Their only timeout controls are
  client-level min/max bounds via `ConnectionPoolOptions` (which clamp the ladder) and a per-call
  end-to-end deadline via `OperationOptions.end_to_end_latency_policy` (which
  always wins). This keeps the user surface narrow and avoids exposing an internal-only escalation
  schedule that may evolve.
- **Query plan timeout escalation**: Deferred until query plan execution is implemented.

---

## 2. Design: Dynamic Timeout Escalation

### Core Concept

Instead of using a single fixed timeout for all attempts, the driver uses an **escalation ladder**:
a fixed sequence of increasing timeout durations indexed by the attempt number. On each retry, the
next (longer) timeout in the ladder is used. The ladder values are not user-configurable — they are
internal defaults chosen to balance latency and reliability.

### Separate Mechanisms

- **Request timeouts**: Escalated per retry attempt via a fixed ladder (see [§3](#3-timeout-ladders)).
  Per-attempt enforcement requires direct access to a concrete `reqwest::Client`'s
  `RequestBuilder::timeout()` because the `azure_core::http::HttpClient` trait dispatch
  (`HttpClient::execute_request(&Request)` in `typespec_client_core`) exposes no per-request
  timeout option — see [§7](#7-implementation-details) for the dispatch path that makes this
  available driver-internally.
- **Connection timeouts**: Adaptively tuned based on observed failure rate (see
  [§8](#8-adaptive-connection-timeout)). Applied at the `HttpClient` level by the
  `ShardedHttpTransport`.

---

## 3. Timeout Ladders

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

**Future: Thin client ladder for Gateway20** *(Status: deferred)*: The current codebase already
supports the Gateway20 transport via `HttpClientConfig::dataplane_gateway20(...)`, with dispatch
through `AdaptiveTransport::gateway20(...)` / `TransportMode::Gateway20`. In this spec, "thin
client" refers to a future Gateway20 mode with distinct retry semantics (for example,
gateway-managed retries or a separate binary-protocol path), not merely the existing HTTP/2
prior-knowledge Gateway20 transport. If that future thin-client mode is introduced, data plane
operations can use a tighter ladder of **6s → 6s → 10s**. Until then, the existing Gateway20
transport continues to use the standard data-plane timeout ladder. In the future thin-client
mode, the gateway performs server-side retries on behalf of the client within the first 6s
window, so a second attempt at 6s is appropriate before escalating to 10s.

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

## 4. Integration with Retry Loop

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

#### Counter Interactions

The transport pipeline now tracks three independent retry counters in one loop:

| Counter | Advances on | Bound |
| --- | --- | --- |
| `throttle_state.attempt_count` (existing) | 429 + `Retry-After` | `Retry-After` schedule and e2e deadline |
| `local_connectivity_retry_count` (existing) | Local connectivity error from a different shard | `MAX_LOCAL_CONNECTIVITY_RETRIES` (1) |
| `timeout_retry_count` (new) | `is_timeout_error(&result)` | `MAX_TIMEOUT_RETRIES` (= ladder length) |

**Evaluation order on a failed attempt** (first matching condition wins, advancing only that
counter):

1. **Throttle (429 with `Retry-After`)** — advances `throttle_state.attempt_count` and applies
   the server-supplied delay. Does NOT advance `timeout_retry_count` (the request did not time
   out, the server told us to back off).
2. **Local connectivity error from a peer shard** — advances `local_connectivity_retry_count`
   (gated by `prior_failed_transport_shards` as today). Does NOT advance the ladder.
3. **Per-attempt timeout** (`is_timeout_error(&result)` true) — advances `timeout_retry_count`.
   The next attempt uses the next ladder tier.
4. **Other transport error** — returned to the operation pipeline, which decides whether to
   advance `failover_retry_count` (region failover) or `session_token_retry_count`.

A single attempt can advance **at most one** counter. Timeout retries are not gated by
`prior_failed_transport_shards` — the same shard is reused with a longer per-attempt timeout
unless the operation pipeline subsequently triggers failover.

`timeout_retry_count` is **bounded by `MAX_TIMEOUT_RETRIES = ladder.len()`** (3 for both
data-plane and metadata ladders, giving 4 total attempts including the initial one — tier 0,
tier 1, tier 2, plus one saturated retry at the final tier). This bound is enforced
regardless of the e2e deadline. Without an upper bound, an operation with no e2e deadline
configured could loop indefinitely on persistent backend slowness, holding shard pool slots
across attempts.

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
[§3 Ladder Behavior](#3-timeout-ladders)).

### Hedging and Timeout Ladder Position

When hedging is enabled (see `TRANSPORT_PIPELINE_SPEC.md` §3.2), the hedged attempt runs in a
secondary region as a speculative execution. **Hedged attempts always start at tier 0 of the
timeout ladder**, independent of the primary attempt's ladder position.

Rationale: The hedged attempt targets a different regional endpoint. Network conditions (latency,
connectivity) are independent between regions. A short initial timeout that was insufficient for
the primary region may be perfectly adequate for the secondary region. Starting the hedged attempt
at the same escalated tier as the primary would waste time if the secondary region is responsive.

### Pseudocode

The timeout selection integrates into the transport pipeline's retry loop in
`execute_transport_pipeline()`. The per-attempt timeout is computed as a `Duration`, clamped
against pool bounds and the remaining end-to-end budget, and passed through the dispatch chain
**without overwriting `request.deadline`** (which remains the e2e budget consulted by the rest of
the retry loop):

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

const MAX_TIMEOUT_RETRIES: usize = DATAPLANE_REQUEST_TIMEOUT_LADDER.len(); // hard upper bound

// Inside execute_transport_pipeline():
let ladder = match pipeline_type {
    PipelineType::DataPlane => DATAPLANE_REQUEST_TIMEOUT_LADDER,
    PipelineType::Metadata => METADATA_REQUEST_TIMEOUT_LADDER,
};
let mut timeout_retry_count = 0_usize;

loop {
    // 1. Pick ladder value, then clamp to pool bounds.
    let ladder_value = timeout_for_attempt(timeout_retry_count, ladder);
    let pool_clamped = ladder_value
        .max(connection_pool.min_request_timeout(pipeline_type))
        .min(connection_pool.max_request_timeout(pipeline_type));

    // 2. Clamp against remaining e2e budget. Do NOT mutate request.deadline.
    let attempt_timeout = match request.deadline {
        Some(e2e) => pool_clamped.min(e2e.saturating_duration_since(Instant::now())),
        None => pool_clamped,
    };

    if attempt_timeout.is_zero() {
        return Err(timeout_exceeded()); // e2e exhausted
    }

    // 3. Dispatch with the per-attempt timeout (threaded through the dispatch chain;
    //    request.deadline is preserved for Retry-After honor and other loop-level checks).
    let result = transport
        .send_with_dispatch(&http_request, attempt_timeout, ...)
        .await;

    // 4. Counter selection (see "Counter Interactions" above — at most one advances).
    if is_throttle_with_retry_after(&result) {
        throttle_state.advance(&result, request.deadline)?;
        continue;
    }
    if is_local_connectivity_error(&result) && can_retry_local_connectivity(...) {
        local_connectivity_retry_count += 1;
        continue;
    }
    if is_timeout_error(&result) {
        timeout_retry_count += 1;
        if timeout_retry_count > MAX_TIMEOUT_RETRIES {
            return Err(timeout_retries_exhausted()); // hard cap, even with no e2e
        }
        continue;
    }
    return result;
}
```

This pseudocode is the **single source of truth** for the integration shape.
[§7](#7-implementation-details) describes only the dispatch-side plumbing
(`send_with_dispatch`, `reqwest::RequestBuilder::timeout()`) — it does not redefine the loop.

---

## 5. Interaction with End-to-End Deadline

The end-to-end operation deadline (from `EndToEndOperationLatencyPolicy`) takes precedence over the
per-attempt timeout ladder. The per-attempt deadline computed from the ladder is clamped against
the e2e deadline — see [§6](#6-interaction-with-connectionpooloptions-bounds) for the full
clamping order.

If the remaining deadline is zero (or already exceeded), the retry loop exits immediately with a
timeout error — no attempt is made. This is existing behavior and is preserved.

### Per-Attempt Deadline ≠ `request.deadline`

The existing `TransportRequest.deadline` field is the **end-to-end** deadline. It is consulted at
multiple points in the transport pipeline retry loop (see `transport_pipeline.rs:201,321,339,354`)
to decide whether to abandon further retries. The per-attempt timeout from the ladder is therefore
**not** stored back into `request.deadline` — doing so would silently shorten the e2e budget that
the rest of the loop relies on for retry decisions, in particular the `Retry-After` honor logic
that compares the suggested back-off to the remaining e2e budget.

Instead, the per-attempt timeout is carried as a separate `attempt_timeout: Duration` value through
the dispatch chain (see [§7](#7-implementation-details) for the threading), and the resulting
`reqwest::RequestBuilder::timeout()` enforcement is independent of `request.deadline`.

---

## 6. Interaction with ConnectionPoolOptions Bounds

The `ConnectionPoolOptions` min/max bounds act as clamping limits on the ladder value **before**
the deadline clamp is applied. The clamping order is critical — pool bounds first, deadline last:

```rust
// Step 1: Clamp ladder value to pool bounds
let pool_clamped = ladder_value
    .max(connection_pool.min_dataplane_request_timeout())
    .min(connection_pool.max_dataplane_request_timeout());

// Step 2: Clamp against remaining e2e budget (e2e always wins). Result is a Duration —
// request.deadline is NOT mutated; see §5 "Per-Attempt Deadline ≠ request.deadline".
let attempt_timeout = match request.deadline {
    Some(e2e) => pool_clamped.min(e2e.saturating_duration_since(Instant::now())),
    None => pool_clamped,
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

### Invariant: `min ≤ max`

The `.max(min).min(max)` chain produces the wrong result if `min > max` (it returns `max` even
when `ladder_value < max`, because `max(min, x) = min > max`). Correctness here depends on the
existing `ConnectionPoolOptionsBuilder::build()` validation that rejects configurations where
`min > max` for any of the four request-timeout fields. Any future relaxation of that validation
would require revisiting the clamp order.

### Migration Note: `min_*_request_timeout` becomes a hard floor

The `min_dataplane_request_timeout` (default 100ms) and `min_metadata_request_timeout`
(default 100ms) options are currently **placeholders** — the codebase does not consult them when
computing the per-request timeout (see `http_client_factory.rs:173`, which only reads the `max_*`
values). After this spec lands, those `min_*` options become an effective **lower bound on every
per-attempt timeout**.

For default configurations this is a no-op: 100ms is well below the smallest ladder tier (5s).
However, a user who deliberately set `min_dataplane_request_timeout = 30s` to "force long
timeouts" will, after this change, see tier 0 (6s) clamped **up** to 30s — a 5× regression on the
fast path. The release notes for the change must call this out, and the `ConnectionPoolOptions`
docs for the four `min_*` / `max_*` fields must be updated to describe their post-spec
semantics.

---

## 7. Implementation Details

### Per-Attempt Timeout Threading

The codebase consistently uses the existing `TransportRequest.deadline: Option<Instant>` field as
the **end-to-end** deadline. As discussed in [§5](#5-interaction-with-end-to-end-deadline), the
per-attempt ladder value is **not** folded into that field, because the rest of the transport
retry loop (`Retry-After` honor, abandonment checks, hedging coordination) reads it as the e2e
budget.

Instead, the per-attempt timeout is plumbed as a separate `Duration` value through the dispatch
chain:

```text
execute_transport_pipeline()        // §4 loop computes attempt_timeout: Duration
        │
        ▼
AdaptiveTransport::send_with_dispatch(req, attempt_timeout, …)
        │
        ▼
ShardedHttpTransport::send(req, attempt_timeout, …)
        │
        ▼
  reqwest::RequestBuilder::timeout(attempt_timeout).send().await
```

The current signatures of `AdaptiveTransport::send_with_dispatch()` and
`ShardedHttpTransport::send()` take only `&Request`; they will gain an
`attempt_timeout: Duration` parameter. The future hedging path in `AdaptiveTransport` (not yet
implemented) gets the same parameter and applies tier-0 (`ladder[0]`) per [§4 "Hedging and Timeout
Ladder Position"](#hedging-and-timeout-ladder-position).

### Reaching `reqwest::RequestBuilder::timeout()` from the Driver

The driver owns the `HttpClientFactory` (`http_client_factory.rs`) that constructs `reqwest::Client`
instances, but the factory currently returns `Arc<dyn TransportClient>`. Once a request is
dispatched via the `TransportClient` trait, there is **no surface** for a per-request timeout —
the trait was designed without one (see tracking issue
[#3878](https://github.com/Azure/azure-sdk-for-rust/issues/3878) and the existing TODO at
`transport_pipeline.rs:225-227`).

Because the driver already owns the construction path, this spec resolves the gap **driver-side**
without waiting on issue #3878:

1. `HttpClientFactory::build()` returns a driver-owned wrapper that exposes the underlying
   concrete `reqwest::Client` (e.g., `Arc<DriverHttpClient>` where `DriverHttpClient` holds a
   `reqwest::Client` directly). The wrapper still implements `azure_core::http::HttpClient` for
   any other consumer that needs the trait, but the driver's transport layer holds and uses the
   concrete type.
2. `ClientShard` (`sharded_transport.rs`) stores the `Arc<DriverHttpClient>` instead of
   `Arc<dyn TransportClient>`. `ShardedHttpTransport::send()` calls
   `client.dispatch_with_timeout(req, attempt_timeout)` which internally builds the
   `reqwest::Request` and applies `RequestBuilder::timeout(attempt_timeout)` before `.send()`.
3. **`HttpClientFactory::build()` no longer sets a client-level
   `builder.timeout(config.request_timeout)`** at `http_client_factory.rs:173`. That call must be
   removed (or set to a very high safety ceiling such as `2 × max_*_request_timeout`) so the
   per-request value has full control. Without this change, every attempt would also be subject
   to the original 6s / 65s client-level cap and the ladder's longer tiers (10s, 65s) would not
   actually take effect.
4. The hedging primary/secondary, the existing `local_connectivity` retry, and any future thin
   client dispatch path all funnel through the same `dispatch_with_timeout()` entry point. There
   is no new code path for timeout enforcement.

This is intentionally a **driver-internal** concrete-reqwest dispatch path — it does not change
`typespec_client_core` or the public `TransportClient` trait. Consumers outside the
driver who pass an `Arc<dyn TransportClient>` into the driver are not supported for timeout
escalation; the driver constructs its dispatch clients itself via `HttpClientFactory` regardless
of any externally supplied `HttpClient`.

### Timeout Control via Options Hierarchy

User-facing timeout control is provided through the driver's options containers.
The timeout ladder is internal behavior — users influence it indirectly via bounds and deadlines.
Users do **not** get to configure individual ladder tiers, the per-attempt timeout, or the
timeout-retry budget directly (see [§1 Non-Goals](#1-goals--motivation)).

**User-Facing Options:**

- **`ConnectionPoolOptions`** (client-level, set once at driver creation)
  - `max_dataplane_request_timeout` (default 6s) — clamps ladder max
  - `min_dataplane_request_timeout` (default 100ms) — clamps ladder min
    (see [§6 Migration Note](#migration-note-min__request_timeout-becomes-a-hard-floor))
  - `max_metadata_request_timeout` (default 65s) — clamps ladder max
  - `min_metadata_request_timeout` (default 100ms) — clamps ladder min
- **`RuntimeOptions`** (inheritable: driver-level → operation-level)
  - `end_to_end_latency_policy` — overall operation deadline
    (set via `DriverOptions` or overridden per-operation via
    `OperationOptions.end_to_end_latency_policy`)

**Internal Driver Behavior:**

- Timeout ladder computes per-attempt `Duration` from constants
- Clamps to `ConnectionPoolOptions` min/max bounds ([§6](#6-interaction-with-connectionpooloptions-bounds))
- Clamps against remaining e2e budget ([§5](#5-interaction-with-end-to-end-deadline)) — e2e always wins
- Threaded through dispatch chain as a `Duration` parameter (see "Per-Attempt Timeout Threading"
  above)
- Enforced via `reqwest::RequestBuilder::timeout()` per request inside the driver-owned
  dispatch wrapper
- `reqwest::Client` built **without** a client-level timeout (or with a high safety ceiling) so
  per-request timeouts have full control — requires removing/changing
  `http_client_factory.rs:173`

#### Example: User sets a 10s per-request max and 30s e2e timeout

```rust
// Client-level: cap per-request timeout at 10s
let pool = ConnectionPoolOptions::builder()
    .with_max_dataplane_request_timeout(Duration::from_secs(10))
    .build()?;

// Operation-level: 30s overall budget
let mut options = OperationOptions::default();
options.end_to_end_latency_policy = Some(
    EndToEndOperationLatencyPolicy::new(Duration::from_secs(30))
);

// Internal behavior (not user-visible):
// Attempt 0: ladder says 6s  → clamped to min(6s, 10s) = 6s  → reqwest timeout = 6s
// Attempt 1: ladder says 10s → clamped to min(10s, 10s) = 10s → reqwest timeout = 10s
// Attempt 2: ladder says 65s → clamped to min(65s, 10s) = 10s → reqwest timeout = 10s
// All attempts also subject to the 30s e2e deadline (request.deadline, unchanged).
```

#### Required Changes (driver-internal only)

1. **Ladder constants and selection helper** in `transport_pipeline.rs`:

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

const MAX_TIMEOUT_RETRIES: usize = DATAPLANE_REQUEST_TIMEOUT_LADDER.len();

fn timeout_for_attempt(timeout_retry_count: usize, ladder: &[Duration]) -> Duration {
    ladder[timeout_retry_count.min(ladder.len() - 1)]
}
```

1. **Loop integration** — see the [§4 pseudocode](#pseudocode), which is the canonical reference
   for counter ordering, bounds, and the `attempt_timeout: Duration` calculation. Do not
   duplicate the loop here.

2. **Dispatch chain plumbing**:

   - `AdaptiveTransport::send_with_dispatch(req, attempt_timeout, …)` — new parameter, forwarded
     to the sharded transport (or thin client transport in the future).
   - `ShardedHttpTransport::send(req, attempt_timeout, …)` — new parameter, forwarded to the
     selected `ClientShard`.
   - Hedging path passes `ladder[0]` for the secondary attempt.

3. **Driver-owned dispatch wrapper** in `http_client_factory.rs` and a new sibling module:

   - `HttpClientFactory::build()` returns `Arc<DriverHttpClient>` (driver-internal type).
   - `DriverHttpClient::dispatch_with_timeout(req, attempt_timeout)` builds the
     `reqwest::Request`, applies `RequestBuilder::timeout(attempt_timeout)`, and dispatches.
   - `DriverHttpClient` continues to implement `azure_core::http::HttpClient` (for the small
     number of cross-crate consumers such as `azsdk_arm` style flows), but the driver transport
     layer never goes through the trait.
   - Remove the unconditional `builder.timeout(config.request_timeout)` call at
     `http_client_factory.rs:173`.

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

- The ladder constants, `MAX_TIMEOUT_RETRIES`, and `timeout_for_attempt()` free function live in
  `src/driver/transport/transport_pipeline.rs` alongside the existing transport retry logic.
- The transport pipeline's retry loop is the integration point — it computes `attempt_timeout`
  and threads it through `AdaptiveTransport::send_with_dispatch`.
- The driver-owned dispatch wrapper (`DriverHttpClient` and `dispatch_with_timeout`) lives next
  to `HttpClientFactory` in `src/driver/transport/http_client_factory.rs` (or a new
  `driver_http_client.rs` sibling module).

---

## 8. Adaptive Connection Timeout

### Motivation

Connection timeouts operate at a different level than request timeouts. The `reqwest::Client` sets
`connect_timeout` at construction time — it cannot be overridden per-request. This means the
per-retry escalation ladder pattern used for request timeouts does not apply.

However, a static connection timeout is also suboptimal. A 1s connection timeout is correct for
virtually all cloud/datacenter network environments and keeps the fast path aggressive. But
developers working from machines with higher-latency connections (e.g., VPN, poor Wi-Fi, remote
networks) may see persistent connection failures at 1s.

### Design: Failure-Rate Adaptive Tuning

The connection timeout starts from an aggressive **1s target**, but the effective initial value is
the 1s target **clamped to the configured connect-timeout bounds** from
`ConnectionPoolOptions`. In other words:

- `initial_connect_timeout = clamp(1s, min_connect_timeout(), max_connect_timeout())`
- `escalated_connect_timeout = max_connect_timeout()`

If the configured `max_connect_timeout()` is below 1s, the initial timeout is that configured max,
so the adaptive mechanism does not begin above the allowed upper bound. Likewise,
`min_connect_timeout()` participates as the lower bound for the initial value, so the initial
timeout is always consistent with `ConnectionPoolOptions` validation.

On sustained connection failures observed by `ShardedHttpTransport`, the client transitions once
from the clamped initial timeout to the configured `max_connect_timeout()`. This is a **one-time,
persistent transition** — not a per-retry escalation. In the common default configuration, this
remains a transition from 1s to 5s.

**Reconciliation with existing config**: The existing `ConnectionPoolOptions::max_connect_timeout()`
defaults to 5s and is currently used directly as the `reqwest::Client` connect timeout. This spec
introduces a new internal **initial** connect timeout derived from the 1s aggressive default, but
clamped to `ConnectionPoolOptions::{min,max}_connect_timeout()`. The adaptive mechanism then
transitions from that clamped initial value to the configured `max_connect_timeout()` on sustained
connection failures. The pool's configured bounds remain authoritative throughout — the adaptive
mechanism never starts outside them and never exceeds the configured max.

```text
Normal state: connect_timeout = clamp(1s, min_connect_timeout, max_connect_timeout)
                                  │
                                  ▼
                      ┌───────────────────────┐
                      │ Connection failure    │
                      │ rate exceeds          │──── YES ───▶ connect_timeout = max_connect_timeout
                      │ threshold?            │             (create new HttpClient)
                      └───────────────────────┘
                                  │ NO
                                  ▼
                    Keep clamped initial timeout
```

### Key Properties

- **Start at 1s**: Sufficient for any well-connected cloud environment. Keeps the common case fast.
- **Fall back to 5s**: Only triggered when connection failures are persistent, indicating the
  client is on a slow or unreliable network. >1s is essentially only needed for developers on
  poor connections — production workloads running in Azure should never hit this.
- **Exactly-once transition**: This is not a per-attempt ladder. Once the `ShardedHttpTransport`
  decides to increase the connection timeout for an endpoint, it creates new `HttpClient` instances
  with the higher timeout and marks old shards as unhealthy for immediate reclamation.
- **No automatic fallback to 1s — accepted trade-off**: Once elevated to 5s, the connection
  timeout stays at 5s for the lifetime of the driver process. A restart resets to 1s. This avoids
  the obvious oscillation failure mode (1s→fail→5s→succeed→1s→fail), but it has a known downside:
  for **long-lived processes** (web services, daemons running for weeks), a single transient
  network blip — sufficient to produce >3 consecutive connection failures within a short window —
  permanently leaves the client running with the slower 4-extra-second connect path until the
  process restarts. For typical Azure-resident production workloads this is acceptable (the
  trigger condition is rare and the latency tax is on the cold/connect path only, not on
  established H2 streams). Operators who need an explicit reset mechanism can rely on process
  restarts (already standard practice for credential/cert rotation). A future revision may add
  a long-window decay (e.g., reset elevation if no new connection failures occur for N minutes
  **and** the elevated state has been in place for >M minutes) if telemetry shows this becomes
  a real operational problem.
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

> **Distinct from `ClientShard.consecutive_failures`**: The existing per-shard
> `ClientShard.consecutive_failures: AtomicU32` (`sharded_transport.rs:561`) feeds the H2 health
> sweep's per-shard eviction decision and counts **all** failures (timeouts, 5xx, etc.). The new
> `EndpointShardPool.consecutive_connect_failures` is per-**endpoint** (aggregated across shards)
> and counts only `ErrorKind::Connection` failures. Both counters can advance from the same
> failed attempt without coupling: the per-shard counter governs shard reuse; the per-endpoint
> counter governs the one-time connect-timeout elevation.

### Implementation in `ShardedHttpTransport`

The `ShardedHttpTransport` (introduced in Step 6, PR #3957, in `sharded_transport.rs`) manages a
pool of `HttpClient` shards per endpoint. It tracks per-shard health metrics (consecutive
failures, inflight count, last success time) and runs a periodic background health sweep that
evicts unhealthy or idle shards.

Connection timeout adaptation fits naturally into this model:

1. **Track connection failures per endpoint**: The `ShardedHttpTransport` monitors connection
   failures (`azure_core::error::ErrorKind::Connection`, which the
   `typespec_client_core/src/http/clients/reqwest.rs` adapter maps from `reqwest::Error::is_connect()`
   — the driver does not call `is_connect()` directly because dispatch-side errors arrive already
   wrapped in `azure_core::Error`) for each endpoint independently, aggregated across **all
   shards** for that endpoint. If any shard's connection attempt fails, the per-endpoint
   consecutive failure counter increments. A successful connection on **any** shard for that
   endpoint resets the counter. This ensures that a single consistently failing shard cannot be
   masked by healthy peers — the endpoint-level view captures systemic issues like DNS resolution
   failures or firewall rules that affect all connections to that host.
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

## 9. Cross-SDK Reference

### This Spec (Rust Driver)

| Request Type | Request Timeout Ladder |
|--------------|------------------------|
| Data plane   | 6s, 10s, 65s           |
| Metadata     | 5s, 10s, 20s           |

---

## 10. Open Questions

1. **Metadata timeout ladder scope**: Should the metadata ladder (5s → 10s → 20s) apply to all
   metadata operations uniformly, or should specific metadata operations (e.g., database account
   reads) have their own ladders? For now, a single metadata ladder is proposed. Query plan and
   address refresh are deferred.
