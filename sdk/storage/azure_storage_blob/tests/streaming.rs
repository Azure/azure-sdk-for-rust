// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![cfg(not(target_arch = "wasm32"))]

use azure_core_test::{recorded, stream::GeneratedStream, TestContext};
use azure_storage_blob::BlobClient;
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
    container_client.delete_container(None).await?;

    Ok(())
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
