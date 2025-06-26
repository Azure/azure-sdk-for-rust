// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::tracer::OpenTelemetryTracer;
use azure_core::tracing::TracerProvider;
use azure_core::Result;
use opentelemetry::{
    global::{BoxedTracer, ObjectSafeTracerProvider},
    InstrumentationScope,
};
use std::sync::Arc;

/// Enum to hold different OpenTelemetry tracer provider implementations.
pub struct OpenTelemetryTracerProvider {
    inner: Arc<dyn ObjectSafeTracerProvider + Send + Sync>,
}

impl OpenTelemetryTracerProvider {
    /// Creates a new Azure telemetry provider with the given SDK tracer provider.
    #[allow(dead_code)]
    pub fn new(provider: Arc<dyn ObjectSafeTracerProvider + Send + Sync>) -> Result<Self> {
        Ok(Self { inner: provider })
    }
}

impl TracerProvider for OpenTelemetryTracerProvider {
    fn get_tracer(
        &self,
        name: &'static str,
        package_version: &'static str,
    ) -> Arc<dyn azure_core::tracing::Tracer> {
        let scope = InstrumentationScope::builder(name)
            .with_version(package_version)
            .build();
        Arc::new(OpenTelemetryTracer::new(BoxedTracer::new(
            self.inner.boxed_tracer(scope),
        )))
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
        let tracer_provider = OpenTelemetryTracerProvider::new(provider);
        assert!(tracer_provider.is_ok());
    }

    #[test]
    fn test_create_tracer_provider_noop_tracer() {
        let provider = Arc::new(NoopTracerProvider::new());
        let tracer_provider = OpenTelemetryTracerProvider::new(provider);
        assert!(tracer_provider.is_ok());
    }
}
