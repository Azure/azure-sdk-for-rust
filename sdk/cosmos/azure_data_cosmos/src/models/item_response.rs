// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Provides the [`ItemResponse`] type for point item operation responses.

use std::sync::Arc;

use crate::diagnostics::DiagnosticsContext;
use crate::models::CosmosStatus;
use crate::models::{CosmosResponse, ResponseBody, ResponseHeaders};
use azure_core::fmt::SafeDebug;
use serde::de::DeserializeOwned;

/// A response from a point item operation (create, read, replace, upsert, delete).
///
/// Provides access to common Cosmos response metadata and the item payload.
///
/// Headers are exposed via the typed [`ResponseHeaders`] struct; use
/// `response.headers().etag()` to access the ETag for optimistic concurrency
/// control. The item payload is consumed via [`into_body`](Self::into_body)
/// or deserialized in one shot via [`into_model::<T>`](Self::into_model).
#[derive(SafeDebug)]
#[safe(true)]
#[non_exhaustive]
pub struct ItemResponse {
    response: CosmosResponse,
}

impl ItemResponse {
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
    ///
    /// Use [`ResponseBody::into_single`] to deserialize the contained
    /// item, or [`into_model::<T>`](Self::into_model) for a one-shot convenience.
    pub fn into_body(self) -> ResponseBody {
        self.response.into_body()
    }

    /// Returns the diagnostics for this operation.
    ///
    /// The returned [`DiagnosticsContext`] surfaces the full per-operation
    /// diagnostics produced by the driver pipeline (request tracking, retries,
    /// regions contacted, RU charges, status, etc.).
    pub fn diagnostics(&self) -> Arc<DiagnosticsContext> {
        self.response.diagnostics()
    }

    /// Deserializes the response body into a model type.
    ///
    /// The target type `T` is supplied at the call site (turbofish) because
    /// `ItemResponse` no longer carries a type parameter; this lets callers
    /// inspect status / headers / diagnostics without committing to a `T`.
    pub fn into_model<T: DeserializeOwned>(self) -> crate::Result<T> {
        self.response.into_model::<T>()
    }
}
