#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Company information of the user to be passed to partners."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CompanyInfo {
    #[doc = "Domain of the company"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[doc = "Business of the company"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business: Option<String>,
    #[doc = "Number of employees in the company"]
    #[serde(rename = "employeesNumber", default, skip_serializing_if = "Option::is_none")]
    pub employees_number: Option<String>,
    #[doc = "State of the company location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Country of the company location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}
impl CompanyInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of deployment in Elastic cloud corresponding to the Elastic monitor resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentInfoResponse {
    #[doc = "Flag specifying if the Elastic deployment status is healthy or not."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ElasticDeploymentStatus>,
    #[doc = "Version of the elasticsearch in Elastic cloud deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "RAM capacity of the elasticsearch in Elastic cloud deployment."]
    #[serde(rename = "memoryCapacity", default, skip_serializing_if = "Option::is_none")]
    pub memory_capacity: Option<String>,
    #[doc = "Disk capacity of the elasticsearch in Elastic cloud deployment."]
    #[serde(rename = "diskCapacity", default, skip_serializing_if = "Option::is_none")]
    pub disk_capacity: Option<String>,
}
impl DeploymentInfoResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the user's elastic deployment associated with the monitor resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticCloudDeployment {
    #[doc = "Elastic deployment name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Elastic deployment Id"]
    #[serde(rename = "deploymentId", default, skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[doc = "Associated Azure subscription Id for the elastic deployment."]
    #[serde(rename = "azureSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub azure_subscription_id: Option<String>,
    #[doc = "Region where Deployment at Elastic side took place."]
    #[serde(rename = "elasticsearchRegion", default, skip_serializing_if = "Option::is_none")]
    pub elasticsearch_region: Option<String>,
    #[doc = "Elasticsearch ingestion endpoint of the Elastic deployment."]
    #[serde(rename = "elasticsearchServiceUrl", default, skip_serializing_if = "Option::is_none")]
    pub elasticsearch_service_url: Option<String>,
    #[doc = "Kibana endpoint of the Elastic deployment."]
    #[serde(rename = "kibanaServiceUrl", default, skip_serializing_if = "Option::is_none")]
    pub kibana_service_url: Option<String>,
    #[doc = "Kibana dashboard sso URL of the Elastic deployment."]
    #[serde(rename = "kibanaSsoUrl", default, skip_serializing_if = "Option::is_none")]
    pub kibana_sso_url: Option<String>,
}
impl ElasticCloudDeployment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the user's elastic account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticCloudUser {
    #[doc = "Email of the Elastic User Account."]
    #[serde(rename = "emailAddress", default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[doc = "User Id of the elastic account of the User."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Elastic cloud default dashboard sso URL of the Elastic user account."]
    #[serde(rename = "elasticCloudSsoDefaultUrl", default, skip_serializing_if = "Option::is_none")]
    pub elastic_cloud_sso_default_url: Option<String>,
}
impl ElasticCloudUser {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Flag specifying if the Elastic deployment status is healthy or not."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ElasticDeploymentStatus")]
pub enum ElasticDeploymentStatus {
    Healthy,
    Unhealthy,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ElasticDeploymentStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ElasticDeploymentStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ElasticDeploymentStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Healthy => serializer.serialize_unit_variant("ElasticDeploymentStatus", 0u32, "Healthy"),
            Self::Unhealthy => serializer.serialize_unit_variant("ElasticDeploymentStatus", 1u32, "Unhealthy"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Monitor resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticMonitorResource {
    #[doc = "ARM id of the monitor resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the monitor resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the monitor resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Microsoft.Elastic SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
    #[doc = "Properties specific to the monitor resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MonitorProperties>,
    #[doc = "Identity properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityProperties>,
    #[doc = "The tags of the monitor resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The location of the monitor resource"]
    pub location: String,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ElasticMonitorResource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            sku: None,
            properties: None,
            identity: None,
            tags: None,
            location,
            system_data: None,
        }
    }
}
#[doc = "Response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticMonitorResourceListResponse {
    #[doc = "Results of a list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ElasticMonitorResource>,
    #[doc = "Link to the next set of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ElasticMonitorResourceListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ElasticMonitorResourceListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Monitor resource update parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticMonitorResourceUpdateParameters {
    #[doc = "elastic monitor resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ElasticMonitorResourceUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Upgrade elastic monitor version"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticMonitorUpgrade {
    #[doc = "Version to which the elastic monitor should be upgraded to"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ElasticMonitorUpgrade {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Elastic Resource Properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ElasticProperties {
    #[doc = "Details of the user's elastic account."]
    #[serde(rename = "elasticCloudUser", default, skip_serializing_if = "Option::is_none")]
    pub elastic_cloud_user: Option<ElasticCloudUser>,
    #[doc = "Details of the user's elastic deployment associated with the monitor resource."]
    #[serde(rename = "elasticCloudDeployment", default, skip_serializing_if = "Option::is_none")]
    pub elastic_cloud_deployment: Option<ElasticCloudDeployment>,
}
impl ElasticProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseBody {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorResponseBody>,
}
impl ErrorResponseBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the response we got from elastic while creating external user"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExternalUserCreationResponse {
    #[doc = "Shows if user is created or updated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<bool>,
}
impl ExternalUserCreationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the request required for creating user on elastic side"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExternalUserInfo {
    #[doc = "Username of the user to be created or updated"]
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[doc = "Full name of the user to be created or updated"]
    #[serde(rename = "fullName", default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[doc = "Password of the user to be created or updated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "Email id of the user to be created or updated"]
    #[serde(rename = "emailId", default, skip_serializing_if = "Option::is_none")]
    pub email_id: Option<String>,
    #[doc = "Roles to be assigned for  created or updated user"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<String>,
}
impl ExternalUserInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The definition of a filtering tag. Filtering tags are used for capturing resources and include/exclude them from being monitored."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FilteringTag {
    #[doc = "The name (also known as the key) of the tag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of the tag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Valid actions for a filtering tag. Exclusion takes priority over inclusion."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<TagAction>,
}
impl FilteringTag {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityProperties {
    #[doc = "The identity ID."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Managed Identity types."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ManagedIdentityTypes>,
}
impl IdentityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LiftrResourceCategories")]
pub enum LiftrResourceCategories {
    Unknown,
    MonitorLogs,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LiftrResourceCategories {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LiftrResourceCategories {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LiftrResourceCategories {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("LiftrResourceCategories", 0u32, "Unknown"),
            Self::MonitorLogs => serializer.serialize_unit_variant("LiftrResourceCategories", 1u32, "MonitorLogs"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Set of rules for sending logs for the Monitor resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogRules {
    #[doc = "Flag specifying if AAD logs should be sent for the Monitor resource."]
    #[serde(rename = "sendAadLogs", default, skip_serializing_if = "Option::is_none")]
    pub send_aad_logs: Option<bool>,
    #[doc = "Flag specifying if subscription logs should be sent for the Monitor resource."]
    #[serde(rename = "sendSubscriptionLogs", default, skip_serializing_if = "Option::is_none")]
    pub send_subscription_logs: Option<bool>,
    #[doc = "Flag specifying if activity logs from Azure resources should be sent for the Monitor resource."]
    #[serde(rename = "sendActivityLogs", default, skip_serializing_if = "Option::is_none")]
    pub send_activity_logs: Option<bool>,
    #[doc = "List of filtering tags to be used for capturing logs. This only takes effect if SendActivityLogs flag is enabled. If empty, all resources will be captured. If only Exclude action is specified, the rules will apply to the list of all available resources. If Include actions are specified, the rules will only include resources with the associated tags."]
    #[serde(rename = "filteringTags", default, skip_serializing_if = "Vec::is_empty")]
    pub filtering_tags: Vec<FilteringTag>,
}
impl LogRules {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed Identity types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedIdentityTypes")]
pub enum ManagedIdentityTypes {
    SystemAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagedIdentityTypes {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagedIdentityTypes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagedIdentityTypes {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::SystemAssigned => serializer.serialize_unit_variant("ManagedIdentityTypes", 0u32, "SystemAssigned"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Properties specific to the monitor resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitorProperties {
    #[doc = "Provisioning state of Elastic resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Flag specifying if the resource monitoring is enabled or disabled."]
    #[serde(rename = "monitoringStatus", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_status: Option<MonitoringStatus>,
    #[doc = "Elastic Resource Properties."]
    #[serde(rename = "elasticProperties", default, skip_serializing_if = "Option::is_none")]
    pub elastic_properties: Option<ElasticProperties>,
    #[doc = "User Information to be passed to partners."]
    #[serde(rename = "userInfo", default, skip_serializing_if = "Option::is_none")]
    pub user_info: Option<UserInfo>,
    #[doc = "Version of elastic of the monitor resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "liftrResourceCategory", default, skip_serializing_if = "Option::is_none")]
    pub liftr_resource_category: Option<LiftrResourceCategories>,
    #[doc = "The priority of the resource."]
    #[serde(rename = "liftrResourcePreference", default, skip_serializing_if = "Option::is_none")]
    pub liftr_resource_preference: Option<i32>,
}
impl MonitorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a resource currently being monitored by the Elastic monitor resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoredResource {
    #[doc = "The ARM id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Flag indicating the status of the resource for sending logs operation to Elastic."]
    #[serde(rename = "sendingLogs", default, skip_serializing_if = "Option::is_none")]
    pub sending_logs: Option<SendingLogs>,
    #[doc = "Reason for why the resource is sending logs (or why it is not sending)."]
    #[serde(rename = "reasonForLogsStatus", default, skip_serializing_if = "Option::is_none")]
    pub reason_for_logs_status: Option<String>,
}
impl MonitoredResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoredResourceListResponse {
    #[doc = "Results of a list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MonitoredResource>,
    #[doc = "Link to the next set of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MonitoredResourceListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MonitoredResourceListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Flag specifying if the resource monitoring is enabled or disabled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MonitoringStatus")]
pub enum MonitoringStatus {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MonitoringStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MonitoringStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MonitoringStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("MonitoringStatus", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("MonitoringStatus", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Capture logs and metrics of Azure resources based on ARM tags."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoringTagRules {
    #[doc = "Name of the rule set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The id of the rule set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the rule set."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Definition of the properties for a TagRules resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MonitoringTagRulesProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl MonitoringTagRules {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoringTagRulesListResponse {
    #[doc = "Results of a list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MonitoringTagRules>,
    #[doc = "Link to the next set of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MonitoringTagRulesListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MonitoringTagRulesListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the properties for a TagRules resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoringTagRulesProperties {
    #[doc = "Provisioning state of Elastic resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Set of rules for sending logs for the Monitor resource."]
    #[serde(rename = "logRules", default, skip_serializing_if = "Option::is_none")]
    pub log_rules: Option<LogRules>,
}
impl MonitoringTagRulesProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The object that represents the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Service provider, i.e., Microsoft.Elastic."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Type on which the operation is performed, e.g., 'monitors'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Operation type, e.g., read, write, delete, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the operation, e.g., 'Write monitors'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of GET request to list the Microsoft.Elastic operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the Microsoft.Elastic provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationResult>,
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
#[doc = "Operation to be performed on the given vm resource id."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OperationName")]
pub enum OperationName {
    Add,
    Delete,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OperationName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OperationName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OperationName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Add => serializer.serialize_unit_variant("OperationName", 0u32, "Add"),
            Self::Delete => serializer.serialize_unit_variant("OperationName", 1u32, "Delete"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A Microsoft.Elastic REST API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResult {
    #[doc = "Operation name, i.e., {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Origin of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl OperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provisioning state of Elastic resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Accepted,
    Creating,
    Updating,
    Deleting,
    Succeeded,
    Failed,
    Canceled,
    Deleted,
    NotSpecified,
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
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Accepted"),
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Canceled"),
            Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Deleted"),
            Self::NotSpecified => serializer.serialize_unit_variant("ProvisioningState", 8u32, "NotSpecified"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "RP default error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderDefaultErrorResponse {
    #[doc = "Error response body."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponseBody>,
}
impl azure_core::Continuable for ResourceProviderDefaultErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ResourceProviderDefaultErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Microsoft.Elastic SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSku {
    #[doc = "Name of the SKU."]
    pub name: String,
}
impl ResourceSku {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[doc = "Flag indicating the status of the resource for sending logs operation to Elastic."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SendingLogs")]
pub enum SendingLogs {
    True,
    False,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SendingLogs {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SendingLogs {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SendingLogs {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::True => serializer.serialize_unit_variant("SendingLogs", 0u32, "True"),
            Self::False => serializer.serialize_unit_variant("SendingLogs", 1u32, "False"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Valid actions for a filtering tag. Exclusion takes priority over inclusion."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TagAction")]
pub enum TagAction {
    Include,
    Exclude,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TagAction {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TagAction {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TagAction {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Include => serializer.serialize_unit_variant("TagAction", 0u32, "Include"),
            Self::Exclude => serializer.serialize_unit_variant("TagAction", 1u32, "Exclude"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Stack Versions that this version can upgrade to"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpgradableVersionsList {
    #[doc = "Current version of the elastic monitor"]
    #[serde(rename = "currentVersion", default, skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    #[doc = "Stack Versions that this version can upgrade to"]
    #[serde(rename = "upgradableVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub upgradable_versions: Vec<String>,
}
impl UpgradableVersionsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User Information to be passed to partners."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserInfo {
    #[doc = "First name of the user"]
    #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "Last name of the user"]
    #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "Company name of the user"]
    #[serde(rename = "companyName", default, skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[doc = "Email of the user used by Elastic for contacting them if needed"]
    #[serde(rename = "emailAddress", default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[doc = "Company information of the user to be passed to partners."]
    #[serde(rename = "companyInfo", default, skip_serializing_if = "Option::is_none")]
    pub company_info: Option<CompanyInfo>,
}
impl UserInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Update VM resource collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmCollectionUpdate {
    #[doc = "ARM id of the VM resource."]
    #[serde(rename = "vmResourceId", default, skip_serializing_if = "Option::is_none")]
    pub vm_resource_id: Option<String>,
    #[doc = "Operation to be performed on the given vm resource id."]
    #[serde(rename = "operationName", default, skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<OperationName>,
}
impl VmCollectionUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmHostListResponse {
    #[doc = "Results of a list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VmResources>,
    #[doc = "Link to the next Vm resource Id, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VmHostListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VmHostListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The vm ingestion details to install an agent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmIngestionDetailsResponse {
    #[doc = "The cloudId of given Elastic monitor resource."]
    #[serde(rename = "cloudId", default, skip_serializing_if = "Option::is_none")]
    pub cloud_id: Option<String>,
    #[doc = "Ingestion details to install agent on given VM."]
    #[serde(rename = "ingestionKey", default, skip_serializing_if = "Option::is_none")]
    pub ingestion_key: Option<String>,
}
impl VmIngestionDetailsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The vm resource properties that is currently being monitored by the Elastic monitor resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmResources {
    #[doc = "The ARM id of the VM resource."]
    #[serde(rename = "vmResourceId", default, skip_serializing_if = "Option::is_none")]
    pub vm_resource_id: Option<String>,
}
impl VmResources {
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
