# Cosmos DB Rust Driver — Retry Mechanisms and Error Code Handling

This document describes the target retry behavior for the Azure Cosmos DB Rust driver (`azure_data_cosmos_driver`). It serves as the authoritative specification for how the driver handles errors, retries, and cross-region failover.

## Design Philosophy

The Rust driver retries writes by default for retryable status codes. This is safe because Cosmos DB's write APIs are designed to be idempotent when used correctly:

- **503 (Service Unavailable)**: Cosmos DB intentionally returns 503 when a write was **not processed** — it is always safe to retry.
- **5xx / 408**: Write retries are safe for CRUD operations because customers can (and should) use ETag preconditions (`If-Match`) to guarantee idempotency on replace and upsert. Create operations are inherently idempotent (a duplicate yields 409 Conflict). Delete operations are inherently idempotent (a duplicate yields 404 Not Found).
- **Stored Procedure execution**: A stored procedure body is opaque to the driver, so a re-run can repeat arbitrary mutations with no way to detect the duplicate. `OperationType::Execute` is therefore **not** retried when the outcome is ambiguous. It *is* still retried on statuses that prove the backend did not run it — see [Stored procedure retries](#stored-procedure-retries).
- **Unsafe server-side PATCH**: An explicit `PatchStrategy::ServerSide` can send
  non-convergent instructions. It follows the same ambiguous-outcome gate as a
  stored procedure: retry only when the result proves the request did not run.

### Stored procedure retries

Stored procedure execution and unsafe explicit server-side PATCH are gated. The
dividing line is whether the response proves the operation did not run.

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

Client-side PATCH is a Read-Modify-Write operation. Non-convergent instruction
lists persist a tracking marker in the same ETag-guarded Replace as the
mutation. A retry that observes the marker returns success without reapplying
the instructions. Duplicate suppression is bounded by retention, capacity,
authoritative verification routing, and cooperating writers preserving the
reserved property and marker order.

`PatchStrategy::Auto` sends retry-safe lists of at most 10 instructions
server-side; unsafe or longer lists use client-side RMW. Client-side-only
settings do not influence that decision. Non-retry-safe client-side lists and
client-side lists with a caller-supplied tracking ID persist a tracking marker,
while retry-safe over-limit lists do not need one. Explicit `ServerSide` never falls back: more than 10
instructions receive service `400`, and unsafe instructions stop on ambiguous
outcomes rather than risk duplicate execution.

`AZURE_COSMOS_PATCH_STRATEGY=ServerSide` selects that behavior process-wide
unless a higher-priority runtime, account, or operation option overrides it.
This can move unsafe PATCHes from marker-backed client-side RMW to server-side
execution. Client-side-only settings such as tracking ID, maximum attempts,
tracking capacity, and retention are ignored whenever strategy resolution uses
the server path.

| Operation                       | Retried?                            | Initial attempt | On retry (duplicate)                                            | App must handle |
| ------------------------------- | ----------------------------------- | --------------- | --------------------------------------------------------------- | --------------- |
| Create                          | Yes                                 | 201 Created     | 409 Conflict                                                    | 409             |
| Delete                          | Yes                                 | 204 No Content  | 404 Not Found                                                   | 404             |
| Replace / Upsert (with ETag)    | Yes                                 | 200 OK          | 412 Precondition Failed (if concurrent update)                  | 412             |
| Replace / Upsert (without ETag) | Yes                                 | 200 OK          | 200 OK (silent overwrite — no concurrency control)              | —               |
| PATCH, tracked client-side RMW  | Yes                                 | 200 OK          | 200 OK (matching marker suppresses duplicate application)       | Reuse ID        |
| PATCH, retry-safe client-side   | Yes                                 | 200 OK          | 200 OK (instruction list converges)                             | —               |
| PATCH, retry-safe server-side   | Yes                                 | 200 OK          | 200 OK (instruction list converges)                             | —               |
| PATCH, unsafe server-side       | **Only when provably not executed** | 200 OK          | N/A — ambiguous outcomes surface                                | Reconcile       |
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

| Operation | Action          | Budget |
| --------- | --------------- | ------ |
| Any       | SDK-owned retry | TBD    |

449 indicates the request must be retried with a modified configuration (e.g., after a collection recreate or partition split). Gateway V1 can handle 449 retries internally, but the Rust SDK always disables Gateway-side 449 retries and owns them in the SDK. This is required for Gateway V2, where all 449 retries must be handled by the SDK.

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

| Operation                                            | Action                          | Budget              |
| ---------------------------------------------------- | ------------------------------- | ------------------- |
| Reads                                                | Cross-region failover retry     | 3 failover attempts |
| Writes allowing ambiguous-outcome retries            | **Cross-region failover retry** | 3 failover attempts |
| Stored procedures and unsafe explicit server PATCHes | **Abort**                       | —                   |

408 indicates a server-side or client-side timeout. The Rust driver retries writes on 408 because:

- CRUD write operations are idempotent when customers use ETag preconditions (see [Idempotency Requirements](#idempotency-requirements) above).
- 412 (Precondition Failed) prevents silent overwrites if a retried write races with a concurrent update.
- Stored procedure execution and unsafe explicit server-side PATCH are excluded
  because their mutations may not be safe to repeat.

For single-write accounts, retry cycles through the available endpoint(s). For multi-write accounts, retry advances to the next preferred write region.

### 410 — Gone

| Operation    | Action                          | Budget              |
| ------------ | ------------------------------- | ------------------- |
| Reads        | Cross-region failover retry     | 3 failover attempts |
| Writes (all) | **Cross-region failover retry** | 3 failover attempts |

410 indicates the partition has moved or is undergoing a split/merge. All operations retry, regardless of idempotency.

### 429 — Too Many Requests (Throttling)

| Substatus              | Action                      | Budget                 |
| ---------------------- | --------------------------- | ---------------------- |
| — (standard)           | Local retry with backoff    | 9 attempts / 30s total |
| 3092 (global throttle) | Cross-region failover retry | 3 failover attempts    |

Standard 429 is handled entirely within the transport pipeline — the operation pipeline never sees it. The transport layer respects `x-ms-retry-after-ms` headers and falls back to exponential backoff (5ms base, 5s cap per attempt).

429/3092 indicates a global/partition-level throttle that cannot be resolved locally. It is escalated to the operation pipeline and treated identically to 503 (cross-region failover).

**No cross-region retry for standard 429.** Throttling is account-wide; moving to another region would not help.

### 5xx — Server Errors (500, 502, 503, 504)

| Operation                                            | Action                          | Budget              |
| ---------------------------------------------------- | ------------------------------- | ------------------- |
| Reads                                                | Cross-region failover retry     | 3 failover attempts |
| Writes allowing ambiguous-outcome retries            | **Cross-region failover retry** | 3 failover attempts |
| Stored procedures and unsafe explicit server PATCHes | **Abort** (except 503)          | —                   |

503 is the canonical "safe to retry" signal from Cosmos DB — when the service intentionally returns 503, it guarantees the write was not processed. Other 5xx codes (500, 502, 504) are retried for operations that allow ambiguous-outcome retries because ETag preconditions can make the final state deterministic (see [Idempotency Requirements](#idempotency-requirements) above). Stored procedure execution and unsafe explicit server-side PATCH abort on those other 5xx responses because their mutations may already have run; 503 remains retryable because it proves the operation did not run. See `is_unsafe_retry_after_possible_execution` in `src/driver/pipeline/retry_evaluation.rs`.

**Endpoint marking**: Individual 5xx failures do not mark endpoints as unavailable. Endpoint unavailability is driven by PPCB's per-partition failure thresholds (see [Per-Partition Circuit Breaker](#per-partition-circuit-breaker-ppcb)). Each failure increments the partition's failure counter; only when the configured threshold is crossed does routing shift to the next preferred region.

**This is the key divergence from other SDKs**: Python gates write retries behind `retry_write`; Java/.NET only retry for multi-write accounts. The Rust driver always retries.

**Note on in-region retries**: Other SDKs (Python, .NET) typically perform 1 local/in-region retry with a delay for 503/500 before escalating to cross-region failover. The Rust driver currently skips this step and goes straight to cross-region failover on the first failure. This may be worth revisiting — a single in-region retry could resolve transient issues without the latency cost of switching regions.

### Transport Errors (Connection Failures)

| Sent Status                              | Operation                                            | Action                          | Budget              |
| ---------------------------------------- | ---------------------------------------------------- | ------------------------------- | ------------------- |
| **Not sent** (request never left client) | Any                                                  | Cross-region failover retry     | 3 failover attempts |
| **Sent** or unknown                      | Reads                                                | Cross-region failover retry     | 3 failover attempts |
| **Sent** or unknown                      | Writes allowing ambiguous-outcome retries            | **Cross-region failover retry** | 3 failover attempts |
| **Sent** or unknown                      | Stored procedures and unsafe explicit server PATCHes | **Abort**                       | —                   |

When the request was definitely not sent (connection refused, DNS failure, TLS error), the endpoint itself is unreachable. The driver marks the endpoint as unavailable (affecting all partitions on it) and records a partition-level failure for PPCB tracking, then retries on the next preferred region.

When the request was possibly sent, the endpoint is clearly reachable — only partition-level marking is applied (via PPCB). The endpoint is not marked unavailable since other partitions on it are unaffected. The partition mark is applied whether or not the operation goes on to retry.

For connectivity errors (connection refused, I/O errors), the transport layer performs 1 local retry on a different TCP shard to the same endpoint before escalating to the operation pipeline for cross-region failover. This local retry is gated by `TransportPipelineContext::allow_sent_transport_retry` (declared and consumed in `src/driver/transport/transport_pipeline.rs`, evaluated by `should_retry_connectivity_failure`). The operation pipeline populates it from `CosmosOperation::allows_ambiguous_outcome_retry` (`src/models/cosmos_operation.rs`) at both call sites in `src/driver/pipeline/operation_pipeline.rs`, so it is `false` for stored procedure execution and unsafe explicit server-side PATCH, and the two retry layers cannot disagree about a single failure. Declining the local retry does not by itself abort the operation; the operation layer applies the same ambiguity gate before cross-region failover.

**Note**: The Rust driver retries non-idempotent writes even when the request may have been sent, because CRUD write operations can reach a deterministic final state when customers use ETag preconditions (see [Idempotency Requirements](#idempotency-requirements) above). Stored procedure execution and unsafe explicit server-side PATCH are excluded.

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
| Single-write | ✅ PPCB-managed | ❌ Not PPCB-managed (PPAF handles writes) |
| Multi-write  | ✅ PPCB-managed | ✅ PPCB-managed                           |

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
| Transport (429)                                            | 9 attempts or 30s                                                                | Per-request, local only     |
| Operation failover (generic — 5xx, 408, 410, transport)    | 3 attempts                                                                       | Per-operation, cross-region |
| Backend-failover (403/1008) — single-write and multi-write | **5s cumulative delay**, immediate first retry then exponential backoff + jitter | Per-operation, cross-region |
| Backend-failover (403/3) — single-write and multi-write    | **5s cumulative delay**, immediate first retry then exponential backoff + jitter | Per-operation, cross-region |
| Session retry (404/1002)                                   | 2 (single-write) or `preferred_endpoints.len()` (multi-write)                    | Per-operation               |

The 403/3 hub-region discovery branch is the one exception: a 403/3 on a read
with the `hub_region_processing_only` latch rotates the cached hub endpoint and
stays on the generic 3-attempt failover budget with no pacing.

## Comparison with Other SDKs

| Behavior                   | Python                  | Java                 | .NET                 | **Rust (Target)**                   |
| -------------------------- | ----------------------- | -------------------- | -------------------- | ----------------------------------- |
| 503 write retry            | Always (no gate)        | Multi-write only     | Multi-write only     | **Always**                          |
| 500 write retry            | Only with `retry_write` | No                   | No                   | **Always**                          |
| 408 write retry            | Only with `retry_write` | No                   | No                   | **Always**                          |
| 502/504 write retry        | Only with `retry_write` | No                   | No                   | **Always**                          |
| Non-idempotent write retry | Gated by `retry_write`  | Gated by multi-write | Gated by multi-write | **Always (no gate)**                |
| Transport sent + write     | Abort                   | Abort                | Abort                | **Retry**                           |
| Stored procedure retry     | No                      | No                   | No                   | **Only when provably not executed** |
| PPAF                       | Yes (single-master)     | Yes                  | Yes                  | **Yes**                             |
| PPCB                       | Yes                     | Yes                  | Yes                  | **Yes**                             |

The Rust driver is intentionally more aggressive about retrying writes. This is a deliberate design choice for maximum availability, leveraging Cosmos DB's conflict detection and the use of Etags as the safety net for duplicates and idempotency concerns. Stored procedure execution is the single carve-out, because the driver cannot reason about a procedure body it never sees.
