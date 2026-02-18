// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB operation result types.

use crate::{diagnostics::DiagnosticsContext, models::CosmosResponseHeaders};
use std::sync::Arc;

/// Result of a Cosmos DB operation.
///
/// Contains the response body (as raw bytes), relevant headers, and comprehensive
/// diagnostics for the entire operation including status codes.
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
/// // Status codes are accessed via diagnostics
/// let diagnostics = result.diagnostics();
/// if let Some(status) = diagnostics.status() {
///     println!("Status: {}", status);
///     println!("RU Charge: {}", result.headers().request_charge().unwrap_or(0.0));
///     if status.is_success() {
///         let body = result.into_body();
///         // Deserialize body...
///     }
/// }
/// ```
#[derive(Debug)]
#[non_exhaustive]
pub struct CosmosResult {
    /// Raw response body (UTF-8 JSON or Cosmos binary encoding).
    body: Vec<u8>,

    /// Extracted Cosmos-specific headers.
    headers: CosmosResponseHeaders,

    /// Full diagnostics context for this operation (contains status codes).
    diagnostics: Arc<DiagnosticsContext>,
}

impl CosmosResult {
    /// Creates a new `CosmosResult`.
    ///
    /// This is typically called by the driver after completing an operation.
    /// The diagnostics context should already contain the status codes
    /// (set via `DiagnosticsContextBuilder::set_operation_status` before completion).
    pub(crate) fn new(
        body: Vec<u8>,
        headers: CosmosResponseHeaders,
        diagnostics: Arc<DiagnosticsContext>,
    ) -> Self {
        Self {
            body,
            headers,
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

    /// Consumes the result and returns all parts.
    pub(crate) fn into_parts(self) -> (Vec<u8>, CosmosResponseHeaders, Arc<DiagnosticsContext>) {
        (self.body, self.headers, self.diagnostics)
    }

    /// Returns a reference to the extracted headers.
    pub fn headers(&self) -> &CosmosResponseHeaders {
        &self.headers
    }

    /// Returns a reference to the diagnostics context.
    ///
    /// The diagnostics context contains detailed information about all
    /// requests made during this operation, including retries, hedging,
    /// and regional failovers. It also holds the operation-level status codes.
    pub fn diagnostics(&self) -> &Arc<DiagnosticsContext> {
        &self.diagnostics
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        diagnostics::DiagnosticsContextBuilder,
        models::{ActivityId, CosmosResponseHeaders, CosmosStatus, RequestCharge, SubStatusCode},
        options::DiagnosticsOptions,
    };
    use azure_core::http::StatusCode;

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
    fn cosmos_result_accessors() {
        let headers = CosmosResponseHeaders::new()
            .with_request_charge(RequestCharge::new(5.5))
            .with_activity_id(ActivityId::from_string("test-activity".to_string()));

        let result = CosmosResult::new(
            b"{\"id\": \"test\"}".to_vec(),
            headers,
            make_diagnostics(Some(StatusCode::Ok), None),
        );

        // Status codes are accessed via diagnostics
        let status = result.diagnostics().status().unwrap();
        assert_eq!(status.status_code(), StatusCode::Ok);
        assert!(status.is_success());
        assert!(status.sub_status_code().is_none());
        assert_eq!(result.body(), b"{\"id\": \"test\"}");
        assert_eq!(
            result.headers().request_charge(),
            Some(RequestCharge::new(5.5))
        );
    }

    #[test]
    fn cosmos_result_error_status() {
        let result = CosmosResult::new(
            b"{}".to_vec(),
            CosmosResponseHeaders::new(),
            make_diagnostics(
                Some(StatusCode::TooManyRequests),
                Some(SubStatusCode::new(3200)),
            ),
        );

        let status = result.diagnostics().status().unwrap();
        assert!(!status.is_success());
        assert!(status.is_throttled());
        assert_eq!(status, &CosmosStatus::RU_BUDGET_EXCEEDED);
    }

    #[test]
    fn cosmos_result_into_parts() {
        let headers = CosmosResponseHeaders::new().with_request_charge(RequestCharge::new(1.0));
        let result = CosmosResult::new(
            b"body".to_vec(),
            headers,
            make_diagnostics(Some(StatusCode::Created), None),
        );

        let (body, headers, diagnostics) = result.into_parts();
        assert_eq!(body, b"body");
        assert_eq!(
            diagnostics.status().unwrap().status_code(),
            StatusCode::Created
        );
        assert!(diagnostics.status().unwrap().sub_status_code().is_none());
        assert_eq!(headers.request_charge(), Some(RequestCharge::new(1.0)));
    }

    #[test]
    fn cosmos_result_status_via_diagnostics() {
        let diagnostics = make_diagnostics(
            Some(StatusCode::NotFound),
            Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE),
        );
        let result = CosmosResult::new(
            b"{}".to_vec(),
            CosmosResponseHeaders::new(),
            diagnostics.clone(),
        );

        // Status codes are only accessible via diagnostics
        let status = diagnostics.status().unwrap();
        assert_eq!(status.status_code(), StatusCode::NotFound);
        assert!(status.is_read_session_not_available());
        // Same via result.diagnostics()
        let result_status = result.diagnostics().status().unwrap();
        assert_eq!(result_status.status_code(), StatusCode::NotFound);
        assert!(result_status.is_read_session_not_available());
    }
}
