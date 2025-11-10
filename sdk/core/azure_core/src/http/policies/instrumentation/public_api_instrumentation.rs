// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{AZ_NAMESPACE_ATTRIBUTE, ERROR_TYPE_ATTRIBUTE};
use crate::{
    http::{Context, Request},
    tracing::{Span, SpanKind, Tracer},
};
use ::tracing::trace;
use std::{borrow::Cow, sync::Arc};
use typespec_client_core::{
    fmt::SafeDebug,
    http::policies::{Policy, PolicyResult},
    tracing::Attribute,
};

/// Information about the public API being instrumented.
///
/// This struct is used to pass information about the public API being instrumented
/// to the `PublicApiInstrumentationPolicy`.
///
/// It contains the name of the API, which is used to create a span for distributed tracing
/// and any additional per-API attributes that might be needed for instrumentation.
///
/// If the `PublicApiInstrumentationPolicy` policy detects a `PublicApiInstrumentationInformation` in the context,
/// it will create a span with the API name and any additional attributes.
#[derive(SafeDebug, Clone)]
pub struct PublicApiInstrumentationInformation {
    /// The name of the API being instrumented.
    ///
    /// The API name should be in the form of `<client>.<api>`, where
    /// `<client>` is the name of the service client and `<api>` is the name of the API.
    ///
    /// For example, if the service client is `MyClient` and the API is `my_api`,
    /// the API name should be `MyClient.my_api`.
    #[safe(true)]
    api_name: Cow<'static, str>,

    /// Additional attributes to be added to the span for this API.
    ///
    /// These attributes can provide additional information about the API being instrumented.
    /// See [Library-specific attributes](https://github.com/Azure/azure-sdk/blob/main/docs/tracing/distributed-tracing-conventions.md#library-specific-attributes)
    /// for more information.
    ///
    attributes: Vec<Attribute>,
}

impl PublicApiInstrumentationInformation {
    /// Creates a new `PublicApiInstrumentationInformation`.
    ///
    /// # Arguments
    /// - `api_name`: The name of the API being instrumented.
    /// - `attributes`: Additional attributes to be added to the span for this API.
    ///
    /// # Returns
    /// A new instance of `PublicApiInstrumentationInformation`.
    ///
    pub fn new(api_name: impl Into<Cow<'static, str>>, attributes: Vec<Attribute>) -> Self {
        Self {
            api_name: api_name.into(),
            attributes,
        }
    }
}

/// Sets distributed tracing information for HTTP requests.
#[derive(Clone, Debug)]
pub(crate) struct PublicApiInstrumentationPolicy {
    tracer: Option<Arc<dyn crate::tracing::Tracer>>,
}

impl PublicApiInstrumentationPolicy {
    /// Creates a new `PublicApiInstrumentationPolicy`.
    ///
    ///
    /// # Returns
    /// A new instance of `PublicApiInstrumentationPolicy`.
    ///
    /// # Note
    /// This policy will only create a tracer if a tracing provider is provided in the options.
    ///
    /// This policy will create a tracer that can be used to instrument HTTP requests.
    /// However this tracer is only used when the client method is NOT instrumented.
    /// A part of the client method instrumentation sets a client-specific tracer into the
    /// request `[Context]` which will be used instead of the tracer from this policy.
    ///
    pub fn new(tracer: Option<Arc<dyn crate::tracing::Tracer>>) -> Self {
        Self { tracer }
    }
}

/// Creates a span for the public API instrumentation policy.
///
/// This function creates a span for the public API instrumentation policy based on the
/// public API information in the context.
///
/// If no PublicApiInstrumentationInformation is provided, then this function will look in the `Context`
/// for a `PublicApiInstrumentationInformation` value, if it is not present, it will return `None`.
///
/// # Arguments
/// - `ctx`: The context containing the public API information.
/// - `tracer`: An optional tracer to use for creating the span.
/// - `public_api_instrumentation`: Optional public API instrumentation information.
///
/// # Returns
/// An optional span if the public API information is present and a tracer is available.
///
/// If the context already has a span, it will return `None` to avoid nested spans.
/// If the context does not have a tracer it will use the value of the `tracer` argument.
/// If no tracer can be determined, it will return `None`.
///
pub fn create_public_api_span(
    ctx: &Context,
    tracer: Option<Arc<dyn Tracer>>,
    public_api_instrumentation: Option<PublicApiInstrumentationInformation>,
) -> Option<Arc<dyn Span>> {
    // If there is a span in the context, we're a nested call, so we just want to forward the request.
    if ctx.value::<Arc<dyn Span>>().is_some() {
        trace!(
            "PublicApiPolicy: Nested call detected, forwarding request without instrumentation."
        );
        return None;
    }

    // We next confirm if the context has public API instrumentation information.
    // Without a public API information, we skip instrumentation.
    let info = public_api_instrumentation
        .or_else(|| ctx.value::<PublicApiInstrumentationInformation>().cloned())?;

    // Get the tracer from either the context or the policy.
    let tracer = match ctx.value::<Arc<dyn Tracer>>() {
        Some(t) => t.clone(),
        None => tracer?,
    };

    // We now have public API information and a tracer.
    // Calculate the span attributes based on the public API information and
    // tracer.
    let mut span_attributes = info
        .attributes
        .iter()
        .map(|attr| {
            // Convert the attribute to a span attribute.
            Attribute {
                key: attr.key.clone(),
                value: attr.value.clone(),
            }
        })
        .collect::<Vec<_>>();

    if let Some(namespace) = tracer.namespace() {
        // If the tracer has a namespace, we set it as an attribute.
        span_attributes.push(Attribute {
            key: AZ_NAMESPACE_ATTRIBUTE.into(),
            value: namespace.into(),
        });
    }

    // Create a span with the public API information and attributes.
    Some(tracer.start_span(info.api_name, SpanKind::Internal, span_attributes))
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl Policy for PublicApiInstrumentationPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        let Some(span) = create_public_api_span(ctx, self.tracer.clone(), None) else {
            return next[0].send(ctx, request, &next[1..]).await;
        };

        // Now add the span to the context, so that it can be used by the next policies.
        let ctx = ctx.clone().with_value(span.clone());

        let result = next[0].send(&ctx, request, &next[1..]).await;

        // Don't bother setting attributes if the span isn't recording.
        if span.is_recording() {
            match &result {
                Err(e) => {
                    // If the request failed, we set the error type on the span.
                    match e.kind() {
                        crate::error::ErrorKind::HttpResponse { status, .. } => {
                            span.set_attribute(ERROR_TYPE_ATTRIBUTE, status.to_string().into());

                            // 5xx status codes SHOULD set status to Error.
                            // The description should not be set because it can be inferred from "http.response.status_code".
                            if status.is_server_error() {
                                span.set_status(crate::tracing::SpanStatus::Error {
                                    description: "".to_string(),
                                });
                            }
                        }
                        _ => {
                            span.set_attribute(ERROR_TYPE_ATTRIBUTE, e.kind().to_string().into());
                            span.set_status(crate::tracing::SpanStatus::Error {
                                description: e.kind().to_string(),
                            });
                        }
                    }
                }
                Ok(response) => {
                    // 5xx status codes SHOULD set status to Error.
                    // The description should not be set because it can be inferred from "http.response.status_code".
                    if response.status().is_server_error() {
                        span.set_status(crate::tracing::SpanStatus::Error {
                            description: "".to_string(),
                        });
                    }
                    if response.status().is_client_error() || response.status().is_server_error() {
                        span.set_attribute(
                            ERROR_TYPE_ATTRIBUTE,
                            response.status().to_string().into(),
                        );
                    }
                }
            }
        }
        span.end();
        result
    }
}

#[cfg(test)]
mod tests {
    // cspell: ignore traceparent
    use super::*;
    use crate::{
        http::{
            headers::Headers,
            policies::{create_public_api_span, RequestInstrumentationPolicy, TransportPolicy},
            AsyncRawResponse, Method, StatusCode, Transport,
        },
        tracing::{SpanStatus, TracerProvider},
        Result, Uuid,
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

    // Test just the public API instrumentation policy without request instrumentation.
    async fn run_public_api_instrumentation_test<C>(
        api_information: Option<PublicApiInstrumentationInformation>,
        create_tracer: bool,
        add_tracer_to_context: bool,
        request: &mut Request,
        callback: C,
    ) -> Arc<MockTracingProvider>
    where
        C: FnMut(&Request) -> BoxFuture<'_, Result<AsyncRawResponse>> + Send + Sync + 'static,
    {
        // Add the public API information and tracer to the context so that it can be used by the policy.
        let mock_tracer_provider = Arc::new(MockTracingProvider::new());

        let tracer = if create_tracer {
            Some(mock_tracer_provider.get_tracer(
                add_tracer_to_context.then_some("test namespace"),
                "test_crate",
                Some("1.0.0"),
            ))
        } else {
            None
        };

        let public_api_policy = {
            let policy_tracer = tracer.clone();
            Arc::new(PublicApiInstrumentationPolicy::new(policy_tracer))
        };

        let transport =
            TransportPolicy::new(Transport::new(Arc::new(MockHttpClient::new(callback))));

        let next: Vec<Arc<dyn Policy>> = vec![Arc::new(transport)];

        let mut ctx = Context::default();
        if let Some(t) = tracer {
            if add_tracer_to_context {
                // If we have a tracer, add it to the context.
                ctx = ctx.with_value(t.clone());
            }
        }

        if let Some(api_information) = api_information {
            // If we have public API information, add it to the context.
            ctx = ctx.with_value(api_information);
        }
        let _result = public_api_policy.send(&ctx, request, &next).await;

        mock_tracer_provider
    }

    async fn run_public_api_instrumentation_test_with_request_instrumentation<C>(
        api_name: Option<&'static str>,
        namespace: Option<&'static str>,
        crate_name: Option<&'static str>,
        version: Option<&'static str>,
        request: &mut Request,
        callback: C,
    ) -> Arc<MockTracingProvider>
    where
        C: FnMut(&Request) -> BoxFuture<'_, Result<AsyncRawResponse>> + Send + Sync + 'static,
    {
        let mock_tracer_provider = Arc::new(MockTracingProvider::new());
        let mock_tracer =
            mock_tracer_provider.get_tracer(namespace, crate_name.unwrap_or("unknown"), version);

        let public_api_policy = Arc::new(PublicApiInstrumentationPolicy::new(Some(
            mock_tracer.clone(),
        )));

        let transport =
            TransportPolicy::new(Transport::new(Arc::new(MockHttpClient::new(callback))));

        let request_instrumentation_policy =
            RequestInstrumentationPolicy::new(Some(mock_tracer.clone()));

        let next: Vec<Arc<dyn Policy>> = vec![
            Arc::new(request_instrumentation_policy),
            Arc::new(transport),
        ];
        let public_api_information = PublicApiInstrumentationInformation::new(
            api_name.unwrap_or("unknown"),
            vec![Attribute {
                key: "az.fake_attribute".into(),
                value: "attribute value".into(),
            }],
        );

        // Add the public API information and tracer to the context so that it can be used by the policy.
        let ctx = Context::default()
            .with_value(public_api_information)
            .with_value(mock_tracer.clone());
        let _result = public_api_policy.send(&ctx, request, &next).await;

        mock_tracer_provider
    }

    // Tests for the create_public_api_span function.
    #[test]
    fn create_public_api_span_tests() {
        let tracer =
            Arc::new(MockTracingProvider::new()).get_tracer(Some("test"), "test", Some("1.0.0"));

        // Test when context has no PublicApiInstrumentationInformation
        {
            let ctx = Context::default();
            let span = create_public_api_span(&ctx, Some(tracer.clone()), None);
            assert!(span.is_none(), "Should return None when no API info exists");
        }
    }

    // Test when context already has a span
    #[test]
    fn create_public_api_span_tests_context_has_span() {
        let tracer =
            Arc::new(MockTracingProvider::new()).get_tracer(Some("test"), "test", Some("1.0.0"));
        {
            let existing_span = tracer.start_span("existing".into(), SpanKind::Internal, vec![]);
            let ctx = Context::default().with_value(existing_span.clone());
            let span = create_public_api_span(&ctx, Some(tracer.clone()), None);
            assert!(
                span.is_none(),
                "Should return None when context already has a span"
            );
        }
    }

    // Tests for the create_public_api_span function.
    #[test]
    fn create_public_api_span_tests_public_api_information_from_param() {
        let tracer =
            Arc::new(MockTracingProvider::new()).get_tracer(Some("test"), "test", Some("1.0.0"));

        // Test when context has no PublicApiInstrumentationInformation
        {
            let ctx = Context::default();
            let span = create_public_api_span(
                &ctx,
                Some(tracer.clone()),
                Some(PublicApiInstrumentationInformation::new(
                    "TestClient.test_api",
                    vec![],
                )),
            );
            assert!(
                span.is_some(),
                "Should return Some when info exists as param"
            );
        }
    }

    // Test with API info but no tracer
    #[test]
    fn create_public_api_span_tests_public_api_info_no_tracer() {
        {
            let api_info = PublicApiInstrumentationInformation::new("TestClient.test_api", vec![]);
            let ctx = Context::default().with_value(api_info);
            let span = create_public_api_span(&ctx, None, None);
            assert!(
                span.is_none(),
                "Should return None when no tracer is available"
            );
        }
    }
    // Test with API info and tracer from context
    #[test]
    fn create_public_api_span_tests_api_info_and_tracer_from_context() {
        let tracer =
            Arc::new(MockTracingProvider::new()).get_tracer(Some("test"), "test", Some("1.0.0"));
        {
            let api_info = PublicApiInstrumentationInformation::new("TestClient.test_api", vec![]);
            let ctx = Context::default()
                .with_value(api_info)
                .with_value(tracer.clone());
            let span = create_public_api_span(&ctx, None, None);
            assert!(
                span.is_some(),
                "Should create span when API info and tracer are available"
            );
        }
    }
    // Test with API info, tracer from parameter, and attributes
    #[test]
    fn create_public_api_span_tests_tracer_from_parameter() {
        let tracer =
            Arc::new(MockTracingProvider::new()).get_tracer(Some("test"), "test", Some("1.0.0"));
        {
            let api_info = PublicApiInstrumentationInformation::new(
                "TestClient.test_api",
                vec![Attribute {
                    key: "test.attribute".into(),
                    value: "test_value".into(),
                }],
            );
            let ctx = Context::default().with_value(api_info);
            let span = create_public_api_span(&ctx, Some(tracer.clone()), None);
            assert!(span.is_some(), "Should create span with attributes");
        }
    }

    #[tokio::test]
    async fn public_api_instrumentation_no_public_api_info() {
        let url = "http://example.com/path";
        let mut request = Request::new(url.parse().unwrap(), Method::Get);

        let mock_tracer = run_public_api_instrumentation_test(
            None, // No public API information.
            true, // Create tracer.
            true,
            &mut request,
            |req| {
                Box::pin(async move {
                    assert_eq!(req.url().host_str(), Some("example.com"));
                    assert_eq!(req.method(), Method::Get);
                    Ok(AsyncRawResponse::from_bytes(
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
                name: "test_crate",
                version: Some("1.0.0"),
                namespace: Some("test namespace"),
                spans: vec![],
            }],
        );
    }

    #[tokio::test]
    async fn public_api_instrumentation_no_tracer() {
        let url = "http://example.com/path";
        let mut request = Request::new(url.parse().unwrap(), Method::Get);

        let mock_tracer = run_public_api_instrumentation_test(
            Some(PublicApiInstrumentationInformation::new(
                "MyClient.MyApi",
                vec![],
            )),
            false, // Create tracer.
            false, // Add tracer to context.
            &mut request,
            |req| {
                Box::pin(async move {
                    assert_eq!(req.url().host_str(), Some("example.com"));
                    assert_eq!(req.method(), Method::Get);
                    Ok(AsyncRawResponse::from_bytes(
                        StatusCode::Ok,
                        Headers::new(),
                        vec![],
                    ))
                })
            },
        )
        .await;

        // No tracer should be created, so we expect no spans.
        check_instrumentation_result(mock_tracer, vec![]);
    }

    #[tokio::test]
    async fn public_api_instrumentation_tracer_not_in_context() {
        let url = "http://example.com/path";
        let mut request = Request::new(url.parse().unwrap(), Method::Get);

        let mock_tracer = run_public_api_instrumentation_test(
            Some(PublicApiInstrumentationInformation::new(
                "MyClient.MyApi",
                vec![],
            )),
            true,  // Create tracer.
            false, // Add tracer to context.
            &mut request,
            |req| {
                Box::pin(async move {
                    assert_eq!(req.url().host_str(), Some("example.com"));
                    assert_eq!(req.method(), Method::Get);
                    Ok(AsyncRawResponse::from_bytes(
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
                name: "test_crate",
                version: Some("1.0.0"),
                namespace: None,
                spans: vec![ExpectedSpanInformation {
                    span_name: "MyClient.MyApi",
                    status: SpanStatus::Unset,
                    kind: SpanKind::Internal,
                    span_id: Uuid::new_v4(),
                    parent_id: None,
                    attributes: vec![],
                }],
            }],
        )
    }

    #[tokio::test]
    async fn simple_public_api_instrumentation_policy() {
        let url = "http://example.com/path";
        let mut request = Request::new(url.parse().unwrap(), Method::Get);

        let mock_tracer = run_public_api_instrumentation_test(
            Some(PublicApiInstrumentationInformation::new(
                "MyClient.MyApi",
                vec![],
            )),
            true, // Create tracer.
            true,
            &mut request,
            |req| {
                Box::pin(async move {
                    assert_eq!(req.url().host_str(), Some("example.com"));
                    assert_eq!(req.method(), Method::Get);
                    Ok(AsyncRawResponse::from_bytes(
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
                name: "test_crate",
                version: Some("1.0.0"),
                namespace: Some("test namespace"),
                spans: vec![ExpectedSpanInformation {
                    span_name: "MyClient.MyApi",
                    status: SpanStatus::Unset,
                    span_id: Uuid::new_v4(),
                    parent_id: None,
                    kind: SpanKind::Internal,
                    attributes: vec![(AZ_NAMESPACE_ATTRIBUTE, "test namespace".into())],
                }],
            }],
        );
    }

    #[tokio::test]
    async fn public_api_instrumentation_policy_with_error() {
        let url = "http://example.com/path";
        let mut request = Request::new(url.parse().unwrap(), Method::Get);

        let mock_tracer = run_public_api_instrumentation_test(
            Some(PublicApiInstrumentationInformation::new(
                "MyClient.MyApi",
                vec![],
            )),
            true,
            true,
            &mut request,
            |req| {
                Box::pin(async move {
                    assert_eq!(req.url().host_str(), Some("example.com"));
                    assert_eq!(req.method(), Method::Get);
                    Ok(AsyncRawResponse::from_bytes(
                        StatusCode::InternalServerError,
                        Headers::new(),
                        vec![],
                    ))
                })
            },
        )
        .await;

        check_instrumentation_result(
            mock_tracer.clone(),
            vec![ExpectedTracerInformation {
                name: "test_crate",
                version: Some("1.0.0"),
                namespace: Some("test namespace"),
                spans: vec![ExpectedSpanInformation {
                    span_name: "MyClient.MyApi",
                    status: SpanStatus::Error {
                        description: "".to_string(),
                    },
                    kind: SpanKind::Internal,
                    span_id: Uuid::new_v4(),
                    parent_id: None,
                    attributes: vec![
                        (AZ_NAMESPACE_ATTRIBUTE, "test namespace".into()),
                        (ERROR_TYPE_ATTRIBUTE, "500".into()),
                    ],
                }],
            }],
        );
    }

    #[tokio::test]
    async fn public_api_instrumentation_policy_with_request_instrumentation() {
        let url = "http://example.com/path_with_request";
        let mut request = Request::new(url.parse().unwrap(), Method::Put);

        let mock_tracer = run_public_api_instrumentation_test_with_request_instrumentation(
            Some("MyClient.MyApi"),
            Some("test.namespace"),
            Some("test_crate"),
            Some("1.0.0"),
            &mut request,
            |req| {
                Box::pin(async move {
                    assert_eq!(req.url().host_str(), Some("example.com"));
                    assert_eq!(req.method(), Method::Put);
                    Ok(AsyncRawResponse::from_bytes(
                        StatusCode::Ok,
                        Headers::new(),
                        vec![],
                    ))
                })
            },
        )
        .await;

        let parent_id = Uuid::new_v4();

        check_instrumentation_result(
            mock_tracer.clone(),
            vec![ExpectedTracerInformation {
                name: "test_crate",
                version: Some("1.0.0"),
                namespace: Some("test.namespace"),
                spans: vec![
                    ExpectedSpanInformation {
                        span_name: "MyClient.MyApi",
                        status: SpanStatus::Unset,
                        kind: SpanKind::Internal,
                        span_id: parent_id,
                        parent_id: None,
                        attributes: vec![
                            (AZ_NAMESPACE_ATTRIBUTE, "test.namespace".into()),
                            ("az.fake_attribute", "attribute value".into()),
                        ],
                    },
                    ExpectedSpanInformation {
                        span_name: "PUT",
                        status: SpanStatus::Unset,
                        kind: SpanKind::Client,
                        span_id: Uuid::new_v4(),
                        parent_id: Some(parent_id),
                        attributes: vec![
                            (AZ_NAMESPACE_ATTRIBUTE, "test.namespace".into()),
                            ("http.request.method", "PUT".into()),
                            ("url.full", "http://example.com/path_with_request".into()),
                            ("server.address", "example.com".into()),
                            ("server.port", 80.into()),
                            ("http.response.status_code", 200.into()),
                        ],
                    },
                ],
            }],
        );
    }
}
