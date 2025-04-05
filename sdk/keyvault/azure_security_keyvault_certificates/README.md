# Azure Key Vault certificates client library for Rust

Azure Key Vault is a cloud service that provides secure storage of certificates for encrypting your data. Multiple certificates, and multiple versions of the same certificate, can be kept in the Azure Key Vault.

The Azure Key Vault certificates client library allows you to securely store and control the access to certificates. This library offers operations to create, import, retrieve the public key, update, delete, purge, backup, restore, and list the certificates and its versions.

[Source code] | [Package (crates.io)] | [API reference documentation] | [Product documentation]

## Getting started

### Install the package

Install the Azure Key Vault certificates client library for Rust with [Cargo]:

```sh
cargo add azure_security_keyvault_certificates
```

### Prerequisites

* An [Azure subscription].
* An existing Azure Key Vault. If you need to create an Azure Key Vault, you can use the Azure Portal or [Azure CLI].
* Authorization to an existing Azure Key Vault using either [RBAC] (recommended) or [access control].

If you use the Azure CLI, replace `<your-resource-group-name>` and `<your-key-vault-name>` with your own, unique names:

```azurecli
az keyvault create --resource-group <your-resource-group-name> --name <your-key-vault-name>
```

### Install dependencies

Add the following crates to your project:

```sh
cargo add azure_identity tokio
```

### Authenticate the client

In order to interact with the Azure Key Vault service, you'll need to create an instance of the `CertificateClient`. You need a **vault url**, which you may see as "DNS Name" in the portal, and credentials to instantiate a client object.

The example shown below uses a `DefaultAzureCredential`, which is appropriate for local development environments. We recommend using a managed identity for authentication in production environments. You can find more information on different ways of authenticating and their corresponding credential types in the [Azure Identity] documentation.

The `DefaultAzureCredential` will automatically pick up on an Azure CLI authentication. Ensure you are logged in with the Azure CLI:

```azurecli
az login
```

Instantiate a `DefaultAzureCredential` to pass to the client. The same instance of a token credential can be used with multiple clients if they will be authenticating with the same identity.

## Key concepts

### CertificateBundle

A Azure Key Vault certificate public key. The private key is never included when retrieving a `CertificateBundle`.

### CertificateClient

The `CertificateClient` provides asynchronous operations for working with Key Vault certificates.

### Thread safety

We guarantee that all client instance methods are thread-safe and independent of each other. This ensures that the recommendation of reusing client instances is always safe, even across threads.

## Examples

> TODO

## Troubleshooting

### General

When you interact with the Azure Key Vault certificates client library using the Rust SDK, errors returned by the service correspond to the same HTTP status codes returned for [REST API] requests.

For example, if you try to retrieve a key that doesn't exist in your Azure Key Vault, a `404` error is returned, indicating `Not Found`.

```rust no_run
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_certificates::CertificateClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DefaultAzureCredential::new()?;
    let client = CertificateClient::new(
        "https://my-vault.vault.azure.net/",
        credential.clone(),
        None,
    )?;

    match client.get_certificate("certificate-name".into(), "".into(), None).await {
        Ok(response) => println!("Certificate: {:#?}", response.into_body().await?.x509_thumbprint),
        Err(err) => println!("Error: {:#?}", err.into_inner()?),
    }

    Ok(())
}
```

You will notice that additional information is logged, like the Client Request ID of the operation.

```text
Error: HttpError {
    status: NotFound,
    details: ErrorDetails {
        code: Some(
            "CertificateNotFound",
        ),
        message: Some(
            "A certificate with (name/id) certificate-name was not found in this key vault. If you recently deleted this certificate you may be able to recover it using the correct recovery command. For help resolving this issue, please see https://go.microsoft.com/fwlink/?linkid=2125182",
        ),
    },
    ..
}
```

## Contributing

See the [CONTRIBUTING.md] for details on building, testing, and contributing to these libraries.

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit <https://opensource.microsoft.com/cla/>.

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct]. For more information see the [Code of Conduct FAQ] or contact <opencode@microsoft.com> with any additional questions or comments.

<!-- LINKS -->
[API reference documentation]: https://docs.rs/azure_security_keyvault_certificates/latest/azure_security_keyvault_certificates
[Azure CLI]: https://learn.microsoft.com/cli/azure
[Azure subscription]: https://azure.microsoft.com/free/
[Azure Identity]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity
[Microsoft Open Source Code of Conduct]: https://opensource.microsoft.com/codeofconduct/
[Product documentation]: https://learn.microsoft.com/azure/key-vault/
[REST API]: https://learn.microsoft.com/rest/api/keyvault/
[Cargo]: https://crates.io/
[Package (crates.io)]: https://crates.io/crates/azure_security_keyvault_certificates
[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/keyvault/azure_security_keyvault_certificates/src
[CONTRIBUTING.md]: https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md
[Code of Conduct FAQ]: https://opensource.microsoft.com/codeofconduct/faq/
[access control]: https://learn.microsoft.com/azure/key-vault/general/assign-access-policy
[RBAC]: https://learn.microsoft.com/azure/key-vault/general/rbac-guide
