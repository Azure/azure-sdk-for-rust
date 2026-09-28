# azure_data_cosmos_observability_harness

- **Description**: Observability soak/load harness for validating the Azure Cosmos DB Rust SDK diagnostics layer end-to-end
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`
  - `fault_injection`
  - `otlp_rustls`
  - `preview_opentelemetry`
- `fault_injection`
- `opentelemetry-otlp`
- `otlp`
- `otlp_rustls`
- `preview_opentelemetry`

```rust
#![crate_name = "azure_data_cosmos_observability_harness"]
#![crate_type = "bin"]
```
