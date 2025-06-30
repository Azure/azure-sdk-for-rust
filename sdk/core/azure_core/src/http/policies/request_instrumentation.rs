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

/// Sets distributed tracing information for HTTP requests.
#[derive(Clone, Debug)]
pub struct RequestInstrumentationPolicy {
    tracer: Option<Arc<dyn crate::tracing::Tracer>>,
}

impl RequestInstrumentationPolicy {
    /// Creates a new `RequestInstrumentationPolicy`.
    ///
    /// # Arguments
    /// - `azure_namespace`: The Azure namespace for the tracer.
    /// - `crate_name`: The name of the crate for which the tracer is created.
    /// - `crate_version`: The version of the crate for which the tracer is created.
    /// - `options`: Options for request instrumentation, including the tracing provider.
    ///
    /// # Returns
    /// A new instance of `RequestInstrumentationPolicy`.
    ///
    /// # Note
    /// This policy will only create a tracer if a tracing provider is provided in the options.
    ///
    /// This policy will create a tracer that can be used to instrument HTTP requests.
    /// However this tracer is only used when the client method is NOT instrumented.
    /// A part of the client method instrumentation sets a client-specific tracer into the
    /// request `[Context]` which will be used instead of the tracer from this policy.
    ///
    pub fn new(
        azure_namespace: Option<&'static str>,
        crate_name: Option<&'static str>,
        crate_version: Option<&'static str>,
        options: &RequestInstrumentationOptions,
    ) -> Self {
        if let Some(tracing_provider) = &options.tracing_provider {
            Self {
                tracer: Some(tracing_provider.get_tracer(
                    azure_namespace,
                    crate_name.unwrap_or("unknown"),
                    crate_version.unwrap_or("unknown"),
                )),
            }
        } else {
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
        // If the context has a tracer (which happens when called from an instrumented method),
        // we prefer the tracer from the context.
        // Otherwise, we use the tracer from the policy itself.
        // This allows for flexibility in using different tracers in different contexts.
        let tracer = if ctx.value::<Arc<dyn crate::tracing::Tracer>>().is_some() {
            ctx.value::<Arc<dyn crate::tracing::Tracer>>()
        } else {
            self.tracer.as_ref()
        };

        if let Some(tracer) = tracer {
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

            if let Some(namespace) = tracer.namespace() {
                // If the tracer has a namespace, we set it as an attribute.
                span_attributes.push(Attribute {
                    key: AZ_NAMESPACE_ATTRIBUTE,
                    value: namespace.into(),
                });
            }

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
            //            let method_str = request.method_as_str();
            let method_str = request.method().as_str();
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

            if let Some(err) = result.as_ref().err() {
                // If the request failed, set an error type attribute.
                let azure_error = err.downcast_ref::<crate::Error>();
                if let Some(err_kind) = azure_error.map(|e| e.kind()) {
                    // If the error is an Azure core error, we set the error type.
                    span.set_attribute(ERROR_TYPE_ATTRIBUTE, err_kind.to_string().into());
                } else {
                    // Otherwise, we set the error type to the error's text. This should never happen
                    // as the error should be an Azure core error.
                    span.set_attribute(ERROR_TYPE_ATTRIBUTE, err.to_string().into());
                }
            }
            if let Ok(response) = result.as_ref() {
                // If the request was successful, set the HTTP response status code.
                span.set_attribute(
                    HTTP_RESPONSE_STATUS_CODE_ATTRIBUTE,
                    u16::from(response.status()).into(),
                );

                if response.status().is_server_error() || response.status().is_client_error() {
                    // If the response status indicates an error, set the span status to error.
                    // Since the reason can be inferred from the status code, description is left empty.
                    span.set_status(crate::tracing::SpanStatus::Error {
                        description: "".to_string(),
                    });
                    // Set the error type attribute for all HTTP 4XX or 5XX errors.
                    span.set_attribute(ERROR_TYPE_ATTRIBUTE, response.status().to_string().into());
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
    use crate::{
        http::{
            headers::Headers, policies::TransportPolicy, Method, RawResponse, StatusCode,
            TransportOptions,
        },
        tracing::{AsAny, AttributeValue, Span, SpanStatus, Tracer, TracerProvider},
        Result,
    };
    use azure_core_test::http::MockHttpClient;
    use futures::future::BoxFuture;
    use std::sync::{Arc, Mutex};

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
            azure_namespace: Option<&'static str>,
            crate_name: &'static str,
            crate_version: &'static str,
        ) -> Arc<dyn crate::tracing::Tracer> {
            let mut tracers = self.tracers.lock().unwrap();
            let tracer = Arc::new(MockTracer {
                namespace: azure_namespace,
                name: crate_name,
                version: crate_version,
                spans: Mutex::new(Vec::new()),
            });

            tracers.push(tracer.clone());
            tracer
        }
    }

    #[derive(Debug)]
    struct MockTracer {
        namespace: Option<&'static str>,
        name: &'static str,
        version: &'static str,
        spans: Mutex<Vec<Arc<MockSpan>>>,
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

        fn propagate_headers(&self, _request: &mut Request) {}
    }

    impl AsAny for MockSpan {
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    async fn run_instrumentation_test<C>(
        test_namespace: Option<&'static str>,
        crate_name: Option<&'static str>,
        version: Option<&'static str>,
        request: &mut Request,
        callback: C,
    ) -> Arc<MockTracingProvider>
    where
        C: FnMut(&Request) -> BoxFuture<'_, Result<RawResponse>> + Send + Sync + 'static,
    {
        let mock_tracer = Arc::new(MockTracingProvider::new());
        let options = RequestInstrumentationOptions {
            tracing_provider: Some(mock_tracer.clone()),
        };
        let policy = Arc::new(RequestInstrumentationPolicy::new(
            test_namespace,
            crate_name,
            version,
            &options,
        ));

        let transport = TransportPolicy::new(TransportOptions::new(Arc::new(MockHttpClient::new(
            callback,
        ))));

        let ctx = Context::default();
        let next: Vec<Arc<dyn Policy>> = vec![Arc::new(transport)];
        let _result = policy.send(&ctx, request, &next).await;

        mock_tracer
    }
    fn check_instrumentation_result(
        mock_tracer: Arc<MockTracingProvider>,
        expected_namespace: Option<&str>,
        expected_name: &str,
        expected_version: &str,
        expected_method: &str,
        expected_status: SpanStatus,
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
        assert_eq!(tracer.namespace, expected_namespace);
        let spans = tracer.spans.lock().unwrap();
        assert_eq!(spans.len(), 1, "Expected one span to be created");
        println!("Spans: {:?}", spans);
        let span = spans.first().unwrap();
        assert_eq!(span.name, expected_method);
        assert_eq!(*span.state.lock().unwrap(), expected_status);
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

        let mock_tracer = run_instrumentation_test(
            Some("test namespace"),
            Some("test_crate"),
            Some("1.0.0"),
            &mut request,
            |req| {
                Box::pin(async move {
                    assert_eq!(req.url().host_str(), Some("example.com"));
                    assert_eq!(req.method(), &Method::Get);
                    Ok(RawResponse::from_bytes(
                        StatusCode::Ok,
                        Headers::new(),
                        vec![],
                    ))
                })
            },
        )
        .await;

        check_instrumentation_result(
            mock_tracer,
            Some("test namespace"),
            "test_crate",
            "1.0.0",
            "GET",
            SpanStatus::Unset,
            vec![
                (
                    AZ_NAMESPACE_ATTRIBUTE,
                    AttributeValue::from("test namespace"),
                ),
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

        let mock_tracer = run_instrumentation_test(
            None,
            Some("test_crate"),
            Some("1.0.0"),
            &mut request,
            |req| {
                Box::pin(async move {
                    assert_eq!(req.url().host_str(), Some("example.com"));
                    assert_eq!(req.method(), &Method::Get);
                    Ok(RawResponse::from_bytes(
                        StatusCode::Ok,
                        Headers::new(),
                        vec![],
                    ))
                })
            },
        )
        .await;

        check_instrumentation_result(
            mock_tracer.clone(),
            None,
            "test_crate",
            "1.0.0",
            "GET",
            SpanStatus::Unset,
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

        let mock_tracer_provider =
            run_instrumentation_test(None, None, None, &mut request, |req| {
                Box::pin(async move {
                    assert_eq!(req.url().host_str(), Some("host"));
                    assert_eq!(req.method(), &Method::Get);
                    Ok(RawResponse::from_bytes(
                        StatusCode::Ok,
                        Headers::new(),
                        vec![],
                    ))
                })
            })
            .await;

        check_instrumentation_result(
            mock_tracer_provider,
            None,
            "unknown",
            "unknown",
            "GET",
            SpanStatus::Unset,
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

    #[tokio::test]
    async fn request_failed() {
        let url = "https://microsoft.com/request_failed.htm";
        let mut request = Request::new(url.parse().unwrap(), Method::Put);
        request.insert_header(headers::REQUEST_ID, "test-service-request-id");

        let mock_tracer = run_instrumentation_test(
            Some("test namespace"),
            Some("test_crate"),
            Some("1.0.0"),
            &mut request,
            |req| {
                Box::pin(async move {
                    assert_eq!(req.url().host_str(), Some("microsoft.com"));
                    assert_eq!(req.method(), &Method::Put);
                    Ok(RawResponse::from_bytes(
                        StatusCode::NotFound,
                        Headers::new(),
                        vec![],
                    ))
                })
            },
        )
        .await;

        check_instrumentation_result(
            mock_tracer.clone(),
            Some("test namespace"),
            "test_crate",
            "1.0.0",
            "PUT",
            SpanStatus::Error {
                description: "".to_string(),
            },
            vec![
                (ERROR_TYPE_ATTRIBUTE, AttributeValue::from("404")),
                (
                    AZ_SERVICE_REQUEST_ID_ATTRIBUTE,
                    AttributeValue::from("test-service-request-id"),
                ),
                (
                    AZ_NAMESPACE_ATTRIBUTE,
                    AttributeValue::from("test namespace"),
                ),
                (AZ_SCHEMA_URL_ATTRIBUTE, AttributeValue::from("https")),
                (
                    AZ_SERVICE_REQUEST_ID_ATTRIBUTE,
                    AttributeValue::from("test-service-request-id"),
                ),
                (
                    HTTP_RESPONSE_STATUS_CODE_ATTRIBUTE,
                    AttributeValue::from(404),
                ),
                (HTTP_REQUEST_METHOD_ATTRIBUTE, AttributeValue::from("PUT")),
                (
                    SERVER_ADDRESS_ATTRIBUTE,
                    AttributeValue::from("microsoft.com"),
                ),
                (SERVER_PORT_ATTRIBUTE, AttributeValue::from(443)),
                (
                    URL_FULL_ATTRIBUTE,
                    AttributeValue::from("https://microsoft.com/request_failed.htm"),
                ),
            ],
        );
    }
}
