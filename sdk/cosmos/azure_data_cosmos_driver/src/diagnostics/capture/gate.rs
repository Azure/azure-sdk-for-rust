// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The op-end **gate** and its policy.
//!
//! After an operation completes, [`DiagnosticsPolicy`] decides whether the captured log is worth
//! materializing. On a fast success the recorder's buffer is returned to the pool for ~free; on a
//! slow or errored operation (or under [`Mode::Always`]) the log is built into the canonical
//! [`DiagnosticsContext`](crate::diagnostics::DiagnosticsContext) via [`super::context`].

use super::context::build_context;
use super::recorder::{parse, DiagnosticsRecorder};
use super::Outcome;
use crate::diagnostics::DiagnosticsContext;
use crate::options::DiagnosticsOptions;
use std::sync::Arc;
use std::time::Duration;

/// How aggressively diagnostics are built at the gate.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Mode {
    /// Never build. Capture is skipped entirely — truly zero cost.
    #[default]
    Off,
    /// Build only when the threshold rule fires (slow, or errored when `capture_on_error`).
    Threshold,
    /// Always build.
    Always,
}

/// The policy evaluated at the end of an operation to decide whether to build diagnostics.
///
/// The default is [`Mode::Off`] (no cost, no behavior change) — diagnostics capture is opt-in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiagnosticsPolicy {
    /// Build aggressiveness.
    pub mode: Mode,
    /// Build when the operation took longer than this. `None` disables the latency gate.
    pub latency_threshold: Option<Duration>,
    /// Build when the operation failed.
    pub capture_on_error: bool,
}

impl Default for DiagnosticsPolicy {
    fn default() -> Self {
        // Opt-in: off by default, so an unconfigured client pays nothing and behaves unchanged.
        Self {
            mode: Mode::Off,
            latency_threshold: None,
            capture_on_error: true,
        }
    }
}

impl DiagnosticsPolicy {
    /// A policy that never builds diagnostics ([`Mode::Off`]) — the opt-in default.
    ///
    /// Equivalent to [`DiagnosticsPolicy::default`]; provided for symmetry with
    /// [`threshold`](Self::threshold) and [`always`](Self::always).
    pub fn off() -> Self {
        Self::default()
    }

    /// A threshold policy that builds on error or when an operation exceeds `latency_threshold`.
    pub fn threshold(latency_threshold: Duration) -> Self {
        Self {
            mode: Mode::Threshold,
            latency_threshold: Some(latency_threshold),
            capture_on_error: true,
        }
    }

    /// Always build a [`DiagnosticsContext`].
    pub fn always() -> Self {
        Self {
            mode: Mode::Always,
            latency_threshold: None,
            capture_on_error: true,
        }
    }
}

/// Evaluates the gate against a recorder's recorded outcome and elapsed time.
pub fn should_build(outcome: Outcome, total_ns: u64, policy: &DiagnosticsPolicy) -> bool {
    match policy.mode {
        Mode::Off => false,
        Mode::Always => true,
        Mode::Threshold => {
            (policy.capture_on_error && outcome == Outcome::Error)
                || policy
                    .latency_threshold
                    .is_some_and(|t| u128::from(total_ns) > t.as_nanos())
        }
    }
}

/// Applies the gate to a finished recorder: drop cheaply, or build the canonical
/// [`DiagnosticsContext`].
///
/// Returns `None` when the gate dropped the diagnostics (fast success). Either way the recorder's
/// buffer is returned to the pool. Call after [`DiagnosticsRecorder::record_end`].
pub fn finish(
    recorder: DiagnosticsRecorder,
    policy: &DiagnosticsPolicy,
    options: Arc<DiagnosticsOptions>,
) -> Option<DiagnosticsContext> {
    if !should_build(recorder.outcome(), recorder.total_ns(), policy) {
        recorder.return_buffer();
        return None;
    }
    let parsed = parse(recorder.bytes());
    let context = build_context(&parsed, options);
    recorder.return_buffer();
    Some(context)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn off_never_builds() {
        let p = DiagnosticsPolicy::default();
        assert_eq!(p.mode, Mode::Off);
        assert!(!should_build(Outcome::Error, u64::MAX, &p));
        assert!(!should_build(Outcome::Success, 0, &p));
    }

    #[test]
    fn always_always_builds() {
        let p = DiagnosticsPolicy::always();
        assert!(should_build(Outcome::Success, 0, &p));
        assert!(should_build(Outcome::Error, 0, &p));
    }

    #[test]
    fn threshold_builds_on_slow_or_error() {
        let p = DiagnosticsPolicy::threshold(Duration::from_millis(5));
        assert!(!should_build(Outcome::Success, 1_000_000, &p));
        assert!(should_build(Outcome::Success, 6_000_000, &p));
        assert!(should_build(Outcome::Error, 1_000_000, &p));
    }

    #[test]
    fn threshold_without_error_capture_only_gates_on_latency() {
        let p = DiagnosticsPolicy {
            capture_on_error: false,
            ..DiagnosticsPolicy::threshold(Duration::from_millis(5))
        };
        assert!(!should_build(Outcome::Error, 1_000_000, &p));
        assert!(should_build(Outcome::Error, 6_000_000, &p));
    }
}
