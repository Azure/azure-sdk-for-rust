// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub(crate) mod content_range;
mod extensions;

use azure_core::fmt::SafeDebug;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, fmt};

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
    BlobClientChangeLeaseResultHeaders, BlobClientCopyFromUrlResult,
    BlobClientCopyFromUrlResultHeaders, BlobClientCreateSnapshotOptions,
    BlobClientCreateSnapshotResult, BlobClientCreateSnapshotResultHeaders,
    BlobClientDeleteImmutabilityPolicyOptions, BlobClientDeleteOptions, BlobClientDownloadOptions,
    BlobClientDownloadResult, BlobClientDownloadResultHeaders, BlobClientGetAccountInfoOptions,
    BlobClientGetAccountInfoResult, BlobClientGetAccountInfoResultHeaders,
    BlobClientGetPropertiesOptions, BlobClientGetPropertiesResult,
    BlobClientGetPropertiesResultHeaders, BlobClientGetTagsOptions, BlobClientReleaseLeaseOptions,
    BlobClientReleaseLeaseResult, BlobClientReleaseLeaseResultHeaders, BlobClientRenewLeaseOptions,
    BlobClientRenewLeaseResult, BlobClientRenewLeaseResultHeaders, BlobClientSetExpiryResult,
    BlobClientSetExpiryResultHeaders, BlobClientSetImmutabilityPolicyOptions,
    BlobClientSetLegalHoldOptions, BlobClientSetMetadataOptions, BlobClientSetPropertiesOptions,
    BlobClientSetTagsOptions, BlobClientSetTierOptions, BlobClientStartCopyFromUrlResult,
    BlobClientStartCopyFromUrlResultHeaders, BlobClientUndeleteOptions,
    BlobContainerClientAcquireLeaseOptions, BlobContainerClientAcquireLeaseResult,
    BlobContainerClientAcquireLeaseResultHeaders, BlobContainerClientBreakLeaseOptions,
    BlobContainerClientBreakLeaseResult, BlobContainerClientBreakLeaseResultHeaders,
    BlobContainerClientChangeLeaseOptions, BlobContainerClientChangeLeaseResult,
    BlobContainerClientChangeLeaseResultHeaders, BlobContainerClientCreateOptions,
    BlobContainerClientDeleteOptions, BlobContainerClientFindBlobsByTagsOptions,
    BlobContainerClientGetAccessPolicyOptions, BlobContainerClientGetAccountInfoResult,
    BlobContainerClientGetAccountInfoResultHeaders, BlobContainerClientGetPropertiesOptions,
    BlobContainerClientGetPropertiesResult, BlobContainerClientGetPropertiesResultHeaders,
    BlobContainerClientListBlobFlatSegmentOptions, BlobContainerClientReleaseLeaseOptions,
    BlobContainerClientReleaseLeaseResult, BlobContainerClientReleaseLeaseResultHeaders,
    BlobContainerClientRenewLeaseOptions, BlobContainerClientRenewLeaseResult,
    BlobContainerClientRenewLeaseResultHeaders, BlobContainerClientSetAccessPolicyOptions,
    BlobContainerClientSetMetadataOptions, BlobCopySourceTags, BlobDeleteType, BlobExpiryOptions,
    BlobFlatListSegment, BlobItemInternal, BlobMetadata, BlobPropertiesInternal,
    BlobServiceClientFindBlobsByTagsOptions, BlobServiceClientGetAccountInfoOptions,
    BlobServiceClientGetAccountInfoResult, BlobServiceClientGetAccountInfoResultHeaders,
    BlobServiceClientGetPropertiesOptions, BlobServiceClientGetStatisticsOptions,
    BlobServiceClientListContainersSegmentOptions, BlobServiceClientSetPropertiesOptions,
    BlobServiceProperties, BlobTag, BlobTags, BlobType, Block,
    BlockBlobClientCommitBlockListOptions, BlockBlobClientCommitBlockListResult,
    BlockBlobClientCommitBlockListResultHeaders, BlockBlobClientGetBlockListOptions,
    BlockBlobClientQueryResult, BlockBlobClientQueryResultHeaders,
    BlockBlobClientStageBlockFromUrlResult, BlockBlobClientStageBlockFromUrlResultHeaders,
    BlockBlobClientStageBlockOptions, BlockBlobClientStageBlockResult,
    BlockBlobClientStageBlockResultHeaders, BlockBlobClientUploadBlobFromUrlOptions,
    BlockBlobClientUploadBlobFromUrlResult, BlockBlobClientUploadBlobFromUrlResultHeaders,
    BlockBlobClientUploadOptions, BlockBlobClientUploadResult, BlockBlobClientUploadResultHeaders,
    BlockList, BlockListHeaders, BlockListType, BlockLookupList, ContainerItem, CopyStatus,
    CorsRule, DeleteSnapshotsOptionType, EncryptionAlgorithmType, FileShareTokenIntent,
    FilterBlobItem, FilterBlobSegment, FilterBlobsIncludeItem, GeoReplication,
    GeoReplicationStatusType, ImmutabilityPolicyMode, LeaseDuration, LeaseState, LeaseStatus,
    ListBlobsFlatSegmentResponse, ListBlobsHierarchySegmentResponse, ListBlobsIncludeItem,
    ListContainersIncludeType, ListContainersSegmentResponse, Logging, Metrics,
    ObjectReplicationMetadata, PageBlobClientClearPagesOptions, PageBlobClientClearPagesResult,
    PageBlobClientClearPagesResultHeaders, PageBlobClientCopyIncrementalResult,
    PageBlobClientCopyIncrementalResultHeaders, PageBlobClientCreateOptions,
    PageBlobClientCreateResult, PageBlobClientCreateResultHeaders,
    PageBlobClientGetPageRangesOptions, PageBlobClientResizeOptions, PageBlobClientResizeResult,
    PageBlobClientResizeResultHeaders, PageBlobClientSetSequenceNumberOptions,
    PageBlobClientSetSequenceNumberResult, PageBlobClientSetSequenceNumberResultHeaders,
    PageBlobClientUploadPagesFromUrlOptions, PageBlobClientUploadPagesFromUrlResult,
    PageBlobClientUploadPagesFromUrlResultHeaders, PageBlobClientUploadPagesOptions,
    PageBlobClientUploadPagesResult, PageBlobClientUploadPagesResultHeaders, PageList,
    PageListHeaders, PremiumPageBlobAccessTier, PublicAccessType, QueryRequestType, QueryType,
    RehydratePriority, RetentionPolicy, SequenceNumberActionType, SignedIdentifier,
    SignedIdentifiers, SignedIdentifiersHeaders, SkuName, StaticWebsite, StorageErrorCode,
    StorageServiceStats, UserDelegationKey,
};

/// Represents a blob name.
///
/// Because [`BlobName::content`] may be encoded, you should use [`BlobName::as_str()`]
/// or other conversion functions to get the raw decoded bytes or attempt to get a [`Cow<'_, str>`] or owned `String`.
#[derive(Clone, Default, Deserialize, SafeDebug, Serialize)]
pub struct BlobName {
    /// The blob name.
    #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Whether the blob name is encoded.
    #[serde(rename = "@Encoded", skip_serializing_if = "Option::is_none")]
    pub encoded: Option<bool>,
}

impl BlobName {
    /// Attempts to convert the [`BlobName`] into a [`Cow`] of `&str`.
    ///
    /// If `BlobName` contains certain characters invalid in XML and you want the raw bytes,
    /// you can call `into()` to get a [`Cow<'_, [u8]>`](Cow) or an owned `[u8]`.
    pub fn as_str(&self) -> std::result::Result<Cow<'_, str>, std::str::Utf8Error> {
        use percent_encoding::percent_decode_str;

        let Some(content) = self.content.as_deref() else {
            return Ok(Cow::Borrowed(""));
        };

        if self.encoded.unwrap_or_default() {
            return percent_decode_str(content).decode_utf8();
        }

        Ok(Cow::Borrowed(content))
    }
}

impl fmt::Display for BlobName {
    /// Format a [`BlobName`] for printing.
    ///
    /// This is a lossy conversion using [`String::from_utf8_lossy`].
    /// If `BlobName` contains certain characters invalid in XML and you want the raw bytes,
    /// you can call `into()` to get a [`Cow<'_, [u8]>`](Cow) or an owned `[u8]`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content: Cow<'_, [u8]> = self.into();
        f.write_str(&String::from_utf8_lossy(&content))
    }
}

impl<'a> From<&'a BlobName> for Cow<'a, [u8]> {
    /// Get the raw decoded bytes from a [`BlobName`].
    ///
    /// If `BlobName` contains certain characters invalid in XML and you want the raw bytes,
    /// you can call `into()` to get a [`Cow<'_, [u8]>`](Cow) or an owned `[u8]`.
    fn from(value: &'a BlobName) -> Self {
        use percent_encoding::percent_decode_str;

        let Some(content) = value.content.as_deref() else {
            return Cow::Borrowed(&[]);
        };

        if value.encoded.unwrap_or_default() {
            return percent_decode_str(content).into();
        }

        Cow::Borrowed(content.as_bytes())
    }
}

impl From<BlobName> for Vec<u8> {
    /// Get the raw decoded bytes from a [`BlobName`].
    ///
    /// If `BlobName` contains certain characters invalid in XML and you want the raw bytes,
    /// you can call `into()` to get a [`Cow<'_, [u8]>`](Cow) or an owned `[u8]`.
    fn from(value: BlobName) -> Self {
        let cow: Cow<'_, [u8]> = (&value).into();
        cow.into_owned()
    }
}

impl<'a> TryFrom<&'a BlobName> for Cow<'a, str> {
    type Error = std::str::Utf8Error;

    /// Attempts to convert the [`BlobName`] into a [`Cow`] of `&str`.
    ///
    /// If `BlobName` contains certain characters invalid in XML and you want the raw bytes,
    /// you can call `into()` to get a [`Cow<'_, [u8]>`](Cow) or an owned `[u8]`.
    fn try_from(value: &'a BlobName) -> Result<Self, Self::Error> {
        value.as_str()
    }
}

impl TryFrom<BlobName> for String {
    type Error = std::str::Utf8Error;

    /// Attempts to convert the [`BlobName`] into a `String`.
    ///
    /// If `BlobName` contains certain characters invalid in XML and you want the raw bytes,
    /// you can call `into()` to get a [`Cow<'_, [u8]>`](Cow) or an owned `[u8]`.
    fn try_from(value: BlobName) -> Result<Self, Self::Error> {
        Ok(value.as_str()?.to_string())
    }
}

impl TryFrom<&BlobName> for String {
    type Error = std::str::Utf8Error;

    /// Attempts to convert the [`BlobName`] into a `String`.
    ///
    /// If `BlobName` contains certain characters invalid in XML and you want the raw bytes,
    /// you can call `into()` to get a [`Cow<'_, [u8]>`](Cow) or an owned `[u8]`.
    fn try_from(value: &BlobName) -> Result<Self, Self::Error> {
        Ok(value.as_str()?.to_string())
    }
}

#[cfg(test)]
mod tests {
    // cspell:ignore fffe ffff
    use super::*;

    #[test]
    fn as_str_simple_ascii() {
        let blob_name = BlobName {
            content: Some("foo".to_string()),
            encoded: Some(false),
        };
        let result = blob_name.as_str().unwrap();
        assert_eq!(result, "foo");
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn as_str_utf8_extended() {
        let blob_name = BlobName {
            content: Some("año".to_string()),
            encoded: Some(false),
        };
        let result = blob_name.as_str().unwrap();
        assert_eq!(result, "año");
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn as_str_percent_encoded_ascii() {
        let blob_name = BlobName {
            content: Some("foo".to_string()),
            encoded: Some(true),
        };
        let result = blob_name.as_str().unwrap();
        assert_eq!(result, "foo");
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn as_str_percent_encoded_utf8() {
        let blob_name = BlobName {
            content: Some("a%C3%B1o".to_string()),
            encoded: Some(true),
        };
        let result = blob_name.as_str().unwrap();
        assert_eq!(result, "año");
        assert!(matches!(result, Cow::Owned(_)));
    }

    #[test]
    fn as_str_invalid_utf8_fffe() {
        let blob_name = BlobName {
            content: Some("%FF%FE".to_string()),
            encoded: Some(true),
        };
        let result = blob_name.as_str();
        assert!(result.is_err());
    }

    #[test]
    fn as_str_invalid_utf8_ffff() {
        let blob_name = BlobName {
            content: Some("%FF%FF".to_string()),
            encoded: Some(true),
        };
        let result = blob_name.as_str();
        assert!(result.is_err());
    }

    #[test]
    fn from_blob_name_to_bytes_simple_ascii() {
        let blob_name = BlobName {
            content: Some("foo".to_string()),
            encoded: Some(false),
        };
        let result: Cow<'_, [u8]> = (&blob_name).into();
        assert_eq!(result.as_ref(), b"foo");
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn from_blob_name_to_bytes_utf8_extended() {
        let blob_name = BlobName {
            content: Some("año".to_string()),
            encoded: Some(false),
        };
        let result: Cow<'_, [u8]> = (&blob_name).into();
        assert_eq!(result.as_ref(), "año".as_bytes());
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn from_blob_name_to_bytes_percent_encoded_ascii() {
        let blob_name = BlobName {
            content: Some("foo".to_string()),
            encoded: Some(true),
        };
        let result: Cow<'_, [u8]> = (&blob_name).into();
        assert_eq!(result.as_ref(), b"foo");
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn from_blob_name_to_bytes_percent_encoded_utf8() {
        let blob_name = BlobName {
            content: Some("a%C3%B1o".to_string()),
            encoded: Some(true),
        };
        let result: Cow<'_, [u8]> = (&blob_name).into();
        assert_eq!(result.as_ref(), "año".as_bytes());
        assert!(matches!(result, Cow::Owned(_)));
    }

    #[test]
    fn from_blob_name_to_bytes_invalid_utf8_fffe() {
        let blob_name = BlobName {
            content: Some("%FF%FE".to_string()),
            encoded: Some(true),
        };
        let result: Cow<'_, [u8]> = (&blob_name).into();
        assert_eq!(result.as_ref(), &[0xFF, 0xFE]);
        assert!(matches!(result, Cow::Owned(_)));
    }

    #[test]
    fn from_blob_name_to_bytes_invalid_utf8_ffff() {
        let blob_name = BlobName {
            content: Some("%FF%FF".to_string()),
            encoded: Some(true),
        };
        let result: Cow<'_, [u8]> = (&blob_name).into();
        assert_eq!(result.as_ref(), &[0xFF, 0xFF]);
        assert!(matches!(result, Cow::Owned(_)));
    }

    #[test]
    fn try_from_blob_name_to_cow_str_simple_ascii() {
        let blob_name = BlobName {
            content: Some("foo".to_string()),
            encoded: Some(false),
        };
        let result: Cow<'_, str> = (&blob_name).try_into().unwrap();
        assert_eq!(result, "foo");
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn try_from_blob_name_to_cow_str_utf8_extended() {
        let blob_name = BlobName {
            content: Some("año".to_string()),
            encoded: Some(false),
        };
        let result: Cow<'_, str> = (&blob_name).try_into().unwrap();
        assert_eq!(result, "año");
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn try_from_blob_name_to_cow_str_percent_encoded_ascii() {
        let blob_name = BlobName {
            content: Some("foo".to_string()),
            encoded: Some(true),
        };
        let result: Cow<'_, str> = (&blob_name).try_into().unwrap();
        assert_eq!(result, "foo");
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn try_from_blob_name_to_cow_str_percent_encoded_utf8() {
        let blob_name = BlobName {
            content: Some("a%C3%B1o".to_string()),
            encoded: Some(true),
        };
        let result: Cow<'_, str> = (&blob_name).try_into().unwrap();
        assert_eq!(result, "año");
        assert!(matches!(result, Cow::Owned(_)));
    }

    #[test]
    fn try_from_blob_name_to_cow_str_invalid_utf8_fffe() {
        let blob_name = BlobName {
            content: Some("%FF%FE".to_string()),
            encoded: Some(true),
        };
        let result: Result<Cow<'_, str>, _> = (&blob_name).try_into();
        assert!(result.is_err());
    }

    #[test]
    fn try_from_blob_name_to_cow_str_invalid_utf8_ffff() {
        let blob_name = BlobName {
            content: Some("%FF%FF".to_string()),
            encoded: Some(true),
        };
        let result: Result<Cow<'_, str>, _> = (&blob_name).try_into();
        assert!(result.is_err());
    }
}
