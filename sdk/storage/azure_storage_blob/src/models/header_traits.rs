// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::Result;
use azure_core::{
    base64,
    http::{
        headers::{HeaderName, Headers},
        NoFormat, Response,
    },
};

use crate::models::BlockBlobClientStageBlockResult;

const CONTENT_CRC64: HeaderName = HeaderName::from_static("x-ms-content-crc64");
const CONTENT_MD5: HeaderName = HeaderName::from_static("content-md5");
const ENCRYPTION_KEY_SHA256: HeaderName = HeaderName::from_static("x-ms-encryption-key-sha256");
const ENCRYPTION_SCOPE: HeaderName = HeaderName::from_static("x-ms-encryption-scope");
const REQUEST_SERVER_ENCRYPTED: HeaderName =
    HeaderName::from_static("x-ms-request-server-encrypted");

/// Provides access to typed response headers for `BlockBlobClient::stage_block_internal()`
///
/// # Examples
///
/// ```ignore
/// use azure_core::{Result, http::{Response, NoFormat}};
/// use azure_storage_blob::models::{BlockBlobClientStageBlockResult, BlockBlobClientStageBlockResultHeaders};
/// async fn example() -> Result<()> {
///     let response: Response<BlockBlobClientStageBlockResult, NoFormat> = unimplemented!();
///     // Access response headers
///     if let Some(content_md5) = response.content_md5()? {
///         println!("content-md5: {:?}", content_md5);
///     }
///     if let Some(content_crc64) = response.content_crc64()? {
///         println!("x-ms-content-crc64: {:?}", content_crc64);
///     }
///     if let Some(encryption_key_sha256) = response.encryption_key_sha256()? {
///         println!("x-ms-encryption-key-sha256: {:?}", encryption_key_sha256);
///     }
///     Ok(())
/// }
/// ```
pub trait BlockBlobClientStageBlockResultHeaders: private::Sealed {
    fn content_md5(&self) -> Result<Option<Vec<u8>>>;
    fn content_crc64(&self) -> Result<Option<Vec<u8>>>;
    fn encryption_key_sha256(&self) -> Result<Option<String>>;
    fn encryption_scope(&self) -> Result<Option<String>>;
    fn is_server_encrypted(&self) -> Result<Option<bool>>;
}

impl BlockBlobClientStageBlockResultHeaders
    for Response<BlockBlobClientStageBlockResult, NoFormat>
{
    /// The blob content MD5 hash. Only returned if the full blob is read.
    fn content_md5(&self) -> Result<Option<Vec<u8>>> {
        Headers::get_optional_with(self.headers(), &CONTENT_MD5, |h| base64::decode(h.as_str()))
    }

    /// The CRC64 hash of the content.
    fn content_crc64(&self) -> Result<Option<Vec<u8>>> {
        Headers::get_optional_with(self.headers(), &CONTENT_CRC64, |h| {
            base64::decode(h.as_str())
        })
    }

    /// The SHA-256 hash of the provided encryption key.
    fn encryption_key_sha256(&self) -> Result<Option<String>> {
        Headers::get_optional_as(self.headers(), &ENCRYPTION_KEY_SHA256)
    }

    /// Specifies the encryption scope used to encrypt the data.
    fn encryption_scope(&self) -> Result<Option<String>> {
        Headers::get_optional_as(self.headers(), &ENCRYPTION_SCOPE)
    }

    /// Indicates whether the contents of the request are successfully encrypted.
    fn is_server_encrypted(&self) -> Result<Option<bool>> {
        Headers::get_optional_as(self.headers(), &REQUEST_SERVER_ENCRYPTED)
    }
}

mod private {
    use super::BlockBlobClientStageBlockResult;
    use azure_core::http::{NoFormat, Response};

    pub trait Sealed {}

    impl Sealed for Response<BlockBlobClientStageBlockResult, NoFormat> {}
}
