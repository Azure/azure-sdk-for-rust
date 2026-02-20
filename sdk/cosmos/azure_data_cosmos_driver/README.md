# Azure Cosmos DB Driver

Core implementation layer for Azure Cosmos DB, providing transport, routing, and protocol handling.

## Purpose

`azure_data_cosmos_driver` is designed for:

- **Cross-language SDK reuse**: Provides a common implementation that can be used by Rust, Java, .NET, and Python SDKs via the C API wrapper (`azure_data_cosmos_native`)
- **Advanced scenarios**: Direct use by developers who need fine-grained control over Cosmos DB operations
- **Internal implementation**: Used internally by `azure_data_cosmos` (the primary Rust SDK)

## Support Model

**Community/GitHub Support Only** - This crate has a public API and accepts contributions, but does **not** receive 24x7 Microsoft Support.

For production Rust applications requiring full Microsoft support, use [`azure_data_cosmos`](https://docs.rs/azure_data_cosmos) instead.

## Key Features

### Schema-Agnostic Data Plane

The driver is intentionally ignorant of document/item schemas. Data plane operations:

- Accept raw bytes (`&[u8]`) for request bodies
- Return buffered responses (`Vec<u8>`) for items (≤16MB payload limit)
- Support both UTF-8 JSON and Cosmos DB binary encoding (detected automatically)

**Serialization is handled by the consuming SDK** using native language APIs.

### Independent Versioning

This crate follows **strict semantic versioning** but can move to new major versions more frequently than `azure_data_cosmos`. Breaking changes in the driver do not force SDK version bumps because the SDK uses adapter patterns to maintain backward compatibility.

## Architecture

```text
┌─────────────────────────────────────┐
│  Language-Specific SDKs             │
│  (azure_data_cosmos, Java, .NET)    │
│  - Type-safe APIs                   │
│  - Native serialization             │
└───────────────┬─────────────────────┘
                │
                ▼
┌─────────────────────────────────────┐
│  azure_data_cosmos_driver           │
│  - Transport & routing              │
│  - Protocol handling                │
│  - Retry logic                      │
│  - Schema-agnostic (raw bytes)      │
└───────────────┬─────────────────────┘
                │
                ▼
┌─────────────────────────────────────┐
│  Azure Cosmos DB Service            │
└─────────────────────────────────────┘
```

## Usage

```rust,no_run
use azure_data_cosmos_driver::{CosmosDriverRuntime, options::DriverOptions};
use azure_data_cosmos_driver::models::AccountReference;
use azure_identity::DeveloperToolsCredential;
use url::Url;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // Use logged-in developer credentials (Azure CLI, azd, etc.)
    let credential = DeveloperToolsCredential::new(None)?;

    let account = AccountReference::with_credential(
        Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
        credential,
    );

    // Create the runtime
    let runtime = CosmosDriverRuntime::builder().build().await?;

    // Get or create a driver for the account (singleton per endpoint)
    let driver = runtime.get_or_create_driver(account, None).await?;

    // Driver operations work with raw bytes
    // let response = driver.execute_operation(operation, options).await?;

    Ok(())
}
```

## Module Organization

- **`diagnostics`**: Operational telemetry (RU consumption, retry counts, timing information)
- **`driver`**: Core transport, routing, and protocol handling
- **`models`**: Resource types, partition keys, status codes, and request metadata
- **`options`**: Configuration types (driver options, connection pool settings, diagnostics)
- **`system`**: System-level utilities (CPU/memory monitoring, VM metadata)

Internal modules (pipeline, routing, handlers) have `pub(crate)` visibility.

## Contributing

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit [https://cla.microsoft.com](https://cla.microsoft.com).

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You'll only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/). For more information, see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.
