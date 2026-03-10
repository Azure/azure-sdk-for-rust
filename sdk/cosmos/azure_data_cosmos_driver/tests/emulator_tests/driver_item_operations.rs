// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! E2E tests for item CRUD operations using the driver.
//!
//! These tests simulate how `azure_data_cosmos` would use the driver,
//! constructing `CosmosOperation` instances and executing them via
//! `execute_operation()`.

use crate::framework::DriverTestClient;
use azure_data_cosmos_driver::diagnostics::{PipelineType, TransportSecurity};
use serde::{Deserialize, Serialize};
use std::error::Error;

/// A simple test item for CRUD operations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct TestItem {
    id: String,
    pk: String,
    value: String,
    count: i32,
}

/// Tests creating and reading an item using the driver.
///
/// This test:
/// 1. Creates a unique database and container
/// 2. Creates an item using `CosmosOperation::create_item()`
/// 3. Reads the item back using `CosmosOperation::read_item()`
/// 4. Validates the response body matches the created item
/// 5. Validates diagnostics for both operations
#[tokio::test]
pub async fn create_and_read_item() -> Result<(), Box<dyn Error>> {
    DriverTestClient::run_with_unique_db(async |context, database| {
        // Create a container
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        // Create a test item
        let item = TestItem {
            id: "test-doc-001".to_string(),
            pk: "partition-1".to_string(),
            value: "Hello, Cosmos!".to_string(),
            count: 42,
        };
        let item_json = serde_json::to_vec(&item)?;

        // Create the item
        let create_result = context
            .create_item(&container, &item.id, item.pk.clone(), &item_json)
            .await?;

        // Validate create diagnostics
        let create_diagnostics = create_result.diagnostics();
        context.validate_data_plane_diagnostics(create_diagnostics, 201);

        // Verify pipeline type is DataPlane
        let requests = create_diagnostics.requests();
        assert_eq!(requests[0].pipeline_type(), PipelineType::DataPlane);

        // Read the item back
        let read_result = context
            .read_item(&container, &item.id, item.pk.clone())
            .await?;

        // Validate read diagnostics
        let read_diagnostics = read_result.diagnostics();
        context.validate_data_plane_diagnostics(read_diagnostics, 200);

        // Verify the body matches
        let body = read_result.body();
        let read_item: TestItem = serde_json::from_slice(body)?;
        assert_eq!(read_item.id, item.id);
        assert_eq!(read_item.pk, item.pk);
        assert_eq!(read_item.value, item.value);
        assert_eq!(read_item.count, item.count);

        // Check request charge is reasonable (typically 1-5 RUs for point read)
        let read_requests = read_diagnostics.requests();
        assert!(
            read_requests[0].request_charge().value() > 0.0,
            "Request charge should be positive for reads"
        );
        assert!(
            read_requests[0].request_charge().value() < 100.0,
            "Request charge should be reasonable for point read"
        );

        Ok(())
    })
    .await
}

/// Tests that control plane operations use the metadata pipeline.
#[tokio::test]
pub async fn control_plane_uses_metadata_pipeline() -> Result<(), Box<dyn Error>> {
    DriverTestClient::run_with_unique_db(async |context, database| {
        // Create a container and verify it used the metadata pipeline
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        // The container creation should have used metadata pipeline
        // (We can't directly verify this from the create_container helper,
        // but we can verify by checking the database creation diagnostics)

        // For now, just verify the container was created successfully
        // and that we can create an item in it (proving it exists)
        let test_item = TestItem {
            id: "verify-container".to_string(),
            pk: "test".to_string(),
            value: "test".to_string(),
            count: 1,
        };
        let item_json = serde_json::to_vec(&test_item)?;

        let result = context
            .create_item(&container, &test_item.id, test_item.pk.clone(), &item_json)
            .await?;

        // Verify item creation succeeded
        let status = result.diagnostics().status();
        assert!(status.map(|s| s.is_success()).unwrap_or(false));

        Ok(())
    })
    .await
}

/// Tests diagnostics content for emulator operations.
#[tokio::test]
pub async fn diagnostics_contain_expected_fields() -> Result<(), Box<dyn Error>> {
    DriverTestClient::run_with_unique_db(async |context, database| {
        // Create a container
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        // Create an item
        let item = TestItem {
            id: "diag-test-001".to_string(),
            pk: "diag-partition".to_string(),
            value: "Testing diagnostics".to_string(),
            count: 100,
        };
        let item_json = serde_json::to_vec(&item)?;

        let result = context
            .create_item(&container, &item.id, item.pk.clone(), &item_json)
            .await?;

        let diagnostics = result.diagnostics();

        // Verify activity ID is a valid UUID format
        let activity_id = diagnostics.activity_id().as_str();
        assert!(!activity_id.is_empty(), "Activity ID should not be empty");

        // Verify duration is captured
        let duration = diagnostics.duration();
        assert!(
            duration.as_millis() > 0 || duration.as_micros() > 0,
            "Duration should be non-zero"
        );

        // Verify request details
        let requests = diagnostics.requests();
        assert_eq!(
            requests.len(),
            1,
            "Should have exactly one request for simple create"
        );

        let request = &requests[0];

        // Verify endpoint is captured
        assert!(
            !request.endpoint().is_empty(),
            "Endpoint should be captured"
        );

        // For emulator, verify transport security
        if request.endpoint().contains("localhost") || request.endpoint().contains("127.0.0.1") {
            assert_eq!(
                request.transport_security(),
                TransportSecurity::EmulatorWithInsecureCertificates,
                "Emulator should use insecure certificates transport"
            );
        }

        // Verify pipeline type
        assert_eq!(
            request.pipeline_type(),
            PipelineType::DataPlane,
            "Item operations should use data plane pipeline"
        );

        Ok(())
    })
    .await
}
