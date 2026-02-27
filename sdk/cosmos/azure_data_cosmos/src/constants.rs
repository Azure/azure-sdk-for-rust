// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Don't spell-check header names (which should start with 'x-').
// cSpell:disable

//! Constants defining HTTP headers and other values relevant to Azure Cosmos DB APIs.

use azure_core::http::{
    headers::{HeaderName, HeaderValue},
    request::options::ContentType,
    StatusCode,
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
    IF_NONE_MATCH => "if-none-match",
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
    // these are not actually sent but are used internally for fault injection
    FAULT_INJECTION_OPERATION => "x-ms-fault-injection-operation",
    FAULT_INJECTION_CONTAINER_ID => "x-ms-fault-injection-container-id",
}

pub const QUERY_CONTENT_TYPE: ContentType = ContentType::from_static("application/query+json");

pub(crate) const PREFER_MINIMAL: HeaderValue = HeaderValue::from_static("return=minimal");

pub const ACCOUNT_PROPERTIES_KEY: &str = "account_properties_key";

/// The Cosmos DB-specific 449 Retry With status code.
///
/// This status code indicates the client must retry with modified request parameters.
/// It is non-retryable because automatic retry without parameter changes will not succeed.
pub(crate) const RETRY_WITH: StatusCode = StatusCode::UnknownValue(449);

// Default HTTP client timeouts.
// See `next_generation_sdks_design_principles.md` for design rationale.

/// Default TCP connection timeout (1s).
/// After 1 second it times out locally.
///
/// Aggressive default per design doc: fast failure on downed nodes improves P9x latency.
#[cfg(feature = "reqwest")]
pub(crate) const DEFAULT_CONNECTION_TIMEOUT: std::time::Duration =
    std::time::Duration::from_secs(1);

/// Default overall request timeout (65s).
///
/// Chosen to balance fast failure with allowing multiple retry attempts and to
/// remain just above typical 60s service timeouts.
/// See `next_generation_sdks_design_principles.md` for detailed rationale.
#[cfg(feature = "reqwest")]
pub(crate) const DEFAULT_REQUEST_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(65);

/// Default maximum idle connections per host (1000).
///
/// Limits connection pool growth to prevent resource exhaustion under high
/// concurrency while still allowing ample connection reuse.
#[cfg(feature = "reqwest")]
pub(crate) const DEFAULT_MAX_CONNECTION_POOL_SIZE: usize = 1000;

/// A newtype wrapper for Cosmos DB sub-status codes.
///
/// Sub-status codes provide additional context for HTTP error responses from Cosmos DB.
/// This type wraps a `usize` value representing the numeric code.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct SubStatusCode(usize);

impl SubStatusCode {
    /// Creates a new `SubStatusCode` from a numeric value.
    pub const fn new(code: usize) -> Self {
        Self(code)
    }

    /// Returns the numeric value of the sub-status code.
    pub const fn value(&self) -> usize {
        self.0
    }

    /// Creates a `SubStatusCode` from a header string.
    /// Returns `None` if parsing fails.
    pub fn from_header_value(s: &str) -> Option<Self> {
        let raw = s.trim();
        raw.parse::<u32>().ok().map(|v| SubStatusCode(v as usize))
    }

    // Internal constants for sub-status codes
    // 400: Bad Request sub-status
    #[allow(dead_code)]
    pub(crate) const PARTITION_KEY_MISMATCH: SubStatusCode = SubStatusCode(1001);
    #[allow(dead_code)]
    pub(crate) const CROSS_PARTITION_QUERY_NOT_SERVABLE: SubStatusCode = SubStatusCode(1004);
    #[allow(dead_code)]
    pub(crate) const SCRIPT_COMPILE_ERROR: SubStatusCode = SubStatusCode(0xFFFF);
    #[allow(dead_code)]
    pub(crate) const ANOTHER_OFFER_REPLACE_OPERATION_IS_IN_PROGRESS: SubStatusCode =
        SubStatusCode(3205);
    #[allow(dead_code)]
    pub(crate) const HTTP_LISTENER_EXCEPTION: SubStatusCode = SubStatusCode(1101);

    // 410: StatusCodeType_Gone: sub-status
    #[allow(dead_code)]
    pub(crate) const NAME_CACHE_IS_STALE: SubStatusCode = SubStatusCode(1000);
    pub(crate) const PARTITION_KEY_RANGE_GONE: SubStatusCode = SubStatusCode(1002);
    #[allow(dead_code)]
    pub(crate) const COMPLETING_SPLIT: SubStatusCode = SubStatusCode(1007);
    #[allow(dead_code)]
    pub(crate) const COMPLETING_PARTITION_MIGRATION: SubStatusCode = SubStatusCode(1008);
    pub(crate) const LEASE_NOT_FOUND: SubStatusCode = SubStatusCode(1022);
    #[allow(dead_code)]
    pub(crate) const ARCHIVAL_PARTITION_NOT_PRESENT: SubStatusCode = SubStatusCode(1024);

    // 404: LSN in session token is higher
    #[allow(dead_code)]
    pub(crate) const OWNER_RESOURCE_NOT_FOUND: SubStatusCode = SubStatusCode(1003);
    #[allow(dead_code)]
    pub(crate) const CONFIGURATION_PROPERTY_NOT_FOUND: SubStatusCode = SubStatusCode(1005);
    #[allow(dead_code)]
    pub(crate) const COLLECTION_CREATE_IN_PROGRESS: SubStatusCode = SubStatusCode(1013);
    #[allow(dead_code)]
    pub(crate) const STORE_NOT_READY: SubStatusCode = SubStatusCode(1023);
    #[allow(dead_code)]
    pub(crate) const AUTH_TOKEN_NOT_FOUND_IN_CACHE: SubStatusCode = SubStatusCode(1030);

    // 404: StatusCodeType_NotFound: sub-status
    #[allow(dead_code)]
    pub(crate) const PARTITION_MIGRATING_COLLECTION_DELETED: SubStatusCode = SubStatusCode(1031);
    #[allow(dead_code)]
    pub(crate) const PARTITION_MIGRATION_SOURCE_PARTITION_DELETED_IN_MASTER: SubStatusCode =
        SubStatusCode(1034);
    #[allow(dead_code)]
    pub(crate) const PARTITION_MIGRATION_SHARED_THROUGHPUT_DATABASE_PARTITION_RESOURCE_NOT_FOUND_IN_MASTER: SubStatusCode = SubStatusCode(1035);
    #[allow(dead_code)]
    pub(crate) const PARTITION_MIGRATION_PARTITION_RESOURCE_NOT_FOUND_IN_MASTER: SubStatusCode =
        SubStatusCode(1036);
    #[allow(dead_code)]
    pub(crate) const PARTITION_MIGRATION_FAILED_TO_UPDATE_DNS: SubStatusCode = SubStatusCode(1037);

    // 403: Forbidden sub-status
    pub(crate) const WRITE_FORBIDDEN: SubStatusCode = SubStatusCode(3);
    #[allow(dead_code)]
    pub(crate) const REDUNDANT_COLLECTION_PUT: SubStatusCode = SubStatusCode(1009);
    #[allow(dead_code)]
    pub(crate) const SHARED_THROUGHPUT_DATABASE_QUOTA_EXCEEDED: SubStatusCode = SubStatusCode(1010);
    #[allow(dead_code)]
    pub(crate) const SHARED_THROUGHPUT_OFFER_GROW_NOT_NEEDED: SubStatusCode = SubStatusCode(1011);
    #[allow(dead_code)]
    pub(crate) const PARTITION_KEY_QUOTA_OVER_LIMIT: SubStatusCode = SubStatusCode(1014);
    #[allow(dead_code)]
    pub(crate) const SHARED_THROUGHPUT_DATABASE_COLLECTION_COUNT_EXCEEDED: SubStatusCode =
        SubStatusCode(1019);
    #[allow(dead_code)]
    pub(crate) const SHARED_THROUGHPUT_DATABASE_COUNT_EXCEEDED: SubStatusCode = SubStatusCode(1020);
    #[allow(dead_code)]
    pub(crate) const COMPUTE_INTERNAL_ERROR: SubStatusCode = SubStatusCode(1021);
    #[allow(dead_code)]
    pub(crate) const THROUGHPUT_CAP_QUOTA_EXCEEDED: SubStatusCode = SubStatusCode(1028);
    #[allow(dead_code)]
    pub(crate) const INVALID_THROUGHPUT_CAP_VALUE: SubStatusCode = SubStatusCode(1029);

    // 409: Conflict exception
    #[allow(dead_code)]
    pub(crate) const CONFLICT_WITH_CONTROL_PLANE: SubStatusCode = SubStatusCode(1006);
    #[allow(dead_code)]
    pub(crate) const DATABASE_NAME_ALREADY_EXISTS: SubStatusCode = SubStatusCode(3206);
    #[allow(dead_code)]
    pub(crate) const PARTITION_KEY_HASH_COLLISION_FOR_ID: SubStatusCode = SubStatusCode(3302);

    // 409: Partition migration Count mismatch conflict sub status codes
    #[allow(dead_code)]
    pub(crate) const PARTITION_MIGRATION_DOCUMENT_COUNT_MISMATCH_BETWEEN_SOURCE_AND_TARGET_PARTITION:
        SubStatusCode = SubStatusCode(3050);
    #[allow(dead_code)]
    pub(crate) const PARTITION_MIGRATION_DOCUMENT_COUNT_MISMATCH_BETWEEN_TARGET_PARTITION_REPLICAS:
        SubStatusCode = SubStatusCode(3051);

    // 503: Service Unavailable due to region being out of capacity for bindable partitions
    #[allow(dead_code)]
    pub(crate) const COMPUTE_FEDERATION_NOT_FOUND: SubStatusCode = SubStatusCode(1012);
    #[allow(dead_code)]
    pub(crate) const OPERATION_PAUSED: SubStatusCode = SubStatusCode(9001);
    #[allow(dead_code)]
    pub(crate) const SERVICE_IS_OFFLINE: SubStatusCode = SubStatusCode(9002);
    #[allow(dead_code)]
    pub(crate) const INSUFFICIENT_CAPACITY: SubStatusCode = SubStatusCode(9003);

    // 412: PreCondition Failed
    #[allow(dead_code)]
    pub(crate) const SPLIT_IS_DISABLED: SubStatusCode = SubStatusCode(2001);
    #[allow(dead_code)]
    pub(crate) const COLLECTIONS_IN_PARTITION_GOT_UPDATED: SubStatusCode = SubStatusCode(2002);
    #[allow(dead_code)]
    pub(crate) const CAN_NOT_ACQUIRE_PKRANGES_LOCK: SubStatusCode = SubStatusCode(2003);
    #[allow(dead_code)]
    pub(crate) const RESOURCE_NOT_FOUND: SubStatusCode = SubStatusCode(2004);
    #[allow(dead_code)]
    pub(crate) const CAN_NOT_ACQUIRE_OFFER_OWNER_LOCK: SubStatusCode = SubStatusCode(2005);
    #[allow(dead_code)]
    pub(crate) const CAN_NOT_ACQUIRE_PKRANGE_LOCK: SubStatusCode = SubStatusCode(2007);
    #[allow(dead_code)]
    pub(crate) const CAN_NOT_ACQUIRE_PARTITION_LOCK: SubStatusCode = SubStatusCode(2008);
    #[allow(dead_code)]
    pub(crate) const STORAGE_SPLIT_CONFLICTING_WITH_NWAY_THROUGHPUT_SPLIT: SubStatusCode =
        SubStatusCode(2011);
    #[allow(dead_code)]
    pub(crate) const MERGE_IS_DISABLED: SubStatusCode = SubStatusCode(2012);
    #[allow(dead_code)]
    pub(crate) const TOMBSTONE_RECORDS_NOT_FOUND: SubStatusCode = SubStatusCode(2015);
    #[allow(dead_code)]
    pub(crate) const INVALID_ACCOUNT_STATUS: SubStatusCode = SubStatusCode(2016);
    #[allow(dead_code)]
    pub(crate) const OFFER_VALIDATION_FAILED: SubStatusCode = SubStatusCode(2017);
    #[allow(dead_code)]
    pub(crate) const CAN_NOT_ACQUIRE_MASTER_PARTITION_ACCESS_LOCK: SubStatusCode =
        SubStatusCode(2018);
    #[allow(dead_code)]
    pub(crate) const CAN_NOT_ACQUIRE_IN_ACCOUNT_RESTORE_IN_PROGRESS_LOCK: SubStatusCode =
        SubStatusCode(2019);
    #[allow(dead_code)]
    pub(crate) const COLLECTION_STATE_CHANGED: SubStatusCode = SubStatusCode(2020);
    #[allow(dead_code)]
    pub(crate) const OFFER_SCALED_UP_BY_USER: SubStatusCode = SubStatusCode(2021);
    #[allow(dead_code)]
    pub(crate) const CAN_NOT_ACQUIRE_LOG_STORE_STORAGE_ACCOUNT_LOAD_BALANCE_LOCK: SubStatusCode =
        SubStatusCode(2101);

    // 500: InternalServerError
    #[allow(dead_code)]
    pub(crate) const CONFIGURATION_NAME_NOT_EMPTY: SubStatusCode = SubStatusCode(3001);
    #[allow(dead_code)]
    pub(crate) const CONFIGURATION_OPERATION_CANCELLED: SubStatusCode = SubStatusCode(3002);
    #[allow(dead_code)]
    pub(crate) const INVALID_ACCOUNT_CONFIGURATION: SubStatusCode = SubStatusCode(3003);
    #[allow(dead_code)]
    pub(crate) const FEDERATION_DOES_NOT_EXIST_OR_IS_LOCKED: SubStatusCode = SubStatusCode(3004);
    #[allow(dead_code)]
    pub(crate) const PARTITION_FAILOVER_ERROR_CODE: SubStatusCode = SubStatusCode(3010);

    // 429: Request Rate Too Large
    #[allow(dead_code)]
    pub(crate) const TOO_MANY_REQUESTS: SubStatusCode = SubStatusCode(429);
    #[allow(dead_code)]
    pub(crate) const PREPARE_TIME_LIMIT_EXCEEDED: SubStatusCode = SubStatusCode(3207);
    #[allow(dead_code)]
    pub(crate) const CLIENT_TCP_CHANNEL_FULL: SubStatusCode = SubStatusCode(3208);
    #[allow(dead_code)]
    pub(crate) const BW_TERM_COUNT_LIMIT_EXCEEDED: SubStatusCode = SubStatusCode(3209);
    #[allow(dead_code)]
    pub(crate) const RU_BUDGET_EXCEEDED: SubStatusCode = SubStatusCode(3200);
    #[allow(dead_code)]
    pub(crate) const GATEWAY_THROTTLED: SubStatusCode = SubStatusCode(3201);
    #[allow(dead_code)]
    pub(crate) const STORED_PROCEDURE_CONCURRENCY: SubStatusCode = SubStatusCode(3084);
    #[allow(dead_code)]
    pub(crate) const SYSTEM_RESOURCE_NOT_AVAILABLE: SubStatusCode = SubStatusCode(3092);

    // Additional aliases for backwards compatibility (internal use only)
    #[allow(dead_code)]
    pub(crate) const CONFIGURATION_NAME_NOT_FOUND: SubStatusCode =
        Self::CROSS_PARTITION_QUERY_NOT_SERVABLE;
    pub(crate) const READ_SESSION_NOT_AVAILABLE: SubStatusCode = Self::PARTITION_KEY_RANGE_GONE;
    #[allow(dead_code)]
    pub(crate) const PROVISION_LIMIT_REACHED: SubStatusCode =
        Self::CONFIGURATION_PROPERTY_NOT_FOUND;
    #[allow(dead_code)]
    pub(crate) const INSUFFICIENT_BINDABLE_PARTITIONS: SubStatusCode = Self::COMPLETING_SPLIT;
    #[allow(dead_code)]
    pub(crate) const DATABASE_ACCOUNT_NOT_FOUND: SubStatusCode =
        Self::COMPLETING_PARTITION_MIGRATION;
}

impl From<SubStatusCode> for usize {
    fn from(code: SubStatusCode) -> Self {
        code.0
    }
}

impl From<u32> for SubStatusCode {
    fn from(value: u32) -> Self {
        SubStatusCode(value as usize)
    }
}

impl std::fmt::Debug for SubStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for SubStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// cSpell:enable

// -----------------------------------------------------------------------
// Environment-variable names
// -----------------------------------------------------------------------

/// Controls whether the per-partition circuit breaker is enabled.
///
/// Expected values: `"true"` or `"false"`. Defaults to `true` when unset.
pub const AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED: &str =
    "AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED";

/// Maximum duration (in seconds) a partition may remain marked unavailable
/// before the background failback loop considers it eligible for health
/// re-evaluation.
///
/// Expected value: an integer string. Falls back to the compiled-in default
/// when unset or not parseable.
pub const AZURE_COSMOS_ALLOWED_PARTITION_UNAVAILABILITY_DURATION_IN_SECONDS: &str =
    "AZURE_COSMOS_ALLOWED_PARTITION_UNAVAILABILITY_DURATION_IN_SECONDS";

/// Interval (in seconds) at which the background failback loop runs to check
/// whether previously failed partitions can be restored to healthy status.
///
/// Expected value: an integer string. Falls back to the compiled-in default
/// when unset or not parseable.
pub const AZURE_COSMOS_PPCB_STALE_PARTITION_UNAVAILABILITY_REFRESH_INTERVAL_IN_SECONDS: &str =
    "AZURE_COSMOS_PPCB_STALE_PARTITION_UNAVAILABILITY_REFRESH_INTERVAL_IN_SECONDS";

/// Consecutive read failure count threshold that triggers the circuit breaker.
///
/// Expected value: an integer string. Falls back to the compiled-in default
/// when unset or not parseable.
pub const AZURE_COSMOS_CIRCUIT_BREAKER_CONSECUTIVE_FAILURE_COUNT_FOR_READS: &str =
    "AZURE_COSMOS_CIRCUIT_BREAKER_CONSECUTIVE_FAILURE_COUNT_FOR_READS";

/// Consecutive write failure count threshold that triggers the circuit breaker.
///
/// Expected value: an integer string. Falls back to the compiled-in default
/// when unset or not parseable.
pub const AZURE_COSMOS_CIRCUIT_BREAKER_CONSECUTIVE_FAILURE_COUNT_FOR_WRITES: &str =
    "AZURE_COSMOS_CIRCUIT_BREAKER_CONSECUTIVE_FAILURE_COUNT_FOR_WRITES";

/// Window (in minutes) after which the read/write failure counters are reset
/// if no new failures occur.
///
/// Expected value: an integer string. Falls back to the compiled-in default
/// when unset or not parseable.
pub const AZURE_COSMOS_CIRCUIT_BREAKER_TIMEOUT_COUNTER_RESET_WINDOW_IN_MINUTES: &str =
    "AZURE_COSMOS_CIRCUIT_BREAKER_TIMEOUT_COUNTER_RESET_WINDOW_IN_MINUTES";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substatus_code_debug_value() {
        let code = SubStatusCode::new(1029);
        assert_eq!(format!("{:?}", code), "1029");
    }

    #[test]
    fn substatus_code_debug_another_value() {
        let code = SubStatusCode::new(1004);
        assert_eq!(format!("{:?}", code), "1004");
    }

    #[test]
    fn substatus_code_debug_unknown_value() {
        let code = SubStatusCode::new(9090);
        assert_eq!(format!("{:?}", code), "9090");
    }

    #[test]
    fn substatus_code_display_value() {
        let code = SubStatusCode::new(1029);
        assert_eq!(format!("{}", code), "1029");
    }

    #[test]
    fn substatus_code_display_unknown_value() {
        let code = SubStatusCode::new(9090);
        assert_eq!(format!("{}", code), "9090");
    }

    #[test]
    fn substatus_code_from_u32() {
        let code = SubStatusCode::from(429u32);
        assert_eq!(code.value(), 429);
        assert_eq!(format!("{}", code), "429");
    }

    #[test]
    fn substatus_code_from_header_value_valid() {
        let code = SubStatusCode::from_header_value("1002");
        assert!(code.is_some());
        assert_eq!(format!("{:?}", code.unwrap()), "1002");
    }

    #[test]
    fn substatus_code_from_header_value_unknown() {
        let code = SubStatusCode::from_header_value("99999");
        assert!(code.is_some());
        assert_eq!(format!("{:?}", code.unwrap()), "99999");
    }

    #[test]
    fn substatus_code_from_header_value_invalid() {
        let code = SubStatusCode::from_header_value("not-a-number");
        assert!(code.is_none());
    }
}
