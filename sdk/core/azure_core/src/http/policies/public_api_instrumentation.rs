// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    http::{Context, Request},
    tracing::{Span, SpanKind, Tracer},
};
use std::sync::Arc;
use typespec_client_core::{
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
#[derive(Debug)]
pub struct PublicApiInstrumentationInformation {
    /// The name of the API being instrumented.
    ///
    /// The API name should be in the form of <client>.<api>, where
    /// `<client>` is the name of the service client and `<api>` is the name of the API.
    ///
    /// For example, if the service client is `MyClient` and the API is `my_api`,
    /// the API name should be `MyClient.my_api`.
    pub api_name: &'static str,

    /// Additional attributes to be added to the span for this API.
    ///
    /// These attributes can provide additional information about the API being instrumented.
    /// See [Library-specific attributes](https://github.com/Azure/azure-sdk/blob/main/docs/tracing/distributed-tracing-conventions.md#library-specific-attributes)
    /// for more information.
    ///
    pub attributes: Vec<Attribute>,
}

const AZ_NAMESPACE_ATTRIBUTE: &str = "az.namespace";
const ERROR_TYPE_ATTRIBUTE: &str = "error.type";

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

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl Policy for PublicApiInstrumentationPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        // If there is a span in the context, we're a nested call, so we just want to forward the request.
        if ctx.value::<Arc<dyn Span>>().is_some() {
            return next[0].send(ctx, request, &next[1..]).await;
        }

        // We're not a nested call, so we can proceed with instrumentation.
        // We first check if the context has public API instrumentation information.
        // If it does, we use that information to create a span for the request.
        // If it doesn't, we skip instrumentation.
        let public_api_information = ctx.value::<PublicApiInstrumentationInformation>();
        let span: Option<Arc<dyn Span>> = if let Some(info) = public_api_information {
            // Use the public API information for instrumentation.
            // If the context has a tracer (which happens when called from an instrumented method),
            // we prefer the tracer from the context.
            // Otherwise, we use the tracer from the policy itself.
            // This allows for flexibility in using different tracers in different contexts.
            let mut span_attributes = vec![];

            for attr in &info.attributes {
                // Add the attributes from the public API information to the span.
                span_attributes.push(Attribute {
                    key: attr.key,
                    value: attr.value.clone(),
                });
            }

            if let Some(tracer) = ctx.value::<Arc<dyn Tracer>>() {
                if let Some(namespace) = tracer.namespace() {
                    // If the tracer has a namespace, we set it as an attribute.
                    span_attributes.push(Attribute {
                        key: AZ_NAMESPACE_ATTRIBUTE,
                        value: namespace.into(),
                    });
                }
                // Create a span with the public API information.
                Some(tracer.start_span_with_current(
                    info.api_name,
                    SpanKind::Internal,
                    span_attributes,
                ))
            } else if let Some(tracer) = &self.tracer {
                // We didn't have a span from the context, but we do have a Tracer from the
                // pipeline construction, so use that.
                Some(tracer.start_span_with_current(
                    info.api_name,
                    SpanKind::Internal,
                    span_attributes,
                ))
            } else {
                // If no tracer is available, we skip instrumentation.
                None
            }
        } else {
            None
        };

        // Now add the span to the context, so that it can be used by the next policies.
        let mut ctx = ctx.clone();
        if let Some(span) = &span {
            // If we have a span, we set it in the context.
            ctx = ctx.with_value(span.clone());
        }

        let result = next[0].send(&ctx, request, &next[1..]).await;

        if let Some(span) = span {
            if let Err(e) = &result {
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
            } else if let Ok(response) = &result {
                // 5xx status codes SHOULD set status to Error.
                // The description should not be set because it can be inferred from "http.response.status_code".
                if response.status().is_server_error() {
                    span.set_status(crate::tracing::SpanStatus::Error {
                        description: "".to_string(),
                    });
                }
                if response.status().is_client_error() || response.status().is_server_error() {
                    span.set_attribute(ERROR_TYPE_ATTRIBUTE, response.status().to_string().into());
                }
            }

            span.end();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    // cspell: ignore traceparent
    use super::super::request_instrumentation::tests::{
        check_request_instrumentation_result, InstrumentationExpectation, MockTracingProvider,
    };
    use super::*;
    use crate::{
        http::{
            headers::Headers,
            policies::{RequestInstrumentationPolicy, TransportPolicy},
            Method, RawResponse, StatusCode, TransportOptions,
        },
        tracing::{AttributeValue, SpanStatus, TracerProvider},
        Result,
    };
    use azure_core_test::http::MockHttpClient;
    use futures::future::BoxFuture;
    use std::sync::Arc;

    // Test just the public API instrumentation policy without request instrumentation.
    async fn run_public_api_instrumentation_test<C>(
        api_name: Option<&'static str>,
        request: &mut Request,
        callback: C,
    ) -> Arc<MockTracingProvider>
    where
        C: FnMut(&Request) -> BoxFuture<'_, Result<RawResponse>> + Send + Sync + 'static,
    {
        let public_api_information = PublicApiInstrumentationInformation {
            api_name: api_name.unwrap_or("unknown"),
            attributes: Vec::new(),
        };
        // Add the public API information and tracer to the context so that it can be used by the policy.
        let mock_tracer_provider = Arc::new(MockTracingProvider::new());
        let tracer = mock_tracer_provider.get_tracer(Some("test namespace"), "test_crate", "1.0.0");

        let public_api_policy = Arc::new(PublicApiInstrumentationPolicy::new(Some(tracer.clone())));

        let transport = TransportPolicy::new(TransportOptions::new(Arc::new(MockHttpClient::new(
            callback,
        ))));

        let next: Vec<Arc<dyn Policy>> = vec![Arc::new(transport)];
        let ctx = Context::default()
            .with_value(public_api_information)
            .with_value(tracer.clone());
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
        C: FnMut(&Request) -> BoxFuture<'_, Result<RawResponse>> + Send + Sync + 'static,
    {
        let mock_tracer_provider = Arc::new(MockTracingProvider::new());
        let mock_tracer = mock_tracer_provider.get_tracer(
            namespace,
            crate_name.unwrap_or("unknown"),
            version.unwrap_or("unknown"),
        );

        let public_api_policy = Arc::new(PublicApiInstrumentationPolicy::new(Some(
            mock_tracer.clone(),
        )));

        let transport = TransportPolicy::new(TransportOptions::new(Arc::new(MockHttpClient::new(
            callback,
        ))));

        let request_instrumentation_policy =
            RequestInstrumentationPolicy::new(Some(mock_tracer.clone()));

        let next: Vec<Arc<dyn Policy>> = vec![
            Arc::new(request_instrumentation_policy),
            Arc::new(transport),
        ];
        let public_api_information = PublicApiInstrumentationInformation {
            api_name: api_name.unwrap_or("unknown"),
            attributes: vec![Attribute {
                key: "az.fake_attribute",
                value: "attribute value".into(),
            }],
        };

        // Add the public API information and tracer to the context so that it can be used by the policy.
        let ctx = Context::default()
            .with_value(public_api_information)
            .with_value(mock_tracer.clone());
        let _result = public_api_policy.send(&ctx, request, &next).await;

        mock_tracer_provider
    }

    fn check_public_api_instrumentation_result(
        mock_tracer: Arc<MockTracingProvider>,
        span_count: usize,
        span_index: usize,
        expected_api_name: Option<&str>,
        expected_kind: SpanKind,
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
        let spans = tracer.spans.lock().unwrap();
        assert_eq!(spans.len(), span_count, "Expected one span to be created");
        println!("Spans: {:?}", spans);
        let span = spans[span_index].as_ref();
        assert_eq!(span.name, expected_api_name.unwrap_or("unknown"));
        assert_eq!(span.kind, expected_kind);
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
    async fn simple_public_api_instrumentation_policy() {
        let url = "http://example.com/path";
        let mut request = Request::new(url.parse().unwrap(), Method::Get);

        let mock_tracer =
            run_public_api_instrumentation_test(Some("MyClient.MyApi"), &mut request, |req| {
                Box::pin(async move {
                    assert_eq!(req.url().host_str(), Some("example.com"));
                    assert_eq!(req.method(), &Method::Get);
                    Ok(RawResponse::from_bytes(
                        StatusCode::Ok,
                        Headers::new(),
                        vec![],
                    ))
                })
            })
            .await;

        check_public_api_instrumentation_result(
            mock_tracer,
            1,
            0,
            Some("MyClient.MyApi"),
            SpanKind::Internal,
            SpanStatus::Unset,
            vec![(AZ_NAMESPACE_ATTRIBUTE, "test namespace".into())],
        );
    }

    #[tokio::test]
    async fn public_api_instrumentation_policy_with_error() {
        let url = "http://example.com/path";
        let mut request = Request::new(url.parse().unwrap(), Method::Get);

        let mock_tracer =
            run_public_api_instrumentation_test(Some("MyClient.MyApi"), &mut request, |req| {
                Box::pin(async move {
                    assert_eq!(req.url().host_str(), Some("example.com"));
                    assert_eq!(req.method(), &Method::Get);
                    Ok(RawResponse::from_bytes(
                        StatusCode::InternalServerError,
                        Headers::new(),
                        vec![],
                    ))
                })
            })
            .await;

        check_public_api_instrumentation_result(
            mock_tracer,
            1,
            0,
            Some("MyClient.MyApi"),
            SpanKind::Internal,
            SpanStatus::Error {
                description: "".to_string(),
            },
            vec![
                (AZ_NAMESPACE_ATTRIBUTE, "test namespace".into()),
                (ERROR_TYPE_ATTRIBUTE, "500".into()),
            ],
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
                    assert_eq!(req.method(), &Method::Put);
                    Ok(RawResponse::from_bytes(
                        StatusCode::Ok,
                        Headers::new(),
                        vec![],
                    ))
                })
            },
        )
        .await;

        check_public_api_instrumentation_result(
            mock_tracer.clone(),
            2,
            0,
            Some("MyClient.MyApi"),
            SpanKind::Internal,
            SpanStatus::Unset,
            vec![
                (AZ_NAMESPACE_ATTRIBUTE, "test.namespace".into()),
                // Attribute comes from the public API information.
                ("az.fake_attribute", "attribute value".into()),
            ],
        );

        check_request_instrumentation_result(
            mock_tracer.clone(),
            2,
            1,
            InstrumentationExpectation {
                namespace: Some("test.namespace"),
                name: "test_crate",
                version: "1.0.0",
                span_name: "PUT",
                kind: SpanKind::Client,
                status: SpanStatus::Unset,
                attributes: vec![
                    (AZ_NAMESPACE_ATTRIBUTE, "test.namespace".into()),
                    ("http.request.method", "PUT".into()),
                    ("url.full", "http://example.com/path_with_request".into()),
                    ("server.address", "example.com".into()),
                    ("server.port", 80.into()),
                    ("http.response.status_code", 200.into()),
                ],
            },
        );
    }
}
