#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Terms properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatadogAgreementProperties {
    #[doc = "Publisher identifier string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Product identifier string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[doc = "Plan identifier string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[doc = "Link to HTML with Microsoft and Publisher terms."]
    #[serde(rename = "licenseTextLink", default, skip_serializing_if = "Option::is_none")]
    pub license_text_link: Option<String>,
    #[doc = "Link to the privacy policy of the publisher."]
    #[serde(rename = "privacyPolicyLink", default, skip_serializing_if = "Option::is_none")]
    pub privacy_policy_link: Option<String>,
    #[doc = "Date and time in UTC of when the terms were accepted. This is empty if Accepted is false."]
    #[serde(rename = "retrieveDatetime", with = "azure_core::date::rfc3339::option")]
    pub retrieve_datetime: Option<time::OffsetDateTime>,
    #[doc = "Terms signature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[doc = "If any version of the terms have been accepted, otherwise false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted: Option<bool>,
}
impl DatadogAgreementProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatadogAgreementResource {
    #[doc = "ARM id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the agreement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Terms properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatadogAgreementProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl DatadogAgreementResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatadogAgreementResourceListResponse {
    #[doc = "Results of a list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatadogAgreementResource>,
    #[doc = "Link to the next set of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatadogAgreementResourceListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DatadogAgreementResourceListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatadogApiKey {
    #[doc = "The user that created the API key."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The name of the API key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of the API key."]
    pub key: String,
    #[doc = "The time of creation of the API key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
}
impl DatadogApiKey {
    pub fn new(key: String) -> Self {
        Self {
            created_by: None,
            name: None,
            key,
            created: None,
        }
    }
}
#[doc = "Response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatadogApiKeyListResponse {
    #[doc = "Results of a list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatadogApiKey>,
    #[doc = "Link to the next set of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatadogApiKeyListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DatadogApiKeyListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatadogHost {
    #[doc = "The name of the host."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The aliases for the host."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
    #[doc = "The Datadog integrations reporting metrics for the host."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub apps: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<DatadogHostMetadata>,
}
impl DatadogHost {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatadogHostListResponse {
    #[doc = "Results of a list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatadogHost>,
    #[doc = "Link to the next set of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatadogHostListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DatadogHostListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatadogHostMetadata {
    #[doc = "The agent version."]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "installMethod", default, skip_serializing_if = "Option::is_none")]
    pub install_method: Option<DatadogInstallMethod>,
    #[serde(rename = "logsAgent", default, skip_serializing_if = "Option::is_none")]
    pub logs_agent: Option<DatadogLogsAgent>,
}
impl DatadogHostMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatadogInstallMethod {
    #[doc = "The tool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool: Option<String>,
    #[doc = "The tool version."]
    #[serde(rename = "toolVersion", default, skip_serializing_if = "Option::is_none")]
    pub tool_version: Option<String>,
    #[doc = "The installer version."]
    #[serde(rename = "installerVersion", default, skip_serializing_if = "Option::is_none")]
    pub installer_version: Option<String>,
}
impl DatadogInstallMethod {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatadogLogsAgent {
    #[doc = "The transport."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport: Option<String>,
}
impl DatadogLogsAgent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatadogMonitorResource {
    #[doc = "ARM id of the monitor resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the monitor resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the monitor resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
    #[doc = "Properties specific to the monitor resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MonitorProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl DatadogMonitorResource {
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
pub struct DatadogMonitorResourceListResponse {
    #[doc = "Results of a list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatadogMonitorResource>,
    #[doc = "Link to the next set of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatadogMonitorResourceListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DatadogMonitorResourceListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters for a PATCH request to a monitor resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatadogMonitorResourceUpdateParameters {
    #[doc = "The set of properties that can be update in a PATCH request to a monitor resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MonitorUpdateProperties>,
    #[doc = "The new tags of the monitor resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
}
impl DatadogMonitorResourceUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Datadog organization properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatadogOrganizationProperties {
    #[doc = "Name of the Datadog organization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Id of the Datadog organization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The auth code used to linking to an existing datadog organization."]
    #[serde(rename = "linkingAuthCode", default, skip_serializing_if = "Option::is_none")]
    pub linking_auth_code: Option<String>,
    #[doc = "The client_id from an existing in exchange for an auth token to link organization."]
    #[serde(rename = "linkingClientId", default, skip_serializing_if = "Option::is_none")]
    pub linking_client_id: Option<String>,
    #[doc = "The redirect uri for linking."]
    #[serde(rename = "redirectUri", default, skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    #[doc = "Api key associated to the Datadog organization."]
    #[serde(rename = "apiKey", default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[doc = "Application key associated to the Datadog organization."]
    #[serde(rename = "applicationKey", default, skip_serializing_if = "Option::is_none")]
    pub application_key: Option<String>,
    #[doc = "The Id of the Enterprise App used for Single sign on."]
    #[serde(rename = "enterpriseAppId", default, skip_serializing_if = "Option::is_none")]
    pub enterprise_app_id: Option<String>,
}
impl DatadogOrganizationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatadogSetPasswordLink {
    #[serde(rename = "setPasswordLink", default, skip_serializing_if = "Option::is_none")]
    pub set_password_link: Option<String>,
}
impl DatadogSetPasswordLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatadogSingleSignOnProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Various states of the SSO resource"]
    #[serde(rename = "singleSignOnState", default, skip_serializing_if = "Option::is_none")]
    pub single_sign_on_state: Option<SingleSignOnStates>,
    #[doc = "The Id of the Enterprise App used for Single sign-on."]
    #[serde(rename = "enterpriseAppId", default, skip_serializing_if = "Option::is_none")]
    pub enterprise_app_id: Option<String>,
    #[doc = "The login URL specific to this Datadog Organization."]
    #[serde(rename = "singleSignOnUrl", default, skip_serializing_if = "Option::is_none")]
    pub single_sign_on_url: Option<String>,
}
impl DatadogSingleSignOnProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatadogSingleSignOnResource {
    #[doc = "ARM id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatadogSingleSignOnProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl DatadogSingleSignOnResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatadogSingleSignOnResourceListResponse {
    #[doc = "Results of a list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatadogSingleSignOnResource>,
    #[doc = "Link to the next set of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatadogSingleSignOnResourceListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DatadogSingleSignOnResourceListResponse {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IdentityProperties {
    #[doc = "The identity ID."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Identity type"]
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
#[doc = "The definition of a linked resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkedResource {
    #[doc = "The ARM id of the linked resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl LinkedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkedResourceListResponse {
    #[doc = "Results of a list operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LinkedResource>,
    #[doc = "Link to the next set of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LinkedResourceListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl LinkedResourceListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Set of rules for sending logs for the Monitor resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogRules {
    #[doc = "Flag specifying if AAD logs should be sent for the Monitor resource."]
    #[serde(rename = "sendAadLogs", default, skip_serializing_if = "Option::is_none")]
    pub send_aad_logs: Option<bool>,
    #[doc = "Flag specifying if Azure subscription logs should be sent for the Monitor resource."]
    #[serde(rename = "sendSubscriptionLogs", default, skip_serializing_if = "Option::is_none")]
    pub send_subscription_logs: Option<bool>,
    #[doc = "Flag specifying if Azure resource logs should be sent for the Monitor resource."]
    #[serde(rename = "sendResourceLogs", default, skip_serializing_if = "Option::is_none")]
    pub send_resource_logs: Option<bool>,
    #[doc = "List of filtering tags to be used for capturing logs. This only takes effect if SendResourceLogs flag is enabled. If empty, all resources will be captured. If only Exclude action is specified, the rules will apply to the list of all available resources. If Include actions are specified, the rules will only include resources with the associated tags."]
    #[serde(rename = "filteringTags", default, skip_serializing_if = "Vec::is_empty")]
    pub filtering_tags: Vec<FilteringTag>,
}
impl LogRules {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedIdentityTypes")]
pub enum ManagedIdentityTypes {
    SystemAssigned,
    UserAssigned,
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
            Self::UserAssigned => serializer.serialize_unit_variant("ManagedIdentityTypes", 1u32, "UserAssigned"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Flag specifying the Marketplace Subscription Status of the resource. If payment is not made in time, the resource will go in Suspended state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MarketplaceSubscriptionStatus")]
pub enum MarketplaceSubscriptionStatus {
    Provisioning,
    Active,
    Suspended,
    Unsubscribed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MarketplaceSubscriptionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MarketplaceSubscriptionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MarketplaceSubscriptionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Provisioning => serializer.serialize_unit_variant("MarketplaceSubscriptionStatus", 0u32, "Provisioning"),
            Self::Active => serializer.serialize_unit_variant("MarketplaceSubscriptionStatus", 1u32, "Active"),
            Self::Suspended => serializer.serialize_unit_variant("MarketplaceSubscriptionStatus", 2u32, "Suspended"),
            Self::Unsubscribed => serializer.serialize_unit_variant("MarketplaceSubscriptionStatus", 3u32, "Unsubscribed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Set of rules for sending metrics for the Monitor resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricRules {
    #[doc = "List of filtering tags to be used for capturing metrics. If empty, all resources will be captured. If only Exclude action is specified, the rules will apply to the list of all available resources. If Include actions are specified, the rules will only include resources with the associated tags."]
    #[serde(rename = "filteringTags", default, skip_serializing_if = "Vec::is_empty")]
    pub filtering_tags: Vec<FilteringTag>,
}
impl MetricRules {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties specific to the monitor resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitorProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Flag specifying if the resource monitoring is enabled or disabled."]
    #[serde(rename = "monitoringStatus", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_status: Option<MonitoringStatus>,
    #[doc = "Flag specifying the Marketplace Subscription Status of the resource. If payment is not made in time, the resource will go in Suspended state."]
    #[serde(rename = "marketplaceSubscriptionStatus", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_subscription_status: Option<MarketplaceSubscriptionStatus>,
    #[doc = "Datadog organization properties"]
    #[serde(rename = "datadogOrganizationProperties", default, skip_serializing_if = "Option::is_none")]
    pub datadog_organization_properties: Option<DatadogOrganizationProperties>,
    #[doc = "User info"]
    #[serde(rename = "userInfo", default, skip_serializing_if = "Option::is_none")]
    pub user_info: Option<UserInfo>,
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
#[doc = "The set of properties that can be update in a PATCH request to a monitor resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitorUpdateProperties {
    #[doc = "Flag specifying if the resource monitoring is enabled or disabled."]
    #[serde(rename = "monitoringStatus", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_status: Option<MonitoringStatus>,
}
impl MonitorUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a resource currently being monitored by the Datadog monitor resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoredResource {
    #[doc = "The ARM id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Flag indicating if resource is sending metrics to Datadog."]
    #[serde(rename = "sendingMetrics", default, skip_serializing_if = "Option::is_none")]
    pub sending_metrics: Option<bool>,
    #[doc = "Reason for why the resource is sending metrics (or why it is not sending)."]
    #[serde(rename = "reasonForMetricsStatus", default, skip_serializing_if = "Option::is_none")]
    pub reason_for_metrics_status: Option<String>,
    #[doc = "Flag indicating if resource is sending logs to Datadog."]
    #[serde(rename = "sendingLogs", default, skip_serializing_if = "Option::is_none")]
    pub sending_logs: Option<bool>,
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
impl Default for MonitoringStatus {
    fn default() -> Self {
        Self::Enabled
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
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Set of rules for sending logs for the Monitor resource."]
    #[serde(rename = "logRules", default, skip_serializing_if = "Option::is_none")]
    pub log_rules: Option<LogRules>,
    #[doc = "Set of rules for sending metrics for the Monitor resource."]
    #[serde(rename = "metricRules", default, skip_serializing_if = "Option::is_none")]
    pub metric_rules: Option<MetricRules>,
}
impl MonitoringTagRulesProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The object that represents the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Service provider, i.e., Microsoft.Datadog."]
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
#[doc = "Result of GET request to list the Microsoft.Datadog operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the Microsoft.Datadog provider."]
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
#[doc = "A Microsoft.Datadog REST API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResult {
    #[doc = "Operation name, i.e., {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
impl OperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
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
#[doc = "Various states of the SSO resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SingleSignOnStates")]
pub enum SingleSignOnStates {
    Initial,
    Enable,
    Disable,
    Existing,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SingleSignOnStates {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SingleSignOnStates {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SingleSignOnStates {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Initial => serializer.serialize_unit_variant("SingleSignOnStates", 0u32, "Initial"),
            Self::Enable => serializer.serialize_unit_variant("SingleSignOnStates", 1u32, "Enable"),
            Self::Disable => serializer.serialize_unit_variant("SingleSignOnStates", 2u32, "Disable"),
            Self::Existing => serializer.serialize_unit_variant("SingleSignOnStates", 3u32, "Existing"),
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
#[doc = "User info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserInfo {
    #[doc = "Name of the user"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Email of the user used by Datadog for contacting them if needed"]
    #[serde(rename = "emailAddress", default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[doc = "Phone number of the user used by Datadog for contacting them if needed"]
    #[serde(rename = "phoneNumber", default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}
impl UserInfo {
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
