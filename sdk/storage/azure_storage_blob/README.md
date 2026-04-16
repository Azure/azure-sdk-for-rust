# Azure Storage Blob client library for Rust

Azure Blob storage is Microsoft's object storage solution for the cloud. Blob storage is optimized for storing massive amounts of unstructured data, such as text or binary data.

[Source code] | [Package (crates.io)] | [API reference documentation] | [REST API documentation] | [Product documentation]

## Getting started

**⚠️ Note: The `azure_storage_blob` crate is currently under active development and not all features may be implemented or work as intended. This crate is in beta and not suitable for Production environments. For any general feedback or usage issues, please open a GitHub issue [here](https://github.com/Azure/azure-sdk-for-rust/issues).**

### Install the package

Install the Azure Storage Blob client library for Rust with [cargo]:

```sh
cargo add azure_storage_blob
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

In order to interact with the Azure Blob Storage service, you'll need to create an instance of a client, `BlobClient`, `BlobContainerClient`, or `BlobServiceClient`. The [Azure Identity] library makes it easy to add Microsoft Entra ID support for authenticating Azure SDK clients with their corresponding Azure services:

```rust no_run
use azure_storage_blob::{BlobClient, BlobClientOptions};
use azure_identity::DeveloperToolsCredential;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a BlobClient that will authenticate through Microsoft Entra ID
    let credential = DeveloperToolsCredential::new(None)?;
    let blob_client = BlobClient::new(
        "https://<storage_account_name>.blob.core.windows.net/", // Endpoint
        "container_name",                                        // Container Name
        "blob_name",                                             // Blob Name
        Some(credential),                                        // Credential
        Some(BlobClientOptions::default()),                      // BlobClient Options
    )?;
    Ok(())
}
```

#### Permissions

You may need to specify RBAC roles to access Blob Storage via Microsoft Entra ID. Please see [Assign an Azure role for access to blob data] for more details.

## Examples

You can find executable examples for all major SDK functions in:

* [blob_hello_world.rs](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_blob/examples/blob_hello_world.rs) - Getting started: create a container, upload and download a blob
* [blob_container_client.rs](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_blob/examples/blob_container_client.rs) - Container-level operations: metadata, list blobs with continuation, access policies
* [blob_service_client.rs](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_blob/examples/blob_service_client.rs) - Service-level operations: list containers, service properties, statistics
* [block_blob_client.rs](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_blob/examples/block_blob_client.rs) - Block blob operations: staged block upload, copy from URL
* [append_blob_client.rs](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_blob/examples/append_blob_client.rs) - Append blob operations: create, append blocks, seal
* [page_blob_client.rs](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_blob/examples/page_blob_client.rs) - Page blob operations: create, upload/clear pages, list page ranges, resize
* [blob_storage_logging.rs](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_blob/examples/blob_storage_logging.rs) - Logging and OpenTelemetry distributed tracing
* [storage_error.rs](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_blob/examples/storage_error.rs) - Structured error handling with `StorageError`

### Upload a blob

```rust no_run
use azure_core::http::RequestContent;
use azure_storage_blob::{BlobClient, BlobClientOptions};
use azure_identity::DeveloperToolsCredential;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DeveloperToolsCredential::new(None)?;
    let blob_client = BlobClient::new(
        "https://<storage_account_name>.blob.core.windows.net/",
        "container_name",
        "blob_name",
        Some(credential),
        Some(BlobClientOptions::default()),
    )?;

    let data = b"hello world";
    blob_client
        .upload(
            RequestContent::from(data.to_vec()), // Data
            None,                                // Upload Options
        )
        .await?;
    Ok(())
}
```

### Download a blob

```rust no_run
use azure_storage_blob::{BlobClient, BlobClientOptions};
use azure_identity::DeveloperToolsCredential;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DeveloperToolsCredential::new(None)?;
    let blob_client = BlobClient::new(
        "https://<storage_account_name>.blob.core.windows.net/", // Endpoint
        "container_name",                                        // Container Name
        "blob_name",                                             // Blob Name
        Some(credential),                                        // Credential
        Some(BlobClientOptions::default()),                      // BlobClient Options
    )?;
    let blob_properties = blob_client.get_properties(None).await?;
    Ok(())
}
```

## Remarks

### Automatic decompression with custom HTTP transports

By default, all storage clients create an HTTP transport with automatic decompression disabled,
which is required for partitioned (multi-part) downloads to work correctly. If you set a custom transport
in client options (e.g., a `reqwest::Client` with gzip enabled) without disabling automatic
decompression, partitioned downloads via [`BlobClient::download`](https://docs.rs/azure_storage_blob/latest/azure_storage_blob/clients/struct.BlobClient.html#method.download).
If you need to provide a custom transport, disable automatic decompression to be consistent with default SDK behavior.

## Next Steps

### Provide Feedback

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
[cargo]: https://dev-doc.rust-lang.org/stable/cargo/commands/cargo.html
[Azure Identity]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity
[API reference documentation]: https://docs.rs/crate/azure_storage_blob/latest
[Package (crates.io)]: https://crates.io/crates/azure_storage_blob
[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_blob
[REST API documentation]: https://learn.microsoft.com/rest/api/storageservices/blob-service-rest-api
[Product documentation]: https://learn.microsoft.com/azure/storage/blobs/storage-blobs-overview
[Assign an Azure role for access to blob data]: https://learn.microsoft.com/azure/storage/blobs/assign-azure-role-data-access?tabs=portal
