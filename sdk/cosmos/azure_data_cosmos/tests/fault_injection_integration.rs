#![cfg(feature = "key_auth")]

//! Advanced integration example showing how to use fault injection with a Cosmos client.
//!
//! This example demonstrates how to create a Cosmos client with fault injection
//! integrated into the transport layer.

mod framework;

use azure_core::http::policies::Policy;
use framework::fault_injection::*;
use std::sync::Arc;

/// Helper function to create a Transport with fault injection.
///
/// This wraps the default HTTP client transport with a fault injection policy.
///
/// Note: This is a simplified example. In production, you would need to properly
/// integrate this with the transport pipeline including the TransportPolicy.
pub fn create_fault_injection_policy() -> Arc<FaultInjectionPolicy> {
    // Create a mock inner policy for demonstration
    // In real usage, this would be your actual transport policy
    #[derive(Debug)]
    struct MockInnerPolicy;

    #[async_trait::async_trait]
    impl Policy for MockInnerPolicy {
        async fn send(
            &self,
            _ctx: &azure_core::http::Context,
            _request: &mut azure_core::http::Request,
            _next: &[Arc<dyn Policy>],
        ) -> azure_core::http::policies::PolicyResult {
            // This is just for demonstration
            Ok(azure_core::http::AsyncRawResponse::from_bytes(
                azure_core::http::StatusCode::Ok,
                azure_core::http::headers::Headers::new(),
                Vec::new(),
            ))
        }
    }

    let inner_policy = Arc::new(MockInnerPolicy);
    Arc::new(FaultInjectionPolicy::new(inner_policy))
}

#[test]
fn test_create_fault_injection_policy() {
    let fault_policy = create_fault_injection_policy();

    // Add some faults for testing
    fault_policy.add_fault(
        predicate_url_contains_id("test-123".to_string()),
        error_request_timeout(),
        None,
        None,
    );

    // Verify the fault was added
    // (In a real test, you would use this transport with a Cosmos client)
    println!("Created transport with fault injection");
}

/// Example: Creating a Cosmos client with fault injection
///
/// Note: This is a conceptual example. In actual usage, you would integrate this
/// with TestClient or directly with CosmosClient.
#[tokio::test]
async fn example_cosmos_client_with_fault_injection() {
    // Step 1: Create fault injection policy
    let fault_policy = create_fault_injection_policy();

    // Step 2: Configure faults
    // Example: Inject timeout error for documents with specific ID
    fault_policy.add_fault(
        predicate_url_contains_id("failing-doc-id".to_string()),
        error_request_timeout(),
        Some(2), // Fail the first 2 attempts
        None,
    );

    // Example: Inject forbidden error for write operations
    fault_policy.add_fault(
        predicate_is_write_operation("dbs/testdb".to_string()),
        error_write_forbidden(),
        None, // Always fail
        None,
    );

    // Step 3: Create Cosmos client with the custom transport
    // In real usage with a Cosmos client:
    // let custom_transport = Transport::with_policy(fault_policy.clone());
    // let mut options = CosmosClientOptions::default();
    // options.client_options.transport = Some(custom_transport);
    // let client = CosmosClient::with_key(endpoint, key, Some(options))?;
    //
    // For this example, we just demonstrate the setup
    println!("Cosmos client options configured with fault injection");

    // Step 4: Use the client - faults will be injected automatically
    // For example:
    // let db_client = client.database_client("testdb");
    // let result = db_client.create_container(...).await;
    // The result would be an error due to the injected fault

    // Step 5: Clear faults when done or reset for next test
    fault_policy.clear_faults();
    fault_policy.reset_counters();
}

/// Example: Testing retry behavior with max count
#[tokio::test]
async fn example_test_retry_with_max_count() {
    let fault_policy = create_fault_injection_policy();

    // Configure a fault that triggers 3 times then succeeds
    fault_policy.add_fault(
        predicate_is_document_operation(),
        error_request_timeout(),
        Some(3), // Fail 3 times
        None,    // Then succeed
    );

    // In your test, make 4 requests:
    // - First 3 should fail with timeout
    // - 4th should succeed
    // This is useful for testing retry logic

    println!("Configured fault injection for retry testing");
}

/// Example: Testing region failover
#[tokio::test]
async fn example_test_region_failover() {
    let fault_policy = create_fault_injection_policy();

    // Simulate region failure
    fault_policy.add_fault(
        predicate_targets_region("https://eastus.documents.azure.com".to_string()),
        error_region_down(),
        None, // Always fail
        None,
    );

    // Your test would verify that:
    // 1. Requests to East US region fail
    // 2. Client automatically fails over to another region
    // 3. Subsequent requests succeed

    println!("Configured fault injection for region failover testing");
}

/// Example: Testing with response transformation
#[tokio::test]
async fn example_test_response_transformation() {
    let fault_policy = create_fault_injection_policy();

    // Transform database account responses to simulate different topologies
    fault_policy.add_response_transformation(
        predicate_is_database_account_call(),
        Arc::new(|_request, response| {
            // In real usage, you would parse and modify the response body
            // to simulate different account configurations, regions, etc.
            
            // For example, you could:
            // 1. Parse the JSON response body
            // 2. Modify the readableLocations or writableLocations
            // 3. Return the modified response
            
            response
        }),
    );

    println!("Configured response transformation for topology testing");
}

/// Example: Using counters to verify fault injection
#[test]
fn example_using_counters() {
    let fault_policy = create_fault_injection_policy();

    // Initial counter value should be 0
    assert_eq!(fault_policy.get_counter("error_with_counter"), Some(0));

    // You can use error_with_counter to track specific errors
    let error = azure_core::Error::with_message(
        azure_core::error::ErrorKind::Other,
        "Test error",
    );
    let _tracked_error = fault_policy.error_with_counter(error);

    // Counter should now be 1
    assert_eq!(fault_policy.get_counter("error_with_counter"), Some(1));

    // Reset counters
    fault_policy.reset_counters();
    assert_eq!(fault_policy.get_counter("error_with_counter"), Some(0));

    println!("Counter tracking works correctly");
}

/// Example: Complex scenario with multiple fault rules
#[tokio::test]
async fn example_complex_fault_injection_scenario() {
    let fault_policy = create_fault_injection_policy();

    // Scenario: Test application behavior under multiple failure conditions

    // 1. Specific document always fails
    fault_policy.add_fault(
        predicate_url_contains_id("corrupted-doc".to_string()),
        error_internal_server_error(),
        None,
        None,
    );

    // 2. Write operations fail first 2 times
    fault_policy.add_fault(
        predicate_is_write_operation("dbs/prod".to_string()),
        error_request_timeout(),
        Some(2),
        None,
    );

    // 3. Region is down
    fault_policy.add_fault(
        predicate_targets_region("https://westus.documents.azure.com".to_string()),
        error_region_down(),
        None,
        None,
    );

    // 4. Transform account topology
    fault_policy.add_response_transformation(
        predicate_is_database_account_call(),
        Arc::new(|_request, response| {
            // Modify account topology to simulate multi-region setup
            response
        }),
    );

    // Your test would verify:
    // - Application handles corrupted documents gracefully
    // - Retry logic works for write operations
    // - Region failover works correctly
    // - Multi-region scenarios are handled properly

    println!("Complex fault injection scenario configured");
    
    // Cleanup
    fault_policy.clear_faults();
    fault_policy.clear_transforms();
}
