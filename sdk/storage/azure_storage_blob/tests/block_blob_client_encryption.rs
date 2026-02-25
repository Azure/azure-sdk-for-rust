// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::{RequestContent, StatusCode};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::models::{
    BlobClientDownloadOptions, BlockBlobClientCommitBlockListOptions,
    BlockBlobClientStageBlockFromUrlOptions, BlockBlobClientStageBlockOptions,
    BlockBlobClientUploadBlobFromUrlOptions, BlockBlobClientUploadOptions, BlockLookupList,
    EncryptionAlgorithmType,
};
use azure_storage_blob_test::{
    create_test_blob, get_blob_name, get_container_client, StorageAccount,
};
use std::error::Error;

fn customer_provided_key() -> (EncryptionAlgorithmType, String, String) {
    (
        EncryptionAlgorithmType::Aes256,
        "AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8=".to_string(),
        "Yw3NKWbEM2aRElRIu7JbT/QSpJxzLbLIq8G4WBvXEN0=".to_string(),
    )
}

fn invalid_encryption_scope() -> String {
    "invalid-encryption-scope-for-tests".to_string()
}

fn assert_bad_request_or_conflict(status: Option<StatusCode>) {
    assert!(matches!(
        status,
        Some(StatusCode::BadRequest | StatusCode::Conflict)
    ));
}

fn block_lookup(block_id: Vec<u8>) -> BlockLookupList {
    BlockLookupList {
        committed: Some(Vec::new()),
        latest: Some(vec![block_id]),
        uncommitted: Some(Vec::new()),
    }
}

#[recorded::test]
async fn test_stage_block_partial_cpk_options_fail(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, _) = customer_provided_key();
    let blob_client =
        container_client.blob_client(&format!("{}-cpk-partial", get_blob_name(recording)));
    let block_blob_client = blob_client.block_blob_client();

    // Key Only Scenario
    let key_only_options = BlockBlobClientStageBlockOptions {
        encryption_key: Some(encryption_key.clone()),
        ..Default::default()
    };
    let result = block_blob_client
        .stage_block(
            b"key-only",
            8,
            RequestContent::from(b"key-only".to_vec()),
            Some(key_only_options),
        )
        .await;

    // Assert
    assert!(result.is_err());

    // Key + Algorithm Without Hash Scenario
    let key_plus_algorithm_options = BlockBlobClientStageBlockOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key),
        // Intentionally omit key hash.
        encryption_key_sha256: None,
        ..Default::default()
    };
    let result = block_blob_client
        .stage_block(
            b"key-plus-algorithm",
            18,
            RequestContent::from(b"key-plus-algorithm".to_vec()),
            Some(key_plus_algorithm_options),
        )
        .await;

    // Assert
    assert!(result.is_err());

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_stage_block_encryption_options(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, encryption_key_sha256) = customer_provided_key();
    let blob_client = container_client.blob_client(&format!("{}-stage", get_blob_name(recording)));
    let block_blob_client = blob_client.block_blob_client();
    let block_id = b"block-1".to_vec();
    let content = b"stage block encrypted";

    // CPK Stage Block Scenario
    let stage_options = BlockBlobClientStageBlockOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    block_blob_client
        .stage_block(
            &block_id,
            u64::try_from(content.len())?,
            RequestContent::from(content.to_vec()),
            Some(stage_options),
        )
        .await?;

    let commit_options = BlockBlobClientCommitBlockListOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    block_blob_client
        .commit_block_list(block_lookup(block_id).try_into()?, Some(commit_options))
        .await?;

    // Assert
    let download_options = BlobClientDownloadOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key),
        encryption_key_sha256: Some(encryption_key_sha256),
        ..Default::default()
    };
    let response = blob_client.download(Some(download_options)).await?;
    let (_, _, body) = response.deconstruct();
    assert_eq!(content.to_vec(), body.collect().await?.to_vec());

    // Invalid Encryption Scope Scenario
    let invalid_blob_client =
        container_client.blob_client(&format!("{}-stage-bad-scope", get_blob_name(recording)));
    let invalid_block_blob_client = invalid_blob_client.block_blob_client();
    let invalid_options = BlockBlobClientStageBlockOptions {
        encryption_scope: Some(invalid_encryption_scope()),
        ..Default::default()
    };
    let result = invalid_block_blob_client
        .stage_block(
            b"block-2",
            9,
            RequestContent::from(b"bad-scope".to_vec()),
            Some(invalid_options),
        )
        .await;

    // Assert
    let status = result.unwrap_err().http_status();
    assert_bad_request_or_conflict(status);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_commit_block_list_encryption_options(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, encryption_key_sha256) = customer_provided_key();
    let blob_client = container_client.blob_client(&format!("{}-commit", get_blob_name(recording)));
    let block_blob_client = blob_client.block_blob_client();
    let block_id = b"block-1".to_vec();
    let content = b"commit encrypted";

    // Setup Scenario
    let stage_options = BlockBlobClientStageBlockOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    block_blob_client
        .stage_block(
            &block_id,
            u64::try_from(content.len())?,
            RequestContent::from(content.to_vec()),
            Some(stage_options),
        )
        .await?;

    // CPK Commit Block List Scenario
    let commit_options = BlockBlobClientCommitBlockListOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    block_blob_client
        .commit_block_list(block_lookup(block_id).try_into()?, Some(commit_options))
        .await?;

    // Assert
    let download_options = BlobClientDownloadOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key),
        encryption_key_sha256: Some(encryption_key_sha256),
        ..Default::default()
    };
    let response = blob_client.download(Some(download_options)).await?;
    let (_, _, body) = response.deconstruct();
    assert_eq!(content.to_vec(), body.collect().await?.to_vec());

    // Invalid Encryption Scope Scenario
    let invalid_blob_client =
        container_client.blob_client(&format!("{}-commit-bad-scope", get_blob_name(recording)));
    let invalid_block_blob_client = invalid_blob_client.block_blob_client();
    let invalid_block_id = b"block-2".to_vec();
    invalid_block_blob_client
        .stage_block(
            &invalid_block_id,
            9,
            RequestContent::from(b"bad-scope".to_vec()),
            None,
        )
        .await?;
    let invalid_commit_options = BlockBlobClientCommitBlockListOptions {
        encryption_scope: Some(invalid_encryption_scope()),
        ..Default::default()
    };
    let result = invalid_block_blob_client
        .commit_block_list(
            block_lookup(invalid_block_id).try_into()?,
            Some(invalid_commit_options),
        )
        .await;

    // Assert
    let status = result.unwrap_err().http_status();
    assert_bad_request_or_conflict(status);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_upload_blob_from_url_encryption_options(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, encryption_key_sha256) = customer_provided_key();

    let source_blob_client =
        container_client.blob_client(&format!("{}-source-upload-url", get_blob_name(recording)));
    let source_content = b"source content upload from url";
    create_test_blob(
        &source_blob_client,
        Some(RequestContent::from(source_content.to_vec())),
        None,
    )
    .await?;

    let dest_blob_client =
        container_client.blob_client(&format!("{}-dest-upload-url", get_blob_name(recording)));

    // Destination CPK Upload Blob From URL Scenario
    let options = BlockBlobClientUploadBlobFromUrlOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    dest_blob_client
        .block_blob_client()
        .upload_blob_from_url(source_blob_client.url().as_str().into(), Some(options))
        .await?;

    // Assert
    let download_options = BlobClientDownloadOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key),
        encryption_key_sha256: Some(encryption_key_sha256),
        ..Default::default()
    };
    let response = dest_blob_client.download(Some(download_options)).await?;
    let (_, _, body) = response.deconstruct();
    assert_eq!(source_content.to_vec(), body.collect().await?.to_vec());

    // Invalid Encryption Scope Scenario
    let invalid_dest_blob_client = container_client.blob_client(&format!(
        "{}-dest-upload-url-bad-scope",
        get_blob_name(recording)
    ));
    let invalid_options = BlockBlobClientUploadBlobFromUrlOptions {
        encryption_scope: Some(invalid_encryption_scope()),
        ..Default::default()
    };
    let result = invalid_dest_blob_client
        .block_blob_client()
        .upload_blob_from_url(
            source_blob_client.url().as_str().into(),
            Some(invalid_options),
        )
        .await;

    // Assert
    let status = result.unwrap_err().http_status();
    assert_bad_request_or_conflict(status);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_stage_block_from_url_encryption_options(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, encryption_key_sha256) = customer_provided_key();

    let source_blob_client =
        container_client.blob_client(&format!("{}-source-stage-url", get_blob_name(recording)));
    let source_content = b"source content stage from url";
    create_test_blob(
        &source_blob_client,
        Some(RequestContent::from(source_content.to_vec())),
        None,
    )
    .await?;

    let dest_blob_client =
        container_client.blob_client(&format!("{}-dest-stage-url", get_blob_name(recording)));
    let dest_block_blob_client = dest_blob_client.block_blob_client();
    let block_id = b"block-1".to_vec();

    // Destination CPK Stage Block From URL Scenario
    let stage_options = BlockBlobClientStageBlockFromUrlOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    dest_block_blob_client
        .stage_block_from_url(
            &block_id,
            u64::try_from(source_content.len())?,
            source_blob_client.url().as_str().into(),
            Some(stage_options),
        )
        .await?;

    let commit_options = BlockBlobClientCommitBlockListOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    dest_block_blob_client
        .commit_block_list(block_lookup(block_id).try_into()?, Some(commit_options))
        .await?;

    // Assert
    let download_options = BlobClientDownloadOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key),
        encryption_key_sha256: Some(encryption_key_sha256),
        ..Default::default()
    };
    let response = dest_blob_client.download(Some(download_options)).await?;
    let (_, _, body) = response.deconstruct();
    assert_eq!(source_content.to_vec(), body.collect().await?.to_vec());

    // Invalid Encryption Scope Scenario
    let invalid_dest_blob_client = container_client.blob_client(&format!(
        "{}-dest-stage-url-bad-scope",
        get_blob_name(recording)
    ));
    let invalid_block_blob_client = invalid_dest_blob_client.block_blob_client();
    let invalid_options = BlockBlobClientStageBlockFromUrlOptions {
        encryption_scope: Some(invalid_encryption_scope()),
        ..Default::default()
    };
    let result = invalid_block_blob_client
        .stage_block_from_url(
            b"block-2",
            u64::try_from(source_content.len())?,
            source_blob_client.url().as_str().into(),
            Some(invalid_options),
        )
        .await;

    // Assert
    let status = result.unwrap_err().http_status();
    assert_bad_request_or_conflict(status);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_stage_block_from_url_source_encryption_options(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, encryption_key_sha256) = customer_provided_key();

    // Source CPK Blob Scenario
    let source_blob_client =
        container_client.blob_client(&format!("{}-source-cpk", get_blob_name(recording)));
    let source_content = b"source encrypted with cpk";
    let source_upload_options = BlockBlobClientUploadOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    source_blob_client
        .upload(
            RequestContent::from(source_content.to_vec()),
            false,
            u64::try_from(source_content.len())?,
            Some(source_upload_options),
        )
        .await?;

    let dest_blob_client =
        container_client.blob_client(&format!("{}-dest-source-cpk", get_blob_name(recording)));
    let dest_block_blob_client = dest_blob_client.block_blob_client();
    let block_id = b"block-1".to_vec();

    // Source Encryption Options Scenario
    let stage_options = BlockBlobClientStageBlockFromUrlOptions {
        source_encryption_algorithm: Some(encryption_algorithm),
        source_encryption_key: Some(encryption_key.clone()),
        source_encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    dest_block_blob_client
        .stage_block_from_url(
            &block_id,
            u64::try_from(source_content.len())?,
            source_blob_client.url().as_str().into(),
            Some(stage_options),
        )
        .await?;

    dest_block_blob_client
        .commit_block_list(block_lookup(block_id).try_into()?, None)
        .await?;

    // Assert
    let response = dest_blob_client.download(None).await?;
    let (_, _, body) = response.deconstruct();
    assert_eq!(source_content.to_vec(), body.collect().await?.to_vec());

    container_client.delete(None).await?;
    Ok(())
}
