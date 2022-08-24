#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The results of Azure-AsyncOperation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureAsyncOperationResults {
    #[doc = "The fully qualified ID for the management group.  For example, /providers/Microsoft.Management/managementGroups/0000000-0000-0000-0000-000000000000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the resource.  For example, Microsoft.Management/managementGroups"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of the management group. For example, 00000000-0000-0000-0000-000000000000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The current status of the asynchronous operation performed . For example, Running, Succeeded, Failed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The generic properties of a management group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagementGroupInfoProperties>,
}
impl AzureAsyncOperationResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Management group name availability check parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityRequest {
    #[doc = "the name to check for availability"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "fully qualified resource type which includes provider namespace"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<check_name_availability_request::Type>,
}
impl CheckNameAvailabilityRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_request {
    use super::*;
    #[doc = "fully qualified resource type which includes provider namespace"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.Management/managementGroups")]
        MicrosoftManagementManagementGroups,
    }
}
#[doc = "Describes the result of the request to check management group name availability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[doc = "Required. True indicates name is valid and available. False indicates the name is invalid, unavailable, or both."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "Required if nameAvailable == false. Invalid indicates the name provided does not match the resource provider's naming requirements (incorrect length, unsupported characters, etc.) AlreadyExists indicates that the name is already in use and is therefore unavailable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_result::Reason>,
    #[doc = "Required if nameAvailable == false. Localized. If reason == invalid, provide the user with the reason why the given name is invalid, and provide the resource naming requirements so that the user can select a valid name. If reason == AlreadyExists, explain that is already in use, and direct them to select a different name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_result {
    use super::*;
    #[doc = "Required if nameAvailable == false. Invalid indicates the name provided does not match the resource provider's naming requirements (incorrect length, unsupported characters, etc.) AlreadyExists indicates that the name is already in use and is therefore unavailable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Invalid,
        AlreadyExists,
    }
}
#[doc = "The child information of a management group used during creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateManagementGroupChildInfo {
    #[doc = "The type of child resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ManagementGroupChildType>,
    #[doc = "The fully qualified ID for the child resource (management group or subscription).  For example, /providers/Microsoft.Management/managementGroups/0000000-0000-0000-0000-000000000000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the child entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The friendly name of the child resource."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The list of children."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<CreateManagementGroupChildInfo>,
}
impl CreateManagementGroupChildInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of a management group used during creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateManagementGroupDetails {
    #[doc = "The version number of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<f64>,
    #[doc = "The date and time when this object was last updated."]
    #[serde(rename = "updatedTime", default, with = "azure_core::date::rfc3339::option")]
    pub updated_time: Option<time::OffsetDateTime>,
    #[doc = "The identity of the principal or process that updated the object."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[doc = "(Optional) The ID of the parent management group used during creation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<CreateParentGroupInfo>,
}
impl CreateManagementGroupDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The generic properties of a management group used during creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateManagementGroupProperties {
    #[doc = "The AAD Tenant ID associated with the management group. For example, 00000000-0000-0000-0000-000000000000"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The friendly name of the management group. If no value is passed then this  field will be set to the groupId."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The details of a management group used during creation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<CreateManagementGroupDetails>,
    #[doc = "The list of children."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<CreateManagementGroupChildInfo>,
}
impl CreateManagementGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Management group creation parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateManagementGroupRequest {
    #[doc = "The fully qualified ID for the management group.  For example, /providers/Microsoft.Management/managementGroups/0000000-0000-0000-0000-000000000000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the resource.  For example, Microsoft.Management/managementGroups"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of the management group. For example, 00000000-0000-0000-0000-000000000000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The generic properties of a management group used during creation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CreateManagementGroupProperties>,
}
impl CreateManagementGroupRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the request to create or update Management Group settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateOrUpdateSettingsProperties {
    #[doc = "Indicates whether RBAC access is required upon group creation under the root Management Group. If set to true, user will require Microsoft.Management/managementGroups/write action on the root Management Group scope in order to create new Groups directly under the root. This will prevent new users from creating new Management Groups, unless they are given access."]
    #[serde(rename = "requireAuthorizationForGroupCreation", default, skip_serializing_if = "Option::is_none")]
    pub require_authorization_for_group_creation: Option<bool>,
    #[doc = "Settings that sets the default Management Group under which new subscriptions get added in this tenant. For example, /providers/Microsoft.Management/managementGroups/defaultGroup"]
    #[serde(rename = "defaultManagementGroup", default, skip_serializing_if = "Option::is_none")]
    pub default_management_group: Option<String>,
}
impl CreateOrUpdateSettingsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for creating or updating Management Group settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateOrUpdateSettingsRequest {
    #[doc = "The properties of the request to create or update Management Group settings"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CreateOrUpdateSettingsProperties>,
}
impl CreateOrUpdateSettingsRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "(Optional) The ID of the parent management group used during creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateParentGroupInfo {
    #[doc = "The fully qualified ID for the parent management group.  For example, /providers/Microsoft.Management/managementGroups/0000000-0000-0000-0000-000000000000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the parent management group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The friendly name of the parent management group."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl CreateParentGroupInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The descendant."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DescendantInfo {
    #[doc = "The fully qualified ID for the descendant.  For example, /providers/Microsoft.Management/managementGroups/0000000-0000-0000-0000-000000000000 or /subscriptions/0000000-0000-0000-0000-000000000000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the resource. For example, Microsoft.Management/managementGroups or /subscriptions"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of the descendant. For example, 00000000-0000-0000-0000-000000000000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The generic properties of an descendant."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DescendantInfoProperties>,
}
impl DescendantInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The generic properties of an descendant."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DescendantInfoProperties {
    #[doc = "The friendly name of the management group."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The ID of the parent management group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<DescendantParentGroupInfo>,
}
impl DescendantInfoProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the result of the request to view descendants."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DescendantListResult {
    #[doc = "The list of descendants."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DescendantInfo>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DescendantListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DescendantListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The ID of the parent management group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DescendantParentGroupInfo {
    #[doc = "The fully qualified ID for the parent management group.  For example, /providers/Microsoft.Management/managementGroups/0000000-0000-0000-0000-000000000000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl DescendantParentGroupInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The management group details for the hierarchy view."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityHierarchyItem {
    #[doc = "The fully qualified ID for the management group.  For example, /providers/Microsoft.Management/managementGroups/0000000-0000-0000-0000-000000000000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the resource.  For example, Microsoft.Management/managementGroups"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of the management group. For example, 00000000-0000-0000-0000-000000000000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The generic properties of a management group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EntityHierarchyItemProperties>,
}
impl EntityHierarchyItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The generic properties of a management group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityHierarchyItemProperties {
    #[doc = "The friendly name of the management group."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The users specific permissions to this item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[doc = "The list of children."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<EntityHierarchyItem>,
}
impl EntityHierarchyItemProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityInfo {
    #[doc = "The fully qualified ID for the entity.  For example, /providers/Microsoft.Management/managementGroups/0000000-0000-0000-0000-000000000000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the resource. For example, Microsoft.Management/managementGroups"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of the entity. For example, 00000000-0000-0000-0000-000000000000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The generic properties of an entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EntityInfoProperties>,
}
impl EntityInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The generic properties of an entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityInfoProperties {
    #[doc = "The AAD Tenant ID associated with the entity. For example, 00000000-0000-0000-0000-000000000000"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The friendly name of the management group."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "(Optional) The ID of the parent management group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<EntityParentGroupInfo>,
    #[doc = "The users specific permissions to this item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[doc = "The users specific permissions to this item."]
    #[serde(rename = "inheritedPermissions", default, skip_serializing_if = "Option::is_none")]
    pub inherited_permissions: Option<Permissions>,
    #[serde(rename = "numberOfDescendants", default, skip_serializing_if = "Option::is_none")]
    pub number_of_descendants: Option<i64>,
    #[doc = "Number of children is the number of Groups and Subscriptions that are exactly one level underneath the current Group."]
    #[serde(rename = "numberOfChildren", default, skip_serializing_if = "Option::is_none")]
    pub number_of_children: Option<i64>,
    #[doc = "Number of children is the number of Groups that are exactly one level underneath the current Group."]
    #[serde(rename = "numberOfChildGroups", default, skip_serializing_if = "Option::is_none")]
    pub number_of_child_groups: Option<i64>,
    #[doc = "The parent display name chain from the root group to the immediate parent"]
    #[serde(rename = "parentDisplayNameChain", default, skip_serializing_if = "Vec::is_empty")]
    pub parent_display_name_chain: Vec<String>,
    #[doc = "The parent name chain from the root group to the immediate parent"]
    #[serde(rename = "parentNameChain", default, skip_serializing_if = "Vec::is_empty")]
    pub parent_name_chain: Vec<String>,
}
impl EntityInfoProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the result of the request to view entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityListResult {
    #[doc = "The list of entities."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EntityInfo>,
    #[doc = "Total count of records that match the filter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EntityListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EntityListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "(Optional) The ID of the parent management group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityParentGroupInfo {
    #[doc = "The fully qualified ID for the parent management group.  For example, /providers/Microsoft.Management/managementGroups/0000000-0000-0000-0000-000000000000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl EntityParentGroupInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of the error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[doc = "One of a server-defined set of error codes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A human-readable representation of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "A human-readable representation of the error's details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The details of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
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
#[doc = "Settings defined at the Management Group scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HierarchySettings {
    #[doc = "The fully qualified ID for the settings object.  For example, /providers/Microsoft.Management/managementGroups/0000000-0000-0000-0000-000000000000/settings/default."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the resource.  For example, Microsoft.Management/managementGroups/settings."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of the object. In this case, default."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The generic properties of hierarchy settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HierarchySettingsProperties>,
}
impl HierarchySettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The hierarchy settings resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HierarchySettingsInfo {
    #[doc = "The fully qualified ID for the settings object.  For example, /providers/Microsoft.Management/managementGroups/0000000-0000-0000-0000-000000000000/settings/default."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the resource.  For example, Microsoft.Management/managementGroups/settings."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of the object. In this case, default."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The generic properties of hierarchy settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HierarchySettingsProperties>,
}
impl HierarchySettingsInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Lists all hierarchy settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HierarchySettingsList {
    #[doc = "The list of hierarchy settings."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HierarchySettingsInfo>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl HierarchySettingsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The generic properties of hierarchy settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HierarchySettingsProperties {
    #[doc = "The AAD Tenant ID associated with the hierarchy settings. For example, 00000000-0000-0000-0000-000000000000"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Indicates whether RBAC access is required upon group creation under the root Management Group. If set to true, user will require Microsoft.Management/managementGroups/write action on the root Management Group scope in order to create new Groups directly under the root. This will prevent new users from creating new Management Groups, unless they are given access."]
    #[serde(rename = "requireAuthorizationForGroupCreation", default, skip_serializing_if = "Option::is_none")]
    pub require_authorization_for_group_creation: Option<bool>,
    #[doc = "Settings that sets the default Management Group under which new subscriptions get added in this tenant. For example, /providers/Microsoft.Management/managementGroups/defaultGroup"]
    #[serde(rename = "defaultManagementGroup", default, skip_serializing_if = "Option::is_none")]
    pub default_management_group: Option<String>,
}
impl HierarchySettingsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of all subscriptions under management group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListSubscriptionUnderManagementGroup {
    #[doc = "The list of subscriptions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SubscriptionUnderManagementGroup>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListSubscriptionUnderManagementGroup {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ListSubscriptionUnderManagementGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The management group details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroup {
    #[doc = "The fully qualified ID for the management group.  For example, /providers/Microsoft.Management/managementGroups/0000000-0000-0000-0000-000000000000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the resource.  For example, Microsoft.Management/managementGroups"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of the management group. For example, 00000000-0000-0000-0000-000000000000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The generic properties of a management group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagementGroupProperties>,
}
impl ManagementGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The child information of a management group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroupChildInfo {
    #[doc = "The type of child resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ManagementGroupChildType>,
    #[doc = "The fully qualified ID for the child resource (management group or subscription).  For example, /providers/Microsoft.Management/managementGroups/0000000-0000-0000-0000-000000000000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the child entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The friendly name of the child resource."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The list of children."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<ManagementGroupChildInfo>,
}
impl ManagementGroupChildInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of child resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ManagementGroupChildType {
    #[serde(rename = "Microsoft.Management/managementGroups")]
    MicrosoftManagementManagementGroups,
    #[serde(rename = "/subscriptions")]
    Subscriptions,
}
#[doc = "The details of a management group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroupDetails {
    #[doc = "The version number of the object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<f64>,
    #[doc = "The date and time when this object was last updated."]
    #[serde(rename = "updatedTime", default, with = "azure_core::date::rfc3339::option")]
    pub updated_time: Option<time::OffsetDateTime>,
    #[doc = "The identity of the principal or process that updated the object."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[doc = "(Optional) The ID of the parent management group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<ParentGroupInfo>,
    #[doc = "The path from the root to the current group."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub path: Vec<ManagementGroupPathElement>,
}
impl ManagementGroupDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The management group resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroupInfo {
    #[doc = "The fully qualified ID for the management group.  For example, /providers/Microsoft.Management/managementGroups/0000000-0000-0000-0000-000000000000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the resource. For example, Microsoft.Management/managementGroups"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of the management group. For example, 00000000-0000-0000-0000-000000000000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The generic properties of a management group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagementGroupInfoProperties>,
}
impl ManagementGroupInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The generic properties of a management group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroupInfoProperties {
    #[doc = "The AAD Tenant ID associated with the management group. For example, 00000000-0000-0000-0000-000000000000"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The friendly name of the management group."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl ManagementGroupInfoProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the result of the request to list management groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroupListResult {
    #[doc = "The list of management groups."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagementGroupInfo>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ManagementGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ManagementGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A path element of a management group ancestors."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroupPathElement {
    #[doc = "The name of the group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The friendly name of the group."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl ManagementGroupPathElement {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The generic properties of a management group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroupProperties {
    #[doc = "The AAD Tenant ID associated with the management group. For example, 00000000-0000-0000-0000-000000000000"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The friendly name of the management group."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The details of a management group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<ManagementGroupDetails>,
    #[doc = "The list of children."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<ManagementGroupChildInfo>,
}
impl ManagementGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation supported by the Microsoft.Management resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayProperties>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The object that represents the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplayProperties {
    #[doc = "The name of the provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The resource on which the operation is performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The operation that can be performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Operation description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplayProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the result of the request to list Microsoft.Management operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the Microsoft.Management resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
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
#[doc = "The results of an asynchronous operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResults {
    #[doc = "The fully qualified ID for the management group.  For example, /providers/Microsoft.Management/managementGroups/0000000-0000-0000-0000-000000000000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the resource.  For example, Microsoft.Management/managementGroups"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of the management group. For example, 00000000-0000-0000-0000-000000000000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The generic properties of a management group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagementGroupInfoProperties>,
}
impl OperationResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "(Optional) The ID of the parent management group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParentGroupInfo {
    #[doc = "The fully qualified ID for the parent management group.  For example, /providers/Microsoft.Management/managementGroups/0000000-0000-0000-0000-000000000000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the parent management group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The friendly name of the parent management group."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl ParentGroupInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Management group patch parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchManagementGroupRequest {
    #[doc = "The friendly name of the management group."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "(Optional) The fully qualified ID for the parent management group.  For example, /providers/Microsoft.Management/managementGroups/0000000-0000-0000-0000-000000000000"]
    #[serde(rename = "parentGroupId", default, skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
}
impl PatchManagementGroupRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The users specific permissions to this item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Permissions {
    #[serde(rename = "noaccess")]
    Noaccess,
    #[serde(rename = "view")]
    View,
    #[serde(rename = "edit")]
    Edit,
    #[serde(rename = "delete")]
    Delete,
}
#[doc = "The details of subscription under management group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionUnderManagementGroup {
    #[doc = "The fully qualified ID for the subscription.  For example, /providers/Microsoft.Management/managementGroups/0000000-0000-0000-0000-000000000000/subscriptions/0000000-0000-0000-0000-000000000001"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the resource.  For example, Microsoft.Management/managementGroups/subscriptions"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The stringified id of the subscription. For example, 00000000-0000-0000-0000-000000000000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The generic properties of subscription under a management group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionUnderManagementGroupProperties>,
}
impl SubscriptionUnderManagementGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The generic properties of subscription under a management group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionUnderManagementGroupProperties {
    #[doc = "The AAD Tenant ID associated with the subscription. For example, 00000000-0000-0000-0000-000000000000"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
    #[doc = "The friendly name of the subscription."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The ID of the parent management group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<DescendantParentGroupInfo>,
    #[doc = "The state of the subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl SubscriptionUnderManagementGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The tenant backfill status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TenantBackfillStatusResult {
    #[doc = "The AAD Tenant ID associated with the management group. For example, 00000000-0000-0000-0000-000000000000"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The status of the Tenant Backfill"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<tenant_backfill_status_result::Status>,
}
impl TenantBackfillStatusResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod tenant_backfill_status_result {
    use super::*;
    #[doc = "The status of the Tenant Backfill"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        NotStarted,
        NotStartedButGroupsExist,
        Started,
        Failed,
        Cancelled,
        Completed,
    }
}
