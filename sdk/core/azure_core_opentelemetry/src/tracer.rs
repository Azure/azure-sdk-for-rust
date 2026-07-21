// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    attributes::OpenTelemetryAttribute,
    span::{OpenTelemetrySpan, OpenTelemetrySpanKind},
};

use azure_core::time::OffsetDateTime;
use azure_core::tracing::{SpanKind, SpanOptions, Tracer};
use opentelemetry::{
    global::BoxedTracer,
    trace::{TraceContextExt, Tracer as OpenTelemetryTracerTrait},
    Context, KeyValue,
};
use std::{borrow::Cow, fmt::Debug, sync::Arc, time::SystemTime};

pub struct OpenTelemetryTracer {
    namespace: Option<&'static str>,
    inner: BoxedTracer,
}

impl OpenTelemetryTracer {
    /// Creates a new OpenTelemetry tracer with the given inner tracer.
    pub(super) fn new(namespace: Option<&'static str>, tracer: BoxedTracer) -> Self {
        Self {
            namespace,
            inner: tracer,
        }
    }

    /// Builds a span within `context`, optionally backdated to `start_time`.
    fn build_span(
        &self,
        name: Cow<'static, str>,
        kind: SpanKind,
        attributes: Vec<azure_core::tracing::Attribute>,
        start_time: Option<OffsetDateTime>,
        context: Context,
    ) -> Arc<dyn azure_core::tracing::Span> {
        let mut span_builder = opentelemetry::trace::SpanBuilder::from_name(name)
            .with_kind(OpenTelemetrySpanKind(kind).into())
            .with_attributes(
                attributes
                    .iter()
                    .map(|attr| KeyValue::from(OpenTelemetryAttribute(attr.clone()))),
            );
        if let Some(start_time) = start_time {
            span_builder = span_builder.with_start_time(SystemTime::from(start_time));
        }
        let span = self.inner.build_with_context(span_builder, &context);

        OpenTelemetrySpan::new(context.with_span(span))
    }

    /// Extracts the OpenTelemetry context from a parent span.
    fn parent_context(parent: &Arc<dyn azure_core::tracing::Span>) -> Context {
        parent
            .as_any()
            .downcast_ref::<OpenTelemetrySpan>()
            .unwrap_or_else(|| {
                panic!(
                    "Could not downcast parent span to OpenTelemetrySpan. Actual type: {}",
                    std::any::type_name::<dyn azure_core::tracing::Span>()
                )
            })
            .context()
            .clone()
    }
}

impl Debug for OpenTelemetryTracer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenTelemetryTracer")
            .field("namespace", &self.namespace)
            .finish_non_exhaustive()
    }
}

impl Tracer for OpenTelemetryTracer {
    fn namespace(&self) -> Option<&'static str> {
        self.namespace
    }

    fn start_span(
        &self,
        name: Cow<'static, str>,
        kind: SpanKind,
        attributes: Vec<azure_core::tracing::Attribute>,
    ) -> Arc<dyn azure_core::tracing::Span> {
        self.build_span(name, kind, attributes, None, Context::current())
    }

    fn start_span_with_options(
        &self,
        name: Cow<'static, str>,
        kind: SpanKind,
        attributes: Vec<azure_core::tracing::Attribute>,
        options: SpanOptions,
    ) -> Arc<dyn azure_core::tracing::Span> {
        self.build_span(
            name,
            kind,
            attributes,
            options.start_time,
            Context::current(),
        )
    }

    fn start_span_with_parent(
        &self,
        name: Cow<'static, str>,
        kind: SpanKind,
        attributes: Vec<azure_core::tracing::Attribute>,
        parent: Arc<dyn azure_core::tracing::Span>,
    ) -> Arc<dyn azure_core::tracing::Span> {
        let context = Self::parent_context(&parent);
        self.build_span(name, kind, attributes, None, context)
    }

    fn start_span_with_parent_and_options(
        &self,
        name: Cow<'static, str>,
        kind: SpanKind,
        attributes: Vec<azure_core::tracing::Attribute>,
        parent: Arc<dyn azure_core::tracing::Span>,
        options: SpanOptions,
    ) -> Arc<dyn azure_core::tracing::Span> {
        let context = Self::parent_context(&parent);
        self.build_span(name, kind, attributes, options.start_time, context)
    }
}

#[cfg(test)]
mod tests {
    use crate::telemetry::OpenTelemetryTracerProvider;
    use azure_core::tracing::{SpanKind, TracerProvider};
    use opentelemetry::trace::noop::NoopTracerProvider;
    use opentelemetry_sdk::trace::SdkTracerProvider;
    use std::sync::Arc;

    #[test]
    fn test_create_tracer() {
        let noop_tracer = NoopTracerProvider::new();
        let otel_provider = OpenTelemetryTracerProvider::new(Arc::new(noop_tracer));
        let tracer = otel_provider.get_tracer(Some("name"), "test_tracer", Some("1.0.0"));
        let span = tracer.start_span("test_span".into(), SpanKind::Internal, vec![]);
        span.end();
    }

    #[test]
    fn test_create_tracer_with_sdk_tracer() {
        let provider = SdkTracerProvider::builder().build();
        let otel_provider = OpenTelemetryTracerProvider::new(Arc::new(provider));
        let _tracer = otel_provider.get_tracer(Some("My.Namespace"), "test_tracer", Some("1.0.0"));
    }

    #[test]
    fn test_create_span_from_tracer() {
        let provider = SdkTracerProvider::builder().build();
        let otel_provider = OpenTelemetryTracerProvider::new(Arc::new(provider));
        let tracer = otel_provider.get_tracer(Some("My.Namespace"), "test_tracer", Some("1.0.0"));
        let _span = tracer.start_span("test_span".into(), SpanKind::Internal, vec![]);
    }
}
