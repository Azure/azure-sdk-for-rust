// Copyright (C) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell: ignore azuresdkforcpp invalidtopleveldomain azurewebsites
//! This file contains an Azure SDK for Rust fake service client API.
//!
use azure_core::{
    credentials::TokenCredential,
    fmt::SafeDebug,
    http::{
        BufResponse, ClientMethodOptions, ClientOptions, InstrumentationOptions, Pipeline, Request,
        Url,
    },
    tracing, Result,
};
use azure_core_opentelemetry::OpenTelemetryTracerProvider;
use opentelemetry_sdk::trace::{InMemorySpanExporter, SdkTracerProvider};
use std::sync::Arc;

#[derive(Clone, SafeDebug)]
pub struct TestServiceClientWithMacrosOptions {
    pub client_options: ClientOptions,
    pub api_version: Option<String>,
}

impl Default for TestServiceClientWithMacrosOptions {
    fn default() -> Self {
        Self {
            client_options: ClientOptions::default(),
            api_version: Some("2023-10-01".to_string()),
        }
    }
}

/// Define a TestServiceClient which is a fake service client for testing purposes.
/// This client demonstrates how to implement a service client using the tracing convenience proc macros.
#[tracing::client]
pub struct TestServiceClientWithMacros {
    endpoint: Url,
    api_version: String,
    pipeline: Pipeline,
}

#[derive(Default, SafeDebug)]
pub struct TestServiceClientWithMacrosGetMethodOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

impl TestServiceClientWithMacros {
    /// Creates a new instance of the TestServiceClient.
    ///
    /// This function demonstrates how to create a service client using the tracing convenience proc macros.
    ///
    /// # Arguments
    /// * `endpoint` - The endpoint URL for the service.
    /// * `_credential` - The credential used for authentication (not used in this example).
    /// * `options` - Optional client options to configure the client.
    ///
    #[tracing::new("Az.TestServiceClient")]
    pub fn new(
        endpoint: &str,
        _credential: Arc<dyn TokenCredential>,
        options: Option<TestServiceClientWithMacrosOptions>,
    ) -> Result<Self> {
        let options = options.unwrap_or_default();
        let mut endpoint = Url::parse(endpoint)?;
        if !endpoint.scheme().starts_with("http") {
            return Err(azure_core::Error::message(
                azure_core::error::ErrorKind::Other,
                format!("{endpoint} must use http(s)"),
            ));
        }
        endpoint.set_query(None);

        Ok(Self {
            endpoint,
            api_version: options.api_version.unwrap_or_default(),
            pipeline: Pipeline::new(
                option_env!("CARGO_PKG_NAME"),
                option_env!("CARGO_PKG_VERSION"),
                options.client_options,
                Vec::default(),
                Vec::default(),
                None,
            ),
        })
    }

    /// Returns the Url associated with this client.
    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    pub async fn get(
        &self,
        path: &str,
        options: Option<TestServiceClientWithMacrosGetMethodOptions<'_>>,
    ) -> Result<BufResponse> {
        let options = options.unwrap_or_default();
        let mut url = self.endpoint.clone();
        url.set_path(path);
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);

        let mut request = Request::new(url, azure_core::http::Method::Get);

        let response = self
            .pipeline
            .send(&options.method_options.context, &mut request)
            .await?;
        if !response.status().is_success() {
            return Err(azure_core::Error::message(
                azure_core::error::ErrorKind::HttpResponse {
                    status: response.status(),
                    error_code: None,
                    raw_response: None,
                },
                format!("Failed to GET {}: {}", request.url(), response.status()),
            ));
        }
        Ok(response)
    }

    /// Returns the result of a Get verb against the configured endpoint with the specified path.
    ///
    /// This method demonstrates a service client which has per-method spans and uses the configured tracing
    /// tracing provider to create per-api spans for the function.
    ///
    /// To configure per-api spans, your service implementation needs to do the following:
    /// 1. If the client is configured with a [`Tracer`], it will create a span whose name matches the function.
    ///    1. The span should be created with the `SpanKind::Internal` kind, and
    ///    2. The span should have the `az.namespace` attribute set to the namespace of the service client.
    /// 2. The function should add the span created in step 1 to  the ClientMethodOptions context.
    /// 3. The function should add the tracer to the ClientMethodOptions context so that the pipeline can use it to populate the `az.namespace` property in the request span.
    /// 4. The function should then perform the normal client operations after setting up the context.
    /// 5. After the client operation completes, if the function failed, it should add an `error.type` attribute to the span
    ///    with the error type.
    ///
    /// # Note
    /// This applies to most HTTP client operations, but not all. CosmosDB has its own set of conventions as listed
    /// [here](https://github.com/open-telemetry/semantic-conventions/blob/main/docs/database/cosmosdb.md)
    ///
    #[tracing::function("macros_get_with_tracing",attributes=(a.b=1,az.telemetry="Abc","string attribute"=path))]
    pub async fn get_with_function_tracing(
        &self,
        path: &str,
        options: Option<TestServiceClientWithMacrosGetMethodOptions<'_>>,
    ) -> Result<BufResponse> {
        let options = options.unwrap_or_default();

        let mut url = self.endpoint.clone();
        url.set_path(path);
        url.query_pairs_mut()
            .append_pair("api-version", &self.api_version);

        let mut request = Request::new(url, azure_core::http::Method::Get);

        let response = self
            .pipeline
            .send(&options.method_options.context, &mut request)
            .await?;
        if !response.status().is_success() {
            return Err(azure_core::Error::message(
                azure_core::error::ErrorKind::HttpResponse {
                    status: response.status(),
                    error_code: None,
                    raw_response: None,
                },
                format!("Failed to GET {}: {}", request.url(), response.status()),
            ));
        }
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use ::tracing::{info, trace};
    use azure_core::{
        http::{ExponentialRetryOptions, RetryOptions},
        tracing::TracerProvider,
        Result,
    };
    use azure_core_test::{
        recorded,
        tracing::{ExpectedApiInformation, ExpectedInstrumentation},
        TestContext,
    };
    use opentelemetry::trace::{
        SpanKind as OpenTelemetrySpanKind, Status as OpenTelemetrySpanStatus,
    };
    use opentelemetry::Value as OpenTelemetryAttributeValue;
    use typespec_client_core::http;

    fn create_exportable_tracer_provider() -> (Arc<SdkTracerProvider>, InMemorySpanExporter) {
        let otel_exporter = InMemorySpanExporter::default();
        let otel_tracer_provider = SdkTracerProvider::builder()
            .with_simple_exporter(otel_exporter.clone())
            .build();
        let otel_tracer_provider = Arc::new(otel_tracer_provider);
        (otel_tracer_provider, otel_exporter)
    }

    fn create_service_client(
        ctx: &TestContext,
        azure_provider: Arc<dyn TracerProvider>,
    ) -> TestServiceClientWithMacros {
        let recording = ctx.recording();
        let endpoint = "https://azuresdkforcpp.azurewebsites.net";
        let credential = recording.credential().clone();
        let mut options = TestServiceClientWithMacrosOptions {
            client_options: ClientOptions {
                instrumentation: InstrumentationOptions {
                    tracer_provider: Some(azure_provider),
                },
                ..Default::default()
            },
            ..Default::default()
        };
        recording.instrument(&mut options.client_options);

        TestServiceClientWithMacros::new(endpoint, credential, Some(options)).unwrap()
    }

    // Span verification utility functions.

    struct ExpectedSpan {
        name: &'static str,
        kind: OpenTelemetrySpanKind,
        parent_span_id: Option<opentelemetry::trace::SpanId>,
        status: OpenTelemetrySpanStatus,
        attributes: Vec<(&'static str, OpenTelemetryAttributeValue)>,
    }

    fn verify_span(
        span: &opentelemetry_sdk::trace::SpanData,
        expected: ExpectedSpan,
    ) -> Result<()> {
        assert_eq!(span.name, expected.name);
        assert_eq!(span.span_kind, expected.kind);
        assert_eq!(span.status, expected.status);
        assert_eq!(
            span.parent_span_id,
            expected
                .parent_span_id
                .unwrap_or(opentelemetry::trace::SpanId::INVALID)
        );

        for attr in span.attributes.iter() {
            println!("Attribute: {} = {:?}", attr.key, attr.value);
            let mut found = false;
            for (key, value) in expected.attributes.iter() {
                if attr.key.as_str() == (*key) {
                    found = true;
                    // Skip checking the value for "<ANY>" as it is a placeholder
                    if *value != OpenTelemetryAttributeValue::String("<ANY>".into()) {
                        assert_eq!(attr.value, *value, "Attribute mismatch for key: {}", *key);
                    }
                    break;
                }
            }
            if !found {
                panic!("Unexpected attribute: {} = {:?}", attr.key, attr.value);
            }
        }
        for (key, value) in expected.attributes.iter() {
            if !span.attributes.iter().any(|attr| attr.key == (*key).into()) {
                panic!("Expected attribute not found: {} = {:?}", key, value);
            }
        }

        Ok(())
    }

    // Basic functionality tests.
    #[recorded::test()]
    async fn test_macro_service_client_new(ctx: TestContext) -> Result<()> {
        let recording = ctx.recording();
        let endpoint = "https://microsoft.com";
        let credential = recording.credential().clone();
        let mut options = TestServiceClientWithMacrosOptions {
            ..Default::default()
        };
        recording.instrument(&mut options.client_options);

        let client = TestServiceClientWithMacros::new(endpoint, credential, Some(options)).unwrap();
        assert_eq!(client.endpoint().as_str(), "https://microsoft.com/");
        assert_eq!(client.api_version, "2023-10-01");

        Ok(())
    }

    #[recorded::test()]
    async fn test_macro_service_client_get_simple(ctx: TestContext) -> Result<()> {
        let (sdk_provider, otel_exporter) = create_exportable_tracer_provider();
        let azure_provider = OpenTelemetryTracerProvider::new(sdk_provider);

        let client = create_service_client(&ctx, azure_provider.clone());

        let response = client.get("get", None).await;
        info!("Response: {:?}", response);
        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.status(), azure_core::http::StatusCode::Ok);

        let spans = otel_exporter.get_finished_spans().unwrap();
        assert_eq!(spans.len(), 1);
        for span in &spans {
            trace!("Span: {:?}", span);

            verify_span(
                span,
                ExpectedSpan {
                    name: "GET",
                    kind: OpenTelemetrySpanKind::Client,
                    status: OpenTelemetrySpanStatus::Unset,
                    parent_span_id: None,
                    attributes: vec![
                        ("http.request.method", "GET".into()),
                        ("az.client_request_id", "<ANY>".into()),
                        (
                            "url.full",
                            format!("{}{}", client.endpoint(), "get?api-version=2023-10-01").into(),
                        ),
                        ("server.address", "azuresdkforcpp.azurewebsites.net".into()),
                        ("server.port", 443.into()),
                        ("http.response.status_code", 200.into()),
                    ],
                },
            )?;
        }

        Ok(())
    }

    #[recorded::test()]
    async fn test_macro_service_client_get_with_error(ctx: TestContext) -> Result<()> {
        let (sdk_provider, otel_exporter) = create_exportable_tracer_provider();
        let azure_provider = OpenTelemetryTracerProvider::new(sdk_provider);

        let client = create_service_client(&ctx, azure_provider.clone());

        let response = client.get("failing_url", None).await;
        info!("Response: {:?}", response);

        let spans = otel_exporter.get_finished_spans().unwrap();
        assert_eq!(spans.len(), 1);
        for span in &spans {
            trace!("Span: {:?}", span);

            verify_span(
                span,
                ExpectedSpan {
                    name: "GET",
                    kind: OpenTelemetrySpanKind::Client,
                    parent_span_id: None,
                    status: OpenTelemetrySpanStatus::Error {
                        description: "".into(),
                    },
                    attributes: vec![
                        ("http.request.method", "GET".into()),
                        ("az.client_request_id", "<ANY>".into()),
                        (
                            "url.full",
                            format!(
                                "{}{}",
                                client.endpoint(),
                                "failing_url?api-version=2023-10-01"
                            )
                            .into(),
                        ),
                        ("server.address", "azuresdkforcpp.azurewebsites.net".into()),
                        ("server.port", 443.into()),
                        ("error.type", "404".into()),
                        ("http.response.status_code", 404.into()),
                    ],
                },
            )?;
        }

        Ok(())
    }

    #[recorded::test()]
    async fn test_macro_service_client_get_with_function_tracing(ctx: TestContext) -> Result<()> {
        let (sdk_provider, otel_exporter) = create_exportable_tracer_provider();
        let azure_provider = OpenTelemetryTracerProvider::new(sdk_provider);

        let client = create_service_client(&ctx, azure_provider.clone());

        let response = client.get_with_function_tracing("get", None).await;
        info!("Response: {:?}", response);

        let spans = otel_exporter.get_finished_spans().unwrap();
        assert_eq!(spans.len(), 2);
        for span in &spans {
            trace!("Span: {:?}", span);
        }
        verify_span(
            &spans[0],
            ExpectedSpan {
                name: "GET",
                kind: OpenTelemetrySpanKind::Client,
                parent_span_id: Some(spans[1].span_context.span_id()),
                status: OpenTelemetrySpanStatus::Unset,
                attributes: vec![
                    ("http.request.method", "GET".into()),
                    ("az.namespace", "Az.TestServiceClient".into()),
                    ("az.client_request_id", "<ANY>".into()),
                    (
                        "url.full",
                        format!("{}{}", client.endpoint(), "get?api-version=2023-10-01").into(),
                    ),
                    ("server.address", "azuresdkforcpp.azurewebsites.net".into()),
                    ("server.port", 443.into()),
                    ("http.response.status_code", 200.into()),
                ],
            },
        )?;
        verify_span(
            &spans[1],
            ExpectedSpan {
                name: "macros_get_with_tracing",
                kind: OpenTelemetrySpanKind::Internal,
                parent_span_id: None,
                status: OpenTelemetrySpanStatus::Unset,
                attributes: vec![
                    ("az.namespace", "Az.TestServiceClient".into()),
                    ("a.b", 1.into()),                  // added by tracing macro.
                    ("az.telemetry", "Abc".into()),     // added by tracing macro
                    ("string attribute", "get".into()), // added by tracing macro.
                ],
            },
        )?;

        Ok(())
    }

    #[recorded::test()]
    async fn test_macro_service_client_get_function_tracing_error(ctx: TestContext) -> Result<()> {
        let (sdk_provider, otel_exporter) = create_exportable_tracer_provider();
        let azure_provider = OpenTelemetryTracerProvider::new(sdk_provider);

        let client = create_service_client(&ctx, azure_provider.clone());

        let response = client.get_with_function_tracing("failing_url", None).await;
        info!("Response: {:?}", response);

        let spans = otel_exporter.get_finished_spans().unwrap();
        assert_eq!(spans.len(), 2);
        for span in &spans {
            trace!("Span: {:?}", span);
        }
        verify_span(
            &spans[0],
            ExpectedSpan {
                name: "GET",
                kind: OpenTelemetrySpanKind::Client,
                parent_span_id: Some(spans[1].span_context.span_id()),
                status: OpenTelemetrySpanStatus::Error {
                    description: "".into(),
                },
                attributes: vec![
                    ("http.request.method", "GET".into()),
                    ("az.namespace", "Az.TestServiceClient".into()),
                    ("az.client_request_id", "<ANY>".into()),
                    (
                        "url.full",
                        format!(
                            "{}{}",
                            client.endpoint(),
                            "failing_url?api-version=2023-10-01"
                        )
                        .into(),
                    ),
                    ("server.address", "azuresdkforcpp.azurewebsites.net".into()),
                    ("server.port", 443.into()),
                    ("http.response.status_code", 404.into()),
                    ("error.type", "404".into()),
                ],
            },
        )?;
        verify_span(
            &spans[1],
            ExpectedSpan {
                name: "macros_get_with_tracing",
                kind: OpenTelemetrySpanKind::Internal,
                parent_span_id: None,
                status: OpenTelemetrySpanStatus::Unset,
                attributes: vec![
                    ("az.namespace", "Az.TestServiceClient".into()),
                    ("error.type", "404".into()),
                    ("a.b", 1.into()),              // added by tracing macro.
                    ("az.telemetry", "Abc".into()), // added by tracing macro
                    ("string attribute", "failing_url".into()), // added by tracing macro.
                ],
            },
        )?;

        Ok(())
    }

    #[recorded::test()]
    async fn test_macro_service_client_get_function_tracing_dns_error(
        ctx: TestContext,
    ) -> Result<()> {
        let (sdk_provider, otel_exporter) = create_exportable_tracer_provider();
        let azure_provider = OpenTelemetryTracerProvider::new(sdk_provider);

        let recording = ctx.recording();
        let endpoint = "https://azuresdkforcpp.azurewebsites.invalidtopleveldomain";
        let credential = recording.credential().clone();
        let options = TestServiceClientWithMacrosOptions {
            client_options: ClientOptions {
                instrumentation: InstrumentationOptions {
                    tracer_provider: Some(azure_provider),
                },
                retry: RetryOptions::exponential(ExponentialRetryOptions {
                    max_retries: 3,
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        };

        let client = TestServiceClientWithMacros::new(endpoint, credential, Some(options)).unwrap();
        let response = client.get_with_function_tracing("failing_url", None).await;
        info!("Response: {:?}", response);

        let spans = otel_exporter.get_finished_spans().unwrap();
        assert_eq!(spans.len(), 5);
        for span in &spans {
            trace!("Span: {:?}", span);
        }
        verify_span(
            &spans[0],
            ExpectedSpan {
                name: "GET",
                kind: OpenTelemetrySpanKind::Client,
                parent_span_id: Some(spans[4].span_context.span_id()),
                status: OpenTelemetrySpanStatus::Unset,
                attributes: vec![
                    ("http.request.method", "GET".into()),
                    ("az.namespace", "Az.TestServiceClient".into()),
                    ("az.client_request_id", "<ANY>".into()),
                    (
                        "url.full",
                        format!(
                            "{}{}",
                            client.endpoint(),
                            "failing_url?api-version=2023-10-01"
                        )
                        .into(),
                    ),
                    (
                        "server.address",
                        "azuresdkforcpp.azurewebsites.invalidtopleveldomain".into(),
                    ),
                    ("server.port", 443.into()),
                    ("error.type", "Io".into()),
                ],
            },
        )?;
        verify_span(
            &spans[1],
            ExpectedSpan {
                name: "GET",
                kind: OpenTelemetrySpanKind::Client,
                parent_span_id: Some(spans[4].span_context.span_id()),
                status: OpenTelemetrySpanStatus::Unset,
                attributes: vec![
                    ("http.request.method", "GET".into()),
                    ("az.namespace", "Az.TestServiceClient".into()),
                    ("az.client_request_id", "<ANY>".into()),
                    (
                        "url.full",
                        format!(
                            "{}{}",
                            client.endpoint(),
                            "failing_url?api-version=2023-10-01"
                        )
                        .into(),
                    ),
                    (
                        "server.address",
                        "azuresdkforcpp.azurewebsites.invalidtopleveldomain".into(),
                    ),
                    ("server.port", 443.into()),
                    ("http.request.resend_count", 1.into()),
                    ("error.type", "Io".into()),
                ],
            },
        )?;
        verify_span(
            &spans[2],
            ExpectedSpan {
                name: "GET",
                kind: OpenTelemetrySpanKind::Client,
                parent_span_id: Some(spans[4].span_context.span_id()),
                status: OpenTelemetrySpanStatus::Unset,
                attributes: vec![
                    ("http.request.method", "GET".into()),
                    ("az.namespace", "Az.TestServiceClient".into()),
                    ("az.client_request_id", "<ANY>".into()),
                    (
                        "url.full",
                        format!(
                            "{}{}",
                            client.endpoint(),
                            "failing_url?api-version=2023-10-01"
                        )
                        .into(),
                    ),
                    (
                        "server.address",
                        "azuresdkforcpp.azurewebsites.invalidtopleveldomain".into(),
                    ),
                    ("server.port", 443.into()),
                    ("http.request.resend_count", 2.into()),
                    ("error.type", "Io".into()),
                ],
            },
        )?;
        verify_span(
            &spans[3],
            ExpectedSpan {
                name: "GET",
                kind: OpenTelemetrySpanKind::Client,
                parent_span_id: Some(spans[4].span_context.span_id()),
                status: OpenTelemetrySpanStatus::Unset,
                attributes: vec![
                    ("http.request.method", "GET".into()),
                    ("az.namespace", "Az.TestServiceClient".into()),
                    ("az.client_request_id", "<ANY>".into()),
                    (
                        "url.full",
                        format!(
                            "{}{}",
                            client.endpoint(),
                            "failing_url?api-version=2023-10-01"
                        )
                        .into(),
                    ),
                    (
                        "server.address",
                        "azuresdkforcpp.azurewebsites.invalidtopleveldomain".into(),
                    ),
                    ("server.port", 443.into()),
                    ("http.request.resend_count", 3.into()),
                    ("error.type", "Io".into()),
                ],
            },
        )?;

        verify_span(
            &spans[4],
            ExpectedSpan {
                name: "macros_get_with_tracing",
                kind: OpenTelemetrySpanKind::Internal,
                parent_span_id: None,
                status: OpenTelemetrySpanStatus::Error {
                    description: "Io".into(),
                },
                attributes: vec![
                    ("az.namespace", "Az.TestServiceClient".into()),
                    ("error.type", "Io".into()),
                    ("a.b", 1.into()),              // added by tracing macro.
                    ("az.telemetry", "Abc".into()), // added by tracing macro
                    ("string attribute", "failing_url".into()), // added by tracing macro.
                ],
            },
        )?;

        Ok(())
    }

    #[recorded::test()]
    async fn test_http_tracing_tests(ctx: TestContext) -> Result<()> {
        let recording = ctx.recording();
        let package_name = recording.var("CARGO_PKG_NAME", None);
        // Compare current version since recorded version may be older.
        let package_version = env!("CARGO_PKG_VERSION").to_string();
        azure_core_test::tracing::assert_instrumentation_information(
            |tracer_provider| Ok(create_service_client(&ctx, tracer_provider)),
            |client| {
                let client = client;
                Box::pin(async move { client.get("get", None).await })
            },
            ExpectedInstrumentation {
                package_name,
                package_version,
                package_namespace: Some("Az.TestServiceClient"),
                api_calls: vec![ExpectedApiInformation {
                    api_name: None,
                    ..Default::default()
                }],
            },
        )
        .await?;

        Ok(())
    }

    #[recorded::test()]
    async fn test_function_tracing_tests(ctx: TestContext) -> Result<()> {
        let package_name = env!("CARGO_PKG_NAME").to_string();
        let package_version = env!("CARGO_PKG_VERSION").to_string();
        azure_core_test::tracing::assert_instrumentation_information(
            |tracer_provider| Ok(create_service_client(&ctx, tracer_provider)),
            |client| {
                let client = client;
                Box::pin(async move { client.get_with_function_tracing("get", None).await })
            },
            ExpectedInstrumentation {
                package_name,
                package_version,
                package_namespace: Some("Az.TestServiceClient"),
                api_calls: vec![ExpectedApiInformation {
                    api_name: Some("macros_get_with_tracing"),
                    additional_api_attributes: vec![
                        ("a.b", 1.into()),
                        ("az.telemetry", "Abc".into()),
                        ("string attribute", "get".into()),
                    ],
                    ..Default::default()
                }],
            },
        )
        .await?;

        Ok(())
    }
    #[recorded::test()]
    async fn test_function_tracing_tests_error(ctx: TestContext) -> Result<()> {
        let package_name = env!("CARGO_PKG_NAME").to_string();
        let package_version = env!("CARGO_PKG_VERSION").to_string();
        azure_core_test::tracing::assert_instrumentation_information(
            |tracer_provider| Ok(create_service_client(&ctx, tracer_provider)),
            |client| {
                let client = client;
                Box::pin(async move { client.get_with_function_tracing("index.htm", None).await })
            },
            ExpectedInstrumentation {
                package_name,
                package_version,
                package_namespace: Some("Az.TestServiceClient"),
                api_calls: vec![ExpectedApiInformation {
                    api_name: Some("macros_get_with_tracing"),
                    expected_status_code: http::StatusCode::NotFound,
                    additional_api_attributes: vec![
                        ("a.b", 1.into()),
                        ("az.telemetry", "Abc".into()),
                        ("string attribute", "index.htm".into()),
                    ],
                    ..Default::default()
                }],
            },
        )
        .await?;

        Ok(())
    }
}
