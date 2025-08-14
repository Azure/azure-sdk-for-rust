// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::models::{
    AppendBlobClientCreateOptions, BlobTag, BlobTags, PageBlobClientCreateOptions,
};
use std::collections::HashMap;

/// Provides usage helpers for setting the `PageBlobClientCreateOptions` optional configurations.
pub trait PageBlobClientCreateOptionsExt {
    /// Augments the current options bag to only create if the Page blob does not already exist.
    /// # Arguments
    ///
    /// * `self` - The options bag to be modified.
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

/// Provides usage helpers for setting the `AppendBlobClientCreateOptions` optional configurations.
pub trait AppendBlobClientCreateOptionsExt {
    /// Augments the current options bag to only create if the Append blob does not already exist.
    /// # Arguments
    ///
    /// * `self` - The options bag to be modified.
    fn with_if_not_exists(self) -> Self;
}

impl AppendBlobClientCreateOptionsExt for AppendBlobClientCreateOptions<'_> {
    fn with_if_not_exists(self) -> Self {
        Self {
            if_none_match: Some("*".into()),
            ..self
        }
    }
}

/// Converts a `BlobTags` struct into `HashMap<String, String>`.
impl TryFrom<BlobTags> for HashMap<String, String> {
    type Error = &'static str;

    fn try_from(blob_tags: BlobTags) -> Result<Self, Self::Error> {
        let mut map = HashMap::new();

        if let Some(tags) = blob_tags.blob_tag_set {
            for tag in tags {
                match (tag.key, tag.value) {
                    (Some(k), Some(v)) => {
                        map.insert(k, v);
                    }
                    _ => return Err("BlobTag missing key or value"),
                }
            }
        }

        Ok(map)
    }
}
