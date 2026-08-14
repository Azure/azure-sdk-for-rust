// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod common;

use azure_core::{
    http::{RequestContent, StatusCode},
    time::{parse_rfc3339, to_rfc3339, OffsetDateTime},
};
use azure_core_test::{recorded, Matcher, TestContext, TestMode, VarOptions};
use azure_storage_blob::format_filter_expression;
use azure_storage_blob::models::{
    AccessPolicy, AccessTier, AccountKind, ArchiveStatus, BlobClientGetPropertiesResultHeaders,
    BlobClientSetImmutabilityPolicyOptions, BlobClientSetTierOptions,
    BlobContainerClientAcquireLeaseResultHeaders, BlobContainerClientBreakLeaseOptions,
    BlobContainerClientChangeLeaseResultHeaders, BlobContainerClientCreateOptions,
    BlobContainerClientFindBlobsByTagsOptions, BlobContainerClientGetAccountInfoResultHeaders,
    BlobContainerClientGetPropertiesResultHeaders, BlobContainerClientListBlobsHierarchicalOptions,
    BlobContainerClientListBlobsOptions, BlobContainerClientSetMetadataOptions, BlobType,
    BlockBlobClientUploadOptions, CopyStatus, ImmutabilityPolicyMode, LeaseDuration, LeaseState,
    LeaseStatus, StorageResponseFormat, ListBlobsIncludeItem,
    PageBlobClientSetSequenceNumberOptions, RehydratePriority, SequenceNumberActionType,
    SignedIdentifiers, StorageErrorCode,
};
use azure_storage_blob::StorageError;
use common::{
    create_test_blob, get_blob_name, get_blob_service_client, get_container_client,
    get_container_name, get_valid_encryption_scope, list_blobs_arrow, poll_until, StorageAccount,
};
use futures::{StreamExt, TryStreamExt};
use std::{collections::HashMap, error::Error, time::Duration};
use tokio::time;

#[recorded::test]
async fn test_create_container(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;

    container_client.create(None).await?;

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_get_container_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;

    // Container Doesn't Exists Scenario
    let response = container_client.get_properties(None).await;

    // Assert
    assert!(response.is_err());
    let error = response.unwrap_err().http_status();
    assert_eq!(StatusCode::NotFound, error.unwrap());
    assert!(!container_client.exists().await?);

    // Container Exists Scenario
    container_client.create(None).await?;
    let container_properties = container_client.get_properties(None).await?;
    let lease_state = container_properties.lease_state()?;
    let has_immutability_policy = container_properties.has_immutability_policy()?;

    // Assert
    assert_eq!(LeaseState::Available, lease_state.unwrap());
    assert!(!has_immutability_policy.unwrap());
    assert!(container_client.exists().await?);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_set_container_metadata(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    // Set Metadata With Values
    let update_metadata = HashMap::from([("hello".to_string(), "world".to_string())]);
    container_client
        .set_metadata(&update_metadata, None)
        .await?;

    // Assert
    let response = container_client.get_properties(None).await?;
    let response_metadata = response.metadata()?;
    assert_eq!(update_metadata, response_metadata);

    // Set Metadata No Values (Clear Metadata)
    container_client.set_metadata(&HashMap::new(), None).await?;

    // Assert
    let response = container_client.get_properties(None).await?;
    let response_metadata = response.metadata()?;
    assert_eq!(HashMap::new(), response_metadata);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs(ctx: TestContext) -> Result<(), Box<dyn Error>> {
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
async fn test_list_blobs_arrow_populates_properties(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
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

    // Act: request the Apache Arrow stream. The SDK transparently decodes Arrow or
    // falls back to XML, so this exercises the field mapping on whichever wire
    // format the live service returns.
    let page = container_client
        .list_blobs(Some(BlobContainerClientListBlobsOptions {
            response_format: Some(StorageResponseFormat::Arrow),
            include: Some(vec![
                ListBlobsIncludeItem::Metadata,
                ListBlobsIncludeItem::Tags,
            ]),
            ..Default::default()
        }))?
        .into_pages()
        .try_next()
        .await?
        .unwrap()
        .into_model()?;

    // Assert: the scalar/timestamp/enum properties round-trip through the mapping.
    let blob = page
        .blob_items
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

    // Map-typed columns decode from the Arrow `map<utf8,utf8>` columns.
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
async fn test_list_blobs_hierarchical_arrow(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;
    container_client.create(None).await?;

    // Arrange: two blobs under a virtual directory plus one at the container root.
    for name in ["dir1/a.txt", "dir1/b.txt", "top.txt"] {
        create_test_blob(&container_client.blob_client(name), None, None).await?;
    }

    // Act: list hierarchically over Apache Arrow, grouping the directory with "/".
    let page = container_client
        .list_blobs_hierarchical(
            "/",
            Some(BlobContainerClientListBlobsHierarchicalOptions {
                response_format: Some(StorageResponseFormat::Arrow),
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
async fn test_list_blobs_arrow_end_before(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;
    container_client.create(None).await?;

    // Arrange: four lexicographically ordered blobs.
    for name in ["aa.txt", "bb.txt", "cc.txt", "dd.txt"] {
        create_test_blob(&container_client.blob_client(name), None, None).await?;
    }

    // Act: Apache Arrow range listing stops before "cc.txt" (exclusive).
    let page = container_client
        .list_blobs(Some(BlobContainerClientListBlobsOptions {
            response_format: Some(StorageResponseFormat::Arrow),
            end_before: Some("cc.txt".to_string()),
            ..Default::default()
        }))?
        .into_pages()
        .try_next()
        .await?
        .expect("expected a page")
        .into_model()?;

    // Assert: only names ordered before "cc.txt" are returned.
    let names: Vec<_> = page
        .blob_items
        .iter()
        .filter_map(|b| b.name.as_deref())
        .collect();
    assert!(names.contains(&"aa.txt"));
    assert!(names.contains(&"bb.txt"));
    assert!(!names.contains(&"cc.txt"));
    assert!(!names.contains(&"dd.txt"));

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs_arrow_stateful_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
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

    // A single Arrow list call covers every blob staged above.
    let items = list_blobs_arrow(
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
async fn test_list_blobs_arrow_version_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
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

    let items = list_blobs_arrow(
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
async fn test_list_blobs_arrow_has_versions_only(ctx: TestContext) -> Result<(), Box<dyn Error>> {
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

    let items = list_blobs_arrow(
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
async fn test_list_blobs_arrow_copy_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
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

    let items = list_blobs_arrow(&container_client, Some(vec![ListBlobsIncludeItem::Copy])).await?;
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
async fn test_list_blobs_arrow_immutability_properties(
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

    let items = list_blobs_arrow(
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
async fn test_list_blobs_arrow_last_accessed_on(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let blob_name = get_blob_name(recording);
    let blob_client = container_client.blob_client(&blob_name);
    create_test_blob(&blob_client, None, None).await?;
    let _ = blob_client.download(None).await?.body.collect().await?;

    let items = list_blobs_arrow(&container_client, None).await?;
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
async fn test_list_blobs_arrow_object_replication_metadata(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let service_client = get_blob_service_client(recording, StorageAccount::Standard, None)?;

    let container_client = service_client.blob_container_client("test1");
    let blobs = list_blobs_arrow(&container_client, None).await?;
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
async fn test_list_blobs_with_continuation(ctx: TestContext) -> Result<(), Box<dyn Error>> {
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
async fn test_list_blobs_decodes_xml_invalid_names(ctx: TestContext) -> Result<(), Box<dyn Error>> {
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
async fn test_container_lease_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let blob_service_client = get_blob_service_client(recording, StorageAccount::Standard, None)?;
    let container_name = get_container_name(recording);
    let container_client = blob_service_client.blob_container_client(&container_name.clone());
    let other_container_client = blob_service_client.blob_container_client(&container_name);
    container_client.create(None).await?;

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
        .set_metadata(&update_metadata, Some(set_metadata_options))
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
    time::sleep(Duration::from_secs(15)).await;

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

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_get_account_info(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

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
async fn test_find_blobs_by_tags(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    ctx.recording()
        .set_matcher(Matcher::HeaderlessMatcher)
        .await?;

    let container_client =
        get_container_client(ctx.recording(), true, StorageAccount::Standard, None).await?;

    // Create Test Blobs with Distinct Tags
    let blob1_name = get_blob_name(ctx.recording());
    let blob1_tags = HashMap::from([("fizz".to_string(), "buzz".to_string())]);
    create_test_blob(
        &container_client.blob_client(&blob1_name.clone()),
        Some(RequestContent::from("ferris the crab".as_bytes().into())),
        Some(BlockBlobClientUploadOptions::default().with_tags(blob1_tags.clone())),
    )
    .await?;

    // Create 3 blobs sharing the same tag to exercise the max-results option
    let shared_tag = HashMap::from([("env".to_string(), "test".to_string())]);
    for _ in 0..3 {
        let name = get_blob_name(ctx.recording());
        create_test_blob(
            &container_client.blob_client(&name),
            Some(RequestContent::from("data".as_bytes().into())),
            Some(BlockBlobClientUploadOptions::default().with_tags(shared_tag.clone())),
        )
        .await?;
    }

    let blob2_name = get_blob_name(ctx.recording());
    create_test_blob(
        &container_client.blob_client(&blob2_name.clone()),
        Some(RequestContent::from("hello world".as_bytes().into())),
        Some(
            BlockBlobClientUploadOptions::default().with_tags(HashMap::from([
                ("foo".to_string(), "bar".to_string()),
                ("alice".to_string(), "bob".to_string()),
            ])),
        ),
    )
    .await?;

    // Find "hello world" blob by its tag {"foo": "bar"}.
    // In live mode, poll until tags are indexed (up to 60s total timeout).
    // In record mode, use a fixed 15s sleep.
    poll_until(ctx.recording(), || async {
        let mut pager = container_client
            .find_blobs_by_tags("\"foo\"='bar'", None)?
            .into_pages();
        if let Some(resp) = pager.try_next().await? {
            let segment = resp.into_model()?;
            if segment
                .blob_items
                .iter()
                .any(|b| b.name.as_deref() == Some(blob2_name.as_str()))
            {
                return Ok(true);
            }
        }
        Ok(false)
    })
    .await?;
    // Final assertion (covers record/playback where poll_until doesn't check the condition)
    {
        let mut pager = container_client
            .find_blobs_by_tags("\"foo\"='bar'", None)?
            .into_pages();
        let filter_blob_segment = pager
            .try_next()
            .await?
            .expect("expected at least one page from find_blobs_by_tags")
            .into_model()?;
        let blobs = &filter_blob_segment.blob_items;
        assert!(
            blobs
                .iter()
                .any(|blob| blob.name.as_deref() == Some(blob2_name.as_str())),
            "Failed to find \"{blob2_name}\" in filtered blob results."
        );
    }

    // Find "ferris the crab" blob by its tag {"fizz": "buzz"}
    let mut pager = container_client
        .find_blobs_by_tags(&format_filter_expression(&blob1_tags)?, None)?
        .into_pages();
    let filter_blob_segment = pager
        .try_next()
        .await?
        .expect("expected at least one page from find_blobs_by_tags")
        .into_model()?;
    let blobs = &filter_blob_segment.blob_items;
    assert!(
        blobs
            .iter()
            .any(|blob| blob.name.as_deref() == Some(blob1_name.as_str())),
        "Failed to find \"{blob1_name}\" in filtered blob results."
    );

    // Max Results Scenario
    let options = BlobContainerClientFindBlobsByTagsOptions {
        maxresults: Some(2),
        ..Default::default()
    };
    let mut pager = container_client
        .find_blobs_by_tags("\"env\"='test'", Some(options))?
        .into_pages();
    let page = pager
        .try_next()
        .await?
        .expect("expected at least one page from find_blobs_by_tags")
        .into_model()?;
    let blobs = &page.blob_items;
    assert!(
        blobs.len() <= 2,
        "page should contain at most 2 blobs due to maxresults=2, got {}",
        blobs.len()
    );

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_container_access_policy(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();

    ctx.recording()
        .set_matcher(Matcher::BodilessMatcher)
        .await?;

    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;
    container_client.create(None).await?;

    // Set Access Policy w/ Multiple Policy Defined
    let expiry = recording.var(
        "expiry",
        Some(VarOptions {
            default_value: Some(
                to_rfc3339(&(OffsetDateTime::now_utc() + Duration::from_secs(10))).into(),
            ),
            ..Default::default()
        }),
    );
    let start = recording.var(
        "start",
        Some(VarOptions {
            default_value: Some(to_rfc3339(&OffsetDateTime::now_utc()).into()),
            ..Default::default()
        }),
    );
    let test_id_1: Option<String> = Some("testid_1".into());
    let test_id_2: Option<String> = Some("testid_2".into());
    let access_policy_1 = AccessPolicy {
        expiry: Some(parse_rfc3339(&expiry)?),
        permission: Some("rw".to_string()),
        start: Some(parse_rfc3339(&start)?),
    };
    let access_policy_2 = AccessPolicy {
        expiry: Some(parse_rfc3339(&expiry)?),
        permission: Some("cd".to_string()),
        start: Some(parse_rfc3339(&start)?),
    };
    let policies: HashMap<String, AccessPolicy> = HashMap::from([
        (test_id_1.clone().unwrap(), access_policy_1.clone()),
        (test_id_2.clone().unwrap(), access_policy_2.clone()),
    ]);
    container_client
        .set_access_policy(
            RequestContent::try_from(SignedIdentifiers::from(policies))?,
            None,
        )
        .await?;

    Ok(())
}

#[recorded::test]
async fn test_create_container_with_metadata(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;

    let metadata = HashMap::from([
        ("author".to_string(), "ferris".to_string()),
        ("project".to_string(), "azure-sdk-for-rust".to_string()),
    ]);
    container_client
        .create(Some(BlobContainerClientCreateOptions {
            metadata: Some(metadata.clone()),
            ..Default::default()
        }))
        .await?;

    let props = container_client.get_properties(None).await?;
    assert_eq!(metadata, props.metadata()?);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs_with_include_options(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;
    container_client.create(None).await?;

    // Create a blob with metadata and one with tags
    let metadata_blob_name = get_blob_name(recording);
    let tags_blob_name = get_blob_name(recording);
    let metadata = HashMap::from([("team".to_string(), "sdk".to_string())]);
    create_test_blob(
        &container_client.blob_client(&metadata_blob_name),
        None,
        Some(BlockBlobClientUploadOptions {
            metadata: Some(metadata.clone()),
            ..Default::default()
        }),
    )
    .await?;
    create_test_blob(
        &container_client.blob_client(&tags_blob_name),
        None,
        Some(
            BlockBlobClientUploadOptions::default()
                .with_tags(HashMap::from([("env".to_string(), "test".to_string())])),
        ),
    )
    .await?;

    // List with both Metadata and Tags includes
    let page = container_client
        .list_blobs(Some(BlobContainerClientListBlobsOptions {
            include: Some(vec![
                ListBlobsIncludeItem::Metadata,
                ListBlobsIncludeItem::Tags,
            ]),
            ..Default::default()
        }))?
        .into_pages()
        .try_next()
        .await?
        .unwrap()
        .into_model()?;

    let items = page.blob_items;

    // Metadata blob: metadata should be populated
    let meta_blob = items
        .iter()
        .find(|b| b.name.as_deref() == Some(metadata_blob_name.as_str()))
        .expect("expected metadata blob in listing");
    let blob_meta = meta_blob
        .metadata
        .as_ref()
        .expect("metadata should be populated");
    assert_eq!(Some(&metadata), blob_meta.values.as_ref());

    // Tags blob: blob_tags should be populated
    let tags_blob = items
        .iter()
        .find(|b| b.name.as_deref() == Some(tags_blob_name.as_str()))
        .expect("expected tags blob in listing");
    assert!(
        tags_blob.blob_tags.is_some(),
        "expected blob_tags to be populated with Tags include"
    );

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs_with_prefix(ctx: TestContext) -> Result<(), Box<dyn Error>> {
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

    let page = container_client
        .list_blobs(Some(BlobContainerClientListBlobsOptions {
            prefix: Some(prefix.to_string()),
            ..Default::default()
        }))?
        .into_pages()
        .try_next()
        .await?
        .unwrap()
        .into_model()?;

    let names: Vec<String> = page.blob_items.into_iter().filter_map(|b| b.name).collect();
    assert_eq!(1, names.len());
    assert_eq!(blob_with_prefix, names[0]);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs_with_uncommitted_blobs_include(
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
    let page_without = container_client
        .list_blobs(None)?
        .into_pages()
        .try_next()
        .await?
        .unwrap()
        .into_model()?;
    assert!(
        page_without
            .blob_items
            .iter()
            .all(|b| b.name.as_deref() != Some(blob_name.as_str())),
        "uncommitted blob should not appear without UncommittedBlobs include"
    );

    // With UncommittedBlobs Include Scenario
    let page_with = container_client
        .list_blobs(Some(BlobContainerClientListBlobsOptions {
            include: Some(vec![ListBlobsIncludeItem::UncommittedBlobs]),
            ..Default::default()
        }))?
        .into_pages()
        .try_next()
        .await?
        .unwrap()
        .into_model()?;
    assert!(
        page_with
            .blob_items
            .iter()
            .any(|b| b.name.as_deref() == Some(blob_name.as_str())),
        "uncommitted blob should appear with UncommittedBlobs include"
    );

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs_with_deleted_include(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // TODO: requires an account with blob soft-delete enabled (set via Set Blob Service Properties,
    // deleteRetentionPolicy.enabled = true). Record this test against such an account.

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
    let page_without = container_client
        .list_blobs(None)?
        .into_pages()
        .try_next()
        .await?
        .unwrap()
        .into_model()?;
    assert!(
        page_without
            .blob_items
            .iter()
            .all(|b| b.name.as_deref() != Some(blob_name.as_str())),
        "deleted blob should not appear without Deleted include"
    );

    // With Deleted Include Scenario
    let page_with = container_client
        .list_blobs(Some(BlobContainerClientListBlobsOptions {
            include: Some(vec![ListBlobsIncludeItem::Deleted]),
            ..Default::default()
        }))?
        .into_pages()
        .try_next()
        .await?
        .unwrap()
        .into_model()?;
    let deleted_blob = page_with
        .blob_items
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

#[recorded::test]
async fn test_list_blobs_with_copy_include(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;
    container_client.create(None).await?;

    // Create source blob and copy it to a destination
    let source_name = get_blob_name(recording);
    let dest_name = get_blob_name(recording);
    let source_blob_client = container_client.blob_client(&source_name);
    create_test_blob(&source_blob_client, None, None).await?;

    let dest_blob_client = container_client.blob_client(&dest_name);
    dest_blob_client
        .block_blob_client()
        .upload_blob_from_url(source_blob_client.url().as_str().into(), None)
        .await?;

    // Copy Include Scenario
    let page = container_client
        .list_blobs(Some(BlobContainerClientListBlobsOptions {
            include: Some(vec![ListBlobsIncludeItem::Copy]),
            ..Default::default()
        }))?
        .into_pages()
        .try_next()
        .await?
        .unwrap()
        .into_model()?;

    // Assert
    // Note: copy_status/copy_id/copy_source are only populated for async Copy Blob
    // operations, not synchronous Put Blob From URL. The Copy include flag is
    // accepted and the destination blob still appears in the listing.
    let dest_blob = page
        .blob_items
        .into_iter()
        .find(|b| b.name.as_deref() == Some(dest_name.as_str()))
        .expect("destination blob should appear in listing");
    assert!(
        dest_blob.properties.is_some(),
        "dest blob should have properties"
    );

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_break_lease_with_break_period(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    // Acquire Lease
    container_client.acquire_lease(60, None).await?;

    // Break Lease
    let options = BlobContainerClientBreakLeaseOptions {
        break_period: Some(0),
        ..Default::default()
    };
    container_client.break_lease(Some(options)).await?;

    // Assert
    let acquire_response = container_client.acquire_lease(15, None).await?;
    let new_lease_id = acquire_response.lease_id()?.unwrap();
    container_client.release_lease(new_lease_id, None).await?;

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_container_error_codes(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    // Do NOT create the container yet
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;

    // ContainerNotFound - get_properties before the container exists
    let err = container_client.get_properties(None).await.unwrap_err();
    let storage_error: StorageError = err.try_into()?;
    assert_eq!(
        storage_error.error_code.as_ref(),
        Some(&StorageErrorCode::ContainerNotFound),
        "expected ContainerNotFound error code"
    );

    // Create the container so it now exists
    container_client.create(None).await?;

    // ContainerAlreadyExists - create it a second time
    let err = container_client.create(None).await.unwrap_err();
    let storage_error: StorageError = err.try_into()?;
    assert_eq!(
        storage_error.error_code.as_ref(),
        Some(&StorageErrorCode::ContainerAlreadyExists),
        "expected ContainerAlreadyExists error code"
    );

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_lease_already_present_error_code(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    // Acquire the lease on the container
    let acquire = container_client.acquire_lease(-1, None).await?;
    let lease_id = acquire.lease_id()?.expect("lease_id must be present");

    // Attempt a second acquire - should fail with LeaseAlreadyPresent
    let err = container_client.acquire_lease(-1, None).await.unwrap_err();
    let storage_error: StorageError = err.try_into()?;

    // Assert
    assert_eq!(
        storage_error.error_code.as_ref(),
        Some(&StorageErrorCode::LeaseAlreadyPresent),
        "expected LeaseAlreadyPresent error code"
    );

    // Clean up
    container_client.release_lease(lease_id, None).await?;
    container_client.delete(None).await?;
    Ok(())
}
