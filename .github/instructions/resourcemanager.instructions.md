---
applyTo: "sdk/*/azure_resourcemanager_*/"
---

# Azure Resource Manager Instructions

- Initialize new Azure Resource Manager (ARM) projects using the [generation guide](https://github.com/Azure/azure-sdk-for-rust/blob/main/doc/dev/resourcemanager-generation.md).
- Keep the `azure_resourcemanager_<service>` crate name and TypeSpec client names with the usual `Client` suffix; do not invent management-specific client renames.
- Enable `azure_core`'s `resource_manager` feature. For every Resource Manager crate, use `azure_core::cloud::resource_manager::Audience` for registration, lookup, and scopes; never emit or re-export a per-crate `Audience`. Data-plane markers remain service-specific.
- In constructors, import `azure_core::cloud::resource_manager::CloudConfigurationExt as _`, borrow `options.client_options.cloud.as_deref().unwrap_or_default()`, and call `endpoint()` and `default_scope()` without audience type parameters. Parse the endpoint and propagate errors. Preserve client options for the pipeline. Do not emit cloud mappings, scope concatenation, or normalization.
- Keep custom identity authority, service endpoint, and audience independent. Require `CustomConfiguration::endpoint` and an audience registered with the shared marker; configure the credential's cloud separately.
- Generate long-running operations with `azure_core::http::poller::resource_manager::{new_poller, PollerOptions, FinalStateVia}` and the existing `Poller`. Preserve omitted final-state metadata as `None`; delegate protocol behavior to core. See the [polling guidance](https://github.com/Azure/azure-sdk-for-rust/blob/main/doc/dev/resourcemanager-generation.md#long-running-operation-generation) for experimental status, result extraction, API versions, and empty-output formats.
- Use "Resource Manager" in public documentation and `resource_manager` in module, field, function, method, and feature names. Prefer nested `resource_manager` modules over `ResourceManager` type or variant prefixes when disambiguating; retain short names within those modules and the `azure_resourcemanager_*` crate prefix.
- Use the `document-crates` skill before planning or editing `README.md` or `CONTRIBUTING.md`; keep contributor workflows in the latter.
- Keep the normal default-cloud README client example first, followed by a custom-cloud example with authority, endpoint, and shared audience registration. Use a cloud-aware credential for the latter; `DeveloperToolsCredential` has no cloud option.
- Integration tests under `tests/` in the crate root directory should be generated for client type exported from the crate root e.g., `azure_resourcemanager_keyvault::KeyVaultClient` that show:
  - Creating a new resource.
  - Listing existing resources available in the current subscription.
  - Updating the properties of an existing resource.
  - Deleting a resource.
- A `README.md` file in the crate root directory should follow the same structure as `sdk/keyvault/azure_security_keyvault_secrets/README.md` that includes:
  - A brief overview of which Azure Resource Provider (RP) it controls.
  - How to install the crate.
  - How to authenticate the client library using a `DeveloperToolsCredential`.
  - Examples of creating, listing, updating, and deleting a resource controlled by the RP.
    - A listing of links to those examples should be in the parent heading.
    - Code such as `async fn main()` can be elided by using the `include-file` crate and `rust ignore <unique-section-name>` code fences where `unique-section-name` can be imported from within a `tests/readme.rs` file e.g., `sdk/keyvault/azure_security_keyvault_secrets/tests/readme.rs`.
    - Do not include any references to the `recording` object; instead, create local variables for recorded properties in `tests/readme.rs` with appropriate names that are used by examples in the `README.md` e.g.,

        ```rust
        let resource_group = recording.var("AZURE_RESOURCE_GROUP", None);
        ```

  - Troubleshooting information.
  - Contributing information.
  - Run markdownlint on the `README.md` and fix all issues.
