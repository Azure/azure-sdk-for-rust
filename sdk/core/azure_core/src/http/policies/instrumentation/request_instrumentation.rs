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
use std::{borrow::Cow, collections::HashSet, sync::Arc};
use typespec_client_core::{
    http::{
        policies::{Policy, PolicyResult, RetryPolicyCount},
        LoggingOptions, Sanitizer, DEFAULT_ALLOWED_QUERY_PARAMETERS,
    },
    tracing::Attribute,
};

/// Sets distributed tracing information for HTTP requests.
#[derive(Clone, Debug)]
pub(crate) struct RequestInstrumentationPolicy {
    tracer: Option<Arc<dyn crate::tracing::Tracer>>,
    allowed_query_params: HashSet<Cow<'static, str>>,
}

impl RequestInstrumentationPolicy {
    /// Creates a new `RequestInstrumentationPolicy`.
    ///
    /// # Arguments
    /// - `tracer`: Pre-configured tracer to use for instrumentation.
    ///
    /// # Returns
    /// A new instance of `RequestInstrumentationPolicy`.
    ///
    /// # Note
    ///
    /// The tracer provided is a "fallback" tracer which is used if the `ctx` parameter
    /// to the `send` method does not contain a tracer.
    ///
    pub fn new(
        tracer: Option<Arc<dyn crate::tracing::Tracer>>,
        logging_options: &LoggingOptions,
    ) -> Self {
        // Merge the customer or service provided log options with the default allowed query parameters for sanitization.
        // This ensures that any query parameters that are allowed to be logged are also allowed to be propagated in the URL_FULL_ATTRIBUTE.
        // If no log options are provided, we just use the default allowed query parameters.
        // This ensures that we do not accidentally propagate any sensitive information in the URL_FULL_ATTRIBUTE.
        // Note that the allowed header names are not used in this policy, as we do not log headers here.
        let mut allowed_query_params: HashSet<Cow<'static, str>> =
            DEFAULT_ALLOWED_QUERY_PARAMETERS.clone();
        allowed_query_params.extend(logging_options.additional_allowed_query_params.clone());

        Self {
            tracer,
            allowed_query_params,
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

        // We use `.or_else` here instead of `.or` because `.or` eagerly evaluates the right-hand side,
        // which can lead to unnecessary overhead if the tracer is not needed.
        #[allow(clippy::unnecessary_lazy_evaluations)]
        let tracer = ctx
            .value::<Arc<dyn crate::tracing::Tracer>>()
            .or_else(|| self.tracer.as_ref());

        let Some(tracer) = tracer else {
            return next[0].send(ctx, request, &next[1..]).await;
        };

        let mut span_attributes = vec![Attribute {
            key: HTTP_REQUEST_METHOD_ATTRIBUTE.into(),
            value: request.method().to_string().into(),
        }];

        if let Some(namespace) = tracer.namespace() {
            // If the tracer has a namespace, we set it as an attribute.
            span_attributes.push(Attribute {
                key: AZ_NAMESPACE_ATTRIBUTE.into(),
                value: namespace.into(),
            });
        }

        // OpenTelemetry requires that we sanitize the URL if it contains a username or password.
        // Since a valid Azure SDK endpoint should never contain a username or password, if
        // the url contains a username or password, we simply omit the URL_FULL_ATTRIBUTE.
        if request.url().username().is_empty() && request.url().password().is_none() {
            span_attributes.push(Attribute {
                key: URL_FULL_ATTRIBUTE.into(),
                value: request.url().sanitize(&self.allowed_query_params).into(),
            });
        }

        if let Some(host) = request.url().host() {
            span_attributes.push(Attribute {
                key: SERVER_ADDRESS_ATTRIBUTE.into(),
                value: host.to_string().into(),
            });
        }
        if let Some(port) = request.url().port_or_known_default() {
            span_attributes.push(Attribute {
                key: SERVER_PORT_ATTRIBUTE.into(),
                value: port.into(),
            });
        }
        // Get the method as a string to avoid lifetime issues
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
            // If no parent span exists, start a new span with the "current" span (if any).
            // It is up to the tracer implementation to determine what "current" means.
            tracer.start_span(method_str, SpanKind::Client, span_attributes)
        };

        if span.is_recording() {
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
                if **retry_count > 0 {
                    span.set_attribute(HTTP_REQUEST_RESEND_COUNT_ATTRIBUTE, (**retry_count).into());
                }
            }
        }

        // Propagate the headers for distributed tracing into the request.
        span.propagate_headers(request);

        let result = next[0].send(ctx, request, &next[1..]).await;

        if span.is_recording() {
            if let Some(err) = result.as_ref().err() {
                // If the request failed, set an error type attribute.
                span.set_attribute(ERROR_TYPE_ATTRIBUTE, err.kind().to_string().into());
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
            headers::Headers, policies::TransportPolicy, BufResponse, Method, StatusCode, Transport,
        },
        tracing::{AttributeValue, SpanStatus, TracerProvider},
        Result,
    };
    use azure_core_test::{
        http::MockHttpClient,
        tracing::{
            check_instrumentation_result, ExpectedSpanInformation, ExpectedTracerInformation,
            MockTracingProvider,
        },
    };
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
        C: FnMut(&Request) -> BoxFuture<'_, Result<BufResponse>> + Send + Sync + 'static,
    {
        let mock_tracer_provider = Arc::new(MockTracingProvider::new());
        let tracer = mock_tracer_provider.get_tracer(
            test_namespace,
            crate_name.unwrap_or("unknown"),
            version,
        );
        let policy = Arc::new(RequestInstrumentationPolicy::new(
            Some(tracer.clone()),
            &LoggingOptions::default(),
        ));

        let transport =
            TransportPolicy::new(Transport::new(Arc::new(MockHttpClient::new(callback))));

        let ctx = Context::default();
        let next: Vec<Arc<dyn Policy>> = vec![Arc::new(transport)];
        let _result = policy.send(&ctx, request, &next).await;

        mock_tracer_provider
    }

    #[tokio::test]
    async fn simple_instrumentation_policy() {
        let url = "http://example.com/path?query=value&api-version=2024-01-01";
        let mut request = Request::new(url.parse().unwrap(), Method::Get);

        let mock_tracer = run_instrumentation_test(
            Some("test namespace"),
            Some("test_crate"),
            Some("1.0.0"),
            &mut request,
            |req| {
                Box::pin(async move {
                    assert_eq!(req.url().host_str(), Some("example.com"));
                    assert_eq!(req.method(), Method::Get);
                    Ok(BufResponse::from_bytes(
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
            vec![ExpectedTracerInformation {
                namespace: Some("test namespace"),
                name: "test_crate",
                version: Some("1.0.0"),
                spans: vec![ExpectedSpanInformation {
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
                            AttributeValue::from(
                                "http://example.com/path?query=REDACTED&api-version=2024-01-01",
                            ),
                        ),
                    ],
                }],
            }],
        );
    }

    #[test]
    fn test_request_instrumentation_policy_creation() {
        let policy = RequestInstrumentationPolicy::new(None, &LoggingOptions::default());
        assert!(policy.tracer.is_none());

        let mock_tracer_provider = Arc::new(MockTracingProvider::new());
        let tracer =
            mock_tracer_provider.get_tracer(Some("test namespace"), "test_crate", Some("1.0.0"));
        let policy_with_tracer =
            RequestInstrumentationPolicy::new(Some(tracer), &LoggingOptions::default());
        assert!(policy_with_tracer.tracer.is_some());
    }

    #[test]
    fn test_request_instrumentation_policy_without_tracer() {
        let policy = RequestInstrumentationPolicy::new(None, &LoggingOptions::default());
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
                    assert_eq!(req.method(), Method::Get);
                    assert_eq!(
                        req.headers()
                            .get_optional_str(&HeaderName::from_static("traceparent")),
                        Some("00-<trace_id>-<span_id>-01")
                    );
                    Ok(BufResponse::from_bytes(
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
            vec![ExpectedTracerInformation {
                namespace: None,
                name: "test_crate",
                version: Some("1.0.0"),
                spans: vec![ExpectedSpanInformation {
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
                }],
            }],
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
                    assert_eq!(req.method(), Method::Get);
                    Ok(BufResponse::from_bytes(
                        StatusCode::Ok,
                        Headers::new(),
                        vec![],
                    ))
                })
            })
            .await;
        // Because the URL contains a username and password, we do not set the URL_FULL_ATTRIBUTE.
        check_instrumentation_result(
            mock_tracer_provider,
            vec![ExpectedTracerInformation {
                namespace: None,
                name: "unknown",
                version: None,
                spans: vec![ExpectedSpanInformation {
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
                }],
            }],
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
                    assert_eq!(req.method(), Method::Put);
                    Ok(BufResponse::from_bytes(
                        StatusCode::NotFound,
                        Headers::new(),
                        vec![],
                    ))
                })
            },
        )
        .await;

        check_instrumentation_result(
            mock_tracer,
            vec![ExpectedTracerInformation {
                namespace: Some("test namespace"),
                name: "test_crate",
                version: Some("1.0.0"),
                spans: vec![ExpectedSpanInformation {
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
                }],
            }],
        );
    }
}
