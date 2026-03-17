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

use azure_core::{
    http::{Body, NoFormat, RequestContent},
    stream::{SeekableStream, DEFAULT_BUFFER_SIZE},
    Bytes,
};
use azure_identity::AzureCliCredential;
use azure_storage_blob::BlobContainerClient;
use futures::{task::Poll, Future};
use std::{
    cmp::min, env, ffi::OsString, io, io::SeekFrom, path::Path, path::PathBuf, pin::Pin, sync::Arc,
    task::Context,
};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncSeekExt, Take},
    sync::Mutex,
};
use tracing::debug;

/// Builds a [`FileStream`].
#[derive(Debug)]
pub struct FileStreamBuilder {
    handle: File,
    offset: Option<u64>,
    buffer_size: Option<usize>,
    block_size: Option<u64>,
}

impl FileStreamBuilder {
    pub fn new(handle: File) -> Self {
        Self {
            handle,
            offset: None,
            buffer_size: None,
            block_size: None,
        }
    }

    /// Offset into the file to start reading from.
    pub fn offset(self, offset: u64) -> Self {
        Self {
            offset: Some(offset),
            ..self
        }
    }

    /// Amount of data to read from the file.
    pub fn block_size(self, block_size: u64) -> Self {
        Self {
            block_size: Some(block_size),
            ..self
        }
    }

    /// Amount of data to buffer in memory during streaming reads.
    pub fn buffer_size(self, buffer_size: usize) -> Self {
        Self {
            buffer_size: Some(buffer_size),
            ..self
        }
    }

    /// Build a [`FileStream`] from this builder.
    pub async fn build(mut self) -> azure_core::Result<FileStream> {
        let stream_size = self.handle.metadata().await?.len();
        let buffer_size = self.buffer_size.unwrap_or(DEFAULT_BUFFER_SIZE);

        let offset = if let Some(offset) = self.offset {
            self.handle.seek(SeekFrom::Start(offset)).await?;
            offset
        } else {
            0
        };

        let block_size = if let Some(block_size) = self.block_size {
            block_size
        } else {
            stream_size - offset
        };

        let handle = Arc::new(Mutex::new(self.handle.take(block_size)));

        Ok(FileStream {
            handle,
            stream_size,
            block_size,
            buffer_size,
            offset,
        })
    }
}

#[derive(Debug, Clone)]
#[pin_project::pin_project]
pub struct FileStream {
    #[pin]
    handle: Arc<Mutex<Take<File>>>,
    pub stream_size: u64,
    pub block_size: u64,
    buffer_size: usize,
    pub offset: u64,
}

impl FileStream {
    async fn read(&mut self, slice: &mut [u8]) -> std::io::Result<usize> {
        let mut handle = self.handle.clone().lock_owned().await;
        handle.read(slice).await
    }

    /// Resets the remaining bytes in this stream window to the configured block size.
    pub async fn next_block(&mut self) -> azure_core::Result<()> {
        debug!("setting limit to {}", self.block_size);
        let mut handle = self.handle.clone().lock_owned().await;
        {
            let inner = handle.get_mut();
            self.offset = inner.stream_position().await?;
        }
        handle.set_limit(self.block_size);
        Ok(())
    }
}

#[async_trait::async_trait]
impl SeekableStream for FileStream {
    async fn reset(&mut self) -> azure_core::Result<()> {
        debug!(
            "resetting stream to offset {} and limit to {}",
            self.offset, self.block_size
        );
        let mut handle = self.handle.clone().lock_owned().await;
        {
            let inner = handle.get_mut();
            inner.seek(SeekFrom::Start(self.offset)).await?;
        }
        handle.set_limit(self.block_size);
        Ok(())
    }

    fn len(&self) -> usize {
        debug!(
            "stream len: {} - {} ... {}",
            self.stream_size, self.offset, self.block_size
        );
        min(self.stream_size - self.offset, self.block_size) as usize
    }

    fn buffer_size(&self) -> usize {
        self.buffer_size
    }
}

impl futures::io::AsyncRead for FileStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        slice: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        std::pin::pin!(self.read(slice)).poll(cx)
    }
}

impl From<&FileStream> for Body {
    fn from(stream: &FileStream) -> Self {
        Body::SeekableStream(Box::new(stream.clone()))
    }
}

impl From<FileStream> for Body {
    fn from(stream: FileStream) -> Self {
        Body::SeekableStream(Box::new(stream))
    }
}

impl<T, F> From<&FileStream> for RequestContent<T, F> {
    fn from(stream: &FileStream) -> Self {
        Body::from(stream).into()
    }
}

impl<T, F> From<FileStream> for RequestContent<T, F> {
    fn from(stream: FileStream) -> Self {
        Body::from(stream).into()
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
    let file = File::open(&args.file_path).await?;
    let stream = FileStreamBuilder::new(file)
        .buffer_size(512usize)
        .build()
        .await?;
    let content: RequestContent<Bytes, NoFormat> = stream.into();

    blob_client.upload(content, None).await?;

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
