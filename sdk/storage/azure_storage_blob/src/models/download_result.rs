// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    base64,
    fmt::SafeDebug,
    http::{
        headers::{HeaderName, Headers},
        response::AsyncResponseBody,
        AsyncRawResponse, Etag,
    },
    time::parse_rfc7231,
};
use std::collections::HashMap;
use time::OffsetDateTime;

use crate::generated::models::{
    BlobType, CopyStatus, ImmutabilityPolicyMode, LeaseDuration, LeaseState, LeaseStatus,
};

/// Result of a `BlobClient::download()` operation.
///
/// The full blob content over the requested range is streamed via [`body`](Self::body).
/// All other fields: [`properties`](Self::properties), and [`headers`](Self::headers), are parsed
/// from the **initial response**.
#[derive(SafeDebug)]
pub struct BlobClientDownloadResult {
    /// The blob content stream.
    pub body: AsyncResponseBody,

    /// Blob properties parsed from the initial response.
    pub properties: BlobDownloadProperties,

    /// All headers from the initial response.
    ///
    /// Use this to access headers that are not surfaced as named fields, such as
    /// `x-ms-request-id`, `x-ms-client-request-id`, and more.
    pub headers: Headers,
}

/// Blob properties parsed from the initial response headers of a `BlobClient::download()` operation.
#[derive(SafeDebug)]
pub struct BlobDownloadProperties {
    /// The blob's ETag (`ETag` header).
    pub etag: Option<Etag>,

    /// Date/time the blob was last modified (`Last-Modified` header).
    pub last_modified: Option<OffsetDateTime>,

    /// Date/time the blob was created (`x-ms-creation-time` header).
    pub created_on: Option<OffsetDateTime>,

    /// Date/time the blob was last read or written (`x-ms-last-access-time` header).
    pub last_accessed: Option<OffsetDateTime>,

    /// Number of bytes in the response body (`Content-Length` header).
    ///
    /// For a full-blob download this equals the total blob size. For a ranged or partitioned
    /// download, this is the byte count of the initial response.
    pub content_length: Option<u64>,

    /// The content type of the blob (`Content-Type` header).
    pub content_type: Option<String>,

    /// The cache control directive for the blob (`Cache-Control` header).
    pub cache_control: Option<String>,

    /// The content disposition of the blob (`Content-Disposition` header).
    pub content_disposition: Option<String>,

    /// The content encoding of the blob (`Content-Encoding` header).
    pub content_encoding: Option<String>,

    /// The content language of the blob (`Content-Language` header).
    pub content_language: Option<String>,

    /// MD5 hash of the response content (`Content-MD5` header).
    ///
    /// For full-blob reads this covers the entire blob. For ranged reads it covers only the
    /// returned range. To get the MD5 of the full blob content for a ranged read, see
    /// [`blob_content_md5`](Self::blob_content_md5).
    pub content_md5: Option<Vec<u8>>,

    /// CRC-64 hash for the downloaded range (`x-ms-content-crc64` header).
    ///
    /// Only returned for ranged reads (when [`range`](crate::models::BlobClientDownloadOptions::range) is set)
    /// and only when [`range_get_content_crc64`](crate::models::BlobClientDownloadOptions::range_get_content_crc64) is `true`.
    /// The range must be 4 MiB or smaller, otherwise the service rejects the request.
    pub content_crc64: Option<Vec<u8>>,

    /// MD5 hash of the full blob content (`x-ms-blob-content-md5` header).
    ///
    /// Only returned for ranged reads. This is the hash of the complete blob, not just the
    /// requested range.
    pub blob_content_md5: Option<Vec<u8>>,

    /// The type of blob (`x-ms-blob-type` header): BlockBlob, PageBlob, or AppendBlob.
    pub blob_type: Option<BlobType>,

    /// Current sequence number for a page blob (`x-ms-blob-sequence-number` header).
    pub blob_sequence_number: Option<i64>,

    /// Number of committed blocks, returned only for append blobs
    /// (`x-ms-blob-committed-block-count` header).
    pub blob_committed_block_count: Option<i32>,

    /// Whether this append blob has been sealed (`x-ms-blob-sealed` header).
    pub is_sealed: Option<bool>,

    /// User-defined metadata on the blob (all `x-ms-meta-*` headers).
    pub metadata: HashMap<String, String>,

    /// Version ID of the blob, if versioning is enabled (`x-ms-version-id` header).
    pub version_id: Option<String>,

    /// State of the blob's lease (`x-ms-lease-state` header).
    pub lease_state: Option<LeaseState>,

    /// Status of the blob's lease (`x-ms-lease-status` header).
    pub lease_status: Option<LeaseStatus>,

    /// Duration type of the lease (`x-ms-lease-duration` header).
    pub lease_duration: Option<LeaseDuration>,

    /// Whether the blob has a legal hold (`x-ms-legal-hold` header).
    pub legal_hold: Option<bool>,

    /// The immutability policy mode (`x-ms-immutability-policy-mode` header).
    pub immutability_policy_mode: Option<ImmutabilityPolicyMode>,

    /// The immutability policy expiry time (`x-ms-immutability-policy-until-date` header).
    pub immutability_policy_expires_on: Option<OffsetDateTime>,

    /// Completion time of the last copy operation (`x-ms-copy-completion-time` header).
    pub copy_completed_on: Option<OffsetDateTime>,

    /// String identifier for the last copy operation (`x-ms-copy-id` header).
    pub copy_id: Option<String>,

    /// Progress of an in-progress copy operation (`x-ms-copy-progress` header).
    pub copy_progress: Option<String>,

    /// URL of the source blob for the last copy operation (`x-ms-copy-source` header).
    pub copy_source: Option<String>,

    /// State of the copy operation (`x-ms-copy-status` header).
    pub copy_status: Option<CopyStatus>,

    /// Description of a failed or aborted copy operation (`x-ms-copy-status-description` header).
    pub copy_status_description: Option<String>,

    /// Destination policy ID for object replication (`x-ms-or-policy-id` header).
    pub object_replication_policy_id: Option<String>,

    /// Object replication rules and their statuses (all `x-ms-or-*` headers).
    pub object_replication_rules: HashMap<String, String>,

    /// Number of tags on the blob (`x-ms-tag-count` header).
    pub tag_count: Option<i64>,

    /// The name of the encryption scope used to encrypt the blob (`x-ms-encryption-scope` header).
    pub encryption_scope: Option<String>,

    /// Base64-encoded SHA-256 hash of the customer-provided encryption key
    /// used to encrypt the blob (`x-ms-encryption-key-sha256` header).
    pub encryption_key_sha256: Option<String>,
}

impl BlobClientDownloadResult {
    /// Constructs a `BlobClientDownloadResult` by parsing headers from the initial response.
    pub(crate) fn from_headers(response: AsyncRawResponse) -> azure_core::Result<Self> {
        let (_, headers, body) = response.deconstruct();
        let properties = BlobDownloadProperties::from_headers(&headers)?;
        Ok(Self {
            body,
            properties,
            headers,
        })
    }
}

impl BlobDownloadProperties {
    fn from_headers(headers: &Headers) -> azure_core::Result<Self> {
        let (metadata, object_replication_rules) =
            crate::parsers::parse_metadata_and_replication_headers(headers);
        Ok(Self {
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
            content_md5: headers
                .get_optional_with(&HeaderName::from_static("content-md5"), |h| {
                    base64::decode(h.as_str())
                })?,
            content_crc64: headers
                .get_optional_with(&HeaderName::from_static("x-ms-content-crc64"), |h| {
                    base64::decode(h.as_str())
                })?,
            blob_content_md5: headers
                .get_optional_with(&HeaderName::from_static("x-ms-blob-content-md5"), |h| {
                    base64::decode(h.as_str())
                })?,
            blob_type: headers.get_optional_as(&HeaderName::from_static("x-ms-blob-type"))?,
            blob_sequence_number: headers
                .get_optional_as(&HeaderName::from_static("x-ms-blob-sequence-number"))?,
            blob_committed_block_count: headers
                .get_optional_as(&HeaderName::from_static("x-ms-blob-committed-block-count"))?,
            is_sealed: headers.get_optional_as(&HeaderName::from_static("x-ms-blob-sealed"))?,
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
            encryption_scope: headers
                .get_optional_as(&HeaderName::from_static("x-ms-encryption-scope"))?,
            encryption_key_sha256: headers
                .get_optional_as(&HeaderName::from_static("x-ms-encryption-key-sha256"))?,
            metadata,
            object_replication_rules,
        })
    }
}
