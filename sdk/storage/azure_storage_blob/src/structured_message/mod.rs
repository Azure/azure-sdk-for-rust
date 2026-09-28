// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod decode;
mod encode_in_place;
mod encode_streaming;
mod smv1;

use std::num::NonZero;

use azure_core::{http::Body, Result};
use bytes::Bytes;

const ENCODE_SEGMENT_LENGTH_USIZE: NonZero<usize> = NonZero::new(4 * 1024 * 1024).unwrap();
const ENCODE_SEGMENT_LENGTH_U64: NonZero<u64> =
    NonZero::new(ENCODE_SEGMENT_LENGTH_USIZE.get() as u64).unwrap();

/// Encodes a [Body] into a structured message using the crc 64 nvme checksum flag.
/// A precalculated crc for the full message may be optionally provided, skipping re-compute.
pub fn encode_with_checksum(content: Body, crc_64_nvme: Option<u64>) -> Result<Body> {
    if let Some(crc) = crc_64_nvme {
        return Ok(Body::SeekableStream(Box::new(
            encode_in_place::wrap_body_with_structured_message(content, crc)?,
        )));
    }
    if let Some(0) = content.len() {
        return Ok(Body::SeekableStream(Box::new(
            encode_in_place::encode_bytes_in_structured_message(
                Bytes::new(),
                ENCODE_SEGMENT_LENGTH_USIZE,
            ),
        )));
    }
    match content {
        Body::Bytes(bytes) => Ok(Body::SeekableStream(Box::new(
            encode_in_place::encode_bytes_in_structured_message(bytes, ENCODE_SEGMENT_LENGTH_USIZE),
        ))),
        Body::SeekableStream(seekable_stream) => Ok(Body::SeekableStream(Box::new(
            encode_streaming::SeekableStructuredMessageEncodingStream::new(
                seekable_stream,
                ENCODE_SEGMENT_LENGTH_U64,
            )?,
        ))),
    }
}

const fn derive_structured_message_length(content_len: u64, segment_len: NonZero<u64>) -> u64 {
    const CRC_64_LEN: u64 = 8;
    // manual impl instead of min() to allow for const fn
    let num_segments = if content_len == 0 {
        1
    } else {
        content_len.div_ceil(segment_len.get())
    };
    content_len
        + smv1::STREAM_HEADER_LENGTH as u64
        + num_segments * (smv1::SEGMENT_HEADER_LENGTH as u64 + CRC_64_LEN)
        + CRC_64_LEN
}

#[cfg(test)]
mod tests {
    use std::{pin::pin, task::Poll};

    use azure_core::{error::ErrorKind, stream::SeekableStream, Error};
    use futures::AsyncRead;

    use super::*;

    #[derive(Clone, Debug)]
    pub struct SeekableStreamOverrideLen {
        pub inner: Box<dyn SeekableStream>,
        pub len_override: Option<u64>,
    }
    impl AsyncRead for SeekableStreamOverrideLen {
        fn poll_read(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
            buf: &mut [u8],
        ) -> Poll<std::io::Result<usize>> {
            pin!(self.get_mut().inner.as_mut()).poll_read(cx, buf)
        }
    }

    #[async_trait::async_trait]
    impl SeekableStream for SeekableStreamOverrideLen {
        fn len(&self) -> Option<u64> {
            self.len_override
        }
        async fn reset(&mut self) -> Result<()> {
            self.inner.reset().await
        }
    }

    #[derive(Clone, Debug)]
    pub struct SeekableStreamFailReset {
        pub inner: Box<dyn SeekableStream>,
    }
    impl AsyncRead for SeekableStreamFailReset {
        fn poll_read(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
            buf: &mut [u8],
        ) -> Poll<std::io::Result<usize>> {
            pin!(self.get_mut().inner.as_mut()).poll_read(cx, buf)
        }
    }

    #[async_trait::async_trait]
    impl SeekableStream for SeekableStreamFailReset {
        fn len(&self) -> Option<u64> {
            self.inner.len()
        }
        async fn reset(&mut self) -> Result<()> {
            Err(Error::with_message(ErrorKind::Io, "Stream reset blocked."))
        }
    }
}
