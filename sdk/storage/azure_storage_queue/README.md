# Azure Queue client library for Rust

Azure Queue Storage is a service for storing large numbers of messages.

[Source code] | [Package (crates.io)] | [API reference documentation] | [REST API documentation] | [Product documentation]

## Getting started

**⚠️ Note: The `azure_storage_queue` crate is currently under active development and not all features may be implemented or work as intended. This crate is in beta and not suitable for Production environments. For any general feedback or usage issues, please open a GitHub issue at <https://github.com/Azure/azure-sdk-for-rust/issues>.**

### Install the package

Install the Azure Storage Queue client library for Rust with [cargo]:

```sh
cargo add azure_storage_queue
```

### Prerequisites

- You must have an [Azure subscription] and an [Azure storage account] to use this package.

### Create a storage account

If you wish to create a new storage account, you can use the
[Azure Portal], [Azure PowerShell], or [Azure CLI]:

```sh
# Create a new resource group to hold the storage account.
# Skip this step if using an existing resource group.
az group create --name my-resource-group --location westus2

# Create the storage account
az storage account create -n my-storage-account-name -g my-resource-group
```

#### Authenticate the client

In order to interact with the Azure Queue service, you'll need to create an instance of a client, `QueueClient` or `QueueServiceClient`. The [Azure Identity] library makes it easy to add Microsoft Entra ID support for authenticating Azure SDK clients with their corresponding Azure services:

```rust no_run
use azure_storage_queue::{QueueClient, QueueClientOptions};
use azure_identity::DeveloperToolsCredential;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a QueueClient that will authenticate through Microsoft Entra ID
    let credential = DeveloperToolsCredential::new(None)?;
    let queue_client = QueueClient::new(
        "https://<storage_account_name>.queue.core.windows.net/", // Endpoint
        "<queue_name>",                                           // Queue Name
        Some(credential),                                         // Credential
        Some(QueueClientOptions::default()),                      // QueueClient Options
    )?;
    Ok(())
}
```

#### Permissions

You may need to specify RBAC roles to access Queues via Microsoft Entra ID. Please see [Assign an Azure role for access to queue data] for more details.

## Examples

You can find executable examples for all major SDK functions in:

- [queue_hello_world.rs](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_queue/examples/queue_hello_world.rs) - Getting started: create a queue, send and receive messages
- [queue_client.rs](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_queue/examples/queue_client.rs) - Queue-level operations: metadata, send/peek/receive/delete, TTL/visibility options
- [queue_service_client.rs](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_queue/examples/queue_service_client.rs) - Service-level operations: list queues, service properties, statistics
- [access_policy.rs](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_queue/examples/access_policy.rs) - Set and get queue access policies (stored access policies for SAS)
- [queue_storage_logging.rs](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_queue/examples/queue_storage_logging.rs) - Logging and OpenTelemetry distributed tracing

### Send a message

```rust no_run
use azure_storage_queue::{models::QueueMessage, QueueClient, QueueClientOptions};
use azure_identity::DeveloperToolsCredential;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DeveloperToolsCredential::new(None)?;
    let queue_client = QueueClient::new(
        "https://<storage_account_name>.queue.core.windows.net/",
        "<queue_name>",
        Some(credential),
        Some(QueueClientOptions::default()),
    )?;

    let message = QueueMessage {
        message_text: Some("hello world".to_string()),
    };
    queue_client.send_message(message.try_into()?, None).await?;
    Ok(())
}
```

### Receive messages

```rust no_run
use azure_storage_queue::{QueueClient, QueueClientOptions};
use azure_identity::DeveloperToolsCredential;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DeveloperToolsCredential::new(None)?;
    let queue_client = QueueClient::new(
        "https://<storage_account_name>.queue.core.windows.net/",
        "<queue_name>",
        Some(credential),
        Some(QueueClientOptions::default()),
    )?;

    let response = queue_client.receive_messages(None).await?;
    let messages = response.into_model()?;
    for msg in messages.items.unwrap_or_default() {
        println!("{}", msg.message_text.as_deref().unwrap_or("<empty>"));
    }
    Ok(())
}
```

## Next steps

### Provide feedback

If you encounter bugs or have suggestions, [open an issue](https://github.com/Azure/azure-sdk-for-rust/issues).

## Contributing

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit [https://cla.microsoft.com](https://cla.microsoft.com).

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You'll only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/). For more information, see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.

<!-- LINKS -->
[Azure subscription]: https://azure.microsoft.com/free/
[Azure storage account]: https://learn.microsoft.com/azure/storage/common/storage-account-overview
[Azure Portal]: https://learn.microsoft.com/azure/storage/common/storage-quickstart-create-account?tabs=azure-portal
[Azure PowerShell]: https://learn.microsoft.com/azure/storage/common/storage-quickstart-create-account?tabs=azure-powershell
[Azure CLI]: https://learn.microsoft.com/azure/storage/common/storage-quickstart-create-account?tabs=azure-cli
[cargo]: https://doc.rust-lang.org/cargo/
[Azure Identity]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity
[API reference documentation]: https://docs.rs/crate/azure_storage_queue/latest
[Package (crates.io)]: https://crates.io/crates/azure_storage_queue
[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_queue
[REST API documentation]: https://learn.microsoft.com/rest/api/storageservices/queue-service-rest-api
[Product documentation]: https://learn.microsoft.com/azure/storage/queues/storage-queues-introduction
[Assign an Azure role for access to queue data]: https://learn.microsoft.com/azure/storage/queues/assign-azure-role-data-access?tabs=portal
