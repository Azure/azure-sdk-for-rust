// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::StatusCode;
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::{
    BlobContainerClient, BlobContainerClientGetPropertiesResultHeaders, BlobContainerClientOptions,
    LeaseState,
};
use azure_storage_blob_test::recorded_test_setup;
use std::error::Error;

#[recorded::test]
async fn test_create_container(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let (options, endpoint) = recorded_test_setup(recording).await;
    let container_name = recording
        .random_string::<17>(Some("container"))
        .to_ascii_lowercase();

    let container_client_options = BlobContainerClientOptions {
        client_options: options.clone(),
        ..Default::default()
    };

    // Act
    let container_client = BlobContainerClient::new(
        &endpoint,
        container_name,
        recording.credential(),
        Some(container_client_options),
    )?;

    // Assert
    container_client.create_container(None).await?;

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_get_container_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let (options, endpoint) = recorded_test_setup(recording).await;
    let container_name = recording
        .random_string::<17>(Some("container"))
        .to_ascii_lowercase();

    let container_client_options = BlobContainerClientOptions {
        client_options: options.clone(),
        ..Default::default()
    };

    // Act
    let container_client = BlobContainerClient::new(
        &endpoint,
        container_name,
        recording.credential(),
        Some(container_client_options),
    )?;

    // Container Doesn't Exists Scenario
    let response = container_client.get_container_properties(None).await;

    // Assert
    assert!(response.is_err());
    let error = response.unwrap_err().http_status();
    assert_eq!(StatusCode::NotFound, error.unwrap());

    // Container Exists Scenario
    container_client.create_container(None).await?;
    let container_properties = container_client.get_container_properties(None).await?;
    let lease_state = container_properties.lease_state()?;
    let has_immutability_policy = container_properties.has_immutability_policy()?;

    // Assert
    assert_eq!(LeaseState::Available, lease_state.unwrap());
    assert!(!has_immutability_policy.unwrap());

    container_client.delete_container(None).await?;
    Ok(())
}
