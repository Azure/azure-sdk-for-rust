// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Provides the [`BatchResponse`] type for transactional batch operation responses.

use std::sync::Arc;

use crate::diagnostics::DiagnosticsContext;
use crate::models::CosmosStatus;
use crate::models::TransactionalBatchResponse;
use crate::models::{CosmosResponse, ResponseBody, ResponseHeaders};
use azure_core::fmt::SafeDebug;

/// A response from a transactional batch operation.
///
/// Provides access to common Cosmos response metadata.
///
/// Note: The batch-level ETag (available via `headers().etag()`) differs from a
/// single-item ETag. It represents the ETag for the entire batch operation, not
/// an individual item's concurrency token. Use individual
/// [`TransactionalBatchOperationResult`](crate::models::TransactionalBatchOperationResult)
/// entries for per-item ETags.
#[derive(SafeDebug)]
#[safe(true)]
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

    /// Returns the diagnostics for this operation.
    ///
    /// The [`DiagnosticsContext`] includes attempts, regions contacted,
    /// request charges, and status.
    pub fn diagnostics(&self) -> Arc<DiagnosticsContext> {
        self.response.diagnostics()
    }

    /// Deserializes the response body into the batch response model.
    ///
    /// # Errors
    ///
    /// Returns an error if the body is not a single payload or cannot be
    /// deserialized as a [`TransactionalBatchResponse`].
    pub fn into_model(self) -> crate::Result<TransactionalBatchResponse> {
        self.response.into_model()
    }
}
