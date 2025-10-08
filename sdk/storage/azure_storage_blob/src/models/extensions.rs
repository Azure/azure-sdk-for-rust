// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::models::{
    AppendBlobClientCreateOptions, BlobTag, BlobTags, BlockBlobClientUploadBlobFromUrlOptions,
    BlockBlobClientUploadOptions, PageBlobClientCreateOptions, StorageError, StorageErrorCode,
};
use azure_core::{error::ErrorKind, http::headers::Headers};
use serde_json::Value;
use std::collections::HashMap;

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
                        return Err(azure_core::Error::with_message(
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

use serde::Deserialize;

/// Internal struct for deserializing Azure Storage XML error responses.
#[derive(Debug, Deserialize)]
#[serde(rename = "Error")]
struct StorageErrorXml {
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "Message")]
    message: String,

    // Dump any unknown fields into a HashMap to avoid deserialization failures.
    // For now I am using "Value" because this lets us capture any type of value.
    // We can additionally get these to all be Strings, but we will need to introduce a lightweight
    // deserializer to go from all possible XML field types to String (e.g. numbers, bools, etc.)
    #[serde(flatten)]
    additional_fields: HashMap<String, Value>,
}

impl TryFrom<azure_core::Error> for StorageError {
    type Error = azure_core::Error;

    fn try_from(error: azure_core::Error) -> Result<Self, Self::Error> {
        match error.kind() {
            ErrorKind::HttpResponse {
                status,
                raw_response,
                ..
            } => {
                // Existence Check for Option<RawResponse>
                let raw_response = raw_response.as_ref().ok_or_else(|| {
                    azure_core::Error::with_message(
                        azure_core::error::ErrorKind::DataConversion,
                        "Cannot convert to StorageError: raw_response is missing.",
                    )
                })?;

                // Extract Headers From Raw Response
                let headers = raw_response.headers().clone();

                // Parse XML Body
                let body = raw_response.body();
                if body.is_empty() {
                    return Err(azure_core::Error::with_message(
                        azure_core::error::ErrorKind::DataConversion,
                        "Cannot convert to StorageError: Response Body is empty.",
                    ));
                }
                let xml_error = azure_core::xml::read_xml::<StorageErrorXml>(body)?;

                // Validate that Error Code and Error Message Are Present
                if xml_error.code.is_empty() {
                    return Err(azure_core::Error::with_message(
                        azure_core::error::ErrorKind::DataConversion,
                        "XML Error Response missing 'Code' field.",
                    ));
                }
                if xml_error.message.is_empty() {
                    return Err(azure_core::Error::with_message(
                        azure_core::error::ErrorKind::DataConversion,
                        "XML Error Response missing 'Message' field.",
                    ));
                }

                // Map Error Code to StorageErrorCode Enum
                let error_code_enum = xml_error
                    .code
                    .parse()
                    .unwrap_or(StorageErrorCode::UnknownValue(xml_error.code));

                Ok(StorageError {
                    status_code: *status,
                    error_code: error_code_enum,
                    message: xml_error.message,
                    headers,
                    additional_error_info: xml_error.additional_fields,
                })
            }
            _ => Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                "ErrorKind was not HttpResponse and could not be parsed.",
            )),
        }
    }
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "HTTP Status Code: {}\n", self.status_code)?;
        writeln!(f, "Error Message: {}\n", self.message)?;
        writeln!(f, "Storage Error Code: {}\n", self.error_code)?;
        writeln!(f, "Response Headers: {:?}\n", self.headers)?;

        if !self.additional_error_info.is_empty() {
            writeln!(f, "\nAdditional Error Info:")?;
            for (key, value) in &self.additional_error_info {
                writeln!(f, "  {}: {}", key, value)?;
            }
        }

        Ok(())
    }
}
