#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Dynatrace Account Information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountInfo {
    #[doc = "Account Id of the account this environment is linked to"]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "Region in which the account is created"]
    #[serde(rename = "regionId", default, skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}
impl AccountInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dynatrace account API Key"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountInfoSecure {
    #[doc = "Account Id of the account this environment is linked to"]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "Credential string."]
    #[serde(rename = "apiKey", default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<SecureString>,
    #[doc = "Region in which the account is created"]
    #[serde(rename = "regionId", default, skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}
impl AccountInfoSecure {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of App Services having Dynatrace OneAgent installed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppServiceInfo {
    #[doc = "App service resource ID"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Version of the Dynatrace agent installed on the App Service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The monitoring mode of OneAgent"]
    #[serde(rename = "monitoringType", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_type: Option<MonitoringType>,
    #[doc = "Update settings of OneAgent."]
    #[serde(rename = "autoUpdateSetting", default, skip_serializing_if = "Option::is_none")]
    pub auto_update_setting: Option<AutoUpdateSetting>,
    #[doc = "The current update status of OneAgent."]
    #[serde(rename = "updateStatus", default, skip_serializing_if = "Option::is_none")]
    pub update_status: Option<UpdateStatus>,
    #[doc = "The availability state of OneAgent."]
    #[serde(rename = "availabilityState", default, skip_serializing_if = "Option::is_none")]
    pub availability_state: Option<AvailabilityState>,
    #[doc = "Tells whether log modules are enabled or not"]
    #[serde(rename = "logModule", default, skip_serializing_if = "Option::is_none")]
    pub log_module: Option<LogModule>,
    #[doc = "The name of the host group"]
    #[serde(rename = "hostGroup", default, skip_serializing_if = "Option::is_none")]
    pub host_group: Option<String>,
    #[doc = "The name of the host"]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
}
impl AppServiceInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of a list App Services Operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppServiceListResponse {
    #[doc = "The items on this page"]
    pub value: Vec<AppServiceInfo>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink")]
    pub next_link: String,
}
impl azure_core::Continuable for AppServiceListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        if self.next_link.is_empty() {
            None
        } else {
            Some(self.next_link.clone())
        }
    }
}
impl AppServiceListResponse {
    pub fn new(value: Vec<AppServiceInfo>, next_link: String) -> Self {
        Self { value, next_link }
    }
}
#[doc = "Update settings of OneAgent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AutoUpdateSetting")]
pub enum AutoUpdateSetting {
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AutoUpdateSetting {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AutoUpdateSetting {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AutoUpdateSetting {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("AutoUpdateSetting", 0u32, "ENABLED"),
            Self::Disabled => serializer.serialize_unit_variant("AutoUpdateSetting", 1u32, "DISABLED"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The availability state of OneAgent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AvailabilityState")]
pub enum AvailabilityState {
    #[serde(rename = "CRASHED")]
    Crashed,
    #[serde(rename = "LOST")]
    Lost,
    #[serde(rename = "MONITORED")]
    Monitored,
    #[serde(rename = "PRE_MONITORED")]
    PreMonitored,
    #[serde(rename = "SHUTDOWN")]
    Shutdown,
    #[serde(rename = "UNEXPECTED_SHUTDOWN")]
    UnexpectedShutdown,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "UNMONITORED")]
    Unmonitored,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AvailabilityState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AvailabilityState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AvailabilityState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Crashed => serializer.serialize_unit_variant("AvailabilityState", 0u32, "CRASHED"),
            Self::Lost => serializer.serialize_unit_variant("AvailabilityState", 1u32, "LOST"),
            Self::Monitored => serializer.serialize_unit_variant("AvailabilityState", 2u32, "MONITORED"),
            Self::PreMonitored => serializer.serialize_unit_variant("AvailabilityState", 3u32, "PRE_MONITORED"),
            Self::Shutdown => serializer.serialize_unit_variant("AvailabilityState", 4u32, "SHUTDOWN"),
            Self::UnexpectedShutdown => serializer.serialize_unit_variant("AvailabilityState", 5u32, "UNEXPECTED_SHUTDOWN"),
            Self::Unknown => serializer.serialize_unit_variant("AvailabilityState", 6u32, "UNKNOWN"),
            Self::Unmonitored => serializer.serialize_unit_variant("AvailabilityState", 7u32, "UNMONITORED"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Properties of the Dynatrace environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DynatraceEnvironmentProperties {
    #[doc = "User id"]
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[doc = "Dynatrace Account Information"]
    #[serde(rename = "accountInfo", default, skip_serializing_if = "Option::is_none")]
    pub account_info: Option<AccountInfo>,
    #[doc = "Dynatrace Environment Information"]
    #[serde(rename = "environmentInfo", default, skip_serializing_if = "Option::is_none")]
    pub environment_info: Option<EnvironmentInfo>,
    #[doc = "The details of a Dynatrace single sign-on."]
    #[serde(rename = "singleSignOnProperties", default, skip_serializing_if = "Option::is_none")]
    pub single_sign_on_properties: Option<DynatraceSingleSignOnProperties>,
}
impl DynatraceEnvironmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of a Dynatrace single sign-on."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DynatraceSingleSignOnProperties {
    #[doc = "Various states of the SSO resource"]
    #[serde(rename = "singleSignOnState", default, skip_serializing_if = "Option::is_none")]
    pub single_sign_on_state: Option<SingleSignOnStates>,
    #[doc = "Version of the Dynatrace agent installed on the VM."]
    #[serde(rename = "enterpriseAppId", default, skip_serializing_if = "Option::is_none")]
    pub enterprise_app_id: Option<String>,
    #[doc = "The login URL specific to this Dynatrace Environment"]
    #[serde(rename = "singleSignOnUrl", default, skip_serializing_if = "Option::is_none")]
    pub single_sign_on_url: Option<String>,
    #[doc = "array of Aad(azure active directory) domains"]
    #[serde(rename = "aadDomains", default, skip_serializing_if = "Vec::is_empty")]
    pub aad_domains: Vec<String>,
    #[doc = "Provisioning state of the monitoring resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl DynatraceSingleSignOnProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Single sign-on configurations for a given monitor resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DynatraceSingleSignOnResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The details of a Dynatrace single sign-on."]
    pub properties: DynatraceSingleSignOnProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl DynatraceSingleSignOnResource {
    pub fn new(properties: DynatraceSingleSignOnProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "The response of a DynatraceSingleSignOnResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DynatraceSingleSignOnResourceListResult {
    #[doc = "The items on this page"]
    pub value: Vec<DynatraceSingleSignOnResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DynatraceSingleSignOnResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DynatraceSingleSignOnResourceListResult {
    pub fn new(value: Vec<DynatraceSingleSignOnResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Dynatrace Environment Information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentInfo {
    #[doc = "Id of the environment created"]
    #[serde(rename = "environmentId", default, skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[doc = "Ingestion key of the environment"]
    #[serde(rename = "ingestionKey", default, skip_serializing_if = "Option::is_none")]
    pub ingestion_key: Option<String>,
    #[doc = "Ingestion endpoint used for sending logs"]
    #[serde(rename = "logsIngestionEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub logs_ingestion_endpoint: Option<String>,
    #[doc = "Landing URL for Dynatrace environment"]
    #[serde(rename = "landingURL", default, skip_serializing_if = "Option::is_none")]
    pub landing_url: Option<String>,
}
impl EnvironmentInfo {
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
#[doc = "The properties of the managed service identities assigned to this resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityProperties {
    #[doc = "The Active Directory tenant id of the principal."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The active directory identifier of this principal."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The kind of managed identity assigned to this resource."]
    #[serde(rename = "type")]
    pub type_: ManagedIdentityType,
    #[doc = "The identities assigned to this resource by the user."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl IdentityProperties {
    pub fn new(type_: ManagedIdentityType) -> Self {
        Self {
            tenant_id: None,
            principal_id: None,
            type_,
            user_assigned_identities: None,
        }
    }
}
#[doc = "Liftr resource category"]
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
#[doc = "Response for getting all the linkable environments"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkableEnvironmentListResponse {
    #[doc = "List of environments for which user is an admin"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LinkableEnvironmentResponse>,
    #[doc = "Link to the next set of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LinkableEnvironmentListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl LinkableEnvironmentListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request for getting all the linkable environments for a user"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkableEnvironmentRequest {
    #[doc = "Tenant Id of the user in which they want to link the environment"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "user principal id of the user"]
    #[serde(rename = "userPrincipal", default, skip_serializing_if = "Option::is_none")]
    pub user_principal: Option<String>,
    #[doc = "Azure region in which we want to link the environment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}
impl LinkableEnvironmentRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for getting all the linkable environments"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkableEnvironmentResponse {
    #[doc = "environment id for which user is an admin"]
    #[serde(rename = "environmentId", default, skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[doc = "Name of the environment"]
    #[serde(rename = "environmentName", default, skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[doc = "Billing plan information."]
    #[serde(rename = "planData", default, skip_serializing_if = "Option::is_none")]
    pub plan_data: Option<PlanData>,
}
impl LinkableEnvironmentResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tells whether log modules are enabled or not"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LogModule")]
pub enum LogModule {
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LogModule {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LogModule {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LogModule {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("LogModule", 0u32, "ENABLED"),
            Self::Disabled => serializer.serialize_unit_variant("LogModule", 1u32, "DISABLED"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Set of rules for sending logs for the Monitor resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogRules {
    #[doc = "Indicates whether AAD logs are being sent."]
    #[serde(rename = "sendAadLogs", default, skip_serializing_if = "Option::is_none")]
    pub send_aad_logs: Option<SendAadLogsStatus>,
    #[doc = "Indicates whether subscription logs are being sent."]
    #[serde(rename = "sendSubscriptionLogs", default, skip_serializing_if = "Option::is_none")]
    pub send_subscription_logs: Option<SendSubscriptionLogsStatus>,
    #[doc = "Indicates whether activity logs are being sent."]
    #[serde(rename = "sendActivityLogs", default, skip_serializing_if = "Option::is_none")]
    pub send_activity_logs: Option<SendActivityLogsStatus>,
    #[doc = "List of filtering tags to be used for capturing logs. This only takes effect if SendActivityLogs flag is enabled. If empty, all resources will be captured.\nIf only Exclude action is specified, the rules will apply to the list of all available resources. If Include actions are specified, the rules will only include resources with the associated tags."]
    #[serde(rename = "filteringTags", default, skip_serializing_if = "Vec::is_empty")]
    pub filtering_tags: Vec<FilteringTag>,
}
impl LogRules {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kind of managed identity assigned to this resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedIdentityType")]
pub enum ManagedIdentityType {
    SystemAssigned,
    UserAssigned,
    SystemAndUserAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagedIdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagedIdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagedIdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::SystemAssigned => serializer.serialize_unit_variant("ManagedIdentityType", 0u32, "SystemAssigned"),
            Self::UserAssigned => serializer.serialize_unit_variant("ManagedIdentityType", 1u32, "UserAssigned"),
            Self::SystemAndUserAssigned => serializer.serialize_unit_variant("ManagedIdentityType", 2u32, "SystemAndUserAssigned"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Flag specifying the Marketplace Subscription Status of the resource. If payment is not made in time, the resource will go in Suspended state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MarketplaceSubscriptionStatus")]
pub enum MarketplaceSubscriptionStatus {
    Active,
    Suspended,
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
            Self::Active => serializer.serialize_unit_variant("MarketplaceSubscriptionStatus", 0u32, "Active"),
            Self::Suspended => serializer.serialize_unit_variant("MarketplaceSubscriptionStatus", 1u32, "Suspended"),
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
    #[doc = "Flag specifying if the resource monitoring is enabled or disabled."]
    #[serde(rename = "monitoringStatus", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_status: Option<MonitoringStatus>,
    #[doc = "Flag specifying the Marketplace Subscription Status of the resource. If payment is not made in time, the resource will go in Suspended state."]
    #[serde(rename = "marketplaceSubscriptionStatus", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_subscription_status: Option<MarketplaceSubscriptionStatus>,
    #[doc = "Properties of the Dynatrace environment."]
    #[serde(rename = "dynatraceEnvironmentProperties", default, skip_serializing_if = "Option::is_none")]
    pub dynatrace_environment_properties: Option<DynatraceEnvironmentProperties>,
    #[doc = "User info."]
    #[serde(rename = "userInfo", default, skip_serializing_if = "Option::is_none")]
    pub user_info: Option<UserInfo>,
    #[doc = "Billing plan information."]
    #[serde(rename = "planData", default, skip_serializing_if = "Option::is_none")]
    pub plan_data: Option<PlanData>,
    #[doc = "Liftr resource category"]
    #[serde(rename = "liftrResourceCategory", default, skip_serializing_if = "Option::is_none")]
    pub liftr_resource_category: Option<LiftrResourceCategories>,
    #[doc = "The priority of the resource."]
    #[serde(rename = "liftrResourcePreference", default, skip_serializing_if = "Option::is_none")]
    pub liftr_resource_preference: Option<i32>,
    #[doc = "Provisioning state of the monitoring resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl MonitorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Dynatrace Monitor Resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties specific to the monitor resource."]
    pub properties: MonitorProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The properties of the managed service identities assigned to this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityProperties>,
}
impl MonitorResource {
    pub fn new(tracked_resource: TrackedResource, properties: MonitorProperties) -> Self {
        Self {
            tracked_resource,
            properties,
            system_data: None,
            identity: None,
        }
    }
}
#[doc = "The response of a MonitorResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitorResourceListResult {
    #[doc = "The items on this page"]
    pub value: Vec<MonitorResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MonitorResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MonitorResourceListResult {
    pub fn new(value: Vec<MonitorResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The updatable properties of the MonitorResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitorResourceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Flag specifying if the resource monitoring is enabled or disabled."]
    #[serde(rename = "monitoringStatus", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_status: Option<MonitoringStatus>,
    #[doc = "Flag specifying the Marketplace Subscription Status of the resource. If payment is not made in time, the resource will go in Suspended state."]
    #[serde(rename = "marketplaceSubscriptionStatus", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_subscription_status: Option<MarketplaceSubscriptionStatus>,
    #[doc = "Properties of the Dynatrace environment."]
    #[serde(rename = "dynatraceEnvironmentProperties", default, skip_serializing_if = "Option::is_none")]
    pub dynatrace_environment_properties: Option<DynatraceEnvironmentProperties>,
    #[doc = "User info."]
    #[serde(rename = "userInfo", default, skip_serializing_if = "Option::is_none")]
    pub user_info: Option<UserInfo>,
    #[doc = "Billing plan information."]
    #[serde(rename = "planData", default, skip_serializing_if = "Option::is_none")]
    pub plan_data: Option<PlanData>,
}
impl MonitorResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of resource being monitored by Dynatrace monitor resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoredResource {
    #[doc = "The ARM id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Indicates whether metrics are being sent."]
    #[serde(rename = "sendingMetrics", default, skip_serializing_if = "Option::is_none")]
    pub sending_metrics: Option<SendingMetricsStatus>,
    #[doc = "Reason for why the resource is sending metrics (or why it is not sending)."]
    #[serde(rename = "reasonForMetricsStatus", default, skip_serializing_if = "Option::is_none")]
    pub reason_for_metrics_status: Option<String>,
    #[doc = "Indicates whether logs are being sent."]
    #[serde(rename = "sendingLogs", default, skip_serializing_if = "Option::is_none")]
    pub sending_logs: Option<SendingLogsStatus>,
    #[doc = "Reason for why the resource is sending logs (or why it is not sending)."]
    #[serde(rename = "reasonForLogsStatus", default, skip_serializing_if = "Option::is_none")]
    pub reason_for_logs_status: Option<String>,
}
impl MonitoredResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of all the resources being monitored by Dynatrace monitor resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitoredResourceListResponse {
    #[doc = "The items on this page"]
    pub value: Vec<MonitoredResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink")]
    pub next_link: String,
}
impl azure_core::Continuable for MonitoredResourceListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        if self.next_link.is_empty() {
            None
        } else {
            Some(self.next_link.clone())
        }
    }
}
impl MonitoredResourceListResponse {
    pub fn new(value: Vec<MonitoredResource>, next_link: String) -> Self {
        Self { value, next_link }
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
#[doc = "Properties for the Tag rules resource of a Monitor account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoringTagRulesProperties {
    #[doc = "Set of rules for sending logs for the Monitor resource."]
    #[serde(rename = "logRules", default, skip_serializing_if = "Option::is_none")]
    pub log_rules: Option<LogRules>,
    #[doc = "Set of rules for sending metrics for the Monitor resource."]
    #[serde(rename = "metricRules", default, skip_serializing_if = "Option::is_none")]
    pub metric_rules: Option<MetricRules>,
    #[doc = "Provisioning state of the monitoring resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl MonitoringTagRulesProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The monitoring mode of OneAgent"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MonitoringType")]
pub enum MonitoringType {
    #[serde(rename = "CLOUD_INFRASTRUCTURE")]
    CloudInfrastructure,
    #[serde(rename = "FULL_STACK")]
    FullStack,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MonitoringType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MonitoringType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MonitoringType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::CloudInfrastructure => serializer.serialize_unit_variant("MonitoringType", 0u32, "CLOUD_INFRASTRUCTURE"),
            Self::FullStack => serializer.serialize_unit_variant("MonitoringType", 1u32, "FULL_STACK"),
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
#[doc = "Billing plan information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanData {
    #[doc = "different usage type like PAYG/COMMITTED. this could be enum"]
    #[serde(rename = "usageType", default, skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,
    #[doc = "different billing cycles like MONTHLY/WEEKLY. this could be enum"]
    #[serde(rename = "billingCycle", default, skip_serializing_if = "Option::is_none")]
    pub billing_cycle: Option<String>,
    #[doc = "plan id as published by Dynatrace"]
    #[serde(rename = "planDetails", default, skip_serializing_if = "Option::is_none")]
    pub plan_details: Option<String>,
    #[doc = "date when plan was applied"]
    #[serde(rename = "effectiveDate", default, with = "azure_core::date::rfc3339::option")]
    pub effective_date: Option<time::OffsetDateTime>,
}
impl PlanData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provisioning state of the monitoring resource"]
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
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request for getting sso details for a user"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SsoDetailsRequest {
    #[doc = "user principal id of the user"]
    #[serde(rename = "userPrincipal", default, skip_serializing_if = "Option::is_none")]
    pub user_principal: Option<String>,
}
impl SsoDetailsRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SSO details from the Dynatrace partner"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SsoDetailsResponse {
    #[doc = "Indicates whether SSO is enabled or not"]
    #[serde(rename = "isSsoEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_sso_enabled: Option<SsoStatus>,
    #[doc = "URL for Azure AD metadata"]
    #[serde(rename = "metadataUrl", default, skip_serializing_if = "Option::is_none")]
    pub metadata_url: Option<String>,
    #[doc = "The login URL specific to this Dynatrace Environment"]
    #[serde(rename = "singleSignOnUrl", default, skip_serializing_if = "Option::is_none")]
    pub single_sign_on_url: Option<String>,
    #[doc = "array of Aad(azure active directory) domains"]
    #[serde(rename = "aadDomains", default, skip_serializing_if = "Vec::is_empty")]
    pub aad_domains: Vec<String>,
    #[doc = "Array of admin user emails."]
    #[serde(rename = "adminUsers", default, skip_serializing_if = "Vec::is_empty")]
    pub admin_users: Vec<String>,
}
impl SsoDetailsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Indicates whether SSO is enabled or not"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SsoStatus")]
pub enum SsoStatus {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SsoStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SsoStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SsoStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("SsoStatus", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("SsoStatus", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type SecureString = String;
#[doc = "Indicates whether AAD logs are being sent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SendAadLogsStatus")]
pub enum SendAadLogsStatus {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SendAadLogsStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SendAadLogsStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SendAadLogsStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("SendAadLogsStatus", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("SendAadLogsStatus", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Indicates whether activity logs are being sent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SendActivityLogsStatus")]
pub enum SendActivityLogsStatus {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SendActivityLogsStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SendActivityLogsStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SendActivityLogsStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("SendActivityLogsStatus", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("SendActivityLogsStatus", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Indicates whether subscription logs are being sent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SendSubscriptionLogsStatus")]
pub enum SendSubscriptionLogsStatus {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SendSubscriptionLogsStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SendSubscriptionLogsStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SendSubscriptionLogsStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("SendSubscriptionLogsStatus", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("SendSubscriptionLogsStatus", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Indicates whether logs are being sent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SendingLogsStatus")]
pub enum SendingLogsStatus {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SendingLogsStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SendingLogsStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SendingLogsStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("SendingLogsStatus", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("SendingLogsStatus", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Indicates whether metrics are being sent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SendingMetricsStatus")]
pub enum SendingMetricsStatus {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SendingMetricsStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SendingMetricsStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SendingMetricsStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("SendingMetricsStatus", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("SendingMetricsStatus", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
#[doc = "Tag rules for a monitor resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties for the Tag rules resource of a Monitor account."]
    pub properties: MonitoringTagRulesProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl TagRule {
    pub fn new(properties: MonitoringTagRulesProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[doc = "The response of a TagRule list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagRuleListResult {
    #[doc = "The items on this page"]
    pub value: Vec<TagRule>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TagRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl TagRuleListResult {
    pub fn new(value: Vec<TagRule>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The updatable properties of the TagRule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagRuleUpdate {
    #[doc = "Set of rules for sending logs for the Monitor resource."]
    #[serde(rename = "logRules", default, skip_serializing_if = "Option::is_none")]
    pub log_rules: Option<LogRules>,
    #[doc = "Set of rules for sending metrics for the Monitor resource."]
    #[serde(rename = "metricRules", default, skip_serializing_if = "Option::is_none")]
    pub metric_rules: Option<MetricRules>,
}
impl TagRuleUpdate {
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
#[doc = "The current update status of OneAgent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UpdateStatus")]
pub enum UpdateStatus {
    #[serde(rename = "INCOMPATIBLE")]
    Incompatible,
    #[serde(rename = "OUTDATED")]
    Outdated,
    #[serde(rename = "SCHEDULED")]
    Scheduled,
    #[serde(rename = "SUPPRESSED")]
    Suppressed,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "UP2DATE")]
    Up2date,
    #[serde(rename = "UPDATE_IN_PROGRESS")]
    UpdateInProgress,
    #[serde(rename = "UPDATE_PENDING")]
    UpdatePending,
    #[serde(rename = "UPDATE_PROBLEM")]
    UpdateProblem,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UpdateStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UpdateStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UpdateStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Incompatible => serializer.serialize_unit_variant("UpdateStatus", 0u32, "INCOMPATIBLE"),
            Self::Outdated => serializer.serialize_unit_variant("UpdateStatus", 1u32, "OUTDATED"),
            Self::Scheduled => serializer.serialize_unit_variant("UpdateStatus", 2u32, "SCHEDULED"),
            Self::Suppressed => serializer.serialize_unit_variant("UpdateStatus", 3u32, "SUPPRESSED"),
            Self::Unknown => serializer.serialize_unit_variant("UpdateStatus", 4u32, "UNKNOWN"),
            Self::Up2date => serializer.serialize_unit_variant("UpdateStatus", 5u32, "UP2DATE"),
            Self::UpdateInProgress => serializer.serialize_unit_variant("UpdateStatus", 6u32, "UPDATE_IN_PROGRESS"),
            Self::UpdatePending => serializer.serialize_unit_variant("UpdateStatus", 7u32, "UPDATE_PENDING"),
            Self::UpdateProblem => serializer.serialize_unit_variant("UpdateStatus", 8u32, "UPDATE_PROBLEM"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A managed identity assigned by the user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAssignedIdentity {
    #[doc = "The active directory client identifier for this principal."]
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[doc = "The active directory identifier for this principal."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
}
impl UserAssignedIdentity {
    pub fn new(client_id: String, principal_id: String) -> Self {
        Self { client_id, principal_id }
    }
}
#[doc = "User info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserInfo {
    #[doc = "First Name of the user"]
    #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "Last Name of the user"]
    #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "Reusable representation of an email address"]
    #[serde(rename = "emailAddress", default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<Email>,
    #[doc = "Phone number of the user used by Dynatrace for contacting them if needed"]
    #[serde(rename = "phoneNumber", default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[doc = "Country of the user"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}
impl UserInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of payload to be passed while installing VM agent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmExtensionPayload {
    #[doc = "Ingestion key of the environment"]
    #[serde(rename = "ingestionKey", default, skip_serializing_if = "Option::is_none")]
    pub ingestion_key: Option<String>,
    #[doc = "Id of the environment created"]
    #[serde(rename = "environmentId", default, skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
}
impl VmExtensionPayload {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of a list VM Host Operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmHostsListResponse {
    #[doc = "The items on this page"]
    pub value: Vec<VmInfo>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink")]
    pub next_link: String,
}
impl azure_core::Continuable for VmHostsListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        if self.next_link.is_empty() {
            None
        } else {
            Some(self.next_link.clone())
        }
    }
}
impl VmHostsListResponse {
    pub fn new(value: Vec<VmInfo>, next_link: String) -> Self {
        Self { value, next_link }
    }
}
#[doc = "Details of VM Resource having Dynatrace OneAgent installed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmInfo {
    #[doc = "Azure VM resource ID"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Version of the Dynatrace agent installed on the VM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The monitoring mode of OneAgent"]
    #[serde(rename = "monitoringType", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_type: Option<MonitoringType>,
    #[doc = "Update settings of OneAgent."]
    #[serde(rename = "autoUpdateSetting", default, skip_serializing_if = "Option::is_none")]
    pub auto_update_setting: Option<AutoUpdateSetting>,
    #[doc = "The current update status of OneAgent."]
    #[serde(rename = "updateStatus", default, skip_serializing_if = "Option::is_none")]
    pub update_status: Option<UpdateStatus>,
    #[doc = "The availability state of OneAgent."]
    #[serde(rename = "availabilityState", default, skip_serializing_if = "Option::is_none")]
    pub availability_state: Option<AvailabilityState>,
    #[doc = "Tells whether log modules are enabled or not"]
    #[serde(rename = "logModule", default, skip_serializing_if = "Option::is_none")]
    pub log_module: Option<LogModule>,
    #[doc = "The name of the host group"]
    #[serde(rename = "hostGroup", default, skip_serializing_if = "Option::is_none")]
    pub host_group: Option<String>,
    #[doc = "The name of the host"]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
}
impl VmInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type Email = String;
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
