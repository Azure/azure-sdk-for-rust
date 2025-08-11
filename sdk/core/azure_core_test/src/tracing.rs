// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell: ignore traceparent
use std::{
    pin::Pin,
    sync::{Arc, Mutex},
};
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
        crate_version: Option<&'static str>,
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
    pub package_version: Option<&'static str>,
    pub spans: Mutex<Vec<Arc<MockSpan>>>,
}

impl Tracer for MockTracer {
    fn namespace(&self) -> Option<&'static str> {
        self.namespace
    }

    fn start_span_with_parent(
        &self,
        name: &str,
        kind: SpanKind,
        attributes: Vec<Attribute>,
        _parent: Arc<dyn crate::tracing::Span>,
    ) -> Arc<dyn crate::tracing::Span> {
        let span = Arc::new(MockSpan::new(name, kind, attributes.clone()));
        self.spans.lock().unwrap().push(span.clone());
        span
    }

    fn start_span(
        &self,
        name: &'static str,
        kind: SpanKind,
        attributes: Vec<Attribute>,
    ) -> Arc<dyn Span> {
        let attributes = attributes
            .into_iter()
            .map(|attr| Attribute {
                key: attr.key.clone(),
                value: attr.value.clone(),
            })
            .collect();
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
        println!("Converted attributes: {:?}", attributes);
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
        attributes.push(Attribute {
            key: key.into(),
            value,
        });
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
        // Convert to an object that doesn't expose the lifetime parameter
        // We're essentially erasing the lifetime here to satisfy the static requirement
        self as &dyn std::any::Any
    }
}

/// Expected information about a tracer.
#[derive(Debug)]
pub struct ExpectedTracerInformation<'a> {
    /// Expected name for the tracer.
    pub name: &'a str,

    /// Expected version for the tracer.
    pub version: Option<&'a str>,

    /// Expected namespace for the tracer.
    pub namespace: Option<&'a str>,

    /// A set of spans which should have been generated.
    pub spans: Vec<ExpectedSpanInformation<'a>>,
}

pub fn check_instrumentation_result(
    mock_tracer: Arc<MockTracingProvider>,
    expected_tracers: Vec<ExpectedTracerInformation<'_>>,
) {
    let tracers = mock_tracer.tracers.lock().unwrap();
    if tracers.len() != expected_tracers.len() {
        println!("Expected tracers: {:?}", expected_tracers);
        println!("Found tracers: {:?}", tracers);
    }
    assert_eq!(
        tracers.len(),
        expected_tracers.len(),
        "Unexpected number of tracers, expected: {}, found: {}",
        expected_tracers.len(),
        tracers.len()
    );
    for (index, expected) in expected_tracers.iter().enumerate() {
        println!("Checking tracer {}: {}", index, expected.name);
        let tracer = &tracers[index];
        assert_eq!(tracer.package_name, expected.name);
        assert_eq!(tracer.package_version, expected.version);
        assert_eq!(tracer.namespace, expected.namespace);

        let spans = tracer.spans.lock().unwrap();
        assert_eq!(
            spans.len(),
            expected.spans.len(),
            "Unexpected number of spans for tracer {}",
            expected.name
        );

        for (span_index, span_expected) in expected.spans.iter().enumerate() {
            println!(
                "Checking span {} of tracer {}: {}",
                span_index, expected.name, span_expected.span_name
            );
            check_span_information(&spans[span_index], span_expected);
        }
    }
}

#[derive(Debug)]
pub struct ExpectedSpanInformation<'a> {
    pub span_name: &'a str,
    pub status: SpanStatus,
    pub kind: SpanKind,
    pub attributes: Vec<(&'a str, AttributeValue)>,
}

fn check_span_information(span: &Arc<MockSpan>, expected: &ExpectedSpanInformation<'_>) {
    assert_eq!(span.name, expected.span_name);
    assert_eq!(span.kind, expected.kind);
    assert_eq!(*span.state.lock().unwrap(), expected.status);
    let attributes = span.attributes.lock().unwrap();
    println!("Expected attributes: {:?}", expected.attributes);
    println!("Found attributes: {:?}", attributes);
    for (index, attr) in attributes.iter().enumerate() {
        println!("Attribute {}: {} = {:?}", index, attr.key, attr.value);
        let mut found = false;
        for (key, value) in &expected.attributes {
            if attr.key == *key {
                // Skip checking the value for "<ANY>" as it is a placeholder
                if *value != AttributeValue::String("<ANY>".into()) {
                    assert_eq!(attr.value, *value, "Attribute mismatch for key: {}", *key);
                }
                found = true;
                break;
            }
        }
        if !found {
            panic!("Unexpected attribute: {} = {:?}", attr.key, attr.value);
        }
    }
    for (key, value) in expected.attributes.iter() {
        if !attributes.iter().any(|attr| attr.key == *key) {
            panic!("Expected attribute not found: {} = {:?}", key, value);
        }
    }
}

#[derive(Debug, Clone)]
pub struct InstrumentedApiInformation {
    pub api_name: Option<&'static str>,
    pub api_verb: azure_core::http::Method,
    pub expected_status_code: azure_core::http::StatusCode,
    pub additional_api_attributes: Vec<(&'static str, AttributeValue)>,
}

impl Default for InstrumentedApiInformation {
    fn default() -> Self {
        Self {
            api_name: None,
            api_verb: azure_core::http::Method::Get,
            expected_status_code: azure_core::http::StatusCode::Ok,
            additional_api_attributes: Vec::new(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct InstrumentationInformation {
    pub package_name: String,
    pub package_version: String,
    pub package_namespace: Option<&'static str>,
    pub api_calls: Vec<InstrumentedApiInformation>,
}

pub async fn test_instrumentation_for_api<C, FnInit, FnTest, T>(
    create_client: FnInit,
    test_api: FnTest,
    api_information: InstrumentationInformation,
) -> azure_core::Result<()>
where
    FnInit: FnOnce(Arc<dyn TracerProvider>) -> azure_core::Result<C>,
    FnTest: FnOnce(C) -> Pin<Box<dyn futures::Future<Output = azure_core::Result<T>>>>,
{
    // Initialize the mock tracer provider
    let mock_tracer = Arc::new(MockTracingProvider::new());

    // Create a client with the mock tracer
    let client = create_client(mock_tracer.clone())?;

    let result = test_api(client).await;
    trace!("Generated traces: {:?}", mock_tracer);

    let mut public_api_tracer = ExpectedTracerInformation {
        name: api_information.package_name.as_str(),
        version: Some(api_information.package_version.as_str()),
        namespace: api_information.package_namespace,
        spans: Vec::new(),
    };

    let mut request_activity_tracer = ExpectedTracerInformation {
        name: api_information.package_name.as_str(),
        version: Some(api_information.package_version.as_str()),
        namespace: None,
        spans: Vec::new(),
    };

    for api_call in api_information.api_calls.iter() {
        let mut expected_spans = Vec::new();

        let mut public_api_attributes = api_call.additional_api_attributes.clone();
        // Add additional attributes as needed.
        if let Some(namespace) = api_information.package_namespace {
            public_api_attributes.push(("az.namespace", namespace.into()));
        }
        if !api_call.expected_status_code.is_success() {
            public_api_attributes.push((
                "error.type",
                api_call.expected_status_code.to_string().into(),
            ));
        }

        if let Some(api_name) = api_call.api_name {
            expected_spans.push(ExpectedSpanInformation {
                span_name: api_name,
                status: if api_call.expected_status_code.is_server_error() {
                    SpanStatus::Error {
                        description: "".into(),
                    }
                } else {
                    SpanStatus::Unset
                },
                kind: SpanKind::Internal,
                attributes: public_api_attributes.clone(),
            });
        }
        // Add the HTTP API span.
        let mut expected_http_attributes = vec![
            ("http.request.method", api_call.api_verb.as_str().into()),
            ("url.full", "<ANY>".into()),
            ("server.address", "<ANY>".into()),
            ("server.port", "<ANY>".into()),
            ("az.client_request_id", "<ANY>".into()),
            (
                "http.response.status_code",
                (*api_call.expected_status_code).into(),
            ),
        ];
        if !api_call.expected_status_code.is_success() {
            expected_http_attributes.push((
                "error.type",
                api_call.expected_status_code.to_string().into(),
            ));
        }
        // If we have no public API information, we won't have a namespace in the HTTP attributes.
        if api_call.api_name.is_some() && api_information.package_namespace.is_some() {
            expected_http_attributes.push((
                "az.namespace",
                api_information.package_namespace.unwrap().into(),
            ));
        }
        expected_spans.push(ExpectedSpanInformation {
            span_name: api_call.api_verb.as_str(),
            status: if api_call.expected_status_code != 200 {
                SpanStatus::Error {
                    description: "".into(),
                }
            } else {
                SpanStatus::Unset
            },
            kind: SpanKind::Client,
            attributes: expected_http_attributes,
        });
        if api_call.api_name.is_some() {
            public_api_tracer.spans.extend(expected_spans);
        } else {
            request_activity_tracer.spans.extend(expected_spans);
        }
    }
    let expected_tracers = vec![public_api_tracer, request_activity_tracer];

    match result {
        Ok(_) => {
            // If the test passes, we can check the instrumentation
            check_instrumentation_result(mock_tracer, expected_tracers);
        }
        Err(_) => {
            check_instrumentation_result(mock_tracer, expected_tracers);
        }
    }
    Ok(())
}
