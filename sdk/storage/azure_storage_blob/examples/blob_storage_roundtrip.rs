// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! This example demonstrates uploading data from an arbitrary `AsyncRead`, in this case an Azure
//! Storage download stream.
//!
//! Please note that the recommended mechanism to copy data from a web resource to Azure Blob
//! Storage is to use [`BlockBlobClient::stage_block_from_url`]. This example just demonstrates
//! library functionality when working with existing network input.
//!
//! # Prerequisites
//!
//! - Authenticate using Azure CLI: `az login`
//!
//! # Usage
//!
//! ```bash
//! az login
//! cargo run --package azure_storage_blob --example blob_storage_upload_file -- \
//!     <ACCOUNT_NAME> \
//!     <CONTAINER_NAME> \
//!     <SOURCE_BLOB_NAME> \
//!     <DESTINATION_BLOB_NAME>
//! ```
//!
//! The `<ACCOUNT_NAME>` flag can also be provided via the `AZURE_STORAGE_ACCOUNT_NAME`
//! environment variable.

use azure_identity::AzureCliCredential;
use azure_storage_blob::{AsyncReadLenExt, BlobContainerClient, StorageUploadBody};
use clap::Parser;
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let endpoint = format!("https://{}.blob.core.windows.net", args.account_name);
    let credential = AzureCliCredential::new(None)?;
    let container =
        BlobContainerClient::new(&endpoint, &args.container_name, Some(credential), None)?;

    let data = "Hello, World!".as_bytes().to_vec();

    // Setup some sample data to work with
    container
        .blob_client(&args.src_blob_name)
        .upload(data.clone().into(), None)
        .await?;

    // Clients for this transfer
    let src_blob = container.blob_client(&args.src_blob_name);
    let dst_blob = container.blob_client(&args.dst_blob_name);

    // Obtain the download stream
    let download_stream = src_blob
        .download(None)
        .await?
        .body
        .map_err(std::io::Error::other)
        .into_async_read();

    // Upload the contents of the download stream
    dst_blob
        .upload(
            StorageUploadBody::AsyncRead(Box::new(
                download_stream.with_len_hint(Some(data.len() as u64)),
            )),
            None,
        )
        .await?;

    assert_eq!(
        src_blob.download(None).await?.body.collect().await?,
        dst_blob.download(None).await?.body.collect().await?,
    );

    Ok(())
}

#[derive(Debug, Parser)]
struct Args {
    /// Azure Storage account name.
    ///
    /// Can also be set via the `AZURE_STORAGE_ACCOUNT_NAME` environment variable.
    #[arg(env = "AZURE_STORAGE_ACCOUNT_NAME")]
    account_name: String,

    /// Blob container name.
    container_name: String,

    /// Source blob name.
    src_blob_name: String,

    /// Destination blob name.
    dst_blob_name: String,
}
