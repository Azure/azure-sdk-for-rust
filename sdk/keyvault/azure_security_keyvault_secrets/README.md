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

### Authenticate the client

In order to interact with the Azure Key Vault service, you'll need to create an instance of the `SecretClient` struct. You need a **vault url**, which you may see as "DNS Name" in the portal, and credentials to instantiate a client object.

The examples shown below use a `DefaultAzureCredential`, which is appropriate for most scenarios including local development and production environments. Additionally, we recommend using a managed identity for authentication in production environments. You can find more information on different ways of authenticating and their corresponding credential types in the [Azure Identity] documentation.

To use the `DefaultAzureCredential` provider shown below, or other credential providers from the Azure SDK, you must first install the `azure_identity` crate:

```sh
cargo add azure_identity
```

The `DefaultAzureCredential` will automatically pick up on an Azure CLI authentication. Ensure you are logged in with the Azure CLI:

```azurecli
az login
```

Instantiate a `DefaultAzureCredential` to pass to the client. The same instance of a token credential can be used with multiple clients if they will be authenticating with the same identity.

```rust
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::SecretClient;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DefaultAzureCredential::default();
    let client = SecretClient::new(Url::parse(&vault_url)?, credential);

    // Create a new secret using the secret client.
    let secret = client.set_secret("secret-name", "secret-value").await?;
    println!("{}", secret.name);
    println!("{}", secret.value);

    // Retrieve a secret using the secret client.
    let secret = client.get_secret("secret-name").await?;
    println!("{}", secret.name);
    println!("{}", secret.value);

    Ok(())
}
```

## Key concepts

### KeyVaultSecret

A `KeyVaultSecret` is the fundamental resource within Azure Key Vault. From a developer's perspective, Azure Key Vault APIs accept and return secret values as strings.

### SecretClient

A `SecretClient` provides both synchronous and asynchronous operations in the SDK allowing for selection of a client based on an application's use case. Once you've initialized a `SecretClient`, you can interact with secrets in Azure Key Vault.

### Thread safety

We guarantee that all client instance methods are thread-safe and independent of each other. This ensures that the recommendation of reusing client instances is always safe, even across threads.

## Examples

The `azure_security_keyvault_secrets` crate supports synchronous and asynchronous APIs.

The following section provides several code snippets using the `client` created above, covering some of the most common Azure Key Vault Secrets service related tasks:

### Sync examples

* [Create a secret](#create-a-secret)
* [Retrieve a secret](#retrieve-a-secret)
* [Update an existing secret](#update-an-existing-secret)
* [Delete a secret](#delete-a-secret)
* [Delete and purge a secret](#delete-and-purge-a-secret)
* [List Secrets](#list-secrets)

### Async examples

* [Create a secret asynchronously](#create-a-secret-asynchronously)
* [List secrets asynchronously](#list-secrets-asynchronously)
* [Delete a secret asynchronously](#delete-a-secret-asynchronously)

### Create a secret

`set_secret` creates a `KeyVaultSecret` to be stored in the Azure Key Vault. If a secret with the same name already exists, then a new version of the secret is created.

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret = client.set_secret("secret-name", "secret-value").await?;
    println!("{}", secret.name);
    println!("{}", secret.value);
    println!("{}", secret.properties.version);
    println!("{}", secret.properties.enabled);

    Ok(())
}
```

### Retrieve a secret

`get_secret` retrieves a secret previously stored in the Azure Key Vault.

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret = client.get_secret("secret-name").await?;
    println!("{}", secret.name);
    println!("{}", secret.value);

    Ok(())
}
```

### Update an existing secret

`update_secret_properties` updates a secret previously stored in the Azure Key Vault. Only the attributes of the secret are updated. To update the value, call `SecretClient::set_secret` on a secret with the same name.

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut secret = client.get_secret("secret-name").await?;
    secret.properties.content_type = Some("text/plain".to_string());
    secret.properties.tags.insert("foo".to_string(), "updated tag".to_string());

    let updated_secret_properties = client.update_secret_properties(secret.properties).await?;
    println!("{}", updated_secret_properties.name);
    println!("{}", updated_secret_properties.version);
    println!("{}", updated_secret_properties.content_type.unwrap());

    Ok(())
}
```

### Delete a secret

`start_delete_secret` starts a long-running operation to delete a secret previously stored in the Azure Key Vault. You can retrieve the secret immediately without waiting for the operation to complete. When [soft-delete] is not enabled for the Azure Key Vault, this operation permanently deletes the secret.

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let operation = client.start_delete_secret("secret-name").await?;
    let secret = operation.value;
    println!("{}", secret.name);
    println!("{}", secret.value);

    Ok(())
}
```

### Delete and purge a secret

You will need to wait for the long-running operation to complete before trying to purge or recover the secret. You can do this by calling `update_status` in a loop as shown below:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut operation = client.start_delete_secret("secret-name").await?;
    while !operation.has_completed {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        operation.update_status().await?;
    }

    let secret = operation.value;
    client.purge_deleted_secret(secret.name).await?;

    Ok(())
}
```

### List secrets

This example lists all the secrets in the specified Azure Key Vault. The value is not returned when listing all secrets. You will need to call `SecretClient::get_secret` to retrieve the value.

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let all_secrets = client.get_properties_of_secrets().await?;
    for secret_properties in all_secrets {
        println!("{}", secret_properties.name);
    }

    Ok(())
}
```

### Create a secret asynchronously

The asynchronous APIs are identical to their synchronous counterparts, but return with the typical "Async" suffix for asynchronous methods and return a `Future`.

This example creates a secret in the Azure Key Vault with the specified optional arguments.

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret = client.set_secret("secret-name", "secret-value").await?;
    println!("{}", secret.name);
    println!("{}", secret.value);

    Ok(())
}
```

### List secrets asynchronously

Listing secrets does not rely on awaiting the `get_properties_of_secrets` method, but returns an `AsyncPageable<SecretProperties>` that you can use with the `await` statement:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let all_secrets = client.get_properties_of_secrets().await?;
    for secret_properties in all_secrets {
        println!("{}", secret_properties.name);
    }

    Ok(())
}
```

### Delete a secret asynchronously

When deleting a secret asynchronously before you purge it, you can await the `wait_for_completion` method on the operation. By default, this loops indefinitely but you can cancel it by passing a `CancellationToken`.

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut operation = client.start_delete_secret("secret-name").await?;
    operation.wait_for_completion().await?;
    let secret = operation.value;
    client.purge_deleted_secret(secret.name).await?;

    Ok(())
}
```

## Troubleshooting

See our [troubleshooting guide] for details on how to diagnose various failure scenarios.

### General

When you interact with the Azure Key Vault Secrets client library using the Rust SDK, errors returned by the service correspond to the same HTTP status codes returned for [REST API] requests.

For example, if you try to retrieve a secret that doesn't exist in your Azure Key Vault, a `404` error is returned, indicating `Not Found`.

```rust
match client.get_secret("some_secret").await {
    Ok(secret) => println!("{}", secret.name),
    Err(err) => println!("{}", err),
}
```

You will notice that additional information is logged, like the Client Request ID of the operation.

```text
Message:
    Azure.RequestFailedException : Service request failed.
    Status: 404 (Not Found)
Content:
    {"error":{"code":"SecretNotFound","message":"Secret not found: some_secret"}}

Headers:
    Cache-Control: no-cache
    Pragma: no-cache
    Server: Microsoft-IIS/10.0
    x-ms-keyvault-region: westus
    x-ms-request-id: 625f870e-10ea-41e5-8380-282e5cf768f2
    x-ms-keyvault-service-version: 1.1.0.866
    x-ms-keyvault-network-info: addr=131.107.174.199;act_addr_fam=InterNetwork;
    X-AspNet-Version: 4.0.30319
    X-Powered-By: ASP.NET
    Strict-Transport-Security: max-age=31536000;includeSubDomains
    X-Content-Type-Options: nosniff
    Date: Tue, 18 Jun 2019 16:02:11 GMT
    Content-Length: 75
    Content-Type: application/json; charset=utf-8
    Expires: -1
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
