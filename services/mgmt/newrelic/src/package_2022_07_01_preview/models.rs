#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Source of Account creation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AccountCreationSource")]
pub enum AccountCreationSource {
    #[serde(rename = "LIFTR")]
    Liftr,
    #[serde(rename = "NEWRELIC")]
    Newrelic,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AccountCreationSource {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AccountCreationSource {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AccountCreationSource {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Liftr => serializer.serialize_unit_variant("AccountCreationSource", 0u32, "LIFTR"),
            Self::Newrelic => serializer.serialize_unit_variant("AccountCreationSource", 1u32, "NEWRELIC"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Account Id parameter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountIdParameter {}
impl AccountIdParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Account Info of the NewRelic account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountInfo {
    #[doc = "Account id"]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "Credential string."]
    #[serde(rename = "ingestionKey", default, skip_serializing_if = "Option::is_none")]
    pub ingestion_key: Option<SecureString>,
    #[doc = "Region where New Relic account is present"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}
impl AccountInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of all the New relic accounts for the given user"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountProperties {
    #[doc = "organization id"]
    #[serde(rename = "organizationId", default, skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    #[doc = "account id"]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "account name"]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "Region where New Relic account is present"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}
impl AccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of a account resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "List of all the New relic accounts for the given user"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccountProperties>,
}
impl AccountResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of get all accounts Operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountsListResponse {
    #[doc = "The AccountResource items on this page"]
    pub value: Vec<AccountResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AccountsListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AccountsListResponse {
    pub fn new(value: Vec<AccountResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Details of VM Resource having NewRelic OneAgent installed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppServiceInfo {
    #[doc = "Azure App service resource ID"]
    #[serde(rename = "azureResourceId", default, skip_serializing_if = "Option::is_none")]
    pub azure_resource_id: Option<String>,
    #[doc = "Version of the NewRelic agent installed on the App service."]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "Status of the NewRelic agent installed on the App service."]
    #[serde(rename = "agentStatus", default, skip_serializing_if = "Option::is_none")]
    pub agent_status: Option<String>,
}
impl AppServiceInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "App services Get Parameter specification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppServicesGetParameter {
    #[doc = "Request of a app services get Operation."]
    pub request: AppServicesGetRequest,
}
impl AppServicesGetParameter {
    pub fn new(request: AppServicesGetRequest) -> Self {
        Self { request }
    }
}
#[doc = "Request of a app services get Operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppServicesGetRequest {
    #[doc = "Azure resource IDs"]
    #[serde(
        rename = "azureResourceIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub azure_resource_ids: Vec<String>,
    #[doc = "Reusable representation of an email address"]
    #[serde(rename = "userEmail")]
    pub user_email: Email,
}
impl AppServicesGetRequest {
    pub fn new(user_email: Email) -> Self {
        Self {
            azure_resource_ids: Vec::new(),
            user_email,
        }
    }
}
#[doc = "Response of a list app services Operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppServicesListResponse {
    #[doc = "The AppServiceInfo items on this page"]
    pub value: Vec<AppServiceInfo>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AppServicesListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AppServicesListResponse {
    pub fn new(value: Vec<AppServiceInfo>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Different usage type like YEARLY/MONTHLY"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BillingCycle")]
pub enum BillingCycle {
    #[serde(rename = "YEARLY")]
    Yearly,
    #[serde(rename = "MONTHLY")]
    Monthly,
    #[serde(rename = "WEEKLY")]
    Weekly,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BillingCycle {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BillingCycle {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BillingCycle {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Yearly => serializer.serialize_unit_variant("BillingCycle", 0u32, "YEARLY"),
            Self::Monthly => serializer.serialize_unit_variant("BillingCycle", 1u32, "MONTHLY"),
            Self::Weekly => serializer.serialize_unit_variant("BillingCycle", 2u32, "WEEKLY"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Billing source"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BillingSource")]
pub enum BillingSource {
    #[serde(rename = "AZURE")]
    Azure,
    #[serde(rename = "NEWRELIC")]
    Newrelic,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BillingSource {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BillingSource {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BillingSource {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Azure => serializer.serialize_unit_variant("BillingSource", 0u32, "AZURE"),
            Self::Newrelic => serializer.serialize_unit_variant("BillingSource", 1u32, "NEWRELIC"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
#[doc = "Host Get Parameter specification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostsGetParameter {
    #[doc = "Request of a Hosts get Operation."]
    pub request: HostsGetRequest,
}
impl HostsGetParameter {
    pub fn new(request: HostsGetRequest) -> Self {
        Self { request }
    }
}
#[doc = "Request of a Hosts get Operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostsGetRequest {
    #[doc = "VM resource IDs"]
    #[serde(
        rename = "vmIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vm_ids: Vec<String>,
    #[doc = "Reusable representation of an email address"]
    #[serde(rename = "userEmail")]
    pub user_email: Email,
}
impl HostsGetRequest {
    pub fn new(user_email: Email) -> Self {
        Self {
            vm_ids: Vec::new(),
            user_email,
        }
    }
}
#[doc = "Liftr Resource category."]
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
#[doc = "Location for NewRelic resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocationParameter {}
impl LocationParameter {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(
        rename = "filteringTags",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub filtering_tags: Vec<FilteringTag>,
}
impl LogRules {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed service identity (system assigned and/or user assigned identities)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedServiceIdentity {
    #[doc = "The service principal ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
    #[serde(rename = "type")]
    pub type_: ManagedServiceIdentityType,
    #[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedIdentities>,
}
impl ManagedServiceIdentity {
    pub fn new(type_: ManagedServiceIdentityType) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
            user_assigned_identities: None,
        }
    }
}
#[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedServiceIdentityType")]
pub enum ManagedServiceIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
    #[serde(rename = "SystemAssigned, UserAssigned")]
    SystemAssignedUserAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagedServiceIdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagedServiceIdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagedServiceIdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ManagedServiceIdentityType", 0u32, "None"),
            Self::SystemAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 1u32, "SystemAssigned"),
            Self::UserAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 2u32, "UserAssigned"),
            Self::SystemAssignedUserAssigned => {
                serializer.serialize_unit_variant("ManagedServiceIdentityType", 3u32, "SystemAssigned, UserAssigned")
            }
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
    #[doc = "Indicates whether metrics are being sent."]
    #[serde(rename = "sendMetrics", default, skip_serializing_if = "Option::is_none")]
    pub send_metrics: Option<SendMetricsStatus>,
    #[doc = "List of filtering tags to be used for capturing metrics."]
    #[serde(
        rename = "filteringTags",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub filtering_tags: Vec<FilteringTag>,
    #[doc = "Reusable representation of an email address"]
    #[serde(rename = "userEmail", default, skip_serializing_if = "Option::is_none")]
    pub user_email: Option<Email>,
}
impl MetricRules {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request of get metrics Operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricsRequest {
    #[doc = "Reusable representation of an email address"]
    #[serde(rename = "userEmail")]
    pub user_email: Email,
}
impl MetricsRequest {
    pub fn new(user_email: Email) -> Self {
        Self { user_email }
    }
}
#[doc = "Get Metrics Status Parameter specification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricsRequestParameter {
    #[doc = "Request of get metrics Operation."]
    pub request: MetricsRequest,
}
impl MetricsRequestParameter {
    pub fn new(request: MetricsRequest) -> Self {
        Self { request }
    }
}
#[doc = "Request of get metrics status Operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricsStatusRequest {
    #[doc = "Azure resource IDs"]
    #[serde(
        rename = "azureResourceIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub azure_resource_ids: Vec<String>,
    #[doc = "Reusable representation of an email address"]
    #[serde(rename = "userEmail")]
    pub user_email: Email,
}
impl MetricsStatusRequest {
    pub fn new(user_email: Email) -> Self {
        Self {
            azure_resource_ids: Vec::new(),
            user_email,
        }
    }
}
#[doc = "Get Metrics Status Parameter specification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricsStatusRequestParameter {
    #[doc = "Request of get metrics status Operation."]
    pub request: MetricsStatusRequest,
}
impl MetricsStatusRequestParameter {
    pub fn new(request: MetricsStatusRequest) -> Self {
        Self { request }
    }
}
#[doc = "Response of get metrics status Operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricsStatusResponse {
    #[doc = "Azure resource IDs"]
    #[serde(
        rename = "azureResourceIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub azure_resource_ids: Vec<String>,
}
impl MetricsStatusResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties specific to the NewRelic Monitor resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitorProperties {
    #[doc = "Provisioning State of the Monitor resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Flag specifying if the resource monitoring is enabled or disabled."]
    #[serde(rename = "monitoringStatus", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_status: Option<MonitoringStatus>,
    #[doc = "Flag specifying the Marketplace Subscription Status of the resource. If payment is not made in time, the resource will go in Suspended state."]
    #[serde(rename = "marketplaceSubscriptionStatus", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_subscription_status: Option<MarketplaceSubscriptionStatus>,
    #[doc = "Marketplace Subscription Id"]
    #[serde(rename = "marketplaceSubscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_subscription_id: Option<String>,
    #[doc = "Properties of the NewRelic account"]
    #[serde(rename = "newRelicAccountProperties", default, skip_serializing_if = "Option::is_none")]
    pub new_relic_account_properties: Option<NewRelicAccountProperties>,
    #[doc = "User Info of NewRelic Monitor resource"]
    #[serde(rename = "userInfo", default, skip_serializing_if = "Option::is_none")]
    pub user_info: Option<UserInfo>,
    #[doc = "Plan data of NewRelic Monitor resource"]
    #[serde(rename = "planData", default, skip_serializing_if = "Option::is_none")]
    pub plan_data: Option<PlanData>,
    #[doc = "Liftr Resource category."]
    #[serde(rename = "liftrResourceCategory", default, skip_serializing_if = "Option::is_none")]
    pub liftr_resource_category: Option<LiftrResourceCategories>,
    #[doc = "Liftr resource preference. The priority of the resource."]
    #[serde(rename = "liftrResourcePreference", default, skip_serializing_if = "Option::is_none")]
    pub liftr_resource_preference: Option<i32>,
    #[doc = "Source of Org creation"]
    #[serde(rename = "orgCreationSource", default, skip_serializing_if = "Option::is_none")]
    pub org_creation_source: Option<OrgCreationSource>,
    #[doc = "Source of Account creation"]
    #[serde(rename = "accountCreationSource", default, skip_serializing_if = "Option::is_none")]
    pub account_creation_source: Option<AccountCreationSource>,
}
impl MonitorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of resource being monitored by NewRelic monitor resource"]
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
#[doc = "List of all the resources being monitored by NewRelic monitor resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonitoredResourceListResponse {
    #[doc = "The MonitoredResource items on this page"]
    pub value: Vec<MonitoredResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MonitoredResourceListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl MonitoredResourceListResponse {
    pub fn new(value: Vec<MonitoredResource>) -> Self {
        Self { value, next_link: None }
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
#[doc = "The resource-specific properties for this resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoringTagRulesProperties {
    #[doc = "Provisioning State of the Monitor resource"]
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
#[doc = "Properties of the NewRelic account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NewRelicAccountProperties {
    #[doc = "User id"]
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[doc = "Account Info of the NewRelic account"]
    #[serde(rename = "accountInfo", default, skip_serializing_if = "Option::is_none")]
    pub account_info: Option<AccountInfo>,
    #[doc = "Organization Info of the NewRelic account"]
    #[serde(rename = "organizationInfo", default, skip_serializing_if = "Option::is_none")]
    pub organization_info: Option<OrganizationInfo>,
    #[doc = "Single sign on Info of the NewRelic account"]
    #[serde(rename = "singleSignOnProperties", default, skip_serializing_if = "Option::is_none")]
    pub single_sign_on_properties: Option<NewRelicSingleSignOnProperties>,
}
impl NewRelicAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Monitor Resource by NewRelic"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NewRelicMonitorResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties specific to the NewRelic Monitor resource"]
    pub properties: MonitorProperties,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl NewRelicMonitorResource {
    pub fn new(tracked_resource: TrackedResource, properties: MonitorProperties) -> Self {
        Self {
            tracked_resource,
            properties,
            identity: None,
        }
    }
}
#[doc = "The response of a NewRelicMonitorResource list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NewRelicMonitorResourceListResult {
    #[doc = "The NewRelicMonitorResource items on this page"]
    pub value: Vec<NewRelicMonitorResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NewRelicMonitorResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NewRelicMonitorResourceListResult {
    pub fn new(value: Vec<NewRelicMonitorResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the NewRelicMonitorResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NewRelicMonitorResourceUpdate {
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The updatable properties of the NewRelicMonitorResource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NewRelicMonitorResourceUpdateProperties>,
}
impl NewRelicMonitorResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the NewRelicMonitorResource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NewRelicMonitorResourceUpdateProperties {
    #[doc = "Properties of the NewRelic account"]
    #[serde(rename = "newRelicAccountProperties", default, skip_serializing_if = "Option::is_none")]
    pub new_relic_account_properties: Option<NewRelicAccountProperties>,
    #[doc = "User Info of NewRelic Monitor resource"]
    #[serde(rename = "userInfo", default, skip_serializing_if = "Option::is_none")]
    pub user_info: Option<UserInfo>,
    #[doc = "Plan data of NewRelic Monitor resource"]
    #[serde(rename = "planData", default, skip_serializing_if = "Option::is_none")]
    pub plan_data: Option<PlanData>,
    #[doc = "Source of Org creation"]
    #[serde(rename = "orgCreationSource", default, skip_serializing_if = "Option::is_none")]
    pub org_creation_source: Option<OrgCreationSource>,
    #[doc = "Source of Account creation"]
    #[serde(rename = "accountCreationSource", default, skip_serializing_if = "Option::is_none")]
    pub account_creation_source: Option<AccountCreationSource>,
}
impl NewRelicMonitorResourceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Single sign on Info of the NewRelic account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NewRelicSingleSignOnProperties {
    #[doc = "Various states of the SSO resource"]
    #[serde(rename = "singleSignOnState", default, skip_serializing_if = "Option::is_none")]
    pub single_sign_on_state: Option<SingleSignOnStates>,
    #[doc = "The Id of the Enterprise App used for Single sign-on."]
    #[serde(rename = "enterpriseAppId", default, skip_serializing_if = "Option::is_none")]
    pub enterprise_app_id: Option<String>,
    #[doc = "The login URL specific to this NewRelic Organization"]
    #[serde(rename = "singleSignOnUrl", default, skip_serializing_if = "Option::is_none")]
    pub single_sign_on_url: Option<String>,
    #[doc = "Provisioning State of the Monitor resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl NewRelicSingleSignOnProperties {
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
#[doc = "Source of Org creation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OrgCreationSource")]
pub enum OrgCreationSource {
    #[serde(rename = "LIFTR")]
    Liftr,
    #[serde(rename = "NEWRELIC")]
    Newrelic,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OrgCreationSource {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OrgCreationSource {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OrgCreationSource {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Liftr => serializer.serialize_unit_variant("OrgCreationSource", 0u32, "LIFTR"),
            Self::Newrelic => serializer.serialize_unit_variant("OrgCreationSource", 1u32, "NEWRELIC"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Organization Id parameter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrganizationIdParameter {}
impl OrganizationIdParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Organization Info of the NewRelic account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrganizationInfo {
    #[doc = "Organization id"]
    #[serde(rename = "organizationId", default, skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
}
impl OrganizationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of Organizations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrganizationProperties {
    #[doc = "organization id"]
    #[serde(rename = "organizationId", default, skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    #[doc = "organization name"]
    #[serde(rename = "organizationName", default, skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,
    #[doc = "Billing source"]
    #[serde(rename = "billingSource", default, skip_serializing_if = "Option::is_none")]
    pub billing_source: Option<BillingSource>,
}
impl OrganizationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of a Organization resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrganizationResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Details of Organizations"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OrganizationProperties>,
}
impl OrganizationResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of get all organizations Operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationsListResponse {
    #[doc = "The OrganizationResource items on this page"]
    pub value: Vec<OrganizationResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OrganizationsListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OrganizationsListResponse {
    pub fn new(value: Vec<OrganizationResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Plan data of NewRelic Monitor resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanData {
    #[doc = "Different usage type like PAYG/COMMITTED"]
    #[serde(rename = "usageType", default, skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<UsageType>,
    #[doc = "Different usage type like YEARLY/MONTHLY"]
    #[serde(rename = "billingCycle", default, skip_serializing_if = "Option::is_none")]
    pub billing_cycle: Option<BillingCycle>,
    #[doc = "plan id as published by NewRelic"]
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
#[doc = "Response of get all plan data Operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanDataListResponse {
    #[doc = "The PlanDataResource items on this page"]
    pub value: Vec<PlanDataResource>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PlanDataListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PlanDataListResponse {
    pub fn new(value: Vec<PlanDataResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Plan details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanDataProperties {
    #[doc = "Plan data of NewRelic Monitor resource"]
    #[serde(rename = "planData", default, skip_serializing_if = "Option::is_none")]
    pub plan_data: Option<PlanData>,
    #[doc = "Source of Org creation"]
    #[serde(rename = "orgCreationSource", default, skip_serializing_if = "Option::is_none")]
    pub org_creation_source: Option<OrgCreationSource>,
    #[doc = "Source of Account creation"]
    #[serde(rename = "accountCreationSource", default, skip_serializing_if = "Option::is_none")]
    pub account_creation_source: Option<AccountCreationSource>,
}
impl PlanDataProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of a PlanData resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanDataResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Plan details"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PlanDataProperties>,
}
impl PlanDataResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provisioning State of the Monitor resource"]
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
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Indicates whether metrics are being sent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SendMetricsStatus")]
pub enum SendMetricsStatus {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SendMetricsStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SendMetricsStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SendMetricsStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("SendMetricsStatus", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("SendMetricsStatus", 1u32, "Disabled"),
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
#[doc = "Switch Billing Parameter specification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwitchBillingParameter {
    #[doc = "Request of a switch billing Operation."]
    pub request: SwitchBillingRequest,
}
impl SwitchBillingParameter {
    pub fn new(request: SwitchBillingRequest) -> Self {
        Self { request }
    }
}
#[doc = "Request of a switch billing Operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwitchBillingRequest {
    #[doc = "Azure resource Id"]
    #[serde(rename = "azureResourceId", default, skip_serializing_if = "Option::is_none")]
    pub azure_resource_id: Option<String>,
    #[doc = "Organization id"]
    #[serde(rename = "organizationId", default, skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    #[doc = "Plan data of NewRelic Monitor resource"]
    #[serde(rename = "planData", default, skip_serializing_if = "Option::is_none")]
    pub plan_data: Option<PlanData>,
    #[doc = "Reusable representation of an email address"]
    #[serde(rename = "userEmail")]
    pub user_email: Email,
}
impl SwitchBillingRequest {
    pub fn new(user_email: Email) -> Self {
        Self {
            azure_resource_id: None,
            organization_id: None,
            plan_data: None,
            user_email,
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
#[doc = "A tag rule belonging to NewRelic account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The resource-specific properties for this resource."]
    pub properties: MonitoringTagRulesProperties,
}
impl TagRule {
    pub fn new(properties: MonitoringTagRulesProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "The response of a TagRule list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TagRuleListResult {
    #[doc = "The TagRule items on this page"]
    pub value: Vec<TagRule>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TagRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TagRuleListResult {
    pub fn new(value: Vec<TagRule>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The type used for update operations of the TagRule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagRuleUpdate {
    #[doc = "The updatable properties of the TagRule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TagRuleUpdateProperties>,
}
impl TagRuleUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The updatable properties of the TagRule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagRuleUpdateProperties {
    #[doc = "Set of rules for sending logs for the Monitor resource."]
    #[serde(rename = "logRules", default, skip_serializing_if = "Option::is_none")]
    pub log_rules: Option<LogRules>,
    #[doc = "Set of rules for sending metrics for the Monitor resource."]
    #[serde(rename = "metricRules", default, skip_serializing_if = "Option::is_none")]
    pub metric_rules: Option<MetricRules>,
}
impl TagRuleUpdateProperties {
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
#[doc = "Different usage type like PAYG/COMMITTED"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UsageType")]
pub enum UsageType {
    #[serde(rename = "PAYG")]
    Payg,
    #[serde(rename = "COMMITTED")]
    Committed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UsageType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UsageType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UsageType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Payg => serializer.serialize_unit_variant("UsageType", 0u32, "PAYG"),
            Self::Committed => serializer.serialize_unit_variant("UsageType", 1u32, "COMMITTED"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentities {}
impl UserAssignedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User assigned identity properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "The principal ID of the assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client ID of the assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User email specification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserEmailParameter {}
impl UserEmailParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User Info of NewRelic Monitor resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserInfo {
    #[doc = "First name"]
    #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "Last name"]
    #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "Reusable representation of an email address"]
    #[serde(rename = "emailAddress", default, skip_serializing_if = "Option::is_none")]
    pub email_address: Option<Email>,
    #[doc = "Contact phone number"]
    #[serde(rename = "phoneNumber", default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[doc = "country if user"]
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
    #[doc = "Ingestion key of the account"]
    #[serde(rename = "ingestionKey", default, skip_serializing_if = "Option::is_none")]
    pub ingestion_key: Option<String>,
}
impl VmExtensionPayload {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of a list VM Host Operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmHostsListResponse {
    #[doc = "The VMInfo items on this page"]
    pub value: Vec<VmInfo>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VmHostsListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl VmHostsListResponse {
    pub fn new(value: Vec<VmInfo>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Details of VM Resource having NewRelic OneAgent installed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VmInfo {
    #[doc = "Azure VM resource ID"]
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[doc = "Version of the NewRelic agent installed on the VM."]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "Status of the NewRelic agent installed on the VM."]
    #[serde(rename = "agentStatus", default, skip_serializing_if = "Option::is_none")]
    pub agent_status: Option<String>,
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
