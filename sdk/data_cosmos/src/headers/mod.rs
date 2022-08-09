use azure_core::headers::HeaderName;

pub(crate) mod from_headers;

pub(crate) const HEADER_DOCUMENTDB_IS_UPSERT: HeaderName =
    HeaderName::from_static("x-ms-documentdb-is-upsert");
pub(crate) const HEADER_INDEXING_DIRECTIVE: HeaderName =
    HeaderName::from_static("x-ms-indexing-directive");
pub(crate) const HEADER_CONSISTENCY_LEVEL: HeaderName =
    HeaderName::from_static("x-ms-consistency-level");
pub(crate) const HEADER_SESSION_TOKEN: HeaderName = HeaderName::from_static("x-ms-session-token");
pub(crate) const HEADER_ALLOW_MULTIPLE_WRITES: HeaderName =
    HeaderName::from_static("x-ms-cosmos-allow-tentative-writes");
pub(crate) const HEADER_A_IM: HeaderName = HeaderName::from_static("A-IM");
pub(crate) const HEADER_DOCUMENTDB_PARTITIONRANGEID: HeaderName =
    HeaderName::from_static("x-ms-documentdb-partitionkeyrangeid");
pub(crate) const HEADER_DOCUMENTDB_PARTITIONKEY: HeaderName =
    HeaderName::from_static("x-ms-documentdb-partitionkey");
pub(crate) const HEADER_NUMBER_OF_READ_REGIONS: HeaderName =
    HeaderName::from_static("x-ms-number-of-read-regions");
pub(crate) const HEADER_REQUEST_CHARGE: HeaderName = HeaderName::from_static("x-ms-request-charge");
pub(crate) const HEADER_OFFER_THROUGHPUT: HeaderName =
    HeaderName::from_static("x-ms-offer-throughput");
pub(crate) const HEADER_OFFER_TYPE: HeaderName = HeaderName::from_static("x-ms-offer-type");
#[allow(dead_code)]
pub(crate) const HEADER_DOCUMENTDB_ISQUERY: HeaderName =
    HeaderName::from_static("x-ms-documentdb-isquery");
pub(crate) const HEADER_DOCUMENTDB_QUERY_ENABLECROSSPARTITION: HeaderName =
    HeaderName::from_static("x-ms-documentdb-query-enablecrosspartition");
pub(crate) const HEADER_DOCUMENTDB_QUERY_PARALLELIZECROSSPARTITIONQUERY: HeaderName =
    HeaderName::from_static("x-ms-documentdb-query-parallelizecrosspartitionquery");
pub(crate) const HEADER_DOCUMENTDB_EXPIRY_SECONDS: HeaderName =
    HeaderName::from_static("x-ms-documentdb-expiry-seconds");
pub(crate) const HEADER_CONTENT_PATH: HeaderName = HeaderName::from_static("x-ms-content-path");
pub(crate) const HEADER_ALT_CONTENT_PATH: HeaderName =
    HeaderName::from_static("x-ms-alt-content-path");
pub(crate) const HEADER_LAST_STATE_CHANGE_UTC: HeaderName =
    HeaderName::from_static("x-ms-last-state-change-utc");
pub(crate) const HEADER_RESOURCE_QUOTA: HeaderName = HeaderName::from_static("x-ms-resource-quota");
pub(crate) const HEADER_RESOURCE_USAGE: HeaderName = HeaderName::from_static("x-ms-resource-usage");
pub(crate) const HEADER_QUORUM_ACKED_LSN: HeaderName =
    HeaderName::from_static("x-ms-quorum-acked-lsn");
pub(crate) const HEADER_CURRENT_WRITE_QUORUM: HeaderName =
    HeaderName::from_static("x-ms-current-write-quorum");
pub(crate) const HEADER_CURRENT_REPLICA_SET_SIZE: HeaderName =
    HeaderName::from_static("x-ms-current-replica-set-size");
pub(crate) const HEADER_SCHEMA_VERSION: HeaderName = HeaderName::from_static("x-ms-schemaversion");
pub(crate) const HEADER_SERVICE_VERSION: HeaderName =
    HeaderName::from_static("x-ms-serviceversion");
pub(crate) const HEADER_GATEWAY_VERSION: HeaderName =
    HeaderName::from_static("x-ms-gatewayversion");
pub(crate) const HEADER_COLLECTION_PARTITION_INDEX: HeaderName =
    HeaderName::from_static("collection-partition-index");
pub(crate) const HEADER_COLLECTION_SERVICE_INDEX: HeaderName =
    HeaderName::from_static("collection-service-index");
pub(crate) const HEADER_LSN: HeaderName = HeaderName::from_static("lsn");
pub(crate) const HEADER_GLOBAL_COMMITTED_LSN: HeaderName =
    HeaderName::from_static("x-ms-global-committed-lsn");
pub(crate) const HEADER_ITEM_LSN: HeaderName = HeaderName::from_static("x-ms-item-lsn");
pub(crate) const HEADER_TRANSPORT_REQUEST_ID: HeaderName =
    HeaderName::from_static("x-ms-transport-request-id");
pub(crate) const HEADER_COSMOS_LLSN: HeaderName = HeaderName::from_static("x-ms-cosmos-llsn");
pub(crate) const HEADER_COSMOS_ITEM_LLSN: HeaderName =
    HeaderName::from_static("x-ms-cosmos-item-llsn");
pub(crate) const HEADER_COSMOS_QUORUM_ACKED_LLSN: HeaderName =
    HeaderName::from_static("x-ms-cosmos-quorum-acked-llsn");
pub(crate) const HEADER_ROLE: HeaderName = HeaderName::from_static("x-ms-xp-role");
pub(crate) const HEADER_MAX_MEDIA_STORAGE_USAGE_MB: HeaderName =
    HeaderName::from_static("x-ms-max-media-storage-usage-mb");
pub(crate) const HEADER_MEDIA_STORAGE_USAGE_MB: HeaderName =
    HeaderName::from_static("x-ms-media-storage-usage-mb");
