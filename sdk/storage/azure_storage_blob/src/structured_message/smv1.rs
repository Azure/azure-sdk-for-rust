// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::borrow::Cow;

use azure_core::{
    error::{ErrorKind, ResultExt},
    Error, Result,
};
use bitflags::bitflags;
use bytes::Bytes;

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
#[derive(Clone, Copy, Default, PartialEq, Eq)]
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
        validate_buffer_length(buffer, STREAM_HEADER_LENGTH, format!("Structured message stream header is exactly {STREAM_HEADER_LENGTH} bytes, buffer to parse was {}.", buffer.len()))?;

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

    pub(crate) fn as_bytes(&self) -> Bytes {
        let mut buffer = vec![0u8; STREAM_HEADER_LENGTH];
        // SAFETY: The buffer has been created above with the required length.
        unsafe { self.write_unchecked(&mut buffer) };
        buffer.into()
    }

    pub(crate) fn write(&self, buffer: &mut [u8]) -> Result<()> {
        validate_buffer_length(buffer, STREAM_HEADER_LENGTH, format!("Structured message stream header is exactly {STREAM_HEADER_LENGTH} bytes, buffer to write to was {}.", buffer.len()))?;
        // SAFETY: The buffer length has been validated above.
        unsafe { self.write_unchecked(buffer) };
        Ok(())
    }

    /// Writes the stream header to the provided buffer without validating its length.
    /// # Safety
    /// The caller must ensure that the buffer is at least `STREAM_HEADER_LENGTH` bytes long.
    unsafe fn write_unchecked(&self, buffer: &mut [u8]) {
        let mut remaining = buffer;

        remaining[0] = MESSAGE_VERSION;
        remaining = &mut remaining[1..];

        remaining[..8].copy_from_slice(&self.message_len.to_le_bytes());
        remaining = &mut remaining[8..];

        remaining[..2].copy_from_slice(&self.flags.bits().to_le_bytes());
        remaining = &mut remaining[2..];

        remaining[..2].copy_from_slice(&self.segment_count.to_le_bytes());
    }
}

impl TryFrom<&[u8]> for StreamHeader {
    type Error = Error;

    fn try_from(buffer: &[u8]) -> Result<Self> {
        Self::parse(buffer)
    }
}

/// A Structured Message version 1 segment header.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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

    pub(crate) fn as_bytes(&self) -> Bytes {
        let mut buffer = vec![0u8; SEGMENT_HEADER_LENGTH];
        // SAFETY: The buffer has been created above with the required length.
        unsafe { self.write_unchecked(&mut buffer) };
        buffer.into()
    }

    pub(crate) fn write(&self, buffer: &mut [u8]) -> Result<()> {
        validate_buffer_length(buffer, SEGMENT_HEADER_LENGTH, format!("Structured message segment header is exactly {SEGMENT_HEADER_LENGTH} bytes, buffer to write to was {}.", buffer.len()))?;
        // SAFETY: The buffer length has been validated above.
        unsafe { self.write_unchecked(buffer) };
        Ok(())
    }

    /// Writes the segment header to the provided buffer without validating its length.
    /// # Safety
    /// The caller must ensure that the buffer is at least `SEGMENT_HEADER_LENGTH` bytes long.
    unsafe fn write_unchecked(&self, buffer: &mut [u8]) {
        let mut remaining = buffer;

        remaining[..2].copy_from_slice(&self.segment_number.to_le_bytes());
        remaining = &mut remaining[2..];

        remaining[..8].copy_from_slice(&self.content_length.to_le_bytes());
    }
}

impl TryFrom<&[u8]> for SegmentHeader {
    type Error = Error;

    fn try_from(buffer: &[u8]) -> Result<Self> {
        Self::parse(buffer)
    }
}

fn validate_buffer_length<C: Into<Cow<'static, str>>>(
    buffer: &[u8],
    expected_length: usize,
    message: C,
) -> Result<()> {
    if buffer.len() != expected_length {
        return Err(Error::with_message(ErrorKind::DataConversion, message));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_stream_header() {
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
    fn parse_stream_header_rejects_unsupported_stream_version() {
        let buffer = [2; STREAM_HEADER_LENGTH];

        StreamHeader::parse(&buffer)
            .err()
            .expect("unsupported version should fail");
    }

    #[test]
    fn parse_stream_header_rejects_unsupported_flags() {
        let buffer = [
            1, // Version
            0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, // Message length
            0x99, 0x99, // Flags
            0x03, 0x00, // Segment count
        ];

        StreamHeader::parse(&buffer)
            .err()
            .expect("unsupported flags should fail");
    }

    #[test]
    fn parse_stream_header_rejects_incorrect_buffer_len() {
        let buffer_short = [1; STREAM_HEADER_LENGTH - 1];
        let buffer_long = [1; STREAM_HEADER_LENGTH + 1];

        StreamHeader::parse(&buffer_short)
            .err()
            .expect("short buffer should fail");
        StreamHeader::parse(&buffer_long)
            .err()
            .expect("long buffer should fail");
    }

    #[test]
    fn write_stream_header() {
        let header = StreamHeader {
            message_len: 0x0102030405060708,
            flags: Flags::CRC_64_NVME,
            segment_count: 3,
        };

        let mut buffer = [0u8; STREAM_HEADER_LENGTH];
        header
            .write(&mut buffer)
            .expect("failed to write stream header");

        assert_eq!(buffer[0], MESSAGE_VERSION);
        assert_eq!(&buffer[1..9], &0x0102030405060708u64.to_le_bytes());
        assert_eq!(&buffer[9..11], &Flags::CRC_64_NVME.bits().to_le_bytes());
        assert_eq!(&buffer[11..STREAM_HEADER_LENGTH], &3u16.to_le_bytes());
    }

    #[test]
    fn write_stream_header_rejects_incorrect_buffer_len() {
        let header = StreamHeader {
            message_len: 0x0102030405060708,
            flags: Flags::CRC_64_NVME,
            segment_count: 3,
        };

        let mut buffer_short = [0u8; STREAM_HEADER_LENGTH - 1];
        let mut buffer_long = [0u8; STREAM_HEADER_LENGTH + 1];

        header
            .write(&mut buffer_short)
            .expect_err("short buffer should fail");
        header
            .write(&mut buffer_long)
            .expect_err("long buffer should fail");
    }

    #[test]
    fn parse_segment_header() {
        let buffer = [
            0x34, 0x12, // Segment number
            0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, // Content length
        ];

        let header = SegmentHeader::parse(buffer.as_slice()).expect("invalid segment header");

        assert_eq!(header.segment_number, 0x1234);
        assert_eq!(header.content_length, 0x0102030405060708);
    }

    #[test]
    fn parse_segment_header_rejects_incorrect_buffer_len() {
        let buffer_short = [1; SEGMENT_HEADER_LENGTH - 1];
        let buffer_long = [1; SEGMENT_HEADER_LENGTH + 1];

        SegmentHeader::parse(&buffer_short).expect_err("short segment header should fail");
        SegmentHeader::parse(&buffer_long).expect_err("long segment header should fail");
    }

    #[test]
    fn write_segment_header() {
        let header = SegmentHeader {
            segment_number: 0x1234,
            content_length: 0x0102030405060708,
        };

        let mut buffer = [0u8; SEGMENT_HEADER_LENGTH];
        header
            .write(&mut buffer)
            .expect("failed to write segment header");

        assert_eq!(&buffer[0..2], &0x1234u16.to_le_bytes());
        assert_eq!(&buffer[2..10], &0x0102030405060708u64.to_le_bytes());
    }

    #[test]
    fn write_segment_header_rejects_incorrect_buffer_len() {
        let header = SegmentHeader {
            segment_number: 0x1234,
            content_length: 0x0102030405060708,
        };

        let mut buffer_short = [0u8; SEGMENT_HEADER_LENGTH - 1];
        let mut buffer_long = [0u8; SEGMENT_HEADER_LENGTH + 1];

        header
            .write(&mut buffer_short)
            .expect_err("short buffer should fail");
        header
            .write(&mut buffer_long)
            .expect_err("long buffer should fail");
    }
}
