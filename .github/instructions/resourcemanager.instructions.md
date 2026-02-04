---
applyTo: "sdk/*/azure_resourcemanager_*/"
---

# Azure Resource Manager Instructions

- Initialize new Azure Resource Manager (ARM) projects using instructions in `doc/dev/resourcemanager-generation.md`.
- Integration tests under `tests/` in the crate root directory should be generated for all clients exported from the `clients` module e.g., `azure_resourcemanager_keyvault::clients::KeyVaultClient` that show:
    - Creating a new resource.
    - Listing existing resources available in the resource group created for the tests.
    - Updating the properties of an existing resource.
    - Deleting a resource.
- A `README.md` file in the crate root directory should follow the same structure as `sdk/keyvault/azure_security_keyvault_secrets/README.md` that includes:
    - A brief overview of which Azure Resource Provider (RP) it controls.
    - How to install the crate.
    - How to authenticate the client library using a `DeveloperToolsCredential`.
    - Examples of creating, listing, updating, and deleting a resource controlled by the RP using clients exported only from the crate root e.g., `azure_resourcemanager_keyvault::KeyVaultClient` that show:
        - A listing of links to those examples should be in the parent heading.
        - Code such as `async fn main()` can be elided by using the `include-file` crate and `rust ignore <unique-section-name>` code fences where `unique-section-name` can be imported from within a `tests/readme.rs` file e.g., `sdk/keyvault/azure_security_keyvault_secrets/tests/readme.rs`.
        - Do not include any references to the `recording` object; instead, create local variables for recorded properties in `tests/readme.rs` with appropriate names that are used by examples in the `README.md` e.g.,

            ```rust
            let resource_group = recording.var("KEYVAULT_RESOURCE_GROUP", None);
            let tenant_id = recording.var("KEYVAULT_TENANT_ID", None);
            ```

            `KEYVAULT` is replaced with the uppercase `<service>` name in the `sdk/<service>` directory.

        - Generate a random name for the individual resource to create, update, and delete instead of getting it from a variable e.g.,

            ```rust
            let resource_name = recording.random_string::<16>(Some("t"));
            ```

            12 is the maximum resource name length and "t" is an optional service name prefix since most cannot start with numbers.
            These requirements vary by service.

    - Troubleshooting information.
    - Contributing information.
    - Run markdownlint on the `README.md` and fix all issues.

- `Cargo.toml` should have a `dev-dependency` on `azure_core_test` to add the `tracing` feature:

    ```toml
    azure_core_test = { workspace = true, features = [
        "tracing",
    ] }
    ```
