// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::cmp::min;

use async_stream::try_stream;
use azure_core::{error::ErrorKind, Error, Result};
use bytes::{Bytes, BytesMut};
use crc_fast::{CrcAlgorithm, Digest};
use futures::{AsyncRead, AsyncReadExt, Stream};

use crate::streams::{CountingAsyncRead, DigestAsyncRead};

use super::smv1;

pub fn decode<R>(reader: R) -> impl Stream<Item = Result<Bytes>>
where
    R: AsyncRead + Unpin + Send,
{
    try_stream! {
        let mut reader = CountingAsyncRead::new(reader);

        let stream_header = smv1::StreamHeader::parse(
            &reader.read_exact_inline_stack::<{smv1::StreamHeader::LENGTH}>().await?)?;

        let mut overall_content_digest = if stream_header.flags.contains(smv1::Flags::CRC_64_NVME) {
            Some(Digest::new(CrcAlgorithm::Crc64Nvme))
        } else {
            None
        };

        for seg_idx in 0..stream_header.segment_count {
            let segment_header = smv1::SegmentHeader::parse(
                &reader.read_exact_inline_stack::<{smv1::SegmentHeader::LENGTH}>().await?)?;
            assert_segment_idx(seg_idx, segment_header.segment_number)?;

            let mut segment_content_digest = if stream_header.flags.contains(smv1::Flags::CRC_64_NVME) {
                Some(Digest::new(CrcAlgorithm::Crc64Nvme))
            } else {
                None
            };

            let mut segment_content_reader = CountingAsyncRead::new(DigestAsyncRead::new(
                reader,
                [&mut overall_content_digest, &mut segment_content_digest]
                    .into_iter().filter_map(|d| d.as_mut())
            ));
            loop {
                const BUF_LIMIT: usize = 8 * 1024;
                let remaining_segment_content = segment_header.content_length - segment_content_reader.bytes_read();
                if remaining_segment_content == 0 {
                    break;
                }

                let mut buf = BytesMut::zeroed(min(BUF_LIMIT, remaining_segment_content.try_into().unwrap_or(usize::MAX)));
                let bytes_read = segment_content_reader.read(&mut buf).await?;
                buf.truncate(bytes_read);

                if bytes_read == 0 {
                    todo!("error on premature eof")
                }

                yield buf.freeze();
            }
            reader = segment_content_reader.into_inner().into_inner();

            // process footer
            if let Some(d) = segment_content_digest {
                read_and_validate_checksum(&mut reader, d.finalize()).await?;
            }
        }

        // process stream footer
        if let Some(d) = overall_content_digest {
            read_and_validate_checksum(&mut reader, d.finalize()).await?;
        }

        // ensure EOF in expected location
        if reader.bytes_read() != stream_header.message_len {
            todo!("structured message complete before expected stream len");
        }
        let mut buf = [0u8; 1];
        if reader.read(&mut buf).await? != 0 {
            todo!("error expected EOF");
        }
    }
}

/// Reads a 64 bit checksum from the stream and validate it is the expected value.
/// Returns the number of bytes read from the stream.
async fn read_and_validate_checksum<R: AsyncRead + Unpin>(
    structured_message_stream: &mut R,
    expected_checksum: u64,
) -> Result<()> {
    let mut buf = [0u8; 8];
    structured_message_stream.read_exact(&mut buf).await?;
    if u64::from_le_bytes(buf) != expected_checksum {
        todo!("error crc mismatch")
    }
    Ok(())
}

fn assert_segment_idx(expected: u16, actual: u16) -> Result<()> {
    if expected == actual {
        Ok(())
    } else {
        Err(Error::with_message(
            ErrorKind::DataConversion,
            format!(
                "Unexpected segment number in structured body. Expected {expected}, got {actual}."
            ),
        ))
    }
}

#[async_trait::async_trait]
trait AsyncReadInlineExt {
    /// Reads exactly `LEN` bytes from the stream into a stack-allocated buffer and returns it.
    /// See `AsyncReadExt::read_exact` for more details.
    async fn read_exact_inline_stack<const LEN: usize>(&mut self) -> std::io::Result<[u8; LEN]>;
}

#[async_trait::async_trait]
impl<R: AsyncRead + Unpin + Send> AsyncReadInlineExt for R {
    async fn read_exact_inline_stack<const LEN: usize>(&mut self) -> std::io::Result<[u8; LEN]> {
        let mut buf = [0u8; LEN];
        self.read_exact(&mut buf).await?;
        Ok(buf)
    }
}
