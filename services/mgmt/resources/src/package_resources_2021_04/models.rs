#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The alias type. "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Alias {
    #[doc = "The alias name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The paths for an alias."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<AliasPath>,
    #[doc = "The type of the alias."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<alias::Type>,
    #[doc = "The default path for an alias."]
    #[serde(rename = "defaultPath", default, skip_serializing_if = "Option::is_none")]
    pub default_path: Option<String>,
    #[doc = "The type of the pattern for an alias path."]
    #[serde(rename = "defaultPattern", default, skip_serializing_if = "Option::is_none")]
    pub default_pattern: Option<AliasPattern>,
    #[serde(rename = "defaultMetadata", default, skip_serializing_if = "Option::is_none")]
    pub default_metadata: Option<AliasPathMetadata>,
}
impl Alias {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod alias {
    use super::*;
    #[doc = "The type of the alias."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        NotSpecified,
        PlainText,
        Mask,
    }
}
#[doc = "The type of the paths for alias."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AliasPath {
    #[doc = "The path of an alias."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The API versions."]
    #[serde(rename = "apiVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub api_versions: Vec<String>,
    #[doc = "The type of the pattern for an alias path."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<AliasPattern>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<AliasPathMetadata>,
}
impl AliasPath {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AliasPathMetadata {
    #[doc = "The type of the token that the alias path is referring to."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<alias_path_metadata::Type>,
    #[doc = "The attributes of the token that the alias path is referring to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<alias_path_metadata::Attributes>,
}
impl AliasPathMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod alias_path_metadata {
    use super::*;
    #[doc = "The type of the token that the alias path is referring to."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        NotSpecified,
        Any,
        String,
        Object,
        Array,
        Integer,
        Number,
        Boolean,
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
                Self::NotSpecified => serializer.serialize_unit_variant("Type", 0u32, "NotSpecified"),
                Self::Any => serializer.serialize_unit_variant("Type", 1u32, "Any"),
                Self::String => serializer.serialize_unit_variant("Type", 2u32, "String"),
                Self::Object => serializer.serialize_unit_variant("Type", 3u32, "Object"),
                Self::Array => serializer.serialize_unit_variant("Type", 4u32, "Array"),
                Self::Integer => serializer.serialize_unit_variant("Type", 5u32, "Integer"),
                Self::Number => serializer.serialize_unit_variant("Type", 6u32, "Number"),
                Self::Boolean => serializer.serialize_unit_variant("Type", 7u32, "Boolean"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The attributes of the token that the alias path is referring to."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Attributes")]
    pub enum Attributes {
        None,
        Modifiable,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Attributes {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Attributes {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Attributes {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Attributes", 0u32, "None"),
                Self::Modifiable => serializer.serialize_unit_variant("Attributes", 1u32, "Modifiable"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The type of the pattern for an alias path."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AliasPattern {
    #[doc = "The alias pattern phrase."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phrase: Option<String>,
    #[doc = "The alias pattern variable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable: Option<String>,
    #[doc = "The type of alias pattern"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<alias_pattern::Type>,
}
impl AliasPattern {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod alias_pattern {
    use super::*;
    #[doc = "The type of alias pattern"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        NotSpecified,
        Extract,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiProfile {
    #[doc = "The profile version."]
    #[serde(rename = "profileVersion", default, skip_serializing_if = "Option::is_none")]
    pub profile_version: Option<String>,
    #[doc = "The API version."]
    #[serde(rename = "apiVersion", default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
}
impl ApiProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment dependency information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BasicDependency {
    #[doc = "The ID of the dependency."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The dependency resource type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The dependency resource name."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}
impl BasicDependency {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response for a resource management request."]
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
#[doc = "The debug setting."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DebugSetting {
    #[doc = "Specifies the type of information to log for debugging. The permitted values are none, requestContent, responseContent, or both requestContent and responseContent separated by a comma. The default is none. When setting this value, carefully consider the type of information you are passing in during deployment. By logging information about the request or response, you could potentially expose sensitive data that is retrieved through the deployment operations."]
    #[serde(rename = "detailLevel", default, skip_serializing_if = "Option::is_none")]
    pub detail_level: Option<String>,
}
impl DebugSetting {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment dependency information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Dependency {
    #[doc = "The list of dependencies."]
    #[serde(rename = "dependsOn", default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<BasicDependency>,
    #[doc = "The ID of the dependency."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The dependency resource type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The dependency resource name."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}
impl Dependency {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment operation parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Deployment {
    #[doc = "The location to store the deployment data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Deployment properties."]
    pub properties: DeploymentProperties,
    #[doc = "Deployment tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Deployment {
    pub fn new(properties: DeploymentProperties) -> Self {
        Self {
            location: None,
            properties,
            tags: None,
        }
    }
}
#[doc = "The deployment export result. "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentExportResult {
    #[doc = "The template content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<serde_json::Value>,
}
impl DeploymentExportResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentExtended {
    #[doc = "The ID of the deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the deployment."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "the location of the deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Deployment properties with additional details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeploymentPropertiesExtended>,
    #[doc = "Deployment tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DeploymentExtended {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentExtendedFilter {
    #[doc = "The provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl DeploymentExtendedFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of deployments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentListResult {
    #[doc = "An array of deployments."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DeploymentExtended>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeploymentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeploymentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment operation information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentOperation {
    #[doc = "Full deployment operation ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Deployment operation ID."]
    #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[doc = "Deployment operation properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeploymentOperationProperties>,
}
impl DeploymentOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment operation properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentOperationProperties {
    #[doc = "The name of the current provisioning operation."]
    #[serde(rename = "provisioningOperation", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_operation: Option<deployment_operation_properties::ProvisioningOperation>,
    #[doc = "The state of the provisioning."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The date and time of the operation."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "The duration of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "Deployment operation service request id."]
    #[serde(rename = "serviceRequestId", default, skip_serializing_if = "Option::is_none")]
    pub service_request_id: Option<String>,
    #[doc = "Operation status code from the resource provider. This property may not be set if a response has not yet been received."]
    #[serde(rename = "statusCode", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[doc = "Operation status message object."]
    #[serde(rename = "statusMessage", default, skip_serializing_if = "Option::is_none")]
    pub status_message: Option<StatusMessage>,
    #[doc = "Target resource."]
    #[serde(rename = "targetResource", default, skip_serializing_if = "Option::is_none")]
    pub target_resource: Option<TargetResource>,
    #[doc = "HTTP message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<HttpMessage>,
    #[doc = "HTTP message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<HttpMessage>,
}
impl DeploymentOperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod deployment_operation_properties {
    use super::*;
    #[doc = "The name of the current provisioning operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningOperation {
        NotSpecified,
        Create,
        Delete,
        Waiting,
        AzureAsyncOperationWaiting,
        ResourceCacheWaiting,
        Action,
        Read,
        EvaluateDeploymentOutput,
        DeploymentCleanup,
    }
}
#[doc = "List of deployment operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentOperationsListResult {
    #[doc = "An array of deployment operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DeploymentOperation>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeploymentOperationsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeploymentOperationsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentProperties {
    #[doc = "The template content. You use this element when you want to pass the template syntax directly in the request rather than link to an existing template. It can be a JObject or well-formed JSON string. Use either the templateLink property or the template property, but not both."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<serde_json::Value>,
    #[doc = "Entity representing the reference to the template."]
    #[serde(rename = "templateLink", default, skip_serializing_if = "Option::is_none")]
    pub template_link: Option<TemplateLink>,
    #[doc = "Name and value pairs that define the deployment parameters for the template. You use this element when you want to provide the parameter values directly in the request rather than link to an existing parameter file. Use either the parametersLink property or the parameters property, but not both. It can be a JObject or a well formed JSON string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Entity representing the reference to the deployment parameters."]
    #[serde(rename = "parametersLink", default, skip_serializing_if = "Option::is_none")]
    pub parameters_link: Option<ParametersLink>,
    #[doc = "The mode that is used to deploy resources. This value can be either Incremental or Complete. In Incremental mode, resources are deployed without deleting existing resources that are not included in the template. In Complete mode, resources are deployed and existing resources in the resource group that are not included in the template are deleted. Be careful when using Complete mode as you may unintentionally delete resources."]
    pub mode: deployment_properties::Mode,
    #[doc = "The debug setting."]
    #[serde(rename = "debugSetting", default, skip_serializing_if = "Option::is_none")]
    pub debug_setting: Option<DebugSetting>,
    #[doc = "Deployment on error behavior."]
    #[serde(rename = "onErrorDeployment", default, skip_serializing_if = "Option::is_none")]
    pub on_error_deployment: Option<OnErrorDeployment>,
    #[doc = "Specifies whether template expressions are evaluated within the scope of the parent template or nested template."]
    #[serde(rename = "expressionEvaluationOptions", default, skip_serializing_if = "Option::is_none")]
    pub expression_evaluation_options: Option<ExpressionEvaluationOptions>,
}
impl DeploymentProperties {
    pub fn new(mode: deployment_properties::Mode) -> Self {
        Self {
            template: None,
            template_link: None,
            parameters: None,
            parameters_link: None,
            mode,
            debug_setting: None,
            on_error_deployment: None,
            expression_evaluation_options: None,
        }
    }
}
pub mod deployment_properties {
    use super::*;
    #[doc = "The mode that is used to deploy resources. This value can be either Incremental or Complete. In Incremental mode, resources are deployed without deleting existing resources that are not included in the template. In Complete mode, resources are deployed and existing resources in the resource group that are not included in the template are deleted. Be careful when using Complete mode as you may unintentionally delete resources."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        Incremental,
        Complete,
    }
}
#[doc = "Deployment properties with additional details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentPropertiesExtended {
    #[doc = "Denotes the state of provisioning."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<deployment_properties_extended::ProvisioningState>,
    #[doc = "The correlation ID of the deployment."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "The timestamp of the template deployment."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "The duration of the template deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "Key/value pairs that represent deployment output."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<serde_json::Value>,
    #[doc = "The list of resource providers needed for the deployment."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub providers: Vec<Provider>,
    #[doc = "The list of deployment dependencies."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<Dependency>,
    #[doc = "Entity representing the reference to the template."]
    #[serde(rename = "templateLink", default, skip_serializing_if = "Option::is_none")]
    pub template_link: Option<TemplateLink>,
    #[doc = "Deployment parameters. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Entity representing the reference to the deployment parameters."]
    #[serde(rename = "parametersLink", default, skip_serializing_if = "Option::is_none")]
    pub parameters_link: Option<ParametersLink>,
    #[doc = "The deployment mode. Possible values are Incremental and Complete."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<deployment_properties_extended::Mode>,
    #[doc = "The debug setting."]
    #[serde(rename = "debugSetting", default, skip_serializing_if = "Option::is_none")]
    pub debug_setting: Option<DebugSetting>,
    #[doc = "Deployment on error behavior with additional details."]
    #[serde(rename = "onErrorDeployment", default, skip_serializing_if = "Option::is_none")]
    pub on_error_deployment: Option<OnErrorDeploymentExtended>,
    #[doc = "The hash produced for the template."]
    #[serde(rename = "templateHash", default, skip_serializing_if = "Option::is_none")]
    pub template_hash: Option<String>,
    #[doc = "Array of provisioned resources."]
    #[serde(rename = "outputResources", default, skip_serializing_if = "Vec::is_empty")]
    pub output_resources: Vec<ResourceReference>,
    #[doc = "Array of validated resources."]
    #[serde(rename = "validatedResources", default, skip_serializing_if = "Vec::is_empty")]
    pub validated_resources: Vec<ResourceReference>,
    #[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl DeploymentPropertiesExtended {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod deployment_properties_extended {
    use super::*;
    #[doc = "Denotes the state of provisioning."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        NotSpecified,
        Accepted,
        Running,
        Ready,
        Creating,
        Created,
        Deleting,
        Deleted,
        Canceled,
        Failed,
        Succeeded,
        Updating,
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
                Self::NotSpecified => serializer.serialize_unit_variant("ProvisioningState", 0u32, "NotSpecified"),
                Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Accepted"),
                Self::Running => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Running"),
                Self::Ready => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Ready"),
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Creating"),
                Self::Created => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Created"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Deleting"),
                Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Deleted"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 9u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 10u32, "Succeeded"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 11u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The deployment mode. Possible values are Incremental and Complete."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        Incremental,
        Complete,
    }
}
#[doc = "Information from validate template deployment response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentValidateResult {
    #[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
    #[doc = "Deployment properties with additional details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeploymentPropertiesExtended>,
}
impl DeploymentValidateResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment What-if operation parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentWhatIf {
    #[doc = "The location to store the deployment data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Deployment What-if properties."]
    pub properties: DeploymentWhatIfProperties,
}
impl DeploymentWhatIf {
    pub fn new(properties: DeploymentWhatIfProperties) -> Self {
        Self {
            location: None,
            properties,
        }
    }
}
#[doc = "Deployment What-if properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentWhatIfProperties {
    #[serde(flatten)]
    pub deployment_properties: DeploymentProperties,
    #[doc = "Deployment What-If operation settings."]
    #[serde(rename = "whatIfSettings", default, skip_serializing_if = "Option::is_none")]
    pub what_if_settings: Option<DeploymentWhatIfSettings>,
}
impl DeploymentWhatIfProperties {
    pub fn new(deployment_properties: DeploymentProperties) -> Self {
        Self {
            deployment_properties,
            what_if_settings: None,
        }
    }
}
#[doc = "Deployment What-If operation settings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentWhatIfSettings {
    #[doc = "The format of the What-If results"]
    #[serde(rename = "resultFormat", default, skip_serializing_if = "Option::is_none")]
    pub result_format: Option<deployment_what_if_settings::ResultFormat>,
}
impl DeploymentWhatIfSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod deployment_what_if_settings {
    use super::*;
    #[doc = "The format of the What-If results"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResultFormat {
        ResourceIdOnly,
        FullResourcePayloads,
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
#[doc = "Export resource group template request parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExportTemplateRequest {
    #[doc = "The IDs of the resources to filter the export by. To export all resources, supply an array with single entry '*'."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
    #[doc = "The export template options. A CSV-formatted list containing zero or more of the following: 'IncludeParameterDefaultValue', 'IncludeComments', 'SkipResourceNameParameterization', 'SkipAllParameterization'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
}
impl ExportTemplateRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies whether template expressions are evaluated within the scope of the parent template or nested template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExpressionEvaluationOptions {
    #[doc = "The scope to be used for evaluation of parameters, variables and functions in a nested template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<expression_evaluation_options::Scope>,
}
impl ExpressionEvaluationOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod expression_evaluation_options {
    use super::*;
    #[doc = "The scope to be used for evaluation of parameters, variables and functions in a nested template."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Scope")]
    pub enum Scope {
        NotSpecified,
        Outer,
        Inner,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Scope {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Scope {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Scope {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("Scope", 0u32, "NotSpecified"),
                Self::Outer => serializer.serialize_unit_variant("Scope", 1u32, "Outer"),
                Self::Inner => serializer.serialize_unit_variant("Scope", 2u32, "Inner"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Resource extended location."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedLocation {
    #[doc = "The extended location type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<extended_location::Type>,
    #[doc = "The extended location name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ExtendedLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod extended_location {
    use super::*;
    #[doc = "The extended location type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        EdgeZone,
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
                Self::EdgeZone => serializer.serialize_unit_variant("Type", 0u32, "EdgeZone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Resource information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenericResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Plan for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,
    #[doc = "The resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The kind of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "ID of the resource that manages this resource."]
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[doc = "SKU for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
impl GenericResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenericResourceExpanded {
    #[serde(flatten)]
    pub generic_resource: GenericResource,
    #[doc = "The created time of the resource. This is only present if requested via the $expand query parameter."]
    #[serde(rename = "createdTime", with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "The changed time of the resource. This is only present if requested via the $expand query parameter."]
    #[serde(rename = "changedTime", with = "azure_core::date::rfc3339::option")]
    pub changed_time: Option<time::OffsetDateTime>,
    #[doc = "The provisioning state of the resource. This is only present if requested via the $expand query parameter."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl GenericResourceExpanded {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GenericResourceFilter {
    #[doc = "The resource type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The tag name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tagname: Option<String>,
    #[doc = "The tag value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tagvalue: Option<String>,
}
impl GenericResourceFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "HTTP message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HttpMessage {
    #[doc = "HTTP message content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
}
impl HttpMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[doc = "The principal ID of resource identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The identity type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
    #[doc = "The list of user identities associated with the resource. The user identity dictionary key references will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}'."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl Identity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity {
    use super::*;
    #[doc = "The identity type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
        None,
    }
}
#[doc = "Deployment on error behavior."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OnErrorDeployment {
    #[doc = "The deployment on error behavior type. Possible values are LastSuccessful and SpecificDeployment."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<on_error_deployment::Type>,
    #[doc = "The deployment to be used on error case."]
    #[serde(rename = "deploymentName", default, skip_serializing_if = "Option::is_none")]
    pub deployment_name: Option<String>,
}
impl OnErrorDeployment {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod on_error_deployment {
    use super::*;
    #[doc = "The deployment on error behavior type. Possible values are LastSuccessful and SpecificDeployment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        LastSuccessful,
        SpecificDeployment,
    }
}
#[doc = "Deployment on error behavior with additional details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OnErrorDeploymentExtended {
    #[doc = "The state of the provisioning for the on error deployment."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The deployment on error behavior type. Possible values are LastSuccessful and SpecificDeployment."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<on_error_deployment_extended::Type>,
    #[doc = "The deployment to be used on error case."]
    #[serde(rename = "deploymentName", default, skip_serializing_if = "Option::is_none")]
    pub deployment_name: Option<String>,
}
impl OnErrorDeploymentExtended {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod on_error_deployment_extended {
    use super::*;
    #[doc = "The deployment on error behavior type. Possible values are LastSuccessful and SpecificDeployment."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        LastSuccessful,
        SpecificDeployment,
    }
}
#[doc = "Microsoft.Resources operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
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
    #[doc = "The object that represents the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft.Resources"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Profile, endpoint, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list Microsoft.Resources operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Microsoft.Resources operations."]
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
#[doc = "Entity representing the reference to the deployment parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParametersLink {
    #[doc = "The URI of the parameters file."]
    pub uri: String,
    #[doc = "If included, must match the ContentVersion in the template."]
    #[serde(rename = "contentVersion", default, skip_serializing_if = "Option::is_none")]
    pub content_version: Option<String>,
}
impl ParametersLink {
    pub fn new(uri: String) -> Self {
        Self {
            uri,
            content_version: None,
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
#[doc = "Plan for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Plan {
    #[doc = "The plan ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The publisher ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "The offer ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[doc = "The promotion code."]
    #[serde(rename = "promotionCode", default, skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    #[doc = "The plan's version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl Plan {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource provider information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Provider {
    #[doc = "The provider ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The namespace of the resource provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "The registration state of the resource provider."]
    #[serde(rename = "registrationState", default, skip_serializing_if = "Option::is_none")]
    pub registration_state: Option<String>,
    #[doc = "The registration policy of the resource provider."]
    #[serde(rename = "registrationPolicy", default, skip_serializing_if = "Option::is_none")]
    pub registration_policy: Option<String>,
    #[doc = "The collection of provider resource types."]
    #[serde(rename = "resourceTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_types: Vec<ProviderResourceType>,
    #[doc = "The provider authorization consent state."]
    #[serde(rename = "providerAuthorizationConsentState", default, skip_serializing_if = "Option::is_none")]
    pub provider_authorization_consent_state: Option<provider::ProviderAuthorizationConsentState>,
}
impl Provider {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod provider {
    use super::*;
    #[doc = "The provider authorization consent state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProviderAuthorizationConsentState")]
    pub enum ProviderAuthorizationConsentState {
        NotSpecified,
        Required,
        NotRequired,
        Consented,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProviderAuthorizationConsentState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProviderAuthorizationConsentState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProviderAuthorizationConsentState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("ProviderAuthorizationConsentState", 0u32, "NotSpecified"),
                Self::Required => serializer.serialize_unit_variant("ProviderAuthorizationConsentState", 1u32, "Required"),
                Self::NotRequired => serializer.serialize_unit_variant("ProviderAuthorizationConsentState", 2u32, "NotRequired"),
                Self::Consented => serializer.serialize_unit_variant("ProviderAuthorizationConsentState", 3u32, "Consented"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The provider consent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderConsentDefinition {
    #[doc = "A value indicating whether authorization is consented or not."]
    #[serde(rename = "consentToAuthorization", default, skip_serializing_if = "Option::is_none")]
    pub consent_to_authorization: Option<bool>,
}
impl ProviderConsentDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The provider extended location. "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderExtendedLocation {
    #[doc = "The azure location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The extended location type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The extended locations for the azure location."]
    #[serde(rename = "extendedLocations", default, skip_serializing_if = "Vec::is_empty")]
    pub extended_locations: Vec<String>,
}
impl ProviderExtendedLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of resource providers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderListResult {
    #[doc = "An array of resource providers."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Provider>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProviderListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ProviderListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The provider permission"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderPermission {
    #[doc = "The application id."]
    #[serde(rename = "applicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[doc = "Role definition properties."]
    #[serde(rename = "roleDefinition", default, skip_serializing_if = "Option::is_none")]
    pub role_definition: Option<RoleDefinition>,
    #[doc = "Role definition properties."]
    #[serde(rename = "managedByRoleDefinition", default, skip_serializing_if = "Option::is_none")]
    pub managed_by_role_definition: Option<RoleDefinition>,
    #[doc = "The provider authorization consent state."]
    #[serde(rename = "providerAuthorizationConsentState", default, skip_serializing_if = "Option::is_none")]
    pub provider_authorization_consent_state: Option<provider_permission::ProviderAuthorizationConsentState>,
}
impl ProviderPermission {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod provider_permission {
    use super::*;
    #[doc = "The provider authorization consent state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProviderAuthorizationConsentState")]
    pub enum ProviderAuthorizationConsentState {
        NotSpecified,
        Required,
        NotRequired,
        Consented,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProviderAuthorizationConsentState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProviderAuthorizationConsentState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProviderAuthorizationConsentState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotSpecified => serializer.serialize_unit_variant("ProviderAuthorizationConsentState", 0u32, "NotSpecified"),
                Self::Required => serializer.serialize_unit_variant("ProviderAuthorizationConsentState", 1u32, "Required"),
                Self::NotRequired => serializer.serialize_unit_variant("ProviderAuthorizationConsentState", 2u32, "NotRequired"),
                Self::Consented => serializer.serialize_unit_variant("ProviderAuthorizationConsentState", 3u32, "Consented"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of provider permissions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderPermissionListResult {
    #[doc = "An array of provider permissions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProviderPermission>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ProviderPermissionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The provider registration definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderRegistrationRequest {
    #[doc = "The provider consent."]
    #[serde(rename = "thirdPartyProviderConsent", default, skip_serializing_if = "Option::is_none")]
    pub third_party_provider_consent: Option<ProviderConsentDefinition>,
}
impl ProviderRegistrationRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource type managed by the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderResourceType {
    #[doc = "The resource type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The collection of locations where this resource type can be created."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[doc = "The location mappings that are supported by this resource type."]
    #[serde(rename = "locationMappings", default, skip_serializing_if = "Vec::is_empty")]
    pub location_mappings: Vec<ProviderExtendedLocation>,
    #[doc = "The aliases that are supported by this resource type."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<Alias>,
    #[doc = "The API version."]
    #[serde(rename = "apiVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub api_versions: Vec<String>,
    #[doc = "The default API version."]
    #[serde(rename = "defaultApiVersion", default, skip_serializing_if = "Option::is_none")]
    pub default_api_version: Option<String>,
    #[serde(rename = "zoneMappings", default, skip_serializing_if = "Vec::is_empty")]
    pub zone_mappings: Vec<ZoneMapping>,
    #[doc = "The API profiles for the resource provider."]
    #[serde(rename = "apiProfiles", default, skip_serializing_if = "Vec::is_empty")]
    pub api_profiles: Vec<ApiProfile>,
    #[doc = "The additional capabilities offered by this resource type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<String>,
    #[doc = "The properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl ProviderResourceType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of resource types of a resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProviderResourceTypeListResult {
    #[doc = "An array of resource types."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ProviderResourceType>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ProviderResourceTypeListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specified resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource extended location."]
    #[serde(rename = "extendedLocation", default, skip_serializing_if = "Option::is_none")]
    pub extended_location: Option<ExtendedLocation>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource group information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceGroup {
    #[doc = "The ID of the resource group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource group."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The resource group properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ResourceGroupProperties>,
    #[doc = "The location of the resource group. It cannot be changed after the resource group has been created. It must be one of the supported Azure locations."]
    pub location: String,
    #[doc = "The ID of the resource that manages this resource group."]
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[doc = "The tags attached to the resource group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ResourceGroup {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            properties: None,
            location,
            managed_by: None,
            tags: None,
        }
    }
}
#[doc = "Resource group export result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGroupExportResult {
    #[doc = "The template content."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<serde_json::Value>,
    #[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl ResourceGroupExportResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource group filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGroupFilter {
    #[doc = "The tag name."]
    #[serde(rename = "tagName", default, skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
    #[doc = "The tag value."]
    #[serde(rename = "tagValue", default, skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}
impl ResourceGroupFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of resource groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGroupListResult {
    #[doc = "An array of resource groups."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceGroup>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResourceGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource group information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGroupPatchable {
    #[doc = "The name of the resource group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource group properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ResourceGroupProperties>,
    #[doc = "The ID of the resource that manages this resource group."]
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[doc = "The tags attached to the resource group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ResourceGroupPatchable {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource group properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceGroupProperties {
    #[doc = "The provisioning state. "]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ResourceGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of resource groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceListResult {
    #[doc = "An array of resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GenericResourceExpanded>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource provider operation's display properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderOperationDisplayProperties {
    #[doc = "Operation description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Operation provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Operation resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Resource provider operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Operation description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ResourceProviderOperationDisplayProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource Id model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceReference {
    #[doc = "The fully qualified resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ResourceReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters of move resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourcesMoveInfo {
    #[doc = "The IDs of the resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
    #[doc = "The target resource group."]
    #[serde(rename = "targetResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_group: Option<String>,
}
impl ResourcesMoveInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role definition properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleDefinition {
    #[doc = "The role definition ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The role definition name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "If this is a service role."]
    #[serde(rename = "isServiceRole", default, skip_serializing_if = "Option::is_none")]
    pub is_service_role: Option<bool>,
    #[doc = "Role definition permissions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<Permission>,
    #[doc = "Role definition assignable scopes."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
}
impl RoleDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment operation parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopedDeployment {
    #[doc = "The location to store the deployment data."]
    pub location: String,
    #[doc = "Deployment properties."]
    pub properties: DeploymentProperties,
    #[doc = "Deployment tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ScopedDeployment {
    pub fn new(location: String, properties: DeploymentProperties) -> Self {
        Self {
            location,
            properties,
            tags: None,
        }
    }
}
#[doc = "Deployment What-if operation parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScopedDeploymentWhatIf {
    #[doc = "The location to store the deployment data."]
    pub location: String,
    #[doc = "Deployment What-if properties."]
    pub properties: DeploymentWhatIfProperties,
}
impl ScopedDeploymentWhatIf {
    pub fn new(location: String, properties: DeploymentWhatIfProperties) -> Self {
        Self { location, properties }
    }
}
#[doc = "SKU for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Sku {
    #[doc = "The SKU name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The SKU tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "The SKU size."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "The SKU family."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "The SKU model."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[doc = "The SKU capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl Sku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation status message object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StatusMessage {
    #[doc = "Status of the deployment operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl StatusMessage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sub-resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResource {
    #[doc = "Resource ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SubResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tag count."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagCount {
    #[doc = "Type of count."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Value of count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}
impl TagCount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tag details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagDetails {
    #[doc = "The tag name ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The tag name."]
    #[serde(rename = "tagName", default, skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
    #[doc = "Tag count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<TagCount>,
    #[doc = "The list of tag values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<TagValue>,
}
impl TagDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tag information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagValue {
    #[doc = "The tag value ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The tag value."]
    #[serde(rename = "tagValue", default, skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
    #[doc = "Tag count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<TagCount>,
}
impl TagValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A dictionary of name and value pairs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Tags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of subscription tags."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsListResult {
    #[doc = "An array of tags."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TagDetails>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TagsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TagsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Wrapper resource for tags patch API request only."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsPatchResource {
    #[doc = "The operation type for the patch API."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<tags_patch_resource::Operation>,
    #[doc = "A dictionary of name and value pairs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Tags>,
}
impl TagsPatchResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod tags_patch_resource {
    use super::*;
    #[doc = "The operation type for the patch API."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operation")]
    pub enum Operation {
        Replace,
        Merge,
        Delete,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Replace => serializer.serialize_unit_variant("Operation", 0u32, "Replace"),
                Self::Merge => serializer.serialize_unit_variant("Operation", 1u32, "Merge"),
                Self::Delete => serializer.serialize_unit_variant("Operation", 2u32, "Delete"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Wrapper resource for tags API requests and responses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagsResource {
    #[doc = "The ID of the tags wrapper resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the tags wrapper resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the tags wrapper resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "A dictionary of name and value pairs."]
    pub properties: Tags,
}
impl TagsResource {
    pub fn new(properties: Tags) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            properties,
        }
    }
}
#[doc = "Target resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TargetResource {
    #[doc = "The ID of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}
impl TargetResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the request to calculate template hash. It contains a string of minified template and its hash."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TemplateHashResult {
    #[doc = "The minified template string."]
    #[serde(rename = "minifiedTemplate", default, skip_serializing_if = "Option::is_none")]
    pub minified_template: Option<String>,
    #[doc = "The template hash."]
    #[serde(rename = "templateHash", default, skip_serializing_if = "Option::is_none")]
    pub template_hash: Option<String>,
}
impl TemplateHashResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Entity representing the reference to the template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TemplateLink {
    #[doc = "The URI of the template to deploy. Use either the uri or id property, but not both."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "The resource id of a Template Spec. Use either the id or uri property, but not both."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The relativePath property can be used to deploy a linked template at a location relative to the parent. If the parent template was linked with a TemplateSpec, this will reference an artifact in the TemplateSpec.  If the parent was linked with a URI, the child deployment will be a combination of the parent and relativePath URIs"]
    #[serde(rename = "relativePath", default, skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
    #[doc = "If included, must match the ContentVersion in the template."]
    #[serde(rename = "contentVersion", default, skip_serializing_if = "Option::is_none")]
    pub content_version: Option<String>,
    #[doc = "The query string (for example, a SAS token) to be used with the templateLink URI."]
    #[serde(rename = "queryString", default, skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
}
impl TemplateLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about a single resource change predicted by What-If operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WhatIfChange {
    #[doc = "Resource ID"]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "Type of change that will be made to the resource when the deployment is executed."]
    #[serde(rename = "changeType")]
    pub change_type: what_if_change::ChangeType,
    #[doc = "The explanation about why the resource is unsupported by What-If."]
    #[serde(rename = "unsupportedReason", default, skip_serializing_if = "Option::is_none")]
    pub unsupported_reason: Option<String>,
    #[doc = "The snapshot of the resource before the deployment is executed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<serde_json::Value>,
    #[doc = "The predicted snapshot of the resource after the deployment is executed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<serde_json::Value>,
    #[doc = "The predicted changes to resource properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub delta: Vec<WhatIfPropertyChange>,
}
impl WhatIfChange {
    pub fn new(resource_id: String, change_type: what_if_change::ChangeType) -> Self {
        Self {
            resource_id,
            change_type,
            unsupported_reason: None,
            before: None,
            after: None,
            delta: Vec::new(),
        }
    }
}
pub mod what_if_change {
    use super::*;
    #[doc = "Type of change that will be made to the resource when the deployment is executed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ChangeType {
        Create,
        Delete,
        Ignore,
        Deploy,
        NoChange,
        Modify,
        Unsupported,
    }
}
#[doc = "Deployment operation properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WhatIfOperationProperties {
    #[doc = "List of resource changes predicted by What-If operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub changes: Vec<WhatIfChange>,
}
impl WhatIfOperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the What-If operation. Contains a list of predicted changes and a URL link to get to the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WhatIfOperationResult {
    #[doc = "Status of the What-If operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Deployment operation properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WhatIfOperationProperties>,
    #[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl WhatIfOperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The predicted change to the resource property."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WhatIfPropertyChange {
    #[doc = "The path of the property."]
    pub path: String,
    #[doc = "The type of property change."]
    #[serde(rename = "propertyChangeType")]
    pub property_change_type: what_if_property_change::PropertyChangeType,
    #[doc = "The value of the property before the deployment is executed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<serde_json::Value>,
    #[doc = "The value of the property after the deployment is executed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<serde_json::Value>,
    #[doc = "Nested property changes."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<WhatIfPropertyChange>,
}
impl WhatIfPropertyChange {
    pub fn new(path: String, property_change_type: what_if_property_change::PropertyChangeType) -> Self {
        Self {
            path,
            property_change_type,
            before: None,
            after: None,
            children: Vec::new(),
        }
    }
}
pub mod what_if_property_change {
    use super::*;
    #[doc = "The type of property change."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PropertyChangeType {
        Create,
        Delete,
        Modify,
        Array,
        NoEffect,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ZoneMapping {
    #[doc = "The location of the zone mapping."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
}
impl ZoneMapping {
    pub fn new() -> Self {
        Self::default()
    }
}
