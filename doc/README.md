# Azure SDK for Rust Documentation

This directory curates engineering guidance, design notes, and process documentation for contributors to the Azure SDK for Rust. Use these pages to understand repository conventions, service onboarding workflows, and supporting infrastructure.

## Documentation Map

-   [Development Guides](https://github.com/Azure/azure-sdk-for-rust/blob/main/doc/dev/README.md)
    -   Management-plane generation walkthroughs and TypeSpec automation tips for building `azure_mgmt_*` crates.
    -   Step-by-step instructions for scaffolding new clients, aligning with the Azure SDK design guidelines.
-   [Deprecation Process](https://github.com/Azure/azure-sdk-for-rust/blob/main/doc/deprecation-process.md)
    -   Policies for deprecating APIs, handling breaking changes, and communicating timeline expectations.
-   [Distributed Tracing](https://github.com/Azure/azure-sdk-for-rust/blob/main/doc/distributed-tracing-for-rust-service-clients.md)
    -   How Rust service clients integrate with OpenTelemetry and instrumentation best practices.
-   [Git Commit Instructions](https://github.com/Azure/azure-sdk-for-rust/blob/main/doc/git-commit-instructions.md)
    -   Required formatting, metadata, and review expectations for commits in this repository.
-   [New TypeSpec-Based Client Guide](https://github.com/Azure/azure-sdk-for-rust/blob/main/doc/new-typespec-based-client.md)
    -   Canonical workflow for producing Rust clients from TypeSpec, covering emitter configuration and regeneration steps.

## Supporting Resources

-   [Repository Overview](https://github.com/Azure/azure-sdk-for-rust/blob/main/README.md)
-   [Contribution Guidelines](https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md)
-   [Azure SDK Design Guidelines for Rust](https://azure.github.io/azure-sdk/rust_introduction.html)

If you add new documentation under `doc/`, link it here and keep cross-references absolute so readers can browse the content both locally and on GitHub.
