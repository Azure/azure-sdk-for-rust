// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub(crate) mod multi_body_stream;
pub(crate) mod multi_bytes_stream;
pub(crate) mod partitioned_stream;

use std::{
    pin::pin,
    task::{ready, Poll},
};

use crc_fast::Digest;
use futures::AsyncRead;

/// An [AsyncRead] wrapper that updates a count of bytes read.
pub struct CountingAsyncRead<R> {
    inner: R,
    bytes_read: u64,
}

impl<R> CountingAsyncRead<R> {
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            bytes_read: 0,
        }
    }

    pub fn into_inner(self) -> R {
        self.inner
    }

    pub fn bytes_read(&self) -> u64 {
        self.bytes_read
    }
}

impl<R: AsyncRead + Unpin> AsyncRead for CountingAsyncRead<R> {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        let this = self.get_mut();
        let read = ready!(pin!(&mut this.inner).poll_read(cx, buf))?;
        this.bytes_read += read as u64;
        Poll::Ready(Ok(read))
    }
}

pub struct ErrorAfterLimitAsyncRead<R, F> {
    inner: CountingAsyncRead<R>,
    limit: u64,
    error_fn: F,
}

impl<R, F> ErrorAfterLimitAsyncRead<R, F> {
    pub fn new(inner: CountingAsyncRead<R>, limit: u64, error_fn: F) -> Self {
        Self {
            inner,
            limit,
            error_fn,
        }
    }

    pub fn into_inner(self) -> CountingAsyncRead<R> {
        self.inner
    }

    pub fn bytes_read(&self) -> u64 {
        self.inner.bytes_read()
    }
}

impl<R, F> AsyncRead for ErrorAfterLimitAsyncRead<R, F>
where
    R: AsyncRead + Unpin,
    F: Fn() -> std::io::Error + Unpin,
{
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        let this = self.get_mut();
        let read = ready!(pin!(&mut this.inner).poll_read(cx, buf))?;
        if this.inner.bytes_read() > this.limit {
            return Poll::Ready(Err((this.error_fn)()));
        }
        Poll::Ready(Ok(read))
    }
}

/// An [AsyncRead] wrapper that updates one or more digests of bytes read.
pub struct DigestAsyncRead<'a, R> {
    inner: R,
    digests: Vec<&'a mut Digest>,
}

impl<'a, R> DigestAsyncRead<'a, R> {
    pub fn new<I: Iterator<Item = &'a mut Digest>>(inner: R, digests: I) -> Self {
        Self {
            inner,
            digests: digests.collect(),
        }
    }

    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl<'a, R: AsyncRead + Unpin> AsyncRead for DigestAsyncRead<'a, R> {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        let this = self.get_mut();
        let read = ready!(pin!(&mut this.inner).poll_read(cx, buf))?;
        for d in &mut this.digests.iter_mut() {
            d.update(&buf[..read]);
        }
        Poll::Ready(Ok(read))
    }
}
