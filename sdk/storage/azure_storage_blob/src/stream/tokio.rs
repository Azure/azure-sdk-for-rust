// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Tokio-based stream implementations.

use azure_core::{
    http::Body,
    stream::{SeekableStream, DEFAULT_BUFFER_SIZE},
};
use std::{
    future::Future,
    io::SeekFrom,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncSeekExt},
    sync::Mutex,
};

/// Builds a [`FileStream`] from a [`tokio::fs::File`].
#[derive(Debug)]
pub struct FileStreamBuilder {
    file: File,
    buffer_size: Option<usize>,
}

impl FileStreamBuilder {
    fn new(file: File) -> Self {
        Self {
            file,
            buffer_size: None,
        }
    }

    /// Sets the size of the buffer to use when reading from the stream.
    pub fn with_buffer_size(mut self, buffer_size: usize) -> Self {
        // Not many APIs I looked at use NonZeroUsize which is a bit unwieldy,
        // but they also don't often protect against this case either.
        debug_assert!(buffer_size > 0, "buffer_size must be greater than 0");

        self.buffer_size = Some(buffer_size);
        self
    }

    /// Builds a [`FileStream`].
    ///
    /// # Notes
    ///
    /// The [`SeekableStream::len()`] is the file size returned from [`Metadata::len()`](std::fs::Metadata)
    /// regardless of the initial position of the [`File`].
    pub async fn build(self) -> azure_core::Result<FileStream> {
        let file_size = self.file.metadata().await?.len();
        let buffer_size = self.buffer_size.unwrap_or(DEFAULT_BUFFER_SIZE);

        Ok(FileStream {
            handle: Arc::new(Mutex::new(self.file)),
            file_size,
            buffer_size,
        })
    }
}

/// A stream over a [`tokio::fs::File`] that implements [`SeekableStream`].
#[derive(Debug, Clone)]
pub struct FileStream {
    handle: Arc<Mutex<File>>,
    file_size: u64,
    buffer_size: usize,
}

impl FileStream {
    /// Creates a new [`FileStreamBuilder`].
    ///
    /// # Arguments
    ///
    /// * `handle` - An open [`tokio::fs::File`] to stream.
    ///
    /// # Notes
    ///
    /// `len()` is the file size returned from [`Metadata::len()`](std::fs::Metadata)
    /// regardless of the initial position of the [`File`].
    pub fn builder(file: File) -> FileStreamBuilder {
        FileStreamBuilder::new(file)
    }

    async fn read(&self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut handle = self.handle.lock().await;
        handle.read(buf).await
    }
}

impl From<FileStream> for Body {
    fn from(stream: FileStream) -> Self {
        Body::SeekableStream(Box::new(stream))
    }
}

#[async_trait::async_trait]
impl SeekableStream for FileStream {
    async fn reset(&mut self) -> azure_core::Result<()> {
        let mut handle = self.handle.lock().await;
        handle.seek(SeekFrom::Start(0)).await?;
        Ok(())
    }

    /// Gets the length of the underlying [`File`].
    ///
    /// # Notes
    ///
    /// `len()` is the file size returned from [`Metadata::len()`](std::fs::Metadata)
    /// regardless of the initial position of the [`File`].
    fn len(&self) -> Option<u64> {
        Some(self.file_size)
    }

    fn buffer_size(&self) -> usize {
        self.buffer_size
    }
}

impl futures::io::AsyncRead for FileStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        let fut = self.read(buf);
        futures::pin_mut!(fut);
        fut.poll(cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::AsyncReadExt;
    use std::path::Path;

    async fn open_this_file(buffer_size: Option<usize>) -> FileStream {
        // file!() returns a workspace-relative path; use a relative traversal
        // from the crate directory to the workspace root.
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join(file!());
        let file = File::open(&path).await.unwrap();
        let mut builder = FileStream::builder(file);
        if let Some(size) = buffer_size {
            builder = builder.with_buffer_size(size);
        }
        builder.build().await.unwrap()
    }

    #[tokio::test]
    async fn stream_large_chunks() {
        let mut stream = open_this_file(None).await;
        let expected_len: usize = stream.len().unwrap().try_into().unwrap();
        assert!(expected_len > 0);

        let mut buf = vec![0u8; expected_len];
        let n = stream.read_to_end(&mut buf).await.unwrap();
        assert_eq!(n, expected_len);
    }

    #[tokio::test]
    async fn stream_small_chunks() {
        const BUFFER_SIZE: usize = 8;

        let stream = open_this_file(Some(BUFFER_SIZE)).await;
        assert_eq!(stream.buffer_size(), BUFFER_SIZE);

        let expected_len: usize = stream.len().unwrap().try_into().unwrap();
        let mut total_read = 0;
        let mut buf = vec![0u8; BUFFER_SIZE];
        loop {
            let n = stream.read(&mut buf).await.unwrap();
            if n == 0 {
                break;
            }
            total_read += n;
        }
        assert_eq!(total_read, expected_len);
    }

    #[tokio::test]
    async fn reset() {
        let mut stream = open_this_file(None).await;
        let expected_len: usize = stream.len().unwrap().try_into().unwrap();

        // First full read.
        let mut buf1 = vec![0u8; expected_len];
        let n = stream.read(&mut buf1).await.unwrap();
        assert_eq!(n, expected_len);

        // Reset and read again.
        stream.reset().await.unwrap();
        let mut buf2 = vec![0u8; expected_len];
        let n = stream.read(&mut buf2).await.unwrap();
        assert_eq!(n, expected_len);
        assert_eq!(buf1, buf2);
    }
}
