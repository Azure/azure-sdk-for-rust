// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{Bytes, RequestContent, StatusCode};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::models::BlobClientDownloadResultHeaders;
use azure_storage_blob::{
    clients::{BlobClient, BlobContainerClient},
    models::{BlobClientGetPropertiesResultHeaders, BlockListType, BlockLookupList, LeaseState},
    BlobClientOptions, BlobContainerClientOptions,
};
use azure_storage_blob_test::recorded_test_setup;
use std::error::Error;

#[recorded::test]
async fn test_get_blob_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    println!("Start tests");
    // Recording Setup
    let recording = ctx.recording();
    let (options, endpoint) = recorded_test_setup(recording).await;
    let container_name = recording
        .random_string::<17>(Some("container"))
        .to_ascii_lowercase();
    let blob_name = recording
        .random_string::<12>(Some("blob"))
        .to_ascii_lowercase();

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

    let blob_client_options = BlobClientOptions {
        client_options: options.clone(),
        ..Default::default()
    };
    let blob_client = BlobClient::new(
        &endpoint,
        container_name,
        blob_name,
        recording.credential(),
        Some(blob_client_options),
    )?;

    // Invalid Container Scenario
    let response = blob_client.get_blob_properties(None).await;

    // Assert
    let error = response.unwrap_err().http_status();
    assert_eq!(StatusCode::NotFound, error.unwrap());

    container_client.create_container(None).await?;

    let data = b"hello rusty world";

    blob_client
        .upload_blob(
            RequestContent::from(data.to_vec()),
            true,
            u64::try_from(data.len())?,
            None,
        )
        .await?;

    // No Option Scenario
    let response = blob_client.get_blob_properties(None).await?;

    // Assert
    let lease_state = response.lease_state()?;
    assert_eq!(LeaseState::Available, lease_state.unwrap());

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_upload_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let (options, endpoint) = recorded_test_setup(recording).await;
    let container_name = recording
        .random_string::<17>(Some("container"))
        .to_ascii_lowercase();
    let blob_name = recording
        .random_string::<12>(Some("blob"))
        .to_ascii_lowercase();

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

    let blob_client_options = BlobClientOptions {
        client_options: options.clone(),
        ..Default::default()
    };
    let blob_client = BlobClient::new(
        &endpoint,
        container_name,
        blob_name,
        recording.credential(),
        Some(blob_client_options),
    )?;

    let data = b"hello rusty world";

    // No Overwrite Scenario
    blob_client
        .upload_blob(
            RequestContent::from(data.to_vec()),
            false,
            u64::try_from(data.len())?,
            None,
        )
        .await?;

    // Assert
    let response = blob_client.download_blob(None).await?;
    let content_length = response.content_length()?;
    let (status_code, _, response_body) = response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(17, content_length.unwrap());
    assert_eq!(Bytes::from_static(data), response_body.collect().await?);

    // Overwrite Scenarios
    let new_data = b"hello overwritten rusty world";

    // Error Case (overwrite=false/none)
    let error_response = blob_client
        .upload_blob(
            RequestContent::from(new_data.to_vec()),
            false,
            u64::try_from(new_data.len())?,
            None,
        )
        .await;

    // Assert
    assert!(error_response.is_err());

    // Working Case (overwrite=true)
    let overwrite_response = blob_client
        .upload_blob(
            RequestContent::from(new_data.to_vec()),
            true,
            u64::try_from(new_data.len())?,
            None,
        )
        .await?;
    let response = blob_client.download_blob(None).await?;
    let content_length = response.content_length()?;

    // Assert
    assert_eq!(overwrite_response.status(), StatusCode::Created);
    let (status_code, _, response_body) = response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(29, content_length.unwrap());
    assert_eq!(Bytes::from_static(new_data), response_body.collect().await?);

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_download_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let (options, endpoint) = recorded_test_setup(recording).await;
    let container_name = recording
        .random_string::<17>(Some("container"))
        .to_ascii_lowercase();
    let blob_name = recording
        .random_string::<12>(Some("blob"))
        .to_ascii_lowercase();

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

    let blob_client_options = BlobClientOptions {
        client_options: options.clone(),
        ..Default::default()
    };
    let blob_client = BlobClient::new(
        &endpoint,
        container_name,
        blob_name,
        recording.credential(),
        Some(blob_client_options),
    )?;
    let data = b"test download content";
    blob_client
        .upload_blob(
            RequestContent::from(data.to_vec()),
            true,
            u64::try_from(data.len())?,
            None,
        )
        .await?;
    let response = blob_client.download_blob(None).await?;

    // Assert
    let content_length = response.content_length()?;
    let (status_code, _, response_body) = response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(21, content_length.unwrap());
    assert_eq!(Bytes::from_static(data), response_body.collect().await?);

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_put_block_list(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let (options, endpoint) = recorded_test_setup(recording).await;
    let container_name = recording
        .random_string::<17>(Some("container"))
        .to_ascii_lowercase();
    let blob_name = recording
        .random_string::<12>(Some("blob"))
        .to_ascii_lowercase();

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

    let blob_client_options = BlobClientOptions {
        client_options: options.clone(),
        ..Default::default()
    };
    let blob_client = BlobClient::new(
        &endpoint,
        container_name,
        blob_name,
        recording.credential(),
        Some(blob_client_options),
    )?;

    let block_1 = b"AAA";
    let block_2 = b"BBB";
    let block_3 = b"CCC";

    let block_1_id: Vec<u8> = b"1".to_vec();
    let block_2_id: Vec<u8> = b"2".to_vec();
    let block_3_id: Vec<u8> = b"3".to_vec();

    blob_client
        .stage_block(
            block_1_id.clone(),
            u64::try_from(block_1.len())?,
            RequestContent::from(block_1.to_vec()),
            None,
        )
        .await?;

    blob_client
        .stage_block(
            block_2_id.clone(),
            u64::try_from(block_2.len())?,
            RequestContent::from(block_2.to_vec()),
            None,
        )
        .await?;
    blob_client
        .stage_block(
            block_3_id.clone(),
            u64::try_from(block_3.len())?,
            RequestContent::from(block_3.to_vec()),
            None,
        )
        .await?;

    let latest_blocks: Vec<Vec<u8>> = vec![block_1_id, block_2_id, block_3_id];

    let block_lookup_list = BlockLookupList {
        committed: Vec::new(),
        latest: latest_blocks,
        uncommitted: Vec::new(),
    };

    let request_content = RequestContent::try_from(block_lookup_list)?;

    blob_client.commit_block_list(request_content, None).await?;

    let response = blob_client.download_blob(None).await?;

    // Assert
    let content_length = response.content_length()?;
    let (status_code, _, response_body) = response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(9, content_length.unwrap());
    assert_eq!(
        Bytes::from_static(b"AAABBBCCC"),
        response_body.collect().await?
    );

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_get_block_list(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let (options, endpoint) = recorded_test_setup(recording).await;
    let container_name = recording
        .random_string::<17>(Some("container"))
        .to_ascii_lowercase();
    let blob_name = recording
        .random_string::<12>(Some("blob"))
        .to_ascii_lowercase();

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

    let blob_client_options = BlobClientOptions {
        client_options: options.clone(),
        ..Default::default()
    };
    let blob_client = BlobClient::new(
        &endpoint,
        container_name,
        blob_name,
        recording.credential(),
        Some(blob_client_options),
    )?;

    let block_1 = b"AAA";
    let block_2 = b"BBB";
    let block_3 = b"CCC";

    let block_1_id: Vec<u8> = b"1".to_vec();
    let block_2_id: Vec<u8> = b"2".to_vec();
    let block_3_id: Vec<u8> = b"3".to_vec();

    blob_client
        .stage_block(
            block_1_id.clone(),
            u64::try_from(block_1.len())?,
            RequestContent::from(block_1.to_vec()),
            None,
        )
        .await?;

    blob_client
        .stage_block(
            block_2_id.clone(),
            u64::try_from(block_2.len())?,
            RequestContent::from(block_2.to_vec()),
            None,
        )
        .await?;
    blob_client
        .stage_block(
            block_3_id.clone(),
            u64::try_from(block_3.len())?,
            RequestContent::from(block_3.to_vec()),
            None,
        )
        .await?;

    // Three Staged Blocks Scenario
    let block_list = blob_client
        .get_block_list(BlockListType::All, None)
        .await?
        .into_body()
        .await?;

    // Assert
    assert_eq!(0, block_list.committed_blocks.len());
    assert_eq!(3, block_list.uncommitted_blocks.len());

    let latest_blocks: Vec<Vec<u8>> = vec![block_1_id, block_2_id, block_3_id];

    let block_lookup_list = BlockLookupList {
        committed: Vec::new(),
        latest: latest_blocks,
        uncommitted: Vec::new(),
    };

    let request_content = RequestContent::try_from(block_lookup_list)?;

    blob_client.commit_block_list(request_content, None).await?;

    // Three Committed Blocks Scenario
    let block_list = blob_client
        .get_block_list(BlockListType::All, None)
        .await?
        .into_body()
        .await?;

    // Assert
    assert_eq!(3, block_list.committed_blocks.len());
    assert_eq!(0, block_list.uncommitted_blocks.len());

    container_client.delete_container(None).await?;
    Ok(())
}
