#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An integration application under space."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The properties of application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationProperties>,
}
impl Application {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response of a Application list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationListResult {
    #[doc = "The Application items on this page"]
    pub value: Vec<Application>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApplicationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ApplicationListResult {
    pub fn new(value: Vec<Application>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationProperties {
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The tracking data stores."]
    #[serde(rename = "trackingDataStores", default, skip_serializing_if = "Option::is_none")]
    pub tracking_data_stores: Option<serde_json::Value>,
}
impl ApplicationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A resource under application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of application resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationResourceProperties>,
}
impl ApplicationResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a ApplicationResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationResourceListResult {
    #[doc = "The ApplicationResource items on this page"]
    pub value: Vec<ApplicationResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApplicationResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ApplicationResourceListResult {
    pub fn new(value: Vec<ApplicationResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of application resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationResourceProperties {
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The type of the application resource."]
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[doc = "The Arm id of the application resource."]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "The kind of the application resource."]
    #[serde(rename = "resourceKind", default, skip_serializing_if = "Option::is_none")]
    pub resource_kind: Option<String>,
}
impl ApplicationResourceProperties {
    pub fn new(resource_type: String, resource_id: String) -> Self {
        Self {
            provisioning_state: None,
            resource_type,
            resource_id,
            resource_kind: None,
        }
    }
}
#[doc = "The type used for update operations of the ApplicationResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationResourceUpdate {
    #[doc = "The updatable properties of the ApplicationResource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationResourceUpdateProperties>,
}
impl ApplicationResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the ApplicationResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationResourceUpdateProperties {
    #[doc = "The type of the application resource."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The Arm id of the application resource."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The kind of the application resource."]
    #[serde(rename = "resourceKind", default, skip_serializing_if = "Option::is_none")]
    pub resource_kind: Option<String>,
}
impl ApplicationResourceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type used for update operations of the Application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the Application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationUpdateProperties>,
}
impl ApplicationUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the Application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationUpdateProperties {
    #[doc = "The description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The tracking data stores."]
    #[serde(rename = "trackingDataStores", default, skip_serializing_if = "Option::is_none")]
    pub tracking_data_stores: Option<serde_json::Value>,
}
impl ApplicationUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A business process under application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BusinessProcess {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of business process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BusinessProcessProperties>,
}
impl BusinessProcess {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of business process development artifact."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BusinessProcessDevelopmentArtifactProperties {
    #[doc = "The description of the business process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The properties of business process identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<BusinessProcessIdentifier>,
    #[doc = "The business process stages."]
    #[serde(rename = "businessProcessStages", default, skip_serializing_if = "Option::is_none")]
    pub business_process_stages: Option<serde_json::Value>,
    #[doc = "The business process mapping."]
    #[serde(rename = "businessProcessMapping", default, skip_serializing_if = "Option::is_none")]
    pub business_process_mapping: Option<serde_json::Value>,
    #[doc = "The tracking profile for the business process."]
    #[serde(rename = "trackingProfiles", default, skip_serializing_if = "Option::is_none")]
    pub tracking_profiles: Option<serde_json::Value>,
}
impl BusinessProcessDevelopmentArtifactProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of business process identifier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BusinessProcessIdentifier {
    #[doc = "The property name of the business process identifier."]
    #[serde(rename = "propertyName", default, skip_serializing_if = "Option::is_none")]
    pub property_name: Option<String>,
    #[doc = "The property type of the business process identifier."]
    #[serde(rename = "propertyType", default, skip_serializing_if = "Option::is_none")]
    pub property_type: Option<String>,
}
impl BusinessProcessIdentifier {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a BusinessProcess list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BusinessProcessListResult {
    #[doc = "The BusinessProcess items on this page"]
    pub value: Vec<BusinessProcess>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BusinessProcessListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BusinessProcessListResult {
    pub fn new(value: Vec<BusinessProcess>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of business process mapping."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BusinessProcessMappingItem {
    #[doc = "The logic app resource id."]
    #[serde(rename = "logicAppResourceId", default, skip_serializing_if = "Option::is_none")]
    pub logic_app_resource_id: Option<String>,
    #[doc = "The workflow name within the logic app."]
    #[serde(rename = "workflowName", default, skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,
    #[doc = "The operation name."]
    #[serde(rename = "operationName", default, skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[doc = "The mapping item operation type of the business process."]
    #[serde(rename = "operationType", default, skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
}
impl BusinessProcessMappingItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of business process."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BusinessProcessProperties {
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The version of the business process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The description of the business process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The table name of the business process."]
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[doc = "The tracking data store reference name."]
    #[serde(rename = "trackingDataStoreReferenceName", default, skip_serializing_if = "Option::is_none")]
    pub tracking_data_store_reference_name: Option<String>,
    #[doc = "The properties of business process identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<BusinessProcessIdentifier>,
    #[doc = "The business process stages."]
    #[serde(rename = "businessProcessStages", default, skip_serializing_if = "Option::is_none")]
    pub business_process_stages: Option<serde_json::Value>,
    #[doc = "The business process mapping."]
    #[serde(rename = "businessProcessMapping", default, skip_serializing_if = "Option::is_none")]
    pub business_process_mapping: Option<serde_json::Value>,
}
impl BusinessProcessProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The business process reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BusinessProcessReference {
    #[doc = "The business process name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The business process version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl BusinessProcessReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of business process stage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BusinessProcessStage {
    #[doc = "The description of the business stage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The properties within the properties of the business process stage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The property to keep track of stages before current in the business process stage."]
    #[serde(
        rename = "stagesBefore",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub stages_before: Vec<String>,
}
impl BusinessProcessStage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type used for update operations of the BusinessProcess."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BusinessProcessUpdate {
    #[doc = "The updatable properties of the BusinessProcess."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BusinessProcessUpdateProperties>,
}
impl BusinessProcessUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the BusinessProcess."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BusinessProcessUpdateProperties {
    #[doc = "The description of the business process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The table name of the business process."]
    #[serde(rename = "tableName", default, skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[doc = "The tracking data store reference name."]
    #[serde(rename = "trackingDataStoreReferenceName", default, skip_serializing_if = "Option::is_none")]
    pub tracking_data_store_reference_name: Option<String>,
    #[doc = "The properties of business process identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<BusinessProcessIdentifier>,
    #[doc = "The business process stages."]
    #[serde(rename = "businessProcessStages", default, skip_serializing_if = "Option::is_none")]
    pub business_process_stages: Option<serde_json::Value>,
    #[doc = "The business process mapping."]
    #[serde(rename = "businessProcessMapping", default, skip_serializing_if = "Option::is_none")]
    pub business_process_mapping: Option<serde_json::Value>,
}
impl BusinessProcessUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A business process version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BusinessProcessVersion {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of business process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BusinessProcessProperties>,
}
impl BusinessProcessVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a BusinessProcessVersion list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BusinessProcessVersionListResult {
    #[doc = "The BusinessProcessVersion items on this page"]
    pub value: Vec<BusinessProcessVersion>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BusinessProcessVersionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl BusinessProcessVersionListResult {
    pub fn new(value: Vec<BusinessProcessVersion>) -> Self {
        Self { value, next_link: None }
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
#[doc = "The workflow tracking definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FlowTrackingDefinition {
    #[doc = "The tracking correlation context."]
    #[serde(rename = "correlationContext", default, skip_serializing_if = "Option::is_none")]
    pub correlation_context: Option<TrackingCorrelationContext>,
    #[doc = "The tracking events."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<serde_json::Value>,
}
impl FlowTrackingDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The business process development artifact get or delete request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetOrDeleteBusinessProcessDevelopmentArtifactRequest {
    #[doc = "The name of the business process development artifact."]
    pub name: String,
}
impl GetOrDeleteBusinessProcessDevelopmentArtifactRequest {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[doc = "An infrastructure resource under Space."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InfrastructureResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of infrastructure resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InfrastructureResourceProperties>,
}
impl InfrastructureResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a InfrastructureResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfrastructureResourceListResult {
    #[doc = "The InfrastructureResource items on this page"]
    pub value: Vec<InfrastructureResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for InfrastructureResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl InfrastructureResourceListResult {
    pub fn new(value: Vec<InfrastructureResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of infrastructure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfrastructureResourceProperties {
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The type of the infrastructure resource."]
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[doc = "The id of the infrastructure resource."]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
}
impl InfrastructureResourceProperties {
    pub fn new(resource_type: String, resource_id: String) -> Self {
        Self {
            provisioning_state: None,
            resource_type,
            resource_id,
        }
    }
}
#[doc = "The type used for update operations of the InfrastructureResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InfrastructureResourceUpdate {
    #[doc = "The updatable properties of the InfrastructureResource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<InfrastructureResourceUpdateProperties>,
}
impl InfrastructureResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the InfrastructureResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InfrastructureResourceUpdateProperties {
    #[doc = "The type of the infrastructure resource."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The id of the infrastructure resource."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl InfrastructureResourceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The business process development artifact get collection response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListBusinessProcessDevelopmentArtifactsResponse {
    #[doc = "The list of the business process development artifact."]
    pub value: Vec<SaveOrGetBusinessProcessDevelopmentArtifactResponse>,
}
impl ListBusinessProcessDevelopmentArtifactsResponse {
    pub fn new(value: Vec<SaveOrGetBusinessProcessDevelopmentArtifactResponse>) -> Self {
        Self { value }
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
#[doc = "The status of the current operation."]
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
#[doc = "The business process development artifact save or get response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SaveOrGetBusinessProcessDevelopmentArtifactResponse {
    #[doc = "The name of the business process development artifact."]
    pub name: String,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The properties of business process development artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BusinessProcessDevelopmentArtifactProperties>,
}
impl SaveOrGetBusinessProcessDevelopmentArtifactResponse {
    pub fn new(name: String) -> Self {
        Self {
            name,
            system_data: None,
            properties: None,
        }
    }
}
#[doc = "The business process development artifact save or validate request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SaveOrValidateBusinessProcessDevelopmentArtifactRequest {
    #[doc = "The name of the business process development artifact."]
    pub name: String,
    #[doc = "The properties of business process development artifact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BusinessProcessDevelopmentArtifactProperties>,
}
impl SaveOrValidateBusinessProcessDevelopmentArtifactRequest {
    pub fn new(name: String) -> Self {
        Self { name, properties: None }
    }
}
#[doc = "An integration space."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Space {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The properties of space."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SpaceResourceProperties>,
}
impl Space {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response of a Space list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpaceListResult {
    #[doc = "The Space items on this page"]
    pub value: Vec<Space>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SpaceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SpaceListResult {
    pub fn new(value: Vec<Space>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of space."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SpaceResourceProperties {
    #[doc = "The status of the current operation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl SpaceResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type used for update operations of the Space."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SpaceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the Space."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SpaceUpdateProperties>,
}
impl SpaceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the Space."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SpaceUpdateProperties {
    #[doc = "The description of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl SpaceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The business process development artifact success response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SuccessfulBusinessProcessDevelopmentArtifactResponse {}
impl SuccessfulBusinessProcessDevelopmentArtifactResponse {
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
#[doc = "The tracking correlation context."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackingCorrelationContext {
    #[doc = "The operation type for correlation context."]
    #[serde(rename = "operationType", default, skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
    #[doc = "The operation name for correlation context."]
    #[serde(rename = "operationName", default, skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[doc = "The name of the correlation property."]
    #[serde(rename = "propertyName", default, skip_serializing_if = "Option::is_none")]
    pub property_name: Option<String>,
    #[doc = "The template expression for correlation context property value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl TrackingCorrelationContext {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of tracking data store."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackingDataStore {
    #[doc = "The database name."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "The data store resource id."]
    #[serde(rename = "dataStoreResourceId", default, skip_serializing_if = "Option::is_none")]
    pub data_store_resource_id: Option<String>,
    #[doc = "The data store URI."]
    #[serde(rename = "dataStoreUri", default, skip_serializing_if = "Option::is_none")]
    pub data_store_uri: Option<String>,
    #[doc = "The data store ingestion URI."]
    #[serde(rename = "dataStoreIngestionUri", default, skip_serializing_if = "Option::is_none")]
    pub data_store_ingestion_uri: Option<String>,
}
impl TrackingDataStore {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The tracking event definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackingEventDefinition {
    #[doc = "The operation type."]
    #[serde(rename = "operationType", default, skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
    #[doc = "The operation name."]
    #[serde(rename = "operationName", default, skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[doc = "The properties to be collected for event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl TrackingEventDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The tracking profile for the business process"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackingProfileDefinition {
    #[doc = "The tracking definition schema uri."]
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[doc = "The business process reference."]
    #[serde(rename = "businessProcess", default, skip_serializing_if = "Option::is_none")]
    pub business_process: Option<BusinessProcessReference>,
    #[doc = "The tracking definitions."]
    #[serde(rename = "trackingDefinitions", default, skip_serializing_if = "Option::is_none")]
    pub tracking_definitions: Option<serde_json::Value>,
}
impl TrackingProfileDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represent a model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Object {}
impl Object {
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
    pub created_at: Option<::time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<::time::OffsetDateTime>,
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
