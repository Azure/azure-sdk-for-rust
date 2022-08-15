#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "A pipeline activity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Activity {
    #[doc = "Activity name."]
    pub name: String,
    #[doc = "Type of activity."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Activity description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Activity depends on condition."]
    #[serde(rename = "dependsOn", default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<ActivityDependency>,
}
impl Activity {
    pub fn new(name: String, type_: String) -> Self {
        Self {
            name,
            type_,
            description: None,
            depends_on: Vec::new(),
        }
    }
}
#[doc = "Activity dependency information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityDependency {
    #[doc = "Activity name."]
    pub activity: String,
    #[doc = "Match-Condition for the dependency."]
    #[serde(rename = "dependencyConditions")]
    pub dependency_conditions: Vec<String>,
}
impl ActivityDependency {
    pub fn new(activity: String, dependency_conditions: Vec<String>) -> Self {
        Self {
            activity,
            dependency_conditions,
        }
    }
}
#[doc = "Information about an activity run in a pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivityRun {
    #[doc = "The name of the pipeline."]
    #[serde(rename = "pipelineName", default, skip_serializing_if = "Option::is_none")]
    pub pipeline_name: Option<String>,
    #[doc = "The id of the pipeline run."]
    #[serde(rename = "pipelineRunId", default, skip_serializing_if = "Option::is_none")]
    pub pipeline_run_id: Option<String>,
    #[doc = "The name of the activity."]
    #[serde(rename = "activityName", default, skip_serializing_if = "Option::is_none")]
    pub activity_name: Option<String>,
    #[doc = "The type of the activity."]
    #[serde(rename = "activityType", default, skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<String>,
    #[doc = "The id of the activity run."]
    #[serde(rename = "activityRunId", default, skip_serializing_if = "Option::is_none")]
    pub activity_run_id: Option<String>,
    #[doc = "The name of the compute linked service."]
    #[serde(rename = "linkedServiceName", default, skip_serializing_if = "Option::is_none")]
    pub linked_service_name: Option<String>,
    #[doc = "The status of the activity run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The start time of the activity run in 'ISO 8601' format."]
    #[serde(rename = "activityRunStart", with = "azure_core::date::rfc3339::option")]
    pub activity_run_start: Option<time::OffsetDateTime>,
    #[doc = "The end time of the activity run in 'ISO 8601' format."]
    #[serde(rename = "activityRunEnd", with = "azure_core::date::rfc3339::option")]
    pub activity_run_end: Option<time::OffsetDateTime>,
    #[doc = "The duration of the activity run."]
    #[serde(rename = "durationInMs", default, skip_serializing_if = "Option::is_none")]
    pub duration_in_ms: Option<i64>,
    #[doc = "The input for the activity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    #[doc = "The output for the activity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,
    #[doc = "The error if any from the activity run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
}
impl ActivityRun {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list activity runs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityRunsListResponse {
    #[doc = "List of activity runs."]
    pub value: Vec<ActivityRun>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ActivityRunsListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ActivityRunsListResponse {
    pub fn new(value: Vec<ActivityRun>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Azure Key Vault secret reference."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureKeyVaultSecretReference {
    #[serde(flatten)]
    pub secret_base: SecretBase,
    #[doc = "Linked service reference type."]
    pub store: LinkedServiceReference,
    #[doc = "The name of the secret in Azure Key Vault. Type: string (or Expression with resultType string)."]
    #[serde(rename = "secretName")]
    pub secret_name: serde_json::Value,
    #[doc = "The version of the secret in Azure Key Vault. The default value is the latest version of the secret. Type: string (or Expression with resultType string)."]
    #[serde(rename = "secretVersion", default, skip_serializing_if = "Option::is_none")]
    pub secret_version: Option<serde_json::Value>,
}
impl AzureKeyVaultSecretReference {
    pub fn new(secret_base: SecretBase, store: LinkedServiceReference, secret_name: serde_json::Value) -> Self {
        Self {
            secret_base,
            store,
            secret_name,
            secret_version: None,
        }
    }
}
#[doc = "Response body with a run identifier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateRunResponse {
    #[doc = "Identifier of a run."]
    #[serde(rename = "runId")]
    pub run_id: String,
}
impl CreateRunResponse {
    pub fn new(run_id: String) -> Self {
        Self { run_id }
    }
}
#[doc = "The Azure Data Factory nested object which identifies data within different data stores, such as tables, files, folders, and documents."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dataset {
    #[doc = "Type of dataset."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Dataset description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Columns that define the structure of the dataset. Type: array (or Expression with resultType array), itemType: DatasetDataElement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub structure: Option<serde_json::Value>,
    #[doc = "Linked service reference type."]
    #[serde(rename = "linkedServiceName")]
    pub linked_service_name: LinkedServiceReference,
    #[doc = "Definition of all parameters for an entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterDefinitionSpecification>,
    #[doc = "List of tags that can be used for describing the Dataset."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<serde_json::Value>,
}
impl Dataset {
    pub fn new(type_: String, linked_service_name: LinkedServiceReference) -> Self {
        Self {
            type_,
            description: None,
            structure: None,
            linked_service_name,
            parameters: None,
            annotations: Vec::new(),
        }
    }
}
#[doc = "A list of dataset resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatasetListResponse {
    #[doc = "List of datasets."]
    pub value: Vec<DatasetResource>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatasetListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DatasetListResponse {
    pub fn new(value: Vec<DatasetResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Dataset reference type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatasetReference {
    #[doc = "Dataset reference type."]
    #[serde(rename = "type")]
    pub type_: dataset_reference::Type,
    #[doc = "Reference dataset name."]
    #[serde(rename = "referenceName")]
    pub reference_name: String,
    #[doc = "An object mapping parameter names to argument values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterValueSpecification>,
}
impl DatasetReference {
    pub fn new(type_: dataset_reference::Type, reference_name: String) -> Self {
        Self {
            type_,
            reference_name,
            parameters: None,
        }
    }
}
pub mod dataset_reference {
    use super::*;
    #[doc = "Dataset reference type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        DatasetReference,
    }
}
#[doc = "Dataset resource type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatasetResource {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The Azure Data Factory nested object which identifies data within different data stores, such as tables, files, folders, and documents."]
    pub properties: Dataset,
}
impl DatasetResource {
    pub fn new(properties: Dataset) -> Self {
        Self {
            sub_resource: SubResource::default(),
            properties,
        }
    }
}
#[doc = "The object that defines the structure of an Azure Data Factory response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[doc = "Error code."]
    pub code: String,
    #[doc = "Error message."]
    pub message: String,
    #[doc = "Property name/path in request associated with error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Array with additional error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorResponse>,
}
impl azure_core::Continuable for ErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponse {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            details: Vec::new(),
        }
    }
}
#[doc = "Azure Data Factory expression definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Expression {
    #[doc = "Expression type."]
    #[serde(rename = "type")]
    pub type_: expression::Type,
    #[doc = "Expression value."]
    pub value: String,
}
impl Expression {
    pub fn new(type_: expression::Type, value: String) -> Self {
        Self { type_, value }
    }
}
pub mod expression {
    use super::*;
    #[doc = "Expression type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Expression,
    }
}
#[doc = "Factory resource type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Factory {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Identity properties of the factory resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<FactoryIdentity>,
    #[doc = "Factory resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FactoryProperties>,
}
impl Factory {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity properties of the factory resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FactoryIdentity {
    #[doc = "The identity type. Currently the only supported type is 'SystemAssigned'."]
    #[serde(rename = "type")]
    pub type_: factory_identity::Type,
    #[doc = "The principal id of the identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client tenant id of the identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl FactoryIdentity {
    pub fn new(type_: factory_identity::Type) -> Self {
        Self {
            type_,
            principal_id: None,
            tenant_id: None,
        }
    }
}
pub mod factory_identity {
    use super::*;
    #[doc = "The identity type. Currently the only supported type is 'SystemAssigned'."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
    }
}
#[doc = "A list of factory resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FactoryListResponse {
    #[doc = "List of factories."]
    pub value: Vec<Factory>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FactoryListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl FactoryListResponse {
    pub fn new(value: Vec<Factory>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Factory resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FactoryProperties {
    #[doc = "Factory provisioning state, example Succeeded."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Time the factory was created in ISO8601 format."]
    #[serde(rename = "createTime", with = "azure_core::date::rfc3339::option")]
    pub create_time: Option<time::OffsetDateTime>,
    #[doc = "Version of the factory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Factory's VSTS repo information."]
    #[serde(rename = "vstsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub vsts_configuration: Option<FactoryVstsConfiguration>,
}
impl FactoryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Factory's VSTS repo information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FactoryRepoUpdate {
    #[doc = "The factory resource id."]
    #[serde(rename = "factoryResourceId", default, skip_serializing_if = "Option::is_none")]
    pub factory_resource_id: Option<String>,
    #[doc = "The resource group name."]
    #[serde(rename = "resourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[doc = "Factory's VSTS repo information."]
    #[serde(rename = "vstsConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub vsts_configuration: Option<FactoryVstsConfiguration>,
}
impl FactoryRepoUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for updating a factory resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FactoryUpdateParameters {
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Identity properties of the factory resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<FactoryIdentity>,
}
impl FactoryUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Factory's VSTS repo information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FactoryVstsConfiguration {
    #[doc = "VSTS account name."]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "VSTS project name."]
    #[serde(rename = "projectName", default, skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[doc = "VSTS repository name."]
    #[serde(rename = "repositoryName", default, skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    #[doc = "VSTS collaboration branch."]
    #[serde(rename = "collaborationBranch", default, skip_serializing_if = "Option::is_none")]
    pub collaboration_branch: Option<String>,
    #[doc = "VSTS root folder."]
    #[serde(rename = "rootFolder", default, skip_serializing_if = "Option::is_none")]
    pub root_folder: Option<String>,
    #[doc = "VSTS last commit id."]
    #[serde(rename = "lastCommitId", default, skip_serializing_if = "Option::is_none")]
    pub last_commit_id: Option<String>,
    #[doc = "VSTS tenant id."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl FactoryVstsConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Data Factory nested object which serves as a compute resource for activities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntime {
    #[doc = "The type of integration runtime."]
    #[serde(rename = "type")]
    pub type_: IntegrationRuntimeType,
    #[doc = "Integration runtime description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl IntegrationRuntime {
    pub fn new(type_: IntegrationRuntimeType) -> Self {
        Self { type_, description: None }
    }
}
#[doc = "The integration runtime authentication keys."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeAuthKeys {
    #[doc = "The primary integration runtime authentication key."]
    #[serde(rename = "authKey1", default, skip_serializing_if = "Option::is_none")]
    pub auth_key1: Option<String>,
    #[doc = "The secondary integration runtime authentication key."]
    #[serde(rename = "authKey2", default, skip_serializing_if = "Option::is_none")]
    pub auth_key2: Option<String>,
}
impl IntegrationRuntimeAuthKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The state of integration runtime auto update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IntegrationRuntimeAutoUpdate")]
pub enum IntegrationRuntimeAutoUpdate {
    On,
    Off,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IntegrationRuntimeAutoUpdate {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IntegrationRuntimeAutoUpdate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IntegrationRuntimeAutoUpdate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::On => serializer.serialize_unit_variant("IntegrationRuntimeAutoUpdate", 0u32, "On"),
            Self::Off => serializer.serialize_unit_variant("IntegrationRuntimeAutoUpdate", 1u32, "Off"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Connection information for encrypting the on-premises data source credentials."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeConnectionInfo {
    #[doc = "The token generated in service. Callers use this token to authenticate to integration runtime."]
    #[serde(rename = "serviceToken", default, skip_serializing_if = "Option::is_none")]
    pub service_token: Option<String>,
    #[doc = "The integration runtime SSL certificate thumbprint. Click-Once application uses it to do server validation."]
    #[serde(rename = "identityCertThumbprint", default, skip_serializing_if = "Option::is_none")]
    pub identity_cert_thumbprint: Option<String>,
    #[doc = "The on-premises integration runtime host URL."]
    #[serde(rename = "hostServiceUri", default, skip_serializing_if = "Option::is_none")]
    pub host_service_uri: Option<String>,
    #[doc = "The integration runtime version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The public key for encrypting a credential when transferring the credential to the integration runtime."]
    #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[doc = "Whether the identity certificate is expired."]
    #[serde(rename = "isIdentityCertExprired", default, skip_serializing_if = "Option::is_none")]
    pub is_identity_cert_exprired: Option<bool>,
}
impl IntegrationRuntimeConnectionInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of integration runtime resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntimeListResponse {
    #[doc = "List of integration runtimes."]
    pub value: Vec<IntegrationRuntimeResource>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for IntegrationRuntimeListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl IntegrationRuntimeListResponse {
    pub fn new(value: Vec<IntegrationRuntimeResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Get monitoring data response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeMonitoringData {
    #[doc = "Integration runtime name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Integration runtime node monitoring data."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<IntegrationRuntimeNodeMonitoringData>,
}
impl IntegrationRuntimeMonitoringData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The IP address of self-hosted integration runtime node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeNodeIpAddress {
    #[doc = "The IP address of self-hosted integration runtime node."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}
impl IntegrationRuntimeNodeIpAddress {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Monitoring data for integration runtime node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeNodeMonitoringData {
    #[doc = "Name of the integration runtime node."]
    #[serde(rename = "nodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[doc = "Available memory (MB) on the integration runtime node."]
    #[serde(rename = "availableMemoryInMB", default, skip_serializing_if = "Option::is_none")]
    pub available_memory_in_mb: Option<i64>,
    #[doc = "CPU percentage on the integration runtime node."]
    #[serde(rename = "cpuUtilization", default, skip_serializing_if = "Option::is_none")]
    pub cpu_utilization: Option<f64>,
    #[doc = "Maximum concurrent jobs on the integration runtime node."]
    #[serde(rename = "concurrentJobsLimit", default, skip_serializing_if = "Option::is_none")]
    pub concurrent_jobs_limit: Option<i64>,
    #[doc = "The number of jobs currently running on the integration runtime node."]
    #[serde(rename = "concurrentJobsRunning", default, skip_serializing_if = "Option::is_none")]
    pub concurrent_jobs_running: Option<i64>,
    #[doc = "The maximum concurrent jobs in this integration runtime."]
    #[serde(rename = "maxConcurrentJobs", default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent_jobs: Option<i64>,
    #[doc = "Sent bytes on the integration runtime node."]
    #[serde(rename = "sentBytes", default, skip_serializing_if = "Option::is_none")]
    pub sent_bytes: Option<f64>,
    #[doc = "Received bytes on the integration runtime node."]
    #[serde(rename = "receivedBytes", default, skip_serializing_if = "Option::is_none")]
    pub received_bytes: Option<f64>,
}
impl IntegrationRuntimeNodeMonitoringData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Integration runtime reference type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntimeReference {
    #[doc = "Type of integration runtime."]
    #[serde(rename = "type")]
    pub type_: integration_runtime_reference::Type,
    #[doc = "Reference integration runtime name."]
    #[serde(rename = "referenceName")]
    pub reference_name: String,
    #[doc = "An object mapping parameter names to argument values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterValueSpecification>,
}
impl IntegrationRuntimeReference {
    pub fn new(type_: integration_runtime_reference::Type, reference_name: String) -> Self {
        Self {
            type_,
            reference_name,
            parameters: None,
        }
    }
}
pub mod integration_runtime_reference {
    use super::*;
    #[doc = "Type of integration runtime."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        IntegrationRuntimeReference,
    }
}
#[doc = "Parameters to regenerate the authentication key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeRegenerateKeyParameters {
    #[doc = "The name of the authentication key to regenerate."]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<integration_runtime_regenerate_key_parameters::KeyName>,
}
impl IntegrationRuntimeRegenerateKeyParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod integration_runtime_regenerate_key_parameters {
    use super::*;
    #[doc = "The name of the authentication key to regenerate."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "KeyName")]
    pub enum KeyName {
        #[serde(rename = "authKey1")]
        AuthKey1,
        #[serde(rename = "authKey2")]
        AuthKey2,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for KeyName {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for KeyName {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for KeyName {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AuthKey1 => serializer.serialize_unit_variant("KeyName", 0u32, "authKey1"),
                Self::AuthKey2 => serializer.serialize_unit_variant("KeyName", 1u32, "authKey2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Request to remove a node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeRemoveNodeRequest {
    #[doc = "The name of the node to be removed."]
    #[serde(rename = "nodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
}
impl IntegrationRuntimeRemoveNodeRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Integration runtime resource type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntimeResource {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Azure Data Factory nested object which serves as a compute resource for activities."]
    pub properties: IntegrationRuntime,
}
impl IntegrationRuntimeResource {
    pub fn new(properties: IntegrationRuntime) -> Self {
        Self {
            sub_resource: SubResource::default(),
            properties,
        }
    }
}
#[doc = "The state of integration runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IntegrationRuntimeState")]
pub enum IntegrationRuntimeState {
    Initial,
    Stopped,
    Started,
    Starting,
    Stopping,
    NeedRegistration,
    Online,
    Limited,
    Offline,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IntegrationRuntimeState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IntegrationRuntimeState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IntegrationRuntimeState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Initial => serializer.serialize_unit_variant("IntegrationRuntimeState", 0u32, "Initial"),
            Self::Stopped => serializer.serialize_unit_variant("IntegrationRuntimeState", 1u32, "Stopped"),
            Self::Started => serializer.serialize_unit_variant("IntegrationRuntimeState", 2u32, "Started"),
            Self::Starting => serializer.serialize_unit_variant("IntegrationRuntimeState", 3u32, "Starting"),
            Self::Stopping => serializer.serialize_unit_variant("IntegrationRuntimeState", 4u32, "Stopping"),
            Self::NeedRegistration => serializer.serialize_unit_variant("IntegrationRuntimeState", 5u32, "NeedRegistration"),
            Self::Online => serializer.serialize_unit_variant("IntegrationRuntimeState", 6u32, "Online"),
            Self::Limited => serializer.serialize_unit_variant("IntegrationRuntimeState", 7u32, "Limited"),
            Self::Offline => serializer.serialize_unit_variant("IntegrationRuntimeState", 8u32, "Offline"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Integration runtime status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntegrationRuntimeStatus {
    #[doc = "The type of integration runtime."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<IntegrationRuntimeType>,
    #[doc = "The data factory name which the integration runtime belong to."]
    #[serde(rename = "dataFactoryName", default, skip_serializing_if = "Option::is_none")]
    pub data_factory_name: Option<String>,
    #[doc = "The state of integration runtime."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<IntegrationRuntimeState>,
}
impl IntegrationRuntimeStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of integration runtime status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntimeStatusListResponse {
    #[doc = "List of integration runtime status."]
    pub value: Vec<IntegrationRuntimeStatusResponse>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl IntegrationRuntimeStatusListResponse {
    pub fn new(value: Vec<IntegrationRuntimeStatusResponse>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Integration runtime status response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegrationRuntimeStatusResponse {
    #[doc = "The integration runtime name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Integration runtime status."]
    pub properties: IntegrationRuntimeStatus,
}
impl IntegrationRuntimeStatusResponse {
    pub fn new(properties: IntegrationRuntimeStatus) -> Self {
        Self { name: None, properties }
    }
}
#[doc = "The type of integration runtime."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IntegrationRuntimeType")]
pub enum IntegrationRuntimeType {
    Managed,
    SelfHosted,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IntegrationRuntimeType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IntegrationRuntimeType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IntegrationRuntimeType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Managed => serializer.serialize_unit_variant("IntegrationRuntimeType", 0u32, "Managed"),
            Self::SelfHosted => serializer.serialize_unit_variant("IntegrationRuntimeType", 1u32, "SelfHosted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The Azure Data Factory nested object which contains the information and credential which can be used to connect with related store or compute resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedService {
    #[doc = "Type of linked service."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Integration runtime reference type."]
    #[serde(rename = "connectVia", default, skip_serializing_if = "Option::is_none")]
    pub connect_via: Option<IntegrationRuntimeReference>,
    #[doc = "Linked service description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Definition of all parameters for an entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterDefinitionSpecification>,
    #[doc = "List of tags that can be used for describing the Dataset."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<serde_json::Value>,
}
impl LinkedService {
    pub fn new(type_: String) -> Self {
        Self {
            type_,
            connect_via: None,
            description: None,
            parameters: None,
            annotations: Vec::new(),
        }
    }
}
#[doc = "A list of linked service resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedServiceListResponse {
    #[doc = "List of linked services."]
    pub value: Vec<LinkedServiceResource>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LinkedServiceListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl LinkedServiceListResponse {
    pub fn new(value: Vec<LinkedServiceResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Linked service reference type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedServiceReference {
    #[doc = "Linked service reference type."]
    #[serde(rename = "type")]
    pub type_: linked_service_reference::Type,
    #[doc = "Reference LinkedService name."]
    #[serde(rename = "referenceName")]
    pub reference_name: String,
    #[doc = "An object mapping parameter names to argument values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterValueSpecification>,
}
impl LinkedServiceReference {
    pub fn new(type_: linked_service_reference::Type, reference_name: String) -> Self {
        Self {
            type_,
            reference_name,
            parameters: None,
        }
    }
}
pub mod linked_service_reference {
    use super::*;
    #[doc = "Linked service reference type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        LinkedServiceReference,
    }
}
#[doc = "Linked service resource type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedServiceResource {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "The Azure Data Factory nested object which contains the information and credential which can be used to connect with related store or compute resource."]
    pub properties: LinkedService,
}
impl LinkedServiceResource {
    pub fn new(properties: LinkedService) -> Self {
        Self {
            sub_resource: SubResource::default(),
            properties,
        }
    }
}
#[doc = "Azure Data Factory API operation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The intended executor of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Metadata associated with the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "Additional details about an operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Metadata associated with the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The description of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[doc = "The name of the provider."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The name of the resource type on which the operation is performed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The type of operation: get, read, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "A list of operations that can be performed by the Data Factory service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResponse {
    #[doc = "List of Data Factory operations supported by the Data Factory resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details about an operation related to logs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationLogSpecification {
    #[doc = "The name of the log category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Blobs created in the customer storage account, per hour."]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl OperationLogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines how often data for a metric becomes available."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetricAvailability {
    #[doc = "The granularity for the metric."]
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[doc = "Blob created in the customer storage account, per hour."]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl OperationMetricAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details about an operation related to metrics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationMetricSpecification {
    #[doc = "The name of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized display name of the metric."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The description of the metric."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "The unit that the metric is measured in."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The type of metric aggregation."]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[doc = "Whether or not the service is using regional MDM accounts."]
    #[serde(rename = "enableRegionalMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub enable_regional_mdm_account: Option<String>,
    #[doc = "The name of the MDM account."]
    #[serde(rename = "sourceMdmAccount", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_account: Option<String>,
    #[doc = "The name of the MDM namespace."]
    #[serde(rename = "sourceMdmNamespace", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_namespace: Option<String>,
    #[doc = "Defines how often data for metrics becomes available."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub availabilities: Vec<OperationMetricAvailability>,
}
impl OperationMetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Additional details about an operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {
    #[doc = "Details about a service operation."]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<OperationServiceSpecification>,
}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details about a service operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationServiceSpecification {
    #[doc = "Details about operations related to logs."]
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<OperationLogSpecification>,
    #[doc = "Details about operations related to metrics."]
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<OperationMetricSpecification>,
}
impl OperationServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of all parameters for an entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParameterDefinitionSpecification {}
impl ParameterDefinitionSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of a single parameter for an entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParameterSpecification {
    #[doc = "Parameter type."]
    #[serde(rename = "type")]
    pub type_: parameter_specification::Type,
    #[doc = "Default value of parameter."]
    #[serde(rename = "defaultValue", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,
}
impl ParameterSpecification {
    pub fn new(type_: parameter_specification::Type) -> Self {
        Self {
            type_,
            default_value: None,
        }
    }
}
pub mod parameter_specification {
    use super::*;
    #[doc = "Parameter type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Object,
        String,
        Int,
        Float,
        Bool,
        Array,
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
                Self::Object => serializer.serialize_unit_variant("Type", 0u32, "Object"),
                Self::String => serializer.serialize_unit_variant("Type", 1u32, "String"),
                Self::Int => serializer.serialize_unit_variant("Type", 2u32, "Int"),
                Self::Float => serializer.serialize_unit_variant("Type", 3u32, "Float"),
                Self::Bool => serializer.serialize_unit_variant("Type", 4u32, "Bool"),
                Self::Array => serializer.serialize_unit_variant("Type", 5u32, "Array"),
                Self::SecureString => serializer.serialize_unit_variant("Type", 6u32, "SecureString"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An object mapping parameter names to argument values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParameterValueSpecification {}
impl ParameterValueSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A data factory pipeline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Pipeline {
    #[doc = "The description of the pipeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "List of activities in pipeline."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub activities: Vec<Activity>,
    #[doc = "Definition of all parameters for an entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterDefinitionSpecification>,
    #[doc = "The max number of concurrent runs for the pipeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<i64>,
    #[doc = "List of tags that can be used for describing the Pipeline."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<serde_json::Value>,
}
impl Pipeline {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of pipeline resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineListResponse {
    #[doc = "List of pipelines."]
    pub value: Vec<PipelineResource>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PipelineListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PipelineListResponse {
    pub fn new(value: Vec<PipelineResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Pipeline reference type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineReference {
    #[doc = "Pipeline reference type."]
    #[serde(rename = "type")]
    pub type_: pipeline_reference::Type,
    #[doc = "Reference pipeline name."]
    #[serde(rename = "referenceName")]
    pub reference_name: String,
    #[doc = "Reference name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl PipelineReference {
    pub fn new(type_: pipeline_reference::Type, reference_name: String) -> Self {
        Self {
            type_,
            reference_name,
            name: None,
        }
    }
}
pub mod pipeline_reference {
    use super::*;
    #[doc = "Pipeline reference type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        PipelineReference,
    }
}
#[doc = "Pipeline resource type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineResource {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "A data factory pipeline."]
    pub properties: Pipeline,
}
impl PipelineResource {
    pub fn new(properties: Pipeline) -> Self {
        Self {
            sub_resource: SubResource::default(),
            properties,
        }
    }
}
#[doc = "Information about a pipeline run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineRun {
    #[doc = "Identifier of a run."]
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[doc = "The pipeline name."]
    #[serde(rename = "pipelineName", default, skip_serializing_if = "Option::is_none")]
    pub pipeline_name: Option<String>,
    #[doc = "The full or partial list of parameter name, value pair used in the pipeline run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "Provides entity name and id that started the pipeline run."]
    #[serde(rename = "invokedBy", default, skip_serializing_if = "Option::is_none")]
    pub invoked_by: Option<PipelineRunInvokedBy>,
    #[doc = "The last updated timestamp for the pipeline run event in ISO8601 format."]
    #[serde(rename = "lastUpdated", with = "azure_core::date::rfc3339::option")]
    pub last_updated: Option<time::OffsetDateTime>,
    #[doc = "The start time of a pipeline run in ISO8601 format."]
    #[serde(rename = "runStart", with = "azure_core::date::rfc3339::option")]
    pub run_start: Option<time::OffsetDateTime>,
    #[doc = "The end time of a pipeline run in ISO8601 format."]
    #[serde(rename = "runEnd", with = "azure_core::date::rfc3339::option")]
    pub run_end: Option<time::OffsetDateTime>,
    #[doc = "The duration of a pipeline run."]
    #[serde(rename = "durationInMs", default, skip_serializing_if = "Option::is_none")]
    pub duration_in_ms: Option<i64>,
    #[doc = "The status of a pipeline run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The message from a pipeline run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl PipelineRun {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Query parameters for listing pipeline runs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineRunFilterParameters {
    #[doc = "The continuation token for getting the next page of results. Null for first page."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[doc = "The time at or after which the pipeline run event was updated in 'ISO 8601' format."]
    #[serde(rename = "lastUpdatedAfter", with = "azure_core::date::rfc3339")]
    pub last_updated_after: time::OffsetDateTime,
    #[doc = "The time at or before which the pipeline run event was updated in 'ISO 8601' format."]
    #[serde(rename = "lastUpdatedBefore", with = "azure_core::date::rfc3339")]
    pub last_updated_before: time::OffsetDateTime,
    #[doc = "List of filters."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<PipelineRunQueryFilter>,
    #[doc = "List of OrderBy option."]
    #[serde(rename = "orderBy", default, skip_serializing_if = "Vec::is_empty")]
    pub order_by: Vec<PipelineRunQueryOrderBy>,
}
impl PipelineRunFilterParameters {
    pub fn new(last_updated_after: time::OffsetDateTime, last_updated_before: time::OffsetDateTime) -> Self {
        Self {
            continuation_token: None,
            last_updated_after,
            last_updated_before,
            filters: Vec::new(),
            order_by: Vec::new(),
        }
    }
}
#[doc = "Provides entity name and id that started the pipeline run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PipelineRunInvokedBy {
    #[doc = "Name of the entity that started the pipeline run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The ID of the entity that started the run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PipelineRunInvokedBy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Query filter option for listing pipeline runs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineRunQueryFilter {
    #[doc = "Parameter name to be used for filter."]
    pub operand: pipeline_run_query_filter::Operand,
    #[doc = "Operator to be used for filter."]
    pub operator: pipeline_run_query_filter::Operator,
    #[doc = "List of filter values."]
    pub values: Vec<String>,
}
impl PipelineRunQueryFilter {
    pub fn new(operand: pipeline_run_query_filter::Operand, operator: pipeline_run_query_filter::Operator, values: Vec<String>) -> Self {
        Self { operand, operator, values }
    }
}
pub mod pipeline_run_query_filter {
    use super::*;
    #[doc = "Parameter name to be used for filter."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operand")]
    pub enum Operand {
        PipelineName,
        Status,
        RunStart,
        RunEnd,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operand {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operand {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operand {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PipelineName => serializer.serialize_unit_variant("Operand", 0u32, "PipelineName"),
                Self::Status => serializer.serialize_unit_variant("Operand", 1u32, "Status"),
                Self::RunStart => serializer.serialize_unit_variant("Operand", 2u32, "RunStart"),
                Self::RunEnd => serializer.serialize_unit_variant("Operand", 3u32, "RunEnd"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Operator to be used for filter."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        Equals,
        NotEquals,
        In,
        NotIn,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Equals => serializer.serialize_unit_variant("Operator", 0u32, "Equals"),
                Self::NotEquals => serializer.serialize_unit_variant("Operator", 1u32, "NotEquals"),
                Self::In => serializer.serialize_unit_variant("Operator", 2u32, "In"),
                Self::NotIn => serializer.serialize_unit_variant("Operator", 3u32, "NotIn"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An object to provide order by options for listing pipeline runs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineRunQueryOrderBy {
    #[doc = "Parameter name to be used for order by."]
    #[serde(rename = "orderBy")]
    pub order_by: pipeline_run_query_order_by::OrderBy,
    #[doc = "Sorting order of the parameter."]
    pub order: pipeline_run_query_order_by::Order,
}
impl PipelineRunQueryOrderBy {
    pub fn new(order_by: pipeline_run_query_order_by::OrderBy, order: pipeline_run_query_order_by::Order) -> Self {
        Self { order_by, order }
    }
}
pub mod pipeline_run_query_order_by {
    use super::*;
    #[doc = "Parameter name to be used for order by."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OrderBy")]
    pub enum OrderBy {
        RunStart,
        RunEnd,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OrderBy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OrderBy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OrderBy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::RunStart => serializer.serialize_unit_variant("OrderBy", 0u32, "RunStart"),
                Self::RunEnd => serializer.serialize_unit_variant("OrderBy", 1u32, "RunEnd"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Sorting order of the parameter."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Order")]
    pub enum Order {
        #[serde(rename = "ASC")]
        Asc,
        #[serde(rename = "DESC")]
        Desc,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Order {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Order {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Order {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Asc => serializer.serialize_unit_variant("Order", 0u32, "ASC"),
                Self::Desc => serializer.serialize_unit_variant("Order", 1u32, "DESC"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list pipeline runs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineRunQueryResponse {
    #[doc = "List of pipeline runs."]
    pub value: Vec<PipelineRun>,
    #[doc = "The continuation token for getting the next page of results, if any remaining results exist, null otherwise."]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
impl PipelineRunQueryResponse {
    pub fn new(value: Vec<PipelineRun>) -> Self {
        Self {
            value,
            continuation_token: None,
        }
    }
}
#[doc = "Azure Data Factory top-level resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "The resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The base definition of a secret type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretBase {
    #[doc = "Type of the secret."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl SecretBase {
    pub fn new(type_: String) -> Self {
        Self { type_ }
    }
}
#[doc = "Azure Data Factory secure string definition. The string value will be masked with asterisks '*' during Get or List API calls."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecureString {
    #[serde(flatten)]
    pub secret_base: SecretBase,
    #[doc = "Value of secure string."]
    pub value: String,
}
impl SecureString {
    pub fn new(secret_base: SecretBase, value: String) -> Self {
        Self { secret_base, value }
    }
}
#[doc = "Properties of Self-hosted integration runtime node."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SelfHostedIntegrationRuntimeNode {
    #[doc = "Name of the integration runtime node."]
    #[serde(rename = "nodeName", default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[doc = "Machine name of the integration runtime node."]
    #[serde(rename = "machineName", default, skip_serializing_if = "Option::is_none")]
    pub machine_name: Option<String>,
    #[doc = "URI for the host machine of the integration runtime."]
    #[serde(rename = "hostServiceUri", default, skip_serializing_if = "Option::is_none")]
    pub host_service_uri: Option<String>,
    #[doc = "Status of the integration runtime node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<self_hosted_integration_runtime_node::Status>,
    #[doc = "The integration runtime capabilities dictionary"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<serde_json::Value>,
    #[doc = "Status of the integration runtime node version."]
    #[serde(rename = "versionStatus", default, skip_serializing_if = "Option::is_none")]
    pub version_status: Option<String>,
    #[doc = "Version of the integration runtime node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The time at which the integration runtime node was registered in ISO8601 format."]
    #[serde(rename = "registerTime", with = "azure_core::date::rfc3339::option")]
    pub register_time: Option<time::OffsetDateTime>,
    #[doc = "The most recent time at which the integration runtime was connected in ISO8601 format."]
    #[serde(rename = "lastConnectTime", with = "azure_core::date::rfc3339::option")]
    pub last_connect_time: Option<time::OffsetDateTime>,
    #[doc = "The time at which the integration runtime will expire in ISO8601 format."]
    #[serde(rename = "expiryTime", with = "azure_core::date::rfc3339::option")]
    pub expiry_time: Option<time::OffsetDateTime>,
    #[doc = "The time the node last started up."]
    #[serde(rename = "lastStartTime", with = "azure_core::date::rfc3339::option")]
    pub last_start_time: Option<time::OffsetDateTime>,
    #[doc = "The integration runtime node last stop time."]
    #[serde(rename = "lastStopTime", with = "azure_core::date::rfc3339::option")]
    pub last_stop_time: Option<time::OffsetDateTime>,
    #[doc = "The result of the last integration runtime node update."]
    #[serde(rename = "lastUpdateResult", default, skip_serializing_if = "Option::is_none")]
    pub last_update_result: Option<self_hosted_integration_runtime_node::LastUpdateResult>,
    #[doc = "The last time for the integration runtime node update start."]
    #[serde(rename = "lastStartUpdateTime", with = "azure_core::date::rfc3339::option")]
    pub last_start_update_time: Option<time::OffsetDateTime>,
    #[doc = "The last time for the integration runtime node update end."]
    #[serde(rename = "lastEndUpdateTime", with = "azure_core::date::rfc3339::option")]
    pub last_end_update_time: Option<time::OffsetDateTime>,
    #[doc = "Indicates whether this node is the active dispatcher for integration runtime requests."]
    #[serde(rename = "isActiveDispatcher", default, skip_serializing_if = "Option::is_none")]
    pub is_active_dispatcher: Option<bool>,
    #[doc = "Maximum concurrent jobs on the integration runtime node."]
    #[serde(rename = "concurrentJobsLimit", default, skip_serializing_if = "Option::is_none")]
    pub concurrent_jobs_limit: Option<i64>,
    #[doc = "The maximum concurrent jobs in this integration runtime."]
    #[serde(rename = "maxConcurrentJobs", default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent_jobs: Option<i64>,
}
impl SelfHostedIntegrationRuntimeNode {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod self_hosted_integration_runtime_node {
    use super::*;
    #[doc = "Status of the integration runtime node."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        NeedRegistration,
        Online,
        Limited,
        Offline,
        Upgrading,
        Initializing,
        InitializeFailed,
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
                Self::NeedRegistration => serializer.serialize_unit_variant("Status", 0u32, "NeedRegistration"),
                Self::Online => serializer.serialize_unit_variant("Status", 1u32, "Online"),
                Self::Limited => serializer.serialize_unit_variant("Status", 2u32, "Limited"),
                Self::Offline => serializer.serialize_unit_variant("Status", 3u32, "Offline"),
                Self::Upgrading => serializer.serialize_unit_variant("Status", 4u32, "Upgrading"),
                Self::Initializing => serializer.serialize_unit_variant("Status", 5u32, "Initializing"),
                Self::InitializeFailed => serializer.serialize_unit_variant("Status", 6u32, "InitializeFailed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The result of the last integration runtime node update."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LastUpdateResult")]
    pub enum LastUpdateResult {
        Succeed,
        Fail,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LastUpdateResult {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LastUpdateResult {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LastUpdateResult {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeed => serializer.serialize_unit_variant("LastUpdateResult", 0u32, "Succeed"),
                Self::Fail => serializer.serialize_unit_variant("LastUpdateResult", 1u32, "Fail"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Azure Data Factory nested resource, which belongs to a factory."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResource {
    #[doc = "The resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Etag identifies change in the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl SubResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure data factory nested object which contains information about creating pipeline run"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Trigger {
    #[doc = "Trigger type."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Trigger description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Enumerates possible state of Triggers."]
    #[serde(rename = "runtimeState", default, skip_serializing_if = "Option::is_none")]
    pub runtime_state: Option<TriggerRuntimeState>,
}
impl Trigger {
    pub fn new(type_: String) -> Self {
        Self {
            type_,
            description: None,
            runtime_state: None,
        }
    }
}
#[doc = "A list of trigger resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerListResponse {
    #[doc = "List of triggers."]
    pub value: Vec<TriggerResource>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TriggerListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TriggerListResponse {
    pub fn new(value: Vec<TriggerResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Pipeline that needs to be triggered with the given parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TriggerPipelineReference {
    #[doc = "Pipeline reference type."]
    #[serde(rename = "pipelineReference", default, skip_serializing_if = "Option::is_none")]
    pub pipeline_reference: Option<PipelineReference>,
    #[doc = "An object mapping parameter names to argument values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterValueSpecification>,
}
impl TriggerPipelineReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Trigger resource type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerResource {
    #[serde(flatten)]
    pub sub_resource: SubResource,
    #[doc = "Azure data factory nested object which contains information about creating pipeline run"]
    pub properties: Trigger,
}
impl TriggerResource {
    pub fn new(properties: Trigger) -> Self {
        Self {
            sub_resource: SubResource::default(),
            properties,
        }
    }
}
#[doc = "Trigger runs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TriggerRun {
    #[doc = "Trigger run id."]
    #[serde(rename = "triggerRunId", default, skip_serializing_if = "Option::is_none")]
    pub trigger_run_id: Option<String>,
    #[doc = "Trigger name."]
    #[serde(rename = "triggerName", default, skip_serializing_if = "Option::is_none")]
    pub trigger_name: Option<String>,
    #[doc = "Trigger type."]
    #[serde(rename = "triggerType", default, skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<String>,
    #[doc = "Trigger run start time."]
    #[serde(rename = "triggerRunTimestamp", with = "azure_core::date::rfc3339::option")]
    pub trigger_run_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Trigger run status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<trigger_run::Status>,
    #[doc = "Trigger error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "List of property name and value related to trigger run. Name, value pair depends on type of trigger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "List of pipeline name and run Id triggered by the trigger run."]
    #[serde(rename = "triggeredPipelines", default, skip_serializing_if = "Option::is_none")]
    pub triggered_pipelines: Option<serde_json::Value>,
}
impl TriggerRun {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod trigger_run {
    use super::*;
    #[doc = "Trigger run status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Succeeded,
        Failed,
        Inprogress,
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
                Self::Succeeded => serializer.serialize_unit_variant("Status", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 1u32, "Failed"),
                Self::Inprogress => serializer.serialize_unit_variant("Status", 2u32, "Inprogress"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of trigger runs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriggerRunListResponse {
    #[doc = "List of trigger runs."]
    pub value: Vec<TriggerRun>,
    #[doc = "The link to the next page of results, if any remaining results exist."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TriggerRunListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TriggerRunListResponse {
    pub fn new(value: Vec<TriggerRun>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Enumerates possible state of Triggers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TriggerRuntimeState")]
pub enum TriggerRuntimeState {
    Started,
    Stopped,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TriggerRuntimeState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TriggerRuntimeState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TriggerRuntimeState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Started => serializer.serialize_unit_variant("TriggerRuntimeState", 0u32, "Started"),
            Self::Stopped => serializer.serialize_unit_variant("TriggerRuntimeState", 1u32, "Stopped"),
            Self::Disabled => serializer.serialize_unit_variant("TriggerRuntimeState", 2u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Update integration runtime node request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateIntegrationRuntimeNodeRequest {
    #[doc = "The number of concurrent jobs permitted to run on the integration runtime node. Values between 1 and maxConcurrentJobs(inclusive) are allowed."]
    #[serde(rename = "concurrentJobsLimit", default, skip_serializing_if = "Option::is_none")]
    pub concurrent_jobs_limit: Option<i64>,
}
impl UpdateIntegrationRuntimeNodeRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Update integration runtime request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateIntegrationRuntimeRequest {
    #[doc = "The state of integration runtime auto update."]
    #[serde(rename = "autoUpdate", default, skip_serializing_if = "Option::is_none")]
    pub auto_update: Option<IntegrationRuntimeAutoUpdate>,
    #[doc = "The time offset (in hours) in the day, e.g., PT03H is 3 hours. The integration runtime auto update will happen on that time."]
    #[serde(rename = "updateDelayOffset", default, skip_serializing_if = "Option::is_none")]
    pub update_delay_offset: Option<String>,
}
impl UpdateIntegrationRuntimeRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
