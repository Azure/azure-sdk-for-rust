// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Model types for Azure Blob Storage.

pub mod error;
mod extensions;
pub(crate) mod http_ranges;
pub mod method_options;

pub use crate::generated::models::*;

/// Serde deserialization helpers for [`BlobName`] XML elements.
///
/// Deserializes a [`BlobName`] XML element directly into a `String`.
/// If the `Encoded` attribute is `true`, the content will be percent-decoded.
/// Otherwise, the content is returned as-is.
///
/// # Example
///
/// ```ignore
/// #[derive(Deserialize)]
/// struct MyStruct {
///     #[serde(deserialize_with = "blob_name::deserialize")]
///     name: String,
/// }
/// ```
///
/// For optional fields, use the [`blob_name::option`] module:
///
/// ```ignore
/// #[derive(Deserialize)]
/// struct MyStruct {
///     #[serde(deserialize_with = "blob_name::option::deserialize")]
///     name: Option<String>,
/// }
/// ```
pub mod blob_name {
    use super::BlobName;
    use percent_encoding::percent_decode_str;
    use serde::{de::Error, Deserialize, Deserializer};

    /// Deserializes a [`BlobName`] XML element into a `String`.
    ///
    /// If the `Encoded` attribute is `true`, the content will be percent-decoded.
    /// Otherwise, the content is returned as-is.
    ///
    /// # Errors
    ///
    /// Returns a deserialization error if:
    /// - The `BlobName` element or its content is missing.
    /// - The content is percent-encoded but contains invalid UTF-8 sequences after decoding.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let blob_name = BlobName::deserialize(deserializer)?;

        let content = blob_name
            .content
            .ok_or_else(|| D::Error::custom("missing BlobName content"))?;

        if blob_name.encoded.unwrap_or_default() {
            let decoded = percent_decode_str(&content)
                .decode_utf8()
                .map_err(D::Error::custom)?;
            Ok(decoded.into_owned())
        } else {
            Ok(content)
        }
    }

    /// Serde deserialization helpers for optional [`BlobName`] XML elements.
    pub mod option {
        use super::BlobName;
        use percent_encoding::percent_decode_str;
        use serde::{de::Error, Deserialize, Deserializer};

        /// Deserializes a [`BlobName`] XML element into an `Option<String>`.
        ///
        /// If the `Encoded` attribute is `true`, the content will be percent-decoded.
        /// Otherwise, the content is returned as-is.
        ///
        /// # Errors
        ///
        /// Returns a deserialization error if the content is percent-encoded but contains
        /// invalid UTF-8 sequences after decoding.
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let blob_name = Option::<BlobName>::deserialize(deserializer)?;

            let Some(blob_name) = blob_name else {
                return Ok(None);
            };

            let Some(content) = blob_name.content else {
                return Ok(None);
            };

            if blob_name.encoded.unwrap_or_default() {
                let decoded = percent_decode_str(&content)
                    .decode_utf8()
                    .map_err(D::Error::custom)?;
                Ok(Some(decoded.into_owned()))
            } else {
                Ok(Some(content))
            }
        }
    }
}

// TODO: Need mechanism to have these models not have "Internal" suffix
pub use crate::generated::models::{
    BlobClientDownloadInternalOptions as BlobClientDownloadOptions,
    BlobClientDownloadInternalResult as BlobClientDownloadResult,
    BlobClientDownloadInternalResultHeaders as BlobClientDownloadResultHeaders,
    BlockBlobClientUploadInternalOptions as BlockBlobClientUploadOptions,
    BlockBlobClientUploadInternalResult as BlockBlobClientUploadResult,
    BlockBlobClientUploadInternalResultHeaders as BlockBlobClientUploadResultHeaders,
};

#[cfg(test)]
mod tests {
    use azure_core::xml;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct RequiredName {
        #[serde(
            deserialize_with = "crate::models::blob_name::deserialize",
            rename = "Name"
        )]
        name: String,
    }

    #[derive(Deserialize)]
    struct OptionalName {
        #[serde(
            deserialize_with = "crate::models::blob_name::option::deserialize",
            rename = "Name",
            default
        )]
        name: Option<String>,
    }

    #[test]
    fn deserialize_plain_name() {
        let input = b"<Root><Name>hello</Name></Root>";
        let result: RequiredName = xml::from_xml(input).unwrap();
        assert_eq!(result.name, "hello");
    }

    #[test]
    fn deserialize_encoded_name() {
        let input = b"<Root><Name Encoded=\"true\">hello%20world</Name></Root>";
        let result: RequiredName = xml::from_xml(input).unwrap();
        assert_eq!(result.name, "hello world");
    }

    #[test]
    fn deserialize_not_encoded_name() {
        let input = b"<Root><Name Encoded=\"false\">hello%20world</Name></Root>";
        let result: RequiredName = xml::from_xml(input).unwrap();
        assert_eq!(result.name, "hello%20world");
    }

    #[test]
    fn deserialize_option_some() {
        let input = b"<Root><Name>hello</Name></Root>";
        let result: OptionalName = xml::from_xml(input).unwrap();
        assert_eq!(result.name.as_deref(), Some("hello"));
    }

    #[test]
    fn deserialize_option_some_encoded() {
        let input = b"<Root><Name Encoded=\"true\">hello%20world</Name></Root>";
        let result: OptionalName = xml::from_xml(input).unwrap();
        assert_eq!(result.name.as_deref(), Some("hello world"));
    }

    #[test]
    fn deserialize_option_none() {
        let input = b"<Root></Root>";
        let result: OptionalName = xml::from_xml(input).unwrap();
        assert_eq!(result.name, None);
    }

    #[test]
    fn deserialize_encoded_xml_invalid_fffe() {
        // U+FFFE → UTF-8: EF BF BE → percent-encoded: %EF%BF%BE
        let input = b"<Root><Name Encoded=\"true\">blob%EF%BF%BEname</Name></Root>";
        let result: RequiredName = xml::from_xml(input).unwrap();
        assert_eq!(result.name, "blob\u{FFFE}name");
    }

    #[test]
    fn deserialize_encoded_xml_invalid_ffff() {
        // U+FFFF → UTF-8: EF BF BF → percent-encoded: %EF%BF%BF
        let input = b"<Root><Name Encoded=\"true\">blob%EF%BF%BFname</Name></Root>";
        let result: RequiredName = xml::from_xml(input).unwrap();
        assert_eq!(result.name, "blob\u{FFFF}name");
    }

    #[test]
    fn deserialize_option_encoded_xml_invalid_fffe() {
        let input = b"<Root><Name Encoded=\"true\">blob%EF%BF%BEname</Name></Root>";
        let result: OptionalName = xml::from_xml(input).unwrap();
        assert_eq!(result.name.as_deref(), Some("blob\u{FFFE}name"));
    }

    #[test]
    fn deserialize_option_encoded_xml_invalid_ffff() {
        let input = b"<Root><Name Encoded=\"true\">blob%EF%BF%BFname</Name></Root>";
        let result: OptionalName = xml::from_xml(input).unwrap();
        assert_eq!(result.name.as_deref(), Some("blob\u{FFFF}name"));
    }

    #[test]
    fn deserialize_encoded_mixed_invalid_and_normal() {
        // Name with both XML-invalid chars and normal path separators
        let input = b"<Root><Name Encoded=\"true\">dir%2Fblob%EF%BF%BEname%EF%BF%BF</Name></Root>";
        let result: RequiredName = xml::from_xml(input).unwrap();
        assert_eq!(result.name, "dir/blob\u{FFFE}name\u{FFFF}");
    }
}
