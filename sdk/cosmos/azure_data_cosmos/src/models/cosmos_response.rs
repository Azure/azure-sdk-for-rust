// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Provides the [`CosmosResponse`] type for wrapping responses from Cosmos DB operations.

use std::sync::Arc;

use crate::models::CosmosDiagnosticsContext;
use crate::SessionToken;
use azure_core::http::{headers::Headers, response::Response, StatusCode};
use azure_data_cosmos_driver::models::CosmosResponseHeaders;
use serde::de::DeserializeOwned;

/// A response from a Cosmos DB operation.
///
/// This wraps the underlying Azure Core typed response and provides convenient access
/// to headers, status code, and Cosmos-specific response metadata.
///
/// This type is internal to the SDK. Public response types like
/// [`ItemResponse`](crate::models::ItemResponse),
/// [`ResourceResponse`](crate::models::ResourceResponse), and
/// [`BatchResponse`](crate::models::BatchResponse) wrap this type and expose
/// only the accessors relevant to their operation.
#[derive(Debug)]
pub(crate) struct CosmosResponse<T> {
    /// The underlying typed HTTP response.
    response: Response<T>,
    /// Parsed Cosmos-specific response headers.
    cosmos_headers: CosmosResponseHeaders,
    /// Diagnostics for this operation.
    ///
    /// Stored as an [`Arc`] so it can be cheaply shared across types
    /// (for example, [`FeedPage`](crate::feed::FeedPage)) without cloning the
    /// underlying request data.
    diagnostics: Arc<CosmosDiagnosticsContext>,
}

impl<T> CosmosResponse<T> {
    /// Creates a `CosmosResponse` from a typed response and pre-parsed driver headers.
    ///
    /// Used by the driver bridge to avoid double-parsing response headers.
    /// The driver already decodes headers (e.g., base64 for index metrics),
    /// so re-parsing from raw headers would fail on values that are no longer
    /// in their wire format. The diagnostics context produced by the driver
    /// pipeline is plumbed through unchanged.
    pub(crate) fn from_driver_response(
        response: Response<T>,
        cosmos_headers: CosmosResponseHeaders,
        diagnostics: Arc<CosmosDiagnosticsContext>,
    ) -> Self {
        Self {
            response,
            cosmos_headers,
            diagnostics,
        }
    }

    /// Returns the HTTP status code of the response.
    pub(crate) fn status(&self) -> StatusCode {
        self.response.status()
    }

    /// Returns a reference to all response headers.
    pub(crate) fn headers(&self) -> &Headers {
        self.response.headers()
    }

    /// Returns a reference to the parsed Cosmos-specific response headers.
    pub(crate) fn cosmos_headers(&self) -> &CosmosResponseHeaders {
        &self.cosmos_headers
    }

    /// Consumes the response and returns the response body.
    pub(crate) fn into_body(self) -> azure_core::http::response::ResponseBody {
        self.response.into_body()
    }

    /// Returns the request charge (RU consumption) for this operation, if available.
    pub(crate) fn request_charge(&self) -> Option<f64> {
        self.cosmos_headers
            .request_charge
            .as_ref()
            .map(|rc| rc.value())
    }

    /// Returns the session token from this response, if available.
    pub(crate) fn session_token(&self) -> Option<SessionToken> {
        self.cosmos_headers
            .session_token
            .as_ref()
            .map(|st| SessionToken::from(st.as_str().to_string()))
    }

    /// Returns a cloned [`Arc`] handle to the diagnostics for this operation.
    ///
    /// The returned [`Arc`] derefs transparently to
    /// [`CosmosDiagnosticsContext`] for read-only inspection, and can be
    /// retained alongside a consumed response body (used, for example, by
    /// [`FeedPage`](crate::feed::FeedPage)).
    pub(crate) fn diagnostics(&self) -> Arc<CosmosDiagnosticsContext> {
        Arc::clone(&self.diagnostics)
    }
}

impl<T: DeserializeOwned> CosmosResponse<T> {
    /// Deserializes the response body into a model type.
    ///
    /// On parse failure, the operation diagnostics are attached to the
    /// returned [`CosmosError`](crate::CosmosError) so callers can correlate
    /// the deserialization failure with the underlying HTTP exchange
    /// (ActivityId, region, status, per-attempt history, etc.).
    pub(crate) fn into_model(self) -> crate::CosmosResult<T> {
        let diagnostics = Arc::clone(&self.diagnostics);
        self.response.into_body().json().map_err(|e| {
            let azure_err =
                azure_data_cosmos_driver::diagnostics::attach_diagnostics(e, diagnostics);
            crate::CosmosError::from(azure_err)
        })
    }
}
