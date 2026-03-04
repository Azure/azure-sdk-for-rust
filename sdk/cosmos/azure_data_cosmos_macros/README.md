# Azure Cosmos DB Macros for Rust

This crate provides procedural macros for the Azure Cosmos DB SDK for Rust.

## `#[derive(CosmosOptions)]`

Generates layered configuration boilerplate for option group structs, including:

- **View structs** for snapshot-based resolution across layers
- **Builder types** for fluent construction
- **`from_env()`** for environment variable loading
- **`Default` impl** (all `Option<T>` fields default to `None`)

See the [Hierarchical Configuration Model](../azure_data_cosmos/docs/HierarchicalConfigModel.md) specification for details.
