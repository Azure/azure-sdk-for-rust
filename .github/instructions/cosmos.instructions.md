---
applyTo: "sdk/cosmos/**/*.*"
---

# Cosmos SDK Instructions

This file contains coding guidelines and architectural patterns specific to the Azure Cosmos DB SDK for Rust (`sdk/cosmos`).

## Core Principles

### Data-Oriented Programming (DOP)

Follow the **Data-Oriented Programming in Rust** principles from [https://analogrelay.github.io/dop-in-rust](https://analogrelay.github.io/dop-in-rust):

- **Separate data from behavior**: Use plain structs for data and separate modules/traits for operations
- **Model data with enums and structs**: Prefer algebraic data types to represent domain concepts
- **Treat data as immutable**: Use owned types and transformations rather than mutation where practical
- **Validate at boundaries**: Parse/validate data when it enters the system, trust it internally

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
  - There is a need for pagination for example for query or change feed results (could retrun multiple pages - each page created by one or multipel responses) - so `futures::Stream` might be used there to achieve pagination - but for transport, assuming buffered transport is sufficient.

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
│   ├── options/           # Request option builders
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

**Definition**: A serializable data structure representing a Cosmos DB management/metadata resource or operation envelope.

**Include in `models/` module**:
- ✅ **Management/metadata resource representations**: Account properties, offers, database properties, container properties, partition key ranges (NOT data plane documents/items)
- ✅ **Supporting structures**: Types that are properties of models (IndexingPolicy, PartitionKeyDefinition, VectorEmbeddingPolicy, consistency levels, indexing modes, connection modes)
- ✅ **Operation-specific envelopes**: Structures created for operation support (TransactionalBatch, PatchDocument, BulkOperations)

**Exclude from `models/` (use dedicated modules)**:
- ❌ Options/configuration types → `options/` module
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

Configuration types follow Azure SDK conventions in dedicated `options/` module:

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
- Integration tests should cover:
  - CRUD operations
  - Query scenarios
  - Partition key handling
  - Continuation token pagination

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
