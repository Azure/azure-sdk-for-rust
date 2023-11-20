#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "AzureBareMetal instance info on Azure (ARM properties and AzureBareMetal properties)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBareMetalInstance {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Describes the properties of an Azure Bare Metal Instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureBareMetalInstanceProperties>,
}
impl AzureBareMetalInstance {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Describes the properties of an Azure Bare Metal Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureBareMetalInstanceProperties {
    #[doc = "Specifies the hardware settings for the Azure Bare Metal Instance."]
    #[serde(rename = "hardwareProfile", default, skip_serializing_if = "Option::is_none")]
    pub hardware_profile: Option<HardwareProfile>,
    #[doc = "Specifies the storage settings for the Azure Bare Metal instance disks."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[doc = "Specifies the operating system settings for the Azure Bare Metal instance."]
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,
    #[doc = "Specifies the network settings for the Azure Bare Metal Instance disks."]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
    #[doc = "Specifies the Azure Bare Metal Instance unique ID."]
    #[serde(rename = "azureBareMetalInstanceId", default, skip_serializing_if = "Option::is_none")]
    pub azure_bare_metal_instance_id: Option<String>,
    #[doc = "Resource power state"]
    #[serde(rename = "powerState", default, skip_serializing_if = "Option::is_none")]
    pub power_state: Option<azure_bare_metal_instance_properties::PowerState>,
    #[doc = "Resource proximity placement group"]
    #[serde(rename = "proximityPlacementGroup", default, skip_serializing_if = "Option::is_none")]
    pub proximity_placement_group: Option<String>,
    #[doc = "Hardware revision of an Azure Bare Metal Instance"]
    #[serde(rename = "hwRevision", default, skip_serializing_if = "Option::is_none")]
    pub hw_revision: Option<String>,
    #[doc = "ARM ID of another AzureBareMetalInstance that will share a network with this AzureBareMetalInstance"]
    #[serde(rename = "partnerNodeId", default, skip_serializing_if = "Option::is_none")]
    pub partner_node_id: Option<String>,
    #[doc = "State of provisioning of the AzureBareMetalInstance"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<azure_bare_metal_instance_properties::ProvisioningState>,
}
impl AzureBareMetalInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod azure_bare_metal_instance_properties {
    use super::*;
    #[doc = "Resource power state"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PowerState")]
    pub enum PowerState {
        #[serde(rename = "starting")]
        Starting,
        #[serde(rename = "started")]
        Started,
        #[serde(rename = "stopping")]
        Stopping,
        #[serde(rename = "stopped")]
        Stopped,
        #[serde(rename = "restarting")]
        Restarting,
        #[serde(rename = "unknown")]
        Unknown,
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
                Self::Starting => serializer.serialize_unit_variant("PowerState", 0u32, "starting"),
                Self::Started => serializer.serialize_unit_variant("PowerState", 1u32, "started"),
                Self::Stopping => serializer.serialize_unit_variant("PowerState", 2u32, "stopping"),
                Self::Stopped => serializer.serialize_unit_variant("PowerState", 3u32, "stopped"),
                Self::Restarting => serializer.serialize_unit_variant("PowerState", 4u32, "restarting"),
                Self::Unknown => serializer.serialize_unit_variant("PowerState", 5u32, "unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "State of provisioning of the AzureBareMetalInstance"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Accepted,
        Creating,
        Updating,
        Failed,
        Succeeded,
        Deleting,
        Migrating,
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
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Accepted"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Updating"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Succeeded"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
                Self::Migrating => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Migrating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The response from the List Azure Bare Metal Instances operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureBareMetalInstancesListResult {
    #[doc = "The list of Azure Bare Metal Instances."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AzureBareMetalInstance>,
    #[doc = "The URL to get the next set of Azure Bare Metal Instances."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AzureBareMetalInstancesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AzureBareMetalInstancesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AzureBareMetalStorageInstance info on Azure (ARM properties and AzureBareMetalStorage properties)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureBareMetalStorageInstance {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Describes the properties of an AzureBareMetalStorageInstance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AzureBareMetalStorageInstanceProperties>,
}
impl AzureBareMetalStorageInstance {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Describes the properties of an AzureBareMetalStorageInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureBareMetalStorageInstanceProperties {
    #[doc = "Specifies the AzureBareMetaStorageInstance unique ID."]
    #[serde(
        rename = "azureBareMetalStorageInstanceUniqueIdentifier",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub azure_bare_metal_storage_instance_unique_identifier: Option<String>,
    #[doc = "described the storage properties of the azure bare metal storage instance"]
    #[serde(rename = "storageProperties", default, skip_serializing_if = "Option::is_none")]
    pub storage_properties: Option<StorageProperties>,
}
impl AzureBareMetalStorageInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response from the Get AzureBareMetalStorageInstances operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureBareMetalStorageInstancesListResult {
    #[doc = "The list of AzureBareMetalStorage instances."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AzureBareMetalStorageInstance>,
    #[doc = "The URL to get the next set of AzureBareMetalStorage instances."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AzureBareMetalStorageInstancesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AzureBareMetalStorageInstancesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the disk information fo the Azure Bare Metal Instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Disk {
    #[doc = "The disk name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the size of an empty data disk in gigabytes."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[doc = "Specifies the logical unit number of the data disk. This value is used to identify data disks within the VM and therefore must be unique for each data disk attached to a VM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
}
impl Disk {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "The active state empowers the server with the ability to forcefully terminate and halt any existing processes that may be running on the server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ForceState {
    #[doc = "Whether to force restart by shutting all processes."]
    #[serde(rename = "forceState", default, skip_serializing_if = "Option::is_none")]
    pub force_state: Option<force_state::ForceState>,
}
impl ForceState {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod force_state {
    use super::*;
    #[doc = "Whether to force restart by shutting all processes."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ForceState")]
    pub enum ForceState {
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "inactive")]
        Inactive,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ForceState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ForceState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ForceState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Active => serializer.serialize_unit_variant("ForceState", 0u32, "active"),
                Self::Inactive => serializer.serialize_unit_variant("ForceState", 1u32, "inactive"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies the hardware settings for the Azure Bare Metal Instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HardwareProfile {
    #[doc = "Name of the hardware type (vendor and/or their product name)"]
    #[serde(rename = "hardwareType", default, skip_serializing_if = "Option::is_none")]
    pub hardware_type: Option<hardware_profile::HardwareType>,
    #[doc = "Specifies the Azure Bare Metal Instance SKU."]
    #[serde(rename = "azureBareMetalInstanceSize", default, skip_serializing_if = "Option::is_none")]
    pub azure_bare_metal_instance_size: Option<hardware_profile::AzureBareMetalInstanceSize>,
}
impl HardwareProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod hardware_profile {
    use super::*;
    #[doc = "Name of the hardware type (vendor and/or their product name)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HardwareType")]
    pub enum HardwareType {
        #[serde(rename = "Cisco_UCS")]
        CiscoUcs,
        #[serde(rename = "HPE")]
        Hpe,
        #[serde(rename = "SDFLEX")]
        Sdflex,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for HardwareType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HardwareType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HardwareType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::CiscoUcs => serializer.serialize_unit_variant("HardwareType", 0u32, "Cisco_UCS"),
                Self::Hpe => serializer.serialize_unit_variant("HardwareType", 1u32, "HPE"),
                Self::Sdflex => serializer.serialize_unit_variant("HardwareType", 2u32, "SDFLEX"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies the Azure Bare Metal Instance SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AzureBareMetalInstanceSize")]
    pub enum AzureBareMetalInstanceSize {
        S72m,
        S144m,
        S72,
        S144,
        S192,
        S192m,
        S192xm,
        S96,
        S112,
        S224,
        S224m,
        S224om,
        S224oo,
        S224oom,
        S224ooo,
        S384,
        S384m,
        S384xm,
        S384xxm,
        S448,
        S448m,
        S448om,
        S448oo,
        S448oom,
        S448ooo,
        S448se,
        S576m,
        S576xm,
        S672,
        S672m,
        S672om,
        S672oo,
        S672oom,
        S672ooo,
        S768,
        S768m,
        S768xm,
        S896,
        S896m,
        S896om,
        S896oo,
        S896oom,
        S896ooo,
        S960m,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AzureBareMetalInstanceSize {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AzureBareMetalInstanceSize {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AzureBareMetalInstanceSize {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::S72m => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 0u32, "S72m"),
                Self::S144m => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 1u32, "S144m"),
                Self::S72 => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 2u32, "S72"),
                Self::S144 => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 3u32, "S144"),
                Self::S192 => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 4u32, "S192"),
                Self::S192m => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 5u32, "S192m"),
                Self::S192xm => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 6u32, "S192xm"),
                Self::S96 => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 7u32, "S96"),
                Self::S112 => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 8u32, "S112"),
                Self::S224 => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 9u32, "S224"),
                Self::S224m => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 10u32, "S224m"),
                Self::S224om => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 11u32, "S224om"),
                Self::S224oo => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 12u32, "S224oo"),
                Self::S224oom => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 13u32, "S224oom"),
                Self::S224ooo => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 14u32, "S224ooo"),
                Self::S384 => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 15u32, "S384"),
                Self::S384m => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 16u32, "S384m"),
                Self::S384xm => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 17u32, "S384xm"),
                Self::S384xxm => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 18u32, "S384xxm"),
                Self::S448 => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 19u32, "S448"),
                Self::S448m => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 20u32, "S448m"),
                Self::S448om => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 21u32, "S448om"),
                Self::S448oo => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 22u32, "S448oo"),
                Self::S448oom => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 23u32, "S448oom"),
                Self::S448ooo => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 24u32, "S448ooo"),
                Self::S448se => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 25u32, "S448se"),
                Self::S576m => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 26u32, "S576m"),
                Self::S576xm => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 27u32, "S576xm"),
                Self::S672 => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 28u32, "S672"),
                Self::S672m => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 29u32, "S672m"),
                Self::S672om => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 30u32, "S672om"),
                Self::S672oo => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 31u32, "S672oo"),
                Self::S672oom => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 32u32, "S672oom"),
                Self::S672ooo => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 33u32, "S672ooo"),
                Self::S768 => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 34u32, "S768"),
                Self::S768m => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 35u32, "S768m"),
                Self::S768xm => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 36u32, "S768xm"),
                Self::S896 => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 37u32, "S896"),
                Self::S896m => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 38u32, "S896m"),
                Self::S896om => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 39u32, "S896om"),
                Self::S896oo => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 40u32, "S896oo"),
                Self::S896oom => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 41u32, "S896oom"),
                Self::S896ooo => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 42u32, "S896ooo"),
                Self::S960m => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 43u32, "S960m"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies the network interfaces of a bare metal resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterface {
    #[doc = "Specifies the IP address of the network interface."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}
impl NetworkInterface {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the network settings for the Azure Bare Metal Instance disks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkProfile {
    #[doc = "Specifies the network interfaces for the Azure Bare Metal Instance."]
    #[serde(
        rename = "networkInterfaces",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub network_interfaces: Vec<NetworkInterface>,
    #[doc = "Specifies the circuit id for connecting to express route."]
    #[serde(rename = "circuitId", default, skip_serializing_if = "Option::is_none")]
    pub circuit_id: Option<String>,
}
impl NetworkProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the operating system settings for the Azure Bare Metal instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsProfile {
    #[doc = "Specifies the host OS name of the Azure Bare Metal instance."]
    #[serde(rename = "computerName", default, skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    #[doc = "This property allows you to specify the type of the OS."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    #[doc = "Specifies version of operating system."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Specifies the SSH public key used to access the operating system."]
    #[serde(rename = "sshPublicKey", default, skip_serializing_if = "Option::is_none")]
    pub ssh_public_key: Option<String>,
}
impl OsProfile {
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
        None
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The OperationStatus object returns the state of an asynchronous operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatus {
    #[doc = "Unique Operation Status Identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<operation_status::Status>,
    #[doc = "Start Time when the operation was initially executed."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "An error from the Azure Bare Metal Infrastructure service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<operation_status::Error>,
}
impl OperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation_status {
    use super::*;
    #[doc = "Status of the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Requesting,
        Executing,
        Succeeded,
        Failed,
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
                Self::Requesting => serializer.serialize_unit_variant("Status", 0u32, "Requesting"),
                Self::Executing => serializer.serialize_unit_variant("Status", 1u32, "Executing"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "An error from the Azure Bare Metal Infrastructure service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[doc = "Server-defined set of error codes."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "Human-readable representation of the error."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. E.g. \"/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}\""]
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
#[doc = "Describes the billing related details of the AzureBareMetalStorageInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageBillingProperties {
    #[doc = "the billing mode for the storage instance"]
    #[serde(rename = "billingMode", default, skip_serializing_if = "Option::is_none")]
    pub billing_mode: Option<String>,
    #[doc = "the SKU type that is provisioned"]
    #[serde(rename = "azureBareMetalStorageInstanceSize", default, skip_serializing_if = "Option::is_none")]
    pub azure_bare_metal_storage_instance_size: Option<String>,
}
impl StorageBillingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the storage settings for the Azure Bare Metal instance disks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageProfile {
    #[doc = "IP Address to connect to storage."]
    #[serde(rename = "nfsIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub nfs_ip_address: Option<String>,
    #[doc = "Specifies information about the operating system disk used by bare metal instance."]
    #[serde(
        rename = "osDisks",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub os_disks: Vec<Disk>,
}
impl StorageProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "described the storage properties of the azure bare metal storage instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageProperties {
    #[doc = "State of provisioning of the AzureBareMetalStorageInstance"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<storage_properties::ProvisioningState>,
    #[doc = "the offering type for which the resource is getting provisioned"]
    #[serde(rename = "offeringType", default, skip_serializing_if = "Option::is_none")]
    pub offering_type: Option<String>,
    #[doc = "the storage protocol for which the resource is getting provisioned"]
    #[serde(rename = "storageType", default, skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[doc = "the kind of storage instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,
    #[doc = "the hardware type of the storage instance"]
    #[serde(rename = "hardwareType", default, skip_serializing_if = "Option::is_none")]
    pub hardware_type: Option<String>,
    #[doc = "the workload for which the resource is getting provisioned"]
    #[serde(rename = "workloadType", default, skip_serializing_if = "Option::is_none")]
    pub workload_type: Option<String>,
    #[doc = "Describes the billing related details of the AzureBareMetalStorageInstance."]
    #[serde(rename = "storageBillingProperties", default, skip_serializing_if = "Option::is_none")]
    pub storage_billing_properties: Option<StorageBillingProperties>,
}
impl StorageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_properties {
    use super::*;
    #[doc = "State of provisioning of the AzureBareMetalStorageInstance"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Accepted,
        Creating,
        Updating,
        Failed,
        Succeeded,
        Deleting,
        Canceled,
        Migrating,
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
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Accepted"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Updating"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Succeeded"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Canceled"),
                Self::Migrating => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Migrating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Tags field of the AzureBareMetal/AzureBareMetaStorage instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tags {
    #[doc = "Tags field of the AzureBareMetal/AzureBareMetaStorage instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Tags {
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
