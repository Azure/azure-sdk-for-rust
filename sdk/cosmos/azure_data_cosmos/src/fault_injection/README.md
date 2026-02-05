# Fault Injection for Azure Cosmos DB Rust SDK

This module provides a fault injection framework for testing error handling and resilience in Azure Cosmos DB client applications. It allows you to simulate various server errors and network conditions at the HTTP transport layer.

## Overview

The fault injection utility intercepts HTTP requests before they reach the Cosmos DB service and can inject errors based on configurable conditions. This enables testing of:

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

The entry point for configuring fault injection. It wraps the default HTTP transport with a fault-injecting client.

### FaultInjectionCondition

Defines when a fault should be applied. Conditions can filter by:

- **Operation Type**: Target specific operations (ReadItem, CreateItem, QueryItem, etc.)
- **Region**: Target requests to specific regions
- **Container ID**: Target requests to specific containers

### FaultInjectionServerError

Defines what error to inject. Supported error types:

| Error Type | HTTP Status | Description |
|------------|-------------|-------------|
| `ServiceUnavailable` | 503 | Service temporarily unavailable |
| `InternalServerError` | 500 | Internal server error |
| `TooManyRequests` | 429 | Rate limiting / throttling |
| `Timeout` | 408 | Request timeout |
| `ReadSessionNotAvailable` | 404 (substatus 1002) | Session consistency error |
| `PartitionIsGone` | 410 (substatus 1002) | Partition moved/split |
| `ResponseDelay` | - | Adds delay to response |
| `ConnectionDelay` | - | Adds delay before connection |

### FaultInjectionRule

Combines a condition with a result and additional controls:

- **Duration**: How long the rule remains active
- **Start Delay**: Delay before the rule becomes active
- **Hit Limit**: Maximum number of times to apply the fault

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
    FaultInjectionRuleBuilder,
    FaultInjectionServerErrorBuilder,
    FaultInjectionServerErrorType,
    FaultOperationType,
};
use azure_data_cosmos::CosmosClientOptions;

// Create a server error to inject
let server_error = FaultInjectionServerErrorBuilder::new(
    FaultInjectionServerErrorType::ServiceUnavailable
)
.build();

// Create a condition for when to inject the fault
let condition = FaultInjectionConditionBuilder::new()
    .with_operation_type(FaultOperationType::ReadItem)
    .build();

// Create a rule combining the condition and error
let rule = FaultInjectionRuleBuilder::new("read-503-rule", server_error)
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
let server_error = FaultInjectionServerErrorBuilder::new(
    FaultInjectionServerErrorType::ServiceUnavailable
)
.build();

let condition = FaultInjectionConditionBuilder::new()
    .with_operation_type(FaultOperationType::ReadItem)
    .build();

let rule = FaultInjectionRuleBuilder::new("read-503", server_error)
    .with_condition(condition)
    .build();
```

### Inject Errors with Hit Limit

```rust
// Only fail the first 3 requests
let server_error = FaultInjectionServerErrorBuilder::new(
    FaultInjectionServerErrorType::TooManyRequests
)
.build();

let rule = FaultInjectionRuleBuilder::new("throttle-3-times", server_error)
    .with_condition(condition)
    .with_hit_limit(3)
    .build();
```

### Inject Errors with Probability

```rust
// Fail 50% of requests
let server_error = FaultInjectionServerErrorBuilder::new(
    FaultInjectionServerErrorType::InternalServerError
)
.with_probability(0.5)
.build();
```

### Inject Errors After Delay

```rust
use std::time::Duration;

let server_error = FaultInjectionServerErrorBuilder::new(
    FaultInjectionServerErrorType::Timeout
)
.with_delay(Duration::from_secs(5))
.build();
```

### Target Specific Region

```rust
let condition = FaultInjectionConditionBuilder::new()
    .with_operation_type(FaultOperationType::CreateItem)
    .with_region("eastus")
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

let rule = FaultInjectionRuleBuilder::new("delayed-rule", server_error)
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

// Run test with both normal and fault clients
TestClient::run_with_shared_db(
    async |run_context, db_client| {
        // Use db_client for normal operations (setup)
        
        // Use fault client for testing error scenarios
        let fault_db_client = run_context.fault_db_client()
            .expect("fault client should be available");
        
        // Test that operations fail as expected
        let result = fault_db_client
            .container_client("my-container")
            .read_item::<MyItem>(&pk, &id, None)
            .await;
        
        assert!(result.is_err());
        
        Ok(())
    },
    Some(TestOptions::new().with_fault_client_options(fault_options)),
).await?;
```

## Thread Safety

The `FaultClient` is thread-safe. All internal state is protected by `Arc` and `Mutex`, allowing safe concurrent access from multiple threads.

## Implementation Details

### How It Works

1. `FaultInjectionClientBuilder::inject()` wraps the default HTTP client with `FaultClient`
2. For each request, `FaultClient` checks all rules in order
3. If a rule's condition matches and the rule is applicable (timing, hit limit), the fault is applied
4. Internal fault injection headers are removed before forwarding requests to the service

### Rule Evaluation Order

Rules are evaluated in the order they were added. The first matching rule is applied.

### Condition Matching

All specified conditions in a `FaultInjectionCondition` must match (AND logic):
- If `operation_type` is set, it must match
- If `region` is set, the URL must contain the region
- If `container_id` is set, the URL must contain the container ID

If no conditions are specified, the rule matches all requests.

## See Also

- [Azure Cosmos DB Documentation](https://docs.microsoft.com/azure/cosmos-db/)
- [Rust SDK Examples](../examples/)
