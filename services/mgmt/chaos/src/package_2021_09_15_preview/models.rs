#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "Model that represents the base action model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Action {
    #[doc = "Enum that discriminates between action models."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "String that represents a URN."]
    pub name: Urn,
}
impl Action {
    pub fn new(type_: String, name: Urn) -> Self {
        Self { type_, name }
    }
}
#[doc = "Model that represents the an action and its status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionStatus {
    #[doc = "The name of the action status."]
    #[serde(rename = "actionName", default, skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    #[doc = "The id of the action status."]
    #[serde(rename = "actionId", default, skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[doc = "The status of the action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "String that represents the start time of the action."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "String that represents the end time of the action."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The array of targets."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub targets: Vec<ExperimentExecutionActionTargetDetailsProperties>,
}
impl ActionStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model that represents a branch in the step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Branch {
    #[doc = "String of the branch name."]
    pub name: String,
    #[doc = "List of actions."]
    pub actions: Vec<Action>,
}
impl Branch {
    pub fn new(name: String, actions: Vec<Action>) -> Self {
        Self { name, actions }
    }
}
#[doc = "Model that represents the a list of actions and action statuses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BranchStatus {
    #[doc = "The name of the branch status."]
    #[serde(rename = "branchName", default, skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    #[doc = "The id of the branch status."]
    #[serde(rename = "branchId", default, skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    #[doc = "The status of the branch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The array of actions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<ActionStatus>,
}
impl BranchStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model that represents a Capability resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Capability {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Model that represents the Capability properties model."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CapabilityProperties>,
}
impl Capability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model that represents a list of Capability resources and a link for pagination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapabilityListResult {
    #[doc = "List of Capability resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Capability>,
    #[doc = "Optional string that represents a URL."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<UrlNullable>,
}
impl azure_core::Continuable for CapabilityListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CapabilityListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model that represents the Capability properties model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapabilityProperties {
    #[doc = "String of the Publisher that this Capability extends."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "String of the Target Type that this Capability extends."]
    #[serde(rename = "targetType", default, skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    #[doc = "Localized string of the description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "String that represents a URL."]
    #[serde(rename = "parametersSchema", default, skip_serializing_if = "Option::is_none")]
    pub parameters_schema: Option<Url>,
    #[doc = "String that represents a URN."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub urn: Option<Urn>,
}
impl CapabilityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model that represents a Capability Type resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapabilityType {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Location of the Capability Type resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Model that represents the Capability Type properties model."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CapabilityTypeProperties>,
}
impl CapabilityType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model that represents a list of Capability Type resources and a link for pagination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapabilityTypeListResult {
    #[doc = "List of Capability Type resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CapabilityType>,
    #[doc = "Optional string that represents a URL."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<UrlNullable>,
}
impl azure_core::Continuable for CapabilityTypeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CapabilityTypeListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model that represents the Capability Type properties model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CapabilityTypeProperties {
    #[doc = "String of the Publisher that this Capability Type extends."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "String of the Target Type that this Capability Type extends."]
    #[serde(rename = "targetType", default, skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    #[doc = "Localized string of the display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Localized string of the description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "String that represents a URL."]
    #[serde(rename = "parametersSchema", default, skip_serializing_if = "Option::is_none")]
    pub parameters_schema: Option<Url>,
    #[doc = "String that represents a URN."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub urn: Option<Urn>,
}
impl CapabilityTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model that represents a Experiment resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Experiment {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The managed identity of a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ResourceIdentity>,
    #[doc = "Model that represents the Experiment properties model."]
    pub properties: ExperimentProperties,
}
impl Experiment {
    pub fn new(tracked_resource: TrackedResource, properties: ExperimentProperties) -> Self {
        Self {
            tracked_resource,
            system_data: None,
            identity: None,
            properties,
        }
    }
}
#[doc = "Model that represents the result of a cancel Experiment operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExperimentCancelOperationResult {
    #[doc = "String of the Experiment name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "String that represents a URL."]
    #[serde(rename = "statusUrl", default, skip_serializing_if = "Option::is_none")]
    pub status_url: Option<Url>,
}
impl ExperimentCancelOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model that represents the Experiment action target details error model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExperimentExecutionActionTargetDetailsError {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ExperimentExecutionActionTargetDetailsError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model that represents the Experiment action target details properties model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExperimentExecutionActionTargetDetailsProperties {
    #[doc = "The status of the execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The target for the action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "String that represents the failed date time."]
    #[serde(rename = "targetFailedTime", with = "azure_core::date::rfc3339::option")]
    pub target_failed_time: Option<time::OffsetDateTime>,
    #[doc = "String that represents the completed date time."]
    #[serde(rename = "targetCompletedTime", with = "azure_core::date::rfc3339::option")]
    pub target_completed_time: Option<time::OffsetDateTime>,
    #[doc = "Model that represents the Experiment action target details error model."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ExperimentExecutionActionTargetDetailsError>,
}
impl ExperimentExecutionActionTargetDetailsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model that represents the execution details of a Experiment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExperimentExecutionDetails {
    #[doc = "String of the resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "String of the fully qualified resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "String of the resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Model that represents the Experiment execution details properties model."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExperimentExecutionDetailsProperties>,
}
impl ExperimentExecutionDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model that represents a list of Experiment execution details and a link for pagination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExperimentExecutionDetailsListResult {
    #[doc = "List of Experiment execution details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ExperimentExecutionDetails>,
    #[doc = "Optional string that represents a URL."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<UrlNullable>,
}
impl azure_core::Continuable for ExperimentExecutionDetailsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ExperimentExecutionDetailsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model that represents the Experiment execution details properties model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExperimentExecutionDetailsProperties {
    #[doc = "The id of the experiment."]
    #[serde(rename = "experimentId", default, skip_serializing_if = "Option::is_none")]
    pub experiment_id: Option<String>,
    #[doc = "The value of the status of the experiment execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The reason why the execution failed."]
    #[serde(rename = "failureReason", default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[doc = "String that represents the created date time."]
    #[serde(rename = "createdDateTime", with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "String that represents the last action date time."]
    #[serde(rename = "lastActionDateTime", with = "azure_core::date::rfc3339::option")]
    pub last_action_date_time: Option<time::OffsetDateTime>,
    #[doc = "String that represents the start date time."]
    #[serde(rename = "startDateTime", with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "String that represents the stop date time."]
    #[serde(rename = "stopDateTime", with = "azure_core::date::rfc3339::option")]
    pub stop_date_time: Option<time::OffsetDateTime>,
    #[doc = "The information of the experiment run."]
    #[serde(rename = "runInformation", default, skip_serializing_if = "Option::is_none")]
    pub run_information: Option<experiment_execution_details_properties::RunInformation>,
}
impl ExperimentExecutionDetailsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod experiment_execution_details_properties {
    use super::*;
    #[doc = "The information of the experiment run."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct RunInformation {
        #[doc = "The steps of the experiment run."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub steps: Vec<StepStatus>,
    }
    impl RunInformation {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Model that represents a list of Experiment resources and a link for pagination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExperimentListResult {
    #[doc = "List of Experiment resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Experiment>,
    #[doc = "Optional string that represents a URL."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<UrlNullable>,
}
impl azure_core::Continuable for ExperimentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ExperimentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model that represents the Experiment properties model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExperimentProperties {
    #[doc = "List of steps."]
    pub steps: Vec<Step>,
    #[doc = "List of selectors."]
    pub selectors: Vec<Selector>,
    #[doc = "A boolean value that indicates if experiment should be started on creation or not."]
    #[serde(rename = "startOnCreation", default, skip_serializing_if = "Option::is_none")]
    pub start_on_creation: Option<bool>,
}
impl ExperimentProperties {
    pub fn new(steps: Vec<Step>, selectors: Vec<Selector>) -> Self {
        Self {
            steps,
            selectors,
            start_on_creation: None,
        }
    }
}
#[doc = "Model that represents the result of a start Experiment operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExperimentStartOperationResult {
    #[doc = "String of the Experiment name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "String that represents a URL."]
    #[serde(rename = "statusUrl", default, skip_serializing_if = "Option::is_none")]
    pub status_url: Option<Url>,
}
impl ExperimentStartOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model that represents the status of a Experiment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExperimentStatus {
    #[doc = "String of the resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "String of the fully qualified resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "String of the resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Model that represents the Experiment status properties model."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExperimentStatusProperties>,
}
impl ExperimentStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model that represents a list of Experiment statuses and a link for pagination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExperimentStatusListResult {
    #[doc = "List of Experiment statuses."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ExperimentStatus>,
    #[doc = "Optional string that represents a URL."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<UrlNullable>,
}
impl azure_core::Continuable for ExperimentStatusListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ExperimentStatusListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model that represents the Experiment status properties model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExperimentStatusProperties {
    #[doc = "String that represents the status of a Experiment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "String that represents the created date time of a Experiment."]
    #[serde(rename = "createdDateUtc", with = "azure_core::date::rfc3339::option")]
    pub created_date_utc: Option<time::OffsetDateTime>,
    #[doc = "String that represents the end date time of a Experiment."]
    #[serde(rename = "endDateUtc", with = "azure_core::date::rfc3339::option")]
    pub end_date_utc: Option<time::OffsetDateTime>,
}
impl ExperimentStatusProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The managed identity of a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceIdentity {
    #[doc = "String of the resource identity type."]
    #[serde(rename = "type")]
    pub type_: resource_identity::Type,
    #[doc = "GUID that represents the principal ID of this resource identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "GUID that represents the tenant ID of this resource identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl ResourceIdentity {
    pub fn new(type_: resource_identity::Type) -> Self {
        Self {
            type_,
            principal_id: None,
            tenant_id: None,
        }
    }
}
pub mod resource_identity {
    use super::*;
    #[doc = "String of the resource identity type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        None,
        SystemAssigned,
    }
}
#[doc = "Model that represents a selector in the Experiment resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Selector {
    #[doc = "Enum of the selector type."]
    #[serde(rename = "type")]
    pub type_: selector::Type,
    #[doc = "String of the selector ID."]
    pub id: String,
    #[doc = "List of Target references."]
    pub targets: Vec<TargetReference>,
}
impl Selector {
    pub fn new(type_: selector::Type, id: String, targets: Vec<TargetReference>) -> Self {
        Self { type_, id, targets }
    }
}
pub mod selector {
    use super::*;
    #[doc = "Enum of the selector type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Percent,
        Random,
        Tag,
        List,
    }
}
#[doc = "Model that represents a step in the Experiment resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Step {
    #[doc = "String of the step name."]
    pub name: String,
    #[doc = "List of branches."]
    pub branches: Vec<Branch>,
}
impl Step {
    pub fn new(name: String, branches: Vec<Branch>) -> Self {
        Self { name, branches }
    }
}
#[doc = "Model that represents the a list of branches and branch statuses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StepStatus {
    #[doc = "The name of the step."]
    #[serde(rename = "stepName", default, skip_serializing_if = "Option::is_none")]
    pub step_name: Option<String>,
    #[doc = "The id of the step."]
    #[serde(rename = "stepId", default, skip_serializing_if = "Option::is_none")]
    pub step_id: Option<String>,
    #[doc = "The value of the status of the step."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The array of branches."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub branches: Vec<BranchStatus>,
}
impl StepStatus {
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
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
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
#[doc = "Model that represents a Target resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Target {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Location of the target resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Model that represents the base Target properties model."]
    pub properties: TargetProperties,
}
impl Target {
    pub fn new(properties: TargetProperties) -> Self {
        Self {
            resource: Resource::default(),
            system_data: None,
            location: None,
            properties,
        }
    }
}
#[doc = "Model that represents a list of Target resources and a link for pagination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TargetListResult {
    #[doc = "List of Target resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Target>,
    #[doc = "Optional string that represents a URL."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<UrlNullable>,
}
impl azure_core::Continuable for TargetListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TargetListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model that represents the base Target properties model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TargetProperties {}
impl TargetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model that represents a reference to a Target in the selector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetReference {
    #[doc = "Enum of the Target reference type."]
    #[serde(rename = "type")]
    pub type_: target_reference::Type,
    #[doc = "String of the resource ID of a Target resource."]
    pub id: String,
}
impl TargetReference {
    pub fn new(type_: target_reference::Type, id: String) -> Self {
        Self { type_, id }
    }
}
pub mod target_reference {
    use super::*;
    #[doc = "Enum of the Target reference type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        ChaosTarget,
    }
}
#[doc = "Model that represents a Target Type resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetType {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Location of the Target Type resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Model that represents the base Target Type properties model."]
    pub properties: TargetTypeProperties,
}
impl TargetType {
    pub fn new(properties: TargetTypeProperties) -> Self {
        Self {
            resource: Resource::default(),
            system_data: None,
            location: None,
            properties,
        }
    }
}
#[doc = "Model that represents a list of Target Type resources and a link for pagination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TargetTypeListResult {
    #[doc = "List of Target Type resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TargetType>,
    #[doc = "Optional string that represents a URL."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<UrlNullable>,
}
impl azure_core::Continuable for TargetTypeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TargetTypeListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Model that represents the base Target Type properties model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TargetTypeProperties {
    #[doc = "Localized string of the display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Localized string of the description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "String that represents a URL."]
    #[serde(rename = "propertiesSchema", default, skip_serializing_if = "Option::is_none")]
    pub properties_schema: Option<Url>,
    #[doc = "List of resource types this Target Type can extend."]
    #[serde(rename = "resourceTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_types: Vec<String>,
}
impl TargetTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type Url = String;
pub type UrlNullable = String;
pub type Urn = String;
