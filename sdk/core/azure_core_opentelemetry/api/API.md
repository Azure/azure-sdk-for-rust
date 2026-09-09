# azure_core_opentelemetry

- **Description**: OpenTelemetry integration for the Azure SDK for Rust
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`
  - `azure_core/default`

```rust
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
pub struct OpenTelemetryTracerProvider {
}
impl OpenTelemetryTracerProvider {
    fn from_global_provider() -> Arc<Self>;
    fn new(provider: Arc<dyn ObjectSafeTracerProvider + Send + Sync>) -> Arc<Self>;
}
impl Debug for OpenTelemetryTracerProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}
impl TracerProvider for OpenTelemetryTracerProvider {
    fn get_tracer(&self, namespace: Option<&'static str>, crate_name: &'static str, crate_version: Option<&'static str>) -> Arc<dyn azure_core::tracing::Tracer>;
}
```
