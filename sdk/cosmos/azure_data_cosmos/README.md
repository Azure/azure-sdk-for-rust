# Azure Cosmos DB SDK for Rust

Use this client library to access Azure Cosmos DB for NoSQL from Rust. It
provides typed clients, Serde-based item serialization, and paginated queries.

Stable public APIs of `azure_data_cosmos` are officially supported, including
driver symbols re-exported through the SDK. Direct use of
`azure_data_cosmos_driver` isn't officially supported. Preview features may
have breaking API or behavior changes in future releases.

[Source code] | [Package (crates.io)] | [API reference documentation] | [Azure Cosmos DB for NoSQL documentation]

## Getting started

### Install the package

Install the SDK with Cargo:

```sh
cargo add azure_data_cosmos
```

### Prerequisites

- An [Azure subscription] with an Azure Cosmos DB for NoSQL account, a
  [free trial account], or the [Azure Cosmos DB Emulator].
- An identity with access to the account. The examples use
  `DeveloperToolsCredential`; run `az login` for local development.

### Create an Azure Cosmos DB account

Create an account using:

- [Azure Portal](https://portal.azure.com).
- [Azure CLI](https://learn.microsoft.com/cli/azure).
- [Azure ARM](https://learn.microsoft.com/azure/cosmos-db/quick-create-template).

### Authenticate the client

Pass an account endpoint and a Microsoft Entra ID credential to
`CosmosClient::builder().build()`. Add `azure_identity` to use
`DeveloperToolsCredential` locally. Use a managed identity in production;
see [Azure Identity] for credential choices.

```rust,no_run
use azure_core::credentials::TokenCredential;
use azure_data_cosmos::{
    AccountEndpoint, AccountReference, CosmosClient, RoutingStrategy,
};
use azure_identity::DeveloperToolsCredential;
use std::sync::Arc;

async fn example() -> Result<(), Box<dyn std::error::Error>> {
    let credential: Arc<dyn TokenCredential> = DeveloperToolsCredential::new(None)?;
    let endpoint: AccountEndpoint = "https://myaccount.documents.azure.com/".parse()?;
    let account = AccountReference::with_credential(endpoint, credential);
    let _client = CosmosClient::builder()
        .build(account, RoutingStrategy::ProximityTo("East US".into()))
        .await?;
    Ok(())
}
```

## Features

Preview features may have breaking API or behavior changes in future releases.

| Feature flag | Support level | Enabled by default | Description |
| --- | --- | --- | --- |
| `key_auth` | Supported | No | Enables account-key and resource-token authentication. |
| `control_plane` | Supported | No | Enables database and container management and throughput operations. These currently require key-based authentication, so enable `key_auth` too. Reading container properties doesn't require this feature. |
| `metrics` | Supported | No | Enables OpenTelemetry metrics. |
| `distributed_tracing` | Supported | No | Enables OpenTelemetry distributed tracing. |
| `fault_injection` | Supported for testing | No | Enables fault-injection rules for testing. |
| `preview_patch` | Preview | No | Enables item PATCH. |
| `preview_dtx` | Preview | No | Enables distributed transactions. |
| `reqwest` | Supported | Yes | Enables the Reqwest HTTP transport. |
| `rustls` | Supported | Yes | Enables Rustls TLS for Reqwest. |
| `native_tls` | Supported | No | Enables native TLS for Reqwest. |
| `hmac_rust` | Supported | Yes | Enables Rust-based HMAC signing. |
| `hmac_openssl` | Supported | No | Enables OpenSSL-based HMAC signing. |

Features beginning with `__` are reserved for SDK development and testing
and aren't listed here.

## Examples

The following examples use a container partitioned on `/category`. Replace
the account, database, and container names with your own.

### Point write and read

The partition key supplied to an item operation must match the item's
partition key value. Item writes don't return the item body by default.

```rust,no_run
use serde::{Serialize, Deserialize};
use azure_data_cosmos::CosmosClient;

#[derive(Serialize, Deserialize)]
struct Item {
    id: String,
    category: String,
}

async fn example(client: &CosmosClient) -> Result<(), Box<dyn std::error::Error>> {
    let container = client
        .database_client("myDatabase")
        .container_client("myContainer", None)
        .await?;
    let item = Item { id: "item1".into(), category: "books".into() };
    container.create_item("books", "item1", &item, None).await?;
    let response = container.read_item("books", "item1", None).await?;
    let saved: Item = response.into_model()?;
    assert_eq!(saved.id, "item1");
    Ok(())
}
```

### Query items

Use a parameterized query with `FeedScope::partition()` to target one logical
partition. Use `FeedScope::full_container()` for cross-partition queries.

```rust,no_run
use azure_data_cosmos::{CosmosClient, FeedScope, Query};
use futures::TryStreamExt;

#[derive(serde::Deserialize)]
struct Item {
    id: String,
}

async fn example(client: &CosmosClient) -> Result<(), Box<dyn std::error::Error>> {
    let container = client
        .database_client("myDatabase")
        .container_client("myContainer", None)
        .await?;
    let query = Query::from("SELECT * FROM c WHERE c.category = @category")
        .with_parameter("@category", "books")?;
    let mut items = container
        .query_items::<Item>(query, FeedScope::partition("books"), None)
        .await?;
    while let Some(item) = items.try_next().await? {
        println!("{}", item.id);
    }
    Ok(())
}
```

### Partial updates with PATCH (preview)

`ContainerClient::patch_item()` applies JSON-Patch-style operations to a single
item. Enable `preview_patch` to use it; preview APIs and behavior may change
in future releases:

```sh
cargo add azure_data_cosmos --features preview_patch
```

```rust,no_run
# #[cfg(feature = "preview_patch")]
use azure_data_cosmos::clients::ContainerClient;
# #[cfg(feature = "preview_patch")]
use azure_data_cosmos::models::{PatchInstructions, PatchOperation};

# #[cfg(feature = "preview_patch")]
async fn example(container: &ContainerClient) -> azure_data_cosmos::Result<()> {
    let patch = PatchInstructions::from(vec![
        PatchOperation::set("/value", serde_json::json!("4")),
    ]);
    let _patched: serde_json::Value = container
        .patch_item("books", "item1", patch, None)
        .await?
        .into_model()?;
    Ok(())
}
```

See the [API reference documentation] for PATCH options and service limits.

## Remarks

### Linux vNext emulator

The Azure Cosmos DB Linux vNext emulator doesn't currently support the binary
JSON encoding enabled by default in the SDK. Disable it when connecting to
this emulator; this restriction doesn't apply to the Windows emulator.
Create an `AccountReference` with the emulator's account key (requires
`key_auth`), then disable binary encoding when building the client:

```rust,no_run
use azure_data_cosmos::{
    options::BinaryEncodingOptions, AccountReference, CosmosClient, RoutingStrategy,
};

async fn example(account: AccountReference) -> Result<(), Box<dyn std::error::Error>> {
    let _client = CosmosClient::builder()
        .with_binary_encoding_options(BinaryEncodingOptions::new().with_enabled(false))
        .build(account, RoutingStrategy::ProximityTo("East US".into()))
        .await?;
    Ok(())
}
```

Alternatively, set `AZURE_COSMOS_BINARY_ENCODING_ENABLED=false` before
starting the application. An explicit client option takes precedence.

## Next steps

- [Resource Model of Azure Cosmos DB Service](https://learn.microsoft.com/azure/cosmos-db/sql-api-resources)
- [Azure Cosmos DB Resource URI](https://learn.microsoft.com/rest/api/documentdb/documentdb-resource-uri-syntax-for-rest)
- [Partitioning](https://learn.microsoft.com/azure/cosmos-db/partition-data)
- [Using emulator](https://github.com/Azure/azure-documentdb-dotnet/blob/master/docs/documentdb-nosql-local-emulator.md)

### Provide feedback

If you encounter bugs or have suggestions, [open an issue](https://github.com/Azure/azure-sdk-for-rust/issues).

## Contributing

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit [https://cla.microsoft.com](https://cla.microsoft.com).

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You'll only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/). For more information, see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.

<!-- LINKS -->
[Azure subscription]: https://azure.microsoft.com/free/
[free trial account]: https://learn.microsoft.com/azure/cosmos-db/try-free
[Azure Cosmos DB Emulator]: https://learn.microsoft.com/azure/cosmos-db/emulator
[Azure Identity]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity
[API reference documentation]: https://docs.rs/azure_data_cosmos/latest/azure_data_cosmos/
[Azure Cosmos DB for NoSQL documentation]: https://learn.microsoft.com/azure/cosmos-db/nosql/
[Package (crates.io)]: https://crates.io/crates/azure_data_cosmos
[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/cosmos/azure_data_cosmos
