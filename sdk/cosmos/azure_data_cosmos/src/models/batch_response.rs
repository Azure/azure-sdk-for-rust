// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Provides the [`BatchResponse`] type for transactional batch operation responses.

use std::sync::Arc;

use crate::models::{
    CosmosResponse, CosmosStatus, DiagnosticsContext, ResponseBody, ResponseHeaders,
};
use crate::transactional_batch::TransactionalBatchResponse;
use crate::SessionToken;

/// A response from a transactional batch operation.
///
/// Provides access to common Cosmos response metadata.
///
/// Note: The batch-level ETag (available via `headers().etag()`) differs from a
/// single-item ETag. It represents the ETag for the entire batch operation, not
/// an individual item's concurrency token. Use individual
/// [`TransactionalBatchOperationResult`](crate::TransactionalBatchOperationResult)
/// entries for per-item ETags.
#[derive(Debug)]
#[non_exhaustive]
pub struct BatchResponse {
    response: CosmosResponse,
}

impl BatchResponse {
    pub(crate) fn new(response: CosmosResponse) -> Self {
        Self { response }
    }

    /// Returns the operation status.
    pub fn status(&self) -> CosmosStatus {
        self.response.status()
    }

    /// Returns a reference to the parsed Cosmos-specific response headers.
    pub fn headers(&self) -> &ResponseHeaders {
        self.response.cosmos_headers()
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
    /// The returned [`DiagnosticsContext`] surfaces the full per-operation
    /// diagnostics produced by the driver pipeline (request tracking, retries,
    /// regions contacted, RU charges, status, etc.).
    pub fn diagnostics(&self) -> Arc<DiagnosticsContext> {
        self.response.diagnostics()
    }

    /// Deserializes the response body into the batch response model.
    pub fn into_model(self) -> azure_core::Result<TransactionalBatchResponse> {
        self.response.into_model()
    }
}
