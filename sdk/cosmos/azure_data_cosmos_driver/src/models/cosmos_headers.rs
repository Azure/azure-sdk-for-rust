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
    pub const OFFER_THROUGHPUT: &str = "x-ms-offer-throughput";
    pub const OFFER_AUTOPILOT_SETTINGS: &str = "x-ms-cosmos-offer-autopilot-settings";
    pub const PRIORITY_LEVEL: &str = "x-ms-cosmos-priority-level";
    pub const THROUGHPUT_BUCKET: &str = "x-ms-cosmos-throughput-bucket";
}

/// Standard Cosmos DB response header names.
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
    pub const OWNER_FULL_NAME: &str = "x-ms-alt-content-path";
    pub const OWNER_ID: &str = "x-ms-content-path";
    pub const OFFER_REPLACE_PENDING: &str = "x-ms-offer-replace-pending";
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
                response_header_names::OWNER_FULL_NAME => {
                    result.owner_full_name = Some(value.as_str().to_owned());
                }
                response_header_names::OWNER_ID => {
                    result.owner_id = Some(value.as_str().to_owned());
                }
                response_header_names::OFFER_REPLACE_PENDING => {
                    result.offer_replace_pending = value.as_str().parse::<bool>().ok();
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
            owner_full_name: Some("dbs/db1/colls/c1".to_string()),
            owner_id: Some("rid1".to_string()),
            offer_replace_pending: None,
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
