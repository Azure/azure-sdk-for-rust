// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{headers::HeaderName, Bytes, RequestContent, StatusCode};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::{
    clients::{BlobClient, ContainerClient},
    models::{BlobBlobClientDownloadOptions, BlobBlobClientGetPropertiesOptions, BlobType},
    BlobClientOptions,
};
use std::{env, error::Error};

#[recorded::test]
async fn test_get_blob_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let mut options = BlobClientOptions::default();
    recording.instrument(&mut options.client_options);

    // Setup
    let endpoint = format!(
        "https://{}.blob.core.windows.net/",
        recording.var("AZURE_STORAGE_ACCOUNT_NAME", None).as_str()
    );

    // Act
    let container_client = ContainerClient::new(
        &endpoint,
        String::from("testcontainer1"),
        recording.credential(),
        Some(options.clone()),
    )?;
    container_client.create_container(None).await?;

    let blob_client = BlobClient::new(
        &endpoint,
        String::from("testcontainer1"),
        String::from("test_blob.txt"),
        recording.credential(),
        Some(options),
    )?;
    let data = b"hello rusty world";
    blob_client
        .upload_blob(
            RequestContent::from(data.to_vec()),
            true,
            i64::try_from(data.len())?,
            None,
        )
        .await?;
    let response = blob_client
        .get_blob_properties(Some(BlobBlobClientGetPropertiesOptions::default()))
        .await;

    // Assert
    assert!(response.is_ok());

    let blob_properties = response?;
    assert_eq!(blob_properties.blob_type, Some(BlobType::BlockBlob));
    assert_eq!(blob_properties.content_length, Some(17));

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_get_blob_properties_invalid_container(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let mut options = BlobClientOptions::default();
    recording.instrument(&mut options.client_options);

    // Setup
    let endpoint = format!(
        "https://{}.blob.core.windows.net/",
        recording.var("AZURE_STORAGE_ACCOUNT_NAME", None).as_str()
    );

    // Act
    let blob_client = BlobClient::new(
        &endpoint,
        String::from("missingcontainer"),
        String::from("test_blob.txt"),
        recording.credential(),
        Some(options),
    )?;
    let response = blob_client
        .get_blob_properties(Some(BlobBlobClientGetPropertiesOptions::default()))
        .await;

    // Assert
    assert_eq!(
        String::from("HttpResponse(NotFound, \"ContainerNotFound\")"),
        response.unwrap_err().kind().to_string()
    );

    Ok(())
}

#[recorded::test]
async fn test_download_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let mut options = BlobClientOptions::default();
    recording.instrument(&mut options.client_options);

    // Setup
    let endpoint = format!(
        "https://{}.blob.core.windows.net/",
        recording.var("AZURE_STORAGE_ACCOUNT_NAME", None).as_str()
    );

    // Act
    let container_client = ContainerClient::new(
        &endpoint,
        String::from("testcontainer2"),
        recording.credential(),
        Some(options.clone()),
    )?;
    container_client.create_container(None).await?;

    let blob_client = BlobClient::new(
        &endpoint,
        String::from("testcontainer2"),
        String::from("test_blob.txt"),
        recording.credential(),
        Some(options),
    )?;
    let data = b"test download content";
    blob_client
        .upload_blob(
            RequestContent::from(data.to_vec()),
            true,
            i64::try_from(data.len())?,
            None,
        )
        .await?;
    let response = blob_client
        .download_blob(Some(BlobBlobClientDownloadOptions::default()))
        .await?;

    // Assert
    let (status_code, headers, response_body) = response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(
        "21",
        headers.get_str(&HeaderName::from_static("content-length"))?
    );
    assert_eq!(Bytes::from_static(data), response_body.collect().await?);

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_upload_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let mut options = BlobClientOptions::default();
    recording.instrument(&mut options.client_options);

    // Setup
    let endpoint = format!(
        "https://{}.blob.core.windows.net/",
        recording.var("AZURE_STORAGE_ACCOUNT_NAME", None).as_str()
    );

    // Act
    let container_client = ContainerClient::new(
        &endpoint,
        String::from("testcontainer3"),
        recording.credential(),
        Some(options.clone()),
    )?;
    container_client.create_container(None).await?;

    let blob_client = BlobClient::new(
        &endpoint,
        String::from("testcontainer3"),
        String::from("test_upload_blob.txt"),
        recording.credential(),
        Some(options),
    )?;

    let data = b"hello rusty world";
    let response = blob_client
        .upload_blob(
            RequestContent::from(data.to_vec()),
            false,
            i64::try_from(data.len())?,
            None,
        )
        .await?;

    // Assert
    assert_eq!(response.status(), StatusCode::Created);

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_upload_blob_overwrite(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let mut options = BlobClientOptions::default();
    recording.instrument(&mut options.client_options);

    // Setup
    let endpoint = format!(
        "https://{}.blob.core.windows.net/",
        recording.var("AZURE_STORAGE_ACCOUNT_NAME", None).as_str()
    );

    // Act
    let container_client = ContainerClient::new(
        &endpoint,
        String::from("testcontainer4"),
        recording.credential(),
        Some(options.clone()),
    )?;
    container_client.create_container(None).await?;

    let blob_client = BlobClient::new(
        &endpoint,
        String::from("testcontainer4"),
        String::from("test_upload_blob_overwrite.txt"),
        recording.credential(),
        Some(options),
    )?;

    let data = b"hello rusty world";
    blob_client
        .upload_blob(
            RequestContent::from(data.to_vec()),
            false,
            i64::try_from(data.len())?,
            None,
        )
        .await?;

    let data2 = b"hello overwritten rusty world";
    let response = blob_client
        .upload_blob(
            RequestContent::from(data2.to_vec()),
            true,
            i64::try_from(data2.len())?,
            None,
        )
        .await?;

    // Assert
    assert_eq!(response.status(), StatusCode::Created);

    container_client.delete_container(None).await?;
    Ok(())
}
