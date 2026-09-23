// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod encode_in_place;
mod encode_streaming;
mod smv1;

use azure_core::{http::Body, Result};

const ENCODE_SEGMENT_LENGTH_USIZE: usize = 4 * 1024 * 1024;
const ENCODE_SEGMENT_LENGTH_U64: u64 = ENCODE_SEGMENT_LENGTH_USIZE as u64;

/// Encodes a [Body] into a structured message using the crc 64 nvme checksum flag.
/// A precalculated crc for the full message may be optionally provided, skipping re-compute.
pub fn encode_with_checksum(content: Body, crc_64_nvme: Option<u64>) -> Result<Body> {
    if let Some(crc) = crc_64_nvme {
        Ok(Body::SeekableStream(Box::new(
            encode_in_place::wrap_body_with_structured_message(content, crc)?,
        )))
    } else {
        match content {
            Body::Bytes(bytes) => Ok(Body::SeekableStream(Box::new(
                encode_in_place::encode_bytes_in_structured_message(
                    bytes,
                    ENCODE_SEGMENT_LENGTH_USIZE,
                ),
            ))),
            Body::SeekableStream(seekable_stream) => Ok(Body::SeekableStream(Box::new(
                encode_streaming::SeekableStructuredMessageEncodingStream::new(
                    seekable_stream,
                    ENCODE_SEGMENT_LENGTH_U64,
                )?,
            ))),
        }
    }
}

const fn derive_structured_message_length(content_len: u64, segment_len: u64) -> u64 {
    const CRC_64_LEN: u64 = 8;
    content_len
        + smv1::STREAM_HEADER_LENGTH as u64
        + (content_len.div_ceil(segment_len)) * (smv1::SEGMENT_HEADER_LENGTH as u64 + CRC_64_LEN)
        + CRC_64_LEN
}

#[cfg(test)]
mod tests {
    use std::{pin::pin, task::Poll};

    use azure_core::{error::ErrorKind, stream::SeekableStream, Error};
    use futures::AsyncRead;

    use super::*;

    #[derive(Clone, Debug)]
    pub struct SeekableStreamHideLen {
        pub inner: Box<dyn SeekableStream>,
    }
    impl AsyncRead for SeekableStreamHideLen {
        fn poll_read(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
            buf: &mut [u8],
        ) -> Poll<std::io::Result<usize>> {
            pin!(self.get_mut().inner.as_mut()).poll_read(cx, buf)
        }
    }

    #[async_trait::async_trait]
    impl SeekableStream for SeekableStreamHideLen {
        fn len(&self) -> Option<u64> {
            None
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
