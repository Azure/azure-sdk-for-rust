# Azure Key Vault Resource Manager client library for Rust

Manage resources provided by `Microsoft.KeyVault`, including vaults, managed HSMs,
keys, secrets, and private endpoint connections. For applications that read secret
values or perform cryptographic operations, use the Key Vault data-plane libraries
instead.

[Source code] | [Package (crates.io)] | [API reference documentation] | [Product documentation]

## Getting started

### Install the package

```sh
cargo add azure_resourcemanager_keyvault azure_identity azure_core futures
cargo add tokio --features macros,rt-multi-thread
```

### Prerequisites

- An Azure subscription and an existing resource group.
- Permission to manage Key Vault resources in that resource group.
- An authenticated Azure CLI session for local development:

```azurecli
az login
```

### Authenticate the client

Use `DeveloperToolsCredential` during local development. Use a managed identity in
production. The client defaults to Public cloud. Use cloud configuration to
select Government, China, or a custom cloud.

```rust no_run create_client
use azure_identity::DeveloperToolsCredential;
use azure_resourcemanager_keyvault::KeyVaultClient;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let credential = DeveloperToolsCredential::new(None)?;
    let client = KeyVaultClient::new(
        "<subscription-id>".into(),
        credential,
        None,
    )?;
    let vaults = client.get_key_vault_vaults_client();
    Ok(())
}
```

Custom cloud configuration is passed through `KeyVaultClientOptions.client_options.cloud`.
Set the management endpoint separately from the authentication audience. Use
`azure_core::cloud::resourcemanager::Audience`, which is shared by all Resource
Manager clients. Configure your developer tool for the custom cloud and sign in
before using `DeveloperToolsCredential`.

```rust no_run create_custom_cloud_client
use azure_core::{
    cloud::{resourcemanager::Audience, Audiences, CustomConfiguration},
    http::ClientOptions,
};
use azure_identity::DeveloperToolsCredential;
use azure_resourcemanager_keyvault::{KeyVaultClient, KeyVaultClientOptions};
use std::sync::Arc;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let mut custom = CustomConfiguration::default();
    custom.authority_host = "https://login.example.com".into();
    custom.endpoint = Some("https://management.example.com".into());
    custom.audiences = Audiences::new()
        .with::<Audience>("https://resource.example.com".into());
    let cloud = Arc::new(custom.into());
    let credential = DeveloperToolsCredential::new(None)?;
    let client = KeyVaultClient::new(
        "<subscription-id>".into(),
        credential,
        Some(KeyVaultClientOptions {
            client_options: ClientOptions {
                cloud: Some(cloud),
                ..Default::default()
            },
            ..Default::default()
        }),
    )?;
    let vaults = client.get_key_vault_vaults_client();
    Ok(())
}
```

## Key concepts

`KeyVaultClient` is scoped to a subscription. Its specialized clients manage individual
resource kinds. Creation and other long-running operations return a poller:
await it to receive the final response, or stream operation-status responses
before awaiting the result. The initial request is sent when the poller is first
polled or awaited.

## Examples

The following examples use `vaults` from the client above, a resource group named
by `resource_group`, a unique `vault_name`, and your subscription's `location` and
`tenant_id`.

- [Create a vault](#create-a-vault)
- [List vaults](#list-vaults)
- [Update a vault](#update-a-vault)
- [Handle errors](#handle-errors)
- [Delete a vault](#delete-a-vault)

### Create a vault

```rust ignore create_vault
use azure_resourcemanager_keyvault::models::{
    Sku, SkuFamily, SkuName, VaultCreateOrUpdateParameters, VaultProperties,
};

let parameters = VaultCreateOrUpdateParameters {
    location: Some(location.clone()),
    properties: Some(VaultProperties {
        tenant_id: Some(tenant_id.parse()?),
        sku: Some(Sku {
            family: Some(SkuFamily::A),
            name: Some(SkuName::Standard),
        }),
        access_policies: Some(vec![]),
        enable_rbac_authorization: Some(true),
        ..Default::default()
    }),
    ..Default::default()
};
let vault = vaults
    .begin_create_or_update(
        &resource_group,
        &vault_name,
        parameters.try_into()?,
        None,
    )?
    .await?
    .into_model()?;
assert_eq!(vault.name.as_deref(), Some(vault_name.as_str()));
```

### List vaults

List resources in the resource group rather than enumerating other resource groups.

```rust ignore list_vaults
use futures::TryStreamExt as _;

let mut pages = vaults
    .list_by_resource_group(&resource_group, None)?
    .into_stream();
let mut found = false;
while let Some(vault) = pages.try_next().await? {
    found |= vault.name.as_deref() == Some(vault_name.as_str());
}
assert!(found);
```

### Update a vault

```rust ignore update_vault
use azure_resourcemanager_keyvault::models::VaultPatchParameters;
use std::collections::HashMap;

let vault = vaults
    .update(
        &resource_group,
        &vault_name,
        VaultPatchParameters {
            tags: Some(HashMap::from([("purpose".into(), "example".into())])),
            ..Default::default()
        }
        .try_into()?,
        None,
    )
    .await?
    .into_model()?;
assert_eq!(
    vault.tags.as_ref().and_then(|tags| tags.get("purpose")).map(String::as_str),
    Some("example"),
);
```

### Handle errors

Inspect an error's HTTP status instead of matching its message.

```rust ignore errors
use azure_core::http::StatusCode;

let error = vaults
    .get(&resource_group, &missing_vault_name, None)
    .await
    .expect_err("the vault does not exist");
assert_eq!(error.http_status(), Some(StatusCode::NotFound));
```

### Delete a vault

Deleting a vault does not purge a soft-deleted vault.

```rust ignore delete_vault
vaults.delete(&resource_group, &vault_name, None).await?;
```

## Troubleshooting

### General

Verify that the credential belongs to the correct tenant, the subscription and
resource group exist, and the identity has the necessary Resource Manager
permissions. Data-plane access requires separate permissions.

Managed HSM operations require service availability and subscription quota.
For custom clouds, supply both the management endpoint and the shared Resource
Manager audience, and ensure the credential authenticates against the correct cloud.

## Contributing

See the [CONTRIBUTING.md] for details on building, testing, and contributing to these libraries.

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit <https://opensource.microsoft.com/cla/>.

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct]. For more information see the [Code of Conduct FAQ] or contact <opencode@microsoft.com> with any additional questions or comments.

<!-- LINKS -->
[API reference documentation]: https://docs.rs/azure_resourcemanager_keyvault/latest/azure_resourcemanager_keyvault
[Microsoft Open Source Code of Conduct]: https://opensource.microsoft.com/codeofconduct/
[Product documentation]: https://learn.microsoft.com/azure/key-vault/
[Package (crates.io)]: https://crates.io/crates/azure_resourcemanager_keyvault
[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/keyvault/azure_resourcemanager_keyvault/src
[CONTRIBUTING.md]: https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md
[Code of Conduct FAQ]: https://opensource.microsoft.com/codeofconduct/faq/
