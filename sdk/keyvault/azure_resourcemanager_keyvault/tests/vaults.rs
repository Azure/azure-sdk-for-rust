// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod common;

use azure_core::{
    http::{StatusCode, Url},
    Result,
};
use azure_core_test::{recorded, TestContext, TestMode};
use azure_resourcemanager_keyvault::{
    models::{
        AccessPolicyEntry, AccessPolicyUpdateKind, Action, CertificatePermissions, CreateMode,
        DeletionRecoveryLevel, JsonWebKeyCurveName, JsonWebKeyOperation, JsonWebKeyType, Key,
        KeyCreateParameters, KeyPermissions, KeyProperties, KeyRotationPolicyActionType,
        KeyRotationPolicyAttributes, LifetimeAction, NetworkRuleAction, NetworkRuleBypassOptions,
        NetworkRuleSet, Permissions, Reason, RotationPolicy, SecretCreateOrUpdateParameters,
        SecretPatchParameters, SecretPatchProperties, SecretPermissions, SecretProperties, Sku,
        SkuFamily, SkuName, StoragePermissions, Trigger, Vault, VaultAccessPolicyParameters,
        VaultAccessPolicyProperties, VaultCheckNameAvailabilityParameters,
        VaultCreateOrUpdateParameters, VaultPatchParameters, VaultPatchProperties, VaultProperties,
        VaultProvisioningState,
    },
    KeyVaultClient,
};
use azure_security_keyvault_test::Retry;
use common::{assert_enum_values, assert_system_data, client, model_and_json};
use futures::TryStreamExt as _;
use serde_json::Value;
use std::collections::HashMap;

fn vault_parameters(ctx: &TestContext) -> Result<VaultCreateOrUpdateParameters> {
    let recording = ctx.recording();
    Ok(VaultCreateOrUpdateParameters {
        location: Some(recording.var("KEYVAULT_LOCATION", None)),
        properties: Some(VaultProperties {
            create_mode: Some(CreateMode::Default),
            tenant_id: Some(recording.var("KEYVAULT_TENANT_ID", None).parse()?),
            sku: Some(Sku {
                family: Some(SkuFamily::A),
                name: Some(SkuName::Standard),
            }),
            access_policies: Some(vec![]),
            enable_rbac_authorization: Some(true),
            enable_soft_delete: Some(true),
            soft_delete_retention_in_days: Some(7),
            ..Default::default()
        }),
        ..Default::default()
    })
}

fn assert_vault(vault: &Vault, raw: &Value) {
    let properties = vault.properties.as_ref().unwrap();
    let sku = properties.sku.as_ref().unwrap();
    assert_eq!(sku.family, Some(SkuFamily::A));
    assert_eq!(sku.name, Some(SkuName::Standard));
    assert_eq!(raw["properties"]["sku"]["family"], "A");
    assert_eq!(raw["properties"]["sku"]["name"], "standard");
    assert_eq!(
        properties.provisioning_state,
        Some(VaultProvisioningState::Succeeded)
    );
    assert_eq!(raw["properties"]["provisioningState"], "Succeeded");
    assert_system_data(vault.system_data.as_ref(), raw);
}

async fn create_vault(ctx: &TestContext, client: &KeyVaultClient, name: &str) -> Result<()> {
    let recording = ctx.recording();
    let (vault, raw) = model_and_json(
        client
            .get_key_vault_vaults_client()
            .begin_create_or_update(
                &recording.var("KEYVAULT_RESOURCE_GROUP", None),
                name,
                vault_parameters(ctx)?.try_into()?,
                None,
            )?
            .await?,
    )?;
    assert_eq!(vault.name.as_deref(), Some(name));
    assert_vault(&vault, &raw);
    Ok(())
}

fn assert_key(
    key: &Key,
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
        // Seven-day retention returns a value missing from the specification.
        assert_eq!(
            level,
            &DeletionRecoveryLevel::UnknownValue("CustomizedRecoverable+Purgeable".into())
        );
        assert_eq!(
            raw["properties"]["attributes"]["recoveryLevel"],
            "CustomizedRecoverable+Purgeable"
        );
    }
}

fn key_version(key: &Key) -> Result<String> {
    let url: Url = key
        .properties
        .as_ref()
        .unwrap()
        .key_uri_with_version
        .as_ref()
        .unwrap()
        .parse()?;
    Ok(url
        .path_segments()
        .unwrap()
        .next_back()
        .unwrap()
        .to_string())
}

#[recorded::test]
async fn key_lifecycle(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let client = client(&ctx).await?;
    let resource_group = recording.var("KEYVAULT_RESOURCE_GROUP", None);
    let vault_name = recording.random_string::<16>(Some("v"));
    let key_name = recording.random_string::<16>(Some("k"));
    create_vault(&ctx, &client, &vault_name).await?;
    let keys = client.get_key_vault_keys_client();
    let operations = vec![
        JsonWebKeyOperation::Encrypt,
        JsonWebKeyOperation::Decrypt,
        JsonWebKeyOperation::WrapKey,
        JsonWebKeyOperation::UnwrapKey,
    ];
    let (key, raw) = model_and_json(
        keys.create_if_not_exist(
            &resource_group,
            &vault_name,
            &key_name,
            KeyCreateParameters {
                properties: Some(KeyProperties {
                    kty: Some(JsonWebKeyType::Rsa),
                    key_size: Some(2048),
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
    assert_eq!(key.name.as_deref(), Some(key_name.as_str()));
    assert_key(&key, &raw, &JsonWebKeyType::Rsa, None, &operations);
    let mut pager = keys.list(&resource_group, &vault_name, None)?.into_stream();
    let mut found = false;
    while let Some(key) = pager.try_next().await? {
        found |= key.name.as_deref() == Some(key_name.as_str());
    }
    assert!(found);
    let (key, raw) = model_and_json(
        keys.get(&resource_group, &vault_name, &key_name, None)
            .await?,
    )?;
    assert_eq!(key.name.as_deref(), Some(key_name.as_str()));
    assert_key(&key, &raw, &JsonWebKeyType::Rsa, None, &operations);
    let (versioned, raw) = model_and_json(
        keys.get_version(
            &resource_group,
            &vault_name,
            &key_name,
            &key_version(&key)?,
            None,
        )
        .await?,
    )?;
    assert_key(&versioned, &raw, &JsonWebKeyType::Rsa, None, &operations);
    let mut versions = keys
        .list_versions(&resource_group, &vault_name, &key_name, None)?
        .into_stream();
    let mut count = 0;
    while let Some(version) = versions.try_next().await? {
        if let Some(kty) = version
            .properties
            .as_ref()
            .and_then(|properties| properties.kty.as_ref())
        {
            assert_eq!(kty, &JsonWebKeyType::Rsa);
        }
        count += 1;
    }
    assert_eq!(count, 1);
    client
        .get_key_vault_vaults_client()
        .delete(&resource_group, &vault_name, None)
        .await?;
    Ok(())
}

#[recorded::test]
async fn ec_key_lifecycle(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let client = client(&ctx).await?;
    let group = recording.var("KEYVAULT_RESOURCE_GROUP", None);
    let vault = recording.random_string::<16>(Some("v"));
    create_vault(&ctx, &client, &vault).await?;
    let keys = client.get_key_vault_keys_client();
    let operations = vec![JsonWebKeyOperation::Sign, JsonWebKeyOperation::Verify];
    for curve in [
        JsonWebKeyCurveName::P256,
        JsonWebKeyCurveName::P256K,
        JsonWebKeyCurveName::P384,
        JsonWebKeyCurveName::P521,
    ] {
        let name = recording.random_string::<16>(Some("k"));
        let (key, raw) = model_and_json(
            keys.create_if_not_exist(
                &group,
                &vault,
                &name,
                KeyCreateParameters {
                    properties: Some(KeyProperties {
                        kty: Some(JsonWebKeyType::Ec),
                        curve_name: Some(curve.clone()),
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
        assert_key(&key, &raw, &JsonWebKeyType::Ec, Some(&curve), &operations);
        let (key, raw) = model_and_json(keys.get(&group, &vault, &name, None).await?)?;
        assert_key(&key, &raw, &JsonWebKeyType::Ec, Some(&curve), &operations);
        let (versioned, raw) = model_and_json(
            keys.get_version(&group, &vault, &name, &key_version(&key)?, None)
                .await?,
        )?;
        assert_key(
            &versioned,
            &raw,
            &JsonWebKeyType::Ec,
            Some(&curve),
            &operations,
        );
        let mut versions = keys
            .list_versions(&group, &vault, &name, None)?
            .into_stream();
        let mut count = 0;
        while let Some(version) = versions.try_next().await? {
            if let Some(properties) = version.properties {
                if let Some(actual) = properties.kty {
                    assert_eq!(actual, JsonWebKeyType::Ec);
                }
                if let Some(actual) = properties.curve_name {
                    assert_eq!(actual, curve);
                }
            }
            count += 1;
        }
        assert_eq!(count, 1);
    }
    let mut keys = keys.list(&group, &vault, None)?.into_stream();
    let mut count = 0;
    while let Some(key) = keys.try_next().await? {
        if let Some(kty) = key.properties.and_then(|properties| properties.kty) {
            assert_eq!(kty, JsonWebKeyType::Ec);
        }
        count += 1;
    }
    assert_eq!(count, 4);
    client
        .get_key_vault_vaults_client()
        .delete(&group, &vault, None)
        .await?;
    Ok(())
}

#[recorded::test]
async fn key_rotation_policy(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let client = client(&ctx).await?;
    let group = recording.var("KEYVAULT_RESOURCE_GROUP", None);
    let vault = recording.random_string::<16>(Some("v"));
    let name = recording.random_string::<16>(Some("k"));
    create_vault(&ctx, &client, &vault).await?;
    let keys = client.get_key_vault_keys_client();
    let policy = RotationPolicy {
        attributes: Some(KeyRotationPolicyAttributes {
            expiry_time: Some("P1Y".into()),
            ..Default::default()
        }),
        lifetime_actions: Some(vec![
            LifetimeAction {
                action: Some(Action {
                    type_prop: Some(KeyRotationPolicyActionType::Notify),
                }),
                trigger: Some(Trigger {
                    time_before_expiry: Some("P30D".into()),
                    ..Default::default()
                }),
            },
            LifetimeAction {
                action: Some(Action {
                    type_prop: Some(KeyRotationPolicyActionType::Rotate),
                }),
                trigger: Some(Trigger {
                    time_after_create: Some("P90D".into()),
                    ..Default::default()
                }),
            },
        ]),
    };
    let key = keys
        .create_if_not_exist(
            &group,
            &vault,
            &name,
            KeyCreateParameters {
                properties: Some(KeyProperties {
                    kty: Some(JsonWebKeyType::Rsa),
                    key_size: Some(2048),
                    rotation_policy: Some(policy),
                    ..Default::default()
                }),
                ..Default::default()
            }
            .try_into()?,
            None,
        )
        .await?
        .into_model()?;
    let version = key_version(&key)?;
    let (key, raw) = model_and_json(keys.get(&group, &vault, &name, None).await?)?;
    let actions = key
        .properties
        .unwrap()
        .rotation_policy
        .unwrap()
        .lifetime_actions
        .unwrap();
    let raw_actions = raw["properties"]["rotationPolicy"]["lifetimeActions"]
        .as_array()
        .unwrap();
    assert_eq!(actions.len(), 2);
    for (expected, wire) in [
        (KeyRotationPolicyActionType::Notify, "Notify"),
        (KeyRotationPolicyActionType::Rotate, "Rotate"),
    ] {
        assert!(actions.iter().any(|action| action
            .action
            .as_ref()
            .and_then(|action| action.type_prop)
            == Some(expected)));
        assert!(raw_actions
            .iter()
            .any(|action| action["action"]["type"] == wire));
    }
    let versioned = keys
        .get_version(&group, &vault, &name, &version, None)
        .await?
        .into_model()?;
    assert_eq!(key_version(&versioned)?, version);
    assert_eq!(versioned.properties.unwrap().kty, Some(JsonWebKeyType::Rsa));
    client
        .get_key_vault_vaults_client()
        .delete(&group, &vault, None)
        .await?;
    Ok(())
}

#[recorded::test]
async fn vault_network_rules(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let client = client(&ctx).await?;
    let group = recording.var("KEYVAULT_RESOURCE_GROUP", None);
    let name = recording.random_string::<16>(Some("v"));
    create_vault(&ctx, &client, &name).await?;
    let vaults = client.get_key_vault_vaults_client();
    for (action, bypass) in [
        (NetworkRuleAction::Deny, NetworkRuleBypassOptions::None),
        (
            NetworkRuleAction::Deny,
            NetworkRuleBypassOptions::AzureServices,
        ),
        (NetworkRuleAction::Allow, NetworkRuleBypassOptions::None),
        (
            NetworkRuleAction::Allow,
            NetworkRuleBypassOptions::AzureServices,
        ),
    ] {
        let (vault, raw) = model_and_json(
            vaults
                .update(
                    &group,
                    &name,
                    VaultPatchParameters {
                        properties: Some(VaultPatchProperties {
                            network_acls: Some(NetworkRuleSet {
                                default_action: Some(action.clone()),
                                bypass: Some(bypass.clone()),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }
                    .try_into()?,
                    None,
                )
                .await?,
        )?;
        for (vault, raw) in [
            (vault, raw),
            model_and_json(vaults.get(&group, &name, None).await?)?,
        ] {
            let rules = vault.properties.unwrap().network_acls;
            let Some(rules) = rules else {
                // The service omits ACLs when restoring both defaults.
                assert_eq!(action, NetworkRuleAction::Allow);
                assert_eq!(bypass, NetworkRuleBypassOptions::AzureServices);
                assert!(raw["properties"]["networkAcls"].is_null());
                continue;
            };
            assert_eq!(rules.default_action.as_ref(), Some(&action));
            assert_eq!(rules.bypass.as_ref(), Some(&bypass));
            assert_eq!(
                raw["properties"]["networkAcls"]["defaultAction"].as_str(),
                Some(action.as_ref())
            );
            assert_eq!(
                raw["properties"]["networkAcls"]["bypass"].as_str(),
                Some(bypass.as_ref())
            );
        }
    }
    vaults.delete(&group, &name, None).await?;
    Ok(())
}

#[recorded::test]
async fn vault_access_policies(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let client = client(&ctx).await?;
    let group = recording.var("KEYVAULT_RESOURCE_GROUP", None);
    let name = recording.random_string::<16>(Some("v"));
    let mut parameters = vault_parameters(&ctx)?;
    parameters
        .properties
        .as_mut()
        .unwrap()
        .enable_rbac_authorization = Some(false);
    let vaults = client.get_key_vault_vaults_client();
    let (vault, raw) = model_and_json(
        vaults
            .begin_create_or_update(&group, &name, parameters.try_into()?, None)?
            .await?,
    )?;
    assert_vault(&vault, &raw);
    let permissions = Permissions {
        keys: Some(vec![
            KeyPermissions::Getrotationpolicy,
            KeyPermissions::Setrotationpolicy,
            KeyPermissions::WrapKey,
            KeyPermissions::UnwrapKey,
        ]),
        secrets: Some(vec![SecretPermissions::Get, SecretPermissions::Set]),
        certificates: Some(vec![
            CertificatePermissions::Getissuers,
            CertificatePermissions::Managecontacts,
            CertificatePermissions::Manageissuers,
        ]),
        storage: Some(vec![
            StoragePermissions::Getsas,
            StoragePermissions::Setsas,
            StoragePermissions::Regeneratekey,
        ]),
    };
    let mut entry = AccessPolicyEntry {
        tenant_id: Some(recording.var("KEYVAULT_TENANT_ID", None).parse()?),
        object_id: Some(recording.var("KEYVAULT_OBJECT_ID", None)),
        permissions: Some(Permissions {
            keys: Some(vec![KeyPermissions::Get]),
            ..Default::default()
        }),
        ..Default::default()
    };
    for kind in [
        AccessPolicyUpdateKind::Add,
        AccessPolicyUpdateKind::Replace,
        AccessPolicyUpdateKind::Remove,
    ] {
        if kind == AccessPolicyUpdateKind::Replace {
            entry.permissions = Some(permissions.clone());
        }
        vaults
            .update_access_policy(
                &group,
                &name,
                kind,
                VaultAccessPolicyParameters {
                    properties: Some(VaultAccessPolicyProperties {
                        access_policies: Some(vec![entry.clone()]),
                    }),
                    ..Default::default()
                }
                .try_into()?,
                None,
            )
            .await?;
        let (vault, raw) = model_and_json(vaults.get(&group, &name, None).await?)?;
        let policies = vault.properties.unwrap().access_policies.unwrap();
        if kind == AccessPolicyUpdateKind::Remove {
            assert!(policies.is_empty());
        } else {
            assert_eq!(policies.len(), 1);
            let expected = entry.permissions.as_ref().unwrap();
            let actual = policies[0].permissions.as_ref().unwrap();
            let raw = &raw["properties"]["accessPolicies"][0]["permissions"];
            assert_enum_values(
                actual.keys.as_deref(),
                expected.keys.as_deref(),
                &raw["keys"],
            );
            assert_enum_values(
                actual.secrets.as_deref(),
                expected.secrets.as_deref(),
                &raw["secrets"],
            );
            assert_enum_values(
                actual.certificates.as_deref(),
                expected.certificates.as_deref(),
                &raw["certificates"],
            );
            assert_enum_values(
                actual.storage.as_deref(),
                expected.storage.as_deref(),
                &raw["storage"],
            );
        }
    }
    vaults.delete(&group, &name, None).await?;
    Ok(())
}

#[recorded::test]
async fn vault_recovery(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let client = client(&ctx).await?;
    let group = recording.var("KEYVAULT_RESOURCE_GROUP", None);
    let location = recording.var("KEYVAULT_LOCATION", None);
    let name = recording.random_string::<16>(Some("v"));
    create_vault(&ctx, &client, &name).await?;
    let vaults = client.get_key_vault_vaults_client();
    vaults.delete(&group, &name, None).await?;
    let mut retry = match recording.test_mode() {
        TestMode::Playback => Retry::immediate(),
        _ => Retry::progressive(None),
    };
    for attempt in 0..10 {
        match vaults.get_deleted(&location, &name, None).await {
            Ok(response) => {
                assert_eq!(response.into_model()?.name.as_deref(), Some(name.as_str()));
                break;
            }
            Err(error) if error.http_status() == Some(StatusCode::NotFound) => {
                eprintln!("Waiting for soft-deleted test vault to become visible");
                if attempt == 9 || retry.next().await.is_none() {
                    return Err(error);
                }
            }
            Err(error) => return Err(error),
        }
    }
    let mut parameters = vault_parameters(&ctx)?;
    parameters.properties.as_mut().unwrap().create_mode = Some(CreateMode::Recover);
    let (vault, raw) = model_and_json(
        vaults
            .begin_create_or_update(&group, &name, parameters.try_into()?, None)?
            .await?,
    )?;
    assert_vault(&vault, &raw);
    let (vault, raw) = model_and_json(vaults.get(&group, &name, None).await?)?;
    assert_vault(&vault, &raw);
    vaults.delete(&group, &name, None).await?;
    Ok(())
}

#[recorded::test]
#[ignore = "the service returns undeclared reason Invalid; awaiting a TypeSpec contract correction"]
async fn vault_name_availability(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let client = client(&ctx).await?;
    let endpoint: Url = recording.var("AZURE_KEYVAULT_URL", None).parse()?;
    let name = endpoint.host_str().unwrap().split('.').next().unwrap();
    let vaults = client.get_key_vault_vaults_client();
    for (name, expected) in [
        ("invalid_vault_name", Reason::AccountNameInvalid),
        (name, Reason::AlreadyExists),
    ] {
        let (result, raw) = model_and_json(
            vaults
                .check_name_availability(
                    VaultCheckNameAvailabilityParameters {
                        name: Some(name.into()),
                        ..Default::default()
                    }
                    .try_into()?,
                    None,
                )
                .await?,
        )?;
        assert_eq!(result.name_available, Some(false));
        assert_eq!(raw["reason"].as_str(), Some(expected.as_ref()));
        assert_eq!(result.reason, Some(expected));
    }
    Ok(())
}

#[recorded::test]
async fn secret_lifecycle(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    let client = client(&ctx).await?;
    let resource_group = recording.var("KEYVAULT_RESOURCE_GROUP", None);
    let vault_name = recording.random_string::<16>(Some("v"));
    let secret_name = recording.random_string::<16>(Some("s"));
    create_vault(&ctx, &client, &vault_name).await?;
    let secrets = client.get_key_vault_secrets_client();
    let secret = secrets
        .create_or_update(
            &resource_group,
            &vault_name,
            &secret_name,
            SecretCreateOrUpdateParameters {
                properties: Some(SecretProperties {
                    value: Some("test-value".into()),
                    ..Default::default()
                }),
                ..Default::default()
            }
            .try_into()?,
            None,
        )
        .await?
        .into_model()?;
    assert_eq!(secret.name.as_deref(), Some(secret_name.as_str()));
    let mut pager = secrets
        .list(&resource_group, &vault_name, None)?
        .into_stream();
    let mut found = false;
    while let Some(secret) = pager.try_next().await? {
        found |= secret.name.as_deref() == Some(secret_name.as_str());
    }
    assert!(found);
    let secret = secrets
        .update(
            &resource_group,
            &vault_name,
            &secret_name,
            SecretPatchParameters {
                properties: Some(SecretPatchProperties {
                    content_type: Some("text/plain".into()),
                    value: Some("test-value".into()),
                    ..Default::default()
                }),
                tags: Some(HashMap::from([("updated".into(), "true".into())])),
            }
            .try_into()?,
            None,
        )
        .await?
        .into_model()?;
    assert_eq!(
        secret
            .properties
            .as_ref()
            .and_then(|properties| properties.content_type.as_deref()),
        Some("text/plain")
    );
    assert_eq!(
        secret
            .tags
            .as_ref()
            .and_then(|tags| tags.get("updated"))
            .map(String::as_str),
        Some("true")
    );
    let secret = secrets
        .get(&resource_group, &vault_name, &secret_name, None)
        .await?
        .into_model()?;
    assert_eq!(secret.name.as_deref(), Some(secret_name.as_str()));
    client
        .get_key_vault_vaults_client()
        .delete(&resource_group, &vault_name, None)
        .await?;
    Ok(())
}
