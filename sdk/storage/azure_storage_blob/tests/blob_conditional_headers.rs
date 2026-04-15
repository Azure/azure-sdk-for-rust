// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Conditional-header tests for Azure Storage Blob clients.
//!
//! Each condition family (etag, time, if-tags) is verified with a small
//! representative set of operations — a read, a write, and a destructive call —
//! rather than exhaustively hitting every API. The typed-blob tests (block,
//! append, page) verify that if_match is wired correctly for their
//! type-specific operations.

use azure_core::http::{RequestContent, StatusCode};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::models::{
    AccessTier, AppendBlobClientAppendBlockOptions, AppendBlobClientCreateOptions,
    AppendBlobClientSealOptions, BlobClientDeleteOptions, BlobClientDownloadOptions,
    BlobClientGetPropertiesResultHeaders, BlobClientSetMetadataOptions,
    BlobClientSetPropertiesOptions, BlobClientSetTierOptions, BlobContainerClientDeleteOptions,
    BlobContainerClientGetPropertiesResultHeaders, BlobContainerClientSetMetadataOptions,
    BlockBlobClientCommitBlockListOptions, BlockBlobClientGetBlockListOptions,
    BlockBlobClientUploadOptions, BlockListType, BlockLookupList, HttpRange,
    PageBlobClientClearPagesOptions, PageBlobClientCreateOptions, PageBlobClientResizeOptions,
    PageBlobClientSetSequenceNumberOptions, PageBlobClientUploadPagesOptions,
    SequenceNumberActionType,
};
use azure_storage_blob_test::{
    create_test_blob, get_blob_name, get_container_client, StorageAccount,
};
use std::{collections::HashMap, error::Error, time::Duration};

const BAD_ETAG: &str = "\"bad-etag-value\"";

mod blob_client {
    use super::*;

    /// Verifies if_match / if_none_match on a representative read (download),
    /// write (set_metadata), and destructive (delete) operation.
    #[recorded::test]
    async fn test_blob_client_etag_conditions(ctx: TestContext) -> Result<(), Box<dyn Error>> {
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, true, StorageAccount::Standard, None).await?;
        let blob_client = container_client.blob_client(&get_blob_name(recording));

        create_test_blob(&blob_client, None, None).await?;
        let props = blob_client.get_properties(None).await?;
        let etag = props.etag()?.unwrap().to_string();

        // Download - if_match success
        blob_client
            .download(Some(BlobClientDownloadOptions {
                if_match: Some(etag.clone()),
                ..Default::default()
            }))
            .await?;

        // Download - if_none_match 304
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

        // Download - if_match failure (bad etag)
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

        // Set Metadata - if_match failure
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

        // Set Metadata - if_match success
        blob_client
            .set_metadata(
                &metadata,
                Some(BlobClientSetMetadataOptions {
                    if_match: Some(etag.into()),
                    ..Default::default()
                }),
            )
            .await?;

        // Refresh etag after mutation
        let props = blob_client.get_properties(None).await?;
        let etag = props.etag()?.unwrap().to_string();

        // Delete - if_match failure
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

        // Delete - if_match success
        blob_client
            .delete(Some(BlobClientDeleteOptions {
                if_match: Some(etag.into()),
                ..Default::default()
            }))
            .await?;

        container_client.delete(None).await?;
        Ok(())
    }

    /// Verifies if_modified_since / if_unmodified_since on download (read),
    /// set_properties (write), and delete (destructive).
    #[recorded::test]
    async fn test_blob_client_time_conditions(ctx: TestContext) -> Result<(), Box<dyn Error>> {
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, true, StorageAccount::Standard, None).await?;
        let blob_client = container_client.blob_client(&get_blob_name(recording));

        create_test_blob(&blob_client, None, None).await?;
        let props = blob_client.get_properties(None).await?;
        let last_modified = props.last_modified()?.unwrap();
        let before = last_modified - Duration::from_secs(60);
        let after = last_modified + Duration::from_secs(60);

        // Download - if_modified_since=before (success)
        blob_client
            .download(Some(BlobClientDownloadOptions {
                if_modified_since: Some(before),
                ..Default::default()
            }))
            .await?;

        // Download - if_modified_since=after (304)
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

        // Download - if_unmodified_since=after (success)
        blob_client
            .download(Some(BlobClientDownloadOptions {
                if_unmodified_since: Some(after),
                ..Default::default()
            }))
            .await?;

        // Download - if_unmodified_since=before (412)
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

        // Set Properties - if_modified_since=after failure
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

        // Set Properties - if_unmodified_since=after success
        blob_client
            .set_properties(Some(BlobClientSetPropertiesOptions {
                if_unmodified_since: Some(after),
                blob_content_type: Some("application/octet-stream".to_string()),
                ..Default::default()
            }))
            .await?;

        // Refresh after mutation
        let props = blob_client.get_properties(None).await?;
        let last_modified = props.last_modified()?.unwrap();
        let before = last_modified - Duration::from_secs(60);

        // Delete - if_unmodified_since=before failure
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

        // Delete - if_modified_since=before success
        blob_client
            .delete(Some(BlobClientDeleteOptions {
                if_modified_since: Some(before),
                ..Default::default()
            }))
            .await?;

        container_client.delete(None).await?;
        Ok(())
    }

    /// Verifies if_tags on download (read), set_tier (write-only-if_tags),
    /// and delete (destructive).
    #[recorded::test]
    async fn test_blob_client_if_tags_condition(ctx: TestContext) -> Result<(), Box<dyn Error>> {
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, true, StorageAccount::Standard, None).await?;
        let blob_client = container_client.blob_client(&get_blob_name(recording));

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

        // Download - matching success
        blob_client
            .download(Some(BlobClientDownloadOptions {
                if_tags: Some(matching_expr.to_string()),
                ..Default::default()
            }))
            .await?;

        // Download - non-matching failure
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

        // Set Tier - non-matching failure
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

        // Set Tier - matching success
        blob_client
            .set_tier(
                AccessTier::Cool,
                Some(BlobClientSetTierOptions {
                    if_tags: Some(matching_expr.to_string()),
                    ..Default::default()
                }),
            )
            .await?;

        // Delete - non-matching failure
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

        // Delete - matching success
        blob_client
            .delete(Some(BlobClientDeleteOptions {
                if_tags: Some(matching_expr.to_string()),
                ..Default::default()
            }))
            .await?;

        container_client.delete(None).await?;
        Ok(())
    }
}

mod block_blob_client {
    use super::*;

    /// Verifies if_match on upload and commit_block_list, and if_tags on
    /// get_block_list (the only condition it supports).
    #[recorded::test]
    async fn test_block_blob_client_conditional_headers(
        ctx: TestContext,
    ) -> Result<(), Box<dyn Error>> {
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, true, StorageAccount::Standard, None).await?;
        let blob_client = container_client.blob_client(&get_blob_name(recording));
        let block_blob_client = blob_client.block_blob_client();

        // Upload initial blob
        create_test_blob(&blob_client, None, None).await?;
        let props = blob_client.get_properties(None).await?;
        let etag = props.etag()?.unwrap().to_string();

        // Upload - if_match failure
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

        // Upload - if_match success
        blob_client
            .upload(
                RequestContent::from(b"updated".to_vec()),
                Some(BlockBlobClientUploadOptions {
                    if_match: Some(etag.into()),
                    ..Default::default()
                }),
            )
            .await?;

        // Commit Block List
        let block_id: Vec<u8> = b"1".to_vec();
        block_blob_client
            .stage_block(&block_id, 3, RequestContent::from(b"abc".to_vec()), None)
            .await?;
        let props = blob_client.get_properties(None).await?;
        let etag = props.etag()?.unwrap().to_string();
        let lookup = BlockLookupList {
            committed: Some(Vec::new()),
            latest: Some(vec![block_id.clone()]),
            uncommitted: Some(Vec::new()),
        };

        // Commit - if_match failure
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

        // Re-stage and commit - if_match success
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

        // Get Block List - if_tags (the only condition get_block_list supports)
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

    /// Verifies if_match on create, append_block, and seal.
    #[recorded::test]
    async fn test_append_blob_client_conditional_headers(
        ctx: TestContext,
    ) -> Result<(), Box<dyn Error>> {
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, true, StorageAccount::Standard, None).await?;
        let blob_client = container_client.blob_client(&get_blob_name(recording));
        let append_blob_client = blob_client.append_blob_client();

        // Create initial append blob
        append_blob_client.create(None).await?;
        let props = blob_client.get_properties(None).await?;
        let etag = props.etag()?.unwrap().to_string();

        // Create - if_match failure
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

        // Create - if_match success (overwrites)
        append_blob_client
            .create(Some(AppendBlobClientCreateOptions {
                if_match: Some(etag.into()),
                ..Default::default()
            }))
            .await?;

        let props = blob_client.get_properties(None).await?;
        let etag = props.etag()?.unwrap().to_string();

        // Append Block - if_match failure
        let chunk = RequestContent::from(b"hello".to_vec());
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

        // Append Block - if_match success
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

        // Seal - if_match failure
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

        // Seal - if_match success
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

    /// Verifies if_match on create, upload_pages, clear_pages, resize, and
    /// set_sequence_number.
    #[recorded::test]
    async fn test_page_blob_client_conditional_headers(
        ctx: TestContext,
    ) -> Result<(), Box<dyn Error>> {
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, true, StorageAccount::Standard, None).await?;
        let blob_client = container_client.blob_client(&get_blob_name(recording));
        let page_blob_client = blob_client.page_blob_client();

        const PAGE_SIZE: usize = 512;
        const BLOB_SIZE: u64 = PAGE_SIZE as u64;

        // Create initial page blob
        page_blob_client.create(BLOB_SIZE, None).await?;
        let props = blob_client.get_properties(None).await?;
        let etag = props.etag()?.unwrap().to_string();

        // Create - if_match failure
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

        // Create - if_match success
        page_blob_client
            .create(
                BLOB_SIZE,
                Some(PageBlobClientCreateOptions {
                    if_match: Some(etag.into()),
                    ..Default::default()
                }),
            )
            .await?;

        let props = blob_client.get_properties(None).await?;
        let etag = props.etag()?.unwrap().to_string();
        let page_data = RequestContent::from(vec![1u8; PAGE_SIZE]);
        let range = HttpRange::new(0, PAGE_SIZE as u64).to_string();

        // Upload Pages - if_match failure
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

        // Upload Pages - if_match success
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

        // Clear Pages - if_match failure
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

        // Clear Pages - if_match success
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

        // Resize - if_match failure
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

        // Resize - if_match success
        page_blob_client
            .resize(
                BLOB_SIZE * 2,
                Some(PageBlobClientResizeOptions {
                    if_match: Some(etag.into()),
                    ..Default::default()
                }),
            )
            .await?;

        let props = blob_client.get_properties(None).await?;
        let etag = props.etag()?.unwrap().to_string();

        // Set Sequence Number - if_match failure
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

        // Set Sequence Number - if_match success
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

    /// Verifies if_modified_since / if_unmodified_since on container delete,
    /// set_metadata, and the final successful delete.
    #[recorded::test]
    async fn test_blob_container_client_conditional_headers(
        ctx: TestContext,
    ) -> Result<(), Box<dyn Error>> {
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, false, StorageAccount::Standard, None).await?;
        container_client.create(None).await?;

        let props = container_client.get_properties(None).await?;
        let last_modified = props.last_modified()?.unwrap();
        let before = last_modified - Duration::from_secs(60);
        let after = last_modified + Duration::from_secs(60);

        // Delete - if_unmodified_since=before failure (container was modified after before)
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

        // Delete - if_modified_since=after failure (container has not been modified after after)
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

        // Set Metadata - if_modified_since=after failure
        let metadata = HashMap::from([("key".to_string(), "val".to_string())]);
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

        // Set Metadata - if_modified_since=before success
        container_client
            .set_metadata(
                &metadata,
                Some(BlobContainerClientSetMetadataOptions {
                    if_modified_since: Some(before),
                    ..Default::default()
                }),
            )
            .await?;

        // Refresh after mutation
        let props = container_client.get_properties(None).await?;
        let last_modified = props.last_modified()?.unwrap();
        let before = last_modified - Duration::from_secs(60);

        // Delete - if_modified_since=before success
        container_client
            .delete(Some(BlobContainerClientDeleteOptions {
                if_modified_since: Some(before),
                ..Default::default()
            }))
            .await?;

        Ok(())
    }
}
