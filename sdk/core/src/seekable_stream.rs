use async_trait::async_trait;
use bytes::Bytes;
use futures::io::AsyncRead;
use futures::stream::Stream;
use futures::task::Poll;

#[async_trait]
pub trait SeekableStream:
    AsyncRead + Unpin + std::fmt::Debug + Send + Sync + dyn_clone::DynClone
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
            Poll::Ready(Ok(0)) => Poll::Ready(None),
            Poll::Ready(Ok(bytes_read)) => {
                let bytes: Bytes = buffer.into();
                let bytes = bytes.slice(0..bytes_read);
                Poll::Ready(Some(Ok(bytes)))
            }
            Poll::Ready(Err(error)) => Poll::Ready(Some(Err(error.into()))),
            Poll::Pending => Poll::Pending,
        }
    }
}
