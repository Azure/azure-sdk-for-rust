# Azure SDK for Rust

This repository is for the development of the [unofficial](https://github.com/Azure/azure-sdk-for-rust/blob/main/FAQ.md#why-is-it-unofficial) Azure SDK for Rust.

## Crates

[All Azure SDK for Rust crates](https://crates.io/teams/github:azure:azure-sdk-publish-rust) are published on crates.io.

### SDK
These [SDK crates](sdk) are available:
- [azure_core](https://crates.io/crates/azure_core)
- [azure_identity](https://crates.io/crates/azure_identity)
- [azure_data_cosmos](https://crates.io/crates/azure_data_cosmos)
- [azure_data_tables](https://crates.io/crates/azure_data_tables)
- [azure_iot_hub](https://crates.io/crates/azure_iot_hub)
- [azure_security_keyvault](https://crates.io/crates/azure_security_keyvault)
- [azure_storage_blobs](https://crates.io/crates/azure_storage_blobs)
- [azure_storage_datalake](https://crates.io/crates/azure_storage_datalake)
- [azure_storage_queues](https://crates.io/crates/azure_storage_queues)

### Services
Azure service crates generated from [Azure REST API Specifications](https://github.com/Azure/azure-rest-api-specs) are available in [services](services).

## Status

🚨 WARNING 🚨: This project is under active development. Be aware that large breaking changes will happen before 1.0 is reached.

This project is the successor to the `azure_sdk*` crates from [MindFlavor/AzureSDKForRust](https://github.com/MindFlavor/AzureSDKForRust). The crates have been renamed, so those older crates should be considered fully deprecated. See [history](HISTORY.md) for more details.

## Project Structure

Each supported Azure service is its own separate crate.

Building each crate should be as straight forward as `cargo build`, but check each crate's README for more specific information.

### Mock testing framework

This library comes with a testing framework that executes against prerecorded sessions to quickly validate code changes without incurring in Azure costs. You can read more about it in the [Mock testing framework's README](docs/mock_transport.md).

## Contributing

This project welcomes contributions and suggestions.  Most contributions require you to agree to a
Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us
the rights to use your contribution. For details, visit https://cla.opensource.microsoft.com.

When you submit a pull request, a CLA bot will automatically determine whether you need to provide
a CLA and decorate the PR appropriately (e.g., status check, comment). Simply follow the instructions
provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/).
For more information see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or
contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.
