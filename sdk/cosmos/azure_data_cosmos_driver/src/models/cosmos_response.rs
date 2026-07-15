// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB operation result types.

use crate::diagnostics::DiagnosticsContext;
use crate::models::{CosmosResponseHeaders, CosmosStatus};
use std::sync::Arc;

/// Result of a Cosmos DB operation.
///
/// Contains the response body (as raw bytes), relevant headers, comprehensive
/// status information, and the always-collected [`DiagnosticsContext`] for the operation.
///
/// # Schema-Agnostic Design
///
/// The driver returns response bodies as raw bytes (`Vec<u8>`). The higher-level
/// SDK (e.g., `azure_data_cosmos`) handles deserialization into typed structures.
/// This allows the driver to be reused across different serialization strategies.
///
/// # Example
///
/// ```ignore
/// let result = driver.execute_operation(/* ... */).await?;
///
/// let status = result.status();
/// println!("Status: {}", status);
/// println!("RU Charge: {}", result.headers().request_charge.unwrap_or_default().value());
///
/// // Diagnostics are always available.
/// let diagnostics = result.diagnostics();
/// println!("Operation: {}", diagnostics.operation_name());
///
/// if status.is_success() {
///     let body = result.into_body();
///     // Deserialize body...
/// }
/// ```
#[derive(Debug)]
#[non_exhaustive]
pub struct CosmosResponse {
    /// Raw response body (UTF-8 JSON or Cosmos binary encoding).
    body: Vec<u8>,

    /// Extracted Cosmos-specific headers.
    headers: CosmosResponseHeaders,

    /// Operation status including HTTP status code and optional sub-status.
    status: CosmosStatus,

    /// Always-collected operation diagnostics.
    diagnostics: Arc<DiagnosticsContext>,
}

impl CosmosResponse {
    /// Creates a new `CosmosResponse`.
    ///
    /// This is typically called by the driver after completing an operation, passing the
    /// [`DiagnosticsContext`] produced while executing it.
    pub(crate) fn new(
        body: Vec<u8>,
        headers: CosmosResponseHeaders,
        status: CosmosStatus,
        diagnostics: Arc<DiagnosticsContext>,
    ) -> Self {
        Self {
            body,
            headers,
            status,
            diagnostics,
        }
    }

    /// Returns a reference to the response body.
    ///
    /// The body is raw bytes - typically UTF-8 JSON but may be Cosmos binary
    /// encoding for certain operations. The higher-level SDK handles parsing.
    pub fn body(&self) -> &[u8] {
        &self.body
    }

    /// Consumes the result and returns the body.
    pub fn into_body(self) -> Vec<u8> {
        self.body
    }

    /// Returns a reference to the extracted headers.
    pub fn headers(&self) -> &CosmosResponseHeaders {
        &self.headers
    }

    /// Returns the operation status.
    pub fn status(&self) -> CosmosStatus {
        self.status
    }

    /// Returns the operation's diagnostics context.
    ///
    /// Diagnostics are always collected, so this never returns `None`. The returned
    /// [`DiagnosticsContext`] is reference-counted and cheap to clone.
    pub fn diagnostics(&self) -> Arc<DiagnosticsContext> {
        self.diagnostics.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::{DiagnosticsContextBuilder, ExecutionContext, RequestDiagnostics};
    use crate::models::{ActivityId, CosmosResponseHeaders, RequestCharge, SubStatusCode};
    use crate::options::{DiagnosticsOptions, Region};
    use azure_core::http::StatusCode;

    fn make_status(
        status_code: Option<StatusCode>,
        sub_status_code: Option<SubStatusCode>,
    ) -> CosmosStatus {
        CosmosStatus::from_parts(status_code.unwrap_or(StatusCode::Ok), sub_status_code)
    }

    /// Builds a completed diagnostics context recording a single attempt with the given status.
    fn diagnostics(status: CosmosStatus) -> Arc<DiagnosticsContext> {
        let mut builder = DiagnosticsContextBuilder::new(
            "read_item",
            ActivityId::from_static("test-op"),
            true,
            Arc::new(DiagnosticsOptions::default()),
        );
        builder.record_request(
            RequestDiagnostics::new(ExecutionContext::Initial, "https://acct/", status)
                .with_region(Region::WEST_US_2)
                .with_request_charge(RequestCharge::new(1.0)),
        );
        Arc::new(builder.complete(Some(status)))
    }

    #[test]
    fn cosmos_response_accessors() {
        let headers = CosmosResponseHeaders {
            request_charge: Some(RequestCharge::new(5.5)),
            activity_id: Some(ActivityId::from_string("test-activity".to_string())),
            ..Default::default()
        };

        let status = make_status(Some(StatusCode::Ok), None);
        let result = CosmosResponse::new(
            b"{\"id\": \"test\"}".to_vec(),
            headers,
            status,
            diagnostics(status),
        );

        let status = result.status();
        assert_eq!(status.status_code(), StatusCode::Ok);
        assert!(status.is_success());
        assert!(status.sub_status().is_none());
        assert_eq!(result.body(), b"{\"id\": \"test\"}");
        assert_eq!(
            result.headers().request_charge,
            Some(RequestCharge::new(5.5))
        );
    }

    #[test]
    fn cosmos_response_error_status() {
        let status = make_status(
            Some(StatusCode::TooManyRequests),
            Some(SubStatusCode::new(3200)),
        );
        let result = CosmosResponse::new(
            b"{}".to_vec(),
            CosmosResponseHeaders::new(),
            status,
            diagnostics(status),
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
        let status = make_status(Some(StatusCode::Created), None);
        let result = CosmosResponse::new(b"body".to_vec(), headers, status, diagnostics(status));

        assert_eq!(result.body(), b"body");
        assert_eq!(result.status().status_code(), StatusCode::Created);
        assert!(result.status().sub_status().is_none());
        assert_eq!(
            result.headers().request_charge,
            Some(RequestCharge::new(1.0))
        );

        let body = result.into_body();
        assert_eq!(body, b"body");
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
            diagnostics(status),
        );

        let result_status = result.status();
        assert_eq!(result_status.status_code(), StatusCode::NotFound);
        assert!(result_status.is_read_session_not_available());
    }

    #[test]
    fn cosmos_response_exposes_diagnostics() {
        let status = make_status(Some(StatusCode::Ok), None);
        let result = CosmosResponse::new(
            b"{}".to_vec(),
            CosmosResponseHeaders::new(),
            status,
            diagnostics(status),
        );

        let diag = result.diagnostics();
        assert_eq!(diag.operation_name(), "read_item");
        assert!(diag.is_completed());
        assert!(!diag.is_failure());
        assert_eq!(diag.requests().len(), 1);
        assert_eq!(diag.contacted_regions(), &[Region::WEST_US_2]);
    }
}
