// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Provides the [`ItemResponse`] type for point item operation responses.

use std::{marker::PhantomData, sync::Arc};

use crate::models::{
    CosmosResponse, CosmosStatus, DiagnosticsContext, ResponseBody, ResponseHeaders,
};
use crate::SessionToken;
use serde::de::DeserializeOwned;

/// A response from a point item operation (create, read, replace, upsert, delete).
///
/// Provides access to common Cosmos response metadata and the item payload.
///
/// Headers are exposed via the typed [`ResponseHeaders`] struct; use
/// `response.headers().etag()` to access the ETag for optimistic concurrency
/// control.
#[derive(Debug)]
pub struct ItemResponse<T> {
    response: CosmosResponse,
    _marker: PhantomData<fn() -> T>,
}

impl<T> ItemResponse<T> {
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
    ///
    /// Use [`ResponseBody::single_item`] to deserialize the contained
    /// item, or [`into_model`](Self::into_model) for a one-shot convenience.
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

    /// The logical sequence number (LSN) of the partition replica that served this request.
    /// Advances with every write on the partition.
    pub fn lsn(&self) -> Option<u64> {
        self.response.cosmos_headers().lsn()
    }

    /// The logical sequence number (LSN) of the specific item/document operated on.
    /// Reflects the last write to this particular item.
    pub fn item_lsn(&self) -> Option<u64> {
        self.response.cosmos_headers().item_lsn()
    }
}

impl<T: DeserializeOwned> ItemResponse<T> {
    /// Deserializes the response body into a model type.
    pub fn into_model(self) -> azure_core::Result<T> {
        self.response.into_model::<T>()
    }
}
