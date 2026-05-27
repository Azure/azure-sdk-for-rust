// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB operation result types.

use crate::diagnostics::DiagnosticsContext;
use crate::models::{CosmosResponseHeaders, CosmosStatus, ResponseBody};
use std::sync::Arc;

/// Wire-level payload of a Cosmos DB response — the response body plus the
/// parsed Cosmos-specific headers. This is the portion of a response that
/// is also meaningful on an [`CosmosError`](crate::error::CosmosError) (which keeps its
/// own copy of [`CosmosStatus`] and the operation
/// [`DiagnosticsContext`](crate::diagnostics::DiagnosticsContext)).
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub(crate) struct CosmosResponsePayload {
    /// Response body, possibly composed of multiple byte slices.
    body: ResponseBody,

    /// Extracted Cosmos-specific headers.
    headers: CosmosResponseHeaders,
}

impl CosmosResponsePayload {
    /// Creates a new payload from a body and parsed headers.
    pub(crate) fn new(body: impl Into<ResponseBody>, headers: CosmosResponseHeaders) -> Self {
        Self {
            body: body.into(),
            headers,
        }
    }

    /// Returns a reference to the typed response body.
    pub(crate) fn body(&self) -> &ResponseBody {
        &self.body
    }

    /// Consumes the payload and returns the body.
    pub(crate) fn into_body(self) -> ResponseBody {
        self.body
    }

    /// Returns a reference to the extracted headers.
    pub(crate) fn headers(&self) -> &CosmosResponseHeaders {
        &self.headers
    }
}

/// Result of a Cosmos DB operation.
///
/// Contains the response body (as a [`ResponseBody`] of one or more
/// reference-counted byte slices), relevant headers, and comprehensive status
/// information for the operation.
///
/// # Schema-Agnostic Design
///
/// The driver returns response bodies as raw bytes via [`ResponseBody`].
/// The higher-level SDK (e.g., `azure_data_cosmos`) handles deserialization into
/// typed structures. This allows the driver to be reused across different
/// serialization strategies.
///
/// # Example
///
/// ```ignore
/// let result = driver.execute_operation(/* ... */).await?;
///
/// let status = result.status();
/// println!("Status: {}", status);
/// println!("RU Charge: {}", result.headers().request_charge.unwrap_or_default().value());
/// if status.is_success() {
///     let body = result.into_body();
///     // Deserialize body...
/// }
/// ```
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CosmosResponse {
    /// Wire-level payload (body + parsed headers).
    payload: CosmosResponsePayload,

    /// Operation status including HTTP status code and optional sub-status.
    status: CosmosStatus,

    /// Full diagnostics context for this operation.
    diagnostics: Arc<DiagnosticsContext>,
}

impl CosmosResponse {
    /// Creates a new `CosmosResponse`.
    ///
    /// This is typically called by the driver after completing an operation.
    /// The `body` may be supplied as raw bytes (e.g., `Vec<u8>`, `Bytes`) and
    /// will be wrapped as a single-part [`ResponseBody`].
    pub(crate) fn new(
        body: impl Into<ResponseBody>,
        headers: CosmosResponseHeaders,
        status: CosmosStatus,
        diagnostics: Arc<DiagnosticsContext>,
    ) -> Self {
        Self {
            payload: CosmosResponsePayload::new(body, headers),
            status,
            diagnostics,
        }
    }

    /// Returns a reference to the wire-level payload (body + headers).
    pub(crate) fn payload(&self) -> &CosmosResponsePayload {
        &self.payload
    }

    /// Returns a reference to the typed response body.
    pub fn body(&self) -> &ResponseBody {
        self.payload.body()
    }

    /// Test-only helper: returns the body as raw bytes, panicking if the body is
    /// not a [`ResponseBody::Bytes`] variant.
    #[cfg(test)]
    pub(crate) fn body_bytes(&self) -> &[u8] {
        match self.body() {
            ResponseBody::Bytes(b) => b.as_ref(),
            _ => panic!("expected ResponseBody::Bytes"),
        }
    }

    /// Consumes the response and returns the body.
    pub fn into_body(self) -> ResponseBody {
        self.payload.into_body()
    }

    /// Returns a reference to the extracted headers.
    pub fn headers(&self) -> &CosmosResponseHeaders {
        self.payload.headers()
    }

    /// Returns the operation status.
    pub fn status(&self) -> CosmosStatus {
        self.status
    }

    /// Returns a cloned [`Arc`] handle to the diagnostics captured for this operation.
    ///
    /// Cloning the [`Arc`] is a cheap atomic increment, allowing the diagnostics
    /// to be retained alongside the response data when the [`CosmosResponse`] is
    /// consumed (for example, by [`into_body`](Self::into_body)). For read-only
    /// inspection, the returned handle derefs transparently to
    /// [`DiagnosticsContext`].
    pub fn diagnostics(&self) -> Arc<DiagnosticsContext> {
        Arc::clone(&self.diagnostics)
    }

    /// Returns a borrow of the diagnostics [`Arc`] without cloning it.
    pub fn diagnostics_ref(&self) -> &Arc<DiagnosticsContext> {
        &self.diagnostics
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::DiagnosticsContextBuilder;
    use crate::models::{ActivityId, CosmosResponseHeaders, RequestCharge, SubStatusCode};
    use crate::options::DiagnosticsOptions;
    use azure_core::http::StatusCode;
    use std::sync::Arc;

    fn make_status(
        status_code: Option<StatusCode>,
        sub_status_code: Option<SubStatusCode>,
    ) -> CosmosStatus {
        CosmosStatus::from_parts(status_code.unwrap_or(StatusCode::Ok), sub_status_code)
    }

    fn make_diagnostics(
        status_code: Option<StatusCode>,
        sub_status_code: Option<SubStatusCode>,
    ) -> Arc<DiagnosticsContext> {
        let mut builder = DiagnosticsContextBuilder::new(
            ActivityId::new_uuid(),
            Arc::new(DiagnosticsOptions::default()),
        );
        if let Some(status) = status_code {
            builder.set_operation_status(status, sub_status_code);
        }
        Arc::new(builder.complete())
    }

    #[test]
    fn cosmos_response_accessors() {
        let headers = CosmosResponseHeaders {
            request_charge: Some(RequestCharge::new(5.5)),
            activity_id: Some(ActivityId::from_string("test-activity".to_string())),
            ..Default::default()
        };

        let result = CosmosResponse::new(
            b"{\"id\": \"test\"}".to_vec(),
            headers,
            make_status(Some(StatusCode::Ok), None),
            make_diagnostics(Some(StatusCode::Ok), None),
        );

        let status = result.status();
        assert_eq!(status.status_code(), StatusCode::Ok);
        assert!(status.is_success());
        assert!(status.sub_status().is_none());
        match result.body() {
            ResponseBody::Bytes(b) => assert_eq!(b.as_ref(), b"{\"id\": \"test\"}"),
            _ => panic!("expected Bytes variant"),
        }
        assert_eq!(
            result.headers().request_charge,
            Some(RequestCharge::new(5.5))
        );
    }

    #[test]
    fn cosmos_response_error_status() {
        let result = CosmosResponse::new(
            b"{}".to_vec(),
            CosmosResponseHeaders::new(),
            make_status(
                Some(StatusCode::TooManyRequests),
                Some(SubStatusCode::new(3200)),
            ),
            make_diagnostics(
                Some(StatusCode::TooManyRequests),
                Some(SubStatusCode::new(3200)),
            ),
        );

        let status = result.status();
        assert!(!status.is_success());
        assert!(status.is_throttled());
        assert_eq!(status, CosmosStatus::RU_BUDGET_EXCEEDED);
    }

    #[test]
    fn cosmos_response_accessors_created_status() {
        let headers = CosmosResponseHeaders {
            request_charge: Some(RequestCharge::new(1.0)),
            ..Default::default()
        };
        let result = CosmosResponse::new(
            b"body".to_vec(),
            headers,
            make_status(Some(StatusCode::Created), None),
            make_diagnostics(Some(StatusCode::Created), None),
        );

        match result.body() {
            ResponseBody::Bytes(b) => assert_eq!(b.as_ref(), b"body"),
            _ => panic!("expected Bytes variant"),
        }
        assert_eq!(result.status().status_code(), StatusCode::Created);
        assert!(result.status().sub_status().is_none());
        assert_eq!(
            result.headers().request_charge,
            Some(RequestCharge::new(1.0))
        );

        let body = result.into_body();
        let bytes = body.single().unwrap();
        assert_eq!(&bytes[..], b"body");
    }

    #[test]
    fn cosmos_response_status_accessor() {
        let status = make_status(
            Some(StatusCode::NotFound),
            Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE),
        );
        let result = CosmosResponse::new(
            b"{}".to_vec(),
            CosmosResponseHeaders::new(),
            status,
            make_diagnostics(
                Some(StatusCode::NotFound),
                Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE),
            ),
        );

        let result_status = result.status();
        assert_eq!(result_status.status_code(), StatusCode::NotFound);
        assert!(result_status.is_read_session_not_available());
        let diagnostics = result.diagnostics();
        let diagnostics_status = diagnostics.status().unwrap();
        assert_eq!(diagnostics_status.status_code(), StatusCode::NotFound);
    }
}
