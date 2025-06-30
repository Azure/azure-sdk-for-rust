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
    fn end(&self) -> Result<()> {
        self.context.span().end();
        Ok(())
    }

    fn span_id(&self) -> [u8; 8] {
        self.context.span().span_context().span_id().to_bytes()
    }

    fn set_attribute(&self, key: &'static str, value: AttributeValue) -> Result<()> {
        let otel_value = opentelemetry::Value::from(ConversionAttributeValue(value));
        self.context
            .span()
            .set_attribute(opentelemetry::KeyValue::new(key, otel_value));
        Ok(())
    }

    fn record_error(&self, error: &dyn StdError) -> Result<()> {
        self.context.span().record_error(error);

        self.context
            .span()
            .set_status(opentelemetry::trace::Status::error(error.to_string()));
        Ok(())
    }

    fn set_status(&self, status: SpanStatus) -> Result<()> {
        let otel_status = match status {
            SpanStatus::Unset => opentelemetry::trace::Status::Unset,
            SpanStatus::Ok => opentelemetry::trace::Status::Ok,
            SpanStatus::Error { description } => opentelemetry::trace::Status::error(description),
        };
        self.context.span().set_status(otel_status);
        Ok(())
    }

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
    use azure_core::tracing::{AttributeValue, SpanKind, SpanStatus, TracerProvider};
    use opentelemetry::trace::TraceContextExt;
    use opentelemetry::{Context, Key, KeyValue, Value};
    use opentelemetry_sdk::trace::{in_memory_exporter::InMemorySpanExporter, SdkTracerProvider};
    use std::io::{Error, ErrorKind};
    use std::sync::Arc;

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
        let tracer = tracer_provider.unwrap().get_tracer("test", "0.1.0");
        let span = tracer.start_span("test_span", SpanKind::Client).unwrap();
        assert!(span.end().is_ok());

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
        let tracer = tracer_provider.unwrap().get_tracer("test", "0.1.0");
        let parent_span = tracer.start_span("parent_span", SpanKind::Server).unwrap();
        let child_span = tracer
            .start_span_with_parent("child_span", SpanKind::Client, parent_span.clone())
            .unwrap();

        assert!(child_span.end().is_ok());
        assert!(parent_span.end().is_ok());

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
        let tracer = tracer_provider.unwrap().get_tracer("test", "0.1.0");
        let span1 = tracer.start_span("span1", SpanKind::Internal).unwrap();
        let span2 = tracer.start_span("span2", SpanKind::Server).unwrap();
        let child_span = tracer
            .start_span_with_parent("child_span", SpanKind::Client, span1.clone())
            .unwrap();

        assert!(child_span.end().is_ok());
        assert!(span2.end().is_ok());
        assert!(span1.end().is_ok());

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
        let tracer = tracer_provider.unwrap().get_tracer("test", "0.1.0");
        let span1 = tracer.start_span("span1", SpanKind::Internal).unwrap();
        let span2 = tracer.start_span("span2", SpanKind::Server).unwrap();
        let _span_guard = span2
            .set_current(&azure_core::http::Context::new())
            .unwrap();
        let child_span = tracer
            .start_span_with_current("child_span", SpanKind::Client)
            .unwrap();

        assert!(child_span.end().is_ok());
        assert!(span2.end().is_ok());
        assert!(span1.end().is_ok());

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
        let tracer = tracer_provider.unwrap().get_tracer("test", "0.1.0");
        let span = tracer.start_span("test_span", SpanKind::Internal).unwrap();

        assert!(span
            .set_attribute("test_key", AttributeValue::String("test_value".to_string()))
            .is_ok());
        assert!(span.end().is_ok());

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
        let tracer = tracer_provider.unwrap().get_tracer("test", "0.1.0");
        let span = tracer.start_span("test_span", SpanKind::Client).unwrap();

        let error = Error::new(ErrorKind::NotFound, "resource not found");
        assert!(span.record_error(&error).is_ok());
        assert!(span.end().is_ok());

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
        let tracer = tracer_provider.unwrap().get_tracer("test", "0.1.0");

        // Test Ok status
        let span = tracer.start_span("test_span_ok", SpanKind::Server).unwrap();
        assert!(span.set_status(SpanStatus::Ok).is_ok());
        assert!(span.end().is_ok());

        // Test Error status
        let span = tracer
            .start_span("test_span_error", SpanKind::Server)
            .unwrap();
        assert!(span
            .set_status(SpanStatus::Error {
                description: "test error".to_string()
            })
            .is_ok());
        assert!(span.end().is_ok());

        let spans = otel_exporter.get_finished_spans().unwrap();
        assert_eq!(spans.len(), 2);

        let ok_span = spans.iter().find(|s| s.name == "test_span_ok").unwrap();
        assert_eq!(ok_span.status, opentelemetry::trace::Status::Ok);

        let error_span = spans.iter().find(|s| s.name == "test_span_error").unwrap();
        assert_eq!(
            error_span.status,
            opentelemetry::trace::Status::error("test error")
        );
    }

    #[tokio::test]
    async fn test_open_telemetry_span_futures() {
        let (otel_tracer_provider, otel_exporter) = create_exportable_tracer_provider();
        let tracer_provider = OpenTelemetryTracerProvider::new(otel_tracer_provider);
        assert!(tracer_provider.is_ok());
        let tracer = tracer_provider.unwrap().get_tracer("test", "0.1.0");

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

        let span = tracer.start_span("test_span", SpanKind::Client).unwrap();

        let azure_context = AzureContext::new();
        let azure_context = azure_context.with_value(span.clone());

        let _guard = span.set_current(&azure_context).unwrap();

        let result = future.await;

        assert_eq!(result, 42);
        span.end().unwrap();

        let spans = otel_exporter.get_finished_spans().unwrap();
        assert_eq!(spans.len(), 1);
        for span in &spans {
            assert_eq!(span.name, "test_span");
            assert_eq!(span.events.len(), 1);
            assert_eq!(span.attributes.len(), 1);
            assert_eq!(span.attributes[0].key, "test_key".into());
            assert_eq!(
                format!("{:?}", span.attributes[0].value),
                "String(Static(\"test_value\"))"
            );
        }
    }
}
