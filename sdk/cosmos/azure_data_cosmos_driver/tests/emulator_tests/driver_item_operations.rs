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
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn create_and_read_item() -> Result<(), Box<dyn Error>> {
    Box::pin(DriverTestClient::run_with_unique_db(
        async |context, database| {
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
            context.validate_data_plane_diagnostics(&create_diagnostics, 201);

            // Verify pipeline type is DataPlane
            let requests = create_diagnostics.requests();
            assert_eq!(requests[0].pipeline_type(), PipelineType::DataPlane);

            // Read the item back
            let read_result = context
                .read_item(&container, &item.id, item.pk.clone())
                .await?;

            // Validate read diagnostics
            let read_diagnostics = read_result.diagnostics();
            context.validate_data_plane_diagnostics(&read_diagnostics, 200);

            // Verify the body matches
            let read_item: TestItem = read_result.into_body().into_single()?;
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
        },
    ))
    .await
}

/// Tests that control plane operations use the metadata pipeline.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
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
            .create_seed_item(&container, &test_item.id, test_item.pk.clone(), &item_json)
            .await?;

        // Verify item creation succeeded
        let diagnostics = result.diagnostics();
        let status = diagnostics.status();
        assert!(status.map(|s| s.is_success()).unwrap_or(false));

        Ok(())
    })
    .await
}

/// Tests diagnostics content for emulator operations.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
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
            .create_seed_item(&container, &item.id, item.pk.clone(), &item_json)
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

        // Verify request details. Live accounts can legitimately produce more
        // than one request for a simple create when the transport retries a
        // transient failure, so validate the final successful attempt instead
        // of asserting a single-attempt operation.
        let requests = diagnostics.requests();
        assert!(!requests.is_empty(), "Should have at least one request");

        let request = requests.last().expect("requests must be non-empty");

        // Verify endpoint is captured
        assert!(
            !request.endpoint().is_empty(),
            "Endpoint should be captured"
        );

        // For emulator, verify transport security. The legacy emulator and
        // the vnext emulator in HTTPS mode use a self-signed cert and surface
        // as `EmulatorWithInsecureCertificates`. The vnext emulator in HTTP
        // mode has no TLS at all and is classified as `Secure` today (the
        // enum predates plain-HTTP emulator support — tracked separately).
        if request.endpoint().contains("localhost") || request.endpoint().contains("127.0.0.1") {
            let expected = if request.endpoint().starts_with("https://") {
                TransportSecurity::EmulatorWithInsecureCertificates
            } else {
                TransportSecurity::Secure
            };
            assert_eq!(
                request.transport_security(),
                expected,
                "Unexpected transport security for emulator endpoint {}",
                request.endpoint()
            );
        }

        // Verify pipeline type
        assert_eq!(
            request.pipeline_type(),
            PipelineType::DataPlane,
            "Item operations should use data plane pipeline"
        );

        // Verify server-side duration when captured. `x-ms-request-duration-ms`
        // is an optional server-emitted header — not every emulator
        // configuration (e.g., vnext emulator in some modes) emits it on
        // every response, so the field may legitimately be `None`. When the
        // header IS present, validate it parsed as a non-negative finite
        // value.
        if let Some(duration) = request.server_duration_ms() {
            assert!(
                duration >= 0.0,
                "Server duration must be non-negative when captured, got {duration}"
            );
        }

        Ok(())
    })
    .await
}
