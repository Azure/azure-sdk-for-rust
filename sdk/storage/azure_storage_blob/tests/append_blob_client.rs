// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::{headers::CONTENT_TYPE, RequestContent, StatusCode};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::models::{
    AppendBlobClientAppendBlockFromUrlOptions, AppendBlobClientAppendBlockOptions,
    AppendBlobClientCreateOptions, BlobClientGetPropertiesResultHeaders, BlobType,
};
use azure_storage_blob_test::{
    create_test_blob, get_blob_name, get_container_client, StorageAccount,
};
use std::{collections::HashMap, error::Error};

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
            blob_content_type: Some("image/jpeg".to_string()),
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
    assert_eq!(Some("image/jpeg".to_string()), content_type);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_append_block_position_condition(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let append_blob_client = blob_client.append_blob_client();
    append_blob_client.create(None).await?;

    let data = b"hello".to_vec();

    // Wrong Append Position Scenario
    let response = append_blob_client
        .append_block(
            RequestContent::from(data.clone()),
            u64::try_from(data.len())?,
            Some(AppendBlobClientAppendBlockOptions {
                append_position: Some(10),
                ..Default::default()
            }),
        )
        .await;

    // Assert
    assert_eq!(
        StatusCode::PreconditionFailed,
        response.unwrap_err().http_status().unwrap()
    );

    // Correct Append Position Scenario
    append_blob_client
        .append_block(
            RequestContent::from(data.clone()),
            u64::try_from(data.len())?,
            Some(AppendBlobClientAppendBlockOptions {
                append_position: Some(0),
                ..Default::default()
            }),
        )
        .await?;

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_append_block_max_size_condition(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let append_blob_client = blob_client.append_blob_client();
    append_blob_client.create(None).await?;

    let data = b"hello world".to_vec(); // 11 bytes

    // Exceeds Max Size Scenario
    let response = append_blob_client
        .append_block(
            RequestContent::from(data.clone()),
            u64::try_from(data.len())?,
            Some(AppendBlobClientAppendBlockOptions {
                max_size: Some(5),
                ..Default::default()
            }),
        )
        .await;

    // Assert
    assert_eq!(
        StatusCode::PreconditionFailed,
        response.unwrap_err().http_status().unwrap()
    );

    // Within Max Size Scenario
    append_blob_client
        .append_block(
            RequestContent::from(data.clone()),
            u64::try_from(data.len())?,
            Some(AppendBlobClientAppendBlockOptions {
                max_size: Some(20),
                ..Default::default()
            }),
        )
        .await?;

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_append_block_transactional_checksums(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let append_blob_client = blob_client.append_blob_client();
    append_blob_client.create(None).await?;

    let content = b"hello".to_vec();
    // MD5("hello") - well-known test vector
    let correct_md5: Vec<u8> = vec![
        0x5d, 0x41, 0x40, 0x2a, 0xbc, 0x4b, 0x2a, 0x76, 0xb9, 0x71, 0x9d, 0x91, 0x10, 0x17, 0xc5,
        0x92,
    ];
    // CRC64-ECMA-182 of b"hello", server-confirmed (base64: V0JSBnCFdzM=)
    let correct_crc64: Vec<u8> = vec![87, 66, 82, 6, 112, 133, 119, 51];

    // MD5 Mismatch Scenario
    let response = append_blob_client
        .append_block(
            RequestContent::from(content.clone()),
            u64::try_from(content.len())?,
            Some(AppendBlobClientAppendBlockOptions {
                transactional_content_md5: Some(vec![0u8; 16]),
                ..Default::default()
            }),
        )
        .await;
    assert_eq!(
        StatusCode::BadRequest,
        response.unwrap_err().http_status().unwrap()
    );

    // MD5 Match Scenario
    append_blob_client
        .append_block(
            RequestContent::from(content.clone()),
            u64::try_from(content.len())?,
            Some(AppendBlobClientAppendBlockOptions {
                transactional_content_md5: Some(correct_md5),
                ..Default::default()
            }),
        )
        .await?;

    // CRC64 Mismatch Scenario
    let response = append_blob_client
        .append_block(
            RequestContent::from(content.clone()),
            u64::try_from(content.len())?,
            Some(AppendBlobClientAppendBlockOptions {
                transactional_content_crc64: Some(vec![0u8; 8]),
                ..Default::default()
            }),
        )
        .await;
    assert_eq!(
        StatusCode::BadRequest,
        response.unwrap_err().http_status().unwrap()
    );

    // CRC64 Match Scenario
    append_blob_client
        .append_block(
            RequestContent::from(content.clone()),
            u64::try_from(content.len())?,
            Some(AppendBlobClientAppendBlockOptions {
                transactional_content_crc64: Some(correct_crc64),
                ..Default::default()
            }),
        )
        .await?;

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_create_append_blob_with_tags(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let append_blob_client = blob_client.append_blob_client();

    let expected = HashMap::from([("env".to_string(), "test".to_string())]);
    append_blob_client
        .create(Some(AppendBlobClientCreateOptions {
            blob_tags_string: Some("env=test".to_string()),
            ..Default::default()
        }))
        .await?;

    // Assert
    let map: HashMap<String, String> = blob_client.get_tags(None).await?.into_model()?.into();
    assert_eq!(expected, map);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_create_append_blob_if_not_exists(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let append_blob_client = blob_client.append_blob_client();

    // Create Initial Blob
    append_blob_client.create(None).await?;

    // If Not Exists Scenario (blob already exists)
    let result = append_blob_client
        .create(Some(
            AppendBlobClientCreateOptions::default().with_if_not_exists(),
        ))
        .await;

    // Assert
    assert_eq!(
        StatusCode::Conflict,
        result.unwrap_err().http_status().unwrap()
    );

    // Blob Should Still Exist
    let props = blob_client.get_properties(None).await?;
    assert_eq!(Some(0), props.content_length()?);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_append_block_from_url_source_if_match(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let source_blob_client = container_client.blob_client(&get_blob_name(recording));
    let source_data = b"source";
    create_test_blob(
        &source_blob_client,
        Some(RequestContent::from(source_data.to_vec())),
        None,
    )
    .await?;
    let etag = source_blob_client
        .get_properties(None)
        .await?
        .etag()?
        .unwrap()
        .to_string();

    let dest_blob_client = container_client.blob_client(&get_blob_name(recording));
    let append_blob_client = dest_blob_client.append_blob_client();
    append_blob_client.create(None).await?;

    // Source If-Match Scenario
    append_blob_client
        .append_block_from_url(
            source_blob_client.url().as_str().into(),
            u64::try_from(source_data.len())?,
            Some(AppendBlobClientAppendBlockFromUrlOptions {
                source_if_match: Some(etag.clone().into()),
                ..Default::default()
            }),
        )
        .await?;

    // Source If-None-Match Scenario (ETag matches, so condition is not satisfied)
    let response = append_blob_client
        .append_block_from_url(
            source_blob_client.url().as_str().into(),
            u64::try_from(source_data.len())?,
            Some(AppendBlobClientAppendBlockFromUrlOptions {
                source_if_none_match: Some(etag.into()),
                ..Default::default()
            }),
        )
        .await;

    // Assert
    assert_eq!(
        StatusCode::NotModified,
        response.unwrap_err().http_status().unwrap()
    );

    container_client.delete(None).await?;
    Ok(())
}
