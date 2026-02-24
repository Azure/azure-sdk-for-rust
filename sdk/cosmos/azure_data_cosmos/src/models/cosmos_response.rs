// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Provides the [`CosmosResponse`] type for wrapping responses from Cosmos DB operations.

use crate::cosmos_request::CosmosRequest;
use crate::SessionToken;
use azure_core::http::{
    headers::{HeaderName, Headers},
    response::Response,
    StatusCode,
};
use serde::de::DeserializeOwned;

/// A response from a Cosmos DB operation.
///
/// This wraps the underlying Azure Core typed response and provides convenient access
/// to headers, status code, the original request, and Cosmos-specific response metadata.
#[derive(Debug)]
pub struct CosmosResponse<T> {
    /// The underlying typed HTTP response.
    response: Response<T>,
    /// The final request used to fulfill the operation.
    #[allow(dead_code)]
    request: CosmosRequest,
}

impl<T> CosmosResponse<T> {
    /// Creates a new `CosmosResponse` from a typed response and the original request.
    pub(crate) fn new(response: Response<T>, request: CosmosRequest) -> Self {
        Self { response, request }
    }

    /// Returns the HTTP status code of the response.
    pub fn status(&self) -> StatusCode {
        self.response.status()
    }

    /// Returns a reference to all response headers.
    pub fn headers(&self) -> &Headers {
        self.response.headers()
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
        self.get_optional_header_str(&crate::constants::REQUEST_CHARGE)
            .and_then(|s| s.parse::<f64>().ok())
    }

    /// Returns the session token from this response, if available.
    pub fn session_token(&self) -> Option<SessionToken> {
        self.get_optional_header_str(&crate::constants::SESSION_TOKEN)
            .map(|s| SessionToken::from(s.to_string()))
    }

    /// Returns the ETag from this response, if available.
    pub fn etag(&self) -> Option<&str> {
        self.get_optional_header_str(&azure_core::http::headers::ETAG)
    }

    /// Deserializes the response body without consuming the response.
    ///
    /// This is used internally to extract a copy of the response model (e.g.,
    /// to populate a cache) while still returning the original response to the
    /// caller. The underlying `Bytes` body is reference-counted so the clone
    /// is cheap.
    pub(crate) fn deserialize_body<U: DeserializeOwned>(&self) -> azure_core::Result<U> {
        self.response.body().json()
    }
}

impl<T: DeserializeOwned> CosmosResponse<T> {
    /// Deserializes the response body into a model type.
    pub fn into_model(self) -> azure_core::Result<T> {
        self.response.into_body().json()
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
}
