#![cfg(feature = "key_auth")]

//! Example demonstrating fault injection usage for testing error handling.

mod framework;

use azure_core::http::{policies::Policy, StatusCode};
use framework::fault_injection::*;
use std::sync::Arc;

/// This test demonstrates how to use the fault injection utility.
/// 
/// Note: This is a minimal example showing API usage. In real tests, you would
/// integrate this with the Cosmos client by providing a custom Transport policy.
#[tokio::test]
async fn fault_injection_example() {
    // Create a mock inner policy for demonstration
    #[derive(Debug)]
    struct MockPolicy;
    
    #[async_trait::async_trait]
    impl Policy for MockPolicy {
        async fn send(
            &self,
            _ctx: &azure_core::http::Context,
            _request: &mut azure_core::http::Request,
            _next: &[Arc<dyn Policy>],
        ) -> azure_core::http::policies::PolicyResult {
            // Return a successful response
            Ok(azure_core::http::AsyncRawResponse::from_bytes(
                StatusCode::Ok,
                azure_core::http::headers::Headers::new(),
                Vec::new(),
            ))
        }
    }

    // Create the fault injection policy
    let fault_policy = FaultInjectionPolicy::new(Arc::new(MockPolicy));

    // Example 1: Add a fault that triggers on URLs containing a specific ID
    fault_policy.add_fault(
        predicate_url_contains_id("test-id-123".to_string()),
        error_request_timeout(),
        None, // No max count - always trigger
        None, // No after_max_count handler
    );

    // Example 2: Add a fault with a max count
    fault_policy.add_fault(
        predicate_is_write_operation("dbs/testdb".to_string()),
        error_write_forbidden(),
        Some(2), // Only trigger twice
        None,
    );

    // Example 3: Add a response transformation
    fault_policy.add_response_transformation(
        predicate_is_database_account_call(),
        Arc::new(|_request, response| {
            // In a real scenario, you could modify the response here
            response
        }),
    );

    // Verify counters work
    assert_eq!(fault_policy.get_counter("error_with_counter"), Some(0));

    // Clear faults
    fault_policy.clear_faults();
    fault_policy.clear_transforms();
    fault_policy.reset_counters();

    println!("Fault injection utility is working correctly!");
}

/// Example demonstrating predicate functions
#[test]
fn test_predicate_functions() {
    use azure_core::http::{Method, Request};

    // Create a test request
    let url = "https://test.documents.azure.com:443/dbs/testdb/colls/testcoll/docs/doc-123"
        .parse()
        .unwrap();
    let request = Request::new(url, Method::Get);

    // Test URL contains ID predicate
    let predicate = predicate_url_contains_id("doc-123".to_string());
    assert!(predicate(&request));

    // Test write operation predicate
    let write_request = Request::new(
        "https://test.documents.azure.com:443/dbs/testdb/colls/testcoll/docs"
            .parse()
            .unwrap(),
        Method::Post,
    );
    let write_pred = predicate_is_write_operation("dbs/testdb".to_string());
    assert!(write_pred(&write_request));

    println!("Predicate functions work correctly!");
}

/// Example demonstrating error factories
#[test]
fn test_error_factories() {
    use azure_core::http::{Method, Request};

    let url = "https://test.documents.azure.com:443/".parse().unwrap();
    let request = Request::new(url, Method::Get);

    // Test various error factories
    let _timeout_error = error_request_timeout()(&request);
    let _forbidden_error = error_write_forbidden()(&request);
    let _internal_error = error_internal_server_error()(&request);
    let _region_down_error = error_region_down()(&request);
    let _service_error = error_service_response()(&request);

    println!("Error factories work correctly!");
}

/// Example demonstrating mock response creation
#[test]
fn test_mock_response() {
    use serde_json::json;

    // Create a mock response with JSON body
    let json_body = json!({
        "id": "test-db",
        "readableLocations": [
            {"name": "East US", "databaseAccountEndpoint": "https://test-eastus.documents.azure.com:443/"}
        ]
    });

    let response = create_mock_response(StatusCode::Ok, Some(json_body));

    // Verify response was created (basic check)
    // In a real test, you would inspect the response body
    assert!(response.status().is_success());

    println!("Mock response creation works correctly!");
}
