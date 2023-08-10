use crate::SeekableStream;
use bytes::Bytes;
use futures::io::AsyncRead;
use futures::stream::Stream;
use std::pin::Pin;
use std::task::Poll;

/// Convenience struct that maps a `bytes::Bytes` buffer into a stream.
///
/// This struct implements both `Stream` and `SeekableStream` for an
/// immutable bytes buffer. It's cheap to clone but remember to `reset`
/// the stream position if you clone it.
#[derive(Debug, Clone)]
pub struct BytesStream {
    bytes: Bytes,
    bytes_read: usize,
}

impl BytesStream {
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

impl From<Bytes> for BytesStream {
    fn from(bytes: Bytes) -> Self {
        Self::new(bytes)
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

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl SeekableStream for BytesStream {
    async fn reset(&mut self) -> crate::Result<()> {
        self.bytes_read = 0;
        Ok(())
    }

    fn len(&self) -> usize {
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

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use futures::io::AsyncReadExt;
    use futures::stream::StreamExt;

    // Test BytesStream Stream
    #[test]
    fn test_bytes_stream() {
        let bytes = Bytes::from("hello world");
        let mut stream = BytesStream::new(bytes.clone());

        let mut buf = Vec::new();
        let mut bytes_read = 0;
        while let Some(Ok(bytes)) = futures::executor::block_on(stream.next()) {
            buf.extend_from_slice(&bytes);
            bytes_read += bytes.len();
        }

        assert_eq!(bytes_read, bytes.len());
        assert_eq!(buf, bytes);
    }

    // Test BytesStream AsyncRead, all bytes at once
    #[test]
    fn test_async_read_all_bytes_at_once() {
        let bytes = Bytes::from("hello world");
        let mut stream = BytesStream::new(bytes.clone());

        let mut buf = [0; 11];
        let bytes_read = futures::executor::block_on(stream.read(&mut buf)).unwrap();
        assert_eq!(bytes_read, 11);
        assert_eq!(&buf[..], &bytes);
    }

    // Test BytesStream AsyncRead, one byte at a time
    #[test]
    fn test_async_read_one_byte_at_a_time() {
        let bytes = Bytes::from("hello world");
        let mut stream = BytesStream::new(bytes.clone());

        for i in 0..bytes.len() {
            let mut buf = [0; 1];
            let bytes_read = futures::executor::block_on(stream.read(&mut buf)).unwrap();
            assert_eq!(bytes_read, 1);
            assert_eq!(buf[0], bytes[i]);
        }
    }
}
