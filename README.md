# \[Unofficial\] Azure SDK for Rust

This repository is for the development of the *unofficial* Azure SDK for Rust. It is *unofficial* because it is not yet supported by Azure Support or the Azure SDK team. It has been built primarily by volunteers on their own time. For more information, see the [project history](HISTORY.md) or [FAQs](FAQ.md):
- When will the crates be published to crates.io?
- How do we build a case for making it official?

## Crates

### SDK
These crates are available from in (sdk):
- azure_core
- azure_identity
- azure_cosmos
- azure_storage
- azure_security_keyvault

### Services
More than 200 Azure service crates are available in [services](services). They are generated from the [Azure REST API Specifications](https://github.com/Azure/azure-rest-api-specs).

## Status

🚨🚨🚨**WARNING**: This project is currently under very active development.🚨🚨🚨

This projects' crates have yet to released to crates.io so in order to use them you will need to specify them as git dependencies. You should be aware that large, breaking changes can happen at any time, and thus it's not yet recommended to use these crates in any serious capacity yet.

Additionally, this project is the logical successor to the previous Azure SDK crates found under [github.com/MindFlavor/AzureSDKForRust](https://github.com/MindFlavor/AzureSDKForRust). The crates have been renamed, so those older crates should be considered fully deprecated.

## Project Structure

Each supported Azure service is its own separate crate. If a particular service provides logically separate sub-services (e.g., Azure Storage offers blob, queue, and table storage), these are exposed as cargo features of the service's crate.

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
