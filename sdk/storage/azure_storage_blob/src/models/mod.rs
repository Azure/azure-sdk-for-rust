// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod extensions;

pub use crate::generated::models::{
    AccessTier, AccountKind, AppendBlobClientAppendBlockFromUrlOptions,
    AppendBlobClientAppendBlockFromUrlResult, AppendBlobClientAppendBlockFromUrlResultHeaders,
    AppendBlobClientAppendBlockOptions, AppendBlobClientAppendBlockResult,
    AppendBlobClientAppendBlockResultHeaders, AppendBlobClientCreateOptions,
    AppendBlobClientCreateResult, AppendBlobClientCreateResultHeaders, AppendBlobClientSealOptions,
    AppendBlobClientSealResult, AppendBlobClientSealResultHeaders, ArchiveStatus,
    BlobClientAbortCopyFromUrlResult, BlobClientAbortCopyFromUrlResultHeaders,
    BlobClientAcquireLeaseOptions, BlobClientAcquireLeaseResult,
    BlobClientAcquireLeaseResultHeaders, BlobClientBreakLeaseOptions, BlobClientBreakLeaseResult,
    BlobClientBreakLeaseResultHeaders, BlobClientChangeLeaseOptions, BlobClientChangeLeaseResult,
    BlobClientChangeLeaseResultHeaders, BlobClientCopyFromUrlResult,
    BlobClientCopyFromUrlResultHeaders, BlobClientCreateSnapshotResult,
    BlobClientCreateSnapshotResultHeaders, BlobClientDeleteImmutabilityPolicyResult,
    BlobClientDeleteImmutabilityPolicyResultHeaders, BlobClientDeleteOptions,
    BlobClientDownloadOptions, BlobClientDownloadResult, BlobClientDownloadResultHeaders,
    BlobClientGetAccountInfoOptions, BlobClientGetAccountInfoResult,
    BlobClientGetAccountInfoResultHeaders, BlobClientGetPropertiesOptions,
    BlobClientGetPropertiesResult, BlobClientGetPropertiesResultHeaders, BlobClientGetTagsOptions,
    BlobClientReleaseLeaseOptions, BlobClientReleaseLeaseResult,
    BlobClientReleaseLeaseResultHeaders, BlobClientRenewLeaseOptions, BlobClientRenewLeaseResult,
    BlobClientRenewLeaseResultHeaders, BlobClientSetExpiryResult, BlobClientSetExpiryResultHeaders,
    BlobClientSetImmutabilityPolicyResult, BlobClientSetImmutabilityPolicyResultHeaders,
    BlobClientSetLegalHoldResult, BlobClientSetLegalHoldResultHeaders,
    BlobClientSetMetadataOptions, BlobClientSetPropertiesOptions, BlobClientSetTagsOptions,
    BlobClientSetTierOptions, BlobClientStartCopyFromUrlResult,
    BlobClientStartCopyFromUrlResultHeaders, BlobClientUndeleteResult,
    BlobClientUndeleteResultHeaders, BlobContainerClientAcquireLeaseOptions,
    BlobContainerClientAcquireLeaseResult, BlobContainerClientAcquireLeaseResultHeaders,
    BlobContainerClientBreakLeaseOptions, BlobContainerClientBreakLeaseResult,
    BlobContainerClientBreakLeaseResultHeaders, BlobContainerClientChangeLeaseOptions,
    BlobContainerClientChangeLeaseResult, BlobContainerClientChangeLeaseResultHeaders,
    BlobContainerClientCreateOptions, BlobContainerClientDeleteOptions,
    BlobContainerClientFindBlobsByTagsOptions, BlobContainerClientGetAccountInfoOptions,
    BlobContainerClientGetAccountInfoResult, BlobContainerClientGetAccountInfoResultHeaders,
    BlobContainerClientGetPropertiesOptions, BlobContainerClientGetPropertiesResult,
    BlobContainerClientGetPropertiesResultHeaders, BlobContainerClientListBlobFlatSegmentOptions,
    BlobContainerClientReleaseLeaseOptions, BlobContainerClientReleaseLeaseResult,
    BlobContainerClientReleaseLeaseResultHeaders, BlobContainerClientRenameResult,
    BlobContainerClientRenameResultHeaders, BlobContainerClientRenewLeaseOptions,
    BlobContainerClientRenewLeaseResult, BlobContainerClientRenewLeaseResultHeaders,
    BlobContainerClientRestoreResult, BlobContainerClientRestoreResultHeaders,
    BlobContainerClientSetAccessPolicyResult, BlobContainerClientSetAccessPolicyResultHeaders,
    BlobContainerClientSetMetadataOptions, BlobCopySourceTags, BlobDeleteType, BlobFlatListSegment,
    BlobImmutabilityPolicyMode, BlobItemInternal, BlobMetadata, BlobName, BlobPropertiesInternal,
    BlobServiceClientFindBlobsByTagsOptions, BlobServiceClientGetAccountInfoOptions,
    BlobServiceClientGetAccountInfoResult, BlobServiceClientGetAccountInfoResultHeaders,
    BlobServiceClientGetPropertiesOptions, BlobServiceClientListContainersSegmentOptions,
    BlobServiceClientSetPropertiesOptions, BlobServiceProperties, BlobTag, BlobTags,
    BlobTagsHeaders, BlobType, Block, BlockBlobClientCommitBlockListOptions,
    BlockBlobClientCommitBlockListResult, BlockBlobClientCommitBlockListResultHeaders,
    BlockBlobClientGetBlockListOptions, BlockBlobClientQueryResult,
    BlockBlobClientQueryResultHeaders, BlockBlobClientStageBlockFromUrlResult,
    BlockBlobClientStageBlockFromUrlResultHeaders, BlockBlobClientStageBlockOptions,
    BlockBlobClientStageBlockResult, BlockBlobClientStageBlockResultHeaders,
    BlockBlobClientUploadBlobFromUrlOptions, BlockBlobClientUploadBlobFromUrlResult,
    BlockBlobClientUploadBlobFromUrlResultHeaders, BlockBlobClientUploadOptions,
    BlockBlobClientUploadResult, BlockBlobClientUploadResultHeaders, BlockList, BlockListHeaders,
    BlockListType, BlockLookupList, ContainerItem, CopyStatus, CorsRule, DeleteSnapshotsOptionType,
    EncryptionAlgorithmType, FileShareTokenIntent, FilterBlobItem, FilterBlobSegment,
    ImmutabilityPolicyMode, LeaseDuration, LeaseState, LeaseStatus, ListBlobsFlatSegmentResponse,
    ListBlobsFlatSegmentResponseHeaders, ListBlobsHierarchySegmentResponse,
    ListBlobsHierarchySegmentResponseHeaders, ListBlobsIncludeItem, ListContainersIncludeType,
    ListContainersSegmentResponse, Logging, Metrics, ObjectReplicationMetadata,
    PageBlobClientClearPagesOptions, PageBlobClientClearPagesResult,
    PageBlobClientClearPagesResultHeaders, PageBlobClientCopyIncrementalResult,
    PageBlobClientCopyIncrementalResultHeaders, PageBlobClientCreateOptions,
    PageBlobClientCreateResult, PageBlobClientCreateResultHeaders,
    PageBlobClientGetPageRangesOptions, PageBlobClientResizeOptions, PageBlobClientResizeResult,
    PageBlobClientResizeResultHeaders, PageBlobClientSetSequenceNumberOptions,
    PageBlobClientSetSequenceNumberResult, PageBlobClientSetSequenceNumberResultHeaders,
    PageBlobClientUploadPagesFromUrlOptions, PageBlobClientUploadPagesFromUrlResult,
    PageBlobClientUploadPagesFromUrlResultHeaders, PageBlobClientUploadPagesOptions,
    PageBlobClientUploadPagesResult, PageBlobClientUploadPagesResultHeaders, PageList,
    PageListHeaders, PremiumPageBlobAccessTier, PublicAccessType, RehydratePriority,
    RetentionPolicy, SequenceNumberActionType, SignedIdentifier, StaticWebsite, StorageErrorCode,
    StorageServiceStats, StorageServiceStatsHeaders, UserDelegationKey, UserDelegationKeyHeaders,
    VecSignedIdentifierHeaders,
};
pub use extensions::*;

use azure_core::error::ErrorKind;
use azure_core::http::{headers::Headers, StatusCode};
use azure_core::Error;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct StorageError {
    /// The HTTP status code.
    pub status_code: StatusCode,
    /// The Storage error code.
    pub error_code: StorageErrorCode,
    /// The error message.
    pub message: String,
    /// The headers from the response.
    pub headers: Headers,
    /// Additional fields from the error response that weren't explicitly mapped.
    pub additional_error_info: HashMap<String, Value>,
}

impl StorageError {
    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    pub fn error_code(&self) -> StorageErrorCode {
        self.error_code.clone()
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    /// Returns any additional error information fields returned by the Service.
    pub fn additional_error_info(&self) -> &HashMap<String, Value> {
        &self.additional_error_info
    }
}
