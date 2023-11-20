#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdministrativeCredentials {
    #[doc = "The password of the administrator of the device used during initialization."]
    pub password: String,
    #[doc = "The username of the administrator of the device used during initialization."]
    pub username: String,
}
impl AdministrativeCredentials {
    pub fn new(password: String, username: String) -> Self {
        Self { password, username }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BareMetalMachine {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
    pub properties: BareMetalMachineProperties,
}
impl BareMetalMachine {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocation, properties: BareMetalMachineProperties) -> Self {
        Self {
            tracked_resource,
            extended_location,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BareMetalMachineCommandSpecification {
    #[doc = "The list of string arguments that will be passed to the script in order as separate arguments."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub arguments: Vec<String>,
    #[doc = "The command to execute against the bare metal machine."]
    pub command: String,
}
impl BareMetalMachineCommandSpecification {
    pub fn new(command: String) -> Self {
        Self {
            arguments: Vec::new(),
            command,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BareMetalMachineConfigurationData {
    #[doc = "The connection string for the baseboard management controller including IP address and protocol."]
    #[serde(rename = "bmcConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub bmc_connection_string: Option<String>,
    #[serde(rename = "bmcCredentials")]
    pub bmc_credentials: AdministrativeCredentials,
    #[doc = "The MAC address of the BMC for this machine."]
    #[serde(rename = "bmcMacAddress")]
    pub bmc_mac_address: String,
    #[doc = "The MAC address associated with the PXE NIC card."]
    #[serde(rename = "bootMacAddress")]
    pub boot_mac_address: String,
    #[doc = "The free-form additional information about the machine, e.g. an asset tag."]
    #[serde(rename = "machineDetails", default, skip_serializing_if = "Option::is_none")]
    pub machine_details: Option<String>,
    #[doc = "The user-provided name for the bare metal machine created from this specification.\nIf not provided, the machine name will be generated programmatically."]
    #[serde(rename = "machineName", default, skip_serializing_if = "Option::is_none")]
    pub machine_name: Option<String>,
    #[doc = "The slot the physical machine is in the rack based on the BOM configuration."]
    #[serde(rename = "rackSlot")]
    pub rack_slot: i64,
    #[doc = "The serial number of the machine. Hardware suppliers may use an alternate value. For example, service tag."]
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
}
impl BareMetalMachineConfigurationData {
    pub fn new(
        bmc_credentials: AdministrativeCredentials,
        bmc_mac_address: String,
        boot_mac_address: String,
        rack_slot: i64,
        serial_number: String,
    ) -> Self {
        Self {
            bmc_connection_string: None,
            bmc_credentials,
            bmc_mac_address,
            boot_mac_address,
            machine_details: None,
            machine_name: None,
            rack_slot,
            serial_number,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BareMetalMachineCordonParameters {
    #[doc = "The indicator of whether to evacuate the node workload when the bare metal machine is cordoned."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evacuate: Option<bare_metal_machine_cordon_parameters::Evacuate>,
}
impl BareMetalMachineCordonParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod bare_metal_machine_cordon_parameters {
    use super::*;
    #[doc = "The indicator of whether to evacuate the node workload when the bare metal machine is cordoned."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Evacuate")]
    pub enum Evacuate {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Evacuate {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Evacuate {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Evacuate {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("Evacuate", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("Evacuate", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Evacuate {
        fn default() -> Self {
            Self::False
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BareMetalMachineKeySet {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
    pub properties: BareMetalMachineKeySetProperties,
}
impl BareMetalMachineKeySet {
    pub fn new(
        tracked_resource: TrackedResource,
        extended_location: ExtendedLocation,
        properties: BareMetalMachineKeySetProperties,
    ) -> Self {
        Self {
            tracked_resource,
            extended_location,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BareMetalMachineKeySetList {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of bare metal machine key sets."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BareMetalMachineKeySet>,
}
impl azure_core::Continuable for BareMetalMachineKeySetList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BareMetalMachineKeySetList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BareMetalMachineKeySetPatchParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BareMetalMachineKeySetPatchProperties>,
    #[doc = "The Azure resource tags that will replace the existing ones."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl BareMetalMachineKeySetPatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BareMetalMachineKeySetPatchProperties {
    #[doc = "The date and time after which the users in this key set will be removed from the bare metal machines."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub expiration: Option<time::OffsetDateTime>,
    #[doc = "The list of IP addresses of jump hosts with management network access from which a login will be allowed for the users."]
    #[serde(
        rename = "jumpHostsAllowed",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub jump_hosts_allowed: Vec<String>,
    #[doc = "The unique list of permitted users."]
    #[serde(
        rename = "userList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub user_list: Vec<KeySetUser>,
}
impl BareMetalMachineKeySetPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BareMetalMachineKeySetProperties {
    #[doc = "The object ID of Azure Active Directory group that all users in the list must be in for access to be granted. Users that are not in the group will not have access."]
    #[serde(rename = "azureGroupId")]
    pub azure_group_id: String,
    #[doc = "The more detailed status of the key set."]
    #[serde(rename = "detailedStatus", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<bare_metal_machine_key_set_properties::DetailedStatus>,
    #[doc = "The descriptive message about the current detailed status."]
    #[serde(rename = "detailedStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status_message: Option<String>,
    #[doc = "The date and time after which the users in this key set will be removed from the bare metal machines."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub expiration: time::OffsetDateTime,
    #[doc = "The list of IP addresses of jump hosts with management network access from which a login will be allowed for the users."]
    #[serde(rename = "jumpHostsAllowed")]
    pub jump_hosts_allowed: Vec<String>,
    #[doc = "The last time this key set was validated."]
    #[serde(rename = "lastValidation", default, with = "azure_core::date::rfc3339::option")]
    pub last_validation: Option<time::OffsetDateTime>,
    #[doc = "The name of the group that users will be assigned to on the operating system of the machines."]
    #[serde(rename = "osGroupName", default, skip_serializing_if = "Option::is_none")]
    pub os_group_name: Option<String>,
    #[doc = "The access level allowed for the users in this key set."]
    #[serde(rename = "privilegeLevel")]
    pub privilege_level: bare_metal_machine_key_set_properties::PrivilegeLevel,
    #[doc = "The provisioning state of the bare metal machine key set."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<bare_metal_machine_key_set_properties::ProvisioningState>,
    #[doc = "The unique list of permitted users."]
    #[serde(rename = "userList")]
    pub user_list: Vec<KeySetUser>,
    #[doc = "The status evaluation of each user."]
    #[serde(
        rename = "userListStatus",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub user_list_status: Vec<KeySetUserStatus>,
}
impl BareMetalMachineKeySetProperties {
    pub fn new(
        azure_group_id: String,
        expiration: time::OffsetDateTime,
        jump_hosts_allowed: Vec<String>,
        privilege_level: bare_metal_machine_key_set_properties::PrivilegeLevel,
        user_list: Vec<KeySetUser>,
    ) -> Self {
        Self {
            azure_group_id,
            detailed_status: None,
            detailed_status_message: None,
            expiration,
            jump_hosts_allowed,
            last_validation: None,
            os_group_name: None,
            privilege_level,
            provisioning_state: None,
            user_list,
            user_list_status: Vec::new(),
        }
    }
}
pub mod bare_metal_machine_key_set_properties {
    use super::*;
    #[doc = "The more detailed status of the key set."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DetailedStatus")]
    pub enum DetailedStatus {
        AllActive,
        SomeInvalid,
        AllInvalid,
        Validating,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DetailedStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DetailedStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DetailedStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AllActive => serializer.serialize_unit_variant("DetailedStatus", 0u32, "AllActive"),
                Self::SomeInvalid => serializer.serialize_unit_variant("DetailedStatus", 1u32, "SomeInvalid"),
                Self::AllInvalid => serializer.serialize_unit_variant("DetailedStatus", 2u32, "AllInvalid"),
                Self::Validating => serializer.serialize_unit_variant("DetailedStatus", 3u32, "Validating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The access level allowed for the users in this key set."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrivilegeLevel")]
    pub enum PrivilegeLevel {
        Standard,
        Superuser,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrivilegeLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrivilegeLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrivilegeLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Standard => serializer.serialize_unit_variant("PrivilegeLevel", 0u32, "Standard"),
                Self::Superuser => serializer.serialize_unit_variant("PrivilegeLevel", 1u32, "Superuser"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state of the bare metal machine key set."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Accepted,
        Provisioning,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Accepted"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BareMetalMachineList {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of bare metal machines."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BareMetalMachine>,
}
impl azure_core::Continuable for BareMetalMachineList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BareMetalMachineList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BareMetalMachinePatchParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BareMetalMachinePatchProperties>,
    #[doc = "The Azure resource tags that will replace the existing ones."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl BareMetalMachinePatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BareMetalMachinePatchProperties {
    #[doc = "The details provided by the customer during the creation of rack manifests\nthat allows for custom data to be associated with this machine."]
    #[serde(rename = "machineDetails", default, skip_serializing_if = "Option::is_none")]
    pub machine_details: Option<String>,
}
impl BareMetalMachinePatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BareMetalMachinePowerOffParameters {
    #[doc = "The indicator of whether to skip the graceful OS shutdown and power off the bare metal machine immediately."]
    #[serde(rename = "skipShutdown", default, skip_serializing_if = "Option::is_none")]
    pub skip_shutdown: Option<bare_metal_machine_power_off_parameters::SkipShutdown>,
}
impl BareMetalMachinePowerOffParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod bare_metal_machine_power_off_parameters {
    use super::*;
    #[doc = "The indicator of whether to skip the graceful OS shutdown and power off the bare metal machine immediately."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SkipShutdown")]
    pub enum SkipShutdown {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SkipShutdown {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SkipShutdown {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SkipShutdown {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("SkipShutdown", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("SkipShutdown", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for SkipShutdown {
        fn default() -> Self {
            Self::False
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BareMetalMachineProperties {
    #[doc = "The connection string for the baseboard management controller including IP address and protocol."]
    #[serde(rename = "bmcConnectionString")]
    pub bmc_connection_string: String,
    #[serde(rename = "bmcCredentials")]
    pub bmc_credentials: AdministrativeCredentials,
    #[doc = "The MAC address of the BMC device."]
    #[serde(rename = "bmcMacAddress")]
    pub bmc_mac_address: String,
    #[doc = "The MAC address of a NIC connected to the PXE network."]
    #[serde(rename = "bootMacAddress")]
    pub boot_mac_address: String,
    #[doc = "The resource ID of the cluster this bare metal machine is associated with."]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc = "The cordon status of the bare metal machine."]
    #[serde(rename = "cordonStatus", default, skip_serializing_if = "Option::is_none")]
    pub cordon_status: Option<bare_metal_machine_properties::CordonStatus>,
    #[doc = "The more detailed status of the bare metal machine."]
    #[serde(rename = "detailedStatus", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<bare_metal_machine_properties::DetailedStatus>,
    #[doc = "The descriptive message about the current detailed status."]
    #[serde(rename = "detailedStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status_message: Option<String>,
    #[serde(rename = "hardwareInventory", default, skip_serializing_if = "Option::is_none")]
    pub hardware_inventory: Option<HardwareInventory>,
    #[serde(rename = "hardwareValidationStatus", default, skip_serializing_if = "Option::is_none")]
    pub hardware_validation_status: Option<HardwareValidationStatus>,
    #[doc = "The list of the resource IDs for the HybridAksClusters that have nodes hosted on this bare metal machine."]
    #[serde(
        rename = "hybridAksClustersAssociatedIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hybrid_aks_clusters_associated_ids: Vec<String>,
    #[doc = "The name of this machine represented by the host object in the Cluster's Kubernetes control plane."]
    #[serde(rename = "kubernetesNodeName", default, skip_serializing_if = "Option::is_none")]
    pub kubernetes_node_name: Option<String>,
    #[doc = "The version of Kubernetes running on this machine."]
    #[serde(rename = "kubernetesVersion", default, skip_serializing_if = "Option::is_none")]
    pub kubernetes_version: Option<String>,
    #[doc = "The custom details provided by the customer."]
    #[serde(rename = "machineDetails")]
    pub machine_details: String,
    #[doc = "The OS-level hostname assigned to this machine."]
    #[serde(rename = "machineName")]
    pub machine_name: String,
    #[doc = "The unique internal identifier of the bare metal machine SKU."]
    #[serde(rename = "machineSkuId")]
    pub machine_sku_id: String,
    #[doc = "The IPv4 address that is assigned to the bare metal machine during the cluster deployment."]
    #[serde(rename = "oamIpv4Address", default, skip_serializing_if = "Option::is_none")]
    pub oam_ipv4_address: Option<String>,
    #[doc = "The IPv6 address that is assigned to the bare metal machine during the cluster deployment."]
    #[serde(rename = "oamIpv6Address", default, skip_serializing_if = "Option::is_none")]
    pub oam_ipv6_address: Option<String>,
    #[doc = "The image that is currently provisioned to the OS disk."]
    #[serde(rename = "osImage", default, skip_serializing_if = "Option::is_none")]
    pub os_image: Option<String>,
    #[doc = "The power state derived from the baseboard management controller."]
    #[serde(rename = "powerState", default, skip_serializing_if = "Option::is_none")]
    pub power_state: Option<bare_metal_machine_properties::PowerState>,
    #[doc = "The provisioning state of the bare metal machine."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<bare_metal_machine_properties::ProvisioningState>,
    #[doc = "The resource ID of the rack where this bare metal machine resides."]
    #[serde(rename = "rackId")]
    pub rack_id: String,
    #[doc = "The rack slot in which this bare metal machine is located, ordered from the bottom up i.e. the lowest slot is 1."]
    #[serde(rename = "rackSlot")]
    pub rack_slot: i64,
    #[doc = "The indicator of whether the bare metal machine is ready to receive workloads."]
    #[serde(rename = "readyState", default, skip_serializing_if = "Option::is_none")]
    pub ready_state: Option<bare_metal_machine_properties::ReadyState>,
    #[doc = "The serial number of the bare metal machine."]
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    #[doc = "The discovered value of the machine's service tag."]
    #[serde(rename = "serviceTag", default, skip_serializing_if = "Option::is_none")]
    pub service_tag: Option<String>,
    #[doc = "The list of the resource IDs for the VirtualMachines that are hosted on this bare metal machine."]
    #[serde(
        rename = "virtualMachinesAssociatedIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub virtual_machines_associated_ids: Vec<String>,
}
impl BareMetalMachineProperties {
    pub fn new(
        bmc_connection_string: String,
        bmc_credentials: AdministrativeCredentials,
        bmc_mac_address: String,
        boot_mac_address: String,
        machine_details: String,
        machine_name: String,
        machine_sku_id: String,
        rack_id: String,
        rack_slot: i64,
        serial_number: String,
    ) -> Self {
        Self {
            bmc_connection_string,
            bmc_credentials,
            bmc_mac_address,
            boot_mac_address,
            cluster_id: None,
            cordon_status: None,
            detailed_status: None,
            detailed_status_message: None,
            hardware_inventory: None,
            hardware_validation_status: None,
            hybrid_aks_clusters_associated_ids: Vec::new(),
            kubernetes_node_name: None,
            kubernetes_version: None,
            machine_details,
            machine_name,
            machine_sku_id,
            oam_ipv4_address: None,
            oam_ipv6_address: None,
            os_image: None,
            power_state: None,
            provisioning_state: None,
            rack_id,
            rack_slot,
            ready_state: None,
            serial_number,
            service_tag: None,
            virtual_machines_associated_ids: Vec::new(),
        }
    }
}
pub mod bare_metal_machine_properties {
    use super::*;
    #[doc = "The cordon status of the bare metal machine."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CordonStatus")]
    pub enum CordonStatus {
        Cordoned,
        Uncordoned,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CordonStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CordonStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CordonStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Cordoned => serializer.serialize_unit_variant("CordonStatus", 0u32, "Cordoned"),
                Self::Uncordoned => serializer.serialize_unit_variant("CordonStatus", 1u32, "Uncordoned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The more detailed status of the bare metal machine."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DetailedStatus")]
    pub enum DetailedStatus {
        Preparing,
        Error,
        Available,
        Provisioning,
        Provisioned,
        Deprovisioning,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DetailedStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DetailedStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DetailedStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Preparing => serializer.serialize_unit_variant("DetailedStatus", 0u32, "Preparing"),
                Self::Error => serializer.serialize_unit_variant("DetailedStatus", 1u32, "Error"),
                Self::Available => serializer.serialize_unit_variant("DetailedStatus", 2u32, "Available"),
                Self::Provisioning => serializer.serialize_unit_variant("DetailedStatus", 3u32, "Provisioning"),
                Self::Provisioned => serializer.serialize_unit_variant("DetailedStatus", 4u32, "Provisioned"),
                Self::Deprovisioning => serializer.serialize_unit_variant("DetailedStatus", 5u32, "Deprovisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The power state derived from the baseboard management controller."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PowerState")]
    pub enum PowerState {
        On,
        Off,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PowerState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PowerState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PowerState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::On => serializer.serialize_unit_variant("PowerState", 0u32, "On"),
                Self::Off => serializer.serialize_unit_variant("PowerState", 1u32, "Off"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state of the bare metal machine."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Provisioning,
        Accepted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Provisioning"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Accepted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The indicator of whether the bare metal machine is ready to receive workloads."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReadyState")]
    pub enum ReadyState {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReadyState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReadyState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReadyState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("ReadyState", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("ReadyState", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BareMetalMachineReplaceParameters {
    #[serde(rename = "bmcCredentials", default, skip_serializing_if = "Option::is_none")]
    pub bmc_credentials: Option<AdministrativeCredentials>,
    #[doc = "The MAC address of the BMC device."]
    #[serde(rename = "bmcMacAddress", default, skip_serializing_if = "Option::is_none")]
    pub bmc_mac_address: Option<String>,
    #[doc = "The MAC address of a NIC connected to the PXE network."]
    #[serde(rename = "bootMacAddress", default, skip_serializing_if = "Option::is_none")]
    pub boot_mac_address: Option<String>,
    #[doc = "The OS-level hostname assigned to this machine."]
    #[serde(rename = "machineName", default, skip_serializing_if = "Option::is_none")]
    pub machine_name: Option<String>,
    #[doc = "The serial number of the bare metal machine."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
}
impl BareMetalMachineReplaceParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BareMetalMachineRunCommandParameters {
    #[doc = "The list of string arguments that will be passed to the script in order as separate arguments."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub arguments: Vec<String>,
    #[doc = "The maximum time the script is allowed to run.\nIf the execution time exceeds the maximum, the script will be stopped, any output produced until then will be captured, and the exit code matching a timeout will be returned (252)."]
    #[serde(rename = "limitTimeSeconds")]
    pub limit_time_seconds: i64,
    #[doc = "The base64 encoded script to execute on the bare metal machine."]
    pub script: String,
}
impl BareMetalMachineRunCommandParameters {
    pub fn new(limit_time_seconds: i64, script: String) -> Self {
        Self {
            arguments: Vec::new(),
            limit_time_seconds,
            script,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BareMetalMachineRunDataExtractsParameters {
    #[doc = "The list of curated data extraction commands to be executed directly against the target machine."]
    pub commands: Vec<BareMetalMachineCommandSpecification>,
    #[doc = "The maximum time the commands are allowed to run.\nIf the execution time exceeds the maximum, the script will be stopped, any output produced until then will be captured, and the exit code matching a timeout will be returned (252)."]
    #[serde(rename = "limitTimeSeconds")]
    pub limit_time_seconds: i64,
}
impl BareMetalMachineRunDataExtractsParameters {
    pub fn new(commands: Vec<BareMetalMachineCommandSpecification>, limit_time_seconds: i64) -> Self {
        Self {
            commands,
            limit_time_seconds,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BareMetalMachineRunReadCommandsParameters {
    #[doc = "The list of read-only commands to be executed directly against the target machine."]
    pub commands: Vec<BareMetalMachineCommandSpecification>,
    #[doc = "The maximum time the commands are allowed to run.\nIf the execution time exceeds the maximum, the script will be stopped, any output produced until then will be captured, and the exit code matching a timeout will be returned (252)."]
    #[serde(rename = "limitTimeSeconds")]
    pub limit_time_seconds: i64,
}
impl BareMetalMachineRunReadCommandsParameters {
    pub fn new(commands: Vec<BareMetalMachineCommandSpecification>, limit_time_seconds: i64) -> Self {
        Self {
            commands,
            limit_time_seconds,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BareMetalMachineValidateHardwareParameters {
    #[doc = "The category of hardware validation to perform."]
    #[serde(rename = "validationCategory")]
    pub validation_category: bare_metal_machine_validate_hardware_parameters::ValidationCategory,
}
impl BareMetalMachineValidateHardwareParameters {
    pub fn new(validation_category: bare_metal_machine_validate_hardware_parameters::ValidationCategory) -> Self {
        Self { validation_category }
    }
}
pub mod bare_metal_machine_validate_hardware_parameters {
    use super::*;
    #[doc = "The category of hardware validation to perform."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ValidationCategory")]
    pub enum ValidationCategory {
        BasicValidation,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ValidationCategory {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ValidationCategory {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ValidationCategory {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BasicValidation => serializer.serialize_unit_variant("ValidationCategory", 0u32, "BasicValidation"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BgpPeer {
    #[doc = "The ASN (Autonomous System Number) of the BGP peer."]
    #[serde(rename = "asNumber")]
    pub as_number: i64,
    #[doc = "The password for this peering neighbor. It defaults to no password if not specified."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "The IPv4 or IPv6 address to peer with the associated CNI Network. The IP version type will drive a peering with the same version type from the Default CNI Network. For example, IPv4 to IPv4 or IPv6 to IPv6."]
    #[serde(rename = "peerIp")]
    pub peer_ip: String,
}
impl BgpPeer {
    pub fn new(as_number: i64, peer_ip: String) -> Self {
        Self {
            as_number,
            password: None,
            peer_ip,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BmcKeySet {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
    pub properties: BmcKeySetProperties,
}
impl BmcKeySet {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocation, properties: BmcKeySetProperties) -> Self {
        Self {
            tracked_resource,
            extended_location,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BmcKeySetList {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of baseboard management controller key sets."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<BmcKeySet>,
}
impl azure_core::Continuable for BmcKeySetList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BmcKeySetList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BmcKeySetPatchParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BmcKeySetPatchProperties>,
    #[doc = "The Azure resource tags that will replace the existing ones."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl BmcKeySetPatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BmcKeySetPatchProperties {
    #[doc = "The date and time after which the users in this key set will be removed from the baseboard management controllers."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub expiration: Option<time::OffsetDateTime>,
    #[doc = "The unique list of permitted users."]
    #[serde(
        rename = "userList",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub user_list: Vec<KeySetUser>,
}
impl BmcKeySetPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BmcKeySetProperties {
    #[doc = "The object ID of Azure Active Directory group that all users in the list must be in for access to be granted. Users that are not in the group will not have access."]
    #[serde(rename = "azureGroupId")]
    pub azure_group_id: String,
    #[doc = "The more detailed status of the key set."]
    #[serde(rename = "detailedStatus", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<bmc_key_set_properties::DetailedStatus>,
    #[doc = "The descriptive message about the current detailed status."]
    #[serde(rename = "detailedStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status_message: Option<String>,
    #[doc = "The date and time after which the users in this key set will be removed from the baseboard management controllers."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub expiration: time::OffsetDateTime,
    #[doc = "The last time this key set was validated."]
    #[serde(rename = "lastValidation", default, with = "azure_core::date::rfc3339::option")]
    pub last_validation: Option<time::OffsetDateTime>,
    #[doc = "The access level allowed for the users in this key set."]
    #[serde(rename = "privilegeLevel")]
    pub privilege_level: bmc_key_set_properties::PrivilegeLevel,
    #[doc = "The provisioning state of the baseboard management controller key set."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<bmc_key_set_properties::ProvisioningState>,
    #[doc = "The unique list of permitted users."]
    #[serde(rename = "userList")]
    pub user_list: Vec<KeySetUser>,
    #[doc = "The status evaluation of each user."]
    #[serde(
        rename = "userListStatus",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub user_list_status: Vec<KeySetUserStatus>,
}
impl BmcKeySetProperties {
    pub fn new(
        azure_group_id: String,
        expiration: time::OffsetDateTime,
        privilege_level: bmc_key_set_properties::PrivilegeLevel,
        user_list: Vec<KeySetUser>,
    ) -> Self {
        Self {
            azure_group_id,
            detailed_status: None,
            detailed_status_message: None,
            expiration,
            last_validation: None,
            privilege_level,
            provisioning_state: None,
            user_list,
            user_list_status: Vec::new(),
        }
    }
}
pub mod bmc_key_set_properties {
    use super::*;
    #[doc = "The more detailed status of the key set."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DetailedStatus")]
    pub enum DetailedStatus {
        AllActive,
        SomeInvalid,
        AllInvalid,
        Validating,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DetailedStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DetailedStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DetailedStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AllActive => serializer.serialize_unit_variant("DetailedStatus", 0u32, "AllActive"),
                Self::SomeInvalid => serializer.serialize_unit_variant("DetailedStatus", 1u32, "SomeInvalid"),
                Self::AllInvalid => serializer.serialize_unit_variant("DetailedStatus", 2u32, "AllInvalid"),
                Self::Validating => serializer.serialize_unit_variant("DetailedStatus", 3u32, "Validating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The access level allowed for the users in this key set."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrivilegeLevel")]
    pub enum PrivilegeLevel {
        ReadOnly,
        Administrator,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrivilegeLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrivilegeLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrivilegeLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ReadOnly => serializer.serialize_unit_variant("PrivilegeLevel", 0u32, "ReadOnly"),
                Self::Administrator => serializer.serialize_unit_variant("PrivilegeLevel", 1u32, "Administrator"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state of the baseboard management controller key set."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Accepted,
        Provisioning,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Accepted"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Upon creation, the additional services that are provided by the platform will be allocated and\nrepresented in the status of this resource. All resources associated with this cloud services network will be part\nof the same layer 2 (L2) isolation domain. At least one service network must be created but may be reused across many\nvirtual machines and/or Hybrid AKS clusters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudServicesNetwork {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CloudServicesNetworkProperties>,
}
impl CloudServicesNetwork {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocation) -> Self {
        Self {
            tracked_resource,
            extended_location,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudServicesNetworkList {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of cloud services networks."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CloudServicesNetwork>,
}
impl azure_core::Continuable for CloudServicesNetworkList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CloudServicesNetworkList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudServicesNetworkPatchParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CloudServicesNetworkPatchProperties>,
    #[doc = "The Azure resource tags that will replace the existing ones."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl CloudServicesNetworkPatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudServicesNetworkPatchProperties {
    #[doc = "The list of egress endpoints. This allows for connection from a Hybrid AKS cluster to the specified endpoint."]
    #[serde(
        rename = "additionalEgressEndpoints",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_egress_endpoints: Vec<EgressEndpoint>,
    #[doc = "The indicator of whether the platform default endpoints are allowed for the egress traffic."]
    #[serde(rename = "enableDefaultEgressEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub enable_default_egress_endpoints: Option<cloud_services_network_patch_properties::EnableDefaultEgressEndpoints>,
}
impl CloudServicesNetworkPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cloud_services_network_patch_properties {
    use super::*;
    #[doc = "The indicator of whether the platform default endpoints are allowed for the egress traffic."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EnableDefaultEgressEndpoints")]
    pub enum EnableDefaultEgressEndpoints {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EnableDefaultEgressEndpoints {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EnableDefaultEgressEndpoints {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EnableDefaultEgressEndpoints {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("EnableDefaultEgressEndpoints", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("EnableDefaultEgressEndpoints", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for EnableDefaultEgressEndpoints {
        fn default() -> Self {
            Self::True
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudServicesNetworkProperties {
    #[doc = "The list of egress endpoints. This allows for connection from a Hybrid AKS cluster to the specified endpoint."]
    #[serde(
        rename = "additionalEgressEndpoints",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_egress_endpoints: Vec<EgressEndpoint>,
    #[doc = "The resource ID of the Network Cloud cluster this cloud services network is associated with."]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc = "The more detailed status of the cloud services network."]
    #[serde(rename = "detailedStatus", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<cloud_services_network_properties::DetailedStatus>,
    #[doc = "The descriptive message about the current detailed status."]
    #[serde(rename = "detailedStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status_message: Option<String>,
    #[doc = "The indicator of whether the platform default endpoints are allowed for the egress traffic."]
    #[serde(rename = "enableDefaultEgressEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub enable_default_egress_endpoints: Option<cloud_services_network_properties::EnableDefaultEgressEndpoints>,
    #[doc = "The full list of additional and default egress endpoints that are currently enabled."]
    #[serde(
        rename = "enabledEgressEndpoints",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enabled_egress_endpoints: Vec<EgressEndpoint>,
    #[doc = "The list of Hybrid AKS cluster resource IDs that are associated with this cloud services network."]
    #[serde(
        rename = "hybridAksClustersAssociatedIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hybrid_aks_clusters_associated_ids: Vec<String>,
    #[doc = "The name of the interface that will be present in the virtual machine to represent this network."]
    #[serde(rename = "interfaceName", default, skip_serializing_if = "Option::is_none")]
    pub interface_name: Option<String>,
    #[doc = "The provisioning state of the cloud services network."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<cloud_services_network_properties::ProvisioningState>,
    #[doc = "The list of virtual machine resource IDs, excluding any Hybrid AKS virtual machines, that are currently using this cloud services network."]
    #[serde(
        rename = "virtualMachinesAssociatedIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub virtual_machines_associated_ids: Vec<String>,
}
impl CloudServicesNetworkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cloud_services_network_properties {
    use super::*;
    #[doc = "The more detailed status of the cloud services network."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DetailedStatus")]
    pub enum DetailedStatus {
        Error,
        Available,
        Provisioning,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DetailedStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DetailedStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DetailedStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Error => serializer.serialize_unit_variant("DetailedStatus", 0u32, "Error"),
                Self::Available => serializer.serialize_unit_variant("DetailedStatus", 1u32, "Available"),
                Self::Provisioning => serializer.serialize_unit_variant("DetailedStatus", 2u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The indicator of whether the platform default endpoints are allowed for the egress traffic."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EnableDefaultEgressEndpoints")]
    pub enum EnableDefaultEgressEndpoints {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EnableDefaultEgressEndpoints {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EnableDefaultEgressEndpoints {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EnableDefaultEgressEndpoints {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("EnableDefaultEgressEndpoints", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("EnableDefaultEgressEndpoints", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for EnableDefaultEgressEndpoints {
        fn default() -> Self {
            Self::True
        }
    }
    #[doc = "The provisioning state of the cloud services network."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Provisioning,
        Accepted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Provisioning"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Accepted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
    pub properties: ClusterProperties,
}
impl Cluster {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocation, properties: ClusterProperties) -> Self {
        Self {
            tracked_resource,
            extended_location,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterAvailableUpgradeVersion {
    #[doc = "The indicator of whether the control plane will be impacted during the upgrade."]
    #[serde(rename = "controlImpact", default, skip_serializing_if = "Option::is_none")]
    pub control_impact: Option<cluster_available_upgrade_version::ControlImpact>,
    #[doc = "The expected duration needed for this upgrade."]
    #[serde(rename = "expectedDuration", default, skip_serializing_if = "Option::is_none")]
    pub expected_duration: Option<String>,
    #[doc = "The impact description including the specific details and release notes."]
    #[serde(rename = "impactDescription", default, skip_serializing_if = "Option::is_none")]
    pub impact_description: Option<String>,
    #[doc = "The last date the version of the platform is supported."]
    #[serde(rename = "supportExpiryDate", default, skip_serializing_if = "Option::is_none")]
    pub support_expiry_date: Option<String>,
    #[doc = "The target version this cluster will be upgraded to."]
    #[serde(rename = "targetClusterVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_cluster_version: Option<String>,
    #[doc = "The indicator of whether the workload will be impacted during the upgrade."]
    #[serde(rename = "workloadImpact", default, skip_serializing_if = "Option::is_none")]
    pub workload_impact: Option<cluster_available_upgrade_version::WorkloadImpact>,
}
impl ClusterAvailableUpgradeVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cluster_available_upgrade_version {
    use super::*;
    #[doc = "The indicator of whether the control plane will be impacted during the upgrade."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ControlImpact")]
    pub enum ControlImpact {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ControlImpact {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ControlImpact {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ControlImpact {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("ControlImpact", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("ControlImpact", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The indicator of whether the workload will be impacted during the upgrade."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "WorkloadImpact")]
    pub enum WorkloadImpact {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for WorkloadImpact {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for WorkloadImpact {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for WorkloadImpact {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("WorkloadImpact", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("WorkloadImpact", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterAvailableVersion {
    #[doc = "The last date the version of the platform is supported."]
    #[serde(rename = "supportExpiryDate", default, skip_serializing_if = "Option::is_none")]
    pub support_expiry_date: Option<String>,
    #[doc = "The version of the cluster to be deployed."]
    #[serde(rename = "targetClusterVersion", default, skip_serializing_if = "Option::is_none")]
    pub target_cluster_version: Option<String>,
}
impl ClusterAvailableVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterCapacity {
    #[doc = "The remaining appliance-based storage in GB available for workload use."]
    #[serde(rename = "availableApplianceStorageGB", default, skip_serializing_if = "Option::is_none")]
    pub available_appliance_storage_gb: Option<i64>,
    #[doc = "The remaining number of cores that are available in this cluster for workload use."]
    #[serde(rename = "availableCoreCount", default, skip_serializing_if = "Option::is_none")]
    pub available_core_count: Option<i64>,
    #[doc = "The remaining machine or host-based storage in GB available for workload use."]
    #[serde(rename = "availableHostStorageGB", default, skip_serializing_if = "Option::is_none")]
    pub available_host_storage_gb: Option<i64>,
    #[doc = "The remaining memory in GB that are available in this cluster for workload use."]
    #[serde(rename = "availableMemoryGB", default, skip_serializing_if = "Option::is_none")]
    pub available_memory_gb: Option<i64>,
    #[doc = "The total appliance-based storage in GB supported by this cluster for workload use."]
    #[serde(rename = "totalApplianceStorageGB", default, skip_serializing_if = "Option::is_none")]
    pub total_appliance_storage_gb: Option<i64>,
    #[doc = "The total number of cores that are supported by this cluster for workload use."]
    #[serde(rename = "totalCoreCount", default, skip_serializing_if = "Option::is_none")]
    pub total_core_count: Option<i64>,
    #[doc = "The total machine or host-based storage in GB supported by this cluster for workload use."]
    #[serde(rename = "totalHostStorageGB", default, skip_serializing_if = "Option::is_none")]
    pub total_host_storage_gb: Option<i64>,
    #[doc = "The total memory supported by this cluster for workload use."]
    #[serde(rename = "totalMemoryGB", default, skip_serializing_if = "Option::is_none")]
    pub total_memory_gb: Option<i64>,
}
impl ClusterCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterDeployParameters {
    #[doc = "The names of bare metal machines in the cluster that should be skipped during environment validation."]
    #[serde(
        rename = "skipValidationsForMachines",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub skip_validations_for_machines: Vec<String>,
}
impl ClusterDeployParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterList {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of clusters."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Cluster>,
}
impl azure_core::Continuable for ClusterList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ClusterList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterManager {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub properties: ClusterManagerProperties,
}
impl ClusterManager {
    pub fn new(tracked_resource: TrackedResource, properties: ClusterManagerProperties) -> Self {
        Self {
            tracked_resource,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterManagerList {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of cluster managers."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ClusterManager>,
}
impl azure_core::Continuable for ClusterManagerList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ClusterManagerList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterManagerPatchParameters {
    #[doc = "The Azure resource tags that will replace the existing ones."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ClusterManagerPatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterManagerProperties {
    #[doc = "The resource ID of the Log Analytics workspace that is used for the logs collection."]
    #[serde(rename = "analyticsWorkspaceId", default, skip_serializing_if = "Option::is_none")]
    pub analytics_workspace_id: Option<String>,
    #[doc = "Field deprecated, this value will no longer influence the cluster manager allocation process and will be removed in a future version. The Azure availability zones within the region that will be used to support the cluster manager resource."]
    #[serde(
        rename = "availabilityZones",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub availability_zones: Vec<String>,
    #[doc = "The list of the cluster versions the manager supports. It is used as input in clusterVersion property of a cluster resource."]
    #[serde(
        rename = "clusterVersions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cluster_versions: Vec<ClusterAvailableVersion>,
    #[doc = "The detailed status that provides additional information about the cluster manager."]
    #[serde(rename = "detailedStatus", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<cluster_manager_properties::DetailedStatus>,
    #[doc = "The descriptive message about the current detailed status."]
    #[serde(rename = "detailedStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status_message: Option<String>,
    #[doc = "The resource ID of the fabric controller that has one to one mapping with the cluster manager."]
    #[serde(rename = "fabricControllerId")]
    pub fabric_controller_id: String,
    #[serde(rename = "managedResourceGroupConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub managed_resource_group_configuration: Option<ManagedResourceGroupConfiguration>,
    #[serde(rename = "managerExtendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub manager_extended_location: Option<ExtendedLocation>,
    #[doc = "The provisioning state of the cluster manager."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<cluster_manager_properties::ProvisioningState>,
    #[doc = "Field deprecated, this value will no longer influence the cluster manager allocation process and will be removed in a future version. The size of the Azure virtual machines to use for hosting the cluster manager resource."]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
}
impl ClusterManagerProperties {
    pub fn new(fabric_controller_id: String) -> Self {
        Self {
            analytics_workspace_id: None,
            availability_zones: Vec::new(),
            cluster_versions: Vec::new(),
            detailed_status: None,
            detailed_status_message: None,
            fabric_controller_id,
            managed_resource_group_configuration: None,
            manager_extended_location: None,
            provisioning_state: None,
            vm_size: None,
        }
    }
}
pub mod cluster_manager_properties {
    use super::*;
    #[doc = "The detailed status that provides additional information about the cluster manager."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DetailedStatus")]
    pub enum DetailedStatus {
        Error,
        Available,
        Provisioning,
        ProvisioningFailed,
        Updating,
        UpdateFailed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DetailedStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DetailedStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DetailedStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Error => serializer.serialize_unit_variant("DetailedStatus", 0u32, "Error"),
                Self::Available => serializer.serialize_unit_variant("DetailedStatus", 1u32, "Available"),
                Self::Provisioning => serializer.serialize_unit_variant("DetailedStatus", 2u32, "Provisioning"),
                Self::ProvisioningFailed => serializer.serialize_unit_variant("DetailedStatus", 3u32, "ProvisioningFailed"),
                Self::Updating => serializer.serialize_unit_variant("DetailedStatus", 4u32, "Updating"),
                Self::UpdateFailed => serializer.serialize_unit_variant("DetailedStatus", 5u32, "UpdateFailed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state of the cluster manager."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Provisioning,
        Accepted,
        Updating,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Provisioning"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Accepted"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterMetricsConfiguration {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
    pub properties: ClusterMetricsConfigurationProperties,
}
impl ClusterMetricsConfiguration {
    pub fn new(
        tracked_resource: TrackedResource,
        extended_location: ExtendedLocation,
        properties: ClusterMetricsConfigurationProperties,
    ) -> Self {
        Self {
            tracked_resource,
            extended_location,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterMetricsConfigurationList {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of metrics configurations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ClusterMetricsConfiguration>,
}
impl azure_core::Continuable for ClusterMetricsConfigurationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ClusterMetricsConfigurationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterMetricsConfigurationPatchParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterMetricsConfigurationPatchProperties>,
    #[doc = "The Azure resource tags that will replace the existing ones."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ClusterMetricsConfigurationPatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterMetricsConfigurationPatchProperties {
    #[doc = "The interval in minutes by which metrics will be collected."]
    #[serde(rename = "collectionInterval", default, skip_serializing_if = "Option::is_none")]
    pub collection_interval: Option<i64>,
    #[doc = "The list of metric names that have been chosen to be enabled in addition to the core set of enabled metrics."]
    #[serde(
        rename = "enabledMetrics",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enabled_metrics: Vec<String>,
}
impl ClusterMetricsConfigurationPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterMetricsConfigurationProperties {
    #[doc = "The interval in minutes by which metrics will be collected."]
    #[serde(rename = "collectionInterval")]
    pub collection_interval: i64,
    #[doc = "The more detailed status of the metrics configuration."]
    #[serde(rename = "detailedStatus", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<cluster_metrics_configuration_properties::DetailedStatus>,
    #[doc = "The descriptive message about the current detailed status."]
    #[serde(rename = "detailedStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status_message: Option<String>,
    #[doc = "The list of metrics that are available for the cluster but disabled at the moment."]
    #[serde(
        rename = "disabledMetrics",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disabled_metrics: Vec<String>,
    #[doc = "The list of metric names that have been chosen to be enabled in addition to the core set of enabled metrics."]
    #[serde(
        rename = "enabledMetrics",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub enabled_metrics: Vec<String>,
    #[doc = "The provisioning state of the metrics configuration."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<cluster_metrics_configuration_properties::ProvisioningState>,
}
impl ClusterMetricsConfigurationProperties {
    pub fn new(collection_interval: i64) -> Self {
        Self {
            collection_interval,
            detailed_status: None,
            detailed_status_message: None,
            disabled_metrics: Vec::new(),
            enabled_metrics: Vec::new(),
            provisioning_state: None,
        }
    }
}
pub mod cluster_metrics_configuration_properties {
    use super::*;
    #[doc = "The more detailed status of the metrics configuration."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DetailedStatus")]
    pub enum DetailedStatus {
        Processing,
        Applied,
        Error,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DetailedStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DetailedStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DetailedStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Processing => serializer.serialize_unit_variant("DetailedStatus", 0u32, "Processing"),
                Self::Applied => serializer.serialize_unit_variant("DetailedStatus", 1u32, "Applied"),
                Self::Error => serializer.serialize_unit_variant("DetailedStatus", 2u32, "Error"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state of the metrics configuration."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Accepted,
        Provisioning,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Accepted"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterPatchParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterPatchProperties>,
    #[doc = "The Azure resource tags that will replace the existing ones."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ClusterPatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterPatchProperties {
    #[serde(rename = "aggregatorOrSingleRackDefinition", default, skip_serializing_if = "Option::is_none")]
    pub aggregator_or_single_rack_definition: Option<RackDefinition>,
    #[doc = "The customer-provided location information to identify where the cluster resides."]
    #[serde(rename = "clusterLocation", default, skip_serializing_if = "Option::is_none")]
    pub cluster_location: Option<String>,
    #[serde(rename = "clusterServicePrincipal", default, skip_serializing_if = "Option::is_none")]
    pub cluster_service_principal: Option<ServicePrincipalInformation>,
    #[serde(rename = "computeDeploymentThreshold", default, skip_serializing_if = "Option::is_none")]
    pub compute_deployment_threshold: Option<ValidationThreshold>,
    #[doc = "The list of rack definitions for the compute racks in a multi-rack\ncluster, or an empty list in a single-rack cluster."]
    #[serde(
        rename = "computeRackDefinitions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub compute_rack_definitions: Vec<RackDefinition>,
}
impl ClusterPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterProperties {
    #[serde(rename = "aggregatorOrSingleRackDefinition")]
    pub aggregator_or_single_rack_definition: RackDefinition,
    #[doc = "The resource ID of the Log Analytics Workspace that will be used for storing relevant logs."]
    #[serde(rename = "analyticsWorkspaceId")]
    pub analytics_workspace_id: String,
    #[doc = "The list of cluster runtime version upgrades available for this cluster."]
    #[serde(
        rename = "availableUpgradeVersions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub available_upgrade_versions: Vec<ClusterAvailableUpgradeVersion>,
    #[serde(rename = "clusterCapacity", default, skip_serializing_if = "Option::is_none")]
    pub cluster_capacity: Option<ClusterCapacity>,
    #[doc = "The latest heartbeat status between the cluster manager and the cluster."]
    #[serde(rename = "clusterConnectionStatus", default, skip_serializing_if = "Option::is_none")]
    pub cluster_connection_status: Option<cluster_properties::ClusterConnectionStatus>,
    #[serde(rename = "clusterExtendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub cluster_extended_location: Option<ExtendedLocation>,
    #[doc = "The customer-provided location information to identify where the cluster resides."]
    #[serde(rename = "clusterLocation", default, skip_serializing_if = "Option::is_none")]
    pub cluster_location: Option<String>,
    #[doc = "The latest connectivity status between cluster manager and the cluster."]
    #[serde(rename = "clusterManagerConnectionStatus", default, skip_serializing_if = "Option::is_none")]
    pub cluster_manager_connection_status: Option<cluster_properties::ClusterManagerConnectionStatus>,
    #[doc = "The resource ID of the cluster manager that manages this cluster. This is set by the Cluster Manager when the cluster is created."]
    #[serde(rename = "clusterManagerId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_manager_id: Option<String>,
    #[serde(rename = "clusterServicePrincipal", default, skip_serializing_if = "Option::is_none")]
    pub cluster_service_principal: Option<ServicePrincipalInformation>,
    #[doc = "The type of rack configuration for the cluster."]
    #[serde(rename = "clusterType")]
    pub cluster_type: cluster_properties::ClusterType,
    #[doc = "The current runtime version of the cluster."]
    #[serde(rename = "clusterVersion")]
    pub cluster_version: String,
    #[serde(rename = "computeDeploymentThreshold", default, skip_serializing_if = "Option::is_none")]
    pub compute_deployment_threshold: Option<ValidationThreshold>,
    #[doc = "The list of rack definitions for the compute racks in a multi-rack\ncluster, or an empty list in a single-rack cluster."]
    #[serde(
        rename = "computeRackDefinitions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub compute_rack_definitions: Vec<RackDefinition>,
    #[doc = "The current detailed status of the cluster."]
    #[serde(rename = "detailedStatus", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<cluster_properties::DetailedStatus>,
    #[doc = "The descriptive message about the detailed status."]
    #[serde(rename = "detailedStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status_message: Option<String>,
    #[serde(rename = "hybridAksExtendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub hybrid_aks_extended_location: Option<ExtendedLocation>,
    #[serde(rename = "managedResourceGroupConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub managed_resource_group_configuration: Option<ManagedResourceGroupConfiguration>,
    #[doc = "The count of Manual Action Taken (MAT) events that have not been validated."]
    #[serde(rename = "manualActionCount", default, skip_serializing_if = "Option::is_none")]
    pub manual_action_count: Option<i64>,
    #[doc = "The resource ID of the Network Fabric associated with the cluster."]
    #[serde(rename = "networkFabricId")]
    pub network_fabric_id: String,
    #[doc = "The provisioning state of the cluster."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<cluster_properties::ProvisioningState>,
    #[doc = "The support end date of the runtime version of the cluster."]
    #[serde(rename = "supportExpiryDate", default, skip_serializing_if = "Option::is_none")]
    pub support_expiry_date: Option<String>,
    #[doc = "The list of workload resource IDs that are hosted within this cluster."]
    #[serde(
        rename = "workloadResourceIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub workload_resource_ids: Vec<String>,
}
impl ClusterProperties {
    pub fn new(
        aggregator_or_single_rack_definition: RackDefinition,
        analytics_workspace_id: String,
        cluster_type: cluster_properties::ClusterType,
        cluster_version: String,
        network_fabric_id: String,
    ) -> Self {
        Self {
            aggregator_or_single_rack_definition,
            analytics_workspace_id,
            available_upgrade_versions: Vec::new(),
            cluster_capacity: None,
            cluster_connection_status: None,
            cluster_extended_location: None,
            cluster_location: None,
            cluster_manager_connection_status: None,
            cluster_manager_id: None,
            cluster_service_principal: None,
            cluster_type,
            cluster_version,
            compute_deployment_threshold: None,
            compute_rack_definitions: Vec::new(),
            detailed_status: None,
            detailed_status_message: None,
            hybrid_aks_extended_location: None,
            managed_resource_group_configuration: None,
            manual_action_count: None,
            network_fabric_id,
            provisioning_state: None,
            support_expiry_date: None,
            workload_resource_ids: Vec::new(),
        }
    }
}
pub mod cluster_properties {
    use super::*;
    #[doc = "The latest heartbeat status between the cluster manager and the cluster."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ClusterConnectionStatus")]
    pub enum ClusterConnectionStatus {
        Connected,
        Timeout,
        Undefined,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ClusterConnectionStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ClusterConnectionStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ClusterConnectionStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Connected => serializer.serialize_unit_variant("ClusterConnectionStatus", 0u32, "Connected"),
                Self::Timeout => serializer.serialize_unit_variant("ClusterConnectionStatus", 1u32, "Timeout"),
                Self::Undefined => serializer.serialize_unit_variant("ClusterConnectionStatus", 2u32, "Undefined"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The latest connectivity status between cluster manager and the cluster."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ClusterManagerConnectionStatus")]
    pub enum ClusterManagerConnectionStatus {
        Connected,
        Unreachable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ClusterManagerConnectionStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ClusterManagerConnectionStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ClusterManagerConnectionStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Connected => serializer.serialize_unit_variant("ClusterManagerConnectionStatus", 0u32, "Connected"),
                Self::Unreachable => serializer.serialize_unit_variant("ClusterManagerConnectionStatus", 1u32, "Unreachable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of rack configuration for the cluster."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ClusterType")]
    pub enum ClusterType {
        SingleRack,
        MultiRack,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ClusterType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ClusterType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ClusterType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SingleRack => serializer.serialize_unit_variant("ClusterType", 0u32, "SingleRack"),
                Self::MultiRack => serializer.serialize_unit_variant("ClusterType", 1u32, "MultiRack"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The current detailed status of the cluster."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DetailedStatus")]
    pub enum DetailedStatus {
        PendingDeployment,
        Deploying,
        Running,
        Updating,
        Degraded,
        Deleting,
        Disconnected,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DetailedStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DetailedStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DetailedStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PendingDeployment => serializer.serialize_unit_variant("DetailedStatus", 0u32, "PendingDeployment"),
                Self::Deploying => serializer.serialize_unit_variant("DetailedStatus", 1u32, "Deploying"),
                Self::Running => serializer.serialize_unit_variant("DetailedStatus", 2u32, "Running"),
                Self::Updating => serializer.serialize_unit_variant("DetailedStatus", 3u32, "Updating"),
                Self::Degraded => serializer.serialize_unit_variant("DetailedStatus", 4u32, "Degraded"),
                Self::Deleting => serializer.serialize_unit_variant("DetailedStatus", 5u32, "Deleting"),
                Self::Disconnected => serializer.serialize_unit_variant("DetailedStatus", 6u32, "Disconnected"),
                Self::Failed => serializer.serialize_unit_variant("DetailedStatus", 7u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state of the cluster."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Accepted,
        Validating,
        Updating,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Accepted"),
                Self::Validating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Validating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterUpdateVersionParameters {
    #[doc = "The version to be applied to the cluster during update."]
    #[serde(rename = "targetClusterVersion")]
    pub target_cluster_version: String,
}
impl ClusterUpdateVersionParameters {
    pub fn new(target_cluster_version: String) -> Self {
        Self { target_cluster_version }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CniBgpConfiguration {
    #[doc = "The list of BgpPeer entities that the Hybrid AKS cluster will peer with in addition to peering that occurs automatically with the switch fabric."]
    #[serde(
        rename = "bgpPeers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub bgp_peers: Vec<BgpPeer>,
    #[doc = "The list of prefix community advertisement properties. Each prefix community specifies a prefix, and the\ncommunities that should be associated with that prefix when it is announced."]
    #[serde(
        rename = "communityAdvertisements",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub community_advertisements: Vec<CommunityAdvertisement>,
    #[doc = "The password of the Calico node mesh. It defaults to a randomly-generated string when not provided."]
    #[serde(rename = "nodeMeshPassword", default, skip_serializing_if = "Option::is_none")]
    pub node_mesh_password: Option<String>,
    #[doc = "The subnet blocks in CIDR format for Kubernetes service external IPs to be advertised over BGP."]
    #[serde(
        rename = "serviceExternalPrefixes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub service_external_prefixes: Vec<String>,
    #[doc = "The subnet blocks in CIDR format for Kubernetes load balancers. Load balancer IPs will only be advertised if they\nare within one of these blocks."]
    #[serde(
        rename = "serviceLoadBalancerPrefixes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub service_load_balancer_prefixes: Vec<String>,
}
impl CniBgpConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommunityAdvertisement {
    #[doc = "The list of community strings to announce with this prefix."]
    pub communities: Vec<String>,
    #[doc = "The subnet in CIDR format for which properties should be advertised."]
    #[serde(rename = "subnetPrefix")]
    pub subnet_prefix: String,
}
impl CommunityAdvertisement {
    pub fn new(communities: Vec<String>, subnet_prefix: String) -> Self {
        Self {
            communities,
            subnet_prefix,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Console {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
    pub properties: ConsoleProperties,
}
impl Console {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocation, properties: ConsoleProperties) -> Self {
        Self {
            tracked_resource,
            extended_location,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsoleList {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of virtual machine consoles."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Console>,
}
impl azure_core::Continuable for ConsoleList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ConsoleList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsolePatchParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConsolePatchProperties>,
    #[doc = "The Azure resource tags that will replace the existing ones."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ConsolePatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsolePatchProperties {
    #[doc = "The credentials used to login to the image repository that has access to the specified image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<console_patch_properties::Enabled>,
    #[doc = "The date and time after which the key will be disallowed access."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub expiration: Option<time::OffsetDateTime>,
    #[serde(rename = "sshPublicKey", default, skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<SshPublicKey>,
}
impl ConsolePatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod console_patch_properties {
    use super::*;
    #[doc = "The credentials used to login to the image repository that has access to the specified image."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Enabled")]
    pub enum Enabled {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Enabled {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Enabled {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Enabled {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("Enabled", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("Enabled", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsoleProperties {
    #[doc = "The more detailed status of the console."]
    #[serde(rename = "detailedStatus", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<console_properties::DetailedStatus>,
    #[doc = "The descriptive message about the current detailed status."]
    #[serde(rename = "detailedStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status_message: Option<String>,
    #[doc = "The indicator of whether the console access is enabled."]
    pub enabled: console_properties::Enabled,
    #[doc = "The date and time after which the key will be disallowed access."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub expiration: Option<time::OffsetDateTime>,
    #[doc = "The resource ID of the private link service that is used to provide virtual machine console access."]
    #[serde(rename = "privateLinkServiceId", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_id: Option<String>,
    #[doc = "The provisioning state of the virtual machine console."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<console_properties::ProvisioningState>,
    #[serde(rename = "sshPublicKey")]
    pub ssh_public_key: SshPublicKey,
    #[doc = "The unique identifier for the virtual machine that is used to access the console."]
    #[serde(rename = "virtualMachineAccessId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_access_id: Option<String>,
}
impl ConsoleProperties {
    pub fn new(enabled: console_properties::Enabled, ssh_public_key: SshPublicKey) -> Self {
        Self {
            detailed_status: None,
            detailed_status_message: None,
            enabled,
            expiration: None,
            private_link_service_id: None,
            provisioning_state: None,
            ssh_public_key,
            virtual_machine_access_id: None,
        }
    }
}
pub mod console_properties {
    use super::*;
    #[doc = "The more detailed status of the console."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DetailedStatus")]
    pub enum DetailedStatus {
        Ready,
        Error,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DetailedStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DetailedStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DetailedStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ready => serializer.serialize_unit_variant("DetailedStatus", 0u32, "Ready"),
                Self::Error => serializer.serialize_unit_variant("DetailedStatus", 1u32, "Error"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The indicator of whether the console access is enabled."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Enabled")]
    pub enum Enabled {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Enabled {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Enabled {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Enabled {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("Enabled", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("Enabled", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state of the virtual machine console."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Provisioning,
        Accepted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Provisioning"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Accepted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefaultCniNetwork {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
    pub properties: DefaultCniNetworkProperties,
}
impl DefaultCniNetwork {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocation, properties: DefaultCniNetworkProperties) -> Self {
        Self {
            tracked_resource,
            extended_location,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DefaultCniNetworkList {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of default CNI networks."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DefaultCniNetwork>,
}
impl azure_core::Continuable for DefaultCniNetworkList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DefaultCniNetworkList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DefaultCniNetworkPatchParameters {
    #[doc = "The Azure resource tags that will replace the existing ones."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DefaultCniNetworkPatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefaultCniNetworkProperties {
    #[doc = "The resource ID of the Network Cloud cluster this default CNI network is associated with."]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc = "The autonomous system number that the fabric expects to peer with, derived from the associated L3 isolation domain."]
    #[serde(rename = "cniAsNumber", default, skip_serializing_if = "Option::is_none")]
    pub cni_as_number: Option<i64>,
    #[serde(rename = "cniBgpConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub cni_bgp_configuration: Option<CniBgpConfiguration>,
    #[doc = "The more detailed status of the default CNI network."]
    #[serde(rename = "detailedStatus", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<default_cni_network_properties::DetailedStatus>,
    #[doc = "The descriptive message about the current detailed status."]
    #[serde(rename = "detailedStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status_message: Option<String>,
    #[doc = "The L3 isolation fabric BGP peering connectivity information necessary for BGP peering the Hybrid AKS Cluster with the switch fabric."]
    #[serde(
        rename = "fabricBgpPeers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub fabric_bgp_peers: Vec<BgpPeer>,
    #[doc = "The list of Hybrid AKS cluster resource ID(s) that are associated with this default CNI network."]
    #[serde(
        rename = "hybridAksClustersAssociatedIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hybrid_aks_clusters_associated_ids: Vec<String>,
    #[doc = "The name of the interface that will be present in the virtual machine to represent this network."]
    #[serde(rename = "interfaceName", default, skip_serializing_if = "Option::is_none")]
    pub interface_name: Option<String>,
    #[doc = "The type of the IP address allocation."]
    #[serde(rename = "ipAllocationType", default, skip_serializing_if = "Option::is_none")]
    pub ip_allocation_type: Option<default_cni_network_properties::IpAllocationType>,
    #[doc = "The IPV4 prefix (CIDR) assigned to this default CNI network. It is required when the IP allocation type\nis IPV4 or DualStack."]
    #[serde(rename = "ipv4ConnectedPrefix", default, skip_serializing_if = "Option::is_none")]
    pub ipv4_connected_prefix: Option<String>,
    #[doc = "The IPV6 prefix (CIDR) assigned to this default CNI network. It is required when the IP allocation type\nis IPV6 or DualStack."]
    #[serde(rename = "ipv6ConnectedPrefix", default, skip_serializing_if = "Option::is_none")]
    pub ipv6_connected_prefix: Option<String>,
    #[doc = "The resource ID of the Network Fabric l3IsolationDomain."]
    #[serde(rename = "l3IsolationDomainId")]
    pub l3_isolation_domain_id: String,
    #[doc = "The provisioning state of the default CNI network."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<default_cni_network_properties::ProvisioningState>,
    #[doc = "The VLAN from the l3IsolationDomain that is used for this network."]
    pub vlan: i64,
}
impl DefaultCniNetworkProperties {
    pub fn new(l3_isolation_domain_id: String, vlan: i64) -> Self {
        Self {
            cluster_id: None,
            cni_as_number: None,
            cni_bgp_configuration: None,
            detailed_status: None,
            detailed_status_message: None,
            fabric_bgp_peers: Vec::new(),
            hybrid_aks_clusters_associated_ids: Vec::new(),
            interface_name: None,
            ip_allocation_type: None,
            ipv4_connected_prefix: None,
            ipv6_connected_prefix: None,
            l3_isolation_domain_id,
            provisioning_state: None,
            vlan,
        }
    }
}
pub mod default_cni_network_properties {
    use super::*;
    #[doc = "The more detailed status of the default CNI network."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DetailedStatus")]
    pub enum DetailedStatus {
        Error,
        Available,
        Provisioning,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DetailedStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DetailedStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DetailedStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Error => serializer.serialize_unit_variant("DetailedStatus", 0u32, "Error"),
                Self::Available => serializer.serialize_unit_variant("DetailedStatus", 1u32, "Available"),
                Self::Provisioning => serializer.serialize_unit_variant("DetailedStatus", 2u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of the IP address allocation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IpAllocationType")]
    pub enum IpAllocationType {
        #[serde(rename = "IPV4")]
        Ipv4,
        #[serde(rename = "IPV6")]
        Ipv6,
        DualStack,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IpAllocationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IpAllocationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IpAllocationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ipv4 => serializer.serialize_unit_variant("IpAllocationType", 0u32, "IPV4"),
                Self::Ipv6 => serializer.serialize_unit_variant("IpAllocationType", 1u32, "IPV6"),
                Self::DualStack => serializer.serialize_unit_variant("IpAllocationType", 2u32, "DualStack"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for IpAllocationType {
        fn default() -> Self {
            Self::DualStack
        }
    }
    #[doc = "The provisioning state of the default CNI network."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Provisioning,
        Accepted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Provisioning"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Accepted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EgressEndpoint {
    #[doc = "The descriptive category name of endpoints accessible by the AKS agent node. For example, azure-resource-management, API server, etc. The platform egress endpoints provided by default will use the category 'default'."]
    pub category: String,
    #[doc = "The list of endpoint dependencies."]
    pub endpoints: Vec<EndpointDependency>,
}
impl EgressEndpoint {
    pub fn new(category: String, endpoints: Vec<EndpointDependency>) -> Self {
        Self { category, endpoints }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointDependency {
    #[doc = "The domain name of the dependency."]
    #[serde(rename = "domainName")]
    pub domain_name: String,
    #[doc = "The port of this endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}
impl EndpointDependency {
    pub fn new(domain_name: String) -> Self {
        Self { domain_name, port: None }
    }
}
#[doc = "The resource management error additional info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[doc = "The additional info type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The additional info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(
        rename = "additionalInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl azure_core::Continuable for ErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtendedLocation {
    #[doc = "The resource ID of the extended location on which the resource will be created."]
    pub name: String,
    #[doc = "The extended location type, for example, CustomLocation."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl ExtendedLocation {
    pub fn new(name: String, type_: String) -> Self {
        Self { name, type_ }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HardwareInventory {
    #[doc = "Freeform data extracted from the environment about this machine. This information varies depending on the specific hardware and configuration."]
    #[serde(rename = "additionalHostInformation", default, skip_serializing_if = "Option::is_none")]
    pub additional_host_information: Option<String>,
    #[doc = "The list of network interfaces and associated details for the bare metal machine."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub interfaces: Vec<HardwareInventoryNetworkInterface>,
    #[doc = "Field Deprecated. Will be removed in an upcoming version. The list of network interface cards and associated details for the bare metal machine."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub nics: Vec<Nic>,
}
impl HardwareInventory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HardwareInventoryNetworkInterface {
    #[doc = "The current status of the link."]
    #[serde(rename = "linkStatus", default, skip_serializing_if = "Option::is_none")]
    pub link_status: Option<String>,
    #[doc = "The MAC address associated with this interface."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "The name of the interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource ID of the network interface for the port on the switch that this machine's interface is connected to."]
    #[serde(rename = "networkInterfaceId", default, skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
}
impl HardwareInventoryNetworkInterface {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HardwareValidationStatus {
    #[doc = "The timestamp of the hardware validation execution."]
    #[serde(rename = "lastValidationTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_validation_time: Option<time::OffsetDateTime>,
    #[doc = "The outcome of the hardware validation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<hardware_validation_status::Result>,
}
impl HardwareValidationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod hardware_validation_status {
    use super::*;
    #[doc = "The outcome of the hardware validation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Result")]
    pub enum Result {
        Pass,
        Fail,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Result {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Result {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Result {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Pass => serializer.serialize_unit_variant("Result", 0u32, "Pass"),
                Self::Fail => serializer.serialize_unit_variant("Result", 1u32, "Fail"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The details are specific to the Network Cloud use of the Hybrid AKS cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HybridAksCluster {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
    pub properties: HybridAksClusterProperties,
}
impl HybridAksCluster {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocation, properties: HybridAksClusterProperties) -> Self {
        Self {
            tracked_resource,
            extended_location,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridAksClusterList {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of additional details related to Hybrid AKS clusters."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<HybridAksCluster>,
}
impl azure_core::Continuable for HybridAksClusterList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl HybridAksClusterList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridAksClusterPatchParameters {
    #[doc = "The Azure resource tags that will replace the existing ones."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl HybridAksClusterPatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HybridAksClusterProperties {
    #[doc = "The list of resource IDs for the workload networks associated with the Hybrid AKS cluster. It can be any of l2Networks, l3Networks, or trunkedNetworks resources. This field will also contain one cloudServicesNetwork and one defaultCniNetwork."]
    #[serde(rename = "associatedNetworkIds")]
    pub associated_network_ids: Vec<String>,
    #[doc = "The resource ID of the associated cloud services network."]
    #[serde(rename = "cloudServicesNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub cloud_services_network_id: Option<String>,
    #[doc = "The resource ID of the Network Cloud cluster hosting the Hybrid AKS cluster."]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc = "The number of control plane node VMs."]
    #[serde(rename = "controlPlaneCount")]
    pub control_plane_count: i64,
    #[doc = "The list of node configurations detailing associated VMs that are part of the control plane nodes of this Hybrid AKS cluster."]
    #[serde(
        rename = "controlPlaneNodes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub control_plane_nodes: Vec<NodeConfiguration>,
    #[doc = "The resource ID of the associated default CNI network."]
    #[serde(rename = "defaultCniNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub default_cni_network_id: Option<String>,
    #[doc = "The more detailed status of this Hybrid AKS cluster."]
    #[serde(rename = "detailedStatus", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<hybrid_aks_cluster_properties::DetailedStatus>,
    #[doc = "The descriptive message about the current detailed status."]
    #[serde(rename = "detailedStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status_message: Option<String>,
    #[doc = "The resource ID of the Hybrid AKS cluster that this additional information is for."]
    #[serde(rename = "hybridAksProvisionedClusterId")]
    pub hybrid_aks_provisioned_cluster_id: String,
    #[doc = "The provisioning state of the Hybrid AKS cluster resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<hybrid_aks_cluster_properties::ProvisioningState>,
    #[doc = "The resource IDs of volumes that are attached to the Hybrid AKS cluster."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub volumes: Vec<String>,
    #[doc = "The number of worker node VMs."]
    #[serde(rename = "workerCount")]
    pub worker_count: i64,
    #[doc = "The list of node configurations detailing associated VMs that are part of the worker nodes of this Hybrid AKS cluster."]
    #[serde(
        rename = "workerNodes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub worker_nodes: Vec<NodeConfiguration>,
}
impl HybridAksClusterProperties {
    pub fn new(
        associated_network_ids: Vec<String>,
        control_plane_count: i64,
        hybrid_aks_provisioned_cluster_id: String,
        worker_count: i64,
    ) -> Self {
        Self {
            associated_network_ids,
            cloud_services_network_id: None,
            cluster_id: None,
            control_plane_count,
            control_plane_nodes: Vec::new(),
            default_cni_network_id: None,
            detailed_status: None,
            detailed_status_message: None,
            hybrid_aks_provisioned_cluster_id,
            provisioning_state: None,
            volumes: Vec::new(),
            worker_count,
            worker_nodes: Vec::new(),
        }
    }
}
pub mod hybrid_aks_cluster_properties {
    use super::*;
    #[doc = "The more detailed status of this Hybrid AKS cluster."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DetailedStatus")]
    pub enum DetailedStatus {
        Error,
        Available,
        Provisioning,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DetailedStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DetailedStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DetailedStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Error => serializer.serialize_unit_variant("DetailedStatus", 0u32, "Error"),
                Self::Available => serializer.serialize_unit_variant("DetailedStatus", 1u32, "Available"),
                Self::Provisioning => serializer.serialize_unit_variant("DetailedStatus", 2u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state of the Hybrid AKS cluster resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HybridAksClusterRestartNodeParameters {
    #[doc = "The name of the node to restart."]
    #[serde(rename = "nodeName")]
    pub node_name: String,
}
impl HybridAksClusterRestartNodeParameters {
    pub fn new(node_name: String) -> Self {
        Self { node_name }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageRepositoryCredentials {
    #[doc = "The password or token used to access an image in the target repository."]
    pub password: String,
    #[doc = "The URL of the authentication server used to validate the repository credentials."]
    #[serde(rename = "registryUrl")]
    pub registry_url: String,
    #[doc = "The username used to access an image in the target repository."]
    pub username: String,
}
impl ImageRepositoryCredentials {
    pub fn new(password: String, registry_url: String, username: String) -> Self {
        Self {
            password,
            registry_url,
            username,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeySetUser {
    #[doc = "The Azure Active Directory user name (email name)."]
    #[serde(rename = "azureUserName")]
    pub azure_user_name: String,
    #[doc = "The free-form description for this user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "sshPublicKey")]
    pub ssh_public_key: SshPublicKey,
}
impl KeySetUser {
    pub fn new(azure_user_name: String, ssh_public_key: SshPublicKey) -> Self {
        Self {
            azure_user_name,
            description: None,
            ssh_public_key,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeySetUserStatus {
    #[doc = "The Azure Active Directory user name (email name)."]
    #[serde(rename = "azureUserName", default, skip_serializing_if = "Option::is_none")]
    pub azure_user_name: Option<String>,
    #[doc = "The indicator of whether the user is currently deployed for access."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<key_set_user_status::Status>,
    #[doc = "The additional information describing the current status of this user, if any available."]
    #[serde(rename = "statusMessage", default, skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}
impl KeySetUserStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod key_set_user_status {
    use super::*;
    #[doc = "The indicator of whether the user is currently deployed for access."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Active,
        Invalid,
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
                Self::Active => serializer.serialize_unit_variant("Status", 0u32, "Active"),
                Self::Invalid => serializer.serialize_unit_variant("Status", 1u32, "Invalid"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct L2Network {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
    pub properties: L2NetworkProperties,
}
impl L2Network {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocation, properties: L2NetworkProperties) -> Self {
        Self {
            tracked_resource,
            extended_location,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct L2NetworkList {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of L2 networks."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<L2Network>,
}
impl azure_core::Continuable for L2NetworkList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl L2NetworkList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct L2NetworkPatchParameters {
    #[doc = "The Azure resource tags that will replace the existing ones."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl L2NetworkPatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct L2NetworkProperties {
    #[doc = "The resource ID of the Network Cloud cluster this L2 network is associated with."]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc = "The more detailed status of the L2 network."]
    #[serde(rename = "detailedStatus", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<l2_network_properties::DetailedStatus>,
    #[doc = "The descriptive message about the current detailed status."]
    #[serde(rename = "detailedStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status_message: Option<String>,
    #[doc = "The list of Hybrid AKS cluster resource ID(s) that are associated with this L2 network."]
    #[serde(
        rename = "hybridAksClustersAssociatedIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hybrid_aks_clusters_associated_ids: Vec<String>,
    #[doc = "The network plugin type for Hybrid AKS."]
    #[serde(rename = "hybridAksPluginType", default, skip_serializing_if = "Option::is_none")]
    pub hybrid_aks_plugin_type: Option<l2_network_properties::HybridAksPluginType>,
    #[doc = "The default interface name for this L2 network in the virtual machine. This name can be overridden by the name supplied in the network attachment configuration of that virtual machine."]
    #[serde(rename = "interfaceName", default, skip_serializing_if = "Option::is_none")]
    pub interface_name: Option<String>,
    #[doc = "The resource ID of the Network Fabric l2IsolationDomain."]
    #[serde(rename = "l2IsolationDomainId")]
    pub l2_isolation_domain_id: String,
    #[doc = "The provisioning state of the L2 network."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<l2_network_properties::ProvisioningState>,
    #[doc = "The list of virtual machine resource ID(s), excluding any Hybrid AKS virtual machines, that are currently using this L2 network."]
    #[serde(
        rename = "virtualMachinesAssociatedIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub virtual_machines_associated_ids: Vec<String>,
}
impl L2NetworkProperties {
    pub fn new(l2_isolation_domain_id: String) -> Self {
        Self {
            cluster_id: None,
            detailed_status: None,
            detailed_status_message: None,
            hybrid_aks_clusters_associated_ids: Vec::new(),
            hybrid_aks_plugin_type: None,
            interface_name: None,
            l2_isolation_domain_id,
            provisioning_state: None,
            virtual_machines_associated_ids: Vec::new(),
        }
    }
}
pub mod l2_network_properties {
    use super::*;
    #[doc = "The more detailed status of the L2 network."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DetailedStatus")]
    pub enum DetailedStatus {
        Error,
        Available,
        Provisioning,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DetailedStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DetailedStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DetailedStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Error => serializer.serialize_unit_variant("DetailedStatus", 0u32, "Error"),
                Self::Available => serializer.serialize_unit_variant("DetailedStatus", 1u32, "Available"),
                Self::Provisioning => serializer.serialize_unit_variant("DetailedStatus", 2u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The network plugin type for Hybrid AKS."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HybridAksPluginType")]
    pub enum HybridAksPluginType {
        #[serde(rename = "DPDK")]
        Dpdk,
        #[serde(rename = "SRIOV")]
        Sriov,
        #[serde(rename = "OSDevice")]
        OsDevice,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HybridAksPluginType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HybridAksPluginType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HybridAksPluginType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Dpdk => serializer.serialize_unit_variant("HybridAksPluginType", 0u32, "DPDK"),
                Self::Sriov => serializer.serialize_unit_variant("HybridAksPluginType", 1u32, "SRIOV"),
                Self::OsDevice => serializer.serialize_unit_variant("HybridAksPluginType", 2u32, "OSDevice"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for HybridAksPluginType {
        fn default() -> Self {
            Self::Sriov
        }
    }
    #[doc = "The provisioning state of the L2 network."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Provisioning,
        Accepted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Provisioning"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Accepted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct L3Network {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
    pub properties: L3NetworkProperties,
}
impl L3Network {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocation, properties: L3NetworkProperties) -> Self {
        Self {
            tracked_resource,
            extended_location,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct L3NetworkList {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of L3 networks."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<L3Network>,
}
impl azure_core::Continuable for L3NetworkList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl L3NetworkList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct L3NetworkPatchParameters {
    #[doc = "The Azure resource tags that will replace the existing ones."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl L3NetworkPatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct L3NetworkProperties {
    #[doc = "The resource ID of the Network Cloud cluster this L3 network is associated with."]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc = "The more detailed status of the L3 network."]
    #[serde(rename = "detailedStatus", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<l3_network_properties::DetailedStatus>,
    #[doc = "The descriptive message about the current detailed status."]
    #[serde(rename = "detailedStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status_message: Option<String>,
    #[doc = "The list of Hybrid AKS cluster resource IDs that are associated with this L3 network."]
    #[serde(
        rename = "hybridAksClustersAssociatedIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hybrid_aks_clusters_associated_ids: Vec<String>,
    #[doc = "The indicator of whether or not to disable IPAM allocation on the network attachment definition injected into the Hybrid AKS Cluster."]
    #[serde(rename = "hybridAksIpamEnabled", default, skip_serializing_if = "Option::is_none")]
    pub hybrid_aks_ipam_enabled: Option<l3_network_properties::HybridAksIpamEnabled>,
    #[doc = "The network plugin type for Hybrid AKS."]
    #[serde(rename = "hybridAksPluginType", default, skip_serializing_if = "Option::is_none")]
    pub hybrid_aks_plugin_type: Option<l3_network_properties::HybridAksPluginType>,
    #[doc = "The default interface name for this L3 network in the virtual machine. This name can be overridden by the name supplied in the network attachment configuration of that virtual machine."]
    #[serde(rename = "interfaceName", default, skip_serializing_if = "Option::is_none")]
    pub interface_name: Option<String>,
    #[doc = "The type of the IP address allocation, defaulted to \"DualStack\"."]
    #[serde(rename = "ipAllocationType", default, skip_serializing_if = "Option::is_none")]
    pub ip_allocation_type: Option<l3_network_properties::IpAllocationType>,
    #[doc = "The IPV4 prefix (CIDR) assigned to this L3 network. Required when the IP allocation type\nis IPV4 or DualStack."]
    #[serde(rename = "ipv4ConnectedPrefix", default, skip_serializing_if = "Option::is_none")]
    pub ipv4_connected_prefix: Option<String>,
    #[doc = "The IPV6 prefix (CIDR) assigned to this L3 network. Required when the IP allocation type\nis IPV6 or DualStack."]
    #[serde(rename = "ipv6ConnectedPrefix", default, skip_serializing_if = "Option::is_none")]
    pub ipv6_connected_prefix: Option<String>,
    #[doc = "The resource ID of the Network Fabric l3IsolationDomain."]
    #[serde(rename = "l3IsolationDomainId")]
    pub l3_isolation_domain_id: String,
    #[doc = "The provisioning state of the L3 network."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<l3_network_properties::ProvisioningState>,
    #[doc = "The list of virtual machine resource IDs, excluding any Hybrid AKS virtual machines, that are currently using this L3 network."]
    #[serde(
        rename = "virtualMachinesAssociatedIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub virtual_machines_associated_ids: Vec<String>,
    #[doc = "The VLAN from the l3IsolationDomain that is used for this network."]
    pub vlan: i64,
}
impl L3NetworkProperties {
    pub fn new(l3_isolation_domain_id: String, vlan: i64) -> Self {
        Self {
            cluster_id: None,
            detailed_status: None,
            detailed_status_message: None,
            hybrid_aks_clusters_associated_ids: Vec::new(),
            hybrid_aks_ipam_enabled: None,
            hybrid_aks_plugin_type: None,
            interface_name: None,
            ip_allocation_type: None,
            ipv4_connected_prefix: None,
            ipv6_connected_prefix: None,
            l3_isolation_domain_id,
            provisioning_state: None,
            virtual_machines_associated_ids: Vec::new(),
            vlan,
        }
    }
}
pub mod l3_network_properties {
    use super::*;
    #[doc = "The more detailed status of the L3 network."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DetailedStatus")]
    pub enum DetailedStatus {
        Error,
        Available,
        Provisioning,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DetailedStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DetailedStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DetailedStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Error => serializer.serialize_unit_variant("DetailedStatus", 0u32, "Error"),
                Self::Available => serializer.serialize_unit_variant("DetailedStatus", 1u32, "Available"),
                Self::Provisioning => serializer.serialize_unit_variant("DetailedStatus", 2u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The indicator of whether or not to disable IPAM allocation on the network attachment definition injected into the Hybrid AKS Cluster."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HybridAksIpamEnabled")]
    pub enum HybridAksIpamEnabled {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HybridAksIpamEnabled {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HybridAksIpamEnabled {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HybridAksIpamEnabled {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("HybridAksIpamEnabled", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("HybridAksIpamEnabled", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for HybridAksIpamEnabled {
        fn default() -> Self {
            Self::True
        }
    }
    #[doc = "The network plugin type for Hybrid AKS."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HybridAksPluginType")]
    pub enum HybridAksPluginType {
        #[serde(rename = "DPDK")]
        Dpdk,
        #[serde(rename = "SRIOV")]
        Sriov,
        #[serde(rename = "OSDevice")]
        OsDevice,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HybridAksPluginType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HybridAksPluginType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HybridAksPluginType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Dpdk => serializer.serialize_unit_variant("HybridAksPluginType", 0u32, "DPDK"),
                Self::Sriov => serializer.serialize_unit_variant("HybridAksPluginType", 1u32, "SRIOV"),
                Self::OsDevice => serializer.serialize_unit_variant("HybridAksPluginType", 2u32, "OSDevice"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for HybridAksPluginType {
        fn default() -> Self {
            Self::Sriov
        }
    }
    #[doc = "The type of the IP address allocation, defaulted to \"DualStack\"."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IpAllocationType")]
    pub enum IpAllocationType {
        #[serde(rename = "IPV4")]
        Ipv4,
        #[serde(rename = "IPV6")]
        Ipv6,
        DualStack,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IpAllocationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IpAllocationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IpAllocationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ipv4 => serializer.serialize_unit_variant("IpAllocationType", 0u32, "IPV4"),
                Self::Ipv6 => serializer.serialize_unit_variant("IpAllocationType", 1u32, "IPV6"),
                Self::DualStack => serializer.serialize_unit_variant("IpAllocationType", 2u32, "DualStack"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for IpAllocationType {
        fn default() -> Self {
            Self::DualStack
        }
    }
    #[doc = "The provisioning state of the L3 network."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Provisioning,
        Accepted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Provisioning"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Accepted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LldpNeighbor {
    #[doc = "The descriptive information about the port on the connected device."]
    #[serde(rename = "portDescription", default, skip_serializing_if = "Option::is_none")]
    pub port_description: Option<String>,
    #[doc = "The system-assigned name of the port on the connected device."]
    #[serde(rename = "portName", default, skip_serializing_if = "Option::is_none")]
    pub port_name: Option<String>,
    #[doc = "The descriptive information about the connected device."]
    #[serde(rename = "systemDescription", default, skip_serializing_if = "Option::is_none")]
    pub system_description: Option<String>,
    #[doc = "The system-assigned name of the connected device."]
    #[serde(rename = "systemName", default, skip_serializing_if = "Option::is_none")]
    pub system_name: Option<String>,
}
impl LldpNeighbor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineDisk {
    #[doc = "The maximum amount of storage in GB."]
    #[serde(rename = "capacityGB", default, skip_serializing_if = "Option::is_none")]
    pub capacity_gb: Option<i64>,
    #[doc = "The connection type of the rack SKU resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection: Option<machine_disk::Connection>,
    #[doc = "The disk type of rack SKU resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<machine_disk::Type>,
}
impl MachineDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod machine_disk {
    use super::*;
    #[doc = "The connection type of the rack SKU resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Connection")]
    pub enum Connection {
        #[serde(rename = "PCIE")]
        Pcie,
        #[serde(rename = "SATA")]
        Sata,
        #[serde(rename = "RAID")]
        Raid,
        #[serde(rename = "SAS")]
        Sas,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Connection {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Connection {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Connection {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Pcie => serializer.serialize_unit_variant("Connection", 0u32, "PCIE"),
                Self::Sata => serializer.serialize_unit_variant("Connection", 1u32, "SATA"),
                Self::Raid => serializer.serialize_unit_variant("Connection", 2u32, "RAID"),
                Self::Sas => serializer.serialize_unit_variant("Connection", 3u32, "SAS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The disk type of rack SKU resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "HDD")]
        Hdd,
        #[serde(rename = "SSD")]
        Ssd,
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
                Self::Hdd => serializer.serialize_unit_variant("Type", 0u32, "HDD"),
                Self::Ssd => serializer.serialize_unit_variant("Type", 1u32, "SSD"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineSkuProperties {
    #[doc = "The type of bootstrap protocol used."]
    #[serde(rename = "bootstrapProtocol", default, skip_serializing_if = "Option::is_none")]
    pub bootstrap_protocol: Option<machine_sku_properties::BootstrapProtocol>,
    #[doc = "The count of CPU cores for this machine."]
    #[serde(rename = "cpuCores", default, skip_serializing_if = "Option::is_none")]
    pub cpu_cores: Option<i64>,
    #[doc = "The count of CPU sockets for this machine."]
    #[serde(rename = "cpuSockets", default, skip_serializing_if = "Option::is_none")]
    pub cpu_sockets: Option<i64>,
    #[doc = "The list of disks."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disks: Vec<MachineDisk>,
    #[doc = "The generation of the architecture."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,
    #[doc = "The hardware version of the machine."]
    #[serde(rename = "hardwareVersion", default, skip_serializing_if = "Option::is_none")]
    pub hardware_version: Option<String>,
    #[doc = "The maximum amount of memory in GB."]
    #[serde(rename = "memoryCapacityGB", default, skip_serializing_if = "Option::is_none")]
    pub memory_capacity_gb: Option<i64>,
    #[doc = "The model of the machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[doc = "The list of network interfaces."]
    #[serde(
        rename = "networkInterfaces",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub network_interfaces: Vec<NetworkInterface>,
    #[doc = "The count of SMT and physical core threads for this machine."]
    #[serde(rename = "totalThreads", default, skip_serializing_if = "Option::is_none")]
    pub total_threads: Option<i64>,
    #[doc = "The make of the machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
}
impl MachineSkuProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod machine_sku_properties {
    use super::*;
    #[doc = "The type of bootstrap protocol used."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BootstrapProtocol")]
    pub enum BootstrapProtocol {
        #[serde(rename = "PXE")]
        Pxe,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BootstrapProtocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BootstrapProtocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BootstrapProtocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Pxe => serializer.serialize_unit_variant("BootstrapProtocol", 0u32, "PXE"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineSkuSlot {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MachineSkuProperties>,
    #[doc = "The position in the rack for the machine."]
    #[serde(rename = "rackSlot", default, skip_serializing_if = "Option::is_none")]
    pub rack_slot: Option<i64>,
}
impl MachineSkuSlot {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedResourceGroupConfiguration {
    #[doc = "The location of the managed resource group. If not specified, the location of the parent resource is chosen."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The name for the managed resource group. If not specified, the unique name is automatically generated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ManagedResourceGroupConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkAttachment {
    #[doc = "The resource ID of the associated network attached to the virtual machine.\nIt can be one of cloudServicesNetwork, l3Network, l2Network or trunkedNetwork resources."]
    #[serde(rename = "attachedNetworkId")]
    pub attached_network_id: String,
    #[doc = "The indicator of whether this is the default gateway.\nOnly one of the attached networks (including the CloudServicesNetwork attachment) for a single machine may be specified as True."]
    #[serde(rename = "defaultGateway", default, skip_serializing_if = "Option::is_none")]
    pub default_gateway: Option<network_attachment::DefaultGateway>,
    #[doc = "The IP allocation mechanism for the virtual machine.\nDynamic and Static are only valid for l3Network which may also specify Disabled.\nOtherwise, Disabled is the only permitted value."]
    #[serde(rename = "ipAllocationMethod")]
    pub ip_allocation_method: network_attachment::IpAllocationMethod,
    #[doc = "The IPv4 address of the virtual machine.\n\nThis field is used only if the attached network has IPAllocationType of IPV4 or DualStack.\n\nIf IPAllocationMethod is:\nStatic - this field must contain a user specified IPv4 address from within the subnet specified in the attached network.\nDynamic - this field is read-only, but will be populated with an address from within the subnet specified in the attached network.\nDisabled - this field will be empty."]
    #[serde(rename = "ipv4Address", default, skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<String>,
    #[doc = "The IPv6 address of the virtual machine.\n\nThis field is used only if the attached network has IPAllocationType of IPV6 or DualStack.\n\nIf IPAllocationMethod is:\nStatic - this field must contain an IPv6 address range from within the range specified in the attached network.\nDynamic - this field is read-only, but will be populated with an range from within the subnet specified in the attached network.\nDisabled - this field will be empty."]
    #[serde(rename = "ipv6Address", default, skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
    #[doc = "The MAC address of the interface for the virtual machine that corresponds to this network attachment."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "The associated network's interface name.\nIf specified, the network attachment name has a maximum length of 15 characters and must be unique to this virtual machine.\nIf the user doesn’t specify this value, the default interface name of the network resource will be used.\nFor a CloudServicesNetwork resource, this name will be ignored."]
    #[serde(rename = "networkAttachmentName", default, skip_serializing_if = "Option::is_none")]
    pub network_attachment_name: Option<String>,
}
impl NetworkAttachment {
    pub fn new(attached_network_id: String, ip_allocation_method: network_attachment::IpAllocationMethod) -> Self {
        Self {
            attached_network_id,
            default_gateway: None,
            ip_allocation_method,
            ipv4_address: None,
            ipv6_address: None,
            mac_address: None,
            network_attachment_name: None,
        }
    }
}
pub mod network_attachment {
    use super::*;
    #[doc = "The indicator of whether this is the default gateway.\nOnly one of the attached networks (including the CloudServicesNetwork attachment) for a single machine may be specified as True."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DefaultGateway")]
    pub enum DefaultGateway {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DefaultGateway {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DefaultGateway {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DefaultGateway {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("DefaultGateway", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("DefaultGateway", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The IP allocation mechanism for the virtual machine.\nDynamic and Static are only valid for l3Network which may also specify Disabled.\nOtherwise, Disabled is the only permitted value."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IpAllocationMethod")]
    pub enum IpAllocationMethod {
        Dynamic,
        Static,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IpAllocationMethod {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IpAllocationMethod {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IpAllocationMethod {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Dynamic => serializer.serialize_unit_variant("IpAllocationMethod", 0u32, "Dynamic"),
                Self::Static => serializer.serialize_unit_variant("IpAllocationMethod", 1u32, "Static"),
                Self::Disabled => serializer.serialize_unit_variant("IpAllocationMethod", 2u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterface {
    #[doc = "The partial address of Peripheral Component Interconnect (PCI)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "The connection type of the device."]
    #[serde(rename = "deviceConnectionType", default, skip_serializing_if = "Option::is_none")]
    pub device_connection_type: Option<network_interface::DeviceConnectionType>,
    #[doc = "The model name of the device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[doc = "The physical slot for this device."]
    #[serde(rename = "physicalSlot", default, skip_serializing_if = "Option::is_none")]
    pub physical_slot: Option<i64>,
    #[doc = "The number of ports on the device."]
    #[serde(rename = "portCount", default, skip_serializing_if = "Option::is_none")]
    pub port_count: Option<i64>,
    #[doc = "The maximum amount of data in GB that the line card transmits through a port at any given second."]
    #[serde(rename = "portSpeed", default, skip_serializing_if = "Option::is_none")]
    pub port_speed: Option<i64>,
    #[doc = "The vendor name of the device."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
}
impl NetworkInterface {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_interface {
    use super::*;
    #[doc = "The connection type of the device."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DeviceConnectionType")]
    pub enum DeviceConnectionType {
        #[serde(rename = "PCI")]
        Pci,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DeviceConnectionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DeviceConnectionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DeviceConnectionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Pci => serializer.serialize_unit_variant("DeviceConnectionType", 0u32, "PCI"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Nic {
    #[serde(rename = "lldpNeighbor", default, skip_serializing_if = "Option::is_none")]
    pub lldp_neighbor: Option<LldpNeighbor>,
    #[doc = "The MAC address associated with this NIC."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "The name of the NIC/interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl Nic {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Node {
    #[doc = "The resource ID of the bare metal machine that hosts this node."]
    #[serde(rename = "bareMetalMachineId", default, skip_serializing_if = "Option::is_none")]
    pub bare_metal_machine_id: Option<String>,
    #[doc = "The machine image last used to deploy this node."]
    #[serde(rename = "imageId", default, skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[doc = "The list of network attachments to the virtual machine."]
    #[serde(
        rename = "networkAttachments",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub network_attachments: Vec<NetworkAttachment>,
    #[doc = "The name of this node, as realized in the Hybrid AKS cluster."]
    #[serde(rename = "nodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[doc = "The power state (On | Off) of the node."]
    #[serde(rename = "powerState", default, skip_serializing_if = "Option::is_none")]
    pub power_state: Option<node::PowerState>,
}
impl Node {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod node {
    use super::*;
    #[doc = "The power state (On | Off) of the node."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PowerState")]
    pub enum PowerState {
        On,
        Off,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PowerState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PowerState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PowerState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::On => serializer.serialize_unit_variant("PowerState", 0u32, "On"),
                Self::Off => serializer.serialize_unit_variant("PowerState", 1u32, "Off"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NodeConfiguration {
    #[doc = "The resource ID of the agent pool that contains the nodes in this configuration."]
    #[serde(rename = "agentPoolId", default, skip_serializing_if = "Option::is_none")]
    pub agent_pool_id: Option<String>,
    #[doc = "The name of the agent pool that contains the nodes in this configuration."]
    #[serde(rename = "agentPoolName", default, skip_serializing_if = "Option::is_none")]
    pub agent_pool_name: Option<String>,
    #[doc = "The number of CPU cores in the virtual machine."]
    #[serde(rename = "cpuCores", default, skip_serializing_if = "Option::is_none")]
    pub cpu_cores: Option<i64>,
    #[doc = "The root disk size of the virtual machine in GB."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i64>,
    #[doc = "The memory size of the virtual machine in GB."]
    #[serde(rename = "memorySizeGB", default, skip_serializing_if = "Option::is_none")]
    pub memory_size_gb: Option<i64>,
    #[doc = "Field deprecated, use agentPoolName instead. This field will be removed in a future version but will reflect the name of the agent pool that contains the nodes in this configuration."]
    #[serde(rename = "nodePoolName", default, skip_serializing_if = "Option::is_none")]
    pub node_pool_name: Option<String>,
    #[doc = "The list of nodes that utilize this configuration."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub nodes: Vec<Node>,
    #[doc = "The number of virtual machines that use this configuration."]
    #[serde(rename = "vmCount", default, skip_serializing_if = "Option::is_none")]
    pub vm_count: Option<i64>,
    #[doc = "The name of the VM size supplied during the creation of the cluster."]
    #[serde(rename = "vmSize", default, skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
}
impl NodeConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of a REST API operation, returned from the Resource Provider Operations API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation, as per Resource-Based Access Control (RBAC). Examples: \"Microsoft.Compute/virtualMachines/write\", \"Microsoft.Compute/virtualMachines/capture/action\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Whether the operation applies to data-plane. This is \"true\" for data-plane operations and \"false\" for ARM/control-plane operations."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Localized display information for this particular operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<operation::ActionType>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Localized display information for this particular operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The localized friendly form of the resource provider name, e.g. \"Microsoft Monitoring Insights\" or \"Microsoft Compute\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The localized friendly name of the resource type related to this operation. E.g. \"Virtual Machines\" or \"Job Schedule Collections\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The concise, localized friendly name for the operation; suitable for dropdowns. E.g. \"Create or Update Virtual Machine\", \"Restart Virtual Machine\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The short, localized friendly description of the operation; suitable for tool tips and detailed views."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Origin")]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "user,system")]
        UserSystem,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Origin {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Origin {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Origin {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("Origin", 0u32, "user"),
                Self::System => serializer.serialize_unit_variant("Origin", 1u32, "system"),
                Self::UserSystem => serializer.serialize_unit_variant("Origin", 2u32, "user,system"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActionType")]
    pub enum ActionType {
        Internal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Internal => serializer.serialize_unit_variant("ActionType", 0u32, "Internal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of REST API operations supported by an Azure Resource Provider. It contains an URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the resource provider"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results (if there are any)."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OsDisk {
    #[doc = "The strategy for creating the OS disk."]
    #[serde(rename = "createOption", default, skip_serializing_if = "Option::is_none")]
    pub create_option: Option<os_disk::CreateOption>,
    #[doc = "The strategy for deleting the OS disk."]
    #[serde(rename = "deleteOption", default, skip_serializing_if = "Option::is_none")]
    pub delete_option: Option<os_disk::DeleteOption>,
    #[doc = "The size of the disk in gigabytes. Required if the createOption is Ephemeral."]
    #[serde(rename = "diskSizeGB")]
    pub disk_size_gb: i64,
}
impl OsDisk {
    pub fn new(disk_size_gb: i64) -> Self {
        Self {
            create_option: None,
            delete_option: None,
            disk_size_gb,
        }
    }
}
pub mod os_disk {
    use super::*;
    #[doc = "The strategy for creating the OS disk."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreateOption")]
    pub enum CreateOption {
        Ephemeral,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CreateOption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CreateOption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CreateOption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ephemeral => serializer.serialize_unit_variant("CreateOption", 0u32, "Ephemeral"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for CreateOption {
        fn default() -> Self {
            Self::Ephemeral
        }
    }
    #[doc = "The strategy for deleting the OS disk."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DeleteOption")]
    pub enum DeleteOption {
        Delete,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DeleteOption {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DeleteOption {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DeleteOption {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Delete => serializer.serialize_unit_variant("DeleteOption", 0u32, "Delete"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for DeleteOption {
        fn default() -> Self {
            Self::Delete
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rack {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
    pub properties: RackProperties,
}
impl Rack {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocation, properties: RackProperties) -> Self {
        Self {
            tracked_resource,
            extended_location,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RackDefinition {
    #[doc = "The zone name used for this rack when created."]
    #[serde(rename = "availabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[doc = "The unordered list of bare metal machine configuration."]
    #[serde(
        rename = "bareMetalMachineConfigurationData",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub bare_metal_machine_configuration_data: Vec<BareMetalMachineConfigurationData>,
    #[doc = "The resource ID of the network rack that matches this rack definition."]
    #[serde(rename = "networkRackId")]
    pub network_rack_id: String,
    #[doc = "The free-form description of the rack's location."]
    #[serde(rename = "rackLocation", default, skip_serializing_if = "Option::is_none")]
    pub rack_location: Option<String>,
    #[doc = "The unique identifier for the rack within Network Cloud cluster. An alternate unique alphanumeric value other than a serial number may be provided if desired."]
    #[serde(rename = "rackSerialNumber")]
    pub rack_serial_number: String,
    #[doc = "The resource ID of the sku for the rack being added."]
    #[serde(rename = "rackSkuId")]
    pub rack_sku_id: String,
    #[doc = "The list of storage appliance configuration data for this rack."]
    #[serde(
        rename = "storageApplianceConfigurationData",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub storage_appliance_configuration_data: Vec<StorageApplianceConfigurationData>,
}
impl RackDefinition {
    pub fn new(network_rack_id: String, rack_serial_number: String, rack_sku_id: String) -> Self {
        Self {
            availability_zone: None,
            bare_metal_machine_configuration_data: Vec::new(),
            network_rack_id,
            rack_location: None,
            rack_serial_number,
            rack_sku_id,
            storage_appliance_configuration_data: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RackList {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of racks."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Rack>,
}
impl azure_core::Continuable for RackList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RackList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RackPatchParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RacksPatchProperties>,
    #[doc = "The Azure resource tags that will replace the existing ones."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl RackPatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RackProperties {
    #[doc = "The value that will be used for machines in this rack to represent the availability zones that can be referenced by Hybrid AKS Clusters for node arrangement."]
    #[serde(rename = "availabilityZone")]
    pub availability_zone: String,
    #[doc = "The resource ID of the cluster the rack is created for. This value is set when the rack is created by the cluster."]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc = "The more detailed status of the rack."]
    #[serde(rename = "detailedStatus", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<rack_properties::DetailedStatus>,
    #[doc = "The descriptive message about the current detailed status."]
    #[serde(rename = "detailedStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status_message: Option<String>,
    #[doc = "The provisioning state of the rack resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<rack_properties::ProvisioningState>,
    #[doc = "The free-form description of the rack location. (e.g. “DTN Datacenter, Floor 3, Isle 9, Rack 2B”)"]
    #[serde(rename = "rackLocation")]
    pub rack_location: String,
    #[doc = "The unique identifier for the rack within Network Cloud cluster. An alternate unique alphanumeric value other than a serial number may be provided if desired."]
    #[serde(rename = "rackSerialNumber")]
    pub rack_serial_number: String,
    #[doc = "The SKU for the rack."]
    #[serde(rename = "rackSkuId")]
    pub rack_sku_id: String,
}
impl RackProperties {
    pub fn new(availability_zone: String, rack_location: String, rack_serial_number: String, rack_sku_id: String) -> Self {
        Self {
            availability_zone,
            cluster_id: None,
            detailed_status: None,
            detailed_status_message: None,
            provisioning_state: None,
            rack_location,
            rack_serial_number,
            rack_sku_id,
        }
    }
}
pub mod rack_properties {
    use super::*;
    #[doc = "The more detailed status of the rack."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DetailedStatus")]
    pub enum DetailedStatus {
        Error,
        Available,
        Provisioning,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DetailedStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DetailedStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DetailedStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Error => serializer.serialize_unit_variant("DetailedStatus", 0u32, "Error"),
                Self::Available => serializer.serialize_unit_variant("DetailedStatus", 1u32, "Available"),
                Self::Provisioning => serializer.serialize_unit_variant("DetailedStatus", 2u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state of the rack resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Provisioning,
        Accepted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Provisioning"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Accepted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RackSku {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: RackSkuProperties,
}
impl RackSku {
    pub fn new(properties: RackSkuProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RackSkuList {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of Rack SKUs."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<RackSku>,
}
impl azure_core::Continuable for RackSkuList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl RackSkuList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RackSkuProperties {
    #[doc = "The list of machine SKUs and associated rack slot for the compute-dedicated machines in this rack model."]
    #[serde(
        rename = "computeMachines",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub compute_machines: Vec<MachineSkuSlot>,
    #[doc = "The list of machine SKUs and associated rack slot for the control-plane dedicated machines in this rack model."]
    #[serde(
        rename = "controllerMachines",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub controller_machines: Vec<MachineSkuSlot>,
    #[doc = "The free-form text describing the rack."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The maximum number of compute racks supported by an aggregator rack. 0 if this is a compute rack or a rack for a single rack cluster(rackType=\"Single\")."]
    #[serde(rename = "maxClusterSlots", default, skip_serializing_if = "Option::is_none")]
    pub max_cluster_slots: Option<i64>,
    #[doc = "The provisioning state of the rack SKU resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<rack_sku_properties::ProvisioningState>,
    #[doc = "The type of the rack."]
    #[serde(rename = "rackType", default, skip_serializing_if = "Option::is_none")]
    pub rack_type: Option<rack_sku_properties::RackType>,
    #[doc = "The list of appliance SKUs and associated rack slot for the storage appliance(s) in this rack model."]
    #[serde(
        rename = "storageAppliances",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub storage_appliances: Vec<StorageApplianceSkuSlot>,
    #[doc = "The list of supported SKUs if the rack is an aggregator."]
    #[serde(
        rename = "supportedRackSkuIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_rack_sku_ids: Vec<String>,
}
impl RackSkuProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod rack_sku_properties {
    use super::*;
    #[doc = "The provisioning state of the rack SKU resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of the rack."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RackType")]
    pub enum RackType {
        Aggregator,
        Compute,
        Single,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RackType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RackType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RackType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Aggregator => serializer.serialize_unit_variant("RackType", 0u32, "Aggregator"),
                Self::Compute => serializer.serialize_unit_variant("RackType", 1u32, "Compute"),
                Self::Single => serializer.serialize_unit_variant("RackType", 2u32, "Single"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RacksPatchProperties {
    #[doc = "The free-form description of the rack location. (e.g. “DTN Datacenter, Floor 3, Isle 9, Rack 2B”)"]
    #[serde(rename = "rackLocation", default, skip_serializing_if = "Option::is_none")]
    pub rack_location: Option<String>,
    #[doc = "The globally unique identifier for the rack."]
    #[serde(rename = "rackSerialNumber", default, skip_serializing_if = "Option::is_none")]
    pub rack_serial_number: Option<String>,
}
impl RacksPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.Compute/virtualMachines\" or \"Microsoft.Storage/storageAccounts\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServicePrincipalInformation {
    #[doc = "The application ID, also known as client ID, of the service principal."]
    #[serde(rename = "applicationId")]
    pub application_id: String,
    #[doc = "The password of the service principal."]
    pub password: String,
    #[doc = "The principal ID, also known as the object ID, of the service principal."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[doc = "The tenant ID, also known as the directory ID, of the tenant in which the service principal is created."]
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
}
impl ServicePrincipalInformation {
    pub fn new(application_id: String, password: String, principal_id: String, tenant_id: String) -> Self {
        Self {
            application_id,
            password,
            principal_id,
            tenant_id,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SshPublicKey {
    #[doc = "The public ssh key of the user."]
    #[serde(rename = "keyData")]
    pub key_data: String,
}
impl SshPublicKey {
    pub fn new(key_data: String) -> Self {
        Self { key_data }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAppliance {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
    pub properties: StorageApplianceProperties,
}
impl StorageAppliance {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocation, properties: StorageApplianceProperties) -> Self {
        Self {
            tracked_resource,
            extended_location,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageApplianceCommandSpecification {
    #[doc = "The list of string arguments that will be passed to the script in order as separate arguments."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub arguments: Vec<String>,
    #[doc = "The read-only command to execute against the storage appliance."]
    pub command: String,
}
impl StorageApplianceCommandSpecification {
    pub fn new(command: String) -> Self {
        Self {
            arguments: Vec::new(),
            command,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageApplianceConfigurationData {
    #[serde(rename = "adminCredentials")]
    pub admin_credentials: AdministrativeCredentials,
    #[doc = "The slot that storage appliance is in the rack based on the BOM configuration."]
    #[serde(rename = "rackSlot")]
    pub rack_slot: i64,
    #[doc = "The serial number of the appliance."]
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    #[doc = "The user-provided name for the storage appliance that will be created from this specification."]
    #[serde(rename = "storageApplianceName", default, skip_serializing_if = "Option::is_none")]
    pub storage_appliance_name: Option<String>,
}
impl StorageApplianceConfigurationData {
    pub fn new(admin_credentials: AdministrativeCredentials, rack_slot: i64, serial_number: String) -> Self {
        Self {
            admin_credentials,
            rack_slot,
            serial_number,
            storage_appliance_name: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageApplianceEnableRemoteVendorManagementParameters {
    #[doc = "Field Deprecated. This field is not used and will be rejected if provided. The list of IPv4 subnets (in CIDR format), IPv6 subnets (in CIDR format), or hostnames that the storage appliance needs accessible in order to turn on the remote vendor management."]
    #[serde(
        rename = "supportEndpoints",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub support_endpoints: Vec<String>,
}
impl StorageApplianceEnableRemoteVendorManagementParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageApplianceList {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of storage appliances."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<StorageAppliance>,
}
impl azure_core::Continuable for StorageApplianceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl StorageApplianceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAppliancePatchParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageAppliancePatchProperties>,
    #[doc = "The Azure resource tags that will replace the existing ones."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl StorageAppliancePatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageAppliancePatchProperties {
    #[doc = "The serial number for the storage appliance."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
}
impl StorageAppliancePatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageApplianceProperties {
    #[serde(rename = "administratorCredentials")]
    pub administrator_credentials: AdministrativeCredentials,
    #[doc = "The total capacity of the storage appliance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
    #[doc = "The amount of storage consumed."]
    #[serde(rename = "capacityUsed", default, skip_serializing_if = "Option::is_none")]
    pub capacity_used: Option<i64>,
    #[doc = "The resource ID of the cluster this storage appliance is associated with."]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc = "The detailed status of the storage appliance."]
    #[serde(rename = "detailedStatus", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<storage_appliance_properties::DetailedStatus>,
    #[doc = "The descriptive message about the current detailed status."]
    #[serde(rename = "detailedStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status_message: Option<String>,
    #[doc = "The endpoint for the management interface of the storage appliance."]
    #[serde(rename = "managementIpv4Address", default, skip_serializing_if = "Option::is_none")]
    pub management_ipv4_address: Option<String>,
    #[doc = "The provisioning state of the storage appliance."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<storage_appliance_properties::ProvisioningState>,
    #[doc = "The resource ID of the rack where this storage appliance resides."]
    #[serde(rename = "rackId")]
    pub rack_id: String,
    #[doc = "The slot the storage appliance is in the rack based on the BOM configuration."]
    #[serde(rename = "rackSlot")]
    pub rack_slot: i64,
    #[doc = "The indicator of whether the storage appliance supports remote vendor management."]
    #[serde(rename = "remoteVendorManagementFeature", default, skip_serializing_if = "Option::is_none")]
    pub remote_vendor_management_feature: Option<storage_appliance_properties::RemoteVendorManagementFeature>,
    #[doc = "The indicator of whether the remote vendor management feature is enabled or disabled, or unsupported if it is an unsupported feature."]
    #[serde(rename = "remoteVendorManagementStatus", default, skip_serializing_if = "Option::is_none")]
    pub remote_vendor_management_status: Option<storage_appliance_properties::RemoteVendorManagementStatus>,
    #[doc = "The serial number for the storage appliance."]
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    #[doc = "The SKU for the storage appliance."]
    #[serde(rename = "storageApplianceSkuId")]
    pub storage_appliance_sku_id: String,
}
impl StorageApplianceProperties {
    pub fn new(
        administrator_credentials: AdministrativeCredentials,
        rack_id: String,
        rack_slot: i64,
        serial_number: String,
        storage_appliance_sku_id: String,
    ) -> Self {
        Self {
            administrator_credentials,
            capacity: None,
            capacity_used: None,
            cluster_id: None,
            detailed_status: None,
            detailed_status_message: None,
            management_ipv4_address: None,
            provisioning_state: None,
            rack_id,
            rack_slot,
            remote_vendor_management_feature: None,
            remote_vendor_management_status: None,
            serial_number,
            storage_appliance_sku_id,
        }
    }
}
pub mod storage_appliance_properties {
    use super::*;
    #[doc = "The detailed status of the storage appliance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DetailedStatus")]
    pub enum DetailedStatus {
        Error,
        Available,
        Provisioning,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DetailedStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DetailedStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DetailedStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Error => serializer.serialize_unit_variant("DetailedStatus", 0u32, "Error"),
                Self::Available => serializer.serialize_unit_variant("DetailedStatus", 1u32, "Available"),
                Self::Provisioning => serializer.serialize_unit_variant("DetailedStatus", 2u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state of the storage appliance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Provisioning,
        Accepted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Provisioning"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Accepted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The indicator of whether the storage appliance supports remote vendor management."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RemoteVendorManagementFeature")]
    pub enum RemoteVendorManagementFeature {
        Supported,
        Unsupported,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RemoteVendorManagementFeature {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RemoteVendorManagementFeature {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RemoteVendorManagementFeature {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Supported => serializer.serialize_unit_variant("RemoteVendorManagementFeature", 0u32, "Supported"),
                Self::Unsupported => serializer.serialize_unit_variant("RemoteVendorManagementFeature", 1u32, "Unsupported"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The indicator of whether the remote vendor management feature is enabled or disabled, or unsupported if it is an unsupported feature."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RemoteVendorManagementStatus")]
    pub enum RemoteVendorManagementStatus {
        Enabled,
        Disabled,
        Unsupported,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RemoteVendorManagementStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RemoteVendorManagementStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RemoteVendorManagementStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("RemoteVendorManagementStatus", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("RemoteVendorManagementStatus", 1u32, "Disabled"),
                Self::Unsupported => serializer.serialize_unit_variant("RemoteVendorManagementStatus", 2u32, "Unsupported"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageApplianceRunReadCommandsParameters {
    #[doc = "The list of read-only commands to run."]
    pub commands: Vec<StorageApplianceCommandSpecification>,
    #[doc = "The maximum time the commands are allowed to run.\nIf the execution time exceeds the maximum, the script will be stopped, any output produced until then will be captured, and the exit code matching a timeout will be returned (252)."]
    #[serde(rename = "limitTimeSeconds")]
    pub limit_time_seconds: i64,
}
impl StorageApplianceRunReadCommandsParameters {
    pub fn new(commands: Vec<StorageApplianceCommandSpecification>, limit_time_seconds: i64) -> Self {
        Self {
            commands,
            limit_time_seconds,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageApplianceSkuProperties {
    #[doc = "The maximum capacity of the storage appliance."]
    #[serde(rename = "capacityGB", default, skip_serializing_if = "Option::is_none")]
    pub capacity_gb: Option<i64>,
    #[doc = "The model of the storage appliance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}
impl StorageApplianceSkuProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageApplianceSkuSlot {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageApplianceSkuProperties>,
    #[doc = "The position in the rack for the storage appliance."]
    #[serde(rename = "rackSlot", default, skip_serializing_if = "Option::is_none")]
    pub rack_slot: Option<i64>,
}
impl StorageApplianceSkuSlot {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageApplianceValidateHardwareParameters {
    #[doc = "The category of hardware validation to perform."]
    #[serde(rename = "validationCategory")]
    pub validation_category: storage_appliance_validate_hardware_parameters::ValidationCategory,
}
impl StorageApplianceValidateHardwareParameters {
    pub fn new(validation_category: storage_appliance_validate_hardware_parameters::ValidationCategory) -> Self {
        Self { validation_category }
    }
}
pub mod storage_appliance_validate_hardware_parameters {
    use super::*;
    #[doc = "The category of hardware validation to perform."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ValidationCategory")]
    pub enum ValidationCategory {
        BasicValidation,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ValidationCategory {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ValidationCategory {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ValidationCategory {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::BasicValidation => serializer.serialize_unit_variant("ValidationCategory", 0u32, "BasicValidation"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageProfile {
    #[serde(rename = "osDisk")]
    pub os_disk: OsDisk,
    #[doc = "The resource IDs of volumes that are requested to be attached to the virtual machine."]
    #[serde(
        rename = "volumeAttachments",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub volume_attachments: Vec<String>,
}
impl StorageProfile {
    pub fn new(os_disk: OsDisk) -> Self {
        Self {
            os_disk,
            volume_attachments: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsParameter {
    #[doc = "The Azure resource tags that will replace the existing ones."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TagsParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for an Azure Resource Manager tracked top level resource which has 'tags' and a 'location'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives"]
    pub location: String,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
            location,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrunkedNetwork {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
    pub properties: TrunkedNetworkProperties,
}
impl TrunkedNetwork {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocation, properties: TrunkedNetworkProperties) -> Self {
        Self {
            tracked_resource,
            extended_location,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrunkedNetworkList {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of trunked networks."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TrunkedNetwork>,
}
impl azure_core::Continuable for TrunkedNetworkList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TrunkedNetworkList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrunkedNetworkPatchParameters {
    #[doc = "The Azure resource tags that will replace the existing ones."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TrunkedNetworkPatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrunkedNetworkProperties {
    #[doc = "The resource ID of the Network Cloud cluster this trunked network is associated with."]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc = "The more detailed status of the trunked network."]
    #[serde(rename = "detailedStatus", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<trunked_network_properties::DetailedStatus>,
    #[doc = "The descriptive message about the current detailed status."]
    #[serde(rename = "detailedStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status_message: Option<String>,
    #[doc = "The list of Hybrid AKS cluster resource IDs that are associated with this trunked network."]
    #[serde(
        rename = "hybridAksClustersAssociatedIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub hybrid_aks_clusters_associated_ids: Vec<String>,
    #[doc = "The network plugin type for Hybrid AKS."]
    #[serde(rename = "hybridAksPluginType", default, skip_serializing_if = "Option::is_none")]
    pub hybrid_aks_plugin_type: Option<trunked_network_properties::HybridAksPluginType>,
    #[doc = "The default interface name for this trunked network in the virtual machine. This name can be overridden by the name supplied in the network attachment configuration of that virtual machine."]
    #[serde(rename = "interfaceName", default, skip_serializing_if = "Option::is_none")]
    pub interface_name: Option<String>,
    #[doc = "The list of resource IDs representing the Network Fabric isolation domains. It can be any combination of l2IsolationDomain and l3IsolationDomain resources."]
    #[serde(rename = "isolationDomainIds")]
    pub isolation_domain_ids: Vec<String>,
    #[doc = "The provisioning state of the trunked network."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<trunked_network_properties::ProvisioningState>,
    #[doc = "The list of virtual machine resource IDs, excluding any Hybrid AKS virtual machines, that are currently using this trunked network."]
    #[serde(
        rename = "virtualMachinesAssociatedIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub virtual_machines_associated_ids: Vec<String>,
    #[doc = "The list of vlans that are selected from the isolation domains for trunking."]
    pub vlans: Vec<i64>,
}
impl TrunkedNetworkProperties {
    pub fn new(isolation_domain_ids: Vec<String>, vlans: Vec<i64>) -> Self {
        Self {
            cluster_id: None,
            detailed_status: None,
            detailed_status_message: None,
            hybrid_aks_clusters_associated_ids: Vec::new(),
            hybrid_aks_plugin_type: None,
            interface_name: None,
            isolation_domain_ids,
            provisioning_state: None,
            virtual_machines_associated_ids: Vec::new(),
            vlans,
        }
    }
}
pub mod trunked_network_properties {
    use super::*;
    #[doc = "The more detailed status of the trunked network."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DetailedStatus")]
    pub enum DetailedStatus {
        Error,
        Available,
        Provisioning,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DetailedStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DetailedStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DetailedStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Error => serializer.serialize_unit_variant("DetailedStatus", 0u32, "Error"),
                Self::Available => serializer.serialize_unit_variant("DetailedStatus", 1u32, "Available"),
                Self::Provisioning => serializer.serialize_unit_variant("DetailedStatus", 2u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The network plugin type for Hybrid AKS."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HybridAksPluginType")]
    pub enum HybridAksPluginType {
        #[serde(rename = "DPDK")]
        Dpdk,
        #[serde(rename = "SRIOV")]
        Sriov,
        #[serde(rename = "OSDevice")]
        OsDevice,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HybridAksPluginType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HybridAksPluginType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HybridAksPluginType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Dpdk => serializer.serialize_unit_variant("HybridAksPluginType", 0u32, "DPDK"),
                Self::Sriov => serializer.serialize_unit_variant("HybridAksPluginType", 1u32, "SRIOV"),
                Self::OsDevice => serializer.serialize_unit_variant("HybridAksPluginType", 2u32, "OSDevice"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for HybridAksPluginType {
        fn default() -> Self {
            Self::Sriov
        }
    }
    #[doc = "The provisioning state of the trunked network."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Provisioning,
        Accepted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Provisioning"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Accepted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidationThreshold {
    #[doc = "Selection of how the type evaluation is applied to the cluster calculation."]
    pub grouping: validation_threshold::Grouping,
    #[doc = "Selection of how the threshold should be evaluated."]
    #[serde(rename = "type")]
    pub type_: validation_threshold::Type,
    #[doc = "The numeric threshold value."]
    pub value: i64,
}
impl ValidationThreshold {
    pub fn new(grouping: validation_threshold::Grouping, type_: validation_threshold::Type, value: i64) -> Self {
        Self { grouping, type_, value }
    }
}
pub mod validation_threshold {
    use super::*;
    #[doc = "Selection of how the type evaluation is applied to the cluster calculation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Grouping")]
    pub enum Grouping {
        PerCluster,
        PerRack,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Grouping {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Grouping {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Grouping {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PerCluster => serializer.serialize_unit_variant("Grouping", 0u32, "PerCluster"),
                Self::PerRack => serializer.serialize_unit_variant("Grouping", 1u32, "PerRack"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Selection of how the threshold should be evaluated."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        CountSuccess,
        PercentSuccess,
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
                Self::CountSuccess => serializer.serialize_unit_variant("Type", 0u32, "CountSuccess"),
                Self::PercentSuccess => serializer.serialize_unit_variant("Type", 1u32, "PercentSuccess"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachine {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
    pub properties: VirtualMachineProperties,
}
impl VirtualMachine {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocation, properties: VirtualMachineProperties) -> Self {
        Self {
            tracked_resource,
            extended_location,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineList {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of virtual machines."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<VirtualMachine>,
}
impl azure_core::Continuable for VirtualMachineList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VirtualMachineList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachinePatchParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachinePatchProperties>,
    #[doc = "The Azure resource tags that will replace the existing ones."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl VirtualMachinePatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachinePatchProperties {
    #[serde(rename = "vmImageRepositoryCredentials", default, skip_serializing_if = "Option::is_none")]
    pub vm_image_repository_credentials: Option<ImageRepositoryCredentials>,
}
impl VirtualMachinePatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachinePlacementHint {
    #[doc = "The specification of whether this hint supports affinity or anti-affinity with the referenced resources."]
    #[serde(rename = "hintType")]
    pub hint_type: virtual_machine_placement_hint::HintType,
    #[doc = "The resource ID of the target object that the placement hints will be checked against, e.g., the bare metal node to host the virtual machine."]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "The indicator of whether the hint is a hard or soft requirement during scheduling."]
    #[serde(rename = "schedulingExecution")]
    pub scheduling_execution: virtual_machine_placement_hint::SchedulingExecution,
    #[doc = "The scope for the virtual machine affinity or anti-affinity placement hint. It should always be \"Machine\" in the case of node affinity."]
    pub scope: virtual_machine_placement_hint::Scope,
}
impl VirtualMachinePlacementHint {
    pub fn new(
        hint_type: virtual_machine_placement_hint::HintType,
        resource_id: String,
        scheduling_execution: virtual_machine_placement_hint::SchedulingExecution,
        scope: virtual_machine_placement_hint::Scope,
    ) -> Self {
        Self {
            hint_type,
            resource_id,
            scheduling_execution,
            scope,
        }
    }
}
pub mod virtual_machine_placement_hint {
    use super::*;
    #[doc = "The specification of whether this hint supports affinity or anti-affinity with the referenced resources."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HintType")]
    pub enum HintType {
        Affinity,
        AntiAffinity,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HintType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HintType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HintType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Affinity => serializer.serialize_unit_variant("HintType", 0u32, "Affinity"),
                Self::AntiAffinity => serializer.serialize_unit_variant("HintType", 1u32, "AntiAffinity"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The indicator of whether the hint is a hard or soft requirement during scheduling."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SchedulingExecution")]
    pub enum SchedulingExecution {
        Hard,
        Soft,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SchedulingExecution {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SchedulingExecution {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SchedulingExecution {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Hard => serializer.serialize_unit_variant("SchedulingExecution", 0u32, "Hard"),
                Self::Soft => serializer.serialize_unit_variant("SchedulingExecution", 1u32, "Soft"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The scope for the virtual machine affinity or anti-affinity placement hint. It should always be \"Machine\" in the case of node affinity."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Scope")]
    pub enum Scope {
        Rack,
        Machine,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Scope {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Scope {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Scope {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Rack => serializer.serialize_unit_variant("Scope", 0u32, "Rack"),
                Self::Machine => serializer.serialize_unit_variant("Scope", 1u32, "Machine"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachinePowerOffParameters {
    #[doc = "The indicator of whether to skip the graceful OS shutdown and power off the virtual machine immediately."]
    #[serde(rename = "skipShutdown", default, skip_serializing_if = "Option::is_none")]
    pub skip_shutdown: Option<virtual_machine_power_off_parameters::SkipShutdown>,
}
impl VirtualMachinePowerOffParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_machine_power_off_parameters {
    use super::*;
    #[doc = "The indicator of whether to skip the graceful OS shutdown and power off the virtual machine immediately."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SkipShutdown")]
    pub enum SkipShutdown {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SkipShutdown {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SkipShutdown {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SkipShutdown {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("SkipShutdown", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("SkipShutdown", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for SkipShutdown {
        fn default() -> Self {
            Self::False
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineProperties {
    #[doc = "The name of the administrator to which the ssh public keys will be added into the authorized keys."]
    #[serde(rename = "adminUsername")]
    pub admin_username: String,
    #[doc = "The resource ID of the bare metal machine the virtual machine has landed to."]
    #[serde(rename = "bareMetalMachineId", default, skip_serializing_if = "Option::is_none")]
    pub bare_metal_machine_id: Option<String>,
    #[doc = "Selects the boot method for the virtual machine."]
    #[serde(rename = "bootMethod", default, skip_serializing_if = "Option::is_none")]
    pub boot_method: Option<virtual_machine_properties::BootMethod>,
    #[serde(rename = "cloudServicesNetworkAttachment")]
    pub cloud_services_network_attachment: NetworkAttachment,
    #[doc = "The resource ID of the cluster the virtual machine is created for."]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc = "The number of CPU cores in the virtual machine."]
    #[serde(rename = "cpuCores")]
    pub cpu_cores: i64,
    #[doc = "The more detailed status of the virtual machine."]
    #[serde(rename = "detailedStatus", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<virtual_machine_properties::DetailedStatus>,
    #[doc = "The descriptive message about the current detailed status."]
    #[serde(rename = "detailedStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status_message: Option<String>,
    #[doc = "Field Deprecated, the value will be ignored if provided. The indicator of whether one of the specified CPU cores is isolated to run the emulator thread for this virtual machine."]
    #[serde(rename = "isolateEmulatorThread", default, skip_serializing_if = "Option::is_none")]
    pub isolate_emulator_thread: Option<virtual_machine_properties::IsolateEmulatorThread>,
    #[doc = "The memory size of the virtual machine in GB."]
    #[serde(rename = "memorySizeGB")]
    pub memory_size_gb: i64,
    #[doc = "The list of network attachments to the virtual machine."]
    #[serde(
        rename = "networkAttachments",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub network_attachments: Vec<NetworkAttachment>,
    #[doc = "The Base64 encoded cloud-init network data."]
    #[serde(rename = "networkData", default, skip_serializing_if = "Option::is_none")]
    pub network_data: Option<String>,
    #[doc = "The scheduling hints for the virtual machine."]
    #[serde(
        rename = "placementHints",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub placement_hints: Vec<VirtualMachinePlacementHint>,
    #[doc = "The power state of the virtual machine."]
    #[serde(rename = "powerState", default, skip_serializing_if = "Option::is_none")]
    pub power_state: Option<virtual_machine_properties::PowerState>,
    #[doc = "The provisioning state of the virtual machine."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<virtual_machine_properties::ProvisioningState>,
    #[doc = "The list of ssh public keys. Each key will be added to the virtual machine using the cloud-init ssh_authorized_keys mechanism for the adminUsername."]
    #[serde(
        rename = "sshPublicKeys",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ssh_public_keys: Vec<SshPublicKey>,
    #[serde(rename = "storageProfile")]
    pub storage_profile: StorageProfile,
    #[doc = "The Base64 encoded cloud-init user data."]
    #[serde(rename = "userData", default, skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
    #[doc = "Field Deprecated, use virtualizationModel instead. The type of the virtio interface."]
    #[serde(rename = "virtioInterface", default, skip_serializing_if = "Option::is_none")]
    pub virtio_interface: Option<virtual_machine_properties::VirtioInterface>,
    #[doc = "The type of the device model to use."]
    #[serde(rename = "vmDeviceModel", default, skip_serializing_if = "Option::is_none")]
    pub vm_device_model: Option<virtual_machine_properties::VmDeviceModel>,
    #[doc = "The virtual machine image that is currently provisioned to the OS disk, using the full url and tag notation used to pull the image."]
    #[serde(rename = "vmImage")]
    pub vm_image: String,
    #[serde(rename = "vmImageRepositoryCredentials", default, skip_serializing_if = "Option::is_none")]
    pub vm_image_repository_credentials: Option<ImageRepositoryCredentials>,
    #[doc = "The resource IDs of volumes that are attached to the virtual machine."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub volumes: Vec<String>,
}
impl VirtualMachineProperties {
    pub fn new(
        admin_username: String,
        cloud_services_network_attachment: NetworkAttachment,
        cpu_cores: i64,
        memory_size_gb: i64,
        storage_profile: StorageProfile,
        vm_image: String,
    ) -> Self {
        Self {
            admin_username,
            bare_metal_machine_id: None,
            boot_method: None,
            cloud_services_network_attachment,
            cluster_id: None,
            cpu_cores,
            detailed_status: None,
            detailed_status_message: None,
            isolate_emulator_thread: None,
            memory_size_gb,
            network_attachments: Vec::new(),
            network_data: None,
            placement_hints: Vec::new(),
            power_state: None,
            provisioning_state: None,
            ssh_public_keys: Vec::new(),
            storage_profile,
            user_data: None,
            virtio_interface: None,
            vm_device_model: None,
            vm_image,
            vm_image_repository_credentials: None,
            volumes: Vec::new(),
        }
    }
}
pub mod virtual_machine_properties {
    use super::*;
    #[doc = "Selects the boot method for the virtual machine."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BootMethod")]
    pub enum BootMethod {
        #[serde(rename = "UEFI")]
        Uefi,
        #[serde(rename = "BIOS")]
        Bios,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BootMethod {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BootMethod {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BootMethod {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Uefi => serializer.serialize_unit_variant("BootMethod", 0u32, "UEFI"),
                Self::Bios => serializer.serialize_unit_variant("BootMethod", 1u32, "BIOS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for BootMethod {
        fn default() -> Self {
            Self::Uefi
        }
    }
    #[doc = "The more detailed status of the virtual machine."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DetailedStatus")]
    pub enum DetailedStatus {
        Error,
        Available,
        Provisioning,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DetailedStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DetailedStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DetailedStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Error => serializer.serialize_unit_variant("DetailedStatus", 0u32, "Error"),
                Self::Available => serializer.serialize_unit_variant("DetailedStatus", 1u32, "Available"),
                Self::Provisioning => serializer.serialize_unit_variant("DetailedStatus", 2u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Field Deprecated, the value will be ignored if provided. The indicator of whether one of the specified CPU cores is isolated to run the emulator thread for this virtual machine."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IsolateEmulatorThread")]
    pub enum IsolateEmulatorThread {
        True,
        False,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IsolateEmulatorThread {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IsolateEmulatorThread {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IsolateEmulatorThread {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::True => serializer.serialize_unit_variant("IsolateEmulatorThread", 0u32, "True"),
                Self::False => serializer.serialize_unit_variant("IsolateEmulatorThread", 1u32, "False"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for IsolateEmulatorThread {
        fn default() -> Self {
            Self::True
        }
    }
    #[doc = "The power state of the virtual machine."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PowerState")]
    pub enum PowerState {
        On,
        Off,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PowerState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PowerState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PowerState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::On => serializer.serialize_unit_variant("PowerState", 0u32, "On"),
                Self::Off => serializer.serialize_unit_variant("PowerState", 1u32, "Off"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state of the virtual machine."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Provisioning,
        Accepted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Provisioning"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Accepted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Field Deprecated, use virtualizationModel instead. The type of the virtio interface."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "VirtioInterface")]
    pub enum VirtioInterface {
        Modern,
        Transitional,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for VirtioInterface {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for VirtioInterface {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for VirtioInterface {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Modern => serializer.serialize_unit_variant("VirtioInterface", 0u32, "Modern"),
                Self::Transitional => serializer.serialize_unit_variant("VirtioInterface", 1u32, "Transitional"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for VirtioInterface {
        fn default() -> Self {
            Self::Modern
        }
    }
    #[doc = "The type of the device model to use."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "VmDeviceModel")]
    pub enum VmDeviceModel {
        T1,
        T2,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for VmDeviceModel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for VmDeviceModel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for VmDeviceModel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::T1 => serializer.serialize_unit_variant("VmDeviceModel", 0u32, "T1"),
                Self::T2 => serializer.serialize_unit_variant("VmDeviceModel", 1u32, "T2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for VmDeviceModel {
        fn default() -> Self {
            Self::T2
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineVolumeParameters {
    #[doc = "The resource ID of the volume."]
    #[serde(rename = "volumeId")]
    pub volume_id: String,
}
impl VirtualMachineVolumeParameters {
    pub fn new(volume_id: String) -> Self {
        Self { volume_id }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
    pub properties: VolumeProperties,
}
impl Volume {
    pub fn new(tracked_resource: TrackedResource, extended_location: ExtendedLocation, properties: VolumeProperties) -> Self {
        Self {
            tracked_resource,
            extended_location,
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeList {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of volumes."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Volume>,
}
impl azure_core::Continuable for VolumeList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VolumeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumePatchParameters {
    #[doc = "The Azure resource tags that will replace the existing ones."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl VolumePatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeProperties {
    #[doc = "The list of resource IDs that attach the volume. It may include virtual machines and Hybrid AKS clusters."]
    #[serde(
        rename = "attachedTo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub attached_to: Vec<String>,
    #[doc = "The more detailed status of the volume."]
    #[serde(rename = "detailedStatus", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<volume_properties::DetailedStatus>,
    #[doc = "The descriptive message about the current detailed status."]
    #[serde(rename = "detailedStatusMessage", default, skip_serializing_if = "Option::is_none")]
    pub detailed_status_message: Option<String>,
    #[doc = "The provisioning state of the volume."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<volume_properties::ProvisioningState>,
    #[doc = "The unique identifier of the volume."]
    #[serde(rename = "serialNumber", default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[doc = "The size of the allocation for this volume in Mebibytes."]
    #[serde(rename = "sizeMiB")]
    pub size_mi_b: i64,
}
impl VolumeProperties {
    pub fn new(size_mi_b: i64) -> Self {
        Self {
            attached_to: Vec::new(),
            detailed_status: None,
            detailed_status_message: None,
            provisioning_state: None,
            serial_number: None,
            size_mi_b,
        }
    }
}
pub mod volume_properties {
    use super::*;
    #[doc = "The more detailed status of the volume."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DetailedStatus")]
    pub enum DetailedStatus {
        Error,
        Active,
        Provisioning,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DetailedStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DetailedStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DetailedStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Error => serializer.serialize_unit_variant("DetailedStatus", 0u32, "Error"),
                Self::Active => serializer.serialize_unit_variant("DetailedStatus", 1u32, "Active"),
                Self::Provisioning => serializer.serialize_unit_variant("DetailedStatus", 2u32, "Provisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state of the volume."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        Provisioning,
        Accepted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Provisioning"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Accepted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Metadata pertaining to creation and last modification of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_data {
    use super::*;
    #[doc = "The type of identity that created the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreatedByType")]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CreatedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CreatedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CreatedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("CreatedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("CreatedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("CreatedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("CreatedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of identity that last modified the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LastModifiedByType")]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LastModifiedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LastModifiedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LastModifiedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("LastModifiedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("LastModifiedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("LastModifiedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("LastModifiedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
