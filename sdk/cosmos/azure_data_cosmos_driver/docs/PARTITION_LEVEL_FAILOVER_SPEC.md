# Per-Partition Automatic Failover (PPAF) & Per-Partition Circuit Breaker (PPCB) Spec

**Status:** Draft / Iterating  
**Date:** 2026-03-11  
**Authors:** (team)  
**Crate:** `azure_data_cosmos_driver`

---

## Table of Contents

1. [Goals & Motivation](#1-goals--motivation)
2. [Architectural Overview](#2-architectural-overview)
3. [Feature Enablement & Configuration](#3-feature-enablement--configuration)
4. [Eligibility Rules](#4-eligibility-rules)
5. [Component Design](#5-component-design)
6. [Partition Failover Flow](#6-partition-failover-flow)
7. [Circuit Breaker Mechanics](#7-circuit-breaker-mechanics)
8. [Operation Pipeline Integration](#8-operation-pipeline-integration)
9. [Background Failback Loop](#9-background-failback-loop)
10. [Status Code Handling Matrix](#10-status-code-handling-matrix)
11. [Configuration Surface](#11-configuration-surface)
12. [Interaction with Account-Level Failover](#12-interaction-with-account-level-failover)
13. [Known Issues & Design Decisions](#13-known-issues--design-decisions)
14. [Test Coverage](#14-test-coverage)
15. [Prerequisites & Missing Pieces](#15-prerequisites--missing-pieces)

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
| Per-Partition Circuit Breaker | **PPCB** | **Reads** (any account), **Writes** on multi-master | **Multi-master** + all accounts for reads | Failure count exceeds threshold |

These two mechanisms are **mutually exclusive per request** — a given request uses
either the PPAF path or the PPCB path, never both. The decision is based on the
request's operation type (read vs. write) and whether the account supports multiple
write locations.

### Design Principles

- **Partition granularity**: Failover state is tracked per `(PartitionKeyRange, Region)` pair.
- **Threshold-gated**: The circuit breaker does not trip on the first failure. Failure
  counters must exceed configurable thresholds before a partition is failed over.
- **Gradual failback**: After a configurable unavailability window, failed
  partitions transition to a `ProbeCandidate` state. A single probe request is
  routed to the original region; only on success is the partition marked healthy.
  This avoids "opening the flood gate" for all traffic at once.
- **Environment-variable configurable**: All thresholds, windows, and intervals are
  overridable via environment variables for testing and operational flexibility.
- **No control-plane dependency**: Failover decisions are made locally by the SDK based
  on observed request failures — no server-side signal is required beyond the HTTP
  status codes.

---

## 2. Architectural Overview

### Driver Architecture vs. SDK Architecture

The driver uses a fundamentally different architecture from the `azure_data_cosmos`
SDK. Where the SDK uses a `ClientRetryPolicy` (azure\_core pipeline policy) with
`before_send_request()` / `should_retry()` callbacks and a separate
`GlobalPartitionEndpointManager` with `RwLock<HashMap>` maps, the driver instead uses:

- A **7-stage operation loop** (`execute_operation_pipeline`) that drives retry
- **Pure evaluation functions** (`evaluate_transport_result`) that emit effects
- A **`LocationEffect` system** that decouples failure classification from state mutation
- **Lock-free CAS state** via `crossbeam_epoch::Atomic<T>` in `LocationStateStore`
- **Immutable state snapshots** (`LocationSnapshot`) consumed by each loop iteration

The partition-level failover state lives in `PartitionEndpointState`, which is
managed alongside `AccountEndpointState` inside `LocationStateStore` using the
same lock-free pattern.

```mermaid
flowchart TD
    subgraph Pipeline["execute_operation_pipeline (7-stage loop)"]
        direction TB
        S1["<b>STAGE 1</b>: Acquire LocationSnapshot (account + partition state)"]
        S2["<b>STAGE 2</b>: resolve_endpoint()<br/>• Account-level endpoint selection (existing)<br/>• Partition-level override (NEW: consult PartitionEndpointState)"]
        S3["<b>STAGE 3</b>: Build TransportRequest"]
        S4["<b>STAGE 4</b>: Execute via transport pipeline → TransportResult"]
        S5["<b>STAGE 5</b>: evaluate_transport_result() → (OperationAction, Vec&lt;Effect&gt;)<br/>• 403/3 → FailoverRetry + MarkPartitionUnavailable (PPAF/PPCB)<br/>• 503 / 429/3092 / 410 → FailoverRetry + MarkPartitionUnavailable<br/>• Eligibility encoded in OperationRetryState + snapshot flags"]
        S6["<b>STAGE 6</b>: location_state_store.apply(effects)<br/>• MarkEndpointUnavailable → CAS on AccountEndpointState<br/>• MarkPartitionUnavailable → CAS on PartitionEndpointState (NEW)<br/>• RefreshAccountProperties → async refresh"]
        S7["<b>STAGE 7</b>: Act on OperationAction (Complete / FailoverRetry / Abort)"]
        S1 --> S2 --> S3 --> S4 --> S5 --> S6 --> S7
    end
    subgraph Store["LocationStateStore — Lock-free CAS via crossbeam_epoch::Atomic&lt;T&gt;"]
        direction TB
        Acct["<b>AccountEndpointState (existing)</b><br/>preferred_read_endpoints: Vec&lt;CosmosEndpoint&gt;<br/>preferred_write_endpoints: Vec&lt;CosmosEndpoint&gt;<br/>unavailable_endpoints: HashMap&lt;CosmosEndpoint, (Instant, Reason)&gt;<br/>multiple_write_locations_enabled: bool"]
        Part["<b>PartitionEndpointState (NEW — replaces empty placeholder)</b><br/>failover_overrides: HashMap&lt;PartitionKeyRangeId, ...&gt;<br/>circuit_breaker_overrides: HashMap&lt;PartitionKeyRangeId, ...&gt;<br/>per_partition_automatic_failover_enabled: bool (AccountProperties)<br/>per_partition_circuit_breaker_enabled: bool (options + AccountProps)"]
        Methods["<b>Methods</b><br/>• snapshot() → LocationSnapshot { account, partitions }<br/>• apply(effects) → CAS on account and/or partition state<br/>• apply_partition(f) → CAS loop on PartitionEndpointState<br/>• sync_account_properties() → also updates PPAF/PPCB flags"]
        BG["<b>Background</b><br/>Failback task spawned via BackgroundTaskManager<br/>(Weak ref, periodic sweep)"]
    end
    Pipeline --> Store
```

### Request Flow Summary

1. **Endpoint resolution** (Stage 2 — `resolve_endpoint`):
   - Select account-level endpoint from `AccountEndpointState` (existing logic).
   - If partition-level failover is enabled and a `partition_key_range_id` is
     available on the `OperationRetryState`, consult `PartitionEndpointState`
     for an override. If found and threshold conditions are met, use the
     partition-level override endpoint instead.

2. **Failure evaluation** (Stage 5 — `evaluate_transport_result`):
   - Classify the response status code.
   - Emit `LocationEffect::MarkPartitionUnavailable` for eligible status codes
     (403/3, 503, 429/3092, 410). This effect carries the `partition_key_range_id`,
     the failed endpoint's region, and whether the request was read-only.
   - Return `OperationAction::FailoverRetry` so the loop re-enters Stage 1,
     acquiring a fresh `LocationSnapshot` with the updated partition state.

3. **Effect application** (Stage 6 — `location_state_store.apply`):
   - `MarkPartitionUnavailable` → CAS loop on `PartitionEndpointState`:
     insert or update a `PartitionFailoverEntry`, advance to the next available
     endpoint in the preferred list.
   - `MarkEndpointUnavailable` → existing CAS on `AccountEndpointState`.
   - Both effects can be emitted simultaneously for 503/429/410 (partition
     marking for future requests + endpoint marking for account-level routing).

4. **Background failback**:
   - A periodic task scans all circuit-breaker `PartitionEndpointState`.
   - Entries whose `first_failure_time` exceeds the configured unavailability
     duration are removed via CAS, restoring default routing.

---

## 3. Feature Enablement & Configuration

### Enable/Disable Flags

| Flag | Source | Default | Description |
|---|---|---|---|
| `per_partition_circuit_breaker_enabled` | `DriverOptions` → env var `AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED` | `true` | Fallback enablement for PPCB when the server flag is not set. Read from `DriverOptions` at construction (currently backed by the env var). The effective PPCB value is `server_flag \|\| options_value`, so PPCB remains enabled if the server flag is `true` regardless of this option. |
| `per_partition_automatic_failover_enabled` | Server-side `AccountProperties.enable_per_partition_failover_behavior` | `false` | PPAF is enabled when the Cosmos DB account has this flag set. Updated dynamically on each account properties refresh. |

> **Configuration resolution**: The PPCB option is read from `DriverOptions` at
> construction time and stored in `PartitionFailoverConfig`. When the
> [Hierarchical Configuration Model](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos/docs/HierarchicalConfigModel.md) lands, this will
> be read from the layered options system (Environment → Runtime → Account →
> Request). Until then, the `DriverOptions` value is backed by `env::var`.

### Dynamic Reconfiguration

Both flags are stored as fields on `PartitionEndpointState` and updated atomically
via the CAS loop when account properties are refreshed:

- **PPAF**: Updated when `sync_account_properties()` processes a new
  `AccountProperties` response. When the server-side account property
  `enable_per_partition_failover_behavior` changes, the next CAS swap on
  `PartitionEndpointState` picks it up.

- **PPCB**: The effective value is:
  ```
  enable_per_partition_failover_behavior || options_circuit_breaker_enabled
  ```
  This means PPCB is enabled if **either** the server flag or the client-side
  option value is set to `true`.

### Initialization

```rust
// In CosmosDriver construction:

// 1. Build PartitionFailoverConfig from DriverOptions.
//    The circuit_breaker_option_enabled value comes from DriverOptions
//    (currently backed by env var AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED,
//     will use Hierarchical Configuration Model once landed).
let config = PartitionFailoverConfig::from_options(&driver_options);

// 2. Initial PartitionEndpointState (PPAF starts disabled — updated on
//    first account properties refresh)
let initial_partition_state = PartitionEndpointState {
    failover_overrides: HashMap::new(),
    circuit_breaker_overrides: HashMap::new(),
    per_partition_automatic_failover_enabled: false,
    per_partition_circuit_breaker_enabled: config.circuit_breaker_option_enabled,
    config,
};

// 3. LocationStateStore is initialized with this partition state
//    (replaces the current empty PartitionEndpointState placeholder)

// 4. On account properties refresh (in sync_account_properties):
//    - Read AccountProperties.enable_per_partition_failover_behavior
//    - CAS-update PartitionEndpointState with new flags:
//        per_partition_automatic_failover_enabled =
//            account_props.enable_per_partition_failover_behavior
//        per_partition_circuit_breaker_enabled =
//            account_props.enable_per_partition_failover_behavior
//            || current.config.circuit_breaker_option_enabled
```

---

## 4. Eligibility Rules

### 4.1 Per-Partition Automatic Failover (PPAF)

A request is eligible when **all** conditions are true:

1. `partition_state.per_partition_automatic_failover_enabled == true`
2. The operation is a **write** (`!operation.is_read_only()`)
3. The account is **single-master** (`!account_state.multiple_write_locations_enabled`)

```rust
// Pure eligibility check — evaluated in resolve_endpoint() and in
// mark_partition_unavailable() routing system
fn is_eligible_for_ppaf(
    partition_state: &PartitionEndpointState,
    account_state: &AccountEndpointState,
    operation: &CosmosOperation,
) -> bool {
    partition_state.per_partition_automatic_failover_enabled
        && !operation.is_read_only()
        && !account_state.multiple_write_locations_enabled
}
```

**Rationale**: On a single-master account, the write region is fixed. When a specific
partition's write endpoint returns 403/3 (WriteForbidden), the **service** has already
decided to redirect writes for that partition to a different region. PPAF is not
"done" by the SDK — it is a service-side decision. The SDK's role is to understand
the hints from the service (the 403/3 status code) and route subsequent requests for
that partition to the next available read region.

### 4.2 Per-Partition Circuit Breaker (PPCB)

A request is eligible when **all** conditions are true:

1. `partition_state.per_partition_circuit_breaker_enabled == true`
2. The operation targets `ResourceType::Document` or
   `ResourceType::StoredProcedure` with `OperationType::Execute`
3. The operation is **either**:
   - A **read** (`operation.is_read_only()`), **or**
   - A **write** on a **multi-master** account

```rust
fn is_eligible_for_ppcb(
    partition_state: &PartitionEndpointState,
    account_state: &AccountEndpointState,
    operation: &CosmosOperation,
) -> bool {
    partition_state.per_partition_circuit_breaker_enabled
        && operation.resource_type().is_partitioned()
        && (operation.is_read_only()
            || account_state.multiple_write_locations_enabled)
}
```

**Rationale**: Multi-master accounts treat all regions as write regions. The circuit
breaker path handles both reads and writes by tracking failure counts per
partition and failing over to the next preferred region when the count exceeds
the threshold.

### 4.3 Shared Pre-Conditions

Both mechanisms share additional validation:

1. At least one of PPAF or PPCB must be enabled on `PartitionEndpointState`.
2. The operation must target a resource type that supports partition-level failover
   (Documents, or StoredProcedures+Execute) — checked via
   `operation.resource_type().is_partitioned()`.
3. There must be **more than one preferred read endpoint** in `AccountEndpointState`
   (otherwise there is nowhere to fail over to).
4. A resolved `partition_key_range_id` must be available on `OperationRetryState`
   (see [§15 Prerequisites](#15-prerequisites--missing-pieces)).
5. When marking a failed location, the endpoint that failed must be known from
   the `RoutingDecision` used for that attempt.

---

## 5. Component Design

### 5.1 `PartitionEndpointState` (replaces empty placeholder)

The central structure for partition-level failover state. Resides in
`src/driver/routing/partition_endpoint_state.rs` and is managed by
`LocationStateStore` via the same lock-free CAS pattern used for
`AccountEndpointState`.

**Key design decision**: Unlike the SDK's `GlobalPartitionEndpointManager` which
stores two `RwLock<HashMap>` with interior mutability, the driver follows its
existing immutable-snapshot pattern. `PartitionEndpointState` is a plain `Clone`
struct. Mutations create a new instance and swap it atomically via
`crossbeam_epoch`. This eliminates reader/writer contention on the hot path.

```rust
/// Immutable partition-level endpoint routing state.
///
/// Managed via CAS in LocationStateStore alongside AccountEndpointState.
#[derive(Clone, Debug)]
pub(crate) struct PartitionEndpointState {
    /// PPAF map: writes on single-master accounts.
    /// Key: partition key range ID.
    pub failover_overrides: HashMap<PartitionKeyRangeId, PartitionFailoverEntry>,

    /// PPCB map: reads (any account) + writes on multi-master.
    /// Key: partition key range ID.
    pub circuit_breaker_overrides: HashMap<PartitionKeyRangeId, PartitionFailoverEntry>,

    /// PPAF enabled (from AccountProperties.enable_per_partition_failover_behavior).
    pub per_partition_automatic_failover_enabled: bool,

    /// PPCB enabled (from options + account property).
    pub per_partition_circuit_breaker_enabled: bool,

    /// Configuration read from DriverOptions at construction time.
    /// Includes `circuit_breaker_option_enabled` for recomputation on account refresh.
    pub config: PartitionFailoverConfig,
}
```

**Why two maps?** Single-master accounts have a distinct write endpoint that differs
from the read endpoints. When a write partition fails over on a single-master
account, it must use the account-level read endpoints (full
`preferred_read_endpoints` list). On multi-master accounts, all regions serve both
reads and writes, so the preferred read endpoints are used for both. Keeping
separate maps avoids cross-contamination between these two routing strategies.

### 5.2 `PartitionFailoverEntry`

Per-partition failover tracking state. One instance exists per failed-over partition
key range.

```rust
/// Per-partition failover entry.
///
/// Immutable value — mutations produce a new instance via CAS.
#[derive(Clone, Debug)]
pub(crate) struct PartitionFailoverEntry {
    // ── Routing State ──────────────────────────────────────────
    /// Current endpoint this partition is routed to.
    pub current_endpoint: CosmosEndpoint,
    /// Original endpoint that first failed (used for failback).
    pub first_failed_endpoint: CosmosEndpoint,
    /// Set of endpoints already tried.
    pub failed_endpoints: HashSet<CosmosEndpoint>,

    // ── Failure Counters ───────────────────────────────────────
    /// Read failure count (not necessarily consecutive — see §13.2).
    pub read_failure_count: i32,
    /// Write failure count (not necessarily consecutive — see §13.2).
    pub write_failure_count: i32,

    // ── Timestamps ─────────────────────────────────────────────
    /// When the first failure occurred (for failback eligibility).
    pub first_failure_time: Instant,
    /// When the most recent failure occurred (for counter reset).
    pub last_failure_time: Instant,
}
```

**Immutability note**: In the SDK, `PartitionKeyRangeFailoverInfo` uses `AtomicI32`
for counters and `RwLock<Instant>` for timestamps because it is mutated in-place
behind an `RwLock<HashMap>`. In the driver, since the entire
`PartitionEndpointState` is swapped atomically via CAS, counters and timestamps are
plain values. Each CAS update produces a new `PartitionFailoverEntry` with
incremented/updated fields.

### 5.3 `PartitionFailoverConfig`

Configuration values read from `DriverOptions` at driver construction time.

```rust
/// Configuration for partition-level failover, read once at construction.
#[derive(Clone, Debug)]
pub(crate) struct PartitionFailoverConfig {
    /// PPCB option value from DriverOptions (default: true).
    /// Retained for recomputation on account refresh:
    ///   effective_ppcb = server_flag || circuit_breaker_option_enabled
    /// Source: DriverOptions (currently backed by env var
    ///   AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED).
    pub circuit_breaker_option_enabled: bool,

    /// Read failures before circuit trips (default: 2).
    /// Env: AZURE_COSMOS_CIRCUIT_BREAKER_FAILURE_COUNT_FOR_READS
    pub read_failure_threshold: i32,

    /// Write failures before circuit trips (default: 5).
    /// Env: AZURE_COSMOS_CIRCUIT_BREAKER_FAILURE_COUNT_FOR_WRITES
    pub write_failure_threshold: i32,

    /// Window after which failure counters reset (default: 5 minutes).
    /// Env: AZURE_COSMOS_CIRCUIT_BREAKER_TIMEOUT_COUNTER_RESET_WINDOW_IN_MINUTES
    pub counter_reset_window: Duration,

    /// Duration a partition must remain unavailable before failback (default: 5s).
    /// Env: AZURE_COSMOS_ALLOWED_PARTITION_UNAVAILABILITY_DURATION_IN_SECONDS
    pub partition_unavailability_duration: Duration,

    /// Interval for the background failback sweep (default: 300s).
    /// Env: AZURE_COSMOS_PPCB_STALE_PARTITION_UNAVAILABILITY_REFRESH_INTERVAL_IN_SECONDS
    pub failback_sweep_interval: Duration,
}
```

### 5.4 Partition Key Range Identity

The key type used to identify partitions in the failover maps is
`PartitionKeyRangeId` — a newtype wrapping a `String`:

```rust
/// Identifies a physical partition key range.
///
/// Newtype wrapper around the raw string ID from the
/// `x-ms-documentdb-partitionkeyrangeid` response header.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct PartitionKeyRangeId(String);

impl PartitionKeyRangeId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for PartitionKeyRangeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::str::FromStr for PartitionKeyRangeId {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}
```

This is simpler than the SDK's `PartitionKeyRange` struct (which also carries
`min_inclusive`/`max_exclusive`) because the driver only needs the ID for map
lookups. Using a newtype rather than a bare `String` prevents accidental
misuse (e.g., passing an account ID where a partition key range ID is expected).

**Source of the partition key range ID**: In gateway mode, the gateway resolves the
physical partition for each request. The partition key range ID is returned in the
response header `x-ms-documentdb-partitionkeyrangeid`. On the first attempt (success
or failure), the driver captures this ID and stores it in `OperationRetryState` for
use in subsequent retry attempts. See [§15 Prerequisites](#15-prerequisites--missing-pieces)
for details.

### 5.5 Existing Driver Components (Modified)

The following existing driver types require modifications for PPAF/PPCB:

| Type | File | Modification |
|------|------|-------------|
| `LocationStateStore` | `routing/location_state_store.rs` | Replace empty `PartitionEndpointState`; add `apply_partition()` CAS method; wire failback loop; update `sync_account_properties()` to update PPAF/PPCB flags |
| `LocationSnapshot` | `routing/location_state_store.rs` | `partitions` field becomes meaningful (currently always `Arc::new(PartitionEndpointState)`) |
| `LocationEffect::MarkPartitionUnavailable` | `routing/location_effects.rs` | Remove `#[allow(dead_code)]`; handled in `apply()` |
| `UnavailablePartition` | `routing/location_effects.rs` | Remove `#[allow(dead_code)]` from fields |
| `OperationRetryState` | `pipeline/components.rs` | Add `partition_key_range_id: Option<PartitionKeyRangeId>` field |
| `evaluate_transport_result` | `pipeline/retry_evaluation.rs` | Wire actual `partition_key_range_id` from `OperationRetryState` (replace `String::new()` TODO) |
| `resolve_endpoint` | `pipeline/operation_pipeline.rs` | Consult `PartitionEndpointState` for partition-level override |
| `execute_operation_pipeline` | `pipeline/operation_pipeline.rs` | Capture `partition_key_range_id` from response headers and store in retry state |

---

## 6. Partition Failover Flow

### 6.1 Override Application (Stage 2 — `resolve_endpoint`)

On every loop iteration, `resolve_endpoint()` checks for existing partition-level
overrides after selecting the account-level endpoint:

```mermaid
flowchart TD
    Start["resolve_endpoint(operation, retry_state, location_snapshot, ttl)"]
    Acct["Select account-level endpoint<br/>(existing logic: preferred_endpoints → skip excluded/unavailable → fallback to default)"]
    Q1{"partition_key_range_id<br/>available on retry_state?"}
    Q2{"eligible for PPCB<br/>(is_eligible_for_ppcb)?"}
    Cb["lookup in partitions.circuit_breaker_overrides[pk_range_id]"]
    CbT{"entry found AND<br/>threshold exceeded<br/>(can_circuit_breaker_trigger_failover)?"}
    CbOver["override endpoint → entry.current_endpoint"]
    CbNo["no override<br/>(continue to account-level endpoint)"]
    Q3{"eligible for PPAF<br/>(is_eligible_for_ppaf)?"}
    Pa["lookup in partitions.failover_overrides[pk_range_id]"]
    PaT{"entry found?"}
    PaOver["override endpoint → entry.current_endpoint"]
    Done["use account-level endpoint"]
    Start --> Acct --> Q1
    Q1 -- no --> Done
    Q1 -- yes --> Q2
    Q2 -- yes --> Cb --> CbT
    CbT -- yes --> CbOver
    CbT -- no --> CbNo --> Done
    Q2 -- no --> Q3
    Q3 -- yes --> Pa --> PaT
    PaT -- yes --> PaOver
    PaT -- no --> Done
    Q3 -- no --> Done
```

**Key difference**: PPAF overrides unconditionally when an entry exists. PPCB
additionally checks `can_circuit_breaker_trigger_failover()` — the threshold
gate — before applying the override. This means PPCB requires the failure count to
exceed the threshold before the partition is actually routed to the alternate region,
even if a failover entry already exists.

### 6.2 Marking Partition Unavailable (Stage 6 — `apply`)

When `evaluate_transport_result()` emits `LocationEffect::MarkPartitionUnavailable`,
`LocationStateStore::apply()` processes it via a CAS loop on `PartitionEndpointState`:

```mermaid
flowchart TD
    Start["apply(effects)"]
    Loop["for each MarkPartitionUnavailable(unavailable_partition):"]
    Apply["apply_partition(|current_state, account_state| {<br/>&nbsp;&nbsp;mark_partition_unavailable(<br/>&nbsp;&nbsp;&nbsp;&nbsp;current_state,<br/>&nbsp;&nbsp;&nbsp;&nbsp;account_state,<br/>&nbsp;&nbsp;&nbsp;&nbsp;&amp;unavailable_partition,<br/>&nbsp;&nbsp;)<br/>})"]
    Other["[other effects:<br/>MarkEndpointUnavailable,<br/>RefreshAccountProperties]"]
    Start --> Loop --> Apply
    Start --> Other
```

### 6.3 `mark_partition_unavailable` (Pure Routing System Function)

A pure function in `routing_systems.rs` that produces a new `PartitionEndpointState`:

```mermaid
flowchart TD
    Start["mark_partition_unavailable(current_state, account_state, unavailable_partition)"]
    Decide["Determine mechanism and target map:<br/>• if eligible for PPCB → use circuit_breaker_overrides<br/>&nbsp;&nbsp;next_endpoints = account_state.preferred_read_endpoints<br/>• else if eligible for PPAF → use failover_overrides<br/>&nbsp;&nbsp;next_endpoints = account_state.preferred_read_endpoints<br/>&nbsp;&nbsp;(full account read list for single-master write failover)"]
    Clone["Clone current_state → new_state"]
    Insert["Get or insert PartitionFailoverEntry in target map<br/>(new entry: current = failed_endpoint, first_failed = failed_endpoint)"]
    Counters["For PPCB: increment failure counter + check reset window:<br/>• if (now - last_failure_time) &gt; counter_reset_window → reset both counters to 0<br/>• increment read or write counter"]
    ThreshGate{"PPCB threshold exceeded?"}
    NoMove["return new_state (no endpoint move)"]
    Move["try_move_next_endpoint(entry, next_endpoints, failed_endpoint):<br/>• if failed_endpoint != entry.current_endpoint → return true (concurrent CAS already moved it)<br/>• for each endpoint in next_endpoints:<br/>&nbsp;&nbsp;– skip if endpoint == current<br/>&nbsp;&nbsp;– skip if endpoint already in failed_endpoints set<br/>&nbsp;&nbsp;– found! → add current to failed_endpoints, set current_endpoint = new endpoint, return true<br/>• return false (all endpoints exhausted)"]
    MoveQ{"moved?"}
    Updated["return new_state with updated entry"]
    Exhausted["remove entry from map, return new_state<br/>(partition returns to default routing on next snapshot)"]
    Start --> Decide --> Clone --> Insert --> Counters --> ThreshGate
    ThreshGate -- "PPCB,<br/>not exceeded" --> NoMove
    ThreshGate -- "exceeded /<br/>PPAF" --> Move --> MoveQ
    MoveQ -- yes --> Updated
    MoveQ -- no --> Exhausted
```

---

## 7. Circuit Breaker Mechanics

### 7.1 Failure Counter Tracking

The circuit breaker maintains per-partition failure counters. The counters
are incremented on each failure and checked against configurable thresholds.

> **Note on naming**: The environment variables use the term "failure count" (e.g.,
> `AZURE_COSMOS_CIRCUIT_BREAKER_FAILURE_COUNT_FOR_READS`).
> These are not strictly consecutive: in the CAS model, a lost CAS can cause a
> counter increment to be dropped (see §13.2), and successful requests between
> failures do not reset the counter. Only the timeout window (§7.3) resets counters.

```mermaid
flowchart TD
    Start["increment_request_failure_counter_and_check_if_partition_can_failover(request)"]
    Validate["Validate eligibility and extract partition key range + failed location"]
    GetOrInsert["Get or insert PartitionKeyRangeFailoverInfo in the appropriate map"]
    IncCounts["increment_request_failure_counts(is_read_only, current_time):<br/>• if (current_time - last_failure_time) &gt; timeout_counter_reset_window → reset both read and write counters to 0<br/>• if is_read_only → read_failure_count += 1<br/>• else → write_failure_count += 1<br/>• update last_request_failure_time = current_time"]
    Check["can_circuit_breaker_trigger_partition_failover(is_read_only):<br/>• if is_read_only → return read_count &gt; read_threshold (default: 2)<br/>• else → return write_count &gt; write_threshold (default: 5)"]
    Start --> Validate --> GetOrInsert --> IncCounts --> Check
```

### 7.2 Threshold Configuration

| Parameter | Default | Environment Variable |
|---|---|---|
| Read failure threshold | 2 | `AZURE_COSMOS_CIRCUIT_BREAKER_FAILURE_COUNT_FOR_READS` |
| Write failure threshold | 5 | `AZURE_COSMOS_CIRCUIT_BREAKER_FAILURE_COUNT_FOR_WRITES` |
| Counter reset window | 5 minutes | `AZURE_COSMOS_CIRCUIT_BREAKER_TIMEOUT_COUNTER_RESET_WINDOW_IN_MINUTES` |

**Why reads = 2, writes = 5?** Reads are idempotent and safe to fail over
aggressively. Writes are more expensive to fail over (potential double-write risk on
multi-master), so a higher threshold reduces false-positive failovers due to transient
errors.

### 7.3 Counter Reset Window

If the time between two failures exceeds `timeout_counter_reset_window`
(default: 5 minutes), **both** read and write counters are reset to zero before the
new failure is recorded. This prevents stale failures from accumulating across long
idle periods — if a partition has been healthy for 5 minutes, any new failure starts
the counter fresh.

### 7.4 Circuit Breaker State Transitions

```mermaid
stateDiagram-v2
    [*] --> HEALTHY
    state "HEALTHY\n(no entry in failover map)" as HEALTHY
    state "COUNTING\n(entry exists, threshold NOT exceeded, counter++)" as COUNTING
    state "TRIPPED\n(entry.current = next region, override applied)" as TRIPPED
    state "PROBE_CANDIDATE\n(single request probes original region)" as PROBE_CANDIDATE

    HEALTHY --> COUNTING: (1) first failure
    COUNTING --> TRIPPED: (2) failure count > threshold
    COUNTING --> HEALTHY: (3) failback removes entry
    TRIPPED --> HEALTHY: (4) all locations exhausted → entry removed
    TRIPPED --> PROBE_CANDIDATE: (5) unavailable duration exceeded
    PROBE_CANDIDATE --> HEALTHY: (5a) probe succeeds
    PROBE_CANDIDATE --> TRIPPED: (5b) probe fails → reset
    TRIPPED --> TRIPPED: (6) next region also fails: move to subsequent region
```

**Transitions:**

| # | From | To | Trigger |
|---|---|---|---|
| 1 | HEALTHY | COUNTING | First failure creates an entry in the failover map; counter incremented but below threshold. |
| 2 | COUNTING | TRIPPED | Counter exceeds threshold; `try_mark_endpoint_unavailable_for_partition_key_range()` moves the partition to the next region; override is now applied on subsequent requests. |
| 3 | COUNTING | HEALTHY | Background failback loop removes the entry after `partition_unavailability_duration` elapses (threshold was never reached). |
| 4 | TRIPPED | HEALTHY | All locations exhausted in `try_move_next_location()`; entry is removed from the map and the partition returns to default routing. |
| 5 | TRIPPED | PROBE_CANDIDATE | Background failback loop transitions the entry to `ProbeCandidate` after `partition_unavailability_duration` elapses. |
| 5a | PROBE_CANDIDATE | HEALTHY | Next request for this partition is routed to the original region as a probe. If it succeeds, the entry is removed. |
| 5b | PROBE_CANDIDATE | TRIPPED | Probe request fails → return to `Unhealthy`, reset timer. Will be probed again after next unavailability window. |
| 6 | TRIPPED | TRIPPED | Alternate region also fails; `try_move_next_location()` advances to the next available region. |

---

## 8. Operation Pipeline Integration

The driver does not have a `ClientRetryPolicy`. Instead, PPAF/PPCB integrates with
the 7-stage operation loop (`execute_operation_pipeline`) and the pure retry
evaluation function (`evaluate_transport_result`).

### 8.1 Integration Points in the Operation Loop

#### 8.1.1 Stage 2: Endpoint Resolution with Partition Override

```rust
// In resolve_endpoint():
fn resolve_endpoint(
    operation: &CosmosOperation,
    retry_state: &OperationRetryState,
    location: &LocationSnapshot,
    endpoint_unavailability_ttl: Duration,
) -> RoutingDecision {
    // 1. Existing account-level endpoint selection
    let account_endpoint = /* existing logic */;

    // 2. NEW: Partition-level override check
    if let Some(pk_range_id) = &retry_state.partition_key_range_id {
        let partitions = location.partitions.as_ref();
        let account = location.account.as_ref();

        if is_eligible_for_ppcb(partitions, account, operation) {
            if let Some(entry) = partitions.circuit_breaker_overrides.get(pk_range_id) {
                if can_circuit_breaker_trigger_failover(
                    entry,
                    operation.is_read_only(),
                    &partitions.config,
                ) {
                    return RoutingDecision {
                        endpoint: entry.current_endpoint.clone(),
                    };
                }
            }
        } else if is_eligible_for_ppaf(partitions, account, operation) {
            if let Some(entry) = partitions.failover_overrides.get(pk_range_id) {
                return RoutingDecision {
                    endpoint: entry.current_endpoint.clone(),
                };
            }
        }
    }

    RoutingDecision { endpoint: account_endpoint }
}
```

#### 8.1.2 Stage 4→5: Capturing Partition Key Range ID

After the transport pipeline returns (Stage 4), the response headers may contain
`x-ms-documentdb-partitionkeyrangeid`. This is captured and stored in
`OperationRetryState` for use in subsequent retry attempts:

```rust
// In execute_operation_pipeline(), after Stage 4:
if retry_state.partition_key_range_id.is_none() {
    if let Some(pk_range_id) = result.partition_key_range_id_from_headers() {
        retry_state.partition_key_range_id = Some(pk_range_id);
    }
}
```

This means that on the **first attempt**, no partition-level override is possible
(the partition key range ID is not yet known). Partition-level routing takes effect
starting from the **second attempt** (first retry).

#### 8.1.3 Stage 5: Retry Evaluation Emits Partition Effects

`evaluate_transport_result()` already emits `LocationEffect::MarkPartitionUnavailable`
for 503/429/410. The change is to wire the actual `partition_key_range_id` from
`OperationRetryState` instead of the current `String::new()` placeholder:

```rust
// In evaluate_transport_result(), for 503/429/410/gone:
LocationEffect::MarkPartitionUnavailable(UnavailablePartition {
    partition_key_range_id: retry_state
        .partition_key_range_id
        .clone()
        .unwrap_or_default(),
    region: endpoint.region().cloned(),
    is_read: operation.is_read_only(),
})
```

For 403/3 WriteForbidden, the effect list is extended to also emit
`MarkPartitionUnavailable` (currently only `MarkEndpointUnavailable` +
`RefreshAccountProperties` is emitted):

```rust
// In evaluate_transport_result(), 403/3 branch:
if status.is_write_forbidden() && retry_state.can_retry_failover() {
    return (
        OperationAction::FailoverRetry { .. },
        vec![
            LocationEffect::RefreshAccountProperties,
            LocationEffect::MarkEndpointUnavailable { .. },
            LocationEffect::MarkPartitionUnavailable(UnavailablePartition {
                partition_key_range_id: retry_state
                    .partition_key_range_id
                    .clone()
                    .unwrap_or_default(),
                region: endpoint.region().cloned(),
                is_read: false, // WriteForbidden is always a write
            }),
        ],
    );
}
```

#### 8.1.4 Stage 6: Effect Application

`LocationStateStore::apply()` handles the previously-ignored
`MarkPartitionUnavailable` effect:

```rust
// In LocationStateStore::apply():
LocationEffect::MarkPartitionUnavailable(partition) => {
    if partition.partition_key_range_id.is_empty() {
        // No partition key range ID available (first attempt);
        // skip partition-level marking.
        continue;
    }
    self.apply_partition(|current_partitions| {
        mark_partition_unavailable(
            current_partitions,
            &self.account_snapshot(),
            &partition,
        )
    });
}
```

### 8.2 `OperationRetryState` Changes

```rust
pub(crate) struct OperationRetryState {
    // ... existing fields ...

    /// Partition key range ID resolved from the first response.
    /// None until the first transport attempt returns headers.
    pub partition_key_range_id: Option<PartitionKeyRangeId>,
}
```

### 8.3 `evaluate_transport_result` Signature Change

The function needs access to the partition key range ID for building the
`UnavailablePartition` effect. Two options:

**Option A** (preferred): Pass `retry_state` to `evaluate_transport_result`:

```rust
pub(crate) fn evaluate_transport_result(
    operation: &CosmosOperation,
    endpoint: &CosmosEndpoint,
    result: TransportResult,
    retry_state: &OperationRetryState,  // already passed
) -> (OperationAction, Vec<LocationEffect>)
```

The function already receives `retry_state` — the partition key range ID is
simply read from it.

**Option B**: Pass the partition key range ID separately. Avoided because it
adds a parameter for a value already available on `retry_state`.

### 8.4 Difference from SDK Retry Policy

In the SDK's `ClientRetryPolicy`, the 403/3 handling has a **priority gate**: if
the partition-level mark succeeds, the request retries immediately without falling
through to account-level failover. In the driver, this priority is naturally
achieved because `evaluate_transport_result` emits all effects at once (both
`MarkPartitionUnavailable` and `MarkEndpointUnavailable` + `RefreshAccountProperties`),
and `apply()` processes them all in Stage 6. The retry loop then re-enters Stage 1
with a fresh snapshot that has both updates.

---

## 9. Background Failback Loop

### 9.1 Loop Structure

The background failback loop is spawned via the driver's
`BackgroundTaskManager` (from `driver::transport::background_task_manager`)
during `LocationStateStore` construction (or on first PPAF/PPCB enablement).
The spawned future holds a `Weak` reference to `LocationStateStore` to avoid
preventing the store from being dropped when the driver is dropped.

Using `BackgroundTaskManager` provides:
- **Abort-on-drop** — when the manager is dropped, the failback task is
  automatically aborted, preventing orphaned background work.
- **Panic safety** — if the failback future panics, `BackgroundTaskManager`
  catches the panic and logs it instead of propagating to the runtime.
- **Graceful shutdown** — `BackgroundTaskManager::shutdown()` can be called
  to abort and await all background tasks before driver teardown.

```rust
// In LocationStateStore construction:
let weak_store: Weak<LocationStateStore> = Arc::downgrade(&store);
let config = partition_config.clone();

background_task_manager.spawn(async move {
    failback_loop(weak_store, config).await;
});

// The failback loop itself:
async fn failback_loop(
    weak_store: Weak<LocationStateStore>,
    config: PartitionFailoverConfig,
) {
    loop {
        tokio::time::sleep(config.failback_sweep_interval).await;

        let Some(store) = weak_store.upgrade() else {
            // LocationStateStore was dropped — exit the loop.
            break;
        };

        store.apply_partition(|current_partitions| {
            expire_partition_overrides(
                current_partitions,
                Instant::now(),
                config.partition_unavailability_duration,
            )
        });
    }
}
```

**Lifecycle**: The `BackgroundTaskManager` instance is owned by
`LocationStateStore` (or its parent `CosmosDriverRuntime`). When the store is
dropped, the manager's `Drop` impl aborts all spawned tasks — including the
failback loop — ensuring no leaked background work. The `Weak` reference
provides an additional safety layer: even if abort delivery is delayed, the
loop will exit on the next iteration when `Weak::upgrade()` returns `None`.

### 9.2 `expire_partition_overrides` (Pure Routing System Function)

A pure function in `routing_systems.rs`:

```mermaid
flowchart TD
    Start["expire_partition_overrides(state, now, unavailability_duration) → PartitionEndpointState"]
    Clone["Clone state → new_state"]
    Cb["Scan new_state.circuit_breaker_overrides:<br/>For entries where (now - entry.first_failure_time) &gt; unavailability_duration<br/>AND entry.health_status == Unhealthy:<br/>→ Transition entry.health_status → ProbeCandidate"]
    Pa["Scan new_state.failover_overrides:<br/>For entries where (now - entry.first_failure_time) &gt; unavailability_duration<br/>AND entry.health_status == Unhealthy:<br/>→ Transition entry.health_status → ProbeCandidate"]
    Done["Return new_state"]
    Start --> Clone --> Cb --> Pa --> Done
```

**Note**: Unlike the SDK, the driver's failback loop scans **both** maps (PPAF and
PPCB). This is a deliberate improvement — in the SDK, PPAF entries are only removed
when all locations are exhausted. The driver's immutable-snapshot pattern makes it
trivial to sweep both maps in the same CAS operation.

### 9.3 Failback Timing

| Parameter | Default | Environment Variable |
|---|---|---|
| Unavailability duration before failback | 5 seconds | `AZURE_COSMOS_ALLOWED_PARTITION_UNAVAILABILITY_DURATION_IN_SECONDS` |
| Background sweep interval | 300 seconds | `AZURE_COSMOS_PPCB_STALE_PARTITION_UNAVAILABILITY_REFRESH_INTERVAL_IN_SECONDS` |

**Interaction**: A partition must have been unavailable for at least 5 seconds
before the failback loop considers it. However, since the loop only runs every
300 seconds by default, the effective failback time is between 5 and 305 seconds.

### 9.4 Gradual Failback (Probe-Based Recovery)

Rather than abruptly redirecting all traffic back to the original region, the
driver should employ a **staged failback** approach to avoid cascading failures
if the original region has not fully recovered.

#### Partition Health States

Each `PartitionFailoverEntry` tracks a `health_status` field:

| State | Description |
|---|---|
| `Unhealthy` | Partition is failed-over to an alternate region. All requests route to the override endpoint. |
| `ProbeCandidate` | Unavailability window has elapsed. The **next single request** for this partition is tentatively routed back to the original region as a health probe. |
| `Healthy` | Probe succeeded. Entry is removed; future requests use default routing. |

> **Relationship to §7.4 circuit breaker states**: The state diagram in §7.4
> shows 4 *logical* states (HEALTHY, COUNTING, TRIPPED, PROBE_CANDIDATE) that
> describe the full circuit breaker lifecycle. These 3 `health_status` values
> map to them as follows:
>
> - **HEALTHY** (§7.4) = no entry in the map (no `health_status` to store).
>   The `Healthy` value here is a transient outcome that triggers entry removal.
> - **COUNTING** and **TRIPPED** (§7.4) both correspond to `Unhealthy`. The
>   distinction between them is derived from comparing failure counters against
>   thresholds, not from the `health_status` field.
> - **PROBE_CANDIDATE** (§7.4) = `ProbeCandidate`.

#### Failback Flow

```mermaid
flowchart TD
    subgraph Sweep["Background failback sweep"]
        S1["For each entry where status == Unhealthy:<br/>if (now - first_failure_time) &gt; unavailability_duration<br/>→ Transition to ProbeCandidate"]
        S2["ProbeCandidate entries are left in the map<br/>for resolve_endpoint to act on"]
        S1 --> S2
    end
    subgraph Resolve["resolve_endpoint()"]
        R1["if entry exists and entry.health_status == ProbeCandidate:<br/>Route this ONE request to the original region (first_failed_endpoint)<br/>(subsequent requests continue to the override endpoint until the probe result is known)"]
    end
    subgraph Apply["evaluate_transport_result() → apply()"]
        A1{"probe request result?"}
        A2["SUCCEEDED:<br/>Remove entry from map → partition returns to Healthy"]
        A3["FAILED:<br/>Transition back to Unhealthy, reset first_failure_time<br/>(will be probed again after the next unavailability window)"]
        A1 -- success --> A2
        A1 -- failure --> A3
    end
    Sweep --> Resolve --> Apply
```

#### Rationale

This approach addresses the concern raised by reviewers that "opening the flood
gate" for all requests at once is unsafe. By sending a single probe request first:

- **Reduced blast radius**: Only one request pays the latency cost if the region
  is still unhealthy.
- **Gradual confidence**: The probe validates that the original region is serving
  the partition before restoring full traffic.
- **No active probing cost**: The probe piggybacks on a real user request rather
  than requiring synthetic health checks.

> **Future enhancement**: If the single-probe model proves insufficient, a
> percentage-based ramp-up (e.g., 1% → 10% → 50% → 100%) could be added. For
> the initial implementation, single-request probing provides a good balance of
> safety and simplicity.

### 9.5 Failback Scope

Unlike the SDK (which only scans the PPCB map in its background loop), the driver's
failback loop scans **both** `circuit_breaker_overrides` and `failover_overrides` in a single
`apply_partition` CAS operation. This is simpler and avoids the SDK's design quirk
where PPAF entries can only be removed when all locations are exhausted.

---

## 10. Status Code Handling Matrix

The following table maps each status code to effects emitted by
`evaluate_transport_result()`:

| Status Code | Sub-Status | LocationEffects Emitted | OperationAction |
|---|---|---|---|
| 403 | 3 (WriteForbidden) | `RefreshAccountProperties` + `MarkEndpointUnavailable(WriteForbidden)` + `MarkPartitionUnavailable` | `FailoverRetry` |
| 408 | Any (RequestTimeout) | `MarkPartitionUnavailable` + `MarkEndpointUnavailable(RequestTimeout)` | `FailoverRetry` |
| 410 | Any (Gone) | `MarkPartitionUnavailable` + `MarkEndpointUnavailable(ServiceUnavailable)` | `FailoverRetry` |
| 429 | 3092 (SystemResourceUnavailable) | `MarkPartitionUnavailable` + `MarkEndpointUnavailable(ServiceUnavailable)` | `FailoverRetry` |
| 500 | Any (reads only) | `MarkPartitionUnavailable` + `MarkEndpointUnavailable(InternalServerError)` | `FailoverRetry` |
| 503 | Any | `MarkPartitionUnavailable` + `MarkEndpointUnavailable(ServiceUnavailable)` | `FailoverRetry` |
| 404 | 1002 (ReadSessionNotAvailable) | None | `SessionRetry` |
| Transport error (not sent) | — | None | `FailoverRetry` |
| Transport error (sent, idempotent) | — | `MarkEndpointUnavailable(TransportError)` | `FailoverRetry` |
| Other | — | None | `Abort` |

> **Note**: 408 (RequestTimeout) and 500 (InternalServerError, reads only) also
> trigger partition-level failure tracking (`MarkPartitionUnavailable`), matching
> the Java SDK's behavior where these status codes invoke PPCB handling via
> `handleLocationExceptionForPartitionKeyRange`.

### Effect Processing in `apply()`

For each status code that emits `MarkPartitionUnavailable`:

1. `MarkPartitionUnavailable` → CAS on `PartitionEndpointState`:
   - For PPCB: increment failure counter, check threshold, potentially move
     to next endpoint
   - For PPAF: unconditionally move to next endpoint
   - If `partition_key_range_id` is empty (first attempt), this effect is
     skipped — no partition-level state change occurs

2. `MarkEndpointUnavailable` → CAS on `AccountEndpointState`:
   - Marks the entire endpoint as temporarily unavailable (existing behavior)
   - Future requests to any partition on this endpoint will skip it during
     account-level endpoint selection

Both effects are applied in the same `apply()` call. The partition effect modifies
future routing for this specific partition, while the endpoint effect modifies
routing for all requests to that region.

---

## 11. Configuration Surface

### 11.1 Environment Variables

| Variable | Type | Default | Description |
|---|---|---|---|
| `AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED` | `bool` | `true` | Master switch for per-partition circuit breaker |
| `AZURE_COSMOS_ALLOWED_PARTITION_UNAVAILABILITY_DURATION_IN_SECONDS` | `i64` | `5` | Minimum time a partition must be unavailable before failback sweep considers it |
| `AZURE_COSMOS_PPCB_STALE_PARTITION_UNAVAILABILITY_REFRESH_INTERVAL_IN_SECONDS` | `i64` | `300` | Interval between background failback sweep iterations |
| `AZURE_COSMOS_CIRCUIT_BREAKER_FAILURE_COUNT_FOR_READS` | `i32` | `2` | Read failure threshold before circuit trips |
| `AZURE_COSMOS_CIRCUIT_BREAKER_FAILURE_COUNT_FOR_WRITES` | `i32` | `5` | Write failure threshold before circuit trips |
| `AZURE_COSMOS_CIRCUIT_BREAKER_TIMEOUT_COUNTER_RESET_WINDOW_IN_MINUTES` | `i64` | `5` | Window (in minutes) after which failure counters reset |

### 11.2 Server-Side Configuration

| Property | Source | Description |
|---|---|---|
| `enable_per_partition_failover_behavior` | `AccountProperties` JSON | Enables PPAF for single-master write accounts. Updated dynamically on account refresh. |

---

## 12. Interaction with Account-Level Failover

### 12.1 Layered Failover

Partition-level and account-level failover operate as complementary layers managed
by the same `LocationStateStore`:

```mermaid
flowchart TD
    Start["evaluate_transport_result() emits effects"]
    Part["MarkPartitionUnavailable → CAS on PartitionEndpointState<br/>Route THIS partition to alternate region<br/>Other partitions in the same region are unaffected"]
    Acct["MarkEndpointUnavailable → CAS on AccountEndpointState<br/>Route ALL requests for the account to alternate region<br/>Marks entire endpoint as unavailable for reads/writes"]
    Start --> Part
    Start --> Acct
```

### 12.2 Priority

1. **Partition-level overrides are checked after account-level endpoint selection**
   in `resolve_endpoint()`. If a partition-level override exists, it takes
   precedence over the account-level routing decision for that specific partition.

2. **For 403/3**, both `MarkPartitionUnavailable` and `MarkEndpointUnavailable` +
   `RefreshAccountProperties` are emitted simultaneously. On the next loop
   iteration, the partition override (if successfully applied) takes priority
   in `resolve_endpoint()`.

3. **For 503/429/410**, both `MarkPartitionUnavailable` and
   `MarkEndpointUnavailable` are emitted. The account-level unavailability
   routing skips the failed endpoint for all requests, while the partition-level
   override routes this specific partition to a known-good alternate endpoint.

### 12.3 Endpoint Lists

| Mechanism | Endpoint Source | Description |
|---|---|---|
| PPAF | `AccountEndpointState.preferred_read_endpoints` | Full set of account-level read endpoints. Used because single-master writes can only fail over to read regions. |
| PPCB | `AccountEndpointState.preferred_read_endpoints` | Preferred read endpoints (same list on multi-master, where all regions serve reads and writes). |

**Note**: In the SDK, PPAF uses `account_read_endpoints()` (unordered account-level
regions) while PPCB uses `read_endpoints()` (preferred-location-ordered). In the
driver, both use `preferred_read_endpoints` from `AccountEndpointState`. If
preferred-location ordering is needed for the PPAF case (using account-level order
rather than preferred order), a separate `account_read_endpoints` list may need to be
added to `AccountEndpointState` in the future.

---

## 13. Known Issues & Design Decisions

### 13.1 CAS Contention on PartitionEndpointState

The driver replaces the SDK's `RwLock<HashMap>` with a CAS loop on an immutable
`PartitionEndpointState` snapshot. Under high concurrency with many partitions
failing simultaneously, multiple threads may race to CAS the same snapshot, causing
retry iterations in the CAS loop.

**Mitigation**: Partition failures are infrequent events. Under normal operation, the
CAS loop completes in a single iteration. Under stress (many concurrent partition
failures), the CAS may retry a few times, but each retry is a cheap clone +
functional update — much cheaper than lock contention. If this becomes a measurable
bottleneck, the partition state could be split into per-partition atomic entries, but
this is not expected to be necessary.

### 13.2 Failure Counter Side Effects in CAS Model

In the SDK, `increment_request_failure_counter_and_check_if_partition_can_failover()`
always increments the counter (side effect) even if the threshold is not reached.
In the driver's immutable model, counter increments happen inside the
`mark_partition_unavailable` pure function, which produces a new state with
updated counters via CAS. This means a failed CAS (concurrent modification)
may lose a counter increment.

**Acceptance criteria**: Losing an occasional counter increment under high concurrency
is acceptable — it delays the threshold trigger by one failure at most, which is a
better trade-off than introducing locks.

### 13.3 Threshold Gate on Override Application

When a PPCB entry exists but the failure count has not yet exceeded the threshold,
`resolve_endpoint()` returns the account-level endpoint (no override applied).
This means the request continues to hit the original (possibly unhealthy) region
until enough failures accumulate. This is a deliberate trade-off:

- **Pro**: Prevents premature failovers on transient, self-healing errors.
- **Con**: Requests continue to fail until the threshold is reached, adding latency.

### 13.4 First Attempt Has No Partition Override

The partition key range ID is not known until the first response is received
(from the `x-ms-documentdb-partitionkeyrangeid` header). This means:

- The **first attempt** always uses account-level routing.
- Partition-level overrides take effect starting from the **first retry**.
- If a partition has been previously failed over and a new request arrives, the
  override is only applied if the partition key range ID was already learned
  from a prior operation for the same partition.

**Future improvement**: If the driver implements direct-mode partition key range
resolution (bypassing the gateway for physical partition discovery), the partition
key range ID could be known before the first attempt.

### 13.5 Stale Override After Account Refresh

When account properties are refreshed and the region topology changes (e.g., a new
region is added), existing partition-level override entries are **not** invalidated.
The overrides continue to route to the previously selected alternate region until
either:
- The failback loop transitions them to `ProbeCandidate` and a successful probe
  removes the entry, or
- All locations are exhausted and the entry is removed.

This is generally acceptable because region topology changes are rare, but it means
the override may point to a less-optimal region after a topology change.

### 13.6 Background Task Lifecycle via `BackgroundTaskManager`

The failback loop (and any future background tasks in the driver) is spawned
through the driver's `BackgroundTaskManager`
(`driver::transport::background_task_manager`). This provides:

- **Abort-on-drop**: When the `BackgroundTaskManager` is dropped, all stored
  `JoinHandle`s are aborted, cancelling background tasks immediately.
- **Panic safety**: Spawned futures are wrapped in `catch_unwind`, so a panic
  in the failback loop is logged rather than crashing the runtime.
- **Graceful shutdown**: `BackgroundTaskManager::shutdown()` aborts and
  *awaits* all tasks, providing deterministic cleanup on driver teardown.
- **Handle pruning**: Completed task handles are pruned on each `spawn()`
  call, preventing unbounded accumulation.

The `Weak` reference inside the failback future provides a secondary exit
condition: if the `LocationStateStore` is dropped before the manager aborts
the task, the loop self-terminates on the next iteration.

---

## 14. Test Coverage

The implementation should include comprehensive tests covering:

### 14.1 Pure Routing System Tests

- `mark_partition_unavailable`: PPAF path creates entry and moves to next endpoint
- `mark_partition_unavailable`: PPCB path increments counter, no move below threshold
- `mark_partition_unavailable`: PPCB path moves endpoint when threshold exceeded
- `mark_partition_unavailable`: all endpoints exhausted → entry removed
- `mark_partition_unavailable`: concurrent CAS (different thread already moved)
- `expire_partition_overrides`: entries older than duration transition to `ProbeCandidate`
- `expire_partition_overrides`: entries newer than duration are preserved as `Unhealthy`
- `expire_partition_overrides`: both PPAF and PPCB maps are scanned
- `expire_partition_overrides`: entries already in `ProbeCandidate` state are not re-transitioned

### 14.2 Eligibility Tests

- PPAF eligibility for write operations on single-master accounts
- PPAF ineligibility for read operations
- PPAF ineligibility on multi-master accounts
- PPCB eligibility for reads on any account type
- PPCB eligibility for writes on multi-master accounts
- PPCB ineligibility for writes on single-master accounts
- Ineligibility when both flags are disabled
- Ineligibility for non-partitioned resource types (Databases, Containers, etc.)
- Ineligibility when only one read endpoint is available

### 14.3 Circuit Breaker Counter Tests

- Read failure counter increment and threshold check
- Write failure counter increment and threshold check
- Counter reset after timeout window elapses
- Threshold not exceeded → no failover
- Threshold exceeded → failover triggered

### 14.4 `resolve_endpoint` Integration Tests

- Partition override applied when PPAF entry exists
- Partition override applied when PPCB entry exists and threshold exceeded
- No partition override when PPCB entry exists but threshold not exceeded
- No partition override when `partition_key_range_id` is `None`
- Partition override takes precedence over account-level endpoint

### 14.5 `evaluate_transport_result` Effect Tests

- 403/3 emits `MarkPartitionUnavailable` + `MarkEndpointUnavailable` + `RefreshAccountProperties`
- 408 emits `MarkPartitionUnavailable` + `MarkEndpointUnavailable(RequestTimeout)`
- 500 (reads only) emits `MarkPartitionUnavailable` + `MarkEndpointUnavailable(InternalServerError)`
- 503 emits `MarkPartitionUnavailable` + `MarkEndpointUnavailable`
- 429/3092 emits `MarkPartitionUnavailable` + `MarkEndpointUnavailable`
- `partition_key_range_id` from `OperationRetryState` is wired into effect

### 14.6 `LocationStateStore::apply` Tests

- `MarkPartitionUnavailable` with empty `partition_key_range_id` is skipped
- `MarkPartitionUnavailable` creates new entry in correct map (PPAF vs PPCB)
- `MarkPartitionUnavailable` updates existing entry (increments counter)
- CAS succeeds under no contention
- CAS retries under simulated contention

### 14.7 Failback Loop Tests

- Background loop exits when `LocationStateStore` is dropped (`Weak` upgrade fails)
- Partitions eligible for failback after unavailability duration
- Partitions NOT eligible before unavailability duration
- `Unhealthy` entry transitions to `ProbeCandidate` after unavailability duration

### 14.8 Gradual Failback (Probe) Tests

- `ProbeCandidate` entry causes `resolve_endpoint` to route one request to original region
- Subsequent requests while probe is in-flight continue to use override endpoint
- Successful probe removes entry → future requests use default routing
- Failed probe transitions entry back to `Unhealthy` with reset `first_failure_time`
- Multiple partitions in `ProbeCandidate` state are probed independently

### 14.9 End-to-End Operation Loop Tests

- Multi-region failover with 3 regions → round-robin through regions via partition override
- Partition key range ID captured from first response, used in retry
- 403/3 with PPAF enabled → partition-level retry with override
- 503 → partition marked + endpoint marked + failover retry

---

## 15. Prerequisites & Missing Pieces

### 15.1 Partition Key Range ID Availability

**Status**: Not yet available on `CosmosOperation` or `OperationRetryState`.

The partition key range ID is essential for partition-level failover — it's the key
for the failover override maps. In the SDK, this comes from
`RequestContext.resolved_partition_key_range` which is set during address resolution.

**In the driver** (gateway mode), the partition key range ID must be extracted from
the gateway response header `x-ms-documentdb-partitionkeyrangeid`. This requires:

1. **Add `partition_key_range_id: Option<PartitionKeyRangeId>` to `OperationRetryState`**
   (in `pipeline/components.rs`).
2. **Extract the header from `TransportResult`** after Stage 4 of the operation loop.
   The extraction should happen for both success and failure responses.
3. **Wire the value into `UnavailablePartition`** when building
   `LocationEffect::MarkPartitionUnavailable` in `evaluate_transport_result()`.

**Limitation**: On the first attempt, no partition key range ID is available. This
means partition-level failover cannot take effect until the first retry. This is
acceptable because:
- Account-level failover handles the first retry (via `MarkEndpointUnavailable`)
- Partition-level override enhances routing for subsequent retries of the same
  request and future requests for the same partition

### 15.2 `ResourceType.is_partitioned()` Method

The eligibility check requires knowing whether the operation targets a partitioned
resource. A convenience method on `ResourceType`:

```rust
impl ResourceType {
    pub fn is_partitioned(&self) -> bool {
        matches!(self, ResourceType::Document | ResourceType::StoredProcedure)
    }
}
```

**Status**: May already exist or need to be added.

### 15.3 Environment Variable Reading

The driver needs to read PPAF/PPCB env vars at construction time. Currently, no
env-var-based configuration exists in the driver. The values should be read once
in `CosmosDriver::new()` (or `CosmosDriverRuntime`) and stored in
`PartitionFailoverConfig`.

### 15.4 `sync_account_properties` Integration

`LocationStateStore::sync_account_properties()` currently only updates
`AccountEndpointState`. It needs to also CAS-update `PartitionEndpointState` flags:

```rust
// In sync_account_properties(), after updating account state:
self.apply_partition(|current| {
    let mut next = current.clone();
    next.per_partition_automatic_failover_enabled = properties.enable_per_partition_failover_behavior;
    next.per_partition_circuit_breaker_enabled = properties.enable_per_partition_failover_behavior
        || current.config.circuit_breaker_option_enabled;
    next
});
```

### 15.5 Files to Create/Modify

| File | Action | Purpose |
|------|--------|---------|
| `src/driver/routing/partition_endpoint_state.rs` | **Create** | `PartitionEndpointState`, `PartitionFailoverEntry`, `PartitionFailoverConfig` |
| `src/driver/routing/routing_systems.rs` | **Modify** | Add `mark_partition_unavailable()`, `expire_partition_overrides()` pure functions |
| `src/driver/routing/location_state_store.rs` | **Modify** | Replace empty `PartitionEndpointState`; add `apply_partition()` CAS method; spawn failback loop via `BackgroundTaskManager`; update `sync_account_properties()` |
| `src/driver/routing/location_effects.rs` | **Modify** | Remove `#[allow(dead_code)]` from `MarkPartitionUnavailable` and `UnavailablePartition` |
| `src/driver/routing/mod.rs` | **Modify** | Export new `partition_endpoint_state` module |
| `src/driver/pipeline/components.rs` | **Modify** | Add `partition_key_range_id: Option<PartitionKeyRangeId>` to `OperationRetryState` |
| `src/driver/pipeline/retry_evaluation.rs` | **Modify** | Wire `partition_key_range_id` from retry state; add `MarkPartitionUnavailable` to 403/3 effects |
| `src/driver/pipeline/operation_pipeline.rs` | **Modify** | Capture `partition_key_range_id` from response headers; consult partition overrides in `resolve_endpoint()` |

---

## Appendix: Data Flow Sequence Diagram

```mermaid
sequenceDiagram
    participant CD as CosmosDriver
    participant EOP as execute_operation_pipeline
    participant LSS as LocationStateStore
    participant T as Transport

    CD->>EOP: execute_operation()
    EOP->>LSS: STAGE 1: snapshot()
    LSS-->>EOP: LocationSnapshot {account, partitions}
    Note over EOP: STAGE 2: resolve_endpoint()<br/>account-level select<br/>partition override? (check partitions map)<br/>→ RoutingDecision
    Note over EOP: STAGE 3: build_transport_request
    EOP->>T: STAGE 4: execute (HTTP request)
    T-->>EOP: TransportResult (503) — HTTP response
    Note over EOP: capture pk_range_id from response headers
    Note over EOP: STAGE 5: evaluate_transport_result()<br/>→ FailoverRetry<br/>+ [MarkPartitionUnavailable, MarkEndpointUnavailable]
    EOP->>LSS: STAGE 6: apply(effects)
    Note over LSS: CAS partition state:<br/>insert/update failover entry
    Note over LSS: CAS account state:<br/>mark endpoint unavailable
    LSS-->>EOP: applied
    Note over EOP: STAGE 7: FailoverRetry → loop back to STAGE 1
    EOP->>LSS: STAGE 1: snapshot()
    LSS-->>EOP: updated partitions with override
    Note over EOP: STAGE 2: resolve_endpoint()<br/>partition override found → alternate region endpoint
    EOP->>T: STAGE 3-4: retry to alternate region
    T-->>EOP: TransportResult (200)
    EOP-->>CD: CosmosResponse
```
