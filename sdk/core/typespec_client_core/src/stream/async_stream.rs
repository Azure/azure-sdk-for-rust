// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::stream::SeekableStream;
use futures::{AsyncRead, AsyncSeek, AsyncSeekExt as _};
use std::{
    fmt, io,
    pin::Pin,
    task::{Context, Poll},
};

/// Implements [`SeekableStream`] for any type that implements both [`AsyncRead`] and [`AsyncSeek`].
pub struct AsyncStream<'a, T> {
    stream: &'a mut T,
    len: usize,
}

impl<'a, T: AsyncRead + AsyncSeek + Unpin> AsyncStream<'a, T> {
    /// Try to create an `AsyncStream` from `stream`.
    ///
    /// The length will need to be read and may fail.
    pub async fn try_from(stream: &'a mut T) -> io::Result<Self> {
        let len = stream.seek(io::SeekFrom::End(0)).await? as usize;
        stream.seek(io::SeekFrom::Start(0)).await?;
        Ok(Self { stream, len })
    }
}

impl<T> fmt::Debug for AsyncStream<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsyncStream").finish_non_exhaustive()
    }
}

#[async_trait::async_trait]
impl<T: AsyncRead + AsyncSeek + Send + Sync + Unpin> SeekableStream for AsyncStream<'_, T> {
    async fn reset(&mut self) -> crate::Result<()> {
        self.stream.seek(io::SeekFrom::Start(0)).await?;
        Ok(())
    }

    async fn len(&self) -> usize {
        self.len
    }
}

impl<T: AsyncRead + Unpin> AsyncRead for AsyncStream<'_, T> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let stream = &mut *self.get_mut().stream;
        Pin::new(stream).poll_read(cx, buf)
    }
}

impl<T: AsyncSeek + Unpin> AsyncSeek for AsyncStream<'_, T> {
    fn poll_seek(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        pos: io::SeekFrom,
    ) -> Poll<io::Result<u64>> {
        let stream = &mut *self.get_mut().stream;
        Pin::new(stream).poll_seek(cx, pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::io::{AsyncReadExt as _, Cursor};

    #[tokio::test]
    async fn try_from_reads_length() {
        let data = b"hello world";
        let mut cursor = Cursor::new(data.to_vec());
        let stream = AsyncStream::try_from(&mut cursor).await.unwrap();
        assert_eq!(stream.len().await, 11);
        assert!(!stream.is_empty().await);
    }

    #[tokio::test]
    async fn try_from_empty() {
        let mut cursor = Cursor::new(Vec::<u8>::new());
        let stream = AsyncStream::try_from(&mut cursor).await.unwrap();
        assert_eq!(stream.len().await, 0);
        assert!(stream.is_empty().await);
    }

    #[tokio::test]
    async fn read_all_bytes() {
        let data = b"hello world";
        let mut cursor = Cursor::new(data.to_vec());
        let mut stream = AsyncStream::try_from(&mut cursor).await.unwrap();

        let mut buf = vec![0; 11];
        let n = stream.read(&mut buf).await.unwrap();
        assert_eq!(n, 11);
        assert_eq!(&buf, data);
    }

    #[tokio::test]
    async fn read_one_byte_at_a_time() {
        let data = b"hello world";
        let mut cursor = Cursor::new(data.to_vec());
        let mut stream = AsyncStream::try_from(&mut cursor).await.unwrap();

        for &expected in data {
            let mut buf = [0; 1];
            let n = stream.read(&mut buf).await.unwrap();
            assert_eq!(n, 1);
            assert_eq!(buf[0], expected);
        }
    }

    #[tokio::test]
    async fn reset_seeks_to_start() {
        let data = b"hello";
        let mut cursor = Cursor::new(data.to_vec());
        let mut stream = AsyncStream::try_from(&mut cursor).await.unwrap();

        // Read all bytes.
        let mut buf = vec![0; 5];
        stream.read_exact(&mut buf).await.unwrap();
        assert_eq!(&buf, data);

        // Reset and read again.
        stream.reset().await.unwrap();
        let mut buf2 = vec![0; 5];
        stream.read_exact(&mut buf2).await.unwrap();
        assert_eq!(&buf2, data);
    }
}
