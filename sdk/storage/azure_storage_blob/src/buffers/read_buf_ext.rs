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
