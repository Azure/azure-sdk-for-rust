// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Streams for testing purposes.

use azure_core::stream::SeekableStream;
#[cfg(not(target_arch = "wasm32"))]
use azure_core::{
    http::{Body, RequestContent},
    Bytes,
};
use futures::{io::AsyncRead, stream::Stream};
use std::{fmt, iter::Cycle, ops::Range, pin::Pin, task::Poll};

/// Implements a [`Stream`] over an endless cycle of bytes.
#[derive(Clone)]
pub struct GeneratedStream<I, const LENGTH: usize, const CHUNK: usize = 1024> {
    generator: Cycle<I>,
    bytes_read: usize,
}

impl<const LENGTH: usize, const CHUNK: usize> GeneratedStream<Range<u8>, LENGTH, CHUNK> {
    /// Creates a `GeneratedStream` over a series of bytes from `0..255`.
    ///
    /// # Examples
    ///
    /// ```
    /// use azure_core_test::stream::GeneratedStream;
    /// use futures::io::AsyncReadExt;
    ///
    /// # #[tokio::main] async fn main() {
    /// let mut stream = GeneratedStream::<_, 4>::new();
    /// let mut buf = Vec::new();
    /// stream.read_to_end(&mut buf).await.unwrap();
    /// assert_eq!(buf, vec![0u8, 1, 2, 3]);
    /// # }
    /// ```
    pub fn new() -> GeneratedStream<Range<u8>, LENGTH, CHUNK> {
        GeneratedStream {
            generator: (0..u8::MAX).cycle(),
            bytes_read: 0,
        }
    }
}

impl<I, const LENGTH: usize, const CHUNK: usize> GeneratedStream<I, LENGTH, CHUNK>
where
    I: Iterator<Item = u8> + Clone,
{
    /// Creates a `GeneratedStream` over a custom iterator of bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use azure_core_test::stream::GeneratedStream;
    /// use futures::io::AsyncReadExt;
    ///
    /// # #[tokio::main] async fn main() {
    /// let iter = b"hello, world!".iter().copied();
    /// let mut stream = GeneratedStream::<_, 18>::from_iter(iter);
    /// let mut buf = Vec::new();
    /// stream.read_to_end(&mut buf).await.unwrap();
    /// let s = String::from_utf8(buf).unwrap();
    /// assert_eq!(s, "hello, world!hello");
    /// # }
    /// ```
    #[allow(clippy::should_implement_trait)]
    pub fn from_iter(iter: I) -> Self {
        GeneratedStream {
            generator: iter.cycle(),
            bytes_read: 0,
        }
    }
}

impl<I, const LENGTH: usize, const CHUNK: usize> fmt::Debug for GeneratedStream<I, LENGTH, CHUNK> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GeneratedStream")
            .field("bytes_read", &self.bytes_read)
            .finish_non_exhaustive()
    }
}

impl<const LENGTH: usize, const CHUNK: usize> Default
    for GeneratedStream<Range<u8>, LENGTH, CHUNK>
{
    /// Creates a `GeneratedStream` over a series of bytes from `0..255`.
    ///
    /// # Examples
    ///
    /// ```
    /// use azure_core_test::stream::GeneratedStream;
    /// use futures::io::AsyncReadExt;
    ///
    /// # #[tokio::main] async fn main() {
    /// let mut stream = GeneratedStream::<_, 4>::default();
    /// let mut buf = Vec::new();
    /// stream.read_to_end(&mut buf).await.unwrap();
    /// assert_eq!(buf, vec![0u8, 1, 2, 3]);
    /// # }
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl<I, const LENGTH: usize, const CHUNK: usize> AsyncRead for GeneratedStream<I, LENGTH, CHUNK>
where
    I: Clone,
    Cycle<I>: Iterator<Item = u8> + Unpin,
{
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        let self_mut = self.get_mut();

        if self_mut.bytes_read >= LENGTH {
            return Poll::Ready(Ok(0));
        }

        let remaining_bytes = LENGTH - self_mut.bytes_read;
        let bytes_to_read = std::cmp::min(remaining_bytes, buf.len());

        for byte_slot in buf.iter_mut().take(bytes_to_read) {
            *byte_slot = self_mut.generator.next().unwrap();
            self_mut.bytes_read += 1;
        }

        tracing::debug!("read {bytes_to_read} bytes");
        Poll::Ready(Ok(bytes_to_read))
    }
}

impl<I, const LENGTH: usize, const CHUNK: usize> Stream for GeneratedStream<I, LENGTH, CHUNK>
where
    I: Clone,
    Cycle<I>: Iterator<Item = u8> + Unpin,
{
    type Item = std::io::Result<Vec<u8>>;

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let self_mut = self.get_mut();

        if self_mut.bytes_read >= LENGTH {
            return Poll::Ready(None);
        }

        let remaining_bytes = LENGTH - self_mut.bytes_read;
        let bytes_to_read = std::cmp::min(remaining_bytes, CHUNK);

        let chunk: Vec<u8> = (0..bytes_to_read)
            .map(|_| {
                self_mut.bytes_read += 1;
                self_mut.generator.next().unwrap()
            })
            .collect();

        tracing::debug!("read {} bytes", chunk.len());
        Poll::Ready(Some(Ok(chunk)))
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl<I, const LENGTH: usize, const CHUNK: usize> SeekableStream
    for GeneratedStream<I, LENGTH, CHUNK>
where
    I: Clone + Send + Sync,
    Cycle<I>: Iterator<Item = u8> + Unpin,
{
    async fn reset(&mut self) -> azure_core::Result<()> {
        self.bytes_read = 0;
        tracing::trace!("reset");
        Ok(())
    }

    fn len(&self) -> usize {
        LENGTH
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<I, const LENGTH: usize, const CHUNK: usize> From<&GeneratedStream<I, LENGTH, CHUNK>> for Body
where
    for<'a> I: Clone + Send + Sync + 'a,
    Cycle<I>: Iterator<Item = u8> + Unpin,
{
    fn from(stream: &GeneratedStream<I, LENGTH, CHUNK>) -> Self {
        Body::SeekableStream(Box::new(stream.clone()))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<I, const LENGTH: usize, const CHUNK: usize> From<GeneratedStream<I, LENGTH, CHUNK>> for Body
where
    for<'a> I: Clone + Send + Sync + 'a,
    Cycle<I>: Iterator<Item = u8> + Unpin,
{
    fn from(stream: GeneratedStream<I, LENGTH, CHUNK>) -> Self {
        Body::SeekableStream(Box::new(stream))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<I, const LENGTH: usize, const CHUNK: usize> From<&GeneratedStream<I, LENGTH, CHUNK>>
    for RequestContent<Bytes>
where
    for<'a> I: Clone + Send + Sync + 'a,
    Cycle<I>: Iterator<Item = u8> + Unpin,
{
    fn from(stream: &GeneratedStream<I, LENGTH, CHUNK>) -> Self {
        Body::from(stream).into()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<I, const LENGTH: usize, const CHUNK: usize> From<GeneratedStream<I, LENGTH, CHUNK>>
    for RequestContent<Bytes>
where
    for<'a> I: Clone + Send + Sync + 'a,
    Cycle<I>: Iterator<Item = u8> + Unpin,
{
    fn from(stream: GeneratedStream<I, LENGTH, CHUNK>) -> Self {
        Body::from(stream).into()
    }
}

#[cfg(test)]
mod tests {
    use super::GeneratedStream;
    use futures::{io::AsyncReadExt as _, stream::StreamExt as _};

    #[tokio::test]
    async fn async_read_all_bytes_at_once() {
        let mut stream = GeneratedStream::<_, 100>::default();
        let mut buf = vec![0u8; 100];

        stream.read_exact(&mut buf).await.unwrap();

        // Verify the pattern matches the cycle (0..255)
        for (i, &byte) in buf.iter().enumerate() {
            assert_eq!(byte, (i % 255) as u8);
        }
    }

    #[tokio::test]
    async fn async_read_partial_chunks() {
        let mut stream = GeneratedStream::<_, 50>::default();
        let mut total_read = 0;
        let mut all_bytes = Vec::new();

        // Read in chunks of 10 bytes
        loop {
            let mut buf = [0u8; 10];
            let bytes_read = stream.read(&mut buf).await.unwrap();
            if bytes_read == 0 {
                break;
            }
            all_bytes.extend_from_slice(&buf[..bytes_read]);
            total_read += bytes_read;
        }

        assert_eq!(total_read, 50);
        assert_eq!(all_bytes.len(), 50);

        // Verify the pattern
        for (i, &byte) in all_bytes.iter().enumerate() {
            assert_eq!(byte, (i % 255) as u8);
        }
    }

    #[tokio::test]
    async fn stream_1024_byte_chunks() {
        let mut stream = GeneratedStream::<_, 3000>::default();
        let mut total_bytes = 0;
        let mut chunk_count = 0;

        while let Some(Ok(chunk)) = stream.next().await {
            chunk_count += 1;
            total_bytes += chunk.len();

            // First two chunks should be 1024 bytes, last chunk should be smaller
            if chunk_count <= 2 {
                assert_eq!(chunk.len(), 1024);
            } else {
                assert!(chunk.len() <= 1024);
            }
        }

        assert_eq!(total_bytes, 3000);
        assert_eq!(chunk_count, 3); // 1024 + 1024 + 952 = 3000
    }

    #[tokio::test]
    async fn stream_respects_max_limit() {
        let mut stream = GeneratedStream::<_, 10>::default();
        let mut total_bytes = 0;

        while let Some(Ok(chunk)) = stream.next().await {
            total_bytes += chunk.len();
        }

        assert_eq!(total_bytes, 10);
    }

    #[tokio::test]
    async fn custom_chunk_size() {
        let mut stream = GeneratedStream::<_, 100, 32>::default();
        let mut total_bytes = 0;
        let mut chunk_count = 0;

        while let Some(Ok(chunk)) = stream.next().await {
            chunk_count += 1;
            total_bytes += chunk.len();

            // All chunks except possibly the last should be 32 bytes
            if total_bytes < 100 {
                assert_eq!(chunk.len(), 32);
            } else {
                assert!(chunk.len() <= 32);
            }
        }

        assert_eq!(total_bytes, 100);
        assert_eq!(chunk_count, 4); // 32 + 32 + 32 + 4 = 100
    }

    #[tokio::test]
    async fn from_iter_hello_world() {
        let hello_world = b"Hello, world!";
        let iter = hello_world.iter().copied();
        let mut stream = GeneratedStream::<_, 16>::from_iter(iter);

        let mut buf = Vec::new();
        stream.read_to_end(&mut buf).await.unwrap();

        assert_eq!(buf.len(), 16);
        let result_str = std::str::from_utf8(&buf).unwrap();
        // Should be "Hello, world!" (13 bytes) + "Hel" (3 more bytes to reach 16 total)
        assert_eq!(result_str, "Hello, world!Hel");
    }
}
