// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::StatusCode;
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::{
    clients::BlobContainerClient,
    models::{BlobContainerClientGetPropertiesResultHeaders, LeaseState},
    BlobContainerClientOptions,
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

    // This is to shove the modified pipeline policy into the container client options
    let container_client_options = BlobContainerClientOptions {
        client_options: options.clone(),
        ..Default::default()
    };
    // Act
    let container_client = BlobContainerClient::new(
        &endpoint,
        container_name.clone(),
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

    // This is to shove the modified pipeline policy into the container client options
    let container_client_options = BlobContainerClientOptions {
        client_options: options.clone(),
        ..Default::default()
    };
    // Act
    let container_client = BlobContainerClient::new(
        &endpoint,
        container_name.clone(),
        recording.credential(),
        Some(container_client_options),
    )?;
    container_client.create_container(None).await?;
    let response = container_client.get_container_properties(None).await?;

    // Assert

    let lease_state = response.lease_state()?;
    let immutability_policy = response.has_immutability_policy()?;
    assert_eq!(lease_state, Some(LeaseState::Available));
    assert_eq!(immutability_policy, Some(false));

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_get_container_properties_invalid_container(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let (options, endpoint) = recorded_test_setup(recording).await;
    let container_name = recording
        .random_string::<17>(Some("container"))
        .to_ascii_lowercase();

    // This is to shove the modified pipeline policy into the container client options
    let container_client_options = BlobContainerClientOptions {
        client_options: options.clone(),
        ..Default::default()
    };
    // Act
    let container_client = BlobContainerClient::new(
        &endpoint,
        container_name.clone(),
        recording.credential(),
        Some(container_client_options),
    )?;
    let response = container_client.get_container_properties(None).await;

    // Assert
    assert!(response.is_err());
    let error = response.unwrap_err().http_status();
    assert_eq!(Some(StatusCode::NotFound), error);

    Ok(())
}
