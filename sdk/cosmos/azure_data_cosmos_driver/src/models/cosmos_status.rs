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
pub struct SubStatusCode(u32);

impl SubStatusCode {
    /// Creates a new `SubStatusCode` from a numeric value.
    pub const fn new(code: u32) -> Self {
        Self(code)
    }

    /// Returns the numeric value of the sub-status code.
    pub const fn value(&self) -> u32 {
        self.0
    }

    /// Creates a `SubStatusCode` from a header string.
    ///
    /// Returns `None` if parsing fails.
    pub fn from_header_value(s: &str) -> Option<Self> {
        s.trim().parse::<u32>().ok().map(SubStatusCode)
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

            // Codes with SINGLE meaning (no status code ambiguity)

            // 204: Head requests - LSN differences (unambiguous)
            2013 => Some("MissedTargetGlobalCommittedLsnOver1000"),
            2014 => Some("MissedTargetGlobalCommittedLsnOver10000"),

            // 400: Bad Request
            1001 => Some("PartitionKeyMismatch"),
            1004 => Some("CrossPartitionQueryNotServable"),
            0xFFFF => Some("ScriptCompileError"),
            3205 => Some("AnotherOfferReplaceOperationIsInProgress"),
            1101 => Some("HttpListenerException"),

            // 403: Forbidden
            3 => Some("WriteForbidden"),
            1005 => Some("ProvisionLimitReached"),
            1009 => Some("RedundantCollectionPut"),
            1010 => Some("SharedThroughputDatabaseQuotaExceeded"),
            1011 => Some("SharedThroughputOfferGrowNotNeeded"),
            1014 => Some("PartitionKeyQuotaOverLimit"),
            1019 => Some("SharedThroughputDatabaseCollectionCountExceeded"),
            1020 => Some("SharedThroughputDatabaseCountExceeded"),
            1021 => Some("ComputeInternalError"),
            1028 => Some("ThroughputCapQuotaExceeded"),
            1029 => Some("InvalidThroughputCapValue"),

            // 404: Not Found
            1003 => Some("OwnerResourceNotFound"),
            1013 => Some("CollectionCreateInProgress"),
            1023 => Some("StoreNotReady"),
            1024 => Some("ArchivalPartitionNotPresent"),
            1030 => Some("AuthTokenNotFoundInCache"),
            1031 => Some("PartitionMigratingCollectionDeleted"),
            1034 => Some("PartitionMigrationSourcePartitionDeletedInMaster"),
            1035 => Some("PartitionMigrationSharedThroughputDbPartitionNotFound"),
            1036 => Some("PartitionMigrationPartitionResourceNotFound"),
            1037 => Some("PartitionMigrationFailedToUpdateDns"),

            // 409: Conflict
            1006 => Some("ConflictWithControlPlane"),
            3206 => Some("DatabaseNameAlreadyExists"),
            3302 => Some("PartitionKeyHashCollisionForId"),
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

            // 429: Too Many Requests
            3200 => Some("RUBudgetExceeded"),
            3201 => Some("GatewayThrottled"),
            3208 => Some("ClientTcpChannelFull"),
            3209 => Some("BWTermCountLimitExceeded"),
            3084 => Some("StoredProcedureConcurrency"),
            3088 => Some("ThrottleDueToSplit"),
            3092 => Some("SystemResourceUnavailable"),
            3103 => Some("ThrottleDueToTransportBufferUsage"),
            3213 => Some("TooManyThroughputBucketUpdates"),
            3300 => Some("MicrosoftFabricCUBudgetExceeded"),

            // 500: Internal Server Error
            3001 => Some("ConfigurationNameNotEmpty"),
            3002 => Some("ConfigurationOperationCancelled"),
            3003 => Some("InvalidAccountConfiguration"),
            3004 => Some("FederationDoesNotExistOrIsLocked"),
            3010 => Some("PartitionFailoverErrorCode"),
            3021 => Some("OperationManagerDequeuePumpStopped"),
            3042 => Some("OperationCancelledWithNoRollback"),
            3043 => Some("SplitTimedOut"),

            // 503: Service Unavailable
            9001 => Some("OperationPaused"),
            9002 => Some("ServiceIsOffline"),
            9003 => Some("InsufficientCapacity"),

            // Federation/Health errors (6xxx)
            6001 => Some("AggregatedHealthStateError"),
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

            // RBAC errors (53xx, 54xx)
            5300 => Some("RbacOperationNotSupported"),
            5301 => Some("RbacUnauthorizedMetadataRequest"),
            5302 => Some("RbacUnauthorizedNameBasedDataRequest"),
            5303 => Some("RbacUnauthorizedRidBasedDataRequest"),
            5304 => Some("RbacRidCannotBeResolved"),
            5305 => Some("RbacMissingUserId"),
            5306 => Some("RbacMissingAction"),
            5307 => Some("NspInboundDenied"),
            5400 => Some("RbacRequestWasNotAuthorized"),

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

    // =========================================================================
    // Constants - organized by HTTP status code context
    // =========================================================================

    // ----- General -----

    /// Unknown sub-status code (0).
    pub const UNKNOWN: SubStatusCode = SubStatusCode(0);

    /// Operation is in progress (100).
    pub const OPERATION_IN_PROGRESS: SubStatusCode = SubStatusCode(100);

    // ----- 400: Bad Request -----

    /// Partition key mismatch (1001).
    pub const PARTITION_KEY_MISMATCH: SubStatusCode = SubStatusCode(1001);

    /// Cross-partition query not servable (1004).
    pub const CROSS_PARTITION_QUERY_NOT_SERVABLE: SubStatusCode = SubStatusCode(1004);

    /// Another offer replace operation is in progress (3205).
    pub const OFFER_REPLACE_IN_PROGRESS: SubStatusCode = SubStatusCode(3205);

    /// Script compile error (65535 / 0xFFFF).
    pub const SCRIPT_COMPILE_ERROR: SubStatusCode = SubStatusCode(0xFFFF);

    /// HTTP listener exception (1101).
    pub const HTTP_LISTENER_EXCEPTION: SubStatusCode = SubStatusCode(1101);

    // ----- 403: Forbidden -----

    /// Write operations forbidden (3).
    pub const WRITE_FORBIDDEN: SubStatusCode = SubStatusCode(3);

    /// Provision limit reached (1005).
    pub const PROVISION_LIMIT_REACHED: SubStatusCode = SubStatusCode(1005);

    /// Database account not found (1008).
    /// Note: Same value as `COMPLETING_PARTITION_MIGRATION` for 410.
    pub const DATABASE_ACCOUNT_NOT_FOUND: SubStatusCode = SubStatusCode(1008);

    /// Shared throughput database quota exceeded (1010).
    pub const DATABASE_QUOTA_EXCEEDED: SubStatusCode = SubStatusCode(1010);

    /// Throughput cap quota exceeded (1028).
    pub const THROUGHPUT_CAP_EXCEEDED: SubStatusCode = SubStatusCode(1028);

    /// Invalid throughput cap value (1029).
    pub const INVALID_THROUGHPUT_CAP_VALUE: SubStatusCode = SubStatusCode(1029);

    /// Redundant collection PUT (1009).
    pub const REDUNDANT_COLLECTION_PUT: SubStatusCode = SubStatusCode(1009);

    /// Shared throughput offer grow not needed (1011).
    pub const SHARED_THROUGHPUT_OFFER_GROW_NOT_NEEDED: SubStatusCode = SubStatusCode(1011);

    /// Compute federation not found (1012).
    /// Note: Also used for 503 Service Unavailable.
    pub const COMPUTE_FEDERATION_NOT_FOUND: SubStatusCode = SubStatusCode(1012);

    /// Partition key quota over limit (1014).
    pub const PARTITION_KEY_QUOTA_OVER_LIMIT: SubStatusCode = SubStatusCode(1014);

    /// Shared throughput database collection count exceeded (1019).
    pub const SHARED_THROUGHPUT_DATABASE_COLLECTION_COUNT_EXCEEDED: SubStatusCode =
        SubStatusCode(1019);

    /// Shared throughput database count exceeded (1020).
    pub const SHARED_THROUGHPUT_DATABASE_COUNT_EXCEEDED: SubStatusCode = SubStatusCode(1020);

    /// Compute internal error (1021).
    pub const COMPUTE_INTERNAL_ERROR: SubStatusCode = SubStatusCode(1021);

    // ----- 404: Not Found -----

    /// Read session not available (1002).
    /// Note: Same value as `PARTITION_KEY_RANGE_GONE` for 410.
    pub const READ_SESSION_NOT_AVAILABLE: SubStatusCode = SubStatusCode(1002);

    /// Owner resource not found (1003).
    pub const OWNER_RESOURCE_NOT_FOUND: SubStatusCode = SubStatusCode(1003);

    /// Collection create in progress (1013).
    pub const COLLECTION_CREATE_IN_PROGRESS: SubStatusCode = SubStatusCode(1013);

    /// Store not ready (1023).
    pub const STORE_NOT_READY: SubStatusCode = SubStatusCode(1023);

    /// Auth token not found in cache (1030).
    pub const AUTH_TOKEN_NOT_FOUND_IN_CACHE: SubStatusCode = SubStatusCode(1030);

    /// Archival partition not present (1024).
    pub const ARCHIVAL_PARTITION_NOT_PRESENT: SubStatusCode = SubStatusCode(1024);

    /// Partition migrating, collection deleted (1031).
    pub const PARTITION_MIGRATING_COLLECTION_DELETED: SubStatusCode = SubStatusCode(1031);

    /// Partition migration source partition deleted in master (1034).
    pub const PARTITION_MIGRATION_SOURCE_PARTITION_DELETED_IN_MASTER: SubStatusCode =
        SubStatusCode(1034);

    /// Partition migration shared throughput database partition resource not found in master (1035).
    pub const PARTITION_MIGRATION_SHARED_THROUGHPUT_DB_PARTITION_NOT_FOUND: SubStatusCode =
        SubStatusCode(1035);

    /// Partition migration partition resource not found in master (1036).
    pub const PARTITION_MIGRATION_PARTITION_RESOURCE_NOT_FOUND: SubStatusCode = SubStatusCode(1036);

    /// Partition migration failed to update DNS (1037).
    pub const PARTITION_MIGRATION_FAILED_TO_UPDATE_DNS: SubStatusCode = SubStatusCode(1037);

    // ----- 409: Conflict -----

    /// Conflict with control plane (1006).
    pub const CONFLICT_WITH_CONTROL_PLANE: SubStatusCode = SubStatusCode(1006);

    /// Database name already exists (3206).
    pub const DATABASE_NAME_EXISTS: SubStatusCode = SubStatusCode(3206);

    /// Partition key hash collision for ID (3302).
    pub const PARTITION_KEY_HASH_COLLISION: SubStatusCode = SubStatusCode(3302);

    /// Partition migration document count mismatch between source and target (3050).
    pub const PARTITION_MIGRATION_DOC_COUNT_MISMATCH_SOURCE_TARGET: SubStatusCode =
        SubStatusCode(3050);

    /// Partition migration document count mismatch between target partition replicas (3051).
    pub const PARTITION_MIGRATION_DOC_COUNT_MISMATCH_TARGET_REPLICAS: SubStatusCode =
        SubStatusCode(3051);

    // ----- 410: Gone -----

    /// Name cache is stale (1000).
    pub const NAME_CACHE_STALE: SubStatusCode = SubStatusCode(1000);

    /// Partition key range gone (1002).
    /// Note: Same value as `READ_SESSION_NOT_AVAILABLE` for 404.
    pub const PARTITION_KEY_RANGE_GONE: SubStatusCode = SubStatusCode(1002);

    /// Completing split or merge (1007).
    /// Note: Same value as `INSUFFICIENT_BINDABLE_PARTITIONS` for 503.
    pub const COMPLETING_SPLIT: SubStatusCode = SubStatusCode(1007);

    /// Completing partition migration (1008).
    /// Note: Same value as `DATABASE_ACCOUNT_NOT_FOUND` for 403.
    pub const COMPLETING_PARTITION_MIGRATION: SubStatusCode = SubStatusCode(1008);

    /// Lease not found (1022).
    pub const LEASE_NOT_FOUND: SubStatusCode = SubStatusCode(1022);

    // ----- 412: Precondition Failed -----

    /// Split is disabled (2001).
    pub const SPLIT_DISABLED: SubStatusCode = SubStatusCode(2001);

    /// Resource not found during precondition check (2004).
    pub const RESOURCE_NOT_FOUND: SubStatusCode = SubStatusCode(2004);

    /// Tombstone records not found (purged) (2015).
    pub const TOMBSTONE_RECORDS_NOT_FOUND: SubStatusCode = SubStatusCode(2015);

    /// Collections in partition got updated (2002).
    pub const COLLECTIONS_IN_PARTITION_GOT_UPDATED: SubStatusCode = SubStatusCode(2002);

    /// Cannot acquire partition key ranges lock (2003).
    pub const CANNOT_ACQUIRE_PKRANGES_LOCK: SubStatusCode = SubStatusCode(2003);

    /// Cannot acquire offer owner lock (2005).
    pub const CANNOT_ACQUIRE_OFFER_OWNER_LOCK: SubStatusCode = SubStatusCode(2005);

    /// Cannot acquire partition key range lock (2007).
    pub const CANNOT_ACQUIRE_PKRANGE_LOCK: SubStatusCode = SubStatusCode(2007);

    /// Cannot acquire partition lock (2008).
    pub const CANNOT_ACQUIRE_PARTITION_LOCK: SubStatusCode = SubStatusCode(2008);

    /// Storage split conflicting with n-way throughput split (2011).
    pub const STORAGE_SPLIT_CONFLICTING_WITH_NWAY_THROUGHPUT_SPLIT: SubStatusCode =
        SubStatusCode(2011);

    /// Merge is disabled (2012).
    pub const MERGE_DISABLED: SubStatusCode = SubStatusCode(2012);

    /// Invalid account status (2016).
    pub const INVALID_ACCOUNT_STATUS: SubStatusCode = SubStatusCode(2016);

    /// Offer validation failed (2017).
    pub const OFFER_VALIDATION_FAILED: SubStatusCode = SubStatusCode(2017);

    /// Cannot acquire master partition access lock (2018).
    pub const CANNOT_ACQUIRE_MASTER_PARTITION_ACCESS_LOCK: SubStatusCode = SubStatusCode(2018);

    /// Cannot acquire in-account restore in progress lock (2019).
    pub const CANNOT_ACQUIRE_IN_ACCOUNT_RESTORE_LOCK: SubStatusCode = SubStatusCode(2019);

    /// Collection state changed (2020).
    pub const COLLECTION_STATE_CHANGED: SubStatusCode = SubStatusCode(2020);

    /// Offer scaled up by user (2021).
    pub const OFFER_SCALED_UP_BY_USER: SubStatusCode = SubStatusCode(2021);

    /// Cannot acquire log store storage account load balance lock (2101).
    pub const CANNOT_ACQUIRE_LOG_STORE_LOAD_BALANCE_LOCK: SubStatusCode = SubStatusCode(2101);

    // ----- 429: Too Many Requests -----

    /// RU budget exceeded (3200).
    pub const RU_BUDGET_EXCEEDED: SubStatusCode = SubStatusCode(3200);

    /// Gateway throttled (3201).
    pub const GATEWAY_THROTTLED: SubStatusCode = SubStatusCode(3201);

    /// Prepare time limit exceeded (3207).
    pub const PREPARE_TIME_EXCEEDED: SubStatusCode = SubStatusCode(3207);

    /// Client TCP channel full (3208).
    pub const CLIENT_TCP_CHANNEL_FULL: SubStatusCode = SubStatusCode(3208);

    /// Stored procedure concurrency limit (3084).
    pub const STORED_PROCEDURE_CONCURRENCY: SubStatusCode = SubStatusCode(3084);

    /// Throttle due to split (3088).
    pub const THROTTLE_DUE_TO_SPLIT: SubStatusCode = SubStatusCode(3088);

    /// System resource unavailable (3092).
    pub const SYSTEM_RESOURCE_UNAVAILABLE: SubStatusCode = SubStatusCode(3092);

    /// BW term count limit exceeded (3209).
    pub const BW_TERM_COUNT_LIMIT_EXCEEDED: SubStatusCode = SubStatusCode(3209);

    // ----- 500: Internal Server Error -----

    /// Invalid account configuration (3003).
    pub const INVALID_ACCOUNT_CONFIGURATION: SubStatusCode = SubStatusCode(3003);

    /// Configuration name not empty (3001).
    pub const CONFIGURATION_NAME_NOT_EMPTY: SubStatusCode = SubStatusCode(3001);

    /// Configuration operation cancelled (3002).
    pub const CONFIGURATION_OPERATION_CANCELLED: SubStatusCode = SubStatusCode(3002);

    /// Federation does not exist or is locked (3004).
    pub const FEDERATION_DOES_NOT_EXIST_OR_IS_LOCKED: SubStatusCode = SubStatusCode(3004);

    /// Partition failover error code (3010).
    pub const PARTITION_FAILOVER_ERROR_CODE: SubStatusCode = SubStatusCode(3010);

    // ----- 503: Service Unavailable -----

    /// Operation paused (9001).
    pub const OPERATION_PAUSED: SubStatusCode = SubStatusCode(9001);

    /// Insufficient capacity (9003).
    pub const INSUFFICIENT_CAPACITY: SubStatusCode = SubStatusCode(9003);

    /// Insufficient bindable partitions (1007).
    /// Note: Same value as `COMPLETING_SPLIT` for 410.
    pub const INSUFFICIENT_BINDABLE_PARTITIONS: SubStatusCode = SubStatusCode(1007);

    /// Service is offline (9002).
    pub const SERVICE_IS_OFFLINE: SubStatusCode = SubStatusCode(9002);

    // ----- SDK Client-side codes (10xxx, 2xxxx) -----

    /// Gateway endpoint unavailable (10001).
    pub const GATEWAY_ENDPOINT_UNAVAILABLE: SubStatusCode = SubStatusCode(10001);

    /// Gateway endpoint read timeout (10002).
    pub const GATEWAY_ENDPOINT_READ_TIMEOUT: SubStatusCode = SubStatusCode(10002);

    /// Throughput control request rate too large (10003).
    pub const THROUGHPUT_CONTROL_REQUEST_RATE_TOO_LARGE: SubStatusCode = SubStatusCode(10003);

    /// Offer not configured (10004).
    pub const OFFER_NOT_CONFIGURED: SubStatusCode = SubStatusCode(10004);

    /// Transport generated 410 (20001).
    pub const TRANSPORT_GENERATED_410: SubStatusCode = SubStatusCode(20001);

    /// Timeout generated 410 (20002).
    pub const TIMEOUT_GENERATED_410: SubStatusCode = SubStatusCode(20002);

    /// Transport generated 503 (20003).
    pub const TRANSPORT_GENERATED_503: SubStatusCode = SubStatusCode(20003);

    /// Client CPU overload (20004).
    pub const CLIENT_CPU_OVERLOAD: SubStatusCode = SubStatusCode(20004);

    /// Client thread starvation (20005).
    pub const CLIENT_THREAD_STARVATION: SubStatusCode = SubStatusCode(20005);

    /// Channel closed (20006).
    pub const CHANNEL_CLOSED: SubStatusCode = SubStatusCode(20006);

    /// Malformed continuation token (20007).
    pub const MALFORMED_CONTINUATION_TOKEN: SubStatusCode = SubStatusCode(20007);

    /// Client operation timeout (20008).
    pub const CLIENT_OPERATION_TIMEOUT: SubStatusCode = SubStatusCode(20008);

    /// Transit timeout (20911).
    pub const TRANSIT_TIMEOUT: SubStatusCode = SubStatusCode(20911);

    /// Closed client (20912).
    pub const CLOSED_CLIENT: SubStatusCode = SubStatusCode(20912);

    // ----- SDK Server-side codes (21xxx) -----

    /// Name cache stale exceeded retry limit (21001).
    pub const NAME_CACHE_STALE_EXCEEDED_RETRY_LIMIT: SubStatusCode = SubStatusCode(21001);

    /// Partition key range gone exceeded retry limit (21002).
    pub const PARTITION_KEY_RANGE_GONE_EXCEEDED_RETRY_LIMIT: SubStatusCode = SubStatusCode(21002);

    /// Completing split exceeded retry limit (21003).
    pub const COMPLETING_SPLIT_EXCEEDED_RETRY_LIMIT: SubStatusCode = SubStatusCode(21003);

    /// Completing partition migration exceeded retry limit (21004).
    pub const COMPLETING_PARTITION_MIGRATION_EXCEEDED_RETRY_LIMIT: SubStatusCode =
        SubStatusCode(21004);

    /// Server generated 410 (21005).
    pub const SERVER_GENERATED_410: SubStatusCode = SubStatusCode(21005);

    /// Global strong write barrier not met (21006).
    pub const GLOBAL_STRONG_WRITE_BARRIER_NOT_MET: SubStatusCode = SubStatusCode(21006);

    /// Read quorum not met (21007).
    pub const READ_QUORUM_NOT_MET: SubStatusCode = SubStatusCode(21007);

    /// Server generated 503 (21008).
    pub const SERVER_GENERATED_503: SubStatusCode = SubStatusCode(21008);

    /// No valid store response (21009).
    pub const NO_VALID_STORE_RESPONSE: SubStatusCode = SubStatusCode(21009);

    /// Server generated 408 (21010).
    pub const SERVER_GENERATED_408: SubStatusCode = SubStatusCode(21010);

    /// Server barrier throttled (21011).
    pub const SERVER_BARRIER_THROTTLED: SubStatusCode = SubStatusCode(21011);

    // ----- AAD/Auth codes (5xxx) -----

    /// AAD token expired (5006).
    pub const AAD_TOKEN_EXPIRED: SubStatusCode = SubStatusCode(5006);

    /// Local auth disabled (5202).
    pub const LOCAL_AUTH_DISABLED: SubStatusCode = SubStatusCode(5202);

    /// RBAC request was not authorized (5400).
    pub const RBAC_REQUEST_NOT_AUTHORIZED: SubStatusCode = SubStatusCode(5400);
}

impl Default for SubStatusCode {
    fn default() -> Self {
        Self::UNKNOWN
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

impl From<u32> for SubStatusCode {
    fn from(value: u32) -> Self {
        SubStatusCode(value)
    }
}

impl From<SubStatusCode> for u32 {
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
    status_code: StatusCode,
    sub_status: Option<SubStatusCode>,
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
    pub fn with_sub_status(mut self, sub_status_code: u32) -> Self {
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

    /// Returns `true` if the HTTP status indicates success (2xx).
    pub fn is_success(&self) -> bool {
        self.status_code.is_success()
    }

    /// Returns `true` if this is a throttling response (HTTP 429).
    pub fn is_throttled(&self) -> bool {
        u16::from(self.status_code) == 429
    }

    /// Returns `true` if this is an HTTP 410 Gone response.
    pub fn is_gone(&self) -> bool {
        u16::from(self.status_code) == 410
    }

    /// Returns `true` if this is an HTTP 404 Not Found response.
    pub fn is_not_found(&self) -> bool {
        u16::from(self.status_code) == 404
    }

    /// Returns `true` if this is a write-forbidden error (HTTP 403, sub-status 3).
    pub fn is_write_forbidden(&self) -> bool {
        u16::from(self.status_code) == 403
            && self.sub_status == Some(SubStatusCode::WRITE_FORBIDDEN)
    }

    /// Returns `true` if this is a read-session-not-available error (HTTP 404, sub-status 1002).
    pub fn is_read_session_not_available(&self) -> bool {
        u16::from(self.status_code) == 404
            && self.sub_status == Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE)
    }

    /// Returns `true` if this is a partition-key-range-gone error (HTTP 410, sub-status 1002).
    pub fn is_partition_key_range_gone(&self) -> bool {
        u16::from(self.status_code) == 410
            && self.sub_status == Some(SubStatusCode::PARTITION_KEY_RANGE_GONE)
    }

    /// Returns `true` if this indicates a transport-generated 503 (client-side).
    pub fn is_transport_generated_503(&self) -> bool {
        u16::from(self.status_code) == 503
            && self.sub_status == Some(SubStatusCode::TRANSPORT_GENERATED_503)
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

    // =========================================================================
    // Well-Known CosmosStatus Constants
    // =========================================================================

    // ----- Transport / Client-Side -----

    /// Transport-generated 503 Service Unavailable (sub-status 20003).
    ///
    /// Generated by the SDK when a transport-level error occurs (connection failure,
    /// DNS error, TLS error, etc.) and no HTTP response was received.
    pub const TRANSPORT_GENERATED_503: CosmosStatus = CosmosStatus {
        status_code: StatusCode::ServiceUnavailable,
        sub_status: Some(SubStatusCode::TRANSPORT_GENERATED_503),
    };

    // ----- 404: Not Found -----

    /// Read session not available (HTTP 404, sub-status 1002).
    ///
    /// Session consistency read could not be satisfied because the target
    /// replica has not yet received the required session token.
    pub const READ_SESSION_NOT_AVAILABLE: CosmosStatus = CosmosStatus {
        status_code: StatusCode::NotFound,
        sub_status: Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE),
    };

    // ----- 403: Forbidden -----

    /// Write forbidden (HTTP 403, sub-status 3).
    ///
    /// The region does not allow write operations (read-only region).
    pub const WRITE_FORBIDDEN: CosmosStatus = CosmosStatus {
        status_code: StatusCode::Forbidden,
        sub_status: Some(SubStatusCode::WRITE_FORBIDDEN),
    };

    // ----- 410: Gone -----

    /// Partition key range gone (HTTP 410, sub-status 1002).
    ///
    /// The partition key range has been split or merged. The client must
    /// refresh its partition key range cache and retry.
    pub const PARTITION_KEY_RANGE_GONE: CosmosStatus = CosmosStatus {
        status_code: StatusCode::Gone,
        sub_status: Some(SubStatusCode::PARTITION_KEY_RANGE_GONE),
    };

    /// Name cache stale (HTTP 410, sub-status 1000).
    pub const NAME_CACHE_STALE: CosmosStatus = CosmosStatus {
        status_code: StatusCode::Gone,
        sub_status: Some(SubStatusCode::NAME_CACHE_STALE),
    };

    /// Completing split or merge (HTTP 410, sub-status 1007).
    pub const COMPLETING_SPLIT: CosmosStatus = CosmosStatus {
        status_code: StatusCode::Gone,
        sub_status: Some(SubStatusCode::COMPLETING_SPLIT),
    };

    /// Completing partition migration (HTTP 410, sub-status 1008).
    pub const COMPLETING_PARTITION_MIGRATION: CosmosStatus = CosmosStatus {
        status_code: StatusCode::Gone,
        sub_status: Some(SubStatusCode::COMPLETING_PARTITION_MIGRATION),
    };

    // ----- 429: Too Many Requests -----

    /// RU budget exceeded (HTTP 429, sub-status 3200).
    pub const RU_BUDGET_EXCEEDED: CosmosStatus = CosmosStatus {
        status_code: StatusCode::TooManyRequests,
        sub_status: Some(SubStatusCode::RU_BUDGET_EXCEEDED),
    };
}

impl fmt::Debug for CosmosStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_u16: u16 = self.status_code.into();
        match (self.sub_status, self.name()) {
            (Some(sub), Some(name)) => {
                write!(f, "CosmosStatus({}/{} {})", status_u16, sub.value(), name)
            }
            (Some(sub), None) => write!(f, "CosmosStatus({}/{})", status_u16, sub.value()),
            (None, _) => write!(f, "CosmosStatus({})", status_u16),
        }
    }
}

impl fmt::Display for CosmosStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_u16: u16 = self.status_code.into();
        match (self.sub_status, self.name()) {
            (Some(sub), Some(name)) => write!(f, "{}/{} ({})", status_u16, sub.value(), name),
            (Some(sub), None) => write!(f, "{}/{}", status_u16, sub.value()),
            (None, _) => write!(f, "{}", status_u16),
        }
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

impl<'de> Deserialize<'de> for CosmosStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            status: Option<String>,
            status_code: Option<u16>,
            sub_status_code: Option<u32>,
        }
        let h = Helper::deserialize(deserializer)?;

        if let Some(status_code) = h.status_code {
            return Ok(CosmosStatus {
                status_code: StatusCode::from(status_code),
                sub_status: h.sub_status_code.map(SubStatusCode::new),
            });
        }

        if let Some(status) = h.status {
            let normalized = status
                .split_once(' ')
                .map_or(status.as_str(), |(left, _)| left);
            if let Some((status_code, sub_status_code)) = normalized.split_once('/') {
                let status_code = status_code
                    .parse::<u16>()
                    .map_err(serde::de::Error::custom)?;
                let sub_status_code = sub_status_code
                    .parse::<u32>()
                    .map_err(serde::de::Error::custom)?;
                return Ok(CosmosStatus {
                    status_code: StatusCode::from(status_code),
                    sub_status: Some(SubStatusCode::new(sub_status_code)),
                });
            }

            let status_code = normalized
                .parse::<u16>()
                .map_err(serde::de::Error::custom)?;
            return Ok(CosmosStatus {
                status_code: StatusCode::from(status_code),
                sub_status: None,
            });
        }

        Err(serde::de::Error::custom(
            "CosmosStatus must include status or status_code",
        ))
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
    fn with_sub_status_unambiguous() {
        let status = CosmosStatus::new(StatusCode::TooManyRequests).with_sub_status(3200);
        assert_eq!(status.status_code(), StatusCode::TooManyRequests);
        assert_eq!(status.sub_status(), Some(SubStatusCode::RU_BUDGET_EXCEEDED));
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
        assert!(CosmosStatus::TRANSPORT_GENERATED_503.is_transport_generated_503());
        assert!(CosmosStatus::READ_SESSION_NOT_AVAILABLE.is_read_session_not_available());
        assert!(CosmosStatus::PARTITION_KEY_RANGE_GONE.is_partition_key_range_gone());
        assert!(CosmosStatus::WRITE_FORBIDDEN.is_write_forbidden());
        assert!(CosmosStatus::RU_BUDGET_EXCEEDED.is_throttled());
    }

    #[test]
    fn is_success() {
        assert!(CosmosStatus::new(StatusCode::Ok).is_success());
        assert!(CosmosStatus::new(StatusCode::Created).is_success());
        assert!(!CosmosStatus::new(StatusCode::NotFound).is_success());
        assert!(!CosmosStatus::RU_BUDGET_EXCEEDED.is_success());
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
        let status = CosmosStatus::new(StatusCode::Ok).with_sub_status(99999);
        assert_eq!(format!("{}", status), "200/99999");
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
            CosmosStatus::READ_SESSION_NOT_AVAILABLE
        );
        assert_ne!(
            CosmosStatus::new(StatusCode::NotFound).with_sub_status(1002),
            CosmosStatus::new(StatusCode::Gone).with_sub_status(1002),
        );
    }

    #[test]
    fn serialization_roundtrip() {
        let status = CosmosStatus::new(StatusCode::TooManyRequests).with_sub_status(3200);
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"status\":\"429/3200 (RUBudgetExceeded)\""));

        let deserialized: CosmosStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, status);
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
    fn from_u32() {
        let code = SubStatusCode::from(3200u32);
        assert_eq!(code, SubStatusCode::RU_BUDGET_EXCEEDED);
    }

    #[test]
    fn into_u32() {
        let value: u32 = SubStatusCode::RU_BUDGET_EXCEEDED.into();
        assert_eq!(value, 3200);
    }

    #[test]
    fn display_known_code() {
        let code = SubStatusCode::RU_BUDGET_EXCEEDED;
        assert_eq!(format!("{}", code), "RUBudgetExceeded (3200)");
    }

    #[test]
    fn display_unknown_code() {
        let code = SubStatusCode::new(99999);
        assert_eq!(format!("{}", code), "99999");
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
        let code = SubStatusCode::RU_BUDGET_EXCEEDED;
        assert_eq!(
            format!("{:?}", code),
            "SubStatusCode::RUBudgetExceeded(3200)"
        );
    }

    #[test]
    fn debug_unknown_code() {
        let code = SubStatusCode::new(99999);
        assert_eq!(format!("{:?}", code), "SubStatusCode(99999)");
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
            SubStatusCode::PARTITION_KEY_RANGE_GONE
        );
        assert_eq!(
            SubStatusCode::new(1002),
            SubStatusCode::READ_SESSION_NOT_AVAILABLE
        );
        assert_ne!(SubStatusCode::new(1002), SubStatusCode::NAME_CACHE_STALE);
    }

    #[test]
    fn default_is_unknown() {
        assert_eq!(SubStatusCode::default(), SubStatusCode::UNKNOWN);
        assert_eq!(SubStatusCode::default().value(), 0);
    }

    #[test]
    fn name_returns_some_for_unambiguous() {
        assert_eq!(
            SubStatusCode::RU_BUDGET_EXCEEDED.name(None),
            Some("RUBudgetExceeded")
        );
    }

    #[test]
    fn name_returns_none_for_unknown() {
        assert_eq!(SubStatusCode::new(99999).name(None), None);
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
        assert_eq!(SubStatusCode::TRANSPORT_GENERATED_410.value(), 20001);
        assert_eq!(SubStatusCode::TIMEOUT_GENERATED_410.value(), 20002);
        assert_eq!(SubStatusCode::TRANSPORT_GENERATED_503.value(), 20003);
        assert_eq!(SubStatusCode::CLIENT_CPU_OVERLOAD.value(), 20004);
        assert_eq!(SubStatusCode::CLIENT_THREAD_STARVATION.value(), 20005);
        assert_eq!(SubStatusCode::CLIENT_OPERATION_TIMEOUT.value(), 20008);
    }

    #[test]
    fn sdk_server_codes() {
        // Verify SDK server-side codes match Java/.NET
        assert_eq!(
            SubStatusCode::NAME_CACHE_STALE_EXCEEDED_RETRY_LIMIT.value(),
            21001
        );
        assert_eq!(
            SubStatusCode::PARTITION_KEY_RANGE_GONE_EXCEEDED_RETRY_LIMIT.value(),
            21002
        );
        assert_eq!(SubStatusCode::SERVER_GENERATED_410.value(), 21005);
        assert_eq!(
            SubStatusCode::GLOBAL_STRONG_WRITE_BARRIER_NOT_MET.value(),
            21006
        );
        assert_eq!(SubStatusCode::READ_QUORUM_NOT_MET.value(), 21007);
        assert_eq!(SubStatusCode::SERVER_GENERATED_503.value(), 21008);
    }
}
