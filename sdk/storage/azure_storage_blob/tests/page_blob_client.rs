// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::{RequestContent, StatusCode};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::{
    format_page_range,
    models::{
        BlobClientDownloadResultHeaders, BlobClientGetPropertiesResultHeaders, BlobType,
        PageBlobClientCreateOptions, PageBlobClientSetSequenceNumberOptions,
        PageBlobClientSetSequenceNumberResultHeaders, SequenceNumberActionType,
    },
};
use azure_storage_blob_test::{get_blob_name, get_container_client, StorageAccount};
use std::error::Error;

#[recorded::test]
async fn test_create_page_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup

    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
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
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let page_blob_client = blob_client.page_blob_client();
    page_blob_client.create(512, None).await?;
    let data = vec![b'A'; 512];
    page_blob_client
        .upload_page(
            RequestContent::from(data.clone()),
            512,
            format_page_range(0, 512)?,
            None,
        )
        .await?;

    // Assert
    let response = blob_client.download(None).await?;
    let content_length = response.content_length()?;
    let (status_code, _, response_body) = response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(512, content_length.unwrap());
    assert_eq!(data, response_body.collect().await?.to_vec());

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_clear_page(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let page_blob_client = blob_client.page_blob_client();
    page_blob_client.create(512, None).await?;
    let data = vec![b'A'; 512];
    page_blob_client
        .upload_page(
            RequestContent::from(data),
            512,
            format_page_range(0, 512)?,
            None,
        )
        .await?;

    page_blob_client
        .clear_page(format_page_range(0, 512)?, None)
        .await?;

    // Assert
    let response = blob_client.download(None).await?;
    let content_length = response.content_length()?;
    let (status_code, _, response_body) = response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(512, content_length.unwrap());
    assert_eq!(vec![0; 512], response_body.collect().await?.to_vec());

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_resize_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let page_blob_client = blob_client.page_blob_client();

    // Blob Too Small Scenario
    page_blob_client.create(512, None).await?;
    let data = vec![b'A'; 1024];
    let response = page_blob_client
        .upload_page(
            RequestContent::from(data.clone()),
            1024,
            format_page_range(0, 1024)?,
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
            format_page_range(0, 1024)?,
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
    assert_eq!(vec![b'A'; 512], response_body.collect().await?.to_vec());

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_set_sequence_number(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup

    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let page_blob_client = blob_client.page_blob_client();

    // Update Action
    page_blob_client.create(1024, None).await?;
    let sequence_number_options = PageBlobClientSetSequenceNumberOptions {
        blob_sequence_number: Some(7),
        ..Default::default()
    };
    let response = page_blob_client
        .set_sequence_number(
            SequenceNumberActionType::Update,
            Some(sequence_number_options),
        )
        .await?;
    let blob_sequence_number = response.blob_sequence_number()?;
    assert_eq!(7, blob_sequence_number.unwrap());

    // Increment Action
    let response = page_blob_client
        .set_sequence_number(SequenceNumberActionType::Increment, None)
        .await?;
    let blob_sequence_number = response.blob_sequence_number()?;
    assert_eq!(8, blob_sequence_number.unwrap());

    // Set Max Action
    let sequence_number_options = PageBlobClientSetSequenceNumberOptions {
        blob_sequence_number: Some(5),
        ..Default::default()
    };
    page_blob_client
        .set_sequence_number(SequenceNumberActionType::Max, Some(sequence_number_options))
        .await?;
    let blob_sequence_number = response.blob_sequence_number()?;
    assert_eq!(8, blob_sequence_number.unwrap());

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
#[ignore = "https://github.com/Azure/azure-sdk-for-rust/issues/3441"]
async fn test_upload_page_from_url(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client_1 = container_client.blob_client(&get_blob_name(recording));
    let blob_client_2 = container_client.blob_client(&get_blob_name(recording));
    let page_blob_client_1 = blob_client_1.page_blob_client();
    let page_blob_client_2 = blob_client_2.page_blob_client();

    // Act
    page_blob_client_1.create(512, None).await?;
    let data_b = vec![b'B'; 512];
    page_blob_client_1
        .upload_page(
            RequestContent::from(data_b.clone()),
            512,
            format_page_range(0, 512)?,
            None,
        )
        .await?;

    page_blob_client_2.create(1024, None).await?;
    let mut data_a = vec![b'A'; 512];
    page_blob_client_2
        .upload_page(
            RequestContent::from(data_a.clone()),
            512,
            format_page_range(0, 512)?,
            None,
        )
        .await?;
    page_blob_client_2
        .upload_pages_from_url(
            blob_client_1.url().as_str().into(),
            format_page_range(0, data_b.len() as u64)?,
            data_b.len() as u64,
            format_page_range(512, data_b.len() as u64)?,
            None,
        )
        .await?;

    // Assert
    let response = blob_client_2.download(None).await?;
    let content_length = response.content_length()?;
    let (status_code, _, response_body) = response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(1024, content_length.unwrap());
    data_a.extend(&data_b);
    assert_eq!(data_a, response_body.collect().await?.to_vec());

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_get_page_ranges(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let page_blob_client = blob_client.page_blob_client();
    page_blob_client.create(1024, None).await?;

    // Empty Page Range Scenario
    let get_page_ranges_response = page_blob_client.get_page_ranges(None).await?;
    // Assert
    let page_ranges = get_page_ranges_response.into_model()?;
    let page_range = page_ranges.page_range;
    assert!(page_range.is_none());

    // Non-Empty Page Range Scenario
    let data = vec![b'A'; 512];
    page_blob_client
        .upload_page(
            RequestContent::from(data.clone()),
            512,
            format_page_range(0, 512)?,
            None,
        )
        .await?;
    let get_page_ranges_response = page_blob_client.get_page_ranges(None).await?;
    // Assert
    let page_ranges = get_page_ranges_response.into_model()?;
    let page_range = page_ranges.page_range.unwrap();
    for range in page_range {
        assert_eq!(0, range.start.unwrap());
        assert_eq!(511, range.end.unwrap());
    }

    container_client.delete_container(None).await?;
    Ok(())
}
