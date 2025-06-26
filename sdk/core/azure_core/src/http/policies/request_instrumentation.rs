// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    http::{headers, options::RequestInstrumentationOptions, Context, Request},
    tracing::{Span, SpanKind},
};
use std::sync::Arc;
use typespec_client_core::{
    http::policies::{Policy, PolicyResult, RetryPolicyCount},
    tracing::Attribute,
};

#[allow(dead_code)]
const AZ_NAMESPACE_ATTRIBUTE: &str = "az.namespace";

const AZ_SCHEMA_URL_ATTRIBUTE: &str = "az.schema.url";
const AZ_CLIENT_REQUEST_ID_ATTRIBUTE: &str = "az.client.request.id";
const ERROR_TYPE_ATTRIBUTE: &str = "error.type";
const AZ_SERVICE_REQUEST_ID_ATTRIBUTE: &str = "az.service.request.id";
const HTTP_REQUEST_RESEND_COUNT_ATTRIBUTE: &str = "http.request.resend.count";
const HTTP_RESPONSE_STATUS_CODE_ATTRIBUTE: &str = "http.response.status_code";
const HTTP_REQUEST_METHOD_ATTRIBUTE: &str = "http.request.method";
const SERVER_ADDRESS_ATTRIBUTE: &str = "server.address";
const SERVER_PORT_ATTRIBUTE: &str = "server.port";
const URL_FULL_ATTRIBUTE: &str = "url.full";

/// Sets the User-Agent header with useful information in a typical format for Azure SDKs.
#[derive(Clone, Debug)]
pub struct RequestInstrumentationPolicy {
    tracer: Option<Arc<dyn crate::tracing::Tracer>>,
}

impl RequestInstrumentationPolicy {
    pub fn new(
        crate_name: Option<&'static str>,
        crate_version: Option<&'static str>,
        options: Option<&RequestInstrumentationOptions>,
    ) -> Self {
        if let Some(tracing_provider) = options.and_then(|o| o.tracing_provider.clone()) {
            Self {
                tracer: Some(tracing_provider.get_tracer(
                    crate_name.unwrap_or("unknown"),
                    crate_version.unwrap_or("unknown"),
                )),
            }
        } else {
            // If no tracing provider is set, we return a policy with no tracer.
            Self { tracer: None }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl Policy for RequestInstrumentationPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        if let Some(tracer) = &self.tracer {
            let mut span_attributes = vec![
                Attribute {
                    key: HTTP_REQUEST_METHOD_ATTRIBUTE,
                    value: request.method().to_string().into(),
                },
                Attribute {
                    key: AZ_SCHEMA_URL_ATTRIBUTE,
                    value: request.url().scheme().into(),
                },
            ];

            if !request.url().username().is_empty() || request.url().password().is_some() {
                // If the URL contains a password, we do not log it for security reasons.
                let full_url = format!(
                    "{}://{}{}{}{}{}",
                    request.url().scheme(),
                    request
                        .url()
                        .host()
                        .map_or_else(|| "unknown_host".to_string(), |h| h.to_string()),
                    request
                        .url()
                        .port()
                        .map_or_else(String::new, |p| format!(":{}", p)),
                    request.url().path(),
                    request
                        .url()
                        .query()
                        .map_or_else(String::new, |q| format!("?{}", q)),
                    request
                        .url()
                        .fragment()
                        .map_or_else(String::new, |f| format!("#{}", f)),
                );
                span_attributes.push(Attribute {
                    key: URL_FULL_ATTRIBUTE,
                    value: full_url.into(),
                });
            } else {
                // If no password is present, we log the full URL.
                span_attributes.push(Attribute {
                    key: URL_FULL_ATTRIBUTE,
                    value: request.url().to_string().into(),
                });
            }

            if let Some(host) = request.url().host() {
                span_attributes.push(Attribute {
                    key: SERVER_ADDRESS_ATTRIBUTE,
                    value: host.to_string().into(),
                });
            }
            if let Some(port) = request.url().port_or_known_default() {
                span_attributes.push(Attribute {
                    key: SERVER_PORT_ATTRIBUTE,
                    value: port.into(),
                });
            }
            // Get the method as a string to avoid lifetime issues
            let method_str = request.method_as_str();
            let span = if let Some(parent_span) = ctx.value::<Arc<dyn Span>>() {
                // If a parent span exists, start a new span with the parent.
                tracer.start_span_with_parent(
                    method_str,
                    SpanKind::Client,
                    span_attributes,
                    parent_span.clone(),
                )
            } else {
                // If no parent span exists, start a new span without a parent.
                tracer.start_span_with_current(method_str, SpanKind::Client, span_attributes)
            };

            if let Some(client_request_id) = request
                .headers()
                .get_optional_str(&headers::CLIENT_REQUEST_ID)
            {
                span.set_attribute(AZ_CLIENT_REQUEST_ID_ATTRIBUTE, client_request_id.into());
            }

            if let Some(service_request_id) =
                request.headers().get_optional_str(&headers::REQUEST_ID)
            {
                span.set_attribute(AZ_SERVICE_REQUEST_ID_ATTRIBUTE, service_request_id.into());
            }

            if let Some(retry_count) = ctx.value::<RetryPolicyCount>() {
                span.set_attribute(HTTP_REQUEST_RESEND_COUNT_ATTRIBUTE, retry_count.0.into());
            }

            let result = next[0].send(ctx, request, &next[1..]).await;

            if result.is_err() {
                // If the request failed, set an error type attribute.
                span.set_attribute(
                    ERROR_TYPE_ATTRIBUTE,
                    result.as_ref().err().unwrap().to_string().into(),
                );
            }
            if let Ok(response) = result.as_ref() {
                // If the request was successful, set the HTTP response status code.
                span.set_attribute(
                    HTTP_RESPONSE_STATUS_CODE_ATTRIBUTE,
                    u16::from(response.status()).into(),
                );

                if response.status().is_server_error() || response.status().is_client_error() {
                    // If the response status indicates an error, set the span status to error.
                    span.set_status(crate::tracing::SpanStatus::Error {
                        description: format!(
                            "HTTP request failed with status code {}: {}",
                            response.status(),
                            response.status().canonical_reason()
                        ),
                    });
                }
            }

            span.end();
            return result;
        } else {
            // If no tracer is set, we simply forward the request without instrumentation.
            next[0].send(ctx, request, &next[1..]).await
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use typespec_client_core::{
        http::{
            headers::Headers, policies::TransportPolicy, Method, RawResponse, StatusCode,
            TransportOptions,
        },
        tracing::{AsAny, AttributeValue, Span, SpanStatus, Tracer, TracerProvider},
    };

    #[derive(Debug)]
    struct MockTransport;

    #[async_trait::async_trait]
    impl Policy for MockTransport {
        async fn send(
            &self,
            _ctx: &Context,
            _request: &mut Request,
            _next: &[Arc<dyn Policy>],
        ) -> PolicyResult {
            PolicyResult::Ok(RawResponse::from_bytes(
                StatusCode::Ok,
                Headers::new(),
                Vec::new(),
            ))
        }
    }

    #[derive(Debug)]
    struct MockTracingProvider {
        tracers: Mutex<Vec<Arc<MockTracer>>>,
    }

    impl MockTracingProvider {
        fn new() -> Self {
            Self {
                tracers: Mutex::new(Vec::new()),
            }
        }
    }
    impl TracerProvider for MockTracingProvider {
        fn get_tracer(
            &self,
            crate_name: &str,
            crate_version: &str,
        ) -> Arc<dyn crate::tracing::Tracer> {
            let mut tracers = self.tracers.lock().unwrap();
            let tracer = Arc::new(MockTracer {
                name: crate_name.to_string(),
                version: crate_version.to_string(),
                spans: Mutex::new(Vec::new()),
            });

            tracers.push(tracer.clone());
            tracer
        }
    }

    #[derive(Debug)]
    struct MockTracer {
        name: String,
        version: String,
        spans: Mutex<Vec<Arc<MockSpan>>>,
    }

    impl Tracer for MockTracer {
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
    struct MockSpan {
        name: String,
        #[allow(dead_code)]
        kind: SpanKind,
        #[allow(dead_code)]
        attributes: Mutex<Vec<Attribute>>,
        state: Mutex<SpanStatus>,
        is_open: Mutex<bool>,
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
        ) -> typespec_client_core::Result<Box<dyn typespec_client_core::tracing::SpanGuard>>
        {
            todo!()
        }
    }

    impl AsAny for MockSpan {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    async fn run_instrumentation_test(
        crate_name: Option<&'static str>,
        version: Option<&'static str>,
        request: &mut Request,
    ) -> Arc<MockTracingProvider> {
        let mock_tracer = Arc::new(MockTracingProvider::new());
        let options = RequestInstrumentationOptions {
            tracing_provider: Some(mock_tracer.clone()),
        };
        let policy = Arc::new(RequestInstrumentationPolicy::new(
            crate_name,
            version,
            Some(&options),
        ));

        let transport =
            TransportPolicy::new(TransportOptions::new_custom_policy(Arc::new(MockTransport)));

        let ctx = Context::default();
        let next: Vec<Arc<dyn Policy>> = vec![Arc::new(transport)];
        let _result = policy.send(&ctx, request, &next).await;

        mock_tracer
    }
    fn check_instrumentation_result(
        mock_tracer: Arc<MockTracingProvider>,
        expected_name: &str,
        expected_version: &str,
        expected_method: &str,
        expected_attributes: Vec<(&str, AttributeValue)>,
    ) {
        assert_eq!(
            mock_tracer.tracers.lock().unwrap().len(),
            1,
            "Expected one tracer to be created",
        );
        let tracers = mock_tracer.tracers.lock().unwrap();
        let tracer = tracers.first().unwrap();
        assert_eq!(tracer.name, expected_name);
        assert_eq!(tracer.version, expected_version);
        let spans = tracer.spans.lock().unwrap();
        assert_eq!(spans.len(), 1, "Expected one span to be created");
        println!("Spans: {:?}", spans);
        let span = spans.first().unwrap();
        assert_eq!(span.name, expected_method);
        let attributes = span.attributes.lock().unwrap();
        for attr in attributes.iter() {
            println!("Attribute: {} = {:?}", attr.key, attr.value);
            let mut found = false;
            for (key, value) in &expected_attributes {
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
        for (key, value) in &expected_attributes {
            if !attributes
                .iter()
                .any(|attr| attr.key == *key && attr.value == *value)
            {
                panic!("Expected attribute not found: {} = {:?}", key, value);
            }
        }
    }

    #[tokio::test]
    async fn simple_instrumentation_policy() {
        let url = "http://example.com/path";
        let mut request = Request::new(url.parse().unwrap(), Method::Get);

        let mock_tracer =
            run_instrumentation_test(Some("test_crate"), Some("1.0.0"), &mut request).await;

        check_instrumentation_result(
            mock_tracer,
            "test_crate",
            "1.0.0",
            "GET",
            vec![
                (AZ_SCHEMA_URL_ATTRIBUTE, AttributeValue::from("http")),
                (
                    HTTP_RESPONSE_STATUS_CODE_ATTRIBUTE,
                    AttributeValue::from(200),
                ),
                (HTTP_REQUEST_METHOD_ATTRIBUTE, AttributeValue::from("GET")),
                (
                    SERVER_ADDRESS_ATTRIBUTE,
                    AttributeValue::from("example.com"),
                ),
                (SERVER_PORT_ATTRIBUTE, AttributeValue::from(80)),
                (
                    URL_FULL_ATTRIBUTE,
                    AttributeValue::from("http://example.com/path"),
                ),
            ],
        );
    }

    #[tokio::test]
    async fn client_request_id() {
        let url = "https://example.com/client_request_id";
        let mut request = Request::new(url.parse().unwrap(), Method::Get);
        request.insert_header(headers::CLIENT_REQUEST_ID, "test-client-request-id");

        let mock_tracer =
            run_instrumentation_test(Some("test_crate"), Some("1.0.0"), &mut request).await;

        check_instrumentation_result(
            mock_tracer.clone(),
            "test_crate",
            "1.0.0",
            "GET",
            vec![
                (AZ_SCHEMA_URL_ATTRIBUTE, AttributeValue::from("https")),
                (
                    AZ_CLIENT_REQUEST_ID_ATTRIBUTE,
                    AttributeValue::from("test-client-request-id"),
                ),
                (
                    HTTP_RESPONSE_STATUS_CODE_ATTRIBUTE,
                    AttributeValue::from(200),
                ),
                (HTTP_REQUEST_METHOD_ATTRIBUTE, AttributeValue::from("GET")),
                (
                    SERVER_ADDRESS_ATTRIBUTE,
                    AttributeValue::from("example.com"),
                ),
                (SERVER_PORT_ATTRIBUTE, AttributeValue::from(443)),
                (
                    URL_FULL_ATTRIBUTE,
                    AttributeValue::from("https://example.com/client_request_id"),
                ),
            ],
        );
    }

    #[tokio::test]
    async fn test_url_with_password() {
        let url = "https://user:password@host:8080/path?query=value#fragment";
        let mut request = Request::new(url.parse().unwrap(), Method::Get);

        let mock_tracer_provider = run_instrumentation_test(None, None, &mut request).await;

        check_instrumentation_result(
            mock_tracer_provider,
            "unknown",
            "unknown",
            "GET",
            vec![
                (AZ_SCHEMA_URL_ATTRIBUTE, AttributeValue::from("https")),
                (
                    HTTP_RESPONSE_STATUS_CODE_ATTRIBUTE,
                    AttributeValue::from(200),
                ),
                (HTTP_REQUEST_METHOD_ATTRIBUTE, AttributeValue::from("GET")),
                (SERVER_ADDRESS_ATTRIBUTE, AttributeValue::from("host")),
                (SERVER_PORT_ATTRIBUTE, AttributeValue::from(8080)),
                (
                    URL_FULL_ATTRIBUTE,
                    AttributeValue::from("https://host:8080/path?query=value#fragment"),
                ),
            ],
        );
    }
}
