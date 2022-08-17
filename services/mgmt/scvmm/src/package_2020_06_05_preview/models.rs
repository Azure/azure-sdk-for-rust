#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Allocation method."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AllocationMethod")]
pub enum AllocationMethod {
    Dynamic,
    Static,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AllocationMethod {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AllocationMethod {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AllocationMethod {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Dynamic => serializer.serialize_unit_variant("AllocationMethod", 0u32, "Dynamic"),
            Self::Static => serializer.serialize_unit_variant("AllocationMethod", 1u32, "Static"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The AvailabilitySets resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailabilitySet {
    #[doc = "Defines the resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvailabilitySetProperties>,
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource Type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets the location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
}
impl AvailabilitySet {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type AvailabilitySetList = Vec<serde_json::Value>;
#[doc = "List of AvailabilitySets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailabilitySetListResult {
    #[doc = "List of AvailabilitySets."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AvailabilitySet>,
    #[doc = "Url to follow for getting next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AvailabilitySetListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AvailabilitySetListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailabilitySetProperties {
    #[doc = "Name of the availability set."]
    #[serde(rename = "availabilitySetName", default, skip_serializing_if = "Option::is_none")]
    pub availability_set_name: Option<String>,
    #[doc = "ARM Id of the vmmServer resource in which this resource resides."]
    #[serde(rename = "vmmServerId", default, skip_serializing_if = "Option::is_none")]
    pub vmm_server_id: Option<String>,
    #[doc = "Gets or sets the provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl AvailabilitySetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Checkpoint {
    #[doc = "Gets ID of parent of the checkpoint."]
    #[serde(rename = "parentCheckpointID", default, skip_serializing_if = "Option::is_none")]
    pub parent_checkpoint_id: Option<String>,
    #[doc = "Gets ID of the checkpoint."]
    #[serde(rename = "checkpointID", default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_id: Option<String>,
    #[doc = "Gets name of the checkpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets description of the checkpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl Checkpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Clouds resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cloud {
    #[doc = "Defines the resource properties."]
    pub properties: CloudProperties,
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource Type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets the location."]
    pub location: String,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The extended location."]
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
}
impl Cloud {
    pub fn new(properties: CloudProperties, location: String, extended_location: ExtendedLocation) -> Self {
        Self {
            properties,
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
            system_data: None,
            extended_location,
        }
    }
}
#[doc = "Cloud Capacity model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudCapacity {
    #[doc = "CPUCount specifies the maximum number of CPUs that can be allocated in the cloud."]
    #[serde(rename = "cpuCount", default, skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<i64>,
    #[doc = "MemoryMB specifies a memory usage limit in megabytes."]
    #[serde(rename = "memoryMB", default, skip_serializing_if = "Option::is_none")]
    pub memory_mb: Option<i64>,
    #[doc = "VMCount gives the max number of VMs that can be deployed in the cloud."]
    #[serde(rename = "vmCount", default, skip_serializing_if = "Option::is_none")]
    pub vm_count: Option<i64>,
}
impl CloudCapacity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Cloud inventory item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudInventoryItem {
    #[serde(flatten)]
    pub inventory_item_properties: InventoryItemProperties,
}
impl CloudInventoryItem {
    pub fn new(inventory_item_properties: InventoryItemProperties) -> Self {
        Self { inventory_item_properties }
    }
}
#[doc = "List of Clouds."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudListResult {
    #[doc = "List of Clouds."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Cloud>,
    #[doc = "Url to follow for getting next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CloudListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CloudListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudProperties {
    #[doc = "Gets or sets the inventory Item ID for the resource."]
    #[serde(rename = "inventoryItemId", default, skip_serializing_if = "Option::is_none")]
    pub inventory_item_id: Option<String>,
    #[doc = "Unique ID of the cloud."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "ARM Id of the vmmServer resource in which this resource resides."]
    #[serde(rename = "vmmServerId", default, skip_serializing_if = "Option::is_none")]
    pub vmm_server_id: Option<String>,
    #[doc = "Name of the cloud in VMMServer."]
    #[serde(rename = "cloudName", default, skip_serializing_if = "Option::is_none")]
    pub cloud_name: Option<String>,
    #[doc = "Cloud Capacity model"]
    #[serde(rename = "cloudCapacity", default, skip_serializing_if = "Option::is_none")]
    pub cloud_capacity: Option<CloudCapacity>,
    #[doc = "List of QoS policies available for the cloud."]
    #[serde(rename = "storageQoSPolicies", default, skip_serializing_if = "Vec::is_empty")]
    pub storage_qo_s_policies: Vec<StorageQoSPolicy>,
    #[doc = "Gets or sets the provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl CloudProperties {
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
#[doc = "The extended location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedLocation {
    #[doc = "The extended location type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The extended location name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ExtendedLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HardwareProfile {
    #[doc = "MemoryMB is the size of a virtual machine's memory, in MB."]
    #[serde(rename = "memoryMB", default, skip_serializing_if = "Option::is_none")]
    pub memory_mb: Option<i32>,
    #[doc = "Gets or sets the number of vCPUs for the vm."]
    #[serde(rename = "cpuCount", default, skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<i32>,
    #[doc = "Gets or sets a value indicating whether to enable processor compatibility mode for live migration of VMs."]
    #[serde(rename = "limitCpuForMigration", default, skip_serializing_if = "Option::is_none")]
    pub limit_cpu_for_migration: Option<hardware_profile::LimitCpuForMigration>,
    #[doc = "Gets or sets a value indicating whether to enable dynamic memory or not."]
    #[serde(rename = "dynamicMemoryEnabled", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_memory_enabled: Option<hardware_profile::DynamicMemoryEnabled>,
    #[doc = "Gets or sets the max dynamic memory for the vm."]
    #[serde(rename = "dynamicMemoryMaxMB", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_memory_max_mb: Option<i32>,
    #[doc = "Gets or sets the min dynamic memory for the vm."]
    #[serde(rename = "dynamicMemoryMinMB", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_memory_min_mb: Option<i32>,
    #[doc = "Gets highly available property."]
    #[serde(rename = "isHighlyAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_highly_available: Option<String>,
}
impl HardwareProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod hardware_profile {
    use super::*;
    #[doc = "Gets or sets a value indicating whether to enable processor compatibility mode for live migration of VMs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LimitCpuForMigration")]
    pub enum LimitCpuForMigration {
        #[serde(rename = "false")]
        False,
        #[serde(rename = "true")]
        True,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LimitCpuForMigration {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LimitCpuForMigration {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LimitCpuForMigration {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::False => serializer.serialize_unit_variant("LimitCpuForMigration", 0u32, "false"),
                Self::True => serializer.serialize_unit_variant("LimitCpuForMigration", 1u32, "true"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets or sets a value indicating whether to enable dynamic memory or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DynamicMemoryEnabled")]
    pub enum DynamicMemoryEnabled {
        #[serde(rename = "false")]
        False,
        #[serde(rename = "true")]
        True,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DynamicMemoryEnabled {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DynamicMemoryEnabled {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DynamicMemoryEnabled {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::False => serializer.serialize_unit_variant("DynamicMemoryEnabled", 0u32, "false"),
                Self::True => serializer.serialize_unit_variant("DynamicMemoryEnabled", 1u32, "true"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HardwareProfileUpdate {
    #[doc = "MemoryMB is the size of a virtual machine's memory, in MB."]
    #[serde(rename = "memoryMB", default, skip_serializing_if = "Option::is_none")]
    pub memory_mb: Option<i32>,
    #[doc = "Gets or sets the number of vCPUs for the vm."]
    #[serde(rename = "cpuCount", default, skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<i32>,
    #[doc = "Gets or sets a value indicating whether to enable processor compatibility mode for live migration of VMs."]
    #[serde(rename = "limitCpuForMigration", default, skip_serializing_if = "Option::is_none")]
    pub limit_cpu_for_migration: Option<hardware_profile_update::LimitCpuForMigration>,
    #[doc = "Gets or sets a value indicating whether to enable dynamic memory or not."]
    #[serde(rename = "dynamicMemoryEnabled", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_memory_enabled: Option<hardware_profile_update::DynamicMemoryEnabled>,
    #[doc = "Gets or sets the max dynamic memory for the vm."]
    #[serde(rename = "dynamicMemoryMaxMB", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_memory_max_mb: Option<i32>,
    #[doc = "Gets or sets the min dynamic memory for the vm."]
    #[serde(rename = "dynamicMemoryMinMB", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_memory_min_mb: Option<i32>,
}
impl HardwareProfileUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod hardware_profile_update {
    use super::*;
    #[doc = "Gets or sets a value indicating whether to enable processor compatibility mode for live migration of VMs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LimitCpuForMigration")]
    pub enum LimitCpuForMigration {
        #[serde(rename = "false")]
        False,
        #[serde(rename = "true")]
        True,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LimitCpuForMigration {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LimitCpuForMigration {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LimitCpuForMigration {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::False => serializer.serialize_unit_variant("LimitCpuForMigration", 0u32, "false"),
                Self::True => serializer.serialize_unit_variant("LimitCpuForMigration", 1u32, "true"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets or sets a value indicating whether to enable dynamic memory or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DynamicMemoryEnabled")]
    pub enum DynamicMemoryEnabled {
        #[serde(rename = "false")]
        False,
        #[serde(rename = "true")]
        True,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DynamicMemoryEnabled {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DynamicMemoryEnabled {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DynamicMemoryEnabled {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::False => serializer.serialize_unit_variant("DynamicMemoryEnabled", 0u32, "false"),
                Self::True => serializer.serialize_unit_variant("DynamicMemoryEnabled", 1u32, "true"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the inventory item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InventoryItem {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Defines the resource properties."]
    pub properties: InventoryItemProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Metadata used by portal/tooling/etc to render different UX experiences for resources of the same type; e.g. ApiApps are a kind of Microsoft.Web/sites type.  If supported, the resource provider must validate and persist this value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
impl InventoryItem {
    pub fn new(properties: InventoryItemProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
            kind: None,
        }
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InventoryItemDetails {
    #[doc = "Gets or sets the inventory Item ID for the resource."]
    #[serde(rename = "inventoryItemId", default, skip_serializing_if = "Option::is_none")]
    pub inventory_item_id: Option<String>,
    #[doc = "Gets or sets the Managed Object name in VMM for the resource."]
    #[serde(rename = "inventoryItemName", default, skip_serializing_if = "Option::is_none")]
    pub inventory_item_name: Option<String>,
}
impl InventoryItemDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InventoryItemProperties {
    #[doc = "The inventory type."]
    #[serde(rename = "inventoryType")]
    pub inventory_type: InventoryType,
    #[doc = "Gets the tracked resource id corresponding to the inventory resource."]
    #[serde(rename = "managedResourceId", default, skip_serializing_if = "Option::is_none")]
    pub managed_resource_id: Option<String>,
    #[doc = "Gets the UUID (which is assigned by VMM) for the inventory item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "Gets the Managed Object name in VMM for the inventory item."]
    #[serde(rename = "inventoryItemName", default, skip_serializing_if = "Option::is_none")]
    pub inventory_item_name: Option<String>,
    #[doc = "Gets the provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl InventoryItemProperties {
    pub fn new(inventory_type: InventoryType) -> Self {
        Self {
            inventory_type,
            managed_resource_id: None,
            uuid: None,
            inventory_item_name: None,
            provisioning_state: None,
        }
    }
}
#[doc = "List of InventoryItems."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InventoryItemsList {
    #[doc = "Url to follow for getting next page of InventoryItems."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of InventoryItems"]
    pub value: Vec<InventoryItem>,
}
impl azure_core::Continuable for InventoryItemsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl InventoryItemsList {
    pub fn new(value: Vec<InventoryItem>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "The inventory type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "InventoryType")]
pub enum InventoryType {
    Cloud,
    VirtualNetwork,
    VirtualMachineTemplate,
    VirtualMachine,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for InventoryType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for InventoryType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for InventoryType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Cloud => serializer.serialize_unit_variant("InventoryType", 0u32, "Cloud"),
            Self::VirtualNetwork => serializer.serialize_unit_variant("InventoryType", 1u32, "VirtualNetwork"),
            Self::VirtualMachineTemplate => serializer.serialize_unit_variant("InventoryType", 2u32, "VirtualMachineTemplate"),
            Self::VirtualMachine => serializer.serialize_unit_variant("InventoryType", 3u32, "VirtualMachine"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Network Interface model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfaces {
    #[doc = "Gets or sets the name of the network interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the display name of the network interface as shown in the vmmServer. This is the fallback label for a NIC when the name is not set."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets or sets the nic ipv4 addresses."]
    #[serde(rename = "ipv4Addresses", default, skip_serializing_if = "Vec::is_empty")]
    pub ipv4_addresses: Vec<String>,
    #[doc = "Gets or sets the nic ipv6 addresses."]
    #[serde(rename = "ipv6Addresses", default, skip_serializing_if = "Vec::is_empty")]
    pub ipv6_addresses: Vec<String>,
    #[doc = "Gets or sets the nic MAC address."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "Gets or sets the ARM Id of the Microsoft.ScVmm/virtualNetwork resource to connect the nic."]
    #[serde(rename = "virtualNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_id: Option<String>,
    #[doc = "Gets or sets the name of the virtual network in vmmServer that the nic is connected to."]
    #[serde(rename = "networkName", default, skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    #[doc = "Allocation method."]
    #[serde(rename = "ipv4AddressType", default, skip_serializing_if = "Option::is_none")]
    pub ipv4_address_type: Option<AllocationMethod>,
    #[doc = "Allocation method."]
    #[serde(rename = "ipv6AddressType", default, skip_serializing_if = "Option::is_none")]
    pub ipv6_address_type: Option<AllocationMethod>,
    #[doc = "Allocation method."]
    #[serde(rename = "macAddressType", default, skip_serializing_if = "Option::is_none")]
    pub mac_address_type: Option<AllocationMethod>,
    #[doc = "Gets or sets the nic id."]
    #[serde(rename = "nicId", default, skip_serializing_if = "Option::is_none")]
    pub nic_id: Option<String>,
}
impl NetworkInterfaces {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network Interface model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkInterfacesUpdate {
    #[doc = "Gets or sets the name of the network interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the nic MAC address."]
    #[serde(rename = "macAddress", default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[doc = "Gets or sets the ARM Id of the Microsoft.ScVmm/virtualNetwork resource to connect the nic."]
    #[serde(rename = "virtualNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_id: Option<String>,
    #[doc = "Allocation method."]
    #[serde(rename = "ipv4AddressType", default, skip_serializing_if = "Option::is_none")]
    pub ipv4_address_type: Option<AllocationMethod>,
    #[doc = "Allocation method."]
    #[serde(rename = "ipv6AddressType", default, skip_serializing_if = "Option::is_none")]
    pub ipv6_address_type: Option<AllocationMethod>,
    #[doc = "Allocation method."]
    #[serde(rename = "macAddressType", default, skip_serializing_if = "Option::is_none")]
    pub mac_address_type: Option<AllocationMethod>,
    #[doc = "Gets or sets the nic id."]
    #[serde(rename = "nicId", default, skip_serializing_if = "Option::is_none")]
    pub nic_id: Option<String>,
}
impl NetworkInterfacesUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkProfile {
    #[doc = "Gets or sets the list of network interfaces associated with the virtual machine."]
    #[serde(rename = "networkInterfaces", default, skip_serializing_if = "Vec::is_empty")]
    pub network_interfaces: Vec<NetworkInterfaces>,
}
impl NetworkProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkProfileUpdate {
    #[doc = "Gets or sets the list of network interfaces associated with the virtual machine."]
    #[serde(rename = "networkInterfaces", default, skip_serializing_if = "Vec::is_empty")]
    pub network_interfaces: Vec<NetworkInterfacesUpdate>,
}
impl NetworkProfileUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsProfile {
    #[doc = "Admin password of the virtual machine."]
    #[serde(rename = "adminPassword", default, skip_serializing_if = "Option::is_none")]
    pub admin_password: Option<String>,
    #[doc = "Gets or sets computer name."]
    #[serde(rename = "computerName", default, skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    #[doc = "Defines the different types of VM guest operating systems."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
    #[doc = "Gets or sets os name."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
}
impl OsProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the different types of VM guest operating systems."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OsType")]
pub enum OsType {
    Windows,
    Linux,
    Other,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OsType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OsType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OsType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Windows => serializer.serialize_unit_variant("OsType", 0u32, "Windows"),
            Self::Linux => serializer.serialize_unit_variant("OsType", 1u32, "Linux"),
            Self::Other => serializer.serialize_unit_variant("OsType", 2u32, "Other"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The resource model definition for a Azure Resource Manager proxy resource. It will not have tags and a location"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
impl ProxyResource {
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
#[doc = "Object containing tags updates for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourcePatch {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ResourcePatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Results of the request to list operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperation {
    #[doc = "Indicates whether the operation applies to data-plane."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<String>,
    #[doc = "Operation name, in format of {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display metadata associated with the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<resource_provider_operation::Display>,
}
impl ResourceProviderOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_provider_operation {
    use super::*;
    #[doc = "Display metadata associated with the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The resource provider."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Type of operation: read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of this operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Results of the request to list operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperationList {
    #[doc = "List of Operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceProviderOperation>,
    #[doc = "Url to follow for getting next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceProviderOperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResourceProviderOperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the stop action properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StopVirtualMachineOptions {
    #[doc = "Gets or sets a value indicating whether to request non-graceful VM shutdown. True value for this flag indicates non-graceful shutdown whereas false indicates otherwise. Defaults to false."]
    #[serde(rename = "skipShutdown", default, skip_serializing_if = "Option::is_none")]
    pub skip_shutdown: Option<bool>,
}
impl StopVirtualMachineOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageProfile {
    #[doc = "Gets or sets the list of virtual disks associated with the virtual machine."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<VirtualDisk>,
}
impl StorageProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageProfileUpdate {
    #[doc = "Gets or sets the list of virtual disks associated with the virtual machine."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<VirtualDiskUpdate>,
}
impl StorageProfileUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The StorageQoSPolicy definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageQoSPolicy {
    #[doc = "The name of the policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The ID of the QoS policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The maximum IO operations per second."]
    #[serde(rename = "iopsMaximum", default, skip_serializing_if = "Option::is_none")]
    pub iops_maximum: Option<i64>,
    #[doc = "The minimum IO operations per second."]
    #[serde(rename = "iopsMinimum", default, skip_serializing_if = "Option::is_none")]
    pub iops_minimum: Option<i64>,
    #[doc = "The Bandwidth Limit for internet traffic."]
    #[serde(rename = "bandwidthLimit", default, skip_serializing_if = "Option::is_none")]
    pub bandwidth_limit: Option<i64>,
    #[doc = "The underlying policy."]
    #[serde(rename = "policyId", default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
}
impl StorageQoSPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The StorageQoSPolicyDetails definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageQoSPolicyDetails {
    #[doc = "The name of the policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The ID of the QoS policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl StorageQoSPolicyDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The VmmServers resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmmServer {
    #[doc = "Defines the resource properties."]
    pub properties: VmmServerProperties,
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource Type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets the location."]
    pub location: String,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The extended location."]
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
}
impl VmmServer {
    pub fn new(properties: VmmServerProperties, location: String, extended_location: ExtendedLocation) -> Self {
        Self {
            properties,
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
            system_data: None,
            extended_location,
        }
    }
}
#[doc = "List of VmmServers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmmServerListResult {
    #[doc = "List of VmmServers."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VmmServer>,
    #[doc = "Url to follow for getting next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VmmServerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VmmServerListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmmServerProperties {
    #[doc = "Credentials to connect to VMMServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<vmm_server_properties::Credentials>,
    #[doc = "Fqdn is the hostname/ip of the vmmServer."]
    pub fqdn: String,
    #[doc = "Port is the port on which the vmmServer is listening."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "Gets or sets the connection status to the vmmServer."]
    #[serde(rename = "connectionStatus", default, skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<String>,
    #[doc = "Gets or sets any error message if connection to vmmServer is having any issue."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "Unique ID of vmmServer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "Version is the version of the vmmSever."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Gets or sets the provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl VmmServerProperties {
    pub fn new(fqdn: String) -> Self {
        Self {
            credentials: None,
            fqdn,
            port: None,
            connection_status: None,
            error_message: None,
            uuid: None,
            version: None,
            provisioning_state: None,
        }
    }
}
pub mod vmm_server_properties {
    use super::*;
    #[doc = "Credentials to connect to VMMServer."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Credentials {
        #[doc = "Username to use to connect to VMMServer."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub username: Option<String>,
        #[doc = "Credentials to use to connect to VMMServer."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub password: Option<String>,
    }
    impl Credentials {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Virtual disk model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualDisk {
    #[doc = "Gets or sets the name of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the display name of the virtual disk as shown in the vmmServer. This is the fallback label for a disk when the name is not set."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets or sets the disk id."]
    #[serde(rename = "diskId", default, skip_serializing_if = "Option::is_none")]
    pub disk_id: Option<String>,
    #[doc = "Gets or sets the disk total size."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[doc = "Gets or sets the max disk size."]
    #[serde(rename = "maxDiskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub max_disk_size_gb: Option<i32>,
    #[doc = "Gets or sets the disk bus."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bus: Option<i32>,
    #[doc = "Gets or sets the disk lun."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
    #[doc = "Gets or sets the disk bus type."]
    #[serde(rename = "busType", default, skip_serializing_if = "Option::is_none")]
    pub bus_type: Option<String>,
    #[doc = "Gets or sets the disk vhd type."]
    #[serde(rename = "vhdType", default, skip_serializing_if = "Option::is_none")]
    pub vhd_type: Option<String>,
    #[doc = "Gets or sets the disk volume type."]
    #[serde(rename = "volumeType", default, skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
    #[doc = "Gets the disk vhd format type."]
    #[serde(rename = "vhdFormatType", default, skip_serializing_if = "Option::is_none")]
    pub vhd_format_type: Option<String>,
    #[doc = "Gets or sets the disk id in the template."]
    #[serde(rename = "templateDiskId", default, skip_serializing_if = "Option::is_none")]
    pub template_disk_id: Option<String>,
    #[doc = "The StorageQoSPolicyDetails definition."]
    #[serde(rename = "storageQoSPolicy", default, skip_serializing_if = "Option::is_none")]
    pub storage_qo_s_policy: Option<StorageQoSPolicyDetails>,
    #[doc = "Gets or sets a value indicating diff disk."]
    #[serde(rename = "createDiffDisk", default, skip_serializing_if = "Option::is_none")]
    pub create_diff_disk: Option<virtual_disk::CreateDiffDisk>,
}
impl VirtualDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_disk {
    use super::*;
    #[doc = "Gets or sets a value indicating diff disk."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreateDiffDisk")]
    pub enum CreateDiffDisk {
        #[serde(rename = "false")]
        False,
        #[serde(rename = "true")]
        True,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CreateDiffDisk {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CreateDiffDisk {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CreateDiffDisk {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::False => serializer.serialize_unit_variant("CreateDiffDisk", 0u32, "false"),
                Self::True => serializer.serialize_unit_variant("CreateDiffDisk", 1u32, "true"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Virtual disk model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualDiskUpdate {
    #[doc = "Gets or sets the name of the disk."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the disk id."]
    #[serde(rename = "diskId", default, skip_serializing_if = "Option::is_none")]
    pub disk_id: Option<String>,
    #[doc = "Gets or sets the disk total size."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[doc = "Gets or sets the disk bus."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bus: Option<i32>,
    #[doc = "Gets or sets the disk lun."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
    #[doc = "Gets or sets the disk bus type."]
    #[serde(rename = "busType", default, skip_serializing_if = "Option::is_none")]
    pub bus_type: Option<String>,
    #[doc = "Gets or sets the disk vhd type."]
    #[serde(rename = "vhdType", default, skip_serializing_if = "Option::is_none")]
    pub vhd_type: Option<String>,
    #[doc = "The StorageQoSPolicyDetails definition."]
    #[serde(rename = "storageQoSPolicy", default, skip_serializing_if = "Option::is_none")]
    pub storage_qo_s_policy: Option<StorageQoSPolicyDetails>,
}
impl VirtualDiskUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The VirtualMachines resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachine {
    #[doc = "Defines the resource properties."]
    pub properties: VirtualMachineProperties,
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource Type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets the location."]
    pub location: String,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The extended location."]
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
}
impl VirtualMachine {
    pub fn new(properties: VirtualMachineProperties, location: String, extended_location: ExtendedLocation) -> Self {
        Self {
            properties,
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
            system_data: None,
            extended_location,
        }
    }
}
#[doc = "Defines the create checkpoint action properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineCreateCheckpoint {
    #[doc = "Name of the checkpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Description of the checkpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl VirtualMachineCreateCheckpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the delete checkpoint action properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineDeleteCheckpoint {
    #[doc = "ID of the checkpoint to be deleted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl VirtualMachineDeleteCheckpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Virtual machine inventory item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineInventoryItem {
    #[serde(flatten)]
    pub inventory_item_properties: InventoryItemProperties,
    #[doc = "Defines the different types of VM guest operating systems."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
    #[doc = "Gets or sets os name."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "Gets the power state of the virtual machine."]
    #[serde(rename = "powerState", default, skip_serializing_if = "Option::is_none")]
    pub power_state: Option<String>,
    #[doc = "Gets or sets the nic ip addresses."]
    #[serde(rename = "ipAddresses", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_addresses: Vec<String>,
    #[doc = "Defines the resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud: Option<InventoryItemDetails>,
}
impl VirtualMachineInventoryItem {
    pub fn new(inventory_item_properties: InventoryItemProperties) -> Self {
        Self {
            inventory_item_properties,
            os_type: None,
            os_name: None,
            power_state: None,
            ip_addresses: Vec::new(),
            cloud: None,
        }
    }
}
#[doc = "List of VirtualMachines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineListResult {
    #[doc = "List of VirtualMachines."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualMachine>,
    #[doc = "Url to follow for getting next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualMachineListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualMachineListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineProperties {
    #[doc = "Gets or sets the inventory Item ID for the resource."]
    #[serde(rename = "inventoryItemId", default, skip_serializing_if = "Option::is_none")]
    pub inventory_item_id: Option<String>,
    #[doc = "ARM Id of the vmmServer resource in which this resource resides."]
    #[serde(rename = "vmmServerId", default, skip_serializing_if = "Option::is_none")]
    pub vmm_server_id: Option<String>,
    #[doc = "ARM Id of the cloud resource to use for deploying the vm."]
    #[serde(rename = "cloudId", default, skip_serializing_if = "Option::is_none")]
    pub cloud_id: Option<String>,
    #[doc = "ARM Id of the template resource to use for deploying the vm."]
    #[serde(rename = "templateId", default, skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[doc = "Type of checkpoint supported for the vm."]
    #[serde(rename = "checkpointType", default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_type: Option<String>,
    #[doc = "Checkpoints in the vm."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub checkpoints: Vec<Checkpoint>,
    #[doc = "Availability Sets in vm."]
    #[serde(rename = "availabilitySets", default, skip_serializing_if = "Option::is_none")]
    pub availability_sets: Option<AvailabilitySetList>,
    #[doc = "Defines the resource properties."]
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,
    #[doc = "Defines the resource properties."]
    #[serde(rename = "hardwareProfile", default, skip_serializing_if = "Option::is_none")]
    pub hardware_profile: Option<HardwareProfile>,
    #[doc = "Defines the resource properties."]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
    #[doc = "Defines the resource properties."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[doc = "VMName is the name of VM on the SCVMM server."]
    #[serde(rename = "vmName", default, skip_serializing_if = "Option::is_none")]
    pub vm_name: Option<String>,
    #[doc = "Unique ID of the virtual machine."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "Gets or sets the generation for the vm."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<i32>,
    #[doc = "Gets the power state of the virtual machine."]
    #[serde(rename = "powerState", default, skip_serializing_if = "Option::is_none")]
    pub power_state: Option<String>,
    #[doc = "Gets or sets the provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl VirtualMachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the restore checkpoint action properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineRestoreCheckpoint {
    #[doc = "ID of the checkpoint to be restored to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl VirtualMachineRestoreCheckpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The VirtualMachineTemplates resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineTemplate {
    #[doc = "Defines the resource properties."]
    pub properties: VirtualMachineTemplateProperties,
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource Type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets the location."]
    pub location: String,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The extended location."]
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
}
impl VirtualMachineTemplate {
    pub fn new(properties: VirtualMachineTemplateProperties, location: String, extended_location: ExtendedLocation) -> Self {
        Self {
            properties,
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
            system_data: None,
            extended_location,
        }
    }
}
#[doc = "The Virtual machine template inventory item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualMachineTemplateInventoryItem {
    #[serde(flatten)]
    pub inventory_item_properties: InventoryItemProperties,
    #[doc = "Gets or sets the desired number of vCPUs for the vm."]
    #[serde(rename = "cpuCount", default, skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<i32>,
    #[doc = "MemoryMB is the desired size of a virtual machine's memory, in MB."]
    #[serde(rename = "memoryMB", default, skip_serializing_if = "Option::is_none")]
    pub memory_mb: Option<i32>,
    #[doc = "Defines the different types of VM guest operating systems."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
    #[doc = "Gets or sets os name."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
}
impl VirtualMachineTemplateInventoryItem {
    pub fn new(inventory_item_properties: InventoryItemProperties) -> Self {
        Self {
            inventory_item_properties,
            cpu_count: None,
            memory_mb: None,
            os_type: None,
            os_name: None,
        }
    }
}
#[doc = "List of VirtualMachineTemplates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineTemplateListResult {
    #[doc = "List of VirtualMachineTemplates."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualMachineTemplate>,
    #[doc = "Url to follow for getting next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualMachineTemplateListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualMachineTemplateListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineTemplateProperties {
    #[doc = "Gets or sets the inventory Item ID for the resource."]
    #[serde(rename = "inventoryItemId", default, skip_serializing_if = "Option::is_none")]
    pub inventory_item_id: Option<String>,
    #[doc = "Unique ID of the virtual machine template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "ARM Id of the vmmServer resource in which this resource resides."]
    #[serde(rename = "vmmServerId", default, skip_serializing_if = "Option::is_none")]
    pub vmm_server_id: Option<String>,
    #[doc = "Defines the different types of VM guest operating systems."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<OsType>,
    #[doc = "Gets or sets os name."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "Gets or sets computer name."]
    #[serde(rename = "computerName", default, skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    #[doc = "MemoryMB is the desired size of a virtual machine's memory, in MB."]
    #[serde(rename = "memoryMB", default, skip_serializing_if = "Option::is_none")]
    pub memory_mb: Option<i32>,
    #[doc = "Gets or sets the desired number of vCPUs for the vm."]
    #[serde(rename = "cpuCount", default, skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<i32>,
    #[doc = "Gets or sets a value indicating whether to enable processor compatibility mode for live migration of VMs."]
    #[serde(rename = "limitCpuForMigration", default, skip_serializing_if = "Option::is_none")]
    pub limit_cpu_for_migration: Option<virtual_machine_template_properties::LimitCpuForMigration>,
    #[doc = "Gets or sets a value indicating whether to enable dynamic memory or not."]
    #[serde(rename = "dynamicMemoryEnabled", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_memory_enabled: Option<virtual_machine_template_properties::DynamicMemoryEnabled>,
    #[doc = "Gets or sets a value indicating whether the vm template is customizable or not."]
    #[serde(rename = "isCustomizable", default, skip_serializing_if = "Option::is_none")]
    pub is_customizable: Option<virtual_machine_template_properties::IsCustomizable>,
    #[doc = "Gets or sets the max dynamic memory for the vm."]
    #[serde(rename = "dynamicMemoryMaxMB", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_memory_max_mb: Option<i32>,
    #[doc = "Gets or sets the min dynamic memory for the vm."]
    #[serde(rename = "dynamicMemoryMinMB", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_memory_min_mb: Option<i32>,
    #[doc = "Gets highly available property."]
    #[serde(rename = "isHighlyAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_highly_available: Option<String>,
    #[doc = "Gets or sets the generation for the vm."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<i32>,
    #[doc = "Gets or sets the network interfaces of the template."]
    #[serde(rename = "networkInterfaces", default, skip_serializing_if = "Vec::is_empty")]
    pub network_interfaces: Vec<NetworkInterfaces>,
    #[doc = "Gets or sets the disks of the template."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<VirtualDisk>,
    #[doc = "Gets or sets the provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl VirtualMachineTemplateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod virtual_machine_template_properties {
    use super::*;
    #[doc = "Gets or sets a value indicating whether to enable processor compatibility mode for live migration of VMs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LimitCpuForMigration")]
    pub enum LimitCpuForMigration {
        #[serde(rename = "false")]
        False,
        #[serde(rename = "true")]
        True,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LimitCpuForMigration {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LimitCpuForMigration {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LimitCpuForMigration {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::False => serializer.serialize_unit_variant("LimitCpuForMigration", 0u32, "false"),
                Self::True => serializer.serialize_unit_variant("LimitCpuForMigration", 1u32, "true"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets or sets a value indicating whether to enable dynamic memory or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DynamicMemoryEnabled")]
    pub enum DynamicMemoryEnabled {
        #[serde(rename = "false")]
        False,
        #[serde(rename = "true")]
        True,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DynamicMemoryEnabled {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DynamicMemoryEnabled {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DynamicMemoryEnabled {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::False => serializer.serialize_unit_variant("DynamicMemoryEnabled", 0u32, "false"),
                Self::True => serializer.serialize_unit_variant("DynamicMemoryEnabled", 1u32, "true"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets or sets a value indicating whether the vm template is customizable or not."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IsCustomizable")]
    pub enum IsCustomizable {
        #[serde(rename = "false")]
        False,
        #[serde(rename = "true")]
        True,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IsCustomizable {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IsCustomizable {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IsCustomizable {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::False => serializer.serialize_unit_variant("IsCustomizable", 0u32, "false"),
                Self::True => serializer.serialize_unit_variant("IsCustomizable", 1u32, "true"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Defines the virtualMachineUpdate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineUpdate {
    #[doc = "Defines the resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VirtualMachineUpdateProperties>,
    #[doc = "Gets or sets the Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl VirtualMachineUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualMachineUpdateProperties {
    #[doc = "Defines the resource properties."]
    #[serde(rename = "hardwareProfile", default, skip_serializing_if = "Option::is_none")]
    pub hardware_profile: Option<HardwareProfileUpdate>,
    #[doc = "Defines the resource properties."]
    #[serde(rename = "storageProfile", default, skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfileUpdate>,
    #[doc = "Defines the resource properties."]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfileUpdate>,
    #[doc = "Availability Sets in vm."]
    #[serde(rename = "availabilitySets", default, skip_serializing_if = "Option::is_none")]
    pub availability_sets: Option<AvailabilitySetList>,
}
impl VirtualMachineUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The VirtualNetworks resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetwork {
    #[doc = "Defines the resource properties."]
    pub properties: VirtualNetworkProperties,
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource Type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Gets or sets the location."]
    pub location: String,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The extended location."]
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
}
impl VirtualNetwork {
    pub fn new(properties: VirtualNetworkProperties, location: String, extended_location: ExtendedLocation) -> Self {
        Self {
            properties,
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
            system_data: None,
            extended_location,
        }
    }
}
#[doc = "The Virtual network inventory item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkInventoryItem {
    #[serde(flatten)]
    pub inventory_item_properties: InventoryItemProperties,
}
impl VirtualNetworkInventoryItem {
    pub fn new(inventory_item_properties: InventoryItemProperties) -> Self {
        Self { inventory_item_properties }
    }
}
#[doc = "List of VirtualNetworks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkListResult {
    #[doc = "List of VirtualNetworks."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualNetwork>,
    #[doc = "Url to follow for getting next page of resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualNetworkListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualNetworkListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkProperties {
    #[doc = "Gets or sets the inventory Item ID for the resource."]
    #[serde(rename = "inventoryItemId", default, skip_serializing_if = "Option::is_none")]
    pub inventory_item_id: Option<String>,
    #[doc = "Unique ID of the virtual network."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "ARM Id of the vmmServer resource in which this resource resides."]
    #[serde(rename = "vmmServerId", default, skip_serializing_if = "Option::is_none")]
    pub vmm_server_id: Option<String>,
    #[doc = "Name of the virtual network in vmmServer."]
    #[serde(rename = "networkName", default, skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    #[doc = "Gets or sets the provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl VirtualNetworkProperties {
    pub fn new() -> Self {
        Self::default()
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
