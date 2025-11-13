// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell: ignore traceparent

//! This module contains a set of tests to help verify correctness of the Distributed Tracing implementation, and correctness of service client implementations of Distributed Tracing.

use azure_core::{
    http::{headers::HeaderName, Context, Request},
    tracing::{
        AsAny, Attribute, AttributeValue, Span, SpanGuard, SpanKind, SpanStatus, Tracer,
        TracerProvider,
    },
    Uuid,
};
use rand::{rng, Rng};
use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, Mutex},
};

/// Mock Tracing Provider - used for testing distributed tracing without involving a specific tracing implementation.
#[derive(Debug)]
pub struct MockTracingProvider {
    tracers: Mutex<Vec<Arc<MockTracer>>>,
}

impl MockTracingProvider {
    /// Instantiate a new instance of a Mock Tracing Provider.
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
/// Mock Tracer - used for testing distributed tracing without involving a specific tracing implementation.
#[derive(Debug)]
pub struct MockTracer {
    namespace: Option<&'static str>,
    package_name: &'static str,
    package_version: Option<&'static str>,
    spans: Mutex<Vec<Arc<MockSpanInner>>>,
}

impl Tracer for MockTracer {
    fn namespace(&self) -> Option<&'static str> {
        self.namespace
    }

    fn start_span_with_parent(
        &self,
        name: Cow<'static, str>,
        kind: SpanKind,
        attributes: Vec<Attribute>,
        parent: Arc<dyn crate::tracing::Span>,
    ) -> Arc<dyn crate::tracing::Span> {
        let span = Arc::new(MockSpanInner::new(
            name,
            kind,
            attributes.clone(),
            Some(parent),
        ));
        self.spans.lock().unwrap().push(span.clone());
        Arc::new(MockSpan { inner: span })
    }

    fn start_span(
        &self,
        name: Cow<'static, str>,
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
        let span = Arc::new(MockSpanInner::new(name, kind, attributes, None));
        self.spans.lock().unwrap().push(span.clone());
        Arc::new(MockSpan { inner: span })
    }
}

/// Mock span for testing purposes.
#[derive(Debug)]
struct MockSpanInner {
    pub name: Cow<'static, str>,
    pub kind: SpanKind,
    pub parent: Option<[u8; 8]>,
    pub id: [u8; 8],
    pub attributes: Mutex<Vec<Attribute>>,
    pub state: Mutex<SpanStatus>,
    pub is_open: Mutex<bool>,
}
impl MockSpanInner {
    fn new<C>(
        name: C,
        kind: SpanKind,
        attributes: Vec<Attribute>,
        parent: Option<Arc<dyn crate::tracing::Span>>,
    ) -> Self
    where
        C: Into<Cow<'static, str>> + Debug,
    {
        eprintln!("Creating MockSpan: {:?}", name);

        eprintln!("Attributes: {:?}", attributes);
        let id = rng().random();

        let parent = parent.map(|p| p.span_id());
        Self {
            name: name.into(),
            kind,
            parent,
            id,
            attributes: Mutex::new(attributes),
            state: Mutex::new(SpanStatus::Unset),
            is_open: Mutex::new(true),
        }
    }

    fn is_open(&self) -> bool {
        let is_open = self.is_open.lock().unwrap();
        *is_open
    }
}

impl AsAny for MockSpanInner {
    fn as_any(&self) -> &dyn std::any::Any {
        // Convert to an object that doesn't expose the lifetime parameter
        // We're essentially erasing the lifetime here to satisfy the static requirement
        self as &dyn std::any::Any
    }
}

impl Span for MockSpanInner {
    fn set_attribute(&self, key: &'static str, value: AttributeValue) {
        eprintln!("{}: Setting attribute {}: {:?}", self.name, key, value);
        let mut attributes = self.attributes.lock().unwrap();
        attributes.push(Attribute {
            key: key.into(),
            value,
        });
    }

    fn set_status(&self, status: crate::tracing::SpanStatus) {
        eprintln!("{}: Setting span status: {:?}", self.name, status);
        let mut state = self.state.lock().unwrap();
        *state = status;
    }

    fn end(&self) {
        eprintln!("Ending span: {}", self.name);
        let mut is_open = self.is_open.lock().unwrap();
        *is_open = false;
    }

    fn is_recording(&self) -> bool {
        true
    }

    fn span_id(&self) -> [u8; 8] {
        self.id
    }

    fn record_error(&self, _error: &dyn std::error::Error) {
        todo!()
    }

    fn set_current(&self, _context: &Context) -> Box<dyn SpanGuard> {
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

pub struct MockSpan {
    inner: Arc<MockSpanInner>,
}

impl Drop for MockSpan {
    fn drop(&mut self) {
        if self.inner.is_open() {
            eprintln!("Warning: Dropping open span: {}", self.inner.name);
            self.inner.end();
        }
    }
}

impl AsAny for MockSpan {
    fn as_any(&self) -> &dyn std::any::Any {
        // Convert to an object that doesn't expose the lifetime parameter
        // We're essentially erasing the lifetime here to satisfy the static requirement
        self as &dyn std::any::Any
    }
}

impl Span for MockSpan {
    fn set_attribute(&self, key: &'static str, value: AttributeValue) {
        self.inner.set_attribute(key, value);
    }

    fn set_status(&self, status: crate::tracing::SpanStatus) {
        self.inner.set_status(status);
    }

    fn end(&self) {
        self.inner.end();
    }

    fn is_recording(&self) -> bool {
        self.inner.is_recording()
    }

    fn span_id(&self) -> [u8; 8] {
        self.inner.span_id()
    }

    fn record_error(&self, error: &dyn std::error::Error) {
        self.inner.record_error(error);
    }

    fn set_current(&self, context: &Context) -> Box<dyn SpanGuard> {
        self.inner.set_current(context)
    }

    fn propagate_headers(&self, request: &mut Request) {
        self.inner.propagate_headers(request);
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

/// Checks the instrumentation result against the expected tracers.
///
/// Used to verify that the mock tracer has recorded the expected spans and attributes. Primarily
/// intended for use in unit tests of the distributed tracing functionality.
///
/// # Arguments
/// - `mock_tracer`: The mock tracer instance that contains the recorded spans.
/// - `expected_tracers`: The expected tracer information to compare against.
pub fn check_instrumentation_result(
    mock_tracer: Arc<MockTracingProvider>,
    expected_tracers: Vec<ExpectedTracerInformation<'_>>,
) {
    let tracers = mock_tracer.tracers.lock().unwrap();
    if tracers.len() != expected_tracers.len() {
        eprintln!("Expected tracers: {:?}", expected_tracers);
        eprintln!("Found tracers: {:?}", tracers);
    }
    assert_eq!(
        tracers.len(),
        expected_tracers.len(),
        "Unexpected number of tracers, expected: {}, found: {}",
        expected_tracers.len(),
        tracers.len()
    );
    for (index, expected) in expected_tracers.iter().enumerate() {
        eprintln!("Checking tracer {}: {}", index, expected.name);
        let tracer = &tracers[index];
        let mut parent_span_map = HashMap::new();
        assert_eq!(tracer.package_name, expected.name);
        assert_eq!(tracer.package_version, expected.version);
        assert_eq!(tracer.namespace, expected.namespace);

        let spans = tracer.spans.lock().unwrap();

        // Check span lengths if there are no wildcard spans.
        if !expected.spans.iter().any(|s| s.is_wildcard) {
            assert_eq!(
                spans.len(),
                expected.spans.len(),
                "Unexpected number of spans for tracer {}",
                expected.name
            );
        }

        let mut expected_index = 0;
        for (span_index, span_actual) in spans.iter().enumerate() {
            eprintln!(
                "Checking span {} of tracer {}: {}",
                span_index, expected.name, span_actual.name
            );
            check_span_information(
                span_actual,
                &expected.spans[expected_index],
                &parent_span_map,
            );
            // Now that we've verified the span, add the mapping between expected span ID and the actual span ID.
            parent_span_map.insert(expected.spans[expected_index].span_id, span_actual.id);
            if expected.spans[expected_index].is_wildcard {
                // If this is a wildcard span, we don't increment the expected index.
                eprintln!(
                    "Span {} is a wildcard, not incrementing expected index",
                    span_actual.name
                );
                if spans.len() > span_index + 1 {
                    let next_span = &spans[span_index + 1];
                    if !compare_span_information(
                        next_span,
                        &expected.spans[expected_index],
                        &parent_span_map,
                    ) {
                        eprintln!(
                            "Next actual span does not match expected span: {}",
                            expected.spans[expected_index].span_name
                        );
                        expected_index += 1;
                    }
                } else {
                    // At the very end, bump the expected index past the wildcard entry.
                    // This ensures that we consume all the expected spans.
                    expected_index += 1;
                }
            } else {
                expected_index += 1;
            }
        }
        assert_eq!(
            expected_index,
            expected.spans.len(),
            "Not all expected spans were found for tracer {}",
            expected.name
        );
    }
}

/// Information about an expected span. Used to assert span properties.
#[derive(Debug)]
pub struct ExpectedSpanInformation<'a> {
    /// The expected name of the span.
    pub span_name: &'a str,
    /// The expected status of the span.
    pub status: SpanStatus,

    /// The unique identifier for the span. Assigned when the span is created.
    pub span_id: Uuid,

    /// The expected parent span ID. When an expected span is a child of another span, this is set to the `span_id` of the parent span.
    pub parent_id: Option<Uuid>,

    /// The expected kind of the span.
    pub kind: SpanKind,

    /// The expected attributes associated with the span.
    pub attributes: Vec<(&'a str, AttributeValue)>,

    pub is_wildcard: bool,
}

impl Default for ExpectedSpanInformation<'_> {
    fn default() -> Self {
        Self {
            span_name: "get",
            status: SpanStatus::Unset,
            span_id: Uuid::new_v4(),
            parent_id: None,
            kind: SpanKind::Client,
            attributes: vec![],
            is_wildcard: false,
        }
    }
}

fn check_span_information(
    span: &Arc<MockSpanInner>,
    expected: &ExpectedSpanInformation<'_>,
    parent_span_map: &HashMap<Uuid, [u8; 8]>,
) {
    assert_eq!(span.name, expected.span_name);
    assert_eq!(span.kind, expected.kind);
    assert_eq!(*span.state.lock().unwrap(), expected.status);
    match span.parent {
        None => assert!(expected.parent_id.is_none()),
        Some(ref parent) => {
            let parent_id = parent_span_map
                .get(expected.parent_id.as_ref().unwrap())
                .unwrap();
            assert_eq!(*parent, *parent_id);
        }
    }
    let attributes = span.attributes.lock().unwrap();
    eprintln!("Expected attributes: {:?}", expected.attributes);
    eprintln!("Found attributes: {:?}", attributes);
    for (index, attr) in attributes.iter().enumerate() {
        eprintln!("Attribute {}: {} = {:?}", index, attr.key, attr.value);
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
    // Finally, ensure the span has been closed (`end()` was called).
    assert!(
        !*span.is_open.lock().unwrap(),
        "Span {} was not ended",
        span.name
    );
}

/// Returns true if the spans match, false otherwise.
fn compare_span_information(
    actual: &Arc<MockSpanInner>,
    expected: &ExpectedSpanInformation<'_>,
    parent_span_map: &HashMap<Uuid, [u8; 8]>,
) -> bool {
    if actual.name != expected.span_name {
        return false;
    }
    if actual.kind != expected.kind {
        return false;
    }
    if *actual.state.lock().unwrap() != expected.status {
        return false;
    }
    match actual.parent {
        None => {
            if expected.parent_id.is_some() {
                return false;
            }
        }
        Some(ref parent) => {
            let parent_id = parent_span_map
                .get(expected.parent_id.as_ref().unwrap())
                .unwrap();
            if *parent != *parent_id {
                return false;
            }
        }
    }
    let attributes = actual.attributes.lock().unwrap();
    eprintln!("Expected attributes: {:?}", expected.attributes);
    eprintln!("Found attributes: {:?}", attributes);
    for (index, attr) in attributes.iter().enumerate() {
        eprintln!("Attribute {}: {} = {:?}", index, attr.key, attr.value);
        let mut found = false;
        for (key, value) in &expected.attributes {
            if attr.key == *key {
                // Skip checking the value for "<ANY>" as it is a placeholder
                if *value != AttributeValue::String("<ANY>".into()) && attr.value != *value {
                    return false;
                }
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }
    for (key, _) in expected.attributes.iter() {
        if !attributes.iter().any(|attr| attr.key == *key) {
            return false;
        }
    }
    true
}

/// Information about an instrumented API call.
///
/// This structure is used to collect information about a specific API call that is being instrumented for tracing.
///
/// It provides hooks which can be use to verify the expected HTTP result type for the API call, and provides the ability
/// to register any service client specific public API attributes which will be generated during the API call.
#[derive(Debug, Clone)]
pub struct ExpectedApiInformation {
    /// The name of the API being called.
    ///
    /// This is the name of the API as it appears in the service documentation. If `None`, it means that
    /// public API instrumentation is not enabled for this API, and the test will only look for request
    /// instrumentation spans.
    pub api_name: Option<&'static str>,

    /// Information about the child spans generated by the API call.
    ///
    /// This is a list of the child spans which are expected to be generated by this API call.
    pub api_children: Vec<ExpectedRestApiSpan>,

    /// A set of optional additional attributes attached to the public API span for service clients which require them.
    /// If the attribute value has the `<ANY>` placeholder, it means that the test should accept any value for that attribute.
    pub additional_api_attributes: Vec<(&'static str, AttributeValue)>,
}

impl Default for ExpectedApiInformation {
    fn default() -> Self {
        Self {
            api_name: None,
            additional_api_attributes: Vec::new(),
            // Expect a single successful `get` API.
            api_children: vec![ExpectedRestApiSpan::default()],
        }
    }
}

/// Information about an instrumented REST API call.
///
/// This structure is used to collect information about a specific REST API call that is being instrumented for tracing.
///
/// It describes the HTTP method and the expected status code associated with the API call.
#[derive(Debug, Clone)]
pub struct ExpectedRestApiSpan {
    /// The HTTP verb used in the REST API request.
    pub api_verb: azure_core::http::Method,

    /// Expected status code returned by the service.
    pub expected_status_code: azure_core::http::StatusCode,

    /// Whether an unknown multiple of this span will be found.
    pub is_wildcard: bool,
}

impl Default for ExpectedRestApiSpan {
    fn default() -> Self {
        Self {
            api_verb: azure_core::http::Method::Get,
            expected_status_code: azure_core::http::StatusCode::Ok,
            is_wildcard: false,
        }
    }
}

/// Information about an instrumented package calling the `test_instrumentation_for_api` test.
#[derive(Debug, Default, Clone)]
pub struct ExpectedInstrumentation {
    /// The package name for the service client.
    ///
    /// **NOTE**: Make sure that the package name comes from `env!("CARGO_PKG_NAME")` to ensure that this continues to work
    /// if test recordings were created with a previous version of the package.
    pub package_name: String,
    /// The package version for the service client.
    ///
    /// **NOTE**: Make sure that the package version comes from `env!("CARGO_PKG_VERSION")` to ensure that this continues to work
    /// if test recordings were created with a previous version of the package.
    pub package_version: String,
    /// The namespace for the service client.
    pub package_namespace: Option<&'static str>,
    /// Individual instrumented API calls from the test function.
    pub api_calls: Vec<ExpectedApiInformation>,
}

/// Tests the instrumentation of a service client API call.
///
/// Asserts that the generated distributed tracing information for a particular API matches the expected shape of the API.
///
/// # Arguments
/// - `create_client`: A function to create the service client.
/// - `test_api`: A function to test the API call.
/// - `api_information`: Information about the API call being tested.
///
/// This function will call the `create_client` function to create a new instance of the service client. It is the responsibility of the `create_client` callback to
/// add the provided distributed tracing `TracerProvider` to the newly created service client.
///
/// Once the client has been created, it will call the `test_api` function to test the API call(s).
///
/// After the APIs have been tested, this function will verify that the expected tracing spans were created.
///
/// To do that, it uses the `[ExpectedInstrumentation]` structure to collect and compare the actual spans generated during the test.
///
/// The code does not verify the actual client URLs or server ports, it only verifies that the relevant attributes are created.
///
/// The `test_api` call may issue multiple service client calls, if it does, this function will verify that all expected spans were created. The caller of the `test_instrumentation_for_api` call
/// should make sure to include all expected APIs in the call.
///
///
pub async fn assert_instrumentation_information<C, FnInit, FnTest, T>(
    create_client: FnInit,
    test_api: FnTest,
    api_information: ExpectedInstrumentation,
) -> azure_core::Result<()>
where
    FnInit: FnOnce(Arc<dyn TracerProvider>) -> azure_core::Result<C>,
    FnTest: AsyncFnOnce(C) -> azure_core::Result<T>,
{
    // Initialize the mock tracer provider
    let mock_tracer = Arc::new(MockTracingProvider::new());

    // Create a client with the mock tracer
    let client = create_client(mock_tracer.clone())?;

    // We don't actually care about the result of the API call - just that it was made.
    let _ = test_api(client).await;

    // There will be two tracers generated - one for public APIs, the second for HTTP calls.
    //
    // If there are public API instrumentation spans, we will see the public API and HTTP API traces on
    // the public API tracer.
    let mut public_api_tracer = ExpectedTracerInformation {
        name: api_information.package_name.as_str(),
        version: Some(api_information.package_version.as_str()),
        namespace: api_information.package_namespace,
        spans: Vec::new(),
    };

    // If there are no public API spans in the API call, they will appear on the Request Activity Tracer.
    let mut request_activity_tracer = ExpectedTracerInformation {
        name: api_information.package_name.as_str(),
        version: Some(api_information.package_version.as_str()),
        namespace: None,
        spans: Vec::new(),
    };

    // Iterate over the expected API calls calculating the expected spans which will be created.
    for api_call in api_information.api_calls.iter() {
        let mut expected_spans = Vec::new();

        let mut public_api_attributes = api_call.additional_api_attributes.clone();
        // Add additional attributes as needed.
        if let Some(namespace) = api_information.package_namespace {
            public_api_attributes.push(("az.namespace", namespace.into()));
        }

        // If any of the child API calls returns an error, we expect the top level span to have the
        // error.type attribute.
        let mut span_status = SpanStatus::Unset;
        for rest_api_call in api_call.api_children.iter() {
            if !rest_api_call.expected_status_code.is_success() {
                public_api_attributes.push((
                    "error.type",
                    rest_api_call.expected_status_code.to_string().into(),
                ));
            }
            if rest_api_call.expected_status_code.is_server_error() {
                span_status = SpanStatus::Error {
                    description: "".into(),
                };
                break;
            }
        }

        let api_id = Uuid::new_v4();

        if let Some(api_name) = api_call.api_name {
            // Public API spans only enter the Error state if the status code is a server error.
            expected_spans.push(ExpectedSpanInformation {
                span_name: api_name,
                span_id: api_id,
                status: span_status,
                kind: SpanKind::Internal,
                parent_id: None,
                is_wildcard: false, // Public API spans cannot be wildcards.
                attributes: public_api_attributes,
            });
        }

        // Now add the child spans for each of the expected Rest API calls.
        for rest_api_call in api_call.api_children.iter() {
            // Add the HTTP API span after creating the expected set of attributes.
            let mut http_request_attributes = vec![
                (
                    "http.request.method",
                    rest_api_call.api_verb.as_str().into(),
                ),
                ("url.full", "<ANY>".into()),
                ("server.address", "<ANY>".into()),
                ("server.port", "<ANY>".into()),
                ("az.client_request_id", "<ANY>".into()),
                (
                    "http.response.status_code",
                    (*rest_api_call.expected_status_code).into(),
                ),
            ];
            if !rest_api_call.expected_status_code.is_success() {
                http_request_attributes.push((
                    "error.type",
                    rest_api_call.expected_status_code.to_string().into(),
                ));
            }
            // If we have no public API information, we won't have a namespace in the HTTP attributes.
            if api_call.api_name.is_some() && api_information.package_namespace.is_some() {
                http_request_attributes.push((
                    "az.namespace",
                    api_information.package_namespace.unwrap().into(),
                ));
            }
            expected_spans.push(ExpectedSpanInformation {
                span_name: rest_api_call.api_verb.as_str(),
                // If the API call has a name, this should be a child span.
                parent_id: if api_call.api_name.is_some() {
                    Some(api_id)
                } else {
                    None
                },
                // If allow_unknown_children is set, we don't know how many child spans there will be.
                // Use a wildcard span ID to indicate that.
                is_wildcard: rest_api_call.is_wildcard,
                span_id: Uuid::new_v4(),
                status: if !rest_api_call.expected_status_code.is_success() {
                    SpanStatus::Error {
                        description: "".into(),
                    }
                } else {
                    SpanStatus::Unset
                },
                kind: SpanKind::Client,
                attributes: http_request_attributes,
            });
        }
        if api_call.api_name.is_some() {
            public_api_tracer.spans.extend(expected_spans);
        } else {
            request_activity_tracer.spans.extend(expected_spans);
        }
    }
    let expected_tracers = vec![public_api_tracer, request_activity_tracer];

    check_instrumentation_result(mock_tracer, expected_tracers);

    Ok(())
}
