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
        let dst = self.spare_capacity_mut();
        let count = stream.read(dst).await?;
        let new_len = self.len() + count;

        if new_len <= self.capacity() {
            // SAFETY: This is being performed immediately after the length is proven
            // to be within capacity.
            unsafe { self.set_len(new_len) };
            Ok(count)
        } else {
            // The AsyncRead implementation returned an impossible byte read count.
            // Whether through a bug or through malice, it can't be trusted.
            Err(Error::with_message(
                ErrorKind::Io,
                "AsyncRead::read returned an impossible byte read count.",
            ))
        }
    }
    async fn read_exactly_from<R>(&mut self, stream: &mut R, amount: usize) -> Result<usize>
    where
        R: AsyncRead + Unpin + Send,
    {
        let mut total_read = 0;
        while total_read < amount {
            let to_read = min(self.remaining(), amount.saturating_sub(total_read));
            let dst = &mut self.spare_capacity_mut()[..to_read];
            let count = stream.read(dst).await?;
            if count == 0 {
                return Ok(total_read);
            }
            total_read += count;
            let new_len = self.len() + count;

            if new_len <= self.capacity() {
                // SAFETY: This is being performed immediately after the length is proven
                // to be within capacity.
                unsafe { self.set_len(new_len) };
            } else {
                // The AsyncRead implementation returned an impossible byte read count.
                // Whether through a bug or through malice, it can't be trusted.
                return Err(Error::with_message(
                    ErrorKind::Io,
                    "AsyncRead::read returned an impossible byte read count.",
                ));
            }
        }

        Ok(total_read)
    }
}
