#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The detected change."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Change {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a change."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ChangeProperties>,
}
impl Change {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of detected changes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangeList {
    #[doc = "The list of changes."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Change>,
    #[doc = "The URI that can be used to request the next page of changes."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ChangeList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ChangeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a change."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangeProperties {
    #[doc = "The resource id that the change is attached to."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The time when the change is detected."]
    #[serde(rename = "timeStamp", with = "azure_core::date::rfc3339::option")]
    pub time_stamp: Option<time::OffsetDateTime>,
    #[doc = "The list of identities who might initiated the change.\r\nThe identity could be user name (email address) or the object ID of the Service Principal."]
    #[serde(rename = "initiatedByList", default, skip_serializing_if = "Vec::is_empty")]
    pub initiated_by_list: Vec<String>,
    #[doc = "The type of the change."]
    #[serde(rename = "changeType", default, skip_serializing_if = "Option::is_none")]
    pub change_type: Option<ChangeType>,
    #[doc = "The list of detailed changes at json property level."]
    #[serde(rename = "propertyChanges", default, skip_serializing_if = "Vec::is_empty")]
    pub property_changes: Vec<PropertyChange>,
}
impl ChangeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The detected change snapshots."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangeSnapshots {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The change snapshot, represented by a pair of before and after resource snapshots."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ChangeSnapshotsProperties>,
}
impl ChangeSnapshots {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The change snapshot, represented by a pair of before and after resource snapshots."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangeSnapshotsProperties {
    #[doc = "The snapshot before the change."]
    #[serde(rename = "beforeSnapshot", default, skip_serializing_if = "Option::is_none")]
    pub before_snapshot: Option<serde_json::Value>,
    #[doc = "The snapshot after the change."]
    #[serde(rename = "afterSnapshot", default, skip_serializing_if = "Option::is_none")]
    pub after_snapshot: Option<serde_json::Value>,
    #[doc = "Is the snapshot hidden"]
    #[serde(rename = "isHidden", default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
}
impl ChangeSnapshotsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of the change."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ChangeType")]
pub enum ChangeType {
    Add,
    Remove,
    Update,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ChangeType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ChangeType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ChangeType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Add => serializer.serialize_unit_variant("ChangeType", 0u32, "Add"),
            Self::Remove => serializer.serialize_unit_variant("ChangeType", 1u32, "Remove"),
            Self::Update => serializer.serialize_unit_variant("ChangeType", 2u32, "Update"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
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
#[serde(remote = "Level")]
pub enum Level {
    Noisy,
    Normal,
    Important,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Level {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Level {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Level {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Noisy => serializer.serialize_unit_variant("Level", 0u32, "Noisy"),
            Self::Normal => serializer.serialize_unit_variant("Level", 1u32, "Normal"),
            Self::Important => serializer.serialize_unit_variant("Level", 2u32, "Important"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Data of a property change."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PropertyChange {
    #[doc = "The type of the change."]
    #[serde(rename = "changeType", default, skip_serializing_if = "Option::is_none")]
    pub change_type: Option<ChangeType>,
    #[doc = "The change category."]
    #[serde(rename = "changeCategory", default, skip_serializing_if = "Option::is_none")]
    pub change_category: Option<property_change::ChangeCategory>,
    #[doc = "The json path of the changed property."]
    #[serde(rename = "jsonPath", default, skip_serializing_if = "Option::is_none")]
    pub json_path: Option<String>,
    #[doc = "The enhanced display name of the json path. E.g., the json path value[0].properties will be translated to something meaningful like slots[\"Staging\"].properties."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,
    #[doc = "The description of the changed property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The value of the property before the change."]
    #[serde(rename = "oldValue", default, skip_serializing_if = "Option::is_none")]
    pub old_value: Option<String>,
    #[doc = "The value of the property after the change."]
    #[serde(rename = "newValue", default, skip_serializing_if = "Option::is_none")]
    pub new_value: Option<String>,
    #[doc = "The boolean indicating whether the oldValue and newValue are masked. The values are masked if it contains sensitive information that the user doesn't have access to."]
    #[serde(rename = "isDataMasked", default, skip_serializing_if = "Option::is_none")]
    pub is_data_masked: Option<bool>,
}
impl PropertyChange {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod property_change {
    use super::*;
    #[doc = "The change category."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ChangeCategory {
        User,
        System,
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
#[doc = "Data on a specific change, represented by a pair of before and after resource snapshots."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGraphChangeData {
    #[doc = "The resource for a change."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The change Id."]
    #[serde(rename = "changeId", default, skip_serializing_if = "Option::is_none")]
    pub change_id: Option<String>,
    #[doc = "The snapshot before the change from ARG."]
    #[serde(rename = "beforeSnapshot", default, skip_serializing_if = "Option::is_none")]
    pub before_snapshot: Option<serde_json::Value>,
    #[doc = "The snapshot after the change from ARG."]
    #[serde(rename = "afterSnapshot", default, skip_serializing_if = "Option::is_none")]
    pub after_snapshot: Option<serde_json::Value>,
    #[doc = "The change type for snapshot. PropertyChanges will be provided in case of Update change type"]
    #[serde(rename = "changeType", default, skip_serializing_if = "Option::is_none")]
    pub change_type: Option<resource_graph_change_data::ChangeType>,
}
impl ResourceGraphChangeData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_graph_change_data {
    use super::*;
    #[doc = "The change type for snapshot. PropertyChanges will be provided in case of Update change type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ChangeType {
        Create,
        Update,
        Delete,
    }
}
#[doc = "Data on a specific resource snapshot."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceGraphSnapshotData {
    #[doc = "The ID of the snapshot."]
    #[serde(rename = "snapshotId", default, skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[doc = "The time when the snapshot was created.\nThe snapshot timestamp provides an approximation as to when a modification to a resource was detected.  There can be a difference between the actual modification time and the detection time.  This is due to differences in how operations that modify a resource are processed, versus how operation that record resource snapshots are processed."]
    #[serde(with = "azure_core::date::rfc3339")]
    pub timestamp: time::OffsetDateTime,
    #[doc = "The resource snapshot content (in resourceChangeDetails response only)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
}
impl ResourceGraphSnapshotData {
    pub fn new(timestamp: time::OffsetDateTime) -> Self {
        Self {
            snapshot_id: None,
            timestamp,
            content: None,
        }
    }
}
#[doc = "The resource provider operation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperationDefinition {
    #[doc = "The resource provider operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource provider operation details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<ResourceProviderOperationDisplay>,
}
impl ResourceProviderOperationDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource provider operation details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperationDisplay {
    #[doc = "Name of the resource provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Name of the resource type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Name of the resource provider operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the resource provider operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ResourceProviderOperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource provider operation list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperationList {
    #[doc = "Resource provider operations list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceProviderOperationDefinition>,
    #[doc = "The URI that can be used to request the next page for list of Azure operations."]
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
