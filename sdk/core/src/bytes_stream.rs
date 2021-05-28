use crate::SeekableStream;
use crate::StreamError;
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
    type Item = Result<Bytes, StreamError>;

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let mut self_mut = self.get_mut();

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
    async fn reset(&mut self) -> Result<(), StreamError> {
        self.bytes_read = 0;
        Ok(())
    }
}

impl AsyncRead for BytesStream {
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        let mut self_mut = self.get_mut();

        if self_mut.bytes_read < self_mut.bytes.len() {
            let bytes_read = self_mut.bytes_read;
            let remaining_bytes = self_mut.bytes.len() - bytes_read;

            let bytes_to_copy = std::cmp::min(remaining_bytes, buf.len());

            for (buf_byte, bytes_byte) in buf
                .iter_mut()
                .zip(self_mut.bytes.slice(self_mut.bytes_read..bytes_to_copy))
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
