## <!-- cspell:ignore tspconfig testresources prereqs binstall quickstarts  apiview dpcodegen -->

---

agent: "agent"
description: "Guide contributors through generating a Rust SDK crate from a TypeSpec definition"

---

# TypeSpec ➜ Rust SDK Quickstart

Use this prompt when someone needs an end-to-end recipe for producing a new Rust SDK package from a TypeSpec service description. Capture concrete service identifiers before you start:

-   ${input:serviceDisplayName:"Contoso Widget Manager"}
-   ${input:serviceDirectory:"widgetmanager"}
-   ${input:crateName:"azure_widgetmanager"}
-   ${input:specRepoRef:"Azure/azure-rest-api-specs@<commit>"}
-   ${input:tspConfigUrl:"https://github.com/Azure/azure-rest-api-specs/.../tspconfig.yaml"}

Note that the crate name, service directory and display name should be in the `tspconfig.yaml` file so you do not need to ask for them separately if you have the URL to the `tspconfig.yaml` file.

## 1. Before you begin

1. Confirm the TypeSpec definitions and `tspconfig.yaml` have landed in the `main` branch of `${input:specRepoRef}`. If not, route the user to finish the REST spec workflow first.
1. Prerequisites (call them out explicitly):

    - Node.js 20+. Node.js can be installed at [nodejs.org](https://nodejs.org/).
    - Rust toolchain from `rustup` matching the repo’s `rust-toolchain.toml` (currently 1.85).
    - `tsp-client` dependencies installed globally via `npm install -g @azure-tools/typespec-client-generator-cli`

1. Confirm that the service and crate naming (namespace) has already been approved by the Azure SDK Architecture Board.

## 2. Lay out the SDK workspace

1. Fork `azure-sdk-for-rust`, and clone it locally.
2. Before you run `tsp-client init`, open the repo-root `Cargo.toml` and add `sdk/${input:serviceDirectory}/${input:crateName}` to the `[workspace]` `members` list so the new crate participates in builds immediately.

## 3. Prepare your TypeSpec inputs

1. Service definitions normally live under `specification/${input:serviceDirectory}` in `azure-rest-api-specs`. Verify they contain:

    - `main.tsp` (the service contract)
    - `client.tsp` (customizations using `@azure-tools/typespec-client-generator-core` with `using Azure.ClientGenerator.Core;`)
    - `tspconfig.yaml` configured with the Rust emitter, for example:

    ```yaml
    options:
        "@azure-tools/typespec-rust":
            emitter-output-dir: "{project-root}/sdk/${input:serviceDirectory}/${input:crateName}"
            crate-name: "${input:crateName}"
            crate-version: "0.1.0"
    ```

1. Highlight that `client.tsp` is where they apply decorators such as `@client`, `@operationGroup`, and `@@clientName`, mirroring the customization table from the knowledge article.
1. Remind the user that generated SDK code must _not_ be edited manually—customizations belong in `client.tsp`.

## 4. Generate the Rust crate with `tsp-client`

Walk them through both the Azure DevOps pipeline flow and the local init/update commands:

1. **Initial generation on your machine**

    ```pwsh
    tsp-client init -c ${input:tspConfigUrl}
    ```

    - Explain that `init` scaffolds `sdk/${input:serviceDirectory}/${input:crateName}`, downloads the referenced `tspconfig.yaml`, and emits Rust code into the crate. Ensure the config URL includes a specific commit SHA for reproducibility.
    - If the spec lives locally instead of via URL, use `-c ../../specification/.../tspconfig.yaml`.
    - Mention that contributors can install `tsp-client` globally (`npm install -g @azure-tools/typespec-client-generator-cli`) if they prefer direct CLI invocation.

1. **Regeneration**

    - Update `tsp-location.yaml` with the new commit SHA.
    - From `sdk/${input:serviceDirectory}/${input:crateName}`, run:

    ```pwsh
    tsp-client update
    ```

    - Capture any generator diff in a standalone commit for easier review.
    - When iterating rapidly, use `tsp-client update --save-inputs` to keep temporary TypeSpec files for local tweaks, and delete stale `generated/` folders if models were renamed so obsolete code doesn’t linger.

1. If the user needs to pin a different emitter version, point them to `eng/emitter-package.json` + lockfile, mirroring the warnings from the other quickstarts.

## 5. Post-generation checklist

Provide a concise list the agent must cover every time:

1. **Build, format, and review docs**

```pwsh
cargo fmt -p ${input:crateName}
cargo clippy -p ${input:crateName} --all-features -- -D warnings
cargo test -p ${input:crateName}
cargo doc --no-deps -p ${input:crateName}
```

-   Inspect the generated documentation (`target/doc/${input:crateName}/index.html`).

2. **Docs, samples, and crate metadata files**

-   Ensure `README.md` follows the service template (auth snippet, example client call, TypeSpec provenance section) and convert placeholder links to absolute URLs. If `tsp-client` did not create the README, author it manually using the latest key vault crates as a reference.
-   Author `CHANGELOG.md` in the crate root with an `Unreleased` section and at least one "Features Added" bullet calling out the initial preview. Reference `.github/instructions/changelog.instructions.md` for formatting.
-   Add `sdk/${input:serviceDirectory}/ci.yml` so the continuous-integration pipeline builds, tests, and releases the new crate. Mirror the existing key vault/client service pipeline structure when adding the artifact entry.
-   Add `examples/` showcasing at least one happy-path scenario using the generated client, keeping code snippets synchronized with README content.

3. **Tests**

-   Implement `#[cfg(test)]` unit tests in the crate (for example, building minimal `tests` modules in each public module) to cover serialization helpers, request builder defaults, and any handwritten helpers. Use the guidance from `.github/copilot-instructions.md` (tests at bottom of module, importing from `super`).
-   Where the service supports recorded integration tests, add skeletons under `tests/` with `#[recorded::test]` attributes and follow `CONTRIBUTING.md` for provisioning resources.
    -   Make sure to implement functional tests to verify the functionality of the generated crate.
    -   When creating functional tests, ensure that a correct test-resources.bicep file is created to create the required test resources for the service client.
    -   Run the new-testresources.ps1 powershell script to create the required test resources for functional tests before running the functional tests initially.
        -   Note that when creating tests, it's best to use the tests in the azure_security_keyvault_secrets package since they are most likely to be up-to-date with guidelines.

4. **Examples**
    - Implement examples for the newly generated service client following the model used for the azure_security_keyvault_secrets examples.
        - Note that when looking for examples, it's best to use the examples in the azure_security_keyvault_secrets package since it is most likely to be up-to-date with guidelines.
5. **Metadata**

-   Update `sdk/${input:serviceDirectory}/ci.yml` if a new pipeline is required.
-   Add an entry to `CHANGELOG.md` under `Unreleased` describing the new preview package.

6. **Validation & local install**

-   If you need to consume the crate locally, run `cargo install --path sdk/${input:serviceDirectory}/${input:crateName}` into a temporary prefix or `cargo publish --dry-run` to validate packaging.
-   Verify no files under `generated/` were manually edited.

7. **Feature checklist**

-   Authentication & credentials: document the supported credential flows and validate them end-to-end.
-   Long-running operations: ensure pollers follow Azure LRO guidance and that samples/tests cover them.
-   File upload/download: verify binary payload helpers and mention required content types/chunking behavior.
-   Samples/tests: add runnable examples and live/playback-style tests where feasible.

## 6. Communication & review reminders

-   Link to `https://aka.ms/azsdk/dpcodegen` for architecture-board policy.
-   Call out that any new SDK requires board approval and onboarding into the release pipeline.
-   Loop in APIView reviewers (`apiview.dev`) once the crate builds.
-   Highlight release readiness: ensure CHANGELOGs, CI definitions, and release scripts are updated before requesting the "increment versions" automation or publishing the crate.

## Deliverable template

When you finish, deliver a short status summary that includes:

-   Confirmation that `tsp-client init/update` ran without errors.
-   Location of the generated crate (`sdk/${input:serviceDirectory}/${input:crateName}`).
-   Build/test/doc command results.
-   Follow-ups (e.g., pending API review, missing recordings, release checklist items).
