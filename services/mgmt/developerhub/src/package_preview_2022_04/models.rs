#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Information on the azure container registry"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Acr {
    #[doc = "ACR subscription id"]
    #[serde(rename = "acrSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub acr_subscription_id: Option<String>,
    #[doc = "ACR resource group"]
    #[serde(rename = "acrResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub acr_resource_group: Option<String>,
    #[doc = "ACR registry"]
    #[serde(rename = "acrRegistryName", default, skip_serializing_if = "Option::is_none")]
    pub acr_registry_name: Option<String>,
    #[doc = "ACR repository"]
    #[serde(rename = "acrRepositoryName", default, skip_serializing_if = "Option::is_none")]
    pub acr_repository_name: Option<String>,
}
impl Acr {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Determines the authorization status of requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AuthorizationStatus")]
pub enum AuthorizationStatus {
    Authorized,
    NotFound,
    Error,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AuthorizationStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AuthorizationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AuthorizationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Authorized => serializer.serialize_unit_variant("AuthorizationStatus", 0u32, "Authorized"),
            Self::NotFound => serializer.serialize_unit_variant("AuthorizationStatus", 1u32, "NotFound"),
            Self::Error => serializer.serialize_unit_variant("AuthorizationStatus", 2u32, "Error"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "delete response if content must be provided on delete operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeleteWorkflowResponse {
    #[doc = "delete status message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl DeleteWorkflowResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentProperties {
    #[doc = "Determines the type of manifests within the repository."]
    #[serde(rename = "manifestType", default, skip_serializing_if = "Option::is_none")]
    pub manifest_type: Option<ManifestType>,
    #[serde(rename = "kubeManifestLocations", default, skip_serializing_if = "Vec::is_empty")]
    pub kube_manifest_locations: Vec<String>,
    #[doc = "Helm chart directory path in repository."]
    #[serde(rename = "helmChartPath", default, skip_serializing_if = "Option::is_none")]
    pub helm_chart_path: Option<String>,
    #[doc = "Helm Values.yaml file location in repository."]
    #[serde(rename = "helmValues", default, skip_serializing_if = "Option::is_none")]
    pub helm_values: Option<String>,
    #[doc = "Manifest override values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overrides: Option<serde_json::Value>,
}
impl DeploymentProperties {
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
#[doc = "GitHubOAuth request object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubOAuthCallRequest {
    #[doc = "The URL the client will redirect to on successful authentication. If empty, no redirect will occur."]
    #[serde(rename = "redirectUrl", default, skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}
impl GitHubOAuthCallRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "URL used to authorize the Developer Hub GitHub App"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubOAuthInfoResponse {
    #[doc = "URL for authorizing the Developer Hub GitHub App"]
    #[serde(rename = "authURL", default, skip_serializing_if = "Option::is_none")]
    pub auth_url: Option<String>,
    #[doc = "OAuth token used to make calls to GitHub"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
impl GitHubOAuthInfoResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response from List GitHubOAuth operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubOAuthListResponse {
    #[doc = "Singleton list response containing one GitHubOAuthResponse response"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GitHubOAuthResponse>,
}
impl GitHubOAuthListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response from List GitHubOAuth operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubOAuthProperties {
    #[doc = "user making request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
impl GitHubOAuthProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Singleton response of GitHubOAuth containing "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubOAuthResponse {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The response from List GitHubOAuth operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GitHubOAuthProperties>,
}
impl GitHubOAuthResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "GitHub Workflow Profile"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitHubWorkflowProfile {
    #[doc = "Repository Owner"]
    #[serde(rename = "repositoryOwner", default, skip_serializing_if = "Option::is_none")]
    pub repository_owner: Option<String>,
    #[doc = "Repository Name"]
    #[serde(rename = "repositoryName", default, skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    #[doc = "Repository Branch Name"]
    #[serde(rename = "branchName", default, skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    #[doc = "Path to the Dockerfile within the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dockerfile: Option<String>,
    #[doc = "Path to Dockerfile Build Context within the repository."]
    #[serde(rename = "dockerBuildContext", default, skip_serializing_if = "Option::is_none")]
    pub docker_build_context: Option<String>,
    #[serde(rename = "deploymentProperties", default, skip_serializing_if = "Option::is_none")]
    pub deployment_properties: Option<DeploymentProperties>,
    #[doc = "Kubernetes namespace the application is deployed to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "Information on the azure container registry"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acr: Option<Acr>,
    #[doc = "The fields needed for OIDC with GitHub."]
    #[serde(rename = "oidcCredentials", default, skip_serializing_if = "Option::is_none")]
    pub oidc_credentials: Option<git_hub_workflow_profile::OidcCredentials>,
    #[doc = "The Azure Kubernetes Cluster Resource the application will be deployed to."]
    #[serde(rename = "aksResourceId", default, skip_serializing_if = "Option::is_none")]
    pub aks_resource_id: Option<String>,
    #[doc = "The URL to the Pull Request submitted against the users repository."]
    #[serde(rename = "prURL", default, skip_serializing_if = "Option::is_none")]
    pub pr_url: Option<String>,
    #[doc = "The number associated with the submitted pull request."]
    #[serde(rename = "pullNumber", default, skip_serializing_if = "Option::is_none")]
    pub pull_number: Option<i32>,
    #[doc = "The status of the Pull Request submitted against the users repository."]
    #[serde(rename = "prStatus", default, skip_serializing_if = "Option::is_none")]
    pub pr_status: Option<PullRequestStatus>,
    #[serde(rename = "lastWorkflowRun", default, skip_serializing_if = "Option::is_none")]
    pub last_workflow_run: Option<WorkflowRun>,
    #[doc = "Determines the authorization status of requests."]
    #[serde(rename = "authStatus", default, skip_serializing_if = "Option::is_none")]
    pub auth_status: Option<AuthorizationStatus>,
}
impl GitHubWorkflowProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod git_hub_workflow_profile {
    use super::*;
    #[doc = "The fields needed for OIDC with GitHub."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct OidcCredentials {
        #[doc = "Azure Application Client ID"]
        #[serde(rename = "azureClientId", default, skip_serializing_if = "Option::is_none")]
        pub azure_client_id: Option<String>,
        #[doc = "Azure Directory (tenant) ID"]
        #[serde(rename = "azureTenantId", default, skip_serializing_if = "Option::is_none")]
        pub azure_tenant_id: Option<String>,
    }
    impl OidcCredentials {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Determines the type of manifests within the repository."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManifestType")]
pub enum ManifestType {
    #[serde(rename = "helm")]
    Helm,
    #[serde(rename = "kube")]
    Kube,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManifestType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManifestType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManifestType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Helm => serializer.serialize_unit_variant("ManifestType", 0u32, "helm"),
            Self::Kube => serializer.serialize_unit_variant("ManifestType", 1u32, "kube"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "The status of the Pull Request submitted against the users repository."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PullRequestStatus")]
pub enum PullRequestStatus {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "submitted")]
    Submitted,
    #[serde(rename = "merged")]
    Merged,
    #[serde(rename = "removed")]
    Removed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PullRequestStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PullRequestStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PullRequestStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("PullRequestStatus", 0u32, "unknown"),
            Self::Submitted => serializer.serialize_unit_variant("PullRequestStatus", 1u32, "submitted"),
            Self::Merged => serializer.serialize_unit_variant("PullRequestStatus", 2u32, "merged"),
            Self::Removed => serializer.serialize_unit_variant("PullRequestStatus", 3u32, "removed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
#[doc = "Resource tags."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TagsObject {
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
#[doc = "Resource representation of a workflow"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Workflow properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkflowProperties>,
}
impl Workflow {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "The response from List Workflows operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowListResult {
    #[doc = "The list of workflows."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workflow>,
    #[doc = "The URL to the next set of workflow results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkflowListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkflowListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workflow properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowProperties {
    #[doc = "GitHub Workflow Profile"]
    #[serde(rename = "githubWorkflowProfile", default, skip_serializing_if = "Option::is_none")]
    pub github_workflow_profile: Option<GitHubWorkflowProfile>,
}
impl WorkflowProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowRun {
    #[doc = "Describes if the workflow run succeeded."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<bool>,
    #[doc = "URL to the run of the workflow."]
    #[serde(rename = "workflowRunURL", default, skip_serializing_if = "Option::is_none")]
    pub workflow_run_url: Option<String>,
    #[doc = "The timestamp of the last workflow run."]
    #[serde(rename = "lastRunAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_run_at: Option<time::OffsetDateTime>,
}
impl WorkflowRun {
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
