// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! This example uploads a local file to Azure Blob Storage.
//!
//! For small files (at most [`DEFAULT_BUFFER_SIZE`] bytes) the file is read entirely into memory
//! with [`std::fs::read`] and uploaded as [`Bytes`]. For larger files a [`tokio::fs::File`] is
//! wrapped in a [`FileStream`] and streamed to the service, avoiding loading the whole file into
//! memory.
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
//!     <BLOB_NAME> \
//!     <FILE_PATH> \
//!     [--buffer-size <BYTES>] \
//!     [--parallel <COUNT>] \
//!     [--partition-size <BYTES>]
//! ```
//!
//! The `<ACCOUNT_NAME>` flag can also be provided via the `AZURE_STORAGE_ACCOUNT_NAME`
//! environment variable.

use azure_core::{http::Body, stream::DEFAULT_BUFFER_SIZE};
use azure_identity::AzureCliCredential;
use azure_storage_blob::{models::BlobClientUploadOptions, stream::tokio::FileStream, BlobClient};
use clap::Parser;
use std::{num::NonZero, path::PathBuf};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let endpoint = format!("https://{}.blob.core.windows.net", args.account_name);
    let credential = AzureCliCredential::new(None)?;
    let client = BlobClient::new(
        &endpoint,
        &args.container_name,
        &args.blob_name,
        Some(credential),
        None,
    )?;

    let file_size = std::fs::metadata(&args.file_path)?.len();

    let body: Body = if file_size <= DEFAULT_BUFFER_SIZE as u64 {
        // Small file: read entirely into memory.
        let bytes = std::fs::read(&args.file_path)?;
        println!("Uploading {} bytes from memory...", bytes.len());
        bytes.into()
    } else {
        // Large file: stream from disk.
        let file = tokio::fs::File::open(&args.file_path).await?;
        let mut builder = FileStream::builder(file);
        if let Some(buffer_size) = args.buffer_size {
            builder = builder.with_buffer_size(buffer_size);
        }
        let stream = builder.build().await?;
        println!("Streaming {} bytes from disk...", file_size);
        stream.into()
    };

    let options = if args.parallel.is_some() || args.partition_size.is_some() {
        Some(BlobClientUploadOptions {
            parallel: args.parallel,
            partition_size: args.partition_size,
            ..Default::default()
        })
    } else {
        None
    };
    client.upload(body.into(), options).await?;
    println!(
        "Uploaded {} bytes from {}",
        file_size,
        args.file_path.display()
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

    /// Blob name.
    blob_name: String,

    /// Path to the local file to upload.
    file_path: PathBuf,

    /// Bytes to buffer in memory for each streaming read.
    #[arg(long, value_name = "BYTES")]
    buffer_size: Option<usize>,

    /// Number of concurrent network transfers during the upload.
    ///
    /// Higher values can improve throughput at the cost of more simultaneous requests.
    /// When omitted, the client chooses a default.
    #[arg(long, value_name = "COUNT")]
    parallel: Option<NonZero<usize>>,

    /// Size, in bytes, of each partition when the upload is split into multiple requests.
    ///
    /// Larger values reduce the number of requests, while smaller values reduce memory
    /// used per in-flight request. When omitted, the client chooses a default.
    #[arg(long, value_name = "BYTES")]
    partition_size: Option<NonZero<u64>>,
}
