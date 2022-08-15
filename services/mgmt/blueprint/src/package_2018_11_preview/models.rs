#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Represents a blueprint artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Artifact {
    #[serde(flatten)]
    pub azure_resource_base: AzureResourceBase,
    #[doc = "Specifies the kind of blueprint artifact."]
    pub kind: artifact::Kind,
}
impl Artifact {
    pub fn new(kind: artifact::Kind) -> Self {
        Self {
            azure_resource_base: AzureResourceBase::default(),
            kind,
        }
    }
}
pub mod artifact {
    use super::*;
    #[doc = "Specifies the kind of blueprint artifact."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        #[serde(rename = "template")]
        Template,
        #[serde(rename = "roleAssignment")]
        RoleAssignment,
        #[serde(rename = "policyAssignment")]
        PolicyAssignment,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Template => serializer.serialize_unit_variant("Kind", 0u32, "template"),
                Self::RoleAssignment => serializer.serialize_unit_variant("Kind", 1u32, "roleAssignment"),
                Self::PolicyAssignment => serializer.serialize_unit_variant("Kind", 2u32, "policyAssignment"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of blueprint artifacts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactList {
    #[doc = "List of blueprint artifacts."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Artifact>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ArtifactList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ArtifactList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common properties shared by different artifacts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactPropertiesBase {
    #[doc = "Artifacts which need to be deployed before the specified artifact."]
    #[serde(rename = "dependsOn", default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
}
impl ArtifactPropertiesBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a blueprint assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Assignment {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Managed identity generic object."]
    pub identity: ManagedServiceIdentity,
    #[doc = "Detailed properties for a blueprint assignment."]
    pub properties: AssignmentProperties,
}
impl Assignment {
    pub fn new(tracked_resource: TrackedResource, identity: ManagedServiceIdentity, properties: AssignmentProperties) -> Self {
        Self {
            tracked_resource,
            identity,
            properties,
        }
    }
}
#[doc = "Represents individual job in given blueprint assignment operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssignmentDeploymentJob {
    #[doc = "Kind of job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Name of the action performed in this job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[doc = "Id of this job."]
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "State of this job."]
    #[serde(rename = "jobState", default, skip_serializing_if = "Option::is_none")]
    pub job_state: Option<String>,
    #[doc = "Result of each individual deployment in a blueprint assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<AssignmentDeploymentJobResult>,
    #[doc = "Result of this deployment job for each retry."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub history: Vec<AssignmentDeploymentJobResult>,
    #[doc = "Reference to deployment job resource id."]
    #[serde(rename = "requestUri", default, skip_serializing_if = "Option::is_none")]
    pub request_uri: Option<String>,
}
impl AssignmentDeploymentJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of each individual deployment in a blueprint assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssignmentDeploymentJobResult {
    #[doc = "Error code and message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<AzureResourceManagerError>,
    #[doc = "Resources created as result of the deployment job."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<AssignmentJobCreatedResource>,
}
impl AssignmentDeploymentJobResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure resource created from deployment job."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssignmentJobCreatedResource {
    #[serde(flatten)]
    pub azure_resource_base: AzureResourceBase,
    #[doc = "Additional properties in a dictionary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl AssignmentJobCreatedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of blueprint assignments"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssignmentList {
    #[doc = "List of blueprint assignments."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Assignment>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AssignmentList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AssignmentList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines how resources deployed by a blueprint assignment are locked."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssignmentLockSettings {
    #[doc = "Lock mode."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<assignment_lock_settings::Mode>,
    #[doc = "List of AAD principals excluded from blueprint locks. Up to 5 principals are permitted."]
    #[serde(rename = "excludedPrincipals", default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_principals: Vec<String>,
    #[doc = "List\u{a0}of\u{a0}management\u{a0}operations\u{a0}that\u{a0}are\u{a0}excluded\u{a0}from\u{a0}blueprint\u{a0}locks.\u{a0}Up\u{a0}to\u{a0}200\u{a0}actions\u{a0}are\u{a0}permitted. If the lock mode is set to 'AllResourcesReadOnly', then the following actions are automatically appended to 'excludedActions': '*/read', 'Microsoft.Network/virtualNetworks/subnets/join/action' and 'Microsoft.Authorization/locks/delete'. If the lock mode is set to 'AllResourcesDoNotDelete', then the following actions are automatically appended to 'excludedActions': 'Microsoft.Authorization/locks/delete'. Duplicate actions will get removed."]
    #[serde(rename = "excludedActions", default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_actions: Vec<String>,
}
impl AssignmentLockSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod assignment_lock_settings {
    use super::*;
    #[doc = "Lock mode."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Mode")]
    pub enum Mode {
        None,
        AllResourcesReadOnly,
        AllResourcesDoNotDelete,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Mode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Mode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Mode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Mode", 0u32, "None"),
                Self::AllResourcesReadOnly => serializer.serialize_unit_variant("Mode", 1u32, "AllResourcesReadOnly"),
                Self::AllResourcesDoNotDelete => serializer.serialize_unit_variant("Mode", 2u32, "AllResourcesDoNotDelete"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents underlying deployment detail for each update to the blueprint assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssignmentOperation {
    #[serde(flatten)]
    pub azure_resource_base: AzureResourceBase,
    #[doc = "Properties of AssignmentOperation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AssignmentOperationProperties>,
}
impl AssignmentOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of AssignmentOperation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssignmentOperationList {
    #[doc = "List of AssignmentOperation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AssignmentOperation>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AssignmentOperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AssignmentOperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of AssignmentOperation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssignmentOperationProperties {
    #[doc = "The published version of the blueprint definition used for the blueprint assignment operation."]
    #[serde(rename = "blueprintVersion", default, skip_serializing_if = "Option::is_none")]
    pub blueprint_version: Option<String>,
    #[doc = "State of this blueprint assignment operation."]
    #[serde(rename = "assignmentState", default, skip_serializing_if = "Option::is_none")]
    pub assignment_state: Option<String>,
    #[doc = "Create time of this blueprint assignment operation."]
    #[serde(rename = "timeCreated", default, skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    #[doc = "Start time of the underlying deployment."]
    #[serde(rename = "timeStarted", default, skip_serializing_if = "Option::is_none")]
    pub time_started: Option<String>,
    #[doc = "Finish time of the overall underlying deployments."]
    #[serde(rename = "timeFinished", default, skip_serializing_if = "Option::is_none")]
    pub time_finished: Option<String>,
    #[doc = "List of jobs in this blueprint assignment operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub deployments: Vec<AssignmentDeploymentJob>,
}
impl AssignmentOperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Detailed properties for a blueprint assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssignmentProperties {
    #[serde(flatten)]
    pub blueprint_resource_properties_base: BlueprintResourcePropertiesBase,
    #[doc = "ID of the published version of a blueprint definition."]
    #[serde(rename = "blueprintId", default, skip_serializing_if = "Option::is_none")]
    pub blueprint_id: Option<String>,
    #[doc = "The target subscription scope of the blueprint assignment (format: '/subscriptions/{subscriptionId}'). For management group level assignments, the property is required."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "Blueprint assignment parameter values."]
    pub parameters: serde_json::Value,
    #[doc = "Names and locations of resource group placeholders."]
    #[serde(rename = "resourceGroups")]
    pub resource_groups: serde_json::Value,
    #[doc = "The status of a blueprint assignment. This field is readonly."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AssignmentStatus>,
    #[doc = "Defines how resources deployed by a blueprint assignment are locked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locks: Option<AssignmentLockSettings>,
    #[doc = "State of the blueprint assignment."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<assignment_properties::ProvisioningState>,
}
impl AssignmentProperties {
    pub fn new(parameters: serde_json::Value, resource_groups: serde_json::Value) -> Self {
        Self {
            blueprint_resource_properties_base: BlueprintResourcePropertiesBase::default(),
            blueprint_id: None,
            scope: None,
            parameters,
            resource_groups,
            status: None,
            locks: None,
            provisioning_state: None,
        }
    }
}
pub mod assignment_properties {
    use super::*;
    #[doc = "State of the blueprint assignment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        #[serde(rename = "creating")]
        Creating,
        #[serde(rename = "validating")]
        Validating,
        #[serde(rename = "waiting")]
        Waiting,
        #[serde(rename = "deploying")]
        Deploying,
        #[serde(rename = "cancelling")]
        Cancelling,
        #[serde(rename = "locking")]
        Locking,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "canceled")]
        Canceled,
        #[serde(rename = "deleting")]
        Deleting,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "creating"),
                Self::Validating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "validating"),
                Self::Waiting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "waiting"),
                Self::Deploying => serializer.serialize_unit_variant("ProvisioningState", 3u32, "deploying"),
                Self::Cancelling => serializer.serialize_unit_variant("ProvisioningState", 4u32, "cancelling"),
                Self::Locking => serializer.serialize_unit_variant("ProvisioningState", 5u32, "locking"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 6u32, "succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 7u32, "failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 8u32, "canceled"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 9u32, "deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The status of a blueprint assignment. This field is readonly."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AssignmentStatus {
    #[serde(flatten)]
    pub blueprint_resource_status_base: BlueprintResourceStatusBase,
    #[doc = "List of resources that were created by the blueprint assignment."]
    #[serde(rename = "managedResources", default, skip_serializing_if = "Vec::is_empty")]
    pub managed_resources: Vec<String>,
}
impl AssignmentStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common properties for all Azure resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureResourceBase {
    #[doc = "String Id used to locate any resource on Azure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Type of this resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Name of this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl AzureResourceBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error code and message"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureResourceManagerError {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl AzureResourceManagerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a Blueprint definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Blueprint {
    #[serde(flatten)]
    pub azure_resource_base: AzureResourceBase,
    #[doc = "Schema for blueprint definition properties."]
    pub properties: BlueprintProperties,
}
impl Blueprint {
    pub fn new(properties: BlueprintProperties) -> Self {
        Self {
            azure_resource_base: AzureResourceBase::default(),
            properties,
        }
    }
}
#[doc = "List of blueprint definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlueprintList {
    #[doc = "List of blueprint definitions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Blueprint>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BlueprintList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BlueprintList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema for blueprint definition properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlueprintProperties {
    #[serde(flatten)]
    pub shared_blueprint_properties: SharedBlueprintProperties,
    #[doc = "Published versions of this blueprint definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub versions: Option<serde_json::Value>,
    #[doc = "Layout view of the blueprint definition for UI reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<serde_json::Value>,
}
impl BlueprintProperties {
    pub fn new() -> Self {
        Self {
            shared_blueprint_properties: SharedBlueprintProperties::default(),
            versions: None,
            layout: None,
        }
    }
}
#[doc = "Shared properties between all blueprint resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlueprintResourcePropertiesBase {
    #[doc = "One-liner string explain this resource."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Multi-line explain this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl BlueprintResourcePropertiesBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Shared status properties between all blueprint resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlueprintResourceStatusBase {
    #[doc = "Creation time of this blueprint definition."]
    #[serde(rename = "timeCreated", with = "azure_core::date::rfc3339::option")]
    pub time_created: Option<time::OffsetDateTime>,
    #[doc = "Last modified time of this blueprint definition."]
    #[serde(rename = "lastModified", with = "azure_core::date::rfc3339::option")]
    pub last_modified: Option<time::OffsetDateTime>,
}
impl BlueprintResourceStatusBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of the blueprint. This field is readonly."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BlueprintStatus {
    #[serde(flatten)]
    pub blueprint_resource_status_base: BlueprintResourceStatusBase,
}
impl BlueprintStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
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
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
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
    pub details: Vec<ErrorResponse>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed identity generic object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedServiceIdentity {
    #[doc = "Type of the managed identity."]
    #[serde(rename = "type")]
    pub type_: managed_service_identity::Type,
    #[doc = "Azure Active Directory principal ID associated with this Identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "ID of the Azure Active Directory."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The list of user-assigned managed identities associated with the resource. Key is the Azure resource Id of the managed identity."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl ManagedServiceIdentity {
    pub fn new(type_: managed_service_identity::Type) -> Self {
        Self {
            type_,
            principal_id: None,
            tenant_id: None,
            user_assigned_identities: None,
        }
    }
}
pub mod managed_service_identity {
    use super::*;
    #[doc = "Type of the managed identity."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        None,
        SystemAssigned,
        UserAssigned,
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
                Self::None => serializer.serialize_unit_variant("Type", 0u32, "None"),
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 1u32, "SystemAssigned"),
                Self::UserAssigned => serializer.serialize_unit_variant("Type", 2u32, "UserAssigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represent a parameter with constrains and metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterDefinition {
    #[doc = "Allowed data types for Resource Manager template parameters."]
    #[serde(rename = "type")]
    pub type_: parameter_definition::Type,
    #[doc = "User-friendly properties for this parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ParameterDefinitionMetadata>,
    #[doc = "Default Value for this parameter."]
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,
    #[doc = "Array of allowed values for this parameter."]
    #[serde(rename = "allowedValues", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_values: Vec<serde_json::Value>,
}
impl ParameterDefinition {
    pub fn new(type_: parameter_definition::Type) -> Self {
        Self {
            type_,
            metadata: None,
            default_value: None,
            allowed_values: Vec::new(),
        }
    }
}
pub mod parameter_definition {
    use super::*;
    #[doc = "Allowed data types for Resource Manager template parameters."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "string")]
        String,
        #[serde(rename = "array")]
        Array,
        #[serde(rename = "bool")]
        Bool,
        #[serde(rename = "int")]
        Int,
        #[serde(rename = "object")]
        Object,
        #[serde(rename = "secureObject")]
        SecureObject,
        #[serde(rename = "secureString")]
        SecureString,
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
                Self::String => serializer.serialize_unit_variant("Type", 0u32, "string"),
                Self::Array => serializer.serialize_unit_variant("Type", 1u32, "array"),
                Self::Bool => serializer.serialize_unit_variant("Type", 2u32, "bool"),
                Self::Int => serializer.serialize_unit_variant("Type", 3u32, "int"),
                Self::Object => serializer.serialize_unit_variant("Type", 4u32, "object"),
                Self::SecureObject => serializer.serialize_unit_variant("Type", 5u32, "secureObject"),
                Self::SecureString => serializer.serialize_unit_variant("Type", 6u32, "secureString"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "User-friendly properties for this parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParameterDefinitionMetadata {
    #[doc = "DisplayName of this parameter/resourceGroup."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Description of this parameter/resourceGroup."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "StrongType for UI to render rich experience during blueprint assignment. Supported strong types are resourceType, principalId and location."]
    #[serde(rename = "strongType", default, skip_serializing_if = "Option::is_none")]
    pub strong_type: Option<String>,
}
impl ParameterDefinitionMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Value for the specified parameter. Can be either 'value' or 'reference' but not both."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParameterValue {
    #[doc = "Parameter value. Any valid JSON value is allowed including objects, arrays, strings, numbers and booleans."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[doc = "Reference to a Key Vault secret."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<SecretValueReference>,
}
impl ParameterValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Blueprint artifact that applies a Policy assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyAssignmentArtifact {
    #[serde(flatten)]
    pub artifact: Artifact,
    #[doc = "Properties of a Policy assignment blueprint artifact."]
    pub properties: PolicyAssignmentArtifactProperties,
}
impl PolicyAssignmentArtifact {
    pub fn new(artifact: Artifact, properties: PolicyAssignmentArtifactProperties) -> Self {
        Self { artifact, properties }
    }
}
#[doc = "Properties of a Policy assignment blueprint artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolicyAssignmentArtifactProperties {
    #[serde(flatten)]
    pub blueprint_resource_properties_base: BlueprintResourcePropertiesBase,
    #[serde(flatten)]
    pub artifact_properties_base: ArtifactPropertiesBase,
    #[doc = "Azure resource ID of the policy definition."]
    #[serde(rename = "policyDefinitionId")]
    pub policy_definition_id: String,
    #[doc = "Parameter values for the policy definition."]
    pub parameters: serde_json::Value,
    #[doc = "Name of the resource group placeholder to which the policy will be assigned."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
impl PolicyAssignmentArtifactProperties {
    pub fn new(policy_definition_id: String, parameters: serde_json::Value) -> Self {
        Self {
            blueprint_resource_properties_base: BlueprintResourcePropertiesBase::default(),
            artifact_properties_base: ArtifactPropertiesBase::default(),
            policy_definition_id,
            parameters,
            resource_group: None,
        }
    }
}
#[doc = "Represents a published blueprint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublishedBlueprint {
    #[serde(flatten)]
    pub azure_resource_base: AzureResourceBase,
    #[doc = "Schema for published blueprint definition properties."]
    pub properties: PublishedBlueprintProperties,
}
impl PublishedBlueprint {
    pub fn new(properties: PublishedBlueprintProperties) -> Self {
        Self {
            azure_resource_base: AzureResourceBase::default(),
            properties,
        }
    }
}
#[doc = "List of published blueprint definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublishedBlueprintList {
    #[doc = "List of published blueprint definitions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PublishedBlueprint>,
    #[doc = "Link to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PublishedBlueprintList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PublishedBlueprintList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema for published blueprint definition properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PublishedBlueprintProperties {
    #[serde(flatten)]
    pub shared_blueprint_properties: SharedBlueprintProperties,
    #[doc = "Name of the published blueprint definition."]
    #[serde(rename = "blueprintName", default, skip_serializing_if = "Option::is_none")]
    pub blueprint_name: Option<String>,
    #[doc = "Version-specific change notes."]
    #[serde(rename = "changeNotes", default, skip_serializing_if = "Option::is_none")]
    pub change_notes: Option<String>,
}
impl PublishedBlueprintProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an Azure resource group in a blueprint definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGroupDefinition {
    #[doc = "Name of this resourceGroup. Leave empty if the resource group name will be specified during the blueprint assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Location of this resourceGroup. Leave empty if the resource group location will be specified during the blueprint assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "User-friendly properties for this parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ParameterDefinitionMetadata>,
    #[doc = "Artifacts which need to be deployed before this resource group."]
    #[serde(rename = "dependsOn", default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
    #[doc = "Tags to be assigned to this resource group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ResourceGroupDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an Azure resource group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGroupValue {
    #[doc = "Name of the resource group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Location of the resource group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl ResourceGroupValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Supported operations of this resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperation {
    #[doc = "Operation name, in format of {provider}/{resource}/{operation}"]
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
        #[doc = "Resource provider: Microsoft Blueprint."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Type of operation: get, read, delete, etc."]
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
    #[doc = "List of operations supported by this resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceProviderOperation>,
}
impl ResourceProviderOperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Blueprint artifact that applies a Role assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentArtifact {
    #[serde(flatten)]
    pub artifact: Artifact,
    #[doc = "Properties of a Role assignment blueprint artifact."]
    pub properties: RoleAssignmentArtifactProperties,
}
impl RoleAssignmentArtifact {
    pub fn new(artifact: Artifact, properties: RoleAssignmentArtifactProperties) -> Self {
        Self { artifact, properties }
    }
}
#[doc = "Properties of a Role assignment blueprint artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignmentArtifactProperties {
    #[serde(flatten)]
    pub blueprint_resource_properties_base: BlueprintResourcePropertiesBase,
    #[serde(flatten)]
    pub artifact_properties_base: ArtifactPropertiesBase,
    #[doc = "Azure resource ID of the RoleDefinition."]
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
    #[doc = "Array of user or group identities in Azure Active Directory. The roleDefinition will apply to each identity."]
    #[serde(rename = "principalIds")]
    pub principal_ids: serde_json::Value,
    #[doc = "RoleAssignment will be scope to this resourceGroup. If empty, it scopes to the subscription."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
}
impl RoleAssignmentArtifactProperties {
    pub fn new(role_definition_id: String, principal_ids: serde_json::Value) -> Self {
        Self {
            blueprint_resource_properties_base: BlueprintResourcePropertiesBase::default(),
            artifact_properties_base: ArtifactPropertiesBase::default(),
            role_definition_id,
            principal_ids,
            resource_group: None,
        }
    }
}
#[doc = "Reference to a Key Vault secret."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretValueReference {
    #[doc = "Specifies the link to a Key Vault."]
    #[serde(rename = "keyVault")]
    pub key_vault: KeyVaultReference,
    #[doc = "Name of the secret."]
    #[serde(rename = "secretName")]
    pub secret_name: String,
    #[doc = "The version of the secret to use. If left blank, the latest version of the secret is used."]
    #[serde(rename = "secretVersion", default, skip_serializing_if = "Option::is_none")]
    pub secret_version: Option<String>,
}
impl SecretValueReference {
    pub fn new(key_vault: KeyVaultReference, secret_name: String) -> Self {
        Self {
            key_vault,
            secret_name,
            secret_version: None,
        }
    }
}
#[doc = "Shared Schema for both blueprintProperties and publishedBlueprintProperties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedBlueprintProperties {
    #[serde(flatten)]
    pub blueprint_resource_properties_base: BlueprintResourcePropertiesBase,
    #[doc = "The status of the blueprint. This field is readonly."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<BlueprintStatus>,
    #[doc = "The scope where this blueprint definition can be assigned."]
    #[serde(rename = "targetScope", default, skip_serializing_if = "Option::is_none")]
    pub target_scope: Option<shared_blueprint_properties::TargetScope>,
    #[doc = "Parameters required by this blueprint definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Resource group placeholders defined by this blueprint definition."]
    #[serde(rename = "resourceGroups", default, skip_serializing_if = "Option::is_none")]
    pub resource_groups: Option<serde_json::Value>,
}
impl SharedBlueprintProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod shared_blueprint_properties {
    use super::*;
    #[doc = "The scope where this blueprint definition can be assigned."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TargetScope")]
    pub enum TargetScope {
        #[serde(rename = "subscription")]
        Subscription,
        #[serde(rename = "managementGroup")]
        ManagementGroup,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TargetScope {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TargetScope {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TargetScope {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Subscription => serializer.serialize_unit_variant("TargetScope", 0u32, "subscription"),
                Self::ManagementGroup => serializer.serialize_unit_variant("TargetScope", 1u32, "managementGroup"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Blueprint artifact that deploys a Resource Manager template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemplateArtifact {
    #[serde(flatten)]
    pub artifact: Artifact,
    #[doc = "Properties of a Resource Manager template blueprint artifact."]
    pub properties: TemplateArtifactProperties,
}
impl TemplateArtifact {
    pub fn new(artifact: Artifact, properties: TemplateArtifactProperties) -> Self {
        Self { artifact, properties }
    }
}
#[doc = "Properties of a Resource Manager template blueprint artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemplateArtifactProperties {
    #[serde(flatten)]
    pub blueprint_resource_properties_base: BlueprintResourcePropertiesBase,
    #[serde(flatten)]
    pub artifact_properties_base: ArtifactPropertiesBase,
    #[doc = "The Resource Manager template blueprint artifact body."]
    pub template: serde_json::Value,
    #[doc = "If applicable, the name of the resource group placeholder to which the Resource Manager template blueprint artifact will be deployed."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "Resource Manager template blueprint artifact parameter values."]
    pub parameters: serde_json::Value,
}
impl TemplateArtifactProperties {
    pub fn new(template: serde_json::Value, parameters: serde_json::Value) -> Self {
        Self {
            blueprint_resource_properties_base: BlueprintResourcePropertiesBase::default(),
            artifact_properties_base: ArtifactPropertiesBase::default(),
            template,
            resource_group: None,
            parameters,
        }
    }
}
#[doc = "Common properties for all Azure tracked resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub azure_resource_base: AzureResourceBase,
    #[doc = "The location of this blueprint assignment."]
    pub location: String,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            azure_resource_base: AzureResourceBase::default(),
            location,
        }
    }
}
#[doc = "User-assigned managed identity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "Azure Active Directory principal ID associated with this Identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Client App Id associated with this identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response schema for querying the Azure Blueprints service principal in the tenant."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WhoIsBlueprintContract {
    #[doc = "AAD object Id of the Azure Blueprints service principal in the tenant."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
}
impl WhoIsBlueprintContract {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the link to a Key Vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultReference {
    #[doc = "Azure resource ID of the Key Vault."]
    pub id: String,
}
impl KeyVaultReference {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
