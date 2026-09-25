// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Provides the [`ResourceResponse`] type for resource management operation responses.

use std::{marker::PhantomData, sync::Arc};

use crate::diagnostics::DiagnosticsContext;
use crate::models::CosmosStatus;
use crate::models::{CosmosResponse, ResponseBody, ResponseHeaders};
use azure_core::fmt::SafeDebug;
use serde::de::DeserializeOwned;

/// A response from a resource management operation (databases, containers, throughput).
///
/// Use [`into_model()`](Self::into_model) to deserialize the body as `T`,
/// or [`into_body()`](Self::into_body) to inspect the raw payload.
#[derive(SafeDebug)]
#[safe(true)]
#[non_exhaustive]
pub struct ResourceResponse<T> {
    response: CosmosResponse,
    _marker: PhantomData<fn() -> T>,
}

impl<T> ResourceResponse<T> {
    pub(crate) fn new(response: CosmosResponse) -> Self {
        Self {
            response,
            _marker: PhantomData,
        }
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
}

impl<T: DeserializeOwned> ResourceResponse<T> {
    /// Deserializes the response body as `T`.
    ///
    /// # Errors
    ///
    /// Returns an error if the body is not a single payload or cannot be
    /// deserialized as `T`.
    pub fn into_model(self) -> crate::Result<T> {
        self.response.into_model::<T>()
    }
}
