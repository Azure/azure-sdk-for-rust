// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Page blob client example for Azure Blob Storage.
//!
//! Page blobs are optimized for random read/write workloads (e.g., VHD images).
//! All reads and writes must be aligned to 512-byte page boundaries. This sample
//! demonstrates:
//! 1. Create a page blob (512 bytes) with the "if not exists" guard.
//! 2. Upload a page of data using `HttpRange`.
//! 3. List the valid page ranges to confirm the write.
//! 4. Clear a page range to zero out the data.
//! 5. Resize the blob to a larger size.
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
//! cargo run --package azure_storage_blob --example page_blob_client
//! ```

use std::env;

use azure_core::http::RequestContent;
use azure_identity::DeveloperToolsCredential;
use azure_storage_blob::{
    models::{BlobClientGetPropertiesResultHeaders, HttpRange, PageBlobClientCreateOptions},
    BlobContainerClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account = env::var("AZURE_STORAGE_ACCOUNT_NAME")
        .expect("Set AZURE_STORAGE_ACCOUNT_NAME environment variable");

    let endpoint = format!("https://{}.blob.core.windows.net/", account);
    let container_name = "test-container-page-blob";

    let credential = DeveloperToolsCredential::new(None)?;
    let container_client =
        BlobContainerClient::new(&endpoint, container_name, Some(credential), None)?;

    println!("Creating container '{container_name}'...");
    container_client.create(None).await?;

    let blob_name = "page-blob.vhd";
    let blob_client = container_client.blob_client(blob_name);
    let page_blob_client = blob_client.page_blob_client();

    // Create with `with_if_not_exists` so a repeated run does not conflict.
    // Page blob sizes must be a multiple of 512.
    let initial_size: u64 = 512;
    let create_options = PageBlobClientCreateOptions::default().with_if_not_exists();
    page_blob_client
        .create(initial_size, Some(create_options))
        .await?;
    println!("Created page blob '{blob_name}' ({initial_size} bytes)");

    // Write 512 bytes of data to bytes 0-511.
    let page_data = vec![b'A'; 512];
    let range = HttpRange::new(0, 512);
    page_blob_client
        .upload_pages(RequestContent::from(page_data), 512, range, None)
        .await?;
    println!("Uploaded page at bytes 0-511");

    // List the valid (non-zero) page ranges.
    let page_ranges = page_blob_client.get_page_ranges(None).await?.into_model()?;
    let ranges = page_ranges.page_range.as_deref().unwrap_or(&[]);
    println!("Valid page ranges ({}):", ranges.len());
    for r in ranges {
        println!("  start={:?}, end={:?}", r.start, r.end);
    }

    // Clear the page range (zeroes out those bytes).
    page_blob_client
        .clear_pages(HttpRange::new(0, 512), None)
        .await?;
    println!("Cleared page range 0-511");

    // Verify the page range is gone after clearing.
    let page_ranges = page_blob_client.get_page_ranges(None).await?.into_model()?;
    let ranges = page_ranges.page_range.as_deref().unwrap_or(&[]);
    println!("Valid page ranges after clear: {}", ranges.len());

    // Resize the blob to 1024 bytes (must be a multiple of 512).
    page_blob_client.resize(1024, None).await?;
    println!("Resized blob to 1024 bytes");

    let properties = blob_client.get_properties(None).await?;
    let length = properties.content_length()?;
    println!("Blob content-length after resize: {:?}", length);

    container_client.delete(None).await?;
    println!("Deleted container '{container_name}'");

    Ok(())
}
