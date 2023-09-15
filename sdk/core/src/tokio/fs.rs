use crate::{
    request::Body,
    seekable_stream::{SeekableStream, DEFAULT_BUFFER_SIZE},
    setters,
};
use futures::{task::Poll, Future};
use std::{cmp::min, io::SeekFrom, pin::Pin, sync::Arc, task::Context};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncSeekExt, Take},
    sync::Mutex,
};

#[derive(Debug)]
pub struct FileStreamBuilder {
    handle: File,
    /// Offset into the file to start reading from
    offset: Option<u64>,
    /// Amount of data to read from the file
    buffer_size: Option<usize>,
    /// How much to buffer in memory during streaming reads
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

    setters! {
        // #[doc = "Offset into the file to start reading from"]
        offset: u64 => Some(offset),
        // #[doc = "Amount of data to read from the file"]
        block_size: u64 => Some(block_size),
        // #[doc = "Amount of data to buffer in memory during streaming reads"]
        buffer_size: usize => Some(buffer_size),
    }

    pub async fn build(mut self) -> crate::Result<FileStream> {
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
            buffer_size,
            block_size,
            stream_size,
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
    /// Attempts to read from the underlying file handle.
    ///
    /// This first acquires a lock the handle, then reads from the handle.  The
    /// lock is released upon completion.  This is necessary due to the
    /// requirement of `Request` (the primary consumer of `FileStream`) must be
    /// `Clone`.
    async fn read(&mut self, slice: &mut [u8]) -> std::io::Result<usize> {
        let mut handle = self.handle.clone().lock_owned().await;
        handle.read(slice).await
    }

    /// Resets the number of bytes that will be read from this instance to the
    /// `stream_size`
    ///
    /// This is useful if you want to read the stream in multiple blocks
    pub async fn next_block(&mut self) -> crate::Result<()> {
        log::info!("setting limit to {}", self.block_size);
        let mut handle = self.handle.clone().lock_owned().await;
        {
            let inner = handle.get_mut();
            self.offset = inner.stream_position().await?;
        }
        handle.set_limit(self.block_size);
        Ok(())
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl SeekableStream for FileStream {
    /// Seek to the specified offset into the file and reset the number of bytes to read
    ///
    /// This is useful upon encountering an error to reset the stream to the last
    async fn reset(&mut self) -> crate::Result<()> {
        log::info!(
            "resetting stream to offset {} and limit to {}",
            self.offset,
            self.block_size
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
        log::info!(
            "stream len:  {} - {} ... {}",
            self.stream_size,
            self.offset,
            self.block_size
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
