// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    http::{RequestContent, StatusCode},
    Bytes,
};
use azure_core_test::{recorded, Matcher, TestContext};
use azure_storage_blob::models::{
    AccessTier, AccountKind, BlobClientAcquireLeaseResultHeaders,
    BlobClientChangeLeaseResultHeaders, BlobClientDownloadOptions, BlobClientDownloadResultHeaders,
    BlobClientGetAccountInfoResultHeaders, BlobClientGetPropertiesOptions,
    BlobClientGetPropertiesResultHeaders, BlobClientSetMetadataOptions,
    BlobClientSetPropertiesOptions, BlobClientSetTierOptions, BlockBlobClientUploadOptions,
    LeaseState,
};
use futures::TryStreamExt;

use azure_storage_blob_test::{create_test_blob, get_blob_name, get_container_client};
use std::{collections::HashMap, error::Error, time::Duration};
use tokio::time;

#[recorded::test]
async fn test_get_blob_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, false).await?;
    let blob_client = container_client.blob_client(get_blob_name(recording));

    // Container Doesn't Exist Scenario
    let response = blob_client.get_properties(None).await;

    // Assert
    let error = response.unwrap_err().http_status();
    assert_eq!(StatusCode::NotFound, error.unwrap());
    assert!(!blob_client.exists().await?);

    container_client.create_container(None).await?;
    assert!(!blob_client.exists().await?);
    create_test_blob(&blob_client, None, None).await?;

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
    assert!(blob_client.exists().await?);

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_set_blob_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true).await?;
    let blob_client = container_client.blob_client(get_blob_name(recording));
    create_test_blob(&blob_client, None, None).await?;

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
    let container_client = get_container_client(recording, true).await?;
    let blob_client = container_client.blob_client(get_blob_name(recording));

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
    assert_eq!(
        Bytes::from_static(data),
        response_body.collect().await?.as_ref()
    );

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
    assert_eq!(
        Bytes::from_static(new_data),
        response_body.collect().await?.as_ref()
    );

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_delete_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true).await?;
    let blob_client = container_client.blob_client(get_blob_name(recording));
    create_test_blob(&blob_client, None, None).await?;

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
    let container_client = get_container_client(recording, true).await?;
    let blob_client = container_client.blob_client(get_blob_name(recording));
    let data = b"hello rusty world";

    blob_client
        .upload(
            RequestContent::from(data.to_vec()),
            false,
            u64::try_from(data.len())?,
            None,
        )
        .await?;
    let response = blob_client.download(None).await?;

    // Assert
    let content_length = response.content_length()?;
    let (status_code, _, response_body) = response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(17, content_length.unwrap());
    assert_eq!(
        b"hello rusty world".to_vec(),
        response_body.collect().await?.to_vec(),
    );

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_set_blob_metadata(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup

    let recording = ctx.recording();
    let container_client = get_container_client(recording, true).await?;
    let blob_client = container_client.blob_client(get_blob_name(recording));
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
    blob_client
        .set_metadata(update_metadata.clone(), None)
        .await?;
    // Assert
    let response = blob_client.get_properties(None).await?;
    let response_metadata = response.metadata()?;
    assert_eq!(update_metadata, response_metadata);

    // Set Metadata No Values (Clear Metadata)
    blob_client.set_metadata(HashMap::new(), None).await?;
    // Assert
    let response = blob_client.get_properties(None).await?;
    let response_metadata = response.metadata()?;
    assert_eq!(HashMap::new(), response_metadata);

    Ok(())
}

#[recorded::test]
async fn test_set_access_tier(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true).await?;
    let blob_client = container_client.blob_client(get_blob_name(recording));
    create_test_blob(&blob_client, None, None).await?;

    let original_response = blob_client.get_properties(None).await?;
    let og_access_tier = original_response.access_tier()?;
    assert_eq!(AccessTier::Hot.to_string(), og_access_tier.unwrap());

    // Set Standard Blob Tier (Cold)
    blob_client.set_tier(AccessTier::Cold, None).await?;
    let response = blob_client.get_properties(None).await?;

    // Assert
    let access_tier = response.access_tier()?;
    assert_eq!(AccessTier::Cold.to_string(), access_tier.unwrap());

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_blob_lease_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true).await?;
    let blob_name = get_blob_name(recording);
    let blob_client = container_client.blob_client(blob_name.clone());
    let other_blob_client = container_client.blob_client(blob_name);
    create_test_blob(&blob_client, None, None).await?;

    // Acquire Lease
    let acquire_response = blob_client.acquire_lease(15, None).await?;
    let lease_id = acquire_response.lease_id()?.unwrap();
    let other_acquire_response = other_blob_client.acquire_lease(15, None).await;
    // Assert
    let error = other_acquire_response.unwrap_err().http_status();
    assert_eq!(StatusCode::Conflict, error.unwrap());

    // Change Lease
    let proposed_lease_id = "00000000-1111-2222-3333-444444444444".to_string();
    let change_lease_response = blob_client
        .change_lease(lease_id, proposed_lease_id.clone(), None)
        .await?;
    // Assert
    let lease_id = change_lease_response.lease_id()?.unwrap();
    assert_eq!(proposed_lease_id.clone().to_string(), lease_id);

    // Sleep until lease expires
    time::sleep(Duration::from_secs(15)).await;

    // Renew Lease
    blob_client
        .renew_lease(proposed_lease_id.clone(), None)
        .await?;
    let other_acquire_response = other_blob_client.acquire_lease(15, None).await;
    // Assert
    let error = other_acquire_response.unwrap_err().http_status();
    assert_eq!(StatusCode::Conflict, error.unwrap());

    // Break Lease
    blob_client.break_lease(None).await?;
    let other_acquire_response = other_blob_client.acquire_lease(15, None).await;
    // Assert
    let error = other_acquire_response.unwrap_err().http_status();
    assert_eq!(StatusCode::Conflict, error.unwrap());

    // Release Lease
    blob_client
        .release_lease(proposed_lease_id.clone(), None)
        .await?;
    other_blob_client.acquire_lease(15, None).await?;

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_leased_blob_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true).await?;
    let blob_name = get_blob_name(recording);
    let blob_client = container_client.blob_client(blob_name.clone());
    create_test_blob(&blob_client, None, None).await?;
    let acquire_response = blob_client.acquire_lease(-1, None).await?;
    let lease_id = acquire_response.lease_id()?.unwrap();

    // Set Properties, Set Metadata, Set Access Tier
    let set_properties_options = BlobClientSetPropertiesOptions {
        blob_content_language: Some("spanish".to_string()),
        blob_content_disposition: Some("inline".to_string()),
        lease_id: Some(lease_id.clone()),
        ..Default::default()
    };
    blob_client
        .set_properties(Some(set_properties_options))
        .await?;

    let update_metadata = HashMap::from([("updated".to_string(), "values".to_string())]);
    let set_metadata_options = BlobClientSetMetadataOptions {
        lease_id: Some(lease_id.clone()),
        ..Default::default()
    };
    blob_client
        .set_metadata(update_metadata.clone(), Some(set_metadata_options))
        .await?;

    let set_tier_options = BlobClientSetTierOptions {
        lease_id: Some(lease_id.clone()),
        ..Default::default()
    };
    blob_client
        .set_tier(AccessTier::Cold, Some(set_tier_options))
        .await?;

    // Assert
    let get_properties_options = BlobClientGetPropertiesOptions {
        lease_id: Some(lease_id.clone()),
        ..Default::default()
    };
    let response = blob_client
        .get_properties(Some(get_properties_options))
        .await?;
    let content_language = response.content_language()?;
    let content_disposition = response.content_disposition()?;
    let response_metadata = response.metadata()?;
    let access_tier = response.access_tier()?;

    assert_eq!("spanish".to_string(), content_language.unwrap());
    assert_eq!("inline".to_string(), content_disposition.unwrap());
    assert_eq!(update_metadata, response_metadata);
    assert_eq!(AccessTier::Cold.to_string(), access_tier.unwrap());

    // Overwrite Upload
    let data = b"overruled!";
    let upload_options = BlockBlobClientUploadOptions {
        lease_id: Some(lease_id.clone()),
        ..Default::default()
    };
    blob_client
        .upload(
            RequestContent::from(data.to_vec()),
            true,
            u64::try_from(data.len())?,
            Some(upload_options),
        )
        .await?;

    // Assert
    let download_options = BlobClientDownloadOptions {
        lease_id: Some(lease_id.clone()),
        ..Default::default()
    };
    let response = blob_client.download(Some(download_options)).await?;
    let content_length = response.content_length()?;
    let (status_code, _, response_body) = response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(10, content_length.unwrap());
    assert_eq!(data.to_vec(), response_body.collect().await?.to_vec());

    blob_client.break_lease(None).await?;
    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_blob_tags(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    recording.set_matcher(Matcher::BodilessMatcher).await?;
    let container_client = get_container_client(recording, true).await?;
    let blob_client = container_client.blob_client(get_blob_name(recording));
    create_test_blob(&blob_client, None, None).await?;

    // Set Tags with Tags Specified
    let blob_tags = HashMap::from([
        ("hello".to_string(), "world".to_string()),
        ("ferris".to_string(), "crab".to_string()),
    ]);
    blob_client.set_tags(blob_tags.clone(), None).await?;

    // Assert
    let response_tags = blob_client.get_tags(None).await?.into_body()?;
    let map: HashMap<String, String> = response_tags.try_into()?;
    assert_eq!(blob_tags, map);

    // Set Tags with No Tags (Clear Tags)
    blob_client.set_tags(HashMap::new(), None).await?;

    // Assert
    let response_tags = blob_client.get_tags(None).await?.into_body()?;
    let map: HashMap<String, String> = response_tags.try_into()?;
    assert_eq!(HashMap::new(), map);

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_get_account_info(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true).await?;
    let blob_client = container_client.blob_client(get_blob_name(recording));

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
async fn test_blob_name_with_leading_spaces(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true).await?;

    // Create a blob with leading spaces
    let blob_name_with_spaces = "  with leading spaces".to_string();
    let blob_client = container_client.blob_client(blob_name_with_spaces.clone());

    // Create the blob using helper function
    create_test_blob(&blob_client, None, None).await?;

    // Use list_blobs API to retrieve the blob name
    let mut list_blobs_response = container_client.list_blobs(None)?;

    // Get the first page of results
    let page = list_blobs_response.try_next().await?;
    let list_blob_segment_response = page.unwrap().into_body()?;
    let blob_list = list_blob_segment_response.segment.blob_items;

    // Assert that we found exactly one blob
    assert_eq!(1, blob_list.len());

    // Get the returned blob name
    let returned_blob_name = blob_list[0]
        .name
        .as_ref()
        .unwrap()
        .content
        .as_ref()
        .unwrap();

    // Assert that the returned blob name matches the original (with spaces preserved)
    assert_eq!(
        blob_name_with_spaces, *returned_blob_name,
        "Blob name with leading spaces should be preserved. Expected: '{}', Got: '{}'",
        blob_name_with_spaces, returned_blob_name
    );

    container_client.delete_container(None).await?;
    Ok(())
}
