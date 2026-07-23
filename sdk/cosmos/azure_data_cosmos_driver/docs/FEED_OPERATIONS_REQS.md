<!-- cspell:ignore rescan -->
# Feed Operations — Requirements & Design Primer

**Crate:** `azure_data_cosmos_driver`
**Scope:** Driver-internal architecture for feed operations (queries, future read-many, change feed)
**Current focus:** `SELECT * [WHERE <predicate>]` using natural order, cross-partition streaming `ORDER BY`, and change feed

---

## 1. Context

The driver currently handles only point operations (single request → single response). Feed operations produce multiple pages of results, may span many physical partitions, and require resumable pagination state that survives process boundaries.

Feed operations must flow through the same execution infrastructure as point operations (region failover, session tokens, retry, diagnostics) without penalizing point-operation latency. We are doing multi-millisecond network I/O per page fetch, so the design optimizes for clarity and correctness over nanosecond-level micro-optimization.

---

## 2. Dataflow Pipeline

All operations — point and feed — are expressed as a **Dataflow Pipeline**: a tree of nodes where leaf nodes perform I/O and intermediate nodes perform sequencing or aggregation.

### Structure

- The pipeline is a **tree**. Nodes own their children. Fan-out creates branching.
- **Leaf nodes** issue a single Cosmos DB request via the existing operation pipeline (retry, failover, auth, transport).
- **Intermediate nodes** orchestrate their children. `SequentialDrain` iterates children in EPK order, fully draining one before advancing to the next (natural-order queries). `UnorderedMerge` polls children round-robin without evicting them (change feed). `StreamingOrderedMerge` k-way merges globally-ordered results across children, each executing a Gateway-rewritten per-partition query (cross-partition `ORDER BY`) — see §8 and `driver::dataflow::streaming_ordered_merge` for the implementation.
- **Trivial pipelines** (point operations, single-partition feeds) are a single leaf node with no intermediate parent. These must add near-zero overhead compared to today's direct execution path.

### Pipeline Lifecycle

- `execute_operation` is called once per page. Each call advances the pipeline by one page of results from one physical partition.
- The pipeline object itself is the in-process iteration state. The consumer of the driver (SDK layer) is responsible for holding the pipeline across calls.
- For cross-process resumption, the pipeline state serializes to a `ContinuationToken` string. On resume, the token reconstitutes a pipeline at the correct position.
- It is cleanest to unify these: `ContinuationToken` holds the live pipeline object in-process, and produces the serialized string form on demand.

### Future Node Types (Design For, Don't Implement Yet)

- **BufferedOrderedMerge**: collect all results, then sort (non-streaming `ORDER BY` — issue #4755).
- **HybridSearch**: issues multiple distinct sub-queries (e.g., vector similarity + full-text keyword) against different child pipelines, then combines/re-ranks their results. Demonstrates that an intermediate node may have heterogeneous children with different semantics.
- **Aggregate**: client-side aggregation across partitions.

Implemented since this primer was written: **UnorderedMerge** (change feed fan-out) and **StreamingOrderedMerge** (streaming cross-partition `ORDER BY`; see §8).

---

## 3. Key Invariants

### Ordering

When no `ORDER BY` is specified, the driver guarantees results in **(EPK, RID) ascending order**. Within each physical partition, the server returns items in ascending RID order. Across partitions, the driver iterates in ascending EPK order. This is a driver-level guarantee for `SELECT *` queries.

### Page Boundaries & Suspension

- For the initial `SequentialDrain` implementation, suspension occurs at page boundaries. A continuation token for this node type only needs to track which partition is active and the server's opaque page token for that partition.
- The continuation token design must allow future node types to store intra-page progress (e.g., a streaming ORDER BY merge may suspend mid-page when its output buffer is full but source partitions are partially consumed).
- A given server continuation token only guarantees you get the *next* page of results from that partition — even if the SDK presents a per-item iterator to the user.

### Fan-Out Limit

Cross-partition queries are expensive by design. Containers may have hundreds of thousands of physical partitions; unbounded fan-out is dangerous from a performance and scalability perspective.

- **Max fan-out**: The pipeline refuses to plan an operation spanning more than N physical partitions. Default: **100**. Configurable by the caller for workloads that intentionally query broadly.
- **Max concurrency**: A separate limit on concurrent in-flight requests within a single pipeline execution. Not needed for the initial `SELECT * WHERE` implementation (sequential drain uses concurrency = 1) but the limit must exist as a configuration point for future concurrent node types.

### Partition Targeting

An operation targets the key space in one of three mutually exclusive ways:

1. **No partition scope** — account/database-level operations.
2. **Logical partition key** — point operations and single-partition feeds. Routes via the gateway using the PK header. No EPK headers. No fan-out.
3. **Feed range (EPK range)** — cross-partition feeds. Resolved to physical partition(s) at plan time. The full container is just the special case of `[min_epk, max_epk)`.

These are mutually exclusive at the type level — not a runtime check.

---

## 4. Continuation Token & Resumption

### Dual Nature

The `ContinuationToken` type serves two roles:

1. **In-process**: holds the live pipeline state. The SDK keeps it across `execute_operation` calls. No serialization needed per page.
2. **Cross-process**: serializes to an opaque string (base64url-encoded JSON). Safe to store in databases, send to browsers, carry across SDK upgrades.

### Token Properties

- **Durable across SDK versions.** Newer SDKs must deserialize tokens from older SDKs. Version field is the option of last resort.
- **O(1) size for sequential drain.** Only the active partition's EPK bounds and server continuation are stored. Drained partitions are reconstructed from the EPK cursor on resume.
- **Bound to the operation.** Tokens include a container RID and operation kind. Replaying a token against a different container or operation type is rejected.
- **Survives partition topology changes.** Tokens store EPK bounds, not physical partition IDs. Splits and merges are handled by re-resolving EPK bounds to current partitions.

### What the Token Does NOT Encode

- Query text or parameters (caller must supply an equivalent operation).
- Session tokens or consistency state.
- Per-partition state for all partitions (only the cursor position for sequential drain).

---

## 5. Pipeline Repair (Splits)

Physical partitions can split at any time. The pipeline must handle this transparently.

### Leaf Node Invariant

At all times, a leaf node targets **one specific physical partition** and **one EPK range** that is contained within that partition and does not overlap with any of its peer leaf nodes. A leaf node can only issue one request, so it is impossible for it to target multiple physical partitions.

### Splits Break the Invariant

When a physical partition splits, a leaf node's EPK range suddenly covers two or more new physical partitions. The pipeline detects this via a 410 (PartitionIsGone) response — either a full page is returned successfully or a 410 is returned; this never occurs mid-page.

The leaf node is responsible for **splitting itself** to restore the invariant:

1. Invalidate the cached partition map for the container.
2. Re-resolve the leaf's EPK range to the new physical partition(s).
3. The single leaf becomes multiple leaves in the parent's children list (the parent must obviously cooperate with this), each targeting one of the new physical partitions with a non-overlapping sub-range of the original EPK range. Depth of the tree remains the same.
4. Execution resumes against the correct new leaf.

### Merges Do Not Require Repair

After a merge, multiple leaf nodes may point to different EPK ranges on the same physical partition. This is acceptable — the leaf still targets a single partition and uses EPK min/max headers to scope its request to its intended slice. No pipeline restructuring is needed. (Consolidating redundant leaves after a merge is a potential future optimization but is out of scope to avoid complicating the design.)

---

## 6. Current Implementation Focus

The initial implementation targets `SELECT [...] [WHERE <predicate>]` queries:

- **Single-partition**: trivial pipeline (one leaf node). The server evaluates the full SQL including any WHERE clause. Paginated via server continuations.
- **Cross-partition**: `SequentialDrain` intermediate node over N leaf nodes (one per physical partition). Drains partitions in EPK order. No query plan fetch required for passthrough SELECT/WHERE.

### What This Exercises

- Partition key range resolution and caching.
- Sequential traversal across partitions in EPK order.
- EPK range scoping via request headers.
- Paginated reads within each partition.
- Continuation token serialization, resume, and topology-change survival.
- Integration with the existing operation pipeline for each sub-request.
- Pipeline repair on partition splits/merges.

Since this initial milestone, change feed (`UnorderedMerge`, §2) and cross-partition streaming `ORDER BY` (`StreamingOrderedMerge`, §2, §7, §8) have landed as additional cross-partition strategies over the same pipeline model.

---

## 7. Design Boundaries

### The Driver Does NOT:

- Deserialize item bodies. It returns raw bytes per item; the SDK handles deserialization.
- Create telemetry spans. It returns structured diagnostics data; the SDK creates OpenTelemetry spans.
- Own the iteration lifetime for multi-page feeds. It executes one page per call; the SDK loops.
- Fetch or interpret backend query plans (for the current SELECT/WHERE scope).

### Item Body Opacity

For `SequentialDrain`, item bodies are fully opaque binary payloads. The pipeline does not inspect them — ordering is already established by the backend.

`StreamingOrderedMerge` (cross-partition `ORDER BY`) is the first node type that parses item bodies: the backend query plan's `rewrittenQuery` reshapes each per-partition result into a standardized envelope (`{"_rid": ..., "orderByItems": [...], "payload": ...}` — sort keys promoted to top-level fields, the raw user document demoted to `payload`). The pipeline parses just enough of this envelope to compare rows and emit `payload` — see `driver::dataflow::order_by` (the value/comparator/resume-value model) and `driver::dataflow::query_response` (envelope parsing and response reassembly). Hybrid search, if implemented, would need the same pattern for its own envelope shape.

Resume for streaming `ORDER BY` is per-range and topology-change-safe. Each still-active range persists its last-emitted key tuple, envelope `_rid`, and a `_rid`-tie `skipCount`. On resume — including after a partition split or merge — the boundary is sent to the backend as the .NET-compatible structured `resumeFilter` field of the query body (`{"value": [<resume values>], "rid": <token rid>, "exclude": false}` — see `driver::dataflow::query_response::with_resume_filter`), not an SDK-rewritten SQL predicate. The query text is unchanged (its Gateway resume-filter placeholder is replaced with `true`, exactly as for a fresh start), and the caller's `parameters` are preserved verbatim; sort direction lives in the query's `ORDER BY` clause. Rust persists one boundary per range and resumes each range with its own boundary in the .NET "target" partition style (`rid` present, `exclude: false`); the already-emitted prefix of the boundary tie run is discarded client-side by a three-phase `(sort key, _rid, skipCount)` seek matching .NET's `FilterNextAsync`: keys before the boundary and `_rid`s before the boundary `_rid` (numeric document-ordinal order, see `models::resource_id::compare_document_rids`) are dropped, then exactly `skipCount` rows sharing the boundary's exact `(sort key, _rid)` are dropped. `skipCount` (mirroring .NET's `OrderByContinuationToken.SkipCount`) is needed only because a JOIN/array-unwind query can emit several result rows from one document that share both the sort key and `_rid`; for ordinary single-row-per-document queries it is always `1` (just the boundary row). The seek is per row, so resume stays exact and clips to sub-ranges across a split — only the sub-range that re-returns the boundary `_rid` consumes `skipCount`. Scalar boundaries round-trip exactly; **array/object** (complex) boundaries travel as a bounded 128-bit hash (`{"type": "array"|"object", "low": <i64>, "high": <i64>}`) that is an exact port of the backend / .NET SDK structural `DistinctHash` (see `driver::dataflow::distinct_hash`), so it is byte-identical to the backend's own hash. A **saved-token** resume across a split or merge is therefore safe for scalar and complex keys alike (there is no positional-rescan or unchanged-topology restriction): the persisted boundary is replayed through the structured `resumeFilter`, and the backend's own `DistinctHash` seek excludes already-emitted complex rows. A **live** split — one hit mid-fan-out — is equally safe: `Request::split_for_topology_change` forwards the split child's own backend continuation into every replacement, so (by the Cosmos contract also relied on by `SequentialDrain` — a parent partition's continuation stays valid on each post-split child under EPK scoping) each replacement resumes *after* every already-emitted row. `StreamingOrderedMerge::handle_split` therefore installs no client-side discard on a live split's first replacement page — reapplying one would wrongly drop later JOIN rows that share the boundary `(sort key, _rid)` — and forwards only the boundary's `skipCount` bookkeeping for future snapshots. Scalar and complex live splits, and every saved-token resume, are all fully supported.

### The Driver DOES:

- Plan the pipeline (determine targeting, resolve partitions, build the node tree).
- Execute one page per call through the existing retry/failover infrastructure.
- Produce and consume continuation tokens.
- Repair the pipeline on topology changes (splits/merges).
- Enforce fan-out limits.
- Collect per-node diagnostics for the SDK to surface.

---

## 8. Future Considerations (Inform Design, Don't Implement)

These capabilities must be achievable without redesigning the pipeline model:

- **Buffered ORDER BY**: collect all partition results, sort client-side (issue #4755, non-streaming `ORDER BY`). Different intermediate node than streaming `ORDER BY` (below); same query-plan requirement.
- **Vector / Hybrid Search**: may require preliminary requests to fetch full-text statistics before issuing the main query. Multi-phase pipeline execution.
- **Read Many Items**: fan-out by (ID, PK) pairs grouped by partition. Concurrent leaf execution with an unordered merge intermediate node.

Implemented since this primer was written:

- **Streaming ORDER BY**: `StreamingOrderedMerge` k-way merges partition streams using the backend query plan's rewritten per-partition query and sort keys. See §2, §7, and `driver::dataflow::streaming_ordered_merge`.
- **Change Feed**: `UnorderedMerge` polls per-range continuation tokens round-robin; see §2.

The pipeline's tree structure, typed node hierarchy, and separation of planning from execution accommodate all of these as new node types and planning strategies without changing the core execution loop.
