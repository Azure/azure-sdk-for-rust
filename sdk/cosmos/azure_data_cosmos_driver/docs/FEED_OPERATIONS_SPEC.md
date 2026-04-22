# Feed Operations Spec for `azure_data_cosmos_driver`

**Status:** Draft / Iterating
**Date:** 2026-04-21
**Authors:** (team)
**Crate:** `azure_data_cosmos_driver`

---

## Table of Contents

1. [Goals & Motivation](#1-goals--motivation)
2. [Architectural Overview](#2-architectural-overview)
3. [CosmosOperation Changes](#3-cosmosoperation-changes)
4. [Operation Plans](#4-operation-plans)
5. [Planner](#5-planner)
6. [Plan Executor](#6-plan-executor)
7. [Continuation Tokens](#7-continuation-tokens)
8. [OpenTelemetry Integration](#8-opentelemetry-integration)
9. [Error Handling & Partition Splits](#9-error-handling--partition-splits)
10. [API Semantics & Invariants](#10-api-semantics--invariants)
11. [Configuration Surface](#11-configuration-surface)
12. [Performance & Non-Regression](#12-performance--non-regression)
13. [Migration Plan](#13-migration-plan)
14. [Testing Strategy](#14-testing-strategy)
15. [Future Work](#15-future-work)

---

## 1. Goals & Motivation

### Problem Statement

The driver currently supports only **point operations** — operations that target a single resource
and produce a single response. Operations like `ReadItem`, `UpsertItem`, and `DeleteContainer` go
through `execute_operation`, which drives the operation pipeline (region failover, session tokens,
transport retry) and returns a single `CosmosResponse`.

**Feed operations** — queries, read-many, read-all-items, and change feed — are fundamentally
different. They produce multiple pages of results, may fan out across partition key ranges, may
require backend-provided query plans, and need pagination state that can be serialized across
request boundaries.

Today, feed operations are handled entirely in the higher-level `azure_data_cosmos` crate, bypassing
the driver's operation pipeline. This means feed operations miss out on the driver's multi-region
failover, partition-level circuit breaker, throughput control, and diagnostics infrastructure.

### Goals

1. **Unified execution model** — Both point and feed operations flow through a common
   Plan → Execute pipeline. Point operations produce a trivial single-step plan. Feed operations
   produce multi-step plans that leverage the existing point-operation pipeline for individual
   HTTP requests.

2. **Resumable pagination** — Feed operations produce a typed continuation token that can be
   serialized to a string and carried across process boundaries (e.g., sent to a browser).
   Resuming with a valid continuation token and an equivalent operation descriptor continues
   where the previous execution left off.

3. **Extensible operation model** — The plan model must support ReadMany (the initial target),
   cross-partition queries, single-partition queries/reads, and change feed, even if some are
   implemented later.

4. **Driver-level concerns** — Feed operations must integrate with multi-region failover,
   partition-level failover (PPAF/PPCB), throughput control, session consistency, and
   diagnostics — all managed by the driver.

5. **Schema-agnostic pages** — The driver returns response pages as raw bytes (`Vec<u8>`).
   The higher-level SDK handles deserialization, consistent with the existing `CosmosResponse`
   model. Future work (sort, aggregate) will require the driver to understand feed envelopes,
   but the initial design reserves space for this without requiring it.

6. **Performance non-regression** — Point operations must not pay measurable overhead for the
   unified plan model. Trivial plans must be allocation-light.

### Non-Goals (This Spec)

- Full cross-partition query execution with ORDER BY merge-sort and aggregation (future work).
- Change feed full design (future work; this spec reserves extension points).
- Client-side query rewriting or optimization.

### Primary Target

**ReadMany** is the first feed operation to implement. It exercises:
- Partition key range resolution (via `PartitionKeyRangeCache`)
- Fan-out across multiple partition key ranges
- Merging results into a single response
- Integration with the operation pipeline for each sub-request

---

## 2. Architectural Overview

```text
┌─────────────────────────────────────────────────────────────────────────────────┐
│                           CosmosDriver                                         │
│                                                                                │
│  execute_operation(op, opts) → CosmosResponse                                  │
│                                                                                │
│  A single entry point for ALL operations (point and feed).                     │
│  Returns a CosmosResponse which optionally includes a continuation             │
│  token. Point reads never have one; feed operations may.                       │
│  The SDK layer decides which operations to expose as pagers.                   │
│                                                                                │
│  Internally:                                                                   │
│    1. Planner creates an OperationPlan                                         │
│    2. PlanExecutor runs one turn of the plan                                   │
│    3. Returns CosmosResponse (with optional continuation token)                │
│                                                                                │
│  ┌──────────────────────────────────────────────────────────────────────────┐  │
│  │                              PLANNER                                     │  │
│  │                                                                          │  │
│  │  Input:  CosmosOperation + OperationOptions                              │  │
│  │  Output: OperationPlan                                                   │  │
│  │                                                                          │  │
│  │  Responsibilities:                                                       │  │
│  │  ┌─ Determine targeting (point EPK, sub-range, full key space)         │  │
│  │  ├─ For ReadMany: group items by PK range, create fan-out steps          │  │
│  │  ├─ For cross-partition query: fetch backend query plan, create steps    │  │
│  │  ├─ For single-partition ops: create single-step plan                    │  │
│  │  └─ For point ops: create trivial single-step plan                       │  │
│  └──────────────────────────────────────────────────────────────────────────┘  │
│                      │                                                         │
│                      ▼                                                         │
│  ┌──────────────────────────────────────────────────────────────────────────┐  │
│  │                          PLAN EXECUTOR                                    │  │
│  │                                                                          │  │
│  │  Input:  OperationPlan                                                   │  │
│  │  Output: CosmosResponse (single turn / single page)                      │  │
│  │                                                                          │  │
│  │  Responsibilities:                                                       │  │
│  │  ┌─ Execute plan steps with configurable concurrency                     │  │
│  │  ├─ Each step calls execute_single_operation() for HTTP                │  │
│  │  ├─ Handle partition splits (re-plan affected ranges)                    │  │
│  │  ├─ Enforce concurrency caps for fan-out                                 │  │
│  │  ├─ Integrate with throughput control                                    │  │
│  │  ├─ Emit OpenTelemetry spans                                             │  │
│  │  └─ Produce continuation token in response (if more pages remain)        │  │
│  └──────────────────────────────────────────────────────────────────────────┘  │
│                      │                                                         │
│                      ▼                                                         │
│  ┌──────────────────────────────────────────────────────────────────────────┐  │
│  │                  OPERATION PIPELINE (existing)                            │  │
│  │                                                                          │  │
│  │  execute_single_operation() — unchanged                                │  │
│  │  Handles: region failover, session tokens, transport retry, auth,        │  │
│  │           429 backoff, diagnostics                                        │  │
│  └──────────────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Layer Separation

The existing `execute_operation_pipeline` function is renamed to **`execute_single_operation`**
in this spec. It remains the internal entry point for executing a single Cosmos DB operation
through the operation pipeline (region failover, session tokens, transport retry, auth, 429
backoff, diagnostics). The feed operations layer calls `execute_single_operation` for each
individual HTTP request within a plan.

| Concern | Component | Location |
|---------|-----------|----------|
| Operation intent & payload | `CosmosOperation` | `models/cosmos_operation.rs` |
| Plan creation | `Planner` | `driver/feed/planner.rs` (new) |
| Plan model | `OperationPlan`, `PlanStep` | `driver/feed/plan.rs` (new) |
| Plan execution | `PlanExecutor` | `driver/feed/executor.rs` (new) |
| Continuation state | `ContinuationToken` | `models/continuation_token.rs` (new) |
| Per-step HTTP execution | `execute_single_operation` | `driver/pipeline/` (existing) |

### Open Issue: Re-Planning on Every Page

Because `execute_operation` is stateless, the driver must re-plan the operation on every
call — including subsequent pages of a paginated feed. The Planner uses the continuation
token to reconstruct the plan state, but still performs the full planning step (PK range
resolution, and for cross-partition queries, potentially a backend query plan fetch) on each
page.

For in-process callers (the common case), this is wasteful: the SDK crate calls
`execute_operation` in a loop, and the plan doesn't change between pages (barring partition
splits). A future optimization could allow `CosmosResponse` and/or `CosmosOperation` to
carry a **cached `OperationPlan`** so that subsequent requests skip re-planning when the
plan is still valid. The cached plan would be invalidated on partition splits (410/1002) or
account metadata changes, falling back to a full re-plan.

This optimization is not required for correctness — the stateless model works correctly
today — but should be considered for performance-sensitive workloads with many small pages.

---

## 3. CosmosOperation Changes

### 3.1 OperationType Refactor

`OperationType` currently carries no data and is `Copy`. Feed operations require variant-specific
data (query text, item lists, etc.). Rather than bloating `OperationType` with payload data — which
would break `Copy` and mix operation semantics with operation payload — we split the concern:

- **`OperationType`** remains a lightweight, `Copy` enum describing operation semantics
  (HTTP method, read-only, idempotent). Unchanged from today.

- **`OperationPayload`** is a new enum carrying variant-specific data. It replaces the untyped
  `body: Option<Vec<u8>>` field on `CosmosOperation`.

```rust
/// Operation-specific payload data.
///
/// Replaces the generic `body: Option<Vec<u8>>` on `CosmosOperation`.
/// Each variant carries exactly the data needed for its operation type.
#[derive(Clone, Debug)]
pub enum OperationPayload {
    /// No payload needed (e.g., ReadItem, DeleteItem, ReadContainer).
    None,

    /// Raw body bytes (e.g., CreateItem, UpsertItem, ReplaceItem).
    /// The caller provides pre-serialized JSON.
    Body(Vec<u8>),

    /// SQL query text with optional parameters (e.g., QueryItems).
    Query {
        /// The SQL query text.
        query: String,
        /// Pre-serialized parameters JSON array, if any.
        parameters: Option<Vec<u8>>,
    },

    /// ReadMany item descriptors: (item_id, partition_key) pairs.
    ReadMany {
        /// The items to read, as (id, partition_key) pairs.
        items: Vec<(String, PartitionKey)>,
    },

    // Future: ChangeFeed { mode, start_from, ... }
}
```

`CosmosOperation` changes from:

```rust
pub struct CosmosOperation {
    operation_type: OperationType,
    resource_type: ResourceType,
    resource_reference: CosmosResourceReference,
    partition_key: Option<PartitionKey>,
    request_headers: CosmosRequestHeaders,
    body: Option<Vec<u8>>,  // ← removed
}
```

to:

```rust
pub struct CosmosOperation {
    operation_type: OperationType,
    resource_type: ResourceType,
    resource_reference: CosmosResourceReference,
    target: OperationTarget,
    request_headers: CosmosRequestHeaders,
    payload: OperationPayload,
}
```

### 3.2 OperationTarget

Partition targeting is currently a single `Option<PartitionKey>` field. Feed operations require
richer targeting. The targeting enum has three modes: no partition scope, a specific logical
partition key (needed for point reads where the raw partition key value must be sent to the
backend), or an EPK range for feed operations spanning one or more partitions.

```rust
/// How the operation is targeted to partitions.
#[derive(Clone, Debug)]
pub enum OperationTarget {
    /// No partition targeting (account-level or database-level operations,
    /// such as CreateDatabase or ReadContainer).
    None,

    /// Target a specific logical partition key.
    ///
    /// Used for point operations (read, create, delete, upsert, replace)
    /// and single-partition feed operations where the raw partition key
    /// value must be included in the request headers.
    PartitionKey(PartitionKey),

    /// Target an effective partition key range.
    ///
    /// Used for feed operations that span one or more partitions.
    /// Uses the existing `EpkRange<EffectivePartitionKey>` type from
    /// `models::range`.
    ///
    /// The pipeline resolves the EPK range to the owning PK range ID(s) via
    /// the `PartitionKeyRangeCache` at execution time.
    EpkRange(EpkRange<EffectivePartitionKey>),
}

impl OperationTarget {
    /// The full key space: targets all partition key ranges.
    pub fn all_ranges() -> Self {
        Self::EpkRange(EpkRange::new(
            EffectivePartitionKey::MIN,
            EffectivePartitionKey::MAX,
            true,
            false,
        ))
    }
}
```

**Future optimization:** `EpkRange` could gain an optional PK range ID hint to skip the
cache lookup when the mapping is already known (e.g., from a previous routing decision or a
cached plan). The hint would be advisory — the pipeline would fall back to EPK-based
resolution if the hint is stale after a partition split.

### 3.3 Factory Method Updates

Existing factory methods are updated to use `OperationPayload` and `OperationTarget`:

```rust
impl CosmosOperation {
    /// Reads an item.
    pub fn read_item(item: ItemReference) -> Self {
        let partition_key = item.partition_key().clone();
        Self::new(OperationType::Read, item)
            .with_target(OperationTarget::PartitionKey(partition_key))
    }

    /// Creates an item. Use `with_body()` to provide the document JSON.
    pub fn create_item(
        container: ContainerReference,
        partition_key: PartitionKey,
    ) -> Self {
        let resource_ref = CosmosResourceReference::from(container)
            .with_resource_type(ResourceType::Document)
            .into_feed_reference();
        Self::new(OperationType::Create, resource_ref)
            .with_target(OperationTarget::PartitionKey(partition_key))
        // Caller attaches body via .with_payload(OperationPayload::Body(...))
    }

    /// Queries items within a single partition.
    pub fn query_items(
        container: ContainerReference,
        partition_key: PartitionKey,
        query: impl Into<String>,
    ) -> Self {
        let resource_ref = CosmosResourceReference::from(container)
            .with_resource_type(ResourceType::Document)
            .into_feed_reference();
        Self::new(OperationType::Query, resource_ref)
            .with_target(OperationTarget::PartitionKey(partition_key))
            .with_payload(OperationPayload::Query {
                query: query.into(),
                parameters: None,
            })
    }

    /// Queries items across all partitions.
    pub fn query_items_cross_partition(
        container: ContainerReference,
        query: impl Into<String>,
    ) -> Self {
        let resource_ref = CosmosResourceReference::from(container)
            .with_resource_type(ResourceType::Document)
            .into_feed_reference();
        Self::new(OperationType::Query, resource_ref)
            .with_target(OperationTarget::all_ranges())
            .with_payload(OperationPayload::Query {
                query: query.into(),
                parameters: None,
            })
    }

    /// Reads multiple items by their ID/partition-key pairs.
    pub fn read_many(
        container: ContainerReference,
        items: Vec<(String, PartitionKey)>,
    ) -> Self {
        let resource_ref = CosmosResourceReference::from(container)
            .with_resource_type(ResourceType::Document)
            .into_feed_reference();
        Self::new(OperationType::Query, resource_ref)
            .with_target(OperationTarget::all_ranges())
            .with_payload(OperationPayload::ReadMany { items })
    }
}
```

### 3.4 Backward Compatibility

The `body: Option<Vec<u8>>` field is removed and replaced with `payload: OperationPayload`.
Factory methods that previously required `.with_body(...)` now accept the body in the factory
method or via `.with_payload(...)`. A convenience method `with_body(Vec<u8>)` can be kept as
sugar for `with_payload(OperationPayload::Body(...))`.

The transport pipeline's request builder must be updated to extract body bytes from
`OperationPayload` when constructing the HTTP request. For `Body` and `Query` variants, this
is straightforward serialization. For `ReadMany`, the Planner decomposes the operation before
it reaches the transport pipeline, so the transport never sees a `ReadMany` payload directly.

---

## 4. Operation Plans

### 4.1 Plan Model

An `OperationPlan` describes the steps needed to execute an operation. It is an enum with
two variants: `Trivial` for single-step plans (stack-allocated, no heap overhead) and
`MultiStep` for fan-out plans.

```rust
/// A plan for executing an operation.
///
/// Plans range from trivial (single step for a point read) to complex
/// (fan-out across partition key ranges with merge). The plan is created
/// by the Planner and executed by the PlanExecutor.
pub(crate) enum OperationPlan {
    /// A single-step plan. Stack-allocated, no heap overhead.
    /// Used for point operations and single-partition feed operations.
    Trivial(PlanStep),

    /// A multi-step plan. The last step in the Vec is the output step.
    /// Used for fan-out operations (ReadMany, cross-partition queries).
    MultiStep {
        steps: Vec<PlanStep>,
    },
}

/// A single step in an operation plan.
pub(crate) enum PlanStep {
    /// Execute a single HTTP request via the operation pipeline.
    ///
    /// The `operation` carries the **unrewritten** query from the backend
    /// query plan, which may contain the `{documentdb-formattableorderbyquery-filter}`
    /// placeholder token. At execution time, the executor replaces this
    /// token with the `resume_filter` (if present) via simple string
    /// substitution before sending the request.
    Fetch {
        /// The operation to execute. Targeted to a specific PK range.
        /// For ORDER BY queries, the query text contains the
        /// `{documentdb-formattableorderbyquery-filter}` placeholder.
        operation: CosmosOperation,
        /// Options for this fetch.
        options: OperationOptions,
        /// Server-provided continuation token for this range, if resuming.
        continuation: Option<String>,
        /// A filter expression to inject into the query by replacing
        /// `{documentdb-formattableorderbyquery-filter}` in the query text.
        /// Set by the Planner when resuming from an `OrderBy` continuation.
        /// For example: `"c.name > 'Baker'"` or `"c.name >= 'Baker'"`.
        /// `None` for first-page execution or non-ORDER BY queries.
        resume_filter: Option<String>,
    },

    /// Merge results from multiple upstream steps with no ordering guarantee.
    ///
    /// Results are concatenated in the order they complete. Used by ReadMany
    /// to combine results from multiple PK ranges. For ReadMany, all upstream
    /// fetches are driven to completion and their results are concatenated.
    ///
    /// Note: Cross-partition queries without an explicit ORDER BY still return
    /// results in (PartitionKey, ID) ascending order within each partition.
    /// The `UnorderedMerge` step concatenates partition results but does NOT
    /// sort across partitions. For globally-ordered results, use `OrderedMerge`
    /// (future).
    UnorderedMerge {
        /// Indices of the steps whose results are merged.
        inputs: Vec<usize>,
    },

    // Future variants:
    // OrderedMerge { inputs: Vec<usize>, order_by: ... },
    // Aggregate { inputs: Vec<usize>, aggregation: ... },
}
```

### 4.2 Plan Examples

#### Point Operation (ReadItem)

```text
Trivial(Fetch(read_item operation)) → output
```

A `Trivial` plan with one `Fetch` step. The executor runs it directly, gets a
`CosmosResponse`, done. No heap allocation.

#### ReadMany

```text
Step 0: Fetch(query to PK range "0", items [(pk_a, a), (pk_b, b)])
Step 1: Fetch(query to PK range "1", items [(pk_c, c)])
Step 2: Fetch(query to PK range "2", items [(pk_d, d), (pk_e, e), (pk_f, f)])
Step 3: UnorderedMerge(inputs: [0, 1, 2]) → output
```

The executor runs steps 0–2 concurrently (subject to concurrency cap), each driving all
server-side pages to completion. Step 3 merges the fully-buffered results.

**Optimization:** When a PK range contains only a single item, the Planner MAY optimize
the `Fetch` step to a point read (`OperationType::Read` with a point EPK range target)
instead of a query, avoiding the overhead of query parsing on the backend.

#### Single-Partition Query

```text
Step 0: Fetch(query to PK "my-pk", continuation: None) → output
```

On subsequent turns, the executor updates the continuation in step 0 and re-executes.
Each turn yields one page.

#### Cross-Partition Query (Future)

```text
Step 0: Fetch(query to PK range "0")
Step 1: Fetch(query to PK range "1")
Step 2: Fetch(query to PK range "2")
Step 3: UnorderedMerge(inputs: [0, 1, 2]) → output
           (or Step 3: OrderedMerge for explicit ORDER BY)
```

Each turn, the executor advances whichever PK range steps have results available.

**Ordering note:** Within each partition, results are always returned in
(PartitionKey, ID) ascending order — even without an explicit `ORDER BY` clause.
The `UnorderedMerge` step concatenates partition results without cross-partition
sorting. For queries with an explicit `ORDER BY`, an `OrderedMerge` step (future)
performs a k-way merge over partition heads to produce globally ordered results.

### 4.3 Incremental Page Production

Plans MUST support incremental page production. The executor does NOT wait for all partition
steps to complete before emitting a page. Instead:

- **Unordered fan-out** (ReadMany, cross-partition query without ORDER BY): Results are
  buffered per partition step. For ReadMany, all partitions are driven to completion and
  merged (single logical page). For queries, pages are emitted as partitions produce them.
  Note that within each partition, results arrive in (PartitionKey, ID) ascending order;
  only the cross-partition merge is unordered.

- **Ordered fan-out** (cross-partition query with explicit ORDER BY, future): A k-way merge
  streams items from partition heads. A page is emitted when enough items are available or
  a partition produces a page boundary.

- **Single-step plans**: Each turn is one HTTP request, one page.

### 4.4 Trivial Plan Optimization

For point operations, the plan model MUST be zero or near-zero overhead compared to the current
direct `execute_single_operation` call. The `OperationPlan::Trivial` variant ensures this:

- **No heap allocation**: The single `PlanStep` is stored inline in the enum, not in a `Vec`.
- **No graph traversal**: The executor matches on `Trivial` and directly calls
  `execute_single_operation`.

---

## 5. Planner

### 5.1 Responsibilities

The Planner transforms a `CosmosOperation` into an `OperationPlan`. It is a synchronous,
deterministic function for most operations, but MAY need to perform async I/O for cross-partition
queries (fetching a backend query plan).

```rust
pub(crate) struct Planner<'a> {
    /// Access to the PK range cache for partition resolution.
    pk_range_cache: &'a PartitionKeyRangeCache,
}

impl<'a> Planner<'a> {
    /// Creates an operation plan from a CosmosOperation.
    ///
    /// For point operations, this is synchronous and trivial.
    /// For feed operations, this may need to resolve PK ranges
    /// and (for cross-partition queries) fetch a backend query plan.
    pub async fn plan(
        &self,
        operation: &CosmosOperation,
        options: &OperationOptions,
        continuation: Option<&ContinuationToken>,
        // Callback for fetching PK ranges (keeps Planner transport-decoupled).
        fetch_pk_ranges: impl Fn(...) -> ...,
        // Callback for fetching query plans (keeps Planner transport-decoupled).
        fetch_query_plan: impl Fn(...) -> ...,
    ) -> azure_core::Result<OperationPlan> {
        // ...
    }
}
```

### 5.2 Planning Logic by Operation Type

| Operation | Targeting | Plan Strategy |
|-----------|-----------|---------------|
| ReadItem, DeleteItem, etc. | `PartitionKey` | Single `Fetch` step. Trivial. |
| CreateDatabase, ReadContainer, etc. | `None` | Single `Fetch` step. Trivial. |
| QueryItems (single partition) | `PartitionKey` | Single `Fetch` step. Paginated. |
| ReadAllItems (single partition) | `PartitionKey` | Single `Fetch` step. Paginated. |
| QueryItems (cross-partition) | `EpkRange` (`all_ranges()`) | Resolve PK ranges → N `Fetch` steps + `UnorderedMerge`. May fetch query plan. |
| ReadMany | `EpkRange` (`all_ranges()`) | Group items by PK range → N `Fetch` steps + `UnorderedMerge`. No pagination. |
| ReadAllItems (cross-partition) | `EpkRange` (`all_ranges()`) | Resolve PK ranges → N `Fetch` steps + `UnorderedMerge`. Paginated. |
| ChangeFeed (future) | varies | TBD |

### 5.3 Operation Decomposition: From One `CosmosOperation` to Many

A key responsibility of the Planner is decomposing a single caller-provided `CosmosOperation`
into multiple targeted `CosmosOperation` instances — one per partition key range — that each
flow through `execute_single_operation` independently. This section illustrates the full
decomposition for two representative operations.

#### Example: Cross-Partition Query

The caller creates a single operation:

```rust
let op = CosmosOperation::query_items_cross_partition(
    container.clone(),
    "SELECT * FROM c WHERE c.status = 'active'",
);
// op.target == OperationTarget::all_ranges() (full EPK range ["", "FF"))
// op.payload == OperationPayload::Query { query: "SELECT ...", parameters: None }
```

The Planner first fetches a **backend query plan** from the service (see [§5.4](#54-query-plan-fetching))
to determine how the query should be distributed across partitions — including whether
client-side sort or aggregation is required. It then resolves the container's partition key
ranges (say, ranges "0", "1", "2") and uses the backend query plan to assemble an
`OperationPlan` with **three separate `CosmosOperation`** instances:

```text
Caller's CosmosOperation
  target: EpkRange ["", "FF")  (all_ranges())
  payload: Query { "SELECT * FROM c WHERE c.status = 'active'" }
                │
                ▼
          ┌─── Planner ──────────────────────────────────┐
          │  1. Fetch backend query plan (via §5.4)      │
          │  2. Resolve PK ranges: 0, 1, 2               │
          │  3. Assemble plan from query plan + PK ranges │
          └───────┬──────────────────────────────────────┘
      ┌───────────┼───────────┐
      ▼           ▼           ▼
 CosmosOperation  CosmosOperation  CosmosOperation
  type: Query      type: Query      type: Query
  target:          target:          target:
   EpkRange        EpkRange         EpkRange
   ["","55")       ["55","AA")      ["AA","FF")
  payload:         payload:         payload:
   Query{rewritten}  Query{rewritten}  Query{rewritten}
      │               │               │
      ▼               ▼               ▼
 execute_operation  execute_operation  execute_operation
 _pipeline()       _pipeline()       _pipeline()
      │               │               │
      ▼               ▼               ▼
 CosmosResponse    CosmosResponse    CosmosResponse
      │               │               │
      └───────────────┼───────────────┘
                      ▼
              UnorderedMerge
                      │
                      ▼
              CosmosResponse
```

Each decomposed `CosmosOperation` is **retargeted** to a specific EPK range. Note that the
query payload may differ from the caller's original SQL: the backend query plan may
**rewrite the query** (e.g., to push down aggregations, add internal projections, or
restructure filters for per-partition execution), and the Planner uses the rewritten query
text in the decomposed operations. The operation pipeline handles region failover, retry,
and auth for each independently.

#### Example: ReadMany

The caller creates one operation with 5 items across 3 PK ranges:

```rust
let op = CosmosOperation::read_many(container.clone(), vec![
    ("id_a".into(), PartitionKey::from("pk_a")),
    ("id_b".into(), PartitionKey::from("pk_b")),
    ("id_c".into(), PartitionKey::from("pk_c")),
    ("id_d".into(), PartitionKey::from("pk_d")),
    ("id_e".into(), PartitionKey::from("pk_e")),
]);
```

The Planner computes EPKs for each partition key, groups by PK range, and produces:

```text
Caller's CosmosOperation
  target: EpkRange ["", "FF")  (all_ranges())
  payload: ReadMany { items: [(id_a,pk_a), (id_b,pk_b), (id_c,pk_c), (id_d,pk_d), (id_e,pk_e)] }
                │
                ▼
          ┌─── Planner ──────────────────────────────────────────┐
          │  EPK(pk_a),EPK(pk_b) → PK range "0"                 │
          │  EPK(pk_c)           → PK range "1"  (single item!) │
          │  EPK(pk_d),EPK(pk_e) → PK range "2"                 │
          └───────┬──────────────────────────────────────────────┘
      ┌───────────┼───────────┐
      ▼           ▼           ▼
 CosmosOperation  CosmosOperation  CosmosOperation
  type: Query      type: Read       type: Query
  target:          target:          target:
   EpkRange        EpkRange         EpkRange
   ["","55")       [EPK(pk_c),      ["AA","FF")
                   EPK(pk_c))
  payload:         payload:         payload:
   Body{query on    None (point     Body{query on
   (pk_a,id_a),     read of id_c)   (pk_d,id_d),
   (pk_b,id_b)}                     (pk_e,id_e)}
```

Note two things:
1. The ReadMany query for each PK range filters on **both partition key and ID**, because
   ID alone is not unique — only (PartitionKey, ID) is unique within a container.
2. PK range "1" contains only a single item, so the Planner **optimizes it to a point read**
   (`OperationType::Read` with a point EPK range), avoiding query overhead.

Each decomposed operation then flows through `execute_single_operation` independently.

### 5.4 Query Plan Fetching

For cross-partition queries, the Planner may need a backend query plan to determine:
- Which partitions to target
- Whether the query requires client-side sort/aggregate
- Optimized partition routing

The Planner uses a **callback** to fetch the query plan, keeping it transport-decoupled. The
callback internally calls `execute_single_operation` (not `execute_operation`), avoiding
re-entry into the Planner. The `OperationType::QueryPlan` variant already exists for this.

```rust
// The Planner calls this callback, which the driver wires to
// execute_single_operation directly (bypassing the Planner).
async fn fetch_query_plan(
    operation: &CosmosOperation,
    options: &OperationOptions,
) -> azure_core::Result<BackendQueryPlan> {
    let query_plan_op = CosmosOperation::query_plan(
        operation.container().unwrap().clone(),
        /* query text from operation payload */
    );
    let response = execute_single_operation(query_plan_op, options, ...).await?;
    BackendQueryPlan::from_response(response)
}
```

This avoids the recursion concern: `fetch_query_plan` calls `execute_single_operation`
directly, which is the internal pipeline function, not the public `execute_operation` that
goes through the Planner.

### 5.5 Resuming from a Continuation Token

When a `ContinuationToken` is provided, the Planner uses it to reconstruct the plan state:

1. Validate the token version, container RID, and operation kind compatibility.
2. Resolve the current partition key ranges for the container.
3. Use `PartitionMapper` to classify each range relative to the token's target range:
   - **Left of target** — ranges whose EPK max ≤ target's EPK min. These are fully drained
     and receive no `Fetch` step (for unordered) or receive a filter-only `Fetch` step
     (for ORDER BY, filtering past the last returned ORDER BY values).
   - **Target range** — the range overlapping the token's EPK bounds. Resumes using the
     stored `server_continuation`. If the range has split, the Planner maps EPK bounds to
     the child range(s) and assigns the server continuation appropriately.
   - **Right of target** — ranges whose EPK min ≥ target's EPK max. Start fresh with no
     continuation (for unordered) or with a filter from the ORDER BY resume state.
4. For `OrderedQuery` tokens, extract the `OrderByResumeState` and generate per-partition
   query filters based on the last returned ORDER BY values. Attach duplicate-elimination
   state (last `_rid`) for the target range.

---

## 6. Plan Executor

### 6.1 Core Execution Loop

The Plan Executor runs an `OperationPlan` and produces pages of results.

```rust
pub(crate) struct PlanExecutor {
    plan: OperationPlan,
    /// Per-step state (continuation, completion status).
    step_states: Vec<StepState>,
    /// Concurrency control for fan-out.
    concurrency_limit: usize,
    /// OpenTelemetry context for span linking.
    trace_context: FeedTraceContext,
}

impl PlanExecutor {
    /// Executes one turn of the plan, producing a `CosmosResponse`.
    ///
    /// The response includes a continuation token if more pages are available.
    /// For non-paginating plans (ReadMany), this drives all steps to completion
    /// and returns the merged result with no continuation token.
    pub async fn execute(
        &mut self,
        driver_context: &DriverContext,
    ) -> azure_core::Result<CosmosResponse> {
        // ...
    }
}
```

### 6.2 Turn Execution

Each call to `execute`:

1. **Emit OpenTelemetry span** for this turn (child of the feed operation span, linked to root).
2. **Identify runnable steps** — steps whose dependencies are satisfied.
3. **Execute runnable steps concurrently** (up to concurrency cap), each via
   `execute_single_operation`.
4. **Collect results** from completed steps.
5. **Advance continuation state** for steps that returned server continuations.
6. **Execute dependent steps** (e.g., `UnorderedMerge`) when their inputs are ready.
7. **Produce the page** from the output step's result.
8. **Update step states** for the next turn.

### 6.3 Concurrency Control

Fan-out steps are executed with a configurable concurrency cap:

```rust
/// Maximum number of concurrent partition key range fetches.
///
/// Defaults to `min(num_pk_ranges, 10)`. Configurable via
/// `OperationOptions::max_concurrency`.
concurrency_limit: usize,
```

The executor uses a semaphore or similar mechanism to limit concurrent
`execute_single_operation` calls. Each concurrent call independently goes through the
full operation pipeline (region failover, retry, etc.).

### 6.4 ReadMany Execution Details

ReadMany is the initial target. Its execution:

1. **Planner** groups `(id, partition_key)` pairs by PK range (via `PartitionKeyRangeCache`).
2. **Plan** has N `Fetch` steps (one per PK range) + one `UnorderedMerge` step.
3. **Executor** runs all `Fetch` steps concurrently (up to concurrency limit).
4. Each `Fetch` step sends a query to its PK range. The query body encodes **both the item IDs
   and the partition keys** for that range, because ID alone is not unique — only the
   (PartitionKey, ID) pair is unique within a container. If the response includes a server
   continuation, the executor continues fetching that range until all items are retrieved.
5. **UnorderedMerge** step concatenates results from all ranges.
6. Returns a single `CosmosResponse` containing all items (with no continuation token).
7. Subsequent calls with the same operation (no continuation) would re-execute from scratch.

**Optimization:** When a PK range contains only a single item, the Planner optimizes the
`Fetch` step to a point read instead of a query (see §4.2).

**Semantics:**
- **Missing items**: Items not found are silently omitted from the result. The response does
  not indicate which items were not found.
- **Order**: Output order is NOT guaranteed to match input order. Items are grouped by
  partition key range.
- **Partial failure**: If any PK range fetch fails after exhausting retries, the entire
  ReadMany operation fails. Partial results are not returned.

### 6.5 Backpressure & Cancellation

- **Caller drops the future**: In-flight `execute_single_operation` futures are
  cancelled via standard Rust drop semantics. The executor does not buffer results beyond
  what is needed for the current turn.
- **Memory bounds**: The executor does not buffer more than `concurrency_limit` concurrent
  page results. For ReadMany (which buffers all results), the total buffered data is bounded
  by the total size of all items — the caller controls this by the size of the input list.
- **Cancellation mid-turn**: If the caller cancels (drops the future) during a turn, any
  in-flight HTTP requests are dropped. The continuation token from the *previous* completed
  turn remains valid for resumption.

---

## 7. Continuation Tokens

### 7.1 Design Principle: O(1) Token Size

A container may have many physical partitions. Storing per-range continuation state
for every partition would make the token size linear in partition count — unacceptable for
tokens that must cross HTTP request boundaries (e.g., sent to a browser in a URL or header).

Instead, the continuation token stores the state of **exactly one partition key range** — the
range where execution last yielded results. On resume, the Planner reconstructs the positions
of all other partitions using **query filter rewriting** rather than stored server tokens.

This follows the same pattern as the Java Cosmos SDK, which exploits the fact that Cosmos DB
data has a composite sort order `(query_sort_order, partition_key_range_id)` to generate
efficient range filters for partitions that don't have stored continuation tokens.

### 7.2 Token Structure

```rust
/// A typed continuation token for resuming a feed operation.
///
/// Opaque to callers. Serializes to a string via `Display` and
/// deserializes via `FromStr`. The internal representation is
/// versioned and validated on deserialization.
///
/// The token mirrors the plan's step graph as a **nested** structure:
/// each pipeline stage wraps the continuation state of its children.
/// This means each layer can interpret its children's state in context
/// — for example, an `OrderBy` node knows how to generate filters for
/// the `Fetch` nodes it wraps, without the Fetch nodes needing to be
/// aware of ORDER BY semantics.
///
/// On resume, the Planner walks the nested token top-down, matching
/// each layer to the corresponding step in the re-created plan.
#[derive(Clone, Debug)]
pub struct ContinuationToken {
    inner: ContinuationTokenInner,
}

/// Internal token representation (not public).
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ContinuationTokenInner {
    /// Token format version for forward/backward compatibility.
    version: u32,

    /// Container identity (RID, not name) to detect container recreation.
    #[serde(rename = "containerRid")]
    container_rid: String,

    /// The nested resume state, rooted at the plan's output step.
    /// Each layer wraps the state of its child steps.
    resume: ResumeState,
}

/// Nested resume state for a plan step.
///
/// Each variant captures the state for one pipeline stage and embeds
/// its children's state. This forms a tree that mirrors the plan DAG.
/// New variants can be added as new pipeline stages are introduced.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum ResumeState {
    /// A single partition fetch, mid-stream or just completed.
    /// This is a leaf node — it has no children.
    #[serde(rename = "fetch")]
    Fetch {
        /// EPK min inclusive of the target range.
        min: String,

        /// EPK max exclusive of the target range.
        max: String,

        /// Server-provided continuation token for this range.
        /// Absent when this range was just completed and the cursor
        /// is at the boundary to the next range.
        #[serde(rename = "serverToken", skip_serializing_if = "Option::is_none")]
        server_continuation: Option<String>,
    },

    /// An unordered (sequential-drain) merge over partitions.
    /// Wraps the child `Fetch` that was active when the token was created.
    /// On resume, partitions left of the child are skipped, the child
    /// resumes from its state, and partitions to the right start fresh.
    #[serde(rename = "drain")]
    Drain {
        /// The resume state of the active child Fetch step.
        inner: Box<ResumeState>,
    },

    /// An ordered (k-way merge) over partitions.
    /// Wraps the child `Fetch` that last produced results, plus the
    /// ORDER BY values needed to generate filters for all other partitions.
    #[serde(rename = "orderBy")]
    OrderBy {
        /// The ORDER BY values of the last document returned.
        /// Used to generate range filters for non-target partitions.
        #[serde(rename = "lastValues")]
        last_order_by_values: Vec<serde_json::Value>,

        /// The `_rid` of the last document returned.
        /// Used for duplicate elimination on the target partition.
        #[serde(rename = "lastRid")]
        last_rid: String,

        /// Whether to include documents matching the last ORDER BY values.
        inclusive: bool,

        /// The resume state of the target child Fetch step.
        inner: Box<ResumeState>,
    },

    // Future variants:
    //
    // /// An offset/limit stage wrapping an inner pipeline.
    // #[serde(rename = "offsetLimit")]
    // OffsetLimit {
    //     skipped: u64,
    //     returned: u64,
    //     inner: Box<ResumeState>,
    // },
}
```

The nesting means each layer owns the interpretation of its children. An `OrderBy` node
knows the `Fetch` inside it is the target partition, and uses `lastValues`/`lastRid` to
generate filters for the other partitions. A `Drain` node knows the `Fetch` inside it is
the cursor position, and partitions left/right of it are skipped/fresh. Neither the `Fetch`
node nor the Planner need to cross-reference sibling state.

#### Wire-format field reference

| Rust type | Field | Wire key | Content |
|-----------|-------|----------|---------|
| `ContinuationTokenInner` | `version` | `version` | Format version (integer) |
| | `container_rid` | `containerRid` | Container RID (string) |
| | `resume` | `resume` | Nested `ResumeState` (root of tree) |
| `ResumeState::Fetch` | *(tag)* | `type` | `"fetch"` |
| | `min` | `min` | EPK min inclusive (hex string) |
| | `max` | `max` | EPK max exclusive (hex string) |
| | `server_continuation` | `serverToken` | Server continuation (omitted if null) |
| `ResumeState::Drain` | *(tag)* | `type` | `"drain"` |
| | `inner` | `inner` | Child `ResumeState` |
| `ResumeState::OrderBy` | *(tag)* | `type` | `"orderBy"` |
| | `last_order_by_values` | `lastValues` | Last ORDER BY values (array) |
| | `last_rid` | `lastRid` | Last document `_rid` (string) |
| | `inclusive` | `inclusive` | Include matching values (bool) |
| | `inner` | `inner` | Child `ResumeState` |

### 7.3 Resume Strategy

On resume, the Planner walks the nested `ResumeState` tree top-down, matching each layer to
the corresponding step in the re-created plan. Each layer interprets its own state and its
child's state in context:

#### `Drain` (unordered cross-partition)

The `Drain` node wraps a `Fetch` child representing the cursor position. On resume:

| Partition position | Action |
|--------------------|--------|
| **Left of child** (EPK max ≤ child's min) | Skip — already drained. |
| **Child range** (matches child's EPK bounds) | Resume using child's `serverToken`. |
| **Right of child** (EPK min ≥ child's max) | Start fresh (not yet visited). |

If the child's range has split, `PartitionMapper` uses the EPK bounds to assign the server
continuation to the appropriate child range(s).

#### `OrderBy` (ordered cross-partition)

The `OrderBy` node wraps a `Fetch` child (the target partition) and carries `lastValues` /
`lastRid` for filter generation. On resume:

| Partition position | Generated filter | Rationale |
|--------------------|-----------------|-----------|
| **Left of child** | ORDER BY values **strictly past** `lastValues` | May have remaining items, but only those after the resume point. |
| **Child range** | Server continuation + ORDER BY values **at or past** `lastValues` | Resume exactly where we stopped. |
| **Right of child** | ORDER BY values **at or past** `lastValues` | Haven't fully explored these yet. |

Duplicate elimination: on the child partition, documents with the same ORDER BY values as
`lastValues` but `_rid ≤ lastRid` have already been returned and are filtered out.

#### `Fetch` (leaf — single partition)

A bare `Fetch` at the root (no wrapping `Drain` or `OrderBy`) represents a single-partition
operation. Resume uses `serverToken` directly.

#### Nesting composes naturally

Future pipeline stages wrap their children the same way:

```text
OffsetLimit { skipped: 50, returned: 20,
  inner: OrderBy { lastValues: ["Baker"], lastRid: "abc", inclusive: true,
    inner: Fetch { min: "55", max: "AA", serverToken: "..." }
  }
}
```

Each layer reads only its own fields plus `inner`. No layer needs to inspect sibling or
grandchild state.

#### Mapping `ResumeState` back to `PlanStep`

The `ResumeState` tree does not map 1:1 to `PlanStep` variants — it maps to the **Planner's
reconstruction logic**:

| `ResumeState` | Effect on plan |
|---------------|----------------|
| `Fetch` | Sets `PlanStep::Fetch.continuation` to the stored `serverToken`. The EPK bounds identify which `Fetch` step in the plan to target. |
| `Drain` | The Planner uses the child `Fetch`'s EPK bounds to determine which partition was active, skips partitions left of it, and starts right partitions fresh. The `UnorderedMerge` step itself is stateless. |
| `OrderBy` | The Planner generates a `resume_filter` string from `lastValues` and sets it on each `Fetch` step. The child `Fetch`'s `continuation` is also restored. Duplicate elimination state (`lastRid`, `inclusive`) is applied at the executor level. |

**Filter injection for ORDER BY queries:** The backend query plan provides a rewritten query
containing the `{documentdb-formattableorderbyquery-filter}` placeholder token. The `Fetch`
step's `operation` holds this **unrewritten** query text. At execution time, the executor
replaces the placeholder with the `resume_filter` via simple string substitution. This means:

- On **first page** (no continuation): the placeholder is replaced with `"true"` (no filter).
- On **resume**: the Planner computes the filter expression from the `OrderBy` resume state
  (e.g., `"c.name > 'Baker'"` for left-of-target partitions, `"c.name >= 'Baker'"` for the
  target and right-of-target) and sets it as `resume_filter` on each `Fetch` step.

This approach keeps the `Fetch` step generic — it doesn't need to understand ORDER BY
semantics, just string substitution on a known placeholder.

### 7.4 Serialization

`ContinuationToken` implements `Display` and `FromStr`. The wire format is base64url-encoded
JSON (using the URL-safe alphabet with no padding):

```rust
impl Display for ContinuationToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = serde_json::to_vec(&self.inner).map_err(|_| fmt::Error)?;
        let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&json);
        f.write_str(&encoded)
    }
}

impl FromStr for ContinuationToken {
    type Err = azure_core::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(s)
            .map_err(|e| azure_core::Error::new(ErrorKind::DataConversion, e))?;
        let inner: ContinuationTokenInner = serde_json::from_slice(&decoded)
            .map_err(|e| azure_core::Error::new(ErrorKind::DataConversion, e))?;
        // Version check
        if inner.version > CURRENT_TOKEN_VERSION {
            return Err(azure_core::Error::with_message(
                ErrorKind::DataConversion,
                "continuation token version is newer than this SDK supports",
            ));
        }
        Ok(Self { inner })
    }
}
```

#### Sample Tokens

**Unordered cross-partition query, mid-stream on partition ["55","AA")**

A `Drain` wraps the active `Fetch`:

JSON (before base64 encoding):
```json
{
  "version": 2,
  "containerRid": "dbs/abc/colls/def",
  "resume": {
    "type": "drain",
    "inner": {
      "type": "fetch",
      "min": "55",
      "max": "AA",
      "serverToken": "+RID:~abc123#RT:1#TRC:10#ISV:2#IEO:65551"
    }
  }
}
```

On resume, the `Drain` sees its child targets `["55","AA")`. Partitions left of `"55"` are
skipped, the target resumes from `serverToken`, and partitions right of `"AA"` start fresh.

**Unordered query, target partition just completed (cursor at boundary)**

```json
{
  "version": 2,
  "containerRid": "dbs/abc/colls/def",
  "resume": {
    "type": "drain",
    "inner": {
      "type": "fetch",
      "min": "55",
      "max": "AA"
    }
  }
}
```

`serverToken` is absent, meaning partition `["55","AA")` is fully drained. The Planner
skips everything up to and including this range, and starts the next partition fresh.

**Single-partition query, mid-stream**

A bare `Fetch` at the root (no wrapping layer):

```json
{
  "version": 2,
  "containerRid": "dbs/abc/colls/def",
  "resume": {
    "type": "fetch",
    "min": "55",
    "max": "AA",
    "serverToken": "-RID:QmFzZTY0#RT:3#TRC:50"
  }
}
```

**ORDER BY cross-partition query, `ORDER BY c.name ASC`**

An `OrderBy` wraps the target `Fetch`, carrying the last returned document's sort values:

```json
{
  "version": 2,
  "containerRid": "dbs/abc/colls/def",
  "resume": {
    "type": "orderBy",
    "lastValues": ["Baker"],
    "lastRid": "R3JlYXQ",
    "inclusive": true,
    "inner": {
      "type": "fetch",
      "min": "55",
      "max": "AA",
      "serverToken": "+RID:~abc456#RT:2#TRC:5#ISV:2#IEO:65551"
    }
  }
}
```

On resume, the `OrderBy` layer generates partition filters from `lastValues`:
- Partitions left of `"55"`: filter `c.name > 'Baker'` (strictly past).
- Target `["55","AA")`: resume from `serverToken`, filter `c.name >= 'Baker'`,
  deduplicate items with `_rid ≤ "R3JlYXQ"`.
- Partitions right of `"AA"`: filter `c.name >= 'Baker'`.

**Compound ORDER BY, `ORDER BY c.name ASC, c.age DESC`**

```json
{
  "version": 2,
  "containerRid": "dbs/abc/colls/def",
  "resume": {
    "type": "orderBy",
    "lastValues": ["Baker", 42],
    "lastRid": "UmVzdW1l",
    "inclusive": true,
    "inner": {
      "type": "fetch",
      "min": "AA",
      "max": "FF",
      "serverToken": "+RID:~abc789#RT:1#TRC:3#ISV:2"
    }
  }
}
```

The `lastValues` array contains one entry per ORDER BY column, in declaration order.

### 7.5 Compatibility Contract

A continuation token is **invalidated** by:

1. **Container recreation** — The token's `containerRid` won't match the new container's RID.
2. **Token version mismatch** — A token produced by a newer SDK version may not be readable
   by an older version.
3. **Structure mismatch** — If the re-created plan produces a different step graph shape
   than the token's nested `ResumeState` (e.g., the operation changed, or the plan type
   differs), the token is rejected.

A continuation token **survives**:

1. **Partition splits** — The token stores EPK bounds, not just PK range IDs. On resume, the
   Planner re-resolves EPK bounds to current PK range IDs.
2. **SDK version upgrades** — The token is versioned. Older token versions are supported by
   newer SDKs (backward compatible deserialization).
3. **Process boundaries** — The token is a self-contained string, safe to send to a browser
   and back.

### 7.6 What the Token Does NOT Encode

- **Per-range state for all partitions** — Only the active Fetch step's state is stored.
  Other partitions' positions are reconstructed via query filter rewriting on resume.
- **Query text or parameters** — The caller must provide an equivalent `CosmosOperation`.
- **Session tokens** — Session consistency is not preserved across process boundaries via
  the continuation token. The driver resolves session tokens from the `SessionManager` cache
  for each turn independently.
- **Container name or database name** — Only the RID is stored. The caller provides routing
  context via the `CosmosOperation`.
- **PK range IDs** — Only EPK bounds are stored, which are stable across partition splits.
  PK range IDs are resolved dynamically from the `PartitionKeyRangeCache` on resume.

---

## 8. OpenTelemetry Integration

### 8.1 Span Hierarchy

Feed operations produce the following span structure:

```text
Feed Operation Span (root)
  ├── db.cosmosdb.operation = "query_items" (or "read_many", etc.)
  ├── db.cosmosdb.container = "my-container"
  ├── db.cosmosdb.feed_operation_id = <uuid>
  │
  ├── Turn 0 Span
  │   ├── db.cosmosdb.feed_turn_index = 0
  │   ├── [linked to Feed Operation Span]
  │   │
  │   ├── PK Range "0" Fetch Span
  │   │   └── (transport pipeline spans)
  │   ├── PK Range "1" Fetch Span
  │   │   └── (transport pipeline spans)
  │   └── UnorderedMerge Span
  │
  ├── Turn 1 Span  (if paginated)
  │   ├── db.cosmosdb.feed_turn_index = 1
  │   └── ...
  ...
```

### 8.2 Cross-Process Span Linking

When a feed operation is resumed from a continuation token in a different process:

1. The original Feed Operation Span is NOT re-opened (it may have ended).
2. A new Feed Operation Span is created in the new process.
3. The continuation token carries the `feed_operation_id` (a UUID).
4. Each Turn Span in the new process includes a **span link** to the original
   feed operation ID, enabling distributed tracing tools to connect the turns
   across process boundaries.

### 8.3 Point Operation Spans

Point operations continue to produce a single span as they do today. The plan/executor layer
does not add additional span nesting for trivial single-step plans.

---

## 9. Error Handling & Partition Splits

### 9.1 Partition Split During Execution

When a `Fetch` step receives a 410/1002 (Gone — PartitionKeyRangeGone) response:

1. **Invalidate** the `PartitionKeyRangeCache` for the affected container.
2. **Re-fetch** the partition key ranges.
3. **Re-plan** the affected step: the original PK range has split into two or more new
   ranges. The executor replaces the single `Fetch` step with new `Fetch` steps for each
   new range.
4. **Update the `UnorderedMerge` step** (if any) to include the new steps.
5. **Resume execution** with the new steps.

The continuation token must survive this: since tokens store EPK bounds (not just PK range
IDs), the re-plan can correctly map EPK bounds to the new PK range IDs.

### 9.2 Error Propagation

| Error Scenario | Behavior |
|----------------|----------|
| 410/1002 (PartitionKeyRangeGone) | Re-plan affected range(s), retry. |
| 429 (Throttled) | Handled by transport pipeline (backoff + retry). |
| 503 (Service Unavailable) | Handled by operation pipeline (region failover). |
| 404 (Not Found) — container | Fail the entire feed operation. |
| 404 (Not Found) — item in ReadMany | Item omitted from results (not an error). |
| Transient network error | Handled by transport pipeline (retry). |
| Invalid continuation token | Fail with `ErrorKind::DataConversion`. |

### 9.3 Partial Failure in Fan-Out

For ReadMany and cross-partition queries, if one PK range fails after exhausting all retries
(transport + operation pipeline), the entire feed operation fails. Partial results from
successful ranges are NOT returned.

**Rationale:** Returning partial results would require the caller to distinguish between
"all items fetched" and "some items fetched, some failed" — a complex API that most callers
don't want. If partial results are needed in the future, they can be exposed via a separate
API or option.

---

## 10. API Semantics & Invariants

### 10.1 Public API

The driver exposes a single `execute_operation` method for **all** operations — both point
and feed. The driver is stateless across calls: each invocation runs one turn of the plan
and returns a `CosmosResponse`. The response optionally includes a continuation token when
more pages are available. The higher-level SDK (e.g., `azure_data_cosmos`) decides which
operations to surface as pagers from a UX perspective.

```rust
impl CosmosDriver {
    /// Executes a Cosmos DB operation (point or feed).
    ///
    /// For point operations (read, create, delete, etc.), this returns the
    /// single response with no continuation token.
    ///
    /// For feed operations (query, read-many, read-all), this executes one
    /// turn of the plan and returns a page of results. If more pages are
    /// available, the response includes a `ContinuationToken`. The caller
    /// passes this token back in `OperationOptions` to fetch the next page.
    ///
    /// The driver does not manage pagination state — it acts as a stateless
    /// service. The SDK layer is responsible for threading continuation tokens
    /// across calls to implement pagers/streams.
    pub async fn execute_operation(
        &self,
        operation: CosmosOperation,
        options: OperationOptions,
    ) -> azure_core::Result<CosmosResponse> {
        // Plan → Execute one turn → return CosmosResponse
    }
}
```

### 10.2 CosmosResponse Changes

`CosmosResponse` gains an optional continuation token:

```rust
#[non_exhaustive]
pub struct CosmosResponse {
    /// Raw response body (UTF-8 JSON or Cosmos binary encoding).
    body: Vec<u8>,

    /// Extracted Cosmos-specific headers.
    headers: CosmosResponseHeaders,

    /// Operation status including HTTP status code and optional sub-status.
    status: CosmosStatus,

    /// Full diagnostics context for this operation.
    diagnostics: Arc<DiagnosticsContext>,

    /// Continuation token for feed operations.
    /// Present when more pages are available; absent for point operations
    /// and when the feed is fully drained.
    continuation_token: Option<ContinuationToken>,
}

impl CosmosResponse {
    /// Returns the continuation token, if more pages are available.
    ///
    /// For point operations, this always returns `None`.
    /// For feed operations, `None` means the operation is complete.
    pub fn continuation_token(&self) -> Option<&ContinuationToken> {
        self.continuation_token.as_ref()
    }
}
```

### 10.3 OperationOptions Changes

`OperationOptions` gains feed-specific fields for continuation and concurrency:

```rust
pub struct OperationOptions {
    // ... existing fields (retry, timeout, consistency, etc.) ...

    /// Maximum number of items per page (feed operations only).
    /// If not set, the server default applies.
    max_item_count: Option<u32>,

    /// Maximum number of concurrent partition key range fetches
    /// (feed operations only). Default: min(num_pk_ranges, 10).
    max_concurrency: Option<usize>,

    /// Continuation token for resuming a previous feed operation.
    /// Pass the token from a previous `CosmosResponse::continuation_token()`.
    continuation: Option<ContinuationToken>,
}
```

These fields are ignored for point operations.

### 10.4 Ordering Guarantees

| Operation | Order Guarantee |
|-----------|-----------------|
| ReadMany | Unordered across partitions. Within each partition, (PartitionKey, ID) ascending. |
| Single-partition query | Server-determined order: (PartitionKey, ID) ascending, or as specified by ORDER BY. |
| Cross-partition query (no ORDER BY) | Within each partition, (PartitionKey, ID) ascending. Across partitions, unordered (partition results are concatenated by `UnorderedMerge`). |
| Cross-partition query (ORDER BY) | Globally ordered per ORDER BY clause (future work: `OrderedMerge` k-way merge). |
| ReadFeed (single partition) | (PartitionKey, ID) ascending. |
| ReadFeed (cross-partition) | Within each partition, (PartitionKey, ID) ascending. Across partitions, unordered. |

### 10.5 Page Boundaries

Page boundaries are determined by:
- **Server-side max item count**: The server may return fewer items than requested.
- **Client-side max item count**: Configurable via `OperationOptions::max_item_count`.
- **Server continuation**: A page boundary occurs whenever the server returns a continuation
  token.

For ReadMany, there is exactly one logical page (the merged result), regardless of how many
server-side pages were consumed internally.

---

## 11. Configuration Surface

### 11.1 OperationOptions Additions

Feed-specific options are added to `OperationOptions` (see §10.3). They are ignored for
point operations. The existing layered resolution applies:

1. `OperationOptions` (per-call)
2. `DriverOptions` (per-driver)
3. `CosmosDriverRuntime` (global)
4. Environment variables

The `max_concurrency`, `max_item_count`, and `continuation` fields follow the same precedence.

---

## 12. Performance & Non-Regression

### 12.1 Point Operation Overhead

The plan model MUST NOT regress point operation performance. Requirements:

- **No heap allocation** for trivial plans beyond what `execute_operation` does today.
- **No additional async machinery** (no spawning, no channels) for single-step plans.
- **Benchmark**: Point operation latency with the plan model must be within 1% of the
  current direct `execute_single_operation` call.

Implementation: For point operations and single-partition feeds, the Planner produces an
`OperationPlan::Trivial` — a stack-allocated single step with no `Vec` overhead. The
executor matches on `Trivial` and calls `execute_single_operation` directly with no
graph traversal. The plan model is only heap-allocated for multi-step fan-out operations.

### 12.2 Fan-Out Memory Bounds

For ReadMany:
- Buffered data is bounded by the total size of all items in the response.
- The executor does not buffer more than `max_concurrency` in-flight requests.

For paginated queries:
- Each turn buffers at most one page per in-flight partition fetch.
- Total buffer: `max_concurrency × max_page_size`.

---

## 13. Migration Plan

### Phase 1: OperationType / OperationPayload Refactor

1. Add `OperationPayload` enum.
2. Add `OperationTarget` enum.
3. Update `CosmosOperation` to use `OperationPayload` and `OperationTarget`.
4. Update factory methods.
5. Update transport pipeline request builder to extract body from `OperationPayload`.
6. Remove `body: Option<Vec<u8>>` from `CosmosOperation`.
7. Update all callers (driver internals, tests, `azure_data_cosmos` bridge).

**This is a breaking internal change.** The `body` field and `partition_key` field on
`CosmosOperation` are replaced. All internal callers must be updated.

### Phase 2: Plan Infrastructure

1. Implement `OperationPlan`, `PlanStep`.
2. Implement `Planner` with trivial single-step planning (point ops only).
3. Implement `PlanExecutor` for single-step plans.
4. Wire `execute_operation` through Plan → Execute path (with fast-path bypass).
5. Validate no performance regression via benchmarks.

### Phase 3: ReadMany

1. Implement ReadMany planning in `Planner`:
   - Group items by PK range (via `PartitionKeyRangeCache`).
   - Create fan-out `Fetch` steps + `UnorderedMerge` step.
2. Implement `UnorderedMerge` step execution in `PlanExecutor`.
3. Wire `execute_operation` to use Plan → Execute for feed operations.
4. Extend `CosmosResponse` with optional `continuation_token` field.
5. Integration tests with partition splits.

### Phase 4: Single-Partition Queries

1. Implement single-partition query planning.
2. Implement paginated execution (continuation threading).
3. Implement `ContinuationToken` serialization.

### Phase 5: Cross-Partition Queries

1. Implement query plan fetching in `Planner`.
2. Implement multi-range query planning.
3. Implement incremental page production for unordered queries.

### Phase 6: Advanced Query Features (Future)

1. ORDER BY merge-sort.
2. Aggregation.
3. Change feed.

---

## 14. Testing Strategy

### 14.1 Unit Tests

| Test Area | Cases |
|-----------|-------|
| Planner — point ops | Verify trivial single-step plan for each point operation type. |
| Planner — ReadMany | Verify correct grouping by PK range. Items spread across ranges. |
| Planner — single-partition query | Verify single `Fetch` step with correct targeting. |
| PlanExecutor — single step | Execute trivial plan, verify result matches direct pipeline call. |
| PlanExecutor — fan-out | Execute multi-step plan with mock pipeline, verify merge. |
| PlanExecutor — concurrency | Verify concurrency cap is respected (at most N concurrent fetches). |
| ContinuationToken — round-trip | Serialize to string, deserialize back, verify equality. |
| ContinuationToken — version compat | Older version tokens deserialize correctly. |
| ContinuationToken — split recovery | Token with EPK bounds spanning a split range maps to correct child ranges. |
| ContinuationToken — O(1) size | Token size is constant regardless of partition count (only one Fetch leaf stored). |
| ContinuationToken — Drain resume | Drain node correctly classifies partitions as left/target/right from nested Fetch. |
| ContinuationToken — OrderBy resume | OrderBy node generates correct range filters and dedup state from nested Fetch + lastValues. |
| ContinuationToken — nesting | Nested tokens (e.g., future OffsetLimit wrapping OrderBy wrapping Fetch) round-trip correctly. |
| OperationTarget — variants | Verify `PartitionKey`, `all_ranges()`, and custom `EpkRange` produce correct targets. |

### 14.2 Integration Tests

| Test Area | Cases |
|-----------|-------|
| ReadMany — basic | Read 10 items across 3 partitions, verify all returned. |
| ReadMany — missing items | Read items where some don't exist, verify present items returned. |
| ReadMany — single partition | All items in one partition, verify no unnecessary fan-out. |
| ReadMany — partition split | Trigger split during ReadMany, verify re-plan and completion. |
| ReadMany — large set | Read 1000 items, verify server-side pagination within each range works. |
| Query — single partition | Execute paginated query, verify continuation threading. |
| Query — resume | Execute query, get continuation, pass token back in next call, verify continues. |
| Diagnostics | Verify RU charges are aggregated across fan-out steps. |
| Throughput control | Verify fan-out respects throughput control group limits. |

### 14.3 Performance Tests

| Test Area | Metric |
|-----------|--------|
| Point op overhead | Latency regression < 1% vs. direct `execute_single_operation`. |
| ReadMany fan-out | Latency scales sub-linearly with partition count (concurrency works). |
| Memory bounds | Peak memory for ReadMany of N items is O(N × item_size). |

---

## 15. Future Work

### 15.1 Change Feed

The change feed is a specialized feed operation with unique characteristics:
- Start-from-beginning, start-from-now, or start-from-timestamp.
- Lease-based partition assignment (for multi-consumer scenarios).
- Scoped to feed ranges (EPK ranges).
- Incremental mode vs. full-fidelity mode.

The current spec reserves extension points in `OperationPayload`, `OperationTarget`,
`PlanStep`, and `ResumeState` for change feed support.

### 15.2 ORDER BY Merge-Sort

Cross-partition queries with ORDER BY require a k-way merge of sorted partition streams.
This will be implemented as a `Sort` variant of `PlanStep` that consumes partition `Fetch`
step heads and produces globally ordered pages.

### 15.3 Aggregation

Queries with aggregation functions (COUNT, SUM, AVG, etc.) require client-side accumulation
across partitions. This will be implemented as an `Aggregate` variant of `PlanStep`.

### 15.4 Payload Awareness

For sort and aggregation, the driver must understand feed response envelopes (the JSON
structure containing the items array, count, etc.). This will require a light JSON parsing
layer in the executor, not full item deserialization.

### 15.5 Hedging for Feed Operations

The existing hedging mechanism (speculative execution in secondary regions) could be extended
to individual plan steps, allowing fan-out fetches to hedge independently.
