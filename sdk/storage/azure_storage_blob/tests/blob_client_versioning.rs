// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    http::{RequestContent, StatusCode},
    time::{parse_rfc3339, to_rfc3339, OffsetDateTime},
};
use azure_core_test::{recorded, TestContext, VarOptions};
use azure_storage_blob::models::{
    AccessTier, BlobClientAcquireLeaseResultHeaders, BlobClientDeleteOptions,
    BlobClientDownloadOptions, BlobClientGetPropertiesOptions,
    BlobClientGetPropertiesResultHeaders, BlobClientSetImmutabilityPolicyOptions,
    BlobClientSetTierOptions, BlobContainerClientListBlobsOptions, BlobTags,
    BlockBlobClientUploadOptions, ListBlobsIncludeItem,
};
use azure_storage_blob_test::{
    create_test_blob, get_blob_name, get_container_client, StorageAccount,
};
use futures::TryStreamExt;
use std::{collections::HashMap, error::Error, time::Duration};

#[recorded::test]
async fn test_blob_version_read_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Versioned, None).await?;
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

    // Download Version 1 Using with_version()
    let version_1_client = blob_client.with_version(&version_1)?;
    let download_response = version_1_client.download(None).await?;
    let (status_code, _, response_body) = download_response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(data_v1.to_vec(), response_body.collect().await?.to_vec());

    // Download Version 1 Using Options (Test query parameter replaces)

    // Create blob_client w/ version_2 with intention to actually download version_1 with options bag
    let version_2_client = blob_client.with_version(&version_2)?;
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

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_blob_version_metadata_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Versioned, None).await?;
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
    blob_client.set_metadata(&metadata_v2, None).await?;
    let response = blob_client.get_properties(None).await?;
    let version_2 = response.version_id()?.unwrap();

    // Verify metadata matches corresponding version
    let version_1_client = blob_client.with_version(&version_1)?;
    let props_v1 = version_1_client.get_properties(None).await?;
    assert_eq!(metadata_v1, props_v1.metadata()?);
    let version_2_client = blob_client.with_version(&version_2)?;
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
    blob_client
        .set_tags(
            RequestContent::try_from(BlobTags::from(tags.clone()))?,
            None,
        )
        .await?;

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

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_blob_version_tier_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Versioned, None).await?;
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
    let version_1_client = blob_client.with_version(&version_1)?;
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
    let version_2_client = blob_client.with_version(&version_2)?;
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

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs_with_versions(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Versioned, None).await?;

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
    let list_options = BlobContainerClientListBlobsOptions {
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
        let name = blob_item.name.as_ref().unwrap();
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

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_blob_version_feature_interactions(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Versioned, None).await?;
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
    let lease_version_1_client = lease_blob_client.with_version(&lease_version_1)?;
    let props = lease_version_1_client.get_properties(None).await?;
    assert_eq!(2, props.content_length()?.unwrap());

    // Release Lease
    lease_blob_client.release_lease(lease_id, None).await?;

    // Test: Conditional Operation with Version
    let etag = props.etag()?.unwrap();
    let get_options = BlobClientGetPropertiesOptions {
        if_match: Some(etag.into()),
        version_id: Some(lease_version_1.clone()),
        ..Default::default()
    };
    let conditional_response = lease_blob_client.get_properties(Some(get_options)).await?;
    assert_eq!(2, conditional_response.content_length()?.unwrap());

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test(playback)]
async fn test_blob_version_immutability_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Versioned, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    container_client.create(None).await?;

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
    let version_1_client = blob_client.with_version(&version_1)?;
    version_1_client.set_legal_hold(true, None).await?;
    let props_v1 = version_1_client.get_properties(None).await?;
    assert!(props_v1.legal_hold()?.unwrap());

    // Verify Version 2 Does Not Have Legal Hold
    let version_2_client = blob_client.with_version(&version_2)?;
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
    let container_client =
        get_container_client(recording, true, StorageAccount::Versioned, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));

    // Create a Blob with One Version
    create_test_blob(&blob_client, None, None).await?;
    let response = blob_client.get_properties(None).await?;
    let valid_version = response.version_id()?.unwrap();

    // Test: Invalid Version ID Format
    let invalid_version_client = blob_client.with_version("invalid-version-id")?;
    let result = invalid_version_client.get_properties(None).await;
    assert!(result.is_err());

    // Test: Non-Existent Version ID
    let fake_version = "2000-05-11T00:00:00.0000000Z";
    let fake_version_client = blob_client.with_version(fake_version)?;
    let result = fake_version_client.get_properties(None).await;
    assert!(result.is_err());
    let error = result.unwrap_err().http_status();
    assert!(error == Some(StatusCode::NotFound));

    // Test: Delete Non-Current Version and Verify It's Gone
    create_test_blob(&blob_client, Some(RequestContent::from(b"v2".into())), None).await?;
    let version_1_client = blob_client.with_version(&valid_version)?;
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

    container_client.delete(None).await?;
    Ok(())
}
