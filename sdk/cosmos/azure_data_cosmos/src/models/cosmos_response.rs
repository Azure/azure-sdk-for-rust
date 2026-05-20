// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Provides the [`CosmosResponse`] type for wrapping responses from Cosmos DB operations.

use std::sync::Arc;

use crate::models::{CosmosStatus, DiagnosticsContext, ResponseBody, ResponseHeaders};
use crate::SessionToken;
use azure_data_cosmos_driver::models::CosmosResponse as DriverResponse;
use serde::de::DeserializeOwned;

/// A response from a Cosmos DB operation.
///
/// Wraps the SDK-owned [`ResponseBody`], parsed [`ResponseHeaders`],
/// [`CosmosStatus`], and diagnostics. This type is internal to the SDK; public
/// wrapper types like [`ItemResponse`](crate::models::ItemResponse),
/// [`ResourceResponse`](crate::models::ResourceResponse), and
/// [`BatchResponse`](crate::models::BatchResponse) wrap it and expose only the
/// accessors relevant to their operation.
///
/// The body's deserialization target is supplied at the call site (via
/// [`into_model::<T>`](Self::into_model)) rather than as a struct parameter, so
/// the same `CosmosResponse` value can be inspected for status / headers
/// without committing to a specific `T`.
#[derive(Debug)]
pub(crate) struct CosmosResponse {
    body: ResponseBody,
    cosmos_headers: ResponseHeaders,
    status: CosmosStatus,
    diagnostics: Arc<DiagnosticsContext>,
}

impl CosmosResponse {
    /// Creates a `CosmosResponse` from the parts produced by the driver.
    ///
    /// The diagnostics context produced by the driver pipeline is plumbed through
    /// unchanged. Headers are already decoded by the driver (e.g., base64 for
    /// index metrics) so they are stored as-is.
    pub(crate) fn from_driver_parts(
        body: ResponseBody,
        cosmos_headers: ResponseHeaders,
        status: CosmosStatus,
        diagnostics: Arc<DiagnosticsContext>,
    ) -> Self {
        Self {
            body,
            cosmos_headers,
            status,
            diagnostics,
        }
    }

    /// Creates a `CosmosResponse` directly from a driver [`DriverResponse`].
    pub(crate) fn from_driver_response(driver_response: DriverResponse) -> Self {
        let status: CosmosStatus = driver_response.status();
        let cosmos_headers: ResponseHeaders = driver_response.headers().clone().into();
        let diagnostics = driver_response.diagnostics();
        let body: ResponseBody = driver_response.into_body().into();
        Self::from_driver_parts(body, cosmos_headers, status, diagnostics)
    }

    /// Returns the operation status.
    pub(crate) fn status(&self) -> CosmosStatus {
        self.status
    }

    /// Returns a reference to the parsed Cosmos-specific response headers.
    pub(crate) fn cosmos_headers(&self) -> &ResponseHeaders {
        &self.cosmos_headers
    }

    /// Consumes the response and returns the response body.
    pub(crate) fn into_body(self) -> ResponseBody {
        self.body
    }

    /// Returns the request charge (RU consumption) for this operation, if available.
    pub(crate) fn request_charge(&self) -> Option<f64> {
        self.cosmos_headers.request_charge().map(|rc| rc.value())
    }

    /// Returns the session token from this response, if available.
    pub(crate) fn session_token(&self) -> Option<SessionToken> {
        self.cosmos_headers
            .session_token()
            .map(|st| SessionToken::from(st.as_str().to_string()))
    }

    /// Returns a cloned [`Arc`] handle to the diagnostics for this operation.
    pub(crate) fn diagnostics(&self) -> Arc<DiagnosticsContext> {
        Arc::clone(&self.diagnostics)
    }

    /// Deserializes the response body into a model type.
    pub(crate) fn into_model<T: DeserializeOwned>(self) -> crate::Result<T> {
        self.body.into_single()
    }
}
