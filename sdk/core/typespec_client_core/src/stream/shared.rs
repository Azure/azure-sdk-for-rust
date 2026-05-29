// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::SeekableStream;
use futures::{io::AsyncRead, lock::Mutex};
use std::{
    fmt,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

/// A cloneable [`SeekableStream`] wrapper backed by a shared, reference-counted mutex.
///
/// `SharedStream` allows a single stream to be shared across multiple owners without
/// deep-copying the underlying data. Cloning a `SharedStream` is a cheap pointer bump;
/// all clones read from the same underlying stream at the same position.
///
/// Unlike the deep-clone that [`Body::clone`](crate::http::Body::clone) performs on a
/// plain `Box<dyn SeekableStream>`, placing a `SharedStream` inside a `Body` makes cloning
/// the body allocation-free for the stream variant.
///
/// # Construction
///
/// Use the [`FuturesAsyncReadExt`] extension trait to wrap any [`futures::io::AsyncRead`] source,
/// or [`TokioAsyncReadExt`] for a [`tokio::io::AsyncRead`] source:
///
/// ```
/// # use typespec_client_core::stream::FuturesAsyncReadExt as _;
/// let data: &[u8] = b"hello";
/// let stream = data.shared(Some(data.len() as u64));
/// ```
///
/// To wrap an existing [`SeekableStream`] directly, use [`SharedStream::new`].
pub struct SharedStream {
    inner: Arc<Mutex<Box<dyn SeekableStream>>>,
    len: Option<u64>,
}

impl SharedStream {
    /// Creates a new `SharedStream` wrapping `stream`.
    ///
    /// The length is captured from [`SeekableStream::len()`] at construction time.
    pub fn new<S: SeekableStream + 'static>(stream: S) -> Self {
        let len = stream.len();
        Self {
            inner: Arc::new(Mutex::new(Box::new(stream))),
            len,
        }
    }
}

impl Clone for SharedStream {
    /// Returns a shallow clone backed by the same [`Arc`].
    ///
    /// This is a cheap pointer-bump operation; it does not copy stream data.
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
            len: self.len,
        }
    }
}

impl fmt::Debug for SharedStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SharedStream").finish_non_exhaustive()
    }
}

#[async_trait::async_trait]
impl SeekableStream for SharedStream {
    async fn reset(&mut self) -> crate::Result<()> {
        self.inner.lock().await.reset().await
    }

    fn len(&self) -> Option<u64> {
        self.len
    }
}

impl AsyncRead for SharedStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        // try_lock always succeeds in practice: SharedStream has a single logical reader
        // at any given time (all clones share position; concurrent reads would race anyway).
        match self.get_mut().inner.try_lock() {
            Some(mut guard) => Pin::new(&mut **guard).poll_read(cx, buf),
            None => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}

/// Extension trait to wrap any [`futures::io::AsyncRead`] source in a [`SharedStream`].
///
/// # Examples
///
/// ```
/// # use typespec_client_core::stream::FuturesAsyncReadExt as _;
/// let data: &[u8] = b"hello world";
/// let stream = data.shared(Some(data.len() as u64));
/// ```
pub trait FuturesAsyncReadExt: Sized {
    /// Wraps `self` in a [`SharedStream`].
    ///
    /// # Arguments
    ///
    /// * `len` - The total byte length of the stream, if known. Used to set the
    ///   `content-length` header when the stream is sent as a request body.
    fn shared(self, len: Option<u64>) -> SharedStream;
}

impl<R> FuturesAsyncReadExt for R
where
    R: AsyncRead + Unpin + Send + Sync + 'static,
{
    fn shared(self, len: Option<u64>) -> SharedStream {
        SharedStream::new(super::ReadStream::new(self, len))
    }
}

/// Extension trait to wrap any [`tokio::io::AsyncRead`] source in a [`SharedStream`].
///
/// # Examples
///
/// ```no_run
/// # #[cfg(feature = "tokio")]
/// # async fn example() {
/// use typespec_client_core::stream::TokioAsyncReadExt as _;
/// let data = std::io::Cursor::new(b"hello".to_vec());
/// let stream = data.shared(Some(5));
/// # }
/// ```
#[cfg(feature = "tokio")]
pub trait TokioAsyncReadExt: Sized {
    /// Wraps `self` in a [`SharedStream`].
    ///
    /// # Arguments
    ///
    /// * `len` - The total byte length of the stream, if known.
    fn shared(self, len: Option<u64>) -> SharedStream;
}

#[cfg(feature = "tokio")]
impl<R> TokioAsyncReadExt for R
where
    R: tokio::io::AsyncRead + Unpin + Send + Sync + 'static,
{
    fn shared(self, len: Option<u64>) -> SharedStream {
        SharedStream::new(super::tokio::ReadStream::new(self, len))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stream::{BytesStream, FuturesAsyncReadExt};
    use futures::io::AsyncReadExt;

    #[tokio::test]
    async fn read_returns_data() {
        let data = b"hello world";
        let mut stream = SharedStream::new(BytesStream::new(data.as_slice()));

        let mut buf = vec![0u8; data.len()];
        let n = stream.read(&mut buf).await.unwrap();
        assert_eq!(n, data.len());
        assert_eq!(&buf, data);
    }

    #[tokio::test]
    async fn clone_shares_arc_not_data() {
        let stream1 = SharedStream::new(BytesStream::new(b"hi".as_slice()));
        let stream2 = stream1.clone();
        assert!(Arc::ptr_eq(&stream1.inner, &stream2.inner));
    }

    #[tokio::test]
    async fn reset_works() {
        let data = b"hello";
        let mut stream = SharedStream::new(BytesStream::new(data.as_slice()));

        let mut buf = vec![0u8; data.len()];
        stream.read_exact(&mut buf).await.unwrap();

        stream.reset().await.unwrap();

        let mut buf2 = vec![0u8; data.len()];
        let n = stream.read(&mut buf2).await.unwrap();
        assert_eq!(n, data.len());
        assert_eq!(&buf2, data);
    }

    #[tokio::test]
    async fn len_returns_correct_value() {
        let data = b"hello world";
        let stream = SharedStream::new(BytesStream::new(data.as_slice()));
        assert_eq!(stream.len(), Some(data.len() as u64));
        assert!(matches!(stream.is_empty(), Some(b) if !b));
    }

    #[tokio::test]
    async fn len_none_when_unknown() {
        let s = SharedStream::new(crate::stream::ReadStream::new(b"hi".as_ref(), None));
        assert_eq!(s.len(), None);
    }

    #[tokio::test]
    async fn futures_async_read_ext() {
        let data = b"hello via ext trait";
        let mut stream = BytesStream::new(data.as_slice()).shared(Some(data.len() as u64));

        assert_eq!(stream.len(), Some(data.len() as u64));

        let mut buf = vec![0u8; data.len()];
        let n = stream.read(&mut buf).await.unwrap();
        assert_eq!(n, data.len());
        assert_eq!(&buf, data);
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn tokio_async_read_ext() {
        use super::TokioAsyncReadExt;

        let data = b"hello from tokio";
        // Use UFCS to avoid ambiguity: std::io::Cursor implements both AsyncRead traits.
        let mut stream =
            TokioAsyncReadExt::shared(std::io::Cursor::new(data.to_vec()), Some(data.len() as u64));

        assert_eq!(stream.len(), Some(data.len() as u64));

        let mut buf = vec![0u8; data.len()];
        let n = stream.read(&mut buf).await.unwrap();
        assert_eq!(n, data.len());
        assert_eq!(&buf, data);
    }
}
