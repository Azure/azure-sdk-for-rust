# Azure Cosmos DB Rust SDK — Configuration Options Specification

This document specifies the configuration option types for the Rust SDK (`azure_data_cosmos`), aligned with the [Hierarchical Configuration Model](HierarchicalConfigModel.md).

## Table of Contents

1. [Layering Overview](#1-layering-overview)
2. [Standalone Types](#2-standalone-types)
3. [Option Groups](#3-option-groups)
   - [OperationOptions](#31-requestoptions)
   - [ConnectionOptions](#32-connectionoptions)
   - [ConnectionPoolOptions](#33-connectionpooloptions)
   - [RegionOptions](#34-regionoptions)
   - [RetryOptions](#35-retryoptions)
   - [SessionRetryOptions](#36-sessionretryoptions)
   - [CosmosAccountOptions](#37-cosmosaccountoptions)
4. [Layer Structs](#4-layer-structs)
   - [CosmosRuntimeOptions](#41-cosmosruntimeoptions)
   - [CosmosClientOptions](#42-cosmosclientoptions)
5. [Operation-Level Types](#5-operation-level-types)
   - [ItemReadOptions](#51-itemreadoptions)
   - [ItemWriteOptions](#52-itemwriteoptions)
   - [QueryOptions](#53-queryoptions)
   - [TransactionalBatchOptions](#54-transactionalbatchoptions)
   - [TransactionalBatchItemOptions](#55-transactionalbatchitemoptions)
   - [Metadata / Management Operations](#56-metadata--management-operations)
6. [Removed Options](#6-removed-options)
7. [Migration from Current Types](#7-migration-from-current-types)

---

## 1. Layering Overview

Configuration values resolve from highest to lowest priority:

| Layer | Scope | Lifetime | Priority |
|---|---|---|---|
| **Operation** | Per request | Single call | Highest |
| **Account** | Per `CosmosClient` | Client lifetime | ↑ |
| **Runtime** | Application-global | App lifetime | ↑ |
| **Environment** | Process-wide | Static (read once) | Lowest |

An **option group** is a `#[non_exhaustive]` struct whose fields are all `Option<T>`. The same struct type is reused at every explicit layer (runtime, account, operation) it participates in. Resolution walks from the highest-priority layer downward, returning the first `Some` value.

Option groups follow the Cosmos SDK Struct Design Rules:
- All fields `pub`, all `Option<T>` → `#[non_exhaustive]`, `Default`, fluent `with_*` setters.
- Prefer enums for closed value sets and newtypes with construction-time validation for constrained values.
- Proc-macro `#[derive(CosmosOptions)]` generates `View` structs, `from_env()`, and builders.

Options that are **only meaningful at the operation level** (e.g., ETags, session tokens) live directly on the operation struct as plain fields, duplicated across operation types rather than grouped.

### Environment Variables

Every option that participates at the **Runtime** layer is specifiable via an `AZURE_COSMOS_*` environment variable. Environment variables form an implicit lowest-priority layer: they are read once during SDK initialization via the generated `from_env()` constructor and cached for the process lifetime.

**Naming convention:** `AZURE_COSMOS_` prefix + `SCREAMING_SNAKE_CASE` of the option name (with group-specific prefixes for nested groups to avoid collisions).

**Parsing rules:**
- `bool` — `"true"` / `"false"` (case-insensitive)
- `usize`, `u32` — standard `.parse()`
- `Duration` — parsed via `FromStr` (ISO 8601 duration format)
- `String` — direct use
- `Url` — parsed via `Url::parse()`
- Enums (`ReadConsistencyStrategy`) — via `FromStr` impl (variant name, case-insensitive)
- `Vec<T>` — comma-separated (e.g. `"West US,East US"`)
- `HashSet<Url>` — comma-separated
- Nested groups — individual fields have their own env vars

---

## 2. Standalone Types

These enums and newtypes are used by multiple option groups and operation types.

### Enums

#### `ReadConsistencyStrategy`

Replaces `ConsistencyLevel` for per-request use. Represents the consistency guarantee requested for a read operation.

```rust
/// Strategy for read consistency. Set at the runtime, account, or operation layer
/// to control read-path consistency. The SDK enforces that per-request consistency
/// can only be weakened (relaxed) relative to the account default.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReadConsistencyStrategy {
    /// Eventual consistency — no ordering or freshness guarantees.
    Eventual,
    /// Globally strong reads.
    GlobalStrong,
    /// Read the latest committed write.
    LatestCommitted,
    /// Session consistency — monotonic reads/writes within a session.
    Session,
}
```

Must implement `FromStr` for environment variable parsing.

> **Note:** `ConsistencyLevel` (with variants `Strong`, `BoundedStaleness`, `Session`, `ConsistentPrefix`, `Eventual`) is retained as a model type for account-level properties returned by the service. It is **not** used in configuration options.

#### `PriorityLevel`

**Deferred.** Throughput control options (including `PriorityLevel` and `throughput_bucket`) will be addressed in a separate follow-up spec.

#### `IndexingDirective`

**Removed.** See [§6.4](#64-indexing-directive-indexing_directive).

#### `Precondition`

Unifies conditional ETag checks into a single discriminated type, replacing separate `if_match_etag` and `if_none_match_etag` fields.

```rust
/// Conditional ETag check for optimistic concurrency or cache validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Precondition {
    /// Succeeds only if the resource's current ETag matches the given value.
    /// Used for optimistic concurrency on writes (replace, delete, patch).
    IfMatch(Etag),
    /// Succeeds only if the resource's current ETag does **not** match the given value.
    /// Used for conditional reads (returns 304 Not Modified if unchanged).
    IfNoneMatch(Etag),
}
```

### Newtypes

#### `RegionName`

Retained as-is. Wraps `Cow<'static, str>`, normalizes to canonical form (lowercase, no whitespace) on construction. Implements `From<String>`, `From<&'static str>`, `Serialize`, `Deserialize`, `Display`.

#### `SessionToken`

Retained as-is. Wraps `String`. Implements `From<String>`, `Display`, `Clone`.

---

## 3. Option Groups

### 3.1 `OperationOptions`

**Layers:** Runtime, Account, Operation

Cross-layer options that apply to individual service requests. This is the most broadly-scoped group, present at all three explicit layers. The `#[derive(CosmosOptions)]` proc macro generates a `OperationOptionsView` for resolution.

```rust
#[derive(CosmosOptions)]
#[options(layers(runtime, account, operation))]
pub struct OperationOptions { /* fields below */ }
```

| Option | Type | Env Var | Notes |
|---|---|---|---|
| `read_consistency_strategy` | `Option<ReadConsistencyStrategy>` | `AZURE_COSMOS_READ_CONSISTENCY_STRATEGY` | Read consistency for the operation. Replaces the legacy `consistency_level` field. The SDK enforces weakening-only semantics relative to the account default. |
| `excluded_regions` | `Option<Vec<RegionName>>` | `AZURE_COSMOS_EXCLUDED_REGIONS` | Regions to exclude from routing. `None` inherits from a lower layer; `Some(vec![])` explicitly clears exclusions. Env var is comma-separated (e.g. `"West US,East US"`). |
| `content_response_on_write` | `Option<bool>` | `AZURE_COSMOS_CONTENT_RESPONSE_ON_WRITE` | Whether write operations return the resource body in the response. Only applicable to write operations; ignored by reads and queries. Cascades from runtime → account → operation, matching .NET/Java/Go behavior. |

### 3.2 `ConnectionOptions`

**Layers:** Runtime, Account

Options controlling network connection behavior. Not available at the operation layer because connections are managed at the client level.

```rust
#[derive(CosmosOptions)]
#[options(layers(runtime, account))]
pub struct ConnectionOptions { /* fields below */ }
```

| Option | Type | Env Var | Notes |
|---|---|---|---|
| `request_timeout` | `Option<Duration>` | `AZURE_COSMOS_REQUEST_TIMEOUT` | Per-request network timeout. |
| `connection_pool` | `Option<ConnectionPoolOptions>` | — | Nested group for connection pool tuning. Marked `#[option(nested)]`. |

### 3.3 `ConnectionPoolOptions`

**Layers:** Runtime *(nested inside `ConnectionOptions`)*

Fine-grained connection pool tuning. Nested via `#[option(nested)]` on `ConnectionOptions.connection_pool`. Applies only at the runtime layer because the application will likely share a single connection pool across all accounts.

```rust
#[derive(CosmosOptions)]
#[options(layers(runtime))]
pub struct ConnectionPoolOptions { /* fields below */ }
```

| Option | Type | Env Var | Notes |
|---|---|---|---|
| `idle_timeout` | `Option<Duration>` | `AZURE_COSMOS_POOL_IDLE_TIMEOUT` | How long idle connections are kept alive. |
| `max_connections` | `Option<usize>` | `AZURE_COSMOS_POOL_MAX_CONNECTIONS` | Maximum number of connections in the pool. |

### 3.4 `RegionOptions`

**Layers:** Runtime, Account

Options controlling region selection and routing. Not available at the operation layer — per-request region exclusion is handled by `OperationOptions.excluded_regions`.

```rust
#[derive(CosmosOptions)]
#[options(layers(runtime, account))]
pub struct RegionOptions { /* fields below */ }
```

| Option | Type | Env Var | Notes |
|---|---|---|---|
| `application_region` | `Option<RegionName>` | `AZURE_COSMOS_APPLICATION_REGION` | The region where the application is running. The SDK and backend negotiate optimal region ordering from this location. Only one of `application_region` should be set (the old `preferred_regions` / `application_preferred_regions` list is removed). |

### 3.5 `RetryOptions`

**Layers:** Runtime, Account

Options controlling retry behavior. Not available at the operation layer because retry policy is an infrastructure concern.

```rust
#[derive(CosmosOptions)]
#[options(layers(runtime, account))]
pub struct RetryOptions { /* fields below */ }
```

| Option | Type | Env Var | Notes |
|---|---|---|---|
| `session_retry` | `Option<SessionRetryOptions>` | — | Nested group for session-consistency retry behavior on 404/1002 errors. Marked `#[option(nested)]`. |

### 3.6 `SessionRetryOptions`

**Layers:** Runtime, Account *(nested inside `RetryOptions`)*

Controls retry behavior for 404/1002 (session not found) errors. Nested via `#[option(nested)]` on `RetryOptions.session_retry`.

```rust
#[derive(CosmosOptions)]
#[options(layers(runtime, account))]
pub struct SessionRetryOptions { /* fields below */ }
```

| Option | Type | Env Var | Notes |
|---|---|---|---|
| `min_in_region_retry_time` | `Option<Duration>` | `AZURE_COSMOS_SESSION_RETRY_MIN_IN_REGION_TIME` | Minimum time spent retrying within the local region before considering a cross-region retry. |
| `max_in_region_retry_count` | `Option<usize>` | `AZURE_COSMOS_SESSION_RETRY_MAX_IN_REGION_COUNT` | Maximum number of retries within the local region. |

> **Migration note:** The current `SessionRetryOptions` struct has non-`Option` fields with concrete defaults (`min_in_region_retry_time: Duration`, etc.). In the new model, all fields become `Option<T>` to support layered resolution. The concrete defaults are applied at resolution time when all layers yield `None`.

### 3.7 `CosmosAccountOptions`

**Layers:** Runtime, Account

Per-account settings that don't fit other groups. Not available at the operation layer.

```rust
#[derive(CosmosOptions)]
#[options(layers(runtime, account))]
pub struct CosmosAccountOptions { /* fields below */ }
```

| Option | Type | Env Var | Notes |
|---|---|---|---|
| `user_agent_suffix` | `Option<String>` | `AZURE_COSMOS_USER_AGENT_SUFFIX` | Application identifier appended to the User-Agent header for telemetry. |
| `account_initialization_custom_endpoints` | `Option<HashSet<Url>>` | `AZURE_COSMOS_CUSTOM_ENDPOINTS` | Custom endpoints for initial account discovery (private endpoints, etc.). Env var is comma-separated. |

---

## 4. Layer Structs

Layer structs are hand-written composites that aggregate the option groups applicable to a layer. Each group is stored behind `Arc` for cheap cloning and fine-grained atomic replacement.

### 4.1 `CosmosRuntimeOptions`

Application-global defaults shared across all `CosmosClient` instances.

```rust
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CosmosRuntimeOptions {
    pub connection: Arc<ConnectionOptions>,
    pub regions: Arc<RegionOptions>,
    pub retry: Arc<RetryOptions>,
    pub operation: Arc<OperationOptions>,
    pub account: Arc<CosmosAccountOptions>,
}
```

All fields default to `Arc::new(T::default())` (all-`None` option groups).

### 4.2 `CosmosClientOptions`

Per-`CosmosClient` instance options. Aggregates Cosmos-specific option groups.

```rust
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CosmosClientOptions {
    pub connection: Arc<ConnectionOptions>,
    pub regions: Arc<RegionOptions>,
    pub retry: Arc<RetryOptions>,
    pub operation: Arc<OperationOptions>,
    pub account: Arc<CosmosAccountOptions>,
}
```

> **Design note:** Storing `Arc`s at the option-group level (not the layer level) means replacing a single group (e.g., swapping `OperationOptions` to add an excluded region) does not disturb other groups or in-flight operations that hold a snapshot of the old value.

---

## 5. Operation-Level Types

Operation types compose a layered option group (`OperationOptions`) with operation-only fields. Fields that are only meaningful at the operation level are plain (non-`Option` or `Option`) fields directly on the struct — duplicated across operation types rather than factored into a shared group.

All operation types are `#[non_exhaustive]` with `Default` and fluent `with_*` setters.

### 5.1 `ItemReadOptions`

Options for item point-read operations (`read_item`).

```rust
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ItemReadOptions {
    // Layered option group — participates in cross-layer resolution
    pub operation: OperationOptions,

    // Operation-only fields
    pub session_token: Option<SessionToken>,
    pub precondition: Option<Precondition>,
}
```

| Option | Type | Notes |
|---|---|---|
| `operation` | `OperationOptions` | Layered group; fields resolve through Operation → Account → Runtime → Env. |
| `session_token` | `Option<SessionToken>` | Session token for session-consistent reads. Operation-only. |
| `precondition` | `Option<Precondition>` | Conditional ETag check. For reads, typically `IfNoneMatch` (returns 304 Not Modified if unchanged). Operation-only. |

### 5.2 `ItemWriteOptions`

Options for item write operations (`create_item`, `replace_item`, `upsert_item`, `delete_item`, `patch_item`).

```rust
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ItemWriteOptions {
    // Layered option group
    pub operation: OperationOptions,

    // Operation-only fields
    pub session_token: Option<SessionToken>,
    pub precondition: Option<Precondition>,
}
```

| Option | Type | Notes |
|---|---|---|
| `operation` | `OperationOptions` | Layered group; `content_response_on_write` is resolved here and applied to write responses. |
| `session_token` | `Option<SessionToken>` | Session token for session-consistent writes. Operation-only. |
| `precondition` | `Option<Precondition>` | Conditional ETag check. For writes, typically `IfMatch` (optimistic concurrency). Operation-only. |

### 5.3 `QueryOptions`

Options for query operations (`query_items`, `query_items_single_partition`).

```rust
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct QueryOptions {
    // Layered option group
    pub operation: OperationOptions,

    // Operation-only fields
    pub session_token: Option<SessionToken>,
    pub enable_scan_if_no_index: Option<bool>,
    pub populate_index_metrics: Option<bool>,
    pub populate_query_advice: Option<bool>,
}
```

| Option | Type | Notes |
|---|---|---|
| `operation` | `OperationOptions` | Layered group; `content_response_on_write` is ignored for queries. |
| `session_token` | `Option<SessionToken>` | Session token for session-consistent queries. Operation-only. |
| `enable_scan_if_no_index` | `Option<bool>` | If the query can't be served by indexes because the relevant paths are not indexed, setting this permits the query engine to perform a full container scan. Operation-only. |
| `populate_index_metrics` | `Option<bool>` | If set to `true`, the response will contain metrics regarding indexes used. Operation-only. |
| `populate_query_advice` | `Option<bool>` | If set to `true`, the response will include query optimization suggestions from the query advisor. Operation-only. |

### 5.4 `TransactionalBatchOptions`

Options for transactional batch operations. The batch as a whole carries cross-layer options via `OperationOptions`, plus batch-level operation-only fields. This follows the same pattern as `ItemWriteOptions`.

```rust
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct TransactionalBatchOptions {
    // Layered option group
    pub operation: OperationOptions,

    // Operation-only fields
    pub session_token: Option<SessionToken>,
}
```

| Option | Type | Notes |
|---|---|---|
| `operation` | `OperationOptions` | Layered group; `content_response_on_write` controls whether batch responses include resource bodies. `read_consistency_strategy` and `excluded_regions` cascade. |
| `session_token` | `Option<SessionToken>` | Session token for the batch. Operation-only. |

### 5.5 `TransactionalBatchItemOptions`

Per-operation options within a transactional batch. These are set on individual batch operations (create, replace, upsert, delete, patch) and do **not** participate in cross-layer resolution.

```rust
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct TransactionalBatchItemOptions {
    pub precondition: Option<Precondition>,
    pub filter_predicate: Option<String>,
}
```

| Option | Type | Notes |
|---|---|---|
| `precondition` | `Option<Precondition>` | Conditional ETag check on this batch item. Typically `IfMatch` for optimistic concurrency. |
| `filter_predicate` | `Option<String>` | SQL-like filter predicate for conditional patch operations within the batch. Only applicable to patch operations; ignored for other operation types. |

**Usage example:**

```rust
let batch = TransactionalBatch::new(partition_key)
    .create_item(item_a, None)
    .replace_item(
        "item-b-id",
        item_b,
        Some(TransactionalBatchItemOptions::default()
            .with_precondition(Precondition::IfMatch(etag_b))),
    )
    .patch_item(
        "item-c-id",
        patch_doc,
        Some(TransactionalBatchItemOptions::default()
            .with_filter_predicate("FROM c WHERE c.status = 'active'")),
    );

let batch_opts = TransactionalBatchOptions {
    operation: OperationOptions::default(),
    ..Default::default()
};

container.execute_transactional_batch(batch, Some(batch_opts)).await?;
```

### 5.6 Metadata / Management Operations

Metadata operations (database and container CRUD, throughput management) remain simple structs with operation-specific fields. They do **not** currently include `OperationOptions` for cross-layer resolution, but all are `#[non_exhaustive]` so option groups can be added later without breaking changes.

| Type | Fields | Notes |
|---|---|---|
| `CreateContainerOptions` | `throughput: Option<ThroughputProperties>` | Provision throughput on creation. |
| `ReplaceContainerOptions` | *(none)* | |
| `DeleteContainerOptions` | *(none)* | |
| `ReadContainerOptions` | *(none)* | |
| `CreateDatabaseOptions` | `throughput: Option<ThroughputProperties>` | Provision throughput on creation. |
| `DeleteDatabaseOptions` | *(none)* | |
| `ReadDatabaseOptions` | *(none)* | |
| `QueryContainersOptions` | *(none)* | |
| `QueryDatabasesOptions` | *(none)* | |
| `ThroughputOptions` | *(none)* | Read or replace throughput settings. |

---

## 6. Removed Options

The following options from the current SDK are **not carried forward** into the new model:

### 6.1 Preferred Regions (`application_preferred_regions`)

**Removed from:** `CosmosClientOptions`

The explicit preferred-region list is replaced by `RegionOptions.application_region`, which specifies where the application is running and lets the SDK and backend negotiate the optimal region order. Per-operation exclusion remains available via `OperationOptions.excluded_regions`.

**Rationale:** Application Region is the modern approach across SDKs. The legacy preferred-regions list is error-prone (manual ordering, stale lists) and redundant when the backend can compute optimal routing from the application's location.

### 6.2 Consistency Level (`consistency_level`)

**Removed from:** `CosmosClientOptions`, `ItemOptions`, `QueryOptions`

Replaced by `OperationOptions.read_consistency_strategy` (type `ReadConsistencyStrategy`), which aligns with the newer strategy-based approach pioneered in the Java SDK. The new enum includes traditional weakening levels (`Eventual`, `Session`) plus new strategies (`LatestCommitted`, `GlobalStrong`).

The `ConsistencyLevel` enum itself is **retained** as a model type for account-level consistency properties returned by the service. It is no longer used in any options struct.

### 6.3 Custom HTTP Headers (`custom_headers`)

**Removed from:** `CosmosClientOptions`, `ItemOptions`, `QueryOptions`

The Rust SDK does not expose a custom HTTP header mechanism. Features that other SDKs surface through custom headers (e.g., dedicated gateway cache control) will be modeled as first-class typed options when supported.

### 6.4 Indexing Directive (`indexing_directive`)

**Removed from:** `ItemOptions`

The `IndexingDirective` enum and the per-write `indexing_directive` option are not carried forward. The Rust SDK relies on the container-level `IndexingPolicy` to control indexing behavior. Per-item indexing directives are a legacy feature that future Cosmos DB SDKs will not support.

### 6.5 Pre/Post Triggers (`pre_triggers`, `post_triggers`)

**Removed from:** `ItemOptions`

Server-side triggers are a legacy Cosmos DB feature. The Rust SDK does not expose `pre_triggers` or `post_triggers` options. Applications needing pre/post-write logic should use Change Feed or application-level orchestration instead.

### 6.6 Azure Core Options (`client_options`, `method_options`)

**Removed from:** `CosmosClientOptions` (was `client_options: ClientOptions`), all operation types (was `method_options: ClientMethodOptions<'a>`)

The Cosmos SDK manages its own transport, retry, and telemetry pipeline internally. Options that the SDK chooses to expose (e.g., request timeout, user-agent suffix) are modeled as first-class Cosmos options within the appropriate option groups, and the SDK converts them to the relevant Azure Core settings internally. This avoids leaking Azure Core types into the public API surface.

---

## 7. Migration from Current Types

| Current Field | Current Location | New Location | Change |
|---|---|---|---|
| `user_agent_suffix` | `CosmosClientOptions` | `CosmosAccountOptions.user_agent_suffix` | Moved to option group |
| `application_region` | `CosmosClientOptions` | `RegionOptions.application_region` | Moved to option group |
| `application_preferred_regions` | `CosmosClientOptions` | — | **Removed** |
| `excluded_regions` | `CosmosClientOptions` | `OperationOptions.excluded_regions` | Moved; now `Option<Vec<_>>` for layered resolution |
| `account_initialization_custom_endpoints` | `CosmosClientOptions` | `CosmosAccountOptions.account_initialization_custom_endpoints` | Moved to option group |
| `consistency_level` | `CosmosClientOptions`, `ItemOptions`, `QueryOptions` | `OperationOptions.read_consistency_strategy` | **Replaced** with `ReadConsistencyStrategy` |
| `request_timeout` | `CosmosClientOptions` | `ConnectionOptions.request_timeout` | Moved to option group |
| `enable_remote_region_preferred_for_session_retry` | `CosmosClientOptions` | — | **Removed**; remote-region-preferred is now always-on behavior |
| `enable_partition_level_circuit_breaker` | `CosmosClientOptions` | — | **Removed**; partition-level circuit breaker is always enabled |
| `disable_partition_level_failover` | `CosmosClientOptions` | — | **Removed**; disabling PPAF degrades availability |
| `enable_upgrade_consistency_to_local_quorum` | `CosmosClientOptions` | — | **Removed**; use `ReadConsistencyStrategy::LatestCommitted` instead |
| `throughput_bucket` | `CosmosClientOptions`, `ItemOptions`, `QueryOptions` | — | **Deferred** to throughput control follow-up spec |
| `session_retry_options` | `CosmosClientOptions` | `RetryOptions.session_retry` | Nested; fields become `Option<T>` |
| `priority` | `CosmosClientOptions`, `ItemOptions`, `QueryOptions` | — | **Deferred** to throughput control follow-up spec |
| `custom_headers` | `CosmosClientOptions`, `ItemOptions`, `QueryOptions` | — | **Removed** (§6.3) |
| `pre_triggers` | `ItemOptions` | — | **Removed** (§6.5) |
| `post_triggers` | `ItemOptions` | — | **Removed** (§6.5) |
| `session_token` | `ItemOptions`, `QueryOptions` | Operation-only on each type | Duplicated across read/write/query/batch |
| `indexing_directive` | `ItemOptions` | — | **Removed** (§6.4) |
| `if_match_etag` | `ItemOptions` | `ItemWriteOptions.precondition` | Replaced by `Precondition::IfMatch(Etag)` |
| `content_response_on_write_enabled` | `ItemOptions` | `OperationOptions.content_response_on_write` | Moved to layered group; renamed; now `Option<bool>` |
| `excluded_regions` | `ItemOptions` | `OperationOptions.excluded_regions` | Consolidated into layered group |
| `ItemOptions` (unified) | — | `ItemReadOptions` / `ItemWriteOptions` | **Split** into separate read and write types |

