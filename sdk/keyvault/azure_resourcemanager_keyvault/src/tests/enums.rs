// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::models::{
    self, AccessPolicyUpdateKind, ActionsRequired, ActivationStatus, CertificatePermissions,
    CreateMode, CreatedByType, DeletionRecoveryLevel, GeoReplicationRegionProvisioningState,
    JsonWebKeyCurveName, JsonWebKeyOperation, JsonWebKeyType, KeyPermissions,
    KeyRotationPolicyActionType, ManagedHsmSkuFamily, ManagedHsmSkuNameV2,
    ManagedServiceIdentityType, NetworkRuleAction, NetworkRuleBypassOptions,
    PrivateEndpointConnectionProvisioningState, PrivateEndpointServiceConnectionStatus,
    ProvisioningState, PublicNetworkAccess, Reason, ResourceProvisioningState, SecretPermissions,
    SkuFamily, SkuName, StoragePermissions, VaultProvisioningState,
};
use azure_core::Result;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};
use std::{fmt::Debug, str::FromStr};

fn check<E>(cases: &[(E, &str)], unknown: Option<fn(String) -> E>) -> Result<()>
where
    E: AsRef<str> + Clone + Debug + DeserializeOwned + PartialEq + Serialize + ToString + FromStr,
    E::Err: Debug,
{
    for (expected, wire) in cases {
        assert_eq!(wire.parse::<E>().unwrap(), *expected, "{wire}");
        assert_eq!(expected.as_ref(), *wire);
        assert_eq!(expected.to_string(), *wire);
        assert_eq!(serde_json::to_value(expected)?, json!(wire));
        assert_eq!(serde_json::from_value::<E>(json!(wire))?, *expected);
        let wrong_case = if wire.to_ascii_lowercase() == *wire {
            wire.to_ascii_uppercase()
        } else {
            wire.to_ascii_lowercase()
        };
        if let Some(unknown) = unknown {
            assert_eq!(
                serde_json::from_value::<E>(json!(wrong_case))?,
                unknown(wrong_case.clone())
            );
            assert_ne!(wrong_case.parse::<E>().unwrap(), *expected);
        } else {
            assert!(wrong_case.parse::<E>().is_err());
            assert!(serde_json::from_value::<E>(json!(wrong_case)).is_err());
        }
    }
    let future = "FutureEnumValue";
    if let Some(unknown) = unknown {
        let expected = unknown(future.into());
        assert_eq!(future.parse::<E>().unwrap(), expected);
        assert_eq!(serde_json::from_value::<E>(json!(future))?, expected);
        assert_eq!(serde_json::to_value(&expected)?, json!(future));
        assert_eq!(expected.as_ref(), future);
        assert_eq!(expected.to_string(), future);
    } else {
        assert!(future.parse::<E>().is_err());
        assert!(serde_json::from_value::<E>(json!(future)).is_err());
    }
    Ok(())
}

#[test]
fn vault_lifecycle_enum_wire_values() -> Result<()> {
    check(&[(SkuFamily::A, "A")], Some(SkuFamily::UnknownValue))?;
    check(
        &[
            (SkuName::Standard, "standard"),
            (SkuName::Premium, "premium"),
        ],
        None,
    )?;
    check(
        &[
            (CreateMode::Default, "default"),
            (CreateMode::Recover, "recover"),
        ],
        None,
    )?;
    check(
        &[
            (VaultProvisioningState::RegisteringDns, "RegisteringDns"),
            (VaultProvisioningState::Succeeded, "Succeeded"),
        ],
        Some(VaultProvisioningState::UnknownValue),
    )?;
    let properties: models::VaultProperties = serde_json::from_value(json!({
        "createMode": "recover",
        "sku": {"family": "A", "name": "standard"},
        "provisioningState": "Succeeded",
    }))?;
    assert_eq!(properties.create_mode, Some(CreateMode::Recover));
    assert_eq!(
        properties.provisioning_state,
        Some(VaultProvisioningState::Succeeded)
    );
    let value = serde_json::to_value(properties)?;
    assert_eq!(value["createMode"], "recover");
    assert_eq!(value["sku"]["name"], "standard");
    Ok(())
}

#[test]
fn access_policy_enum_wire_values() -> Result<()> {
    check(
        &[
            (AccessPolicyUpdateKind::Add, "add"),
            (AccessPolicyUpdateKind::Remove, "remove"),
            (AccessPolicyUpdateKind::Replace, "replace"),
        ],
        None,
    )?;
    check(
        &[
            (KeyPermissions::All, "all"),
            (KeyPermissions::Backup, "backup"),
            (KeyPermissions::Create, "create"),
            (KeyPermissions::Decrypt, "decrypt"),
            (KeyPermissions::Delete, "delete"),
            (KeyPermissions::Encrypt, "encrypt"),
            (KeyPermissions::Get, "get"),
            (KeyPermissions::Getrotationpolicy, "getrotationpolicy"),
            (KeyPermissions::Import, "import"),
            (KeyPermissions::List, "list"),
            (KeyPermissions::Purge, "purge"),
            (KeyPermissions::Recover, "recover"),
            (KeyPermissions::Release, "release"),
            (KeyPermissions::Restore, "restore"),
            (KeyPermissions::Rotate, "rotate"),
            (KeyPermissions::Setrotationpolicy, "setrotationpolicy"),
            (KeyPermissions::Sign, "sign"),
            (KeyPermissions::UnwrapKey, "unwrapKey"),
            (KeyPermissions::Update, "update"),
            (KeyPermissions::Verify, "verify"),
            (KeyPermissions::WrapKey, "wrapKey"),
        ],
        Some(KeyPermissions::UnknownValue),
    )?;
    check(
        &[
            (SecretPermissions::All, "all"),
            (SecretPermissions::Backup, "backup"),
            (SecretPermissions::Delete, "delete"),
            (SecretPermissions::Get, "get"),
            (SecretPermissions::List, "list"),
            (SecretPermissions::Purge, "purge"),
            (SecretPermissions::Recover, "recover"),
            (SecretPermissions::Restore, "restore"),
            (SecretPermissions::Set, "set"),
        ],
        Some(SecretPermissions::UnknownValue),
    )?;
    check(
        &[
            (CertificatePermissions::All, "all"),
            (CertificatePermissions::Backup, "backup"),
            (CertificatePermissions::Create, "create"),
            (CertificatePermissions::Delete, "delete"),
            (CertificatePermissions::Deleteissuers, "deleteissuers"),
            (CertificatePermissions::Get, "get"),
            (CertificatePermissions::Getissuers, "getissuers"),
            (CertificatePermissions::Import, "import"),
            (CertificatePermissions::List, "list"),
            (CertificatePermissions::Listissuers, "listissuers"),
            (CertificatePermissions::Managecontacts, "managecontacts"),
            (CertificatePermissions::Manageissuers, "manageissuers"),
            (CertificatePermissions::Purge, "purge"),
            (CertificatePermissions::Recover, "recover"),
            (CertificatePermissions::Restore, "restore"),
            (CertificatePermissions::Setissuers, "setissuers"),
            (CertificatePermissions::Update, "update"),
        ],
        Some(CertificatePermissions::UnknownValue),
    )?;
    check(
        &[
            (StoragePermissions::All, "all"),
            (StoragePermissions::Backup, "backup"),
            (StoragePermissions::Delete, "delete"),
            (StoragePermissions::Deletesas, "deletesas"),
            (StoragePermissions::Get, "get"),
            (StoragePermissions::Getsas, "getsas"),
            (StoragePermissions::List, "list"),
            (StoragePermissions::Listsas, "listsas"),
            (StoragePermissions::Purge, "purge"),
            (StoragePermissions::Recover, "recover"),
            (StoragePermissions::Regeneratekey, "regeneratekey"),
            (StoragePermissions::Restore, "restore"),
            (StoragePermissions::Set, "set"),
            (StoragePermissions::Setsas, "setsas"),
            (StoragePermissions::Update, "update"),
        ],
        Some(StoragePermissions::UnknownValue),
    )?;
    let permissions = models::Permissions {
        keys: Some(vec![KeyPermissions::WrapKey]),
        secrets: Some(vec![SecretPermissions::Set]),
        certificates: Some(vec![CertificatePermissions::Manageissuers]),
        storage: Some(vec![StoragePermissions::Getsas]),
    };
    let value = serde_json::to_value(&permissions)?;
    assert_eq!(
        value,
        json!({
            "keys": ["wrapKey"], "secrets": ["set"],
            "certificates": ["manageissuers"], "storage": ["getsas"],
        })
    );
    let decoded: models::Permissions = serde_json::from_value(value)?;
    assert_eq!(decoded.keys, permissions.keys);
    assert_eq!(decoded.storage, permissions.storage);
    Ok(())
}

#[test]
fn resource_identity_enum_wire_values() -> Result<()> {
    check(
        &[
            (CreatedByType::Application, "Application"),
            (CreatedByType::Key, "Key"),
            (CreatedByType::ManagedIdentity, "ManagedIdentity"),
            (CreatedByType::User, "User"),
        ],
        Some(CreatedByType::UnknownValue),
    )?;
    check(
        &[
            (ManagedServiceIdentityType::None, "None"),
            (ManagedServiceIdentityType::SystemAssigned, "SystemAssigned"),
            (ManagedServiceIdentityType::UserAssigned, "UserAssigned"),
            (
                ManagedServiceIdentityType::SystemAssignedUserAssigned,
                "SystemAssigned,UserAssigned",
            ),
        ],
        Some(ManagedServiceIdentityType::UnknownValue),
    )?;
    let hsm: models::ManagedHsm = serde_json::from_value(json!({
        "identity": {"type": "SystemAssigned,UserAssigned"},
        "systemData": {"createdByType": "Application", "lastModifiedByType": "User"},
    }))?;
    assert_eq!(
        hsm.identity.as_ref().unwrap().type_prop,
        Some(ManagedServiceIdentityType::SystemAssignedUserAssigned)
    );
    assert_eq!(
        hsm.system_data.as_ref().unwrap().created_by_type,
        Some(CreatedByType::Application)
    );
    assert_eq!(
        serde_json::to_value(hsm)?["identity"]["type"],
        "SystemAssigned,UserAssigned"
    );
    Ok(())
}

#[test]
fn network_rule_enum_wire_values() -> Result<()> {
    check(
        &[
            (NetworkRuleAction::Allow, "Allow"),
            (NetworkRuleAction::Deny, "Deny"),
        ],
        Some(NetworkRuleAction::UnknownValue),
    )?;
    check(
        &[
            (NetworkRuleBypassOptions::AzureServices, "AzureServices"),
            (NetworkRuleBypassOptions::None, "None"),
        ],
        Some(NetworkRuleBypassOptions::UnknownValue),
    )?;
    check(
        &[
            (PublicNetworkAccess::Disabled, "Disabled"),
            (PublicNetworkAccess::Enabled, "Enabled"),
        ],
        Some(PublicNetworkAccess::UnknownValue),
    )?;
    let value = json!({"defaultAction":"Deny","bypass":"AzureServices"});
    let vault: models::NetworkRuleSet = serde_json::from_value(value.clone())?;
    let hsm: models::MHSMNetworkRuleSet = serde_json::from_value(value.clone())?;
    assert_eq!(vault.default_action, Some(NetworkRuleAction::Deny));
    assert_eq!(hsm.bypass, Some(NetworkRuleBypassOptions::AzureServices));
    assert_eq!(serde_json::to_value(vault)?, value);
    assert_eq!(serde_json::to_value(hsm)?, value);
    Ok(())
}

#[test]
fn key_algorithm_enum_wire_values() -> Result<()> {
    check(
        &[
            (JsonWebKeyType::Ec, "EC"),
            (JsonWebKeyType::EcHsm, "EC-HSM"),
            (JsonWebKeyType::OctHsm, "oct-HSM"),
            (JsonWebKeyType::Rsa, "RSA"),
            (JsonWebKeyType::RsaHsm, "RSA-HSM"),
        ],
        Some(JsonWebKeyType::UnknownValue),
    )?;
    check(
        &[
            (JsonWebKeyCurveName::P256, "P-256"),
            (JsonWebKeyCurveName::P256K, "P-256K"),
            (JsonWebKeyCurveName::P384, "P-384"),
            (JsonWebKeyCurveName::P521, "P-521"),
        ],
        Some(JsonWebKeyCurveName::UnknownValue),
    )?;
    check(
        &[
            (JsonWebKeyOperation::Decrypt, "decrypt"),
            (JsonWebKeyOperation::Encrypt, "encrypt"),
            (JsonWebKeyOperation::Import, "import"),
            (JsonWebKeyOperation::Release, "release"),
            (JsonWebKeyOperation::Sign, "sign"),
            (JsonWebKeyOperation::UnwrapKey, "unwrapKey"),
            (JsonWebKeyOperation::Verify, "verify"),
            (JsonWebKeyOperation::WrapKey, "wrapKey"),
        ],
        Some(JsonWebKeyOperation::UnknownValue),
    )?;
    let value = json!({"kty":"EC","curveName":"P-256K","keyOps":["sign","verify"]});
    let vault: models::KeyProperties = serde_json::from_value(value.clone())?;
    let hsm: models::ManagedHsmKeyProperties = serde_json::from_value(value.clone())?;
    assert_eq!(vault.kty, Some(JsonWebKeyType::Ec));
    assert_eq!(hsm.curve_name, Some(JsonWebKeyCurveName::P256K));
    assert_eq!(serde_json::to_value(vault)?, value);
    assert_eq!(serde_json::to_value(hsm)?, value);
    Ok(())
}

#[test]
fn key_policy_enum_wire_values() -> Result<()> {
    // The service returned Notify; the pinned specification declares notify/rotate.
    check(
        &[
            (KeyRotationPolicyActionType::Notify, "Notify"),
            (KeyRotationPolicyActionType::Rotate, "Rotate"),
        ],
        None,
    )?;
    check(
        &[
            (DeletionRecoveryLevel::Purgeable, "Purgeable"),
            (DeletionRecoveryLevel::Recoverable, "Recoverable"),
            (
                DeletionRecoveryLevel::RecoverableProtectedSubscription,
                "Recoverable+ProtectedSubscription",
            ),
            (
                DeletionRecoveryLevel::RecoverablePurgeable,
                "Recoverable+Purgeable",
            ),
        ],
        Some(DeletionRecoveryLevel::UnknownValue),
    )?;
    for wire in ["Notify", "Rotate"] {
        let value = json!({"type": wire});
        let vault: models::Action = serde_json::from_value(value.clone())?;
        let hsm: models::ManagedHsmAction = serde_json::from_value(value.clone())?;
        assert_eq!(serde_json::to_value(vault)?, value);
        assert_eq!(serde_json::to_value(hsm)?, value);
    }
    let value = json!({"recoveryLevel":"Recoverable+Purgeable"});
    let vault: models::KeyAttributes = serde_json::from_value(value.clone())?;
    let hsm: models::ManagedHsmKeyAttributes = serde_json::from_value(value)?;
    assert_eq!(
        vault.recovery_level,
        Some(DeletionRecoveryLevel::RecoverablePurgeable)
    );
    assert_eq!(hsm.recovery_level, vault.recovery_level);
    Ok(())
}

#[test]
fn private_endpoint_enum_wire_values() -> Result<()> {
    check(
        &[(ActionsRequired::None, "None")],
        Some(ActionsRequired::UnknownValue),
    )?;
    check(
        &[
            (
                PrivateEndpointConnectionProvisioningState::Creating,
                "Creating",
            ),
            (
                PrivateEndpointConnectionProvisioningState::Deleting,
                "Deleting",
            ),
            (
                PrivateEndpointConnectionProvisioningState::Disconnected,
                "Disconnected",
            ),
            (PrivateEndpointConnectionProvisioningState::Failed, "Failed"),
            (
                PrivateEndpointConnectionProvisioningState::Succeeded,
                "Succeeded",
            ),
            (
                PrivateEndpointConnectionProvisioningState::Updating,
                "Updating",
            ),
        ],
        Some(PrivateEndpointConnectionProvisioningState::UnknownValue),
    )?;
    check(
        &[
            (PrivateEndpointServiceConnectionStatus::Approved, "Approved"),
            (
                PrivateEndpointServiceConnectionStatus::Disconnected,
                "Disconnected",
            ),
            (PrivateEndpointServiceConnectionStatus::Pending, "Pending"),
            (PrivateEndpointServiceConnectionStatus::Rejected, "Rejected"),
        ],
        Some(PrivateEndpointServiceConnectionStatus::UnknownValue),
    )?;
    let value = json!({"properties":{
        "provisioningState":"Succeeded",
        "privateLinkServiceConnectionState":{"status":"Approved","actionsRequired":"None"},
    }});
    let vault: models::PrivateEndpointConnection = serde_json::from_value(value.clone())?;
    let hsm: models::MHSMPrivateEndpointConnection = serde_json::from_value(value)?;
    let vault = vault.properties.unwrap();
    let hsm = hsm.properties.unwrap();
    assert_eq!(
        vault.provisioning_state,
        Some(PrivateEndpointConnectionProvisioningState::Succeeded)
    );
    assert_eq!(hsm.provisioning_state, vault.provisioning_state);
    let state = vault.private_link_service_connection_state.unwrap();
    assert_eq!(
        state.status,
        Some(PrivateEndpointServiceConnectionStatus::Approved)
    );
    assert_eq!(state.actions_required, Some(ActionsRequired::None));
    assert_eq!(serde_json::to_value(state)?["status"], "Approved");
    Ok(())
}

#[test]
fn managed_hsm_sku_enum_wire_values() -> Result<()> {
    check(
        &[(ManagedHsmSkuFamily::B, "B"), (ManagedHsmSkuFamily::C, "C")],
        Some(ManagedHsmSkuFamily::UnknownValue),
    )?;
    check(
        &[
            (ManagedHsmSkuNameV2::CustomB32, "Custom_B32"),
            (ManagedHsmSkuNameV2::CustomB6, "Custom_B6"),
            (ManagedHsmSkuNameV2::CustomC10, "Custom_C10"),
            (ManagedHsmSkuNameV2::CustomC42, "Custom_C42"),
            (ManagedHsmSkuNameV2::StandardB1, "Standard_B1"),
            (ManagedHsmSkuNameV2::StandardB10V2, "Standard_B10v2"),
            (ManagedHsmSkuNameV2::StandardB15V2, "Standard_B15v2"),
            (ManagedHsmSkuNameV2::StandardB1V2, "Standard_B1v2"),
            (ManagedHsmSkuNameV2::StandardB20V2, "Standard_B20v2"),
            (ManagedHsmSkuNameV2::StandardB5V2, "Standard_B5v2"),
        ],
        Some(ManagedHsmSkuNameV2::UnknownValue),
    )?;
    let value = json!({"family":"B","name":"Standard_B10v2"});
    let sku: models::ManagedHsmSku = serde_json::from_value(value.clone())?;
    assert_eq!(sku.name, Some(ManagedHsmSkuNameV2::StandardB10V2));
    assert_eq!(serde_json::to_value(sku)?, value);
    Ok(())
}

#[test]
fn managed_hsm_state_enum_wire_values() -> Result<()> {
    check(
        &[
            (ActivationStatus::Active, "Active"),
            (ActivationStatus::Failed, "Failed"),
            (ActivationStatus::NotActivated, "NotActivated"),
            (ActivationStatus::Unknown, "Unknown"),
        ],
        Some(ActivationStatus::UnknownValue),
    )?;
    check(
        &[
            (ProvisioningState::Activated, "Activated"),
            (ProvisioningState::Deleting, "Deleting"),
            (ProvisioningState::Failed, "Failed"),
            (ProvisioningState::Provisioning, "Provisioning"),
            (ProvisioningState::Restoring, "Restoring"),
            (
                ProvisioningState::SecurityDomainRestore,
                "SecurityDomainRestore",
            ),
            (ProvisioningState::Succeeded, "Succeeded"),
            (ProvisioningState::Updating, "Updating"),
        ],
        Some(ProvisioningState::UnknownValue),
    )?;
    check(
        &[
            (GeoReplicationRegionProvisioningState::Cleanup, "Cleanup"),
            (GeoReplicationRegionProvisioningState::Deleting, "Deleting"),
            (GeoReplicationRegionProvisioningState::Failed, "Failed"),
            (
                GeoReplicationRegionProvisioningState::Preprovisioning,
                "Preprovisioning",
            ),
            (
                GeoReplicationRegionProvisioningState::Provisioning,
                "Provisioning",
            ),
            (
                GeoReplicationRegionProvisioningState::Succeeded,
                "Succeeded",
            ),
        ],
        Some(GeoReplicationRegionProvisioningState::UnknownValue),
    )?;
    let hsm: models::ManagedHsmProperties = serde_json::from_value(json!({
        "provisioningState":"Succeeded",
        "securityDomainProperties":{"activationStatus":"NotActivated"},
        "regions":[{"provisioningState":"Succeeded"}],
    }))?;
    assert_eq!(hsm.provisioning_state, Some(ProvisioningState::Succeeded));
    assert_eq!(
        hsm.security_domain_properties.unwrap().activation_status,
        Some(ActivationStatus::NotActivated)
    );
    assert_eq!(
        hsm.regions.unwrap()[0].provisioning_state,
        Some(GeoReplicationRegionProvisioningState::Succeeded)
    );
    Ok(())
}

#[test]
fn name_availability_enum_wire_values() -> Result<()> {
    check(
        &[
            (Reason::AccountNameInvalid, "AccountNameInvalid"),
            (Reason::AlreadyExists, "AlreadyExists"),
        ],
        Some(Reason::UnknownValue),
    )?;
    for (wire, expected) in [
        ("AccountNameInvalid", Reason::AccountNameInvalid),
        ("AlreadyExists", Reason::AlreadyExists),
    ] {
        let value = json!({"nameAvailable":false,"reason":wire});
        let vault: models::CheckNameAvailabilityResult = serde_json::from_value(value.clone())?;
        let hsm: models::CheckMhsmNameAvailabilityResult = serde_json::from_value(value)?;
        assert_eq!(vault.reason, Some(expected));
        assert_eq!(hsm.reason, vault.reason);
    }
    Ok(())
}

#[test]
fn operation_status_enum_wire_values() -> Result<()> {
    check(
        &[
            (ResourceProvisioningState::Canceled, "Canceled"),
            (ResourceProvisioningState::Failed, "Failed"),
            (ResourceProvisioningState::Succeeded, "Succeeded"),
        ],
        Some(ResourceProvisioningState::UnknownValue),
    )?;
    for (wire, expected) in [
        ("Canceled", ResourceProvisioningState::Canceled),
        ("Failed", ResourceProvisioningState::Failed),
        ("Succeeded", ResourceProvisioningState::Succeeded),
    ] {
        let value: Value = json!({"status":wire});
        let vault: models::KeyVaultVaultsClientBeginCreateOrUpdateOperationStatus =
            serde_json::from_value(value.clone())?;
        let hsm: models::KeyVaultManagedHsmsClientBeginDeleteOperationStatus =
            serde_json::from_value(value)?;
        assert_eq!(vault.status, Some(expected));
        assert_eq!(hsm.status, vault.status);
    }
    Ok(())
}
