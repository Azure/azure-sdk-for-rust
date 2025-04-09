// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::{RequestContent, StatusCode};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::{
    models::{BlobContainerClientGetPropertiesResultHeaders, LeaseState},
    BlobClient, BlobClientOptions, BlobContainerClient, BlobContainerClientOptions,
};
use azure_storage_blob_test::recorded_test_setup;
use std::error::Error;

#[recorded::test]
async fn test_create_container(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let (options, endpoint) = recorded_test_setup(recording);
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
    let (options, endpoint) = recorded_test_setup(recording);
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
    let response = container_client.get_properties(None).await;

    // Assert
    assert!(response.is_err());
    let error = response.unwrap_err().http_status();
    assert_eq!(StatusCode::NotFound, error.unwrap());

    // Container Exists Scenario
    container_client.create_container(None).await?;
    let container_properties = container_client.get_properties(None).await?;
    let lease_state = container_properties.lease_state()?;
    let has_immutability_policy = container_properties.has_immutability_policy()?;

    // Assert
    assert_eq!(LeaseState::Available, lease_state.unwrap());
    assert!(!has_immutability_policy.unwrap());

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let (options, endpoint) = recorded_test_setup(recording);
    let container_name = recording
        .random_string::<17>(Some("container"))
        .to_ascii_lowercase();
    let blob_name_1 = recording
        .random_string::<12>(Some("blob"))
        .to_ascii_lowercase();
    let blob_name_2 = recording
        .random_string::<12>(Some("blob"))
        .to_ascii_lowercase();

    let container_client_options = BlobContainerClientOptions {
        client_options: options.clone(),
        ..Default::default()
    };
    let container_client = BlobContainerClient::new(
        &endpoint,
        container_name.clone(),
        recording.credential(),
        Some(container_client_options),
    )?;
    container_client.create_container(None).await?;
    let blob_client_options = BlobClientOptions {
        client_options: options.clone(),
        ..Default::default()
    };
    let blob_client_1 = BlobClient::new(
        &endpoint,
        container_name.clone(),
        blob_name_1,
        recording.credential(),
        Some(blob_client_options.clone()),
    )?;
    let blob_client_2 = BlobClient::new(
        &endpoint,
        container_name,
        blob_name_2,
        recording.credential(),
        Some(blob_client_options),
    )?;
    let data = b"hello world";
    blob_client_1
        .upload(
            RequestContent::from(data.to_vec()),
            true,
            u64::try_from(data.len())?,
            None,
        )
        .await?;
    blob_client_2
        .upload(
            RequestContent::from(data.to_vec()),
            true,
            u64::try_from(data.len())?,
            None,
        )
        .await?;

    // Act
    let list_blob_response = container_client.list_blobs(None).await?.into_body().await?;

    println!("{}", list_blob_response.container_name.is_none()); // False
    println!("{}", list_blob_response.marker.is_none()); // True
    println!("{}", list_blob_response.max_results.is_none()); // True
    println!("{}", list_blob_response.next_marker.is_none()); // False
    println!("{}", list_blob_response.prefix.is_none()); // True
    println!("{}", list_blob_response.segment.is_none()); // True
    println!("{}", list_blob_response.service_endpoint.is_none()); // False

    // Assert

    Ok(())
}
