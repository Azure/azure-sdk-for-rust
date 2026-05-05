// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::io::Write;

use azure_core::{error::ErrorKind, Error, Result};
use futures::{AsyncRead, AsyncReadExt};

use crate::buffers::read_buf::ReadBuf;

/// Extensions for `ReadBuf`.
pub(crate) trait ReadBufExt {
    fn extend_from_slice(&mut self, data: &[u8]) -> Result<()>;
    async fn read_from<R>(&mut self, read: &mut R) -> Result<usize>
    where
        R: AsyncRead + Unpin;
}

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

    async fn read_from<R>(&mut self, read: &mut R) -> Result<usize>
    where
        R: AsyncRead + Unpin,
    {
        let dst = self.spare_capacity_mut();
        let count = read.read(dst).await?;
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
}
