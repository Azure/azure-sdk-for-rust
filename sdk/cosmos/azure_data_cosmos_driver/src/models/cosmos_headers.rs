// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB request/response header models.

use crate::models::{ActivityId, ETag, RequestCharge, SessionToken, SubStatusCode};
use azure_core::http::headers::{HeaderValue, Headers};

/// Standard Cosmos DB request header names.
///
/// All names are lowercase as required by [`HeaderName`]. The azure_core [`Headers`]
/// type normalizes header names to lowercase on insertion, so lookups are case-sensitive
/// but will always match since both sides are lowercase.
mod request_header_names {
    use azure_core::http::headers::HeaderName;

    pub static ACTIVITY_ID: HeaderName = HeaderName::from_static("x-ms-activity-id");
    pub static SESSION_TOKEN: HeaderName = HeaderName::from_static("x-ms-session-token");
}

/// Standard Cosmos DB response header names.
mod response_header_names {
    use azure_core::http::headers::HeaderName;

    pub static ACTIVITY_ID: HeaderName = HeaderName::from_static("x-ms-activity-id");
    pub static REQUEST_CHARGE: HeaderName = HeaderName::from_static("x-ms-request-charge");
    pub static SESSION_TOKEN: HeaderName = HeaderName::from_static("x-ms-session-token");
    pub static ETAG: HeaderName = HeaderName::from_static("etag");
    pub static CONTINUATION: HeaderName = HeaderName::from_static("x-ms-continuation");
    pub static ITEM_COUNT: HeaderName = HeaderName::from_static("x-ms-item-count");
    pub static SUBSTATUS: HeaderName = HeaderName::from_static("x-ms-substatus");
}

/// Cosmos request headers for operation-level customization.
///
/// Only whitelisted request headers are supported.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct CosmosRequestHeaders {
    /// Activity ID for request correlation (`x-ms-activity-id`).
    activity_id: Option<ActivityId>,

    /// Session token for session consistency (`x-ms-session-token`).
    session_token: Option<SessionToken>,
}

impl CosmosRequestHeaders {
    /// Creates an empty `CosmosRequestHeaders`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the activity ID for request correlation.
    pub fn activity_id(&self) -> Option<&ActivityId> {
        self.activity_id.as_ref()
    }

    /// Returns the session token for session consistency.
    pub fn session_token(&self) -> Option<&SessionToken> {
        self.session_token.as_ref()
    }

    /// Sets the activity ID.
    pub fn with_activity_id(mut self, activity_id: ActivityId) -> Self {
        self.activity_id = Some(activity_id);
        self
    }

    /// Sets the session token.
    pub fn with_session_token(mut self, token: impl Into<SessionToken>) -> Self {
        self.session_token = Some(token.into());
        self
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
    activity_id: Option<ActivityId>,

    /// Request charge in Request Units (`x-ms-request-charge`).
    request_charge: Option<RequestCharge>,

    /// Session token for session consistency (`x-ms-session-token`).
    session_token: Option<SessionToken>,

    /// ETag for optimistic concurrency (`etag`).
    etag: Option<ETag>,

    /// Continuation token for pagination (`x-ms-continuation`).
    continuation: Option<String>,

    /// Item count in response (`x-ms-item-count`).
    item_count: Option<u32>,

    /// Cosmos substatus code (`x-ms-substatus`).
    substatus: Option<SubStatusCode>,
}

impl CosmosResponseHeaders {
    /// Creates an empty `CosmosResponseHeaders`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Extracts Cosmos headers from HTTP response headers.
    ///
    /// This parses standard Cosmos headers into typed fields for easy access.
    pub(crate) fn from_headers(headers: &Headers) -> Self {
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
        }
    }

    /// Returns the activity ID for request correlation.
    pub fn activity_id(&self) -> Option<&ActivityId> {
        self.activity_id.as_ref()
    }

    /// Returns the request charge (RU) for this response.
    pub fn request_charge(&self) -> Option<RequestCharge> {
        self.request_charge
    }

    /// Returns the session token for session consistency.
    pub fn session_token(&self) -> Option<&SessionToken> {
        self.session_token.as_ref()
    }

    /// Returns the ETag for optimistic concurrency control.
    pub fn etag(&self) -> Option<&ETag> {
        self.etag.as_ref()
    }

    /// Returns the continuation token for pagination.
    pub fn continuation(&self) -> Option<&str> {
        self.continuation.as_deref()
    }

    /// Returns the item count in this response.
    pub fn item_count(&self) -> Option<u32> {
        self.item_count
    }

    /// Returns the Cosmos substatus code, if present.
    pub fn substatus(&self) -> Option<SubStatusCode> {
        self.substatus
    }

    /// Sets the activity ID.
    pub fn with_activity_id(mut self, activity_id: ActivityId) -> Self {
        self.activity_id = Some(activity_id);
        self
    }

    /// Sets the request charge.
    pub fn with_request_charge(mut self, charge: RequestCharge) -> Self {
        self.request_charge = Some(charge);
        self
    }

    /// Sets the session token.
    pub fn with_session_token(mut self, token: impl Into<SessionToken>) -> Self {
        self.session_token = Some(token.into());
        self
    }

    /// Sets the ETag.
    pub fn with_etag(mut self, etag: impl Into<ETag>) -> Self {
        self.etag = Some(etag.into());
        self
    }

    /// Sets the continuation token.
    pub fn with_continuation(mut self, continuation: String) -> Self {
        self.continuation = Some(continuation);
        self
    }

    /// Sets the item count.
    pub fn with_item_count(mut self, count: u32) -> Self {
        self.item_count = Some(count);
        self
    }

    /// Sets the substatus code.
    pub fn with_substatus(mut self, substatus: SubStatusCode) -> Self {
        self.substatus = Some(substatus);
        self
    }

}

#[cfg(test)]
mod tests {
    use super::*;

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

        let cosmos_headers = CosmosResponseHeaders::from_headers(&headers);

        assert_eq!(
            cosmos_headers.activity_id().map(|a| a.as_str()),
            Some("abc-123")
        );
        assert!((cosmos_headers.request_charge().unwrap().value() - 5.67).abs() < f64::EPSILON);
        assert_eq!(
            cosmos_headers.session_token().map(SessionToken::as_str),
            Some("session:456")
        );
        assert_eq!(
            cosmos_headers.etag().map(ETag::as_str),
            Some("\"version-1\"")
        );
        assert_eq!(cosmos_headers.continuation(), Some("next-page-token"));
        assert_eq!(cosmos_headers.item_count(), Some(10));
        assert_eq!(cosmos_headers.substatus(), Some(SubStatusCode::new(3200)));
    }

    #[test]
    fn cosmos_response_headers_builder_pattern() {
        let headers = CosmosResponseHeaders::new()
            .with_activity_id(ActivityId::from_string("test".to_string()))
            .with_request_charge(RequestCharge::new(2.0))
            .with_session_token("token".to_string())
            .with_etag("etag".to_string())
            .with_continuation("cont".to_string())
            .with_item_count(5)
            .with_substatus(SubStatusCode::new(1002));

        assert_eq!(headers.activity_id().map(|a| a.as_str()), Some("test"));
        assert_eq!(headers.request_charge(), Some(RequestCharge::new(2.0)));
        assert_eq!(
            headers.session_token().map(SessionToken::as_str),
            Some("token")
        );
        assert_eq!(headers.etag().map(ETag::as_str), Some("etag"));
        assert_eq!(headers.continuation(), Some("cont"));
        assert_eq!(headers.item_count(), Some(5));
        assert_eq!(headers.substatus(), Some(SubStatusCode::new(1002)));
    }

    #[test]
    fn cosmos_response_headers_default_empty() {
        let headers = CosmosResponseHeaders::default();

        assert!(headers.activity_id().is_none());
        assert!(headers.request_charge().is_none());
        assert!(headers.session_token().is_none());
        assert!(headers.etag().is_none());
        assert!(headers.continuation().is_none());
        assert!(headers.item_count().is_none());
        assert!(headers.substatus().is_none());
    }

    #[test]
    fn cosmos_request_headers_builder_pattern() {
        let headers = CosmosRequestHeaders::new()
            .with_activity_id(ActivityId::from_string("test-request".to_string()))
            .with_session_token("session-token");

        assert_eq!(
            headers.activity_id().map(ActivityId::as_str),
            Some("test-request")
        );
        assert_eq!(
            headers.session_token().map(SessionToken::as_str),
            Some("session-token")
        );
    }

    #[test]
    fn cosmos_request_headers_write_to_headers() {
        let cosmos_headers = CosmosRequestHeaders::new()
            .with_activity_id(ActivityId::from_string("test-request".to_string()))
            .with_session_token("session-token");
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
}
