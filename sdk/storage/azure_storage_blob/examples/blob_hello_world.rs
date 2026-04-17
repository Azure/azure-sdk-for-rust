// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Hello-world sample for Azure Blob Storage.
//!
//! This is the quickest path to uploading and downloading a blob:
//! 1. Create a [`BlobContainerClient`] authenticated with Microsoft Entra ID.
//! 2. Create a container.
//! 3. Upload a text blob.
//! 4. Download and print the blob content.
//! 5. Delete the container.
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
//! cargo run --package azure_storage_blob --example blob_hello_world
//! ```

use azure_core::http::RequestContent;
use azure_identity::DeveloperToolsCredential;
use azure_storage_blob::BlobContainerClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account = std::env::var("AZURE_STORAGE_ACCOUNT_NAME")
        .expect("Set AZURE_STORAGE_ACCOUNT_NAME environment variable");

    let endpoint = format!("https://{}.blob.core.windows.net/", account);
    let container_name = "hello-world-container";
    let blob_name = "hello_world.txt";

    let credential = DeveloperToolsCredential::new(None)?;
    let container_client =
        BlobContainerClient::new(&endpoint, container_name, Some(credential), None)?;

    // Create the container.
    container_client.create(None).await?;
    println!("Created container '{container_name}'");

    // Get a blob client and upload text content.
    let blob_client = container_client.blob_client(blob_name);
    let content = b"Hello, Azure Blob Storage!";
    blob_client
        .upload(RequestContent::from(content.to_vec()), None)
        .await?;
    println!("Uploaded blob '{blob_name}'");

    // Download the blob and print its content.
    let response = blob_client.download(None).await?;
    let data = response.body.collect().await?;
    println!("Downloaded: {}", String::from_utf8_lossy(&data));

    // Delete the container (also deletes all blobs inside it).
    container_client.delete(None).await?;
    println!("Deleted container '{container_name}'");

    Ok(())
}
