// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::{RequestContent, StatusCode};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::models::{
    BlobClientAcquireLeaseResultHeaders, BlobClientCreateSnapshotOptions,
    BlobClientCreateSnapshotResultHeaders, BlobClientDeleteOptions, BlobClientDownloadOptions,
    BlobClientGetPropertiesResultHeaders, BlobClientSetPropertiesOptions,
    BlobContainerClientListBlobsOptions, BlobTags, BlockBlobClientUploadOptions,
    DeleteSnapshotsOptionType, ListBlobsIncludeItem,
};
use azure_storage_blob_test::{
    create_test_blob, get_blob_name, get_container_client, StorageAccount,
};
use futures::TryStreamExt;
use std::{collections::HashMap, error::Error};

#[recorded::test]
async fn test_blob_snapshot_basic_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
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

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_blob_snapshot_metadata_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
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
    blob_client.set_metadata(&new_base_metadata, None).await?;

    // Verify Snapshots Unchanged
    let props_1 = snapshot_1_client.get_properties(None).await?;
    assert_eq!(base_metadata, props_1.metadata()?);
    let props_2 = snapshot_2_client.get_properties(None).await?;
    assert_eq!(snapshot_metadata, props_2.metadata()?);

    // Verify Base Blob Has New Metadata
    let base_props = blob_client.get_properties(None).await?;
    assert_eq!(new_base_metadata, base_props.metadata()?);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs_with_snapshots(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

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
    let list_options = BlobContainerClientListBlobsOptions {
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
        let name = blob_item.name.as_ref().unwrap();
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

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_blob_snapshot_delete_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
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

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_blob_snapshot_conditional_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
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
        if_match: Some(etag.into()),
        ..Default::default()
    };
    let conditional_snapshot = blob_client
        .create_snapshot(Some(conditional_options))
        .await?;
    assert!(conditional_snapshot.snapshot()?.is_some());

    // Test Blob Tags Behavior with Snapshots
    let tags = HashMap::from([("test_key".to_string(), "test_value".to_string())]);
    blob_client
        .set_tags(
            RequestContent::try_from(BlobTags::from(tags.clone()))?,
            None,
        )
        .await?;

    // Verify Tags on Base Blob
    let response_tags = blob_client.get_tags(None).await?.into_model()?;
    let retrieved_tags: HashMap<String, String> = response_tags.into();
    assert_eq!(tags, retrieved_tags);

    // Verify Snapshot Does NOT Have Tags (Tags Not Inherited)
    let snapshot_tags = snapshot_client.get_tags(None).await?.into_model()?;
    let snapshot_tag_map: HashMap<String, String> = snapshot_tags.into();
    assert_eq!(HashMap::new(), snapshot_tag_map);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_blob_snapshot_error_cases(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
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
    let result = snapshot_client.set_metadata(&metadata, None).await;
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

    container_client.delete(None).await?;
    Ok(())
}
