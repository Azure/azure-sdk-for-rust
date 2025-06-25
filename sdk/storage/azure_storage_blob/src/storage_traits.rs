// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::models::PageBlobClientCreateOptions;
use crate::PageBlobClient;

pub trait PageBlobClientExt {
    fn format_http_range(offset: u64, length: u64) -> String;
}
/// Takes in an offset and a length and returns the HTTP range in string format.
///
/// # Arguments
///
/// * `offset` - Start of the byte range to use for writing to a section of the blob.
///   The offset specified must be a modulus of 512.
/// * `length` - Number of bytes to use for writing to a section of the blob.
///   The length specified must be a modulus of 512.
impl PageBlobClientExt for PageBlobClient {
    fn format_http_range(offset: u64, length: u64) -> String {
        if offset % 512 != 0 {
            panic!("offset must be aligned to a 512-byte boundary.");
        }
        if length % 512 != 0 {
            panic!("length must be aligned to a 512-byte boundary.");
        }
        let end_range = offset + length - 1;
        let content_range = format!("bytes={}-{}", offset, end_range);
        content_range
    }
}

pub trait PageBlobClientCreateOptionsExt {
    fn with_if_not_exists(self) -> Self;
}

impl PageBlobClientCreateOptionsExt for PageBlobClientCreateOptions<'_> {
    fn with_if_not_exists(self) -> Self {
        Self {
            if_none_match: Some("*".into()),
            ..self
        }
    }
}
