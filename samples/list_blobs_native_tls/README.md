# List Blobs with native-tls

This sample lists blobs from Azure Storage using [reqwest] with [native-tls],
which uses `schannel` on Windows and `openssl` everywhere else.
Azure SDK crates enable `aws-lc-rs` by default, so this sample disables default features
on Azure SDK crates and enables reqwest's `native-tls` feature instead.

## Prerequisites

- An Azure subscription and a storage account (or use `azd up` below).
- Azure CLI and Azure Developer CLI (`azd`) installed.
- Rust toolchain installed.

## Usage

### Provision Azure resources

This sample includes [Azure Developer CLI][azd] infrastructure to provision a
storage account and upload sample blobs:

```sh
azd up
```

This provisions a storage account, creates a `samples` container, assigns you
the Storage Blob Data Contributor role, and uploads a few sample blobs.

### Run the sample

```sh
export AZURE_STORAGE_ACCOUNT_NAME=$(azd env get-value AZURE_STORAGE_ACCOUNT_NAME)
cargo run
```

You can also pass the account name explicitly:

```sh
cargo run -- --account-name <storage_account_name>
```

### Clean up

```sh
azd down
```

[azd]: https://learn.microsoft.com/azure/developer/azure-developer-cli/
[reqwest]: https://crates.io/crates/reqwest
[native-tls]: https://crates.io/crates/native-tls
