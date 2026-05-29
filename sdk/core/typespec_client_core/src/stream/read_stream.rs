// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::SeekableStream;
use crate::error::{Error, ErrorKind};
use futures::{io::AsyncRead, lock::Mutex};
use std::{
    fmt,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

/// A [`SeekableStream`] adapter for any [`AsyncRead`] source.
///
/// Because the underlying reader is not necessarily seekable, [`reset`](SeekableStream::reset)
/// always returns an error. The reader is stored behind an `Arc<Mutex<R>>` so that
/// `ReadStream<R>` is always [`Clone`] regardless of whether `R` itself is — enabling
/// non-`Clone` readers to be wrapped without requiring `R: Clone`.
pub struct ReadStream<R> {
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
    pub fn new(reader: R, len: Option<u64>) -> Self {
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
    R: AsyncRead + Unpin + Send + Sync,
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

impl<R> AsyncRead for ReadStream<R>
where
    R: AsyncRead + Unpin,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        // try_lock always succeeds in practice: ReadStream has a single logical reader
        // at any given time (SharedStream ensures only one clone reads at once).
        match self.get_mut().reader.try_lock() {
            Some(mut guard) => Pin::new(&mut *guard).poll_read(cx, buf),
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
    use crate::stream::BytesStream;
    use futures::io::{AsyncRead, AsyncReadExt, Cursor};
    use std::task::{Context, Poll};

    /// A minimal [`AsyncRead`] that does not implement [`Clone`], used to verify
    /// that [`ReadStream`] works with non-`Clone` sources.
    struct NonCloneReader(Cursor<Vec<u8>>);

    impl Unpin for NonCloneReader {}

    impl AsyncRead for NonCloneReader {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut [u8],
        ) -> Poll<std::io::Result<usize>> {
            Pin::new(&mut self.0).poll_read(cx, buf)
        }
    }

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
    async fn wraps_non_clone_reader() {
        let data = b"non-clone source";
        let mut stream = ReadStream::new(
            NonCloneReader(Cursor::new(data.to_vec())),
            Some(data.len() as u64),
        );

        assert_eq!(stream.len(), Some(data.len() as u64));

        let mut buf = vec![0u8; data.len()];
        let n = stream.read(&mut buf).await.unwrap();
        assert_eq!(n, data.len());
        assert_eq!(&buf, data);
    }

    #[tokio::test]
    async fn wraps_bytes_stream() {
        let data = b"hello from BytesStream";
        let inner = BytesStream::new(data.as_slice());
        let mut stream = ReadStream::new(inner, Some(data.len() as u64));

        assert_eq!(stream.len(), Some(data.len() as u64));

        let mut buf = vec![0u8; data.len()];
        let n = stream.read(&mut buf).await.unwrap();
        assert_eq!(n, data.len());
        assert_eq!(&buf, data);
    }
}
