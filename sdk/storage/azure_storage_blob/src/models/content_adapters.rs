// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::future::Future;

use azure_core::stream::SeekableStream;
use bytes::Bytes;
use futures::{AsyncRead, AsyncReadExt};

pub enum StorageUploadBody {
    Bytes(Bytes),
    AsyncRead(Box<dyn AsyncReadWithLenHint>),
}

impl StorageUploadBody {
    pub fn len(&self) -> Option<u64> {
        match self {
            Self::Bytes(bytes) => Some(bytes.len() as u64),
            Self::AsyncRead(read) => read.len(),
        }
    }
    pub fn is_empty(&self) -> Option<bool> {
        self.len().map(|len| len == 0)
    }
}

impl<B> From<B> for StorageUploadBody
where
    B: Into<Bytes>,
{
    fn from(bytes: B) -> Self {
        Self::Bytes(bytes.into())
    }
}

impl From<&StorageUploadBody> for Bytes {
    fn from(value: &StorageUploadBody) -> Self {
        match value {
            StorageUploadBody::Bytes(bytes) => bytes.clone(),
            StorageUploadBody::AsyncRead(_) => unimplemented!(),
        }
    }
}

impl From<Box<dyn AsyncReadWithLenHint>> for StorageUploadBody {
    fn from(async_read: Box<dyn AsyncReadWithLenHint>) -> Self {
        Self::AsyncRead(async_read)
    }
}

pub trait AsyncReadWithLenHint: AsyncRead + Send + Sync + Unpin {
    fn len(&self) -> Option<u64>;
    fn is_empty(&self) -> Option<bool> {
        self.len().map(|len| len == 0)
    }
}

impl<T: SeekableStream> AsyncReadWithLenHint for T {
    fn len(&self) -> Option<u64> {
        self.len()
    }
}

struct AsyncReadLenHintWrapper<T> {
    pub async_read: T,
    pub len_hint: Option<u64>,
}
impl<T: AsyncRead + Unpin> AsyncRead for AsyncReadLenHintWrapper<T> {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        let fut = self.async_read.read(buf);
        futures::pin_mut!(fut);
        fut.poll(cx)
    }
}
impl<T: AsyncRead + Send + Sync + Unpin> AsyncReadWithLenHint for AsyncReadLenHintWrapper<T> {
    fn len(&self) -> Option<u64> {
        self.len_hint
    }
}

pub trait AsyncReadLenExt {
    fn with_len_hint(self, len_hint: Option<u64>) -> impl AsyncReadWithLenHint;
}

impl<T> AsyncReadLenExt for T
where
    T: AsyncRead + Send + Sync + Unpin,
{
    fn with_len_hint(self, len_hint: Option<u64>) -> impl AsyncReadWithLenHint {
        AsyncReadLenHintWrapper {
            async_read: self,
            len_hint,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::stream::SeekableStream;

    /// Sanity check that types do as expected.
    fn seekable_stream_convert<T>(stream: T) -> StorageUploadBody
    where
        T: SeekableStream + 'static,
    {
        StorageUploadBody::AsyncRead(Box::new(stream))
    }

    /// Sanity check that types do as expected.
    fn async_read_convert<T>(stream: T) -> StorageUploadBody
    where
        T: AsyncRead + Clone + std::fmt::Debug + Send + Sync + Unpin + 'static,
    {
        StorageUploadBody::AsyncRead(Box::new(stream.with_len_hint(None)))
    }
}
