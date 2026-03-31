// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    fmt::SafeDebug,
    http::{response::AsyncResponseBody, Etag, RawResponse},
};
use std::collections::HashMap;
use time::OffsetDateTime;

use crate::generated::models::{
    BlobType, CopyStatus, ImmutabilityPolicyMode, LeaseDuration, LeaseState, LeaseStatus,
};

/// Result of a `BlockBlobClient::download()` or `BlobClient::download()` operation.
#[derive(SafeDebug)]
pub struct BlockBlobClientDownloadResult {
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

    /// Duration type of the lease (fixed or infinite).
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

    /// Progress of an in-progress copy operation (e.g. `"123/456"`).
    pub copy_progress: Option<String>,

    /// URL of the source blob for the last copy operation.
    pub copy_source: Option<String>,

    /// State of the copy operation.
    pub copy_status: Option<CopyStatus>,

    /// Description of a failed or aborted copy operation.
    pub copy_status_description: Option<String>,

    /// Destination policy ID for object replication.
    pub object_replication_policy_id: Option<String>,

    /// Object replication rule results (`"policyId_ruleId"` → status).
    pub object_replication_rules: HashMap<String, String>,

    /// Number of tags on the blob.
    pub tag_count: Option<i64>,

    /// Raw HTTP response from the initial (first) partition request.
    /// The body has been consumed into `body`; headers are preserved here.
    pub raw_response: RawResponse,
}
