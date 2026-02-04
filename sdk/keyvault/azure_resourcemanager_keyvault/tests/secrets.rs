// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::error::Result;
use azure_core_test::{recorded, TestContext};
use azure_resourcemanager_keyvault::models::{
    SecretCreateOrUpdateParameters, SecretPatchParameters, SecretProperties, Sku, SkuFamily,
    SkuName, VaultCreateOrUpdateParameters, VaultProperties,
};
use azure_resourcemanager_keyvault::KeyVaultClient;
use futures::TryStreamExt;
use std::collections::HashMap;

#[recorded::test]
async fn secret_lifecycle(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();

    let mut options = azure_resourcemanager_keyvault::KeyVaultClientOptions::default();
    recording.instrument(&mut options.client_options);

    let subscription_id = recording.var("KEYVAULT_SUBSCRIPTION_ID", None);
    let client = KeyVaultClient::new(subscription_id, recording.credential(), Some(options))?;

    let resource_group = recording.var("KEYVAULT_RESOURCE_GROUP", None);
    let location = recording.var("KEYVAULT_LOCATION", None);
    let vault_name = recording.random_string::<16>(Some("t"));
    let secret_name = recording.random_string::<16>(Some("s"));
    let tenant_id = recording.var("KEYVAULT_TENANT_ID", None);

    let vaults = client.get_key_vault_vaults_client();
    let secrets = client.get_key_vault_secrets_client();

    // Parse tenant ID
    let tenant_id = tenant_id
        .parse()
        .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::Other, e))?;

    // First, create a vault to hold the secret
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
            "secret-test".into(),
        )])),
    };

    let vault = vaults
        .create_or_update(&resource_group, &vault_name, vault_params.try_into()?, None)?
        .await?
        .into_model()?;

    println!("Created vault: {:?}", vault.name);

    // Create a secret in the vault
    println!("Creating secret: {}", secret_name);
    let secret_params = SecretCreateOrUpdateParameters {
        properties: Some(SecretProperties {
            value: Some("my-secret-value".into()),
            ..Default::default()
        }),
        tags: Some(HashMap::from_iter(vec![
            ("environment".into(), "test".into()),
            ("classification".into(), "confidential".into()),
        ])),
    };

    let secret = secrets
        .create_or_update(
            &resource_group,
            &vault_name,
            &secret_name,
            secret_params.try_into()?,
            None,
        )
        .await?
        .into_model()?;

    println!("Created secret: {:?}", secret.name);
    assert_eq!(secret.name.as_deref(), Some(secret_name.as_str()));

    // List secrets in the vault
    println!("Listing secrets in vault");
    let mut pager = secrets
        .list(&resource_group, &vault_name, None)?
        .into_stream();

    let mut found = false;
    while let Some(secret) = pager.try_next().await? {
        println!("Found secret: {:?}", secret.name);
        if secret.name.as_deref() == Some(secret_name.as_str()) {
            found = true;
        }
    }
    assert!(found, "Created secret not found in list");

    // Update the secret tags
    println!("Updating secret tags");
    let patch_params = SecretPatchParameters {
        tags: Some(HashMap::from_iter(vec![
            ("environment".into(), "production".into()),
            ("classification".into(), "secret".into()),
            ("updated".into(), "true".into()),
        ])),
        ..Default::default()
    };

    let updated_secret = secrets
        .update(
            &resource_group,
            &vault_name,
            &secret_name,
            patch_params.try_into()?,
            None,
        )
        .await?
        .into_model()?;

    println!("Updated secret: {:?}", updated_secret.name);
    assert_eq!(
        updated_secret
            .tags
            .as_ref()
            .and_then(|t| t.get("environment")),
        Some(&"production".to_string())
    );
    assert_eq!(
        updated_secret.tags.as_ref().and_then(|t| t.get("updated")),
        Some(&"true".to_string())
    );

    // Get the secret
    println!("Getting secret");
    let retrieved_secret = secrets
        .get(&resource_group, &vault_name, &secret_name, None)
        .await?
        .into_model()?;

    println!("Retrieved secret: {:?}", retrieved_secret.name);
    assert_eq!(retrieved_secret.name.as_deref(), Some(secret_name.as_str()));

    // Delete the vault (which will also delete the secret)
    println!("Deleting vault");
    vaults.delete(&resource_group, &vault_name, None).await?;

    println!("Deleted vault and secret");

    Ok(())
}
