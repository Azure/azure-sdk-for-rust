#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "A2A add disk(s) input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aAddDisksInput {
    #[serde(flatten)]
    pub add_disks_provider_specific_input: AddDisksProviderSpecificInput,
    #[doc = "The list of vm disk details."]
    #[serde(rename = "vmDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_disks: Vec<A2aVmDiskInputDetails>,
    #[doc = "The list of vm managed disk details."]
    #[serde(rename = "vmManagedDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_managed_disks: Vec<A2aVmManagedDiskInputDetails>,
}
impl A2aAddDisksInput {
    pub fn new(add_disks_provider_specific_input: AddDisksProviderSpecificInput) -> Self {
        Self {
            add_disks_provider_specific_input,
            vm_disks: Vec::new(),
            vm_managed_disks: Vec::new(),
        }
    }
}
#[doc = "ApplyRecoveryPoint input specific to A2A provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aApplyRecoveryPointInput {
    #[serde(flatten)]
    pub apply_recovery_point_provider_specific_input: ApplyRecoveryPointProviderSpecificInput,
}
impl A2aApplyRecoveryPointInput {
    pub fn new(apply_recovery_point_provider_specific_input: ApplyRecoveryPointProviderSpecificInput) -> Self {
        Self {
            apply_recovery_point_provider_specific_input,
        }
    }
}
#[doc = "A2A cloud creation input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aContainerCreationInput {
    #[serde(flatten)]
    pub replication_provider_specific_container_creation_input: ReplicationProviderSpecificContainerCreationInput,
}
impl A2aContainerCreationInput {
    pub fn new(replication_provider_specific_container_creation_input: ReplicationProviderSpecificContainerCreationInput) -> Self {
        Self {
            replication_provider_specific_container_creation_input,
        }
    }
}
#[doc = "A2A container mapping input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aContainerMappingInput {
    #[serde(flatten)]
    pub replication_provider_specific_container_mapping_input: ReplicationProviderSpecificContainerMappingInput,
    #[doc = "A value indicating whether the auto update is enabled."]
    #[serde(rename = "agentAutoUpdateStatus", default, skip_serializing_if = "Option::is_none")]
    pub agent_auto_update_status: Option<a2a_container_mapping_input::AgentAutoUpdateStatus>,
    #[doc = "The automation account arm id."]
    #[serde(rename = "automationAccountArmId", default, skip_serializing_if = "Option::is_none")]
    pub automation_account_arm_id: Option<String>,
    #[doc = "A value indicating the type authentication to use for automation Account."]
    #[serde(rename = "automationAccountAuthenticationType", default, skip_serializing_if = "Option::is_none")]
    pub automation_account_authentication_type: Option<a2a_container_mapping_input::AutomationAccountAuthenticationType>,
}
impl A2aContainerMappingInput {
    pub fn new(replication_provider_specific_container_mapping_input: ReplicationProviderSpecificContainerMappingInput) -> Self {
        Self {
            replication_provider_specific_container_mapping_input,
            agent_auto_update_status: None,
            automation_account_arm_id: None,
            automation_account_authentication_type: None,
        }
    }
}
pub mod a2a_container_mapping_input {
    use super::*;
    #[doc = "A value indicating whether the auto update is enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AgentAutoUpdateStatus")]
    pub enum AgentAutoUpdateStatus {
        Disabled,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AgentAutoUpdateStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AgentAutoUpdateStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AgentAutoUpdateStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("AgentAutoUpdateStatus", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("AgentAutoUpdateStatus", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "A value indicating the type authentication to use for automation Account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AutomationAccountAuthenticationType")]
    pub enum AutomationAccountAuthenticationType {
        RunAsAccount,
        SystemAssignedIdentity,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AutomationAccountAuthenticationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AutomationAccountAuthenticationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AutomationAccountAuthenticationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::RunAsAccount => serializer.serialize_unit_variant("AutomationAccountAuthenticationType", 0u32, "RunAsAccount"),
                Self::SystemAssignedIdentity => {
                    serializer.serialize_unit_variant("AutomationAccountAuthenticationType", 1u32, "SystemAssignedIdentity")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for AutomationAccountAuthenticationType {
        fn default() -> Self {
            Self::RunAsAccount
        }
    }
}
#[doc = "A2A create protection intent input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aCreateProtectionIntentInput {
    #[serde(flatten)]
    pub create_protection_intent_provider_specific_details: CreateProtectionIntentProviderSpecificDetails,
    #[doc = "The fabric specific object Id of the virtual machine."]
    #[serde(rename = "fabricObjectId")]
    pub fabric_object_id: String,
    #[doc = "The primary location for the virtual machine."]
    #[serde(rename = "primaryLocation")]
    pub primary_location: String,
    #[doc = "The recovery location for the virtual machine."]
    #[serde(rename = "recoveryLocation")]
    pub recovery_location: String,
    #[doc = "The recovery subscription Id of the virtual machine."]
    #[serde(rename = "recoverySubscriptionId")]
    pub recovery_subscription_id: String,
    #[doc = "The recovery availability type of the virtual machine."]
    #[serde(rename = "recoveryAvailabilityType")]
    pub recovery_availability_type: a2a_create_protection_intent_input::RecoveryAvailabilityType,
    #[doc = "Protection Profile custom input."]
    #[serde(rename = "protectionProfileCustomInput", default, skip_serializing_if = "Option::is_none")]
    pub protection_profile_custom_input: Option<ProtectionProfileCustomDetails>,
    #[doc = "The recovery resource group Id. Valid for V2 scenarios."]
    #[serde(rename = "recoveryResourceGroupId")]
    pub recovery_resource_group_id: String,
    #[doc = "Storage account custom input."]
    #[serde(
        rename = "primaryStagingStorageAccountCustomInput",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_staging_storage_account_custom_input: Option<StorageAccountCustomDetails>,
    #[doc = "Recovery Availability Set custom input."]
    #[serde(rename = "recoveryAvailabilitySetCustomInput", default, skip_serializing_if = "Option::is_none")]
    pub recovery_availability_set_custom_input: Option<RecoveryAvailabilitySetCustomDetails>,
    #[doc = "Recovery Virtual network custom input."]
    #[serde(rename = "recoveryVirtualNetworkCustomInput", default, skip_serializing_if = "Option::is_none")]
    pub recovery_virtual_network_custom_input: Option<RecoveryVirtualNetworkCustomDetails>,
    #[doc = "Recovery Proximity placement group custom input."]
    #[serde(
        rename = "recoveryProximityPlacementGroupCustomInput",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recovery_proximity_placement_group_custom_input: Option<RecoveryProximityPlacementGroupCustomDetails>,
    #[doc = "A value indicating whether the auto protection is enabled."]
    #[serde(rename = "autoProtectionOfDataDisk", default, skip_serializing_if = "Option::is_none")]
    pub auto_protection_of_data_disk: Option<a2a_create_protection_intent_input::AutoProtectionOfDataDisk>,
    #[doc = "The list of vm disk inputs."]
    #[serde(rename = "vmDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_disks: Vec<A2aProtectionIntentDiskInputDetails>,
    #[doc = "The list of vm managed disk inputs."]
    #[serde(rename = "vmManagedDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_managed_disks: Vec<A2aProtectionIntentManagedDiskInputDetails>,
    #[doc = "The multi vm group name."]
    #[serde(rename = "multiVmGroupName", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_group_name: Option<String>,
    #[doc = "The multi vm group id."]
    #[serde(rename = "multiVmGroupId", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_group_id: Option<String>,
    #[doc = "Storage account custom input."]
    #[serde(rename = "recoveryBootDiagStorageAccount", default, skip_serializing_if = "Option::is_none")]
    pub recovery_boot_diag_storage_account: Option<StorageAccountCustomDetails>,
    #[doc = "Recovery disk encryption info (BEK and KEK)."]
    #[serde(rename = "diskEncryptionInfo", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_info: Option<DiskEncryptionInfo>,
    #[doc = "The recovery availability zone."]
    #[serde(rename = "recoveryAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub recovery_availability_zone: Option<String>,
    #[doc = "A value indicating whether the auto update is enabled."]
    #[serde(rename = "agentAutoUpdateStatus", default, skip_serializing_if = "Option::is_none")]
    pub agent_auto_update_status: Option<a2a_create_protection_intent_input::AgentAutoUpdateStatus>,
    #[doc = "A value indicating the authentication type for automation account. The default value is \"RunAsAccount\"."]
    #[serde(rename = "automationAccountAuthenticationType", default, skip_serializing_if = "Option::is_none")]
    pub automation_account_authentication_type: Option<a2a_create_protection_intent_input::AutomationAccountAuthenticationType>,
    #[doc = "The automation account arm id."]
    #[serde(rename = "automationAccountArmId", default, skip_serializing_if = "Option::is_none")]
    pub automation_account_arm_id: Option<String>,
}
impl A2aCreateProtectionIntentInput {
    pub fn new(
        create_protection_intent_provider_specific_details: CreateProtectionIntentProviderSpecificDetails,
        fabric_object_id: String,
        primary_location: String,
        recovery_location: String,
        recovery_subscription_id: String,
        recovery_availability_type: a2a_create_protection_intent_input::RecoveryAvailabilityType,
        recovery_resource_group_id: String,
    ) -> Self {
        Self {
            create_protection_intent_provider_specific_details,
            fabric_object_id,
            primary_location,
            recovery_location,
            recovery_subscription_id,
            recovery_availability_type,
            protection_profile_custom_input: None,
            recovery_resource_group_id,
            primary_staging_storage_account_custom_input: None,
            recovery_availability_set_custom_input: None,
            recovery_virtual_network_custom_input: None,
            recovery_proximity_placement_group_custom_input: None,
            auto_protection_of_data_disk: None,
            vm_disks: Vec::new(),
            vm_managed_disks: Vec::new(),
            multi_vm_group_name: None,
            multi_vm_group_id: None,
            recovery_boot_diag_storage_account: None,
            disk_encryption_info: None,
            recovery_availability_zone: None,
            agent_auto_update_status: None,
            automation_account_authentication_type: None,
            automation_account_arm_id: None,
        }
    }
}
pub mod a2a_create_protection_intent_input {
    use super::*;
    #[doc = "The recovery availability type of the virtual machine."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryAvailabilityType")]
    pub enum RecoveryAvailabilityType {
        Single,
        AvailabilitySet,
        AvailabilityZone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryAvailabilityType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryAvailabilityType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryAvailabilityType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Single => serializer.serialize_unit_variant("RecoveryAvailabilityType", 0u32, "Single"),
                Self::AvailabilitySet => serializer.serialize_unit_variant("RecoveryAvailabilityType", 1u32, "AvailabilitySet"),
                Self::AvailabilityZone => serializer.serialize_unit_variant("RecoveryAvailabilityType", 2u32, "AvailabilityZone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "A value indicating whether the auto protection is enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AutoProtectionOfDataDisk")]
    pub enum AutoProtectionOfDataDisk {
        Disabled,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AutoProtectionOfDataDisk {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AutoProtectionOfDataDisk {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AutoProtectionOfDataDisk {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("AutoProtectionOfDataDisk", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("AutoProtectionOfDataDisk", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "A value indicating whether the auto update is enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AgentAutoUpdateStatus")]
    pub enum AgentAutoUpdateStatus {
        Disabled,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AgentAutoUpdateStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AgentAutoUpdateStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AgentAutoUpdateStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("AgentAutoUpdateStatus", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("AgentAutoUpdateStatus", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "A value indicating the authentication type for automation account. The default value is \"RunAsAccount\"."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AutomationAccountAuthenticationType")]
    pub enum AutomationAccountAuthenticationType {
        RunAsAccount,
        SystemAssignedIdentity,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AutomationAccountAuthenticationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AutomationAccountAuthenticationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AutomationAccountAuthenticationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::RunAsAccount => serializer.serialize_unit_variant("AutomationAccountAuthenticationType", 0u32, "RunAsAccount"),
                Self::SystemAssignedIdentity => {
                    serializer.serialize_unit_variant("AutomationAccountAuthenticationType", 1u32, "SystemAssignedIdentity")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for AutomationAccountAuthenticationType {
        fn default() -> Self {
            Self::RunAsAccount
        }
    }
}
#[doc = "ApplyRecoveryPoint input specific to A2ACrossClusterMigration provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aCrossClusterMigrationApplyRecoveryPointInput {
    #[serde(flatten)]
    pub apply_recovery_point_provider_specific_input: ApplyRecoveryPointProviderSpecificInput,
}
impl A2aCrossClusterMigrationApplyRecoveryPointInput {
    pub fn new(apply_recovery_point_provider_specific_input: ApplyRecoveryPointProviderSpecificInput) -> Self {
        Self {
            apply_recovery_point_provider_specific_input,
        }
    }
}
#[doc = "A2ACrossClusterMigration cloud creation input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aCrossClusterMigrationContainerCreationInput {
    #[serde(flatten)]
    pub replication_provider_specific_container_creation_input: ReplicationProviderSpecificContainerCreationInput,
}
impl A2aCrossClusterMigrationContainerCreationInput {
    pub fn new(replication_provider_specific_container_creation_input: ReplicationProviderSpecificContainerCreationInput) -> Self {
        Self {
            replication_provider_specific_container_creation_input,
        }
    }
}
#[doc = "A2A Cross-Cluster Migration enable protection input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aCrossClusterMigrationEnableProtectionInput {
    #[serde(flatten)]
    pub enable_protection_provider_specific_input: EnableProtectionProviderSpecificInput,
    #[doc = "The fabric specific object Id of the virtual machine."]
    #[serde(rename = "fabricObjectId", default, skip_serializing_if = "Option::is_none")]
    pub fabric_object_id: Option<String>,
    #[doc = "The recovery container Id."]
    #[serde(rename = "recoveryContainerId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_container_id: Option<String>,
}
impl A2aCrossClusterMigrationEnableProtectionInput {
    pub fn new(enable_protection_provider_specific_input: EnableProtectionProviderSpecificInput) -> Self {
        Self {
            enable_protection_provider_specific_input,
            fabric_object_id: None,
            recovery_container_id: None,
        }
    }
}
#[doc = "A2A Cross-Cluster Migration Policy creation input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aCrossClusterMigrationPolicyCreationInput {
    #[serde(flatten)]
    pub policy_provider_specific_input: PolicyProviderSpecificInput,
}
impl A2aCrossClusterMigrationPolicyCreationInput {
    pub fn new(policy_provider_specific_input: PolicyProviderSpecificInput) -> Self {
        Self {
            policy_provider_specific_input,
        }
    }
}
#[doc = "A2A provider specific settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aCrossClusterMigrationReplicationDetails {
    #[serde(flatten)]
    pub replication_provider_specific_settings: ReplicationProviderSpecificSettings,
    #[doc = "The fabric specific object Id of the virtual machine."]
    #[serde(rename = "fabricObjectId", default, skip_serializing_if = "Option::is_none")]
    pub fabric_object_id: Option<String>,
    #[doc = "Primary fabric location."]
    #[serde(rename = "primaryFabricLocation", default, skip_serializing_if = "Option::is_none")]
    pub primary_fabric_location: Option<String>,
    #[doc = "The type of operating system."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "The protection state for the vm."]
    #[serde(rename = "vmProtectionState", default, skip_serializing_if = "Option::is_none")]
    pub vm_protection_state: Option<String>,
    #[doc = "The protection state description for the vm."]
    #[serde(rename = "vmProtectionStateDescription", default, skip_serializing_if = "Option::is_none")]
    pub vm_protection_state_description: Option<String>,
    #[doc = "An id associated with the PE that survives actions like switch protection which change the backing PE/CPE objects internally.The lifecycle id gets carried forward to have a link/continuity in being able to have an Id that denotes the \"same\" protected item even though other internal Ids/ARM Id might be changing."]
    #[serde(rename = "lifecycleId", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_id: Option<String>,
}
impl A2aCrossClusterMigrationReplicationDetails {
    pub fn new(replication_provider_specific_settings: ReplicationProviderSpecificSettings) -> Self {
        Self {
            replication_provider_specific_settings,
            fabric_object_id: None,
            primary_fabric_location: None,
            os_type: None,
            vm_protection_state: None,
            vm_protection_state_description: None,
            lifecycle_id: None,
        }
    }
}
#[doc = "A2A enable protection input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aEnableProtectionInput {
    #[serde(flatten)]
    pub enable_protection_provider_specific_input: EnableProtectionProviderSpecificInput,
    #[doc = "The fabric specific object Id of the virtual machine."]
    #[serde(rename = "fabricObjectId")]
    pub fabric_object_id: String,
    #[doc = "The recovery container Id."]
    #[serde(rename = "recoveryContainerId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_container_id: Option<String>,
    #[doc = "The recovery resource group Id. Valid for V2 scenarios."]
    #[serde(rename = "recoveryResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_resource_group_id: Option<String>,
    #[doc = "The recovery cloud service Id. Valid for V1 scenarios."]
    #[serde(rename = "recoveryCloudServiceId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_cloud_service_id: Option<String>,
    #[doc = "The recovery availability set Id."]
    #[serde(rename = "recoveryAvailabilitySetId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_availability_set_id: Option<String>,
    #[doc = "The recovery proximity placement group Id."]
    #[serde(rename = "recoveryProximityPlacementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_proximity_placement_group_id: Option<String>,
    #[doc = "The list of vm disk details."]
    #[serde(rename = "vmDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_disks: Vec<A2aVmDiskInputDetails>,
    #[doc = "The list of vm managed disk details."]
    #[serde(rename = "vmManagedDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_managed_disks: Vec<A2aVmManagedDiskInputDetails>,
    #[doc = "The multi vm group name."]
    #[serde(rename = "multiVmGroupName", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_group_name: Option<String>,
    #[doc = "The multi vm group id."]
    #[serde(rename = "multiVmGroupId", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_group_id: Option<String>,
    #[doc = "The boot diagnostic storage account."]
    #[serde(rename = "recoveryBootDiagStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_boot_diag_storage_account_id: Option<String>,
    #[doc = "Recovery disk encryption info (BEK and KEK)."]
    #[serde(rename = "diskEncryptionInfo", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_info: Option<DiskEncryptionInfo>,
    #[doc = "The recovery availability zone."]
    #[serde(rename = "recoveryAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub recovery_availability_zone: Option<String>,
    #[doc = "Extended location of the resource."]
    #[serde(rename = "recoveryExtendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub recovery_extended_location: Option<ExtendedLocation>,
    #[doc = "The recovery Azure virtual network ARM id."]
    #[serde(rename = "recoveryAzureNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_azure_network_id: Option<String>,
    #[doc = "The recovery subnet name."]
    #[serde(rename = "recoverySubnetName", default, skip_serializing_if = "Option::is_none")]
    pub recovery_subnet_name: Option<String>,
    #[doc = "The virtual machine scale set Id."]
    #[serde(rename = "recoveryVirtualMachineScaleSetId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_virtual_machine_scale_set_id: Option<String>,
    #[doc = "The recovery capacity reservation group Id."]
    #[serde(rename = "recoveryCapacityReservationGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_capacity_reservation_group_id: Option<String>,
}
impl A2aEnableProtectionInput {
    pub fn new(enable_protection_provider_specific_input: EnableProtectionProviderSpecificInput, fabric_object_id: String) -> Self {
        Self {
            enable_protection_provider_specific_input,
            fabric_object_id,
            recovery_container_id: None,
            recovery_resource_group_id: None,
            recovery_cloud_service_id: None,
            recovery_availability_set_id: None,
            recovery_proximity_placement_group_id: None,
            vm_disks: Vec::new(),
            vm_managed_disks: Vec::new(),
            multi_vm_group_name: None,
            multi_vm_group_id: None,
            recovery_boot_diag_storage_account_id: None,
            disk_encryption_info: None,
            recovery_availability_zone: None,
            recovery_extended_location: None,
            recovery_azure_network_id: None,
            recovery_subnet_name: None,
            recovery_virtual_machine_scale_set_id: None,
            recovery_capacity_reservation_group_id: None,
        }
    }
}
#[doc = "Model class for event details of a A2A event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aEventDetails {
    #[serde(flatten)]
    pub event_provider_specific_details: EventProviderSpecificDetails,
    #[doc = "The protected item arm name."]
    #[serde(rename = "protectedItemName", default, skip_serializing_if = "Option::is_none")]
    pub protected_item_name: Option<String>,
    #[doc = "The azure vm arm id."]
    #[serde(rename = "fabricObjectId", default, skip_serializing_if = "Option::is_none")]
    pub fabric_object_id: Option<String>,
    #[doc = "Fabric arm name."]
    #[serde(rename = "fabricName", default, skip_serializing_if = "Option::is_none")]
    pub fabric_name: Option<String>,
    #[doc = "The fabric location."]
    #[serde(rename = "fabricLocation", default, skip_serializing_if = "Option::is_none")]
    pub fabric_location: Option<String>,
    #[doc = "Remote fabric arm name."]
    #[serde(rename = "remoteFabricName", default, skip_serializing_if = "Option::is_none")]
    pub remote_fabric_name: Option<String>,
    #[doc = "Remote fabric location."]
    #[serde(rename = "remoteFabricLocation", default, skip_serializing_if = "Option::is_none")]
    pub remote_fabric_location: Option<String>,
}
impl A2aEventDetails {
    pub fn new(event_provider_specific_details: EventProviderSpecificDetails) -> Self {
        Self {
            event_provider_specific_details,
            protected_item_name: None,
            fabric_object_id: None,
            fabric_name: None,
            fabric_location: None,
            remote_fabric_name: None,
            remote_fabric_location: None,
        }
    }
}
#[doc = "A2A Policy creation input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aPolicyCreationInput {
    #[serde(flatten)]
    pub policy_provider_specific_input: PolicyProviderSpecificInput,
    #[doc = "The duration in minutes until which the recovery points need to be stored."]
    #[serde(rename = "recoveryPointHistory", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_history: Option<i32>,
    #[doc = "The crash consistent snapshot frequency (in minutes)."]
    #[serde(rename = "crashConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub crash_consistent_frequency_in_minutes: Option<i32>,
    #[doc = "The app consistent snapshot frequency (in minutes)."]
    #[serde(rename = "appConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub app_consistent_frequency_in_minutes: Option<i32>,
    #[doc = "A value indicating whether multi-VM sync has to be enabled. Value should be 'Enabled' or 'Disabled'."]
    #[serde(rename = "multiVmSyncStatus")]
    pub multi_vm_sync_status: a2a_policy_creation_input::MultiVmSyncStatus,
}
impl A2aPolicyCreationInput {
    pub fn new(
        policy_provider_specific_input: PolicyProviderSpecificInput,
        multi_vm_sync_status: a2a_policy_creation_input::MultiVmSyncStatus,
    ) -> Self {
        Self {
            policy_provider_specific_input,
            recovery_point_history: None,
            crash_consistent_frequency_in_minutes: None,
            app_consistent_frequency_in_minutes: None,
            multi_vm_sync_status,
        }
    }
}
pub mod a2a_policy_creation_input {
    use super::*;
    #[doc = "A value indicating whether multi-VM sync has to be enabled. Value should be 'Enabled' or 'Disabled'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MultiVmSyncStatus")]
    pub enum MultiVmSyncStatus {
        Enable,
        Disable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MultiVmSyncStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MultiVmSyncStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MultiVmSyncStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enable => serializer.serialize_unit_variant("MultiVmSyncStatus", 0u32, "Enable"),
                Self::Disable => serializer.serialize_unit_variant("MultiVmSyncStatus", 1u32, "Disable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A2A specific policy details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aPolicyDetails {
    #[serde(flatten)]
    pub policy_provider_specific_details: PolicyProviderSpecificDetails,
    #[doc = "The recovery point threshold in minutes."]
    #[serde(rename = "recoveryPointThresholdInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_threshold_in_minutes: Option<i32>,
    #[doc = "The duration in minutes until which the recovery points need to be stored."]
    #[serde(rename = "recoveryPointHistory", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_history: Option<i32>,
    #[doc = "The app consistent snapshot frequency in minutes."]
    #[serde(rename = "appConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub app_consistent_frequency_in_minutes: Option<i32>,
    #[doc = "A value indicating whether multi-VM sync has to be enabled."]
    #[serde(rename = "multiVmSyncStatus", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_sync_status: Option<String>,
    #[doc = "The crash consistent snapshot frequency in minutes."]
    #[serde(rename = "crashConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub crash_consistent_frequency_in_minutes: Option<i32>,
}
impl A2aPolicyDetails {
    pub fn new(policy_provider_specific_details: PolicyProviderSpecificDetails) -> Self {
        Self {
            policy_provider_specific_details,
            recovery_point_threshold_in_minutes: None,
            recovery_point_history: None,
            app_consistent_frequency_in_minutes: None,
            multi_vm_sync_status: None,
            crash_consistent_frequency_in_minutes: None,
        }
    }
}
#[doc = "A2A protected disk details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct A2aProtectedDiskDetails {
    #[doc = "The disk uri."]
    #[serde(rename = "diskUri", default, skip_serializing_if = "Option::is_none")]
    pub disk_uri: Option<String>,
    #[doc = "The recovery disk storage account."]
    #[serde(rename = "recoveryAzureStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_azure_storage_account_id: Option<String>,
    #[doc = "The primary disk storage account."]
    #[serde(rename = "primaryDiskAzureStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub primary_disk_azure_storage_account_id: Option<String>,
    #[doc = "Recovery disk uri."]
    #[serde(rename = "recoveryDiskUri", default, skip_serializing_if = "Option::is_none")]
    pub recovery_disk_uri: Option<String>,
    #[doc = "The disk name."]
    #[serde(rename = "diskName", default, skip_serializing_if = "Option::is_none")]
    pub disk_name: Option<String>,
    #[doc = "The disk capacity in bytes."]
    #[serde(rename = "diskCapacityInBytes", default, skip_serializing_if = "Option::is_none")]
    pub disk_capacity_in_bytes: Option<i64>,
    #[doc = "The primary staging storage account."]
    #[serde(rename = "primaryStagingAzureStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub primary_staging_azure_storage_account_id: Option<String>,
    #[doc = "The type of disk."]
    #[serde(rename = "diskType", default, skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<String>,
    #[doc = "A value indicating whether resync is required for this disk."]
    #[serde(rename = "resyncRequired", default, skip_serializing_if = "Option::is_none")]
    pub resync_required: Option<bool>,
    #[doc = "The percentage of the monitoring job. The type of the monitoring job is defined by MonitoringJobType property."]
    #[serde(rename = "monitoringPercentageCompletion", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_percentage_completion: Option<i32>,
    #[doc = "The type of the monitoring job. The progress is contained in MonitoringPercentageCompletion property."]
    #[serde(rename = "monitoringJobType", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_job_type: Option<String>,
    #[doc = "The data pending for replication in MB at staging account."]
    #[serde(
        rename = "dataPendingInStagingStorageAccountInMB",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_pending_in_staging_storage_account_in_mb: Option<f64>,
    #[doc = "The data pending at source virtual machine in MB."]
    #[serde(rename = "dataPendingAtSourceAgentInMB", default, skip_serializing_if = "Option::is_none")]
    pub data_pending_at_source_agent_in_mb: Option<f64>,
    #[doc = "The disk state."]
    #[serde(rename = "diskState", default, skip_serializing_if = "Option::is_none")]
    pub disk_state: Option<String>,
    #[doc = "The disk level operations list."]
    #[serde(rename = "allowedDiskLevelOperation", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_disk_level_operation: Vec<String>,
    #[doc = "A value indicating whether vm has encrypted os disk or not."]
    #[serde(rename = "isDiskEncrypted", default, skip_serializing_if = "Option::is_none")]
    pub is_disk_encrypted: Option<bool>,
    #[doc = "The secret URL / identifier (BEK)."]
    #[serde(rename = "secretIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub secret_identifier: Option<String>,
    #[doc = "The KeyVault resource id for secret (BEK)."]
    #[serde(rename = "dekKeyVaultArmId", default, skip_serializing_if = "Option::is_none")]
    pub dek_key_vault_arm_id: Option<String>,
    #[doc = "A value indicating whether disk key got encrypted or not."]
    #[serde(rename = "isDiskKeyEncrypted", default, skip_serializing_if = "Option::is_none")]
    pub is_disk_key_encrypted: Option<bool>,
    #[doc = "The key URL / identifier (KEK)."]
    #[serde(rename = "keyIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub key_identifier: Option<String>,
    #[doc = "The KeyVault resource id for key (KEK)."]
    #[serde(rename = "kekKeyVaultArmId", default, skip_serializing_if = "Option::is_none")]
    pub kek_key_vault_arm_id: Option<String>,
    #[doc = "The failover name for the managed disk."]
    #[serde(rename = "failoverDiskName", default, skip_serializing_if = "Option::is_none")]
    pub failover_disk_name: Option<String>,
    #[doc = "The test failover name for the managed disk."]
    #[serde(rename = "tfoDiskName", default, skip_serializing_if = "Option::is_none")]
    pub tfo_disk_name: Option<String>,
}
impl A2aProtectedDiskDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A2A protected managed disk details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct A2aProtectedManagedDiskDetails {
    #[doc = "The managed disk Arm id."]
    #[serde(rename = "diskId", default, skip_serializing_if = "Option::is_none")]
    pub disk_id: Option<String>,
    #[doc = "The recovery disk resource group Arm Id."]
    #[serde(rename = "recoveryResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_resource_group_id: Option<String>,
    #[doc = "Recovery target disk Arm Id."]
    #[serde(rename = "recoveryTargetDiskId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_target_disk_id: Option<String>,
    #[doc = "Recovery replica disk Arm Id."]
    #[serde(rename = "recoveryReplicaDiskId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_replica_disk_id: Option<String>,
    #[doc = "Recovery original target disk Arm Id."]
    #[serde(rename = "recoveryOrignalTargetDiskId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_orignal_target_disk_id: Option<String>,
    #[doc = "The replica disk type. Its an optional value and will be same as source disk type if not user provided."]
    #[serde(rename = "recoveryReplicaDiskAccountType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_replica_disk_account_type: Option<String>,
    #[doc = "The target disk type after failover. Its an optional value and will be same as source disk type if not user provided."]
    #[serde(rename = "recoveryTargetDiskAccountType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_target_disk_account_type: Option<String>,
    #[doc = "The recovery disk encryption set Id."]
    #[serde(rename = "recoveryDiskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_disk_encryption_set_id: Option<String>,
    #[doc = "The primary disk encryption set Id."]
    #[serde(rename = "primaryDiskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub primary_disk_encryption_set_id: Option<String>,
    #[doc = "The disk name."]
    #[serde(rename = "diskName", default, skip_serializing_if = "Option::is_none")]
    pub disk_name: Option<String>,
    #[doc = "The disk capacity in bytes."]
    #[serde(rename = "diskCapacityInBytes", default, skip_serializing_if = "Option::is_none")]
    pub disk_capacity_in_bytes: Option<i64>,
    #[doc = "The primary staging storage account."]
    #[serde(rename = "primaryStagingAzureStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub primary_staging_azure_storage_account_id: Option<String>,
    #[doc = "The type of disk."]
    #[serde(rename = "diskType", default, skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<String>,
    #[doc = "A value indicating whether resync is required for this disk."]
    #[serde(rename = "resyncRequired", default, skip_serializing_if = "Option::is_none")]
    pub resync_required: Option<bool>,
    #[doc = "The percentage of the monitoring job. The type of the monitoring job is defined by MonitoringJobType property."]
    #[serde(rename = "monitoringPercentageCompletion", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_percentage_completion: Option<i32>,
    #[doc = "The type of the monitoring job. The progress is contained in MonitoringPercentageCompletion property."]
    #[serde(rename = "monitoringJobType", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_job_type: Option<String>,
    #[doc = "The data pending for replication in MB at staging account."]
    #[serde(
        rename = "dataPendingInStagingStorageAccountInMB",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_pending_in_staging_storage_account_in_mb: Option<f64>,
    #[doc = "The data pending at source virtual machine in MB."]
    #[serde(rename = "dataPendingAtSourceAgentInMB", default, skip_serializing_if = "Option::is_none")]
    pub data_pending_at_source_agent_in_mb: Option<f64>,
    #[doc = "The disk state."]
    #[serde(rename = "diskState", default, skip_serializing_if = "Option::is_none")]
    pub disk_state: Option<String>,
    #[doc = "The disk level operations list."]
    #[serde(rename = "allowedDiskLevelOperation", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_disk_level_operation: Vec<String>,
    #[doc = "A value indicating whether vm has encrypted os disk or not."]
    #[serde(rename = "isDiskEncrypted", default, skip_serializing_if = "Option::is_none")]
    pub is_disk_encrypted: Option<bool>,
    #[doc = "The secret URL / identifier (BEK)."]
    #[serde(rename = "secretIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub secret_identifier: Option<String>,
    #[doc = "The KeyVault resource id for secret (BEK)."]
    #[serde(rename = "dekKeyVaultArmId", default, skip_serializing_if = "Option::is_none")]
    pub dek_key_vault_arm_id: Option<String>,
    #[doc = "A value indicating whether disk key got encrypted or not."]
    #[serde(rename = "isDiskKeyEncrypted", default, skip_serializing_if = "Option::is_none")]
    pub is_disk_key_encrypted: Option<bool>,
    #[doc = "The key URL / identifier (KEK)."]
    #[serde(rename = "keyIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub key_identifier: Option<String>,
    #[doc = "The KeyVault resource id for key (KEK)."]
    #[serde(rename = "kekKeyVaultArmId", default, skip_serializing_if = "Option::is_none")]
    pub kek_key_vault_arm_id: Option<String>,
    #[doc = "The failover name for the managed disk."]
    #[serde(rename = "failoverDiskName", default, skip_serializing_if = "Option::is_none")]
    pub failover_disk_name: Option<String>,
    #[doc = "The test failover name for the managed disk."]
    #[serde(rename = "tfoDiskName", default, skip_serializing_if = "Option::is_none")]
    pub tfo_disk_name: Option<String>,
}
impl A2aProtectedManagedDiskDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A2A provider specific settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aProtectionContainerMappingDetails {
    #[serde(flatten)]
    pub protection_container_mapping_provider_specific_details: ProtectionContainerMappingProviderSpecificDetails,
    #[doc = "A value indicating whether the auto update is enabled."]
    #[serde(rename = "agentAutoUpdateStatus", default, skip_serializing_if = "Option::is_none")]
    pub agent_auto_update_status: Option<a2a_protection_container_mapping_details::AgentAutoUpdateStatus>,
    #[doc = "The automation account arm id."]
    #[serde(rename = "automationAccountArmId", default, skip_serializing_if = "Option::is_none")]
    pub automation_account_arm_id: Option<String>,
    #[doc = "A value indicating the type authentication to use for automation Account."]
    #[serde(rename = "automationAccountAuthenticationType", default, skip_serializing_if = "Option::is_none")]
    pub automation_account_authentication_type: Option<a2a_protection_container_mapping_details::AutomationAccountAuthenticationType>,
    #[doc = "The schedule arm name."]
    #[serde(rename = "scheduleName", default, skip_serializing_if = "Option::is_none")]
    pub schedule_name: Option<String>,
    #[doc = "The job schedule arm name."]
    #[serde(rename = "jobScheduleName", default, skip_serializing_if = "Option::is_none")]
    pub job_schedule_name: Option<String>,
}
impl A2aProtectionContainerMappingDetails {
    pub fn new(protection_container_mapping_provider_specific_details: ProtectionContainerMappingProviderSpecificDetails) -> Self {
        Self {
            protection_container_mapping_provider_specific_details,
            agent_auto_update_status: None,
            automation_account_arm_id: None,
            automation_account_authentication_type: None,
            schedule_name: None,
            job_schedule_name: None,
        }
    }
}
pub mod a2a_protection_container_mapping_details {
    use super::*;
    #[doc = "A value indicating whether the auto update is enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AgentAutoUpdateStatus")]
    pub enum AgentAutoUpdateStatus {
        Disabled,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AgentAutoUpdateStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AgentAutoUpdateStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AgentAutoUpdateStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("AgentAutoUpdateStatus", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("AgentAutoUpdateStatus", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "A value indicating the type authentication to use for automation Account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AutomationAccountAuthenticationType")]
    pub enum AutomationAccountAuthenticationType {
        RunAsAccount,
        SystemAssignedIdentity,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AutomationAccountAuthenticationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AutomationAccountAuthenticationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AutomationAccountAuthenticationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::RunAsAccount => serializer.serialize_unit_variant("AutomationAccountAuthenticationType", 0u32, "RunAsAccount"),
                Self::SystemAssignedIdentity => {
                    serializer.serialize_unit_variant("AutomationAccountAuthenticationType", 1u32, "SystemAssignedIdentity")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for AutomationAccountAuthenticationType {
        fn default() -> Self {
            Self::RunAsAccount
        }
    }
}
#[doc = "Azure VM unmanaged disk input details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aProtectionIntentDiskInputDetails {
    #[doc = "The disk Uri."]
    #[serde(rename = "diskUri")]
    pub disk_uri: String,
    #[doc = "Storage account custom input."]
    #[serde(
        rename = "recoveryAzureStorageAccountCustomInput",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recovery_azure_storage_account_custom_input: Option<StorageAccountCustomDetails>,
    #[doc = "Storage account custom input."]
    #[serde(
        rename = "primaryStagingStorageAccountCustomInput",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_staging_storage_account_custom_input: Option<StorageAccountCustomDetails>,
}
impl A2aProtectionIntentDiskInputDetails {
    pub fn new(disk_uri: String) -> Self {
        Self {
            disk_uri,
            recovery_azure_storage_account_custom_input: None,
            primary_staging_storage_account_custom_input: None,
        }
    }
}
#[doc = "Azure VM managed disk input details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aProtectionIntentManagedDiskInputDetails {
    #[doc = "The disk Id."]
    #[serde(rename = "diskId")]
    pub disk_id: String,
    #[doc = "Storage account custom input."]
    #[serde(
        rename = "primaryStagingStorageAccountCustomInput",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_staging_storage_account_custom_input: Option<StorageAccountCustomDetails>,
    #[doc = "Recovery Resource Group custom input."]
    #[serde(rename = "recoveryResourceGroupCustomInput", default, skip_serializing_if = "Option::is_none")]
    pub recovery_resource_group_custom_input: Option<RecoveryResourceGroupCustomDetails>,
    #[doc = "The replica disk type. Its an optional value and will be same as source disk type if not user provided."]
    #[serde(rename = "recoveryReplicaDiskAccountType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_replica_disk_account_type: Option<String>,
    #[doc = "The target disk type after failover. Its an optional value and will be same as source disk type if not user provided."]
    #[serde(rename = "recoveryTargetDiskAccountType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_target_disk_account_type: Option<String>,
    #[doc = "The recovery disk encryption set Id."]
    #[serde(rename = "recoveryDiskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_disk_encryption_set_id: Option<String>,
    #[doc = "Recovery disk encryption info (BEK and KEK)."]
    #[serde(rename = "diskEncryptionInfo", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_info: Option<DiskEncryptionInfo>,
}
impl A2aProtectionIntentManagedDiskInputDetails {
    pub fn new(disk_id: String) -> Self {
        Self {
            disk_id,
            primary_staging_storage_account_custom_input: None,
            recovery_resource_group_custom_input: None,
            recovery_replica_disk_account_type: None,
            recovery_target_disk_account_type: None,
            recovery_disk_encryption_set_id: None,
            disk_encryption_info: None,
        }
    }
}
#[doc = "A2A provider specific recovery point details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aRecoveryPointDetails {
    #[serde(flatten)]
    pub provider_specific_recovery_point_details: ProviderSpecificRecoveryPointDetails,
    #[doc = "A value indicating whether the recovery point is multi VM consistent."]
    #[serde(rename = "recoveryPointSyncType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_sync_type: Option<a2a_recovery_point_details::RecoveryPointSyncType>,
    #[doc = "List of disk ids representing a recovery point."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<String>,
}
impl A2aRecoveryPointDetails {
    pub fn new(provider_specific_recovery_point_details: ProviderSpecificRecoveryPointDetails) -> Self {
        Self {
            provider_specific_recovery_point_details,
            recovery_point_sync_type: None,
            disks: Vec::new(),
        }
    }
}
pub mod a2a_recovery_point_details {
    use super::*;
    #[doc = "A value indicating whether the recovery point is multi VM consistent."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryPointSyncType")]
    pub enum RecoveryPointSyncType {
        MultiVmSyncRecoveryPoint,
        PerVmRecoveryPoint,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryPointSyncType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryPointSyncType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryPointSyncType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MultiVmSyncRecoveryPoint => {
                    serializer.serialize_unit_variant("RecoveryPointSyncType", 0u32, "MultiVmSyncRecoveryPoint")
                }
                Self::PerVmRecoveryPoint => serializer.serialize_unit_variant("RecoveryPointSyncType", 1u32, "PerVmRecoveryPoint"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A2A remove disk(s) input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aRemoveDisksInput {
    #[serde(flatten)]
    pub remove_disks_provider_specific_input: RemoveDisksProviderSpecificInput,
    #[doc = "The list of vm disk vhd URIs."]
    #[serde(rename = "vmDisksUris", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_disks_uris: Vec<String>,
    #[doc = "The list of vm managed disk Ids."]
    #[serde(rename = "vmManagedDisksIds", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_managed_disks_ids: Vec<String>,
}
impl A2aRemoveDisksInput {
    pub fn new(remove_disks_provider_specific_input: RemoveDisksProviderSpecificInput) -> Self {
        Self {
            remove_disks_provider_specific_input,
            vm_disks_uris: Vec::new(),
            vm_managed_disks_ids: Vec::new(),
        }
    }
}
#[doc = "A2A provider specific settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aReplicationDetails {
    #[serde(flatten)]
    pub replication_provider_specific_settings: ReplicationProviderSpecificSettings,
    #[doc = "The fabric specific object Id of the virtual machine."]
    #[serde(rename = "fabricObjectId", default, skip_serializing_if = "Option::is_none")]
    pub fabric_object_id: Option<String>,
    #[doc = "The initial primary availability zone."]
    #[serde(rename = "initialPrimaryZone", default, skip_serializing_if = "Option::is_none")]
    pub initial_primary_zone: Option<String>,
    #[doc = "The initial primary fabric location."]
    #[serde(rename = "initialPrimaryFabricLocation", default, skip_serializing_if = "Option::is_none")]
    pub initial_primary_fabric_location: Option<String>,
    #[doc = "The initial recovery availability zone."]
    #[serde(rename = "initialRecoveryZone", default, skip_serializing_if = "Option::is_none")]
    pub initial_recovery_zone: Option<String>,
    #[doc = "Extended location of the resource."]
    #[serde(rename = "initialPrimaryExtendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub initial_primary_extended_location: Option<ExtendedLocation>,
    #[doc = "Extended location of the resource."]
    #[serde(rename = "initialRecoveryExtendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub initial_recovery_extended_location: Option<ExtendedLocation>,
    #[doc = "The initial recovery fabric location."]
    #[serde(rename = "initialRecoveryFabricLocation", default, skip_serializing_if = "Option::is_none")]
    pub initial_recovery_fabric_location: Option<String>,
    #[doc = "The multi vm group Id."]
    #[serde(rename = "multiVmGroupId", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_group_id: Option<String>,
    #[doc = "The multi vm group name."]
    #[serde(rename = "multiVmGroupName", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_group_name: Option<String>,
    #[doc = "Whether Multi VM group is auto created or specified by user."]
    #[serde(rename = "multiVmGroupCreateOption", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_group_create_option: Option<a2a_replication_details::MultiVmGroupCreateOption>,
    #[doc = "The management Id."]
    #[serde(rename = "managementId", default, skip_serializing_if = "Option::is_none")]
    pub management_id: Option<String>,
    #[doc = "The list of protected disks."]
    #[serde(rename = "protectedDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub protected_disks: Vec<A2aProtectedDiskDetails>,
    #[doc = "The list of unprotected disks."]
    #[serde(rename = "unprotectedDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub unprotected_disks: Vec<A2aUnprotectedDiskDetails>,
    #[doc = "The list of protected managed disks."]
    #[serde(rename = "protectedManagedDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub protected_managed_disks: Vec<A2aProtectedManagedDiskDetails>,
    #[doc = "The recovery boot diagnostic storage account Arm Id."]
    #[serde(rename = "recoveryBootDiagStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_boot_diag_storage_account_id: Option<String>,
    #[doc = "Primary fabric location."]
    #[serde(rename = "primaryFabricLocation", default, skip_serializing_if = "Option::is_none")]
    pub primary_fabric_location: Option<String>,
    #[doc = "The recovery fabric location."]
    #[serde(rename = "recoveryFabricLocation", default, skip_serializing_if = "Option::is_none")]
    pub recovery_fabric_location: Option<String>,
    #[doc = "The type of operating system."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "The size of recovery virtual machine."]
    #[serde(rename = "recoveryAzureVMSize", default, skip_serializing_if = "Option::is_none")]
    pub recovery_azure_vm_size: Option<String>,
    #[doc = "The name of recovery virtual machine."]
    #[serde(rename = "recoveryAzureVMName", default, skip_serializing_if = "Option::is_none")]
    pub recovery_azure_vm_name: Option<String>,
    #[doc = "The recovery resource group."]
    #[serde(rename = "recoveryAzureResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_azure_resource_group_id: Option<String>,
    #[doc = "The recovery cloud service."]
    #[serde(rename = "recoveryCloudService", default, skip_serializing_if = "Option::is_none")]
    pub recovery_cloud_service: Option<String>,
    #[doc = "The recovery availability set."]
    #[serde(rename = "recoveryAvailabilitySet", default, skip_serializing_if = "Option::is_none")]
    pub recovery_availability_set: Option<String>,
    #[doc = "The recovery virtual network."]
    #[serde(rename = "selectedRecoveryAzureNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub selected_recovery_azure_network_id: Option<String>,
    #[doc = "The test failover virtual network."]
    #[serde(rename = "selectedTfoAzureNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub selected_tfo_azure_network_id: Option<String>,
    #[doc = "The virtual machine nic details."]
    #[serde(rename = "vmNics", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_nics: Vec<VmNicDetails>,
    #[doc = "Azure to Azure VM synced configuration details."]
    #[serde(rename = "vmSyncedConfigDetails", default, skip_serializing_if = "Option::is_none")]
    pub vm_synced_config_details: Option<AzureToAzureVmSyncedConfigDetails>,
    #[doc = "The percentage of the monitoring job. The type of the monitoring job is defined by MonitoringJobType property."]
    #[serde(rename = "monitoringPercentageCompletion", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_percentage_completion: Option<i32>,
    #[doc = "The type of the monitoring job. The progress is contained in MonitoringPercentageCompletion property."]
    #[serde(rename = "monitoringJobType", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_job_type: Option<String>,
    #[doc = "The last heartbeat received from the source server."]
    #[serde(rename = "lastHeartbeat", with = "azure_core::date::rfc3339::option")]
    pub last_heartbeat: Option<time::OffsetDateTime>,
    #[doc = "The agent version."]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "Agent expiry date."]
    #[serde(rename = "agentExpiryDate", with = "azure_core::date::rfc3339::option")]
    pub agent_expiry_date: Option<time::OffsetDateTime>,
    #[doc = "A value indicating whether replication agent update is required."]
    #[serde(rename = "isReplicationAgentUpdateRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_replication_agent_update_required: Option<bool>,
    #[doc = "Agent certificate expiry date."]
    #[serde(rename = "agentCertificateExpiryDate", with = "azure_core::date::rfc3339::option")]
    pub agent_certificate_expiry_date: Option<time::OffsetDateTime>,
    #[doc = "A value indicating whether agent certificate update is required."]
    #[serde(
        rename = "isReplicationAgentCertificateUpdateRequired",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_replication_agent_certificate_update_required: Option<bool>,
    #[doc = "The recovery fabric object Id."]
    #[serde(rename = "recoveryFabricObjectId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_fabric_object_id: Option<String>,
    #[doc = "The protection state for the vm."]
    #[serde(rename = "vmProtectionState", default, skip_serializing_if = "Option::is_none")]
    pub vm_protection_state: Option<String>,
    #[doc = "The protection state description for the vm."]
    #[serde(rename = "vmProtectionStateDescription", default, skip_serializing_if = "Option::is_none")]
    pub vm_protection_state_description: Option<String>,
    #[doc = "An id associated with the PE that survives actions like switch protection which change the backing PE/CPE objects internally.The lifecycle id gets carried forward to have a link/continuity in being able to have an Id that denotes the \"same\" protected item even though other internal Ids/ARM Id might be changing."]
    #[serde(rename = "lifecycleId", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_id: Option<String>,
    #[doc = "The test failover fabric object Id."]
    #[serde(rename = "testFailoverRecoveryFabricObjectId", default, skip_serializing_if = "Option::is_none")]
    pub test_failover_recovery_fabric_object_id: Option<String>,
    #[doc = "The last RPO value in seconds."]
    #[serde(rename = "rpoInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub rpo_in_seconds: Option<i64>,
    #[doc = "The time (in UTC) when the last RPO value was calculated by Protection Service."]
    #[serde(rename = "lastRpoCalculatedTime", with = "azure_core::date::rfc3339::option")]
    pub last_rpo_calculated_time: Option<time::OffsetDateTime>,
    #[doc = "The primary availability zone."]
    #[serde(rename = "primaryAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub primary_availability_zone: Option<String>,
    #[doc = "The recovery availability zone."]
    #[serde(rename = "recoveryAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub recovery_availability_zone: Option<String>,
    #[doc = "Extended location of the resource."]
    #[serde(rename = "primaryExtendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub primary_extended_location: Option<ExtendedLocation>,
    #[doc = "Extended location of the resource."]
    #[serde(rename = "recoveryExtendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub recovery_extended_location: Option<ExtendedLocation>,
    #[doc = "The encryption type of the VM."]
    #[serde(rename = "vmEncryptionType", default, skip_serializing_if = "Option::is_none")]
    pub vm_encryption_type: Option<a2a_replication_details::VmEncryptionType>,
    #[doc = "The test failover vm name."]
    #[serde(rename = "tfoAzureVMName", default, skip_serializing_if = "Option::is_none")]
    pub tfo_azure_vm_name: Option<String>,
    #[doc = "The recovery azure generation."]
    #[serde(rename = "recoveryAzureGeneration", default, skip_serializing_if = "Option::is_none")]
    pub recovery_azure_generation: Option<String>,
    #[doc = "The recovery proximity placement group Id."]
    #[serde(rename = "recoveryProximityPlacementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_proximity_placement_group_id: Option<String>,
    #[doc = "A value indicating whether the auto protection is enabled."]
    #[serde(rename = "autoProtectionOfDataDisk", default, skip_serializing_if = "Option::is_none")]
    pub auto_protection_of_data_disk: Option<a2a_replication_details::AutoProtectionOfDataDisk>,
    #[doc = "The recovery virtual machine scale set id."]
    #[serde(rename = "recoveryVirtualMachineScaleSetId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_virtual_machine_scale_set_id: Option<String>,
    #[doc = "The recovery capacity reservation group Id."]
    #[serde(rename = "recoveryCapacityReservationGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_capacity_reservation_group_id: Option<String>,
}
impl A2aReplicationDetails {
    pub fn new(replication_provider_specific_settings: ReplicationProviderSpecificSettings) -> Self {
        Self {
            replication_provider_specific_settings,
            fabric_object_id: None,
            initial_primary_zone: None,
            initial_primary_fabric_location: None,
            initial_recovery_zone: None,
            initial_primary_extended_location: None,
            initial_recovery_extended_location: None,
            initial_recovery_fabric_location: None,
            multi_vm_group_id: None,
            multi_vm_group_name: None,
            multi_vm_group_create_option: None,
            management_id: None,
            protected_disks: Vec::new(),
            unprotected_disks: Vec::new(),
            protected_managed_disks: Vec::new(),
            recovery_boot_diag_storage_account_id: None,
            primary_fabric_location: None,
            recovery_fabric_location: None,
            os_type: None,
            recovery_azure_vm_size: None,
            recovery_azure_vm_name: None,
            recovery_azure_resource_group_id: None,
            recovery_cloud_service: None,
            recovery_availability_set: None,
            selected_recovery_azure_network_id: None,
            selected_tfo_azure_network_id: None,
            vm_nics: Vec::new(),
            vm_synced_config_details: None,
            monitoring_percentage_completion: None,
            monitoring_job_type: None,
            last_heartbeat: None,
            agent_version: None,
            agent_expiry_date: None,
            is_replication_agent_update_required: None,
            agent_certificate_expiry_date: None,
            is_replication_agent_certificate_update_required: None,
            recovery_fabric_object_id: None,
            vm_protection_state: None,
            vm_protection_state_description: None,
            lifecycle_id: None,
            test_failover_recovery_fabric_object_id: None,
            rpo_in_seconds: None,
            last_rpo_calculated_time: None,
            primary_availability_zone: None,
            recovery_availability_zone: None,
            primary_extended_location: None,
            recovery_extended_location: None,
            vm_encryption_type: None,
            tfo_azure_vm_name: None,
            recovery_azure_generation: None,
            recovery_proximity_placement_group_id: None,
            auto_protection_of_data_disk: None,
            recovery_virtual_machine_scale_set_id: None,
            recovery_capacity_reservation_group_id: None,
        }
    }
}
pub mod a2a_replication_details {
    use super::*;
    #[doc = "Whether Multi VM group is auto created or specified by user."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MultiVmGroupCreateOption")]
    pub enum MultiVmGroupCreateOption {
        AutoCreated,
        UserSpecified,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MultiVmGroupCreateOption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MultiVmGroupCreateOption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MultiVmGroupCreateOption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AutoCreated => serializer.serialize_unit_variant("MultiVmGroupCreateOption", 0u32, "AutoCreated"),
                Self::UserSpecified => serializer.serialize_unit_variant("MultiVmGroupCreateOption", 1u32, "UserSpecified"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The encryption type of the VM."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "VmEncryptionType")]
    pub enum VmEncryptionType {
        NotEncrypted,
        OnePassEncrypted,
        TwoPassEncrypted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for VmEncryptionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for VmEncryptionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for VmEncryptionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotEncrypted => serializer.serialize_unit_variant("VmEncryptionType", 0u32, "NotEncrypted"),
                Self::OnePassEncrypted => serializer.serialize_unit_variant("VmEncryptionType", 1u32, "OnePassEncrypted"),
                Self::TwoPassEncrypted => serializer.serialize_unit_variant("VmEncryptionType", 2u32, "TwoPassEncrypted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "A value indicating whether the auto protection is enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AutoProtectionOfDataDisk")]
    pub enum AutoProtectionOfDataDisk {
        Disabled,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AutoProtectionOfDataDisk {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AutoProtectionOfDataDisk {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AutoProtectionOfDataDisk {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("AutoProtectionOfDataDisk", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("AutoProtectionOfDataDisk", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A2A provider specific settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aReplicationIntentDetails {
    #[serde(flatten)]
    pub replication_protection_intent_provider_specific_settings: ReplicationProtectionIntentProviderSpecificSettings,
    #[doc = "The fabric specific object Id of the virtual machine."]
    #[serde(rename = "fabricObjectId", default, skip_serializing_if = "Option::is_none")]
    pub fabric_object_id: Option<String>,
    #[doc = "The primary location for the virtual machine."]
    #[serde(rename = "primaryLocation", default, skip_serializing_if = "Option::is_none")]
    pub primary_location: Option<String>,
    #[doc = "The recovery location for the virtual machine."]
    #[serde(rename = "recoveryLocation", default, skip_serializing_if = "Option::is_none")]
    pub recovery_location: Option<String>,
    #[doc = "The recovery subscription Id of the virtual machine."]
    #[serde(rename = "recoverySubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_subscription_id: Option<String>,
    #[doc = "The list of vm disk details."]
    #[serde(rename = "vmDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_disks: Vec<A2aProtectionIntentDiskInputDetails>,
    #[doc = "The list of vm managed disk details."]
    #[serde(rename = "vmManagedDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_managed_disks: Vec<A2aProtectionIntentManagedDiskInputDetails>,
    #[doc = "The recovery resource group id."]
    #[serde(rename = "recoveryResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_resource_group_id: Option<String>,
    #[doc = "Protection Profile custom input."]
    #[serde(rename = "protectionProfile", default, skip_serializing_if = "Option::is_none")]
    pub protection_profile: Option<ProtectionProfileCustomDetails>,
    #[doc = "Storage account custom input."]
    #[serde(rename = "primaryStagingStorageAccount", default, skip_serializing_if = "Option::is_none")]
    pub primary_staging_storage_account: Option<StorageAccountCustomDetails>,
    #[doc = "Recovery Availability Set custom input."]
    #[serde(rename = "recoveryAvailabilitySet", default, skip_serializing_if = "Option::is_none")]
    pub recovery_availability_set: Option<RecoveryAvailabilitySetCustomDetails>,
    #[doc = "Recovery Virtual network custom input."]
    #[serde(rename = "recoveryVirtualNetwork", default, skip_serializing_if = "Option::is_none")]
    pub recovery_virtual_network: Option<RecoveryVirtualNetworkCustomDetails>,
    #[doc = "Recovery Proximity placement group custom input."]
    #[serde(rename = "recoveryProximityPlacementGroup", default, skip_serializing_if = "Option::is_none")]
    pub recovery_proximity_placement_group: Option<RecoveryProximityPlacementGroupCustomDetails>,
    #[doc = "A value indicating whether the auto protection is enabled."]
    #[serde(rename = "autoProtectionOfDataDisk", default, skip_serializing_if = "Option::is_none")]
    pub auto_protection_of_data_disk: Option<a2a_replication_intent_details::AutoProtectionOfDataDisk>,
    #[doc = "The multi vm group name."]
    #[serde(rename = "multiVmGroupName", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_group_name: Option<String>,
    #[doc = "The multi vm group id."]
    #[serde(rename = "multiVmGroupId", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_group_id: Option<String>,
    #[doc = "Storage account custom input."]
    #[serde(rename = "recoveryBootDiagStorageAccount", default, skip_serializing_if = "Option::is_none")]
    pub recovery_boot_diag_storage_account: Option<StorageAccountCustomDetails>,
    #[doc = "Recovery disk encryption info (BEK and KEK)."]
    #[serde(rename = "diskEncryptionInfo", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_info: Option<DiskEncryptionInfo>,
    #[doc = "The recovery availability zone."]
    #[serde(rename = "recoveryAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub recovery_availability_zone: Option<String>,
    #[doc = "The recovery availability type of the virtual machine."]
    #[serde(rename = "recoveryAvailabilityType")]
    pub recovery_availability_type: String,
    #[doc = "A value indicating whether the auto update is enabled."]
    #[serde(rename = "agentAutoUpdateStatus", default, skip_serializing_if = "Option::is_none")]
    pub agent_auto_update_status: Option<a2a_replication_intent_details::AgentAutoUpdateStatus>,
    #[doc = "The automation account arm id."]
    #[serde(rename = "automationAccountArmId", default, skip_serializing_if = "Option::is_none")]
    pub automation_account_arm_id: Option<String>,
    #[doc = "A value indicating the type authentication to use for automation Account."]
    #[serde(rename = "automationAccountAuthenticationType", default, skip_serializing_if = "Option::is_none")]
    pub automation_account_authentication_type: Option<a2a_replication_intent_details::AutomationAccountAuthenticationType>,
}
impl A2aReplicationIntentDetails {
    pub fn new(
        replication_protection_intent_provider_specific_settings: ReplicationProtectionIntentProviderSpecificSettings,
        recovery_availability_type: String,
    ) -> Self {
        Self {
            replication_protection_intent_provider_specific_settings,
            fabric_object_id: None,
            primary_location: None,
            recovery_location: None,
            recovery_subscription_id: None,
            vm_disks: Vec::new(),
            vm_managed_disks: Vec::new(),
            recovery_resource_group_id: None,
            protection_profile: None,
            primary_staging_storage_account: None,
            recovery_availability_set: None,
            recovery_virtual_network: None,
            recovery_proximity_placement_group: None,
            auto_protection_of_data_disk: None,
            multi_vm_group_name: None,
            multi_vm_group_id: None,
            recovery_boot_diag_storage_account: None,
            disk_encryption_info: None,
            recovery_availability_zone: None,
            recovery_availability_type,
            agent_auto_update_status: None,
            automation_account_arm_id: None,
            automation_account_authentication_type: None,
        }
    }
}
pub mod a2a_replication_intent_details {
    use super::*;
    #[doc = "A value indicating whether the auto protection is enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AutoProtectionOfDataDisk")]
    pub enum AutoProtectionOfDataDisk {
        Disabled,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AutoProtectionOfDataDisk {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AutoProtectionOfDataDisk {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AutoProtectionOfDataDisk {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("AutoProtectionOfDataDisk", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("AutoProtectionOfDataDisk", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "A value indicating whether the auto update is enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AgentAutoUpdateStatus")]
    pub enum AgentAutoUpdateStatus {
        Disabled,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AgentAutoUpdateStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AgentAutoUpdateStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AgentAutoUpdateStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("AgentAutoUpdateStatus", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("AgentAutoUpdateStatus", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "A value indicating the type authentication to use for automation Account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AutomationAccountAuthenticationType")]
    pub enum AutomationAccountAuthenticationType {
        RunAsAccount,
        SystemAssignedIdentity,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AutomationAccountAuthenticationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AutomationAccountAuthenticationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AutomationAccountAuthenticationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::RunAsAccount => serializer.serialize_unit_variant("AutomationAccountAuthenticationType", 0u32, "RunAsAccount"),
                Self::SystemAssignedIdentity => {
                    serializer.serialize_unit_variant("AutomationAccountAuthenticationType", 1u32, "SystemAssignedIdentity")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for AutomationAccountAuthenticationType {
        fn default() -> Self {
            Self::RunAsAccount
        }
    }
}
#[doc = "Azure specific reprotect input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aReprotectInput {
    #[serde(flatten)]
    pub reverse_replication_provider_specific_input: ReverseReplicationProviderSpecificInput,
    #[doc = "The recovery container Id."]
    #[serde(rename = "recoveryContainerId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_container_id: Option<String>,
    #[doc = "The list of vm disk details."]
    #[serde(rename = "vmDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_disks: Vec<A2aVmDiskInputDetails>,
    #[doc = "The recovery resource group Id. Valid for V2 scenarios."]
    #[serde(rename = "recoveryResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_resource_group_id: Option<String>,
    #[doc = "The recovery cloud service Id. Valid for V1 scenarios."]
    #[serde(rename = "recoveryCloudServiceId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_cloud_service_id: Option<String>,
    #[doc = "The recovery availability set."]
    #[serde(rename = "recoveryAvailabilitySetId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_availability_set_id: Option<String>,
    #[doc = "The Policy Id."]
    #[serde(rename = "policyId", default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
}
impl A2aReprotectInput {
    pub fn new(reverse_replication_provider_specific_input: ReverseReplicationProviderSpecificInput) -> Self {
        Self {
            reverse_replication_provider_specific_input,
            recovery_container_id: None,
            vm_disks: Vec::new(),
            recovery_resource_group_id: None,
            recovery_cloud_service_id: None,
            recovery_availability_set_id: None,
            policy_id: None,
        }
    }
}
#[doc = "A2A specific switch protection input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aSwitchProtectionInput {
    #[serde(flatten)]
    pub switch_protection_provider_specific_input: SwitchProtectionProviderSpecificInput,
    #[doc = "The recovery container Id."]
    #[serde(rename = "recoveryContainerId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_container_id: Option<String>,
    #[doc = "The list of vm disk details."]
    #[serde(rename = "vmDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_disks: Vec<A2aVmDiskInputDetails>,
    #[doc = "The list of vm managed disk details."]
    #[serde(rename = "vmManagedDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_managed_disks: Vec<A2aVmManagedDiskInputDetails>,
    #[doc = "The recovery resource group Id. Valid for V2 scenarios."]
    #[serde(rename = "recoveryResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_resource_group_id: Option<String>,
    #[doc = "The recovery cloud service Id. Valid for V1 scenarios."]
    #[serde(rename = "recoveryCloudServiceId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_cloud_service_id: Option<String>,
    #[doc = "The recovery availability set."]
    #[serde(rename = "recoveryAvailabilitySetId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_availability_set_id: Option<String>,
    #[doc = "The Policy Id."]
    #[serde(rename = "policyId", default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[doc = "The boot diagnostic storage account."]
    #[serde(rename = "recoveryBootDiagStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_boot_diag_storage_account_id: Option<String>,
    #[doc = "The recovery availability zone."]
    #[serde(rename = "recoveryAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub recovery_availability_zone: Option<String>,
    #[doc = "The recovery proximity placement group Id."]
    #[serde(rename = "recoveryProximityPlacementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_proximity_placement_group_id: Option<String>,
    #[doc = "The virtual machine scale set id."]
    #[serde(rename = "recoveryVirtualMachineScaleSetId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_virtual_machine_scale_set_id: Option<String>,
    #[doc = "The recovery capacity reservation group Id."]
    #[serde(rename = "recoveryCapacityReservationGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_capacity_reservation_group_id: Option<String>,
    #[doc = "Recovery disk encryption info (BEK and KEK)."]
    #[serde(rename = "diskEncryptionInfo", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_info: Option<DiskEncryptionInfo>,
}
impl A2aSwitchProtectionInput {
    pub fn new(switch_protection_provider_specific_input: SwitchProtectionProviderSpecificInput) -> Self {
        Self {
            switch_protection_provider_specific_input,
            recovery_container_id: None,
            vm_disks: Vec::new(),
            vm_managed_disks: Vec::new(),
            recovery_resource_group_id: None,
            recovery_cloud_service_id: None,
            recovery_availability_set_id: None,
            policy_id: None,
            recovery_boot_diag_storage_account_id: None,
            recovery_availability_zone: None,
            recovery_proximity_placement_group_id: None,
            recovery_virtual_machine_scale_set_id: None,
            recovery_capacity_reservation_group_id: None,
            disk_encryption_info: None,
        }
    }
}
#[doc = "A2A provider specific input for test failover."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aTestFailoverInput {
    #[serde(flatten)]
    pub test_failover_provider_specific_input: TestFailoverProviderSpecificInput,
    #[doc = "The recovery point id to be passed to test failover to a particular recovery point. In case of latest recovery point, null should be passed."]
    #[serde(rename = "recoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_id: Option<String>,
    #[doc = "A value indicating whether to use recovery cloud service for TFO or not."]
    #[serde(rename = "cloudServiceCreationOption", default, skip_serializing_if = "Option::is_none")]
    pub cloud_service_creation_option: Option<String>,
}
impl A2aTestFailoverInput {
    pub fn new(test_failover_provider_specific_input: TestFailoverProviderSpecificInput) -> Self {
        Self {
            test_failover_provider_specific_input,
            recovery_point_id: None,
            cloud_service_creation_option: None,
        }
    }
}
#[doc = "A2A provider specific input for unplanned failover."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aUnplannedFailoverInput {
    #[serde(flatten)]
    pub unplanned_failover_provider_specific_input: UnplannedFailoverProviderSpecificInput,
    #[doc = "The recovery point id to be passed to failover to a particular recovery point. In case of latest recovery point, null should be passed."]
    #[serde(rename = "recoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_id: Option<String>,
    #[doc = "A value indicating whether to use recovery cloud service for failover or not."]
    #[serde(rename = "cloudServiceCreationOption", default, skip_serializing_if = "Option::is_none")]
    pub cloud_service_creation_option: Option<String>,
}
impl A2aUnplannedFailoverInput {
    pub fn new(unplanned_failover_provider_specific_input: UnplannedFailoverProviderSpecificInput) -> Self {
        Self {
            unplanned_failover_provider_specific_input,
            recovery_point_id: None,
            cloud_service_creation_option: None,
        }
    }
}
#[doc = "A2A unprotected disk details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct A2aUnprotectedDiskDetails {
    #[doc = "The source lun Id for the data disk."]
    #[serde(rename = "diskLunId", default, skip_serializing_if = "Option::is_none")]
    pub disk_lun_id: Option<i32>,
    #[doc = "A value indicating whether the disk auto protection is enabled."]
    #[serde(rename = "diskAutoProtectionStatus", default, skip_serializing_if = "Option::is_none")]
    pub disk_auto_protection_status: Option<a2a_unprotected_disk_details::DiskAutoProtectionStatus>,
}
impl A2aUnprotectedDiskDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod a2a_unprotected_disk_details {
    use super::*;
    #[doc = "A value indicating whether the disk auto protection is enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DiskAutoProtectionStatus")]
    pub enum DiskAutoProtectionStatus {
        Disabled,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DiskAutoProtectionStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DiskAutoProtectionStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DiskAutoProtectionStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("DiskAutoProtectionStatus", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("DiskAutoProtectionStatus", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A2A update protection container mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aUpdateContainerMappingInput {
    #[serde(flatten)]
    pub replication_provider_specific_update_container_mapping_input: ReplicationProviderSpecificUpdateContainerMappingInput,
    #[doc = "A value indicating whether the auto update is enabled."]
    #[serde(rename = "agentAutoUpdateStatus", default, skip_serializing_if = "Option::is_none")]
    pub agent_auto_update_status: Option<a2a_update_container_mapping_input::AgentAutoUpdateStatus>,
    #[doc = "The automation account arm id."]
    #[serde(rename = "automationAccountArmId", default, skip_serializing_if = "Option::is_none")]
    pub automation_account_arm_id: Option<String>,
    #[doc = "A value indicating the type authentication to use for automation Account."]
    #[serde(rename = "automationAccountAuthenticationType", default, skip_serializing_if = "Option::is_none")]
    pub automation_account_authentication_type: Option<a2a_update_container_mapping_input::AutomationAccountAuthenticationType>,
}
impl A2aUpdateContainerMappingInput {
    pub fn new(
        replication_provider_specific_update_container_mapping_input: ReplicationProviderSpecificUpdateContainerMappingInput,
    ) -> Self {
        Self {
            replication_provider_specific_update_container_mapping_input,
            agent_auto_update_status: None,
            automation_account_arm_id: None,
            automation_account_authentication_type: None,
        }
    }
}
pub mod a2a_update_container_mapping_input {
    use super::*;
    #[doc = "A value indicating whether the auto update is enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AgentAutoUpdateStatus")]
    pub enum AgentAutoUpdateStatus {
        Disabled,
        Enabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AgentAutoUpdateStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AgentAutoUpdateStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AgentAutoUpdateStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Disabled => serializer.serialize_unit_variant("AgentAutoUpdateStatus", 0u32, "Disabled"),
                Self::Enabled => serializer.serialize_unit_variant("AgentAutoUpdateStatus", 1u32, "Enabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "A value indicating the type authentication to use for automation Account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AutomationAccountAuthenticationType")]
    pub enum AutomationAccountAuthenticationType {
        RunAsAccount,
        SystemAssignedIdentity,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AutomationAccountAuthenticationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AutomationAccountAuthenticationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AutomationAccountAuthenticationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::RunAsAccount => serializer.serialize_unit_variant("AutomationAccountAuthenticationType", 0u32, "RunAsAccount"),
                Self::SystemAssignedIdentity => {
                    serializer.serialize_unit_variant("AutomationAccountAuthenticationType", 1u32, "SystemAssignedIdentity")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for AutomationAccountAuthenticationType {
        fn default() -> Self {
            Self::RunAsAccount
        }
    }
}
#[doc = "InMage Azure V2 input to update replication protected item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aUpdateReplicationProtectedItemInput {
    #[serde(flatten)]
    pub update_replication_protected_item_provider_input: UpdateReplicationProtectedItemProviderInput,
    #[doc = "The target cloud service ARM Id (for V1)."]
    #[serde(rename = "recoveryCloudServiceId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_cloud_service_id: Option<String>,
    #[doc = "The target resource group ARM Id (for V2)."]
    #[serde(rename = "recoveryResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_resource_group_id: Option<String>,
    #[doc = "Managed disk update details."]
    #[serde(rename = "managedDiskUpdateDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub managed_disk_update_details: Vec<A2aVmManagedDiskUpdateDetails>,
    #[doc = "The boot diagnostic storage account."]
    #[serde(rename = "recoveryBootDiagStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_boot_diag_storage_account_id: Option<String>,
    #[doc = "Recovery disk encryption info (BEK and KEK)."]
    #[serde(rename = "diskEncryptionInfo", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_info: Option<DiskEncryptionInfo>,
    #[doc = "The user given name for Test Failover VM."]
    #[serde(rename = "tfoAzureVMName", default, skip_serializing_if = "Option::is_none")]
    pub tfo_azure_vm_name: Option<String>,
    #[doc = "The recovery proximity placement group Id."]
    #[serde(rename = "recoveryProximityPlacementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_proximity_placement_group_id: Option<String>,
    #[doc = "The recovery virtual machine scale set Id."]
    #[serde(rename = "recoveryVirtualMachineScaleSetId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_virtual_machine_scale_set_id: Option<String>,
    #[doc = "The recovery capacity reservation group Id."]
    #[serde(rename = "recoveryCapacityReservationGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_capacity_reservation_group_id: Option<String>,
}
impl A2aUpdateReplicationProtectedItemInput {
    pub fn new(update_replication_protected_item_provider_input: UpdateReplicationProtectedItemProviderInput) -> Self {
        Self {
            update_replication_protected_item_provider_input,
            recovery_cloud_service_id: None,
            recovery_resource_group_id: None,
            managed_disk_update_details: Vec::new(),
            recovery_boot_diag_storage_account_id: None,
            disk_encryption_info: None,
            tfo_azure_vm_name: None,
            recovery_proximity_placement_group_id: None,
            recovery_virtual_machine_scale_set_id: None,
            recovery_capacity_reservation_group_id: None,
        }
    }
}
#[doc = "A2A disk input details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aVmDiskInputDetails {
    #[doc = "The disk Uri."]
    #[serde(rename = "diskUri")]
    pub disk_uri: String,
    #[doc = "The recovery VHD storage account Id."]
    #[serde(rename = "recoveryAzureStorageAccountId")]
    pub recovery_azure_storage_account_id: String,
    #[doc = "The primary staging storage account Id."]
    #[serde(rename = "primaryStagingAzureStorageAccountId")]
    pub primary_staging_azure_storage_account_id: String,
}
impl A2aVmDiskInputDetails {
    pub fn new(disk_uri: String, recovery_azure_storage_account_id: String, primary_staging_azure_storage_account_id: String) -> Self {
        Self {
            disk_uri,
            recovery_azure_storage_account_id,
            primary_staging_azure_storage_account_id,
        }
    }
}
#[doc = "A2A managed disk input details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct A2aVmManagedDiskInputDetails {
    #[doc = "The disk Id."]
    #[serde(rename = "diskId")]
    pub disk_id: String,
    #[doc = "The primary staging storage account Arm Id."]
    #[serde(rename = "primaryStagingAzureStorageAccountId")]
    pub primary_staging_azure_storage_account_id: String,
    #[doc = "The target resource group Arm Id."]
    #[serde(rename = "recoveryResourceGroupId")]
    pub recovery_resource_group_id: String,
    #[doc = "The replica disk type. Its an optional value and will be same as source disk type if not user provided."]
    #[serde(rename = "recoveryReplicaDiskAccountType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_replica_disk_account_type: Option<String>,
    #[doc = "The target disk type after failover. Its an optional value and will be same as source disk type if not user provided."]
    #[serde(rename = "recoveryTargetDiskAccountType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_target_disk_account_type: Option<String>,
    #[doc = "The recovery disk encryption set Id."]
    #[serde(rename = "recoveryDiskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_disk_encryption_set_id: Option<String>,
    #[doc = "Recovery disk encryption info (BEK and KEK)."]
    #[serde(rename = "diskEncryptionInfo", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_info: Option<DiskEncryptionInfo>,
}
impl A2aVmManagedDiskInputDetails {
    pub fn new(disk_id: String, primary_staging_azure_storage_account_id: String, recovery_resource_group_id: String) -> Self {
        Self {
            disk_id,
            primary_staging_azure_storage_account_id,
            recovery_resource_group_id,
            recovery_replica_disk_account_type: None,
            recovery_target_disk_account_type: None,
            recovery_disk_encryption_set_id: None,
            disk_encryption_info: None,
        }
    }
}
#[doc = "A2A Vm managed disk update details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct A2aVmManagedDiskUpdateDetails {
    #[doc = "The disk Id."]
    #[serde(rename = "diskId", default, skip_serializing_if = "Option::is_none")]
    pub disk_id: Option<String>,
    #[doc = "The target disk type before failover."]
    #[serde(rename = "recoveryTargetDiskAccountType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_target_disk_account_type: Option<String>,
    #[doc = "The replica disk type before failover."]
    #[serde(rename = "recoveryReplicaDiskAccountType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_replica_disk_account_type: Option<String>,
    #[doc = "Recovery disk encryption info (BEK and KEK)."]
    #[serde(rename = "diskEncryptionInfo", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_info: Option<DiskEncryptionInfo>,
    #[doc = "The target disk name for unplanned failover operation."]
    #[serde(rename = "failoverDiskName", default, skip_serializing_if = "Option::is_none")]
    pub failover_disk_name: Option<String>,
    #[doc = "The target disk name for test failover operation."]
    #[serde(rename = "tfoDiskName", default, skip_serializing_if = "Option::is_none")]
    pub tfo_disk_name: Option<String>,
}
impl A2aVmManagedDiskUpdateDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Zone details data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct A2aZoneDetails {
    #[doc = "Source zone info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "The target zone info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl A2aZoneDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Task of the Job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AsrTask {
    #[doc = "The Id."]
    #[serde(rename = "taskId", default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[doc = "The unique Task name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The start time."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The state/actions applicable on this task."]
    #[serde(rename = "allowedActions", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_actions: Vec<String>,
    #[doc = "The name."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The State. It is one of these values - NotStarted, InProgress, Succeeded, Failed, Cancelled, Suspended or Other."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The description of the task state. For example - For Succeeded state, description can be Completed, PartiallySucceeded, CompletedWithInformation or Skipped."]
    #[serde(rename = "stateDescription", default, skip_serializing_if = "Option::is_none")]
    pub state_description: Option<String>,
    #[doc = "The type of task. Details in CustomDetails property depend on this type."]
    #[serde(rename = "taskType", default, skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    #[doc = "Task details based on specific task type."]
    #[serde(rename = "customDetails", default, skip_serializing_if = "Option::is_none")]
    pub custom_details: Option<TaskTypeDetails>,
    #[doc = "This class represents the group task details when parent child relationship exists in the drill down."]
    #[serde(rename = "groupTaskCustomDetails", default, skip_serializing_if = "Option::is_none")]
    pub group_task_custom_details: Option<GroupTaskDetails>,
    #[doc = "The task error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<JobErrorDetails>,
}
impl AsrTask {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Input for add disk(s) operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddDisksInput {
    #[doc = "Add Disks input properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AddDisksInputProperties>,
}
impl AddDisksInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Add Disks input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddDisksInputProperties {
    #[doc = "Add Disks provider specific input."]
    #[serde(rename = "providerSpecificDetails")]
    pub provider_specific_details: AddDisksProviderSpecificInput,
}
impl AddDisksInputProperties {
    pub fn new(provider_specific_details: AddDisksProviderSpecificInput) -> Self {
        Self { provider_specific_details }
    }
}
#[doc = "Add Disks provider specific input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddDisksProviderSpecificInput {
    #[doc = "The class type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl AddDisksProviderSpecificInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Input required to add a provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddRecoveryServicesProviderInput {
    #[doc = "The properties of an add provider request."]
    pub properties: AddRecoveryServicesProviderInputProperties,
}
impl AddRecoveryServicesProviderInput {
    pub fn new(properties: AddRecoveryServicesProviderInputProperties) -> Self {
        Self { properties }
    }
}
#[doc = "The properties of an add provider request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddRecoveryServicesProviderInputProperties {
    #[doc = "The name of the machine where the provider is getting added."]
    #[serde(rename = "machineName")]
    pub machine_name: String,
    #[doc = "The Id of the machine where the provider is getting added."]
    #[serde(rename = "machineId", default, skip_serializing_if = "Option::is_none")]
    pub machine_id: Option<String>,
    #[doc = "The Bios Id of the machine."]
    #[serde(rename = "biosId", default, skip_serializing_if = "Option::is_none")]
    pub bios_id: Option<String>,
    #[doc = "Identity provider input."]
    #[serde(rename = "authenticationIdentityInput")]
    pub authentication_identity_input: IdentityProviderInput,
    #[doc = "Identity provider input."]
    #[serde(rename = "resourceAccessIdentityInput")]
    pub resource_access_identity_input: IdentityProviderInput,
    #[doc = "Identity provider input."]
    #[serde(rename = "dataPlaneAuthenticationIdentityInput", default, skip_serializing_if = "Option::is_none")]
    pub data_plane_authentication_identity_input: Option<IdentityProviderInput>,
}
impl AddRecoveryServicesProviderInputProperties {
    pub fn new(
        machine_name: String,
        authentication_identity_input: IdentityProviderInput,
        resource_access_identity_input: IdentityProviderInput,
    ) -> Self {
        Self {
            machine_name,
            machine_id: None,
            bios_id: None,
            authentication_identity_input,
            resource_access_identity_input,
            data_plane_authentication_identity_input: None,
        }
    }
}
#[doc = "Input required to add vCenter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddVCenterRequest {
    #[doc = "The properties of an add vCenter request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AddVCenterRequestProperties>,
}
impl AddVCenterRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an add vCenter request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddVCenterRequestProperties {
    #[doc = "The friendly name of the vCenter."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The IP address of the vCenter to be discovered."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The process server Id from where the discovery is orchestrated."]
    #[serde(rename = "processServerId", default, skip_serializing_if = "Option::is_none")]
    pub process_server_id: Option<String>,
    #[doc = "The port number for discovery."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[doc = "The account Id which has privileges to discover the vCenter."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
}
impl AddVCenterRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Agent details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentDetails {
    #[doc = "The Id of the agent running on the server."]
    #[serde(rename = "agentId", default, skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[doc = "The Id of the machine to which the agent is registered."]
    #[serde(rename = "machineId", default, skip_serializing_if = "Option::is_none")]
    pub machine_id: Option<String>,
    #[doc = "The machine BIOS Id."]
    #[serde(rename = "biosId", default, skip_serializing_if = "Option::is_none")]
    pub bios_id: Option<String>,
    #[doc = "The machine FQDN."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "The disks."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<AgentDiskDetails>,
}
impl AgentDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Agent disk details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AgentDiskDetails {
    #[doc = "The disk Id."]
    #[serde(rename = "diskId", default, skip_serializing_if = "Option::is_none")]
    pub disk_id: Option<String>,
    #[doc = "The disk name."]
    #[serde(rename = "diskName", default, skip_serializing_if = "Option::is_none")]
    pub disk_name: Option<String>,
    #[doc = "A value indicating whether the disk is the OS disk."]
    #[serde(rename = "isOSDisk", default, skip_serializing_if = "Option::is_none")]
    pub is_os_disk: Option<String>,
    #[doc = "The disk capacity in bytes."]
    #[serde(rename = "capacityInBytes", default, skip_serializing_if = "Option::is_none")]
    pub capacity_in_bytes: Option<i64>,
    #[doc = "The lun of disk."]
    #[serde(rename = "lunId", default, skip_serializing_if = "Option::is_none")]
    pub lun_id: Option<i32>,
}
impl AgentDiskDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Implements the Alert class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Alert {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of an alert."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertProperties>,
}
impl Alert {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of alerts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertCollection {
    #[doc = "The list of alerts."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Alert>,
    #[doc = "The value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AlertCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AlertCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an alert."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertProperties {
    #[doc = "A value indicating whether to send email to subscription administrator."]
    #[serde(rename = "sendToOwners", default, skip_serializing_if = "Option::is_none")]
    pub send_to_owners: Option<String>,
    #[doc = "The custom email address for sending emails."]
    #[serde(rename = "customEmailAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub custom_email_addresses: Vec<String>,
    #[doc = "The locale for the email notification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}
impl AlertProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of appliance details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplianceCollection {
    #[doc = "The appliance details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReplicationAppliance>,
    #[doc = "The value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApplianceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ApplianceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Query parameter to get appliance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplianceQueryParameter {
    #[doc = "The providerType to be used for fetching appliance details."]
    #[serde(rename = "providerType", default, skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
}
impl ApplianceQueryParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Appliance specific details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplianceSpecificDetails {
    #[doc = "Gets the class type. Overridden in derived classes."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl ApplianceSpecificDetails {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Input to apply recovery point."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplyRecoveryPointInput {
    #[doc = "Input properties to apply recovery point."]
    pub properties: ApplyRecoveryPointInputProperties,
}
impl ApplyRecoveryPointInput {
    pub fn new(properties: ApplyRecoveryPointInputProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Input properties to apply recovery point."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplyRecoveryPointInputProperties {
    #[doc = "The recovery point Id."]
    #[serde(rename = "recoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_id: Option<String>,
    #[doc = "Provider specific input for apply recovery point."]
    #[serde(rename = "providerSpecificDetails")]
    pub provider_specific_details: ApplyRecoveryPointProviderSpecificInput,
}
impl ApplyRecoveryPointInputProperties {
    pub fn new(provider_specific_details: ApplyRecoveryPointProviderSpecificInput) -> Self {
        Self {
            recovery_point_id: None,
            provider_specific_details,
        }
    }
}
#[doc = "Provider specific input for apply recovery point."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplyRecoveryPointProviderSpecificInput {
    #[doc = "The class type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl ApplyRecoveryPointProviderSpecificInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "This class represents job details based on specific job type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AsrJobDetails {
    #[serde(flatten)]
    pub job_details: JobDetails,
}
impl AsrJobDetails {
    pub fn new(job_details: JobDetails) -> Self {
        Self { job_details }
    }
}
#[doc = "This class represents the task details for an automation runbook."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomationRunbookTaskDetails {
    #[serde(flatten)]
    pub task_type_details: TaskTypeDetails,
    #[doc = "The recovery plan task name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The cloud service of the automation runbook account."]
    #[serde(rename = "cloudServiceName", default, skip_serializing_if = "Option::is_none")]
    pub cloud_service_name: Option<String>,
    #[doc = "The subscription Id of the automation runbook account."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The automation account name of the runbook."]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "The runbook Id."]
    #[serde(rename = "runbookId", default, skip_serializing_if = "Option::is_none")]
    pub runbook_id: Option<String>,
    #[doc = "The runbook name."]
    #[serde(rename = "runbookName", default, skip_serializing_if = "Option::is_none")]
    pub runbook_name: Option<String>,
    #[doc = "The job Id of the runbook execution."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "The execution output of the runbook."]
    #[serde(rename = "jobOutput", default, skip_serializing_if = "Option::is_none")]
    pub job_output: Option<String>,
    #[doc = "A value indicating whether it is a primary side script or not."]
    #[serde(rename = "isPrimarySideScript", default, skip_serializing_if = "Option::is_none")]
    pub is_primary_side_script: Option<bool>,
}
impl AutomationRunbookTaskDetails {
    pub fn new(task_type_details: TaskTypeDetails) -> Self {
        Self {
            task_type_details,
            name: None,
            cloud_service_name: None,
            subscription_id: None,
            account_name: None,
            runbook_id: None,
            runbook_name: None,
            job_id: None,
            job_output: None,
            is_primary_side_script: None,
        }
    }
}
#[doc = "Fabric provider specific settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFabricCreationInput {
    #[serde(flatten)]
    pub fabric_specific_creation_input: FabricSpecificCreationInput,
    #[doc = "The Location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl AzureFabricCreationInput {
    pub fn new(fabric_specific_creation_input: FabricSpecificCreationInput) -> Self {
        Self {
            fabric_specific_creation_input,
            location: None,
        }
    }
}
#[doc = "Azure Fabric Specific Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFabricSpecificDetails {
    #[serde(flatten)]
    pub fabric_specific_details: FabricSpecificDetails,
    #[doc = "The Location for the Azure fabric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The container Ids for the Azure fabric."]
    #[serde(rename = "containerIds", default, skip_serializing_if = "Vec::is_empty")]
    pub container_ids: Vec<String>,
    #[doc = "The zones."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<A2aZoneDetails>,
}
impl AzureFabricSpecificDetails {
    pub fn new(fabric_specific_details: FabricSpecificDetails) -> Self {
        Self {
            fabric_specific_details,
            location: None,
            container_ids: Vec::new(),
            zones: Vec::new(),
        }
    }
}
#[doc = "Create network mappings input properties/behavior specific to Azure to Azure Network mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureToAzureCreateNetworkMappingInput {
    #[serde(flatten)]
    pub fabric_specific_create_network_mapping_input: FabricSpecificCreateNetworkMappingInput,
    #[doc = "The primary azure vnet Id."]
    #[serde(rename = "primaryNetworkId")]
    pub primary_network_id: String,
}
impl AzureToAzureCreateNetworkMappingInput {
    pub fn new(fabric_specific_create_network_mapping_input: FabricSpecificCreateNetworkMappingInput, primary_network_id: String) -> Self {
        Self {
            fabric_specific_create_network_mapping_input,
            primary_network_id,
        }
    }
}
#[doc = "A2A Network Mapping fabric specific settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureToAzureNetworkMappingSettings {
    #[serde(flatten)]
    pub network_mapping_fabric_specific_settings: NetworkMappingFabricSpecificSettings,
    #[doc = "The primary fabric location."]
    #[serde(rename = "primaryFabricLocation", default, skip_serializing_if = "Option::is_none")]
    pub primary_fabric_location: Option<String>,
    #[doc = "The recovery fabric location."]
    #[serde(rename = "recoveryFabricLocation", default, skip_serializing_if = "Option::is_none")]
    pub recovery_fabric_location: Option<String>,
}
impl AzureToAzureNetworkMappingSettings {
    pub fn new(network_mapping_fabric_specific_settings: NetworkMappingFabricSpecificSettings) -> Self {
        Self {
            network_mapping_fabric_specific_settings,
            primary_fabric_location: None,
            recovery_fabric_location: None,
        }
    }
}
#[doc = "Updates network mappings input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureToAzureUpdateNetworkMappingInput {
    #[serde(flatten)]
    pub fabric_specific_update_network_mapping_input: FabricSpecificUpdateNetworkMappingInput,
    #[doc = "The primary azure vnet Id."]
    #[serde(rename = "primaryNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub primary_network_id: Option<String>,
}
impl AzureToAzureUpdateNetworkMappingInput {
    pub fn new(fabric_specific_update_network_mapping_input: FabricSpecificUpdateNetworkMappingInput) -> Self {
        Self {
            fabric_specific_update_network_mapping_input,
            primary_network_id: None,
        }
    }
}
#[doc = "Azure to Azure VM synced configuration details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureToAzureVmSyncedConfigDetails {
    #[doc = "The Azure VM tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The Azure VM input endpoints."]
    #[serde(rename = "inputEndpoints", default, skip_serializing_if = "Vec::is_empty")]
    pub input_endpoints: Vec<InputEndpoint>,
}
impl AzureToAzureVmSyncedConfigDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Disk details for E2A provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureVmDiskDetails {
    #[doc = "VHD type."]
    #[serde(rename = "vhdType", default, skip_serializing_if = "Option::is_none")]
    pub vhd_type: Option<String>,
    #[doc = "The VHD id."]
    #[serde(rename = "vhdId", default, skip_serializing_if = "Option::is_none")]
    pub vhd_id: Option<String>,
    #[doc = "The disk resource id."]
    #[serde(rename = "diskId", default, skip_serializing_if = "Option::is_none")]
    pub disk_id: Option<String>,
    #[doc = "VHD name."]
    #[serde(rename = "vhdName", default, skip_serializing_if = "Option::is_none")]
    pub vhd_name: Option<String>,
    #[doc = "Max side in MB."]
    #[serde(rename = "maxSizeMB", default, skip_serializing_if = "Option::is_none")]
    pub max_size_mb: Option<String>,
    #[doc = "Blob uri of the Azure disk."]
    #[serde(rename = "targetDiskLocation", default, skip_serializing_if = "Option::is_none")]
    pub target_disk_location: Option<String>,
    #[doc = "The target Azure disk name."]
    #[serde(rename = "targetDiskName", default, skip_serializing_if = "Option::is_none")]
    pub target_disk_name: Option<String>,
    #[doc = "Ordinal\\LunId of the disk for the Azure VM."]
    #[serde(rename = "lunId", default, skip_serializing_if = "Option::is_none")]
    pub lun_id: Option<String>,
    #[doc = "The DiskEncryptionSet ARM ID."]
    #[serde(rename = "diskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set_id: Option<String>,
    #[doc = "The custom target Azure disk name."]
    #[serde(rename = "customTargetDiskName", default, skip_serializing_if = "Option::is_none")]
    pub custom_target_disk_name: Option<String>,
}
impl AzureVmDiskDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the error used to indicate why the target compute size is not applicable."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ComputeSizeErrorDetails {
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The severity of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}
impl ComputeSizeErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Replication provider specific settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationSettings {
    #[doc = "Gets the class type. Overridden in derived classes."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl ConfigurationSettings {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Request to configure alerts for the system."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigureAlertRequest {
    #[doc = "Properties of a configure alert request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfigureAlertRequestProperties>,
}
impl ConfigureAlertRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a configure alert request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigureAlertRequestProperties {
    #[doc = "A value indicating whether to send email to subscription administrator."]
    #[serde(rename = "sendToOwners", default, skip_serializing_if = "Option::is_none")]
    pub send_to_owners: Option<String>,
    #[doc = "The custom email address for sending emails."]
    #[serde(rename = "customEmailAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub custom_email_addresses: Vec<String>,
    #[doc = "The locale for the email notification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}
impl ConfigureAlertRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This class contains monitoring details of all the inconsistent Protected Entities in Vmm."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsistencyCheckTaskDetails {
    #[serde(flatten)]
    pub task_type_details: TaskTypeDetails,
    #[doc = "The list of inconsistent Vm details."]
    #[serde(rename = "vmDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_details: Vec<InconsistentVmDetails>,
}
impl ConsistencyCheckTaskDetails {
    pub fn new(task_type_details: TaskTypeDetails) -> Self {
        Self {
            task_type_details,
            vm_details: Vec::new(),
        }
    }
}
#[doc = "Create network mappings input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateNetworkMappingInput {
    #[doc = "Common input details for network mapping operation."]
    pub properties: CreateNetworkMappingInputProperties,
}
impl CreateNetworkMappingInput {
    pub fn new(properties: CreateNetworkMappingInputProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Common input details for network mapping operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateNetworkMappingInputProperties {
    #[doc = "Recovery fabric Name."]
    #[serde(rename = "recoveryFabricName", default, skip_serializing_if = "Option::is_none")]
    pub recovery_fabric_name: Option<String>,
    #[doc = "Recovery network Id."]
    #[serde(rename = "recoveryNetworkId")]
    pub recovery_network_id: String,
    #[doc = "Input details specific to fabrics during Network Mapping."]
    #[serde(rename = "fabricSpecificDetails", default, skip_serializing_if = "Option::is_none")]
    pub fabric_specific_details: Option<FabricSpecificCreateNetworkMappingInput>,
}
impl CreateNetworkMappingInputProperties {
    pub fn new(recovery_network_id: String) -> Self {
        Self {
            recovery_fabric_name: None,
            recovery_network_id,
            fabric_specific_details: None,
        }
    }
}
#[doc = "Protection Policy input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreatePolicyInput {
    #[doc = "Policy creation properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CreatePolicyInputProperties>,
}
impl CreatePolicyInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy creation properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreatePolicyInputProperties {
    #[doc = "Base class for provider specific input."]
    #[serde(rename = "providerSpecificInput", default, skip_serializing_if = "Option::is_none")]
    pub provider_specific_input: Option<PolicyProviderSpecificInput>,
}
impl CreatePolicyInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Create protection container input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateProtectionContainerInput {
    #[doc = "Create protection container input properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CreateProtectionContainerInputProperties>,
}
impl CreateProtectionContainerInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Create protection container input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateProtectionContainerInputProperties {
    #[doc = "Provider specific inputs for container creation."]
    #[serde(rename = "providerSpecificInput", default, skip_serializing_if = "Vec::is_empty")]
    pub provider_specific_input: Vec<ReplicationProviderSpecificContainerCreationInput>,
}
impl CreateProtectionContainerInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configure pairing input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateProtectionContainerMappingInput {
    #[doc = "Configure pairing input properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CreateProtectionContainerMappingInputProperties>,
}
impl CreateProtectionContainerMappingInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configure pairing input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateProtectionContainerMappingInputProperties {
    #[doc = "The target unique protection container name."]
    #[serde(rename = "targetProtectionContainerId", default, skip_serializing_if = "Option::is_none")]
    pub target_protection_container_id: Option<String>,
    #[doc = "Applicable policy."]
    #[serde(rename = "policyId", default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[doc = "Provider specific input for pairing operations."]
    #[serde(rename = "providerSpecificInput", default, skip_serializing_if = "Option::is_none")]
    pub provider_specific_input: Option<ReplicationProviderSpecificContainerMappingInput>,
}
impl CreateProtectionContainerMappingInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Create protection intent input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateProtectionIntentInput {
    #[doc = "Create protection intent input properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CreateProtectionIntentProperties>,
}
impl CreateProtectionIntentInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Create protection intent input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateProtectionIntentProperties {
    #[doc = "Create protection intent provider specific input."]
    #[serde(rename = "providerSpecificDetails", default, skip_serializing_if = "Option::is_none")]
    pub provider_specific_details: Option<CreateProtectionIntentProviderSpecificDetails>,
}
impl CreateProtectionIntentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Create protection intent provider specific input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateProtectionIntentProviderSpecificDetails {
    #[doc = "The class type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl CreateProtectionIntentProviderSpecificDetails {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Create recovery plan input class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateRecoveryPlanInput {
    #[doc = "Recovery plan creation properties."]
    pub properties: CreateRecoveryPlanInputProperties,
}
impl CreateRecoveryPlanInput {
    pub fn new(properties: CreateRecoveryPlanInputProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Recovery plan creation properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateRecoveryPlanInputProperties {
    #[doc = "The primary fabric Id."]
    #[serde(rename = "primaryFabricId")]
    pub primary_fabric_id: String,
    #[doc = "The recovery fabric Id."]
    #[serde(rename = "recoveryFabricId")]
    pub recovery_fabric_id: String,
    #[doc = "The failover deployment model."]
    #[serde(rename = "failoverDeploymentModel", default, skip_serializing_if = "Option::is_none")]
    pub failover_deployment_model: Option<create_recovery_plan_input_properties::FailoverDeploymentModel>,
    #[doc = "The recovery plan groups."]
    pub groups: Vec<RecoveryPlanGroup>,
    #[doc = "The provider specific input."]
    #[serde(rename = "providerSpecificInput", default, skip_serializing_if = "Vec::is_empty")]
    pub provider_specific_input: Vec<RecoveryPlanProviderSpecificInput>,
}
impl CreateRecoveryPlanInputProperties {
    pub fn new(primary_fabric_id: String, recovery_fabric_id: String, groups: Vec<RecoveryPlanGroup>) -> Self {
        Self {
            primary_fabric_id,
            recovery_fabric_id,
            failover_deployment_model: None,
            groups,
            provider_specific_input: Vec::new(),
        }
    }
}
pub mod create_recovery_plan_input_properties {
    use super::*;
    #[doc = "The failover deployment model."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FailoverDeploymentModel")]
    pub enum FailoverDeploymentModel {
        NotApplicable,
        Classic,
        ResourceManager,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FailoverDeploymentModel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FailoverDeploymentModel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FailoverDeploymentModel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotApplicable => serializer.serialize_unit_variant("FailoverDeploymentModel", 0u32, "NotApplicable"),
                Self::Classic => serializer.serialize_unit_variant("FailoverDeploymentModel", 1u32, "Classic"),
                Self::ResourceManager => serializer.serialize_unit_variant("FailoverDeploymentModel", 2u32, "ResourceManager"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Current job details of the migration item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CurrentJobDetails {
    #[doc = "The job name."]
    #[serde(rename = "jobName", default, skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[doc = "The ARM Id of the job being executed."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "The start time of the job."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
}
impl CurrentJobDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Current scenario details of the protected entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CurrentScenarioDetails {
    #[doc = "Scenario name."]
    #[serde(rename = "scenarioName", default, skip_serializing_if = "Option::is_none")]
    pub scenario_name: Option<String>,
    #[doc = "ARM Id of the job being executed."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "Start time of the workflow."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
}
impl CurrentScenarioDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The datastore details of the MT."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataStore {
    #[doc = "The symbolic name of data store."]
    #[serde(rename = "symbolicName", default, skip_serializing_if = "Option::is_none")]
    pub symbolic_name: Option<String>,
    #[doc = "The uuid of data store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "The capacity of data store in GBs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<String>,
    #[doc = "The free space of data store in GBs."]
    #[serde(rename = "freeSpace", default, skip_serializing_if = "Option::is_none")]
    pub free_space: Option<String>,
    #[doc = "The type of data store."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl DataStore {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Disable protection input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DisableProtectionInput {
    #[doc = "Disable protection input properties."]
    pub properties: DisableProtectionInputProperties,
}
impl DisableProtectionInput {
    pub fn new(properties: DisableProtectionInputProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Disable protection input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DisableProtectionInputProperties {
    #[doc = "Disable protection reason. It can have values NotSpecified/MigrationComplete."]
    #[serde(rename = "disableProtectionReason", default, skip_serializing_if = "Option::is_none")]
    pub disable_protection_reason: Option<disable_protection_input_properties::DisableProtectionReason>,
    #[doc = "Disable protection provider specific input."]
    #[serde(rename = "replicationProviderInput", default, skip_serializing_if = "Option::is_none")]
    pub replication_provider_input: Option<DisableProtectionProviderSpecificInput>,
}
impl DisableProtectionInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod disable_protection_input_properties {
    use super::*;
    #[doc = "Disable protection reason. It can have values NotSpecified/MigrationComplete."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DisableProtectionReason")]
    pub enum DisableProtectionReason {
        NotSpecified,
        MigrationComplete,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DisableProtectionReason {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DisableProtectionReason {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DisableProtectionReason {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("DisableProtectionReason", 0u32, "NotSpecified"),
                Self::MigrationComplete => serializer.serialize_unit_variant("DisableProtectionReason", 1u32, "MigrationComplete"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Disable protection provider specific input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DisableProtectionProviderSpecificInput {
    #[doc = "The class type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl DisableProtectionProviderSpecificInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Request to add a physical machine as a protectable item in a container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiscoverProtectableItemRequest {
    #[doc = "Discover protectable item properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiscoverProtectableItemRequestProperties>,
}
impl DiscoverProtectableItemRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Discover protectable item properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiscoverProtectableItemRequestProperties {
    #[doc = "The friendly name of the physical machine."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The IP address of the physical machine to be discovered."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The OS type on the physical machine."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
}
impl DiscoverProtectableItemRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Onprem disk details data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskDetails {
    #[doc = "The hard disk max size in MB."]
    #[serde(rename = "maxSizeMB", default, skip_serializing_if = "Option::is_none")]
    pub max_size_mb: Option<i64>,
    #[doc = "The type of the volume."]
    #[serde(rename = "vhdType", default, skip_serializing_if = "Option::is_none")]
    pub vhd_type: Option<String>,
    #[doc = "The VHD Id."]
    #[serde(rename = "vhdId", default, skip_serializing_if = "Option::is_none")]
    pub vhd_id: Option<String>,
    #[doc = "The VHD name."]
    #[serde(rename = "vhdName", default, skip_serializing_if = "Option::is_none")]
    pub vhd_name: Option<String>,
}
impl DiskDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Recovery disk encryption info (BEK and KEK)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskEncryptionInfo {
    #[doc = "Disk Encryption Key Information (BitLocker Encryption Key (BEK) on Windows)."]
    #[serde(rename = "diskEncryptionKeyInfo", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_key_info: Option<DiskEncryptionKeyInfo>,
    #[doc = "Key Encryption Key (KEK) information."]
    #[serde(rename = "keyEncryptionKeyInfo", default, skip_serializing_if = "Option::is_none")]
    pub key_encryption_key_info: Option<KeyEncryptionKeyInfo>,
}
impl DiskEncryptionInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Disk Encryption Key Information (BitLocker Encryption Key (BEK) on Windows)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskEncryptionKeyInfo {
    #[doc = "The secret url / identifier."]
    #[serde(rename = "secretIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub secret_identifier: Option<String>,
    #[doc = "The KeyVault resource ARM id for secret."]
    #[serde(rename = "keyVaultResourceArmId", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_resource_arm_id: Option<String>,
}
impl DiskEncryptionKeyInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Volume details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskVolumeDetails {
    #[doc = "The volume label."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "The volume name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl DiskVolumeDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains the localized display information for this particular operation / action. These value will be used by several clients for (1) custom role definitions for RBAC; (2) complex query filters for the event service; and (3) audit history / records for management operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Display {
    #[doc = "The provider. The localized friendly form of the resource provider name - it is expected to also include the publisher/company responsible. It should use Title Casing and begin with \"Microsoft\" for 1st party services. e.g. \"Microsoft Monitoring Insights\" or \"Microsoft Compute.\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The resource. The localized friendly form of the resource related to this action/operation - it should match the public documentation for the resource provider. It should use Title Casing. This value should be unique for a particular URL type (e.g. nested types should *not* reuse their parent's display.resource field). e.g. \"Virtual Machines\" or \"Scheduler Job Collections\", or \"Virtual Machine VM Sizes\" or \"Scheduler Jobs\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The operation. The localized friendly name for the operation, as it should be shown to the user. It should be concise (to fit in drop downs) but clear (i.e. self-documenting). It should use Title Casing. Prescriptive guidance: Read Create or Update Delete 'ActionName'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The description. The localized friendly description for the operation, as it should be shown to the user. It should be thorough, yet concise - it will be used in tool tips and detailed views. Prescriptive guidance for namespaces: Read any 'display.provider' resource Create or Update any 'display.provider' resource Delete any 'display.provider' resource Perform any other action on any 'display.provider' resource Prescriptive guidance for namespaces: Read any 'display.resource' Create or Update any 'display.resource' Delete any 'display.resource' 'ActionName' any 'display.resources'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl Display {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DRA details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DraDetails {
    #[doc = "The DRA Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The DRA name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The DRA Bios Id."]
    #[serde(rename = "biosId", default, skip_serializing_if = "Option::is_none")]
    pub bios_id: Option<String>,
    #[doc = "The version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The last heartbeat received from the DRA."]
    #[serde(rename = "lastHeartbeatUtc", with = "azure_core::date::rfc3339::option")]
    pub last_heartbeat_utc: Option<time::OffsetDateTime>,
    #[doc = "The health."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<dra_details::Health>,
    #[doc = "The health errors."]
    #[serde(rename = "healthErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub health_errors: Vec<HealthError>,
    #[doc = "The count of protected items which are protected in forward direction."]
    #[serde(rename = "forwardProtectedItemCount", default, skip_serializing_if = "Option::is_none")]
    pub forward_protected_item_count: Option<i32>,
    #[doc = "The count of protected items which are protected in reverse direction."]
    #[serde(rename = "reverseProtectedItemCount", default, skip_serializing_if = "Option::is_none")]
    pub reverse_protected_item_count: Option<i32>,
}
impl DraDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod dra_details {
    use super::*;
    #[doc = "The health."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Health")]
    pub enum Health {
        None,
        Normal,
        Warning,
        Critical,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Health {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Health {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Health {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Health", 0u32, "None"),
                Self::Normal => serializer.serialize_unit_variant("Health", 1u32, "Normal"),
                Self::Warning => serializer.serialize_unit_variant("Health", 2u32, "Warning"),
                Self::Critical => serializer.serialize_unit_variant("Health", 3u32, "Critical"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Enable migration input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnableMigrationInput {
    #[doc = "Enable migration input properties."]
    pub properties: EnableMigrationInputProperties,
}
impl EnableMigrationInput {
    pub fn new(properties: EnableMigrationInputProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Enable migration input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnableMigrationInputProperties {
    #[doc = "The policy Id."]
    #[serde(rename = "policyId")]
    pub policy_id: String,
    #[doc = "Enable migration provider specific input."]
    #[serde(rename = "providerSpecificDetails")]
    pub provider_specific_details: EnableMigrationProviderSpecificInput,
}
impl EnableMigrationInputProperties {
    pub fn new(policy_id: String, provider_specific_details: EnableMigrationProviderSpecificInput) -> Self {
        Self {
            policy_id,
            provider_specific_details,
        }
    }
}
#[doc = "Enable migration provider specific input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnableMigrationProviderSpecificInput {
    #[doc = "The class type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl EnableMigrationProviderSpecificInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Enable protection input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnableProtectionInput {
    #[doc = "Enable protection input properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnableProtectionInputProperties>,
}
impl EnableProtectionInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enable protection input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnableProtectionInputProperties {
    #[doc = "The Policy Id."]
    #[serde(rename = "policyId", default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[doc = "The protectable item Id."]
    #[serde(rename = "protectableItemId", default, skip_serializing_if = "Option::is_none")]
    pub protectable_item_id: Option<String>,
    #[doc = "Enable protection provider specific input."]
    #[serde(rename = "providerSpecificDetails", default, skip_serializing_if = "Option::is_none")]
    pub provider_specific_details: Option<EnableProtectionProviderSpecificInput>,
}
impl EnableProtectionInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Enable protection provider specific input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnableProtectionProviderSpecificInput {
    #[doc = "The class type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl EnableProtectionProviderSpecificInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Encryption details for the fabric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionDetails {
    #[doc = "The key encryption key state for the Vmm."]
    #[serde(rename = "kekState", default, skip_serializing_if = "Option::is_none")]
    pub kek_state: Option<String>,
    #[doc = "The key encryption key certificate thumbprint."]
    #[serde(rename = "kekCertThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub kek_cert_thumbprint: Option<String>,
    #[doc = "The key encryption key certificate expiry date."]
    #[serde(rename = "kekCertExpiryDate", with = "azure_core::date::rfc3339::option")]
    pub kek_cert_expiry_date: Option<time::OffsetDateTime>,
}
impl EncryptionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Implements the Event class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Event {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of a monitoring event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventProperties>,
}
impl Event {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of fabric details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventCollection {
    #[doc = "The list of events."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Event>,
    #[doc = "The value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EventCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EventCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a monitoring event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventProperties {
    #[doc = "The Id of the monitoring event."]
    #[serde(rename = "eventCode", default, skip_serializing_if = "Option::is_none")]
    pub event_code: Option<String>,
    #[doc = "The event name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The type of the event. for example: VM Health, Server Health, Job Failure etc."]
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[doc = "The friendly name of the source of the event on which it is raised (for example, VM, VMM etc)."]
    #[serde(rename = "affectedObjectFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub affected_object_friendly_name: Option<String>,
    #[doc = "The affected object correlationId for the event."]
    #[serde(rename = "affectedObjectCorrelationId", default, skip_serializing_if = "Option::is_none")]
    pub affected_object_correlation_id: Option<String>,
    #[doc = "The severity of the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[doc = "The time of occurrence of the event."]
    #[serde(rename = "timeOfOccurrence", with = "azure_core::date::rfc3339::option")]
    pub time_of_occurrence: Option<time::OffsetDateTime>,
    #[doc = "The ARM ID of the fabric."]
    #[serde(rename = "fabricId", default, skip_serializing_if = "Option::is_none")]
    pub fabric_id: Option<String>,
    #[doc = "Model class for provider specific details for an event."]
    #[serde(rename = "providerSpecificDetails", default, skip_serializing_if = "Option::is_none")]
    pub provider_specific_details: Option<EventProviderSpecificDetails>,
    #[doc = "Model class for event specific details for an event."]
    #[serde(rename = "eventSpecificDetails", default, skip_serializing_if = "Option::is_none")]
    pub event_specific_details: Option<EventSpecificDetails>,
    #[doc = "The list of errors / warnings capturing details associated with the issue(s)."]
    #[serde(rename = "healthErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub health_errors: Vec<HealthError>,
}
impl EventProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model class for provider specific details for an event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventProviderSpecificDetails {
    #[doc = "Gets the class type. Overridden in derived classes."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl EventProviderSpecificDetails {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Implements the event query parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventQueryParameter {
    #[doc = "The source id of the events to be queried."]
    #[serde(rename = "eventCode", default, skip_serializing_if = "Option::is_none")]
    pub event_code: Option<String>,
    #[doc = "The severity of the events to be queried."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[doc = "The type of the events to be queried."]
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[doc = "The affected object server id of the events to be queried."]
    #[serde(rename = "fabricName", default, skip_serializing_if = "Option::is_none")]
    pub fabric_name: Option<String>,
    #[doc = "The affected object name of the events to be queried."]
    #[serde(rename = "affectedObjectFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub affected_object_friendly_name: Option<String>,
    #[doc = "The affected object correlationId for the events to be queried."]
    #[serde(rename = "affectedObjectCorrelationId", default, skip_serializing_if = "Option::is_none")]
    pub affected_object_correlation_id: Option<String>,
    #[doc = "The start time of the time range within which the events are to be queried."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time of the time range within which the events are to be queried."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
}
impl EventQueryParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model class for event specific details for an event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventSpecificDetails {
    #[doc = "Gets the class type. Overridden in derived classes."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl EventSpecificDetails {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Existing storage account input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExistingProtectionProfile {
    #[serde(flatten)]
    pub protection_profile_custom_details: ProtectionProfileCustomDetails,
    #[doc = "The protection profile Arm Id. Throw error, if resource does not exists."]
    #[serde(rename = "protectionProfileId")]
    pub protection_profile_id: String,
}
impl ExistingProtectionProfile {
    pub fn new(protection_profile_custom_details: ProtectionProfileCustomDetails, protection_profile_id: String) -> Self {
        Self {
            protection_profile_custom_details,
            protection_profile_id,
        }
    }
}
#[doc = "Existing recovery availability set input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExistingRecoveryAvailabilitySet {
    #[serde(flatten)]
    pub recovery_availability_set_custom_details: RecoveryAvailabilitySetCustomDetails,
    #[doc = "The recovery availability set Id. Will throw error, if resource does not exist."]
    #[serde(rename = "recoveryAvailabilitySetId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_availability_set_id: Option<String>,
}
impl ExistingRecoveryAvailabilitySet {
    pub fn new(recovery_availability_set_custom_details: RecoveryAvailabilitySetCustomDetails) -> Self {
        Self {
            recovery_availability_set_custom_details,
            recovery_availability_set_id: None,
        }
    }
}
#[doc = "Existing recovery proximity placement group input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExistingRecoveryProximityPlacementGroup {
    #[serde(flatten)]
    pub recovery_proximity_placement_group_custom_details: RecoveryProximityPlacementGroupCustomDetails,
    #[doc = "The recovery proximity placement group Id. Will throw error, if resource does not exist."]
    #[serde(rename = "recoveryProximityPlacementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_proximity_placement_group_id: Option<String>,
}
impl ExistingRecoveryProximityPlacementGroup {
    pub fn new(recovery_proximity_placement_group_custom_details: RecoveryProximityPlacementGroupCustomDetails) -> Self {
        Self {
            recovery_proximity_placement_group_custom_details,
            recovery_proximity_placement_group_id: None,
        }
    }
}
#[doc = "Existing recovery resource group input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExistingRecoveryRecoveryResourceGroup {
    #[serde(flatten)]
    pub recovery_resource_group_custom_details: RecoveryResourceGroupCustomDetails,
    #[doc = "The recovery resource group Id. Valid for V2 scenarios."]
    #[serde(rename = "recoveryResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_resource_group_id: Option<String>,
}
impl ExistingRecoveryRecoveryResourceGroup {
    pub fn new(recovery_resource_group_custom_details: RecoveryResourceGroupCustomDetails) -> Self {
        Self {
            recovery_resource_group_custom_details,
            recovery_resource_group_id: None,
        }
    }
}
#[doc = "Existing recovery virtual network input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExistingRecoveryVirtualNetwork {
    #[serde(flatten)]
    pub recovery_virtual_network_custom_details: RecoveryVirtualNetworkCustomDetails,
    #[doc = "The recovery virtual network Id. Will throw error, if resource does not exist."]
    #[serde(rename = "recoveryVirtualNetworkId")]
    pub recovery_virtual_network_id: String,
    #[doc = "The recovery subnet name."]
    #[serde(rename = "recoverySubnetName", default, skip_serializing_if = "Option::is_none")]
    pub recovery_subnet_name: Option<String>,
}
impl ExistingRecoveryVirtualNetwork {
    pub fn new(recovery_virtual_network_custom_details: RecoveryVirtualNetworkCustomDetails, recovery_virtual_network_id: String) -> Self {
        Self {
            recovery_virtual_network_custom_details,
            recovery_virtual_network_id,
            recovery_subnet_name: None,
        }
    }
}
#[doc = "Existing storage account input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExistingStorageAccount {
    #[serde(flatten)]
    pub storage_account_custom_details: StorageAccountCustomDetails,
    #[doc = "The storage account Arm Id. Throw error, if resource does not exists."]
    #[serde(rename = "azureStorageAccountId")]
    pub azure_storage_account_id: String,
}
impl ExistingStorageAccount {
    pub fn new(storage_account_custom_details: StorageAccountCustomDetails, azure_storage_account_id: String) -> Self {
        Self {
            storage_account_custom_details,
            azure_storage_account_id,
        }
    }
}
#[doc = "This class represents details for export jobs workflow."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportJobDetails {
    #[serde(flatten)]
    pub job_details: JobDetails,
    #[doc = "BlobUri of the exported jobs."]
    #[serde(rename = "blobUri", default, skip_serializing_if = "Option::is_none")]
    pub blob_uri: Option<String>,
    #[doc = "The sas token to access blob."]
    #[serde(rename = "sasToken", default, skip_serializing_if = "Option::is_none")]
    pub sas_token: Option<String>,
}
impl ExportJobDetails {
    pub fn new(job_details: JobDetails) -> Self {
        Self {
            job_details,
            blob_uri: None,
            sas_token: None,
        }
    }
}
#[doc = "Extended location of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtendedLocation {
    #[doc = "The name of the extended location."]
    pub name: String,
    #[doc = "The extended location type."]
    #[serde(rename = "type")]
    pub type_: extended_location::Type,
}
impl ExtendedLocation {
    pub fn new(name: String, type_: extended_location::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod extended_location {
    use super::*;
    #[doc = "The extended location type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        EdgeZone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Type {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::EdgeZone => serializer.serialize_unit_variant("Type", 0u32, "EdgeZone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Fabric definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Fabric {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Fabric properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FabricProperties>,
}
impl Fabric {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of fabric details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FabricCollection {
    #[doc = "The fabric details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Fabric>,
    #[doc = "The value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FabricCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl FabricCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Site details provided during the time of site creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FabricCreationInput {
    #[doc = "Properties of site details provided during the time of site creation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FabricCreationInputProperties>,
}
impl FabricCreationInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of site details provided during the time of site creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FabricCreationInputProperties {
    #[doc = "Fabric provider specific settings."]
    #[serde(rename = "customDetails", default, skip_serializing_if = "Option::is_none")]
    pub custom_details: Option<FabricSpecificCreationInput>,
}
impl FabricCreationInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Fabric properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FabricProperties {
    #[doc = "Friendly name of the fabric."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Encryption details for the fabric."]
    #[serde(rename = "encryptionDetails", default, skip_serializing_if = "Option::is_none")]
    pub encryption_details: Option<EncryptionDetails>,
    #[doc = "Encryption details for the fabric."]
    #[serde(rename = "rolloverEncryptionDetails", default, skip_serializing_if = "Option::is_none")]
    pub rollover_encryption_details: Option<EncryptionDetails>,
    #[doc = "Dra Registration Id."]
    #[serde(rename = "internalIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub internal_identifier: Option<String>,
    #[doc = "BCDR state of the fabric."]
    #[serde(rename = "bcdrState", default, skip_serializing_if = "Option::is_none")]
    pub bcdr_state: Option<String>,
    #[doc = "Fabric specific details."]
    #[serde(rename = "customDetails", default, skip_serializing_if = "Option::is_none")]
    pub custom_details: Option<FabricSpecificDetails>,
    #[doc = "Fabric health error details."]
    #[serde(rename = "healthErrorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub health_error_details: Vec<HealthError>,
    #[doc = "Health of fabric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,
}
impl FabricProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Query parameter to get fabric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FabricQueryParameter {
    #[doc = "A value indicating whether the zone to zone mappings are to be returned."]
    #[serde(rename = "zoneToZoneMappings", default, skip_serializing_if = "Option::is_none")]
    pub zone_to_zone_mappings: Option<String>,
    #[doc = "A value indicating whether the agent details are to be fetched."]
    #[serde(rename = "fetchAgentDetails", default, skip_serializing_if = "Option::is_none")]
    pub fetch_agent_details: Option<String>,
    #[doc = "The BIOS Id to be used for fetching agent details."]
    #[serde(rename = "biosId", default, skip_serializing_if = "Option::is_none")]
    pub bios_id: Option<String>,
    #[doc = "The FQDN to be used for fetching agent details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "The type of the discovered machine to be used for fetching agent details."]
    #[serde(rename = "discoveryType", default, skip_serializing_if = "Option::is_none")]
    pub discovery_type: Option<String>,
    #[doc = "The OS type to be used for fetching agent details."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
}
impl FabricQueryParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This class represents the fabric replication group task details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FabricReplicationGroupTaskDetails {
    #[serde(flatten)]
    pub job_task_details: JobTaskDetails,
    #[doc = "The skipped reason."]
    #[serde(rename = "skippedReason", default, skip_serializing_if = "Option::is_none")]
    pub skipped_reason: Option<String>,
    #[doc = "The skipped reason string."]
    #[serde(rename = "skippedReasonString", default, skip_serializing_if = "Option::is_none")]
    pub skipped_reason_string: Option<String>,
}
impl FabricReplicationGroupTaskDetails {
    pub fn new(job_task_details: JobTaskDetails) -> Self {
        Self {
            job_task_details,
            skipped_reason: None,
            skipped_reason_string: None,
        }
    }
}
#[doc = "Input details specific to fabrics during Network Mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FabricSpecificCreateNetworkMappingInput {
    #[doc = "The instance type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl FabricSpecificCreateNetworkMappingInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Fabric provider specific settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FabricSpecificCreationInput {
    #[doc = "Gets the class type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl FabricSpecificCreationInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Fabric specific details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FabricSpecificDetails {
    #[doc = "Gets the class type. Overridden in derived classes."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl FabricSpecificDetails {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Input details specific to fabrics during Network Mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FabricSpecificUpdateNetworkMappingInput {
    #[doc = "The instance type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl FabricSpecificUpdateNetworkMappingInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "This class represents the details for a failover job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailoverJobDetails {
    #[serde(flatten)]
    pub job_details: JobDetails,
    #[doc = "The test VM details."]
    #[serde(rename = "protectedItemDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub protected_item_details: Vec<FailoverReplicationProtectedItemDetails>,
}
impl FailoverJobDetails {
    pub fn new(job_details: JobDetails) -> Self {
        Self {
            job_details,
            protected_item_details: Vec::new(),
        }
    }
}
#[doc = "Request to failover a process server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailoverProcessServerRequest {
    #[doc = "The properties of the Failover Process Server request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FailoverProcessServerRequestProperties>,
}
impl FailoverProcessServerRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the Failover Process Server request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailoverProcessServerRequestProperties {
    #[doc = "The container identifier."]
    #[serde(rename = "containerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[doc = "The source process server."]
    #[serde(rename = "sourceProcessServerId", default, skip_serializing_if = "Option::is_none")]
    pub source_process_server_id: Option<String>,
    #[doc = "The new process server."]
    #[serde(rename = "targetProcessServerId", default, skip_serializing_if = "Option::is_none")]
    pub target_process_server_id: Option<String>,
    #[doc = "The VMS to migrate."]
    #[serde(rename = "vmsToMigrate", default, skip_serializing_if = "Vec::is_empty")]
    pub vms_to_migrate: Vec<String>,
    #[doc = "A value for failover type. It can be systemlevel/serverlevel."]
    #[serde(rename = "updateType", default, skip_serializing_if = "Option::is_none")]
    pub update_type: Option<String>,
}
impl FailoverProcessServerRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Failover details for a replication protected item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailoverReplicationProtectedItemDetails {
    #[doc = "The name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The friendly name."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The test Vm name."]
    #[serde(rename = "testVmName", default, skip_serializing_if = "Option::is_none")]
    pub test_vm_name: Option<String>,
    #[doc = "The test Vm friendly name."]
    #[serde(rename = "testVmFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub test_vm_friendly_name: Option<String>,
    #[doc = "The network connection status."]
    #[serde(rename = "networkConnectionStatus", default, skip_serializing_if = "Option::is_none")]
    pub network_connection_status: Option<String>,
    #[doc = "The network friendly name."]
    #[serde(rename = "networkFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub network_friendly_name: Option<String>,
    #[doc = "The network subnet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
    #[doc = "The recovery point Id."]
    #[serde(rename = "recoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_id: Option<String>,
    #[doc = "The recovery point time."]
    #[serde(rename = "recoveryPointTime", with = "azure_core::date::rfc3339::option")]
    pub recovery_point_time: Option<time::OffsetDateTime>,
}
impl FailoverReplicationProtectedItemDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This class represents the group task details when parent child relationship exists in the drill down."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupTaskDetails {
    #[doc = "The type of task details."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
    #[doc = "The child tasks."]
    #[serde(rename = "childTasks", default, skip_serializing_if = "Vec::is_empty")]
    pub child_tasks: Vec<AsrTask>,
}
impl GroupTaskDetails {
    pub fn new(instance_type: String) -> Self {
        Self {
            instance_type,
            child_tasks: Vec::new(),
        }
    }
}
#[doc = "Health Error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthError {
    #[doc = "The inner health errors. HealthError having a list of HealthError as child errors is problematic. InnerHealthError is used because this will prevent an infinite loop of structures when Hydra tries to auto-generate the contract. We are exposing the related health errors as inner health errors and all API consumers can utilize this in the same fashion as Exception -&gt; InnerException."]
    #[serde(rename = "innerHealthErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub inner_health_errors: Vec<InnerHealthError>,
    #[doc = "Source of error."]
    #[serde(rename = "errorSource", default, skip_serializing_if = "Option::is_none")]
    pub error_source: Option<String>,
    #[doc = "Type of error."]
    #[serde(rename = "errorType", default, skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
    #[doc = "Level of error."]
    #[serde(rename = "errorLevel", default, skip_serializing_if = "Option::is_none")]
    pub error_level: Option<String>,
    #[doc = "Category of error."]
    #[serde(rename = "errorCategory", default, skip_serializing_if = "Option::is_none")]
    pub error_category: Option<String>,
    #[doc = "Error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "Summary message of the entity."]
    #[serde(rename = "summaryMessage", default, skip_serializing_if = "Option::is_none")]
    pub summary_message: Option<String>,
    #[doc = "Error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "Possible causes of error."]
    #[serde(rename = "possibleCauses", default, skip_serializing_if = "Option::is_none")]
    pub possible_causes: Option<String>,
    #[doc = "Recommended action to resolve error."]
    #[serde(rename = "recommendedAction", default, skip_serializing_if = "Option::is_none")]
    pub recommended_action: Option<String>,
    #[doc = "Error creation time (UTC)."]
    #[serde(rename = "creationTimeUtc", with = "azure_core::date::rfc3339::option")]
    pub creation_time_utc: Option<time::OffsetDateTime>,
    #[doc = "DRA error message."]
    #[serde(rename = "recoveryProviderErrorMessage", default, skip_serializing_if = "Option::is_none")]
    pub recovery_provider_error_message: Option<String>,
    #[doc = "ID of the entity."]
    #[serde(rename = "entityId", default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[doc = "The health error unique id."]
    #[serde(rename = "errorId", default, skip_serializing_if = "Option::is_none")]
    pub error_id: Option<String>,
    #[doc = "Value indicating whether the health error is customer resolvable."]
    #[serde(rename = "customerResolvability", default, skip_serializing_if = "Option::is_none")]
    pub customer_resolvability: Option<health_error::CustomerResolvability>,
}
impl HealthError {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod health_error {
    use super::*;
    #[doc = "Value indicating whether the health error is customer resolvable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CustomerResolvability")]
    pub enum CustomerResolvability {
        Allowed,
        NotAllowed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CustomerResolvability {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CustomerResolvability {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CustomerResolvability {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allowed => serializer.serialize_unit_variant("CustomerResolvability", 0u32, "Allowed"),
                Self::NotAllowed => serializer.serialize_unit_variant("CustomerResolvability", 1u32, "NotAllowed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "class to define the summary of the health error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthErrorSummary {
    #[doc = "The code of the health error."]
    #[serde(rename = "summaryCode", default, skip_serializing_if = "Option::is_none")]
    pub summary_code: Option<String>,
    #[doc = "The category of the health error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<health_error_summary::Category>,
    #[doc = "Severity of error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<health_error_summary::Severity>,
    #[doc = "The summary message of the health error."]
    #[serde(rename = "summaryMessage", default, skip_serializing_if = "Option::is_none")]
    pub summary_message: Option<String>,
    #[doc = "The type of affected ARM resource."]
    #[serde(rename = "affectedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub affected_resource_type: Option<String>,
    #[doc = "The sub type of any subcomponent within the ARM resource that this might be applicable. Value remains null if not applicable."]
    #[serde(rename = "affectedResourceSubtype", default, skip_serializing_if = "Option::is_none")]
    pub affected_resource_subtype: Option<String>,
    #[doc = "The list of affected resource correlation Ids. This can be used to uniquely identify the count of items affected by a specific category and severity as well as count of item affected by an specific issue."]
    #[serde(rename = "affectedResourceCorrelationIds", default, skip_serializing_if = "Vec::is_empty")]
    pub affected_resource_correlation_ids: Vec<String>,
}
impl HealthErrorSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod health_error_summary {
    use super::*;
    #[doc = "The category of the health error."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Category")]
    pub enum Category {
        None,
        Replication,
        TestFailover,
        Configuration,
        FabricInfrastructure,
        VersionExpiry,
        AgentAutoUpdateInfra,
        AgentAutoUpdateArtifactDeleted,
        AgentAutoUpdateRunAsAccount,
        AgentAutoUpdateRunAsAccountExpiry,
        AgentAutoUpdateRunAsAccountExpired,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Category {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Category {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Category {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Category", 0u32, "None"),
                Self::Replication => serializer.serialize_unit_variant("Category", 1u32, "Replication"),
                Self::TestFailover => serializer.serialize_unit_variant("Category", 2u32, "TestFailover"),
                Self::Configuration => serializer.serialize_unit_variant("Category", 3u32, "Configuration"),
                Self::FabricInfrastructure => serializer.serialize_unit_variant("Category", 4u32, "FabricInfrastructure"),
                Self::VersionExpiry => serializer.serialize_unit_variant("Category", 5u32, "VersionExpiry"),
                Self::AgentAutoUpdateInfra => serializer.serialize_unit_variant("Category", 6u32, "AgentAutoUpdateInfra"),
                Self::AgentAutoUpdateArtifactDeleted => {
                    serializer.serialize_unit_variant("Category", 7u32, "AgentAutoUpdateArtifactDeleted")
                }
                Self::AgentAutoUpdateRunAsAccount => serializer.serialize_unit_variant("Category", 8u32, "AgentAutoUpdateRunAsAccount"),
                Self::AgentAutoUpdateRunAsAccountExpiry => {
                    serializer.serialize_unit_variant("Category", 9u32, "AgentAutoUpdateRunAsAccountExpiry")
                }
                Self::AgentAutoUpdateRunAsAccountExpired => {
                    serializer.serialize_unit_variant("Category", 10u32, "AgentAutoUpdateRunAsAccountExpired")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Severity of error."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Severity")]
    pub enum Severity {
        #[serde(rename = "NONE")]
        None,
        Warning,
        Error,
        Info,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Severity {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Severity {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Severity {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Severity", 0u32, "NONE"),
                Self::Warning => serializer.serialize_unit_variant("Severity", 1u32, "Warning"),
                Self::Error => serializer.serialize_unit_variant("Severity", 2u32, "Error"),
                Self::Info => serializer.serialize_unit_variant("Severity", 3u32, "Info"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Hyper-V host details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HyperVHostDetails {
    #[doc = "The Hyper-V host Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The Hyper-V host name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The Mars agent version."]
    #[serde(rename = "marsAgentVersion", default, skip_serializing_if = "Option::is_none")]
    pub mars_agent_version: Option<String>,
}
impl HyperVHostDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model class for event details of a HyperVReplica E2E event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVReplica2012EventDetails {
    #[serde(flatten)]
    pub event_provider_specific_details: EventProviderSpecificDetails,
    #[doc = "The container friendly name."]
    #[serde(rename = "containerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[doc = "The fabric friendly name."]
    #[serde(rename = "fabricName", default, skip_serializing_if = "Option::is_none")]
    pub fabric_name: Option<String>,
    #[doc = "The remote container name."]
    #[serde(rename = "remoteContainerName", default, skip_serializing_if = "Option::is_none")]
    pub remote_container_name: Option<String>,
    #[doc = "The remote fabric name."]
    #[serde(rename = "remoteFabricName", default, skip_serializing_if = "Option::is_none")]
    pub remote_fabric_name: Option<String>,
}
impl HyperVReplica2012EventDetails {
    pub fn new(event_provider_specific_details: EventProviderSpecificDetails) -> Self {
        Self {
            event_provider_specific_details,
            container_name: None,
            fabric_name: None,
            remote_container_name: None,
            remote_fabric_name: None,
        }
    }
}
#[doc = "Model class for event details of a HyperVReplica blue E2E event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVReplica2012R2EventDetails {
    #[serde(flatten)]
    pub event_provider_specific_details: EventProviderSpecificDetails,
    #[doc = "The container friendly name."]
    #[serde(rename = "containerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[doc = "The fabric friendly name."]
    #[serde(rename = "fabricName", default, skip_serializing_if = "Option::is_none")]
    pub fabric_name: Option<String>,
    #[doc = "The remote container name."]
    #[serde(rename = "remoteContainerName", default, skip_serializing_if = "Option::is_none")]
    pub remote_container_name: Option<String>,
    #[doc = "The remote fabric name."]
    #[serde(rename = "remoteFabricName", default, skip_serializing_if = "Option::is_none")]
    pub remote_fabric_name: Option<String>,
}
impl HyperVReplica2012R2EventDetails {
    pub fn new(event_provider_specific_details: EventProviderSpecificDetails) -> Self {
        Self {
            event_provider_specific_details,
            container_name: None,
            fabric_name: None,
            remote_container_name: None,
            remote_fabric_name: None,
        }
    }
}
#[doc = "ApplyRecoveryPoint input specific to HyperVReplicaAzure provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVReplicaAzureApplyRecoveryPointInput {
    #[serde(flatten)]
    pub apply_recovery_point_provider_specific_input: ApplyRecoveryPointProviderSpecificInput,
    #[doc = "The primary kek certificate pfx."]
    #[serde(rename = "primaryKekCertificatePfx", default, skip_serializing_if = "Option::is_none")]
    pub primary_kek_certificate_pfx: Option<String>,
    #[doc = "The secondary kek certificate pfx."]
    #[serde(rename = "secondaryKekCertificatePfx", default, skip_serializing_if = "Option::is_none")]
    pub secondary_kek_certificate_pfx: Option<String>,
}
impl HyperVReplicaAzureApplyRecoveryPointInput {
    pub fn new(apply_recovery_point_provider_specific_input: ApplyRecoveryPointProviderSpecificInput) -> Self {
        Self {
            apply_recovery_point_provider_specific_input,
            primary_kek_certificate_pfx: None,
            secondary_kek_certificate_pfx: None,
        }
    }
}
#[doc = "Disk input details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HyperVReplicaAzureDiskInputDetails {
    #[doc = "The DiskId."]
    #[serde(rename = "diskId", default, skip_serializing_if = "Option::is_none")]
    pub disk_id: Option<String>,
    #[doc = "The LogStorageAccountId."]
    #[serde(rename = "logStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub log_storage_account_id: Option<String>,
    #[doc = "The DiskType."]
    #[serde(rename = "diskType", default, skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<hyper_v_replica_azure_disk_input_details::DiskType>,
    #[doc = "The DiskEncryptionSet ARM ID."]
    #[serde(rename = "diskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set_id: Option<String>,
}
impl HyperVReplicaAzureDiskInputDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod hyper_v_replica_azure_disk_input_details {
    use super::*;
    #[doc = "The DiskType."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DiskType")]
    pub enum DiskType {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(rename = "StandardSSD_LRS")]
        StandardSsdLrs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DiskType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DiskType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DiskType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::StandardLrs => serializer.serialize_unit_variant("DiskType", 0u32, "Standard_LRS"),
                Self::PremiumLrs => serializer.serialize_unit_variant("DiskType", 1u32, "Premium_LRS"),
                Self::StandardSsdLrs => serializer.serialize_unit_variant("DiskType", 2u32, "StandardSSD_LRS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "HyperVReplicaAzure specific enable protection input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVReplicaAzureEnableProtectionInput {
    #[serde(flatten)]
    pub enable_protection_provider_specific_input: EnableProtectionProviderSpecificInput,
    #[doc = "The Hyper-V host VM Id."]
    #[serde(rename = "hvHostVmId", default, skip_serializing_if = "Option::is_none")]
    pub hv_host_vm_id: Option<String>,
    #[doc = "The VM Name."]
    #[serde(rename = "vmName", default, skip_serializing_if = "Option::is_none")]
    pub vm_name: Option<String>,
    #[doc = "The OS type associated with VM."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "The OS disk VHD id associated with VM."]
    #[serde(rename = "vhdId", default, skip_serializing_if = "Option::is_none")]
    pub vhd_id: Option<String>,
    #[doc = "The storage account Id."]
    #[serde(rename = "targetStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub target_storage_account_id: Option<String>,
    #[doc = "The selected target Azure network Id."]
    #[serde(rename = "targetAzureNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub target_azure_network_id: Option<String>,
    #[doc = "The selected target Azure subnet Id."]
    #[serde(rename = "targetAzureSubnetId", default, skip_serializing_if = "Option::is_none")]
    pub target_azure_subnet_id: Option<String>,
    #[doc = "The selected option to enable RDP\\SSH on target vm after failover. String value of SrsDataContract.EnableRDPOnTargetOption enum."]
    #[serde(rename = "enableRdpOnTargetOption", default, skip_serializing_if = "Option::is_none")]
    pub enable_rdp_on_target_option: Option<String>,
    #[doc = "The target azure VM Name."]
    #[serde(rename = "targetAzureVmName", default, skip_serializing_if = "Option::is_none")]
    pub target_azure_vm_name: Option<String>,
    #[doc = "The storage account to be used for logging during replication."]
    #[serde(rename = "logStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub log_storage_account_id: Option<String>,
    #[doc = "The list of VHD Ids of disks to be protected."]
    #[serde(rename = "disksToInclude", default, skip_serializing_if = "Vec::is_empty")]
    pub disks_to_include: Vec<String>,
    #[doc = "The Id of the target resource group (for classic deployment) in which the failover VM is to be created."]
    #[serde(rename = "targetAzureV1ResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub target_azure_v1_resource_group_id: Option<String>,
    #[doc = "The Id of the target resource group (for resource manager deployment) in which the failover VM is to be created."]
    #[serde(rename = "targetAzureV2ResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub target_azure_v2_resource_group_id: Option<String>,
    #[doc = "A value indicating whether managed disks should be used during failover."]
    #[serde(rename = "useManagedDisks", default, skip_serializing_if = "Option::is_none")]
    pub use_managed_disks: Option<String>,
    #[doc = "The target availability set ARM Id for resource manager deployment."]
    #[serde(rename = "targetAvailabilitySetId", default, skip_serializing_if = "Option::is_none")]
    pub target_availability_set_id: Option<String>,
    #[doc = "The target availability zone."]
    #[serde(rename = "targetAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub target_availability_zone: Option<String>,
    #[doc = "License type."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<hyper_v_replica_azure_enable_protection_input::LicenseType>,
    #[doc = "The SQL Server license type."]
    #[serde(rename = "sqlServerLicenseType", default, skip_serializing_if = "Option::is_none")]
    pub sql_server_license_type: Option<hyper_v_replica_azure_enable_protection_input::SqlServerLicenseType>,
    #[doc = "The target VM size."]
    #[serde(rename = "targetVmSize", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_size: Option<String>,
    #[doc = "The proximity placement group ARM Id."]
    #[serde(rename = "targetProximityPlacementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub target_proximity_placement_group_id: Option<String>,
    #[doc = "A value indicating whether managed disks should be used during replication."]
    #[serde(rename = "useManagedDisksForReplication", default, skip_serializing_if = "Option::is_none")]
    pub use_managed_disks_for_replication: Option<String>,
    #[doc = "The DiskType."]
    #[serde(rename = "diskType", default, skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<hyper_v_replica_azure_enable_protection_input::DiskType>,
    #[doc = "The disks to include list for managed disks."]
    #[serde(rename = "disksToIncludeForManagedDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub disks_to_include_for_managed_disks: Vec<HyperVReplicaAzureDiskInputDetails>,
    #[doc = "The DiskEncryptionSet ARM Id."]
    #[serde(rename = "diskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set_id: Option<String>,
    #[doc = "The target VM tags."]
    #[serde(rename = "targetVmTags", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_tags: Option<serde_json::Value>,
    #[doc = "The tags for the seed managed disks."]
    #[serde(rename = "seedManagedDiskTags", default, skip_serializing_if = "Option::is_none")]
    pub seed_managed_disk_tags: Option<serde_json::Value>,
    #[doc = "The tags for the target managed disks."]
    #[serde(rename = "targetManagedDiskTags", default, skip_serializing_if = "Option::is_none")]
    pub target_managed_disk_tags: Option<serde_json::Value>,
    #[doc = "The tags for the target NICs."]
    #[serde(rename = "targetNicTags", default, skip_serializing_if = "Option::is_none")]
    pub target_nic_tags: Option<serde_json::Value>,
}
impl HyperVReplicaAzureEnableProtectionInput {
    pub fn new(enable_protection_provider_specific_input: EnableProtectionProviderSpecificInput) -> Self {
        Self {
            enable_protection_provider_specific_input,
            hv_host_vm_id: None,
            vm_name: None,
            os_type: None,
            vhd_id: None,
            target_storage_account_id: None,
            target_azure_network_id: None,
            target_azure_subnet_id: None,
            enable_rdp_on_target_option: None,
            target_azure_vm_name: None,
            log_storage_account_id: None,
            disks_to_include: Vec::new(),
            target_azure_v1_resource_group_id: None,
            target_azure_v2_resource_group_id: None,
            use_managed_disks: None,
            target_availability_set_id: None,
            target_availability_zone: None,
            license_type: None,
            sql_server_license_type: None,
            target_vm_size: None,
            target_proximity_placement_group_id: None,
            use_managed_disks_for_replication: None,
            disk_type: None,
            disks_to_include_for_managed_disks: Vec::new(),
            disk_encryption_set_id: None,
            target_vm_tags: None,
            seed_managed_disk_tags: None,
            target_managed_disk_tags: None,
            target_nic_tags: None,
        }
    }
}
pub mod hyper_v_replica_azure_enable_protection_input {
    use super::*;
    #[doc = "License type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseType")]
    pub enum LicenseType {
        NotSpecified,
        NoLicenseType,
        WindowsServer,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("LicenseType", 0u32, "NotSpecified"),
                Self::NoLicenseType => serializer.serialize_unit_variant("LicenseType", 1u32, "NoLicenseType"),
                Self::WindowsServer => serializer.serialize_unit_variant("LicenseType", 2u32, "WindowsServer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The SQL Server license type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SqlServerLicenseType")]
    pub enum SqlServerLicenseType {
        NotSpecified,
        NoLicenseType,
        #[serde(rename = "PAYG")]
        Payg,
        #[serde(rename = "AHUB")]
        Ahub,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SqlServerLicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SqlServerLicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SqlServerLicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("SqlServerLicenseType", 0u32, "NotSpecified"),
                Self::NoLicenseType => serializer.serialize_unit_variant("SqlServerLicenseType", 1u32, "NoLicenseType"),
                Self::Payg => serializer.serialize_unit_variant("SqlServerLicenseType", 2u32, "PAYG"),
                Self::Ahub => serializer.serialize_unit_variant("SqlServerLicenseType", 3u32, "AHUB"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The DiskType."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DiskType")]
    pub enum DiskType {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(rename = "StandardSSD_LRS")]
        StandardSsdLrs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DiskType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DiskType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DiskType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::StandardLrs => serializer.serialize_unit_variant("DiskType", 0u32, "Standard_LRS"),
                Self::PremiumLrs => serializer.serialize_unit_variant("DiskType", 1u32, "Premium_LRS"),
                Self::StandardSsdLrs => serializer.serialize_unit_variant("DiskType", 2u32, "StandardSSD_LRS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Model class for event details of a HyperVReplica E2A event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVReplicaAzureEventDetails {
    #[serde(flatten)]
    pub event_provider_specific_details: EventProviderSpecificDetails,
    #[doc = "The container friendly name."]
    #[serde(rename = "containerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[doc = "The fabric friendly name."]
    #[serde(rename = "fabricName", default, skip_serializing_if = "Option::is_none")]
    pub fabric_name: Option<String>,
    #[doc = "The remote container name."]
    #[serde(rename = "remoteContainerName", default, skip_serializing_if = "Option::is_none")]
    pub remote_container_name: Option<String>,
}
impl HyperVReplicaAzureEventDetails {
    pub fn new(event_provider_specific_details: EventProviderSpecificDetails) -> Self {
        Self {
            event_provider_specific_details,
            container_name: None,
            fabric_name: None,
            remote_container_name: None,
        }
    }
}
#[doc = "HyperVReplicaAzureFailback specific planned failover input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVReplicaAzureFailbackProviderInput {
    #[serde(flatten)]
    pub planned_failover_provider_specific_failover_input: PlannedFailoverProviderSpecificFailoverInput,
    #[doc = "Data sync option."]
    #[serde(rename = "dataSyncOption", default, skip_serializing_if = "Option::is_none")]
    pub data_sync_option: Option<String>,
    #[doc = "ALR options to create alternate recovery."]
    #[serde(rename = "recoveryVmCreationOption", default, skip_serializing_if = "Option::is_none")]
    pub recovery_vm_creation_option: Option<String>,
    #[doc = "Provider Id for alternate location."]
    #[serde(rename = "providerIdForAlternateRecovery", default, skip_serializing_if = "Option::is_none")]
    pub provider_id_for_alternate_recovery: Option<String>,
}
impl HyperVReplicaAzureFailbackProviderInput {
    pub fn new(planned_failover_provider_specific_failover_input: PlannedFailoverProviderSpecificFailoverInput) -> Self {
        Self {
            planned_failover_provider_specific_failover_input,
            data_sync_option: None,
            recovery_vm_creation_option: None,
            provider_id_for_alternate_recovery: None,
        }
    }
}
#[doc = "Hyper-V Managed disk details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HyperVReplicaAzureManagedDiskDetails {
    #[doc = "The disk Id."]
    #[serde(rename = "diskId", default, skip_serializing_if = "Option::is_none")]
    pub disk_id: Option<String>,
    #[doc = "Seed managed disk Id."]
    #[serde(rename = "seedManagedDiskId", default, skip_serializing_if = "Option::is_none")]
    pub seed_managed_disk_id: Option<String>,
    #[doc = "The replica disk type."]
    #[serde(rename = "replicaDiskType", default, skip_serializing_if = "Option::is_none")]
    pub replica_disk_type: Option<String>,
    #[doc = "The disk encryption set ARM Id."]
    #[serde(rename = "diskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set_id: Option<String>,
}
impl HyperVReplicaAzureManagedDiskDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "HyperVReplicaAzure specific planned failover input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVReplicaAzurePlannedFailoverProviderInput {
    #[serde(flatten)]
    pub planned_failover_provider_specific_failover_input: PlannedFailoverProviderSpecificFailoverInput,
    #[doc = "Primary kek certificate pfx."]
    #[serde(rename = "primaryKekCertificatePfx", default, skip_serializing_if = "Option::is_none")]
    pub primary_kek_certificate_pfx: Option<String>,
    #[doc = "Secondary kek certificate pfx."]
    #[serde(rename = "secondaryKekCertificatePfx", default, skip_serializing_if = "Option::is_none")]
    pub secondary_kek_certificate_pfx: Option<String>,
    #[doc = "The recovery point id to be passed to failover to a particular recovery point. In case of latest recovery point, null should be passed."]
    #[serde(rename = "recoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_id: Option<String>,
}
impl HyperVReplicaAzurePlannedFailoverProviderInput {
    pub fn new(planned_failover_provider_specific_failover_input: PlannedFailoverProviderSpecificFailoverInput) -> Self {
        Self {
            planned_failover_provider_specific_failover_input,
            primary_kek_certificate_pfx: None,
            secondary_kek_certificate_pfx: None,
            recovery_point_id: None,
        }
    }
}
#[doc = "Hyper-V Replica Azure specific protection profile details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVReplicaAzurePolicyDetails {
    #[serde(flatten)]
    pub policy_provider_specific_details: PolicyProviderSpecificDetails,
    #[doc = "The duration (in hours) to which point the recovery history needs to be maintained."]
    #[serde(rename = "recoveryPointHistoryDurationInHours", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_history_duration_in_hours: Option<i32>,
    #[doc = "The interval (in hours) at which Hyper-V Replica should create an application consistent snapshot within the VM."]
    #[serde(
        rename = "applicationConsistentSnapshotFrequencyInHours",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub application_consistent_snapshot_frequency_in_hours: Option<i32>,
    #[doc = "The replication interval."]
    #[serde(rename = "replicationInterval", default, skip_serializing_if = "Option::is_none")]
    pub replication_interval: Option<i32>,
    #[doc = "The scheduled start time for the initial replication. If this parameter is Null, the initial replication starts immediately."]
    #[serde(rename = "onlineReplicationStartTime", default, skip_serializing_if = "Option::is_none")]
    pub online_replication_start_time: Option<String>,
    #[doc = "A value indicating whether encryption is enabled for virtual machines in this cloud."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<String>,
    #[doc = "The active storage account Id."]
    #[serde(rename = "activeStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub active_storage_account_id: Option<String>,
}
impl HyperVReplicaAzurePolicyDetails {
    pub fn new(policy_provider_specific_details: PolicyProviderSpecificDetails) -> Self {
        Self {
            policy_provider_specific_details,
            recovery_point_history_duration_in_hours: None,
            application_consistent_snapshot_frequency_in_hours: None,
            replication_interval: None,
            online_replication_start_time: None,
            encryption: None,
            active_storage_account_id: None,
        }
    }
}
#[doc = "Hyper-V Replica Azure specific input for creating a protection profile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVReplicaAzurePolicyInput {
    #[serde(flatten)]
    pub policy_provider_specific_input: PolicyProviderSpecificInput,
    #[doc = "The duration (in hours) to which point the recovery history needs to be maintained."]
    #[serde(rename = "recoveryPointHistoryDuration", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_history_duration: Option<i32>,
    #[doc = "The interval (in hours) at which Hyper-V Replica should create an application consistent snapshot within the VM."]
    #[serde(
        rename = "applicationConsistentSnapshotFrequencyInHours",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub application_consistent_snapshot_frequency_in_hours: Option<i32>,
    #[doc = "The replication interval."]
    #[serde(rename = "replicationInterval", default, skip_serializing_if = "Option::is_none")]
    pub replication_interval: Option<i32>,
    #[doc = "The scheduled start time for the initial replication. If this parameter is Null, the initial replication starts immediately."]
    #[serde(rename = "onlineReplicationStartTime", default, skip_serializing_if = "Option::is_none")]
    pub online_replication_start_time: Option<String>,
    #[doc = "The list of storage accounts to which the VMs in the primary cloud can replicate to."]
    #[serde(rename = "storageAccounts", default, skip_serializing_if = "Vec::is_empty")]
    pub storage_accounts: Vec<String>,
}
impl HyperVReplicaAzurePolicyInput {
    pub fn new(policy_provider_specific_input: PolicyProviderSpecificInput) -> Self {
        Self {
            policy_provider_specific_input,
            recovery_point_history_duration: None,
            application_consistent_snapshot_frequency_in_hours: None,
            replication_interval: None,
            online_replication_start_time: None,
            storage_accounts: Vec::new(),
        }
    }
}
#[doc = "Hyper V Replica Azure provider specific settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVReplicaAzureReplicationDetails {
    #[serde(flatten)]
    pub replication_provider_specific_settings: ReplicationProviderSpecificSettings,
    #[doc = "Azure VM Disk details."]
    #[serde(rename = "azureVmDiskDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_vm_disk_details: Vec<AzureVmDiskDetails>,
    #[doc = "Recovery Azure given name."]
    #[serde(rename = "recoveryAzureVmName", default, skip_serializing_if = "Option::is_none")]
    pub recovery_azure_vm_name: Option<String>,
    #[doc = "The Recovery Azure VM size."]
    #[serde(rename = "recoveryAzureVMSize", default, skip_serializing_if = "Option::is_none")]
    pub recovery_azure_vm_size: Option<String>,
    #[doc = "The recovery Azure storage account."]
    #[serde(rename = "recoveryAzureStorageAccount", default, skip_serializing_if = "Option::is_none")]
    pub recovery_azure_storage_account: Option<String>,
    #[doc = "The ARM id of the log storage account used for replication. This will be set to null if no log storage account was provided during enable protection."]
    #[serde(rename = "recoveryAzureLogStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_azure_log_storage_account_id: Option<String>,
    #[doc = "The Last replication time."]
    #[serde(rename = "lastReplicatedTime", with = "azure_core::date::rfc3339::option")]
    pub last_replicated_time: Option<time::OffsetDateTime>,
    #[doc = "Last RPO value."]
    #[serde(rename = "rpoInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub rpo_in_seconds: Option<i64>,
    #[doc = "The last RPO calculated time."]
    #[serde(rename = "lastRpoCalculatedTime", with = "azure_core::date::rfc3339::option")]
    pub last_rpo_calculated_time: Option<time::OffsetDateTime>,
    #[doc = "The virtual machine Id."]
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[doc = "The protection state for the vm."]
    #[serde(rename = "vmProtectionState", default, skip_serializing_if = "Option::is_none")]
    pub vm_protection_state: Option<String>,
    #[doc = "The protection state description for the vm."]
    #[serde(rename = "vmProtectionStateDescription", default, skip_serializing_if = "Option::is_none")]
    pub vm_protection_state_description: Option<String>,
    #[doc = "Initial replication details."]
    #[serde(rename = "initialReplicationDetails", default, skip_serializing_if = "Option::is_none")]
    pub initial_replication_details: Option<InitialReplicationDetails>,
    #[doc = "The PE Network details."]
    #[serde(rename = "vmNics", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_nics: Vec<VmNicDetails>,
    #[doc = "The selected recovery azure network Id."]
    #[serde(rename = "selectedRecoveryAzureNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub selected_recovery_azure_network_id: Option<String>,
    #[doc = "The selected source nic Id which will be used as the primary nic during failover."]
    #[serde(rename = "selectedSourceNicId", default, skip_serializing_if = "Option::is_none")]
    pub selected_source_nic_id: Option<String>,
    #[doc = "The encryption info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<String>,
    #[doc = "Disk Details."]
    #[serde(rename = "oSDetails", default, skip_serializing_if = "Option::is_none")]
    pub o_s_details: Option<OsDetails>,
    #[doc = "The RAM size of the VM on the primary side."]
    #[serde(rename = "sourceVmRamSizeInMB", default, skip_serializing_if = "Option::is_none")]
    pub source_vm_ram_size_in_mb: Option<i32>,
    #[doc = "The CPU count of the VM on the primary side."]
    #[serde(rename = "sourceVmCpuCount", default, skip_serializing_if = "Option::is_none")]
    pub source_vm_cpu_count: Option<i32>,
    #[doc = "The selected option to enable RDP\\SSH on target vm after failover. String value of SrsDataContract.EnableRDPOnTargetOption enum."]
    #[serde(rename = "enableRdpOnTargetOption", default, skip_serializing_if = "Option::is_none")]
    pub enable_rdp_on_target_option: Option<String>,
    #[doc = "The target resource group Id."]
    #[serde(rename = "recoveryAzureResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_azure_resource_group_id: Option<String>,
    #[doc = "The recovery availability set Id."]
    #[serde(rename = "recoveryAvailabilitySetId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_availability_set_id: Option<String>,
    #[doc = "The target availability zone."]
    #[serde(rename = "targetAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub target_availability_zone: Option<String>,
    #[doc = "The target proximity placement group Id."]
    #[serde(rename = "targetProximityPlacementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub target_proximity_placement_group_id: Option<String>,
    #[doc = "A value indicating whether managed disks should be used during failover."]
    #[serde(rename = "useManagedDisks", default, skip_serializing_if = "Option::is_none")]
    pub use_managed_disks: Option<String>,
    #[doc = "License Type of the VM to be used."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    #[doc = "The SQL Server license type."]
    #[serde(rename = "sqlServerLicenseType", default, skip_serializing_if = "Option::is_none")]
    pub sql_server_license_type: Option<String>,
    #[doc = "The last recovery point received time."]
    #[serde(rename = "lastRecoveryPointReceived", with = "azure_core::date::rfc3339::option")]
    pub last_recovery_point_received: Option<time::OffsetDateTime>,
    #[doc = "The target VM tags."]
    #[serde(rename = "targetVmTags", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_tags: Option<serde_json::Value>,
    #[doc = "The tags for the seed managed disks."]
    #[serde(rename = "seedManagedDiskTags", default, skip_serializing_if = "Option::is_none")]
    pub seed_managed_disk_tags: Option<serde_json::Value>,
    #[doc = "The tags for the target managed disks."]
    #[serde(rename = "targetManagedDiskTags", default, skip_serializing_if = "Option::is_none")]
    pub target_managed_disk_tags: Option<serde_json::Value>,
    #[doc = "The tags for the target NICs."]
    #[serde(rename = "targetNicTags", default, skip_serializing_if = "Option::is_none")]
    pub target_nic_tags: Option<serde_json::Value>,
    #[doc = "The list of protected managed disks."]
    #[serde(rename = "protectedManagedDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub protected_managed_disks: Vec<HyperVReplicaAzureManagedDiskDetails>,
}
impl HyperVReplicaAzureReplicationDetails {
    pub fn new(replication_provider_specific_settings: ReplicationProviderSpecificSettings) -> Self {
        Self {
            replication_provider_specific_settings,
            azure_vm_disk_details: Vec::new(),
            recovery_azure_vm_name: None,
            recovery_azure_vm_size: None,
            recovery_azure_storage_account: None,
            recovery_azure_log_storage_account_id: None,
            last_replicated_time: None,
            rpo_in_seconds: None,
            last_rpo_calculated_time: None,
            vm_id: None,
            vm_protection_state: None,
            vm_protection_state_description: None,
            initial_replication_details: None,
            vm_nics: Vec::new(),
            selected_recovery_azure_network_id: None,
            selected_source_nic_id: None,
            encryption: None,
            o_s_details: None,
            source_vm_ram_size_in_mb: None,
            source_vm_cpu_count: None,
            enable_rdp_on_target_option: None,
            recovery_azure_resource_group_id: None,
            recovery_availability_set_id: None,
            target_availability_zone: None,
            target_proximity_placement_group_id: None,
            use_managed_disks: None,
            license_type: None,
            sql_server_license_type: None,
            last_recovery_point_received: None,
            target_vm_tags: None,
            seed_managed_disk_tags: None,
            target_managed_disk_tags: None,
            target_nic_tags: None,
            protected_managed_disks: Vec::new(),
        }
    }
}
#[doc = "Azure specific reprotect input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVReplicaAzureReprotectInput {
    #[serde(flatten)]
    pub reverse_replication_provider_specific_input: ReverseReplicationProviderSpecificInput,
    #[doc = "The Hyper-V host Vm Id."]
    #[serde(rename = "hvHostVmId", default, skip_serializing_if = "Option::is_none")]
    pub hv_host_vm_id: Option<String>,
    #[doc = "The Vm Name."]
    #[serde(rename = "vmName", default, skip_serializing_if = "Option::is_none")]
    pub vm_name: Option<String>,
    #[doc = "The OS type associated with vm."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "The OS disk VHD id associated with vm."]
    #[serde(rename = "vHDId", default, skip_serializing_if = "Option::is_none")]
    pub v_hd_id: Option<String>,
    #[doc = "The storage account name."]
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[doc = "The storage account to be used for logging during replication."]
    #[serde(rename = "logStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub log_storage_account_id: Option<String>,
}
impl HyperVReplicaAzureReprotectInput {
    pub fn new(reverse_replication_provider_specific_input: ReverseReplicationProviderSpecificInput) -> Self {
        Self {
            reverse_replication_provider_specific_input,
            hv_host_vm_id: None,
            vm_name: None,
            os_type: None,
            v_hd_id: None,
            storage_account_id: None,
            log_storage_account_id: None,
        }
    }
}
#[doc = "HvrA provider specific input for test failover."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVReplicaAzureTestFailoverInput {
    #[serde(flatten)]
    pub test_failover_provider_specific_input: TestFailoverProviderSpecificInput,
    #[doc = "Primary kek certificate pfx."]
    #[serde(rename = "primaryKekCertificatePfx", default, skip_serializing_if = "Option::is_none")]
    pub primary_kek_certificate_pfx: Option<String>,
    #[doc = "Secondary kek certificate pfx."]
    #[serde(rename = "secondaryKekCertificatePfx", default, skip_serializing_if = "Option::is_none")]
    pub secondary_kek_certificate_pfx: Option<String>,
    #[doc = "The recovery point id to be passed to test failover to a particular recovery point. In case of latest recovery point, null should be passed."]
    #[serde(rename = "recoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_id: Option<String>,
}
impl HyperVReplicaAzureTestFailoverInput {
    pub fn new(test_failover_provider_specific_input: TestFailoverProviderSpecificInput) -> Self {
        Self {
            test_failover_provider_specific_input,
            primary_kek_certificate_pfx: None,
            secondary_kek_certificate_pfx: None,
            recovery_point_id: None,
        }
    }
}
#[doc = "HvrA provider specific input for unplanned failover."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVReplicaAzureUnplannedFailoverInput {
    #[serde(flatten)]
    pub unplanned_failover_provider_specific_input: UnplannedFailoverProviderSpecificInput,
    #[doc = "Primary kek certificate pfx."]
    #[serde(rename = "primaryKekCertificatePfx", default, skip_serializing_if = "Option::is_none")]
    pub primary_kek_certificate_pfx: Option<String>,
    #[doc = "Secondary kek certificate pfx."]
    #[serde(rename = "secondaryKekCertificatePfx", default, skip_serializing_if = "Option::is_none")]
    pub secondary_kek_certificate_pfx: Option<String>,
    #[doc = "The recovery point id to be passed to failover to a particular recovery point. In case of latest recovery point, null should be passed."]
    #[serde(rename = "recoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_id: Option<String>,
}
impl HyperVReplicaAzureUnplannedFailoverInput {
    pub fn new(unplanned_failover_provider_specific_input: UnplannedFailoverProviderSpecificInput) -> Self {
        Self {
            unplanned_failover_provider_specific_input,
            primary_kek_certificate_pfx: None,
            secondary_kek_certificate_pfx: None,
            recovery_point_id: None,
        }
    }
}
#[doc = "HyperV replica Azure input to update replication protected item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVReplicaAzureUpdateReplicationProtectedItemInput {
    #[serde(flatten)]
    pub update_replication_protected_item_provider_input: UpdateReplicationProtectedItemProviderInput,
    #[doc = "The recovery Azure resource group Id for classic deployment."]
    #[serde(rename = "recoveryAzureV1ResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_azure_v1_resource_group_id: Option<String>,
    #[doc = "The recovery Azure resource group Id for resource manager deployment."]
    #[serde(rename = "recoveryAzureV2ResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_azure_v2_resource_group_id: Option<String>,
    #[doc = "A value indicating whether managed disks should be used during failover."]
    #[serde(rename = "useManagedDisks", default, skip_serializing_if = "Option::is_none")]
    pub use_managed_disks: Option<String>,
    #[doc = "The dictionary of disk resource Id to disk encryption set ARM Id."]
    #[serde(rename = "diskIdToDiskEncryptionMap", default, skip_serializing_if = "Option::is_none")]
    pub disk_id_to_disk_encryption_map: Option<serde_json::Value>,
    #[doc = "The target proximity placement group Id."]
    #[serde(rename = "targetProximityPlacementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub target_proximity_placement_group_id: Option<String>,
    #[doc = "The target availability zone."]
    #[serde(rename = "targetAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub target_availability_zone: Option<String>,
    #[doc = "The target VM tags."]
    #[serde(rename = "targetVmTags", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_tags: Option<serde_json::Value>,
    #[doc = "The tags for the target managed disks."]
    #[serde(rename = "targetManagedDiskTags", default, skip_serializing_if = "Option::is_none")]
    pub target_managed_disk_tags: Option<serde_json::Value>,
    #[doc = "The tags for the target NICs."]
    #[serde(rename = "targetNicTags", default, skip_serializing_if = "Option::is_none")]
    pub target_nic_tags: Option<serde_json::Value>,
    #[doc = "The SQL Server license type."]
    #[serde(rename = "sqlServerLicenseType", default, skip_serializing_if = "Option::is_none")]
    pub sql_server_license_type: Option<hyper_v_replica_azure_update_replication_protected_item_input::SqlServerLicenseType>,
    #[doc = "The list of disk update properties."]
    #[serde(rename = "vmDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_disks: Vec<UpdateDiskInput>,
}
impl HyperVReplicaAzureUpdateReplicationProtectedItemInput {
    pub fn new(update_replication_protected_item_provider_input: UpdateReplicationProtectedItemProviderInput) -> Self {
        Self {
            update_replication_protected_item_provider_input,
            recovery_azure_v1_resource_group_id: None,
            recovery_azure_v2_resource_group_id: None,
            use_managed_disks: None,
            disk_id_to_disk_encryption_map: None,
            target_proximity_placement_group_id: None,
            target_availability_zone: None,
            target_vm_tags: None,
            target_managed_disk_tags: None,
            target_nic_tags: None,
            sql_server_license_type: None,
            vm_disks: Vec::new(),
        }
    }
}
pub mod hyper_v_replica_azure_update_replication_protected_item_input {
    use super::*;
    #[doc = "The SQL Server license type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SqlServerLicenseType")]
    pub enum SqlServerLicenseType {
        NotSpecified,
        NoLicenseType,
        #[serde(rename = "PAYG")]
        Payg,
        #[serde(rename = "AHUB")]
        Ahub,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SqlServerLicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SqlServerLicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SqlServerLicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("SqlServerLicenseType", 0u32, "NotSpecified"),
                Self::NoLicenseType => serializer.serialize_unit_variant("SqlServerLicenseType", 1u32, "NoLicenseType"),
                Self::Payg => serializer.serialize_unit_variant("SqlServerLicenseType", 2u32, "PAYG"),
                Self::Ahub => serializer.serialize_unit_variant("SqlServerLicenseType", 3u32, "AHUB"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Abstract model class for event details of a HyperVReplica E2E event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVReplicaBaseEventDetails {
    #[serde(flatten)]
    pub event_provider_specific_details: EventProviderSpecificDetails,
    #[doc = "The container friendly name."]
    #[serde(rename = "containerName", default, skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[doc = "The fabric friendly name."]
    #[serde(rename = "fabricName", default, skip_serializing_if = "Option::is_none")]
    pub fabric_name: Option<String>,
    #[doc = "The remote container name."]
    #[serde(rename = "remoteContainerName", default, skip_serializing_if = "Option::is_none")]
    pub remote_container_name: Option<String>,
    #[doc = "The remote fabric name."]
    #[serde(rename = "remoteFabricName", default, skip_serializing_if = "Option::is_none")]
    pub remote_fabric_name: Option<String>,
}
impl HyperVReplicaBaseEventDetails {
    pub fn new(event_provider_specific_details: EventProviderSpecificDetails) -> Self {
        Self {
            event_provider_specific_details,
            container_name: None,
            fabric_name: None,
            remote_container_name: None,
            remote_fabric_name: None,
        }
    }
}
#[doc = "Base class for HyperVReplica policy details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVReplicaBasePolicyDetails {
    #[serde(flatten)]
    pub policy_provider_specific_details: PolicyProviderSpecificDetails,
    #[doc = "A value indicating the number of recovery points."]
    #[serde(rename = "recoveryPoints", default, skip_serializing_if = "Option::is_none")]
    pub recovery_points: Option<i32>,
    #[doc = "A value indicating the application consistent frequency."]
    #[serde(
        rename = "applicationConsistentSnapshotFrequencyInHours",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub application_consistent_snapshot_frequency_in_hours: Option<i32>,
    #[doc = "A value indicating whether compression has to be enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression: Option<String>,
    #[doc = "A value indicating whether IR is online."]
    #[serde(rename = "initialReplicationMethod", default, skip_serializing_if = "Option::is_none")]
    pub initial_replication_method: Option<String>,
    #[doc = "A value indicating the online IR start time."]
    #[serde(rename = "onlineReplicationStartTime", default, skip_serializing_if = "Option::is_none")]
    pub online_replication_start_time: Option<String>,
    #[doc = "A value indicating the offline IR import path."]
    #[serde(rename = "offlineReplicationImportPath", default, skip_serializing_if = "Option::is_none")]
    pub offline_replication_import_path: Option<String>,
    #[doc = "A value indicating the offline IR export path."]
    #[serde(rename = "offlineReplicationExportPath", default, skip_serializing_if = "Option::is_none")]
    pub offline_replication_export_path: Option<String>,
    #[doc = "A value indicating the recovery HTTPS port."]
    #[serde(rename = "replicationPort", default, skip_serializing_if = "Option::is_none")]
    pub replication_port: Option<i32>,
    #[doc = "A value indicating the authentication type."]
    #[serde(rename = "allowedAuthenticationType", default, skip_serializing_if = "Option::is_none")]
    pub allowed_authentication_type: Option<i32>,
    #[doc = "A value indicating whether the VM has to be auto deleted. Supported Values: String.Empty, None, OnRecoveryCloud."]
    #[serde(rename = "replicaDeletionOption", default, skip_serializing_if = "Option::is_none")]
    pub replica_deletion_option: Option<String>,
}
impl HyperVReplicaBasePolicyDetails {
    pub fn new(policy_provider_specific_details: PolicyProviderSpecificDetails) -> Self {
        Self {
            policy_provider_specific_details,
            recovery_points: None,
            application_consistent_snapshot_frequency_in_hours: None,
            compression: None,
            initial_replication_method: None,
            online_replication_start_time: None,
            offline_replication_import_path: None,
            offline_replication_export_path: None,
            replication_port: None,
            allowed_authentication_type: None,
            replica_deletion_option: None,
        }
    }
}
#[doc = "Hyper V replica provider specific settings base class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVReplicaBaseReplicationDetails {
    #[serde(flatten)]
    pub replication_provider_specific_settings: ReplicationProviderSpecificSettings,
    #[doc = "The Last replication time."]
    #[serde(rename = "lastReplicatedTime", with = "azure_core::date::rfc3339::option")]
    pub last_replicated_time: Option<time::OffsetDateTime>,
    #[doc = "The PE Network details."]
    #[serde(rename = "vmNics", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_nics: Vec<VmNicDetails>,
    #[doc = "The virtual machine Id."]
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[doc = "The protection state for the vm."]
    #[serde(rename = "vmProtectionState", default, skip_serializing_if = "Option::is_none")]
    pub vm_protection_state: Option<String>,
    #[doc = "The protection state description for the vm."]
    #[serde(rename = "vmProtectionStateDescription", default, skip_serializing_if = "Option::is_none")]
    pub vm_protection_state_description: Option<String>,
    #[doc = "Initial replication details."]
    #[serde(rename = "initialReplicationDetails", default, skip_serializing_if = "Option::is_none")]
    pub initial_replication_details: Option<InitialReplicationDetails>,
    #[doc = "VM disk details."]
    #[serde(rename = "vMDiskDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub v_m_disk_details: Vec<DiskDetails>,
}
impl HyperVReplicaBaseReplicationDetails {
    pub fn new(replication_provider_specific_settings: ReplicationProviderSpecificSettings) -> Self {
        Self {
            replication_provider_specific_settings,
            last_replicated_time: None,
            vm_nics: Vec::new(),
            vm_id: None,
            vm_protection_state: None,
            vm_protection_state_description: None,
            initial_replication_details: None,
            v_m_disk_details: Vec::new(),
        }
    }
}
#[doc = "Hyper-V Replica Blue specific protection profile details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVReplicaBluePolicyDetails {
    #[serde(flatten)]
    pub policy_provider_specific_details: PolicyProviderSpecificDetails,
    #[doc = "A value indicating the replication interval."]
    #[serde(rename = "replicationFrequencyInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub replication_frequency_in_seconds: Option<i32>,
    #[doc = "A value indicating the number of recovery points."]
    #[serde(rename = "recoveryPoints", default, skip_serializing_if = "Option::is_none")]
    pub recovery_points: Option<i32>,
    #[doc = "A value indicating the application consistent frequency."]
    #[serde(
        rename = "applicationConsistentSnapshotFrequencyInHours",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub application_consistent_snapshot_frequency_in_hours: Option<i32>,
    #[doc = "A value indicating whether compression has to be enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression: Option<String>,
    #[doc = "A value indicating whether IR is online."]
    #[serde(rename = "initialReplicationMethod", default, skip_serializing_if = "Option::is_none")]
    pub initial_replication_method: Option<String>,
    #[doc = "A value indicating the online IR start time."]
    #[serde(rename = "onlineReplicationStartTime", default, skip_serializing_if = "Option::is_none")]
    pub online_replication_start_time: Option<String>,
    #[doc = "A value indicating the offline IR import path."]
    #[serde(rename = "offlineReplicationImportPath", default, skip_serializing_if = "Option::is_none")]
    pub offline_replication_import_path: Option<String>,
    #[doc = "A value indicating the offline IR export path."]
    #[serde(rename = "offlineReplicationExportPath", default, skip_serializing_if = "Option::is_none")]
    pub offline_replication_export_path: Option<String>,
    #[doc = "A value indicating the recovery HTTPS port."]
    #[serde(rename = "replicationPort", default, skip_serializing_if = "Option::is_none")]
    pub replication_port: Option<i32>,
    #[doc = "A value indicating the authentication type."]
    #[serde(rename = "allowedAuthenticationType", default, skip_serializing_if = "Option::is_none")]
    pub allowed_authentication_type: Option<i32>,
    #[doc = "A value indicating whether the VM has to be auto deleted. Supported Values: String.Empty, None, OnRecoveryCloud"]
    #[serde(rename = "replicaDeletionOption", default, skip_serializing_if = "Option::is_none")]
    pub replica_deletion_option: Option<String>,
}
impl HyperVReplicaBluePolicyDetails {
    pub fn new(policy_provider_specific_details: PolicyProviderSpecificDetails) -> Self {
        Self {
            policy_provider_specific_details,
            replication_frequency_in_seconds: None,
            recovery_points: None,
            application_consistent_snapshot_frequency_in_hours: None,
            compression: None,
            initial_replication_method: None,
            online_replication_start_time: None,
            offline_replication_import_path: None,
            offline_replication_export_path: None,
            replication_port: None,
            allowed_authentication_type: None,
            replica_deletion_option: None,
        }
    }
}
#[doc = "HyperV Replica Blue policy input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVReplicaBluePolicyInput {
    #[serde(flatten)]
    pub hyper_v_replica_policy_input: HyperVReplicaPolicyInput,
    #[doc = "A value indicating the replication interval."]
    #[serde(rename = "replicationFrequencyInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub replication_frequency_in_seconds: Option<i32>,
}
impl HyperVReplicaBluePolicyInput {
    pub fn new(hyper_v_replica_policy_input: HyperVReplicaPolicyInput) -> Self {
        Self {
            hyper_v_replica_policy_input,
            replication_frequency_in_seconds: None,
        }
    }
}
#[doc = "HyperV replica 2012 R2 (Blue) replication details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVReplicaBlueReplicationDetails {
    #[serde(flatten)]
    pub replication_provider_specific_settings: ReplicationProviderSpecificSettings,
    #[doc = "The Last replication time."]
    #[serde(rename = "lastReplicatedTime", with = "azure_core::date::rfc3339::option")]
    pub last_replicated_time: Option<time::OffsetDateTime>,
    #[doc = "The PE Network details."]
    #[serde(rename = "vmNics", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_nics: Vec<VmNicDetails>,
    #[doc = "The virtual machine Id."]
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[doc = "The protection state for the vm."]
    #[serde(rename = "vmProtectionState", default, skip_serializing_if = "Option::is_none")]
    pub vm_protection_state: Option<String>,
    #[doc = "The protection state description for the vm."]
    #[serde(rename = "vmProtectionStateDescription", default, skip_serializing_if = "Option::is_none")]
    pub vm_protection_state_description: Option<String>,
    #[doc = "Initial replication details."]
    #[serde(rename = "initialReplicationDetails", default, skip_serializing_if = "Option::is_none")]
    pub initial_replication_details: Option<InitialReplicationDetails>,
    #[doc = "VM disk details."]
    #[serde(rename = "vMDiskDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub v_m_disk_details: Vec<DiskDetails>,
}
impl HyperVReplicaBlueReplicationDetails {
    pub fn new(replication_provider_specific_settings: ReplicationProviderSpecificSettings) -> Self {
        Self {
            replication_provider_specific_settings,
            last_replicated_time: None,
            vm_nics: Vec::new(),
            vm_id: None,
            vm_protection_state: None,
            vm_protection_state_description: None,
            initial_replication_details: None,
            v_m_disk_details: Vec::new(),
        }
    }
}
#[doc = "Hyper-V Replica Blue specific protection profile details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVReplicaPolicyDetails {
    #[serde(flatten)]
    pub policy_provider_specific_details: PolicyProviderSpecificDetails,
    #[doc = "A value indicating the number of recovery points."]
    #[serde(rename = "recoveryPoints", default, skip_serializing_if = "Option::is_none")]
    pub recovery_points: Option<i32>,
    #[doc = "A value indicating the application consistent frequency."]
    #[serde(
        rename = "applicationConsistentSnapshotFrequencyInHours",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub application_consistent_snapshot_frequency_in_hours: Option<i32>,
    #[doc = "A value indicating whether compression has to be enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression: Option<String>,
    #[doc = "A value indicating whether IR is online."]
    #[serde(rename = "initialReplicationMethod", default, skip_serializing_if = "Option::is_none")]
    pub initial_replication_method: Option<String>,
    #[doc = "A value indicating the online IR start time."]
    #[serde(rename = "onlineReplicationStartTime", default, skip_serializing_if = "Option::is_none")]
    pub online_replication_start_time: Option<String>,
    #[doc = "A value indicating the offline IR import path."]
    #[serde(rename = "offlineReplicationImportPath", default, skip_serializing_if = "Option::is_none")]
    pub offline_replication_import_path: Option<String>,
    #[doc = "A value indicating the offline IR export path."]
    #[serde(rename = "offlineReplicationExportPath", default, skip_serializing_if = "Option::is_none")]
    pub offline_replication_export_path: Option<String>,
    #[doc = "A value indicating the recovery HTTPS port."]
    #[serde(rename = "replicationPort", default, skip_serializing_if = "Option::is_none")]
    pub replication_port: Option<i32>,
    #[doc = "A value indicating the authentication type."]
    #[serde(rename = "allowedAuthenticationType", default, skip_serializing_if = "Option::is_none")]
    pub allowed_authentication_type: Option<i32>,
    #[doc = "A value indicating whether the VM has to be auto deleted. Supported Values: String.Empty, None, OnRecoveryCloud"]
    #[serde(rename = "replicaDeletionOption", default, skip_serializing_if = "Option::is_none")]
    pub replica_deletion_option: Option<String>,
}
impl HyperVReplicaPolicyDetails {
    pub fn new(policy_provider_specific_details: PolicyProviderSpecificDetails) -> Self {
        Self {
            policy_provider_specific_details,
            recovery_points: None,
            application_consistent_snapshot_frequency_in_hours: None,
            compression: None,
            initial_replication_method: None,
            online_replication_start_time: None,
            offline_replication_import_path: None,
            offline_replication_export_path: None,
            replication_port: None,
            allowed_authentication_type: None,
            replica_deletion_option: None,
        }
    }
}
#[doc = "Hyper-V Replica specific policy Input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVReplicaPolicyInput {
    #[serde(flatten)]
    pub policy_provider_specific_input: PolicyProviderSpecificInput,
    #[doc = "A value indicating the number of recovery points."]
    #[serde(rename = "recoveryPoints", default, skip_serializing_if = "Option::is_none")]
    pub recovery_points: Option<i32>,
    #[doc = "A value indicating the application consistent frequency."]
    #[serde(
        rename = "applicationConsistentSnapshotFrequencyInHours",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub application_consistent_snapshot_frequency_in_hours: Option<i32>,
    #[doc = "A value indicating whether compression has to be enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression: Option<String>,
    #[doc = "A value indicating whether IR is online."]
    #[serde(rename = "initialReplicationMethod", default, skip_serializing_if = "Option::is_none")]
    pub initial_replication_method: Option<String>,
    #[doc = "A value indicating the online IR start time."]
    #[serde(rename = "onlineReplicationStartTime", default, skip_serializing_if = "Option::is_none")]
    pub online_replication_start_time: Option<String>,
    #[doc = "A value indicating the offline IR import path."]
    #[serde(rename = "offlineReplicationImportPath", default, skip_serializing_if = "Option::is_none")]
    pub offline_replication_import_path: Option<String>,
    #[doc = "A value indicating the offline IR export path."]
    #[serde(rename = "offlineReplicationExportPath", default, skip_serializing_if = "Option::is_none")]
    pub offline_replication_export_path: Option<String>,
    #[doc = "A value indicating the recovery HTTPS port."]
    #[serde(rename = "replicationPort", default, skip_serializing_if = "Option::is_none")]
    pub replication_port: Option<i32>,
    #[doc = "A value indicating the authentication type."]
    #[serde(rename = "allowedAuthenticationType", default, skip_serializing_if = "Option::is_none")]
    pub allowed_authentication_type: Option<i32>,
    #[doc = "A value indicating whether the VM has to be auto deleted."]
    #[serde(rename = "replicaDeletion", default, skip_serializing_if = "Option::is_none")]
    pub replica_deletion: Option<String>,
}
impl HyperVReplicaPolicyInput {
    pub fn new(policy_provider_specific_input: PolicyProviderSpecificInput) -> Self {
        Self {
            policy_provider_specific_input,
            recovery_points: None,
            application_consistent_snapshot_frequency_in_hours: None,
            compression: None,
            initial_replication_method: None,
            online_replication_start_time: None,
            offline_replication_import_path: None,
            offline_replication_export_path: None,
            replication_port: None,
            allowed_authentication_type: None,
            replica_deletion: None,
        }
    }
}
#[doc = "HyperV replica 2012 replication details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVReplicaReplicationDetails {
    #[serde(flatten)]
    pub replication_provider_specific_settings: ReplicationProviderSpecificSettings,
    #[doc = "The Last replication time."]
    #[serde(rename = "lastReplicatedTime", with = "azure_core::date::rfc3339::option")]
    pub last_replicated_time: Option<time::OffsetDateTime>,
    #[doc = "The PE Network details."]
    #[serde(rename = "vmNics", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_nics: Vec<VmNicDetails>,
    #[doc = "The virtual machine Id."]
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[doc = "The protection state for the vm."]
    #[serde(rename = "vmProtectionState", default, skip_serializing_if = "Option::is_none")]
    pub vm_protection_state: Option<String>,
    #[doc = "The protection state description for the vm."]
    #[serde(rename = "vmProtectionStateDescription", default, skip_serializing_if = "Option::is_none")]
    pub vm_protection_state_description: Option<String>,
    #[doc = "Initial replication details."]
    #[serde(rename = "initialReplicationDetails", default, skip_serializing_if = "Option::is_none")]
    pub initial_replication_details: Option<InitialReplicationDetails>,
    #[doc = "VM disk details."]
    #[serde(rename = "vMDiskDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub v_m_disk_details: Vec<DiskDetails>,
}
impl HyperVReplicaReplicationDetails {
    pub fn new(replication_provider_specific_settings: ReplicationProviderSpecificSettings) -> Self {
        Self {
            replication_provider_specific_settings,
            last_replicated_time: None,
            vm_nics: Vec::new(),
            vm_id: None,
            vm_protection_state: None,
            vm_protection_state_description: None,
            initial_replication_details: None,
            v_m_disk_details: Vec::new(),
        }
    }
}
#[doc = "HyperVSite fabric specific details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVSiteDetails {
    #[serde(flatten)]
    pub fabric_specific_details: FabricSpecificDetails,
    #[doc = "The list of Hyper-V hosts associated with the fabric."]
    #[serde(rename = "hyperVHosts", default, skip_serializing_if = "Vec::is_empty")]
    pub hyper_v_hosts: Vec<HyperVHostDetails>,
}
impl HyperVSiteDetails {
    pub fn new(fabric_specific_details: FabricSpecificDetails) -> Self {
        Self {
            fabric_specific_details,
            hyper_v_hosts: Vec::new(),
        }
    }
}
#[doc = "Single Host fabric provider specific VM settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyperVVirtualMachineDetails {
    #[serde(flatten)]
    pub configuration_settings: ConfigurationSettings,
    #[doc = "The source id of the object."]
    #[serde(rename = "sourceItemId", default, skip_serializing_if = "Option::is_none")]
    pub source_item_id: Option<String>,
    #[doc = "The id of the object in fabric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,
    #[doc = "Disk Details."]
    #[serde(rename = "osDetails", default, skip_serializing_if = "Option::is_none")]
    pub os_details: Option<OsDetails>,
    #[doc = "The Last successful failover time."]
    #[serde(rename = "diskDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub disk_details: Vec<DiskDetails>,
    #[doc = "A value indicating whether the VM has a physical disk attached. String value of SrsDataContract.PresenceStatus enum."]
    #[serde(rename = "hasPhysicalDisk", default, skip_serializing_if = "Option::is_none")]
    pub has_physical_disk: Option<hyper_v_virtual_machine_details::HasPhysicalDisk>,
    #[doc = "A value indicating whether the VM has a fibre channel adapter attached. String value of SrsDataContract.PresenceStatus enum."]
    #[serde(rename = "hasFibreChannelAdapter", default, skip_serializing_if = "Option::is_none")]
    pub has_fibre_channel_adapter: Option<hyper_v_virtual_machine_details::HasFibreChannelAdapter>,
    #[doc = "A value indicating whether the VM has a shared VHD attached. String value of SrsDataContract.PresenceStatus enum."]
    #[serde(rename = "hasSharedVhd", default, skip_serializing_if = "Option::is_none")]
    pub has_shared_vhd: Option<hyper_v_virtual_machine_details::HasSharedVhd>,
    #[doc = "The Id of the hyper-v host in fabric."]
    #[serde(rename = "hyperVHostId", default, skip_serializing_if = "Option::is_none")]
    pub hyper_v_host_id: Option<String>,
}
impl HyperVVirtualMachineDetails {
    pub fn new(configuration_settings: ConfigurationSettings) -> Self {
        Self {
            configuration_settings,
            source_item_id: None,
            generation: None,
            os_details: None,
            disk_details: Vec::new(),
            has_physical_disk: None,
            has_fibre_channel_adapter: None,
            has_shared_vhd: None,
            hyper_v_host_id: None,
        }
    }
}
pub mod hyper_v_virtual_machine_details {
    use super::*;
    #[doc = "A value indicating whether the VM has a physical disk attached. String value of SrsDataContract.PresenceStatus enum."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HasPhysicalDisk")]
    pub enum HasPhysicalDisk {
        Unknown,
        Present,
        NotPresent,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HasPhysicalDisk {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HasPhysicalDisk {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HasPhysicalDisk {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("HasPhysicalDisk", 0u32, "Unknown"),
                Self::Present => serializer.serialize_unit_variant("HasPhysicalDisk", 1u32, "Present"),
                Self::NotPresent => serializer.serialize_unit_variant("HasPhysicalDisk", 2u32, "NotPresent"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "A value indicating whether the VM has a fibre channel adapter attached. String value of SrsDataContract.PresenceStatus enum."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HasFibreChannelAdapter")]
    pub enum HasFibreChannelAdapter {
        Unknown,
        Present,
        NotPresent,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HasFibreChannelAdapter {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HasFibreChannelAdapter {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HasFibreChannelAdapter {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("HasFibreChannelAdapter", 0u32, "Unknown"),
                Self::Present => serializer.serialize_unit_variant("HasFibreChannelAdapter", 1u32, "Present"),
                Self::NotPresent => serializer.serialize_unit_variant("HasFibreChannelAdapter", 2u32, "NotPresent"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "A value indicating whether the VM has a shared VHD attached. String value of SrsDataContract.PresenceStatus enum."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HasSharedVhd")]
    pub enum HasSharedVhd {
        Unknown,
        Present,
        NotPresent,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HasSharedVhd {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HasSharedVhd {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HasSharedVhd {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Unknown => serializer.serialize_unit_variant("HasSharedVhd", 0u32, "Unknown"),
                Self::Present => serializer.serialize_unit_variant("HasSharedVhd", 1u32, "Present"),
                Self::NotPresent => serializer.serialize_unit_variant("HasSharedVhd", 2u32, "NotPresent"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpConfigDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isPrimary", default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    #[serde(rename = "subnetName", default, skip_serializing_if = "Option::is_none")]
    pub subnet_name: Option<String>,
    #[serde(rename = "staticIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub static_ip_address: Option<String>,
    #[serde(rename = "ipAddressType", default, skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "isSeletedForFailover", default, skip_serializing_if = "Option::is_none")]
    pub is_seleted_for_failover: Option<bool>,
    #[serde(rename = "recoverySubnetName", default, skip_serializing_if = "Option::is_none")]
    pub recovery_subnet_name: Option<String>,
    #[serde(rename = "recoveryStaticIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub recovery_static_ip_address: Option<String>,
    #[serde(rename = "recoveryIPAddressType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_ip_address_type: Option<String>,
    #[serde(rename = "recoveryPublicIPAddressId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_public_ip_address_id: Option<String>,
    #[serde(rename = "recoveryLBBackendAddressPoolIds", default, skip_serializing_if = "Vec::is_empty")]
    pub recovery_lb_backend_address_pool_ids: Vec<String>,
    #[serde(rename = "tfoSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub tfo_subnet_name: Option<String>,
    #[serde(rename = "tfoStaticIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub tfo_static_ip_address: Option<String>,
    #[serde(rename = "tfoPublicIPAddressId", default, skip_serializing_if = "Option::is_none")]
    pub tfo_public_ip_address_id: Option<String>,
    #[serde(rename = "tfoLBBackendAddressPoolIds", default, skip_serializing_if = "Vec::is_empty")]
    pub tfo_lb_backend_address_pool_ids: Vec<String>,
}
impl IpConfigDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpConfigInputDetails {
    #[serde(rename = "ipConfigName", default, skip_serializing_if = "Option::is_none")]
    pub ip_config_name: Option<String>,
    #[serde(rename = "isPrimary", default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    #[serde(rename = "isSeletedForFailover", default, skip_serializing_if = "Option::is_none")]
    pub is_seleted_for_failover: Option<bool>,
    #[serde(rename = "recoverySubnetName", default, skip_serializing_if = "Option::is_none")]
    pub recovery_subnet_name: Option<String>,
    #[serde(rename = "recoveryStaticIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub recovery_static_ip_address: Option<String>,
    #[serde(rename = "recoveryPublicIPAddressId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_public_ip_address_id: Option<String>,
    #[serde(rename = "recoveryLBBackendAddressPoolIds", default, skip_serializing_if = "Vec::is_empty")]
    pub recovery_lb_backend_address_pool_ids: Vec<String>,
    #[serde(rename = "tfoSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub tfo_subnet_name: Option<String>,
    #[serde(rename = "tfoStaticIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub tfo_static_ip_address: Option<String>,
    #[serde(rename = "tfoPublicIPAddressId", default, skip_serializing_if = "Option::is_none")]
    pub tfo_public_ip_address_id: Option<String>,
    #[serde(rename = "tfoLBBackendAddressPoolIds", default, skip_serializing_if = "Vec::is_empty")]
    pub tfo_lb_backend_address_pool_ids: Vec<String>,
}
impl IpConfigInputDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity provider details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityProviderDetails {
    #[doc = "The tenant Id for the service principal with which the on-premise management/data plane components would communicate with our Azure services."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The application/client Id for the service principal with which the on-premise management/data plane components would communicate with our Azure services."]
    #[serde(rename = "applicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[doc = "The object Id of the service principal with which the on-premise management/data plane components would communicate with our Azure services."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "The intended Audience of the service principal with which the on-premise management/data plane components would communicate with our Azure services."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[doc = "The base authority for Azure Active Directory authentication."]
    #[serde(rename = "aadAuthority", default, skip_serializing_if = "Option::is_none")]
    pub aad_authority: Option<String>,
}
impl IdentityProviderDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity provider input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityProviderInput {
    #[doc = "The tenant Id for the service principal with which the on-premise management/data plane components would communicate with our Azure services."]
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[doc = "The application/client Id for the service principal with which the on-premise management/data plane components would communicate with our Azure services."]
    #[serde(rename = "applicationId")]
    pub application_id: String,
    #[doc = "The object Id of the service principal with which the on-premise management/data plane components would communicate with our Azure services."]
    #[serde(rename = "objectId")]
    pub object_id: String,
    #[doc = "The intended Audience of the service principal with which the on-premise management/data plane components would communicate with our Azure services."]
    pub audience: String,
    #[doc = "The base authority for Azure Active Directory authentication."]
    #[serde(rename = "aadAuthority")]
    pub aad_authority: String,
}
impl IdentityProviderInput {
    pub fn new(tenant_id: String, application_id: String, object_id: String, audience: String, aad_authority: String) -> Self {
        Self {
            tenant_id,
            application_id,
            object_id,
            audience,
            aad_authority,
        }
    }
}
#[doc = "The details of the InMage agent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageAgentDetails {
    #[doc = "The agent version."]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "A value indicating whether installed agent needs to be updated."]
    #[serde(rename = "agentUpdateStatus", default, skip_serializing_if = "Option::is_none")]
    pub agent_update_status: Option<String>,
    #[doc = "A value indicating whether reboot is required after update is applied."]
    #[serde(rename = "postUpdateRebootStatus", default, skip_serializing_if = "Option::is_none")]
    pub post_update_reboot_status: Option<String>,
    #[doc = "Agent expiry date."]
    #[serde(rename = "agentExpiryDate", with = "azure_core::date::rfc3339::option")]
    pub agent_expiry_date: Option<time::OffsetDateTime>,
}
impl InMageAgentDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ApplyRecoveryPoint input specific to InMageAzureV2 provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageAzureV2ApplyRecoveryPointInput {
    #[serde(flatten)]
    pub apply_recovery_point_provider_specific_input: ApplyRecoveryPointProviderSpecificInput,
}
impl InMageAzureV2ApplyRecoveryPointInput {
    pub fn new(apply_recovery_point_provider_specific_input: ApplyRecoveryPointProviderSpecificInput) -> Self {
        Self {
            apply_recovery_point_provider_specific_input,
        }
    }
}
#[doc = "Disk input details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageAzureV2DiskInputDetails {
    #[doc = "The DiskId."]
    #[serde(rename = "diskId", default, skip_serializing_if = "Option::is_none")]
    pub disk_id: Option<String>,
    #[doc = "The LogStorageAccountId."]
    #[serde(rename = "logStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub log_storage_account_id: Option<String>,
    #[doc = "The DiskType."]
    #[serde(rename = "diskType", default, skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<in_mage_azure_v2_disk_input_details::DiskType>,
    #[doc = "The DiskEncryptionSet ARM ID."]
    #[serde(rename = "diskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set_id: Option<String>,
}
impl InMageAzureV2DiskInputDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod in_mage_azure_v2_disk_input_details {
    use super::*;
    #[doc = "The DiskType."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DiskType")]
    pub enum DiskType {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(rename = "StandardSSD_LRS")]
        StandardSsdLrs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DiskType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DiskType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DiskType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::StandardLrs => serializer.serialize_unit_variant("DiskType", 0u32, "Standard_LRS"),
                Self::PremiumLrs => serializer.serialize_unit_variant("DiskType", 1u32, "Premium_LRS"),
                Self::StandardSsdLrs => serializer.serialize_unit_variant("DiskType", 2u32, "StandardSSD_LRS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "VMware Azure specific enable protection input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageAzureV2EnableProtectionInput {
    #[serde(flatten)]
    pub enable_protection_provider_specific_input: EnableProtectionProviderSpecificInput,
    #[doc = "The Master target Id."]
    #[serde(rename = "masterTargetId", default, skip_serializing_if = "Option::is_none")]
    pub master_target_id: Option<String>,
    #[doc = "The Process Server Id."]
    #[serde(rename = "processServerId", default, skip_serializing_if = "Option::is_none")]
    pub process_server_id: Option<String>,
    #[doc = "The storage account Id."]
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[doc = "The CS account Id."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "The multi VM group Id."]
    #[serde(rename = "multiVmGroupId", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_group_id: Option<String>,
    #[doc = "The multi VM group name."]
    #[serde(rename = "multiVmGroupName", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_group_name: Option<String>,
    #[doc = "The disks to include list."]
    #[serde(rename = "disksToInclude", default, skip_serializing_if = "Vec::is_empty")]
    pub disks_to_include: Vec<InMageAzureV2DiskInputDetails>,
    #[doc = "The selected target Azure network Id."]
    #[serde(rename = "targetAzureNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub target_azure_network_id: Option<String>,
    #[doc = "The selected target Azure subnet Id."]
    #[serde(rename = "targetAzureSubnetId", default, skip_serializing_if = "Option::is_none")]
    pub target_azure_subnet_id: Option<String>,
    #[doc = "The selected option to enable RDP\\SSH on target VM after failover. String value of SrsDataContract.EnableRDPOnTargetOption enum."]
    #[serde(rename = "enableRdpOnTargetOption", default, skip_serializing_if = "Option::is_none")]
    pub enable_rdp_on_target_option: Option<String>,
    #[doc = "The target azure VM Name."]
    #[serde(rename = "targetAzureVmName", default, skip_serializing_if = "Option::is_none")]
    pub target_azure_vm_name: Option<String>,
    #[doc = "The storage account to be used for logging during replication."]
    #[serde(rename = "logStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub log_storage_account_id: Option<String>,
    #[doc = "The Id of the target resource group (for classic deployment) in which the failover VM is to be created."]
    #[serde(rename = "targetAzureV1ResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub target_azure_v1_resource_group_id: Option<String>,
    #[doc = "The Id of the target resource group (for resource manager deployment) in which the failover VM is to be created."]
    #[serde(rename = "targetAzureV2ResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub target_azure_v2_resource_group_id: Option<String>,
    #[doc = "The DiskType."]
    #[serde(rename = "diskType", default, skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<in_mage_azure_v2_enable_protection_input::DiskType>,
    #[doc = "The target availability set ARM Id for resource manager deployment."]
    #[serde(rename = "targetAvailabilitySetId", default, skip_serializing_if = "Option::is_none")]
    pub target_availability_set_id: Option<String>,
    #[doc = "The target availability zone."]
    #[serde(rename = "targetAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub target_availability_zone: Option<String>,
    #[doc = "The proximity placement group ARM Id."]
    #[serde(rename = "targetProximityPlacementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub target_proximity_placement_group_id: Option<String>,
    #[doc = "License type."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<in_mage_azure_v2_enable_protection_input::LicenseType>,
    #[doc = "The SQL Server license type."]
    #[serde(rename = "sqlServerLicenseType", default, skip_serializing_if = "Option::is_none")]
    pub sql_server_license_type: Option<in_mage_azure_v2_enable_protection_input::SqlServerLicenseType>,
    #[doc = "The target VM size."]
    #[serde(rename = "targetVmSize", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_size: Option<String>,
    #[doc = "The DiskEncryptionSet ARM Id."]
    #[serde(rename = "diskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set_id: Option<String>,
    #[doc = "The target VM tags."]
    #[serde(rename = "targetVmTags", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_tags: Option<serde_json::Value>,
    #[doc = "The tags for the seed managed disks."]
    #[serde(rename = "seedManagedDiskTags", default, skip_serializing_if = "Option::is_none")]
    pub seed_managed_disk_tags: Option<serde_json::Value>,
    #[doc = "The tags for the target managed disks."]
    #[serde(rename = "targetManagedDiskTags", default, skip_serializing_if = "Option::is_none")]
    pub target_managed_disk_tags: Option<serde_json::Value>,
    #[doc = "The tags for the target NICs."]
    #[serde(rename = "targetNicTags", default, skip_serializing_if = "Option::is_none")]
    pub target_nic_tags: Option<serde_json::Value>,
}
impl InMageAzureV2EnableProtectionInput {
    pub fn new(enable_protection_provider_specific_input: EnableProtectionProviderSpecificInput) -> Self {
        Self {
            enable_protection_provider_specific_input,
            master_target_id: None,
            process_server_id: None,
            storage_account_id: None,
            run_as_account_id: None,
            multi_vm_group_id: None,
            multi_vm_group_name: None,
            disks_to_include: Vec::new(),
            target_azure_network_id: None,
            target_azure_subnet_id: None,
            enable_rdp_on_target_option: None,
            target_azure_vm_name: None,
            log_storage_account_id: None,
            target_azure_v1_resource_group_id: None,
            target_azure_v2_resource_group_id: None,
            disk_type: None,
            target_availability_set_id: None,
            target_availability_zone: None,
            target_proximity_placement_group_id: None,
            license_type: None,
            sql_server_license_type: None,
            target_vm_size: None,
            disk_encryption_set_id: None,
            target_vm_tags: None,
            seed_managed_disk_tags: None,
            target_managed_disk_tags: None,
            target_nic_tags: None,
        }
    }
}
pub mod in_mage_azure_v2_enable_protection_input {
    use super::*;
    #[doc = "The DiskType."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DiskType")]
    pub enum DiskType {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(rename = "StandardSSD_LRS")]
        StandardSsdLrs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DiskType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DiskType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DiskType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::StandardLrs => serializer.serialize_unit_variant("DiskType", 0u32, "Standard_LRS"),
                Self::PremiumLrs => serializer.serialize_unit_variant("DiskType", 1u32, "Premium_LRS"),
                Self::StandardSsdLrs => serializer.serialize_unit_variant("DiskType", 2u32, "StandardSSD_LRS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "License type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseType")]
    pub enum LicenseType {
        NotSpecified,
        NoLicenseType,
        WindowsServer,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("LicenseType", 0u32, "NotSpecified"),
                Self::NoLicenseType => serializer.serialize_unit_variant("LicenseType", 1u32, "NoLicenseType"),
                Self::WindowsServer => serializer.serialize_unit_variant("LicenseType", 2u32, "WindowsServer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The SQL Server license type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SqlServerLicenseType")]
    pub enum SqlServerLicenseType {
        NotSpecified,
        NoLicenseType,
        #[serde(rename = "PAYG")]
        Payg,
        #[serde(rename = "AHUB")]
        Ahub,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SqlServerLicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SqlServerLicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SqlServerLicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("SqlServerLicenseType", 0u32, "NotSpecified"),
                Self::NoLicenseType => serializer.serialize_unit_variant("SqlServerLicenseType", 1u32, "NoLicenseType"),
                Self::Payg => serializer.serialize_unit_variant("SqlServerLicenseType", 2u32, "PAYG"),
                Self::Ahub => serializer.serialize_unit_variant("SqlServerLicenseType", 3u32, "AHUB"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Model class for event details of a VMwareAzureV2 event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageAzureV2EventDetails {
    #[serde(flatten)]
    pub event_provider_specific_details: EventProviderSpecificDetails,
    #[doc = "InMage Event type. Takes one of the values of InMageDataContract.InMageMonitoringEventType."]
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[doc = "InMage Event Category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "InMage Event Component."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[doc = "Corrective Action string for the event."]
    #[serde(rename = "correctiveAction", default, skip_serializing_if = "Option::is_none")]
    pub corrective_action: Option<String>,
    #[doc = "InMage Event Details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[doc = "InMage Event Summary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[doc = "VMware Site name."]
    #[serde(rename = "siteName", default, skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
}
impl InMageAzureV2EventDetails {
    pub fn new(event_provider_specific_details: EventProviderSpecificDetails) -> Self {
        Self {
            event_provider_specific_details,
            event_type: None,
            category: None,
            component: None,
            corrective_action: None,
            details: None,
            summary: None,
            site_name: None,
        }
    }
}
#[doc = "InMageAzureV2 Managed disk details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageAzureV2ManagedDiskDetails {
    #[doc = "The disk id."]
    #[serde(rename = "diskId", default, skip_serializing_if = "Option::is_none")]
    pub disk_id: Option<String>,
    #[doc = "Seed managed disk Id."]
    #[serde(rename = "seedManagedDiskId", default, skip_serializing_if = "Option::is_none")]
    pub seed_managed_disk_id: Option<String>,
    #[doc = "The replica disk type."]
    #[serde(rename = "replicaDiskType", default, skip_serializing_if = "Option::is_none")]
    pub replica_disk_type: Option<String>,
    #[doc = "The DiskEncryptionSet ARM ID."]
    #[serde(rename = "diskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set_id: Option<String>,
    #[doc = "The target disk name."]
    #[serde(rename = "targetDiskName", default, skip_serializing_if = "Option::is_none")]
    pub target_disk_name: Option<String>,
}
impl InMageAzureV2ManagedDiskDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "InMage Azure v2 specific protection profile details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageAzureV2PolicyDetails {
    #[serde(flatten)]
    pub policy_provider_specific_details: PolicyProviderSpecificDetails,
    #[doc = "The crash consistent snapshot frequency in minutes."]
    #[serde(rename = "crashConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub crash_consistent_frequency_in_minutes: Option<i32>,
    #[doc = "The recovery point threshold in minutes."]
    #[serde(rename = "recoveryPointThresholdInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_threshold_in_minutes: Option<i32>,
    #[doc = "The duration in minutes until which the recovery points need to be stored."]
    #[serde(rename = "recoveryPointHistory", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_history: Option<i32>,
    #[doc = "The app consistent snapshot frequency in minutes."]
    #[serde(rename = "appConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub app_consistent_frequency_in_minutes: Option<i32>,
    #[doc = "A value indicating whether multi-VM sync has to be enabled."]
    #[serde(rename = "multiVmSyncStatus", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_sync_status: Option<String>,
}
impl InMageAzureV2PolicyDetails {
    pub fn new(policy_provider_specific_details: PolicyProviderSpecificDetails) -> Self {
        Self {
            policy_provider_specific_details,
            crash_consistent_frequency_in_minutes: None,
            recovery_point_threshold_in_minutes: None,
            recovery_point_history: None,
            app_consistent_frequency_in_minutes: None,
            multi_vm_sync_status: None,
        }
    }
}
#[doc = "VMWare Azure specific policy Input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageAzureV2PolicyInput {
    #[serde(flatten)]
    pub policy_provider_specific_input: PolicyProviderSpecificInput,
    #[doc = "The recovery point threshold in minutes."]
    #[serde(rename = "recoveryPointThresholdInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_threshold_in_minutes: Option<i32>,
    #[doc = "The duration in minutes until which the recovery points need to be stored."]
    #[serde(rename = "recoveryPointHistory", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_history: Option<i32>,
    #[doc = "The crash consistent snapshot frequency (in minutes)."]
    #[serde(rename = "crashConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub crash_consistent_frequency_in_minutes: Option<i32>,
    #[doc = "The app consistent snapshot frequency (in minutes)."]
    #[serde(rename = "appConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub app_consistent_frequency_in_minutes: Option<i32>,
    #[doc = "A value indicating whether multi-VM sync has to be enabled. Value should be 'Enabled' or 'Disabled'."]
    #[serde(rename = "multiVmSyncStatus")]
    pub multi_vm_sync_status: in_mage_azure_v2_policy_input::MultiVmSyncStatus,
}
impl InMageAzureV2PolicyInput {
    pub fn new(
        policy_provider_specific_input: PolicyProviderSpecificInput,
        multi_vm_sync_status: in_mage_azure_v2_policy_input::MultiVmSyncStatus,
    ) -> Self {
        Self {
            policy_provider_specific_input,
            recovery_point_threshold_in_minutes: None,
            recovery_point_history: None,
            crash_consistent_frequency_in_minutes: None,
            app_consistent_frequency_in_minutes: None,
            multi_vm_sync_status,
        }
    }
}
pub mod in_mage_azure_v2_policy_input {
    use super::*;
    #[doc = "A value indicating whether multi-VM sync has to be enabled. Value should be 'Enabled' or 'Disabled'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MultiVmSyncStatus")]
    pub enum MultiVmSyncStatus {
        Enable,
        Disable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MultiVmSyncStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MultiVmSyncStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MultiVmSyncStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enable => serializer.serialize_unit_variant("MultiVmSyncStatus", 0u32, "Enable"),
                Self::Disable => serializer.serialize_unit_variant("MultiVmSyncStatus", 1u32, "Disable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "InMageAzureV2 protected disk details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageAzureV2ProtectedDiskDetails {
    #[doc = "The disk id."]
    #[serde(rename = "diskId", default, skip_serializing_if = "Option::is_none")]
    pub disk_id: Option<String>,
    #[doc = "The disk name."]
    #[serde(rename = "diskName", default, skip_serializing_if = "Option::is_none")]
    pub disk_name: Option<String>,
    #[doc = "The protection stage."]
    #[serde(rename = "protectionStage", default, skip_serializing_if = "Option::is_none")]
    pub protection_stage: Option<String>,
    #[doc = "The health error code for the disk."]
    #[serde(rename = "healthErrorCode", default, skip_serializing_if = "Option::is_none")]
    pub health_error_code: Option<String>,
    #[doc = "The RPO in seconds."]
    #[serde(rename = "rpoInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub rpo_in_seconds: Option<i64>,
    #[doc = "A value indicating whether resync is required for this disk."]
    #[serde(rename = "resyncRequired", default, skip_serializing_if = "Option::is_none")]
    pub resync_required: Option<String>,
    #[doc = "The resync progress percentage."]
    #[serde(rename = "resyncProgressPercentage", default, skip_serializing_if = "Option::is_none")]
    pub resync_progress_percentage: Option<i32>,
    #[doc = "The resync duration in seconds."]
    #[serde(rename = "resyncDurationInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub resync_duration_in_seconds: Option<i64>,
    #[doc = "The disk capacity in bytes."]
    #[serde(rename = "diskCapacityInBytes", default, skip_serializing_if = "Option::is_none")]
    pub disk_capacity_in_bytes: Option<i64>,
    #[doc = "The disk file system capacity in bytes."]
    #[serde(rename = "fileSystemCapacityInBytes", default, skip_serializing_if = "Option::is_none")]
    pub file_system_capacity_in_bytes: Option<i64>,
    #[doc = "The source data transit in MB."]
    #[serde(rename = "sourceDataInMegaBytes", default, skip_serializing_if = "Option::is_none")]
    pub source_data_in_mega_bytes: Option<f64>,
    #[doc = "The PS data transit in MB."]
    #[serde(rename = "psDataInMegaBytes", default, skip_serializing_if = "Option::is_none")]
    pub ps_data_in_mega_bytes: Option<f64>,
    #[doc = "The target data transit in MB."]
    #[serde(rename = "targetDataInMegaBytes", default, skip_serializing_if = "Option::is_none")]
    pub target_data_in_mega_bytes: Option<f64>,
    #[doc = "A value indicating whether disk is resized."]
    #[serde(rename = "diskResized", default, skip_serializing_if = "Option::is_none")]
    pub disk_resized: Option<String>,
    #[doc = "The last RPO calculated time."]
    #[serde(rename = "lastRpoCalculatedTime", with = "azure_core::date::rfc3339::option")]
    pub last_rpo_calculated_time: Option<time::OffsetDateTime>,
    #[doc = "The resync processed bytes."]
    #[serde(rename = "resyncProcessedBytes", default, skip_serializing_if = "Option::is_none")]
    pub resync_processed_bytes: Option<i64>,
    #[doc = "The resync total transferred bytes."]
    #[serde(rename = "resyncTotalTransferredBytes", default, skip_serializing_if = "Option::is_none")]
    pub resync_total_transferred_bytes: Option<i64>,
    #[doc = "The resync last 15 minutes transferred bytes."]
    #[serde(rename = "resyncLast15MinutesTransferredBytes", default, skip_serializing_if = "Option::is_none")]
    pub resync_last15_minutes_transferred_bytes: Option<i64>,
    #[doc = "The last data transfer time in UTC."]
    #[serde(rename = "resyncLastDataTransferTimeUTC", with = "azure_core::date::rfc3339::option")]
    pub resync_last_data_transfer_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The resync start time."]
    #[serde(rename = "resyncStartTime", with = "azure_core::date::rfc3339::option")]
    pub resync_start_time: Option<time::OffsetDateTime>,
    #[doc = "The Progress Health."]
    #[serde(rename = "progressHealth", default, skip_serializing_if = "Option::is_none")]
    pub progress_health: Option<String>,
    #[doc = "The Progress Status."]
    #[serde(rename = "progressStatus", default, skip_serializing_if = "Option::is_none")]
    pub progress_status: Option<String>,
    #[doc = "The seconds to take for switch provider."]
    #[serde(rename = "secondsToTakeSwitchProvider", default, skip_serializing_if = "Option::is_none")]
    pub seconds_to_take_switch_provider: Option<i64>,
}
impl InMageAzureV2ProtectedDiskDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "InMage Azure V2 provider specific recovery point details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageAzureV2RecoveryPointDetails {
    #[serde(flatten)]
    pub provider_specific_recovery_point_details: ProviderSpecificRecoveryPointDetails,
    #[doc = "A value indicating whether the recovery point is multi VM consistent."]
    #[serde(rename = "isMultiVmSyncPoint", default, skip_serializing_if = "Option::is_none")]
    pub is_multi_vm_sync_point: Option<String>,
}
impl InMageAzureV2RecoveryPointDetails {
    pub fn new(provider_specific_recovery_point_details: ProviderSpecificRecoveryPointDetails) -> Self {
        Self {
            provider_specific_recovery_point_details,
            is_multi_vm_sync_point: None,
        }
    }
}
#[doc = "InMageAzureV2 provider specific settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageAzureV2ReplicationDetails {
    #[serde(flatten)]
    pub replication_provider_specific_settings: ReplicationProviderSpecificSettings,
    #[doc = "The infrastructure VM Id."]
    #[serde(rename = "infrastructureVmId", default, skip_serializing_if = "Option::is_none")]
    pub infrastructure_vm_id: Option<String>,
    #[doc = "The vCenter infrastructure Id."]
    #[serde(rename = "vCenterInfrastructureId", default, skip_serializing_if = "Option::is_none")]
    pub v_center_infrastructure_id: Option<String>,
    #[doc = "The protection stage."]
    #[serde(rename = "protectionStage", default, skip_serializing_if = "Option::is_none")]
    pub protection_stage: Option<String>,
    #[doc = "The virtual machine Id."]
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[doc = "The protection state for the vm."]
    #[serde(rename = "vmProtectionState", default, skip_serializing_if = "Option::is_none")]
    pub vm_protection_state: Option<String>,
    #[doc = "The protection state description for the vm."]
    #[serde(rename = "vmProtectionStateDescription", default, skip_serializing_if = "Option::is_none")]
    pub vm_protection_state_description: Option<String>,
    #[doc = "The resync progress percentage."]
    #[serde(rename = "resyncProgressPercentage", default, skip_serializing_if = "Option::is_none")]
    pub resync_progress_percentage: Option<i32>,
    #[doc = "The RPO in seconds."]
    #[serde(rename = "rpoInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub rpo_in_seconds: Option<i64>,
    #[doc = "The compressed data change rate in MB."]
    #[serde(rename = "compressedDataRateInMB", default, skip_serializing_if = "Option::is_none")]
    pub compressed_data_rate_in_mb: Option<f64>,
    #[doc = "The uncompressed data change rate in MB."]
    #[serde(rename = "uncompressedDataRateInMB", default, skip_serializing_if = "Option::is_none")]
    pub uncompressed_data_rate_in_mb: Option<f64>,
    #[doc = "The source IP address."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The agent version."]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "Agent expiry date."]
    #[serde(rename = "agentExpiryDate", with = "azure_core::date::rfc3339::option")]
    pub agent_expiry_date: Option<time::OffsetDateTime>,
    #[doc = "A value indicating whether installed agent needs to be updated."]
    #[serde(rename = "isAgentUpdateRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_agent_update_required: Option<String>,
    #[doc = "A value indicating whether the source server requires a restart after update."]
    #[serde(rename = "isRebootAfterUpdateRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_reboot_after_update_required: Option<String>,
    #[doc = "The last heartbeat received from the source server."]
    #[serde(rename = "lastHeartbeat", with = "azure_core::date::rfc3339::option")]
    pub last_heartbeat: Option<time::OffsetDateTime>,
    #[doc = "The process server Id."]
    #[serde(rename = "processServerId", default, skip_serializing_if = "Option::is_none")]
    pub process_server_id: Option<String>,
    #[doc = "The process server name."]
    #[serde(rename = "processServerName", default, skip_serializing_if = "Option::is_none")]
    pub process_server_name: Option<String>,
    #[doc = "The multi vm group Id."]
    #[serde(rename = "multiVmGroupId", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_group_id: Option<String>,
    #[doc = "The multi vm group name."]
    #[serde(rename = "multiVmGroupName", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_group_name: Option<String>,
    #[doc = "A value indicating whether multi vm sync is enabled or disabled."]
    #[serde(rename = "multiVmSyncStatus", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_sync_status: Option<String>,
    #[doc = "The list of protected disks."]
    #[serde(rename = "protectedDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub protected_disks: Vec<InMageAzureV2ProtectedDiskDetails>,
    #[doc = "A value indicating whether any disk is resized for this VM."]
    #[serde(rename = "diskResized", default, skip_serializing_if = "Option::is_none")]
    pub disk_resized: Option<String>,
    #[doc = "The master target Id."]
    #[serde(rename = "masterTargetId", default, skip_serializing_if = "Option::is_none")]
    pub master_target_id: Option<String>,
    #[doc = "The CPU count of the VM on the primary side."]
    #[serde(rename = "sourceVmCpuCount", default, skip_serializing_if = "Option::is_none")]
    pub source_vm_cpu_count: Option<i32>,
    #[doc = "The RAM size of the VM on the primary side."]
    #[serde(rename = "sourceVmRamSizeInMB", default, skip_serializing_if = "Option::is_none")]
    pub source_vm_ram_size_in_mb: Option<i32>,
    #[doc = "The type of the OS on the VM."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "The OS disk VHD name."]
    #[serde(rename = "vhdName", default, skip_serializing_if = "Option::is_none")]
    pub vhd_name: Option<String>,
    #[doc = "The id of the disk containing the OS."]
    #[serde(rename = "osDiskId", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_id: Option<String>,
    #[doc = "Azure VM Disk details."]
    #[serde(rename = "azureVMDiskDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_vm_disk_details: Vec<AzureVmDiskDetails>,
    #[doc = "Recovery Azure given name."]
    #[serde(rename = "recoveryAzureVMName", default, skip_serializing_if = "Option::is_none")]
    pub recovery_azure_vm_name: Option<String>,
    #[doc = "The Recovery Azure VM size."]
    #[serde(rename = "recoveryAzureVMSize", default, skip_serializing_if = "Option::is_none")]
    pub recovery_azure_vm_size: Option<String>,
    #[doc = "The recovery Azure storage account."]
    #[serde(rename = "recoveryAzureStorageAccount", default, skip_serializing_if = "Option::is_none")]
    pub recovery_azure_storage_account: Option<String>,
    #[doc = "The ARM id of the log storage account used for replication. This will be set to null if no log storage account was provided during enable protection."]
    #[serde(rename = "recoveryAzureLogStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_azure_log_storage_account_id: Option<String>,
    #[doc = "The PE Network details."]
    #[serde(rename = "vmNics", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_nics: Vec<VmNicDetails>,
    #[doc = "The selected recovery azure network Id."]
    #[serde(rename = "selectedRecoveryAzureNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub selected_recovery_azure_network_id: Option<String>,
    #[doc = "The test failover virtual network."]
    #[serde(rename = "selectedTfoAzureNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub selected_tfo_azure_network_id: Option<String>,
    #[doc = "The selected source nic Id which will be used as the primary nic during failover."]
    #[serde(rename = "selectedSourceNicId", default, skip_serializing_if = "Option::is_none")]
    pub selected_source_nic_id: Option<String>,
    #[doc = "A value indicating the discovery type of the machine. Value can be vCenter or physical."]
    #[serde(rename = "discoveryType", default, skip_serializing_if = "Option::is_none")]
    pub discovery_type: Option<String>,
    #[doc = "The selected option to enable RDP\\SSH on target vm after failover. String value of SrsDataContract.EnableRDPOnTargetOption enum."]
    #[serde(rename = "enableRdpOnTargetOption", default, skip_serializing_if = "Option::is_none")]
    pub enable_rdp_on_target_option: Option<String>,
    #[doc = "The datastores of the on-premise machine. Value can be list of strings that contain datastore names."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub datastores: Vec<String>,
    #[doc = "The ARM Id of the target Azure VM. This value will be null until the VM is failed over. Only after failure it will be populated with the ARM Id of the Azure VM."]
    #[serde(rename = "targetVmId", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_id: Option<String>,
    #[doc = "The target resource group Id."]
    #[serde(rename = "recoveryAzureResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_azure_resource_group_id: Option<String>,
    #[doc = "The recovery availability set Id."]
    #[serde(rename = "recoveryAvailabilitySetId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_availability_set_id: Option<String>,
    #[doc = "The target availability zone."]
    #[serde(rename = "targetAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub target_availability_zone: Option<String>,
    #[doc = "The target proximity placement group Id."]
    #[serde(rename = "targetProximityPlacementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub target_proximity_placement_group_id: Option<String>,
    #[doc = "A value indicating whether managed disks should be used during failover."]
    #[serde(rename = "useManagedDisks", default, skip_serializing_if = "Option::is_none")]
    pub use_managed_disks: Option<String>,
    #[doc = "License Type of the VM to be used."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    #[doc = "The SQL Server license type."]
    #[serde(rename = "sqlServerLicenseType", default, skip_serializing_if = "Option::is_none")]
    pub sql_server_license_type: Option<String>,
    #[doc = "The validation errors of the on-premise machine Value can be list of validation errors."]
    #[serde(rename = "validationErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub validation_errors: Vec<HealthError>,
    #[doc = "The last RPO calculated time."]
    #[serde(rename = "lastRpoCalculatedTime", with = "azure_core::date::rfc3339::option")]
    pub last_rpo_calculated_time: Option<time::OffsetDateTime>,
    #[doc = "The last update time received from on-prem components."]
    #[serde(rename = "lastUpdateReceivedTime", with = "azure_core::date::rfc3339::option")]
    pub last_update_received_time: Option<time::OffsetDateTime>,
    #[doc = "The replica id of the protected item."]
    #[serde(rename = "replicaId", default, skip_serializing_if = "Option::is_none")]
    pub replica_id: Option<String>,
    #[doc = "The OS Version of the protected item."]
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[doc = "The list of protected managed disks."]
    #[serde(rename = "protectedManagedDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub protected_managed_disks: Vec<InMageAzureV2ManagedDiskDetails>,
    #[doc = "The last recovery point received time."]
    #[serde(rename = "lastRecoveryPointReceived", with = "azure_core::date::rfc3339::option")]
    pub last_recovery_point_received: Option<time::OffsetDateTime>,
    #[doc = "The firmware type of this protected item."]
    #[serde(rename = "firmwareType", default, skip_serializing_if = "Option::is_none")]
    pub firmware_type: Option<String>,
    #[doc = "The target generation for this protected item."]
    #[serde(rename = "azureVmGeneration", default, skip_serializing_if = "Option::is_none")]
    pub azure_vm_generation: Option<String>,
    #[doc = "A value indicating whether additional IR stats are available or not."]
    #[serde(rename = "isAdditionalStatsAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_additional_stats_available: Option<bool>,
    #[doc = "The total transferred data in bytes."]
    #[serde(rename = "totalDataTransferred", default, skip_serializing_if = "Option::is_none")]
    pub total_data_transferred: Option<i64>,
    #[doc = "The progress health."]
    #[serde(rename = "totalProgressHealth", default, skip_serializing_if = "Option::is_none")]
    pub total_progress_health: Option<String>,
    #[doc = "The target VM tags."]
    #[serde(rename = "targetVmTags", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_tags: Option<serde_json::Value>,
    #[doc = "The tags for the seed managed disks."]
    #[serde(rename = "seedManagedDiskTags", default, skip_serializing_if = "Option::is_none")]
    pub seed_managed_disk_tags: Option<serde_json::Value>,
    #[doc = "The tags for the target managed disks."]
    #[serde(rename = "targetManagedDiskTags", default, skip_serializing_if = "Option::is_none")]
    pub target_managed_disk_tags: Option<serde_json::Value>,
    #[doc = "The tags for the target NICs."]
    #[serde(rename = "targetNicTags", default, skip_serializing_if = "Option::is_none")]
    pub target_nic_tags: Option<serde_json::Value>,
    #[doc = "The switch provider blocking error information."]
    #[serde(rename = "switchProviderBlockingErrorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub switch_provider_blocking_error_details: Vec<InMageAzureV2SwitchProviderBlockingErrorDetails>,
    #[doc = "InMageAzureV2 switch provider details."]
    #[serde(rename = "switchProviderDetails", default, skip_serializing_if = "Option::is_none")]
    pub switch_provider_details: Option<InMageAzureV2SwitchProviderDetails>,
}
impl InMageAzureV2ReplicationDetails {
    pub fn new(replication_provider_specific_settings: ReplicationProviderSpecificSettings) -> Self {
        Self {
            replication_provider_specific_settings,
            infrastructure_vm_id: None,
            v_center_infrastructure_id: None,
            protection_stage: None,
            vm_id: None,
            vm_protection_state: None,
            vm_protection_state_description: None,
            resync_progress_percentage: None,
            rpo_in_seconds: None,
            compressed_data_rate_in_mb: None,
            uncompressed_data_rate_in_mb: None,
            ip_address: None,
            agent_version: None,
            agent_expiry_date: None,
            is_agent_update_required: None,
            is_reboot_after_update_required: None,
            last_heartbeat: None,
            process_server_id: None,
            process_server_name: None,
            multi_vm_group_id: None,
            multi_vm_group_name: None,
            multi_vm_sync_status: None,
            protected_disks: Vec::new(),
            disk_resized: None,
            master_target_id: None,
            source_vm_cpu_count: None,
            source_vm_ram_size_in_mb: None,
            os_type: None,
            vhd_name: None,
            os_disk_id: None,
            azure_vm_disk_details: Vec::new(),
            recovery_azure_vm_name: None,
            recovery_azure_vm_size: None,
            recovery_azure_storage_account: None,
            recovery_azure_log_storage_account_id: None,
            vm_nics: Vec::new(),
            selected_recovery_azure_network_id: None,
            selected_tfo_azure_network_id: None,
            selected_source_nic_id: None,
            discovery_type: None,
            enable_rdp_on_target_option: None,
            datastores: Vec::new(),
            target_vm_id: None,
            recovery_azure_resource_group_id: None,
            recovery_availability_set_id: None,
            target_availability_zone: None,
            target_proximity_placement_group_id: None,
            use_managed_disks: None,
            license_type: None,
            sql_server_license_type: None,
            validation_errors: Vec::new(),
            last_rpo_calculated_time: None,
            last_update_received_time: None,
            replica_id: None,
            os_version: None,
            protected_managed_disks: Vec::new(),
            last_recovery_point_received: None,
            firmware_type: None,
            azure_vm_generation: None,
            is_additional_stats_available: None,
            total_data_transferred: None,
            total_progress_health: None,
            target_vm_tags: None,
            seed_managed_disk_tags: None,
            target_managed_disk_tags: None,
            target_nic_tags: None,
            switch_provider_blocking_error_details: Vec::new(),
            switch_provider_details: None,
        }
    }
}
#[doc = "InMageAzureV2 specific provider input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageAzureV2ReprotectInput {
    #[serde(flatten)]
    pub reverse_replication_provider_specific_input: ReverseReplicationProviderSpecificInput,
    #[doc = "The Master target Id."]
    #[serde(rename = "masterTargetId", default, skip_serializing_if = "Option::is_none")]
    pub master_target_id: Option<String>,
    #[doc = "The Process Server Id."]
    #[serde(rename = "processServerId", default, skip_serializing_if = "Option::is_none")]
    pub process_server_id: Option<String>,
    #[doc = "The storage account id."]
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[doc = "The CS account Id."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "The Policy Id."]
    #[serde(rename = "policyId", default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[doc = "The storage account to be used for logging during replication."]
    #[serde(rename = "logStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub log_storage_account_id: Option<String>,
    #[doc = "The disks to include list."]
    #[serde(rename = "disksToInclude", default, skip_serializing_if = "Vec::is_empty")]
    pub disks_to_include: Vec<String>,
}
impl InMageAzureV2ReprotectInput {
    pub fn new(reverse_replication_provider_specific_input: ReverseReplicationProviderSpecificInput) -> Self {
        Self {
            reverse_replication_provider_specific_input,
            master_target_id: None,
            process_server_id: None,
            storage_account_id: None,
            run_as_account_id: None,
            policy_id: None,
            log_storage_account_id: None,
            disks_to_include: Vec::new(),
        }
    }
}
#[doc = "InMageAzureV2 switch provider blocking error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageAzureV2SwitchProviderBlockingErrorDetails {
    #[doc = "The error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "The error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "The possible causes."]
    #[serde(rename = "possibleCauses", default, skip_serializing_if = "Option::is_none")]
    pub possible_causes: Option<String>,
    #[doc = "The recommended action."]
    #[serde(rename = "recommendedAction", default, skip_serializing_if = "Option::is_none")]
    pub recommended_action: Option<String>,
    #[doc = "The error message parameters."]
    #[serde(rename = "errorMessageParameters", default, skip_serializing_if = "Option::is_none")]
    pub error_message_parameters: Option<serde_json::Value>,
    #[doc = "The error tags."]
    #[serde(rename = "errorTags", default, skip_serializing_if = "Option::is_none")]
    pub error_tags: Option<serde_json::Value>,
}
impl InMageAzureV2SwitchProviderBlockingErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "InMageAzureV2 switch provider details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageAzureV2SwitchProviderDetails {
    #[doc = "The target vault Id."]
    #[serde(rename = "targetVaultId", default, skip_serializing_if = "Option::is_none")]
    pub target_vault_id: Option<String>,
    #[doc = "The target resource Id."]
    #[serde(rename = "targetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_id: Option<String>,
    #[doc = "The target fabric Id."]
    #[serde(rename = "targetFabricId", default, skip_serializing_if = "Option::is_none")]
    pub target_fabric_id: Option<String>,
    #[doc = "The target appliance Id."]
    #[serde(rename = "targetApplianceId", default, skip_serializing_if = "Option::is_none")]
    pub target_appliance_id: Option<String>,
}
impl InMageAzureV2SwitchProviderDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provider specific input for InMageAzureV2 switch provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageAzureV2SwitchProviderProviderInput {
    #[serde(flatten)]
    pub switch_provider_provider_specific_input: SwitchProviderProviderSpecificInput,
    #[doc = "The target vault Id."]
    #[serde(rename = "targetVaultID")]
    pub target_vault_id: String,
    #[doc = "The target fabric Id."]
    #[serde(rename = "targetFabricID")]
    pub target_fabric_id: String,
    #[doc = "The target appliance Id."]
    #[serde(rename = "targetApplianceID")]
    pub target_appliance_id: String,
}
impl InMageAzureV2SwitchProviderProviderInput {
    pub fn new(
        switch_provider_provider_specific_input: SwitchProviderProviderSpecificInput,
        target_vault_id: String,
        target_fabric_id: String,
        target_appliance_id: String,
    ) -> Self {
        Self {
            switch_provider_provider_specific_input,
            target_vault_id,
            target_fabric_id,
            target_appliance_id,
        }
    }
}
#[doc = "InMageAzureV2 provider specific input for test failover."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageAzureV2TestFailoverInput {
    #[serde(flatten)]
    pub test_failover_provider_specific_input: TestFailoverProviderSpecificInput,
    #[doc = "The recovery point id to be passed to test failover to a particular recovery point. In case of latest recovery point, null should be passed."]
    #[serde(rename = "recoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_id: Option<String>,
}
impl InMageAzureV2TestFailoverInput {
    pub fn new(test_failover_provider_specific_input: TestFailoverProviderSpecificInput) -> Self {
        Self {
            test_failover_provider_specific_input,
            recovery_point_id: None,
        }
    }
}
#[doc = "InMageAzureV2 provider specific input for unplanned failover."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageAzureV2UnplannedFailoverInput {
    #[serde(flatten)]
    pub unplanned_failover_provider_specific_input: UnplannedFailoverProviderSpecificInput,
    #[doc = "The recovery point id to be passed to failover to a particular recovery point. In case of latest recovery point, null should be passed."]
    #[serde(rename = "recoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_id: Option<String>,
}
impl InMageAzureV2UnplannedFailoverInput {
    pub fn new(unplanned_failover_provider_specific_input: UnplannedFailoverProviderSpecificInput) -> Self {
        Self {
            unplanned_failover_provider_specific_input,
            recovery_point_id: None,
        }
    }
}
#[doc = "InMage Azure V2 input to update replication protected item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageAzureV2UpdateReplicationProtectedItemInput {
    #[serde(flatten)]
    pub update_replication_protected_item_provider_input: UpdateReplicationProtectedItemProviderInput,
    #[doc = "The recovery Azure resource group Id for classic deployment."]
    #[serde(rename = "recoveryAzureV1ResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_azure_v1_resource_group_id: Option<String>,
    #[doc = "The recovery Azure resource group Id for resource manager deployment."]
    #[serde(rename = "recoveryAzureV2ResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_azure_v2_resource_group_id: Option<String>,
    #[doc = "A value indicating whether managed disks should be used during failover."]
    #[serde(rename = "useManagedDisks", default, skip_serializing_if = "Option::is_none")]
    pub use_managed_disks: Option<String>,
    #[doc = "The target proximity placement group Id."]
    #[serde(rename = "targetProximityPlacementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub target_proximity_placement_group_id: Option<String>,
    #[doc = "The target availability zone."]
    #[serde(rename = "targetAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub target_availability_zone: Option<String>,
    #[doc = "The target VM tags."]
    #[serde(rename = "targetVmTags", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_tags: Option<serde_json::Value>,
    #[doc = "The tags for the target managed disks."]
    #[serde(rename = "targetManagedDiskTags", default, skip_serializing_if = "Option::is_none")]
    pub target_managed_disk_tags: Option<serde_json::Value>,
    #[doc = "The tags for the target NICs."]
    #[serde(rename = "targetNicTags", default, skip_serializing_if = "Option::is_none")]
    pub target_nic_tags: Option<serde_json::Value>,
    #[doc = "The SQL Server license type."]
    #[serde(rename = "sqlServerLicenseType", default, skip_serializing_if = "Option::is_none")]
    pub sql_server_license_type: Option<in_mage_azure_v2_update_replication_protected_item_input::SqlServerLicenseType>,
    #[doc = "The list of disk update properties."]
    #[serde(rename = "vmDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_disks: Vec<UpdateDiskInput>,
}
impl InMageAzureV2UpdateReplicationProtectedItemInput {
    pub fn new(update_replication_protected_item_provider_input: UpdateReplicationProtectedItemProviderInput) -> Self {
        Self {
            update_replication_protected_item_provider_input,
            recovery_azure_v1_resource_group_id: None,
            recovery_azure_v2_resource_group_id: None,
            use_managed_disks: None,
            target_proximity_placement_group_id: None,
            target_availability_zone: None,
            target_vm_tags: None,
            target_managed_disk_tags: None,
            target_nic_tags: None,
            sql_server_license_type: None,
            vm_disks: Vec::new(),
        }
    }
}
pub mod in_mage_azure_v2_update_replication_protected_item_input {
    use super::*;
    #[doc = "The SQL Server license type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SqlServerLicenseType")]
    pub enum SqlServerLicenseType {
        NotSpecified,
        NoLicenseType,
        #[serde(rename = "PAYG")]
        Payg,
        #[serde(rename = "AHUB")]
        Ahub,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SqlServerLicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SqlServerLicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SqlServerLicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("SqlServerLicenseType", 0u32, "NotSpecified"),
                Self::NoLicenseType => serializer.serialize_unit_variant("SqlServerLicenseType", 1u32, "NoLicenseType"),
                Self::Payg => serializer.serialize_unit_variant("SqlServerLicenseType", 2u32, "PAYG"),
                Self::Ahub => serializer.serialize_unit_variant("SqlServerLicenseType", 3u32, "AHUB"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Base class for the policies of providers using InMage replication."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageBasePolicyDetails {
    #[serde(flatten)]
    pub policy_provider_specific_details: PolicyProviderSpecificDetails,
    #[doc = "The recovery point threshold in minutes."]
    #[serde(rename = "recoveryPointThresholdInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_threshold_in_minutes: Option<i32>,
    #[doc = "The duration in minutes until which the recovery points need to be stored."]
    #[serde(rename = "recoveryPointHistory", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_history: Option<i32>,
    #[doc = "The app consistent snapshot frequency in minutes."]
    #[serde(rename = "appConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub app_consistent_frequency_in_minutes: Option<i32>,
    #[doc = "A value indicating whether multi-VM sync has to be enabled."]
    #[serde(rename = "multiVmSyncStatus", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_sync_status: Option<String>,
}
impl InMageBasePolicyDetails {
    pub fn new(policy_provider_specific_details: PolicyProviderSpecificDetails) -> Self {
        Self {
            policy_provider_specific_details,
            recovery_point_threshold_in_minutes: None,
            recovery_point_history: None,
            app_consistent_frequency_in_minutes: None,
            multi_vm_sync_status: None,
        }
    }
}
#[doc = "InMage disable protection provider specific input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageDisableProtectionProviderSpecificInput {
    #[serde(flatten)]
    pub disable_protection_provider_specific_input: DisableProtectionProviderSpecificInput,
    #[doc = "A value indicating whether the replica VM should be destroyed or retained. Values from Delete and Retain."]
    #[serde(rename = "replicaVmDeletionStatus", default, skip_serializing_if = "Option::is_none")]
    pub replica_vm_deletion_status: Option<String>,
}
impl InMageDisableProtectionProviderSpecificInput {
    pub fn new(disable_protection_provider_specific_input: DisableProtectionProviderSpecificInput) -> Self {
        Self {
            disable_protection_provider_specific_input,
            replica_vm_deletion_status: None,
        }
    }
}
#[doc = "VMware/Physical specific Disk Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageDiskDetails {
    #[doc = "The disk Id."]
    #[serde(rename = "diskId", default, skip_serializing_if = "Option::is_none")]
    pub disk_id: Option<String>,
    #[doc = "The disk name."]
    #[serde(rename = "diskName", default, skip_serializing_if = "Option::is_none")]
    pub disk_name: Option<String>,
    #[doc = "The disk size in MB."]
    #[serde(rename = "diskSizeInMB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_in_mb: Option<String>,
    #[doc = "Whether disk is system disk or data disk."]
    #[serde(rename = "diskType", default, skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<String>,
    #[doc = "Whether disk is dynamic disk or basic disk."]
    #[serde(rename = "diskConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub disk_configuration: Option<String>,
    #[doc = "Volumes of the disk."]
    #[serde(rename = "volumeList", default, skip_serializing_if = "Vec::is_empty")]
    pub volume_list: Vec<DiskVolumeDetails>,
}
impl InMageDiskDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DiskExclusionInput when doing enable protection of virtual machine in InMage provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageDiskExclusionInput {
    #[doc = "The volume label based option for disk exclusion."]
    #[serde(rename = "volumeOptions", default, skip_serializing_if = "Vec::is_empty")]
    pub volume_options: Vec<InMageVolumeExclusionOptions>,
    #[doc = "The guest disk signature based option for disk exclusion."]
    #[serde(rename = "diskSignatureOptions", default, skip_serializing_if = "Vec::is_empty")]
    pub disk_signature_options: Vec<InMageDiskSignatureExclusionOptions>,
}
impl InMageDiskExclusionInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Guest disk signature based disk exclusion option when doing enable protection of virtual machine in InMage provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageDiskSignatureExclusionOptions {
    #[doc = "The guest signature of disk to be excluded from replication."]
    #[serde(rename = "diskSignature", default, skip_serializing_if = "Option::is_none")]
    pub disk_signature: Option<String>,
}
impl InMageDiskSignatureExclusionOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "VMware Azure specific enable protection input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageEnableProtectionInput {
    #[serde(flatten)]
    pub enable_protection_provider_specific_input: EnableProtectionProviderSpecificInput,
    #[doc = "The VM Name."]
    #[serde(rename = "vmFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub vm_friendly_name: Option<String>,
    #[doc = "The Master Target Id."]
    #[serde(rename = "masterTargetId")]
    pub master_target_id: String,
    #[doc = "The Process Server Id."]
    #[serde(rename = "processServerId")]
    pub process_server_id: String,
    #[doc = "The retention drive to use on the MT."]
    #[serde(rename = "retentionDrive")]
    pub retention_drive: String,
    #[doc = "The CS account Id."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "The multi VM group Id."]
    #[serde(rename = "multiVmGroupId")]
    pub multi_vm_group_id: String,
    #[doc = "The multi VM group name."]
    #[serde(rename = "multiVmGroupName")]
    pub multi_vm_group_name: String,
    #[doc = "The target datastore name."]
    #[serde(rename = "datastoreName", default, skip_serializing_if = "Option::is_none")]
    pub datastore_name: Option<String>,
    #[doc = "DiskExclusionInput when doing enable protection of virtual machine in InMage provider."]
    #[serde(rename = "diskExclusionInput", default, skip_serializing_if = "Option::is_none")]
    pub disk_exclusion_input: Option<InMageDiskExclusionInput>,
    #[doc = "The disks to include list."]
    #[serde(rename = "disksToInclude", default, skip_serializing_if = "Vec::is_empty")]
    pub disks_to_include: Vec<String>,
}
impl InMageEnableProtectionInput {
    pub fn new(
        enable_protection_provider_specific_input: EnableProtectionProviderSpecificInput,
        master_target_id: String,
        process_server_id: String,
        retention_drive: String,
        multi_vm_group_id: String,
        multi_vm_group_name: String,
    ) -> Self {
        Self {
            enable_protection_provider_specific_input,
            vm_friendly_name: None,
            master_target_id,
            process_server_id,
            retention_drive,
            run_as_account_id: None,
            multi_vm_group_id,
            multi_vm_group_name,
            datastore_name: None,
            disk_exclusion_input: None,
            disks_to_include: Vec::new(),
        }
    }
}
#[doc = "InMageFabric switch provider blocking error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageFabricSwitchProviderBlockingErrorDetails {
    #[doc = "The error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "The error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "The possible causes."]
    #[serde(rename = "possibleCauses", default, skip_serializing_if = "Option::is_none")]
    pub possible_causes: Option<String>,
    #[doc = "The recommended action."]
    #[serde(rename = "recommendedAction", default, skip_serializing_if = "Option::is_none")]
    pub recommended_action: Option<String>,
    #[doc = "The error message parameters."]
    #[serde(rename = "errorMessageParameters", default, skip_serializing_if = "Option::is_none")]
    pub error_message_parameters: Option<serde_json::Value>,
    #[doc = "The error tags."]
    #[serde(rename = "errorTags", default, skip_serializing_if = "Option::is_none")]
    pub error_tags: Option<serde_json::Value>,
}
impl InMageFabricSwitchProviderBlockingErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "InMage specific protection profile details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMagePolicyDetails {
    #[serde(flatten)]
    pub policy_provider_specific_details: PolicyProviderSpecificDetails,
    #[doc = "The recovery point threshold in minutes."]
    #[serde(rename = "recoveryPointThresholdInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_threshold_in_minutes: Option<i32>,
    #[doc = "The duration in minutes until which the recovery points need to be stored."]
    #[serde(rename = "recoveryPointHistory", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_history: Option<i32>,
    #[doc = "The app consistent snapshot frequency in minutes."]
    #[serde(rename = "appConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub app_consistent_frequency_in_minutes: Option<i32>,
    #[doc = "A value indicating whether multi-VM sync has to be enabled."]
    #[serde(rename = "multiVmSyncStatus", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_sync_status: Option<String>,
}
impl InMagePolicyDetails {
    pub fn new(policy_provider_specific_details: PolicyProviderSpecificDetails) -> Self {
        Self {
            policy_provider_specific_details,
            recovery_point_threshold_in_minutes: None,
            recovery_point_history: None,
            app_consistent_frequency_in_minutes: None,
            multi_vm_sync_status: None,
        }
    }
}
#[doc = "VMWare Azure specific protection profile Input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMagePolicyInput {
    #[serde(flatten)]
    pub policy_provider_specific_input: PolicyProviderSpecificInput,
    #[doc = "The recovery point threshold in minutes."]
    #[serde(rename = "recoveryPointThresholdInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_threshold_in_minutes: Option<i32>,
    #[doc = "The duration in minutes until which the recovery points need to be stored."]
    #[serde(rename = "recoveryPointHistory", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_history: Option<i32>,
    #[doc = "The app consistent snapshot frequency (in minutes)."]
    #[serde(rename = "appConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub app_consistent_frequency_in_minutes: Option<i32>,
    #[doc = "A value indicating whether multi-VM sync has to be enabled. Value should be 'Enabled' or 'Disabled'."]
    #[serde(rename = "multiVmSyncStatus")]
    pub multi_vm_sync_status: in_mage_policy_input::MultiVmSyncStatus,
}
impl InMagePolicyInput {
    pub fn new(
        policy_provider_specific_input: PolicyProviderSpecificInput,
        multi_vm_sync_status: in_mage_policy_input::MultiVmSyncStatus,
    ) -> Self {
        Self {
            policy_provider_specific_input,
            recovery_point_threshold_in_minutes: None,
            recovery_point_history: None,
            app_consistent_frequency_in_minutes: None,
            multi_vm_sync_status,
        }
    }
}
pub mod in_mage_policy_input {
    use super::*;
    #[doc = "A value indicating whether multi-VM sync has to be enabled. Value should be 'Enabled' or 'Disabled'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MultiVmSyncStatus")]
    pub enum MultiVmSyncStatus {
        Enable,
        Disable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MultiVmSyncStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MultiVmSyncStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MultiVmSyncStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enable => serializer.serialize_unit_variant("MultiVmSyncStatus", 0u32, "Enable"),
                Self::Disable => serializer.serialize_unit_variant("MultiVmSyncStatus", 1u32, "Disable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "InMage protected disk details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageProtectedDiskDetails {
    #[doc = "The disk id."]
    #[serde(rename = "diskId", default, skip_serializing_if = "Option::is_none")]
    pub disk_id: Option<String>,
    #[doc = "The disk name."]
    #[serde(rename = "diskName", default, skip_serializing_if = "Option::is_none")]
    pub disk_name: Option<String>,
    #[doc = "The protection stage."]
    #[serde(rename = "protectionStage", default, skip_serializing_if = "Option::is_none")]
    pub protection_stage: Option<String>,
    #[doc = "The health error code for the disk."]
    #[serde(rename = "healthErrorCode", default, skip_serializing_if = "Option::is_none")]
    pub health_error_code: Option<String>,
    #[doc = "The RPO in seconds."]
    #[serde(rename = "rpoInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub rpo_in_seconds: Option<i64>,
    #[doc = "A value indicating whether resync is required for this disk."]
    #[serde(rename = "resyncRequired", default, skip_serializing_if = "Option::is_none")]
    pub resync_required: Option<String>,
    #[doc = "The resync progress percentage."]
    #[serde(rename = "resyncProgressPercentage", default, skip_serializing_if = "Option::is_none")]
    pub resync_progress_percentage: Option<i32>,
    #[doc = "The resync duration in seconds."]
    #[serde(rename = "resyncDurationInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub resync_duration_in_seconds: Option<i64>,
    #[doc = "The disk capacity in bytes."]
    #[serde(rename = "diskCapacityInBytes", default, skip_serializing_if = "Option::is_none")]
    pub disk_capacity_in_bytes: Option<i64>,
    #[doc = "The file system capacity in bytes."]
    #[serde(rename = "fileSystemCapacityInBytes", default, skip_serializing_if = "Option::is_none")]
    pub file_system_capacity_in_bytes: Option<i64>,
    #[doc = "The source data transit in MB."]
    #[serde(rename = "sourceDataInMB", default, skip_serializing_if = "Option::is_none")]
    pub source_data_in_mb: Option<f64>,
    #[doc = "The PS data transit in MB."]
    #[serde(rename = "psDataInMB", default, skip_serializing_if = "Option::is_none")]
    pub ps_data_in_mb: Option<f64>,
    #[doc = "The target data transit in MB."]
    #[serde(rename = "targetDataInMB", default, skip_serializing_if = "Option::is_none")]
    pub target_data_in_mb: Option<f64>,
    #[doc = "A value indicating whether disk is resized."]
    #[serde(rename = "diskResized", default, skip_serializing_if = "Option::is_none")]
    pub disk_resized: Option<String>,
    #[doc = "The last RPO calculated time."]
    #[serde(rename = "lastRpoCalculatedTime", with = "azure_core::date::rfc3339::option")]
    pub last_rpo_calculated_time: Option<time::OffsetDateTime>,
    #[doc = "The resync processed bytes."]
    #[serde(rename = "resyncProcessedBytes", default, skip_serializing_if = "Option::is_none")]
    pub resync_processed_bytes: Option<i64>,
    #[doc = "The resync total transferred bytes."]
    #[serde(rename = "resyncTotalTransferredBytes", default, skip_serializing_if = "Option::is_none")]
    pub resync_total_transferred_bytes: Option<i64>,
    #[doc = "The resync last 15 minutes transferred bytes."]
    #[serde(rename = "resyncLast15MinutesTransferredBytes", default, skip_serializing_if = "Option::is_none")]
    pub resync_last15_minutes_transferred_bytes: Option<i64>,
    #[doc = "The last data transfer time in UTC."]
    #[serde(rename = "resyncLastDataTransferTimeUTC", with = "azure_core::date::rfc3339::option")]
    pub resync_last_data_transfer_time_utc: Option<time::OffsetDateTime>,
    #[doc = "The resync start time."]
    #[serde(rename = "resyncStartTime", with = "azure_core::date::rfc3339::option")]
    pub resync_start_time: Option<time::OffsetDateTime>,
    #[doc = "The Progress Health."]
    #[serde(rename = "progressHealth", default, skip_serializing_if = "Option::is_none")]
    pub progress_health: Option<String>,
    #[doc = "The Progress Status."]
    #[serde(rename = "progressStatus", default, skip_serializing_if = "Option::is_none")]
    pub progress_status: Option<String>,
}
impl InMageProtectedDiskDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "InMageRcm source agent upgrade blocking error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageRcmAgentUpgradeBlockingErrorDetails {
    #[doc = "The error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "The error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "The possible causes."]
    #[serde(rename = "possibleCauses", default, skip_serializing_if = "Option::is_none")]
    pub possible_causes: Option<String>,
    #[doc = "The recommended action."]
    #[serde(rename = "recommendedAction", default, skip_serializing_if = "Option::is_none")]
    pub recommended_action: Option<String>,
    #[doc = "The error message parameters."]
    #[serde(rename = "errorMessageParameters", default, skip_serializing_if = "Option::is_none")]
    pub error_message_parameters: Option<serde_json::Value>,
    #[doc = "The error tags."]
    #[serde(rename = "errorTags", default, skip_serializing_if = "Option::is_none")]
    pub error_tags: Option<serde_json::Value>,
}
impl InMageRcmAgentUpgradeBlockingErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "InMageRcm appliance details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageRcmApplianceDetails {
    #[doc = "The appliance Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The appliance name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The fabric ARM Id."]
    #[serde(rename = "fabricArmId", default, skip_serializing_if = "Option::is_none")]
    pub fabric_arm_id: Option<String>,
    #[doc = "Process server details."]
    #[serde(rename = "processServer", default, skip_serializing_if = "Option::is_none")]
    pub process_server: Option<ProcessServerDetails>,
    #[doc = "RCM proxy details."]
    #[serde(rename = "rcmProxy", default, skip_serializing_if = "Option::is_none")]
    pub rcm_proxy: Option<RcmProxyDetails>,
    #[doc = "Push installer details."]
    #[serde(rename = "pushInstaller", default, skip_serializing_if = "Option::is_none")]
    pub push_installer: Option<PushInstallerDetails>,
    #[doc = "Replication agent details."]
    #[serde(rename = "replicationAgent", default, skip_serializing_if = "Option::is_none")]
    pub replication_agent: Option<ReplicationAgentDetails>,
    #[doc = "Reprotect agent details."]
    #[serde(rename = "reprotectAgent", default, skip_serializing_if = "Option::is_none")]
    pub reprotect_agent: Option<ReprotectAgentDetails>,
    #[doc = "Mars agent details."]
    #[serde(rename = "marsAgent", default, skip_serializing_if = "Option::is_none")]
    pub mars_agent: Option<MarsAgentDetails>,
    #[doc = "DRA details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dra: Option<DraDetails>,
    #[doc = "The switch provider blocking error information."]
    #[serde(rename = "switchProviderBlockingErrorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub switch_provider_blocking_error_details: Vec<InMageRcmFabricSwitchProviderBlockingErrorDetails>,
}
impl InMageRcmApplianceDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "InMageRcm appliance specific details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmApplianceSpecificDetails {
    #[serde(flatten)]
    pub appliance_specific_details: ApplianceSpecificDetails,
    #[doc = "The list of appliances."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub appliances: Vec<InMageRcmApplianceDetails>,
}
impl InMageRcmApplianceSpecificDetails {
    pub fn new(appliance_specific_details: ApplianceSpecificDetails) -> Self {
        Self {
            appliance_specific_details,
            appliances: Vec::new(),
        }
    }
}
#[doc = "ApplyRecoveryPoint input specific to InMageRcm provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmApplyRecoveryPointInput {
    #[serde(flatten)]
    pub apply_recovery_point_provider_specific_input: ApplyRecoveryPointProviderSpecificInput,
    #[doc = "The recovery point Id."]
    #[serde(rename = "recoveryPointId")]
    pub recovery_point_id: String,
}
impl InMageRcmApplyRecoveryPointInput {
    pub fn new(apply_recovery_point_provider_specific_input: ApplyRecoveryPointProviderSpecificInput, recovery_point_id: String) -> Self {
        Self {
            apply_recovery_point_provider_specific_input,
            recovery_point_id,
        }
    }
}
#[doc = "InMageRcm discovered protected VM details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageRcmDiscoveredProtectedVmDetails {
    #[doc = "The VCenter Id."]
    #[serde(rename = "vCenterId", default, skip_serializing_if = "Option::is_none")]
    pub v_center_id: Option<String>,
    #[doc = "The VCenter fqdn."]
    #[serde(rename = "vCenterFqdn", default, skip_serializing_if = "Option::is_none")]
    pub v_center_fqdn: Option<String>,
    #[doc = "The list of datastores."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub datastores: Vec<String>,
    #[doc = "The list of IP addresses."]
    #[serde(rename = "ipAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_addresses: Vec<String>,
    #[doc = "The VMware tools status."]
    #[serde(rename = "vmwareToolsStatus", default, skip_serializing_if = "Option::is_none")]
    pub vmware_tools_status: Option<String>,
    #[doc = "The VM power status."]
    #[serde(rename = "powerStatus", default, skip_serializing_if = "Option::is_none")]
    pub power_status: Option<String>,
    #[doc = "The VM fqdn."]
    #[serde(rename = "vmFqdn", default, skip_serializing_if = "Option::is_none")]
    pub vm_fqdn: Option<String>,
    #[doc = "The VM's OS name."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "The SDS created timestamp."]
    #[serde(rename = "createdTimestamp", with = "azure_core::date::rfc3339::option")]
    pub created_timestamp: Option<time::OffsetDateTime>,
    #[doc = "The SDS updated timestamp."]
    #[serde(rename = "updatedTimestamp", with = "azure_core::date::rfc3339::option")]
    pub updated_timestamp: Option<time::OffsetDateTime>,
    #[doc = "A value indicating whether the VM is deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "The last time when SDS information discovered in SRS."]
    #[serde(rename = "lastDiscoveryTimeInUtc", with = "azure_core::date::rfc3339::option")]
    pub last_discovery_time_in_utc: Option<time::OffsetDateTime>,
}
impl InMageRcmDiscoveredProtectedVmDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "InMageRcm disk input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmDiskInput {
    #[doc = "The disk Id."]
    #[serde(rename = "diskId")]
    pub disk_id: String,
    #[doc = "The log storage account ARM Id."]
    #[serde(rename = "logStorageAccountId")]
    pub log_storage_account_id: String,
    #[doc = "The disk type."]
    #[serde(rename = "diskType")]
    pub disk_type: in_mage_rcm_disk_input::DiskType,
    #[doc = "The DiskEncryptionSet ARM Id."]
    #[serde(rename = "diskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set_id: Option<String>,
}
impl InMageRcmDiskInput {
    pub fn new(disk_id: String, log_storage_account_id: String, disk_type: in_mage_rcm_disk_input::DiskType) -> Self {
        Self {
            disk_id,
            log_storage_account_id,
            disk_type,
            disk_encryption_set_id: None,
        }
    }
}
pub mod in_mage_rcm_disk_input {
    use super::*;
    #[doc = "The disk type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DiskType")]
    pub enum DiskType {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(rename = "StandardSSD_LRS")]
        StandardSsdLrs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DiskType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DiskType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DiskType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::StandardLrs => serializer.serialize_unit_variant("DiskType", 0u32, "Standard_LRS"),
                Self::PremiumLrs => serializer.serialize_unit_variant("DiskType", 1u32, "Premium_LRS"),
                Self::StandardSsdLrs => serializer.serialize_unit_variant("DiskType", 2u32, "StandardSSD_LRS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "InMageRcm disk input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmDisksDefaultInput {
    #[doc = "The log storage account ARM Id."]
    #[serde(rename = "logStorageAccountId")]
    pub log_storage_account_id: String,
    #[doc = "The disk type."]
    #[serde(rename = "diskType")]
    pub disk_type: in_mage_rcm_disks_default_input::DiskType,
    #[doc = "The DiskEncryptionSet ARM Id."]
    #[serde(rename = "diskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set_id: Option<String>,
}
impl InMageRcmDisksDefaultInput {
    pub fn new(log_storage_account_id: String, disk_type: in_mage_rcm_disks_default_input::DiskType) -> Self {
        Self {
            log_storage_account_id,
            disk_type,
            disk_encryption_set_id: None,
        }
    }
}
pub mod in_mage_rcm_disks_default_input {
    use super::*;
    #[doc = "The disk type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DiskType")]
    pub enum DiskType {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(rename = "StandardSSD_LRS")]
        StandardSsdLrs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DiskType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DiskType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DiskType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::StandardLrs => serializer.serialize_unit_variant("DiskType", 0u32, "Standard_LRS"),
                Self::PremiumLrs => serializer.serialize_unit_variant("DiskType", 1u32, "Premium_LRS"),
                Self::StandardSsdLrs => serializer.serialize_unit_variant("DiskType", 2u32, "StandardSSD_LRS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "InMageRcm specific enable protection input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmEnableProtectionInput {
    #[serde(flatten)]
    pub enable_protection_provider_specific_input: EnableProtectionProviderSpecificInput,
    #[doc = "The ARM Id of discovered machine."]
    #[serde(rename = "fabricDiscoveryMachineId")]
    pub fabric_discovery_machine_id: String,
    #[doc = "The disks to include list."]
    #[serde(rename = "disksToInclude", default, skip_serializing_if = "Vec::is_empty")]
    pub disks_to_include: Vec<InMageRcmDiskInput>,
    #[doc = "InMageRcm disk input."]
    #[serde(rename = "disksDefault", default, skip_serializing_if = "Option::is_none")]
    pub disks_default: Option<InMageRcmDisksDefaultInput>,
    #[doc = "The target resource group ARM Id."]
    #[serde(rename = "targetResourceGroupId")]
    pub target_resource_group_id: String,
    #[doc = "The selected target network ARM Id."]
    #[serde(rename = "targetNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub target_network_id: Option<String>,
    #[doc = "The selected test network ARM Id."]
    #[serde(rename = "testNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub test_network_id: Option<String>,
    #[doc = "The selected target subnet name."]
    #[serde(rename = "targetSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub target_subnet_name: Option<String>,
    #[doc = "The selected test subnet name."]
    #[serde(rename = "testSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub test_subnet_name: Option<String>,
    #[doc = "The target VM name."]
    #[serde(rename = "targetVmName", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_name: Option<String>,
    #[doc = "The target VM size."]
    #[serde(rename = "targetVmSize", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_size: Option<String>,
    #[doc = "The license type."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<in_mage_rcm_enable_protection_input::LicenseType>,
    #[doc = "The target availability set ARM Id."]
    #[serde(rename = "targetAvailabilitySetId", default, skip_serializing_if = "Option::is_none")]
    pub target_availability_set_id: Option<String>,
    #[doc = "The target availability zone."]
    #[serde(rename = "targetAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub target_availability_zone: Option<String>,
    #[doc = "The target proximity placement group Id."]
    #[serde(rename = "targetProximityPlacementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub target_proximity_placement_group_id: Option<String>,
    #[doc = "The target boot diagnostics storage account ARM Id."]
    #[serde(rename = "targetBootDiagnosticsStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub target_boot_diagnostics_storage_account_id: Option<String>,
    #[doc = "The run-as account Id."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "The process server Id."]
    #[serde(rename = "processServerId")]
    pub process_server_id: String,
    #[doc = "The multi VM group name."]
    #[serde(rename = "multiVmGroupName", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_group_name: Option<String>,
}
impl InMageRcmEnableProtectionInput {
    pub fn new(
        enable_protection_provider_specific_input: EnableProtectionProviderSpecificInput,
        fabric_discovery_machine_id: String,
        target_resource_group_id: String,
        process_server_id: String,
    ) -> Self {
        Self {
            enable_protection_provider_specific_input,
            fabric_discovery_machine_id,
            disks_to_include: Vec::new(),
            disks_default: None,
            target_resource_group_id,
            target_network_id: None,
            test_network_id: None,
            target_subnet_name: None,
            test_subnet_name: None,
            target_vm_name: None,
            target_vm_size: None,
            license_type: None,
            target_availability_set_id: None,
            target_availability_zone: None,
            target_proximity_placement_group_id: None,
            target_boot_diagnostics_storage_account_id: None,
            run_as_account_id: None,
            process_server_id,
            multi_vm_group_name: None,
        }
    }
}
pub mod in_mage_rcm_enable_protection_input {
    use super::*;
    #[doc = "The license type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseType")]
    pub enum LicenseType {
        NotSpecified,
        NoLicenseType,
        WindowsServer,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("LicenseType", 0u32, "NotSpecified"),
                Self::NoLicenseType => serializer.serialize_unit_variant("LicenseType", 1u32, "NoLicenseType"),
                Self::WindowsServer => serializer.serialize_unit_variant("LicenseType", 2u32, "WindowsServer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Event details for InMageRcm provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmEventDetails {
    #[serde(flatten)]
    pub event_provider_specific_details: EventProviderSpecificDetails,
    #[doc = "The protected item name."]
    #[serde(rename = "protectedItemName", default, skip_serializing_if = "Option::is_none")]
    pub protected_item_name: Option<String>,
    #[doc = "The protected item name."]
    #[serde(rename = "vmName", default, skip_serializing_if = "Option::is_none")]
    pub vm_name: Option<String>,
    #[doc = "The latest agent version."]
    #[serde(rename = "latestAgentVersion", default, skip_serializing_if = "Option::is_none")]
    pub latest_agent_version: Option<String>,
    #[doc = "The job Id."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "The fabric name."]
    #[serde(rename = "fabricName", default, skip_serializing_if = "Option::is_none")]
    pub fabric_name: Option<String>,
    #[doc = "The appliance name."]
    #[serde(rename = "applianceName", default, skip_serializing_if = "Option::is_none")]
    pub appliance_name: Option<String>,
    #[doc = "The server type."]
    #[serde(rename = "serverType", default, skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[doc = "The component display name."]
    #[serde(rename = "componentDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub component_display_name: Option<String>,
}
impl InMageRcmEventDetails {
    pub fn new(event_provider_specific_details: EventProviderSpecificDetails) -> Self {
        Self {
            event_provider_specific_details,
            protected_item_name: None,
            vm_name: None,
            latest_agent_version: None,
            job_id: None,
            fabric_name: None,
            appliance_name: None,
            server_type: None,
            component_display_name: None,
        }
    }
}
#[doc = "InMageRcm fabric provider specific settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmFabricCreationInput {
    #[serde(flatten)]
    pub fabric_specific_creation_input: FabricSpecificCreationInput,
    #[doc = "The ARM Id of the VMware site."]
    #[serde(rename = "vmwareSiteId")]
    pub vmware_site_id: String,
    #[doc = "The ARM Id of the physical site."]
    #[serde(rename = "physicalSiteId")]
    pub physical_site_id: String,
    #[doc = "Identity provider input."]
    #[serde(rename = "sourceAgentIdentity")]
    pub source_agent_identity: IdentityProviderInput,
}
impl InMageRcmFabricCreationInput {
    pub fn new(
        fabric_specific_creation_input: FabricSpecificCreationInput,
        vmware_site_id: String,
        physical_site_id: String,
        source_agent_identity: IdentityProviderInput,
    ) -> Self {
        Self {
            fabric_specific_creation_input,
            vmware_site_id,
            physical_site_id,
            source_agent_identity,
        }
    }
}
#[doc = "InMageRcm fabric specific details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmFabricSpecificDetails {
    #[serde(flatten)]
    pub fabric_specific_details: FabricSpecificDetails,
    #[doc = "The ARM Id of the VMware site."]
    #[serde(rename = "vmwareSiteId", default, skip_serializing_if = "Option::is_none")]
    pub vmware_site_id: Option<String>,
    #[doc = "The ARM Id of the physical site."]
    #[serde(rename = "physicalSiteId", default, skip_serializing_if = "Option::is_none")]
    pub physical_site_id: Option<String>,
    #[doc = "The service endpoint."]
    #[serde(rename = "serviceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub service_endpoint: Option<String>,
    #[doc = "The service resource Id."]
    #[serde(rename = "serviceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub service_resource_id: Option<String>,
    #[doc = "The service container Id."]
    #[serde(rename = "serviceContainerId", default, skip_serializing_if = "Option::is_none")]
    pub service_container_id: Option<String>,
    #[doc = "The data plane Uri."]
    #[serde(rename = "dataPlaneUri", default, skip_serializing_if = "Option::is_none")]
    pub data_plane_uri: Option<String>,
    #[doc = "The control plane Uri."]
    #[serde(rename = "controlPlaneUri", default, skip_serializing_if = "Option::is_none")]
    pub control_plane_uri: Option<String>,
    #[doc = "Identity provider details."]
    #[serde(rename = "sourceAgentIdentityDetails", default, skip_serializing_if = "Option::is_none")]
    pub source_agent_identity_details: Option<IdentityProviderDetails>,
    #[doc = "The list of process servers."]
    #[serde(rename = "processServers", default, skip_serializing_if = "Vec::is_empty")]
    pub process_servers: Vec<ProcessServerDetails>,
    #[doc = "The list of RCM proxies."]
    #[serde(rename = "rcmProxies", default, skip_serializing_if = "Vec::is_empty")]
    pub rcm_proxies: Vec<RcmProxyDetails>,
    #[doc = "The list of push installers."]
    #[serde(rename = "pushInstallers", default, skip_serializing_if = "Vec::is_empty")]
    pub push_installers: Vec<PushInstallerDetails>,
    #[doc = "The list of replication agents."]
    #[serde(rename = "replicationAgents", default, skip_serializing_if = "Vec::is_empty")]
    pub replication_agents: Vec<ReplicationAgentDetails>,
    #[doc = "The list of reprotect agents."]
    #[serde(rename = "reprotectAgents", default, skip_serializing_if = "Vec::is_empty")]
    pub reprotect_agents: Vec<ReprotectAgentDetails>,
    #[doc = "The list of Mars agents."]
    #[serde(rename = "marsAgents", default, skip_serializing_if = "Vec::is_empty")]
    pub mars_agents: Vec<MarsAgentDetails>,
    #[doc = "The list of DRAs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dras: Vec<DraDetails>,
    #[doc = "The list of agent details."]
    #[serde(rename = "agentDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub agent_details: Vec<AgentDetails>,
}
impl InMageRcmFabricSpecificDetails {
    pub fn new(fabric_specific_details: FabricSpecificDetails) -> Self {
        Self {
            fabric_specific_details,
            vmware_site_id: None,
            physical_site_id: None,
            service_endpoint: None,
            service_resource_id: None,
            service_container_id: None,
            data_plane_uri: None,
            control_plane_uri: None,
            source_agent_identity_details: None,
            process_servers: Vec::new(),
            rcm_proxies: Vec::new(),
            push_installers: Vec::new(),
            replication_agents: Vec::new(),
            reprotect_agents: Vec::new(),
            mars_agents: Vec::new(),
            dras: Vec::new(),
            agent_details: Vec::new(),
        }
    }
}
#[doc = "InMageRcmFabric switch provider blocking error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageRcmFabricSwitchProviderBlockingErrorDetails {
    #[doc = "The error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "The error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "The possible causes."]
    #[serde(rename = "possibleCauses", default, skip_serializing_if = "Option::is_none")]
    pub possible_causes: Option<String>,
    #[doc = "The recommended action."]
    #[serde(rename = "recommendedAction", default, skip_serializing_if = "Option::is_none")]
    pub recommended_action: Option<String>,
    #[doc = "The error message parameters."]
    #[serde(rename = "errorMessageParameters", default, skip_serializing_if = "Option::is_none")]
    pub error_message_parameters: Option<serde_json::Value>,
    #[doc = "The error tags."]
    #[serde(rename = "errorTags", default, skip_serializing_if = "Option::is_none")]
    pub error_tags: Option<serde_json::Value>,
}
impl InMageRcmFabricSwitchProviderBlockingErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "InMageRcmFailback discovered VM details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageRcmFailbackDiscoveredProtectedVmDetails {
    #[doc = "The VCenter Id."]
    #[serde(rename = "vCenterId", default, skip_serializing_if = "Option::is_none")]
    pub v_center_id: Option<String>,
    #[doc = "The VCenter fqdn."]
    #[serde(rename = "vCenterFqdn", default, skip_serializing_if = "Option::is_none")]
    pub v_center_fqdn: Option<String>,
    #[doc = "The list of datastores."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub datastores: Vec<String>,
    #[doc = "The list of IP addresses."]
    #[serde(rename = "ipAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_addresses: Vec<String>,
    #[doc = "The VMware tools status."]
    #[serde(rename = "vmwareToolsStatus", default, skip_serializing_if = "Option::is_none")]
    pub vmware_tools_status: Option<String>,
    #[doc = "The VM power status."]
    #[serde(rename = "powerStatus", default, skip_serializing_if = "Option::is_none")]
    pub power_status: Option<String>,
    #[doc = "The VM fqdn."]
    #[serde(rename = "vmFqdn", default, skip_serializing_if = "Option::is_none")]
    pub vm_fqdn: Option<String>,
    #[doc = "The VM's OS name."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "The SDS created timestamp."]
    #[serde(rename = "createdTimestamp", with = "azure_core::date::rfc3339::option")]
    pub created_timestamp: Option<time::OffsetDateTime>,
    #[doc = "The SDS updated timestamp."]
    #[serde(rename = "updatedTimestamp", with = "azure_core::date::rfc3339::option")]
    pub updated_timestamp: Option<time::OffsetDateTime>,
    #[doc = "A value indicating whether the VM is deleted."]
    #[serde(rename = "isDeleted", default, skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    #[doc = "The last time when SDS information discovered in SRS."]
    #[serde(rename = "lastDiscoveryTimeInUtc", with = "azure_core::date::rfc3339::option")]
    pub last_discovery_time_in_utc: Option<time::OffsetDateTime>,
}
impl InMageRcmFailbackDiscoveredProtectedVmDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Event details for InMageRcmFailback provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmFailbackEventDetails {
    #[serde(flatten)]
    pub event_provider_specific_details: EventProviderSpecificDetails,
    #[doc = "The protected item name."]
    #[serde(rename = "protectedItemName", default, skip_serializing_if = "Option::is_none")]
    pub protected_item_name: Option<String>,
    #[doc = "The protected item name."]
    #[serde(rename = "vmName", default, skip_serializing_if = "Option::is_none")]
    pub vm_name: Option<String>,
    #[doc = "The appliance name."]
    #[serde(rename = "applianceName", default, skip_serializing_if = "Option::is_none")]
    pub appliance_name: Option<String>,
    #[doc = "The server type."]
    #[serde(rename = "serverType", default, skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[doc = "The component display name."]
    #[serde(rename = "componentDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub component_display_name: Option<String>,
}
impl InMageRcmFailbackEventDetails {
    pub fn new(event_provider_specific_details: EventProviderSpecificDetails) -> Self {
        Self {
            event_provider_specific_details,
            protected_item_name: None,
            vm_name: None,
            appliance_name: None,
            server_type: None,
            component_display_name: None,
        }
    }
}
#[doc = "InMageRcmFailback mobility agent details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageRcmFailbackMobilityAgentDetails {
    #[doc = "The agent version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The latest agent version available."]
    #[serde(rename = "latestVersion", default, skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[doc = "The driver version."]
    #[serde(rename = "driverVersion", default, skip_serializing_if = "Option::is_none")]
    pub driver_version: Option<String>,
    #[doc = "The latest upgradeable version available without reboot."]
    #[serde(rename = "latestUpgradableVersionWithoutReboot", default, skip_serializing_if = "Option::is_none")]
    pub latest_upgradable_version_without_reboot: Option<String>,
    #[doc = "The agent version expiry date."]
    #[serde(rename = "agentVersionExpiryDate", with = "azure_core::date::rfc3339::option")]
    pub agent_version_expiry_date: Option<time::OffsetDateTime>,
    #[doc = "The driver version expiry date."]
    #[serde(rename = "driverVersionExpiryDate", with = "azure_core::date::rfc3339::option")]
    pub driver_version_expiry_date: Option<time::OffsetDateTime>,
    #[doc = "The time of the last heartbeat received from the agent."]
    #[serde(rename = "lastHeartbeatUtc", with = "azure_core::date::rfc3339::option")]
    pub last_heartbeat_utc: Option<time::OffsetDateTime>,
    #[doc = "The whether update is possible or not."]
    #[serde(rename = "reasonsBlockingUpgrade", default, skip_serializing_if = "Vec::is_empty")]
    pub reasons_blocking_upgrade: Vec<String>,
    #[doc = "A value indicating whether agent is upgradeable or not."]
    #[serde(rename = "isUpgradeable", default, skip_serializing_if = "Option::is_none")]
    pub is_upgradeable: Option<String>,
}
impl InMageRcmFailbackMobilityAgentDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "InMageRcmFailback NIC details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageRcmFailbackNicDetails {
    #[doc = "The mac address."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "The network name."]
    #[serde(rename = "networkName", default, skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    #[doc = "The adapter type."]
    #[serde(rename = "adapterType", default, skip_serializing_if = "Option::is_none")]
    pub adapter_type: Option<String>,
    #[doc = "The IP address."]
    #[serde(rename = "sourceIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub source_ip_address: Option<String>,
}
impl InMageRcmFailbackNicDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provider specific input for InMageRcmFailback failover."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmFailbackPlannedFailoverProviderInput {
    #[serde(flatten)]
    pub planned_failover_provider_specific_failover_input: PlannedFailoverProviderSpecificFailoverInput,
    #[doc = "The recovery point type."]
    #[serde(rename = "recoveryPointType")]
    pub recovery_point_type: in_mage_rcm_failback_planned_failover_provider_input::RecoveryPointType,
}
impl InMageRcmFailbackPlannedFailoverProviderInput {
    pub fn new(
        planned_failover_provider_specific_failover_input: PlannedFailoverProviderSpecificFailoverInput,
        recovery_point_type: in_mage_rcm_failback_planned_failover_provider_input::RecoveryPointType,
    ) -> Self {
        Self {
            planned_failover_provider_specific_failover_input,
            recovery_point_type,
        }
    }
}
pub mod in_mage_rcm_failback_planned_failover_provider_input {
    use super::*;
    #[doc = "The recovery point type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryPointType")]
    pub enum RecoveryPointType {
        ApplicationConsistent,
        CrashConsistent,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryPointType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryPointType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryPointType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ApplicationConsistent => serializer.serialize_unit_variant("RecoveryPointType", 0u32, "ApplicationConsistent"),
                Self::CrashConsistent => serializer.serialize_unit_variant("RecoveryPointType", 1u32, "CrashConsistent"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "InMageRcmFailback policy creation input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmFailbackPolicyCreationInput {
    #[serde(flatten)]
    pub policy_provider_specific_input: PolicyProviderSpecificInput,
    #[doc = "The crash consistent snapshot frequency (in minutes)."]
    #[serde(rename = "crashConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub crash_consistent_frequency_in_minutes: Option<i32>,
    #[doc = "The app consistent snapshot frequency (in minutes)."]
    #[serde(rename = "appConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub app_consistent_frequency_in_minutes: Option<i32>,
}
impl InMageRcmFailbackPolicyCreationInput {
    pub fn new(policy_provider_specific_input: PolicyProviderSpecificInput) -> Self {
        Self {
            policy_provider_specific_input,
            crash_consistent_frequency_in_minutes: None,
            app_consistent_frequency_in_minutes: None,
        }
    }
}
#[doc = "InMageRcm failback specific policy details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmFailbackPolicyDetails {
    #[serde(flatten)]
    pub policy_provider_specific_details: PolicyProviderSpecificDetails,
    #[doc = "The app consistent snapshot frequency in minutes."]
    #[serde(rename = "appConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub app_consistent_frequency_in_minutes: Option<i32>,
    #[doc = "The crash consistent snapshot frequency in minutes."]
    #[serde(rename = "crashConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub crash_consistent_frequency_in_minutes: Option<i32>,
}
impl InMageRcmFailbackPolicyDetails {
    pub fn new(policy_provider_specific_details: PolicyProviderSpecificDetails) -> Self {
        Self {
            policy_provider_specific_details,
            app_consistent_frequency_in_minutes: None,
            crash_consistent_frequency_in_minutes: None,
        }
    }
}
#[doc = "InMageRcmFailback protected disk details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageRcmFailbackProtectedDiskDetails {
    #[doc = "The disk Id (reported by source agent)."]
    #[serde(rename = "diskId", default, skip_serializing_if = "Option::is_none")]
    pub disk_id: Option<String>,
    #[doc = "The disk name."]
    #[serde(rename = "diskName", default, skip_serializing_if = "Option::is_none")]
    pub disk_name: Option<String>,
    #[doc = "A value indicating whether the disk is the OS disk."]
    #[serde(rename = "isOSDisk", default, skip_serializing_if = "Option::is_none")]
    pub is_os_disk: Option<String>,
    #[doc = "The disk capacity in bytes."]
    #[serde(rename = "capacityInBytes", default, skip_serializing_if = "Option::is_none")]
    pub capacity_in_bytes: Option<i64>,
    #[doc = "The disk Uuid (reported by vCenter)."]
    #[serde(rename = "diskUuid", default, skip_serializing_if = "Option::is_none")]
    pub disk_uuid: Option<String>,
    #[doc = "The data pending in log data store in MB."]
    #[serde(rename = "dataPendingInLogDataStoreInMB", default, skip_serializing_if = "Option::is_none")]
    pub data_pending_in_log_data_store_in_mb: Option<f64>,
    #[doc = "The data pending at source agent in MB."]
    #[serde(rename = "dataPendingAtSourceAgentInMB", default, skip_serializing_if = "Option::is_none")]
    pub data_pending_at_source_agent_in_mb: Option<f64>,
    #[doc = "A value indicating whether initial replication is complete or not."]
    #[serde(rename = "isInitialReplicationComplete", default, skip_serializing_if = "Option::is_none")]
    pub is_initial_replication_complete: Option<String>,
    #[doc = "InMageRcmFailback disk level sync details."]
    #[serde(rename = "irDetails", default, skip_serializing_if = "Option::is_none")]
    pub ir_details: Option<InMageRcmFailbackSyncDetails>,
    #[doc = "InMageRcmFailback disk level sync details."]
    #[serde(rename = "resyncDetails", default, skip_serializing_if = "Option::is_none")]
    pub resync_details: Option<InMageRcmFailbackSyncDetails>,
    #[doc = "The last sync time."]
    #[serde(rename = "lastSyncTime", with = "azure_core::date::rfc3339::option")]
    pub last_sync_time: Option<time::OffsetDateTime>,
}
impl InMageRcmFailbackProtectedDiskDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "InMageRcmFailback provider specific details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmFailbackReplicationDetails {
    #[serde(flatten)]
    pub replication_provider_specific_settings: ReplicationProviderSpecificSettings,
    #[doc = "The virtual machine internal identifier."]
    #[serde(rename = "internalIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub internal_identifier: Option<String>,
    #[doc = "The ARM Id of the azure VM."]
    #[serde(rename = "azureVirtualMachineId", default, skip_serializing_if = "Option::is_none")]
    pub azure_virtual_machine_id: Option<String>,
    #[doc = "The multi VM group name."]
    #[serde(rename = "multiVmGroupName", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_group_name: Option<String>,
    #[doc = "The reprotect agent Id."]
    #[serde(rename = "reprotectAgentId", default, skip_serializing_if = "Option::is_none")]
    pub reprotect_agent_id: Option<String>,
    #[doc = "The reprotect agent name."]
    #[serde(rename = "reprotectAgentName", default, skip_serializing_if = "Option::is_none")]
    pub reprotect_agent_name: Option<String>,
    #[doc = "The type of the OS on the VM."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "The log storage account ARM Id."]
    #[serde(rename = "logStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub log_storage_account_id: Option<String>,
    #[doc = "The target vCenter Id."]
    #[serde(rename = "targetvCenterId", default, skip_serializing_if = "Option::is_none")]
    pub targetv_center_id: Option<String>,
    #[doc = "The target datastore name."]
    #[serde(rename = "targetDataStoreName", default, skip_serializing_if = "Option::is_none")]
    pub target_data_store_name: Option<String>,
    #[doc = "The target VM name."]
    #[serde(rename = "targetVmName", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_name: Option<String>,
    #[doc = "The initial replication progress percentage."]
    #[serde(rename = "initialReplicationProgressPercentage", default, skip_serializing_if = "Option::is_none")]
    pub initial_replication_progress_percentage: Option<i32>,
    #[doc = "The initial replication processed bytes. This includes sum of total bytes transferred and matched bytes on all selected disks in source VM."]
    #[serde(rename = "initialReplicationProcessedBytes", default, skip_serializing_if = "Option::is_none")]
    pub initial_replication_processed_bytes: Option<i64>,
    #[doc = "The initial replication transferred bytes from source VM to target for all selected disks on source VM."]
    #[serde(rename = "initialReplicationTransferredBytes", default, skip_serializing_if = "Option::is_none")]
    pub initial_replication_transferred_bytes: Option<i64>,
    #[doc = "The initial replication progress health."]
    #[serde(rename = "initialReplicationProgressHealth", default, skip_serializing_if = "Option::is_none")]
    pub initial_replication_progress_health: Option<in_mage_rcm_failback_replication_details::InitialReplicationProgressHealth>,
    #[doc = "The resync progress percentage."]
    #[serde(rename = "resyncProgressPercentage", default, skip_serializing_if = "Option::is_none")]
    pub resync_progress_percentage: Option<i32>,
    #[doc = "The resync processed bytes. This includes sum of total bytes transferred and matched bytes on all selected disks in source VM."]
    #[serde(rename = "resyncProcessedBytes", default, skip_serializing_if = "Option::is_none")]
    pub resync_processed_bytes: Option<i64>,
    #[doc = "The resync transferred bytes from source VM to target for all selected disks on source VM."]
    #[serde(rename = "resyncTransferredBytes", default, skip_serializing_if = "Option::is_none")]
    pub resync_transferred_bytes: Option<i64>,
    #[doc = "The resync progress health."]
    #[serde(rename = "resyncProgressHealth", default, skip_serializing_if = "Option::is_none")]
    pub resync_progress_health: Option<in_mage_rcm_failback_replication_details::ResyncProgressHealth>,
    #[doc = "A value indicating whether resync is required."]
    #[serde(rename = "resyncRequired", default, skip_serializing_if = "Option::is_none")]
    pub resync_required: Option<String>,
    #[doc = "The resync state."]
    #[serde(rename = "resyncState", default, skip_serializing_if = "Option::is_none")]
    pub resync_state: Option<in_mage_rcm_failback_replication_details::ResyncState>,
    #[doc = "The list of protected disks."]
    #[serde(rename = "protectedDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub protected_disks: Vec<InMageRcmFailbackProtectedDiskDetails>,
    #[doc = "InMageRcmFailback mobility agent details."]
    #[serde(rename = "mobilityAgentDetails", default, skip_serializing_if = "Option::is_none")]
    pub mobility_agent_details: Option<InMageRcmFailbackMobilityAgentDetails>,
    #[doc = "The network details."]
    #[serde(rename = "vmNics", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_nics: Vec<InMageRcmFailbackNicDetails>,
    #[doc = "The last planned failover start time."]
    #[serde(rename = "lastPlannedFailoverStartTime", with = "azure_core::date::rfc3339::option")]
    pub last_planned_failover_start_time: Option<time::OffsetDateTime>,
    #[doc = "The last planned failover status."]
    #[serde(rename = "lastPlannedFailoverStatus", default, skip_serializing_if = "Option::is_none")]
    pub last_planned_failover_status: Option<in_mage_rcm_failback_replication_details::LastPlannedFailoverStatus>,
    #[doc = "InMageRcmFailback discovered VM details."]
    #[serde(rename = "discoveredVmDetails", default, skip_serializing_if = "Option::is_none")]
    pub discovered_vm_details: Option<InMageRcmFailbackDiscoveredProtectedVmDetails>,
    #[doc = "The policy Id used by the forward replication."]
    #[serde(rename = "lastUsedPolicyId", default, skip_serializing_if = "Option::is_none")]
    pub last_used_policy_id: Option<String>,
    #[doc = "The policy friendly name used by the forward replication."]
    #[serde(rename = "lastUsedPolicyFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub last_used_policy_friendly_name: Option<String>,
    #[doc = "A value indicating whether agent registration was successful after failover."]
    #[serde(
        rename = "isAgentRegistrationSuccessfulAfterFailover",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_agent_registration_successful_after_failover: Option<bool>,
}
impl InMageRcmFailbackReplicationDetails {
    pub fn new(replication_provider_specific_settings: ReplicationProviderSpecificSettings) -> Self {
        Self {
            replication_provider_specific_settings,
            internal_identifier: None,
            azure_virtual_machine_id: None,
            multi_vm_group_name: None,
            reprotect_agent_id: None,
            reprotect_agent_name: None,
            os_type: None,
            log_storage_account_id: None,
            targetv_center_id: None,
            target_data_store_name: None,
            target_vm_name: None,
            initial_replication_progress_percentage: None,
            initial_replication_processed_bytes: None,
            initial_replication_transferred_bytes: None,
            initial_replication_progress_health: None,
            resync_progress_percentage: None,
            resync_processed_bytes: None,
            resync_transferred_bytes: None,
            resync_progress_health: None,
            resync_required: None,
            resync_state: None,
            protected_disks: Vec::new(),
            mobility_agent_details: None,
            vm_nics: Vec::new(),
            last_planned_failover_start_time: None,
            last_planned_failover_status: None,
            discovered_vm_details: None,
            last_used_policy_id: None,
            last_used_policy_friendly_name: None,
            is_agent_registration_successful_after_failover: None,
        }
    }
}
pub mod in_mage_rcm_failback_replication_details {
    use super::*;
    #[doc = "The initial replication progress health."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "InitialReplicationProgressHealth")]
    pub enum InitialReplicationProgressHealth {
        None,
        InProgress,
        SlowProgress,
        NoProgress,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for InitialReplicationProgressHealth {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for InitialReplicationProgressHealth {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for InitialReplicationProgressHealth {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("InitialReplicationProgressHealth", 0u32, "None"),
                Self::InProgress => serializer.serialize_unit_variant("InitialReplicationProgressHealth", 1u32, "InProgress"),
                Self::SlowProgress => serializer.serialize_unit_variant("InitialReplicationProgressHealth", 2u32, "SlowProgress"),
                Self::NoProgress => serializer.serialize_unit_variant("InitialReplicationProgressHealth", 3u32, "NoProgress"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The resync progress health."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResyncProgressHealth")]
    pub enum ResyncProgressHealth {
        None,
        InProgress,
        SlowProgress,
        NoProgress,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResyncProgressHealth {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResyncProgressHealth {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResyncProgressHealth {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ResyncProgressHealth", 0u32, "None"),
                Self::InProgress => serializer.serialize_unit_variant("ResyncProgressHealth", 1u32, "InProgress"),
                Self::SlowProgress => serializer.serialize_unit_variant("ResyncProgressHealth", 2u32, "SlowProgress"),
                Self::NoProgress => serializer.serialize_unit_variant("ResyncProgressHealth", 3u32, "NoProgress"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The resync state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResyncState")]
    pub enum ResyncState {
        None,
        PreparedForResynchronization,
        StartedResynchronization,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResyncState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResyncState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResyncState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ResyncState", 0u32, "None"),
                Self::PreparedForResynchronization => {
                    serializer.serialize_unit_variant("ResyncState", 1u32, "PreparedForResynchronization")
                }
                Self::StartedResynchronization => serializer.serialize_unit_variant("ResyncState", 2u32, "StartedResynchronization"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The last planned failover status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LastPlannedFailoverStatus")]
    pub enum LastPlannedFailoverStatus {
        Succeeded,
        Failed,
        Cancelled,
        Unknown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LastPlannedFailoverStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LastPlannedFailoverStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LastPlannedFailoverStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("LastPlannedFailoverStatus", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("LastPlannedFailoverStatus", 1u32, "Failed"),
                Self::Cancelled => serializer.serialize_unit_variant("LastPlannedFailoverStatus", 2u32, "Cancelled"),
                Self::Unknown => serializer.serialize_unit_variant("LastPlannedFailoverStatus", 3u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "InMageRcmFailback specific provider input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmFailbackReprotectInput {
    #[serde(flatten)]
    pub reverse_replication_provider_specific_input: ReverseReplicationProviderSpecificInput,
    #[doc = "The process server Id."]
    #[serde(rename = "processServerId")]
    pub process_server_id: String,
    #[doc = "The run as account Id."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "The Policy Id."]
    #[serde(rename = "policyId")]
    pub policy_id: String,
}
impl InMageRcmFailbackReprotectInput {
    pub fn new(
        reverse_replication_provider_specific_input: ReverseReplicationProviderSpecificInput,
        process_server_id: String,
        policy_id: String,
    ) -> Self {
        Self {
            reverse_replication_provider_specific_input,
            process_server_id,
            run_as_account_id: None,
            policy_id,
        }
    }
}
#[doc = "InMageRcmFailback disk level sync details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageRcmFailbackSyncDetails {
    #[doc = "The progress health."]
    #[serde(rename = "progressHealth", default, skip_serializing_if = "Option::is_none")]
    pub progress_health: Option<in_mage_rcm_failback_sync_details::ProgressHealth>,
    #[doc = "The transferred bytes from source VM to azure for the disk."]
    #[serde(rename = "transferredBytes", default, skip_serializing_if = "Option::is_none")]
    pub transferred_bytes: Option<i64>,
    #[doc = "The bytes transferred in last 15 minutes from source VM to target."]
    #[serde(rename = "last15MinutesTransferredBytes", default, skip_serializing_if = "Option::is_none")]
    pub last15_minutes_transferred_bytes: Option<i64>,
    #[doc = "The time of the last data transfer from source VM to target."]
    #[serde(rename = "lastDataTransferTimeUtc", default, skip_serializing_if = "Option::is_none")]
    pub last_data_transfer_time_utc: Option<String>,
    #[doc = "The total processed bytes. This includes bytes that are transferred from source VM to target and matched bytes."]
    #[serde(rename = "processedBytes", default, skip_serializing_if = "Option::is_none")]
    pub processed_bytes: Option<i64>,
    #[doc = "The start time."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The last refresh time."]
    #[serde(rename = "lastRefreshTime", default, skip_serializing_if = "Option::is_none")]
    pub last_refresh_time: Option<String>,
    #[doc = "Progress in percentage. Progress percentage is calculated based on processed bytes."]
    #[serde(rename = "progressPercentage", default, skip_serializing_if = "Option::is_none")]
    pub progress_percentage: Option<i32>,
}
impl InMageRcmFailbackSyncDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod in_mage_rcm_failback_sync_details {
    use super::*;
    #[doc = "The progress health."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProgressHealth")]
    pub enum ProgressHealth {
        None,
        InProgress,
        SlowProgress,
        NoProgress,
        Queued,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProgressHealth {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProgressHealth {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProgressHealth {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ProgressHealth", 0u32, "None"),
                Self::InProgress => serializer.serialize_unit_variant("ProgressHealth", 1u32, "InProgress"),
                Self::SlowProgress => serializer.serialize_unit_variant("ProgressHealth", 2u32, "SlowProgress"),
                Self::NoProgress => serializer.serialize_unit_variant("ProgressHealth", 3u32, "NoProgress"),
                Self::Queued => serializer.serialize_unit_variant("ProgressHealth", 4u32, "Queued"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "InMageRcm last source agent upgrade error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageRcmLastAgentUpgradeErrorDetails {
    #[doc = "The error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "The error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "The possible causes."]
    #[serde(rename = "possibleCauses", default, skip_serializing_if = "Option::is_none")]
    pub possible_causes: Option<String>,
    #[doc = "The recommended action."]
    #[serde(rename = "recommendedAction", default, skip_serializing_if = "Option::is_none")]
    pub recommended_action: Option<String>,
    #[doc = "The error message parameters."]
    #[serde(rename = "errorMessageParameters", default, skip_serializing_if = "Option::is_none")]
    pub error_message_parameters: Option<serde_json::Value>,
    #[doc = "The error tags."]
    #[serde(rename = "errorTags", default, skip_serializing_if = "Option::is_none")]
    pub error_tags: Option<serde_json::Value>,
}
impl InMageRcmLastAgentUpgradeErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "InMageRcm mobility agent details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageRcmMobilityAgentDetails {
    #[doc = "The agent version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The latest agent version available."]
    #[serde(rename = "latestVersion", default, skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<String>,
    #[doc = "The latest agent version release date."]
    #[serde(rename = "latestAgentReleaseDate", default, skip_serializing_if = "Option::is_none")]
    pub latest_agent_release_date: Option<String>,
    #[doc = "The driver version."]
    #[serde(rename = "driverVersion", default, skip_serializing_if = "Option::is_none")]
    pub driver_version: Option<String>,
    #[doc = "The latest upgradeable version available without reboot."]
    #[serde(rename = "latestUpgradableVersionWithoutReboot", default, skip_serializing_if = "Option::is_none")]
    pub latest_upgradable_version_without_reboot: Option<String>,
    #[doc = "The agent version expiry date."]
    #[serde(rename = "agentVersionExpiryDate", with = "azure_core::date::rfc3339::option")]
    pub agent_version_expiry_date: Option<time::OffsetDateTime>,
    #[doc = "The driver version expiry date."]
    #[serde(rename = "driverVersionExpiryDate", with = "azure_core::date::rfc3339::option")]
    pub driver_version_expiry_date: Option<time::OffsetDateTime>,
    #[doc = "The time of the last heartbeat received from the agent."]
    #[serde(rename = "lastHeartbeatUtc", with = "azure_core::date::rfc3339::option")]
    pub last_heartbeat_utc: Option<time::OffsetDateTime>,
    #[doc = "The whether update is possible or not."]
    #[serde(rename = "reasonsBlockingUpgrade", default, skip_serializing_if = "Vec::is_empty")]
    pub reasons_blocking_upgrade: Vec<String>,
    #[doc = "A value indicating whether agent is upgradeable or not."]
    #[serde(rename = "isUpgradeable", default, skip_serializing_if = "Option::is_none")]
    pub is_upgradeable: Option<String>,
}
impl InMageRcmMobilityAgentDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "InMageRcm NIC details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageRcmNicDetails {
    #[doc = "The NIC Id."]
    #[serde(rename = "nicId", default, skip_serializing_if = "Option::is_none")]
    pub nic_id: Option<String>,
    #[doc = "A value indicating whether this is the primary NIC."]
    #[serde(rename = "isPrimaryNic", default, skip_serializing_if = "Option::is_none")]
    pub is_primary_nic: Option<String>,
    #[doc = "A value indicating whether this NIC is selected for failover."]
    #[serde(rename = "isSelectedForFailover", default, skip_serializing_if = "Option::is_none")]
    pub is_selected_for_failover: Option<String>,
    #[doc = "The source IP address."]
    #[serde(rename = "sourceIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub source_ip_address: Option<String>,
    #[doc = "The source IP address type."]
    #[serde(rename = "sourceIPAddressType", default, skip_serializing_if = "Option::is_none")]
    pub source_ip_address_type: Option<in_mage_rcm_nic_details::SourceIpAddressType>,
    #[doc = "Source network Id."]
    #[serde(rename = "sourceNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub source_network_id: Option<String>,
    #[doc = "Source subnet name."]
    #[serde(rename = "sourceSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub source_subnet_name: Option<String>,
    #[doc = "The target IP address."]
    #[serde(rename = "targetIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub target_ip_address: Option<String>,
    #[doc = "The target IP address type."]
    #[serde(rename = "targetIPAddressType", default, skip_serializing_if = "Option::is_none")]
    pub target_ip_address_type: Option<in_mage_rcm_nic_details::TargetIpAddressType>,
    #[doc = "Target subnet name."]
    #[serde(rename = "targetSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub target_subnet_name: Option<String>,
    #[doc = "Test subnet name."]
    #[serde(rename = "testSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub test_subnet_name: Option<String>,
    #[doc = "The test IP address."]
    #[serde(rename = "testIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub test_ip_address: Option<String>,
    #[doc = "The test IP address type."]
    #[serde(rename = "testIPAddressType", default, skip_serializing_if = "Option::is_none")]
    pub test_ip_address_type: Option<in_mage_rcm_nic_details::TestIpAddressType>,
}
impl InMageRcmNicDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod in_mage_rcm_nic_details {
    use super::*;
    #[doc = "The source IP address type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SourceIpAddressType")]
    pub enum SourceIpAddressType {
        Dynamic,
        Static,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SourceIpAddressType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SourceIpAddressType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SourceIpAddressType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Dynamic => serializer.serialize_unit_variant("SourceIpAddressType", 0u32, "Dynamic"),
                Self::Static => serializer.serialize_unit_variant("SourceIpAddressType", 1u32, "Static"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The target IP address type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TargetIpAddressType")]
    pub enum TargetIpAddressType {
        Dynamic,
        Static,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TargetIpAddressType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TargetIpAddressType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TargetIpAddressType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Dynamic => serializer.serialize_unit_variant("TargetIpAddressType", 0u32, "Dynamic"),
                Self::Static => serializer.serialize_unit_variant("TargetIpAddressType", 1u32, "Static"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The test IP address type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TestIpAddressType")]
    pub enum TestIpAddressType {
        Dynamic,
        Static,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TestIpAddressType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TestIpAddressType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TestIpAddressType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Dynamic => serializer.serialize_unit_variant("TestIpAddressType", 0u32, "Dynamic"),
                Self::Static => serializer.serialize_unit_variant("TestIpAddressType", 1u32, "Static"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "InMageRcm NIC input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmNicInput {
    #[doc = "The NIC Id."]
    #[serde(rename = "nicId")]
    pub nic_id: String,
    #[doc = "A value indicating whether this is the primary NIC."]
    #[serde(rename = "isPrimaryNic")]
    pub is_primary_nic: String,
    #[doc = "A value indicating whether this NIC is selected for failover."]
    #[serde(rename = "isSelectedForFailover", default, skip_serializing_if = "Option::is_none")]
    pub is_selected_for_failover: Option<String>,
    #[doc = "Target subnet name."]
    #[serde(rename = "targetSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub target_subnet_name: Option<String>,
    #[doc = "The target static IP address."]
    #[serde(rename = "targetStaticIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub target_static_ip_address: Option<String>,
    #[doc = "The test subnet name."]
    #[serde(rename = "testSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub test_subnet_name: Option<String>,
    #[doc = "The test static IP address."]
    #[serde(rename = "testStaticIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub test_static_ip_address: Option<String>,
}
impl InMageRcmNicInput {
    pub fn new(nic_id: String, is_primary_nic: String) -> Self {
        Self {
            nic_id,
            is_primary_nic,
            is_selected_for_failover: None,
            target_subnet_name: None,
            target_static_ip_address: None,
            test_subnet_name: None,
            test_static_ip_address: None,
        }
    }
}
#[doc = "InMageRcm policy creation input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmPolicyCreationInput {
    #[serde(flatten)]
    pub policy_provider_specific_input: PolicyProviderSpecificInput,
    #[doc = "The duration in minutes until which the recovery points need to be stored."]
    #[serde(rename = "recoveryPointHistoryInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_history_in_minutes: Option<i32>,
    #[doc = "The crash consistent snapshot frequency (in minutes)."]
    #[serde(rename = "crashConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub crash_consistent_frequency_in_minutes: Option<i32>,
    #[doc = "The app consistent snapshot frequency (in minutes)."]
    #[serde(rename = "appConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub app_consistent_frequency_in_minutes: Option<i32>,
    #[doc = "A value indicating whether multi-VM sync has to be enabled."]
    #[serde(rename = "enableMultiVmSync", default, skip_serializing_if = "Option::is_none")]
    pub enable_multi_vm_sync: Option<String>,
}
impl InMageRcmPolicyCreationInput {
    pub fn new(policy_provider_specific_input: PolicyProviderSpecificInput) -> Self {
        Self {
            policy_provider_specific_input,
            recovery_point_history_in_minutes: None,
            crash_consistent_frequency_in_minutes: None,
            app_consistent_frequency_in_minutes: None,
            enable_multi_vm_sync: None,
        }
    }
}
#[doc = "InMageRcm specific policy details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmPolicyDetails {
    #[serde(flatten)]
    pub policy_provider_specific_details: PolicyProviderSpecificDetails,
    #[doc = "The duration in minutes until which the recovery points need to be stored."]
    #[serde(rename = "recoveryPointHistoryInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_history_in_minutes: Option<i32>,
    #[doc = "The app consistent snapshot frequency in minutes."]
    #[serde(rename = "appConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub app_consistent_frequency_in_minutes: Option<i32>,
    #[doc = "The crash consistent snapshot frequency in minutes."]
    #[serde(rename = "crashConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub crash_consistent_frequency_in_minutes: Option<i32>,
    #[doc = "A value indicating whether multi-VM sync has to be enabled."]
    #[serde(rename = "enableMultiVmSync", default, skip_serializing_if = "Option::is_none")]
    pub enable_multi_vm_sync: Option<String>,
}
impl InMageRcmPolicyDetails {
    pub fn new(policy_provider_specific_details: PolicyProviderSpecificDetails) -> Self {
        Self {
            policy_provider_specific_details,
            recovery_point_history_in_minutes: None,
            app_consistent_frequency_in_minutes: None,
            crash_consistent_frequency_in_minutes: None,
            enable_multi_vm_sync: None,
        }
    }
}
#[doc = "InMageRcm protected disk details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageRcmProtectedDiskDetails {
    #[doc = "The disk Id."]
    #[serde(rename = "diskId", default, skip_serializing_if = "Option::is_none")]
    pub disk_id: Option<String>,
    #[doc = "The disk name."]
    #[serde(rename = "diskName", default, skip_serializing_if = "Option::is_none")]
    pub disk_name: Option<String>,
    #[doc = "A value indicating whether the disk is the OS disk."]
    #[serde(rename = "isOSDisk", default, skip_serializing_if = "Option::is_none")]
    pub is_os_disk: Option<String>,
    #[doc = "The disk capacity in bytes."]
    #[serde(rename = "capacityInBytes", default, skip_serializing_if = "Option::is_none")]
    pub capacity_in_bytes: Option<i64>,
    #[doc = "The log storage account ARM Id."]
    #[serde(rename = "logStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub log_storage_account_id: Option<String>,
    #[doc = "The DiskEncryptionSet ARM Id."]
    #[serde(rename = "diskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set_id: Option<String>,
    #[doc = "The ARM Id of the seed managed disk."]
    #[serde(rename = "seedManagedDiskId", default, skip_serializing_if = "Option::is_none")]
    pub seed_managed_disk_id: Option<String>,
    #[doc = "The ARM Id of the target managed disk."]
    #[serde(rename = "targetManagedDiskId", default, skip_serializing_if = "Option::is_none")]
    pub target_managed_disk_id: Option<String>,
    #[doc = "The disk type."]
    #[serde(rename = "diskType", default, skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<in_mage_rcm_protected_disk_details::DiskType>,
    #[doc = "The data pending in log data store in MB."]
    #[serde(rename = "dataPendingInLogDataStoreInMB", default, skip_serializing_if = "Option::is_none")]
    pub data_pending_in_log_data_store_in_mb: Option<f64>,
    #[doc = "The data pending at source agent in MB."]
    #[serde(rename = "dataPendingAtSourceAgentInMB", default, skip_serializing_if = "Option::is_none")]
    pub data_pending_at_source_agent_in_mb: Option<f64>,
    #[doc = "A value indicating whether initial replication is complete or not."]
    #[serde(rename = "isInitialReplicationComplete", default, skip_serializing_if = "Option::is_none")]
    pub is_initial_replication_complete: Option<String>,
    #[doc = "InMageRcm disk level sync details."]
    #[serde(rename = "irDetails", default, skip_serializing_if = "Option::is_none")]
    pub ir_details: Option<InMageRcmSyncDetails>,
    #[doc = "InMageRcm disk level sync details."]
    #[serde(rename = "resyncDetails", default, skip_serializing_if = "Option::is_none")]
    pub resync_details: Option<InMageRcmSyncDetails>,
}
impl InMageRcmProtectedDiskDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod in_mage_rcm_protected_disk_details {
    use super::*;
    #[doc = "The disk type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DiskType")]
    pub enum DiskType {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(rename = "StandardSSD_LRS")]
        StandardSsdLrs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DiskType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DiskType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DiskType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::StandardLrs => serializer.serialize_unit_variant("DiskType", 0u32, "Standard_LRS"),
                Self::PremiumLrs => serializer.serialize_unit_variant("DiskType", 1u32, "Premium_LRS"),
                Self::StandardSsdLrs => serializer.serialize_unit_variant("DiskType", 2u32, "StandardSSD_LRS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "InMageRcm provider specific container mapping details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmProtectionContainerMappingDetails {
    #[serde(flatten)]
    pub protection_container_mapping_provider_specific_details: ProtectionContainerMappingProviderSpecificDetails,
    #[doc = "A value indicating whether the flag for enable agent auto upgrade."]
    #[serde(rename = "enableAgentAutoUpgrade", default, skip_serializing_if = "Option::is_none")]
    pub enable_agent_auto_upgrade: Option<String>,
}
impl InMageRcmProtectionContainerMappingDetails {
    pub fn new(protection_container_mapping_provider_specific_details: ProtectionContainerMappingProviderSpecificDetails) -> Self {
        Self {
            protection_container_mapping_provider_specific_details,
            enable_agent_auto_upgrade: None,
        }
    }
}
#[doc = "InMageRcm provider specific recovery point details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmRecoveryPointDetails {
    #[serde(flatten)]
    pub provider_specific_recovery_point_details: ProviderSpecificRecoveryPointDetails,
    #[doc = "A value indicating whether the recovery point is multi VM consistent."]
    #[serde(rename = "isMultiVmSyncPoint", default, skip_serializing_if = "Option::is_none")]
    pub is_multi_vm_sync_point: Option<String>,
}
impl InMageRcmRecoveryPointDetails {
    pub fn new(provider_specific_recovery_point_details: ProviderSpecificRecoveryPointDetails) -> Self {
        Self {
            provider_specific_recovery_point_details,
            is_multi_vm_sync_point: None,
        }
    }
}
#[doc = "InMageRcm provider specific details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmReplicationDetails {
    #[serde(flatten)]
    pub replication_provider_specific_settings: ReplicationProviderSpecificSettings,
    #[doc = "The virtual machine internal identifier."]
    #[serde(rename = "internalIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub internal_identifier: Option<String>,
    #[doc = "The ARM Id of the discovered VM."]
    #[serde(rename = "fabricDiscoveryMachineId", default, skip_serializing_if = "Option::is_none")]
    pub fabric_discovery_machine_id: Option<String>,
    #[doc = "The multi VM group name."]
    #[serde(rename = "multiVmGroupName", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_group_name: Option<String>,
    #[doc = "The type of the discovered VM."]
    #[serde(rename = "discoveryType", default, skip_serializing_if = "Option::is_none")]
    pub discovery_type: Option<String>,
    #[doc = "The process server Id."]
    #[serde(rename = "processServerId", default, skip_serializing_if = "Option::is_none")]
    pub process_server_id: Option<String>,
    #[doc = "The processor core count."]
    #[serde(rename = "processorCoreCount", default, skip_serializing_if = "Option::is_none")]
    pub processor_core_count: Option<i32>,
    #[doc = "The allocated memory in MB."]
    #[serde(rename = "allocatedMemoryInMB", default, skip_serializing_if = "Option::is_none")]
    pub allocated_memory_in_mb: Option<f64>,
    #[doc = "The process server name."]
    #[serde(rename = "processServerName", default, skip_serializing_if = "Option::is_none")]
    pub process_server_name: Option<String>,
    #[doc = "The run-as account Id."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "The type of the OS on the VM."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "The firmware type."]
    #[serde(rename = "firmwareType", default, skip_serializing_if = "Option::is_none")]
    pub firmware_type: Option<String>,
    #[doc = "The IP address of the primary network interface."]
    #[serde(rename = "primaryNicIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub primary_nic_ip_address: Option<String>,
    #[doc = "The target generation."]
    #[serde(rename = "targetGeneration", default, skip_serializing_if = "Option::is_none")]
    pub target_generation: Option<String>,
    #[doc = "License Type of the VM to be used."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    #[doc = "Target VM name."]
    #[serde(rename = "targetVmName", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_name: Option<String>,
    #[doc = "The target VM size."]
    #[serde(rename = "targetVmSize", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_size: Option<String>,
    #[doc = "The target resource group Id."]
    #[serde(rename = "targetResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_group_id: Option<String>,
    #[doc = "The target location."]
    #[serde(rename = "targetLocation", default, skip_serializing_if = "Option::is_none")]
    pub target_location: Option<String>,
    #[doc = "The target availability set Id."]
    #[serde(rename = "targetAvailabilitySetId", default, skip_serializing_if = "Option::is_none")]
    pub target_availability_set_id: Option<String>,
    #[doc = "The target availability zone."]
    #[serde(rename = "targetAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub target_availability_zone: Option<String>,
    #[doc = "The target proximity placement group Id."]
    #[serde(rename = "targetProximityPlacementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub target_proximity_placement_group_id: Option<String>,
    #[doc = "The target boot diagnostics storage account ARM Id."]
    #[serde(rename = "targetBootDiagnosticsStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub target_boot_diagnostics_storage_account_id: Option<String>,
    #[doc = "The target network Id."]
    #[serde(rename = "targetNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub target_network_id: Option<String>,
    #[doc = "The test network Id."]
    #[serde(rename = "testNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub test_network_id: Option<String>,
    #[doc = "The recovery point Id to which the VM was failed over."]
    #[serde(rename = "failoverRecoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub failover_recovery_point_id: Option<String>,
    #[doc = "The last recovery point received time."]
    #[serde(rename = "lastRecoveryPointReceived", with = "azure_core::date::rfc3339::option")]
    pub last_recovery_point_received: Option<time::OffsetDateTime>,
    #[doc = "The last recovery point objective value."]
    #[serde(rename = "lastRpoInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub last_rpo_in_seconds: Option<i64>,
    #[doc = "The last recovery point objective calculated time."]
    #[serde(rename = "lastRpoCalculatedTime", with = "azure_core::date::rfc3339::option")]
    pub last_rpo_calculated_time: Option<time::OffsetDateTime>,
    #[doc = "The last recovery point Id."]
    #[serde(rename = "lastRecoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub last_recovery_point_id: Option<String>,
    #[doc = "The initial replication progress percentage. This is calculated based on total bytes processed for all disks in the source VM."]
    #[serde(rename = "initialReplicationProgressPercentage", default, skip_serializing_if = "Option::is_none")]
    pub initial_replication_progress_percentage: Option<i32>,
    #[doc = "The initial replication processed bytes. This includes sum of total bytes transferred and matched bytes on all selected disks in source VM."]
    #[serde(rename = "initialReplicationProcessedBytes", default, skip_serializing_if = "Option::is_none")]
    pub initial_replication_processed_bytes: Option<i64>,
    #[doc = "The initial replication transferred bytes from source VM to azure for all selected disks on source VM."]
    #[serde(rename = "initialReplicationTransferredBytes", default, skip_serializing_if = "Option::is_none")]
    pub initial_replication_transferred_bytes: Option<i64>,
    #[doc = "The initial replication progress health."]
    #[serde(rename = "initialReplicationProgressHealth", default, skip_serializing_if = "Option::is_none")]
    pub initial_replication_progress_health: Option<in_mage_rcm_replication_details::InitialReplicationProgressHealth>,
    #[doc = "The resync progress percentage. This is calculated based on total bytes processed for all disks in the source VM."]
    #[serde(rename = "resyncProgressPercentage", default, skip_serializing_if = "Option::is_none")]
    pub resync_progress_percentage: Option<i32>,
    #[doc = "The resync processed bytes. This includes sum of total bytes transferred and matched bytes on all selected disks in source VM."]
    #[serde(rename = "resyncProcessedBytes", default, skip_serializing_if = "Option::is_none")]
    pub resync_processed_bytes: Option<i64>,
    #[doc = "The resync transferred bytes from source VM to azure for all selected disks on source VM."]
    #[serde(rename = "resyncTransferredBytes", default, skip_serializing_if = "Option::is_none")]
    pub resync_transferred_bytes: Option<i64>,
    #[doc = "The resync progress health."]
    #[serde(rename = "resyncProgressHealth", default, skip_serializing_if = "Option::is_none")]
    pub resync_progress_health: Option<in_mage_rcm_replication_details::ResyncProgressHealth>,
    #[doc = "A value indicating whether resync is required."]
    #[serde(rename = "resyncRequired", default, skip_serializing_if = "Option::is_none")]
    pub resync_required: Option<String>,
    #[doc = "The resync state."]
    #[serde(rename = "resyncState", default, skip_serializing_if = "Option::is_none")]
    pub resync_state: Option<in_mage_rcm_replication_details::ResyncState>,
    #[doc = "The agent auto upgrade state."]
    #[serde(rename = "agentUpgradeState", default, skip_serializing_if = "Option::is_none")]
    pub agent_upgrade_state: Option<in_mage_rcm_replication_details::AgentUpgradeState>,
    #[doc = "The last agent upgrade type."]
    #[serde(rename = "lastAgentUpgradeType", default, skip_serializing_if = "Option::is_none")]
    pub last_agent_upgrade_type: Option<String>,
    #[doc = "The agent upgrade job Id."]
    #[serde(rename = "agentUpgradeJobId", default, skip_serializing_if = "Option::is_none")]
    pub agent_upgrade_job_id: Option<String>,
    #[doc = "The agent version to which last agent upgrade was attempted."]
    #[serde(rename = "agentUpgradeAttemptToVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_upgrade_attempt_to_version: Option<String>,
    #[doc = "The list of protected disks."]
    #[serde(rename = "protectedDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub protected_disks: Vec<InMageRcmProtectedDiskDetails>,
    #[doc = "A value indicating whether last agent upgrade was successful or not."]
    #[serde(rename = "isLastUpgradeSuccessful", default, skip_serializing_if = "Option::is_none")]
    pub is_last_upgrade_successful: Option<String>,
    #[doc = "A value indicating whether agent registration was successful after failover."]
    #[serde(
        rename = "isAgentRegistrationSuccessfulAfterFailover",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_agent_registration_successful_after_failover: Option<bool>,
    #[doc = "InMageRcm mobility agent details."]
    #[serde(rename = "mobilityAgentDetails", default, skip_serializing_if = "Option::is_none")]
    pub mobility_agent_details: Option<InMageRcmMobilityAgentDetails>,
    #[doc = "The last agent upgrade error information."]
    #[serde(rename = "lastAgentUpgradeErrorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub last_agent_upgrade_error_details: Vec<InMageRcmLastAgentUpgradeErrorDetails>,
    #[doc = "The agent upgrade blocking error information."]
    #[serde(rename = "agentUpgradeBlockingErrorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub agent_upgrade_blocking_error_details: Vec<InMageRcmAgentUpgradeBlockingErrorDetails>,
    #[doc = "The network details."]
    #[serde(rename = "vmNics", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_nics: Vec<InMageRcmNicDetails>,
    #[doc = "InMageRcm discovered protected VM details."]
    #[serde(rename = "discoveredVmDetails", default, skip_serializing_if = "Option::is_none")]
    pub discovered_vm_details: Option<InMageRcmDiscoveredProtectedVmDetails>,
}
impl InMageRcmReplicationDetails {
    pub fn new(replication_provider_specific_settings: ReplicationProviderSpecificSettings) -> Self {
        Self {
            replication_provider_specific_settings,
            internal_identifier: None,
            fabric_discovery_machine_id: None,
            multi_vm_group_name: None,
            discovery_type: None,
            process_server_id: None,
            processor_core_count: None,
            allocated_memory_in_mb: None,
            process_server_name: None,
            run_as_account_id: None,
            os_type: None,
            firmware_type: None,
            primary_nic_ip_address: None,
            target_generation: None,
            license_type: None,
            target_vm_name: None,
            target_vm_size: None,
            target_resource_group_id: None,
            target_location: None,
            target_availability_set_id: None,
            target_availability_zone: None,
            target_proximity_placement_group_id: None,
            target_boot_diagnostics_storage_account_id: None,
            target_network_id: None,
            test_network_id: None,
            failover_recovery_point_id: None,
            last_recovery_point_received: None,
            last_rpo_in_seconds: None,
            last_rpo_calculated_time: None,
            last_recovery_point_id: None,
            initial_replication_progress_percentage: None,
            initial_replication_processed_bytes: None,
            initial_replication_transferred_bytes: None,
            initial_replication_progress_health: None,
            resync_progress_percentage: None,
            resync_processed_bytes: None,
            resync_transferred_bytes: None,
            resync_progress_health: None,
            resync_required: None,
            resync_state: None,
            agent_upgrade_state: None,
            last_agent_upgrade_type: None,
            agent_upgrade_job_id: None,
            agent_upgrade_attempt_to_version: None,
            protected_disks: Vec::new(),
            is_last_upgrade_successful: None,
            is_agent_registration_successful_after_failover: None,
            mobility_agent_details: None,
            last_agent_upgrade_error_details: Vec::new(),
            agent_upgrade_blocking_error_details: Vec::new(),
            vm_nics: Vec::new(),
            discovered_vm_details: None,
        }
    }
}
pub mod in_mage_rcm_replication_details {
    use super::*;
    #[doc = "The initial replication progress health."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "InitialReplicationProgressHealth")]
    pub enum InitialReplicationProgressHealth {
        None,
        InProgress,
        SlowProgress,
        NoProgress,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for InitialReplicationProgressHealth {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for InitialReplicationProgressHealth {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for InitialReplicationProgressHealth {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("InitialReplicationProgressHealth", 0u32, "None"),
                Self::InProgress => serializer.serialize_unit_variant("InitialReplicationProgressHealth", 1u32, "InProgress"),
                Self::SlowProgress => serializer.serialize_unit_variant("InitialReplicationProgressHealth", 2u32, "SlowProgress"),
                Self::NoProgress => serializer.serialize_unit_variant("InitialReplicationProgressHealth", 3u32, "NoProgress"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The resync progress health."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResyncProgressHealth")]
    pub enum ResyncProgressHealth {
        None,
        InProgress,
        SlowProgress,
        NoProgress,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResyncProgressHealth {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResyncProgressHealth {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResyncProgressHealth {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ResyncProgressHealth", 0u32, "None"),
                Self::InProgress => serializer.serialize_unit_variant("ResyncProgressHealth", 1u32, "InProgress"),
                Self::SlowProgress => serializer.serialize_unit_variant("ResyncProgressHealth", 2u32, "SlowProgress"),
                Self::NoProgress => serializer.serialize_unit_variant("ResyncProgressHealth", 3u32, "NoProgress"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The resync state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResyncState")]
    pub enum ResyncState {
        None,
        PreparedForResynchronization,
        StartedResynchronization,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResyncState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResyncState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResyncState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ResyncState", 0u32, "None"),
                Self::PreparedForResynchronization => {
                    serializer.serialize_unit_variant("ResyncState", 1u32, "PreparedForResynchronization")
                }
                Self::StartedResynchronization => serializer.serialize_unit_variant("ResyncState", 2u32, "StartedResynchronization"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The agent auto upgrade state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AgentUpgradeState")]
    pub enum AgentUpgradeState {
        None,
        Started,
        Completed,
        Commit,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AgentUpgradeState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AgentUpgradeState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AgentUpgradeState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("AgentUpgradeState", 0u32, "None"),
                Self::Started => serializer.serialize_unit_variant("AgentUpgradeState", 1u32, "Started"),
                Self::Completed => serializer.serialize_unit_variant("AgentUpgradeState", 2u32, "Completed"),
                Self::Commit => serializer.serialize_unit_variant("AgentUpgradeState", 3u32, "Commit"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "InMageRcm specific provider input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmReprotectInput {
    #[serde(flatten)]
    pub reverse_replication_provider_specific_input: ReverseReplicationProviderSpecificInput,
    #[doc = "The reprotect agent Id."]
    #[serde(rename = "reprotectAgentId")]
    pub reprotect_agent_id: String,
    #[doc = "The target datastore name."]
    #[serde(rename = "datastoreName")]
    pub datastore_name: String,
    #[doc = "The log storage account ARM Id."]
    #[serde(rename = "logStorageAccountId")]
    pub log_storage_account_id: String,
    #[doc = "The Policy Id."]
    #[serde(rename = "policyId", default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
}
impl InMageRcmReprotectInput {
    pub fn new(
        reverse_replication_provider_specific_input: ReverseReplicationProviderSpecificInput,
        reprotect_agent_id: String,
        datastore_name: String,
        log_storage_account_id: String,
    ) -> Self {
        Self {
            reverse_replication_provider_specific_input,
            reprotect_agent_id,
            datastore_name,
            log_storage_account_id,
            policy_id: None,
        }
    }
}
#[doc = "InMageRcm disk level sync details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageRcmSyncDetails {
    #[doc = "The progress health."]
    #[serde(rename = "progressHealth", default, skip_serializing_if = "Option::is_none")]
    pub progress_health: Option<in_mage_rcm_sync_details::ProgressHealth>,
    #[doc = "The transferred bytes from source VM to azure for the disk."]
    #[serde(rename = "transferredBytes", default, skip_serializing_if = "Option::is_none")]
    pub transferred_bytes: Option<i64>,
    #[doc = "The bytes transferred in last 15 minutes from source VM to azure."]
    #[serde(rename = "last15MinutesTransferredBytes", default, skip_serializing_if = "Option::is_none")]
    pub last15_minutes_transferred_bytes: Option<i64>,
    #[doc = "The time of the last data transfer from source VM to azure."]
    #[serde(rename = "lastDataTransferTimeUtc", default, skip_serializing_if = "Option::is_none")]
    pub last_data_transfer_time_utc: Option<String>,
    #[doc = "The total processed bytes. This includes bytes that are transferred from source VM to azure and matched bytes."]
    #[serde(rename = "processedBytes", default, skip_serializing_if = "Option::is_none")]
    pub processed_bytes: Option<i64>,
    #[doc = "The start time."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The last refresh time."]
    #[serde(rename = "lastRefreshTime", default, skip_serializing_if = "Option::is_none")]
    pub last_refresh_time: Option<String>,
    #[doc = "Progress in percentage. Progress percentage is calculated based on processed bytes."]
    #[serde(rename = "progressPercentage", default, skip_serializing_if = "Option::is_none")]
    pub progress_percentage: Option<i32>,
}
impl InMageRcmSyncDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod in_mage_rcm_sync_details {
    use super::*;
    #[doc = "The progress health."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProgressHealth")]
    pub enum ProgressHealth {
        None,
        InProgress,
        SlowProgress,
        NoProgress,
        Queued,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProgressHealth {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProgressHealth {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProgressHealth {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ProgressHealth", 0u32, "None"),
                Self::InProgress => serializer.serialize_unit_variant("ProgressHealth", 1u32, "InProgress"),
                Self::SlowProgress => serializer.serialize_unit_variant("ProgressHealth", 2u32, "SlowProgress"),
                Self::NoProgress => serializer.serialize_unit_variant("ProgressHealth", 3u32, "NoProgress"),
                Self::Queued => serializer.serialize_unit_variant("ProgressHealth", 4u32, "Queued"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "InMageRcm provider specific input for test failover."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmTestFailoverInput {
    #[serde(flatten)]
    pub test_failover_provider_specific_input: TestFailoverProviderSpecificInput,
    #[doc = "The test network Id."]
    #[serde(rename = "networkId", default, skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    #[doc = "The recovery point id to be passed to test failover to a particular recovery point. In case of latest recovery point, null should be passed."]
    #[serde(rename = "recoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_id: Option<String>,
}
impl InMageRcmTestFailoverInput {
    pub fn new(test_failover_provider_specific_input: TestFailoverProviderSpecificInput) -> Self {
        Self {
            test_failover_provider_specific_input,
            network_id: None,
            recovery_point_id: None,
        }
    }
}
#[doc = "InMageRcm provider specific input for unplanned failover."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmUnplannedFailoverInput {
    #[serde(flatten)]
    pub unplanned_failover_provider_specific_input: UnplannedFailoverProviderSpecificInput,
    #[doc = "A value indicating whether VM is to be shutdown."]
    #[serde(rename = "performShutdown")]
    pub perform_shutdown: String,
    #[doc = "The recovery point id to be passed to failover to a particular recovery point. In case of latest recovery point, null should be passed."]
    #[serde(rename = "recoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_id: Option<String>,
}
impl InMageRcmUnplannedFailoverInput {
    pub fn new(unplanned_failover_provider_specific_input: UnplannedFailoverProviderSpecificInput, perform_shutdown: String) -> Self {
        Self {
            unplanned_failover_provider_specific_input,
            perform_shutdown,
            recovery_point_id: None,
        }
    }
}
#[doc = "InMageRcm provider specific input to update appliance for replication protected item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmUpdateApplianceForReplicationProtectedItemInput {
    #[serde(flatten)]
    pub update_appliance_for_replication_protected_item_provider_specific_input:
        UpdateApplianceForReplicationProtectedItemProviderSpecificInput,
    #[doc = "The run as account Id."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
}
impl InMageRcmUpdateApplianceForReplicationProtectedItemInput {
    pub fn new(
        update_appliance_for_replication_protected_item_provider_specific_input : UpdateApplianceForReplicationProtectedItemProviderSpecificInput,
    ) -> Self {
        Self {
            update_appliance_for_replication_protected_item_provider_specific_input,
            run_as_account_id: None,
        }
    }
}
#[doc = "InMageRcm update protection container mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmUpdateContainerMappingInput {
    #[serde(flatten)]
    pub replication_provider_specific_update_container_mapping_input: ReplicationProviderSpecificUpdateContainerMappingInput,
    #[doc = "A value indicating whether agent auto upgrade has to be enabled."]
    #[serde(rename = "enableAgentAutoUpgrade")]
    pub enable_agent_auto_upgrade: String,
}
impl InMageRcmUpdateContainerMappingInput {
    pub fn new(
        replication_provider_specific_update_container_mapping_input: ReplicationProviderSpecificUpdateContainerMappingInput,
        enable_agent_auto_upgrade: String,
    ) -> Self {
        Self {
            replication_provider_specific_update_container_mapping_input,
            enable_agent_auto_upgrade,
        }
    }
}
#[doc = "InMageRcm provider specific input to update replication protected item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageRcmUpdateReplicationProtectedItemInput {
    #[serde(flatten)]
    pub update_replication_protected_item_provider_input: UpdateReplicationProtectedItemProviderInput,
    #[doc = "The target VM name."]
    #[serde(rename = "targetVmName", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_name: Option<String>,
    #[doc = "The target VM size."]
    #[serde(rename = "targetVmSize", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_size: Option<String>,
    #[doc = "The target resource group ARM Id."]
    #[serde(rename = "targetResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_group_id: Option<String>,
    #[doc = "The target availability set ARM Id."]
    #[serde(rename = "targetAvailabilitySetId", default, skip_serializing_if = "Option::is_none")]
    pub target_availability_set_id: Option<String>,
    #[doc = "The target availability zone."]
    #[serde(rename = "targetAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub target_availability_zone: Option<String>,
    #[doc = "The target proximity placement group Id."]
    #[serde(rename = "targetProximityPlacementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub target_proximity_placement_group_id: Option<String>,
    #[doc = "The target boot diagnostics storage account ARM Id."]
    #[serde(rename = "targetBootDiagnosticsStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub target_boot_diagnostics_storage_account_id: Option<String>,
    #[doc = "The target network ARM Id."]
    #[serde(rename = "targetNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub target_network_id: Option<String>,
    #[doc = "The test network ARM Id."]
    #[serde(rename = "testNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub test_network_id: Option<String>,
    #[doc = "The list of NIC details."]
    #[serde(rename = "vmNics", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_nics: Vec<InMageRcmNicInput>,
    #[doc = "The license type."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<in_mage_rcm_update_replication_protected_item_input::LicenseType>,
}
impl InMageRcmUpdateReplicationProtectedItemInput {
    pub fn new(update_replication_protected_item_provider_input: UpdateReplicationProtectedItemProviderInput) -> Self {
        Self {
            update_replication_protected_item_provider_input,
            target_vm_name: None,
            target_vm_size: None,
            target_resource_group_id: None,
            target_availability_set_id: None,
            target_availability_zone: None,
            target_proximity_placement_group_id: None,
            target_boot_diagnostics_storage_account_id: None,
            target_network_id: None,
            test_network_id: None,
            vm_nics: Vec::new(),
            license_type: None,
        }
    }
}
pub mod in_mage_rcm_update_replication_protected_item_input {
    use super::*;
    #[doc = "The license type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseType")]
    pub enum LicenseType {
        NotSpecified,
        NoLicenseType,
        WindowsServer,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("LicenseType", 0u32, "NotSpecified"),
                Self::NoLicenseType => serializer.serialize_unit_variant("LicenseType", 1u32, "NoLicenseType"),
                Self::WindowsServer => serializer.serialize_unit_variant("LicenseType", 2u32, "WindowsServer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "InMage provider specific settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageReplicationDetails {
    #[serde(flatten)]
    pub replication_provider_specific_settings: ReplicationProviderSpecificSettings,
    #[doc = "The active location of the VM. If the VM is being protected from Azure, this field will take values from { Azure, OnPrem }. If the VM is being protected between two data-centers, this field will be OnPrem always."]
    #[serde(rename = "activeSiteType", default, skip_serializing_if = "Option::is_none")]
    pub active_site_type: Option<String>,
    #[doc = "The CPU count of the VM on the primary side."]
    #[serde(rename = "sourceVmCpuCount", default, skip_serializing_if = "Option::is_none")]
    pub source_vm_cpu_count: Option<i32>,
    #[doc = "The RAM size of the VM on the primary side."]
    #[serde(rename = "sourceVmRamSizeInMB", default, skip_serializing_if = "Option::is_none")]
    pub source_vm_ram_size_in_mb: Option<i32>,
    #[doc = "Details of the OS Disk."]
    #[serde(rename = "osDetails", default, skip_serializing_if = "Option::is_none")]
    pub os_details: Option<OsDiskDetails>,
    #[doc = "The protection stage."]
    #[serde(rename = "protectionStage", default, skip_serializing_if = "Option::is_none")]
    pub protection_stage: Option<String>,
    #[doc = "The virtual machine Id."]
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[doc = "The protection state for the vm."]
    #[serde(rename = "vmProtectionState", default, skip_serializing_if = "Option::is_none")]
    pub vm_protection_state: Option<String>,
    #[doc = "The protection state description for the vm."]
    #[serde(rename = "vmProtectionStateDescription", default, skip_serializing_if = "Option::is_none")]
    pub vm_protection_state_description: Option<String>,
    #[doc = "Initial replication details."]
    #[serde(rename = "resyncDetails", default, skip_serializing_if = "Option::is_none")]
    pub resync_details: Option<InitialReplicationDetails>,
    #[doc = "The retention window start time."]
    #[serde(rename = "retentionWindowStart", with = "azure_core::date::rfc3339::option")]
    pub retention_window_start: Option<time::OffsetDateTime>,
    #[doc = "The retention window end time."]
    #[serde(rename = "retentionWindowEnd", with = "azure_core::date::rfc3339::option")]
    pub retention_window_end: Option<time::OffsetDateTime>,
    #[doc = "The compressed data change rate in MB."]
    #[serde(rename = "compressedDataRateInMB", default, skip_serializing_if = "Option::is_none")]
    pub compressed_data_rate_in_mb: Option<f64>,
    #[doc = "The uncompressed data change rate in MB."]
    #[serde(rename = "uncompressedDataRateInMB", default, skip_serializing_if = "Option::is_none")]
    pub uncompressed_data_rate_in_mb: Option<f64>,
    #[doc = "The RPO in seconds."]
    #[serde(rename = "rpoInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub rpo_in_seconds: Option<i64>,
    #[doc = "The list of protected disks."]
    #[serde(rename = "protectedDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub protected_disks: Vec<InMageProtectedDiskDetails>,
    #[doc = "The source IP address."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The last heartbeat received from the source server."]
    #[serde(rename = "lastHeartbeat", with = "azure_core::date::rfc3339::option")]
    pub last_heartbeat: Option<time::OffsetDateTime>,
    #[doc = "The process server Id."]
    #[serde(rename = "processServerId", default, skip_serializing_if = "Option::is_none")]
    pub process_server_id: Option<String>,
    #[doc = "The master target Id."]
    #[serde(rename = "masterTargetId", default, skip_serializing_if = "Option::is_none")]
    pub master_target_id: Option<String>,
    #[doc = "The collection of Consistency points."]
    #[serde(rename = "consistencyPoints", default, skip_serializing_if = "Option::is_none")]
    pub consistency_points: Option<serde_json::Value>,
    #[doc = "A value indicating whether any disk is resized for this VM."]
    #[serde(rename = "diskResized", default, skip_serializing_if = "Option::is_none")]
    pub disk_resized: Option<String>,
    #[doc = "A value indicating whether the source server requires a restart after update."]
    #[serde(rename = "rebootAfterUpdateStatus", default, skip_serializing_if = "Option::is_none")]
    pub reboot_after_update_status: Option<String>,
    #[doc = "The multi vm group Id, if any."]
    #[serde(rename = "multiVmGroupId", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_group_id: Option<String>,
    #[doc = "The multi vm group name, if any."]
    #[serde(rename = "multiVmGroupName", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_group_name: Option<String>,
    #[doc = "A value indicating whether the multi vm sync is enabled or disabled."]
    #[serde(rename = "multiVmSyncStatus", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_sync_status: Option<String>,
    #[doc = "The details of the InMage agent."]
    #[serde(rename = "agentDetails", default, skip_serializing_if = "Option::is_none")]
    pub agent_details: Option<InMageAgentDetails>,
    #[doc = "The vCenter infrastructure Id."]
    #[serde(rename = "vCenterInfrastructureId", default, skip_serializing_if = "Option::is_none")]
    pub v_center_infrastructure_id: Option<String>,
    #[doc = "The infrastructure VM Id."]
    #[serde(rename = "infrastructureVmId", default, skip_serializing_if = "Option::is_none")]
    pub infrastructure_vm_id: Option<String>,
    #[doc = "The PE Network details."]
    #[serde(rename = "vmNics", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_nics: Vec<VmNicDetails>,
    #[doc = "A value indicating the discovery type of the machine."]
    #[serde(rename = "discoveryType", default, skip_serializing_if = "Option::is_none")]
    pub discovery_type: Option<String>,
    #[doc = "A value indicating the underlying Azure storage account. If the VM is not running in Azure, this value shall be set to null."]
    #[serde(rename = "azureStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub azure_storage_account_id: Option<String>,
    #[doc = "The datastores of the on-premise machine Value can be list of strings that contain datastore names."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub datastores: Vec<String>,
    #[doc = "The validation errors of the on-premise machine Value can be list of validation errors."]
    #[serde(rename = "validationErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub validation_errors: Vec<HealthError>,
    #[doc = "The last RPO calculated time."]
    #[serde(rename = "lastRpoCalculatedTime", with = "azure_core::date::rfc3339::option")]
    pub last_rpo_calculated_time: Option<time::OffsetDateTime>,
    #[doc = "The last update time received from on-prem components."]
    #[serde(rename = "lastUpdateReceivedTime", with = "azure_core::date::rfc3339::option")]
    pub last_update_received_time: Option<time::OffsetDateTime>,
    #[doc = "The replica id of the protected item."]
    #[serde(rename = "replicaId", default, skip_serializing_if = "Option::is_none")]
    pub replica_id: Option<String>,
    #[doc = "The OS Version of the protected item."]
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[doc = "A value indicating whether additional IR stats are available or not."]
    #[serde(rename = "isAdditionalStatsAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_additional_stats_available: Option<bool>,
    #[doc = "The total transferred data in bytes."]
    #[serde(rename = "totalDataTransferred", default, skip_serializing_if = "Option::is_none")]
    pub total_data_transferred: Option<i64>,
    #[doc = "The progress health."]
    #[serde(rename = "totalProgressHealth", default, skip_serializing_if = "Option::is_none")]
    pub total_progress_health: Option<String>,
}
impl InMageReplicationDetails {
    pub fn new(replication_provider_specific_settings: ReplicationProviderSpecificSettings) -> Self {
        Self {
            replication_provider_specific_settings,
            active_site_type: None,
            source_vm_cpu_count: None,
            source_vm_ram_size_in_mb: None,
            os_details: None,
            protection_stage: None,
            vm_id: None,
            vm_protection_state: None,
            vm_protection_state_description: None,
            resync_details: None,
            retention_window_start: None,
            retention_window_end: None,
            compressed_data_rate_in_mb: None,
            uncompressed_data_rate_in_mb: None,
            rpo_in_seconds: None,
            protected_disks: Vec::new(),
            ip_address: None,
            last_heartbeat: None,
            process_server_id: None,
            master_target_id: None,
            consistency_points: None,
            disk_resized: None,
            reboot_after_update_status: None,
            multi_vm_group_id: None,
            multi_vm_group_name: None,
            multi_vm_sync_status: None,
            agent_details: None,
            v_center_infrastructure_id: None,
            infrastructure_vm_id: None,
            vm_nics: Vec::new(),
            discovery_type: None,
            azure_storage_account_id: None,
            datastores: Vec::new(),
            validation_errors: Vec::new(),
            last_rpo_calculated_time: None,
            last_update_received_time: None,
            replica_id: None,
            os_version: None,
            is_additional_stats_available: None,
            total_data_transferred: None,
            total_progress_health: None,
        }
    }
}
#[doc = "InMageAzureV2 specific provider input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageReprotectInput {
    #[serde(flatten)]
    pub reverse_replication_provider_specific_input: ReverseReplicationProviderSpecificInput,
    #[doc = "The Master Target Id."]
    #[serde(rename = "masterTargetId")]
    pub master_target_id: String,
    #[doc = "The Process Server Id."]
    #[serde(rename = "processServerId")]
    pub process_server_id: String,
    #[doc = "The retention drive to use on the MT."]
    #[serde(rename = "retentionDrive")]
    pub retention_drive: String,
    #[doc = "The CS account Id."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "The target datastore name."]
    #[serde(rename = "datastoreName", default, skip_serializing_if = "Option::is_none")]
    pub datastore_name: Option<String>,
    #[doc = "DiskExclusionInput when doing enable protection of virtual machine in InMage provider."]
    #[serde(rename = "diskExclusionInput", default, skip_serializing_if = "Option::is_none")]
    pub disk_exclusion_input: Option<InMageDiskExclusionInput>,
    #[doc = "The Policy Id."]
    #[serde(rename = "profileId")]
    pub profile_id: String,
    #[doc = "The disks to include list."]
    #[serde(rename = "disksToInclude", default, skip_serializing_if = "Vec::is_empty")]
    pub disks_to_include: Vec<String>,
}
impl InMageReprotectInput {
    pub fn new(
        reverse_replication_provider_specific_input: ReverseReplicationProviderSpecificInput,
        master_target_id: String,
        process_server_id: String,
        retention_drive: String,
        profile_id: String,
    ) -> Self {
        Self {
            reverse_replication_provider_specific_input,
            master_target_id,
            process_server_id,
            retention_drive,
            run_as_account_id: None,
            datastore_name: None,
            disk_exclusion_input: None,
            profile_id,
            disks_to_include: Vec::new(),
        }
    }
}
#[doc = "Provider specific input for InMage test failover."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageTestFailoverInput {
    #[serde(flatten)]
    pub test_failover_provider_specific_input: TestFailoverProviderSpecificInput,
    #[doc = "The recovery point type. Values from LatestTime, LatestTag or Custom. In the case of custom, the recovery point provided by RecoveryPointId will be used. In the other two cases, recovery point id will be ignored."]
    #[serde(rename = "recoveryPointType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_type: Option<in_mage_test_failover_input::RecoveryPointType>,
    #[doc = "The recovery point id to be passed to test failover to a particular recovery point. In case of latest recovery point, null should be passed."]
    #[serde(rename = "recoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_id: Option<String>,
}
impl InMageTestFailoverInput {
    pub fn new(test_failover_provider_specific_input: TestFailoverProviderSpecificInput) -> Self {
        Self {
            test_failover_provider_specific_input,
            recovery_point_type: None,
            recovery_point_id: None,
        }
    }
}
pub mod in_mage_test_failover_input {
    use super::*;
    #[doc = "The recovery point type. Values from LatestTime, LatestTag or Custom. In the case of custom, the recovery point provided by RecoveryPointId will be used. In the other two cases, recovery point id will be ignored."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryPointType")]
    pub enum RecoveryPointType {
        LatestTime,
        LatestTag,
        Custom,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryPointType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryPointType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryPointType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::LatestTime => serializer.serialize_unit_variant("RecoveryPointType", 0u32, "LatestTime"),
                Self::LatestTag => serializer.serialize_unit_variant("RecoveryPointType", 1u32, "LatestTag"),
                Self::Custom => serializer.serialize_unit_variant("RecoveryPointType", 2u32, "Custom"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Provider specific input for InMage unplanned failover."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InMageUnplannedFailoverInput {
    #[serde(flatten)]
    pub unplanned_failover_provider_specific_input: UnplannedFailoverProviderSpecificInput,
    #[doc = "The recovery point type. Values from LatestTime, LatestTag or Custom. In the case of custom, the recovery point provided by RecoveryPointId will be used. In the other two cases, recovery point id will be ignored."]
    #[serde(rename = "recoveryPointType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_type: Option<in_mage_unplanned_failover_input::RecoveryPointType>,
    #[doc = "The recovery point id to be passed to failover to a particular recovery point. In case of latest recovery point, null should be passed."]
    #[serde(rename = "recoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_id: Option<String>,
}
impl InMageUnplannedFailoverInput {
    pub fn new(unplanned_failover_provider_specific_input: UnplannedFailoverProviderSpecificInput) -> Self {
        Self {
            unplanned_failover_provider_specific_input,
            recovery_point_type: None,
            recovery_point_id: None,
        }
    }
}
pub mod in_mage_unplanned_failover_input {
    use super::*;
    #[doc = "The recovery point type. Values from LatestTime, LatestTag or Custom. In the case of custom, the recovery point provided by RecoveryPointId will be used. In the other two cases, recovery point id will be ignored."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryPointType")]
    pub enum RecoveryPointType {
        LatestTime,
        LatestTag,
        Custom,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryPointType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryPointType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryPointType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::LatestTime => serializer.serialize_unit_variant("RecoveryPointType", 0u32, "LatestTime"),
                Self::LatestTag => serializer.serialize_unit_variant("RecoveryPointType", 1u32, "LatestTag"),
                Self::Custom => serializer.serialize_unit_variant("RecoveryPointType", 2u32, "Custom"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Guest disk signature based disk exclusion option when doing enable protection of virtual machine in InMage provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InMageVolumeExclusionOptions {
    #[doc = "The volume label. The disk having any volume with this label will be excluded from replication."]
    #[serde(rename = "volumeLabel", default, skip_serializing_if = "Option::is_none")]
    pub volume_label: Option<String>,
    #[doc = "The value indicating whether to exclude multi volume disk or not. If a disk has multiple volumes and one of the volume has label matching with VolumeLabel this disk will be excluded from replication if OnlyExcludeIfSingleVolume is false."]
    #[serde(rename = "onlyExcludeIfSingleVolume", default, skip_serializing_if = "Option::is_none")]
    pub only_exclude_if_single_volume: Option<String>,
}
impl InMageVolumeExclusionOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This class stores the monitoring details for consistency check of inconsistent Protected Entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InconsistentVmDetails {
    #[doc = "The Vm name."]
    #[serde(rename = "vmName", default, skip_serializing_if = "Option::is_none")]
    pub vm_name: Option<String>,
    #[doc = "The Cloud name."]
    #[serde(rename = "cloudName", default, skip_serializing_if = "Option::is_none")]
    pub cloud_name: Option<String>,
    #[doc = "The list of details regarding state of the Protected Entity in SRS and On prem."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<String>,
    #[doc = "The list of error ids."]
    #[serde(rename = "errorIds", default, skip_serializing_if = "Vec::is_empty")]
    pub error_ids: Vec<String>,
}
impl InconsistentVmDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Initial replication details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InitialReplicationDetails {
    #[doc = "Initial replication type."]
    #[serde(rename = "initialReplicationType", default, skip_serializing_if = "Option::is_none")]
    pub initial_replication_type: Option<String>,
    #[doc = "The initial replication progress percentage."]
    #[serde(rename = "initialReplicationProgressPercentage", default, skip_serializing_if = "Option::is_none")]
    pub initial_replication_progress_percentage: Option<String>,
}
impl InitialReplicationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This class represents the inline workflow task details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineWorkflowTaskDetails {
    #[serde(flatten)]
    pub group_task_details: GroupTaskDetails,
    #[doc = "The list of child workflow ids."]
    #[serde(rename = "workflowIds", default, skip_serializing_if = "Vec::is_empty")]
    pub workflow_ids: Vec<String>,
}
impl InlineWorkflowTaskDetails {
    pub fn new(group_task_details: GroupTaskDetails) -> Self {
        Self {
            group_task_details,
            workflow_ids: Vec::new(),
        }
    }
}
#[doc = "Implements InnerHealthError class. HealthError object has a list of InnerHealthErrors as child errors. InnerHealthError is used because this will prevent an infinite loop of structures when Hydra tries to auto-generate the contract. We are exposing the related health errors as inner health errors and all API consumers can utilize this in the same fashion as Exception -&gt; InnerException."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InnerHealthError {
    #[doc = "Source of error."]
    #[serde(rename = "errorSource", default, skip_serializing_if = "Option::is_none")]
    pub error_source: Option<String>,
    #[doc = "Type of error."]
    #[serde(rename = "errorType", default, skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
    #[doc = "Level of error."]
    #[serde(rename = "errorLevel", default, skip_serializing_if = "Option::is_none")]
    pub error_level: Option<String>,
    #[doc = "Category of error."]
    #[serde(rename = "errorCategory", default, skip_serializing_if = "Option::is_none")]
    pub error_category: Option<String>,
    #[doc = "Error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "Summary message of the entity."]
    #[serde(rename = "summaryMessage", default, skip_serializing_if = "Option::is_none")]
    pub summary_message: Option<String>,
    #[doc = "Error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "Possible causes of error."]
    #[serde(rename = "possibleCauses", default, skip_serializing_if = "Option::is_none")]
    pub possible_causes: Option<String>,
    #[doc = "Recommended action to resolve error."]
    #[serde(rename = "recommendedAction", default, skip_serializing_if = "Option::is_none")]
    pub recommended_action: Option<String>,
    #[doc = "Error creation time (UTC)."]
    #[serde(rename = "creationTimeUtc", with = "azure_core::date::rfc3339::option")]
    pub creation_time_utc: Option<time::OffsetDateTime>,
    #[doc = "DRA error message."]
    #[serde(rename = "recoveryProviderErrorMessage", default, skip_serializing_if = "Option::is_none")]
    pub recovery_provider_error_message: Option<String>,
    #[doc = "ID of the entity."]
    #[serde(rename = "entityId", default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[doc = "The health error unique id."]
    #[serde(rename = "errorId", default, skip_serializing_if = "Option::is_none")]
    pub error_id: Option<String>,
    #[doc = "Value indicating whether the health error is customer resolvable."]
    #[serde(rename = "customerResolvability", default, skip_serializing_if = "Option::is_none")]
    pub customer_resolvability: Option<inner_health_error::CustomerResolvability>,
}
impl InnerHealthError {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod inner_health_error {
    use super::*;
    #[doc = "Value indicating whether the health error is customer resolvable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CustomerResolvability")]
    pub enum CustomerResolvability {
        Allowed,
        NotAllowed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CustomerResolvability {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CustomerResolvability {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CustomerResolvability {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allowed => serializer.serialize_unit_variant("CustomerResolvability", 0u32, "Allowed"),
                Self::NotAllowed => serializer.serialize_unit_variant("CustomerResolvability", 1u32, "NotAllowed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InputEndpoint {
    #[serde(rename = "endpointName", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,
    #[serde(rename = "privatePort", default, skip_serializing_if = "Option::is_none")]
    pub private_port: Option<i32>,
    #[serde(rename = "publicPort", default, skip_serializing_if = "Option::is_none")]
    pub public_port: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}
impl InputEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Job {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Job custom data details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<JobProperties>,
}
impl Job {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of jobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobCollection {
    #[doc = "The list of jobs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Job>,
    #[doc = "The value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for JobCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl JobCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job details based on specific job type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobDetails {
    #[doc = "Gets the type of job details (see JobDetailsTypes enum for possible values)."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
    #[doc = "The affected object properties like source server, source cloud, target server, target cloud etc. based on the workflow object details."]
    #[serde(rename = "affectedObjectDetails", default, skip_serializing_if = "Option::is_none")]
    pub affected_object_details: Option<serde_json::Value>,
}
impl JobDetails {
    pub fn new(instance_type: String) -> Self {
        Self {
            instance_type,
            affected_object_details: None,
        }
    }
}
#[doc = "This class contains the minimal job details required to navigate to the desired drill down."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobEntity {
    #[doc = "The job id."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "The job display name."]
    #[serde(rename = "jobFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub job_friendly_name: Option<String>,
    #[doc = "The object id."]
    #[serde(rename = "targetObjectId", default, skip_serializing_if = "Option::is_none")]
    pub target_object_id: Option<String>,
    #[doc = "The object name."]
    #[serde(rename = "targetObjectName", default, skip_serializing_if = "Option::is_none")]
    pub target_object_name: Option<String>,
    #[doc = "The workflow affected object type."]
    #[serde(rename = "targetInstanceType", default, skip_serializing_if = "Option::is_none")]
    pub target_instance_type: Option<String>,
    #[doc = "The job name. Enum type ScenarioName."]
    #[serde(rename = "jobScenarioName", default, skip_serializing_if = "Option::is_none")]
    pub job_scenario_name: Option<String>,
}
impl JobEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This class contains the error details per object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobErrorDetails {
    #[doc = "ASR error model."]
    #[serde(rename = "serviceErrorDetails", default, skip_serializing_if = "Option::is_none")]
    pub service_error_details: Option<ServiceError>,
    #[doc = "This class contains the error details per object."]
    #[serde(rename = "providerErrorDetails", default, skip_serializing_if = "Option::is_none")]
    pub provider_error_details: Option<ProviderError>,
    #[doc = "Error level of error."]
    #[serde(rename = "errorLevel", default, skip_serializing_if = "Option::is_none")]
    pub error_level: Option<String>,
    #[doc = "The creation time of job error."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "The Id of the task."]
    #[serde(rename = "taskId", default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}
impl JobErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Job custom data details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobProperties {
    #[doc = "The activity id."]
    #[serde(rename = "activityId", default, skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
    #[doc = "The ScenarioName."]
    #[serde(rename = "scenarioName", default, skip_serializing_if = "Option::is_none")]
    pub scenario_name: Option<String>,
    #[doc = "The DisplayName."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The status of the Job. It is one of these values - NotStarted, InProgress, Succeeded, Failed, Cancelled, Suspended or Other."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The description of the state of the Job. For e.g. - For Succeeded state, description can be Completed, PartiallySucceeded, CompletedWithInformation or Skipped."]
    #[serde(rename = "stateDescription", default, skip_serializing_if = "Option::is_none")]
    pub state_description: Option<String>,
    #[doc = "The tasks."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tasks: Vec<AsrTask>,
    #[doc = "The errors."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<JobErrorDetails>,
    #[doc = "The start time."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The Allowed action the job."]
    #[serde(rename = "allowedActions", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_actions: Vec<String>,
    #[doc = "The affected Object Id."]
    #[serde(rename = "targetObjectId", default, skip_serializing_if = "Option::is_none")]
    pub target_object_id: Option<String>,
    #[doc = "The name of the affected object."]
    #[serde(rename = "targetObjectName", default, skip_serializing_if = "Option::is_none")]
    pub target_object_name: Option<String>,
    #[doc = "The type of the affected object which is of Microsoft.Azure.SiteRecovery.V2015_11_10.AffectedObjectType class."]
    #[serde(rename = "targetInstanceType", default, skip_serializing_if = "Option::is_none")]
    pub target_instance_type: Option<String>,
    #[doc = "Job details based on specific job type."]
    #[serde(rename = "customDetails", default, skip_serializing_if = "Option::is_none")]
    pub custom_details: Option<JobDetails>,
}
impl JobProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Query parameter to enumerate jobs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobQueryParameter {
    #[doc = "Date time to get jobs from."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "Date time to get jobs upto."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The Id of the fabric to search jobs under."]
    #[serde(rename = "fabricId", default, skip_serializing_if = "Option::is_none")]
    pub fabric_id: Option<String>,
    #[doc = "The type of objects."]
    #[serde(rename = "affectedObjectTypes", default, skip_serializing_if = "Option::is_none")]
    pub affected_object_types: Option<String>,
    #[doc = "The states of the job to be filtered can be in."]
    #[serde(rename = "jobStatus", default, skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[doc = "The output type of the jobs."]
    #[serde(rename = "jobOutputType", default, skip_serializing_if = "Option::is_none")]
    pub job_output_type: Option<job_query_parameter::JobOutputType>,
    #[doc = "The job Name."]
    #[serde(rename = "jobName", default, skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[doc = "The timezone offset for the location of the request (in minutes)."]
    #[serde(rename = "timezoneOffset", default, skip_serializing_if = "Option::is_none")]
    pub timezone_offset: Option<f64>,
}
impl JobQueryParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_query_parameter {
    use super::*;
    #[doc = "The output type of the jobs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "JobOutputType")]
    pub enum JobOutputType {
        Json,
        Xml,
        Excel,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for JobOutputType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for JobOutputType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for JobOutputType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Json => serializer.serialize_unit_variant("JobOutputType", 0u32, "Json"),
                Self::Xml => serializer.serialize_unit_variant("JobOutputType", 1u32, "Xml"),
                Self::Excel => serializer.serialize_unit_variant("JobOutputType", 2u32, "Excel"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Model class for event details of a job status event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobStatusEventDetails {
    #[serde(flatten)]
    pub event_specific_details: EventSpecificDetails,
    #[doc = "Job arm id for the event."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "JobName for the Event."]
    #[serde(rename = "jobFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub job_friendly_name: Option<String>,
    #[doc = "JobStatus for the Event."]
    #[serde(rename = "jobStatus", default, skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[doc = "AffectedObjectType for the event."]
    #[serde(rename = "affectedObjectType", default, skip_serializing_if = "Option::is_none")]
    pub affected_object_type: Option<String>,
}
impl JobStatusEventDetails {
    pub fn new(event_specific_details: EventSpecificDetails) -> Self {
        Self {
            event_specific_details,
            job_id: None,
            job_friendly_name: None,
            job_status: None,
            affected_object_type: None,
        }
    }
}
#[doc = "This class represents a task which is actually a workflow so that one can navigate to its individual drill down."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobTaskDetails {
    #[serde(flatten)]
    pub task_type_details: TaskTypeDetails,
    #[doc = "This class contains the minimal job details required to navigate to the desired drill down."]
    #[serde(rename = "jobTask", default, skip_serializing_if = "Option::is_none")]
    pub job_task: Option<JobEntity>,
}
impl JobTaskDetails {
    pub fn new(task_type_details: TaskTypeDetails) -> Self {
        Self {
            task_type_details,
            job_task: None,
        }
    }
}
#[doc = "Key Encryption Key (KEK) information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyEncryptionKeyInfo {
    #[doc = "The key URL / identifier."]
    #[serde(rename = "keyIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub key_identifier: Option<String>,
    #[doc = "The KeyVault resource ARM Id for key."]
    #[serde(rename = "keyVaultResourceArmId", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_resource_arm_id: Option<String>,
}
impl KeyEncryptionKeyInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Logical network data model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogicalNetwork {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Logical Network Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LogicalNetworkProperties>,
}
impl LogicalNetwork {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of logical networks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogicalNetworkCollection {
    #[doc = "The Logical Networks list details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LogicalNetwork>,
    #[doc = "The value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LogicalNetworkCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl LogicalNetworkCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Logical Network Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogicalNetworkProperties {
    #[doc = "The Friendly Name."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "A value indicating whether Network Virtualization is enabled for the logical network."]
    #[serde(rename = "networkVirtualizationStatus", default, skip_serializing_if = "Option::is_none")]
    pub network_virtualization_status: Option<String>,
    #[doc = "A value indicating whether logical network is used as private test network by test failover."]
    #[serde(rename = "logicalNetworkUsage", default, skip_serializing_if = "Option::is_none")]
    pub logical_network_usage: Option<String>,
    #[doc = "A value indicating whether logical network definitions are isolated."]
    #[serde(rename = "logicalNetworkDefinitionsStatus", default, skip_serializing_if = "Option::is_none")]
    pub logical_network_definitions_status: Option<String>,
}
impl LogicalNetworkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This class represents the manual action task details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManualActionTaskDetails {
    #[serde(flatten)]
    pub task_type_details: TaskTypeDetails,
    #[doc = "The name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The instructions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[doc = "The observation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observation: Option<String>,
}
impl ManualActionTaskDetails {
    pub fn new(task_type_details: TaskTypeDetails) -> Self {
        Self {
            task_type_details,
            name: None,
            instructions: None,
            observation: None,
        }
    }
}
#[doc = "Mars agent details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MarsAgentDetails {
    #[doc = "The Mars agent Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The Mars agent name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The Mars agent Bios Id."]
    #[serde(rename = "biosId", default, skip_serializing_if = "Option::is_none")]
    pub bios_id: Option<String>,
    #[doc = "The fabric object Id."]
    #[serde(rename = "fabricObjectId", default, skip_serializing_if = "Option::is_none")]
    pub fabric_object_id: Option<String>,
    #[doc = "The Mars agent Fqdn."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "The version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The last heartbeat received from the Mars agent."]
    #[serde(rename = "lastHeartbeatUtc", with = "azure_core::date::rfc3339::option")]
    pub last_heartbeat_utc: Option<time::OffsetDateTime>,
    #[doc = "The health of the Mars agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<mars_agent_details::Health>,
    #[doc = "The health errors."]
    #[serde(rename = "healthErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub health_errors: Vec<HealthError>,
}
impl MarsAgentDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod mars_agent_details {
    use super::*;
    #[doc = "The health of the Mars agent."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Health")]
    pub enum Health {
        None,
        Normal,
        Warning,
        Critical,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Health {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Health {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Health {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Health", 0u32, "None"),
                Self::Normal => serializer.serialize_unit_variant("Health", 1u32, "Normal"),
                Self::Warning => serializer.serialize_unit_variant("Health", 2u32, "Warning"),
                Self::Critical => serializer.serialize_unit_variant("Health", 3u32, "Critical"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Details of a Master Target Server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MasterTargetServer {
    #[doc = "The server Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The IP address of the server."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The server name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The OS type of the server."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "The version of the scout component on the server."]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "The last heartbeat received from the server."]
    #[serde(rename = "lastHeartbeat", with = "azure_core::date::rfc3339::option")]
    pub last_heartbeat: Option<time::OffsetDateTime>,
    #[doc = "Version status."]
    #[serde(rename = "versionStatus", default, skip_serializing_if = "Option::is_none")]
    pub version_status: Option<String>,
    #[doc = "The retention volumes of Master target Server."]
    #[serde(rename = "retentionVolumes", default, skip_serializing_if = "Vec::is_empty")]
    pub retention_volumes: Vec<RetentionVolume>,
    #[doc = "The list of data stores in the fabric."]
    #[serde(rename = "dataStores", default, skip_serializing_if = "Vec::is_empty")]
    pub data_stores: Vec<DataStore>,
    #[doc = "Validation errors."]
    #[serde(rename = "validationErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub validation_errors: Vec<HealthError>,
    #[doc = "Health errors."]
    #[serde(rename = "healthErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub health_errors: Vec<HealthError>,
    #[doc = "Disk count of the master target."]
    #[serde(rename = "diskCount", default, skip_serializing_if = "Option::is_none")]
    pub disk_count: Option<i32>,
    #[doc = "OS Version of the master target."]
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[doc = "Agent expiry date."]
    #[serde(rename = "agentExpiryDate", with = "azure_core::date::rfc3339::option")]
    pub agent_expiry_date: Option<time::OffsetDateTime>,
    #[doc = "MARS agent version."]
    #[serde(rename = "marsAgentVersion", default, skip_serializing_if = "Option::is_none")]
    pub mars_agent_version: Option<String>,
    #[doc = "MARS agent expiry date."]
    #[serde(rename = "marsAgentExpiryDate", with = "azure_core::date::rfc3339::option")]
    pub mars_agent_expiry_date: Option<time::OffsetDateTime>,
    #[doc = "Version related details."]
    #[serde(rename = "agentVersionDetails", default, skip_serializing_if = "Option::is_none")]
    pub agent_version_details: Option<VersionDetails>,
    #[doc = "Version related details."]
    #[serde(rename = "marsAgentVersionDetails", default, skip_serializing_if = "Option::is_none")]
    pub mars_agent_version_details: Option<VersionDetails>,
}
impl MasterTargetServer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Input for migrate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateInput {
    #[doc = "Migrate input properties."]
    pub properties: MigrateInputProperties,
}
impl MigrateInput {
    pub fn new(properties: MigrateInputProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Migrate input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateInputProperties {
    #[doc = "Migrate provider specific input."]
    #[serde(rename = "providerSpecificDetails")]
    pub provider_specific_details: MigrateProviderSpecificInput,
}
impl MigrateInputProperties {
    pub fn new(provider_specific_details: MigrateProviderSpecificInput) -> Self {
        Self { provider_specific_details }
    }
}
#[doc = "Migrate provider specific input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrateProviderSpecificInput {
    #[doc = "The class type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl MigrateProviderSpecificInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Migration item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationItem {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Migration item properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MigrationItemProperties>,
}
impl MigrationItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Migration item collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationItemCollection {
    #[doc = "The list of migration items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MigrationItem>,
    #[doc = "The value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MigrationItemCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MigrationItemCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Migration item properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationItemProperties {
    #[doc = "The on-premise virtual machine name."]
    #[serde(rename = "machineName", default, skip_serializing_if = "Option::is_none")]
    pub machine_name: Option<String>,
    #[doc = "The ARM Id of policy governing this item."]
    #[serde(rename = "policyId", default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[doc = "The name of policy governing this item."]
    #[serde(rename = "policyFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub policy_friendly_name: Option<String>,
    #[doc = "The migration status."]
    #[serde(rename = "migrationState", default, skip_serializing_if = "Option::is_none")]
    pub migration_state: Option<migration_item_properties::MigrationState>,
    #[doc = "The migration state description."]
    #[serde(rename = "migrationStateDescription", default, skip_serializing_if = "Option::is_none")]
    pub migration_state_description: Option<String>,
    #[doc = "The last test migration time."]
    #[serde(rename = "lastTestMigrationTime", with = "azure_core::date::rfc3339::option")]
    pub last_test_migration_time: Option<time::OffsetDateTime>,
    #[doc = "The status of the last test migration."]
    #[serde(rename = "lastTestMigrationStatus", default, skip_serializing_if = "Option::is_none")]
    pub last_test_migration_status: Option<String>,
    #[doc = "The test migrate state."]
    #[serde(rename = "testMigrateState", default, skip_serializing_if = "Option::is_none")]
    pub test_migrate_state: Option<migration_item_properties::TestMigrateState>,
    #[doc = "The test migrate state description."]
    #[serde(rename = "testMigrateStateDescription", default, skip_serializing_if = "Option::is_none")]
    pub test_migrate_state_description: Option<String>,
    #[doc = "The consolidated health."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<migration_item_properties::Health>,
    #[doc = "The list of health errors."]
    #[serde(rename = "healthErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub health_errors: Vec<HealthError>,
    #[doc = "The allowed operations on the migration item based on the current migration state of the item."]
    #[serde(rename = "allowedOperations", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_operations: Vec<String>,
    #[doc = "Current job details of the migration item."]
    #[serde(rename = "currentJob", default, skip_serializing_if = "Option::is_none")]
    pub current_job: Option<CurrentJobDetails>,
    #[doc = "The correlation Id for events associated with this migration item."]
    #[serde(rename = "eventCorrelationId", default, skip_serializing_if = "Option::is_none")]
    pub event_correlation_id: Option<String>,
    #[doc = "Migration provider specific settings."]
    #[serde(rename = "providerSpecificDetails", default, skip_serializing_if = "Option::is_none")]
    pub provider_specific_details: Option<MigrationProviderSpecificSettings>,
}
impl MigrationItemProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod migration_item_properties {
    use super::*;
    #[doc = "The migration status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MigrationState")]
    pub enum MigrationState {
        None,
        EnableMigrationInProgress,
        EnableMigrationFailed,
        DisableMigrationInProgress,
        DisableMigrationFailed,
        InitialSeedingInProgress,
        InitialSeedingFailed,
        Replicating,
        MigrationInProgress,
        MigrationSucceeded,
        MigrationFailed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MigrationState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MigrationState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MigrationState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("MigrationState", 0u32, "None"),
                Self::EnableMigrationInProgress => serializer.serialize_unit_variant("MigrationState", 1u32, "EnableMigrationInProgress"),
                Self::EnableMigrationFailed => serializer.serialize_unit_variant("MigrationState", 2u32, "EnableMigrationFailed"),
                Self::DisableMigrationInProgress => serializer.serialize_unit_variant("MigrationState", 3u32, "DisableMigrationInProgress"),
                Self::DisableMigrationFailed => serializer.serialize_unit_variant("MigrationState", 4u32, "DisableMigrationFailed"),
                Self::InitialSeedingInProgress => serializer.serialize_unit_variant("MigrationState", 5u32, "InitialSeedingInProgress"),
                Self::InitialSeedingFailed => serializer.serialize_unit_variant("MigrationState", 6u32, "InitialSeedingFailed"),
                Self::Replicating => serializer.serialize_unit_variant("MigrationState", 7u32, "Replicating"),
                Self::MigrationInProgress => serializer.serialize_unit_variant("MigrationState", 8u32, "MigrationInProgress"),
                Self::MigrationSucceeded => serializer.serialize_unit_variant("MigrationState", 9u32, "MigrationSucceeded"),
                Self::MigrationFailed => serializer.serialize_unit_variant("MigrationState", 10u32, "MigrationFailed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The test migrate state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TestMigrateState")]
    pub enum TestMigrateState {
        None,
        TestMigrationInProgress,
        TestMigrationSucceeded,
        TestMigrationFailed,
        TestMigrationCleanupInProgress,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TestMigrateState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TestMigrateState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TestMigrateState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("TestMigrateState", 0u32, "None"),
                Self::TestMigrationInProgress => serializer.serialize_unit_variant("TestMigrateState", 1u32, "TestMigrationInProgress"),
                Self::TestMigrationSucceeded => serializer.serialize_unit_variant("TestMigrateState", 2u32, "TestMigrationSucceeded"),
                Self::TestMigrationFailed => serializer.serialize_unit_variant("TestMigrateState", 3u32, "TestMigrationFailed"),
                Self::TestMigrationCleanupInProgress => {
                    serializer.serialize_unit_variant("TestMigrateState", 4u32, "TestMigrationCleanupInProgress")
                }
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The consolidated health."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Health")]
    pub enum Health {
        None,
        Normal,
        Warning,
        Critical,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Health {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Health {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Health {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Health", 0u32, "None"),
                Self::Normal => serializer.serialize_unit_variant("Health", 1u32, "Normal"),
                Self::Warning => serializer.serialize_unit_variant("Health", 2u32, "Warning"),
                Self::Critical => serializer.serialize_unit_variant("Health", 3u32, "Critical"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Query parameter to enumerate migration items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationItemsQueryParameter {
    #[doc = "The source fabric name filter."]
    #[serde(rename = "sourceFabricName", default, skip_serializing_if = "Option::is_none")]
    pub source_fabric_name: Option<String>,
    #[doc = "The source container name filter."]
    #[serde(rename = "sourceContainerName", default, skip_serializing_if = "Option::is_none")]
    pub source_container_name: Option<String>,
    #[doc = "The replication provider type."]
    #[serde(rename = "instanceType", default, skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
}
impl MigrationItemsQueryParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Migration provider specific settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MigrationProviderSpecificSettings {
    #[doc = "Gets the instance type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl MigrationProviderSpecificSettings {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Recovery point for a migration item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationRecoveryPoint {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Migration item recovery point properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MigrationRecoveryPointProperties>,
}
impl MigrationRecoveryPoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of migration recovery points."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationRecoveryPointCollection {
    #[doc = "The migration recovery point details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MigrationRecoveryPoint>,
    #[doc = "The value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MigrationRecoveryPointCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MigrationRecoveryPointCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Migration item recovery point properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MigrationRecoveryPointProperties {
    #[doc = "The recovery point time."]
    #[serde(rename = "recoveryPointTime", with = "azure_core::date::rfc3339::option")]
    pub recovery_point_time: Option<time::OffsetDateTime>,
    #[doc = "The recovery point type."]
    #[serde(rename = "recoveryPointType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_type: Option<migration_recovery_point_properties::RecoveryPointType>,
}
impl MigrationRecoveryPointProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod migration_recovery_point_properties {
    use super::*;
    #[doc = "The recovery point type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryPointType")]
    pub enum RecoveryPointType {
        NotSpecified,
        ApplicationConsistent,
        CrashConsistent,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryPointType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryPointType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryPointType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("RecoveryPointType", 0u32, "NotSpecified"),
                Self::ApplicationConsistent => serializer.serialize_unit_variant("RecoveryPointType", 1u32, "ApplicationConsistent"),
                Self::CrashConsistent => serializer.serialize_unit_variant("RecoveryPointType", 2u32, "CrashConsistent"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Mobility Service update details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MobilityServiceUpdate {
    #[doc = "The version of the latest update."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The reboot status of the update - whether it is required or not."]
    #[serde(rename = "rebootStatus", default, skip_serializing_if = "Option::is_none")]
    pub reboot_status: Option<String>,
    #[doc = "The OS type."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
}
impl MobilityServiceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Network {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Network Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkProperties>,
}
impl Network {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of networks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkCollection {
    #[doc = "The Networks list details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Network>,
    #[doc = "The value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NetworkCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network Mapping model. Ideally it should have been possible to inherit this class from prev version in InheritedModels as long as there is no difference in structure or method signature. Since there were no base Models for certain fields and methods viz NetworkMappingProperties and Load with required return type, the class has been introduced in its entirety with references to base models to facilitate extensions in subsequent versions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkMapping {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Network Mapping Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkMappingProperties>,
}
impl NetworkMapping {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of network mappings. As with NetworkMapping, it should be possible to reuse a prev version of this class. It doesn't seem likely this class could be anything more than a slightly bespoke collection of NetworkMapping. Hence it makes sense to override Load with Base.NetworkMapping instead of existing CurrentVersion.NetworkMapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkMappingCollection {
    #[doc = "The Network Mappings list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NetworkMapping>,
    #[doc = "The value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkMappingCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NetworkMappingCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network Mapping fabric specific settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkMappingFabricSpecificSettings {
    #[doc = "Gets the Instance type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl NetworkMappingFabricSpecificSettings {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Network Mapping Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkMappingProperties {
    #[doc = "The pairing state for network mapping."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The primary network friendly name."]
    #[serde(rename = "primaryNetworkFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub primary_network_friendly_name: Option<String>,
    #[doc = "The primary network id for network mapping."]
    #[serde(rename = "primaryNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub primary_network_id: Option<String>,
    #[doc = "The primary fabric friendly name."]
    #[serde(rename = "primaryFabricFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub primary_fabric_friendly_name: Option<String>,
    #[doc = "The recovery network friendly name."]
    #[serde(rename = "recoveryNetworkFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub recovery_network_friendly_name: Option<String>,
    #[doc = "The recovery network id for network mapping."]
    #[serde(rename = "recoveryNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_network_id: Option<String>,
    #[doc = "The recovery fabric ARM id."]
    #[serde(rename = "recoveryFabricArmId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_fabric_arm_id: Option<String>,
    #[doc = "The recovery fabric friendly name."]
    #[serde(rename = "recoveryFabricFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub recovery_fabric_friendly_name: Option<String>,
    #[doc = "Network Mapping fabric specific settings."]
    #[serde(rename = "fabricSpecificSettings", default, skip_serializing_if = "Option::is_none")]
    pub fabric_specific_settings: Option<NetworkMappingFabricSpecificSettings>,
}
impl NetworkMappingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkProperties {
    #[doc = "The Fabric Type."]
    #[serde(rename = "fabricType", default, skip_serializing_if = "Option::is_none")]
    pub fabric_type: Option<String>,
    #[doc = "The List of subnets."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subnets: Vec<Subnet>,
    #[doc = "The Friendly Name."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The Network Type."]
    #[serde(rename = "networkType", default, skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
}
impl NetworkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "New Protection profile input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NewProtectionProfile {
    #[serde(flatten)]
    pub protection_profile_custom_details: ProtectionProfileCustomDetails,
    #[doc = "The protection profile input."]
    #[serde(rename = "policyName")]
    pub policy_name: String,
    #[doc = "The duration in minutes until which the recovery points need to be stored."]
    #[serde(rename = "recoveryPointHistory", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_history: Option<i32>,
    #[doc = "The crash consistent snapshot frequency (in minutes)."]
    #[serde(rename = "crashConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub crash_consistent_frequency_in_minutes: Option<i32>,
    #[doc = "The app consistent snapshot frequency (in minutes)."]
    #[serde(rename = "appConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub app_consistent_frequency_in_minutes: Option<i32>,
    #[doc = "A value indicating whether multi-VM sync has to be enabled. Value should be 'Enabled' or 'Disabled'."]
    #[serde(rename = "multiVmSyncStatus")]
    pub multi_vm_sync_status: new_protection_profile::MultiVmSyncStatus,
}
impl NewProtectionProfile {
    pub fn new(
        protection_profile_custom_details: ProtectionProfileCustomDetails,
        policy_name: String,
        multi_vm_sync_status: new_protection_profile::MultiVmSyncStatus,
    ) -> Self {
        Self {
            protection_profile_custom_details,
            policy_name,
            recovery_point_history: None,
            crash_consistent_frequency_in_minutes: None,
            app_consistent_frequency_in_minutes: None,
            multi_vm_sync_status,
        }
    }
}
pub mod new_protection_profile {
    use super::*;
    #[doc = "A value indicating whether multi-VM sync has to be enabled. Value should be 'Enabled' or 'Disabled'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MultiVmSyncStatus")]
    pub enum MultiVmSyncStatus {
        Enable,
        Disable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MultiVmSyncStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MultiVmSyncStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MultiVmSyncStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enable => serializer.serialize_unit_variant("MultiVmSyncStatus", 0u32, "Enable"),
                Self::Disable => serializer.serialize_unit_variant("MultiVmSyncStatus", 1u32, "Disable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Recovery virtual network input to create new virtual network from given source network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NewRecoveryVirtualNetwork {
    #[serde(flatten)]
    pub recovery_virtual_network_custom_details: RecoveryVirtualNetworkCustomDetails,
    #[doc = "The name of the resource group to be used to create the recovery virtual network. If absent, target network would be created in the same resource group as target VM."]
    #[serde(
        rename = "recoveryVirtualNetworkResourceGroupName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recovery_virtual_network_resource_group_name: Option<String>,
    #[doc = "The recovery virtual network name."]
    #[serde(rename = "recoveryVirtualNetworkName", default, skip_serializing_if = "Option::is_none")]
    pub recovery_virtual_network_name: Option<String>,
}
impl NewRecoveryVirtualNetwork {
    pub fn new(recovery_virtual_network_custom_details: RecoveryVirtualNetworkCustomDetails) -> Self {
        Self {
            recovery_virtual_network_custom_details,
            recovery_virtual_network_resource_group_name: None,
            recovery_virtual_network_name: None,
        }
    }
}
#[doc = "Disk Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsDetails {
    #[doc = "VM Disk details."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "Product type."]
    #[serde(rename = "productType", default, skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    #[doc = "The OSEdition."]
    #[serde(rename = "osEdition", default, skip_serializing_if = "Option::is_none")]
    pub os_edition: Option<String>,
    #[doc = "The OS Version."]
    #[serde(rename = "oSVersion", default, skip_serializing_if = "Option::is_none")]
    pub o_s_version: Option<String>,
    #[doc = "The OS Major Version."]
    #[serde(rename = "oSMajorVersion", default, skip_serializing_if = "Option::is_none")]
    pub o_s_major_version: Option<String>,
    #[doc = "The OS Minor Version."]
    #[serde(rename = "oSMinorVersion", default, skip_serializing_if = "Option::is_none")]
    pub o_s_minor_version: Option<String>,
}
impl OsDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the OS Disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsDiskDetails {
    #[doc = "The id of the disk containing the OS."]
    #[serde(rename = "osVhdId", default, skip_serializing_if = "Option::is_none")]
    pub os_vhd_id: Option<String>,
    #[doc = "The type of the OS on the VM."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "The OS disk VHD name."]
    #[serde(rename = "vhdName", default, skip_serializing_if = "Option::is_none")]
    pub vhd_name: Option<String>,
}
impl OsDiskDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Wrapper model for OSVersion to include version and service pack info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsVersionWrapper {
    #[doc = "The version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The service pack."]
    #[serde(rename = "servicePack", default, skip_serializing_if = "Option::is_none")]
    pub service_pack: Option<String>,
}
impl OsVersionWrapper {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operations discovery class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsDiscovery {
    #[doc = "Name of the API. The name of the operation being performed on this particular object. It should match the action name that appears in RBAC / the event service. Examples of operations include: * Microsoft.Compute/virtualMachine/capture/action * Microsoft.Compute/virtualMachine/restart/action * Microsoft.Compute/virtualMachine/write * Microsoft.Compute/virtualMachine/read * Microsoft.Compute/virtualMachine/delete Each action should include, in order: (1) Resource Provider Namespace (2) Type hierarchy for which the action applies (e.g. server/databases for a SQL Azure database) (3) Read, Write, Action or Delete indicating which type applies. If it is a PUT/PATCH on a collection or named value, Write should be used. If it is a GET, Read should be used. If it is a DELETE, Delete should be used. If it is a POST, Action should be used. As a note: all resource providers would need to include the \"{Resource Provider Namespace}/register/action\" operation in their response. This API is used to register for their service, and should include details about the operation (e.g. a localized name for the resource provider + any special considerations like PII release)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Contains the localized display information for this particular operation / action. These value will be used by several clients for (1) custom role definitions for RBAC; (2) complex query filters for the event service; and (3) audit history / records for management operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Display>,
    #[doc = "Origin. The intended executor of the operation; governs the display of the operation in the RBAC UX and the audit logs UX. Default value is \"user,system\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "ClientDiscovery properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationsDiscoveryProperties>,
}
impl OperationsDiscovery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of ClientDiscovery details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsDiscoveryCollection {
    #[doc = "The ClientDiscovery details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationsDiscovery>,
    #[doc = "The value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationsDiscoveryCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationsDiscoveryCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ClientDiscovery properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsDiscoveryProperties {}
impl OperationsDiscoveryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Input definition for planned failover."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlannedFailoverInput {
    #[doc = "Input definition for planned failover input properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PlannedFailoverInputProperties>,
}
impl PlannedFailoverInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Input definition for planned failover input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlannedFailoverInputProperties {
    #[doc = "Failover direction."]
    #[serde(rename = "failoverDirection", default, skip_serializing_if = "Option::is_none")]
    pub failover_direction: Option<String>,
    #[doc = "Provider specific failover input."]
    #[serde(rename = "providerSpecificDetails", default, skip_serializing_if = "Option::is_none")]
    pub provider_specific_details: Option<PlannedFailoverProviderSpecificFailoverInput>,
}
impl PlannedFailoverInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provider specific failover input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlannedFailoverProviderSpecificFailoverInput {
    #[doc = "The class type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl PlannedFailoverProviderSpecificFailoverInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Protection profile details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Policy {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Protection profile custom data details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PolicyProperties>,
}
impl Policy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Protection Profile Collection details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyCollection {
    #[doc = "The policy details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Policy>,
    #[doc = "The value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PolicyCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PolicyCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Protection profile custom data details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyProperties {
    #[doc = "The FriendlyName."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Base class for Provider specific details for policies."]
    #[serde(rename = "providerSpecificDetails", default, skip_serializing_if = "Option::is_none")]
    pub provider_specific_details: Option<PolicyProviderSpecificDetails>,
}
impl PolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for Provider specific details for policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyProviderSpecificDetails {
    #[doc = "Gets the class type. Overridden in derived classes."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl PolicyProviderSpecificDetails {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Base class for provider specific input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyProviderSpecificInput {
    #[doc = "The class type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl PolicyProviderSpecificInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Details of the Process Server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessServer {
    #[doc = "The Process Server's friendly name."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The Process Server Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The IP address of the server."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The OS type of the server."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "The version of the scout component on the server."]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "The last heartbeat received from the server."]
    #[serde(rename = "lastHeartbeat", with = "azure_core::date::rfc3339::option")]
    pub last_heartbeat: Option<time::OffsetDateTime>,
    #[doc = "Version status."]
    #[serde(rename = "versionStatus", default, skip_serializing_if = "Option::is_none")]
    pub version_status: Option<String>,
    #[doc = "The list of the mobility service updates available on the Process Server."]
    #[serde(rename = "mobilityServiceUpdates", default, skip_serializing_if = "Vec::is_empty")]
    pub mobility_service_updates: Vec<MobilityServiceUpdate>,
    #[doc = "The agent generated Id."]
    #[serde(rename = "hostId", default, skip_serializing_if = "Option::is_none")]
    pub host_id: Option<String>,
    #[doc = "The servers configured with this PS."]
    #[serde(rename = "machineCount", default, skip_serializing_if = "Option::is_none")]
    pub machine_count: Option<String>,
    #[doc = "The number of replication pairs configured in this PS."]
    #[serde(rename = "replicationPairCount", default, skip_serializing_if = "Option::is_none")]
    pub replication_pair_count: Option<String>,
    #[doc = "The percentage of the system load."]
    #[serde(rename = "systemLoad", default, skip_serializing_if = "Option::is_none")]
    pub system_load: Option<String>,
    #[doc = "The system load status."]
    #[serde(rename = "systemLoadStatus", default, skip_serializing_if = "Option::is_none")]
    pub system_load_status: Option<String>,
    #[doc = "The percentage of the CPU load."]
    #[serde(rename = "cpuLoad", default, skip_serializing_if = "Option::is_none")]
    pub cpu_load: Option<String>,
    #[doc = "The CPU load status."]
    #[serde(rename = "cpuLoadStatus", default, skip_serializing_if = "Option::is_none")]
    pub cpu_load_status: Option<String>,
    #[doc = "The total memory."]
    #[serde(rename = "totalMemoryInBytes", default, skip_serializing_if = "Option::is_none")]
    pub total_memory_in_bytes: Option<i64>,
    #[doc = "The available memory."]
    #[serde(rename = "availableMemoryInBytes", default, skip_serializing_if = "Option::is_none")]
    pub available_memory_in_bytes: Option<i64>,
    #[doc = "The memory usage status."]
    #[serde(rename = "memoryUsageStatus", default, skip_serializing_if = "Option::is_none")]
    pub memory_usage_status: Option<String>,
    #[doc = "The total space."]
    #[serde(rename = "totalSpaceInBytes", default, skip_serializing_if = "Option::is_none")]
    pub total_space_in_bytes: Option<i64>,
    #[doc = "The available space."]
    #[serde(rename = "availableSpaceInBytes", default, skip_serializing_if = "Option::is_none")]
    pub available_space_in_bytes: Option<i64>,
    #[doc = "The space usage status."]
    #[serde(rename = "spaceUsageStatus", default, skip_serializing_if = "Option::is_none")]
    pub space_usage_status: Option<String>,
    #[doc = "The PS service status."]
    #[serde(rename = "psServiceStatus", default, skip_serializing_if = "Option::is_none")]
    pub ps_service_status: Option<String>,
    #[doc = "The PS SSL cert expiry date."]
    #[serde(rename = "sslCertExpiryDate", with = "azure_core::date::rfc3339::option")]
    pub ssl_cert_expiry_date: Option<time::OffsetDateTime>,
    #[doc = "CS SSL cert expiry date."]
    #[serde(rename = "sslCertExpiryRemainingDays", default, skip_serializing_if = "Option::is_none")]
    pub ssl_cert_expiry_remaining_days: Option<i32>,
    #[doc = "OS Version of the process server. Note: This will get populated if user has CS version greater than 9.12.0.0."]
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[doc = "Health errors."]
    #[serde(rename = "healthErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub health_errors: Vec<HealthError>,
    #[doc = "Agent expiry date."]
    #[serde(rename = "agentExpiryDate", with = "azure_core::date::rfc3339::option")]
    pub agent_expiry_date: Option<time::OffsetDateTime>,
    #[doc = "Version related details."]
    #[serde(rename = "agentVersionDetails", default, skip_serializing_if = "Option::is_none")]
    pub agent_version_details: Option<VersionDetails>,
    #[doc = "The health of Process Server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<process_server::Health>,
    #[doc = "The process server stats refresh time."]
    #[serde(rename = "psStatsRefreshTime", with = "azure_core::date::rfc3339::option")]
    pub ps_stats_refresh_time: Option<time::OffsetDateTime>,
    #[doc = "The uploading pending data in bytes."]
    #[serde(rename = "throughputUploadPendingDataInBytes", default, skip_serializing_if = "Option::is_none")]
    pub throughput_upload_pending_data_in_bytes: Option<i64>,
    #[doc = "The throughput in MBps."]
    #[serde(rename = "throughputInMBps", default, skip_serializing_if = "Option::is_none")]
    pub throughput_in_m_bps: Option<i64>,
    #[doc = "The throughput in bytes."]
    #[serde(rename = "throughputInBytes", default, skip_serializing_if = "Option::is_none")]
    pub throughput_in_bytes: Option<i64>,
    #[doc = "The throughput status."]
    #[serde(rename = "throughputStatus", default, skip_serializing_if = "Option::is_none")]
    pub throughput_status: Option<String>,
    #[doc = "The MARS communication status."]
    #[serde(rename = "marsCommunicationStatus", default, skip_serializing_if = "Option::is_none")]
    pub mars_communication_status: Option<String>,
    #[doc = "The MARS registration status."]
    #[serde(rename = "marsRegistrationStatus", default, skip_serializing_if = "Option::is_none")]
    pub mars_registration_status: Option<String>,
}
impl ProcessServer {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod process_server {
    use super::*;
    #[doc = "The health of Process Server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Health")]
    pub enum Health {
        None,
        Normal,
        Warning,
        Critical,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Health {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Health {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Health {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Health", 0u32, "None"),
                Self::Normal => serializer.serialize_unit_variant("Health", 1u32, "Normal"),
                Self::Warning => serializer.serialize_unit_variant("Health", 2u32, "Warning"),
                Self::Critical => serializer.serialize_unit_variant("Health", 3u32, "Critical"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Process server details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProcessServerDetails {
    #[doc = "The process server Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The process server name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The process server Bios Id."]
    #[serde(rename = "biosId", default, skip_serializing_if = "Option::is_none")]
    pub bios_id: Option<String>,
    #[doc = "The fabric object Id."]
    #[serde(rename = "fabricObjectId", default, skip_serializing_if = "Option::is_none")]
    pub fabric_object_id: Option<String>,
    #[doc = "The process server Fqdn."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "The list of IP addresses for communicating with the RCM component."]
    #[serde(rename = "ipAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_addresses: Vec<String>,
    #[doc = "The version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The last heartbeat received from the process server."]
    #[serde(rename = "lastHeartbeatUtc", with = "azure_core::date::rfc3339::option")]
    pub last_heartbeat_utc: Option<time::OffsetDateTime>,
    #[doc = "The total memory."]
    #[serde(rename = "totalMemoryInBytes", default, skip_serializing_if = "Option::is_none")]
    pub total_memory_in_bytes: Option<i64>,
    #[doc = "The available memory."]
    #[serde(rename = "availableMemoryInBytes", default, skip_serializing_if = "Option::is_none")]
    pub available_memory_in_bytes: Option<i64>,
    #[doc = "The used memory."]
    #[serde(rename = "usedMemoryInBytes", default, skip_serializing_if = "Option::is_none")]
    pub used_memory_in_bytes: Option<i64>,
    #[doc = "The memory usage percentage."]
    #[serde(rename = "memoryUsagePercentage", default, skip_serializing_if = "Option::is_none")]
    pub memory_usage_percentage: Option<f64>,
    #[doc = "The total disk space."]
    #[serde(rename = "totalSpaceInBytes", default, skip_serializing_if = "Option::is_none")]
    pub total_space_in_bytes: Option<i64>,
    #[doc = "The available disk space."]
    #[serde(rename = "availableSpaceInBytes", default, skip_serializing_if = "Option::is_none")]
    pub available_space_in_bytes: Option<i64>,
    #[doc = "The used disk space."]
    #[serde(rename = "usedSpaceInBytes", default, skip_serializing_if = "Option::is_none")]
    pub used_space_in_bytes: Option<i64>,
    #[doc = "The free disk space percentage."]
    #[serde(rename = "freeSpacePercentage", default, skip_serializing_if = "Option::is_none")]
    pub free_space_percentage: Option<f64>,
    #[doc = "The uploading pending data in bytes."]
    #[serde(rename = "throughputUploadPendingDataInBytes", default, skip_serializing_if = "Option::is_none")]
    pub throughput_upload_pending_data_in_bytes: Option<i64>,
    #[doc = "The throughput in bytes."]
    #[serde(rename = "throughputInBytes", default, skip_serializing_if = "Option::is_none")]
    pub throughput_in_bytes: Option<i64>,
    #[doc = "The processor usage percentage."]
    #[serde(rename = "processorUsagePercentage", default, skip_serializing_if = "Option::is_none")]
    pub processor_usage_percentage: Option<f64>,
    #[doc = "The throughput status."]
    #[serde(rename = "throughputStatus", default, skip_serializing_if = "Option::is_none")]
    pub throughput_status: Option<process_server_details::ThroughputStatus>,
    #[doc = "The system load."]
    #[serde(rename = "systemLoad", default, skip_serializing_if = "Option::is_none")]
    pub system_load: Option<i64>,
    #[doc = "The system load status."]
    #[serde(rename = "systemLoadStatus", default, skip_serializing_if = "Option::is_none")]
    pub system_load_status: Option<process_server_details::SystemLoadStatus>,
    #[doc = "The disk usage status."]
    #[serde(rename = "diskUsageStatus", default, skip_serializing_if = "Option::is_none")]
    pub disk_usage_status: Option<process_server_details::DiskUsageStatus>,
    #[doc = "The memory usage status."]
    #[serde(rename = "memoryUsageStatus", default, skip_serializing_if = "Option::is_none")]
    pub memory_usage_status: Option<process_server_details::MemoryUsageStatus>,
    #[doc = "The processor usage status."]
    #[serde(rename = "processorUsageStatus", default, skip_serializing_if = "Option::is_none")]
    pub processor_usage_status: Option<process_server_details::ProcessorUsageStatus>,
    #[doc = "The health of the process server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<process_server_details::Health>,
    #[doc = "The health errors."]
    #[serde(rename = "healthErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub health_errors: Vec<HealthError>,
    #[doc = "The protected item count."]
    #[serde(rename = "protectedItemCount", default, skip_serializing_if = "Option::is_none")]
    pub protected_item_count: Option<i32>,
    #[doc = "The historic health of the process server based on the health in last 24 hours."]
    #[serde(rename = "historicHealth", default, skip_serializing_if = "Option::is_none")]
    pub historic_health: Option<process_server_details::HistoricHealth>,
}
impl ProcessServerDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod process_server_details {
    use super::*;
    #[doc = "The throughput status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ThroughputStatus")]
    pub enum ThroughputStatus {
        Healthy,
        Warning,
        Critical,
        Unknown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ThroughputStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ThroughputStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ThroughputStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Healthy => serializer.serialize_unit_variant("ThroughputStatus", 0u32, "Healthy"),
                Self::Warning => serializer.serialize_unit_variant("ThroughputStatus", 1u32, "Warning"),
                Self::Critical => serializer.serialize_unit_variant("ThroughputStatus", 2u32, "Critical"),
                Self::Unknown => serializer.serialize_unit_variant("ThroughputStatus", 3u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The system load status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SystemLoadStatus")]
    pub enum SystemLoadStatus {
        Healthy,
        Warning,
        Critical,
        Unknown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SystemLoadStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SystemLoadStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SystemLoadStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Healthy => serializer.serialize_unit_variant("SystemLoadStatus", 0u32, "Healthy"),
                Self::Warning => serializer.serialize_unit_variant("SystemLoadStatus", 1u32, "Warning"),
                Self::Critical => serializer.serialize_unit_variant("SystemLoadStatus", 2u32, "Critical"),
                Self::Unknown => serializer.serialize_unit_variant("SystemLoadStatus", 3u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The disk usage status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DiskUsageStatus")]
    pub enum DiskUsageStatus {
        Healthy,
        Warning,
        Critical,
        Unknown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DiskUsageStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DiskUsageStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DiskUsageStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Healthy => serializer.serialize_unit_variant("DiskUsageStatus", 0u32, "Healthy"),
                Self::Warning => serializer.serialize_unit_variant("DiskUsageStatus", 1u32, "Warning"),
                Self::Critical => serializer.serialize_unit_variant("DiskUsageStatus", 2u32, "Critical"),
                Self::Unknown => serializer.serialize_unit_variant("DiskUsageStatus", 3u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The memory usage status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MemoryUsageStatus")]
    pub enum MemoryUsageStatus {
        Healthy,
        Warning,
        Critical,
        Unknown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MemoryUsageStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MemoryUsageStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MemoryUsageStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Healthy => serializer.serialize_unit_variant("MemoryUsageStatus", 0u32, "Healthy"),
                Self::Warning => serializer.serialize_unit_variant("MemoryUsageStatus", 1u32, "Warning"),
                Self::Critical => serializer.serialize_unit_variant("MemoryUsageStatus", 2u32, "Critical"),
                Self::Unknown => serializer.serialize_unit_variant("MemoryUsageStatus", 3u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The processor usage status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProcessorUsageStatus")]
    pub enum ProcessorUsageStatus {
        Healthy,
        Warning,
        Critical,
        Unknown,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProcessorUsageStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProcessorUsageStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProcessorUsageStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Healthy => serializer.serialize_unit_variant("ProcessorUsageStatus", 0u32, "Healthy"),
                Self::Warning => serializer.serialize_unit_variant("ProcessorUsageStatus", 1u32, "Warning"),
                Self::Critical => serializer.serialize_unit_variant("ProcessorUsageStatus", 2u32, "Critical"),
                Self::Unknown => serializer.serialize_unit_variant("ProcessorUsageStatus", 3u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The health of the process server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Health")]
    pub enum Health {
        None,
        Normal,
        Warning,
        Critical,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Health {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Health {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Health {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Health", 0u32, "None"),
                Self::Normal => serializer.serialize_unit_variant("Health", 1u32, "Normal"),
                Self::Warning => serializer.serialize_unit_variant("Health", 2u32, "Warning"),
                Self::Critical => serializer.serialize_unit_variant("Health", 3u32, "Critical"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The historic health of the process server based on the health in last 24 hours."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HistoricHealth")]
    pub enum HistoricHealth {
        None,
        Normal,
        Warning,
        Critical,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HistoricHealth {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HistoricHealth {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HistoricHealth {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("HistoricHealth", 0u32, "None"),
                Self::Normal => serializer.serialize_unit_variant("HistoricHealth", 1u32, "Normal"),
                Self::Warning => serializer.serialize_unit_variant("HistoricHealth", 2u32, "Warning"),
                Self::Critical => serializer.serialize_unit_variant("HistoricHealth", 3u32, "Critical"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Replication protected item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectableItem {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Replication protected item custom data details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProtectableItemProperties>,
}
impl ProtectableItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Protectable item collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectableItemCollection {
    #[doc = "The Protectable item details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProtectableItem>,
    #[doc = "The value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProtectableItemCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProtectableItemCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Replication protected item custom data details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectableItemProperties {
    #[doc = "The name."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The protection status."]
    #[serde(rename = "protectionStatus", default, skip_serializing_if = "Option::is_none")]
    pub protection_status: Option<String>,
    #[doc = "The ARM resource of protected items."]
    #[serde(rename = "replicationProtectedItemId", default, skip_serializing_if = "Option::is_none")]
    pub replication_protected_item_id: Option<String>,
    #[doc = "The recovery provider ARM Id."]
    #[serde(rename = "recoveryServicesProviderId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_services_provider_id: Option<String>,
    #[doc = "The Current protection readiness errors."]
    #[serde(rename = "protectionReadinessErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub protection_readiness_errors: Vec<String>,
    #[doc = "The list of replication providers supported for the protectable item."]
    #[serde(rename = "supportedReplicationProviders", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_replication_providers: Vec<String>,
    #[doc = "Replication provider specific settings."]
    #[serde(rename = "customDetails", default, skip_serializing_if = "Option::is_none")]
    pub custom_details: Option<ConfigurationSettings>,
}
impl ProtectableItemProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Query parameter to enumerate Protectable items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectableItemQueryParameter {
    #[doc = "State of the Protectable item query filter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl ProtectableItemQueryParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Query parameter to enumerate protected items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectedItemsQueryParameter {
    #[doc = "The source fabric name filter."]
    #[serde(rename = "sourceFabricName", default, skip_serializing_if = "Option::is_none")]
    pub source_fabric_name: Option<String>,
    #[doc = "The recovery plan filter."]
    #[serde(rename = "recoveryPlanName", default, skip_serializing_if = "Option::is_none")]
    pub recovery_plan_name: Option<String>,
    #[doc = "The source fabric location filter."]
    #[serde(rename = "sourceFabricLocation", default, skip_serializing_if = "Option::is_none")]
    pub source_fabric_location: Option<String>,
    #[doc = "The fabric object Id filter."]
    #[serde(rename = "fabricObjectId", default, skip_serializing_if = "Option::is_none")]
    pub fabric_object_id: Option<String>,
    #[doc = "The vCenter name filter."]
    #[serde(rename = "vCenterName", default, skip_serializing_if = "Option::is_none")]
    pub v_center_name: Option<String>,
    #[doc = "The replication provider type."]
    #[serde(rename = "instanceType", default, skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[doc = "Whether Multi VM group is auto created or specified by user."]
    #[serde(rename = "multiVmGroupCreateOption", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_group_create_option: Option<protected_items_query_parameter::MultiVmGroupCreateOption>,
    #[doc = "The process server Id filter."]
    #[serde(rename = "processServerId", default, skip_serializing_if = "Option::is_none")]
    pub process_server_id: Option<String>,
}
impl ProtectedItemsQueryParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod protected_items_query_parameter {
    use super::*;
    #[doc = "Whether Multi VM group is auto created or specified by user."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MultiVmGroupCreateOption")]
    pub enum MultiVmGroupCreateOption {
        AutoCreated,
        UserSpecified,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MultiVmGroupCreateOption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MultiVmGroupCreateOption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MultiVmGroupCreateOption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AutoCreated => serializer.serialize_unit_variant("MultiVmGroupCreateOption", 0u32, "AutoCreated"),
                Self::UserSpecified => serializer.serialize_unit_variant("MultiVmGroupCreateOption", 1u32, "UserSpecified"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Protection container details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectionContainer {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Protection profile custom data details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProtectionContainerProperties>,
}
impl ProtectionContainer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Protection Container collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectionContainerCollection {
    #[doc = "The Protection Container details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProtectionContainer>,
    #[doc = "The value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProtectionContainerCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProtectionContainerCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class for fabric specific details of container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectionContainerFabricSpecificDetails {
    #[doc = "Gets the class type. Overridden in derived classes."]
    #[serde(rename = "instanceType", default, skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
}
impl ProtectionContainerFabricSpecificDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Protection container mapping object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectionContainerMapping {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Protection container mapping properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProtectionContainerMappingProperties>,
}
impl ProtectionContainerMapping {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Protection container mapping collection class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectionContainerMappingCollection {
    #[doc = "List of container mappings."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProtectionContainerMapping>,
    #[doc = "Link to fetch rest of the data."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProtectionContainerMappingCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProtectionContainerMappingCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Protection container mapping properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectionContainerMappingProperties {
    #[doc = "Paired protection container ARM ID."]
    #[serde(rename = "targetProtectionContainerId", default, skip_serializing_if = "Option::is_none")]
    pub target_protection_container_id: Option<String>,
    #[doc = "Friendly name of paired container."]
    #[serde(rename = "targetProtectionContainerFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub target_protection_container_friendly_name: Option<String>,
    #[doc = "Container mapping provider specific details."]
    #[serde(rename = "providerSpecificDetails", default, skip_serializing_if = "Option::is_none")]
    pub provider_specific_details: Option<ProtectionContainerMappingProviderSpecificDetails>,
    #[doc = "Health of pairing."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,
    #[doc = "Health error."]
    #[serde(rename = "healthErrorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub health_error_details: Vec<HealthError>,
    #[doc = "Policy ARM Id."]
    #[serde(rename = "policyId", default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[doc = "Association Status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Friendly name of source protection container."]
    #[serde(rename = "sourceProtectionContainerFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub source_protection_container_friendly_name: Option<String>,
    #[doc = "Friendly name of source fabric."]
    #[serde(rename = "sourceFabricFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub source_fabric_friendly_name: Option<String>,
    #[doc = "Friendly name of target fabric."]
    #[serde(rename = "targetFabricFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub target_fabric_friendly_name: Option<String>,
    #[doc = "Friendly name of replication policy."]
    #[serde(rename = "policyFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub policy_friendly_name: Option<String>,
}
impl ProtectionContainerMappingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container mapping provider specific details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtectionContainerMappingProviderSpecificDetails {
    #[doc = "Gets the class type. Overridden in derived classes."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl ProtectionContainerMappingProviderSpecificDetails {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Protection profile custom data details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProtectionContainerProperties {
    #[doc = "Fabric friendly name."]
    #[serde(rename = "fabricFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub fabric_friendly_name: Option<String>,
    #[doc = "The name."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The fabric type."]
    #[serde(rename = "fabricType", default, skip_serializing_if = "Option::is_none")]
    pub fabric_type: Option<String>,
    #[doc = "Number of protected PEs."]
    #[serde(rename = "protectedItemCount", default, skip_serializing_if = "Option::is_none")]
    pub protected_item_count: Option<i32>,
    #[doc = "The pairing status of this cloud."]
    #[serde(rename = "pairingStatus", default, skip_serializing_if = "Option::is_none")]
    pub pairing_status: Option<String>,
    #[doc = "The role of this cloud."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[doc = "Base class for fabric specific details of container."]
    #[serde(rename = "fabricSpecificDetails", default, skip_serializing_if = "Option::is_none")]
    pub fabric_specific_details: Option<ProtectionContainerFabricSpecificDetails>,
}
impl ProtectionContainerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Protection Profile custom input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtectionProfileCustomDetails {
    #[doc = "The class type."]
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}
impl ProtectionProfileCustomDetails {
    pub fn new(resource_type: String) -> Self {
        Self { resource_type }
    }
}
#[doc = "This class contains the error details per object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderError {
    #[doc = "The Error code."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[doc = "The Error message."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "The Provider error Id."]
    #[serde(rename = "errorId", default, skip_serializing_if = "Option::is_none")]
    pub error_id: Option<String>,
    #[doc = "The possible causes for the error."]
    #[serde(rename = "possibleCauses", default, skip_serializing_if = "Option::is_none")]
    pub possible_causes: Option<String>,
    #[doc = "The recommended action to resolve the error."]
    #[serde(rename = "recommendedAction", default, skip_serializing_if = "Option::is_none")]
    pub recommended_action: Option<String>,
}
impl ProviderError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Replication provider specific recovery point details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProviderSpecificRecoveryPointDetails {
    #[doc = "Gets the provider type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl ProviderSpecificRecoveryPointDetails {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Push installer details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PushInstallerDetails {
    #[doc = "The push installer Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The push installer name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The push installer Bios Id."]
    #[serde(rename = "biosId", default, skip_serializing_if = "Option::is_none")]
    pub bios_id: Option<String>,
    #[doc = "The fabric object Id."]
    #[serde(rename = "fabricObjectId", default, skip_serializing_if = "Option::is_none")]
    pub fabric_object_id: Option<String>,
    #[doc = "The push installer Fqdn."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "The version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The last heartbeat received from the push installer."]
    #[serde(rename = "lastHeartbeatUtc", with = "azure_core::date::rfc3339::option")]
    pub last_heartbeat_utc: Option<time::OffsetDateTime>,
    #[doc = "The health of the push installer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<push_installer_details::Health>,
    #[doc = "The health errors."]
    #[serde(rename = "healthErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub health_errors: Vec<HealthError>,
}
impl PushInstallerDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod push_installer_details {
    use super::*;
    #[doc = "The health of the push installer."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Health")]
    pub enum Health {
        None,
        Normal,
        Warning,
        Critical,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Health {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Health {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Health {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Health", 0u32, "None"),
                Self::Normal => serializer.serialize_unit_variant("Health", 1u32, "Normal"),
                Self::Warning => serializer.serialize_unit_variant("Health", 2u32, "Warning"),
                Self::Critical => serializer.serialize_unit_variant("Health", 3u32, "Critical"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "RCM proxy details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RcmProxyDetails {
    #[doc = "The RCM proxy Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The RCM proxy name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The RCM proxy Bios Id."]
    #[serde(rename = "biosId", default, skip_serializing_if = "Option::is_none")]
    pub bios_id: Option<String>,
    #[doc = "The fabric object Id."]
    #[serde(rename = "fabricObjectId", default, skip_serializing_if = "Option::is_none")]
    pub fabric_object_id: Option<String>,
    #[doc = "The RCM proxy Fqdn."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "The client authentication type."]
    #[serde(rename = "clientAuthenticationType", default, skip_serializing_if = "Option::is_none")]
    pub client_authentication_type: Option<String>,
    #[doc = "The version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The last heartbeat received from the RCM proxy."]
    #[serde(rename = "lastHeartbeatUtc", with = "azure_core::date::rfc3339::option")]
    pub last_heartbeat_utc: Option<time::OffsetDateTime>,
    #[doc = "The health of the RCM proxy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<rcm_proxy_details::Health>,
    #[doc = "The health errors."]
    #[serde(rename = "healthErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub health_errors: Vec<HealthError>,
}
impl RcmProxyDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod rcm_proxy_details {
    use super::*;
    #[doc = "The health of the RCM proxy."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Health")]
    pub enum Health {
        None,
        Normal,
        Warning,
        Critical,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Health {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Health {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Health {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Health", 0u32, "None"),
                Self::Normal => serializer.serialize_unit_variant("Health", 1u32, "Normal"),
                Self::Warning => serializer.serialize_unit_variant("Health", 2u32, "Warning"),
                Self::Critical => serializer.serialize_unit_variant("Health", 3u32, "Critical"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Recovery Availability Set custom input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryAvailabilitySetCustomDetails {
    #[doc = "The class type."]
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}
impl RecoveryAvailabilitySetCustomDetails {
    pub fn new(resource_type: String) -> Self {
        Self { resource_type }
    }
}
#[doc = "Recovery plan details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoveryPlan {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Recovery plan properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecoveryPlanProperties>,
}
impl RecoveryPlan {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Recovery plan A2A specific details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanA2aDetails {
    #[serde(flatten)]
    pub recovery_plan_provider_specific_details: RecoveryPlanProviderSpecificDetails,
    #[doc = "The primary zone."]
    #[serde(rename = "primaryZone", default, skip_serializing_if = "Option::is_none")]
    pub primary_zone: Option<String>,
    #[doc = "The recovery zone."]
    #[serde(rename = "recoveryZone", default, skip_serializing_if = "Option::is_none")]
    pub recovery_zone: Option<String>,
}
impl RecoveryPlanA2aDetails {
    pub fn new(recovery_plan_provider_specific_details: RecoveryPlanProviderSpecificDetails) -> Self {
        Self {
            recovery_plan_provider_specific_details,
            primary_zone: None,
            recovery_zone: None,
        }
    }
}
#[doc = "Recovery plan A2A failover input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanA2aFailoverInput {
    #[serde(flatten)]
    pub recovery_plan_provider_specific_failover_input: RecoveryPlanProviderSpecificFailoverInput,
    #[doc = "The recovery point type."]
    #[serde(rename = "recoveryPointType")]
    pub recovery_point_type: recovery_plan_a2a_failover_input::RecoveryPointType,
    #[doc = "A value indicating whether to use recovery cloud service for TFO or not."]
    #[serde(rename = "cloudServiceCreationOption", default, skip_serializing_if = "Option::is_none")]
    pub cloud_service_creation_option: Option<String>,
    #[doc = "A value indicating whether multi VM sync enabled VMs should use multi VM sync points for failover."]
    #[serde(rename = "multiVmSyncPointOption", default, skip_serializing_if = "Option::is_none")]
    pub multi_vm_sync_point_option: Option<recovery_plan_a2a_failover_input::MultiVmSyncPointOption>,
}
impl RecoveryPlanA2aFailoverInput {
    pub fn new(
        recovery_plan_provider_specific_failover_input: RecoveryPlanProviderSpecificFailoverInput,
        recovery_point_type: recovery_plan_a2a_failover_input::RecoveryPointType,
    ) -> Self {
        Self {
            recovery_plan_provider_specific_failover_input,
            recovery_point_type,
            cloud_service_creation_option: None,
            multi_vm_sync_point_option: None,
        }
    }
}
pub mod recovery_plan_a2a_failover_input {
    use super::*;
    #[doc = "The recovery point type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryPointType")]
    pub enum RecoveryPointType {
        Latest,
        LatestApplicationConsistent,
        LatestCrashConsistent,
        LatestProcessed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryPointType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryPointType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryPointType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Latest => serializer.serialize_unit_variant("RecoveryPointType", 0u32, "Latest"),
                Self::LatestApplicationConsistent => {
                    serializer.serialize_unit_variant("RecoveryPointType", 1u32, "LatestApplicationConsistent")
                }
                Self::LatestCrashConsistent => serializer.serialize_unit_variant("RecoveryPointType", 2u32, "LatestCrashConsistent"),
                Self::LatestProcessed => serializer.serialize_unit_variant("RecoveryPointType", 3u32, "LatestProcessed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "A value indicating whether multi VM sync enabled VMs should use multi VM sync points for failover."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MultiVmSyncPointOption")]
    pub enum MultiVmSyncPointOption {
        UseMultiVmSyncRecoveryPoint,
        UsePerVmRecoveryPoint,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MultiVmSyncPointOption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MultiVmSyncPointOption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MultiVmSyncPointOption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::UseMultiVmSyncRecoveryPoint => {
                    serializer.serialize_unit_variant("MultiVmSyncPointOption", 0u32, "UseMultiVmSyncRecoveryPoint")
                }
                Self::UsePerVmRecoveryPoint => serializer.serialize_unit_variant("MultiVmSyncPointOption", 1u32, "UsePerVmRecoveryPoint"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Recovery plan A2A input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanA2aInput {
    #[serde(flatten)]
    pub recovery_plan_provider_specific_input: RecoveryPlanProviderSpecificInput,
    #[doc = "The primary zone."]
    #[serde(rename = "primaryZone", default, skip_serializing_if = "Option::is_none")]
    pub primary_zone: Option<String>,
    #[doc = "The recovery zone."]
    #[serde(rename = "recoveryZone", default, skip_serializing_if = "Option::is_none")]
    pub recovery_zone: Option<String>,
    #[doc = "Extended location of the resource."]
    #[serde(rename = "primaryExtendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub primary_extended_location: Option<ExtendedLocation>,
    #[doc = "Extended location of the resource."]
    #[serde(rename = "recoveryExtendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub recovery_extended_location: Option<ExtendedLocation>,
}
impl RecoveryPlanA2aInput {
    pub fn new(recovery_plan_provider_specific_input: RecoveryPlanProviderSpecificInput) -> Self {
        Self {
            recovery_plan_provider_specific_input,
            primary_zone: None,
            recovery_zone: None,
            primary_extended_location: None,
            recovery_extended_location: None,
        }
    }
}
#[doc = "Recovery plan action details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanAction {
    #[doc = "The action name."]
    #[serde(rename = "actionName")]
    pub action_name: String,
    #[doc = "The list of failover types."]
    #[serde(rename = "failoverTypes")]
    pub failover_types: Vec<String>,
    #[doc = "The list of failover directions."]
    #[serde(rename = "failoverDirections")]
    pub failover_directions: Vec<String>,
    #[doc = "Recovery plan action custom details."]
    #[serde(rename = "customDetails")]
    pub custom_details: RecoveryPlanActionDetails,
}
impl RecoveryPlanAction {
    pub fn new(
        action_name: String,
        failover_types: Vec<String>,
        failover_directions: Vec<String>,
        custom_details: RecoveryPlanActionDetails,
    ) -> Self {
        Self {
            action_name,
            failover_types,
            failover_directions,
            custom_details,
        }
    }
}
#[doc = "Recovery plan action custom details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanActionDetails {
    #[doc = "Gets the type of action details (see RecoveryPlanActionDetailsTypes enum for possible values)."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl RecoveryPlanActionDetails {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Recovery plan Automation runbook action details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanAutomationRunbookActionDetails {
    #[serde(flatten)]
    pub recovery_plan_action_details: RecoveryPlanActionDetails,
    #[doc = "The runbook ARM Id."]
    #[serde(rename = "runbookId", default, skip_serializing_if = "Option::is_none")]
    pub runbook_id: Option<String>,
    #[doc = "The runbook timeout."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[doc = "The fabric location."]
    #[serde(rename = "fabricLocation")]
    pub fabric_location: recovery_plan_automation_runbook_action_details::FabricLocation,
}
impl RecoveryPlanAutomationRunbookActionDetails {
    pub fn new(
        recovery_plan_action_details: RecoveryPlanActionDetails,
        fabric_location: recovery_plan_automation_runbook_action_details::FabricLocation,
    ) -> Self {
        Self {
            recovery_plan_action_details,
            runbook_id: None,
            timeout: None,
            fabric_location,
        }
    }
}
pub mod recovery_plan_automation_runbook_action_details {
    use super::*;
    #[doc = "The fabric location."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FabricLocation")]
    pub enum FabricLocation {
        Primary,
        Recovery,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FabricLocation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FabricLocation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FabricLocation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Primary => serializer.serialize_unit_variant("FabricLocation", 0u32, "Primary"),
                Self::Recovery => serializer.serialize_unit_variant("FabricLocation", 1u32, "Recovery"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Recovery plan collection details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoveryPlanCollection {
    #[doc = "The list of recovery plans."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RecoveryPlan>,
    #[doc = "The value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RecoveryPlanCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RecoveryPlanCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Recovery plan group details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanGroup {
    #[doc = "The group type."]
    #[serde(rename = "groupType")]
    pub group_type: recovery_plan_group::GroupType,
    #[doc = "The list of protected items."]
    #[serde(rename = "replicationProtectedItems", default, skip_serializing_if = "Vec::is_empty")]
    pub replication_protected_items: Vec<RecoveryPlanProtectedItem>,
    #[doc = "The start group actions."]
    #[serde(rename = "startGroupActions", default, skip_serializing_if = "Vec::is_empty")]
    pub start_group_actions: Vec<RecoveryPlanAction>,
    #[doc = "The end group actions."]
    #[serde(rename = "endGroupActions", default, skip_serializing_if = "Vec::is_empty")]
    pub end_group_actions: Vec<RecoveryPlanAction>,
}
impl RecoveryPlanGroup {
    pub fn new(group_type: recovery_plan_group::GroupType) -> Self {
        Self {
            group_type,
            replication_protected_items: Vec::new(),
            start_group_actions: Vec::new(),
            end_group_actions: Vec::new(),
        }
    }
}
pub mod recovery_plan_group {
    use super::*;
    #[doc = "The group type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "GroupType")]
    pub enum GroupType {
        Shutdown,
        Boot,
        Failover,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for GroupType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for GroupType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for GroupType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Shutdown => serializer.serialize_unit_variant("GroupType", 0u32, "Shutdown"),
                Self::Boot => serializer.serialize_unit_variant("GroupType", 1u32, "Boot"),
                Self::Failover => serializer.serialize_unit_variant("GroupType", 2u32, "Failover"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "This class represents the recovery plan group task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanGroupTaskDetails {
    #[serde(flatten)]
    pub group_task_details: GroupTaskDetails,
    #[doc = "The name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The group identifier."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The group type."]
    #[serde(rename = "rpGroupType", default, skip_serializing_if = "Option::is_none")]
    pub rp_group_type: Option<String>,
}
impl RecoveryPlanGroupTaskDetails {
    pub fn new(group_task_details: GroupTaskDetails) -> Self {
        Self {
            group_task_details,
            name: None,
            group_id: None,
            rp_group_type: None,
        }
    }
}
#[doc = "Recovery plan HVR Azure failback input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanHyperVReplicaAzureFailbackInput {
    #[serde(flatten)]
    pub recovery_plan_provider_specific_failover_input: RecoveryPlanProviderSpecificFailoverInput,
    #[doc = "The data sync option."]
    #[serde(rename = "dataSyncOption")]
    pub data_sync_option: recovery_plan_hyper_v_replica_azure_failback_input::DataSyncOption,
    #[doc = "The ALR option."]
    #[serde(rename = "recoveryVmCreationOption")]
    pub recovery_vm_creation_option: recovery_plan_hyper_v_replica_azure_failback_input::RecoveryVmCreationOption,
}
impl RecoveryPlanHyperVReplicaAzureFailbackInput {
    pub fn new(
        recovery_plan_provider_specific_failover_input: RecoveryPlanProviderSpecificFailoverInput,
        data_sync_option: recovery_plan_hyper_v_replica_azure_failback_input::DataSyncOption,
        recovery_vm_creation_option: recovery_plan_hyper_v_replica_azure_failback_input::RecoveryVmCreationOption,
    ) -> Self {
        Self {
            recovery_plan_provider_specific_failover_input,
            data_sync_option,
            recovery_vm_creation_option,
        }
    }
}
pub mod recovery_plan_hyper_v_replica_azure_failback_input {
    use super::*;
    #[doc = "The data sync option."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataSyncOption")]
    pub enum DataSyncOption {
        ForDownTime,
        ForSynchronization,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataSyncOption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataSyncOption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataSyncOption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ForDownTime => serializer.serialize_unit_variant("DataSyncOption", 0u32, "ForDownTime"),
                Self::ForSynchronization => serializer.serialize_unit_variant("DataSyncOption", 1u32, "ForSynchronization"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The ALR option."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryVmCreationOption")]
    pub enum RecoveryVmCreationOption {
        CreateVmIfNotFound,
        NoAction,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryVmCreationOption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryVmCreationOption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryVmCreationOption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::CreateVmIfNotFound => serializer.serialize_unit_variant("RecoveryVmCreationOption", 0u32, "CreateVmIfNotFound"),
                Self::NoAction => serializer.serialize_unit_variant("RecoveryVmCreationOption", 1u32, "NoAction"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Recovery plan HVR Azure failover input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanHyperVReplicaAzureFailoverInput {
    #[serde(flatten)]
    pub recovery_plan_provider_specific_failover_input: RecoveryPlanProviderSpecificFailoverInput,
    #[doc = "The primary KEK certificate PFX."]
    #[serde(rename = "primaryKekCertificatePfx", default, skip_serializing_if = "Option::is_none")]
    pub primary_kek_certificate_pfx: Option<String>,
    #[doc = "The secondary KEK certificate PFX."]
    #[serde(rename = "secondaryKekCertificatePfx", default, skip_serializing_if = "Option::is_none")]
    pub secondary_kek_certificate_pfx: Option<String>,
    #[doc = "The recovery point type."]
    #[serde(rename = "recoveryPointType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_type: Option<recovery_plan_hyper_v_replica_azure_failover_input::RecoveryPointType>,
}
impl RecoveryPlanHyperVReplicaAzureFailoverInput {
    pub fn new(recovery_plan_provider_specific_failover_input: RecoveryPlanProviderSpecificFailoverInput) -> Self {
        Self {
            recovery_plan_provider_specific_failover_input,
            primary_kek_certificate_pfx: None,
            secondary_kek_certificate_pfx: None,
            recovery_point_type: None,
        }
    }
}
pub mod recovery_plan_hyper_v_replica_azure_failover_input {
    use super::*;
    #[doc = "The recovery point type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryPointType")]
    pub enum RecoveryPointType {
        Latest,
        LatestApplicationConsistent,
        LatestProcessed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryPointType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryPointType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryPointType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Latest => serializer.serialize_unit_variant("RecoveryPointType", 0u32, "Latest"),
                Self::LatestApplicationConsistent => {
                    serializer.serialize_unit_variant("RecoveryPointType", 1u32, "LatestApplicationConsistent")
                }
                Self::LatestProcessed => serializer.serialize_unit_variant("RecoveryPointType", 2u32, "LatestProcessed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Recovery plan InMageAzureV2 failover input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanInMageAzureV2FailoverInput {
    #[serde(flatten)]
    pub recovery_plan_provider_specific_failover_input: RecoveryPlanProviderSpecificFailoverInput,
    #[doc = "The recovery point type."]
    #[serde(rename = "recoveryPointType")]
    pub recovery_point_type: recovery_plan_in_mage_azure_v2_failover_input::RecoveryPointType,
    #[doc = "A value indicating whether multi VM sync enabled VMs should use multi VM sync points for failover."]
    #[serde(rename = "useMultiVmSyncPoint", default, skip_serializing_if = "Option::is_none")]
    pub use_multi_vm_sync_point: Option<String>,
}
impl RecoveryPlanInMageAzureV2FailoverInput {
    pub fn new(
        recovery_plan_provider_specific_failover_input: RecoveryPlanProviderSpecificFailoverInput,
        recovery_point_type: recovery_plan_in_mage_azure_v2_failover_input::RecoveryPointType,
    ) -> Self {
        Self {
            recovery_plan_provider_specific_failover_input,
            recovery_point_type,
            use_multi_vm_sync_point: None,
        }
    }
}
pub mod recovery_plan_in_mage_azure_v2_failover_input {
    use super::*;
    #[doc = "The recovery point type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryPointType")]
    pub enum RecoveryPointType {
        Latest,
        LatestApplicationConsistent,
        LatestCrashConsistent,
        LatestProcessed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryPointType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryPointType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryPointType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Latest => serializer.serialize_unit_variant("RecoveryPointType", 0u32, "Latest"),
                Self::LatestApplicationConsistent => {
                    serializer.serialize_unit_variant("RecoveryPointType", 1u32, "LatestApplicationConsistent")
                }
                Self::LatestCrashConsistent => serializer.serialize_unit_variant("RecoveryPointType", 2u32, "LatestCrashConsistent"),
                Self::LatestProcessed => serializer.serialize_unit_variant("RecoveryPointType", 3u32, "LatestProcessed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Recovery plan InMage failover input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanInMageFailoverInput {
    #[serde(flatten)]
    pub recovery_plan_provider_specific_failover_input: RecoveryPlanProviderSpecificFailoverInput,
    #[doc = "The recovery point type."]
    #[serde(rename = "recoveryPointType")]
    pub recovery_point_type: recovery_plan_in_mage_failover_input::RecoveryPointType,
}
impl RecoveryPlanInMageFailoverInput {
    pub fn new(
        recovery_plan_provider_specific_failover_input: RecoveryPlanProviderSpecificFailoverInput,
        recovery_point_type: recovery_plan_in_mage_failover_input::RecoveryPointType,
    ) -> Self {
        Self {
            recovery_plan_provider_specific_failover_input,
            recovery_point_type,
        }
    }
}
pub mod recovery_plan_in_mage_failover_input {
    use super::*;
    #[doc = "The recovery point type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryPointType")]
    pub enum RecoveryPointType {
        LatestTime,
        LatestTag,
        Custom,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryPointType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryPointType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryPointType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::LatestTime => serializer.serialize_unit_variant("RecoveryPointType", 0u32, "LatestTime"),
                Self::LatestTag => serializer.serialize_unit_variant("RecoveryPointType", 1u32, "LatestTag"),
                Self::Custom => serializer.serialize_unit_variant("RecoveryPointType", 2u32, "Custom"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Recovery plan InMageRcmFailback failover input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanInMageRcmFailbackFailoverInput {
    #[serde(flatten)]
    pub recovery_plan_provider_specific_failover_input: RecoveryPlanProviderSpecificFailoverInput,
    #[doc = "The recovery point type."]
    #[serde(rename = "recoveryPointType")]
    pub recovery_point_type: recovery_plan_in_mage_rcm_failback_failover_input::RecoveryPointType,
    #[doc = "A value indicating whether multi VM sync enabled VMs should use multi VM sync points for failover."]
    #[serde(rename = "useMultiVmSyncPoint", default, skip_serializing_if = "Option::is_none")]
    pub use_multi_vm_sync_point: Option<String>,
}
impl RecoveryPlanInMageRcmFailbackFailoverInput {
    pub fn new(
        recovery_plan_provider_specific_failover_input: RecoveryPlanProviderSpecificFailoverInput,
        recovery_point_type: recovery_plan_in_mage_rcm_failback_failover_input::RecoveryPointType,
    ) -> Self {
        Self {
            recovery_plan_provider_specific_failover_input,
            recovery_point_type,
            use_multi_vm_sync_point: None,
        }
    }
}
pub mod recovery_plan_in_mage_rcm_failback_failover_input {
    use super::*;
    #[doc = "The recovery point type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryPointType")]
    pub enum RecoveryPointType {
        ApplicationConsistent,
        CrashConsistent,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryPointType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryPointType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryPointType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ApplicationConsistent => serializer.serialize_unit_variant("RecoveryPointType", 0u32, "ApplicationConsistent"),
                Self::CrashConsistent => serializer.serialize_unit_variant("RecoveryPointType", 1u32, "CrashConsistent"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Recovery plan InMageRcm failover input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanInMageRcmFailoverInput {
    #[serde(flatten)]
    pub recovery_plan_provider_specific_failover_input: RecoveryPlanProviderSpecificFailoverInput,
    #[doc = "The recovery point type."]
    #[serde(rename = "recoveryPointType")]
    pub recovery_point_type: recovery_plan_in_mage_rcm_failover_input::RecoveryPointType,
    #[doc = "A value indicating whether multi VM sync enabled VMs should use multi VM sync points for failover."]
    #[serde(rename = "useMultiVmSyncPoint", default, skip_serializing_if = "Option::is_none")]
    pub use_multi_vm_sync_point: Option<String>,
}
impl RecoveryPlanInMageRcmFailoverInput {
    pub fn new(
        recovery_plan_provider_specific_failover_input: RecoveryPlanProviderSpecificFailoverInput,
        recovery_point_type: recovery_plan_in_mage_rcm_failover_input::RecoveryPointType,
    ) -> Self {
        Self {
            recovery_plan_provider_specific_failover_input,
            recovery_point_type,
            use_multi_vm_sync_point: None,
        }
    }
}
pub mod recovery_plan_in_mage_rcm_failover_input {
    use super::*;
    #[doc = "The recovery point type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecoveryPointType")]
    pub enum RecoveryPointType {
        Latest,
        LatestApplicationConsistent,
        LatestCrashConsistent,
        LatestProcessed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecoveryPointType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecoveryPointType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecoveryPointType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Latest => serializer.serialize_unit_variant("RecoveryPointType", 0u32, "Latest"),
                Self::LatestApplicationConsistent => {
                    serializer.serialize_unit_variant("RecoveryPointType", 1u32, "LatestApplicationConsistent")
                }
                Self::LatestCrashConsistent => serializer.serialize_unit_variant("RecoveryPointType", 2u32, "LatestCrashConsistent"),
                Self::LatestProcessed => serializer.serialize_unit_variant("RecoveryPointType", 3u32, "LatestProcessed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Recovery plan manual action details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanManualActionDetails {
    #[serde(flatten)]
    pub recovery_plan_action_details: RecoveryPlanActionDetails,
    #[doc = "The manual action description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl RecoveryPlanManualActionDetails {
    pub fn new(recovery_plan_action_details: RecoveryPlanActionDetails) -> Self {
        Self {
            recovery_plan_action_details,
            description: None,
        }
    }
}
#[doc = "Recovery plan planned failover input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanPlannedFailoverInput {
    #[doc = "Recovery plan planned failover input properties."]
    pub properties: RecoveryPlanPlannedFailoverInputProperties,
}
impl RecoveryPlanPlannedFailoverInput {
    pub fn new(properties: RecoveryPlanPlannedFailoverInputProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Recovery plan planned failover input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanPlannedFailoverInputProperties {
    #[doc = "The failover direction."]
    #[serde(rename = "failoverDirection")]
    pub failover_direction: recovery_plan_planned_failover_input_properties::FailoverDirection,
    #[doc = "The provider specific properties."]
    #[serde(rename = "providerSpecificDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub provider_specific_details: Vec<RecoveryPlanProviderSpecificFailoverInput>,
}
impl RecoveryPlanPlannedFailoverInputProperties {
    pub fn new(failover_direction: recovery_plan_planned_failover_input_properties::FailoverDirection) -> Self {
        Self {
            failover_direction,
            provider_specific_details: Vec::new(),
        }
    }
}
pub mod recovery_plan_planned_failover_input_properties {
    use super::*;
    #[doc = "The failover direction."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FailoverDirection")]
    pub enum FailoverDirection {
        PrimaryToRecovery,
        RecoveryToPrimary,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FailoverDirection {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FailoverDirection {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FailoverDirection {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PrimaryToRecovery => serializer.serialize_unit_variant("FailoverDirection", 0u32, "PrimaryToRecovery"),
                Self::RecoveryToPrimary => serializer.serialize_unit_variant("FailoverDirection", 1u32, "RecoveryToPrimary"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Recovery plan properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoveryPlanProperties {
    #[doc = "The friendly name."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The primary fabric Id."]
    #[serde(rename = "primaryFabricId", default, skip_serializing_if = "Option::is_none")]
    pub primary_fabric_id: Option<String>,
    #[doc = "The primary fabric friendly name."]
    #[serde(rename = "primaryFabricFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub primary_fabric_friendly_name: Option<String>,
    #[doc = "The recovery fabric Id."]
    #[serde(rename = "recoveryFabricId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_fabric_id: Option<String>,
    #[doc = "The recovery fabric friendly name."]
    #[serde(rename = "recoveryFabricFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub recovery_fabric_friendly_name: Option<String>,
    #[doc = "The failover deployment model."]
    #[serde(rename = "failoverDeploymentModel", default, skip_serializing_if = "Option::is_none")]
    pub failover_deployment_model: Option<String>,
    #[doc = "The list of replication providers."]
    #[serde(rename = "replicationProviders", default, skip_serializing_if = "Vec::is_empty")]
    pub replication_providers: Vec<String>,
    #[doc = "The list of allowed operations."]
    #[serde(rename = "allowedOperations", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_operations: Vec<String>,
    #[doc = "The start time of the last planned failover."]
    #[serde(rename = "lastPlannedFailoverTime", with = "azure_core::date::rfc3339::option")]
    pub last_planned_failover_time: Option<time::OffsetDateTime>,
    #[doc = "The start time of the last unplanned failover."]
    #[serde(rename = "lastUnplannedFailoverTime", with = "azure_core::date::rfc3339::option")]
    pub last_unplanned_failover_time: Option<time::OffsetDateTime>,
    #[doc = "The start time of the last test failover."]
    #[serde(rename = "lastTestFailoverTime", with = "azure_core::date::rfc3339::option")]
    pub last_test_failover_time: Option<time::OffsetDateTime>,
    #[doc = "Current scenario details of the protected entity."]
    #[serde(rename = "currentScenario", default, skip_serializing_if = "Option::is_none")]
    pub current_scenario: Option<CurrentScenarioDetails>,
    #[doc = "The recovery plan status."]
    #[serde(rename = "currentScenarioStatus", default, skip_serializing_if = "Option::is_none")]
    pub current_scenario_status: Option<String>,
    #[doc = "The recovery plan status description."]
    #[serde(rename = "currentScenarioStatusDescription", default, skip_serializing_if = "Option::is_none")]
    pub current_scenario_status_description: Option<String>,
    #[doc = "The recovery plan groups."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<RecoveryPlanGroup>,
    #[doc = "The provider id and provider specific details."]
    #[serde(rename = "providerSpecificDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub provider_specific_details: Vec<RecoveryPlanProviderSpecificDetails>,
}
impl RecoveryPlanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Recovery plan protected item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoveryPlanProtectedItem {
    #[doc = "The ARM Id of the recovery plan protected item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The virtual machine Id."]
    #[serde(rename = "virtualMachineId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_id: Option<String>,
}
impl RecoveryPlanProtectedItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Recovery plan provider specific details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanProviderSpecificDetails {
    #[doc = "Gets the Instance type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl RecoveryPlanProviderSpecificDetails {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Recovery plan provider specific failover input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanProviderSpecificFailoverInput {
    #[doc = "The class type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl RecoveryPlanProviderSpecificFailoverInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Recovery plan provider specific input base class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanProviderSpecificInput {
    #[doc = "Gets the Instance type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl RecoveryPlanProviderSpecificInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Recovery plan script action details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanScriptActionDetails {
    #[serde(flatten)]
    pub recovery_plan_action_details: RecoveryPlanActionDetails,
    #[doc = "The script path."]
    pub path: String,
    #[doc = "The script timeout."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[doc = "The fabric location."]
    #[serde(rename = "fabricLocation")]
    pub fabric_location: recovery_plan_script_action_details::FabricLocation,
}
impl RecoveryPlanScriptActionDetails {
    pub fn new(
        recovery_plan_action_details: RecoveryPlanActionDetails,
        path: String,
        fabric_location: recovery_plan_script_action_details::FabricLocation,
    ) -> Self {
        Self {
            recovery_plan_action_details,
            path,
            timeout: None,
            fabric_location,
        }
    }
}
pub mod recovery_plan_script_action_details {
    use super::*;
    #[doc = "The fabric location."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FabricLocation")]
    pub enum FabricLocation {
        Primary,
        Recovery,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FabricLocation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FabricLocation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FabricLocation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Primary => serializer.serialize_unit_variant("FabricLocation", 0u32, "Primary"),
                Self::Recovery => serializer.serialize_unit_variant("FabricLocation", 1u32, "Recovery"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "This class represents the recovery plan shutdown group task details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanShutdownGroupTaskDetails {
    #[serde(flatten)]
    pub recovery_plan_group_task_details: RecoveryPlanGroupTaskDetails,
}
impl RecoveryPlanShutdownGroupTaskDetails {
    pub fn new(recovery_plan_group_task_details: RecoveryPlanGroupTaskDetails) -> Self {
        Self {
            recovery_plan_group_task_details,
        }
    }
}
#[doc = "Recovery plan test failover cleanup input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanTestFailoverCleanupInput {
    #[doc = "Recovery plan test failover cleanup input properties."]
    pub properties: RecoveryPlanTestFailoverCleanupInputProperties,
}
impl RecoveryPlanTestFailoverCleanupInput {
    pub fn new(properties: RecoveryPlanTestFailoverCleanupInputProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Recovery plan test failover cleanup input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoveryPlanTestFailoverCleanupInputProperties {
    #[doc = "The test failover cleanup comments."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}
impl RecoveryPlanTestFailoverCleanupInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Recovery plan test failover input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanTestFailoverInput {
    #[doc = "Recovery plan test failover input properties."]
    pub properties: RecoveryPlanTestFailoverInputProperties,
}
impl RecoveryPlanTestFailoverInput {
    pub fn new(properties: RecoveryPlanTestFailoverInputProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Recovery plan test failover input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanTestFailoverInputProperties {
    #[doc = "The failover direction."]
    #[serde(rename = "failoverDirection")]
    pub failover_direction: recovery_plan_test_failover_input_properties::FailoverDirection,
    #[doc = "The network type to be used for test failover."]
    #[serde(rename = "networkType")]
    pub network_type: String,
    #[doc = "The Id of the network to be used for test failover."]
    #[serde(rename = "networkId", default, skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    #[doc = "The provider specific properties."]
    #[serde(rename = "providerSpecificDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub provider_specific_details: Vec<RecoveryPlanProviderSpecificFailoverInput>,
}
impl RecoveryPlanTestFailoverInputProperties {
    pub fn new(failover_direction: recovery_plan_test_failover_input_properties::FailoverDirection, network_type: String) -> Self {
        Self {
            failover_direction,
            network_type,
            network_id: None,
            provider_specific_details: Vec::new(),
        }
    }
}
pub mod recovery_plan_test_failover_input_properties {
    use super::*;
    #[doc = "The failover direction."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FailoverDirection")]
    pub enum FailoverDirection {
        PrimaryToRecovery,
        RecoveryToPrimary,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FailoverDirection {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FailoverDirection {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FailoverDirection {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PrimaryToRecovery => serializer.serialize_unit_variant("FailoverDirection", 0u32, "PrimaryToRecovery"),
                Self::RecoveryToPrimary => serializer.serialize_unit_variant("FailoverDirection", 1u32, "RecoveryToPrimary"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Recovery plan unplanned failover input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanUnplannedFailoverInput {
    #[doc = "Recovery plan unplanned failover input properties."]
    pub properties: RecoveryPlanUnplannedFailoverInputProperties,
}
impl RecoveryPlanUnplannedFailoverInput {
    pub fn new(properties: RecoveryPlanUnplannedFailoverInputProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Recovery plan unplanned failover input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryPlanUnplannedFailoverInputProperties {
    #[doc = "The failover direction."]
    #[serde(rename = "failoverDirection")]
    pub failover_direction: recovery_plan_unplanned_failover_input_properties::FailoverDirection,
    #[doc = "A value indicating whether source site operations are required."]
    #[serde(rename = "sourceSiteOperations")]
    pub source_site_operations: recovery_plan_unplanned_failover_input_properties::SourceSiteOperations,
    #[doc = "The provider specific properties."]
    #[serde(rename = "providerSpecificDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub provider_specific_details: Vec<RecoveryPlanProviderSpecificFailoverInput>,
}
impl RecoveryPlanUnplannedFailoverInputProperties {
    pub fn new(
        failover_direction: recovery_plan_unplanned_failover_input_properties::FailoverDirection,
        source_site_operations: recovery_plan_unplanned_failover_input_properties::SourceSiteOperations,
    ) -> Self {
        Self {
            failover_direction,
            source_site_operations,
            provider_specific_details: Vec::new(),
        }
    }
}
pub mod recovery_plan_unplanned_failover_input_properties {
    use super::*;
    #[doc = "The failover direction."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FailoverDirection")]
    pub enum FailoverDirection {
        PrimaryToRecovery,
        RecoveryToPrimary,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FailoverDirection {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FailoverDirection {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FailoverDirection {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PrimaryToRecovery => serializer.serialize_unit_variant("FailoverDirection", 0u32, "PrimaryToRecovery"),
                Self::RecoveryToPrimary => serializer.serialize_unit_variant("FailoverDirection", 1u32, "RecoveryToPrimary"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "A value indicating whether source site operations are required."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SourceSiteOperations")]
    pub enum SourceSiteOperations {
        Required,
        NotRequired,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SourceSiteOperations {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SourceSiteOperations {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SourceSiteOperations {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Required => serializer.serialize_unit_variant("SourceSiteOperations", 0u32, "Required"),
                Self::NotRequired => serializer.serialize_unit_variant("SourceSiteOperations", 1u32, "NotRequired"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Recovery point."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoveryPoint {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Recovery point properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecoveryPointProperties>,
}
impl RecoveryPoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of recovery point details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoveryPointCollection {
    #[doc = "The recovery point details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RecoveryPoint>,
    #[doc = "The value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RecoveryPointCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RecoveryPointCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Recovery point properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoveryPointProperties {
    #[doc = "The recovery point time."]
    #[serde(rename = "recoveryPointTime", with = "azure_core::date::rfc3339::option")]
    pub recovery_point_time: Option<time::OffsetDateTime>,
    #[doc = "The recovery point type: ApplicationConsistent, CrashConsistent."]
    #[serde(rename = "recoveryPointType", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_type: Option<String>,
    #[doc = "Replication provider specific recovery point details."]
    #[serde(rename = "providerSpecificDetails", default, skip_serializing_if = "Option::is_none")]
    pub provider_specific_details: Option<ProviderSpecificRecoveryPointDetails>,
}
impl RecoveryPointProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Recovery Proximity placement group custom input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryProximityPlacementGroupCustomDetails {
    #[doc = "The class type."]
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}
impl RecoveryProximityPlacementGroupCustomDetails {
    pub fn new(resource_type: String) -> Self {
        Self { resource_type }
    }
}
#[doc = "Recovery Resource Group custom input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryResourceGroupCustomDetails {
    #[doc = "The class type."]
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}
impl RecoveryResourceGroupCustomDetails {
    pub fn new(resource_type: String) -> Self {
        Self { resource_type }
    }
}
#[doc = "Provider details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoveryServicesProvider {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Recovery services provider properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RecoveryServicesProviderProperties>,
}
impl RecoveryServicesProvider {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of providers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoveryServicesProviderCollection {
    #[doc = "The Servers details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RecoveryServicesProvider>,
    #[doc = "The value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RecoveryServicesProviderCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RecoveryServicesProviderCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Recovery services provider properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecoveryServicesProviderProperties {
    #[doc = "Type of the site."]
    #[serde(rename = "fabricType", default, skip_serializing_if = "Option::is_none")]
    pub fabric_type: Option<String>,
    #[doc = "Friendly name of the DRA."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The provider version."]
    #[serde(rename = "providerVersion", default, skip_serializing_if = "Option::is_none")]
    pub provider_version: Option<String>,
    #[doc = "The fabric provider."]
    #[serde(rename = "serverVersion", default, skip_serializing_if = "Option::is_none")]
    pub server_version: Option<String>,
    #[doc = "DRA version status."]
    #[serde(rename = "providerVersionState", default, skip_serializing_if = "Option::is_none")]
    pub provider_version_state: Option<String>,
    #[doc = "Expiry date of the version."]
    #[serde(rename = "providerVersionExpiryDate", with = "azure_core::date::rfc3339::option")]
    pub provider_version_expiry_date: Option<time::OffsetDateTime>,
    #[doc = "The fabric friendly name."]
    #[serde(rename = "fabricFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub fabric_friendly_name: Option<String>,
    #[doc = "Time when last heartbeat was sent by the DRA."]
    #[serde(rename = "lastHeartBeat", with = "azure_core::date::rfc3339::option")]
    pub last_heart_beat: Option<time::OffsetDateTime>,
    #[doc = "A value indicating whether DRA is responsive."]
    #[serde(rename = "connectionStatus", default, skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<String>,
    #[doc = "Number of protected VMs currently managed by the DRA."]
    #[serde(rename = "protectedItemCount", default, skip_serializing_if = "Option::is_none")]
    pub protected_item_count: Option<i32>,
    #[doc = "The scenarios allowed on this provider."]
    #[serde(rename = "allowedScenarios", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_scenarios: Vec<String>,
    #[doc = "The recovery services provider health error details."]
    #[serde(rename = "healthErrorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub health_error_details: Vec<HealthError>,
    #[doc = "The DRA Id."]
    #[serde(rename = "draIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub dra_identifier: Option<String>,
    #[doc = "The machine Id."]
    #[serde(rename = "machineId", default, skip_serializing_if = "Option::is_none")]
    pub machine_id: Option<String>,
    #[doc = "The machine name."]
    #[serde(rename = "machineName", default, skip_serializing_if = "Option::is_none")]
    pub machine_name: Option<String>,
    #[doc = "The Bios Id."]
    #[serde(rename = "biosId", default, skip_serializing_if = "Option::is_none")]
    pub bios_id: Option<String>,
    #[doc = "Identity provider details."]
    #[serde(rename = "authenticationIdentityDetails", default, skip_serializing_if = "Option::is_none")]
    pub authentication_identity_details: Option<IdentityProviderDetails>,
    #[doc = "Identity provider details."]
    #[serde(rename = "resourceAccessIdentityDetails", default, skip_serializing_if = "Option::is_none")]
    pub resource_access_identity_details: Option<IdentityProviderDetails>,
    #[doc = "Identity provider details."]
    #[serde(
        rename = "dataPlaneAuthenticationIdentityDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_plane_authentication_identity_details: Option<IdentityProviderDetails>,
    #[doc = "Version related details."]
    #[serde(rename = "providerVersionDetails", default, skip_serializing_if = "Option::is_none")]
    pub provider_version_details: Option<VersionDetails>,
}
impl RecoveryServicesProviderProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Recovery Virtual network custom input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecoveryVirtualNetworkCustomDetails {
    #[doc = "The class type."]
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}
impl RecoveryVirtualNetworkCustomDetails {
    pub fn new(resource_type: String) -> Self {
        Self { resource_type }
    }
}
#[doc = "Input for remove disk(s) operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemoveDisksInput {
    #[doc = "Remove Disk input properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RemoveDisksInputProperties>,
}
impl RemoveDisksInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Remove Disk input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemoveDisksInputProperties {
    #[doc = "Remove Disk provider specific input."]
    #[serde(rename = "providerSpecificDetails", default, skip_serializing_if = "Option::is_none")]
    pub provider_specific_details: Option<RemoveDisksProviderSpecificInput>,
}
impl RemoveDisksInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Remove Disk provider specific input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RemoveDisksProviderSpecificInput {
    #[doc = "The class type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl RemoveDisksProviderSpecificInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Container unpairing input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemoveProtectionContainerMappingInput {
    #[doc = "Unpairing input properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RemoveProtectionContainerMappingInputProperties>,
}
impl RemoveProtectionContainerMappingInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Unpairing input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RemoveProtectionContainerMappingInputProperties {
    #[doc = "Provider specific input for unpairing operations."]
    #[serde(rename = "providerSpecificInput", default, skip_serializing_if = "Option::is_none")]
    pub provider_specific_input: Option<ReplicationProviderContainerUnmappingInput>,
}
impl RemoveProtectionContainerMappingInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Certificate renewal input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RenewCertificateInput {
    #[doc = "Renew Certificate input properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RenewCertificateInputProperties>,
}
impl RenewCertificateInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Renew Certificate input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RenewCertificateInputProperties {
    #[doc = "Renew certificate type."]
    #[serde(rename = "renewCertificateType", default, skip_serializing_if = "Option::is_none")]
    pub renew_certificate_type: Option<String>,
}
impl RenewCertificateInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Replication agent details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationAgentDetails {
    #[doc = "The replication agent Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The replication agent name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The replication agent Bios Id."]
    #[serde(rename = "biosId", default, skip_serializing_if = "Option::is_none")]
    pub bios_id: Option<String>,
    #[doc = "The fabric object Id."]
    #[serde(rename = "fabricObjectId", default, skip_serializing_if = "Option::is_none")]
    pub fabric_object_id: Option<String>,
    #[doc = "The replication agent Fqdn."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "The version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The last heartbeat received from the replication agent."]
    #[serde(rename = "lastHeartbeatUtc", with = "azure_core::date::rfc3339::option")]
    pub last_heartbeat_utc: Option<time::OffsetDateTime>,
    #[doc = "The health of the replication agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<replication_agent_details::Health>,
    #[doc = "The health errors."]
    #[serde(rename = "healthErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub health_errors: Vec<HealthError>,
}
impl ReplicationAgentDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod replication_agent_details {
    use super::*;
    #[doc = "The health of the replication agent."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Health")]
    pub enum Health {
        None,
        Normal,
        Warning,
        Critical,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Health {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Health {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Health {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Health", 0u32, "None"),
                Self::Normal => serializer.serialize_unit_variant("Health", 1u32, "Normal"),
                Self::Warning => serializer.serialize_unit_variant("Health", 2u32, "Warning"),
                Self::Critical => serializer.serialize_unit_variant("Health", 3u32, "Critical"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Replication appliance definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationAppliance {
    #[doc = "Replication appliance properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReplicationApplianceProperties>,
}
impl ReplicationAppliance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Replication appliance properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationApplianceProperties {
    #[doc = "Appliance specific details."]
    #[serde(rename = "providerSpecificDetails", default, skip_serializing_if = "Option::is_none")]
    pub provider_specific_details: Option<ApplianceSpecificDetails>,
}
impl ReplicationApplianceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Replication eligibility results response model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationEligibilityResults {
    #[doc = "Gets the name of this object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the object type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets Unique ARM identifier for this object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Properties model for replication eligibility results API."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReplicationEligibilityResultsProperties>,
}
impl ReplicationEligibilityResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Replication eligibility results collection response model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationEligibilityResultsCollection {
    #[doc = "The replication eligibility results details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReplicationEligibilityResults>,
}
impl ReplicationEligibilityResultsCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error model that can be exposed to the user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationEligibilityResultsErrorInfo {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The possible causes."]
    #[serde(rename = "possibleCauses", default, skip_serializing_if = "Option::is_none")]
    pub possible_causes: Option<String>,
    #[doc = "The recommended action."]
    #[serde(rename = "recommendedAction", default, skip_serializing_if = "Option::is_none")]
    pub recommended_action: Option<String>,
    #[doc = "The error status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl ReplicationEligibilityResultsErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties model for replication eligibility results API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationEligibilityResultsProperties {
    #[doc = "The client request Id."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "The error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ReplicationEligibilityResultsErrorInfo>,
}
impl ReplicationEligibilityResultsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Replication group details. This will be used in case of San."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicationGroupDetails {
    #[serde(flatten)]
    pub configuration_settings: ConfigurationSettings,
}
impl ReplicationGroupDetails {
    pub fn new(configuration_settings: ConfigurationSettings) -> Self {
        Self { configuration_settings }
    }
}
#[doc = "Replication protected item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationProtectedItem {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Replication protected item custom data details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReplicationProtectedItemProperties>,
}
impl ReplicationProtectedItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Replication protected item collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationProtectedItemCollection {
    #[doc = "The Replication protected item details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReplicationProtectedItem>,
    #[doc = "The value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ReplicationProtectedItemCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ReplicationProtectedItemCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Replication protected item custom data details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationProtectedItemProperties {
    #[doc = "The name."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The type of protected item type."]
    #[serde(rename = "protectedItemType", default, skip_serializing_if = "Option::is_none")]
    pub protected_item_type: Option<String>,
    #[doc = "The protected item ARM Id."]
    #[serde(rename = "protectableItemId", default, skip_serializing_if = "Option::is_none")]
    pub protectable_item_id: Option<String>,
    #[doc = "The recovery provider ARM Id."]
    #[serde(rename = "recoveryServicesProviderId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_services_provider_id: Option<String>,
    #[doc = "The friendly name of the primary fabric."]
    #[serde(rename = "primaryFabricFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub primary_fabric_friendly_name: Option<String>,
    #[doc = "The fabric provider of the primary fabric."]
    #[serde(rename = "primaryFabricProvider", default, skip_serializing_if = "Option::is_none")]
    pub primary_fabric_provider: Option<String>,
    #[doc = "The friendly name of recovery fabric."]
    #[serde(rename = "recoveryFabricFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub recovery_fabric_friendly_name: Option<String>,
    #[doc = "The Arm Id of recovery fabric."]
    #[serde(rename = "recoveryFabricId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_fabric_id: Option<String>,
    #[doc = "The name of primary protection container friendly name."]
    #[serde(
        rename = "primaryProtectionContainerFriendlyName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_protection_container_friendly_name: Option<String>,
    #[doc = "The name of recovery container friendly name."]
    #[serde(
        rename = "recoveryProtectionContainerFriendlyName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recovery_protection_container_friendly_name: Option<String>,
    #[doc = "The protection status."]
    #[serde(rename = "protectionState", default, skip_serializing_if = "Option::is_none")]
    pub protection_state: Option<String>,
    #[doc = "The protection state description."]
    #[serde(rename = "protectionStateDescription", default, skip_serializing_if = "Option::is_none")]
    pub protection_state_description: Option<String>,
    #[doc = "The Current active location of the PE."]
    #[serde(rename = "activeLocation", default, skip_serializing_if = "Option::is_none")]
    pub active_location: Option<String>,
    #[doc = "The Test failover state."]
    #[serde(rename = "testFailoverState", default, skip_serializing_if = "Option::is_none")]
    pub test_failover_state: Option<String>,
    #[doc = "The Test failover state description."]
    #[serde(rename = "testFailoverStateDescription", default, skip_serializing_if = "Option::is_none")]
    pub test_failover_state_description: Option<String>,
    #[doc = "The switch provider state."]
    #[serde(rename = "switchProviderState", default, skip_serializing_if = "Option::is_none")]
    pub switch_provider_state: Option<String>,
    #[doc = "The switch provider state description."]
    #[serde(rename = "switchProviderStateDescription", default, skip_serializing_if = "Option::is_none")]
    pub switch_provider_state_description: Option<String>,
    #[doc = "The allowed operations on the Replication protected item."]
    #[serde(rename = "allowedOperations", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_operations: Vec<String>,
    #[doc = "The consolidated protection health for the VM taking any issues with SRS as well as all the replication units associated with the VM's replication group into account. This is a string representation of the ProtectionHealth enumeration."]
    #[serde(rename = "replicationHealth", default, skip_serializing_if = "Option::is_none")]
    pub replication_health: Option<String>,
    #[doc = "The consolidated failover health for the VM."]
    #[serde(rename = "failoverHealth", default, skip_serializing_if = "Option::is_none")]
    pub failover_health: Option<String>,
    #[doc = "List of health errors."]
    #[serde(rename = "healthErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub health_errors: Vec<HealthError>,
    #[doc = "The ID of Policy governing this PE."]
    #[serde(rename = "policyId", default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[doc = "The name of Policy governing this PE."]
    #[serde(rename = "policyFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub policy_friendly_name: Option<String>,
    #[doc = "The Last successful failover time."]
    #[serde(rename = "lastSuccessfulFailoverTime", with = "azure_core::date::rfc3339::option")]
    pub last_successful_failover_time: Option<time::OffsetDateTime>,
    #[doc = "The Last successful test failover time."]
    #[serde(rename = "lastSuccessfulTestFailoverTime", with = "azure_core::date::rfc3339::option")]
    pub last_successful_test_failover_time: Option<time::OffsetDateTime>,
    #[doc = "Current scenario details of the protected entity."]
    #[serde(rename = "currentScenario", default, skip_serializing_if = "Option::is_none")]
    pub current_scenario: Option<CurrentScenarioDetails>,
    #[doc = "The recovery point ARM Id to which the Vm was failed over."]
    #[serde(rename = "failoverRecoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub failover_recovery_point_id: Option<String>,
    #[doc = "Replication provider specific settings."]
    #[serde(rename = "providerSpecificDetails", default, skip_serializing_if = "Option::is_none")]
    pub provider_specific_details: Option<ReplicationProviderSpecificSettings>,
    #[doc = "The recovery container Id."]
    #[serde(rename = "recoveryContainerId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_container_id: Option<String>,
    #[doc = "The correlation Id for events associated with this protected item."]
    #[serde(rename = "eventCorrelationId", default, skip_serializing_if = "Option::is_none")]
    pub event_correlation_id: Option<String>,
}
impl ReplicationProtectedItemProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Replication protection intent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationProtectionIntent {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Replication protection intent custom data details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReplicationProtectionIntentProperties>,
}
impl ReplicationProtectionIntent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Replication protection intent objects collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationProtectionIntentCollection {
    #[doc = "The Replication protection intent details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReplicationProtectionIntent>,
    #[doc = "The value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ReplicationProtectionIntentCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ReplicationProtectionIntentCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Replication protection intent custom data details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationProtectionIntentProperties {
    #[doc = "The name."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The job Id."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "The job state."]
    #[serde(rename = "jobState", default, skip_serializing_if = "Option::is_none")]
    pub job_state: Option<String>,
    #[doc = "A value indicating whether the intent object is active."]
    #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[doc = "The creation time in UTC."]
    #[serde(rename = "creationTimeUTC", default, skip_serializing_if = "Option::is_none")]
    pub creation_time_utc: Option<String>,
    #[doc = "Replication provider specific settings."]
    #[serde(rename = "providerSpecificDetails", default, skip_serializing_if = "Option::is_none")]
    pub provider_specific_details: Option<ReplicationProtectionIntentProviderSpecificSettings>,
}
impl ReplicationProtectionIntentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Replication provider specific settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicationProtectionIntentProviderSpecificSettings {
    #[doc = "Gets the Instance type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl ReplicationProtectionIntentProviderSpecificSettings {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Provider specific input for unpairing operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReplicationProviderContainerUnmappingInput {
    #[doc = "The class type."]
    #[serde(rename = "instanceType", default, skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
}
impl ReplicationProviderContainerUnmappingInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provider specific input for container creation operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicationProviderSpecificContainerCreationInput {
    #[doc = "The class type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl ReplicationProviderSpecificContainerCreationInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Provider specific input for pairing operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicationProviderSpecificContainerMappingInput {
    #[doc = "The class type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl ReplicationProviderSpecificContainerMappingInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Replication provider specific settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicationProviderSpecificSettings {
    #[doc = "Gets the Instance type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl ReplicationProviderSpecificSettings {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Provider specific input for update pairing operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReplicationProviderSpecificUpdateContainerMappingInput {
    #[doc = "The class type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl ReplicationProviderSpecificUpdateContainerMappingInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Reprotect agent details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReprotectAgentDetails {
    #[doc = "The reprotect agent Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The reprotect agent name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The reprotect agent Bios Id."]
    #[serde(rename = "biosId", default, skip_serializing_if = "Option::is_none")]
    pub bios_id: Option<String>,
    #[doc = "The fabric object Id."]
    #[serde(rename = "fabricObjectId", default, skip_serializing_if = "Option::is_none")]
    pub fabric_object_id: Option<String>,
    #[doc = "The reprotect agent Fqdn."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "The version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The last heartbeat received from the reprotect agent."]
    #[serde(rename = "lastHeartbeatUtc", with = "azure_core::date::rfc3339::option")]
    pub last_heartbeat_utc: Option<time::OffsetDateTime>,
    #[doc = "The health of the reprotect agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<reprotect_agent_details::Health>,
    #[doc = "The health errors."]
    #[serde(rename = "healthErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub health_errors: Vec<HealthError>,
    #[doc = "The protected item count."]
    #[serde(rename = "protectedItemCount", default, skip_serializing_if = "Option::is_none")]
    pub protected_item_count: Option<i32>,
    #[doc = "The list of accessible datastores fetched from discovery."]
    #[serde(rename = "accessibleDatastores", default, skip_serializing_if = "Vec::is_empty")]
    pub accessible_datastores: Vec<String>,
    #[doc = "The Vcenter Id."]
    #[serde(rename = "vcenterId", default, skip_serializing_if = "Option::is_none")]
    pub vcenter_id: Option<String>,
    #[doc = "The last time when SDS information discovered in SRS."]
    #[serde(rename = "lastDiscoveryInUtc", with = "azure_core::date::rfc3339::option")]
    pub last_discovery_in_utc: Option<time::OffsetDateTime>,
}
impl ReprotectAgentDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod reprotect_agent_details {
    use super::*;
    #[doc = "The health of the reprotect agent."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Health")]
    pub enum Health {
        None,
        Normal,
        Warning,
        Critical,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Health {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Health {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Health {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Health", 0u32, "None"),
                Self::Normal => serializer.serialize_unit_variant("Health", 1u32, "Normal"),
                Self::Warning => serializer.serialize_unit_variant("Health", 2u32, "Warning"),
                Self::Critical => serializer.serialize_unit_variant("Health", 3u32, "Critical"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Resolve health errors input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResolveHealthError {
    #[doc = "Health error id."]
    #[serde(rename = "healthErrorId", default, skip_serializing_if = "Option::is_none")]
    pub health_error_id: Option<String>,
}
impl ResolveHealthError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resolve health input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResolveHealthInput {
    #[doc = "Resolve health input properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ResolveHealthInputProperties>,
}
impl ResolveHealthInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resolve health input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResolveHealthInputProperties {
    #[doc = "Health errors."]
    #[serde(rename = "healthErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub health_errors: Vec<ResolveHealthError>,
}
impl ResolveHealthInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource Type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource Location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Base class to define the health summary of the resources contained under an Arm resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceHealthSummary {
    #[doc = "The count of total resources under the container."]
    #[serde(rename = "resourceCount", default, skip_serializing_if = "Option::is_none")]
    pub resource_count: Option<i32>,
    #[doc = "The list of summary of health errors across the resources under the container."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub issues: Vec<HealthErrorSummary>,
    #[doc = "The categorized resource counts."]
    #[serde(rename = "categorizedResourceCounts", default, skip_serializing_if = "Option::is_none")]
    pub categorized_resource_counts: Option<serde_json::Value>,
}
impl ResourceHealthSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resume job params."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResumeJobParams {
    #[doc = "Resume job properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ResumeJobParamsProperties>,
}
impl ResumeJobParams {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resume job properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResumeJobParamsProperties {
    #[doc = "Resume job comments."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}
impl ResumeJobParamsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resync input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResyncInput {
    #[doc = "Resync input properties."]
    pub properties: ResyncInputProperties,
}
impl ResyncInput {
    pub fn new(properties: ResyncInputProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Resync input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResyncInputProperties {
    #[doc = "Resync provider specific input."]
    #[serde(rename = "providerSpecificDetails")]
    pub provider_specific_details: ResyncProviderSpecificInput,
}
impl ResyncInputProperties {
    pub fn new(provider_specific_details: ResyncProviderSpecificInput) -> Self {
        Self { provider_specific_details }
    }
}
#[doc = "Resync provider specific input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResyncProviderSpecificInput {
    #[doc = "The class type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl ResyncProviderSpecificInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "The retention details of the MT."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RetentionVolume {
    #[doc = "The volume name."]
    #[serde(rename = "volumeName", default, skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<String>,
    #[doc = "The volume capacity."]
    #[serde(rename = "capacityInBytes", default, skip_serializing_if = "Option::is_none")]
    pub capacity_in_bytes: Option<i64>,
    #[doc = "The free space available in this volume."]
    #[serde(rename = "freeSpaceInBytes", default, skip_serializing_if = "Option::is_none")]
    pub free_space_in_bytes: Option<i64>,
    #[doc = "The threshold percentage."]
    #[serde(rename = "thresholdPercentage", default, skip_serializing_if = "Option::is_none")]
    pub threshold_percentage: Option<i32>,
}
impl RetentionVolume {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reverse replication input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReverseReplicationInput {
    #[doc = "Reverse replication input properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReverseReplicationInputProperties>,
}
impl ReverseReplicationInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reverse replication input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReverseReplicationInputProperties {
    #[doc = "Failover direction."]
    #[serde(rename = "failoverDirection", default, skip_serializing_if = "Option::is_none")]
    pub failover_direction: Option<String>,
    #[doc = "Provider specific reverse replication input."]
    #[serde(rename = "providerSpecificDetails", default, skip_serializing_if = "Option::is_none")]
    pub provider_specific_details: Option<ReverseReplicationProviderSpecificInput>,
}
impl ReverseReplicationInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provider specific reverse replication input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReverseReplicationProviderSpecificInput {
    #[doc = "The class type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl ReverseReplicationProviderSpecificInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Azure role assignment details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignment {
    #[doc = "The ARM Id of the role assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the role assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Role assignment scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "Principal Id."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Role definition id."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
}
impl RoleAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "CS Accounts Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunAsAccount {
    #[doc = "The CS RunAs account Id."]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "The CS RunAs account name."]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
}
impl RunAsAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This class represents the script action task details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptActionTaskDetails {
    #[serde(flatten)]
    pub task_type_details: TaskTypeDetails,
    #[doc = "The name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The path."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[doc = "A value indicating whether it is a primary side script or not."]
    #[serde(rename = "isPrimarySideScript", default, skip_serializing_if = "Option::is_none")]
    pub is_primary_side_script: Option<bool>,
}
impl ScriptActionTaskDetails {
    pub fn new(task_type_details: TaskTypeDetails) -> Self {
        Self {
            task_type_details,
            name: None,
            path: None,
            output: None,
            is_primary_side_script: None,
        }
    }
}
#[doc = "ASR error model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceError {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Possible causes of error."]
    #[serde(rename = "possibleCauses", default, skip_serializing_if = "Option::is_none")]
    pub possible_causes: Option<String>,
    #[doc = "Recommended action to resolve error."]
    #[serde(rename = "recommendedAction", default, skip_serializing_if = "Option::is_none")]
    pub recommended_action: Option<String>,
    #[doc = "Activity Id."]
    #[serde(rename = "activityId", default, skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
}
impl ServiceError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage account custom input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccountCustomDetails {
    #[doc = "The class type."]
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}
impl StorageAccountCustomDetails {
    pub fn new(resource_type: String) -> Self {
        Self { resource_type }
    }
}
#[doc = "Storage object definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageClassification {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Storage object properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageClassificationProperties>,
}
impl StorageClassification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of storage details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageClassificationCollection {
    #[doc = "The storage details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StorageClassification>,
    #[doc = "The value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StorageClassificationCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl StorageClassificationCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage mapping object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageClassificationMapping {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Storage mapping properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageClassificationMappingProperties>,
}
impl StorageClassificationMapping {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of storage mapping details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageClassificationMappingCollection {
    #[doc = "The storage details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StorageClassificationMapping>,
    #[doc = "The value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StorageClassificationMappingCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl StorageClassificationMappingCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage mapping input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageClassificationMappingInput {
    #[doc = "Storage mapping input properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageMappingInputProperties>,
}
impl StorageClassificationMappingInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage mapping properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageClassificationMappingProperties {
    #[doc = "Target storage object Id."]
    #[serde(rename = "targetStorageClassificationId", default, skip_serializing_if = "Option::is_none")]
    pub target_storage_classification_id: Option<String>,
}
impl StorageClassificationMappingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage object properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageClassificationProperties {
    #[doc = "Friendly name of the Storage classification."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
impl StorageClassificationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage mapping input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageMappingInputProperties {
    #[doc = "The ID of the storage object."]
    #[serde(rename = "targetStorageClassificationId", default, skip_serializing_if = "Option::is_none")]
    pub target_storage_classification_id: Option<String>,
}
impl StorageMappingInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subnets of the network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Subnet {
    #[doc = "The subnet name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The subnet friendly name."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The list of addresses for the subnet."]
    #[serde(rename = "addressList", default, skip_serializing_if = "Vec::is_empty")]
    pub address_list: Vec<String>,
}
impl Subnet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Supported operating system details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedOsDetails {
    #[doc = "The name."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "The type."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "The list of version for operating system."]
    #[serde(rename = "osVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub os_versions: Vec<OsVersionWrapper>,
}
impl SupportedOsDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Supported operating systems properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedOsProperties {
    #[doc = "The supported operating systems property list."]
    #[serde(rename = "supportedOsList", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_os_list: Vec<SupportedOsProperty>,
}
impl SupportedOsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Supported operating systems property."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedOsProperty {
    #[doc = "The replication provider type."]
    #[serde(rename = "instanceType", default, skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[doc = "The list of supported operating systems."]
    #[serde(rename = "supportedOs", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_os: Vec<SupportedOsDetails>,
}
impl SupportedOsProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Supported operating systems."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedOperatingSystems {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Supported operating systems properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SupportedOsProperties>,
}
impl SupportedOperatingSystems {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Switch protection input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SwitchProtectionInput {
    #[doc = "Switch protection input properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SwitchProtectionInputProperties>,
}
impl SwitchProtectionInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Switch protection input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SwitchProtectionInputProperties {
    #[doc = "The unique replication protected item name."]
    #[serde(rename = "replicationProtectedItemName", default, skip_serializing_if = "Option::is_none")]
    pub replication_protected_item_name: Option<String>,
    #[doc = "Provider specific switch protection input."]
    #[serde(rename = "providerSpecificDetails", default, skip_serializing_if = "Option::is_none")]
    pub provider_specific_details: Option<SwitchProtectionProviderSpecificInput>,
}
impl SwitchProtectionInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This class represents details for switch protection job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwitchProtectionJobDetails {
    #[serde(flatten)]
    pub job_details: JobDetails,
    #[doc = "ARM Id of the new replication protected item."]
    #[serde(rename = "newReplicationProtectedItemId", default, skip_serializing_if = "Option::is_none")]
    pub new_replication_protected_item_id: Option<String>,
}
impl SwitchProtectionJobDetails {
    pub fn new(job_details: JobDetails) -> Self {
        Self {
            job_details,
            new_replication_protected_item_id: None,
        }
    }
}
#[doc = "Provider specific switch protection input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwitchProtectionProviderSpecificInput {
    #[doc = "Gets the Instance type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl SwitchProtectionProviderSpecificInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Input definition for switch provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SwitchProviderInput {
    #[doc = "Input definition for switch provider input properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SwitchProviderInputProperties>,
}
impl SwitchProviderInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Input definition for switch provider input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SwitchProviderInputProperties {
    #[doc = "Target provider type."]
    #[serde(rename = "targetInstanceType", default, skip_serializing_if = "Option::is_none")]
    pub target_instance_type: Option<String>,
    #[doc = "Provider specific switch provider input."]
    #[serde(rename = "providerSpecificDetails", default, skip_serializing_if = "Option::is_none")]
    pub provider_specific_details: Option<SwitchProviderProviderSpecificInput>,
}
impl SwitchProviderInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provider specific switch provider input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwitchProviderProviderSpecificInput {
    #[doc = "The class type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl SwitchProviderProviderSpecificInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Represents applicable recovery vm sizes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TargetComputeSize {
    #[doc = "The Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The Type of the object."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Represents applicable recovery vm sizes properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TargetComputeSizeProperties>,
}
impl TargetComputeSize {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Target compute size collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TargetComputeSizeCollection {
    #[doc = "The list of target compute sizes."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TargetComputeSize>,
    #[doc = "The value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TargetComputeSizeCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TargetComputeSizeCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents applicable recovery vm sizes properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TargetComputeSizeProperties {
    #[doc = "Target compute size name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Target compute size display name."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The maximum cpu cores count supported by target compute size."]
    #[serde(rename = "cpuCoresCount", default, skip_serializing_if = "Option::is_none")]
    pub cpu_cores_count: Option<i32>,
    #[doc = "The Available vCPUs supported by target compute size."]
    #[serde(rename = "vCPUsAvailable", default, skip_serializing_if = "Option::is_none")]
    pub v_cp_us_available: Option<i32>,
    #[doc = "The maximum memory in GB supported by target compute size."]
    #[serde(rename = "memoryInGB", default, skip_serializing_if = "Option::is_none")]
    pub memory_in_gb: Option<f64>,
    #[doc = "The maximum data disks count supported by target compute size."]
    #[serde(rename = "maxDataDiskCount", default, skip_serializing_if = "Option::is_none")]
    pub max_data_disk_count: Option<i32>,
    #[doc = "The maximum Nics count supported by target compute size."]
    #[serde(rename = "maxNicsCount", default, skip_serializing_if = "Option::is_none")]
    pub max_nics_count: Option<i32>,
    #[doc = "The reasons why the target compute size is not applicable for the protected item."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ComputeSizeErrorDetails>,
    #[doc = "The value indicating whether the target compute size supports high Iops."]
    #[serde(rename = "highIopsSupported", default, skip_serializing_if = "Option::is_none")]
    pub high_iops_supported: Option<String>,
    #[doc = "The supported HyperV Generations."]
    #[serde(rename = "hyperVGenerations", default, skip_serializing_if = "Vec::is_empty")]
    pub hyper_v_generations: Vec<String>,
}
impl TargetComputeSizeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Task details based on specific task type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskTypeDetails {
    #[doc = "The type of task details."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl TaskTypeDetails {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Input definition for test failover cleanup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestFailoverCleanupInput {
    #[doc = "Input definition for test failover cleanup input properties."]
    pub properties: TestFailoverCleanupInputProperties,
}
impl TestFailoverCleanupInput {
    pub fn new(properties: TestFailoverCleanupInputProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Input definition for test failover cleanup input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestFailoverCleanupInputProperties {
    #[doc = "Test failover cleanup comments."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}
impl TestFailoverCleanupInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Input definition for test failover."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestFailoverInput {
    #[doc = "Input definition for test failover input properties."]
    pub properties: TestFailoverInputProperties,
}
impl TestFailoverInput {
    pub fn new(properties: TestFailoverInputProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Input definition for test failover input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestFailoverInputProperties {
    #[doc = "Test failover direction."]
    #[serde(rename = "failoverDirection", default, skip_serializing_if = "Option::is_none")]
    pub failover_direction: Option<String>,
    #[doc = "Network type to be used for test failover."]
    #[serde(rename = "networkType", default, skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[doc = "The id of the network to be used for test failover."]
    #[serde(rename = "networkId", default, skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    #[doc = "Provider specific test failover input."]
    #[serde(rename = "providerSpecificDetails", default, skip_serializing_if = "Option::is_none")]
    pub provider_specific_details: Option<TestFailoverProviderSpecificInput>,
}
impl TestFailoverInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This class represents the details for a test failover job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestFailoverJobDetails {
    #[serde(flatten)]
    pub job_details: JobDetails,
    #[doc = "The test failover status."]
    #[serde(rename = "testFailoverStatus", default, skip_serializing_if = "Option::is_none")]
    pub test_failover_status: Option<String>,
    #[doc = "The test failover comments."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[doc = "The test network name."]
    #[serde(rename = "networkName", default, skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    #[doc = "The test network friendly name."]
    #[serde(rename = "networkFriendlyName", default, skip_serializing_if = "Option::is_none")]
    pub network_friendly_name: Option<String>,
    #[doc = "The test network type (see TestFailoverInput enum for possible values)."]
    #[serde(rename = "networkType", default, skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[doc = "The test VM details."]
    #[serde(rename = "protectedItemDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub protected_item_details: Vec<FailoverReplicationProtectedItemDetails>,
}
impl TestFailoverJobDetails {
    pub fn new(job_details: JobDetails) -> Self {
        Self {
            job_details,
            test_failover_status: None,
            comments: None,
            network_name: None,
            network_friendly_name: None,
            network_type: None,
            protected_item_details: Vec::new(),
        }
    }
}
#[doc = "Provider specific test failover input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestFailoverProviderSpecificInput {
    #[doc = "The class type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl TestFailoverProviderSpecificInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Input for test migrate cleanup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestMigrateCleanupInput {
    #[doc = "Test migrate cleanup input properties."]
    pub properties: TestMigrateCleanupInputProperties,
}
impl TestMigrateCleanupInput {
    pub fn new(properties: TestMigrateCleanupInputProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Test migrate cleanup input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestMigrateCleanupInputProperties {
    #[doc = "Test migrate cleanup comments."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}
impl TestMigrateCleanupInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Input for test migrate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestMigrateInput {
    #[doc = "Test migrate input properties."]
    pub properties: TestMigrateInputProperties,
}
impl TestMigrateInput {
    pub fn new(properties: TestMigrateInputProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Test migrate input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestMigrateInputProperties {
    #[doc = "Test migrate provider specific input."]
    #[serde(rename = "providerSpecificDetails")]
    pub provider_specific_details: TestMigrateProviderSpecificInput,
}
impl TestMigrateInputProperties {
    pub fn new(provider_specific_details: TestMigrateProviderSpecificInput) -> Self {
        Self { provider_specific_details }
    }
}
#[doc = "Test migrate provider specific input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestMigrateProviderSpecificInput {
    #[doc = "The class type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl TestMigrateProviderSpecificInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Input definition for unplanned failover."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnplannedFailoverInput {
    #[doc = "Input definition for unplanned failover input properties."]
    pub properties: UnplannedFailoverInputProperties,
}
impl UnplannedFailoverInput {
    pub fn new(properties: UnplannedFailoverInputProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Input definition for unplanned failover input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UnplannedFailoverInputProperties {
    #[doc = "Failover direction."]
    #[serde(rename = "failoverDirection", default, skip_serializing_if = "Option::is_none")]
    pub failover_direction: Option<String>,
    #[doc = "Source site operations status."]
    #[serde(rename = "sourceSiteOperations", default, skip_serializing_if = "Option::is_none")]
    pub source_site_operations: Option<String>,
    #[doc = "Provider specific unplanned failover input."]
    #[serde(rename = "providerSpecificDetails", default, skip_serializing_if = "Option::is_none")]
    pub provider_specific_details: Option<UnplannedFailoverProviderSpecificInput>,
}
impl UnplannedFailoverInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provider specific unplanned failover input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnplannedFailoverProviderSpecificInput {
    #[doc = "The class type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl UnplannedFailoverProviderSpecificInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Update appliance for replication protected item input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateApplianceForReplicationProtectedItemInput {
    #[doc = "Update appliance for protected item input properties."]
    pub properties: UpdateApplianceForReplicationProtectedItemInputProperties,
}
impl UpdateApplianceForReplicationProtectedItemInput {
    pub fn new(properties: UpdateApplianceForReplicationProtectedItemInputProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Update appliance for protected item input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateApplianceForReplicationProtectedItemInputProperties {
    #[doc = "The target appliance Id."]
    #[serde(rename = "targetApplianceId")]
    pub target_appliance_id: String,
    #[doc = "Update replication protected item provider specific input."]
    #[serde(rename = "providerSpecificDetails")]
    pub provider_specific_details: UpdateApplianceForReplicationProtectedItemProviderSpecificInput,
}
impl UpdateApplianceForReplicationProtectedItemInputProperties {
    pub fn new(
        target_appliance_id: String,
        provider_specific_details: UpdateApplianceForReplicationProtectedItemProviderSpecificInput,
    ) -> Self {
        Self {
            target_appliance_id,
            provider_specific_details,
        }
    }
}
#[doc = "Update replication protected item provider specific input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateApplianceForReplicationProtectedItemProviderSpecificInput {
    #[doc = "The class type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl UpdateApplianceForReplicationProtectedItemProviderSpecificInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Disk input for update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateDiskInput {
    #[doc = "The disk Id."]
    #[serde(rename = "diskId")]
    pub disk_id: String,
    #[doc = "The target disk name."]
    #[serde(rename = "targetDiskName", default, skip_serializing_if = "Option::is_none")]
    pub target_disk_name: Option<String>,
}
impl UpdateDiskInput {
    pub fn new(disk_id: String) -> Self {
        Self {
            disk_id,
            target_disk_name: None,
        }
    }
}
#[doc = "Update migration item input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateMigrationItemInput {
    #[doc = "Update migration item input properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateMigrationItemInputProperties>,
}
impl UpdateMigrationItemInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Update migration item input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateMigrationItemInputProperties {
    #[doc = "Update migration item provider specific input."]
    #[serde(rename = "providerSpecificDetails")]
    pub provider_specific_details: UpdateMigrationItemProviderSpecificInput,
}
impl UpdateMigrationItemInputProperties {
    pub fn new(provider_specific_details: UpdateMigrationItemProviderSpecificInput) -> Self {
        Self { provider_specific_details }
    }
}
#[doc = "Update migration item provider specific input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateMigrationItemProviderSpecificInput {
    #[doc = "The class type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl UpdateMigrationItemProviderSpecificInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Request to update the mobility service on a protected item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateMobilityServiceRequest {
    #[doc = "The properties of an update mobility service request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateMobilityServiceRequestProperties>,
}
impl UpdateMobilityServiceRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an update mobility service request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateMobilityServiceRequestProperties {
    #[doc = "The CS run as account Id."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
}
impl UpdateMobilityServiceRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Update network mapping input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateNetworkMappingInput {
    #[doc = "Common input details for network mapping operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateNetworkMappingInputProperties>,
}
impl UpdateNetworkMappingInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common input details for network mapping operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateNetworkMappingInputProperties {
    #[doc = "Recovery fabric name."]
    #[serde(rename = "recoveryFabricName", default, skip_serializing_if = "Option::is_none")]
    pub recovery_fabric_name: Option<String>,
    #[doc = "Recovery network Id."]
    #[serde(rename = "recoveryNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_network_id: Option<String>,
    #[doc = "Input details specific to fabrics during Network Mapping."]
    #[serde(rename = "fabricSpecificDetails", default, skip_serializing_if = "Option::is_none")]
    pub fabric_specific_details: Option<FabricSpecificUpdateNetworkMappingInput>,
}
impl UpdateNetworkMappingInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Update policy input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdatePolicyInput {
    #[doc = "Policy update properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdatePolicyInputProperties>,
}
impl UpdatePolicyInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy update properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdatePolicyInputProperties {
    #[doc = "Base class for provider specific input."]
    #[serde(rename = "replicationProviderSettings", default, skip_serializing_if = "Option::is_none")]
    pub replication_provider_settings: Option<PolicyProviderSpecificInput>,
}
impl UpdatePolicyInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container pairing update input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateProtectionContainerMappingInput {
    #[doc = "Container pairing update input."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateProtectionContainerMappingInputProperties>,
}
impl UpdateProtectionContainerMappingInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container pairing update input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateProtectionContainerMappingInputProperties {
    #[doc = "Provider specific input for update pairing operations."]
    #[serde(rename = "providerSpecificInput", default, skip_serializing_if = "Option::is_none")]
    pub provider_specific_input: Option<ReplicationProviderSpecificUpdateContainerMappingInput>,
}
impl UpdateProtectionContainerMappingInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Update recovery plan input class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateRecoveryPlanInput {
    #[doc = "Recovery plan update properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateRecoveryPlanInputProperties>,
}
impl UpdateRecoveryPlanInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Recovery plan update properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateRecoveryPlanInputProperties {
    #[doc = "The recovery plan groups."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<RecoveryPlanGroup>,
}
impl UpdateRecoveryPlanInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Update replication protected item input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateReplicationProtectedItemInput {
    #[doc = "Update protected item input properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateReplicationProtectedItemInputProperties>,
}
impl UpdateReplicationProtectedItemInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Update protected item input properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateReplicationProtectedItemInputProperties {
    #[doc = "Target Azure VM name given by the user."]
    #[serde(rename = "recoveryAzureVMName", default, skip_serializing_if = "Option::is_none")]
    pub recovery_azure_vm_name: Option<String>,
    #[doc = "Target Azure VM size."]
    #[serde(rename = "recoveryAzureVMSize", default, skip_serializing_if = "Option::is_none")]
    pub recovery_azure_vm_size: Option<String>,
    #[doc = "Target Azure Network Id."]
    #[serde(rename = "selectedRecoveryAzureNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub selected_recovery_azure_network_id: Option<String>,
    #[doc = "The Azure Network Id for test failover."]
    #[serde(rename = "selectedTfoAzureNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub selected_tfo_azure_network_id: Option<String>,
    #[doc = "The selected source nic Id which will be used as the primary nic during failover."]
    #[serde(rename = "selectedSourceNicId", default, skip_serializing_if = "Option::is_none")]
    pub selected_source_nic_id: Option<String>,
    #[doc = "The selected option to enable RDP\\SSH on target vm after failover. String value of SrsDataContract.EnableRDPOnTargetOption enum."]
    #[serde(rename = "enableRdpOnTargetOption", default, skip_serializing_if = "Option::is_none")]
    pub enable_rdp_on_target_option: Option<String>,
    #[doc = "The list of VM nic details."]
    #[serde(rename = "vmNics", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_nics: Vec<VmNicInputDetails>,
    #[doc = "License type."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<update_replication_protected_item_input_properties::LicenseType>,
    #[doc = "The target availability set Id."]
    #[serde(rename = "recoveryAvailabilitySetId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_availability_set_id: Option<String>,
    #[doc = "Update replication protected item provider specific input."]
    #[serde(rename = "providerSpecificDetails", default, skip_serializing_if = "Option::is_none")]
    pub provider_specific_details: Option<UpdateReplicationProtectedItemProviderInput>,
}
impl UpdateReplicationProtectedItemInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod update_replication_protected_item_input_properties {
    use super::*;
    #[doc = "License type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseType")]
    pub enum LicenseType {
        NotSpecified,
        NoLicenseType,
        WindowsServer,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("LicenseType", 0u32, "NotSpecified"),
                Self::NoLicenseType => serializer.serialize_unit_variant("LicenseType", 1u32, "NoLicenseType"),
                Self::WindowsServer => serializer.serialize_unit_variant("LicenseType", 2u32, "WindowsServer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Update replication protected item provider specific input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateReplicationProtectedItemProviderInput {
    #[doc = "The class type."]
    #[serde(rename = "instanceType")]
    pub instance_type: String,
}
impl UpdateReplicationProtectedItemProviderInput {
    pub fn new(instance_type: String) -> Self {
        Self { instance_type }
    }
}
#[doc = "Input required to update vCenter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateVCenterRequest {
    #[doc = "The properties of an update vCenter request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateVCenterRequestProperties>,
}
impl UpdateVCenterRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an update vCenter request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateVCenterRequestProperties {
    #[doc = "The friendly name of the vCenter."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The IP address of the vCenter to be discovered."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The process server Id from where the update can be orchestrated."]
    #[serde(rename = "processServerId", default, skip_serializing_if = "Option::is_none")]
    pub process_server_id: Option<String>,
    #[doc = "The port number for discovery."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[doc = "The CS account Id which has privileges to update the vCenter."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
}
impl UpdateVCenterRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "vCenter definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VCenter {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "vCenter properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VCenterProperties>,
}
impl VCenter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of vCenter details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VCenterCollection {
    #[doc = "The vCenter details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VCenter>,
    #[doc = "The value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VCenterCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VCenterCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "vCenter properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VCenterProperties {
    #[doc = "Friendly name of the vCenter."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "VCenter internal ID."]
    #[serde(rename = "internalId", default, skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<String>,
    #[doc = "The time when the last heartbeat was received by vCenter."]
    #[serde(rename = "lastHeartbeat", with = "azure_core::date::rfc3339::option")]
    pub last_heartbeat: Option<time::OffsetDateTime>,
    #[doc = "The VCenter discovery status."]
    #[serde(rename = "discoveryStatus", default, skip_serializing_if = "Option::is_none")]
    pub discovery_status: Option<String>,
    #[doc = "The process server Id."]
    #[serde(rename = "processServerId", default, skip_serializing_if = "Option::is_none")]
    pub process_server_id: Option<String>,
    #[doc = "The IP address of the vCenter."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The infrastructure Id of vCenter."]
    #[serde(rename = "infrastructureId", default, skip_serializing_if = "Option::is_none")]
    pub infrastructure_id: Option<String>,
    #[doc = "The port number for discovery."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[doc = "The account Id which has privileges to discover the vCenter."]
    #[serde(rename = "runAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub run_as_account_id: Option<String>,
    #[doc = "The ARM resource name of the fabric containing this VCenter."]
    #[serde(rename = "fabricArmResourceName", default, skip_serializing_if = "Option::is_none")]
    pub fabric_arm_resource_name: Option<String>,
    #[doc = "The health errors for this VCenter."]
    #[serde(rename = "healthErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub health_errors: Vec<HealthError>,
}
impl VCenterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Hyper V VM network details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmNicDetails {
    #[doc = "The nic Id."]
    #[serde(rename = "nicId", default, skip_serializing_if = "Option::is_none")]
    pub nic_id: Option<String>,
    #[doc = "The replica nic Id."]
    #[serde(rename = "replicaNicId", default, skip_serializing_if = "Option::is_none")]
    pub replica_nic_id: Option<String>,
    #[doc = "The source nic ARM Id."]
    #[serde(rename = "sourceNicArmId", default, skip_serializing_if = "Option::is_none")]
    pub source_nic_arm_id: Option<String>,
    #[doc = "VM network name."]
    #[serde(rename = "vMNetworkName", default, skip_serializing_if = "Option::is_none")]
    pub v_m_network_name: Option<String>,
    #[doc = "Recovery VM network Id."]
    #[serde(rename = "recoveryVMNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_vm_network_id: Option<String>,
    #[doc = "The IP configurations of the NIC."]
    #[serde(rename = "ipConfigs", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_configs: Vec<IpConfigDetails>,
    #[doc = "Selection type for failover."]
    #[serde(rename = "selectionType", default, skip_serializing_if = "Option::is_none")]
    pub selection_type: Option<String>,
    #[doc = "The id of the NSG associated with the NIC."]
    #[serde(rename = "recoveryNetworkSecurityGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_network_security_group_id: Option<String>,
    #[doc = "A value indicating whether the NIC has accelerated networking enabled."]
    #[serde(rename = "enableAcceleratedNetworkingOnRecovery", default, skip_serializing_if = "Option::is_none")]
    pub enable_accelerated_networking_on_recovery: Option<bool>,
    #[doc = "The network to be used by NIC during test failover."]
    #[serde(rename = "tfoVMNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub tfo_vm_network_id: Option<String>,
    #[doc = "The NSG to be used by NIC during test failover."]
    #[serde(rename = "tfoNetworkSecurityGroupId", default, skip_serializing_if = "Option::is_none")]
    pub tfo_network_security_group_id: Option<String>,
    #[doc = "Whether the TFO NIC has accelerated networking enabled."]
    #[serde(rename = "enableAcceleratedNetworkingOnTfo", default, skip_serializing_if = "Option::is_none")]
    pub enable_accelerated_networking_on_tfo: Option<bool>,
    #[doc = "The name of the NIC to be used when creating target NICs."]
    #[serde(rename = "recoveryNicName", default, skip_serializing_if = "Option::is_none")]
    pub recovery_nic_name: Option<String>,
    #[doc = "The resource group of the NIC to be used when creating target NICs."]
    #[serde(rename = "recoveryNicResourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub recovery_nic_resource_group_name: Option<String>,
    #[doc = "A value indicating whether an existing NIC is allowed to be reused during failover subject to availability."]
    #[serde(rename = "reuseExistingNic", default, skip_serializing_if = "Option::is_none")]
    pub reuse_existing_nic: Option<bool>,
    #[doc = "The name of the NIC to be used when creating target NICs in TFO."]
    #[serde(rename = "tfoRecoveryNicName", default, skip_serializing_if = "Option::is_none")]
    pub tfo_recovery_nic_name: Option<String>,
    #[doc = "The resource group of the NIC to be used when creating target NICs in TFO."]
    #[serde(rename = "tfoRecoveryNicResourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub tfo_recovery_nic_resource_group_name: Option<String>,
    #[doc = "A value indicating whether an existing NIC is allowed to be reused during test failover subject to availability."]
    #[serde(rename = "tfoReuseExistingNic", default, skip_serializing_if = "Option::is_none")]
    pub tfo_reuse_existing_nic: Option<bool>,
    #[doc = "Target NIC name."]
    #[serde(rename = "targetNicName", default, skip_serializing_if = "Option::is_none")]
    pub target_nic_name: Option<String>,
}
impl VmNicDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Hyper V VM network input details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmNicInputDetails {
    #[doc = "The nic Id."]
    #[serde(rename = "nicId", default, skip_serializing_if = "Option::is_none")]
    pub nic_id: Option<String>,
    #[doc = "The IP configurations to be used by NIC during test failover and failover."]
    #[serde(rename = "ipConfigs", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_configs: Vec<IpConfigInputDetails>,
    #[doc = "Selection type for failover."]
    #[serde(rename = "selectionType", default, skip_serializing_if = "Option::is_none")]
    pub selection_type: Option<String>,
    #[doc = "The id of the NSG associated with the NIC."]
    #[serde(rename = "recoveryNetworkSecurityGroupId", default, skip_serializing_if = "Option::is_none")]
    pub recovery_network_security_group_id: Option<String>,
    #[doc = "Whether the NIC has accelerated networking enabled."]
    #[serde(rename = "enableAcceleratedNetworkingOnRecovery", default, skip_serializing_if = "Option::is_none")]
    pub enable_accelerated_networking_on_recovery: Option<bool>,
    #[doc = "The NSG to be used by NIC during test failover."]
    #[serde(rename = "tfoNetworkSecurityGroupId", default, skip_serializing_if = "Option::is_none")]
    pub tfo_network_security_group_id: Option<String>,
    #[doc = "Whether the test NIC has accelerated networking enabled."]
    #[serde(rename = "enableAcceleratedNetworkingOnTfo", default, skip_serializing_if = "Option::is_none")]
    pub enable_accelerated_networking_on_tfo: Option<bool>,
    #[doc = "The name of the NIC to be used when creating target NICs."]
    #[serde(rename = "recoveryNicName", default, skip_serializing_if = "Option::is_none")]
    pub recovery_nic_name: Option<String>,
    #[doc = "The resource group of the NIC to be used when creating target NICs."]
    #[serde(rename = "recoveryNicResourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub recovery_nic_resource_group_name: Option<String>,
    #[doc = "A value indicating whether an existing NIC is allowed to be reused during failover subject to availability."]
    #[serde(rename = "reuseExistingNic", default, skip_serializing_if = "Option::is_none")]
    pub reuse_existing_nic: Option<bool>,
    #[doc = "The name of the NIC to be used when creating target NICs in TFO."]
    #[serde(rename = "tfoNicName", default, skip_serializing_if = "Option::is_none")]
    pub tfo_nic_name: Option<String>,
    #[doc = "The resource group of the NIC to be used when creating target NICs in TFO."]
    #[serde(rename = "tfoNicResourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub tfo_nic_resource_group_name: Option<String>,
    #[doc = "A value indicating whether an existing NIC is allowed to be reused during test failover subject to availability."]
    #[serde(rename = "tfoReuseExistingNic", default, skip_serializing_if = "Option::is_none")]
    pub tfo_reuse_existing_nic: Option<bool>,
    #[doc = "Target NIC name."]
    #[serde(rename = "targetNicName", default, skip_serializing_if = "Option::is_none")]
    pub target_nic_name: Option<String>,
}
impl VmNicInputDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "VMwareCbt container creation input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareCbtContainerCreationInput {
    #[serde(flatten)]
    pub replication_provider_specific_container_creation_input: ReplicationProviderSpecificContainerCreationInput,
}
impl VMwareCbtContainerCreationInput {
    pub fn new(replication_provider_specific_container_creation_input: ReplicationProviderSpecificContainerCreationInput) -> Self {
        Self {
            replication_provider_specific_container_creation_input,
        }
    }
}
#[doc = "VMwareCbt container mapping input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareCbtContainerMappingInput {
    #[serde(flatten)]
    pub replication_provider_specific_container_mapping_input: ReplicationProviderSpecificContainerMappingInput,
    #[doc = "The target key vault ARM Id."]
    #[serde(rename = "keyVaultId", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_id: Option<String>,
    #[doc = "The target key vault URL."]
    #[serde(rename = "keyVaultUri", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_uri: Option<String>,
    #[doc = "The storage account ARM Id."]
    #[serde(rename = "storageAccountId")]
    pub storage_account_id: String,
    #[doc = "The secret name of the storage account."]
    #[serde(rename = "storageAccountSasSecretName", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_sas_secret_name: Option<String>,
    #[doc = "The secret name of the service bus connection string."]
    #[serde(rename = "serviceBusConnectionStringSecretName", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_connection_string_secret_name: Option<String>,
    #[doc = "The target location."]
    #[serde(rename = "targetLocation")]
    pub target_location: String,
}
impl VMwareCbtContainerMappingInput {
    pub fn new(
        replication_provider_specific_container_mapping_input: ReplicationProviderSpecificContainerMappingInput,
        storage_account_id: String,
        target_location: String,
    ) -> Self {
        Self {
            replication_provider_specific_container_mapping_input,
            key_vault_id: None,
            key_vault_uri: None,
            storage_account_id,
            storage_account_sas_secret_name: None,
            service_bus_connection_string_secret_name: None,
            target_location,
        }
    }
}
#[doc = "VMwareCbt disk input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareCbtDiskInput {
    #[doc = "The disk Id."]
    #[serde(rename = "diskId")]
    pub disk_id: String,
    #[doc = "The disk type."]
    #[serde(rename = "diskType", default, skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<v_mware_cbt_disk_input::DiskType>,
    #[doc = "A value indicating whether the disk is the OS disk."]
    #[serde(rename = "isOSDisk")]
    pub is_os_disk: String,
    #[doc = "The log storage account ARM Id."]
    #[serde(rename = "logStorageAccountId")]
    pub log_storage_account_id: String,
    #[doc = "The key vault secret name of the log storage account."]
    #[serde(rename = "logStorageAccountSasSecretName")]
    pub log_storage_account_sas_secret_name: String,
    #[doc = "The DiskEncryptionSet ARM Id."]
    #[serde(rename = "diskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set_id: Option<String>,
}
impl VMwareCbtDiskInput {
    pub fn new(disk_id: String, is_os_disk: String, log_storage_account_id: String, log_storage_account_sas_secret_name: String) -> Self {
        Self {
            disk_id,
            disk_type: None,
            is_os_disk,
            log_storage_account_id,
            log_storage_account_sas_secret_name,
            disk_encryption_set_id: None,
        }
    }
}
pub mod v_mware_cbt_disk_input {
    use super::*;
    #[doc = "The disk type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DiskType")]
    pub enum DiskType {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(rename = "StandardSSD_LRS")]
        StandardSsdLrs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DiskType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DiskType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DiskType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::StandardLrs => serializer.serialize_unit_variant("DiskType", 0u32, "Standard_LRS"),
                Self::PremiumLrs => serializer.serialize_unit_variant("DiskType", 1u32, "Premium_LRS"),
                Self::StandardSsdLrs => serializer.serialize_unit_variant("DiskType", 2u32, "StandardSSD_LRS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "VMwareCbt specific enable migration input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareCbtEnableMigrationInput {
    #[serde(flatten)]
    pub enable_migration_provider_specific_input: EnableMigrationProviderSpecificInput,
    #[doc = "The ARM Id of the VM discovered in VMware."]
    #[serde(rename = "vmwareMachineId")]
    pub vmware_machine_id: String,
    #[doc = "The disks to include list."]
    #[serde(rename = "disksToInclude")]
    pub disks_to_include: Vec<VMwareCbtDiskInput>,
    #[doc = "License type."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<v_mware_cbt_enable_migration_input::LicenseType>,
    #[doc = "The SQL Server license type."]
    #[serde(rename = "sqlServerLicenseType", default, skip_serializing_if = "Option::is_none")]
    pub sql_server_license_type: Option<v_mware_cbt_enable_migration_input::SqlServerLicenseType>,
    #[doc = "The data mover run as account Id."]
    #[serde(rename = "dataMoverRunAsAccountId")]
    pub data_mover_run_as_account_id: String,
    #[doc = "The snapshot run as account Id."]
    #[serde(rename = "snapshotRunAsAccountId")]
    pub snapshot_run_as_account_id: String,
    #[doc = "The target VM name."]
    #[serde(rename = "targetVmName", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_name: Option<String>,
    #[doc = "The target VM size."]
    #[serde(rename = "targetVmSize", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_size: Option<String>,
    #[doc = "The target resource group ARM Id."]
    #[serde(rename = "targetResourceGroupId")]
    pub target_resource_group_id: String,
    #[doc = "The target network ARM Id."]
    #[serde(rename = "targetNetworkId")]
    pub target_network_id: String,
    #[doc = "The selected test network ARM Id."]
    #[serde(rename = "testNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub test_network_id: Option<String>,
    #[doc = "The target subnet name."]
    #[serde(rename = "targetSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub target_subnet_name: Option<String>,
    #[doc = "The selected test subnet name."]
    #[serde(rename = "testSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub test_subnet_name: Option<String>,
    #[doc = "The target availability set ARM Id."]
    #[serde(rename = "targetAvailabilitySetId", default, skip_serializing_if = "Option::is_none")]
    pub target_availability_set_id: Option<String>,
    #[doc = "The target availability zone."]
    #[serde(rename = "targetAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub target_availability_zone: Option<String>,
    #[doc = "The target proximity placement group ARM Id."]
    #[serde(rename = "targetProximityPlacementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub target_proximity_placement_group_id: Option<String>,
    #[doc = "The target boot diagnostics storage account ARM Id."]
    #[serde(rename = "targetBootDiagnosticsStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub target_boot_diagnostics_storage_account_id: Option<String>,
    #[doc = "A value indicating whether auto resync is to be done."]
    #[serde(rename = "performAutoResync", default, skip_serializing_if = "Option::is_none")]
    pub perform_auto_resync: Option<String>,
    #[doc = "The target VM tags."]
    #[serde(rename = "targetVmTags", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_tags: Option<serde_json::Value>,
    #[doc = "The tags for the seed disks."]
    #[serde(rename = "seedDiskTags", default, skip_serializing_if = "Option::is_none")]
    pub seed_disk_tags: Option<serde_json::Value>,
    #[doc = "The tags for the target disks."]
    #[serde(rename = "targetDiskTags", default, skip_serializing_if = "Option::is_none")]
    pub target_disk_tags: Option<serde_json::Value>,
    #[doc = "The tags for the target NICs."]
    #[serde(rename = "targetNicTags", default, skip_serializing_if = "Option::is_none")]
    pub target_nic_tags: Option<serde_json::Value>,
}
impl VMwareCbtEnableMigrationInput {
    pub fn new(
        enable_migration_provider_specific_input: EnableMigrationProviderSpecificInput,
        vmware_machine_id: String,
        disks_to_include: Vec<VMwareCbtDiskInput>,
        data_mover_run_as_account_id: String,
        snapshot_run_as_account_id: String,
        target_resource_group_id: String,
        target_network_id: String,
    ) -> Self {
        Self {
            enable_migration_provider_specific_input,
            vmware_machine_id,
            disks_to_include,
            license_type: None,
            sql_server_license_type: None,
            data_mover_run_as_account_id,
            snapshot_run_as_account_id,
            target_vm_name: None,
            target_vm_size: None,
            target_resource_group_id,
            target_network_id,
            test_network_id: None,
            target_subnet_name: None,
            test_subnet_name: None,
            target_availability_set_id: None,
            target_availability_zone: None,
            target_proximity_placement_group_id: None,
            target_boot_diagnostics_storage_account_id: None,
            perform_auto_resync: None,
            target_vm_tags: None,
            seed_disk_tags: None,
            target_disk_tags: None,
            target_nic_tags: None,
        }
    }
}
pub mod v_mware_cbt_enable_migration_input {
    use super::*;
    #[doc = "License type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseType")]
    pub enum LicenseType {
        NotSpecified,
        NoLicenseType,
        WindowsServer,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("LicenseType", 0u32, "NotSpecified"),
                Self::NoLicenseType => serializer.serialize_unit_variant("LicenseType", 1u32, "NoLicenseType"),
                Self::WindowsServer => serializer.serialize_unit_variant("LicenseType", 2u32, "WindowsServer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The SQL Server license type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SqlServerLicenseType")]
    pub enum SqlServerLicenseType {
        NotSpecified,
        NoLicenseType,
        #[serde(rename = "PAYG")]
        Payg,
        #[serde(rename = "AHUB")]
        Ahub,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SqlServerLicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SqlServerLicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SqlServerLicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("SqlServerLicenseType", 0u32, "NotSpecified"),
                Self::NoLicenseType => serializer.serialize_unit_variant("SqlServerLicenseType", 1u32, "NoLicenseType"),
                Self::Payg => serializer.serialize_unit_variant("SqlServerLicenseType", 2u32, "PAYG"),
                Self::Ahub => serializer.serialize_unit_variant("SqlServerLicenseType", 3u32, "AHUB"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Event details for VMwareCbt provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareCbtEventDetails {
    #[serde(flatten)]
    pub event_provider_specific_details: EventProviderSpecificDetails,
    #[doc = "The migration item name."]
    #[serde(rename = "migrationItemName", default, skip_serializing_if = "Option::is_none")]
    pub migration_item_name: Option<String>,
}
impl VMwareCbtEventDetails {
    pub fn new(event_provider_specific_details: EventProviderSpecificDetails) -> Self {
        Self {
            event_provider_specific_details,
            migration_item_name: None,
        }
    }
}
#[doc = "VMwareCbt specific migrate input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareCbtMigrateInput {
    #[serde(flatten)]
    pub migrate_provider_specific_input: MigrateProviderSpecificInput,
    #[doc = "A value indicating whether VM is to be shutdown."]
    #[serde(rename = "performShutdown")]
    pub perform_shutdown: String,
}
impl VMwareCbtMigrateInput {
    pub fn new(migrate_provider_specific_input: MigrateProviderSpecificInput, perform_shutdown: String) -> Self {
        Self {
            migrate_provider_specific_input,
            perform_shutdown,
        }
    }
}
#[doc = "VMwareCbt provider specific settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareCbtMigrationDetails {
    #[serde(flatten)]
    pub migration_provider_specific_settings: MigrationProviderSpecificSettings,
    #[doc = "The ARM Id of the VM discovered in VMware."]
    #[serde(rename = "vmwareMachineId", default, skip_serializing_if = "Option::is_none")]
    pub vmware_machine_id: Option<String>,
    #[doc = "The type of the OS on the VM."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "The firmware type."]
    #[serde(rename = "firmwareType", default, skip_serializing_if = "Option::is_none")]
    pub firmware_type: Option<String>,
    #[doc = "The target generation."]
    #[serde(rename = "targetGeneration", default, skip_serializing_if = "Option::is_none")]
    pub target_generation: Option<String>,
    #[doc = "License Type of the VM to be used."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    #[doc = "The SQL Server license type."]
    #[serde(rename = "sqlServerLicenseType", default, skip_serializing_if = "Option::is_none")]
    pub sql_server_license_type: Option<String>,
    #[doc = "The data mover run as account Id."]
    #[serde(rename = "dataMoverRunAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub data_mover_run_as_account_id: Option<String>,
    #[doc = "The snapshot run as account Id."]
    #[serde(rename = "snapshotRunAsAccountId", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_run_as_account_id: Option<String>,
    #[doc = "Target VM name."]
    #[serde(rename = "targetVmName", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_name: Option<String>,
    #[doc = "The target VM size."]
    #[serde(rename = "targetVmSize", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_size: Option<String>,
    #[doc = "The target location."]
    #[serde(rename = "targetLocation", default, skip_serializing_if = "Option::is_none")]
    pub target_location: Option<String>,
    #[doc = "The target resource group Id."]
    #[serde(rename = "targetResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_group_id: Option<String>,
    #[doc = "The target availability set Id."]
    #[serde(rename = "targetAvailabilitySetId", default, skip_serializing_if = "Option::is_none")]
    pub target_availability_set_id: Option<String>,
    #[doc = "The target availability zone."]
    #[serde(rename = "targetAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub target_availability_zone: Option<String>,
    #[doc = "The target proximity placement group Id."]
    #[serde(rename = "targetProximityPlacementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub target_proximity_placement_group_id: Option<String>,
    #[doc = "The target boot diagnostics storage account ARM Id."]
    #[serde(rename = "targetBootDiagnosticsStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub target_boot_diagnostics_storage_account_id: Option<String>,
    #[doc = "The target VM tags."]
    #[serde(rename = "targetVmTags", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_tags: Option<serde_json::Value>,
    #[doc = "The list of protected disks."]
    #[serde(rename = "protectedDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub protected_disks: Vec<VMwareCbtProtectedDiskDetails>,
    #[doc = "The target network Id."]
    #[serde(rename = "targetNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub target_network_id: Option<String>,
    #[doc = "The network details."]
    #[serde(rename = "vmNics", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_nics: Vec<VMwareCbtNicDetails>,
    #[doc = "The tags for the target NICs."]
    #[serde(rename = "targetNicTags", default, skip_serializing_if = "Option::is_none")]
    pub target_nic_tags: Option<serde_json::Value>,
    #[doc = "The recovery point Id to which the VM was migrated."]
    #[serde(rename = "migrationRecoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub migration_recovery_point_id: Option<String>,
    #[doc = "The last recovery point received time."]
    #[serde(rename = "lastRecoveryPointReceived", with = "azure_core::date::rfc3339::option")]
    pub last_recovery_point_received: Option<time::OffsetDateTime>,
    #[doc = "The last recovery point Id."]
    #[serde(rename = "lastRecoveryPointId", default, skip_serializing_if = "Option::is_none")]
    pub last_recovery_point_id: Option<String>,
    #[doc = "The initial seeding progress percentage."]
    #[serde(rename = "initialSeedingProgressPercentage", default, skip_serializing_if = "Option::is_none")]
    pub initial_seeding_progress_percentage: Option<i32>,
    #[doc = "The migration progress percentage."]
    #[serde(rename = "migrationProgressPercentage", default, skip_serializing_if = "Option::is_none")]
    pub migration_progress_percentage: Option<i32>,
    #[doc = "The resync progress percentage."]
    #[serde(rename = "resyncProgressPercentage", default, skip_serializing_if = "Option::is_none")]
    pub resync_progress_percentage: Option<i32>,
    #[doc = "The initial seeding retry count."]
    #[serde(rename = "initialSeedingRetryCount", default, skip_serializing_if = "Option::is_none")]
    pub initial_seeding_retry_count: Option<i64>,
    #[doc = "The resync retry count."]
    #[serde(rename = "resyncRetryCount", default, skip_serializing_if = "Option::is_none")]
    pub resync_retry_count: Option<i64>,
    #[doc = "A value indicating whether resync is required."]
    #[serde(rename = "resyncRequired", default, skip_serializing_if = "Option::is_none")]
    pub resync_required: Option<String>,
    #[doc = "The resync state."]
    #[serde(rename = "resyncState", default, skip_serializing_if = "Option::is_none")]
    pub resync_state: Option<v_mware_cbt_migration_details::ResyncState>,
    #[doc = "A value indicating whether auto resync is to be done."]
    #[serde(rename = "performAutoResync", default, skip_serializing_if = "Option::is_none")]
    pub perform_auto_resync: Option<String>,
    #[doc = "The tags for the seed disks."]
    #[serde(rename = "seedDiskTags", default, skip_serializing_if = "Option::is_none")]
    pub seed_disk_tags: Option<serde_json::Value>,
    #[doc = "The tags for the target disks."]
    #[serde(rename = "targetDiskTags", default, skip_serializing_if = "Option::is_none")]
    pub target_disk_tags: Option<serde_json::Value>,
}
impl VMwareCbtMigrationDetails {
    pub fn new(migration_provider_specific_settings: MigrationProviderSpecificSettings) -> Self {
        Self {
            migration_provider_specific_settings,
            vmware_machine_id: None,
            os_type: None,
            firmware_type: None,
            target_generation: None,
            license_type: None,
            sql_server_license_type: None,
            data_mover_run_as_account_id: None,
            snapshot_run_as_account_id: None,
            target_vm_name: None,
            target_vm_size: None,
            target_location: None,
            target_resource_group_id: None,
            target_availability_set_id: None,
            target_availability_zone: None,
            target_proximity_placement_group_id: None,
            target_boot_diagnostics_storage_account_id: None,
            target_vm_tags: None,
            protected_disks: Vec::new(),
            target_network_id: None,
            vm_nics: Vec::new(),
            target_nic_tags: None,
            migration_recovery_point_id: None,
            last_recovery_point_received: None,
            last_recovery_point_id: None,
            initial_seeding_progress_percentage: None,
            migration_progress_percentage: None,
            resync_progress_percentage: None,
            initial_seeding_retry_count: None,
            resync_retry_count: None,
            resync_required: None,
            resync_state: None,
            perform_auto_resync: None,
            seed_disk_tags: None,
            target_disk_tags: None,
        }
    }
}
pub mod v_mware_cbt_migration_details {
    use super::*;
    #[doc = "The resync state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ResyncState")]
    pub enum ResyncState {
        None,
        PreparedForResynchronization,
        StartedResynchronization,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ResyncState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ResyncState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ResyncState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ResyncState", 0u32, "None"),
                Self::PreparedForResynchronization => {
                    serializer.serialize_unit_variant("ResyncState", 1u32, "PreparedForResynchronization")
                }
                Self::StartedResynchronization => serializer.serialize_unit_variant("ResyncState", 2u32, "StartedResynchronization"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "VMwareCbt NIC details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VMwareCbtNicDetails {
    #[doc = "The NIC Id."]
    #[serde(rename = "nicId", default, skip_serializing_if = "Option::is_none")]
    pub nic_id: Option<String>,
    #[doc = "A value indicating whether this is the primary NIC."]
    #[serde(rename = "isPrimaryNic", default, skip_serializing_if = "Option::is_none")]
    pub is_primary_nic: Option<String>,
    #[doc = "The source IP address."]
    #[serde(rename = "sourceIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub source_ip_address: Option<String>,
    #[doc = "The source IP address type."]
    #[serde(rename = "sourceIPAddressType", default, skip_serializing_if = "Option::is_none")]
    pub source_ip_address_type: Option<v_mware_cbt_nic_details::SourceIpAddressType>,
    #[doc = "Source network Id."]
    #[serde(rename = "sourceNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub source_network_id: Option<String>,
    #[doc = "The target IP address."]
    #[serde(rename = "targetIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub target_ip_address: Option<String>,
    #[doc = "The target IP address type."]
    #[serde(rename = "targetIPAddressType", default, skip_serializing_if = "Option::is_none")]
    pub target_ip_address_type: Option<v_mware_cbt_nic_details::TargetIpAddressType>,
    #[doc = "Target subnet name."]
    #[serde(rename = "targetSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub target_subnet_name: Option<String>,
    #[doc = "Source network Id."]
    #[serde(rename = "testNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub test_network_id: Option<String>,
    #[doc = "Test subnet name."]
    #[serde(rename = "testSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub test_subnet_name: Option<String>,
    #[doc = "The test IP address."]
    #[serde(rename = "testIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub test_ip_address: Option<String>,
    #[doc = "The test IP address type."]
    #[serde(rename = "testIPAddressType", default, skip_serializing_if = "Option::is_none")]
    pub test_ip_address_type: Option<v_mware_cbt_nic_details::TestIpAddressType>,
    #[doc = "Target NIC name."]
    #[serde(rename = "targetNicName", default, skip_serializing_if = "Option::is_none")]
    pub target_nic_name: Option<String>,
    #[doc = "A value indicating whether this NIC is selected for migration."]
    #[serde(rename = "isSelectedForMigration", default, skip_serializing_if = "Option::is_none")]
    pub is_selected_for_migration: Option<String>,
}
impl VMwareCbtNicDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod v_mware_cbt_nic_details {
    use super::*;
    #[doc = "The source IP address type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SourceIpAddressType")]
    pub enum SourceIpAddressType {
        Dynamic,
        Static,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SourceIpAddressType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SourceIpAddressType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SourceIpAddressType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Dynamic => serializer.serialize_unit_variant("SourceIpAddressType", 0u32, "Dynamic"),
                Self::Static => serializer.serialize_unit_variant("SourceIpAddressType", 1u32, "Static"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The target IP address type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TargetIpAddressType")]
    pub enum TargetIpAddressType {
        Dynamic,
        Static,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TargetIpAddressType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TargetIpAddressType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TargetIpAddressType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Dynamic => serializer.serialize_unit_variant("TargetIpAddressType", 0u32, "Dynamic"),
                Self::Static => serializer.serialize_unit_variant("TargetIpAddressType", 1u32, "Static"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The test IP address type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TestIpAddressType")]
    pub enum TestIpAddressType {
        Dynamic,
        Static,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TestIpAddressType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TestIpAddressType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TestIpAddressType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Dynamic => serializer.serialize_unit_variant("TestIpAddressType", 0u32, "Dynamic"),
                Self::Static => serializer.serialize_unit_variant("TestIpAddressType", 1u32, "Static"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "VMwareCbt NIC input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareCbtNicInput {
    #[doc = "The NIC Id."]
    #[serde(rename = "nicId")]
    pub nic_id: String,
    #[doc = "A value indicating whether this is the primary NIC."]
    #[serde(rename = "isPrimaryNic")]
    pub is_primary_nic: String,
    #[doc = "Target subnet name."]
    #[serde(rename = "targetSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub target_subnet_name: Option<String>,
    #[doc = "The static IP address."]
    #[serde(rename = "targetStaticIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub target_static_ip_address: Option<String>,
    #[doc = "A value indicating whether this NIC is selected for migration."]
    #[serde(rename = "isSelectedForMigration", default, skip_serializing_if = "Option::is_none")]
    pub is_selected_for_migration: Option<String>,
    #[doc = "Target NIC name."]
    #[serde(rename = "targetNicName", default, skip_serializing_if = "Option::is_none")]
    pub target_nic_name: Option<String>,
    #[doc = "The test subnet name."]
    #[serde(rename = "testSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub test_subnet_name: Option<String>,
    #[doc = "The test static IP address."]
    #[serde(rename = "testStaticIPAddress", default, skip_serializing_if = "Option::is_none")]
    pub test_static_ip_address: Option<String>,
}
impl VMwareCbtNicInput {
    pub fn new(nic_id: String, is_primary_nic: String) -> Self {
        Self {
            nic_id,
            is_primary_nic,
            target_subnet_name: None,
            target_static_ip_address: None,
            is_selected_for_migration: None,
            target_nic_name: None,
            test_subnet_name: None,
            test_static_ip_address: None,
        }
    }
}
#[doc = "VMware Cbt policy creation input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareCbtPolicyCreationInput {
    #[serde(flatten)]
    pub policy_provider_specific_input: PolicyProviderSpecificInput,
    #[doc = "The duration in minutes until which the recovery points need to be stored."]
    #[serde(rename = "recoveryPointHistoryInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_history_in_minutes: Option<i32>,
    #[doc = "The crash consistent snapshot frequency (in minutes)."]
    #[serde(rename = "crashConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub crash_consistent_frequency_in_minutes: Option<i32>,
    #[doc = "The app consistent snapshot frequency (in minutes)."]
    #[serde(rename = "appConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub app_consistent_frequency_in_minutes: Option<i32>,
}
impl VMwareCbtPolicyCreationInput {
    pub fn new(policy_provider_specific_input: PolicyProviderSpecificInput) -> Self {
        Self {
            policy_provider_specific_input,
            recovery_point_history_in_minutes: None,
            crash_consistent_frequency_in_minutes: None,
            app_consistent_frequency_in_minutes: None,
        }
    }
}
#[doc = "VMwareCbt protected disk details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VMwareCbtProtectedDiskDetails {
    #[doc = "The disk id."]
    #[serde(rename = "diskId", default, skip_serializing_if = "Option::is_none")]
    pub disk_id: Option<String>,
    #[doc = "The disk name."]
    #[serde(rename = "diskName", default, skip_serializing_if = "Option::is_none")]
    pub disk_name: Option<String>,
    #[doc = "The disk type."]
    #[serde(rename = "diskType", default, skip_serializing_if = "Option::is_none")]
    pub disk_type: Option<v_mware_cbt_protected_disk_details::DiskType>,
    #[doc = "The disk path."]
    #[serde(rename = "diskPath", default, skip_serializing_if = "Option::is_none")]
    pub disk_path: Option<String>,
    #[doc = "A value indicating whether the disk is the OS disk."]
    #[serde(rename = "isOSDisk", default, skip_serializing_if = "Option::is_none")]
    pub is_os_disk: Option<String>,
    #[doc = "The disk capacity in bytes."]
    #[serde(rename = "capacityInBytes", default, skip_serializing_if = "Option::is_none")]
    pub capacity_in_bytes: Option<i64>,
    #[doc = "The log storage account ARM Id."]
    #[serde(rename = "logStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub log_storage_account_id: Option<String>,
    #[doc = "The key vault secret name of the log storage account."]
    #[serde(rename = "logStorageAccountSasSecretName", default, skip_serializing_if = "Option::is_none")]
    pub log_storage_account_sas_secret_name: Option<String>,
    #[doc = "The DiskEncryptionSet ARM Id."]
    #[serde(rename = "diskEncryptionSetId", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_set_id: Option<String>,
    #[doc = "The ARM Id of the seed managed disk."]
    #[serde(rename = "seedManagedDiskId", default, skip_serializing_if = "Option::is_none")]
    pub seed_managed_disk_id: Option<String>,
    #[doc = "The ARM Id of the target managed disk."]
    #[serde(rename = "targetManagedDiskId", default, skip_serializing_if = "Option::is_none")]
    pub target_managed_disk_id: Option<String>,
    #[doc = "The name for the target managed disk."]
    #[serde(rename = "targetDiskName", default, skip_serializing_if = "Option::is_none")]
    pub target_disk_name: Option<String>,
}
impl VMwareCbtProtectedDiskDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod v_mware_cbt_protected_disk_details {
    use super::*;
    #[doc = "The disk type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DiskType")]
    pub enum DiskType {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(rename = "StandardSSD_LRS")]
        StandardSsdLrs,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DiskType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DiskType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DiskType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::StandardLrs => serializer.serialize_unit_variant("DiskType", 0u32, "Standard_LRS"),
                Self::PremiumLrs => serializer.serialize_unit_variant("DiskType", 1u32, "Premium_LRS"),
                Self::StandardSsdLrs => serializer.serialize_unit_variant("DiskType", 2u32, "StandardSSD_LRS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "VMwareCbt provider specific container mapping details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareCbtProtectionContainerMappingDetails {
    #[serde(flatten)]
    pub protection_container_mapping_provider_specific_details: ProtectionContainerMappingProviderSpecificDetails,
    #[doc = "The target key vault ARM Id."]
    #[serde(rename = "keyVaultId", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_id: Option<String>,
    #[doc = "The target key vault URI."]
    #[serde(rename = "keyVaultUri", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_uri: Option<String>,
    #[doc = "The storage account ARM Id."]
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[doc = "The secret name of the storage account."]
    #[serde(rename = "storageAccountSasSecretName", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_sas_secret_name: Option<String>,
    #[doc = "The secret name of the service bus connection string."]
    #[serde(rename = "serviceBusConnectionStringSecretName", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_connection_string_secret_name: Option<String>,
    #[doc = "The target location."]
    #[serde(rename = "targetLocation", default, skip_serializing_if = "Option::is_none")]
    pub target_location: Option<String>,
}
impl VMwareCbtProtectionContainerMappingDetails {
    pub fn new(protection_container_mapping_provider_specific_details: ProtectionContainerMappingProviderSpecificDetails) -> Self {
        Self {
            protection_container_mapping_provider_specific_details,
            key_vault_id: None,
            key_vault_uri: None,
            storage_account_id: None,
            storage_account_sas_secret_name: None,
            service_bus_connection_string_secret_name: None,
            target_location: None,
        }
    }
}
#[doc = "VMwareCbt specific resync input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareCbtResyncInput {
    #[serde(flatten)]
    pub resync_provider_specific_input: ResyncProviderSpecificInput,
    #[doc = "A value indicating whether CBT is to be reset."]
    #[serde(rename = "skipCbtReset")]
    pub skip_cbt_reset: String,
}
impl VMwareCbtResyncInput {
    pub fn new(resync_provider_specific_input: ResyncProviderSpecificInput, skip_cbt_reset: String) -> Self {
        Self {
            resync_provider_specific_input,
            skip_cbt_reset,
        }
    }
}
#[doc = "VMwareCbt specific test migrate input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareCbtTestMigrateInput {
    #[serde(flatten)]
    pub test_migrate_provider_specific_input: TestMigrateProviderSpecificInput,
    #[doc = "The recovery point Id."]
    #[serde(rename = "recoveryPointId")]
    pub recovery_point_id: String,
    #[doc = "The test network Id."]
    #[serde(rename = "networkId")]
    pub network_id: String,
    #[doc = "The list of NIC details."]
    #[serde(rename = "vmNics", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_nics: Vec<VMwareCbtNicInput>,
}
impl VMwareCbtTestMigrateInput {
    pub fn new(
        test_migrate_provider_specific_input: TestMigrateProviderSpecificInput,
        recovery_point_id: String,
        network_id: String,
    ) -> Self {
        Self {
            test_migrate_provider_specific_input,
            recovery_point_id,
            network_id,
            vm_nics: Vec::new(),
        }
    }
}
#[doc = "VMwareCbt disk input for update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareCbtUpdateDiskInput {
    #[doc = "The disk Id."]
    #[serde(rename = "diskId")]
    pub disk_id: String,
    #[doc = "The target disk name."]
    #[serde(rename = "targetDiskName", default, skip_serializing_if = "Option::is_none")]
    pub target_disk_name: Option<String>,
}
impl VMwareCbtUpdateDiskInput {
    pub fn new(disk_id: String) -> Self {
        Self {
            disk_id,
            target_disk_name: None,
        }
    }
}
#[doc = "VMwareCbt specific update migration item input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareCbtUpdateMigrationItemInput {
    #[serde(flatten)]
    pub update_migration_item_provider_specific_input: UpdateMigrationItemProviderSpecificInput,
    #[doc = "The target VM name."]
    #[serde(rename = "targetVmName", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_name: Option<String>,
    #[doc = "The target VM size."]
    #[serde(rename = "targetVmSize", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_size: Option<String>,
    #[doc = "The target resource group ARM Id."]
    #[serde(rename = "targetResourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_group_id: Option<String>,
    #[doc = "The target availability set ARM Id."]
    #[serde(rename = "targetAvailabilitySetId", default, skip_serializing_if = "Option::is_none")]
    pub target_availability_set_id: Option<String>,
    #[doc = "The target availability zone."]
    #[serde(rename = "targetAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub target_availability_zone: Option<String>,
    #[doc = "The target proximity placement group ARM Id."]
    #[serde(rename = "targetProximityPlacementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub target_proximity_placement_group_id: Option<String>,
    #[doc = "The target boot diagnostics storage account ARM Id."]
    #[serde(rename = "targetBootDiagnosticsStorageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub target_boot_diagnostics_storage_account_id: Option<String>,
    #[doc = "The target network ARM Id."]
    #[serde(rename = "targetNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub target_network_id: Option<String>,
    #[doc = "The test network ARM Id."]
    #[serde(rename = "testNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub test_network_id: Option<String>,
    #[doc = "The list of NIC details."]
    #[serde(rename = "vmNics", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_nics: Vec<VMwareCbtNicInput>,
    #[doc = "The list of disk update properties."]
    #[serde(rename = "vmDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_disks: Vec<VMwareCbtUpdateDiskInput>,
    #[doc = "The license type."]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<v_mware_cbt_update_migration_item_input::LicenseType>,
    #[doc = "The SQL Server license type."]
    #[serde(rename = "sqlServerLicenseType", default, skip_serializing_if = "Option::is_none")]
    pub sql_server_license_type: Option<v_mware_cbt_update_migration_item_input::SqlServerLicenseType>,
    #[doc = "A value indicating whether auto resync is to be done."]
    #[serde(rename = "performAutoResync", default, skip_serializing_if = "Option::is_none")]
    pub perform_auto_resync: Option<String>,
    #[doc = "The target VM tags."]
    #[serde(rename = "targetVmTags", default, skip_serializing_if = "Option::is_none")]
    pub target_vm_tags: Option<serde_json::Value>,
    #[doc = "The tags for the target disks."]
    #[serde(rename = "targetDiskTags", default, skip_serializing_if = "Option::is_none")]
    pub target_disk_tags: Option<serde_json::Value>,
    #[doc = "The tags for the target NICs."]
    #[serde(rename = "targetNicTags", default, skip_serializing_if = "Option::is_none")]
    pub target_nic_tags: Option<serde_json::Value>,
}
impl VMwareCbtUpdateMigrationItemInput {
    pub fn new(update_migration_item_provider_specific_input: UpdateMigrationItemProviderSpecificInput) -> Self {
        Self {
            update_migration_item_provider_specific_input,
            target_vm_name: None,
            target_vm_size: None,
            target_resource_group_id: None,
            target_availability_set_id: None,
            target_availability_zone: None,
            target_proximity_placement_group_id: None,
            target_boot_diagnostics_storage_account_id: None,
            target_network_id: None,
            test_network_id: None,
            vm_nics: Vec::new(),
            vm_disks: Vec::new(),
            license_type: None,
            sql_server_license_type: None,
            perform_auto_resync: None,
            target_vm_tags: None,
            target_disk_tags: None,
            target_nic_tags: None,
        }
    }
}
pub mod v_mware_cbt_update_migration_item_input {
    use super::*;
    #[doc = "The license type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LicenseType")]
    pub enum LicenseType {
        NotSpecified,
        NoLicenseType,
        WindowsServer,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("LicenseType", 0u32, "NotSpecified"),
                Self::NoLicenseType => serializer.serialize_unit_variant("LicenseType", 1u32, "NoLicenseType"),
                Self::WindowsServer => serializer.serialize_unit_variant("LicenseType", 2u32, "WindowsServer"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The SQL Server license type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SqlServerLicenseType")]
    pub enum SqlServerLicenseType {
        NotSpecified,
        NoLicenseType,
        #[serde(rename = "PAYG")]
        Payg,
        #[serde(rename = "AHUB")]
        Ahub,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SqlServerLicenseType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SqlServerLicenseType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SqlServerLicenseType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("SqlServerLicenseType", 0u32, "NotSpecified"),
                Self::NoLicenseType => serializer.serialize_unit_variant("SqlServerLicenseType", 1u32, "NoLicenseType"),
                Self::Payg => serializer.serialize_unit_variant("SqlServerLicenseType", 2u32, "PAYG"),
                Self::Ahub => serializer.serialize_unit_variant("SqlServerLicenseType", 3u32, "AHUB"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Store the fabric details specific to the VMware fabric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareDetails {
    #[serde(flatten)]
    pub fabric_specific_details: FabricSpecificDetails,
    #[doc = "The list of Process Servers associated with the fabric."]
    #[serde(rename = "processServers", default, skip_serializing_if = "Vec::is_empty")]
    pub process_servers: Vec<ProcessServer>,
    #[doc = "The list of Master Target servers associated with the fabric."]
    #[serde(rename = "masterTargetServers", default, skip_serializing_if = "Vec::is_empty")]
    pub master_target_servers: Vec<MasterTargetServer>,
    #[doc = "The list of run as accounts created on the server."]
    #[serde(rename = "runAsAccounts", default, skip_serializing_if = "Vec::is_empty")]
    pub run_as_accounts: Vec<RunAsAccount>,
    #[doc = "The number of replication pairs configured in this CS."]
    #[serde(rename = "replicationPairCount", default, skip_serializing_if = "Option::is_none")]
    pub replication_pair_count: Option<String>,
    #[doc = "The number of process servers."]
    #[serde(rename = "processServerCount", default, skip_serializing_if = "Option::is_none")]
    pub process_server_count: Option<String>,
    #[doc = "The number of source and target servers configured to talk to this CS."]
    #[serde(rename = "agentCount", default, skip_serializing_if = "Option::is_none")]
    pub agent_count: Option<String>,
    #[doc = "The number of protected servers."]
    #[serde(rename = "protectedServers", default, skip_serializing_if = "Option::is_none")]
    pub protected_servers: Option<String>,
    #[doc = "The percentage of the system load."]
    #[serde(rename = "systemLoad", default, skip_serializing_if = "Option::is_none")]
    pub system_load: Option<String>,
    #[doc = "The system load status."]
    #[serde(rename = "systemLoadStatus", default, skip_serializing_if = "Option::is_none")]
    pub system_load_status: Option<String>,
    #[doc = "The percentage of the CPU load."]
    #[serde(rename = "cpuLoad", default, skip_serializing_if = "Option::is_none")]
    pub cpu_load: Option<String>,
    #[doc = "The CPU load status."]
    #[serde(rename = "cpuLoadStatus", default, skip_serializing_if = "Option::is_none")]
    pub cpu_load_status: Option<String>,
    #[doc = "The total memory."]
    #[serde(rename = "totalMemoryInBytes", default, skip_serializing_if = "Option::is_none")]
    pub total_memory_in_bytes: Option<i64>,
    #[doc = "The available memory."]
    #[serde(rename = "availableMemoryInBytes", default, skip_serializing_if = "Option::is_none")]
    pub available_memory_in_bytes: Option<i64>,
    #[doc = "The memory usage status."]
    #[serde(rename = "memoryUsageStatus", default, skip_serializing_if = "Option::is_none")]
    pub memory_usage_status: Option<String>,
    #[doc = "The total space."]
    #[serde(rename = "totalSpaceInBytes", default, skip_serializing_if = "Option::is_none")]
    pub total_space_in_bytes: Option<i64>,
    #[doc = "The available space."]
    #[serde(rename = "availableSpaceInBytes", default, skip_serializing_if = "Option::is_none")]
    pub available_space_in_bytes: Option<i64>,
    #[doc = "The space usage status."]
    #[serde(rename = "spaceUsageStatus", default, skip_serializing_if = "Option::is_none")]
    pub space_usage_status: Option<String>,
    #[doc = "The web load."]
    #[serde(rename = "webLoad", default, skip_serializing_if = "Option::is_none")]
    pub web_load: Option<String>,
    #[doc = "The web load status."]
    #[serde(rename = "webLoadStatus", default, skip_serializing_if = "Option::is_none")]
    pub web_load_status: Option<String>,
    #[doc = "The database server load."]
    #[serde(rename = "databaseServerLoad", default, skip_serializing_if = "Option::is_none")]
    pub database_server_load: Option<String>,
    #[doc = "The database server load status."]
    #[serde(rename = "databaseServerLoadStatus", default, skip_serializing_if = "Option::is_none")]
    pub database_server_load_status: Option<String>,
    #[doc = "The CS service status."]
    #[serde(rename = "csServiceStatus", default, skip_serializing_if = "Option::is_none")]
    pub cs_service_status: Option<String>,
    #[doc = "The IP address."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The agent Version."]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "The host name."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "The last heartbeat received from CS server."]
    #[serde(rename = "lastHeartbeat", with = "azure_core::date::rfc3339::option")]
    pub last_heartbeat: Option<time::OffsetDateTime>,
    #[doc = "Version status."]
    #[serde(rename = "versionStatus", default, skip_serializing_if = "Option::is_none")]
    pub version_status: Option<String>,
    #[doc = "CS SSL cert expiry date."]
    #[serde(rename = "sslCertExpiryDate", with = "azure_core::date::rfc3339::option")]
    pub ssl_cert_expiry_date: Option<time::OffsetDateTime>,
    #[doc = "CS SSL cert expiry date."]
    #[serde(rename = "sslCertExpiryRemainingDays", default, skip_serializing_if = "Option::is_none")]
    pub ssl_cert_expiry_remaining_days: Option<i32>,
    #[doc = "PS template version."]
    #[serde(rename = "psTemplateVersion", default, skip_serializing_if = "Option::is_none")]
    pub ps_template_version: Option<String>,
    #[doc = "Agent expiry date."]
    #[serde(rename = "agentExpiryDate", with = "azure_core::date::rfc3339::option")]
    pub agent_expiry_date: Option<time::OffsetDateTime>,
    #[doc = "Version related details."]
    #[serde(rename = "agentVersionDetails", default, skip_serializing_if = "Option::is_none")]
    pub agent_version_details: Option<VersionDetails>,
    #[doc = "The switch provider blocking error information."]
    #[serde(rename = "switchProviderBlockingErrorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub switch_provider_blocking_error_details: Vec<InMageFabricSwitchProviderBlockingErrorDetails>,
}
impl VMwareDetails {
    pub fn new(fabric_specific_details: FabricSpecificDetails) -> Self {
        Self {
            fabric_specific_details,
            process_servers: Vec::new(),
            master_target_servers: Vec::new(),
            run_as_accounts: Vec::new(),
            replication_pair_count: None,
            process_server_count: None,
            agent_count: None,
            protected_servers: None,
            system_load: None,
            system_load_status: None,
            cpu_load: None,
            cpu_load_status: None,
            total_memory_in_bytes: None,
            available_memory_in_bytes: None,
            memory_usage_status: None,
            total_space_in_bytes: None,
            available_space_in_bytes: None,
            space_usage_status: None,
            web_load: None,
            web_load_status: None,
            database_server_load: None,
            database_server_load_status: None,
            cs_service_status: None,
            ip_address: None,
            agent_version: None,
            host_name: None,
            last_heartbeat: None,
            version_status: None,
            ssl_cert_expiry_date: None,
            ssl_cert_expiry_remaining_days: None,
            ps_template_version: None,
            agent_expiry_date: None,
            agent_version_details: None,
            switch_provider_blocking_error_details: Vec::new(),
        }
    }
}
#[doc = "VMwareV2 fabric provider specific settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareV2FabricCreationInput {
    #[serde(flatten)]
    pub fabric_specific_creation_input: FabricSpecificCreationInput,
    #[doc = "The ARM Id of the VMware site."]
    #[serde(rename = "vmwareSiteId", default, skip_serializing_if = "Option::is_none")]
    pub vmware_site_id: Option<String>,
    #[doc = "The ARM Id of the physical site."]
    #[serde(rename = "physicalSiteId", default, skip_serializing_if = "Option::is_none")]
    pub physical_site_id: Option<String>,
    #[doc = "The ARM Id of the migration solution."]
    #[serde(rename = "migrationSolutionId")]
    pub migration_solution_id: String,
}
impl VMwareV2FabricCreationInput {
    pub fn new(fabric_specific_creation_input: FabricSpecificCreationInput, migration_solution_id: String) -> Self {
        Self {
            fabric_specific_creation_input,
            vmware_site_id: None,
            physical_site_id: None,
            migration_solution_id,
        }
    }
}
#[doc = "VMwareV2 fabric specific details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareV2FabricSpecificDetails {
    #[serde(flatten)]
    pub fabric_specific_details: FabricSpecificDetails,
    #[doc = "The ARM Id of the VMware site."]
    #[serde(rename = "vmwareSiteId", default, skip_serializing_if = "Option::is_none")]
    pub vmware_site_id: Option<String>,
    #[doc = "The ARM Id of the physical site."]
    #[serde(rename = "physicalSiteId", default, skip_serializing_if = "Option::is_none")]
    pub physical_site_id: Option<String>,
    #[doc = "The Migration solution ARM Id."]
    #[serde(rename = "migrationSolutionId", default, skip_serializing_if = "Option::is_none")]
    pub migration_solution_id: Option<String>,
    #[doc = "The service endpoint."]
    #[serde(rename = "serviceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub service_endpoint: Option<String>,
    #[doc = "The service resource Id."]
    #[serde(rename = "serviceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub service_resource_id: Option<String>,
    #[doc = "The service container Id."]
    #[serde(rename = "serviceContainerId", default, skip_serializing_if = "Option::is_none")]
    pub service_container_id: Option<String>,
    #[doc = "The list of process servers."]
    #[serde(rename = "processServers", default, skip_serializing_if = "Vec::is_empty")]
    pub process_servers: Vec<ProcessServerDetails>,
}
impl VMwareV2FabricSpecificDetails {
    pub fn new(fabric_specific_details: FabricSpecificDetails) -> Self {
        Self {
            fabric_specific_details,
            vmware_site_id: None,
            physical_site_id: None,
            migration_solution_id: None,
            service_endpoint: None,
            service_resource_id: None,
            service_container_id: None,
            process_servers: Vec::new(),
        }
    }
}
#[doc = "VMware provider specific settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VMwareVirtualMachineDetails {
    #[serde(flatten)]
    pub configuration_settings: ConfigurationSettings,
    #[doc = "The ID generated by the InMage agent after it gets installed on guest. This is the ID to be used during InMage CreateProtection."]
    #[serde(rename = "agentGeneratedId", default, skip_serializing_if = "Option::is_none")]
    pub agent_generated_id: Option<String>,
    #[doc = "The value indicating if InMage scout agent is installed on guest."]
    #[serde(rename = "agentInstalled", default, skip_serializing_if = "Option::is_none")]
    pub agent_installed: Option<String>,
    #[doc = "The OsType installed on VM."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "The agent version."]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "The IP address."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The value indicating whether VM is powered on."]
    #[serde(rename = "poweredOn", default, skip_serializing_if = "Option::is_none")]
    pub powered_on: Option<String>,
    #[doc = "The VCenter infrastructure Id."]
    #[serde(rename = "vCenterInfrastructureId", default, skip_serializing_if = "Option::is_none")]
    pub v_center_infrastructure_id: Option<String>,
    #[doc = "A value indicating the discovery type of the machine. Value can be vCenter or physical."]
    #[serde(rename = "discoveryType", default, skip_serializing_if = "Option::is_none")]
    pub discovery_type: Option<String>,
    #[doc = "The disk details."]
    #[serde(rename = "diskDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub disk_details: Vec<InMageDiskDetails>,
    #[doc = "The validation errors."]
    #[serde(rename = "validationErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub validation_errors: Vec<HealthError>,
}
impl VMwareVirtualMachineDetails {
    pub fn new(configuration_settings: ConfigurationSettings) -> Self {
        Self {
            configuration_settings,
            agent_generated_id: None,
            agent_installed: None,
            os_type: None,
            agent_version: None,
            ip_address: None,
            powered_on: None,
            v_center_infrastructure_id: None,
            discovery_type: None,
            disk_details: Vec::new(),
            validation_errors: Vec::new(),
        }
    }
}
#[doc = "Vault health details definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultHealthDetails {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "class to define the health summary of the Vault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VaultHealthProperties>,
}
impl VaultHealthDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "class to define the health summary of the Vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultHealthProperties {
    #[doc = "The list of errors on the vault."]
    #[serde(rename = "vaultErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub vault_errors: Vec<HealthError>,
    #[doc = "Base class to define the health summary of the resources contained under an Arm resource."]
    #[serde(rename = "protectedItemsHealth", default, skip_serializing_if = "Option::is_none")]
    pub protected_items_health: Option<ResourceHealthSummary>,
    #[doc = "Base class to define the health summary of the resources contained under an Arm resource."]
    #[serde(rename = "fabricsHealth", default, skip_serializing_if = "Option::is_none")]
    pub fabrics_health: Option<ResourceHealthSummary>,
    #[doc = "Base class to define the health summary of the resources contained under an Arm resource."]
    #[serde(rename = "containersHealth", default, skip_serializing_if = "Option::is_none")]
    pub containers_health: Option<ResourceHealthSummary>,
}
impl VaultHealthProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Vault setting."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultSetting {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Vault setting properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VaultSettingProperties>,
}
impl VaultSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Vault setting collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultSettingCollection {
    #[doc = "The list of vault setting."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VaultSetting>,
    #[doc = "The value of next link."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VaultSettingCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VaultSettingCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Input to create vault setting."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VaultSettingCreationInput {
    #[doc = "Input to create vault setting."]
    pub properties: VaultSettingCreationInputProperties,
}
impl VaultSettingCreationInput {
    pub fn new(properties: VaultSettingCreationInputProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Input to create vault setting."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultSettingCreationInputProperties {
    #[doc = "The migration solution Id."]
    #[serde(rename = "migrationSolutionId", default, skip_serializing_if = "Option::is_none")]
    pub migration_solution_id: Option<String>,
    #[doc = "VMware to Azure provider type."]
    #[serde(rename = "vmwareToAzureProviderType", default, skip_serializing_if = "Option::is_none")]
    pub vmware_to_azure_provider_type: Option<String>,
}
impl VaultSettingCreationInputProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Vault setting properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaultSettingProperties {
    #[doc = "The migration solution ARM Id."]
    #[serde(rename = "migrationSolutionId", default, skip_serializing_if = "Option::is_none")]
    pub migration_solution_id: Option<String>,
    #[doc = "VMware to Azure provider type."]
    #[serde(rename = "vmwareToAzureProviderType", default, skip_serializing_if = "Option::is_none")]
    pub vmware_to_azure_provider_type: Option<String>,
}
impl VaultSettingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Version related details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VersionDetails {
    #[doc = "The agent version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Version expiry date."]
    #[serde(rename = "expiryDate", with = "azure_core::date::rfc3339::option")]
    pub expiry_date: Option<time::OffsetDateTime>,
    #[doc = "A value indicating whether security update required."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<version_details::Status>,
}
impl VersionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod version_details {
    use super::*;
    #[doc = "A value indicating whether security update required."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Supported,
        NotSupported,
        Deprecated,
        UpdateRequired,
        SecurityUpdateRequired,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Status {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Status {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Status {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Supported => serializer.serialize_unit_variant("Status", 0u32, "Supported"),
                Self::NotSupported => serializer.serialize_unit_variant("Status", 1u32, "NotSupported"),
                Self::Deprecated => serializer.serialize_unit_variant("Status", 2u32, "Deprecated"),
                Self::UpdateRequired => serializer.serialize_unit_variant("Status", 3u32, "UpdateRequired"),
                Self::SecurityUpdateRequired => serializer.serialize_unit_variant("Status", 4u32, "SecurityUpdateRequired"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "This class represents the virtual machine task details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineTaskDetails {
    #[serde(flatten)]
    pub job_task_details: JobTaskDetails,
    #[doc = "The skipped reason."]
    #[serde(rename = "skippedReason", default, skip_serializing_if = "Option::is_none")]
    pub skipped_reason: Option<String>,
    #[doc = "The skipped reason string."]
    #[serde(rename = "skippedReasonString", default, skip_serializing_if = "Option::is_none")]
    pub skipped_reason_string: Option<String>,
}
impl VirtualMachineTaskDetails {
    pub fn new(job_task_details: JobTaskDetails) -> Self {
        Self {
            job_task_details,
            skipped_reason: None,
            skipped_reason_string: None,
        }
    }
}
#[doc = "This class represents the vm NicUpdates task details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmNicUpdatesTaskDetails {
    #[serde(flatten)]
    pub task_type_details: TaskTypeDetails,
    #[doc = "Virtual machine Id."]
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[doc = "Nic Id."]
    #[serde(rename = "nicId", default, skip_serializing_if = "Option::is_none")]
    pub nic_id: Option<String>,
    #[doc = "Name of the Nic."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl VmNicUpdatesTaskDetails {
    pub fn new(task_type_details: TaskTypeDetails) -> Self {
        Self {
            task_type_details,
            vm_id: None,
            nic_id: None,
            name: None,
        }
    }
}
#[doc = "VMM fabric specific details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmmDetails {
    #[serde(flatten)]
    pub fabric_specific_details: FabricSpecificDetails,
}
impl VmmDetails {
    pub fn new(fabric_specific_details: FabricSpecificDetails) -> Self {
        Self { fabric_specific_details }
    }
}
#[doc = "Create network mappings input properties/behavior specific to Vmm to Azure Network mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmmToAzureCreateNetworkMappingInput {
    #[serde(flatten)]
    pub fabric_specific_create_network_mapping_input: FabricSpecificCreateNetworkMappingInput,
}
impl VmmToAzureCreateNetworkMappingInput {
    pub fn new(fabric_specific_create_network_mapping_input: FabricSpecificCreateNetworkMappingInput) -> Self {
        Self {
            fabric_specific_create_network_mapping_input,
        }
    }
}
#[doc = "E2A Network Mapping fabric specific settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmmToAzureNetworkMappingSettings {
    #[serde(flatten)]
    pub network_mapping_fabric_specific_settings: NetworkMappingFabricSpecificSettings,
}
impl VmmToAzureNetworkMappingSettings {
    pub fn new(network_mapping_fabric_specific_settings: NetworkMappingFabricSpecificSettings) -> Self {
        Self {
            network_mapping_fabric_specific_settings,
        }
    }
}
#[doc = "Update network mappings input properties/behavior specific to vmm to azure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmmToAzureUpdateNetworkMappingInput {
    #[serde(flatten)]
    pub fabric_specific_update_network_mapping_input: FabricSpecificUpdateNetworkMappingInput,
}
impl VmmToAzureUpdateNetworkMappingInput {
    pub fn new(fabric_specific_update_network_mapping_input: FabricSpecificUpdateNetworkMappingInput) -> Self {
        Self {
            fabric_specific_update_network_mapping_input,
        }
    }
}
#[doc = "Create network mappings input properties/behavior specific to vmm to vmm Network mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmmToVmmCreateNetworkMappingInput {
    #[serde(flatten)]
    pub fabric_specific_create_network_mapping_input: FabricSpecificCreateNetworkMappingInput,
}
impl VmmToVmmCreateNetworkMappingInput {
    pub fn new(fabric_specific_create_network_mapping_input: FabricSpecificCreateNetworkMappingInput) -> Self {
        Self {
            fabric_specific_create_network_mapping_input,
        }
    }
}
#[doc = "E2E Network Mapping fabric specific settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmmToVmmNetworkMappingSettings {
    #[serde(flatten)]
    pub network_mapping_fabric_specific_settings: NetworkMappingFabricSpecificSettings,
}
impl VmmToVmmNetworkMappingSettings {
    pub fn new(network_mapping_fabric_specific_settings: NetworkMappingFabricSpecificSettings) -> Self {
        Self {
            network_mapping_fabric_specific_settings,
        }
    }
}
#[doc = "Update network mappings input properties/behavior specific to vmm to vmm."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmmToVmmUpdateNetworkMappingInput {
    #[serde(flatten)]
    pub fabric_specific_update_network_mapping_input: FabricSpecificUpdateNetworkMappingInput,
}
impl VmmToVmmUpdateNetworkMappingInput {
    pub fn new(fabric_specific_update_network_mapping_input: FabricSpecificUpdateNetworkMappingInput) -> Self {
        Self {
            fabric_specific_update_network_mapping_input,
        }
    }
}
#[doc = "VMM fabric provider specific VM settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmmVirtualMachineDetails {
    #[serde(flatten)]
    pub hyper_v_virtual_machine_details: HyperVVirtualMachineDetails,
}
impl VmmVirtualMachineDetails {
    pub fn new(hyper_v_virtual_machine_details: HyperVVirtualMachineDetails) -> Self {
        Self {
            hyper_v_virtual_machine_details,
        }
    }
}
#[doc = "VMware Cbt specific policy details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmwareCbtPolicyDetails {
    #[serde(flatten)]
    pub policy_provider_specific_details: PolicyProviderSpecificDetails,
    #[doc = "The duration in minutes until which the recovery points need to be stored."]
    #[serde(rename = "recoveryPointHistoryInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub recovery_point_history_in_minutes: Option<i32>,
    #[doc = "The app consistent snapshot frequency in minutes."]
    #[serde(rename = "appConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub app_consistent_frequency_in_minutes: Option<i32>,
    #[doc = "The crash consistent snapshot frequency in minutes."]
    #[serde(rename = "crashConsistentFrequencyInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub crash_consistent_frequency_in_minutes: Option<i32>,
}
impl VmwareCbtPolicyDetails {
    pub fn new(policy_provider_specific_details: PolicyProviderSpecificDetails) -> Self {
        Self {
            policy_provider_specific_details,
            recovery_point_history_in_minutes: None,
            app_consistent_frequency_in_minutes: None,
            crash_consistent_frequency_in_minutes: None,
        }
    }
}
