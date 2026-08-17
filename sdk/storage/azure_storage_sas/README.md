# Azure Storage User Delegation SAS Builder for Rust

This crate provides a type-safe builder for constructing **user delegation** Shared Access Signature (SAS) tokens for Azure Storage resources. Account SAS and stored access policies are not supported.

[Source code] | [Package (crates.io)] | [API reference documentation] | [REST API documentation] | [Product documentation]

## Getting started

### Install the package

Install the Azure Storage User Delegation SAS builder for Rust with [cargo]:

```sh
cargo add azure_storage_sas
```

### Prerequisites

- You must have an [Azure subscription] and an [Azure storage account] to use this package.
- A `UserDelegationKey` obtained from `BlobServiceClient::get_user_delegation_key` (in `azure_storage_blob`) or `QueueServiceClient::get_user_delegation_key` (in `azure_storage_queue`). The key is signed by Microsoft Entra ID and is what binds the SAS to a delegated identity.

### Which API should I use?

Two builders are provided:

- **`SasTokenBuilder`** — produces a [`SasToken`], the signed query string (e.g. `sv=...&sr=b&...&sig=...`). Use this when you need to attach the query string to an existing URL yourself. `build()` is infallible.
- **`SasUrlBuilder`** — produces a [`SasUrl`], a complete URL with the signed query string already embedded. Use this when you want the full URL ready to share. `build()` returns a `Result`. Use [`SasUrlBuilder::endpoint`] to override the default `https://{account}.{service}.core.windows.net` base (e.g. for Azurite or sovereign clouds).

Both builders expose the same resource selectors and permission setters.

## Examples

### Generate a blob SAS URL

Use `SasUrlBuilder` to produce a complete signed URL in one step:

```rust ignore blob_sas_url
use azure_storage_sas::{SasUrlBuilder, UserDelegationKey};
use time::OffsetDateTime;

let url = SasUrlBuilder::new(
        "myaccount",
        &udk,
        OffsetDateTime::now_utc() + time::Duration::hours(1),
    )?
    .blob("images", "photo.jpg")
    .read()
    .content_type("image/jpeg")
    .build()?;

// `url` is a complete signed URL ready to share
println!("{url}");
```

### Generate a blob SAS token (query string only)

Use `SasTokenBuilder` when you need just the signed query string to attach to an existing URL:

```rust ignore blob_sas_token
use azure_core::{http::Url, time::{Duration, OffsetDateTime}};
use azure_storage_sas::{SasTokenBuilder, UserDelegationKey};

let storage_account_name = "myaccount";
let container_name = "images";
let blob_name = "photo.jpg";

let token = SasTokenBuilder::new(
        storage_account_name,
        &udk,
        OffsetDateTime::now_utc() + Duration::hours(1),
    )?
    .blob(container_name, blob_name)
    .read()
    .content_type("image/jpeg")
    .build();

// `set_query` overwrites any existing query; if the URL already has one
// (e.g. version/snapshot), the token must be appended instead.
let mut sas_url = Url::parse(&format!(
    "https://{storage_account_name}.blob.core.windows.net/{container_name}/{blob_name}"
))?;
sas_url.set_query(Some(&*token));
```

### Scope a container SAS to HTTPS and a single IP range

Layer optional restrictions onto a container-level SAS: limit the caller to HTTPS only, pin them to a corporate egress range, and grant just enough permissions to list and read:

```rust ignore container_ip_range_sas
use azure_storage_sas::{SasTokenBuilder, SasIpRange, SasProtocol};
use std::net::Ipv4Addr;
use time::OffsetDateTime;

let token = SasTokenBuilder::new(
        "myaccount",
        &udk,
        OffsetDateTime::now_utc() + time::Duration::hours(4),
    )?
    .container("logs")
    .read()
    .list()
    .protocol(SasProtocol::Https)
    .ip_range(SasIpRange::InclusiveRange {
        start: Ipv4Addr::new(10, 0, 0, 1),
        end: Ipv4Addr::new(10, 0, 0, 255),
    })
    .build();
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
[cargo]: https://doc.rust-lang.org/cargo/
[API reference documentation]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_sas
[Package (crates.io)]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_sas
[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/storage/azure_storage_sas
[REST API documentation]: https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas
[Product documentation]: https://learn.microsoft.com/azure/storage/common/storage-sas-overview
