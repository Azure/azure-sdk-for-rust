// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::{RequestContent, StatusCode};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::models::{
    BlobClientDownloadResultHeaders, BlobClientGetPropertiesResultHeaders, BlobType,
};
use azure_storage_blob_test::{
    create_test_blob, get_blob_name, get_container_client, StorageAccount,
};
use std::error::Error;

#[recorded::test]
async fn test_create_append_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let append_blob_client = blob_client.append_blob_client();

    append_blob_client.create(None).await?;

    // Assert
    let blob_properties = blob_client.get_properties(None).await?;
    let blob_type = blob_properties.blob_type()?;
    let content_length = blob_properties.content_length()?;

    assert_eq!(0, content_length.unwrap());
    assert_eq!(BlobType::AppendBlob, blob_type.unwrap());

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_append_block(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let append_blob_client = blob_client.append_blob_client();
    append_blob_client.create(None).await?;
    let mut block_1 = b"hello".to_vec();
    let block_2 = b" rusty world".to_vec();

    // Act
    append_blob_client
        .append_block(
            RequestContent::from(block_1.clone()),
            u64::try_from(block_1.len())?,
            None,
        )
        .await?;
    append_blob_client
        .append_block(
            RequestContent::from(block_2.clone()),
            u64::try_from(block_2.len())?,
            None,
        )
        .await?;

    // Assert
    let response = blob_client.download(None).await?;
    let content_length = response.content_length()?;
    let (status_code, _, response_body) = response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(17, content_length.unwrap());
    block_1.extend(&block_2);
    assert_eq!(block_1, response_body.collect().await?.to_vec());

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_append_block_from_url(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let blob_client_2 = container_client.blob_client(&get_blob_name(recording));
    create_test_blob(&blob_client_2, None, None).await?;
    let append_blob_client = blob_client.append_blob_client();
    append_blob_client.create(None).await?;

    // Act
    append_blob_client
        .append_block_from_url(blob_client_2.url().as_str().into(), 17, None)
        .await?;

    // Assert
    let response = blob_client.download(None).await?;
    let content_length = response.content_length()?;
    let (status_code, _, response_body) = response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(17, content_length.unwrap());
    assert_eq!(
        b"hello rusty world".to_vec(),
        response_body.collect().await?.to_vec(),
    );

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_seal_append_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let append_blob_client = blob_client.append_blob_client();
    append_blob_client.create(None).await?;
    let test_block = b"test".to_vec();

    // Act
    append_blob_client.seal(None).await?;
    let response = append_blob_client
        .append_block(
            RequestContent::from(test_block.clone()),
            u64::try_from(test_block.len())?,
            None,
        )
        .await;

    // Assert
    let error = response.unwrap_err().http_status();
    assert_eq!(StatusCode::Conflict, error.unwrap());

    // Check Read-Only
    let response = blob_client.download(None).await?;
    let content_length = response.content_length()?;
    let (status_code, _, response_body) = response.deconstruct();

    // Assert
    assert!(status_code.is_success());
    assert_eq!(0, content_length.unwrap());
    assert_eq!(b"".to_vec(), response_body.collect().await?.to_vec());

    container_client.delete_container(None).await?;
    Ok(())
}
