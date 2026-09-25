# Azure Cosmos DB Driver for Rust

This crate provides transport, routing, and protocol handling for Azure Cosmos
DB SDKs. It accepts raw item bytes and returns buffered responses; the
consuming SDK handles application-defined item serialization.

**The driver is intended as an internal SDK component. Direct use of its public
APIs isn't officially supported.** For Rust applications, use the
[`azure_data_cosmos` SDK], which is fully supported. This includes use of
driver symbols re-exported through the SDK's public APIs; using the same
symbols directly through this crate isn't officially supported.

[Source code] | [Package (crates.io)] | [API reference documentation] | [`azure_data_cosmos` SDK]

## Getting started

### Install the package

SDK implementers can add the driver with Cargo:

```sh
cargo add azure_data_cosmos_driver
```

Applications should install [`azure_data_cosmos` SDK] instead.

### Prerequisites

- An [Azure subscription] with an Azure Cosmos DB for NoSQL account.
- An identity with access to the account. The example uses
  `DeveloperToolsCredential`; run `az login` for local development.

### Connect to an account

Build a driver runtime and create a driver for the account. The example uses
the default Tokio runtime and Reqwest transport. Add `azure_identity` for the
credential and `url` for the account endpoint.

```rust,no_run
use azure_data_cosmos_driver::{
    models::AccountReference, options::DriverOptions, CosmosDriverRuntime,
};
use azure_identity::DeveloperToolsCredential;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = Url::parse("https://myaccount.documents.azure.com/")?;
    let credential = DeveloperToolsCredential::new(None)?;
    let account = AccountReference::with_credential(endpoint, credential);
    let runtime = CosmosDriverRuntime::builder().build().await?;
    let _driver = runtime
        .create_driver(DriverOptions::builder(account).build())
        .await?;
    Ok(())
}
```

## Features

Direct use of driver public APIs isn't officially supported, regardless of
feature selection. Driver symbols used through the supported SDK's public APIs
retain SDK support. Preview features may have breaking API or behavior changes
in future releases.

| Feature flag | Support level | Enabled by default | Description |
| --- | --- | --- | --- |
| `tokio` | Internal SDK use | Yes | Enables the Tokio runtime integration. |
| `reqwest` | Internal SDK use | Yes | Enables the Reqwest HTTP transport. |
| `rustls` | Internal SDK use | Yes | Enables Rustls TLS for Reqwest. |
| `native_tls` | Internal SDK use | No | Enables native TLS for Reqwest. |
| `fault_injection` | Internal SDK testing | No | Enables fault-injection rules for testing. |
| `preview_dtx` | Preview | No | Enables distributed transactions. |
| `preview_patch` | Preview | No | Enables PATCH strategy configuration. Core PATCH operations don't require this feature. |

Features beginning with `__` are reserved for SDK development and testing
and aren't listed here.

## Examples

These examples show how an SDK integrates with the driver. They require an
account with a `myDatabase` database and a `myContainer` container partitioned
on `/category`. Applications should use [`azure_data_cosmos` SDK] instead.

### Point write and read

Resolve the container before constructing an item reference. The partition
key must match the item body's `/category` value. The driver returns response
bodies without deserializing them into application types.

```rust,no_run
use azure_data_cosmos_driver::{
    models::{CosmosOperation, ItemReference, PartitionKey},
    options::OperationOptions, CosmosDriver,
};

async fn example(driver: &CosmosDriver) -> Result<(), Box<dyn std::error::Error>> {
    let container = driver
        .resolve_container_by_name("myDatabase", "myContainer", OperationOptions::default())
        .await?;
    let item = ItemReference::from_name(&container, PartitionKey::from("books"), "item1");
    let body = serde_json::to_vec(&serde_json::json!({
        "id": "item1", "category": "books"
    }))?;
    driver
        .execute_operation(
            CosmosOperation::create_item(item.clone()).with_body(body),
            OperationOptions::default(),
        )
        .await?;
    let response = driver
        .execute_operation(CosmosOperation::read_item(item), OperationOptions::default())
        .await?;
    if let Some(response) = response {
        println!("{:?}", response.body());
    }
    Ok(())
}
```

### Query items

Plan a feed query, then request pages until `execute_plan()` returns `None`.
Query results are raw response bodies; the consuming SDK deserializes items.

```rust,no_run
use azure_data_cosmos_driver::{
    models::{CosmosOperation, FeedRange},
    options::{OperationOptions, PlanOptions}, CosmosDriver,
};

async fn example(driver: &CosmosDriver) -> Result<(), Box<dyn std::error::Error>> {
    let options = OperationOptions::default();
    let container = driver
        .resolve_container_by_name("myDatabase", "myContainer", options.clone())
        .await?;
    let query = serde_json::to_vec(&serde_json::json!({
        "query": "SELECT * FROM c WHERE c.category = @category",
        "parameters": [{"name": "@category", "value": "books"}]
    }))?;
    let operation = CosmosOperation::query_items(container.clone(), Some(FeedRange::full()))
        .with_body(query);
    let mut plan = driver
        .plan_operation(operation, &options, None, &PlanOptions::default())
        .await?;
    while let Some(page) = driver
        .execute_plan(&mut plan, Some(container.clone()), options.clone())
        .await?
    {
        println!("{:?}", page.body());
    }
    Ok(())
}
```

## Remarks

### Error backtraces

`CosmosError` can capture a backtrace when `RUST_LIB_BACKTRACE` or
`RUST_BACKTRACE` enables it. Capture is disabled by default. Two independent
per-second limits control stack capture and symbol resolution: 1,000 captures
and five resolutions by default when enabled. Cached symbols don't consume
the resolution budget.

Set `AZURE_COSMOS_BACKTRACE_CAPTURES_PER_SECOND` and
`AZURE_COSMOS_BACKTRACE_RESOLUTIONS_PER_SECOND` to override the corresponding
defaults. Programmatic `set_backtrace_options()` overrides the environment.
Setting both limits to zero disables capture.

```rust
use azure_data_cosmos_driver::error::{set_backtrace_options, BacktraceOptions};

let mut options = BacktraceOptions::default();
options.max_captures_per_second = 500;
options.max_resolutions_per_second = 50;
set_backtrace_options(options);
```

The `backtrace()` method on `CosmosError` returns `None` when capture is
disabled or throttled, or when uncached symbols cannot be resolved within
the current budget.

## Next steps

- See the [`azure_data_cosmos` SDK] for supported application APIs.
- See the [API reference documentation] for driver types and options.
- [Open an issue] to report a bug.

## Contributing

Contributions require agreement to the [Contributor License Agreement].
This project follows the [Microsoft Open Source Code of Conduct].

<!-- LINKS -->
[API reference documentation]: https://docs.rs/azure_data_cosmos_driver/latest/azure_data_cosmos_driver/
[Azure subscription]: https://azure.microsoft.com/free/
[Contributor License Agreement]: https://cla.microsoft.com
[Microsoft Open Source Code of Conduct]: https://opensource.microsoft.com/codeofconduct/
[Open an issue]: https://github.com/Azure/azure-sdk-for-rust/issues
[Package (crates.io)]: https://crates.io/crates/azure_data_cosmos_driver
[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/cosmos/azure_data_cosmos_driver
[`azure_data_cosmos` SDK]: https://docs.rs/azure_data_cosmos/latest/azure_data_cosmos/
