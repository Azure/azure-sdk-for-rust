// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{cmp::min, io::Write};

use async_trait::async_trait;
use azure_core::{error::ErrorKind, Error, Result};
use futures::{AsyncRead, AsyncReadExt};

use crate::buffers::read_buf::ReadBuf;

/// Extensions for `ReadBuf`.
#[async_trait]
pub(crate) trait ReadBufExt {
    fn extend_from_slice(&mut self, data: &[u8]) -> Result<()>;
    async fn read_from<R>(&mut self, read: &mut R) -> Result<usize>
    where
        R: AsyncRead + Unpin + Send;
    async fn read_exactly_from<R>(&mut self, read: &mut R, amount: usize) -> Result<usize>
    where
        R: AsyncRead + Unpin + Send;
}

#[async_trait]
impl ReadBufExt for ReadBuf {
    fn extend_from_slice(&mut self, mut data: &[u8]) -> Result<()> {
        while !data.is_empty() {
            let remaining = self.capacity() - self.len();
            if remaining < data.len() {
                self.extend_zeroed(data.len() - remaining);
            }
            let mut dst = self.spare_capacity_mut();
            let written = dst.write(data)?;
            // SAFETY: We trust values returned from std::io::impls.
            unsafe { self.set_len(self.len() + written) };
            data = &data[written..];
        }
        Ok(())
    }

    async fn read_from<R>(&mut self, stream: &mut R) -> Result<usize>
    where
        R: AsyncRead + Unpin + Send,
    {
        let count = validated_read(self.spare_capacity_mut(), stream).await?;
        // SAFETY: `count` has been validated as <= `self.spare_capacity_mut().len()`
        unsafe { self.set_len(self.len() + count) };
        Ok(count)
    }
    async fn read_exactly_from<R>(&mut self, stream: &mut R, amount: usize) -> Result<usize>
    where
        R: AsyncRead + Unpin + Send,
    {
        let mut total_read = 0;
        while total_read < amount {
            let to_read = min(self.remaining(), amount.saturating_sub(total_read));
            let dst = &mut self.spare_capacity_mut()[..to_read];
            let count = validated_read(dst, stream).await?;
            // SAFETY: `count` has been validated as <= `dst.len()`, a slice of `self.spare_capacity_mut()`
            unsafe { self.set_len(self.len() + count) };

            if count == 0 {
                return Ok(total_read);
            }
            total_read += count;
        }

        Ok(total_read)
    }
}

/// Performs a read into the given slice.
/// Validates that the count returned is accurate to the buffer provided.
///
/// # Returns
///
/// - Err if `stream.read(buf).await?` > `buf.len()`
/// - Otherwise, the `Result` of `stream.read(buf).await`
async fn validated_read<R>(buf: &mut [u8], stream: &mut R) -> Result<usize>
where
    R: AsyncRead + Unpin,
{
    let count = stream.read(buf).await?;
    if count > buf.len() {
        return Err(Error::with_message(
            ErrorKind::Io,
            "AsyncRead::read returned an impossible byte read count.",
        ));
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use std::task::Poll;

    use super::*;

    struct MockAsyncRead {
        count: Option<usize>,
    }
    impl AsyncRead for MockAsyncRead {
        fn poll_read(
            self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
            _buf: &mut [u8],
        ) -> std::task::Poll<std::io::Result<usize>> {
            Poll::Ready(
                self.count
                    .ok_or_else(|| std::io::Error::other("mock error")),
            )
        }
    }

    #[tokio::test]
    async fn validated_read_success() {
        for (slice_len, async_read_count) in [
            (0, 0),
            (1, 0),
            (1, 1),
            (1024, 1024),
            (12345, 1024),
            (12345, 123),
        ] {
            // sanity check we're testing the right thing
            assert!(slice_len >= async_read_count);

            let mut buf = vec![0; slice_len];
            let mut async_read = MockAsyncRead {
                count: Some(async_read_count),
            };
            assert_eq!(
                validated_read(&mut buf, &mut async_read).await.unwrap(),
                async_read_count
            )
        }
    }

    #[tokio::test]
    async fn validated_read_propagates_error() {
        let mut buf = vec![0; 1024];
        let mut async_read = MockAsyncRead { count: None };
        assert_eq!(
            validated_read(&mut buf, &mut async_read)
                .await
                .unwrap_err()
                .to_string(),
            "mock error"
        )
    }

    #[tokio::test]
    async fn validated_read_detects_bad_count() {
        for (slice_len, async_read_count) in [(0, 1), (1, 2), (1024, 1234), (123, 12345)] {
            // sanity check we're testing the right thing
            assert!(slice_len < async_read_count);

            let mut buf = vec![0; slice_len];
            let mut async_read = MockAsyncRead {
                count: Some(async_read_count),
            };
            assert!(validated_read(&mut buf, &mut async_read).await.is_err())
        }
    }
}
