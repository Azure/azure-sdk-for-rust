use bytes::Bytes;
use futures::io::AsyncRead;
use futures::stream::Stream;
use futures::task::Poll;

/// Enable a type implementing `AsyncRead` to be consumed as if it were
/// a `Stream` of `Bytes`.
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
pub trait SeekableStream:
    AsyncRead + Unpin + std::fmt::Debug + Send + Sync + dyn_clone::DynClone
{
    async fn reset(&mut self) -> crate::error::Result<()>;
    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

dyn_clone::clone_trait_object!(SeekableStream);

impl Stream for dyn SeekableStream {
    type Item = crate::error::Result<Bytes>;

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
            Poll::Ready(Err(err)) => Poll::Ready(Some(Err(crate::error::Error::full(
                crate::error::ErrorKind::Io,
                err,
                "an error was encountered when trying to read from a stream",
            )))),
            Poll::Pending => Poll::Pending,
        }
    }
}
