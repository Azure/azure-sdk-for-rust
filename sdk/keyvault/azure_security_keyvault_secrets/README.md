# Azure Key Vault secrets client library for Rust

Azure Key Vault is a cloud service that provides a secure storage of secrets, such as passwords and database connection strings.

The Azure Key Vault secrets client library allows you to securely store and control the access to tokens, passwords, API keys, and other secrets. This library offers operations to create, retrieve, update, delete, purge, backup, restore, and list the secrets and its versions.

[Source code] | [Package (crates.io)] | [API reference documentation] | [Product documentation]

## Getting started

### Install the package

Install the Azure Key Vault secrets client library for Rust with [Cargo]:

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
cargo add azure_identity azure_core tokio
```

### Authenticate the client

In order to interact with the Azure Key Vault service, you'll need to create an instance of the `SecretClient`. You need a **vault url**, which you may see as "DNS Name" in the portal, and credentials to instantiate a client object.

The example shown below use a `DefaultAzureCredential`, which is appropriate for most scenarios including local development and production environments. Additionally, we recommend using a managed identity for authentication in production environments. You can find more information on different ways of authenticating and their corresponding credential types in the [Azure Identity] documentation.

The `DefaultAzureCredential` will automatically pick up on an Azure CLI authentication. Ensure you are logged in with the Azure CLI:

```azurecli
az login
```

Instantiate a `DefaultAzureCredential` to pass to the client. The same instance of a token credential can be used with multiple clients if they will be authenticating with the same identity.

### Set and Get a Secret

```rust no_run
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::{
    models::{SecretBundle, SecretSetParameters},
    ResourceExt, SecretClient,
};

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
    let secret_set_parameters = SecretSetParameters {
        value: Some("secret-value".into()),
        ..Default::default()
    };

    let secret: SecretBundle = client
        .set_secret(
            "secret-name".into(),
            secret_set_parameters.try_into()?,
            None,
        )
        .await?
        .into_body()
        .await?;

    // Get version of created secret.
    let version = secret.resource_id()?.version.unwrap_or_default();

    // Retrieve a secret using the secret client.
    let secret: SecretBundle = client
        .get_secret("secret-name".into(), version, None)
        .await?
        .into_body()
        .await?;
    println!("{:?}", secret.value);

    Ok(())
}
```

## Key concepts

### SecretBundle

A `SecretBundle` is the fundamental resource within Azure Key Vault. From a developer's perspective, Azure Key Vault APIs accept and return secret values as strings.

### SecretClient

The `SecretClient` provides asynchronous operations for working with Key Vault secrets.

### Thread safety

We guarantee that all client instance methods are thread-safe and independent of each other. This ensures that the recommendation of reusing client instances is always safe, even across threads.

## Examples

The following section provides several code snippets using the `SecretClient`, covering some of the most common Azure Key Vault secrets service related tasks:

* [Create a secret](#create-a-secret)
* [Retrieve a secret](#retrieve-a-secret)
* [Update an existing secret](#update-an-existing-secret)
* [Delete a secret](#delete-a-secret)
* [Delete and purge a secret](#delete-and-purge-a-secret)
* [List secrets](#list-secrets)

### Create a secret

`set_secret` creates a Key Vault secret to be stored in the Azure Key Vault. If a secret with the same name already exists, then a new version of the secret is created.

```rust no_run
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::{
    models::{SecretBundle, SecretSetParameters},
    ResourceExt, SecretClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DefaultAzureCredential::new()?;
    let client = SecretClient::new(
        "https://your-key-vault-name.vault.azure.net/",
        credential,
        None,
    )?;

    // Create a new secret using the secret client.
    let secret_set_parameters = SecretSetParameters {
        value: Some("secret-value".into()),
        ..Default::default()
    };

    let secret: SecretBundle = client
        .set_secret(
            "secret-name".into(),
            secret_set_parameters.try_into()?,
            None,
        )
        .await?
        .into_body()
        .await?;

    println!(
        "Secret Name: {:?}, Value: {:?}, Version: {:?}",
        secret.resource_id()?.name,
        secret.value,
        secret.resource_id()?.version
    );

    Ok(())
}
```

### Retrieve a secret

`get_secret` retrieves a secret previously stored in the Azure Key Vault. Setting the `secret-version` to an empty string will return the latest version.

```rust no_run
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::{models::SecretBundle, SecretClient};

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
        .get_secret("secret-name".into(), "secret-version".into(), None)
        .await?
        .into_body()
        .await?;

    println!("Secret Value: {:?}", secret.value);

    Ok(())
}
```

### Update an existing secret

`update_secret` updates a secret previously stored in the Azure Key Vault. Only the attributes of the secret are updated. To update the value, call `SecretClient::set_secret` on a secret with the same name.

```rust no_run
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::{
    models::{SecretAttributes, SecretUpdateParameters},
    SecretClient,
};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DefaultAzureCredential::new()?;
    let client = SecretClient::new(
        "https://your-key-vault-name.vault.azure.net/",
        credential,
        None,
    )?;

    // Update a secret using the secret client.
    let secret_update_parameters = SecretUpdateParameters {
        content_type: Some("text/plain".into()),
        secret_attributes: Some(SecretAttributes::default()),
        tags: Some(HashMap::from_iter(vec![(
            "tag-name".into(),
            "tag-value".into(),
        )])),
    };

    client
        .update_secret(
            "secret-name".into(),
            "".into(),
            secret_update_parameters.try_into()?,
            None,
        )
        .await?
        .into_body()
        .await?;

    Ok(())
}
```

### Delete a secret

`delete_secret` starts a long-running operation to delete a secret previously stored in the Azure Key Vault. You can retrieve the secret immediately without waiting for the operation to complete. When [soft-delete] is not enabled for the Azure Key Vault, this operation permanently deletes the secret.

```rust no_run
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::SecretClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DefaultAzureCredential::new()?;
    let client = SecretClient::new(
        "https://your-key-vault-name.vault.azure.net/",
        credential,
        None,
    )?;

    // Delete a secret using the secret client.
    client
        .delete_secret("secret-name".into(), None)
        .await?;

    Ok(())
}
```

### Delete and purge a secret

You will need to wait for the long-running operation to complete before trying to purge or recover the secret. You can do this by calling `update_status` in a loop as shown below:

```rust no_run
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::SecretClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DefaultAzureCredential::new()?;
    let client = SecretClient::new(
        "https://your-key-vault-name.vault.azure.net/",
        credential,
        None,
    )?;

    // Delete a secret using the secret client.
    client.delete_secret("secret-name".into(), None).await?;

    // Purge deleted secret using the secret client.
    client
        .purge_deleted_secret("secret-name".into(), None)
        .await?;

    Ok(())
}
```

### List secrets

This example lists all the secrets in the specified Azure Key Vault. The value is not returned when listing all secrets. You will need to call `SecretClient::get_secret` to retrieve the value.

```rust no_run
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

When you interact with the Azure Key Vault secrets client library using the Rust SDK, errors returned by the service correspond to the same HTTP status codes returned for [REST API] requests.

For example, if you try to retrieve a secret that doesn't exist in your Azure Key Vault, a `404` error is returned, indicating `Not Found`.

```rust no_run
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
            "secret-name".into(),
            "".into(),
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
[Certificates client library]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/keyvault/azure_security_keyvault_certificates
[Microsoft Open Source Code of Conduct]: https://opensource.microsoft.com/codeofconduct/
[Keys client library]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/keyvault/azure_security_keyvault_keys
[Product documentation]: https://learn.microsoft.com/azure/key-vault/
[REST API]: https://learn.microsoft.com/rest/api/keyvault/
[Cargo]: https://crates.io/
[Package (crates.io)]: https://crates.io/crates/azure_security_keyvault_secrets
[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/keyvault/azure_security_keyvault_secrets/src
[CONTRIBUTING.md]: https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/keyvault/CONTRIBUTING.md
[Code of Conduct FAQ]: https://opensource.microsoft.com/codeofconduct/faq/
[access control]: https://learn.microsoft.com/azure/key-vault/general/assign-access-policy
[RBAC]: https://learn.microsoft.com/azure/key-vault/general/rbac-guide
