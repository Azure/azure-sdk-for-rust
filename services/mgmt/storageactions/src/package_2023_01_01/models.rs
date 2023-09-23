#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The else block of storage task operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElseCondition {
    #[doc = "List of operations to execute in the else block"]
    pub operations: Vec<StorageTaskOperation>,
}
impl ElseCondition {
    pub fn new(operations: Vec<StorageTaskOperation>) -> Self {
        Self { operations }
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
#[doc = "The if block of storage task operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IfCondition {
    #[doc = "The condition predicate which is composed of object properties, eg: blob and container properties."]
    pub condition: String,
    #[doc = "List of operations to execute when the condition predicate satisfies."]
    pub operations: Vec<StorageTaskOperation>,
}
impl IfCondition {
    pub fn new(condition: String, operations: Vec<StorageTaskOperation>) -> Self {
        Self { condition, operations }
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
    #[serde(rename = "SystemAssigned,UserAssigned")]
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
                serializer.serialize_unit_variant("ManagedServiceIdentityType", 3u32, "SystemAssigned,UserAssigned")
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
#[doc = "Represents Storage Task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageTask {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Properties of the storage task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageTaskProperties>,
}
impl StorageTask {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            identity: None,
            properties: None,
        }
    }
}
#[doc = "The storage task action represents conditional statements and operations to be performed on target objects."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageTaskAction {
    #[doc = "The if block of storage task operation"]
    #[serde(rename = "if")]
    pub if_: IfCondition,
    #[doc = "The else block of storage task operation"]
    #[serde(rename = "else", default, skip_serializing_if = "Option::is_none")]
    pub else_: Option<ElseCondition>,
}
impl StorageTaskAction {
    pub fn new(if_: IfCondition) -> Self {
        Self { if_, else_: None }
    }
}
#[doc = "Fetch the Storage task assignment ARM ids."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageTaskAssignment {
    #[doc = "ARM Id of the storage task assignments, associated with the storage tasks."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl StorageTaskAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response from the List Storage Tasks operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageTaskAssignmentsListResult {
    #[doc = "Gets the list of storage task assignment Ids."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<StorageTaskAssignment>,
    #[doc = "Request URL that can be used to query next page of storage task assignment Ids. Returned when total number of requested storage task assignment Ids exceed maximum page size."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StorageTaskAssignmentsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl StorageTaskAssignmentsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an operation to be performed on the object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageTaskOperation {
    #[doc = "The operation to be performed on the object."]
    pub name: storage_task_operation::Name,
    #[doc = "Key-value parameters for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Action to be taken when the operation is successful for a object."]
    #[serde(rename = "onSuccess", default, skip_serializing_if = "Option::is_none")]
    pub on_success: Option<storage_task_operation::OnSuccess>,
    #[doc = "Action to be taken when the operation fails for a object."]
    #[serde(rename = "onFailure", default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<storage_task_operation::OnFailure>,
}
impl StorageTaskOperation {
    pub fn new(name: storage_task_operation::Name) -> Self {
        Self {
            name,
            parameters: None,
            on_success: None,
            on_failure: None,
        }
    }
}
pub mod storage_task_operation {
    use super::*;
    #[doc = "The operation to be performed on the object."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        SetBlobTier,
        SetBlobTags,
        SetBlobImmutabilityPolicy,
        SetBlobLegalHold,
        SetBlobExpiry,
        DeleteBlob,
        UndeleteBlob,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Name {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Name {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SetBlobTier => serializer.serialize_unit_variant("Name", 0u32, "SetBlobTier"),
                Self::SetBlobTags => serializer.serialize_unit_variant("Name", 1u32, "SetBlobTags"),
                Self::SetBlobImmutabilityPolicy => serializer.serialize_unit_variant("Name", 2u32, "SetBlobImmutabilityPolicy"),
                Self::SetBlobLegalHold => serializer.serialize_unit_variant("Name", 3u32, "SetBlobLegalHold"),
                Self::SetBlobExpiry => serializer.serialize_unit_variant("Name", 4u32, "SetBlobExpiry"),
                Self::DeleteBlob => serializer.serialize_unit_variant("Name", 5u32, "DeleteBlob"),
                Self::UndeleteBlob => serializer.serialize_unit_variant("Name", 6u32, "UndeleteBlob"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Action to be taken when the operation is successful for a object."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OnSuccess {
        #[serde(rename = "continue")]
        Continue,
    }
    #[doc = "Action to be taken when the operation fails for a object."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OnFailure {
        #[serde(rename = "break")]
        Break,
    }
}
#[doc = "Storage Task Preview Action."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageTaskPreviewAction {
    #[doc = "Storage task preview action properties."]
    pub properties: StorageTaskPreviewActionProperties,
}
impl StorageTaskPreviewAction {
    pub fn new(properties: StorageTaskPreviewActionProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Represents the storage task conditions to be tested for a match with container and blob properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageTaskPreviewActionCondition {
    #[doc = "Represents storage task preview action condition."]
    #[serde(rename = "if")]
    pub if_: StorageTaskPreviewActionIfCondition,
    #[doc = "Specify whether the else block is present in the condition."]
    #[serde(rename = "elseBlockExists")]
    pub else_block_exists: bool,
}
impl StorageTaskPreviewActionCondition {
    pub fn new(if_: StorageTaskPreviewActionIfCondition, else_block_exists: bool) -> Self {
        Self { if_, else_block_exists }
    }
}
#[doc = "Represents storage task preview action condition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageTaskPreviewActionIfCondition {
    #[doc = "Storage task condition to bes tested for a match."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
}
impl StorageTaskPreviewActionIfCondition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage task preview action properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageTaskPreviewActionProperties {
    #[doc = "Storage task preview container properties"]
    pub container: StorageTaskPreviewContainerProperties,
    #[doc = "Preview action container properties to be tested for a match with the provided condition."]
    pub blobs: Vec<StorageTaskPreviewBlobProperties>,
    #[doc = "Represents the storage task conditions to be tested for a match with container and blob properties."]
    pub action: StorageTaskPreviewActionCondition,
}
impl StorageTaskPreviewActionProperties {
    pub fn new(
        container: StorageTaskPreviewContainerProperties,
        blobs: Vec<StorageTaskPreviewBlobProperties>,
        action: StorageTaskPreviewActionCondition,
    ) -> Self {
        Self { container, blobs, action }
    }
}
#[doc = "Storage task preview container properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageTaskPreviewBlobProperties {
    #[doc = "property for the container name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "properties key value pairs to be tested for a match against the provided condition."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub properties: Vec<StorageTaskPreviewKeyValueProperties>,
    #[doc = "metadata key value pairs to be tested for a match against the provided condition."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metadata: Vec<StorageTaskPreviewKeyValueProperties>,
    #[doc = "tags key value pairs to be tested for a match against the provided condition."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tags: Vec<StorageTaskPreviewKeyValueProperties>,
    #[doc = "Represents the condition block name that matched blob properties."]
    #[serde(rename = "matchedBlock", default, skip_serializing_if = "Option::is_none")]
    pub matched_block: Option<storage_task_preview_blob_properties::MatchedBlock>,
}
impl StorageTaskPreviewBlobProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_task_preview_blob_properties {
    use super::*;
    #[doc = "Represents the condition block name that matched blob properties."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MatchedBlock")]
    pub enum MatchedBlock {
        If,
        Else,
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MatchedBlock {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MatchedBlock {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MatchedBlock {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::If => serializer.serialize_unit_variant("MatchedBlock", 0u32, "If"),
                Self::Else => serializer.serialize_unit_variant("MatchedBlock", 1u32, "Else"),
                Self::None => serializer.serialize_unit_variant("MatchedBlock", 2u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Storage task preview container properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageTaskPreviewContainerProperties {
    #[doc = "property for the container name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "metadata key value pairs to be tested for a match against the provided condition."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metadata: Vec<StorageTaskPreviewKeyValueProperties>,
}
impl StorageTaskPreviewContainerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage task preview object key value pair properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageTaskPreviewKeyValueProperties {
    #[doc = "Represents the key property of the pair."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "Represents the value property of the pair."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl StorageTaskPreviewKeyValueProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the storage task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageTaskProperties {
    #[doc = "Storage task version."]
    #[serde(rename = "taskVersion", default, skip_serializing_if = "Option::is_none")]
    pub task_version: Option<i64>,
    #[doc = "Storage Task is enabled when set to true and disabled when set to false"]
    pub enabled: bool,
    #[doc = "Text that describes the purpose of the storage task"]
    pub description: String,
    #[doc = "The storage task action represents conditional statements and operations to be performed on target objects."]
    pub action: StorageTaskAction,
    #[doc = "Represents the provisioning state of the storage task."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<storage_task_properties::ProvisioningState>,
    #[doc = "The creation date and time of the storage task in UTC."]
    #[serde(rename = "creationTimeInUtc", default, with = "azure_core::date::rfc3339::option")]
    pub creation_time_in_utc: Option<time::OffsetDateTime>,
}
impl StorageTaskProperties {
    pub fn new(enabled: bool, description: String, action: StorageTaskAction) -> Self {
        Self {
            task_version: None,
            enabled,
            description,
            action,
            provisioning_state: None,
            creation_time_in_utc: None,
        }
    }
}
pub mod storage_task_properties {
    use super::*;
    #[doc = "Represents the provisioning state of the storage task."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        ValidateSubscriptionQuotaBegin,
        ValidateSubscriptionQuotaEnd,
        Creating,
        Succeeded,
        Deleting,
        Canceled,
        Failed,
    }
}
#[doc = "Storage Tasks run report instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageTaskReportInstance {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Storage task execution report for a run instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageTaskReportProperties>,
}
impl StorageTaskReportInstance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage task execution report for a run instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageTaskReportProperties {
    #[doc = "Represents the Storage Task Assignment Id associated with the storage task that provided an execution context."]
    #[serde(rename = "taskAssignmentId", default, skip_serializing_if = "Option::is_none")]
    pub task_assignment_id: Option<String>,
    #[doc = "Represents the Storage Account Id where the storage task definition was applied and executed."]
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[doc = "Start time of the run instance. Filter options such as startTime gt '2023-06-26T20:51:24.4494016Z' and other comparison operators can be used as described for DateTime properties in https://learn.microsoft.com/en-us/rest/api/storageservices/querying-tables-and-entities#supported-comparison-operators"]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "End time of the run instance. Filter options such as startTime gt '2023-06-26T20:51:24.4494016Z' and other comparison operators can be used as described for DateTime properties in https://learn.microsoft.com/en-us/rest/api/storageservices/querying-tables-and-entities#supported-comparison-operators"]
    #[serde(rename = "finishTime", default, skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<String>,
    #[doc = "Total number of objects that meet the condition as defined in the storage task assignment execution context. Filter options such as objectsTargetedCount gt 50 and other comparison operators can be used as described for Numerical properties in https://learn.microsoft.com/en-us/rest/api/storageservices/querying-tables-and-entities#supported-comparison-operators"]
    #[serde(rename = "objectsTargetedCount", default, skip_serializing_if = "Option::is_none")]
    pub objects_targeted_count: Option<String>,
    #[doc = "Total number of objects that meet the storage tasks condition and were operated upon. Filter options such as objectsOperatedOnCount ge 100 and other comparison operators can be used as described for Numerical properties in https://learn.microsoft.com/en-us/rest/api/storageservices/querying-tables-and-entities#supported-comparison-operators"]
    #[serde(rename = "objectsOperatedOnCount", default, skip_serializing_if = "Option::is_none")]
    pub objects_operated_on_count: Option<String>,
    #[doc = "Total number of objects where task operation failed when was attempted. Filter options such as objectFailedCount eq 0 and other comparison operators can be used as described for Numerical properties in https://learn.microsoft.com/en-us/rest/api/storageservices/querying-tables-and-entities#supported-comparison-operators"]
    #[serde(rename = "objectFailedCount", default, skip_serializing_if = "Option::is_none")]
    pub object_failed_count: Option<String>,
    #[doc = "Total number of objects where task operation succeeded when was attempted.Filter options such as objectsSucceededCount gt 150 and other comparison operators can be used as described for Numerical properties in https://learn.microsoft.com/en-us/rest/api/storageservices/querying-tables-and-entities#supported-comparison-operators"]
    #[serde(rename = "objectsSucceededCount", default, skip_serializing_if = "Option::is_none")]
    pub objects_succeeded_count: Option<String>,
    #[doc = "Well known Azure Storage error code that represents the error encountered during execution of the run instance."]
    #[serde(rename = "runStatusError", default, skip_serializing_if = "Option::is_none")]
    pub run_status_error: Option<String>,
    #[doc = "Represents the status of the execution."]
    #[serde(rename = "runStatusEnum", default, skip_serializing_if = "Option::is_none")]
    pub run_status_enum: Option<storage_task_report_properties::RunStatusEnum>,
    #[doc = "Full path to the verbose report stored in the reporting container as specified in the assignment execution context for the storage account. "]
    #[serde(rename = "summaryReportPath", default, skip_serializing_if = "Option::is_none")]
    pub summary_report_path: Option<String>,
    #[doc = "Storage Task Arm Id."]
    #[serde(rename = "taskId", default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[doc = "Storage Task Version"]
    #[serde(rename = "taskVersion", default, skip_serializing_if = "Option::is_none")]
    pub task_version: Option<String>,
    #[doc = "Represents the overall result of the execution for the run instance"]
    #[serde(rename = "runResult", default, skip_serializing_if = "Option::is_none")]
    pub run_result: Option<storage_task_report_properties::RunResult>,
}
impl StorageTaskReportProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_task_report_properties {
    use super::*;
    #[doc = "Represents the status of the execution."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RunStatusEnum")]
    pub enum RunStatusEnum {
        InProgress,
        Finished,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RunStatusEnum {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RunStatusEnum {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RunStatusEnum {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::InProgress => serializer.serialize_unit_variant("RunStatusEnum", 0u32, "InProgress"),
                Self::Finished => serializer.serialize_unit_variant("RunStatusEnum", 1u32, "Finished"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Represents the overall result of the execution for the run instance"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RunResult")]
    pub enum RunResult {
        Succeeded,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RunResult {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RunResult {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RunResult {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("RunResult", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("RunResult", 1u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Fetch Storage Tasks Run Summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageTaskReportSummary {
    #[doc = "Gets storage tasks run result summary."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<StorageTaskReportInstance>,
    #[doc = "Request URL that can be used to query next page of storage task run results summary. Returned when the number of run instances and summary reports exceed maximum page size."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StorageTaskReportSummary {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl StorageTaskReportSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters of the storage task update request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageTaskUpdateParameters {
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Gets or sets a list of key value pairs that describe the resource. These tags can be used in viewing and grouping this resource (across resource groups). A maximum of 15 tags can be provided for a resource. Each tag must have a key no greater in length than 128 characters and a value no greater in length than 256 characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties of the storage task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageTaskProperties>,
}
impl StorageTaskUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response from the List Storage Tasks operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageTasksListResult {
    #[doc = "Gets the list of storage tasks and their properties."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<StorageTask>,
    #[doc = "Request URL that can be used to query next page of storage tasks. Returned when total number of requested storage tasks exceed maximum page size."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StorageTasksListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl StorageTasksListResult {
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
