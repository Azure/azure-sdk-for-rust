// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    fmt::SafeDebug,
    http::{Etag, RawResponse},
};
use time::OffsetDateTime;

/// Result of a `BlockBlobClient::upload()` or `BlobClient::upload()` operation.
#[derive(SafeDebug)]
pub struct BlockBlobClientUploadResult {
    /// The blob's ETag.
    pub etag: Option<Etag>,

    /// The date/time the blob was last modified.
    pub last_modified: Option<OffsetDateTime>,

    /// The MD5 hash of the blob content, if present.
    pub content_md5: Option<Vec<u8>>,

    /// The CRC64 hash of the blob content, if present.
    pub content_crc64: Option<Vec<u8>>,

    /// The SHA-256 hash of the encryption key used to encrypt the blob, if any.
    pub encryption_key_sha256: Option<String>,

    /// The encryption scope used to encrypt the blob, if any.
    pub encryption_scope: Option<String>,

    /// Whether the blob was successfully encrypted server-side.
    pub is_server_encrypted: Option<bool>,

    /// The version ID of the blob, if versioning is enabled on the storage account.
    pub version_id: Option<String>,

    /// The raw HTTP response.
    pub raw_response: RawResponse,
}
