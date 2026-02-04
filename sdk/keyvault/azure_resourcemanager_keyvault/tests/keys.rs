// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::error::Result;
use azure_core_test::{recorded, TestContext};
use azure_resourcemanager_keyvault::models::{
    JsonWebKeyType, KeyCreateParameters, KeyProperties, Sku, SkuFamily, SkuName,
    VaultCreateOrUpdateParameters, VaultProperties,
};
use azure_resourcemanager_keyvault::KeyVaultClient;
use futures::TryStreamExt;
use std::collections::HashMap;

#[recorded::test]
async fn key_lifecycle(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();

    let mut options = azure_resourcemanager_keyvault::KeyVaultClientOptions::default();
    recording.instrument(&mut options.client_options);

    let subscription_id = recording.var("KEYVAULT_SUBSCRIPTION_ID", None);
    let client = KeyVaultClient::new(subscription_id, recording.credential(), Some(options))?;

    let resource_group = recording.var("KEYVAULT_RESOURCE_GROUP", None);
    let location = recording.var("KEYVAULT_LOCATION", None);
    let vault_name = recording.random_string::<16>(Some("t"));
    let key_name = recording.random_string::<16>(Some("k"));
    let tenant_id = recording.var("KEYVAULT_TENANT_ID", None);

    let vaults = client.get_key_vault_vaults_client();
    let keys = client.get_key_vault_keys_client();

    // Parse tenant ID
    let tenant_id = tenant_id
        .parse()
        .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::Other, e))?;

    // First, create a vault to hold the key
    println!("Creating vault: {}", vault_name);
    let vault_params = VaultCreateOrUpdateParameters {
        location: Some(location),
        properties: Some(VaultProperties {
            tenant_id: Some(tenant_id),
            sku: Some(Sku {
                family: Some(SkuFamily::A),
                name: Some(SkuName::Standard),
            }),
            access_policies: Some(vec![]),
            ..Default::default()
        }),
        tags: Some(HashMap::from_iter(vec![(
            "purpose".into(),
            "key-test".into(),
        )])),
    };

    let vault = vaults
        .create_or_update(&resource_group, &vault_name, vault_params.try_into()?, None)?
        .await?
        .into_model()?;

    println!("Created vault: {:?}", vault.name);

    // Create a key in the vault
    println!("Creating key: {}", key_name);
    let key_params = KeyCreateParameters {
        properties: Some(KeyProperties {
            kty: Some(JsonWebKeyType::Rsa),
            key_size: Some(2048),
            ..Default::default()
        }),
        tags: Some(HashMap::from_iter(vec![
            ("environment".into(), "test".into()),
            ("type".into(), "rsa".into()),
        ])),
    };

    let key = keys
        .create_if_not_exist(
            &resource_group,
            &vault_name,
            &key_name,
            key_params.try_into()?,
            None,
        )
        .await?
        .into_model()?;

    println!("Created key: {:?}", key.name);
    assert_eq!(key.name.as_deref(), Some(key_name.as_str()));

    // List keys in the vault
    println!("Listing keys in vault");
    let mut pager = keys.list(&resource_group, &vault_name, None)?.into_stream();

    let mut found = false;
    while let Some(key) = pager.try_next().await? {
        println!("Found key: {:?}", key.name);
        if key.name.as_deref() == Some(key_name.as_str()) {
            found = true;
        }
    }
    assert!(found, "Created key not found in list");

    // Get the key
    println!("Getting key");
    let retrieved_key = keys
        .get(&resource_group, &vault_name, &key_name, None)
        .await?
        .into_model()?;

    println!("Retrieved key: {:?}", retrieved_key.name);
    assert_eq!(retrieved_key.name.as_deref(), Some(key_name.as_str()));
    assert_eq!(
        retrieved_key.tags.as_ref().and_then(|t| t.get("type")),
        Some(&"rsa".to_string())
    );

    // Delete the vault (which will also delete the key)
    println!("Deleting vault");
    vaults.delete(&resource_group, &vault_name, None).await?;

    println!("Deleted vault and key");

    Ok(())
}
