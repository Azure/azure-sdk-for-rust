# Azure Queue client library for Rust

Azure Queue Storage is a service for storing large numbers of messages.

[Source code] | [Package (crates.io)] | [API reference documentation] | [REST API documentation] | [Product documentation]

## Getting started

**⚠️ Note: The `azure_storage_queue` crate is currently under active development and not all features may be implemented or work as intended. This crate is in beta and not suitable for Production environments. For any general feedback or usage issues, please open a GitHub issue [here](https://github.com/Azure/azure-sdk-for-rust/issues).**

### Install the package

Install the Azure Storage Queue client library for Rust with [cargo]:

```sh
cargo add azure_storage_queue
```

### Prerequisites

* You must have an [Azure subscription] and an [Azure storage account] to use this package.

### Create a storage account

If you wish to create a new storage account, you can use the
[Azure Portal], [Azure PowerShell], or [Azure CLI]:

```sh
# Create a new resource group to hold the storage account -
# if using an existing resource group, skip this step
az group create --name my-resource-group --location westus2

# Create the storage account
az storage account create -n my-storage-account-name -g my-resource-group
```

#### Authenticate the client

In order to interact with the Azure Queue service, you'll need to create an instance of a client, `QueueClient`. The [Azure Identity] library makes it easy to add Microsoft Entra ID support for authenticating Azure SDK clients with their corresponding Azure services:

```rust no_run
use azure_storage_queue::clients::{QueueClient, QueueClientOptions};
use azure_identity::DefaultAzureCredential;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a QueueClient that will authenticate through Microsoft Entra ID
    let credential = DefaultAzureCredential::new()?;
    let queue_client = QueueClient::new(
        "https://<storage_account_name>.blob.core.windows.net/", // endpoint
        "queue-name",                                            // queue name
        credential,                                              // credential
        Some(QueueClientOptions::default()),                     // QueueClient options
    )?;
    Ok(())
}
```

#### Permissions

You may need to specify RBAC roles to access Queues via Microsoft Entra ID. Please see [Assign an Azure role for access to queue data] for more details.

## Features

The following methods are available on the ```QueueClient``` class:

- ```new```: Create a new instance of the ```QueueClient```.
- ```create```: Creates a new queue, will fail if the queue already exists.
- ```create_if_not_exists```: Creates a new queue, will _not_ fail if the queue already exists.
- ```delete```: Deletes a queue, will fail if the queue does not exist.
- ```delete_if_exists```: Deletes a queue, will _not_ fail if the queue does not exist.
- ```delete_message```: Deletes a single message from the queue.
- ```delete_messages```: Deletes all the messages in a queue. Requires Account Owner permission or it will fail.
- ```exists```: Returns bool representing whether the queue exists or not.
- ```get_metadata```: Returns metadata for the queue.
- ```get_properties```: Returns the properties of the queue service.
- ```peek_message```: Peeks a single message from the front of the queue without removing it.
- ```peek_messages```: Peeks multiple messages from the front of the queue without removing them.
- ```receive_message```: Receive a single message from the queue.
- ```receive_messages```: Receive multiple messages from the queue. The number of messages to return is determined by the ```AzureQueueStorageMessagesOperationsClientDequeueOptions::number_of_messages``` property.
- ```send_message```: Sends a message to the queue.
- ```set_metadata```: Sets metadata on the queue.
- ```update_message```: Updates a specific message in the queue.

## Examples

<!-- TODO: Update the link below when the PR is merged -->
Executable examples of all the functions provided by this SDK can be found in the [queue_client.rs]<!--(https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_queue/examples/queue_client.rs)--> file in the examples directory.

## Next steps

### Provide feedback

If you encounter bugs or have suggestions, [open an issue](https://github.com/Azure/azure-sdk-for-rust/issues).

## Contributing

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit [https://cla.microsoft.com](https://cla.microsoft.com).

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You'll only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/). For more information, see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.

<!-- TODO: Update the links below when the crate is published -->
<!-- LINKS -->
[Azure subscription]: https://azure.microsoft.com/free/
[Azure storage account]: https://learn.microsoft.com/azure/storage/common/storage-account-overview
[Azure Portal]: https://learn.microsoft.com/azure/storage/common/storage-quickstart-create-account?tabs=azure-portal
[Azure PowerShell]: https://learn.microsoft.com/azure/storage/common/storage-quickstart-create-account?tabs=azure-powershell
[Azure CLI]: https://learn.microsoft.com/azure/storage/common/storage-quickstart-create-account?tabs=azure-cli
[cargo]: https://dev-doc.rust-lang.org/stable/cargo/commands/cargo.html
[Azure Identity]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity
<!--[API reference documentation]: https://docs.rs/crate/azure_storage_queue/latest-->
<!--[Package (crates.io)]: https://crates.io/crates/azure_storage_queue-->
[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_queue
[REST API documentation]: https://learn.microsoft.com/rest/api/storageservices/blob-service-rest-api
[Product documentation]: https://learn.microsoft.com/azure/storage/blobs/storage-blobs-overview
[Assign an Azure role for access to queue data]: https://learn.microsoft.com/azure/storage/queues/assign-azure-role-data-access?tabs=portal
