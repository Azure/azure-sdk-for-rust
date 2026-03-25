// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{Bytes, SeekableStream};
use crate::http::Body;
use futures::{
    io::{AsyncRead, AsyncSeek},
    stream::Stream,
};
use std::{fmt, io, pin::Pin, task::Poll};

/// Convenience struct that maps a `bytes::Bytes` buffer into a stream.
///
/// This struct implements both `Stream` and `SeekableStream` for an
/// immutable bytes buffer. It's cheap to clone but remember to `reset`
/// the stream position if you clone it.
#[derive(Clone)]
pub struct BytesStream {
    bytes: Bytes,
    bytes_read: usize,
}

impl BytesStream {
    /// Creates a new `BytesStream` from the given bytes.
    ///
    /// # Arguments
    /// * `bytes` - The bytes to be streamed.
    pub fn new(bytes: impl Into<Bytes>) -> Self {
        Self {
            bytes: bytes.into(),
            bytes_read: 0,
        }
    }

    /// Creates a stream that resolves immediately with no data.
    pub fn new_empty() -> Self {
        Self::new(Bytes::new())
    }
}

impl fmt::Debug for BytesStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BytesStream")
            .field("bytes", &format!("(len: {})", self.bytes.len()))
            .field("bytes_read", &self.bytes_read)
            .finish()
    }
}

impl From<Bytes> for BytesStream {
    fn from(bytes: Bytes) -> Self {
        Self::new(bytes)
    }
}

impl From<BytesStream> for Body {
    fn from(stream: BytesStream) -> Self {
        Body::SeekableStream(Box::new(stream))
    }
}

impl Stream for BytesStream {
    type Item = crate::Result<Bytes>;

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let self_mut = self.get_mut();

        // we return all the available bytes in one call.
        if self_mut.bytes_read < self_mut.bytes.len() {
            let bytes_read = self_mut.bytes_read;
            self_mut.bytes_read = self_mut.bytes.len();
            Poll::Ready(Some(Ok(self_mut.bytes.slice(bytes_read..))))
        } else {
            Poll::Ready(None)
        }
    }
}

#[async_trait::async_trait]
impl SeekableStream for BytesStream {
    async fn reset(&mut self) -> crate::Result<()> {
        self.bytes_read = 0;
        Ok(())
    }

    async fn len(&self) -> usize {
        self.bytes.len()
    }
}

impl AsyncRead for BytesStream {
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        let self_mut = self.get_mut();

        if self_mut.bytes_read < self_mut.bytes.len() {
            let bytes_read = self_mut.bytes_read;
            let remaining_bytes = self_mut.bytes.len() - bytes_read;

            let bytes_to_copy = std::cmp::min(remaining_bytes, buf.len());
            let bytes_to_read_end = self_mut.bytes_read + bytes_to_copy;

            for (buf_byte, bytes_byte) in buf
                .iter_mut()
                .zip(self_mut.bytes.slice(self_mut.bytes_read..bytes_to_read_end))
            {
                *buf_byte = bytes_byte;
            }

            self_mut.bytes_read += bytes_to_copy;

            Poll::Ready(Ok(bytes_to_copy))
        } else {
            Poll::Ready(Ok(0))
        }
    }
}

impl AsyncSeek for BytesStream {
    fn poll_seek(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        pos: io::SeekFrom,
    ) -> Poll<io::Result<u64>> {
        let self_mut = self.get_mut();
        let len = self_mut.bytes.len() as i64;
        let new_pos = match pos {
            io::SeekFrom::Start(offset) => offset as i64,
            io::SeekFrom::End(offset) => len + offset,
            io::SeekFrom::Current(offset) => self_mut.bytes_read as i64 + offset,
        };

        if new_pos < 0 {
            return Poll::Ready(Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "seek to a negative position",
            )));
        }

        self_mut.bytes_read = std::cmp::min(new_pos as usize, self_mut.bytes.len());
        Poll::Ready(Ok(self_mut.bytes_read as u64))
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use futures::io::{AsyncReadExt, AsyncSeekExt};
    use futures::stream::StreamExt;

    // Test BytesStream Stream
    #[tokio::test]
    async fn bytes_stream() {
        let bytes = Bytes::from("hello world");
        let mut stream = BytesStream::new(bytes.clone());

        let mut buf = Vec::new();
        let mut bytes_read = 0;
        while let Some(Ok(bytes)) = stream.next().await {
            buf.extend_from_slice(&bytes);
            bytes_read += bytes.len();
        }

        assert_eq!(bytes_read, bytes.len());
        assert_eq!(buf, bytes);
    }

    // Test BytesStream AsyncRead, all bytes at once
    #[tokio::test]
    async fn async_read_all_bytes_at_once() {
        let bytes = Bytes::from("hello world");
        let mut stream = BytesStream::new(bytes.clone());

        let mut buf = [0; 11];
        let bytes_read = stream.read(&mut buf).await.unwrap();
        assert_eq!(bytes_read, 11);
        assert_eq!(&buf[..], &bytes);
    }

    // Test BytesStream AsyncRead, one byte at a time
    #[tokio::test]
    async fn async_read_one_byte_at_a_time() {
        let bytes = Bytes::from("hello world");
        let mut stream = BytesStream::new(bytes.clone());

        for i in 0..bytes.len() {
            let mut buf = [0; 1];
            let bytes_read = stream.read(&mut buf).await.unwrap();
            assert_eq!(bytes_read, 1);
            assert_eq!(buf[0], bytes[i]);
        }
    }

    #[tokio::test]
    async fn seek_from_start() {
        let mut stream = BytesStream::new(Bytes::from("hello world"));
        let pos = stream.seek(io::SeekFrom::Start(6)).await.unwrap();
        assert_eq!(pos, 6);

        let mut buf = [0; 5];
        stream.read_exact(&mut buf).await.unwrap();
        assert_eq!(&buf, b"world");
    }

    #[tokio::test]
    async fn seek_from_end() {
        let mut stream = BytesStream::new(Bytes::from("hello world"));
        let pos = stream.seek(io::SeekFrom::End(-5)).await.unwrap();
        assert_eq!(pos, 6);

        let mut buf = [0; 5];
        stream.read_exact(&mut buf).await.unwrap();
        assert_eq!(&buf, b"world");
    }

    #[tokio::test]
    async fn seek_from_current() {
        let mut stream = BytesStream::new(Bytes::from("hello world"));

        // Read "hello" then seek forward past the space.
        let mut buf = [0; 5];
        stream.read_exact(&mut buf).await.unwrap();
        assert_eq!(&buf, b"hello");

        let pos = stream.seek(io::SeekFrom::Current(1)).await.unwrap();
        assert_eq!(pos, 6);

        let mut buf2 = [0; 5];
        stream.read_exact(&mut buf2).await.unwrap();
        assert_eq!(&buf2, b"world");
    }

    #[tokio::test]
    async fn seek_negative_position_fails() {
        let mut stream = BytesStream::new(Bytes::from("hello"));
        let pos = stream.seek(io::SeekFrom::Start(0)).await.unwrap();
        assert_eq!(pos, 0);

        let err = stream.seek(io::SeekFrom::Current(-1)).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }

    #[tokio::test]
    async fn seek_past_end_clamps_to_length() {
        let mut stream = BytesStream::new(Bytes::from("hello"));
        let pos = stream.seek(io::SeekFrom::Start(100)).await.unwrap();
        assert_eq!(pos, 5);

        let n = stream.read(&mut [0; 1]).await.unwrap();
        assert_eq!(n, 0);
    }
}
