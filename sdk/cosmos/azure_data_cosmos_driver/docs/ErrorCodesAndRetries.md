# Cosmos DB Rust Driver — Retry Mechanisms and Error Code Handling

This document describes the target retry behavior for the Azure Cosmos DB Rust driver (`azure_data_cosmos_driver`). It serves as the authoritative specification for how the driver handles errors, retries, and cross-region failover.

## Design Philosophy

The Rust driver retries writes by default for retryable status codes. This is safe because Cosmos DB's write APIs are designed to be idempotent when used correctly:

- **503 (Service Unavailable)**: Cosmos DB intentionally returns 503 when a write was **not processed** — it is always safe to retry.
- **5xx / 408**: Write retries are safe for CRUD operations because customers can (and should) use ETag preconditions (`If-Match`) to guarantee idempotency on replace and upsert. Create operations are inherently idempotent (a duplicate yields 409 Conflict). Delete operations are inherently idempotent (a duplicate yields 404 Not Found).
- **Stored Procedure execution**: Stored procedures are **not idempotent** and must **not** be retried on timeout or server error. The driver disables write retries for stored procedure operations.

### Idempotency Requirements

Write retries are not strictly idempotent — the initial attempt and a retry may return different status codes (e.g., create returns 201 on success, then 409 on retry). What makes retries "safe" is that the final state of the resource is the same regardless of how many times the operation is executed, and the non-2xx status codes are deterministic signals the application can handle.

For replace and upsert operations, the driver **always retries** regardless of whether an ETag precondition is provided. If the application developer has concerns about idempotency or wants optimistic locking, ETag preconditions (`If-Match` headers) are the appropriate mitigation. Without ETags, there is no concurrency control — concurrent writers or retried writes can silently overwrite each other.

| Operation | Retried? | Initial attempt | On retry (duplicate) | App must handle |
|-----------|----------|-----------------|----------------------|-----------------|
| Create | Yes | 201 Created | 409 Conflict | 409 |
| Delete | Yes | 204 No Content | 404 Not Found | 404 |
| Replace / Upsert (with ETag) | Yes | 200 OK | 412 Precondition Failed (if concurrent update) | 412 |
| Replace / Upsert (without ETag) | Yes | 200 OK | 200 OK (silent overwrite — no concurrency control) | — |
| Patch | Yes | 200 OK | 200 OK (operation-level idempotency) | — |
| Stored Procedure | **No** | Varies | Undefined (side effects may repeat) | N/A — retries disabled |

## Status Code Handling

### Non-Retryable (Abort Immediately)

| Status | Substatus | Meaning | Action |
|--------|-----------|---------|--------|
| 400 | — | Bad Request | Abort |
| 401 | — | Unauthorized | Abort |
| 404 | 0 | Not Found | Abort |
| 409 | — | Conflict | Abort |
| 412 | — | Precondition Failed | Abort |

These are deterministic client errors. No retry will change the outcome.

### 449 — Retry With

| Operation | Action | Budget |
|-----------|--------|--------|
| Any | SDK-owned retry | TBD |

449 indicates the request must be retried with a modified configuration (e.g., after a collection recreate or partition split). Gateway V1 can handle 449 retries internally, but the Rust SDK always disables Gateway-side 449 retries and owns them in the SDK. This is required for Gateway V2, where all 449 retries must be handled by the SDK.

### 403 — Forbidden

| Substatus | Meaning | Action | Budget (multi-write) | Budget (single-write) |
|-----------|---------|--------|----------------------|-----------------------|
| 3 | `WriteForbidden` — region is not currently a valid write region for this partition (writes only) | Refresh account topology + cross-region failover retry | **120** attempts × **1000 ms** delay (dedicated `backend_failover_retry_count`) | 3 attempts × 0 ms delay (shared generic budget) |
| 1008 | `DatabaseAccountNotFound` — region no longer owns this account (all op types, including reads, writes, queries, feed-range queries, metadata) | Refresh account topology + cross-region failover retry | **120** attempts × **1000 ms** delay (dedicated `backend_failover_retry_count`) | 3 attempts × 0 ms delay (shared generic budget) |
| Other | Permission denied | Abort | — | — |

Both 403/3 and 403/1008 signal that the cached topology in the SDK has diverged from the backend's current routing — typically during a backend-initiated failover or a customer-initiated topology change. On each retry the driver requests `LocationEffect::RefreshAccountProperties` so the next attempt routes against the freshly learned region set. The metadata refresh itself is rate-limited (at most one network fetch per `refresh_interval`, default 5 s) and is independent of the caller's `excluded_regions` — the GetDatabaseAccount probe iterates the global endpoint and the cached `readable_locations` regardless of the operation-level exclusion list, because excluding a region from data-plane routing should not blind the SDK to topology changes happening in that region.

**Why a dedicated 120-attempt × 1s budget on multi-write?** The 3-attempt generic failover budget is exhausted long before the backend's topology change finishes propagating, turning a recoverable convergence window into a hard application-visible failure. 120 attempts × 1 s ≈ 2 minutes of bounded retries gives the backend time to settle while still guaranteeing eventual bubble-up. The 1000 ms flat interval (no exponential backoff) matches Java's `endpointFailoverRetryIntervalInMs` and Python's `EndpointDiscoveryRetryPolicy.Retry_after_in_milliseconds` defaults. Without the delay, the SDK hot-loops 120 cross-region requests in well under a second — faster than the backend convergence window it is reacting to. Exponential backoff was rejected because the bound is already short, the signal is a topology change rather than a load spike, and growth would push worst-case wall time into the 10-minute range.

**Exhaustion behavior — surface the original status unchanged.** When the budget is exhausted, the bubble-up reports the original 403/3 or 403/1008 verbatim. The SDK deliberately does NOT rewrite these as 503: the original substatus is the most diagnostically useful signal about why the operation failed (the topology never converged from this client's vantage point), and re-classifying it as 503 would obscure that root cause in logs and trip caller-side retry policies that treat 5xx and 4xx differently.

#### `excluded_regions` interaction

The dedicated backend-failover budget filters out `excluded_regions` on every attempt. If the only remaining valid region is excluded, the operation exhausts its budget and surfaces the original 403/3 or 403/1008 to the caller.

Honoring `excluded_regions` uniformly is a deliberate choice. Customer policy is sacred — the SDK does not route into a region the caller explicitly opted out of, even under pressure (compliance boundaries, cost controls, latency floors, deliberate manual failover). The trade-off is lower availability when the excluded region is the only healthy one. An earlier prototype carried a per-operation `bypass_excluded_regions` flag on the theory that 1008 means "your view of the topology is stale" and the region list might also be stale, but the customer's `excluded_regions` value is set per-operation at call time — it is a live preference, not derived from cached topology, so conflating those two concerns surprises callers and breaks DR drills.

### 404/1002 — Read Session Not Available

| Account Type | Action | Budget |
|--------------|--------|--------|
| Single-write | Session retry to write region (hub region) | 2 attempts |
| Multi-write | Session retry, advance through preferred endpoints | `preferred_endpoints.len()` attempts |

The session token is preserved on all retry attempts — it is never cleared to allow stale reads, as that would violate the customer's chosen consistency guarantees. When all session retries are exhausted, the 404/1002 error is surfaced to the caller.

### 408 — Request Timeout

| Operation | Action | Budget |
|-----------|--------|--------|
| Reads | Cross-region failover retry | 3 failover attempts |
| Writes (except stored procedures) | **Cross-region failover retry** | 3 failover attempts |
| Stored Procedure writes | **Abort** | — |

408 indicates a server-side or client-side timeout. The Rust driver retries writes on 408 because:

- CRUD write operations are idempotent when customers use ETag preconditions (see [Idempotency Requirements](#idempotency-requirements) above).
- 412 (Precondition Failed) prevents silent overwrites if a retried write races with a concurrent update.
- Stored procedure execution is excluded from write retries (not idempotent).

For single-write accounts, retry cycles through the available endpoint(s). For multi-write accounts, retry advances to the next preferred write region.

### 410 — Gone

| Operation | Action | Budget |
|-----------|--------|--------|
| Reads | Cross-region failover retry | 3 failover attempts |
| Writes (all) | **Cross-region failover retry** | 3 failover attempts |

410 indicates the partition has moved or is undergoing a split/merge. All operations retry, regardless of idempotency.

### 429 — Too Many Requests (Throttling)

| Substatus | Action | Budget |
|-----------|--------|--------|
| — (standard) | Local retry with backoff | 9 attempts / 30s total |
| 3092 (global throttle) | Cross-region failover retry | 3 failover attempts |

Standard 429 is handled entirely within the transport pipeline — the operation pipeline never sees it. The transport layer respects `x-ms-retry-after-ms` headers and falls back to exponential backoff (5ms base, 5s cap per attempt).

429/3092 indicates a global/partition-level throttle that cannot be resolved locally. It is escalated to the operation pipeline and treated identically to 503 (cross-region failover).

**No cross-region retry for standard 429.** Throttling is account-wide; moving to another region would not help.

### 5xx — Server Errors (500, 502, 503, 504)

| Operation | Action | Budget |
|-----------|--------|--------|
| Reads | Cross-region failover retry | 3 failover attempts |
| Writes (all) | **Cross-region failover retry** | 3 failover attempts |

All 5xx errors are retried uniformly. 503 is the canonical "safe to retry" signal from Cosmos DB — when the service intentionally returns 503, it guarantees the write was not processed. All other 5xx codes (500, 502, 504) are retried identically because CRUD write operations are idempotent when customers use ETag preconditions (see [Idempotency Requirements](#idempotency-requirements) above). 502/504 may be raised by intermediate proxies, but ETag preconditions (412 on stale ETag) prevent silent overwrites on retry. Stored procedure execution is excluded from write retries.

**Endpoint marking**: Individual 5xx failures do not mark endpoints as unavailable. Endpoint unavailability is driven by PPCB's per-partition failure thresholds (see [Per-Partition Circuit Breaker](#per-partition-circuit-breaker-ppcb)). Each failure increments the partition's failure counter; only when the configured threshold is crossed does routing shift to the next preferred region.

**This is the key divergence from other SDKs**: Python gates write retries behind `retry_write`; Java/.NET only retry for multi-write accounts. The Rust driver always retries.

**Note on in-region retries**: Other SDKs (Python, .NET) typically perform 1 local/in-region retry with a delay for 503/500 before escalating to cross-region failover. The Rust driver currently skips this step and goes straight to cross-region failover on the first failure. This may be worth revisiting — a single in-region retry could resolve transient issues without the latency cost of switching regions.

### Transport Errors (Connection Failures)

| Sent Status | Operation | Action | Budget |
|-------------|-----------|--------|--------|
| **Not sent** (request never left client) | Any | Cross-region failover retry | 3 failover attempts |
| **Sent** or unknown | Reads | Cross-region failover retry | 3 failover attempts |
| **Sent** or unknown | Writes (all) | **Cross-region failover retry** | 3 failover attempts |

When the request was definitely not sent (connection refused, DNS failure, TLS error), the endpoint itself is unreachable. The driver marks the endpoint as unavailable (affecting all partitions on it) and records a partition-level failure for PPCB tracking, then retries on the next preferred region.

When the request was possibly sent, the endpoint is clearly reachable — only partition-level marking is applied (via PPCB). The endpoint is not marked unavailable since other partitions on it are unaffected.

For connectivity errors (connection refused, I/O errors), the transport layer performs 1 local retry on a different TCP shard to the same endpoint before escalating to the operation pipeline for cross-region failover.

**Note**: The Rust driver retries non-idempotent writes even when the request may have been sent, because CRUD write operations are idempotent when customers use ETag preconditions (see [Idempotency Requirements](#idempotency-requirements) above). Stored procedure execution is excluded.

### Deadline Exceeded (Client-Side Timeout)

| Operation | Action | Budget |
|-----------|--------|--------|
| Any | **Abort** — synthesize 408 / `CLIENT_OPERATION_TIMEOUT` | — |

When the client's end-to-end deadline is exceeded, no retry is attempted. The operation has already consumed its time budget.

## Cross-Region Failover Behavior

### Single-Write Accounts

For retryable errors (5xx, 408, 410, transport errors):

- **Writes**: There is only one write endpoint, so retries hit the same endpoint unless PPAF is enabled (which allows routing to read endpoints for write-region discovery).
- **Reads**: The driver cycles through preferred read regions on each retry attempt.
- Budget: 3 failover attempts total.

### Multi-Write Accounts

For retryable errors on writes:

1. First attempt goes to the current preferred write endpoint.
2. On failure, advance to the next preferred write region (cross-region failover).
3. Continue cycling through `write_regions` endpoints.
4. Budget: 3 failover attempts total (or `preferred_endpoints.len()` for session retries).

Cross-region retry is the natural behavior for multi-write accounts since any write region can accept writes.

### Endpoint Exhaustion Fallback

When all regional endpoints are excluded or unavailable, the driver falls back differently for data-plane vs metadata operations:

- **Data-plane operations**: Fall back to the hub (write region) endpoint for single-write accounts, or the first entry in `preferred_write_endpoints` for multi-write accounts. The global account endpoint is **never** used for data-plane traffic.
- **Metadata operations** (e.g., account topology discovery): Fall back to the global account endpoint.

## Per-Partition Automatic Failover (PPAF)

PPAF is an **opt-in** feature for **single-master write accounts only**. When enabled (via server account flag `enable_per_partition_failover_behavior`):

- Partition-level routing overrides are recorded on **successful write confirmation** — not on failure.
- If a write succeeds on a non-write region during retry, that region is recorded as the partition's current write region.
- PPAF entries do **not** participate in probe-based failback; they are updated only by success-time discovery.

**With the Rust driver's "always retry writes" stance, PPAF primarily adds the partition-level routing intelligence** — the retry itself already happens regardless. PPAF makes the routing *smarter* by remembering which region last successfully served a given partition, and by providing further availability through processing writes in other read regions.

## Per-Partition Circuit Breaker (PPCB)

PPCB is an **opt-out** feature (enabled by default) that provides partition-level health tracking and routing:

| Account Type | Reads | Writes |
|--------------|-------|--------|
| Single-write | ✅ PPCB-managed | ❌ Not PPCB-managed (PPAF handles writes) |
| Multi-write | ✅ PPCB-managed | ✅ PPCB-managed |

### Behavior

- Tracks per-partition failure counts (`read_failure_count`, `write_failure_count`) with timestamps.
- **Endpoints are NOT marked unavailable on individual failures.** Unavailability is only triggered when a partition's failure count crosses the configured threshold (e.g., 10 consecutive failures for reads). Individual retry failures during an operation do not affect routing for other operations.
- When the threshold is crossed, the circuit "trips" and routes subsequent requests for that partition to the next preferred region.
- **Recovery (probes are for marking *available* only)**: Probes do not detect failures or mark endpoints unavailable — they only restore previously-tripped partitions. After `partition_unavailability_duration`, a single probe request is sent. Success removes the entry; failure resets the timer.
- When PPCB is managing an endpoint, `MarkEndpointUnavailable` effects are **suppressed** — PPCB owns the routing decision.

### PPCB vs Endpoint-Level Marking

Without PPCB, the driver marks entire endpoints as unavailable when errors occur. With PPCB, the granularity improves to per-partition:

- Individual partition failures don't poison the entire endpoint.
- Other partitions on the same endpoint continue to be served normally.
- Recovery is also per-partition rather than endpoint-wide.

## Retry Budget Summary

| Layer | Budget | Scope |
|-------|--------|-------|
| Transport (429) | 9 attempts or 30s | Per-request, local only |
| Operation failover (generic — 5xx, 408, 410, transport) | 3 attempts | Per-operation, cross-region |
| Backend-failover (403/3, 403/1008) — multi-write | **120 attempts × 1000 ms** | Per-operation, cross-region |
| Backend-failover (403/3, 403/1008) — single-write | 3 attempts × 0 ms (generic) | Per-operation, cross-region |
| Session retry (404/1002) | 2 (single-write) or `preferred_endpoints.len()` (multi-write) | Per-operation |

## Comparison with Other SDKs

| Behavior | Python | Java | .NET | **Rust (Target)** |
|----------|--------|------|------|-------------------|
| 503 write retry | Always (no gate) | Multi-write only | Multi-write only | **Always** |
| 500 write retry | Only with `retry_write` | No | No | **Always** |
| 408 write retry | Only with `retry_write` | No | No | **Always** |
| 502/504 write retry | Only with `retry_write` | No | No | **Always** |
| Non-idempotent write retry | Gated by `retry_write` | Gated by multi-write | Gated by multi-write | **Always (no gate)** |
| Transport sent + write | Abort | Abort | Abort | **Retry** |
| PPAF | Yes (single-master) | Yes | Yes | **Yes** |
| PPCB | Yes | Yes | Yes | **Yes** |

The Rust driver is intentionally more aggressive about retrying writes. This is a deliberate design choice for maximum availability, leveraging Cosmos DB's conflict detection and the use of Etags as the safety net for duplicates and idempotency concerns.

