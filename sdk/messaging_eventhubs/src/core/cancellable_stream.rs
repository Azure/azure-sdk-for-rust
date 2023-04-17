use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures_util::{Future, Stream};

pin_project_lite::pin_project! {
    pub struct CancelStreamFut<'a, T> {
        #[pin]
        stream: &'a mut T,
    }
}

impl<'a, T, O, E> Future for CancelStreamFut<'a, T>
where
    T: CancellableStream<Item = Result<O, E>> + Unpin,
    T::CancelError: From<E>,
{
    type Output = Result<(), T::CancelError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        this.stream.as_mut().try_cancel()?;

        loop {
            match this.stream.as_mut().poll_next(cx) {
                Poll::Ready(Some(Ok(_))) => continue,
                Poll::Ready(Some(Err(e))) => return Poll::Ready(Err(e.into())),
                Poll::Ready(None) => return Poll::Ready(Ok(())),
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}

pub trait CancellableStream: Stream {
    type CancelFut<'a>: Future<Output = Result<(), Self::CancelError>>
    where
        Self: 'a;
    type CancelError: std::error::Error;

    fn try_cancel(&mut self) -> Result<(), Self::CancelError>;

    fn cancel(&mut self) -> Self::CancelFut<'_>;
}
