<!-- cspell: ignore tspconfig eventgrid -->
# Creating a TypeSpec-Based Rust SDK Client

This guide provides step-by-step instructions for creating a new Rust SDK client from TypeSpec definitions. There are two approaches:

1. **Fully Generated Client** - All code is generated from TypeSpec (e.g., `azure_security_keyvault_secrets`)
2. **Partially Generated Client** - Generated code with custom client wrappers (e.g., `azure_security_keyvault_certificates`)

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

1. **Rust toolchain** (version 1.85 or later)

To install the rustup tool, follow the [rust-lang install instructions](https://rust-lang.org/tools/install)

   ```bash
   rustup toolchain install 1.85
   ```

1. **Node.js 20+** and **npm**

To install node initially, follow the [node installation instructions](https://nodejs.org/en/download)

   ```bash
   node --version  # Should be 20.x or later
   npm --version
   ```

1. **tsp-client dependencies** (from the repository root)
Installed globally (preferred):

```bash
npm install -g @azure-tools/typespec-client-generator-cli
```

or locally:

```bash
npm ci --prefix eng/common/tsp-client
```

1. **Azure CLI** (for testing and resource provisioning)

To install the Azure CLI, follow the  [Azure CLI install documentation](https://learn.microsoft.com/cli/azure/install-azure-cli?view=azure-cli-latest)

   ```bash
   az --version
   ```

1. **PowerShell** (for running test resource scripts on Windows)

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
1. Create a new branch
1. Edit `specification/<service-name>/<TypeSpec.Dir>/tspconfig.yaml`
1. Add the Rust emitter configuration under the `options` section:

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

1. Create a pull request with title: "Add Rust emitter support to `<ServiceName>` TypeSpec"
1. Wait for the PR to be merged (or use a local clone for development)

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
keywords = ["Keywords", "for", "your", "package"]
categories = ["categories", "for", "your", "package"]
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

**Example** for Key Vault:

```bicep
param baseName string = resourceGroup().name
param tenantId string = '72f988bf-86f1-41af-91ab-2d7cd011db47'
param testApplicationOid string
param location string = resourceGroup().location
@allowed(['standard', 'premium'])
param keyVaultSku string = 'premium'

var kvAdminDefinitionId = '00482a5a-887f-4fb3-b363-3b7fe8e74483'
var kvAdminAssignmentName = guid(resourceGroup().id, kvAdminDefinitionId, testApplicationOid)

resource kv 'Microsoft.KeyVault/vaults@2023-07-01' = {
  name: baseName
  location: location
  properties: {
    sku: {
      family: 'A'
      name: keyVaultSku
    }
    tenantId: tenantId
    enableRbacAuthorization: true
    softDeleteRetentionInDays: 7
  }
}

resource kvAdmin 'Microsoft.Authorization/roleAssignments@2022-04-01' = {
  name: kvAdminAssignmentName
  properties: {
    roleDefinitionId: resourceId('Microsoft.Authorization/roleDefinitions', kvAdminDefinitionId)
    principalId: testApplicationOid
  }
}

output AZURE_KEYVAULT_URL string = kv.properties.vaultUri
```

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

**Example** from Key Vault Secrets:

```rust
use azure_core::Result;
use azure_core_test::{recorded, TestContext};
use azure_security_keyvault_secrets::{
    models::{SetSecretParameters, SecretClientGetSecretOptions},
    ResourceExt as _, SecretClient, SecretClientOptions,
};

#[recorded::test]
async fn secret_roundtrip(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();

    let mut options = SecretClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = SecretClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Set a secret
    let body = SetSecretParameters {
        value: Some("secret-value".into()),
        ..Default::default()
    };
    let secret = client
        .set_secret("secret-roundtrip", body.try_into()?, None)
        .await?
        .into_model()?;
    assert_eq!(secret.value, Some("secret-value".into()));

    // Get the secret
    let secret_version = secret.resource_id()?.version;
    let secret = client
        .get_secret(
            "secret-roundtrip",
            Some(SecretClientGetSecretOptions {
                secret_version,
                ..Default::default()
            }),
        )
        .await?
        .into_model()?;
    assert_eq!(secret.value, Some("secret-value".into()));

    Ok(())
}
```

### Step 5.3: Provision Test Resources

Before running live tests, provision the required Azure resources:

```powershell
# From repository root
eng/common/TestResources/New-TestResources.ps1 -ServiceDirectory <service-dir>
```

**Example**:

```powershell
eng/common/TestResources/New-TestResources.ps1 -ServiceDirectory keyvault
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

## Part 6: Documentation and Examples

### Step 6.1: Create README.md

Create `sdk/<service-dir>/<crate-name>/README.md`:

````markdown
# Azure <Service Name> client library for Rust

<Brief description of the service>

The Azure <Service Name> client library allows you to <describe main capabilities>.

[Source code] | [Package (crates.io)] | [API reference documentation] | [Product documentation]

## Getting started

### Install the package

Install the Azure <Service Name> client library for Rust with [Cargo]:

```sh
cargo add <crate-name>
```

### Prerequisites

* An [Azure subscription].
* An existing Azure <Service Name> resource. If you need to create one, you can use the Azure Portal or [Azure CLI].

If you use the Azure CLI:

```azurecli
az <service> create --resource-group <resource-group-name> --name <resource-name>
```

### Authenticate the client

In order to interact with the Azure <Service Name> service, you'll need to create an instance of the `<ClientName>`. You need a **service url** and credentials to instantiate a client object.

```rust no_run
use azure_identity::DeveloperToolsCredential;
use <crate_name>::{<ClientName>, models::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DeveloperToolsCredential::new(None)?;
    let client = <ClientName>::new(
        "https://your-service.azure.net/",
        credential,
        None,
    )?;

    // Use the client
    Ok(())
}
```

## Key concepts

<Describe key concepts, entities, and terminology>

## Examples

### Example 1: <Primary Operation>

```rust no_run
use azure_identity::DeveloperToolsCredential;
use <crate_name>::{<ClientName>, models::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DeveloperToolsCredential::new(None)?;
    let client = <ClientName>::new(
        "https://your-service.azure.net/",
        credential,
        None,
    )?;

    // Perform operation
    let result = client.operation("name", parameters).await?;
    println!("{:?}", result);

    Ok(())
}
```

## Troubleshooting

### Logging

Enable trace logging to see detailed information about service requests:

```rust
use tracing_subscriber;

tracing_subscriber::fmt()
    .with_max_level(tracing::Level::TRACE)
    .init();
```

## Next steps

<Links to additional documentation, samples, or related services>

## Contributing

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA).

This project has adopted the [Microsoft Open Source Code of Conduct]. For more information see the [Code of Conduct FAQ] or contact [opencode@microsoft.com] with any questions or comments.

## Provenance

This crate is generated from TypeSpec definitions in the [Azure/azure-rest-api-specs] repository.

[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/<service-dir>/<crate-name>
[Package (crates.io)]: https://crates.io/crates/<crate-name>
[API reference documentation]: https://docs.rs/<crate-name>
[Product documentation]: https://azure.microsoft.com/services/<service>/
[Azure subscription]: https://azure.microsoft.com/free/
[Azure CLI]: https://docs.microsoft.com/cli/azure
[Cargo]: https://doc.rust-lang.org/cargo/
[Microsoft Open Source Code of Conduct]: https://opensource.microsoft.com/codeofconduct/
[Code of Conduct FAQ]: https://opensource.microsoft.com/codeofconduct/faq/
[opencode@microsoft.com]: mailto:opencode@microsoft.com
[Azure/azure-rest-api-specs]: https://github.com/Azure/azure-rest-api-specs
````

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
