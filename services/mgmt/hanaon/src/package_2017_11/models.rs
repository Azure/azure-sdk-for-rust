#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Specifies the disk information for the HANA instance"]
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
#[doc = "Detailed HANA operation information"]
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
#[doc = "Describes the format of Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Describes the error object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<error_response::Error>,
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
pub mod error_response {
    use super::*;
    #[doc = "Describes the error object."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[doc = "Error code"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "Error message indicating why the operation failed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "HANA instance info on Azure (ARM properties and HANA properties)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HanaInstance {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes the properties of a HANA instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HanaInstanceProperties>,
}
impl HanaInstance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a HANA instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HanaInstanceProperties {
    #[doc = "Specifies the hardware settings for the HANA instance."]
    #[serde(rename = "hardwareProfile", default, skip_serializing_if = "Option::is_none")]
    pub hardware_profile: Option<HardwareProfile>,
    #[doc = "Specifies the storage settings for the HANA instance disks."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[doc = "Specifies the operating system settings for the HANA instance."]
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,
    #[doc = "Specifies the network settings for the HANA instance disks."]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
    #[doc = "Specifies the HANA instance unique ID."]
    #[serde(rename = "hanaInstanceId", default, skip_serializing_if = "Option::is_none")]
    pub hana_instance_id: Option<String>,
    #[doc = "Resource power state"]
    #[serde(rename = "powerState", default, skip_serializing_if = "Option::is_none")]
    pub power_state: Option<hana_instance_properties::PowerState>,
    #[doc = "Resource proximity placement group"]
    #[serde(rename = "proximityPlacementGroup", default, skip_serializing_if = "Option::is_none")]
    pub proximity_placement_group: Option<String>,
    #[doc = "Hardware revision of a HANA instance"]
    #[serde(rename = "hwRevision", default, skip_serializing_if = "Option::is_none")]
    pub hw_revision: Option<String>,
    #[doc = "ARM ID of another HanaInstance that will share a network with this HanaInstance"]
    #[serde(rename = "partnerNodeId", default, skip_serializing_if = "Option::is_none")]
    pub partner_node_id: Option<String>,
    #[doc = "State of provisioning of the HanaInstance"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<hana_instance_properties::ProvisioningState>,
}
impl HanaInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod hana_instance_properties {
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
    #[doc = "State of provisioning of the HanaInstance"]
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
#[doc = "The response from the List HANA Instances operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HanaInstancesListResult {
    #[doc = "The list of SAP HANA on Azure instances."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HanaInstance>,
    #[doc = "The URL to get the next set of HANA instances."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HanaInstancesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl HanaInstancesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the hardware settings for the HANA instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HardwareProfile {
    #[doc = "Name of the hardware type (vendor and/or their product name)"]
    #[serde(rename = "hardwareType", default, skip_serializing_if = "Option::is_none")]
    pub hardware_type: Option<hardware_profile::HardwareType>,
    #[doc = "Specifies the HANA instance SKU."]
    #[serde(rename = "hanaInstanceSize", default, skip_serializing_if = "Option::is_none")]
    pub hana_instance_size: Option<hardware_profile::HanaInstanceSize>,
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
    #[doc = "Specifies the HANA instance SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "HanaInstanceSize")]
    pub enum HanaInstanceSize {
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
    impl FromStr for HanaInstanceSize {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for HanaInstanceSize {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for HanaInstanceSize {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::S72m => serializer.serialize_unit_variant("HanaInstanceSize", 0u32, "S72m"),
                Self::S144m => serializer.serialize_unit_variant("HanaInstanceSize", 1u32, "S144m"),
                Self::S72 => serializer.serialize_unit_variant("HanaInstanceSize", 2u32, "S72"),
                Self::S144 => serializer.serialize_unit_variant("HanaInstanceSize", 3u32, "S144"),
                Self::S192 => serializer.serialize_unit_variant("HanaInstanceSize", 4u32, "S192"),
                Self::S192m => serializer.serialize_unit_variant("HanaInstanceSize", 5u32, "S192m"),
                Self::S192xm => serializer.serialize_unit_variant("HanaInstanceSize", 6u32, "S192xm"),
                Self::S96 => serializer.serialize_unit_variant("HanaInstanceSize", 7u32, "S96"),
                Self::S112 => serializer.serialize_unit_variant("HanaInstanceSize", 8u32, "S112"),
                Self::S224 => serializer.serialize_unit_variant("HanaInstanceSize", 9u32, "S224"),
                Self::S224m => serializer.serialize_unit_variant("HanaInstanceSize", 10u32, "S224m"),
                Self::S224om => serializer.serialize_unit_variant("HanaInstanceSize", 11u32, "S224om"),
                Self::S224oo => serializer.serialize_unit_variant("HanaInstanceSize", 12u32, "S224oo"),
                Self::S224oom => serializer.serialize_unit_variant("HanaInstanceSize", 13u32, "S224oom"),
                Self::S224ooo => serializer.serialize_unit_variant("HanaInstanceSize", 14u32, "S224ooo"),
                Self::S384 => serializer.serialize_unit_variant("HanaInstanceSize", 15u32, "S384"),
                Self::S384m => serializer.serialize_unit_variant("HanaInstanceSize", 16u32, "S384m"),
                Self::S384xm => serializer.serialize_unit_variant("HanaInstanceSize", 17u32, "S384xm"),
                Self::S384xxm => serializer.serialize_unit_variant("HanaInstanceSize", 18u32, "S384xxm"),
                Self::S448 => serializer.serialize_unit_variant("HanaInstanceSize", 19u32, "S448"),
                Self::S448m => serializer.serialize_unit_variant("HanaInstanceSize", 20u32, "S448m"),
                Self::S448om => serializer.serialize_unit_variant("HanaInstanceSize", 21u32, "S448om"),
                Self::S448oo => serializer.serialize_unit_variant("HanaInstanceSize", 22u32, "S448oo"),
                Self::S448oom => serializer.serialize_unit_variant("HanaInstanceSize", 23u32, "S448oom"),
                Self::S448ooo => serializer.serialize_unit_variant("HanaInstanceSize", 24u32, "S448ooo"),
                Self::S576m => serializer.serialize_unit_variant("HanaInstanceSize", 25u32, "S576m"),
                Self::S576xm => serializer.serialize_unit_variant("HanaInstanceSize", 26u32, "S576xm"),
                Self::S672 => serializer.serialize_unit_variant("HanaInstanceSize", 27u32, "S672"),
                Self::S672m => serializer.serialize_unit_variant("HanaInstanceSize", 28u32, "S672m"),
                Self::S672om => serializer.serialize_unit_variant("HanaInstanceSize", 29u32, "S672om"),
                Self::S672oo => serializer.serialize_unit_variant("HanaInstanceSize", 30u32, "S672oo"),
                Self::S672oom => serializer.serialize_unit_variant("HanaInstanceSize", 31u32, "S672oom"),
                Self::S672ooo => serializer.serialize_unit_variant("HanaInstanceSize", 32u32, "S672ooo"),
                Self::S768 => serializer.serialize_unit_variant("HanaInstanceSize", 33u32, "S768"),
                Self::S768m => serializer.serialize_unit_variant("HanaInstanceSize", 34u32, "S768m"),
                Self::S768xm => serializer.serialize_unit_variant("HanaInstanceSize", 35u32, "S768xm"),
                Self::S896 => serializer.serialize_unit_variant("HanaInstanceSize", 36u32, "S896"),
                Self::S896m => serializer.serialize_unit_variant("HanaInstanceSize", 37u32, "S896m"),
                Self::S896om => serializer.serialize_unit_variant("HanaInstanceSize", 38u32, "S896om"),
                Self::S896oo => serializer.serialize_unit_variant("HanaInstanceSize", 39u32, "S896oo"),
                Self::S896oom => serializer.serialize_unit_variant("HanaInstanceSize", 40u32, "S896oom"),
                Self::S896ooo => serializer.serialize_unit_variant("HanaInstanceSize", 41u32, "S896ooo"),
                Self::S960m => serializer.serialize_unit_variant("HanaInstanceSize", 42u32, "S960m"),
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
#[doc = "Details needed to monitor a Hana Instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoringDetails {
    #[doc = "ARM ID of an Azure Subnet with access to the HANA instance."]
    #[serde(rename = "hanaSubnet", default, skip_serializing_if = "Option::is_none")]
    pub hana_subnet: Option<String>,
    #[doc = "Hostname of the HANA Instance blade."]
    #[serde(rename = "hanaHostname", default, skip_serializing_if = "Option::is_none")]
    pub hana_hostname: Option<String>,
    #[doc = "Name of the database itself."]
    #[serde(rename = "hanaDbName", default, skip_serializing_if = "Option::is_none")]
    pub hana_db_name: Option<String>,
    #[doc = "The port number of the tenant DB. Used to connect to the DB."]
    #[serde(rename = "hanaDbSqlPort", default, skip_serializing_if = "Option::is_none")]
    pub hana_db_sql_port: Option<i64>,
    #[doc = "Username for the HANA database to login to for monitoring"]
    #[serde(rename = "hanaDbUsername", default, skip_serializing_if = "Option::is_none")]
    pub hana_db_username: Option<String>,
    #[doc = "Password for the HANA database to login for monitoring"]
    #[serde(rename = "hanaDbPassword", default, skip_serializing_if = "Option::is_none")]
    pub hana_db_password: Option<String>,
}
impl MonitoringDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the network settings for the HANA instance disks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkProfile {
    #[doc = "Specifies the network interfaces for the HANA instance."]
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
#[doc = "Specifies the operating system settings for the HANA instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsProfile {
    #[doc = "Specifies the host OS name of the HANA instance."]
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
#[doc = "HANA operation information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation being performed on this particular object. This name should match the action name that appears in RBAC / the event service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Detailed HANA operation information"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Display>,
    #[doc = "Indicates whether the operation applies to data-plane."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of HANA operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[doc = "List of HANA operations"]
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
#[doc = "The resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies information related to a SAP system ID"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SapSystemId {
    #[doc = "Group ID of the HANA database user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gid: Option<String>,
    #[doc = "Percent of memory to allocate to this SID."]
    #[serde(rename = "memoryAllocation", default, skip_serializing_if = "Option::is_none")]
    pub memory_allocation: Option<String>,
    #[doc = "SAP system ID as database identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[doc = "Name of the HANA database user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "User ID of the HANA database user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}
impl SapSystemId {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the storage settings for the HANA instance disks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageProfile {
    #[doc = "IP Address to connect to storage."]
    #[serde(rename = "nfsIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub nfs_ip_address: Option<String>,
    #[doc = "Specifies information about the operating system disk used by the hana instance."]
    #[serde(rename = "osDisks", default, skip_serializing_if = "Vec::is_empty")]
    pub os_disks: Vec<Disk>,
    #[doc = "Specifies information related to SAP system IDs for the hana instance."]
    #[serde(rename = "hanaSids", default, skip_serializing_if = "Vec::is_empty")]
    pub hana_sids: Vec<SapSystemId>,
}
impl StorageProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tags field of the HANA instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tags {
    #[doc = "Tags field of the HANA instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Tags {
    pub fn new() -> Self {
        Self::default()
    }
}
