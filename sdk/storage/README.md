# Azure SDK for Rust - Azure Storage 

Azure storage crate for the unofficial Microsoft Azure SDK for Rust. This crate is part of a collection of crates: for more information please refer to [https://github.com/azure/azure-sdk-for-rust](https://github.com/azure/azure-sdk-for-rust).

## Building

This crate has several features depending on what part of the storage service you need:
* Account
* Blob
* Queue
* Table

By default all of these features are turned on. If you only need a specific feature, make sure to specify which features you need in your `Cargo.toml` file like so:

```toml
azure_storage = { version = "0.2", default-features = false, features = ["blob"] }
```
