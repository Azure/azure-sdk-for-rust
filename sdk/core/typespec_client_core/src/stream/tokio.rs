// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::SeekableStream;
use crate::error::{Error, ErrorKind};
use futures::lock::Mutex;
use std::{
    fmt,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

/// A [`SeekableStream`] adapter for any [`tokio::io::AsyncRead`] source.
///
/// Because the underlying reader is not necessarily seekable, [`reset`](SeekableStream::reset)
/// always returns an error. The reader is stored behind an `Arc<Mutex<R>>` so that
/// `ReadStream<R>` is always [`Clone`] regardless of whether `R` itself is — enabling
/// non-`Clone` readers (e.g. `tokio_util::io::StreamReader`) to be wrapped without
/// requiring `R: Clone`.
pub(crate) struct ReadStream<R> {
    reader: Arc<Mutex<R>>,
    len: Option<u64>,
}

impl<R> Clone for ReadStream<R> {
    fn clone(&self) -> Self {
        Self {
            reader: Arc::clone(&self.reader),
            len: self.len,
        }
    }
}

impl<R> ReadStream<R> {
    /// Creates a new `ReadStream` with the given reader and total length.
    pub(crate) fn new(reader: R, len: Option<u64>) -> Self {
        Self {
            reader: Arc::new(Mutex::new(reader)),
            len,
        }
    }
}

impl<R> fmt::Debug for ReadStream<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReadStream").finish_non_exhaustive()
    }
}

#[async_trait::async_trait]
impl<R> SeekableStream for ReadStream<R>
where
    R: tokio::io::AsyncRead + Unpin + Send + Sync,
{
    async fn reset(&mut self) -> crate::Result<()> {
        Err(Error::with_message(
            ErrorKind::Io,
            "ReadStream does not support reset",
        ))
    }

    fn len(&self) -> Option<u64> {
        self.len
    }
}

impl<R> futures::io::AsyncRead for ReadStream<R>
where
    R: tokio::io::AsyncRead + Unpin,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        // try_lock always succeeds in practice: ReadStream has a single logical reader
        // at any given time (SharedStream ensures only one clone reads at once).
        match self.get_mut().reader.try_lock() {
            Some(mut guard) => {
                let mut read_buf = tokio::io::ReadBuf::new(buf);
                match Pin::new(&mut *guard).poll_read(cx, &mut read_buf) {
                    Poll::Ready(Ok(())) => Poll::Ready(Ok(read_buf.filled().len())),
                    Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
                    Poll::Pending => Poll::Pending,
                }
            }
            None => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::io::AsyncReadExt;
    use std::io::Cursor;

    #[tokio::test]
    async fn read_returns_data() {
        let data = b"hello world";
        let mut stream = ReadStream::new(Cursor::new(data.to_vec()), Some(data.len() as u64));

        let mut buf = vec![0u8; data.len()];
        let n = stream.read(&mut buf).await.unwrap();
        assert_eq!(n, data.len());
        assert_eq!(&buf, data);
    }

    #[tokio::test]
    async fn reset_returns_error() {
        let mut stream = ReadStream::new(Cursor::new(vec![]), None);
        let result = stream.reset().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn len_returns_total_length() {
        let stream = ReadStream::new(Cursor::new(vec![0u8; 42]), Some(42));
        assert_eq!(stream.len(), Some(42));
        assert!(matches!(stream.is_empty(), Some(b) if !b));
    }

    #[tokio::test]
    async fn read_via_tokio_read() {
        let data = b"hello from tokio";
        let mut stream = ReadStream::new(Cursor::new(data.to_vec()), Some(data.len() as u64));

        let mut buf = vec![0u8; data.len()];
        // Use futures AsyncRead via the SeekableStream blanket (poll_read bridge)
        let n = stream.read(&mut buf).await.unwrap();
        assert_eq!(n, data.len());
        assert_eq!(&buf, data);
    }
}
