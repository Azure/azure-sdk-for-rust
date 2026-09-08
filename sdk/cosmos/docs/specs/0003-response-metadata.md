# Response Metadata Type System Design Spec

## Motivation

The SDK needs a type-safe way to expose operation-specific response metadata.
Different operations return different headers (e.g. queries return index metrics,
point operations return ETags). The type system should prevent callers from
accessing headers that don't apply to their operation.

## Design (Wrapper Types with Composition)

### Point operation wrappers

Each operation category gets its own response wrapper that composes over
the internal `CosmosResponse<T>`:

```rust
/// Point item operations (create, read, replace, upsert, delete).
pub struct ItemResponse<T> {
    response: CosmosResponse<T>,
    etag: Option<Etag>,           // azure_core::http::Etag
}

/// Resource management operations (databases, containers, throughput).
pub struct ResourceResponse<T> {
    response: CosmosResponse<T>,
}

/// Transactional batch operations.
pub struct BatchResponse {
    response: CosmosResponse<TransactionalBatchResponse>,
    etag: Option<Etag>,
}
```

### Feed page composition

`FeedPage<T>` is a generic, reusable page type for any feed operation.
`QueryFeedPage<T>` composes over it, adding query-specific fields:

```rust
/// Generic feed page — used for any feed operation (queries, read-many,
/// change feed, offers, etc.).
pub struct FeedPage<T> {
    items: Vec<T>,
    continuation: Option<String>,
    raw_headers: Headers,
    headers: CosmosResponseHeaders,
    diagnostics: CosmosDiagnostics,
}

/// Query-specific feed page — wraps FeedPage and adds query metadata.
pub struct QueryFeedPage<T> {
    page: FeedPage<T>,
    index_metrics: Option<String>,
    query_metrics: Option<String>,
}
```

### Common accessors (on all wrapper types)

- `status()` / `headers()` — HTTP response data
- `request_charge()` — RU consumption
- `session_token()` — session token for consistency
- `diagnostics()` — `CosmosDiagnostics` (activity ID, server duration)
- `into_body()` / `into_model()` — consume response body

### Operation-specific accessors

| Type | Extra methods |
|------|--------------|
| `ItemResponse<T>` | `etag() -> Option<&Etag>` |
| `ResourceResponse<T>` | (none — future-proof) |
| `BatchResponse` | `etag() -> Option<&Etag>` |
| `FeedPage<T>` | (common methods only) |
| `QueryFeedPage<T>` | `index_metrics() -> Option<&str>`, `query_metrics() -> Option<&str>` |

### Decisions made

1. **Wrapper types** preferred over generic metadata params.
2. **`CosmosResponse<T>` is `pub(crate)`** — wrapper types are the public API.
3. **`QueryFeedPage<T>` composes over `FeedPage<T>`** — query-specific fields on the wrapper.
4. **`FeedPage<T>` is public** — reusable for future read-many, change feed, etc.
5. **`FeedItemIterator<T>` and `FeedPageIterator<T>`** yield `QueryFeedPage<T>` for queries.
6. **ETags use `azure_core::http::Etag`** — not raw strings.
7. **`CosmosDiagnostics`** is universal, available on all response types via `diagnostics()`.

### Open questions

None — all design decisions resolved.
