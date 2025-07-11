// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Instrumentation pipeline policies.

mod public_api_instrumentation;
mod request_instrumentation;

// Distributed tracing span attribute names. Defined in
// [OpenTelemetrySpans](https://github.com/open-telemetry/semantic-conventions/blob/main/docs/http/http-spans.md)
// and [Azure conventions for open telemetry spans](https://github.com/Azure/azure-sdk/blob/main/docs/tracing/distributed-tracing-conventions.md)
const AZ_NAMESPACE_ATTRIBUTE: &str = "az.namespace";
const AZ_CLIENT_REQUEST_ID_ATTRIBUTE: &str = "az.client.request.id";
const ERROR_TYPE_ATTRIBUTE: &str = "error.type";
const AZ_SERVICE_REQUEST_ID_ATTRIBUTE: &str = "az.service_request.id";
const HTTP_REQUEST_RESEND_COUNT_ATTRIBUTE: &str = "http.request.resend_count";
const HTTP_RESPONSE_STATUS_CODE_ATTRIBUTE: &str = "http.response.status_code";
const HTTP_REQUEST_METHOD_ATTRIBUTE: &str = "http.request.method";
const SERVER_ADDRESS_ATTRIBUTE: &str = "server.address";
const SERVER_PORT_ATTRIBUTE: &str = "server.port";
const URL_FULL_ATTRIBUTE: &str = "url.full";

pub use public_api_instrumentation::PublicApiInstrumentationInformation;
pub(crate) use public_api_instrumentation::PublicApiInstrumentationPolicy;
pub(crate) use request_instrumentation::*;

#[cfg(test)]
mod tests {
    // cspell: ignore traceparent
    use std::sync::{Arc, Mutex};
    use typespec_client_core::{
        http::{headers::HeaderName, Context, Request},
        tracing::{
            AsAny, Attribute, AttributeValue, Span, SpanKind, SpanStatus, Tracer, TracerProvider,
        },
    };

    #[derive(Debug)]
    pub(super) struct MockTracingProvider {
        pub(super) tracers: Mutex<Vec<Arc<MockTracer>>>,
    }

    impl MockTracingProvider {
        pub(super) fn new() -> Self {
            Self {
                tracers: Mutex::new(Vec::new()),
            }
        }
    }
    impl TracerProvider for MockTracingProvider {
        fn get_tracer(
            &self,
            azure_namespace: Option<&'static str>,
            crate_name: &'static str,
            crate_version: &'static str,
        ) -> Arc<dyn crate::tracing::Tracer> {
            let mut tracers = self.tracers.lock().unwrap();
            let tracer = Arc::new(MockTracer {
                namespace: azure_namespace,
                package_name: crate_name,
                package_version: crate_version,
                spans: Mutex::new(Vec::new()),
            });

            tracers.push(tracer.clone());
            tracer
        }
    }

    #[derive(Debug)]
    pub(super) struct MockTracer {
        pub(super) namespace: Option<&'static str>,
        pub(super) package_name: &'static str,
        pub(super) package_version: &'static str,
        pub(super) spans: Mutex<Vec<Arc<MockSpan>>>,
    }

    impl Tracer for MockTracer {
        fn namespace(&self) -> Option<&'static str> {
            self.namespace
        }

        fn start_span_with_current(
            &self,
            name: &str,
            kind: SpanKind,
            attributes: Vec<Attribute>,
        ) -> Arc<dyn crate::tracing::Span> {
            let span = Arc::new(MockSpan::new(name, kind, attributes));
            self.spans.lock().unwrap().push(span.clone());
            span
        }

        fn start_span_with_parent(
            &self,
            name: &str,
            kind: SpanKind,
            attributes: Vec<Attribute>,
            _parent: Arc<dyn crate::tracing::Span>,
        ) -> Arc<dyn crate::tracing::Span> {
            let span = Arc::new(MockSpan::new(name, kind, attributes));
            self.spans.lock().unwrap().push(span.clone());
            span
        }

        fn start_span(
            &self,
            name: &str,
            kind: SpanKind,
            attributes: Vec<Attribute>,
        ) -> Arc<dyn Span> {
            let span = Arc::new(MockSpan::new(name, kind, attributes));
            self.spans.lock().unwrap().push(span.clone());
            span
        }
    }

    #[derive(Debug)]
    pub(super) struct MockSpan {
        pub(super) name: String,
        pub(super) kind: SpanKind,
        pub(super) attributes: Mutex<Vec<Attribute>>,
        pub(super) state: Mutex<SpanStatus>,
        pub(super) is_open: Mutex<bool>,
    }
    impl MockSpan {
        fn new(name: &str, kind: SpanKind, attributes: Vec<Attribute>) -> Self {
            println!("Creating MockSpan: {}", name);
            println!("Attributes: {:?}", attributes);
            Self {
                name: name.to_string(),
                kind,
                attributes: Mutex::new(attributes),
                state: Mutex::new(SpanStatus::Unset),
                is_open: Mutex::new(true),
            }
        }
    }

    impl Span for MockSpan {
        fn set_attribute(&self, key: &'static str, value: AttributeValue) {
            println!("{}: Setting attribute {}: {:?}", self.name, key, value);
            let mut attributes = self.attributes.lock().unwrap();
            attributes.push(Attribute { key, value });
        }

        fn set_status(&self, status: crate::tracing::SpanStatus) {
            println!("{}: Setting span status: {:?}", self.name, status);
            let mut state = self.state.lock().unwrap();
            *state = status;
        }

        fn end(&self) {
            println!("Ending span: {}", self.name);
            let mut is_open = self.is_open.lock().unwrap();
            *is_open = false;
        }

        fn is_recording(&self) -> bool {
            true
        }

        fn span_id(&self) -> [u8; 8] {
            [0; 8] // Mock span ID
        }

        fn record_error(&self, _error: &dyn std::error::Error) {
            todo!()
        }

        fn set_current(
            &self,
            _context: &Context,
        ) -> Box<dyn typespec_client_core::tracing::SpanGuard> {
            todo!()
        }

        /// Insert two dummy headers for distributed tracing.
        // cspell: ignore traceparent tracestate
        fn propagate_headers(&self, request: &mut Request) {
            request.insert_header(
                HeaderName::from_static("traceparent"),
                "00-<trace_id>-<span_id>-01",
            );
            request.insert_header(HeaderName::from_static("tracestate"), "<key>=<value>");
        }
    }

    impl AsAny for MockSpan {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    pub(super) struct InstrumentationExpectation<'a> {
        pub(super) namespace: Option<&'a str>,
        pub(super) name: &'a str,
        pub(super) version: &'a str,
        pub(super) span_name: &'a str,
        pub(super) status: SpanStatus,
        pub(super) kind: SpanKind,
        pub(super) attributes: Vec<(&'a str, AttributeValue)>,
    }
    pub(super) fn check_request_instrumentation_result(
        mock_tracer: Arc<MockTracingProvider>,
        expected_span_count: usize,
        span_index: usize,
        expectation: InstrumentationExpectation,
    ) {
        assert_eq!(
            mock_tracer.tracers.lock().unwrap().len(),
            1,
            "Expected one tracer to be created",
        );
        let tracers = mock_tracer.tracers.lock().unwrap();
        let tracer = tracers.first().unwrap();
        assert_eq!(tracer.package_name, expectation.name);
        assert_eq!(tracer.package_version, expectation.version);
        assert_eq!(tracer.namespace, expectation.namespace);
        let spans = tracer.spans.lock().unwrap();
        assert_eq!(
            spans.len(),
            expected_span_count,
            "Expected one span to be created"
        );
        println!("Spans: {:?}", spans);
        let span = spans[span_index].as_ref();
        assert_eq!(span.name, expectation.span_name);
        assert_eq!(span.kind, expectation.kind);
        assert_eq!(*span.state.lock().unwrap(), expectation.status);
        let attributes = span.attributes.lock().unwrap();
        for attr in attributes.iter() {
            println!("Attribute: {} = {:?}", attr.key, attr.value);
            let mut found = false;
            for (key, value) in &expectation.attributes {
                if attr.key == *key {
                    assert_eq!(attr.value, *value, "Attribute mismatch for key: {}", key);
                    found = true;
                    break;
                }
            }
            if !found {
                panic!("Unexpected attribute: {} = {:?}", attr.key, attr.value);
            }
        }
        for (key, value) in &expectation.attributes {
            if !attributes
                .iter()
                .any(|attr| attr.key == *key && attr.value == *value)
            {
                panic!("Expected attribute not found: {} = {:?}", key, value);
            }
        }
    }
}
