// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{cmp::min, iter::Cycle, num::NonZero, ops::Range, pin::Pin, task::Poll};

use azure_core::{
    http::{Body, NoFormat, RequestContent},
    stream::SeekableStream,
};
use bytes::{BufMut, Bytes};
use crc_fast::{CrcAlgorithm, Digest};
use futures::{AsyncRead, Stream};
use rand::random;

pub fn random_data_stream_with_checksum(
    len: u64,
    algorithm: CrcAlgorithm,
) -> (impl SeekableStream, u64) {
    let src_bytes: [u8; 9999] = random();

    let mut digest = Digest::new(algorithm);
    let mut read = 0;
    while read < len {
        let to_digest = min(src_bytes.len(), (len - read) as usize);
        digest.update(&src_bytes[..to_digest]);
        read += to_digest as u64;
    }

    (
        GeneratedStream::from_iter(src_bytes.into_iter(), len, None),
        digest.finalize(),
    )
}

pub fn random_data_memory_with_checksum(len: usize, algorithm: CrcAlgorithm) -> (Vec<u8>, u64) {
    let buf: [u8; 9999] = random();
    let mut data = Vec::with_capacity(len);

    for i in (0..len).step_by(buf.len()) {
        let copy = min(buf.len(), len - i);
        data.put(&buf[..copy])
    }

    let mut digest = Digest::new(algorithm);
    digest.update(&data);
    (data, digest.finalize())
}

/// Implements a [`Stream`] over an endless cycle of bytes.
#[derive(Clone)]
pub struct GeneratedStream<I> {
    /// Generator for bytes in the stream.
    generator: Cycle<I>,
    /// Initial state of the generator, used to reset the stream.
    generator_reset_src: Cycle<I>,
    /// Position in the stream, in bytes.
    cursor: u64,
    /// Stream length.
    len: u64,
    /// The maximum number of bytes to return in a single poll.
    chunk: NonZero<usize>,
}

impl GeneratedStream<Range<u8>> {
    pub fn new(len: u64, chunk: Option<NonZero<usize>>) -> GeneratedStream<Range<u8>> {
        GeneratedStream {
            generator: (0..u8::MAX).cycle(),
            generator_reset_src: (0..u8::MAX).cycle(),
            cursor: 0,
            len,
            chunk: chunk.unwrap_or(NonZero::new(usize::MAX).unwrap()),
        }
    }
}

impl<I> GeneratedStream<I>
where
    I: Iterator<Item = u8> + Clone,
{
    #[allow(clippy::should_implement_trait)]
    pub fn from_iter(iter: I, len: u64, chunk: Option<NonZero<usize>>) -> Self {
        GeneratedStream {
            generator: iter.clone().cycle(),
            generator_reset_src: iter.cycle(),
            cursor: 0,
            len,
            chunk: chunk.unwrap_or(NonZero::new(usize::MAX).unwrap()),
        }
    }
}

impl<I> std::fmt::Debug for GeneratedStream<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GeneratedStream")
            .field("bytes_read", &self.cursor)
            .finish_non_exhaustive()
    }
}

impl<I> AsyncRead for GeneratedStream<I>
where
    I: Clone + Unpin,
    Cycle<I>: Iterator<Item = u8>,
{
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        let self_mut = self.get_mut();

        if self_mut.cursor >= self_mut.len {
            return Poll::Ready(Ok(0));
        }

        let remaining_bytes = self_mut.len - self_mut.cursor;
        let bytes_to_read = std::cmp::min(remaining_bytes, buf.len() as u64) as usize;

        for byte_slot in buf.iter_mut().take(bytes_to_read) {
            *byte_slot = self_mut.generator.next().unwrap();
            self_mut.cursor += 1;
        }

        Poll::Ready(Ok(bytes_to_read))
    }
}

impl<I> Stream for GeneratedStream<I>
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

        if self_mut.cursor >= self_mut.len {
            return Poll::Ready(None);
        }

        let remaining_bytes = self_mut.len - self_mut.cursor;
        let bytes_to_read = std::cmp::min(remaining_bytes, self_mut.chunk.get() as u64);

        let chunk: Vec<u8> = (0..bytes_to_read)
            .map(|_| {
                self_mut.cursor += 1;
                self_mut.generator.next().unwrap()
            })
            .collect();

        Poll::Ready(Some(Ok(chunk)))
    }
}

#[async_trait::async_trait]
impl<I> SeekableStream for GeneratedStream<I>
where
    I: Clone + Send + Sync + Unpin,
    Cycle<I>: Iterator<Item = u8> + Unpin,
{
    async fn reset(&mut self) -> azure_core::Result<()> {
        self.cursor = 0;
        self.generator = self.generator_reset_src.clone();
        Ok(())
    }

    fn len(&self) -> Option<u64> {
        Some(self.len)
    }
}

impl<I> From<GeneratedStream<I>> for Body
where
    for<'a> I: Clone + Send + Sync + Unpin + 'a,
    Cycle<I>: Iterator<Item = u8> + Unpin,
{
    fn from(stream: GeneratedStream<I>) -> Self {
        Body::SeekableStream(Box::new(stream))
    }
}

impl<I> From<GeneratedStream<I>> for RequestContent<Bytes, NoFormat>
where
    for<'a> I: Clone + Send + Sync + Unpin + 'a,
    Cycle<I>: Iterator<Item = u8> + Unpin,
{
    fn from(stream: GeneratedStream<I>) -> Self {
        Body::from(stream).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{AsyncReadExt, TryStreamExt};

    #[tokio::test]
    async fn generated_stream_as_stream() -> azure_core::Result<()> {
        for buf_len in [1, 100, 256, 9999] {
            for stream_len in [buf_len, buf_len - 1, buf_len + 1, buf_len * 10, buf_len / 2] {
                let mut src_buf = vec![0u8; buf_len];
                for b in src_buf.iter_mut() {
                    *b = random();
                }

                let mut stream = GeneratedStream::from_iter(
                    src_buf.clone().into_iter(),
                    stream_len as u64,
                    None,
                );

                assert_eq!(stream.len(), Some(stream_len as u64));
                let streamed_data_1 = (&mut stream).try_concat().await?;
                assert_eq!(streamed_data_1.len(), stream_len);

                stream.reset().await?;

                let streamed_data_2 = stream.try_concat().await?;
                assert_eq!(streamed_data_1, streamed_data_2);

                let mut remaining = &streamed_data_1[..];
                while !remaining.is_empty() {
                    let min_len = min(src_buf.len(), remaining.len());
                    assert_eq!(src_buf[..min_len], remaining[..min_len]);
                    remaining = &remaining[min_len..];
                }
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn generated_stream_as_read() -> azure_core::Result<()> {
        for buf_len in [1, 100, 256, 9999] {
            for stream_len in [buf_len, buf_len - 1, buf_len + 1, buf_len * 10, buf_len / 2] {
                let mut src_buf = vec![0u8; buf_len];
                for b in src_buf.iter_mut() {
                    *b = random();
                }

                let mut stream = GeneratedStream::from_iter(
                    src_buf.clone().into_iter(),
                    stream_len as u64,
                    None,
                );
                let buf_1 = &mut vec![0u8; stream_len];
                let buf_2 = &mut vec![0u8; stream_len];

                assert_eq!(stream.len(), Some(stream_len as u64));
                stream.read_exact(buf_1).await?;
                assert_eq!(0, stream.read(&mut [0; 1024]).await?);

                stream.reset().await?;

                stream.read_exact(buf_2).await?;
                assert_eq!(0, stream.read(&mut [0; 1024]).await?);
                assert_eq!(buf_1, buf_2);

                let mut remaining = &buf_1[..];
                while !remaining.is_empty() {
                    let min_len = min(src_buf.len(), remaining.len());
                    assert_eq!(src_buf[..min_len], remaining[..min_len]);
                    remaining = &remaining[min_len..];
                }
            }
        }
        Ok(())
    }
}
