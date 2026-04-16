// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::RequestContent;
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::{
    format_page_range,
    models::{
        AppendBlobClientAppendBlockFromUrlOptions, AppendBlobClientAppendBlockOptions,
        AppendBlobClientCreateOptions, BlobClientCreateSnapshotOptions,
        BlobClientCreateSnapshotResultHeaders, BlobClientDownloadOptions,
        BlobClientGetPropertiesOptions, BlobClientGetPropertiesResultHeaders,
        BlobClientSetMetadataOptions, BlobType, BlockBlobClientCommitBlockListOptions,
        BlockBlobClientStageBlockFromUrlOptions, BlockBlobClientStageBlockOptions,
        BlockBlobClientUploadBlobFromUrlOptions, BlockBlobClientUploadOptions,
        PageBlobClientClearPagesOptions, PageBlobClientCreateOptions, PageBlobClientResizeOptions,
        PageBlobClientUploadPagesFromUrlOptions, PageBlobClientUploadPagesOptions,
    },
};
use azure_storage_blob_test::{
    assert_bad_request_or_conflict, block_lookup, create_test_blob, get_blob_name,
    get_container_client, get_cpk, get_cpk_2, get_invalid_encryption_scope,
    get_valid_encryption_scope, invalid_key_sha256, StorageAccount,
};
use std::{collections::HashMap, error::Error};

mod blob_client {
    use super::*;

    #[recorded::test]
    async fn test_upload_blob_partial_cpk_options_fail(
        ctx: TestContext,
    ) -> Result<(), Box<dyn Error>> {
        // Recording Setup
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, true, StorageAccount::Standard, None).await?;
        let (encryption_algorithm, encryption_key, _) = get_cpk();

        // Key Only Scenario
        let key_only_blob =
            container_client.blob_client(&format!("{}-cpk-key-only", get_blob_name(recording)));
        let result = key_only_blob
            .upload(
                RequestContent::from(b"key-only".to_vec()),
                Some(BlockBlobClientUploadOptions {
                    encryption_key: Some(encryption_key.clone()),
                    ..Default::default()
                }),
            )
            .await;
        assert_bad_request_or_conflict(result.unwrap_err().http_status());

        // Key + Algorithm Without Hash Scenario
        let key_plus_algorithm_blob = container_client.blob_client(&format!(
            "{}-cpk-key-plus-algorithm",
            get_blob_name(recording)
        ));
        let result = key_plus_algorithm_blob
            .upload(
                RequestContent::from(b"key-plus-algorithm".to_vec()),
                Some(BlockBlobClientUploadOptions {
                    encryption_algorithm: Some(encryption_algorithm),
                    encryption_key: Some(encryption_key),
                    encryption_key_sha256: None,
                    ..Default::default()
                }),
            )
            .await;
        assert_bad_request_or_conflict(result.unwrap_err().http_status());

        container_client.delete(None).await?;
        Ok(())
    }

    #[recorded::test]
    async fn test_blob_client_cpk_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
        // Recording Setup
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, true, StorageAccount::Standard, None).await?;
        let (algo, key, hash) = get_cpk();
        let blob_client = container_client.blob_client(&get_blob_name(recording));

        // CPK Upload
        let content = b"blob-cpk-operations";
        blob_client
            .upload(
                RequestContent::from(content.to_vec()),
                Some(BlockBlobClientUploadOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(key.clone()),
                    encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;

        // CPK Download
        let response = blob_client
            .download(Some(BlobClientDownloadOptions {
                encryption_algorithm: Some(algo),
                encryption_key: Some(key.clone()),
                encryption_key_sha256: Some(hash.clone()),
                ..Default::default()
            }))
            .await?;
        let body_data = response.body.collect().await?;
        assert_eq!(content.to_vec(), body_data);

        // Wrong Hash Download
        let err = blob_client
            .download(Some(BlobClientDownloadOptions {
                encryption_algorithm: Some(algo),
                encryption_key: Some(key.clone()),
                encryption_key_sha256: Some(invalid_key_sha256()),
                ..Default::default()
            }))
            .await;
        assert_bad_request_or_conflict(err.unwrap_err().http_status());

        // Get Properties with CPK
        let props = blob_client
            .get_properties(Some(BlobClientGetPropertiesOptions {
                encryption_algorithm: Some(algo),
                encryption_key: Some(key.clone()),
                encryption_key_sha256: Some(hash.clone()),
                ..Default::default()
            }))
            .await?;
        assert_eq!(
            u64::try_from(content.len())?,
            props.content_length()?.unwrap()
        );

        // Wrong Hash Get Properties
        let err = blob_client
            .get_properties(Some(BlobClientGetPropertiesOptions {
                encryption_algorithm: Some(algo),
                encryption_key: Some(key.clone()),
                encryption_key_sha256: Some(invalid_key_sha256()),
                ..Default::default()
            }))
            .await;
        assert_bad_request_or_conflict(err.unwrap_err().http_status());

        // Set Metadata with CPK
        let metadata = HashMap::from([("cpk_key".to_string(), "cpk_value".to_string())]);
        blob_client
            .set_metadata(
                &metadata,
                Some(BlobClientSetMetadataOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(key.clone()),
                    encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;
        let props = blob_client
            .get_properties(Some(BlobClientGetPropertiesOptions {
                encryption_algorithm: Some(algo),
                encryption_key: Some(key.clone()),
                encryption_key_sha256: Some(hash.clone()),
                ..Default::default()
            }))
            .await?;
        assert_eq!(metadata, props.metadata()?);

        // Invalid Scope Set Metadata
        let err = blob_client
            .set_metadata(
                &metadata,
                Some(BlobClientSetMetadataOptions {
                    encryption_scope: Some(get_invalid_encryption_scope()),
                    ..Default::default()
                }),
            )
            .await;
        assert_bad_request_or_conflict(err.unwrap_err().http_status());

        // Create Snapshot with CPK
        let snapshot_response = blob_client
            .create_snapshot(Some(BlobClientCreateSnapshotOptions {
                encryption_algorithm: Some(algo),
                encryption_key: Some(key.clone()),
                encryption_key_sha256: Some(hash.clone()),
                ..Default::default()
            }))
            .await?;
        let snapshot_id = snapshot_response.snapshot()?.unwrap();

        // Invalid Scope Create Snapshot
        let err = blob_client
            .create_snapshot(Some(BlobClientCreateSnapshotOptions {
                encryption_scope: Some(get_invalid_encryption_scope()),
                ..Default::default()
            }))
            .await;
        assert_bad_request_or_conflict(err.unwrap_err().http_status());

        // Overwrite with v2
        let v2 = b"blob-cpk-operations-v2";
        blob_client
            .upload(
                RequestContent::from(v2.to_vec()),
                Some(BlockBlobClientUploadOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(key.clone()),
                    encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;

        // Snapshot Download via Options Bag
        let response = blob_client
            .download(Some(BlobClientDownloadOptions {
                snapshot: Some(snapshot_id),
                encryption_algorithm: Some(algo),
                encryption_key: Some(key.clone()),
                encryption_key_sha256: Some(hash.clone()),
                ..Default::default()
            }))
            .await?;
        let body_data = response.body.collect().await?;
        assert_eq!(content.to_vec(), body_data);

        container_client.delete(None).await?;
        Ok(())
    }

    #[recorded::test]
    async fn test_blob_client_encryption_scope(ctx: TestContext) -> Result<(), Box<dyn Error>> {
        // Recording Setup
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, true, StorageAccount::Standard, None).await?;
        let scope = get_valid_encryption_scope();
        let blob_client =
            container_client.blob_client(&format!("{}-scope", get_blob_name(recording)));

        // Encryption Scope Upload
        let content = b"scope-success";
        blob_client
            .upload(
                RequestContent::from(content.to_vec()),
                Some(BlockBlobClientUploadOptions {
                    encryption_scope: Some(scope.clone()),
                    ..Default::default()
                }),
            )
            .await?;

        // Assert Encryption Scope
        let response = blob_client.get_properties(None).await?;
        assert_eq!(
            Some(scope.as_str()),
            response.encryption_scope()?.as_deref()
        );

        // Invalid Scope Upload
        let invalid_blob =
            container_client.blob_client(&format!("{}-bad-scope", get_blob_name(recording)));
        let err = invalid_blob
            .upload(
                RequestContent::from(b"bad scope".to_vec()),
                Some(BlockBlobClientUploadOptions {
                    encryption_scope: Some(get_invalid_encryption_scope()),
                    ..Default::default()
                }),
            )
            .await;
        assert_bad_request_or_conflict(err.unwrap_err().http_status());

        container_client.delete(None).await?;
        Ok(())
    }
}

mod block_blob_client {
    use super::*;

    #[recorded::test]
    async fn test_stage_block_partial_cpk_options_fail(
        ctx: TestContext,
    ) -> Result<(), Box<dyn Error>> {
        // Recording Setup
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, true, StorageAccount::Standard, None).await?;
        let (encryption_algorithm, encryption_key, _) = get_cpk();
        let blob_client =
            container_client.blob_client(&format!("{}-cpk-partial", get_blob_name(recording)));
        let block_blob_client = blob_client.block_blob_client();

        // Key Only Scenario
        let result = block_blob_client
            .stage_block(
                b"key-only",
                8,
                RequestContent::from(b"key-only".to_vec()),
                Some(BlockBlobClientStageBlockOptions {
                    encryption_key: Some(encryption_key.clone()),
                    ..Default::default()
                }),
            )
            .await;
        assert_bad_request_or_conflict(result.unwrap_err().http_status());

        // Key + Algorithm Without Hash Scenario
        let result = block_blob_client
            .stage_block(
                b"key-plus-algorithm",
                18,
                RequestContent::from(b"key-plus-algorithm".to_vec()),
                Some(BlockBlobClientStageBlockOptions {
                    encryption_algorithm: Some(encryption_algorithm),
                    encryption_key: Some(encryption_key),
                    encryption_key_sha256: None,
                    ..Default::default()
                }),
            )
            .await;
        assert_bad_request_or_conflict(result.unwrap_err().http_status());

        container_client.delete(None).await?;
        Ok(())
    }

    #[recorded::test]
    async fn test_block_blob_stage_and_commit_cpk(ctx: TestContext) -> Result<(), Box<dyn Error>> {
        // Recording Setup
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, true, StorageAccount::Standard, None).await?;
        let (algo, key, hash) = get_cpk();
        let blob_client = container_client.blob_client(&get_blob_name(recording));
        let block_blob_client = blob_client.block_blob_client();
        let block_id = b"block-1".to_vec();
        let content = b"stage-and-commit-cpk";

        // Stage Block with CPK
        block_blob_client
            .stage_block(
                &block_id,
                u64::try_from(content.len())?,
                RequestContent::from(content.to_vec()),
                Some(BlockBlobClientStageBlockOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(key.clone()),
                    encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;

        // Commit Block List with CPK
        block_blob_client
            .commit_block_list(
                block_lookup(block_id).try_into()?,
                Some(BlockBlobClientCommitBlockListOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(key.clone()),
                    encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;

        // Verify Content
        let response = blob_client
            .download(Some(BlobClientDownloadOptions {
                encryption_algorithm: Some(algo),
                encryption_key: Some(key.clone()),
                encryption_key_sha256: Some(hash.clone()),
                ..Default::default()
            }))
            .await?;
        let body_data = response.body.collect().await?;
        assert_eq!(content.to_vec(), body_data);

        // Invalid Scope Stage Block
        let invalid_blob =
            container_client.blob_client(&format!("{}-bad-scope", get_blob_name(recording)));
        let err = invalid_blob
            .block_blob_client()
            .stage_block(
                b"block-x",
                9,
                RequestContent::from(b"bad-scope".to_vec()),
                Some(BlockBlobClientStageBlockOptions {
                    encryption_scope: Some(get_invalid_encryption_scope()),
                    ..Default::default()
                }),
            )
            .await;
        assert_bad_request_or_conflict(err.unwrap_err().http_status());

        container_client.delete(None).await?;
        Ok(())
    }

    #[recorded::test]
    async fn test_upload_blob_from_url_cpk(ctx: TestContext) -> Result<(), Box<dyn Error>> {
        // Recording Setup
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, true, StorageAccount::Standard, None).await?;
        let (algo, key, hash) = get_cpk();
        let (_, wrong_key, wrong_hash) = get_cpk_2();

        // Setup
        let source_blob =
            container_client.blob_client(&format!("{}-source", get_blob_name(recording)));
        let source_content = b"upload-from-url-source";
        create_test_blob(
            &source_blob,
            Some(RequestContent::from(source_content.to_vec())),
            None,
        )
        .await?;

        // Upload from URL with Destination CPK
        let dest_blob =
            container_client.blob_client(&format!("{}-dest-cpk", get_blob_name(recording)));
        dest_blob
            .block_blob_client()
            .upload_blob_from_url(
                source_blob.url().as_str().into(),
                Some(BlockBlobClientUploadBlobFromUrlOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(key.clone()),
                    encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;
        let response = dest_blob
            .download(Some(BlobClientDownloadOptions {
                encryption_algorithm: Some(algo),
                encryption_key: Some(key.clone()),
                encryption_key_sha256: Some(hash.clone()),
                ..Default::default()
            }))
            .await?;
        let body_data = response.body.collect().await?;
        assert_eq!(source_content.to_vec(), body_data);

        // Setup CPK Source
        let cpk_source_blob =
            container_client.blob_client(&format!("{}-source-cpk", get_blob_name(recording)));
        let cpk_source_content = b"upload-from-url-cpk-source";
        cpk_source_blob
            .upload(
                RequestContent::from(cpk_source_content.to_vec()),
                Some(BlockBlobClientUploadOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(key.clone()),
                    encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;

        // Upload from URL with Source CPK
        let dest_source_cpk_blob =
            container_client.blob_client(&format!("{}-dest-source-cpk", get_blob_name(recording)));
        dest_source_cpk_blob
            .block_blob_client()
            .upload_blob_from_url(
                cpk_source_blob.url().as_str().into(),
                Some(BlockBlobClientUploadBlobFromUrlOptions {
                    source_encryption_algorithm: Some(algo),
                    source_encryption_key: Some(key.clone()),
                    source_encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;
        let response = dest_source_cpk_blob.download(None).await?;
        let body_data = response.body.collect().await?;
        assert_eq!(cpk_source_content.to_vec(), body_data);

        // Source CPK Mismatch
        let dest_mismatch_blob =
            container_client.blob_client(&format!("{}-dest-mismatch", get_blob_name(recording)));
        let err = dest_mismatch_blob
            .block_blob_client()
            .upload_blob_from_url(
                cpk_source_blob.url().as_str().into(),
                Some(BlockBlobClientUploadBlobFromUrlOptions {
                    source_encryption_algorithm: Some(algo),
                    source_encryption_key: Some(wrong_key),
                    source_encryption_key_sha256: Some(wrong_hash),
                    ..Default::default()
                }),
            )
            .await;
        assert_bad_request_or_conflict(err.unwrap_err().http_status());

        // Invalid Scope Upload from URL
        let dest_bad_scope_blob =
            container_client.blob_client(&format!("{}-dest-bad-scope", get_blob_name(recording)));
        let err = dest_bad_scope_blob
            .block_blob_client()
            .upload_blob_from_url(
                source_blob.url().as_str().into(),
                Some(BlockBlobClientUploadBlobFromUrlOptions {
                    encryption_scope: Some(get_invalid_encryption_scope()),
                    ..Default::default()
                }),
            )
            .await;
        assert_bad_request_or_conflict(err.unwrap_err().http_status());

        container_client.delete(None).await?;
        Ok(())
    }

    #[recorded::test]
    async fn test_stage_block_from_url_cpk(ctx: TestContext) -> Result<(), Box<dyn Error>> {
        // Recording Setup
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, true, StorageAccount::Standard, None).await?;
        let (algo, key, hash) = get_cpk();
        let (_, wrong_key, wrong_hash) = get_cpk_2();

        // Setup
        let source_blob =
            container_client.blob_client(&format!("{}-source", get_blob_name(recording)));
        let source_content = b"stage-from-url-source";
        create_test_blob(
            &source_blob,
            Some(RequestContent::from(source_content.to_vec())),
            None,
        )
        .await?;

        // Stage from URL with Destination CPK
        let dest_blob =
            container_client.blob_client(&format!("{}-dest-cpk", get_blob_name(recording)));
        let dest_block = dest_blob.block_blob_client();
        let block_id = b"block-1".to_vec();
        dest_block
            .stage_block_from_url(
                &block_id,
                u64::try_from(source_content.len())?,
                source_blob.url().as_str().into(),
                Some(BlockBlobClientStageBlockFromUrlOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(key.clone()),
                    encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;
        dest_block
            .commit_block_list(
                block_lookup(block_id).try_into()?,
                Some(BlockBlobClientCommitBlockListOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(key.clone()),
                    encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;
        let response = dest_blob
            .download(Some(BlobClientDownloadOptions {
                encryption_algorithm: Some(algo),
                encryption_key: Some(key.clone()),
                encryption_key_sha256: Some(hash.clone()),
                ..Default::default()
            }))
            .await?;
        let body_data = response.body.collect().await?;
        assert_eq!(source_content.to_vec(), body_data);

        // Setup CPK Source
        let cpk_source_blob =
            container_client.blob_client(&format!("{}-source-cpk", get_blob_name(recording)));
        let cpk_source_content = b"stage-from-url-cpk-source";
        cpk_source_blob
            .upload(
                RequestContent::from(cpk_source_content.to_vec()),
                Some(BlockBlobClientUploadOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(key.clone()),
                    encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;

        // Stage from URL with Source CPK
        let dest_source_cpk_blob =
            container_client.blob_client(&format!("{}-dest-source-cpk", get_blob_name(recording)));
        let dest_source_cpk_block = dest_source_cpk_blob.block_blob_client();
        let block_id = b"block-1".to_vec();
        dest_source_cpk_block
            .stage_block_from_url(
                &block_id,
                u64::try_from(cpk_source_content.len())?,
                cpk_source_blob.url().as_str().into(),
                Some(BlockBlobClientStageBlockFromUrlOptions {
                    source_encryption_algorithm: Some(algo),
                    source_encryption_key: Some(key.clone()),
                    source_encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;
        dest_source_cpk_block
            .commit_block_list(block_lookup(block_id).try_into()?, None)
            .await?;
        let response = dest_source_cpk_blob.download(None).await?;
        let body_data = response.body.collect().await?;
        assert_eq!(cpk_source_content.to_vec(), body_data);

        // Source CPK Mismatch
        let dest_mismatch_blob =
            container_client.blob_client(&format!("{}-dest-mismatch", get_blob_name(recording)));
        let err = dest_mismatch_blob
            .block_blob_client()
            .stage_block_from_url(
                b"block-1",
                u64::try_from(cpk_source_content.len())?,
                cpk_source_blob.url().as_str().into(),
                Some(BlockBlobClientStageBlockFromUrlOptions {
                    source_encryption_algorithm: Some(algo),
                    source_encryption_key: Some(wrong_key),
                    source_encryption_key_sha256: Some(wrong_hash),
                    ..Default::default()
                }),
            )
            .await;
        assert_bad_request_or_conflict(err.unwrap_err().http_status());

        // Invalid Scope Stage from URL
        let dest_bad_scope_blob =
            container_client.blob_client(&format!("{}-dest-bad-scope", get_blob_name(recording)));
        let err = dest_bad_scope_blob
            .block_blob_client()
            .stage_block_from_url(
                b"block-2",
                u64::try_from(source_content.len())?,
                source_blob.url().as_str().into(),
                Some(BlockBlobClientStageBlockFromUrlOptions {
                    encryption_scope: Some(get_invalid_encryption_scope()),
                    ..Default::default()
                }),
            )
            .await;
        assert_bad_request_or_conflict(err.unwrap_err().http_status());

        container_client.delete(None).await?;
        Ok(())
    }
}

mod append_blob_client {
    use super::*;

    #[recorded::test]
    async fn test_append_blob_partial_cpk_options_fail(
        ctx: TestContext,
    ) -> Result<(), Box<dyn Error>> {
        // Recording Setup
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, true, StorageAccount::Standard, None).await?;
        let (encryption_algorithm, encryption_key, _) = get_cpk();

        // Key Only Create Scenario
        let key_only_blob =
            container_client.blob_client(&format!("{}-cpk-key-only", get_blob_name(recording)));
        let result = key_only_blob
            .append_blob_client()
            .create(Some(AppendBlobClientCreateOptions {
                encryption_key: Some(encryption_key.clone()),
                ..Default::default()
            }))
            .await;
        assert_bad_request_or_conflict(result.unwrap_err().http_status());

        // Key + Algorithm Without Hash Append Block Scenario
        let key_plus_algorithm_blob = container_client.blob_client(&format!(
            "{}-cpk-key-plus-algorithm",
            get_blob_name(recording)
        ));
        key_plus_algorithm_blob
            .append_blob_client()
            .create(None)
            .await?;
        let result = key_plus_algorithm_blob
            .append_blob_client()
            .append_block(
                RequestContent::from(b"key-plus-algorithm".to_vec()),
                18,
                Some(AppendBlobClientAppendBlockOptions {
                    encryption_algorithm: Some(encryption_algorithm),
                    encryption_key: Some(encryption_key),
                    encryption_key_sha256: None,
                    ..Default::default()
                }),
            )
            .await;
        assert_bad_request_or_conflict(result.unwrap_err().http_status());

        container_client.delete(None).await?;
        Ok(())
    }

    #[recorded::test]
    async fn test_append_blob_cpk_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
        // Recording Setup
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, true, StorageAccount::Standard, None).await?;
        let (algo, key, hash) = get_cpk();
        let (_, wrong_key, wrong_hash) = get_cpk_2();

        // Setup Unencrypted Source
        let plain_source_blob =
            container_client.blob_client(&format!("{}-plain-source", get_blob_name(recording)));
        let plain_source_content = b"append-from-url-plain";
        create_test_blob(
            &plain_source_blob,
            Some(RequestContent::from(plain_source_content.to_vec())),
            None,
        )
        .await?;

        // Setup CPK Source
        let cpk_source_blob =
            container_client.blob_client(&format!("{}-cpk-source", get_blob_name(recording)));
        let cpk_source_content = b"append-from-url-cpk";
        cpk_source_blob
            .upload(
                RequestContent::from(cpk_source_content.to_vec()),
                Some(BlockBlobClientUploadOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(key.clone()),
                    encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;

        // Create append blob with CPK
        let blob_client = container_client.blob_client(&get_blob_name(recording));
        let append_blob = blob_client.append_blob_client();
        append_blob
            .create(Some(AppendBlobClientCreateOptions {
                encryption_algorithm: Some(algo),
                encryption_key: Some(key.clone()),
                encryption_key_sha256: Some(hash.clone()),
                ..Default::default()
            }))
            .await?;

        // Get Properties with CPK
        let props = blob_client
            .get_properties(Some(BlobClientGetPropertiesOptions {
                encryption_algorithm: Some(algo),
                encryption_key: Some(key.clone()),
                encryption_key_sha256: Some(hash.clone()),
                ..Default::default()
            }))
            .await?;
        assert_eq!(BlobType::AppendBlob, props.blob_type()?.unwrap());

        // Invalid Scope Create
        let bad_scope_blob =
            container_client.blob_client(&format!("{}-bad-scope", get_blob_name(recording)));
        let err = bad_scope_blob
            .append_blob_client()
            .create(Some(AppendBlobClientCreateOptions {
                encryption_scope: Some(get_invalid_encryption_scope()),
                ..Default::default()
            }))
            .await;
        assert_bad_request_or_conflict(err.unwrap_err().http_status());

        // Append Block with CPK
        let append_content = b"append-with-cpk";
        append_blob
            .append_block(
                RequestContent::from(append_content.to_vec()),
                u64::try_from(append_content.len())?,
                Some(AppendBlobClientAppendBlockOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(key.clone()),
                    encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;

        // Invalid Scope Append Block
        let err = append_blob
            .append_block(
                RequestContent::from(b"bad-scope".to_vec()),
                9,
                Some(AppendBlobClientAppendBlockOptions {
                    encryption_scope: Some(get_invalid_encryption_scope()),
                    ..Default::default()
                }),
            )
            .await;
        assert_bad_request_or_conflict(err.unwrap_err().http_status());

        // Append from URL with Destination CPK
        append_blob
            .append_block_from_url(
                plain_source_blob.url().as_str().into(),
                u64::try_from(plain_source_content.len())?,
                Some(AppendBlobClientAppendBlockFromUrlOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(key.clone()),
                    encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;

        // Append from URL with Source CPK
        append_blob
            .append_block_from_url(
                cpk_source_blob.url().as_str().into(),
                u64::try_from(cpk_source_content.len())?,
                Some(AppendBlobClientAppendBlockFromUrlOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(key.clone()),
                    encryption_key_sha256: Some(hash.clone()),
                    source_encryption_algorithm: Some(algo),
                    source_encryption_key: Some(key.clone()),
                    source_encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;

        // Verify Accumulated Content
        let response = blob_client
            .download(Some(BlobClientDownloadOptions {
                encryption_algorithm: Some(algo),
                encryption_key: Some(key.clone()),
                encryption_key_sha256: Some(hash.clone()),
                ..Default::default()
            }))
            .await?;
        let mut expected = append_content.to_vec();
        expected.extend_from_slice(plain_source_content);
        expected.extend_from_slice(cpk_source_content);
        let body_data = response.body.collect().await?;
        assert_eq!(expected, body_data);

        // Source CPK Mismatch
        let err = append_blob
            .append_block_from_url(
                cpk_source_blob.url().as_str().into(),
                u64::try_from(cpk_source_content.len())?,
                Some(AppendBlobClientAppendBlockFromUrlOptions {
                    source_encryption_algorithm: Some(algo),
                    source_encryption_key: Some(wrong_key.clone()),
                    source_encryption_key_sha256: Some(wrong_hash.clone()),
                    ..Default::default()
                }),
            )
            .await;
        assert_bad_request_or_conflict(err.unwrap_err().http_status());

        // Destination CPK Mismatch
        let err = append_blob
            .append_block_from_url(
                plain_source_blob.url().as_str().into(),
                u64::try_from(plain_source_content.len())?,
                Some(AppendBlobClientAppendBlockFromUrlOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(wrong_key),
                    encryption_key_sha256: Some(wrong_hash),
                    ..Default::default()
                }),
            )
            .await;
        assert_bad_request_or_conflict(err.unwrap_err().http_status());

        // Invalid Scope Append from URL
        let err = append_blob
            .append_block_from_url(
                plain_source_blob.url().as_str().into(),
                u64::try_from(plain_source_content.len())?,
                Some(AppendBlobClientAppendBlockFromUrlOptions {
                    encryption_scope: Some(get_invalid_encryption_scope()),
                    ..Default::default()
                }),
            )
            .await;
        assert_bad_request_or_conflict(err.unwrap_err().http_status());

        container_client.delete(None).await?;
        Ok(())
    }
}

mod page_blob_client {
    use super::*;

    #[recorded::test]
    async fn test_page_blob_partial_cpk_options_fail(
        ctx: TestContext,
    ) -> Result<(), Box<dyn Error>> {
        // Recording Setup
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, true, StorageAccount::Standard, None).await?;
        let (encryption_algorithm, encryption_key, _) = get_cpk();

        // Key Only Create Scenario
        let key_only_blob =
            container_client.blob_client(&format!("{}-cpk-key-only", get_blob_name(recording)));
        let result = key_only_blob
            .page_blob_client()
            .create(
                512,
                Some(PageBlobClientCreateOptions {
                    encryption_key: Some(encryption_key.clone()),
                    ..Default::default()
                }),
            )
            .await;
        assert_bad_request_or_conflict(result.unwrap_err().http_status());

        // Key + Algorithm Without Hash Upload Scenario
        let key_plus_algorithm_blob = container_client.blob_client(&format!(
            "{}-cpk-key-plus-algorithm",
            get_blob_name(recording)
        ));
        let key_plus_algorithm_page_blob = key_plus_algorithm_blob.page_blob_client();
        key_plus_algorithm_page_blob.create(512, None).await?;
        let result = key_plus_algorithm_page_blob
            .upload_pages(
                RequestContent::from(vec![b'P'; 512]),
                512,
                format_page_range(0, 512)?,
                Some(PageBlobClientUploadPagesOptions {
                    encryption_algorithm: Some(encryption_algorithm),
                    encryption_key: Some(encryption_key),
                    encryption_key_sha256: None,
                    ..Default::default()
                }),
            )
            .await;
        assert_bad_request_or_conflict(result.unwrap_err().http_status());

        container_client.delete(None).await?;
        Ok(())
    }

    #[recorded::test]
    async fn test_page_blob_cpk_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
        // Recording Setup
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, true, StorageAccount::Standard, None).await?;
        let (algo, key, hash) = get_cpk();
        let blob_client = container_client.blob_client(&get_blob_name(recording));
        let page_blob = blob_client.page_blob_client();

        // Create Page Blob with CPK
        page_blob
            .create(
                512,
                Some(PageBlobClientCreateOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(key.clone()),
                    encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;
        let props = blob_client
            .get_properties(Some(BlobClientGetPropertiesOptions {
                encryption_algorithm: Some(algo),
                encryption_key: Some(key.clone()),
                encryption_key_sha256: Some(hash.clone()),
                ..Default::default()
            }))
            .await?;
        assert_eq!(512, props.content_length()?.unwrap());

        // Setup Invalid Scope Blob
        let bad_scope_blob =
            container_client.blob_client(&format!("{}-bad-scope", get_blob_name(recording)));
        let bad_scope_page_blob = bad_scope_blob.page_blob_client();

        // Invalid Scope Create
        let err = bad_scope_blob
            .page_blob_client()
            .create(
                512,
                Some(PageBlobClientCreateOptions {
                    encryption_scope: Some(get_invalid_encryption_scope()),
                    ..Default::default()
                }),
            )
            .await;
        assert_bad_request_or_conflict(err.unwrap_err().http_status());

        // Upload Pages with CPK
        let content = vec![b'A'; 512];
        page_blob
            .upload_pages(
                RequestContent::from(content.clone()),
                512,
                format_page_range(0, 512)?,
                Some(PageBlobClientUploadPagesOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(key.clone()),
                    encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;
        let response = blob_client
            .download(Some(BlobClientDownloadOptions {
                encryption_algorithm: Some(algo),
                encryption_key: Some(key.clone()),
                encryption_key_sha256: Some(hash.clone()),
                ..Default::default()
            }))
            .await?;
        let body_data = response.body.collect().await?;
        assert_eq!(content, body_data);

        // Invalid Scope Upload Pages
        bad_scope_page_blob.create(512, None).await?;
        let err = bad_scope_page_blob
            .upload_pages(
                RequestContent::from(vec![b'B'; 512]),
                512,
                format_page_range(0, 512)?,
                Some(PageBlobClientUploadPagesOptions {
                    encryption_scope: Some(get_invalid_encryption_scope()),
                    ..Default::default()
                }),
            )
            .await;
        assert_bad_request_or_conflict(err.unwrap_err().http_status());

        // Clear Pages with CPK
        page_blob
            .clear_pages(
                format_page_range(0, 512)?,
                Some(PageBlobClientClearPagesOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(key.clone()),
                    encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;
        let response = blob_client
            .download(Some(BlobClientDownloadOptions {
                encryption_algorithm: Some(algo),
                encryption_key: Some(key.clone()),
                encryption_key_sha256: Some(hash.clone()),
                ..Default::default()
            }))
            .await?;
        let body_data = response.body.collect().await?;
        assert_eq!(vec![0; 512], body_data);

        // Invalid Scope Clear Pages
        let err = bad_scope_page_blob
            .clear_pages(
                format_page_range(0, 512)?,
                Some(PageBlobClientClearPagesOptions {
                    encryption_scope: Some(get_invalid_encryption_scope()),
                    ..Default::default()
                }),
            )
            .await;
        assert_bad_request_or_conflict(err.unwrap_err().http_status());

        // Resize with CPK
        page_blob
            .resize(
                1024,
                Some(PageBlobClientResizeOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(key.clone()),
                    encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;
        let props = blob_client
            .get_properties(Some(BlobClientGetPropertiesOptions {
                encryption_algorithm: Some(algo),
                encryption_key: Some(key.clone()),
                encryption_key_sha256: Some(hash.clone()),
                ..Default::default()
            }))
            .await?;
        assert_eq!(1024, props.content_length()?.unwrap());

        container_client.delete(None).await?;
        Ok(())
    }

    #[recorded::test]
    async fn test_upload_pages_from_url_cpk(ctx: TestContext) -> Result<(), Box<dyn Error>> {
        // Recording Setup
        let recording = ctx.recording();
        let container_client =
            get_container_client(recording, true, StorageAccount::Standard, None).await?;
        let (algo, key, hash) = get_cpk();
        let (_, wrong_key, wrong_hash) = get_cpk_2();

        // Setup CPK Source
        let source_blob =
            container_client.blob_client(&format!("{}-source", get_blob_name(recording)));
        let source_page_blob = source_blob.page_blob_client();
        source_page_blob
            .create(
                512,
                Some(PageBlobClientCreateOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(key.clone()),
                    encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;
        let source_content = vec![b'D'; 512];
        source_page_blob
            .upload_pages(
                RequestContent::from(source_content.clone()),
                512,
                format_page_range(0, 512)?,
                Some(PageBlobClientUploadPagesOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(key.clone()),
                    encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;

        // Setup CPK Destination
        let dest_blob = container_client.blob_client(&format!("{}-dest", get_blob_name(recording)));
        let dest_page_blob = dest_blob.page_blob_client();
        dest_page_blob
            .create(
                512,
                Some(PageBlobClientCreateOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(key.clone()),
                    encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;

        // Upload Pages from URL with CPK
        dest_page_blob
            .upload_pages_from_url(
                source_blob.url().as_str().into(),
                format_page_range(0, 512)?,
                512,
                format_page_range(0, 512)?,
                Some(PageBlobClientUploadPagesFromUrlOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(key.clone()),
                    encryption_key_sha256: Some(hash.clone()),
                    source_encryption_algorithm: Some(algo),
                    source_encryption_key: Some(key.clone()),
                    source_encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;
        let response = dest_blob
            .download(Some(BlobClientDownloadOptions {
                encryption_algorithm: Some(algo),
                encryption_key: Some(key.clone()),
                encryption_key_sha256: Some(hash.clone()),
                ..Default::default()
            }))
            .await?;
        let body_data = response.body.collect().await?;
        assert_eq!(source_content, body_data);

        // Source CPK Mismatch
        let source_mismatch_dest_blob = container_client
            .blob_client(&format!("{}-dest-src-mismatch", get_blob_name(recording)));
        let source_mismatch_dest_page_blob = source_mismatch_dest_blob.page_blob_client();
        source_mismatch_dest_page_blob.create(512, None).await?;
        let err = source_mismatch_dest_page_blob
            .upload_pages_from_url(
                source_blob.url().as_str().into(),
                format_page_range(0, 512)?,
                512,
                format_page_range(0, 512)?,
                Some(PageBlobClientUploadPagesFromUrlOptions {
                    source_encryption_algorithm: Some(algo),
                    source_encryption_key: Some(wrong_key.clone()),
                    source_encryption_key_sha256: Some(wrong_hash.clone()),
                    ..Default::default()
                }),
            )
            .await;
        assert_bad_request_or_conflict(err.unwrap_err().http_status());

        // Setup Unencrypted Source
        let plain_source_blob =
            container_client.blob_client(&format!("{}-plain-source", get_blob_name(recording)));
        let plain_source_page_blob = plain_source_blob.page_blob_client();
        plain_source_page_blob.create(512, None).await?;
        plain_source_page_blob
            .upload_pages(
                RequestContent::from(vec![b'T'; 512]),
                512,
                format_page_range(0, 512)?,
                None,
            )
            .await?;

        // Destination CPK Mismatch
        let dest_mismatch_blob = container_client
            .blob_client(&format!("{}-dest-dst-mismatch", get_blob_name(recording)));
        let dest_mismatch_page_blob = dest_mismatch_blob.page_blob_client();
        dest_mismatch_page_blob
            .create(
                512,
                Some(PageBlobClientCreateOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(key.clone()),
                    encryption_key_sha256: Some(hash.clone()),
                    ..Default::default()
                }),
            )
            .await?;
        let err = dest_mismatch_page_blob
            .upload_pages_from_url(
                plain_source_blob.url().as_str().into(),
                format_page_range(0, 512)?,
                512,
                format_page_range(0, 512)?,
                Some(PageBlobClientUploadPagesFromUrlOptions {
                    encryption_algorithm: Some(algo),
                    encryption_key: Some(wrong_key),
                    encryption_key_sha256: Some(wrong_hash),
                    ..Default::default()
                }),
            )
            .await;
        assert_bad_request_or_conflict(err.unwrap_err().http_status());

        // Invalid Scope Upload Pages from URL
        let bad_scope_dest_blob =
            container_client.blob_client(&format!("{}-dest-bad-scope", get_blob_name(recording)));
        let bad_scope_dest_page_blob = bad_scope_dest_blob.page_blob_client();
        bad_scope_dest_page_blob.create(512, None).await?;
        let err = bad_scope_dest_page_blob
            .upload_pages_from_url(
                plain_source_blob.url().as_str().into(),
                format_page_range(0, 512)?,
                512,
                format_page_range(0, 512)?,
                Some(PageBlobClientUploadPagesFromUrlOptions {
                    encryption_scope: Some(get_invalid_encryption_scope()),
                    ..Default::default()
                }),
            )
            .await;
        assert_bad_request_or_conflict(err.unwrap_err().http_status());

        container_client.delete(None).await?;
        Ok(())
    }
}
