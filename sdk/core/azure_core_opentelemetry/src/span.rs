// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! OpenTelemetry implementation of typespec_client_core tracing traits.

use crate::attributes::AttributeValue as ConversionAttributeValue;
use azure_core::{
    http::headers::{HeaderName, HeaderValue},
    tracing::{AsAny, AttributeValue, Span, SpanGuard, SpanStatus},
};
use opentelemetry::{
    propagation::{Injector, TextMapPropagator},
    trace::TraceContextExt,
};
use opentelemetry_sdk::propagation::TraceContextPropagator;
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

struct HeaderMap(Vec<(HeaderName, HeaderValue)>);

impl Injector for HeaderMap {
    fn set(&mut self, key: &str, value: String) {
        // Convert the key and value to HeaderName and HeaderValue. Note that we need to
        // allocate new strings for the key because it's passed in as a string slice reference.
        self.0.push((key.to_owned().into(), value.into()));
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

    fn propagate_headers(&self, request: &mut azure_core::http::Request) {
        // A TraceContextPropagator is used to inject trace context information into HTTP headers.
        let trace_propagator = TraceContextPropagator::new();
        // We need to map between a header map (which is what the OpenTelemetry SDK requires)
        // and the Azure Core request headers.
        //
        // We start with an empty header map and inject the OpenTelemetry headers into it.
        let mut header_map = HeaderMap(Vec::new());
        trace_propagator.inject_context(&self.context, &mut header_map);

        // We then insert each of the headers from the OpenTelemetry header map into the
        // Request's header map.
        for (key, value) in header_map.0 {
            request.insert_header(
                key,
                // The value is guaranteed to be a valid UTF-8 string by the OpenTelemetry SDK,
                // so we can safely unwrap it.
                value,
            );
        }
    }

    fn set_current(&self, _context: &azure_core::http::Context) -> Box<dyn SpanGuard> {
        // Create a context with the current span
        let context_guard = self.context.clone().attach();

        Box::new(OpenTelemetrySpanGuard {
            _inner: context_guard,
        })
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
    fn end(self) {
        // The span is ended when the guard is dropped, so no action needed here.
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
    use azure_core::http::{Context as AzureContext, Url};
    use azure_core::tracing::{AttributeValue, SpanKind, SpanStatus, TracerProvider};
    use opentelemetry::trace::TraceContextExt;
    use opentelemetry::Context;
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
        let tracer =
            tracer_provider.get_tracer(Some("Microsoft.SpecialCase"), "test", Some("0.1.0"));
        let span = tracer.start_span("test_span".into(), SpanKind::Client, vec![]);
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

    // cspell: ignore traceparent tracestate
    #[test]
    fn test_open_telemetry_span_propagate() {
        let (otel_tracer_provider, otel_exporter) = create_exportable_tracer_provider();

        let tracer_provider = OpenTelemetryTracerProvider::new(otel_tracer_provider);
        let tracer = tracer_provider.get_tracer(Some("Microsoft.SpecialCase"), "test", None);
        let span = tracer.start_span("test_span".into(), SpanKind::Client, vec![]);
        let mut request = azure_core::http::Request::new(
            Url::parse("http://example.com").unwrap(),
            azure_core::http::Method::Get,
        );
        span.propagate_headers(&mut request);
        trace!("Request headers after propagation: {:?}", request.headers());
        let traceparent = azure_core::http::headers::HeaderName::from("traceparent");
        let tracestate = azure_core::http::headers::HeaderName::from("tracestate");
        request.headers().get_as::<String, _>(&traceparent).unwrap();
        request.headers().get_as::<String, _>(&tracestate).unwrap();
        span.end();

        let finished_spans = otel_exporter.get_finished_spans().unwrap();
        assert_eq!(finished_spans.len(), 1);
    }

    #[test]
    fn test_open_telemetry_span_hierarchy() {
        let (otel_tracer_provider, otel_exporter) = create_exportable_tracer_provider();
        let tracer_provider = OpenTelemetryTracerProvider::new(otel_tracer_provider);
        let tracer = tracer_provider.get_tracer(Some("Special Name"), "test", Some("0.1.0"));
        let parent_span = tracer.start_span("parent_span".into(), SpanKind::Server, vec![]);
        let child_span = tracer.start_span_with_parent(
            "child_span".into(),
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
        let tracer = tracer_provider.get_tracer(Some("MyNamespace"), "test", Some("0.1.0"));
        let span1 = tracer.start_span("span1".into(), SpanKind::Internal, vec![]);
        let span2 = tracer.start_span("span2".into(), SpanKind::Server, vec![]);
        let child_span = tracer.start_span_with_parent(
            "child_span".into(),
            SpanKind::Client,
            vec![],
            span1.clone(),
        );

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
        let tracer = tracer_provider.get_tracer(Some("Namespace"), "test", Some("0.1.0"));
        let span1 = tracer.start_span("span1".into(), SpanKind::Internal, vec![]);
        let span2 = tracer.start_span("span2".into(), SpanKind::Server, vec![]);
        let _span_guard = span2.set_current(&azure_core::http::Context::new());
        let child_span = tracer.start_span("child_span".into(), SpanKind::Client, vec![]);

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
        let tracer = tracer_provider.get_tracer(Some("ThisNamespace"), "test", Some("0.1.0"));
        let span = tracer.start_span("test_span".into(), SpanKind::Internal, vec![]);

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
        let tracer = tracer_provider.get_tracer(Some("namespace"), "test", Some("0.1.0"));
        let span = tracer.start_span("test_span".into(), SpanKind::Client, vec![]);

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
        let tracer = tracer_provider.get_tracer(Some("Namespace"), "test", Some("0.1.0"));

        // Test Unset status
        let span = tracer.start_span("test_span_unset".into(), SpanKind::Server, vec![]);
        span.end();

        // Test Error status
        let span = tracer.start_span("test_span_error".into(), SpanKind::Server, vec![]);
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

    /// Tests span context propagation in three async scenarios to expose a fundamental
    /// limitation of thread-local OTel context in a multi-threaded async runtime.
    ///
    /// **Scenario 1 (sync, no yield):** Starting a child span on the same thread as the
    /// parent works because `Context::current()` reads from thread-local storage, which
    /// holds the parent context. ✅ Passes.
    ///
    /// **Scenario 2 (async, no yield):** An async closure that never yields also works
    /// because the Tokio scheduler never migrates the task to a different thread. ✅ Passes.
    ///
    /// **Scenario 3 (async with yield, I/O completion on a separate OS thread):** This
    /// models real async I/O — the operation starts on one thread, yields, and is completed
    /// (i.e., resumed) on a **different OS thread** (e.g., an epoll/kqueue completion
    /// handler, a thread-pool worker, or a kernel I/O completion port). The span context
    /// set on the original thread is invisible on the I/O completion thread because
    /// [`opentelemetry::Context::current`] uses `thread_local!` storage that is not
    /// propagated across OS thread boundaries.
    ///
    /// Scenario 3 **panics** (caught by `#[should_panic]`) to prove the limitation:
    /// a span started before an `await` point cannot be automatically maintained across
    /// the entirety of an async operation that completes on a different thread.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_open_telemetry_span_futures() {
        let (otel_tracer_provider, otel_exporter) = create_exportable_tracer_provider();
        let tracer_provider = OpenTelemetryTracerProvider::new(otel_tracer_provider);
        let tracer = tracer_provider.get_tracer(Some("Namespace"), "test", Some("0.1.0"));

        let azure_context = AzureContext::new();
        let parent_span = tracer.start_span("parent_span".into(), SpanKind::Server, vec![]);
        // Attach the parent span to the current thread's OTel context.
        let _parent_guard = parent_span.set_current(&azure_context);

        // --- Scenario 1: Synchronous, no yield ---
        // Context::current() reads the thread-local holding the parent, so the child
        // span is correctly parented.
        {
            let child = tracer.start_span("sync_child".into(), SpanKind::Internal, vec![]);
            child.end();
        }

        // --- Scenario 2: Async closure, no yield ---
        // The closure runs entirely on the same thread, so the parent is still in
        // thread-local storage when start_span is called.
        {
            let tracer = Arc::clone(&tracer);
            async move {
                let child =
                    tracer.start_span("async_no_yield_child".into(), SpanKind::Internal, vec![]);
                child.end();
            }
            .await;
        }

        // Verify scenarios 1 and 2: each child span is recorded with the correct parent.
        {
            let finished = otel_exporter.get_finished_spans().unwrap();
            let parent_id =
                opentelemetry::trace::SpanId::from_bytes(parent_span.span_id());

            let sync_child = finished.iter().find(|s| s.name == "sync_child").unwrap();
            assert_eq!(
                sync_child.parent_span_id, parent_id,
                "Scenario 1: sync child span must have the parent as its parent"
            );

            let async_child = finished
                .iter()
                .find(|s| s.name == "async_no_yield_child")
                .unwrap();
            assert_eq!(
                async_child.parent_span_id, parent_id,
                "Scenario 2: async no-yield child span must have the parent as its parent"
            );
        }

        // --- Scenario 3: Async closure that yields and is completed on a different OS thread ---
        //
        // This models real async I/O (e.g., a file read or a socket write):
        //   1. The task initiates the I/O and yields (`await`-ing the result).
        //   2. The OS / runtime signals I/O completion on whatever thread it deems
        //      appropriate — this is frequently a different OS thread from the one
        //      that started the operation.
        //   3. The task resumes (or its I/O-completion code runs) on that thread.
        //
        // We model step 2 with `std::thread::spawn`, which is guaranteed to create a
        // fresh OS thread. This is the mechanism used by platforms such as Windows I/O
        // Completion Ports, Linux AIO, and Tokio's own blocking thread pool — all of
        // which deliver completions on threads that differ from the initiating thread.
        //
        // After the yield the async closure checks whether the span context is still
        // active on the I/O completion thread. The assertion PANICS because the
        // thread-local OTel context set on the original thread is invisible there.
        {
            let tracer = Arc::clone(&tracer);
            let test_start_thread = std::thread::current().id();

            async move {
                // On the original thread: start a child span. Because we are still on
                // the same thread as _parent_guard, Context::current() sees the parent
                // and the new span is correctly linked to it.
                let child =
                    tracer.start_span("async_yield_child".into(), SpanKind::Internal, vec![]);
                let _child_guard = child.set_current(&AzureContext::new());

                // Yield the async task while the "I/O operation" runs on a separate OS
                // thread. This is the suspend point that models a real await on I/O.
                //
                // `std::thread::spawn` is guaranteed to create a new OS thread — not a
                // Tokio worker reuse — so `io_completion_thread != test_start_thread`
                // is unconditionally true. The spawned thread represents the I/O
                // completion handler that in real systems would be invoked by the kernel
                // or by the async runtime's I/O driver on a thread of its choosing.
                let (tx, rx) =
                    tokio::sync::oneshot::channel::<(std::thread::ThreadId, bool)>();
                std::thread::spawn(move || {
                    let io_completion_thread = std::thread::current().id();
                    // Check whether the span context set on the Tokio worker thread
                    // (the one that initiated the "I/O") is visible here.
                    let context_active = Context::current().span().is_recording();
                    let _ = tx.send((io_completion_thread, context_active));
                });

                // Await the completion: the async task is suspended here (yielded to
                // the executor) and resumes once the OS thread sends on the channel.
                let (io_completion_thread, context_active_on_io_thread) = rx.await.unwrap();

                // Prove that the I/O completion definitively ran on a different OS thread.
                // `std::thread::spawn` always creates a fresh thread, so this always holds.
                assert_ne!(
                    io_completion_thread,
                    test_start_thread,
                    "I/O completion must run on a different OS thread than the one that started the operation"
                );

                // PROVE THE LIMITATION: the span context is NOT visible on the I/O
                // completion thread. Any code that tries to record events, set attributes,
                // or start child spans via Context::current() after this yield point will
                // silently see an empty (no-op) context, effectively losing the span.
                //
                // THIS ASSERTION PANICS, demonstrating that the built-in tracing
                // abstractions provide no way to extend a span across the entirety of an
                // operation that delegates to a different OS thread.
                assert!(
                    context_active_on_io_thread,
                    "span context should be active on the I/O completion thread"
                );
            }
            .await;
        }
    }
}
