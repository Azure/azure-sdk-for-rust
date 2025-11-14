<!-- cspell:ignore tspconfig testresources prereqs binstall quickstarts nextest xtask apiview dpcodegen -->
````prompt
---
mode: "agent"
description: "Guide contributors through generating a Rust SDK crate from a TypeSpec definition"
---

# TypeSpec ➜ Rust SDK Quickstart

Use this prompt when someone needs an end-to-end recipe for producing a new Rust SDK package from a TypeSpec service description. Capture concrete service identifiers before you start:

- ${input:serviceDisplayName:"Contoso Widget Manager"}
- ${input:serviceDirectory:"widgetmanager"}
- ${input:crateName:"azure_widgetmanager"}
- ${input:specRepoRef:"Azure/azure-rest-api-specs@<commit>"}
- ${input:tspConfigUrl:"https://github.com/Azure/azure-rest-api-specs/.../tspconfig.yaml"}

## 1. Before you begin

1. Confirm the TypeSpec definitions and `tspconfig.yaml` have landed in the `main` branch of `${input:specRepoRef}` (per the Java quickstart). If not, route the user to finish the REST spec workflow first.
2. Prerequisites (call them out explicitly):
	- Node.js 20+, npm, and pnpm (to mirror the JS quickstart prereqs).
	- Rust toolchain from `rustup` matching the repo’s `rust-toolchain.toml` (currently 1.85).
	- `tsp-client` dependencies installed once via `npm ci --prefix eng/common/tsp-client` at the repo root.
	- Optional: `cargo binstall cargo-nextest` if tests will leverage nextest.
3. Mention the `TypeSpec Discussion` and `DPG` Teams channels for help.
4. Confirm that the service and crate naming (namespace) has already been approved by the Azure SDK Architecture Board.

## 2. Lay out the SDK workspace

1. Fork/clone `azure-sdk-for-rust`, then create `sdk/${input:serviceDirectory}/${input:crateName}`. Align `${input:serviceDirectory}` with the REST spec directory (service) and `${input:crateName}` with the approved namespace, mirroring the "Module folder" guidance from the Java quickstart.
2. Inside the new crate:
	- Let `tsp-client init` scaffold the crate’s `Cargo.toml`, README, and sample layout directly into `sdk/${input:serviceDirectory}/${input:crateName}`. After it runs, review the generated metadata (`package`, `authors`, `keywords`, license headers) and adjust as needed.
	- Add `tsp-location.yaml` alongside `Cargo.toml` pointing at `${input:specRepoRef}`:

	  ```yaml
	  directory: specification/${input:serviceDirectory}
	  commit: <commit-sha>
	  repo: Azure/azure-rest-api-specs
	  ```

	- Ensure `Cargo.toml` uses workspace-managed dependencies (`azure_core.workspace = true`, etc.).
3. Before you run `tsp-client init`, open the repo-root `Cargo.toml` and add `sdk/${input:serviceDirectory}/${input:crateName}` to the `[workspace]` `members` list so the new crate participates in builds immediately.
4. Remind the user that generated code must *not* be edited manually—customizations belong in `client.tsp`.

## 3. Prepare your TypeSpec inputs

1. Service definitions normally live under `specification/${input:serviceDirectory}` in `azure-rest-api-specs`. Verify they contain:
	- `main.tsp` (the service contract)
	- `client.tsp` (customizations using `@azure-tools/typespec-client-generator-core` with `using Azure.ClientGenerator.Core;`)
	- `tspconfig.yaml` configured with the Rust emitter, for example (adapted from the Java quickstart sample):

	  ```yaml
	  emit: ["@azure-tools/typespec-rust"]
	  options:
	    "@azure-tools/typespec-rust":
	      emitter-output-dir: "{project-root}/sdk/${input:serviceDirectory}/${input:crateName}"
	      namespace: "${input:crateName}"
	      package-version: "1.0.0-beta.1"
	  ```

2. Highlight that `client.tsp` is where they apply decorators such as `@client`, `@operationGroup`, and `@@clientName`, mirroring the customization table from the knowledge article.
3. If the repo does not yet include `client.tsp`, guide them to create it following `eng/common/knowledge/customizing-client-tsp.md`.

## 4. Generate the Rust crate with `tsp-client`

Walk them through both the Azure DevOps pipeline flow and the local init/update commands (matching the JS and Java guides):

1. **Prefer the Azure SDK Generation Pipeline when available**

	- Just like the Java quickstart, recommend configuring `tspconfig.yaml` directly in `${input:specRepoRef}` and queueing the internal Azure DevOps "SDK Generation Pipeline" (definition ID 7421). Provide the config path, target `api-version`, and commit SHA if the REST spec PR is still open.
	- Capture the generated PR or artifacts from the pipeline and merge them into your working branch before continuing locally.

2. **Initial generation on your machine**

	```pwsh
	$tsp = "eng/common/tsp-client"
	npm --prefix $tsp ci            # first time only
	npm exec --prefix $tsp --no -- tsp-client init -c ${input:tspConfigUrl}
	```

	- Explain that `init` scaffolds `sdk/${input:serviceDirectory}/${input:crateName}`, downloads the referenced `tspconfig.yaml`, and emits Rust code into the crate. As in the Java wiki, ensure the config URL includes a specific commit SHA for reproducibility.
	- If the spec lives locally instead of via URL, use `-c ..\..\specification\...\tspconfig.yaml`.
	- Mention that contributors can install `tsp-client` globally (`npm install -g @azure-tools/typespec-client-generator-cli`) if they prefer direct CLI invocation like the Java workflow.

3. **Regeneration**

	- Update `tsp-location.yaml` with the new commit SHA.
	- From `sdk/${input:serviceDirectory}/${input:crateName}`, run:

	  ```pwsh
	  npm exec --prefix ..\..\..\eng\common\tsp-client --no -- tsp-client update
	  ```

	- Capture any generator diff in a standalone commit for easier review.
	- When iterating rapidly (akin to the "Follow-up" section in the Java quickstart), use `tsp-client update --save-inputs` to keep temporary TypeSpec files for local tweaks, and delete stale `generated/` folders if models were renamed so obsolete code doesn’t linger.

3. If the user needs to pin a different emitter version, point them to `eng/emitter-package.json` + lockfile, mirroring the warnings from the other quickstarts.

## 5. Post-generation checklist

Provide a concise list the agent must cover every time:

1. **Build, format, and review docs**

	```pwsh
	cargo fmt -p ${input:crateName}
	cargo clippy -p ${input:crateName} --all-features -- -D warnings
	cargo test -p ${input:crateName}
	cargo doc --no-deps -p ${input:crateName}
	```

	- Inspect the generated documentation (`target/doc/${input:crateName}/index.html`) similar to how the Java quickstart reviews Javadoc output.

2. **Docs, samples, and crate metadata files**
	- Ensure `README.md` follows the service template (auth snippet, example client call, TypeSpec provenance section) and convert placeholder links to absolute URLs, as emphasized in the Java instructions. If `tsp-client` did not create the README, author it manually using the latest key vault crates as a reference.
	- Author `CHANGELOG.md` in the crate root with an `Unreleased` section and at least one "Features Added" bullet calling out the initial preview. Reference `.github/instructions/changelog.instructions.md` for formatting.
	- Add `sdk/${input:serviceDirectory}/${input:crateName}/ci.yml` (or update the service-level `ci.yml`) so the continuous-integration pipeline builds, tests, and releases the new crate. Mirror the existing key vault/client service pipeline structure when adding the artifact entry.
	- Add `examples/` or `samples/` showcasing at least one happy-path scenario using the generated client, keeping code snippets synchronized with README content like the Java README/ReadmeSamples pairing.
3. **Tests**
	- Implement `#[cfg(test)]` unit tests in the crate (for example, building minimal `tests` modules in each public module) to cover serialization helpers, request builder defaults, and any handwritten helpers. Use the guidance from `.github/copilot-instructions.md` (tests at bottom of module, importing from `super`).
	- Where the service supports recorded integration tests, add skeletons under `tests/` with `#[recorded::test]` attributes and follow `CONTRIBUTING.md` for provisioning resources.
    - Make sure to implement functional tests to verify the functionality of the generated crate.
    - When creating functional tests, ensure that a correct test-resources.bicep file is created to create the required test resources for the service client.
    - Run the new-testresources.ps1 powershell script to create the required test resources for functional tests before running the functional tests initially.
4. **Examples**
    - Implement examples for the newly generated service client following the model used for the azure_security_keyvault_secrets examples.
5. **Metadata**
	- Update `sdk/${input:serviceDirectory}/${input:crateName}/ci.yml` if a new pipeline is required.
	- Add an entry to `CHANGELOG.md` under `Unreleased` describing the new preview package.
6. **Validation & local install**
	- Run `cargo xtask fix --stage fmt` if the repo uses it (mention checking the contributing guide).
	- If you need to consume the crate locally (analogous to `mvn install`), run `cargo install --path sdk/${input:serviceDirectory}/${input:crateName}` into a temporary prefix or `cargo publish --dry-run` to validate packaging.
	- Verify no files under `generated/` were manually edited.

7. **Feature checklist**
	- Authentication & credentials: document the supported credential flows and validate them end-to-end.
	- Long-running operations: ensure pollers follow Azure LRO guidance and that samples/tests cover them.
	- File upload/download: verify binary payload helpers and mention required content types/chunking behavior.
	- Samples/tests: add runnable examples and live/playback-style tests where feasible, similar to the Java guidance on generated tests.

## 6. Communication & review reminders

- Link to `aka.ms/azsdk/dpcodegen` for architecture-board policy.
- Call out that any new SDK requires board approval and onboarding into the release pipeline.
- Encourage filing questions in the TypeSpec Teams channel (`@DPG Rust`) and JavaScript/Java review channels as applicable.
- Loop in APIView reviewers (`apiview.dev`) once the crate builds, mirroring the Java quickstart’s dual review model (PR + APIView).
- Highlight release readiness: ensure CHANGELOGs, CI definitions, and release scripts are updated before requesting the "increment versions" automation or publishing the crate.

## Deliverable template

When you finish, deliver a short status summary that includes:

- Confirmation that `tsp-client init/update` or the Azure SDK Generation Pipeline ran (include command output snippets or pipeline links).
- Location of the generated crate (`sdk/${input:serviceDirectory}/${input:crateName}`).
- Build/test/doc command results.
- Follow-ups (e.g., pending API review, missing recordings, release checklist items).

````
