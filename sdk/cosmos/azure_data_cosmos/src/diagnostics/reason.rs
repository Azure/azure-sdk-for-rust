// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Why a completed operation was sampled for emission.
//!
//! The sampling handlers emit a diagnostic when an operation *fails* or *crosses
//! a threshold*. [`EmitReason`] captures which of those it was — and, for a
//! threshold breach, which specific bound — so the emitted diagnostic can record
//! *why* it was surfaced, not just that it was.

use azure_data_cosmos_driver::diagnostics::{DiagnosticsContext, ThresholdBreach};
use azure_data_cosmos_driver::DiagnosticsThresholds;

use crate::diagnostics::CosmosOperationContext;

/// The reason a completed operation passed the tail-based sampling gate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum EmitReason {
    /// The operation failed.
    Failure,
    /// The operation succeeded but crossed a sampling threshold.
    Threshold(ThresholdBreach),
}

impl EmitReason {
    /// A stable, low-cardinality identifier suitable for a log field or span
    /// attribute value.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            EmitReason::Failure => "failure",
            EmitReason::Threshold(breach) => match breach {
                ThresholdBreach::PointLatency => "point_latency",
                ThresholdBreach::NonPointLatency => "non_point_latency",
                ThresholdBreach::RequestCharge => "request_charge",
                // `ThresholdBreach` is `#[non_exhaustive]`; keep a stable
                // fallback if a new breach kind is added upstream.
                _ => "threshold",
            },
        }
    }

    /// Computes why `diagnostics` would be sampled against `thresholds`: a failure
    /// takes precedence, otherwise the specific threshold crossed. Returns `None`
    /// when the operation is neither a failure nor a threshold breach.
    ///
    /// `op` supplies the SDK operation identity used to pick the point vs
    /// non-point latency threshold, mirroring the sampling gate.
    pub(crate) fn of(
        diagnostics: &DiagnosticsContext,
        thresholds: &DiagnosticsThresholds,
        op: Option<&CosmosOperationContext>,
    ) -> Option<EmitReason> {
        if diagnostics.is_failure() {
            return Some(EmitReason::Failure);
        }
        diagnostics
            .threshold_breach_for(
                thresholds,
                op.and_then(CosmosOperationContext::operation_name),
            )
            .map(EmitReason::Threshold)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_str_is_stable() {
        assert_eq!(EmitReason::Failure.as_str(), "failure");
        assert_eq!(
            EmitReason::Threshold(ThresholdBreach::PointLatency).as_str(),
            "point_latency"
        );
        assert_eq!(
            EmitReason::Threshold(ThresholdBreach::NonPointLatency).as_str(),
            "non_point_latency"
        );
        assert_eq!(
            EmitReason::Threshold(ThresholdBreach::RequestCharge).as_str(),
            "request_charge"
        );
    }
}
