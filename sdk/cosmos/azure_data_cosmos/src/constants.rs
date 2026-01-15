// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Don't spell-check header names (which should start with 'x-').
// cSpell:ignoreRegExp /x-[^\s]+/

//! Constants defining HTTP headers and other values relevant to Azure Cosmos DB APIs.

use azure_core::http::{
    headers::{HeaderName, HeaderValue},
    request::options::ContentType,
};

pub const QUERY: HeaderName = HeaderName::from_static("x-ms-documentdb-query");
pub const PARTITION_KEY: HeaderName = HeaderName::from_static("x-ms-documentdb-partitionkey");
pub const PARTITION_KEY_RANGE_ID: HeaderName =
    HeaderName::from_static("x-ms-documentdb-partitionkeyrangeid");
pub const QUERY_ENABLE_CROSS_PARTITION: HeaderName =
    HeaderName::from_static("x-ms-documentdb-query-enablecrosspartition");
pub const IS_QUERY_PLAN_REQUEST: HeaderName =
    HeaderName::from_static("x-ms-cosmos-is-query-plan-request");
pub const SUPPORTED_QUERY_FEATURES: HeaderName =
    HeaderName::from_static("x-ms-cosmos-supported-query-features");
pub const CONTINUATION: HeaderName = HeaderName::from_static("x-ms-continuation");
pub const INDEX_METRICS: HeaderName = HeaderName::from_static("x-ms-cosmos-index-utilization");
pub const QUERY_METRICS: HeaderName = HeaderName::from_static("x-ms-documentdb-query-metrics");
pub const IS_UPSERT: HeaderName = HeaderName::from_static("x-ms-documentdb-is-upsert");
pub const OFFER_THROUGHPUT: HeaderName = HeaderName::from_static("x-ms-offer-throughput");
pub const OFFER_AUTOPILOT_SETTINGS: HeaderName =
    HeaderName::from_static("x-ms-cosmos-offer-autopilot-settings");
pub const CONSISTENCY_LEVEL: HeaderName = HeaderName::from_static("x-ms-consistency-level");
pub const PRE_TRIGGER_INCLUDE: HeaderName =
    HeaderName::from_static("x-ms-documentdb-pre-trigger-include");
pub const POST_TRIGGER_INCLUDE: HeaderName =
    HeaderName::from_static("x-ms-documentdb-post-trigger-include");
pub const SESSION_TOKEN: HeaderName = HeaderName::from_static("x-ms-session-token");
pub const INDEXING_DIRECTIVE: HeaderName = HeaderName::from_static("x-ms-indexing-directive");
pub const SUB_STATUS: HeaderName = HeaderName::from_static("x-ms-substatus");
pub const THROUGHPUT_BUCKET: HeaderName = HeaderName::from_static("x-ms-cosmos-throughput-bucket");
pub const PRIORITY_LEVEL: HeaderName = HeaderName::from_static("x-ms-cosmos-priority-level");
// Used to allow tentative writes in multi-write accounts.
pub const ALLOW_TENTATIVE_WRITES: HeaderName =
    HeaderName::from_static("x-ms-cosmos-allow-tentative-writes");

pub const QUERY_CONTENT_TYPE: ContentType = ContentType::from_static("application/query+json");

pub(crate) const PREFER_MINIMAL: HeaderValue = HeaderValue::from_static("return=minimal");

pub const ACCOUNT_PROPERTIES_KEY: &str = "account_properties_key";

// Unauthorized headers that should never be included are `authorization`, `proxy-authorization`
// The comments indicate headers that are already included in the default allow list by azure-core
pub const COSMOS_ALLOWED_HEADERS: &[&str] = &[
    // Standard HTTP Headers
    // "etag", // Already in default list
    "x-http-method",
    "slug",
    // "content-type", // Already in default list
    // "last-modified", // Already in default list
    "content-encoding",
    "characterset",
    // "user-agent", // Already in default list
    // "if-modified-since", // Already in default list
    // "if-match", // Already in default list
    // "if-none-match", // Already in default list
    // "content-length", // Already in default list
    "accept-encoding",
    "keep-alive",
    // "cache-control", // Already in default list
    // "transfer-encoding", // Already in default list
    "content-language",
    "content-location",
    "content-md5",
    "content-range",
    // "accept", // Already in default list
    "accept-charset",
    "accept-language",
    "if-range",
    // "if-unmodified-since", // Already in default list
    "max-forwards",
    "accept-ranges",
    "proxy-authenticate",
    // "retry-after", // Already in default list
    "set-cookie",
    // "www-authenticate", // Already in default list
    "origin",
    "host",
    "access-control-allow-origin",
    "access-control-allow-headers",
    // "date", // Already in default list
    "prefer",
    "location",
    "referer",
    // "pragma", // Already in default list
    // "server", // Already in default list
    "strict-transport-security",
    // Bulk/Batch
    "x-ms-cosmos-is-batch-request",
    "x-ms-cosmos-batch-atomic",
    "x-ms-cosmos-batch-continue-on-error",
    // Query
    "x-ms-documentdb-query",
    "x-ms-documentdb-isquery",
    "x-ms-cosmos-is-query-plan-request",
    "x-ms-cosmos-supported-query-features",
    "x-ms-cosmos-query-version",
    "x-ms-documentdb-query-metrics",
    "x-ms-cosmos-query-execution-info",
    "x-ms-cosmos-index-utilization",
    // DocDB headers
    "x-ms-continuation",
    "x-ms-max-item-count",
    "x-ms-documentdb-responsecontinuationtokenlimitinkb",
    "x-ms-cosmos-priority-level",
    "x-ms-activity-id",
    "x-ms-cosmos-correlated-activityid",
    "x-ms-documentdb-pre-trigger-include",
    "x-ms-documentdb-pre-trigger-exclude",
    "x-ms-documentdb-post-trigger-include",
    "x-ms-documentdb-post-trigger-exclude",
    "x-ms-indexing-directive",
    "x-ms-session-token",
    "x-ms-consistency-level",
    "x-ms-date",
    "x-ms-collection-partition-info",
    "x-ms-collection-service-info",
    "x-ms-retry-after-ms",
    "x-ms-is-feed-unfiltered",
    "x-ms-documentdb-expiry-seconds",
    "x-ms-documentdb-query-enable-scan",
    "x-ms-documentdb-query-emit-traces",
    "x-ms-substatus",
    "x-ms-alt-content-path",
    "x-ms-content-path",
    "x-ms-documentdb-query-iscontinuationexpected",
    "x-ms-documentdb-populatequerymetrics",
    "x-ms-cosmos-populateindexmetrics",
    "x-ms-resource-quota",
    "x-ms-resource-usage",
    "x-ms-cosmos-intended-collection-rid",
    // Quota Info
    "x-ms-root-entity-max-count",
    "x-ms-root-entity-current-count",
    "x-ms-collection-quota-mb",
    "x-ms-collection-usage-mb",
    "x-ms-cosmos-max-content-length",
    "x-ms-max-media-storage-usage-mb",
    "x-ms-databaseaccount-consumed-mb",
    "x-ms-databaseaccount-provisioned-mb",
    // Collection quota
    "x-ms-documentdb-populatequotainfo",
    "x-ms-documentdb-populatepartitionstatistics",
    "collection-partition-index",
    "collection-service-index",
    // Usage Info
    "x-ms-media-storage-usage-mb",
    "x-ms-request-charge",
    // Address related headers
    "x-ms-force-refresh",
    "x-ms-item-count",
    "x-ms-new-resource-id",
    "x-ms-use-master-collection-resolver",
    // Admin Headers
    "x-ms-force-full-upgrade",
    "x-ms-only-upgrade-system-applications",
    "x-ms-only-upgrade-non-system-applications",
    "x-ms-upgrade-fabric-code-config",
    "x-ms-ignore-inprogress-upgrade",
    "x-ms-upgrade-verification-kind",
    "x-ms-iscanary",
    // Version
    "x-ms-version",
    // RDFE
    "ocp-resourceprovider-registered-uri",
    // Request management
    // "x-ms-request-id", // Already in default list
    // State change
    "x-ms-last-state-change-utc",
    // Offer
    "x-ms-offer-type",
    "x-ms-offer-throughput",
    "x-ms-cosmos-offer-autopilot-settings",
    // RU/minute
    "x-ms-documentdb-disable-ru-per-minute-usage",
    "x-ms-documentdb-is-ru-per-minute-used",
    "x-ms-offer-is-ru-per-minute-throughput-enabled",
    "x-ms-cosmos-throughput-bucket",
    // Partitioned collections
    "x-ms-documentdb-partitionkey",
    "x-ms-documentdb-query-enablecrosspartition",
    "x-ms-documentdb-partitionkeyrangeid",
    "x-ms-cosmos-physical-partition-id",
    "x-ms-cosmos-is-partition-key-delete-pending",
    "x-ms-start-epk",
    "x-ms-end-epk",
    "x-ms-read-key-type",
    "x-ms-cosmos-sdk-supportedcapabilities",
    // Upsert
    "x-ms-documentdb-is-upsert",
    // Index progress
    "x-ms-documentdb-collection-index-transformation-progress",
    "x-ms-documentdb-collection-lazy-indexing-progress",
    // Client retry
    "x-ms-throttle-retry-count",
    "x-ms-throttle-retry-wait-time-ms",
    // StoredProcedure
    "x-ms-documentdb-script-enable-logging",
    "x-ms-documentdb-script-log-results",
    // Change feed
    "a-im",
    "x-ms-cosmos-changefeed-wire-format-version",
    // Multiple Write Locations
    "x-ms-cosmos-allow-tentative-writes",
    // Dedicated Gateway
    "x-ms-dedicatedgateway-max-age",
    "x-ms-cosmos-cachehit",
    // Backend
    "lsn",
    "x-ms-schemaversion",
    "x-ms-gatewayversion",
    "x-ms-serviceversion",
    "x-ms-quorum-acked-lsn",
    "x-ms-current-write-quorum",
    "x-ms-current-replica-set-size",
    "x-ms-xp-role",
    "x-ms-global-committed-lsn",
    "x-ms-number-of-read-regions",
    "x-ms-transport-request-id",
    "x-ms-item-lsn",
    "x-ms-cosmos-item-llsn",
    "x-ms-cosmos-llsn",
    "x-ms-cosmos-quorum-acked-llsn",
    "x-ms-request-duration-ms",
    "x-ms-internal-partition-id",
    // Thin Client
    "x-ms-thinclient-proxy-operation-type",
    "x-ms-thinclient-proxy-resource-type",
    // Client ID
    "x-ms-client-id",
];

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum SubStatusCode {
    TooManyRequests = 429,

    // 400: Bad Request sub-status
    PartitionKeyMismatch = 1001,
    CrossPartitionQueryNotServable = 1004,
    ScriptCompileError = 0xFFFF, // From ExecuteStoredProcedure.
    AnotherOfferReplaceOperationIsInProgress = 3205,
    HttpListenerException = 1101,

    // 410: StatusCodeType_Gone: sub-status
    NameCacheIsStale = 1000,
    PartitionKeyRangeGone = 1002,
    CompletingSplit = 1007,
    CompletingPartitionMigration = 1008,
    LeaseNotFound = 1022,
    ArchivalPartitionNotPresent = 1024,

    // 404: LSN in session token is higher
    OwnerResourceNotFound = 1003,
    ConfigurationPropertyNotFound = 1005,
    CollectionCreateInProgress = 1013,
    StoreNotReady = 1023,
    AuthTokenNotFoundInCache = 1030,

    // 404: StatusCodeType_NotFound: sub-status
    PartitionMigratingCollectionDeleted = 1031,
    PartitionMigrationSourcePartitionDeletedInMaster = 1034,
    PartitionMigrationSharedThroughputDatabasePartitionResourceNotFoundInMaster = 1035,
    PartitionMigrationPartitionResourceNotFoundInMaster = 1036,
    PartitionMigrationFailedToUpdateDNS = 1037,

    // 403: Forbidden sub-status.
    WriteForbidden = 3,
    RedundantCollectionPut = 1009,
    SharedThroughputDatabaseQuotaExceeded = 1010,
    SharedThroughputOfferGrowNotNeeded = 1011,
    PartitionKeyQuotaOverLimit = 1014,
    SharedThroughputDatabaseCollectionCountExceeded = 1019,
    SharedThroughputDatabaseCountExceeded = 1020,
    ComputeInternalError = 1021,
    ThroughputCapQuotaExceeded = 1028,
    InvalidThroughputCapValue = 1029,

    // 409: Conflict exception
    ConflictWithControlPlane = 1006,
    DatabaseNameAlreadyExists = 3206,
    PartitionKeyHashCollisionForId = 3302,

    // 409: Partition migration Count mismatch conflict sub status codes
    PartitionMigrationDocumentCountMismatchBetweenSourceAndTargetPartition = 3050,
    PartitionMigrationDocumentCountMismatchBetweenTargetPartitionReplicas = 3051,

    // 503: Service Unavailable due to region being out of capacity for bindable partitions
    ComputeFederationNotFound = 1012,
    OperationPaused = 9001,
    ServiceIsOffline = 9002,
    InsufficientCapacity = 9003,

    //412: PreCondition Failed
    SplitIsDisabled = 2001,
    CollectionsInPartitionGotUpdated = 2002,
    CanNotAcquirePKRangesLock = 2003,
    ResourceNotFound = 2004,
    CanNotAcquireOfferOwnerLock = 2005,
    CanNotAcquirePKRangeLock = 2007,
    CanNotAcquirePartitionLock = 2008,
    StorageSplitConflictingWithNWayThroughputSplit = 2011,
    MergeIsDisabled = 2012,
    TombstoneRecordsNotFound = 2015, // Tombstone records were not found because they were purged.
    InvalidAccountStatus = 2016,
    OfferValidationFailed = 2017,
    CanNotAcquireMasterPartitionAccessLock = 2018,
    CanNotAcquireInAccountRestoreInProgressLock = 2019,
    CollectionStateChanged = 2020,
    OfferScaledUpByUser = 2021,
    CanNotAcquireLogStoreStorageAccountLoadBalanceLock = 2101,

    // 500: InternalServerError
    ConfigurationNameNotEmpty = 3001,
    ConfigurationOperationCancelled = 3002,
    InvalidAccountConfiguration = 3003,
    FederationDoesNotExistOrIsLocked = 3004,
    PartitionFailoverErrorCode = 3010,

    // 429: Request Rate Too Large
    PrepareTimeLimitExceeded = 3207,
    ClientTcpChannelFull = 3208,
    BWTermCountLimitExceeded = 3209,
    RUBudgetExceeded = 3200,
    GatewayThrottled = 3201,
    StoredProcedureConcurrency = 3084,
}

impl SubStatusCode {
    pub const CONFIGURATION_NAME_NOT_FOUND: SubStatusCode =
        SubStatusCode::CrossPartitionQueryNotServable;
    pub const READ_SESSION_NOT_AVAILABLE: SubStatusCode = SubStatusCode::PartitionKeyRangeGone;
    pub const PROVISION_LIMIT_REACHED: SubStatusCode = SubStatusCode::ConfigurationPropertyNotFound;
    pub const INSUFFICIENT_BINDABLE_PARTITIONS: SubStatusCode = SubStatusCode::CompletingSplit;
    pub const DATABASE_ACCOUNT_NOT_FOUND: SubStatusCode =
        SubStatusCode::CompletingPartitionMigration;

    /// Attempts to create a `SubStatusCode` from a header string.
    /// Returns `None` if parsing fails or code is unknown.
    pub fn from_header_value(s: &str) -> Option<Self> {
        let raw = s.trim();
        if let Ok(v) = raw.parse::<u32>() {
            SubStatusCode::try_from(v).ok()
        } else {
            None
        }
    }
}

impl TryFrom<u32> for SubStatusCode {
    type Error = (); // Unknown code

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        use SubStatusCode::*;
        let code = match value {
            3 => WriteForbidden,
            429 => TooManyRequests,
            1000 => NameCacheIsStale,
            1001 => PartitionKeyMismatch,
            1002 => PartitionKeyRangeGone,
            1003 => OwnerResourceNotFound,
            1004 => CrossPartitionQueryNotServable,
            1005 => ConfigurationPropertyNotFound,
            1006 => ConflictWithControlPlane,
            1007 => CompletingSplit,
            1008 => CompletingPartitionMigration,
            1009 => RedundantCollectionPut,
            1010 => SharedThroughputDatabaseQuotaExceeded,
            1011 => SharedThroughputOfferGrowNotNeeded,
            1012 => ComputeFederationNotFound,
            1013 => CollectionCreateInProgress,
            1014 => PartitionKeyQuotaOverLimit,
            1019 => SharedThroughputDatabaseCollectionCountExceeded,
            1020 => SharedThroughputDatabaseCountExceeded,
            1021 => ComputeInternalError,
            1022 => LeaseNotFound,
            1023 => StoreNotReady,
            1024 => ArchivalPartitionNotPresent,
            1028 => ThroughputCapQuotaExceeded,
            1029 => InvalidThroughputCapValue,
            1030 => AuthTokenNotFoundInCache,
            1031 => PartitionMigratingCollectionDeleted,
            1034 => PartitionMigrationSourcePartitionDeletedInMaster,
            1035 => PartitionMigrationSharedThroughputDatabasePartitionResourceNotFoundInMaster,
            1036 => PartitionMigrationPartitionResourceNotFoundInMaster,
            1037 => PartitionMigrationFailedToUpdateDNS,
            1101 => HttpListenerException,
            2001 => SplitIsDisabled,
            2002 => CollectionsInPartitionGotUpdated,
            2003 => CanNotAcquirePKRangesLock,
            2004 => ResourceNotFound,
            2005 => CanNotAcquireOfferOwnerLock,
            2007 => CanNotAcquirePKRangeLock,
            2008 => CanNotAcquirePartitionLock,
            2011 => StorageSplitConflictingWithNWayThroughputSplit,
            2012 => MergeIsDisabled,
            2015 => TombstoneRecordsNotFound,
            2016 => InvalidAccountStatus,
            2017 => OfferValidationFailed,
            2018 => CanNotAcquireMasterPartitionAccessLock,
            2019 => CanNotAcquireInAccountRestoreInProgressLock,
            2020 => CollectionStateChanged,
            2021 => OfferScaledUpByUser,
            2101 => CanNotAcquireLogStoreStorageAccountLoadBalanceLock,
            3001 => ConfigurationNameNotEmpty,
            3002 => ConfigurationOperationCancelled,
            3003 => InvalidAccountConfiguration,
            3004 => FederationDoesNotExistOrIsLocked,
            3010 => PartitionFailoverErrorCode,
            3050 => PartitionMigrationDocumentCountMismatchBetweenSourceAndTargetPartition,
            3051 => PartitionMigrationDocumentCountMismatchBetweenTargetPartitionReplicas,
            3084 => StoredProcedureConcurrency,
            3200 => RUBudgetExceeded,
            3201 => GatewayThrottled,
            3205 => AnotherOfferReplaceOperationIsInProgress,
            3206 => DatabaseNameAlreadyExists,
            3207 => PrepareTimeLimitExceeded,
            3208 => ClientTcpChannelFull,
            3209 => BWTermCountLimitExceeded,
            3302 => PartitionKeyHashCollisionForId,
            9001 => OperationPaused,
            9002 => ServiceIsOffline,
            9003 => InsufficientCapacity,
            0xFFFF => ScriptCompileError,
            _ => return Err(()),
        };
        Ok(code)
    }
}
