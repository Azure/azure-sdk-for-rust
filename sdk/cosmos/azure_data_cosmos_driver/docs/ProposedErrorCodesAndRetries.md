# Cosmos DB Rust Driver — Retry Mechanisms and Error Code Handling

This document describes the target retry behavior for the Azure Cosmos DB Rust driver (`azure_data_cosmos_driver`). It serves as the authoritative specification for how the driver handles errors, retries, and cross-region failover.

## Design Philosophy

The Rust driver **prefers availability over idempotency concerns**. Unlike other Azure Cosmos SDKs that gate write retries behind opt-in flags or idempotency checks, the Rust driver retries writes by default for retryable status codes. This decision was made explicitly by the team, acknowledging that:

- Cosmos DB intentionally returns 503 when a write was **not processed** — it is safe to retry.
- For 500/408, the service team explicitly requested write retries. We extend this to be all 500 status codes for convenience.

## Status Code Handling

### Non-Retryable (Abort Immediately)

| Status | Substatus | Meaning | Action |
|--------|-----------|---------|--------|
| 400 | — | Bad Request | Abort |
| 401 | — | Unauthorized | Abort |
| 409 | — | Conflict | Abort |
| 412 | — | Precondition Failed | Abort |
| 449 | — | Retry With (client must fix request) | Abort |

These are deterministic client errors. No retry will change the outcome.

### 403 — Forbidden

| Substatus | Meaning | Action | Budget |
|-----------|---------|--------|--------|
| 3 | Write Forbidden (region failover) | Cross-region failover retry | 3 failover attempts |
| 1008 | Write Forbidden (partition moved) | Cross-region failover retry | 3 failover attempts |
| Other | Permission denied | Abort | — |

403/3 and 403/1008 indicate the current write endpoint is no longer valid. The driver refreshes account properties to discover the new write region and retries there.

### 404/1002 — Read Session Not Available

| Account Type | Action | Budget |
|--------------|--------|--------|
| Single-write | Session retry to account primary | 2 attempts |
| Multi-write | Session retry, advance through preferred endpoints | `preferred_endpoints.len()` attempts |

On the final attempt, the session token is cleared to allow the target region to serve a potentially stale read rather than failing.

### 408 — Request Timeout

| Operation | Action | Budget |
|-----------|--------|--------|
| Reads | Cross-region failover retry | 3 failover attempts |
| Writes (all) | **Cross-region failover retry** | 3 failover attempts |

408 indicates a server-side timeout. The Rust driver retries writes on 408 because:

- The team explicitly agreed to retry writes for 408.
- Availability is preferred over the theoretical risk of duplicate processing.
- Cosmos DB's conflict detection (409/412) catches actual duplicates.

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

All 5xx errors are retried uniformly. 503 is the canonical "safe to retry" signal from Cosmos DB — when the service intentionally returns 503, it guarantees the write was not processed. The team decided all other 5xx codes (500, 502, 504) should be retried identically, applying the "availability over idempotency" philosophy. 502/504 may be raised by intermediate proxies (e.g., Envoy), but Cosmos DB's conflict detection (409/412) catches actual duplicates.

**This is the key divergence from other SDKs**: Python gates write retries behind `retry_write`; Java/.NET only retry for multi-write accounts. The Rust driver always retries.

**Note on in-region retries**: Other SDKs (Python, .NET) typically perform 1 local/in-region retry with a delay for 503/500 before escalating to cross-region failover. The Rust driver currently skips this step and goes straight to cross-region failover on the first failure. This may be worth revisiting — a single in-region retry could resolve transient issues without the latency cost of switching regions.

### Transport Errors (Connection Failures)

| Sent Status | Operation | Action | Budget |
|-------------|-----------|--------|--------|
| **Not sent** (request never left client) | Any | Cross-region failover retry | 3 failover attempts |
| **Sent** or unknown | Reads | Cross-region failover retry | 3 failover attempts |
| **Sent** or unknown | Writes (all) | **Cross-region failover retry** | 3 failover attempts |

When the request was definitely not sent, no effects are applied because the failure may be transient and the endpoint is not necessarily unhealthy.

When the request was possibly sent, endpoint/partition marks are applied to route subsequent requests away from the failing endpoint.

For connectivity errors (connection refused, I/O errors), the transport layer performs 1 local retry on a different TCP shard to the same endpoint before escalating to the operation pipeline for cross-region failover.

**Note**: The Rust driver retries non-idempotent writes even when the request may have been sent. This aligns with the "availability over idempotency" philosophy.

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

## Per-Partition Automatic Failover (PPAF)

PPAF is an **opt-in** feature for **single-master write accounts only**. When enabled (via server account flag `enable_per_partition_failover_behavior`):

- Partition-level routing overrides are recorded on **successful write confirmation** — not on failure.
- If a write succeeds on a non-primary region during retry, that region is recorded as the partition's current primary.
- PPAF entries do **not** participate in probe-based failback; they are updated only by success-time discovery.

**With the Rust driver's "always retry writes" stance, PPAF primarily adds the partition-level routing intelligence** — the retry itself already happens regardless. PPAF makes the routing *smarter* by remembering which region last successfully served a given partition, and by providing further availability through processing writes in other read regions.

## Per-Partition Circuit Breaker (PPCB)

PPCB is an **opt-in** feature that provides partition-level health tracking and routing:

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
| Operation failover | 3 attempts | Per-operation, cross-region |
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

The Rust driver is intentionally more aggressive about retrying writes. This is a deliberate design choice for maximum availability, leveraging Cosmos DB's conflict detection as the safety net for the rare case of duplicate processing.

## Implementation Status

> **Note**: The current implementation does not fully match this specification. The following gaps exist and should be addressed:

| Gap | Current Behavior | Target Behavior |
|-----|-----------------|-----------------|
| 503 non-idempotent writes (no PPAF) | Abort | Retry (cross-region failover) |
| 500 non-idempotent writes (no PPAF) | Abort | Retry (cross-region failover) |
| 408 non-idempotent writes (no PPAF) | Abort | Retry (cross-region failover) |
| 502/504 non-idempotent writes (no PPAF) | Abort | Retry (cross-region failover) |
| Transport sent + non-idempotent writes (no PPAF) | Abort | Retry (cross-region failover) |

All of these gaps share the same root cause: the retry evaluation currently checks `is_idempotent() || ppaf_write_retry_allowed` before allowing write retries. The fix is to remove the idempotency gate entirely for retryable status codes, making the retry unconditional for all writes.
