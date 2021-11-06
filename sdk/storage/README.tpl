# Azure SDK for Rust - Azure Storage 

{{readme}}

## Usage

This crate has several features depending on what part of the storage service you need:
* Account
* Blob
* Queue
* Table

By default all of these features are turned on. If you only need a specific feature, make sure to specify which features you need in your `Cargo.toml` file like so:

```toml
[dependencies]
# specifying strictly "blob" feature
{{crate}} = { version = "{{version}}", git = "https://github.com/Azure/azure-sdk-for-rust", default-features = false, features = ["blob"] }
```
