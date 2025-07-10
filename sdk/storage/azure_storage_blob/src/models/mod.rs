// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod extensions;

pub use crate::generated::models::{
    AccessTierOptional, AppendBlobClientAppendBlockFromUrlOptions,
    AppendBlobClientAppendBlockFromUrlResult, AppendBlobClientAppendBlockOptions,
    AppendBlobClientAppendBlockResult, AppendBlobClientCreateOptions, AppendBlobClientCreateResult,
    AppendBlobClientSealOptions, AppendBlobClientSealResult, ArchiveStatus,
    BlobClientAcquireLeaseOptions, BlobClientAcquireLeaseResultHeaders,
    BlobClientBreakLeaseOptions, BlobClientChangeLeaseOptions, BlobClientChangeLeaseResultHeaders,
    BlobClientDeleteOptions, BlobClientDownloadOptions, BlobClientDownloadResult,
    BlobClientDownloadResultHeaders, BlobClientGetPropertiesOptions, BlobClientGetPropertiesResult,
    BlobClientGetPropertiesResultHeaders, BlobClientReleaseLeaseOptions,
    BlobClientRenewLeaseOptions, BlobClientSetMetadataOptions, BlobClientSetPropertiesOptions,
    BlobClientSetTierOptions, BlobContainerClientAcquireLeaseOptions,
    BlobContainerClientAcquireLeaseResultHeaders, BlobContainerClientBreakLeaseOptions,
    BlobContainerClientChangeLeaseOptions, BlobContainerClientChangeLeaseResultHeaders,
    BlobContainerClientCreateOptions, BlobContainerClientDeleteOptions,
    BlobContainerClientGetPropertiesOptions, BlobContainerClientGetPropertiesResult,
    BlobContainerClientGetPropertiesResultHeaders, BlobContainerClientListBlobFlatSegmentOptions,
    BlobContainerClientReleaseLeaseOptions, BlobContainerClientRenewLeaseOptions,
    BlobContainerClientSetMetadataOptions, BlobImmutabilityPolicyMode,
    BlobServiceClientGetPropertiesOptions, BlobServiceClientListContainersSegmentOptions, BlobType,
    BlockBlobClientCommitBlockListOptions, BlockBlobClientCommitBlockListResult,
    BlockBlobClientGetBlockListOptions, BlockBlobClientStageBlockOptions,
    BlockBlobClientStageBlockResult, BlockBlobClientUploadOptions, BlockBlobClientUploadResult,
    BlockList, BlockListType, BlockLookupList, CopyStatus, LeaseState, LeaseStatus,
    ListBlobsFlatSegmentResponse, ListContainersSegmentResponse, PageBlobClientClearPagesOptions,
    PageBlobClientClearPagesResult, PageBlobClientCreateOptions, PageBlobClientCreateResult,
    PageBlobClientResizeOptions, PageBlobClientResizeResult, PageBlobClientUploadPagesOptions,
    PageBlobClientUploadPagesResult, PublicAccessType, RehydratePriority, StorageServiceProperties,
};
pub use extensions::*;
