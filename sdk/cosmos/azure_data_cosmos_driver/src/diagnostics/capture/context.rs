// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Reconstructs a captured [`EventLog`] into a [`DiagnosticsContext`](crate::diagnostics::DiagnosticsContext).
//!
//! **Prototype, behind the `capture_engine` feature.** This is the deferred capture engine's
//! reconstruction step and is **not** used by the driver's default diagnostics path (which surfaces
//! the `DiagnosticsContextBuilder`-produced context directly). It runs only past the gate (a slow or
//! errored operation, or `Mode::Always`), walking the flat span / attr lists back into a tree and
//! replaying them onto a [`DiagnosticsContextBuilder`].
//!
//! Pipeline/transport facets (pipeline type, transport security/kind, HTTP version) and the
//! server-reported duration are reconstructed from what the producer captured for each attempt;
//! each facet falls back to its common-case default only when it was not recorded. The per-attempt
//! client-observed span timing is retained separately from the server-reported duration.
//!
//! Each captured [`SpanKind::Attempt`] becomes a
//! [`RequestDiagnostics`](crate::diagnostics::RequestDiagnostics) with its
//! [`ExecutionContext`](crate::diagnostics::ExecutionContext), region, endpoint, status,
//! sub-status, request charge, and request-sent signal. A [`SpanKind::Hedge`] span additionally
//! attaches a [`HedgeDiagnostics`](crate::diagnostics::HedgeDiagnostics) describing the region legs
//! and the terminal state.

use super::event::{AttrKey, EventLogStorage, Span, SpanId, SpanKind};
use super::recorder::HedgeOutcome;
use crate::diagnostics::{
    DiagnosticsContext, DiagnosticsContextBuilder, ExecutionContext, HedgeDiagnostics,
    HedgingStrategyConfig, PipelineType, RequestSentStatus, TransportHttpVersion, TransportKind,
    TransportSecurity,
};
use crate::driver::routing::CosmosEndpoint;
use crate::error::CosmosStatus;
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

/// Reconstructs the pipeline type from its captured discriminant, defaulting to data-plane.
fn pipeline_type_from_u64(value: Option<u64>) -> PipelineType {
    match value {
        Some(0) => PipelineType::Metadata,
        _ => PipelineType::DataPlane,
    }
}

/// Reconstructs the transport security from its captured discriminant, defaulting to secure.
fn transport_security_from_u64(value: Option<u64>) -> TransportSecurity {
    match value {
        Some(1) => TransportSecurity::EmulatorWithInsecureCertificates,
        _ => TransportSecurity::Secure,
    }
}

/// Reconstructs the transport kind from its captured discriminant, defaulting to gateway.
fn transport_kind_from_u64(value: Option<u64>) -> TransportKind {
    match value {
        Some(1) => TransportKind::Gateway20,
        _ => TransportKind::Gateway,
    }
}

/// Reconstructs the HTTP version from its captured discriminant, defaulting to HTTP/2.
fn http_version_from_u64(value: Option<u64>) -> TransportHttpVersion {
    match value {
        Some(0) => TransportHttpVersion::Http11,
        _ => TransportHttpVersion::Http2,
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
fn replay_attempt(
    builder: &mut DiagnosticsContextBuilder,
    log: &EventLogStorage,
    id: SpanId,
    span: &Span,
) {
    let execution_context =
        exec_context_from_u64(log.attr_u64_of(id, AttrKey::ExecutionContext).unwrap_or(0));
    let region = log.attr_str_of(id, AttrKey::Region).unwrap_or("");
    let endpoint_url = log.attr_str_of(id, AttrKey::Endpoint).unwrap_or("");
    // A present status is a real response; its absence marks a transport failure (no response).
    let status = log.attr_status_of(id, AttrKey::Status);
    let service_request_id = log.attr_str_of(id, AttrKey::ServiceRequestId);
    let request_charge = log.attr_f64_of(id, AttrKey::RequestCharge);
    let request_sent = log.attr_str_of(id, AttrKey::RequestSent).unwrap_or("");
    // Transport / pipeline facets are reconstructed from what was captured for this attempt (each
    // falls back to its common-case default when the producer did not record it).
    let pipeline_type = pipeline_type_from_u64(log.attr_u64_of(id, AttrKey::PipelineType));
    let transport_security =
        transport_security_from_u64(log.attr_u64_of(id, AttrKey::TransportSecurity));
    let transport_kind = transport_kind_from_u64(log.attr_u64_of(id, AttrKey::TransportKind));
    let http_version = http_version_from_u64(log.attr_u64_of(id, AttrKey::TransportHttpVersion));
    // Prefer the captured server-reported duration; fall back to the client-observed span only when
    // no server duration was recorded.
    let server_duration_ms = log
        .attr_f64_of(id, AttrKey::ServerDurationMs)
        .unwrap_or_else(|| span.end.saturating_sub(span.start).as_nanos() as f64 / 1_000_000.0);

    let endpoint = endpoint_for(region, endpoint_url);
    let handle = builder.start_request(
        execution_context,
        pipeline_type,
        transport_security,
        transport_kind,
        http_version,
        &endpoint,
    );

    builder.update_request(handle, |req| {
        if let Some(id) = service_request_id {
            req.with_activity_id(ActivityId::from_string(id.to_string()));
        }
        if let Some(ru) = request_charge {
            req.with_charge(RequestCharge::new(ru));
        }
        if let Some(sub) = status.and_then(|s| s.sub_status()) {
            req.with_sub_status(sub);
        }
        if server_duration_ms > 0.0 {
            req.with_server_duration_ms(server_duration_ms);
        }
    });

    match status {
        Some(status) => {
            builder.complete_request(handle, status.status_code(), status.sub_status());
        }
        None => builder.fail_transport_request(
            handle,
            "transport failure (no response)",
            request_sent_from_str(request_sent),
            transport_failure_status(),
        ),
    }
}

/// Builds the `HedgeDiagnostics` for a captured hedge span.
fn build_hedge(log: &EventLogStorage, id: SpanId) -> HedgeDiagnostics {
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

/// Reconstructs a captured [`EventLogStorage`] into a [`DiagnosticsContext`].
pub(crate) fn build_context(
    log: &EventLogStorage,
    options: Arc<DiagnosticsOptions>,
) -> DiagnosticsContext {
    // The operation root is the first span (see `DiagnosticsRecorder::start`).
    let op_span = SpanId::from_index(0);
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

    let final_status = match log.attr_status_of(op_span, AttrKey::FinalStatus) {
        Some(status) => status,
        None => {
            // No explicit final status recorded (e.g. a cancelled op): fall back to the outcome.
            let code = match log.attr_u64_of(op_span, AttrKey::Outcome) {
                Some(1) => StatusCode::from(503),
                _ => StatusCode::from(200),
            };
            CosmosStatus::from_parts(code, None)
        }
    };
    builder.set_operation_status(final_status.status_code(), final_status.sub_status());

    builder.complete()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::capture::{
        finish, AttemptRecord, DiagnosticsPolicy, DiagnosticsRecorder, LogPool, Outcome,
    };
    use crate::diagnostics::DiagnosticsContextBuilder;
    use std::sync::Arc;

    fn options() -> Arc<DiagnosticsOptions> {
        Arc::new(DiagnosticsOptions::default())
    }

    /// Builds a context through the capture path (record -> gate -> reconstruct) for an attempt that
    /// carries non-default transport facets and a server-reported duration.
    fn captured_context() -> DiagnosticsContext {
        let pool = Arc::new(LogPool::default());
        let mut rec = DiagnosticsRecorder::start(&pool, "read_item", "https://acct/", "act-parity");
        rec.record_attempt(
            AttemptRecord::new(ExecutionContext::Initial, "West US", "https://west/", 200)
                .with_service_request_id("svc-200")
                .with_request_charge(3.5)
                .with_transport(
                    PipelineType::Metadata,
                    TransportSecurity::EmulatorWithInsecureCertificates,
                    TransportKind::Gateway20,
                    TransportHttpVersion::Http11,
                )
                .with_server_duration_ms(12.0)
                .with_duration_ns(20_000_000),
        );
        rec.record_end(Outcome::Success, 1, 200, None, Some(20_000_000));
        finish(rec, &DiagnosticsPolicy::always(), options()).expect("context built")
    }

    /// The same logical attempt fed straight to the builder — the parity reference.
    fn reference_context() -> DiagnosticsContext {
        let mut builder =
            DiagnosticsContextBuilder::new(ActivityId::from_string("act-parity".into()), options());
        let endpoint = endpoint_for("West US", "https://west/");
        let handle = builder.start_request(
            ExecutionContext::Initial,
            PipelineType::Metadata,
            TransportSecurity::EmulatorWithInsecureCertificates,
            TransportKind::Gateway20,
            TransportHttpVersion::Http11,
            &endpoint,
        );
        builder.update_request(handle, |req| {
            req.with_activity_id(ActivityId::from_string("svc-200".into()));
            req.with_charge(RequestCharge::new(3.5));
            req.with_server_duration_ms(12.0);
        });
        builder.complete_request(handle, StatusCode::from(200), None);
        builder.set_operation_status(StatusCode::from(200), None);
        builder.complete()
    }

    #[test]
    fn reconstruction_carries_real_facets_not_hardcoded_defaults() {
        let ctx = captured_context();
        let req = &ctx.requests()[0];
        // These would all be the old hardcoded DataPlane/Secure/Gateway/Http2 values if the
        // reconstruction were still fabricating them.
        assert_eq!(req.pipeline_type(), PipelineType::Metadata);
        assert_eq!(
            req.transport_security(),
            TransportSecurity::EmulatorWithInsecureCertificates
        );
        assert_eq!(req.transport_kind(), TransportKind::Gateway20);
        assert_eq!(req.transport_http_version(), TransportHttpVersion::Http11);
        // Server-reported duration is the captured 12ms, not the 20ms client span.
        assert_eq!(req.server_duration_ms(), Some(12.0));
    }

    #[test]
    fn capture_reconstruction_matches_builder_reference() {
        let captured = captured_context();
        let reference = reference_context();
        let cap = &captured.requests()[0];
        let refr = &reference.requests()[0];
        assert_eq!(cap.execution_context(), refr.execution_context());
        assert_eq!(cap.pipeline_type(), refr.pipeline_type());
        assert_eq!(cap.transport_security(), refr.transport_security());
        assert_eq!(cap.transport_kind(), refr.transport_kind());
        assert_eq!(cap.transport_http_version(), refr.transport_http_version());
        assert_eq!(cap.region(), refr.region());
        assert_eq!(cap.status().status_code(), refr.status().status_code());
        assert_eq!(cap.request_charge(), refr.request_charge());
        assert_eq!(cap.server_duration_ms(), refr.server_duration_ms());
        assert_eq!(captured.request_count(), reference.request_count());
    }
}
