// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod common;

use azure_core::{http::Url, Result};
use azure_core_test::{recorded, TestContext};
use azure_resourcemanager_keyvault::models::{
    ActivationStatus, CreateMode, DeletionRecoveryLevel, GeoReplicationRegionProvisioningState,
    JsonWebKeyCurveName, JsonWebKeyOperation, JsonWebKeyType, KeyRotationPolicyActionType,
    MHSMNetworkRuleSet, ManagedHsm, ManagedHsmKey, ManagedHsmKeyCreateParameters,
    ManagedHsmKeyProperties, ManagedHsmProperties, ManagedHsmSku, ManagedHsmSkuFamily,
    ManagedHsmSkuNameV2, NetworkRuleAction, NetworkRuleBypassOptions, ProvisioningState,
    PublicNetworkAccess,
};
use common::{assert_enum_values, assert_system_data, client, model_and_json};
use futures::TryStreamExt as _;
use serde_json::Value;
use std::collections::HashMap;

fn hsm_parameters(ctx: &TestContext) -> Result<ManagedHsm> {
    let recording = ctx.recording();
    Ok(ManagedHsm {
        location: Some(recording.var("KEYVAULT_LOCATION", None)),
        properties: Some(ManagedHsmProperties {
            create_mode: Some(CreateMode::Default),
            tenant_id: Some(recording.var("KEYVAULT_TENANT_ID", None).parse()?),
            initial_admin_object_ids: Some(vec![recording.var("KEYVAULT_OBJECT_ID", None)]),
            ..Default::default()
        }),
        sku: Some(ManagedHsmSku {
            family: Some(ManagedHsmSkuFamily::B),
            name: Some(ManagedHsmSkuNameV2::StandardB1),
        }),
        ..Default::default()
    })
}

fn assert_hsm(hsm: &ManagedHsm, raw: &Value) {
    let sku = hsm.sku.as_ref().unwrap();
    assert_eq!(sku.family, Some(ManagedHsmSkuFamily::B));
    assert_eq!(sku.name, Some(ManagedHsmSkuNameV2::StandardB1));
    assert_eq!(raw["sku"]["family"], "B");
    assert_eq!(raw["sku"]["name"], "Standard_B1");
    let properties = hsm.properties.as_ref().unwrap();
    let state = properties.provisioning_state.as_ref().unwrap();
    assert!(matches!(
        state,
        ProvisioningState::Succeeded | ProvisioningState::Activated
    ));
    assert_eq!(
        raw["properties"]["provisioningState"].as_str(),
        Some(state.as_ref())
    );
    if let Some(activation) = properties
        .security_domain_properties
        .as_ref()
        .and_then(|domain| domain.activation_status.as_ref())
    {
        assert!(matches!(
            activation,
            ActivationStatus::Active | ActivationStatus::NotActivated
        ));
        assert_eq!(
            raw["properties"]["securityDomainProperties"]["activationStatus"].as_str(),
            Some(activation.as_ref())
        );
    }
    if let Some(regions) = &properties.regions {
        for (region, raw_region) in regions
            .iter()
            .zip(raw["properties"]["regions"].as_array().unwrap())
        {
            if let Some(state) = &region.provisioning_state {
                assert!(matches!(
                    state,
                    GeoReplicationRegionProvisioningState::Cleanup
                        | GeoReplicationRegionProvisioningState::Deleting
                        | GeoReplicationRegionProvisioningState::Failed
                        | GeoReplicationRegionProvisioningState::Preprovisioning
                        | GeoReplicationRegionProvisioningState::Provisioning
                        | GeoReplicationRegionProvisioningState::Succeeded
                ));
                assert_eq!(
                    raw_region["provisioningState"].as_str(),
                    Some(state.as_ref())
                );
            }
        }
    }
    assert_system_data(hsm.system_data.as_ref(), raw);
}

#[recorded::test]
#[ignore = "requires Managed HSM availability, quota, and special provisioning"]
async fn managed_hsm_lifecycle(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let client = client(&ctx).await?;
    let group = recording.var("KEYVAULT_RESOURCE_GROUP", None);
    let name = recording.random_string::<16>(Some("h"));
    let hsms = client.get_key_vault_managed_hsms_client();
    let (hsm, raw) = model_and_json(
        hsms.begin_create_or_update(&group, &name, hsm_parameters(&ctx)?.try_into()?, None)?
            .await?,
    )?;
    assert_eq!(hsm.name.as_deref(), Some(name.as_str()));
    assert_hsm(&hsm, &raw);
    let (hsm, raw) = model_and_json(hsms.get(&group, &name, None).await?)?;
    assert_hsm(&hsm, &raw);
    let mut pager = hsms.list_by_resource_group(&group, None)?.into_stream();
    let mut found = false;
    while let Some(hsm) = pager.try_next().await? {
        found |= hsm.name.as_deref() == Some(name.as_str());
    }
    assert!(found);
    let (hsm, raw) = model_and_json(
        hsms.begin_update(
            &group,
            &name,
            ManagedHsm {
                tags: Some(HashMap::from([("updated".into(), "true".into())])),
                ..Default::default()
            }
            .try_into()?,
            None,
        )?
        .await?,
    )?;
    assert_hsm(&hsm, &raw);
    assert_eq!(
        hsm.tags
            .as_ref()
            .and_then(|tags| tags.get("updated"))
            .map(String::as_str),
        Some("true")
    );
    hsms.begin_delete(&group, &name, None)?.await?;
    Ok(())
}

#[recorded::test]
#[ignore = "requires a test-created Managed HSM and regional quota"]
async fn managed_hsm_network_rules(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let client = client(&ctx).await?;
    let group = recording.var("KEYVAULT_RESOURCE_GROUP", None);
    let name = recording.random_string::<16>(Some("h"));
    let hsms = client.get_key_vault_managed_hsms_client();
    hsms.begin_create_or_update(&group, &name, hsm_parameters(&ctx)?.try_into()?, None)?
        .await?;
    for (action, bypass, public) in [
        (
            NetworkRuleAction::Deny,
            NetworkRuleBypassOptions::None,
            PublicNetworkAccess::Disabled,
        ),
        (
            NetworkRuleAction::Allow,
            NetworkRuleBypassOptions::AzureServices,
            PublicNetworkAccess::Enabled,
        ),
    ] {
        let response = hsms
            .begin_update(
                &group,
                &name,
                ManagedHsm {
                    properties: Some(ManagedHsmProperties {
                        network_acls: Some(MHSMNetworkRuleSet {
                            default_action: Some(action.clone()),
                            bypass: Some(bypass.clone()),
                            ..Default::default()
                        }),
                        public_network_access: Some(public.clone()),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
                .try_into()?,
                None,
            )?
            .await?;
        for response in [response, hsms.get(&group, &name, None).await?] {
            let (hsm, raw) = model_and_json(response)?;
            assert_hsm(&hsm, &raw);
            let properties = hsm.properties.unwrap();
            let rules = properties.network_acls.unwrap();
            assert_eq!(rules.default_action.as_ref(), Some(&action));
            assert_eq!(rules.bypass.as_ref(), Some(&bypass));
            assert_eq!(properties.public_network_access.as_ref(), Some(&public));
            assert_eq!(
                raw["properties"]["networkAcls"]["defaultAction"].as_str(),
                Some(action.as_ref())
            );
            assert_eq!(
                raw["properties"]["networkAcls"]["bypass"].as_str(),
                Some(bypass.as_ref())
            );
            assert_eq!(
                raw["properties"]["publicNetworkAccess"].as_str(),
                Some(public.as_ref())
            );
        }
    }
    hsms.begin_delete(&group, &name, None)?.await?;
    Ok(())
}

fn assert_key(
    key: &ManagedHsmKey,
    raw: &Value,
    kty: &JsonWebKeyType,
    curve: Option<&JsonWebKeyCurveName>,
    operations: &[JsonWebKeyOperation],
) {
    let properties = key.properties.as_ref().unwrap();
    assert_eq!(properties.kty.as_ref(), Some(kty));
    assert_eq!(raw["properties"]["kty"].as_str(), Some(kty.as_ref()));
    assert_eq!(properties.curve_name.as_ref(), curve);
    if let Some(curve) = curve {
        assert_eq!(
            raw["properties"]["curveName"].as_str(),
            Some(curve.as_ref())
        );
    }
    assert_enum_values(
        properties.key_ops.as_deref(),
        Some(operations),
        &raw["properties"]["keyOps"],
    );
    if let Some(level) = properties
        .attributes
        .as_ref()
        .and_then(|attributes| attributes.recovery_level.as_ref())
    {
        assert!(matches!(
            level,
            DeletionRecoveryLevel::Purgeable
                | DeletionRecoveryLevel::Recoverable
                | DeletionRecoveryLevel::RecoverableProtectedSubscription
                | DeletionRecoveryLevel::RecoverablePurgeable
        ));
        assert_eq!(
            raw["properties"]["attributes"]["recoveryLevel"].as_str(),
            Some(level.as_ref())
        );
    }
    if let Some(policy) = &properties.rotation_policy {
        if let Some(actions) = &policy.lifetime_actions {
            let raw_actions = raw["properties"]["rotationPolicy"]["lifetimeActions"]
                .as_array()
                .unwrap();
            assert_eq!(actions.len(), raw_actions.len());
            for (action, raw_action) in actions.iter().zip(raw_actions) {
                let action = action.action.as_ref().unwrap().type_prop.unwrap();
                assert!(matches!(
                    action,
                    KeyRotationPolicyActionType::Notify | KeyRotationPolicyActionType::Rotate
                ));
                assert_eq!(raw_action["action"]["type"].as_str(), Some(action.as_ref()));
            }
        }
    }
}

#[recorded::test]
#[ignore = "requires a dedicated activated disposable HSM with external provisioning and teardown"]
async fn managed_hsm_key_lifecycle(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    assert_eq!(
        recording.var("KEYVAULT_MANAGED_HSM_DISPOSABLE", None),
        "true",
        "the fixture owner must arrange disposal of this activated test HSM"
    );
    let client = client(&ctx).await?;
    let group = recording.var("KEYVAULT_RESOURCE_GROUP", None);
    let hsm = recording.var("KEYVAULT_MANAGED_HSM_NAME", None);
    let keys = client.get_key_vault_managed_hsm_keys_client();
    let mut names = Vec::new();
    for (kty, curve, operations) in [
        (
            JsonWebKeyType::RsaHsm,
            None,
            vec![
                JsonWebKeyOperation::Encrypt,
                JsonWebKeyOperation::Decrypt,
                JsonWebKeyOperation::WrapKey,
                JsonWebKeyOperation::UnwrapKey,
            ],
        ),
        (
            JsonWebKeyType::EcHsm,
            Some(JsonWebKeyCurveName::P256),
            vec![JsonWebKeyOperation::Sign, JsonWebKeyOperation::Verify],
        ),
    ] {
        let name = recording.random_string::<16>(Some("k"));
        let (key, raw) = model_and_json(
            keys.create_if_not_exist(
                &group,
                &hsm,
                &name,
                ManagedHsmKeyCreateParameters {
                    properties: Some(ManagedHsmKeyProperties {
                        kty: Some(kty.clone()),
                        key_size: if curve.is_none() { Some(2048) } else { None },
                        curve_name: curve.clone(),
                        key_ops: Some(operations.clone()),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
                .try_into()?,
                None,
            )
            .await?,
        )?;
        assert_key(&key, &raw, &kty, curve.as_ref(), &operations);
        let (key, raw) = model_and_json(keys.get(&group, &hsm, &name, None).await?)?;
        assert_key(&key, &raw, &kty, curve.as_ref(), &operations);
        let url: Url = key
            .properties
            .as_ref()
            .unwrap()
            .key_uri_with_version
            .as_ref()
            .unwrap()
            .parse()?;
        let version = url.path_segments().unwrap().next_back().unwrap();
        let (key, raw) =
            model_and_json(keys.get_version(&group, &hsm, &name, version, None).await?)?;
        assert_key(&key, &raw, &kty, curve.as_ref(), &operations);
        let mut versions = keys.list_versions(&group, &hsm, &name, None)?.into_stream();
        let mut count = 0;
        while let Some(key) = versions.try_next().await? {
            if let Some(actual) = key.properties.and_then(|properties| properties.kty) {
                assert_eq!(actual, kty);
            }
            count += 1;
        }
        assert_eq!(count, 1);
        names.push(name);
    }
    let mut pager = keys.list(&group, &hsm, None)?.into_stream();
    while let Some(key) = pager.try_next().await? {
        if let Some(name) = key.name {
            names.retain(|expected| expected != &name);
        }
    }
    assert!(names.is_empty());
    Ok(())
}
