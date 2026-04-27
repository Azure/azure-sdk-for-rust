# Azure Key Vault keys client library for Rust

Azure Key Vault is a cloud service that provides secure storage of keys for encrypting your data. Multiple keys, and multiple versions of the same key, can be kept in the Azure Key Vault. Cryptographic keys in Azure Key Vault are represented as [JSON Web Key (JWK)](https://tools.ietf.org/html/rfc7517) objects.

Azure Key Vault Managed HSM is a fully-managed, highly-available, single-tenant, standards-compliant cloud service that enables you to safeguard cryptographic keys for your cloud applications using FIPS 140-2 Level 3 validated HSMs.

The Azure Key Vault keys library client supports RSA keys and Elliptic Curve (EC) keys, each with corresponding support in hardware security modules (HSM). It offers operations to create, retrieve, update, delete, purge, backup, restore, and list the keys and its versions.

[Source code] | [Package (crates.io)] | [API reference documentation] | [Product documentation]

## Getting started

### Install the package

Install the Azure Key Vault keys client library for Rust with [Cargo]:

```sh
cargo add azure_security_keyvault_keys
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

In order to interact with the Azure Key Vault service, you'll need to create an instance of the `KeyClient`. You need a **vault url**, which you may see as "DNS Name" in the portal, and credentials to instantiate a client object.

The example shown below uses a `DeveloperToolsCredential`, which is appropriate for local development environments. We recommend using a managed identity for authentication in production environments. You can find more information on different ways of authenticating and their corresponding credential types in the [Azure Identity] documentation.

The `DeveloperToolsCredential` will automatically pick up on an Azure CLI authentication. Ensure you are logged in with the Azure CLI:

```azurecli
az login
```

Instantiate a `DeveloperToolsCredential` to pass to the client. The same instance of a token credential can be used with multiple clients if they will be authenticating with the same identity.

#### Activate your Managed HSM

This section only applies if you are creating a Managed HSM. All data plane commands are disabled until the HSM is activated. You will not be able to create keys or assign roles. Only the designated administrators that were assigned during the create command can activate the HSM. To activate the HSM you must download the security domain.

To activate your HSM you need:

* Minimum 3 RSA key-pairs (maximum 10)
* Specify minimum number of keys required to decrypt the security domain (quorum)

To activate the HSM you send at least 3 (maximum 10) RSA public keys to the HSM. The HSM encrypts the security domain with these keys and sends it back. Once this security domain is successfully downloaded, your HSM is ready to use. You also need to specify quorum, which is the minimum number of private keys required to decrypt the security domain.

The example below shows how to use openssl to generate 3 self-signed certificate.

```sh
openssl req -newkey rsa:2048 -nodes -keyout cert_0.key -x509 -days 365 -out cert_0.cer
openssl req -newkey rsa:2048 -nodes -keyout cert_1.key -x509 -days 365 -out cert_1.cer
openssl req -newkey rsa:2048 -nodes -keyout cert_2.key -x509 -days 365 -out cert_2.cer
```

Use the `az keyvault security-domain download` command to download the security domain and activate your managed HSM. The example below uses 3 RSA key pairs (only public keys are needed for this command) and sets the quorum to 2.

```azurecli
az keyvault security-domain download --hsm-name <your-key-vault-name> --sd-wrapping-keys ./certs/cert_0.cer ./certs/cert_1.cer ./certs/cert_2.cer --sd-quorum 2 --security-domain-file ContosoHSM-SD.json
```

### Instantiate a client

```rust no_run
use azure_identity::DeveloperToolsCredential;
use azure_security_keyvault_keys::KeyClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new key client
    let credential = DeveloperToolsCredential::new(None)?;
    let client = KeyClient::new(
        "https://<your-key-vault-name>.vault.azure.net/",
        credential.clone(),
        None,
    )?;

    // Get a key using the key client.
    let key = client
        .get_key("key-name", None)
        .await?
        .into_model()?;
    println!("JWT: {:?}", key.key);

    Ok(())
}
```

## Key concepts

### Key

Azure Key Vault supports multiple key types and algorithms, and enables the use of hardware security modules (HSM) for high value keys.

### KeyClient

The `KeyClient` provides asynchronous operations for working with Key Vault keys.

### Thread safety

We guarantee that all client instance methods are thread-safe and independent of each other. This ensures that the recommendation of reusing client instances is always safe, even across threads.

## Examples

The following section provides several code snippets using a `KeyClient` like we [instantiated above](#instantiate-a-client):

* [Create a key](#create-a-key)
* [Retrieve a key](#retrieve-a-key)
* [Update an existing key](#update-an-existing-key)
* [Delete a key](#delete-a-key)
* [List keys](#list-keys)
* [Encrypt and decrypt](#encrypt-and-decrypt)

### Create a key

`create_key` creates a Key Vault key to be stored in the Azure Key Vault. If a key with the same name already exists, then a new version of the key is created.

```rust ignore create_key
use azure_security_keyvault_keys::{
    models::{CreateKeyParameters, CurveName, KeyType},
    ResourceExt,
};

// Create an EC key.
let body = CreateKeyParameters {
    kty: Some(KeyType::Ec),
    curve: Some(CurveName::P256),
    ..Default::default()
};

let key = client
    .create_key("key-name", body.try_into()?, None)
    .await?
    .into_model()?;

println!(
    "Key Name: {:?}, Type: {:?}, Version: {:?}",
    key.resource_id()?.name,
    key.key.as_ref().map(|k| k.kty.as_ref()),
    key.resource_id()?.version,
);
```

### Retrieve a key

`get_key` retrieves a public key (or only metadata for symmetric keys) previously stored in the Azure Key Vault. Setting the `key-version` to an empty string will return the latest version.

```rust ignore get_key
use azure_security_keyvault_keys::models::KeyClientGetKeyOptions;

// Retrieve a public key as a JWK using the key client.
let key_options = KeyClientGetKeyOptions {
    key_version: Some("key-version".to_string()),
    ..Default::default()
};
let key = client
    .get_key("key-name", None)
    .await?
    .into_model()?;

println!("Key: {:#?}", key.key);
```

### Update an existing key

`update_key_properties` updates a key previously stored in the Azure Key Vault. Only the attributes of the key are updated. To update the value, call `KeyClient::create_key` on a key with the same name.

```rust ignore update_key
use azure_security_keyvault_keys::models::UpdateKeyPropertiesParameters;
use std::collections::HashMap;

// Update a key using the key client.
let key_update_parameters = UpdateKeyPropertiesParameters {
    tags: Some(HashMap::from_iter(vec![("tag-name".into(), "tag-value".into())])),
    ..Default::default()
};

client
    .update_key_properties("key-name", key_update_parameters.try_into()?, None)
    .await?
    .into_model()?;
```

### Delete a key

`delete_key` will tell Key Vault to delete a key but it is not deleted immediately. It will not be deleted until the service-configured data retention period - the default is 90 days - or until you call `purge_key` on the returned `DeletedKey.id`.

```rust ignore delete_key
// Delete a key using the key client.
client.delete_key("key-name", None).await?;
```

### List keys

This example lists all the keys in the specified Azure Key Vault.

```rust ignore list_keys
use azure_security_keyvault_keys::ResourceExt;
use futures::TryStreamExt;

let mut pager = client.list_key_properties(None)?.into_stream();
while let Some(key) = pager.try_next().await? {
    // Get the key name from the ID.
    let name = key.resource_id()?.name;
    println!("Found Key with Name: {}", name);
}
```

### Encrypt and decrypt

You can create an asymmetric key in Azure Key Vault (Managed HSM also supports AES symmetric key encryption) and encrypt or decrypt data
without the private key ever leaving the HSM.

```rust ignore encrypt_decrypt
use azure_security_keyvault_keys::{
    models::{
        CreateKeyParameters, EncryptionAlgorithm, KeyOperationParameters, KeyType,
    },
    ResourceExt,
};
use rand::random;

// Create a key encryption key (KEK) using RSA.
let body = CreateKeyParameters {
    kty: Some(KeyType::Rsa),
    key_size: Some(2048),
    ..Default::default()
};

let key = client
    .create_key("key-name", body.try_into()?, None)
    .await?
    .into_model()?;
let key_version = key.resource_id()?.version.expect("key version required");

// Generate a symmetric data encryption key (DEK). You'd encrypt your data using this DEK.
let dek = random::<u32>().to_le_bytes().to_vec();

// Wrap the DEK. You'd store the wrapped DEK along with your encrypted data.
let mut parameters = KeyOperationParameters {
    algorithm: Some(EncryptionAlgorithm::RsaOaep256),
    value: Some(dek.clone()),
    ..Default::default()
};
let wrapped = client
    .wrap_key("key-name", &key_version, parameters.clone().try_into()?, None)
    .await?
    .into_model()?;

assert!(matches!(wrapped.result.as_ref(), Some(result) if !result.is_empty()));

// Unwrap the DEK.
parameters.value = wrapped.result;
let unwrapped = client
    .unwrap_key("key-name", &key_version, parameters.try_into()?, None)
    .await?
    .into_model()?;

assert!(matches!(unwrapped.result, Some(result) if result.eq(&dek)));
```

## Troubleshooting

### General

When you interact with the Azure Key Vault keys client library using the Rust SDK, errors returned by the service correspond to the same HTTP status codes returned for [REST API] requests.

For example, if you try to retrieve a key that doesn't exist in your Azure Key Vault, a `404` error is returned, indicating `Not Found`.

```rust ignore errors
match client.get_key("key-name".into(), None).await {
    Ok(response) => println!("Key: {:#?}", response.into_model()?.key),
    Err(err) => println!("Error: {:#?}", err.into_inner()?),
}
```

You will notice that additional information is logged, like the Client Request ID of the operation.

```text
Error: ErrorResponse {
    error: ErrorDetails {
        code: Some(
            "KeyNotFound",
        ),
        message: Some(
            "A key with (name/id) key-name was not found in this key vault. If you recently deleted this key you may be able to recover it using the correct recovery command. For help resolving this issue, please see https://go.microsoft.com/fwlink/?linkid=2125182",
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
[API reference documentation]: https://docs.rs/azure_security_keyvault_keys/latest/azure_security_keyvault_keys
[Azure CLI]: https://learn.microsoft.com/cli/azure
[Azure subscription]: https://azure.microsoft.com/free/
[Azure Identity]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity
[Microsoft Open Source Code of Conduct]: https://opensource.microsoft.com/codeofconduct/
[Product documentation]: https://learn.microsoft.com/azure/key-vault/
[REST API]: https://learn.microsoft.com/rest/api/keyvault/
[Cargo]: https://crates.io/
[Package (crates.io)]: https://crates.io/crates/azure_security_keyvault_keys
[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/keyvault/azure_security_keyvault_keys/src
[CONTRIBUTING.md]: https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md
[Code of Conduct FAQ]: https://opensource.microsoft.com/codeofconduct/faq/
[access control]: https://learn.microsoft.com/azure/key-vault/general/assign-access-policy
[RBAC]: https://learn.microsoft.com/azure/key-vault/general/rbac-guide
