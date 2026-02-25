// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::{RequestContent, StatusCode};
use azure_core_test::recorded;
use azure_core_test::TestContext;
use azure_storage_blob::models::{
    BlobClientCreateSnapshotOptions, BlobClientCreateSnapshotResultHeaders,
    BlobClientDownloadOptions, BlobClientGetPropertiesOptions,
    BlobClientGetPropertiesResultHeaders, BlobClientSetMetadataOptions,
    BlockBlobClientUploadOptions, EncryptionAlgorithmType,
};
use azure_storage_blob_test::{get_blob_name, get_container_client, StorageAccount};
use std::{collections::HashMap, error::Error};

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

fn invalid_key_sha256() -> String {
    // Valid base64, but intentionally not the hash of our test key.
    "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=".to_string()
}

fn assert_bad_request_or_conflict(status: Option<StatusCode>) {
    assert!(matches!(
        status,
        Some(StatusCode::BadRequest | StatusCode::Conflict)
    ));
}

#[recorded::test]
async fn test_upload_blob_partial_cpk_options_fail(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, _) = customer_provided_key();

    // Key Only Scenario
    let key_only_blob =
        container_client.blob_client(&format!("{}-cpk-key-only", get_blob_name(recording)));
    let key_only_options = BlockBlobClientUploadOptions {
        encryption_key: Some(encryption_key.clone()),
        ..Default::default()
    };
    let result = key_only_blob
        .upload(
            RequestContent::from(b"key-only".to_vec()),
            false,
            8,
            Some(key_only_options),
        )
        .await;

    // Assert
    assert!(result.is_err());

    // Key + Algorithm Without Hash Scenario
    let key_plus_algorithm_blob = container_client.blob_client(&format!(
        "{}-cpk-key-plus-algorithm",
        get_blob_name(recording)
    ));
    let key_plus_algorithm_options = BlockBlobClientUploadOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key),
        // Intentionally omit key hash.
        encryption_key_sha256: None,
        ..Default::default()
    };
    let result = key_plus_algorithm_blob
        .upload(
            RequestContent::from(b"key-plus-algorithm".to_vec()),
            false,
            18,
            Some(key_plus_algorithm_options),
        )
        .await;

    // Assert
    assert!(result.is_err());

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_upload_blob_encryption_options(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, encryption_key_sha256) = customer_provided_key();
    let blob_name = format!("{}-upload", get_blob_name(recording));
    let blob_client = container_client.blob_client(&blob_name);

    // CPK Upload Scenario
    let content = b"upload with cpk";
    let upload_options = BlockBlobClientUploadOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    blob_client
        .upload(
            RequestContent::from(content.to_vec()),
            false,
            u64::try_from(content.len())?,
            Some(upload_options),
        )
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
    let invalid_scope_blob =
        container_client.blob_client(&format!("{}-bad-scope", get_blob_name(recording)));
    let invalid_scope_options = BlockBlobClientUploadOptions {
        encryption_scope: Some(invalid_encryption_scope()),
        ..Default::default()
    };
    let result = invalid_scope_blob
        .upload(
            RequestContent::from(b"bad scope".to_vec()),
            false,
            9,
            Some(invalid_scope_options),
        )
        .await;

    // Assert
    let status = result.unwrap_err().http_status();
    assert_bad_request_or_conflict(status);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_download_blob_encryption_options(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, encryption_key_sha256) = customer_provided_key();
    let blob_client =
        container_client.blob_client(&format!("{}-download", get_blob_name(recording)));

    // Setup Scenario
    let content = b"download with cpk";
    let upload_options = BlockBlobClientUploadOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    blob_client
        .upload(
            RequestContent::from(content.to_vec()),
            false,
            u64::try_from(content.len())?,
            Some(upload_options),
        )
        .await?;

    // CPK Download Scenario
    let download_options = BlobClientDownloadOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    let response = blob_client.download(Some(download_options)).await?;
    let (status_code, _, body) = response.deconstruct();

    // Assert
    assert!(status_code.is_success());
    assert_eq!(content.to_vec(), body.collect().await?.to_vec());

    // Invalid CPK Hash Scenario
    let invalid_download_options = BlobClientDownloadOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key),
        encryption_key_sha256: Some(invalid_key_sha256()),
        ..Default::default()
    };
    let error = blob_client.download(Some(invalid_download_options)).await;

    // Assert
    let status = error.unwrap_err().http_status();
    assert_bad_request_or_conflict(status);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_get_blob_properties_encryption_options(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, encryption_key_sha256) = customer_provided_key();
    let blob_client =
        container_client.blob_client(&format!("{}-properties", get_blob_name(recording)));

    // Setup Scenario
    let content = b"properties with cpk";
    let upload_options = BlockBlobClientUploadOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    blob_client
        .upload(
            RequestContent::from(content.to_vec()),
            false,
            u64::try_from(content.len())?,
            Some(upload_options),
        )
        .await?;

    // CPK Get Properties Scenario
    let get_options = BlobClientGetPropertiesOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256),
        ..Default::default()
    };
    let response = blob_client.get_properties(Some(get_options)).await?;

    // Assert
    assert_eq!(
        u64::try_from(content.len())?,
        response.content_length()?.unwrap()
    );

    // Invalid CPK Hash Scenario
    let invalid_get_options = BlobClientGetPropertiesOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key),
        encryption_key_sha256: Some(invalid_key_sha256()),
        ..Default::default()
    };
    let error = blob_client.get_properties(Some(invalid_get_options)).await;

    // Assert
    let status = error.unwrap_err().http_status();
    assert_bad_request_or_conflict(status);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_set_blob_metadata_encryption_options(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, encryption_key_sha256) = customer_provided_key();
    let blob_client =
        container_client.blob_client(&format!("{}-metadata", get_blob_name(recording)));

    // Setup Scenario
    let content = b"metadata with cpk";
    let upload_options = BlockBlobClientUploadOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    blob_client
        .upload(
            RequestContent::from(content.to_vec()),
            false,
            u64::try_from(content.len())?,
            Some(upload_options),
        )
        .await?;

    // CPK Set Metadata Scenario
    let metadata = HashMap::from([("test_key".to_string(), "test-value".to_string())]);
    let metadata_options = BlobClientSetMetadataOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    blob_client
        .set_metadata(&metadata, Some(metadata_options))
        .await?;

    // Assert
    let get_options = BlobClientGetPropertiesOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key),
        encryption_key_sha256: Some(encryption_key_sha256),
        ..Default::default()
    };
    let response = blob_client.get_properties(Some(get_options)).await?;
    assert_eq!(metadata, response.metadata()?);

    // Invalid Encryption Scope Scenario
    let invalid_scope_metadata = BlobClientSetMetadataOptions {
        encryption_scope: Some(invalid_encryption_scope()),
        ..Default::default()
    };
    let result = blob_client
        .set_metadata(&metadata, Some(invalid_scope_metadata))
        .await;

    // Assert
    let status = result.unwrap_err().http_status();
    assert_bad_request_or_conflict(status);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_create_blob_snapshot_encryption_options(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, encryption_key_sha256) = customer_provided_key();
    let blob_client =
        container_client.blob_client(&format!("{}-snapshot", get_blob_name(recording)));

    // Setup Scenario
    let content = b"snapshot with cpk";
    let upload_options = BlockBlobClientUploadOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    blob_client
        .upload(
            RequestContent::from(content.to_vec()),
            false,
            u64::try_from(content.len())?,
            Some(upload_options),
        )
        .await?;

    // CPK Create Snapshot Scenario
    let snapshot_options = BlobClientCreateSnapshotOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    let snapshot_response = blob_client.create_snapshot(Some(snapshot_options)).await?;
    let snapshot_id = snapshot_response.snapshot()?.unwrap();

    // Assert
    let snapshot_client = blob_client.with_snapshot(&snapshot_id)?;
    let download_options = BlobClientDownloadOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key),
        encryption_key_sha256: Some(encryption_key_sha256),
        ..Default::default()
    };
    let response = snapshot_client.download(Some(download_options)).await?;
    let (status_code, _, body) = response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(content.to_vec(), body.collect().await?.to_vec());

    // Invalid Encryption Scope Scenario
    let invalid_scope_snapshot = BlobClientCreateSnapshotOptions {
        encryption_scope: Some(invalid_encryption_scope()),
        ..Default::default()
    };
    let result = blob_client
        .create_snapshot(Some(invalid_scope_snapshot))
        .await;

    // Assert
    let status = result.unwrap_err().http_status();
    assert_bad_request_or_conflict(status);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_download_blob_snapshot_with_cpk_options(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, encryption_key_sha256) = customer_provided_key();
    let blob_client =
        container_client.blob_client(&format!("{}-snapshot-cpk", get_blob_name(recording)));

    // Create Version 1 and Snapshot Scenario
    let v1 = b"snapshot-version-1";
    let upload_v1_options = BlockBlobClientUploadOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    blob_client
        .upload(
            RequestContent::from(v1.to_vec()),
            false,
            u64::try_from(v1.len())?,
            Some(upload_v1_options),
        )
        .await?;

    let snapshot_response = blob_client
        .create_snapshot(Some(BlobClientCreateSnapshotOptions {
            encryption_algorithm: Some(encryption_algorithm),
            encryption_key: Some(encryption_key.clone()),
            encryption_key_sha256: Some(encryption_key_sha256.clone()),
            ..Default::default()
        }))
        .await?;
    let snapshot_id = snapshot_response.snapshot()?.unwrap();

    // Overwrite Current Blob Scenario
    let v2 = b"snapshot-version-2";
    let upload_v2_options = BlockBlobClientUploadOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    blob_client
        .upload(
            RequestContent::from(v2.to_vec()),
            true,
            u64::try_from(v2.len())?,
            Some(upload_v2_options),
        )
        .await?;

    // Snapshot Download with CPK + Options Bag Scenario
    let snapshot_download = blob_client
        .download(Some(BlobClientDownloadOptions {
            snapshot: Some(snapshot_id),
            encryption_algorithm: Some(encryption_algorithm),
            encryption_key: Some(encryption_key),
            encryption_key_sha256: Some(encryption_key_sha256),
            ..Default::default()
        }))
        .await?;
    let (_, _, body) = snapshot_download.deconstruct();

    // Assert
    assert_eq!(v1.to_vec(), body.collect().await?.to_vec());

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test(playback)]
async fn test_upload_blob_encryption_scope(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let scope = "testscope".to_string();

    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&format!("{}-scope", get_blob_name(recording)));

    // Encryption Scope Upload Scenario
    let content = b"scope-success";
    let upload_options = BlockBlobClientUploadOptions {
        encryption_scope: Some(scope.clone()),
        ..Default::default()
    };
    blob_client
        .upload(
            RequestContent::from(content.to_vec()),
            false,
            u64::try_from(content.len())?,
            Some(upload_options),
        )
        .await?;

    // Assert
    let response = blob_client.get_properties(None).await?;
    if let Some(response_scope) = response.encryption_scope()? {
        assert_eq!(scope, response_scope);
    }

    container_client.delete(None).await?;
    Ok(())
}
