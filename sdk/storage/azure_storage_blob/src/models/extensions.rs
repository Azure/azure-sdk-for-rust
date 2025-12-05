// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::models::{
    AccessPolicy, AppendBlobClientCreateOptions, BlobTag, BlobTags,
    BlockBlobClientUploadBlobFromUrlOptions, BlockBlobClientUploadOptions,
    PageBlobClientCreateOptions, SignedIdentifier, SignedIdentifiers,
};
use std::collections::HashMap;
use time::OffsetDateTime;

/// Access conditions for blob operations.
///
/// Specifies HTTP conditional headers to control when a blob operation should be performed.
#[derive(Clone, Default, Debug)]
pub struct AccessConditions {
    /// A condition that must be met (ETag match) for the request to be processed.
    pub if_match: Option<String>,

    /// A date-time value. Request is made only if the resource has been modified since the specified date-time.
    pub if_modified_since: Option<OffsetDateTime>,

    /// A condition that must be met (ETag does not match) for the request to be processed.
    pub if_none_match: Option<String>,

    /// A date-time value. Request is made only if the resource has not been modified since the specified date-time.
    pub if_unmodified_since: Option<OffsetDateTime>,

    /// A SQL where clause on blob tags to operate only on blobs with matching tag values.
    pub if_tags: Option<String>,
}

impl AccessConditions {
    /// Creates access conditions that allow overwriting an existing blob.
    ///
    /// This is equivalent to having no access conditions - the operation will succeed
    /// whether the blob exists or not. This is the most common case for uploads.
    pub fn allow_overwrite() -> Self {
        Self::default()
    }

    /// Creates access conditions that only succeed if the resource does not exist.
    ///
    /// This sets `if_none_match` to "*" which causes the operation to fail if the resource already exists.
    /// Use this when you want to ensure you're creating a new blob, not overwriting an existing one.
    pub fn if_not_exists() -> Self {
        Self {
            if_none_match: Some("*".to_string()),
            ..Default::default()
        }
    }

    /// Creates access conditions that require the resource to exist with a specific ETag.
    ///
    /// Use this for optimistic concurrency - the operation will only succeed if the blob
    /// hasn't been modified since you last read it.
    pub fn if_match(etag: impl Into<String>) -> Self {
        Self {
            if_match: Some(etag.into()),
            ..Default::default()
        }
    }

    /// Creates empty access conditions (no restrictions).
    ///
    /// Alias for `allow_overwrite()`. The operation will succeed whether the blob exists or not.
    pub fn none() -> Self {
        Self::default()
    }
}

/// Augments the current options bag to only create if the Page blob does not already exist.
/// # Arguments
///
/// * `self` - The options bag to be modified.
impl PageBlobClientCreateOptions<'_> {
    pub fn with_if_not_exists(self) -> Self {
        Self {
            if_none_match: Some("*".into()),
            ..self
        }
    }
}

/// Augments the current options bag to only create if the Append blob does not already exist.
/// # Arguments
///
/// * `self` - The options bag to be modified.
impl AppendBlobClientCreateOptions<'_> {
    pub fn with_if_not_exists(self) -> Self {
        Self {
            if_none_match: Some("*".into()),
            ..self
        }
    }
}

/// Augments the current options bag to only create if the Block blob does not already exist.
/// # Arguments
///
/// * `self` - The options bag to be modified.
impl BlockBlobClientUploadBlobFromUrlOptions<'_> {
    pub fn with_if_not_exists(self) -> Self {
        Self {
            if_none_match: Some("*".into()),
            ..self
        }
    }
}

/// Augments the current options bag to include blob tags.
/// # Arguments
///
/// * `self` - The options bag to be modified.
/// * `tags` - A HashMap of key-value pairs representing the blob tags.
impl BlockBlobClientUploadOptions<'_> {
    pub fn with_tags(self, tags: HashMap<String, String>) -> Self {
        let tags_string = tags
            .iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<_>>()
            .join("&");

        Self {
            blob_tags_string: Some(tags_string),
            ..self
        }
    }
}

/// Converts a `BlobTags` struct into `HashMap<String, String>`.
impl From<BlobTags> for HashMap<String, String> {
    fn from(blob_tags: BlobTags) -> Self {
        let mut map = HashMap::new();

        if let Some(tags) = blob_tags.blob_tag_set {
            for tag in tags {
                if let (Some(key), Some(value)) = (tag.key, tag.value) {
                    map.insert(key, value);
                }
            }
        }
        map
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

/// Converts a `HashMap<String, AccessPolicy>` into a `SignedIdentifiers` struct.
impl From<HashMap<String, AccessPolicy>> for SignedIdentifiers {
    fn from(policies: HashMap<String, AccessPolicy>) -> Self {
        if policies.is_empty() {
            return SignedIdentifiers { items: None };
        }

        let signed_identifiers: Vec<SignedIdentifier> = policies
            .into_iter()
            .map(|(id, access_policy)| SignedIdentifier {
                id: Some(id),
                access_policy: Some(access_policy),
            })
            .collect();

        SignedIdentifiers {
            items: Some(signed_identifiers),
        }
    }
}
