// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    cmp::min,
    pin::pin,
    task::{ready, Poll},
};

use async_trait::async_trait;
use azure_core::{
    error::{ErrorKind, ResultExt},
    stream::SeekableStream,
    Error, Result,
};
use bytes::Bytes;
use crc_fast::{checksum_combine, CrcAlgorithm, Digest};
use futures::AsyncRead;

use super::{derive_structured_message_length, smv1};

#[derive(Clone, Debug)]
pub struct SeekableStructuredMessageEncodingStream {
    /// Underlying content to be encoded into the structured message.
    content: Box<dyn SeekableStream>,

    /// Cached length of content. Constructor fails if content.len() == None and so we can skip all the Option
    /// checks and just work with a value. Additionally, structured message cannot tolerate a change to this value,
    /// so we do not need to accommodate such a check.
    content_len: u64,

    /// Number of bytes that have been read so far from content.
    content_read: u64,

    /// Exact number of content bytes to encode per segment, excluding the final segment which may be smaller.
    segment_len: u64,

    /// Checksums which have already been calculated for the segments, in order. These are held even after use
    /// in case of stream reset. This not only avoids recompute, it also catches any corruption between the
    /// initial streaming and post-reset streaming.
    segment_checksums: Vec<u64>,

    /// State of which section of a structured message is currently being read.
    state: StructuredMessageStateMachine,
}

/// What is currently being read from the structured message.
#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)] // SegmentContent is the 99% use case
enum StructuredMessageStateMachine {
    /// Contains the number of bytes read from the segment so far and the running digest of the segment content.
    SegmentContent(u64, Digest),

    /// Currently reading structured metadata: bytes that are not part of the underlying content.
    /// Contains the type of structured metadata being read and the remaining bytes of that metadata.
    StructuredMetadata(StructuredMetadata, Bytes),

    /// End of structured message.
    Complete,
}

#[derive(Clone, Debug)]
enum StructuredMetadata {
    StreamHeader,
    StreamFooter,
    SegmentHeader,
    SegmentFooter,
}

impl SeekableStructuredMessageEncodingStream {
    pub fn new(content: Box<dyn SeekableStream>, segment_len: u64) -> Result<Self> {
        let Some(content_len) = content.len() else {
            return Err(Error::with_message(
                ErrorKind::Io,
                "Structured message requires content of a known length.",
            ));
        };
        Ok(Self {
            content,
            content_len,
            content_read: 0,
            segment_len,
            segment_checksums: Vec::with_capacity(
                (content_len.div_ceil(segment_len))
                    .try_into()
                    .with_context(
                        ErrorKind::DataConversion,
                        "usize overflow constructing a SeekableStructuredMessageEncodingStream",
                    )?,
            ),
            state: StructuredMessageStateMachine::StructuredMetadata(
                StructuredMetadata::StreamHeader,
                smv1::StreamHeader {
                    message_len: derive_structured_message_length(content_len, segment_len),
                    flags: smv1::Flags::CRC_64_NVME,
                    segment_count: content_len.div_ceil(segment_len) as u16,
                }
                .as_bytes(),
            ),
        })
    }
}

impl AsyncRead for SeekableStructuredMessageEncodingStream {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        mut buf: &mut [u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        if buf.is_empty() {
            return Poll::Ready(Ok(0));
        }
        let this = self.get_mut();
        let mut total_read = 0;
        loop {
            match &mut this.state {
                StructuredMessageStateMachine::SegmentContent(segment_cursor, digest) => {
                    // If we've already read some bytes into the buffer, they should be returned
                    // before attempting to perform a read that could result in an error.
                    if total_read > 0 {
                        break;
                    }

                    // we've already determined buf is not empty at start of this method.
                    // if limit == 0, it means we've reached the end of the current segment.
                    let limit = min(this.segment_len - *segment_cursor, buf.len() as u64) as usize;
                    if limit == 0 {
                        todo!("transition to segment footer")
                    }

                    let inner_read =
                        ready!(pin!(&mut this.content).poll_read(cx, &mut buf[..limit]))?;
                    if inner_read == 0 {
                        todo!("handle end of content stream. get checksum. finish segment and finish stream.")
                    }
                    digest.update(&buf[..inner_read]);
                    this.content_read += inner_read as u64;
                    total_read += inner_read;
                    buf = &mut buf[inner_read..];
                }
                StructuredMessageStateMachine::StructuredMetadata(structured_metadata, bytes) => {
                    let read = min(bytes.len(), buf.len());
                    let remaining = bytes.split_off(read);
                    buf[..read].copy_from_slice(&bytes);

                    total_read += read;
                    buf = &mut buf[read..];

                    if remaining.is_empty() {
                        this.state = match structured_metadata {
                            // move to next segment if any
                            StructuredMetadata::StreamHeader
                            | StructuredMetadata::SegmentFooter => {
                                // if underlying content stream finished, move to stream footer, where we must compose checksums
                                if this.content_read >= this.content_len {
                                    // pair each segment checksum with the segment length
                                    let mut checksums = this
                                        .segment_checksums
                                        .iter()
                                        .map(|crc| ChecksumPair {
                                            crc: *crc,
                                            length: this.segment_len,
                                        })
                                        .collect::<Vec<_>>();
                                    // special-case last segment, which may be shorter than other segments
                                    if let Some(last) = checksums.last_mut() {
                                        last.length = this.content_len % this.segment_len;
                                        // special-case modulo resulting in 0. the segment was full-length, not empty
                                        if last.length == 0 {
                                            last.length = this.segment_len;
                                        }
                                    }
                                    // change state to stream footer as the bytes of the composed checksums
                                    StructuredMessageStateMachine::StructuredMetadata(
                                        StructuredMetadata::StreamFooter,
                                        checksum_multi_compose(checksums)
                                            .to_le_bytes()
                                            .to_vec()
                                            .into(),
                                    )
                                } else {
                                    StructuredMessageStateMachine::StructuredMetadata(
                                        StructuredMetadata::SegmentHeader,
                                        smv1::SegmentHeader {
                                            segment_number: (this.content_read / this.segment_len)
                                                .try_into()
                                                .map_err(std::io::Error::other)?,
                                            content_length: min(
                                                this.segment_len,
                                                this.content_len - this.content_read,
                                            ),
                                        }
                                        .as_bytes(),
                                    )
                                }
                            }
                            StructuredMetadata::SegmentHeader => {
                                StructuredMessageStateMachine::SegmentContent(
                                    0,
                                    Digest::new(CrcAlgorithm::Crc64Nvme),
                                )
                            }
                            StructuredMetadata::StreamFooter => {
                                StructuredMessageStateMachine::Complete
                            }
                        }
                    } else {
                        *bytes = remaining
                    };
                }
                StructuredMessageStateMachine::Complete => break,
            }
        }
        Poll::Ready(Ok(total_read))
    }
}

#[async_trait]
impl SeekableStream for SeekableStructuredMessageEncodingStream {
    async fn reset(&mut self) -> azure_core::Result<()> {
        self.content.reset().await?;
        self.content_read = 0;
        self.state = StructuredMessageStateMachine::StructuredMetadata(
            StructuredMetadata::StreamHeader,
            smv1::StreamHeader {
                message_len: derive_structured_message_length(self.content_len, self.segment_len),
                flags: smv1::Flags::CRC_64_NVME,
                segment_count: self.content_len.div_ceil(self.segment_len) as u16,
            }
            .as_bytes(),
        );
        Ok(())
    }

    fn len(&self) -> Option<u64> {
        let num_segments = self.content_len.div_ceil(self.segment_len);

        const CRC_LEN: u64 = 8;
        return Some(
            self.content_len
                + smv1::STREAM_HEADER_LENGTH as u64 // header
                + num_segments * (smv1::SEGMENT_HEADER_LENGTH as u64 + CRC_LEN) // segment headers and footers
                + CRC_LEN, // footer
        );
    }
}

#[derive(Clone, Copy, Debug)]
struct ChecksumPair {
    crc: u64,
    length: u64,
}
fn checksum_multi_compose<I>(into_iter: I) -> u64
where
    I: IntoIterator<Item = ChecksumPair>,
{
    let mut iter = into_iter.into_iter();
    let mut composed_crc = iter.next().map(|pair| pair.crc).unwrap_or(0);
    for ChecksumPair { crc, length } in iter {
        composed_crc = checksum_combine(CrcAlgorithm::Crc64Nvme, composed_crc, crc, length);
    }
    composed_crc
}

#[cfg(test)]
mod tests {
    use azure_core::stream::BytesStream;
    use futures::AsyncReadExt;

    use super::*;

    #[tokio::test]
    async fn test_encode_single_segment() {
        const DATA_LEN: usize = 1024;
        const SEGMENT_LEN: u64 = usize::MAX as u64;
        const TOTAL_STRUCTURED_LEN: usize =
            derive_structured_message_length(DATA_LEN as u64, SEGMENT_LEN) as usize;

        let data = rand::random::<[u8; DATA_LEN]>();
        let expected_data_crc = crc_inline(&data);

        let mut sm_stream = SeekableStructuredMessageEncodingStream::new(
            Box::new(BytesStream::new(data.to_vec())),
            SEGMENT_LEN,
        )
        .unwrap();

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
    async fn test_encode_multi_segment() {
        const DATA_LEN: usize = 1024;
        const SEGMENT_0_LEN: usize = 999; // results in 2 segments of uneven length
        const TOTAL_STRUCTURED_LEN: usize =
            derive_structured_message_length(DATA_LEN as u64, SEGMENT_0_LEN as u64) as usize;

        let data = rand::random::<[u8; DATA_LEN]>();
        let expected_segment_0_crc = crc_inline(&data[..SEGMENT_0_LEN]);
        let expected_segment_1_crc = crc_inline(&data[SEGMENT_0_LEN..]);
        let expected_data_crc = crc_inline(&data);

        let mut sm_stream = SeekableStructuredMessageEncodingStream::new(
            Box::new(BytesStream::new(data.to_vec())),
            SEGMENT_0_LEN as u64,
        )
        .unwrap();

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
        assert_eq!(
            &dst[dst_offset + SEGMENT_0_LEN..],
            &expected_data_crc.to_le_bytes()[..],
        );
    }

    fn crc_inline(data: &[u8]) -> u64 {
        let mut digest = Digest::new(CrcAlgorithm::Crc64Nvme);
        digest.update(&data);
        digest.finalize()
    }
}
