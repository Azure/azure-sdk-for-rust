// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! OpenTelemetry implementation of typespec_client_core tracing traits.

use crate::attributes::AttributeValue as ConversionAttributeValue;
use azure_core::{
    tracing::{AsAny, AttributeValue, Span, SpanGuard, SpanStatus},
    Result,
};
use opentelemetry::trace::TraceContextExt;
use std::{error::Error as StdError, sync::Arc};

/// newtype for Azure Core SpanKind to enable conversion to OpenTelemetry SpanKind
pub(crate) struct OpenTelemetrySpanKind(pub azure_core::tracing::SpanKind);

impl From<OpenTelemetrySpanKind> for opentelemetry::trace::SpanKind {
    fn from(span_kind: OpenTelemetrySpanKind) -> Self {
        match span_kind.0 {
            azure_core::tracing::SpanKind::Internal => opentelemetry::trace::SpanKind::Internal,
            azure_core::tracing::SpanKind::Server => opentelemetry::trace::SpanKind::Server,
            azure_core::tracing::SpanKind::Client => opentelemetry::trace::SpanKind::Client,
            azure_core::tracing::SpanKind::Producer => opentelemetry::trace::SpanKind::Producer,
            azure_core::tracing::SpanKind::Consumer => opentelemetry::trace::SpanKind::Consumer,
        }
    }
}

/// OpenTelemetry implementation of Span
pub(super) struct OpenTelemetrySpan {
    context: opentelemetry::Context,
}

impl OpenTelemetrySpan {
    pub fn new(context: opentelemetry::Context) -> Arc<Self> {
        Arc::new(Self { context })
    }
    pub(super) fn context(&self) -> &opentelemetry::Context {
        &self.context
    }
}

impl Span for OpenTelemetrySpan {
    fn is_recording(&self) -> bool {
        self.context.span().is_recording()
    }

    fn end(&self) {
        self.context.span().end();
    }

    fn span_id(&self) -> [u8; 8] {
        self.context.span().span_context().span_id().to_bytes()
    }

    fn set_attribute(&self, key: &'static str, value: AttributeValue) {
        let otel_value = opentelemetry::Value::from(ConversionAttributeValue(value));
        self.context
            .span()
            .set_attribute(opentelemetry::KeyValue::new(key, otel_value));
    }

    fn record_error(&self, error: &dyn StdError) {
        self.context.span().record_error(error);

        self.context
            .span()
            .set_status(opentelemetry::trace::Status::error(error.to_string()));
    }

    fn set_status(&self, status: SpanStatus) {
        let otel_status = match status {
            SpanStatus::Unset => opentelemetry::trace::Status::Unset,
            SpanStatus::Error { description } => opentelemetry::trace::Status::error(description),
        };
        self.context.span().set_status(otel_status);
    }

    fn propagate_headers(&self, _request: &mut azure_core::http::Request) {}

    fn set_current(
        &self,
        _context: &azure_core::http::Context,
    ) -> typespec_client_core::Result<Box<dyn SpanGuard>> {
        // Create a context with the current span
        let context_guard = self.context.clone().attach();

        Ok(Box::new(OpenTelemetrySpanGuard {
            _inner: context_guard,
        }))
    }
}

impl AsAny for OpenTelemetrySpan {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

struct OpenTelemetrySpanGuard {
    _inner: opentelemetry::ContextGuard,
}

impl SpanGuard for OpenTelemetrySpanGuard {
    fn end(self) -> Result<()> {
        // The span is ended when the guard is dropped, so no action needed here.
        Ok(())
    }
}

impl Drop for OpenTelemetrySpanGuard {
    fn drop(&mut self) {
        // The OpenTelemetry context guard will automatically end the span when dropped.
    }
}

#[cfg(test)]
mod tests {
    use crate::telemetry::OpenTelemetryTracerProvider;
    use azure_core::http::Context as AzureContext;
    use azure_core::tracing::{Attribute, AttributeValue, SpanKind, SpanStatus, TracerProvider};
    use opentelemetry::trace::TraceContextExt;
    use opentelemetry::{Context, Key, KeyValue, Value};
    use opentelemetry_sdk::trace::{in_memory_exporter::InMemorySpanExporter, SdkTracerProvider};
    use std::io::{Error, ErrorKind};
    use std::sync::Arc;
    use tracing::trace;

    fn create_exportable_tracer_provider() -> (Arc<SdkTracerProvider>, InMemorySpanExporter) {
        let otel_exporter = InMemorySpanExporter::default();
        let otel_tracer_provider = SdkTracerProvider::builder()
            .with_simple_exporter(otel_exporter.clone())
            .build();
        let otel_tracer_provider = Arc::new(otel_tracer_provider);
        (otel_tracer_provider, otel_exporter)
    }

    #[test]
    fn test_open_telemetry_span_new() {
        let (otel_tracer_provider, otel_exporter) = create_exportable_tracer_provider();

        let tracer_provider = OpenTelemetryTracerProvider::new(otel_tracer_provider);
        assert!(tracer_provider.is_ok());
        let tracer =
            tracer_provider
                .unwrap()
                .get_tracer(Some("Microsoft.SpecialCase"), "test", "0.1.0");
        let span = tracer.start_span("test_span", SpanKind::Client, vec![]);
        span.end();

        let spans = otel_exporter.get_finished_spans().unwrap();
        assert_eq!(spans.len(), 1);
        for span in &spans {
            println!("Span: {:?}", span);
            assert_eq!(span.name, "test_span");
            assert_eq!(span.status, opentelemetry::trace::Status::Unset);
            assert!(span.attributes.is_empty());
        }
    }

    #[test]
    fn test_open_telemetry_span_hierarchy() {
        let (otel_tracer_provider, otel_exporter) = create_exportable_tracer_provider();
        let tracer_provider = OpenTelemetryTracerProvider::new(otel_tracer_provider);
        assert!(tracer_provider.is_ok());
        let tracer = tracer_provider
            .unwrap()
            .get_tracer(Some("Special Name"), "test", "0.1.0");
        let parent_span = tracer.start_span("parent_span", SpanKind::Server, vec![]);
        let child_span = tracer.start_span_with_parent(
            "child_span",
            SpanKind::Client,
            vec![],
            parent_span.clone(),
        );

        child_span.end();
        parent_span.end();

        let spans = otel_exporter.get_finished_spans().unwrap();
        assert_eq!(spans.len(), 2);
        for span in &spans {
            println!("Span: {:?}", span);
            if span.name == "parent_span" {
                assert_eq!(span.status, opentelemetry::trace::Status::Unset);
                assert_eq!(span.parent_span_id, opentelemetry::trace::SpanId::INVALID);
            } else if span.name == "child_span" {
                assert_eq!(span.status, opentelemetry::trace::Status::Unset);
                assert_ne!(span.parent_span_id, opentelemetry::trace::SpanId::INVALID);
                assert_eq!(span.parent_span_id.to_bytes(), parent_span.span_id());
            }
        }
    }

    #[test]
    fn test_open_telemetry_span_start_with_parent() {
        let (otel_tracer_provider, otel_exporter) = create_exportable_tracer_provider();
        let tracer_provider = OpenTelemetryTracerProvider::new(otel_tracer_provider);
        assert!(tracer_provider.is_ok());
        let tracer = tracer_provider
            .unwrap()
            .get_tracer(Some("MyNamespace"), "test", "0.1.0");
        let span1 = tracer.start_span("span1", SpanKind::Internal, vec![]);
        let span2 = tracer.start_span("span2", SpanKind::Server, vec![]);
        let child_span =
            tracer.start_span_with_parent("child_span", SpanKind::Client, vec![], span1.clone());

        child_span.end();
        span2.end();
        span1.end();

        let spans = otel_exporter.get_finished_spans().unwrap();
        assert_eq!(spans.len(), 3);
        for span in &spans {
            println!("Span: {:?}", span);
            if span.name == "span1" {
                assert_eq!(span.status, opentelemetry::trace::Status::Unset);
                assert_eq!(span.parent_span_id, opentelemetry::trace::SpanId::INVALID);
            } else if span.name == "child_span" {
                assert_eq!(span.status, opentelemetry::trace::Status::Unset);
                assert_ne!(span.parent_span_id, opentelemetry::trace::SpanId::INVALID);
                assert_eq!(span.parent_span_id.to_bytes(), span1.span_id());
            }
        }
    }

    #[test]
    fn test_open_telemetry_span_start_with_current() {
        let (otel_tracer_provider, otel_exporter) = create_exportable_tracer_provider();
        let tracer_provider = OpenTelemetryTracerProvider::new(otel_tracer_provider);
        assert!(tracer_provider.is_ok());
        let tracer = tracer_provider
            .unwrap()
            .get_tracer(Some("Namespace"), "test", "0.1.0");
        let span1 = tracer.start_span("span1", SpanKind::Internal, vec![]);
        let span2 = tracer.start_span("span2", SpanKind::Server, vec![]);
        let _span_guard = span2
            .set_current(&azure_core::http::Context::new())
            .unwrap();
        let child_span = tracer.start_span_with_current("child_span", SpanKind::Client, vec![]);

        child_span.end();
        span2.end();
        span1.end();

        let spans = otel_exporter.get_finished_spans().unwrap();
        assert_eq!(spans.len(), 3);
        for span in &spans {
            println!("Span: {:?}", span);
            if span.name == "span1" {
                assert_eq!(span.status, opentelemetry::trace::Status::Unset);
                assert_eq!(span.parent_span_id, opentelemetry::trace::SpanId::INVALID);
            } else if span.name == "child_span" {
                assert_eq!(span.status, opentelemetry::trace::Status::Unset);
                assert_ne!(span.parent_span_id, opentelemetry::trace::SpanId::INVALID);
                assert_eq!(span.parent_span_id.to_bytes(), span2.span_id());
            }
        }
    }

    #[test]
    fn test_open_telemetry_span_set_attribute() {
        let (otel_tracer_provider, otel_exporter) = create_exportable_tracer_provider();
        let tracer_provider = OpenTelemetryTracerProvider::new(otel_tracer_provider);
        assert!(tracer_provider.is_ok());
        let tracer = tracer_provider
            .unwrap()
            .get_tracer(Some("ThisNamespace"), "test", "0.1.0");
        let span = tracer.start_span("test_span", SpanKind::Internal, vec![]);

        span.set_attribute("test_key", AttributeValue::String("test_value".to_string()));
        span.end();

        let spans = otel_exporter.get_finished_spans().unwrap();
        assert_eq!(spans.len(), 1);
        for span in &spans {
            assert_eq!(span.name, "test_span");
            assert_eq!(span.attributes.len(), 1);
            assert_eq!(span.attributes[0].key, "test_key".into());
            assert_eq!(
                format!("{:?}", span.attributes[0].value),
                "String(Owned(\"test_value\"))"
            );
        }
    }

    #[test]
    fn test_open_telemetry_span_record_error() {
        let (otel_tracer_provider, otel_exporter) = create_exportable_tracer_provider();
        let tracer_provider = OpenTelemetryTracerProvider::new(otel_tracer_provider);
        assert!(tracer_provider.is_ok());
        let tracer = tracer_provider
            .unwrap()
            .get_tracer(Some("namespace"), "test", "0.1.0");
        let span = tracer.start_span("test_span", SpanKind::Client, vec![]);

        let error = Error::new(ErrorKind::NotFound, "resource not found");
        span.record_error(&error);
        span.end();

        let spans = otel_exporter.get_finished_spans().unwrap();
        assert_eq!(spans.len(), 1);
        for span in &spans {
            assert_eq!(span.name, "test_span");
            assert_eq!(
                span.status,
                opentelemetry::trace::Status::error("resource not found")
            );
            assert_eq!(span.events.len(), 1);
            assert_eq!(span.events[0].name, "exception");
            assert_eq!(span.events[0].attributes.len(), 1);
            assert_eq!(span.events[0].attributes[0].key, "exception.message".into());
        }
    }

    #[test]
    fn test_open_telemetry_span_set_status() {
        let (otel_tracer_provider, otel_exporter) = create_exportable_tracer_provider();
        let tracer_provider = OpenTelemetryTracerProvider::new(otel_tracer_provider);
        assert!(tracer_provider.is_ok());
        let tracer = tracer_provider
            .unwrap()
            .get_tracer(Some("Namespace"), "test", "0.1.0");

        // Test Unset status
        let span = tracer.start_span("test_span_unset", SpanKind::Server, vec![]);
        span.end();

        // Test Error status
        let span = tracer.start_span("test_span_error", SpanKind::Server, vec![]);
        span.set_status(SpanStatus::Error {
            description: "test error".to_string(),
        });
        span.end();

        let spans = otel_exporter.get_finished_spans().unwrap();
        assert_eq!(spans.len(), 2);

        let error_span = spans.iter().find(|s| s.name == "test_span_error").unwrap();
        assert_eq!(
            error_span.status,
            opentelemetry::trace::Status::error("test error")
        );
        let unset_span = spans.iter().find(|s| s.name == "test_span_unset").unwrap();
        assert_eq!(unset_span.status, opentelemetry::trace::Status::Unset);
    }

    #[tokio::test]
    async fn test_open_telemetry_span_futures() {
        let (otel_tracer_provider, otel_exporter) = create_exportable_tracer_provider();
        let tracer_provider = OpenTelemetryTracerProvider::new(otel_tracer_provider);
        assert!(tracer_provider.is_ok());
        let tracer = tracer_provider
            .unwrap()
            .get_tracer(Some("Namespace"), "test", "0.1.0");

        let future = async {
            let context = Context::current();
            println!("In captured context: {:?}", context);
            context.span().add_event("name", vec![]);
            context.span().set_attribute(KeyValue::new(
                Key::from("test_key"),
                Value::from("test_value"),
            ));
            context.span().end();
            42
        };

        let span = tracer.start_span(
            "test_span",
            SpanKind::Client,
            vec![Attribute {
                key: "test_key",
                value: "test_value".into(),
            }],
        );

        let azure_context = AzureContext::new();
        let azure_context = azure_context.with_value(span.clone());

        let _guard = span.set_current(&azure_context).unwrap();

        let result = future.await;

        assert_eq!(result, 42);
        span.end();

        let spans = otel_exporter.get_finished_spans().unwrap();
        assert_eq!(spans.len(), 1);
        for span in &spans {
            trace!("Span: {:?}", span);
            assert_eq!(span.name, "test_span");
            assert_eq!(span.events.len(), 1);
            assert_eq!(span.attributes.len(), 2);
            assert_eq!(span.attributes[0].key, "test_key".into());
            assert_eq!(
                format!("{:?}", span.attributes[0].value),
                "String(Owned(\"test_value\"))"
            );
        }
    }
}
