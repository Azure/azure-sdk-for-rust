
# Azure Event Hubs Checkpoint Store for Blob Storage

This crate provides a checkpoint store implementation for Azure Event Hubs using Azure Blob Storage as the backend. It implements the `CheckpointStore` trait from the `azure_messaging_eventhubs` crate, allowing you to persist checkpoints (event positions) to Azure Blob Storage.

## Features

- **Persistent Checkpoints**: Store event processing positions in Azure Blob Storage
- **High Availability**: Leverage Azure Blob Storage's durability and availability
- **Concurrency Support**: Handle multiple processors with proper concurrency control
- **Easy Integration**: Drop-in replacement for other checkpoint store implementations

[Source code] | [Package (crates.io)] | [API reference documentation] | [Product documentation]

## Getting started

### Install the package

Install the Azure Event Hubs client library for Rust with [Cargo]:

```sh
cargo add azure_messaging_eventhubs
```

### Prerequisites

- A Rust Compiler. See [the rust compiler installation instructions](https://www.rust-lang.org/tools/install).
- An [Azure subscription]
- The [Azure CLI]
- An [Event Hub namespace](https://learn.microsoft.com/azure/event-hubs/).
- An Event Hub instance. You can create an Event Hub instance in your Event Hubs Namespace using the [Azure Portal](https://learn.microsoft.com/azure/event-hubs/event-hubs-create), or the [Azure CLI](https://learn.microsoft.com/azure/event-hubs/event-hubs-quickstart-cli).

If you use the Azure CLI, replace `<your-resource-group-name>`, `<your-eventhubs-namespace-name>`, and `<your-eventhub-name>` with your own, unique names:

Create an Event Hubs Namespace:

```azurecli
az eventhubs namespace create --resource-group <your-resource-group-name> --name <your-eventhubs-namespace-name> --sku Standard
```

Create an Event Hub Instance:

```azurecli
az eventhubs eventhub create --resource-group <your-resource-group-name> --namespace-name <your-eventhubs-namespace-name> --name <your-eventhub-name>
```

### Install dependencies

Add the following crates to your project:

```sh
cargo add azure_identity tokio azure_messaging_eventhubs azure_storage_blob
```

### Authenticate the client

In order to interact with the Azure Event Hubs service and blob storage service, you'll need to create an instance of the `ProducerClient` or the `ConsumerClient`. You need an **event hub namespace host URL** (which you may see as `serviceBusEndpoint` in the Azure CLI response when creating the Even Hubs Namespace), an **Event Hub name** (which you may see as `name` in the Azure CLI response when creating the Event Hub instance), and credentials to instantiate a client object.

The example shown below uses a [`DefaultAzureCredential`][default_cred_ref], which is appropriate for most local development environments. Additionally, we recommend using a managed identity for authentication in production environments. You can find more information on different ways of authenticating and their corresponding credential types in the [Azure Identity] documentation.

The `DefaultAzureCredential` will automatically pick up on an Azure CLI authentication. Ensure you are logged in with the Azure CLI:

```azurecli
az login
```

Instantiate a `DefaultAzureCredential` to pass to the client. The same instance of a token credential can be used with multiple clients if they will be authenticating with the same identity.

### Basic Example

This example creates a blob container client on the storage account which will hold the blob checkpoint store, and configures a blob checkpoint store to use that storage client.

It then creates an EventHubs processor client using the blob checkpoint store and starts the processor.

```rust no_run
use azure_messaging_eventhubs_checkpointstore_blob::BlobCheckpointStore;
use azure_messaging_eventhubs::{ConsumerClient, EventProcessor, ProcessorStrategy};
use azure_storage_blob::BlobContainerClient;
use azure_identity::DefaultAzureCredential;
use std::sync::Arc;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create blob service client
    let credential = DefaultAzureCredential::new()?;
    let blob_client = BlobContainerClient::new(
        "https://yourstorageaccount.blob.core.windows.net",
        "yourcontainername".to_string(),
        credential.clone(),
        None,
    )?;

    // Create checkpoint store
    let checkpoint_store = BlobCheckpointStore::new(blob_client);

    let consumer_client = ConsumerClient::builder()
        .open(
            "my-eventhubs-host-name",
            "my-eventhub-name".to_string(),
            credential.clone(),
        )
        .await?;

    let event_processor = EventProcessor::builder()
        .with_load_balancing_strategy(ProcessorStrategy::Greedy)
        .build(
            Arc::new(consumer_client),
            checkpoint_store,
        )
        .await?;

    // Start processing
    tokio::spawn(async move { event_processor.run().await });

    Ok(())
}
```

## Examples

See the `examples/` directory for more detailed usage examples:

- `checkpoint_store_basic.rs`: Basic checkpoint operations
- `processor_with_blob_checkpoints.rs`: Complete EventHubs processor setup

## Troubleshooting

### General

When you interact with the Azure Event Hubs Checkpoint Store  using the Rust SDK, errors returned by the service are returned as `azure_core::Error` values. However in general, client applications will not interact with the Checkpoint Store - the Checkpoint Store functionality is primarily used by the Event Hubs event processor.

### Logging

The Event Hubs SDK client uses the [tracing](https://docs.rs/tracing/latest/tracing/) package to
enable diagnostics.

## Contributing

See the [CONTRIBUTING.md] for details on building, testing, and contributing to these libraries.

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit <https://opensource.microsoft.com/cla/>.

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct]. For more information see the [Code of Conduct FAQ] or contact <opencode@microsoft.com> with any additional questions or comments.

### Reporting security issues and security bugs

Security issues and bugs should be reported privately, via email, to the Microsoft Security Response Center (MSRC) <secure@microsoft.com>. You should receive a response within 24 hours. If for some reason you do not, please follow up via email to ensure we received your original message. Further information, including the MSRC PGP key, can be found in the [Security TechCenter](https://www.microsoft.com/msrc/faqs-report-an-issue).

### License

Azure SDK for Rust is licensed under the [MIT](https://github.com/Azure/azure-sdk-for-cpp/blob/main/LICENSE.txt) license.

<!-- LINKS -->
[API reference documentation]: https://docs.rs/azure_messaging_eventhubs_checkpointstore_blob/latest/azure_messaging_eventhubs_checkpointstore_blob
[Azure CLI]: https://learn.microsoft.com/cli/azure
[Azure subscription]: https://azure.microsoft.com/free/
[Azure Identity]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity
[Microsoft Open Source Code of Conduct]: https://opensource.microsoft.com/codeofconduct/
[Product documentation]: https://learn.microsoft.com/azure/event-hubs/
[Cargo]: https://crates.io/
[Package (crates.io)]: https://crates.io/crates/azure_messaging_eventhubs_checkpointstore_blob
[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/eventhubs/azure_messaging_eventhubs_checkpointstore_blob/src
[CONTRIBUTING.md]: https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md
[Code of Conduct FAQ]: https://opensource.microsoft.com/codeofconduct/faq/
[default_cred_ref]: https://docs.rs/azure_identity/latest/azure_identity/struct.DefaultAzureCredential.html
