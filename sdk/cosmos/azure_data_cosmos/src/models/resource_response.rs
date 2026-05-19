// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Provides the [`ResourceResponse`] type for resource management operation responses.

use std::sync::Arc;

use crate::models::{CosmosDiagnosticsContext, CosmosResponse};
use crate::SessionToken;
use azure_core::http::{headers::Headers, response::ResponseBody, StatusCode};
use serde::de::DeserializeOwned;

/// A response from a resource management operation (databases, containers, throughput).
///
/// Provides access to common response metadata.
/// Currently has no operation-specific fields, but using a dedicated type ensures
/// future fields can be added without breaking changes.
#[derive(Debug)]
pub struct ResourceResponse<T> {
    response: CosmosResponse<T>,
}

impl<T> ResourceResponse<T> {
    pub(crate) fn new(response: CosmosResponse<T>) -> Self {
        Self { response }
    }

    /// Returns the HTTP status code of the response.
    pub fn status(&self) -> StatusCode {
        self.response.status()
    }

    /// Returns a reference to all response headers.
    pub fn headers(&self) -> &Headers {
        self.response.headers()
    }

    /// Consumes the response and returns the response body.
    pub fn into_body(self) -> ResponseBody {
        self.response.into_body()
    }

    /// Returns the request charge (RU consumption) for this operation, if available.
    pub fn request_charge(&self) -> Option<f64> {
        self.response.request_charge()
    }

    /// Returns the session token from this response, if available.
    pub fn session_token(&self) -> Option<SessionToken> {
        self.response.session_token()
    }

    /// Returns the diagnostics for this operation.
    ///
    /// The returned [`CosmosDiagnosticsContext`] surfaces the full per-operation
    /// diagnostics produced by the driver pipeline (request tracking, retries,
    /// regions contacted, RU charges, status, etc.).
    pub fn diagnostics(&self) -> Arc<CosmosDiagnosticsContext> {
        self.response.diagnostics()
    }
}

impl<T: DeserializeOwned> ResourceResponse<T> {
    /// Deserializes the response body into a model type.
    pub fn into_model(self) -> azure_core::Result<T> {
        self.response.into_model()
    }
}
