// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Provides the [`CosmosResponse`] type for wrapping responses from Cosmos DB operations.

use crate::cosmos_request::CosmosRequest;
use crate::models::CosmosDiagnostics;
use crate::SessionToken;
use azure_core::http::{headers::Headers, response::Response, StatusCode};
use azure_data_cosmos_driver::models::CosmosResponseHeaders;
use serde::de::DeserializeOwned;

/// A response from a Cosmos DB operation.
///
/// This wraps the underlying Azure Core typed response and provides convenient access
/// to headers, status code, the original request, and Cosmos-specific response metadata.
///
/// This type is internal to the SDK. Public response types like
/// [`ItemResponse`](crate::models::ItemResponse),
/// [`ResourceResponse`](crate::models::ResourceResponse), and
/// [`BatchResponse`](crate::models::BatchResponse) wrap this type and expose
/// only the accessors relevant to their operation.
#[derive(Debug)]
pub(crate) struct CosmosResponse<T> {
    /// The underlying typed HTTP response.
    response: Response<T>,
    /// The final request used to fulfill the operation, if available.
    ///
    // TODO: Remove this field once all operations are routed through the driver.
    // Driver-routed operations set this to `None` because the driver uses
    // `CosmosOperation` + `OperationOptions` instead of `CosmosRequest`.
    #[allow(dead_code)]
    request: Option<CosmosRequest>,
    /// Parsed Cosmos-specific response headers.
    cosmos_headers: CosmosResponseHeaders,
    /// Diagnostics for this operation.
    diagnostics: CosmosDiagnostics,
}

impl<T> CosmosResponse<T> {
    /// Creates a new `CosmosResponse` from a typed response and the original request.
    pub(crate) fn new(response: Response<T>, request: CosmosRequest) -> Self {
        let cosmos_headers = CosmosResponseHeaders::from_headers(response.headers());
        let diagnostics = CosmosDiagnostics::from_headers(&cosmos_headers);
        Self {
            response,
            request: Some(request),
            cosmos_headers,
            diagnostics,
        }
    }

    /// Creates a `CosmosResponse` from a typed response and pre-parsed headers.
    ///
    /// Used by the driver bridge to avoid re-parsing headers that were already
    /// parsed by the driver pipeline.
    pub(crate) fn from_driver_response(
        response: Response<T>,
        cosmos_headers: CosmosResponseHeaders,
    ) -> Self {
        let diagnostics = CosmosDiagnostics::from_headers(&cosmos_headers);
        Self {
            response,
            request: None,
            cosmos_headers,
            diagnostics,
        }
    }

    /// Returns the HTTP status code of the response.
    pub(crate) fn status(&self) -> StatusCode {
        self.response.status()
    }

    /// Returns a reference to all response headers.
    pub(crate) fn headers(&self) -> &Headers {
        self.response.headers()
    }

    /// Returns a reference to the parsed Cosmos-specific response headers.
    pub(crate) fn cosmos_headers(&self) -> &CosmosResponseHeaders {
        &self.cosmos_headers
    }

    /// Returns the final request used to fulfill the operation.
    #[cfg(feature = "fault_injection")]
    #[allow(dead_code)]
    pub(crate) fn request(&self) -> Option<&CosmosRequest> {
        self.request.as_ref()
    }

    /// Returns the final request URL used to fulfill the operation.
    /// Returns `None` for driver-routed operations.
    #[cfg(feature = "fault_injection")]
    pub(crate) fn request_url(&self) -> Option<azure_core::http::Url> {
        self.request
            .clone()
            .map(|r| r.into_raw_request().url().clone())
    }

    /// Consumes the response and returns the response body.
    pub(crate) fn into_body(self) -> azure_core::http::response::ResponseBody {
        self.response.into_body()
    }

    /// Returns the request charge (RU consumption) for this operation, if available.
    pub(crate) fn request_charge(&self) -> Option<f64> {
        self.cosmos_headers
            .request_charge
            .as_ref()
            .map(|rc| rc.value())
    }

    /// Returns the session token from this response, if available.
    pub(crate) fn session_token(&self) -> Option<SessionToken> {
        self.cosmos_headers
            .session_token
            .as_ref()
            .map(|st| SessionToken::from(st.as_str().to_string()))
    }

    /// Returns the diagnostics for this operation.
    pub(crate) fn diagnostics(&self) -> &CosmosDiagnostics {
        &self.diagnostics
    }
}

impl<T: DeserializeOwned> CosmosResponse<T> {
    /// Deserializes the response body into a model type.
    pub(crate) fn into_model(self) -> azure_core::Result<T> {
        self.response.into_body().json()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ItemResponse;
    use crate::operation_context::OperationType;
    use crate::resource_context::{ResourceLink, ResourceType};
    use crate::PartitionKey;
    use azure_core::http::{RawResponse, StatusCode};
    use azure_core::Bytes;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestModel {
        id: String,
        value: i32,
    }

    fn create_mock_request() -> CosmosRequest {
        let resource_link = ResourceLink::root(ResourceType::Databases);
        CosmosRequest::builder(OperationType::Read, resource_link)
            .partition_key(PartitionKey::from("test"))
            .build()
            .unwrap()
    }

    fn create_response_with_body(body: &str) -> CosmosResponse<TestModel> {
        let raw_response = RawResponse::from_bytes(
            StatusCode::Ok,
            Headers::new(),
            Bytes::from(body.to_string()),
        );
        let typed_response: Response<TestModel> = raw_response.into();
        CosmosResponse::new(typed_response, create_mock_request())
    }

    fn create_response_with_headers(headers: Headers) -> CosmosResponse<TestModel> {
        let raw_response = RawResponse::from_bytes(
            StatusCode::Ok,
            headers,
            Bytes::from(r#"{"id":"test","value":1}"#),
        );
        let typed_response: Response<TestModel> = raw_response.into();
        CosmosResponse::new(typed_response, create_mock_request())
    }

    #[test]
    fn into_model_with_valid_json_succeeds() {
        let body = r#"{"id": "test-id", "value": 42}"#;
        let response = create_response_with_body(body);
        let result = response.into_model();

        assert!(result.is_ok());
        let model = result.unwrap();
        assert_eq!(model.id, "test-id");
        assert_eq!(model.value, 42);
    }

    #[test]
    fn into_model_with_malformed_json_returns_error() {
        let body = r#"{"id": "test-id", "value": not_a_number}"#;
        let response = create_response_with_body(body);
        let result = response.into_model();

        assert!(result.is_err());
        let error = result.unwrap_err();
        let error_message = error.to_string();
        assert!(
            error_message.contains("expected")
                || error_message.contains("JSON")
                || error_message.contains("parse"),
            "Expected JSON parse error, got: {}",
            error_message
        );
    }

    #[test]
    fn into_model_with_empty_json_returns_error() {
        let body = "";
        let response = create_response_with_body(body);
        let result = response.into_model();

        assert!(result.is_err());
    }

    #[test]
    fn into_model_with_incomplete_json_returns_error() {
        let body = r#"{"id": "test-id""#;
        let response = create_response_with_body(body);
        let result = response.into_model();

        assert!(result.is_err());
    }

    #[test]
    fn into_model_with_missing_required_field_returns_error() {
        let body = r#"{"id": "test-id"}"#;
        let response = create_response_with_body(body);
        let result = response.into_model();

        assert!(result.is_err());
        let error = result.unwrap_err();
        let error_message = error.to_string();
        assert!(
            error_message.contains("value") || error_message.contains("missing"),
            "Expected missing field error, got: {}",
            error_message
        );
    }

    #[test]
    fn activity_id_returns_header_value() {
        let mut headers = Headers::new();
        headers.insert("x-ms-activity-id", "abc-123-def");
        let response = create_response_with_headers(headers);
        assert_eq!(response.diagnostics().activity_id(), Some("abc-123-def"));
    }

    #[test]
    fn server_duration_ms_returns_parsed_value() {
        let mut headers = Headers::new();
        headers.insert("x-ms-request-duration-ms", "4.56");
        let response = create_response_with_headers(headers);
        assert!((response.diagnostics().server_duration_ms().unwrap() - 4.56).abs() < f64::EPSILON);
    }

    #[test]
    fn server_duration_ms_rejects_non_finite() {
        for value in ["NaN", "inf", "-inf", "Infinity", "-Infinity", "-1.0"] {
            let mut headers = Headers::new();
            headers.insert("x-ms-request-duration-ms", value);
            let response = create_response_with_headers(headers);
            assert!(
                response.diagnostics().server_duration_ms().is_none(),
                "Expected None for '{value}'"
            );
        }
    }

    #[test]
    fn missing_headers_return_none() {
        let response = create_response_with_headers(Headers::new());
        assert!(response.diagnostics().activity_id().is_none());
        assert!(response.diagnostics().server_duration_ms().is_none());
        assert!(response.request_charge().is_none());
        assert!(response.session_token().is_none());
    }

    #[test]
    fn item_response_has_etag() {
        let mut headers = Headers::new();
        headers.insert("etag", "\"some-etag\"");
        let raw_response = RawResponse::from_bytes(
            StatusCode::Ok,
            headers,
            Bytes::from(r#"{"id":"test","value":1}"#),
        );
        let typed_response: Response<TestModel> = raw_response.into();
        let response = CosmosResponse::new(typed_response, create_mock_request());
        let item_response = ItemResponse::new(response);
        assert_eq!(
            item_response.etag().map(|e| e.to_string()),
            Some("\"some-etag\"".to_string())
        );
    }

    #[test]
    fn item_response_lsn_returns_parsed_value() {
        let mut headers = Headers::new();
        headers.insert("lsn", "42");
        let raw_response = RawResponse::from_bytes(
            StatusCode::Ok,
            headers,
            Bytes::from(r#"{"id":"test","value":1}"#),
        );
        let typed_response: Response<TestModel> = raw_response.into();
        let response = CosmosResponse::new(typed_response, create_mock_request());
        let item_response = ItemResponse::new(response);
        assert_eq!(item_response.lsn(), Some(42));
    }

    #[test]
    fn item_response_lsn_returns_none_when_missing() {
        let raw_response = RawResponse::from_bytes(
            StatusCode::Ok,
            Headers::new(),
            Bytes::from(r#"{"id":"test","value":1}"#),
        );
        let typed_response: Response<TestModel> = raw_response.into();
        let response = CosmosResponse::new(typed_response, create_mock_request());
        let item_response = ItemResponse::new(response);
        assert_eq!(item_response.lsn(), None);
    }

    #[test]
    fn item_response_item_lsn_returns_parsed_value() {
        let mut headers = Headers::new();
        headers.insert("x-ms-item-lsn", "37");
        let raw_response = RawResponse::from_bytes(
            StatusCode::Ok,
            headers,
            Bytes::from(r#"{"id":"test","value":1}"#),
        );
        let typed_response: Response<TestModel> = raw_response.into();
        let response = CosmosResponse::new(typed_response, create_mock_request());
        let item_response = ItemResponse::new(response);
        assert_eq!(item_response.item_lsn(), Some(37));
    }

    #[test]
    fn item_response_item_lsn_returns_none_when_missing() {
        let raw_response = RawResponse::from_bytes(
            StatusCode::Ok,
            Headers::new(),
            Bytes::from(r#"{"id":"test","value":1}"#),
        );
        let typed_response: Response<TestModel> = raw_response.into();
        let response = CosmosResponse::new(typed_response, create_mock_request());
        let item_response = ItemResponse::new(response);
        assert_eq!(item_response.item_lsn(), None);
    }

    #[test]
    fn batch_response_has_etag() {
        use crate::models::BatchResponse;
        let mut headers = Headers::new();
        headers.insert("etag", "\"batch-etag\"");
        let raw_response = RawResponse::from_bytes(
            StatusCode::Ok,
            headers,
            Bytes::from(r#"{"id":"test","value":1}"#),
        );
        let typed_response: Response<crate::TransactionalBatchResponse> = raw_response.into();
        let response = CosmosResponse::new(typed_response, create_mock_request());
        let batch_response = BatchResponse::new(response);
        assert_eq!(
            batch_response.etag().map(|e| e.to_string()),
            Some("\"batch-etag\"".to_string())
        );
    }

    #[test]
    fn query_feed_page_has_index_and_query_metrics() {
        use crate::feed::QueryFeedPage;
        let mut headers = Headers::new();
        headers.insert(
            "x-ms-cosmos-index-utilization",
            // cspell:disable-next-line
            "eyJVdGlsaXplZFNpbmdsZUluZGV4ZXMiOltdfQ==",
        );
        headers.insert(
            "x-ms-documentdb-query-metrics",
            "totalExecutionTimeInMs=1.23;queryCompileTimeInMs=0.01",
        );
        let raw_response = RawResponse::from_bytes(
            StatusCode::Ok,
            headers,
            Bytes::from(r#"{"Documents":[{"id":"test","value":1}]}"#),
        );
        let typed_response: Response<crate::feed::FeedBody<TestModel>> = raw_response.into();
        let cosmos_response = CosmosResponse::new(typed_response, create_mock_request());

        let rt = tokio::runtime::Runtime::new().unwrap();
        let page = rt
            .block_on(QueryFeedPage::from_response(cosmos_response))
            .unwrap();
        assert_eq!(
            page.index_metrics(),
            Some(r#"{"UtilizedSingleIndexes":[]}"#)
        );
        assert_eq!(
            page.query_metrics(),
            Some("totalExecutionTimeInMs=1.23;queryCompileTimeInMs=0.01")
        );
    }
}
