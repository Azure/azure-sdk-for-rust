// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The [`CosmosTracingHandler`]: a tail-sampled OpenTelemetry tracing handler.

use std::time::{Instant, SystemTime};

use azure_core::http::Context;
use azure_data_cosmos_driver::{diagnostics::DiagnosticsContext, DiagnosticsThresholds};
use opentelemetry::global;

use super::span_builder::emit_backdated_span_tree;
use crate::diagnostics::rate_limiter::{RateLimiter, RateLimiterConfig};
use crate::diagnostics::reason::EmitReason;
use crate::diagnostics::{CosmosOperationContext, DiagnosticsHandler};

/// The instrumentation scope name used for the Cosmos tracer.
const TRACER_NAME: &str = "azure_data_cosmos";

/// `tracing` target for the "suppressed N span tree(s)" notice.
const SUPPRESSED_TARGET: &str = "azure_data_cosmos::diagnostics::tracing_suppressed";

/// A [`DiagnosticsHandler`] that emits a backdated OpenTelemetry span tree for
/// operations that fail or cross a sampling threshold.
///
/// This handler implements **tail-based sampling**: it inspects the *completed*
/// [`DiagnosticsContext`] and only emits when
/// [`should_emit`](Self::should_emit) is satisfied — i.e. the operation failed
/// or breached one of the configured [`DiagnosticsThresholds`]. A fast, successful
/// point read therefore produces **no** span at all, keeping the common path
/// free of tracing overhead.
///
/// When it does emit, it reconstructs the operation as a root span with one child
/// span per retained attempt, each backdated to the time the work actually
/// happened (see the module docs).
///
/// Register it with
/// [`CosmosClientBuilder::with_diagnostics_handler`](crate::CosmosClientBuilder::with_diagnostics_handler).
/// Spans are emitted through the globally-installed OpenTelemetry tracer provider,
/// resolved lazily on each sampled emission — so a provider installed *after* the
/// handler (or client) is constructed is still picked up. With no provider
/// installed, emission is a no-op.
///
/// Span emission is **rate-limited** per window (≈100/min by default, with a
/// bounded failure reserve) so an error storm can't reconstruct millions of span
/// trees per second and overwhelm exporters. When trees are suppressed, a single
/// "suppressed N" warning is emitted per window on the
/// `azure_data_cosmos::diagnostics::tracing_suppressed` target. Tune it with
/// [`with_thresholds_and_rate_limit`](Self::with_thresholds_and_rate_limit).
///
/// # Examples
///
/// ```
/// use std::sync::Arc;
/// use azure_data_cosmos::diagnostics::CosmosTracingHandler;
///
/// let handler = Arc::new(CosmosTracingHandler::new());
/// // let client = CosmosClient::builder(endpoint, credential)
/// //     .with_diagnostics_handler(handler)
/// //     .build()?;
/// # let _ = handler;
/// ```
pub struct CosmosTracingHandler {
    thresholds: DiagnosticsThresholds,
    limiter: RateLimiter,
}

impl CosmosTracingHandler {
    /// Creates a handler using the default sampling thresholds and default span
    /// emission rate limiting (~100/min).
    pub fn new() -> Self {
        Self::with_thresholds(DiagnosticsThresholds::default())
    }

    /// Creates a handler using the supplied sampling thresholds and default span
    /// emission rate limiting.
    pub fn with_thresholds(thresholds: DiagnosticsThresholds) -> Self {
        Self::with_thresholds_and_rate_limit(thresholds, RateLimiterConfig::default())
    }

    /// Creates a handler using the supplied sampling thresholds and span emission
    /// rate-limiter configuration.
    ///
    /// The rate limit bounds how many span trees are reconstructed per window
    /// across all operations, so a failure storm can't overwhelm CPU/exporters.
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

    /// Returns whether the given completed context should emit a span, per the
    /// tail-based sampling policy: emit iff the operation failed or crossed a
    /// threshold.
    pub fn should_emit(&self, diagnostics: &DiagnosticsContext) -> bool {
        should_emit_span(diagnostics, &self.thresholds, None)
    }
}

impl Default for CosmosTracingHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DiagnosticsHandler for CosmosTracingHandler {
    fn handle(&self, diagnostics: &DiagnosticsContext, cx: &Context<'_>) {
        let op = cx.value::<CosmosOperationContext>();
        if !should_emit_span(diagnostics, &self.thresholds, op) {
            return;
        }

        // Bound span reconstruction across operations so an error storm can't
        // emit millions of trees per second. This is a synchronous, per-failure
        // cost, so the limit applies before we build anything.
        let decision = self.limiter.check(diagnostics.is_failure(), Instant::now());
        if let Some(suppressed) = decision.suppression_notice {
            tracing::warn!(
                target: SUPPRESSED_TARGET,
                suppressed,
                "cosmos diagnostics: suppressed {suppressed} tracing span tree(s) until window reset"
            );
        }
        if !decision.emit {
            return;
        }

        // Resolve the global tracer lazily, on the (rare) sampled emission path,
        // rather than caching it at construction. `global::tracer` binds to
        // whatever provider is installed *now*; caching it in the handler would
        // permanently capture the no-op default whenever the handler is built
        // before `global::set_tracer_provider`, silently dropping every sampled
        // span even after a provider is installed later.
        let tracer = global::tracer(TRACER_NAME);
        let reason = EmitReason::of(diagnostics, &self.thresholds, op).map(EmitReason::as_str);
        emit_backdated_span_tree(
            &tracer,
            diagnostics,
            op,
            reason,
            Instant::now(),
            SystemTime::now(),
        );
    }
}

/// The tail-based sampling decision: emit a span iff the operation completed and
/// either failed or crossed one of the sampling thresholds.
///
/// `op` supplies the SDK-side operation identity so the threshold classifier
/// can distinguish point from non-point operations; production driver contexts
/// do not carry the operation name.
pub(crate) fn should_emit_span(
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
