// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Applies transport observations to the operation's diagnostic builder.

use super::{
    dispatch::DispatchError,
    recovery::{self, TransportRetryProgress},
    request::PreparationError,
    sharded_transport::TransportDispatch,
    types::{TransportOutcome, TransportRequest, TransportResult},
    TransportPipelineConfig, TransportPipelineServices,
};
use crate::{
    diagnostics::{
        DiagnosticsContextBuilder, RequestEvent, RequestEventType, RequestHandle, RequestSentStatus,
    },
    driver,
};

/// Creates an attempt record with routing and prior shard-failure history, returning its handle.
pub(super) fn start_attempt(
    request: &TransportRequest,
    config: &TransportPipelineConfig<'_>,
    services: &TransportPipelineServices<'_>,
    progress: &TransportRetryProgress,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> RequestHandle {
    let handle = diagnostics.start_request(
        recovery::execution_context(request.execution_context, progress),
        config.pipeline_type,
        config.transport_security,
        services.transport.diagnostics_kind(),
        services.transport.diagnostics_http_version(),
        &request.endpoint,
    );
    if let Some(reason) = request.routing_fallback {
        diagnostics.add_event(
            handle,
            RequestEvent::new(RequestEventType::RoutingFallback).with_details(reason.as_str()),
        );
    }
    for failed in progress.failed_shards.iter().cloned() {
        diagnostics.add_failed_transport_shard(handle, failed);
    }
    for _ in 0..progress.local_connectivity_retry_count {
        diagnostics.increment_local_shard_retry_count(handle);
    }
    handle
}

/// Adds the transport-start event after preparation succeeds and before dispatch.
// Inline the thin event adapter.
#[inline(always)]
pub(super) fn start_transport(handle: RequestHandle, diagnostics: &mut DiagnosticsContextBuilder) {
    diagnostics.add_event(handle, RequestEvent::new(RequestEventType::TransportStart));
}

/// Records a completed dispatch's shard and, for responses, its headers-received event before decoding.
pub(super) fn record_dispatch(
    dispatch: &TransportDispatch,
    handle: RequestHandle,
    diagnostics: &mut DiagnosticsContextBuilder,
) {
    if dispatch.result.is_ok() {
        diagnostics.add_event(
            handle,
            RequestEvent::new(RequestEventType::ResponseHeadersReceived),
        );
    }
    if let Some(shard) = dispatch.shard_diagnostics.clone() {
        diagnostics.set_transport_shard(handle, shard);
    }
}

/// Records a preparation failure as unsent, without adding transport events.
pub(super) fn fail_preparation(
    error: &PreparationError,
    handle: RequestHandle,
    diagnostics: &mut DiagnosticsContextBuilder,
) {
    let details = match error {
        PreparationError::Authorization(error) => error.to_string(),
        PreparationError::Encoding(error) => driver::error_chain_summary(error),
    };
    diagnostics.fail_transport_request(handle, details, RequestSentStatus::NotSent, error.status());
}

/// Writes response headers or failure details into the attempt record and marks it terminal.
pub(super) fn complete_attempt(
    outcome: &Result<TransportResult, DispatchError>,
    handle: RequestHandle,
    diagnostics: &mut DiagnosticsContextBuilder,
) {
    let (result, failed_after_headers) = match outcome {
        Ok(result) => (result, false),
        Err(error) => (error.result(), error.failed_after_headers()),
    };
    match &result.outcome {
        TransportOutcome::Success {
            status,
            cosmos_headers,
            ..
        }
        | TransportOutcome::HttpError {
            status,
            cosmos_headers,
            ..
        } => {
            diagnostics.record_response(handle, status.status_code(), cosmos_headers);
        }
        TransportOutcome::TransportError {
            status,
            error,
            request_sent,
        } => {
            let details = driver::error_chain_summary(error);
            if failed_after_headers {
                diagnostics.add_event(
                    handle,
                    RequestEvent::new(RequestEventType::ResponseHeadersReceived),
                );
            }
            diagnostics.add_event(
                handle,
                RequestEvent::new(RequestEventType::TransportFailed).with_details(details.clone()),
            );
            diagnostics.fail_transport_request(handle, details, *request_sent, *status);
        }
        TransportOutcome::DeadlineExceeded { .. } => {
            diagnostics.add_event(
                handle,
                RequestEvent::new(RequestEventType::TransportFailed)
                    .with_details("end-to-end operation timeout exceeded"),
            );
            diagnostics.timeout_request(handle);
        }
    }
}
