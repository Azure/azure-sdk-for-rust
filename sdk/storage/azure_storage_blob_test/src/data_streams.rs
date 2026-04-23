use azure_core::{
    http::{Body, NoFormat, RequestContent},
    stream::SeekableStream,
};
use bytes::Bytes;
use futures::{io::AsyncRead, stream::Stream};
use std::{fmt, iter::Cycle, ops::Range, pin::Pin, task::Poll};

/// Implements a [`Stream`] over an endless cycle of bytes.
#[derive(Clone)]
pub struct GeneratedStream<I> {
    generator: Cycle<I>,
    bytes_read: u64,
    len: u64,
    chunk: usize,
}

impl GeneratedStream<Range<u8>> {
    pub fn new(len: u64, chunk: Option<usize>) -> GeneratedStream<Range<u8>> {
        GeneratedStream {
            generator: (0..u8::MAX).cycle(),
            bytes_read: 0,
            len,
            chunk: chunk.unwrap_or(1024),
        }
    }
}

impl<I> GeneratedStream<I>
where
    I: Iterator<Item = u8> + Clone,
{
    #[allow(clippy::should_implement_trait)]
    pub fn from_iter(iter: I, len: u64, chunk: Option<usize>) -> Self {
        GeneratedStream {
            generator: iter.cycle(),
            bytes_read: 0,
            len,
            chunk: chunk.unwrap_or(1024),
        }
    }
}

impl<I> fmt::Debug for GeneratedStream<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GeneratedStream")
            .field("bytes_read", &self.bytes_read)
            .finish_non_exhaustive()
    }
}

impl<I> AsyncRead for GeneratedStream<I>
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

        if self_mut.bytes_read >= self_mut.len {
            return Poll::Ready(Ok(0));
        }

        let remaining_bytes = self_mut.len - self_mut.bytes_read;
        let bytes_to_read = std::cmp::min(remaining_bytes, buf.len() as u64) as usize;

        for byte_slot in buf.iter_mut().take(bytes_to_read) {
            *byte_slot = self_mut.generator.next().unwrap();
            self_mut.bytes_read += 1;
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

        if self_mut.bytes_read >= self_mut.len {
            return Poll::Ready(None);
        }

        let remaining_bytes = self_mut.len - self_mut.bytes_read;
        let bytes_to_read = std::cmp::min(remaining_bytes, self_mut.chunk as u64);

        let chunk: Vec<u8> = (0..bytes_to_read)
            .map(|_| {
                self_mut.bytes_read += 1;
                self_mut.generator.next().unwrap()
            })
            .collect();

        Poll::Ready(Some(Ok(chunk)))
    }
}

#[async_trait::async_trait]
impl<I> SeekableStream for GeneratedStream<I>
where
    I: Clone + Send + Sync,
    Cycle<I>: Iterator<Item = u8> + Unpin,
{
    async fn reset(&mut self) -> azure_core::Result<()> {
        self.bytes_read = 0;
        Ok(())
    }

    fn len(&self) -> Option<u64> {
        Some(self.len)
    }
}

impl<I> From<GeneratedStream<I>> for Body
where
    for<'a> I: Clone + Send + Sync + 'a,
    Cycle<I>: Iterator<Item = u8> + Unpin,
{
    fn from(stream: GeneratedStream<I>) -> Self {
        Body::SeekableStream(Box::new(stream))
    }
}

impl<I> From<GeneratedStream<I>> for RequestContent<Bytes, NoFormat>
where
    for<'a> I: Clone + Send + Sync + 'a,
    Cycle<I>: Iterator<Item = u8> + Unpin,
{
    fn from(stream: GeneratedStream<I>) -> Self {
        Body::from(stream).into()
    }
}
