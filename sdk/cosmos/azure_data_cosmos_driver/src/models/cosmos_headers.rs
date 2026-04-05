// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB request/response header models.

use crate::models::{ActivityId, ETag, Precondition, RequestCharge, SessionToken, SubStatusCode};
use azure_core::http::headers::{HeaderValue, Headers};
use base64::{engine::general_purpose::STANDARD, Engine as _};

/// Standard Cosmos DB request header names.
///
/// All names are lowercase as required by [`HeaderName`]. The azure_core [`Headers`]
/// type normalizes header names to lowercase on insertion, so lookups are case-sensitive
/// but will always match since both sides are lowercase.
pub(crate) mod request_header_names {
    use azure_core::http::headers::HeaderName;

    pub static ACTIVITY_ID: HeaderName = HeaderName::from_static("x-ms-activity-id");
    pub static SESSION_TOKEN: HeaderName = HeaderName::from_static("x-ms-session-token");
    pub static IF_MATCH: HeaderName = HeaderName::from_static("if-match");
    pub static IF_NONE_MATCH: HeaderName = HeaderName::from_static("if-none-match");
    pub static PREFER: HeaderName = HeaderName::from_static("prefer");
}

/// Standard Cosmos DB response header names.
pub(crate) mod response_header_names {
    use azure_core::http::headers::HeaderName;

    pub static ACTIVITY_ID: HeaderName = HeaderName::from_static("x-ms-activity-id");
    pub static REQUEST_CHARGE: HeaderName = HeaderName::from_static("x-ms-request-charge");
    pub static SESSION_TOKEN: HeaderName = HeaderName::from_static("x-ms-session-token");
    pub static ETAG: HeaderName = HeaderName::from_static("etag");
    pub static CONTINUATION: HeaderName = HeaderName::from_static("x-ms-continuation");
    pub static ITEM_COUNT: HeaderName = HeaderName::from_static("x-ms-item-count");
    pub static SUBSTATUS: HeaderName = HeaderName::from_static("x-ms-substatus");
    pub static INDEX_METRICS: HeaderName = HeaderName::from_static("x-ms-cosmos-index-utilization");
    pub static QUERY_METRICS: HeaderName = HeaderName::from_static("x-ms-documentdb-query-metrics");
    pub static SERVER_DURATION_MS: HeaderName = HeaderName::from_static("x-ms-request-duration-ms");
    pub static LSN: HeaderName = HeaderName::from_static("lsn");
    pub static OWNER_FULL_NAME: HeaderName = HeaderName::from_static("x-ms-alt-content-path");
    pub static OWNER_ID: HeaderName = HeaderName::from_static("x-ms-content-path");
    pub static OFFER_REPLACE_PENDING: HeaderName =
        HeaderName::from_static("x-ms-offer-replace-pending");
}

/// Header names used by the fault injection framework.
#[cfg(feature = "fault_injection")]
pub(crate) mod fault_injection_header_names {
    use azure_core::http::headers::HeaderName;

    /// Operation type header set on requests for fault injection rule matching.
    pub static FAULT_INJECTION_OPERATION: HeaderName =
        HeaderName::from_static("x-ms-fault-injection-operation");
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
                request_header_names::ACTIVITY_ID.clone(),
                HeaderValue::from(activity_id.as_str().to_owned()),
            );
        }
        if let Some(session_token) = self.session_token.as_ref() {
            headers.insert(
                request_header_names::SESSION_TOKEN.clone(),
                HeaderValue::from(session_token.as_str().to_owned()),
            );
        }
        if let Some(precondition) = self.precondition.as_ref() {
            match precondition {
                Precondition::IfMatch(etag) => headers.insert(
                    request_header_names::IF_MATCH.clone(),
                    HeaderValue::from(etag.as_str().to_owned()),
                ),
                Precondition::IfNoneMatch(etag) => headers.insert(
                    request_header_names::IF_NONE_MATCH.clone(),
                    HeaderValue::from(etag.as_str().to_owned()),
                ),
            }
        }
    }
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
        Self {
            activity_id: headers
                .get_optional_str(&response_header_names::ACTIVITY_ID)
                .map(|s| ActivityId::from_string(s.to_owned())),
            request_charge: headers
                .get_optional_str(&response_header_names::REQUEST_CHARGE)
                .and_then(|s| s.parse::<f64>().ok())
                .map(RequestCharge::new),
            session_token: headers
                .get_optional_str(&response_header_names::SESSION_TOKEN)
                .map(|s| SessionToken::new(s.to_owned())),
            etag: headers
                .get_optional_str(&response_header_names::ETAG)
                .map(|s| ETag::new(s.to_owned())),
            continuation: headers
                .get_optional_str(&response_header_names::CONTINUATION)
                .map(|s| s.to_owned()),
            item_count: headers
                .get_optional_str(&response_header_names::ITEM_COUNT)
                .and_then(|s| s.parse().ok()),
            substatus: headers
                .get_optional_str(&response_header_names::SUBSTATUS)
                .and_then(SubStatusCode::from_header_value),
            index_metrics: headers
                .get_optional_str(&response_header_names::INDEX_METRICS)
                .and_then(|s| match STANDARD.decode(s) {
                    Ok(bytes) => match String::from_utf8(bytes) {
                        Ok(s) => Some(s),
                        Err(e) => {
                            tracing::warn!(
                                header = "x-ms-cosmos-index-utilization",
                                error = %e,
                                "Failed to UTF-8 decode index metrics after base64 decode"
                            );
                            None
                        }
                    },
                    Err(e) => {
                        tracing::warn!(
                            header = "x-ms-cosmos-index-utilization",
                            error = %e,
                            "Failed to base64-decode index metrics header"
                        );
                        None
                    }
                }),
            query_metrics: headers
                .get_optional_str(&response_header_names::QUERY_METRICS)
                .map(|s| s.to_owned()),
            server_duration_ms: headers
                .get_optional_str(&response_header_names::SERVER_DURATION_MS)
                .and_then(|s| s.parse::<f64>().ok())
                .filter(|v| v.is_finite() && *v >= 0.0),
            lsn: headers
                .get_optional_str(&response_header_names::LSN)
                .and_then(|s| s.parse().ok()),
            owner_full_name: headers
                .get_optional_str(&response_header_names::OWNER_FULL_NAME)
                .map(|s| s.to_owned()),
            owner_id: headers
                .get_optional_str(&response_header_names::OWNER_ID)
                .map(|s| s.to_owned()),
            offer_replace_pending: headers
                .get_optional_str(&response_header_names::OFFER_REPLACE_PENDING)
                .and_then(|s| s.parse::<bool>().ok()),
        }
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
