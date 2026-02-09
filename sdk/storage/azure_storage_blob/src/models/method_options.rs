// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{collections::HashMap, num::NonZero, ops::Range};

use azure_core::{fmt::SafeDebug, http::ClientMethodOptions};
use time::OffsetDateTime;

use crate::models::{AccessTier, EncryptionAlgorithmType, ImmutabilityPolicyMode};

/// Options to be passed to `BlockBlobClient::managed_download()`
#[derive(Clone, Default, SafeDebug)]
pub struct BlobClientManagedDownloadOptions<'a> {
    /// Optional. Version 2019-07-07 and later. Specifies the algorithm to use for encryption. If not specified, the default is
    /// AES256.
    pub encryption_algorithm: Option<EncryptionAlgorithmType>,

    /// Optional. Version 2019-07-07 and later. Specifies the encryption key to use to encrypt the data provided in the request.
    /// If not specified, the request will be encrypted with the root account key.
    pub encryption_key: Option<String>,

    /// Optional. Version 2019-07-07 and later. Specifies the SHA256 hash of the encryption key used to encrypt the data provided
    /// in the request. This header is only used for encryption with a customer-provided key. If the request is authenticated
    /// with a client token, this header should be specified using the SHA256 hash of the encryption key.
    pub encryption_key_sha256: Option<String>,

    /// The request should only proceed if an entity matches this string.
    pub if_match: Option<String>,

    /// The request should only proceed if the entity was modified after this time.
    pub if_modified_since: Option<OffsetDateTime>,

    /// The request should only proceed if no entity matches this string.
    pub if_none_match: Option<String>,

    /// Specify a SQL where clause on blob tags to operate only on blobs with a matching value.
    pub if_tags: Option<String>,

    /// The request should only proceed if the entity was not modified after this time.
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

    /// Optional. Range of the blob to download.
    /// None will result in the entire blob being downloaded.
    pub range: Option<Range<u64>>,

    /// Optional. When this header is set to true and specified together with the Range header, the service returns the CRC64
    /// hash for the range, as long as the range is less than or equal to 4 MB in size.
    pub range_get_content_crc64: Option<bool>,

    /// When set to true and specified together with the Range, the service returns the MD5 hash for the range, as long as the
    /// range is less than or equal to 4 MB in size.
    pub range_get_content_md5: Option<bool>,

    /// The snapshot parameter is an opaque DateTime value that, when present, specifies the blob snapshot to retrieve. For more
    /// information on working with blob snapshots, see [Creating a Snapshot of a Blob.](https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/creating-a-snapshot-of-a-blob)
    pub snapshot: Option<String>,

    /// Specifies the response content should be returned as a structured message and specifies the message schema version and
    /// properties.
    pub structured_body_type: Option<String>,

    /// The timeout parameter is expressed in seconds. For more information, see [Setting Timeouts for Blob Service Operations.](https://docs.microsoft.com/en-us/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations)
    pub timeout: Option<i32>,

    /// The version id parameter is an opaque DateTime value that, when present, specifies the version of the blob to operate
    /// on. It's for service version 2019-10-10 and newer.
    pub version_id: Option<String>,
}

/// Options to be passed to `BlockBlobClient::managed_upload()`
#[derive(Clone, Default, SafeDebug)]
pub struct BlockBlobClientManagedUploadOptions<'a> {
    /// Optional. Sets the blob's cache control. If specified, this property is stored with the blob and returned with a read
    /// request.
    pub blob_cache_control: Option<String>,

    /// Optional. Sets the blob's content disposition. If specified, this property is stored with the blob and returned with a
    /// read request.
    pub blob_content_disposition: Option<String>,

    /// Optional. Sets the blob's content encoding. If specified, this property is stored with the blob and returned with a read
    /// request.
    pub blob_content_encoding: Option<String>,

    /// Optional. Set the blob's content language. If specified, this property is stored with the blob and returned with a read
    /// request.
    pub blob_content_language: Option<String>,

    /// Optional. An MD5 hash of the blob content. Note that this hash is not validated, as the hashes for the individual blocks
    /// were validated when each was uploaded.
    pub blob_content_md5: Option<Vec<u8>>,

    /// Optional. Sets the blob's content type. If specified, this property is stored with the blob and returned with a read request.
    pub blob_content_type: Option<String>,

    /// Optional. Used to set blob tags in various blob operations.
    pub blob_tags_string: Option<String>,

    /// Optional. Version 2019-07-07 and later. Specifies the algorithm to use for encryption. If not specified, the default is
    /// AES256.
    pub encryption_algorithm: Option<EncryptionAlgorithmType>,

    /// Optional. Version 2019-07-07 and later. Specifies the encryption key to use to encrypt the data provided in the request.
    /// If not specified, the request will be encrypted with the root account key.
    pub encryption_key: Option<String>,

    /// Optional. Version 2019-07-07 and later. Specifies the SHA256 hash of the encryption key used to encrypt the data provided
    /// in the request. This header is only used for encryption with a customer-provided key. If the request is authenticated
    /// with a client token, this header should be specified using the SHA256 hash of the encryption key.
    pub encryption_key_sha256: Option<String>,

    /// Optional. Version 2019-07-07 and later. Specifies the encryption scope to use to encrypt the data provided in the request.
    /// If not specified, the request will be encrypted with the root account key.
    pub encryption_scope: Option<String>,

    /// Optional. Specifies the date time when the blobs immutability policy is set to expire.
    pub immutability_policy_expiry: Option<OffsetDateTime>,

    /// Optional. Specifies the immutability policy mode to set on the blob.
    pub immutability_policy_mode: Option<ImmutabilityPolicyMode>,

    /// Optional. Applied only to the final commit of the new block blob.
    /// A condition that must be met in order for the request to be processed.
    pub if_match: Option<String>,

    /// Optional. Applied only to the final commit of the new block blob.
    /// A date-time value. A request is made under the condition that the resource has been modified since the specified date-time.
    pub if_modified_since: Option<OffsetDateTime>,

    /// Optional. Applied only to the final commit of the new block blob.
    /// A condition that must be met in order for the request to be processed.
    pub if_none_match: Option<String>,

    /// Optional. Applied only to the final commit of the new block blob.
    /// Specify a SQL where clause on blob tags to operate only on blobs with a matching value.
    pub if_tags: Option<String>,

    /// Optional. Applied only to the final commit of the new block blob.
    /// A date-time value. A request is made under the condition that the resource has not been modified since the specified date-time.
    pub if_unmodified_since: Option<OffsetDateTime>,

    /// Optional. Applied to all requests.
    /// If specified, the operation only succeeds if the resource's lease is active and matches this ID.
    pub lease_id: Option<String>,

    /// Optional. Specified if a legal hold should be set on the blob.
    pub legal_hold: Option<bool>,

    /// Optional. The metadata headers.
    pub metadata: Option<HashMap<String, String>>,

    /// Optional. Allows customization of the method call.
    pub method_options: ClientMethodOptions<'a>,

    /// Optional. Number of concurrent network transfers to maintain for this operation.
    /// A default value will be chosen if none is provided.
    pub parallel: Option<NonZero<usize>>,

    /// Optional. Size to partition data into.
    /// A default value will be chosen if none is provided.
    pub partition_size: Option<NonZero<usize>>,

    /// Optional. The server-side timeout to apply on each individual request. This is not a timeout for the whole operation.
    /// The timeout parameter is expressed in seconds. For more information, see
    /// [Setting Timeouts for Blob Service Operations.](https://docs.microsoft.com/rest/api/storageservices/fileservices/setting-timeouts-for-blob-service-operations)
    pub per_request_timeout: Option<i32>,

    /// Optional. The tier to be set on the blob.
    pub tier: Option<AccessTier>,
}
