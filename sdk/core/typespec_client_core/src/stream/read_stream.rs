// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::SeekableStream;
use crate::error::{Error, ErrorKind};
use futures::io::AsyncRead;
use std::{
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

/// A [`SeekableStream`] adapter for any [`AsyncRead`] source.
///
/// Because the underlying reader is not necessarily seekable, [`reset`](SeekableStream::reset)
/// always returns an error.
#[derive(Clone)]
pub struct ReadStream<R> {
    reader: R,
    len: u64,
}

impl<R> ReadStream<R> {
    /// Creates a new `ReadStream` with the given reader and total length.
    pub fn new(reader: R, len: u64) -> Self {
        Self { reader, len }
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
    R: AsyncRead + Unpin + Send + Sync + Clone,
{
    async fn reset(&mut self) -> crate::Result<()> {
        Err(Error::with_message(
            ErrorKind::Io,
            "ReadStream does not support reset",
        ))
    }

    fn len(&self) -> u64 {
        self.len
    }
}

impl<R> AsyncRead for ReadStream<R>
where
    R: AsyncRead + Unpin,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.reader).poll_read(cx, buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::BytesStream;
    use futures::io::{AsyncReadExt, Cursor};

    #[tokio::test]
    async fn read_returns_data() {
        let data = b"hello world";
        let mut stream = ReadStream::new(Cursor::new(data.to_vec()), data.len() as u64);

        let mut buf = vec![0u8; data.len()];
        let n = stream.read(&mut buf).await.unwrap();
        assert_eq!(n, data.len());
        assert_eq!(&buf, data);
    }

    #[tokio::test]
    async fn reset_returns_error() {
        let mut stream = ReadStream::new(Cursor::new(vec![]), 0);
        let result = stream.reset().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn len_returns_total_length() {
        let stream = ReadStream::new(Cursor::new(vec![0u8; 42]), 42);
        assert_eq!(stream.len(), 42);
        assert!(!stream.is_empty());
    }

    #[tokio::test]
    async fn wraps_bytes_stream() {
        let data = b"hello from BytesStream";
        let inner = BytesStream::new(data.as_slice());
        let mut stream = ReadStream::new(inner, data.len() as u64);

        assert_eq!(stream.len(), data.len() as u64);

        let mut buf = vec![0u8; data.len()];
        let n = stream.read(&mut buf).await.unwrap();
        assert_eq!(n, data.len());
        assert_eq!(&buf, data);
    }
}
