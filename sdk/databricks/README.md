# Azure SDK for Rust - Azure Databricks

Azure Databricks crate for the unofficial Microsoft Azure SDK for Rust. This crate is part of a collection of crates: for more information please refer to [https://github.com/azure/azure-sdk-for-rust](https://github.com/azure/azure-sdk-for-rust).

## Building

This crate has several features depending on what part of the databricks service you need:

* clusters

By default all of these features are turned on. If you only need a specific feature, make sure to specify which features you need in your `Cargo.toml` file like so:

```toml
azure_databricks = { version = "0.1", default-features = false, features = ["clusters"] }
```
