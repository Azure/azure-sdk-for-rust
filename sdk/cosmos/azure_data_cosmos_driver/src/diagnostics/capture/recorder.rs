// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The per-operation **capture recorder** — the entire hot-path cost of diagnostics.
//!
//! A [`DiagnosticsRecorder`] rents one [`EventLog`](super::event::EventLog) lease from a
//! [`LogPool`](super::pool::LogPool) at operation start and appends plain Rust structs — a
//! [`Span`](super::event::Span) per attempt / hedge leg plus its [`Attr`](super::event::Attr)s —
//! as the operation runs. Appends go through `&mut self`, so there is **no lock on the per-attempt
//! hot path**; the pool's brief lock is touched only at operation boundaries (rent / drop). No
//! bytes are encoded and no varints are written on the hot path — storing an event is a single
//! `Vec::push` of a small, typed value.
//!
//! At operation end the recorder is handed to the gate ([`super::gate`]). On a fast success the
//! recorder is simply dropped; on a slow or errored operation the recorded spans / attrs are
//! walked into the canonical
//! [`DiagnosticsContext`](crate::diagnostics::DiagnosticsContext) (see [`super::context`]) first.
//!
//! ## Cancellation & panic safety
//!
//! The recorder owns its [`EventLog`] lease, which is an RAII handle: dropping it returns the
//! backing storage to the pool. So whether the operation future completes, is cancelled
//! (`select!`, timeout), or panics, the storage is returned automatically — there is no explicit
//! "return the buffer" step, and a partially written storage is cleared before reuse.

use super::event::{AttrKey, EventLog, SpanId, SpanKind, TimeOffset};
use super::pool::LogPool;
use super::Outcome;
use crate::diagnostics::{
    ExecutionContext, PipelineType, TransportHttpVersion, TransportKind, TransportSecurity,
};
use crate::error::{CosmosStatus, SubStatusCode};
use azure_core::http::StatusCode;
use std::sync::Arc;
use std::time::Instant;

/// Maps an [`ExecutionContext`] to its stable attribute discriminant.
pub(crate) fn exec_context_to_u64(ctx: ExecutionContext) -> u64 {
    match ctx {
        ExecutionContext::Initial => 0,
        ExecutionContext::Retry => 1,
        ExecutionContext::TransportRetry => 2,
        ExecutionContext::Hedging => 3,
        ExecutionContext::RegionFailover => 4,
        ExecutionContext::CircuitBreakerProbe => 5,
    }
}

/// Maps a [`PipelineType`] to its stable attribute discriminant.
pub(crate) fn pipeline_type_to_u64(value: PipelineType) -> u64 {
    match value {
        PipelineType::Metadata => 0,
        PipelineType::DataPlane => 1,
    }
}

/// Maps a [`TransportSecurity`] to its stable attribute discriminant.
pub(crate) fn transport_security_to_u64(value: TransportSecurity) -> u64 {
    match value {
        TransportSecurity::Secure => 0,
        TransportSecurity::EmulatorWithInsecureCertificates => 1,
    }
}

/// Maps a [`TransportKind`] to its stable attribute discriminant.
pub(crate) fn transport_kind_to_u64(value: TransportKind) -> u64 {
    match value {
        TransportKind::Gateway => 0,
        TransportKind::Gateway20 => 1,
    }
}

/// Maps a [`TransportHttpVersion`] to its stable attribute discriminant.
pub(crate) fn http_version_to_u64(value: TransportHttpVersion) -> u64 {
    match value {
        TransportHttpVersion::Http11 => 0,
        TransportHttpVersion::Http2 => 1,
    }
}

/// The terminal outcome of a hedging race, captured for the built `HedgeDiagnostics`.
///
/// [`HedgeDiagnostics`]: crate::diagnostics::HedgeDiagnostics
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HedgeOutcome {
    /// The primary returned before the hedge threshold fired; no alternate spawned.
    PrimaryWonPreThreshold,
    /// The primary returned after the threshold, beating the spawned alternate.
    PrimaryWonAfterHedge,
    /// The alternate (hedge) leg won the race.
    AlternateWon,
    /// Both legs returned retriable failures; `deadline_elapsed` carries through.
    BothTransient {
        /// Whether the operation deadline had elapsed when the race concluded.
        deadline_elapsed: bool,
    },
}

impl HedgeOutcome {
    /// Stable discriminant stored as the [`AttrKey::HedgeOutcome`] value.
    pub(crate) fn to_u64(self) -> u64 {
        match self {
            HedgeOutcome::PrimaryWonPreThreshold => 0,
            HedgeOutcome::PrimaryWonAfterHedge => 1,
            HedgeOutcome::AlternateWon => 2,
            HedgeOutcome::BothTransient {
                deadline_elapsed: false,
            } => 3,
            HedgeOutcome::BothTransient {
                deadline_elapsed: true,
            } => 4,
        }
    }

    /// Inverse of [`HedgeOutcome::to_u64`], defaulting unknown values to `PrimaryWonPreThreshold`.
    pub(crate) fn from_u64(value: u64) -> Self {
        match value {
            1 => HedgeOutcome::PrimaryWonAfterHedge,
            2 => HedgeOutcome::AlternateWon,
            4 => HedgeOutcome::BothTransient {
                deadline_elapsed: true,
            },
            3 => HedgeOutcome::BothTransient {
                deadline_elapsed: false,
            },
            _ => HedgeOutcome::PrimaryWonPreThreshold,
        }
    }
}

/// Per-attempt data captured on the hot path. `start_ns`/`duration_ns` are relative to op start.
///
/// This is a plain value type so a concurrent fan-out / hedge task can capture its own attempt
/// and the operation layer merges it later — keeping per-task capture lock-free.
#[derive(Clone, Debug)]
pub struct AttemptRecord {
    /// Which execution context this attempt ran under (Initial / Retry / Hedging / ...).
    pub execution_context: ExecutionContext,
    /// The region this attempt targeted (empty when unknown / global endpoint).
    pub region: String,
    /// The endpoint URL this attempt targeted.
    pub endpoint: String,
    /// HTTP status code (`0` for a transport failure with no response).
    pub status: u16,
    /// Cosmos sub-status, when present.
    pub sub_status: Option<u16>,
    /// Service request id / activity id from the response.
    pub service_request_id: Option<String>,
    /// Request charge (RU).
    pub request_charge: Option<f64>,
    /// Retry-safety signal on a transport failure (`sent` / `not_sent` / `unknown`).
    pub request_sent: Option<String>,
    /// Pipeline type this attempt ran under, when known.
    pub pipeline_type: Option<PipelineType>,
    /// Transport security mode of this attempt, when known.
    pub transport_security: Option<TransportSecurity>,
    /// Transport kind of this attempt, when known.
    pub transport_kind: Option<TransportKind>,
    /// Negotiated HTTP version of this attempt, when known.
    pub http_version: Option<TransportHttpVersion>,
    /// Server-reported request duration (ms), distinct from the client-observed span, when known.
    pub server_duration_ms: Option<f64>,
    /// Start relative to the operation start (nanoseconds); filled from the clock when `0`.
    pub start_ns: u64,
    /// Attempt duration (nanoseconds).
    pub duration_ns: u64,
}

impl AttemptRecord {
    /// Creates an attempt record for the common success/response case.
    pub fn new(
        execution_context: ExecutionContext,
        region: impl Into<String>,
        endpoint: impl Into<String>,
        status: u16,
    ) -> Self {
        Self {
            execution_context,
            region: region.into(),
            endpoint: endpoint.into(),
            status,
            sub_status: None,
            service_request_id: None,
            request_charge: None,
            request_sent: None,
            pipeline_type: None,
            transport_security: None,
            transport_kind: None,
            http_version: None,
            server_duration_ms: None,
            start_ns: 0,
            duration_ns: 0,
        }
    }

    /// Sets the service request id / activity id (builder-style).
    pub fn with_service_request_id(mut self, id: impl Into<String>) -> Self {
        self.service_request_id = Some(id.into());
        self
    }

    /// Sets the request charge (builder-style).
    pub fn with_request_charge(mut self, ru: f64) -> Self {
        self.request_charge = Some(ru);
        self
    }

    /// Sets the Cosmos sub-status (builder-style).
    pub fn with_sub_status(mut self, sub_status: u16) -> Self {
        self.sub_status = Some(sub_status);
        self
    }

    /// Sets the retry-safety signal for a transport failure (builder-style).
    pub fn with_request_sent(mut self, request_sent: impl Into<String>) -> Self {
        self.request_sent = Some(request_sent.into());
        self
    }

    /// Sets the transport / pipeline facets of this attempt (builder-style).
    pub fn with_transport(
        mut self,
        pipeline_type: PipelineType,
        transport_security: TransportSecurity,
        transport_kind: TransportKind,
        http_version: TransportHttpVersion,
    ) -> Self {
        self.pipeline_type = Some(pipeline_type);
        self.transport_security = Some(transport_security);
        self.transport_kind = Some(transport_kind);
        self.http_version = Some(http_version);
        self
    }

    /// Sets the server-reported request duration in milliseconds (builder-style).
    pub fn with_server_duration_ms(mut self, server_duration_ms: f64) -> Self {
        self.server_duration_ms = Some(server_duration_ms);
        self
    }

    /// Sets the attempt duration in nanoseconds (builder-style).
    pub fn with_duration_ns(mut self, duration_ns: u64) -> Self {
        self.duration_ns = duration_ns;
        self
    }
}

/// A per-operation append-only capture recorder backed by a pooled [`EventLog`] lease.
#[derive(Debug)]
pub struct DiagnosticsRecorder {
    log: EventLog,
    /// Id of the operation root span.
    op_span: SpanId,
    start: Instant,
    outcome: Outcome,
    attempt_count: u32,
    total: TimeOffset,
}

impl DiagnosticsRecorder {
    /// Begins capture for an operation, renting a log lease and recording the operation root span.
    ///
    /// `operation` is the operation name and `activity_id` the operation's activity id (used as the
    /// top-level id of the built `DiagnosticsContext`). `_endpoint` is accepted for call-site parity
    /// with the pipeline's envelope; the per-attempt endpoint is what reconstruction uses.
    pub fn start(pool: &Arc<LogPool>, operation: &str, _endpoint: &str, activity_id: &str) -> Self {
        let mut log = pool.rent();
        let op_span = log.push_span(
            SpanKind::Operation,
            None,
            TimeOffset::ZERO,
            TimeOffset::ZERO,
        );
        log.attr_str(op_span, AttrKey::OperationName, operation);
        log.attr_str(op_span, AttrKey::ActivityId, activity_id);
        Self {
            log,
            op_span,
            start: Instant::now(),
            outcome: Outcome::Success,
            attempt_count: 0,
            total: TimeOffset::ZERO,
        }
    }

    /// Nanoseconds elapsed since the operation started (monotonic, from [`Instant`]).
    pub fn elapsed_ns(&self) -> u64 {
        self.elapsed().as_nanos()
    }

    /// Elapsed time since the operation started, as a [`TimeOffset`].
    fn elapsed(&self) -> TimeOffset {
        TimeOffset::from_nanos(self.start.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64)
    }

    /// Records one attempt (or hedge leg). `start_ns` is set from the recorder's clock if `0`.
    pub fn record_attempt(&mut self, mut attempt: AttemptRecord) {
        if attempt.start_ns == 0 {
            attempt.start_ns = self.elapsed_ns();
        }
        let op_span = self.op_span;
        let start = TimeOffset::from_nanos(attempt.start_ns);
        let end = TimeOffset::from_nanos(attempt.start_ns.saturating_add(attempt.duration_ns));
        let log = &mut self.log;
        let span = log.push_span(SpanKind::Attempt, Some(op_span), start, end);

        log.attr_u64(
            span,
            AttrKey::ExecutionContext,
            exec_context_to_u64(attempt.execution_context),
        );
        if !attempt.region.is_empty() {
            log.attr_str(span, AttrKey::Region, attempt.region.as_str());
        }
        if !attempt.endpoint.is_empty() {
            log.attr_str(span, AttrKey::Endpoint, attempt.endpoint.as_str());
        }
        // A real response is stored as a first-class status; a transport failure (status `0`,
        // no response) records no status attribute, which reconstruction reads as a failure.
        if attempt.status != 0 {
            let status = CosmosStatus::from_parts(
                StatusCode::from(attempt.status),
                attempt.sub_status.map(SubStatusCode::new),
            );
            log.attr_status(span, AttrKey::Status, status);
        }
        if let Some(id) = attempt
            .service_request_id
            .as_deref()
            .filter(|s| !s.is_empty())
        {
            log.attr_str(span, AttrKey::ServiceRequestId, id);
        }
        if let Some(ru) = attempt.request_charge.filter(|ru| *ru != 0.0) {
            log.attr_f64(span, AttrKey::RequestCharge, ru);
        }
        if let Some(sent) = attempt.request_sent.as_deref().filter(|s| !s.is_empty()) {
            log.attr_str(span, AttrKey::RequestSent, sent);
        }
        if let Some(pt) = attempt.pipeline_type {
            log.attr_u64(span, AttrKey::PipelineType, pipeline_type_to_u64(pt));
        }
        if let Some(ts) = attempt.transport_security {
            log.attr_u64(
                span,
                AttrKey::TransportSecurity,
                transport_security_to_u64(ts),
            );
        }
        if let Some(tk) = attempt.transport_kind {
            log.attr_u64(span, AttrKey::TransportKind, transport_kind_to_u64(tk));
        }
        if let Some(hv) = attempt.http_version {
            log.attr_u64(span, AttrKey::TransportHttpVersion, http_version_to_u64(hv));
        }
        if let Some(sd) = attempt.server_duration_ms {
            log.attr_f64(span, AttrKey::ServerDurationMs, sd);
        }

        self.attempt_count += 1;
    }

    /// Records the hedge race outcome (regions + terminal state) for the built `HedgeDiagnostics`.
    pub fn record_hedge_outcome(
        &mut self,
        outcome: HedgeOutcome,
        threshold: std::time::Duration,
        primary_region: &str,
        alternate_region: Option<&str>,
        response_region: Option<&str>,
    ) {
        let op_span = self.op_span;
        let at = self.elapsed();
        let log = &mut self.log;
        let span = log.push_span(SpanKind::Hedge, Some(op_span), at, at);
        log.attr_u64(span, AttrKey::HedgeOutcome, outcome.to_u64());
        log.attr_u64(
            span,
            AttrKey::HedgeThresholdNs,
            threshold.as_nanos().min(u128::from(u64::MAX)) as u64,
        );
        if !primary_region.is_empty() {
            log.attr_str(span, AttrKey::PrimaryRegion, primary_region);
        }
        if let Some(alt) = alternate_region.filter(|r| !r.is_empty()) {
            log.attr_str(span, AttrKey::AlternateRegion, alt);
        }
        if let Some(resp) = response_region.filter(|r| !r.is_empty()) {
            log.attr_str(span, AttrKey::ResponseRegion, resp);
        }
    }

    /// Records the operation outcome and finalizes the operation root span.
    ///
    /// `final_status` / `final_sub_status` set the top-level status of the built context. If
    /// `total_ns` is `None`, the recorder's own monotonic elapsed time is used.
    pub fn record_end(
        &mut self,
        outcome: Outcome,
        attempt_count: u32,
        final_status: u16,
        final_sub_status: Option<u16>,
        total_ns: Option<u64>,
    ) {
        let total = total_ns
            .map(TimeOffset::from_nanos)
            .unwrap_or_else(|| self.elapsed());
        self.outcome = outcome;
        self.attempt_count = attempt_count;
        self.total = total;
        let op_span = self.op_span;
        let log = &mut self.log;
        log.set_span_end(op_span, total);
        log.attr_u64(
            op_span,
            AttrKey::Outcome,
            match outcome {
                Outcome::Success => 0,
                Outcome::Error => 1,
            },
        );
        log.attr_u64(op_span, AttrKey::AttemptCount, u64::from(attempt_count));
        if final_status != 0 {
            let status = CosmosStatus::from_parts(
                StatusCode::from(final_status),
                final_sub_status.map(SubStatusCode::new),
            );
            log.attr_status(op_span, AttrKey::FinalStatus, status);
        }
    }

    /// The recorded operation outcome.
    pub fn outcome(&self) -> Outcome {
        self.outcome
    }

    /// The recorded total elapsed time (nanoseconds).
    pub fn total_ns(&self) -> u64 {
        self.total.as_nanos()
    }

    /// The number of log entries (spans + attributes) appended on the hot path.
    pub fn raw_len(&self) -> usize {
        self.log.spans().len() + self.log.attrs().len()
    }

    /// Borrows the captured event storage (for reconstruction past the gate).
    pub(crate) fn log(&self) -> &super::event::EventLogStorage {
        self.log.storage()
    }
}
