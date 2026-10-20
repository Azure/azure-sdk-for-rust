<!-- cspell: ignore tspconfig resourcemanager -->

# Generating Management Plane Crates from TypeSpec

This document complements [Creating a TypeSpec-Based Rust SDK Client](https://github.com/Azure/azure-sdk-for-rust/blob/main/doc/dev/typespec-based-generation.md). Read that guide first for the canonical, step-by-step walkthrough; use this page to layer on the management plane specifics that keep our Resource Manager crates consistent with the rest of the Azure SDK.

## When to reach for this guide

You are in the right place if all of the following are true:

- Your TypeSpec project targets Resource Manager APIs (`import "@azure-tools/typespec-azure-resource-manager";`).
- You plan to ship the resulting crate under the `azure_resourcemanager_*` naming pattern in this repository.
- You already understand the base TypeSpec → Rust workflow described in [typespec-based-generation.md](https://github.com/Azure/azure-sdk-for-rust/blob/main/doc/dev/typespec-based-generation.md) and only need the management-plane adjustments.

## Management-plane prerequisites

In addition to the standard tooling covered in the base guide, keep these points in mind when preparing the TypeSpec inputs:

- **Emitter options** – Add Rust options to `tspconfig.yaml` alongside the other languages' configuration. Preserve the existing `emit` list; `tsp-client` selects the Rust emitter explicitly. A minimal options block looks like:

    ```yaml
    options:
        "@azure-tools/typespec-rust":
            emitter-output-dir: "{output-dir}/{service-dir}/{crate-name}"
            crate-name: "azure_resourcemanager_<service>"
            crate-version: "0.1.0"
        "@azure-tools/typespec-azure-core":
            azure-resource-provider: "Microsoft.<ProviderName>"
    ```

- **Naming** – Follow the `azure_resourcemanager_<service>` crate pattern and mirror the service folder layout used by other languages (`sdk/<service>/<crate-name>`). For example, the Key Vault management library is `sdk/keyvault/azure_resourcemanager_keyvault`, distinct from the data-plane `azure_security_keyvault_*` crates.
- **Spec changes** – Capture the `azure-rest-api-specs` commit SHA (or PR branch commit) that contains the TypeSpec updates; you will reference it from `tsp-location.yaml`.

### Client names

Use the client names supplied by TypeSpec and its applicable client customizations,
with the normal Rust emitter's `Client` suffix. Resource Manager clients do not
need a separate naming scheme. For example, `KeyVault` becomes `KeyVaultClient`
and `KeyVaultVaults` becomes `KeyVaultVaultsClient`. Do not shorten the latter to
`VaultClient` or add `Management` merely because another language uses that name.

Review existing `client.tsp` language scopes before adding Rust. Shared naming
customizations may be useful, but compatibility overrides for another language
are not automatically appropriate. Apply related renames together when needed
to avoid collisions. Client or enum member renames change SDK identifiers, not
serialized wire values.

## Repository setup checklist

The base client guide covers initialization in detail. For management plane, double-check these repo-specific tasks once your service directory exists:

- Add `sdk/<service>/<crate-name>` to the workspace members in the root `Cargo.toml`.
- Drop a `ci.yml` into `sdk/<service>/` if the service folder is new so the CI pipeline can pick the crate up. Copy an existing management plane `ci.yml` as the starting point.
- Create `tsp-location.yaml` in the crate folder with `repo`, `commit`, and `directory` keys pointing at the Resource Manager TypeSpec project.

## Running `tsp-client`

Use the same commands outlined in the base guide—`tsp-client init` for brand-new crates, `tsp-client update` for refreshes. The only management-plane twist is path selection:

```powershell
# Example update call
tsp-client update https://github.com/Azure/azure-rest-api-specs/blob/<commit>/specification/<service>/<project>/tspconfig.yaml

# Local iteration
tsp-client update --local-spec-repo "<your-repo-path>/Azure/azure-rest-api-specs/specification/<service>/<project>/tspconfig.yaml"

```

Generation writes into `src/generated/`; keep the directory read-only and place any ergonomic layers or custom entry points next to `lib.rs`, exactly as described in the [guidelines](https://azure.github.io/azure-sdk/rust_implementation.html#rust-client-convenience).

## Management-plane finishing work

Most polish items match the data-plane flow, but a few deserve special emphasis for Resource Manager crates:

- **Cargo metadata** – Ensure `Cargo.toml` advertises the crate as “Azure Resource Manager” in `description` and includes:

  ```toml
  [lints]
  workspace = true
  ```

- **Credential surface** – Make sure the public constructors accept `azure_identity` credentials and expose subscription-scoped builders, matching existing management plane crates.
- **Integration tests** – Provision test resources with `eng/common/TestResources/New-TestResources.ps1 -ServiceDirectory <service>` and record with `#[recorded::test]`. Management plane tests frequently require role assignments, so verify the Bicep template grants the test application proper `Microsoft.Authorization/roleAssignments` permissions.

After these tweaks, run the usual validation commands (`cargo fmt`, `cargo clippy`, `cargo test`) as instructed in the base guide. When long-running operations are generated, add a `cargo check -p <crate> --all-features` run to ensure every poller compiles.

### Enum coverage and test resources

Follow the base guide's [enum wire-contract testing](typespec-based-generation.md#enum-wire-contract-tests)
for both offline serialization checks and recorded service responses. Group live
scenarios by related operations, not by enum declaration: RSA keys, EC curves,
rotation policy, secrets, network rules, access policies, and vault recovery can
be separate subjects. Reuse one test-owned vault for related cases, such as
multiple EC curves, rather than creating a vault per enum value.

Keep standard vault and managed HSM scenarios in separate test files. Document
resource counts, provisioning cycles, cleanup, and any dedicated or activated
fixtures in the crate's `CONTRIBUTING.md`. Leave scenarios requiring unavailable
provisioning explicitly ignored with a reason; an ignored scenario is not live
coverage.

Use each operation's actual response contract. For example, Key Vault's
unversioned key GET returns its rotation policy, while a versioned key GET can
omit it. Vault responses can omit network ACLs when both settings are defaults;
exercise non-default combinations to verify each enum spelling rather than
counting an absent field as coverage.

## Regeneration cadence

When new API versions land:

1. Update the TypeSpec inputs in `azure-rest-api-specs` and grab the merge commit.
2. Refresh `tsp-location.yaml` with that commit.
3. At the location of `tsp-location.yaml`, Re-run `tsp-client update` and inspect changes outside `generated/` for breaking changes, especially in manual subscription helpers or feature-gated modules.
4. Update tests, recordings, and documentation to track any surface changes.

As always, fix issues in TypeSpec or the emitter rather than editing generated Rust by hand.

## Troubleshooting

| Symptom                             | Quick check                                                                                                                               |
| ----------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `tsp-client` cannot find templates  | Confirm the Rust emitter is declared in `tspconfig.yaml` and you are on the latest `@azure-tools/typespec-rust` npm package.              |
| Missing `subscription_id` arguments | Ensure the TypeSpec decorators include `@armResource` or other ARM helpers so the emitter wires subscription scoping.                     |
| Access denied during recordings     | Re-run the test resource script and verify your `AZURE_TENANT_ID`, `AZURE_CLIENT_ID`, and `AZURE_CLIENT_SECRET` exports before recording. |

## Related reading

- [Creating a TypeSpec-Based Rust SDK Client](https://github.com/Azure/azure-sdk-for-rust/blob/main/doc/dev/typespec-based-generation.md)
- [Azure SDK Design Guidelines for Rust](https://azure.github.io/azure-sdk/rust_introduction.html)
- [TypeSpec Client Generator CLI](https://github.com/Azure/azure-sdk-tools/tree/main/tools/tsp-client)
- [Azure REST API Specs Repository](https://github.com/Azure/azure-rest-api-specs)
