// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Tail-based sampling thresholds for diagnostics emission.
//!
//! [`DiagnosticsThresholds`] captures the per-operation limits that the SDK's
//! emission handlers (tracing, sampled logging) use to decide *whether* a
//! completed operation is "interesting" enough to emit. A completed operation
//! is interesting when it fails or when it crosses one of these thresholds —
//! see [`DiagnosticsContext::is_threshold_violated`](crate::diagnostics::DiagnosticsContext::is_threshold_violated).
//!
//! The defaults mirror the Java SDK's `CosmosDiagnosticsThresholds` so the two
//! stacks behave consistently out of the box, and every threshold is
//! configurable through the usual builder-style options chain.

use std::time::Duration;

/// Default point-operation latency threshold (1 second), matching Java.
const DEFAULT_POINT_OPERATION_LATENCY: Duration = Duration::from_secs(1);

/// Default non-point-operation latency threshold (3 seconds), matching Java.
const DEFAULT_NON_POINT_OPERATION_LATENCY: Duration = Duration::from_secs(3);

/// Default request-charge threshold in Request Units (1000 RU), matching Java.
const DEFAULT_REQUEST_CHARGE_RU: f64 = 1000.0;

/// Default payload-size threshold in bytes (1 MiB), matching Java.
const DEFAULT_PAYLOAD_SIZE_BYTES: u64 = 1024 * 1024;

/// Per-operation thresholds that gate diagnostics emission (tail-based sampling).
///
/// An operation is considered "interesting" — and therefore eligible for a span
/// or a sampled log line — when it *fails* or when it crosses one of these
/// thresholds. Latency is checked against
/// [`point_operation_latency`](Self::point_operation_latency) for point
/// operations (single-item reads/writes) and
/// [`non_point_operation_latency`](Self::non_point_operation_latency) for
/// everything else (queries, batches, bulk, …).
///
/// # Defaults
///
/// The defaults match the Java SDK:
///
/// | Threshold | Default |
/// | --------- | ------- |
/// | Point-operation latency | 1 s |
/// | Non-point-operation latency | 3 s |
/// | Request charge | 1000 RU |
/// | Payload size | 1 MiB |
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use azure_data_cosmos_driver::options::DiagnosticsThresholds;
///
/// // Java-like defaults.
/// let thresholds = DiagnosticsThresholds::default();
/// assert_eq!(thresholds.point_operation_latency(), Duration::from_secs(1));
///
/// // Tune via the options chain.
/// let strict = DiagnosticsThresholds::default()
///     .with_point_operation_latency(Duration::from_millis(250))
///     .with_request_charge(50.0);
/// assert_eq!(strict.point_operation_latency(), Duration::from_millis(250));
/// assert_eq!(strict.request_charge(), 50.0);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct DiagnosticsThresholds {
    point_operation_latency: Duration,
    non_point_operation_latency: Duration,
    request_charge: f64,
    payload_size: u64,
}

impl DiagnosticsThresholds {
    /// Creates thresholds with the Java-like defaults.
    ///
    /// Equivalent to [`DiagnosticsThresholds::default`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the latency threshold for point operations (single-item CRUD).
    ///
    /// Operations slower than this are eligible for emission. Default: 1 second.
    #[must_use]
    pub fn with_point_operation_latency(mut self, latency: Duration) -> Self {
        self.point_operation_latency = latency;
        self
    }

    /// Sets the latency threshold for non-point operations (queries, batches, …).
    ///
    /// Operations slower than this are eligible for emission. Default: 3 seconds.
    #[must_use]
    pub fn with_non_point_operation_latency(mut self, latency: Duration) -> Self {
        self.non_point_operation_latency = latency;
        self
    }

    /// Sets the total request-charge threshold, in Request Units (RU).
    ///
    /// Operations charging more than this are eligible for emission.
    /// Default: 1000 RU.
    ///
    /// Non-finite (`NaN`/±∞) or negative values are **ignored** and leave the
    /// current threshold unchanged. Such values would otherwise corrupt
    /// RU-based sampling: `NaN` makes every `charge > threshold` comparison
    /// false (silently disabling RU sampling), and a negative bound makes it
    /// always true (sampling every operation).
    #[must_use]
    pub fn with_request_charge(mut self, request_charge: f64) -> Self {
        if request_charge.is_finite() && request_charge >= 0.0 {
            self.request_charge = request_charge;
        }
        self
    }

    /// Sets the payload-size threshold, in bytes.
    ///
    /// Default: 1 MiB.
    ///
    /// **Note:** this threshold is retained for parity with the Java SDK and
    /// forward compatibility, but is not evaluated yet because
    /// [`DiagnosticsContext`](crate::diagnostics::DiagnosticsContext) does not
    /// currently carry request/response body sizes.
    #[must_use]
    pub fn with_payload_size(mut self, payload_size: u64) -> Self {
        self.payload_size = payload_size;
        self
    }

    /// Returns the point-operation latency threshold.
    pub fn point_operation_latency(&self) -> Duration {
        self.point_operation_latency
    }

    /// Returns the non-point-operation latency threshold.
    pub fn non_point_operation_latency(&self) -> Duration {
        self.non_point_operation_latency
    }

    /// Returns the request-charge threshold, in Request Units (RU).
    pub fn request_charge(&self) -> f64 {
        self.request_charge
    }

    /// Returns the payload-size threshold, in bytes.
    pub fn payload_size(&self) -> u64 {
        self.payload_size
    }
}

impl Default for DiagnosticsThresholds {
    fn default() -> Self {
        Self {
            point_operation_latency: DEFAULT_POINT_OPERATION_LATENCY,
            non_point_operation_latency: DEFAULT_NON_POINT_OPERATION_LATENCY,
            request_charge: DEFAULT_REQUEST_CHARGE_RU,
            payload_size: DEFAULT_PAYLOAD_SIZE_BYTES,
        }
    }
}

/// Returns `true` when `operation_name` denotes a single-item ("point")
/// operation, which is latency-gated by
/// [`DiagnosticsThresholds::point_operation_latency`].
///
/// Anything not recognized as a point operation (queries, batches, bulk,
/// change feed, …) is treated as a non-point operation. Names are the
/// canonical semantic-convention `db.operation.name` values.
pub(crate) fn is_point_operation(operation_name: &str) -> bool {
    matches!(
        operation_name,
        "read_item" | "create_item" | "upsert_item" | "replace_item" | "delete_item" | "patch_item"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_java() {
        let t = DiagnosticsThresholds::default();
        assert_eq!(t.point_operation_latency(), Duration::from_secs(1));
        assert_eq!(t.non_point_operation_latency(), Duration::from_secs(3));
        assert_eq!(t.request_charge(), 1000.0);
        assert_eq!(t.payload_size(), 1024 * 1024);
    }

    #[test]
    fn builder_overrides_apply() {
        let t = DiagnosticsThresholds::new()
            .with_point_operation_latency(Duration::from_millis(100))
            .with_non_point_operation_latency(Duration::from_millis(500))
            .with_request_charge(10.0)
            .with_payload_size(4096);
        assert_eq!(t.point_operation_latency(), Duration::from_millis(100));
        assert_eq!(t.non_point_operation_latency(), Duration::from_millis(500));
        assert_eq!(t.request_charge(), 10.0);
        assert_eq!(t.payload_size(), 4096);
    }

    #[test]
    fn point_operation_classification() {
        assert!(is_point_operation("read_item"));
        assert!(is_point_operation("create_item"));
        assert!(is_point_operation("patch_item"));
        assert!(!is_point_operation("query_items"));
        assert!(!is_point_operation("execute_batch"));
        assert!(!is_point_operation("unknown"));
    }

    #[test]
    fn request_charge_ignores_non_finite_and_negative() {
        // A valid override applies.
        let base = DiagnosticsThresholds::default().with_request_charge(250.0);
        assert_eq!(base.request_charge(), 250.0);

        // NaN would make `charge > threshold` false for every operation,
        // silently disabling RU sampling; it must be ignored and leave the
        // prior value intact.
        assert_eq!(base.with_request_charge(f64::NAN).request_charge(), 250.0);
        // A negative bound would sample every operation; ignored.
        assert_eq!(base.with_request_charge(-1.0).request_charge(), 250.0);
        // Infinities are non-finite; ignored.
        assert_eq!(
            base.with_request_charge(f64::INFINITY).request_charge(),
            250.0
        );
        // Zero is a valid (finite, non-negative) threshold.
        assert_eq!(base.with_request_charge(0.0).request_charge(), 0.0);
    }
}
