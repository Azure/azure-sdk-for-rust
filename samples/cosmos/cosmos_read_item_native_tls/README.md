# Read a Cosmos DB item with native-tls

This sample reads a single item from an Azure Cosmos DB container using
[reqwest] with [native-tls], which uses `schannel` on Windows and `openssl`
everywhere else. Azure SDK crates enable `aws-lc-rs` by default, so this sample
disables default features on Azure SDK crates and enables reqwest's `native-tls`
feature instead.

## Prerequisites

- An Azure subscription and a Cosmos DB account (or use `azd up` below).
- Azure CLI and Azure Developer CLI (`azd`) installed.
- Rust toolchain installed.

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
[reqwest]: https://crates.io/crates/reqwest
[native-tls]: https://crates.io/crates/native-tls
