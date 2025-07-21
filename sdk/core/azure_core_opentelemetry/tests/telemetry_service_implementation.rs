// Copyright (C) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! This file contains an Azure SDK for Rust fake service client API.
//!
use azure_core::{
    credentials::TokenCredential,
    fmt::SafeDebug,
    http::{
        ClientMethodOptions, ClientOptions, Pipeline, RawResponse, Request,
        RequestInstrumentationOptions, Url,
    },
    tracing::{PublicApiInstrumentationInformation, Tracer},
    Result,
};
use azure_core_opentelemetry::OpenTelemetryTracerProvider;
use opentelemetry_sdk::trace::{InMemorySpanExporter, SdkTracerProvider};
use std::sync::Arc;

#[derive(Clone, SafeDebug)]
pub struct TestServiceClientOptions {
    pub azure_client_options: ClientOptions,
    pub api_version: Option<String>,
}

impl Default for TestServiceClientOptions {
    fn default() -> Self {
        Self {
            azure_client_options: ClientOptions::default(),
            api_version: Some("2023-10-01".to_string()),
        }
    }
}

pub struct TestServiceClient {
    endpoint: Url,
    api_version: String,
    pipeline: Pipeline,
    tracer: Option<Arc<dyn Tracer>>,
}

#[derive(Default, SafeDebug)]
pub struct TestServiceClientGetMethodOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

impl TestServiceClient {
    pub fn new(
        endpoint: &str,
        _credential: Arc<dyn TokenCredential>,
        options: Option<TestServiceClientOptions>,
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

        let tracer =
            if let Some(tracer_options) = &options.azure_client_options.request_instrumentation {
                tracer_options
                    .tracer_provider
                    .as_ref()
                    .map(|tracer_provider| {
                        tracer_provider.get_tracer(
                            Some("Az.TestServiceClient"),
                            option_env!("CARGO_PKG_NAME").unwrap_or("UNKNOWN"),
                            option_env!("CARGO_PKG_VERSION"),
                        )
                    })
            } else {
                None
            };

        Ok(Self {
            endpoint,
            api_version: options.api_version.unwrap_or_default(),
            pipeline: Pipeline::new(
                option_env!("CARGO_PKG_NAME"),
                option_env!("CARGO_PKG_VERSION"),
                options.azure_client_options,
                Vec::default(),
                Vec::default(),
            ),
            tracer,
        })
    }

    /// Returns the Url associated with this client.
    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    /// Returns the result of a Get verb against the configured endpoint with the specified path.
    ///
    /// This method demonstrates a service client which does not have per-method spans but which will create
    /// HTTP client spans if the `RequestInstrumentationOptions` are configured in the client options.
    ///
    pub async fn get(
        &self,
        path: &str,
        options: Option<TestServiceClientGetMethodOptions<'_>>,
    ) -> Result<RawResponse> {
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
    pub async fn get_with_function_tracing(
        &self,
        path: &str,
        options: Option<TestServiceClientGetMethodOptions<'_>>,
    ) -> Result<RawResponse> {
        let mut options = options.unwrap_or_default();

        let public_api_info = PublicApiInstrumentationInformation {
            api_name: "get_with_tracing",
            attributes: vec![],
        };
        // Add the span to the tracer.
        let mut ctx = options.method_options.context.with_value(public_api_info);
        // If the service has a tracer, we add it to the context.
        if let Some(tracer) = &self.tracer {
            ctx = ctx.with_value(tracer.clone());
        }
        options.method_options.context = ctx;
        self.get(path, Some(options)).await
    }
}

use azure_core_test::{recorded, TestContext};
use opentelemetry::trace::{SpanKind as OpenTelemetrySpanKind, Status as OpenTelemetrySpanStatus};
use opentelemetry::Value as OpenTelemetryAttributeValue;
use tracing::{info, trace};

fn create_exportable_tracer_provider() -> (Arc<SdkTracerProvider>, InMemorySpanExporter) {
    let otel_exporter = InMemorySpanExporter::default();
    let otel_tracer_provider = SdkTracerProvider::builder()
        .with_simple_exporter(otel_exporter.clone())
        .build();
    let otel_tracer_provider = Arc::new(otel_tracer_provider);
    (otel_tracer_provider, otel_exporter)
}

// Span verification utility functions.

struct ExpectedSpan {
    name: &'static str,
    kind: OpenTelemetrySpanKind,
    parent_span_id: Option<opentelemetry::trace::SpanId>,
    status: OpenTelemetrySpanStatus,
    attributes: Vec<(&'static str, OpenTelemetryAttributeValue)>,
}

fn verify_span(span: &opentelemetry_sdk::trace::SpanData, expected: ExpectedSpan) -> Result<()> {
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
async fn test_service_client_new(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let endpoint = "https://example.com";
    let credential = recording.credential().clone();
    let options = TestServiceClientOptions {
        ..Default::default()
    };

    let client = TestServiceClient::new(endpoint, credential, Some(options)).unwrap();
    assert_eq!(client.endpoint().as_str(), "https://example.com/");
    assert_eq!(client.api_version, "2023-10-01");

    Ok(())
}

// Ensure that the the test client actually does what it's supposed to do without telemetry.
#[recorded::test()]
async fn test_service_client_get(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let endpoint = "https://example.com";
    let credential = recording.credential().clone();

    let client = TestServiceClient::new(endpoint, credential, None).unwrap();
    let response = client.get("index.html", None).await;
    info!("Response: {:?}", response);
    assert!(response.is_ok());
    let response = response.unwrap();
    assert_eq!(response.status(), azure_core::http::StatusCode::Ok);
    Ok(())
}

#[recorded::test()]
async fn test_service_client_get_with_tracing(ctx: TestContext) -> Result<()> {
    let (sdk_provider, otel_exporter) = create_exportable_tracer_provider();
    let azure_provider = OpenTelemetryTracerProvider::new(sdk_provider);

    let recording = ctx.recording();
    let endpoint = "https://example.com";
    let credential = recording.credential().clone();
    let options = TestServiceClientOptions {
        azure_client_options: ClientOptions {
            request_instrumentation: Some(RequestInstrumentationOptions {
                tracer_provider: Some(azure_provider),
            }),
            ..Default::default()
        },
        ..Default::default()
    };

    let client = TestServiceClient::new(endpoint, credential, Some(options)).unwrap();
    let response = client.get("index.html", None).await;
    info!("Response: {:?}", response);
    assert!(response.is_ok());
    let response = response.unwrap();
    assert_eq!(response.status(), azure_core::http::StatusCode::Ok);

    let spans = otel_exporter.get_finished_spans().unwrap();
    for (i, span) in spans.iter().enumerate() {
        trace!("Span {i}: {span:?}");
    }
    assert_eq!(spans.len(), 1);
    verify_span(
        &spans[0],
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
                    format!(
                        "{}{}",
                        client.endpoint(),
                        "index.html?api-version=2023-10-01"
                    )
                    .into(),
                ),
                ("server.address", "example.com".into()),
                ("server.port", 443.into()),
                ("http.response.status_code", 200.into()),
            ],
        },
    )?;

    Ok(())
}

#[recorded::test()]
async fn test_service_client_get_with_tracing_error(ctx: TestContext) -> Result<()> {
    let (sdk_provider, otel_exporter) = create_exportable_tracer_provider();
    let azure_provider = OpenTelemetryTracerProvider::new(sdk_provider);

    let recording = ctx.recording();
    let endpoint = "https://example.com";
    let credential = recording.credential().clone();
    let options = TestServiceClientOptions {
        azure_client_options: ClientOptions {
            request_instrumentation: Some(RequestInstrumentationOptions {
                tracer_provider: Some(azure_provider),
            }),
            ..Default::default()
        },
        ..Default::default()
    };

    let client = TestServiceClient::new(endpoint, credential, Some(options)).unwrap();
    let response = client.get("failing_url", None).await;
    info!("Response: {:?}", response);

    let spans = otel_exporter.get_finished_spans().unwrap();
    for (i, span) in spans.iter().enumerate() {
        trace!("Span {i}: {span:?}");
    }
    assert_eq!(spans.len(), 1);

    verify_span(
        &spans[0],
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
                ("server.address", "example.com".into()),
                ("server.port", 443.into()),
                ("error.type", "404".into()),
                ("http.response.status_code", 404.into()),
            ],
        },
    )?;

    Ok(())
}

#[recorded::test()]
async fn test_service_client_get_with_function_tracing(ctx: TestContext) -> Result<()> {
    let (sdk_provider, otel_exporter) = create_exportable_tracer_provider();
    let azure_provider = OpenTelemetryTracerProvider::new(sdk_provider);

    let recording = ctx.recording();
    let endpoint = "https://example.com";
    let credential = recording.credential().clone();
    let options = TestServiceClientOptions {
        azure_client_options: ClientOptions {
            request_instrumentation: Some(RequestInstrumentationOptions {
                tracer_provider: Some(azure_provider),
            }),
            ..Default::default()
        },
        ..Default::default()
    };

    let client = TestServiceClient::new(endpoint, credential, Some(options)).unwrap();
    let response = client.get_with_function_tracing("index.html", None).await;
    info!("Response: {:?}", response);

    let spans = otel_exporter.get_finished_spans().unwrap();
    for (i, span) in spans.iter().enumerate() {
        trace!("Span {i}: {span:?}");
    }
    assert_eq!(spans.len(), 2);
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
                    format!(
                        "{}{}",
                        client.endpoint(),
                        "index.html?api-version=2023-10-01"
                    )
                    .into(),
                ),
                ("server.address", "example.com".into()),
                ("server.port", 443.into()),
                ("http.response.status_code", 200.into()),
            ],
        },
    )?;
    verify_span(
        &spans[1],
        ExpectedSpan {
            name: "get_with_tracing",
            kind: OpenTelemetrySpanKind::Internal,
            parent_span_id: None,
            status: OpenTelemetrySpanStatus::Unset,
            attributes: vec![("az.namespace", "Az.TestServiceClient".into())],
        },
    )?;

    Ok(())
}

#[recorded::test()]
async fn test_service_client_get_with_function_tracing_error(ctx: TestContext) -> Result<()> {
    let (sdk_provider, otel_exporter) = create_exportable_tracer_provider();
    let azure_provider = OpenTelemetryTracerProvider::new(sdk_provider);

    let recording = ctx.recording();
    let endpoint = "https://example.com";
    let credential = recording.credential().clone();
    let options = TestServiceClientOptions {
        azure_client_options: ClientOptions {
            request_instrumentation: Some(RequestInstrumentationOptions {
                tracer_provider: Some(azure_provider),
            }),
            ..Default::default()
        },
        ..Default::default()
    };

    let client = TestServiceClient::new(endpoint, credential, Some(options)).unwrap();
    let response = client.get_with_function_tracing("failing_url", None).await;
    info!("Response: {:?}", response);

    let spans = otel_exporter.get_finished_spans().unwrap();
    for (i, span) in spans.iter().enumerate() {
        trace!("Span {i}: {span:?}");
    }
    assert_eq!(spans.len(), 2);
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
                ("server.address", "example.com".into()),
                ("server.port", 443.into()),
                ("http.response.status_code", 404.into()),
                ("error.type", "404".into()),
            ],
        },
    )?;
    verify_span(
        &spans[1],
        ExpectedSpan {
            name: "get_with_tracing",
            kind: OpenTelemetrySpanKind::Internal,
            parent_span_id: None,
            status: OpenTelemetrySpanStatus::Unset,
            attributes: vec![
                ("az.namespace", "Az.TestServiceClient".into()),
                ("error.type", "404".into()),
            ],
        },
    )?;

    Ok(())
}
