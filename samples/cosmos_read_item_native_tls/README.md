# Read a Cosmos DB item with native-tls

This sample reads a single item from an Azure Cosmos DB container using
the platform's [native TLS][native-tls] stack instead of the default [rustls]
backend. Azure SDK crates enable `rustls` by default, so this sample disables
default features on `azure_data_cosmos` and enables the `native_tls` feature
instead.

## Prerequisites

- An Azure subscription and a Cosmos DB account (or use `azd up` below).
- Azure CLI and Azure Developer CLI (`azd`) installed.
- Rust toolchain installed.

### Linux

Building with `native-tls` requires OpenSSL development headers. Install them
with your distribution's package manager:

```sh
# Debian / Ubuntu
sudo apt-get install pkg-config libssl-dev

# Fedora / RHEL
sudo dnf install openssl-devel
```

### macOS

No extra dependencies are needed — `native-tls` uses Security Framework.

### Windows

No extra dependencies are needed — `native-tls` uses Schannel.

## Usage

### Provision Azure resources

This sample includes [Azure Developer CLI][azd] infrastructure to provision a
Cosmos DB account with a database, container, and a sample item:

```sh
azd up
```

This provisions a Cosmos DB account, creates a `SampleDB` database and
`SampleContainer` container, assigns you the necessary data-plane RBAC role,
and inserts a sample item.

### Run the sample

```sh
export AZURE_COSMOS_ENDPOINT=$(azd env get-value AZURE_COSMOS_ENDPOINT)
cargo run
```

You can also pass arguments explicitly:

```sh
cargo run -- --endpoint <endpoint> --database SampleDB --container SampleContainer --item-id 1 --partition-key sample-partition
```

### Clean up

```sh
azd down
```

[azd]: https://learn.microsoft.com/azure/developer/azure-developer-cli/
[native-tls]: https://crates.io/crates/native-tls
[reqwest]: https://crates.io/crates/reqwest
[rustls]: https://crates.io/crates/rustls
