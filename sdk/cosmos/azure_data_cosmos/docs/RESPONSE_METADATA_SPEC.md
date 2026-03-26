# Response Metadata Type System Design Spec

## Motivation

The SDK needs a type-safe way to expose operation-specific response metadata.
Currently all accessors live on `CosmosResponse<T>` regardless of operation type,
meaning query-only fields like `index_metrics()` are available (but always `None`)
on point operation responses.

## Design

### Type hierarchy

```rust
// Universal response — contains metadata common to ALL operations.
// T is the deserialized response body.
pub struct CosmosResponse<T> {
    response: Response<T>,
    request: CosmosRequest,
    cosmos_headers: CosmosResponseHeaders,
}

// Feed page — wraps CosmosResponse and adds items + operation-specific metadata.
// T is the item type, M is the metadata type.
pub struct FeedPage<T, M> {
    items: Vec<T>,
    continuation: Option<String>,
    metadata: M,
    cosmos_headers: CosmosResponseHeaders,
}
```

### Metadata types

```rust
/// Metadata specific to query operations.
///
/// Contains headers only populated when the corresponding request headers are set.
#[derive(Debug, Clone)]
pub struct QueryMetadata {
    /// Index utilization metrics as decoded JSON (`x-ms-cosmos-index-utilization`).
    /// Only populated when `x-ms-cosmos-populateindexmetrics` request header is set.
    index_metrics: Option<String>,

    /// Query execution metrics (`x-ms-documentdb-query-metrics`).
    /// Semicolon-delimited key=value pairs.
    /// Only populated when `x-ms-documentdb-populatequerymetrics` request header is set.
    query_metrics: Option<String>,
}

/// Metadata specific to point item operations (create, read, replace, upsert, delete).
#[derive(Debug, Clone)]
pub struct ItemMetadata {
    /// ETag for optimistic concurrency control.
    etag: Option<String>,
}

/// Metadata for change feed operations (future).
#[derive(Debug, Clone)]
pub struct ChangeFeedMetadata {
    // lsn, continuation mode, etc.
}

/// Metadata for read-many operations (future).
#[derive(Debug, Clone)]
pub struct ReadManyMetadata {
    // TBD
}

/// Metadata for resource management operations (databases, containers, throughput).
///
/// Currently empty — reserved for future fields without breaking changes.
#[derive(Debug, Clone, Default)]
pub struct ResourceMetadata {}
```

### What goes where

| Field | Location | Rationale |
| ----- | -------- | --------- |
| `diagnostics()` | `CosmosResponse`, `FeedPage` | Universal — contains `activity_id()` and `server_duration_ms()` |
| `request_charge()` | `CosmosResponse`, `FeedPage` | Universal — all operations report RU |
| `session_token()` | `CosmosResponse`, `FeedPage` | Universal — session consistency |
| `headers()` | `CosmosResponse`, `FeedPage` | Universal — raw header access |
| `status()` | `CosmosResponse` | Universal — HTTP status |
| `etag()` | `ItemMetadata` | Point-op specific — concurrency control |
| `index_metrics()` | `QueryMetadata` | Query-only — opt-in via request header |
| `query_metrics()` | `QueryMetadata` | Query-only — opt-in via request header |

### Principle

If a header/field is only meaningful for a specific operation type, it goes in that
operation's metadata type. If it's meaningful across all operations, it stays on
`CosmosResponse`.

### Return types by operation

**Point operations → `CosmosResponse<T, ItemMetadata>`:**

| Method | Return |
| ------ | ------ |
| `read_item<T>()` | `CosmosResponse<T, ItemMetadata>` |
| `create_item()` | `CosmosResponse<(), ItemMetadata>` |
| `replace_item()` | `CosmosResponse<(), ItemMetadata>` |
| `upsert_item()` | `CosmosResponse<(), ItemMetadata>` |
| `delete_item()` | `CosmosResponse<(), ItemMetadata>` |

**Batch operations → `CosmosResponse<T, BatchMetadata>`:**

| Method | Return |
| ------ | ------ |
| `execute_transactional_batch()` | `CosmosResponse<TransactionalBatchResponse, BatchMetadata>` |

**Resource operations → `CosmosResponse<T, ResourceMetadata>`:**

| Method | Return |
| ------ | ------ |
| `create_database()` | `CosmosResponse<DatabaseProperties, ResourceMetadata>` |
| `read()` (container) | `CosmosResponse<ContainerProperties, ResourceMetadata>` |
| `replace()` (container) | `CosmosResponse<ContainerProperties, ResourceMetadata>` |
| `delete()` | `CosmosResponse<(), ResourceMetadata>` |
| `read()` (database) | `CosmosResponse<DatabaseProperties, ResourceMetadata>` |
| `replace_throughput()` | `CosmosResponse<ThroughputProperties, ResourceMetadata>` |

**Query operations → `FeedPage<T, QueryMetadata>`:**

| Method | Return |
| ------ | ------ |
| `query_items<T>()` | `FeedItemIterator<T>` yielding `FeedPage<T, QueryMetadata>` |
| `query_containers()` | `FeedItemIterator<ContainerProperties>` |
| `query_databases()` | `FeedItemIterator<DatabaseProperties>` |

**Future operations:**

| Method | Return |
| ------ | ------ |
| `change_feed<T>()` | `FeedPage<T, ChangeFeedMetadata>` |
| `read_many<T>()` | `FeedPage<T, ReadManyMetadata>` |

### Implementation

#### `CosmosResponse<T, M = ()>`

`CosmosResponse` uses a second generic parameter with a default of `()` for operations
that don't need extra metadata:

```rust
pub struct CosmosResponse<T, M = ()> {
    response: Response<T>,
    request: CosmosRequest,
    cosmos_headers: CosmosResponseHeaders,
    metadata: M,
    diagnostics: CosmosDiagnostics,
}
```

Universal accessors are on `impl<T, M> CosmosResponse<T, M>`:

```rust
impl<T, M> CosmosResponse<T, M> {
    pub fn status(&self) -> StatusCode;
    pub fn headers(&self) -> &Headers;
    pub fn request_charge(&self) -> Option<f64>;
    pub fn session_token(&self) -> Option<SessionToken>;
    pub fn diagnostics(&self) -> &CosmosDiagnostics;

    /// Access the operation-specific metadata.
    pub fn metadata(&self) -> &M;
}
```

`activity_id()` and `server_duration_ms()` are accessed via `diagnostics()`:

```rust
response.diagnostics().activity_id()
response.diagnostics().server_duration_ms()
```

Operation-specific accessors are on constrained impls:

```rust
impl<T> CosmosResponse<T, ItemMetadata> {
    pub fn etag(&self) -> Option<&str> { self.metadata.etag() }
}

impl<T> CosmosResponse<T, BatchMetadata> {
    pub fn etag(&self) -> Option<&str> { self.metadata.etag() }
}
```

`index_metrics()` and `query_metrics()` move off `CosmosResponse` → into `QueryMetadata`.
`etag()` moves off `CosmosResponse` → into `ItemMetadata`.

#### `FeedPage<T, M>`

`FeedPage` stores `CosmosResponseHeaders` directly (no `CosmosResponse<()>` wrapper):

```rust
pub struct FeedPage<T, M> {
    items: Vec<T>,
    continuation: Option<String>,
    cosmos_headers: CosmosResponseHeaders,
    metadata: M,
}

impl<T, M> FeedPage<T, M> {
    // Universal accessors (delegate to cosmos_headers)
    pub fn request_charge(&self) -> Option<f64>;
    pub fn session_token(&self) -> Option<SessionToken>;
    pub fn activity_id(&self) -> Option<&str>;
    pub fn server_duration_ms(&self) -> Option<f64>;

    // Page-specific
    pub fn items(&self) -> &[T];
    pub fn into_items(self) -> Vec<T>;
    pub fn continuation(&self) -> Option<&str>;

    // Metadata access
    pub fn metadata(&self) -> &M;
}

impl<T> FeedPage<T, QueryMetadata> {
    pub fn index_metrics(&self) -> Option<&str> { self.metadata.index_metrics.as_deref() }
    pub fn query_metrics(&self) -> Option<&str> { self.metadata.query_metrics.as_deref() }
}
```

`FeedItemIterator` and `FeedPageIterator` do not carry the `M` param. They always
yield `FeedPage<T, QueryMetadata>` since all current feed operations are queries:

```rust
pub struct FeedItemIterator<T: Send> { ... }
pub struct FeedPageIterator<T: Send> { ... }
```

### Usage

**Point operation:**

```rust
let response = container.read_item::<MyItem>("id", pk, None).await?;
let charge = response.request_charge();
let etag = response.etag();  // Only available on ItemResponse / CosmosResponse<T, ItemMetadata>
```

**Query:**

```rust
let mut pages = container.query_items::<MyItem>(query, pk, Some(options))?.into_pages();
while let Some(page) = pages.next().await {
    let page = page?;
    let charge = page.request_charge();
    let metrics = page.index_metrics();  // Only available on FeedPage<T, QueryMetadata>
    let stats = page.query_metrics();
}
```

**Compile-time safety:**

```rust
// This doesn't compile — no index_metrics on point operations:
let response = container.read_item::<MyItem>("id", pk, None).await?;
response.index_metrics(); // ERROR: method not found

// This doesn't compile — no etag on query pages:
let page: FeedPage<MyItem, QueryMetadata> = ...;
page.etag(); // ERROR: method not found
```

### Decisions made

1. **Second generic param** (`CosmosResponse<T, M = ()>`) preferred over wrapper types.
2. **`FeedPage` stores `CosmosResponseHeaders` directly** — no `CosmosResponse<()>` wrapper.
3. **`FeedItemIterator` does NOT carry the metadata param** — only `FeedPage<T, M>` has it.
4. **Resource operations use `ResourceMetadata`** — prevents breaking changes if fields are
   added later. Changing from `CosmosResponse<T>` to `CosmosResponse<T, ResourceMetadata>`
   later would be breaking for callers who wrote out the type explicitly.

### Open questions

None — all design decisions resolved.
