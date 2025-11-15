// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use time::{format_description::FormatItem, macros::format_description, OffsetDateTime, UtcOffset};

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

/// Takes a OffsetDateTime and converts to RFC3339-like string with exactly 7 decimal precision and converted to UTC.
///
/// # Arguments
///
/// * `datetime` - OffsetDateTime to format.
pub fn format_storage_datetime(datetime: OffsetDateTime) -> Result<String, Error> {
    let utc = datetime.to_offset(UtcOffset::UTC);
    utc.format(RFC3339_7).map_err(|e| {
        Error::new(
            ErrorKind::InvalidInput,
            format!("Failed to format datetime: {}", e),
        )
    })
}
