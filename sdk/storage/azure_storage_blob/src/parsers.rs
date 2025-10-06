// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::models::{BlobTag, BlobTags};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use url::Url;

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

/// Parses the container name and blob name from a full blob URL.
///
/// # Arguments
///
/// * `url` - The URL to parse.
///
/// # Returns
///
/// A tuple containing (container_name, blob_name). Both will be URL-decoded.
pub(crate) fn parse_url_name_components(url: &Url) -> azure_core::Result<(String, String)> {
    let path_segments: Vec<&str> = url
        .path_segments()
        .ok_or_else(|| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "URL cannot be a base URL",
            )
        })?
        .filter(|s| !s.is_empty()) // Filter out empty segments
        .collect();

    if path_segments.is_empty() {
        return Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            "URL path must contain at least a container name",
        ));
    }

    // First segment is the container name (automatically URL-decoded by url crate)
    let container_name = path_segments[0].to_string();

    // Remaining segments form the blob name
    let blob_name = if path_segments.len() > 1 {
        path_segments[1..].join("/")
    } else {
        return Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            "URL path must contain both container name and blob name",
        ));
    };

    Ok((container_name, blob_name))
}
