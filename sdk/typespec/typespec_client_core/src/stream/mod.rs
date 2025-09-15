// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Asynchronous streams.

mod bytes_stream;

use crate::{
    error::{Error, ErrorKind, Result},
    Bytes,
};
pub use bytes_stream::*;
use dyn_clone::DynClone;
use futures::{io::AsyncRead, stream::Stream, task::Poll};
use std::{pin::Pin, task::Context};

/// Amount of the stream to buffer in memory during streaming uploads.
pub const DEFAULT_BUFFER_SIZE: usize = 1024 * 64;

/// Enable a type implementing `AsyncRead` to be consumed as if it were a `Stream` of `Bytes`.
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
pub trait SeekableStream: AsyncRead + Unpin + std::fmt::Debug + Send + Sync + DynClone {
    /// Resets the stream position to the beginning.
    async fn reset(&mut self) -> Result<()>;

    /// Returns the total length of the stream in bytes.
    fn len(&self) -> usize;

    /// Returns `true` if the stream is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the size of the buffer to use when reading from the stream.
    fn buffer_size(&self) -> usize {
        DEFAULT_BUFFER_SIZE
    }
}

dyn_clone::clone_trait_object!(SeekableStream);

impl Stream for dyn SeekableStream {
    type Item = Result<Bytes>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut buffer = vec![0_u8; self.buffer_size()];

        match self.poll_read(cx, &mut buffer) {
            Poll::Ready(Ok(0)) => Poll::Ready(None),
            Poll::Ready(Ok(bytes_read)) => {
                let bytes: Bytes = buffer.into();
                let bytes = bytes.slice(0..bytes_read);
                Poll::Ready(Some(Ok(bytes)))
            }
            Poll::Ready(Err(err)) => Poll::Ready(Some(Err(Error::full(
                ErrorKind::Io,
                err,
                "an error was encountered when trying to read from a stream",
            )))),
            Poll::Pending => Poll::Pending,
        }
    }
}
