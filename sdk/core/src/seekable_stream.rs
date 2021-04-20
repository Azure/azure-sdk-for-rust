use async_std::io::{Read, Seek, SeekFrom};
use async_std::prelude::*;
use async_trait::async_trait;
use bytes::Bytes;
use futures::stream::Stream;
use std::pin::Pin;

#[async_trait]
pub trait SeekableStream:
    Read + Unpin + std::fmt::Debug + Send + Sync + dyn_clone::DynClone
{
    async fn reset(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

dyn_clone::clone_trait_object!(SeekableStream);

impl Stream for dyn SeekableStream {
    type Item = Result<Bytes, Box<dyn std::error::Error + Send + Sync>>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let mut buffer = vec![0_u8; 1024 * 64];

        match self.poll_read(cx, &mut buffer) {
            async_std::task::Poll::Ready(Ok(bytes_read)) => {
                if bytes_read == 0 {
                    futures::task::Poll::Ready(None)
                } else {
                    let bytes: Bytes = buffer.into();
                    let bytes = bytes.slice(0..bytes_read);
                    futures::task::Poll::Ready(Some(Ok(bytes)))
                }
            }
            async_std::task::Poll::Ready(Err(error)) => {
                futures::task::Poll::Ready(Some(Err(error.into())))
            }
            async_std::task::Poll::Pending => futures::task::Poll::Pending,
        }
    }
}

pub struct SelfResetStream<S>
where
    S: Read + Seek,
{
    stream: Pin<Box<S>>,
    buffer: [u8; 1024 * 64],
}

impl<S> SelfResetStream<S>
where
    S: Read + Seek + Unpin,
{
    pub async fn new(mut stream: S) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        stream.seek(SeekFrom::Start(0)).await?;

        Ok(Self {
            stream: Box::pin(stream),
            buffer: [0; 1024 * 64],
        })
    }
}

impl<S> Stream for SelfResetStream<S>
where
    S: Read + Seek,
{
    type Item = Result<Bytes, Box<dyn std::error::Error + Send + Sync>>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let mutable_self = self.get_mut();

        match mutable_self
            .stream
            .as_mut()
            .poll_read(cx, &mut mutable_self.buffer)
        {
            async_std::task::Poll::Ready(Ok(bytes_read)) => {
                if bytes_read == 0 {
                    futures::task::Poll::Ready(None)
                } else {
                    let bytes = Bytes::copy_from_slice(&mutable_self.buffer[..bytes_read]);
                    futures::task::Poll::Ready(Some(Ok(bytes)))
                }
            }
            async_std::task::Poll::Ready(Err(error)) => {
                futures::task::Poll::Ready(Some(Err(error.into())))
            }
            async_std::task::Poll::Pending => futures::task::Poll::Pending,
        }
    }
}
