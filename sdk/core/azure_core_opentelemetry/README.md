# Azure Core OpenTelemetry Tracing

This crate provides OpenTelemetry distributed tracing support for the Azure SDK for Rust.
It bridges the standardized azure_core tracing traits with OpenTelemetry implementation,
enabling automatic span creation, context propagation, and telemetry collection for Azure services.

It allows Rust applications which use the [OpenTelemetry](https://opentelemetry.io/) APIs to generate OpenTelemetry spans for Azure SDK for Rust Clients.

## OpenTelemetry integration with the Azure SDK for Rust

To integrate the OpenTelemetry APIs with the Azure SDK for Rust, you create a [`OpenTelemetryTracerProvider`] and pass it into your SDK ClientOptions.

```rust no_run
# use azure_identity::DefaultAzureCredential;
# use azure_core::{http::{ClientOptions, RequestInstrumentationOptions}};
# #[derive(Default)]
# struct ServiceClientOptions {
#    client_options: ClientOptions,
# }
use azure_core_opentelemetry::OpenTelemetryTracerProvider;
use opentelemetry_sdk::trace::SdkTracerProvider;
use std::sync::Arc;

# fn test_fn() -> azure_core::Result<()> {
// Create an OpenTelemetry tracer provider adapter from an OpenTelemetry TracerProvider
let otel_tracer_provider = Arc::new(SdkTracerProvider::builder().build());

let azure_provider = OpenTelemetryTracerProvider::new(otel_tracer_provider);

let options = ServiceClientOptions {
    client_options: ClientOptions {
        request_instrumentation: Some(RequestInstrumentationOptions {
            tracing_provider: Some(azure_provider),
        }),
        ..Default::default()
    },
    ..Default::default()
    };

#   Ok(())
# }
```

If it is more convenient to use the global OpenTelemetry provider, then the [`OpenTelemetryTracerProvider::new_from_global_provider`] method will configure the OpenTelemetry support to use the global provider instead of a custom configured provider.

```rust no_run
# use azure_identity::DefaultAzureCredential;
# use azure_core::{http::{ClientOptions, RequestInstrumentationOptions}};

# #[derive(Default)]
# struct ServiceClientOptions {
#    client_options: ClientOptions,
# }
use azure_core_opentelemetry::OpenTelemetryTracerProvider;
use opentelemetry_sdk::trace::SdkTracerProvider;
use std::sync::Arc;

# fn test_fn() -> azure_core::Result<()> {

let azure_provider = OpenTelemetryTracerProvider::new_from_global_provider();

let options = ServiceClientOptions {
    client_options: ClientOptions {
        request_instrumentation: Some(RequestInstrumentationOptions {
            tracing_provider: Some(azure_provider),
        }),
        ..Default::default()
    },
};

#   Ok(())
# }
```

Once the `OpenTelemetryTracerProvider` is integrated with the Azure Service ClientOptions, the Azure SDK will be configured to capture per-API and per-HTTP operation tracing options, and the HTTP requests will be annotated with [W3C Trace Context headers](https://www.w3.org/TR/trace-context/).
