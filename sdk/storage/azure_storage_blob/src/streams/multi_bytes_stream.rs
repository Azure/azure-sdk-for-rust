// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{cmp::min, task::Poll};

use async_trait::async_trait;
use azure_core::stream::SeekableStream;
use bytes::Bytes;
use futures::AsyncRead;

#[derive(Clone, Debug, Default)]
pub(crate) struct MultiBytesStream {
    buffers: Vec<Bytes>,
    vec_cursor: usize,
    bytes_cursor: usize,
}

impl MultiBytesStream {
    pub(crate) fn new<Iter: IntoIterator<Item = Bytes>>(data: Iter) -> Self {
        Self {
            buffers: data.into_iter().collect(),
            ..Default::default()
        }
    }
}

impl AsyncRead for MultiBytesStream {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
        mut buf: &mut [u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        let this = self.get_mut();
        let mut total_copied = 0;
        while !buf.is_empty() {
            let bytes = match this.buffers.get(this.vec_cursor) {
                Some(bytes) => bytes.slice(this.bytes_cursor..),
                None => break,
            };
            if bytes.is_empty() {
                this.vec_cursor += 1;
                this.bytes_cursor = 0;
                continue;
            }

            let copy = min(buf.len(), bytes.len());
            buf[..copy].copy_from_slice(&bytes[..copy]);

            buf = &mut buf[copy..];
            this.bytes_cursor += copy;
            total_copied += copy;
        }
        Poll::Ready(Ok(total_copied))
    }
}

#[async_trait]
impl SeekableStream for MultiBytesStream {
    async fn reset(&mut self) -> azure_core::Result<()> {
        self.vec_cursor = 0;
        self.bytes_cursor = 0;
        Ok(())
    }

    fn len(&self) -> u64 {
        self.buffers
            .iter()
            .map(|bytes| bytes.len() as u64)
            .sum::<u64>()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
