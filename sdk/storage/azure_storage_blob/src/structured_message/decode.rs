// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::cmp::min;

use async_stream::try_stream;
use azure_core::{error::ErrorKind, Error, Result};
use bytes::{Bytes, BytesMut};
use crc_fast::{CrcAlgorithm, Digest};
use futures::{AsyncRead, AsyncReadExt, Stream};

use crate::streams::{CountingAsyncRead, DigestAsyncRead, ErrorAfterLimitAsyncRead};

use super::smv1;

/// The maximum buffer allocation size for reading segment content.
const BUF_ALLOCATION_LIMIT: usize = 8 * 1024;

pub fn decode<R>(reader: R) -> impl Stream<Item = Result<Bytes>>
where
    R: AsyncRead + Unpin + Send,
{
    try_stream! {
        let mut reader = CountingAsyncRead::new(reader);

        let stream_header = smv1::StreamHeader::parse(
            &reader.read_exact_inline_stack::<{smv1::StreamHeader::LENGTH}>().await?)?;

        // uses the inner CountingAsyncRead to track, ensuring message header is counted for this limit
        // even though limit was only discovered within that header
        let mut reader = ErrorAfterLimitAsyncRead::new(
            reader,
            stream_header.message_len,
            || std::io::Error::new(std::io::ErrorKind::Other, Error::with_message(ErrorKind::DataConversion,
                format!("Structured body invalid. Exceeded defined length {}.", stream_header.message_len))),
        );

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
                let remaining_segment_content = segment_header.content_length - segment_content_reader.bytes_read();
                if remaining_segment_content == 0 {
                    break;
                }

                let mut buf = BytesMut::zeroed(min(BUF_ALLOCATION_LIMIT, remaining_segment_content.try_into().unwrap_or(usize::MAX)));
                let bytes_read = segment_content_reader.read(&mut buf).await?;
                buf.truncate(bytes_read);

                if bytes_read == 0 {
                    Err(Error::new(ErrorKind::Io, "Premature EOF while reading segment content"))?;
                }

                yield buf.freeze();
            }
            reader = segment_content_reader.into_inner().into_inner();

            // process segment footer
            if let Some(d) = segment_content_digest {
                read_and_validate_checksum(&mut reader, d.finalize()).await?;
            }
        }

        // process stream footer
        if let Some(d) = overall_content_digest {
            read_and_validate_checksum(&mut reader, d.finalize()).await?;
        }

        // structured message complete, ensure we have read exactly the expected number of bytes
        if reader.bytes_read() != stream_header.message_len {
            Err(Error::with_message(ErrorKind::DataConversion, "Structured message decode complete before encoded message len was fully read."))?;
        }
        // ensure no bytes remain in the stream
        let mut buf = [0u8; 1];
        if reader.read(&mut buf).await? != 0 {
            Err(Error::with_message(ErrorKind::DataConversion, "Stream data encountered after structured message was fully read."))?;
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
        Err(Error::with_message(
            ErrorKind::DataConversion,
            "Checksum mismatch",
        ))?
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

#[cfg(test)]
mod tests {
    use std::{
        ops::{Range, RangeFrom, RangeTo},
        pin::pin,
    };

    use azure_core::stream::BytesStream;
    use futures::TryStreamExt;
    use rand::random;

    use crate::structured_message::{
        derive_structured_message_length, encode_in_place::encode_bytes_in_structured_message,
    };

    use super::*;

    #[tokio::test]
    async fn test_single_segment_decode() {
        let data = random::<[u8; 1024]>().to_vec();
        let structured_body = [
            smv1::StreamHeader {
                message_len: derive_structured_message_length(data.len() as u64, u64::MAX),
                flags: smv1::Flags::CRC_64_NVME,
                segment_count: 1,
            }
            .to_vec(),
            smv1::SegmentHeader {
                segment_number: 0,
                content_length: data.len() as u64,
            }
            .to_vec(),
            data.clone(),
            crc_inline(&data).to_le_bytes().to_vec(),
            crc_inline(&data).to_le_bytes().to_vec(),
        ]
        .concat();

        let decoded_data = decode(BytesStream::new(structured_body))
            .map_ok(|bytes| bytes.to_vec())
            .try_concat()
            .await
            .unwrap();
        assert_eq!(decoded_data, data);
    }

    #[tokio::test]
    async fn test_multi_segment_decode() {
        const SEGMENT_LEN: usize = 501;
        const SEG_1_RANGE: RangeTo<usize> = ..SEGMENT_LEN;
        const SEG_2_RANGE: Range<usize> = SEGMENT_LEN..SEGMENT_LEN * 2;
        const SEG_3_RANGE: RangeFrom<usize> = SEGMENT_LEN * 2..;

        let data = random::<[u8; 1024]>().to_vec();

        let seg_1 = [
            smv1::SegmentHeader {
                segment_number: 0,
                content_length: SEGMENT_LEN as u64,
            }
            .to_vec(),
            data[SEG_1_RANGE].to_vec(),
            crc_inline(&data[SEG_1_RANGE]).to_le_bytes().to_vec(),
        ]
        .concat();

        let seg_2 = [
            smv1::SegmentHeader {
                segment_number: 1,
                content_length: SEGMENT_LEN as u64,
            }
            .to_vec(),
            data[SEG_2_RANGE].to_vec(),
            crc_inline(&data[SEG_2_RANGE]).to_le_bytes().to_vec(),
        ]
        .concat();

        let seg_3 = [
            smv1::SegmentHeader {
                segment_number: 2,
                content_length: (data.len() - SEGMENT_LEN * 2) as u64,
            }
            .to_vec(),
            data[SEG_3_RANGE].to_vec(),
            crc_inline(&data[SEG_3_RANGE]).to_le_bytes().to_vec(),
        ]
        .concat();

        let structured_body = [
            smv1::StreamHeader {
                message_len: derive_structured_message_length(
                    data.len() as u64,
                    SEGMENT_LEN as u64,
                ),
                flags: smv1::Flags::CRC_64_NVME,
                segment_count: 3,
            }
            .to_vec(),
            seg_1,
            seg_2,
            seg_3,
            crc_inline(&data).to_le_bytes().to_vec(),
        ]
        .concat();

        let decoded_data = decode(BytesStream::new(structured_body))
            .map_ok(|bytes| bytes.to_vec())
            .try_concat()
            .await
            .unwrap();
        assert_eq!(decoded_data, data);
    }

    #[tokio::test]
    async fn test_detect_bad_version() {
        let data = random::<[u8; 1024]>().to_vec();
        let mut structured_body = encode_bytes_in_structured_message(data.into(), 1024).concat();
        structured_body[0] = 0xFF;
        assert!(pin!(decode(BytesStream::new(structured_body)))
            .try_next()
            .await
            .is_err());
    }

    #[tokio::test]
    async fn test_detect_incorrectly_encoded_length() {
        let data = random::<[u8; 1024]>().to_vec();
        let mut structured_body = encode_bytes_in_structured_message(data.into(), 1024).concat();

        let tampered_header = smv1::StreamHeader {
            message_len: 99999,
            flags: smv1::Flags::CRC_64_NVME,
            segment_count: 1,
        };
        structured_body.splice(..smv1::StreamHeader::LENGTH, tampered_header.to_vec());

        assert!(pin!(decode(BytesStream::new(structured_body)))
            .map_ok(|bytes| bytes.to_vec())
            .try_concat()
            .await
            .is_err());
    }

    #[tokio::test]
    async fn test_detect_invalid_flags() {
        let data = random::<[u8; 1024]>().to_vec();
        let mut structured_body =
            encode_bytes_in_structured_message(data.clone().into(), 1024).concat();

        let tampered_header = smv1::StreamHeader {
            message_len: derive_structured_message_length(data.len() as u64, 1024),
            flags: smv1::Flags::from_bits_retain(0xFFFF),
            segment_count: 1,
        };
        structured_body.splice(..smv1::StreamHeader::LENGTH, tampered_header.to_vec());

        assert!(pin!(decode(BytesStream::new(structured_body)))
            .map_ok(|bytes| bytes.to_vec())
            .try_concat()
            .await
            .is_err());
    }

    #[tokio::test]
    async fn test_detect_valid_inaccurate_flags() {
        let data = random::<[u8; 1024]>().to_vec();
        let mut structured_body =
            encode_bytes_in_structured_message(data.clone().into(), 1024).concat();

        let tampered_header = smv1::StreamHeader {
            message_len: derive_structured_message_length(data.len() as u64, 1024),
            flags: smv1::Flags::NONE,
            segment_count: 1,
        };
        structured_body.splice(..smv1::StreamHeader::LENGTH, tampered_header.to_vec());

        assert!(pin!(decode(BytesStream::new(structured_body)))
            .map_ok(|bytes| bytes.to_vec())
            .try_concat()
            .await
            .is_err());
    }

    #[tokio::test]
    async fn test_detect_incorrectly_encoded_segment_count() {
        let data = random::<[u8; 1024]>().to_vec();
        let mut structured_body =
            encode_bytes_in_structured_message(data.clone().into(), 1024).concat();

        let tampered_header = smv1::StreamHeader {
            message_len: derive_structured_message_length(data.len() as u64, 1024),
            flags: smv1::Flags::CRC_64_NVME,
            segment_count: 9999,
        };
        structured_body.splice(..smv1::StreamHeader::LENGTH, tampered_header.to_vec());

        assert!(pin!(decode(BytesStream::new(structured_body)))
            .map_ok(|bytes| bytes.to_vec())
            .try_concat()
            .await
            .is_err());
    }

    fn crc_inline(data: &[u8]) -> u64 {
        let mut digest = Digest::new(CrcAlgorithm::Crc64Nvme);
        digest.update(&data);
        digest.finalize()
    }
}
