// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The op-end **gate** and its policy.
//!
//! After an operation completes, [`DiagnosticsPolicy`] decides whether the captured log is worth
//! materializing. On a fast success the recorder's buffer is returned to the pool for ~free; on a
//! slow or errored operation (or under [`Mode::Always`]) the log is built into the canonical
//! [`DiagnosticsContext`](crate::diagnostics::DiagnosticsContext) via [`super::context`].

use super::context::build_context;
use super::recorder::DiagnosticsRecorder;
use super::Outcome;
use crate::diagnostics::DiagnosticsContext;
use crate::options::DiagnosticsOptions;
use std::sync::Arc;
use std::time::Duration;

/// How aggressively diagnostics are built at the gate.
///
/// `Mode` intentionally does **not** implement [`Default`]: the meaningful default lives on
/// [`DiagnosticsPolicy`] (which defaults to [`Mode::Always`]). Deriving `Default` here would make
/// `Mode::default()` return `Off`, silently contradicting the policy default — so it is omitted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// Never build. Capture is skipped entirely — truly zero cost.
    Off,
    /// Build only when the threshold rule fires (slow, or errored when `capture_on_error`).
    Threshold,
    /// Always build.
    Always,
}

/// The policy evaluated at the end of an operation to decide whether to surface diagnostics.
///
/// The default is [`Mode::Always`] — diagnostics are produced out-of-the-box (matching the
/// driver's historical always-on behavior). Set [`Mode::Threshold`] or [`Mode::Off`] via
/// [`DriverOptions`](crate::options::DriverOptions) to make the hot path cheaper.
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
        // Always-on by default so diagnostics are produced out-of-the-box, matching the driver's
        // historical behavior. Callers opt into the cheaper Threshold/Off modes explicitly.
        Self {
            mode: Mode::Always,
            latency_threshold: None,
            capture_on_error: true,
        }
    }
}

impl DiagnosticsPolicy {
    /// A policy that never builds diagnostics ([`Mode::Off`]) — the cheapest hot path.
    ///
    /// Provided for symmetry with [`threshold`](Self::threshold) and [`always`](Self::always).
    pub fn off() -> Self {
        Self {
            mode: Mode::Off,
            latency_threshold: None,
            capture_on_error: true,
        }
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
/// backing storage is returned to the pool automatically when `recorder` drops (RAII). Call after
/// [`DiagnosticsRecorder::record_end`].
pub fn finish(
    recorder: DiagnosticsRecorder,
    policy: &DiagnosticsPolicy,
    options: Arc<DiagnosticsOptions>,
) -> Option<DiagnosticsContext> {
    if !should_build(recorder.outcome(), recorder.total_ns(), policy) {
        return None;
    }
    // The typed event log *is* the parsed form — reconstruct the tree directly, no byte parse.
    Some(build_context(recorder.log(), options))
    // `recorder` drops here, returning its pooled storage via the `EventLog` lease.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn off_never_builds() {
        let p = DiagnosticsPolicy::off();
        assert_eq!(p.mode, Mode::Off);
        assert!(!should_build(Outcome::Error, u64::MAX, &p));
        assert!(!should_build(Outcome::Success, 0, &p));
    }

    #[test]
    fn default_is_always() {
        let p = DiagnosticsPolicy::default();
        assert_eq!(p.mode, Mode::Always);
        assert!(should_build(Outcome::Success, 0, &p));
        assert!(should_build(Outcome::Error, 0, &p));
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
