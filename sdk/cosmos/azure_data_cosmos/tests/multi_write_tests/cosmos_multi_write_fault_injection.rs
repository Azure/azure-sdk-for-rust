// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![cfg(feature = "key_auth")]
#![cfg(feature = "fault_injection")]

use super::framework;

use azure_core::http::StatusCode;
use azure_data_cosmos::fault_injection::{
    FaultInjectionClientBuilder, FaultInjectionConditionBuilder, FaultInjectionErrorType,
    FaultInjectionResultBuilder, FaultInjectionRuleBuilder, FaultOperationType,
};
use azure_data_cosmos::models::{ContainerProperties, ThroughputProperties};
use azure_data_cosmos::CosmosClientOptions;
use framework::{get_global_endpoint, TestClient, TestOptions, HUB_REGION, SATELLITE_REGION};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, error::Error};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct NestedItem {
    nested_value: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct TestItem {
    id: Cow<'static, str>,
    partition_key: Option<Cow<'static, str>>,
    value: usize,
    nested: NestedItem,
    bool_value: bool,
}

/// Test that verifies fault injection with various server error types.
/// Each test case creates a fault injection rule that returns the specified error
/// and verifies that the read operation fails with the expected HTTP status code.
///
/// This test uses two clients:
/// - A normal client for creating items
/// - A fault injection client for reading items (which will fail with the injected error)
#[tokio::test]
pub async fn item_read_with_fault_injection_all_fail() -> Result<(), Box<dyn Error>> {
    let test_cases = vec![
        (
            "503 Service Unavailable",
            FaultInjectionErrorType::ServiceUnavailable,
            StatusCode::ServiceUnavailable,
        ),
        (
            "500 Internal Server Error",
            FaultInjectionErrorType::InternalServerError,
            StatusCode::InternalServerError,
        ),
        (
            "429 Too Many Requests",
            FaultInjectionErrorType::TooManyRequests,
            StatusCode::TooManyRequests,
        ),
        (
            "408 Request Timeout",
            FaultInjectionErrorType::Timeout,
            StatusCode::RequestTimeout,
        ),
        (
            "410 Partition Is Gone",
            FaultInjectionErrorType::PartitionIsGone,
            StatusCode::Gone,
        ),
    ];

    for (name, error_type, expected_status) in test_cases {
        println!("Testing fault injection: {}", name);

        // Create a fault injection rule that always returns the specified error
        let server_error = FaultInjectionResultBuilder::new()
            .with_error(error_type)
            .build();

        let condition = FaultInjectionConditionBuilder::new()
            .with_operation_type(FaultOperationType::ReadItem)
            .build();

        let rule = FaultInjectionRuleBuilder::new(&format!("{:?}-always", name), server_error)
            .with_condition(condition)
            .build();

        let mut fault_builder = FaultInjectionClientBuilder::new();
        fault_builder.with_rule(rule);

        // Inject the fault into client options for the fault client
        let fault_options = fault_builder.inject(CosmosClientOptions::default());

        TestClient::run_with_unique_db(
            async |run_context, db_client| {
                let container_id = format!("Container-{}", Uuid::new_v4());
                let container_client = run_context
                    .create_container_with_throughput(
                        db_client,
                        ContainerProperties {
                            id: container_id.clone().into(),
                            partition_key: "/partition_key".into(),
                            ..Default::default()
                        },
                        ThroughputProperties::manual(400),
                    )
                    .await?;

                let unique_id = Uuid::new_v4().to_string();

                // Create an item using the normal client (this should succeed)
                let item = TestItem {
                    id: format!("Item1-{}", unique_id).into(),
                    partition_key: Some(format!("Partition1-{}", unique_id).into()),
                    value: 42,
                    nested: NestedItem {
                        nested_value: "Nested".into(),
                    },
                    bool_value: true,
                };

                let pk = format!("Partition1-{}", unique_id);
                let item_id = format!("Item1-{}", unique_id);

                container_client.create_item(&pk, &item, None).await?;

                // Get the fault client's container client
                let fault_client = run_context
                    .fault_client()
                    .expect("fault client should be available");
                let fault_db_client = fault_client.database_client(&db_client.id());
                let fault_container_client = fault_db_client.container_client(&container_id);

                // Try to read the item using the fault client - this should fail with the injected error
                let result = run_context
                    .read_item::<TestItem>(&fault_container_client, &pk, &item_id, None)
                    .await;

                // Verify we got the expected error
                let err = result.expect_err(&format!(
                    "expected the read to fail with {:?}",
                    expected_status
                ));
                assert_eq!(
                    Some(expected_status),
                    err.http_status(),
                    "Test case '{}': expected {:?}, got {:?}",
                    name,
                    expected_status,
                    err.http_status()
                );

                Ok(())
            },
            Some(TestOptions::new().with_fault_client_options(fault_options)),
        )
        .await?;
    }

    Ok(())
}

/// Test that verifies fault injection only affects the specified operation type.
/// When the fault condition is set to CreateItem, ReadItem operations should succeed.
///
/// This test uses two clients:
/// - A normal client for creating items
/// - A fault injection client (with CreateItem fault) for reading items (which should succeed)
#[tokio::test]
pub async fn item_read_succeeds_when_fault_targets_create_item() -> Result<(), Box<dyn Error>> {
    // Create a fault injection rule that returns 503 for CreateItem operations
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .build();

    let rule = FaultInjectionRuleBuilder::new("create-item-503", server_error)
        .with_condition(condition)
        .build();

    let mut fault_builder = FaultInjectionClientBuilder::new();
    fault_builder.with_rule(rule);

    // Inject the fault into client options for the fault client
    let fault_options = fault_builder.inject(CosmosClientOptions::default());

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            // Create a container using the normal client
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
                    ThroughputProperties::manual(400),
                )
                .await?;

            let unique_id = Uuid::new_v4().to_string();

            let item = TestItem {
                id: format!("Item1-{}", unique_id).into(),
                partition_key: Some(format!("Partition1-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };

            let pk = format!("Partition1-{}", unique_id);
            let item_id = format!("Item1-{}", unique_id);

            // Create the item using the normal client (this should succeed)
            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

            // Read the item using the fault client - this should succeed because the fault only targets CreateItem
            let result = run_context
                .read_item::<TestItem>(&fault_container_client, &pk, &item_id, None)
                .await;

            // Verify the read succeeded
            assert!(
                result.is_ok(),
                "Read should succeed when fault targets CreateItem, but got error: {:?}",
                result.err()
            );

            let response = result.unwrap();
            assert_eq!(response.status(), StatusCode::Ok);
            assert_eq!(
                response
                    .request()
                    .clone()
                    .into_raw_request()
                    .url()
                    .host_str()
                    .unwrap(),
                get_global_endpoint()
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_client_options(fault_options)),
    )
    .await
}

/// Test read region retries - inject 503 for primary region, verify cross region retries.
#[tokio::test]
pub async fn fault_injection_read_region_retry_503() -> Result<(), Box<dyn Error>> {
    // Create a fault injection rule that returns 503 for reads targeting the primary region
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_times(1)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::ReadItem)
        .with_region(HUB_REGION)
        .build();

    let rule = FaultInjectionRuleBuilder::new("primary-region-503", server_error)
        .with_condition(condition)
        .build();

    let mut fault_builder = FaultInjectionClientBuilder::new();
    fault_builder.with_rule(rule);
    let client_options = CosmosClientOptions {
        application_preferred_regions: vec![HUB_REGION, SATELLITE_REGION],
        ..Default::default()
    };
    let fault_options = fault_builder.inject(client_options);

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            let container_client = run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
                    ThroughputProperties::manual(400),
                )
                .await?;

            let unique_id = Uuid::new_v4().to_string();
            let item = TestItem {
                id: format!("Item-{}", unique_id).into(),
                partition_key: Some(format!("Partition-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };

            let pk = format!("Partition-{}", unique_id);
            let item_id = format!("Item-{}", unique_id);

            container_client.create_item(&pk, &item, None).await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

            // Read should succeed on satellite region after primary returns 503
            let result = run_context
                .read_item::<TestItem>(&fault_container_client, &pk, &item_id, None)
                .await;

            let response = result.unwrap();
            let request_url = response
                .request()
                .clone()
                .into_raw_request()
                .url()
                .to_string();
            println!("Request succeeded via failover, final URL: {}", request_url);
            // Verify the request went to a different endpoint than the faulted one
            assert!(
                request_url.contains(&SATELLITE_REGION.as_str()),
                "request should have failed over to secondary region"
            );

            Ok(())
        },
        Some(TestOptions::new().with_fault_client_options(fault_options)),
    )
    .await
}

/// Test write region retries - inject 503 for primary region, verify cross region retries.
#[tokio::test]
pub async fn fault_injection_write_region_retry_503() -> Result<(), Box<dyn Error>> {
    let server_error = FaultInjectionResultBuilder::new()
        .with_error(FaultInjectionErrorType::ServiceUnavailable)
        .with_times(1)
        .build();

    let condition = FaultInjectionConditionBuilder::new()
        .with_operation_type(FaultOperationType::CreateItem)
        .with_region(HUB_REGION)
        .build();

    let rule = FaultInjectionRuleBuilder::new("write-region-503", server_error)
        .with_condition(condition)
        .build();

    let mut fault_builder = FaultInjectionClientBuilder::new();
    fault_builder.with_rule(rule);

    let client_options = CosmosClientOptions {
        application_preferred_regions: vec![HUB_REGION, SATELLITE_REGION],
        ..Default::default()
    };
    let fault_options = fault_builder.inject(client_options);

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let container_id = format!("Container-{}", Uuid::new_v4());
            run_context
                .create_container_with_throughput(
                    db_client,
                    ContainerProperties {
                        id: container_id.clone().into(),
                        partition_key: "/partition_key".into(),
                        ..Default::default()
                    },
                    ThroughputProperties::manual(400),
                )
                .await?;

            let fault_client = run_context
                .fault_client()
                .expect("fault client should be available");
            let fault_db_client = fault_client.database_client(&db_client.id());
            let fault_container_client = fault_db_client.container_client(&container_id);

            let unique_id = Uuid::new_v4().to_string();
            let item = TestItem {
                id: format!("Item-{}", unique_id).into(),
                partition_key: Some(format!("Partition-{}", unique_id).into()),
                value: 42,
                nested: NestedItem {
                    nested_value: "Nested".into(),
                },
                bool_value: true,
            };
            let pk = format!("Partition-{}", unique_id);

            // Try to create using fault client - should  succeed via retry
            let result = fault_container_client.create_item(&pk, &item, None).await;

            let response = result.unwrap();
            let request_url = response
                .request()
                .clone()
                .into_raw_request()
                .url()
                .to_string();
            println!("Write succeeded via failover, final URL: {}", request_url);

            Ok(())
        },
        Some(TestOptions::new().with_fault_client_options(fault_options)),
    )
    .await
}

// TODO:
//  Test for 404:1002 - try to perform read on satellite region using preferred regions
//  the request should be retried on the hub region and succeed.
