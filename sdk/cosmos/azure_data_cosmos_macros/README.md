# Azure Cosmos DB Macros for Rust

This crate provides procedural macros for the Azure Cosmos DB SDK for Rust. It is intended for internal use. No official Microsoft support is provided when using this package directly.

See the [Cosmos SDK project documentation] for the configuration architecture
that these macros support.

## `#[derive(CosmosOptions)]`

Generates layered configuration boilerplate for option group structs, including:

- **View structs** for snapshot-based resolution across layers
- **Builder types** for fluent construction
- **`from_env()`** for environment variable loading
- **`Default` impl** (all `Option<T>` fields default to `None`)

See the Hierarchical Configuration Model specification for details.

[Cosmos SDK project documentation]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/docs/README.md
