#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Assignment Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Assignment {
    #[doc = "Status of the assignment resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AssignmentStatus>,
    #[doc = "A type definition that refers the id to an ARM resource."]
    pub id: AssignmentId,
}
impl Assignment {
    pub fn new(id: AssignmentId) -> Self {
        Self { status: None, id }
    }
}
pub type AssignmentId = String;
#[doc = "Status of the assignment resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignmentStatus {
    #[doc = "Status of the assignment resource"]
    pub state: AssignmentStatusState,
    #[doc = "Reason for the status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl AssignmentStatus {
    pub fn new(state: AssignmentStatusState) -> Self {
        Self { state, message: None }
    }
}
#[doc = "Status of the assignment resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AssignmentStatusState")]
pub enum AssignmentStatusState {
    Assigning,
    Assigned,
    Unassigning,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AssignmentStatusState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AssignmentStatusState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AssignmentStatusState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Assigning => serializer.serialize_unit_variant("AssignmentStatusState", 0u32, "Assigning"),
            Self::Assigned => serializer.serialize_unit_variant("AssignmentStatusState", 1u32, "Assigned"),
            Self::Unassigning => serializer.serialize_unit_variant("AssignmentStatusState", 2u32, "Unassigning"),
            Self::Failed => serializer.serialize_unit_variant("AssignmentStatusState", 3u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Azure Disk Pool Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDisk {
    #[doc = "Managed resource group for the pool."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "SKU of the underlying managed disk"]
    #[serde(rename = "skuName", default, skip_serializing_if = "Option::is_none")]
    pub sku_name: Option<AzureDiskSkuName>,
    #[doc = "Encryption key properties for the pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[doc = "Only required if individual disk selection is desired. Path to disk, e.g. <nodename>:/dev/sda or WWN. Supports specifying multiple disks (same syntax as tags)."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disks: Vec<Disk>,
}
impl AzureDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SKU of the underlying managed disk"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureDiskSkuName")]
pub enum AzureDiskSkuName {
    #[serde(rename = "Premium_LRS")]
    PremiumLrs,
    #[serde(rename = "Standard_LRS")]
    StandardLrs,
    #[serde(rename = "StandardSSD_LRS")]
    StandardSsdLrs,
    #[serde(rename = "UltraSSD_LRS")]
    UltraSsdLrs,
    #[serde(rename = "Premium_ZRS")]
    PremiumZrs,
    #[serde(rename = "PremiumV2_LRS")]
    PremiumV2Lrs,
    #[serde(rename = "StandardSSD_ZRS")]
    StandardSsdZrs,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureDiskSkuName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureDiskSkuName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureDiskSkuName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::PremiumLrs => serializer.serialize_unit_variant("AzureDiskSkuName", 0u32, "Premium_LRS"),
            Self::StandardLrs => serializer.serialize_unit_variant("AzureDiskSkuName", 1u32, "Standard_LRS"),
            Self::StandardSsdLrs => serializer.serialize_unit_variant("AzureDiskSkuName", 2u32, "StandardSSD_LRS"),
            Self::UltraSsdLrs => serializer.serialize_unit_variant("AzureDiskSkuName", 3u32, "UltraSSD_LRS"),
            Self::PremiumZrs => serializer.serialize_unit_variant("AzureDiskSkuName", 4u32, "Premium_ZRS"),
            Self::PremiumV2Lrs => serializer.serialize_unit_variant("AzureDiskSkuName", 5u32, "PremiumV2_LRS"),
            Self::StandardSsdZrs => serializer.serialize_unit_variant("AzureDiskSkuName", 6u32, "StandardSSD_ZRS"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Azure Disk Pool Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureDiskUpdate {
    #[doc = "Only required if individual disk selection is desired. Path to disk, e.g. <nodename>:/dev/sda or WWN. Supports specifying multiple disks (same syntax as tags)."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disks: Vec<Disk>,
}
impl AzureDiskUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model for disk for that pool is using"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Disk {
    #[doc = "ID is the disk identifier visible to the OS. It is typically the WWN or disk ID in formats such as eui.e8238fa6bf530001001b448b45263379 or 0x5002cf6cbc5dd460"]
    pub id: String,
    #[doc = "Reference is the location of the disk in an external system."]
    pub reference: String,
}
impl Disk {
    pub fn new(id: String, reference: String) -> Self {
        Self { id, reference }
    }
}
#[doc = "Properties shared between the azureDisk and ephemeralDisk"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskPoolProperties {
    #[doc = "Only required if individual disk selection is desired. Path to disk, e.g. <nodename>:/dev/sda or WWN. Supports specifying multiple disks (same syntax as tags)."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disks: Vec<Disk>,
}
impl DiskPoolProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Elastic San Pool Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticSan {
    #[doc = "Managed resource group for the pool."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "ElasticSAN SKUs"]
    #[serde(rename = "skuName", default, skip_serializing_if = "Option::is_none")]
    pub sku_name: Option<ElasticSanSkuName>,
    #[doc = "Encryption key properties for the pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
}
impl ElasticSan {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ElasticSAN SKUs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ElasticSanSkuName")]
pub enum ElasticSanSkuName {
    #[serde(rename = "Premium_LRS")]
    PremiumLrs,
    #[serde(rename = "Premium_ZRS")]
    PremiumZrs,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ElasticSanSkuName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ElasticSanSkuName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ElasticSanSkuName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::PremiumLrs => serializer.serialize_unit_variant("ElasticSanSkuName", 0u32, "Premium_LRS"),
            Self::PremiumZrs => serializer.serialize_unit_variant("ElasticSanSkuName", 1u32, "Premium_ZRS"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Elastic San Pool Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticSanUpdate {}
impl ElasticSanUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the ElasticSAN iSCSI target"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticSanVolumeProperties {
    #[doc = "iSCSI Target IQN (iSCSI Qualified Name); example: \"iqn.2005-03.org.iscsi:server\""]
    #[serde(rename = "targetIqn")]
    pub target_iqn: String,
    #[doc = "iSCSI Target Portal Host Name"]
    #[serde(rename = "targetPortalHostname")]
    pub target_portal_hostname: String,
    #[doc = "iSCSI Target Portal Port"]
    #[serde(rename = "targetPortalPort")]
    pub target_portal_port: i32,
}
impl ElasticSanVolumeProperties {
    pub fn new(target_iqn: String, target_portal_hostname: String, target_portal_port: i32) -> Self {
        Self {
            target_iqn,
            target_portal_hostname,
            target_portal_port,
        }
    }
}
#[doc = "Encryption key properties for the pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Encryption {
    #[doc = "The name of the key vault key."]
    #[serde(rename = "keyName")]
    pub key_name: String,
    #[doc = "The URI of the key vault."]
    #[serde(rename = "keyVaultUri")]
    pub key_vault_uri: String,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl Encryption {
    pub fn new(key_name: String, key_vault_uri: String) -> Self {
        Self {
            key_name,
            key_vault_uri,
            identity: None,
        }
    }
}
#[doc = "Ephemeral Disk Pool Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EphemeralDisk {
    #[doc = "The number of data copies. Default 3."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i64>,
    #[doc = "Only required if individual disk selection is desired. Path to disk, e.g. <nodename>:/dev/sda or WWN. Supports specifying multiple disks (same syntax as tags)."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub disks: Vec<Disk>,
}
impl EphemeralDisk {
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
#[doc = "Managed service identity (system assigned and/or user assigned identities)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedServiceIdentity {
    #[doc = "The service principal ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
    #[serde(rename = "type")]
    pub type_: ManagedServiceIdentityType,
    #[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedIdentities>,
}
impl ManagedServiceIdentity {
    pub fn new(type_: ManagedServiceIdentityType) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
            user_assigned_identities: None,
        }
    }
}
#[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedServiceIdentityType")]
pub enum ManagedServiceIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
    #[serde(rename = "SystemAssigned, UserAssigned")]
    SystemAssignedUserAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagedServiceIdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagedServiceIdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagedServiceIdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ManagedServiceIdentityType", 0u32, "None"),
            Self::SystemAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 1u32, "SystemAssigned"),
            Self::UserAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 2u32, "UserAssigned"),
            Self::SystemAssignedUserAssigned => {
                serializer.serialize_unit_variant("ManagedServiceIdentityType", 3u32, "SystemAssigned, UserAssigned")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
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
        self.next_link.clone().filter(|value| !value.is_empty())
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
    #[doc = "Status of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResourceOperationalStatus>,
    #[doc = "List of availability zones that resources can be created in."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub zones: Vec<Zone>,
    #[doc = "Resource Requests for the pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Resources>,
    #[doc = "Type of the Pool: ephemeralDisk, azureDisk, or elasticsan"]
    #[serde(rename = "poolType")]
    pub pool_type: PoolType,
    #[doc = "Reclaim policy"]
    #[serde(rename = "reclaimPolicy", default, skip_serializing_if = "Option::is_none")]
    pub reclaim_policy: Option<ReclaimPolicy>,
    #[doc = "List of resources that should have access to the pool. Typically ARM references to AKS clusters or ACI Container Groups. For local and standard this must be a single reference. For ElasticSAN there can be many."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub assignments: Vec<Assignment>,
}
impl PoolProperties {
    pub fn new(pool_type: PoolType) -> Self {
        Self {
            provisioning_state: None,
            status: None,
            zones: Vec::new(),
            resources: None,
            pool_type,
            reclaim_policy: None,
            assignments: Vec::new(),
        }
    }
}
#[doc = "Type of the Pool: ephemeralDisk, azureDisk, or elasticsan"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PoolType {
    #[doc = "Azure Disk Pool Properties"]
    #[serde(rename = "azureDisk", default, skip_serializing_if = "Option::is_none")]
    pub azure_disk: Option<AzureDisk>,
    #[doc = "Elastic San Pool Properties"]
    #[serde(rename = "elasticSan", default, skip_serializing_if = "Option::is_none")]
    pub elastic_san: Option<ElasticSan>,
    #[doc = "Ephemeral Disk Pool Properties"]
    #[serde(rename = "ephemeralDisk", default, skip_serializing_if = "Option::is_none")]
    pub ephemeral_disk: Option<EphemeralDisk>,
}
impl PoolType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of the Pool: ephemeralDisk, azureDisk, or elasticsan"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PoolTypeUpdate {
    #[doc = "Azure Disk Pool Properties"]
    #[serde(rename = "azureDisk", default, skip_serializing_if = "Option::is_none")]
    pub azure_disk: Option<AzureDiskUpdate>,
    #[doc = "Elastic San Pool Properties"]
    #[serde(rename = "elasticSan", default, skip_serializing_if = "Option::is_none")]
    pub elastic_san: Option<ElasticSanUpdate>,
    #[doc = "Ephemeral Disk Pool Properties"]
    #[serde(rename = "ephemeralDisk", default, skip_serializing_if = "Option::is_none")]
    pub ephemeral_disk: Option<EphemeralDisk>,
}
impl PoolTypeUpdate {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "Resource Requests for the pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Resources>,
    #[doc = "Type of the Pool: ephemeralDisk, azureDisk, or elasticsan"]
    #[serde(rename = "poolType", default, skip_serializing_if = "Option::is_none")]
    pub pool_type: Option<PoolTypeUpdate>,
    #[doc = "List of resources that should have access to the pool. Typically ARM references to AKS clusters or ACI Container Groups. For local and standard this must be a single reference. For ElasticSAN there can be many."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub assignments: Vec<Assignment>,
}
impl PoolUpdateProperties {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Reclaim policy"]
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
#[doc = "Requests for capacity for the pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Requests {
    #[doc = "Requested capacity of the pool in GiB."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<i64>,
}
impl Requests {
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
#[doc = "State of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResourceOperationStatusState")]
pub enum ResourceOperationStatusState {
    Pending,
    Available,
    Unavailable,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResourceOperationStatusState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResourceOperationStatusState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResourceOperationStatusState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pending => serializer.serialize_unit_variant("ResourceOperationStatusState", 0u32, "Pending"),
            Self::Available => serializer.serialize_unit_variant("ResourceOperationStatusState", 1u32, "Available"),
            Self::Unavailable => serializer.serialize_unit_variant("ResourceOperationStatusState", 2u32, "Unavailable"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Status of the resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceOperationalStatus {
    #[doc = "State of the resource."]
    pub state: ResourceOperationStatusState,
    #[doc = "Reason for state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ResourceOperationalStatus {
    pub fn new(state: ResourceOperationStatusState) -> Self {
        Self { state, message: None }
    }
}
#[doc = "Resource Requests for the pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resources {
    #[doc = "Requests for capacity for the pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<Requests>,
}
impl Resources {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Concrete proxy resource types can be created by aliasing this type using a specific property type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Snapshot {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Volume Snapshot Properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SnapshotProperties>,
}
impl Snapshot {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a Snapshot list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotListResult {
    #[doc = "The Snapshot items on this page"]
    pub value: Vec<Snapshot>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SnapshotListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SnapshotListResult {
    pub fn new(value: Vec<Snapshot>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Volume Snapshot Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Status of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResourceOperationalStatus>,
    #[doc = "Reference to the source volume"]
    pub source: String,
}
impl SnapshotProperties {
    pub fn new(source: String) -> Self {
        Self {
            provisioning_state: None,
            status: None,
            source,
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
#[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentities {}
impl UserAssignedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User assigned identity properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "The principal ID of the assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client ID of the assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
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
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VolumeListResult {
    pub fn new(value: Vec<Volume>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Volume Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Status of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ResourceOperationalStatus>,
    #[doc = "String KV pairs indicating labels"]
    pub labels: serde_json::Value,
    #[doc = "Requested capacity in GiB"]
    #[serde(rename = "capacityGiB")]
    pub capacity_gi_b: i64,
    #[doc = "Properties of the volume"]
    #[serde(rename = "volumeType", default, skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<VolumeType>,
}
impl VolumeProperties {
    pub fn new(labels: serde_json::Value, capacity_gi_b: i64) -> Self {
        Self {
            provisioning_state: None,
            status: None,
            labels,
            capacity_gi_b,
            volume_type: None,
        }
    }
}
#[doc = "Properties of the volume"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VolumeType {
    #[doc = "Properties of the ElasticSAN iSCSI target"]
    #[serde(rename = "elasticSan", default, skip_serializing_if = "Option::is_none")]
    pub elastic_san: Option<ElasticSanVolumeProperties>,
}
impl VolumeType {
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
}
impl VolumeUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Availability Zones"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Zone")]
pub enum Zone {
    #[serde(rename = "1")]
    N1,
    #[serde(rename = "2")]
    N2,
    #[serde(rename = "3")]
    N3,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Zone {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Zone {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Zone {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::N1 => serializer.serialize_unit_variant("Zone", 0u32, "1"),
            Self::N2 => serializer.serialize_unit_variant("Zone", 1u32, "2"),
            Self::N3 => serializer.serialize_unit_variant("Zone", 2u32, "3"),
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
