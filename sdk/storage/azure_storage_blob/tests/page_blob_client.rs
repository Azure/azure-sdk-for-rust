// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::{RequestContent, StatusCode};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::{
    models::{
        BlobClientDownloadResultHeaders, BlobClientGetPropertiesResultHeaders, BlobType,
        PageBlobClientCreateOptions,
    },
    PageBlobClient, PageBlobClientCreateOptionsExt, PageBlobClientExt,
};
use azure_storage_blob_test::{get_blob_name, get_container_client};
use std::error::Error;

#[recorded::test]
async fn test_create_page_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup

    let recording = ctx.recording();
    let container_client = get_container_client(recording, true).await?;
    let blob_client = container_client.blob_client(get_blob_name(recording));
    let page_blob_client = blob_client.page_blob_client();

    // Regular Create Scenario
    page_blob_client.create(1024, None).await?;
    // Assert
    let blob_properties = blob_client.get_properties(None).await?;
    let blob_type = blob_properties.blob_type()?;
    let content_length = blob_properties.content_length()?;
    assert_eq!(1024, content_length.unwrap());
    assert_eq!(BlobType::PageBlob, blob_type.unwrap());

    // Create If Not Exists Scenario
    let create_options = PageBlobClientCreateOptions::default().with_if_not_exists();
    let response = page_blob_client
        .create(1024, Some(create_options.clone()))
        .await;
    // Assert
    let error = response.unwrap_err().http_status();
    assert_eq!(StatusCode::Conflict, error.unwrap());

    blob_client.delete(None).await?;
    page_blob_client.create(1024, Some(create_options)).await?;
    // Assert
    let blob_properties = blob_client.get_properties(None).await?;
    let blob_type = blob_properties.blob_type()?;
    let content_length = blob_properties.content_length()?;
    assert_eq!(1024, content_length.unwrap());
    assert_eq!(BlobType::PageBlob, blob_type.unwrap());

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_upload_page(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true).await?;
    let blob_client = container_client.blob_client(get_blob_name(recording));
    let page_blob_client = blob_client.page_blob_client();
    page_blob_client.create(512, None).await?;
    let data = vec![b'A'; 512];
    page_blob_client
        .upload_page(
            RequestContent::from(data.clone()),
            512,
            PageBlobClient::format_http_range(0, 512),
            None,
        )
        .await?;

    // Assert
    let response = blob_client.download(None).await?;
    let content_length = response.content_length()?;
    let (status_code, _, response_body) = response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(512, content_length.unwrap());
    assert_eq!(data, response_body.collect().await?);

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_clear_page(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true).await?;
    let blob_client = container_client.blob_client(get_blob_name(recording));
    let page_blob_client = blob_client.page_blob_client();
    page_blob_client.create(512, None).await?;
    let data = vec![b'A'; 512];
    page_blob_client
        .upload_page(
            RequestContent::from(data),
            512,
            PageBlobClient::format_http_range(0, 512),
            None,
        )
        .await?;

    page_blob_client
        .clear_page(PageBlobClient::format_http_range(0, 512), None)
        .await?;

    // Assert
    let response = blob_client.download(None).await?;
    let content_length = response.content_length()?;
    let (status_code, _, response_body) = response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(512, content_length.unwrap());
    assert_eq!(vec![0; 512], response_body.collect().await?);

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_resize_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true).await?;
    let blob_client = container_client.blob_client(get_blob_name(recording));
    let page_blob_client = blob_client.page_blob_client();

    // Blob Too Small Scenario
    page_blob_client.create(512, None).await?;
    let data = vec![b'A'; 1024];
    let response = page_blob_client
        .upload_page(
            RequestContent::from(data.clone()),
            1024,
            PageBlobClient::format_http_range(0, 1024),
            None,
        )
        .await;
    // Assert
    let error = response.unwrap_err().http_status();
    assert_eq!(StatusCode::RequestedRangeNotSatisfiable, error.unwrap());

    page_blob_client.resize(1024, None).await?;
    page_blob_client
        .upload_page(
            RequestContent::from(data.clone()),
            1024,
            PageBlobClient::format_http_range(0, 1024),
            None,
        )
        .await?;

    // Truncate Blob Scenario
    page_blob_client.resize(512, None).await?;
    // Assert
    let response = blob_client.download(None).await?;
    let content_length = response.content_length()?;
    let (status_code, _, response_body) = response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(512, content_length.unwrap());
    assert_eq!(vec![b'A'; 512], response_body.collect().await?);

    container_client.delete_container(None).await?;
    Ok(())
}
