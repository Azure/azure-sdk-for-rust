// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::models::{
    AppendBlobClientCreateOptions, BlobTag, BlobTags, PageBlobClientCreateOptions,
};
use azure_core::error::ErrorKind;
use std::collections::{BTreeMap, HashMap};

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
    type Error = azure_core::Error;

    fn try_from(blob_tags: BlobTags) -> Result<Self, azure_core::Error> {
        let mut map = HashMap::new();

        if let Some(tags) = blob_tags.blob_tag_set {
            for tag in tags {
                match (tag.key, tag.value) {
                    (Some(k), Some(v)) => {
                        map.insert(k, v);
                    }
                    _ => {
                        return Err(azure_core::Error::message(
                            azure_core::error::ErrorKind::DataConversion,
                            "BlobTag missing key or value",
                        ));
                    }
                }
            }
        }

        Ok(map)
    }
}

/// Converts a `HashMap<String, String>` into a `BlobTags` struct.
impl From<HashMap<String, String>> for BlobTags {
    fn from(tags: HashMap<String, String>) -> Self {
        let blob_tags = tags
            .into_iter()
            .map(|(k, v)| BlobTag {
                key: Some(k),
                value: Some(v),
            })
            .collect();
        BlobTags {
            blob_tag_set: Some(blob_tags),
        }
    }
}
