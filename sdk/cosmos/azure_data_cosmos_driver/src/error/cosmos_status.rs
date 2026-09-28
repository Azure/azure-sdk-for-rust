// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Combined HTTP status and Cosmos DB sub-status code.
//!
//! Cosmos DB uses sub-status codes (returned in the `x-ms-substatus` header) to provide
//! additional context for HTTP status codes. Some sub-status code integers have different
//! meanings depending on the HTTP status code. For example, sub-status `1002` means:
//!
//! - `ReadSessionNotAvailable` when paired with HTTP 404 (Not Found)
//! - `PartitionKeyRangeGone` when paired with HTTP 410 (Gone)
//!
//! [`CosmosStatus`] encapsulates both values together so the correct interpretation is
//! always available.

use azure_core::http::StatusCode;
use serde::{Deserialize, Serialize};
use std::fmt;

// =========================================================================
// SubStatusCode
// =========================================================================
//
// Sub-status codes are derived from:
// - .NET SDK: `Microsoft.Azure.Documents.SubStatusCodes` enum
// - Java SDK: `com.azure.cosmos.implementation.HttpConstants.SubStatusCodes`
//
// Some sub-status codes have duplicate numeric values with different meanings
// depending on the HTTP status code context (e.g., 1002 means `ReadSessionNotAvailable`
// for 404 but `PartitionKeyRangeGone` for 410).

/// A newtype wrapper for Cosmos DB sub-status codes.
///
/// Sub-status codes provide additional context for HTTP error responses from Cosmos DB.
/// They are returned in the `x-ms-substatus` header and help distinguish between
/// different error conditions that share the same HTTP status code.
///
/// # Important Note on Duplicate Values
///
/// Some numeric sub-status codes have different meanings depending on the HTTP status code.
/// For example, `1002` means:
/// - `ReadSessionNotAvailable` when paired with HTTP 404
/// - `PartitionKeyRangeGone` when paired with HTTP 410
///
/// Always interpret sub-status codes in the context of their HTTP status code.
/// Use [`CosmosStatus::name()`] for automatic disambiguation based on the paired
/// HTTP status code.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SubStatusCode(u16);

impl SubStatusCode {
    /// Creates a new `SubStatusCode` from a numeric value.
    pub const fn new(code: u16) -> Self {
        Self(code)
    }

    /// Returns the numeric value of the sub-status code.
    pub const fn value(&self) -> u16 {
        self.0
    }

    /// Creates a `SubStatusCode` from a header string.
    ///
    /// Returns `None` if parsing fails.
    pub fn from_header_value(s: &str) -> Option<Self> {
        s.trim().parse::<u16>().ok().map(SubStatusCode)
    }

    /// Returns the name of this sub-status code, if known.
    ///
    /// Some sub-status codes have different meanings depending on the HTTP status code.
    /// When `status_code` is provided, the method uses the (status, sub-status) tuple to
    /// determine the correct name. When `status_code` is `None` and the sub-status code
    /// has multiple meanings, returns `None` to avoid ambiguity.
    ///
    /// For unknown codes, returns `None`. Use `Display` for a string
    /// representation that always works.
    pub fn name(&self, status_code: Option<StatusCode>) -> Option<&'static str> {
        match self.0 {
            0 => Some("Unknown"),
            100 => Some("OperationInProgress"),

            // Codes with MULTIPLE meanings depending on HTTP status code
            // These return None if status_code is not provided

            // 1002: ReadSessionNotAvailable (404) / PartitionKeyRangeGone (410)
            1002 => match u16::from(status_code?) {
                404 => Some("ReadSessionNotAvailable"),
                410 => Some("PartitionKeyRangeGone"),
                _ => None,
            },

            // 1007: CompletingSplitOrMerge (410) / InsufficientBindablePartitions (503)
            1007 => match u16::from(status_code?) {
                410 => Some("CompletingSplitOrMerge"),
                503 => Some("InsufficientBindablePartitions"),
                _ => None,
            },

            // 1008: DatabaseAccountNotFound (403) / CompletingPartitionMigration (410)
            1008 => match u16::from(status_code?) {
                403 => Some("DatabaseAccountNotFound"),
                410 => Some("CompletingPartitionMigration"),
                _ => None,
            },

            // 1012: RedundantDatabasePut (403) / ComputeFederationNotFound (503)
            1012 => match u16::from(status_code?) {
                403 => Some("RedundantDatabasePut"),
                503 => Some("ComputeFederationNotFound"),
                _ => None,
            },

            // 2001: MissedTargetLsn (204) / SplitIsDisabled (412)
            2001 => match u16::from(status_code?) {
                204 => Some("MissedTargetLsn"),
                412 => Some("SplitIsDisabled"),
                _ => None,
            },

            // 2002: MissedTargetLsnOver100 (204) / CollectionsInPartitionGotUpdated (412)
            2002 => match u16::from(status_code?) {
                204 => Some("MissedTargetLsnOver100"),
                412 => Some("CollectionsInPartitionGotUpdated"),
                _ => None,
            },

            // 2003: MissedTargetLsnOver1000 (204) / CannotAcquirePkrangesLock (412)
            2003 => match u16::from(status_code?) {
                204 => Some("MissedTargetLsnOver1000"),
                412 => Some("CannotAcquirePkrangesLock"),
                _ => None,
            },

            // 2004: MissedTargetLsnOver10000 (204) / ResourceNotFound (412)
            2004 => match u16::from(status_code?) {
                204 => Some("MissedTargetLsnOver10000"),
                412 => Some("ResourceNotFound"),
                _ => None,
            },

            // 2011: MissedTargetGlobalCommittedLsn (204) / StorageSplitConflictingWithNwayThroughputSplit (412)
            2011 => match u16::from(status_code?) {
                204 => Some("MissedTargetGlobalCommittedLsn"),
                412 => Some("StorageSplitConflictingWithNwayThroughputSplit"),
                _ => None,
            },

            // 2012: MissedTargetGlobalCommittedLsnOver100 (204) / MergeIsDisabled (412)
            2012 => match u16::from(status_code?) {
                204 => Some("MissedTargetGlobalCommittedLsnOver100"),
                412 => Some("MergeIsDisabled"),
                _ => None,
            },

            // 3207: ConfigurationNameAlreadyExists (409) / PrepareTimeLimitExceeded (429)
            3207 => match u16::from(status_code?) {
                409 => Some("ConfigurationNameAlreadyExists"),
                429 => Some("PrepareTimeLimitExceeded"),
                _ => None,
            },

            // 1013: PartitionKeyDefinitionNotSpecified (400) / CollectionCreateInProgress (404)
            1013 => match u16::from(status_code?) {
                400 => Some("PartitionKeyDefinitionNotSpecified"),
                404 => Some("CollectionCreateInProgress"),
                _ => None,
            },

            // 1024: CollectionRidMismatch (400) / ArchivalPartitionNotPresent (410)
            1024 => match u16::from(status_code?) {
                400 => Some("CollectionRidMismatch"),
                410 => Some("ArchivalPartitionNotPresent"),
                _ => None,
            },

            // 1031: SystemPartitionKeyNotAllowed (403) / PartitionMigratingCollectionDeleted (404)
            1031 => match u16::from(status_code?) {
                403 => Some("SystemPartitionKeyNotAllowed"),
                404 => Some("PartitionMigratingCollectionDeleted"),
                _ => None,
            },

            // 1034: ResourceSoftDeleted (403) / PartitionMigrationSourcePartitionDeletedInMaster (404)
            1034 => match u16::from(status_code?) {
                403 => Some("ResourceSoftDeleted"),
                404 => Some("PartitionMigrationSourcePartitionDeletedInMaster"),
                _ => None,
            },

            // 6001: AggregatedHealthStateError (503) / QueryWaitForSequentialProgress (other)
            6001 => match u16::from(status_code?) {
                503 => Some("AggregatedHealthStateError"),
                _ => Some("QueryWaitForSequentialProgress"),
            },

            // Codes with SINGLE meaning (no status code ambiguity)

            // 204: Head requests - LSN differences (unambiguous)
            2013 => Some("MissedTargetGlobalCommittedLsnOver1000"),
            2014 => Some("MissedTargetGlobalCommittedLsnOver10000"),

            // 400: Bad Request
            1001 => Some("PartitionKeyMismatch"),
            1004 => Some("CrossPartitionQueryNotServable"),
            1016 => Some("SchemaOwnerIdMismatch"),
            1017 => Some("SchemaHashOrIdMismatch"),
            1018 => Some("PartitionKeyDefinitionMissingForAutopilot"),
            0xFFFF => Some("ScriptCompileError"),
            3205 => Some("AnotherOfferReplaceOperationIsInProgress"),
            1101 => Some("HttpListenerException"),
            1102 => Some("TransactionAlreadyActive"),
            1103 => Some("InvalidTransactionId"),
            1104 => Some("CrossCollectionTransactionNotSupported"),
            1105 => Some("InvalidTopologyChangeRequest"),

            // 403: Forbidden
            3 => Some("WriteForbidden"),
            1005 => Some("ProvisionLimitReached"),
            1009 => Some("RedundantCollectionPut"),
            1010 => Some("SharedThroughputDatabaseQuotaExceeded"),
            1011 => Some("SharedThroughputOfferGrowNotNeeded"),
            1014 => Some("PartitionKeyQuotaOverLimit"),
            1015 => Some("OfferReplaceDisabledAutoScaleOffer"),
            1019 => Some("SharedThroughputDatabaseCollectionCountExceeded"),
            1020 => Some("SharedThroughputDatabaseCountExceeded"),
            1021 => Some("ComputeInternalError"),
            1026 => Some("ClientIdMismatch"),
            1027 => Some("UniqueIndexReIndexInProgress"),
            1028 => Some("ThroughputCapQuotaExceeded"),
            1029 => Some("InvalidThroughputCapValue"),
            1032 => Some("PartitionKeyDeleteRequestLimitExceeded"),
            1033 => Some("LeakedPartition"),
            1110 => Some("PatchConditionNotMet"),

            // 404: Not Found
            1003 => Some("OwnerResourceNotFound"),
            1023 => Some("StoreNotReady"),
            1030 => Some("AuthTokenNotFoundInCache"),
            1035 => Some("PartitionMigrationSharedThroughputDbPartitionNotFound"),
            1036 => Some("PartitionMigrationPartitionResourceNotFound"),
            1037 => Some("PartitionMigrationFailedToUpdateDns"),

            // 408: Request Timeout
            1900 => Some("RequestPreempted"),

            // 409: Conflict
            1006 => Some("ConflictWithControlPlane"),
            3206 => Some("DatabaseNameAlreadyExists"),
            3301 => Some("UniqueIndexConflict"),
            3302 => Some("PartitionKeyHashCollisionForId"),
            3303 => Some("AzureBackupVaultIncrementalBackupPaused"),
            3304 => Some("AzureBackupVaultIncrementalBackupRestoreDisabled"),
            3050 => Some("PartitionMigrationDocCountMismatchSourceTarget"),
            3051 => Some("PartitionMigrationDocCountMismatchTargetReplicas"),

            // 410: Gone
            1000 => Some("NameCacheIsStale"),
            1022 => Some("LeaseNotFound"),

            // 412: Precondition Failed
            2005 => Some("CannotAcquireOfferOwnerLock"),
            2007 => Some("CannotAcquirePkrangeLock"),
            2008 => Some("CannotAcquirePartitionLock"),
            2015 => Some("TombstoneRecordsNotFound"),
            2016 => Some("InvalidAccountStatus"),
            2017 => Some("OfferValidationFailed"),
            2018 => Some("CannotAcquireMasterPartitionAccessLock"),
            2019 => Some("CannotAcquireInAccountRestoreLock"),
            2020 => Some("CollectionStateChanged"),
            2021 => Some("OfferScaledUpByUser"),
            2101 => Some("CannotAcquireLogStoreLoadBalanceLock"),

            // 413: Request Entity Too Large
            3401 => Some("TransactionLimitExceeded"),
            3402 => Some("BatchResponseSizeExceeded"),

            // 429: Too Many Requests (SLA Violations 30xx)
            3073 => Some("BwTreeIORateLimiter"),
            3074 => Some("StalenessExceededBound"),
            3075 => Some("ReplicationQueueFull"),
            3076 => Some("BwTreeLogFullBackpressure"),
            3077 => Some("ConnectionRateLimiter"),
            3078 => Some("XPCompositeReplicator"),
            3079 => Some("Unexpected"),
            3080 => Some("AsyncReaderWriterLock"),
            3081 => Some("ServiceModule"),
            3082 => Some("ValueDoesNotMatchExpectedBound"),
            3083 => Some("SinkPartitionValueDoesNotMatchExpectedBound"),
            3084 => Some("StoredProcedureConcurrency"),
            3085 => Some("RntbdClientChannel"),
            3086 => Some("LogFlushQueueDepthBackpressure"),
            3087 => Some("CheckpointQueueDepthBackpressure"),
            3088 => Some("ThrottleDueToSplit"),
            3089 => Some("AEQueueFull"),
            3090 => Some("QuotaExceeded"),
            3091 => Some("CollectionQuotaExceeded"),
            3092 => Some("SystemResourceUnavailable"),
            3093 => Some("PartitionedResourceQuotaExceeded"),
            3094 => Some("ThrottleDueToResourceExhaustion"),
            3095 => Some("ThrottleDueToStagingIndexQueueFull"),
            3096 => Some("ThrottleDueToReplicationBackpressure"),
            3097 => Some("CollectionQuotaExceededAutopilot"),
            3098 => Some("LogStoreNoFreeSegments"),
            3099 => Some("ThrottledByBlobRead"),
            3100 => Some("OperationLogSizeTooBig"),
            3101 => Some("ArchivalPartitionPendingCatchup"),
            3102 => Some("ThrottleDueToTrafficRegulation"),
            3103 => Some("ThrottleDueToTransportBufferUsage"),
            // 429: Too Many Requests (Non-SLA Violations 32xx)
            3200 => Some("RUBudgetExceeded"),
            3201 => Some("GatewayThrottled"),
            3202 => Some("RUpmPartitionLimitExceeded"),
            3203 => Some("RUpmSharedBudgetExceeded"),
            3204 => Some("ThrottledOfferScaleDown"),
            3208 => Some("ClientTcpChannelFull"),
            3209 => Some("BWTermCountLimitExceeded"),
            3210 => Some("RUBudgetExceededForMaster"),
            3211 => Some("ThrottleDueToEncryptedRevokedStoreLogNotEmpty"),
            3212 => Some("ThroughputBucketLimitExhausted"),
            3213 => Some("TooManyThroughputBucketUpdates"),
            3214 => Some("HotPartitionKeyThrottled"),
            3300 => Some("MicrosoftFabricCUBudgetExceeded"),

            // 449: Retry With
            5350 => Some("RbacAadGroupUnavailable"),
            5351 => Some("AzureRbacAccessDecisionUnavailable"),
            5352 => Some("DtcCoordinatorRaceConflict"),

            // 500: Internal Server Error
            3001 => Some("ConfigurationNameNotEmpty"),
            3002 => Some("ConfigurationOperationCancelled"),
            3003 => Some("InvalidAccountConfiguration"),
            3004 => Some("FederationDoesNotExistOrIsLocked"),
            3010 => Some("PartitionFailoverErrorCode"),
            3021 => Some("OperationManagerDequeuePumpStopped"),
            3042 => Some("OperationCancelledWithNoRollback"),
            3043 => Some("SplitTimedOut"),
            5360 => Some("RbacDisabledDueToArmPath"),
            5411 => Some("DtcLedgerFailure"),
            5412 => Some("DtcAccountConfigFailure"),
            5413 => Some("DtcDispatchFailure"),
            5415 => Some("DtcOperationRolledBack"),

            // 503: Service Unavailable
            1337 => Some("GoneException"),
            1338 => Some("QuorumNotMet"),
            1339 => Some("TooManyTentativeWritesToSatelliteRegion"),
            9001 => Some("OperationPaused"),
            9002 => Some("ServiceIsOffline"),
            9003 => Some("InsufficientCapacity"),

            // Federation/Health errors (6xxx)
            6002 => Some("ApplicationHealthStateError"),
            6003 => Some("HealthStateError"),
            6004 => Some("UnhealthyEventFound"),
            6005 => Some("ClusterHealthEmpty"),
            6006 => Some("AllocationFailed"),
            6007 => Some("OperationResultNull"),
            6008 => Some("OperationResultUnexpected"),
            6009 => Some("FabricNodesHealthError"),

            // Key Vault errors (4xxx)
            4000 => Some("AadClientCredentialsGrantFailure"),
            4001 => Some("AadServiceUnavailable"),
            4002 => Some("KeyVaultAuthenticationFailure"),
            4003 => Some("KeyVaultKeyNotFound"),
            4004 => Some("KeyVaultServiceUnavailable"),
            4005 => Some("KeyVaultWrapUnwrapFailure"),
            4006 => Some("InvalidKeyVaultKeyUri"),
            4007 => Some("InvalidInputBytes"),
            4008 => Some("KeyVaultInternalServerError"),
            4009 => Some("KeyVaultDnsNotResolved"),
            4010 => Some("InvalidKeyVaultCertUri"),
            4011 => Some("InvalidKeyVaultKeyAndCertUri"),
            4012 => Some("CustomerKeyRotated"),
            4013 => Some("MissingRequestParameter"),
            4014 => Some("InvalidKeyVaultSecretUri"),
            4015 => Some("UndefinedDefaultIdentity"),
            4016 => Some("KeyVaultOutboundDeniedByNsp"),
            4017 => Some("KeyVaultNotFound"),
            4018 => Some("KeyDisabledOrExpired"),
            4019 => Some("MasterServiceUnavailable"),

            // AAD/Auth errors (5xxx)
            5000 => Some("MissingAuthHeader"),
            5001 => Some("InvalidAuthHeaderFormat"),
            5002 => Some("AadAuthDisabled"),
            5003 => Some("AadTokenInvalidFormat"),
            5004 => Some("AadTokenInvalidSignature"),
            5005 => Some("AadTokenNotYetValid"),
            5006 => Some("AadTokenExpired"),
            5007 => Some("AadTokenInvalidIssuer"),
            5008 => Some("AadTokenInvalidAudience"),
            5009 => Some("AadTokenInvalidScope"),
            5010 => Some("FailedToGetAadToken"),
            5011 => Some("AadTokenMissingObjectIdentifier"),
            5012 => Some("SasTokenAuthDisabled"),
            5200 => Some("AadTokenInvalidSigningKey"),
            5201 => Some("AadTokenGroupExpansionError"),
            5202 => Some("LocalAuthDisabled"),
            5203 => Some("LocalAuthDisabled"),

            // RBAC errors (53xx)
            5300 => Some("RbacOperationNotSupported"),
            5301 => Some("RbacUnauthorizedMetadataRequest"),
            5302 => Some("RbacUnauthorizedNameBasedDataRequest"),
            5303 => Some("RbacUnauthorizedRidBasedDataRequest"),
            5304 => Some("RbacRidCannotBeResolved"),
            5305 => Some("RbacMissingUserId"),
            5306 => Some("RbacMissingAction"),
            5307 => Some("NspInboundDenied"),

            // 412: Precondition Failed (Migration)
            5325 => Some("MismatchingCollectionRidsOnMigratePartitionDuringMigration"),
            5326 => Some("PartitionNotInMigratingStatusForMigratePartitionRequest"),
            5327 => Some("MissingPartitionResourceOnCompleteMigration"),
            5328 => Some("MissingPartitionResourceOnAbortMigration"),

            // RBAC and retriable writes (54xx)
            5400 => Some("RbacRequestWasNotAuthorized"),
            5401 => Some("InitialRetriableWriteRequestCompleted"),
            5402 => Some("DuplicateRetriableWriteRequest"),
            5403 => Some("ConflictOperationInUserTransaction"),
            5404 => Some("RetriableWriteRequestResponseExpiredInPrimaryCache"),

            // Query execution (6xxx)
            6000 => Some("QueryRequestInitialized"),
            6100 => Some("QueryExecutionInProgress"),
            6200 => Some("QueryExecutionComplete"),
            6300 => Some("CollectionTruncateNotAllowedDuringMerge"),

            // Fabric codes (605x)
            6050 => Some("InsufficientFabricPermissions"),
            6051 => Some("FabricAuthorizationFailed"),
            6052 => Some("FabricOperationUnsupported"),
            6053 => Some("FabricTokenValidationFailed"),
            6054 => Some("InvalidFabricAppId"),
            6055 => Some("InvalidFabricTenantId"),
            6056 => Some("InvalidFabricArtifactId"),

            // SDK Client-side codes (10xxx, 2xxxx) - consistent across .NET and Java
            10001 => Some("GatewayEndpointUnavailable"),
            10002 => Some("GatewayEndpointReadTimeout"),
            10003 => Some("ThroughputControlRequestRateTooLarge"),
            10004 => Some("OfferNotConfigured"),
            10005 => Some("ThroughputControlBulkRequestRateTooLarge"),
            10101 => Some("CustomSerializerException"),
            10102 => Some("InvalidIdValue"),
            20001 => Some("TransportGenerated410"),
            20002 => Some("TimeoutGenerated410"),
            20003 => Some("TransportGenerated503"),
            20004 => Some("ClientCpuOverload"),
            20005 => Some("ClientThreadStarvation"),
            20006 => Some("ChannelClosed"),
            20007 => Some("MalformedContinuationToken"),
            20008 => Some("ClientOperationTimeout"),
            20020 => Some("SerializationResponseBodyInvalid"),
            20021 => Some("SerializationRequestBodyInvalid"),
            20401 => Some("ClientGenerated401"),
            20901 => Some("NegativeTimeoutProvided"),
            20902 => Some("MissingPartitionKeyRangeIdInContext"),
            20903 => Some("InvalidRegionsInSessionToken"),
            20904 => Some("NonPartitionedResources"),
            20905 => Some("PartitionKeyIsNull"),
            20906 => Some("UnknownAuthorizationTokenKind"),
            20907 => Some("RecreateRequestOnHttpClient"),
            20908 => Some("InvalidBackendResponse"),
            20909 => Some("UnknownQuorumResult"),
            20910 => Some("InvalidResult"),
            20911 => Some("TransitTimeout"),
            20912 => Some("ClosedClient"),
            20913 => Some("WriteRegionBarrierChangedMidOperation"),
            20914 => Some("RegionScopedSessionContainerInBadState"),

            // Client SDK–synthesized error codes (20100-20349) — see
            // the constants block on `impl SubStatusCode` for the full
            // catalog and rationale.
            20100 => Some("ClientPartitionKeyEmpty"),
            20101 => Some("ClientPartitionKeyTooManyComponents"),
            20102 => Some("ClientPrefixPartitionKeyRequiresMultiHash"),
            20103 => Some("ClientNonMultiHashPartitionKeyArityMismatch"),
            20104 => Some("ClientConnectionStringEmpty"),
            20105 => Some("ClientConnectionStringMalformedPart"),
            20106 => Some("ClientConnectionStringMissingAccountEndpoint"),
            20107 => Some("ClientConnectionStringMissingAccountKey"),
            20108 => Some("ClientInvalidAccountEndpointUrl"),
            20109 => Some("ClientInvalidUrl"),
            20110 => Some("ClientUnknownConsistencyLevel"),
            20111 => Some("ClientUnknownPriorityLevel"),
            20112 => Some("ClientFeedRangeRequiresFanoutPipeline"),
            20113 => Some("ClientUnsupportedQueryFeature"),
            20114 => Some("ClientQueryPlanInvalidTopOffsetLimit"),
            20115 => Some("ClientQueryPlanComplexProjectionUnsupported"),
            20116 => Some("ClientOpaqueTokenInvalidForCrossPartitionQuery"),
            20117 => Some("ClientContinuationTokenNonQueryOperation"),
            20118 => Some("ClientCrossPartitionFanOutExceeded"),
            20119 => Some("ClientOrderByComplexValueUnsupported"),
            20120 => Some("ClientInvalidResourceId"),
            20121 => Some("ClientMixedNameRidAddressing"),
            20122 => Some("ClientQueryRewriteBodyInvalid"),
            20123 => Some("ClientDistinctValueTooDeeplyNested"),
            20124 => Some("ClientBufferedQueryContinuationUnsupported"),
            20125 => Some("ClientBufferedQueryRequiresFiniteWindow"),
            20126 => Some("ClientNonStreamingOrderByWindowTooLarge"),
            20127 => Some("ClientPartitionKeyNumberNonFinite"),
            20128 => Some("ClientUserAgentSuffixInvalid"),
            20150 => Some("ClientDuplicateFaultInjectionRuleId"),
            20153 => Some("ClientHttpClientConstructionFailed"),
            20154 => Some("ClientReqwestFeatureRequired"),
            20155 => Some("ClientRequestUrlMissingHost"),
            20156 => Some("ClientRequestUrlMissingKnownPort"),
            20157 => Some("ClientImdsHttpClientConstructionFailed"),
            20158 => Some("ClientImdsReqwestFeatureRequired"),
            20200 => Some("ClientContinuationTokenFetchInFlight"),
            20201 => Some("ClientTopologyProviderMissing"),
            20202 => Some("ClientDriverNotInitialized"),
            20203 => Some("ClientContinuationTokenShapeMismatch"),
            20204 => Some("ClientContinuationTokenUnexpectedNestedShape"),
            20205 => Some("ClientContinuationTokenInvalidEpkRange"),
            20206 => Some("ClientSplitRetriesExhausted"),
            20207 => Some("ClientBuildResponseInvokedOnFailure"),
            20208 => Some("ClientRootNodeCannotRequestSplit"),
            20217 => Some("ClientDistinctCannotForwardSplit"),
            20209 => Some("ClientCrossPartitionQueryRequiresContainerRef"),
            20210 => Some("ClientSingletonOperationReturnedEmptyPage"),
            20211 => Some("ClientComputeRangeInvokedWithEmptyPartitionKey"),
            20212 => Some("ClientChangeFeedPipelineUnexpectedlyDrained"),
            20213 => Some("ClientContinuationTokenSavedRangeUnhonored"),
            20214 => Some("ClientContinuationTokenOrderByStateInvalid"),
            20215 => Some("ClientStreamingMergeSplitReplacementInvalid"),
            20216 => Some("ClientContinuationTokenAfterTranscodeFailure"),
            20300 => Some("ClientNoOverlappingFeedRangesForSessionToken"),
            20301 => Some("ClientNoThroughputOfferForResource"),
            20302 => Some("ClientQueryPlanProducedEmptyRanges"),
            20303 => Some("ServiceReturnedOfferWithoutId"),
            20304 => Some("ClientThroughputPollerIncomplete"),
            20305 => Some("ClientTopologyResolutionFailed"),
            20306 => Some("ServiceReturnedObjectWithoutRid"),
            20307 => Some("ClientQueryPlanRangeNotCoveredByTopology"),
            20308 => Some("ServiceOrderByEnvelopeInvalid"),
            20309 => Some("ServiceQueryPlanOrderByMissingRewrittenQuery"),

            // Native FFI wrapper pre-flight / plumbing codes (20350-20399)
            20350 => Some("ClientFfiNullArgument"),
            20351 => Some("ClientFfiInvalidUtf8"),
            20352 => Some("ClientFfiInvalidHeader"),
            20353 => Some("ClientFfiInvalidOptionValue"),
            20354 => Some("ClientFfiOperationConsumed"),
            20355 => Some("ClientFfiPreconditionAlreadySet"),
            20356 => Some("ClientFfiUnsupportedOperationForMutator"),
            20357 => Some("ClientFfiFeedExhausted"),
            20358 => Some("ClientFfiQueueShutdown"),
            20359 => Some("ClientFfiQueueFull"),
            20360 => Some("ClientFfiOperationCancelled"),
            20361 => Some("ClientFfiRuntimeBuildFailed"),
            20362 => Some("ClientFfiPanic"),

            // SDK Server-side codes (21xxx) - consistent across .NET and Java
            21001 => Some("NameCacheIsStaleExceededRetryLimit"),
            21002 => Some("PartitionKeyRangeGoneExceededRetryLimit"),
            21003 => Some("CompletingSplitExceededRetryLimit"),
            21004 => Some("CompletingPartitionMigrationExceededRetryLimit"),
            21005 => Some("ServerGenerated410"),
            21006 => Some("GlobalStrongWriteBarrierNotMet"),
            21007 => Some("ReadQuorumNotMet"),
            21008 => Some("ServerGenerated503"),
            21009 => Some("NoValidStoreResponse"),
            21010 => Some("ServerGenerated408"),
            21011 => Some("ServerBarrierThrottled"),
            21012 => Some("NRegionCommitWriteBarrierNotMet"),

            // ThinProxy codes (13xxx)
            13000 => Some("ThinProxyMultipleAccountsNotAllowed"),
            13001 => Some("ThinProxyPublicEndpointDisabled"),
            13008 => Some("ThinProxyGenerated401"),
            13009 => Some("ThinProxyGenerated408"),
            13010 => Some("ThinProxyRequestThrottled"),
            13011 => Some("ThinProxyGenerated500"),
            13012 => Some("ThinProxyGenerated503"),

            _ => None,
        }
    }
}

impl Default for SubStatusCode {
    fn default() -> Self {
        Self::new(0)
    }
}

impl fmt::Debug for SubStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // For Debug without status code context, show value only for ambiguous codes
        match self.name(None) {
            Some(name) => write!(f, "SubStatusCode::{}({})", name, self.0),
            None => write!(f, "SubStatusCode({})", self.0),
        }
    }
}

impl fmt::Display for SubStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // For Display without status code context, show value only for ambiguous codes
        match self.name(None) {
            Some(name) => write!(f, "{} ({})", name, self.0),
            None => write!(f, "{}", self.0),
        }
    }
}

impl From<u16> for SubStatusCode {
    fn from(value: u16) -> Self {
        SubStatusCode(value)
    }
}

impl From<SubStatusCode> for u16 {
    fn from(code: SubStatusCode) -> Self {
        code.0
    }
}

/// Combined HTTP status code and optional Cosmos DB sub-status code.
///
/// This type keeps the HTTP status code and Cosmos sub-status code together,
/// which is essential because the meaning of a sub-status code depends on
/// the HTTP status code it's paired with.
///
/// # Sub-Status Ambiguity
///
/// Some sub-status codes have different meanings depending on the HTTP status code:
///
/// | Sub-Status | HTTP 404 | HTTP 410 |
/// |-----------|----------|----------|
/// | 1002 | ReadSessionNotAvailable | PartitionKeyRangeGone |
/// | 1007 | — | CompletingSplitOrMerge (410), InsufficientBindablePartitions (503) |
/// | 1008 | — | CompletingPartitionMigration (410), DatabaseAccountNotFound (403) |
///
/// By pairing both codes, `CosmosStatus` can always resolve the correct name.
///
/// # Example
///
/// ```rust
/// use azure_core::http::StatusCode;
/// use azure_data_cosmos_driver::models::CosmosStatus;
///
/// // Unambiguous status
/// let throttled = CosmosStatus::new(StatusCode::TooManyRequests).with_sub_status(3200);
/// assert_eq!(throttled.name(), Some("RUBudgetExceeded"));
/// assert!(throttled.is_throttled());
///
/// // Disambiguated by HTTP status code
/// let session_not_available = CosmosStatus::new(StatusCode::NotFound).with_sub_status(1002);
/// assert_eq!(session_not_available.name(), Some("ReadSessionNotAvailable"));
///
/// let pk_range_gone = CosmosStatus::new(StatusCode::Gone).with_sub_status(1002);
/// assert_eq!(pk_range_gone.name(), Some("PartitionKeyRangeGone"));
/// ```
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct CosmosStatus {
    pub(crate) status_code: StatusCode,
    pub(crate) sub_status: Option<SubStatusCode>,
}

impl CosmosStatus {
    /// Creates a `CosmosStatus` with only an HTTP status code (no sub-status).
    pub fn new(status_code: StatusCode) -> Self {
        Self {
            status_code,
            sub_status: None,
        }
    }

    /// Sets the sub-status code on this `CosmosStatus`, returning the modified value.
    pub fn with_sub_status(mut self, sub_status_code: u16) -> Self {
        self.sub_status = Some(SubStatusCode::new(sub_status_code));
        self
    }

    /// Creates a `CosmosStatus` from raw parts.
    pub(crate) fn from_parts(status_code: StatusCode, sub_status: Option<SubStatusCode>) -> Self {
        Self {
            status_code,
            sub_status,
        }
    }

    /// Returns the HTTP status code.
    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    /// Returns the sub-status code, if present.
    pub fn sub_status(&self) -> Option<SubStatusCode> {
        self.sub_status
    }

    /// Returns `true` if the HTTP status indicates success (2xx) or 304 Not Modified.
    pub fn is_success(&self) -> bool {
        self.status_code.is_success() || u16::from(self.status_code) == 304
    }

    /// Returns `true` if this is a throttling response (HTTP 429).
    pub fn is_throttled(&self) -> bool {
        u16::from(self.status_code) == 429
    }

    /// Returns `true` if this is an HTTP 449 RetryWith response.
    ///
    /// 449 RetryWith is returned by Cosmos backends for transient
    /// concurrency conflicts (e.g. concurrent writes racing through the
    /// store, RBAC info momentarily unavailable). The client is expected
    /// to retry in the same region after a short delay. See
    /// `try_handle_retry_with` in
    /// `driver::pipeline::retry_evaluation` and the cross-SDK
    /// `RetryWithRetryPolicy` for the policy.
    pub fn is_retry_with(&self) -> bool {
        u16::from(self.status_code) == 449
    }

    /// Returns `true` if this is an HTTP 410 Gone response.
    pub fn is_gone(&self) -> bool {
        u16::from(self.status_code) == 410
    }

    /// Returns `true` if this is a "clean" HTTP 404 Not Found response — that
    /// is, status code 404 with either no sub-status or sub-status `0`
    /// (`UNKNOWN`).
    ///
    /// Non-zero sub-statuses on 404 carry meaningfully different semantics
    /// (e.g. `1002` `READ_SESSION_NOT_AVAILABLE` is a transient session-
    /// consistency signal, `1003` `OWNER_RESOURCE_NOT_FOUND` indicates the
    /// parent database/container is missing, etc.) and would be misleading
    /// to surface as a generic "not found". Callers wanting to detect those
    /// should match the corresponding [`CosmosStatus`] predicate or constant
    /// explicitly.
    pub fn is_not_found(&self) -> bool {
        u16::from(self.status_code) == 404
            && self
                .sub_status
                .is_none_or(|s| s == crate::error::status_codes::substatus::UNKNOWN)
    }

    /// Returns `true` if this is an HTTP 409 Conflict response.
    pub fn is_conflict(&self) -> bool {
        u16::from(self.status_code) == 409
    }

    /// Returns `true` if this is an HTTP 412 Precondition Failed response.
    pub fn is_precondition_failed(&self) -> bool {
        u16::from(self.status_code) == 412
    }

    /// Returns `true` if this is an HTTP 408 (request timeout) response —
    /// covers both a service-side timeout and a synthetic client-side
    /// end-to-end timeout (`408 / 20008`).
    pub fn is_timeout(&self) -> bool {
        u16::from(self.status_code) == 408
    }

    /// Returns `true` if this is an HTTP 400 (bad request) response.
    pub fn is_bad_request(&self) -> bool {
        u16::from(self.status_code) == 400
    }

    /// Returns `true` if this is an HTTP 401 (unauthorized) response —
    /// covers both a service-side 401 and the SDK-synthesized
    /// `CLIENT_GENERATED_401` / `AUTHENTICATION_TOKEN_ACQUISITION_FAILED`.
    pub fn is_unauthorized(&self) -> bool {
        u16::from(self.status_code) == 401
    }

    /// Returns `true` if this is an HTTP 403 (forbidden) response. Use
    /// [`is_write_forbidden`](Self::is_write_forbidden) for the specific
    /// 403 / 3 case that indicates the region is not the write region.
    pub fn is_forbidden(&self) -> bool {
        u16::from(self.status_code) == 403
    }

    /// Returns `true` if this is an HTTP 503 (service unavailable) response
    /// — covers both a service-side 503 and synthetic transport-generated
    /// 503s. Use [`is_transport_generated_503`](Self::is_transport_generated_503)
    /// to detect the synthetic case specifically.
    pub fn is_service_unavailable(&self) -> bool {
        u16::from(self.status_code) == 503
    }

    /// Returns `true` if the error is generally considered transient and could
    /// reasonably be retried by a higher layer.
    ///
    /// The categorical retry-trigger set is `408 / 429 / 449 / 503`, which
    /// covers both real service responses (e.g. a service-side 503) and the
    /// SDK's synthetic transport-generated codes (`TRANSPORT_GENERATED_503`,
    /// `CLIENT_OPERATION_TIMEOUT` on `408`, etc.) since both share the same
    /// HTTP status code by construction.
    pub fn is_transient(&self) -> bool {
        matches!(u16::from(self.status_code), 408 | 429 | 449 | 503)
    }

    /// Returns `true` if this is a write-forbidden error (HTTP 403, sub-status 3).
    pub fn is_write_forbidden(&self) -> bool {
        u16::from(self.status_code) == 403
            && self.sub_status == Some(crate::error::status_codes::substatus::WRITE_FORBIDDEN)
    }

    /// Returns `true` for HTTP 403/sub-status 1008, where the region no longer owns the account.
    ///
    /// Note: sub-status 1008 is overloaded on HTTP 410 for partition migration.
    pub fn is_database_account_not_found(&self) -> bool {
        u16::from(self.status_code) == 403
            && self.sub_status
                == Some(crate::error::status_codes::substatus::DATABASE_ACCOUNT_NOT_FOUND)
    }

    /// Returns `true` if this is a read-session-not-available error (HTTP 404, sub-status 1002).
    pub fn is_read_session_not_available(&self) -> bool {
        u16::from(self.status_code) == 404
            && self.sub_status
                == Some(crate::error::status_codes::substatus::READ_SESSION_NOT_AVAILABLE)
    }

    /// Returns `true` if this is a partition-key-range-gone error (HTTP 410, sub-status 1002).
    pub fn is_partition_key_range_gone(&self) -> bool {
        u16::from(self.status_code) == 410
            && self.sub_status
                == Some(crate::error::status_codes::substatus::PARTITION_KEY_RANGE_GONE)
    }

    /// Returns `true` if this is an HTTP 410 caused by partition topology changing.
    pub(crate) fn is_partition_topology_change(&self) -> bool {
        u16::from(self.status_code) == 410
            && matches!(
                self.sub_status,
                Some(
                    crate::error::status_codes::substatus::PARTITION_KEY_RANGE_GONE
                        | crate::error::status_codes::substatus::COMPLETING_SPLIT
                        | crate::error::status_codes::substatus::COMPLETING_PARTITION_MIGRATION
                )
            )
    }

    /// Returns `true` if this indicates a transport-generated 503 (client-side).
    pub fn is_transport_generated_503(&self) -> bool {
        u16::from(self.status_code) == 503
            && self.sub_status
                == Some(crate::error::status_codes::substatus::TRANSPORT_GENERATED_503)
    }

    /// Returns `true` when this status is a **final** (non-retriable) outcome
    /// for the purposes of cross-region hedging.
    ///
    /// Final statuses are:
    /// * any 1xx / 2xx / 3xx response,
    /// * the explicitly non-retriable client errors `400`, `401`, `403`
    ///   (regardless of sub-status), `405`, `409`, `412`, `413`, `422`,
    ///   `451`,
    /// * the explicitly non-retriable server errors `501` and `505`,
    /// * `404` with no sub-status (or sub-status `0`),
    /// * `429` with a sub-status describing throttling that a second region
    ///   cannot relieve — `3200` (`RU_BUDGET_EXCEEDED`), `3210`
    ///   (`RU_BUDGET_EXCEEDED_FOR_MASTER`), and `3214`
    ///   (`HOT_PARTITION_KEY_THROTTLED`).
    ///
    /// Everything else — including `404/1002`, `408`, generic/`3092` `429`,
    /// `410`, `500`, and `503` — is treated as retriable so the racing hedge
    /// gets a chance to win.
    ///
    /// The `429` carve-out mirrors the *hedge-spawn* guard (§7.2.1): RU-budget
    /// and hot-partition throttles are account-/partition-wide, so racing a
    /// second region only doubles the load on an already-saturated resource.
    /// A generic `429` (no sub-status) and the transient-capacity `3092`
    /// (`SYSTEM_RESOURCE_UNAVAILABLE`) stay retriable, since another region
    /// genuinely may have spare capacity.
    ///
    /// Protocol-level and policy errors (`403`, `422`, `451`, `501`, `505`)
    /// are final because no alternate region can change the outcome:
    /// * `403` is an authorization/ownership decision (RBAC, write-forbidden,
    ///   account ownership) — racing another region either duplicates the
    ///   denial or unnecessarily doubles a security-sensitive signal. When a
    ///   `403` *can* be retried (e.g., write-forbidden on single-master
    ///   PPAF), the dedicated retry path handles it via the normal retry
    ///   loop rather than via a parallel hedge race.
    ///   * **Exception — `403 / 1008` (`DatabaseAccountNotFound`):** topology
    ///     ownership changed, so the outer retry pipeline must refresh and fail over.
    /// * `422`, `451`, `501`, `505` are payload/policy/protocol issues that
    ///   another region cannot resolve. Racing a hedge against them only
    ///   wastes RU and request budget.
    pub(crate) fn is_final_result(&self) -> bool {
        let code: u16 = self.status_code.into();
        if code < 400 {
            return true;
        }

        let sub = self.sub_status.map(|s| s.value()).unwrap_or(0);
        if code == 403 && sub == 1008 {
            // DatabaseAccountNotFound — see exception in the doc comment above.
            return false;
        }
        matches!(
            code,
            400 | 401 | 403 | 405 | 409 | 412 | 413 | 422 | 451 | 501 | 505
        ) || (code == 404 && sub == 0)
            || (code == 429
                && matches!(
                    sub,
                    3200 // RU_BUDGET_EXCEEDED
                        | 3210 // RU_BUDGET_EXCEEDED_FOR_MASTER
                        | 3214 // HOT_PARTITION_KEY_THROTTLED
                ))
    }

    /// Returns the human-readable name of this status combination, if known.
    ///
    /// Unlike the raw sub-status code, this method always resolves ambiguous
    /// codes correctly because it has the HTTP status code context.
    ///
    /// Returns `None` for unknown sub-status codes or when no sub-status is present.
    ///
    /// # Example
    ///
    /// ```rust
    /// use azure_core::http::StatusCode;
    /// use azure_data_cosmos_driver::models::CosmosStatus;
    ///
    /// let status = CosmosStatus::new(StatusCode::NotFound).with_sub_status(1002);
    /// assert_eq!(status.name(), Some("ReadSessionNotAvailable"));
    ///
    /// let status = CosmosStatus::new(StatusCode::Gone).with_sub_status(1002);
    /// assert_eq!(status.name(), Some("PartitionKeyRangeGone"));
    ///
    /// let status = CosmosStatus::new(StatusCode::Ok);
    /// assert_eq!(status.name(), None); // No sub-status
    /// ```
    pub fn name(&self) -> Option<&'static str> {
        let sub = self.sub_status?;
        sub.name(Some(self.status_code))
    }
}

impl fmt::Debug for CosmosStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_u16: u16 = self.status_code.into();
        match (self.sub_status, self.name()) {
            (Some(sub), Some(name)) => {
                write!(f, "CosmosStatus({}/{} {})", status_u16, sub.value(), name,)
            }
            (Some(sub), None) => write!(f, "CosmosStatus({}/{})", status_u16, sub.value(),),
            (None, _) => write!(f, "CosmosStatus({})", status_u16),
        }
    }
}

impl fmt::Display for CosmosStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_u16: u16 = self.status_code.into();
        match (self.sub_status, self.name()) {
            (Some(sub), Some(name)) => write!(f, "{}/{} ({})", status_u16, sub.value(), name,),
            (Some(sub), None) => write!(f, "{}/{}", status_u16, sub.value()),
            (None, _) => write!(f, "{}", status_u16),
        }
    }
}

/// Allows ergonomic comparisons like `assert_eq!(status, StatusCode::Ok)`.
///
/// Compares only the HTTP status code, ignoring sub-status. Use
/// [`CosmosStatus::sub_status`] explicitly when sub-status comparison is required.
impl PartialEq<StatusCode> for CosmosStatus {
    fn eq(&self, other: &StatusCode) -> bool {
        self.status_code == *other
    }
}

impl PartialEq<CosmosStatus> for StatusCode {
    fn eq(&self, other: &CosmosStatus) -> bool {
        *self == other.status_code
    }
}

impl From<CosmosStatus> for StatusCode {
    fn from(s: CosmosStatus) -> Self {
        s.status_code
    }
}

impl From<CosmosStatus> for u16 {
    fn from(s: CosmosStatus) -> Self {
        s.status_code.into()
    }
}

impl Serialize for CosmosStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("CosmosStatus", 1)?;
        s.serialize_field("status", &self.to_string())?;
        s.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_without_sub_status() {
        let status = CosmosStatus::new(StatusCode::Ok);
        assert_eq!(status.status_code(), StatusCode::Ok);
        assert!(status.sub_status().is_none());
        assert!(status.is_success());
        assert!(status.name().is_none());
    }

    #[test]
    fn client_rid_addressing_status_names() {
        // The 20120/20121 client statuses must resolve to searchable names so the
        // deterministic client-side errors are useful in diagnostics and logs.
        assert_eq!(
            crate::error::status_codes::CLIENT_INVALID_RESOURCE_ID.name(),
            Some("ClientInvalidResourceId")
        );
        assert_eq!(
            crate::error::status_codes::CLIENT_MIXED_NAME_RID_ADDRESSING.name(),
            Some("ClientMixedNameRidAddressing")
        );
    }

    #[test]
    fn buffered_query_status_codes_and_names() {
        for (code, expected, name) in [
            (
                20124,
                crate::error::status_codes::CLIENT_BUFFERED_QUERY_CONTINUATION_UNSUPPORTED,
                "ClientBufferedQueryContinuationUnsupported",
            ),
            (
                20125,
                crate::error::status_codes::CLIENT_BUFFERED_QUERY_REQUIRES_FINITE_WINDOW,
                "ClientBufferedQueryRequiresFiniteWindow",
            ),
            (
                20126,
                crate::error::status_codes::CLIENT_NON_STREAMING_ORDER_BY_WINDOW_TOO_LARGE,
                "ClientNonStreamingOrderByWindowTooLarge",
            ),
        ] {
            let status = CosmosStatus::new(StatusCode::BadRequest).with_sub_status(code);
            assert_eq!(status, expected);
            assert_eq!(status.name(), Some(name));
        }
        assert_eq!(
            CosmosStatus::new(StatusCode::BadRequest)
                .with_sub_status(20129)
                .name(),
            None
        );
    }

    #[test]
    fn constructor_validation_status_codes_and_names() {
        for (code, expected, name) in [
            (
                20127,
                crate::error::status_codes::CLIENT_PARTITION_KEY_NUMBER_NON_FINITE,
                "ClientPartitionKeyNumberNonFinite",
            ),
            (
                20128,
                crate::error::status_codes::CLIENT_USER_AGENT_SUFFIX_INVALID,
                "ClientUserAgentSuffixInvalid",
            ),
        ] {
            let status = CosmosStatus::new(StatusCode::BadRequest).with_sub_status(code);
            assert_eq!(status, expected);
            assert_eq!(status.name(), Some(name));
        }
    }

    #[test]
    fn buffered_query_continuation_aliases() {
        let status = crate::error::status_codes::CLIENT_BUFFERED_QUERY_CONTINUATION_UNSUPPORTED;
        assert_eq!(
            status,
            crate::error::status_codes::CLIENT_DISTINCT_CONTINUATION_UNSUPPORTED
        );
        assert_eq!(
            status,
            crate::error::status_codes::CLIENT_NON_STREAMING_ORDER_BY_CONTINUATION_UNSUPPORTED
        );
        assert_eq!(status.sub_status(), Some(SubStatusCode::new(20124)));
        assert_eq!(status.sub_status(), Some(SubStatusCode::new(20124)));
    }

    #[test]
    fn buffered_query_admission_aliases() {
        let status = CosmosStatus::new(StatusCode::BadRequest).with_sub_status(20125);
        assert_eq!(
            status,
            crate::error::status_codes::CLIENT_BUFFERED_QUERY_REQUIRES_FINITE_WINDOW
        );
        assert_eq!(
            status,
            crate::error::status_codes::CLIENT_NON_STREAMING_ORDER_BY_REQUIRES_FINITE_WINDOW
        );
        assert_eq!(status.sub_status(), Some(SubStatusCode::new(20125)));
        assert_eq!(
            status.name(),
            Some("ClientBufferedQueryRequiresFiniteWindow")
        );
    }

    #[test]
    fn with_sub_status_unambiguous() {
        let status = CosmosStatus::new(StatusCode::TooManyRequests).with_sub_status(3200);
        assert_eq!(status.status_code(), StatusCode::TooManyRequests);
        assert_eq!(
            status.sub_status(),
            Some(crate::error::status_codes::substatus::RU_BUDGET_EXCEEDED)
        );
        assert!(status.is_throttled());
        assert_eq!(status.name(), Some("RUBudgetExceeded"));
    }

    #[test]
    fn disambiguates_1002_404_vs_410() {
        let not_found = CosmosStatus::new(StatusCode::NotFound).with_sub_status(1002);
        assert_eq!(not_found.name(), Some("ReadSessionNotAvailable"));
        assert!(not_found.is_read_session_not_available());
        assert!(!not_found.is_partition_key_range_gone());

        let gone = CosmosStatus::new(StatusCode::Gone).with_sub_status(1002);
        assert_eq!(gone.name(), Some("PartitionKeyRangeGone"));
        assert!(gone.is_partition_key_range_gone());
        assert!(!gone.is_read_session_not_available());
    }

    #[test]
    fn disambiguates_1008_403_vs_410() {
        let forbidden = CosmosStatus::new(StatusCode::Forbidden).with_sub_status(1008);
        assert_eq!(forbidden.name(), Some("DatabaseAccountNotFound"));

        let gone = CosmosStatus::new(StatusCode::Gone).with_sub_status(1008);
        assert_eq!(gone.name(), Some("CompletingPartitionMigration"));
    }

    #[test]
    fn well_known_constants() {
        assert!(crate::error::status_codes::TRANSPORT_GENERATED_503.is_transport_generated_503());
        assert!(
            crate::error::status_codes::READ_SESSION_NOT_AVAILABLE.is_read_session_not_available()
        );
        assert!(crate::error::status_codes::PARTITION_KEY_RANGE_GONE.is_partition_key_range_gone());
        assert!(crate::error::status_codes::WRITE_FORBIDDEN.is_write_forbidden());
        assert!(crate::error::status_codes::RU_BUDGET_EXCEEDED.is_throttled());
    }

    #[test]
    fn is_success() {
        assert!(CosmosStatus::new(StatusCode::Ok).is_success());
        assert!(CosmosStatus::new(StatusCode::Created).is_success());
        assert!(!CosmosStatus::new(StatusCode::NotFound).is_success());
        assert!(!crate::error::status_codes::RU_BUDGET_EXCEEDED.is_success());
    }

    #[test]
    fn display_with_name() {
        let status = CosmosStatus::new(StatusCode::TooManyRequests).with_sub_status(3200);
        assert_eq!(format!("{}", status), "429/3200 (RUBudgetExceeded)");
    }

    #[test]
    fn display_without_sub_status() {
        let status = CosmosStatus::new(StatusCode::Ok);
        assert_eq!(format!("{}", status), "200");
    }

    #[test]
    fn display_unknown_sub_status() {
        let status = CosmosStatus::new(StatusCode::Ok).with_sub_status(65000);
        assert_eq!(format!("{}", status), "200/65000");
    }

    #[test]
    fn debug_format() {
        let status = CosmosStatus::new(StatusCode::NotFound).with_sub_status(1002);
        assert_eq!(
            format!("{:?}", status),
            "CosmosStatus(404/1002 ReadSessionNotAvailable)"
        );
    }

    #[test]
    fn equality() {
        assert_eq!(
            CosmosStatus::new(StatusCode::NotFound).with_sub_status(1002),
            crate::error::status_codes::READ_SESSION_NOT_AVAILABLE
        );
        assert_ne!(
            CosmosStatus::new(StatusCode::NotFound).with_sub_status(1002),
            CosmosStatus::new(StatusCode::Gone).with_sub_status(1002),
        );
    }

    #[test]
    fn serializes_named_substatus() {
        let status = CosmosStatus::new(StatusCode::TooManyRequests).with_sub_status(3200);
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"status\":\"429/3200 (RUBudgetExceeded)\""));
    }

    #[test]
    fn serialization_without_sub_status() {
        let status = CosmosStatus::new(StatusCode::Ok);
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"status\":\"200\""));
    }

    #[test]
    fn new_with_zero_status() {
        let status = CosmosStatus::new(StatusCode::from(0));
        assert_eq!(u16::from(status.status_code()), 0);
        assert!(status.sub_status().is_none());
    }

    // =========================================================================
    // SubStatusCode tests
    // =========================================================================

    #[test]
    fn new_and_value() {
        let code = SubStatusCode::new(1002);
        assert_eq!(code.value(), 1002);
    }

    #[test]
    fn from_header_value_valid() {
        let code = SubStatusCode::from_header_value("1002");
        assert!(code.is_some());
        assert_eq!(code.unwrap().value(), 1002);
    }

    #[test]
    fn from_header_value_with_whitespace() {
        let code = SubStatusCode::from_header_value("  1002  ");
        assert!(code.is_some());
        assert_eq!(code.unwrap().value(), 1002);
    }

    #[test]
    fn from_header_value_invalid() {
        let code = SubStatusCode::from_header_value("not-a-number");
        assert!(code.is_none());
    }

    #[test]
    fn from_u16() {
        let code = SubStatusCode::from(3200u16);
        assert_eq!(
            code,
            crate::error::status_codes::substatus::RU_BUDGET_EXCEEDED
        );
    }

    #[test]
    fn into_u16() {
        let value: u16 = crate::error::status_codes::substatus::RU_BUDGET_EXCEEDED.into();
        assert_eq!(value, 3200);
    }

    #[test]
    fn display_known_code() {
        let code = crate::error::status_codes::substatus::RU_BUDGET_EXCEEDED;
        assert_eq!(format!("{}", code), "RUBudgetExceeded (3200)");
    }

    #[test]
    fn display_unknown_code() {
        let code = SubStatusCode::new(65000);
        assert_eq!(format!("{}", code), "65000");
    }

    #[test]
    fn display_ambiguous_code_without_context() {
        // 1002 is ambiguous (404 vs 410), so without status code it shows just the number
        let code = SubStatusCode::new(1002);
        assert_eq!(format!("{}", code), "1002");
    }

    #[test]
    fn debug_known_code() {
        // RU_BUDGET_EXCEEDED (3200) is unambiguous
        let code = crate::error::status_codes::substatus::RU_BUDGET_EXCEEDED;
        assert_eq!(
            format!("{:?}", code),
            "SubStatusCode::RUBudgetExceeded(3200)"
        );
    }

    #[test]
    fn debug_unknown_code() {
        let code = SubStatusCode::new(65000);
        assert_eq!(format!("{:?}", code), "SubStatusCode(65000)");
    }

    #[test]
    fn debug_ambiguous_code_without_context() {
        // 1002 is ambiguous, so Debug shows just the value
        let code = SubStatusCode::new(1002);
        assert_eq!(format!("{:?}", code), "SubStatusCode(1002)");
    }

    #[test]
    fn sub_status_code_equality() {
        assert_eq!(
            SubStatusCode::new(1002),
            crate::error::status_codes::substatus::PARTITION_KEY_RANGE_GONE
        );
        assert_eq!(
            SubStatusCode::new(1002),
            crate::error::status_codes::substatus::READ_SESSION_NOT_AVAILABLE
        );
        assert_ne!(
            SubStatusCode::new(1002),
            crate::error::status_codes::substatus::NAME_CACHE_STALE
        );
    }

    #[test]
    fn default_is_unknown() {
        assert_eq!(
            SubStatusCode::default(),
            crate::error::status_codes::substatus::UNKNOWN
        );
        assert_eq!(SubStatusCode::default().value(), 0);
    }

    #[test]
    fn name_returns_some_for_unambiguous() {
        assert_eq!(
            crate::error::status_codes::substatus::RU_BUDGET_EXCEEDED.name(None),
            Some("RUBudgetExceeded")
        );
    }

    #[test]
    fn name_returns_client_generated_401() {
        // Regression guard: the 20401 name mapping must stay in lockstep with
        // the `CLIENT_GENERATED_401` constant so diagnostics keep rendering a
        // name instead of `None`.
        assert_eq!(
            crate::error::status_codes::substatus::CLIENT_GENERATED_401.name(None),
            Some("ClientGenerated401")
        );
    }

    #[test]
    fn name_returns_none_for_unknown() {
        assert_eq!(SubStatusCode::new(65000).name(None), None);
    }

    #[test]
    fn name_returns_serialization_boundary_codes() {
        // Regression guard: the 20020/20021 name mappings must stay in lockstep
        // with the `SERIALIZATION_*_BODY_INVALID` constants so diagnostics render
        // a symbolic name instead of a bare number.
        assert_eq!(
            crate::error::status_codes::substatus::SERIALIZATION_RESPONSE_BODY_INVALID.name(None),
            Some("SerializationResponseBodyInvalid")
        );
        assert_eq!(
            crate::error::status_codes::substatus::SERIALIZATION_REQUEST_BODY_INVALID.name(None),
            Some("SerializationRequestBodyInvalid")
        );
    }

    #[test]
    fn name_returns_none_for_ambiguous_without_status() {
        // 1002 is ambiguous between 404 and 410
        assert_eq!(SubStatusCode::new(1002).name(None), None);
    }

    #[test]
    fn name_disambiguates_with_status_code_404() {
        let code = SubStatusCode::new(1002);
        assert_eq!(
            code.name(Some(StatusCode::NotFound)),
            Some("ReadSessionNotAvailable")
        );
    }

    #[test]
    fn name_disambiguates_with_status_code_410() {
        let code = SubStatusCode::new(1002);
        assert_eq!(
            code.name(Some(StatusCode::Gone)),
            Some("PartitionKeyRangeGone")
        );
    }

    #[test]
    fn name_1007_disambiguates_410_vs_503() {
        let code = SubStatusCode::new(1007);
        assert_eq!(code.name(None), None); // Ambiguous
        assert_eq!(
            code.name(Some(StatusCode::Gone)),
            Some("CompletingSplitOrMerge")
        );
        assert_eq!(
            code.name(Some(StatusCode::ServiceUnavailable)),
            Some("InsufficientBindablePartitions")
        );
    }

    #[test]
    fn name_1008_disambiguates_403_vs_410() {
        let code = SubStatusCode::new(1008);
        assert_eq!(code.name(None), None); // Ambiguous
        assert_eq!(
            code.name(Some(StatusCode::Forbidden)),
            Some("DatabaseAccountNotFound")
        );
        assert_eq!(
            code.name(Some(StatusCode::Gone)),
            Some("CompletingPartitionMigration")
        );
    }

    #[test]
    fn name_3207_disambiguates_409_vs_429() {
        let code = SubStatusCode::new(3207);
        assert_eq!(code.name(None), None); // Ambiguous
        assert_eq!(
            code.name(Some(StatusCode::Conflict)),
            Some("ConfigurationNameAlreadyExists")
        );
        assert_eq!(
            code.name(Some(StatusCode::TooManyRequests)),
            Some("PrepareTimeLimitExceeded")
        );
    }

    #[test]
    fn serialization() {
        let code = SubStatusCode::new(3200);
        let json = serde_json::to_string(&code).unwrap();
        assert_eq!(json, "3200");

        let deserialized: SubStatusCode = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, code);
    }

    #[test]
    fn sdk_client_codes() {
        // Verify SDK client-side codes match Java/NET
        assert_eq!(
            crate::error::status_codes::substatus::TRANSPORT_GENERATED_503.value(),
            20003
        );
        assert_eq!(
            crate::error::status_codes::substatus::CLIENT_OPERATION_TIMEOUT.value(),
            20008
        );
    }
}
