---
applyTo: "sdk/cosmos/**/*.*"
---

# Cosmos SDK Instructions

This file contains coding guidelines and architectural patterns specific to the Azure Cosmos DB SDK for Rust (`sdk/cosmos`).

## Core Principles

### Design guidelines
- Follow the [Azure SDK Design Guidelines for Rust](https://azure.github.io/azure-sdk/rust_introduction.html) as the primary reference for API design, error handling, async patterns, and module organization unless they conflict with Cosmos-specific requirements outlined below or in other `cosmos.*.instructions.md` files.

### Data-Oriented Programming (DOP)

Follow the **Data-Oriented Programming in Rust** principles from [https://analogrelay.github.io/dop-in-rust](https://analogrelay.github.io/dop-in-rust):

- **Separate data from behavior**: Use plain structs for data and separate modules/traits for operations
- **Model data with enums and structs**: Prefer algebraic data types to represent domain concepts
- **Treat data as immutable**: Use owned types and transformations rather than mutation where practical
- **Validate at boundaries**: Parse/validate data when it enters the system, trust it internally

### Prefer Standard Traits Over Custom Methods

Always implement standard Rust traits instead of creating custom public methods when equivalent functionality exists:

- **Parsing strings**: Implement `std::str::FromStr` instead of `pub fn from_str(s: &str)`
- **Converting types**: Implement `From<T>`/`Into<T>` instead of `pub fn to_x()` or `pub fn from_x()`
- **Default values**: Implement `Default` instead of `pub fn new_default()`
- **Iteration**: Implement `Iterator`/`IntoIterator` instead of `pub fn get_items()`
- **Display formatting**: Implement `Display`/`Debug` instead of `pub fn to_string()`
- **Cloning**: Implement `Clone` instead of `pub fn copy()`
- **Comparison**: Implement `PartialEq`/`Eq`/`PartialOrd`/`Ord` instead of custom comparison methods

**Rationale**: Standard traits enable generic programming, work with Rust's type system, and follow Rust idioms.

**Example**:
```rust
// ❌ BAD: Custom public method duplicates trait functionality
impl MyType {
    pub fn from_str(s: &str) -> Option<Self> { /* ... */ }
}

// ✅ GOOD: Implement the standard trait
impl std::str::FromStr for MyType {
    type Err = azure_core::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> { /* ... */ }
}

// Users can then use: "value".parse::<MyType>()
```

If you need a non-fallible parse internally, create a **private** helper method and have the trait implementation call it.

### Cosmos-Specific Patterns

#### Request Building

- Use the builder pattern for complex requests (queries, operations)
- Builders should be type-safe, guiding users through required parameters
- Prefer method chaining with clear, descriptive names

#### Error Handling

- Use `azure_core::Result<T>` for all fallible operations
- Cosmos-specific errors should provide:
  - HTTP status code
  - Request charge (RU/s) when available
  - Activity ID for tracing
  - Sub-status code for detailed error classification

#### Async/Await

- All I/O operations must be async
- Async runtime
  - For `sdk/cosmos/azure_data_cosmos` and `sdk/cosmos/azure_data_cosmos_driver` keep using the same async runtime abstractions as `azure_core` does.
  - Use `tokio` as the async runtime for `sdk/cosmos/azure_data_cosmos_native`
- Streaming  (use of `futures::Stream`)
  - There is no need to consider streaming for payloads of individual requests/responses because the Cosmos DB service enforces rather strict limits on request and response payload size (max. 4 MB - only via config overrides extendable to 16 MB per response payload)
  - There is a need for pagination for example for query or change feed results (could return multiple pages - each page created by one or multiple responses) - so `futures::Stream` might be used there to achieve pagination - but for transport, assuming buffered transport is sufficient.

#### Resource Management

- Model Cosmos resources (databases, containers, items) as distinct types
- Use type state pattern for operations that require specific resource context
- Lifetime parameters should be avoided in public APIs unless absolutely necessary

#### Partitioning

- Make partition key handling explicit and type-safe
- Provide clear guidance on partition key selection in documentation
- Support both single and hierarchical partition keys

#### Query API

- Provide a fluent, type-safe query builder
- Support both SQL API and parameterized queries
- Make continuation tokens transparent but accessible for pagination control

## Code Organization

```text
sdk/cosmos/azure_data_cosmos/
├── src/
│   ├── clients/           # Client types (CosmosClient, Database, Container)
│   ├── models/            # Data models (separate from behavior)
│   ├── operations/        # Operation implementations
│   ├── options/           # Request/Operation option builders
│   ├── query/             # Query builder and execution
│   └── responses/         # Response types with metadata
```

## Azure Cosmos DB Driver Architecture

### Multi-Crate Strategy

The Cosmos DB implementation is split across three crates with distinct purposes and support models:

#### **azure_data_cosmos_driver** (Core Implementation Layer)
- **Purpose**: Core Cosmos DB protocol implementation, transport, routing, and retry handling
- **API**: Public API available for advanced scenarios and cross-language SDK implementation
- **Support**: Community/GitHub support only (no 24x7 Microsoft Support)
- **Versioning**: Strict semantic versioning; can move to new major versions more frequently
- **Consumers**: `azure_data_cosmos` (Rust SDK), `azure_data_cosmos_native` (C API), potentially other language SDKs

#### **azure_data_cosmos** (Primary Rust SDK)
- **Purpose**: Idiomatic Rust API for Cosmos DB with type-safe serialization
- **API**: Full public SDK following Azure SDK Design Guidelines
- **Support**: Full 24x7 Microsoft Support
- **Versioning**: Strict semantic versioning; must maintain backward compatibility for years per major version
- **Dependency**: Uses `azure_data_cosmos_driver` internally

#### **azure_data_cosmos_native** (C API Wrapper)
- **Purpose**: C-compatible FFI for cross-language reuse (Java, .NET, Python SDKs)
- **API**: C ABI with cdylib/staticlib output
- **Support**: Community/GitHub support (no 24x7 Microsoft Support)
- **Dependency**: Uses `azure_data_cosmos_driver` internally

### Versioning Strategy: No Model Sharing

**Avoid sharing models between crates** to prevent version lock-in:

- ❌ **Do NOT** create `azure_data_cosmos_common` for shared models
- ❌ **Do NOT** directly expose `azure_data_cosmos_driver` models in `azure_data_cosmos` public API
- ✅ **DO** keep models separate in each crate
- ✅ **DO** use adapter functions/traits for conversion between crate models
- ✅ **DO** duplicate model definitions when needed to maintain independent versioning

**Rationale**: `azure_data_cosmos` must maintain multi-year backward compatibility. If it exposes `azure_data_cosmos_driver` models directly, the driver cannot make breaking changes without forcing an SDK major version bump.

**Exception**: Truly stable types unlikely to change (e.g., fundamental constants) can be shared via workspace dependencies.

### Schema-Agnostic Data Plane Principle

**Critical Architectural Rule**: `azure_data_cosmos_driver` is completely ignorant of document/item schemas and serialization formats.

**Rationale**:
- Cosmos DB is a **schemaless database** - item structure is application-defined
- Driver must support multiple language SDKs (Rust, Java, .NET, Python) each with native serialization patterns
- Serialization is a **core capability** that must be handled natively in the consuming SDK

**Driver Data Plane Contract**:
```rust
// Driver APIs work with raw bytes (buffered, ≤16MB payload limit)
pub async fn create_item(
    &self,
    partition_key: &PartitionKey,
    item_body: &[u8],  // UTF-8 JSON or Cosmos binary encoding
    options: &CreateItemOptions,
) -> Result<ItemResponse> {
    // Driver handles transport, routing, retries
    // Driver is IGNORANT of item schema
}

pub struct ItemResponse {
    pub body: Vec<u8>,  // Raw response body (buffered)
    pub diagnostics: DiagnosticsContext,
    pub charge: f64,
    // ... metadata, no typed item
}
```

**SDK Layer Responsibility** (`azure_data_cosmos`):
```rust
// SDK provides type-safe serialization on top of driver
pub async fn create_item<T: Serialize>(
    &self,
    item: &T,
    options: &CreateItemOptions,
) -> azure_core::Result<ItemResponse<T>>
where
    T: for<'de> Deserialize<'de>,
{
    let body = serde_json::to_vec(item)?;  // SDK handles serialization
    let raw_response = self.driver.create_item(pk, &body, options).await?;
    let typed_item: T = serde_json::from_slice(&raw_response.body)?;
    // SDK provides typed response
}
```

**Content Encoding**: UTF-8 JSON vs Cosmos binary encoding is detected automatically based on the first byte value (transparent to API).

**Implications**:
- Driver `models/` contains **zero** item/document types
- Driver APIs accept `&[u8]` for request bodies
- Driver APIs return `Vec<u8>` for response bodies (buffered; Cosmos enforces 4MB default, 16MB absolute max)
- Each consuming SDK implements its own serialization strategy

## Module Organization Patterns

### What Qualifies as a "Model"

**Definition**: A serializable data structure representing values transmitted over the wire to or from the Cosmos DB service (request/response body or header values).

**Critical Requirement**: All types in `models/` **must be serializable** and used for wire format. Configuration types that control SDK behavior belong in `options/`.

**Include in `models/` module**:
- ✅ **Management/metadata resource representations**: Account properties, offers, database properties, container properties, partition key ranges (NOT data plane documents/items)
- ✅ **Supporting structures**: Types that are properties of models (IndexingPolicy, PartitionKeyDefinition, VectorEmbeddingPolicy, consistency levels, indexing modes, connection modes)
- ✅ **Operation-specific envelopes**: Structures created for operation support (TransactionalBatch, PatchDocument, BulkOperations)
- ✅ **Header/wire values**: Types serialized into request headers or body (ETag, SessionToken, PartitionKey, TriggerReference, ThroughputControlGroupName)

**Exclude from `models/` (use dedicated modules)**:
- ❌ **Configuration/options types** → `options/` module (e.g., Region for excluded regions, TriggerOptions for which triggers to use, ThroughputControlGroupOptions)
- ❌ Client types → `clients/` module
- ❌ Error types → `error.rs` or crate root
- ❌ **Diagnostics/telemetry types** → `diagnostics/` module
- ❌ Pipeline/routing/internal types → separate modules with `pub(crate)` visibility
- ❌ **Data plane items/documents** → NOT in driver (see Schema-Agnostic Data Plane above)
- ❌ Builders → `builders/` submodule or inline in `models/builders/`

**Model Characteristics**:
```rust
/// Models should be serializable and debuggable
#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[non_exhaustive]  // For forward compatibility
pub struct ContainerProperties {
    pub id: Cow<'static, str>,
    pub partition_key: PartitionKeyDefinition,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexing_policy: Option<IndexingPolicy>,
}

impl ContainerProperties {
    // Models CAN have methods (builders, accessors, conversions)
    pub fn builder() -> ContainerPropertiesBuilder {
        ContainerPropertiesBuilder::default()
    }
}
```

### Diagnostics Module Pattern

Cosmos DB provides rich diagnostic information beyond standard Azure SDK telemetry. This operational metadata lives in a dedicated `diagnostics/` module:

**Structure**:
```text
src/diagnostics/
  ├── mod.rs              # Public exports
  ├── context.rs          # DiagnosticsContext (Rust equivalent of CosmosDiagnosticsContext)
  ├── metrics.rs          # Request metrics, RU consumption, timings
  └── regions.rs          # Regional endpoint contact info
```

**Rationale**: Diagnostics types are NOT service resources - they are SDK operational metadata. Keeping them separate from `models/` maintains clean separation between:
- **Models**: What Cosmos DB service defines (resources, policies, envelopes)
- **Diagnostics**: What the SDK tracks about operations (timings, retries, endpoints contacted)

**Example**:
```rust
/// Context containing diagnostic information about a Cosmos DB operation
pub struct DiagnosticsContext {
    pub request_charge: f64,
    pub regions_contacted: Vec<RegionContactInfo>,
    pub retry_attempts: Vec<RetryInfo>,
    pub request_timeline: RequestTimeline,
}

// Attached to operation results, not stored in models
pub struct ItemResponse {
    pub body: Vec<u8>,  // Raw bytes, schema-agnostic
    pub diagnostics: DiagnosticsContext,
}
```

### Options Module Pattern

Configuration types for individual operations/requests follow Azure SDK conventions in dedicated `options/` module:

**Structure**:
```text
src/options/
  ├── mod.rs          # Public exports
  ├── client.rs       # {Crate}Options (client-level config)
  ├── connection.rs   # ConnectionPoolOptions, environment config
  └── operations.rs   # {Operation}Options (per-request config)
```

**Naming Convention**:
- Client-level: `DriverOptions`, `CosmosClientOptions`
- Operation-level: `{Operation}{Action}Options` (e.g., `ExecuteQueryOptions`, `CreateContainerOptions`)
- Domain-specific: Descriptive names (`ConnectionPoolOptions`, `RetryOptions`)

**Hierarchy (Environment → Client → Operation)**:
```rust
// Client options nest azure_core types
pub struct DriverOptions {
    pub client_options: ClientOptions,  // From azure_core
    pub connection_pool: ConnectionPoolOptions,
    pub default_consistency: Option<ConsistencyLevel>,
}

// Operation options can override client defaults
pub struct ExecuteQueryOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,  // For tracing context
    pub consistency: Option<ConsistencyLevel>,    // Override client default
    pub max_item_count: Option<i32>,
}
```

### Builder Pattern

Use builders for complex client construction:

```rust
pub mod builders {
    pub struct DriverBuilder {
        connection_pool: Option<ConnectionPoolOptions>,
        retry: Option<RetryOptions>,
    }

    impl DriverBuilder {
        pub fn new() -> Self { Self::default() }

        pub fn with_connection_pool(mut self, opts: ConnectionPoolOptions) -> Self {
            self.connection_pool = Some(opts);
            self
        }

        pub async fn build(
            self,
            endpoint: impl Into<String>,
            credential: impl TokenCredential,
            options: DriverOptions,
        ) -> azure_core::Result<Driver> {
            // ... construction logic
        }
    }
}

// Usage:
let driver = Driver::builder()
    .with_connection_pool(pool_opts)
    .build(endpoint, credential, opts).await?;
```

## Testing

- Use `#[recorded::test]` for integration tests that interact with Cosmos DB
- Unit tests should focus on:
  - Request building logic
  - Response parsing
  - Error handling paths
  - AVOID writing tests for trivial or obvious functionality, like derived trait behavior, constructor behavior that just assigns fields, or simple getters/setters.
- Integration tests should cover:
  - CRUD operations
  - Query scenarios
  - Partition key handling
  - Continuation token pagination
- Tests should use standard trait implementations (e.g., `.parse::<T>()` instead of calling `T::from_str()` directly)
- Use `assert!` for boolean assertions instead of `assert_eq!(value, true/false)`

## Code Quality and Validation

### Automatic Formatting (CRITICAL)

**ALWAYS run `cargo fmt` FIRST before any other validation steps**. The CI pipeline will fail if code is not properly formatted:

```bash
# Format the modified crate
cargo fmt -p <crate-name>

# Check if formatting is correct without modifying files
cargo fmt -p <crate-name> -- --check
```

**This is the most common CI failure** - always run `cargo fmt` after making ANY code changes, including:
- Adding new code
- Modifying existing code
- Moving code between files
- Refactoring

### Automatic Clippy Validation

**ALWAYS run `cargo clippy` after making code changes** to catch common issues and ensure code quality:

```bash
# Run clippy for the modified crate
cargo clippy -p <crate-name> --all-features --all-targets

# Run clippy for the entire workspace (use sparingly)
cargo clippy --workspace --all-features --all-targets --keep-going --no-deps
```

**Fix all clippy warnings before considering code complete**. Common patterns to watch for:

- ✅ Implement standard traits instead of custom methods (`clippy::should_implement_trait`)
- ✅ Use `assert!(value)` instead of `assert_eq!(value, true)`
- ✅ Use `assert!(!value)` instead of `assert_eq!(value, false)`
- ✅ Avoid unnecessary clones, allocations, or copies
- ✅ Use idiomatic patterns (e.g., `if let`, pattern matching)

**When to suppress warnings**: Only use `#[allow(clippy::...)]` when:
- The warning is a false positive
- Following the suggestion would make code less clear
- The pattern is required for FFI or external constraints

Always add a comment explaining **why** the warning is allowed.

### Code Formatting

- Always run `cargo fmt` on generated or modified Rust code before considering the task complete.
- When editing existing files, ensure the changes conform to `rustfmt` standards.
- **The CI pipeline will reject any code that is not properly formatted.**

### Pre-Completion Validation Checklist

Before considering any task complete, run the following checks **in order** on all modified crates:

1. **Format check (MUST BE FIRST)**: `cargo fmt -p <crate-name>`
   - This is the #1 cause of CI failures - always run this first!
2. **Build check**: `cargo build -p <crate-name>`
3. **Clippy lint check**: `cargo clippy -p <crate-name> --all-features --all-targets`
4. **Documentation check**: `cargo doc -p <crate-name> --no-deps --all-features`
   - This catches broken intra-doc links (e.g., referencing non-existent methods in `[`backtick links`]`)
   - All documentation warnings must be resolved before completing the task
5. **Test check** (if tests exist): `cargo test -p <crate-name> --all-features`

**Common documentation link errors to avoid**:
- When documenting factory methods or APIs, ensure the linked method names match the actual implementation
- Use the exact method name (e.g., `method_by_name` not just `method`) in doc links like `[`StructName::method_by_name`]`

### Copyright Header

**All Rust source files** in the `sdk/cosmos` directory must begin with the following copyright header:

```rust
// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
```

This header must be:
- The first two lines of every `.rs` file
- Followed by a blank line before any other content (imports, module docs, etc.)
- Exactly as shown above (no variations in formatting or text)

**Example**:
```rust
// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Module documentation starts here.

use std::time::Duration;
```

## Performance Considerations

- Minimize allocations in hot paths
- Use `Cow<'_, str>` for fields that might be borrowed or owned
- Stream large responses rather than buffering entirely
- Provide options for controlling request throughput (RU/s limits)

## Documentation

Every public API should document:

- **Purpose**: What the API does
- **Example**: Runnable code showing typical usage
- **Errors**: What errors can be returned and why
- **Performance**: RU/s implications, if relevant
- **Partition Key**: Whether the operation is partition-scoped

## Additional Resources

- [Azure Cosmos DB REST API Reference](https://learn.microsoft.com/rest/api/cosmos-db/)
- [Cosmos DB Best Practices](https://learn.microsoft.com/azure/cosmos-db/best-practice-dotnet)
- [Azure SDK Design Guidelines for Rust](https://azure.github.io/azure-sdk/rust_introduction.html)
