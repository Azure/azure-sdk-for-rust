# Hierarchical Configuration Model

## Status

**Proposal** — under review, not yet implemented.

## Summary

This document proposes a hierarchical configuration model for the Azure Cosmos DB Rust SDK.
The model introduces layered option groups, a proc macro for code generation, systematic
environment variable support, and lazy read-time resolution.

## Motivation

The current options system has several issues that become more painful as the number of
configurable settings grows:

1. **Field duplication** — `consistency_level`, `priority`, `throughput_bucket`, and
   `custom_headers` are repeated across `CosmosClientOptions`, `ItemOptions`,
   `QueryOptions`, and other option types. Each copy requires its own `with_*` builder
   method and header-serialization logic.

2. **No formal layering** — Merging today happens implicitly through HTTP header priority
   (`CosmosRequest::client_headers()` skips headers already set by request-level options).
   There is no explicit resolution model, no way to add application-global defaults, and no
   way to reason about which layer a value came from.

3. **No Runtime (global) layer** — The lowest layer is per-Account
   (`CosmosClientOptions`). Applications that create multiple `CosmosClient` instances have
   no way to set shared defaults. As more customers look for multi-tenant solutions that
   require a single application to connect to various Cosmos Accounts, we need a way to manage
   options across all Accounts used by an application.

4. **Ad-hoc environment variable support** — A handful of env vars are read in
   `global_partition_endpoint_manager.rs`, but there is no systematic pattern, naming
   convention, or discovery mechanism.

5. **Boilerplate** — Adding a single cross-layer option today requires changes in N struct
   definitions, N `with_*` methods, and N serialization impls. This slows development and
   invites inconsistency.

## Design

### Layers

The configuration model defines four layers, listed from lowest to highest priority:

| Layer | Scope | Lifetime | Example |
|-------|-------|----------|---------|
| **Environment** | Process-wide | Static | `AZURE_COSMOS_CONSISTENCY_LEVEL=Session` |
| **Runtime** | Application-global | App lifetime | Shared defaults across all clients |
| **Account** | Per `CosmosClient` | Client lifetime | Options passed at client construction |
| **Operation** | Per request | Single call | Options passed to `create_item()`, etc. |

When a value is needed, the system walks from the highest-priority layer that applies down
to the lowest, returning the first `Some` value found, or `None` if the option isn't set in any layer.

**Resolution order** (highest priority first): **Operation → Account → Runtime → Environment**

### Option Groups

An **option group** is a plain Rust struct whose fields are all `Option<T>`. The same
struct type is reused at every explicit layer (runtime, account, operation) it participates in.
All fields within a group must share the same set of these explicit layers.

If a setting is only relevant at certain layers, it belongs in a group scoped to those
layers. This constraint keeps the model simple: there are no per-field annotations for the
explicit layers (runtime, account, operation) and no per-layer struct variants. It does
mean we need to carefully design option groups to ensure they are all aligned to the same
explicit layering.

Environment variable support is orthogonal to these explicit layers. Marking a field with
`#[option(env = "...")]` opts that field into an implicit **Environment** layer for its group.
The `#[options(layers(...))]` annotation only controls which layers users can explicitly set
values at (`runtime`, `account`, `operation`). Any group that has at least one
`#[option(env)]` field will:

- Participate in the implicit environment layer, and
- Have an `env` field on its generated `View` struct,

regardless of which explicit layers are listed in `layers()`. For example, a
`ConnectionOptions` group with `#[options(layers(runtime, account))]` and at least one
`#[option(env)]` field will still have its values resolved from environment variables and its
`View` will include an `env` field.

#### Example: `RequestOptions`

(This example is intended to illustrate functionality, not to present a concrete proposal for a 'RequestOptions' type)

```rust
/// Options controlling per-request behavior. Applicable at runtime, account,
/// and operation layers.
#[derive(CosmosOptions)]
#[options(layers(runtime, account, operation))]
pub struct RequestOptions {
    /// Consistency level for the operation.
    #[option(env = "AZURE_COSMOS_CONSISTENCY_LEVEL")]
    pub consistency_level: Option<ConsistencyLevel>,

    /// Priority-based execution level.
    #[option(env = "AZURE_COSMOS_PRIORITY")]
    pub priority: Option<PriorityLevel>,

    /// Throughput bucket for the request.
    pub throughput_bucket: Option<usize>,

    /// Additional custom headers. Merged additively across layers.
    #[option(merge = "extend")]
    pub custom_headers: Option<HashMap<HeaderName, HeaderValue>>,

    /// Regions to exclude from routing.
    pub excluded_regions: Option<Vec<RegionName>>,
}
```

Because `RequestOptions` is declared with `layers(runtime, account, operation)`, the same
struct appears in `CosmosRuntimeOptions`, `CosmosClientOptions`, and in operation types
like `ItemOptions`. Adding a new field to `RequestOptions` automatically makes it available
at every applicable layer.

### Sample Option Groups

#### `ConnectionOptions` — layers: runtime, account

```rust
#[derive(CosmosOptions)]
#[options(layers(runtime, account))]
pub struct ConnectionOptions {
    #[option(env = "AZURE_COSMOS_REQUEST_TIMEOUT")]
    pub request_timeout: Option<Duration>,

    #[option(nested)]
    pub connection_pool: Option<ConnectionPoolOptions>,
}

#[derive(CosmosOptions)]
#[options(layers(runtime, account))]
pub struct ConnectionPoolOptions {
    #[option(env = "AZURE_COSMOS_POOL_IDLE_TIMEOUT")]
    pub idle_timeout: Option<Duration>,

    #[option(env = "AZURE_COSMOS_POOL_MAX_CONNECTIONS")]
    pub max_connections: Option<usize>,
}
```

#### `RegionOptions` — layers: runtime, account

```rust
#[derive(CosmosOptions)]
#[options(layers(runtime, account))]
pub struct RegionOptions {
    pub application_region: Option<RegionName>,
    pub preferred_regions: Option<Vec<RegionName>>,
}
```

`excluded_regions` lives in `RequestOptions` because it spans all three layers including
operation.

#### `RetryOptions` — layers: runtime, account

```rust
#[derive(CosmosOptions)]
#[options(layers(runtime, account))]
pub struct RetryOptions {
    #[option(nested)]
    pub session_retry: Option<SessionRetryOptions>,

    #[option(env = "AZURE_COSMOS_ENABLE_PARTITION_CIRCUIT_BREAKER")]
    pub enable_partition_level_circuit_breaker: Option<bool>,

    pub disable_partition_level_failover: Option<bool>,
    pub enable_remote_region_preferred_for_session_retry: Option<bool>,
}

#[derive(CosmosOptions)]
#[options(layers(runtime, account))]
pub struct SessionRetryOptions {
    pub min_in_region_retry_time: Option<Duration>,
    pub max_in_region_retry_count: Option<usize>,
    pub remote_region_preferred: Option<bool>,
}
```

#### `RequestOptions` — layers: runtime, account, operation

```rust
#[derive(CosmosOptions)]
#[options(layers(runtime, account, operation))]
pub struct RequestOptions {
    #[option(env = "AZURE_COSMOS_CONSISTENCY_LEVEL")]
    pub consistency_level: Option<ConsistencyLevel>,

    #[option(env = "AZURE_COSMOS_PRIORITY")]
    pub priority: Option<PriorityLevel>,

    pub throughput_bucket: Option<usize>,

    #[option(merge = "extend")]
    pub custom_headers: Option<HashMap<HeaderName, HeaderValue>>,

    pub excluded_regions: Option<Vec<RegionName>>,
}
```

#### `CosmosAccountOptions` — layers: runtime, account

```rust
#[derive(CosmosOptions)]
#[options(layers(runtime, account))]
pub struct CosmosAccountOptions {
    pub application_name: Option<String>,
    pub custom_endpoints: Option<HashSet<String>>,
    pub enable_upgrade_consistency_to_local_quorum: Option<bool>,
}
```

### Layer Structs

Layer structs are hand-written composites that aggregate the relevant option groups for a
given layer. Each option group within a layer is stored behind its own `Arc`, so that
individual groups can be replaced atomically without affecting sibling groups or in-flight
operations that hold a snapshot of the previous value.

```rust
/// Runtime-level options (application-global defaults).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CosmosRuntimeOptions {
    pub connection: Arc<ConnectionOptions>,
    pub regions: Arc<RegionOptions>,
    pub retry: Arc<RetryOptions>,
    pub request: Arc<RequestOptions>,
    pub account: Arc<CosmosAccountOptions>,
}

/// Account-level options (per CosmosClient instance).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CosmosClientOptions {
    pub client_options: ClientOptions,
    pub connection: Arc<ConnectionOptions>,
    pub regions: Arc<RegionOptions>,
    pub retry: Arc<RetryOptions>,
    pub request: Arc<RequestOptions>,
    pub account: Arc<CosmosAccountOptions>,
}
```

Storing `Arc`s at the option-group level rather than the layer level means a user can
replace a single group (e.g., `RequestOptions`) without touching unrelated groups (e.g.,
`ConnectionOptions`). Cloning a layer struct is cheap — it increments one `Arc` refcount
per group.

See [Snapshot-Based Resolution](#snapshot-based-resolution) for how these `Arc`s are used.

### Operation-Level Types

Operation types compose the operation-layer option groups relevant to that operation.
Options that are **only meaningful at the operation level** (e.g., ETags, session tokens,
triggers) live directly on the operation struct as plain fields rather than in a separate
option group. This keeps the most commonly-adjusted per-request settings immediately
visible and avoids the indirection of a sub-group for options that will never participate
in cross-layer resolution.

For operation-only fields, **duplication across operation structs is preferred** over
grouping them. Option groups exist to solve the cross-layer resolution problem; for
single-layer options the extra abstraction adds noise without value.

#### Data-plane operations

Item operations are split into `ItemReadOptions` and `ItemWriteOptions` because read and
write operations have different per-request concerns:

```rust
/// Options for item read operations (read_item, read_all_items).
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ItemReadOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,

    // Layered option group — participates in cross-layer resolution
    pub request: RequestOptions,

    // Operation-only fields — no cross-layer resolution needed
    pub session_token: Option<SessionToken>,
}

/// Options for item write operations (create, replace, upsert, delete, patch).
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ItemWriteOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,

    // Layered option group
    pub request: RequestOptions,

    // Operation-only fields
    pub session_token: Option<SessionToken>,
    pub if_match_etag: Option<Etag>,
    pub content_response_on_write: Option<bool>,
    pub indexing_directive: Option<IndexingDirective>,
    pub pre_triggers: Option<Vec<String>>,
    pub post_triggers: Option<Vec<String>>,
}

/// Options for query operations.
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct QueryOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,

    // Layered option group
    pub request: RequestOptions,

    // Operation-only fields
    pub session_token: Option<SessionToken>,
}
```

Note that `session_token` appears on all three structs. This is intentional — duplication
at the operation level is preferable to a shared group, because these fields are what
customers interact with most directly and grouping them would add unnecessary nesting.

#### Metadata/management operations

These do not currently need the layered option groups and remain simple:

```rust
#[derive(Clone, Default)]
#[non_exhaustive]
pub struct CreateContainerOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
    pub throughput: Option<ThroughputProperties>,
}

#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ReplaceContainerOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Clone, Default)]
#[non_exhaustive]
pub struct DeleteContainerOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ReadContainerOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Clone, Default)]
#[non_exhaustive]
pub struct CreateDatabaseOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
    pub throughput: Option<ThroughputProperties>,
}

#[derive(Clone, Default)]
#[non_exhaustive]
pub struct DeleteDatabaseOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ReadDatabaseOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Clone, Default)]
#[non_exhaustive]
pub struct QueryContainersOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Clone, Default)]
#[non_exhaustive]
pub struct QueryDatabasesOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

#[derive(Clone, Default)]
#[non_exhaustive]
pub struct ThroughputOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}
```

Because all operation types are `#[non_exhaustive]`, option groups can be added to metadata
operations in the future without a breaking change.

### Snapshot-Based Resolution

Each option group at the environment, runtime, and account layers is stored behind its own
`Arc`. When an operation begins, it clones the relevant `Arc`s — one per option group it
needs — creating a cheap snapshot of the configuration in effect at that moment. If the
user later replaces an option group (e.g., swaps the `Arc<RequestOptions>` on the account
layer), in-flight operations continue using the old snapshot. Once all in-flight operations
complete, the old `Arc` is dropped automatically.

This approach:

- **Avoids locks on the hot path** — `Arc::clone()` is an atomic increment, not a mutex.
- **Guarantees consistency within an operation** — a single request always sees a coherent
  set of options per group, even if the user mutates the client concurrently.
- **Enables fine-grained replacement** — replacing one option group (e.g., `RequestOptions`)
  does not disturb snapshots of other groups (e.g., `ConnectionOptions`).
- **Makes replacement explicit** — users create a new options struct and replace the `Arc`;
  there is no interior mutability on the options themselves.

#### Resolution View

The proc macro generates a **View** struct for each option group. The View holds an `Arc`
to the group instance at each applicable layer and provides accessor methods that walk from
highest to lowest priority.

```rust
/// Snapshot view across all layers for RequestOptions.
pub struct RequestOptionsView {
    env: Arc<RequestOptions>,
    runtime: Arc<RequestOptions>,
    account: Arc<RequestOptions>,
    operation: RequestOptions,  // Owned, not Arc — operation layer is per-request
}

impl RequestOptionsView {
    pub fn new(
        env: Arc<RequestOptions>,
        runtime: Arc<RequestOptions>,
        account: Arc<RequestOptions>,
        operation: RequestOptions,
    ) -> Self {
        Self { env, runtime, account, operation }
    }

    /// Walks Operation → Account → Runtime → Env.
    pub fn consistency_level(&self) -> Option<&ConsistencyLevel> {
        self.operation.consistency_level.as_ref()
            .or(self.account.consistency_level.as_ref())
            .or(self.runtime.consistency_level.as_ref())
            .or(self.env.consistency_level.as_ref())
    }

    pub fn priority(&self) -> Option<&PriorityLevel> {
        self.operation.priority.as_ref()
            .or(self.account.priority.as_ref())
            .or(self.runtime.priority.as_ref())
            .or(self.env.priority.as_ref())
    }

    pub fn throughput_bucket(&self) -> Option<usize> {
        self.operation.throughput_bucket
            .or(self.account.throughput_bucket)
            .or(self.runtime.throughput_bucket)
            .or(self.env.throughput_bucket)
    }

    /// Per-key merged headers: env → runtime → account → operation.
    pub fn custom_headers(&self) -> HashMap<HeaderName, HeaderValue> {
        let mut merged = HashMap::new();
        if let Some(ref h) = self.env.custom_headers { merged.extend(h.clone()); }
        if let Some(ref h) = self.runtime.custom_headers { merged.extend(h.clone()); }
        if let Some(ref h) = self.account.custom_headers { merged.extend(h.clone()); }
        if let Some(ref h) = self.operation.custom_headers { merged.extend(h.clone()); }
        merged
    }

    pub fn excluded_regions(&self) -> Option<&Vec<RegionName>> {
        self.operation.excluded_regions.as_ref()
            .or(self.account.excluded_regions.as_ref())
            .or(self.runtime.excluded_regions.as_ref())
            .or(self.env.excluded_regions.as_ref())
    }
}
```

The operation layer is **owned** (not `Arc`), because it is constructed per-request and
does not need to be shared. The env, runtime, and account layers are `Arc` clones taken
at the start of the operation.

#### Accessor rules

The macro generates two kinds of accessors:

- **Shadow (default)**: Returns `Option<&T>` for non-`Copy` types, `Option<T>` for `Copy`
  types. The highest-priority layer with a `Some` value wins.
- **Merge (`#[option(merge = "extend")]`)**: Returns an owned merged collection. Layers are
  merged from lowest to highest priority, so higher layers overwrite per-key.

#### Constructing the snapshot

When an operation starts, the pipeline clones the `Arc`s for the groups it needs:

```rust
// At the start of an operation (e.g., in container_client.create_item())
let request_view = RequestOptionsView::new(
    self.env.request.clone(),       // env layer's RequestOptions
    self.runtime.request.clone(),   // runtime layer's RequestOptions
    self.account.request.clone(),   // account layer's RequestOptions
    operation_options.request,      // Moved from the caller's operation options
);

// The View is self-contained — it owns or shares all the data it needs.
// No lifetime parameters, no borrowing issues across await points.
if let Some(consistency) = request_view.consistency_level() {
    // Use resolved value
}
```

#### Replacing an option group at runtime

When a user wants to change a single option group (e.g., to add an excluded region to
account-level request options), they assign a new `Arc` to the relevant field on the
client's options. Since the layer structs expose their `Arc` fields as `pub`, replacement
is a direct field assignment:

```rust
// User creates a new RequestOptions via the builder
let new_request_opts = RequestOptionsBuilder::new()
    .excluded_regions(vec!["West US".into()])
    .build();

// Direct field assignment replaces just the Arc<RequestOptions>
client.options.request = Arc::new(new_request_opts);

// In-flight operations that cloned the old Arc<RequestOptions> are unaffected.
// Other groups (ConnectionOptions, RetryOptions, etc.) are untouched.
// New operations will pick up the new RequestOptions.
```

### Nested Option Groups

The `#[option(nested)]` attribute signals that a field's type is itself a `CosmosOptions`
group. The parent View delegates resolution into a child View:

```rust
impl ConnectionOptionsView {
    pub fn request_timeout(&self) -> Option<&Duration> {
        self.account.request_timeout.as_ref()
            .or(self.runtime.request_timeout.as_ref())
            .or(self.env.request_timeout.as_ref())
    }

    pub fn connection_pool(&self) -> ConnectionPoolOptionsView {
        // For nested groups, extract the inner group from each layer's Arc,
        // wrapping in a new Arc. Layers that don't set the nested group
        // fall back to Default (all-None) so inner resolution continues.
        let default = Arc::new(ConnectionPoolOptions::default());
        ConnectionPoolOptionsView::new(
            self.env.connection_pool.as_ref()
                .map(|cp| Arc::new(cp.clone()))
                .unwrap_or_else(|| Arc::clone(&default)),
            self.runtime.connection_pool.as_ref()
                .map(|cp| Arc::new(cp.clone()))
                .unwrap_or_else(|| Arc::clone(&default)),
            self.account.connection_pool.as_ref()
                .map(|cp| Arc::new(cp.clone()))
                .unwrap_or_else(|| Arc::clone(&default)),
        )
    }
}
```

When a layer has `None` for a nested struct, resolution substitutes `Default::default()`
(all-`None`) so that inner field resolution can continue through to lower layers.

### Environment Variables

Fields annotated with `#[option(env = "AZURE_COSMOS_...")]` participate in environment
variable loading. The macro generates a `from_env()` constructor on the group struct:

```rust
impl RequestOptions {
    pub fn from_env() -> Self {
        Self {
            consistency_level: std::env::var("AZURE_COSMOS_CONSISTENCY_LEVEL")
                .ok()
                .and_then(|v| v.parse().ok()),
            priority: std::env::var("AZURE_COSMOS_PRIORITY")
                .ok()
                .and_then(|v| v.parse().ok()),
            throughput_bucket: None,
            custom_headers: None,
            excluded_regions: None,
        }
    }
}
```

Types used with `#[option(env)]` must implement `FromStr`. Parsing rules:

- Primitives (`usize`, `u32`, `bool`) — standard `.parse()`
- `String` — direct use
- `azure_core::time::Duration` — parsed using its `FromStr` implementation (the required
  string format, such as an ISO 8601 duration, is determined by that type; the macro
  does not add custom "seconds-only" parsing)
- Enums (`ConsistencyLevel`, `PriorityLevel`) — require `FromStr` impls
- `Vec<T>` — comma-separated (e.g., `"West US,East US"`)

All `AZURE_COSMOS_*` env vars follow a consistent naming scheme. The environment layer is
read once during construction and cached.

## Proc Macro Crate

A new proc-macro crate at `sdk/cosmos/azure_data_cosmos_macros/` provides
`#[derive(CosmosOptions)]`. It reads:

- **Struct-level** `#[options(layers(...))]` — determines which layers the group
  participates in, and thus the shape of the generated View.
- **Field-level** `#[option(env = "...")]` — env var name for `from_env()` generation.
- **Field-level** `#[option(merge = "extend")]` — additive merge instead of shadow.
- **Field-level** `#[option(nested)]` — delegates resolution to a child View.

The macro also generates a `RequestOptionsBuilder` type for each option group, and a
`Default` implementation for groups whose fields are all `Option<T>` (all fields default to
`None`), enabling patterns like `..Default::default()`.

Builder types provide a fluent API for constructing option groups:

```rust
let request = RequestOptionsBuilder::new()
    .consistency_level(ConsistencyLevel::Session)
    .priority(PriorityLevel::High)
    .build();
```

### Crate structure

```text
sdk/cosmos/azure_data_cosmos_macros/
├── Cargo.toml
└── src/
    ├── lib.rs         # #[derive(CosmosOptions)] entry point
    ├── parse.rs       # Attribute parsing
    ├── view.rs        # View struct + accessor generation
    ├── env.rs         # from_env() generation
    └── builder.rs     # XxxOptionsBuilder generation
```

## Module Structure

```text
sdk/cosmos/azure_data_cosmos/src/options/
├── mod.rs                   # Re-exports, standalone types (enums, SessionToken)
├── connection_options.rs    # ConnectionOptions, ConnectionPoolOptions
├── region_options.rs        # RegionOptions
├── retry_options.rs         # RetryOptions, SessionRetryOptions
├── request_options.rs       # RequestOptions
├── account_options.rs       # CosmosAccountOptions
├── layers.rs                # CosmosRuntimeOptions, CosmosClientOptions
└── operations.rs            # ItemReadOptions, ItemWriteOptions, QueryOptions, metadata types
```

## Migration

The new system replaces all existing option structs in a single migration:

| Current Type | New Type | Change |
|---|---|---|
| `CosmosClientOptions` | `CosmosClientOptions` | Fields redistributed across option groups |
| `ItemOptions` | `ItemReadOptions` / `ItemWriteOptions` | Split by read vs. write; layered fields in `RequestOptions` group, operation-only fields inline |
| `QueryOptions` | `QueryOptions` | Layered fields in `RequestOptions`, operation-only fields inline |
| `CreateContainerOptions` | `CreateContainerOptions` | Unchanged |
| `ReplaceContainerOptions` | `ReplaceContainerOptions` | Unchanged |
| `DeleteContainerOptions` | `DeleteContainerOptions` | Unchanged |
| `ReadContainerOptions` | `ReadContainerOptions` | Unchanged |
| `CreateDatabaseOptions` | `CreateDatabaseOptions` | Unchanged |
| `DeleteDatabaseOptions` | `DeleteDatabaseOptions` | Unchanged |
| `ReadDatabaseOptions` | `ReadDatabaseOptions` | Unchanged |
| `QueryContainersOptions` | `QueryContainersOptions` | Unchanged |
| `QueryDatabasesOptions` | `QueryDatabasesOptions` | Unchanged |
| `ThroughputOptions` | `ThroughputOptions` | Unchanged |
| `SessionRetryOptions` | `SessionRetryOptions` | Nested in `RetryOptions` |
| `ConsistencyLevel` | `ConsistencyLevel` | Gains `FromStr` impl |
| `PriorityLevel` | `PriorityLevel` | Gains `FromStr` impl |
| `SessionToken` | `SessionToken` | Unchanged |
| `IndexingDirective` | `IndexingDirective` | Unchanged |

## Example Usage

```rust
use azure_data_cosmos::options::*;
use std::sync::Arc;

// Runtime layer — application-global defaults (each group wrapped in Arc)
let runtime = CosmosRuntimeOptions {
    request: Arc::new(
        RequestOptionsBuilder::new()
            .consistency_level(ConsistencyLevel::Session)
            .priority(PriorityLevel::High)
            .build()
    ),
    ..Default::default()
};

// Account layer — same RequestOptions struct, different instance
let client_options = CosmosClientOptions {
    request: Arc::new(
        RequestOptionsBuilder::new()
            .throughput_bucket(5)
            .build()
    ),
    regions: Arc::new(RegionOptions {
        preferred_regions: Some(vec!["West US".into(), "East US".into()]),
        ..Default::default()
    }),
    ..Default::default()
};

let client = CosmosClient::new(endpoint, credential, Some(client_options))?;

// Operation layer — plain structs (no Arc), operation-only fields inline
let write_opts = ItemWriteOptions {
    request: RequestOptionsBuilder::new()
        .priority(PriorityLevel::Low) // overrides runtime High
        .build(),
    if_match_etag: Some(etag),
    content_response_on_write: Some(true),
    ..Default::default()
};

// Resolved: priority=Low (operation), consistency=Session (runtime), throughput=5 (account)
client.database("db").container("coll")
    .create_item("pk", item, Some(write_opts)).await?;

// Replace just the account-level RequestOptions — other groups untouched
client.options.request = Arc::new(
    RequestOptionsBuilder::new()
        .excluded_regions(vec!["West US".into()])
        .build()
);

// Item read with minimal options
let read_opts = ItemReadOptions {
    session_token: Some(session_token),
    ..Default::default()
};

client.database("db").container("coll")
    .read_item("pk", "item-id", Some(read_opts)).await?;

// Metadata operations — simple, no layered groups
let create_opts = CreateContainerOptions {
    throughput: Some(ThroughputProperties::manual(400)),
    ..Default::default()
};
client.database("db").create_container(props, Some(create_opts)).await?;
```

## Design Decisions

The following items were considered during design and are now resolved:

1. **Default values** — Resolution returns `Option<T>`. Callers apply their own defaults
   when `None` is returned. There is no macro-level `#[option(default = ...)]` attribute.

2. **`Vec` and `HashMap` merge semantics** — The `#[option(merge = "extend")]` attribute
   works uniformly for both `HashMap` and `Vec` fields: lower layers contribute first,
   higher layers extend (for `Vec`, appending; for `HashMap`, overwriting per-key). Fields
   without `merge = "extend"` use shadow semantics (higher layer replaces lower entirely).

3. **Environment variable immutability** — Environment variables are assumed immutable after
   process startup. The environment-level config structs are captured once via `from_env()`
   during SDK initialization and never re-read.

4. **Builder APIs for option groups** — The proc macro generates a builder type for each
   option group (e.g., `RequestOptionsBuilder`). Builders provide a fluent API and produce
   the `Option<T>`-field struct on `.build()`.

5. **Option group replacement via `pub` fields** — Layer structs expose their `Arc<T>`
   fields as `pub`. Replacing an option group is a direct field assignment
   (`client.options.request = Arc::new(...)`) which drops the old `Arc`. In-flight
   operations that cloned the old `Arc` are unaffected. No special setter API or `ArcSwap`
   is needed.

## Open Questions

1. **Runtime layer delivery mechanism** — The structs are defined, but how the runtime
   layer is provided to the SDK (constructor argument, global registry, Driver-level
   config) is deferred pending the Driver refactoring.

2. **Environment variable priority** — This design places env vars at the bottom (lowest
   priority). If env vars should override code-specified values for operational flexibility,
   they would move to the top.

3. **Thread-safe option group replacement** — Replacing an `Arc<T>` field on a layer struct
   requires `&mut` access, meaning the caller needs an exclusive handle to the component
   that owns the layer options. This is fine for single-threaded or task-local usage, but
   concurrent replacement from multiple threads would require the user to wrap the client
   (or the layer struct) in an `RwLock` or similar. Should the SDK provide a built-in
   thread-safe mutation mechanism (e.g., using `ArcSwap` or an internal `RwLock` around
   each `Arc` field), or leave this as a user responsibility? The current design favors
   simplicity — users who need concurrent replacement can add their own synchronization.
