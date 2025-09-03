// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::tracer::OpenTelemetryTracer;
use azure_core::tracing::TracerProvider;
use opentelemetry::{
    global::{BoxedTracer, ObjectSafeTracerProvider},
    InstrumentationScope,
};
use std::{fmt::Debug, sync::Arc};

/// Enum to hold different OpenTelemetry tracer provider implementations.
pub struct OpenTelemetryTracerProvider {
    inner: Option<Arc<dyn ObjectSafeTracerProvider + Send + Sync>>,
}

impl OpenTelemetryTracerProvider {
    /// Creates a new Azure telemetry provider with the given SDK tracer provider.
    ///
    /// # Arguments
    /// - `provider`: An `Arc` to an object-safe tracer provider that implements the
    ///   `ObjectSafeTracerProvider` trait.
    ///
    /// # Returns
    /// An `Arc` to the newly created `OpenTelemetryTracerProvider`.
    ///
    ///
    pub fn new(provider: Arc<dyn ObjectSafeTracerProvider + Send + Sync>) -> Arc<Self> {
        Arc::new(Self {
            inner: Some(provider),
        })
    }

    /// Creates a new Azure telemetry provider that uses the global OpenTelemetry tracer provider.
    ///
    /// This is useful when you want to use the global OpenTelemetry provider without
    /// explicitly instantiating a specific provider.
    ///
    /// # Returns
    /// An `Arc` to the newly created `OpenTelemetryTracerProvider` that uses the global provider.
    ///
    pub fn from_global_provider() -> Arc<Self> {
        Arc::new(Self { inner: None })
    }
}

impl Debug for OpenTelemetryTracerProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenTelemetryTracerProvider")
            .finish_non_exhaustive()
    }
}

impl TracerProvider for OpenTelemetryTracerProvider {
    fn get_tracer(
        &self,
        namespace: Option<&'static str>,
        crate_name: &'static str,
        crate_version: Option<&'static str>,
    ) -> Arc<dyn azure_core::tracing::Tracer> {
        let mut builder = InstrumentationScope::builder(crate_name);
        if let Some(crate_version) = crate_version {
            builder = builder.with_version(crate_version);
        }
        let scope = builder
            .with_schema_url("https://opentelemetry.io/schemas/1.23.0")
            .build();
        if let Some(provider) = &self.inner {
            // If we have a specific provider set, use it to create the tracer.
            Arc::new(OpenTelemetryTracer::new(
                namespace,
                BoxedTracer::new(provider.boxed_tracer(scope)),
            ))
        } else {
            // Use the global tracer if no specific provider has been set.
            Arc::new(OpenTelemetryTracer::new(
                namespace,
                opentelemetry::global::tracer_with_scope(scope),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use opentelemetry::trace::noop::NoopTracerProvider;
    use opentelemetry_sdk::trace::SdkTracerProvider;

    #[test]
    fn test_create_tracer_provider_sdk_tracer() {
        let provider = Arc::new(SdkTracerProvider::builder().build());
        let _tracer_provider = OpenTelemetryTracerProvider::new(provider);
    }

    #[test]
    fn test_create_tracer_provider_noop_tracer() {
        let provider = Arc::new(NoopTracerProvider::new());
        let _tracer_provider = OpenTelemetryTracerProvider::new(provider);
    }

    #[test]
    fn test_create_tracer_provider_from_global() {
        let tracer_provider = OpenTelemetryTracerProvider::from_global_provider();
        let _tracer = tracer_provider.get_tracer(Some("My Namespace"), "test", Some("0.1.0"));
    }

    #[test]
    fn test_create_tracer_provider_from_global_provider_set() {
        let provider = SdkTracerProvider::builder().build();
        opentelemetry::global::set_tracer_provider(provider);

        let tracer_provider = OpenTelemetryTracerProvider::from_global_provider();
        let _tracer = tracer_provider.get_tracer(Some("My Namespace"), "test", Some("0.1.0"));
    }
}
