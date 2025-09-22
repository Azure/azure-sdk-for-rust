# Azure Core OpenTelemetry Tracing

This crate provides [OpenTelemetry](https://opentelemetry.io) distributed tracing support for the Azure SDK for Rust.
It bridges the standardized `azure_core` tracing traits with the OpenTelemetry for Rust implementation,
enabling automatic span creation, context propagation, and telemetry collection for Azure services.

It allows Rust applications which use the [OpenTelemetry](https://opentelemetry.io/) APIs to generate OpenTelemetry spans for Azure SDK for Rust Clients.

## OpenTelemetry integration with the Azure SDK for Rust

To integrate the OpenTelemetry APIs with the Azure SDK for Rust, you create a `OpenTelemetryTracerProvider` and pass it into your SDK ClientOptions.

```rust no_run
# use azure_identity::DeveloperToolsCredential;
# use azure_core::{http::{ClientOptions, InstrumentationOptions}};
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
        instrumentation: InstrumentationOptions {
            tracer_provider: Some(azure_provider),
        },
        ..Default::default()
    },
    ..Default::default()
};

#   Ok(())
# }
```

If it is more convenient to use the global OpenTelemetry provider, then the `OpenTelemetryTracerProvider::from_global_provider` method will configure the OpenTelemetry support to use the global provider instead of a custom configured provider.

```rust no_run
# use azure_identity::DeveloperToolsCredential;
# use azure_core::{http::{ClientOptions, InstrumentationOptions}};

# #[derive(Default)]
# struct ServiceClientOptions {
#    client_options: ClientOptions,
# }
use azure_core_opentelemetry::OpenTelemetryTracerProvider;
use opentelemetry_sdk::trace::SdkTracerProvider;
use std::sync::Arc;

# fn test_fn() -> azure_core::Result<()> {

let azure_provider = OpenTelemetryTracerProvider::from_global_provider();

let options = ServiceClientOptions {
    client_options: ClientOptions {
        instrumentation: InstrumentationOptions {
            tracer_provider: Some(azure_provider),
        },
        ..Default::default()
    },
};

#   Ok(())
# }
```

Once the `OpenTelemetryTracerProvider` is integrated with the Azure Service ClientOptions, the Azure SDK will be configured to capture per-API and per-HTTP operation tracing options, and the HTTP requests will be annotated with [W3C Trace Context headers](https://www.w3.org/TR/trace-context/).

## Contributing

See the [CONTRIBUTING.md] for details on building, testing, and contributing to these libraries.

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit <https://opensource.microsoft.com/cla/>.

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct]. For more information see the [Code of Conduct FAQ] or contact <opencode@microsoft.com> with any additional questions or comments.

## Reporting security issues and security bugs

Security issues and bugs should be reported privately, via email, to the Microsoft Security Response Center (MSRC) <secure@microsoft.com>. You should receive a response within 24 hours. If for some reason you do not, please follow up via email to ensure we received your original message. Further information, including the MSRC PGP key, can be found in the [Security TechCenter](https://www.microsoft.com/msrc/faqs-report-an-issue).

## License

Azure SDK for Rust is licensed under the [MIT](https://github.com/Azure/azure-sdk-for-cpp/blob/main/LICENSE.txt) license.

<!-- LINKS -->

[Microsoft Open Source Code of Conduct]: https://opensource.microsoft.com/codeofconduct/
[CONTRIBUTING.md]: https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md
[Code of Conduct FAQ]: https://opensource.microsoft.com/codeofconduct/faq/
