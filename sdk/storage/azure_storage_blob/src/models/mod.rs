// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Model types for Azure Blob Storage.

pub mod error;
mod extensions;
pub(crate) mod http_ranges;
pub mod method_options;

pub use crate::generated::models::*;

use serde::{de::Error, Deserialize, Deserializer};

/// Deserializes a [`BlobName`] XML element directly into an `Option<String>`.
///
/// Use this with `#[serde(deserialize_with = "deserialize_blob_name")]` on fields
/// that should be decoded from the `BlobName` XML structure into a plain `Option<String>`.
///
/// If the `Encoded` attribute is `true`, the content will be percent-decoded.
/// Otherwise, the content is returned as-is.
///
/// # Errors
///
/// Returns a deserialization error if the content is percent-encoded but contains
/// invalid UTF-8 sequences after decoding.
///
/// # Example
///
/// ```ignore
/// #[derive(Deserialize)]
/// struct MyStruct {
///     #[serde(deserialize_with = "deserialize_blob_name")]
///     name: Option<String>,
/// }
/// ```
pub fn deserialize_blob_name<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
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
        use percent_encoding::percent_decode_str;
        let decoded = percent_decode_str(&content)
            .decode_utf8()
            .map_err(D::Error::custom)?;
        Ok(Some(decoded.into_owned()))
    } else {
        Ok(Some(content))
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
