// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::RequestContent;
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::models::{
    AppendBlobClientAppendBlockFromUrlOptions, AppendBlobClientAppendBlockOptions,
    AppendBlobClientCreateOptions, BlobClientDownloadOptions, BlobClientGetPropertiesOptions,
    BlobClientGetPropertiesResultHeaders, BlobType,
};
use azure_storage_blob_test::{
    assert_bad_request_or_conflict, create_test_blob, get_blob_name, get_container_client, get_cpk,
    get_cpk_2, get_invalid_encryption_scope, StorageAccount,
};
use std::error::Error;

#[recorded::test]
async fn test_append_blob_partial_cpk_options_fail(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, _) = get_cpk();

    // Key Only Create Scenario
    let key_only_blob =
        container_client.blob_client(&format!("{}-cpk-key-only", get_blob_name(recording)));
    let key_only_create_options = AppendBlobClientCreateOptions {
        encryption_key: Some(encryption_key.clone()),
        ..Default::default()
    };
    let result = key_only_blob
        .append_blob_client()
        .create(Some(key_only_create_options))
        .await;

    // Assert
    assert!(result.is_err());

    // Key + Algorithm Without Hash Append Block Scenario
    let key_plus_algorithm_blob = container_client.blob_client(&format!(
        "{}-cpk-key-plus-algorithm",
        get_blob_name(recording)
    ));
    key_plus_algorithm_blob
        .append_blob_client()
        .create(None)
        .await?;
    let key_plus_algorithm_options = AppendBlobClientAppendBlockOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key),
        // Intentionally omit key hash.
        encryption_key_sha256: None,
        ..Default::default()
    };
    let result = key_plus_algorithm_blob
        .append_blob_client()
        .append_block(
            RequestContent::from(b"key-plus-algorithm".to_vec()),
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
async fn test_create_append_blob_encryption_options(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, encryption_key_sha256) = get_cpk();
    let blob_client = container_client.blob_client(&format!("{}-create", get_blob_name(recording)));
    let append_blob_client = blob_client.append_blob_client();

    // CPK Create Scenario
    let create_options = AppendBlobClientCreateOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    append_blob_client.create(Some(create_options)).await?;

    // Assert
    let get_properties_options = BlobClientGetPropertiesOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key),
        encryption_key_sha256: Some(encryption_key_sha256),
        ..Default::default()
    };
    let response = blob_client
        .get_properties(Some(get_properties_options))
        .await?;
    let blob_type = response.blob_type()?;
    assert_eq!(BlobType::AppendBlob, blob_type.unwrap());

    // Invalid Encryption Scope Scenario
    let invalid_blob_client =
        container_client.blob_client(&format!("{}-create-bad-scope", get_blob_name(recording)));
    let invalid_append_blob_client = invalid_blob_client.append_blob_client();
    let invalid_options = AppendBlobClientCreateOptions {
        encryption_scope: Some(get_invalid_encryption_scope()),
        ..Default::default()
    };
    let result = invalid_append_blob_client
        .create(Some(invalid_options))
        .await;

    // Assert
    let status = result.unwrap_err().http_status();
    assert_bad_request_or_conflict(status);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_append_block_encryption_options(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, encryption_key_sha256) = get_cpk();
    let blob_client = container_client.blob_client(&format!("{}-append", get_blob_name(recording)));
    let append_blob_client = blob_client.append_blob_client();

    // Setup Scenario
    let create_options = AppendBlobClientCreateOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    append_blob_client.create(Some(create_options)).await?;

    // CPK Append Block Scenario
    let content = b"append with cpk";
    let append_options = AppendBlobClientAppendBlockOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    append_blob_client
        .append_block(
            RequestContent::from(content.to_vec()),
            u64::try_from(content.len())?,
            Some(append_options),
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
    let invalid_blob_client =
        container_client.blob_client(&format!("{}-append-bad-scope", get_blob_name(recording)));
    let invalid_append_blob_client = invalid_blob_client.append_blob_client();
    invalid_append_blob_client.create(None).await?;
    let invalid_options = AppendBlobClientAppendBlockOptions {
        encryption_scope: Some(get_invalid_encryption_scope()),
        ..Default::default()
    };
    let result = invalid_append_blob_client
        .append_block(
            RequestContent::from(b"bad-scope".to_vec()),
            9,
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
async fn test_append_block_from_url_encryption_options(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, encryption_key_sha256) = get_cpk();

    let source_blob_client =
        container_client.blob_client(&format!("{}-source-url", get_blob_name(recording)));
    let source_content = b"append from url content";
    create_test_blob(
        &source_blob_client,
        Some(RequestContent::from(source_content.to_vec())),
        None,
    )
    .await?;

    let dest_blob_client =
        container_client.blob_client(&format!("{}-dest-url", get_blob_name(recording)));
    let append_blob_client = dest_blob_client.append_blob_client();
    let dest_create_options = AppendBlobClientCreateOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    append_blob_client.create(Some(dest_create_options)).await?;

    // Destination CPK Append Block From URL Scenario
    let append_from_url_options = AppendBlobClientAppendBlockFromUrlOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    append_blob_client
        .append_block_from_url(
            source_blob_client.url().as_str().into(),
            u64::try_from(source_content.len())?,
            Some(append_from_url_options),
        )
        .await?;

    // Assert
    let download_options = BlobClientDownloadOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    let response = dest_blob_client.download(Some(download_options)).await?;
    let (_, _, body) = response.deconstruct();
    assert_eq!(source_content.to_vec(), body.collect().await?.to_vec());

    // Source Encryption Options Scenario
    let source_cpk_blob_client =
        container_client.blob_client(&format!("{}-source-url-cpk", get_blob_name(recording)));
    let source_cpk_content = b"source cpk for append from url";
    source_cpk_blob_client
        .upload(
            RequestContent::from(source_cpk_content.to_vec()),
            false,
            u64::try_from(source_cpk_content.len())?,
            Some(azure_storage_blob::models::BlockBlobClientUploadOptions {
                encryption_algorithm: Some(encryption_algorithm),
                encryption_key: Some(encryption_key.clone()),
                encryption_key_sha256: Some(encryption_key_sha256.clone()),
                ..Default::default()
            }),
        )
        .await?;

    let source_options = AppendBlobClientAppendBlockFromUrlOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        source_encryption_algorithm: Some(encryption_algorithm),
        source_encryption_key: Some(encryption_key.clone()),
        source_encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    append_blob_client
        .append_block_from_url(
            source_cpk_blob_client.url().as_str().into(),
            u64::try_from(source_cpk_content.len())?,
            Some(source_options),
        )
        .await?;

    // Assert
    let response = dest_blob_client
        .download(Some(BlobClientDownloadOptions {
            encryption_algorithm: Some(encryption_algorithm),
            encryption_key: Some(encryption_key.clone()),
            encryption_key_sha256: Some(encryption_key_sha256.clone()),
            ..Default::default()
        }))
        .await?;
    let (_, _, body) = response.deconstruct();
    let mut expected = source_content.to_vec();
    expected.extend(source_cpk_content);
    assert_eq!(expected, body.collect().await?.to_vec());

    // Invalid Encryption Scope Scenario
    let invalid_dest_blob_client =
        container_client.blob_client(&format!("{}-dest-url-bad-scope", get_blob_name(recording)));
    let invalid_append_blob_client = invalid_dest_blob_client.append_blob_client();
    invalid_append_blob_client.create(None).await?;
    let invalid_options = AppendBlobClientAppendBlockFromUrlOptions {
        encryption_scope: Some(get_invalid_encryption_scope()),
        ..Default::default()
    };
    let result = invalid_append_blob_client
        .append_block_from_url(
            source_blob_client.url().as_str().into(),
            u64::try_from(source_content.len())?,
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
async fn test_append_block_from_url_source_cpk_mismatch_fails(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, encryption_key_sha256) = get_cpk();
    let (_, wrong_key, wrong_key_sha256) = get_cpk_2();

    // Source CPK Blob Scenario
    let source_blob_client =
        container_client.blob_client(&format!("{}-source-cpk-mismatch", get_blob_name(recording)));
    let source_content = b"source encrypted with cpk";
    source_blob_client
        .upload(
            RequestContent::from(source_content.to_vec()),
            false,
            u64::try_from(source_content.len())?,
            Some(azure_storage_blob::models::BlockBlobClientUploadOptions {
                encryption_algorithm: Some(encryption_algorithm),
                encryption_key: Some(encryption_key),
                encryption_key_sha256: Some(encryption_key_sha256),
                ..Default::default()
            }),
        )
        .await?;

    let dest_blob_client =
        container_client.blob_client(&format!("{}-dest-cpk-mismatch", get_blob_name(recording)));
    let append_blob_client = dest_blob_client.append_blob_client();
    append_blob_client.create(None).await?;

    // Source CPK Mismatch Scenario
    let options = AppendBlobClientAppendBlockFromUrlOptions {
        source_encryption_algorithm: Some(encryption_algorithm),
        source_encryption_key: Some(wrong_key),
        source_encryption_key_sha256: Some(wrong_key_sha256),
        ..Default::default()
    };
    let result = append_blob_client
        .append_block_from_url(
            source_blob_client.url().as_str().into(),
            u64::try_from(source_content.len())?,
            Some(options),
        )
        .await;

    // Assert
    let status = result.unwrap_err().http_status();
    assert_bad_request_or_conflict(status);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_append_block_from_url_destination_cpk_mismatch_fails(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, encryption_key_sha256) = get_cpk();
    let (_, wrong_key, wrong_key_sha256) = get_cpk_2();

    let source_blob_client = container_client.blob_client(&format!(
        "{}-source-dest-mismatch",
        get_blob_name(recording)
    ));
    let source_content = b"plain source content";
    create_test_blob(
        &source_blob_client,
        Some(RequestContent::from(source_content.to_vec())),
        None,
    )
    .await?;

    let dest_blob_client =
        container_client.blob_client(&format!("{}-dest-dest-mismatch", get_blob_name(recording)));
    let append_blob_client = dest_blob_client.append_blob_client();
    append_blob_client
        .create(Some(AppendBlobClientCreateOptions {
            encryption_algorithm: Some(encryption_algorithm),
            encryption_key: Some(encryption_key),
            encryption_key_sha256: Some(encryption_key_sha256),
            ..Default::default()
        }))
        .await?;

    // Destination CPK Mismatch Scenario
    let options = AppendBlobClientAppendBlockFromUrlOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(wrong_key),
        encryption_key_sha256: Some(wrong_key_sha256),
        ..Default::default()
    };
    let result = append_blob_client
        .append_block_from_url(
            source_blob_client.url().as_str().into(),
            u64::try_from(source_content.len())?,
            Some(options),
        )
        .await;

    // Assert
    let status = result.unwrap_err().http_status();
    assert_bad_request_or_conflict(status);

    container_client.delete(None).await?;
    Ok(())
}
