// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Don't spell-check header names (which should start with 'x-').
// cSpell:disable

//! Constants defining HTTP headers and other values used internally by the SDK.

use azure_core::http::{headers::HeaderName, request::options::ContentType};

/// Macro to define Cosmos DB header constants and the allowed headers list in one place.
macro_rules! cosmos_headers {
    ($($name:ident => $value:literal),* $(,)?) => {
        $(
            #[allow(dead_code)]
            pub const $name: HeaderName = HeaderName::from_static($value);
        )*

        /// A list of all Cosmos DB specific headers that should be allowed in logging.
        #[allow(dead_code)]
        pub const COSMOS_ALLOWED_HEADERS: &[&HeaderName] = &[
            $(&$name,)*
            &azure_data_cosmos_driver::constants::GATEWAY20_OPERATION_TYPE,
            &azure_data_cosmos_driver::constants::GATEWAY20_RESOURCE_TYPE,
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
    OFFER_REPLACE_PENDING => "x-ms-offer-replace-pending",
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
    // Client ID
    CLIENT_ID => "x-ms-client-id",
    // these are not actually sent but are used internally for fault injection
    FAULT_INJECTION_OPERATION => "x-ms-fault-injection-operation",
    FAULT_INJECTION_CONTAINER_ID => "x-ms-fault-injection-container-id",
}

#[allow(dead_code)]
pub const QUERY_CONTENT_TYPE: ContentType = ContentType::from_static("application/query+json");

#[allow(dead_code)]
pub const ACCOUNT_PROPERTIES_KEY: &str = "account_properties_key";

// cSpell:enable

// -----------------------------------------------------------------------
// Environment-variable names
// -----------------------------------------------------------------------

/// Controls whether the per-partition circuit breaker is enabled.
///
/// Expected values: `"true"` or `"false"`. Defaults to `true` when unset.
pub const AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED: &str =
    "AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED";
