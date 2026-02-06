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
cargo add azure_identity tokio
```

### Authenticate the client

In order to interact with the Azure Key Vault service, you'll need to create an instance of the `SecretClient`. You need a **vault url**, which you may see as "DNS Name" in the portal, and credentials to instantiate a client object.

The example shown below uses a `DeveloperToolsCredential`, which is appropriate for local development environments. We recommend using a managed identity for authentication in production environments. You can find more information on different ways of authenticating and their corresponding credential types in the [Azure Identity] documentation.

The `DeveloperToolsCredential` will automatically pick up on an Azure CLI authentication. Ensure you are logged in with the Azure CLI:

```azurecli
az login
```

Instantiate a `DeveloperToolsCredential` to pass to the client. The same instance of a token credential can be used with multiple clients if they will be authenticating with the same identity.

### Instantiate a client

```rust no_run
use azure_identity::DeveloperToolsCredential;
use azure_security_keyvault_secrets::SecretClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new secret client
    let credential = DeveloperToolsCredential::new(None)?;
    let client = SecretClient::new(
        "https://<your-key-vault-name>.vault.azure.net/",
        credential.clone(),
        None,
    )?;

    // Get a secret using the secret client.
    let secret = client
        .get_secret("secret-name", None)
        .await?
        .into_model()?;
    println!("Secret: {:?}", secret.value);

    Ok(())
}
```

## Key concepts

### Secret

A `Secret` is the fundamental resource within Azure Key Vault. From a developer's perspective, Azure Key Vault APIs accept and return secret values as strings.

### SecretClient

The `SecretClient` provides asynchronous operations for working with Key Vault secrets.

### Thread safety

We guarantee that all client instance methods are thread-safe and independent of each other. This ensures that the recommendation of reusing client instances is always safe, even across threads.

## Examples

The following section provides several code snippets using a `SecretClient` like we [instantiated above](#instantiate-a-client):

* [Create a secret](#create-a-secret)
* [Retrieve a secret](#retrieve-a-secret)
* [Update an existing secret](#update-an-existing-secret)
* [Delete a secret](#delete-a-secret)
* [List secrets](#list-secrets)

### Create a secret

`set_secret` creates a Key Vault secret to be stored in the Azure Key Vault. If a secret with the same name already exists, then a new version of the secret is created.

```rust ignore create_secret
use azure_security_keyvault_secrets::{models::SetSecretParameters, ResourceExt};

// Create a new secret using the secret client.
let secret_set_parameters = SetSecretParameters {
    value: Some("secret-value".into()),
    ..Default::default()
};

let secret = client
    .set_secret("secret-name", secret_set_parameters.try_into()?, None)
    .await?
    .into_model()?;

println!(
    "Secret Name: {:?}, Value: {:?}, Version: {:?}",
    secret.resource_id()?.name,
    secret.value,
    secret.resource_id()?.version
);
```

### Retrieve a secret

`get_secret` retrieves a secret previously stored in the Azure Key Vault. Setting the `secret-version` to an empty string will return the latest version.

```rust ignore get_secret
use azure_security_keyvault_secrets::models::SecretClientGetSecretOptions;

// Retrieve a secret using the secret client.
let get_options = SecretClientGetSecretOptions {
    secret_version: Some("secret-version".to_string()),
    ..Default::default()
};
let secret = client
    .get_secret("secret-name", None)
    .await?
    .into_model()?;

println!("Secret Value: {:?}", secret.value);
```

### Update an existing secret

`update_secret_properties` updates a secret previously stored in the Azure Key Vault. Only the attributes of the secret are updated. To update the value, call `SecretClient::set_secret` on a secret with the same name.

```rust ignore update_secret
use azure_security_keyvault_secrets::models::UpdateSecretPropertiesParameters;
use std::collections::HashMap;

// Update a secret using the secret client.
let secret_update_parameters = UpdateSecretPropertiesParameters {
    content_type: Some("text/plain".into()),
    tags: Some(HashMap::from_iter(vec![(
        "tag-name".into(),
        "tag-value".into(),
    )])),
    ..Default::default()
};

client
    .update_secret_properties(
        "secret-name",
        secret_update_parameters.try_into()?,
        None,
    )
    .await?
    .into_model()?;
```

### Delete a secret

`delete_secret` will tell Key Vault to delete a secret but it is not deleted immediately. It will not be deleted until the service-configured data retention period - the default is 90 days - or until you call `purge_secret` on the returned `DeletedSecret.id`.

```rust ignore delete_secret
// Delete a secret using the secret client.
client.delete_secret("secret-name", None).await?;
```

### List secrets

This example lists all the secrets in the specified Azure Key Vault. The value is not returned when listing all secrets. You will need to call `SecretClient::get_secret` to retrieve the value.

```rust ignore list_secrets
use azure_security_keyvault_secrets::ResourceExt;
use futures::TryStreamExt;

let mut pager = client.list_secret_properties(None)?.into_stream();
while let Some(secret) = pager.try_next().await? {
    // Get the secret name from the ID.
    let name = secret.resource_id()?.name;
    println!("Found Secret with Name: {}", name);
}
```

## Troubleshooting

### General

When you interact with the Azure Key Vault secrets client library using the Rust SDK, errors returned by the service correspond to the same HTTP status codes returned for [REST API] requests.

For example, if you try to retrieve a secret that doesn't exist in your Azure Key Vault, a `404` error is returned, indicating `Not Found`.

```rust ignore errors
match client.get_secret("secret-name", None).await {
    Ok(response) => println!("Secret Value: {:?}", response.into_model()?.value),
    Err(err) => println!("Error: {:#?}", err.into_inner()?),
}
```

You will notice that additional information is logged, like the Client Request ID of the operation.

```text
Error: ErrorResponse {
    error: ErrorDetails {
        code: Some(
            "SecretNotFound",
        ),
        message: Some(
            "A secret with (name/id) secret-name was not found in this key vault. If you recently deleted this secret you may be able to recover it using the correct recovery command. For help resolving this issue, please see https://go.microsoft.com/fwlink/?linkid=2125182",
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
[API reference documentation]: https://docs.rs/azure_security_keyvault_secrets/latest/azure_security_keyvault_secrets
[Azure CLI]: https://learn.microsoft.com/cli/azure
[Azure subscription]: https://azure.microsoft.com/free/
[Azure Identity]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity
[Microsoft Open Source Code of Conduct]: https://opensource.microsoft.com/codeofconduct/
[Product documentation]: https://learn.microsoft.com/azure/key-vault/
[REST API]: https://learn.microsoft.com/rest/api/keyvault/
[Cargo]: https://crates.io/
[Package (crates.io)]: https://crates.io/crates/azure_security_keyvault_secrets
[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/keyvault/azure_security_keyvault_secrets/src
[CONTRIBUTING.md]: https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md
[Code of Conduct FAQ]: https://opensource.microsoft.com/codeofconduct/faq/
[access control]: https://learn.microsoft.com/azure/key-vault/general/assign-access-policy
[RBAC]: https://learn.microsoft.com/azure/key-vault/general/rbac-guide

Trivial change to trigger testing of azure_security_keyvault_secrets crate.
