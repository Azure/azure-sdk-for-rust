# Azure Key Vault management client library for Rust

Azure Key Vault is a cloud service that provides secure storage of secrets, keys, and certificates. This library
provides Azure Resource Manager (ARM) operations for managing Key Vault resources.

The Azure Key Vault management client library allows you to manage Key Vault resources, including vaults, managed
HSMs, keys, and secrets at the Azure Resource Provider level. This includes operations to create, retrieve, update,
delete, and list Key Vault resources.

[Source code] | [Package (crates.io)] | [API reference documentation] | [Product documentation]

## Getting started

### Install the package

Install the Azure Key Vault management client library for Rust with [Cargo]:

```sh
cargo add azure_resourcemanager_keyvault
```

### Prerequisites

- An [Azure subscription].
- Authorization to manage Azure Key Vault resources using either [RBAC].

### Install dependencies

Add the following crates to your project:

```sh
cargo add azure_identity tokio
```

### Authenticate the client

In order to interact with the Azure Key Vault management service, you'll need to create an instance of the
`KeyVaultClient`. You need an **endpoint** (typically `https://management.azure.com`), a **subscription ID**, and
credentials to instantiate a client object.

The example shown below uses a `DeveloperToolsCredential`, which is appropriate for local development environments.
We recommend using a managed identity for authentication in production environments. You can find more information on
different ways of authenticating and their corresponding credential types in the [Azure Identity] documentation.

The `DeveloperToolsCredential` will automatically pick up on an Azure CLI authentication. Ensure you are logged in
with the Azure CLI:

```azurecli
az login
```

Instantiate a `DeveloperToolsCredential` to pass to the client. The same instance of a token credential can be used
with multiple clients if they will be authenticating with the same identity.

### Instantiate a client

```rust no_run
use azure_identity::DeveloperToolsCredential;
use azure_resourcemanager_keyvault::KeyVaultClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new Key Vault management client
    let credential = DeveloperToolsCredential::new(None)?;
    let subscription_id = std::env::var("AZURE_SUBSCRIPTION_ID")?;
    let client = KeyVaultClient::new(
        "https://management.azure.com",
        credential.into(),
        subscription_id,
        None,
    )?;

    // Get a vault client
    let vaults = client.get_key_vault_vaults_client();

    // List vaults in the subscription
    let mut pager = vaults.list_by_subscription(None)?.into_stream();

    Ok(())
}
```

## Key concepts

### KeyVaultClient

The `KeyVaultClient` is the main entry point for interacting with Azure Key Vault management operations. It provides
methods to access specialized clients for different resource types:

- `get_key_vault_vaults_client()` - Manage Key Vault instances
- `get_key_vault_managed_hsms_client()` - Manage Managed HSM instances
- `get_key_vault_keys_client()` - Manage keys within vaults
- `get_key_vault_secrets_client()` - Manage secrets within vaults

### Vault

A `Vault` is an Azure Key Vault resource that can store keys, secrets, and certificates. The management API allows
you to create, configure, and manage vault properties such as access policies, network rules, and SKUs.

### Thread safety

We guarantee that all client instance methods are thread-safe and independent of each other. This ensures that the
recommendation of reusing client instances is always safe, even across threads.

## Examples

The following section provides several code snippets using a `KeyVaultClient` like we
[instantiated above](#instantiate-a-client):

- [Create a vault](#create-a-vault)
- [List vaults](#list-vaults)
- [Update a vault](#update-a-vault)
- [Delete a vault](#delete-a-vault)

### Create a vault

`create_or_update` creates a new Azure Key Vault in the specified resource group. If a vault with the same name
already exists, its properties are updated.

```rust ignore create_vault
use azure_resourcemanager_keyvault::models::{
    Sku, SkuFamily, SkuName, VaultCreateOrUpdateParameters, VaultProperties,
};
use std::collections::HashMap;

// Get the vaults client
let vaults = client.get_key_vault_vaults_client();

// Parse tenant ID
let tenant_id = tenant_id.parse()
    .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::Other, e))?;

// Create vault parameters
let vault_params = VaultCreateOrUpdateParameters {
    location: Some("eastus".into()),
    properties: Some(VaultProperties {
        tenant_id: Some(tenant_id),
        sku: Some(Sku {
            family: Some(SkuFamily::A),
            name: Some(SkuName::Standard),
        }),
        access_policies: Some(vec![]),
        ..Default::default()
    }),
    tags: Some(HashMap::from_iter(vec![
        ("environment".into(), "test".into()),
    ])),
};

// Create or update the vault (this is a long-running operation)
let vault = vaults
    .create_or_update(
        &resource_group,
        &vault_name,
        vault_params.try_into()?,
        None,
    )?
    .await?
    .into_model()?;

println!("Created vault: {:?}", vault.name);
```

### List vaults

This example lists all Key Vaults in the current subscription.

```rust ignore list_vaults
use futures::TryStreamExt;

let vaults = client.get_key_vault_vaults_client();

// List all vaults in the subscription
let mut pager = vaults.list_by_subscription(None)?.into_stream();

while let Some(vault) = pager.try_next().await? {
    println!("Found vault: {:?}", vault.name);
}
```

### Update a vault

`update` modifies the properties of an existing Azure Key Vault.

```rust ignore update_vault
use azure_resourcemanager_keyvault::models::VaultPatchParameters;
use std::collections::HashMap;

let vaults = client.get_key_vault_vaults_client();

// Update vault tags
let patch_params = VaultPatchParameters {
    properties: None,
    tags: Some(HashMap::from_iter(vec![
        ("environment".into(), "production".into()),
        ("team".into(), "platform".into()),
    ])),
};

let vault = vaults
    .update(
        &resource_group,
        &vault_name,
        patch_params.try_into()?,
        None,
    )
    .await?
    .into_model()?;

println!("Updated vault: {:?}", vault.name);
```

### Delete a vault

`delete` removes an Azure Key Vault. Note that vaults are soft-deleted by default and can be recovered within the
retention period.

```rust ignore delete_vault
let vaults = client.get_key_vault_vaults_client();

// Delete the vault
vaults
    .delete(
        &resource_group,
        &vault_name,
        None,
    )
    .await?;

println!("Deleted vault");
```

## Troubleshooting

### General

When you interact with the Azure Key Vault management client library using the Rust SDK, errors returned by the
service correspond to the same HTTP status codes returned for [REST API] requests.

For example, if you try to retrieve a vault that doesn't exist, a `404` error is returned, indicating `Not Found`.

```rust ignore errors
let vaults = client.get_key_vault_vaults_client();

match vaults.get(
    &resource_group,
    "nonexistent-vault",
    None,
).await {
    Ok(response) => println!("Vault: {:?}", response.into_model()?.name),
    Err(err) => println!("Error: {:#?}", err),
}
```

You will notice that additional information is logged, like the Client Request ID of the operation.

### Authentication

Ensure that the identity you're using has the necessary permissions to manage Key Vault resources. At minimum,
you'll need:

- `Microsoft.KeyVault/vaults/read` to list and get vaults
- `Microsoft.KeyVault/vaults/write` to create or update vaults
- `Microsoft.KeyVault/vaults/delete` to delete vaults

## Contributing

See the [CONTRIBUTING.md] for details on building, testing, and contributing to these libraries.

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License
Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution.
For details, visit <https://opensource.microsoft.com/cla/>.

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate
the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You will only need to
do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct]. For more information see the
[Code of Conduct FAQ] or contact <opencode@microsoft.com> with any additional questions or comments.

<!-- LINKS -->
[API reference documentation]: https://docs.rs/azure_resourcemanager_keyvault/latest/azure_resourcemanager_keyvault
[Azure subscription]: https://azure.microsoft.com/free/
[Azure Identity]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity
[Microsoft Open Source Code of Conduct]: https://opensource.microsoft.com/codeofconduct/
[Product documentation]: https://learn.microsoft.com/azure/key-vault/
[REST API]: https://learn.microsoft.com/rest/api/keyvault/
[Cargo]: https://crates.io/
[Package (crates.io)]: https://crates.io/crates/azure_resourcemanager_keyvault
[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/keyvault/azure_resourcemanager_keyvault/src
[CONTRIBUTING.md]: https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md
[Code of Conduct FAQ]: https://opensource.microsoft.com/codeofconduct/faq/
[RBAC]: https://learn.microsoft.com/azure/role-based-access-control/overview
