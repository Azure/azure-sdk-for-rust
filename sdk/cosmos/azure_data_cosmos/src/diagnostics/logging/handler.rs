// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The [`SamplingLogHandler`]: a rate-limited, tail-sampled diagnostics logger.

use std::time::Instant;

use azure_core::http::Context;
use azure_data_cosmos_driver::{
    diagnostics::DiagnosticsContext, DiagnosticsThresholds, DiagnosticsVerbosity,
};

use super::rate_limiter::{RateLimiter, RateLimiterConfig};
use crate::diagnostics::{CosmosOperationContext, DiagnosticsHandler};

/// `tracing` target for emitted sampled-diagnostics lines.
const SAMPLED_TARGET: &str = "azure_data_cosmos::diagnostics::sampled";

/// `tracing` target for the "suppressed N until reset" notice.
const SUPPRESSED_TARGET: &str = "azure_data_cosmos::diagnostics::suppressed";

/// A [`DiagnosticsHandler`] that logs a compact diagnostics line for operations
/// that fail or cross a sampling threshold, rate-limited during storms.
///
/// For each completed operation the handler applies the same tail-based sampling
/// gate as the tracing handler: the diagnostics are logged if, and only if, the
/// operation *is completed* and *failed* or breached a [`DiagnosticsThresholds`].
/// In addition, the handler limits how many lines are emitted during a time window
/// (≈100/min by default). The number of diagnostics emitted is always bounded by
/// the specified rate limit. When diagnostics are suppressed, a single warning is
/// emitted indicating the time remaining until diagnostics are restored.
///
/// Diagnostics are all emitted through [`tracing`](https://docs.rs/tracing).
///
/// Register this handler using
/// [`CosmosClientBuilder::with_diagnostics_handler`](crate::CosmosClientBuilder::with_diagnostics_handler).
///
/// # Examples
///
/// ```
/// use std::sync::Arc;
/// use azure_data_cosmos::diagnostics::SamplingLogHandler;
///
/// let handler = Arc::new(SamplingLogHandler::new());
/// # let _ = handler;
/// ```
pub struct SamplingLogHandler {
    thresholds: DiagnosticsThresholds,
    limiter: RateLimiter,
}

impl SamplingLogHandler {
    /// Creates a handler with default thresholds and rate limiting (~100/min).
    pub fn new() -> Self {
        Self::with_thresholds_and_rate_limit(
            DiagnosticsThresholds::default(),
            RateLimiterConfig::default(),
        )
    }

    /// Creates a handler with the supplied sampling thresholds and default rate
    /// limiting.
    pub fn with_thresholds(thresholds: DiagnosticsThresholds) -> Self {
        Self::with_thresholds_and_rate_limit(thresholds, RateLimiterConfig::default())
    }

    /// Creates a handler with the supplied sampling thresholds and rate-limiter
    /// configuration.
    pub fn with_thresholds_and_rate_limit(
        thresholds: DiagnosticsThresholds,
        rate_limit: RateLimiterConfig,
    ) -> Self {
        Self {
            thresholds,
            limiter: RateLimiter::new(rate_limit),
        }
    }

    /// Returns the sampling thresholds this handler applies.
    pub fn thresholds(&self) -> &DiagnosticsThresholds {
        &self.thresholds
    }

    /// Returns whether the given completed context should be logged, per the
    /// tail-based sampling gate (before rate limiting).
    pub fn should_log(&self, diagnostics: &DiagnosticsContext) -> bool {
        should_log(diagnostics, &self.thresholds, None)
    }
}

impl Default for SamplingLogHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DiagnosticsHandler for SamplingLogHandler {
    fn handle(&self, diagnostics: &DiagnosticsContext, cx: &Context<'_>) {
        let op = cx.value::<CosmosOperationContext>();
        if !should_log(diagnostics, &self.thresholds, op) {
            return;
        }

        let is_failure = diagnostics.is_failure();
        let decision = self.limiter.check(is_failure, Instant::now());

        if let Some(suppressed) = decision.suppression_notice {
            tracing::warn!(
                target: SUPPRESSED_TARGET,
                suppressed,
                "cosmos diagnostics: suppressed {suppressed} sampled log line(s) until window reset"
            );
        }

        if decision.emit {
            // Compute the JSON line inside each macro call so `to_json_string` is
            // only evaluated when a subscriber is actually listening for the event
            // (the `tracing` macros run an "is enabled" check before evaluating
            // field expressions).
            if is_failure {
                tracing::warn!(target: SAMPLED_TARGET, diagnostics = %diagnostics.to_json_string(Some(DiagnosticsVerbosity::Summary)), "cosmos operation diagnostics");
            } else {
                tracing::info!(target: SAMPLED_TARGET, diagnostics = %diagnostics.to_json_string(Some(DiagnosticsVerbosity::Summary)), "cosmos operation diagnostics");
            }
        }
    }
}

/// The tail-based sampling gate: log iff the operation is completed and either
/// failed or crossed a sampling threshold.
///
/// `op` supplies the SDK-side operation identity so the threshold classifier
/// can distinguish point from non-point operations; production driver contexts
/// do not carry the operation name.
pub(crate) fn should_log(
    diagnostics: &DiagnosticsContext,
    thresholds: &DiagnosticsThresholds,
    op: Option<&CosmosOperationContext>,
) -> bool {
    diagnostics.is_completed()
        && (diagnostics.is_failure()
            || diagnostics.is_threshold_violated_for(
                thresholds,
                op.and_then(CosmosOperationContext::operation_name),
            ))
}
