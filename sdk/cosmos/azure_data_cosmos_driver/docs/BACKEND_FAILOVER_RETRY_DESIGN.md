# Backend-driven failover: 403/3 and 403/1008 retry design

## Context

Two HTTP-403 substatus codes signal that the cached topology in the SDK has
diverged from the backend's current routing:

| Substatus | Name | Meaning | Affects |
|-----------|------|---------|---------|
| **3** | `WriteForbidden` | The targeted region is not currently a valid write region for this partition. | Writes only. |
| **1008** | `DatabaseAccountNotFound` | The targeted region no longer owns this database account at all (typical trigger: the region was removed from the account's geo-replication topology). | All ops — reads, writes, queries, feed-range queries, metadata. |

Both can fire repeatedly during a backend-initiated failover or after a
customer-initiated topology change, until either:

1. The SDK refreshes account properties and learns the new region set, AND
2. Retries land on a region the backend currently agrees is valid.

This document describes the chosen retry behavior, the alternatives that
were considered, and the rationale for each axis of the decision.

## Decisions

| Axis | Multi-write accounts | Single-write accounts |
|------|----------------------|-----------------------|
| Retry budget | **120** attempts (dedicated `backend_failover_retry_count`) | **3** attempts (shared generic `failover_retry_count`) |
| Inter-attempt delay | **1000 ms** flat (`BACKEND_FAILOVER_RETRY_INTERVAL`) | **0 ms** (immediate, generic budget) |
| Honor `excluded_regions`? | **Yes** | **No** (recommended) — see below |
| Topology refresh per attempt | Yes (`LocationEffect::RefreshAccountProperties`) | Yes |
| On budget exhaustion | Wrap as **HTTP 503 + substatus `TRANSPORT_GENERATED_503` (20003)**; original status/body/headers preserved on `response_parts` | Same |

These align with the Java v4 SDK (`endpointFailoverRetryIntervalInMs = 1000`,
`endpointFailoverMaxRetryCount = 120`) and the Python SDK
(`EndpointDiscoveryRetryPolicy.Retry_after_in_milliseconds = 1000`,
`Max_retry_attempt_count = 120`).

## Rationale

### Budget = 120 (multi-write)

Multi-write accounts can see 403/3 fire many times in a row as the backend
rotates writes through the current write-region set during a failover. The
3-attempt generic budget — sized for "occasional transient errors" — is
exhausted before the backend's topology change finishes propagating, which
turns a recoverable convergence window into a hard application-visible
failure.

120 attempts × 1 second ≈ 2 minutes of bounded retries gives the backend
enough time to settle while still guaranteeing the operation eventually
bubbles up rather than hanging indefinitely.

### Delay = 1000 ms (multi-write)

Without any delay, the SDK hot-loops 120 cross-region requests in well under
a second — faster than any realistic backend convergence window. This makes
the bug ironically harder to recover from than the .NET / Java / Python
equivalents because the backend never gets time to settle between probes.

The flat 1-second value matches Java and Python and is well-understood in
on-call playbooks. Exponential backoff was rejected because:
- The bound is already small (120 s worst case).
- Exponential growth would push the worst-case wall time into the
  10-minute range, which is worse for callers waiting on a response.
- The signal we are reacting to is a topology change, not a load spike —
  there is nothing to "back off" from in the load-shedding sense.

### Honor `excluded_regions` (multi-write)

`excluded_regions` is the customer's explicit per-operation policy: do not
route this request into the listed regions, full stop. Common reasons:
compliance boundaries, cost controls, latency floors, or a deliberate
manual failover the customer is driving themselves.

Two design options were considered.

#### Option A (chosen): honor `excluded_regions` uniformly

The 120-attempt failover budget filters out excluded regions on every
attempt. If the only remaining valid region is excluded, the operation
exhausts its budget and bubbles up as 503.

- **Pro:** Customer policy is sacred. We never route into a region the
  caller explicitly opted out of, even under pressure.
- **Pro:** Diagnostically honest — every attempted region in the diagnostic
  trail is one the caller could have predicted.
- **Pro:** Avoids cross-region compliance violations during DR drills.
- **Con:** Lower availability when the excluded region is the only healthy
  one. Operation fails with 503 even though the backend would have served it.

#### Option B (rejected for multi-write): bypass `excluded_regions` on backend-signal retries

A previous prototype carried a per-operation `bypass_excluded_regions` flag
that the 1008 handler flipped on, on the theory that 1008 signals "your
view of the topology is stale" and therefore the customer's region list
might also be stale.

- **Pro:** Higher availability — the operation succeeds whenever the backend
  has any healthy region for it.
- **Con:** The customer's `excluded_regions` value is set per-operation at
  call time, not derived from the cached topology. It cannot be "stale" in
  the topology sense — it is a live preference. Bypassing it on 1008
  conflates two different concerns.
- **Con:** Surprises the caller — diagnostics show a request in a region
  they thought was off-limits, with no audit trail of the override.
- **Con:** Breaks DR drills and compliance scenarios where the entire point
  of the exclusion is "do not let any retry path escape it".

### `excluded_regions` for single-write (recommended: do NOT honor)

Single-write accounts have exactly one valid write region at any moment.
When the backend signals 403/3 or 403/1008 to a single-write account, it
is telling the SDK "the write region has moved to a new region you may
not know about yet."

- If the new write region happens to be in the caller's `excluded_regions`,
  honoring the exclusion means the account is **functionally write-dead**
  from this client's perspective until the customer updates their config —
  the SDK has nowhere to retry. There is no second valid write region to
  fall back to.
- The exclusion was likely set against the **old** topology, not the
  new one the backend just announced. Bypassing it on backend-signal
  retries lets writes recover automatically across a failover.
- The risk of crossing into an excluded region is real but bounded:
  single-write means there is only one possible write target, so the
  caller can predict the worst case (the excluded region is the one the
  backend chose) and decide whether to abandon the write.

The recommendation for single-write is therefore the **opposite** of
multi-write: bypass `excluded_regions` for backend-signal retries
(403/3 and 403/1008 specifically — not for generic 503/410 retries).

Note: this single-write carve-out is **NOT** currently implemented in the
codebase. The current behavior honors `excluded_regions` uniformly for
both modes (a deliberate simplification when the `bypass_excluded_regions`
flag was removed during the multi-write redesign). Adopting the
single-write bypass is a follow-up change.

### Wrap exhausted-retry bubble-up as HTTP 503 + `TRANSPORT_GENERATED_503`

When the 120-attempt budget is exhausted, the original 403/3 or 403/1008
reaches the bubble-up path. Surfacing it as-is is misleading: a caller
that retries on 4xx will not retry (the 4xx is meant to signal a
permanent client error), but the SDK has just spent ~2 minutes proving
the condition is transient.

The wrapper produces:
- HTTP status: **503 Service Unavailable**.
- Cosmos sub-status: **`TRANSPORT_GENERATED_503` (20003)** — an existing
  constant the SDK already uses elsewhere to identify a 503 that the SDK
  itself synthesized rather than received from the backend.
- Diagnostics: original wire body and response headers preserved on the
  error's `response_parts`, so substatus-level detail (the actual 403/3 or
  403/1008 the backend last returned) remains inspectable in logs.
- Message string: names the original substatus for log readability.

Reusing `TRANSPORT_GENERATED_503` rather than introducing a new substatus
gives callers a single "SDK-synthesized 503" code to match on. Genuine
backend 503s (substatus from the backend) are untouched.

#### Alternative considered: leave 4xx unwrapped

Pass the original 403/3 or 403/1008 through unchanged after the SDK gives
up.

- **Pro:** Diagnostically transparent — the caller sees exactly what the
  backend last returned.
- **Con:** Misclassifies a transient situation as permanent at the caller's
  retry layer. Application code following the standard "retry 5xx, abandon
  4xx" pattern will abandon a request the SDK knows is recoverable.
- **Con:** The caller has no way to distinguish "the backend really meant
  this 4xx" from "the SDK tried this 120 times and gave up." The original
  status code is preserved in diagnostics either way.

## Summary table

| Substatus | Account mode | Budget | Delay | Honor `excluded_regions`? | On exhaustion |
|-----------|--------------|--------|-------|---------------------------|---------------|
| 403/3 | Multi-write | 120 | 1000 ms | Yes (chosen) | 503 + `TRANSPORT_GENERATED_503` |
| 403/3 | Single-write | 3 (generic) | 0 ms | **No** (recommended; not yet implemented) | 503 + `TRANSPORT_GENERATED_503` |
| 403/1008 | Multi-write | 120 | 1000 ms | Yes (chosen) | 503 + `TRANSPORT_GENERATED_503` |
| 403/1008 | Single-write | 3 (generic) | 0 ms | **No** (recommended; not yet implemented) | 503 + `TRANSPORT_GENERATED_503` |

## Cross-SDK comparison

| | This SDK (multi-write) | Java v4 | Python |
|---|------------------------|---------|--------|
| 403/3 max attempts | 120 | 120 (`endpointFailoverMaxRetryCount`) | 120 (`Max_retry_attempt_count`) |
| 403/3 delay | 1000 ms flat | 0 ms for first 2, then 1000 ms flat (writes); 1000 ms always (reads) | 1000 ms flat |
| 403/1008 delay | 1000 ms flat | 1000 ms flat (reads only; writes not retried by this policy) | 1000 ms flat |
| Exponential backoff or jitter | No | No | No |
| Budget configurable? | No (compile-time const) | Yes (JVM property) | No (class const) |
| Honors customer exclusion list? | Yes | Yes | Yes |
| Wraps exhausted-retry as 503? | Yes (`TRANSPORT_GENERATED_503`) | No (surfaces original) | No (surfaces original) |
