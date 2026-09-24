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

use crate::models::extensions::VecExt;

use super::{derive_structured_message_length, smv1};

#[derive(Clone, Debug)]
pub struct SeekableStructuredMessageEncodingStream {
    /// Underlying content to be encoded into the structured message.
    content: Box<dyn SeekableStream>,

    /// Cached length of content. Constructor fails if content.len() == None and so we can skip all the Option
    /// checks and just work with a value.
    ///
    /// Structured message encodes the content length into the stream header. Therefore, a change to len() during
    /// streaming is intolerable. This cached value helps enforce that constraint.
    /// On stream reset, a change is technically tolerated by the spec, but is undesirable for a content-validation
    /// feature and should also not be tolerated. Therefore, this value must remain constant over the stream lifetime.
    ///
    /// DO NOT MODIFY.
    content_len: u64,

    /// Number of bytes that have been read so far from content.
    /// This value should be reset on stream reset.
    content_read: u64,

    /// Current segment index. This is tracked separately because the current segment index is often needed exactly on
    /// the border of `content_read / segment_len` on both sides of the border.
    ///
    /// The value is modified when transitioning to the next segment (when `state` is set to
    /// `StructuredMetadata(SegmentHeader, _)`). Therefore, it will always reflect the correct segment while operating
    /// with `SegmentContent`, `StructuredMetadata(SegmentHeader, _)`, and `StructuredMetadata(SegmentFooter, _)`
    current_segment: u16,

    /// Exact number of content bytes to encode per segment, excluding the final segment which may be smaller.
    ///
    /// DO NOT MODIFY.
    segment_len: u64,

    /// Checksums which have already been calculated for the segments, in order. These are held even after use
    /// in case of stream reset. This not only avoids recompute, it also catches any corruption between the
    /// initial streaming and post-reset streaming.
    ///
    /// Once placed, a checksum should never be modified.
    segment_checksums: Vec<Option<ChecksumPair>>,

    /// State of which section of a structured message is currently being read.
    state: StructuredMessageStateMachine,
}

/// What is currently being read from the structured message.
#[derive(Clone, Debug)]
#[allow(clippy::large_enum_variant)] // SegmentContent is the 99% use case
enum StructuredMessageStateMachine {
    /// Contains the number of bytes read from the segment so far and the running digest of the segment content.
    /// Digest is optional to avoid recalculation on stream reset.
    SegmentContent(u64, Option<Digest>),

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
        let segment_count: u16 = content_len.div_ceil(segment_len).try_into().with_context(
            ErrorKind::DataConversion,
            "Unsupported segment count (exceeds u16). Increase segment length to support the content length.",
        )?;
        Ok(Self {
            content,
            content_len,
            content_read: 0,
            current_segment: 0,
            segment_len,
            segment_checksums: vec![None; segment_count as usize],
            state: StructuredMessageStateMachine::StructuredMetadata(
                StructuredMetadata::StreamHeader,
                smv1::StreamHeader {
                    message_len: derive_structured_message_length(content_len, segment_len),
                    flags: smv1::Flags::CRC_64_NVME,
                    segment_count,
                }
                .as_bytes(),
            ),
        })
    }

    /// Total segments in structured message.
    /// # Error
    /// Returns an error if the calculation exceeds the u16 limit.
    /// This should never happen in practice, as this limit is checked at construction and the values involved in the
    /// calculation should never change over the struct lifetime.
    fn segment_count(&self) -> std::io::Result<u16> {
        self.content_len
            .div_ceil(self.segment_len)
            .try_into()
            .map_err(std::io::Error::other)
    }

    /// Composes the overall checksum from individual segment checksums.
    /// # Error
    /// Returns an error if any segment checksums are missing.
    fn compose_checksum_cache(&self) -> std::io::Result<u64> {
        // get all Some values from segment checksum slots
        let checksums = self
            .segment_checksums
            .iter()
            .filter_map(|crc_slot| *crc_slot)
            .collect::<Vec<_>>();

        // if any empty slots, fail fast
        // we should not be calling this method before all checksums are calculated
        if checksums.len() != self.segment_count()? as usize {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Error composing cached segment checksums. Found {}, expected {}",
                    checksums.len(),
                    self.segment_count()?
                ),
            ));
        }

        Ok(checksum_multi_compose(checksums))
    }

    /// Transitions state from segment content to segment footer, handling crc finalization and caching.
    /// # Error
    /// Returns an error if the current state is not segment content.
    /// Returns and error if there is no crc available for the current segment.
    fn transition_to_segment_footer(&mut self) -> std::io::Result<()> {
        let StructuredMessageStateMachine::SegmentContent(segment_cursor, digest) = self.state
        else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Invalid state transition. Attempted to transition to segment footer, but current state was not segment content.",
            ));
        };
        let segment_checksum_cache_slot = self
            .segment_checksums
            // effectively we will never need to actually extend the vector, as it is initialized
            // upfront with the encoded segment count, but the most harm it does is deny a fast
            // fail and it buys us a safe, guaranteed successful get()
            .get_or_extend_mut(self.current_segment as usize, None);
        let segment_crc: u64 =
            // if there's a cached value, always use it
            if let Some(cached_crc) = segment_checksum_cache_slot {
                cached_crc.crc
            } else {
                let crc = digest.map(|d| d.finalize()).ok_or(
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "No crc for the given segment",
                    ),
                )?;
                *segment_checksum_cache_slot = Some(ChecksumPair { crc, length: segment_cursor });
                crc
            };
        self.state = StructuredMessageStateMachine::StructuredMetadata(
            StructuredMetadata::SegmentFooter,
            segment_crc.to_le_bytes().to_vec().into(),
        );
        Ok(())
    }
}

impl AsyncRead for SeekableStructuredMessageEncodingStream {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        mut buf: &mut [u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        let this = self.get_mut();
        let mut total_read = 0;
        loop {
            if buf.is_empty() || total_read > 0 {
                break;
            }
            match &mut this.state {
                StructuredMessageStateMachine::Complete => break,
                StructuredMessageStateMachine::SegmentContent(segment_cursor, segment_digest) => {
                    // Limit read by remaining max segment len, transition if at limit
                    let limit = min(this.segment_len - *segment_cursor, buf.len() as u64) as usize;
                    if limit == 0 {
                        this.transition_to_segment_footer()?;
                        continue;
                    }

                    let inner_read =
                        ready!(pin!(&mut this.content).poll_read(cx, &mut buf[..limit]))?;
                    if inner_read == 0 {
                        // handle premature EOF
                        if this.content_read < this.content_len {
                            return Poll::Ready(Err(std::io::Error::new(
                                std::io::ErrorKind::UnexpectedEof,
                                "Premature EOF reading structured message content",
                            )));
                        }
                        this.transition_to_segment_footer()?;
                        continue;
                    }
                    if let Some(digest) = segment_digest {
                        digest.update(&buf[..inner_read]);
                    }
                    *segment_cursor += inner_read as u64;
                    this.content_read += inner_read as u64;
                    total_read += inner_read;
                    if this.content_read > this.content_len {
                        return Poll::Ready(Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Content stream exceeded reported length producing invalid structured message.",
                        )));
                    }
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
                            // move to next segment header if any
                            StructuredMetadata::StreamHeader
                            | StructuredMetadata::SegmentFooter => {
                                // if underlying content stream finished, move to stream footer, where we must compose checksums
                                if this.content_read >= this.content_len {
                                    StructuredMessageStateMachine::StructuredMetadata(
                                        StructuredMetadata::StreamFooter,
                                        this.compose_checksum_cache()?
                                            .to_le_bytes()
                                            .to_vec()
                                            .into(),
                                    )
                                // otherwise move to the next segment header
                                } else {
                                    this.current_segment = (this.content_read / this.segment_len)
                                        .try_into()
                                        .map_err(std::io::Error::other)?;
                                    StructuredMessageStateMachine::StructuredMetadata(
                                        StructuredMetadata::SegmentHeader,
                                        smv1::SegmentHeader {
                                            segment_number: this.current_segment,
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
                                let segment_number =
                                    (this.content_read / this.segment_len) as usize;
                                StructuredMessageStateMachine::SegmentContent(
                                    0,
                                    // if checksum already calculated, don't recalculate it
                                    if let Some(Some(_crc)) =
                                        this.segment_checksums.get(segment_number)
                                    {
                                        None
                                    } else {
                                        Some(Digest::new(CrcAlgorithm::Crc64Nvme))
                                    },
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

    use crate::structured_message::tests::*;

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
        assert_eq!(&dst[dst_offset..], &expected_data_crc.to_le_bytes()[..],);
    }

    #[tokio::test]
    async fn test_len() {
        const DATA_LEN: usize = 1024;
        const SEGMENT_LEN: u64 = usize::MAX as u64;
        const TOTAL_STRUCTURED_LEN: usize =
            derive_structured_message_length(DATA_LEN as u64, SEGMENT_LEN) as usize;

        let data = rand::random::<[u8; DATA_LEN]>();

        let sm_stream = SeekableStructuredMessageEncodingStream::new(
            Box::new(BytesStream::new(data.to_vec())),
            SEGMENT_LEN,
        )
        .unwrap();

        assert_eq!(sm_stream.len().unwrap(), TOTAL_STRUCTURED_LEN as u64);
    }

    #[tokio::test]
    async fn test_no_len_fail_construct() {
        const DATA_LEN: usize = 1024;
        const SEGMENT_LEN: u64 = usize::MAX as u64;

        let data = rand::random::<[u8; DATA_LEN]>();

        let stream_no_len = SeekableStreamOverrideLen {
            inner: Box::new(BytesStream::new(data.to_vec())),
            len_override: None,
        };

        assert!(
            SeekableStructuredMessageEncodingStream::new(Box::new(stream_no_len), SEGMENT_LEN)
                .is_err()
        );
    }

    #[tokio::test]
    async fn test_reset() {
        const DATA_LEN: usize = 1024;
        const SEGMENT_LEN: u64 = usize::MAX as u64;

        let data = rand::random::<[u8; DATA_LEN]>();

        let mut sm_stream = SeekableStructuredMessageEncodingStream::new(
            Box::new(BytesStream::new(data.to_vec())),
            SEGMENT_LEN,
        )
        .unwrap();

        let mut dst_1 = Vec::new();
        let mut dst_2 = Vec::new();

        sm_stream.read_to_end(&mut dst_1).await.unwrap();
        sm_stream.reset().await.unwrap();
        sm_stream.read_to_end(&mut dst_2).await.unwrap();

        assert_eq!(dst_1, dst_2);
    }

    #[tokio::test]
    async fn test_reset_fail_propagates() {
        const DATA_LEN: usize = 1024;
        const SEGMENT_LEN: u64 = usize::MAX as u64;

        let data = rand::random::<[u8; DATA_LEN]>();

        let stream_no_reset = SeekableStreamFailReset {
            inner: Box::new(BytesStream::new(data.to_vec())),
        };

        let mut sm_stream =
            SeekableStructuredMessageEncodingStream::new(Box::new(stream_no_reset), SEGMENT_LEN)
                .unwrap();

        assert!(sm_stream.reset().await.is_err());
    }

    #[tokio::test]
    async fn test_early_eof() {
        const DATA_LEN: usize = 1024;
        const SEGMENT_LEN: u64 = usize::MAX as u64;

        let data = rand::random::<[u8; DATA_LEN]>();

        let stream_no_len = SeekableStreamOverrideLen {
            inner: Box::new(BytesStream::new(data.to_vec())),
            len_override: Some(data.len() as u64 * 2),
        };
        let mut sm_stream =
            SeekableStructuredMessageEncodingStream::new(Box::new(stream_no_len), SEGMENT_LEN)
                .unwrap();

        assert!(sm_stream.read_to_end(&mut Vec::new()).await.is_err());
    }

    #[tokio::test]
    async fn test_late_eof() {
        const DATA_LEN: usize = 1024;
        const SEGMENT_LEN: u64 = usize::MAX as u64;

        let data = rand::random::<[u8; DATA_LEN]>();

        let stream_no_len = SeekableStreamOverrideLen {
            inner: Box::new(BytesStream::new(data.to_vec())),
            len_override: Some(data.len() as u64 / 2),
        };
        let mut sm_stream =
            SeekableStructuredMessageEncodingStream::new(Box::new(stream_no_len), SEGMENT_LEN)
                .unwrap();

        assert!(sm_stream.read_to_end(&mut Vec::new()).await.is_err());
    }

    fn crc_inline(data: &[u8]) -> u64 {
        let mut digest = Digest::new(CrcAlgorithm::Crc64Nvme);
        digest.update(&data);
        digest.finalize()
    }
}
