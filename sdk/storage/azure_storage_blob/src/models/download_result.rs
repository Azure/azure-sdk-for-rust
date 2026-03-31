// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    base64,
    fmt::SafeDebug,
    http::{
        headers::{HeaderName, Headers},
        response::AsyncResponseBody,
        Etag,
    },
    time::parse_rfc7231,
};
use std::collections::HashMap;
use time::OffsetDateTime;

use crate::generated::models::{
    BlobType, CopyStatus, ImmutabilityPolicyMode, LeaseDuration, LeaseState, LeaseStatus,
};

/// Result of a `BlobClient::download()` operation.
#[derive(SafeDebug)]
pub struct BlobClientDownloadResult {
    /// The blob content stream.
    pub body: AsyncResponseBody,

    /// The blob's ETag, from the initial partition response.
    pub etag: Option<Etag>,

    /// Date/time the blob was last modified.
    pub last_modified: Option<OffsetDateTime>,

    /// Date/time the blob was created.
    pub created_on: Option<OffsetDateTime>,

    /// Date/time the blob was last read or written.
    pub last_accessed: Option<OffsetDateTime>,

    /// Total size of the blob in bytes.
    pub content_length: Option<u64>,

    /// The content type of the blob.
    pub content_type: Option<String>,

    /// The cache control directive for the blob.
    pub cache_control: Option<String>,

    /// The content disposition of the blob.
    pub content_disposition: Option<String>,

    /// The content encoding of the blob.
    pub content_encoding: Option<String>,

    /// The content language of the blob.
    pub content_language: Option<String>,

    /// The content range returned, if a range was requested.
    pub content_range: Option<String>,

    /// MD5 hash of the response content (Content-MD5 header).
    pub content_hash: Option<Vec<u8>>,

    /// CRC-64 hash of the response content (x-ms-content-crc64 header).
    pub content_crc64: Option<Vec<u8>>,

    /// The stored MD5 hash of the full blob content (x-ms-blob-content-md5 header).
    /// Only returned for ranged reads; may differ from `content_hash`.
    pub blob_content_hash: Option<Vec<u8>>,

    /// The type of blob (BlockBlob, PageBlob, AppendBlob).
    pub blob_type: Option<BlobType>,

    /// Current sequence number for a page blob.
    pub blob_sequence_number: Option<i64>,

    /// Number of committed blocks, returned only for append blobs.
    pub blob_committed_block_count: Option<i32>,

    /// Whether this append blob has been sealed.
    pub is_sealed: Option<bool>,

    /// User-defined metadata on the blob.
    pub metadata: HashMap<String, String>,

    /// Whether the blob is server-side encrypted.
    pub is_server_encrypted: Option<bool>,

    /// Encryption scope used, if any.
    pub encryption_scope: Option<String>,

    /// SHA-256 hash of the customer-provided encryption key, if any.
    pub encryption_key_sha256: Option<String>,

    /// Version ID of the blob, if versioning is enabled.
    pub version_id: Option<String>,

    /// State of the blob's lease.
    pub lease_state: Option<LeaseState>,

    /// Status of the blob's lease.
    pub lease_status: Option<LeaseStatus>,

    /// Duration type of the lease.
    pub lease_duration: Option<LeaseDuration>,

    /// Whether the blob has a legal hold.
    pub legal_hold: Option<bool>,

    /// The immutability policy mode.
    pub immutability_policy_mode: Option<ImmutabilityPolicyMode>,

    /// The immutability policy expiry time.
    pub immutability_policy_expires_on: Option<OffsetDateTime>,

    /// Completion time of the last copy operation.
    pub copy_completed_on: Option<OffsetDateTime>,

    /// String identifier for the last copy operation.
    pub copy_id: Option<String>,

    /// Progress of an in-progress copy operation.
    pub copy_progress: Option<String>,

    /// URL of the source blob for the last copy operation.
    pub copy_source: Option<String>,

    /// State of the copy operation.
    pub copy_status: Option<CopyStatus>,

    /// Description of a failed or aborted copy operation.
    pub copy_status_description: Option<String>,

    /// Destination policy ID for object replication.
    pub object_replication_policy_id: Option<String>,

    /// Object replication rules and statuses.
    pub object_replication_rules: HashMap<String, String>,

    /// Number of tags on the blob.
    pub tag_count: Option<i64>,
}

impl BlobClientDownloadResult {
    /// Constructs a `BlobClientDownloadResult` by parsing headers from the initial response.
    pub(crate) fn from_headers(
        headers: Headers,
        body: azure_core::http::response::PinnedStream,
    ) -> azure_core::Result<Self> {
        let (metadata, object_replication_rules) =
            crate::parsers::parse_metadata_and_replication_headers(&headers);
        Ok(Self {
            body: AsyncResponseBody::new(body),
            etag: headers.get_optional_as(&HeaderName::from_static("etag"))?,
            last_modified: headers
                .get_optional_with(&HeaderName::from_static("last-modified"), |h| {
                    parse_rfc7231(h.as_str())
                })?,
            created_on: headers
                .get_optional_with(&HeaderName::from_static("x-ms-creation-time"), |h| {
                    parse_rfc7231(h.as_str())
                })?,
            last_accessed: headers
                .get_optional_with(&HeaderName::from_static("x-ms-last-access-time"), |h| {
                    parse_rfc7231(h.as_str())
                })?,
            content_length: headers.get_optional_as(&HeaderName::from_static("content-length"))?,
            content_type: headers.get_optional_as(&HeaderName::from_static("content-type"))?,
            cache_control: headers.get_optional_as(&HeaderName::from_static("cache-control"))?,
            content_disposition: headers
                .get_optional_as(&HeaderName::from_static("content-disposition"))?,
            content_encoding: headers
                .get_optional_as(&HeaderName::from_static("content-encoding"))?,
            content_language: headers
                .get_optional_as(&HeaderName::from_static("content-language"))?,
            content_range: headers.get_optional_as(&HeaderName::from_static("content-range"))?,
            content_hash: headers
                .get_optional_with(&HeaderName::from_static("content-md5"), |h| {
                    base64::decode(h.as_str())
                })?,
            content_crc64: headers
                .get_optional_with(&HeaderName::from_static("x-ms-content-crc64"), |h| {
                    base64::decode(h.as_str())
                })?,
            blob_content_hash: headers
                .get_optional_with(&HeaderName::from_static("x-ms-blob-content-md5"), |h| {
                    base64::decode(h.as_str())
                })?,
            blob_type: headers.get_optional_as(&HeaderName::from_static("x-ms-blob-type"))?,
            blob_sequence_number: headers
                .get_optional_as(&HeaderName::from_static("x-ms-blob-sequence-number"))?,
            blob_committed_block_count: headers
                .get_optional_as(&HeaderName::from_static("x-ms-blob-committed-block-count"))?,
            is_sealed: headers.get_optional_as(&HeaderName::from_static("x-ms-blob-sealed"))?,
            is_server_encrypted: headers
                .get_optional_as(&HeaderName::from_static("x-ms-server-encrypted"))?,
            encryption_scope: headers
                .get_optional_as(&HeaderName::from_static("x-ms-encryption-scope"))?,
            encryption_key_sha256: headers
                .get_optional_as(&HeaderName::from_static("x-ms-encryption-key-sha256"))?,
            version_id: headers.get_optional_as(&HeaderName::from_static("x-ms-version-id"))?,
            lease_state: headers.get_optional_as(&HeaderName::from_static("x-ms-lease-state"))?,
            lease_status: headers.get_optional_as(&HeaderName::from_static("x-ms-lease-status"))?,
            lease_duration: headers
                .get_optional_as(&HeaderName::from_static("x-ms-lease-duration"))?,
            legal_hold: headers.get_optional_as(&HeaderName::from_static("x-ms-legal-hold"))?,
            immutability_policy_mode: headers
                .get_optional_as(&HeaderName::from_static("x-ms-immutability-policy-mode"))?,
            immutability_policy_expires_on: headers.get_optional_with(
                &HeaderName::from_static("x-ms-immutability-policy-until-date"),
                |h| parse_rfc7231(h.as_str()),
            )?,
            copy_completed_on: headers
                .get_optional_with(&HeaderName::from_static("x-ms-copy-completion-time"), |h| {
                    parse_rfc7231(h.as_str())
                })?,
            copy_id: headers.get_optional_as(&HeaderName::from_static("x-ms-copy-id"))?,
            copy_progress: headers
                .get_optional_as(&HeaderName::from_static("x-ms-copy-progress"))?,
            copy_source: headers.get_optional_as(&HeaderName::from_static("x-ms-copy-source"))?,
            copy_status: headers.get_optional_as(&HeaderName::from_static("x-ms-copy-status"))?,
            copy_status_description: headers
                .get_optional_as(&HeaderName::from_static("x-ms-copy-status-description"))?,
            object_replication_policy_id: headers
                .get_optional_as(&HeaderName::from_static("x-ms-or-policy-id"))?,
            tag_count: headers.get_optional_as(&HeaderName::from_static("x-ms-tag-count"))?,
            metadata,
            object_replication_rules,
        })
    }
}
