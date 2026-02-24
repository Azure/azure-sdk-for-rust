# Azure Cosmos DB Driver - API Overview

This document provides a comprehensive overview of the `azure_data_cosmos_driver` public API for team discussion and onboarding.

## Three-Layer Architecture

The Azure Cosmos DB Rust ecosystem consists of three distinct layers:

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│ Layer 3: Language-Specific SDKs                                             │
│                                                                             │
│ ┌─────────────────────────────────────────────────────────────────────────┐ │
│ │ azure_data_cosmos (Rust SDK)                                            │ │
│ │ - Idiomatic Rust API with serde serialization                           │ │
│ │ - Microsoft 24x7 support                                                │ │
│ └──────────────────────────────┬──────────────────────────────────────────┘ │
│                                │  DIRECT dependency (does NOT use native)   │
│                                ▼                                            │
│                      ┌─────────────────────────────────────────────────┐    │
│                      │ Layer 1: azure_data_cosmos_driver               │    │
│                      │ - Transport, routing, protocol handling         │    │
│                      │ - raw-byte payloads                             │    │
│                      │ - Community support only                        │    │
│                      └─────────────────────────────────────────────────┘    │
│                                      ▲                                      │
│ ┌──────────────────────┐             │            ┌───────────────────────┐ │
│ │ Java SDK (via JNI)   │             │            │ .NET / Python SDKs    │ │
│ │ - Jackson types      │             │            │ - native interop      │ │
│ └──────────┬───────────┘             │            └───────────┬───────────┘ │
│            │                         │                        │             │
│            ▼                         │                        ▼             │
│ ┌─────────────────────────────────────────────────────────────────────────┐ │
│ │ Layer 2: azure_data_cosmos_native (C-FFI, non-Rust interop only)        │ │
│ │ - Stable C ABI for cross-language interop                               │ │
│ │ - Memory-safe wrappers around driver                                    │ │
│ └─────────────────────────────────────────────────────────────────────────┘ │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Layer Responsibilities

| Layer | Crate                      | Responsibility                                       | Support Level    |
|-------|----------------------------|------------------------------------------------------|------------------|
| 1     | `azure_data_cosmos_driver` | Transport, routing, protocol, retries                | Community/GitHub |
| 2     | `azure_data_cosmos_native` | C-FFI wrapper for non-Rust languages                 | Internal         |
| 3     | `azure_data_cosmos`        | Idiomatic Rust API with serde (uses driver directly) | Microsoft 24x7   |

> **Note**: The Rust SDK (`azure_data_cosmos`) depends directly on `azure_data_cosmos_driver` - it does **not** go through the native layer. The native layer exists solely for cross-language interop (Java, .NET, Python, etc.) via C-FFI.

---

## High-Level Type Overview

```text
  CosmosDriverRuntime                    (Entry point - singleton per process)
          │
          │ get_or_create_driver()
          ▼
    CosmosDriver                         (Per-account driver instance)
          │
          │ execute_operation()
          │
          ▼
    CosmosOperation                      (Built via factory methods)
          │
          │ CosmosOperation::create(resource_ref)
          │ CosmosOperation::read(resource_ref)
          │ CosmosOperation::query(resource_ref)
          │ etc.
          │
          ▼
    CosmosResourceReference              (Typed resource targeting)
          │
          │ Built from typed references:
          │ - ContainerReference::from_name(...)
          │ - ItemReference::from_name(...)
          │ - DatabaseReference::from_name(...)
          │
          ▼
    CosmosResponse                       (Response with diagnostics)
          │
          ├── response_bytes: Vec<u8>
          ├── headers: ResponseHeaders
          └── diagnostics: CosmosDiagnostics
```

### Core Flow

1. **Runtime** manages connection pools, background tasks, caching
2. **Driver** provides access to a specific Cosmos account
3. **Resource Reference** built from typed references (`ContainerReference`, `ItemReference`, etc.)
4. **Operation** created via factory methods: `CosmosOperation::create(resource_ref)`, `.read()`, `.query()`, etc.
5. **Execution** happens via `driver.execute_operation(operation)` - returns `CosmosResponse`

---

## Schema-Agnostic Data Plane / Typed Metadata Plane

The driver follows a split design philosophy for different operation types:

### Data Plane Operations (Schema-Agnostic)

**Applies to**: Item CRUD operations (create/read/update/delete/query items/documents)

The driver is **completely ignorant of item/document schemas and serialization formats**. Item operations work exclusively with raw bytes (`&[u8]` for requests, `Vec<u8>` for responses):

```rust
// Driver API - raw bytes in/out
pub async fn execute_operation(
    &self,
    operation: CosmosOperation,  // operation.body() is Vec<u8>
) -> Result<CosmosResponse> {
  // CosmosResponse::response_bytes() returns Vec<u8>
}
```

**Rationale**:

- Cosmos DB is a schemaless database - item structure is application-defined
- Serialization must be handled by each language SDK natively (Rust: serde, Java: Jackson, .NET: System.Text.Json)
- Driver supports multiple consuming SDKs (Rust, Java, .NET, Python) via the native layer

### Metadata/Control Plane Operations (Typed APIs)

**Applies to**: Database/container/user/permission/offer management operations

Even for metadata operations serialization must be handled by each language SDK natively (Rust: serde, Java: Jackson, .NET: System.Text.Json) - but the driver might also deserialize certain metadata responses - like Container or account metadata to populate driver-internal caches.

### Current Implementation

The driver currently:

- ✅ Uses typed metadata models internally for cache resolution (`DatabaseProperties`, `ContainerProperties`)
- ✅ Returns raw bytes for all data plane  and metadata operations

---

## Code Examples

### Example 1: Simple Master Key Authentication

```rust,no_run
use azure_data_cosmos_driver::{
    CosmosDriverRuntime,
    models::{
        AccountReference, ContainerReference, CosmosOperation, PartitionKey,
    },
};
use url::Url;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // Create runtime (typically once per application)
    let runtime = CosmosDriverRuntime::builder().build().await?;

    // Configure account with master key
    let account = AccountReference::with_master_key(
        Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
        "your-master-key-here",
    );

    // Get or create driver for this account
    let driver = runtime.get_or_create_driver(account.clone(), None).await?;

    // Create a JSON document payload
    let document_json = r#"{
        "id": "doc_001",
        "pk": "HelloWorld",
        "message": "Hello from Cosmos DB!",
        "count": 42
    }"#;

    // Build typed container reference (no raw resource link strings!)
    let container = ContainerReference::from_name(
        account,
        "myDatabase",
        "myContainer",
    );

    // Create a document in the container (target is the container itself)
    let operation = CosmosOperation::create(container)
        .with_partition_key(PartitionKey::from("HelloWorld"))
        .with_body(document_json.as_bytes().to_vec());

    // Execute the operation
    let result = driver.execute_operation(operation, None).await?;

    // Access the response
    println!("Request charge: {} RUs", result.headers().request_charge());
    println!("Activity ID: {}", result.headers().activity_id());
    println!("Response: {}", String::from_utf8_lossy(result.response_bytes()));

    // Access diagnostics for debugging
    let diagnostics = result.diagnostics();
    println!("Total latency: {:?}", diagnostics.elapsed());
    println!("Regions contacted: {:?}", diagnostics.regions_contacted());

    Ok(())
}
```

### Example 2: AAD Authentication with Configuration Mutation

```rust,no_run
use azure_data_cosmos_driver::{
    CosmosDriverRuntime,
    models::{
        AccountReference, ContainerReference, CosmosOperation,
        CosmosResourceReference, ItemReference, PartitionKey,
    },
    options::{DriverOptions, RetryOptions, ConnectionPoolOptions},
};
use azure_identity::DefaultAzureCredential;
use url::Url;
use std::time::Duration;
use std::sync::Arc;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // Build runtime with custom options
    let runtime = CosmosDriverRuntime::builder()
        .driver_options(
            DriverOptions::builder()
                .retry_options(
                    RetryOptions::builder()
                        .max_retries(5)
                        .initial_delay(Duration::from_millis(100))
                        .max_delay(Duration::from_secs(30))
                        .build()
                )
                .connection_pool_options(
                    ConnectionPoolOptions::builder()
                        .max_idle_connections_per_host(20)
                        .idle_timeout(Duration::from_secs(90))
                        .build()
                )
                .build()
        )
        .build()
        .await?;

    // Use AAD credential (recommended for production)
    let credential = Arc::new(DefaultAzureCredential::new()?);

    let account = AccountReference::with_credential(
        Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
        credential,
    );

    // Driver-level option override
    let driver_opts = DriverOptions::builder()
        .retry_options(
            RetryOptions::builder()
                .max_retries(10)  // More aggressive retry for this account
                .build()
        )
        .build();

    let driver = runtime.get_or_create_driver(account.clone(), Some(driver_opts)).await?;

    // Read an existing document using typed references
    let item_ref = ItemReference::from_name(
        account,
        "myDatabase",
        "myContainer",
        "doc_001",
    );

    let read_operation = CosmosOperation::read(item_ref)
        .with_partition_key(PartitionKey::from("HelloWorld"));

    let result = driver.execute_operation(read_operation, None).await?;
    println!("Document: {}", String::from_utf8_lossy(result.response_bytes()));

    Ok(())
}
```

---

## Configuration Hierarchy

Configuration cascades from most general to most specific:

```text
Environment Variables (AZURE_COSMOS_*)
        │
        ▼
Runtime-Level Options (DriverOptions on runtime)
        │
        ▼
Driver-Level Options (per-account overrides)
        │
        ▼
Operation-Level Options (per-request overrides)
```

Each level can selectively override settings from the level above.

---

## Diagnostics Context

The `DiagnosticsContext` provides comprehensive visibility into operation execution for debugging, monitoring, and troubleshooting.

### Type Structure

```text
DiagnosticsContext                       (Immutable, per-operation)
    │
    ├── activity_id: ActivityId          (Unique identifier for the operation)
    ├── duration: Duration               (Total operation time)
    ├── status_code: StatusCode          (Final HTTP status after retries)
    ├── sub_status_code: SubStatusCode   (Cosmos-specific error classification)
    │
    └── requests: Arc<Vec<RequestDiagnostics>>
                    │
                    └── RequestDiagnostics   (Per-HTTP-request details)
                            │
                            ├── execution_context: ExecutionContext
                            │       ├── Initial        (First attempt)
                            │       ├── Retry          (Retry after 429/503/etc.)
                            │       ├── Hedging        (Speculative request)
                            │       ├── RegionFailover (Cross-region retry)
                            │       └── CircuitBreakerProbe (Recovery check)
                            │
                            ├── region: Region
                            ├── endpoint: String
                            ├── status_code: StatusCode
                            ├── sub_status_code: Option<SubStatusCode>
                            ├── request_charge: f64
                            ├── duration_ms: u64
                            ├── request_sent: RequestSentStatus
                            │       ├── Sent     (Definitely transmitted)
                            │       ├── NotSent  (Definitely NOT transmitted)
                            │       └── Unknown  (Cannot determine)
                            │
                            └── events: Vec<RequestEvent>
                                    │
                                    └── RequestEvent
                                            ├── event_type: RequestEventType
                                            │       ├── TransportStart
                                            │       ├── ResponseHeadersReceived
                                            │       ├── TransportComplete
                                            │       └── TransportFailed
                                            ├── timestamp: Instant
                                            ├── duration_ms: Option<u64>
                                            └── details: Option<String>
```

### Pipeline Events & Reqwest Limitations

> ⚠️ **Team Discussion Point**: The driver uses `reqwest` as the HTTP transport. Unlike Reactor Netty (used in the Java SDK), reqwest does **not** expose fine-grained connection lifecycle callbacks directly. Eventually we might need to look into the ClientBuilder::connector_layer which allows us to apply a tower::Layer to the Tower service that handles connections. It's quite low-level but I believe it would allow us to track connection establishment and teardown.

#### What We **Cannot** Track (reqwest limitation)

| Metric                      | Java SDK (Reactor Netty) | Rust SDK (reqwest)     |
|-----------------------------|--------------------------|------------------------|
| DNS resolution time         | ✅ Separate event         | ❌ Bundled in transport |
| Connection pool acquisition | ✅ Separate event         | ❌ Not exposed          |
| New connection vs reused    | ✅ Separate event         | ❌ Not exposed          |
| TLS handshake time          | ✅ Separate event         | ❌ Not exposed          |
| Time to first byte          | ✅ Separate event         | ❌ Not exposed          |
| Request body sent           | ✅ Separate event         | ❌ Not exposed          |

#### What We **Can** Track

| Event                     | Description                                                          |
|---------------------------|----------------------------------------------------------------------|
| `TransportStart`          | Request handed to reqwest - DNS/connect/TLS/send all happen opaquely |
| `ResponseHeadersReceived` | Response headers received (confirms request was sent)                |
| `TransportComplete`       | Headers + body fully received                                        |
| `TransportFailed`         | Error occurred (analyze error type for retry safety)                 |

---

### Verbosity Levels

The diagnostics output can be formatted at two verbosity levels:

| Level      | Description                         | Use Case                                          |
|------------|-------------------------------------|---------------------------------------------------|
| `Detailed` | Full output with every request      | Deep debugging, local development                 |
| `Summary`  | Compacted output with deduplication | Production logging, size-constrained environments |

```rust
use azure_data_cosmos_driver::options::DiagnosticsVerbosity;

// Get JSON output at different verbosity levels
let detailed_json = result.diagnostics().to_json_string(DiagnosticsVerbosity::Detailed);
let summary_json = result.diagnostics().to_json_string(DiagnosticsVerbosity::Summary);
```

### Size Limits (Summary Mode)

Summary mode respects a configurable maximum size to fit log constraints:

```rust
use azure_data_cosmos_driver::options::DiagnosticsOptions;

let options = DiagnosticsOptions::builder()
    .max_summary_size_bytes(8 * 1024)  // 8 KB limit (default)
    .default_verbosity(DiagnosticsVerbosity::Summary)
    .build()?;
```

- **Default**: 8 KB
- **Minimum**: 4 KB
- **Environment Variable**: `AZURE_COSMOS_DIAGNOSTICS_MAX_SUMMARY_SIZE_BYTES`

If output exceeds the limit, it's truncated with an indicator.

### Compaction & Deduplication (Summary Mode)

Summary mode applies intelligent compaction:

1. **Group by region** - Requests are organized by target region
2. **Keep first and last** - Full details preserved for boundary requests
3. **Deduplicate middle** - Similar requests grouped by `(endpoint, status, sub_status, execution_context)`
4. **Statistics** - Count, total RU, min/max/P50 duration for each group

```text
Region: West US 2
├── First Request (full details)
├── Deduplicated Groups:
│   └── [429/3200 Retry × 8] → min: 45ms, max: 890ms, P50: 120ms, total: 8.0 RU
└── Last Request (full details)
```

---

### Example: Successful Read Item (Detailed)

Simple successful read with a single request:

```json
{
  "activity_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "total_duration_ms": 23,
  "total_request_charge": 1.0,
  "request_count": 1,
  "requests": [
    {
      "execution_context": "initial",
      "region": "West US 2",
      "endpoint": "https://myaccount-westus2.documents.azure.com:443/dbs/myDatabase/colls/myContainer/docs/doc_001",
      "status_code": 200,
      "request_charge": 1.0,
      "activity_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
      "duration_ms": 23
    }
  ]
}
```

---

### Example: 10 × 429/3200 Retries Then Success

Scenario: Request throttled 10 times (429/3200) before succeeding on the 11th attempt.

#### Detailed Verbosity (Full Output)

```json
{
  "activity_id": "b2c3d4e5-f6a7-8901-bcde-f12345678901",
  "total_duration_ms": 4523,
  "total_request_charge": 11.0,
  "request_count": 11,
  "requests": [
    {
      "execution_context": "initial",
      "region": "West US 2",
      "endpoint": "https://myaccount-westus2.documents.azure.com:443/dbs/myDatabase/colls/myContainer/docs/doc_001",
      "status_code": 429,
      "sub_status_code": 3200,
      "request_charge": 1.0,
      "duration_ms": 45
    },
    {
      "execution_context": "retry",
      "region": "West US 2",
      "endpoint": "https://myaccount-westus2.documents.azure.com:443/dbs/myDatabase/colls/myContainer/docs/doc_001",
      "status_code": 429,
      "sub_status_code": 3200,
      "request_charge": 1.0,
      "duration_ms": 52
    },
    {
      "execution_context": "retry",
      "region": "West US 2",
      "endpoint": "https://myaccount-westus2.documents.azure.com:443/dbs/myDatabase/colls/myContainer/docs/doc_001",
      "status_code": 429,
      "sub_status_code": 3200,
      "request_charge": 1.0,
      "duration_ms": 78
    },
    {
      "execution_context": "retry",
      "region": "West US 2",
      "endpoint": "https://myaccount-westus2.documents.azure.com:443/dbs/myDatabase/colls/myContainer/docs/doc_001",
      "status_code": 429,
      "sub_status_code": 3200,
      "request_charge": 1.0,
      "duration_ms": 120
    },
    {
      "execution_context": "retry",
      "region": "West US 2",
      "endpoint": "https://myaccount-westus2.documents.azure.com:443/dbs/myDatabase/colls/myContainer/docs/doc_001",
      "status_code": 429,
      "sub_status_code": 3200,
      "request_charge": 1.0,
      "duration_ms": 189
    },
    {
      "execution_context": "retry",
      "region": "West US 2",
      "endpoint": "https://myaccount-westus2.documents.azure.com:443/dbs/myDatabase/colls/myContainer/docs/doc_001",
      "status_code": 429,
      "sub_status_code": 3200,
      "request_charge": 1.0,
      "duration_ms": 312
    },
    {
      "execution_context": "retry",
      "region": "West US 2",
      "endpoint": "https://myaccount-westus2.documents.azure.com:443/dbs/myDatabase/colls/myContainer/docs/doc_001",
      "status_code": 429,
      "sub_status_code": 3200,
      "request_charge": 1.0,
      "duration_ms": 456
    },
    {
      "execution_context": "retry",
      "region": "West US 2",
      "endpoint": "https://myaccount-westus2.documents.azure.com:443/dbs/myDatabase/colls/myContainer/docs/doc_001",
      "status_code": 429,
      "sub_status_code": 3200,
      "request_charge": 1.0,
      "duration_ms": 623
    },
    {
      "execution_context": "retry",
      "region": "West US 2",
      "endpoint": "https://myaccount-westus2.documents.azure.com:443/dbs/myDatabase/colls/myContainer/docs/doc_001",
      "status_code": 429,
      "sub_status_code": 3200,
      "request_charge": 1.0,
      "duration_ms": 780
    },
    {
      "execution_context": "retry",
      "region": "West US 2",
      "endpoint": "https://myaccount-westus2.documents.azure.com:443/dbs/myDatabase/colls/myContainer/docs/doc_001",
      "status_code": 429,
      "sub_status_code": 3200,
      "request_charge": 1.0,
      "duration_ms": 890
    },
    {
      "execution_context": "retry",
      "region": "West US 2",
      "endpoint": "https://myaccount-westus2.documents.azure.com:443/dbs/myDatabase/colls/myContainer/docs/doc_001",
      "status_code": 200,
      "request_charge": 1.0,
      "activity_id": "b2c3d4e5-f6a7-8901-bcde-f12345678901",
      "duration_ms": 28
    }
  ]
}
```

#### Summary Verbosity (Compacted Output)

Same operation with deduplication applied:

```json
{
  "activity_id": "b2c3d4e5-f6a7-8901-bcde-f12345678901",
  "total_duration_ms": 4523,
  "total_request_charge": 11.0,
  "request_count": 11,
  "regions": [
    {
      "region": "West US 2",
      "request_count": 11,
      "total_request_charge": 11.0,
      "first": {
        "execution_context": "initial",
        "endpoint": "https://myaccount-westus2.documents.azure.com:443/dbs/myDatabase/colls/myContainer/docs/doc_001",
        "status_code": 429,
        "sub_status_code": 3200,
        "request_charge": 1.0,
        "duration_ms": 45
      },
      "last": {
        "execution_context": "retry",
        "endpoint": "https://myaccount-westus2.documents.azure.com:443/dbs/myDatabase/colls/myContainer/docs/doc_001",
        "status_code": 200,
        "request_charge": 1.0,
        "duration_ms": 28
      },
      "deduplicated_groups": [
        {
          "endpoint": "https://myaccount-westus2.documents.azure.com:443/dbs/myDatabase/colls/myContainer/docs/doc_001",
          "status_code": 429,
          "sub_status_code": 3200,
          "execution_context": "retry",
          "count": 9,
          "total_request_charge": 9.0,
          "min_duration_ms": 52,
          "max_duration_ms": 890,
          "p50_duration_ms": 312
        }
      ]
    }
  ]
}
```

**Key Differences:**

| Aspect              | Detailed          | Summary                       |
|---------------------|-------------------|-------------------------------|
| Size                | ~2.8 KB           | ~0.8 KB                       |
| Individual requests | All 11 shown      | First + Last only             |
| Middle requests     | Full details each | Grouped as 1 entry with stats |
| Debugging value     | Maximum           | Sufficient for most cases     |

---

## Public API Reference

### Root Module (`azure_data_cosmos_driver`)

#### Core Types

| Type                         | Description                                           |
|------------------------------|-------------------------------------------------------|
| `CosmosDriverRuntime`        | Entry point; manages drivers, pools, background tasks |
| `CosmosDriverRuntimeBuilder` | Builder for `CosmosDriverRuntime`                     |
| `CosmosDriver`               | Per-account driver for executing operations           |
| `CosmosOperation`            | Single operation with context and options             |
| `CosmosResponse`             | Response containing bytes, headers, diagnostics       |

---

### Module: `options`

Configuration types with builder pattern throughout.

#### Option Types

| Type                           | Description                         |
|--------------------------------|-------------------------------------|
| `DriverOptions`                | Top-level driver configuration      |
| `DriverOptionsBuilder`         | Builder for `DriverOptions`         |
| `RetryOptions`                 | Retry policy configuration          |
| `RetryOptionsBuilder`          | Builder for `RetryOptions`          |
| `ConnectionPoolOptions`        | HTTP connection pool settings       |
| `ConnectionPoolOptionsBuilder` | Builder for `ConnectionPoolOptions` |
| `TimeoutOptions`               | Request/operation timeout settings  |
| `TimeoutOptionsBuilder`        | Builder for `TimeoutOptions`        |
| `TelemetryOptions`             | Distributed tracing and logging     |
| `TelemetryOptionsBuilder`      | Builder for `TelemetryOptions`      |

#### Key Configuration Fields

```rust
// RetryOptions
struct RetryOptions {
    max_retries: u32,           // Default: 3
    initial_delay: Duration,    // Default: 100ms
    max_delay: Duration,        // Default: 30s
    retry_on_throttle: bool,    // Default: true
}

// ConnectionPoolOptions
struct ConnectionPoolOptions {
    max_idle_connections_per_host: usize,  // Default: 10
    idle_timeout: Duration,                 // Default: 90s
}

// TimeoutOptions
struct TimeoutOptions {
    request_timeout: Duration,     // Per-request timeout
    operation_timeout: Duration,   // Total operation timeout (incl. retries)
}
```

---

### Module: `models`

Resource definitions and metadata types.

#### Account & Connection

| Type                | Description                                                   |
|---------------------|---------------------------------------------------------------|
| `AccountReference`  | Account endpoint + credentials                                |
| `AccountProperties` | Account metadata (regions, capabilities)                      |
| `ConsistencyLevel`  | Strong, BoundedStaleness, Session, Eventual, ConsistentPrefix |

#### Database & Container

| Type                            | Description                          |
|---------------------------------|--------------------------------------|
| `DatabaseProperties`            | Database metadata                    |
| `ContainerProperties`           | Container configuration              |
| `ContainerPropertiesBuilder`    | Builder for `ContainerProperties`    |
| `PartitionKeyDefinition`        | Partition key path(s) and kind       |
| `PartitionKeyDefinitionBuilder` | Builder for `PartitionKeyDefinition` |
| `PartitionKeyKind`              | Hash, Range, MultiHash               |

#### Indexing

| Type                    | Description                      |
|-------------------------|----------------------------------|
| `IndexingPolicy`        | Container indexing configuration |
| `IndexingPolicyBuilder` | Builder for `IndexingPolicy`     |
| `IndexingMode`          | Consistent, Lazy, None           |
| `IncludedPath`          | Paths to include in index        |
| `ExcludedPath`          | Paths to exclude from index      |
| `SpatialIndex`          | Geospatial index configuration   |
| `CompositeIndex`        | Multi-property composite index   |
| `CompositeIndexOrder`   | Ascending, Descending            |

#### Throughput & Scaling

| Type                          | Description                         |
|-------------------------------|-------------------------------------|
| `ThroughputProperties`        | Provisioned or autoscale throughput |
| `ThroughputPropertiesBuilder` | Builder for `ThroughputProperties`  |
| `AutoscaleSettings`           | Autoscale max throughput            |

#### Conflicts & TTL

| Type                              | Description                    |
|-----------------------------------|--------------------------------|
| `ConflictResolutionPolicy`        | LastWriterWins, Custom, Manual |
| `ConflictResolutionPolicyBuilder` | Builder for conflict policy    |
| `DefaultTimeToLive`               | Off, NoDefault, Seconds(i32)   |

---

### Module: `diagnostics`

Operational telemetry for debugging and monitoring.

#### Core Diagnostics

| Type                   | Description                     |
|------------------------|---------------------------------|
| `CosmosDiagnostics`    | Top-level diagnostics container |
| `OperationDiagnostics` | Per-operation summary           |
| `RequestDiagnostics`   | Per-HTTP-request details        |

#### Metrics & Timing

| Type            | Description                                   |
|-----------------|-----------------------------------------------|
| `RequestCharge` | RU consumption (total, per-request breakdown) |
| `RetryInfo`     | Retry count, reasons, delays                  |
| `TimingInfo`    | Request/response timing breakdown             |
| `RegionInfo`    | Which region(s) handled the request           |

#### Request Tracking

| Type                | Description                                                |
|---------------------|------------------------------------------------------------|
| `RequestSentStatus` | Sent, NotSent, Unknown - tracks if request left the client |
| `RequestEvent`      | Lifecycle events (headers received, body buffered, etc.)   |

#### Key Diagnostic Fields

```rust
struct CosmosDiagnostics {
    operation_id: String,
    total_request_charge: f64,
    total_duration: Duration,
    retry_count: u32,
    requests: Vec<RequestDiagnostics>,
}

struct RequestDiagnostics {
    request_id: String,
    status_code: Option<u16>,
    sub_status_code: Option<u32>,
    request_charge: f64,
    region: String,
    request_sent: RequestSentStatus,
    duration: Duration,
    events: Vec<RequestEvent>,
}
```

---

### Module: `builders`

Fluent builders for complex type construction.

| Type               | Description                  |
|--------------------|------------------------------|
| `PointReadBuilder` | Build point read operations  |
| `QueryBuilder`     | Build query operations       |
| `UpsertBuilder`    | Build upsert operations      |
| `DeleteBuilder`    | Build delete operations      |
| `PatchBuilder`     | Build patch operations       |
| `BulkBuilder`      | Build bulk operation batches |

---

### Enums Summary

| Enum                  | Variants                                                      | Description             |
|-----------------------|---------------------------------------------------------------|-------------------------|
| `ConsistencyLevel`    | Strong, BoundedStaleness, Session, Eventual, ConsistentPrefix | Read consistency        |
| `PartitionKeyKind`    | Hash, Range, MultiHash                                        | Partition strategy      |
| `IndexingMode`        | Consistent, Lazy, None                                        | When to index           |
| `CompositeIndexOrder` | Ascending, Descending                                         | Sort order              |
| `DefaultTimeToLive`   | Off, NoDefault, Seconds(i32)                                  | Document expiration     |
| `RequestSentStatus`   | Sent, NotSent, Unknown                                        | Request lifecycle state |

---

## Error Handling

All fallible operations return `azure_core::Result<T>` (alias for `Result<T, azure_core::Error>`).

### Error Categories

| Category             | When                          | Retryable?        |
|----------------------|-------------------------------|-------------------|
| `HttpError`          | Network/transport failures    | Usually yes       |
| `ServiceError`       | Cosmos DB returned error      | Depends on status |
| `CredentialError`    | Auth token acquisition failed | Usually no        |
| `ConfigurationError` | Invalid options/setup         | No                |

### Status Code Handling

```rust
match result {
    Ok(response) => { /* success */ }
    Err(e) if e.is_throttling() => { /* 429 - back off */ }
    Err(e) if e.is_not_found() => { /* 404 - item missing */ }
    Err(e) if e.is_conflict() => { /* 409 - ETag mismatch */ }
    Err(e) => { /* other error */ }
}
```

---

## Thread Safety

All core types are `Send + Sync`:

- `CosmosDriverRuntime` - safe to share across threads
- `CosmosDriver` - safe to share across threads
- Operations should be created per-request (not shared)

Recommended pattern:

```rust
// Create once at startup
let runtime = Arc::new(CosmosDriverRuntime::builder().build().await?);

// Share across request handlers
let runtime_clone = runtime.clone();
tokio::spawn(async move {
    let driver = runtime_clone.get_or_create_driver(account, None).await?;
    // ... use driver
});
```

---

## Performance Considerations

1. **Runtime is expensive to create** - create once, reuse globally
2. **Driver is cached per-account** - `get_or_create_driver` returns existing instance
3. **Connection pooling is automatic** - configured via `ConnectionPoolOptions`
4. **Retries have backoff** - exponential with jitter, configurable limits
5. **Diagnostics are always collected** - no runtime cost to enable

---

## See Also

<!-- TODO: Add links once files exist in main branch -->
<!--
- README.md - Quick start and basic usage
- CHANGELOG.md - Version history
- [Azure Cosmos DB Documentation](https://learn.microsoft.com/azure/cosmos-db/)
-->
