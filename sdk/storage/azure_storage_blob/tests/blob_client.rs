// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    http::{RequestContent, StatusCode},
    Bytes,
};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::{
    models::{
        AccessTier, AccountKind, BlobClientDownloadResultHeaders,
        BlobClientGetAccountInfoResultHeaders, BlobClientGetPropertiesResultHeaders,
        BlobImmutabilityPolicyMode, BlockListType, BlockLookupList, LeaseState,
    },
    BlobClientSetImmutabilityPolicyOptions, BlobClientSetMetadataOptions,
    BlobClientSetPropertiesOptions, BlockBlobClientUploadOptions,
};
use azure_storage_blob_test::{create_test_blob, get_blob_client, get_container_client};
use std::{collections::HashMap, error::Error};
use time::OffsetDateTime;

#[recorded::test]
async fn test_get_blob_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording)?;
    let blob_client = get_blob_client(
        Some(container_client.container_name().to_string()),
        recording,
    )?;

    // Invalid Container Scenario
    let response = blob_client.get_properties(None).await;

    // Assert
    let error = response.unwrap_err().http_status();
    assert_eq!(StatusCode::NotFound, error.unwrap());

    container_client.create_container(None).await?;
    create_test_blob(&blob_client).await?;

    // No Option Scenario
    let response = blob_client.get_properties(None).await?;

    // Assert
    let lease_state = response.lease_state()?;
    let content_length = response.content_length()?;
    let etag = response.etag()?;
    let creation_time = response.creation_time()?;

    assert_eq!(LeaseState::Available, lease_state.unwrap());
    assert_eq!(17, content_length.unwrap());
    assert!(etag.is_some());
    assert!(creation_time.is_some());

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_set_blob_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording)?;
    let blob_client = get_blob_client(
        Some(container_client.container_name().to_string()),
        recording,
    )?;
    container_client.create_container(None).await?;
    create_test_blob(&blob_client).await?;

    // Set Content Settings
    let set_properties_options = BlobClientSetPropertiesOptions {
        blob_content_language: Some("spanish".to_string()),
        blob_content_disposition: Some("inline".to_string()),
        ..Default::default()
    };
    blob_client
        .set_properties(Some(set_properties_options))
        .await?;

    // Assert
    let response = blob_client.get_properties(None).await?;
    let content_language = response.content_language()?;
    let content_disposition = response.content_disposition()?;

    assert_eq!("spanish".to_string(), content_language.unwrap());
    assert_eq!("inline".to_string(), content_disposition.unwrap());

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_upload_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup

    let recording = ctx.recording();
    let container_client = get_container_client(recording)?;
    let blob_client = get_blob_client(
        Some(container_client.container_name().to_string()),
        recording,
    )?;
    container_client.create_container(None).await?;

    let data = b"hello rusty world";

    // No Overwrite Scenario
    blob_client
        .upload(
            RequestContent::from(data.to_vec()),
            false,
            u64::try_from(data.len())?,
            None,
        )
        .await?;

    // Assert
    let response = blob_client.download(None).await?;
    let content_length = response.content_length()?;
    let (status_code, _, response_body) = response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(17, content_length.unwrap());
    assert_eq!(Bytes::from_static(data), response_body.collect().await?);

    // Overwrite Scenarios
    let new_data = b"hello overwritten rusty world";

    // Error Case (overwrite=false/none)
    let response = blob_client
        .upload(
            RequestContent::from(new_data.to_vec()),
            false,
            u64::try_from(new_data.len())?,
            None,
        )
        .await;

    // Assert
    assert!(response.is_err());
    let error = response.unwrap_err().http_status();
    assert_eq!(StatusCode::Conflict, error.unwrap());

    // Working Case (overwrite=true)
    let overwrite_response = blob_client
        .upload(
            RequestContent::from(new_data.to_vec()),
            true,
            u64::try_from(new_data.len())?,
            None,
        )
        .await?;
    let response = blob_client.download(None).await?;
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
async fn test_delete_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup

    let recording = ctx.recording();
    let container_client = get_container_client(recording)?;
    let blob_client = get_blob_client(
        Some(container_client.container_name().to_string()),
        recording,
    )?;
    container_client.create_container(None).await?;
    create_test_blob(&blob_client).await?;

    // Existence Check
    blob_client.get_properties(None).await?;

    blob_client.delete(None).await?;

    let response = blob_client.download(None).await;

    // Assert
    let error = response.unwrap_err().http_status();
    assert_eq!(StatusCode::NotFound, error.unwrap());

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_download_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording)?;
    let blob_client = get_blob_client(
        Some(container_client.container_name().to_string()),
        recording,
    )?;
    container_client.create_container(None).await?;
    create_test_blob(&blob_client).await?;

    let response = blob_client.download(None).await?;

    // Assert
    let content_length = response.content_length()?;
    let (status_code, _, response_body) = response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(17, content_length.unwrap());
    assert_eq!(
        b"hello rusty world".to_vec(),
        response_body.collect().await?
    );

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_set_blob_metadata(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup

    let recording = ctx.recording();
    let container_client = get_container_client(recording)?;
    let blob_client = get_blob_client(
        Some(container_client.container_name().to_string()),
        recording,
    )?;
    container_client.create_container(None).await?;
    let data = b"hello rusty world";

    // Upload Blob With Metadata
    let initial_metadata = HashMap::from([("initial".to_string(), "metadata".to_string())]);

    let options_with_metadata = BlockBlobClientUploadOptions {
        metadata: Some(initial_metadata.clone()),
        ..Default::default()
    };
    blob_client
        .upload(
            RequestContent::from(data.to_vec()),
            false,
            u64::try_from(data.len())?,
            Some(options_with_metadata),
        )
        .await?;
    // Assert
    let response = blob_client.get_properties(None).await?;
    let response_metadata = response.metadata()?;
    assert_eq!(initial_metadata, response_metadata);

    // Set Metadata With Values
    let update_metadata = HashMap::from([("updated".to_string(), "values".to_string())]);
    let set_metadata_options = BlobClientSetMetadataOptions {
        metadata: Some(update_metadata.clone()),
        ..Default::default()
    };
    blob_client.set_metadata(Some(set_metadata_options)).await?;
    // Assert
    let response = blob_client.get_properties(None).await?;
    let response_metadata = response.metadata()?;
    assert_eq!(update_metadata, response_metadata);

    // Set Metadata No Values (Clear Metadata)
    blob_client.set_metadata(None).await?;
    // Assert
    let response = blob_client.get_properties(None).await?;
    let response_metadata = response.metadata()?;
    assert_eq!(HashMap::new(), response_metadata);

    Ok(())
}

#[recorded::test]
async fn test_put_block_list(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording)?;
    let blob_client = get_blob_client(
        Some(container_client.container_name().to_string()),
        recording,
    )?;
    container_client.create_container(None).await?;

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

    let response = blob_client.download(None).await?;

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
    let container_client = get_container_client(recording)?;
    let blob_client = get_blob_client(
        Some(container_client.container_name().to_string()),
        recording,
    )?;
    container_client.create_container(None).await?;

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

#[recorded::test]
async fn test_set_access_tier(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording)?;
    let blob_client = get_blob_client(
        Some(container_client.container_name().to_string()),
        recording,
    )?;
    container_client.create_container(None).await?;
    create_test_blob(&blob_client).await?;

    let original_response = blob_client.get_properties(None).await?;
    let og_access_tier = original_response.tier()?;
    assert_eq!(AccessTier::Hot, og_access_tier.unwrap());

    // Set Standard Blob Tier (Cold)
    blob_client.set_tier(AccessTier::Cold, None).await?;
    let response = blob_client.get_properties(None).await?;

    // Assert
    let access_tier = response.tier()?;
    assert_eq!(AccessTier::Cold, access_tier.unwrap());

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_get_account_info(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let blob_client = get_blob_client(None, recording)?;

    // Act
    let response = blob_client.get_account_info(None).await?;

    // Assert
    let sku_name = response.sku_name()?;
    let account_kind = response.account_kind()?;

    assert!(sku_name.is_some());
    assert_eq!(AccountKind::StorageV2, account_kind.unwrap());

    Ok(())
}

#[recorded::test]
async fn test_immutability_policy(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // TODO: Support versioning, will also need some way to indicate this needs to be versioned
    // TODO: In Python, we use the management client to enable ImmutableStorageWithVersioning

    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording)?;
    let blob_client = get_blob_client(
        Some(container_client.container_name().to_string()),
        recording,
    )?;
    container_client.create_container(None).await?;
    create_test_blob(&blob_client).await?;

    // Act
    let response = blob_client.get_properties(None).await?;
    let immutability_policy = response.immutability_policy_mode()?;
    assert!(immutability_policy.is_none());

    // Set Immutability Policy
    let set_immutability_policy_options = BlobClientSetImmutabilityPolicyOptions {
        immutability_policy_expiry: Some(
            OffsetDateTime::now_utc().saturating_add(time::Duration::minutes(5)),
        ),
        immutability_policy_mode: Some(BlobImmutabilityPolicyMode::Unlocked),
        ..Default::default()
    };
    blob_client
        .set_immutability_policy(Some(set_immutability_policy_options))
        .await?;

    let response = blob_client.get_properties(None).await?;
    // Assert
    let immutability_policy = response.immutability_policy_mode()?;
    assert_eq!(
        BlobImmutabilityPolicyMode::Mutable,
        immutability_policy.unwrap()
    );

    // Delete Immutability Policy
    blob_client.delete_immutability_policy(None).await?;

    let response = blob_client.get_properties(None).await?;
    // Assert
    let immutability_policy = response.immutability_policy_mode()?;
    assert!(immutability_policy.is_none());

    Ok(())
}
