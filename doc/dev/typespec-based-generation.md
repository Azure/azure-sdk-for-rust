<!-- cspell: ignore tspconfig eventgrid -->
# Creating a TypeSpec-Based Rust SDK Client

This guide provides step-by-step instructions for creating a new Rust SDK client from TypeSpec definitions. There are two approaches:

1. **Fully Generated Client** - All code is generated from TypeSpec (e.g., `azure_security_keyvault_secrets`). This is the solution which is expected for the vast majority of Rust clients.
2. **Partially Generated Client** - Generated code with custom client wrappers (e.g., `azure_security_keyvault_certificates`). This solution should be limited to the very
small set of SDK clients which require additional levels of customization beyond what is available in the `client.tsp` file.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Part 1: TypeSpec Specification Setup](#part-1-typespec-specification-setup)
- [Part 2: Initial Crate Setup](#part-2-initial-crate-setup)
- [Part 3: Generating the SDK - Fully Generated Client](#part-3-generating-the-sdk---fully-generated-client)
- [Part 4: Generating the SDK - Partially Generated Client](#part-4-generating-the-sdk---partially-generated-client)
- [Part 5: Testing](#part-5-testing)
- [Part 6: Documentation and Examples](#part-6-documentation-and-examples)
- [Part 7: CI/CD Configuration](#part-7-cicd-configuration)
- [Part 8: Updating Generated Code](#part-8-updating-generated-code)

## Prerequisites

Before you begin, ensure you have the following installed:

1. Familiarize yourself with the [Azure API Guidelines for Rust](https://azure.github.io/azure-sdk/rust_introduction.html). These define the required
conventions for an Azure SDK for Rust crate.

2. **Rust toolchain** (version 1.85 or later)

    To install the rustup tool, follow the [rust-lang install instructions](https://rust-lang.org/tools/install)

3. **Node.js 22+** and **npm**

    To install node initially, follow the [node installation instructions](https://nodejs.org/en/download)

4. **tsp-client dependencies** (from the repository root)
    Installed globally (preferred):

    ```bash
    npm install -g @azure-tools/typespec-client-generator-cli
    ```

    or locally:

    ```bash
    npm ci --prefix eng/common/tsp-client
    ```

5. **Azure CLI** (for testing and resource provisioning)

    To install the Azure CLI, follow the  [Azure CLI install documentation](https://learn.microsoft.com/cli/azure/install-azure-cli?view=azure-cli-latest)

    ```bash
    az --version
    ```

6. Bicep (for testing and resource provisioning)

    ```bash
    az bicep install
    ```

7. **PowerShell** (for running test resource scripts on Windows)

    Powershell install instructions [can be found here](https://learn.microsoft.com/powershell/scripting/install/installing-powershell?view=powershell-7.5).

## Part 1: TypeSpec Specification Setup

### Step 1.1: Verify TypeSpec Definitions Exist

Your service's TypeSpec definitions should exist in the [Azure/azure-rest-api-specs](https://github.com/Azure/azure-rest-api-specs) repository under `specification/<service-name>/`.

Required files:

- `main.tsp` - The service contract
- `client.tsp` - Client customizations
- `tspconfig.yaml` - TypeSpec compiler configuration

    **Example**: For Key Vault Secrets, the files are located at:

    ```text
    specification/keyvault/Security.KeyVault.Secrets/
    ├── main.tsp
    ├── client.tsp
    └── tspconfig.yaml
    ```

### Step 1.2: Work with your language architect to determine the language specific name for your crate

Typically the crate name will be similar to existing crate names, but that is not at all guaranteed.

### Step 1.3: Add Rust Emitter Configuration to tspconfig.yaml

If the `tspconfig.yaml` doesn't already have Rust emitter configuration, you need to add it:

1. Fork or clone the `Azure/azure-rest-api-specs` repository
2. Create a new branch
3. Edit `specification/<service-name>/<TypeSpec.Dir>/tspconfig.yaml`
4. Add the Rust emitter configuration under the `options` section:

    ```yaml
    options:
      "@azure-tools/typespec-rust":
        emitter-output-dir: "{output-dir}/<service-dir>/<crate-name>"
        crate-name: "<crate-name>"
        crate-version: "0.1.0"
    ```

    **Example** for EventGrid:

    ```yaml
    options:
      "@azure-tools/typespec-rust":
        emitter-output-dir: "{output-dir}/{service-dir}/{crate-name}"
        crate-name: "azure_messaging_eventgrid"
        crate-version: "0.1.0"
    ```

5. Create a pull request with title: "Add Rust emitter support to `<ServiceName>` TypeSpec"
6. Wait for the PR to be merged (or use a local clone for development)

### Step 1.4: Note the Commit SHA

Once your changes are merged (or if you're using a local clone for development), note the commit SHA:

```bash
cd /path/to/azure-rest-api-specs
git rev-parse HEAD
```

You'll need this SHA for the `tsp-location.yaml` file.

## Part 2: Initial Crate Setup

### Step 2.1: Add Crate to Workspace

Edit the root `Cargo.toml` file to add your new crate to the workspace members:

```toml
[workspace]
members = [
    # ... existing members ...
    "sdk/<service-dir>/<crate-name>",
]
```

### Step 2.2: Generate Initial Code with tsp-client init

From your azure-sdk-for-rust root directory, run:

```bash
tsp-client init https://github.com/Azure/azure-rest-api-specs/blob/<commit>/specification/<service>/<path-to-tspconfig.yaml file>
```

**Example**:

```bash
tsp-client init https://github.com/Azure/azure-rest-api-specs/blob/fc08a74c8fd3b28ce3aba302d53785031ede3189/specification/eventgrid/Azure.Messaging.EventGrid/tspconfig.yaml
```

If using a local clone of azure-rest-api-specs for development:

```bash
tsp-client init --tsp-config /path/to/azure-rest-api-specs/...
```

This will:

- Download the TypeSpec files from the specified commit
- Generate Rust code in `<service>/src/generated/`
- Create initial module structure, including cargo.toml and other components.
- Populate the tsp-location.yaml with the information from the URL .

This will NOT:

- Create a README.md file for your package
- Create a CHANGELOG.md file for your package
- Create tests for your package.
- Customize the crate.toml file

Those steps will be listed in subsequent steps.

### Step 2.3: Updates to SDK client after the initial generation

To update the SDK client after the client TSP file has been updated, run the `tsp-client` command again, specifying the `update` command instead of the `init` command.

For example:

```bash
tsp-client update https://github.com/Azure/azure-rest-api-specs/blob/fc08a74c8fd3b28ce3aba302d53785031ede3189/specification/eventgrid/Azure.Messaging.EventGrid/tspconfig.yaml
```

## Part 3: Generating the SDK - Fully Generated Client

The `tsp-client init` command should generate a fully functional Rust package for the client, the next steps involve filling in the missing pieces to create a fully functional SDK client.

### Part 3.1: Update the cargo.toml file

The cargo.toml file created will be missing some important sections which need to be filled in:

```toml
[package]
description = "Rust wrappers around Microsoft Azure REST APIs - Azure Key Vault Secrets"
readme = "README.md"
homepage = "https://github.com/azure/azure-sdk-for-rust"
documentation = "https://docs.rs/<your package name>"
# Note: There can be no more than 5 keywords for your package.
keywords = ["Keywords", "for", "your", "package"]
categories = ["api-bindings"]
```

In addition to the sections above, there are a number of additional sections to add to the cargo.toml file:

```toml
# enable common lints for all packages.
[lints]
workspace = true
```

```toml
[dev-dependencies]
# Development time dependencies (dependencies for perf tests, etc).
```

### Step 3.2: Build and Format

Build and format the generated code:

```bash
cargo fmt -p <crate-name>
cargo clippy -p <crate-name> --all-features -- -D warnings
cargo build -p <crate-name>
cargo test -p <crate-name>
```

## Part 4: Generating the SDK - Partially Generated Client

A partially generated client wraps the generated code with custom implementations to provide better ergonomics or additional functionality.

To create a partially generated client, modify the `src/lib.rs` file to include your modifications.

For an example of a partially modified client, look at the [Azure KeyVault Certificates](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/keyvault/azure_security_keyvault_certificates).

## Part 5: Testing

### Step 5.1: Create test-resources.bicep

Create `sdk/<service-dir>/test-resources.bicep` to define Azure resources needed for testing:

```bicep
// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

param baseName string = resourceGroup().name
param testApplicationOid string
param location string = resourceGroup().location


var adminDefinitionId = '<role-definition-guid>'
var adminAssignmentName = guid(resourceGroup().id, adminDefinitionId, testApplicationOid)

resource serviceResource 'Microsoft.<Provider>/<ResourceType>@<api-version>' = {
    name: baseName
    location: location
    properties: {
    // Resource-specific properties
    }
}

resource admin 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
    name: adminAssignmentName
    properties: {
    roleDefinitionId: resourceId('Microsoft.Authorization/roleDefinitions', adminDefinitionId)
    principalId: testApplicationOid
    }
}

output AZURE_<SERVICE>_URL string = serviceResource.properties.endpoint
```

See other existing `test-resources.bicep` files in the repository for more detailed examples.

### Step 5.2: Create Integration Tests

Create test files in `tests/` directory using the `#[recorded::test]` attribute:

**File**: `tests/<client_name>.rs`

```rust
// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::Result;
use azure_core_test::{recorded, TestContext};
use <crate_name>::{<ClientName>, <ClientName>Options};

#[recorded::test]
async fn basic_operation(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();

    let mut options = <ClientName>Options::default();
    recording.instrument(&mut options.client_options);

    let client = <ClientName>::new(
        recording.var("AZURE_<SERVICE>_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Perform test operations
    let result = client.some_operation("test-name").await?;
    assert!(result.is_ok());

    Ok(())
}
```

### Step 5.3: Provision Test Resources

Before running live tests, provision the required Azure resources:

```powershell
# From repository root
eng/common/TestResources/New-TestResources.ps1 -ServiceDirectory <service-dir>
```

This script will:

1. Deploy the resources defined in `test-resources.bicep`
2. Create a `.env` file with connection information
3. Set environment variables for testing

### Step 5.4: Run Tests

```bash
# Run tests in playback mode (using recordings)
cargo test -p <crate-name>

# Record new test sessions (requires provisioned resources)
AZURE_TEST_MODE=record cargo test -p <crate-name>

# Run tests in live mode
AZURE_TEST_MODE=live cargo test -p <crate-name>
```

### Enum wire-contract tests

Rust variant names and service wire values are separate contracts. Test exact
wire spellings rather than deriving expectations from a variant's name or
accepting successful deserialization alone.

Group offline tests by related enum families. Cover every declared value through
JSON deserialization and serialization, along with `FromStr`, `AsRef<str>`, and
`Display` where generated. Include representative nested models, wrong-case
inputs, and unknown values: extensible enums should preserve unrecognized
strings, while closed enums should reject them. These tests verify the declared
contract without provisioning resources; a round trip alone cannot prove that
the service uses the same contract.

Recorded tests should compare actual response strings with explicit expected
wire values and named Rust variants. Check serialized request values for enums
used as inputs, but do not send every enum value merely to enumerate it. Cover
related operation scenarios and respect whether fields are writable, read-only,
optional, or omitted by a particular operation. Compare enum arrays without
assuming order unless the service guarantees it, while checking their complete
contents and count.

If a recording reveals a mismatch, inspect the specification, generated wire
mapping, and service payload before assigning the cause. For example, Key Vault
declared `notify` and `rotate`, but recorded policy responses returned `Notify`
and `Rotate`; PascalCase Rust identifiers were not evidence of an emitter bug.
Record required specification or emitter corrections separately from local test
work. If a known service value is missing from an extensible enum, an explicit
assertion of its preserved `UnknownValue` can document the discrepancy, but is
not named-variant coverage. Do not normalize casing globally to hide it.

Inspect sanitizers before treating a playback value as service evidence. The
default `SANITIZE_BODY_NAME` targets `$..name`, including nested SKU names.
Configure any necessary exception per recording, preserve other sanitization,
and inspect recordings for sensitive values. Re-record only when the request
sequence or recording configuration needs updating; use
`AZURE_TEST_MODE=playback` to validate saved recordings.

## Part 6: Documentation and Examples

### Step 6.1: Create README.md

Create `sdk/<service-dir>/<crate-name>/README.md`.

Keep the README consumer-focused. Use this section order, following the published
[Key Vault secrets README](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/keyvault/azure_security_keyvault_secrets/README.md):

1. Library title, service overview, and source/package/API/product links.
2. **Getting started**: install the package, prerequisites, install example
   dependencies, authenticate, and instantiate a client.
3. **Key concepts**.
4. **Examples** of primary scenarios.
5. **Troubleshooting**, beginning with **General**.
6. **Contributing**: the standard repository contribution link, CLA, and Code of
   Conduct text.
7. Reference-style link definitions under `<!-- LINKS -->`.

Preserve executable example labels used by README tests. Use `rust no_run` for
examples with placeholders, and keep service-specific limitations relevant to
consumers visible. Copy the standard contribution boilerplate from a published
crate rather than replacing it with a link to a crate-local contributor guide.

Put generation commands, test provisioning, environment variables, recording
instructions, resource budgets, and contributor troubleshooting in a sibling
`CONTRIBUTING.md`. Link to the repository contribution guide there and document
only crate-specific differences. Crate-local contributor guides need not be
linked from the README.

The [document-crates skill](../../.github/skills/document-crates/SKILL.md) provides
a compact checklist for planning or editing these files.

It is often helpful to look at the `README.md` for the package you are creating from other SDK languages e.g., [sdk/keyvault/azure_security_keyvault_secrets/README.md](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/keyvault/azure_security_keyvault_secrets/README.md). They usually contain helpful information that should be included in your `README.md` (this is helpful for the Key Concepts and Hero scenarios sections. It is also extremely helpful to look at other packages within the Azure SDK for Rust SDK for examples of content (this is especially true for the Prerequisites and References sections).

### Step 6.2: Create CHANGELOG.md

Create `sdk/<service-dir>/<crate-name>/CHANGELOG.md`:

```markdown
# Release History

## 0.1.0 (Unreleased)

### Features Added

- Initial preview release of the `<crate-name>` crate
- Support for <list key operations>

### Breaking Changes

### Bugs Fixed

### Other Changes
```

## Part 7: CI/CD Configuration

### Step 7.1: Create or Update ci.yml

Create `sdk/<service-dir>/ci.yml` if it doesn't exist:

```yaml
# NOTE: Please refer to https://aka.ms/azsdk/engsys/ci-yaml before editing this file.
trigger:
    branches:
    include:
    - main
    - hotfix/*
    - release/*
    paths:
    include:
    - sdk/<service-dir>/

parameters:
- name: RunLiveTests
    displayName: Run live tests
    type: boolean
    default: false
- name: release_<crate_name>
    displayName: <crate-name>
    type: boolean
    default: false

extends:
    template: /eng/pipelines/templates/stages/archetype-sdk-client.yml
    parameters:
    ServiceDirectory: <service-dir>
    RunLiveTests: ${{ or(parameters.RunLiveTests, eq(variables['Build.Reason'], 'Schedule')) }}
    Artifacts:
    - name: <crate-name>
        releaseInBatch: ${{ parameters.release_<crate_name> }}
```

Or update an existing `ci.yml` to add your crate to the artifacts list:

```yaml
Artifacts:
- name: <existing-crate>
    releaseInBatch: ${{ parameters.release_<existing_crate> }}
- name: <new-crate>
    releaseInBatch: ${{ parameters.release_<new_crate> }}
```

## Part 8: Updating Generated Code

### Step 8.1: Update TypeSpec Commit

When TypeSpec definitions are updated in azure-rest-api-specs:

1. Note the new commit SHA
2. Update `tsp-location.yaml` with the new commit:

    ```yaml
    directory: specification/<service-name>/<TypeSpec.Dir>
    commit: <new-commit-sha>
    repo: Azure/azure-rest-api-specs
    ```

### Step 8.2: Regenerate Code

From the azure-sdk-for-rust repo root directory:

```bash
tsp-client update
```

### Step 8.3: Review and Test

1. Review the generated code changes
2. Update any custom code if needed
3. Run tests to ensure compatibility
4. Update CHANGELOG.md with breaking changes or new features

## Summary Checklist

Before submitting your SDK for review:

- [ ] TypeSpec definitions exist and include Rust emitter configuration
- [ ] `tsp-location.yaml` created with correct commit SHA
- [ ] Crate added to workspace `Cargo.toml`
- [ ] Code successfully generated with `tsp-client update`
- [ ] `Cargo.toml` created with proper metadata and dependencies
- [ ] `src/lib.rs` exports generated and custom code appropriately
- [ ] Custom implementations in `src/clients.rs` and `src/models.rs` (if partially generated)
- [ ] `src/resource.rs` provides resource utilities
- [ ] `test-resources.bicep` defines required Azure resources
- [ ] Integration tests created in `tests/` directory
- [ ] Tests pass: `cargo test -p <crate-name>`
- [ ] Code formatted: `cargo fmt -p <crate-name>`
- [ ] No clippy warnings: `cargo clippy -p <crate-name> -- -D warnings`
- [ ] `README.md` created with examples and authentication instructions
- [ ] `CHANGELOG.md` created with initial release notes
- [ ] `ci.yml` updated to include new crate
- [ ] Documentation builds: `cargo doc --no-deps -p <crate-name>`

## Additional Resources

- [TypeSpec Documentation](https://microsoft.github.io/typespec/)
- [Azure SDK Design Guidelines for Rust](https://azure.github.io/azure-sdk/rust_introduction.html)
- [Contributing Guide](https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md)
- [Azure REST API Specs Repository](https://github.com/Azure/azure-rest-api-specs)
