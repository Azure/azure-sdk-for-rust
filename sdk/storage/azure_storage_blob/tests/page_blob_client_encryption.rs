// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::{RequestContent, StatusCode};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::{
    format_page_range,
    models::{
        BlobClientDownloadOptions, BlobClientGetPropertiesOptions,
        BlobClientGetPropertiesResultHeaders, EncryptionAlgorithmType,
        PageBlobClientClearPagesOptions, PageBlobClientCreateOptions, PageBlobClientResizeOptions,
        PageBlobClientUploadPagesFromUrlOptions, PageBlobClientUploadPagesOptions,
    },
};
use azure_storage_blob_test::{get_blob_name, get_container_client, StorageAccount};
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

#[recorded::test]
async fn test_page_blob_partial_cpk_options_fail(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, _) = customer_provided_key();

    // Key Only Create Scenario
    let key_only_blob =
        container_client.blob_client(&format!("{}-cpk-key-only", get_blob_name(recording)));
    let key_only_create_options = PageBlobClientCreateOptions {
        encryption_key: Some(encryption_key.clone()),
        ..Default::default()
    };
    let result = key_only_blob
        .page_blob_client()
        .create(512, Some(key_only_create_options))
        .await;

    // Assert
    assert!(result.is_err());

    // Key + Algorithm Without Hash Upload Scenario
    let key_plus_algorithm_blob = container_client.blob_client(&format!(
        "{}-cpk-key-plus-algorithm",
        get_blob_name(recording)
    ));
    let key_plus_algorithm_page_blob_client = key_plus_algorithm_blob.page_blob_client();
    key_plus_algorithm_page_blob_client
        .create(512, None)
        .await?;
    let key_plus_algorithm_upload_options = PageBlobClientUploadPagesOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key),
        // Intentionally omit key hash.
        encryption_key_sha256: None,
        ..Default::default()
    };
    let result = key_plus_algorithm_page_blob_client
        .upload_pages(
            RequestContent::from(vec![b'P'; 512]),
            512,
            format_page_range(0, 512)?,
            Some(key_plus_algorithm_upload_options),
        )
        .await;

    // Assert
    assert!(result.is_err());

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_create_page_blob_encryption_options(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, encryption_key_sha256) = customer_provided_key();
    let blob_client = container_client.blob_client(&format!("{}-create", get_blob_name(recording)));
    let page_blob_client = blob_client.page_blob_client();

    // CPK Create Scenario
    let create_options = PageBlobClientCreateOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    page_blob_client.create(512, Some(create_options)).await?;

    // Assert
    let get_options = BlobClientGetPropertiesOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key),
        encryption_key_sha256: Some(encryption_key_sha256),
        ..Default::default()
    };
    let response = blob_client.get_properties(Some(get_options)).await?;
    assert_eq!(512, response.content_length()?.unwrap());

    // Invalid Encryption Scope Scenario
    let invalid_blob_client =
        container_client.blob_client(&format!("{}-create-bad-scope", get_blob_name(recording)));
    let invalid_page_blob_client = invalid_blob_client.page_blob_client();
    let invalid_options = PageBlobClientCreateOptions {
        encryption_scope: Some(invalid_encryption_scope()),
        ..Default::default()
    };
    let result = invalid_page_blob_client
        .create(512, Some(invalid_options))
        .await;

    // Assert
    let status = result.unwrap_err().http_status();
    assert_bad_request_or_conflict(status);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_upload_pages_encryption_options(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, encryption_key_sha256) = customer_provided_key();
    let blob_client = container_client.blob_client(&format!("{}-upload", get_blob_name(recording)));
    let page_blob_client = blob_client.page_blob_client();

    // Setup Scenario
    let create_options = PageBlobClientCreateOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    page_blob_client.create(512, Some(create_options)).await?;

    // CPK Upload Pages Scenario
    let content = vec![b'A'; 512];
    let upload_options = PageBlobClientUploadPagesOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    page_blob_client
        .upload_pages(
            RequestContent::from(content.clone()),
            512,
            format_page_range(0, 512)?,
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
    assert_eq!(content, body.collect().await?.to_vec());

    // Invalid Encryption Scope Scenario
    let invalid_blob_client =
        container_client.blob_client(&format!("{}-upload-bad-scope", get_blob_name(recording)));
    let invalid_page_blob_client = invalid_blob_client.page_blob_client();
    invalid_page_blob_client.create(512, None).await?;
    let invalid_options = PageBlobClientUploadPagesOptions {
        encryption_scope: Some(invalid_encryption_scope()),
        ..Default::default()
    };
    let result = invalid_page_blob_client
        .upload_pages(
            RequestContent::from(vec![b'B'; 512]),
            512,
            format_page_range(0, 512)?,
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
async fn test_clear_pages_encryption_options(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, encryption_key_sha256) = customer_provided_key();
    let blob_client = container_client.blob_client(&format!("{}-clear", get_blob_name(recording)));
    let page_blob_client = blob_client.page_blob_client();

    // Setup Scenario
    page_blob_client
        .create(
            512,
            Some(PageBlobClientCreateOptions {
                encryption_algorithm: Some(encryption_algorithm),
                encryption_key: Some(encryption_key.clone()),
                encryption_key_sha256: Some(encryption_key_sha256.clone()),
                ..Default::default()
            }),
        )
        .await?;
    page_blob_client
        .upload_pages(
            RequestContent::from(vec![b'C'; 512]),
            512,
            format_page_range(0, 512)?,
            Some(PageBlobClientUploadPagesOptions {
                encryption_algorithm: Some(encryption_algorithm),
                encryption_key: Some(encryption_key.clone()),
                encryption_key_sha256: Some(encryption_key_sha256.clone()),
                ..Default::default()
            }),
        )
        .await?;

    // CPK Clear Pages Scenario
    let clear_options = PageBlobClientClearPagesOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    page_blob_client
        .clear_pages(format_page_range(0, 512)?, Some(clear_options))
        .await?;

    // Assert
    let response = blob_client
        .download(Some(BlobClientDownloadOptions {
            encryption_algorithm: Some(encryption_algorithm),
            encryption_key: Some(encryption_key),
            encryption_key_sha256: Some(encryption_key_sha256),
            ..Default::default()
        }))
        .await?;
    let (_, _, body) = response.deconstruct();
    assert_eq!(vec![0; 512], body.collect().await?.to_vec());

    // Invalid Encryption Scope Scenario
    let invalid_blob_client =
        container_client.blob_client(&format!("{}-clear-bad-scope", get_blob_name(recording)));
    let invalid_page_blob_client = invalid_blob_client.page_blob_client();
    invalid_page_blob_client.create(512, None).await?;
    let invalid_options = PageBlobClientClearPagesOptions {
        encryption_scope: Some(invalid_encryption_scope()),
        ..Default::default()
    };
    let result = invalid_page_blob_client
        .clear_pages(format_page_range(0, 512)?, Some(invalid_options))
        .await;

    // Assert
    let status = result.unwrap_err().http_status();
    assert_bad_request_or_conflict(status);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_resize_page_blob_encryption_options(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, encryption_key_sha256) = customer_provided_key();
    let blob_client = container_client.blob_client(&format!("{}-resize", get_blob_name(recording)));
    let page_blob_client = blob_client.page_blob_client();

    // Setup Scenario
    page_blob_client
        .create(
            512,
            Some(PageBlobClientCreateOptions {
                encryption_algorithm: Some(encryption_algorithm),
                encryption_key: Some(encryption_key.clone()),
                encryption_key_sha256: Some(encryption_key_sha256.clone()),
                ..Default::default()
            }),
        )
        .await?;

    // CPK Resize Scenario
    let resize_options = PageBlobClientResizeOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    page_blob_client.resize(1024, Some(resize_options)).await?;

    // Assert
    let response = blob_client
        .get_properties(Some(BlobClientGetPropertiesOptions {
            encryption_algorithm: Some(encryption_algorithm),
            encryption_key: Some(encryption_key),
            encryption_key_sha256: Some(encryption_key_sha256),
            ..Default::default()
        }))
        .await?;
    assert_eq!(1024, response.content_length()?.unwrap());

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_upload_pages_from_url_encryption_options(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    let (encryption_algorithm, encryption_key, encryption_key_sha256) = customer_provided_key();

    // Setup Source CPK Page Blob Scenario
    let source_blob_client =
        container_client.blob_client(&format!("{}-source-url", get_blob_name(recording)));
    let source_page_blob_client = source_blob_client.page_blob_client();
    source_page_blob_client
        .create(
            512,
            Some(PageBlobClientCreateOptions {
                encryption_algorithm: Some(encryption_algorithm),
                encryption_key: Some(encryption_key.clone()),
                encryption_key_sha256: Some(encryption_key_sha256.clone()),
                ..Default::default()
            }),
        )
        .await?;
    let source_content = vec![b'D'; 512];
    source_page_blob_client
        .upload_pages(
            RequestContent::from(source_content.clone()),
            512,
            format_page_range(0, 512)?,
            Some(PageBlobClientUploadPagesOptions {
                encryption_algorithm: Some(encryption_algorithm),
                encryption_key: Some(encryption_key.clone()),
                encryption_key_sha256: Some(encryption_key_sha256.clone()),
                ..Default::default()
            }),
        )
        .await?;

    // Setup Destination CPK Page Blob Scenario
    let dest_blob_client =
        container_client.blob_client(&format!("{}-dest-url", get_blob_name(recording)));
    let dest_page_blob_client = dest_blob_client.page_blob_client();
    dest_page_blob_client
        .create(
            512,
            Some(PageBlobClientCreateOptions {
                encryption_algorithm: Some(encryption_algorithm),
                encryption_key: Some(encryption_key.clone()),
                encryption_key_sha256: Some(encryption_key_sha256.clone()),
                ..Default::default()
            }),
        )
        .await?;

    // Destination + Source Encryption Upload Pages From URL Scenario
    let upload_from_url_options = PageBlobClientUploadPagesFromUrlOptions {
        encryption_algorithm: Some(encryption_algorithm),
        encryption_key: Some(encryption_key.clone()),
        encryption_key_sha256: Some(encryption_key_sha256.clone()),
        source_encryption_algorithm: Some(encryption_algorithm),
        source_encryption_key: Some(encryption_key.clone()),
        source_encryption_key_sha256: Some(encryption_key_sha256.clone()),
        ..Default::default()
    };
    dest_page_blob_client
        .upload_pages_from_url(
            source_blob_client.url().as_str().into(),
            format_page_range(0, 512)?,
            512,
            format_page_range(0, 512)?,
            Some(upload_from_url_options),
        )
        .await?;

    // Assert
    let response = dest_blob_client
        .download(Some(BlobClientDownloadOptions {
            encryption_algorithm: Some(encryption_algorithm),
            encryption_key: Some(encryption_key),
            encryption_key_sha256: Some(encryption_key_sha256),
            ..Default::default()
        }))
        .await?;
    let (_, _, body) = response.deconstruct();
    assert_eq!(source_content, body.collect().await?.to_vec());

    // Invalid Encryption Scope Scenario
    let invalid_dest_blob_client =
        container_client.blob_client(&format!("{}-dest-url-bad-scope", get_blob_name(recording)));
    let invalid_dest_page_blob_client = invalid_dest_blob_client.page_blob_client();
    invalid_dest_page_blob_client.create(512, None).await?;
    let invalid_options = PageBlobClientUploadPagesFromUrlOptions {
        encryption_scope: Some(invalid_encryption_scope()),
        ..Default::default()
    };
    let result = invalid_dest_page_blob_client
        .upload_pages_from_url(
            source_blob_client.url().as_str().into(),
            format_page_range(0, 512)?,
            512,
            format_page_range(0, 512)?,
            Some(invalid_options),
        )
        .await;

    // Assert
    let status = result.unwrap_err().http_status();
    assert_bad_request_or_conflict(status);

    container_client.delete(None).await?;
    Ok(())
}
