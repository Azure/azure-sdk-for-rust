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
│  execute_operation(op, opts) → CosmosResponse         [point operations]       │
│  execute_feed_operation(op, opts) → FeedPager          [feed operations]       │
│                                                                                │
│  Both internally:                                                              │
│    1. Planner creates an OperationPlan                                         │
│    2. PlanExecutor runs the plan                                               │
│    3. Point ops: executor drains single page, returns CosmosResponse           │
│    4. Feed ops: executor is wrapped in FeedPager for caller iteration          │
│                                                                                │
│  ┌──────────────────────────────────────────────────────────────────────────┐  │
│  │                              PLANNER                                     │  │
│  │                                                                          │  │
│  │  Input:  CosmosOperation + OperationOptions                              │  │
│  │  Output: OperationPlan                                                   │  │
│  │                                                                          │  │
│  │  Responsibilities:                                                       │  │
│  │  ┌─ Determine targeting (single PK, EPK range, all ranges)            │  │
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
│  │  Output: Stream of FeedResponsePage (or single page for point ops)       │  │
│  │                                                                          │  │
│  │  Responsibilities:                                                       │  │
│  │  ┌─ Execute plan steps with configurable concurrency                     │  │
│  │  ├─ Each step calls execute_operation_pipeline() for HTTP                │  │
│  │  ├─ Manage continuation state across turns                               │  │
│  │  ├─ Handle partition splits (re-plan affected ranges)                    │  │
│  │  ├─ Enforce concurrency caps for fan-out                                 │  │
│  │  ├─ Integrate with throughput control                                    │  │
│  │  ├─ Emit OpenTelemetry spans per turn                                    │  │
│  │  └─ Produce continuation tokens for serialization                        │  │
│  └──────────────────────────────────────────────────────────────────────────┘  │
│                      │                                                         │
│                      ▼                                                         │
│  ┌──────────────────────────────────────────────────────────────────────────┐  │
│  │                  OPERATION PIPELINE (existing)                            │  │
│  │                                                                          │  │
│  │  execute_operation_pipeline() — unchanged                                │  │
│  │  Handles: region failover, session tokens, transport retry, auth,        │  │
│  │           429 backoff, diagnostics                                        │  │
│  └──────────────────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### Layer Separation

| Concern | Component | Location |
|---------|-----------|----------|
| Operation intent & payload | `CosmosOperation` | `models/cosmos_operation.rs` |
| Plan creation | `Planner` | `driver/feed/planner.rs` (new) |
| Plan model | `OperationPlan`, `PlanStep` | `driver/feed/plan.rs` (new) |
| Plan execution | `PlanExecutor` | `driver/feed/executor.rs` (new) |
| Public pager | `FeedPager` | `driver/feed/pager.rs` (new) |
| Continuation state | `ContinuationToken` | `models/continuation_token.rs` (new) |
| Per-step HTTP execution | `execute_operation_pipeline` | `driver/pipeline/` (existing) |

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

Partition targeting is currently a single `Option<PartitionKey>` field. Feed operations introduce
additional targeting modes. These are mutually exclusive, so they become an enum:

```rust
/// How the operation is targeted to partitions.
///
/// Determines which partition key range(s) the operation executes against.
/// Only one targeting mode is active per operation.
#[derive(Clone, Debug)]
pub enum OperationTarget {
    /// No partition targeting (account-level or database-level operations).
    None,

    /// Target a specific logical partition key.
    /// Used for: single-partition reads, writes, queries.
    PartitionKey(PartitionKey),

    /// Target a specific effective partition key range.
    /// Used for: scoped feed operations on a sub-range.
    EpkRange {
        min_inclusive: EffectivePartitionKey,
        max_exclusive: EffectivePartitionKey,
    },

    /// Target a specific partition key range by its server-assigned ID.
    /// Used for: resuming from a continuation that recorded the range ID.
    PkRangeId(String),

    /// Target all partition key ranges.
    /// Semantically equivalent to `EpkRange { min: "00", max: "FF" }`.
    /// Used for: cross-partition queries, read-all-items, read-many.
    AllRanges,
}
```

### 3.3 Factory Method Updates

Existing factory methods are updated to use `OperationPayload` and `OperationTarget`:

```rust
impl CosmosOperation {
    /// Reads an item.
    pub fn read_item(item: ItemReference) -> Self {
        let partition_key = item.partition_key().clone();
        Self::new(OperationType::Read, item)
            .with_target(OperationTarget::PartitionKey(partition_key))
            // No payload needed — item ID is in the resource reference.
    }

    /// Creates an item. Use `with_body()` to provide the document JSON.
    pub fn create_item(container: ContainerReference, partition_key: PartitionKey) -> Self {
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
            .with_target(OperationTarget::AllRanges)
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
            .with_target(OperationTarget::AllRanges)
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

An `OperationPlan` is a directed acyclic graph (DAG) of `PlanStep` nodes. Each step represents
a unit of work that produces a partial result. Steps may depend on other steps (for merge/sort),
and may produce continuations for subsequent turns.

```rust
/// A plan for executing an operation.
///
/// Plans range from trivial (single step for a point read) to complex
/// (fan-out across partition key ranges with merge). The plan is created
/// by the Planner and executed by the PlanExecutor.
pub(crate) struct OperationPlan {
    /// The steps in this plan, indexed by StepId.
    steps: Vec<PlanStep>,

    /// Which step produces the final output.
    /// For single-step plans, this is step 0.
    /// For fan-out plans, this is typically a merge step.
    output_step: StepId,

    /// Whether this plan supports pagination (multiple turns).
    /// ReadMany plans do not paginate; query plans do.
    paginates: bool,
}

/// A unique identifier for a step within an OperationPlan.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct StepId(usize);

/// A single step in an operation plan.
pub(crate) enum PlanStep {
    /// Execute a single HTTP request via the operation pipeline.
    ///
    /// This is the leaf step that actually talks to the Cosmos DB service.
    /// It carries a `CosmosOperation` configured for a specific PK range.
    Fetch {
        /// The operation to execute. Targeted to a specific PK range.
        operation: CosmosOperation,
        /// Options for this fetch.
        options: OperationOptions,
        /// Server-provided continuation token for this range, if resuming.
        continuation: Option<String>,
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
        /// The steps whose results are merged.
        inputs: Vec<StepId>,
    },

    // Future variants:
    // OrderedMerge { inputs: Vec<StepId>, order_by: ... },
    // Aggregate { inputs: Vec<StepId>, aggregation: ... },
}
```

### 4.2 Plan Examples

#### Point Operation (ReadItem)

```text
Step 0: Fetch(read_item operation) → output
```

A trivial plan with one step. The executor runs it, gets a `CosmosResponse`, done.

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
the `Fetch` step to a point read (`OperationType::Read` with `OperationTarget::PartitionKey`)
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
         (or OrderedMerge for explicit ORDER BY)
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
direct `execute_operation_pipeline` call. Implementation strategies:

- **Inline the trivial case**: When the plan is a single `Fetch` step with no dependencies,
  the executor can skip graph traversal and directly call `execute_operation_pipeline`.
- **Stack allocation**: Trivial plans can use a fixed-size array or inline representation
  rather than heap-allocated `Vec<PlanStep>`.

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
| QueryItems (cross-partition) | `AllRanges` | Resolve PK ranges → N `Fetch` steps + `UnorderedMerge`. May fetch query plan. |
| ReadMany | `AllRanges` | Group items by PK range → N `Fetch` steps + `UnorderedMerge`. No pagination. |
| ReadAllItems (cross-partition) | `AllRanges` | Resolve PK ranges → N `Fetch` steps + `UnorderedMerge`. Paginated. |
| ChangeFeed (future) | varies | TBD |

### 5.3 Operation Decomposition: From One `CosmosOperation` to Many

A key responsibility of the Planner is decomposing a single caller-provided `CosmosOperation`
into multiple targeted `CosmosOperation` instances — one per partition key range — that each
flow through `execute_operation_pipeline` independently. This section illustrates the full
decomposition for two representative operations.

#### Example: Cross-Partition Query

The caller creates a single operation:

```rust
let op = CosmosOperation::query_items_cross_partition(
    container.clone(),
    "SELECT * FROM c WHERE c.status = 'active'",
);
// op.target == OperationTarget::AllRanges
// op.payload == OperationPayload::Query { query: "SELECT ...", parameters: None }
```

The Planner resolves the container's partition key ranges (say, ranges "0", "1", "2") and
produces a plan with **three separate `CosmosOperation`** instances:

```text
Caller's CosmosOperation
  target: AllRanges
  payload: Query { "SELECT * FROM c WHERE c.status = 'active'" }
                │
                ▼
          ┌─── Planner ───┐
          │  Resolve PK    │
          │  ranges: 0,1,2 │
          └───────┬────────┘
      ┌───────────┼───────────┐
      ▼           ▼           ▼
 CosmosOperation  CosmosOperation  CosmosOperation
  type: Query      type: Query      type: Query
  target:          target:          target:
   PkRangeId("0")   PkRangeId("1")   PkRangeId("2")
  payload:         payload:         payload:
   Query{same SQL}  Query{same SQL}  Query{same SQL}
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
              FeedResponsePage
```

Each decomposed `CosmosOperation` carries the same query payload but is **retargeted** to a
specific PK range ID. The operation pipeline handles region failover, retry, and auth for each
independently.

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
  target: AllRanges
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
   PkRangeId("0")   PK(pk_c)        PkRangeId("2")
  payload:         payload:         payload:
   Body{query on    None (point     Body{query on
   (pk_a,id_a),     read of id_c)   (pk_d,id_d),
   (pk_b,id_b)}                     (pk_e,id_e)}
```

Note two things:
1. The ReadMany query for each PK range filters on **both partition key and ID**, because
   ID alone is not unique — only (PartitionKey, ID) is unique within a container.
2. PK range "1" contains only a single item, so the Planner **optimizes it to a point read**
   (`OperationType::Read` with `OperationTarget::PartitionKey`), avoiding query overhead.

Each decomposed operation then flows through `execute_operation_pipeline` independently.

### 5.4 Query Plan Fetching

For cross-partition queries, the Planner may need a backend query plan to determine:
- Which partitions to target
- Whether the query requires client-side sort/aggregate
- Optimized partition routing

The Planner uses a **callback** to fetch the query plan, keeping it transport-decoupled. The
callback internally calls `execute_operation_pipeline` (not `execute_operation`), avoiding
re-entry into the Planner. The `OperationType::QueryPlan` variant already exists for this.

```rust
// The Planner calls this callback, which the driver wires to
// execute_operation_pipeline directly (bypassing the Planner).
async fn fetch_query_plan(
    operation: &CosmosOperation,
    options: &OperationOptions,
) -> azure_core::Result<BackendQueryPlan> {
    let query_plan_op = CosmosOperation::query_plan(
        operation.container().unwrap().clone(),
        /* query text from operation payload */
    );
    let response = execute_operation_pipeline(query_plan_op, options, ...).await?;
    BackendQueryPlan::from_response(response)
}
```

This avoids the recursion concern: `fetch_query_plan` calls `execute_operation_pipeline`
directly, which is the internal pipeline function, not the public `execute_operation` that
goes through the Planner.

### 5.5 Resuming from a Continuation Token

When a `ContinuationToken` is provided, the Planner uses it to reconstruct the plan state:

1. Validate the token version and operation compatibility.
2. Restore per-range continuation state.
3. Skip ranges that are already completed.
4. If a PK range ID in the token no longer exists (partition split), re-resolve using the
   EPK range bounds stored in the token and map to the new PK range(s).

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
    /// Executes the next turn, producing one page of results.
    ///
    /// Returns `None` when the plan is complete (no more pages).
    ///
    /// For non-paginating plans (ReadMany), the first call drives all
    /// steps to completion and returns the merged result. Subsequent
    /// calls return `None`.
    pub async fn next_turn(
        &mut self,
        driver_context: &DriverContext,
    ) -> azure_core::Result<Option<FeedResponsePage>> {
        // ...
    }

    /// Serializes the current execution state into a continuation token.
    ///
    /// Returns `None` if the plan is complete or does not support pagination.
    pub fn continuation_token(&self) -> Option<ContinuationToken> {
        // ...
    }
}
```

### 6.2 Turn Execution

Each call to `next_turn`:

1. **Emit OpenTelemetry span** for this turn (child of the feed operation span, linked to root).
2. **Identify runnable steps** — steps whose dependencies are satisfied.
3. **Execute runnable steps concurrently** (up to concurrency cap), each via
   `execute_operation_pipeline`.
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
/// `FeedOperationOptions::max_concurrency`.
concurrency_limit: usize,
```

The executor uses a semaphore or similar mechanism to limit concurrent
`execute_operation_pipeline` calls. Each concurrent call independently goes through the
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
6. Returns a single `FeedResponsePage` containing all items.
7. Subsequent `next_turn` calls return `None` (ReadMany does not paginate).

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

- **Caller drops the `FeedPager`**: In-flight `execute_operation_pipeline` futures are
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

### 7.1 Token Structure

```rust
/// A typed continuation token for resuming a feed operation.
///
/// Opaque to callers. Serializes to a string via `Display` and
/// deserializes via `FromStr`. The internal representation is
/// versioned and validated on deserialization.
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
    container_rid: String,

    /// The operation kind this token is valid for.
    operation_kind: ContinuationOperationKind,

    /// Per-partition-range state.
    ranges: Vec<RangeContinuation>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RangeContinuation {
    /// The EPK bounds of this range (stable across splits).
    min_inclusive_epk: String,
    max_exclusive_epk: String,

    /// The PK range ID at the time the continuation was created.
    /// Used as a hint for fast resolution; falls back to EPK bounds
    /// if the range has split.
    pk_range_id: String,

    /// Server-provided continuation token for this range.
    /// `None` means this range is completed.
    server_continuation: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum ContinuationOperationKind {
    Query,
    ReadFeed,
    // Future: ChangeFeed, etc.
}
```

### 7.2 Serialization

`ContinuationToken` implements `Display` and `FromStr`. The wire format is base64-encoded JSON:

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

### 7.3 Compatibility Contract

A continuation token is **invalidated** by:

1. **Container recreation** — The token's `container_rid` won't match the new container's RID.
2. **Token version mismatch** — A token produced by a newer SDK version may not be readable
   by an older version.
3. **Operation kind mismatch** — Resuming a `Query` continuation for a `ReadFeed` operation
   is rejected.

A continuation token **survives**:

1. **Partition splits** — The token stores EPK bounds, not just PK range IDs. On resume, the
   Planner re-resolves EPK bounds to current PK range IDs.
2. **SDK version upgrades** — The token is versioned. Older token versions are supported by
   newer SDKs (backward compatible deserialization).
3. **Process boundaries** — The token is a self-contained string, safe to send to a browser
   and back.

### 7.4 What the Token Does NOT Encode

- **Query text or parameters** — The caller must provide an equivalent `CosmosOperation`.
- **Session tokens** — Session consistency is not preserved across process boundaries via
  the continuation token. The driver resolves session tokens from the `SessionManager` cache
  for each turn independently.
- **Container name or database name** — Only the RID is stored. The caller provides routing
  context via the `CosmosOperation`.

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

```rust
impl CosmosDriver {
    /// Executes a point operation (read, write, delete).
    ///
    /// Internally, this creates a trivial single-step plan and executes it.
    /// The overhead is negligible compared to the HTTP round-trip.
    pub async fn execute_operation(
        &self,
        operation: CosmosOperation,
        options: OperationOptions,
    ) -> azure_core::Result<CosmosResponse> {
        // Plan → Execute → drain single page → return CosmosResponse
    }

    /// Executes a feed operation (query, read-many, read-all).
    ///
    /// Returns a `FeedPager` that yields pages of results. The caller
    /// pulls pages by calling `next_page()`. Each page includes a
    /// continuation token for resumption.
    pub async fn execute_feed_operation(
        &self,
        operation: CosmosOperation,
        options: FeedOperationOptions,
    ) -> azure_core::Result<FeedPager> {
        // Plan → wrap executor in FeedPager → return
    }
}
```

### 10.2 FeedPager

`FeedPager` is the public-facing page iterator. It wraps the internal `PlanExecutor` and
provides a stable API that does not expose plan/executor internals.

```rust
/// An iterator over pages of feed operation results.
///
/// Created by [`CosmosDriver::execute_feed_operation`]. Yields pages
/// of results until the operation is complete.
///
/// Dropping the `FeedPager` cancels any in-flight requests.
pub struct FeedPager {
    executor: PlanExecutor,
}

impl FeedPager {
    /// Fetches the next page of results.
    ///
    /// Returns `Ok(None)` when no more pages are available.
    pub async fn next_page(&mut self) -> azure_core::Result<Option<FeedResponsePage>> {
        self.executor.next_turn(/* ... */).await
    }

    /// Returns the continuation token for the current position.
    ///
    /// The token can be serialized to a string and used to resume the
    /// operation later by passing it to `FeedOperationOptions::with_continuation`.
    ///
    /// Returns `None` if the operation is complete or does not support
    /// pagination (e.g., ReadMany).
    pub fn continuation_token(&self) -> Option<ContinuationToken> {
        self.executor.continuation_token()
    }
}
```

### 10.3 FeedResponsePage

```rust
/// A single page of results from a feed operation.
///
/// Contains raw response bytes and metadata. The higher-level SDK
/// handles deserialization into typed items.
pub struct FeedResponsePage {
    /// Raw response body (the items array as JSON bytes).
    body: Vec<u8>,

    /// Cosmos-specific response headers (RU charge, session token, etc.).
    headers: CosmosResponseHeaders,

    /// Diagnostics for this page (may aggregate multiple sub-request diagnostics).
    diagnostics: Arc<DiagnosticsContext>,
}
```

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
- **Client-side max item count**: Configurable via `FeedOperationOptions::max_item_count`.
- **Server continuation**: A page boundary occurs whenever the server returns a continuation
  token.

For ReadMany, there is exactly one logical page (the merged result), regardless of how many
server-side pages were consumed internally.

---

## 11. Configuration Surface

### 11.1 FeedOperationOptions

```rust
/// Options specific to feed operations.
///
/// Extends `OperationOptions` with feed-specific settings.
pub struct FeedOperationOptions {
    /// Base operation options (retry, timeout, consistency, etc.).
    base: OperationOptions,

    /// Maximum number of items per page.
    /// If not set, the server default applies.
    max_item_count: Option<u32>,

    /// Maximum number of concurrent partition key range fetches.
    /// Default: min(num_pk_ranges, 10).
    max_concurrency: Option<usize>,

    /// Continuation token for resuming a previous operation.
    continuation: Option<ContinuationToken>,
}
```

### 11.2 Layered Options Resolution

Feed operation options follow the same layered resolution as existing operation options:

1. `FeedOperationOptions` (per-call)
2. `DriverOptions` (per-driver)
3. `CosmosDriverRuntime` (global)
4. Environment variables

The `max_concurrency` and `max_item_count` fields follow the same precedence.

---

## 12. Performance & Non-Regression

### 12.1 Point Operation Overhead

The plan model MUST NOT regress point operation performance. Requirements:

- **No heap allocation** for trivial plans beyond what `execute_operation` does today.
- **No additional async machinery** (no spawning, no channels) for single-step plans.
- **Benchmark**: Point operation latency with the plan model must be within 1% of the
  current direct `execute_operation_pipeline` call.

Implementation: The `execute_operation` method detects trivial operations (based on
`OperationType` and `OperationTarget`) and calls `execute_operation_pipeline` directly,
bypassing the Planner/Executor entirely. The plan model is only instantiated for feed
operations.

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

1. Implement `OperationPlan`, `PlanStep`, `StepId`.
2. Implement `Planner` with trivial single-step planning (point ops only).
3. Implement `PlanExecutor` for single-step plans.
4. Wire `execute_operation` through Plan → Execute path (with fast-path bypass).
5. Validate no performance regression via benchmarks.

### Phase 3: ReadMany

1. Implement ReadMany planning in `Planner`:
   - Group items by PK range (via `PartitionKeyRangeCache`).
   - Create fan-out `Fetch` steps + `UnorderedMerge` step.
2. Implement `UnorderedMerge` step execution in `PlanExecutor`.
3. Implement `FeedPager` and `FeedResponsePage`.
4. Add `execute_feed_operation` to `CosmosDriver`.
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
| ContinuationToken — split recovery | Token with stale PK range ID maps to new ranges via EPK bounds. |
| OperationTarget — mutual exclusivity | Verify builder rejects invalid combinations. |

### 14.2 Integration Tests

| Test Area | Cases |
|-----------|-------|
| ReadMany — basic | Read 10 items across 3 partitions, verify all returned. |
| ReadMany — missing items | Read items where some don't exist, verify present items returned. |
| ReadMany — single partition | All items in one partition, verify no unnecessary fan-out. |
| ReadMany — partition split | Trigger split during ReadMany, verify re-plan and completion. |
| ReadMany — large set | Read 1000 items, verify server-side pagination within each range works. |
| Query — single partition | Execute paginated query, verify continuation threading. |
| Query — resume | Execute query, get continuation, resume in new FeedPager, verify continues. |
| Diagnostics | Verify RU charges are aggregated across fan-out steps. |
| Throughput control | Verify fan-out respects throughput control group limits. |

### 14.3 Performance Tests

| Test Area | Metric |
|-----------|--------|
| Point op overhead | Latency regression < 1% vs. direct `execute_operation_pipeline`. |
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
`ContinuationOperationKind`, and `PlanStep` for change feed support.

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
