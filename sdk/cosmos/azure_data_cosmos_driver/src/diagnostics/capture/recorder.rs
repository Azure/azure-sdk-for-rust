// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The per-operation **capture recorder** — the entire hot-path cost of diagnostics.
//!
//! A [`DiagnosticsRecorder`] rents one [`EventLog`](super::event::EventLog) from a
//! [`LogPool`](super::pool::LogPool) at operation start and appends plain Rust structs — a
//! [`Span`](super::event::Span) per attempt / hedge leg plus its [`Attr`](super::event::Attr)s —
//! as the operation runs. Appends go through `&mut self`, so there is **no lock on the per-attempt
//! hot path**; the pool's brief lock is touched only at operation boundaries (rent / return). No
//! bytes are encoded and no varints are written on the hot path — storing an event is a single
//! `Vec::push` of a small, typed value.
//!
//! At operation end the recorder is handed to the gate ([`super::gate`]). On a fast success the
//! log is returned to the pool for ~free; on a slow or errored operation the recorded spans /
//! attrs are walked into the canonical
//! [`DiagnosticsContext`](crate::diagnostics::DiagnosticsContext) (see [`super::context`]).
//!
//! ## Cancellation & panic safety
//!
//! The recorder owns its log in an [`Option`]. [`DiagnosticsRecorder::return_buffer`] takes the
//! log out so the gate can return it to the pool. If the operation future is dropped before the
//! gate runs (cancellation, timeout, `select!`) or a panic unwinds through it, the [`Drop`] impl
//! returns the still-owned log to the pool — so a cancelled or panicking operation never leaks a
//! pooled log, and a partially written log is `clear()`-ed before reuse.

use super::event::{AttrKey, EventLog, SpanKind, NO_PARENT};
use super::pool::LogPool;
use super::Outcome;
use crate::diagnostics::ExecutionContext;
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

    /// Sets the attempt duration in nanoseconds (builder-style).
    pub fn with_duration_ns(mut self, duration_ns: u64) -> Self {
        self.duration_ns = duration_ns;
        self
    }
}

/// A per-operation append-only capture recorder backed by a typed [`EventLog`].
#[derive(Debug)]
pub struct DiagnosticsRecorder {
    pool: LogPool,
    log: Option<EventLog>,
    /// Id of the operation root span (always `0` while the log is owned).
    op_span: u32,
    start: Instant,
    outcome: Outcome,
    attempt_count: u32,
    total_ns: u64,
}

impl DiagnosticsRecorder {
    /// Begins capture for an operation, renting a log and recording the operation root span.
    ///
    /// `operation` is the operation name and `activity_id` the operation's activity id (used as the
    /// top-level id of the built `DiagnosticsContext`). `endpoint` is accepted for call-site parity
    /// with the previous byte recorder; the per-attempt endpoint is what reconstruction uses.
    pub fn start(pool: &LogPool, operation: &str, endpoint: &str, activity_id: &str) -> Self {
        let _ = endpoint;
        let mut log = pool.rent();
        let op_span = log.push_span(SpanKind::Operation, NO_PARENT, 0, 0);
        log.attr_str(op_span, AttrKey::OperationName, operation);
        log.attr_str(op_span, AttrKey::ActivityId, activity_id);
        Self {
            pool: pool.clone(),
            log: Some(log),
            op_span,
            start: Instant::now(),
            outcome: Outcome::Success,
            attempt_count: 0,
            total_ns: 0,
        }
    }

    /// Nanoseconds elapsed since the operation started (monotonic, from [`Instant`]).
    pub fn elapsed_ns(&self) -> u64 {
        self.start.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64
    }

    /// Records one attempt (or hedge leg). `start_ns` is set from the recorder's clock if `0`.
    pub fn record_attempt(&mut self, mut attempt: AttemptRecord) {
        let op_span = self.op_span;
        let Some(log) = self.log.as_mut() else {
            return;
        };
        if attempt.start_ns == 0 {
            attempt.start_ns = self.start.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64;
        }
        let end_ns = attempt.start_ns.saturating_add(attempt.duration_ns);
        let span = log.push_span(SpanKind::Attempt, op_span, attempt.start_ns, end_ns);

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
        log.attr_u64(span, AttrKey::Status, u64::from(attempt.status));
        if let Some(sub) = attempt.sub_status {
            log.attr_u64(span, AttrKey::SubStatus, u64::from(sub));
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
        let at = self.start.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64;
        let Some(log) = self.log.as_mut() else {
            return;
        };
        let span = log.push_span(SpanKind::Hedge, op_span, at, at);
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
        let total = total_ns.unwrap_or_else(|| self.elapsed_ns());
        self.outcome = outcome;
        self.attempt_count = attempt_count;
        self.total_ns = total;
        let op_span = self.op_span;
        if let Some(log) = self.log.as_mut() {
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
            log.attr_u64(op_span, AttrKey::FinalStatus, u64::from(final_status));
            if let Some(sub) = final_sub_status {
                log.attr_u64(op_span, AttrKey::FinalSubStatus, u64::from(sub));
            }
        }
    }

    /// The recorded operation outcome.
    pub fn outcome(&self) -> Outcome {
        self.outcome
    }

    /// The recorded total elapsed time (nanoseconds).
    pub fn total_ns(&self) -> u64 {
        self.total_ns
    }

    /// The number of log entries (spans + attributes) appended on the hot path.
    pub fn raw_len(&self) -> usize {
        self.log
            .as_ref()
            .map_or(0, |log| log.spans().len() + log.attrs().len())
    }

    /// Borrows the captured event log (for reconstruction past the gate).
    pub(crate) fn log(&self) -> Option<&EventLog> {
        self.log.as_ref()
    }

    /// Returns the backing log to the pool, consuming the recorder.
    pub(crate) fn return_buffer(mut self) {
        if let Some(log) = self.log.take() {
            self.pool.give_back(log);
        }
    }
}

impl Drop for DiagnosticsRecorder {
    fn drop(&mut self) {
        // Cancellation / panic safety: return the log to the pool if the gate didn't consume it.
        // `give_back` clears it, so a partially written log never poisons the next rent.
        if let Some(log) = self.log.take() {
            self.pool.give_back(log);
        }
    }
}
