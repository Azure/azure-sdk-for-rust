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
    pub fn new(response: Response<T>, request: CosmosRequest) -> Self {
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
    pub fn request(&self) -> &CosmosRequest {
        &self.request
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
}

impl<T: DeserializeOwned> CosmosResponse<T> {
    /// Deserializes the response body into a model type.
    pub fn into_model(self) -> azure_core::Result<T> {
        self.response.into_body().json()
    }
}
