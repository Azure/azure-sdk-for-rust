// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Provides the [`BatchResponse`] type for transactional batch operation responses.

use std::sync::Arc;

use crate::models::{CosmosDiagnosticsContext, CosmosResponse};
use crate::transactional_batch::TransactionalBatchResponse;
use crate::SessionToken;
use azure_core::http::{headers::Headers, Etag, StatusCode};

/// A response from a transactional batch operation.
///
/// Provides access to common response metadata as well as the batch-level ETag.
///
/// Note: The batch-level ETag differs from a single-item ETag. It represents
/// the ETag for the entire batch operation, not an individual item's concurrency token.
/// Use individual [`TransactionalBatchOperationResult`](crate::TransactionalBatchOperationResult) entries for per-item ETags.
#[derive(Debug)]
pub struct BatchResponse {
    response: CosmosResponse<TransactionalBatchResponse>,
    etag: Option<Etag>,
}

impl BatchResponse {
    pub(crate) fn new(response: CosmosResponse<TransactionalBatchResponse>) -> Self {
        let etag = response
            .cosmos_headers()
            .etag
            .as_ref()
            .map(|e| Etag::from(e.as_str()));
        Self { response, etag }
    }

    /// Returns the batch-level ETag, if available.
    ///
    /// This is the ETag for the entire batch operation, not an individual item's
    /// concurrency token.
    pub fn etag(&self) -> Option<&Etag> {
        self.etag.as_ref()
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
    pub fn into_body(self) -> azure_core::http::response::ResponseBody {
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

    /// Deserializes the response body into the batch response model.
    pub fn into_model(self) -> crate::CosmosResult<TransactionalBatchResponse> {
        self.response.into_model()
    }
}
