// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell: ignore traceparent
use std::sync::{Arc, Mutex};
use tracing::trace;
use typespec_client_core::{
    http::{headers::HeaderName, Context, Request},
    tracing::{
        AsAny, Attribute, AttributeValue, Span, SpanKind, SpanStatus, Tracer, TracerProvider,
    },
};

#[derive(Debug)]
pub struct MockTracingProvider {
    tracers: Mutex<Vec<Arc<MockTracer>>>,
}

impl MockTracingProvider {
    pub fn new() -> Self {
        Self {
            tracers: Mutex::new(Vec::new()),
        }
    }
}

impl Default for MockTracingProvider {
    fn default() -> Self {
        Self::new()
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
pub struct MockTracer {
    pub namespace: Option<&'static str>,
    pub package_name: &'static str,
    pub package_version: &'static str,
    pub spans: Mutex<Vec<Arc<MockSpan>>>,
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

    fn start_span(&self, name: &str, kind: SpanKind, attributes: Vec<Attribute>) -> Arc<dyn Span> {
        let span = Arc::new(MockSpan::new(name, kind, attributes));
        self.spans.lock().unwrap().push(span.clone());
        span
    }
}

#[derive(Debug)]
pub struct MockSpan {
    pub name: String,
    pub kind: SpanKind,
    pub attributes: Mutex<Vec<Attribute>>,
    pub state: Mutex<SpanStatus>,
    pub is_open: Mutex<bool>,
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

    fn set_current(&self, _context: &Context) -> Box<dyn typespec_client_core::tracing::SpanGuard> {
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

#[derive(Debug)]
pub struct ExpectedTracerInformation<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub namespace: Option<&'a str>,
    pub spans: Vec<ExpectedSpanInformation<'a>>,
}

#[derive(Debug)]
pub struct ExpectedSpanInformation<'a> {
    pub span_name: &'a str,
    pub status: SpanStatus,
    pub kind: SpanKind,
    pub attributes: Vec<(&'a str, AttributeValue)>,
}

pub fn check_instrumentation_result(
    mock_tracer: Arc<MockTracingProvider>,
    expected_tracers: Vec<ExpectedTracerInformation<'_>>,
) {
    assert_eq!(
        mock_tracer.tracers.lock().unwrap().len(),
        expected_tracers.len(),
        "Unexpected number of tracers",
    );
    let tracers = mock_tracer.tracers.lock().unwrap();
    for (index, expectation) in expected_tracers.iter().enumerate() {
        trace!("Checking tracer {}: {}", index, expectation.name);
        let tracer = &tracers[index];
        assert_eq!(tracer.package_name, expectation.name);
        assert_eq!(tracer.package_version, expectation.version);
        assert_eq!(tracer.namespace, expectation.namespace);

        let spans = tracer.spans.lock().unwrap();
        assert_eq!(
            spans.len(),
            expectation.spans.len(),
            "Unexpected number of spans for tracer {}",
            expectation.name
        );

        for (span_index, span_expectation) in expectation.spans.iter().enumerate() {
            println!(
                "Checking span {} of tracer {}: {}",
                span_index, expectation.name, span_expectation.span_name
            );
            check_span_information(&spans[span_index], span_expectation);
        }
    }
}

fn check_span_information(span: &Arc<MockSpan>, expectation: &ExpectedSpanInformation<'_>) {
    assert_eq!(span.name, expectation.span_name);
    assert_eq!(span.kind, expectation.kind);
    assert_eq!(*span.state.lock().unwrap(), expectation.status);
    let attributes = span.attributes.lock().unwrap();
    for (index, attr) in attributes.iter().enumerate() {
        println!("Attribute {}: {} = {:?}", index, attr.key, attr.value);
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
