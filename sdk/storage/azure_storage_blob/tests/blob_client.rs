// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    http::{RequestContent, StatusCode, Url},
    Bytes,
};
use azure_core_test::{recorded, Matcher, TestContext, TestMode};
use azure_storage_blob::{
    models::{
        AccessTier, AccountKind, BlobClientAcquireLeaseResultHeaders,
        BlobClientChangeLeaseResultHeaders, BlobClientDownloadOptions,
        BlobClientDownloadResultHeaders, BlobClientGetAccountInfoResultHeaders,
        BlobClientGetPropertiesOptions, BlobClientGetPropertiesResultHeaders,
        BlobClientSetMetadataOptions, BlobClientSetPropertiesOptions, BlobClientSetTierOptions,
        BlockBlobClientUploadOptions, ContainerClientCreateOptions, LeaseState, PublicAccessType,
    },
    BlobClient,
};

use azure_storage_blob_test::{
    create_test_blob, get_blob_name, get_blob_service_client, get_container_client,
};
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
        response_body.collect().await?
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
    assert_eq!(data.to_vec(), response_body.collect().await?);

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
async fn test_public_access(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Mark as playback-only
    if ctx.recording().test_mode() != TestMode::Playback {
        return Ok(());
    }

    // Arrange
    let recording = ctx.recording();
    let container_client = get_container_client(recording, false).await?;
    let blob_client = container_client.blob_client(get_blob_name(recording));

    let public_access_create_options = ContainerClientCreateOptions {
        access: Some(PublicAccessType::Blob),
        ..Default::default()
    };
    container_client
        .create_container(Some(public_access_create_options))
        .await?;

    create_test_blob(&blob_client, None, None).await?;

    // Unauthenticated Blob Client
    let endpoint = format!(
        "https://{}.blob.core.windows.net/",
        recording.var("AZURE_STORAGE_ACCOUNT_NAME", None).as_str()
    );
    let unauthenticated_blob_client = BlobClient::new(
        Url::parse(&endpoint)?,
        blob_client.container_name().to_string(),
        blob_client.blob_name().to_string(),
        None,
        None,
    )?;

    // Act
    let response = unauthenticated_blob_client.get_properties(None).await?;

    // Assert
    let lease_state = response.lease_state()?;
    let content_length = response.content_length()?;
    let etag = response.etag()?;
    let creation_time = response.creation_time()?;

    assert_eq!(LeaseState::Available, lease_state.unwrap());
    assert_eq!(17, content_length.unwrap());
    assert!(etag.is_some());
    assert!(creation_time.is_some());
    assert!(unauthenticated_blob_client.exists().await?);

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_encoding_edge_cases(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let service_client = get_blob_service_client(recording)?;
    let blob_url = format!(
        "https://{}.blob.core.windows.net/",
        recording.var("AZURE_STORAGE_ACCOUNT_NAME", None).as_str()
    );

    // [Simple Case - Baseline]
    let test_names_simple = ("test-container-simple", "test_blob_encoding");
    let container_client_1 = service_client.blob_container_client(test_names_simple.0.into());
    let blob_client_1 = container_client_1.blob_client(test_names_simple.1.into());
    container_client_1.create_container(None).await?;
    create_test_blob(&blob_client_1, None, None).await?;
    blob_client_1.get_properties(None).await?;
    assert_eq!(test_names_simple.0, container_client_1.container_name());
    assert_eq!(test_names_simple.0, blob_client_1.container_name());
    assert_eq!(test_names_simple.1, blob_client_1.blob_name());

    let blob_client_1_manual = BlobClient::new(
        Url::parse(&blob_url)?,
        test_names_simple.0.into(),
        test_names_simple.1.into(),
        Some(recording.credential()),
        None,
    )?;
    blob_client_1_manual.get_properties(None).await?;
    assert_eq!(test_names_simple.0, blob_client_1_manual.container_name());
    assert_eq!(test_names_simple.1, blob_client_1_manual.blob_name());

    // [Comprehensive Space Handling - leading, trailing, consecutive, and embedded]
    let test_names_spaces = (
        "test-container-spaces",
        " leading  with   multiple   spaces trailing ",
    );
    let container_client_2 = service_client.blob_container_client(test_names_spaces.0.into());
    let blob_client_2 = container_client_2.blob_client(test_names_spaces.1.into());
    container_client_2.create_container(None).await?;
    create_test_blob(&blob_client_2, None, None).await?;
    blob_client_2.get_properties(None).await?;
    assert_eq!(test_names_spaces.0, container_client_2.container_name());
    assert_eq!(test_names_spaces.0, blob_client_2.container_name());
    assert_eq!(test_names_spaces.1, blob_client_2.blob_name());

    let blob_client_2_manual = BlobClient::new(
        Url::parse(&blob_url)?,
        test_names_spaces.0.into(),
        test_names_spaces.1.into(),
        Some(recording.credential()),
        None,
    )?;
    blob_client_2_manual.get_properties(None).await?;
    assert_eq!(test_names_spaces.0, blob_client_2_manual.container_name());
    assert_eq!(test_names_spaces.1, blob_client_2_manual.blob_name());

    // [URL-Unsafe and Delimiter Characters - &, ?, =, #, ;, comma, @, |]
    let test_names_unsafe = (
        "test-container-unsafe",
        "file&param?query=val#frag;ver,data@email|pipe.txt",
    );
    let container_client_3 = service_client.blob_container_client(test_names_unsafe.0.into());
    let blob_client_3 = container_client_3.blob_client(test_names_unsafe.1.into());
    container_client_3.create_container(None).await?;
    create_test_blob(&blob_client_3, None, None).await?;
    blob_client_3.get_properties(None).await?;
    assert_eq!(test_names_unsafe.0, container_client_3.container_name());
    assert_eq!(test_names_unsafe.0, blob_client_3.container_name());
    assert_eq!(test_names_unsafe.1, blob_client_3.blob_name());

    let blob_client_3_manual = BlobClient::new(
        Url::parse(&blob_url)?,
        test_names_unsafe.0.into(),
        test_names_unsafe.1.into(),
        Some(recording.credential()),
        None,
    )?;
    blob_client_3_manual.get_properties(None).await?;
    assert_eq!(test_names_unsafe.0, blob_client_3_manual.container_name());
    assert_eq!(test_names_unsafe.1, blob_client_3_manual.blob_name());

    // [Path Separators - forward slashes, backslashes mixed, and encoded forward slash]
    let test_names_paths = (
        "test-container-paths",
        "folder/subfolder\\file/mixed\\paths%2Fencoded.txt",
    );
    let container_client_4 = service_client.blob_container_client(test_names_paths.0.into());
    let blob_client_4 = container_client_4.blob_client(test_names_paths.1.into());
    container_client_4.create_container(None).await?;
    create_test_blob(&blob_client_4, None, None).await?;
    blob_client_4.get_properties(None).await?;
    assert_eq!(test_names_paths.0, container_client_4.container_name());
    assert_eq!(test_names_paths.0, blob_client_4.container_name());
    assert_eq!(test_names_paths.1, blob_client_4.blob_name());

    let blob_client_4_manual = BlobClient::new(
        Url::parse(&blob_url)?,
        test_names_paths.0.into(),
        test_names_paths.1.into(),
        Some(recording.credential()),
        None,
    )?;
    blob_client_4_manual.get_properties(None).await?;
    assert_eq!(test_names_paths.0, blob_client_4_manual.container_name());
    assert_eq!(test_names_paths.1, blob_client_4_manual.blob_name());

    // [Percent Encoding - literal %, already-encoded, and mixed encoding]
    let test_names_percent = (
        "test-container-percent",
        "50%off-%20encoded-my%20file (2).txt",
    );
    let container_client_5 = service_client.blob_container_client(test_names_percent.0.into());
    let blob_client_5 = container_client_5.blob_client(test_names_percent.1.into());
    container_client_5.create_container(None).await?;
    create_test_blob(&blob_client_5, None, None).await?;
    blob_client_5.get_properties(None).await?;
    assert_eq!(test_names_percent.0, container_client_5.container_name());
    assert_eq!(test_names_percent.0, blob_client_5.container_name());
    assert_eq!(test_names_percent.1, blob_client_5.blob_name());

    let blob_client_5_manual = BlobClient::new(
        Url::parse(&blob_url)?,
        test_names_percent.0.into(),
        test_names_percent.1.into(),
        Some(recording.credential()),
        None,
    )?;
    blob_client_5_manual.get_properties(None).await?;
    assert_eq!(test_names_percent.0, blob_client_5_manual.container_name());
    assert_eq!(test_names_percent.1, blob_client_5_manual.blob_name());

    // [Special Characters - brackets, braces, quotes, apostrophes, angle brackets, asterisks, starting/ending with special chars, consecutive special chars]
    let test_names_special = (
        "test-container-special",
        "***file[1]''test''_with...special~~~chars<<v2>>{{{copy}}}***.txt!!!",
    );
    let container_client_6 = service_client.blob_container_client(test_names_special.0.into());
    let blob_client_6 = container_client_6.blob_client(test_names_special.1.into());
    container_client_6.create_container(None).await?;
    create_test_blob(&blob_client_6, None, None).await?;
    blob_client_6.get_properties(None).await?;
    assert_eq!(test_names_special.0, container_client_6.container_name());
    assert_eq!(test_names_special.0, blob_client_6.container_name());
    assert_eq!(test_names_special.1, blob_client_6.blob_name());

    let blob_client_6_manual = BlobClient::new(
        Url::parse(&blob_url)?,
        test_names_special.0.into(),
        test_names_special.1.into(),
        Some(recording.credential()),
        None,
    )?;
    blob_client_6_manual.get_properties(None).await?;
    assert_eq!(test_names_special.0, blob_client_6_manual.container_name());
    assert_eq!(test_names_special.1, blob_client_6_manual.blob_name());

    // [Advanced Encoding - unicode, emojis, accents, multi-byte chars, plus signs, form encoding]
    let test_names_advanced = (
        "test-container-advanced",
        "café+🦀+カニのフェリス~émoji+plus~tilde.txt",
    );
    let container_client_7 = service_client.blob_container_client(test_names_advanced.0.into());
    let blob_client_7 = container_client_7.blob_client(test_names_advanced.1.into());
    container_client_7.create_container(None).await?;
    create_test_blob(&blob_client_7, None, None).await?;
    blob_client_7.get_properties(None).await?;
    assert_eq!(test_names_advanced.0, container_client_7.container_name());
    assert_eq!(test_names_advanced.0, blob_client_7.container_name());
    assert_eq!(test_names_advanced.1, blob_client_7.blob_name());

    let blob_client_7_manual = BlobClient::new(
        Url::parse(&blob_url)?,
        test_names_advanced.0.into(),
        test_names_advanced.1.into(),
        Some(recording.credential()),
        None,
    )?;
    blob_client_7_manual.get_properties(None).await?;
    assert_eq!(test_names_advanced.0, blob_client_7_manual.container_name());
    assert_eq!(test_names_advanced.1, blob_client_7_manual.blob_name());

    // Cleanup all containers
    container_client_1.delete_container(None).await?;
    container_client_2.delete_container(None).await?;
    container_client_3.delete_container(None).await?;
    container_client_4.delete_container(None).await?;
    container_client_5.delete_container(None).await?;
    container_client_6.delete_container(None).await?;
    container_client_7.delete_container(None).await?;

    Ok(())
}

// #[recorded::test]
// async fn test_sas(ctx: TestContext) -> Result<(), Box<dyn Error>> {
//     // SAS
//     let blob_url = "<VALID_SAS_URL>";

//     let sas_blob_client = BlobClient::from_blob_url(blob_url, None, None)?;
//     println!(
//         "Container Name:{}, Blob Name:{}",
//         sas_blob_client.container_name(),
//         sas_blob_client.blob_name()
//     );

//     let blob_properties = sas_blob_client.get_properties(None).await?;
//     let content_length = blob_properties.content_length()?;
//     assert_eq!(11, content_length.unwrap());

//     Ok(())
// }
