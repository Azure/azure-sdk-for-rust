// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    http::{ClientOptions, RequestContent, StatusCode, Url},
    time::{parse_rfc3339, to_rfc3339, OffsetDateTime},
    Bytes,
};
use azure_core_test::{recorded, Matcher, TestContext, VarOptions};
use azure_storage_blob::{
    models::{
        AccessTier, AccountKind, BlobClientAcquireLeaseResultHeaders,
        BlobClientChangeLeaseResultHeaders, BlobClientCreateSnapshotOptions,
        BlobClientCreateSnapshotResultHeaders, BlobClientDeleteOptions, BlobClientDownloadOptions,
        BlobClientDownloadResultHeaders, BlobClientGetAccountInfoResultHeaders,
        BlobClientGetPropertiesOptions, BlobClientGetPropertiesResultHeaders,
        BlobClientSetImmutabilityPolicyOptions, BlobClientSetMetadataOptions,
        BlobClientSetPropertiesOptions, BlobClientSetTierOptions,
        BlobContainerClientListBlobFlatSegmentOptions, BlockBlobClientUploadOptions,
        DeleteSnapshotsOptionType, ImmutabilityPolicyMode, LeaseState, ListBlobsIncludeItem,
    },
    BlobClient, BlobClientOptions, BlobContainerClient, BlobContainerClientOptions,
};
use azure_storage_blob_test::{
    create_test_blob, get_blob_name, get_container_client, StorageAccount,
};
use futures::TryStreamExt;
use std::{collections::HashMap, error::Error, time::Duration};
use tokio::time;

#[recorded::test]
async fn test_get_blob_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, false, StorageAccount::Standard).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));

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
    let container_client = get_container_client(recording, true, StorageAccount::Standard).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
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
    let container_client = get_container_client(recording, true, StorageAccount::Standard).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));

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
    let container_client = get_container_client(recording, true, StorageAccount::Standard).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
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
async fn test_undelete_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true, StorageAccount::Standard).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    create_test_blob(&blob_client, None, None).await?;

    // Existence Check
    blob_client.get_properties(None).await?;

    // Delete Blob
    blob_client.delete(None).await?;
    let response = blob_client.download(None).await;
    let error = response.unwrap_err().http_status();
    assert_eq!(StatusCode::NotFound, error.unwrap());

    // Undelete and Assert
    blob_client.undelete(None).await?;
    let response = blob_client.get_properties(None).await?;
    let content_length = response.content_length()?;
    assert_eq!(17, content_length.unwrap());

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_download_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true, StorageAccount::Standard).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
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
    let container_client = get_container_client(recording, true, StorageAccount::Standard).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
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
    let container_client = get_container_client(recording, true, StorageAccount::Standard).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
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
    let container_client = get_container_client(recording, true, StorageAccount::Standard).await?;
    let blob_name = get_blob_name(recording);
    let blob_client = container_client.blob_client(&blob_name.clone());
    let other_blob_client = container_client.blob_client(&blob_name);
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
    let container_client = get_container_client(recording, true, StorageAccount::Standard).await?;
    let blob_name = get_blob_name(recording);
    let blob_client = container_client.blob_client(&blob_name.clone());
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
    let container_client = get_container_client(recording, true, StorageAccount::Standard).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    create_test_blob(&blob_client, None, None).await?;

    // Set Tags with Tags Specified
    let blob_tags = HashMap::from([
        ("hello".to_string(), "world".to_string()),
        ("ferris".to_string(), "crab".to_string()),
    ]);
    blob_client.set_tags(blob_tags.clone(), None).await?;

    // Assert
    let response_tags = blob_client.get_tags(None).await?.into_model()?;
    let map: HashMap<String, String> = response_tags.into();
    assert_eq!(blob_tags, map);

    // Set Tags with No Tags (Clear Tags)
    blob_client.set_tags(HashMap::new(), None).await?;

    // Assert
    let response_tags = blob_client.get_tags(None).await?.into_model()?;
    let map: HashMap<String, String> = response_tags.into();
    assert_eq!(HashMap::new(), map);

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_get_account_info(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup

    let recording = ctx.recording();
    let container_client = get_container_client(recording, true, StorageAccount::Standard).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));

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
async fn test_encoding_edge_cases(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let mut client_options = ClientOptions::default();
    recording.instrument(&mut client_options);

    // ContainerClient Options
    let container_client_options = BlobContainerClientOptions {
        client_options: client_options.clone(),
        ..Default::default()
    };

    // BlobClient Options
    let blob_client_options = BlobClientOptions {
        client_options: client_options.clone(),
        ..Default::default()
    };

    // Endpoint
    let endpoint = format!(
        "https://{}.blob.core.windows.net/",
        recording.var("AZURE_STORAGE_ACCOUNT_NAME", None).as_str()
    );

    let container_name = "test-container-encoding-edge-cases";
    // Create Container & Container Client
    let container_client = BlobContainerClient::new(
        &endpoint,
        container_name,
        Some(recording.credential()),
        Some(container_client_options.clone()),
    )?;
    container_client.create_container(None).await?;

    // Test Data for Parameterization
    let test_cases = [
        // Basic + paths - combines simple case with forward slashes (virtual directories)
        "folder/subfolder/file.txt",
        // Reserved URL characters requiring encoding - combines spaces, %, ?, &, =, #, + in one test
        "Q4 2024/report 50%+tax?final=true&approved#section-1.pdf",
        // Unicode (multi-script) + unreserved chars - combines UTF-8 with chars that don't need encoding
        "カニのフェリス_🦀.txt",
        // Consecutive special chars
        "path\\\\with___...~~~consecutive///chars",
        // Additional reserved chars: parentheses, brackets, colon, quotes, leading/trailing spaces
        " file (copy) [2024]:version'1'.txt ",
        // Mix of forward and backslashes to test normalization/preservation
        "forward/back\\forward/back\\",
        // Test of already encoded characters but we want them preserved as-is
        "data%20set%ferris%3D1%the%23crab%2D2",
    ];
    for blob_name in test_cases {
        // Test Case 1: Initialize BlobClient using new() constructor
        let blob_client_new = BlobClient::new(
            &endpoint,
            container_name,
            blob_name,
            Some(recording.credential()),
            Some(blob_client_options.clone()),
        )?;

        // Upload Blob
        blob_client_new
            .upload(
                RequestContent::from(b"hello rusty world".to_vec()),
                true,
                17,
                None,
            )
            .await?;

        // Get Properties
        let properties = blob_client_new.get_properties(None).await?;
        assert_eq!(17, properties.content_length()?.unwrap());

        // Test Case 2: Initialize BlobClient using from_blob_url(), separate path segments
        let mut blob_url = Url::parse(&endpoint)?;
        blob_url
            .path_segments_mut()
            .expect("Storage Endpoint must be a valid base URL with http/https scheme")
            .push(container_name)
            .push(blob_name);

        let blob_client_from_url = BlobClient::from_url(
            blob_url,
            Some(recording.credential()),
            Some(blob_client_options.clone()),
        )?;

        // Upload Blob
        blob_client_from_url
            .upload(
                RequestContent::from(b"hello rusty world".to_vec()),
                true,
                17,
                None,
            )
            .await?;

        // Get Properties
        let properties = blob_client_from_url.get_properties(None).await?;
        assert_eq!(17, properties.content_length()?.unwrap());

        // Test Case 3: Initialize BlobClient using ContainerClient accessor
        let blob_client_from_cc = container_client.blob_client(blob_name);

        // Upload Blob
        blob_client_from_cc
            .upload(
                RequestContent::from(b"hello rusty world".to_vec()),
                true,
                17,
                None,
            )
            .await?;

        // Get Properties
        let properties = blob_client_from_cc.get_properties(None).await?;
        assert_eq!(17, properties.content_length()?.unwrap());
    }

    // Check name equality for all test cases
    let mut list_blobs_response = container_client.list_blobs(None)?.into_pages();
    let page = list_blobs_response.try_next().await?;
    let list_blob_segment_response = page.unwrap().into_model()?;
    let blob_items = list_blob_segment_response.segment.blob_items;

    // Ensure we have the expected number of blobs
    assert_eq!(test_cases.len(), blob_items.len());

    // Extract all blob names from list_blobs() response
    let listed_blob_names: Vec<String> = blob_items
        .iter()
        .map(|blob| blob.name.clone().unwrap().content.unwrap())
        .collect();
    // Verify each test case blob name appears in the list (with normalization)
    for blob_name in test_cases {
        let normalized_name = blob_name.replace('\\', "/");
        assert!(
            listed_blob_names.contains(&normalized_name),
            "Blob name '{}' (normalized: '{}') not found in list: {:?}",
            blob_name,
            normalized_name,
            listed_blob_names
        );
    }

    container_client.delete_container(None).await?;

    Ok(())
}

#[recorded::test(playback)]
async fn test_set_legal_hold(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, false, StorageAccount::Standard).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    container_client.create_container(None).await?;
    create_test_blob(&blob_client, None, None).await?;

    // Set Legal Hold
    blob_client.set_legal_hold(true, None).await?;
    let response = blob_client.get_properties(None).await?;
    // Assert
    let legal_hold = response.legal_hold()?;
    assert!(legal_hold.unwrap());

    // Attempt Operation While Legal Hold Active
    let response = blob_client.delete(None).await;
    // Assert
    let error = response.unwrap_err().http_status();
    assert_eq!(StatusCode::Conflict, error.unwrap());

    // Remove Legal Hold
    blob_client.set_legal_hold(false, None).await?;
    let response = blob_client.get_properties(None).await?;
    // Assert
    let legal_hold = response.legal_hold()?;
    assert!(!legal_hold.unwrap());

    blob_client.delete(None).await?;

    Ok(())
}

#[recorded::test(playback)]
async fn test_immutability_policy(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Versioned).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    container_client.create_container(None).await?;
    create_test_blob(&blob_client, None, None).await?;

    // Set Immutability Policy (No Mode Specified, Default to Unlocked)
    let expiry_1_str = recording.var(
        "expiry_1",
        Some(VarOptions {
            default_value: Some(
                to_rfc3339(&(OffsetDateTime::now_utc() + Duration::from_secs(5))).into(),
            ),
            ..Default::default()
        }),
    );
    let expiry_1 = parse_rfc3339(&expiry_1_str)?;

    blob_client.set_immutability_policy(&expiry_1, None).await?;

    // Assert
    let response = blob_client.get_properties(None).await?;
    let mode = response.immutability_policy_mode()?;
    let expires_on = response.immutability_policy_expires_on()?;
    assert_eq!(ImmutabilityPolicyMode::Unlocked, mode.unwrap());
    // Need to ignore nanoseconds due to Service truncation
    assert_eq!(expiry_1.replace_nanosecond(0)?, expires_on.unwrap());

    // Delete Immutability Policy
    blob_client.delete_immutability_policy(None).await?;
    let response = blob_client.get_properties(None).await?;

    // Assert
    let mode = response.immutability_policy_mode()?;
    let expires_on = response.immutability_policy_expires_on()?;
    assert!(mode.is_none());
    assert!(expires_on.is_none());

    // Set Immutability Policy (Locked Mode)
    let expiry_2_str = recording.var(
        "expiry_2",
        Some(VarOptions {
            default_value: Some(
                to_rfc3339(&(OffsetDateTime::now_utc() + Duration::from_secs(5))).into(),
            ),
            ..Default::default()
        }),
    );
    let expiry_2 = parse_rfc3339(&expiry_2_str)?;
    let immutability_policy_options = BlobClientSetImmutabilityPolicyOptions {
        immutability_policy_mode: Some(ImmutabilityPolicyMode::Locked),
        ..Default::default()
    };
    blob_client
        .set_immutability_policy(&expiry_2, Some(immutability_policy_options))
        .await?;

    // Assert
    let response = blob_client.get_properties(None).await?;
    let mode = response.immutability_policy_mode()?;
    let expires_on = response.immutability_policy_expires_on()?;
    assert_eq!(ImmutabilityPolicyMode::Locked, mode.unwrap());
    // Need to ignore nanoseconds due to Service truncation
    assert_eq!(expiry_2.replace_nanosecond(0)?, expires_on.unwrap());

    // Sleep to allow immutability policy to expire
    time::sleep(Duration::from_secs(5)).await;

    blob_client.delete(None).await?;

    Ok(())
}

#[recorded::test]
async fn test_blob_version_read_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true, StorageAccount::Versioned).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));

    // Create Multiple Versions
    let data_v1 = b"version 1 content";
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(data_v1.to_vec())),
        None,
    )
    .await?;
    let response = blob_client.get_properties(None).await?;
    let version_1 = response.version_id()?.unwrap();
    let data_v2 = b"version 2 content";
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(data_v2.to_vec())),
        None,
    )
    .await?;
    let response = blob_client.get_properties(None).await?;
    let version_2 = response.version_id()?.unwrap();

    // Download Version 1 Using with_version_id()
    let version_1_client = blob_client.with_version_id(&version_1)?;
    let download_response = version_1_client.download(None).await?;
    let (status_code, _, response_body) = download_response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(data_v1.to_vec(), response_body.collect().await?.to_vec());

    // Download Version 1 Using Options (Test query parameter replaces)

    // Create blob_client w/ version_2 with intention to actually download version_1 with options bag
    let version_2_client = blob_client.with_version_id(&version_2)?;
    let download_options = BlobClientDownloadOptions {
        version_id: Some(version_1.clone()),
        ..Default::default()
    };
    let download_response = version_2_client.download(Some(download_options)).await?;
    let (status_code, _, response_body) = download_response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(data_v1.to_vec(), response_body.collect().await?.to_vec());

    // Get Properties
    let props_v1 = version_1_client.get_properties(None).await?;
    assert_eq!(
        u64::try_from(data_v1.len())?,
        props_v1.content_length()?.unwrap()
    );
    assert_eq!(version_1, props_v1.version_id()?.unwrap());
    let props_v2 = version_2_client.get_properties(None).await?;
    assert_eq!(
        u64::try_from(data_v2.len())?,
        props_v2.content_length()?.unwrap()
    );
    assert_eq!(version_2, props_v2.version_id()?.unwrap());

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_blob_version_metadata_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true, StorageAccount::Versioned).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));

    // Create Version 1 with Metadata
    let metadata_v1 = HashMap::from([("version".to_string(), "one".to_string())]);
    let upload_options = BlockBlobClientUploadOptions {
        metadata: Some(metadata_v1.clone()),
        ..Default::default()
    };
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(b"content v1".to_vec())),
        Some(upload_options),
    )
    .await?;
    let response = blob_client.get_properties(None).await?;
    let version_1 = response.version_id()?.unwrap();

    // Set Metadata on Current Version (Creates Version 2)
    let metadata_v2 = HashMap::from([("version".to_string(), "two".to_string())]);
    blob_client.set_metadata(metadata_v2.clone(), None).await?;
    let response = blob_client.get_properties(None).await?;
    let version_2 = response.version_id()?.unwrap();

    // Verify metadata matches corresponding version
    let version_1_client = blob_client.with_version_id(&version_1)?;
    let props_v1 = version_1_client.get_properties(None).await?;
    assert_eq!(metadata_v1, props_v1.metadata()?);
    let version_2_client = blob_client.with_version_id(&version_2)?;
    let props_v2 = version_2_client.get_properties(None).await?;
    assert_eq!(metadata_v2, props_v2.metadata()?);
    assert_ne!(version_1, version_2);

    // Upload New Content (Creates Version 3)
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(b"content v3".to_vec())),
        None,
    )
    .await?;
    let response = blob_client.get_properties(None).await?;
    let version_3 = response.version_id()?.unwrap();
    assert_ne!(version_2, version_3);

    // Set Tags on Current Version (Does NOT Create New Version)
    let tags = HashMap::from([("env".to_string(), "test".to_string())]);
    blob_client.set_tags(tags.clone(), None).await?;

    // Verify version_id hasn't changed after setting tags
    let response = blob_client.get_properties(None).await?;
    let version_after_tags = response.version_id()?.unwrap();
    assert_eq!(version_3, version_after_tags);

    // Verify Previous Versions Have No Tags
    let response_tags = version_2_client.get_tags(None).await?.into_model()?;
    let retrieved_tags: HashMap<String, String> = response_tags.into();
    assert_eq!(HashMap::new(), retrieved_tags);

    let response_tags = version_1_client.get_tags(None).await?.into_model()?;
    let retrieved_tags: HashMap<String, String> = response_tags.into();
    assert_eq!(HashMap::new(), retrieved_tags);

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_blob_version_tier_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true, StorageAccount::Versioned).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));

    // Create Version 1 in Hot Tier
    create_test_blob(&blob_client, None, None).await?;
    let response = blob_client.get_properties(None).await?;
    let version_1 = response.version_id()?.unwrap();
    assert_eq!(
        AccessTier::Hot.to_string(),
        response.access_tier()?.unwrap()
    );

    // Create Version 2
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(b"version 2 content".to_vec())),
        None,
    )
    .await?;
    let response = blob_client.get_properties(None).await?;
    let version_2 = response.version_id()?.unwrap();

    // Set Tier on Version 1 (Non-Current)
    let version_1_client = blob_client.with_version_id(&version_1)?;
    let set_tier_options = BlobClientSetTierOptions {
        version_id: Some(version_1.clone()),
        ..Default::default()
    };
    version_1_client
        .set_tier(AccessTier::Cool, Some(set_tier_options))
        .await?;

    // Verify version_1 is Cool, version_2 is Hot
    let props_v1 = version_1_client.get_properties(None).await?;
    assert_eq!(
        AccessTier::Cool.to_string(),
        props_v1.access_tier()?.unwrap()
    );
    let version_2_client = blob_client.with_version_id(&version_2)?;
    let props_v2 = version_2_client.get_properties(None).await?;
    assert_eq!(
        AccessTier::Hot.to_string(),
        props_v2.access_tier()?.unwrap()
    );

    // Set Tier on Current Version
    blob_client.set_tier(AccessTier::Cool, None).await?;
    let props_current = blob_client.get_properties(None).await?;
    assert_eq!(
        AccessTier::Cool.to_string(),
        props_current.access_tier()?.unwrap()
    );

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs_with_versions(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true, StorageAccount::Versioned).await?;

    // Create Blob 1 with Multiple Versions
    let blob_1_name = get_blob_name(recording);
    let blob_1_client = container_client.blob_client(&blob_1_name);
    create_test_blob(&blob_1_client, None, None).await?;
    create_test_blob(&blob_1_client, None, None).await?;

    // Create Blob 2 with Multiple Versions
    let blob_2_name = get_blob_name(recording);
    let blob_2_client = container_client.blob_client(&blob_2_name);
    create_test_blob(&blob_2_client, None, None).await?;
    create_test_blob(&blob_2_client, None, None).await?;
    create_test_blob(&blob_2_client, None, None).await?;

    // List Blobs Without Versions
    let mut list_response = container_client.list_blobs(None)?.into_pages();
    let page = list_response.try_next().await?;
    let segment = page.unwrap().into_model()?;
    let blob_items = segment.segment.blob_items;
    // Only current versions
    assert_eq!(2, blob_items.len());

    // List Blobs With Versions
    let list_options = BlobContainerClientListBlobFlatSegmentOptions {
        include: Some(vec![ListBlobsIncludeItem::Versions]),
        ..Default::default()
    };
    let mut list_response = container_client
        .list_blobs(Some(list_options))?
        .into_pages();
    let page = list_response.try_next().await?;
    let segment = page.unwrap().into_model()?;
    let blob_items = segment.segment.blob_items;

    // Verify all 5 versions (2 from blob_1 + 3 from blob_2) are present
    assert_eq!(5, blob_items.len());

    // Count Versions Per Blob
    let mut version_counts: HashMap<&str, usize> = HashMap::new();
    let mut current_versions = 0;

    for blob_item in &blob_items {
        let name = blob_item.name.as_ref().unwrap().content.as_ref().unwrap();
        let version_id = blob_item.version_id.as_ref();
        let is_current = blob_item.is_current_version.unwrap_or(false);
        assert!(version_id.is_some(),);

        *version_counts.entry(name.as_str()).or_insert(0) += 1;

        if is_current {
            current_versions += 1;
        }
    }

    // Assert
    assert_eq!(2, version_counts[blob_1_name.as_str()]);
    assert_eq!(3, version_counts[blob_2_name.as_str()]);
    assert_eq!(2, current_versions);

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_blob_version_feature_interactions(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true, StorageAccount::Versioned).await?;
    let source_blob_name = format!("{}-source", get_blob_name(recording));
    let source_blob_client = container_client.blob_client(&source_blob_name);

    // Create Source Blob with Multiple Versions
    let data_v1 = b"source version 1";
    source_blob_client
        .upload(
            RequestContent::from(data_v1.to_vec()),
            false,
            u64::try_from(data_v1.len())?,
            None,
        )
        .await?;

    let data_v2 = b"source version 2";
    source_blob_client
        .upload(
            RequestContent::from(data_v2.to_vec()),
            true,
            u64::try_from(data_v2.len())?,
            None,
        )
        .await?;

    // Test: Lease on Current Version
    let lease_blob_name = format!("{}-lease", get_blob_name(recording));
    let lease_blob_client = container_client.blob_client(&lease_blob_name);
    lease_blob_client
        .upload(RequestContent::from(b"v1".to_vec()), false, 2, None)
        .await?;
    let response = lease_blob_client.get_properties(None).await?;
    let lease_version_1 = response.version_id()?.unwrap();

    lease_blob_client
        .upload(RequestContent::from(b"v2".to_vec()), true, 2, None)
        .await?;

    // Acquire Lease on Current Version
    let acquire_response = lease_blob_client.acquire_lease(-1, None).await?;
    let lease_id = acquire_response.lease_id()?.unwrap();

    // Verify Older Version is Still Accessible Without Lease
    let lease_version_1_client = lease_blob_client.with_version_id(&lease_version_1)?;
    let props = lease_version_1_client.get_properties(None).await?;
    assert_eq!(2, props.content_length()?.unwrap());

    // Release Lease
    lease_blob_client.release_lease(lease_id, None).await?;

    // Test: Conditional Operation with Version
    let etag = props.etag()?.unwrap();
    let get_options = BlobClientGetPropertiesOptions {
        if_match: Some(etag.clone()),
        version_id: Some(lease_version_1.clone()),
        ..Default::default()
    };
    let conditional_response = lease_blob_client.get_properties(Some(get_options)).await?;
    assert_eq!(2, conditional_response.content_length()?.unwrap());

    container_client.delete_container(None).await?;
    Ok(())
}

// #[recorded::test(playback)]
#[recorded::test]
async fn test_blob_version_immutability_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Versioned).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    container_client.create_container(None).await?;

    // Create Version 1 & 2
    create_test_blob(&blob_client, None, None).await?;
    let response = blob_client.get_properties(None).await?;
    let version_1 = response.version_id()?.unwrap();
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(b"version 2".to_vec())),
        None,
    )
    .await?;
    let response = blob_client.get_properties(None).await?;
    let version_2 = response.version_id()?.unwrap();

    // Set Legal Hold on Version 1
    let version_1_client = blob_client.with_version_id(&version_1)?;
    version_1_client.set_legal_hold(true, None).await?;
    let props_v1 = version_1_client.get_properties(None).await?;
    assert!(props_v1.legal_hold()?.unwrap());

    // Verify Version 2 Does Not Have Legal Hold
    let version_2_client = blob_client.with_version_id(&version_2)?;
    let props_v2 = version_2_client.get_properties(None).await?;
    assert!(!props_v2.legal_hold()?.unwrap_or(false));

    // Attempt to Delete Version 1
    let delete_options = BlobClientDeleteOptions {
        version_id: Some(version_1.clone()),
        ..Default::default()
    };
    let result = version_1_client.delete(Some(delete_options)).await;
    assert!(result.is_err());

    // Remove Legal Hold from Version 1
    version_1_client.set_legal_hold(false, None).await?;

    // Set Immutability Policy on Version 2
    let expiry = recording.var(
        "version_2_expiry",
        Some(VarOptions {
            default_value: Some(
                to_rfc3339(&(OffsetDateTime::now_utc() + Duration::from_secs(5))).into(),
            ),
            ..Default::default()
        }),
    );
    let expiry_time = parse_rfc3339(&expiry)?;
    let immutability_options = BlobClientSetImmutabilityPolicyOptions {
        version_id: Some(version_2.clone()),
        ..Default::default()
    };
    version_2_client
        .set_immutability_policy(&expiry_time, Some(immutability_options))
        .await?;

    // Verify Immutability Policy on version_2, None on version_1
    let props_v2 = version_2_client.get_properties(None).await?;
    assert!(props_v2.immutability_policy_expires_on()?.is_some());
    let props_v1 = version_1_client.get_properties(None).await?;
    assert!(props_v1.immutability_policy_expires_on()?.is_none());

    Ok(())
}

#[recorded::test]
async fn test_blob_version_error_cases(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true, StorageAccount::Versioned).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));

    // Create a Blob with One Version
    create_test_blob(&blob_client, None, None).await?;
    let response = blob_client.get_properties(None).await?;
    let valid_version = response.version_id()?.unwrap();

    // Test: Invalid Version ID Format
    let invalid_version_client = blob_client.with_version_id("invalid-version-id")?;
    let result = invalid_version_client.get_properties(None).await;
    assert!(result.is_err());

    // Test: Non-Existent Version ID
    let fake_version = "2000-05-11T00:00:00.0000000Z";
    let fake_version_client = blob_client.with_version_id(fake_version)?;
    let result = fake_version_client.get_properties(None).await;
    assert!(result.is_err());
    let error = result.unwrap_err().http_status();
    assert!(error == Some(StatusCode::NotFound));

    // Test: Delete Non-Current Version and Verify It's Gone
    create_test_blob(&blob_client, Some(RequestContent::from(b"v2".into())), None).await?;
    let version_1_client = blob_client.with_version_id(&valid_version)?;
    let delete_options = BlobClientDeleteOptions {
        version_id: Some(valid_version.clone()),
        ..Default::default()
    };
    version_1_client.delete(Some(delete_options)).await?;

    // Verify Version is Deleted
    let result = version_1_client.get_properties(None).await;
    assert!(result.is_err());
    let error = result.unwrap_err().http_status();
    assert_eq!(StatusCode::NotFound, error.unwrap());

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_blob_snapshot_basic_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true, StorageAccount::Standard).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let data_v1 = b"snapshot version 1";

    // Create Base Blob
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(data_v1.to_vec())),
        None,
    )
    .await?;

    // Create Snapshot
    let snapshot_response = blob_client.create_snapshot(None).await?;
    let snapshot_1 = snapshot_response.snapshot()?.unwrap();
    assert!(!snapshot_1.is_empty());

    // Get Snapshot Properties
    let snapshot_1_client = blob_client.with_snapshot(&snapshot_1)?;
    let props = snapshot_1_client.get_properties(None).await?;
    assert_eq!(
        u64::try_from(data_v1.len())?,
        props.content_length()?.unwrap()
    );

    // Download Snapshot Content
    let download_response = snapshot_1_client.download(None).await?;
    let (status_code, _, response_body) = download_response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(data_v1.to_vec(), response_body.collect().await?.to_vec());

    // Modify Base Blob
    let data_v2 = b"snapshot version 2";
    blob_client
        .upload(
            RequestContent::from(data_v2.to_vec()),
            true,
            u64::try_from(data_v2.len())?,
            None,
        )
        .await?;

    // Create Second Snapshot
    let snapshot_response_2 = blob_client.create_snapshot(None).await?;
    let snapshot_2 = snapshot_response_2.snapshot()?.unwrap();

    // Verify First Snapshot is Unchanged
    let download_response = snapshot_1_client.download(None).await?;
    let (status_code, _, response_body) = download_response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(data_v1.to_vec(), response_body.collect().await?.to_vec());

    // Test Snapshot Parameter Replacement (Options Override Client)
    let snapshot_2_client = blob_client.with_snapshot(&snapshot_2)?;
    let download_options = BlobClientDownloadOptions {
        snapshot: Some(snapshot_1.clone()),
        ..Default::default()
    };
    let download_response = snapshot_2_client.download(Some(download_options)).await?;
    let (status_code, _, response_body) = download_response.deconstruct();
    assert!(status_code.is_success());
    // Should get snapshot_1 content, not snapshot_2
    assert_eq!(data_v1.to_vec(), response_body.collect().await?.to_vec());

    // Verify Base Blob Has New Content
    let download_response = blob_client.download(None).await?;
    let (status_code, _, response_body) = download_response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(data_v2.to_vec(), response_body.collect().await?.to_vec());

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_blob_snapshot_metadata_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true, StorageAccount::Standard).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));

    // Create Blob with Metadata
    let base_metadata = HashMap::from([("base".to_string(), "model".to_string())]);
    let upload_options = BlockBlobClientUploadOptions {
        metadata: Some(base_metadata.clone()),
        ..Default::default()
    };
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(b"based model".to_vec())),
        Some(upload_options),
    )
    .await?;

    // Create Snapshot (Inherits Base Blob Metadata)
    let snapshot_response = blob_client.create_snapshot(None).await?;
    let snapshot_1 = snapshot_response.snapshot()?.unwrap();
    let snapshot_1_client = blob_client.with_snapshot(&snapshot_1)?;
    let props = snapshot_1_client.get_properties(None).await?;
    assert_eq!(base_metadata, props.metadata()?);

    // Create Snapshot with New Metadata
    let snapshot_metadata = HashMap::from([("something".to_string(), "different".to_string())]);
    let snapshot_options = BlobClientCreateSnapshotOptions {
        metadata: Some(snapshot_metadata.clone()),
        ..Default::default()
    };
    let snapshot_response_2 = blob_client.create_snapshot(Some(snapshot_options)).await?;
    let snapshot_2 = snapshot_response_2.snapshot()?.unwrap();
    let snapshot_2_client = blob_client.with_snapshot(&snapshot_2)?;
    let props_2 = snapshot_2_client.get_properties(None).await?;
    assert_eq!(snapshot_metadata, props_2.metadata()?);

    // Modify Current (Base) Blob Metadata
    let new_base_metadata = HashMap::from([("end".to_string(), "game".to_string())]);
    blob_client
        .set_metadata(new_base_metadata.clone(), None)
        .await?;

    // Verify Snapshots Unchanged
    let props_1 = snapshot_1_client.get_properties(None).await?;
    assert_eq!(base_metadata, props_1.metadata()?);
    let props_2 = snapshot_2_client.get_properties(None).await?;
    assert_eq!(snapshot_metadata, props_2.metadata()?);

    // Verify Base Blob Has New Metadata
    let base_props = blob_client.get_properties(None).await?;
    assert_eq!(new_base_metadata, base_props.metadata()?);

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs_with_snapshots(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true, StorageAccount::Standard).await?;

    // Create Blob 1 with Multiple Snapshots
    let blob_1_name = get_blob_name(recording);
    let blob_1_client = container_client.blob_client(&blob_1_name);
    create_test_blob(&blob_1_client, None, None).await?;
    blob_1_client.create_snapshot(None).await?;
    blob_1_client.create_snapshot(None).await?;

    // Create Blob 2 with Multiple Snapshots
    let blob_2_name = get_blob_name(recording);
    let blob_2_client = container_client.blob_client(&blob_2_name);
    create_test_blob(&blob_2_client, None, None).await?;
    blob_2_client.create_snapshot(None).await?;
    blob_2_client.create_snapshot(None).await?;
    blob_2_client.create_snapshot(None).await?;

    // List Blobs Without Snapshots
    let mut list_response = container_client.list_blobs(None)?.into_pages();
    let page = list_response.try_next().await?;
    let segment = page.unwrap().into_model()?;
    let blob_items = segment.segment.blob_items;
    // Only base blobs, no snapshots
    assert_eq!(2, blob_items.len());
    for blob_item in &blob_items {
        assert!(blob_item.snapshot.is_none());
    }

    // List Blobs With Snapshots
    let list_options = BlobContainerClientListBlobFlatSegmentOptions {
        include: Some(vec![ListBlobsIncludeItem::Snapshots]),
        ..Default::default()
    };
    let mut list_response = container_client
        .list_blobs(Some(list_options))?
        .into_pages();
    let page = list_response.try_next().await?;
    let segment = page.unwrap().into_model()?;
    let blob_items = segment.segment.blob_items;

    // Verify all blobs and snapshots (2 base + 2 snapshots for blob_1 + 3 snapshots for blob_2 = 7)
    assert_eq!(7, blob_items.len());

    // Count Snapshots Per Blob
    let mut snapshot_counts: HashMap<&str, usize> = HashMap::new();
    let mut base_blob_count = 0;

    for blob_item in &blob_items {
        let name = blob_item.name.as_ref().unwrap().content.as_ref().unwrap();
        if blob_item.snapshot.is_some() {
            *snapshot_counts.entry(name.as_str()).or_insert(0) += 1;
        } else {
            base_blob_count += 1;
        }
    }

    // Assert
    assert_eq!(2, snapshot_counts[blob_1_name.as_str()]);
    assert_eq!(3, snapshot_counts[blob_2_name.as_str()]);
    assert_eq!(2, base_blob_count);

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_blob_snapshot_delete_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true, StorageAccount::Standard).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    create_test_blob(&blob_client, None, None).await?;

    // Create multiple snapshots
    let snapshot_response_1 = blob_client.create_snapshot(None).await?;
    let snapshot_1 = snapshot_response_1.snapshot()?.unwrap();
    let snapshot_response_2 = blob_client.create_snapshot(None).await?;
    let snapshot_2 = snapshot_response_2.snapshot()?.unwrap();
    let snapshot_response_3 = blob_client.create_snapshot(None).await?;
    let snapshot_3 = snapshot_response_3.snapshot()?.unwrap();

    // Delete Specific Snapshot (snapshot_2)
    let snapshot_2_client = blob_client.with_snapshot(&snapshot_2)?;
    snapshot_2_client.delete(None).await?;

    // Verify snapshot_2 is deleted and that snapshot_1 and base blob (snapshot_3) still exists
    let result = snapshot_2_client.get_properties(None).await;
    assert!(result.is_err());
    let error = result.unwrap_err().http_status();
    assert_eq!(StatusCode::NotFound, error.unwrap());
    let snapshot_1_client = blob_client.with_snapshot(&snapshot_1)?;
    snapshot_1_client.get_properties(None).await?;
    let snapshot_3_client = blob_client.with_snapshot(&snapshot_3)?;
    snapshot_3_client.get_properties(None).await?;
    blob_client.get_properties(None).await?;

    // Delete Only Snapshots (Base Blob Remains)
    let delete_options = BlobClientDeleteOptions {
        delete_snapshots: Some(DeleteSnapshotsOptionType::Only),
        ..Default::default()
    };
    blob_client.delete(Some(delete_options)).await?;

    // Verify snapshots are deleted and that base blob still exists
    let result = snapshot_1_client.get_properties(None).await;
    assert!(result.is_err());
    let result = snapshot_3_client.get_properties(None).await;
    assert!(result.is_err());
    blob_client.get_properties(None).await?;

    // Create new snapshots
    blob_client.create_snapshot(None).await?;
    blob_client.create_snapshot(None).await?;

    // Delete All
    let delete_options = BlobClientDeleteOptions {
        delete_snapshots: Some(DeleteSnapshotsOptionType::Include),
        ..Default::default()
    };
    blob_client.delete(Some(delete_options)).await?;

    // Assert
    let result = blob_client.get_properties(None).await;
    assert!(result.is_err());

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_blob_snapshot_conditional_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true, StorageAccount::Standard).await?;
    let blob_name = get_blob_name(recording);
    let blob_client = container_client.blob_client(&blob_name);
    create_test_blob(&blob_client, None, None).await?;

    // Acquire Lease on Base Blob
    let acquire_response = blob_client.acquire_lease(-1, None).await?;
    let lease_id = acquire_response.lease_id()?.unwrap();

    // Create Snapshot on Leased Blob
    let snapshot_options = BlobClientCreateSnapshotOptions {
        lease_id: Some(lease_id.clone()),
        ..Default::default()
    };
    let snapshot_response = blob_client.create_snapshot(Some(snapshot_options)).await?;
    let snapshot_id = snapshot_response.snapshot()?.unwrap();
    let snapshot_client = blob_client.with_snapshot(&snapshot_id)?;

    // Release Lease
    blob_client.release_lease(lease_id.clone(), None).await?;

    // Test Conditional Snapshot Creation
    let props = blob_client.get_properties(None).await?;
    let etag = props.etag()?.unwrap();
    let conditional_options = BlobClientCreateSnapshotOptions {
        if_match: Some(etag.clone()),
        ..Default::default()
    };
    let conditional_snapshot = blob_client
        .create_snapshot(Some(conditional_options))
        .await?;
    assert!(conditional_snapshot.snapshot()?.is_some());

    // Test Blob Tags Behavior with Snapshots
    let tags = HashMap::from([("test_key".to_string(), "test_value".to_string())]);
    blob_client.set_tags(tags.clone(), None).await?;

    // Verify Tags on Base Blob
    let response_tags = blob_client.get_tags(None).await?.into_model()?;
    let retrieved_tags: HashMap<String, String> = response_tags.into();
    assert_eq!(tags, retrieved_tags);

    // Verify Snapshot Does NOT Have Tags (Tags Not Inherited)
    let snapshot_tags = snapshot_client.get_tags(None).await?.into_model()?;
    let snapshot_tag_map: HashMap<String, String> = snapshot_tags.into();
    assert_eq!(HashMap::new(), snapshot_tag_map);

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_blob_snapshot_error_cases(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true, StorageAccount::Standard).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));

    // Test Snapshot Non-Existent Blob Fails
    let result = blob_client.create_snapshot(None).await;
    assert!(result.is_err());
    let error = result.unwrap_err().http_status();
    assert_eq!(StatusCode::NotFound, error.unwrap());

    // Create Blob for remaining tests
    create_test_blob(&blob_client, None, None).await?;
    let snapshot_response = blob_client.create_snapshot(None).await?;
    let snapshot_id = snapshot_response.snapshot()?.unwrap();

    // Test Invalid Snapshot ID Format Fails
    let invalid_snapshot_client = blob_client.with_snapshot("invalid-snapshot-id")?;
    let result = invalid_snapshot_client.get_properties(None).await;
    assert!(result.is_err());

    // Test Non-Existent Snapshot ID Fails
    let fake_snapshot = "2000-12-01T00:00:00.0000000Z";
    let fake_snapshot_client = blob_client.with_snapshot(fake_snapshot)?;
    let result = fake_snapshot_client.get_properties(None).await;
    assert!(result.is_err());
    let error = result.unwrap_err().http_status();
    assert_eq!(StatusCode::NotFound, error.unwrap());

    // Test Snapshots Are Read Only
    let snapshot_client = blob_client.with_snapshot(&snapshot_id)?;

    // Try to Set Properties
    let set_properties_options = BlobClientSetPropertiesOptions {
        blob_content_language: Some("spanish".to_string()),
        ..Default::default()
    };
    let result = snapshot_client
        .set_properties(Some(set_properties_options))
        .await;
    assert!(result.is_err());

    // Try to Set Metadata
    let metadata = HashMap::from([("test_value".to_string(), "test_key".to_string())]);
    let result = snapshot_client.set_metadata(metadata, None).await;
    assert!(result.is_err());

    // Try to Upload to Snapshot
    let data = b"squash data";
    let result = snapshot_client
        .upload(
            RequestContent::from(data.to_vec()),
            true,
            u64::try_from(data.len())?,
            None,
        )
        .await;
    assert!(result.is_err());

    container_client.delete_container(None).await?;
    Ok(())
}
