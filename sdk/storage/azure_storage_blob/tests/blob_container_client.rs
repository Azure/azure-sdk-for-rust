// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::StatusCode;
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::models::{
    BlobContainerClientGetPropertiesResultHeaders, BlobContainerClientListBlobFlatSegmentOptions,
    BlobContainerClientSetMetadataOptions, BlobType, LeaseState,
};
use azure_storage_blob_test::{create_test_blob, get_container_client};
use futures::TryStreamExt;
use std::{collections::HashMap, error::Error};

#[recorded::test]
async fn test_create_container(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, false).await?;

    container_client.create_container(None).await?;

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_get_container_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, false).await?;

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
async fn test_set_container_metadata(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true).await?;

    // Set Metadata With Values
    let update_metadata = HashMap::from([("hello".to_string(), "world".to_string())]);
    let set_metadata_options = BlobContainerClientSetMetadataOptions {
        metadata: Some(update_metadata.clone()),
        ..Default::default()
    };
    container_client
        .set_metadata(Some(set_metadata_options))
        .await?;

    // Assert
    let response = container_client.get_properties(None).await?;
    let response_metadata = response.metadata()?;
    assert_eq!(update_metadata, response_metadata);

    // Set Metadata No Values (Clear Metadata)
    container_client.set_metadata(None).await?;

    // Assert
    let response = container_client.get_properties(None).await?;
    let response_metadata = response.metadata()?;
    assert_eq!(HashMap::new(), response_metadata);

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, false).await?;
    let blob_names = ["testblob1".to_string(), "testblob2".to_string()];

    container_client.create_container(None).await?;
    create_test_blob(&container_client.blob_client(blob_names[0].clone())).await?;
    create_test_blob(&container_client.blob_client(blob_names[1].clone())).await?;

    let mut list_blobs_response = container_client.list_blobs(None).await?;

    let page = list_blobs_response.try_next().await?;
    let list_blob_segment_response = page.unwrap().into_body().await?;
    let blob_list = list_blob_segment_response.segment.blob_items;
    for blob in blob_list {
        let blob_name = blob.name.unwrap().content.unwrap();
        let blob_type = blob.properties.unwrap().blob_type.unwrap();
        assert!(blob_names.contains(&blob_name));
        assert_eq!(BlobType::BlockBlob, blob_type);
    }

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs_with_continuation(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, false).await?;
    let blob_names = [
        "testblob1".to_string(),
        "testblob2".to_string(),
        "testblob3".to_string(),
        "testblob4".to_string(),
    ];

    container_client.create_container(None).await?;
    create_test_blob(&container_client.blob_client(blob_names[0].clone())).await?;
    create_test_blob(&container_client.blob_client(blob_names[1].clone())).await?;
    create_test_blob(&container_client.blob_client(blob_names[2].clone())).await?;
    create_test_blob(&container_client.blob_client(blob_names[3].clone())).await?;

    // First Page
    let list_blobs_options = BlobContainerClientListBlobFlatSegmentOptions {
        maxresults: Some(2),
        ..Default::default()
    };
    let mut list_blobs_response = container_client
        .list_blobs(Some(list_blobs_options))
        .await?;
    let first_page = list_blobs_response.try_next().await?;
    let list_blob_segment_response = first_page.unwrap().into_body().await?;
    let continuation_token = list_blob_segment_response.next_marker;
    let blob_list = list_blob_segment_response.segment.blob_items;
    assert_eq!(2, blob_list.len());
    for blob in blob_list {
        let blob_name = blob.name.unwrap().content.unwrap();
        println!("1st page: {}", blob_name.clone());
        let blob_type = blob.properties.unwrap().blob_type.unwrap();
        assert!(blob_names.contains(&blob_name));
        assert_eq!(BlobType::BlockBlob, blob_type);
    }

    // Second Page
    let list_blobs_options = BlobContainerClientListBlobFlatSegmentOptions {
        marker: continuation_token,
        ..Default::default()
    };
    let mut list_blobs_response = container_client
        .list_blobs(Some(list_blobs_options))
        .await?;
    let second_page = list_blobs_response.try_next().await?;
    let list_blob_segment_response = second_page.unwrap().into_body().await?;
    let blob_list = list_blob_segment_response.segment.blob_items;
    assert_eq!(2, blob_list.len());
    for blob in blob_list {
        let blob_name = blob.name.unwrap().content.unwrap();
        println!("2nd page: {}", blob_name.clone());
        let blob_type = blob.properties.unwrap().blob_type.unwrap();
        assert!(blob_names.contains(&blob_name));
        assert_eq!(BlobType::BlockBlob, blob_type);
    }

    container_client.delete_container(None).await?;
    Ok(())
}
