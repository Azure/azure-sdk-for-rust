// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB request/response header models.

use std::borrow::Cow;

use crate::models::{ActivityId, Precondition, RequestCharge, SessionToken, SubStatusCode};
use azure_core::http::headers::{HeaderValue, Headers};
use azure_core::http::Etag;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::Serialize;
use std::num::NonZeroU32;

/// Per-page item-count hint for Cosmos feed-style operations
/// (`x-ms-max-item-count`).
///
/// Used by query and changefeed reads. Modeled as an explicit enum so callers
/// don't have to traffic in the `-1` wire sentinel directly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum MaxItemCountHint {
    /// Let the service decide the page size (emits `x-ms-max-item-count: -1`).
    ServerDecides,

    /// Cap the page at `N` items.
    Limit(NonZeroU32),
}

/// Standard Cosmos DB request header names.
///
/// All names are lowercase as required by the azure_core [`HeaderName`] type.
/// HTTP header names are technically case-insensitive, but `azure_core` normalizes
/// them to lowercase on insertion, so lookups are case-sensitive and will always
/// match since both sides are lowercase.
pub(crate) mod request_header_names {
    pub const ACTIVITY_ID: &str = "x-ms-activity-id";
    pub const SESSION_TOKEN: &str = "x-ms-session-token";
    pub const IF_MATCH: &str = "if-match";
    pub const IF_NONE_MATCH: &str = "if-none-match";
    pub const IF_MODIFIED_SINCE: &str = "if-modified-since";
    pub const PREFER: &str = "prefer";
    pub const IS_QUERY: &str = "x-ms-documentdb-isquery";
    pub const IS_QUERY_PLAN_REQUEST: &str = "x-ms-cosmos-is-query-plan-request";
    pub const SUPPORTED_QUERY_FEATURES: &str = "x-ms-cosmos-supported-query-features";
    pub const IS_UPSERT: &str = "x-ms-documentdb-is-upsert";
    pub const MAX_ITEM_COUNT: &str = "x-ms-max-item-count";
    /// Change-feed indicator ("Incremental Feed"). HTTP standard name `a-im`.
    pub const A_IM: &str = "a-im";
    pub const INCREMENTAL_FEED: &str = "Incremental Feed";
    /// Wire format version for change feed responses.
    pub const CHANGEFEED_WIRE_FORMAT_VERSION: &str = "x-ms-cosmos-changefeed-wire-format-version";
    /// The wire format version value used by this SDK.
    pub const CHANGEFEED_WIRE_FORMAT_VERSION_2021_09_15: &str = "2021-09-15";
    pub const POPULATE_INDEX_METRICS: &str = "x-ms-cosmos-populateindexmetrics";
    pub const POPULATE_QUERY_METRICS: &str = "x-ms-documentdb-populatequerymetrics";
    pub const ENABLE_CROSS_PARTITION_QUERY: &str = "x-ms-documentdb-query-enablecrosspartition";
    pub const IS_BATCH_REQUEST: &str = "x-ms-cosmos-is-batch-request";
    pub const BATCH_ATOMIC: &str = "x-ms-cosmos-batch-atomic";
    pub const BATCH_CONTINUE_ON_ERROR: &str = "x-ms-cosmos-batch-continue-on-error";
    pub const CONTINUATION: &str = "x-ms-continuation";
    pub const OFFER_THROUGHPUT: &str = "x-ms-offer-throughput";
    pub const OFFER_AUTOPILOT_SETTINGS: &str = "x-ms-cosmos-offer-autopilot-settings";
    pub const PRIORITY_LEVEL: &str = "x-ms-cosmos-priority-level";
    pub const THROUGHPUT_BUCKET: &str = "x-ms-cosmos-throughput-bucket";
    pub const START_EPK: &str = "x-ms-start-epk";
    pub const END_EPK: &str = "x-ms-end-epk";
    pub const READ_FEED_KEY_TYPE: &str = "x-ms-read-key-type";
    #[allow(dead_code)] // Reserved for future direct partition-key header writes.
    pub const PARTITION_KEY: &str = "x-ms-documentdb-partitionkey";
    pub const PARTITION_KEY_RANGE_ID: &str = "x-ms-documentdb-partitionkeyrangeid";
    /// Request-only header that asks the backend, on retries after a
    /// `404 / 1002 (READ_SESSION_NOT_AVAILABLE)` on single-master accounts,
    /// to route only to a region that has caught up to the requested LSN.
    ///
    /// Forward-compatible by design: backends that ignore the header
    /// preserve the existing single-master 1002 retry behavior. Mirrors
    /// .NET parity (Azure/azure-cosmos-dotnet-v3#5447).
    pub const HUB_REGION_PROCESSING_ONLY: &str = "x-ms-cosmos-hub-region-processing-only";

    /// Request-only header that opts the client into multi-master tentative
    /// writes. Multi-write Cosmos accounts require this header on every write;
    /// without it, satellite write regions return `403 / 3 (WriteForbidden)`
    /// because the request looks like single-master traffic to a non-primary
    /// region. The same wire name appears under
    /// [`response_header_names::HAS_TENTATIVE_WRITES`] on responses.
    pub const ALLOW_TENTATIVE_WRITES: &str = "x-ms-cosmos-allow-tentative-writes";
    /// DTX idempotency token (`Uuid`) generated once per distributed transaction.
    #[cfg(feature = "preview_dtx")]
    pub const DTX_IDEMPOTENCY_TOKEN: &str = "x-ms-cosmos-idempotency-token";
    /// DTX coordinator operation type (`CommitDistributedTransaction` or `Read`).
    #[cfg(feature = "preview_dtx")]
    pub const DTX_OPERATION_TYPE: &str = "x-ms-cosmos-operation-type";
    /// DTX coordinator resource type (`DistributedTransactionBatch`).
    #[cfg(feature = "preview_dtx")]
    pub const DTX_RESOURCE_TYPE: &str = "x-ms-cosmos-resource-type";
}

#[cfg(feature = "preview_dtx")]
pub(crate) const DTX_RESOURCE_TYPE_HEADER_VALUE: &str = "DistributedTransactionBatch";

/// Standard Cosmos DB response header names.
// cspell:ignore activityid acked llsn gatewayversion serviceversion
pub(crate) mod response_header_names {
    pub const ACTIVITY_ID: &str = "x-ms-activity-id";
    pub const REQUEST_CHARGE: &str = "x-ms-request-charge";
    pub const SESSION_TOKEN: &str = "x-ms-session-token";
    pub const ETAG: &str = "etag";
    pub const CONTINUATION: &str = "x-ms-continuation";
    pub const ITEM_COUNT: &str = "x-ms-item-count";
    pub const SUBSTATUS: &str = "x-ms-substatus";
    pub const INDEX_METRICS: &str = "x-ms-cosmos-index-utilization";
    pub const QUERY_METRICS: &str = "x-ms-documentdb-query-metrics";
    pub const SERVER_DURATION_MS: &str = "x-ms-request-duration-ms";
    pub const LSN: &str = "lsn";
    pub const ITEM_LSN: &str = "x-ms-item-lsn";
    pub const OWNER_FULL_NAME: &str = "x-ms-alt-content-path";
    pub const OWNER_ID: &str = "x-ms-content-path";
    pub const OFFER_REPLACE_PENDING: &str = "x-ms-offer-replace-pending";
    pub const RETRY_AFTER_MS: &str = "x-ms-retry-after-ms";
    pub const CORRELATED_ACTIVITY_ID: &str = "x-ms-cosmos-correlated-activityid";
    pub const TRANSPORT_REQUEST_ID: &str = "x-ms-transport-request-id";
    pub const GLOBAL_COMMITTED_LSN: &str = "x-ms-global-committed-lsn";
    pub const QUORUM_ACKED_LSN: &str = "x-ms-quorum-acked-lsn";
    pub const QUORUM_ACKED_LOCAL_LSN: &str = "x-ms-cosmos-quorum-acked-llsn";
    pub const LOCAL_LSN: &str = "x-ms-cosmos-llsn";
    pub const ITEM_LOCAL_LSN: &str = "x-ms-cosmos-item-llsn";
    pub const NUMBER_OF_READ_REGIONS: &str = "x-ms-number-of-read-regions";
    pub const LAST_STATE_CHANGE_UTC: &str = "x-ms-last-state-change-utc";
    pub const GATEWAY_VERSION: &str = "x-ms-gatewayversion";
    pub const SERVICE_VERSION: &str = "x-ms-serviceversion";
    pub const RESOURCE_QUOTA: &str = "x-ms-resource-quota";
    pub const RESOURCE_USAGE: &str = "x-ms-resource-usage";
    pub const HAS_TENTATIVE_WRITES: &str = "x-ms-cosmos-allow-tentative-writes";
    pub const PARTITION_KEY_RANGE_ID: &str = "x-ms-documentdb-partitionkeyrangeid";
    pub const INTERNAL_PARTITION_ID: &str = "x-ms-cosmos-internal-partition-id";
    pub const LOG_RESULTS: &str = "x-ms-documentdb-script-log-results";
    pub const COLLECTION_INDEX_TRANSFORMATION_PROGRESS: &str =
        "x-ms-documentdb-collection-index-transformation-progress";
    pub const COLLECTION_LAZY_INDEXING_PROGRESS: &str =
        "x-ms-documentdb-collection-lazy-indexing-progress";
    /// DTX idempotency token echoed by the coordinator.
    #[cfg(feature = "preview_dtx")]
    pub const DTX_IDEMPOTENCY_TOKEN: &str = "x-ms-cosmos-idempotency-token";
}

pub const QUERY_CONTENT_TYPE: &str = "application/query+json";

/// Header names used by the fault injection framework.
#[cfg(feature = "fault_injection")]
pub(crate) mod fault_injection_header_names {
    /// Operation type header set on requests for fault injection rule matching.
    pub const FAULT_INJECTION_OPERATION: &str = "x-ms-fault-injection-operation";
}

/// Cosmos request headers for operation-level customization.
///
/// Only whitelisted request headers are supported.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct CosmosRequestHeaders {
    /// Activity ID for request correlation (`x-ms-activity-id`).
    pub activity_id: Option<ActivityId>,

    /// Session token for session consistency (`x-ms-session-token`).
    pub session_token: Option<SessionToken>,

    /// Precondition for optimistic concurrency (`if-match` / `if-none-match`).
    pub precondition: Option<Precondition>,

    /// Manual throughput in RU/s (`x-ms-offer-throughput`).
    pub offer_throughput: Option<usize>,

    /// Autoscale settings (`x-ms-cosmos-offer-autopilot-settings`).
    ///
    /// The driver serializes this to JSON for the header value.
    pub offer_autopilot_settings: Option<OfferAutoscaleSettings>,

    /// Maximum number of items to return per page (`x-ms-max-item-count`).
    ///
    /// Used by feed/query/changefeed reads. See [`MaxItemCountHint`] for the two
    /// explicit values; the `-1` wire sentinel for "server decides" is
    /// represented by [`MaxItemCountHint::ServerDecides`].
    pub max_item_count: Option<MaxItemCountHint>,

    /// Requests an incremental change feed read (`a-im: Incremental Feed`).
    ///
    /// When `true`, the driver emits the standard change-feed indicator
    /// header. Combine with [`Precondition::if_none_match`] to pass a
    /// continuation token.
    pub incremental_feed: bool,

    /// When `true`, emits the change-feed wire format version header
    /// (`x-ms-cosmos-changefeed-wire-format-version: 2021-09-15`).
    ///
    /// This selects the structured change feed wire format, where each item is
    /// returned inside an envelope (`{ current, ... }`). It is sent for both
    /// LatestVersion and AllVersionsAndDeletes so the response shape stays
    /// consistent across modes; for LatestVersion the envelope simply omits the
    /// pre-image. The SDK iterator unwraps `current` back into the caller's
    /// document type.
    pub changefeed_wire_format_version: bool,

    /// If-Modified-Since timestamp for change feed point-in-time start.
    ///
    /// When set, the driver emits `If-Modified-Since: <value>`. The caller
    /// is responsible for formatting the timestamp in RFC 1123 format.
    pub if_modified_since: Option<String>,

    /// Request index-utilization metrics on the response
    /// (`x-ms-cosmos-populateindexmetrics`). Only meaningful for query
    /// operations.
    ///
    /// `None` omits the header (service default); `Some(true)` / `Some(false)`
    /// explicitly opt in or out. Using `Option<bool>` mirrors
    /// [`max_item_count`](Self::max_item_count) and lets the query executor
    /// distinguish "caller already chose" from "caller did not say".
    pub populate_index_metrics: Option<bool>,

    /// Request per-query metrics on the response
    /// (`x-ms-documentdb-populatequerymetrics`). Only meaningful for query
    /// operations. See [`populate_index_metrics`](Self::populate_index_metrics)
    /// for the `Option<bool>` semantics.
    pub populate_query_metrics: Option<bool>,

    /// When `true`, the Gateway is allowed to route the query across multiple
    /// partitions (`x-ms-documentdb-query-enablecrosspartition`). Required for
    /// query-plan requests and for queries without a partition-key scope.
    pub enable_cross_partition_query: bool,

    /// Supported query features (`x-ms-cosmos-supported-query-features`).
    ///
    /// Sent on query plan requests to indicate which query capabilities the
    /// client supports. The backend uses this to shape its response.
    pub supported_query_features: Option<Cow<'static, str>>,
}

impl CosmosRequestHeaders {
    /// Creates an empty `CosmosRequestHeaders`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Writes allowed request headers into the provided HTTP header map.
    pub(crate) fn write_to_headers(&self, headers: &mut Headers) {
        if let Some(activity_id) = self.activity_id.as_ref() {
            headers.insert(
                request_header_names::ACTIVITY_ID,
                HeaderValue::from(activity_id.as_str().to_owned()),
            );
        }
        if let Some(session_token) = self.session_token.as_ref() {
            headers.insert(
                request_header_names::SESSION_TOKEN,
                HeaderValue::from(session_token.as_str().to_owned()),
            );
        }
        if let Some(precondition) = self.precondition.as_ref() {
            match precondition {
                Precondition::IfMatch(etag) => headers.insert(
                    request_header_names::IF_MATCH,
                    HeaderValue::from(etag.to_string()),
                ),
                Precondition::IfNoneMatch(etag) => headers.insert(
                    request_header_names::IF_NONE_MATCH,
                    HeaderValue::from(etag.to_string()),
                ),
            }
        }
        if let Some(throughput) = self.offer_throughput {
            headers.insert(
                request_header_names::OFFER_THROUGHPUT,
                HeaderValue::from(throughput.to_string()),
            );
        }
        if let Some(autopilot) = self.offer_autopilot_settings.as_ref() {
            if let Ok(json) = serde_json::to_string(autopilot) {
                headers.insert(
                    request_header_names::OFFER_AUTOPILOT_SETTINGS,
                    HeaderValue::from(json),
                );
            }
        }
        if let Some(count) = self.max_item_count {
            let wire = match count {
                MaxItemCountHint::ServerDecides => "-1".to_string(),
                MaxItemCountHint::Limit(n) => n.get().to_string(),
            };
            headers.insert(
                request_header_names::MAX_ITEM_COUNT,
                HeaderValue::from(wire),
            );
        }
        if self.incremental_feed {
            headers.insert(
                request_header_names::A_IM,
                HeaderValue::from_static(request_header_names::INCREMENTAL_FEED),
            );
        }
        if self.changefeed_wire_format_version {
            headers.insert(
                request_header_names::CHANGEFEED_WIRE_FORMAT_VERSION,
                HeaderValue::from_static(
                    request_header_names::CHANGEFEED_WIRE_FORMAT_VERSION_2021_09_15,
                ),
            );
        }
        if let Some(ref ts) = self.if_modified_since {
            headers.insert(
                request_header_names::IF_MODIFIED_SINCE,
                HeaderValue::from(ts.clone()),
            );
        }
        if let Some(v) = self.populate_index_metrics {
            headers.insert(
                request_header_names::POPULATE_INDEX_METRICS,
                HeaderValue::from_static(if v { "true" } else { "false" }),
            );
        }
        if let Some(v) = self.populate_query_metrics {
            headers.insert(
                request_header_names::POPULATE_QUERY_METRICS,
                HeaderValue::from_static(if v { "true" } else { "false" }),
            );
        }
        if self.enable_cross_partition_query {
            headers.insert(
                request_header_names::ENABLE_CROSS_PARTITION_QUERY,
                HeaderValue::from_static("True"),
            );
        }
        if let Some(features) = self.supported_query_features.as_ref() {
            headers.insert(
                request_header_names::SUPPORTED_QUERY_FEATURES,
                match features {
                    Cow::Borrowed(s) => HeaderValue::from(*s),
                    Cow::Owned(s) => HeaderValue::from(s.clone()),
                },
            );
        }
    }
}

/// Autoscale throughput settings for the `x-ms-cosmos-offer-autopilot-settings` header.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OfferAutoscaleSettings {
    /// Maximum throughput in RU/s for autoscale.
    pub max_throughput: usize,

    /// Auto-upgrade policy for scaling behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_policy: Option<AutoscaleAutoUpgradePolicy>,
}

impl OfferAutoscaleSettings {
    /// Creates autoscale settings with the given maximum throughput.
    pub fn new(max_throughput: usize) -> Self {
        Self {
            max_throughput,
            auto_upgrade_policy: None,
        }
    }

    /// Sets the auto-upgrade policy with the given increment percent.
    pub fn with_increment_percent(mut self, increment_percent: usize) -> Self {
        self.auto_upgrade_policy = Some(AutoscaleAutoUpgradePolicy {
            throughput_policy: Some(AutoscaleThroughputPolicy { increment_percent }),
        });
        self
    }
}

/// Auto-upgrade policy for autoscale throughput.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AutoscaleAutoUpgradePolicy {
    /// Throughput scaling policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_policy: Option<AutoscaleThroughputPolicy>,
}

/// Throughput scaling policy for autoscale.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AutoscaleThroughputPolicy {
    /// Percentage to increment throughput during auto-upgrade.
    pub increment_percent: usize,
}

/// Cosmos-specific headers extracted from HTTP response.
///
/// These headers contain important metadata about the operation including
/// request charges (RU), session tokens, and activity IDs for debugging.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct CosmosResponseHeaders {
    /// Activity ID for request correlation (`x-ms-activity-id`).
    pub activity_id: Option<ActivityId>,

    /// Request charge in Request Units (`x-ms-request-charge`).
    pub request_charge: Option<RequestCharge>,

    /// Session token for session consistency (`x-ms-session-token`).
    pub session_token: Option<SessionToken>,

    /// ETag for optimistic concurrency (`etag`).
    pub etag: Option<Etag>,

    /// Continuation token for pagination (`x-ms-continuation`).
    pub continuation: Option<String>,

    /// Item count in response (`x-ms-item-count`).
    pub item_count: Option<u32>,

    /// Cosmos substatus code (`x-ms-substatus`).
    pub substatus: Option<SubStatusCode>,

    /// Index utilization metrics as a decoded JSON string (`x-ms-cosmos-index-utilization`).
    ///
    /// The service returns this header as a base64-encoded JSON string. This field
    /// contains the decoded JSON. Only populated when the
    /// `x-ms-cosmos-populateindexmetrics` request header is set.
    pub index_metrics: Option<String>,

    /// Query execution metrics (`x-ms-documentdb-query-metrics`).
    ///
    /// Semicolon-delimited key=value pairs. Only populated when the
    /// `x-ms-documentdb-populatequerymetrics` request header is set.
    pub query_metrics: Option<String>,

    /// Server-side request processing duration in milliseconds (`x-ms-request-duration-ms`).
    ///
    /// Non-finite and negative values are filtered during parsing and will be `None`.
    pub server_duration_ms: Option<f64>,

    /// Logical Sequence Number of the resource (`lsn`).
    pub lsn: Option<u64>,

    /// Item Logical Sequence Number (`x-ms-item-lsn`).
    ///
    /// Only returned on item/document operations (create, read, replace, upsert, delete).
    pub item_lsn: Option<u64>,

    /// Owner full name / alternate content path (`x-ms-alt-content-path`).
    ///
    /// Contains the name-based path of the owning collection, e.g. `dbs/mydb/colls/mycoll`.
    /// Will be used for container identity validation in follow-up work.
    #[allow(dead_code)] // Used in follow-up PR for container identity validation
    pub(crate) owner_full_name: Option<String>,

    /// Owner resource ID / content path (`x-ms-content-path`).
    ///
    /// Contains the RID of the owning collection. Will be used for
    /// RID mismatch validation in container-recreate detection.
    #[allow(dead_code)] // Used in follow-up PR for RID validation
    pub(crate) owner_id: Option<String>,

    /// Indicates whether an offer replace is still pending (`x-ms-offer-replace-pending`).
    ///
    /// When `true`, a throughput change is still being processed asynchronously.
    pub offer_replace_pending: Option<bool>,

    /// Retry-after duration in milliseconds (`x-ms-retry-after-ms`).
    ///
    /// Returned on 429 (Too Many Requests) responses to indicate how long
    /// the client should wait before retrying.
    pub retry_after_ms: Option<u64>,

    /// Correlated activity ID (`x-ms-cosmos-correlated-activityid`).
    ///
    /// Links related operations across service boundaries for distributed tracing.
    pub correlated_activity_id: Option<String>,

    /// Transport-level request ID (`x-ms-transport-request-id`).
    pub transport_request_id: Option<u32>,

    /// Global committed LSN across all regions (`x-ms-global-committed-lsn`).
    pub global_committed_lsn: Option<i64>,

    /// Quorum-acknowledged LSN (`x-ms-quorum-acked-lsn`).
    pub quorum_acked_lsn: Option<i64>,

    /// Quorum-acknowledged local LSN (`x-ms-cosmos-quorum-acked-llsn`).
    pub quorum_acked_local_lsn: Option<i64>,

    /// Local LSN of the partition (`x-ms-cosmos-llsn`).
    pub local_lsn: Option<u64>,

    /// Item-level local LSN (`x-ms-cosmos-item-llsn`).
    pub item_local_lsn: Option<u64>,

    /// Number of read regions (`x-ms-number-of-read-regions`).
    pub number_of_read_regions: Option<u32>,

    /// Timestamp of the last state change (`x-ms-last-state-change-utc`).
    pub last_state_change_utc: Option<String>,

    /// Gateway version (`x-ms-gatewayversion`).
    pub gateway_version: Option<String>,

    /// Service version (`x-ms-serviceversion`).
    pub service_version: Option<String>,

    /// Resource quota information (`x-ms-resource-quota`).
    pub resource_quota: Option<String>,

    /// Resource usage information (`x-ms-resource-usage`).
    pub resource_usage: Option<String>,

    /// Whether the region has tentative (not yet committed) writes (`x-ms-cosmos-allow-tentative-writes`).
    pub has_tentative_writes: Option<bool>,

    /// Partition key range ID for the responding partition
    /// (`x-ms-documentdb-partitionkeyrangeid`).
    ///
    /// Identifies which physical partition handled the operation. For
    /// informational and diagnostic purposes only — clients should not use
    /// this value to route subsequent requests, as the topology may change
    /// (split / merge) without notice.
    pub partition_key_range_id: Option<String>,

    /// Internal partition ID (`x-ms-cosmos-internal-partition-id`).
    ///
    /// Opaque identifier assigned by the service for diagnostic correlation.
    /// May change without notice — do not depend on its format or stability.
    pub internal_partition_id: Option<String>,

    /// Stored procedure log output (`x-ms-documentdb-script-log-results`).
    pub log_results: Option<String>,

    /// Collection index transformation progress percentage (`x-ms-documentdb-collection-index-transformation-progress`).
    pub collection_index_transformation_progress: Option<i64>,

    /// Collection lazy indexing progress percentage (`x-ms-documentdb-collection-lazy-indexing-progress`).
    pub collection_lazy_indexing_progress: Option<i64>,

    /// Distributed transaction idempotency token returned by the coordinator
    /// (`x-ms-cosmos-idempotency-token`).
    #[cfg(feature = "preview_dtx")]
    pub distributed_transaction_idempotency_token: Option<uuid::Uuid>,
}

impl CosmosResponseHeaders {
    /// Creates an empty `CosmosResponseHeaders`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Extracts Cosmos headers from HTTP response headers.
    ///
    /// This parses standard Cosmos headers into typed fields for easy access.
    /// The `index_metrics` field is base64-decoded from the raw header value.
    ///
    /// This is part of the public API to allow cross-crate access from `azure_data_cosmos`.
    pub fn from_headers(headers: &Headers) -> Self {
        let mut result = Self::default();
        for (name, value) in headers.iter() {
            match name.as_str() {
                response_header_names::ACTIVITY_ID => {
                    result.activity_id = Some(ActivityId::from_string(value.as_str().to_owned()));
                }
                response_header_names::REQUEST_CHARGE => {
                    result.request_charge =
                        value.as_str().parse::<f64>().ok().map(RequestCharge::new);
                }
                response_header_names::SESSION_TOKEN => {
                    result.session_token = Some(SessionToken::new(value.as_str().to_owned()));
                }
                response_header_names::ETAG => {
                    result.etag = Some(Etag::from(value.as_str().to_owned()));
                }
                response_header_names::CONTINUATION => {
                    result.continuation = Some(value.as_str().to_owned());
                }
                response_header_names::ITEM_COUNT => {
                    result.item_count = value.as_str().parse().ok();
                }
                response_header_names::SUBSTATUS => {
                    result.substatus = SubStatusCode::from_header_value(value.as_str());
                }
                response_header_names::INDEX_METRICS => {
                    result.index_metrics = match STANDARD.decode(value.as_str()) {
                        Ok(bytes) => match String::from_utf8(bytes) {
                            Ok(s) => Some(s),
                            Err(e) => {
                                tracing::warn!(
                                    header = response_header_names::INDEX_METRICS,
                                    error = %e,
                                    "Failed to UTF-8 decode index metrics after base64 decode"
                                );
                                None
                            }
                        },
                        Err(e) => {
                            tracing::warn!(
                                header = response_header_names::INDEX_METRICS,
                                error = %e,
                                "Failed to base64-decode index metrics header"
                            );
                            None
                        }
                    };
                }
                response_header_names::QUERY_METRICS => {
                    result.query_metrics = Some(value.as_str().to_owned());
                }
                response_header_names::SERVER_DURATION_MS => {
                    result.server_duration_ms = value
                        .as_str()
                        .parse::<f64>()
                        .ok()
                        .filter(|v| v.is_finite() && *v >= 0.0);
                }
                response_header_names::LSN => {
                    result.lsn = value.as_str().parse().ok();
                }
                response_header_names::ITEM_LSN => {
                    result.item_lsn = value.as_str().parse().ok();
                }
                response_header_names::OWNER_FULL_NAME => {
                    result.owner_full_name = Some(value.as_str().to_owned());
                }
                response_header_names::OWNER_ID => {
                    result.owner_id = Some(value.as_str().to_owned());
                }
                response_header_names::OFFER_REPLACE_PENDING => {
                    result.offer_replace_pending = parse_bool_ci(value.as_str());
                }
                response_header_names::RETRY_AFTER_MS => {
                    result.retry_after_ms = value.as_str().parse().ok();
                }
                response_header_names::CORRELATED_ACTIVITY_ID => {
                    result.correlated_activity_id = Some(value.as_str().to_owned());
                }
                response_header_names::TRANSPORT_REQUEST_ID => {
                    result.transport_request_id = value.as_str().parse().ok();
                }
                response_header_names::GLOBAL_COMMITTED_LSN => {
                    result.global_committed_lsn = value.as_str().parse().ok();
                }
                response_header_names::QUORUM_ACKED_LSN => {
                    result.quorum_acked_lsn = value.as_str().parse().ok();
                }
                response_header_names::QUORUM_ACKED_LOCAL_LSN => {
                    result.quorum_acked_local_lsn = value.as_str().parse().ok();
                }
                response_header_names::LOCAL_LSN => {
                    result.local_lsn = value.as_str().parse().ok();
                }
                response_header_names::ITEM_LOCAL_LSN => {
                    result.item_local_lsn = value.as_str().parse().ok();
                }
                response_header_names::NUMBER_OF_READ_REGIONS => {
                    result.number_of_read_regions = value.as_str().parse().ok();
                }
                response_header_names::LAST_STATE_CHANGE_UTC => {
                    result.last_state_change_utc = Some(value.as_str().to_owned());
                }
                response_header_names::GATEWAY_VERSION => {
                    result.gateway_version = Some(value.as_str().to_owned());
                }
                response_header_names::SERVICE_VERSION => {
                    result.service_version = Some(value.as_str().to_owned());
                }
                response_header_names::RESOURCE_QUOTA => {
                    result.resource_quota = Some(value.as_str().to_owned());
                }
                response_header_names::RESOURCE_USAGE => {
                    result.resource_usage = Some(value.as_str().to_owned());
                }
                response_header_names::HAS_TENTATIVE_WRITES => {
                    result.has_tentative_writes = parse_bool_ci(value.as_str());
                }
                response_header_names::PARTITION_KEY_RANGE_ID => {
                    result.partition_key_range_id = Some(value.as_str().to_owned());
                }
                response_header_names::INTERNAL_PARTITION_ID => {
                    result.internal_partition_id = Some(value.as_str().to_owned());
                }
                response_header_names::LOG_RESULTS => {
                    result.log_results = Some(value.as_str().to_owned());
                }
                response_header_names::COLLECTION_INDEX_TRANSFORMATION_PROGRESS => {
                    result.collection_index_transformation_progress = value.as_str().parse().ok();
                }
                response_header_names::COLLECTION_LAZY_INDEXING_PROGRESS => {
                    result.collection_lazy_indexing_progress = value.as_str().parse().ok();
                }
                #[cfg(feature = "preview_dtx")]
                response_header_names::DTX_IDEMPOTENCY_TOKEN => {
                    result.distributed_transaction_idempotency_token =
                        uuid::Uuid::parse_str(value.as_str()).ok();
                }
                _ => {}
            }
        }
        result
    }

    /// Reconstructs an [`azure_core::http::headers::Headers`] from this
    /// typed projection. Inverse of [`from_headers`](Self::from_headers).
    ///
    /// Used at the SDK boundary so that an [`azure_core::Error`] minted
    /// from a Cosmos `CosmosError` carries a usable `raw_response.headers()`
    /// for callers that consume the foundation error type without
    /// downcasting back to the typed Cosmos surface.
    ///
    /// Only fields that were populated by [`from_headers`](Self::from_headers)
    /// round-trip — fields that were never set (`None`) are omitted from
    /// the output, matching the on-wire absence of the corresponding
    /// header.
    ///
    /// String formatting follows the on-wire conventions:
    ///
    /// * Numbers (`u32`, `u64`, `i64`, `f64`) use their natural `Display`
    ///   representation.
    /// * Booleans are emitted as Pascal-case `"True"` / `"False"` because
    ///   that is what real Cosmos DB sends (matching the case-insensitive
    ///   parser in `from_headers`).
    /// * `index_metrics` is **re-encoded to base64** because the on-wire
    ///   header is base64-encoded JSON.
    pub fn to_raw_headers(&self) -> Headers {
        use azure_core::http::headers::HeaderName;

        let mut h = Headers::new();
        // Closure: insert `name` → `value` (stringified) when `value` is `Some`.
        // The lambda form keeps each call site to one line and avoids
        // re-typing the `HeaderName::from_static` wrapper.
        let mut put_str = |name: &'static str, value: Option<String>| {
            if let Some(v) = value {
                h.insert(HeaderName::from_static(name), HeaderValue::from(v));
            }
        };
        let bool_to_wire = |b: bool| if b { "True" } else { "False" };

        put_str(
            response_header_names::ACTIVITY_ID,
            self.activity_id.as_ref().map(ToString::to_string),
        );
        put_str(
            response_header_names::REQUEST_CHARGE,
            self.request_charge.as_ref().map(ToString::to_string),
        );
        put_str(
            response_header_names::SESSION_TOKEN,
            self.session_token.as_ref().map(ToString::to_string),
        );
        put_str(
            response_header_names::ETAG,
            self.etag.as_ref().map(ToString::to_string),
        );
        put_str(
            response_header_names::CONTINUATION,
            self.continuation.clone(),
        );
        put_str(
            response_header_names::ITEM_COUNT,
            self.item_count.map(|v| v.to_string()),
        );
        put_str(
            response_header_names::SUBSTATUS,
            self.substatus.map(|s| s.value().to_string()),
        );
        // `index_metrics` is stored decoded; re-encode to match the on-wire
        // base64 form so a parser round-trips correctly.
        put_str(
            response_header_names::INDEX_METRICS,
            self.index_metrics.as_deref().map(|s| STANDARD.encode(s)),
        );
        put_str(
            response_header_names::QUERY_METRICS,
            self.query_metrics.clone(),
        );
        put_str(
            response_header_names::SERVER_DURATION_MS,
            self.server_duration_ms.map(|v| v.to_string()),
        );
        put_str(response_header_names::LSN, self.lsn.map(|v| v.to_string()));
        put_str(
            response_header_names::ITEM_LSN,
            self.item_lsn.map(|v| v.to_string()),
        );
        put_str(
            response_header_names::OWNER_FULL_NAME,
            self.owner_full_name.clone(),
        );
        put_str(response_header_names::OWNER_ID, self.owner_id.clone());
        put_str(
            response_header_names::OFFER_REPLACE_PENDING,
            self.offer_replace_pending
                .map(|b| bool_to_wire(b).to_owned()),
        );
        put_str(
            response_header_names::RETRY_AFTER_MS,
            self.retry_after_ms.map(|v| v.to_string()),
        );
        put_str(
            response_header_names::CORRELATED_ACTIVITY_ID,
            self.correlated_activity_id.clone(),
        );
        put_str(
            response_header_names::TRANSPORT_REQUEST_ID,
            self.transport_request_id.map(|v| v.to_string()),
        );
        put_str(
            response_header_names::GLOBAL_COMMITTED_LSN,
            self.global_committed_lsn.map(|v| v.to_string()),
        );
        put_str(
            response_header_names::QUORUM_ACKED_LSN,
            self.quorum_acked_lsn.map(|v| v.to_string()),
        );
        put_str(
            response_header_names::QUORUM_ACKED_LOCAL_LSN,
            self.quorum_acked_local_lsn.map(|v| v.to_string()),
        );
        put_str(
            response_header_names::LOCAL_LSN,
            self.local_lsn.map(|v| v.to_string()),
        );
        put_str(
            response_header_names::ITEM_LOCAL_LSN,
            self.item_local_lsn.map(|v| v.to_string()),
        );
        put_str(
            response_header_names::NUMBER_OF_READ_REGIONS,
            self.number_of_read_regions.map(|v| v.to_string()),
        );
        put_str(
            response_header_names::LAST_STATE_CHANGE_UTC,
            self.last_state_change_utc.clone(),
        );
        put_str(
            response_header_names::GATEWAY_VERSION,
            self.gateway_version.clone(),
        );
        put_str(
            response_header_names::SERVICE_VERSION,
            self.service_version.clone(),
        );
        put_str(
            response_header_names::RESOURCE_QUOTA,
            self.resource_quota.clone(),
        );
        put_str(
            response_header_names::RESOURCE_USAGE,
            self.resource_usage.clone(),
        );
        put_str(
            response_header_names::HAS_TENTATIVE_WRITES,
            self.has_tentative_writes
                .map(|b| bool_to_wire(b).to_owned()),
        );
        put_str(
            response_header_names::PARTITION_KEY_RANGE_ID,
            self.partition_key_range_id.clone(),
        );
        put_str(
            response_header_names::INTERNAL_PARTITION_ID,
            self.internal_partition_id.clone(),
        );
        put_str(response_header_names::LOG_RESULTS, self.log_results.clone());
        put_str(
            response_header_names::COLLECTION_INDEX_TRANSFORMATION_PROGRESS,
            self.collection_index_transformation_progress
                .map(|v| v.to_string()),
        );
        put_str(
            response_header_names::COLLECTION_LAZY_INDEXING_PROGRESS,
            self.collection_lazy_indexing_progress
                .map(|v| v.to_string()),
        );
        #[cfg(feature = "preview_dtx")]
        put_str(
            response_header_names::DTX_IDEMPOTENCY_TOKEN,
            self.distributed_transaction_idempotency_token
                .map(|value| value.to_string()),
        );
        h
    }
}

/// Parses a boolean header value, accepting `"true"` / `"false"` case-insensitively.
///
/// Cosmos response headers (e.g. `x-ms-offer-replace-pending`) historically use
/// Pascal-case (`"True"`/`"False"`) on the wire, while `bool::FromStr` only
/// accepts strict lowercase. Returns `None` for any value other than the two
/// recognized tokens so the field stays absent on malformed input.
fn parse_bool_ci(s: &str) -> Option<bool> {
    if s.eq_ignore_ascii_case("true") {
        Some(true)
    } else if s.eq_ignore_ascii_case("false") {
        Some(false)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::http::headers::HeaderName;

    #[test]
    fn cosmos_response_headers_from_azure_headers() {
        let mut headers = Headers::new();
        headers.insert("x-ms-activity-id", "abc-123");
        headers.insert("x-ms-request-charge", "5.67");
        headers.insert("x-ms-session-token", "session:456");
        headers.insert("x-ms-substatus", "3200");
        headers.insert("etag", "\"version-1\"");
        headers.insert("x-ms-continuation", "next-page-token");
        headers.insert("x-ms-item-count", "10");
        headers.insert(
            "x-ms-cosmos-index-utilization",
            // base64 of r#"{"UtilizedSingleIndexes":[]}"#
            // cspell:disable-next-line
            "eyJVdGlsaXplZFNpbmdsZUluZGV4ZXMiOltdfQ==",
        );
        headers.insert(
            "x-ms-documentdb-query-metrics",
            "totalExecutionTimeInMs=1.23;queryCompileTimeInMs=0.01",
        );
        headers.insert("x-ms-request-duration-ms", "4.56");
        headers.insert("lsn", "42");
        headers.insert("x-ms-item-lsn", "37");
        headers.insert("x-ms-retry-after-ms", "1000");
        headers.insert("x-ms-cosmos-correlated-activityid", "corr-456");
        headers.insert("x-ms-transport-request-id", "99");
        headers.insert("x-ms-global-committed-lsn", "50");
        headers.insert("x-ms-quorum-acked-lsn", "48");
        headers.insert("x-ms-cosmos-quorum-acked-llsn", "47");
        headers.insert("x-ms-cosmos-llsn", "51");
        headers.insert("x-ms-cosmos-item-llsn", "39");
        headers.insert("x-ms-number-of-read-regions", "2");
        headers.insert("x-ms-last-state-change-utc", "2024-01-01T00:00:00Z");
        headers.insert("x-ms-gatewayversion", "2.18.0");
        headers.insert("x-ms-serviceversion", "version 2.18.0");
        headers.insert("x-ms-resource-quota", "documentSize=10240;");
        headers.insert("x-ms-resource-usage", "documentSize=0;");
        headers.insert("x-ms-cosmos-allow-tentative-writes", "true");
        headers.insert("x-ms-documentdb-partitionkeyrangeid", "0");
        // cspell:disable-next-line
        headers.insert("x-ms-documentdb-script-log-results", "logoutput");
        headers.insert(
            "x-ms-documentdb-collection-index-transformation-progress",
            "100",
        );
        headers.insert("x-ms-documentdb-collection-lazy-indexing-progress", "75");

        let cosmos_headers = CosmosResponseHeaders::from_headers(&headers);

        assert_eq!(
            cosmos_headers.activity_id.as_ref().map(|a| a.as_str()),
            Some("abc-123")
        );
        assert!((cosmos_headers.request_charge.unwrap().value() - 5.67).abs() < f64::EPSILON);
        assert_eq!(
            cosmos_headers
                .session_token
                .as_ref()
                .map(SessionToken::as_str),
            Some("session:456")
        );
        assert_eq!(
            cosmos_headers.etag.as_ref().map(|e| -> &str { e.as_ref() }),
            Some("\"version-1\"")
        );
        assert_eq!(
            cosmos_headers.continuation.as_deref(),
            Some("next-page-token")
        );
        assert_eq!(cosmos_headers.item_count, Some(10));
        assert_eq!(cosmos_headers.substatus, Some(SubStatusCode::new(3200)));
        assert_eq!(
            cosmos_headers.index_metrics.as_deref(),
            Some(r#"{"UtilizedSingleIndexes":[]}"#)
        );
        assert_eq!(
            cosmos_headers.query_metrics.as_deref(),
            Some("totalExecutionTimeInMs=1.23;queryCompileTimeInMs=0.01")
        );
        assert!((cosmos_headers.server_duration_ms.unwrap() - 4.56).abs() < f64::EPSILON);
        assert_eq!(cosmos_headers.lsn, Some(42));
        assert_eq!(cosmos_headers.item_lsn, Some(37));
        assert_eq!(cosmos_headers.retry_after_ms, Some(1000));
        assert_eq!(
            cosmos_headers.correlated_activity_id.as_deref(),
            Some("corr-456")
        );
        assert_eq!(cosmos_headers.transport_request_id, Some(99));
        assert_eq!(cosmos_headers.global_committed_lsn, Some(50));
        assert_eq!(cosmos_headers.quorum_acked_lsn, Some(48));
        assert_eq!(cosmos_headers.quorum_acked_local_lsn, Some(47));
        assert_eq!(cosmos_headers.local_lsn, Some(51));
        assert_eq!(cosmos_headers.item_local_lsn, Some(39));
        assert_eq!(cosmos_headers.number_of_read_regions, Some(2));
        assert_eq!(
            cosmos_headers.last_state_change_utc.as_deref(),
            Some("2024-01-01T00:00:00Z")
        );
        assert_eq!(cosmos_headers.gateway_version.as_deref(), Some("2.18.0"));
        assert_eq!(
            cosmos_headers.service_version.as_deref(),
            Some("version 2.18.0")
        );
        assert_eq!(
            cosmos_headers.resource_quota.as_deref(),
            Some("documentSize=10240;")
        );
        assert_eq!(
            cosmos_headers.resource_usage.as_deref(),
            Some("documentSize=0;")
        );
        assert_eq!(cosmos_headers.has_tentative_writes, Some(true));
        assert_eq!(cosmos_headers.partition_key_range_id.as_deref(), Some("0"));
        assert_eq!(
            cosmos_headers.log_results.as_deref(),
            // cspell:disable-next-line
            Some("logoutput")
        );
        assert_eq!(
            cosmos_headers.collection_index_transformation_progress,
            Some(100)
        );
        assert_eq!(cosmos_headers.collection_lazy_indexing_progress, Some(75));
    }

    #[test]
    fn non_finite_server_duration_returns_none() {
        for value in ["NaN", "inf", "-inf", "Infinity", "-Infinity", "-1.0"] {
            let mut headers = Headers::new();
            headers.insert("x-ms-request-duration-ms", value);
            let cosmos_headers = CosmosResponseHeaders::from_headers(&headers);
            assert!(
                cosmos_headers.server_duration_ms.is_none(),
                "Expected None for '{value}'"
            );
        }
    }

    #[test]
    fn invalid_base64_index_metrics_returns_none() {
        let mut headers = Headers::new();
        headers.insert("x-ms-cosmos-index-utilization", "not-valid-base64!!!");
        let cosmos_headers = CosmosResponseHeaders::from_headers(&headers);
        assert!(cosmos_headers.index_metrics.is_none());
    }

    #[test]
    fn cosmos_response_headers_builder_pattern() {
        let headers = CosmosResponseHeaders {
            activity_id: Some(ActivityId::from_string("test".to_string())),
            request_charge: Some(RequestCharge::new(2.0)),
            session_token: Some(SessionToken::new("token".to_string())),
            etag: Some(Etag::from("etag".to_string())),
            continuation: Some("cont".to_string()),
            item_count: Some(5),
            substatus: Some(SubStatusCode::new(1002)),
            index_metrics: Some(r#"{"metrics":"data"}"#.to_string()),
            query_metrics: Some("totalExecutionTimeInMs=1.0".to_string()),
            server_duration_ms: Some(4.56),
            lsn: Some(100),
            item_lsn: Some(99),
            owner_full_name: Some("dbs/db1/colls/c1".to_string()),
            owner_id: Some("rid1".to_string()),
            offer_replace_pending: None,
            ..Default::default()
        };

        assert_eq!(
            headers.activity_id.as_ref().map(|a| a.as_str()),
            Some("test")
        );
        assert_eq!(headers.request_charge, Some(RequestCharge::new(2.0)));
        assert_eq!(
            headers.session_token.as_ref().map(SessionToken::as_str),
            Some("token")
        );
        assert_eq!(
            headers.etag.as_ref().map(|e| -> &str { e.as_ref() }),
            Some("etag")
        );
        assert_eq!(headers.continuation.as_deref(), Some("cont"));
        assert_eq!(headers.item_count, Some(5));
        assert_eq!(headers.substatus, Some(SubStatusCode::new(1002)));
    }

    #[test]
    fn cosmos_response_headers_default_empty() {
        let headers = CosmosResponseHeaders::default();

        assert!(headers.activity_id.is_none());
        assert!(headers.request_charge.is_none());
        assert!(headers.session_token.is_none());
        assert!(headers.etag.is_none());
        assert!(headers.continuation.is_none());
        assert!(headers.item_count.is_none());
        assert!(headers.substatus.is_none());
        assert!(headers.index_metrics.is_none());
        assert!(headers.query_metrics.is_none());
        assert!(headers.server_duration_ms.is_none());
        assert!(headers.lsn.is_none());
        assert!(headers.item_lsn.is_none());
        assert!(headers.partition_key_range_id.is_none());
        assert!(headers.internal_partition_id.is_none());
        assert!(headers.retry_after_ms.is_none());
        assert!(headers.correlated_activity_id.is_none());
        assert!(headers.transport_request_id.is_none());
        assert!(headers.global_committed_lsn.is_none());
        assert!(headers.quorum_acked_lsn.is_none());
        assert!(headers.quorum_acked_local_lsn.is_none());
        assert!(headers.local_lsn.is_none());
        assert!(headers.item_local_lsn.is_none());
        assert!(headers.number_of_read_regions.is_none());
        assert!(headers.last_state_change_utc.is_none());
        assert!(headers.gateway_version.is_none());
        assert!(headers.service_version.is_none());
        assert!(headers.resource_quota.is_none());
        assert!(headers.resource_usage.is_none());
        assert!(headers.has_tentative_writes.is_none());
        assert!(headers.log_results.is_none());
        assert!(headers.collection_index_transformation_progress.is_none());
        assert!(headers.collection_lazy_indexing_progress.is_none());
    }

    #[test]
    fn cosmos_request_headers_builder_pattern() {
        let headers = CosmosRequestHeaders {
            activity_id: Some(ActivityId::from_string("test-request".to_string())),
            session_token: Some(SessionToken::new("session-token".to_string())),
            ..Default::default()
        };

        assert_eq!(
            headers.activity_id.as_ref().map(ActivityId::as_str),
            Some("test-request")
        );
        assert_eq!(
            headers.session_token.as_ref().map(SessionToken::as_str),
            Some("session-token")
        );
    }

    #[test]
    fn cosmos_request_headers_write_to_headers() {
        let cosmos_headers = CosmosRequestHeaders {
            activity_id: Some(ActivityId::from_string("test-request".to_string())),
            session_token: Some(SessionToken::new("session-token".to_string())),
            ..Default::default()
        };
        let mut headers = Headers::new();

        cosmos_headers.write_to_headers(&mut headers);

        assert_eq!(
            headers.get_optional_str(&HeaderName::from_static("x-ms-activity-id")),
            Some("test-request")
        );
        assert_eq!(
            headers.get_optional_str(&HeaderName::from_static("x-ms-session-token")),
            Some("session-token")
        );
    }

    #[test]
    fn write_to_headers_precondition_if_match() {
        let cosmos_headers = CosmosRequestHeaders {
            precondition: Some(Precondition::if_match(Etag::from("etag-value-1"))),
            ..Default::default()
        };
        let mut headers = Headers::new();

        cosmos_headers.write_to_headers(&mut headers);

        assert_eq!(
            headers.get_optional_str(&HeaderName::from_static("if-match")),
            Some("etag-value-1")
        );
        assert_eq!(
            headers.get_optional_str(&HeaderName::from_static("if-none-match")),
            None
        );
    }

    #[test]
    fn write_to_headers_precondition_if_none_match() {
        let cosmos_headers = CosmosRequestHeaders {
            precondition: Some(Precondition::if_none_match(Etag::from("*"))),
            ..Default::default()
        };
        let mut headers = Headers::new();

        cosmos_headers.write_to_headers(&mut headers);

        assert_eq!(
            headers.get_optional_str(&HeaderName::from_static("if-none-match")),
            Some("*")
        );
        assert_eq!(
            headers.get_optional_str(&HeaderName::from_static("if-match")),
            None
        );
    }

    #[test]
    fn write_to_headers_no_precondition_omits_both_headers() {
        let cosmos_headers = CosmosRequestHeaders::default();
        let mut headers = Headers::new();

        cosmos_headers.write_to_headers(&mut headers);

        assert_eq!(
            headers.get_optional_str(&HeaderName::from_static("if-match")),
            None
        );
        assert_eq!(
            headers.get_optional_str(&HeaderName::from_static("if-none-match")),
            None
        );
    }

    #[test]
    fn write_to_headers_all_fields() {
        let cosmos_headers = CosmosRequestHeaders {
            activity_id: Some(ActivityId::from_string("corr-id-1".to_string())),
            session_token: Some(SessionToken::new("session:100".to_string())),
            precondition: Some(Precondition::if_match(Etag::from("etag-abc"))),
            ..Default::default()
        };
        let mut headers = Headers::new();

        cosmos_headers.write_to_headers(&mut headers);

        assert_eq!(
            headers.get_optional_str(&HeaderName::from_static("x-ms-activity-id")),
            Some("corr-id-1")
        );
        assert_eq!(
            headers.get_optional_str(&HeaderName::from_static("x-ms-session-token")),
            Some("session:100")
        );
        assert_eq!(
            headers.get_optional_str(&HeaderName::from_static("if-match")),
            Some("etag-abc")
        );
        assert_eq!(
            headers.get_optional_str(&HeaderName::from_static("if-none-match")),
            None
        );
    }
    #[test]
    fn offer_replace_pending_parses_case_insensitively() {
        for v in ["true", "True", "TRUE", "tRuE"] {
            let mut h = Headers::new();
            h.insert(response_header_names::OFFER_REPLACE_PENDING, v.to_owned());
            assert_eq!(
                CosmosResponseHeaders::from_headers(&h).offer_replace_pending,
                Some(true),
                "{v:?} should parse as Some(true)"
            );
        }
        for v in ["false", "False", "FALSE"] {
            let mut h = Headers::new();
            h.insert(response_header_names::OFFER_REPLACE_PENDING, v.to_owned());
            assert_eq!(
                CosmosResponseHeaders::from_headers(&h).offer_replace_pending,
                Some(false),
                "{v:?} should parse as Some(false)"
            );
        }
        for v in ["yes", "1", "garbage", ""] {
            let mut h = Headers::new();
            h.insert(response_header_names::OFFER_REPLACE_PENDING, v.to_owned());
            assert_eq!(
                CosmosResponseHeaders::from_headers(&h).offer_replace_pending,
                None,
                "{v:?} should not parse"
            );
        }
    }

    #[test]
    fn write_to_headers_emits_max_item_count() {
        let cosmos_headers = CosmosRequestHeaders {
            max_item_count: Some(MaxItemCountHint::Limit(NonZeroU32::new(7).unwrap())),
            ..Default::default()
        };
        let mut headers = Headers::new();
        cosmos_headers.write_to_headers(&mut headers);
        assert_eq!(
            headers.get_optional_str(&HeaderName::from_static("x-ms-max-item-count")),
            Some("7")
        );
    }

    #[test]
    fn write_to_headers_omits_max_item_count_when_none() {
        let cosmos_headers = CosmosRequestHeaders::default();
        let mut headers = Headers::new();
        cosmos_headers.write_to_headers(&mut headers);
        assert_eq!(
            headers.get_optional_str(&HeaderName::from_static("x-ms-max-item-count")),
            None
        );
    }

    /// Round-trips a fully-populated [`CosmosResponseHeaders`] through
    /// [`to_raw_headers`](CosmosResponseHeaders::to_raw_headers) followed
    /// by [`from_headers`](CosmosResponseHeaders::from_headers) and
    /// asserts every public field is preserved.
    ///
    /// Pins the on-wire encoding contracts the `From<CosmosError> for
    /// azure_core::Error` boundary relies on:
    /// * Numeric fields format via `Display` (no unexpected locale / precision drift).
    /// * Booleans round-trip via Pascal-case `"True"` / `"False"`.
    /// * `index_metrics` re-encodes to base64 so the parser sees the same
    ///   on-wire shape it would from the real service.
    /// * `None` fields are not emitted (no stray empty-string headers).
    #[test]
    fn to_raw_headers_round_trips_through_from_headers() {
        let original = CosmosResponseHeaders {
            activity_id: Some(ActivityId::from_string("abc-123".into())),
            request_charge: Some(RequestCharge::new(5.67)),
            session_token: Some(SessionToken::new("0:1#100")),
            etag: Some(Etag::from("\"v1\"")),
            continuation: Some("next-page".into()),
            item_count: Some(10),
            substatus: Some(SubStatusCode::THROTTLE_DUE_TO_SPLIT),
            index_metrics: Some("{\"UtilizedSingleIndexes\":[]}".into()),
            query_metrics: Some("totalExecutionTimeInMs=1.23".into()),
            server_duration_ms: Some(4.5),
            lsn: Some(42),
            item_lsn: Some(37),
            owner_full_name: Some("dbs/d/colls/c".into()),
            owner_id: Some("rid-xyz".into()),
            offer_replace_pending: Some(true),
            retry_after_ms: Some(1000),
            correlated_activity_id: Some("corr-456".into()),
            transport_request_id: Some(99),
            global_committed_lsn: Some(50),
            quorum_acked_lsn: Some(48),
            quorum_acked_local_lsn: Some(47),
            local_lsn: Some(51),
            item_local_lsn: Some(39),
            number_of_read_regions: Some(2),
            last_state_change_utc: Some("2024-01-01T00:00:00Z".into()),
            gateway_version: Some("2.18.0".into()),
            service_version: Some("version 2.18.0".into()),
            resource_quota: Some("documentSize=10240;".into()),
            resource_usage: Some("documentSize=0;".into()),
            has_tentative_writes: Some(false),
            partition_key_range_id: Some("0".into()),
            internal_partition_id: Some("internal-xyz".into()),
            log_results: Some("ok".into()),
            collection_index_transformation_progress: Some(100),
            collection_lazy_indexing_progress: Some(75),
            #[cfg(feature = "preview_dtx")]
            distributed_transaction_idempotency_token: Some(uuid::Uuid::new_v4()),
        };

        let raw = original.to_raw_headers();
        // Pascal-case wire form for booleans — matches what real Cosmos
        // sends and what the case-insensitive parser accepts.
        assert_eq!(
            raw.get_optional_str(&HeaderName::from_static(
                response_header_names::OFFER_REPLACE_PENDING
            )),
            Some("True")
        );
        assert_eq!(
            raw.get_optional_str(&HeaderName::from_static(
                response_header_names::HAS_TENTATIVE_WRITES
            )),
            Some("False")
        );
        // Sub-status is emitted as the bare numeric value.
        assert_eq!(
            raw.get_optional_str(&HeaderName::from_static(response_header_names::SUBSTATUS)),
            Some(SubStatusCode::THROTTLE_DUE_TO_SPLIT.value().to_string()).as_deref()
        );
        // `index_metrics` is base64 of the decoded JSON.
        assert_eq!(
            raw.get_optional_str(&HeaderName::from_static(
                response_header_names::INDEX_METRICS
            )),
            Some(STANDARD.encode("{\"UtilizedSingleIndexes\":[]}")).as_deref()
        );

        let round_tripped = CosmosResponseHeaders::from_headers(&raw);
        assert_eq!(
            round_tripped.activity_id.as_ref().map(|a| a.as_str()),
            original.activity_id.as_ref().map(|a| a.as_str())
        );
        assert!(
            (round_tripped.request_charge.unwrap().value()
                - original.request_charge.unwrap().value())
            .abs()
                < f64::EPSILON
        );
        assert_eq!(
            round_tripped
                .session_token
                .as_ref()
                .map(SessionToken::as_str),
            original.session_token.as_ref().map(SessionToken::as_str)
        );
        assert_eq!(
            round_tripped.etag.as_ref().map(|e| -> &str { e.as_ref() }),
            original.etag.as_ref().map(|e| -> &str { e.as_ref() })
        );
        assert_eq!(round_tripped.continuation, original.continuation);
        assert_eq!(round_tripped.item_count, original.item_count);
        assert_eq!(round_tripped.substatus, original.substatus);
        assert_eq!(round_tripped.index_metrics, original.index_metrics);
        assert_eq!(round_tripped.query_metrics, original.query_metrics);
        assert_eq!(
            round_tripped.server_duration_ms,
            original.server_duration_ms
        );
        assert_eq!(round_tripped.lsn, original.lsn);
        assert_eq!(round_tripped.item_lsn, original.item_lsn);
        assert_eq!(round_tripped.owner_full_name, original.owner_full_name);
        assert_eq!(round_tripped.owner_id, original.owner_id);
        assert_eq!(
            round_tripped.offer_replace_pending,
            original.offer_replace_pending
        );
        assert_eq!(round_tripped.retry_after_ms, original.retry_after_ms);
        assert_eq!(
            round_tripped.correlated_activity_id,
            original.correlated_activity_id
        );
        assert_eq!(
            round_tripped.transport_request_id,
            original.transport_request_id
        );
        assert_eq!(
            round_tripped.global_committed_lsn,
            original.global_committed_lsn
        );
        assert_eq!(round_tripped.quorum_acked_lsn, original.quorum_acked_lsn);
        assert_eq!(
            round_tripped.quorum_acked_local_lsn,
            original.quorum_acked_local_lsn
        );
        assert_eq!(round_tripped.local_lsn, original.local_lsn);
        assert_eq!(round_tripped.item_local_lsn, original.item_local_lsn);
        assert_eq!(
            round_tripped.number_of_read_regions,
            original.number_of_read_regions
        );
        assert_eq!(
            round_tripped.last_state_change_utc,
            original.last_state_change_utc
        );
        assert_eq!(round_tripped.gateway_version, original.gateway_version);
        assert_eq!(round_tripped.service_version, original.service_version);
        assert_eq!(round_tripped.resource_quota, original.resource_quota);
        assert_eq!(round_tripped.resource_usage, original.resource_usage);
        assert_eq!(
            round_tripped.has_tentative_writes,
            original.has_tentative_writes
        );
        assert_eq!(
            round_tripped.partition_key_range_id,
            original.partition_key_range_id
        );
        assert_eq!(
            round_tripped.internal_partition_id,
            original.internal_partition_id
        );
        assert_eq!(round_tripped.log_results, original.log_results);
        assert_eq!(
            round_tripped.collection_index_transformation_progress,
            original.collection_index_transformation_progress
        );
        assert_eq!(
            round_tripped.collection_lazy_indexing_progress,
            original.collection_lazy_indexing_progress
        );
        #[cfg(feature = "preview_dtx")]
        assert_eq!(
            round_tripped.distributed_transaction_idempotency_token,
            original.distributed_transaction_idempotency_token
        );
    }

    /// `to_raw_headers` on a defaulted (empty) value must produce an
    /// empty `Headers` — no stray empty-string headers from `None`
    /// fields.
    #[test]
    fn to_raw_headers_empty_when_all_fields_none() {
        let raw = CosmosResponseHeaders::default().to_raw_headers();
        assert_eq!(raw.iter().count(), 0);
    }
}
