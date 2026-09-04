// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![cfg(not(feature = "arrow"))]

mod common;

use azure_core::http::RequestContent;
use azure_core::time::{parse_rfc3339, to_rfc3339, OffsetDateTime};
use azure_core_test::{recorded, TestContext};
use azure_core_test::{TestMode, VarOptions};
use azure_storage_blob::models::{
    AccessTier, ArchiveStatus, BlobClientGetPropertiesResultHeaders,
    BlobClientSetImmutabilityPolicyOptions, BlobClientSetTierOptions,
    BlobContainerClientListBlobsHierarchicalOptions, CopyStatus, ImmutabilityPolicyMode,
    LeaseDuration, LeaseState, LeaseStatus, PageBlobClientSetSequenceNumberOptions,
    RehydratePriority, SequenceNumberActionType,
};
use azure_storage_blob::models::{
    BlobContainerClientListBlobsOptions, BlobType, BlockBlobClientUploadOptions,
    ListBlobsIncludeItem,
};
use common::{create_test_blob, get_blob_name, get_container_client, StorageAccount};
use common::{get_blob_service_client, get_valid_encryption_scope, list_blobs_with_include};
use futures::{StreamExt, TryStreamExt};
use std::time::Duration;
use std::{collections::HashMap, error::Error};
use tokio::time;

#[recorded::test]
async fn test_list_blobs_xml(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;
    let blob_names = ["testblob1".to_string(), "testblob2".to_string()];

    container_client.create(None).await?;
    create_test_blob(
        &container_client.blob_client(&blob_names[0].clone()),
        None,
        None,
    )
    .await?;
    create_test_blob(
        &container_client.blob_client(&blob_names[1].clone()),
        None,
        None,
    )
    .await?;

    let mut list_blobs_response = container_client.list_blobs(None)?.into_pages();

    let page = list_blobs_response.try_next().await?;
    let list_blob_segment_response = page.unwrap().into_model()?;
    let blob_list = list_blob_segment_response.blob_items;
    for blob in blob_list {
        let blob_name = blob.name.unwrap();
        let properties = blob.properties.unwrap();
        let blob_type = properties.blob_type.unwrap();
        let etag = properties.etag;
        assert!(blob_names.contains(&blob_name));
        assert_eq!(BlobType::BlockBlob, blob_type);
        assert!(etag.is_some());
    }

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs_populates_properties_xml(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;
    container_client.create(None).await?;

    // Arrange: upload a blob populating as many listable properties as possible.
    let blob_name = get_blob_name(recording);
    let payload = b"arrow phase a payload".to_vec();
    // Base64 MD5 of `payload`; the service validates x-ms-blob-content-md5 against the body.
    let content_md5 = azure_core::base64::decode("IU+6Y1iDGdD2YaCH1kdRpg==")?;
    let metadata = HashMap::from([("team".to_string(), "sdk".to_string())]);
    let upload_options = BlockBlobClientUploadOptions {
        blob_cache_control: Some("max-age=3600".to_string()),
        blob_content_disposition: Some("inline".to_string()),
        blob_content_encoding: Some("gzip".to_string()),
        blob_content_language: Some("en-US".to_string()),
        blob_content_md5: Some(content_md5.clone()),
        blob_content_type: Some("text/plain".to_string()),
        metadata: Some(metadata.clone()),
        tier: Some(AccessTier::Hot),
        ..Default::default()
    }
    .with_tags(HashMap::from([("env".to_string(), "test".to_string())]));
    create_test_blob(
        &container_client.blob_client(&blob_name),
        Some(RequestContent::from(payload.clone())),
        Some(upload_options),
    )
    .await?;

    // Act: list blobs with metadata and tags included.
    let items: Vec<_> = container_client
        .list_blobs(Some(BlobContainerClientListBlobsOptions {
            include: Some(vec![
                ListBlobsIncludeItem::Metadata,
                ListBlobsIncludeItem::Tags,
            ]),
            ..Default::default()
        }))?
        .try_collect()
        .await?;

    // Assert: the scalar/timestamp/enum properties round-trip through the mapping.
    let blob = items
        .iter()
        .find(|b| b.name.as_deref() == Some(blob_name.as_str()))
        .expect("expected uploaded blob in listing");
    let props = blob.properties.as_ref().expect("expected blob properties");

    assert_eq!(Some(BlobType::BlockBlob), props.blob_type);
    assert!(props.etag.is_some());
    assert_eq!(Some(payload.len() as u64), props.content_length);
    assert_eq!(Some("text/plain".to_string()), props.content_type);
    assert_eq!(Some("gzip".to_string()), props.content_encoding);
    assert_eq!(Some("en-US".to_string()), props.content_language);
    assert_eq!(Some("inline".to_string()), props.content_disposition);
    assert_eq!(Some("max-age=3600".to_string()), props.cache_control);
    assert_eq!(Some(content_md5), props.content_md5);
    assert!(props.creation_time.is_some());
    assert!(props.last_modified.is_some());
    assert!(props.access_tier.is_some());
    assert!(props.access_tier_change_time.is_some());
    assert_eq!(Some(LeaseState::Available), props.lease_state);
    assert_eq!(Some(LeaseStatus::Unlocked), props.lease_status);
    assert_eq!(Some(true), props.server_encrypted);
    assert_eq!(Some(1), props.tag_count);

    // Map-valued metadata and tags are populated from the response.
    let blob_meta = blob
        .metadata
        .as_ref()
        .expect("metadata should be populated");
    assert_eq!(Some(&metadata), blob_meta.values.as_ref());
    let tags = blob
        .blob_tags
        .as_ref()
        .expect("blob_tags should be populated")
        .blob_tag_set
        .as_ref()
        .expect("tag set should be present");
    assert!(tags
        .iter()
        .any(|t| t.key.as_deref() == Some("env") && t.value.as_deref() == Some("test")));

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs_hierarchical_xml(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;
    container_client.create(None).await?;

    // Arrange: two blobs under a virtual directory plus one at the container root.
    for name in ["dir1/a.txt", "dir1/b.txt", "top.txt"] {
        create_test_blob(&container_client.blob_client(name), None, None).await?;
    }

    // Act: list hierarchically, grouping the directory with "/".
    // Paging is used here because virtual-directory prefixes are only exposed on the page
    // envelope, not through the item iterator.
    let page = container_client
        .list_blobs_hierarchical(
            "/",
            Some(BlobContainerClientListBlobsHierarchicalOptions {
                ..Default::default()
            }),
        )?
        .into_pages()
        .try_next()
        .await?
        .expect("expected a page")
        .into_model()?;

    // Assert: the directory collapses into a BlobPrefix and the root blob is listed.
    let prefixes = page
        .hierarchical_list
        .blob_prefixes
        .expect("expected blob prefixes");
    assert!(prefixes.iter().any(|p| p.name.as_deref() == Some("dir1/")));
    assert!(page
        .hierarchical_list
        .blob_items
        .iter()
        .any(|b| b.name.as_deref() == Some("top.txt")));

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs_stateful_properties_xml(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    // Blob with an infinite lease -> Leased / Locked / Infinite.
    let leased_name = get_blob_name(recording);
    let leased_client = container_client.blob_client(&leased_name);
    create_test_blob(&leased_client, None, None).await?;
    leased_client.acquire_lease(-1, None).await?;

    // Sealed append blob -> is_sealed, blob_type = AppendBlob.
    let sealed_name = get_blob_name(recording);
    let append_client = container_client
        .blob_client(&sealed_name)
        .append_blob_client();
    append_client.create(None).await?;
    append_client.seal(None).await?;

    // Page blob with a sequence number -> blob_sequence_number, blob_type = PageBlob.
    let page_name = get_blob_name(recording);
    let page_client = container_client.blob_client(&page_name).page_blob_client();
    page_client.create(1024, None).await?;
    page_client
        .set_sequence_number(
            SequenceNumberActionType::Update,
            Some(PageBlobClientSetSequenceNumberOptions {
                blob_sequence_number: Some(7),
                ..Default::default()
            }),
        )
        .await?;

    // Blob with a snapshot -> snapshot row present with the Snapshots include.
    let snapshot_name = get_blob_name(recording);
    let snapshot_client = container_client.blob_client(&snapshot_name);
    create_test_blob(&snapshot_client, None, None).await?;
    snapshot_client.create_snapshot(None).await?;

    // Blob uploaded with an encryption scope -> encryption_scope.
    let scope_name = get_blob_name(recording);
    let scope_client = container_client.blob_client(&scope_name);
    create_test_blob(
        &scope_client,
        None,
        Some(BlockBlobClientUploadOptions {
            encryption_scope: Some(get_valid_encryption_scope()),
            ..Default::default()
        }),
    )
    .await?;

    // Archived blob rehydrating to Hot -> access_tier = Archive, archive_status, rehydrate_priority.
    let archive_name = get_blob_name(recording);
    let archive_client = container_client.blob_client(&archive_name);
    create_test_blob(&archive_client, None, None).await?;
    archive_client.set_tier(AccessTier::Archive, None).await?;
    archive_client
        .set_tier(
            AccessTier::Hot,
            Some(BlobClientSetTierOptions {
                rehydrate_priority: Some(RehydratePriority::High),
                ..Default::default()
            }),
        )
        .await?;

    // Blob uploaded without an explicit tier -> access_tier_inferred.
    let inferred_name = get_blob_name(recording);
    let inferred_client = container_client.blob_client(&inferred_name);
    create_test_blob(&inferred_client, None, None).await?;

    // Soft-deleted blob -> deleted, deleted_time, remaining_retention_days with the Deleted include.
    let deleted_name = get_blob_name(recording);
    let deleted_client = container_client.blob_client(&deleted_name);
    create_test_blob(&deleted_client, None, None).await?;
    deleted_client.delete(None).await?;

    // A single list call covers every blob staged above.
    let items = list_blobs_with_include(
        &container_client,
        Some(vec![
            ListBlobsIncludeItem::Snapshots,
            ListBlobsIncludeItem::Deleted,
        ]),
    )
    .await?;
    let find = |name: &str| {
        items
            .iter()
            .find(|b| b.name.as_deref() == Some(name) && b.snapshot.is_none())
            .unwrap_or_else(|| panic!("expected blob {name} in listing"))
    };

    // Lease.
    let props = find(&leased_name)
        .properties
        .as_ref()
        .expect("leased properties");
    assert_eq!(Some(LeaseState::Leased), props.lease_state);
    assert_eq!(Some(LeaseStatus::Locked), props.lease_status);
    assert_eq!(Some(LeaseDuration::Infinite), props.lease_duration);

    // Sealed append blob.
    let props = find(&sealed_name)
        .properties
        .as_ref()
        .expect("sealed properties");
    assert_eq!(Some(true), props.is_sealed);
    assert_eq!(Some(BlobType::AppendBlob), props.blob_type);

    // Page blob sequence number.
    let props = find(&page_name)
        .properties
        .as_ref()
        .expect("page properties");
    assert_eq!(Some(7), props.blob_sequence_number);
    assert_eq!(Some(BlobType::PageBlob), props.blob_type);

    // Encryption scope.
    let props = find(&scope_name)
        .properties
        .as_ref()
        .expect("scope properties");
    assert_eq!(Some(get_valid_encryption_scope()), props.encryption_scope);

    // Archive + rehydrate.
    let props = find(&archive_name)
        .properties
        .as_ref()
        .expect("archive properties");
    assert_eq!(Some(AccessTier::Archive), props.access_tier);
    assert_eq!(
        Some(ArchiveStatus::RehydratePendingToHot),
        props.archive_status
    );
    assert_eq!(Some(RehydratePriority::High), props.rehydrate_priority);

    // Inferred tier.
    let props = find(&inferred_name)
        .properties
        .as_ref()
        .expect("inferred properties");
    assert_eq!(Some(true), props.access_tier_inferred);

    // Soft-deleted blob.
    let deleted = find(&deleted_name);
    assert_eq!(Some(true), deleted.deleted);
    let props = deleted.properties.as_ref().expect("deleted properties");
    assert!(props.deleted_time.is_some());
    assert!(props.remaining_retention_days.is_some());

    // Snapshot row (distinct from the base blob row).
    assert!(
        items
            .iter()
            .any(|b| b.name.as_deref() == Some(snapshot_name.as_str()) && b.snapshot.is_some()),
        "expected a snapshot row for {snapshot_name}"
    );

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs_version_properties_xml(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Versioned, None).await?;

    // Blob with two versions, current version retained -> version_id, is_current_version.
    let versioned_name = get_blob_name(recording);
    let versioned_client = container_client.blob_client(&versioned_name);
    create_test_blob(
        &versioned_client,
        Some(RequestContent::from(b"version 1".to_vec())),
        None,
    )
    .await?;
    create_test_blob(
        &versioned_client,
        Some(RequestContent::from(b"version 2".to_vec())),
        None,
    )
    .await?;

    let items = list_blobs_with_include(
        &container_client,
        Some(vec![ListBlobsIncludeItem::Versions]),
    )
    .await?;

    // The retained blob exposes version_id on every row and exactly one current version.
    let versions: Vec<_> = items
        .iter()
        .filter(|b| b.name.as_deref() == Some(versioned_name.as_str()))
        .collect();
    assert!(
        versions.len() >= 2,
        "expected at least two versions for {versioned_name}, got {}",
        versions.len()
    );
    assert!(
        versions.iter().all(|b| b.version_id.is_some()),
        "every version row should carry a version_id"
    );
    assert_eq!(
        1,
        versions
            .iter()
            .filter(|b| b.is_current_version == Some(true))
            .count(),
        "exactly one row should be the current version"
    );

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs_has_versions_only_xml(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Versioned, None).await?;

    // Create two versions, then delete the base blob so only versions remain. Listing with
    // DeletedWithVersions surfaces the blob as a root entry with has_versions_only set.
    let name = get_blob_name(recording);
    let blob_client = container_client.blob_client(&name);
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(b"v1".to_vec())),
        None,
    )
    .await?;
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(b"v2".to_vec())),
        None,
    )
    .await?;
    blob_client.delete(None).await?;

    let items = list_blobs_with_include(
        &container_client,
        Some(vec![ListBlobsIncludeItem::DeletedWithVersions]),
    )
    .await?;

    let blob = items
        .iter()
        .find(|b| b.name.as_deref() == Some(name.as_str()))
        .expect("expected versions-only blob in listing");
    assert_eq!(Some(true), blob.has_versions_only);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs_copy_properties_xml(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    // Async Copy Blob populates the copy_* properties on the destination blob.
    let source_name = get_blob_name(recording);
    let source_client = container_client.blob_client(&source_name);
    create_test_blob(
        &source_client,
        Some(RequestContent::from(b"arrow copy source".to_vec())),
        None,
    )
    .await?;

    let dest_name = get_blob_name(recording);
    let dest_client = container_client.blob_client(&dest_name);
    dest_client
        .start_copy_from_url(source_client.url().as_str().into(), None)
        .await?;

    // Wait for the async copy to reach a terminal state so the listing reports it.
    let mut copy_status = None;
    for _ in 0..10 {
        copy_status = dest_client.get_properties(None).await?.copy_status()?;
        if copy_status != Some(CopyStatus::Pending) {
            break;
        }
        if recording.test_mode() == TestMode::Live || recording.test_mode() == TestMode::Record {
            time::sleep(Duration::from_secs(1)).await;
        }
    }
    assert_eq!(Some(CopyStatus::Success), copy_status);

    let items =
        list_blobs_with_include(&container_client, Some(vec![ListBlobsIncludeItem::Copy])).await?;
    let blob = items
        .iter()
        .find(|b| b.name.as_deref() == Some(dest_name.as_str()))
        .expect("expected destination blob in listing");
    let props = blob.properties.as_ref().expect("expected blob properties");

    // copy_status_description, incremental_copy, and destination_snapshot are not emitted by a
    // successful non-incremental copy; they remain covered by the decoder unit tests.
    assert!(props.copy_id.is_some());
    assert_eq!(Some(CopyStatus::Success), props.copy_status);
    assert!(props.copy_source.is_some());
    assert!(props.copy_progress.is_some());
    assert!(props.copy_completion_time.is_some());

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test(playback)]
async fn test_list_blobs_immutability_properties_xml(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let blob_name = get_blob_name(recording);
    let blob_client = container_client.blob_client(&blob_name);
    create_test_blob(&blob_client, None, None).await?;

    let expiry = parse_rfc3339(
        recording
            .var(
                "IMMUTABILITY_EXPIRY",
                Some(VarOptions {
                    default_value: Some(
                        to_rfc3339(
                            &(OffsetDateTime::now_utc() + Duration::from_secs(60 * 60 * 24)),
                        )
                        .into(),
                    ),
                    ..Default::default()
                }),
            )
            .as_str(),
    )?;
    blob_client
        .set_immutability_policy(
            &expiry,
            Some(BlobClientSetImmutabilityPolicyOptions {
                immutability_policy_mode: Some(ImmutabilityPolicyMode::Unlocked),
                ..Default::default()
            }),
        )
        .await?;
    blob_client.set_legal_hold(true, None).await?;

    let items = list_blobs_with_include(
        &container_client,
        Some(vec![
            ListBlobsIncludeItem::ImmutabilityPolicy,
            ListBlobsIncludeItem::LegalHold,
        ]),
    )
    .await?;
    let blob = items
        .iter()
        .find(|b| b.name.as_deref() == Some(blob_name.as_str()))
        .expect("expected blob in listing");
    let props = blob.properties.as_ref().expect("expected blob properties");

    // Assert
    assert_eq!(
        Some(ImmutabilityPolicyMode::Unlocked),
        props.immutability_policy_mode
    );
    assert!(props.immutability_policy_expires_on.is_some());
    assert_eq!(Some(true), props.legal_hold);

    blob_client.set_legal_hold(false, None).await?;
    blob_client.delete_immutability_policy(None).await?;
    blob_client.delete(None).await?;
    let _ = container_client.delete(None).await;
    Ok(())
}

#[recorded::test(playback)]
async fn test_list_blobs_last_accessed_on_xml(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let blob_name = get_blob_name(recording);
    let blob_client = container_client.blob_client(&blob_name);
    create_test_blob(&blob_client, None, None).await?;
    let _ = blob_client.download(None).await?.body.collect().await?;

    let items = list_blobs_with_include(&container_client, None).await?;
    let blob = items
        .iter()
        .find(|b| b.name.as_deref() == Some(blob_name.as_str()))
        .expect("expected blob in listing");
    let props = blob.properties.as_ref().expect("expected blob properties");

    // Assert
    assert!(props.last_accessed_on.is_some());

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test(playback)]
async fn test_list_blobs_object_replication_metadata_xml(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let service_client = get_blob_service_client(recording, StorageAccount::Standard, None)?;

    let container_client = service_client.blob_container_client("test1");
    let blobs = list_blobs_with_include(&container_client, None).await?;
    let blob = blobs
        .iter()
        .find(|blob| {
            blob.name.as_deref() == Some("bla.txt")
                && blob.version_id.as_deref() == Some("2022-08-29T21:54:26.5412339Z")
                && blob.is_current_version == Some(true)
        })
        .expect("expected configured object-replication source blob version");

    let properties = blob
        .object_replication_metadata
        .as_ref()
        .and_then(|metadata| metadata.additional_properties.as_ref())
        .expect("expected object replication metadata on configured source blob");

    // Assert
    assert!(!properties.is_empty());
    assert_eq!(
        Some("complete"),
        properties
            .get("or-c570de93-3a83-4718-8ebe-f17b20d38a4f_49f6dc14-f5f7-4471-bf13-da984b86d136")
            .map(String::as_str)
    );
    Ok(())
}

#[recorded::test]
async fn test_list_blobs_with_continuation_xml(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;
    let blob_names = [
        "testblob1".to_string(),
        "testblob2".to_string(),
        "testblob3".to_string(),
        "testblob4".to_string(),
    ];

    container_client.create(None).await?;
    create_test_blob(
        &container_client.blob_client(&blob_names[0].clone()),
        None,
        None,
    )
    .await?;
    create_test_blob(
        &container_client.blob_client(&blob_names[1].clone()),
        None,
        None,
    )
    .await?;
    create_test_blob(
        &container_client.blob_client(&blob_names[2].clone()),
        None,
        None,
    )
    .await?;
    create_test_blob(
        &container_client.blob_client(&blob_names[3].clone()),
        None,
        None,
    )
    .await?;

    // Continuation Token with Token Provided
    let list_blobs_options = BlobContainerClientListBlobsOptions {
        maxresults: Some(2),
        ..Default::default()
    };
    let mut list_blobs_response = container_client
        .list_blobs(Some(list_blobs_options))?
        .into_pages();
    let first_page = list_blobs_response.try_next().await?;
    let list_blob_segment_response = first_page.unwrap().into_model()?;
    let continuation_token = list_blob_segment_response.next_marker;
    let blob_list = list_blob_segment_response.blob_items;
    assert_eq!(2, blob_list.len());
    for blob in blob_list {
        let blob_name = blob.name.unwrap();
        let blob_type = blob.properties.unwrap().blob_type.unwrap();
        assert!(blob_names.contains(&blob_name));
        assert_eq!(BlobType::BlockBlob, blob_type);
    }
    let list_blobs_options = BlobContainerClientListBlobsOptions {
        marker: continuation_token,
        ..Default::default()
    };
    let mut list_blobs_response = container_client
        .list_blobs(Some(list_blobs_options.clone()))?
        .into_pages();
    let second_page = list_blobs_response.try_next().await?;
    let list_blob_segment_response = second_page.unwrap().into_model()?;
    let blob_list = list_blob_segment_response.blob_items;
    assert_eq!(2, blob_list.len());
    for blob in blob_list {
        let blob_name = blob.name.unwrap();
        let blob_type = blob.properties.unwrap().blob_type.unwrap();
        assert!(blob_names.contains(&blob_name));
        assert_eq!(BlobType::BlockBlob, blob_type);
    }

    // Continuation Token, Automatic Paging
    let mut pager_response = container_client
        .list_blobs(Some(list_blobs_options))?
        .into_pages();
    let mut page_count = 0;

    while let Some(page) = pager_response.next().await {
        page_count += 1;
        let current_page = page.unwrap().into_model()?;
        match page_count {
            1 => {
                let blob_list = current_page.blob_items;
                assert_eq!(2, blob_list.len());

                for blob in blob_list {
                    let blob_name = blob.name.unwrap();
                    let blob_type = blob.properties.unwrap().blob_type.unwrap();
                    assert!(blob_names.contains(&blob_name));
                    assert_eq!(BlobType::BlockBlob, blob_type);
                }
            }
            2 => {
                let blob_list = current_page.blob_items;
                assert_eq!(2, blob_list.len());

                for blob in blob_list {
                    let blob_name = blob.name.unwrap();
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

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs_decodes_xml_invalid_names_xml(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    // Upload blobs with XML-invalid characters (U+FFFE and U+FFFF) in their names.
    // Per the Storage REST API (version 2021-02-12+), List Blobs will percent-encode
    // Name values containing these characters and set Encoded="true" on the element.
    let test_cases = [
        ("blob_with_fffe", "blob\u{FFFE}name".to_string()),
        ("blob_with_ffff", "blob\u{FFFF}name".to_string()),
        ("blob_with_both", "blob\u{FFFE}and\u{FFFF}chars".to_string()),
    ];

    for (_, blob_name) in &test_cases {
        let blob_client = container_client.blob_client(blob_name);
        create_test_blob(&blob_client, None, None).await?;
    }

    // List blobs and verify the names are correctly percent-decoded
    let mut list_blobs_response = container_client.list_blobs(None)?.into_pages();
    let page = list_blobs_response.try_next().await?;
    let list_blob_segment_response = page.unwrap().into_model()?;
    let blob_items = list_blob_segment_response.blob_items;

    // Assert
    assert_eq!(test_cases.len(), blob_items.len());

    let listed_blob_names: Vec<String> = blob_items
        .iter()
        .map(|blob| blob.name.clone().unwrap())
        .collect();

    for (label, expected_name) in &test_cases {
        assert!(
            listed_blob_names.contains(expected_name),
            "Blob '{}' with name '{}' not found in listed names: {:?}",
            label,
            expected_name,
            listed_blob_names
        );
    }

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs_with_prefix_xml(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;
    container_client.create(None).await?;

    let prefix = "aa-";
    let blob_with_prefix = format!("{}{}", prefix, get_blob_name(recording));
    let blob_no_prefix = format!("zz-{}", get_blob_name(recording));

    create_test_blob(&container_client.blob_client(&blob_with_prefix), None, None).await?;
    create_test_blob(&container_client.blob_client(&blob_no_prefix), None, None).await?;

    let names: Vec<String> = container_client
        .list_blobs(Some(BlobContainerClientListBlobsOptions {
            prefix: Some(prefix.to_string()),
            ..Default::default()
        }))?
        .try_collect::<Vec<_>>()
        .await?
        .into_iter()
        .filter_map(|b| b.name)
        .collect();
    assert_eq!(1, names.len());
    assert_eq!(blob_with_prefix, names[0]);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs_with_uncommitted_blobs_include_xml(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;
    container_client.create(None).await?;

    // Stage a block without committing - creates an uncommitted blob entry
    let blob_name = get_blob_name(recording);
    let block_blob_client = container_client.blob_client(&blob_name).block_blob_client();
    let block_id: Vec<u8> = b"block1".to_vec();
    block_blob_client
        .stage_block(&block_id, 5, RequestContent::from(b"hello".to_vec()), None)
        .await?;

    // Without UncommittedBlobs Include Scenario
    let items_without: Vec<_> = container_client.list_blobs(None)?.try_collect().await?;
    assert!(
        items_without
            .iter()
            .all(|b| b.name.as_deref() != Some(blob_name.as_str())),
        "uncommitted blob should not appear without UncommittedBlobs include"
    );

    // With UncommittedBlobs Include Scenario
    let items_with: Vec<_> = container_client
        .list_blobs(Some(BlobContainerClientListBlobsOptions {
            include: Some(vec![ListBlobsIncludeItem::UncommittedBlobs]),
            ..Default::default()
        }))?
        .try_collect()
        .await?;
    assert!(
        items_with
            .iter()
            .any(|b| b.name.as_deref() == Some(blob_name.as_str())),
        "uncommitted blob should appear with UncommittedBlobs include"
    );

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs_with_deleted_include_xml(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;
    container_client.create(None).await?;

    let blob_name = get_blob_name(recording);
    let blob_client = container_client.blob_client(&blob_name);
    create_test_blob(&blob_client, None, None).await?;

    // Soft-delete the blob
    blob_client.delete(None).await?;

    // Without Deleted Include Scenario
    let items_without: Vec<_> = container_client.list_blobs(None)?.try_collect().await?;
    assert!(
        items_without
            .iter()
            .all(|b| b.name.as_deref() != Some(blob_name.as_str())),
        "deleted blob should not appear without Deleted include"
    );

    // With Deleted Include Scenario
    let items_with: Vec<_> = container_client
        .list_blobs(Some(BlobContainerClientListBlobsOptions {
            include: Some(vec![ListBlobsIncludeItem::Deleted]),
            ..Default::default()
        }))?
        .try_collect()
        .await?;
    let deleted_blob = items_with
        .into_iter()
        .find(|b| b.name.as_deref() == Some(blob_name.as_str()))
        .expect("soft-deleted blob should appear with Deleted include");
    assert!(
        deleted_blob.deleted.unwrap_or(false),
        "blob should be marked as deleted"
    );

    container_client.delete(None).await?;
    Ok(())
}
