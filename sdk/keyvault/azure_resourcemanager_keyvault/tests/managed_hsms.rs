// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::error::Result;
use azure_core_test::{recorded, TestContext};
use azure_resourcemanager_keyvault::models::{
    ManagedHsm, ManagedHsmProperties, ManagedHsmSku, ManagedHsmSkuFamily, ManagedHsmSkuName,
};
use azure_resourcemanager_keyvault::KeyVaultClient;
use futures::TryStreamExt;
use std::collections::HashMap;

#[recorded::test]
#[ignore = "requires special provisioning"]
async fn managed_hsm_lifecycle(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();

    let mut options = azure_resourcemanager_keyvault::KeyVaultClientOptions::default();
    recording.instrument(&mut options.client_options);

    let subscription_id = recording.var("KEYVAULT_SUBSCRIPTION_ID", None);
    let client = KeyVaultClient::new(subscription_id, recording.credential(), Some(options))?;

    let resource_group = recording.var("KEYVAULT_RESOURCE_GROUP", None);
    let location = recording.var("KEYVAULT_LOCATION", None);
    let hsm_name = recording.random_string::<16>(Some("t"));
    let tenant_id = recording.var("KEYVAULT_TENANT_ID", None);
    let object_id = recording.var("KEYVAULT_OBJECT_ID", None);

    let managed_hsms = client.get_key_vault_managed_hsms_client();

    // Parse tenant ID
    let tenant_id = tenant_id
        .parse()
        .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::Other, e))?;

    // Create managed HSM parameters
    println!("Creating managed HSM: {}", hsm_name);
    let hsm_params = ManagedHsm {
        location: Some(location),
        properties: Some(ManagedHsmProperties {
            tenant_id: Some(tenant_id),
            initial_admin_object_ids: Some(vec![object_id.clone()]),
            ..Default::default()
        }),
        sku: Some(ManagedHsmSku {
            family: Some(ManagedHsmSkuFamily::B),
            name: Some(ManagedHsmSkuName::StandardB1),
        }),
        tags: Some(HashMap::from_iter(vec![
            ("environment".into(), "test".into()),
            ("purpose".into(), "integration-test".into()),
        ])),
        ..Default::default()
    };

    // Create the managed HSM (this is a long-running operation)
    let hsm = managed_hsms
        .create_or_update(&resource_group, &hsm_name, hsm_params.try_into()?, None)?
        .await?
        .into_model()?;

    println!("Created managed HSM: {:?}", hsm.name);
    assert_eq!(hsm.name.as_deref(), Some(hsm_name.as_str()));

    // List managed HSMs in the resource group
    println!("Listing managed HSMs in resource group");
    let mut pager = managed_hsms
        .list_by_resource_group(&resource_group, None)?
        .into_stream();

    let mut found = false;
    while let Some(hsm) = pager.try_next().await? {
        println!("Found managed HSM: {:?}", hsm.name);
        if hsm.name.as_deref() == Some(hsm_name.as_str()) {
            found = true;
        }
    }
    assert!(found, "Created managed HSM not found in list");

    // Update managed HSM tags
    println!("Updating managed HSM tags");
    let update_params = ManagedHsm {
        tags: Some(HashMap::from_iter(vec![
            ("environment".into(), "production".into()),
            ("team".into(), "platform".into()),
            ("updated".into(), "true".into()),
        ])),
        ..Default::default()
    };

    let updated_hsm = managed_hsms
        .update(&resource_group, &hsm_name, update_params.try_into()?, None)?
        .await?
        .into_model()?;

    println!("Updated managed HSM: {:?}", updated_hsm.name);
    assert_eq!(
        updated_hsm.tags.as_ref().and_then(|t| t.get("environment")),
        Some(&"production".to_string())
    );
    assert_eq!(
        updated_hsm.tags.as_ref().and_then(|t| t.get("updated")),
        Some(&"true".to_string())
    );

    // Delete the managed HSM
    println!("Deleting managed HSM");
    managed_hsms
        .delete(&resource_group, &hsm_name, None)?
        .await?;

    println!("Deleted managed HSM");

    Ok(())
}
