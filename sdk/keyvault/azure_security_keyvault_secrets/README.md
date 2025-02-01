# Azure Key Vault Secret client library for Rust

Azure Key Vault is a cloud service that provides a secure storage of secrets, such as passwords and database connection strings.

The Azure Key Vault Secrets client library allows you to securely store and control the access to tokens, passwords, API keys, and other secrets. This library offers operations to create, retrieve, update, delete, purge, backup, restore, and list the secrets and its versions.

[Source code] | [Package (crates.io)] | [API reference documentation] | [Product documentation]

## Getting started

### Install the package

Install the Azure Key Vault Secrets client library for Rust with Cargo:

```sh
cargo add azure_security_keyvault_secrets
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
cargo add azure_identity azure_core serde_json tokio
```

### Authenticate the client

In order to interact with the Azure Key Vault service, you'll need to create an instance of the `SecretClient` struct. You need a **vault url**, which you may see as "DNS Name" in the portal, and credentials to instantiate a client object.

The example shown below use a `DefaultAzureCredential`, which is appropriate for most scenarios including local development and production environments. Additionally, we recommend using a managed identity for authentication in production environments. You can find more information on different ways of authenticating and their corresponding credential types in the [Azure Identity] documentation.

The `DefaultAzureCredential` will automatically pick up on an Azure CLI authentication. Ensure you are logged in with the Azure CLI:

```azurecli
az login
```

Instantiate a `DefaultAzureCredential` to pass to the client. The same instance of a token credential can be used with multiple clients if they will be authenticating with the same identity.

### Set and Get a Secret

```rust
use azure_core::{RequestContent, Response};
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::{
    models::{SecretBundle, SecretSetParameters},
    SecretClient,
};
use serde_json::to_vec;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new secret client
    let credential = DefaultAzureCredential::new()?;
    let client = SecretClient::new(
        "https://your-key-vault-name.vault.azure.net/",
        credential,
        None,
    )?;

    // Create a new secret using the secret client.
    let mut secret_set_parameters = SecretSetParameters::default();
    secret_set_parameters.value = Some("secret-value".to_string());

    // Serialize secret_set_parameters to Vec<u8>
    let secret_set_parameters_bytes: Vec<u8> = to_vec(&secret_set_parameters)?;

    let secret: Response<SecretBundle> = client
        .set_secret(
            "secret-name".to_string(),
            RequestContent::from(secret_set_parameters_bytes),
            None,
        )
        .await?;
    println!("Response Code: {:?}", secret.status());

    // Retrieve a secret using the secret client.
    let secret: SecretBundle = client
        .get_secret(
            "secret-name".to_string(),
            "secret-version".to_string(),
            None,
        )
        .await?
        .into_body()
        .await?;
    println!("{:?}", secret.value);

    Ok(())
}
```

## Key concepts

### KeyVaultSecret

A `KeyVaultSecret` is the fundamental resource within Azure Key Vault. From a developer's perspective, Azure Key Vault APIs accept and return secret values as strings.

### SecretClient

A `SecretClient` asynchronous operations in the SDK allowing for selection of a client based on an application's use case. Once you've initialized a `SecretClient`, you can interact with secrets in Azure Key Vault.

### Thread safety

We guarantee that all client instance methods are thread-safe and independent of each other. This ensures that the recommendation of reusing client instances is always safe, even across threads.

## Examples

The following section provides several code snippets using the `SecretClient`, covering some of the most common Azure Key Vault Secrets service related tasks:

* [Create a secret](#create-a-secret)
* [Retrieve a secret](#retrieve-a-secret)
* [Update an existing secret](#update-an-existing-secret)
* [Delete a secret](#delete-a-secret)
* [Delete and purge a secret](#delete-and-purge-a-secret)
* [List Secrets](#list-secrets)

### Create a secret

`set_secret` creates a `KeyVaultSecret` to be stored in the Azure Key Vault. If a secret with the same name already exists, then a new version of the secret is created.

```rust
use azure_core::{RequestContent, Response};
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::{
    models::{SecretBundle, SecretSetParameters},
    SecretClient,
};
use serde_json::to_vec;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DefaultAzureCredential::new()?;
    let client = SecretClient::new(
        "https://your-key-vault-name.vault.azure.net/",
        credential,
        None,
    )?;

    // Create a new secret using the secret client.
    let mut secret_set_parameters = SecretSetParameters::default();
    secret_set_parameters.value = Some("secret-value".to_string());

    // Serialize secret_set_parameters to Vec<u8>
    let secret_set_parameters_bytes: Vec<u8> = to_vec(&secret_set_parameters)?;

    let response: Response<SecretBundle> = client
        .set_secret(
            "secret-name".to_string(),
            RequestContent::from(secret_set_parameters_bytes),
            None,
        )
        .await?;
    println!("Response Code: {:?}", secret.status());

    Ok(())
}
```

### Retrieve a secret

`get_secret` retrieves a secret previously stored in the Azure Key Vault. Setting the `secret-version` to an empty string will return the latest version.

```rust
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::{
    models::SecretBundle,
    SecretClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DefaultAzureCredential::new()?;
    let client = SecretClient::new(
        "https://your-key-vault-name.vault.azure.net/",
        credential,
        None,
    )?;

    // Retrieve a secret using the secret client.
    let secret: SecretBundle = client
        .get_secret(
            "secret-name".to_string(),
            "secret-version".to_string(),
            None,
        )
        .await?
        .into_body()
        .await?;
    println!("{:?}", secret.value);

    Ok(())
}
```

### Update an existing secret

`update_secret` updates a secret previously stored in the Azure Key Vault. Only the attributes of the secret are updated. To update the value, call `SecretClient::set_secret` on a secret with the same name.

```rust
use azure_core::{RequestContent, Response};
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::{models::{SecretBundle, SecretUpdateParameters}, SecretClient};
use serde_json::to_vec;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DefaultAzureCredential::new()?;
    let client = SecretClient::new(
        "https://your-key-vault-name.vault.azure.net/",
        credential,
        None,
    )?;

    // Update a secret using the secret client.
    let mut secret_update_parameters = SecretUpdateParameters::default();
    if let Some(tags) = &mut secret_update_parameters.tags {
        tags.insert("foo".to_string(), "buzz".to_string());
    }

    // Serialize secret_update_parameters to Vec<u8>
    let secret_update_parameters_bytes: Vec<u8> = to_vec(&secret_update_parameters)?;

    let response: Response<SecretBundle> = client
        .update_secret(
            "secret-name".to_string(),
            "secret-version".to_string(),
            RequestContent::from(secret_update_parameters_bytes),
            None,
        )
        .await?;
    println!("Response Code: {:?}", response.status());

    Ok(())
}
```

### Delete a secret

`delete_secret` starts a long-running operation to delete a secret previously stored in the Azure Key Vault. You can retrieve the secret immediately without waiting for the operation to complete. When [soft-delete] is not enabled for the Azure Key Vault, this operation permanently deletes the secret.

```rust
use azure_core::Response;
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::{models::DeletedSecretBundle, SecretClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DefaultAzureCredential::new()?;
    let client = SecretClient::new(
        "https://your-key-vault-name.vault.azure.net/",
        credential,
        None,
    )?;

    // Delete a secret using the secret client.
    let response: Response<DeletedSecretBundle> = client
        .delete_secret(
            "secret-name".to_string(),
            None,
        ).await?;
    println!("Response Code: {:?}", response.status());

    Ok(())
}
```

### Delete and purge a secret

You will need to wait for the long-running operation to complete before trying to purge or recover the secret. You can do this by calling `update_status` in a loop as shown below:

```rust
use azure_core::Response;
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::{models::DeletedSecretBundle, SecretClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DefaultAzureCredential::new()?;
    let client = SecretClient::new(
        "https://your-key-vault-name.vault.azure.net/",
        credential,
        None,
    )?;

    // Delete a secret using the secret client.
    let response: Response<DeletedSecretBundle> = client
        .delete_secret("secret-name".to_string(), None)
        .await?;
    println!("Delete Response Code: {:?}", response.status());

    // Purge deleted secret using the secret client.
    let response: Response<()> = client
        .purge_deleted_secret("secret-name".to_string(), None)
        .await?;
    println!("Purge Response Code: {:?}", response.status());

    Ok(())
}

```

### List secrets

This example lists all the secrets in the specified Azure Key Vault. The value is not returned when listing all secrets. You will need to call `SecretClient::get_secret` to retrieve the value.

```rust
use azure_core::Pager;
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::{models::SecretListResult, SecretClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DefaultAzureCredential::new()?;
    let client = SecretClient::new(
        "https://your-key-vault-name.vault.azure.net/",
        credential,
        None,
    )?;

    // List secrets using the secret client.
    let response: Pager<SecretListResult>= client.get_secrets(None)?;
    println!("{:?}", response);

    Ok(())
}
```

## Troubleshooting

### General

When you interact with the Azure Key Vault Secrets client library using the Rust SDK, errors returned by the service correspond to the same HTTP status codes returned for [REST API] requests.

For example, if you try to retrieve a secret that doesn't exist in your Azure Key Vault, a `404` error is returned, indicating `Not Found`.

```rust
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::SecretClient;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DefaultAzureCredential::new()?;
    let client = SecretClient::new(
        "https://ronnieg-keyvault.vault.azure.net/",
        credential,
        None,
    )?;

    match client
        .get_secret(
            "secret-name".to_string(),
            "".to_string(),
            None,
        )
        .await
    {
        Ok(response) => println!("Secret Value: {:?}", response.into_body().await?.value),
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
            "SecretNotFound",
        ),
        message: Some(
            "A secret with (name/id) secret-name1 was not found in this key vault. If you recently deleted this secret you may be able to recover it using the correct recovery command. For help resolving this issue, please see https://go.microsoft.com/fwlink/?linkid=2125182",
        ),
    },
    ..
}
```

## Next steps

Several Azure Key Vault Secrets client library samples are available to you in this GitHub repository. These samples provide example code for additional scenarios commonly encountered while working with Azure Key Vault:

* [Sample1_HelloWorld.md] - for working with Azure Key Vault, including:
  * Create a secret
  * Get an existing secret
  * Update an existing secret
  * Delete secret

* [Sample2_BackupAndRestore.md] - contains the code snippets working with Azure Key Vault Secrets, including:
  * Backup and recover a secret

* [Sample3_GetSecrets.md] - example code for working with Azure Key Vault Secrets, including:
  * Create secrets
  * List all secrets in the Key Vault
  * Update secrets in the Key Vault
  * List versions of a specified secret
  * Delete secrets from the Key Vault
  * List deleted secrets in the Key Vault

### Additional Documentation

* For more extensive documentation on Azure Key Vault, see the [API reference documentation].
* For Keys client library see [Keys client library].
* For Certificates client library see [Certificates client library].

## Contributing

See the [CONTRIBUTING.md] for details on building, testing, and contributing to these libraries.

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit <https://cla.microsoft.com>.

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct]. For more information see the [Code of Conduct FAQ] or contact <opencode@microsoft.com> with any additional questions or comments.

<!-- LINKS -->
[API reference documentation]: https://docs.rs/azure_security_keyvault_secrets/latest/azure_security_keyvault_secrets
[Azure CLI]: https://learn.microsoft.com/cli/azure
[Azure subscription]: https://azure.microsoft.com/free/rust/
[Azure Identity]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity
[backup_and_restore_sample]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/keyvault/azure_security_keyvault_secrets/samples/Sample2_BackupAndRestore.md
[Certificates client library]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/keyvault/azure_security_keyvault_certificates
[Microsoft Open Source Code of Conduct]: https://opensource.microsoft.com/codeofconduct/
[get_secrets_sample]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/keyvault/azure_security_keyvault_secrets/samples/Sample3_GetSecrets.md
[hello_world_sample]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/keyvault/azure_security_keyvault_secrets/samples/Sample1_HelloWorld.md
[Keys client library]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/keyvault/azure_security_keyvault_keys
[Product documentation]: https://learn.microsoft.com/azure/key-vault/
[REST API]: https://learn.microsoft.com/rest/api/keyvault/
[Cargo]: https://crates.io/
[Package (crates.io)]: https://crates.io/crates/azure_security_keyvault_secrets
[Samples]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/keyvault/azure_security_keyvault_secrets/samples
[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/keyvault/azure_security_keyvault_secrets/src
[soft_delete]: https://learn.microsoft.com/azure/key-vault/general/soft-delete-overview
[DefaultAzureCredential]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/identity/azure_identity/src/default_azure_credential.rs
[CONTRIBUTING.md]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/keyvault/CONTRIBUTING.md
[Code of Conduct FAQ]: https://opensource.microsoft.com/codeofconduct/faq/
[Migration guide]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/keyvault/azure_security_keyvault_secrets/MigrationGuide.md
[access control]: https://learn.microsoft.com/azure/key-vault/general/assign-access-policy
[RBAC]: https://learn.microsoft.com/azure/key-vault/general/rbac-guide
[troubleshooting guide]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/keyvault/azure_security_keyvault_secrets/TROUBLESHOOTING.md

![Impressions](https://azure-sdk-impressions.azurewebsites.net/api/impressions/azure-sdk-for-rust%2Fsdk%2Fkeyvault%2Fazure_security_keyvault_secrets%2FREADME.png)
