// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{collections::HashMap, num::NonZero};

use azure_core::{
    fmt::SafeDebug,
    http::{ClientMethodOptions, Etag},
};
use time::OffsetDateTime;

use crate::models::{
    AccessTier, BlobClientDownloadInternalOptions, EncryptionAlgorithmType, HttpRange,
    ImmutabilityPolicyMode,
};

/// Options to be passed to `BlobClient::download()`
#[derive(Clone, Default, SafeDebug)]
pub struct BlobClientDownloadOptions<'a> {
    /// The algorithm used to produce the encryption key hash. Must be provided if the encryption key is provided.
    pub encryption_algorithm: Option<EncryptionAlgorithmType>,

    /// Specifies the encryption key to use to encrypt the data provided in the request.
    pub encryption_key: Option<String>,

    /// The SHA-256 hash of the provided encryption key. Must be provided if the encryption key is provided.
    pub encryption_key_sha256: Option<String>,

    /// Specify this value to operate only on a blob with a matching Etag value.
    pub if_match: Option<Etag>,

    /// Specify this value to operate only on a blob if it has been modified since the specified date-time.
    pub if_modified_since: Option<OffsetDateTime>,

    /// Specify this value to operate only on a blob with a non-matching Etag value.
    pub if_none_match: Option<Etag>,

    /// Specifies a SQL-like where clause on blob tags to operate only on a blob with matching tags.
    pub if_tags: Option<String>,

    /// Specify this value to operate only on a blob if it has not been modified since the specified date-time.
    pub if_unmodified_since: Option<OffsetDateTime>,

    /// If specified, the operation only succeeds if the resource's lease is active and matches this ID.
    pub lease_id: Option<String>,

    /// Allows customization of the method call.
    pub method_options: ClientMethodOptions<'a>,

    /// Optional. Number of concurrent network transfers to maintain for this operation.
    /// A default value will be chosen if none is provided.
    pub parallel: Option<NonZero<usize>>,

    /// Optional. Size to partition data into.
    /// A default value will be chosen if none is provided.
    pub partition_size: Option<NonZero<usize>>,

    /// Optional range of the blob to download.
    ///
    /// Accepts an [`HttpRange`] value. You can convert from standard Rust range types
    /// using `.into()`, for example `(0..100u64).into()` or `(100u64..).into()`.
    ///
    /// When set to `None`, the entire blob will be downloaded.
    pub range: Option<HttpRange>,

    /// When set to true and the request includes a Range header, the service returns the CRC64 hash for the range, as long as
    /// the range is less than or equal to 4 MiB in size.
    pub range_get_content_crc64: Option<bool>,

    /// When set to true and specified together with the `Range` header, the service returns the MD5 hash for the range, as long
    /// as the range is less than or equal to 4 MiB in size.
    pub range_get_content_md5: Option<bool>,

    /// Specifies the snapshot of the blob.
    pub snapshot: Option<String>,

    /// The timeout parameter is expressed in seconds. For more information, see [Setting Timeouts for Blob Service Operations.](https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations)
    pub timeout: Option<i32>,

    /// Specifies the version ID of the blob.
    pub version_id: Option<String>,
}

impl<'a> From<BlobClientDownloadOptions<'a>> for BlobClientDownloadInternalOptions<'_> {
    fn from(value: BlobClientDownloadOptions) -> Self {
        // Construct exhaustively to catch new options.
        Self {
            encryption_algorithm: value.encryption_algorithm,
            encryption_key: value.encryption_key,
            encryption_key_sha256: value.encryption_key_sha256,
            if_match: value.if_match,
            if_modified_since: value.if_modified_since,
            if_none_match: value.if_none_match,
            if_tags: value.if_tags,
            if_unmodified_since: value.if_unmodified_since,
            lease_id: value.lease_id,
            // requires into_owned due to BlobClientDownloadBehavior w/ 'static Behavior
            method_options: ClientMethodOptions {
                context: value.method_options.context.into_owned(),
            },
            range: None,
            range_get_content_crc64: value.range_get_content_crc64,
            range_get_content_md5: value.range_get_content_md5,
            snapshot: value.snapshot,
            structured_body_type: None,
            timeout: value.timeout,
            version_id: value.version_id,
        }
    }
}

/// Options to be passed to `BlockBlobClient::upload()`
#[derive(Clone, Default, SafeDebug)]
pub struct BlockBlobClientUploadOptions<'a> {
    /// Specifies the blob's Cache-Control. If specified, this property is stored with the blob and returned with a read request.
    pub blob_cache_control: Option<String>,

    /// Specifies the blob's Content-Disposition. If specified, this property is stored with the blob and returned with a read
    /// request.
    pub blob_content_disposition: Option<String>,

    /// Specifies the blob's Content-Encoding. If specified, this property is stored with the blob and returned with a read request.
    pub blob_content_encoding: Option<String>,

    /// Specifies the blob's Content-Language. If specified, this property is stored with the blob and returned with a read request.
    pub blob_content_language: Option<String>,

    /// The MD5 hash of the blob content that is stored as a property on the blob. Note: This hash is not validated.
    pub blob_content_md5: Option<Vec<u8>>,

    /// Specifies the blob's Content-Type. If specified, this property is stored with the blob and returned with a read request.
    pub blob_content_type: Option<String>,

    /// The blob tags.
    ///
    /// This is the percent-encoded `x-ms-tags` header value (`key=value&key2=value2`).
    /// Use [`Self::with_tags`] to set this from a `HashMap<String, String>` or `BlobTags`.
    pub blob_tags_string: Option<String>,

    /// The algorithm used to produce the encryption key hash. Must be provided if the encryption key is provided.
    pub encryption_algorithm: Option<EncryptionAlgorithmType>,

    /// Specifies the encryption key to use to encrypt the data provided in the request.
    pub encryption_key: Option<String>,

    /// The SHA-256 hash of the provided encryption key. Must be provided if the encryption key is provided.
    pub encryption_key_sha256: Option<String>,

    /// Specifies the encryption scope used to encrypt the data.
    pub encryption_scope: Option<String>,

    /// The date-time that indicates the time at which the blob immutability policy will expire.
    pub immutability_policy_expiry: Option<OffsetDateTime>,

    /// Indicates the immutability policy mode of the blob.
    pub immutability_policy_mode: Option<ImmutabilityPolicyMode>,

    /// Specify this value to operate only on a blob with a matching Etag value.
    pub if_match: Option<Etag>,

    /// Specify this value to operate only on a blob if it has been modified since the specified date-time.
    pub if_modified_since: Option<OffsetDateTime>,

    /// Specify this value to operate only on a blob with a non-matching Etag value.
    pub if_none_match: Option<Etag>,

    /// Specifies a SQL-like where clause on blob tags to operate only on a blob with matching tags.
    pub if_tags: Option<String>,

    /// Specify this value to operate only on a blob if it has not been modified since the specified date-time.
    pub if_unmodified_since: Option<OffsetDateTime>,

    /// If specified, the operation only succeeds if the resource's lease is active and matches this ID.
    pub lease_id: Option<String>,

    /// Indicates whether the blob has a legal hold.
    pub legal_hold: Option<bool>,

    /// The metadata headers.
    pub metadata: Option<HashMap<String, String>>,

    /// Allows customization of the method call.
    pub method_options: ClientMethodOptions<'a>,

    /// Optional. Number of concurrent network transfers to maintain for this operation.
    /// A default value will be chosen if none is provided.
    pub parallel: Option<NonZero<usize>>,

    /// Optional. Size to partition data into.
    /// A default value will be chosen if none is provided.
    pub partition_size: Option<NonZero<u64>>,

    /// Optional. The server-side timeout to apply on each individual request. This is not a timeout for the whole operation.
    /// The timeout parameter is expressed in seconds. For more information, see
    /// [Setting Timeouts for Blob Service Operations.](https://docs.microsoft.com/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations)
    pub per_request_timeout: Option<i32>,

    /// The tier to be set on the blob.
    pub tier: Option<AccessTier>,
}
