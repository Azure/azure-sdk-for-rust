# Azure Storage Blob client library for Rust

Azure Blob storage is Microsoft's object storage solution for the cloud. Blob storage is optimized for storing massive amounts of unstructured data, such as text or binary data.

[Source code] | [Package (crates.io)] | [API reference documentation] | [REST API documentation] | [Product documentation]

## Getting started

### Install the package

Install the Azure Storage Blob client library for Rust with [cargo]:

```sh
cargo add azure_storage_blob
```

### Prerequisites

- You must have an [Azure subscription] and an [Azure storage account] to use this package.

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

In order to interact with the Azure Blob Storage service, you'll need to create an instance of a client, `BlobClient`, `BlobContainerClient`, or `BlobServiceClient`. `BlobServiceClient` is the recommended entry point. Construct it once using `BlobServiceClient::new()`, then call `BlobServiceClient::blob_container_client()`  or `BlobServiceClient::blob_client()` to get a `BlobContainerClient` or `BlobClient` respectively. If you already have a fully-formed (for example, SAS-scoped) URL for a single container or blob, call `BlobContainerClient::new()` or `BlobClient::new()` with that URL directly instead.

The [Azure Identity] library makes it easy to add Microsoft Entra ID support for authenticating Azure SDK clients with their corresponding Azure services:

```rust no_run
use azure_core::http::Url;
use azure_storage_blob::BlobServiceClient;
use azure_identity::DeveloperToolsCredential;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a BlobServiceClient that will authenticate through Microsoft Entra ID
    let credential = DeveloperToolsCredential::new(None)?;
    let service_url = Url::parse("https://<storage_account_name>.blob.core.windows.net/")?;
    let service_client = BlobServiceClient::new(
        service_url,
        Some(credential),
        None,
    )?;

    // Derive container and blob clients by name.
    let container_client = service_client.blob_container_client("<container_name>");
    let blob_client = container_client.blob_client("<blob_name>");
    Ok(())
}
```

#### Permissions

You may need to specify RBAC roles to access Blob Storage via Microsoft Entra ID. Please see [Assign an Azure role for access to blob data] for more details.

## Examples

You can find executable examples for all major SDK functions in:

- [blob_hello_world.rs](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_blob/examples/blob_hello_world.rs) - Getting started: create a container, upload and download a blob
- [blob_client.rs](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_blob/examples/blob_client.rs) - Blob-level operations: exists, metadata, index tags, access tier
- [blob_container_client.rs](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_blob/examples/blob_container_client.rs) - Container-level operations: metadata, list blobs with continuation, access policies
- [blob_service_client.rs](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_blob/examples/blob_service_client.rs) - Service-level operations: list containers, service properties, statistics
- [block_blob_client.rs](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_blob/examples/block_blob_client.rs) - Block blob operations: staged block upload, copy from URL
- [append_blob_client.rs](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_blob/examples/append_blob_client.rs) - Append blob operations: create, append blocks, seal
- [page_blob_client.rs](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_blob/examples/page_blob_client.rs) - Page blob operations: create, upload/clear pages, list page ranges, resize
- [blob_storage_upload_file.rs](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_blob/examples/blob_storage_upload_file.rs) - Upload a local file with streaming support for large files
- [samples/storage_blob_logging](https://github.com/Azure/azure-sdk-for-rust/tree/main/samples/storage_blob_logging) - Logging and OpenTelemetry distributed tracing
- [storage_error.rs](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_blob/examples/storage_error.rs) - Structured error handling with `StorageError`

### Upload a blob

```rust no_run
use azure_core::http::{RequestContent, Url};
use azure_storage_blob::BlobServiceClient;
use azure_identity::DeveloperToolsCredential;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DeveloperToolsCredential::new(None)?;
    let service_url = Url::parse("https://<storage_account_name>.blob.core.windows.net/")?;
    let service_client = BlobServiceClient::new(service_url, Some(credential), None)?;
    let blob_client = service_client.blob_client("<container_name>", "<blob_name>");

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
use azure_core::http::Url;
use azure_storage_blob::BlobServiceClient;
use azure_identity::DeveloperToolsCredential;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DeveloperToolsCredential::new(None)?;
    let service_url = Url::parse("https://<storage_account_name>.blob.core.windows.net/")?;
    let service_client = BlobServiceClient::new(service_url, Some(credential), None)?;
    let blob_client = service_client.blob_client("<container_name>", "<blob_name>");

    let response = blob_client.download(None).await?;
    let data = String::from_utf8(response.body.collect().await?.into())?;
    println!("Downloaded: {data}");
    Ok(())
}
```

## Remarks

### Generating SAS URLs

Use the [`azure_storage_sas`] crate to create a user delegation SAS. Obtain a `UserDelegationKey` from `BlobServiceClient::get_user_delegation_key`, build the token with `SasBuilder`, then set it as the query string on the resource URL.

```rust no_run
use azure_core::{
    http::{RequestContent, Url, XmlFormat},
    time::OffsetDateTime,
};
use azure_storage_blob::{models::KeyInfo, BlobServiceClient};
use azure_storage_sas::SasBuilder;
use azure_identity::DeveloperToolsCredential;
use time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DeveloperToolsCredential::new(None)?;
    let storage_account_name = "<storage_account_name>";
    let container_name = "<container_name>";
    let blob_name = "<blob_name>";

    let service_url = Url::parse(&format!(
        "https://{storage_account_name}.blob.core.windows.net/"
    ))?;
    let service_client = BlobServiceClient::new(service_url, Some(credential), None)?;

    // Request a user delegation key from the service. The key is signed by
    // Microsoft Entra ID and binds the SAS to the caller's identity.
    let now = OffsetDateTime::now_utc();
    let key_info = KeyInfo {
        start: Some(now),
        expiry: Some(now + Duration::hours(1)),
        ..Default::default()
    };
    let request_content: RequestContent<KeyInfo, XmlFormat> = key_info.try_into()?;
    let udk = service_client
        .get_user_delegation_key(request_content, None)
        .await?
        .into_model()?;

    // Build a read-only SAS token for a single blob, then set it on the blob URL.
    let token = SasBuilder::new(storage_account_name, &udk, now + Duration::hours(1))?
        .blob(container_name, blob_name)
        .read()
        .build();
    let mut sas_url = Url::parse(&format!(
        "https://{storage_account_name}.blob.core.windows.net/{container_name}/{blob_name}"
    ))?;
    sas_url.set_query(Some(&token));

    println!("{sas_url}");
    Ok(())
}
```

### Automatic decompression with custom HTTP transports

By default, all storage clients create an HTTP transport with automatic decompression disabled,
which is required for partitioned (multi-part) downloads to work correctly. If you set a custom transport
in client options (e.g., a `reqwest::Client` with gzip enabled) without disabling automatic
decompression, partitioned downloads via [`BlobClient::download`](https://docs.rs/azure_storage_blob/latest/azure_storage_blob/clients/struct.BlobClient.html#method.download) may not work correctly.
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
[cargo]: https://doc.rust-lang.org/cargo/
[Azure Identity]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity
[API reference documentation]: https://docs.rs/crate/azure_storage_blob/latest
[Package (crates.io)]: https://crates.io/crates/azure_storage_blob
[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_blob
[REST API documentation]: https://learn.microsoft.com/rest/api/storageservices/blob-service-rest-api
[Product documentation]: https://learn.microsoft.com/azure/storage/blobs/storage-blobs-overview
[Assign an Azure role for access to blob data]: https://learn.microsoft.com/azure/storage/blobs/assign-azure-role-data-access?tabs=portal
[`azure_storage_sas`]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_sas
