// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB request/response header models.

use crate::models::{ActivityId, ETag, Precondition, RequestCharge, SessionToken, SubStatusCode};
use azure_core::http::headers::{HeaderValue, Headers};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::Serialize;

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
    pub const PREFER: &str = "prefer";
    pub const IS_UPSERT: &str = "x-ms-documentdb-is-upsert";
    pub const IS_BATCH_REQUEST: &str = "x-ms-cosmos-is-batch-request";
    pub const BATCH_ATOMIC: &str = "x-ms-cosmos-batch-atomic";
    pub const BATCH_CONTINUE_ON_ERROR: &str = "x-ms-cosmos-batch-continue-on-error";
    pub const OFFER_THROUGHPUT: &str = "x-ms-offer-throughput";
    pub const OFFER_AUTOPILOT_SETTINGS: &str = "x-ms-cosmos-offer-autopilot-settings";
    pub const PRIORITY_LEVEL: &str = "x-ms-cosmos-priority-level";
    pub const THROUGHPUT_BUCKET: &str = "x-ms-cosmos-throughput-bucket";
}

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
}

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
                    HeaderValue::from(etag.as_str().to_owned()),
                ),
                Precondition::IfNoneMatch(etag) => headers.insert(
                    request_header_names::IF_NONE_MATCH,
                    HeaderValue::from(etag.as_str().to_owned()),
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
    pub etag: Option<ETag>,

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

    /// Partition key range ID for the responding partition (`x-ms-documentdb-partitionkeyrangeid`).
    pub partition_key_range_id: Option<String>,

    /// Internal partition ID (`x-ms-cosmos-internal-partition-id`).
    pub internal_partition_id: Option<String>,

    /// Stored procedure log output (`x-ms-documentdb-script-log-results`).
    pub log_results: Option<String>,

    /// Collection index transformation progress percentage (`x-ms-documentdb-collection-index-transformation-progress`).
    pub collection_index_transformation_progress: Option<i64>,

    /// Collection lazy indexing progress percentage (`x-ms-documentdb-collection-lazy-indexing-progress`).
    pub collection_lazy_indexing_progress: Option<i64>,
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
                    result.etag = Some(ETag::new(value.as_str().to_owned()));
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
                    result.offer_replace_pending = value.as_str().parse::<bool>().ok();
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
                    result.has_tentative_writes = value.as_str().parse::<bool>().ok();
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
                _ => {}
            }
        }
        result
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
            cosmos_headers.etag.as_ref().map(ETag::as_str),
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
            etag: Some(ETag::new("etag".to_string())),
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
        assert_eq!(headers.etag.as_ref().map(ETag::as_str), Some("etag"));
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
    }

    #[test]
    fn cosmos_request_headers_builder_pattern() {
        let headers = CosmosRequestHeaders {
            activity_id: Some(ActivityId::from_string("test-request".to_string())),
            session_token: Some(SessionToken::new("session-token".to_string())),
            precondition: None,
            offer_throughput: None,
            offer_autopilot_settings: None,
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
            precondition: None,
            offer_throughput: None,
            offer_autopilot_settings: None,
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
            activity_id: None,
            session_token: None,
            precondition: Some(Precondition::if_match(ETag::new("etag-value-1"))),
            offer_throughput: None,
            offer_autopilot_settings: None,
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
            activity_id: None,
            session_token: None,
            precondition: Some(Precondition::if_none_match(ETag::new("*"))),
            offer_throughput: None,
            offer_autopilot_settings: None,
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
        let cosmos_headers = CosmosRequestHeaders {
            activity_id: None,
            session_token: None,
            precondition: None,
            offer_throughput: None,
            offer_autopilot_settings: None,
        };
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
            precondition: Some(Precondition::if_match(ETag::new("etag-abc"))),
            offer_throughput: None,
            offer_autopilot_settings: None,
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
}
