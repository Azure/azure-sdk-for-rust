# ReadMany (Read Multiple Items by ID and Partition Key)

## Status

**Proposal** — under review, not yet implemented.

## Summary

A client-side ReadMany API for the driver crate (schema-agnostic, raw bytes) consumed
by the SDK and C FFI layers. Supports two modes:

- **(id, pk) mode** — fetch specific items by document ID and partition key.
- **PK-only mode** — fetch *all* items matching a set of partition key values (no IDs).

Both modes group inputs by physical partition using EPK hashing, dispatch work units
(point reads for single-item groups in (id,pk) mode, parameterized queries otherwise),
execute concurrently, and aggregate results.

## Motivation

1. **N point reads = N round-trips.** `read_item` is one HTTP call per item — serial
   latency grows linearly.
2. **Manual query building is error-prone.** Callers must construct `IN` clauses, handle
   partition routing, and paginate. No built-in optimization for physical partition grouping.
3. **No middle ground.** `read_all_items` fetches everything; there is no "fetch these
   specific items" primitive.

ReadMany fills the gap. No dedicated REST endpoint exists — the SDK synthesizes
per-partition queries (and point reads for single-item groups) internally.

### Prior Art

All major SDKs implement the same core algorithm:
- **Java**: `readMany()` — hybrid point-read/query strategy per partition group
- **Python**: `read_items()` — EPK grouping, chunks at 1,000, concurrent execution
- **Go**: `ReadManyItems()` — query-based with EPK range grouping ([PR #26007](https://github.com/Azure/azure-sdk-for-go/pull/26007))
- **.NET**: `ReadManyItemsAsync<T>()` — groups by partition, batched queries

## Design

### Layering

| Layer | Responsibility |
|-------|---------------|
| **Driver** (`azure_data_cosmos_driver`) | Core algorithm: EPK grouping, point reads / parameterized queries, concurrent execution, raw byte results |
| **SDK** (`azure_data_cosmos`) | Typed wrapper: `ReadManyResponse<T>` with deserialization |

### Driver API

```rust
/// Identifies a single item to read.
#[derive(Debug, Clone)]
pub struct ItemIdentity {
    id: String,
    partition_key: PartitionKey,
}

impl ItemIdentity {
    pub fn new(id: impl Into<String>, partition_key: PartitionKey) -> Self {
        Self { id: id.into(), partition_key }
    }
    pub fn id(&self) -> &str { &self.id }
    pub fn partition_key(&self) -> &PartitionKey { &self.partition_key }
}
```

`ReadManyInput` unifies both modes through a single API entry point:

```rust
/// What to read: specific items by (id, pk) or all items matching PKs.
pub enum ReadManyInput {
    /// Fetch specific items by document ID and partition key.
    Items(Vec<ItemIdentity>),
    /// Fetch all items matching these partition key values (no IDs).
    /// Always uses queries, never point reads.
    PartitionKeys(Vec<PartitionKey>),
}
```

```rust
/// Options for ReadMany.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ReadManyOptions {
    /// Maximum concurrent sub-operations. Default: `min(num_ranges, 32)`.
    pub max_concurrency: Option<usize>,
}
```

```rust
/// Response from a ReadMany operation.
#[derive(Debug)]
pub struct ReadManyResponse {
    /// Raw JSON bodies. Items not found are silently omitted.
    /// Order is **unspecified**.
    items: Vec<Vec<u8>>,
    request_charge: RequestCharge,
    diagnostics: OperationDiagnostics,
    activity_id: Option<ActivityId>,
}

impl ReadManyResponse {
    pub fn items(&self) -> &[Vec<u8>] { &self.items }
    pub fn into_items(self) -> Vec<Vec<u8>> { self.items }
    pub fn request_charge(&self) -> RequestCharge { self.request_charge }
    pub fn diagnostics(&self) -> &OperationDiagnostics { &self.diagnostics }
    pub fn activity_id(&self) -> Option<&ActivityId> { self.activity_id.as_ref() }
}
```

```rust
impl CosmosDriver {
    /// Reads multiple items identified by (id, pk) pairs, or all items
    /// matching a set of partition key values.
    ///
    /// In (id,pk) mode, single-item partition groups use point reads
    /// (1 RU, lowest latency). Multi-item groups use batched parameterized
    /// queries. In PK-only mode, all groups use queries.
    ///
    /// All work units execute concurrently. Items not found are silently
    /// omitted.
    pub async fn read_many(
        &self,
        container: &ContainerReference,
        input: ReadManyInput,
        options: ReadManyOptions,
        operation_options: OperationOptions,
    ) -> azure_core::Result<ReadManyResponse>;
}
```

### Algorithm

```text
read_many(container, input, read_many_options, operation_options)
  │
  ├─ 1. Validate inputs
  │     ├─ Items mode: non-empty, each id non-empty
  │     ├─ PK-only mode: non-empty
  │     └─ MULTI_HASH: validate PK component count
  │
  ├─ 2. Get container metadata (PK definition, container RID)
  │
  ├─ 3. Fetch partition key ranges
  │     └─ GET /dbs/{db}/colls/{coll}/pkranges
  │
  ├─ 4. Group inputs by physical partition range
  │     ├─ Compute EPK hash per item/PK (MurmurHash3 V1/V2)
  │     ├─ Binary search sorted ranges → owning range
  │     ├─ Items mode: HashMap<RangeId, Vec<ItemIdentity>>
  │     └─ PK-only mode: HashMap<RangeId, Vec<PartitionKey>>
  │
  ├─ 5. Create work units per partition group
  │     ├─ Items mode, 1 item  → PointRead (GET /docs/{id})
  │     ├─ Items mode, 2+ items → Query chunks (max 1,000 per chunk)
  │     └─ PK-only mode → Query chunks (always)
  │
  ├─ 6. Build request for each work unit
  │     ├─ PointRead: GET with x-ms-documentdb-partitionkey
  │     │   404 → silently omit
  │     ├─ Query (Items mode): Shape 1, 2, or 3 (see below)
  │     └─ Query (PK-only mode): Shape 4 (see below)
  │
  ├─ 7. Execute all work units concurrently
  │     ├─ Concurrency: max_concurrency option (default min(num_ranges, 32))
  │     ├─ Paginate queries until continuation exhausted
  │     ├─ On 410 response: enter retry policy (see below)
  │     └─ On non-retryable error (except 404): cancel remaining, propagate
  │
  └─ 8. Aggregate: concatenate items, sum RU, merge diagnostics
  │     └─ Single OperationDiagnostics per response (not per sub-op)
```

### Query Shapes

All queries use parameterized values. Shape selection depends on the mode, PK
definition, and item distribution within each chunk.

#### (id, pk) Mode Shapes

**Shape 1 — ID-Only IN** (PK path is `/id`):

```sql
SELECT * FROM c WHERE c.id IN (@p0, @p1, @p2)
```

**Shape 2 — Single-PK + ID IN** (all items share one logical PK):

```sql
SELECT * FROM c WHERE c.myPk = @pk AND c.id IN (@p0, @p1, @p2)
```

**Shape 3 — OR-of-Conjunctions** (mixed logical PKs in same physical range):

```sql
SELECT * FROM c WHERE
  (c.id = @id0 AND c.myPk = @pk0) OR
  (c.id = @id1 AND c.myPk = @pk1)
```

Hierarchical partition keys add all PK components per conjunction:

```sql
SELECT * FROM c WHERE
  (c.id = @id0 AND c.tenantId = @t0 AND c.userId = @u0) OR
  (c.id = @id1 AND c.tenantId = @t1 AND c.userId = @u1)
```

**PK path `/id` with pk ≠ id**: Error. When the partition key path is `/id`, the
partition key value *must* equal the document ID. A mismatch is a caller bug — do not
silently fall back to Shape 3.

#### PK-Only Mode Shapes

**Shape 4 — PK IN** (single-level PK):

```sql
SELECT * FROM c WHERE c.myPk IN (@pk0, @pk1, @pk2)
```

**Shape 4h — Hierarchical PK OR-of-Conjunctions**:

```sql
SELECT * FROM c WHERE
  (c.tenantId = @t0 AND c.userId = @u0) OR
  (c.tenantId = @t1 AND c.userId = @u1)
```

PK-only mode never uses point reads — there is no document ID, so a point read is
not possible. Each partition group always produces query work units.

The key optimization for PK-only mode is EPK-aware routing: each physical partition
receives *only* the PK values whose EPK falls within that range. Without this, a
cross-partition `WHERE c.pk IN (...)` query forces every partition to perform index
lookups for PK values that provably cannot exist on that partition, wasting RUs.

**Special cases:**
- Null PK → `IS_NULL(c.pk)`
- Undefined PK → `NOT IS_DEFINED(c.pk)`
- Nested paths (`/address/zipCode`) → `c["address"]["zipCode"]`
- Non-identifier paths (`/my-pk`) → `c["my-pk"]`

**Field expression mapping:**

| PK Path | SQL Expression |
|---------|---------------|
| `/pk` | `c.pk` |
| `/address/zipCode` | `c["address"]["zipCode"]` |
| `/my-pk` | `c["my-pk"]` |
| `/a/b/c` | `c["a"]["b"]["c"]` |

### Partition Key Range Fetching

```rust
// New factory method on CosmosOperation
pub fn read_partition_key_ranges(container: ContainerReference) -> Self {
    let resource_ref = CosmosResourceReference::from(container)
        .with_resource_type(ResourceType::PartitionKeyRange)
        .into_feed_reference();
    Self::new(OperationType::ReadFeed, resource_ref)
}
```

Response:

```json
{
  "PartitionKeyRanges": [
    { "id": "0", "minInclusive": "", "maxExclusive": "05C1DFFFFFFFFC" },
    { "id": "1", "minInclusive": "05C1DFFFFFFFFC", "maxExclusive": "FF" }
  ]
}
```

### EPK Hashing

The SDK crate has MurmurHash3 V1/V2 in `murmur_hash.rs` and EPK construction logic in
`hash.rs`. **Recommended**: move EPK hashing into the driver crate — it's a pure
function of PK values and PK definition with no SDK-layer dependencies. This lets the
C FFI layer use ReadMany without re-implementing hashing.

### Point Read Optimization

When a partition group contains exactly **one item**, issue a point read instead of
a query:

| | Point Read | Query (1 item) |
|---|---|---|
| **RU cost** | 1 RU (≤1KB) | >1 RU (query overhead) |
| **Latency** | Index lookup | Plan compilation + execution |
| **Not found** | 404 → skip | Empty result set |

Threshold: **1 item → point read, 2+ items → query**. At 2 items, a query costs
~2 RUs (comparable to 2 point reads) but saves an HTTP round-trip.

### Concurrency

Work units execute concurrently via async tasks (runtime-agnostic — no direct Tokio
dependency). Default concurrency: `min(num_ranges, 32)`, configurable via
`ReadManyOptions::max_concurrency`. On first non-retryable error (excluding point read
404 and 410 retries), remaining tasks are cancelled and the error propagates.

### Error Handling

| Condition | Behavior |
|-----------|----------|
| Empty input | Error immediately |
| Empty item ID (Items mode) | Error immediately |
| PK path `/id` with pk ≠ id | Error immediately |
| Multi-hash PK component mismatch | Error immediately |
| Point read 404 | Silently omit |
| Query returns fewer items | Silently omit missing |
| Partition split (410/1002) | Refresh PK ranges, re-group affected items, retry |
| Partition merge (410/1007) | Refresh PK ranges, merge groups to new range, retry |
| Stale collection cache (410/1000) | Invalidate cache, re-fetch metadata, retry entire operation |
| Throttling (429) | Handled by existing retry policy |
| Other transient failure | Propagate error, cancel remaining tasks |

### 410 Retry Policy

A 410 (Gone) response from any sub-operation triggers a retry flow based on its
sub-status code. Only the affected work units are retried — successfully completed
work units are not re-executed.

```text
work unit returns 410
  │
  ├─ Sub-status 1002 (partition split)
  │     ├─ The physical partition was split into two or more child ranges.
  │     ├─ Refresh PK ranges: GET /dbs/{db}/colls/{coll}/pkranges
  │     ├─ Re-compute EPK for each item/PK in the failed work unit
  │     ├─ Re-group into the new child range(s)
  │     ├─ Build new work unit(s) for each child range
  │     └─ Retry only those new work units
  │
  ├─ Sub-status 1007 (partition merge)
  │     ├─ Two or more physical partitions were merged into one.
  │     ├─ Refresh PK ranges
  │     ├─ Re-group items from the failed work unit into the merged range
  │     ├─ If other in-flight work units targeted ranges that were also
  │     │   merged, cancel them and merge their items into the new group
  │     └─ Retry the consolidated work unit
  │
  └─ Sub-status 1000 (stale collection / name cache)
        ├─ The collection was recreated or metadata is stale.
        ├─ Invalidate all cached metadata (container RID, PK definition,
        │   PK ranges)
        └─ Retry the entire ReadMany operation from step 2
```

Retry limit: bounded by the driver's existing retry policy (default 3 attempts per
sub-operation). If the retry budget is exhausted, propagate the 410 as an error.

### Additional Behaviors

- **Pagination**: Each query chunk may return continuation tokens — loop until exhausted.
- **Chunking**: `MAX_ITEMS_PER_QUERY = 1_000` (matches Python/Go SDKs).
- **Diagnostics aggregation**: A single `OperationDiagnostics` per `ReadManyResponse`
  — not one per sub-operation. Merge RU, latency, retries, and region contacts from
  all sub-operations into this single value. 1:1 cardinality between request and
  diagnostics.
- **End-to-end timeout**: If configured, applies to the entire ReadMany — not per
  sub-operation. Exceeding budget cancels remaining work units.
- **Thread safety**: `read_many` takes `&self`. `ReadManyResponse` is `Send + Sync`.
- **Response ordering**: Unspecified. SDK layer can re-order if needed.

## SDK API (`azure_data_cosmos`)

```rust
pub use azure_data_cosmos_driver::models::{ItemIdentity, ReadManyInput};

#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ReadManyOptions {
    pub session_token: Option<SessionToken>,
    pub custom_headers: HashMap<HeaderName, HeaderValue>,
    /// Maximum concurrent sub-operations.
    pub max_concurrency: Option<usize>,
}

pub struct ReadManyResponse<T> {
    items: Vec<T>,
    request_charge: f64,
    diagnostics: OperationDiagnostics,
    activity_id: Option<ActivityId>,
}

impl<T> ReadManyResponse<T> {
    pub fn items(&self) -> &[T] { &self.items }
    pub fn into_items(self) -> Vec<T> { self.items }
    pub fn request_charge(&self) -> f64 { self.request_charge }
    pub fn diagnostics(&self) -> &OperationDiagnostics { &self.diagnostics }
    pub fn activity_id(&self) -> Option<&ActivityId> { self.activity_id.as_ref() }
}
```

```rust
impl ContainerClient {
    /// Reads multiple items by (id, pk) pairs or by partition key values.
    ///
    /// ```rust no_run
    /// # async fn example(container: ContainerClient) -> azure_core::Result<()> {
    /// // (id, pk) mode
    /// let items = ReadManyInput::Items(vec![
    ///     ItemIdentity::new("item1", PartitionKey::from("pk-a")),
    ///     ItemIdentity::new("item2", PartitionKey::from("pk-b")),
    /// ]);
    /// let response = container.read_many::<serde_json::Value>(items, None).await?;
    ///
    /// // PK-only mode — fetch all items in these partitions
    /// let pks = ReadManyInput::PartitionKeys(vec![
    ///     PartitionKey::from("pk-a"),
    ///     PartitionKey::from("pk-b"),
    /// ]);
    /// let response = container.read_many::<serde_json::Value>(pks, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read_many<T: DeserializeOwned>(
        &self,
        input: ReadManyInput,
        options: Option<ReadManyOptions>,
    ) -> azure_core::Result<ReadManyResponse<T>>;
}
```

SDK flow: resolve container → map `ReadManyInput` + `ReadManyOptions` to driver
types → call `driver.read_many()` → deserialize raw bytes into `T` → return
`ReadManyResponse<T>`.

## C FFI API (`azure_data_cosmos_native`)

```c
typedef struct {
    const char* id;
    const cosmos_partition_key* partition_key;
} cosmos_item_identity;

typedef struct {
    const uint8_t** items;
    const size_t* item_lengths;
    size_t item_count;
    double request_charge;
} cosmos_read_many_response;

// (id, pk) mode
cosmos_error cosmos_container_read_many(
    const cosmos_container_client* client,
    const cosmos_item_identity* items,
    size_t item_count,
    const cosmos_operation_options* options,
    cosmos_read_many_response** out_response);

// PK-only mode
cosmos_error cosmos_container_read_many_by_partition_keys(
    const cosmos_container_client* client,
    const cosmos_partition_key** partition_keys,
    size_t pk_count,
    const cosmos_operation_options* options,
    cosmos_read_many_response** out_response);

void cosmos_read_many_response_free(cosmos_read_many_response* response);
```

## Files to Create or Modify

### Driver Crate (`azure_data_cosmos_driver`)

| File | Action | Description |
|------|--------|-------------|
| `src/models/item_identity.rs` | Create | `ItemIdentity`, `ReadManyInput` types |
| `src/models/read_many_response.rs` | Create | `ReadManyResponse` type |
| `src/models/read_many_options.rs` | Create | `ReadManyOptions` (max_concurrency) |
| `src/models/partition_key_range.rs` | Create | `PartitionKeyRange` boundaries |
| `src/models/mod.rs` | Modify | Export new types |
| `src/driver/read_many.rs` | Create | Core algorithm |
| `src/driver/query_builder.rs` | Create | Parameterized query builder (3 shapes) |
| `src/driver/epk.rs` | Create | EPK hashing (from SDK crate) |
| `src/driver/cosmos_driver.rs` | Modify | Add `read_many()` |
| `src/driver/mod.rs` | Modify | Declare new modules |
| `src/models/cosmos_operation.rs` | Modify | Add `read_partition_key_ranges()` |

### SDK Crate (`azure_data_cosmos`)

| File | Action | Description |
|------|--------|-------------|
| `src/clients/container_client.rs` | Modify | Add `read_many<T>()` |
| `src/options/mod.rs` | Modify | Add `ReadManyOptions` |
| `src/models/mod.rs` | Modify | Re-export `ItemIdentity`, `ReadManyInput`, add `ReadManyResponse<T>` |
| `src/hash.rs` | Modify | Move EPK functions to driver |
| `src/lib.rs` | Modify | Export new types |

### Native Crate (`azure_data_cosmos_native`)

| File | Action | Description |
|------|--------|-------------|
| `src/clients/cosmos_container_client.rs` | Modify | Add FFI function |
| `include/azurecosmos.h` | Regenerated | cbindgen |

## Test Cases

### Unit Tests

| ID | Test | Verifies |
|----|------|----------|
| U1 | `query_builder_id_only_in` | PK path `/id` → Shape 1 |
| U2 | `query_builder_id_pk_mismatch_error` | PK path `/id`, pk ≠ id → error (not Shape 3) |
| U3 | `query_builder_single_pk` | Same PK → Shape 2 |
| U4 | `query_builder_multi_pk` | Mixed PKs → Shape 3 |
| U5 | `query_builder_nested_pk_path` | `/address/zip` → bracket notation |
| U6 | `query_builder_non_identifier_pk` | `/my-pk` → bracket notation |
| U7 | `query_builder_null_pk` | `IS_NULL(c.pk)` |
| U8 | `query_builder_undefined_pk` | `NOT IS_DEFINED(c.pk)` |
| U9 | `query_builder_hierarchical_pk` | Multi-path → multiple conditions |
| U10 | `single_item_uses_point_read` | 1 item → point read work unit |
| U11 | `epk_grouping_single_range` | All items → one chunk |
| U12 | `epk_grouping_multi_range` | Items across ranges → multiple chunks |
| U13 | `chunking_at_1000` | 2,500 items → 3 chunks |
| U14 | `empty_items_error` | Empty list → error |
| U15 | `empty_id_error` | Empty ID → error |
| U16 | `field_expression_simple` | `/pk` → `c.pk` |
| U17 | `field_expression_nested` | `/a/b/c` → `c["a"]["b"]["c"]` |
| U18 | `mixed_point_read_and_query` | 1+5+2 items → 1 point read + 2 queries |
| U19 | `multi_hash_pk_component_count` | Wrong component count → error |
| U20 | `pk_only_single_level` | PK-only → Shape 4 query |
| U21 | `pk_only_hierarchical` | PK-only hierarchical → Shape 4h query |
| U22 | `pk_only_no_point_reads` | PK-only mode never produces point read work units |
| U23 | `pk_only_epk_grouping` | PK values grouped to correct physical ranges |
| U24 | `max_concurrency_option` | Custom max_concurrency limits parallel work units |

### Integration Tests (emulator)

| ID | Test | Verifies |
|----|------|----------|
| E1 | `read_many_basic` | 10 items round-trip |
| E2 | `read_many_missing_items` | Missing items silently omitted |
| E3 | `read_many_mixed_partitions` | Cross-partition correctness |
| E4 | `read_many_large_batch` | 1,500 items (chunking boundary) |
| E5 | `read_many_single_item` | ReadMany with 1 item |
| E6 | `read_many_hierarchical_pk` | Multi-path PK container |
| E7 | `read_many_request_charge` | Aggregate RU > 0 |
| E8 | `read_many_session_consistency` | Session token propagation |
| E9 | `read_many_point_read_fallback` | Single-item groups use point read (RU check) |
| E10 | `read_many_diagnostics_aggregated` | Single OperationDiagnostics per response |
| E11 | `read_many_pk_only_basic` | PK-only: fetch items by PK values |
| E12 | `read_many_pk_only_mixed_partitions` | PK-only: PKs spanning multiple physical partitions |
| E13 | `read_many_pk_only_hierarchical` | PK-only: hierarchical PK container |
| E14 | `read_many_partition_split_retry` | 410/1002 → refresh ranges, retry |

## Open Questions

1. **EPK hashing location**: Driver crate (recommended for C FFI reuse) or SDK crate?
2. **Response ordering**: No guarantee (recommended, matches Go SDK). SDK can re-order.
3. **Duplicate items**: Deduplicate at grouping stage (recommended). At most one copy returned.
4. **Projections**: Allow callers to specify field paths (`SELECT c.id, c.name` instead of
   `SELECT *`)? Could significantly reduce RU and bandwidth for large documents, but adds
   API complexity and only applies to query work units (point reads always return full
   documents). Ship without and add later, or include in v1?
