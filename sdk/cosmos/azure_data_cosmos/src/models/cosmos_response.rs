// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Provides the [`CosmosResponse`] type for wrapping responses from Cosmos DB operations.

use crate::cosmos_request::CosmosRequest;
use crate::models::{CosmosDiagnostics, ItemMetadata};
use crate::SessionToken;
use azure_core::http::{
    headers::{HeaderName, Headers},
    response::Response,
    StatusCode,
};
use azure_data_cosmos_driver::models::CosmosResponseHeaders;
use serde::de::DeserializeOwned;

/// A response from a Cosmos DB operation.
///
/// This wraps the underlying Azure Core typed response and provides convenient access
/// to headers, status code, the original request, and Cosmos-specific response metadata.
///
/// The type parameter `M` carries operation-specific metadata (e.g. [`ItemMetadata`],
/// [`QueryMetadata`](crate::models::QueryMetadata),
/// [`ResourceMetadata`](crate::models::ResourceMetadata)).
/// It defaults to `()` for internal use where no metadata is needed.
#[derive(Debug)]
pub struct CosmosResponse<T, M = ()> {
    /// The underlying typed HTTP response.
    response: Response<T>,
    /// The final request used to fulfill the operation.
    #[allow(dead_code)]
    request: CosmosRequest,
    /// Parsed Cosmos-specific response headers.
    cosmos_headers: CosmosResponseHeaders,
    /// Operation-specific metadata.
    metadata: M,
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
            request,
            cosmos_headers,
            metadata: (),
            diagnostics,
        }
    }
}

impl<T, M> CosmosResponse<T, M> {
    /// Transforms this response's metadata by applying a closure to the parsed
    /// Cosmos response headers, producing a new `CosmosResponse` with a
    /// different metadata type.
    pub(crate) fn map_metadata<N>(
        self,
        f: impl FnOnce(&CosmosResponseHeaders) -> N,
    ) -> CosmosResponse<T, N> {
        let metadata = f(&self.cosmos_headers);
        CosmosResponse {
            response: self.response,
            request: self.request,
            cosmos_headers: self.cosmos_headers,
            metadata,
            diagnostics: self.diagnostics,
        }
    }

    /// Returns the HTTP status code of the response.
    pub fn status(&self) -> StatusCode {
        self.response.status()
    }

    /// Returns a reference to all response headers.
    pub fn headers(&self) -> &Headers {
        self.response.headers()
    }

    /// Returns a reference to the parsed Cosmos-specific response headers.
    pub(crate) fn cosmos_headers(&self) -> &CosmosResponseHeaders {
        &self.cosmos_headers
    }

    /// Gets an optional header value as a string by name.
    ///
    /// Returns `Some(&str)` if the header exists,
    /// or `None` if the header doesn't exist.
    pub fn get_optional_header_str(&self, name: &HeaderName) -> Option<&str> {
        self.response.headers().get_optional_str(name)
    }

    /// Returns the final request used to fulfill the operation.
    /// This api is subject to change without a major version bump.
    ///
    #[cfg(feature = "fault_injection")]
    #[allow(dead_code)]
    pub(crate) fn request(&self) -> &CosmosRequest {
        &self.request
    }

    /// Returns the final request URL used to fulfill the operation.
    /// This api is subject to change without a major version bump.
    #[cfg(feature = "fault_injection")]
    pub fn request_url(&self) -> azure_core::http::Url {
        self.request.clone().into_raw_request().url().clone()
    }

    /// Consumes the response and returns the response body.
    pub fn into_body(self) -> azure_core::http::response::ResponseBody {
        self.response.into_body()
    }

    /// Returns the request charge (RU consumption) for this operation, if available.
    pub fn request_charge(&self) -> Option<f64> {
        self.cosmos_headers
            .request_charge
            .as_ref()
            .map(|rc| rc.value())
    }

    /// Returns the session token from this response, if available.
    pub fn session_token(&self) -> Option<SessionToken> {
        self.cosmos_headers
            .session_token
            .as_ref()
            .map(|st| SessionToken::from(st.as_str().to_string()))
    }

    /// Returns the diagnostics for this operation.
    ///
    /// Provides access to the activity ID, server-side duration, and other
    /// diagnostic information for debugging and performance analysis.
    pub fn diagnostics(&self) -> &CosmosDiagnostics {
        &self.diagnostics
    }

    /// Returns the operation-specific metadata.
    pub fn metadata(&self) -> &M {
        &self.metadata
    }

    /// Deserializes the response body without consuming the response.
    ///
    /// This is used internally to extract a copy of the response model (e.g.,
    /// to populate a cache) while still returning the original response to the
    /// caller. The underlying `Bytes` body is reference-counted so the clone
    /// is cheap.
    #[allow(dead_code)] // Useful utility for future cache population scenarios
    pub(crate) fn deserialize_body<U: DeserializeOwned>(&self) -> azure_core::Result<U> {
        self.response.body().json()
    }
}

impl<T: DeserializeOwned, M> CosmosResponse<T, M> {
    /// Deserializes the response body into a model type.
    pub fn into_model(self) -> azure_core::Result<T> {
        self.response.into_body().json()
    }
}

impl<T> CosmosResponse<T, ItemMetadata> {
    /// Returns the ETag from this response, if available.
    pub fn etag(&self) -> Option<&str> {
        self.metadata.etag()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        // The error should be a JSON parsing error
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
        // Missing "value" field which is required in TestModel
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
        for value in ["NaN", "inf", "-inf", "Infinity", "-Infinity"] {
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
    fn map_metadata_produces_item_metadata_with_etag() {
        let mut headers = Headers::new();
        headers.insert("etag", "\"some-etag\"");
        let raw_response = RawResponse::from_bytes(
            StatusCode::Ok,
            headers,
            Bytes::from(r#"{"id":"test","value":1}"#),
        );
        let typed_response: Response<TestModel> = raw_response.into();
        let response = CosmosResponse::new(typed_response, create_mock_request());
        let response = response.map_metadata(ItemMetadata::from_headers);
        assert_eq!(response.etag(), Some("\"some-etag\""));
    }

    #[test]
    fn map_metadata_index_and_query_metrics() {
        use crate::models::QueryMetadata;
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
            Bytes::from(r#"{"id":"test","value":1}"#),
        );
        let typed_response: Response<TestModel> = raw_response.into();
        let response = CosmosResponse::new(typed_response, create_mock_request());
        let response = response.map_metadata(QueryMetadata::from_headers);
        assert_eq!(
            response.metadata().index_metrics(),
            Some(r#"{"UtilizedSingleIndexes":[]}"#)
        );
        assert_eq!(
            response.metadata().query_metrics(),
            Some("totalExecutionTimeInMs=1.23;queryCompileTimeInMs=0.01")
        );
    }
}
