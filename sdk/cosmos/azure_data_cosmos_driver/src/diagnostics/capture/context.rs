// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Reconstructs a captured [`EventLog`] into the canonical
//! [`DiagnosticsContext`](crate::diagnostics::DiagnosticsContext).
//!
//! This runs **only past the gate** (a slow or errored operation, or `Mode::Always`). It walks the
//! flat span / attr lists back into a tree and replays it onto a [`DiagnosticsContextBuilder`], so
//! the gated capture front-end produces the **same diagnostics type** the rest of the driver
//! returns — there is one canonical diagnostics model, not a parallel one. There is no byte parse
//! step: the typed event log *is* the parsed form.
//!
//! Each captured [`SpanKind::Attempt`] becomes a
//! [`RequestDiagnostics`](crate::diagnostics::RequestDiagnostics) with its
//! [`ExecutionContext`](crate::diagnostics::ExecutionContext), region, endpoint, status,
//! sub-status, request charge, and request-sent signal. A [`SpanKind::Hedge`] span additionally
//! attaches a [`HedgeDiagnostics`](crate::diagnostics::HedgeDiagnostics) describing the region legs
//! and the terminal state.

use super::event::{AttrKey, EventLog, Span, SpanKind};
use super::recorder::HedgeOutcome;
use super::Outcome;
use crate::diagnostics::{
    DiagnosticsContext, DiagnosticsContextBuilder, ExecutionContext, HedgeDiagnostics,
    HedgingStrategyConfig, PipelineType, RequestSentStatus, TransportHttpVersion, TransportKind,
    TransportSecurity,
};
use crate::driver::routing::CosmosEndpoint;
use crate::error::{CosmosStatus, SubStatusCode};
use crate::models::{ActivityId, RequestCharge};
use crate::options::{DiagnosticsOptions, HedgeThreshold, Region};
use azure_core::http::StatusCode;
use std::sync::Arc;

/// Inverse of [`exec_context_to_u64`](super::recorder::exec_context_to_u64).
fn exec_context_from_u64(value: u64) -> ExecutionContext {
    match value {
        1 => ExecutionContext::Retry,
        2 => ExecutionContext::TransportRetry,
        3 => ExecutionContext::Hedging,
        4 => ExecutionContext::RegionFailover,
        5 => ExecutionContext::CircuitBreakerProbe,
        _ => ExecutionContext::Initial,
    }
}

/// A synthetic Cosmos status used for an attempt that failed at the transport layer (no response).
fn transport_failure_status() -> CosmosStatus {
    // 503 Service Unavailable is the closest standard mapping for "no response from the service".
    CosmosStatus::from_parts(StatusCode::from(503), None)
}

/// Builds a [`CosmosEndpoint`] from a captured region + endpoint URL string.
fn endpoint_for(region: &str, endpoint: &str) -> CosmosEndpoint {
    let url = url::Url::parse(endpoint)
        .or_else(|_| url::Url::parse("https://unknown.documents.azure.com/"))
        .expect("fallback URL is valid");
    if region.is_empty() {
        CosmosEndpoint::global(url)
    } else {
        CosmosEndpoint::regional(Region::new(region.to_string()), url)
    }
}

fn request_sent_from_str(s: &str) -> RequestSentStatus {
    match s {
        "sent" => RequestSentStatus::Sent,
        "not_sent" => RequestSentStatus::NotSent,
        _ => RequestSentStatus::Unknown,
    }
}

/// Replays a single captured attempt span onto the builder as a `RequestDiagnostics`.
fn replay_attempt(builder: &mut DiagnosticsContextBuilder, log: &EventLog, id: u32, span: &Span) {
    let execution_context =
        exec_context_from_u64(log.attr_u64_of(id, AttrKey::ExecutionContext).unwrap_or(0));
    let region = log.attr_str_of(id, AttrKey::Region).unwrap_or("");
    let endpoint_url = log.attr_str_of(id, AttrKey::Endpoint).unwrap_or("");
    let status = log.attr_u64_of(id, AttrKey::Status).unwrap_or(0);
    let sub_status = log
        .attr_u64_of(id, AttrKey::SubStatus)
        .map(|s| SubStatusCode::new(s.min(u64::from(u16::MAX)) as u16));
    let service_request_id = log.attr_str_of(id, AttrKey::ServiceRequestId);
    let request_charge = log.attr_f64_of(id, AttrKey::RequestCharge);
    let request_sent = log.attr_str_of(id, AttrKey::RequestSent).unwrap_or("");
    // The captured client-observed attempt duration is surfaced as the server-duration field; the
    // builder's own wall-clock timing measures the (synchronous) replay, not the original request,
    // so the captured duration is the authoritative per-attempt latency signal.
    let duration_ms = span.end_ns.saturating_sub(span.start_ns) as f64 / 1_000_000.0;

    let endpoint = endpoint_for(region, endpoint_url);
    let handle = builder.start_request(
        execution_context,
        PipelineType::DataPlane,
        TransportSecurity::Secure,
        TransportKind::Gateway,
        TransportHttpVersion::Http2,
        &endpoint,
    );

    builder.update_request(handle, |req| {
        if let Some(id) = service_request_id {
            req.with_activity_id(ActivityId::from_string(id.to_string()));
        }
        if let Some(ru) = request_charge {
            req.with_charge(RequestCharge::new(ru));
        }
        if let Some(sub) = sub_status {
            req.with_sub_status(sub);
        }
        if duration_ms > 0.0 {
            req.with_server_duration_ms(duration_ms);
        }
    });

    if status == 0 {
        builder.fail_transport_request(
            handle,
            "transport failure (no response)",
            request_sent_from_str(request_sent),
            transport_failure_status(),
        );
    } else {
        builder.complete_request(
            handle,
            StatusCode::from(status.min(u64::from(u16::MAX)) as u16),
            sub_status,
        );
    }
}

/// Builds the `HedgeDiagnostics` for a captured hedge span.
fn build_hedge(log: &EventLog, id: u32) -> HedgeDiagnostics {
    let outcome = HedgeOutcome::from_u64(log.attr_u64_of(id, AttrKey::HedgeOutcome).unwrap_or(0));
    let threshold_ns = log.attr_u64_of(id, AttrKey::HedgeThresholdNs).unwrap_or(0);
    let threshold = HedgeThreshold::new(std::time::Duration::from_nanos(threshold_ns))
        .unwrap_or_else(|| {
            HedgeThreshold::new(std::time::Duration::from_millis(1)).expect("1ms is valid")
        });
    let config = HedgingStrategyConfig::new(threshold);
    let primary = Region::new(
        log.attr_str_of(id, AttrKey::PrimaryRegion)
            .unwrap_or("")
            .to_string(),
    );
    let alternate = log
        .attr_str_of(id, AttrKey::AlternateRegion)
        .map(|r| Region::new(r.to_string()));

    match (outcome, alternate) {
        (HedgeOutcome::PrimaryWonPreThreshold, _) | (_, None) => {
            HedgeDiagnostics::primary_only(config, primary)
        }
        (HedgeOutcome::PrimaryWonAfterHedge, Some(alt)) => {
            HedgeDiagnostics::primary_won_after_hedge(config, primary, alt)
        }
        (HedgeOutcome::AlternateWon, Some(alt)) => {
            HedgeDiagnostics::hedge_won(config, primary, alt)
        }
        (HedgeOutcome::BothTransient { deadline_elapsed }, Some(alt)) => {
            HedgeDiagnostics::both_transient(config, primary, alt, deadline_elapsed)
        }
    }
}

/// Reconstructs a captured [`EventLog`] into a [`DiagnosticsContext`].
pub(crate) fn build_context(
    log: &EventLog,
    options: Arc<DiagnosticsOptions>,
) -> DiagnosticsContext {
    // The operation root is span 0 (see `DiagnosticsRecorder::start`).
    let op_span = 0u32;
    let activity_id = ActivityId::from_string(
        log.attr_str_of(op_span, AttrKey::ActivityId)
            .unwrap_or("")
            .to_string(),
    );
    let mut builder = DiagnosticsContextBuilder::new(activity_id, options);

    for (id, span) in log.children(op_span) {
        match span.kind {
            SpanKind::Attempt => replay_attempt(&mut builder, log, id, span),
            SpanKind::Hedge => builder.set_hedge_diagnostics(build_hedge(log, id)),
            SpanKind::Operation => {}
        }
    }

    let recorded_status = log.attr_u64_of(op_span, AttrKey::FinalStatus).unwrap_or(0);
    let final_status = if recorded_status == 0 {
        // No explicit final status recorded (e.g. a cancelled op): fall back to the outcome.
        let outcome = match log.attr_u64_of(op_span, AttrKey::Outcome) {
            Some(1) => Outcome::Error,
            _ => Outcome::Success,
        };
        match outcome {
            Outcome::Success => StatusCode::from(200),
            Outcome::Error => StatusCode::from(503),
        }
    } else {
        StatusCode::from(recorded_status.min(u64::from(u16::MAX)) as u16)
    };
    let final_sub_status = log
        .attr_u64_of(op_span, AttrKey::FinalSubStatus)
        .map(|s| SubStatusCode::new(s.min(u64::from(u16::MAX)) as u16));
    builder.set_operation_status(final_status, final_sub_status);

    builder.complete()
}
