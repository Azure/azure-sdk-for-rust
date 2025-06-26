// Copyright (C) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! This file contains an Azure SDK for Rust fake service client API.
//!
use azure_core::{
    credentials::TokenCredential,
    fmt::SafeDebug,
    http::{
        policies::{BearerTokenCredentialPolicy, Policy},
        ClientMethodOptions, ClientOptions, Pipeline, RawResponse, Request,
        RequestInstrumentationOptions, Url,
    },
    Result,
};
use azure_core_opentelemetry::OpenTelemetryTracerProvider;
use opentelemetry_sdk::trace::{InMemorySpanExporter, SdkTracerProvider};
use std::sync::Arc;

#[derive(Default, Clone, SafeDebug)]
pub struct TestServiceClientOptions {
    pub azure_client_options: ClientOptions,
    pub api_version: String,
}

pub struct TestServiceClient {
    endpoint: Url,
    api_version: String,
    pipeline: Pipeline,
}

#[derive(Default, SafeDebug)]
pub struct TestServiceClientGetMethodOptions<'a> {
    pub method_options: ClientMethodOptions<'a>,
}

impl TestServiceClient {
    pub fn new(
        endpoint: &str,
        credential: Arc<dyn TokenCredential>,
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
        let auth_policy: Arc<dyn Policy> = Arc::new(BearerTokenCredentialPolicy::new(
            credential,
            vec!["https://vault.azure.net/.default"],
        ));
        let request_instrumentation_policy = Arc::new(
            azure_core::http::policies::RequestInstrumentationPolicy::new(
                option_env!("CARGO_PKG_NAME"),
                option_env!("CARGO_PKG_VERSION"),
                options
                    .azure_client_options
                    .request_instrumentation
                    .as_ref(),
            ),
        );
        Ok(Self {
            endpoint,
            api_version: options.api_version,
            pipeline: Pipeline::new(
                option_env!("CARGO_PKG_NAME"),
                option_env!("CARGO_PKG_VERSION"),
                options.azure_client_options,
                Vec::default(),
                vec![auth_policy, request_instrumentation_policy],
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::Result;
    use azure_core_test::{recorded, TestContext};
    use tracing::{info, trace};

    fn create_exportable_tracer_provider() -> (Arc<SdkTracerProvider>, InMemorySpanExporter) {
        let otel_exporter = InMemorySpanExporter::default();
        let otel_tracer_provider = SdkTracerProvider::builder()
            .with_simple_exporter(otel_exporter.clone())
            .build();
        let otel_tracer_provider = Arc::new(otel_tracer_provider);
        (otel_tracer_provider, otel_exporter)
    }

    #[recorded::test()]
    async fn test_service_client_new(ctx: TestContext) -> Result<()> {
        let recording = ctx.recording();
        let endpoint = "https://example.com";
        let credential = recording.credential().clone();
        let options = TestServiceClientOptions {
            api_version: "2023-10-01".to_string(),
            ..Default::default()
        };

        let client = TestServiceClient::new(endpoint, credential, Some(options)).unwrap();
        assert_eq!(client.endpoint().as_str(), "https://example.com/");
        assert_eq!(client.api_version, "2023-10-01");

        Ok(())
    }

    #[recorded::test()]
    async fn test_service_client_get(ctx: TestContext) -> Result<()> {
        let recording = ctx.recording();
        let endpoint = "https://example.com";
        let credential = recording.credential().clone();
        let options = TestServiceClientOptions {
            api_version: "2023-10-01".to_string(),
            ..Default::default()
        };

        let client = TestServiceClient::new(endpoint, credential, Some(options)).unwrap();
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
        let azure_provider = Arc::new(OpenTelemetryTracerProvider::new(sdk_provider)?);

        let recording = ctx.recording();
        let endpoint = "https://example.com";
        let credential = recording.credential().clone();
        let options = TestServiceClientOptions {
            api_version: "2023-10-01".to_string(),
            azure_client_options: ClientOptions {
                request_instrumentation: Some(RequestInstrumentationOptions {
                    tracing_provider: Some(azure_provider),
                }),
                ..Default::default()
            },
        };

        let client = TestServiceClient::new(endpoint, credential, Some(options)).unwrap();
        let response = client.get("index.html", None).await;
        info!("Response: {:?}", response);
        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.status(), azure_core::http::StatusCode::Ok);

        let spans = otel_exporter.get_finished_spans().unwrap();
        assert_eq!(spans.len(), 1);
        for span in &spans {
            trace!("Span: {:?}", span);
        }

        Ok(())
    }
}
