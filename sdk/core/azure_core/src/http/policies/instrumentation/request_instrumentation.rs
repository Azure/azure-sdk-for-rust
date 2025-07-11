// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    AZ_CLIENT_REQUEST_ID_ATTRIBUTE, AZ_NAMESPACE_ATTRIBUTE, AZ_SERVICE_REQUEST_ID_ATTRIBUTE,
    ERROR_TYPE_ATTRIBUTE, HTTP_REQUEST_METHOD_ATTRIBUTE, HTTP_REQUEST_RESEND_COUNT_ATTRIBUTE,
    HTTP_RESPONSE_STATUS_CODE_ATTRIBUTE, SERVER_ADDRESS_ATTRIBUTE, SERVER_PORT_ATTRIBUTE,
    URL_FULL_ATTRIBUTE,
};
use crate::{
    http::{headers, Context, Request},
    tracing::{Span, SpanKind},
};
use std::sync::Arc;
use typespec_client_core::{
    http::policies::{Policy, PolicyResult, RetryPolicyCount},
    tracing::Attribute,
};

/// Sets distributed tracing information for HTTP requests.
#[derive(Clone, Debug)]
pub(crate) struct RequestInstrumentationPolicy {
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
        tracer: Option<Arc<dyn crate::tracing::Tracer>>,
        // azure_namespace: Option<&'static str>,
        // crate_name: Option<&'static str>,
        // crate_version: Option<&'static str>,
        // options: &RequestInstrumentationOptions,
    ) -> Self {
        Self { tracer }
        // if let Some(tracing_provider) = &options.tracing_provider {
        //     Self {
        //         tracer: Some(tracing_provider.get_tracer(
        //             azure_namespace,
        //             crate_name.unwrap_or("unknown"),
        //             crate_version.unwrap_or("unknown"),
        //         )),
        //     }
        // } else {
        //     Self { tracer: None }
        // }
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

        // If there is a span in the context, if it's not recording, just forward the request
        // without instrumentation.
        if let Some(span) = ctx.value::<Arc<dyn Span>>() {
            if !span.is_recording() {
                // If the span is not recording, we skip instrumentation.
                return next[0].send(ctx, request, &next[1..]).await;
            }
        }

        let Some(tracer) = tracer else {
            return next[0].send(ctx, request, &next[1..]).await;
        };
        let mut span_attributes = vec![Attribute {
            key: HTTP_REQUEST_METHOD_ATTRIBUTE,
            value: request.method().to_string().into(),
        }];

        if let Some(namespace) = tracer.namespace() {
            // If the tracer has a namespace, we set it as an attribute.
            span_attributes.push(Attribute {
                key: AZ_NAMESPACE_ATTRIBUTE,
                value: namespace.into(),
            });
        }

        // OpenTelemetry requires that we sanitize the URL if it contains a username or password.
        // Since a valid Azure SDK endpoint should never contain a username or password, if
        // the url contains a username or password, we simply omit the URL_FULL_ATTRIBUTE.
        if request.url().username().is_empty() && request.url().password().is_none() {
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

        if !span.is_recording() {
            // If the span is not recording, we skip instrumentation.
            return next[0].send(ctx, request, &next[1..]).await;
        }

        if let Some(client_request_id) = request
            .headers()
            .get_optional_str(&headers::CLIENT_REQUEST_ID)
        {
            span.set_attribute(AZ_CLIENT_REQUEST_ID_ATTRIBUTE, client_request_id.into());
        }

        if let Some(service_request_id) = request.headers().get_optional_str(&headers::REQUEST_ID) {
            span.set_attribute(AZ_SERVICE_REQUEST_ID_ATTRIBUTE, service_request_id.into());
        }

        if let Some(retry_count) = ctx.value::<RetryPolicyCount>() {
            span.set_attribute(HTTP_REQUEST_RESEND_COUNT_ATTRIBUTE, (**retry_count).into());
        }

        // Propagate the headers for distributed tracing into the request.
        span.propagate_headers(request);

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
    }
}
#[cfg(test)]
pub(crate) mod tests {
    // cspell: ignore traceparent
    use super::*;
    use crate::{
        http::{
            headers::Headers,
            policies::{
                instrumentation::tests::{
                    check_request_instrumentation_result, InstrumentationExpectation,
                    MockTracingProvider,
                },
                TransportPolicy,
            },
            Method, RawResponse, StatusCode, TransportOptions,
        },
        tracing::{AttributeValue, SpanStatus, TracerProvider},
        Result,
    };
    use azure_core_test::http::MockHttpClient;
    use futures::future::BoxFuture;
    use std::sync::Arc;
    use typespec_client_core::http::headers::HeaderName;

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
        let mock_tracer_provider = Arc::new(MockTracingProvider::new());
        let tracer = mock_tracer_provider.get_tracer(
            test_namespace,
            crate_name.unwrap_or("unknown"),
            version.unwrap_or("unknown"),
        );
        let policy = Arc::new(RequestInstrumentationPolicy::new(Some(tracer.clone())));

        let transport = TransportPolicy::new(TransportOptions::new(Arc::new(MockHttpClient::new(
            callback,
        ))));

        let ctx = Context::default();
        let next: Vec<Arc<dyn Policy>> = vec![Arc::new(transport)];
        let _result = policy.send(&ctx, request, &next).await;

        mock_tracer_provider
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

        check_request_instrumentation_result(
            mock_tracer,
            1,
            0,
            InstrumentationExpectation {
                namespace: Some("test namespace"),
                name: "test_crate",
                version: "1.0.0",
                span_name: "GET",
                status: SpanStatus::Unset,
                kind: SpanKind::Client,
                attributes: vec![
                    (
                        AZ_NAMESPACE_ATTRIBUTE,
                        AttributeValue::from("test namespace"),
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
                    (SERVER_PORT_ATTRIBUTE, AttributeValue::from(80)),
                    (
                        URL_FULL_ATTRIBUTE,
                        AttributeValue::from("http://example.com/path"),
                    ),
                ],
            },
        );
    }

    #[test]
    fn test_request_instrumentation_policy_creation() {
        let policy = RequestInstrumentationPolicy::new(None);
        assert!(policy.tracer.is_none());

        let mock_tracer_provider = Arc::new(MockTracingProvider::new());
        let tracer = mock_tracer_provider.get_tracer(Some("test namespace"), "test_crate", "1.0.0");
        let policy_with_tracer = RequestInstrumentationPolicy::new(Some(tracer));
        assert!(policy_with_tracer.tracer.is_some());
    }

    #[test]
    fn test_request_instrumentation_policy_without_tracer() {
        let policy = RequestInstrumentationPolicy::new(None);
        assert!(policy.tracer.is_none());
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
                    assert_eq!(
                        req.headers()
                            .get_optional_str(&HeaderName::from_static("traceparent")),
                        Some("00-<trace_id>-<span_id>-01")
                    );
                    Ok(RawResponse::from_bytes(
                        StatusCode::Ok,
                        Headers::new(),
                        vec![],
                    ))
                })
            },
        )
        .await;

        check_request_instrumentation_result(
            mock_tracer.clone(),
            1,
            0,
            InstrumentationExpectation {
                namespace: None,
                name: "test_crate",
                version: "1.0.0",
                span_name: "GET",
                status: SpanStatus::Unset,
                kind: SpanKind::Client,
                attributes: vec![
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
            },
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
        // Because the URL contains a username and password, we do not set the URL_FULL_ATTRIBUTE.
        check_request_instrumentation_result(
            mock_tracer_provider,
            1,
            0,
            InstrumentationExpectation {
                namespace: None,
                name: "unknown",
                version: "unknown",
                span_name: "GET",
                status: SpanStatus::Unset,
                kind: SpanKind::Client,
                attributes: vec![
                    (
                        HTTP_RESPONSE_STATUS_CODE_ATTRIBUTE,
                        AttributeValue::from(200),
                    ),
                    (HTTP_REQUEST_METHOD_ATTRIBUTE, AttributeValue::from("GET")),
                    (SERVER_ADDRESS_ATTRIBUTE, AttributeValue::from("host")),
                    (SERVER_PORT_ATTRIBUTE, AttributeValue::from(8080)),
                ],
            },
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
        check_request_instrumentation_result(
            mock_tracer,
            1,
            0,
            InstrumentationExpectation {
                namespace: Some("test namespace"),
                name: "test_crate",
                version: "1.0.0",
                span_name: "PUT",
                status: SpanStatus::Error {
                    description: "".to_string(),
                },
                kind: SpanKind::Client,
                attributes: vec![
                    (ERROR_TYPE_ATTRIBUTE, AttributeValue::from("404")),
                    (
                        AZ_SERVICE_REQUEST_ID_ATTRIBUTE,
                        AttributeValue::from("test-service-request-id"),
                    ),
                    (
                        AZ_NAMESPACE_ATTRIBUTE,
                        AttributeValue::from("test namespace"),
                    ),
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
            },
        );
    }
}
