# Azure Identity client library for Rust

The Azure Identity library provides [Microsoft Entra ID](https://learn.microsoft.com/entra/fundamentals/whatis) ([formerly Azure Active Directory](https://learn.microsoft.com/entra/fundamentals/new-name)) authentication support across the Azure SDK.

[Source code] | [Package (crates.io)] | [API reference documentation] | [Microsoft Entra ID documentation]

## Getting started

### Install the package

Install the Azure Identity library for Rust with cargo:

```bash
cargo add azure_identity
```

### Prerequisites

* An [Azure subscription].

### Authenticate during local development

The Azure Identity library supports authenticating through developer tools to simplify local development and debugging.

#### Authenticate via the Azure CLI

`DeveloperToolsCredential` and `AzureCliCredential` can authenticate as the user signed in to the [Azure CLI]. To log in to the Azure CLI, run `az login` as described in [Azure CLI documentation](https://learn.microsoft.com/cli/azure/get-started-with-azure-cli#sign-in-to-azure).

#### Authenticate via the Azure Developer CLI

`DeveloperToolsCredential` and `AzureDeveloperCliCredential` can authenticate as the user signed in to the [Azure Developer CLI]. To log in to the Azure Developer CLI, run `azd auth login` as described in [Azure Developer CLI documentation](https://learn.microsoft.com/azure/developer/azure-developer-cli/reference#azd-auth-login).

## Key concepts

### Credentials

A credential is a struct that can acquire access tokens for Azure resources. The Azure Identity library offers various credentials for use by Azure SDK service clients. See the [Credential structures](#credential-structures "Credential structures") section for a list of this library's credentials.

## Examples

### Authenticate with `DeveloperToolsCredential`

`DeveloperToolsCredential` simplifies authentication while developing apps. It attempts to authenticate via a series of developer tools such as the Azure CLI, stopping when one succeeds. After receiving a token from a particular tool, it uses that tool for all subsequent token requests. See the type's [reference documentation][devtool_cred_ref] for more details.

This example demonstrates authenticating the `SecretClient` from the [azure_security_keyvault_secrets] crate using `DeveloperToolsCredential`.

```rust ignore dev
use azure_identity::DeveloperToolsCredential;
use azure_security_keyvault_secrets::SecretClient;

let credential = DeveloperToolsCredential::new(None)?;
let client = SecretClient::new("https://TODO.vault.azure.net/", credential.clone(), None)?;
```

### Authenticating with a Federated Identity Credential (FIC)

This example demonstrates how to authenticate an Entra application with an access token from a managed identity. See [Configure an application to trust a managed identity](https://learn.microsoft.com/entra/workload-id/workload-identity-federation-config-app-trust-managed-identity) for more information about this scenario. This example shows a Key Vault client, however the same approach can work with any Azure SDK client that uses a `TokenCredential`.

```rust no_run
use azure_core::credentials::TokenCredential;
use azure_core::http::ClientMethodOptions;
use azure_identity::{ClientAssertion, ClientAssertionCredential, ManagedIdentityCredential};
use azure_security_keyvault_secrets::SecretClient;
use std::sync::Arc;

#[derive(Debug)]
struct AccessTokenAssertion {
    credential: Arc<dyn TokenCredential>,
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl ClientAssertion for AccessTokenAssertion {
    async fn secret(&self, _: Option<ClientMethodOptions<'_>>) -> azure_core::Result<String> {
        Ok(self
            .credential
            .get_token(&[&"api://AzureADTokenExchange/.default"], None)
            .await?
            .token
            .secret()
            .to_string())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let assertion = AccessTokenAssertion {
        credential: ManagedIdentityCredential::new(None)?,
    };

    // this credential exchanges the managed identity's tokens for the specified Entra application's tokens
    let credential = ClientAssertionCredential::new(
        String::from("tenant ID"),
        String::from("client ID"),
        assertion,
        None,
    )?;

    let _client = SecretClient::new("https://TODO.vault.azure.net/", credential.clone(), None)?;

    Ok(())
}
```

## Credential structures

### Developer tools

|Credential|Usage
|-|-
|[`AzureCliCredential`][cli_cred_ref]| Authenticate with [Azure CLI][Azure CLI].
|[`AzureDeveloperCliCredential`][azd_cred_ref]| Authenticate with [Azure Developer CLI][Azure Developer CLI].
|[`DeveloperToolsCredential`][devtool_cred_ref]| Simplified authentication for application development.

### Azure-hosted applications

|Credential|Usage
|-|-
|[`ManagedIdentityCredential`][managed_id_cred_ref]| Authenticate the managed identity of an Azure resource.
|[`WorkloadIdentityCredential`][workload_id_cred_ref]| Supports [Workload Identity on Kubernetes](https://learn.microsoft.com/azure/aks/workload-identity-overview).

### Service principals

See [Service principal authentication](https://learn.microsoft.com/entra/identity-platform/app-objects-and-service-principals) for general information about service principals.

|Credential|Usage
|-|-
|[`AzurePipelinesCredential`][az_pipelines_cred_ref]| Authenticate an Azure Pipelines [service connection](https://learn.microsoft.com/azure/devops/pipelines/library/service-endpoints).
|[`ClientAssertionCredential`][assert_cred_ref]| Authenticate a service principal with client assertions.
|[`ClientCertificateCredential`][cert_cred_ref]| Authenticate a service principal with a certificate.
|[`ClientSecretCredential`][secret_cred_ref]| Authenticate a service principal with a secret.

## Next steps

### Client library support

Client and management libraries listed on the [Azure SDK release page](https://azure.github.io/azure-sdk/releases/latest/#rust) that support Microsoft Entra authentication accept credentials from this library. You can learn more about using these libraries in their documentation, which is available at [Docs.rs](https://docs.rs).

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
[Azure Developer CLI]: https://learn.microsoft.com/azure/developer/azure-developer-cli/overview
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
