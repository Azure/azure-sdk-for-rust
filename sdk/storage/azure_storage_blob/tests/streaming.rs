// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![cfg(not(target_arch = "wasm32"))]

use azure_core::{
    http::{Body, NoFormat, RequestContent},
    stream::BytesStream,
    Bytes,
};
use azure_core_test::{recorded, stream::GeneratedStream, TestContext};
use azure_storage_blob::{
    format_page_range,
    models::{BlobClientDownloadResultHeaders, BlockLookupList},
    BlobClient,
};
use azure_storage_blob_test::{get_blob_name, get_container_client, StorageAccount};
use futures::TryStreamExt as _;
use std::error::Error;

#[recorded::test(live)]
async fn stream(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));

    // Upload from a stream.
    const CONTENT_LENGTH: usize = 40_960_000;
    upload::<CONTENT_LENGTH>(&blob_client).await?;

    // Download to a stream.
    let len = download(&blob_client).await? as usize;
    assert_eq!(len, CONTENT_LENGTH);

    // Cleanup
    container_client.delete(None).await?;

    Ok(())
}

#[recorded::test]
async fn stream_blob_upload(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));

    let data = b"streaming a stream";
    let content = request_content_from_bytes(data);

    // Upload via BlobClient using a seekable stream body
    blob_client
        .upload(content, true, data.len() as u64, None)
        .await?;

    // Assert
    let response = blob_client.download(None).await?;
    let content_length = response.content_length()?;
    let (status_code, _, response_body) = response.deconstruct();
    assert!(status_code.is_success());
    assert_eq!(data.len() as u64, content_length.unwrap());
    assert_eq!(data.to_vec(), response_body.collect().await?.to_vec());

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn stream_stage_block(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let block_blob_client = blob_client.block_blob_client();

    let block_1_data = b"one block";
    let block_2_data = b"two block";
    let block_1_id: Vec<u8> = b"1".to_vec();
    let block_2_id: Vec<u8> = b"2".to_vec();

    // Stage blocks using seekable stream bodies
    block_blob_client
        .stage_block(
            &block_1_id,
            block_1_data.len() as u64,
            request_content_from_bytes(block_1_data),
            None,
        )
        .await?;
    block_blob_client
        .stage_block(
            &block_2_id,
            block_2_data.len() as u64,
            request_content_from_bytes(block_2_data),
            None,
        )
        .await?;

    // Commit the block list
    let block_lookup_list = BlockLookupList {
        committed: Some(Vec::new()),
        latest: Some(vec![block_1_id, block_2_id]),
        uncommitted: Some(Vec::new()),
    };
    block_blob_client
        .commit_block_list(block_lookup_list.try_into()?, None)
        .await?;

    // Assert
    let response = blob_client.download(None).await?;
    let content_length = response.content_length()?;
    let (status_code, _, response_body) = response.deconstruct();
    assert!(status_code.is_success());
    let expected_len = block_1_data.len() + block_2_data.len();
    assert_eq!(expected_len as u64, content_length.unwrap());
    let mut expected = block_1_data.to_vec();
    expected.extend_from_slice(block_2_data);
    assert_eq!(expected, response_body.collect().await?.to_vec());

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn stream_append_block(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let append_blob_client = blob_client.append_blob_client();
    append_blob_client.create(None).await?;

    let block_1 = b"appended via";
    let block_2 = b" seekable stream";

    // Append blocks using seekable stream bodies
    append_blob_client
        .append_block(
            request_content_from_bytes(block_1),
            block_1.len() as u64,
            None,
        )
        .await?;
    append_blob_client
        .append_block(
            request_content_from_bytes(block_2),
            block_2.len() as u64,
            None,
        )
        .await?;

    // Assert
    let response = blob_client.download(None).await?;
    let content_length = response.content_length()?;
    let (status_code, _, response_body) = response.deconstruct();
    assert!(status_code.is_success());
    let expected_len = block_1.len() + block_2.len();
    assert_eq!(expected_len as u64, content_length.unwrap());
    let mut expected = block_1.to_vec();
    expected.extend_from_slice(block_2);
    assert_eq!(expected, response_body.collect().await?.to_vec());

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn stream_upload_pages(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let page_blob_client = blob_client.page_blob_client();

    // Page blobs require 512-byte aligned data
    let data = vec![b'P'; 512];
    page_blob_client.create(512, None).await?;

    // Upload pages using a seekable stream body
    page_blob_client
        .upload_pages(
            request_content_from_bytes(&data),
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

    container_client.delete(None).await?;
    Ok(())
}

/// Creates a `RequestContent<Bytes, NoFormat>` from a byte slice by wrapping it in a
/// `BytesStream` and flowing through `Body::SeekableStream`. This mirrors the data-flow
/// of uploading from a file using `FileStream`, but uses in-memory data instead.
fn request_content_from_bytes(data: &[u8]) -> RequestContent<Bytes, NoFormat> {
    let bytes = Bytes::copy_from_slice(data);
    let stream = BytesStream::new(bytes);
    let body = Body::SeekableStream(Box::new(stream));
    body.into()
}

#[tracing::instrument(skip_all, fields(content_length = CONTENT_LENGTH), err)]
async fn upload<const CONTENT_LENGTH: usize>(client: &BlobClient) -> azure_core::Result<()> {
    // Would read from a file stream e.g., `azure_core::fs::FileStream`,
    // but to avoid consuming a large amount of drive space generate content.
    let stream = GeneratedStream::<_, CONTENT_LENGTH>::default();

    client
        .upload(stream.into(), true, CONTENT_LENGTH as u64, None)
        .await?;

    Ok(())
}

#[tracing::instrument(skip_all, fields(content_length), err)]
async fn download(client: &BlobClient) -> azure_core::Result<u64> {
    let mut len = 0;
    let mut response = client.download(None).await?.into_body();
    while let Some(data) = response.try_next().await? {
        tracing::debug!("received {} bytes", data.len());

        // Would write to a file stream e.g., `azure_core::fs::FileStream`,
        // but to avoid consuming a large amount of drive space only tally content length.
        len += data.len() as u64;
    }

    tracing::Span::current().record("content_length", len);
    Ok(len)
}
