// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! This example uploads a local file through a seekable stream.
//!
//! # Prerequisites
//!
//! - Authenticate using Azure CLI: `az login`
//!
//! # Usage
//!
//! ```bash
//! az login
//! cargo run --package azure_storage_blob --example blob_upload_seekable_stream -- \
//!   <account-name> \
//!   <container-name> \
//!   <blob-name> \
//!   <file-path>
//! ```

use azure_core::stream::{SeekableStream, DEFAULT_BUFFER_SIZE};
use azure_identity::AzureCliCredential;
use azure_storage_blob::BlobContainerClient;
use futures::{task::Poll, Future};
use std::{
    env, ffi::OsString, fmt, io, io::SeekFrom, path::Path, path::PathBuf, sync::Arc, task::Context,
};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncSeekExt},
    sync::Mutex,
};

#[derive(Clone)]
struct FileStream {
    handle: Arc<Mutex<File>>,
    len: usize,
    buffer_size: usize,
}

impl FileStream {
    async fn open(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let file = File::open(path).await?;
        let len = file.metadata().await?.len() as usize;

        Ok(Self {
            handle: Arc::new(Mutex::new(file)),
            len,
            buffer_size: DEFAULT_BUFFER_SIZE,
        })
    }

    async fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut handle = self.handle.lock().await;
        handle.read(buf).await
    }
}

impl fmt::Debug for FileStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileStream")
            .field("len", &self.len)
            .field("buffer_size", &self.buffer_size)
            .finish()
    }
}

#[async_trait::async_trait]
impl SeekableStream for FileStream {
    async fn reset(&mut self) -> azure_core::Result<()> {
        let mut handle = self.handle.lock().await;
        handle.seek(SeekFrom::Start(0)).await?;
        Ok(())
    }

    fn len(&self) -> usize {
        self.len
    }

    fn buffer_size(&self) -> usize {
        self.buffer_size
    }
}

impl futures::io::AsyncRead for FileStream {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        std::pin::pin!(self.read(buf)).poll(cx)
    }
}

struct ExampleArgs {
    account_name: String,
    container_name: String,
    blob_name: String,
    file_path: PathBuf,
}

impl ExampleArgs {
    fn parse() -> io::Result<Self> {
        let mut args = env::args_os();
        let program = args
            .next()
            .unwrap_or_else(|| OsString::from("blob_upload_seekable_stream"));

        let usage = || {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "usage: {} <account-name> <container-name> <blob-name> <file-path>",
                    Path::new(&program).display()
                ),
            )
        };

        let Some(account_name) = args.next() else {
            return Err(usage());
        };
        let Some(container_name) = args.next() else {
            return Err(usage());
        };
        let Some(blob_name) = args.next() else {
            return Err(usage());
        };
        let Some(file_path) = args.next() else {
            return Err(usage());
        };

        if args.next().is_some() {
            return Err(usage());
        }

        Ok(Self {
            account_name: account_name.into_string().map_err(|value| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("account name must be valid UTF-8: {:?}", value),
                )
            })?,
            container_name: container_name.into_string().map_err(|value| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("container name must be valid UTF-8: {:?}", value),
                )
            })?,
            blob_name: blob_name.into_string().map_err(|value| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("blob name must be valid UTF-8: {:?}", value),
                )
            })?,
            file_path: file_path.into(),
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ExampleArgs::parse()?;

    let endpoint = format!("https://{}.blob.core.windows.net", args.account_name);

    let credential = AzureCliCredential::new(None)?;
    let container_client =
        BlobContainerClient::new(&endpoint, &args.container_name, Some(credential), None)?;

    if !container_client.exists().await? {
        container_client.create(None).await?;
    }

    let blob_client = container_client.blob_client(&args.blob_name);
    let expected_content = tokio::fs::read(&args.file_path).await?;
    let stream = FileStream::open(&args.file_path).await?;

    blob_client.upload_stream(stream, None).await?;

    let response = blob_client.download(None).await?;
    let (_, _, body) = response.deconstruct();
    let content = body.collect().await?;
    assert_eq!(content, expected_content);

    println!(
        "Uploaded and verified {} bytes from {}",
        content.len(),
        args.file_path.display()
    );

    Ok(())
}
