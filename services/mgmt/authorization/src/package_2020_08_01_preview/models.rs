#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Classic Administrators"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClassicAdministrator {
    #[doc = "The ID of the administrator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the administrator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the administrator."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Classic Administrator properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClassicAdministratorProperties>,
}
impl ClassicAdministrator {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ClassicAdministrator list result information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClassicAdministratorListResult {
    #[doc = "An array of administrators."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ClassicAdministrator>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ClassicAdministratorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ClassicAdministratorListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Classic Administrator properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClassicAdministratorProperties {
    #[doc = "The email address of the administrator."]
    #[serde(rename = "emailAddress", default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[doc = "The role of the administrator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}
impl ClassicAdministratorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deny Assignment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DenyAssignment {
    #[doc = "The deny assignment ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The deny assignment name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The deny assignment type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Deny assignment properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DenyAssignmentProperties>,
}
impl DenyAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deny Assignments filter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DenyAssignmentFilter {
    #[doc = "Return deny assignment with specified name."]
    #[serde(rename = "denyAssignmentName", default, skip_serializing_if = "Option::is_none")]
    pub deny_assignment_name: Option<String>,
    #[doc = "Return all deny assignments where the specified principal is listed in the principals list of deny assignments."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Return all deny assignments where the specified principal is listed either in the principals list or exclude principals list of deny assignments."]
    #[serde(rename = "gdprExportPrincipalId", default, skip_serializing_if = "Option::is_none")]
    pub gdpr_export_principal_id: Option<String>,
}
impl DenyAssignmentFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deny assignment list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DenyAssignmentListResult {
    #[doc = "Deny assignment list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DenyAssignment>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DenyAssignmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DenyAssignmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deny assignment permissions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DenyAssignmentPermission {
    #[doc = "Actions to which the deny assignment does not grant access."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<String>,
    #[doc = "Actions to exclude from that the deny assignment does not grant access."]
    #[serde(rename = "notActions", default, skip_serializing_if = "Vec::is_empty")]
    pub not_actions: Vec<String>,
    #[doc = "Data actions to which the deny assignment does not grant access."]
    #[serde(rename = "dataActions", default, skip_serializing_if = "Vec::is_empty")]
    pub data_actions: Vec<String>,
    #[doc = "Data actions to exclude from that the deny assignment does not grant access."]
    #[serde(rename = "notDataActions", default, skip_serializing_if = "Vec::is_empty")]
    pub not_data_actions: Vec<String>,
}
impl DenyAssignmentPermission {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deny assignment properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DenyAssignmentProperties {
    #[doc = "The display name of the deny assignment."]
    #[serde(rename = "denyAssignmentName", default, skip_serializing_if = "Option::is_none")]
    pub deny_assignment_name: Option<String>,
    #[doc = "The description of the deny assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "An array of permissions that are denied by the deny assignment."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<DenyAssignmentPermission>,
    #[doc = "The deny assignment scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "Determines if the deny assignment applies to child scopes. Default value is false."]
    #[serde(rename = "doNotApplyToChildScopes", default, skip_serializing_if = "Option::is_none")]
    pub do_not_apply_to_child_scopes: Option<bool>,
    #[doc = "Array of principals to which the deny assignment applies."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub principals: Vec<Principal>,
    #[doc = "Array of principals to which the deny assignment does not apply."]
    #[serde(rename = "excludePrincipals", default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_principals: Vec<Principal>,
    #[doc = "Specifies whether this deny assignment was created by Azure and cannot be edited or deleted."]
    #[serde(rename = "isSystemProtected", default, skip_serializing_if = "Option::is_none")]
    pub is_system_protected: Option<bool>,
}
impl DenyAssignmentProperties {
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
#[doc = "Role definition permissions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Permission {
    #[doc = "Allowed actions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<String>,
    #[doc = "Denied actions."]
    #[serde(rename = "notActions", default, skip_serializing_if = "Vec::is_empty")]
    pub not_actions: Vec<String>,
    #[doc = "Allowed Data actions."]
    #[serde(rename = "dataActions", default, skip_serializing_if = "Vec::is_empty")]
    pub data_actions: Vec<String>,
    #[doc = "Denied Data actions."]
    #[serde(rename = "notDataActions", default, skip_serializing_if = "Vec::is_empty")]
    pub not_data_actions: Vec<String>,
}
impl Permission {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Permissions information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PermissionGetResult {
    #[doc = "An array of permissions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Permission>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PermissionGetResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PermissionGetResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The name of the entity last modified it"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Principal {
    #[doc = "The id of the principal made changes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the principal made changes"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Type of principal such as user , group etc"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Email of principal"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
impl Principal {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderOperation {
    #[doc = "The operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The operation display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The operation description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The operation origin."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "The operation properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The dataAction flag to specify the operation type."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
impl ProviderOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provider Operations metadata"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderOperationsMetadata {
    #[doc = "The provider id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The provider name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The provider type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The provider display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The provider resource types"]
    #[serde(rename = "resourceTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_types: Vec<ResourceType>,
    #[doc = "The provider operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<ProviderOperation>,
}
impl ProviderOperationsMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provider operations metadata list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderOperationsMetadataListResult {
    #[doc = "The list of providers."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProviderOperationsMetadata>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProviderOperationsMetadataListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProviderOperationsMetadataListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource Type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceType {
    #[doc = "The resource type name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The resource type operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<ProviderOperation>,
}
impl ResourceType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role Assignments"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignment {
    #[doc = "The role assignment ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The role assignment name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The role assignment type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Role assignment properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleAssignmentProperties>,
}
impl RoleAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role assignment create parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentCreateParameters {
    #[doc = "Role assignment properties."]
    pub properties: RoleAssignmentProperties,
}
impl RoleAssignmentCreateParameters {
    pub fn new(properties: RoleAssignmentProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Role Assignments filter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentFilter {
    #[doc = "Returns role assignment of the specific principal."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
}
impl RoleAssignmentFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role assignment list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentListResult {
    #[doc = "Role assignment list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RoleAssignment>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleAssignmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RoleAssignmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role Assignment Metrics"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentMetricsResult {
    #[doc = "The subscription ID."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The role assignment limit."]
    #[serde(rename = "roleAssignmentsLimit", default, skip_serializing_if = "Option::is_none")]
    pub role_assignments_limit: Option<i64>,
    #[doc = "The number of current role assignments."]
    #[serde(rename = "roleAssignmentsCurrentCount", default, skip_serializing_if = "Option::is_none")]
    pub role_assignments_current_count: Option<i64>,
    #[doc = "The number of remaining role assignments available."]
    #[serde(rename = "roleAssignmentsRemainingCount", default, skip_serializing_if = "Option::is_none")]
    pub role_assignments_remaining_count: Option<i64>,
}
impl RoleAssignmentMetricsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role assignment properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentProperties {
    #[doc = "The role assignment scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The role definition ID."]
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
    #[doc = "The principal ID."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[doc = "The principal type of the assigned principal ID."]
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<role_assignment_properties::PrincipalType>,
    #[doc = "Description of role assignment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The conditions on the role assignment. This limits the resources it can be assigned to. e.g.: @Resource[Microsoft.Storage/storageAccounts/blobServices/containers:ContainerName] StringEqualsIgnoreCase 'foo_storage_container'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Version of the condition. Currently accepted value is '2.0'"]
    #[serde(rename = "conditionVersion", default, skip_serializing_if = "Option::is_none")]
    pub condition_version: Option<String>,
    #[doc = "Time it was created"]
    #[serde(rename = "createdOn", default, with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Time it was updated"]
    #[serde(rename = "updatedOn", default, with = "azure_core::date::rfc3339::option")]
    pub updated_on: Option<time::OffsetDateTime>,
    #[doc = "Id of the user who created the assignment"]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "Id of the user who updated the assignment"]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    #[doc = "Id of the delegated managed identity resource"]
    #[serde(rename = "delegatedManagedIdentityResourceId", default, skip_serializing_if = "Option::is_none")]
    pub delegated_managed_identity_resource_id: Option<String>,
}
impl RoleAssignmentProperties {
    pub fn new(role_definition_id: String, principal_id: String) -> Self {
        Self {
            scope: None,
            role_definition_id,
            principal_id,
            principal_type: None,
            description: None,
            condition: None,
            condition_version: None,
            created_on: None,
            updated_on: None,
            created_by: None,
            updated_by: None,
            delegated_managed_identity_resource_id: None,
        }
    }
}
pub mod role_assignment_properties {
    use super::*;
    #[doc = "The principal type of the assigned principal ID."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrincipalType")]
    pub enum PrincipalType {
        User,
        Group,
        ServicePrincipal,
        ForeignGroup,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrincipalType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrincipalType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrincipalType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("PrincipalType", 0u32, "User"),
                Self::Group => serializer.serialize_unit_variant("PrincipalType", 1u32, "Group"),
                Self::ServicePrincipal => serializer.serialize_unit_variant("PrincipalType", 2u32, "ServicePrincipal"),
                Self::ForeignGroup => serializer.serialize_unit_variant("PrincipalType", 3u32, "ForeignGroup"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for PrincipalType {
        fn default() -> Self {
            Self::User
        }
    }
}
#[doc = "Role definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleDefinition {
    #[doc = "The role definition ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The role definition name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The role definition type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Role definition properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleDefinitionProperties>,
}
impl RoleDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role Definitions filter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleDefinitionFilter {
    #[doc = "Returns role definition with the specific name."]
    #[serde(rename = "roleName", default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[doc = "Returns role definition with the specific type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl RoleDefinitionFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role definition list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleDefinitionListResult {
    #[doc = "Role definition list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RoleDefinition>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleDefinitionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RoleDefinitionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role definition properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleDefinitionProperties {
    #[doc = "The role name."]
    #[serde(rename = "roleName", default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[doc = "The role definition description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The role type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Role definition permissions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<Permission>,
    #[doc = "Role definition assignable scopes."]
    #[serde(rename = "assignableScopes", default, skip_serializing_if = "Vec::is_empty")]
    pub assignable_scopes: Vec<String>,
}
impl RoleDefinitionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Validation response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidationResponse {
    #[doc = "Whether or not validation succeeded"]
    #[serde(rename = "isValid", default, skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
    #[doc = "Failed validation result details"]
    #[serde(rename = "errorInfo", default, skip_serializing_if = "Option::is_none")]
    pub error_info: Option<ValidationResponseErrorInfo>,
}
impl ValidationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Failed validation result details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidationResponseErrorInfo {
    #[doc = "Error code indicating why validation failed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Message indicating why validation failed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ValidationResponseErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
