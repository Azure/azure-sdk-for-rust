# Azure App Configuration client library for Rust

Azure App Configuration centrally manages application settings and feature flags.
This client library provides asynchronous operations for configuration settings, labels,
revisions, locks, and snapshots.

[Source code] | [Package (crates.io)] | [API reference documentation] | [Product documentation]

## Getting started

### Install the package

This crate is under development and isn't published yet. Add it from your local
SDK checkout:

```toml
[dependencies]
azure_data_appconfiguration = { path = "<sdk-checkout>/sdk/appconfiguration/azure_data_appconfiguration" }
azure_core = { path = "<sdk-checkout>/sdk/core/azure_core" }
azure_identity = { path = "<sdk-checkout>/sdk/identity/azure_identity" }
```

Use Azure Core and Azure Identity from the same checkout so that credential and
client-option types match this development crate.

### Prerequisites

- An [Azure subscription].
- An existing App Configuration store and its endpoint, such as
  `https://<store-name>.azconfig.io`.
- The [App Configuration Data Reader] role to read settings, or the
  [App Configuration Data Owner] role to modify them.

Management-plane roles such as Contributor don't grant access to configuration data.

### Install dependencies

Add an asynchronous runtime and stream support:

```sh
cargo add futures
cargo add tokio --features macros,rt-multi-thread
```

### Authenticate the client

The client supports Microsoft Entra ID authentication through
`azure_core::credentials::TokenCredential`. Connection-string authentication
isn't implemented.

Use `DeveloperToolsCredential` for local development after signing in with the
[Azure CLI]:

```azurecli
az login
```

Use a managed identity in production. See [Azure Identity] for credential options.

### Instantiate a client

```rust no_run
use azure_data_appconfiguration::ConfigurationClient;
use azure_identity::DeveloperToolsCredential;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credential = DeveloperToolsCredential::new(None)?;
    let client = ConfigurationClient::new(
        "https://<store-name>.azconfig.io",
        credential,
        None,
    )?;

    let setting = client.get_configuration_setting("app:message", None).await?.into_model()?;
    println!("Setting value: {:?}", setting.value);
    Ok(())
}
```

## Key concepts

- A `ConfigurationSetting` stores a setting's key, value, content type, and tags.
- Labels distinguish settings with the same key, such as settings for different
  environments. Set the operation's `label` option to select a label.
- ETags enable conditional writes through the `if_match` and `if_none_match` options.
  Supply a quoted HTTP ETag, such as `"\"etag-value\""`, or the wildcard `"*"`.
- Sync tokens returned by the service can be passed through the `sync_token`
  operation option for consistency across requests. The client doesn't
  automatically track sync tokens.
- Snapshots preserve an immutable set of settings selected by filters.
  Snapshot creation currently has a generated poller limitation; see the crate's
  [snapshot testing limitation](CONTRIBUTING.md#snapshot-testing-limitation).

The default API version is `2026-04-01`. Set
`ConfigurationClientOptions::api_version` to override it.

## Examples

### Create or update a setting

```rust no_run
use azure_core::Result;
use azure_data_appconfiguration::{
    models::{ConfigurationClientSetConfigurationSettingOptions, ConfigurationSetting, SetConfigurationSettingRequestContentType},
    ConfigurationClient,
};

async fn set_setting(client: &ConfigurationClient) -> Result<()> {
    let setting = ConfigurationSetting {
        value: Some("Hello from App Configuration".into()),
        ..Default::default()
    };
    client.set_configuration_setting(
        SetConfigurationSettingRequestContentType::ApplicationVndMicrosoftAppconfigKvJson,
        "app:message",
        Some(ConfigurationClientSetConfigurationSettingOptions {
            entity: Some(setting.try_into()?),
            label: Some("development".into()),
            ..Default::default()
        }),
    ).await?;
    Ok(())
}
```

### List settings

List operations return a pager. Iterate over pages to read settings and follow
continuation links automatically:

```rust no_run
use azure_core::Result;
use azure_data_appconfiguration::{
    models::ConfigurationClientListConfigurationSettingsOptions,
    ConfigurationClient,
};
use futures::TryStreamExt;

async fn list_settings(client: &ConfigurationClient) -> Result<()> {
    let mut pages = client.list_configuration_settings(Some(ConfigurationClientListConfigurationSettingsOptions {
        key: Some("app:*".into()),
        label: Some("development".into()),
        ..Default::default()
    }))?.into_pages();
    while let Some(page) = pages.try_next().await? {
        for setting in page.into_model()?.items {
            println!("{:?}: {:?}", setting.key, setting.value);
        }
    }
    Ok(())
}
```

### Delete a setting

```rust no_run
use azure_core::Result;
use azure_data_appconfiguration::{
    models::ConfigurationClientDeleteConfigurationSettingOptions,
    ConfigurationClient,
};

async fn delete_setting(client: &ConfigurationClient) -> Result<()> {
    client.delete_configuration_setting("app:message", Some(ConfigurationClientDeleteConfigurationSettingOptions {
        label: Some("development".into()),
        ..Default::default()
    })).await?;
    Ok(())
}
```

## Troubleshooting

### General

Service failures return `azure_core::Error`. A `403` response can indicate that
the identity lacks a data-plane role; a `404` can indicate that the key and label
combination doesn't exist. See the [REST API] for status codes and request options.

The default feature enables Azure Core's default HTTP transport. Applications
that disable default features must configure a transport through
`ConfigurationClientOptions::client_options`.

## Contributing

See the [CONTRIBUTING.md] for details on building, testing, and contributing to these libraries.

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit <https://opensource.microsoft.com/cla/>.

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct]. For more information see the [Code of Conduct FAQ] or contact <opencode@microsoft.com> with any additional questions or comments.

<!-- LINKS -->
[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/appconfiguration/azure_data_appconfiguration/src
[Package (crates.io)]: https://crates.io/crates/azure_data_appconfiguration
[API reference documentation]: https://docs.rs/azure_data_appconfiguration
[Product documentation]: https://learn.microsoft.com/azure/azure-app-configuration/
[Azure subscription]: https://azure.microsoft.com/free/
[App Configuration Data Reader]: https://learn.microsoft.com/azure/azure-app-configuration/concept-enable-rbac#azure-built-in-roles-for-azure-app-configuration
[App Configuration Data Owner]: https://learn.microsoft.com/azure/azure-app-configuration/concept-enable-rbac#azure-built-in-roles-for-azure-app-configuration
[Azure CLI]: https://learn.microsoft.com/cli/azure
[Azure Identity]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity
[REST API]: https://learn.microsoft.com/rest/api/appconfiguration/
[CONTRIBUTING.md]: https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md
[Microsoft Open Source Code of Conduct]: https://opensource.microsoft.com/codeofconduct/
[Code of Conduct FAQ]: https://opensource.microsoft.com/codeofconduct/faq/
