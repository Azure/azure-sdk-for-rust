// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Composable, tail-sampled diagnostics logging.
//!
//! [`SamplingLogHandler`] is a **wrapper**: it applies tail-based sampling and a
//! per-window rate limit, then delegates the actual emission to an inner
//! [`DiagnosticsHandler`]. [`TracingLogHandler`] is the default **leaf**: it
//! writes a compact diagnostics line through [`tracing`]. Composing them
//! (`SamplingLogHandler` around `TracingLogHandler`) reproduces the built-in
//! "sampled log" behavior, while letting callers swap in any inner handler.
//!
//! [`tracing`]: https://docs.rs/tracing

use std::sync::Arc;
use std::time::Instant;

use azure_core::http::Context;
use azure_data_cosmos_driver::{
    diagnostics::DiagnosticsContext, DiagnosticsThresholds, DiagnosticsVerbosity,
};

use crate::diagnostics::rate_limiter::{RateLimiter, RateLimiterConfig};
use crate::diagnostics::reason::EmitReason;
use crate::diagnostics::{CosmosOperationContext, DiagnosticsHandler};

/// `tracing` target for emitted sampled-diagnostics lines.
const SAMPLED_TARGET: &str = "azure_data_cosmos::diagnostics::sampled";

/// `tracing` target for the "suppressed N until reset" notice.
const SUPPRESSED_TARGET: &str = "azure_data_cosmos::diagnostics::suppressed";

/// A leaf [`DiagnosticsHandler`] that writes a compact diagnostics line for the
/// context it is handed, through the [`tracing`](https://docs.rs/tracing)
/// ecosystem.
///
/// It performs **no** sampling or rate limiting of its own — it emits for every
/// context passed to [`handle`](DiagnosticsHandler::handle). Wrap it in a
/// [`SamplingLogHandler`] (the default composition) to gate emission on failures
/// / threshold breaches and cap the rate under a storm, or register it directly
/// to log every completed operation.
///
/// Failed operations are logged at `warn`, everything else at `info`; both use
/// the `azure_data_cosmos::diagnostics::sampled` target.
///
/// # Examples
///
/// ```
/// use std::sync::Arc;
/// use azure_data_cosmos::diagnostics::TracingLogHandler;
///
/// let handler = Arc::new(TracingLogHandler::new());
/// # let _ = handler;
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct TracingLogHandler;

impl TracingLogHandler {
    /// Creates a `TracingLogHandler`.
    pub fn new() -> Self {
        Self
    }
}

impl DiagnosticsHandler for TracingLogHandler {
    fn handle(&self, diagnostics: &DiagnosticsContext, cx: &Context<'_>) {
        // The sampling wrapper stamps *why* the operation was sampled onto the
        // context; when this handler is used directly (unsampled), fall back to a
        // stable marker so the field is always present.
        let reason = cx
            .value::<EmitReason>()
            .copied()
            .map(EmitReason::as_str)
            .unwrap_or("unsampled");

        // Compute the JSON line inside each macro call so `to_json_string` is
        // only evaluated when a subscriber is actually listening for the event
        // (the `tracing` macros run an "is enabled" check before evaluating
        // field expressions).
        if diagnostics.is_failure() {
            tracing::warn!(target: SAMPLED_TARGET, reason, diagnostics = %diagnostics.to_json_string(Some(DiagnosticsVerbosity::Summary)), "cosmos operation diagnostics");
        } else {
            tracing::info!(target: SAMPLED_TARGET, reason, diagnostics = %diagnostics.to_json_string(Some(DiagnosticsVerbosity::Summary)), "cosmos operation diagnostics");
        }
    }
}

/// A wrapping [`DiagnosticsHandler`] that applies tail-based sampling and a
/// per-window rate limit, delegating emission to an inner handler.
///
/// For each completed operation it first applies the same tail-based sampling
/// gate the tracing handler uses — emit if, and only if, the operation *is
/// completed* and *failed* or breached a [`DiagnosticsThresholds`]. It then
/// limits how many emissions are dispatched during a time window (≈100/min by
/// default); when emissions are suppressed a single "suppressed N until reset"
/// warning is emitted per window. Only when both gates allow does it call the
/// inner handler.
///
/// The inner handler defaults to a [`TracingLogHandler`], so a bare
/// `SamplingLogHandler::new()` reproduces the built-in sampled-log behavior. Pass
/// your own inner handler with [`with_handler`](Self::with_handler) (or the
/// `*_and_handler` constructors) to feed sampled, rate-limited contexts to any
/// other sink.
///
/// Register it with
/// [`CosmosClientBuilder::with_diagnostics_handler`](crate::CosmosClientBuilder::with_diagnostics_handler).
///
/// # Examples
///
/// ```
/// use std::sync::Arc;
/// use azure_data_cosmos::diagnostics::SamplingLogHandler;
///
/// // Default: samples + rate-limits, then logs via `tracing`.
/// let handler = Arc::new(SamplingLogHandler::new());
/// # let _ = handler;
/// ```
pub struct SamplingLogHandler {
    thresholds: DiagnosticsThresholds,
    limiter: RateLimiter,
    inner: Arc<dyn DiagnosticsHandler>,
}

impl SamplingLogHandler {
    /// Creates a handler with default thresholds and rate limiting (~100/min)
    /// wrapping a default [`TracingLogHandler`].
    pub fn new() -> Self {
        Self::with_thresholds(DiagnosticsThresholds::default())
    }

    /// Creates a handler with the supplied sampling thresholds, default rate
    /// limiting, and a default [`TracingLogHandler`] inner.
    pub fn with_thresholds(thresholds: DiagnosticsThresholds) -> Self {
        Self::with_thresholds_and_rate_limit(thresholds, RateLimiterConfig::default())
    }

    /// Creates a handler with the supplied sampling thresholds and rate-limiter
    /// configuration, wrapping a default [`TracingLogHandler`] inner.
    pub fn with_thresholds_and_rate_limit(
        thresholds: DiagnosticsThresholds,
        rate_limit: RateLimiterConfig,
    ) -> Self {
        Self::with_thresholds_rate_limit_and_handler(
            thresholds,
            rate_limit,
            Arc::new(TracingLogHandler::new()),
        )
    }

    /// Creates a handler that samples and rate-limits with the defaults, then
    /// delegates emission to `inner`.
    pub fn with_handler(inner: Arc<dyn DiagnosticsHandler>) -> Self {
        Self::with_thresholds_rate_limit_and_handler(
            DiagnosticsThresholds::default(),
            RateLimiterConfig::default(),
            inner,
        )
    }

    /// Creates a handler with the supplied thresholds and default rate limiting,
    /// delegating emission to `inner`.
    pub fn with_thresholds_and_handler(
        thresholds: DiagnosticsThresholds,
        inner: Arc<dyn DiagnosticsHandler>,
    ) -> Self {
        Self::with_thresholds_rate_limit_and_handler(
            thresholds,
            RateLimiterConfig::default(),
            inner,
        )
    }

    /// Creates a handler with the supplied thresholds and rate-limiter
    /// configuration, delegating emission to `inner`.
    pub fn with_thresholds_rate_limit_and_handler(
        thresholds: DiagnosticsThresholds,
        rate_limit: RateLimiterConfig,
        inner: Arc<dyn DiagnosticsHandler>,
    ) -> Self {
        Self {
            thresholds,
            limiter: RateLimiter::new(rate_limit),
            inner,
        }
    }

    /// Returns the sampling thresholds this handler applies.
    pub fn thresholds(&self) -> &DiagnosticsThresholds {
        &self.thresholds
    }

    /// Returns whether the given completed context would pass the tail-based
    /// sampling gate (before rate limiting).
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

        let decision = self.limiter.check(diagnostics.is_failure(), Instant::now());

        if let Some(suppressed) = decision.suppression_notice {
            tracing::warn!(
                target: SUPPRESSED_TARGET,
                suppressed,
                "cosmos diagnostics: suppressed {suppressed} sampled log line(s) until window reset"
            );
        }

        if decision.emit {
            // Stamp *why* the operation was sampled onto the context so the inner
            // handler can surface it. `should_log` passed, so a reason exists.
            match EmitReason::of(diagnostics, &self.thresholds, op) {
                Some(reason) => {
                    let cx = cx.clone().with_value(reason);
                    self.inner.handle(diagnostics, &cx);
                }
                None => self.inner.handle(diagnostics, cx),
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
