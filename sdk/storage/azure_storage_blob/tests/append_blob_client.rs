// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::{headers::CONTENT_TYPE, RequestContent, StatusCode};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::models::{
    AppendBlobClientCreateOptions, BlobClientGetPropertiesResultHeaders, BlobType,
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

    container_client.delete(None).await?;
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
    assert_eq!(17, response.properties.content_length.unwrap());
    let body_data = response.body.collect().await?;
    block_1.extend(&block_2);
    assert_eq!(block_1, body_data);

    container_client.delete(None).await?;
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
    assert_eq!(17, response.properties.content_length.unwrap());
    let body_data = response.body.collect().await?;
    assert_eq!(b"hello rusty world".to_vec(), body_data);

    container_client.delete(None).await?;
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
    // Assert
    assert_eq!(0, response.properties.content_length.unwrap());
    let body_data = response.body.collect().await?;
    assert_eq!(b"".to_vec(), body_data);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_create_append_blob_content_headers(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let append_blob_client = blob_client.append_blob_client();

    // Create with Content Headers
    // Note: blob_content_md5 is validated against actual content on create and is excluded
    // here; it is tested as stored metadata via set_properties in blob_client tests.
    append_blob_client
        .create(Some(AppendBlobClientCreateOptions {
            blob_cache_control: Some("must-revalidate".to_string()),
            blob_content_disposition: Some("inline".to_string()),
            blob_content_encoding: Some("identity".to_string()),
            blob_content_language: Some("es-ES".to_string()),
            blob_content_type: Some("text/csv".to_string()),
            ..Default::default()
        }))
        .await?;

    // Assert Content Headers Roundtrip
    let props = blob_client.get_properties(None).await?;
    assert_eq!(Some("must-revalidate".to_string()), props.cache_control()?);
    assert_eq!(Some("inline".to_string()), props.content_disposition()?);
    assert_eq!(Some("identity".to_string()), props.content_encoding()?);
    assert_eq!(Some("es-ES".to_string()), props.content_language()?);
    let content_type: Option<String> = props.headers().get_optional_as(&CONTENT_TYPE)?;
    assert_eq!(Some("text/csv".to_string()), content_type);

    container_client.delete(None).await?;
    Ok(())
}
