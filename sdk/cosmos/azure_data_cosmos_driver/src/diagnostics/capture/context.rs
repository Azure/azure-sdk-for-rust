// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Materializes a captured operation log into the canonical
//! [`DiagnosticsContext`](crate::diagnostics::DiagnosticsContext).
//!
//! This runs **only past the gate** (a slow or errored operation, or `Mode::Always`). It replays
//! the captured attempts and hedge legs onto a [`DiagnosticsContextBuilder`], so the gated capture
//! front-end produces the **same diagnostics type** the rest of the driver returns — there is one
//! canonical diagnostics model, not a parallel one.
//!
//! Each captured attempt becomes a [`RequestDiagnostics`](crate::diagnostics::RequestDiagnostics)
//! with its [`ExecutionContext`](crate::diagnostics::ExecutionContext) (Initial / Retry / Hedging /
//! RegionFailover / …), region, endpoint, status, sub-status, request charge, and request-sent
//! signal. A hedged operation additionally attaches a
//! [`HedgeDiagnostics`](crate::diagnostics::HedgeDiagnostics) describing the region legs and the
//! terminal state.

use super::recorder::{HedgeOutcome, Parsed, ParsedHedge};
use super::Outcome;
use crate::diagnostics::{
    DiagnosticsContext, DiagnosticsContextBuilder, HedgeDiagnostics, HedgingStrategyConfig,
    PipelineType, RequestSentStatus, TransportHttpVersion, TransportKind, TransportSecurity,
};
use crate::driver::routing::CosmosEndpoint;
use crate::error::{CosmosStatus, SubStatusCode};
use crate::models::{ActivityId, RequestCharge};
use crate::options::{DiagnosticsOptions, HedgeThreshold, Region};
use azure_core::http::StatusCode;
use std::sync::Arc;

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

/// Builds the `HedgeDiagnostics` for a parsed hedge outcome.
fn build_hedge(hedge: &ParsedHedge) -> HedgeDiagnostics {
    let threshold = HedgeThreshold::new(std::time::Duration::from_nanos(hedge.threshold_ns))
        .unwrap_or_else(|| {
            HedgeThreshold::new(std::time::Duration::from_millis(1)).expect("1ms is valid")
        });
    let config = HedgingStrategyConfig::new(threshold);
    let primary = Region::new(hedge.primary_region.clone());
    let alternate = hedge
        .alternate_region
        .as_ref()
        .map(|r| Region::new(r.clone()));

    match (hedge.outcome, alternate) {
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

/// Materializes a parsed capture log into a [`DiagnosticsContext`].
pub(crate) fn build_context(
    parsed: &Parsed,
    options: Arc<DiagnosticsOptions>,
) -> DiagnosticsContext {
    let activity_id = ActivityId::from_string(parsed.activity_id.clone());
    let mut builder = DiagnosticsContextBuilder::new(activity_id, options);

    for attempt in &parsed.attempts {
        let endpoint = endpoint_for(&attempt.region, &attempt.endpoint);
        let handle = builder.start_request(
            attempt.execution_context,
            PipelineType::DataPlane,
            TransportSecurity::Secure,
            TransportKind::Gateway,
            TransportHttpVersion::Http2,
            &endpoint,
        );

        let sub_status = attempt.sub_status.map(SubStatusCode::new);
        let service_request_id = attempt.service_request_id.clone();
        let request_charge = attempt.request_charge;
        // The captured client-observed attempt duration is surfaced as the server-duration field;
        // the builder's own wall-clock timing measures the (synchronous) replay, not the original
        // request, so the captured duration is the authoritative per-attempt latency signal.
        let duration_ms = attempt.duration_ns as f64 / 1_000_000.0;
        builder.update_request(handle, |req| {
            if !service_request_id.is_empty() {
                req.with_activity_id(ActivityId::from_string(service_request_id.clone()));
            }
            if request_charge != 0.0 {
                req.with_charge(RequestCharge::new(f64::from(request_charge)));
            }
            if let Some(sub) = sub_status {
                req.with_sub_status(sub);
            }
            if duration_ms > 0.0 {
                req.with_server_duration_ms(duration_ms);
            }
        });

        if attempt.status == 0 {
            builder.fail_transport_request(
                handle,
                "transport failure (no response)",
                request_sent_from_str(&attempt.request_sent),
                transport_failure_status(),
            );
        } else {
            builder.complete_request(handle, StatusCode::from(attempt.status), sub_status);
        }
    }

    if let Some(hedge) = &parsed.hedge {
        builder.set_hedge_diagnostics(build_hedge(hedge));
    }

    let final_status = if parsed.final_status == 0 {
        // No explicit final status recorded (e.g. a cancelled op): fall back to the outcome.
        match parsed.outcome {
            Outcome::Success => StatusCode::from(200),
            Outcome::Error => StatusCode::from(503),
        }
    } else {
        StatusCode::from(parsed.final_status)
    };
    builder.set_operation_status(
        final_status,
        parsed.final_sub_status.map(SubStatusCode::new),
    );

    builder.complete()
}
