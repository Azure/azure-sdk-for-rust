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
10. [Adaptive Connection Timeout](#10-adaptive-connection-timeout)
11. [Cross-SDK Reference](#11-cross-sdk-reference)
12. [Open Questions](#12-open-questions)

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
  in [§10](#10-adaptive-connection-timeout).
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
| Metadata request (min)      | 100ms   | 100ms | 65s   |
| Metadata request (max)      | 65s     | 100ms | 65s   |

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
  Applied per-request through `Request::set_timeout()`.
- **Connection timeouts**: Adaptively tuned based on observed failure rate (see
  [§10](#10-adaptive-connection-timeout)). Applied at the `HttpClient` level by the
  `ShardedHttpTransport`.

---

## 4. Timeout Ladders

### Data Plane Request Timeout Ladder

| Attempt | Timeout |
|---------|---------|
| 0       | 6s      |
| 1       | 10s     |
| 2       | 65s     |

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

The operation pipeline in `execute_operation_pipeline()` already supports multiple retry attempts
via `FailoverRetry` (default budget: 3 retries) and `SessionRetry` (default budget: 1 retry).
Dynamic timeout escalation is indexed by the **total attempt number** (i.e.,
`failover_retry_count + session_token_retry_count`), so timeouts escalate regardless of the retry
reason.

With `max_failover_retries = 3`, all 3 tiers of the timeout ladder are reachable on the first 3
attempts.

### Timeout Selection per Attempt

Before each attempt, the retry loop selects the appropriate timeouts from the ladders:

```rust
fn timeout_for_attempt(attempt: usize, ladder: &[Duration]) -> Duration {
    ladder[attempt.min(ladder.len() - 1)]
}
```

The selected timeout is applied to the HTTP request for that attempt.

### Hedging and Timeout Ladder Position

When hedging is enabled (see `TRANSPORT_PIPELINE_SPEC.md` §4.2), the hedged attempt runs in a
secondary region as a speculative execution. **Hedged attempts always start at tier 0 of the
timeout ladder**, independent of the primary attempt's ladder position.

Rationale: The hedged attempt targets a different regional endpoint. Network conditions (latency,
connectivity) are independent between regions. A short initial timeout that was insufficient for
the primary region may be perfectly adequate for the secondary region. Starting the hedged attempt
at the same escalated tier as the primary would waste time if the secondary region is responsive.

### Pseudocode

The timeout selection integrates into the 7-stage loop in `execute_operation_pipeline()`. Between
Stage 2 (resolve endpoint) and Stage 3 (build transport request), the attempt timeouts are
computed and passed to `build_transport_request()` which includes them in the `TransportRequest`:

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

loop {
    // ── STAGE 2: Resolve endpoint ──────────────────────────────────
    let routing = resolve_endpoint(operation, &retry_state, &location);

    // ── Compute attempt timeout ────────────────────────────────────
    let attempt = (retry_state.failover_retry_count
        + retry_state.session_token_retry_count) as usize;

    let request_timeout = timeout_for_attempt(
        attempt,
        if is_dataplane {
            DATAPLANE_REQUEST_TIMEOUT_LADDER
        } else {
            METADATA_REQUEST_TIMEOUT_LADDER
        },
    );

    // ── STAGE 3: Build transport request ───────────────────────────
    let transport_request = build_transport_request(
        operation, &routing, &activity_id, execution_context,
        deadline, request_timeout,
    );

    // ── STAGE 4: Execute transport pipeline ────────────────────────
    let result = execute_transport_pipeline(transport_request, ...).await;

    // ── STAGE 5–7: Evaluate, apply effects, act ────────────────────
    match action {
        OperationAction::Complete(result) => return Ok(result),
        OperationAction::FailoverRetry { new_state, .. } => {
            retry_state = new_state;
            continue;
        }
        OperationAction::SessionRetry { new_state } => {
            retry_state = new_state;
            continue;
        }
        OperationAction::Abort { error, .. } => return Err(error),
    }
}
```

---

## 6. Interaction with End-to-End Deadline

The end-to-end operation deadline (from `EndToEndOperationLatencyPolicy`) takes precedence over the
per-attempt timeout ladder. If the remaining time until the deadline is less than the ladder value,
the per-attempt timeout is clamped to the remaining deadline:

```rust
let effective_request_timeout = match deadline {
    Some(d) => {
        let remaining = d.saturating_duration_since(Instant::now());
        request_timeout.min(remaining)
    }
    None => request_timeout,
};
```

If the remaining deadline is zero (or already exceeded), the retry loop exits immediately with a
timeout error — no attempt is made. This is existing behavior and is preserved.

---

## 7. Interaction with ConnectionPoolOptions Bounds

The `ConnectionPoolOptions` min/max bounds still act as clamping limits on the effective timeout.
The ladder value for a given attempt is clamped to the configured bounds:

```rust
let effective_timeout = ladder_value
    .max(connection_pool.min_dataplane_request_timeout())
    .min(connection_pool.max_dataplane_request_timeout());
```

This ensures that:
- If a user sets `max_dataplane_request_timeout` to 10s, the 3rd tier (65s) is clamped to 10s.
- If a user sets `min_dataplane_request_timeout` to 2s, the 1st tier (6s) stays at 6s (already
  above the minimum).
- The existing validation bounds in `ConnectionPoolOptionsBuilder::build()` remain unchanged.

---

## 8. Implementation Details

### Per-Attempt Timeout Delivery via `TransportRequest`

The per-attempt request timeout is delivered to the transport layer as a field on the
`TransportRequest` struct. The operation pipeline computes the timeout for each attempt and
includes it when building the transport request via `build_transport_request()`.

The transport pipeline (`execute_transport_pipeline()`) reads the timeout from the
`TransportRequest` and applies it to the underlying HTTP call.

#### Why `TransportRequest` instead of `Context`

The operation pipeline's 7-stage loop constructs a `TransportRequest` struct for each attempt and
passes it to `execute_transport_pipeline()`. The pipeline `Context` is only created inside the
transport layer — it is not visible in the operation loop. Carrying the timeout on
`TransportRequest` keeps the data flow explicit and avoids coupling to the transport layer's
internal `Context`.

#### Required Changes

**In `typespec_client_core`** — Add an optional per-request timeout to `Request`:

The `Request` struct (`typespec_client_core::http::request::Request`) currently has no timeout
field. Per-request timeouts are needed so that callers (like the Cosmos driver) can override the
client-level timeout on individual requests. The change is minimal and backwards-compatible:

```rust
// In typespec_client_core::http::request::Request
#[derive(Clone)]
pub struct Request {
    pub(crate) url: Url,
    pub(crate) method: Method,
    pub(crate) headers: Headers,
    pub(crate) body: Body,
    pub(crate) timeout: Option<Duration>,  // NEW — per-request timeout override
}

impl Request {
    pub fn new(url: Url, method: Method) -> Self {
        Self {
            url,
            method,
            headers: Headers::new(),
            body: Body::Bytes(Bytes::new()),
            timeout: None,  // No override by default
        }
    }

    /// Returns the per-request timeout override, if set.
    ///
    /// When `Some`, this overrides the client-level timeout for this request only.
    /// When `None`, the client-level timeout applies.
    pub fn timeout(&self) -> Option<Duration> {
        self.timeout
    }

    /// Sets a per-request timeout that overrides the client-level timeout.
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = Some(timeout);
    }
}
```

**In `typespec_client_core`** — Apply the timeout in the `reqwest` `HttpClient` implementation:

The `reqwest::RequestBuilder` already supports per-request timeouts via `.timeout()`. The
`HttpClient` implementation reads the timeout from the `Request` and applies it:

```rust
// In typespec_client_core::http::clients::reqwest
impl HttpClient for ::reqwest::Client {
    async fn execute_request(&self, request: &Request) -> Result<AsyncRawResponse> {
        let url = request.url().clone();
        let method = request.method();
        let mut req = self.request(from_method(method), url.clone());

        // Apply per-request timeout if set (overrides client-level timeout)
        if let Some(timeout) = request.timeout() {
            req = req.timeout(timeout);
        }

        for (name, value) in request.headers().iter() {
            req = req.header(name.as_str(), value.as_str());
        }
        // ... rest unchanged ...
    }
}
```

**In `azure_data_cosmos_driver`**:

1. **Add `request_timeout` to `TransportRequest`** (`components.rs`):

```rust
pub(crate) struct TransportRequest {
    pub method: Method,
    pub endpoint: CosmosEndpoint,
    pub url: Url,
    pub headers: Headers,
    pub body: Option<Bytes>,
    pub auth_context: AuthorizationContext,
    pub execution_context: ExecutionContext,
    pub deadline: Option<Instant>,
    pub request_timeout: Duration,  // NEW
}
```

2. **Set in `build_transport_request()`** (`operation_pipeline.rs`):

```rust
fn build_transport_request(
    operation: &CosmosOperation,
    routing: &RoutingDecision,
    activity_id: &ActivityId,
    execution_context: ExecutionContext,
    deadline: Option<Instant>,
    request_timeout: Duration,  // NEW parameter
) -> azure_core::Result<TransportRequest> {
    // ... existing logic ...
    Ok(TransportRequest {
        // ... existing fields ...
        request_timeout,
    })
}
```

3. **Apply in `execute_transport_pipeline()`** (`transport_pipeline.rs`): Before calling
   `http_client.execute_request()`, set the per-request timeout on the `azure_core::http::Request`:

```rust
// In execute_transport_pipeline(), after building the azure_core Request:
request.set_timeout(transport_request.request_timeout);
```

4. **Integration with `ShardedHttpTransport`**: The sharded transport
   (`sharded_transport.rs`) sends requests via per-endpoint shard pools. The transport pipeline
   already applies `per_request_timeout` via `azure_core::sleep()` racing the HTTP future
   (see `transport_pipeline.rs`). The `request_timeout` from the escalation ladder feeds into
   this existing per-request timeout mechanism.

### Data Structures

```rust
/// Fixed escalation ladder for timeout values across retry attempts.
///
/// Each entry corresponds to an attempt index (0 = initial, 1 = first retry, etc.).
/// If the attempt index exceeds the ladder length, the last value is used.
struct TimeoutLadder {
    steps: &'static [Duration],
}

impl TimeoutLadder {
    /// Returns the timeout for the given attempt, saturating at the last step.
    fn timeout_for_attempt(&self, attempt: usize) -> Duration {
        self.steps[attempt.min(self.steps.len() - 1)]
    }
}
```

### Diagnostic Recording

The effective per-attempt timeout values are recorded in `DiagnosticsContext` for observability.
This helps users understand why a retry succeeded when the initial attempt failed (e.g., "attempt 0
timed out at 6s, attempt 1 succeeded with a 10s timeout").

For each request attempt, the effective request timeout is recorded:

```rust
/// Timeout information recorded per attempt in diagnostics.
pub(crate) struct AttemptTimeoutDiagnostics {
    /// The effective request timeout used for this attempt (after clamping).
    pub request_timeout: Duration,
}
```

This is recorded via `DiagnosticsContextBuilder::update_request()` alongside existing per-request
metadata (charge, activity ID, session token). The timeout value recorded is the **effective**
value after clamping by both `ConnectionPoolOptions` bounds and the end-to-end deadline.

### Where It Lives

- `TimeoutLadder` and the default ladder constants should be defined in the driver crate, likely
  in a new module `src/driver/timeouts.rs` or inline in `src/driver/pipeline/components.rs`.
- The retry loop in `execute_operation_pipeline()` (`operation_pipeline.rs`) is the integration
  point — it computes the request timeout per attempt and passes it to `build_transport_request()`.
- `AttemptTimeoutDiagnostics` lives alongside the existing diagnostics types in
  `src/diagnostics/`.

---

## 10. Adaptive Connection Timeout

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
  of the driver. A restart resets to 1s. This keeps the logic simple and avoids oscillation.

### Implementation in `ShardedHttpTransport`

The `ShardedHttpTransport` (`sharded_transport.rs`) manages a pool of `HttpClient` shards per
endpoint. It tracks per-shard health metrics (consecutive failures, inflight count, last success
time) and runs a periodic background health sweep that evicts unhealthy or idle shards.

Connection timeout adaptation fits naturally into this model:

1. **Track connection failures per endpoint**: The `ShardedHttpTransport` monitors connection
   failures (errors where `is_connect()` returns `true`) for each endpoint independently. Since
   the sharded transport already manages shard pools per endpoint, per-endpoint tracking is the
   natural granularity — a connectivity issue in one region does not penalize healthy regions.
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
4. **No `azure_core` changes needed**: This is entirely internal to the `ShardedHttpTransport`.
   The `HttpClientFactory::build()` method reads `connect_timeout` from `ConnectionPoolOptions`
   (via `max_connect_timeout()`). To support adaptive connection timeouts, the factory accepts
   an overridden connect timeout or the `ShardedHttpTransport` updates its `base_client_config`
   before creating new shards.

```rust
// In ShardedHttpTransport, per-endpoint health tracking:
if endpoint_state.consecutive_connect_failures > 3 {
    endpoint_state.effective_connect_timeout = Duration::from_secs(5);
    // Mark existing shards as unhealthy for immediate reclamation.
    endpoint_state.mark_old_shards_unhealthy();
    // New shards will be created with the elevated timeout.
}
```

---

## 11. Cross-SDK Reference

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

## 12. Open Questions

1. **Metadata timeout ladder scope**: Should the metadata ladder (5s → 10s → 20s) apply to all
   metadata operations uniformly, or should specific metadata operations (e.g., database account
   reads) have their own ladders? For now, a single metadata ladder is proposed. Query plan and
   address refresh are deferred.
