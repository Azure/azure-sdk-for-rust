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
//!   <file-path> \
//!   [--buffer-size <bytes>] \
//!   [--parallel <count>] \
//!   [--partition-size <bytes>]
//! ```

use azure_core::{
    http::{Body, NoFormat, RequestContent},
    stream::{SeekableStream, DEFAULT_BUFFER_SIZE},
    Bytes,
};
use azure_identity::AzureCliCredential;
use azure_storage_blob::{models::BlockBlobClientUploadOptions, BlobContainerClient};
use clap::Parser;
use futures::{io::AsyncRead, task::Poll};
use std::{io::SeekFrom, num::NonZeroUsize, path::PathBuf, pin::Pin, sync::Arc, task::Context};
use tokio::{
    fs::{read, File},
    io::{AsyncSeekExt, ReadBuf},
    sync::{Mutex, OwnedMutexGuard},
};
use tracing::debug;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let endpoint = format!("https://{}.blob.core.windows.net", args.account_name);

    let credential = AzureCliCredential::new(None)?;
    let container_client =
        BlobContainerClient::new(&endpoint, &args.container_name, Some(credential), None)?;

    if !container_client.exists().await? {
        container_client.create(None).await?;
    }

    let blob_client = container_client.blob_client(&args.blob_name);
    let expected_content = read(&args.file_path).await?;
    let file = File::open(&args.file_path).await?;
    let stream = FileStream::new(file, args.buffer_size).await?;
    let content: RequestContent<Bytes, NoFormat> =
        Body::from(Box::new(stream) as Box<dyn SeekableStream>).into();
    let upload_options = if args.parallel.is_some() || args.partition_size.is_some() {
        Some(BlockBlobClientUploadOptions {
            parallel: args.parallel,
            partition_size: args.partition_size,
            ..Default::default()
        })
    } else {
        None
    };

    blob_client.upload(content, upload_options).await?;

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

pub struct FileStream {
    handle: Arc<Mutex<File>>,
    pub stream_size: u64,
    buffer_size: usize,
    read_state: Mutex<ReadState>,
}

impl std::fmt::Debug for FileStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FileStream")
            .field("stream_size", &self.stream_size)
            .field("buffer_size", &self.buffer_size)
            .finish()
    }
}

impl Clone for FileStream {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
            stream_size: self.stream_size,
            buffer_size: self.buffer_size,
            read_state: Mutex::new(ReadState::default()),
        }
    }
}

impl FileStream {
    /// Creates a new [`FileStream`] with explicit stream settings.
    pub async fn new(handle: File, buffer_size: usize) -> azure_core::Result<Self> {
        let stream_size = handle.metadata().await?.len();
        let handle = Arc::new(Mutex::new(handle));

        Ok(Self {
            handle,
            stream_size,
            buffer_size,
            read_state: Mutex::new(ReadState::default()),
        })
    }
}

#[async_trait::async_trait]
impl SeekableStream for FileStream {
    async fn reset(&mut self) -> azure_core::Result<()> {
        debug!("resetting stream to beginning");
        *self.read_state.lock().await = ReadState::Idle;
        let mut handle = self.handle.clone().lock_owned().await;
        handle.seek(SeekFrom::Start(0)).await?;
        Ok(())
    }

    fn len(&self) -> usize {
        debug!("stream len: {}", self.stream_size);
        self.stream_size as usize
    }

    fn buffer_size(&self) -> usize {
        self.buffer_size
    }
}

type FileLockFuture = Pin<Box<dyn std::future::Future<Output = OwnedMutexGuard<File>> + Send>>;

#[derive(Default)]
enum ReadState {
    #[default]
    Idle,
    Locking(FileLockFuture),
    Locked(OwnedMutexGuard<File>),
}

impl AsyncRead for FileStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        slice: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        let this = self.get_mut();
        let mut state = this
            .read_state
            .try_lock()
            .expect("read_state mutex should not be locked across poll calls");

        loop {
            match &mut *state {
                ReadState::Idle => {
                    *state = ReadState::Locking(Box::pin(this.handle.clone().lock_owned()));
                }
                ReadState::Locking(lock_future) => {
                    let poll_result = std::future::Future::poll(Pin::as_mut(lock_future), cx);
                    match poll_result {
                        Poll::Ready(guard) => *state = ReadState::Locked(guard),
                        Poll::Pending => return Poll::Pending,
                    }
                }
                ReadState::Locked(guard) => {
                    let mut read_buf = ReadBuf::new(slice);

                    return match tokio::io::AsyncRead::poll_read(
                        Pin::new(&mut **guard),
                        cx,
                        &mut read_buf,
                    ) {
                        Poll::Ready(Ok(())) => Poll::Ready(Ok(read_buf.filled().len())),
                        Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
                        Poll::Pending => Poll::Pending,
                    };
                }
            }
        }
    }
}

#[derive(Debug, Parser)]
struct Args {
    account_name: String,
    container_name: String,
    blob_name: String,
    file_path: PathBuf,
    /// Control how many bytes are buffered in memory for each streaming read.
    #[arg(long, value_name = "BYTES", default_value_t = DEFAULT_BUFFER_SIZE)]
    buffer_size: usize,
    /// Number of concurrent network transfers to keep in flight during the upload.
    ///
    /// Higher values can improve throughput at the cost of more simultaneous requests.
    /// When omitted, the client chooses a default.
    #[arg(long, value_name = "COUNT")]
    parallel: Option<NonZeroUsize>,
    /// Size, in bytes, of each partition when the upload is split into multiple requests.
    ///
    /// Larger values reduce the number of requests, while smaller values reduce memory used
    /// per in-flight request. When omitted, the client chooses a default.
    #[arg(long, value_name = "BYTES")]
    partition_size: Option<NonZeroUsize>,
}
