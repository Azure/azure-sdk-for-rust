// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! OpenTelemetry wiring for the soak harness.
//!
//! Installs a global [`SdkMeterProvider`] and [`SdkTracerProvider`] backed by a
//! selectable exporter (stdout, OTLP/gRPC, or none) plus a `tracing` subscriber
//! that surfaces the SDK's sampled-diagnostics log lines. The built-in Cosmos
//! diagnostics handlers resolve the global providers, so these must be installed
//! *before* the client is built (for metrics) and can be resolved lazily (for
//! tracing).

use std::error::Error;
use std::time::Duration;

use opentelemetry::global;
use opentelemetry_sdk::metrics::{PeriodicReader, SdkMeterProvider};
use opentelemetry_sdk::trace::SdkTracerProvider;
use opentelemetry_sdk::Resource;
use tracing_subscriber::EnvFilter;

use crate::config::{Config, Exporter};

/// Instrumentation scope / service name reported for all emitted telemetry.
const SERVICE_NAME: &str = "cosmos-observability-harness";

/// Owns the installed OpenTelemetry providers so they can be flushed and shut
/// down cleanly at the end of the run (ensuring the final metric/span batch is
/// exported before the process exits).
pub struct Telemetry {
    meter_provider: Option<SdkMeterProvider>,
    tracer_provider: Option<SdkTracerProvider>,
}

impl Telemetry {
    /// Initializes the `tracing` subscriber and installs the global OpenTelemetry
    /// providers per the configured exporter.
    pub fn install(config: &Config) -> Result<Self, Box<dyn Error>> {
        init_tracing_subscriber();

        let (meter_provider, tracer_provider) = match config.exporter {
            Exporter::None => {
                tracing::info!("telemetry exporter: none (sampled logs only)");
                (None, None)
            }
            Exporter::Stdout => {
                tracing::info!("telemetry exporter: stdout");
                let interval = Duration::from_secs(config.metric_export_interval_secs.max(1));
                (
                    Some(build_stdout_meter_provider(interval)),
                    Some(build_stdout_tracer_provider()),
                )
            }
            Exporter::Otlp => {
                let interval = Duration::from_secs(config.metric_export_interval_secs.max(1));
                let providers = build_otlp_providers(&config.otlp_endpoint, interval)?;
                tracing::info!(endpoint = %config.otlp_endpoint, "telemetry exporter: OTLP/gRPC");
                (Some(providers.0), Some(providers.1))
            }
        };

        if let Some(meter_provider) = &meter_provider {
            global::set_meter_provider(meter_provider.clone());
        }
        if let Some(tracer_provider) = &tracer_provider {
            global::set_tracer_provider(tracer_provider.clone());
        }

        Ok(Self {
            meter_provider,
            tracer_provider,
        })
    }

    /// Flushes and shuts down the installed providers, blocking until the final
    /// batch has been exported.
    pub fn shutdown(self) {
        if let Some(meter_provider) = self.meter_provider {
            let _ = meter_provider.force_flush();
            let _ = meter_provider.shutdown();
        }
        if let Some(tracer_provider) = self.tracer_provider {
            let _ = tracer_provider.force_flush();
            let _ = tracer_provider.shutdown();
        }
    }
}

/// Builds the OpenTelemetry [`Resource`] describing this process.
fn resource() -> Resource {
    Resource::builder().with_service_name(SERVICE_NAME).build()
}

/// Installs a `tracing` subscriber. Defaults to a quiet-at-steady-state filter
/// that still surfaces the SDK's sampled-diagnostics lines and harness progress;
/// `RUST_LOG` overrides it entirely.
fn init_tracing_subscriber() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new(
            "warn,azure_data_cosmos_observability_harness=info,azure_data_cosmos::diagnostics=info",
        )
    });
    // `try_init` (not `init`) so a second install in tests is a no-op rather than
    // a panic.
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .try_init();
}

/// Builds a stdout-backed meter provider whose periodic reader flushes on the
/// given interval.
fn build_stdout_meter_provider(interval: Duration) -> SdkMeterProvider {
    let exporter = opentelemetry_stdout::MetricExporter::default();
    let reader = PeriodicReader::builder(exporter)
        .with_interval(interval)
        .build();
    SdkMeterProvider::builder()
        .with_reader(reader)
        .with_resource(resource())
        .build()
}

/// Builds a stdout-backed tracer provider. Uses a simple (synchronous) exporter
/// so sampled span trees print as soon as they are emitted.
fn build_stdout_tracer_provider() -> SdkTracerProvider {
    SdkTracerProvider::builder()
        .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
        .with_resource(resource())
        .build()
}

/// Builds OTLP/gRPC-backed meter and tracer providers targeting `endpoint`.
#[cfg(feature = "otlp")]
fn build_otlp_providers(
    endpoint: &str,
    interval: Duration,
) -> Result<(SdkMeterProvider, SdkTracerProvider), Box<dyn Error>> {
    use opentelemetry_otlp::WithExportConfig;

    let metric_exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .build()?;
    let reader = PeriodicReader::builder(metric_exporter)
        .with_interval(interval)
        .build();
    let meter_provider = SdkMeterProvider::builder()
        .with_reader(reader)
        .with_resource(resource())
        .build();

    let span_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .build()?;
    let tracer_provider = SdkTracerProvider::builder()
        .with_batch_exporter(span_exporter)
        .with_resource(resource())
        .build();

    Ok((meter_provider, tracer_provider))
}

/// Fallback when the `otlp` feature is disabled: report a clear, actionable error
/// instead of silently degrading.
#[cfg(not(feature = "otlp"))]
fn build_otlp_providers(
    _endpoint: &str,
    _interval: Duration,
) -> Result<(SdkMeterProvider, SdkTracerProvider), Box<dyn Error>> {
    Err("`--exporter otlp` requires building with `--features otlp`".into())
}
