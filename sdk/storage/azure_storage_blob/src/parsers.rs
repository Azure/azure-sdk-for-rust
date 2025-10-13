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

// /// Parses the container name and blob name from a full blob URL.
// ///
// /// # Arguments
// ///
// /// * `url` - The URL to parse.
// ///
// /// # Returns
// ///
// /// A tuple containing (container_name, blob_name).
// pub(crate) fn parse_url_name_components(url: &Url) -> azure_core::Result<(String, String)> {
//     let path_segments: Vec<&str> = url
//         .path_segments()
//         .ok_or_else(|| {
//             azure_core::Error::with_message(
//                 azure_core::error::ErrorKind::Other,
//                 format!("Invalid blob URL '{}': URL must have a path component with container name and blob name.", url),
//             )
//         })?
//         .collect();

//     if path_segments.len() < 2 {
//         return Err(azure_core::Error::with_message(
//             azure_core::error::ErrorKind::Other,
//             format!(
//                 "Invalid blob URL '{}': URL path must contain both container name and blob name",
//                 url
//             ),
//         ));
//     }

//     // First segment is the container name
//     let container_name = path_segments[0].to_string();

//     // Remaining segments form the blob name
//     let blob_name = path_segments[1..].join("/");

//     Ok((container_name, blob_name))
// }

/// Parses the container name and blob name from a full blob URL and applies first-order decoding.
///
/// # Arguments
///
/// * `url` - The URL to parse.
///
/// # Returns
///
/// A tuple containing (container_name, blob_name). Both will be percent-decoded.
/// Parses the container name and blob name from a full blob URL and applies percent-decoding.
///
/// # Arguments
///
/// * `url` - The URL to parse.
///
/// # Returns
///
/// A tuple containing (container_name, blob_name). Both will be percent-decoded.
pub(crate) fn parse_url_name_components_decoded(url: &Url) -> azure_core::Result<(String, String)> {
    let path_segments: Vec<&str> = url
        .path_segments()
        .ok_or_else(|| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("Invalid blob URL '{}': URL must have a path component with container name and blob name.", url),
            )
        })?
        .collect();

    if path_segments.len() < 2 {
        return Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            format!(
                "Invalid blob URL '{}': URL path must contain both container name and blob name",
                url
            ),
        ));
    }

    // First segment is the container name - percent-decode it
    let container_name = percent_encoding::percent_decode_str(path_segments[0])
        .decode_utf8()
        .map_err(|e| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                format!("Failed to decode container name: {}", e),
            )
        })?
        .to_string();

    // Remaining segments form the blob name - join then percent-decode
    let blob_name_encoded = path_segments[1..].join("/");
    let blob_name = percent_encoding::percent_decode_str(&blob_name_encoded)
        .decode_utf8()
        .map_err(|e| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                format!("Failed to decode blob name: {}", e),
            )
        })?
        .to_string();

    Ok((container_name, blob_name))
}
