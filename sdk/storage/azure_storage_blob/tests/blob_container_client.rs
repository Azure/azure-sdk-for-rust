// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    http::StatusCode,
    time::{Duration, OffsetDateTime},
};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::models::{
    AccessPolicy, AccountKind, BlobContainerClientAcquireLeaseResultHeaders,
    BlobContainerClientChangeLeaseResultHeaders, BlobContainerClientGetAccountInfoResultHeaders,
    BlobContainerClientGetPropertiesResultHeaders, BlobContainerClientListBlobFlatSegmentOptions,
    BlobContainerClientSetMetadataOptions, BlobType, LeaseState, SignedIdentifier,
};
use azure_storage_blob_test::{
    create_test_blob, get_blob_service_client, get_container_client, get_container_name,
};
use futures::{StreamExt, TryStreamExt};
use std::{collections::HashMap, error::Error};
use tokio::time;

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
    container_client
        .set_metadata(update_metadata.clone(), None)
        .await?;

    // Assert
    let response = container_client.get_properties(None).await?;
    let response_metadata = response.metadata()?;
    assert_eq!(update_metadata, response_metadata);

    // Set Metadata No Values (Clear Metadata)
    container_client.set_metadata(HashMap::new(), None).await?;

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
    create_test_blob(&container_client.blob_client(blob_names[0].clone()), None).await?;
    create_test_blob(&container_client.blob_client(blob_names[1].clone()), None).await?;

    let mut list_blobs_response = container_client.list_blobs(None)?;

    let page = list_blobs_response.try_next().await?;
    let list_blob_segment_response = page.unwrap().into_body().await?;
    let blob_list = list_blob_segment_response.segment.blob_items;
    for blob in blob_list {
        let blob_name = blob.name.unwrap().content.unwrap();
        let properties = blob.properties.unwrap();
        let blob_type = properties.blob_type.unwrap();
        let etag = properties.etag;
        assert!(blob_names.contains(&blob_name));
        assert_eq!(BlobType::BlockBlob, blob_type);
        assert!(etag.is_some());
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
    create_test_blob(&container_client.blob_client(blob_names[0].clone()), None).await?;
    create_test_blob(&container_client.blob_client(blob_names[1].clone()), None).await?;
    create_test_blob(&container_client.blob_client(blob_names[2].clone()), None).await?;
    create_test_blob(&container_client.blob_client(blob_names[3].clone()), None).await?;

    // Continuation Token with Token Provided
    let list_blobs_options = BlobContainerClientListBlobFlatSegmentOptions {
        maxresults: Some(2),
        ..Default::default()
    };
    let mut list_blobs_response = container_client.list_blobs(Some(list_blobs_options))?;
    let first_page = list_blobs_response.try_next().await?;
    let list_blob_segment_response = first_page.unwrap().into_body().await?;
    let continuation_token = list_blob_segment_response.next_marker;
    let blob_list = list_blob_segment_response.segment.blob_items;
    assert_eq!(2, blob_list.len());
    for blob in blob_list {
        let blob_name = blob.name.unwrap().content.unwrap();
        let blob_type = blob.properties.unwrap().blob_type.unwrap();
        assert!(blob_names.contains(&blob_name));
        assert_eq!(BlobType::BlockBlob, blob_type);
    }
    let list_blobs_options = BlobContainerClientListBlobFlatSegmentOptions {
        marker: continuation_token,
        ..Default::default()
    };
    let mut list_blobs_response = container_client.list_blobs(Some(list_blobs_options.clone()))?;
    let second_page = list_blobs_response.try_next().await?;
    let list_blob_segment_response = second_page.unwrap().into_body().await?;
    let blob_list = list_blob_segment_response.segment.blob_items;
    assert_eq!(2, blob_list.len());
    for blob in blob_list {
        let blob_name = blob.name.unwrap().content.unwrap();
        let blob_type = blob.properties.unwrap().blob_type.unwrap();
        assert!(blob_names.contains(&blob_name));
        assert_eq!(BlobType::BlockBlob, blob_type);
    }

    // Continuation Token, Automatic Paging
    let mut pager_response = container_client.list_blobs(Some(list_blobs_options))?;
    let mut page_count = 0;

    while let Some(page) = pager_response.next().await {
        page_count += 1;
        let current_page = page.unwrap().into_body().await?;
        match page_count {
            1 => {
                let blob_list = current_page.segment.blob_items;
                assert_eq!(2, blob_list.len());

                for blob in blob_list {
                    let blob_name = blob.name.unwrap().content.unwrap();
                    let blob_type = blob.properties.unwrap().blob_type.unwrap();
                    assert!(blob_names.contains(&blob_name));
                    assert_eq!(BlobType::BlockBlob, blob_type);
                }
            }
            2 => {
                let blob_list = current_page.segment.blob_items;
                assert_eq!(2, blob_list.len());

                for blob in blob_list {
                    let blob_name = blob.name.unwrap().content.unwrap();
                    let blob_type = blob.properties.unwrap().blob_type.unwrap();
                    assert!(blob_names.contains(&blob_name));
                    assert_eq!(BlobType::BlockBlob, blob_type);
                }
            }
            _ => {
                panic!("Unexpected page number reached.")
            }
        }
    }

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_container_lease_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let blob_service_client = get_blob_service_client(recording)?;
    let container_name = get_container_name(recording);
    let container_client = blob_service_client.blob_container_client(container_name.clone());
    let other_container_client = blob_service_client.blob_container_client(container_name);
    container_client.create_container(None).await?;

    // Acquire Lease
    let acquire_response = container_client.acquire_lease(15, None).await?;
    let lease_id = acquire_response.lease_id()?.unwrap();
    let other_acquire_response = other_container_client.acquire_lease(15, None).await;
    // Assert
    let error = other_acquire_response.unwrap_err().http_status();
    assert_eq!(StatusCode::Conflict, error.unwrap());

    let update_metadata = HashMap::from([("hello".to_string(), "world".to_string())]);
    let set_metadata_options = BlobContainerClientSetMetadataOptions {
        lease_id: Some(lease_id.clone()),
        ..Default::default()
    };
    container_client
        .set_metadata(update_metadata, Some(set_metadata_options))
        .await?;

    // Change Lease
    let proposed_lease_id = "00000000-1111-2222-3333-444444444444".to_string();
    let change_lease_response = container_client
        .change_lease(lease_id, proposed_lease_id.clone(), None)
        .await?;
    // Assert
    let lease_id = change_lease_response.lease_id()?.unwrap();
    assert_eq!(proposed_lease_id.clone().to_string(), lease_id);

    // Sleep until lease expires
    time::sleep(std::time::Duration::from_secs(15)).await;

    // Renew Lease
    container_client
        .renew_lease(proposed_lease_id.clone(), None)
        .await?;
    let other_acquire_response = other_container_client.acquire_lease(15, None).await;
    // Assert
    let error = other_acquire_response.unwrap_err().http_status();
    assert_eq!(StatusCode::Conflict, error.unwrap());

    // Break Lease
    container_client.break_lease(None).await?;
    let other_acquire_response = other_container_client.acquire_lease(15, None).await;
    // Assert
    let error = other_acquire_response.unwrap_err().http_status();
    assert_eq!(StatusCode::Conflict, error.unwrap());

    // Release Lease
    container_client
        .release_lease(proposed_lease_id.clone(), None)
        .await?;
    let other_acquire_response = other_container_client.acquire_lease(15, None).await;
    let lease_id = other_acquire_response?.lease_id().unwrap();
    other_container_client
        .release_lease(lease_id.unwrap(), None)
        .await?;

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_get_account_info(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true).await?;

    // Act
    let response = container_client.get_account_info(None).await?;

    // Assert
    let sku_name = response.sku_name()?;
    let account_kind = response.account_kind()?;

    assert!(sku_name.is_some());
    assert_eq!(AccountKind::StorageV2, account_kind.unwrap());

    Ok(())
}

#[recorded::test]
async fn test_container_access_policy(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, false).await?;
    container_client.create_container(None).await?;

    // Set Access Policy w/ Policy Defined
    let access_policy = AccessPolicy {
        expiry: Some(OffsetDateTime::now_utc() + Duration::seconds(10)),
        permission: Some("rw".to_string()),
        start: Some(OffsetDateTime::now_utc()),
    };
    let signed_identifier = SignedIdentifier {
        access_policy: Some(access_policy),
        id: None,
    };

    container_client
        .set_access_policy(signed_identifier.into()?, None)
        .await?;

    // Assert
    let access_policy_response = container_client.get_access_policy(None).await?;
    let signed_identifiers = access_policy_response.into_body().await?;
    for signed_identifier in &signed_identifiers {
        if let Some(access_policy) = &signed_identifier.access_policy {
            assert!(signed_identifier.id.is_some());
            assert!(access_policy.start.is_some());
            assert!(access_policy.expiry.is_some());
            assert_eq!("rw", access_policy.permission.as_ref().unwrap());
        }
    }

    container_client.delete_container(None).await?;
    Ok(())
}
