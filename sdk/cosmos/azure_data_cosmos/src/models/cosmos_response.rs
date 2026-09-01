// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Provides the [`CosmosResponse`] type for wrapping responses from Cosmos DB operations.

use std::sync::Arc;

use crate::diagnostics::DiagnosticsContext;
use crate::models::CosmosStatus;
#[cfg(feature = "preview_patch")]
use crate::models::PatchTrackingId;
use crate::models::{ResponseBody, ResponseHeaders};
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

    /// Returns a cloned [`Arc`] handle to the diagnostics for this operation.
    pub(crate) fn diagnostics(&self) -> Arc<DiagnosticsContext> {
        Arc::clone(&self.diagnostics)
    }

    /// Returns the effective duplicate-suppression identity for a tracked PATCH.
    #[cfg(feature = "preview_patch")]
    pub(crate) fn patch_tracking_id(&self) -> Option<PatchTrackingId> {
        self.diagnostics
            .patch_tracking_id()
            .map(PatchTrackingId::from_driver)
    }

    /// Deserializes the response body into a model type.
    pub(crate) fn into_model<T: DeserializeOwned>(self) -> crate::Result<T> {
        #[cfg(feature = "preview_patch")]
        let tracking_id = self.patch_tracking_id();
        let diagnostics = self.diagnostics;
        self.body.into_single().map_err(|error| {
            let error = error.with_diagnostics(diagnostics);
            #[cfg(feature = "preview_patch")]
            if let Some(tracking_id) = tracking_id {
                return error.with_patch_tracking_id(tracking_id);
            }
            error
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::http::StatusCode;
    use azure_data_cosmos_driver::{
        diagnostics::DiagnosticsContext,
        models::{ActivityId, CosmosStatus, ResponseBody as DriverResponseBody},
    };
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct StrictItem {
        #[allow(dead_code)]
        id: String,
    }

    #[test]
    fn model_decode_error_preserves_response_diagnostics() {
        let diagnostics = Arc::new(DiagnosticsContext::for_testing(ActivityId::new_uuid()));
        let response = CosmosResponse::from_driver_parts(
            DriverResponseBody::from_bytes(br#"{"id":"item","unexpected":true}"#.to_vec()).into(),
            ResponseHeaders::default(),
            CosmosStatus::new(StatusCode::Ok),
            Arc::clone(&diagnostics),
        );

        let error = response
            .into_model::<StrictItem>()
            .expect_err("strict model must reject the reserved/unknown field");

        assert_eq!(
            error.diagnostics().unwrap().activity_id(),
            diagnostics.activity_id()
        );
    }
}
