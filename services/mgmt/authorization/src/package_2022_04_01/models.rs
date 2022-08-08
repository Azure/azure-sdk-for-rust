#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The approval settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApprovalSettings {
    #[doc = "Determines whether approval is required or not."]
    #[serde(rename = "isApprovalRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_approval_required: Option<bool>,
    #[doc = "Determines whether approval is required for assignment extension."]
    #[serde(rename = "isApprovalRequiredForExtension", default, skip_serializing_if = "Option::is_none")]
    pub is_approval_required_for_extension: Option<bool>,
    #[doc = "Determine whether requestor justification is required."]
    #[serde(rename = "isRequestorJustificationRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_requestor_justification_required: Option<bool>,
    #[doc = "The type of rule"]
    #[serde(rename = "approvalMode", default, skip_serializing_if = "Option::is_none")]
    pub approval_mode: Option<approval_settings::ApprovalMode>,
    #[doc = "The approval stages of the request."]
    #[serde(rename = "approvalStages", default, skip_serializing_if = "Vec::is_empty")]
    pub approval_stages: Vec<ApprovalStage>,
}
impl ApprovalSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod approval_settings {
    use super::*;
    #[doc = "The type of rule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ApprovalMode")]
    pub enum ApprovalMode {
        SingleStage,
        Serial,
        Parallel,
        NoApproval,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ApprovalMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ApprovalMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ApprovalMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SingleStage => serializer.serialize_unit_variant("ApprovalMode", 0u32, "SingleStage"),
                Self::Serial => serializer.serialize_unit_variant("ApprovalMode", 1u32, "Serial"),
                Self::Parallel => serializer.serialize_unit_variant("ApprovalMode", 2u32, "Parallel"),
                Self::NoApproval => serializer.serialize_unit_variant("ApprovalMode", 3u32, "NoApproval"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The approval stage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApprovalStage {
    #[doc = "The time in days when approval request would be timed out"]
    #[serde(rename = "approvalStageTimeOutInDays", default, skip_serializing_if = "Option::is_none")]
    pub approval_stage_time_out_in_days: Option<i32>,
    #[doc = "Determines whether approver need to provide justification for his decision."]
    #[serde(rename = "isApproverJustificationRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_approver_justification_required: Option<bool>,
    #[doc = "The time in minutes when the approval request would be escalated if the primary approver does not approve"]
    #[serde(rename = "escalationTimeInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub escalation_time_in_minutes: Option<i32>,
    #[doc = "The primary approver of the request."]
    #[serde(rename = "primaryApprovers", default, skip_serializing_if = "Vec::is_empty")]
    pub primary_approvers: Vec<UserSet>,
    #[doc = "The value determine whether escalation feature is enabled."]
    #[serde(rename = "isEscalationEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_escalation_enabled: Option<bool>,
    #[doc = "The escalation approver of the request."]
    #[serde(rename = "escalationApprovers", default, skip_serializing_if = "Vec::is_empty")]
    pub escalation_approvers: Vec<UserSet>,
}
impl ApprovalStage {
    pub fn new() -> Self {
        Self::default()
    }
}
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
#[doc = "An error response from the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response from the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl azure_core::Continuable for CloudError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CloudErrorBody {
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
    #[doc = "The conditions on the Deny assignment permission. This limits the resources it applies to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Version of the condition."]
    #[serde(rename = "conditionVersion", default, skip_serializing_if = "Option::is_none")]
    pub condition_version: Option<String>,
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
#[doc = "Eligible child resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EligibleChildResource {
    #[doc = "The resource scope Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl EligibleChildResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Eligible child resources list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EligibleChildResourcesListResult {
    #[doc = "Eligible child resource list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EligibleChildResource>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EligibleChildResourcesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EligibleChildResourcesListResult {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExpandedProperties {
    #[doc = "Details of the resource scope"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<expanded_properties::Scope>,
    #[doc = "Details of role definition"]
    #[serde(rename = "roleDefinition", default, skip_serializing_if = "Option::is_none")]
    pub role_definition: Option<expanded_properties::RoleDefinition>,
    #[doc = "Details of the principal"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principal: Option<expanded_properties::Principal>,
}
impl ExpandedProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod expanded_properties {
    use super::*;
    #[doc = "Details of the resource scope"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Scope {
        #[doc = "Scope id of the resource"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[doc = "Display name of the resource"]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "Type of the resource"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }
    impl Scope {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Details of role definition"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct RoleDefinition {
        #[doc = "Id of the role definition"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[doc = "Display name of the role definition"]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "Type of the role definition"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }
    impl RoleDefinition {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Details of the principal"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Principal {
        #[doc = "Id of the principal"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[doc = "Display name of the principal"]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "Email id of the principal"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[doc = "Type of the principal"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }
    impl Principal {
        pub fn new() -> Self {
            Self::default()
        }
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
#[doc = "Expanded info of resource scope, role definition and policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyAssignmentProperties {
    #[doc = "Details of the resource scope"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<policy_assignment_properties::Scope>,
    #[doc = "Details of role definition"]
    #[serde(rename = "roleDefinition", default, skip_serializing_if = "Option::is_none")]
    pub role_definition: Option<policy_assignment_properties::RoleDefinition>,
    #[doc = "Details of the policy"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<policy_assignment_properties::Policy>,
}
impl PolicyAssignmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod policy_assignment_properties {
    use super::*;
    #[doc = "Details of the resource scope"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Scope {
        #[doc = "Scope id of the resource"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[doc = "Display name of the resource"]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "Type of the resource"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }
    impl Scope {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Details of role definition"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct RoleDefinition {
        #[doc = "Id of the role definition"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[doc = "Display name of the role definition"]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "Type of the role definition"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }
    impl RoleDefinition {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Details of the policy"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Policy {
        #[doc = "Id of the policy"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[doc = "The name of the entity last modified it"]
        #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
        pub last_modified_by: Option<Principal>,
        #[doc = "The last modified date time."]
        #[serde(rename = "lastModifiedDateTime", with = "azure_core::date::rfc3339::option")]
        pub last_modified_date_time: Option<time::OffsetDateTime>,
    }
    impl Policy {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Expanded info of resource scope"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyProperties {
    #[doc = "Details of the resource scope"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<policy_properties::Scope>,
}
impl PolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod policy_properties {
    use super::*;
    #[doc = "Details of the resource scope"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Scope {
        #[doc = "Scope id of the resource"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[doc = "Display name of the resource"]
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[doc = "Type of the resource"]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }
    impl Scope {
        pub fn new() -> Self {
            Self::default()
        }
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
    #[doc = "The skipToken to use for getting the next set of results."]
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
    #[doc = "Version of the condition. Currently the only accepted value is '2.0'"]
    #[serde(rename = "conditionVersion", default, skip_serializing_if = "Option::is_none")]
    pub condition_version: Option<String>,
    #[doc = "Time it was created"]
    #[serde(rename = "createdOn", with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Time it was updated"]
    #[serde(rename = "updatedOn", with = "azure_core::date::rfc3339::option")]
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
        Device,
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
                Self::Device => serializer.serialize_unit_variant("PrincipalType", 4u32, "Device"),
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
#[doc = "Role Assignment schedule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentSchedule {
    #[doc = "The role assignment schedule Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The role assignment schedule name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The role assignment schedule type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Role assignment schedule properties with scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleAssignmentScheduleProperties>,
}
impl RoleAssignmentSchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role assignment schedule filter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentScheduleFilter {
    #[doc = "Returns role assignment schedule of the specific principal."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Returns role assignment schedule of the specific role definition."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "Returns role assignment schedule instances of the specific status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl RoleAssignmentScheduleFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about current or upcoming role assignment schedule instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentScheduleInstance {
    #[doc = "The role assignment schedule instance ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The role assignment schedule instance name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The role assignment schedule instance type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Role assignment schedule properties with scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleAssignmentScheduleInstanceProperties>,
}
impl RoleAssignmentScheduleInstance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role assignment schedule instance filter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentScheduleInstanceFilter {
    #[doc = "Returns role assignment schedule instances of the specific principal."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Returns role assignment schedule instances of the specific role definition."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "Returns role assignment schedule instances of the specific status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Returns role assignment schedule instances belonging to a specific role assignment schedule."]
    #[serde(rename = "roleAssignmentScheduleId", default, skip_serializing_if = "Option::is_none")]
    pub role_assignment_schedule_id: Option<String>,
}
impl RoleAssignmentScheduleInstanceFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role assignment schedule instance list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentScheduleInstanceListResult {
    #[doc = "Role assignment schedule instance list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RoleAssignmentScheduleInstance>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleAssignmentScheduleInstanceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RoleAssignmentScheduleInstanceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role assignment schedule properties with scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentScheduleInstanceProperties {
    #[doc = "The role assignment schedule scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The role definition ID."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "The principal ID."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The principal type of the assigned principal ID."]
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<role_assignment_schedule_instance_properties::PrincipalType>,
    #[doc = "Id of the master role assignment schedule"]
    #[serde(rename = "roleAssignmentScheduleId", default, skip_serializing_if = "Option::is_none")]
    pub role_assignment_schedule_id: Option<String>,
    #[doc = "Role Assignment Id in external system"]
    #[serde(rename = "originRoleAssignmentId", default, skip_serializing_if = "Option::is_none")]
    pub origin_role_assignment_id: Option<String>,
    #[doc = "The status of the role assignment schedule instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<role_assignment_schedule_instance_properties::Status>,
    #[doc = "The startDateTime of the role assignment schedule instance"]
    #[serde(rename = "startDateTime", with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "The endDateTime of the role assignment schedule instance"]
    #[serde(rename = "endDateTime", with = "azure_core::date::rfc3339::option")]
    pub end_date_time: Option<time::OffsetDateTime>,
    #[doc = "roleEligibilityScheduleId used to activate"]
    #[serde(rename = "linkedRoleEligibilityScheduleId", default, skip_serializing_if = "Option::is_none")]
    pub linked_role_eligibility_schedule_id: Option<String>,
    #[doc = "roleEligibilityScheduleInstanceId linked to this roleAssignmentScheduleInstance"]
    #[serde(
        rename = "linkedRoleEligibilityScheduleInstanceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub linked_role_eligibility_schedule_instance_id: Option<String>,
    #[doc = "Assignment type of the role assignment schedule"]
    #[serde(rename = "assignmentType", default, skip_serializing_if = "Option::is_none")]
    pub assignment_type: Option<role_assignment_schedule_instance_properties::AssignmentType>,
    #[doc = "Membership type of the role assignment schedule"]
    #[serde(rename = "memberType", default, skip_serializing_if = "Option::is_none")]
    pub member_type: Option<role_assignment_schedule_instance_properties::MemberType>,
    #[doc = "The conditions on the role assignment. This limits the resources it can be assigned to. e.g.: @Resource[Microsoft.Storage/storageAccounts/blobServices/containers:ContainerName] StringEqualsIgnoreCase 'foo_storage_container'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Version of the condition. Currently accepted value is '2.0'"]
    #[serde(rename = "conditionVersion", default, skip_serializing_if = "Option::is_none")]
    pub condition_version: Option<String>,
    #[doc = "DateTime when role assignment schedule was created"]
    #[serde(rename = "createdOn", with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[serde(rename = "expandedProperties", default, skip_serializing_if = "Option::is_none")]
    pub expanded_properties: Option<ExpandedProperties>,
}
impl RoleAssignmentScheduleInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod role_assignment_schedule_instance_properties {
    use super::*;
    #[doc = "The principal type of the assigned principal ID."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrincipalType")]
    pub enum PrincipalType {
        User,
        Group,
        ServicePrincipal,
        ForeignGroup,
        Device,
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
                Self::Device => serializer.serialize_unit_variant("PrincipalType", 4u32, "Device"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the role assignment schedule instance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Accepted,
        PendingEvaluation,
        Granted,
        Denied,
        PendingProvisioning,
        Provisioned,
        PendingRevocation,
        Revoked,
        Canceled,
        Failed,
        PendingApprovalProvisioning,
        PendingApproval,
        FailedAsResourceIsLocked,
        PendingAdminDecision,
        AdminApproved,
        AdminDenied,
        TimedOut,
        ProvisioningStarted,
        Invalid,
        PendingScheduleCreation,
        ScheduleCreated,
        PendingExternalProvisioning,
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
                Self::Accepted => serializer.serialize_unit_variant("Status", 0u32, "Accepted"),
                Self::PendingEvaluation => serializer.serialize_unit_variant("Status", 1u32, "PendingEvaluation"),
                Self::Granted => serializer.serialize_unit_variant("Status", 2u32, "Granted"),
                Self::Denied => serializer.serialize_unit_variant("Status", 3u32, "Denied"),
                Self::PendingProvisioning => serializer.serialize_unit_variant("Status", 4u32, "PendingProvisioning"),
                Self::Provisioned => serializer.serialize_unit_variant("Status", 5u32, "Provisioned"),
                Self::PendingRevocation => serializer.serialize_unit_variant("Status", 6u32, "PendingRevocation"),
                Self::Revoked => serializer.serialize_unit_variant("Status", 7u32, "Revoked"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 8u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("Status", 9u32, "Failed"),
                Self::PendingApprovalProvisioning => serializer.serialize_unit_variant("Status", 10u32, "PendingApprovalProvisioning"),
                Self::PendingApproval => serializer.serialize_unit_variant("Status", 11u32, "PendingApproval"),
                Self::FailedAsResourceIsLocked => serializer.serialize_unit_variant("Status", 12u32, "FailedAsResourceIsLocked"),
                Self::PendingAdminDecision => serializer.serialize_unit_variant("Status", 13u32, "PendingAdminDecision"),
                Self::AdminApproved => serializer.serialize_unit_variant("Status", 14u32, "AdminApproved"),
                Self::AdminDenied => serializer.serialize_unit_variant("Status", 15u32, "AdminDenied"),
                Self::TimedOut => serializer.serialize_unit_variant("Status", 16u32, "TimedOut"),
                Self::ProvisioningStarted => serializer.serialize_unit_variant("Status", 17u32, "ProvisioningStarted"),
                Self::Invalid => serializer.serialize_unit_variant("Status", 18u32, "Invalid"),
                Self::PendingScheduleCreation => serializer.serialize_unit_variant("Status", 19u32, "PendingScheduleCreation"),
                Self::ScheduleCreated => serializer.serialize_unit_variant("Status", 20u32, "ScheduleCreated"),
                Self::PendingExternalProvisioning => serializer.serialize_unit_variant("Status", 21u32, "PendingExternalProvisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Assignment type of the role assignment schedule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AssignmentType")]
    pub enum AssignmentType {
        Activated,
        Assigned,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AssignmentType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AssignmentType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AssignmentType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Activated => serializer.serialize_unit_variant("AssignmentType", 0u32, "Activated"),
                Self::Assigned => serializer.serialize_unit_variant("AssignmentType", 1u32, "Assigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Membership type of the role assignment schedule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MemberType")]
    pub enum MemberType {
        Inherited,
        Direct,
        Group,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MemberType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MemberType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MemberType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Inherited => serializer.serialize_unit_variant("MemberType", 0u32, "Inherited"),
                Self::Direct => serializer.serialize_unit_variant("MemberType", 1u32, "Direct"),
                Self::Group => serializer.serialize_unit_variant("MemberType", 2u32, "Group"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Role assignment schedule list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentScheduleListResult {
    #[doc = "Role assignment schedule list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RoleAssignmentSchedule>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleAssignmentScheduleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RoleAssignmentScheduleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role assignment schedule properties with scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentScheduleProperties {
    #[doc = "The role assignment schedule scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The role definition ID."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "The principal ID."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The principal type of the assigned principal ID."]
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<role_assignment_schedule_properties::PrincipalType>,
    #[doc = "The id of roleAssignmentScheduleRequest used to create this roleAssignmentSchedule"]
    #[serde(rename = "roleAssignmentScheduleRequestId", default, skip_serializing_if = "Option::is_none")]
    pub role_assignment_schedule_request_id: Option<String>,
    #[doc = "The id of roleEligibilitySchedule used to activated this roleAssignmentSchedule"]
    #[serde(rename = "linkedRoleEligibilityScheduleId", default, skip_serializing_if = "Option::is_none")]
    pub linked_role_eligibility_schedule_id: Option<String>,
    #[doc = "Assignment type of the role assignment schedule"]
    #[serde(rename = "assignmentType", default, skip_serializing_if = "Option::is_none")]
    pub assignment_type: Option<role_assignment_schedule_properties::AssignmentType>,
    #[doc = "Membership type of the role assignment schedule"]
    #[serde(rename = "memberType", default, skip_serializing_if = "Option::is_none")]
    pub member_type: Option<role_assignment_schedule_properties::MemberType>,
    #[doc = "The status of the role assignment schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<role_assignment_schedule_properties::Status>,
    #[doc = "Start DateTime when role assignment schedule"]
    #[serde(rename = "startDateTime", with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "End DateTime when role assignment schedule"]
    #[serde(rename = "endDateTime", with = "azure_core::date::rfc3339::option")]
    pub end_date_time: Option<time::OffsetDateTime>,
    #[doc = "The conditions on the role assignment. This limits the resources it can be assigned to. e.g.: @Resource[Microsoft.Storage/storageAccounts/blobServices/containers:ContainerName] StringEqualsIgnoreCase 'foo_storage_container'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Version of the condition. Currently accepted value is '2.0'"]
    #[serde(rename = "conditionVersion", default, skip_serializing_if = "Option::is_none")]
    pub condition_version: Option<String>,
    #[doc = "DateTime when role assignment schedule was created"]
    #[serde(rename = "createdOn", with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "DateTime when role assignment schedule was modified"]
    #[serde(rename = "updatedOn", with = "azure_core::date::rfc3339::option")]
    pub updated_on: Option<time::OffsetDateTime>,
    #[serde(rename = "expandedProperties", default, skip_serializing_if = "Option::is_none")]
    pub expanded_properties: Option<ExpandedProperties>,
}
impl RoleAssignmentScheduleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod role_assignment_schedule_properties {
    use super::*;
    #[doc = "The principal type of the assigned principal ID."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrincipalType")]
    pub enum PrincipalType {
        User,
        Group,
        ServicePrincipal,
        ForeignGroup,
        Device,
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
                Self::Device => serializer.serialize_unit_variant("PrincipalType", 4u32, "Device"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Assignment type of the role assignment schedule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AssignmentType")]
    pub enum AssignmentType {
        Activated,
        Assigned,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AssignmentType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AssignmentType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AssignmentType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Activated => serializer.serialize_unit_variant("AssignmentType", 0u32, "Activated"),
                Self::Assigned => serializer.serialize_unit_variant("AssignmentType", 1u32, "Assigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Membership type of the role assignment schedule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MemberType")]
    pub enum MemberType {
        Inherited,
        Direct,
        Group,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MemberType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MemberType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MemberType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Inherited => serializer.serialize_unit_variant("MemberType", 0u32, "Inherited"),
                Self::Direct => serializer.serialize_unit_variant("MemberType", 1u32, "Direct"),
                Self::Group => serializer.serialize_unit_variant("MemberType", 2u32, "Group"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the role assignment schedule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Accepted,
        PendingEvaluation,
        Granted,
        Denied,
        PendingProvisioning,
        Provisioned,
        PendingRevocation,
        Revoked,
        Canceled,
        Failed,
        PendingApprovalProvisioning,
        PendingApproval,
        FailedAsResourceIsLocked,
        PendingAdminDecision,
        AdminApproved,
        AdminDenied,
        TimedOut,
        ProvisioningStarted,
        Invalid,
        PendingScheduleCreation,
        ScheduleCreated,
        PendingExternalProvisioning,
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
                Self::Accepted => serializer.serialize_unit_variant("Status", 0u32, "Accepted"),
                Self::PendingEvaluation => serializer.serialize_unit_variant("Status", 1u32, "PendingEvaluation"),
                Self::Granted => serializer.serialize_unit_variant("Status", 2u32, "Granted"),
                Self::Denied => serializer.serialize_unit_variant("Status", 3u32, "Denied"),
                Self::PendingProvisioning => serializer.serialize_unit_variant("Status", 4u32, "PendingProvisioning"),
                Self::Provisioned => serializer.serialize_unit_variant("Status", 5u32, "Provisioned"),
                Self::PendingRevocation => serializer.serialize_unit_variant("Status", 6u32, "PendingRevocation"),
                Self::Revoked => serializer.serialize_unit_variant("Status", 7u32, "Revoked"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 8u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("Status", 9u32, "Failed"),
                Self::PendingApprovalProvisioning => serializer.serialize_unit_variant("Status", 10u32, "PendingApprovalProvisioning"),
                Self::PendingApproval => serializer.serialize_unit_variant("Status", 11u32, "PendingApproval"),
                Self::FailedAsResourceIsLocked => serializer.serialize_unit_variant("Status", 12u32, "FailedAsResourceIsLocked"),
                Self::PendingAdminDecision => serializer.serialize_unit_variant("Status", 13u32, "PendingAdminDecision"),
                Self::AdminApproved => serializer.serialize_unit_variant("Status", 14u32, "AdminApproved"),
                Self::AdminDenied => serializer.serialize_unit_variant("Status", 15u32, "AdminDenied"),
                Self::TimedOut => serializer.serialize_unit_variant("Status", 16u32, "TimedOut"),
                Self::ProvisioningStarted => serializer.serialize_unit_variant("Status", 17u32, "ProvisioningStarted"),
                Self::Invalid => serializer.serialize_unit_variant("Status", 18u32, "Invalid"),
                Self::PendingScheduleCreation => serializer.serialize_unit_variant("Status", 19u32, "PendingScheduleCreation"),
                Self::ScheduleCreated => serializer.serialize_unit_variant("Status", 20u32, "ScheduleCreated"),
                Self::PendingExternalProvisioning => serializer.serialize_unit_variant("Status", 21u32, "PendingExternalProvisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Role Assignment schedule request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentScheduleRequest {
    #[doc = "The role assignment schedule request ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The role assignment schedule request name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The role assignment schedule request type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Role assignment schedule request properties with scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleAssignmentScheduleRequestProperties>,
}
impl RoleAssignmentScheduleRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role assignment schedule request filter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentScheduleRequestFilter {
    #[doc = "Returns role assignment requests of the specific principal."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Returns role assignment requests of the specific role definition."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "Returns role assignment requests created by specific principal."]
    #[serde(rename = "requestorId", default, skip_serializing_if = "Option::is_none")]
    pub requestor_id: Option<String>,
    #[doc = "Returns role assignment requests of specific status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl RoleAssignmentScheduleRequestFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role assignment schedule request list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentScheduleRequestListResult {
    #[doc = "Role assignment schedule request list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RoleAssignmentScheduleRequest>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleAssignmentScheduleRequestListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RoleAssignmentScheduleRequestListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role assignment schedule request properties with scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentScheduleRequestProperties {
    #[doc = "The role assignment schedule request scope."]
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
    pub principal_type: Option<role_assignment_schedule_request_properties::PrincipalType>,
    #[doc = "The type of the role assignment schedule request. Eg: SelfActivate, AdminAssign etc"]
    #[serde(rename = "requestType")]
    pub request_type: role_assignment_schedule_request_properties::RequestType,
    #[doc = "The status of the role assignment schedule request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<role_assignment_schedule_request_properties::Status>,
    #[doc = "The approvalId of the role assignment schedule request."]
    #[serde(rename = "approvalId", default, skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[doc = "The resultant role assignment schedule id or the role assignment schedule id being updated"]
    #[serde(rename = "targetRoleAssignmentScheduleId", default, skip_serializing_if = "Option::is_none")]
    pub target_role_assignment_schedule_id: Option<String>,
    #[doc = "The role assignment schedule instance id being updated"]
    #[serde(
        rename = "targetRoleAssignmentScheduleInstanceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_role_assignment_schedule_instance_id: Option<String>,
    #[doc = "Schedule info of the role assignment schedule"]
    #[serde(rename = "scheduleInfo", default, skip_serializing_if = "Option::is_none")]
    pub schedule_info: Option<role_assignment_schedule_request_properties::ScheduleInfo>,
    #[doc = "The linked role eligibility schedule id - to activate an eligibility."]
    #[serde(rename = "linkedRoleEligibilityScheduleId", default, skip_serializing_if = "Option::is_none")]
    pub linked_role_eligibility_schedule_id: Option<String>,
    #[doc = "Justification for the role assignment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
    #[doc = "Ticket Info of the role assignment"]
    #[serde(rename = "ticketInfo", default, skip_serializing_if = "Option::is_none")]
    pub ticket_info: Option<role_assignment_schedule_request_properties::TicketInfo>,
    #[doc = "The conditions on the role assignment. This limits the resources it can be assigned to. e.g.: @Resource[Microsoft.Storage/storageAccounts/blobServices/containers:ContainerName] StringEqualsIgnoreCase 'foo_storage_container'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Version of the condition. Currently accepted value is '2.0'"]
    #[serde(rename = "conditionVersion", default, skip_serializing_if = "Option::is_none")]
    pub condition_version: Option<String>,
    #[doc = "DateTime when role assignment schedule request was created"]
    #[serde(rename = "createdOn", with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Id of the user who created this request"]
    #[serde(rename = "requestorId", default, skip_serializing_if = "Option::is_none")]
    pub requestor_id: Option<String>,
    #[serde(rename = "expandedProperties", default, skip_serializing_if = "Option::is_none")]
    pub expanded_properties: Option<ExpandedProperties>,
}
impl RoleAssignmentScheduleRequestProperties {
    pub fn new(
        role_definition_id: String,
        principal_id: String,
        request_type: role_assignment_schedule_request_properties::RequestType,
    ) -> Self {
        Self {
            scope: None,
            role_definition_id,
            principal_id,
            principal_type: None,
            request_type,
            status: None,
            approval_id: None,
            target_role_assignment_schedule_id: None,
            target_role_assignment_schedule_instance_id: None,
            schedule_info: None,
            linked_role_eligibility_schedule_id: None,
            justification: None,
            ticket_info: None,
            condition: None,
            condition_version: None,
            created_on: None,
            requestor_id: None,
            expanded_properties: None,
        }
    }
}
pub mod role_assignment_schedule_request_properties {
    use super::*;
    #[doc = "The principal type of the assigned principal ID."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrincipalType")]
    pub enum PrincipalType {
        User,
        Group,
        ServicePrincipal,
        ForeignGroup,
        Device,
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
                Self::Device => serializer.serialize_unit_variant("PrincipalType", 4u32, "Device"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of the role assignment schedule request. Eg: SelfActivate, AdminAssign etc"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RequestType")]
    pub enum RequestType {
        AdminAssign,
        AdminRemove,
        AdminUpdate,
        AdminExtend,
        AdminRenew,
        SelfActivate,
        SelfDeactivate,
        SelfExtend,
        SelfRenew,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RequestType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RequestType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RequestType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AdminAssign => serializer.serialize_unit_variant("RequestType", 0u32, "AdminAssign"),
                Self::AdminRemove => serializer.serialize_unit_variant("RequestType", 1u32, "AdminRemove"),
                Self::AdminUpdate => serializer.serialize_unit_variant("RequestType", 2u32, "AdminUpdate"),
                Self::AdminExtend => serializer.serialize_unit_variant("RequestType", 3u32, "AdminExtend"),
                Self::AdminRenew => serializer.serialize_unit_variant("RequestType", 4u32, "AdminRenew"),
                Self::SelfActivate => serializer.serialize_unit_variant("RequestType", 5u32, "SelfActivate"),
                Self::SelfDeactivate => serializer.serialize_unit_variant("RequestType", 6u32, "SelfDeactivate"),
                Self::SelfExtend => serializer.serialize_unit_variant("RequestType", 7u32, "SelfExtend"),
                Self::SelfRenew => serializer.serialize_unit_variant("RequestType", 8u32, "SelfRenew"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the role assignment schedule request."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Accepted,
        PendingEvaluation,
        Granted,
        Denied,
        PendingProvisioning,
        Provisioned,
        PendingRevocation,
        Revoked,
        Canceled,
        Failed,
        PendingApprovalProvisioning,
        PendingApproval,
        FailedAsResourceIsLocked,
        PendingAdminDecision,
        AdminApproved,
        AdminDenied,
        TimedOut,
        ProvisioningStarted,
        Invalid,
        PendingScheduleCreation,
        ScheduleCreated,
        PendingExternalProvisioning,
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
                Self::Accepted => serializer.serialize_unit_variant("Status", 0u32, "Accepted"),
                Self::PendingEvaluation => serializer.serialize_unit_variant("Status", 1u32, "PendingEvaluation"),
                Self::Granted => serializer.serialize_unit_variant("Status", 2u32, "Granted"),
                Self::Denied => serializer.serialize_unit_variant("Status", 3u32, "Denied"),
                Self::PendingProvisioning => serializer.serialize_unit_variant("Status", 4u32, "PendingProvisioning"),
                Self::Provisioned => serializer.serialize_unit_variant("Status", 5u32, "Provisioned"),
                Self::PendingRevocation => serializer.serialize_unit_variant("Status", 6u32, "PendingRevocation"),
                Self::Revoked => serializer.serialize_unit_variant("Status", 7u32, "Revoked"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 8u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("Status", 9u32, "Failed"),
                Self::PendingApprovalProvisioning => serializer.serialize_unit_variant("Status", 10u32, "PendingApprovalProvisioning"),
                Self::PendingApproval => serializer.serialize_unit_variant("Status", 11u32, "PendingApproval"),
                Self::FailedAsResourceIsLocked => serializer.serialize_unit_variant("Status", 12u32, "FailedAsResourceIsLocked"),
                Self::PendingAdminDecision => serializer.serialize_unit_variant("Status", 13u32, "PendingAdminDecision"),
                Self::AdminApproved => serializer.serialize_unit_variant("Status", 14u32, "AdminApproved"),
                Self::AdminDenied => serializer.serialize_unit_variant("Status", 15u32, "AdminDenied"),
                Self::TimedOut => serializer.serialize_unit_variant("Status", 16u32, "TimedOut"),
                Self::ProvisioningStarted => serializer.serialize_unit_variant("Status", 17u32, "ProvisioningStarted"),
                Self::Invalid => serializer.serialize_unit_variant("Status", 18u32, "Invalid"),
                Self::PendingScheduleCreation => serializer.serialize_unit_variant("Status", 19u32, "PendingScheduleCreation"),
                Self::ScheduleCreated => serializer.serialize_unit_variant("Status", 20u32, "ScheduleCreated"),
                Self::PendingExternalProvisioning => serializer.serialize_unit_variant("Status", 21u32, "PendingExternalProvisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Schedule info of the role assignment schedule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ScheduleInfo {
        #[doc = "Start DateTime of the role assignment schedule."]
        #[serde(rename = "startDateTime", with = "azure_core::date::rfc3339::option")]
        pub start_date_time: Option<time::OffsetDateTime>,
        #[doc = "Expiration of the role assignment schedule"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub expiration: Option<schedule_info::Expiration>,
    }
    impl ScheduleInfo {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod schedule_info {
        use super::*;
        #[doc = "Expiration of the role assignment schedule"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Expiration {
            #[doc = "Type of the role assignment schedule expiration"]
            #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
            pub type_: Option<expiration::Type>,
            #[doc = "End DateTime of the role assignment schedule."]
            #[serde(rename = "endDateTime", with = "azure_core::date::rfc3339::option")]
            pub end_date_time: Option<time::OffsetDateTime>,
            #[doc = "Duration of the role assignment schedule in TimeSpan."]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub duration: Option<String>,
        }
        impl Expiration {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod expiration {
            use super::*;
            #[doc = "Type of the role assignment schedule expiration"]
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            #[serde(remote = "Type")]
            pub enum Type {
                AfterDuration,
                AfterDateTime,
                NoExpiration,
                #[serde(skip_deserializing)]
                UnknownValue(String),
            }
            impl FromStr for Type {
                type Err = value::Error;
                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    Self::deserialize(s.into_deserializer())
                }
            }
            impl<'de> Deserialize<'de> for Type {
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    let s = String::deserialize(deserializer)?;
                    let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                    Ok(deserialized)
                }
            }
            impl Serialize for Type {
                fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    match self {
                        Self::AfterDuration => serializer.serialize_unit_variant("Type", 0u32, "AfterDuration"),
                        Self::AfterDateTime => serializer.serialize_unit_variant("Type", 1u32, "AfterDateTime"),
                        Self::NoExpiration => serializer.serialize_unit_variant("Type", 2u32, "NoExpiration"),
                        Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                    }
                }
            }
        }
    }
    #[doc = "Ticket Info of the role assignment"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct TicketInfo {
        #[doc = "Ticket number for the role assignment"]
        #[serde(rename = "ticketNumber", default, skip_serializing_if = "Option::is_none")]
        pub ticket_number: Option<String>,
        #[doc = "Ticket system name for the role assignment"]
        #[serde(rename = "ticketSystem", default, skip_serializing_if = "Option::is_none")]
        pub ticket_system: Option<String>,
    }
    impl TicketInfo {
        pub fn new() -> Self {
            Self::default()
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
#[doc = "Role eligibility schedule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleEligibilitySchedule {
    #[doc = "The role eligibility schedule Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The role eligibility schedule name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The role eligibility schedule type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Role eligibility schedule properties with scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleEligibilityScheduleProperties>,
}
impl RoleEligibilitySchedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role eligibility schedule filter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleEligibilityScheduleFilter {
    #[doc = "Returns role eligibility schedule of the specific principal."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Returns role eligibility schedule of the specific role definition."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "Returns role eligibility schedule of the specific status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl RoleEligibilityScheduleFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about current or upcoming role eligibility schedule instance"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleEligibilityScheduleInstance {
    #[doc = "The role eligibility schedule instance ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The role eligibility schedule instance name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The role eligibility schedule instance type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Role eligibility schedule properties with scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleEligibilityScheduleInstanceProperties>,
}
impl RoleEligibilityScheduleInstance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role eligibility schedule instance filter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleEligibilityScheduleInstanceFilter {
    #[doc = "Returns role eligibility schedule instances of the specific principal."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Returns role eligibility schedule instances of the specific role definition."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "Returns role eligibility schedule instances of the specific status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Returns role eligibility schedule instances belonging to a specific role eligibility schedule."]
    #[serde(rename = "roleEligibilityScheduleId", default, skip_serializing_if = "Option::is_none")]
    pub role_eligibility_schedule_id: Option<String>,
}
impl RoleEligibilityScheduleInstanceFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role eligibility schedule instance list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleEligibilityScheduleInstanceListResult {
    #[doc = "Role eligibility schedule instance list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RoleEligibilityScheduleInstance>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleEligibilityScheduleInstanceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RoleEligibilityScheduleInstanceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role eligibility schedule properties with scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleEligibilityScheduleInstanceProperties {
    #[doc = "The role eligibility schedule scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The role definition ID."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "The principal ID."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The principal type of the assigned principal ID."]
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<role_eligibility_schedule_instance_properties::PrincipalType>,
    #[doc = "Id of the master role eligibility schedule"]
    #[serde(rename = "roleEligibilityScheduleId", default, skip_serializing_if = "Option::is_none")]
    pub role_eligibility_schedule_id: Option<String>,
    #[doc = "The status of the role eligibility schedule instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<role_eligibility_schedule_instance_properties::Status>,
    #[doc = "The startDateTime of the role eligibility schedule instance"]
    #[serde(rename = "startDateTime", with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "The endDateTime of the role eligibility schedule instance"]
    #[serde(rename = "endDateTime", with = "azure_core::date::rfc3339::option")]
    pub end_date_time: Option<time::OffsetDateTime>,
    #[doc = "Membership type of the role eligibility schedule"]
    #[serde(rename = "memberType", default, skip_serializing_if = "Option::is_none")]
    pub member_type: Option<role_eligibility_schedule_instance_properties::MemberType>,
    #[doc = "The conditions on the role assignment. This limits the resources it can be assigned to. e.g.: @Resource[Microsoft.Storage/storageAccounts/blobServices/containers:ContainerName] StringEqualsIgnoreCase 'foo_storage_container'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Version of the condition. Currently accepted value is '2.0'"]
    #[serde(rename = "conditionVersion", default, skip_serializing_if = "Option::is_none")]
    pub condition_version: Option<String>,
    #[doc = "DateTime when role eligibility schedule was created"]
    #[serde(rename = "createdOn", with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[serde(rename = "expandedProperties", default, skip_serializing_if = "Option::is_none")]
    pub expanded_properties: Option<ExpandedProperties>,
}
impl RoleEligibilityScheduleInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod role_eligibility_schedule_instance_properties {
    use super::*;
    #[doc = "The principal type of the assigned principal ID."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrincipalType")]
    pub enum PrincipalType {
        User,
        Group,
        ServicePrincipal,
        ForeignGroup,
        Device,
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
                Self::Device => serializer.serialize_unit_variant("PrincipalType", 4u32, "Device"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the role eligibility schedule instance"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Accepted,
        PendingEvaluation,
        Granted,
        Denied,
        PendingProvisioning,
        Provisioned,
        PendingRevocation,
        Revoked,
        Canceled,
        Failed,
        PendingApprovalProvisioning,
        PendingApproval,
        FailedAsResourceIsLocked,
        PendingAdminDecision,
        AdminApproved,
        AdminDenied,
        TimedOut,
        ProvisioningStarted,
        Invalid,
        PendingScheduleCreation,
        ScheduleCreated,
        PendingExternalProvisioning,
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
                Self::Accepted => serializer.serialize_unit_variant("Status", 0u32, "Accepted"),
                Self::PendingEvaluation => serializer.serialize_unit_variant("Status", 1u32, "PendingEvaluation"),
                Self::Granted => serializer.serialize_unit_variant("Status", 2u32, "Granted"),
                Self::Denied => serializer.serialize_unit_variant("Status", 3u32, "Denied"),
                Self::PendingProvisioning => serializer.serialize_unit_variant("Status", 4u32, "PendingProvisioning"),
                Self::Provisioned => serializer.serialize_unit_variant("Status", 5u32, "Provisioned"),
                Self::PendingRevocation => serializer.serialize_unit_variant("Status", 6u32, "PendingRevocation"),
                Self::Revoked => serializer.serialize_unit_variant("Status", 7u32, "Revoked"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 8u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("Status", 9u32, "Failed"),
                Self::PendingApprovalProvisioning => serializer.serialize_unit_variant("Status", 10u32, "PendingApprovalProvisioning"),
                Self::PendingApproval => serializer.serialize_unit_variant("Status", 11u32, "PendingApproval"),
                Self::FailedAsResourceIsLocked => serializer.serialize_unit_variant("Status", 12u32, "FailedAsResourceIsLocked"),
                Self::PendingAdminDecision => serializer.serialize_unit_variant("Status", 13u32, "PendingAdminDecision"),
                Self::AdminApproved => serializer.serialize_unit_variant("Status", 14u32, "AdminApproved"),
                Self::AdminDenied => serializer.serialize_unit_variant("Status", 15u32, "AdminDenied"),
                Self::TimedOut => serializer.serialize_unit_variant("Status", 16u32, "TimedOut"),
                Self::ProvisioningStarted => serializer.serialize_unit_variant("Status", 17u32, "ProvisioningStarted"),
                Self::Invalid => serializer.serialize_unit_variant("Status", 18u32, "Invalid"),
                Self::PendingScheduleCreation => serializer.serialize_unit_variant("Status", 19u32, "PendingScheduleCreation"),
                Self::ScheduleCreated => serializer.serialize_unit_variant("Status", 20u32, "ScheduleCreated"),
                Self::PendingExternalProvisioning => serializer.serialize_unit_variant("Status", 21u32, "PendingExternalProvisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Membership type of the role eligibility schedule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MemberType")]
    pub enum MemberType {
        Inherited,
        Direct,
        Group,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MemberType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MemberType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MemberType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Inherited => serializer.serialize_unit_variant("MemberType", 0u32, "Inherited"),
                Self::Direct => serializer.serialize_unit_variant("MemberType", 1u32, "Direct"),
                Self::Group => serializer.serialize_unit_variant("MemberType", 2u32, "Group"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "role eligibility schedule list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleEligibilityScheduleListResult {
    #[doc = "role eligibility schedule list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RoleEligibilitySchedule>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleEligibilityScheduleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RoleEligibilityScheduleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role eligibility schedule properties with scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleEligibilityScheduleProperties {
    #[doc = "The role eligibility schedule scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The role definition ID."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "The principal ID."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The principal type of the assigned principal ID."]
    #[serde(rename = "principalType", default, skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<role_eligibility_schedule_properties::PrincipalType>,
    #[doc = "The id of roleEligibilityScheduleRequest used to create this roleAssignmentSchedule"]
    #[serde(rename = "roleEligibilityScheduleRequestId", default, skip_serializing_if = "Option::is_none")]
    pub role_eligibility_schedule_request_id: Option<String>,
    #[doc = "Membership type of the role eligibility schedule"]
    #[serde(rename = "memberType", default, skip_serializing_if = "Option::is_none")]
    pub member_type: Option<role_eligibility_schedule_properties::MemberType>,
    #[doc = "The status of the role eligibility schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<role_eligibility_schedule_properties::Status>,
    #[doc = "Start DateTime when role eligibility schedule"]
    #[serde(rename = "startDateTime", with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "End DateTime when role eligibility schedule"]
    #[serde(rename = "endDateTime", with = "azure_core::date::rfc3339::option")]
    pub end_date_time: Option<time::OffsetDateTime>,
    #[doc = "The conditions on the role assignment. This limits the resources it can be assigned to. e.g.: @Resource[Microsoft.Storage/storageAccounts/blobServices/containers:ContainerName] StringEqualsIgnoreCase 'foo_storage_container'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Version of the condition. Currently accepted value is '2.0'"]
    #[serde(rename = "conditionVersion", default, skip_serializing_if = "Option::is_none")]
    pub condition_version: Option<String>,
    #[doc = "DateTime when role eligibility schedule was created"]
    #[serde(rename = "createdOn", with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "DateTime when role eligibility schedule was modified"]
    #[serde(rename = "updatedOn", with = "azure_core::date::rfc3339::option")]
    pub updated_on: Option<time::OffsetDateTime>,
    #[serde(rename = "expandedProperties", default, skip_serializing_if = "Option::is_none")]
    pub expanded_properties: Option<ExpandedProperties>,
}
impl RoleEligibilityScheduleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod role_eligibility_schedule_properties {
    use super::*;
    #[doc = "The principal type of the assigned principal ID."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrincipalType")]
    pub enum PrincipalType {
        User,
        Group,
        ServicePrincipal,
        ForeignGroup,
        Device,
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
                Self::Device => serializer.serialize_unit_variant("PrincipalType", 4u32, "Device"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Membership type of the role eligibility schedule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MemberType")]
    pub enum MemberType {
        Inherited,
        Direct,
        Group,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MemberType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MemberType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MemberType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Inherited => serializer.serialize_unit_variant("MemberType", 0u32, "Inherited"),
                Self::Direct => serializer.serialize_unit_variant("MemberType", 1u32, "Direct"),
                Self::Group => serializer.serialize_unit_variant("MemberType", 2u32, "Group"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the role eligibility schedule."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Accepted,
        PendingEvaluation,
        Granted,
        Denied,
        PendingProvisioning,
        Provisioned,
        PendingRevocation,
        Revoked,
        Canceled,
        Failed,
        PendingApprovalProvisioning,
        PendingApproval,
        FailedAsResourceIsLocked,
        PendingAdminDecision,
        AdminApproved,
        AdminDenied,
        TimedOut,
        ProvisioningStarted,
        Invalid,
        PendingScheduleCreation,
        ScheduleCreated,
        PendingExternalProvisioning,
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
                Self::Accepted => serializer.serialize_unit_variant("Status", 0u32, "Accepted"),
                Self::PendingEvaluation => serializer.serialize_unit_variant("Status", 1u32, "PendingEvaluation"),
                Self::Granted => serializer.serialize_unit_variant("Status", 2u32, "Granted"),
                Self::Denied => serializer.serialize_unit_variant("Status", 3u32, "Denied"),
                Self::PendingProvisioning => serializer.serialize_unit_variant("Status", 4u32, "PendingProvisioning"),
                Self::Provisioned => serializer.serialize_unit_variant("Status", 5u32, "Provisioned"),
                Self::PendingRevocation => serializer.serialize_unit_variant("Status", 6u32, "PendingRevocation"),
                Self::Revoked => serializer.serialize_unit_variant("Status", 7u32, "Revoked"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 8u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("Status", 9u32, "Failed"),
                Self::PendingApprovalProvisioning => serializer.serialize_unit_variant("Status", 10u32, "PendingApprovalProvisioning"),
                Self::PendingApproval => serializer.serialize_unit_variant("Status", 11u32, "PendingApproval"),
                Self::FailedAsResourceIsLocked => serializer.serialize_unit_variant("Status", 12u32, "FailedAsResourceIsLocked"),
                Self::PendingAdminDecision => serializer.serialize_unit_variant("Status", 13u32, "PendingAdminDecision"),
                Self::AdminApproved => serializer.serialize_unit_variant("Status", 14u32, "AdminApproved"),
                Self::AdminDenied => serializer.serialize_unit_variant("Status", 15u32, "AdminDenied"),
                Self::TimedOut => serializer.serialize_unit_variant("Status", 16u32, "TimedOut"),
                Self::ProvisioningStarted => serializer.serialize_unit_variant("Status", 17u32, "ProvisioningStarted"),
                Self::Invalid => serializer.serialize_unit_variant("Status", 18u32, "Invalid"),
                Self::PendingScheduleCreation => serializer.serialize_unit_variant("Status", 19u32, "PendingScheduleCreation"),
                Self::ScheduleCreated => serializer.serialize_unit_variant("Status", 20u32, "ScheduleCreated"),
                Self::PendingExternalProvisioning => serializer.serialize_unit_variant("Status", 21u32, "PendingExternalProvisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Role Eligibility schedule request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleEligibilityScheduleRequest {
    #[doc = "The role eligibility schedule request ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The role eligibility schedule request name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The role eligibility schedule request type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Role eligibility schedule request properties with scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleEligibilityScheduleRequestProperties>,
}
impl RoleEligibilityScheduleRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role eligibility schedule request filter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleEligibilityScheduleRequestFilter {
    #[doc = "Returns role eligibility requests of the specific principal."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Returns role eligibility requests of the specific role definition."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "Returns role eligibility requests created by specific principal."]
    #[serde(rename = "requestorId", default, skip_serializing_if = "Option::is_none")]
    pub requestor_id: Option<String>,
    #[doc = "Returns role eligibility requests of specific status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl RoleEligibilityScheduleRequestFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role eligibility schedule request list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleEligibilityScheduleRequestListResult {
    #[doc = "Role eligibility schedule request list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RoleEligibilityScheduleRequest>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleEligibilityScheduleRequestListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RoleEligibilityScheduleRequestListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role eligibility schedule request properties with scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleEligibilityScheduleRequestProperties {
    #[doc = "The role eligibility schedule request scope."]
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
    pub principal_type: Option<role_eligibility_schedule_request_properties::PrincipalType>,
    #[doc = "The type of the role assignment schedule request. Eg: SelfActivate, AdminAssign etc"]
    #[serde(rename = "requestType")]
    pub request_type: role_eligibility_schedule_request_properties::RequestType,
    #[doc = "The status of the role eligibility schedule request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<role_eligibility_schedule_request_properties::Status>,
    #[doc = "The approvalId of the role eligibility schedule request."]
    #[serde(rename = "approvalId", default, skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    #[doc = "Schedule info of the role eligibility schedule"]
    #[serde(rename = "scheduleInfo", default, skip_serializing_if = "Option::is_none")]
    pub schedule_info: Option<role_eligibility_schedule_request_properties::ScheduleInfo>,
    #[doc = "The resultant role eligibility schedule id or the role eligibility schedule id being updated"]
    #[serde(rename = "targetRoleEligibilityScheduleId", default, skip_serializing_if = "Option::is_none")]
    pub target_role_eligibility_schedule_id: Option<String>,
    #[doc = "The role eligibility schedule instance id being updated"]
    #[serde(
        rename = "targetRoleEligibilityScheduleInstanceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_role_eligibility_schedule_instance_id: Option<String>,
    #[doc = "Justification for the role eligibility"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
    #[doc = "Ticket Info of the role eligibility"]
    #[serde(rename = "ticketInfo", default, skip_serializing_if = "Option::is_none")]
    pub ticket_info: Option<role_eligibility_schedule_request_properties::TicketInfo>,
    #[doc = "The conditions on the role assignment. This limits the resources it can be assigned to. e.g.: @Resource[Microsoft.Storage/storageAccounts/blobServices/containers:ContainerName] StringEqualsIgnoreCase 'foo_storage_container'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Version of the condition. Currently accepted value is '2.0'"]
    #[serde(rename = "conditionVersion", default, skip_serializing_if = "Option::is_none")]
    pub condition_version: Option<String>,
    #[doc = "DateTime when role eligibility schedule request was created"]
    #[serde(rename = "createdOn", with = "azure_core::date::rfc3339::option")]
    pub created_on: Option<time::OffsetDateTime>,
    #[doc = "Id of the user who created this request"]
    #[serde(rename = "requestorId", default, skip_serializing_if = "Option::is_none")]
    pub requestor_id: Option<String>,
    #[doc = "Expanded info of resource, role and principal"]
    #[serde(rename = "expandedProperties", default, skip_serializing_if = "Option::is_none")]
    pub expanded_properties: Option<ExpandedProperties>,
}
impl RoleEligibilityScheduleRequestProperties {
    pub fn new(
        role_definition_id: String,
        principal_id: String,
        request_type: role_eligibility_schedule_request_properties::RequestType,
    ) -> Self {
        Self {
            scope: None,
            role_definition_id,
            principal_id,
            principal_type: None,
            request_type,
            status: None,
            approval_id: None,
            schedule_info: None,
            target_role_eligibility_schedule_id: None,
            target_role_eligibility_schedule_instance_id: None,
            justification: None,
            ticket_info: None,
            condition: None,
            condition_version: None,
            created_on: None,
            requestor_id: None,
            expanded_properties: None,
        }
    }
}
pub mod role_eligibility_schedule_request_properties {
    use super::*;
    #[doc = "The principal type of the assigned principal ID."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrincipalType")]
    pub enum PrincipalType {
        User,
        Group,
        ServicePrincipal,
        ForeignGroup,
        Device,
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
                Self::Device => serializer.serialize_unit_variant("PrincipalType", 4u32, "Device"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of the role assignment schedule request. Eg: SelfActivate, AdminAssign etc"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RequestType")]
    pub enum RequestType {
        AdminAssign,
        AdminRemove,
        AdminUpdate,
        AdminExtend,
        AdminRenew,
        SelfActivate,
        SelfDeactivate,
        SelfExtend,
        SelfRenew,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RequestType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RequestType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RequestType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AdminAssign => serializer.serialize_unit_variant("RequestType", 0u32, "AdminAssign"),
                Self::AdminRemove => serializer.serialize_unit_variant("RequestType", 1u32, "AdminRemove"),
                Self::AdminUpdate => serializer.serialize_unit_variant("RequestType", 2u32, "AdminUpdate"),
                Self::AdminExtend => serializer.serialize_unit_variant("RequestType", 3u32, "AdminExtend"),
                Self::AdminRenew => serializer.serialize_unit_variant("RequestType", 4u32, "AdminRenew"),
                Self::SelfActivate => serializer.serialize_unit_variant("RequestType", 5u32, "SelfActivate"),
                Self::SelfDeactivate => serializer.serialize_unit_variant("RequestType", 6u32, "SelfDeactivate"),
                Self::SelfExtend => serializer.serialize_unit_variant("RequestType", 7u32, "SelfExtend"),
                Self::SelfRenew => serializer.serialize_unit_variant("RequestType", 8u32, "SelfRenew"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the role eligibility schedule request."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Accepted,
        PendingEvaluation,
        Granted,
        Denied,
        PendingProvisioning,
        Provisioned,
        PendingRevocation,
        Revoked,
        Canceled,
        Failed,
        PendingApprovalProvisioning,
        PendingApproval,
        FailedAsResourceIsLocked,
        PendingAdminDecision,
        AdminApproved,
        AdminDenied,
        TimedOut,
        ProvisioningStarted,
        Invalid,
        PendingScheduleCreation,
        ScheduleCreated,
        PendingExternalProvisioning,
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
                Self::Accepted => serializer.serialize_unit_variant("Status", 0u32, "Accepted"),
                Self::PendingEvaluation => serializer.serialize_unit_variant("Status", 1u32, "PendingEvaluation"),
                Self::Granted => serializer.serialize_unit_variant("Status", 2u32, "Granted"),
                Self::Denied => serializer.serialize_unit_variant("Status", 3u32, "Denied"),
                Self::PendingProvisioning => serializer.serialize_unit_variant("Status", 4u32, "PendingProvisioning"),
                Self::Provisioned => serializer.serialize_unit_variant("Status", 5u32, "Provisioned"),
                Self::PendingRevocation => serializer.serialize_unit_variant("Status", 6u32, "PendingRevocation"),
                Self::Revoked => serializer.serialize_unit_variant("Status", 7u32, "Revoked"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 8u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("Status", 9u32, "Failed"),
                Self::PendingApprovalProvisioning => serializer.serialize_unit_variant("Status", 10u32, "PendingApprovalProvisioning"),
                Self::PendingApproval => serializer.serialize_unit_variant("Status", 11u32, "PendingApproval"),
                Self::FailedAsResourceIsLocked => serializer.serialize_unit_variant("Status", 12u32, "FailedAsResourceIsLocked"),
                Self::PendingAdminDecision => serializer.serialize_unit_variant("Status", 13u32, "PendingAdminDecision"),
                Self::AdminApproved => serializer.serialize_unit_variant("Status", 14u32, "AdminApproved"),
                Self::AdminDenied => serializer.serialize_unit_variant("Status", 15u32, "AdminDenied"),
                Self::TimedOut => serializer.serialize_unit_variant("Status", 16u32, "TimedOut"),
                Self::ProvisioningStarted => serializer.serialize_unit_variant("Status", 17u32, "ProvisioningStarted"),
                Self::Invalid => serializer.serialize_unit_variant("Status", 18u32, "Invalid"),
                Self::PendingScheduleCreation => serializer.serialize_unit_variant("Status", 19u32, "PendingScheduleCreation"),
                Self::ScheduleCreated => serializer.serialize_unit_variant("Status", 20u32, "ScheduleCreated"),
                Self::PendingExternalProvisioning => serializer.serialize_unit_variant("Status", 21u32, "PendingExternalProvisioning"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Schedule info of the role eligibility schedule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ScheduleInfo {
        #[doc = "Start DateTime of the role eligibility schedule."]
        #[serde(rename = "startDateTime", with = "azure_core::date::rfc3339::option")]
        pub start_date_time: Option<time::OffsetDateTime>,
        #[doc = "Expiration of the role eligibility schedule"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub expiration: Option<schedule_info::Expiration>,
    }
    impl ScheduleInfo {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod schedule_info {
        use super::*;
        #[doc = "Expiration of the role eligibility schedule"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct Expiration {
            #[doc = "Type of the role eligibility schedule expiration"]
            #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
            pub type_: Option<expiration::Type>,
            #[doc = "End DateTime of the role eligibility schedule."]
            #[serde(rename = "endDateTime", with = "azure_core::date::rfc3339::option")]
            pub end_date_time: Option<time::OffsetDateTime>,
            #[doc = "Duration of the role eligibility schedule in TimeSpan."]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub duration: Option<String>,
        }
        impl Expiration {
            pub fn new() -> Self {
                Self::default()
            }
        }
        pub mod expiration {
            use super::*;
            #[doc = "Type of the role eligibility schedule expiration"]
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            #[serde(remote = "Type")]
            pub enum Type {
                AfterDuration,
                AfterDateTime,
                NoExpiration,
                #[serde(skip_deserializing)]
                UnknownValue(String),
            }
            impl FromStr for Type {
                type Err = value::Error;
                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    Self::deserialize(s.into_deserializer())
                }
            }
            impl<'de> Deserialize<'de> for Type {
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    let s = String::deserialize(deserializer)?;
                    let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                    Ok(deserialized)
                }
            }
            impl Serialize for Type {
                fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    match self {
                        Self::AfterDuration => serializer.serialize_unit_variant("Type", 0u32, "AfterDuration"),
                        Self::AfterDateTime => serializer.serialize_unit_variant("Type", 1u32, "AfterDateTime"),
                        Self::NoExpiration => serializer.serialize_unit_variant("Type", 2u32, "NoExpiration"),
                        Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                    }
                }
            }
        }
    }
    #[doc = "Ticket Info of the role eligibility"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct TicketInfo {
        #[doc = "Ticket number for the role eligibility"]
        #[serde(rename = "ticketNumber", default, skip_serializing_if = "Option::is_none")]
        pub ticket_number: Option<String>,
        #[doc = "Ticket system name for the role eligibility"]
        #[serde(rename = "ticketSystem", default, skip_serializing_if = "Option::is_none")]
        pub ticket_system: Option<String>,
    }
    impl TicketInfo {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Role management policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleManagementPolicy {
    #[doc = "The role management policy Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The role management policy name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The role management policy type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Role management policy properties with scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleManagementPolicyProperties>,
}
impl RoleManagementPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The role management policy approval rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleManagementPolicyApprovalRule {
    #[serde(flatten)]
    pub role_management_policy_rule: RoleManagementPolicyRule,
    #[doc = "The approval settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub setting: Option<ApprovalSettings>,
}
impl RoleManagementPolicyApprovalRule {
    pub fn new(role_management_policy_rule: RoleManagementPolicyRule) -> Self {
        Self {
            role_management_policy_rule,
            setting: None,
        }
    }
}
#[doc = "Role management policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleManagementPolicyAssignment {
    #[doc = "The role management policy Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The role management policy name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The role management policy type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Role management policy assignment properties with scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleManagementPolicyAssignmentProperties>,
}
impl RoleManagementPolicyAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role management policy assignment list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleManagementPolicyAssignmentListResult {
    #[doc = "Role management policy assignment list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RoleManagementPolicyAssignment>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleManagementPolicyAssignmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RoleManagementPolicyAssignmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role management policy assignment properties with scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleManagementPolicyAssignmentProperties {
    #[doc = "The role management policy scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The role definition of management policy assignment."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "The policy id role management policy assignment."]
    #[serde(rename = "policyId", default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[doc = "The readonly computed rule applied to the policy."]
    #[serde(rename = "effectiveRules", default, skip_serializing_if = "Vec::is_empty")]
    pub effective_rules: Vec<RoleManagementPolicyRule>,
    #[doc = "Expanded info of resource scope, role definition and policy"]
    #[serde(rename = "policyAssignmentProperties", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_properties: Option<PolicyAssignmentProperties>,
}
impl RoleManagementPolicyAssignmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The role management policy authentication context rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleManagementPolicyAuthenticationContextRule {
    #[serde(flatten)]
    pub role_management_policy_rule: RoleManagementPolicyRule,
    #[doc = "The value indicating if rule is enabled."]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "The claim value."]
    #[serde(rename = "claimValue", default, skip_serializing_if = "Option::is_none")]
    pub claim_value: Option<String>,
}
impl RoleManagementPolicyAuthenticationContextRule {
    pub fn new(role_management_policy_rule: RoleManagementPolicyRule) -> Self {
        Self {
            role_management_policy_rule,
            is_enabled: None,
            claim_value: None,
        }
    }
}
#[doc = "The role management policy enablement rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleManagementPolicyEnablementRule {
    #[serde(flatten)]
    pub role_management_policy_rule: RoleManagementPolicyRule,
    #[doc = "The list of enabled rules."]
    #[serde(rename = "enabledRules", default, skip_serializing_if = "Vec::is_empty")]
    pub enabled_rules: Vec<String>,
}
impl RoleManagementPolicyEnablementRule {
    pub fn new(role_management_policy_rule: RoleManagementPolicyRule) -> Self {
        Self {
            role_management_policy_rule,
            enabled_rules: Vec::new(),
        }
    }
}
#[doc = "The role management policy expiration rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleManagementPolicyExpirationRule {
    #[serde(flatten)]
    pub role_management_policy_rule: RoleManagementPolicyRule,
    #[doc = "The value indicating whether expiration is required."]
    #[serde(rename = "isExpirationRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_expiration_required: Option<bool>,
    #[doc = "The maximum duration of expiration in timespan."]
    #[serde(rename = "maximumDuration", default, skip_serializing_if = "Option::is_none")]
    pub maximum_duration: Option<String>,
}
impl RoleManagementPolicyExpirationRule {
    pub fn new(role_management_policy_rule: RoleManagementPolicyRule) -> Self {
        Self {
            role_management_policy_rule,
            is_expiration_required: None,
            maximum_duration: None,
        }
    }
}
#[doc = "Role management policy list operation result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleManagementPolicyListResult {
    #[doc = "Role management policy list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RoleManagementPolicy>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RoleManagementPolicyListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RoleManagementPolicyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The role management policy notification rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleManagementPolicyNotificationRule {
    #[serde(flatten)]
    pub role_management_policy_rule: RoleManagementPolicyRule,
    #[doc = "The type of notification."]
    #[serde(rename = "notificationType", default, skip_serializing_if = "Option::is_none")]
    pub notification_type: Option<role_management_policy_notification_rule::NotificationType>,
    #[doc = "The notification level."]
    #[serde(rename = "notificationLevel", default, skip_serializing_if = "Option::is_none")]
    pub notification_level: Option<role_management_policy_notification_rule::NotificationLevel>,
    #[doc = "The recipient type."]
    #[serde(rename = "recipientType", default, skip_serializing_if = "Option::is_none")]
    pub recipient_type: Option<role_management_policy_notification_rule::RecipientType>,
    #[doc = "The list of notification recipients."]
    #[serde(rename = "notificationRecipients", default, skip_serializing_if = "Vec::is_empty")]
    pub notification_recipients: Vec<String>,
    #[doc = "Determines if the notification will be sent to the recipient type specified in the policy rule."]
    #[serde(rename = "isDefaultRecipientsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_default_recipients_enabled: Option<bool>,
}
impl RoleManagementPolicyNotificationRule {
    pub fn new(role_management_policy_rule: RoleManagementPolicyRule) -> Self {
        Self {
            role_management_policy_rule,
            notification_type: None,
            notification_level: None,
            recipient_type: None,
            notification_recipients: Vec::new(),
            is_default_recipients_enabled: None,
        }
    }
}
pub mod role_management_policy_notification_rule {
    use super::*;
    #[doc = "The type of notification."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NotificationType")]
    pub enum NotificationType {
        Email,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NotificationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NotificationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NotificationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Email => serializer.serialize_unit_variant("NotificationType", 0u32, "Email"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The notification level."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "NotificationLevel")]
    pub enum NotificationLevel {
        None,
        Critical,
        All,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for NotificationLevel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for NotificationLevel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for NotificationLevel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("NotificationLevel", 0u32, "None"),
                Self::Critical => serializer.serialize_unit_variant("NotificationLevel", 1u32, "Critical"),
                Self::All => serializer.serialize_unit_variant("NotificationLevel", 2u32, "All"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The recipient type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RecipientType")]
    pub enum RecipientType {
        Requestor,
        Approver,
        Admin,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RecipientType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RecipientType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RecipientType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Requestor => serializer.serialize_unit_variant("RecipientType", 0u32, "Requestor"),
                Self::Approver => serializer.serialize_unit_variant("RecipientType", 1u32, "Approver"),
                Self::Admin => serializer.serialize_unit_variant("RecipientType", 2u32, "Admin"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Role management policy properties with scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleManagementPolicyProperties {
    #[doc = "The role management policy scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The role management policy display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The role management policy description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The role management policy is default policy."]
    #[serde(rename = "isOrganizationDefault", default, skip_serializing_if = "Option::is_none")]
    pub is_organization_default: Option<bool>,
    #[doc = "The name of the entity last modified it"]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<Principal>,
    #[doc = "The last modified date time."]
    #[serde(rename = "lastModifiedDateTime", with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "The rule applied to the policy."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<RoleManagementPolicyRule>,
    #[doc = "The readonly computed rule applied to the policy."]
    #[serde(rename = "effectiveRules", default, skip_serializing_if = "Vec::is_empty")]
    pub effective_rules: Vec<RoleManagementPolicyRule>,
    #[doc = "Expanded info of resource scope"]
    #[serde(rename = "policyProperties", default, skip_serializing_if = "Option::is_none")]
    pub policy_properties: Option<PolicyProperties>,
}
impl RoleManagementPolicyProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The role management policy rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleManagementPolicyRule {
    #[doc = "The id of the rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of rule"]
    #[serde(rename = "ruleType")]
    pub rule_type: RoleManagementPolicyRuleType,
    #[doc = "The role management policy rule target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<RoleManagementPolicyRuleTarget>,
}
impl RoleManagementPolicyRule {
    pub fn new(rule_type: RoleManagementPolicyRuleType) -> Self {
        Self {
            id: None,
            rule_type,
            target: None,
        }
    }
}
#[doc = "The role management policy rule target."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleManagementPolicyRuleTarget {
    #[doc = "The caller of the setting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caller: Option<String>,
    #[doc = "The type of operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<String>,
    #[doc = "The assignment level to which rule is applied."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[doc = "The list of target objects."]
    #[serde(rename = "targetObjects", default, skip_serializing_if = "Vec::is_empty")]
    pub target_objects: Vec<String>,
    #[doc = "The list of inheritable settings."]
    #[serde(rename = "inheritableSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub inheritable_settings: Vec<String>,
    #[doc = "The list of enforced settings."]
    #[serde(rename = "enforcedSettings", default, skip_serializing_if = "Vec::is_empty")]
    pub enforced_settings: Vec<String>,
}
impl RoleManagementPolicyRuleTarget {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of rule"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RoleManagementPolicyRuleType")]
pub enum RoleManagementPolicyRuleType {
    RoleManagementPolicyApprovalRule,
    RoleManagementPolicyAuthenticationContextRule,
    RoleManagementPolicyEnablementRule,
    RoleManagementPolicyExpirationRule,
    RoleManagementPolicyNotificationRule,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RoleManagementPolicyRuleType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RoleManagementPolicyRuleType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RoleManagementPolicyRuleType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::RoleManagementPolicyApprovalRule => {
                serializer.serialize_unit_variant("RoleManagementPolicyRuleType", 0u32, "RoleManagementPolicyApprovalRule")
            }
            Self::RoleManagementPolicyAuthenticationContextRule => serializer.serialize_unit_variant(
                "RoleManagementPolicyRuleType",
                1u32,
                "RoleManagementPolicyAuthenticationContextRule",
            ),
            Self::RoleManagementPolicyEnablementRule => {
                serializer.serialize_unit_variant("RoleManagementPolicyRuleType", 2u32, "RoleManagementPolicyEnablementRule")
            }
            Self::RoleManagementPolicyExpirationRule => {
                serializer.serialize_unit_variant("RoleManagementPolicyRuleType", 3u32, "RoleManagementPolicyExpirationRule")
            }
            Self::RoleManagementPolicyNotificationRule => {
                serializer.serialize_unit_variant("RoleManagementPolicyRuleType", 4u32, "RoleManagementPolicyNotificationRule")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The detail of a user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserSet {
    #[doc = "The type of user."]
    #[serde(rename = "userType", default, skip_serializing_if = "Option::is_none")]
    pub user_type: Option<user_set::UserType>,
    #[doc = "The value indicating whether the user is a backup fallback approver"]
    #[serde(rename = "isBackup", default, skip_serializing_if = "Option::is_none")]
    pub is_backup: Option<bool>,
    #[doc = "The object id of the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The description of the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl UserSet {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod user_set {
    use super::*;
    #[doc = "The type of user."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "UserType")]
    pub enum UserType {
        User,
        Group,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for UserType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for UserType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for UserType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("UserType", 0u32, "User"),
                Self::Group => serializer.serialize_unit_variant("UserType", 1u32, "Group"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
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
