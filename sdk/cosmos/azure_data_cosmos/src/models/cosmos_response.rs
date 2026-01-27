// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Provides the [`CosmosResponse`] type for wrapping responses from Cosmos DB operations.

use azure_core::http::{
    headers::{HeaderName, Headers},
    response::Response,
    StatusCode,
};
use serde::de::DeserializeOwned;

use crate::cosmos_request::CosmosRequest;

/// A response from a Cosmos DB operation.
///
/// This wraps the underlying Azure Core typed response and provides convenient access
/// to headers, status code, the original request, and Cosmos-specific response metadata.
#[derive(Debug)]
pub struct CosmosResponse<T> {
    /// The underlying typed HTTP response.
    response: Response<T>,
    /// The final Cosmos request that produced this response.
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

    /// Gets a header value as a string by name.
    ///
    /// Returns `Ok(&str)` if the header exists and is valid UTF-8,
    /// or an error otherwise.
    pub fn get_header_str(&self, name: &HeaderName) -> azure_core::Result<&str> {
        self.response.headers().get_str(name)
    }

    /// Gets an optional header value as a string by name.
    ///
    /// Returns `Some(&str)` if the header exists,
    /// or `None` if the header doesn't exist.
    pub fn get_optional_header_str(&self, name: &HeaderName) -> Option<&str> {
        self.response.headers().get_optional_str(name)
    }

    /// Returns a reference to the original Cosmos request.
    pub(crate) fn request(&self) -> &CosmosRequest {
        &self.request
    }

    /// Consumes the response and returns the underlying typed response.
    pub fn into_inner(self) -> Response<T> {
        self.response
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

    /// Returns the activity ID for this request, if available.
    pub fn activity_id(&self) -> Option<&str> {
        self.get_optional_header_str(&crate::constants::ACTIVITY_ID)
    }

    /// Returns the session token from this response, if available.
    pub fn session_token(&self) -> Option<&str> {
        self.get_optional_header_str(&crate::constants::SESSION_TOKEN)
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


