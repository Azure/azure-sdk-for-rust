// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Don't spell-check header names (which should start with 'x-').
// cSpell:disable

//! Constants defining HTTP headers and other values relevant to Azure Cosmos DB APIs.

use azure_core::http::{
    headers::{HeaderName, HeaderValue},
    request::options::ContentType,
};

/// Macro to define Cosmos DB header constants and the allowed headers list in one place.
macro_rules! cosmos_headers {
    ($($name:ident => $value:literal),* $(,)?) => {
        $(
            pub const $name: HeaderName = HeaderName::from_static($value);
        )*

        /// A list of all Cosmos DB specific headers that should be allowed in logging.
        pub const COSMOS_ALLOWED_HEADERS: &[&HeaderName] = &[
            $(&$name,)*
        ];
    };
}

cosmos_headers! {
    QUERY => "x-ms-documentdb-query",
    PARTITION_KEY => "x-ms-documentdb-partitionkey",
    PARTITION_KEY_RANGE_ID => "x-ms-documentdb-partitionkeyrangeid",
    QUERY_ENABLE_CROSS_PARTITION => "x-ms-documentdb-query-enablecrosspartition",
    IS_QUERY_PLAN_REQUEST => "x-ms-cosmos-is-query-plan-request",
    SUPPORTED_QUERY_FEATURES => "x-ms-cosmos-supported-query-features",
    CONTINUATION => "x-ms-continuation",
    INDEX_METRICS => "x-ms-cosmos-index-utilization",
    QUERY_METRICS => "x-ms-documentdb-query-metrics",
    IS_UPSERT => "x-ms-documentdb-is-upsert",
    OFFER_THROUGHPUT => "x-ms-offer-throughput",
    OFFER_AUTOPILOT_SETTINGS => "x-ms-cosmos-offer-autopilot-settings",
    CONSISTENCY_LEVEL => "x-ms-consistency-level",
    PRE_TRIGGER_INCLUDE => "x-ms-documentdb-pre-trigger-include",
    POST_TRIGGER_INCLUDE => "x-ms-documentdb-post-trigger-include",
    SESSION_TOKEN => "x-ms-session-token",
    INDEXING_DIRECTIVE => "x-ms-indexing-directive",
    SUB_STATUS => "x-ms-substatus",
    THROUGHPUT_BUCKET => "x-ms-cosmos-throughput-bucket",
    PRIORITY_LEVEL => "x-ms-cosmos-priority-level",
    ALLOW_TENTATIVE_WRITES => "x-ms-cosmos-allow-tentative-writes",
    // Standard HTTP Headers
    X_HTTP_METHOD => "x-http-method",
    SLUG => "slug",
    CONTENT_ENCODING => "content-encoding",
    CHARACTERSET => "characterset",
    ACCEPT_ENCODING => "accept-encoding",
    KEEP_ALIVE => "keep-alive",
    CONTENT_LANGUAGE => "content-language",
    CONTENT_LOCATION => "content-location",
    CONTENT_MD5 => "content-md5",
    CONTENT_RANGE => "content-range",
    ACCEPT_CHARSET => "accept-charset",
    ACCEPT_LANGUAGE => "accept-language",
    IF_RANGE => "if-range",
    MAX_FORWARDS => "max-forwards",
    ACCEPT_RANGES => "accept-ranges",
    PROXY_AUTHENTICATE => "proxy-authenticate",
    SET_COOKIE => "set-cookie",
    ORIGIN => "origin",
    HOST => "host",
    ACCESS_CONTROL_ALLOW_ORIGIN => "access-control-allow-origin",
    ACCESS_CONTROL_ALLOW_HEADERS => "access-control-allow-headers",
    PREFER => "prefer",
    LOCATION => "location",
    REFERER => "referer",
    STRICT_TRANSPORT_SECURITY => "strict-transport-security",
    // Bulk/Batch
    COSMOS_IS_BATCH_REQUEST => "x-ms-cosmos-is-batch-request",
    COSMOS_BATCH_ATOMIC => "x-ms-cosmos-batch-atomic",
    COSMOS_BATCH_CONTINUE_ON_ERROR => "x-ms-cosmos-batch-continue-on-error",
    // Query
    DOCUMENTDB_ISQUERY => "x-ms-documentdb-isquery",
    COSMOS_QUERY_VERSION => "x-ms-cosmos-query-version",
    COSMOS_QUERY_EXECUTION_INFO => "x-ms-cosmos-query-execution-info",
    // DocDB headers
    MAX_ITEM_COUNT => "x-ms-max-item-count",
    DOCUMENTDB_RESPONSECONTINUATIONTOKENLIMITINKB => "x-ms-documentdb-responsecontinuationtokenlimitinkb",
    ACTIVITY_ID => "x-ms-activity-id",
    COSMOS_CORRELATED_ACTIVITYID => "x-ms-cosmos-correlated-activityid",
    DOCUMENTDB_PRE_TRIGGER_EXCLUDE => "x-ms-documentdb-pre-trigger-exclude",
    DOCUMENTDB_POST_TRIGGER_EXCLUDE => "x-ms-documentdb-post-trigger-exclude",
    DATE => "x-ms-date",
    COLLECTION_PARTITION_INFO => "x-ms-collection-partition-info",
    COLLECTION_SERVICE_INFO => "x-ms-collection-service-info",
    RETRY_AFTER_MS => "x-ms-retry-after-ms",
    IS_FEED_UNFILTERED => "x-ms-is-feed-unfiltered",
    DOCUMENTDB_EXPIRY_SECONDS => "x-ms-documentdb-expiry-seconds",
    DOCUMENTDB_QUERY_ENABLE_SCAN => "x-ms-documentdb-query-enable-scan",
    DOCUMENTDB_QUERY_EMIT_TRACES => "x-ms-documentdb-query-emit-traces",
    ALT_CONTENT_PATH => "x-ms-alt-content-path",
    CONTENT_PATH => "x-ms-content-path",
    DOCUMENTDB_QUERY_ISCONTINUATIONEXPECTED => "x-ms-documentdb-query-iscontinuationexpected",
    DOCUMENTDB_POPULATEQUERYMETRICS => "x-ms-documentdb-populatequerymetrics",
    COSMOS_POPULATEINDEXMETRICS => "x-ms-cosmos-populateindexmetrics",
    RESOURCE_QUOTA => "x-ms-resource-quota",
    RESOURCE_USAGE => "x-ms-resource-usage",
    COSMOS_INTENDED_COLLECTION_RID => "x-ms-cosmos-intended-collection-rid",
    // Quota Info
    ROOT_ENTITY_MAX_COUNT => "x-ms-root-entity-max-count",
    ROOT_ENTITY_CURRENT_COUNT => "x-ms-root-entity-current-count",
    COLLECTION_QUOTA_MB => "x-ms-collection-quota-mb",
    COLLECTION_USAGE_MB => "x-ms-collection-usage-mb",
    COSMOS_MAX_CONTENT_LENGTH => "x-ms-cosmos-max-content-length",
    MAX_MEDIA_STORAGE_USAGE_MB => "x-ms-max-media-storage-usage-mb",
    DATABASEACCOUNT_CONSUMED_MB => "x-ms-databaseaccount-consumed-mb",
    DATABASEACCOUNT_PROVISIONED_MB => "x-ms-databaseaccount-provisioned-mb",
    // Collection quota
    DOCUMENTDB_POPULATEQUOTAINFO => "x-ms-documentdb-populatequotainfo",
    DOCUMENTDB_POPULATEPARTITIONSTATISTICS => "x-ms-documentdb-populatepartitionstatistics",
    COLLECTION_PARTITION_INDEX => "collection-partition-index",
    COLLECTION_SERVICE_INDEX => "collection-service-index",
    // Usage Info
    MEDIA_STORAGE_USAGE_MB => "x-ms-media-storage-usage-mb",
    REQUEST_CHARGE => "x-ms-request-charge",
    // Address related headers
    FORCE_REFRESH => "x-ms-force-refresh",
    ITEM_COUNT => "x-ms-item-count",
    NEW_RESOURCE_ID => "x-ms-new-resource-id",
    USE_MASTER_COLLECTION_RESOLVER => "x-ms-use-master-collection-resolver",
    // Admin Headers
    FORCE_FULL_UPGRADE => "x-ms-force-full-upgrade",
    ONLY_UPGRADE_SYSTEM_APPLICATIONS => "x-ms-only-upgrade-system-applications",
    ONLY_UPGRADE_NON_SYSTEM_APPLICATIONS => "x-ms-only-upgrade-non-system-applications",
    UPGRADE_FABRIC_CODE_CONFIG => "x-ms-upgrade-fabric-code-config",
    IGNORE_INPROGRESS_UPGRADE => "x-ms-ignore-inprogress-upgrade",
    UPGRADE_VERIFICATION_KIND => "x-ms-upgrade-verification-kind",
    ISCANARY => "x-ms-iscanary",
    // Version
    VERSION => "x-ms-version",
    // RDFE
    OCP_RESOURCEPROVIDER_REGISTERED_URI => "ocp-resourceprovider-registered-uri",
    // State change
    LAST_STATE_CHANGE_UTC => "x-ms-last-state-change-utc",
    // Offer
    OFFER_TYPE => "x-ms-offer-type",
    // RU/minute
    DOCUMENTDB_DISABLE_RU_PER_MINUTE_USAGE => "x-ms-documentdb-disable-ru-per-minute-usage",
    DOCUMENTDB_IS_RU_PER_MINUTE_USED => "x-ms-documentdb-is-ru-per-minute-used",
    OFFER_IS_RU_PER_MINUTE_THROUGHPUT_ENABLED => "x-ms-offer-is-ru-per-minute-throughput-enabled",
    // Partitioned collections
    COSMOS_PHYSICAL_PARTITION_ID => "x-ms-cosmos-physical-partition-id",
    COSMOS_IS_PARTITION_KEY_DELETE_PENDING => "x-ms-cosmos-is-partition-key-delete-pending",
    START_EPK => "x-ms-start-epk",
    END_EPK => "x-ms-end-epk",
    READ_KEY_TYPE => "x-ms-read-key-type",
    COSMOS_SDK_SUPPORTEDCAPABILITIES => "x-ms-cosmos-sdk-supportedcapabilities",
    // Index progress
    DOCUMENTDB_COLLECTION_INDEX_TRANSFORMATION_PROGRESS => "x-ms-documentdb-collection-index-transformation-progress",
    DOCUMENTDB_COLLECTION_LAZY_INDEXING_PROGRESS => "x-ms-documentdb-collection-lazy-indexing-progress",
    // Client retry
    THROTTLE_RETRY_COUNT => "x-ms-throttle-retry-count",
    THROTTLE_RETRY_WAIT_TIME_MS => "x-ms-throttle-retry-wait-time-ms",
    // StoredProcedure
    DOCUMENTDB_SCRIPT_ENABLE_LOGGING => "x-ms-documentdb-script-enable-logging",
    DOCUMENTDB_SCRIPT_LOG_RESULTS => "x-ms-documentdb-script-log-results",
    // Change feed
    A_IM => "a-im",
    COSMOS_CHANGEFEED_WIRE_FORMAT_VERSION => "x-ms-cosmos-changefeed-wire-format-version",
    // Dedicated Gateway
    DEDICATEDGATEWAY_MAX_AGE => "x-ms-dedicatedgateway-max-age",
    COSMOS_CACHEHIT => "x-ms-cosmos-cachehit",
    // Backend
    LSN => "lsn",
    SCHEMAVERSION => "x-ms-schemaversion",
    GATEWAYVERSION => "x-ms-gatewayversion",
    SERVICEVERSION => "x-ms-serviceversion",
    QUORUM_ACKED_LSN => "x-ms-quorum-acked-lsn",
    CURRENT_WRITE_QUORUM => "x-ms-current-write-quorum",
    CURRENT_REPLICA_SET_SIZE => "x-ms-current-replica-set-size",
    XP_ROLE => "x-ms-xp-role",
    GLOBAL_COMMITTED_LSN => "x-ms-global-committed-lsn",
    NUMBER_OF_READ_REGIONS => "x-ms-number-of-read-regions",
    TRANSPORT_REQUEST_ID => "x-ms-transport-request-id",
    ITEM_LSN => "x-ms-item-lsn",
    COSMOS_ITEM_LLSN => "x-ms-cosmos-item-llsn",
    COSMOS_LLSN => "x-ms-cosmos-llsn",
    COSMOS_QUORUM_ACKED_LLSN => "x-ms-cosmos-quorum-acked-llsn",
    REQUEST_DURATION_MS => "x-ms-request-duration-ms",
    COSMOS_INTERNAL_PARTITION_ID => "x-ms-cosmos-internal-partition-id",
    // Thin Client
    THINCLIENT_PROXY_OPERATION_TYPE => "x-ms-thinclient-proxy-operation-type",
    THINCLIENT_PROXY_RESOURCE_TYPE => "x-ms-thinclient-proxy-resource-type",
    // Client ID
    CLIENT_ID => "x-ms-client-id",
}

pub const QUERY_CONTENT_TYPE: ContentType = ContentType::from_static("application/query+json");

pub(crate) const PREFER_MINIMAL: HeaderValue = HeaderValue::from_static("return=minimal");

pub const ACCOUNT_PROPERTIES_KEY: &str = "account_properties_key";

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
