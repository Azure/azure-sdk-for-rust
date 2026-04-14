// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    error::ErrorKind,
    http::{headers::CONTENT_TYPE, ClientOptions, RequestContent, StatusCode, Url},
    time::{parse_rfc3339, to_rfc3339, OffsetDateTime},
    Bytes,
};
use azure_core_test::{recorded, Matcher, TestContext, VarOptions};
use azure_storage_blob::{
    models::{
        AccessTier, AccountKind, BlobClientAcquireLeaseOptions,
        BlobClientAcquireLeaseResultHeaders, BlobClientChangeLeaseResultHeaders,
        BlobClientDownloadOptions, BlobClientGetAccountInfoResultHeaders,
        BlobClientGetPropertiesOptions, BlobClientGetPropertiesResultHeaders,
        BlobClientSetImmutabilityPolicyOptions, BlobClientSetMetadataOptions,
        BlobClientSetPropertiesOptions, BlobClientSetTierOptions, BlobTags,
        BlockBlobClientUploadOptions, ImmutabilityPolicyMode, LeaseState, RehydratePriority,
        StorageErrorCode,
    },
    BlobClient, BlobClientOptions, BlobContainerClient, BlobContainerClientOptions, StorageError,
};
use azure_storage_blob_test::{
    create_test_blob, get_blob_name, get_container_client, get_container_name, ClientOptionsExt,
    StorageAccount, TestPolicy,
};
use bytes::{BufMut, BytesMut};
use flate2::{write::GzEncoder, Compression};
use futures::TryStreamExt;
use std::{
    cmp::min,
    collections::HashMap,
    error::Error,
    io::Write,
    num::NonZero,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::time;

#[recorded::test]
async fn test_get_blob_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));

    // Container Doesn't Exist Scenario
    let response = blob_client.get_properties(None).await;

    // Assert
    let error = response.unwrap_err().http_status();
    assert_eq!(StatusCode::NotFound, error.unwrap());
    assert!(!blob_client.exists().await?);

    container_client.create(None).await?;
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

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_set_blob_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
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

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_upload_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));

    let data = b"hello rusty world";

    // No Overwrite Scenario
    blob_client
        .upload(RequestContent::from(data.to_vec()), None)
        .await?;

    // Assert
    let response = blob_client.download(None).await?;
    assert_eq!(17, response.properties.content_length.unwrap());
    let body_data = response.body.collect().await?;
    assert_eq!(Bytes::from_static(data), body_data);

    // Overwrite Scenarios
    let new_data = b"hello overwritten rusty world";

    // Error Case (overwrite=false/none)
    let response = blob_client
        .upload(
            RequestContent::from(new_data.to_vec()),
            Some(BlockBlobClientUploadOptions::default().with_if_not_exists()),
        )
        .await;

    // Assert
    assert!(response.is_err());
    let error = response.unwrap_err().http_status();
    assert_eq!(StatusCode::Conflict, error.unwrap());

    // Working Case (overwrite=true)
    blob_client
        .upload(RequestContent::from(new_data.to_vec()), None)
        .await?;
    let response = blob_client.download(None).await?;
    // Assert
    assert_eq!(29, response.properties.content_length.unwrap());
    let body_data = response.body.collect().await?;
    assert_eq!(Bytes::from_static(new_data), body_data);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_delete_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    create_test_blob(&blob_client, None, None).await?;

    // Existence Check
    blob_client.get_properties(None).await?;

    blob_client.delete(None).await?;

    let response = blob_client.download(None).await;

    // Assert
    let error = response.unwrap_err().http_status();
    assert_eq!(StatusCode::NotFound, error.unwrap());

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_undelete_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
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

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_download_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let data = b"hello rusty world";

    blob_client
        .upload(RequestContent::from(data.to_vec()), None)
        .await?;
    let response = blob_client.download(None).await?;

    // Assert
    assert_eq!(17, response.properties.content_length.unwrap());
    let body_data = response.body.collect().await?;
    assert_eq!(b"hello rusty world".as_ref(), &body_data[..]);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_set_blob_metadata(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup

    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
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
            Some(options_with_metadata),
        )
        .await?;
    // Assert
    let response = blob_client.get_properties(None).await?;
    let response_metadata = response.metadata()?;
    assert_eq!(initial_metadata, response_metadata);

    // Set Metadata With Values
    let update_metadata = HashMap::from([("updated".to_string(), "values".to_string())]);
    blob_client.set_metadata(&update_metadata, None).await?;
    // Assert
    let response = blob_client.get_properties(None).await?;
    let response_metadata = response.metadata()?;
    assert_eq!(update_metadata, response_metadata);

    // Set Metadata No Values (Clear Metadata)
    blob_client.set_metadata(&HashMap::new(), None).await?;
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
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
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

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_blob_lease_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
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

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_leased_blob_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
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
        .set_metadata(&update_metadata, Some(set_metadata_options))
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
        .upload(RequestContent::from(data.to_vec()), Some(upload_options))
        .await?;

    // Assert
    let download_options = BlobClientDownloadOptions {
        lease_id: Some(lease_id.clone()),
        ..Default::default()
    };
    let response = blob_client.download(Some(download_options)).await?;
    assert_eq!(10, response.properties.content_length.unwrap());
    let body_data = response.body.collect().await?;
    assert_eq!(data.as_ref(), &body_data[..]);

    blob_client.break_lease(None).await?;
    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_blob_tags(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    recording.set_matcher(Matcher::BodilessMatcher).await?;
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    create_test_blob(&blob_client, None, None).await?;

    // Set Tags with Tags Specified
    let blob_tags = HashMap::from([
        ("hello".to_string(), "world".to_string()),
        ("ferris".to_string(), "crab".to_string()),
    ]);
    blob_client
        .set_tags(
            RequestContent::try_from(BlobTags::from(blob_tags.clone()))?,
            None,
        )
        .await?;

    // Assert
    let response_tags = blob_client.get_tags(None).await?.into_model()?;
    let map: HashMap<String, String> = response_tags.into();
    assert_eq!(blob_tags, map);

    // Set Tags with No Tags (Clear Tags)
    blob_client
        .set_tags(
            RequestContent::try_from(BlobTags::from(HashMap::new()))?,
            None,
        )
        .await?;

    // Assert
    let response_tags = blob_client.get_tags(None).await?.into_model()?;
    let map: HashMap<String, String> = response_tags.into();
    assert_eq!(HashMap::new(), map);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_get_account_info(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup

    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
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

    let container_name = get_container_name(recording);
    // Create Container & Container Client
    let container_client = BlobContainerClient::new(
        &endpoint,
        &container_name,
        Some(recording.credential()),
        Some(container_client_options.clone()),
    )?;
    container_client.create(None).await?;

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
            &container_name,
            blob_name,
            Some(recording.credential()),
            Some(blob_client_options.clone()),
        )?;

        // Upload Blob
        blob_client_new
            .upload(RequestContent::from(b"hello rusty world".to_vec()), None)
            .await?;

        // Get Properties
        let properties = blob_client_new.get_properties(None).await?;
        assert_eq!(17, properties.content_length()?.unwrap());

        // Test Case 2: Initialize BlobClient using from_blob_url(), separate path segments
        let mut blob_url = Url::parse(&endpoint)?;
        blob_url
            .path_segments_mut()
            .expect("Storage Endpoint must be a valid base URL with http/https scheme")
            .push(&container_name)
            .push(blob_name);

        let blob_client_from_url = BlobClient::from_url(
            blob_url,
            Some(recording.credential()),
            Some(blob_client_options.clone()),
        )?;

        // Upload Blob
        blob_client_from_url
            .upload(RequestContent::from(b"hello rusty world".to_vec()), None)
            .await?;

        // Get Properties
        let properties = blob_client_from_url.get_properties(None).await?;
        assert_eq!(17, properties.content_length()?.unwrap());

        // Test Case 3: Initialize BlobClient using ContainerClient accessor
        let blob_client_from_cc = container_client.blob_client(blob_name);

        // Upload Blob
        blob_client_from_cc
            .upload(RequestContent::from(b"hello rusty world".to_vec()), None)
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
        .map(|blob| blob.name.clone().unwrap())
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

    // container_client.delete(None).await?;

    Ok(())
}

#[recorded::test(playback)]
async fn test_set_legal_hold(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // This test requires legal hold feature enabled.
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    container_client.create(None).await?;
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
    // This test requires ImmutableStorageWithVersioning feature enabled.
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Versioned, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    container_client.create(None).await?;
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
async fn test_storage_error_model(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));

    // Act - Download a blob that doesn't exist (container exists but blob doesn't)
    let response = blob_client.download(None).await;
    let error_response = response.unwrap_err();
    let storage_error: StorageError = error_response.try_into()?;

    // Assert
    assert_eq!(storage_error.status_code, StatusCode::NotFound);
    assert_eq!(
        storage_error.error_code.as_ref(),
        Some(&StorageErrorCode::BlobNotFound)
    );
    assert!(
        storage_error
            .message
            .as_deref()
            .is_some_and(|m| m.starts_with("The specified blob does not exist.")),
        "Expected message to start with 'The specified blob does not exist.'"
    );
    assert!(
        storage_error.request_id.is_some(),
        "Expected request_id to be populated."
    );

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_storage_error_model_bodiless(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));

    // Act - get_properties returns a bodiless 404 response
    let response = blob_client.get_properties(None).await;
    let error_response = response.unwrap_err();
    let error_kind = error_response.kind();
    assert!(matches!(error_kind, ErrorKind::HttpResponse { .. }));
    let storage_error: StorageError = error_response.try_into()?;

    // Assert
    assert_eq!(storage_error.status_code, StatusCode::NotFound);
    assert_eq!(
        storage_error.message.as_deref(),
        Some("Not Found"),
        "Expected canonical reason phrase for bodiless response."
    );
    assert!(
        storage_error.request_id.is_some(),
        "Expected request_id to be populated from headers."
    );
    assert!(
        storage_error.additional_error_info.is_empty(),
        "Expected no additional_error_info for bodiless response."
    );

    Ok(())
}

#[recorded::test]
async fn test_storage_error_model_additional_info(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let source_blob_client = container_client.blob_client(&get_blob_name(recording));
    create_test_blob(&source_blob_client, None, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let blob_name = get_blob_name(recording);

    // Act
    let overwrite_blob_client = container_client.blob_client(&blob_name);
    create_test_blob(
        &overwrite_blob_client,
        Some(RequestContent::from(b"overruled!".to_vec())),
        None,
    )
    .await?;

    // Inject an erroneous 'c' so we raise Copy Source Errors
    let container_name = container_client
        .url()
        .path_segments()
        .and_then(|mut segments| segments.next())
        .unwrap();
    let overwrite_url = format!(
        "{}{}c/{}",
        overwrite_blob_client.url(),
        container_name,
        blob_name
    );

    // Copy Source Error Scenario
    let response = blob_client
        .block_blob_client()
        .upload_blob_from_url(overwrite_url.clone(), None)
        .await;

    let error = response.unwrap_err();
    assert_eq!(StatusCode::NotFound, error.http_status().unwrap());
    let storage_error: StorageError = error.try_into()?;

    // Assert
    assert_eq!(storage_error.status_code, StatusCode::NotFound);
    assert_eq!(
        storage_error.copy_source_status_code,
        Some(StatusCode::NotFound)
    );
    assert_eq!(
        storage_error.copy_source_error_code.as_deref(),
        Some("BlobNotFound")
    );
    assert_eq!(
        storage_error.copy_source_error_message.as_deref(),
        Some("The specified blob does not exist.")
    );

    container_client.delete(None).await?;
    Ok(())
}

struct TestManagedDownloadArgSet {
    data_len: usize,
    parallel: usize,
    partition_len: usize,
    download_range: Option<(usize, usize)>,
    expected_gets: usize,
}
fn test_managed_download_args() -> impl IntoIterator<Item = TestManagedDownloadArgSet> {
    const DATA_LEN: usize = 1024;
    [
        (2, DATA_LEN, None, 1),
        (2, DATA_LEN * 2, None, 1),
        (2, 512, None, 2),
        (1, 256, None, 4),
        (8, 31, None, 34),
        (1, 16, Some((0, 16)), 1),
        (4, 16, Some((16, 20)), 1),
        (4, 256, Some((0, 12345)), 4),
        (4, 100, Some((123, 223)), 1),
    ]
    .map(|(parallel, partition_len, download_range, expected_gets)| {
        TestManagedDownloadArgSet {
            data_len: DATA_LEN,
            parallel,
            partition_len,
            download_range,
            expected_gets,
        }
    })
}

#[recorded::test]
async fn test_managed_download(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let request_count = Arc::new(AtomicUsize::new(0));
    let count_policy = Arc::new(TestPolicy::count_requests(request_count.clone(), None));

    let recording = ctx.recording();
    let container_client = get_container_client(
        recording,
        true,
        StorageAccount::Standard,
        Some(BlobContainerClientOptions::default().with_per_call_policy(count_policy.clone())),
    )
    .await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));

    for TestManagedDownloadArgSet {
        data_len,
        parallel,
        partition_len,
        download_range,
        expected_gets,
    } in test_managed_download_args()
    {
        let data: Vec<u8> = (0..data_len).map(|_| recording.random()).collect();
        blob_client
            .upload(RequestContent::from(data.to_vec()), None)
            .await?;

        request_count.store(0, Ordering::Relaxed);
        let _scope = count_policy.check_request_scope();
        let mut download_stream = blob_client
            .download(Some(BlobClientDownloadOptions {
                partition_size: Some(NonZero::new(partition_len).unwrap()),
                parallel: Some(NonZero::new(parallel).unwrap()),
                range: download_range.map(|r| r.0..r.1),
                ..Default::default()
            }))
            .await?
            .body;

        let mut downloaded_data = BytesMut::new();
        while let Some(bytes) = download_stream.try_next().await? {
            downloaded_data.put(bytes);
        }
        let downloaded_data = downloaded_data.freeze();
        assert_eq!(
            &downloaded_data,
            match download_range {
                Some(r) => &data[r.0..min(r.1, data_len)],
                None => &data,
            }
        );
        assert_eq!(request_count.load(Ordering::Relaxed), expected_gets);
    }

    Ok(())
}

// TODO edge case where a range was requested on a 0-length blob
#[recorded::test]
async fn test_managed_download_empty(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let request_count = Arc::new(AtomicUsize::new(0));
    let count_policy = Arc::new(TestPolicy::count_requests(request_count.clone(), None));

    let recording = ctx.recording();
    let container_client = get_container_client(
        recording,
        true,
        StorageAccount::Standard,
        Some(BlobContainerClientOptions::default().with_per_call_policy(count_policy.clone())),
    )
    .await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));

    blob_client
        .upload(RequestContent::from(vec![]), None)
        .await?;

    request_count.store(0, Ordering::Relaxed);
    let _scope = count_policy.check_request_scope();
    let mut download_stream = blob_client.download(None).await?.body;

    let mut downloaded_data = BytesMut::new();
    while let Some(bytes) = download_stream.try_next().await? {
        downloaded_data.put(bytes);
    }
    let downloaded_data = downloaded_data.freeze();

    assert_eq!(downloaded_data.len(), 0);
    // 1 op with a range, 1 op without after the first one fails
    assert_eq!(request_count.load(Ordering::Relaxed), 2);

    Ok(())
}

#[recorded::test]
async fn test_upload_blob_content_headers(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let content = b"upload-content-headers";

    // Upload with Content Headers
    // Note: blob_content_md5 is validated against actual content on Put Blob and is excluded
    // here; it is tested as stored metadata via set_properties in test_set_properties_content_headers.
    blob_client
        .upload(
            RequestContent::from(content.to_vec()),
            Some(BlockBlobClientUploadOptions {
                blob_cache_control: Some("no-cache".to_string()),
                blob_content_disposition: Some("inline".to_string()),
                blob_content_encoding: Some("identity".to_string()),
                blob_content_language: Some("en-US".to_string()),
                blob_content_type: Some("image/png".to_string()),
                ..Default::default()
            }),
        )
        .await?;

    // Assert Content Headers via get_properties
    let props = blob_client.get_properties(None).await?;
    assert_eq!(Some("no-cache".to_string()), props.cache_control()?);
    assert_eq!(Some("inline".to_string()), props.content_disposition()?);
    assert_eq!(Some("identity".to_string()), props.content_encoding()?);
    assert_eq!(Some("en-US".to_string()), props.content_language()?);
    let content_type: Option<String> = props.headers().get_optional_as(&CONTENT_TYPE)?;
    assert_eq!(Some("image/png".to_string()), content_type);

    // Assert Content Headers on Download Response
    let response = blob_client.download(None).await?;
    assert_eq!(
        Some("no-cache".to_string()),
        response.properties.cache_control
    );
    assert_eq!(
        Some("inline".to_string()),
        response.properties.content_disposition
    );
    assert_eq!(
        Some("identity".to_string()),
        response.properties.content_encoding
    );
    assert_eq!(
        Some("en-US".to_string()),
        response.properties.content_language
    );
    let content_type: Option<String> = response.headers.get_optional_as(&CONTENT_TYPE)?;
    assert_eq!(Some("image/png".to_string()), content_type);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_set_blob_properties_content_headers(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let content = b"set-properties-content-headers";
    let md5: Vec<u8> = (0u8..16).collect();

    // Upload with Default Content Headers
    blob_client
        .upload(RequestContent::from(content.to_vec()), None)
        .await?;

    // Set All Content Headers via Set Properties
    blob_client
        .set_properties(Some(BlobClientSetPropertiesOptions {
            blob_cache_control: Some("no-store".to_string()),
            blob_content_disposition: Some("attachment; filename=\"file.txt\"".to_string()),
            blob_content_encoding: Some("identity".to_string()),
            blob_content_language: Some("fr-FR".to_string()),
            blob_content_md5: Some(md5.clone()),
            blob_content_type: Some("application/pdf".to_string()),
            ..Default::default()
        }))
        .await?;

    // Assert Content Headers Roundtrip
    let props = blob_client.get_properties(None).await?;
    assert_eq!(Some("no-store".to_string()), props.cache_control()?);
    assert_eq!(
        Some("attachment; filename=\"file.txt\"".to_string()),
        props.content_disposition()?
    );
    assert_eq!(Some("identity".to_string()), props.content_encoding()?);
    assert_eq!(Some("fr-FR".to_string()), props.content_language()?);
    assert_eq!(Some(md5), props.content_md5()?);
    let content_type: Option<String> = props.headers().get_optional_as(&CONTENT_TYPE)?;
    assert_eq!(Some("application/pdf".to_string()), content_type);

    // Overwrite with Only Content-Type - all other headers absent, so service clears them
    // Note: set_properties does not validate blob_content_md5 against actual content,
    // so we can use an arbitrary value to test storage and clearing behavior.
    blob_client
        .set_properties(Some(BlobClientSetPropertiesOptions {
            blob_content_type: Some("image/gif".to_string()),
            ..Default::default()
        }))
        .await?;

    // Assert Only Content-Type Remains; Others Are Cleared
    let props = blob_client.get_properties(None).await?;
    assert_eq!(None, props.cache_control()?);
    assert_eq!(None, props.content_disposition()?);
    assert_eq!(None, props.content_encoding()?);
    assert_eq!(None, props.content_language()?);
    assert_eq!(None, props.content_md5()?);
    let content_type: Option<String> = props.headers().get_optional_as(&CONTENT_TYPE)?;
    assert_eq!(Some("image/gif".to_string()), content_type);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
#[ignore = "need to investigate live test pipeline failures"]
async fn test_upload_blob_overwrite_content_headers(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let content_v1 = b"overwrite-headers-v1";
    let content_v2 = b"overwrite-headers-v2";

    // Upload v1 with Initial Content Headers
    blob_client
        .upload(
            RequestContent::from(content_v1.to_vec()),
            Some(BlockBlobClientUploadOptions {
                blob_cache_control: Some("no-cache".to_string()),
                blob_content_type: Some("application/octet-stream".to_string()),
                ..Default::default()
            }),
        )
        .await?;

    // Overwrite v2 with Different Content Headers
    blob_client
        .upload(
            RequestContent::from(content_v2.to_vec()),
            Some(BlockBlobClientUploadOptions {
                blob_cache_control: Some("max-age=3600".to_string()),
                blob_content_type: Some("application/json".to_string()),
                ..Default::default()
            }),
        )
        .await?;

    // Assert v2 Headers Replace v1 Headers
    let props = blob_client.get_properties(None).await?;
    assert_eq!(Some("max-age=3600".to_string()), props.cache_control()?);
    let content_type: Option<String> = props.headers().get_optional_as(&CONTENT_TYPE)?;
    assert_eq!(Some("application/json".to_string()), content_type);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
#[ignore = "need to investigate live test pipeline failures"]
async fn test_acquire_lease_with_proposed_id(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    create_test_blob(&blob_client, None, None).await?;

    let proposed_id = "00000000-1111-2222-3333-444444444444".to_string();

    // Acquire With Proposed Lease ID Scenario
    let response = blob_client
        .acquire_lease(
            15,
            Some(BlobClientAcquireLeaseOptions {
                proposed_lease_id: Some(proposed_id.clone()),
                ..Default::default()
            }),
        )
        .await?;

    // Assert
    assert_eq!(proposed_id, response.lease_id()?.unwrap());

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
#[ignore = "need to investigate live test pipeline failures"]
async fn test_blob_error_codes(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));

    // BlobNotFound - get_properties on a blob that doesn't exist
    let err = blob_client.get_properties(None).await.unwrap_err();
    let storage_error: StorageError = err.try_into()?;
    assert_eq!(
        storage_error.error_code.as_ref(),
        Some(&StorageErrorCode::BlobNotFound),
        "expected BlobNotFound error code"
    );

    // Upload once so the blob exists
    create_test_blob(&blob_client, None, None).await?;

    // BlobAlreadyExists - upload again with overwrite=false
    let err = blob_client
        .upload(
            RequestContent::from(b"duplicate".to_vec()),
            Some(BlockBlobClientUploadOptions::default().with_if_not_exists()),
        )
        .await
        .unwrap_err();
    let storage_error: StorageError = err.try_into()?;
    assert_eq!(
        storage_error.error_code.as_ref(),
        Some(&StorageErrorCode::BlobAlreadyExists),
        "expected BlobAlreadyExists error code"
    );

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
#[ignore = "need to investigate live test pipeline failures"]
async fn test_set_tier_rehydrate_priority(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    create_test_blob(&blob_client, None, None).await?;

    // Move blob to Archive tier
    blob_client.set_tier(AccessTier::Archive, None).await?;

    // Rehydrate to Hot with High Priority Scenario
    blob_client
        .set_tier(
            AccessTier::Hot,
            Some(BlobClientSetTierOptions {
                rehydrate_priority: Some(RehydratePriority::High),
                ..Default::default()
            }),
        )
        .await?;

    // Assert
    let response = blob_client.get_properties(None).await?;
    assert_eq!(
        Some(RehydratePriority::High),
        response.rehydrate_priority()?
    );

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_gzip_blob_no_metadata_roundtrip(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));

    // Compress plaintext and upload with no content metadata.
    let plaintext = b"hello world";
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(plaintext)?;
    let gzip_bytes = encoder.finish()?;

    blob_client
        .upload(RequestContent::from(gzip_bytes.clone()), None)
        .await?;

    // Download with the default client.
    let response = blob_client.download(None).await?;

    // No content-encoding means no decompression; body is raw gzip.
    assert_eq!(
        None, response.properties.content_encoding,
        "content-encoding must be absent: the service did not inject a content-coding header"
    );
    let downloaded_bytes = response.body.collect().await?;
    assert_eq!(
        gzip_bytes,
        downloaded_bytes.to_vec(),
        "body must be the raw gzip stream unchanged"
    );

    // Verify the raw bytes gunzip back to the original plaintext.
    let mut decoder = flate2::read::GzDecoder::new(downloaded_bytes.as_ref());
    let mut decompressed = Vec::new();
    std::io::Read::read_to_end(&mut decoder, &mut decompressed)?;
    assert_eq!(plaintext.to_vec(), decompressed);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_gzip_blob_with_metadata_roundtrip(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));

    // Compress plaintext and upload with blob_content_encoding: gzip.
    let plaintext = b"hello gzip world";
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(plaintext)?;
    let gzip_bytes = encoder.finish()?;

    blob_client
        .upload(
            RequestContent::from(gzip_bytes.clone()),
            Some(BlockBlobClientUploadOptions {
                blob_content_encoding: Some("gzip".to_string()),
                ..Default::default()
            }),
        )
        .await?;

    // Download - auto-decompression is disabled so we get raw bytes.
    let response = blob_client.download(None).await?;

    // content-encoding header is present because reqwest did not strip it.
    assert_eq!(
        Some("gzip".to_string()),
        response.properties.content_encoding,
        "content-encoding: gzip must be present when auto-decompression is disabled"
    );

    let downloaded_bytes = response.body.collect().await?;
    // The body is a valid gzip stream that decodes back to the original plaintext.
    // Note: we compare decompressed output rather than raw bytes because the service
    // may normalize the OS byte in the gzip header.
    let mut decoder = flate2::read::GzDecoder::new(downloaded_bytes.as_ref());
    let mut decompressed = Vec::new();
    std::io::Read::read_to_end(&mut decoder, &mut decompressed)?;
    assert_eq!(
        plaintext.to_vec(),
        decompressed,
        "decompressed body must match original plaintext"
    );

    container_client.delete(None).await?;
    Ok(())
}
