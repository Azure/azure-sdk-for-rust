// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{SeekableStream, DEFAULT_BUFFER_SIZE};
use crate::http::Body;
use futures::io::{AsyncRead, AsyncSeek};
use std::{
    fmt, io,
    pin::Pin,
    task::{Context, Poll},
};

/// A buffered stream adapter for reading files asynchronously.
///
/// `FileStream<T>` wraps an inner reader `T` and provides internal buffering
/// (like [`std::io::BufReader`]) along with implementations of
/// [`AsyncRead`] and [`AsyncSeek`].
///
/// Use [`FileStream::new`] to create a stream with the default buffer size
/// ([`DEFAULT_BUFFER_SIZE`]), or chain [`FileStream::with_buffer_size`] to
/// customize the buffer capacity:
///
/// ```ignore
/// let stream = FileStream::new(reader).with_buffer_size(8192);
/// ```
pub struct FileStream<T>
where
    T: AsyncRead + AsyncSeek,
{
    stream: T,
    buf: Box<[u8]>,
    pos: usize,
    filled: usize,
}

impl<T: AsyncRead + AsyncSeek> FileStream<T> {
    /// Creates a new `FileStream` wrapping `stream` with the default buffer size.
    pub fn new(stream: T) -> Self {
        Self::with_buffer_size_inner(stream, DEFAULT_BUFFER_SIZE)
    }

    /// Returns a `FileStream` with the specified internal buffer size.
    pub fn with_buffer_size(self, size: usize) -> Self {
        Self::with_buffer_size_inner(self.stream, size)
    }

    fn with_buffer_size_inner(stream: T, size: usize) -> Self {
        Self {
            stream,
            buf: vec![0; size].into_boxed_slice(),
            pos: 0,
            filled: 0,
        }
    }

    /// Discards the buffered data so the next read comes from the inner stream.
    fn discard_buffer(&mut self) {
        self.pos = 0;
        self.filled = 0;
    }
}

impl<T: SeekableStream + AsyncSeek + Clone + 'static> From<FileStream<T>> for Body {
    fn from(stream: FileStream<T>) -> Self {
        Body::SeekableStream(Box::new(stream))
    }
}

impl<T: AsyncRead + AsyncSeek> fmt::Debug for FileStream<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileStream").finish_non_exhaustive()
    }
}

/// # Warning
///
/// Clones share the underlying stream. This type should only be used with read-only streams.
impl<T: AsyncRead + AsyncSeek + Clone> Clone for FileStream<T> {
    fn clone(&self) -> Self {
        Self {
            stream: self.stream.clone(),
            buf: self.buf.clone(),
            pos: self.pos,
            filled: self.filled,
        }
    }
}

impl<T: AsyncRead + AsyncSeek + Unpin> AsyncRead for FileStream<T> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let this = self.get_mut();

        // If there's buffered data, serve from it first.
        if this.pos < this.filled {
            let available = &this.buf[this.pos..this.filled];
            let n = std::cmp::min(available.len(), buf.len());
            buf[..n].copy_from_slice(&available[..n]);
            this.pos += n;
            return Poll::Ready(Ok(n));
        }

        // Buffer is empty. If the caller's buffer is at least as large as ours,
        // bypass the internal buffer and read directly.
        if buf.len() >= this.buf.len() {
            return Pin::new(&mut this.stream).poll_read(cx, buf);
        }

        // Otherwise refill our buffer and then copy.
        this.pos = 0;
        this.filled = 0;
        match Pin::new(&mut this.stream).poll_read(cx, &mut this.buf) {
            Poll::Ready(Ok(n)) => {
                this.filled = n;
                let to_copy = std::cmp::min(n, buf.len());
                buf[..to_copy].copy_from_slice(&this.buf[..to_copy]);
                this.pos = to_copy;
                Poll::Ready(Ok(to_copy))
            }
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<T: AsyncSeek + AsyncRead + Unpin> AsyncSeek for FileStream<T> {
    fn poll_seek(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        pos: io::SeekFrom,
    ) -> Poll<io::Result<u64>> {
        let this = self.get_mut();
        this.discard_buffer();
        Pin::new(&mut this.stream).poll_seek(cx, pos)
    }
}

#[async_trait::async_trait]
impl<T: SeekableStream + AsyncSeek + Clone> SeekableStream for FileStream<T> {
    async fn reset(&mut self) -> crate::Result<()> {
        self.discard_buffer();
        self.stream.reset().await
    }

    async fn len(&self) -> usize {
        self.stream.len().await
    }

    fn buffer_size(&self) -> usize {
        self.buf.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::BytesStream;
    use futures::io::{AsyncReadExt as _, AsyncSeekExt as _, Cursor};

    #[tokio::test]
    async fn read_all_bytes() {
        let data = b"hello world";
        let cursor = Cursor::new(data.to_vec());
        let mut stream = FileStream::new(cursor);

        let mut buf = vec![0; 11];
        let n = stream.read(&mut buf).await.unwrap();
        assert_eq!(n, 11);
        assert_eq!(&buf, data);
    }

    #[tokio::test]
    async fn read_one_byte_at_a_time() {
        let data = b"hello world";
        let cursor = Cursor::new(data.to_vec());
        let mut stream = FileStream::new(cursor);

        for &expected in data {
            let mut buf = [0; 1];
            let n = stream.read(&mut buf).await.unwrap();
            assert_eq!(n, 1);
            assert_eq!(buf[0], expected);
        }
    }

    #[tokio::test]
    async fn with_buffer_size_changes_buffer() {
        let inner = BytesStream::new(b"hello".to_vec());
        let stream = FileStream::new(inner).with_buffer_size(128);
        assert_eq!(stream.buffer_size(), 128);
    }

    #[tokio::test]
    async fn seek_invalidates_buffer_and_rereads() {
        let data = b"hello world";
        let cursor = Cursor::new(data.to_vec());
        let mut stream = FileStream::new(cursor);

        // Read "hello".
        let mut buf = vec![0; 5];
        stream.read_exact(&mut buf).await.unwrap();
        assert_eq!(&buf, b"hello");

        // Seek back to start.
        let pos = stream.seek(io::SeekFrom::Start(0)).await.unwrap();
        assert_eq!(pos, 0);

        // Re-read from the start.
        let mut buf2 = vec![0; 5];
        stream.read_exact(&mut buf2).await.unwrap();
        assert_eq!(&buf2, b"hello");
    }

    #[tokio::test]
    async fn reset_seeks_to_start() {
        let inner = BytesStream::new(b"hello".to_vec());
        let mut stream = FileStream::new(inner);

        let mut buf = vec![0; 5];
        stream.read_exact(&mut buf).await.unwrap();
        assert_eq!(&buf, b"hello");

        stream.reset().await.unwrap();
        let mut buf2 = vec![0; 5];
        stream.read_exact(&mut buf2).await.unwrap();
        assert_eq!(&buf2, b"hello");
    }

    #[tokio::test]
    async fn len_delegates_to_inner() {
        let inner = BytesStream::new(b"hello world".to_vec());
        let stream = FileStream::new(inner);
        assert_eq!(stream.len().await, 11);
    }

    #[tokio::test]
    async fn large_read_bypasses_buffer() {
        let data = vec![0xAB; DEFAULT_BUFFER_SIZE + 100];
        let cursor = Cursor::new(data.clone());
        let mut stream = FileStream::new(cursor);

        let mut buf = vec![0; DEFAULT_BUFFER_SIZE + 100];
        stream.read_exact(&mut buf).await.unwrap();
        assert_eq!(buf, data);
    }
}
