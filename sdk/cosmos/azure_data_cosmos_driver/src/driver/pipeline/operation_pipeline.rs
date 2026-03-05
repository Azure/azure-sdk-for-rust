// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Operation pipeline: the core loop for executing Cosmos DB operations.
//!
//! This is the "slim" Step 1 version: single-region, no hedging, no circuit
//! breaker, no session consistency. It establishes the architectural pattern
//! (7-stage loop) that later steps will expand.

use std::sync::Arc;
use std::time::Instant;

use azure_core::http::headers::{HeaderName, HeaderValue};
use azure_core::http::HttpClient;
use url::Url;

use crate::{
    diagnostics::{DiagnosticsContextBuilder, ExecutionContext, PipelineType, TransportSecurity},
    models::{
        AccountEndpoint, ActivityId, CosmosOperation, CosmosResponse, CosmosResponseHeaders,
        Credential, SubStatusCode,
    },
    options::{OperationOptions, Region, RuntimeOptions},
};

use super::{
    components::{
        OperationAction, OperationRetryState, RoutingDecision, TransportOutcome, TransportRequest,
        TransportResult,
    },
    retry_evaluation::evaluate_transport_result,
};

use crate::driver::transport::{
    transport_pipeline::execute_transport_pipeline, AuthorizationContext,
};

/// Executes a Cosmos DB operation through the new pipeline architecture.
///
/// This is the entry point called by `CosmosDriver::execute_operation`.
/// It orchestrates the 7-stage operation loop described in the spec.
#[allow(clippy::too_many_arguments)]
pub(crate) async fn execute_operation_pipeline(
    operation: &CosmosOperation,
    _options: &OperationOptions,
    effective_options: &RuntimeOptions,
    endpoint: &AccountEndpoint,
    region: Option<Region>,
    http_client: Arc<dyn HttpClient>,
    credential: &Credential,
    user_agent: &str,
    activity_id: &ActivityId,
    pipeline_type: PipelineType,
    transport_security: TransportSecurity,
    diagnostics: DiagnosticsContextBuilder,
) -> azure_core::Result<CosmosResponse> {
    let mut diagnostics = diagnostics;
    let mut retry_state = OperationRetryState::initial();

    let deadline = effective_options
        .end_to_end_latency_policy
        .as_ref()
        .map(|p| Instant::now() + p.timeout());

    loop {
        // ── STAGE 1: Acquire LocationSnapshot ──────────────────────────
        // Step 1: no LocationSnapshot — single region.

        // ── STAGE 2: Resolve endpoint ──────────────────────────────────
        let routing = resolve_endpoint(endpoint, region.clone());

        // ── STAGE 3: Build transport request ───────────────────────────
        let execution_context = if retry_state.transport_retry_count == 0 {
            ExecutionContext::Initial
        } else {
            ExecutionContext::Retry
        };

        let transport_request = build_transport_request(
            operation,
            &routing,
            activity_id,
            execution_context,
            deadline,
        );

        // ── STAGE 4: Execute via transport pipeline ────────────────────
        let result = execute_transport_pipeline(
            transport_request,
            http_client.as_ref(),
            credential,
            user_agent,
            pipeline_type,
            transport_security,
            &mut diagnostics,
        )
        .await;

        // ── STAGE 5: Evaluate result → action ──────────────────────────
        let action = evaluate_transport_result(operation, result, &retry_state);

        // ── STAGE 6: Apply location effects ────────────────────────────
        // Step 1: no location effects.

        // ── STAGE 7: Act on the control-flow decision ──────────────────
        match action {
            OperationAction::Complete(result) => {
                return build_cosmos_response(result, diagnostics);
            }
            OperationAction::TransportRetry { new_state } => {
                // Check deadline before retrying
                if let Some(d) = deadline {
                    if Instant::now() >= d {
                        let timeout_duration = effective_options
                            .end_to_end_latency_policy
                            .as_ref()
                            .map(|p| p.timeout())
                            .unwrap_or_default();

                        diagnostics.set_operation_status(
                            azure_core::http::StatusCode::RequestTimeout,
                            Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT),
                        );
                        return Err(azure_core::Error::new(
                            azure_core::error::ErrorKind::Other,
                            format!("end-to-end operation timeout exceeded ({timeout_duration:?})"),
                        ));
                    }
                }
                retry_state = new_state;
                // → next iteration
            }
            OperationAction::Abort { error, status } => {
                if let Some((status_code, sub_status)) = status {
                    diagnostics.set_operation_status(status_code, sub_status);
                }
                return Err(error);
            }
        }
    }
}

/// Resolves the endpoint for this attempt.
///
/// Step 1: trivially wraps the provided endpoint and region.
/// Step 2 will use `LocationSnapshot` and `AccountEndpointState`.
fn resolve_endpoint(endpoint: &AccountEndpoint, region: Option<Region>) -> RoutingDecision {
    RoutingDecision {
        endpoint: endpoint.url().clone(),
        region,
    }
}

/// Builds a `TransportRequest` from the operation and routing decision.
fn build_transport_request(
    operation: &CosmosOperation,
    routing: &RoutingDecision,
    activity_id: &ActivityId,
    execution_context: ExecutionContext,
    deadline: Option<Instant>,
) -> TransportRequest {
    let resource_ref = operation.resource_reference();
    let request_path = resource_ref.request_path();
    let url = join_url(&routing.endpoint, &request_path);

    let method = operation.operation_type().http_method();
    let resource_type = operation.resource_type();
    let resource_link = resource_ref.link_for_signing();
    let signing_link = resource_link.trim_start_matches('/');

    let auth_context = AuthorizationContext::new(method, resource_type, signing_link);

    // Build headers from the operation
    let mut headers = azure_core::http::headers::Headers::new();
    operation.request_headers().write_to_headers(&mut headers);

    // Add activity ID if not already set by the operation
    if operation.request_headers().activity_id.is_none() {
        headers.insert(
            HeaderName::from_static("x-ms-activity-id"),
            HeaderValue::from(activity_id.as_str().to_owned()),
        );
    }

    // Add partition key headers
    if let Some(pk) = operation.partition_key() {
        use azure_core::http::headers::AsHeaders;
        if let Ok(pk_headers) = pk.as_headers() {
            for (name, value) in pk_headers {
                headers.insert(name, value);
            }
        }
    }

    TransportRequest {
        method,
        url,
        headers,
        body: operation.body().map(|b| b.to_vec()),
        auth_context,
        execution_context,
        deadline,
    }
}

/// Builds a `CosmosResponse` from a successful `TransportResult`.
fn build_cosmos_response(
    result: TransportResult,
    mut diagnostics: DiagnosticsContextBuilder,
) -> azure_core::Result<CosmosResponse> {
    match result.outcome {
        TransportOutcome::Success {
            status,
            headers,
            body,
        } => {
            let cosmos_headers = CosmosResponseHeaders::from_headers(&headers);
            diagnostics.set_operation_status(status.status_code(), status.sub_status());

            let diagnostics_ctx = Arc::new(diagnostics.complete());

            Ok(CosmosResponse::new(
                body,
                cosmos_headers,
                status,
                diagnostics_ctx,
            ))
        }
        _ => {
            // This should only be called with a Complete(Success) result
            Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "build_cosmos_response called with non-success result",
            ))
        }
    }
}

/// Joins a base URL with a path, handling trailing/leading slashes.
fn join_url(base: &Url, path: &str) -> Url {
    let mut url = base.clone();
    {
        let mut segments = url.path_segments_mut().expect("URL cannot be a base");
        for segment in path.trim_start_matches('/').split('/') {
            if !segment.is_empty() {
                segments.push(segment);
            }
        }
    }
    url
}
