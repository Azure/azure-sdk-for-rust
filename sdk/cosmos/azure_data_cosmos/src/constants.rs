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

// Standard HTTP Headers
pub const X_HTTP_METHOD: HeaderName = HeaderName::from_static("x-http-method");
pub const SLUG: HeaderName = HeaderName::from_static("slug");
pub const CONTENT_ENCODING: HeaderName = HeaderName::from_static("content-encoding");
pub const CHARACTERSET: HeaderName = HeaderName::from_static("characterset");
pub const ACCEPT_ENCODING: HeaderName = HeaderName::from_static("accept-encoding");
pub const KEEP_ALIVE: HeaderName = HeaderName::from_static("keep-alive");
pub const CONTENT_LANGUAGE: HeaderName = HeaderName::from_static("content-language");
pub const CONTENT_LOCATION: HeaderName = HeaderName::from_static("content-location");
pub const CONTENT_MD5: HeaderName = HeaderName::from_static("content-md5");
pub const CONTENT_RANGE: HeaderName = HeaderName::from_static("content-range");
pub const ACCEPT_CHARSET: HeaderName = HeaderName::from_static("accept-charset");
pub const ACCEPT_LANGUAGE: HeaderName = HeaderName::from_static("accept-language");
pub const IF_RANGE: HeaderName = HeaderName::from_static("if-range");
pub const MAX_FORWARDS: HeaderName = HeaderName::from_static("max-forwards");
pub const ACCEPT_RANGES: HeaderName = HeaderName::from_static("accept-ranges");
pub const PROXY_AUTHENTICATE: HeaderName = HeaderName::from_static("proxy-authenticate");
pub const SET_COOKIE: HeaderName = HeaderName::from_static("set-cookie");
pub const ORIGIN: HeaderName = HeaderName::from_static("origin");
pub const HOST: HeaderName = HeaderName::from_static("host");
pub const ACCESS_CONTROL_ALLOW_ORIGIN: HeaderName =
    HeaderName::from_static("access-control-allow-origin");
pub const ACCESS_CONTROL_ALLOW_HEADERS: HeaderName =
    HeaderName::from_static("access-control-allow-headers");
pub const PREFER: HeaderName = HeaderName::from_static("prefer");
pub const LOCATION: HeaderName = HeaderName::from_static("location");
pub const REFERER: HeaderName = HeaderName::from_static("referer");
pub const STRICT_TRANSPORT_SECURITY: HeaderName =
    HeaderName::from_static("strict-transport-security");

// Bulk/Batch
pub const COSMOS_IS_BATCH_REQUEST: HeaderName =
    HeaderName::from_static("x-ms-cosmos-is-batch-request");
pub const COSMOS_BATCH_ATOMIC: HeaderName = HeaderName::from_static("x-ms-cosmos-batch-atomic");
pub const COSMOS_BATCH_CONTINUE_ON_ERROR: HeaderName =
    HeaderName::from_static("x-ms-cosmos-batch-continue-on-error");

// Query
pub const DOCUMENTDB_ISQUERY: HeaderName = HeaderName::from_static("x-ms-documentdb-isquery");
pub const COSMOS_QUERY_VERSION: HeaderName = HeaderName::from_static("x-ms-cosmos-query-version");
pub const COSMOS_QUERY_EXECUTION_INFO: HeaderName =
    HeaderName::from_static("x-ms-cosmos-query-execution-info");

// DocDB headers
pub const MAX_ITEM_COUNT: HeaderName = HeaderName::from_static("x-ms-max-item-count");
pub const DOCUMENTDB_RESPONSECONTINUATIONTOKENLIMITINKB: HeaderName =
    HeaderName::from_static("x-ms-documentdb-responsecontinuationtokenlimitinkb");
pub const ACTIVITY_ID: HeaderName = HeaderName::from_static("x-ms-activity-id");
pub const COSMOS_CORRELATED_ACTIVITYID: HeaderName =
    HeaderName::from_static("x-ms-cosmos-correlated-activityid");
pub const DOCUMENTDB_PRE_TRIGGER_EXCLUDE: HeaderName =
    HeaderName::from_static("x-ms-documentdb-pre-trigger-exclude");
pub const DOCUMENTDB_POST_TRIGGER_EXCLUDE: HeaderName =
    HeaderName::from_static("x-ms-documentdb-post-trigger-exclude");
pub const DATE: HeaderName = HeaderName::from_static("x-ms-date");
pub const COLLECTION_PARTITION_INFO: HeaderName =
    HeaderName::from_static("x-ms-collection-partition-info");
pub const COLLECTION_SERVICE_INFO: HeaderName =
    HeaderName::from_static("x-ms-collection-service-info");
pub const RETRY_AFTER_MS: HeaderName = HeaderName::from_static("x-ms-retry-after-ms");
pub const IS_FEED_UNFILTERED: HeaderName = HeaderName::from_static("x-ms-is-feed-unfiltered");
pub const DOCUMENTDB_EXPIRY_SECONDS: HeaderName =
    HeaderName::from_static("x-ms-documentdb-expiry-seconds");
pub const DOCUMENTDB_QUERY_ENABLE_SCAN: HeaderName =
    HeaderName::from_static("x-ms-documentdb-query-enable-scan");
pub const DOCUMENTDB_QUERY_EMIT_TRACES: HeaderName =
    HeaderName::from_static("x-ms-documentdb-query-emit-traces");
pub const ALT_CONTENT_PATH: HeaderName = HeaderName::from_static("x-ms-alt-content-path");
pub const CONTENT_PATH: HeaderName = HeaderName::from_static("x-ms-content-path");
pub const DOCUMENTDB_QUERY_ISCONTINUATIONEXPECTED: HeaderName =
    HeaderName::from_static("x-ms-documentdb-query-iscontinuationexpected");
pub const DOCUMENTDB_POPULATEQUERYMETRICS: HeaderName =
    HeaderName::from_static("x-ms-documentdb-populatequerymetrics");
pub const COSMOS_POPULATEINDEXMETRICS: HeaderName =
    HeaderName::from_static("x-ms-cosmos-populateindexmetrics");
pub const RESOURCE_QUOTA: HeaderName = HeaderName::from_static("x-ms-resource-quota");
pub const RESOURCE_USAGE: HeaderName = HeaderName::from_static("x-ms-resource-usage");
pub const COSMOS_INTENDED_COLLECTION_RID: HeaderName =
    HeaderName::from_static("x-ms-cosmos-intended-collection-rid");

// Quota Info
pub const ROOT_ENTITY_MAX_COUNT: HeaderName = HeaderName::from_static("x-ms-root-entity-max-count");
pub const ROOT_ENTITY_CURRENT_COUNT: HeaderName =
    HeaderName::from_static("x-ms-root-entity-current-count");
pub const COLLECTION_QUOTA_MB: HeaderName = HeaderName::from_static("x-ms-collection-quota-mb");
pub const COLLECTION_USAGE_MB: HeaderName = HeaderName::from_static("x-ms-collection-usage-mb");
pub const COSMOS_MAX_CONTENT_LENGTH: HeaderName =
    HeaderName::from_static("x-ms-cosmos-max-content-length");
pub const MAX_MEDIA_STORAGE_USAGE_MB: HeaderName =
    HeaderName::from_static("x-ms-max-media-storage-usage-mb");
pub const DATABASEACCOUNT_CONSUMED_MB: HeaderName =
    HeaderName::from_static("x-ms-databaseaccount-consumed-mb");
pub const DATABASEACCOUNT_PROVISIONED_MB: HeaderName =
    HeaderName::from_static("x-ms-databaseaccount-provisioned-mb");

// Collection quota
pub const DOCUMENTDB_POPULATEQUOTAINFO: HeaderName =
    HeaderName::from_static("x-ms-documentdb-populatequotainfo");
pub const DOCUMENTDB_POPULATEPARTITIONSTATISTICS: HeaderName =
    HeaderName::from_static("x-ms-documentdb-populatepartitionstatistics");
pub const COLLECTION_PARTITION_INDEX: HeaderName =
    HeaderName::from_static("collection-partition-index");
pub const COLLECTION_SERVICE_INDEX: HeaderName =
    HeaderName::from_static("collection-service-index");

// Usage Info
pub const MEDIA_STORAGE_USAGE_MB: HeaderName =
    HeaderName::from_static("x-ms-media-storage-usage-mb");
pub const REQUEST_CHARGE: HeaderName = HeaderName::from_static("x-ms-request-charge");

// Address related headers
pub const FORCE_REFRESH: HeaderName = HeaderName::from_static("x-ms-force-refresh");
pub const ITEM_COUNT: HeaderName = HeaderName::from_static("x-ms-item-count");
pub const NEW_RESOURCE_ID: HeaderName = HeaderName::from_static("x-ms-new-resource-id");
pub const USE_MASTER_COLLECTION_RESOLVER: HeaderName =
    HeaderName::from_static("x-ms-use-master-collection-resolver");

// Admin Headers
pub const FORCE_FULL_UPGRADE: HeaderName = HeaderName::from_static("x-ms-force-full-upgrade");
pub const ONLY_UPGRADE_SYSTEM_APPLICATIONS: HeaderName =
    HeaderName::from_static("x-ms-only-upgrade-system-applications");
pub const ONLY_UPGRADE_NON_SYSTEM_APPLICATIONS: HeaderName =
    HeaderName::from_static("x-ms-only-upgrade-non-system-applications");
pub const UPGRADE_FABRIC_CODE_CONFIG: HeaderName =
    HeaderName::from_static("x-ms-upgrade-fabric-code-config");
pub const IGNORE_INPROGRESS_UPGRADE: HeaderName =
    HeaderName::from_static("x-ms-ignore-inprogress-upgrade");
pub const UPGRADE_VERIFICATION_KIND: HeaderName =
    HeaderName::from_static("x-ms-upgrade-verification-kind");
pub const ISCANARY: HeaderName = HeaderName::from_static("x-ms-iscanary");

// Version
pub const VERSION: HeaderName = HeaderName::from_static("x-ms-version");

// RDFE
pub const OCP_RESOURCEPROVIDER_REGISTERED_URI: HeaderName =
    HeaderName::from_static("ocp-resourceprovider-registered-uri");

// State change
pub const LAST_STATE_CHANGE_UTC: HeaderName = HeaderName::from_static("x-ms-last-state-change-utc");

// Offer
pub const OFFER_TYPE: HeaderName = HeaderName::from_static("x-ms-offer-type");

// RU/minute
pub const DOCUMENTDB_DISABLE_RU_PER_MINUTE_USAGE: HeaderName =
    HeaderName::from_static("x-ms-documentdb-disable-ru-per-minute-usage");
pub const DOCUMENTDB_IS_RU_PER_MINUTE_USED: HeaderName =
    HeaderName::from_static("x-ms-documentdb-is-ru-per-minute-used");
pub const OFFER_IS_RU_PER_MINUTE_THROUGHPUT_ENABLED: HeaderName =
    HeaderName::from_static("x-ms-offer-is-ru-per-minute-throughput-enabled");

// Partitioned collections
pub const COSMOS_PHYSICAL_PARTITION_ID: HeaderName =
    HeaderName::from_static("x-ms-cosmos-physical-partition-id");
pub const COSMOS_IS_PARTITION_KEY_DELETE_PENDING: HeaderName =
    HeaderName::from_static("x-ms-cosmos-is-partition-key-delete-pending");
pub const START_EPK: HeaderName = HeaderName::from_static("x-ms-start-epk");
pub const END_EPK: HeaderName = HeaderName::from_static("x-ms-end-epk");
pub const READ_KEY_TYPE: HeaderName = HeaderName::from_static("x-ms-read-key-type");
pub const COSMOS_SDK_SUPPORTEDCAPABILITIES: HeaderName =
    HeaderName::from_static("x-ms-cosmos-sdk-supportedcapabilities");

// Index progress
pub const DOCUMENTDB_COLLECTION_INDEX_TRANSFORMATION_PROGRESS: HeaderName =
    HeaderName::from_static("x-ms-documentdb-collection-index-transformation-progress");
pub const DOCUMENTDB_COLLECTION_LAZY_INDEXING_PROGRESS: HeaderName =
    HeaderName::from_static("x-ms-documentdb-collection-lazy-indexing-progress");

// Client retry
pub const THROTTLE_RETRY_COUNT: HeaderName = HeaderName::from_static("x-ms-throttle-retry-count");
pub const THROTTLE_RETRY_WAIT_TIME_MS: HeaderName =
    HeaderName::from_static("x-ms-throttle-retry-wait-time-ms");

// StoredProcedure
pub const DOCUMENTDB_SCRIPT_ENABLE_LOGGING: HeaderName =
    HeaderName::from_static("x-ms-documentdb-script-enable-logging");
pub const DOCUMENTDB_SCRIPT_LOG_RESULTS: HeaderName =
    HeaderName::from_static("x-ms-documentdb-script-log-results");

// Change feed
pub const A_IM: HeaderName = HeaderName::from_static("a-im");
pub const COSMOS_CHANGEFEED_WIRE_FORMAT_VERSION: HeaderName =
    HeaderName::from_static("x-ms-cosmos-changefeed-wire-format-version");

// Dedicated Gateway
pub const DEDICATEDGATEWAY_MAX_AGE: HeaderName =
    HeaderName::from_static("x-ms-dedicatedgateway-max-age");
pub const COSMOS_CACHEHIT: HeaderName = HeaderName::from_static("x-ms-cosmos-cachehit");

// Backend
pub const LSN: HeaderName = HeaderName::from_static("lsn");
pub const SCHEMAVERSION: HeaderName = HeaderName::from_static("x-ms-schemaversion");
pub const GATEWAYVERSION: HeaderName = HeaderName::from_static("x-ms-gatewayversion");
pub const SERVICEVERSION: HeaderName = HeaderName::from_static("x-ms-serviceversion");
pub const QUORUM_ACKED_LSN: HeaderName = HeaderName::from_static("x-ms-quorum-acked-lsn");
pub const CURRENT_WRITE_QUORUM: HeaderName = HeaderName::from_static("x-ms-current-write-quorum");
pub const CURRENT_REPLICA_SET_SIZE: HeaderName =
    HeaderName::from_static("x-ms-current-replica-set-size");
pub const XP_ROLE: HeaderName = HeaderName::from_static("x-ms-xp-role");
pub const GLOBAL_COMMITTED_LSN: HeaderName = HeaderName::from_static("x-ms-global-committed-lsn");
pub const NUMBER_OF_READ_REGIONS: HeaderName =
    HeaderName::from_static("x-ms-number-of-read-regions");
pub const TRANSPORT_REQUEST_ID: HeaderName = HeaderName::from_static("x-ms-transport-request-id");
pub const ITEM_LSN: HeaderName = HeaderName::from_static("x-ms-item-lsn");
pub const COSMOS_ITEM_LLSN: HeaderName = HeaderName::from_static("x-ms-cosmos-item-llsn");
pub const COSMOS_LLSN: HeaderName = HeaderName::from_static("x-ms-cosmos-llsn");
pub const COSMOS_QUORUM_ACKED_LLSN: HeaderName =
    HeaderName::from_static("x-ms-cosmos-quorum-acked-llsn");
pub const REQUEST_DURATION_MS: HeaderName = HeaderName::from_static("x-ms-request-duration-ms");
pub const COSMOS_INTERNAL_PARTITION_ID: HeaderName =
    HeaderName::from_static("x-ms-cosmos-internal-partition-id");

// Thin Client
pub const THINCLIENT_PROXY_OPERATION_TYPE: HeaderName =
    HeaderName::from_static("x-ms-thinclient-proxy-operation-type");
pub const THINCLIENT_PROXY_RESOURCE_TYPE: HeaderName =
    HeaderName::from_static("x-ms-thinclient-proxy-resource-type");

// Client ID
pub const CLIENT_ID: HeaderName = HeaderName::from_static("x-ms-client-id");

pub const QUERY_CONTENT_TYPE: ContentType = ContentType::from_static("application/query+json");

pub(crate) const PREFER_MINIMAL: HeaderValue = HeaderValue::from_static("return=minimal");

pub const ACCOUNT_PROPERTIES_KEY: &str = "account_properties_key";

use std::borrow::Cow;
use std::sync::LazyLock;

// Unauthorized headers that should never be included are `authorization`, `proxy-authorization`
// The comments indicate headers that are already included in the default allow list by azure-core
/// A lazily initialized list of all Cosmos DB specific headers that should be allowed in logging.
/// This is constructed from the header constants defined above.
pub static COSMOS_ALLOWED_HEADERS: LazyLock<Vec<Cow<'static, str>>> = LazyLock::new(|| {
    [
        // Standard HTTP Headers
        // "etag", // Already in default list
        &X_HTTP_METHOD,
        &SLUG,
        // "content-type", // Already in default list
        // "last-modified", // Already in default list
        &CONTENT_ENCODING,
        &CHARACTERSET,
        // "user-agent", // Already in default list
        // "if-modified-since", // Already in default list
        // "if-match", // Already in default list
        // "if-none-match", // Already in default list
        // "content-length", // Already in default list
        &ACCEPT_ENCODING,
        &KEEP_ALIVE,
        // "cache-control", // Already in default list
        // "transfer-encoding", // Already in default list
        &CONTENT_LANGUAGE,
        &CONTENT_LOCATION,
        &CONTENT_MD5,
        &CONTENT_RANGE,
        // "accept", // Already in default list
        &ACCEPT_CHARSET,
        &ACCEPT_LANGUAGE,
        &IF_RANGE,
        // "if-unmodified-since", // Already in default list
        &MAX_FORWARDS,
        &ACCEPT_RANGES,
        &PROXY_AUTHENTICATE,
        // "retry-after", // Already in default list
        &SET_COOKIE,
        // "www-authenticate", // Already in default list
        &ORIGIN,
        &HOST,
        &ACCESS_CONTROL_ALLOW_ORIGIN,
        &ACCESS_CONTROL_ALLOW_HEADERS,
        // "date", // Already in default list
        &PREFER,
        &LOCATION,
        &REFERER,
        // "pragma", // Already in default list
        // "server", // Already in default list
        &STRICT_TRANSPORT_SECURITY,
        // Bulk/Batch
        &COSMOS_IS_BATCH_REQUEST,
        &COSMOS_BATCH_ATOMIC,
        &COSMOS_BATCH_CONTINUE_ON_ERROR,
        // Query
        &QUERY,
        &DOCUMENTDB_ISQUERY,
        &IS_QUERY_PLAN_REQUEST,
        &SUPPORTED_QUERY_FEATURES,
        &COSMOS_QUERY_VERSION,
        &QUERY_METRICS,
        &COSMOS_QUERY_EXECUTION_INFO,
        &INDEX_METRICS,
        // DocDB headers
        &CONTINUATION,
        &MAX_ITEM_COUNT,
        &DOCUMENTDB_RESPONSECONTINUATIONTOKENLIMITINKB,
        &PRIORITY_LEVEL,
        &ACTIVITY_ID,
        &COSMOS_CORRELATED_ACTIVITYID,
        &PRE_TRIGGER_INCLUDE,
        &DOCUMENTDB_PRE_TRIGGER_EXCLUDE,
        &POST_TRIGGER_INCLUDE,
        &DOCUMENTDB_POST_TRIGGER_EXCLUDE,
        &INDEXING_DIRECTIVE,
        &SESSION_TOKEN,
        &CONSISTENCY_LEVEL,
        &DATE,
        &COLLECTION_PARTITION_INFO,
        &COLLECTION_SERVICE_INFO,
        &RETRY_AFTER_MS,
        &IS_FEED_UNFILTERED,
        &DOCUMENTDB_EXPIRY_SECONDS,
        &DOCUMENTDB_QUERY_ENABLE_SCAN,
        &DOCUMENTDB_QUERY_EMIT_TRACES,
        &SUB_STATUS,
        &ALT_CONTENT_PATH,
        &CONTENT_PATH,
        &DOCUMENTDB_QUERY_ISCONTINUATIONEXPECTED,
        &DOCUMENTDB_POPULATEQUERYMETRICS,
        &COSMOS_POPULATEINDEXMETRICS,
        &RESOURCE_QUOTA,
        &RESOURCE_USAGE,
        &COSMOS_INTENDED_COLLECTION_RID,
        // Quota Info
        &ROOT_ENTITY_MAX_COUNT,
        &ROOT_ENTITY_CURRENT_COUNT,
        &COLLECTION_QUOTA_MB,
        &COLLECTION_USAGE_MB,
        &COSMOS_MAX_CONTENT_LENGTH,
        &MAX_MEDIA_STORAGE_USAGE_MB,
        &DATABASEACCOUNT_CONSUMED_MB,
        &DATABASEACCOUNT_PROVISIONED_MB,
        // Collection quota
        &DOCUMENTDB_POPULATEQUOTAINFO,
        &DOCUMENTDB_POPULATEPARTITIONSTATISTICS,
        &COLLECTION_PARTITION_INDEX,
        &COLLECTION_SERVICE_INDEX,
        // Usage Info
        &MEDIA_STORAGE_USAGE_MB,
        &REQUEST_CHARGE,
        // Address related headers
        &FORCE_REFRESH,
        &ITEM_COUNT,
        &NEW_RESOURCE_ID,
        &USE_MASTER_COLLECTION_RESOLVER,
        // Admin Headers
        &FORCE_FULL_UPGRADE,
        &ONLY_UPGRADE_SYSTEM_APPLICATIONS,
        &ONLY_UPGRADE_NON_SYSTEM_APPLICATIONS,
        &UPGRADE_FABRIC_CODE_CONFIG,
        &IGNORE_INPROGRESS_UPGRADE,
        &UPGRADE_VERIFICATION_KIND,
        &ISCANARY,
        // Version
        &VERSION,
        // RDFE
        &OCP_RESOURCEPROVIDER_REGISTERED_URI,
        // Request management
        // "x-ms-request-id", // Already in default list
        // State change
        &LAST_STATE_CHANGE_UTC,
        // Offer
        &OFFER_TYPE,
        &OFFER_THROUGHPUT,
        &OFFER_AUTOPILOT_SETTINGS,
        // RU/minute
        &DOCUMENTDB_DISABLE_RU_PER_MINUTE_USAGE,
        &DOCUMENTDB_IS_RU_PER_MINUTE_USED,
        &OFFER_IS_RU_PER_MINUTE_THROUGHPUT_ENABLED,
        &THROUGHPUT_BUCKET,
        // Partitioned collections
        &PARTITION_KEY,
        &QUERY_ENABLE_CROSS_PARTITION,
        &PARTITION_KEY_RANGE_ID,
        &COSMOS_PHYSICAL_PARTITION_ID,
        &COSMOS_IS_PARTITION_KEY_DELETE_PENDING,
        &START_EPK,
        &END_EPK,
        &READ_KEY_TYPE,
        &COSMOS_SDK_SUPPORTEDCAPABILITIES,
        // Upsert
        &IS_UPSERT,
        // Index progress
        &DOCUMENTDB_COLLECTION_INDEX_TRANSFORMATION_PROGRESS,
        &DOCUMENTDB_COLLECTION_LAZY_INDEXING_PROGRESS,
        // Client retry
        &THROTTLE_RETRY_COUNT,
        &THROTTLE_RETRY_WAIT_TIME_MS,
        // StoredProcedure
        &DOCUMENTDB_SCRIPT_ENABLE_LOGGING,
        &DOCUMENTDB_SCRIPT_LOG_RESULTS,
        // Change feed
        &A_IM,
        &COSMOS_CHANGEFEED_WIRE_FORMAT_VERSION,
        // Multiple Write Locations
        &ALLOW_TENTATIVE_WRITES,
        // Dedicated Gateway
        &DEDICATEDGATEWAY_MAX_AGE,
        &COSMOS_CACHEHIT,
        // Backend
        &LSN,
        &SCHEMAVERSION,
        &GATEWAYVERSION,
        &SERVICEVERSION,
        &QUORUM_ACKED_LSN,
        &CURRENT_WRITE_QUORUM,
        &CURRENT_REPLICA_SET_SIZE,
        &XP_ROLE,
        &GLOBAL_COMMITTED_LSN,
        &NUMBER_OF_READ_REGIONS,
        &TRANSPORT_REQUEST_ID,
        &ITEM_LSN,
        &COSMOS_ITEM_LLSN,
        &COSMOS_LLSN,
        &COSMOS_QUORUM_ACKED_LLSN,
        &REQUEST_DURATION_MS,
        &COSMOS_INTERNAL_PARTITION_ID,
        // Thin Client
        &THINCLIENT_PROXY_OPERATION_TYPE,
        &THINCLIENT_PROXY_RESOURCE_TYPE,
        // Client ID
        &CLIENT_ID,
    ]
    .iter()
    .map(|h| Cow::Owned(h.as_str().to_owned()))
    .collect()
});

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
