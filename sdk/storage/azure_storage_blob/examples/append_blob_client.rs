// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Append blob client example for Azure Blob Storage.
//!
//! Append blobs are optimized for append-only workloads such as logging and
//! audit trails. This sample demonstrates:
//! 1. Create an append blob with the "if not exists" guard.
//! 2. Append several blocks in a loop.
//! 3. Seal the blob so that no further appends are accepted.
//! 4. Download and print the final content.
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
//! cargo run --package azure_storage_blob --example append_blob_client
//! ```

use std::env;

use azure_core::http::RequestContent;
use azure_identity::DeveloperToolsCredential;
use azure_storage_blob::{models::AppendBlobClientCreateOptions, BlobContainerClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account = env::var("AZURE_STORAGE_ACCOUNT_NAME")
        .expect("Set AZURE_STORAGE_ACCOUNT_NAME environment variable");

    let endpoint = format!("https://{}.blob.core.windows.net/", account);
    let container_name = "test-container-append-blob";

    let credential = DeveloperToolsCredential::new(None)?;
    let container_client =
        BlobContainerClient::new(&endpoint, container_name, Some(credential), None)?;

    println!("Creating container '{container_name}'...");
    container_client.create(None).await?;

    let blob_name = "append-log.txt";
    let blob_client = container_client.blob_client(blob_name);
    let append_blob_client = blob_client.append_blob_client();

    // Create with `with_if_not_exists` so a repeated run does not conflict.
    let create_options = AppendBlobClientCreateOptions::default().with_if_not_exists();
    append_blob_client.create(Some(create_options)).await?;
    println!("Created append blob '{blob_name}'");

    // Append several lines, simulating an incrementally-written log.
    let log_lines = [
        "2000-05-11T00:00:01Z INFO  service started\n",
        "2000-05-11T00:00:02Z INFO  processing request\n",
        "2000-05-11T00:00:03Z INFO  request completed\n",
    ];

    for line in &log_lines {
        let data = line.as_bytes();
        append_blob_client
            .append_block(
                RequestContent::from(data.to_vec()),
                u64::try_from(data.len())?,
                None,
            )
            .await?;
        println!("Appended: {}", line.trim_end());
    }

    // Seal the blob to mark it as read-only (no further appends allowed).
    append_blob_client.seal(None).await?;
    println!("Sealed blob '{blob_name}' - no further appends are accepted");

    // Download and verify the assembled log content.
    let response = blob_client.download(None).await?;
    let data = response.body.collect().await?;
    println!("\nFull blob content:\n{}", String::from_utf8_lossy(&data));

    container_client.delete(None).await?;
    println!("Deleted container '{container_name}'");

    Ok(())
}
