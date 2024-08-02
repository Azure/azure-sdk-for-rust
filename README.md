# Azure SDK for Rust

This repository is for the active development of the Azure SDK for Rust. For consumers of the SDK you can follow the links below to visit the documentation you are interested in

* [Overview of Azure SDK for Rust](https://docs.microsoft.com/azure/developer/rust/)
* [SDK Reference](https://pkg.go.dev/github.com/Azure/azure-sdk-for-go/sdk)
* [Code Samples for Azure SDK for Rust](https://github.com/azure-samples/azure-sdk-for-rust-samples)
* [Azure REST API Docs](https://docs.microsoft.com/rest/api/)
* [General Azure Docs](https://docs.microsoft.com/azure)
* [Share your feedback to our Azure SDK](https://www.surveymonkey.com/r/FWPGFGG)

## Getting Started

To get started with a crate, see the README.md file located in the crate's project folder.  You can find these crate folders grouped by service in the `/sdk` directory.

## Crates available

Each service can have both 'client' and 'management' crates. 'Client' crates are used to consume the service, whereas 'management' crates are used to configure and manage the service.

[All Azure SDK for Rust crates](https://crates.io/teams/github:azure:azure-sdk-publish-rust) are published on crates.io.

### Client modules

Our client crates follow the [Azure Rust SDK guidelines](https://azure.github.io/azure-sdk/rust_introduction.html). These crates allow you to use, consume, and interact with existing resources, for example, uploading a blob. They also share a number of core functionalities including retries, logging, transport protocols, authentication protocols, etc. that can be found in the [azure_identity](https://crates.io/crates/azure_identity) module.

You can find the most up-to-date list of new modules on our [latest page](https://azure.github.io/azure-sdk/releases/latest/index.html#rust).

> [!NOTE]
> If you need to ensure your code is ready for production use one of the stable, non-beta modules.

### Services
Azure service crates generated from [Azure REST API Specifications](https://github.com/Azure/azure-rest-api-specs) are available in [services](services).

## Status

🚨 WARNING 🚨: This project is under active development. Be aware that large breaking changes will happen before 1.0 is reached.

### Historical releases

This project is the successor to the `azure_sdk*` crates from [MindFlavor/AzureSDKForRust](https://github.com/MindFlavor/AzureSDKForRust). The crates have been renamed, so those older crates should be considered fully deprecated. See [history](HISTORY.md) for more details.

## Reporting security issues and security bugs

Security issues and bugs should be reported privately, via email, to the Microsoft Security Response Center (MSRC) <secure@microsoft.com>. You should receive a response within 24 hours. If for some reason you do not, please follow up via email to ensure we received your original message. Further information, including the MSRC PGP key, can be found in the [Security TechCenter](https://www.microsoft.com/msrc/faqs-report-an-issue).

## Need help?

* File an issue via [Github Issues](https://github.com/Azure/azure-sdk-for-rust/issues)
* Check [previous questions](https://stackoverflow.com/questions/tagged/azure+rust) or ask new ones on StackOverflow using `azure` and `rust` tags.

## Community

## Contribute

See [CONTRIBUTING.md](https://github.com/Azure/azure-sdk-for-go/blob/main/CONTRIBUTING.md).

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/). For more information see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.

## Trademarks

This project may contain trademarks or logos for projects, products, or services. Authorized use of Microsoft trademarks or logos is subject to and must follow [Microsoft's Trademark & Brand Guidelines](https://www.microsoft.com/legal/intellectualproperty/trademarks/usage/general). Use of Microsoft trademarks or logos in modified versions of this project must not cause confusion or imply Microsoft sponsorship. Any use of third-party trademarks or logos are subject to those third-party's policies.
