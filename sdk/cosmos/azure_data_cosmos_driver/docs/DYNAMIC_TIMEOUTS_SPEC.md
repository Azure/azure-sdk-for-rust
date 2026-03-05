# Dynamic Timeout Escalation Spec for `azure_data_cosmos_driver`

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
4. **Increase transport retry count**: Raise the maximum transport retry count from 1 to 2
   (3 total attempts) so that all tiers of the escalation ladder are reachable. This mirrors the
   connection-error retry behavior already present in the SDK.

### Non-Goals

- **User-configurable ladders**: The escalation ladder is a fixed internal default. Users cannot
  override the step durations. The existing `ConnectionPoolOptions` min/max bounds still act as
  clamping limits (see [§7](#7-interaction-with-connectionpooloptions-bounds)).
- **Query plan timeout escalation**: Deferred until query plan execution is implemented.
- **Address refresh timeout escalation**: Deferred until direct mode is implemented.

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

The current retry loop in `CosmosDriver::execute_operation_internal` uses a fixed
`MAX_TRANSPORT_RETRIES = 1` (2 total attempts). On transport failure, if the operation is
idempotent or the request was definitely not sent, the request is retried once with the same
timeout.

```rust
const MAX_TRANSPORT_RETRIES: usize = 1;

fn should_retry_transport_failure(
    attempt: usize,
    max_transport_retries: usize,
    is_idempotent: bool,
    request_sent: TransportRequestSentStatus,
) -> bool {
    attempt < max_transport_retries && (is_idempotent || request_sent.definitely_not_sent())
}
```

---

## 3. Design: Dynamic Timeout Escalation

### Core Concept

Instead of using a single fixed timeout for all attempts, the driver uses an **escalation ladder**:
a fixed sequence of increasing timeout durations indexed by the attempt number. On each retry, the
next (longer) timeout in the ladder is used. The ladder values are not user-configurable — they are
internal defaults chosen to balance latency and reliability.

### Separate Ladders

Two independent ladders govern different timeout types:

1. **Request timeout ladder**: Controls how long to wait for a response after the connection is
   established. Applied to the HTTP request timeout.
2. **Connection timeout ladder**: Controls how long to wait for a TCP connection to be established.
   Applied to the HTTP connection timeout.

Each ladder has separate default values for data plane vs. metadata request types.

---

## 4. Timeout Ladders

### Data Plane Request Timeout Ladder

| Attempt | Timeout |
|---------|---------|
| 0       | 6s      |
| 1       | 10s     |
| 2       | 65s     |

### Data Plane Connection Timeout Ladder

| Attempt | Timeout |
|---------|---------|
| 0       | 1s      |
| 1       | 3s      |
| 2       | 5s      |

### Metadata Request Timeout Ladder

Following the Java SDK pattern for `DatabaseAccount` metadata calls:

| Attempt | Timeout |
|---------|---------|
| 0       | 5s      |
| 1       | 10s     |
| 2       | 20s     |

### Metadata Connection Timeout Ladder

Metadata requests use the same connection timeout ladder as data plane requests:

| Attempt | Timeout |
|---------|---------|
| 0       | 1s      |
| 1       | 3s      |
| 2       | 5s      |

### Ladder Behavior

- If `attempt >= ladder.len()`, use the last value in the ladder (the ladder "saturates" at its
  final tier).
- The ladder values are **not configurable** by users. They are internal constants.

---

## 5. Integration with Retry Loop

### Retry Count Change

The maximum transport retry count is increased from 1 to 2:

```rust
// Before
const MAX_TRANSPORT_RETRIES: usize = 1;

// After
const MAX_TRANSPORT_RETRIES: usize = 2;
```

This means 3 total attempts (initial + 2 retries), matching the 3 tiers in the timeout ladders.

### Timeout Selection per Attempt

Before each attempt, the retry loop selects the appropriate timeouts from the ladders:

```rust
fn timeout_for_attempt(attempt: usize, ladder: &[Duration]) -> Duration {
    ladder[attempt.min(ladder.len() - 1)]
}
```

The selected timeouts are applied to the HTTP request for that attempt. Both the connection timeout
and the request timeout are set per-attempt.

### Pseudocode

```rust
const DATAPLANE_REQUEST_TIMEOUT_LADDER: &[Duration] = &[
    Duration::from_secs(6),
    Duration::from_secs(10),
    Duration::from_secs(65),
];

const DATAPLANE_CONNECTION_TIMEOUT_LADDER: &[Duration] = &[
    Duration::from_secs(1),
    Duration::from_secs(3),
    Duration::from_secs(5),
];

const METADATA_REQUEST_TIMEOUT_LADDER: &[Duration] = &[
    Duration::from_secs(5),
    Duration::from_secs(10),
    Duration::from_secs(20),
];

const METADATA_CONNECTION_TIMEOUT_LADDER: &[Duration] = &[
    Duration::from_secs(1),
    Duration::from_secs(3),
    Duration::from_secs(5),
];

const MAX_TRANSPORT_RETRIES: usize = 2;

loop {
    let request_timeout = timeout_for_attempt(
        attempt,
        if is_dataplane {
            DATAPLANE_REQUEST_TIMEOUT_LADDER
        } else {
            METADATA_REQUEST_TIMEOUT_LADDER
        },
    );
    let connection_timeout = timeout_for_attempt(
        attempt,
        if is_dataplane {
            DATAPLANE_CONNECTION_TIMEOUT_LADDER
        } else {
            METADATA_CONNECTION_TIMEOUT_LADDER
        },
    );

    // Apply timeouts to the request/transport for this attempt
    // ...

    match result {
        Ok(response) => return Ok(response),
        Err(e) => {
            if should_retry_transport_failure(attempt, MAX_TRANSPORT_RETRIES, ...) {
                attempt += 1;
                continue;
            }
            return Err(e);
        }
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
- If a user sets `min_dataplane_request_timeout` to 2s, the 1st tier connection timeout (1s) is
  raised to 2s.
- The existing validation bounds in `ConnectionPoolOptionsBuilder::build()` remain unchanged.

---

## 8. Implementation Details

### Per-Attempt Connection Timeout

The current architecture creates `HttpClient` instances with a fixed `connect_timeout` set at
creation time (via `HttpClientConfig`). To support per-attempt connection timeouts, one of the
following approaches must be used:

#### Option A: Set Timeout on the Request

Extend the per-request context or `Request` to carry both a connection timeout and a request
timeout. The `HttpClient` / transport layer reads these values per-request and applies them.
This requires changes to `azure_core::http::Request` or the pipeline `Context`.

#### Option B: Create Per-Attempt HttpClients

Create a new `HttpClient` for each attempt with the appropriate connection timeout. This is
wasteful (new TLS handshake, new connection pool) and is not recommended.

#### Option C: Use Tokio Timeout Wrapper

Wrap the connection phase in a `tokio::time::timeout()` at the transport layer, independently of
the `HttpClient`'s built-in connection timeout. The `HttpClient`'s connection timeout is set to the
maximum ladder value, and the per-attempt timeout is enforced externally.

**Recommended**: **Option A** is the cleanest approach. The `ShardedHttpTransport` design
(see `TRANSPORT_PIPELINE_SPEC.md` §6) already plans to accept per-request configuration including
timeouts. Until that transport is implemented, the current pipeline can set per-request timeouts
via the `Request` timeout mechanism if available, or via the `Context`.

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

/// Timeout configuration for a single attempt, combining connection and request timeouts.
struct AttemptTimeouts {
    connection_timeout: Duration,
    request_timeout: Duration,
}
```

### Where It Lives

- `TimeoutLadder` and the default ladder constants should be defined in the driver crate, likely in
  a new module `src/driver/timeouts.rs` or inline in `src/driver/cosmos_driver.rs`.
- The retry loop in `CosmosDriver::execute_operation_internal` is the integration point.

---

## 9. Cross-SDK Reference

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

| Request Type | Request Timeout Ladder | Connection Timeout Ladder |
|--------------|------------------------|---------------------------|
| Data plane   | 6s, 10s, 65s           | 1s, 3s, 5s               |
| Metadata     | 5s, 10s, 20s           | 1s, 3s, 5s               |

**Differences from Java SDK:**
- Rust escalates connection timeouts per attempt; Java uses a flat 45s/5s.
- Rust data plane uses 6s/10s/65s; Java direct mode uses a flat 5s.
- Java gateway "Other HTTP calls" use a flat 60s; Rust data plane starts lower (6s).

---

## 10. Open Questions

1. **Per-attempt connection timeout mechanism**: Which approach (Option A/B/C in §8) should be used
   to apply per-attempt connection timeouts? This depends on the timeline for
   `ShardedHttpTransport` (see `TRANSPORT_PIPELINE_SPEC.md` §6).

2. **Metadata timeout ladder scope**: Should the metadata ladder (5s → 10s → 20s) apply to all
   metadata operations uniformly, or should specific metadata operations (e.g., database account
   reads) have their own ladders? For now, a single metadata ladder is proposed. Query plan and
   address refresh are deferred.

3. **Interaction with hedging**: When hedging is enabled (see `TRANSPORT_PIPELINE_SPEC.md` §4.2),
   the hedged attempt runs in a secondary region. Should the hedged attempt use the same ladder
   position as the primary, or always start at tier 0? The hedged attempt is architecturally
   independent, so starting at tier 0 is likely correct.

4. **Diagnostic reporting**: Should the per-attempt timeout values be recorded in
   `DiagnosticsContext` for observability? This would help users understand why a retry succeeded
   when the initial attempt failed.
