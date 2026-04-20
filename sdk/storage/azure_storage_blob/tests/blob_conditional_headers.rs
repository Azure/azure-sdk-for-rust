// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::{RequestContent, StatusCode};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::models::{
    AccessTier, AppendBlobClientAppendBlockOptions, AppendBlobClientCreateOptions,
    AppendBlobClientSealOptions, BlobClientAcquireLeaseOptions,
    BlobClientAcquireLeaseResultHeaders, BlobClientBreakLeaseOptions, BlobClientChangeLeaseOptions,
    BlobClientChangeLeaseResultHeaders, BlobClientCreateSnapshotOptions, BlobClientDeleteOptions,
    BlobClientDownloadOptions, BlobClientGetPropertiesOptions,
    BlobClientGetPropertiesResultHeaders, BlobClientGetTagsOptions, BlobClientReleaseLeaseOptions,
    BlobClientRenewLeaseOptions, BlobClientSetMetadataOptions, BlobClientSetPropertiesOptions,
    BlobClientSetTagsOptions, BlobClientSetTierOptions, BlobContainerClientAcquireLeaseOptions,
    BlobContainerClientAcquireLeaseResultHeaders, BlobContainerClientBreakLeaseOptions,
    BlobContainerClientChangeLeaseOptions, BlobContainerClientDeleteOptions,
    BlobContainerClientGetPropertiesResultHeaders, BlobContainerClientReleaseLeaseOptions,
    BlobContainerClientRenewLeaseOptions, BlobContainerClientSetAccessPolicyOptions,
    BlobContainerClientSetMetadataOptions, BlobTags, BlockBlobClientCommitBlockListOptions,
    BlockBlobClientGetBlockListOptions, BlockBlobClientUploadOptions, BlockListType,
    BlockLookupList, DeleteSnapshotsOptionType, HttpRange, PageBlobClientClearPagesOptions,
    PageBlobClientCreateOptions, PageBlobClientGetPageRangesOptions, PageBlobClientResizeOptions,
    PageBlobClientSetSequenceNumberOptions, PageBlobClientUploadPagesOptions,
    SequenceNumberActionType, SignedIdentifiers,
};
use azure_storage_blob_test::{
    create_test_blob, get_blob_name, get_container_client, StorageAccount,
};
use std::{collections::HashMap, error::Error, time::Duration};

const BAD_ETAG: &str = "\"bad-etag-value\"";

mod blob_client {
    use super::*;

    #[recorded::test]
    #[ignore = "need to investigate live test pipeline failures"]
    async fn test_blob_client_etag_conditions(ctx: TestContext) -> Result<(), Box<dyn Error>> {
        // Recording Setup
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, true, StorageAccount::Standard, None).await?;
        let blob_client = container_client.blob_client(&get_blob_name(recording));

        create_test_blob(&blob_client, None, None).await?;
        let props = blob_client.get_properties(None).await?;
        let etag = props.etag()?.unwrap().to_string();

        // Read Operations - if_match Success + if_none_match 304

        // Download
        blob_client
            .download(Some(BlobClientDownloadOptions {
                if_match: Some(etag.clone()),
                ..Default::default()
            }))
            .await?;
        let err = blob_client
            .download(Some(BlobClientDownloadOptions {
                if_none_match: Some(etag.clone()),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::NotModified,
            err.unwrap_err().http_status().unwrap()
        );

        // Download if_match Failure
        let err = blob_client
            .download(Some(BlobClientDownloadOptions {
                if_match: Some(BAD_ETAG.to_string()),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );

        // Get Properties
        blob_client
            .get_properties(Some(BlobClientGetPropertiesOptions {
                if_match: Some(etag.clone().into()),
                ..Default::default()
            }))
            .await?;
        let err = blob_client
            .get_properties(Some(BlobClientGetPropertiesOptions {
                if_none_match: Some(etag.clone().into()),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::NotModified,
            err.unwrap_err().http_status().unwrap()
        );

        // Get Tags
        blob_client
            .get_tags(Some(BlobClientGetTagsOptions {
                if_match: Some(etag.clone().into()),
                ..Default::default()
            }))
            .await?;
        let err = blob_client
            .get_tags(Some(BlobClientGetTagsOptions {
                if_none_match: Some(etag.clone().into()),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::NotModified,
            err.unwrap_err().http_status().unwrap()
        );

        // Create Snapshot
        blob_client
            .create_snapshot(Some(BlobClientCreateSnapshotOptions {
                if_match: Some(etag.clone().into()),
                ..Default::default()
            }))
            .await?;
        let err = blob_client
            .create_snapshot(Some(BlobClientCreateSnapshotOptions {
                if_none_match: Some(etag.clone().into()),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );

        // Write Operations (Mutating)

        // Set Metadata - Failure First, Then Success
        let metadata = HashMap::from([("key".to_string(), "val".to_string())]);
        let err = blob_client
            .set_metadata(
                &metadata,
                Some(BlobClientSetMetadataOptions {
                    if_match: Some(BAD_ETAG.to_string().into()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        blob_client
            .set_metadata(
                &metadata,
                Some(BlobClientSetMetadataOptions {
                    if_match: Some(etag.clone().into()),
                    ..Default::default()
                }),
            )
            .await?;
        // Set Metadata Changes the ETag - Refresh
        let props = blob_client.get_properties(None).await?;
        let etag = props.etag()?.unwrap().to_string();

        // if_none_match Failure on Set Metadata
        let err = blob_client
            .set_metadata(
                &metadata,
                Some(BlobClientSetMetadataOptions {
                    if_none_match: Some(etag.clone().into()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );

        // Set Properties - Failure First, Then Success
        let err = blob_client
            .set_properties(Some(BlobClientSetPropertiesOptions {
                if_match: Some(BAD_ETAG.to_string().into()),
                blob_content_type: Some("application/octet-stream".to_string()),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        blob_client
            .set_properties(Some(BlobClientSetPropertiesOptions {
                if_match: Some(etag.clone().into()),
                blob_content_type: Some("application/octet-stream".to_string()),
                ..Default::default()
            }))
            .await?;
        // Set Properties Changes the ETag - Refresh
        let props = blob_client.get_properties(None).await?;
        let etag = props.etag()?.unwrap().to_string();

        // Set Tags - Does Not Change the ETag, so etag remains valid for repeated use
        let err = blob_client
            .set_tags(
                RequestContent::try_from(BlobTags::from(HashMap::from([(
                    "env".to_string(),
                    "test".to_string(),
                )])))?,
                Some(BlobClientSetTagsOptions {
                    if_match: Some(BAD_ETAG.to_string().into()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        blob_client
            .set_tags(
                RequestContent::try_from(BlobTags::from(HashMap::from([(
                    "env".to_string(),
                    "test".to_string(),
                )])))?,
                Some(BlobClientSetTagsOptions {
                    if_match: Some(etag.clone().into()),
                    ..Default::default()
                }),
            )
            .await?;

        // Lease Operations

        // Acquire Lease - Failure
        let err = blob_client
            .acquire_lease(
                -1,
                Some(BlobClientAcquireLeaseOptions {
                    if_match: Some(BAD_ETAG.to_string().into()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );

        // Acquire Lease - Success
        let lease_resp = blob_client
            .acquire_lease(
                -1,
                Some(BlobClientAcquireLeaseOptions {
                    if_match: Some(etag.clone().into()),
                    ..Default::default()
                }),
            )
            .await?;
        let lease_id_1 = lease_resp.lease_id()?.unwrap().to_string();

        // Renew Lease - Failure
        let err = blob_client
            .renew_lease(
                lease_id_1.clone(),
                Some(BlobClientRenewLeaseOptions {
                    if_match: Some(BAD_ETAG.to_string().into()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );

        // Renew Lease - Success
        blob_client
            .renew_lease(
                lease_id_1.clone(),
                Some(BlobClientRenewLeaseOptions {
                    if_match: Some(etag.clone().into()),
                    ..Default::default()
                }),
            )
            .await?;

        // Change Lease - Failure
        let proposed_id = "aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa".to_string();
        let err = blob_client
            .change_lease(
                lease_id_1.clone(),
                proposed_id.clone(),
                Some(BlobClientChangeLeaseOptions {
                    if_match: Some(BAD_ETAG.to_string().into()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );

        // Change Lease - Success
        let change_resp = blob_client
            .change_lease(
                lease_id_1,
                proposed_id.clone(),
                Some(BlobClientChangeLeaseOptions {
                    if_match: Some(etag.clone().into()),
                    ..Default::default()
                }),
            )
            .await?;
        let lease_id_2 = change_resp.lease_id()?.unwrap().to_string();

        // Release Lease - Failure
        let err = blob_client
            .release_lease(
                lease_id_2.clone(),
                Some(BlobClientReleaseLeaseOptions {
                    if_match: Some(BAD_ETAG.to_string().into()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );

        // Release Lease - Success
        blob_client
            .release_lease(
                lease_id_2,
                Some(BlobClientReleaseLeaseOptions {
                    if_match: Some(etag.clone().into()),
                    ..Default::default()
                }),
            )
            .await?;

        // Break Lease - Acquire Fresh Lease, Then Test Break With Conditions
        let fresh_lease_resp = blob_client.acquire_lease(-1, None).await?;
        let _fresh_lease_id = fresh_lease_resp.lease_id()?.unwrap().to_string();

        let err = blob_client
            .break_lease(Some(BlobClientBreakLeaseOptions {
                if_match: Some(BAD_ETAG.to_string().into()),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        blob_client
            .break_lease(Some(BlobClientBreakLeaseOptions {
                if_match: Some(etag.clone().into()),
                ..Default::default()
            }))
            .await?;

        // Delete - Last (Destructive)
        let err = blob_client
            .delete(Some(BlobClientDeleteOptions {
                if_match: Some(BAD_ETAG.to_string().into()),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        blob_client
            .delete(Some(BlobClientDeleteOptions {
                if_match: Some(etag.clone().into()),
                delete_snapshots: Some(DeleteSnapshotsOptionType::Include),
                ..Default::default()
            }))
            .await?;

        container_client.delete(None).await?;
        Ok(())
    }

    #[recorded::test]
    #[ignore = "need to investigate live test pipeline failures"]
    async fn test_blob_client_time_conditions(ctx: TestContext) -> Result<(), Box<dyn Error>> {
        // Recording Setup
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, true, StorageAccount::Standard, None).await?;
        let blob_client = container_client.blob_client(&get_blob_name(recording));

        create_test_blob(&blob_client, None, None).await?;
        let props = blob_client.get_properties(None).await?;
        let last_modified = props.last_modified()?.unwrap();
        let before = last_modified - Duration::from_secs(60);
        let after = last_modified + Duration::from_secs(60);

        // Download
        // if_modified_since=before - Success
        blob_client
            .download(Some(BlobClientDownloadOptions {
                if_modified_since: Some(before),
                ..Default::default()
            }))
            .await?;
        // if_modified_since=after - Not Modified (304)
        let err = blob_client
            .download(Some(BlobClientDownloadOptions {
                if_modified_since: Some(after),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::NotModified,
            err.unwrap_err().http_status().unwrap()
        );
        // if_unmodified_since=after - Success
        blob_client
            .download(Some(BlobClientDownloadOptions {
                if_unmodified_since: Some(after),
                ..Default::default()
            }))
            .await?;
        // if_unmodified_since=before - Failure
        let err = blob_client
            .download(Some(BlobClientDownloadOptions {
                if_unmodified_since: Some(before),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );

        // Get Properties
        blob_client
            .get_properties(Some(BlobClientGetPropertiesOptions {
                if_modified_since: Some(before),
                ..Default::default()
            }))
            .await?;
        let err = blob_client
            .get_properties(Some(BlobClientGetPropertiesOptions {
                if_modified_since: Some(after),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::NotModified,
            err.unwrap_err().http_status().unwrap()
        );
        blob_client
            .get_properties(Some(BlobClientGetPropertiesOptions {
                if_unmodified_since: Some(after),
                ..Default::default()
            }))
            .await?;
        let err = blob_client
            .get_properties(Some(BlobClientGetPropertiesOptions {
                if_unmodified_since: Some(before),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );

        // Get Tags
        blob_client
            .get_tags(Some(BlobClientGetTagsOptions {
                if_modified_since: Some(before),
                ..Default::default()
            }))
            .await?;
        let err = blob_client
            .get_tags(Some(BlobClientGetTagsOptions {
                if_unmodified_since: Some(before),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );

        // Create Snapshot
        blob_client
            .create_snapshot(Some(BlobClientCreateSnapshotOptions {
                if_unmodified_since: Some(after),
                ..Default::default()
            }))
            .await?;
        let err = blob_client
            .create_snapshot(Some(BlobClientCreateSnapshotOptions {
                if_modified_since: Some(after),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );

        // Set Metadata - Mutating, Test Failure + One Success Then Re-Capture Timestamp
        let metadata = HashMap::from([("key".to_string(), "val".to_string())]);
        let err = blob_client
            .set_metadata(
                &metadata,
                Some(BlobClientSetMetadataOptions {
                    if_unmodified_since: Some(before),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        blob_client
            .set_metadata(
                &metadata,
                Some(BlobClientSetMetadataOptions {
                    if_modified_since: Some(before),
                    ..Default::default()
                }),
            )
            .await?;
        // Re-Capture last_modified After Successful Set Metadata
        let props = blob_client.get_properties(None).await?;
        let last_modified = props.last_modified()?.unwrap();
        let after = last_modified + Duration::from_secs(60);

        // Set Properties
        let err = blob_client
            .set_properties(Some(BlobClientSetPropertiesOptions {
                if_modified_since: Some(after),
                blob_content_type: Some("application/octet-stream".to_string()),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        blob_client
            .set_properties(Some(BlobClientSetPropertiesOptions {
                if_unmodified_since: Some(after),
                blob_content_type: Some("application/octet-stream".to_string()),
                ..Default::default()
            }))
            .await?;
        // Re-Capture After Set Properties
        let props = blob_client.get_properties(None).await?;
        let last_modified = props.last_modified()?.unwrap();
        let before = last_modified - Duration::from_secs(60);
        let after = last_modified + Duration::from_secs(60);

        // Set Tags - Tags Do Not Change last_modified
        blob_client
            .set_tags(
                RequestContent::try_from(BlobTags::from(HashMap::new()))?,
                Some(BlobClientSetTagsOptions {
                    if_unmodified_since: Some(after),
                    ..Default::default()
                }),
            )
            .await?;
        let err = blob_client
            .set_tags(
                RequestContent::try_from(BlobTags::from(HashMap::new()))?,
                Some(BlobClientSetTagsOptions {
                    if_unmodified_since: Some(before),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );

        // Lease Operations - Lease Operations Do Not Change Blob's last_modified

        // Acquire Lease - Failure
        let err = blob_client
            .acquire_lease(
                -1,
                Some(BlobClientAcquireLeaseOptions {
                    if_modified_since: Some(after),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // Acquire Lease - Success
        let lease_resp = blob_client
            .acquire_lease(
                -1,
                Some(BlobClientAcquireLeaseOptions {
                    if_unmodified_since: Some(after),
                    ..Default::default()
                }),
            )
            .await?;
        let lease_id_1 = lease_resp.lease_id()?.unwrap().to_string();

        // Renew Lease - Failure
        let err = blob_client
            .renew_lease(
                lease_id_1.clone(),
                Some(BlobClientRenewLeaseOptions {
                    if_unmodified_since: Some(before),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // Renew Lease - Success
        blob_client
            .renew_lease(
                lease_id_1.clone(),
                Some(BlobClientRenewLeaseOptions {
                    if_unmodified_since: Some(after),
                    ..Default::default()
                }),
            )
            .await?;

        // Change Lease - Failure
        let proposed_id = "bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb".to_string();
        let err = blob_client
            .change_lease(
                lease_id_1.clone(),
                proposed_id.clone(),
                Some(BlobClientChangeLeaseOptions {
                    if_modified_since: Some(after),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // Change Lease - Success
        blob_client
            .change_lease(
                lease_id_1,
                proposed_id.clone(),
                Some(BlobClientChangeLeaseOptions {
                    if_modified_since: Some(before),
                    ..Default::default()
                }),
            )
            .await?;
        let lease_id_2 = proposed_id;

        // Release Lease - Failure
        let err = blob_client
            .release_lease(
                lease_id_2.clone(),
                Some(BlobClientReleaseLeaseOptions {
                    if_unmodified_since: Some(before),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // Release Lease - Success
        blob_client
            .release_lease(
                lease_id_2,
                Some(BlobClientReleaseLeaseOptions {
                    if_unmodified_since: Some(after),
                    ..Default::default()
                }),
            )
            .await?;

        // Break Lease
        blob_client.acquire_lease(-1, None).await?;
        let err = blob_client
            .break_lease(Some(BlobClientBreakLeaseOptions {
                if_modified_since: Some(after),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        blob_client
            .break_lease(Some(BlobClientBreakLeaseOptions {
                if_modified_since: Some(before),
                ..Default::default()
            }))
            .await?;

        // Delete
        let err = blob_client
            .delete(Some(BlobClientDeleteOptions {
                if_unmodified_since: Some(before),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        blob_client
            .delete(Some(BlobClientDeleteOptions {
                if_modified_since: Some(before),
                delete_snapshots: Some(DeleteSnapshotsOptionType::Include),
                ..Default::default()
            }))
            .await?;

        container_client.delete(None).await?;
        Ok(())
    }

    #[recorded::test]
    #[ignore = "need to investigate live test pipeline failures"]
    async fn test_blob_client_if_tags_condition(ctx: TestContext) -> Result<(), Box<dyn Error>> {
        // Recording Setup
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, true, StorageAccount::Standard, None).await?;
        let blob_client = container_client.blob_client(&get_blob_name(recording));

        // Upload With a Tag That the if_tags Conditions Can Reference
        create_test_blob(
            &blob_client,
            None,
            Some(BlockBlobClientUploadOptions {
                blob_tags_string: Some("env=test".to_string()),
                ..Default::default()
            }),
        )
        .await?;

        let matching_expr = "\"env\"='test'";
        let non_matching_expr = "\"env\"='wrong'";

        // Download
        blob_client
            .download(Some(BlobClientDownloadOptions {
                if_tags: Some(matching_expr.to_string()),
                ..Default::default()
            }))
            .await?;
        let err = blob_client
            .download(Some(BlobClientDownloadOptions {
                if_tags: Some(non_matching_expr.to_string()),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );

        // Get Properties
        blob_client
            .get_properties(Some(BlobClientGetPropertiesOptions {
                if_tags: Some(matching_expr.to_string()),
                ..Default::default()
            }))
            .await?;
        let err = blob_client
            .get_properties(Some(BlobClientGetPropertiesOptions {
                if_tags: Some(non_matching_expr.to_string()),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );

        // Get Tags
        blob_client
            .get_tags(Some(BlobClientGetTagsOptions {
                if_tags: Some(matching_expr.to_string()),
                ..Default::default()
            }))
            .await?;
        let err = blob_client
            .get_tags(Some(BlobClientGetTagsOptions {
                if_tags: Some(non_matching_expr.to_string()),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );

        // Create Snapshot
        blob_client
            .create_snapshot(Some(BlobClientCreateSnapshotOptions {
                if_tags: Some(matching_expr.to_string()),
                ..Default::default()
            }))
            .await?;
        let err = blob_client
            .create_snapshot(Some(BlobClientCreateSnapshotOptions {
                if_tags: Some(non_matching_expr.to_string()),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );

        // Set Metadata
        let metadata = HashMap::from([("key".to_string(), "val".to_string())]);
        let err = blob_client
            .set_metadata(
                &metadata,
                Some(BlobClientSetMetadataOptions {
                    if_tags: Some(non_matching_expr.to_string()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        blob_client
            .set_metadata(
                &metadata,
                Some(BlobClientSetMetadataOptions {
                    if_tags: Some(matching_expr.to_string()),
                    ..Default::default()
                }),
            )
            .await?;

        // Set Properties
        let err = blob_client
            .set_properties(Some(BlobClientSetPropertiesOptions {
                if_tags: Some(non_matching_expr.to_string()),
                blob_content_type: Some("application/octet-stream".to_string()),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        blob_client
            .set_properties(Some(BlobClientSetPropertiesOptions {
                if_tags: Some(matching_expr.to_string()),
                blob_content_type: Some("application/octet-stream".to_string()),
                ..Default::default()
            }))
            .await?;

        // Set Tags
        let err = blob_client
            .set_tags(
                RequestContent::try_from(BlobTags::from(HashMap::from([(
                    "env".to_string(),
                    "test".to_string(),
                )])))?,
                Some(BlobClientSetTagsOptions {
                    if_tags: Some(non_matching_expr.to_string()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        blob_client
            .set_tags(
                RequestContent::try_from(BlobTags::from(HashMap::from([(
                    "env".to_string(),
                    "test".to_string(),
                )])))?,
                Some(BlobClientSetTagsOptions {
                    if_tags: Some(matching_expr.to_string()),
                    ..Default::default()
                }),
            )
            .await?;

        // Set Tier - if_tags Only (No ETag/Time Conditions on This Method)
        let err = blob_client
            .set_tier(
                AccessTier::Cool,
                Some(BlobClientSetTierOptions {
                    if_tags: Some(non_matching_expr.to_string()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        blob_client
            .set_tier(
                AccessTier::Cool,
                Some(BlobClientSetTierOptions {
                    if_tags: Some(matching_expr.to_string()),
                    ..Default::default()
                }),
            )
            .await?;

        // Lease Operations

        // Acquire Lease - Failure
        let err = blob_client
            .acquire_lease(
                -1,
                Some(BlobClientAcquireLeaseOptions {
                    if_tags: Some(non_matching_expr.to_string()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // Acquire Lease - Success
        let lease_resp = blob_client
            .acquire_lease(
                -1,
                Some(BlobClientAcquireLeaseOptions {
                    if_tags: Some(matching_expr.to_string()),
                    ..Default::default()
                }),
            )
            .await?;
        let lease_id_1 = lease_resp.lease_id()?.unwrap().to_string();

        // Renew Lease - Failure
        let err = blob_client
            .renew_lease(
                lease_id_1.clone(),
                Some(BlobClientRenewLeaseOptions {
                    if_tags: Some(non_matching_expr.to_string()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // Renew Lease - Success
        blob_client
            .renew_lease(
                lease_id_1.clone(),
                Some(BlobClientRenewLeaseOptions {
                    if_tags: Some(matching_expr.to_string()),
                    ..Default::default()
                }),
            )
            .await?;

        // Change Lease - Failure
        let proposed_id = "cccccccc-cccc-cccc-cccc-cccccccccccc".to_string();
        let err = blob_client
            .change_lease(
                lease_id_1.clone(),
                proposed_id.clone(),
                Some(BlobClientChangeLeaseOptions {
                    if_tags: Some(non_matching_expr.to_string()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // Change Lease - Success
        let change_resp = blob_client
            .change_lease(
                lease_id_1,
                proposed_id,
                Some(BlobClientChangeLeaseOptions {
                    if_tags: Some(matching_expr.to_string()),
                    ..Default::default()
                }),
            )
            .await?;
        let lease_id_2 = change_resp.lease_id()?.unwrap().to_string();

        // Release Lease - Failure
        let err = blob_client
            .release_lease(
                lease_id_2.clone(),
                Some(BlobClientReleaseLeaseOptions {
                    if_tags: Some(non_matching_expr.to_string()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // Release Lease - Success
        blob_client
            .release_lease(
                lease_id_2,
                Some(BlobClientReleaseLeaseOptions {
                    if_tags: Some(matching_expr.to_string()),
                    ..Default::default()
                }),
            )
            .await?;

        // Break Lease
        blob_client.acquire_lease(-1, None).await?;
        let err = blob_client
            .break_lease(Some(BlobClientBreakLeaseOptions {
                if_tags: Some(non_matching_expr.to_string()),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        blob_client
            .break_lease(Some(BlobClientBreakLeaseOptions {
                if_tags: Some(matching_expr.to_string()),
                ..Default::default()
            }))
            .await?;

        // Delete
        let err = blob_client
            .delete(Some(BlobClientDeleteOptions {
                if_tags: Some(non_matching_expr.to_string()),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        blob_client
            .delete(Some(BlobClientDeleteOptions {
                if_tags: Some(matching_expr.to_string()),
                delete_snapshots: Some(DeleteSnapshotsOptionType::Include),
                ..Default::default()
            }))
            .await?;

        container_client.delete(None).await?;
        Ok(())
    }
}

mod block_blob_client {
    use super::*;

    #[recorded::test]
    #[ignore = "need to investigate live test pipeline failures"]
    async fn test_block_blob_client_conditional_headers(
        ctx: TestContext,
    ) -> Result<(), Box<dyn Error>> {
        // Recording Setup
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, true, StorageAccount::Standard, None).await?;
        let blob_client = container_client.blob_client(&get_blob_name(recording));
        let block_blob_client = blob_client.block_blob_client();

        // Upload - BlockBlobClientUploadOptions

        // Upload Initial Blob
        create_test_blob(&blob_client, None, None).await?;
        let props = blob_client.get_properties(None).await?;
        let etag = props.etag()?.unwrap().to_string();
        let last_modified = props.last_modified()?.unwrap();
        let before = last_modified - Duration::from_secs(60);
        let after = last_modified + Duration::from_secs(60);

        // if_match Failure
        let err = blob_client
            .upload(
                RequestContent::from(b"new-content".to_vec()),
                Some(BlockBlobClientUploadOptions {
                    if_match: Some(BAD_ETAG.to_string().into()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_none_match Failure
        let err = blob_client
            .upload(
                RequestContent::from(b"new-content".to_vec()),
                Some(BlockBlobClientUploadOptions {
                    if_none_match: Some(etag.clone().into()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_modified_since Failure
        let err = blob_client
            .upload(
                RequestContent::from(b"new-content".to_vec()),
                Some(BlockBlobClientUploadOptions {
                    if_modified_since: Some(after),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_unmodified_since Failure
        let err = blob_client
            .upload(
                RequestContent::from(b"new-content".to_vec()),
                Some(BlockBlobClientUploadOptions {
                    if_unmodified_since: Some(before),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_tags Failure - Upload With Tag First
        let err = blob_client
            .upload(
                RequestContent::from(b"new-content".to_vec()),
                Some(BlockBlobClientUploadOptions {
                    if_tags: Some("\"env\"='wrong'".to_string()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_match Success
        blob_client
            .upload(
                RequestContent::from(b"updated".to_vec()),
                Some(BlockBlobClientUploadOptions {
                    if_match: Some(etag.clone().into()),
                    ..Default::default()
                }),
            )
            .await?;

        // Commit Block List - BlockBlobClientCommitBlockListOptions

        // Stage a Block to Commit
        let block_id: Vec<u8> = b"1".to_vec();
        block_blob_client
            .stage_block(&block_id, 3, RequestContent::from(b"abc".to_vec()), None)
            .await?;

        let props = blob_client.get_properties(None).await?;
        let etag = props.etag()?.unwrap().to_string();
        let last_modified = props.last_modified()?.unwrap();
        let before = last_modified - Duration::from_secs(60);
        let after = last_modified + Duration::from_secs(60);
        let lookup = BlockLookupList {
            committed: Some(Vec::new()),
            latest: Some(vec![block_id.clone()]),
            uncommitted: Some(Vec::new()),
        };

        // if_match Failure
        let err = block_blob_client
            .commit_block_list(
                RequestContent::try_from(lookup.clone())?,
                Some(BlockBlobClientCommitBlockListOptions {
                    if_match: Some(BAD_ETAG.to_string().into()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_none_match Failure
        let err = block_blob_client
            .commit_block_list(
                RequestContent::try_from(lookup.clone())?,
                Some(BlockBlobClientCommitBlockListOptions {
                    if_none_match: Some(etag.clone().into()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_modified_since Failure
        let err = block_blob_client
            .commit_block_list(
                RequestContent::try_from(lookup.clone())?,
                Some(BlockBlobClientCommitBlockListOptions {
                    if_modified_since: Some(after),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_unmodified_since Failure
        let err = block_blob_client
            .commit_block_list(
                RequestContent::try_from(lookup.clone())?,
                Some(BlockBlobClientCommitBlockListOptions {
                    if_unmodified_since: Some(before),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_tags Failure
        let err = block_blob_client
            .commit_block_list(
                RequestContent::try_from(lookup.clone())?,
                Some(BlockBlobClientCommitBlockListOptions {
                    if_tags: Some("\"env\"='wrong'".to_string()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // Re-Stage (Stages Don't Consume Until Committed) and Commit Success
        block_blob_client
            .stage_block(&block_id, 3, RequestContent::from(b"abc".to_vec()), None)
            .await?;
        block_blob_client
            .commit_block_list(
                RequestContent::try_from(lookup)?,
                Some(BlockBlobClientCommitBlockListOptions {
                    if_match: Some(etag.into()),
                    ..Default::default()
                }),
            )
            .await?;

        // Get Block List - if_tags Only

        // Upload Blob With Tag
        blob_client
            .upload(
                RequestContent::from(b"tagged".to_vec()),
                Some(BlockBlobClientUploadOptions {
                    blob_tags_string: Some("kind=block".to_string()),
                    ..Default::default()
                }),
            )
            .await?;

        let err = block_blob_client
            .get_block_list(
                BlockListType::All,
                Some(BlockBlobClientGetBlockListOptions {
                    if_tags: Some("\"kind\"='wrong'".to_string()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        block_blob_client
            .get_block_list(
                BlockListType::All,
                Some(BlockBlobClientGetBlockListOptions {
                    if_tags: Some("\"kind\"='block'".to_string()),
                    ..Default::default()
                }),
            )
            .await?;

        container_client.delete(None).await?;
        Ok(())
    }
}

mod append_blob_client {
    use super::*;

    #[recorded::test]
    #[ignore = "need to investigate live test pipeline failures"]
    async fn test_append_blob_client_conditional_headers(
        ctx: TestContext,
    ) -> Result<(), Box<dyn Error>> {
        // Recording Setup
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, true, StorageAccount::Standard, None).await?;
        let blob_client = container_client.blob_client(&get_blob_name(recording));
        let append_blob_client = blob_client.append_blob_client();

        // Create Initial Append Blob (No Conditions)
        append_blob_client.create(None).await?;
        let props = blob_client.get_properties(None).await?;
        let etag = props.etag()?.unwrap().to_string();
        let last_modified = props.last_modified()?.unwrap();
        let before = last_modified - Duration::from_secs(60);
        let after = last_modified + Duration::from_secs(60);

        // Create - AppendBlobClientCreateOptions

        // if_match Failure
        let err = append_blob_client
            .create(Some(AppendBlobClientCreateOptions {
                if_match: Some(BAD_ETAG.to_string().into()),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_none_match Failure
        let err = append_blob_client
            .create(Some(AppendBlobClientCreateOptions {
                if_none_match: Some(etag.clone().into()),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_modified_since Failure
        let err = append_blob_client
            .create(Some(AppendBlobClientCreateOptions {
                if_modified_since: Some(after),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_unmodified_since Failure
        let err = append_blob_client
            .create(Some(AppendBlobClientCreateOptions {
                if_unmodified_since: Some(before),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_tags Failure
        let err = append_blob_client
            .create(Some(AppendBlobClientCreateOptions {
                if_tags: Some("\"env\"='missing'".to_string()),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // Create Success With if_match
        append_blob_client
            .create(Some(AppendBlobClientCreateOptions {
                if_match: Some(etag.into()),
                blob_tags_string: Some("env=test".to_string()),
                ..Default::default()
            }))
            .await?;

        let props = blob_client.get_properties(None).await?;
        let etag = props.etag()?.unwrap().to_string();
        let last_modified = props.last_modified()?.unwrap();
        let before = last_modified - Duration::from_secs(60);
        let after = last_modified + Duration::from_secs(60);

        // Append Block - AppendBlobClientAppendBlockOptions

        let chunk = RequestContent::from(b"hello".to_vec());

        // if_match Failure
        let err = append_blob_client
            .append_block(
                chunk.clone(),
                5u64,
                Some(AppendBlobClientAppendBlockOptions {
                    if_match: Some(BAD_ETAG.to_string().into()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_none_match Failure
        let err = append_blob_client
            .append_block(
                chunk.clone(),
                5u64,
                Some(AppendBlobClientAppendBlockOptions {
                    if_none_match: Some(etag.clone().into()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_modified_since Failure
        let err = append_blob_client
            .append_block(
                chunk.clone(),
                5u64,
                Some(AppendBlobClientAppendBlockOptions {
                    if_modified_since: Some(after),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_unmodified_since Failure
        let err = append_blob_client
            .append_block(
                chunk.clone(),
                5u64,
                Some(AppendBlobClientAppendBlockOptions {
                    if_unmodified_since: Some(before),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_tags Failure
        let err = append_blob_client
            .append_block(
                chunk.clone(),
                5u64,
                Some(AppendBlobClientAppendBlockOptions {
                    if_tags: Some("\"env\"='wrong'".to_string()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // Append Block Success
        append_blob_client
            .append_block(
                chunk,
                5u64,
                Some(AppendBlobClientAppendBlockOptions {
                    if_match: Some(etag.into()),
                    ..Default::default()
                }),
            )
            .await?;

        let props = blob_client.get_properties(None).await?;
        let etag = props.etag()?.unwrap().to_string();
        let last_modified = props.last_modified()?.unwrap();
        let before = last_modified - Duration::from_secs(60);
        let after = last_modified + Duration::from_secs(60);

        // Seal - AppendBlobClientSealOptions (No if_tags)

        // if_match Failure
        let err = append_blob_client
            .seal(Some(AppendBlobClientSealOptions {
                if_match: Some(BAD_ETAG.to_string().into()),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_none_match Failure
        let err = append_blob_client
            .seal(Some(AppendBlobClientSealOptions {
                if_none_match: Some(etag.clone().into()),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_modified_since Failure
        let err = append_blob_client
            .seal(Some(AppendBlobClientSealOptions {
                if_modified_since: Some(after),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_unmodified_since Failure
        let err = append_blob_client
            .seal(Some(AppendBlobClientSealOptions {
                if_unmodified_since: Some(before),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // Seal Success
        append_blob_client
            .seal(Some(AppendBlobClientSealOptions {
                if_match: Some(etag.into()),
                ..Default::default()
            }))
            .await?;

        container_client.delete(None).await?;
        Ok(())
    }
}

mod page_blob_client {
    use super::*;

    #[recorded::test]
    #[ignore = "need to investigate live test pipeline failures"]
    async fn test_page_blob_client_conditional_headers(
        ctx: TestContext,
    ) -> Result<(), Box<dyn Error>> {
        // Recording Setup
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, true, StorageAccount::Standard, None).await?;
        let blob_client = container_client.blob_client(&get_blob_name(recording));
        let page_blob_client = blob_client.page_blob_client();

        // Pages Must Be 512-Byte Aligned
        const PAGE_SIZE: usize = 512;
        const BLOB_SIZE: u64 = PAGE_SIZE as u64;

        // Create - PageBlobClientCreateOptions

        // Create Initial Page Blob
        page_blob_client.create(BLOB_SIZE, None).await?;
        let props = blob_client.get_properties(None).await?;
        let etag = props.etag()?.unwrap().to_string();
        let last_modified = props.last_modified()?.unwrap();
        let before = last_modified - Duration::from_secs(60);
        let after = last_modified + Duration::from_secs(60);

        // if_match Failure on Re-Create
        let err = page_blob_client
            .create(
                BLOB_SIZE,
                Some(PageBlobClientCreateOptions {
                    if_match: Some(BAD_ETAG.to_string().into()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_none_match Failure
        let err = page_blob_client
            .create(
                BLOB_SIZE,
                Some(PageBlobClientCreateOptions {
                    if_none_match: Some(etag.clone().into()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_modified_since Failure
        let err = page_blob_client
            .create(
                BLOB_SIZE,
                Some(PageBlobClientCreateOptions {
                    if_modified_since: Some(after),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_unmodified_since Failure
        let err = page_blob_client
            .create(
                BLOB_SIZE,
                Some(PageBlobClientCreateOptions {
                    if_unmodified_since: Some(before),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_tags Failure (No Tags Set Yet)
        let err = page_blob_client
            .create(
                BLOB_SIZE,
                Some(PageBlobClientCreateOptions {
                    if_tags: Some("\"env\"='missing'".to_string()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // Create Success With if_match
        page_blob_client
            .create(
                BLOB_SIZE,
                Some(PageBlobClientCreateOptions {
                    if_match: Some(etag.into()),
                    blob_tags_string: Some("env=test".to_string()),
                    ..Default::default()
                }),
            )
            .await?;

        let props = blob_client.get_properties(None).await?;
        let etag = props.etag()?.unwrap().to_string();
        let last_modified = props.last_modified()?.unwrap();
        let before = last_modified - Duration::from_secs(60);
        let after = last_modified + Duration::from_secs(60);

        // Upload Pages - PageBlobClientUploadPagesOptions

        let page_data = RequestContent::from(vec![1u8; PAGE_SIZE]);
        let range = HttpRange::new(0, PAGE_SIZE as u64).to_string();

        // if_match Failure
        let err = page_blob_client
            .upload_pages(
                page_data.clone(),
                PAGE_SIZE as u64,
                range.clone(),
                Some(PageBlobClientUploadPagesOptions {
                    if_match: Some(BAD_ETAG.to_string().into()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_none_match Failure
        let err = page_blob_client
            .upload_pages(
                page_data.clone(),
                PAGE_SIZE as u64,
                range.clone(),
                Some(PageBlobClientUploadPagesOptions {
                    if_none_match: Some(etag.clone().into()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_modified_since Failure
        let err = page_blob_client
            .upload_pages(
                page_data.clone(),
                PAGE_SIZE as u64,
                range.clone(),
                Some(PageBlobClientUploadPagesOptions {
                    if_modified_since: Some(after),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_unmodified_since Failure
        let err = page_blob_client
            .upload_pages(
                page_data.clone(),
                PAGE_SIZE as u64,
                range.clone(),
                Some(PageBlobClientUploadPagesOptions {
                    if_unmodified_since: Some(before),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_tags Failure
        let err = page_blob_client
            .upload_pages(
                page_data.clone(),
                PAGE_SIZE as u64,
                range.clone(),
                Some(PageBlobClientUploadPagesOptions {
                    if_tags: Some("\"env\"='wrong'".to_string()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // Upload Pages Success
        page_blob_client
            .upload_pages(
                page_data,
                PAGE_SIZE as u64,
                range.clone(),
                Some(PageBlobClientUploadPagesOptions {
                    if_match: Some(etag.into()),
                    ..Default::default()
                }),
            )
            .await?;

        let props = blob_client.get_properties(None).await?;
        let etag = props.etag()?.unwrap().to_string();
        let last_modified = props.last_modified()?.unwrap();
        let before = last_modified - Duration::from_secs(60);

        // Clear Pages - PageBlobClientClearPagesOptions

        // if_match Failure
        let err = page_blob_client
            .clear_pages(
                range.clone(),
                Some(PageBlobClientClearPagesOptions {
                    if_match: Some(BAD_ETAG.to_string().into()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_tags Failure
        let err = page_blob_client
            .clear_pages(
                range.clone(),
                Some(PageBlobClientClearPagesOptions {
                    if_tags: Some("\"env\"='wrong'".to_string()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_unmodified_since Failure
        let err = page_blob_client
            .clear_pages(
                range.clone(),
                Some(PageBlobClientClearPagesOptions {
                    if_unmodified_since: Some(before),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // Clear Pages Success
        page_blob_client
            .clear_pages(
                range,
                Some(PageBlobClientClearPagesOptions {
                    if_match: Some(etag.into()),
                    ..Default::default()
                }),
            )
            .await?;

        let props = blob_client.get_properties(None).await?;
        let etag = props.etag()?.unwrap().to_string();
        let last_modified = props.last_modified()?.unwrap();
        let before = last_modified - Duration::from_secs(60);
        let after = last_modified + Duration::from_secs(60);

        // Get Page Ranges - PageBlobClientGetPageRangesOptions

        // if_match Failure
        let err = page_blob_client
            .get_page_ranges(Some(PageBlobClientGetPageRangesOptions {
                if_match: Some(BAD_ETAG.to_string().into()),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_tags Failure
        let err = page_blob_client
            .get_page_ranges(Some(PageBlobClientGetPageRangesOptions {
                if_tags: Some("\"env\"='wrong'".to_string()),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_modified_since - Not Modified (304)
        let err = page_blob_client
            .get_page_ranges(Some(PageBlobClientGetPageRangesOptions {
                if_modified_since: Some(after),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::NotModified,
            err.unwrap_err().http_status().unwrap()
        );
        // Get Page Ranges Success
        page_blob_client
            .get_page_ranges(Some(PageBlobClientGetPageRangesOptions {
                if_match: Some(etag.clone().into()),
                ..Default::default()
            }))
            .await?;

        // Resize - PageBlobClientResizeOptions

        // if_match Failure
        let err = page_blob_client
            .resize(
                BLOB_SIZE * 2,
                Some(PageBlobClientResizeOptions {
                    if_match: Some(BAD_ETAG.to_string().into()),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_modified_since Failure
        let err = page_blob_client
            .resize(
                BLOB_SIZE * 2,
                Some(PageBlobClientResizeOptions {
                    if_modified_since: Some(after),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_unmodified_since Failure
        let err = page_blob_client
            .resize(
                BLOB_SIZE * 2,
                Some(PageBlobClientResizeOptions {
                    if_unmodified_since: Some(before),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // Resize Success
        page_blob_client
            .resize(
                BLOB_SIZE * 2,
                Some(PageBlobClientResizeOptions {
                    if_match: Some(etag.clone().into()),
                    ..Default::default()
                }),
            )
            .await?;

        let props = blob_client.get_properties(None).await?;
        let etag = props.etag()?.unwrap().to_string();
        let last_modified = props.last_modified()?.unwrap();
        let before = last_modified - Duration::from_secs(60);
        let after = last_modified + Duration::from_secs(60);

        // Set Sequence Number - PageBlobClientSetSequenceNumberOptions

        // if_match Failure
        let err = page_blob_client
            .set_sequence_number(
                SequenceNumberActionType::Update,
                Some(PageBlobClientSetSequenceNumberOptions {
                    if_match: Some(BAD_ETAG.to_string().into()),
                    blob_sequence_number: Some(1),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_none_match Failure
        let err = page_blob_client
            .set_sequence_number(
                SequenceNumberActionType::Update,
                Some(PageBlobClientSetSequenceNumberOptions {
                    if_none_match: Some(etag.clone().into()),
                    blob_sequence_number: Some(1),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_modified_since Failure
        let err = page_blob_client
            .set_sequence_number(
                SequenceNumberActionType::Update,
                Some(PageBlobClientSetSequenceNumberOptions {
                    if_modified_since: Some(after),
                    blob_sequence_number: Some(1),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_unmodified_since Failure
        let err = page_blob_client
            .set_sequence_number(
                SequenceNumberActionType::Update,
                Some(PageBlobClientSetSequenceNumberOptions {
                    if_unmodified_since: Some(before),
                    blob_sequence_number: Some(1),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_tags Failure
        let err = page_blob_client
            .set_sequence_number(
                SequenceNumberActionType::Update,
                Some(PageBlobClientSetSequenceNumberOptions {
                    if_tags: Some("\"env\"='wrong'".to_string()),
                    blob_sequence_number: Some(1),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // Set Sequence Number Success
        page_blob_client
            .set_sequence_number(
                SequenceNumberActionType::Update,
                Some(PageBlobClientSetSequenceNumberOptions {
                    if_match: Some(etag.into()),
                    blob_sequence_number: Some(42),
                    ..Default::default()
                }),
            )
            .await?;

        container_client.delete(None).await?;
        Ok(())
    }
}

mod blob_container_client {
    use super::*;

    #[recorded::test]
    #[ignore = "need to investigate live test pipeline failures"]
    async fn test_blob_container_client_conditional_headers(
        ctx: TestContext,
    ) -> Result<(), Box<dyn Error>> {
        // Recording Setup
        let recording = ctx.recording();
        // Do Not Auto-Create; Capture Creation Timestamp Ourselves
        let container_client =
            get_container_client(recording, false, StorageAccount::Standard, None).await?;
        container_client.create(None).await?;

        let props = container_client.get_properties(None).await?;
        let last_modified = props.last_modified()?.unwrap();
        let before = last_modified - Duration::from_secs(60);
        let after = last_modified + Duration::from_secs(60);

        // Delete - Failure Cases Only (Container Survives for Subsequent Operations)

        // if_unmodified_since=before: Container Was Modified Since before, 412
        let err = container_client
            .delete(Some(BlobContainerClientDeleteOptions {
                if_unmodified_since: Some(before),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_modified_since=after: Container Has Not Been Modified After after, 412
        let err = container_client
            .delete(Some(BlobContainerClientDeleteOptions {
                if_modified_since: Some(after),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );

        // Set Metadata - if_modified_since Only (No if_unmodified_since for Containers)

        let metadata = HashMap::from([("key".to_string(), "val".to_string())]);
        // if_modified_since=after Failure
        let err = container_client
            .set_metadata(
                &metadata,
                Some(BlobContainerClientSetMetadataOptions {
                    if_modified_since: Some(after),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_modified_since=before Success
        container_client
            .set_metadata(
                &metadata,
                Some(BlobContainerClientSetMetadataOptions {
                    if_modified_since: Some(before),
                    ..Default::default()
                }),
            )
            .await?;
        // Re-Capture After Set Metadata
        let props = container_client.get_properties(None).await?;
        let last_modified = props.last_modified()?.unwrap();
        let before = last_modified - Duration::from_secs(60);
        let after = last_modified + Duration::from_secs(60);

        // Set Access Policy - if_modified_since and if_unmodified_since

        // if_unmodified_since=before Failure
        let err = container_client
            .set_access_policy(
                RequestContent::try_from(SignedIdentifiers::default())?,
                Some(BlobContainerClientSetAccessPolicyOptions {
                    if_unmodified_since: Some(before),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // if_modified_since=after Failure
        let err = container_client
            .set_access_policy(
                RequestContent::try_from(SignedIdentifiers::default())?,
                Some(BlobContainerClientSetAccessPolicyOptions {
                    if_modified_since: Some(after),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // Set Access Policy Success
        container_client
            .set_access_policy(
                RequestContent::try_from(SignedIdentifiers::default())?,
                Some(BlobContainerClientSetAccessPolicyOptions {
                    if_unmodified_since: Some(after),
                    ..Default::default()
                }),
            )
            .await?;

        // Re-Capture After Set Access Policy
        let props = container_client.get_properties(None).await?;
        let last_modified = props.last_modified()?.unwrap();
        let before = last_modified - Duration::from_secs(60);
        let after = last_modified + Duration::from_secs(60);

        // Lease Operations - if_modified_since and if_unmodified_since

        // Acquire Lease - Failure
        let err = container_client
            .acquire_lease(
                -1,
                Some(BlobContainerClientAcquireLeaseOptions {
                    if_modified_since: Some(after),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // Acquire Lease - Success
        let lease_resp = container_client
            .acquire_lease(
                -1,
                Some(BlobContainerClientAcquireLeaseOptions {
                    if_unmodified_since: Some(after),
                    ..Default::default()
                }),
            )
            .await?;
        let lease_id_1 = lease_resp.lease_id()?.unwrap().to_string();

        // Renew Lease - Failure
        let err = container_client
            .renew_lease(
                lease_id_1.clone(),
                Some(BlobContainerClientRenewLeaseOptions {
                    if_unmodified_since: Some(before),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // Renew Lease - Success
        container_client
            .renew_lease(
                lease_id_1.clone(),
                Some(BlobContainerClientRenewLeaseOptions {
                    if_modified_since: Some(before),
                    ..Default::default()
                }),
            )
            .await?;

        // Change Lease - Failure
        let proposed_id = "dddddddd-dddd-dddd-dddd-dddddddddddd".to_string();
        let err = container_client
            .change_lease(
                lease_id_1.clone(),
                proposed_id.clone(),
                Some(BlobContainerClientChangeLeaseOptions {
                    if_modified_since: Some(after),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // Change Lease - Success
        container_client
            .change_lease(
                lease_id_1,
                proposed_id.clone(),
                Some(BlobContainerClientChangeLeaseOptions {
                    if_unmodified_since: Some(after),
                    ..Default::default()
                }),
            )
            .await?;
        let lease_id_2 = proposed_id;

        // Release Lease - Failure
        let err = container_client
            .release_lease(
                lease_id_2.clone(),
                Some(BlobContainerClientReleaseLeaseOptions {
                    if_unmodified_since: Some(before),
                    ..Default::default()
                }),
            )
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        // Release Lease - Success
        container_client
            .release_lease(
                lease_id_2,
                Some(BlobContainerClientReleaseLeaseOptions {
                    if_modified_since: Some(before),
                    ..Default::default()
                }),
            )
            .await?;

        // Break Lease - Acquire Fresh Lease, Then Test Break Conditions
        container_client.acquire_lease(-1, None).await?;
        let err = container_client
            .break_lease(Some(BlobContainerClientBreakLeaseOptions {
                if_modified_since: Some(after),
                ..Default::default()
            }))
            .await;
        assert_eq!(
            StatusCode::PreconditionFailed,
            err.unwrap_err().http_status().unwrap()
        );
        container_client
            .break_lease(Some(BlobContainerClientBreakLeaseOptions {
                if_modified_since: Some(before),
                ..Default::default()
            }))
            .await?;

        // Delete - Success
        container_client
            .delete(Some(BlobContainerClientDeleteOptions {
                if_modified_since: Some(before),
                ..Default::default()
            }))
            .await?;

        Ok(())
    }
}
