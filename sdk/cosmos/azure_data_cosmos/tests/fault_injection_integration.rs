#![cfg(feature = "key_auth")]

//! End-to-end integration tests for fault injection.
//!
//! These tests construct a real `CosmosClient` (via the integration test framework) but override
//! the HTTP transport with a `FaultInjectionPolicy` placed at the end of the pipeline.
//!
//! The goal is to validate that injected faults interrupt real SDK operations as expected.

mod framework;

use std::sync::Arc;

use azure_core::http::{policies::{Policy, TransportPolicy}, Transport};
use azure_data_cosmos::{models::ContainerProperties, CosmosClientOptions, CreateContainerOptions};

use framework::{fault_injection::*, TestClient};

fn cosmos_options_with_fault_injection(
    fault_policy: Arc<FaultInjectionPolicy>,
) -> CosmosClientOptions {
    let mut options = CosmosClientOptions::default();

    // Put the fault injection policy at the end of the pipeline (transport position).
    options.client_options.transport = Some(Transport::new_custom_policy(
        fault_policy as Arc<dyn Policy>,
    ));

    options
}

#[tokio::test]
pub async fn fault_injection_injects_timeout_on_create_container() -> Result<(), Box<dyn std::error::Error>> {
    let container_id = "FaultInjectionTimeout";

    // Inject a timeout for requests that contain our container id.
    let inner_transport = Arc::new(TransportPolicy::default());
    let fault_policy = Arc::new(FaultInjectionPolicy::new(inner_transport));

    fault_policy.add_fault(
        predicate_url_contains_id(container_id.to_string()),
        error_request_timeout(),
        None,
        None,
    );

    let options = cosmos_options_with_fault_injection(fault_policy.clone());

    TestClient::run_with_db_client_options(options, async move |_, db_client| {
        let properties = ContainerProperties {
            id: container_id.into(),
            partition_key: "/id".into(),
            ..Default::default()
        };

        let err = db_client
            .create_container(properties, Some(CreateContainerOptions::default()))
            .await
            .expect_err("expected injected timeout error");

        assert_eq!(err.http_status(), Some(azure_core::http::StatusCode::RequestTimeout));

        Ok(())
    })
    .await?;

    // Counter reset / cleanup isn't required for correctness here, but keeping it tidy helps
    // when reusing a policy in more complex tests.
    fault_policy.clear_faults();
    fault_policy.reset_counters();

    Ok(())
}

#[tokio::test]
pub async fn fault_injection_max_count_allows_success_after_failures() -> Result<(), Box<dyn std::error::Error>> {
    let container_id = "FaultInjectionRetry";

    // Inject a timeout twice, then let the request through.
    let inner_transport = Arc::new(TransportPolicy::default());
    let fault_policy = Arc::new(FaultInjectionPolicy::new(inner_transport));

    fault_policy.add_fault(
        predicate_url_contains_id(container_id.to_string()),
        error_request_timeout(),
        Some(2),
        None,
    );

    let options = cosmos_options_with_fault_injection(fault_policy.clone());

    TestClient::run_with_db_client_options(options, async move |_, db_client| {
        let properties = ContainerProperties {
            id: container_id.into(),
            partition_key: "/id".into(),
            ..Default::default()
        };

        // 1st attempt fails
        let err = db_client
            .create_container(properties.clone(), Some(CreateContainerOptions::default()))
            .await
            .expect_err("expected injected timeout on attempt 1");
        assert_eq!(err.http_status(), Some(azure_core::http::StatusCode::RequestTimeout));

        // 2nd attempt fails
        let err = db_client
            .create_container(properties.clone(), Some(CreateContainerOptions::default()))
            .await
            .expect_err("expected injected timeout on attempt 2");
        assert_eq!(err.http_status(), Some(azure_core::http::StatusCode::RequestTimeout));

        // 3rd attempt succeeds (fault has max_count=2 and then doesn't fire)
        let _created = db_client
            .create_container(properties, Some(CreateContainerOptions::default()))
            .await?;

        Ok(())
    })
    .await?;

    Ok(())
}
