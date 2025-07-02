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

### QueueServiceClient

The `QueueServiceClient` provides operations to interact with the Azure Storage Queue service at the account level.

| Method                | Parameters                                                                                                                                         | Return Type                                                            | Description                                                                                                                                                                                              |
|-----------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `new`                 | `endpoint: &str`<br>`credential: Arc<dyn TokenCredential>`<br>`options: Option<QueueServiceClientOptions>`                                         | `Result<Self>`                                                         | Creates a new QueueServiceClient using Entra ID authentication                                                                                                                                           |
| `endpoint`            | `&self`                                                                                                                                            | `&Url`                                                                 | Returns the endpoint URL of the Azure storage account                                                                                                                                                    |
| `queue_client`        | `&self`<br>`queue_name: String`                                                                                                                    | `QueueClient`                                                          | Returns a new QueueClient instance for a specific queue                                                                                                                                                  |
| `create_queue`        | `&self`<br>`queue_name: &str`<br>`options: Option<QueueClientCreateOptions<'_>>`                                                                   | `Result<Response<(), NoFormat>>`                                       | Creates a new queue under the account                                                                                                                                                                    |
| `delete_queue`        | `&self`<br>`queue_name: &str`<br>`options: Option<QueueClientDeleteOptions<'_>>`                                                                   | `Result<Response<(), NoFormat>>`                                       | Permanently deletes the specified queue                                                                                                                                                                  |
| `get_properties`      | `&self`<br>`options: Option<QueueServiceClientGetPropertiesOptions<'_>>`                                                                           | `Result<Response<StorageServiceProperties, XmlFormat>>`                | Retrieves the properties of the queue service                                                                                                                                                            |
| `set_properties`      | `&self`<br>`storage_service_properties: RequestContent<StorageServiceProperties>`<br>`options: Option<QueueServiceClientSetPropertiesOptions<'_>>` | `Result<Response<(), NoFormat>>`                                       | Sets the properties of the queue service                                                                                                                                                                 |
| `list_queues`         | `&self`<br>`options: Option<QueueServiceClientListQueuesOptions<'_>>`                                                                              | `Result<PageIterator<Response<ListQueuesResponse, XmlFormat>>>`        | Lists queues in the storage account with pagination                                                                                                                                                      |
| `get_statistics`      | `&self`<br>`options: Option<QueueServiceClientGetStatisticsOptions<'_>>`                                                                           | `Result<Response<StorageServiceStats, XmlFormat>>`                     | Retrieves statistics related to replication for the Queue service. It is only available on the secondary location endpoint when read-access geo-redundant replication is enabled for the storage account |

---

### QueueClient

The `QueueClient` provides operations to interact with a specific Azure Storage Queue.

| Method       | Parameters                                                                                                                | Return Type    | Description                                                  |
|--------------|---------------------------------------------------------------------------------------------------------------------------|----------------|--------------------------------------------------------------|
| `new`        | `endpoint: &str`<br>`queue_name: &str`<br>`credential: Arc<dyn TokenCredential>`<br>`options: Option<QueueClientOptions>` | `Result<Self>` | Creates a new QueueClient using Entra ID authentication      |
| `endpoint`   | `&self`                                                                                                                   | `&Url`         | Returns the endpoint URL of the Azure storage account        |
| `queue_name` | `&self`                                                                                                                   | `&str`         | Returns the name of the queue this client is associated with |

#### Queue Management

| Method                 | Parameters                                                 | Return Type                      | Description                                 |
|------------------------|------------------------------------------------------------|----------------------------------|---------------------------------------------|
| `create`               | `&self`<br>`options: Option<QueueClientCreateOptions<'_>>` | `Result<Response<(), NoFormat>>` | Creates a new queue                         |
| `create_if_not_exists` | `&self`<br>`options: Option<QueueClientCreateOptions<'_>>` | `Result<Response<(), NoFormat>>` | Creates a queue if it doesn't already exist |
| `delete`               | `&self`<br>`options: Option<QueueClientDeleteOptions<'_>>` | `Result<Response<(), NoFormat>>` | Permanently deletes the queue               |
| `delete_if_exists`     | `&self`<br>`options: Option<QueueClientDeleteOptions<'_>>` | `Result<Response<(), NoFormat>>` | Deletes the queue if it exists              |
| `exists`               | `&self`                                                    | `Result<bool>`                   | Checks if the queue exists                  |
| `clear`                | `&self`<br>`options: Option<QueueClientClearOptions<'_>>`  | `Result<Response<(), NoFormat>>` | Clears all messages in the queue            |

#### Metadata Operations

| Method         | Parameters                                                      | Return Type                                                | Description                         |
|----------------|-----------------------------------------------------------------|------------------------------------------------------------|-------------------------------------|
| `set_metadata` | `&self`<br>`options: Option<QueueClientSetMetadataOptions<'_>>` | `Result<Response<(), NoFormat>>`                           | Sets the metadata for the queue     |
| `get_metadata` | `&self`<br>`options: Option<QueueClientGetMetadataOptions<'_>>` | `Result<Response<QueueClientGetMetadataResult, NoFormat>>` | Retrieves the metadata of the queue |

#### Message Operations

| Method             | Parameters                                                                                                                                       | Return Type                                                      | Description                                             |
|--------------------|--------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------|---------------------------------------------------------|
| `send_message`     | `&self`<br>`queue_message: RequestContent<QueueMessage>`<br>`options: Option<QueueClientSendMessageOptions<'_>>`                                 | `Result<Response<Option<SentMessage>, XmlFormat>>`               | Sends a message to the queue                            |
| `receive_message`  | `&self`<br>`options: Option<QueueClientReceiveMessagesOptions<'_>>`                                                                              | `Result<Response<Option<ReceivedMessage>, XmlFormat>>`           | Retrieves a single message from the front of the queue  |
| `receive_messages` | `&self`<br>`options: Option<QueueClientReceiveMessagesOptions<'_>>`                                                                              | `Result<Response<ListOfReceivedMessage, XmlFormat>>`             | Retrieves multiple messages from the front of the queue |
| `peek_message`     | `&self`<br>`options: Option<QueueClientPeekMessagesOptions<'_>>`                                                                                 | `Result<Response<Option<PeekedMessage>, XmlFormat>>`             | Peeks a single message without removing it              |
| `peek_messages`    | `&self`<br>`options: Option<QueueClientPeekMessagesOptions<'_>>`                                                                                 | `Result<Response<ListOfPeekedMessage, XmlFormat>>`               | Peeks multiple messages without removing them           |
| `delete_message`   | `&self`<br>`message_id: &str`<br>`pop_receipt: &str`<br>`options: Option<QueueClientDeleteMessageOptions<'_>>`                                   | `Result<Response<(), NoFormat>>`                                 | Deletes a specific message from the queue               |
| `update_message`   | `&self`<br>`message_id: &str`<br>`pop_receipt: &str`<br>`visibility_timeout: i32`<br>`options: Option<QueueClientUpdateOptions<'_>>`             | `Result<Response<(), NoFormat>>`                                 | Updates a specific message in the queue                 |

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
