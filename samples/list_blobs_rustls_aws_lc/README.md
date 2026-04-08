# List Blobs with rustls and aws-lc-rs

This sample lists blobs from Azure Storage using [reqwest] with [rustls] and the
default [aws-lc-rs] crypto provider. Azure SDK crates enable `native-tls` by
default, so this sample disables default features on Azure SDK crates and enables
reqwest's `rustls` feature instead.

## Prerequisites

- An Azure subscription and a storage account (or use `azd up` below).
- Azure CLI and Azure Developer CLI (`azd`) installed.
- Rust toolchain installed.

### Windows

Building [aws-lc-rs] on Windows requires Visual Studio Build Tools 2017 or later
with the C++/CLI and C++ CMake components. NASM is recommended for x86-64 but can
be avoided using prebuilt NASM objects by enabling the `prebuilt-nasm` feature or
setting `AWS_LC_SYS_PREBUILT_NASM=1`.

See the [aws-lc-rs Windows requirements] for full details.

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

[aws-lc-rs]: https://crates.io/crates/aws-lc-rs
[aws-lc-rs Windows requirements]: https://aws.github.io/aws-lc-rs/requirements/windows.html
[azd]: https://learn.microsoft.com/azure/developer/azure-developer-cli/
[reqwest]: https://crates.io/crates/reqwest
[rustls]: https://crates.io/crates/rustls
