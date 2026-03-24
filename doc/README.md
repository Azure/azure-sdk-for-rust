<!-- cspell: ignore tspconfig resourcemanager -->

# Azure SDK for Rust Documentation

This directory curates engineering guidance, design notes, and process documentation for contributors to the Azure SDK for Rust. Use these pages to understand repository conventions, service onboarding workflows, and supporting infrastructure.

## Documentation Map

- [Consumer Guides](https://github.com/Azure/azure-sdk-for-rust/blob/main/doc/): contains documentation for consumers of SDKs
- [Development Guides](https://github.com/Azure/azure-sdk-for-rust/blob/main/doc/dev/README.md): contains advanced documentation for developers of SDKs (not consumers of SDKs)

### Consumer Guides

- [Distributed Tracing](https://github.com/Azure/azure-sdk-for-rust/blob/main/doc/distributed-tracing-for-rust-service-clients.md)
  - How Rust service clients integrate with OpenTelemetry and instrumentation best practices.
- [Git Commit Instructions](https://github.com/Azure/azure-sdk-for-rust/blob/main/.github/instructions/git-commit.instructions.md)
  - Required formatting, metadata, and review expectations for commits in this repository.

## Supporting Resources

- [Repository Overview](https://github.com/Azure/azure-sdk-for-rust/blob/main/README.md)
- [Contribution Guidelines](https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md)
- [Azure SDK Design Guidelines for Rust](https://azure.github.io/azure-sdk/rust_introduction.html)

If you add new documentation under `doc/`, link it here and keep cross-references absolute so readers can browse the content both locally and on GitHub.
