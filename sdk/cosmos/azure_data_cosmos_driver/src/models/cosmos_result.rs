// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB operation result types.

use crate::{
    diagnostics::DiagnosticsContext,
    models::{ActivityId, RequestCharge},
};
use azure_core::http::headers::Headers;
use std::sync::Arc;

/// Standard Cosmos DB response header names.
///
/// All names are lowercase as required by [`HeaderName`]. The azure_core [`Headers`]
/// type normalizes header names to lowercase on insertion, so lookups are case-sensitive
/// but will always match since both sides are lowercase.
mod header_names {
    use azure_core::http::headers::HeaderName;

    pub static ACTIVITY_ID: HeaderName = HeaderName::from_static("x-ms-activity-id");
    pub static REQUEST_CHARGE: HeaderName = HeaderName::from_static("x-ms-request-charge");
    pub static SESSION_TOKEN: HeaderName = HeaderName::from_static("x-ms-session-token");
    pub static CONTENT_LOCATION: HeaderName = HeaderName::from_static("content-location");
    pub static ETAG: HeaderName = HeaderName::from_static("etag");
    pub static CONTINUATION: HeaderName = HeaderName::from_static("x-ms-continuation");
    pub static ITEM_COUNT: HeaderName = HeaderName::from_static("x-ms-item-count");
    pub static SUBSTATUS: HeaderName = HeaderName::from_static("x-ms-substatus");
}

/// Result of a Cosmos DB operation.
///
/// Contains the response body (as raw bytes), relevant headers, and comprehensive
/// diagnostics for the entire operation including status codes.
///
/// # Schema-Agnostic Design
///
/// The driver returns response bodies as raw bytes (`Vec<u8>`). The higher-level
/// SDK (e.g., `azure_data_cosmos`) handles deserialization into typed structures.
/// This allows the driver to be reused across different serialization strategies.
///
/// # Example
///
/// ```ignore
/// let result = driver.execute_operation(/* ... */).await?;
///
/// // Status codes are accessed via diagnostics
/// let diagnostics = result.diagnostics();
/// if let Some(status) = diagnostics.status() {
///     println!("Status: {}", status);
///     println!("RU Charge: {}", result.headers().request_charge().unwrap_or(0.0));
///     if status.is_success() {
///         let body = result.into_body();
///         // Deserialize body...
///     }
/// }
/// ```
#[derive(Debug)]
#[non_exhaustive]
pub struct CosmosResult {
    /// Raw response body (UTF-8 JSON or Cosmos binary encoding).
    body: Vec<u8>,

    /// Extracted Cosmos-specific headers.
    headers: CosmosHeaders,

    /// Full diagnostics context for this operation (contains status codes).
    diagnostics: Arc<DiagnosticsContext>,
}

impl CosmosResult {
    /// Creates a new `CosmosResult`.
    ///
    /// This is typically called by the driver after completing an operation.
    /// The diagnostics context should already contain the status codes
    /// (set via `DiagnosticsContextBuilder::set_operation_status` before completion).
    pub(crate) fn new(
        body: Vec<u8>,
        headers: CosmosHeaders,
        diagnostics: Arc<DiagnosticsContext>,
    ) -> Self {
        Self {
            body,
            headers,
            diagnostics,
        }
    }

    /// Returns a reference to the response body.
    ///
    /// The body is raw bytes - typically UTF-8 JSON but may be Cosmos binary
    /// encoding for certain operations. The higher-level SDK handles parsing.
    pub fn body(&self) -> &[u8] {
        &self.body
    }

    /// Consumes the result and returns the body.
    pub fn into_body(self) -> Vec<u8> {
        self.body
    }

    /// Consumes the result and returns all parts.
    pub(crate) fn into_parts(self) -> (Vec<u8>, CosmosHeaders, Arc<DiagnosticsContext>) {
        (self.body, self.headers, self.diagnostics)
    }

    /// Returns a reference to the extracted headers.
    pub(crate) fn headers(&self) -> &CosmosHeaders {
        &self.headers
    }

    /// Returns a reference to the diagnostics context.
    ///
    /// The diagnostics context contains detailed information about all
    /// requests made during this operation, including retries, hedging,
    /// and regional failovers. It also holds the operation-level status codes.
    pub fn diagnostics(&self) -> &Arc<DiagnosticsContext> {
        &self.diagnostics
    }
}

/// Cosmos-specific headers extracted from HTTP response.
///
/// These headers contain important metadata about the operation including
/// request charges (RU), session tokens, and activity IDs for debugging.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub(crate) struct CosmosHeaders {
    /// Activity ID for request correlation (`x-ms-activity-id`).
    activity_id: Option<ActivityId>,

    /// Request charge in Request Units (`x-ms-request-charge`).
    request_charge: Option<RequestCharge>,

    /// Session token for session consistency (`x-ms-session-token`).
    session_token: Option<String>,

    /// Content location URI (`content-location`).
    content_location: Option<String>,

    /// ETag for optimistic concurrency (`etag`).
    etag: Option<String>,

    /// Continuation token for pagination (`x-ms-continuation`).
    continuation: Option<String>,

    /// Item count in response (`x-ms-item-count`).
    item_count: Option<u32>,
}

impl CosmosHeaders {
    /// Creates an empty `CosmosHeaders`.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Extracts Cosmos headers from HTTP response headers.
    ///
    /// This parses standard Cosmos headers into typed fields for easy access.
    pub(crate) fn from_headers(headers: &Headers) -> Self {
        Self {
            activity_id: headers
                .get_optional_str(&header_names::ACTIVITY_ID)
                .map(|s| ActivityId::from_string(s.to_owned())),
            request_charge: headers
                .get_optional_str(&header_names::REQUEST_CHARGE)
                .and_then(|s| s.parse::<f64>().ok())
                .map(RequestCharge::new),
            session_token: headers
                .get_optional_str(&header_names::SESSION_TOKEN)
                .map(|s| s.to_owned()),
            content_location: headers
                .get_optional_str(&header_names::CONTENT_LOCATION)
                .map(|s| s.to_owned()),
            etag: headers
                .get_optional_str(&header_names::ETAG)
                .map(|s| s.to_owned()),
            continuation: headers
                .get_optional_str(&header_names::CONTINUATION)
                .map(|s| s.to_owned()),
            item_count: headers
                .get_optional_str(&header_names::ITEM_COUNT)
                .and_then(|s| s.parse().ok()),
        }
    }

    /// Returns the activity ID for request correlation.
    pub fn activity_id(&self) -> Option<&ActivityId> {
        self.activity_id.as_ref()
    }

    /// Returns the request charge (RU) for this response.
    ///
    /// For the total RU across all requests (including retries),
    /// use [`DiagnosticsContext::total_request_charge`].
    pub fn request_charge(&self) -> Option<RequestCharge> {
        self.request_charge
    }

    /// Returns the session token for session consistency.
    ///
    /// When using session consistency, this token should be propagated
    /// to subsequent requests to ensure read-your-writes semantics.
    pub fn session_token(&self) -> Option<&str> {
        self.session_token.as_deref()
    }

    /// Returns the content location URI.
    pub fn content_location(&self) -> Option<&str> {
        self.content_location.as_deref()
    }

    /// Returns the ETag for optimistic concurrency control.
    pub fn etag(&self) -> Option<&str> {
        self.etag.as_deref()
    }

    /// Returns the continuation token for pagination.
    ///
    /// If present, there are more results available. Pass this token
    /// to the next request to retrieve the next page.
    pub fn continuation(&self) -> Option<&str> {
        self.continuation.as_deref()
    }

    /// Returns the item count in this response.
    pub fn item_count(&self) -> Option<u32> {
        self.item_count
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
    pub fn with_session_token(mut self, token: String) -> Self {
        self.session_token = Some(token);
        self
    }

    /// Sets the ETag.
    pub fn with_etag(mut self, etag: String) -> Self {
        self.etag = Some(etag);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        diagnostics::DiagnosticsContextBuilder,
        models::{CosmosStatus, RequestCharge, SubStatusCode},
        options::DiagnosticsOptions,
    };
    use azure_core::http::StatusCode;

    fn make_diagnostics(
        status_code: Option<StatusCode>,
        sub_status_code: Option<SubStatusCode>,
    ) -> Arc<DiagnosticsContext> {
        let mut builder = DiagnosticsContextBuilder::new(
            ActivityId::new_uuid(),
            Arc::new(DiagnosticsOptions::default()),
        );
        if let Some(status) = status_code {
            builder.set_operation_status(status, sub_status_code);
        }
        Arc::new(builder.complete())
    }

    #[test]
    fn cosmos_result_accessors() {
        let headers = CosmosHeaders::new()
            .with_request_charge(RequestCharge::new(5.5))
            .with_activity_id(ActivityId::from_string("test-activity".to_string()));

        let result = CosmosResult::new(
            b"{\"id\": \"test\"}".to_vec(),
            headers,
            make_diagnostics(Some(StatusCode::Ok), None),
        );

        // Status codes are accessed via diagnostics
        let status = result.diagnostics().status().unwrap();
        assert_eq!(status.status_code(), StatusCode::Ok);
        assert!(status.is_success());
        assert!(status.sub_status_code().is_none());
        assert_eq!(result.body(), b"{\"id\": \"test\"}");
        assert_eq!(
            result.headers().request_charge(),
            Some(RequestCharge::new(5.5))
        );
    }

    #[test]
    fn cosmos_result_error_status() {
        let result = CosmosResult::new(
            b"{}".to_vec(),
            CosmosHeaders::new(),
            make_diagnostics(
                Some(StatusCode::TooManyRequests),
                Some(SubStatusCode::new(3200)),
            ),
        );

        let status = result.diagnostics().status().unwrap();
        assert!(!status.is_success());
        assert!(status.is_throttled());
        assert_eq!(status, &CosmosStatus::RU_BUDGET_EXCEEDED);
    }

    #[test]
    fn cosmos_result_into_parts() {
        let headers = CosmosHeaders::new().with_request_charge(RequestCharge::new(1.0));
        let result = CosmosResult::new(
            b"body".to_vec(),
            headers,
            make_diagnostics(Some(StatusCode::Created), None),
        );

        let (body, headers, diagnostics) = result.into_parts();
        assert_eq!(body, b"body");
        assert_eq!(
            diagnostics.status().unwrap().status_code(),
            StatusCode::Created
        );
        assert!(diagnostics.status().unwrap().sub_status_code().is_none());
        assert_eq!(headers.request_charge(), Some(RequestCharge::new(1.0)));
    }

    #[test]
    fn cosmos_result_status_via_diagnostics() {
        let diagnostics = make_diagnostics(
            Some(StatusCode::NotFound),
            Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE),
        );
        let result = CosmosResult::new(b"{}".to_vec(), CosmosHeaders::new(), diagnostics.clone());

        // Status codes are only accessible via diagnostics
        let status = diagnostics.status().unwrap();
        assert_eq!(status.status_code(), StatusCode::NotFound);
        assert!(status.is_read_session_not_available());
        // Same via result.diagnostics()
        let result_status = result.diagnostics().status().unwrap();
        assert_eq!(result_status.status_code(), StatusCode::NotFound);
        assert!(result_status.is_read_session_not_available());
    }

    #[test]
    fn cosmos_headers_from_azure_headers() {
        let mut headers = Headers::new();
        headers.insert("x-ms-activity-id", "abc-123");
        headers.insert("x-ms-request-charge", "5.67");
        headers.insert("x-ms-session-token", "session:456");
        headers.insert("etag", "\"version-1\"");
        headers.insert("x-ms-continuation", "next-page-token");
        headers.insert("x-ms-item-count", "10");

        let cosmos_headers = CosmosHeaders::from_headers(&headers);

        assert_eq!(
            cosmos_headers.activity_id().map(|a| a.as_str()),
            Some("abc-123")
        );
        assert!((cosmos_headers.request_charge().unwrap().value() - 5.67).abs() < f64::EPSILON);
        assert_eq!(cosmos_headers.session_token(), Some("session:456"));
        assert_eq!(cosmos_headers.etag(), Some("\"version-1\""));
        assert_eq!(cosmos_headers.continuation(), Some("next-page-token"));
        assert_eq!(cosmos_headers.item_count(), Some(10));
    }

    #[test]
    fn cosmos_headers_builder_pattern() {
        let headers = CosmosHeaders::new()
            .with_activity_id(ActivityId::from_string("test".to_string()))
            .with_request_charge(RequestCharge::new(2.0))
            .with_session_token("token".to_string())
            .with_etag("etag".to_string())
            .with_continuation("cont".to_string())
            .with_item_count(5);

        assert_eq!(headers.activity_id().map(|a| a.as_str()), Some("test"));
        assert_eq!(headers.request_charge(), Some(RequestCharge::new(2.0)));
        assert_eq!(headers.session_token(), Some("token"));
        assert_eq!(headers.etag(), Some("etag"));
        assert_eq!(headers.continuation(), Some("cont"));
        assert_eq!(headers.item_count(), Some(5));
    }

    #[test]
    fn cosmos_headers_default_empty() {
        let headers = CosmosHeaders::default();

        assert!(headers.activity_id().is_none());
        assert!(headers.request_charge().is_none());
        assert!(headers.session_token().is_none());
        assert!(headers.etag().is_none());
        assert!(headers.continuation().is_none());
        assert!(headers.item_count().is_none());
    }
}
