// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::headers::HeaderValue;
use std::fmt;

pub(crate) mod content_range;
pub mod error;
mod extensions;
pub mod method_options;

/// Represents an HTTP Range header value for blob operations.
///
/// Defines a range of bytes within an HTTP resource, starting at an offset and
/// ending at offset+length-1 inclusively. This matches the semantics of .NET's
/// `Azure.HttpRange`.
///
/// # Examples
///
/// ```
/// use azure_storage_blob::models::HttpRange;
///
/// // Range of 512 bytes starting at offset 0: bytes=0-511
/// let range = HttpRange::new(0, 512);
/// assert_eq!(range.to_string(), "bytes=0-511");
///
/// // Open-ended range starting at offset 255: bytes=255-
/// let range = HttpRange::from_offset(255);
/// assert_eq!(range.to_string(), "bytes=255-");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpRange {
    /// The starting byte offset.
    pub offset: u64,
    /// The length of the range. If `None`, the range extends to the end.
    pub length: Option<u64>,
}

impl HttpRange {
    /// Creates a new `HttpRange` with the specified offset and length.
    ///
    /// The range will cover bytes from `offset` to `offset + length - 1` inclusive.
    ///
    /// # Arguments
    ///
    /// * `offset` - The starting byte offset.
    /// * `length` - The number of bytes in the range.
    pub fn new(offset: u64, length: u64) -> Self {
        Self {
            offset,
            length: Some(length),
        }
    }

    /// Creates a new `HttpRange` that starts at the specified offset and extends to the end.
    ///
    /// # Arguments
    ///
    /// * `offset` - The starting byte offset.
    pub fn from_offset(offset: u64) -> Self {
        Self {
            offset,
            length: None,
        }
    }
}

impl fmt::Display for HttpRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.length {
            Some(length) => write!(f, "bytes={}-{}", self.offset, self.offset + length - 1),
            None => write!(f, "bytes={}-", self.offset),
        }
    }
}

impl From<HttpRange> for String {
    fn from(range: HttpRange) -> Self {
        range.to_string()
    }
}

impl From<HttpRange> for HeaderValue {
    fn from(range: HttpRange) -> Self {
        HeaderValue::from(range.to_string())
    }
}

pub use crate::generated::models::{
    AccessPolicy, AccessTier, AccountKind, AppendBlobClientAppendBlockFromUrlOptions,
    AppendBlobClientAppendBlockFromUrlResult, AppendBlobClientAppendBlockFromUrlResultHeaders,
    AppendBlobClientAppendBlockOptions, AppendBlobClientAppendBlockResult,
    AppendBlobClientAppendBlockResultHeaders, AppendBlobClientCreateOptions,
    AppendBlobClientCreateResult, AppendBlobClientCreateResultHeaders, AppendBlobClientSealOptions,
    AppendBlobClientSealResult, AppendBlobClientSealResultHeaders, ArchiveStatus,
    BlobClientAcquireLeaseOptions, BlobClientAcquireLeaseResult,
    BlobClientAcquireLeaseResultHeaders, BlobClientBreakLeaseOptions, BlobClientBreakLeaseResult,
    BlobClientBreakLeaseResultHeaders, BlobClientChangeLeaseOptions, BlobClientChangeLeaseResult,
    BlobClientChangeLeaseResultHeaders, BlobClientCreateSnapshotOptions,
    BlobClientCreateSnapshotResult, BlobClientCreateSnapshotResultHeaders,
    BlobClientDeleteImmutabilityPolicyOptions, BlobClientDeleteOptions, BlobClientDownloadOptions,
    BlobClientDownloadResult, BlobClientDownloadResultHeaders, BlobClientGetAccountInfoOptions,
    BlobClientGetAccountInfoResult, BlobClientGetAccountInfoResultHeaders,
    BlobClientGetPropertiesOptions, BlobClientGetPropertiesResult,
    BlobClientGetPropertiesResultHeaders, BlobClientGetTagsOptions, BlobClientReleaseLeaseOptions,
    BlobClientReleaseLeaseResult, BlobClientReleaseLeaseResultHeaders, BlobClientRenewLeaseOptions,
    BlobClientRenewLeaseResult, BlobClientRenewLeaseResultHeaders,
    BlobClientSetImmutabilityPolicyOptions, BlobClientSetLegalHoldOptions,
    BlobClientSetMetadataOptions, BlobClientSetPropertiesOptions, BlobClientSetTagsOptions,
    BlobClientSetTierOptions, BlobClientUndeleteOptions, BlobContainerClientAcquireLeaseOptions,
    BlobContainerClientAcquireLeaseResult, BlobContainerClientAcquireLeaseResultHeaders,
    BlobContainerClientBreakLeaseOptions, BlobContainerClientBreakLeaseResult,
    BlobContainerClientBreakLeaseResultHeaders, BlobContainerClientChangeLeaseOptions,
    BlobContainerClientChangeLeaseResult, BlobContainerClientChangeLeaseResultHeaders,
    BlobContainerClientCreateOptions, BlobContainerClientDeleteOptions,
    BlobContainerClientFindBlobsByTagsOptions, BlobContainerClientGetAccessPolicyOptions,
    BlobContainerClientGetAccountInfoResult, BlobContainerClientGetAccountInfoResultHeaders,
    BlobContainerClientGetPropertiesOptions, BlobContainerClientGetPropertiesResult,
    BlobContainerClientGetPropertiesResultHeaders, BlobContainerClientListBlobFlatSegmentOptions,
    BlobContainerClientReleaseLeaseOptions, BlobContainerClientReleaseLeaseResult,
    BlobContainerClientReleaseLeaseResultHeaders, BlobContainerClientRenewLeaseOptions,
    BlobContainerClientRenewLeaseResult, BlobContainerClientRenewLeaseResultHeaders,
    BlobContainerClientSetAccessPolicyOptions, BlobContainerClientSetMetadataOptions,
    BlobCopySourceTags, BlobDeleteType, BlobFlatListSegment, BlobItem, BlobMetadata, BlobName,
    BlobProperties, BlobServiceClientFindBlobsByTagsOptions,
    BlobServiceClientGetAccountInfoOptions, BlobServiceClientGetAccountInfoResult,
    BlobServiceClientGetAccountInfoResultHeaders, BlobServiceClientGetPropertiesOptions,
    BlobServiceClientGetStatisticsOptions, BlobServiceClientListContainersSegmentOptions,
    BlobServiceClientSetPropertiesOptions, BlobServiceProperties, BlobTag, BlobTags, BlobType,
    Block, BlockBlobClientCommitBlockListOptions, BlockBlobClientCommitBlockListResult,
    BlockBlobClientCommitBlockListResultHeaders, BlockBlobClientGetBlockListOptions,
    BlockBlobClientStageBlockFromUrlOptions, BlockBlobClientStageBlockFromUrlResult,
    BlockBlobClientStageBlockFromUrlResultHeaders, BlockBlobClientStageBlockOptions,
    BlockBlobClientStageBlockResult, BlockBlobClientStageBlockResultHeaders,
    BlockBlobClientUploadBlobFromUrlOptions, BlockBlobClientUploadBlobFromUrlResult,
    BlockBlobClientUploadBlobFromUrlResultHeaders, BlockBlobClientUploadOptions,
    BlockBlobClientUploadResult, BlockBlobClientUploadResultHeaders, BlockList, BlockListHeaders,
    BlockListType, BlockLookupList, ContainerItem, CopyStatus, CorsRule, DeleteSnapshotsOptionType,
    EncryptionAlgorithmType, FileShareTokenIntent, FilterBlobItem, FilterBlobSegment,
    FilterBlobsIncludeItem, GeoReplication, GeoReplicationStatusType, ImmutabilityPolicyMode,
    LeaseDuration, LeaseState, LeaseStatus, ListBlobsFlatSegmentResponse, ListBlobsIncludeItem,
    ListContainersIncludeType, ListContainersSegmentResponse, Logging, Metrics,
    ObjectReplicationMetadata, PageBlobClientClearPagesOptions, PageBlobClientClearPagesResult,
    PageBlobClientClearPagesResultHeaders, PageBlobClientCreateOptions, PageBlobClientCreateResult,
    PageBlobClientCreateResultHeaders, PageBlobClientGetPageRangesOptions,
    PageBlobClientResizeOptions, PageBlobClientResizeResult, PageBlobClientResizeResultHeaders,
    PageBlobClientSetSequenceNumberOptions, PageBlobClientSetSequenceNumberResult,
    PageBlobClientSetSequenceNumberResultHeaders, PageBlobClientUploadPagesFromUrlOptions,
    PageBlobClientUploadPagesFromUrlResult, PageBlobClientUploadPagesFromUrlResultHeaders,
    PageBlobClientUploadPagesOptions, PageBlobClientUploadPagesResult,
    PageBlobClientUploadPagesResultHeaders, PageList, PageListHeaders, PremiumPageBlobAccessTier,
    PublicAccessType, RehydratePriority, RetentionPolicy, SequenceNumberActionType,
    SignedIdentifier, SignedIdentifiers, SignedIdentifiersHeaders, SkuName, StaticWebsite,
    StorageErrorCode, StorageServiceStats,
};

pub use error::{Result, StorageError};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_bounded_range() {
        let range = HttpRange::new(0, 512);
        assert_eq!(range.offset, 0);
        assert_eq!(range.length, Some(512));
    }

    #[test]
    fn from_offset_creates_open_ended_range() {
        let range = HttpRange::from_offset(255);
        assert_eq!(range.offset, 255);
        assert_eq!(range.length, None);
    }

    #[test]
    fn display_bounded_range() {
        let range = HttpRange::new(0, 512);
        assert_eq!(range.to_string(), "bytes=0-511");
    }

    #[test]
    fn display_open_ended_range() {
        let range = HttpRange::from_offset(255);
        assert_eq!(range.to_string(), "bytes=255-");
    }

    #[test]
    fn into_string() {
        let range = HttpRange::new(100, 101);
        let s: String = range.into();
        assert_eq!(s, "bytes=100-200");
    }

    #[test]
    fn into_header_value() {
        let range = HttpRange::new(0, 512);
        let header_value: HeaderValue = range.into();
        assert_eq!(header_value.as_str(), "bytes=0-511");
    }
}
