# Fault Injection for Azure Cosmos DB Rust SDK

This module provides a fault injection framework for testing error handling and resilience in Azure Cosmos DB client applications. It allows you to simulate various server errors and network conditions at the HTTP transport layer.

## Overview

The fault injection utility intercepts HTTP requests at the transport layer, below the retry policy. When a fault is injected, it triggers the same retry and failover behavior as a real service error. This enables testing of:

- Error handling for various HTTP status codes (503, 500, 429, 408, etc.)
- Retry logic and backoff behavior
- Regional failover scenarios
- Operation-specific error handling

## Enabling Fault Injection

Fault injection requires the `fault_injection` feature flag:

```toml
[dependencies]
azure_data_cosmos = { version = "0.31", features = ["fault_injection"] }
```

## Core Components

### FaultInjectionClientBuilder

The entry point for configuring fault injection. It wraps the default HTTP transport with a `FaultClient` and sets `fault_injection_enabled` on the `CosmosClientOptions`.

### FaultInjectionCondition

Defines when a fault should be applied. Conditions can filter by:

- **Operation Type**: Target specific operations (ReadItem, CreateItem, QueryItem, etc.)
- **Region**: Target requests to specific regions
- **Container ID**: Target requests to specific containers

### FaultInjectionResult

Defines what error to inject. Built using `FaultInjectionResultBuilder`.

Configurable fields:

| Field | Description |
|-------|-------------|
| `error_type` | The `FaultInjectionErrorType` to inject (optional; omit for delay-only faults) |
| `delay` | Artificial latency applied after the request or fault response (default: none) |
| `probability` | Chance of injecting the fault per request, 0.0–1.0 (default: 1.0) |
| `times` | Number of times to inject the error (default: unlimited) |

Supported error types (`FaultInjectionErrorType`):

| Error Type | HTTP Status | Description |
|------------|-------------|-------------|
| `InternalServerError` | 500 | Internal server error |
| `TooManyRequests` | 429 | Rate limiting / throttling |
| `ReadSessionNotAvailable` | 404 (substatus 1022) | Session consistency error |
| `Timeout` | 408 | Request timeout |
| `ServiceUnavailable` | 503 | Service temporarily unavailable |
| `PartitionIsGone` | 410 (substatus 1002) | Partition moved/split |
| `WriteForbidden` | 403 (substatus 3) | Write forbidden |
| `DatabaseAccountNotFound` | 403 (substatus 1008) | Database account not found |

### FaultInjectionRule

Combines a condition with a result and additional controls. Built using `FaultInjectionRuleBuilder`.

| Field | Description |
|-------|-------------|
| `id` | Unique identifier for the rule |
| `condition` | `FaultInjectionCondition` controlling when the rule applies |
| `result` | `FaultInjectionResult` defining the injected fault |
| `duration` | How long the rule remains active after activation (default: infinite) |
| `start_delay` | Delay before the rule becomes active (default: none) |
| `hit_limit` | Maximum number of times the rule fires (default: unlimited) |

## Supported Operation Types

```rust
pub enum FaultOperationType {
    // Document operations
    ReadItem,
    QueryItem,
    CreateItem,
    UpsertItem,
    ReplaceItem,
    DeleteItem,
    PatchItem,
    BatchItem,
    ChangeFeedItem,

    // Metadata operations
    MetadataReadContainer,
    MetadataReadDatabaseAccount,
    MetadataQueryPlan,
    MetadataPartitionKeyRanges,
}
```

## Usage Examples

### Basic Setup

```rust
use azure_data_cosmos::fault_injection::{
    FaultInjectionClientBuilder,
    FaultInjectionConditionBuilder,
    FaultInjectionErrorType,
    FaultInjectionResultBuilder,
    FaultInjectionRuleBuilder,
    FaultOperationType,
};
use azure_data_cosmos::CosmosClientOptions;

// Create a fault result to inject
let result = FaultInjectionResultBuilder::new()
    .with_error(FaultInjectionErrorType::ServiceUnavailable)
    .build();

// Create a condition for when to inject the fault
let condition = FaultInjectionConditionBuilder::new()
    .with_operation_type(FaultOperationType::ReadItem)
    .build();

// Create a rule combining the condition and result
let rule = FaultInjectionRuleBuilder::new("read-503-rule", result)
    .with_condition(condition)
    .build();

// Build the fault injection client
let mut fault_builder = FaultInjectionClientBuilder::new();
fault_builder.with_rule(rule);

// Inject into CosmosClientOptions
let options = fault_builder.inject(CosmosClientOptions::default());

// Create your Cosmos client with fault injection enabled
let client = CosmosClient::with_key(&endpoint, key, Some(options))?;
```

### Inject 503 Errors on All Read Operations

```rust
let result = FaultInjectionResultBuilder::new()
    .with_error(FaultInjectionErrorType::ServiceUnavailable)
    .build();

let condition = FaultInjectionConditionBuilder::new()
    .with_operation_type(FaultOperationType::ReadItem)
    .build();

let rule = FaultInjectionRuleBuilder::new("read-503", result)
    .with_condition(condition)
    .build();
```

### Inject Errors with Hit Limit

```rust
// Only fail the first 3 requests
let result = FaultInjectionResultBuilder::new()
    .with_error(FaultInjectionErrorType::TooManyRequests)
    .build();

let rule = FaultInjectionRuleBuilder::new("throttle-3-times", result)
    .with_condition(condition)
    .with_hit_limit(3)
    .build();
```

### Inject Errors with Probability

```rust
// Fail 50% of requests
let result = FaultInjectionResultBuilder::new()
    .with_error(FaultInjectionErrorType::InternalServerError)
    .with_probability(0.5)
    .build();
```

### Inject Delay After Request

```rust
use std::time::Duration;

// Delay-only fault (no error type) — the real request proceeds, then the
// configured delay is applied after the response is received.
let result = FaultInjectionResultBuilder::new()
    .with_delay(Duration::from_secs(5))
    .build();
```

### Inject Error with Delay

```rust
use std::time::Duration;

// When both an error type and delay are set, the error is injected and
// the delay is applied after the fault response.
let result = FaultInjectionResultBuilder::new()
    .with_error(FaultInjectionErrorType::Timeout)
    .with_delay(Duration::from_millis(500))
    .build();
```

### Target Specific Region

```rust
let condition = FaultInjectionConditionBuilder::new()
    .with_operation_type(FaultOperationType::CreateItem)
    .with_region("East US")
    .build();
```

### Target Specific Container

```rust
let condition = FaultInjectionConditionBuilder::new()
    .with_operation_type(FaultOperationType::QueryItem)
    .with_container_id("my-container")
    .build();
```

### Rule with Duration and Start Delay

```rust
use std::time::Duration;

let rule = FaultInjectionRuleBuilder::new("delayed-rule", result)
    .with_condition(condition)
    .with_start_delay(Duration::from_secs(10))  // Start after 10 seconds
    .with_duration(Duration::from_secs(60))     // Active for 60 seconds
    .build();
```

## Testing with Fault Injection

### Test Framework Integration

When using the test framework, you can use `TestOptions` to configure fault injection:

```rust
use framework::{TestClient, TestOptions};

// Create fault injection options
let fault_options = fault_builder.inject(CosmosClientOptions::default());

// Run test with both normal and fault-injecting clients
TestClient::run_with_unique_db(
    async |run_context, db_client| {
        // Use db_client for normal operations (setup)

        // Get the fault-injecting client from the run context
        let fault_client = run_context
            .fault_client()
            .expect("fault client should be available");
        let fault_db_client = fault_client.database_client(&db_client.id());
        let fault_container_client = fault_db_client.container_client("my-container");

        // Test that operations fail as expected
        let result = fault_container_client
            .read_item::<MyItem>(&pk, &id, None)
            .await;

        assert!(result.is_err());

        Ok(())
    },
    Some(TestOptions::new().with_fault_client_options(fault_options)),
).await?;
```

## Implementation Details

### How It Works

1. `FaultInjectionClientBuilder::inject()` wraps the default HTTP client with a `FaultClient` and sets `fault_injection_enabled = true` on the options
2. When `fault_injection_enabled` is set, the `GatewayPipeline` adds internal headers (e.g., `x-ms-fault-injection-operation`) to each request identifying the operation type
3. For each request, `FaultClient` checks all rules in order against the request headers
4. If a rule's condition matches and the rule is applicable (timing, hit limit, probability), the fault is injected
5. Internal fault injection headers are removed before forwarding requests to the service

### Rule Evaluation Order

Rules are evaluated in the order they were added. The first matching rule is applied.

### Condition Matching

All specified conditions in a `FaultInjectionCondition` must match (AND logic):
- If `operation_type` is set, it must match the operation header on the request
- If `region` is set, the request URL must contain the region
- If `container_id` is set, the request URL must contain the container ID

If no conditions are specified, the rule matches all requests.

### Thread Safety

`FaultClient` is thread safe. All mutable rule state (hit counts, activation times) is protected by a `Mutex` that is held only during rule evaluation and released before any `.await` points.

