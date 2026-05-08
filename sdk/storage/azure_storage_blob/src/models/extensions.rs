// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::models::{
    method_options::BlockBlobClientUploadOptions, AccessPolicy, AppendBlobClientCreateOptions,
    BlobTag, BlobTags, BlockBlobClientCommitBlockListOptions,
    BlockBlobClientUploadBlobFromUrlOptions, PageBlobClientCreateOptions, SignedIdentifier,
    SignedIdentifiers,
};
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use std::collections::HashMap;

/// Converts a `BlobTags` to the `x-ms-tags` header string format (`key=value&key2=value2`).
///
/// Keys and values are percent-encoded to handle special characters (`&`, `=`, spaces, etc.).
/// Returns `None` if there are no valid tag entries.
pub(crate) fn encode_tags(tags: &BlobTags) -> Option<String> {
    let result = match &tags.blob_tag_set {
        Some(tag_set) => tag_set
            .iter()
            .filter_map(|tag| match (&tag.key, &tag.value) {
                (Some(k), Some(v)) => {
                    let encoded_key = percent_encode(k.as_bytes(), NON_ALPHANUMERIC);
                    let encoded_value = percent_encode(v.as_bytes(), NON_ALPHANUMERIC);
                    Some(format!("{}={}", encoded_key, encoded_value))
                }
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("&"),
        None => String::new(),
    };
    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

/// Augments the current options bag to only create if the Page blob does not already exist.
/// # Arguments
///
/// * `self` - The options bag to be modified.
impl PageBlobClientCreateOptions<'_> {
    pub fn if_not_exists(self) -> Self {
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
    pub fn if_not_exists(self) -> Self {
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
    pub fn if_not_exists(self) -> Self {
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
impl BlockBlobClientUploadOptions<'_> {
    pub fn if_not_exists(self) -> Self {
        Self {
            if_none_match: Some("*".into()),
            ..self
        }
    }
}

/// Sets blob tags on the options bag, encoded as the `x-ms-tags` header value.
///
/// Accepts anything convertible to [`BlobTags`], including `HashMap<String, String>`.
impl PageBlobClientCreateOptions<'_> {
    pub fn with_tags(mut self, tags: impl Into<BlobTags>) -> Self {
        self.blob_tags_string = encode_tags(&tags.into());
        self
    }
}

impl AppendBlobClientCreateOptions<'_> {
    pub fn with_tags(mut self, tags: impl Into<BlobTags>) -> Self {
        self.blob_tags_string = encode_tags(&tags.into());
        self
    }
}

impl BlockBlobClientUploadBlobFromUrlOptions<'_> {
    pub fn with_tags(mut self, tags: impl Into<BlobTags>) -> Self {
        self.blob_tags_string = encode_tags(&tags.into());
        self
    }
}

impl BlockBlobClientUploadOptions<'_> {
    pub fn with_tags(mut self, tags: impl Into<BlobTags>) -> Self {
        self.blob_tags_string = encode_tags(&tags.into());
        self
    }
}

impl BlockBlobClientCommitBlockListOptions<'_> {
    pub fn with_tags(mut self, tags: impl Into<BlobTags>) -> Self {
        self.blob_tags_string = encode_tags(&tags.into());
        self
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
