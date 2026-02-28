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

The example shown below uses a `DeveloperToolsCredential`, which is appropriate for local development environments. We recommend using a managed identity for authentication in production environments. You can find more information on different ways of authenticating and their corresponding credential types in the [Azure Identity] documentation.

The `DeveloperToolsCredential` will automatically pick up on an Azure CLI authentication. Ensure you are logged in with the Azure CLI:

```azurecli
az login
```

Instantiate a `DeveloperToolsCredential` to pass to the client. The same instance of a token credential can be used with multiple clients if they will be authenticating with the same identity.

### Instantiate a client

```rust no_run
use azure_core::base64;
use azure_identity::DeveloperToolsCredential;
use azure_security_keyvault_certificates::CertificateClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new certificate client
    let credential = DeveloperToolsCredential::new(None)?;
    let client = CertificateClient::new(
        "https://<your-key-vault-name>.vault.azure.net/",
        credential.clone(),
        None,
    )?;

    // Get a certificate using the certificate client.
    let certificate = client
        .get_certificate("certificate-name", None)
        .await?
        .into_model()?;
    println!(
        "Thumbprint: {:?}",
        certificate.x509_thumbprint.map(base64::encode_url_safe)
    );

    Ok(())
}
```

## Key concepts

### Certificate

A Azure Key Vault certificate public key. The private key is never included when retrieving a `Certificate`.

### CertificateClient

The `CertificateClient` provides asynchronous operations for working with Key Vault certificates.

### Thread safety

We guarantee that all client instance methods are thread-safe and independent of each other. This ensures that the recommendation of reusing client instances is always safe, even across threads.

## Examples

The following section provides several code snippets using a `CertificateClient` like we [instantiated above](#instantiate-a-client):

* [Create a certificate](#create-a-certificate)
* [Retrieve a certificate](#retrieve-a-certificate)
* [Update an existing certificate](#update-an-existing-certificate)
* [Delete a certificate](#delete-a-certificate)
* [List certificates](#list-certificates)
* [Key operations using certificates](#key-operations-using-certificates)

### Create a certificate

`create_certificate` creates a Key Vault certificate to be stored in the Azure Key Vault. If a certificate with the same name already exists, then a new version of the certificate is created.
Before we can create a new certificate, though, we need to define a certificate policy. This is used for the first certificate version and all subsequent versions of that certificate until changed.

`create_certificate` returns a `Poller<CertificateOperation>`, which implements both `std::future::IntoFuture` and `futures::Stream`.
You can `await` the `Poller` to get the final result - a `Certificate` - or asynchronously iterate over each status update.

```rust ignore create_certificate
use azure_security_keyvault_certificates::models::{
    CertificatePolicy, CreateCertificateParameters, IssuerParameters, X509CertificateProperties,
};

// Create a self-signed certificate.
let policy = CertificatePolicy {
    x509_certificate_properties: Some(X509CertificateProperties {
        subject: Some("CN=DefaultPolicy".into()),
        ..Default::default()
    }),
    issuer_parameters: Some(IssuerParameters {
        name: Some("Self".into()),
        ..Default::default()
    }),
    ..Default::default()
};
let body = CreateCertificateParameters {
    certificate_policy: Some(policy),
    ..Default::default()
};

// Wait for the certificate operation to complete.
// The Poller implements futures::Stream and automatically waits between polls.
let certificate = client
    .create_certificate("certificate-name", body.try_into()?, None)?
    .await?
    .into_model()?;
```

### Retrieve a certificate

`get_certificate` retrieves a certificate that was created or even still in progress in Key Vault.
Setting the `certificate-version` to an empty string will return the latest version.

```rust ignore get_certificate
use azure_core::base64;
use azure_security_keyvault_certificates::models::CertificateClientGetCertificateOptions;

let get_options = CertificateClientGetCertificateOptions{
    certificate_version: Some("certificate-version".to_string()),
    ..Default::default()
};
let certificate = client
    .get_certificate("certificate-name", Some(get_options))
    .await?
    .into_model()?;

println!(
    "Certificate thumbprint: {:?}",
    certificate.x509_thumbprint.map(base64::encode)
);
```

### Update an existing certificate

`update_certificate_properties` updates a certificate previously stored in the Azure Key Vault.
Only the attributes of the certificate are updated. To regenerate the certificate, call `CertificateClient::create_certificate` on a certificate with the same name.

```rust ignore update_certificate
use azure_security_keyvault_certificates::models::UpdateCertificatePropertiesParameters;
use std::collections::HashMap;

// Update a certificate using the certificate client.
let certificate_update_parameters = UpdateCertificatePropertiesParameters {
    tags: Some(HashMap::from_iter(vec![("tag-name".into(), "tag-value".into())])),
    ..Default::default()
};

client
    .update_certificate_properties(
        "certificate-name",
        certificate_update_parameters.try_into()?,
        None,
    )
    .await?
    .into_model()?;
```

### Delete a certificate

`delete_certificate` will tell Key Vault to delete a certificate but it is not deleted immediately.
It will not be deleted until the service-configured data retention period - the default is 90 days - or until you call `purge_certificate` on the returned `DeletedCertificate.id`.

```rust ignore delete_certificate
// Delete a certificate using the certificate client.
client.delete_certificate("certificate-name", None).await?;
```

### List certificates

This example lists all the certificates in the specified Azure Key Vault.

```rust ignore list_certificates
use azure_security_keyvault_certificates::ResourceExt;
use futures::TryStreamExt;

let mut pager = client.list_certificate_properties(None)?.into_stream();
while let Some(certificate) = pager.try_next().await? {
    // Get the certificate name from the ID.
    let name = certificate.resource_id()?.name;
    println!("Found Certificate with Name: {}", name);
}
```

### Key operations using certificates

You can use a `KeyClient` to perform key operations on a certificate created with a `CertificateClient`.
The following example shows how to sign data using an EC certificate key.

```rust ignore key_operations
use azure_core::base64;
use azure_security_keyvault_certificates::{
    models::{
        CertificatePolicy, CreateCertificateParameters, CurveName, IssuerParameters,
        KeyProperties, KeyType, KeyUsageType, X509CertificateProperties,
    },
    ResourceExt,
};
use azure_security_keyvault_keys::{
    models::{SignParameters, SignatureAlgorithm},
};
use openssl::sha::sha256;

let plaintext = "plaintext";

// Create an EC certificate policy for signing.
let policy = CertificatePolicy {
    x509_certificate_properties: Some(X509CertificateProperties {
        subject: Some("CN=DefaultPolicy".into()),
        key_usage: Some(vec![KeyUsageType::DigitalSignature]),
        ..Default::default()
    }),
    issuer_parameters: Some(IssuerParameters {
        name: Some("Self".into()),
        ..Default::default()
    }),
    key_properties: Some(KeyProperties {
        key_type: Some(KeyType::Ec),
        curve: Some(CurveName::P256),
        ..Default::default()
    }),
    ..Default::default()
};

// Create a self-signed certificate.
let body = CreateCertificateParameters {
    certificate_policy: Some(policy),
    ..Default::default()
};

// Wait for the certificate operation to complete.
let certificate = client
    .create_certificate("ec-signing-certificate", body.try_into()?, None)?
    .await?
    .into_model()?;
let certificate_version = certificate
    .resource_id()?
    .version
    .expect("certificate version required");

// Hash the plaintext to be signed.
let digest = sha256(plaintext.as_bytes()).to_vec();

// Use a KeyClient using the certificate to sign the digest.
let body = SignParameters {
    algorithm: Some(SignatureAlgorithm::Es256),
    value: Some(digest),
};

let signature = key_client
    .sign("ec-signing-certificate", &certificate_version, body.try_into()?, None)
    .await?
    .into_model()?;

if let Some(signature) = signature.result.map(base64::encode_url_safe) {
    println!("Signature: {}", signature);
}
```

## Troubleshooting

### General

When you interact with the Azure Key Vault certificates client library using the Rust SDK, errors returned by the service correspond to the same HTTP status codes returned for [REST API] requests.

For example, if you try to retrieve a key that doesn't exist in your Azure Key Vault, a `404` error is returned, indicating `Not Found`.

```rust ignore errors
match client.get_certificate("certificate-name".into(), None).await {
    Ok(response) => println!("Certificate: {:#?}", response.into_model()?.x509_thumbprint),
    Err(err) => println!("Error: {:#?}", err.into_inner()?),
}
```

You will notice that additional information is logged, like the Client Request ID of the operation.

```text
Error: ErrorResponse {
    error: ErrorDetails {
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
