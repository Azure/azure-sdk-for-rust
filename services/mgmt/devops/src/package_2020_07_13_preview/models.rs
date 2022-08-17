#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Authorization info used to access a resource (like code repository)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Authorization {
    #[doc = "Type of authorization."]
    #[serde(rename = "authorizationType")]
    pub authorization_type: authorization::AuthorizationType,
    #[doc = "Authorization parameters corresponding to the authorization type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
impl Authorization {
    pub fn new(authorization_type: authorization::AuthorizationType) -> Self {
        Self {
            authorization_type,
            parameters: None,
        }
    }
}
pub mod authorization {
    use super::*;
    #[doc = "Type of authorization."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AuthorizationType")]
    pub enum AuthorizationType {
        #[serde(rename = "personalAccessToken")]
        PersonalAccessToken,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AuthorizationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AuthorizationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AuthorizationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PersonalAccessToken => serializer.serialize_unit_variant("AuthorizationType", 0u32, "personalAccessToken"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Configuration used to bootstrap a Pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BootstrapConfiguration {
    #[doc = "Repository containing the source code for a pipeline."]
    #[serde(rename = "sourceRepository", default, skip_serializing_if = "Option::is_none")]
    pub source_repository: Option<CodeRepository>,
    #[doc = "Template used to bootstrap the pipeline."]
    pub template: PipelineTemplate,
}
impl BootstrapConfiguration {
    pub fn new(template: PipelineTemplate) -> Self {
        Self {
            source_repository: None,
            template,
        }
    }
}
#[doc = "An error response from the Pipelines Resource Provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response from the Pipelines Resource Provider."]
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
#[doc = "An error response from the Pipelines Resource Provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error. For example, the name of the property in error or the method where the error occurred."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "A list of additional details about the error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Repository containing the source code for a pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeRepository {
    #[doc = "Type of code repository."]
    #[serde(rename = "repositoryType")]
    pub repository_type: code_repository::RepositoryType,
    #[doc = "Unique immutable identifier of the code repository."]
    pub id: String,
    #[doc = "Default branch used to configure Continuous Integration (CI) in the pipeline."]
    #[serde(rename = "defaultBranch")]
    pub default_branch: String,
    #[doc = "Authorization info used to access a resource (like code repository)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Authorization>,
    #[doc = "Repository-specific properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl CodeRepository {
    pub fn new(repository_type: code_repository::RepositoryType, id: String, default_branch: String) -> Self {
        Self {
            repository_type,
            id,
            default_branch,
            authorization: None,
            properties: None,
        }
    }
}
pub mod code_repository {
    use super::*;
    #[doc = "Type of code repository."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RepositoryType")]
    pub enum RepositoryType {
        #[serde(rename = "gitHub")]
        GitHub,
        #[serde(rename = "vstsGit")]
        VstsGit,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RepositoryType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RepositoryType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RepositoryType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::GitHub => serializer.serialize_unit_variant("RepositoryType", 0u32, "gitHub"),
                Self::VstsGit => serializer.serialize_unit_variant("RepositoryType", 1u32, "vstsGit"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Representation of a pipeline template input parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputDescriptor {
    #[doc = "Identifier of the input parameter."]
    pub id: String,
    #[doc = "Description of the input parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Data type of the value of the input parameter."]
    #[serde(rename = "type")]
    pub type_: input_descriptor::Type,
    #[doc = "List of possible values for the input parameter."]
    #[serde(rename = "possibleValues", default, skip_serializing_if = "Vec::is_empty")]
    pub possible_values: Vec<InputValue>,
}
impl InputDescriptor {
    pub fn new(id: String, type_: input_descriptor::Type) -> Self {
        Self {
            id,
            description: None,
            type_,
            possible_values: Vec::new(),
        }
    }
}
pub mod input_descriptor {
    use super::*;
    #[doc = "Data type of the value of the input parameter."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        String,
        SecureString,
        Int,
        Bool,
        Authorization,
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
                Self::String => serializer.serialize_unit_variant("Type", 0u32, "String"),
                Self::SecureString => serializer.serialize_unit_variant("Type", 1u32, "SecureString"),
                Self::Int => serializer.serialize_unit_variant("Type", 2u32, "Int"),
                Self::Bool => serializer.serialize_unit_variant("Type", 3u32, "Bool"),
                Self::Authorization => serializer.serialize_unit_variant("Type", 4u32, "Authorization"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Representation of a pipeline template input parameter value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InputValue {
    #[doc = "Value of an input parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Description of the input parameter value."]
    #[serde(rename = "displayValue", default, skip_serializing_if = "Option::is_none")]
    pub display_value: Option<String>,
}
impl InputValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an Operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation applies to data-plane."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<String>,
    #[doc = "Display information of an operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayValue>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Display information of an operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplayValue {
    #[doc = "Friendly name of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Friendly name of the resource type the operation applies to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Friendly description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Friendly name of the resource provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}
impl OperationDisplayValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of a request to list all operations supported by Microsoft.DevOps resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by Microsoft.DevOps resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "The URL to get the next set of operations, if there are any."]
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
#[doc = "Reference to an Azure DevOps Organization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationReference {
    #[doc = "Unique immutable identifier for the Azure DevOps Organization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the Azure DevOps Organization."]
    pub name: String,
}
impl OrganizationReference {
    pub fn new(name: String) -> Self {
        Self { id: None, name }
    }
}
#[doc = "Pipeline used to configure Continuous Integration (CI) & Continuous Delivery (CD) for Azure resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pipeline {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Custom properties of a Pipeline."]
    pub properties: PipelineProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Pipeline {
    pub fn new(properties: PipelineProperties) -> Self {
        Self {
            resource: Resource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "Result of a request to list all Pipelines under a given scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineListResult {
    #[doc = "List of pipelines."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Pipeline>,
    #[doc = "URL to get the next set of Pipelines, if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PipelineListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PipelineListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Custom properties of a Pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineProperties {
    #[doc = "Unique identifier of the Pipeline"]
    #[serde(rename = "pipelineId", default, skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<i64>,
    #[doc = "Specifies which CI/CD provider to use. Valid options are 'azurePipeline', 'githubWorkflow'."]
    #[serde(rename = "pipelineType")]
    pub pipeline_type: pipeline_properties::PipelineType,
    #[doc = "Configuration used to bootstrap a Pipeline."]
    #[serde(rename = "bootstrapConfiguration")]
    pub bootstrap_configuration: BootstrapConfiguration,
}
impl PipelineProperties {
    pub fn new(pipeline_type: pipeline_properties::PipelineType, bootstrap_configuration: BootstrapConfiguration) -> Self {
        Self {
            pipeline_id: None,
            pipeline_type,
            bootstrap_configuration,
        }
    }
}
pub mod pipeline_properties {
    use super::*;
    #[doc = "Specifies which CI/CD provider to use. Valid options are 'azurePipeline', 'githubWorkflow'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PipelineType")]
    pub enum PipelineType {
        #[serde(rename = "githubWorkflow")]
        GithubWorkflow,
        #[serde(rename = "azurePipeline")]
        AzurePipeline,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PipelineType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PipelineType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PipelineType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::GithubWorkflow => serializer.serialize_unit_variant("PipelineType", 0u32, "githubWorkflow"),
                Self::AzurePipeline => serializer.serialize_unit_variant("PipelineType", 1u32, "azurePipeline"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Template used to bootstrap the pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineTemplate {
    #[doc = "Unique identifier of the pipeline template."]
    pub id: String,
    #[doc = "Dictionary of input parameters used in the pipeline template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
impl PipelineTemplate {
    pub fn new(id: String) -> Self {
        Self { id, parameters: None }
    }
}
#[doc = "Definition of a pipeline template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineTemplateDefinition {
    #[doc = "Unique identifier of the pipeline template."]
    pub id: String,
    #[doc = "Description of the pipeline enabled by the template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "List of input parameters required by the template to create a pipeline."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inputs: Vec<InputDescriptor>,
}
impl PipelineTemplateDefinition {
    pub fn new(id: String) -> Self {
        Self {
            id,
            description: None,
            inputs: Vec::new(),
        }
    }
}
#[doc = "Result of a request to list all pipeline template definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineTemplateDefinitionListResult {
    #[doc = "List of pipeline template definitions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PipelineTemplateDefinition>,
    #[doc = "The URL to get the next set of pipeline template definitions, if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PipelineTemplateDefinitionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PipelineTemplateDefinitionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request payload used to update an existing Azure Pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineUpdateParameters {
    #[doc = "Dictionary of key-value pairs to be set as tags on the Azure Pipeline. This will overwrite any existing tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl PipelineUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reference to an Azure DevOps Project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectReference {
    #[doc = "Unique immutable identifier of the Azure DevOps Project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the Azure DevOps Project."]
    pub name: String,
}
impl ProjectReference {
    pub fn new(name: String) -> Self {
        Self { id: None, name }
    }
}
#[doc = "An Azure Resource Manager (ARM) resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource Type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource Tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Resource Location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Custom properties of a Azure Pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzurePipelineProperties {
    #[serde(flatten)]
    pub pipeline_properties: PipelineProperties,
    #[doc = "Reference to an Azure DevOps Organization."]
    pub organization: OrganizationReference,
    #[doc = "Reference to an Azure DevOps Project."]
    pub project: ProjectReference,
}
impl AzurePipelineProperties {
    pub fn new(pipeline_properties: PipelineProperties, organization: OrganizationReference, project: ProjectReference) -> Self {
        Self {
            pipeline_properties,
            organization,
            project,
        }
    }
}
#[doc = "Custom properties of an Github Pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GithubWorkflowProperties {
    #[serde(flatten)]
    pub pipeline_properties: PipelineProperties,
    #[doc = "Repository containing the source code for a pipeline."]
    pub repository: CodeRepository,
}
impl GithubWorkflowProperties {
    pub fn new(pipeline_properties: PipelineProperties, repository: CodeRepository) -> Self {
        Self {
            pipeline_properties,
            repository,
        }
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
