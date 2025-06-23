// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::generated::models::{BlobTag, BlobTags};
use azure_core::http::RequestContent;
use std::collections::HashMap;

/// Takes in an offset and a length and returns the HTTP range in string format.
///
/// # Arguments
///
/// * `tags` - A hash map containing the name-value pairs associated with the blob as tags.
pub fn serialize_blob_tags(tags: HashMap<String, String>) -> BlobTags {
    let mut blob_tags: Vec<BlobTag> = vec![];

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
