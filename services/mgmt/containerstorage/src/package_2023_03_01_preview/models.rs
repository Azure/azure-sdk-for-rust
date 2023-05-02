#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AccessMode")]
pub enum AccessMode {
    ReadWriteOnce,
    ReadOnlyMany,
    ReadWriteMany,
    ReadWriteOncePod,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AccessMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AccessMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AccessMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ReadWriteOnce => serializer.serialize_unit_variant("AccessMode", 0u32, "ReadWriteOnce"),
            Self::ReadOnlyMany => serializer.serialize_unit_variant("AccessMode", 1u32, "ReadOnlyMany"),
            Self::ReadWriteMany => serializer.serialize_unit_variant("AccessMode", 2u32, "ReadWriteMany"),
            Self::ReadWriteOncePod => serializer.serialize_unit_variant("AccessMode", 3u32, "ReadWriteOncePod"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AccountTier")]
pub enum AccountTier {
    Invalid,
    Standard,
    Premium,
    Ultra,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AccountTier {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AccountTier {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AccountTier {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("AccountTier", 0u32, "Invalid"),
            Self::Standard => serializer.serialize_unit_variant("AccountTier", 1u32, "Standard"),
            Self::Premium => serializer.serialize_unit_variant("AccountTier", 2u32, "Premium"),
            Self::Ultra => serializer.serialize_unit_variant("AccountTier", 3u32, "Ultra"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Action")]
pub enum Action {
    Deny,
    Allow,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Action {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Action {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Deny => serializer.serialize_unit_variant("Action", 0u32, "Deny"),
            Self::Allow => serializer.serialize_unit_variant("Action", 1u32, "Allow"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The location uri of the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomLocationHeader {}
impl CustomLocationHeader {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The retry-after envelope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomRetryAfterHeaderCustom {}
impl CustomRetryAfterHeaderCustom {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Disk Pool Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskPoolProperties {
    #[doc = "List of KV pairs to set in StorageClass to configure CSI driver."]
    #[serde(rename = "csiParams", default, skip_serializing_if = "Option::is_none")]
    pub csi_params: Option<serde_json::Value>,
    #[doc = "Maximum capacity of the volumes in GiB the user intends to create. Default 512."]
    #[serde(rename = "maxVolumeCapacityGiB", default, skip_serializing_if = "Option::is_none")]
    pub max_volume_capacity_gi_b: Option<i64>,
    #[doc = "Only required if individual disk selection is desired. Path to disk, e.g. <nodename>:/dev/sda or WWN. Supports specifying multiple disks (same syntax as tags)."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disks: Vec<String>,
}
impl DiskPoolProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Disk Properties Model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskProperties {
    #[doc = "Reference to Managed Disk"]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "Manufacturer Disk ID"]
    pub wwn: String,
    #[doc = "Capacity of disk in GiB"]
    #[serde(rename = "capacityGiB")]
    pub capacity_gi_b: i64,
    #[serde(rename = "diskType")]
    pub disk_type: DiskType,
    #[doc = "SKU or Manufacturer type/name"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Name of the node disk is attached on"]
    #[serde(rename = "nodeName")]
    pub node_name: String,
    #[doc = "Used to import SSD to discover WWN"]
    #[serde(rename = "devicePath")]
    pub device_path: String,
}
impl DiskProperties {
    pub fn new(
        resource_id: String,
        wwn: String,
        capacity_gi_b: i64,
        disk_type: DiskType,
        display_name: String,
        node_name: String,
        device_path: String,
    ) -> Self {
        Self {
            resource_id,
            wwn,
            capacity_gi_b,
            disk_type,
            display_name,
            node_name,
            device_path,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DiskType")]
pub enum DiskType {
    Local,
    Managed,
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
            Self::Local => serializer.serialize_unit_variant("DiskType", 0u32, "Local"),
            Self::Managed => serializer.serialize_unit_variant("DiskType", 1u32, "Managed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Elastic San Pool Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticSanPoolProperties {
    #[doc = "Name of an existing SAN."]
    #[serde(rename = "sanName")]
    pub san_name: String,
    #[doc = "Resource group of an existing SAN."]
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    #[doc = "Volume group of an existing SAN."]
    #[serde(rename = "volumeGroup")]
    pub volume_group: String,
}
impl ElasticSanPoolProperties {
    pub fn new(san_name: String, resource_group: String, volume_group: String) -> Self {
        Self {
            san_name,
            resource_group,
            volume_group,
        }
    }
}
#[doc = "Elastic San Pool Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticSanPoolPropertiesUpdate {
    #[doc = "Name of an existing SAN."]
    #[serde(rename = "sanName", default, skip_serializing_if = "Option::is_none")]
    pub san_name: Option<String>,
    #[doc = "Resource group of an existing SAN."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "Volume group of an existing SAN."]
    #[serde(rename = "volumeGroup", default, skip_serializing_if = "Option::is_none")]
    pub volume_group: Option<String>,
}
impl ElasticSanPoolPropertiesUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EncryptionType")]
pub enum EncryptionType {
    EncryptionAtRestWithPlatformKey,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EncryptionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EncryptionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EncryptionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::EncryptionAtRestWithPlatformKey => {
                serializer.serialize_unit_variant("EncryptionType", 0u32, "EncryptionAtRestWithPlatformKey")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Ephemeral Pool Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EphemeralPoolProperties {
    #[doc = "Template name or KV pairs containing disk selection criteria, e.g. model=\"Microsoft NVMe Direct Disk\" to match all Lsv2 NVMe disks."]
    #[serde(rename = "diskSelector")]
    pub disk_selector: Vec<String>,
    #[doc = "Only required if individual disk selection is desired. Path to disk, e.g. <nodename>:/dev/sda or WWN. Supports specifying multiple disks (same syntax as tags)."]
    pub disks: Vec<String>,
    #[doc = "Consent to format the local disks."]
    #[serde(rename = "diskFormat", default, skip_serializing_if = "Option::is_none")]
    pub disk_format: Option<bool>,
}
impl EphemeralPoolProperties {
    pub fn new(disk_selector: Vec<String>, disks: Vec<String>) -> Self {
        Self {
            disk_selector,
            disks,
            disk_format: None,
        }
    }
}
#[doc = "Ephemeral Pool Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EphemeralPoolPropertiesUpdate {
    #[doc = "Template name or KV pairs containing disk selection criteria, e.g. model=\"Microsoft NVMe Direct Disk\" to match all Lsv2 NVMe disks."]
    #[serde(
        rename = "diskSelector",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disk_selector: Vec<String>,
    #[doc = "Only required if individual disk selection is desired. Path to disk, e.g. <nodename>:/dev/sda or WWN. Supports specifying multiple disks (same syntax as tags)."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disks: Vec<String>,
    #[doc = "Consent to format the local disks."]
    #[serde(rename = "diskFormat", default, skip_serializing_if = "Option::is_none")]
    pub disk_format: Option<bool>,
}
impl EphemeralPoolPropertiesUpdate {
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
#[doc = "Managed Pool Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedPoolProperties {}
impl ManagedPoolProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network Rule Set definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkRuleSet {
    #[serde(rename = "defaultAction")]
    pub default_action: Action,
    #[doc = "Virtual network rules"]
    #[serde(rename = "virtualNetworkRules")]
    pub virtual_network_rules: Vec<VirtualNetworkRule>,
}
impl NetworkRuleSet {
    pub fn new(default_action: Action, virtual_network_rules: Vec<VirtualNetworkRule>) -> Self {
        Self {
            default_action,
            virtual_network_rules,
        }
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
        self.next_link.clone()
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Placement Properties Model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlacementProperties {
    #[doc = "Affinity label selector"]
    #[serde(rename = "affinityLabelSelector")]
    pub affinity_label_selector: String,
    #[doc = "Anti affinity label selector"]
    #[serde(rename = "antiAffinityLabelSelector")]
    pub anti_affinity_label_selector: String,
    #[serde(rename = "whenUnsatisfiable")]
    pub when_unsatisfiable: WhenUnsatisfiable,
}
impl PlacementProperties {
    pub fn new(affinity_label_selector: String, anti_affinity_label_selector: String, when_unsatisfiable: WhenUnsatisfiable) -> Self {
        Self {
            affinity_label_selector,
            anti_affinity_label_selector,
            when_unsatisfiable,
        }
    }
}
#[doc = "Pool resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pool {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Pool Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PoolProperties>,
}
impl Pool {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response of a Pool list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolListResult {
    #[doc = "The Pool items on this page"]
    pub value: Vec<Pool>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PoolListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PoolListResult {
    pub fn new(value: Vec<Pool>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Pool Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[serde(rename = "poolType")]
    pub pool_type: PoolType,
    #[doc = "List of availability zones that resources can be created in."]
    pub zones: Vec<String>,
    #[doc = "Initial capacity of the pool in GiB."]
    #[serde(rename = "poolCapacityGiB")]
    pub pool_capacity_gi_b: i64,
    #[doc = "List of resources that should have access to the pool. Typically ARM references to AKS clusters or ACI Container Groups. For local and standard this must be a single reference. For portable there can be many."]
    pub assignments: Vec<String>,
    #[doc = "Ephemeral Pool Properties"]
    #[serde(rename = "ephemeralPoolProperties", default, skip_serializing_if = "Option::is_none")]
    pub ephemeral_pool_properties: Option<EphemeralPoolProperties>,
    #[doc = "Disk Pool Properties"]
    #[serde(rename = "diskPoolProperties", default, skip_serializing_if = "Option::is_none")]
    pub disk_pool_properties: Option<DiskPoolProperties>,
    #[doc = "Elastic San Pool Properties"]
    #[serde(rename = "elasticSanPoolProperties")]
    pub elastic_san_pool_properties: ElasticSanPoolProperties,
}
impl PoolProperties {
    pub fn new(
        pool_type: PoolType,
        zones: Vec<String>,
        pool_capacity_gi_b: i64,
        assignments: Vec<String>,
        elastic_san_pool_properties: ElasticSanPoolProperties,
    ) -> Self {
        Self {
            provisioning_state: None,
            pool_type,
            zones,
            pool_capacity_gi_b,
            assignments,
            ephemeral_pool_properties: None,
            disk_pool_properties: None,
            elastic_san_pool_properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PoolType")]
pub enum PoolType {
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PoolType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PoolType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PoolType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The type used for update operations of the Pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PoolUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the Pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PoolUpdateProperties>,
}
impl PoolUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the Pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PoolUpdateProperties {
    #[serde(rename = "poolType", default, skip_serializing_if = "Option::is_none")]
    pub pool_type: Option<PoolType>,
    #[doc = "List of availability zones that resources can be created in."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub zones: Vec<String>,
    #[doc = "Initial capacity of the pool in GiB."]
    #[serde(rename = "poolCapacityGiB", default, skip_serializing_if = "Option::is_none")]
    pub pool_capacity_gi_b: Option<i64>,
    #[doc = "List of resources that should have access to the pool. Typically ARM references to AKS clusters or ACI Container Groups. For local and standard this must be a single reference. For portable there can be many."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub assignments: Vec<String>,
    #[doc = "Ephemeral Pool Properties"]
    #[serde(rename = "ephemeralPoolProperties", default, skip_serializing_if = "Option::is_none")]
    pub ephemeral_pool_properties: Option<EphemeralPoolPropertiesUpdate>,
    #[doc = "Disk Pool Properties"]
    #[serde(rename = "diskPoolProperties", default, skip_serializing_if = "Option::is_none")]
    pub disk_pool_properties: Option<DiskPoolProperties>,
    #[doc = "Elastic San Pool Properties"]
    #[serde(rename = "elasticSanPoolProperties", default, skip_serializing_if = "Option::is_none")]
    pub elastic_san_pool_properties: Option<ElasticSanPoolPropertiesUpdate>,
}
impl PoolUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProtocolType")]
pub enum ProtocolType {
    None,
    #[serde(rename = "iSCSI")]
    IScsi,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProtocolType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProtocolType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProtocolType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ProtocolType", 0u32, "None"),
            Self::IScsi => serializer.serialize_unit_variant("ProtocolType", 1u32, "iSCSI"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Provisioning state of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Provisioning,
    Updating,
    Deleting,
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
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Accepted"),
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReclaimPolicy")]
pub enum ReclaimPolicy {
    Delete,
    Retain,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReclaimPolicy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReclaimPolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReclaimPolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Delete => serializer.serialize_unit_variant("ReclaimPolicy", 0u32, "Delete"),
            Self::Retain => serializer.serialize_unit_variant("ReclaimPolicy", 1u32, "Retain"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
#[doc = "Sku definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: SkuName,
    pub tier: AccountTier,
}
impl Sku {
    pub fn new(name: SkuName, tier: AccountTier) -> Self {
        Self { name, tier }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SkuName")]
pub enum SkuName {
    Invalid,
    #[serde(rename = "Premium_LRS")]
    PremiumLrs,
    #[serde(rename = "Premium_ZRS")]
    PremiumZrs,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SkuName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SkuName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SkuName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Invalid => serializer.serialize_unit_variant("SkuName", 0u32, "Invalid"),
            Self::PremiumLrs => serializer.serialize_unit_variant("SkuName", 1u32, "Premium_LRS"),
            Self::PremiumZrs => serializer.serialize_unit_variant("SkuName", 2u32, "Premium_ZRS"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
#[doc = "Virtual Network Rule definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    #[doc = "Id of rule"]
    pub id: String,
}
impl VirtualNetworkRule {
    pub fn new(id: String) -> Self {
        Self { action: None, id }
    }
}
#[doc = "Concrete proxy resource types can be created by aliasing this type using a specific property type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Volume {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Volume Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VolumeProperties>,
}
impl Volume {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VolumeBindingMode")]
pub enum VolumeBindingMode {
    Immediate,
    WaitFirstForCustomer,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VolumeBindingMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VolumeBindingMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VolumeBindingMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Immediate => serializer.serialize_unit_variant("VolumeBindingMode", 0u32, "Immediate"),
            Self::WaitFirstForCustomer => serializer.serialize_unit_variant("VolumeBindingMode", 1u32, "WaitFirstForCustomer"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The response of a Volume list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeListResult {
    #[doc = "The Volume items on this page"]
    pub value: Vec<Volume>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VolumeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VolumeListResult {
    pub fn new(value: Vec<Volume>) -> Self {
        Self { value, next_link: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VolumeMode")]
pub enum VolumeMode {
    Filesystem,
    Raw,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VolumeMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VolumeMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VolumeMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Filesystem => serializer.serialize_unit_variant("VolumeMode", 0u32, "Filesystem"),
            Self::Raw => serializer.serialize_unit_variant("VolumeMode", 1u32, "Raw"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Volume Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "String KV pairs indicating labels"]
    pub labels: serde_json::Value,
    #[doc = "Requested capacity in GiB"]
    #[serde(rename = "capacityGiB")]
    pub capacity_gi_b: i64,
    #[serde(rename = "volumeMode")]
    pub volume_mode: VolumeMode,
    #[serde(rename = "reclaimPolicy")]
    pub reclaim_policy: ReclaimPolicy,
    #[doc = "List of string mount options"]
    #[serde(rename = "mountOptions")]
    pub mount_options: Vec<String>,
}
impl VolumeProperties {
    pub fn new(
        labels: serde_json::Value,
        capacity_gi_b: i64,
        volume_mode: VolumeMode,
        reclaim_policy: ReclaimPolicy,
        mount_options: Vec<String>,
    ) -> Self {
        Self {
            provisioning_state: None,
            labels,
            capacity_gi_b,
            volume_mode,
            reclaim_policy,
            mount_options,
        }
    }
}
#[doc = "Concrete proxy resource types can be created by aliasing this type using a specific property type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeSnapshot {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Volume Snapshot Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VolumeSnapshotProperties>,
}
impl VolumeSnapshot {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a VolumeSnapshot list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeSnapshotListResult {
    #[doc = "The VolumeSnapshot items on this page"]
    pub value: Vec<VolumeSnapshot>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VolumeSnapshotListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VolumeSnapshotListResult {
    pub fn new(value: Vec<VolumeSnapshot>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Volume Snapshot Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeSnapshotProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Reference to the source volume"]
    pub source: String,
    #[serde(rename = "volumeMode")]
    pub volume_mode: VolumeMode,
    #[serde(rename = "reclaimPolicy")]
    pub reclaim_policy: ReclaimPolicy,
    #[doc = "List of string mount options"]
    #[serde(rename = "mountOptions")]
    pub mount_options: Vec<String>,
}
impl VolumeSnapshotProperties {
    pub fn new(source: String, volume_mode: VolumeMode, reclaim_policy: ReclaimPolicy, mount_options: Vec<String>) -> Self {
        Self {
            provisioning_state: None,
            source,
            volume_mode,
            reclaim_policy,
            mount_options,
        }
    }
}
#[doc = "The type used for update operations of the VolumeSnapshot."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeSnapshotUpdate {
    #[doc = "The updatable properties of the VolumeSnapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VolumeSnapshotUpdateProperties>,
}
impl VolumeSnapshotUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the VolumeSnapshot."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeSnapshotUpdateProperties {
    #[doc = "Reference to the source volume"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "volumeMode", default, skip_serializing_if = "Option::is_none")]
    pub volume_mode: Option<VolumeMode>,
    #[serde(rename = "reclaimPolicy", default, skip_serializing_if = "Option::is_none")]
    pub reclaim_policy: Option<ReclaimPolicy>,
    #[doc = "List of string mount options"]
    #[serde(
        rename = "mountOptions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub mount_options: Vec<String>,
}
impl VolumeSnapshotUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type used for update operations of the Volume."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeUpdate {
    #[doc = "The updatable properties of the Volume."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<VolumeUpdateProperties>,
}
impl VolumeUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the Volume."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeUpdateProperties {
    #[doc = "String KV pairs indicating labels"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<serde_json::Value>,
    #[doc = "Requested capacity in GiB"]
    #[serde(rename = "capacityGiB", default, skip_serializing_if = "Option::is_none")]
    pub capacity_gi_b: Option<i64>,
    #[serde(rename = "volumeMode", default, skip_serializing_if = "Option::is_none")]
    pub volume_mode: Option<VolumeMode>,
    #[serde(rename = "reclaimPolicy", default, skip_serializing_if = "Option::is_none")]
    pub reclaim_policy: Option<ReclaimPolicy>,
    #[doc = "List of string mount options"]
    #[serde(
        rename = "mountOptions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub mount_options: Vec<String>,
}
impl VolumeUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WhenUnsatisfiable")]
pub enum WhenUnsatisfiable {
    DoNotSchedule,
    ScheduleAnyway,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WhenUnsatisfiable {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WhenUnsatisfiable {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WhenUnsatisfiable {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::DoNotSchedule => serializer.serialize_unit_variant("WhenUnsatisfiable", 0u32, "DoNotSchedule"),
            Self::ScheduleAnyway => serializer.serialize_unit_variant("WhenUnsatisfiable", 1u32, "ScheduleAnyway"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
