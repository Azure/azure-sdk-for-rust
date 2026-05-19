// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Provides the [`ResourceResponse`] type for resource management operation responses.

use std::{marker::PhantomData, sync::Arc};

use crate::models::{
    CosmosResponse, CosmosStatus, DiagnosticsContext, ResponseBody, ResponseHeaders,
};
use crate::SessionToken;
use serde::de::DeserializeOwned;

/// A response from a resource management operation (databases, containers, throughput).
///
/// Carries common Cosmos response metadata plus a type parameter `T` that names
/// the model the body deserializes into. Unlike [`ItemResponse`](crate::ItemResponse)
/// — where the payload type is user-defined and the SDK never knows it — every
/// `ResourceResponse`-returning client method statically knows its model
/// (`DatabaseProperties`, `ContainerProperties`, `ThroughputProperties`, …), so
/// keeping `T` on the response type lets callers write `.into_model()?` without
/// a turbofish.
#[derive(Debug)]
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
}

impl<T: DeserializeOwned> ResourceResponse<T> {
    /// Deserializes the response body into the model type `T` named by this
    /// response.
    pub fn into_model(self) -> azure_core::Result<T> {
        self.response.into_model::<T>()
    }
}
