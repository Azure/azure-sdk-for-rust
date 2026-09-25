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
/// span per retained attempt, each backdated to the time the work happened.
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
/// `azure_data_cosmos::diagnostics::tracing_suppressed` target. Configure it with
/// [`builder`](Self::builder).
///
/// # Examples
///
/// ```
/// use std::sync::Arc;
/// use azure_data_cosmos::diagnostics::CosmosTracingHandler;
///
/// let handler = Arc::new(CosmosTracingHandler::builder().build());
/// assert_eq!(Arc::strong_count(&handler), 1);
/// ```
pub struct CosmosTracingHandler {
    thresholds: DiagnosticsThresholds,
    limiter: RateLimiter,
}

/// Builder for a tail-sampled Cosmos tracing handler.
#[derive(Default)]
pub struct CosmosTracingHandlerBuilder {
    thresholds: DiagnosticsThresholds,
    rate_limit: RateLimiterConfig,
}

/// A Cosmos tracing handler bound to an explicit OpenTelemetry tracer.
pub struct CosmosTracingHandlerWithTracer<T> {
    handler: CosmosTracingHandler,
    tracer: T,
}

impl CosmosTracingHandler {
    /// Creates a builder for configuring a tracing handler.
    pub fn builder() -> CosmosTracingHandlerBuilder {
        CosmosTracingHandlerBuilder::default()
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

    fn prepare_emission(
        &self,
        diagnostics: &DiagnosticsContext,
        cx: &Context<'_>,
    ) -> Option<Option<&'static str>> {
        let op = cx.value::<CosmosOperationContext>();
        if !should_emit_span(diagnostics, &self.thresholds, op) {
            return None;
        }
        let decision = self.limiter.check(diagnostics.is_failure(), Instant::now());
        if let Some(suppressed) = decision.suppression_notice {
            tracing::warn!(
                target: SUPPRESSED_TARGET,
                suppressed,
                "cosmos diagnostics: suppressed {suppressed} tracing span tree(s) until window reset"
            );
        }
        if !decision.emit {
            return None;
        }
        Some(EmitReason::of(diagnostics, &self.thresholds, op).map(EmitReason::as_str))
    }

    fn emit_with<T>(&self, tracer: &T, diagnostics: &DiagnosticsContext, cx: &Context<'_>)
    where
        T: opentelemetry::trace::Tracer,
    {
        let Some(reason) = self.prepare_emission(diagnostics, cx) else {
            return;
        };
        emit_backdated_span_tree(
            tracer,
            diagnostics,
            cx.value::<CosmosOperationContext>(),
            reason,
            Instant::now(),
            SystemTime::now(),
        );
    }
}

impl CosmosTracingHandlerBuilder {
    /// Sets the tail-sampling thresholds.
    pub fn with_thresholds(mut self, thresholds: DiagnosticsThresholds) -> Self {
        self.thresholds = thresholds;
        self
    }

    /// Sets the span-tree emission rate limit.
    pub fn with_rate_limit(mut self, rate_limit: RateLimiterConfig) -> Self {
        self.rate_limit = rate_limit;
        self
    }

    /// Builds a handler that resolves the process-global tracer when an
    /// operation is selected for emission.
    pub fn build(self) -> CosmosTracingHandler {
        CosmosTracingHandler {
            thresholds: self.thresholds,
            limiter: RateLimiter::new(self.rate_limit),
        }
    }

    /// Builds a handler bound to `tracer` rather than process-global state.
    pub fn build_with_tracer<T>(self, tracer: T) -> CosmosTracingHandlerWithTracer<T>
    where
        T: opentelemetry::trace::Tracer + Send + Sync + 'static,
    {
        CosmosTracingHandlerWithTracer {
            handler: self.build(),
            tracer,
        }
    }
}

impl DiagnosticsHandler for CosmosTracingHandler {
    fn handle(&self, diagnostics: &DiagnosticsContext, cx: &Context<'_>) {
        // Resolve the global tracer lazily, on the (rare) sampled emission path,
        // rather than caching it at construction. `global::tracer` binds to
        // whatever provider is installed *now*; caching it in the handler would
        // permanently capture the no-op default whenever the handler is built
        // before `global::set_tracer_provider`, silently dropping every sampled
        // span even after a provider is installed later.
        let Some(reason) = self.prepare_emission(diagnostics, cx) else {
            return;
        };
        let tracer = global::tracer(TRACER_NAME);
        emit_backdated_span_tree(
            &tracer,
            diagnostics,
            cx.value::<CosmosOperationContext>(),
            reason,
            Instant::now(),
            SystemTime::now(),
        );
    }
}

impl<T> DiagnosticsHandler for CosmosTracingHandlerWithTracer<T>
where
    T: opentelemetry::trace::Tracer + Send + Sync + 'static,
{
    fn handle(&self, diagnostics: &DiagnosticsContext, cx: &Context<'_>) {
        self.handler.emit_with(&self.tracer, diagnostics, cx);
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
