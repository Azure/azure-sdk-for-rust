// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::collections::HashMap;
use std::io::{Error, ErrorKind};

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
    if !offset.is_multiple_of(512) {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!(
                "provided offset {} is not aligned to a 512-byte boundary.",
                offset
            ),
        ));
    }
    if !length.is_multiple_of(512) {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!(
                "provided length {} is not aligned to a 512-byte boundary.",
                length
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

/// Parses `x-ms-meta-*` and `x-ms-or-*` headers from a response into separate maps.
///
/// Returns `(metadata, object_replication_rules)` where:
/// - `metadata` maps unprefixed key names to values from `x-ms-meta-<key>` headers.
/// - `object_replication_rules` maps `<policyId_ruleId>` to status from
///   `x-ms-or-<policyId_ruleId>` headers (excluding `x-ms-or-policy-id`).
pub(crate) fn parse_metadata_and_replication_headers(
    headers: &azure_core::http::headers::Headers,
) -> (HashMap<String, String>, HashMap<String, String>) {
    const META_PREFIX: &str = "x-ms-meta-";
    const OR_PREFIX: &str = "x-ms-or-";
    let mut metadata = HashMap::new();
    let mut object_replication_rules = HashMap::new();
    for (name, value) in headers.iter() {
        let name = name.as_str();
        if name.len() > META_PREFIX.len() && name.starts_with(META_PREFIX) {
            metadata.insert(
                name[META_PREFIX.len()..].to_owned(),
                value.as_str().to_owned(),
            );
        } else if name.len() > OR_PREFIX.len()
            && name.starts_with(OR_PREFIX)
            && name != "x-ms-or-policy-id"
        {
            object_replication_rules.insert(
                name[OR_PREFIX.len()..].to_owned(),
                value.as_str().to_owned(),
            );
        }
    }
    (metadata, object_replication_rules)
}
