// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Deferred, threshold-gated diagnostics **capture** — the Cosmos driver's diagnostics engine.
//!
//! This module **owns** the canonical diagnostics model
//! ([`DiagnosticsContext`](crate::diagnostics::DiagnosticsContext) and its builder, in the `model`
//! submodule) and provides a cheap, append-only, lock-free hot-path recorder plus an
//! operation-end gate. The driver collects diagnostics by feeding the capture-owned builder, and
//! the gate decides whether the resulting context is surfaced. There is one diagnostics model, not
//! a parallel one; the model is re-exported from `crate::diagnostics` so the public boundary is
//! unchanged.
//!
//! 1. **Hot path — append-only, pooled, lock-free.** Each operation rents one buffer from a
//!    [`LogPool`](crate::diagnostics::capture::LogPool) and a
//!    [`DiagnosticsRecorder`](crate::diagnostics::capture::DiagnosticsRecorder) appends a compact
//!    record per attempt / hedge leg. Appends go through `&mut`, so there is no per-attempt lock
//!    and almost nothing is allocated after pool warm-up.
//! 2. **Gate — decide at the end.** When the outcome and elapsed time are known, a
//!    [`DiagnosticsPolicy`](crate::diagnostics::capture::DiagnosticsPolicy) decides whether to
//!    surface diagnostics. If not, the buffer goes back to the pool — effectively free.
//! 3. **Build — only when wanted.** Past the gate, the log is parsed once and replayed onto the
//!    capture-owned `DiagnosticsContextBuilder` to produce a [`DiagnosticsContext`], mapping each
//!    attempt to a [`RequestDiagnostics`](crate::diagnostics::RequestDiagnostics) (with the right
//!    [`ExecutionContext`](crate::diagnostics::ExecutionContext)) and attaching
//!    [`HedgeDiagnostics`](crate::diagnostics::HedgeDiagnostics) for a hedged operation. In the
//!    live driver path the pipeline feeds the same builder with the full rich data and true
//!    wall-clock timing.
//!
//! The gate defaults to [`Mode::Always`](crate::diagnostics::capture::Mode::Always) — diagnostics
//! are produced out-of-the-box; set [`Mode::Threshold`](crate::diagnostics::capture::Mode::Threshold)
//! or [`Mode::Off`](crate::diagnostics::capture::Mode::Off) via
//! [`DriverOptionsBuilder::with_capture_diagnostics_policy`](crate::options::DriverOptionsBuilder)
//! (via [`DriverOptions::builder`](crate::options::DriverOptions::builder)) to make the
//! hot path cheaper.
//!
//! # Example
//!
//! ```
//! use azure_data_cosmos_driver::diagnostics::capture::{
//!     AttemptRecord, DiagnosticsPolicy, DiagnosticsRecorder, LogPool, Outcome,
//! };
//! use azure_data_cosmos_driver::diagnostics::ExecutionContext;
//! use azure_data_cosmos_driver::options::DiagnosticsOptions;
//! use std::sync::Arc;
//! use std::time::Duration;
//!
//! let pool = Arc::new(LogPool::default());
//! let policy = DiagnosticsPolicy::threshold(Duration::from_millis(5));
//!
//! let mut rec = DiagnosticsRecorder::start(&pool, "read_item", "https://acct/", "activity-1");
//! rec.record_attempt(
//!     AttemptRecord::new(ExecutionContext::Initial, "East US", "https://east/", 429)
//!         .with_service_request_id("svc-429")
//!         .with_request_charge(4.2)
//!         .with_duration_ns(3_000_000),
//! );
//! rec.record_attempt(
//!     AttemptRecord::new(ExecutionContext::Retry, "East US", "https://east/", 200)
//!         .with_service_request_id("svc-200")
//!         .with_request_charge(4.2)
//!         .with_duration_ns(4_000_000),
//! );
//! rec.record_end(Outcome::Success, 2, 200, None, Some(7_000_000));
//!
//! // 7 ms > 5 ms threshold => a DiagnosticsContext is built.
//! let ctx = azure_data_cosmos_driver::diagnostics::capture::finish(
//!     rec,
//!     &policy,
//!     Arc::new(DiagnosticsOptions::default()),
//! )
//! .expect("built");
//! assert_eq!(ctx.request_count(), 2);
//! ```

mod context;
mod encode;
mod event;
mod gate;
mod model;
mod pool;
mod recorder;

pub use event::{
    Attr, AttrKey, AttrValue, EventLog, EventLogStorage, Span, SpanId, SpanKind, TimeOffset,
};
pub use gate::{finish, should_build, DiagnosticsPolicy, Mode};
pub use pool::LogPool;
pub use recorder::{AttemptRecord, DiagnosticsRecorder, HedgeOutcome};

// The capture module is the home/owner of the canonical diagnostics model. The driver (its
// pipeline, retry, hedging, transport layers) collects diagnostics by feeding this builder, and
// the gate decides whether to surface the resulting context. These types are re-exported from
// `crate::diagnostics` so the public boundary (`diagnostics::DiagnosticsContext`, consumed by the
// `azure_data_cosmos` SDK) is unchanged.
pub(crate) use model::DiagnosticsContextBuilder;
pub use model::{
    DiagnosticsContext, DiagnosticsSummary, ExecutionContext, FailedTransportShardDiagnostics,
    PipelineType, RequestDiagnostics, RequestEvent, RequestEventType, RequestHandle,
    RequestSentStatus, TransportHttpVersion, TransportKind, TransportSecurity,
    TransportShardDiagnostics,
};

/// The terminal outcome of an operation, used by the gate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// The operation completed successfully.
    Success,
    /// The operation failed.
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::{DiagnosticsContext, ExecutionContext, HedgeTerminalState};
    use crate::options::DiagnosticsOptions;
    use std::sync::Arc;
    use std::time::Duration;

    fn options() -> Arc<DiagnosticsOptions> {
        Arc::new(DiagnosticsOptions::default())
    }

    /// Records an S2-shaped op (retry 429 -> 200) and finishes against `policy`.
    fn render_s2(pool: &Arc<LogPool>, policy: &DiagnosticsPolicy) -> Option<DiagnosticsContext> {
        let mut rec = DiagnosticsRecorder::start(pool, "read_item", "https://acct/", "act-2");
        rec.record_attempt(
            AttemptRecord::new(ExecutionContext::Initial, "East US", "https://east/", 429)
                .with_service_request_id("svc-429")
                .with_request_charge(4.2)
                .with_sub_status(3200)
                .with_duration_ns(3_000_000),
        );
        rec.record_attempt(
            AttemptRecord::new(ExecutionContext::Retry, "East US", "https://east/", 200)
                .with_service_request_id("svc-200")
                .with_request_charge(4.2)
                .with_duration_ns(4_000_000),
        );
        rec.record_end(Outcome::Success, 2, 200, None, Some(7_000_000));
        finish(rec, policy, options())
    }

    #[test]
    fn fast_success_is_dropped_and_buffer_pooled() {
        let pool = Arc::new(LogPool::default());
        let policy = DiagnosticsPolicy::threshold(Duration::from_millis(5));
        let mut rec = DiagnosticsRecorder::start(&pool, "read_item", "https://acct/", "a-1");
        rec.record_attempt(
            AttemptRecord::new(ExecutionContext::Initial, "East US", "https://east/", 200)
                .with_service_request_id("svc-200")
                .with_request_charge(2.5)
                .with_duration_ns(1_000_000),
        );
        rec.record_end(Outcome::Success, 1, 200, None, Some(1_000_000));
        let ctx = finish(rec, &policy, options());
        assert!(ctx.is_none(), "fast success should be gated away");
        assert_eq!(pool.pooled(), 1, "buffer returned to the pool");
    }

    #[test]
    fn slow_op_builds_canonical_diagnostics_context() {
        let pool = Arc::new(LogPool::default());
        let ctx = render_s2(
            &pool,
            &DiagnosticsPolicy::threshold(Duration::from_millis(5)),
        )
        .expect("slow op builds a context");
        assert_eq!(ctx.activity_id().as_str(), "act-2");
        assert_eq!(ctx.request_count(), 2);
        assert_eq!(ctx.status().map(|s| u16::from(s.status_code())), Some(200));
        let requests = ctx.requests();
        assert_eq!(requests[0].execution_context(), ExecutionContext::Initial);
        assert_eq!(requests[1].execution_context(), ExecutionContext::Retry);
        assert_eq!(u16::from(requests[0].status().status_code()), 429);
        assert_eq!(pool.pooled(), 1);
    }

    #[test]
    fn hedged_operation_records_legs_and_terminal_state() {
        let pool = Arc::new(LogPool::default());
        let mut rec = DiagnosticsRecorder::start(&pool, "read_item", "https://acct/", "act-hedge");
        // Primary leg (East US) is slow / no response; the alternate (West US) wins.
        rec.record_attempt(
            AttemptRecord::new(ExecutionContext::Hedging, "East US", "https://east/", 0)
                .with_request_sent("sent")
                .with_duration_ns(8_000_000),
        );
        rec.record_attempt(
            AttemptRecord::new(ExecutionContext::Hedging, "West US", "https://west/", 200)
                .with_service_request_id("svc-west-200")
                .with_request_charge(3.1)
                .with_duration_ns(4_000_000),
        );
        rec.record_hedge_outcome(
            HedgeOutcome::AlternateWon,
            Duration::from_millis(500),
            "East US",
            Some("West US"),
            Some("West US"),
        );
        rec.record_end(Outcome::Success, 2, 200, None, Some(9_000_000));

        let ctx = finish(rec, &DiagnosticsPolicy::always(), options()).expect("built");
        assert_eq!(
            ctx.request_count(),
            2,
            "both hedge legs are RequestDiagnostics"
        );
        let requests = ctx.requests();
        assert_eq!(requests[0].execution_context(), ExecutionContext::Hedging);
        assert_eq!(requests[1].execution_context(), ExecutionContext::Hedging);
        // Regions are normalized through `Region::new` ("East US" -> "eastus") so the
        // capture-built context matches the rest of the driver's canonical region naming.
        assert_eq!(requests[0].region().map(|r| r.as_str()), Some("eastus"));
        assert_eq!(requests[1].region().map(|r| r.as_str()), Some("westus"));

        let hedge = ctx.hedge_diagnostics().expect("hedge diagnostics attached");
        assert_eq!(hedge.terminal_state(), HedgeTerminalState::AlternateWon);
        assert_eq!(hedge.primary_region().as_str(), "eastus");
        assert_eq!(hedge.alternate_region().map(|r| r.as_str()), Some("westus"));
        assert_eq!(hedge.response_region().map(|r| r.as_str()), Some("westus"));
    }

    #[test]
    fn dropped_recorder_before_finish_returns_buffer() {
        let pool = Arc::new(LogPool::default());
        {
            let mut rec = DiagnosticsRecorder::start(&pool, "read_item", "https://acct/", "a-3");
            rec.record_attempt(AttemptRecord::new(
                ExecutionContext::Initial,
                "East US",
                "https://east/",
                200,
            ));
            // no record_end / finish — simulate a cancelled future
        }
        assert_eq!(pool.pooled(), 1, "dropped recorder must return its buffer");
    }

    #[test]
    fn context_json_carries_no_auth_material() {
        let pool = Arc::new(LogPool::default());
        let ctx = render_s2(&pool, &DiagnosticsPolicy::always()).expect("built");
        let json = ctx.to_json_string(None).to_lowercase();
        assert!(!json.contains("authorization"));
        assert!(!json.contains("\"secret\""));
        assert!(!json.contains("bearer "));
        assert!(!json.contains("sig="));
    }
}
