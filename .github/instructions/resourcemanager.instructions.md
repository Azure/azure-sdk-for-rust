---
applyTo: "sdk/*/azure_resourcemanager_*/"
---

# Azure Resource Manager Instructions

- Initialize new Azure Resource Manager (ARM) projects using instructions in `doc/dev/resourcemanager-generation.md`.
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
