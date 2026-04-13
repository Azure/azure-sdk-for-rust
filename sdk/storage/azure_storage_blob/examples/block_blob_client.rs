// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Block blob client example for Azure Blob Storage.
//!
//! Block blobs are the most common blob type and are optimized for sequential
//! reads/writes of large streams. This sample demonstrates:
//! 1. Staged upload: stage three blocks individually, then commit them as a single blob.
//! 2. Inspect the committed block list and download to verify the assembled content.
//! 3. Copy a blob from another URL using `upload_blob_from_url` with the
//!    "if not exists" guard to prevent accidental overwrites.
//!
//! # Prerequisites
//!
//! - Set `AZURE_STORAGE_ACCOUNT_NAME` to your storage account name.
//! - Sign in with `az login` (or any other credential flow supported by [`DeveloperToolsCredential`]).
//!
//! # Usage
//!
//! ```bash
//! az login
//! export AZURE_STORAGE_ACCOUNT_NAME="<your-storage-account>"
//! cargo run --package azure_storage_blob --example block_blob_client
//! ```

use std::{collections::HashMap, env};

use azure_core::http::RequestContent;
use azure_identity::DeveloperToolsCredential;
use azure_storage_blob::{
    models::{
        BlockBlobClientUploadBlobFromUrlOptions, BlockBlobClientUploadOptions, BlockListType,
        BlockLookupList,
    },
    BlobContainerClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account = env::var("AZURE_STORAGE_ACCOUNT_NAME")
        .expect("Set AZURE_STORAGE_ACCOUNT_NAME environment variable");

    let endpoint = format!("https://{}.blob.core.windows.net/", account);
    let container_name = "test-container-block-blob";

    let credential = DeveloperToolsCredential::new(None)?;
    let container_client =
        BlobContainerClient::new(&endpoint, container_name, Some(credential), None)?;

    println!("Creating container '{container_name}'...");
    container_client.create(None).await?;

    staged_upload(&container_client).await?;
    copy_from_url(&container_client).await?;

    container_client.delete(None).await?;
    println!("Deleted container '{container_name}'");

    Ok(())
}

/// Stages three blocks individually, commits them as a single blob, and then
/// downloads the result to verify the content matches.
async fn staged_upload(
    container_client: &BlobContainerClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let blob_name = "staged-upload.txt";
    let block_blob_client = container_client.blob_client(blob_name).block_blob_client();

    let blocks: &[(&[u8], &[u8])] = &[
        (b"block-1", b"Hello, "),
        (b"block-2", b"block "),
        (b"block-3", b"blobs!"),
    ];

    // Stage each block. Block IDs are arbitrary byte sequences (same length per blob).
    for (block_id, data) in blocks {
        block_blob_client
            .stage_block(
                block_id,
                u64::try_from(data.len())?,
                RequestContent::from(data.to_vec()),
                None,
            )
            .await?;
        println!("Staged block '{}'", String::from_utf8_lossy(block_id));
    }

    // Commit them all in order.
    let latest_blocks: Vec<Vec<u8>> = blocks.iter().map(|(id, _)| id.to_vec()).collect();
    let block_lookup_list = BlockLookupList {
        committed: None,
        latest: Some(latest_blocks),
        uncommitted: None,
    };
    block_blob_client
        .commit_block_list(block_lookup_list.try_into()?, None)
        .await?;
    println!("Committed block list for '{blob_name}'");

    // Verify the committed block list.
    let block_list = block_blob_client
        .get_block_list(BlockListType::Committed, None)
        .await?
        .into_model()?;
    let committed = block_list.committed_blocks.as_deref().unwrap_or(&[]);
    println!("Block list now has {} committed block(s)", committed.len());

    // Download and verify the assembled content.
    let response = container_client
        .blob_client(blob_name)
        .download(None)
        .await?;
    let data = response.body.collect().await?;
    println!("Downloaded: {}", String::from_utf8_lossy(&data));

    Ok(())
}

/// Copies a blob from another URL and demonstrates the `with_if_not_exists`
/// guard that prevents clobbering an existing destination blob.
async fn copy_from_url(
    container_client: &BlobContainerClient,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a source blob to copy from, tagging it and guarding against accidental
    // overwrites with `with_if_not_exists` + `with_tags`.
    let source_blob_name = "copy-source.txt";
    let source_client = container_client.blob_client(source_blob_name);
    let upload_options = BlockBlobClientUploadOptions::default()
        .with_if_not_exists()
        .with_tags(HashMap::from([(
            "origin".to_string(),
            "sample".to_string(),
        )]));
    source_client
        .upload(
            RequestContent::from(b"original source content".to_vec()),
            Some(upload_options),
        )
        .await?;
    println!("Created source blob '{source_blob_name}'");

    // Read the tags back; `into_model()?.into()` converts BlobTags → HashMap via the From impl.
    let tags: HashMap<String, String> = source_client.get_tags(None).await?.into_model()?.into();
    println!("Tags on '{source_blob_name}': {tags:?}");

    // First copy: destination does not yet exist, so this succeeds.
    let dest_blob_name = "copy-dest.txt";
    let dest_client = container_client.blob_client(dest_blob_name);
    dest_client
        .block_blob_client()
        .upload_blob_from_url(source_client.url().as_str().into(), None)
        .await?;
    println!("Copied '{source_blob_name}' → '{dest_blob_name}'");

    // Second copy attempt with `with_if_not_exists`: destination already exists,
    // so the service returns 409 Conflict.
    let guard_options = BlockBlobClientUploadBlobFromUrlOptions::default().with_if_not_exists();
    match dest_client
        .block_blob_client()
        .upload_blob_from_url(source_client.url().as_str().into(), Some(guard_options))
        .await
    {
        Ok(_) => println!("Unexpected success - blob should already exist"),
        Err(err) => println!(
            "Copy blocked as expected (blob already exists): {}",
            err.http_status()
                .map(|s| s.to_string())
                .unwrap_or_else(|| err.to_string())
        ),
    }

    Ok(())
}
