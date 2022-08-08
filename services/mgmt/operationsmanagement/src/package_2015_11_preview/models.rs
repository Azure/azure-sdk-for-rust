#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Parameter to pass to ARM template"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmTemplateParameter {
    #[doc = "name of the parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "value for the parameter. In Jtoken "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ArmTemplateParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error body contract."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CodeMessageError {
    #[doc = "The error details for a failed request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<code_message_error::Error>,
}
impl CodeMessageError {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod code_message_error {
    use super::*;
    #[doc = "The error details for a failed request."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[doc = "The error type."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "The error message."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The container for solution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementAssociation {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "ManagementAssociation properties supported by the OperationsManagement resource provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagementAssociationProperties>,
}
impl ManagementAssociation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ManagementAssociation properties supported by the OperationsManagement resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementAssociationProperties {
    #[doc = "The applicationId of the appliance for this association."]
    #[serde(rename = "applicationId")]
    pub application_id: String,
}
impl ManagementAssociationProperties {
    pub fn new(application_id: String) -> Self {
        Self { application_id }
    }
}
#[doc = "the list of ManagementAssociation response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementAssociationPropertiesList {
    #[doc = "List of Management Association properties within the subscription."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagementAssociation>,
}
impl ManagementAssociationPropertiesList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The container for solution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementConfiguration {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "ManagementConfiguration properties supported by the OperationsManagement resource provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagementConfigurationProperties>,
}
impl ManagementConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ManagementConfiguration properties supported by the OperationsManagement resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagementConfigurationProperties {
    #[doc = "The applicationId of the appliance for this Management."]
    #[serde(rename = "applicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[doc = "The type of the parent resource."]
    #[serde(rename = "parentResourceType")]
    pub parent_resource_type: String,
    #[doc = "Parameters to run the ARM template"]
    pub parameters: Vec<ArmTemplateParameter>,
    #[doc = "The provisioning state for the ManagementConfiguration."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The Json object containing the ARM template to deploy"]
    pub template: serde_json::Value,
}
impl ManagementConfigurationProperties {
    pub fn new(parent_resource_type: String, parameters: Vec<ArmTemplateParameter>, template: serde_json::Value) -> Self {
        Self {
            application_id: None,
            parent_resource_type,
            parameters,
            provisioning_state: None,
            template,
        }
    }
}
#[doc = "the list of ManagementConfiguration response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementConfigurationPropertiesList {
    #[doc = "List of Management Configuration properties within the subscription."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagementConfiguration>,
}
impl ManagementConfigurationPropertiesList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Supported operation of OperationsManagement resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display metadata associated with the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Display metadata associated with the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft OperationsManagement."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Type of operation: get, read, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list solution operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of solution operations supported by the OperationsManagement resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The container for solution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Solution {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Plan for solution object supported by the OperationsManagement resource provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<SolutionPlan>,
    #[doc = "Solution properties supported by the OperationsManagement resource provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SolutionProperties>,
}
impl Solution {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a Solution that can be patched."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SolutionPatch {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl SolutionPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Plan for solution object supported by the OperationsManagement resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SolutionPlan {
    #[doc = "name of the solution to be created. For Microsoft published solution it should be in the format of solutionType(workspaceName). SolutionType part is case sensitive. For third party solution, it can be anything."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Publisher name. For gallery solution, it is Microsoft."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "promotionCode, Not really used now, can you left as empty"]
    #[serde(rename = "promotionCode", default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    #[doc = "name of the solution to enabled/add. For Microsoft published gallery solution it should be in the format of OMSGallery/<solutionType>. This is case sensitive"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
}
impl SolutionPlan {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Solution properties supported by the OperationsManagement resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SolutionProperties {
    #[doc = "The azure resourceId for the workspace where the solution will be deployed/enabled."]
    #[serde(rename = "workspaceResourceId")]
    pub workspace_resource_id: String,
    #[doc = "The provisioning state for the solution."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The azure resources that will be contained within the solutions. They will be locked and gets deleted automatically when the solution is deleted."]
    #[serde(rename = "containedResources", default, skip_serializing_if = "Vec::is_empty")]
    pub contained_resources: Vec<String>,
    #[doc = "The resources that will be referenced from this solution. Deleting any of those solution out of band will break the solution."]
    #[serde(rename = "referencedResources", default, skip_serializing_if = "Vec::is_empty")]
    pub referenced_resources: Vec<String>,
}
impl SolutionProperties {
    pub fn new(workspace_resource_id: String) -> Self {
        Self {
            workspace_resource_id,
            provisioning_state: None,
            contained_resources: Vec::new(),
            referenced_resources: Vec::new(),
        }
    }
}
#[doc = "the list of solution response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SolutionPropertiesList {
    #[doc = "List of solution properties within the subscription."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Solution>,
}
impl SolutionPropertiesList {
    pub fn new() -> Self {
        Self::default()
    }
}
