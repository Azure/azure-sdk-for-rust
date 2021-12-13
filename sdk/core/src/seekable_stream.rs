use crate::StreamError;
use async_trait::async_trait;
use bytes::Bytes;
use futures::io::AsyncRead;
use futures::stream::Stream;
use futures::task::Poll;

/// Enable a type implementing `AsyncRead` to be consumed as if it were
/// a `Stream` of `Bytes`.
#[async_trait]
pub trait SeekableStream:
    AsyncRead + Unpin + std::fmt::Debug + Send + Sync + dyn_clone::DynClone
{
    async fn reset(&mut self) -> Result<(), StreamError>;
}

dyn_clone::clone_trait_object!(SeekableStream);

impl Stream for dyn SeekableStream {
    type Item = Result<Bytes, StreamError>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let mut buffer = vec![0_u8; 1024 * 64];

        match self.poll_read(cx, &mut buffer) {
            Poll::Ready(Ok(0)) => Poll::Ready(None),
            Poll::Ready(Ok(bytes_read)) => {
                let bytes: Bytes = buffer.into();
                let bytes = bytes.slice(0..bytes_read);
                Poll::Ready(Some(Ok(bytes)))
            }
            Poll::Ready(Err(err)) => Poll::Ready(Some(Err(StreamError::PollError(err)))),
            Poll::Pending => Poll::Pending,
        }
    }
}
