// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Provides the [`ItemResponse`] type for point item operation responses.

use crate::models::{CosmosDiagnostics, CosmosResponse};
use crate::SessionToken;
use azure_core::http::{headers::Headers, response::ResponseBody, Etag, StatusCode};
use serde::de::DeserializeOwned;

/// A response from a point item operation (create, read, replace, upsert, delete).
///
/// Provides access to common response metadata as well as the item-specific
/// ETag for optimistic concurrency control.
#[derive(Debug)]
pub struct ItemResponse<T> {
    response: CosmosResponse<T>,
    etag: Option<Etag>,
}

impl<T> ItemResponse<T> {
    pub(crate) fn new(response: CosmosResponse<T>) -> Self {
        let etag = response
            .cosmos_headers()
            .etag
            .as_ref()
            .map(|e| Etag::from(e.as_str()));
        Self { response, etag }
    }

    /// Returns the ETag for optimistic concurrency control, if available.
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

    /// Returns the final request URL used to fulfill the operation.
    /// Returns `None` for driver-routed operations.
    /// This api is subject to change without a major version bump.
    #[cfg(feature = "fault_injection")]
    pub fn request_url(&self) -> Option<azure_core::http::Url> {
        self.response.request_url()
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
    pub fn diagnostics(&self) -> &CosmosDiagnostics {
        self.response.diagnostics()
    }

    /// The logical sequence number (LSN) of the partition replica that served this request.
    /// Advances with every write on the partition.
    pub fn lsn(&self) -> Option<u64> {
        self.response.cosmos_headers().lsn
    }

    /// The logical sequence number (LSN) of the specific item/document operated on.
    /// Reflects the last write to this particular item.
    pub fn item_lsn(&self) -> Option<u64> {
        self.response.cosmos_headers().item_lsn
    }
}

impl<T: DeserializeOwned> ItemResponse<T> {
    /// Deserializes the response body into a model type.
    pub fn into_model(self) -> azure_core::Result<T> {
        self.response.into_model()
    }
}
