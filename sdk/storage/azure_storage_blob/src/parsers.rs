// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::generated::models::{BlobTag, BlobTags};
use azure_core::http::RequestContent;
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

/// Takes in a HashMap of blob tags and serializes them into the `BlobTags` model.
///
/// # Arguments
///
/// * `tags` - A hash map containing the name-value pairs associated with the blob as tags.
pub fn serialize_blob_tags(tags: HashMap<String, String>) -> BlobTags {
    let mut blob_tags = vec![];

    for (k, v) in tags.into_iter() {
        let blob_tag = BlobTag {
            key: Some(k),
            value: Some(v),
        };
        blob_tags.push(blob_tag);
    }
    BlobTags {
        blob_tag_set: Some(blob_tags),
    }
}
