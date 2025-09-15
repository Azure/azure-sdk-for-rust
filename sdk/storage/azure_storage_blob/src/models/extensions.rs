// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    http::{RequestContent, XmlFormat},
    xml::to_xml_with_root,
};
use serde::Serialize;

use crate::models::{
    AppendBlobClientCreateOptions, BlobTag, BlobTags, BlockBlobClientUploadBlobFromUrlOptions,
    BlockBlobClientUploadOptions, PageBlobClientCreateOptions, SignedIdentifier,
};
use azure_core::error::ErrorKind;
use std::collections::HashMap;

/// Augments the current options bag to only create if the Page blob does not already exist.
/// # Arguments
///
/// * `self` - The options bag to be modified.
impl<'a> PageBlobClientCreateOptions<'a> {
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
impl<'a> AppendBlobClientCreateOptions<'a> {
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
impl<'a> BlockBlobClientUploadBlobFromUrlOptions<'a> {
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
impl<'a> BlockBlobClientUploadOptions<'a> {
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

// SignedIdentifiers wrapper for correct XML serialization.
#[derive(Serialize)]
struct SignedIdentifiersWrapper {
    #[serde(rename = "SignedIdentifier")]
    items: Vec<SignedIdentifier>,
}

// Converts a `Vec<SignedIdentifier>` into `RequestContent<Vec<SignedIdentifier>, XmlFormat>`.
pub(crate) fn format_signed_identifiers(
    value: Vec<SignedIdentifier>,
) -> Result<RequestContent<Vec<SignedIdentifier>, XmlFormat>, azure_core::Error> {
    let wrapper = SignedIdentifiersWrapper { items: value };
    Ok(to_xml_with_root("SignedIdentifiers", &wrapper)?.into())
}
