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
    #[doc = "Describes the properties of an AzureBareMetal instance."]
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
#[doc = "Describes the properties of an AzureBareMetal instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureBareMetalInstanceProperties {
    #[doc = "Specifies the hardware settings for the AzureBareMetal instance."]
    #[serde(rename = "hardwareProfile", default, skip_serializing_if = "Option::is_none")]
    pub hardware_profile: Option<HardwareProfile>,
    #[doc = "Specifies the storage settings for the AzureBareMetal instance disks."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[doc = "Specifies the operating system settings for the AzureBareMetal instance."]
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,
    #[doc = "Specifies the network settings for the AzureBareMetal instance disks."]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
    #[doc = "Specifies the AzureBareMetal instance unique ID."]
    #[serde(rename = "azureBareMetalInstanceId", default, skip_serializing_if = "Option::is_none")]
    pub azure_bare_metal_instance_id: Option<String>,
    #[doc = "Resource power state"]
    #[serde(rename = "powerState", default, skip_serializing_if = "Option::is_none")]
    pub power_state: Option<azure_bare_metal_instance_properties::PowerState>,
    #[doc = "Resource proximity placement group"]
    #[serde(rename = "proximityPlacementGroup", default, skip_serializing_if = "Option::is_none")]
    pub proximity_placement_group: Option<String>,
    #[doc = "Hardware revision of an AzureBareMetal instance"]
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
#[doc = "The response from the List AzureBareMetal Instances operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureBareMetalInstancesListResult {
    #[doc = "The list of Azure BareMetal instances."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AzureBareMetalInstance>,
    #[doc = "The URL to get the next set of AzureBareMetal instances."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AzureBareMetalInstancesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AzureBareMetalInstancesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the disk information fo the AzureBareMetal instance"]
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
#[doc = "Detailed BareMetal operation information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Display {
    #[doc = "The localized friendly form of the resource provider name. This form is also expected to include the publisher/company responsible. Use Title Casing. Begin with \"Microsoft\" for 1st party services."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The localized friendly form of the resource type related to this action/operation. This form should match the public documentation for the resource provider. Use Title Casing. For examples, refer to the “name” section."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The localized friendly name for the operation as shown to the user. This name should be concise (to fit in drop downs), but clear (self-documenting). Use Title Casing and include the entity/resource to which it applies."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The localized friendly description for the operation as shown to the user. This description should be thorough, yet concise. It will be used in tool-tips and detailed views."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The intended executor of the operation; governs the display of the operation in the RBAC UX and the audit logs UX. Default value is 'user,system'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl Display {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDefinition {
    #[doc = "Service specific error code which serves as the substatus for the HTTP error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Description of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Internal error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDefinition>,
}
impl ErrorDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
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
#[doc = "Specifies the hardware settings for the AzureBareMetal instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HardwareProfile {
    #[doc = "Name of the hardware type (vendor and/or their product name)"]
    #[serde(rename = "hardwareType", default, skip_serializing_if = "Option::is_none")]
    pub hardware_type: Option<hardware_profile::HardwareType>,
    #[doc = "Specifies the AzureBareMetal instance SKU."]
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Specifies the AzureBareMetal instance SKU."]
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
                Self::S576m => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 25u32, "S576m"),
                Self::S576xm => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 26u32, "S576xm"),
                Self::S672 => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 27u32, "S672"),
                Self::S672m => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 28u32, "S672m"),
                Self::S672om => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 29u32, "S672om"),
                Self::S672oo => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 30u32, "S672oo"),
                Self::S672oom => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 31u32, "S672oom"),
                Self::S672ooo => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 32u32, "S672ooo"),
                Self::S768 => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 33u32, "S768"),
                Self::S768m => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 34u32, "S768m"),
                Self::S768xm => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 35u32, "S768xm"),
                Self::S896 => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 36u32, "S896"),
                Self::S896m => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 37u32, "S896m"),
                Self::S896om => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 38u32, "S896om"),
                Self::S896oo => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 39u32, "S896oo"),
                Self::S896oom => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 40u32, "S896oom"),
                Self::S896ooo => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 41u32, "S896ooo"),
                Self::S960m => serializer.serialize_unit_variant("AzureBareMetalInstanceSize", 42u32, "S960m"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies the IP address of the network interface."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpAddress {
    #[doc = "Specifies the IP address of the network interface."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}
impl IpAddress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the network settings for the AzureBareMetal instance disks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkProfile {
    #[doc = "Specifies the network interfaces for the AzureBareMetal instance."]
    #[serde(rename = "networkInterfaces", default, skip_serializing_if = "Vec::is_empty")]
    pub network_interfaces: Vec<IpAddress>,
    #[doc = "Specifies the circuit id for connecting to express route."]
    #[serde(rename = "circuitId", default, skip_serializing_if = "Option::is_none")]
    pub circuit_id: Option<String>,
}
impl NetworkProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the operating system settings for the AzureBareMetal instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsProfile {
    #[doc = "Specifies the host OS name of the AzureBareMetal instance."]
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
#[doc = "AzureBareMetal operation information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation being performed on this particular object. This name should match the action name that appears in RBAC / the event service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Detailed BareMetal operation information"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Display>,
    #[doc = "indicates whether an operation is a data action or not."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of AzureBareMetal operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[doc = "List of AzureBareMetal operations"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl OperationList {
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
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sample result definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Result {
    #[doc = "Sample property of type string"]
    #[serde(rename = "sampleProperty", default, skip_serializing_if = "Option::is_none")]
    pub sample_property: Option<String>,
}
impl Result {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the storage settings for the AzureBareMetal instance disks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageProfile {
    #[doc = "IP Address to connect to storage."]
    #[serde(rename = "nfsIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub nfs_ip_address: Option<String>,
    #[doc = "Specifies information about the operating system disk used by baremetal instance."]
    #[serde(rename = "osDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub os_disks: Vec<Disk>,
}
impl StorageProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tags field of the AzureBareMetal instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tags {
    #[doc = "Tags field of the AzureBareMetal instance."]
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
