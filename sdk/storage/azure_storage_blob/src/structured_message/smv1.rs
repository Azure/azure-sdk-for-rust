// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    error::{ErrorKind, ResultExt},
    Error, Result,
};
use bitflags::bitflags;

/// Structured Message version 1.
pub(crate) const MESSAGE_VERSION: u8 = 1;

/// Length of a Structured Message version 1 stream header.
/// 1 byte for version, 8 bytes for message length, 2 bytes for flags, 2 bytes for segment count.
pub(crate) const STREAM_HEADER_LENGTH: usize = 13;

/// Length of a Structured Message version 1 segment header.
/// 2 bytes for segment number, 8 bytes for content length.
pub(crate) const SEGMENT_HEADER_LENGTH: usize = 10;

bitflags! {
    #[derive(Clone, Copy, Default, PartialEq, Eq)]
    pub struct Flags: u16 {
        const NONE = 0x0000;
        const CRC_64_NVME = 0x0001;
    }
}

/// A Structured Message version 1 stream header.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct StreamHeader {
    /// Total message length recorded by the stream header.
    pub(crate) message_len: u64,

    /// Features enabled for the structured message.
    pub(crate) flags: Flags,

    /// Number of segments in the structured message.
    pub(crate) segment_count: u16,
}

impl StreamHeader {
    /// Parses a stream header from raw bytes.
    /// The buffer is expected to be exactly `STREAM_HEADER_LENGTH` bytes long.
    pub(crate) fn parse(buffer: &[u8]) -> Result<Self> {
        if buffer.len() != STREAM_HEADER_LENGTH {
            return Err(Error::with_message(
                ErrorKind::DataConversion,
                format!(
                    "Structured message stream header is exactly {STREAM_HEADER_LENGTH} bytes, buffer to parse was {}.",
                    buffer.len()
                ),
            ));
        }

        let (version_byte, remaining) = buffer.split_at(1);
        if version_byte[0] != MESSAGE_VERSION {
            return Err(Error::with_message(
                ErrorKind::DataConversion,
                "Bad version identifier.",
            ));
        }

        let (len_bytes, remaining) = remaining.split_at(8);
        let message_length = u64::from_le_bytes(
            len_bytes
                .try_into()
                .with_context(ErrorKind::DataConversion, "Incorrect slice length.")?,
        );

        let (flags_bytes, remaining) = remaining.split_at(2);
        let flags = Flags::from_bits(u16::from_le_bytes(
            flags_bytes
                .try_into()
                .with_context(ErrorKind::DataConversion, "Incorrect slice length.")?,
        ))
        .ok_or_else(|| {
            Error::with_message(ErrorKind::DataConversion, "Invalid flags in stream header.")
        })?;

        let segment_count = u16::from_le_bytes(
            remaining
                .try_into()
                .with_context(ErrorKind::DataConversion, "Incorrect slice length.")?,
        );

        Ok(Self {
            message_len: message_length,
            flags,
            segment_count,
        })
    }
}

impl TryFrom<&[u8]> for StreamHeader {
    type Error = Error;

    fn try_from(buffer: &[u8]) -> Result<Self> {
        Self::parse(buffer)
    }
}

/// A Structured Message version 1 segment header.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct SegmentHeader {
    /// Zero-based or service-defined segment number from the wire format.
    pub(crate) segment_number: u16,

    /// Number of content bytes in the segment.
    pub(crate) content_length: u64,
}

impl SegmentHeader {
    /// Parses a segment header from raw bytes.
    /// The buffer is expected to be exactly `SEGMENT_HEADER_LENGTH` bytes long.
    pub(crate) fn parse(buffer: &[u8]) -> Result<Self> {
        if buffer.len() != SEGMENT_HEADER_LENGTH {
            return Err(Error::with_message(
                ErrorKind::DataConversion,
                format!(
                    "Structured message segment header is exactly {SEGMENT_HEADER_LENGTH} bytes, buffer to parse was {}.",
                    buffer.len()
                ),
            ));
        }

        let (segment_num_bytes, content_length_bytes) = buffer.split_at(2);
        let segment_number = u16::from_le_bytes(
            segment_num_bytes
                .try_into()
                .with_context(ErrorKind::DataConversion, "Incorrect slice length.")?,
        );
        let content_length = u64::from_le_bytes(
            content_length_bytes
                .try_into()
                .with_context(ErrorKind::DataConversion, "Incorrect slice length.")?,
        );

        Ok(Self {
            segment_number,
            content_length,
        })
    }
}

impl TryFrom<&[u8]> for SegmentHeader {
    type Error = Error;

    fn try_from(buffer: &[u8]) -> Result<Self> {
        Self::parse(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_stream_header() {
        let buffer = [
            1, // Version
            0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, // Message length
            0x01, 0x00, // Flags
            0x03, 0x00, // Segment count
        ];

        let header = StreamHeader::parse(&buffer).expect("invalid stream header");

        assert_eq!(header.message_len, 0x0102030405060708);
        assert!(header.flags.contains(Flags::CRC_64_NVME));
        assert_eq!(header.segment_count, 3);
    }

    #[test]
    fn rejects_unsupported_stream_version() {
        let buffer = [2; STREAM_HEADER_LENGTH];

        StreamHeader::parse(&buffer)
            .err()
            .expect("unsupported version should fail");
    }

    #[test]
    fn rejects_incorrect_stream_header_length() {
        let buffer_short = [1; STREAM_HEADER_LENGTH - 1];
        let buffer_long = [1; STREAM_HEADER_LENGTH + 1];

        StreamHeader::parse(&buffer_short)
            .err()
            .expect("short header should fail");
        StreamHeader::parse(&buffer_long)
            .err()
            .expect("long header should fail");
    }

    #[test]
    fn parses_segment_header() {
        let buffer = [
            0x34, 0x12, // Segment number
            0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, // Content length
        ];

        let header = SegmentHeader::parse(buffer.as_slice()).expect("invalid segment header");

        assert_eq!(header.segment_number, 0x1234);
        assert_eq!(header.content_length, 0x0102030405060708);
    }

    #[test]
    fn rejects_incorrect_segment_header_length() {
        let buffer_short = [1; SEGMENT_HEADER_LENGTH - 1];
        let buffer_long = [1; SEGMENT_HEADER_LENGTH + 1];

        SegmentHeader::parse(&buffer_short)
            .err()
            .expect("short segment header should fail");
        SegmentHeader::parse(&buffer_long)
            .err()
            .expect("long segment header should fail");
    }
}
