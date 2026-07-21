// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The [`CosmosTracingHandler`]: a tail-sampled OpenTelemetry tracing handler.

use std::time::{Instant, SystemTime};

use azure_core::http::Context;
use azure_data_cosmos_driver::{diagnostics::DiagnosticsContext, DiagnosticsThresholds};
use opentelemetry::global::{self, BoxedTracer};

use super::span_builder::emit_backdated_span_tree;
use crate::diagnostics::{CosmosOperationContext, DiagnosticsHandler};

/// The instrumentation scope name used for the Cosmos tracer.
const TRACER_NAME: &str = "azure_data_cosmos";

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
/// Spans are emitted through the globally-installed OpenTelemetry tracer provider;
/// with no provider installed, emission is a no-op.
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
    tracer: BoxedTracer,
}

impl CosmosTracingHandler {
    /// Creates a handler using the default sampling thresholds.
    pub fn new() -> Self {
        Self::with_thresholds(DiagnosticsThresholds::default())
    }

    /// Creates a handler using the supplied sampling thresholds.
    pub fn with_thresholds(thresholds: DiagnosticsThresholds) -> Self {
        Self {
            thresholds,
            tracer: global::tracer(TRACER_NAME),
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
        emit_backdated_span_tree(
            &self.tracer,
            diagnostics,
            op,
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
