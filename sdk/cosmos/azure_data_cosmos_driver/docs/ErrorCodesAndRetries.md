# Cosmos DB Rust Driver — Retry Mechanisms and Error Code Handling

This document describes the implemented retry behavior for the Azure Cosmos DB Rust driver (`azure_data_cosmos_driver`). It serves as the authoritative specification for how the driver handles errors, retries, and cross-region failover.

## Design Philosophy

The Rust driver retries writes by default for retryable status codes. This is safe because Cosmos DB's write APIs are designed to be idempotent when used correctly:

- **503 (Service Unavailable)**: Cosmos DB intentionally returns 503 when a write was **not processed** — it is always safe to retry.
- **5xx / 408**: Write retries are safe for CRUD operations because customers can (and should) use ETag preconditions (`If-Match`) to guarantee idempotency on replace and upsert. Create operations are inherently idempotent (a duplicate yields 409 Conflict). Delete operations are inherently idempotent (a duplicate yields 404 Not Found).
- **Stored Procedure execution**: A stored procedure body is opaque to the driver, so a re-run can repeat arbitrary mutations with no way to detect the duplicate. `OperationType::Execute` is therefore **not** retried when the outcome is ambiguous. It *is* still retried on statuses that prove the backend did not run it — see [Stored procedure retries](#stored-procedure-retries).

### Stored procedure retries

Stored procedure execution is the only data-plane operation that is gated. The
dividing line is whether the response proves the procedure did not run, not
whether the operation is idempotent.

| Outcome                                          | Stored procedure  | Why                                    |
| ------------------------------------------------ | ----------------- | -------------------------------------- |
| Transport error, request definitely **not sent** | Retry             | Never reached the backend              |
| Transport error, **sent** or unknown             | **Abort**         | May have run to completion             |
| 408 Request Timeout                              | **Abort**         | Outcome unknown                        |
| 500 / 502 / 504                                  | **Abort**         | Outcome unknown                        |
| 503 Service Unavailable                          | Retry             | Returned only for unprocessed requests |
| 410 Gone                                         | Retry             | Routing rejection, before execution    |
| 429 / 429-3092                                   | Retry             | Throttled, before execution            |
| 449 Retry With                                   | Retry (in-region) | Request never completed                |
| 403/3, 403/1008                                  | Retry             | Rejected on topology, before execution |

Enforced by `is_unsafe_retry_after_possible_execution` in
`src/driver/pipeline/retry_evaluation.rs`, which delegates the operation-type
decision to `CosmosOperation::allows_ambiguous_outcome_retry` in
`src/models/cosmos_operation.rs`.

### Idempotency Requirements

Write retries are not strictly idempotent — the initial attempt and a retry may return different status codes (e.g., create returns 201 on success, then 409 on retry). What makes retries "safe" is that the final state of the resource is the same regardless of how many times the operation is executed, and the non-2xx status codes are deterministic signals the application can handle.

For replace and upsert operations, the driver **always retries** regardless of whether an ETag precondition is provided. If the application developer has concerns about idempotency or wants optimistic locking, ETag preconditions (`If-Match` headers) are the appropriate mitigation. Without ETags, there is no concurrency control — concurrent writers or retried writes can silently overwrite each other.

| Operation                       | Retried?                            | Initial attempt | On retry (duplicate)                                            | App must handle |
| ------------------------------- | ----------------------------------- | --------------- | --------------------------------------------------------------- | --------------- |
| Create                          | Yes                                 | 201 Created     | 409 Conflict                                                    | 409             |
| Delete                          | Yes                                 | 204 No Content  | 404 Not Found                                                   | 404             |
| Replace / Upsert (with ETag)    | Yes                                 | 200 OK          | 412 Precondition Failed (if concurrent update)                  | 412             |
| Replace / Upsert (without ETag) | Yes                                 | 200 OK          | 200 OK (silent overwrite — no concurrency control)              | —               |
| Patch                           | Yes                                 | 200 OK          | 200 OK (operation-level idempotency)                            | —               |
| Stored Procedure                | **Only when provably not executed** | Varies          | N/A — see [Stored procedure retries](#stored-procedure-retries) | N/A             |

## Status Code Handling

### Non-Retryable (Abort Immediately)

| Status | Substatus | Meaning             | Action |
| ------ | --------- | ------------------- | ------ |
| 400    | —         | Bad Request         | Abort  |
| 401    | —         | Unauthorized        | Abort  |
| 404    | 0         | Not Found           | Abort  |
| 409    | —         | Conflict            | Abort  |
| 412    | —         | Precondition Failed | Abort  |

These are deterministic client errors. No retry will change the outcome.

### 449 — Retry With

| Operation | Action                    | Budget                                          |
| --------- | ------------------------- | ----------------------------------------------- |
| Any       | Same-region delayed retry | 30s cumulative wait; 1s maximum per retry delay |

449 indicates a transient server-side concurrency conflict. The driver retries in
the same region, starting at 10ms plus a random salt in `[0ms, 5ms)`, doubling
the delay on each retry, and capping each delay at 1s. The retry stops before
the next delay would exceed the 30s cumulative wait budget. This dedicated
budget does not consume the cross-region failover budget.

Gateway V1 retries are disabled so the SDK owns this policy consistently for
both Gateway V1 and Gateway V2.

### 403 — Forbidden

| Substatus | Meaning                                                                                                                                       | Action                                                 | Budget (multi-write)                                                                                                   | Budget (single-write)                                                                                                  |
| --------- | --------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| 3         | `WriteForbidden` — region is not currently a valid write region for this partition (writes only)                                              | Refresh account topology + cross-region failover retry | **5s cumulative delay**, immediate first retry then exponential backoff with jitter (dedicated backend-failover state) | **5s cumulative delay**, immediate first retry then exponential backoff with jitter (dedicated backend-failover state) |
| 1008      | `DatabaseAccountNotFound` — region no longer owns this account (all op types, including reads, writes, queries, feed-range queries, metadata) | Refresh account topology + cross-region failover retry | **5s cumulative delay**, immediate first retry then exponential backoff with jitter (dedicated backend-failover state) | **5s cumulative delay**, immediate first retry then exponential backoff with jitter (dedicated backend-failover state) |
| Other     | Permission denied                                                                                                                             | Abort                                                  | —                                                                                                                      | —                                                                                                                      |

Both 403/3 and 403/1008 signal that the cached topology in the SDK has diverged from the backend's current routing — typically during a backend-initiated failover or a customer-initiated topology change. On each retry the driver requests `LocationEffect::RefreshAccountProperties` so the next attempt routes against the freshly learned region set. The metadata refresh itself is throttled by a lease on `refresh_interval` (default 5 s): an event-driven caller stamps the clock *before* fetching, which suppresses other event-driven callers for that interval. This is a throttle, not mutual exclusion — a fetch that outlives the interval (metadata requests are allowed up to 65 s) can be joined by a second refresh, and the background timer refresh bypasses the lease entirely. A failed or cancelled refresh also releases its claim so the next retry can fetch immediately. Metadata traffic is therefore not strictly bounded to one fetch per interval. The refresh is independent of the caller's `excluded_regions` — the GetDatabaseAccount probe iterates the global endpoint and the cached `readable_locations` regardless of the operation-level exclusion list, because excluding a region from data-plane routing should not blind the SDK to topology changes happening in that region.

#### `excluded_regions` interaction

The dedicated backend-failover policy filters out `excluded_regions` while selecting preferred endpoints. Exclusions are honored as long as at least one preferred endpoint remains eligible.

Two established availability fallbacks can bypass exclusions:

- If every preferred region is excluded, endpoint resolution makes one last-resort attempt against the first preferred write endpoint (the authoritative hub) rather than failing without sending a request.
- A PPAF per-partition write override is selected from backend-directed failover state and is not re-filtered through `excluded_regions`; it persists until the backend signals another routing change.

Outside these explicit exceptions, excluded regions remain a hard per-operation routing filter. Callers requiring strict “never contact this region” behavior should avoid excluding every preferred endpoint and should not enable PPAF for the affected write path.

### 404/1002 — Read Session Not Available

| Account Type | Action                                             | Budget                               |
| ------------ | -------------------------------------------------- | ------------------------------------ |
| Single-write | Session retry to write region (hub region)         | 2 attempts                           |
| Multi-write  | Session retry, advance through preferred endpoints | `preferred_endpoints.len()` attempts |

The session token is preserved on all retry attempts — it is never cleared to allow stale reads, as that would violate the customer's chosen consistency guarantees. When all session retries are exhausted, the 404/1002 error is surfaced to the caller.

### 408 — Request Timeout

| Operation                         | Action                          | Budget              |
| --------------------------------- | ------------------------------- | ------------------- |
| Reads                             | Cross-region failover retry     | 3 failover attempts |
| Writes (except stored procedures) | **Cross-region failover retry** | 3 failover attempts |
| Stored Procedure writes           | **Abort**                       | —                   |

408 indicates a server-side or client-side timeout. The Rust driver retries writes on 408 because:

- CRUD write operations are idempotent when customers use ETag preconditions (see [Idempotency Requirements](#idempotency-requirements) above).
- 412 (Precondition Failed) prevents silent overwrites if a retried write races with a concurrent update.
- Stored procedure execution is excluded from write retries (not idempotent).

For single-write accounts, retry cycles through the available endpoint(s). For multi-write accounts, retry advances to the next preferred write region.

### 410 — Gone

| Context                  | Action                          | Budget              |
| ------------------------ | ------------------------------- | ------------------- |
| Partition topology       | Dataflow routing-map refresh    | Dataflow retry limit|
| Other 410 response       | Cross-region failover retry     | 3 failover attempts |

Partition topology changes are handled by the dataflow layer, which refreshes
the partition-key-range cache and repairs the affected request node. Other 410
responses use the ordinary cross-region failover path.

### 429 — Too Many Requests (Throttling)

| Request class | Substatus | Action                                        | Default local budget                                   |
| ------------- | --------- | --------------------------------------------- | ------------------------------------------------------ |
| Data plane    | Any       | Local retry with backoff                      | 18 retries / 270s cumulative wait / 15s per retry      |
| Metadata      | Any       | Local retry with backoff                      | 9 retries / 30s cumulative wait / 5s per retry         |
| Any           | 3092      | Fail over after local budget is exhausted     | 3 failover retries after the local budget is exhausted |

The transport pipeline first applies the local throttle policy to every 429,
including 3092. It honors `x-ms-retry-after-ms`; when that header is absent it
uses exponential backoff from 5ms with ±25% jitter. The request-class cap
applies to both service-provided and fallback delays.

The count and cumulative-wait values can be overridden with
`ThrottlingRetryOptions`. The local budget is scoped to one transport-pipeline
invocation, so each operation-level failover leg starts a fresh throttle
budget. If the wait budget is exhausted before the count budget, the driver
may make one final retry while the operation deadline still permits it.

After local retries are exhausted, a standard 429 is surfaced. A 429/3092 is
instead escalated to the operation pipeline and treated like 503, including
cross-region failover and location-health effects.

### 5xx — Server Errors (500, 502, 503, 504)

| Operation                  | Action                          | Budget              |
| -------------------------- | ------------------------------- | ------------------- |
| Reads                      | Cross-region failover retry     | 3 failover attempts |
| Writes (all)               | **Cross-region failover retry** | 3 failover attempts |
| Stored Procedure execution | **Abort** (except 503)          | —                   |

All 5xx errors are retried uniformly. 503 is the canonical "safe to retry" signal from Cosmos DB — when the service intentionally returns 503, it guarantees the write was not processed. All other 5xx codes (500, 502, 504) are retried identically because CRUD write operations are idempotent when customers use ETag preconditions (see [Idempotency Requirements](#idempotency-requirements) above). 502/504 may be raised by intermediate proxies, but ETag preconditions (412 on stale ETag) prevent silent overwrites on retry. Stored procedure execution aborts on every 5xx except 503, which alone proves the procedure did not run — see `is_unsafe_retry_after_possible_execution` in `src/driver/pipeline/retry_evaluation.rs`.

**Location marking**: Each eligible 5xx records a partition failure. When PPCB
owns the route, endpoint-level marking is suppressed and PPCB shifts traffic
only after its per-partition threshold is crossed. When PPCB does not own the
route, the driver also marks the endpoint unavailable immediately.

The driver goes directly to the next failover route on the first 5xx; it does
not first repeat the HTTP-status retry in the same region.

### Transport Errors (Connection Failures)

| Sent Status                              | Operation                  | Action                          | Budget              |
| ---------------------------------------- | -------------------------- | ------------------------------- | ------------------- |
| **Not sent** (request never left client) | Any                        | Cross-region failover retry     | 3 failover attempts |
| **Sent** or unknown                      | Reads                      | Cross-region failover retry     | 3 failover attempts |
| **Sent** or unknown                      | Writes (all)               | **Cross-region failover retry** | 3 failover attempts |
| **Sent** or unknown                      | Stored Procedure execution | **Abort**                       | —                   |

When the request was definitely not sent (for example, connection or DNS
failure), the driver marks only the endpoint unavailable and retries on the
next preferred region. It does not increment PPCB's partition counter because
the failure is endpoint-wide rather than partition-specific.

When the request was possibly sent, the endpoint is clearly reachable — only partition-level marking is applied (via PPCB). The endpoint is not marked unavailable since other partitions on it are unaffected. The partition mark is applied whether or not the operation goes on to retry.

For connectivity errors, the transport layer performs at most one local retry
on a different TCP shard for the same endpoint before escalating to the
operation pipeline. A definitely-not-sent request is always eligible. A
sent-or-unknown request is eligible only when
`CosmosOperation::allows_ambiguous_outcome_retry` is true, which excludes
stored procedure execution. If no different shard is available, the failure
escalates without a local retry.

**Note**: The Rust driver retries non-idempotent writes even when the request may have been sent, because CRUD write operations are idempotent when customers use ETag preconditions (see [Idempotency Requirements](#idempotency-requirements) above). Stored procedure execution is excluded.

### Deadline Exceeded (Client-Side Timeout)

| Operation | Action                                                  | Budget |
| --------- | ------------------------------------------------------- | ------ |
| Any       | **Abort** — synthesize 408 / `CLIENT_OPERATION_TIMEOUT` | —      |

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

| Account Type | Reads          | Writes                                   |
| ------------ | -------------- | ---------------------------------------- |
| Single-write | ✅ PPCB-managed| ❌ Not PPCB-managed (PPAF handles writes)|
| Multi-write  | ✅ PPCB-managed| ✅ PPCB-managed                          |

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

| Layer                                                      | Budget                                                                           | Scope                       |
| ---------------------------------------------------------- | -------------------------------------------------------------------------------- | --------------------------- |
| Transport 429 (data plane)                                 | 18 retries, 270s wait, 15s per retry                                             | Per transport invocation    |
| Transport 429 (metadata)                                   | 9 retries, 30s wait, 5s per retry                                                | Per transport invocation    |
| Operation failover (generic — 5xx, 408, 410, transport)    | 3 attempts                                                                       | Per-operation, cross-region |
| Backend-failover (403/1008) — single-write and multi-write | **5s cumulative delay**, immediate first retry then exponential backoff + jitter | Per-operation, cross-region |
| Backend-failover (403/3) — single-write and multi-write    | **5s cumulative delay**, immediate first retry then exponential backoff + jitter | Per-operation, cross-region |
| Session retry (404/1002)                                   | 2 (single-write) or `preferred_endpoints.len()` (multi-write)                    | Per-operation               |

The 403/3 hub-region discovery branch is the one exception: a 403/3 on a read
with the `hub_region_processing_only` latch rotates the cached hub endpoint and
stays on the generic 3-attempt failover budget with no pacing.
