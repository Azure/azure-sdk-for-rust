// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![cfg(not(target_arch = "wasm32"))]

use azure_core_test::{recorded, stream::GeneratedStream, TestContext};
use azure_storage_blob::{
    format_page_range,
    models::{BlockListType, BlockLookupList},
    BlobClient,
};
use azure_storage_blob_test::{get_blob_name, get_container_client, StorageAccount};
use futures::TryStreamExt as _;
use std::error::Error;

#[tracing::instrument(skip_all, fields(content_length = CONTENT_LENGTH), err)]
async fn upload<const CONTENT_LENGTH: usize>(client: &BlobClient) -> azure_core::Result<()> {
    // Would read from a file stream e.g., `azure_core::fs::FileStream`,
    // but to avoid consuming a large amount of drive space generate content.
    let stream = GeneratedStream::<_, CONTENT_LENGTH>::default();

    client.upload(stream.into(), true, None).await?;

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

#[recorded::test]
async fn test_stream_blob_client_upload(ctx: TestContext) -> Result<(), Box<dyn Error>> {
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
    container_client.delete_container(None).await?;

    Ok(())
}

#[recorded::test]
async fn test_stream_page_blob_upload_page(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let page_blob_client = blob_client.page_blob_client();

    // Page blob size must be aligned to 512 bytes
    // Use 1MB to exercise streaming across multiple chunks (default chunk is 1KB)
    const CONTENT_LENGTH: usize = 512 * 2048; // 1,048,576 bytes (1MB, 2048 pages)

    // Create the page blob first with sufficient size
    page_blob_client.create(CONTENT_LENGTH as u64, None).await?;

    // Upload from a stream
    let stream = GeneratedStream::<_, CONTENT_LENGTH>::default();
    page_blob_client
        .upload_page(
            stream.into(),
            format_page_range(0, CONTENT_LENGTH as u64)?,
            None,
        )
        .await?;

    // Verify by downloading
    let len = download(&blob_client).await? as usize;
    assert_eq!(len, CONTENT_LENGTH);

    // Cleanup
    container_client.delete_container(None).await?;

    Ok(())
}

#[recorded::test]
async fn test_stream_append_blob_append_block(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let append_blob_client = blob_client.append_blob_client();

    // Create the append blob first
    append_blob_client.create(None).await?;

    // Append first block from a stream (1MB to exercise streaming across multiple chunks)
    const BLOCK1_LENGTH: usize = 1024 * 1024;
    let stream1 = GeneratedStream::<_, BLOCK1_LENGTH>::default();
    append_blob_client
        .append_block(stream1.into(), None)
        .await?;

    // Append second block from a stream (512KB)
    const BLOCK2_LENGTH: usize = 512 * 1024;
    let stream2 = GeneratedStream::<_, BLOCK2_LENGTH>::default();
    append_blob_client
        .append_block(stream2.into(), None)
        .await?;

    // Verify total length by downloading
    let len = download(&blob_client).await? as usize;
    assert_eq!(len, BLOCK1_LENGTH + BLOCK2_LENGTH);

    // Cleanup
    container_client.delete_container(None).await?;

    Ok(())
}

#[recorded::test]
async fn test_stream_block_blob_stage_block(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let block_blob_client = blob_client.block_blob_client();

    // Stage first block from a stream (2MB to exercise streaming across multiple chunks)
    const BLOCK1_LENGTH: usize = 2 * 1024 * 1024;
    let block1_id: Vec<u8> = b"block001".to_vec();
    let stream1 = GeneratedStream::<_, BLOCK1_LENGTH>::default();
    block_blob_client
        .stage_block(&block1_id, stream1.into(), None)
        .await?;

    // Stage second block from a stream (1MB)
    const BLOCK2_LENGTH: usize = 1024 * 1024;
    let block2_id: Vec<u8> = b"block002".to_vec();
    let stream2 = GeneratedStream::<_, BLOCK2_LENGTH>::default();
    block_blob_client
        .stage_block(&block2_id, stream2.into(), None)
        .await?;

    // Verify blocks are staged (uncommitted)
    let block_list = block_blob_client
        .get_block_list(BlockListType::All, None)
        .await?
        .into_model()?;
    assert!(block_list.committed_blocks.is_none());
    assert_eq!(
        2,
        block_list
            .uncommitted_blocks
            .expect("expected uncommitted_blocks")
            .len()
    );

    // Commit the blocks
    let block_lookup_list = BlockLookupList {
        committed: Some(Vec::new()),
        latest: Some(vec![block1_id, block2_id]),
        uncommitted: Some(Vec::new()),
    };
    block_blob_client
        .commit_block_list(block_lookup_list.try_into()?, None)
        .await?;

    // Verify total length by downloading
    let len = download(&blob_client).await? as usize;
    assert_eq!(len, BLOCK1_LENGTH + BLOCK2_LENGTH);

    // Cleanup
    container_client.delete_container(None).await?;

    Ok(())
}
