# Fault Injection Utility for Azure Cosmos DB Testing

This module provides a fault injection framework for testing error handling in Azure Cosmos DB client code. It allows you to intercept HTTP requests at the transport layer and inject various error conditions or transform responses.

## Overview

The fault injection utility is inspired by the Python SDK's fault injection transport and provides similar capabilities in Rust. It implements the `Policy` trait from `azure_core::http::policies`, making it easy to integrate into the request pipeline.

## Core Components

### FaultInjectionPolicy

The main policy that wraps an inner transport policy and applies fault injection rules. It maintains:
- **Fault rules**: Conditions and errors to inject
- **Response transformations**: Functions to modify responses
- **Counters**: Track fault injection statistics

### Predicates

Functions that determine when a fault should be applied to a request:

- `predicate_url_contains_id(id)` - Match URLs containing a specific ID
- `predicate_targets_region(endpoint)` - Match requests to a specific region endpoint
- `predicate_req_payload_contains_id(id)` - Match request payloads containing an ID
- `predicate_req_payload_contains_field(field, value)` - Match payloads with specific field
- `predicate_req_for_document_with_id(id)` - Match document operations by ID
- `predicate_is_database_account_call()` - Match database account operations
- `predicate_is_document_operation()` - Match document operations
- `predicate_is_resource_type(type)` - Match specific resource types
- `predicate_is_operation_type(type)` - Match specific operation types
- `predicate_is_write_operation(uri_prefix)` - Match write operations to a URI prefix

### Fault Factories

Pre-built error generators for common scenarios:

- `error_write_forbidden()` - HTTP 403 Forbidden error
- `error_request_timeout()` - HTTP 408 Request Timeout error
- `error_internal_server_error()` - HTTP 500 Internal Server Error
- `error_region_down()` - IO error simulating connection failure
- `error_service_response()` - Generic service error
- `error_after_delay(ms, error_factory)` - Inject error after a delay

### Response Transformations

- `create_mock_response(status, json_body)` - Create a mock HTTP response

## Usage Example

```rust
use azure_core::http::{policies::Policy, Transport};
use azure_data_cosmos::CosmosClientOptions;
use std::sync::Arc;

// Import fault injection utilities
use framework::fault_injection::*;

// Create a Cosmos client with fault injection
let mut options = CosmosClientOptions::default();

// Get the default transport
let inner_transport = Transport::default();

// Create a transport policy from the inner transport  
// (You'll need to create this based on how your transport is structured)

// Wrap it with fault injection
let fault_policy = FaultInjectionPolicy::new(inner_policy);

// Add a fault that triggers on specific document IDs
fault_policy.add_fault(
    predicate_url_contains_id("test-doc-123".to_string()),
    error_request_timeout(),
    None, // No max count - always trigger
    None, // No after_max_count handler
);

// Add a fault with limited occurrences
fault_policy.add_fault(
    predicate_is_write_operation("dbs/mydb".to_string()),
    error_write_forbidden(),
    Some(3), // Only trigger 3 times
    None,
);

// Add a response transformation
fault_policy.add_response_transformation(
    predicate_is_database_account_call(),
    Arc::new(|request, response| {
        // Modify the response (e.g., change topology)
        response
    }),
);

// Use the client for testing...
// The faults will be injected based on the configured rules
```

## Integration with Cosmos Client

To integrate with the Cosmos client, you can provide a custom transport policy:

```rust
use azure_core::http::Transport;
use typespec_client_core::http::policies::TransportPolicy;

// Create your fault injection policy with an inner transport
let transport = Transport::default();
let transport_policy = Arc::new(TransportPolicy::new(transport));
let fault_policy = Arc::new(FaultInjectionPolicy::new(transport_policy));

// Create a custom transport using the fault injection policy
let custom_transport = Transport::with_policy(fault_policy);

// Set it in client options
let mut options = CosmosClientOptions::default();
options.client_options.transport = Some(custom_transport);

// Create the Cosmos client
let client = CosmosClient::with_key(&endpoint, key, Some(options))?;
```

## Testing Scenarios

### Test Request Timeouts

```rust
fault_policy.add_fault(
    predicate_is_document_operation(),
    error_request_timeout(),
    Some(1), // Fail once
    None,
);

// Your test code that performs document operations
// The first operation will timeout, subsequent ones will succeed
```

### Test Region Failover

```rust
fault_policy.add_fault(
    predicate_targets_region("https://eastus.documents.azure.com".to_string()),
    error_region_down(),
    None, // Always fail
    None,
);

// Test that your code correctly fails over to another region
```

### Test Write Restrictions

```rust
fault_policy.add_fault(
    predicate_is_write_operation("dbs/mydb".to_string()),
    error_write_forbidden(),
    None,
    None,
);

// Test read-only scenarios
```

### Test Delayed Responses

```rust
fault_policy.add_fault(
    predicate_url_contains_id("slow-doc".to_string()),
    error_after_delay(5000, error_request_timeout()),
    None,
    None,
);

// Test timeout handling with actual delays
```

## Thread Safety

The `FaultInjectionPolicy` is thread-safe and can be shared across multiple threads. All internal state is protected by mutexes.

## Counters

The utility tracks fault injection statistics:

```rust
// Get the current count
let count = fault_policy.get_counter("error_with_counter");

// Reset all counters
fault_policy.reset_counters();

// Use the counter function to track a specific error
let error = fault_policy.error_with_counter(some_error);
```

## Cleaning Up

```rust
// Clear all fault rules
fault_policy.clear_faults();

// Clear all response transformations  
fault_policy.clear_transforms();

// Reset counters
fault_policy.reset_counters();
```

## Comparison with Python Implementation

This Rust implementation provides similar capabilities to the Python SDK's fault injection:

| Feature | Python | Rust |
|---------|--------|------|
| Request predicates | ✓ | ✓ |
| Fault factories | ✓ | ✓ |
| Response transformations | ✓ | ✓ |
| Max count with fallback | ✓ | ✓ |
| Counters | ✓ | ✓ |
| Thread-safe | - | ✓ |

Key differences:
- Rust version uses `Arc` and `Mutex` for thread safety
- Rust version integrates with the `Policy` trait from azure_core
- Rust version uses type-safe error handling with `Result` types
- Rust predicates and factories use closures with `Arc` for sharing

## See Also

- `tests/fault_injection_example.rs` - Example usage and tests
- Python SDK fault injection: Similar concept from Azure Cosmos DB Python SDK
