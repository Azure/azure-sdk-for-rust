// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::tracing::{SpanKind, TracerProvider as _};
use azure_core_opentelemetry::OpenTelemetryTracerProvider;
use opentelemetry_sdk::trace::SdkTracerProvider;
use std::error::Error;
use std::sync::Arc;

#[tokio::test]
async fn test_span_creation() -> Result<(), Box<dyn Error>> {
    // Set up a tracer provider for testing
    let sdk_provider = Arc::new(SdkTracerProvider::builder().build());
    let azure_provider = OpenTelemetryTracerProvider::new(sdk_provider)?;

    // Get a tracer from the Azure provider
    let tracer = azure_provider.get_tracer("test_namespace", "test_tracer", "1.0.0");

    // Create a span using the Azure tracer
    let span = tracer.start_span("test_span", SpanKind::Internal, vec![]);

    // Add attributes to the span using individual set_attribute calls
    span.set_attribute(
        "test_key",
        azure_core::tracing::AttributeValue::String("test_value".to_string()),
    );
    span.set_attribute(
        "service.name",
        azure_core::tracing::AttributeValue::String("azure-test".to_string()),
    );

    // End the span
    span.end();

    Ok(())
}

#[tokio::test]
async fn test_tracer_provider_creation() -> Result<(), Box<dyn Error>> {
    // Create multiple tracer provider instances to test initialization
    let sdk_provider = Arc::new(SdkTracerProvider::builder().build());
    let azure_provider = OpenTelemetryTracerProvider::new(sdk_provider)?;

    // Get a tracer and verify it works
    let tracer = azure_provider.get_tracer("tes.namespace", "test_tracer", "1.0.0");
    let span = tracer.start_span("test_span", SpanKind::Internal, vec![]);
    span.end();

    Ok(())
}

#[tokio::test]
async fn test_span_attributes() -> Result<(), Box<dyn Error>> {
    // Set up a tracer provider for testing
    let sdk_provider = Arc::new(SdkTracerProvider::builder().build());
    let azure_provider = OpenTelemetryTracerProvider::new(sdk_provider)?;

    // Get a tracer from the Azure provider
    let tracer = azure_provider.get_tracer("test.namespace", "test_tracer", "1.0.0");

    // Create span with multiple attributes
    let span = tracer.start_span("test_span", SpanKind::Internal, vec![]);

    // Add attributes using individual set_attribute calls
    span.set_attribute(
        "service.name",
        azure_core::tracing::AttributeValue::String("test-service".to_string()),
    );
    span.set_attribute(
        "operation.name",
        azure_core::tracing::AttributeValue::String("test-operation".to_string()),
    );
    span.set_attribute(
        "request.id",
        azure_core::tracing::AttributeValue::String("req-123".to_string()),
    );

    // End the span
    span.end();

    Ok(())
}
