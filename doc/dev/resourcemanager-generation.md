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

- **Credential surface** – Accept `Arc<dyn azure_core::credentials::TokenCredential>` so callers can use `azure_identity` or another credential implementation. Preserve the subscription arguments defined by TypeSpec.
- **Core dependency** – Enable `azure_core`'s `resourcemanager` feature. Require a core version containing the shared cloud configuration and polling APIs used by the generated code; preserve the usual transport feature forwarding.
- **Integration tests** – Provision test resources with `eng/common/TestResources/New-TestResources.ps1 -ServiceDirectory <service>` and record with `#[recorded::test]`. Management plane tests frequently require role assignments, so verify the Bicep template grants the test application proper `Microsoft.Authorization/roleAssignments` permissions.

After these tweaks, run the usual validation commands (`cargo fmt`, `cargo clippy`, `cargo test`) as instructed in the base guide. When long-running operations are generated, add a `cargo check -p <crate> --all-features` run to ensure every poller compiles.

### Cloud configuration and shared audience

All `azure_resourcemanager_*` crates use
`azure_core::cloud::resourcemanager::Audience` for custom audience registration.
The marker and `CloudConfigurationExt` trait are gated by `azure_core`'s
`resourcemanager` feature. Generated and hand-written constructors must import
`azure_core::cloud::resourcemanager::CloudConfigurationExt as _` for endpoint,
audience, and scope resolution without audience type parameters. Core uses the
shared marker internally; do not generate or re-export a service-specific
`Audience`. Audience entries are keyed by Rust
type identity: one custom Resource Manager audience registration must work with
every Resource Manager crate. Data-plane crates retain their service-specific
audience markers.

Constructors read `options.client_options.cloud`, borrowing the configured cloud
or the default without cloning it or allocating a default `Arc`:

```rust ignore
use azure_core::cloud::resourcemanager::CloudConfigurationExt as _;

let options = options.unwrap_or_default();
let cloud = options.client_options.cloud.as_deref().unwrap_or_default();
let endpoint = cloud
    .endpoint()
    .ok_or_else(|| Error::new(ErrorKind::Other, "missing Resource Manager endpoint"))?;
let endpoint = Url::parse(endpoint)?;
let scope = cloud.default_scope()?.into_owned();
```

Pass the scope to `BearerTokenAuthorizationPolicy`, then move the original
`options.client_options` into `Pipeline::new`. Propagate missing configuration
and URL errors. Public cloud is the default; Public, Government, and China
endpoint and scope mappings belong to `azure_core`, not generated service code.
Do not generate cloud matches, endpoint tables, or manual scope concatenation
or normalization.

For custom clouds, keep these values distinct:

- `CustomConfiguration::authority_host` configures the credential's identity authority.
- `CustomConfiguration::endpoint` supplies the Resource Manager service URL.
- `Audiences::with::<Audience>(...)` registers the authentication audience.
  The constructor calls `cloud.default_scope()` to derive the token scope.

Configuration of `CloudConfiguration` and `CustomConfiguration` is unchanged.
Caller examples retain the shared `Audience` import for registration but do not
need `CloudConfigurationExt`; only constructor implementations need the trait.

The endpoint and audience need not be the same URL; never infer one from the
other or use the login authority as the service endpoint. A custom cloud must
supply both endpoint and audience. Scope formatting belongs to core; its current
custom-cloud implementation appends `/.default` verbatim, so the emitter must
not introduce its own trailing-slash trimming.

Keep the normal default-cloud client example first in each crate README, followed
by a custom-cloud example. Register the audience with the shared core marker and
pass the same cloud to the service client and a cloud-aware credential, such as
`ClientSecretCredential`. `DeveloperToolsCredential` instead uses the developer
tools' cloud settings and has no cloud option. Load secrets from a secure source.

Constructor tests should cover omitted and explicit Public cloud, Government,
China, distinct custom endpoint and audience URLs, missing custom endpoint or
audience, and invalid endpoint URLs. Emitter fixtures should verify delegation to
core, the constructor's extension-trait import, and the absence of a generated audience marker, not duplicate core's cloud
mapping tests. Register only the shared marker in custom-cloud tests so a
mistaken service-specific marker fails lookup.

### Long-running operation generation

Generate calls to `azure_core::http::poller::resourcemanager::new_poller` and
method options using that module's `PollerOptions`. Continue returning the
existing `Poller<OperationStatus>`; do not introduce a service-specific poller or
generate a competing polling state machine.

Pass final-state metadata as `Option<FinalStateVia<_>>` to the helper. Omitted
metadata stays `None`, allowing method- and response-based runtime rules; it is
not an implicit Location selector. For inline Operation-Location results,
generate the extraction callback from result-path metadata, preserving response
status, headers, and original result JSON bytes. Empty final results use
`StatusMonitor::Output = ()` and `NoFormat`; resource results retain their format.

Delegate polling protocols, API-version handling, retry timing, and final GET
selection to core. Preserve lazy initial execution and caller context. The core
helper creates body-free continuation GETs and retains the initial `Accept`
header for polling and final requests. Test emitted metadata and runtime wiring
rather than reimplementing these protocol rules in emitter fixtures.

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
