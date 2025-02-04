# Azure Identity client library for Rust

The Azure Identity library provides [Microsoft Entra ID](https://learn.microsoft.com/entra/fundamentals/whatis) ([formerly Azure Active Directory](https://learn.microsoft.com/entra/fundamentals/new-name)) token authentication support across the Azure SDK. It provides a set of [`TokenCredential`][token_cred_ref] implementations that can be used to construct Azure SDK clients that support Microsoft Entra token authentication.

[Source code] | [Package (crates.io)] | [API reference documentation] | [Microsoft Entra ID documentation]

## Getting started

### Install the package

Install the Azure Identity library for Rust with cargo:

```bash
cargo add azure_identity
```

### Prerequisites

* An [Azure subscription].
* The [Azure CLI] can also be useful for authenticating in a development environment, creating accounts, and managing account roles.

### Authenticate during local development

When debugging and executing code locally, it's typical for developers to use their own accounts for authenticating calls to Azure services. The Azure Identity library supports authenticating through developer tools to simplify local development.

#### Authenticate via the Azure CLI

`DefaultAzureCredential` and `AzureCliCredential` can authenticate as the user signed in to the [Azure CLI]. To sign in to the Azure CLI, run `az login`. On a system with a default web browser, the Azure CLI launches the browser to authenticate a user.

When no default browser is available, `az login` uses the device code authentication flow. This flow can also be selected manually by running `az login --use-device-code`.

## Key concepts

### Credentials

A credential is a class that contains or can obtain the data needed for a service client to authenticate requests. Service clients across the Azure SDK accept a credential instance when they're constructed, and use that credential to authenticate requests.

The Azure Identity library focuses on OAuth authentication with Microsoft Entra ID. It offers various credential classes capable of acquiring a Microsoft Entra access token. See the [Credential classes](#credential-classes "Credential classes") section for a list of this library's credential classes.

### DefaultAzureCredential

`DefaultAzureCredential` simplifies authentication while developing apps that deploy to Azure by combining credentials used in Azure hosting environments with credentials used in local development. For more information, see [DefaultAzureCredential overview][dac_overview].

#### Continuation policy

`DefaultAzureCredential` attempts to authenticate with all developer credentials until one succeeds, regardless of any errors previous developer credentials experienced. For example, a developer credential may attempt to get a token and fail, so `DefaultAzureCredential` will continue to the next credential in the flow. Deployed service credentials stop the flow with a thrown exception if they're able to attempt token retrieval, but don't receive one.

This allows for trying all of the developer credentials on your machine while having predictable deployed behavior.

## Examples

The following examples are provided:
<!-- no toc -->
* [Authenticate with DefaultAzureCredential](#authenticate-with-defaultazurecredential "Authenticate with DefaultAzureCredential")

### Authenticate with `DefaultAzureCredential`

More details on configuring your environment to use `DefaultAzureCredential` can be found in the class's [reference documentation][default_cred_ref].

This example demonstrates authenticating the `SecretClient` from the [azure_security_keyvault_secrets] crate using `DefaultAzureCredential`.

```rust
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::SecretClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DefaultAzureCredential::new()?;
    let client = SecretClient::new("https://your-key-vault-name.vault.azure.net/", credential.clone(), None)?;
    Ok(())
}
```

## Credential classes

### Credential chains

|Credential|Usage
|-|-
|[`DefaultAzureCredential`][default_cred_ref]| Provides a simplified authentication experience to quickly start developing applications run in Azure.

### Authenticate Azure-hosted applications

|Credential|Usage
|-|-
|[`ImdsManagedIdentityCredential`][managed_id_cred_ref]| Authenticates the managed identity of an Azure resource.
|[`WorkloadIdentityCredential`][workload_id_cred_ref]| Supports [Microsoft Entra Workload ID](https://learn.microsoft.com/azure/aks/workload-identity-overview) on Kubernetes.

### Authenticate service principals

|Credential|Usage|Reference
|-|-|-
|[`ClientCertificateCredential`][cert_cred_ref]| Authenticates a service principal using a certificate. | [Service principal authentication](https://learn.microsoft.com/entra/identity-platform/app-objects-and-service-principals)

### Authenticate via development tools

|Credential|Usage|Reference
|-|-|-
|[`AzureCliCredential`][cli_cred_ref]| Authenticates in a development environment with the Azure CLI. | [Azure CLI authentication](https://learn.microsoft.com/cli/azure/authenticate-azure-cli)

## Next steps

### Client library support

Client and management libraries <!-- TODO: Update link and uncomment when Rust SDK has a page on the releases site.> listed on the [Azure SDK release page](https://azure.github.io/azure-sdk/releases/latest/python.html)</!--> that support Microsoft Entra authentication accept credentials from this library. You can learn more about using these libraries in their documentation, which is <!-- TODO: uncomment when Rust SDK has a release page.>linked from the release page</!-->available at [Docs.rs](https://Docs.rs).

### Provide feedback

If you encounter bugs or have suggestions, [open an issue](https://github.com/Azure/azure-sdk-for-rust/issues).

## Contributing

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit [https://cla.microsoft.com](https://cla.microsoft.com).

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You'll only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/). For more information, see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.

<!-- LINKS -->
[Azure CLI]: https://learn.microsoft.com/cli/azure
[azure_security_keyvault_secrets]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/keyvault/azure_security_keyvault_secrets
[Azure subscription]: https://azure.microsoft.com/free/
[cert_cred_ref]: <!-- TODO: When Docs.rs page for ClientCertificateCredential ref docs are available -->
[cli_cred_ref]: <!-- TODO: When Docs.rs page for AzureCliCredential ref docs are available>
[dac_overview]: <!-- TODO: When we have a conceptual doc on Credential chains with a section on DefaultAzureCredential overview. Python example: https://learn.microsoft.com/azure/developer/python/sdk/authentication/credential-chains?tabs=dac#defaultazurecredential-overview -->
[default_cred_ref]: <!-- TODO: When Docs.rs page for DefaultAzureCredential ref docs are available -->
[Microsoft Entra ID documentation]: https://learn.microsoft.com/entra/identity/
[API reference documentation]: https://docs.rs/azure_identity/latest/azure_identity/
[managed_id_cred_ref]: <!-- TODO: When Docs.rs page for ImdsManagedIdentityCredential ref docs are available -->
[Package (crates.io)]: https://crates.io/crates/azure_identity
[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity
[token_cred_ref]: <!-- TODO: When Docs.rs page for TokenCredential trait ref docs are available -->
[workload_id_cred_ref]: <!-- TODO: When Docs.rs page for WorkloadIdentityCredential ref docs are available -->
