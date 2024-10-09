# Azure SDK for Rust

This repository is for the active development of the Azure SDK for Rust. For consumers of the SDK we recommend visiting [Docs.rs](https://docs.rs/) and looking up the docs for any of libraries in the SDK.

## Getting started

To get started with a library, see the `README.md` file located in the library's project folder. You can find these library project folders grouped by service in the `/sdk` directory.

For full file code examples, check out the `/examples` directory in any library project folder.

## Crates

[All Azure SDK for Rust crates](https://crates.io/teams/github:azure:azure-sdk-publish-rust) are published on crates.io.

## Status

🚨 WARNING 🚨: This project is under active development. Be aware that large breaking changes may happen before 1.0 is reached.

This project is the successor to the `azure_sdk*` crates from [MindFlavor/AzureSDKForRust](https://github.com/MindFlavor/AzureSDKForRust). The crates have been renamed, so those older crates should be considered fully deprecated. See [history](HISTORY.md) for more details.

## Project Structure

Each supported Azure service is its own separate crate.

Building each crate should be as straight forward as `cargo build`, but check each crate's README for more specific information.

### Mock testing framework

This library comes with a testing framework that executes against prerecorded sessions to quickly validate code changes without incurring in Azure costs. You can read more about it in the [Mock testing framework's README](https://github.com/Azure/azure-sdk-for-rust/tree/feature/track2/doc/mock_transport.md).

## Need help?

- For reference documentation visit [docs.rs](https://docs.rs/).
- For examples, go to the `/examples` sub-directory in any library's project folder.
  - Ex:`/sdk/identity/azure_identity/examples`
- Have a question, or find a bug? File an issue via [GitHub Issues](https://github.com/Azure/azure-sdk-for-rust/issues/new/choose).
- Check [previous questions](https://stackoverflow.com/questions/tagged/azure+rust) or ask new ones on StackOverflow using the `azure` and `rust` tags.

### Reporting security issues and security bugs

Security issues and bugs should be reported privately, via email, to the Microsoft Security Response Center (MSRC) <secure@microsoft.com>. You should receive a response within 24 hours. If for some reason you do not, please follow up via email to ensure we received your original message. Further information, including the MSRC PGP key, can be found in the [Security TechCenter](https://www.microsoft.com/msrc/faqs-report-an-issue).

## We want your thoughts!

### Feature Requests

What features are important to you? You can let us know by looking at our open [feature requests](https://github.com/Azure/azure-sdk-for-rust/issues?q=is%3Aopen+is%3Aissue+label%3Afeature-request+sort%3Areactions-%2B1-desc) and sharing your thoughts by giving the issue a thumbs up or thumbs down. (Note the list is sorted by the number of thumbs up in descending order.)

### Design Discussions

We would love to incorporate the community's input into our library design process. Here's a list of [design discussions](https://github.com/Azure/azure-sdk-for-rust/labels/design-discussion) that we're currently having. Participate in the discussions by leaving your comments in the issue!

## Contributing

For details on contributing to this repository, see the [contributing guide](https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md).

This project welcomes contributions and suggestions.  Most contributions require you to agree to a
Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us
the rights to use your contribution. For details, visit <https://cla.opensource.microsoft.com>.

When you submit a pull request, a CLA bot will automatically determine whether you need to provide
a CLA and decorate the PR appropriately (e.g., status check, comment). Simply follow the instructions
provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/).
For more information see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or
contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.
