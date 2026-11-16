// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Export SDK traces, logs, and native metrics over OTLP/HTTP.
//! Run with `--features metrics`. Without `--live`, exercise a failed local
//! connection without contacting Azure. Set OTEL_EXPORTER_OTLP_ENDPOINT to the
//! collector base URL (default: http://localhost:4318).

use azure_core::{time::Duration, tracing::TracerProvider};
use azure_core_opentelemetry::OpenTelemetryTracerProvider;
use azure_messaging_eventhubs::{ProducerClient, RetryOptions};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_sdk::{
    logs::SdkLoggerProvider, metrics::SdkMeterProvider, trace::SdkTracerProvider, Resource,
};
use std::{error::Error, sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

fn main() -> Result<(), Box<dyn Error>> {
    let resource = Resource::builder()
        .with_service_name("eventhubs-rust-example")
        .build();
    let traces = SdkTracerProvider::builder()
        .with_resource(resource.clone())
        .with_batch_exporter(
            opentelemetry_otlp::SpanExporter::builder()
                .with_http()
                .build()?,
        )
        .build();
    let logs = SdkLoggerProvider::builder()
        .with_resource(resource.clone())
        .with_batch_exporter(
            opentelemetry_otlp::LogExporter::builder()
                .with_http()
                .build()?,
        )
        .build();
    let metrics = SdkMeterProvider::builder()
        .with_resource(resource)
        .with_periodic_exporter(
            opentelemetry_otlp::MetricExporter::builder()
                .with_http()
                .build()?,
        )
        .build();
    // Install before constructing clients: they retain their meter's instruments.
    opentelemetry::global::set_meter_provider(metrics.clone());
    tracing_subscriber::registry()
        .with(
            OpenTelemetryTracingBridge::new(&logs)
                .with_filter(EnvFilter::new("azure_messaging_eventhubs=info")),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_filter(EnvFilter::new("azure_messaging_eventhubs=info")),
        )
        .try_init()?;
    let provider = OpenTelemetryTracerProvider::new(Arc::new(traces.clone()));
    let live = std::env::args().any(|arg| arg == "--live");
    let runtime = tokio::runtime::Runtime::new()?;
    let result = runtime.block_on(run(live, provider));
    // Always flush, including after a service error. Exporters use blocking HTTP
    // on their own worker threads; shutdown runs outside the async runtime.
    let trace_shutdown = traces.shutdown();
    let log_shutdown = logs.shutdown();
    let metric_shutdown = metrics.shutdown();
    result?;
    trace_shutdown?;
    log_shutdown?;
    metric_shutdown?;
    Ok(())
}

async fn run(live: bool, provider: Arc<dyn TracerProvider>) -> Result<(), Box<dyn Error>> {
    if live {
        let namespace = std::env::var("EVENTHUBS_HOST")?;
        let hub = std::env::var("EVENTHUB_NAME")?;
        let credential = azure_identity::DeveloperToolsCredential::new(None)?;
        let producer = ProducerClient::builder()
            .with_tracing_provider(provider)
            .open(&namespace, &hub, credential)
            .await?;
        let result = producer.send_event("OpenTelemetry example", None).await;
        let closed = producer.close().await;
        result?;
        closed?;
    } else {
        // Port zero cannot host a service. This creates a real SDK connection
        // failure span, lifecycle logs, and lifecycle metrics without credentials.
        let result = ProducerClient::builder().with_tracing_provider(provider)
            .with_custom_endpoint("amqp://127.0.0.1:0".to_owned())
            .with_retry_options(RetryOptions { max_retries: 0, max_total_elapsed: Duration::seconds(1), ..Default::default() })
            .open_with_connection_string("Endpoint=sb://127.0.0.1:0;SharedAccessKeyName=sample;SharedAccessKey=c2FtcGxl;EntityPath=smoke", None).await;
        if result.is_ok() {
            return Err("smoke connection unexpectedly succeeded".into());
        }
        println!("Expected local connection failure recorded; flushing all three signals.");
    }
    Ok(())
}
