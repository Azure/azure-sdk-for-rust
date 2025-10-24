// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::time::{parse_rfc3339, OffsetDateTime};
use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
};
use time::{
    format_description::{well_known::Rfc3339, FormatItem},
    macros::format_description,
    UtcOffset,
};

// Compile-time format description: RFC3339 with exactly 7 fractional digits and a Z suffix.
static RFC3339_7: &[FormatItem<'_>] =
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:7]Z");

/// Takes in an offset and a length, verifies alignment to a 512-byte boundary, and
///  returns the HTTP range in String format.
///
/// # Arguments
///
/// * `offset` - Start of the byte range to use for writing to a section of the blob.
///   The offset specified must be a modulus of 512.
/// * `length` - Number of bytes to use for writing to a section of the blob.
///   The length specified must be a modulus of 512.
pub fn format_page_range(offset: u64, length: u64) -> Result<String, Error> {
    if offset % 512 != 0 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!(
                "provided offset {} is not aligned to a 512-byte boundary.",
                offset
            ),
        ));
    }
    if length % 512 != 0 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!(
                "provided length {} is not aligned to a 512-byte boundary.",
                offset
            ),
        ));
    }
    let end_range = offset + length - 1;
    let content_range = format!("bytes={}-{}", offset, end_range);
    Ok(content_range)
}

/// Takes in a HashMap of tag key-value pairs and converts them to a filter expression
/// for use with [`BlobServiceClient::find_blobs_by_tags()`](crate::BlobServiceClient::find_blobs_by_tags) or [`BlobContainerClient::find_blobs_by_tags()`](crate::BlobContainerClient::find_blobs_by_tags).
///
/// # Arguments
///
/// * `tags` - A HashMap containing tag key-value pairs representing the
///   expression to find blobs whose tags match the specified condition.
pub fn format_filter_expression(tags: &HashMap<String, String>) -> Result<String, Error> {
    if tags.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Tags HashMap cannot be empty.".to_string(),
        ));
    }

    let format_expression: Vec<String> = tags
        .iter()
        .map(|(key, value)| format!("\"{}\"='{}'", key, value))
        .collect();

    Ok(format_expression.join(" and "))
}

/// Convert an OffsetDateTime to RFC3339 string with exactly 7 decimal precision.
///
/// # Arguments
/// * `datetime` - OffsetDateTime to format.
pub fn format_datetime(datetime: OffsetDateTime) -> Result<String, Error> {
    let utc = datetime.to_offset(UtcOffset::UTC);
    utc.format(&RFC3339_7)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_datetime_with_7_decimals() -> Result<(), Error> {
        // Test with microsecond precision (6 digits) - should pad to 7
        let dt = parse_rfc3339("2025-09-22T19:20:10.622383Z").unwrap();
        let formatted = format_datetime(dt)?;
        assert_eq!(formatted, "2025-09-22T19:20:10.6223830Z");

        // Test with nanosecond precision (9 digits) - should truncate to 7
        let dt = parse_rfc3339("2025-09-22T19:20:00.622429456Z").unwrap();
        let formatted = format_datetime(dt)?;
        assert_eq!(formatted, "2025-09-22T19:20:00.6224294Z");

        // Test with no fractional seconds - should pad with zeros
        let dt = parse_rfc3339("2025-09-22T19:20:00Z").unwrap();
        let formatted = format_datetime(dt)?;
        assert_eq!(formatted, "2025-09-22T19:20:00.0000000Z");

        // Test with millisecond precision (3 digits) - should pad to 7
        let dt = parse_rfc3339("2025-09-22T19:20:00.123Z").unwrap();
        let formatted = format_datetime(dt)?;
        assert_eq!(formatted, "2025-09-22T19:20:00.1230000Z");

        Ok(())
    }

    #[test]
    fn test_format_datetime_no_fractional_seconds() -> Result<(), Error> {
        // Test with no fractional seconds in UTC - exercises the else branch
        let dt = parse_rfc3339("2025-09-22T19:20:00Z").unwrap();
        let formatted = format_datetime(dt)?;
        assert_eq!(formatted, "2025-09-22T19:20:00.0000000Z");

        // Test with no fractional seconds and offset - should convert to UTC
        let dt = parse_rfc3339("2025-09-22T19:20:00-05:00").unwrap();
        let formatted = format_datetime(dt)?;
        assert_eq!(formatted, "2025-09-23T00:20:00.0000000Z");

        // Test with no fractional seconds and positive offset - should convert to UTC
        let dt = parse_rfc3339("2025-09-22T19:20:00+03:30").unwrap();
        let formatted = format_datetime(dt)?;
        assert_eq!(formatted, "2025-09-22T15:50:00.0000000Z");

        Ok(())
    }

    #[test]
    fn test_format_datetime_edge_cases() -> Result<(), Error> {
        // Test with exactly 7 fractional digits - should not truncate or pad
        let dt = parse_rfc3339("2025-09-22T19:20:00.1234567Z").unwrap();
        let formatted = format_datetime(dt)?;
        assert_eq!(formatted, "2025-09-22T19:20:00.1234567Z");

        // Test with single fractional digit - should pad to 7
        let dt = parse_rfc3339("2025-09-22T19:20:00.1Z").unwrap();
        let formatted = format_datetime(dt)?;
        assert_eq!(formatted, "2025-09-22T19:20:00.1000000Z");

        // Test with boundary timezone offset
        let dt = parse_rfc3339("2025-09-22T19:20:00.123+14:00").unwrap();
        let formatted = format_datetime(dt)?;
        assert_eq!(formatted, "2025-09-22T05:20:00.1230000Z");

        Ok(())
    }
}
