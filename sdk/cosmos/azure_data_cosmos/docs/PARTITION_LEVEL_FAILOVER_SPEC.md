# Per-Partition Automatic Failover (PPAF) & Per-Partition Circuit Breaker (PPCB) Spec

**Status:** Draft / Iterating  
**Date:** 2026-03-05  
**Authors:** (team)

---

## Table of Contents

1. [Goals & Motivation](#1-goals--motivation)
2. [Architectural Overview](#2-architectural-overview)
3. [Feature Enablement & Configuration](#3-feature-enablement--configuration)
4. [Eligibility Rules](#4-eligibility-rules)
5. [Component Design](#5-component-design)
6. [Partition Failover Flow](#6-partition-failover-flow)
7. [Circuit Breaker Mechanics](#7-circuit-breaker-mechanics)
8. [Retry Policy Integration](#8-retry-policy-integration)
9. [Background Failback Loop](#9-background-failback-loop)
10. [Status Code Handling Matrix](#10-status-code-handling-matrix)
11. [Configuration Surface](#11-configuration-surface)
12. [Interaction with Account-Level Failover](#12-interaction-with-account-level-failover)
13. [Known Issues & Design Decisions](#13-known-issues--design-decisions)
14. [Test Coverage](#14-test-coverage)

---

## 1. Goals & Motivation

### Problem Statement

Cosmos DB accounts span multiple regions. When a single partition in a region becomes
unhealthy (503, 429/3092, 410/1022) or when a write region changes (403/3), the
**entire region** does not need to be marked unavailable — only the affected partition
should be failed over to the next available region. This provides:

1. **Finer-grained fault isolation** — healthy partitions in the same region continue
   to be served locally, avoiding unnecessary cross-region latency for unaffected
   requests.
2. **Faster recovery** — partition-level failovers are tracked independently, allowing
   the background failback loop to restore each partition as soon as the original
   region recovers, rather than waiting for a full account-level failover reversal.
3. **Multi-master write support** — on accounts with multiple write regions, the
   circuit breaker enables partition-level failover for both reads and writes across
   preferred regions.

### Two Complementary Mechanisms

The SDK implements two distinct but complementary partition-level failover mechanisms:

| Mechanism | Abbreviation | Applies To | Account Type | Trigger |
|---|---|---|---|---|
| Per-Partition Automatic Failover | **PPAF** | **Writes only** | **Single-master** (one write region) | 403/3 WriteForbidden, 503, 429/3092, 410/1022 |
| Per-Partition Circuit Breaker | **PPCB** | **Reads** (any account), **Writes** on multi-master | **Multi-master** + all accounts for reads | Consecutive failure threshold exceeded |

These two mechanisms are **mutually exclusive per request** — a given request uses
either the PPAF path or the PPCB path, never both. The decision is based on the
request's operation type (read vs. write) and whether the account supports multiple
write locations.

### Design Principles

- **Partition granularity**: Failover state is tracked per `(PartitionKeyRange, Region)` pair.
- **Threshold-gated**: The circuit breaker does not trip on the first failure. Consecutive
  failure counters must exceed configurable thresholds before a partition is failed over.
- **Non-deterministic failback**: After a configurable unavailability window, failed
  partitions are optimistically marked healthy. Actual health is verified by subsequent
  requests routing back to the original region.
- **Environment-variable configurable**: All thresholds, windows, and intervals are
  overridable via environment variables for testing and operational flexibility.
- **No control-plane dependency**: Failover decisions are made locally by the SDK based
  on observed request failures — no server-side signal is required beyond the HTTP
  status codes.

---

## 2. Architectural Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         ClientRetryPolicy                                   │
│                                                                             │
│  before_send_request()                                                      │
│  ├─ Resolve endpoint via GlobalEndpointManager                              │
│  ├─ Set route on RequestContext                                             │
│  └─ try_add_partition_level_location_override()  ◄── PPAF/PPCB override     │
│                                                                             │
│  should_retry()                                                             │
│  ├─ 403/3 WriteForbidden                                                    │
│  │   ├─ [PPAF] try_mark_endpoint_unavailable_for_partition_key_range()      │
│  │   ├─ [PPCB] increment + check threshold → try_mark + retry              │
│  │   └─ [else] account-level failover (should_retry_on_endpoint_failure)    │
│  ├─ 503 / 429/3092 / 410/1022                                              │
│  │   ├─ try_mark_endpoint_unavailable_for_pk_range()                        │
│  │   └─ should_retry_on_unavailable_endpoint_status_codes()                 │
│  └─ Reads: retry on any non-fatal status code                              │
│                                                                             │
└──────────────────────┬──────────────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                   GlobalPartitionEndpointManager                            │
│                                                                             │
│  State:                                                                     │
│  ┌─ partition_key_range_to_location_for_write ──────────────────────────┐   │
│  │  HashMap<PartitionKeyRange, PartitionKeyRangeFailoverInfo>           │   │
│  │  Used by: PPAF (single-master writes)                                │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│  ┌─ partition_key_range_to_location_for_read_and_write ─────────────────┐   │
│  │  HashMap<PartitionKeyRange, PartitionKeyRangeFailoverInfo>           │   │
│  │  Used by: PPCB (reads + multi-master writes)                         │   │
│  └──────────────────────────────────────────────────────────────────────┘   │
│                                                                             │
│  Methods:                                                                   │
│  ├─ try_add_partition_level_location_override(request)                      │
│  │   Checks existing failover entries → overrides request routing           │
│  ├─ try_mark_endpoint_unavailable_for_partition_key_range(request)          │
│  │   Adds or updates failover entry → moves to next region                 │
│  ├─ increment_request_failure_counter_and_check_if_partition_can_failover() │
│  │   Increments failure count → returns true when threshold exceeded        │
│  └─ is_request_eligible_for_*()                                            │
│      Eligibility checks for PPAF vs PPCB routing                           │
│                                                                             │
│  Background:                                                                │
│  └─ initiate_circuit_breaker_failback_loop()                               │
│      Periodic sweep: mark expired entries healthy, remove overrides         │
│                                                                             │
└──────────────────────┬──────────────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                       GlobalEndpointManager                                 │
│                                                                             │
│  Provides:                                                                  │
│  ├─ read_endpoints() → Vec<Url>      (ordered by preferred regions)         │
│  ├─ account_read_endpoints() → Vec<Url>  (account-level read regions)       │
│  ├─ can_support_multiple_write_locations(resource, operation) → bool        │
│  └─ resolve_service_endpoint(request) → Url                                │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Request Flow Summary

1. **Before send** (`before_send_request`):
   - Resolve the default endpoint via `GlobalEndpointManager`.
   - If partition-level failover is enabled and the request targets a partitioned
     resource, call `try_add_partition_level_location_override()` to check if an
     existing failover override should redirect this request to an alternate region.

2. **On failure** (`should_retry`):
   - Classify the error (403/3, 503, 429/3092, etc.).
   - If the request is eligible for PPAF or PPCB, record the failure and
     potentially move the partition to the next available region.
   - Return `Retry` so the retry loop re-invokes `before_send_request`, which
     will now apply the updated partition-level override.

3. **Background failback**:
   - A periodic task scans all failed-over partitions.
   - Partitions whose first failure is older than the configured unavailability
     duration are optimistically marked healthy and their overrides are removed.

---

## 3. Feature Enablement & Configuration

### Enable/Disable Flags

| Flag | Source | Default | Description |
|---|---|---|---|
| `partition_level_circuit_breaker_enabled` | Env var `AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED` | `true` | Master switch for PPCB. When `false`, PPCB is disabled entirely. |
| `partition_level_automatic_failover_enabled` | Server-side `AccountProperties.enable_per_partition_failover_behavior` | `false` | PPAF is enabled when the Cosmos DB account has this flag set. Updated dynamically on each account properties refresh. |

### Dynamic Reconfiguration

Both flags are stored as `AtomicBool` values on `GlobalPartitionEndpointManager` and
can be updated at runtime:

- **PPAF**: Updated via `configure_partition_level_automatic_failover()`, called from
  the `on_account_refresh` callback registered during client construction. When the
  server-side account property `enable_per_partition_failover_behavior` changes, the
  next account properties refresh picks it up.

- **PPCB**: Updated via `configure_per_partition_circuit_breaker()`, also called from
  the same callback. The effective value is:
  ```
  enable_per_partition_failover_behavior || env_var_circuit_breaker_enabled
  ```
  This means PPCB is enabled if **either** the server flag or the client-side
  environment variable is set to `true`.

### Initialization

```rust
// In CosmosClientBuilder::build():

// 1. Read env var (defaults to true)
let enable_partition_level_circuit_breaker =
    env::var("AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED")
        .ok()
        .and_then(|v| v.parse::<bool>().ok())
        .unwrap_or(true);

// 2. Create manager with initial flags
//    PPAF starts as false — updated on first account refresh
let partition_manager = GlobalPartitionEndpointManager::new(
    global_endpoint_manager.clone(),
    /* partition_level_failover_enabled: */ false,
    enable_partition_level_circuit_breaker,
);

// 3. Register account refresh callback for dynamic updates
global_endpoint_manager.set_on_account_refresh_callback(Arc::new(
    move |account_props: &AccountProperties| {
        partition_manager.configure_partition_level_automatic_failover(
            account_props.enable_per_partition_failover_behavior,
        );
        partition_manager.configure_per_partition_circuit_breaker(
            account_props.enable_per_partition_failover_behavior
                || enable_partition_level_circuit_breaker,
        );
    },
));
```

---

## 4. Eligibility Rules

### 4.1 Per-Partition Automatic Failover (PPAF)

A request is eligible when **all** conditions are true:

1. `partition_level_automatic_failover_enabled == true`
2. The request is a **write** operation (`!is_read_only_request()`)
3. The account is **single-master** (`!can_support_multiple_write_locations()`)

```rust
fn is_request_eligible_for_per_partition_automatic_failover(request) -> bool {
    partition_level_automatic_failover_enabled
        && !request.is_read_only_request()
        && !global_endpoint_manager.can_support_multiple_write_locations(
            request.resource_type,
            request.operation_type,
        )
}
```

**Rationale**: On a single-master account, the write region is fixed. When a specific
partition's write endpoint returns 403/3 (WriteForbidden), it means the server is
redirecting writes for that partition to a read region. PPAF handles this by routing
subsequent writes for that partition to the next available read region.

### 4.2 Per-Partition Circuit Breaker (PPCB)

A request is eligible when **all** conditions are true:

1. `partition_level_circuit_breaker_enabled == true`
2. The request targets `ResourceType::Documents` or
   `ResourceType::StoredProcedures` with `OperationType::Execute`
3. The request is **either**:
   - A **read** operation, **or**
   - A **write** operation on a **multi-master** account

```rust
fn is_request_eligible_for_partition_level_circuit_breaker(request) -> bool {
    partition_level_circuit_breaker_enabled
        && (request.resource_type == Documents
            || (request.resource_type == StoredProcedures
                && request.operation_type == Execute))
        && (request.is_read_only_request()
            || global_endpoint_manager.can_support_multiple_write_locations(
                request.resource_type,
                request.operation_type,
            ))
}
```

**Rationale**: Multi-master accounts treat all regions as write regions. The circuit
breaker path handles both reads and writes by tracking consecutive failures per
partition and failing over to the next preferred region when the threshold is exceeded.

### 4.3 Shared Pre-Conditions

Both mechanisms share additional validation via `is_request_eligible_for_partition_failover()`:

1. At least one of PPAF or PPCB must be enabled.
2. The request must target a resource type that supports partition-level failover
   (Documents, or StoredProcedures+Execute).
3. There must be **more than one read endpoint** available (otherwise there is
   nowhere to fail over to).
4. A resolved `PartitionKeyRange` must exist on the request context.
5. When validating a failed location, a valid `location_endpoint_to_route` must
   be present.

---

## 5. Component Design

### 5.1 `GlobalPartitionEndpointManager`

The central coordinator for partition-level failover state. Manages two separate
failover maps and the background failback loop.

```rust
pub struct GlobalPartitionEndpointManager {
    /// Reference to account-level endpoint manager
    global_endpoint_manager: Arc<GlobalEndpointManager>,

    /// Duration a partition must remain marked unavailable before failback
    /// Default: 5 seconds
    /// Env: AZURE_COSMOS_ALLOWED_PARTITION_UNAVAILABILITY_DURATION_IN_SECONDS
    partition_unavailability_duration_secs: i64,

    /// Interval for the background failback sweep
    /// Default: 300 seconds (5 minutes)
    /// Env: AZURE_COSMOS_PPCB_STALE_PARTITION_UNAVAILABILITY_REFRESH_INTERVAL_IN_SECONDS
    background_connection_init_interval_secs: i64,

    /// PPAF map: writes on single-master accounts
    partition_key_range_to_location_for_write:
        Arc<RwLock<HashMap<PartitionKeyRange, PartitionKeyRangeFailoverInfo>>>,

    /// PPCB map: reads (any account) + writes on multi-master
    partition_key_range_to_location_for_read_and_write:
        Arc<RwLock<HashMap<PartitionKeyRange, PartitionKeyRangeFailoverInfo>>>,

    /// Atomic flags
    partition_level_automatic_failover_enabled: AtomicBool,  // PPAF
    partition_level_circuit_breaker_enabled: AtomicBool,     // PPCB

    /// Manages the background failback task lifecycle
    background_task_manager: BackgroundTaskManager,
}
```

**Why two maps?** Single-master accounts have a distinct write endpoint that differs
from the read endpoints. When a write partition fails over on a single-master
account, it must use the account-level read endpoints (`account_read_endpoints()`).
On multi-master accounts, all regions serve both reads and writes, so the preferred
read endpoints (`read_endpoints()`) are used for both. Keeping separate maps avoids
cross-contamination between these two routing strategies.

### 5.2 `PartitionKeyRangeFailoverInfo`

Per-partition failover tracking state. One instance exists per failed-over partition
key range.

```rust
pub struct PartitionKeyRangeFailoverInfo {
    // ── Routing State ──────────────────────────────────────────
    /// Current endpoint URI this partition is routed to
    pub current: String,
    /// Original endpoint that first failed (used for failback)
    pub first_failed_location: String,
    /// Collection resource ID
    pub collection_rid: String,
    /// Set of locations already tried (location → timestamp)
    failed_locations: Mutex<HashMap<String, Instant>>,

    // ── Failure Counters ───────────────────────────────────────
    /// Consecutive read failures (atomic, lock-free increment)
    consecutive_read_request_failure_count: AtomicI32,
    /// Consecutive write failures (atomic, lock-free increment)
    consecutive_write_request_failure_count: AtomicI32,

    // ── Thresholds ─────────────────────────────────────────────
    /// Read failures before circuit trips (default: 2)
    read_request_failure_counter_threshold: i32,
    /// Write failures before circuit trips (default: 5)
    write_request_failure_counter_threshold: i32,
    /// Window after which counters reset (default: 5 minutes)
    timeout_counter_reset_window: Duration,

    // ── Timestamps ─────────────────────────────────────────────
    /// When the first failure occurred (for failback eligibility)
    pub first_request_failure_time: Instant,
    /// When the most recent failure occurred (for counter reset)
    last_request_failure_time: RwLock<Instant>,
}
```

### 5.3 `PartitionKeyRange`

The key type used to identify partitions in the failover maps.

```rust
pub struct PartitionKeyRange {
    pub id: String,
    pub min_inclusive: String,
    pub max_exclusive: String,
}
```

Equality and hashing are based on `id` only, which uniquely identifies a partition
key range within a collection.

---

## 6. Partition Failover Flow

### 6.1 Override Application (Before Send)

On every request attempt, `before_send_request()` checks for existing partition-level
overrides:

```
before_send_request(request)
  │
  ├─ resolve default endpoint via GlobalEndpointManager
  ├─ set RequestContext.location_endpoint_to_route
  │
  └─ if partition_level_failover_enabled && resource.is_partitioned():
      │
      └─ try_add_partition_level_location_override(request)
          │
          ├─ Extract PartitionKeyRange from request context
          │
          ├─ if eligible for PPCB:
          │   └─ lookup in partition_key_range_to_location_for_read_and_write
          │       ├─ if entry found AND threshold exceeded:
          │       │   └─ override request endpoint → entry.current
          │       └─ if entry found BUT threshold NOT exceeded:
          │           └─ no override (continue to original endpoint)
          │
          └─ else if eligible for PPAF:
              └─ lookup in partition_key_range_to_location_for_write
                  └─ if entry found:
                      └─ override request endpoint → entry.current
```

**Key difference**: PPAF overrides unconditionally when an entry exists. PPCB
additionally checks `can_circuit_breaker_trigger_partition_failover()` — the threshold
gate — before applying the override. This means PPCB requires the failure count to
exceed the threshold before the partition is actually routed to the alternate region,
even if a failover entry already exists.

### 6.2 Marking Endpoint Unavailable (On Failure)

When a request fails with an eligible status code:

```
try_mark_endpoint_unavailable_for_partition_key_range(request)
  │
  ├─ Validate eligibility (partition key range, failed location, etc.)
  │
  ├─ if eligible for PPCB:
  │   │
  │   │  next_locations = global_endpoint_manager.read_endpoints()
  │   │  (preferred read regions — all regions on multi-master)
  │   │
  │   └─ try_add_or_update_partition_failover_info_and_move_to_next_location(
  │         partition_key_range,
  │         failed_location,
  │         next_locations,
  │         request,
  │         partition_key_range_to_location_for_read_and_write,
  │      )
  │
  └─ else if eligible for PPAF:
      │
      │  next_locations = global_endpoint_manager.account_read_endpoints()
      │  (account-level read regions — for single-master write failover)
      │
      └─ try_add_or_update_partition_failover_info_and_move_to_next_location(
            partition_key_range,
            failed_location,
            next_locations,
            request,
            partition_key_range_to_location_for_write,
         )
```

### 6.3 Moving to Next Location

```
try_add_or_update_partition_failover_info_and_move_to_next_location(
    partition_key_range, failed_location, next_locations, request, map)
  │
  ├─ Acquire write lock on map
  │
  ├─ Get or insert PartitionKeyRangeFailoverInfo for this partition
  │   (new entry: current = failed_location, first_failed = failed_location)
  │
  ├─ try_move_next_location(next_locations, failed_location):
  │   │
  │   ├─ if failed_location != current:
  │   │   └─ return true  (another thread already moved it)
  │   │
  │   ├─ for each location in next_locations:
  │   │   ├─ skip if location == current
  │   │   ├─ skip if location already in failed_locations set
  │   │   └─ found! → record failed_location in failed_set,
  │   │              update current = location, return true
  │   │
  │   └─ return false  (all locations exhausted)
  │
  ├─ if moved successfully → return true
  │
  └─ if all locations exhausted:
      └─ remove entry from map → return false
         (partition returns to default routing on next request)
```

---

## 7. Circuit Breaker Mechanics

### 7.1 Failure Counter Tracking

The circuit breaker maintains per-partition consecutive failure counters. The counters
are incremented on each failure and checked against configurable thresholds.

```
increment_request_failure_counter_and_check_if_partition_can_failover(request)
  │
  ├─ Validate eligibility and extract partition key range + failed location
  │
  ├─ Get or insert PartitionKeyRangeFailoverInfo in the appropriate map
  │
  ├─ increment_request_failure_counts(is_read_only, current_time):
  │   │
  │   ├─ if (current_time - last_failure_time) > timeout_counter_reset_window:
  │   │   └─ reset both read and write counters to 0
  │   │
  │   ├─ if is_read_only:
  │   │   └─ consecutive_read_failure_count += 1
  │   └─ else:
  │       └─ consecutive_write_failure_count += 1
  │   │
  │   └─ update last_request_failure_time = current_time
  │
  └─ can_circuit_breaker_trigger_partition_failover(is_read_only):
      ├─ if is_read_only:
      │   └─ return read_count > read_threshold  (default: 2)
      └─ else:
          └─ return write_count > write_threshold  (default: 5)
```

### 7.2 Threshold Configuration

| Parameter | Default | Environment Variable |
|---|---|---|
| Read failure threshold | 2 | `AZURE_COSMOS_CIRCUIT_BREAKER_CONSECUTIVE_FAILURE_COUNT_FOR_READS` |
| Write failure threshold | 5 | `AZURE_COSMOS_CIRCUIT_BREAKER_CONSECUTIVE_FAILURE_COUNT_FOR_WRITES` |
| Counter reset window | 5 minutes | `AZURE_COSMOS_CIRCUIT_BREAKER_TIMEOUT_COUNTER_RESET_WINDOW_IN_MINUTES` |

**Why reads = 2, writes = 5?** Reads are idempotent and safe to fail over
aggressively. Writes are more expensive to fail over (potential double-write risk on
multi-master), so a higher threshold reduces false-positive failovers due to transient
errors.

### 7.3 Counter Reset Window

If the time between two consecutive failures exceeds `timeout_counter_reset_window`
(default: 5 minutes), **both** read and write counters are reset to zero before the
new failure is recorded. This prevents stale failures from accumulating across long
idle periods — if a partition has been healthy for 5 minutes, any new failure starts
the counter fresh.

### 7.4 Circuit Breaker State Transitions

```
                                ┌──────────────┐
                                │   HEALTHY     │
              ┌─────────────────│ (no entry in  │◄─────────────┐
              │   request       │  failover map)│              │
              │   failure       └──────┬───────┘              │
              │                        │                       │
              ▼                        │                       │
     ┌────────────────┐                │               ┌────────┴─────────┐
     │  COUNTING       │                │              │   FAILBACK       │
     │ (entry exists,  │                │              │ (background loop │
     │  threshold NOT  │                │              │  removes entry   │
     │  exceeded)      │                │              │ after cool-down) │
     │                 │    consecutive  │             └──────────────────┘
     │  counter++      │    failures    │                       ▲
     │                 │    > threshold  │                       │
     └────────┬───────┘                │                       │
              │                        │                       │
              ▼                        │                       │
     ┌────────────────┐                │                       │
     │   TRIPPED       │ ──────────────┘                       │
     │ (entry.current  │                                       │
     │  = next region, │                                       │
     │  override       ├───────────────────────────────────────┘
     │  applied)       │   unavailability_duration exceeded
     └────────────────┘   (non-deterministic health recovery)
```

---

## 8. Retry Policy Integration

### 8.1 `ClientRetryPolicy` Interaction Points

The `ClientRetryPolicy` integrates with PPAF/PPCB at two points:

#### 8.1.1 Before Send (`before_send_request`)

```rust
// After resolving the default endpoint:
if self.partition_key_range_location_cache.partition_level_failover_enabled()
    && request.resource_type.is_partitioned()
{
    self.partition_key_range_location_cache
        .try_add_partition_level_location_override(request);
}
```

This applies any existing partition-level override to the request's routing.

#### 8.1.2 On 403/3 WriteForbidden

```rust
// In should_retry_on_http_status():
if status_code == 403 && sub_status == WRITE_FORBIDDEN {
    if self.request.is_some()
        && (eligible_for_ppaf || eligible_for_ppcb)
        && try_mark_endpoint_unavailable_for_partition_key_range(request)
    {
        return Retry { after: 0 };  // Immediate retry with new override
    }

    // Fall through to account-level failover
    return should_retry_on_endpoint_failure(force_refresh: true);
}
```

**Critical behavior**: When the partition-level mark succeeds, the retry is
immediate (`after: 0`). The next `before_send_request` call will find the
updated override and route to the new region. When the partition-level mark
fails (e.g., all regions exhausted), the request falls through to account-level
failover with a location cache refresh.

#### 8.1.3 On 503 / 429/3092 / 410/1022

```rust
// For these status codes:
fn try_mark_endpoint_unavailable_for_pk_range_and_retry_on_service_unavailable() {
    // Step 1: Try to mark partition unavailable (if eligible)
    try_mark_endpoint_unavailable_for_pk_range();

    // Step 2: Always retry on next available endpoint (account-level)
    should_retry_on_unavailable_endpoint_status_codes()
}
```

For these codes, the partition marking happens **in addition to** the account-level
retry. The partition mark ensures future requests for the same partition avoid the
failed region, while the account-level retry routes the current request to the next
endpoint.

### 8.2 PPCB Threshold Check in Retry Path

The `is_request_eligible_for_partition_level_circuit_breaker()` method on
`ClientRetryPolicy` performs a **combined** check:

```rust
fn is_request_eligible_for_partition_level_circuit_breaker(&self) -> bool {
    if let Some(request) = self.request.as_ref() {
        return self.partition_key_range_location_cache
                   .is_request_eligible_for_partition_level_circuit_breaker(request)
            && self.partition_key_range_location_cache
                   .increment_request_failure_counter_and_check_if_partition_can_failover(request);
    }
    false
}
```

This means:
1. Check eligibility (circuit breaker enabled, correct resource/operation type).
2. **AND** increment the failure counter **AND** check if the threshold is now
   exceeded.

The increment-and-check is atomic: every call to this method records the failure,
even if the threshold is not yet reached. The failover only triggers when the
accumulated count crosses the threshold.

---

## 9. Background Failback Loop

### 9.1 Loop Structure

The background failback loop runs as a spawned async task, managed by
`BackgroundTaskManager`. It uses a `Weak<Self>` reference to avoid preventing the
`GlobalPartitionEndpointManager` from being dropped when the client is dropped.

```
initiate_circuit_breaker_failback_loop(weak_self)
  │
  loop:
  │
  ├─ sleep(background_connection_init_interval_secs)  // default: 300s
  │
  ├─ upgrade weak_self → strong (exit loop if None)
  │
  └─ initiate_failback_to_unhealthy_endpoints():
      │
      ├─ Scan partition_key_range_to_location_for_read_and_write (read lock)
      │
      ├─ For each entry where:
      │     (now - first_request_failure_time) > partition_unavailability_duration
      │   Collect: (pk_range, collection_rid, original_failed_location)
      │
      ├─ mark_endpoints_to_healthy():
      │   └─ Set health status to Healthy for all collected entries
      │
      └─ For each entry marked Healthy:
          └─ Remove from partition_key_range_to_location_for_read_and_write (write lock)
              (future requests will route back to original region)
```

### 9.2 Failback Timing

| Parameter | Default | Environment Variable |
|---|---|---|
| Unavailability duration before failback | 5 seconds | `AZURE_COSMOS_ALLOWED_PARTITION_UNAVAILABILITY_DURATION_IN_SECONDS` |
| Background sweep interval | 300 seconds | `AZURE_COSMOS_PPCB_STALE_PARTITION_UNAVAILABILITY_REFRESH_INTERVAL_IN_SECONDS` |

**Interaction**: A partition must have been unavailable for at least 5 seconds
before the failback loop considers it. However, since the loop only runs every
300 seconds by default, the effective failback time is between 5 and 305 seconds.

### 9.3 Non-Deterministic Health Recovery

The failback is called "non-deterministic" because no actual health probe is
performed. When the unavailability window expires:

1. The override entry is removed from the map.
2. The next request for that partition will route to the **default** endpoint
   (the original region).
3. If the original region is still unhealthy, the request will fail again and
   the partition will be re-failed-over through the normal failure path.

This optimistic approach avoids the complexity and cost of active health probing
while still ensuring eventual recovery.

### 9.4 Scope Limitation

Currently, the failback loop only scans `partition_key_range_to_location_for_read_and_write`
(the PPCB map). The `partition_key_range_to_location_for_write` (PPAF map) is **not**
scanned by the background loop. PPAF entries are only removed when all locations are
exhausted during `try_move_next_location()`.

---

## 10. Status Code Handling Matrix

The following table maps each status code to its behavior with respect to PPAF/PPCB:

| Status Code | Sub-Status | PPAF Action | PPCB Action | Account-Level Action |
|---|---|---|---|---|
| 403 | 3 (WriteForbidden) | Mark partition unavailable → retry | Increment counter → if threshold exceeded, mark unavailable → retry | Fallback: `should_retry_on_endpoint_failure(force_refresh: true)` |
| 503 | Any | Mark partition unavailable | Mark partition unavailable | `should_retry_on_unavailable_endpoint_status_codes()` |
| 429 | 3092 (SystemResourceUnavailable) | Mark partition unavailable | Mark partition unavailable | Treated as 503 for write requests on multi-master |
| 410 | 1022 (LeaseNotFound) | Mark partition unavailable | Mark partition unavailable | `should_retry_on_unavailable_endpoint_status_codes()` |
| 404 | 1022 (ReadSessionNotAvailable) | N/A | N/A | `should_retry_on_session_not_available()` |
| Any other | Read operations | N/A | N/A | Retry on non-fatal status codes |

### Partition Marking vs. Account-Level Retry

For 503, 429/3092, and 410/1022: **both** partition marking and account-level retry
happen. The partition marking affects **future** requests for the same partition (they
will be routed to the override region), while the account-level retry routes the
**current** request to the next endpoint.

For 403/3: partition-level handling takes **priority**. If the partition is
successfully marked, the request retries immediately without falling through to
account-level failover. Only when partition-level marking fails (all regions exhausted
or request not eligible) does it fall through to `should_retry_on_endpoint_failure()`.

---

## 11. Configuration Surface

### 11.1 Environment Variables

| Variable | Type | Default | Description |
|---|---|---|---|
| `AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED` | `bool` | `true` | Master switch for per-partition circuit breaker |
| `AZURE_COSMOS_ALLOWED_PARTITION_UNAVAILABILITY_DURATION_IN_SECONDS` | `i64` | `5` | Minimum time a partition must be unavailable before failback sweep considers it |
| `AZURE_COSMOS_PPCB_STALE_PARTITION_UNAVAILABILITY_REFRESH_INTERVAL_IN_SECONDS` | `i64` | `300` | Interval between background failback sweep iterations |
| `AZURE_COSMOS_CIRCUIT_BREAKER_CONSECUTIVE_FAILURE_COUNT_FOR_READS` | `i32` | `2` | Read failure threshold before circuit trips |
| `AZURE_COSMOS_CIRCUIT_BREAKER_CONSECUTIVE_FAILURE_COUNT_FOR_WRITES` | `i32` | `5` | Write failure threshold before circuit trips |
| `AZURE_COSMOS_CIRCUIT_BREAKER_TIMEOUT_COUNTER_RESET_WINDOW_IN_MINUTES` | `i64` | `5` | Window (in minutes) after which failure counters reset |

### 11.2 Server-Side Configuration

| Property | Source | Description |
|---|---|---|
| `enable_per_partition_failover_behavior` | `AccountProperties` JSON | Enables PPAF for single-master write accounts. Updated dynamically on account refresh. |

---

## 12. Interaction with Account-Level Failover

### 12.1 Layered Failover

Partition-level and account-level failover operate as complementary layers:

```
Request fails
  │
  ├─ Partition-level (PPAF/PPCB):
  │   Route THIS partition to alternate region
  │   Other partitions in the same region are unaffected
  │
  └─ Account-level (GlobalEndpointManager):
      Route ALL requests for the account to alternate region
      Marks entire endpoint as unavailable for reads/writes
```

### 12.2 Priority

1. **Partition-level overrides are applied last** in `before_send_request()`, after
   account-level endpoint resolution. This means a partition-level override takes
   precedence over the account-level routing decision.

2. **For 403/3**, partition-level handling is attempted first. Account-level failover
   is the fallback.

3. **For 503/429/3092/410/1022**, both partition-level and account-level actions
   happen in parallel — the partition is marked for future requests, and the current
   request retries on the next account-level endpoint.

### 12.3 Location Lists

| Mechanism | Location Source | Description |
|---|---|---|
| PPAF | `global_endpoint_manager.account_read_endpoints()` | Account-level read regions. Used because single-master writes can only fail over to read regions. |
| PPCB | `global_endpoint_manager.read_endpoints()` | Preferred read endpoints (respects `preferred_locations` ordering). Used because multi-master treats all regions as read/write. |

---

## 13. Known Issues & Design Decisions

### 13.1 PPAF Map Not Scanned by Failback Loop

The background failback loop only scans the PPCB map
(`partition_key_range_to_location_for_read_and_write`), not the PPAF map
(`partition_key_range_to_location_for_write`). This means PPAF entries persist
until all locations are exhausted. This is by design — PPAF is triggered by
server-side region changes (403/3), and the server-side state is recovered via
account properties refresh, not via the SDK's background loop.

### 13.2 Failure Counter Side Effects

The `is_request_eligible_for_partition_level_circuit_breaker()` method on
`ClientRetryPolicy` calls `increment_request_failure_counter_and_check_if_partition_can_failover()`,
which **always increments** the counter even if the threshold is not yet exceeded.
This is intentional — every failure must be recorded to eventually cross the
threshold — but it means calling the eligibility check has a side effect.

### 13.3 Threshold Gate on Override Application

When a PPCB entry exists but the failure count has not yet exceeded the threshold,
`try_route_request_for_partition_level_override()` returns `false` and no override
is applied. This means the request continues to hit the original (possibly unhealthy)
region until enough failures accumulate. This is a deliberate trade-off:

- **Pro**: Prevents premature failovers on transient, self-healing errors.
- **Con**: Requests continue to fail until the threshold is reached, adding latency.

### 13.4 Lock Granularity

The failover maps use `RwLock<HashMap<...>>`. The read path
(`try_route_request_for_partition_level_override`) acquires a read lock, and the
write paths (`try_mark_endpoint_unavailable_for_partition_key_range`,
`try_move_next_location`) acquire a write lock. Under high concurrency with many
distinct partitions failing simultaneously, write-lock contention on the map could
become a bottleneck.

**Future improvement**: Migration to the unified `LocationStateStore` (see
[Transport Pipeline Spec §4.6](TRANSPORT_PIPELINE_SPEC.md)) with epoch-guarded
atomic pointers (`crossbeam::epoch::Atomic<T>`) will eliminate reader/writer
contention on the hot path.

### 13.5 Stale Override After Account Refresh

When account properties are refreshed and the region topology changes (e.g., a new
region is added), existing partition-level override entries are **not** invalidated.
The overrides continue to route to the previously selected alternate region until
either:
- The failback loop removes them, or
- All locations are exhausted and the entry is removed.

This is generally acceptable because region topology changes are rare, but it means
the override may point to a less-optimal region after a topology change.

---

## 14. Test Coverage

The implementation includes comprehensive tests covering:

### 14.1 Eligibility Tests

- PPAF eligibility for write operations on single-master accounts
- PPAF ineligibility for read operations
- PPAF ineligibility on multi-master accounts
- PPCB eligibility for reads on any account type
- PPCB eligibility for writes on multi-master accounts
- PPCB ineligibility for writes on single-master accounts
- Ineligibility when both flags are disabled
- Ineligibility for non-partitioned resource types (Databases, Containers, etc.)
- Ineligibility when only one read endpoint is available

### 14.2 Failover Mechanics Tests

- Moving to next location skips current and already-tried locations
- All locations exhausted → entry removed
- Concurrent move detection (another thread already moved)
- Write lock semantics on failover map updates

### 14.3 Circuit Breaker Tests

- Read failure counter increment and threshold check
- Write failure counter increment and threshold check
- Counter reset after timeout window elapses
- Threshold not exceeded → no failover
- Threshold exceeded → failover triggered

### 14.4 Failback Tests

- Background loop exits when manager is dropped (Weak upgrade fails)
- Partitions eligible for failback after unavailability duration
- Partitions NOT eligible before unavailability duration
- Override entry removed after failback

### 14.5 Integration Tests (Retry Policy)

- 403/3 with PPCB enabled → partition-level retry
- 403/3 without PPCB → account-level failover
- 503 → partition marked + account-level retry
- 429/3092 on multi-master → treated as 503
- Multi-region failover with 3 regions → round-robin through regions
- Location exhaustion → entry cleanup

---

## Appendix: Data Flow Sequence Diagram

```
Client ───────────────── ClientRetryPolicy ──── GlobalPartitionEndpointMgr ──── GlobalEndpointMgr
  │                           │                           │                           │
  │  send request             │                           │                           │
  │ ─────────────────────►    │                           │                           │
  │                           │  resolve_service_endpoint │                           │
  │                           │ ──────────────────────────────────────────────────►    │
  │                           │                           │                  endpoint  │
  │                           │ ◄──────────────────────────────────────────────────    │
  │                           │                           │                           │
  │                           │  try_add_partition_level  │                           │
  │                           │  _location_override       │                           │
  │                           │ ─────────────────────►    │                           │
  │                           │                           │──┐ lookup in map          │
  │                           │                           │  │ check threshold        │
  │                           │    override endpoint      │◄─┘                        │
  │                           │ ◄─────────────────────    │                           │
  │                           │                           │                           │
  │           HTTP request to overridden endpoint         │                           │
  │ ◄─────────────────────    │                           │                           │
  │                           │                           │                           │
  │  response: 503            │                           │                           │
  │ ─────────────────────►    │                           │                           │
  │                           │  should_retry()           │                           │
  │                           │──┐                        │                           │
  │                           │  │ try_mark_endpoint_     │                           │
  │                           │  │ unavailable_for_       │                           │
  │                           │  │ pk_range               │                           │
  │                           │  └────────────────────►   │                           │
  │                           │                           │──┐ get_or_insert entry    │
  │                           │                           │  │ try_move_next_location │
  │                           │                           │◄─┘                        │
  │                           │       marked=true         │                           │
  │                           │ ◄─────────────────────    │                           │
  │                           │                           │                           │
  │                           │  should_retry_on_unavailable_endpoint_codes            │
  │                           │──┐ service_unavailable_   │                           │
  │                           │  │ retry_count++          │                           │
  │                           │◄─┘                        │                           │
  │                           │                           │                           │
  │  Retry { after: 0 }      │                           │                           │
  │ ◄─────────────────────    │                           │                           │
  │                           │                           │                           │
  │  next attempt (new        │                           │                           │
  │  endpoint from override)  │                           │                           │
  │ ─────────────────────►    │                           │                           │
  ...                         ...                         ...                         ...
```
