// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::cmp::min;

use azure_core::{error::ErrorKind, http::Body, stream::SeekableStream, Error, Result};
use bytes::Bytes;
use crc_fast::{CrcAlgorithm, Digest};

use super::{derive_structured_message_length, smv1};
use crate::streams::multi_body_stream::MultiBodyStream;

/// Wraps a body in a single-segment structured message using the CRC 64 NVME feature with the provided crc.
/// This method treats the provided `body` as opaque.
///
/// If `body` has a length, returns a [MultiBodyStream] containing the structured message.
/// Otherwise, returns [Err].
pub fn wrap_body_with_structured_message(
    body: Body,
    crc_64_nvme: u64,
) -> Result<impl SeekableStream> {
    const CRC_64_LEN: u64 = 8;
    let body_len = body
        .len()
        .ok_or_else(|| Error::with_message(ErrorKind::Io, "Failed to get body length."))?;
    let message_len = smv1::STREAM_HEADER_LENGTH as u64
        + smv1::SEGMENT_HEADER_LENGTH as u64
        + body_len
        + CRC_64_LEN
        + CRC_64_LEN;

    let stream_header = smv1::StreamHeader {
        message_len,
        flags: smv1::Flags::CRC_64_NVME,
        segment_count: 1,
    }
    .as_bytes();
    let segment_header = smv1::SegmentHeader {
        segment_number: 0,
        content_length: body_len,
    }
    .as_bytes();
    let segment_footer = Bytes::from(crc_64_nvme.to_le_bytes().to_vec());
    let stream_footer = Bytes::from(crc_64_nvme.to_le_bytes().to_vec());

    Ok(MultiBodyStream::new([
        stream_header.into(),
        segment_header.into(),
        body,
        segment_footer.into(),
        stream_footer.into(),
    ]))
}

/// Encodes a [Bytes] into a structured message using the crc 64 nvme checksum flag.
/// A precalculated crc for the full message may be optionally provided, skipping re-compute.
/// When a precalculated checksum is provided, a single message segment is forced, respecting the skipping of re-compute.
pub fn encode_bytes_in_structured_message(
    content: Bytes,
    segment_len: usize,
) -> impl SeekableStream {
    let content_crc = crc_inline(&content);

    let segments_with_checksums = (0..content.len())
        .step_by(segment_len)
        .map(|offset| {
            let segment = content.slice(offset..min(offset + segment_len, content.len()));
            let crc = crc_inline(&segment);
            (segment, crc)
        })
        .collect::<Vec<_>>();

    let stream_header = smv1::StreamHeader {
        message_len: derive_structured_message_length(content.len() as u64, segment_len as u64),
        flags: smv1::Flags::CRC_64_NVME,
        segment_count: segments_with_checksums.len() as u16,
    }
    .as_bytes();

    // stream header + (header + content + footer)-per-segment + stream footer
    let mut sequence = Vec::with_capacity(1 + segments_with_checksums.len() * 3 + 1);

    sequence.push(stream_header);
    for (i, (segment, crc)) in segments_with_checksums.iter().enumerate() {
        let segment_header = smv1::SegmentHeader {
            segment_number: i as u16,
            content_length: segment.len() as u64,
        }
        .as_bytes();
        let segment_footer = Bytes::from(crc.to_le_bytes().to_vec());

        sequence.push(segment_header);
        sequence.push(segment.clone());
        sequence.push(segment_footer);
    }
    let stream_footer = Bytes::from(content_crc.to_le_bytes().to_vec());
    sequence.push(stream_footer);

    MultiBodyStream::new(sequence.into_iter().map(|bytes| bytes.into()))
}

fn crc_inline(data: &[u8]) -> u64 {
    let mut digest = Digest::new(CrcAlgorithm::Crc64Nvme);
    digest.update(&data);
    digest.finalize()
}

#[cfg(test)]
mod tests {
    use azure_core::stream::BytesStream;
    use futures::AsyncReadExt;

    use crate::structured_message::tests::*;

    use super::*;

    #[tokio::test]
    async fn test_wrap_body_with_structured_message() {
        const DATA_LEN: usize = 1024;
        const TOTAL_STRUCTURED_LEN: usize =
            derive_structured_message_length(DATA_LEN as u64, usize::MAX as u64) as usize;

        let data = rand::random::<[u8; DATA_LEN]>();
        let data_crc = crc_inline(&data);

        for body in [
            Body::Bytes(data.to_vec().into()),
            Body::SeekableStream(Box::new(BytesStream::new(data.to_vec()))),
        ] {
            let mut sm_stream = wrap_body_with_structured_message(body, data_crc).unwrap();

            let mut dst = Vec::new();
            assert_eq!(
                sm_stream.read_to_end(&mut dst).await.unwrap(),
                TOTAL_STRUCTURED_LEN
            );
            assert_eq!(
                &dst[..smv1::STREAM_HEADER_LENGTH],
                smv1::StreamHeader {
                    message_len: TOTAL_STRUCTURED_LEN as u64,
                    flags: smv1::Flags::CRC_64_NVME,
                    segment_count: 1,
                }
                .as_bytes()
            );
            assert_eq!(
                &dst[smv1::STREAM_HEADER_LENGTH
                    ..smv1::STREAM_HEADER_LENGTH + smv1::SEGMENT_HEADER_LENGTH],
                smv1::SegmentHeader {
                    segment_number: 0,
                    content_length: DATA_LEN as u64,
                }
                .as_bytes()
            );
            assert_eq!(
                &dst[smv1::STREAM_HEADER_LENGTH + smv1::SEGMENT_HEADER_LENGTH
                    ..smv1::STREAM_HEADER_LENGTH + smv1::SEGMENT_HEADER_LENGTH + DATA_LEN],
                &data[..],
            );
            assert_eq!(
                &dst[smv1::STREAM_HEADER_LENGTH + smv1::SEGMENT_HEADER_LENGTH + DATA_LEN
                    ..smv1::STREAM_HEADER_LENGTH + smv1::SEGMENT_HEADER_LENGTH + DATA_LEN + 8],
                &data_crc.to_le_bytes()[..],
            );
            assert_eq!(
                &dst[smv1::STREAM_HEADER_LENGTH + smv1::SEGMENT_HEADER_LENGTH + DATA_LEN + 8..],
                &data_crc.to_le_bytes()[..],
            );
        }
    }

    #[test]
    fn test_wrap_body_with_structured_message_len() {
        const DATA_LEN: usize = 1024;
        const TOTAL_STRUCTURED_LEN: usize =
            derive_structured_message_length(DATA_LEN as u64, usize::MAX as u64) as usize;

        let data = rand::random::<[u8; DATA_LEN]>();
        let data_crc = crc_inline(&data);

        let sm_stream =
            wrap_body_with_structured_message(Body::Bytes(data.to_vec().into()), data_crc).unwrap();

        assert_eq!(sm_stream.len().unwrap(), TOTAL_STRUCTURED_LEN as u64);
    }

    #[test]
    fn test_wrap_body_with_structured_message_fails_no_len() {
        const DATA_LEN: usize = 1024;
        const TOTAL_STRUCTURED_LEN: usize =
            derive_structured_message_length(DATA_LEN as u64, usize::MAX as u64) as usize;

        let data = rand::random::<[u8; DATA_LEN]>();
        let data_crc = crc_inline(&data);

        let stream_no_len = SeekableStreamHideLen {
            inner: Box::new(BytesStream::new(data.to_vec())),
        };

        assert!(wrap_body_with_structured_message(
            Body::SeekableStream(Box::new(stream_no_len)),
            data_crc
        )
        .is_err());
    }

    #[tokio::test]
    async fn test_wrap_body_with_structured_message_reset() {
        const DATA_LEN: usize = 1024;

        let data = rand::random::<[u8; DATA_LEN]>();
        let data_crc = crc_inline(&data);

        let mut sm_stream =
            wrap_body_with_structured_message(Body::Bytes(data.to_vec().into()), data_crc).unwrap();

        let mut dst_1 = Vec::new();
        let mut dst_2 = Vec::new();

        sm_stream.read_to_end(&mut dst_1).await.unwrap();
        sm_stream.reset().await.unwrap();
        sm_stream.read_to_end(&mut dst_2).await.unwrap();

        assert_eq!(dst_1, dst_2);
    }

    #[tokio::test]
    async fn test_wrap_body_with_structured_message_reset_fail_propagates() {
        const DATA_LEN: usize = 1024;

        let data = rand::random::<[u8; DATA_LEN]>();
        let data_crc = crc_inline(&data);

        let stream_no_reset = SeekableStreamFailReset {
            inner: Box::new(BytesStream::new(data.to_vec())),
        };

        let mut sm_stream = wrap_body_with_structured_message(
            Body::SeekableStream(Box::new(stream_no_reset)),
            data_crc,
        )
        .unwrap();

        assert!(sm_stream.reset().await.is_err());
    }

    #[tokio::test]
    async fn test_wrap_body_with_structured_message_accepts_any_checksum() {
        const DATA_LEN: usize = 1024;
        const TOTAL_STRUCTURED_LEN: usize =
            derive_structured_message_length(DATA_LEN as u64, usize::MAX as u64) as usize;

        let data = [1u8; DATA_LEN];
        let incorrect_data_crc: u64 = 0x0123456789abcdef;

        let mut sm_stream = wrap_body_with_structured_message(
            Body::Bytes(data.to_vec().into()),
            incorrect_data_crc,
        )
        .unwrap();

        let mut dst = Vec::new();
        assert_eq!(
            sm_stream.read_to_end(&mut dst).await.unwrap(),
            TOTAL_STRUCTURED_LEN
        );
        assert_eq!(
            &dst[smv1::STREAM_HEADER_LENGTH + smv1::SEGMENT_HEADER_LENGTH + DATA_LEN
                ..smv1::STREAM_HEADER_LENGTH + smv1::SEGMENT_HEADER_LENGTH + DATA_LEN + 8],
            &incorrect_data_crc.to_le_bytes()[..],
        );
        assert_eq!(
            &dst[smv1::STREAM_HEADER_LENGTH + smv1::SEGMENT_HEADER_LENGTH + DATA_LEN + 8..],
            &incorrect_data_crc.to_le_bytes()[..],
        );
    }

    #[tokio::test]
    async fn test_encode_bytes_in_structured_message_single_segment() {
        const DATA_LEN: usize = 1024;
        const SEGMENT_LEN: usize = usize::MAX;
        const TOTAL_STRUCTURED_LEN: usize =
            derive_structured_message_length(DATA_LEN as u64, SEGMENT_LEN as u64) as usize;

        let data = rand::random::<[u8; DATA_LEN]>();
        let expected_data_crc = crc_inline(&data);

        let mut sm_stream = encode_bytes_in_structured_message(data.to_vec().into(), SEGMENT_LEN);

        let mut dst = Vec::new();
        assert_eq!(
            sm_stream.read_to_end(&mut dst).await.unwrap(),
            TOTAL_STRUCTURED_LEN
        );
        assert_eq!(
            &dst[..smv1::STREAM_HEADER_LENGTH],
            smv1::StreamHeader {
                message_len: TOTAL_STRUCTURED_LEN as u64,
                flags: smv1::Flags::CRC_64_NVME,
                segment_count: 1,
            }
            .as_bytes()
        );
        assert_eq!(
            &dst[smv1::STREAM_HEADER_LENGTH
                ..smv1::STREAM_HEADER_LENGTH + smv1::SEGMENT_HEADER_LENGTH],
            smv1::SegmentHeader {
                segment_number: 0,
                content_length: DATA_LEN as u64,
            }
            .as_bytes()
        );
        assert_eq!(
            &dst[smv1::STREAM_HEADER_LENGTH + smv1::SEGMENT_HEADER_LENGTH
                ..smv1::STREAM_HEADER_LENGTH + smv1::SEGMENT_HEADER_LENGTH + DATA_LEN],
            &data[..],
        );
        assert_eq!(
            &dst[smv1::STREAM_HEADER_LENGTH + smv1::SEGMENT_HEADER_LENGTH + DATA_LEN
                ..smv1::STREAM_HEADER_LENGTH + smv1::SEGMENT_HEADER_LENGTH + DATA_LEN + 8],
            &expected_data_crc.to_le_bytes()[..],
        );
        assert_eq!(
            &dst[smv1::STREAM_HEADER_LENGTH + smv1::SEGMENT_HEADER_LENGTH + DATA_LEN + 8..],
            &expected_data_crc.to_le_bytes()[..],
        );
    }

    #[tokio::test]
    async fn test_encode_bytes_in_structured_message_multi_segment() {
        const DATA_LEN: usize = 1024;
        const SEGMENT_0_LEN: usize = 999; // results in 2 segments of uneven length
        const TOTAL_STRUCTURED_LEN: usize =
            derive_structured_message_length(DATA_LEN as u64, SEGMENT_0_LEN as u64) as usize;

        let data = rand::random::<[u8; DATA_LEN]>();
        let expected_segment_0_crc = crc_inline(&data[..SEGMENT_0_LEN]);
        let expected_segment_1_crc = crc_inline(&data[SEGMENT_0_LEN..]);
        let expected_data_crc = crc_inline(&data);

        let mut sm_stream = encode_bytes_in_structured_message(data.to_vec().into(), SEGMENT_0_LEN);

        let mut dst = Vec::new();
        assert_eq!(
            sm_stream.read_to_end(&mut dst).await.unwrap(),
            TOTAL_STRUCTURED_LEN
        );
        let mut dst_offset = 0;

        // check stream header
        assert_eq!(
            &dst[..smv1::STREAM_HEADER_LENGTH],
            smv1::StreamHeader {
                message_len: TOTAL_STRUCTURED_LEN as u64,
                flags: smv1::Flags::CRC_64_NVME,
                segment_count: 2,
            }
            .as_bytes()
        );
        dst_offset += smv1::STREAM_HEADER_LENGTH;

        // check segment 0 header
        assert_eq!(
            &dst[dst_offset..dst_offset + smv1::SEGMENT_HEADER_LENGTH],
            smv1::SegmentHeader {
                segment_number: 0,
                content_length: SEGMENT_0_LEN as u64,
            }
            .as_bytes()
        );
        dst_offset += smv1::SEGMENT_HEADER_LENGTH;

        // check segment 0 content
        assert_eq!(
            &dst[dst_offset..dst_offset + SEGMENT_0_LEN],
            &data[..SEGMENT_0_LEN],
        );
        dst_offset += SEGMENT_0_LEN;

        // check segment 0 footer
        assert_eq!(
            &dst[dst_offset..dst_offset + 8],
            &expected_segment_0_crc.to_le_bytes()[..],
        );
        dst_offset += 8;

        // check segment 1 header
        assert_eq!(
            &dst[dst_offset..dst_offset + smv1::SEGMENT_HEADER_LENGTH],
            smv1::SegmentHeader {
                segment_number: 1,
                content_length: (DATA_LEN - SEGMENT_0_LEN) as u64,
            }
            .as_bytes()
        );
        dst_offset += smv1::SEGMENT_HEADER_LENGTH;

        // check segment 1 content
        assert_eq!(
            &dst[dst_offset..dst_offset + DATA_LEN - SEGMENT_0_LEN],
            &data[SEGMENT_0_LEN..],
        );
        dst_offset += DATA_LEN - SEGMENT_0_LEN;

        // check segment 1 footer
        assert_eq!(
            &dst[dst_offset..dst_offset + 8],
            &expected_segment_1_crc.to_le_bytes()[..],
        );
        dst_offset += 8;

        // check stream footer
        assert_eq!(&dst[dst_offset..], &expected_data_crc.to_le_bytes()[..],);
    }

    #[test]
    fn test_encode_bytes_in_structured_message_len() {
        const DATA_LEN: usize = 1024;
        let data = rand::random::<[u8; DATA_LEN]>();

        for segment_len in [usize::MAX, DATA_LEN, DATA_LEN + 1, DATA_LEN - 1, 1] {
            let sm_stream = encode_bytes_in_structured_message(data.to_vec().into(), segment_len);
            assert_eq!(
                sm_stream.len().unwrap(),
                derive_structured_message_length(DATA_LEN as u64, segment_len as u64)
            );
        }
    }

    #[tokio::test]
    async fn test_encode_bytes_in_structured_message_reset() {
        const DATA_LEN: usize = 1024;
        const SEGMENT_LEN: usize = 999;

        let data = rand::random::<[u8; DATA_LEN]>();

        let mut sm_stream = encode_bytes_in_structured_message(data.to_vec().into(), SEGMENT_LEN);

        let mut dst_1 = Vec::new();
        let mut dst_2 = Vec::new();

        sm_stream.read_to_end(&mut dst_1).await.unwrap();
        sm_stream.reset().await.unwrap();
        sm_stream.read_to_end(&mut dst_2).await.unwrap();

        assert_eq!(dst_1, dst_2);
    }
}
