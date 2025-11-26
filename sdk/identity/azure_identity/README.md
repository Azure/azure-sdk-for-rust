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

`DeveloperToolsCredential` and `AzureCliCredential` can authenticate as the user signed in to the [Azure CLI]. To sign in to the Azure CLI, run `az login`. On a system with a default web browser, the Azure CLI launches the browser to authenticate a user.

When no default browser is available, `az login` uses the device code authentication flow. This flow can also be selected manually by running `az login --use-device-code`.

## Key concepts

### Credentials

A credential is a struct that contains or can obtain the data needed for a service client to authenticate requests. Service clients across the Azure SDK accept a credential instance when they're constructed, and use that credential to authenticate requests.

The Azure Identity library focuses on OAuth authentication with Microsoft Entra ID. It offers various credentials capable of acquiring a Microsoft Entra access token. See the [Credential structures](#credential-structures "Credential structures") section for a list of this library's credentials.

## Examples

### Authenticate with `DeveloperToolsCredential`

`DeveloperToolsCredential` simplifies authentication while developing apps. It attempts to authenticate via developer tools such as the Azure CLI, stopping when one succeeds. After receiving a token from a particular tool, it uses that tool for all subsequent token requests. See the type's [reference documentation][devtool_cred_ref] for more details.

This example demonstrates authenticating the `SecretClient` from the [azure_security_keyvault_secrets] crate using `DeveloperToolsCredential`.

```rust
use azure_identity::DeveloperToolsCredential;
use azure_security_keyvault_secrets::SecretClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DeveloperToolsCredential::new(None)?;
    let client = SecretClient::new("https://your-key-vault-name.vault.azure.net/", credential.clone(), None)?;
    Ok(())
}
```

### Authenticate with `ClientAssertionCredential`

This example demonstrates how to use the `ClientAssertionCredential` in conjunction with `VirtualMachineManagedIdentityCredential` in order to retrieve an access token as an app registration
that a virtual machine identity has been federated for, which can be used in "service to service"
authentication flows. For more details on this scenario see [Configure an application to trust a managed identity](https://learn.microsoft.com/entra/workload-id/workload-identity-federation-config-app-trust-managed-identity?tabs=microsoft-entra-admin-center)

```rust no_run
use azure_core::credentials::{AccessToken, TokenCredential};
use azure_core::http::ClientMethodOptions;
use azure_identity::{ClientAssertion, ClientAssertionCredential, ManagedIdentityCredential};
use std::sync::Arc;

#[derive(Debug)]
struct VmClientAssertion {
    credential: Arc<dyn TokenCredential>,
    scope: String,
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl ClientAssertion for VmClientAssertion {
    async fn secret(&self, _: Option<ClientMethodOptions<'_>>) -> azure_core::Result<String> {
        Ok(self
            .credential
            .get_token(&[&self.scope], None)
            .await?
            .token
            .secret()
            .to_string())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let assertion = VmClientAssertion {
        credential: ManagedIdentityCredential::new(None)?,
        scope: String::from("api://AzureADTokenExchange/.default"),
    };

    let client_assertion_credential = ClientAssertionCredential::new(
        String::from("guid-for-aad-tenant-id"),
        String::from("guid-for-app-id-of-client-app-registration"),
        assertion,
        None,
    )?;

    let fic_scope = String::from("your-service-app.com/scope");
    let fic_token = client_assertion_credential.get_token(&[&fic_scope], None).await?;
    Ok(())
}

```

## Credential structures

### Credential chains

|Credential|Usage
|-|-
|[`DeveloperToolsCredential`][devtool_cred_ref]| Provides a simplified authentication experience to quickly start developing applications.

### Authenticate Azure-hosted applications

|Credential|Usage
|-|-
|[`ManagedIdentityCredential`][managed_id_cred_ref]| Authenticates the managed identity of an Azure resource.
|[`WorkloadIdentityCredential`][workload_id_cred_ref]| Supports [Microsoft Entra Workload ID](https://learn.microsoft.com/azure/aks/workload-identity-overview) on Kubernetes.

### Authenticate service principals

|Credential|Usage|Reference
|-|-|-
|[`AzurePipelinesCredential`][az_pipelines_cred_ref]| Supports [Microsoft Entra Workload ID](https://learn.microsoft.com/azure/devops/pipelines/release/configure-workload-identity?view=azure-devops) on Azure Pipelines. |
|[`ClientAssertionCredential`][assert_cred_ref]| Authenticates a service principal using a signed client assertion. | [Service principal authentication](https://learn.microsoft.com/entra/identity-platform/app-objects-and-service-principals)
|[`ClientCertificateCredential`][cert_cred_ref]| Authenticates a service principal using a certificate. | [Service principal authentication](https://learn.microsoft.com/entra/identity-platform/app-objects-and-service-principals)
|[`ClientSecretCredential`][secret_cred_ref]| Authenticates a service principal using a secret. | [Service principal authentication](https://learn.microsoft.com/entra/identity-platform/app-objects-and-service-principals)

### Authenticate via development tools

|Credential|Usage|Reference
|-|-|-
|[`AzureCliCredential`][cli_cred_ref]| Authenticates in a development environment with the Azure CLI. | [Azure CLI authentication](https://learn.microsoft.com/cli/azure/authenticate-azure-cli)
|[`AzureDeveloperCliCredential`][azd_cred_ref]| Authenticates in a development environment with the Azure Developer CLI. | [Azure Developer CLI reference](https://learn.microsoft.com/azure/developer/azure-developer-cli/reference)

## Next steps

### Client library support

Client and management libraries listed on the [Azure SDK release page](https://azure.github.io/azure-sdk/releases/latest/#rust)that support Microsoft Entra authentication accept credentials from this library. You can learn more about using these libraries in their documentation, which is available at [Docs.rs](https://Docs.rs).

### Provide feedback

If you encounter bugs or have suggestions, [open an issue](https://github.com/Azure/azure-sdk-for-rust/issues).

## Contributing

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit [https://cla.microsoft.com](https://cla.microsoft.com).

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You'll only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/). For more information, see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.

<!-- LINKS -->
[assert_cred_ref]: https://docs.rs/azure_identity/latest/azure_identity/struct.ClientAssertionCredential.html
[az_pipelines_cred_ref]: https://docs.rs/azure_identity/latest/azure_identity/struct.AzurePipelinesCredential.html
[azd_cred_ref]: https://docs.rs/azure_identity/latest/azure_identity/struct.AzureDeveloperCliCredential.html
[Azure CLI]: https://learn.microsoft.com/cli/azure
[azure_security_keyvault_secrets]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/keyvault/azure_security_keyvault_secrets
[Azure subscription]: https://azure.microsoft.com/free/
[cert_cred_ref]: https://docs.rs/azure_identity/latest/azure_identity/struct.ClientCertificateCredential.html
[cli_cred_ref]: https://docs.rs/azure_identity/latest/azure_identity/struct.AzureCliCredential.html
[devtool_cred_ref]: https://docs.rs/azure_identity/latest/azure_identity/struct.DeveloperToolsCredential.html
[managed_id_cred_ref]: https://docs.rs/azure_identity/latest/azure_identity/struct.ManagedIdentityCredential.html
[Microsoft Entra ID documentation]: https://learn.microsoft.com/entra/identity/
[API reference documentation]: https://docs.rs/azure_identity/latest/azure_identity/
[Package (crates.io)]: https://crates.io/crates/azure_identity
[secret_cred_ref]: https://docs.rs/azure_identity/latest/azure_identity/struct.ClientSecretCredential.html
[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity
[token_cred_ref]: https://docs.rs/azure_core/latest/azure_core/credentials/trait.TokenCredential.html
[workload_id_cred_ref]: https://docs.rs/azure_identity/latest/azure_identity/struct.WorkloadIdentityCredential.html
