#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Cognitive Services account is an Azure resource representing the provisioned account, it's type, location and SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Account {
    #[serde(flatten)]
    pub azure_entity_resource: AzureEntityResource,
    #[doc = "The kind (type) of cognitive service account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    #[doc = "The resource model definition representing SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Properties of Cognitive Services account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccountProperties>,
}
impl Account {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of cognitive services accounts operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountListResult {
    #[doc = "The link used to get the next page of accounts."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Gets the list of Cognitive Services accounts and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Account>,
}
impl azure_core::Continuable for AccountListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AccountListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of Cognitive Services account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountProperties {
    #[doc = "Gets the status of the cognitive services account at the time the operation was called."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<account_properties::ProvisioningState>,
    #[doc = "Endpoint of the created account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[doc = "The internal identifier (deprecated, do not use this property)."]
    #[serde(rename = "internalId", default, skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<String>,
    #[doc = "Gets the capabilities of the cognitive services account. Each item indicates the capability of a specific feature. The values are read-only and for reference only."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<SkuCapability>,
    #[doc = "If the resource is migrated from an existing key."]
    #[serde(rename = "isMigrated", default, skip_serializing_if = "Option::is_none")]
    pub is_migrated: Option<bool>,
    #[doc = "Resource migration token."]
    #[serde(rename = "migrationToken", default, skip_serializing_if = "Option::is_none")]
    pub migration_token: Option<String>,
    #[doc = "Sku change info of account."]
    #[serde(rename = "skuChangeInfo", default, skip_serializing_if = "Option::is_none")]
    pub sku_change_info: Option<SkuChangeInfo>,
    #[doc = "Optional subdomain name used for token-based authentication."]
    #[serde(rename = "customSubDomainName", default, skip_serializing_if = "Option::is_none")]
    pub custom_sub_domain_name: Option<String>,
    #[doc = "A set of rules governing the network accessibility."]
    #[serde(rename = "networkAcls", default, skip_serializing_if = "Option::is_none")]
    pub network_acls: Option<NetworkRuleSet>,
    #[doc = "Properties to configure Encryption"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[doc = "The storage accounts for this resource."]
    #[serde(rename = "userOwnedStorage", default, skip_serializing_if = "Vec::is_empty")]
    pub user_owned_storage: Vec<UserOwnedStorage>,
    #[doc = "The private endpoint connection associated with the Cognitive Services account."]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "Whether or not public endpoint access is allowed for this account. Value is optional but if passed in, must be 'Enabled' or 'Disabled'"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<account_properties::PublicNetworkAccess>,
    #[doc = "The api properties for special APIs."]
    #[serde(rename = "apiProperties", default, skip_serializing_if = "Option::is_none")]
    pub api_properties: Option<ApiProperties>,
    #[doc = "Gets the date of cognitive services account creation."]
    #[serde(rename = "dateCreated", default, skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[doc = "The call rate limit Cognitive Services account."]
    #[serde(rename = "callRateLimit", default, skip_serializing_if = "Option::is_none")]
    pub call_rate_limit: Option<CallRateLimit>,
    #[serde(rename = "quotaLimit", default, skip_serializing_if = "Option::is_none")]
    pub quota_limit: Option<QuotaLimit>,
    #[serde(rename = "restrictOutboundNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub restrict_outbound_network_access: Option<bool>,
    #[serde(rename = "allowedFqdnList", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_fqdn_list: Vec<String>,
    #[serde(rename = "disableLocalAuth", default, skip_serializing_if = "Option::is_none")]
    pub disable_local_auth: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore: Option<bool>,
}
impl AccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod account_properties {
    use super::*;
    #[doc = "Gets the status of the cognitive services account at the time the operation was called."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Accepted,
        Creating,
        Deleting,
        Moving,
        Failed,
        Succeeded,
        #[serde(rename = "ResolvingDNS")]
        ResolvingDns,
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
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Moving"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Succeeded"),
                Self::ResolvingDns => serializer.serialize_unit_variant("ProvisioningState", 6u32, "ResolvingDNS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Whether or not public endpoint access is allowed for this account. Value is optional but if passed in, must be 'Enabled' or 'Disabled'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicNetworkAccess")]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicNetworkAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicNetworkAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicNetworkAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Cognitive Services resource type and SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountSku {
    #[doc = "Resource Namespace and Type"]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The resource model definition representing SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl AccountSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of cognitive services accounts operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountSkuListResult {
    #[doc = "Gets the list of Cognitive Services accounts and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AccountSku>,
}
impl AccountSkuListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The access keys for the cognitive services account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiKeys {
    #[doc = "Gets the value of key 1."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key1: Option<String>,
    #[doc = "Gets the value of key 2."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key2: Option<String>,
}
impl ApiKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The api properties for special APIs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiProperties {
    #[doc = "(QnAMaker Only) The runtime endpoint of QnAMaker."]
    #[serde(rename = "qnaRuntimeEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub qna_runtime_endpoint: Option<String>,
    #[doc = "(QnAMaker Only) The Azure Search endpoint key of QnAMaker."]
    #[serde(rename = "qnaAzureSearchEndpointKey", default, skip_serializing_if = "Option::is_none")]
    pub qna_azure_search_endpoint_key: Option<String>,
    #[doc = "(QnAMaker Only) The Azure Search endpoint id of QnAMaker."]
    #[serde(rename = "qnaAzureSearchEndpointId", default, skip_serializing_if = "Option::is_none")]
    pub qna_azure_search_endpoint_id: Option<String>,
    #[doc = "(Bing Search Only) The flag to enable statistics of Bing Search."]
    #[serde(rename = "statisticsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub statistics_enabled: Option<bool>,
    #[doc = "(Personalization Only) The flag to enable statistics of Bing Search."]
    #[serde(rename = "eventHubConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_connection_string: Option<String>,
    #[doc = "(Personalization Only) The storage account connection string."]
    #[serde(rename = "storageAccountConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_connection_string: Option<String>,
    #[doc = "(Metrics Advisor Only) The Azure AD Client Id (Application Id)."]
    #[serde(rename = "aadClientId", default, skip_serializing_if = "Option::is_none")]
    pub aad_client_id: Option<String>,
    #[doc = "(Metrics Advisor Only) The Azure AD Tenant Id."]
    #[serde(rename = "aadTenantId", default, skip_serializing_if = "Option::is_none")]
    pub aad_tenant_id: Option<String>,
    #[doc = "(Metrics Advisor Only) The super user of Metrics Advisor."]
    #[serde(rename = "superUser", default, skip_serializing_if = "Option::is_none")]
    pub super_user: Option<String>,
    #[doc = "(Metrics Advisor Only) The website name of Metrics Advisor."]
    #[serde(rename = "websiteName", default, skip_serializing_if = "Option::is_none")]
    pub website_name: Option<String>,
}
impl ApiProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for an Azure Resource Manager resource with an etag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureEntityResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource Etag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl AzureEntityResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The call rate limit Cognitive Services account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CallRateLimit {
    #[doc = "The count value of Call Rate Limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,
    #[doc = "The renewal period in seconds of Call Rate Limit."]
    #[serde(rename = "renewalPeriod", default, skip_serializing_if = "Option::is_none")]
    pub renewal_period: Option<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<ThrottlingRule>,
}
impl CallRateLimit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Check Domain availability parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckDomainAvailabilityParameter {
    #[doc = "The subdomain name to use."]
    #[serde(rename = "subdomainName")]
    pub subdomain_name: String,
    #[doc = "The Type of the resource."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The kind (type) of cognitive service account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
}
impl CheckDomainAvailabilityParameter {
    pub fn new(subdomain_name: String, type_: String) -> Self {
        Self {
            subdomain_name,
            type_,
            kind: None,
        }
    }
}
#[doc = "Check SKU availability parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckSkuAvailabilityParameter {
    #[doc = "The SKU of the resource."]
    pub skus: Vec<SkuName>,
    #[doc = "The kind (type) of cognitive service account."]
    pub kind: Kind,
    #[doc = "The Type of the resource."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl CheckSkuAvailabilityParameter {
    pub fn new(skus: Vec<SkuName>, kind: Kind, type_: String) -> Self {
        Self { skus, kind, type_ }
    }
}
#[doc = "Cognitive Services account commitment cost."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommitmentCost {
    #[doc = "Commitment meter Id."]
    #[serde(rename = "commitmentMeterId", default, skip_serializing_if = "Option::is_none")]
    pub commitment_meter_id: Option<String>,
    #[doc = "Overage meter Id."]
    #[serde(rename = "overageMeterId", default, skip_serializing_if = "Option::is_none")]
    pub overage_meter_id: Option<String>,
}
impl CommitmentCost {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cognitive Services account commitment period."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommitmentPeriod {
    #[doc = "Commitment period commitment tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "Commitment period commitment count."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[doc = "Cognitive Services account commitment quota."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quota: Option<CommitmentQuota>,
    #[doc = "Commitment period start date."]
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[doc = "Commitment period end date."]
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}
impl CommitmentPeriod {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cognitive Services account commitment plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommitmentPlan {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Resource Etag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Properties of Cognitive Services account commitment plan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CommitmentPlanProperties>,
}
impl CommitmentPlan {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of cognitive services accounts operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommitmentPlanListResult {
    #[doc = "The link used to get the next page of CommitmentPlan."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Gets the list of Cognitive Services accounts CommitmentPlan and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CommitmentPlan>,
}
impl azure_core::Continuable for CommitmentPlanListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CommitmentPlanListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of Cognitive Services account commitment plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommitmentPlanProperties {
    #[doc = "Account hosting model."]
    #[serde(rename = "hostingModel", default, skip_serializing_if = "Option::is_none")]
    pub hosting_model: Option<HostingModel>,
    #[doc = "Commitment plan type."]
    #[serde(rename = "planType", default, skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    #[doc = "Cognitive Services account commitment period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current: Option<CommitmentPeriod>,
    #[doc = "AutoRenew commitment plan."]
    #[serde(rename = "autoRenew", default, skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    #[doc = "Cognitive Services account commitment period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<CommitmentPeriod>,
    #[doc = "Cognitive Services account commitment period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last: Option<CommitmentPeriod>,
}
impl CommitmentPlanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cognitive Services account commitment quota."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommitmentQuota {
    #[doc = "Commitment quota quantity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[doc = "Commitment quota unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
impl CommitmentQuota {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cognitive Services account commitment tier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommitmentTier {
    #[doc = "The kind (type) of cognitive service account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    #[doc = "The name of the SKU. Ex - P3. It is typically a letter+number code"]
    #[serde(rename = "skuName", default, skip_serializing_if = "Option::is_none")]
    pub sku_name: Option<String>,
    #[doc = "Account hosting model."]
    #[serde(rename = "hostingModel", default, skip_serializing_if = "Option::is_none")]
    pub hosting_model: Option<HostingModel>,
    #[doc = "Commitment plan type."]
    #[serde(rename = "planType", default, skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    #[doc = "Commitment period commitment tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "Commitment period commitment max count."]
    #[serde(rename = "maxCount", default, skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i32>,
    #[doc = "Cognitive Services account commitment quota."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quota: Option<CommitmentQuota>,
    #[doc = "Cognitive Services account commitment cost."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost: Option<CommitmentCost>,
}
impl CommitmentTier {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of cognitive services accounts operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommitmentTierListResult {
    #[doc = "The link used to get the next page of CommitmentTier."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Gets the list of Cognitive Services accounts CommitmentTier and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CommitmentTier>,
}
impl azure_core::Continuable for CommitmentTierListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CommitmentTierListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cognitive Services account deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Deployment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Resource Etag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Properties of Cognitive Services account deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeploymentProperties>,
}
impl Deployment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of cognitive services accounts operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentListResult {
    #[doc = "The link used to get the next page of Deployment."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Gets the list of Cognitive Services accounts Deployment and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Deployment>,
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
#[doc = "Properties of Cognitive Services account deployment model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentModel {
    #[doc = "Deployment model format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[doc = "Deployment model name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Deployment model version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl DeploymentModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of Cognitive Services account deployment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentProperties {
    #[doc = "Gets the status of the resource at the time the operation was called."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<deployment_properties::ProvisioningState>,
    #[doc = "Properties of Cognitive Services account deployment model."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<DeploymentModel>,
    #[doc = "Properties of Cognitive Services account deployment model."]
    #[serde(rename = "scaleSettings", default, skip_serializing_if = "Option::is_none")]
    pub scale_settings: Option<DeploymentScaleSettings>,
}
impl DeploymentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod deployment_properties {
    use super::*;
    #[doc = "Gets the status of the resource at the time the operation was called."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Accepted,
        Creating,
        Deleting,
        Moving,
        Failed,
        Succeeded,
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
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Moving"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Succeeded"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties of Cognitive Services account deployment model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentScaleSettings {
    #[doc = "Deployment scale type."]
    #[serde(rename = "scaleType", default, skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<deployment_scale_settings::ScaleType>,
    #[doc = "Deployment capacity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl DeploymentScaleSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod deployment_scale_settings {
    use super::*;
    #[doc = "Deployment scale type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScaleType")]
    pub enum ScaleType {
        Standard,
        Manual,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScaleType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScaleType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScaleType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Standard => serializer.serialize_unit_variant("ScaleType", 0u32, "Standard"),
                Self::Manual => serializer.serialize_unit_variant("ScaleType", 1u32, "Manual"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Domain availability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainAvailability {
    #[doc = "Indicates the given SKU is available or not."]
    #[serde(rename = "isSubdomainAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_subdomain_available: Option<bool>,
    #[doc = "Reason why the SKU is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The subdomain name to use."]
    #[serde(rename = "subdomainName", default, skip_serializing_if = "Option::is_none")]
    pub subdomain_name: Option<String>,
    #[doc = "The Type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The kind (type) of cognitive service account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
}
impl DomainAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties to configure Encryption"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Encryption {
    #[doc = "Properties to configure keyVault Properties"]
    #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_properties: Option<KeyVaultProperties>,
    #[doc = "Enumerates the possible value of keySource for Encryption"]
    #[serde(rename = "keySource", default, skip_serializing_if = "Option::is_none")]
    pub key_source: Option<encryption::KeySource>,
}
impl Encryption {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod encryption {
    use super::*;
    #[doc = "Enumerates the possible value of keySource for Encryption"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "KeySource")]
    pub enum KeySource {
        #[serde(rename = "Microsoft.CognitiveServices")]
        MicrosoftCognitiveServices,
        #[serde(rename = "Microsoft.KeyVault")]
        MicrosoftKeyVault,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for KeySource {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for KeySource {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for KeySource {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MicrosoftCognitiveServices => serializer.serialize_unit_variant("KeySource", 0u32, "Microsoft.CognitiveServices"),
                Self::MicrosoftKeyVault => serializer.serialize_unit_variant("KeySource", 1u32, "Microsoft.KeyVault"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for KeySource {
        fn default() -> Self {
            Self::MicrosoftKeyVault
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
#[doc = "Account hosting model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HostingModel")]
pub enum HostingModel {
    Web,
    ConnectedContainer,
    DisconnectedContainer,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HostingModel {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HostingModel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HostingModel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Web => serializer.serialize_unit_variant("HostingModel", 0u32, "Web"),
            Self::ConnectedContainer => serializer.serialize_unit_variant("HostingModel", 1u32, "ConnectedContainer"),
            Self::DisconnectedContainer => serializer.serialize_unit_variant("HostingModel", 2u32, "DisconnectedContainer"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Identity for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[doc = "The identity type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
    #[doc = "The tenant ID of resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The principal ID of resource identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The list of user assigned identities associated with the resource. The user identity dictionary key references will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}"]
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
        None,
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
    }
}
#[doc = "A rule governing the accessibility from a specific ip address or ip range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpRule {
    #[doc = "An IPv4 address range in CIDR notation, such as '124.56.78.91' (simple IP address) or '124.56.78.0/24' (all addresses that start with 124.56.78)."]
    pub value: String,
}
impl IpRule {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
#[doc = "Properties to configure keyVault Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultProperties {
    #[doc = "Name of the Key from KeyVault"]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[doc = "Version of the Key from KeyVault"]
    #[serde(rename = "keyVersion", default, skip_serializing_if = "Option::is_none")]
    pub key_version: Option<String>,
    #[doc = "Uri of KeyVault"]
    #[serde(rename = "keyVaultUri", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_uri: Option<String>,
    #[serde(rename = "identityClientId", default, skip_serializing_if = "Option::is_none")]
    pub identity_client_id: Option<String>,
}
impl KeyVaultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type Kind = String;
#[doc = "A metric name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricName {
    #[doc = "The name of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The friendly name of the metric."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl MetricName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A set of rules governing the network accessibility."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkRuleSet {
    #[doc = "The default action when no rule from ipRules and from virtualNetworkRules match. This is only used after the bypass property has been evaluated."]
    #[serde(rename = "defaultAction", default, skip_serializing_if = "Option::is_none")]
    pub default_action: Option<network_rule_set::DefaultAction>,
    #[doc = "The list of IP address rules."]
    #[serde(rename = "ipRules", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_rules: Vec<IpRule>,
    #[doc = "The list of virtual network rules."]
    #[serde(rename = "virtualNetworkRules", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_network_rules: Vec<VirtualNetworkRule>,
}
impl NetworkRuleSet {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_rule_set {
    use super::*;
    #[doc = "The default action when no rule from ipRules and from virtualNetworkRules match. This is only used after the bypass property has been evaluated."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DefaultAction")]
    pub enum DefaultAction {
        Allow,
        Deny,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DefaultAction {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DefaultAction {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DefaultAction {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allow => serializer.serialize_unit_variant("DefaultAction", 0u32, "Allow"),
                Self::Deny => serializer.serialize_unit_variant("DefaultAction", 1u32, "Deny"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
#[doc = "The Private Endpoint resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "The ARM identifier for Private Endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Private Endpoint Connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub azure_entity_resource: AzureEntityResource,
    #[doc = "Properties of the PrivateEndpointConnectProperties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "The location of the private endpoint connection"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private endpoint connections"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "Array of private endpoint connections"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
}
impl PrivateEndpointConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the PrivateEndpointConnectProperties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "The Private Endpoint resource."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "A collection of information about the state of the connection between service consumer and provider."]
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
    #[doc = "The private link resource group ids."]
    #[serde(rename = "groupIds", default, skip_serializing_if = "Vec::is_empty")]
    pub group_ids: Vec<String>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new(private_link_service_connection_state: PrivateLinkServiceConnectionState) -> Self {
        Self {
            private_endpoint: None,
            private_link_service_connection_state,
            provisioning_state: None,
            group_ids: Vec::new(),
        }
    }
}
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointConnectionProvisioningState")]
pub enum PrivateEndpointConnectionProvisioningState {
    Succeeded,
    Creating,
    Deleting,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateEndpointConnectionProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateEndpointConnectionProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateEndpointConnectionProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 0u32, "Succeeded"),
            Self::Creating => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 1u32, "Creating"),
            Self::Deleting => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 2u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 3u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The private endpoint connection status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointServiceConnectionStatus")]
pub enum PrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateEndpointServiceConnectionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateEndpointServiceConnectionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateEndpointServiceConnectionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pending => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 0u32, "Pending"),
            Self::Approved => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 1u32, "Approved"),
            Self::Rejected => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 2u32, "Rejected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private link resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceListResult {
    #[doc = "Array of private link resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
}
impl PrivateLinkResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "The private link resource group id."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The private link resource required member names."]
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "The private link resource Private link DNS zone name."]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
    #[doc = "The private link resource display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of information about the state of the connection between service consumer and provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "The private endpoint connection status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
    #[doc = "The reason for approval/rejection of the connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A message indicating if changes on the service provider require any updates on the consumer."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaLimit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,
    #[serde(rename = "renewalPeriod", default, skip_serializing_if = "Option::is_none")]
    pub renewal_period: Option<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<ThrottlingRule>,
}
impl QuotaLimit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Regenerate key parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegenerateKeyParameters {
    #[doc = "key name to generate (Key1|Key2)"]
    #[serde(rename = "keyName")]
    pub key_name: regenerate_key_parameters::KeyName,
}
impl RegenerateKeyParameters {
    pub fn new(key_name: regenerate_key_parameters::KeyName) -> Self {
        Self { key_name }
    }
}
pub mod regenerate_key_parameters {
    use super::*;
    #[doc = "key name to generate (Key1|Key2)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyName {
        Key1,
        Key2,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestMatchPattern {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}
impl RequestMatchPattern {
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
#[doc = "Describes an available Cognitive Services SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSku {
    #[doc = "The type of resource the SKU applies to."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The name of SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the tier of Cognitive Services account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "The Kind of resources that are supported in this SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "The set of locations that the SKU is available."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[doc = "The restrictions because of which SKU cannot be used. This is empty if there are no restrictions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<ResourceSkuRestrictions>,
}
impl ResourceSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Get Skus operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSkuListResult {
    #[doc = "The list of skus available for the subscription."]
    pub value: Vec<ResourceSku>,
    #[doc = "The uri to fetch the next page of Skus."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceSkuListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResourceSkuListResult {
    pub fn new(value: Vec<ResourceSku>) -> Self {
        Self { value, next_link: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuRestrictionInfo {
    #[doc = "Locations where the SKU is restricted"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[doc = "List of availability zones where the SKU is restricted."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
}
impl ResourceSkuRestrictionInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes restrictions of a SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuRestrictions {
    #[doc = "The type of restrictions."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<resource_sku_restrictions::Type>,
    #[doc = "The value of restrictions. If the restriction type is set to location. This would be different locations where the SKU is restricted."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
    #[serde(rename = "restrictionInfo", default, skip_serializing_if = "Option::is_none")]
    pub restriction_info: Option<ResourceSkuRestrictionInfo>,
    #[doc = "The reason for restriction."]
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<resource_sku_restrictions::ReasonCode>,
}
impl ResourceSkuRestrictions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_sku_restrictions {
    use super::*;
    #[doc = "The type of restrictions."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Location,
        Zone,
    }
    #[doc = "The reason for restriction."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReasonCode")]
    pub enum ReasonCode {
        QuotaId,
        NotAvailableForSubscription,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReasonCode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReasonCode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReasonCode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::QuotaId => serializer.serialize_unit_variant("ReasonCode", 0u32, "QuotaId"),
                Self::NotAvailableForSubscription => serializer.serialize_unit_variant("ReasonCode", 1u32, "NotAvailableForSubscription"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The resource model definition representing SKU"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The name of the SKU. Ex - P3. It is typically a letter+number code"]
    pub name: String,
    #[doc = "This field is required to be implemented by the Resource Provider if the service has more than one tier, but is not required on a PUT."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sku::Tier>,
    #[doc = "The SKU size. When the name field is the combination of tier and some other value, this would be the standalone code. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "If the service has different generations of hardware, for the same SKU, then that can be captured here."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "If the SKU supports scale out/in then the capacity integer should be included. If scale out/in is not possible for the resource this may be omitted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl Sku {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tier: None,
            size: None,
            family: None,
            capacity: None,
        }
    }
}
pub mod sku {
    use super::*;
    #[doc = "This field is required to be implemented by the Resource Provider if the service has more than one tier, but is not required on a PUT."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        Free,
        Basic,
        Standard,
        Premium,
        Enterprise,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Tier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Tier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Tier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Free => serializer.serialize_unit_variant("Tier", 0u32, "Free"),
                Self::Basic => serializer.serialize_unit_variant("Tier", 1u32, "Basic"),
                Self::Standard => serializer.serialize_unit_variant("Tier", 2u32, "Standard"),
                Self::Premium => serializer.serialize_unit_variant("Tier", 3u32, "Premium"),
                Self::Enterprise => serializer.serialize_unit_variant("Tier", 4u32, "Enterprise"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "SKU availability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuAvailability {
    #[doc = "The kind (type) of cognitive service account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    #[doc = "The Type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of SKU."]
    #[serde(rename = "skuName", default, skip_serializing_if = "Option::is_none")]
    pub sku_name: Option<SkuName>,
    #[doc = "Indicates the given SKU is available or not."]
    #[serde(rename = "skuAvailable", default, skip_serializing_if = "Option::is_none")]
    pub sku_available: Option<bool>,
    #[doc = "Reason why the SKU is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "Additional error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl SkuAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Check SKU availability result list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuAvailabilityListResult {
    #[doc = "Check SKU availability result list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SkuAvailability>,
}
impl SkuAvailabilityListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SkuCapability indicates the capability of a certain feature."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuCapability {
    #[doc = "The name of the SkuCapability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of the SkuCapability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl SkuCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sku change info of account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuChangeInfo {
    #[doc = "Gets the count of downgrades."]
    #[serde(rename = "countOfDowngrades", default, skip_serializing_if = "Option::is_none")]
    pub count_of_downgrades: Option<f64>,
    #[doc = "Gets the count of upgrades after downgrades."]
    #[serde(rename = "countOfUpgradesAfterDowngrades", default, skip_serializing_if = "Option::is_none")]
    pub count_of_upgrades_after_downgrades: Option<f64>,
    #[doc = "Gets the last change date."]
    #[serde(rename = "lastChangeDate", default, skip_serializing_if = "Option::is_none")]
    pub last_change_date: Option<String>,
}
impl SkuChangeInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type SkuName = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ThrottlingRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "renewalPeriod", default, skip_serializing_if = "Option::is_none")]
    pub renewal_period: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,
    #[serde(rename = "minCount", default, skip_serializing_if = "Option::is_none")]
    pub min_count: Option<f64>,
    #[serde(rename = "dynamicThrottlingEnabled", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_throttling_enabled: Option<bool>,
    #[serde(rename = "matchPatterns", default, skip_serializing_if = "Vec::is_empty")]
    pub match_patterns: Vec<RequestMatchPattern>,
}
impl ThrottlingRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The unit of the metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UnitType")]
pub enum UnitType {
    Count,
    Bytes,
    Seconds,
    Percent,
    CountPerSecond,
    BytesPerSecond,
    Milliseconds,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UnitType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UnitType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UnitType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Count => serializer.serialize_unit_variant("UnitType", 0u32, "Count"),
            Self::Bytes => serializer.serialize_unit_variant("UnitType", 1u32, "Bytes"),
            Self::Seconds => serializer.serialize_unit_variant("UnitType", 2u32, "Seconds"),
            Self::Percent => serializer.serialize_unit_variant("UnitType", 3u32, "Percent"),
            Self::CountPerSecond => serializer.serialize_unit_variant("UnitType", 4u32, "CountPerSecond"),
            Self::BytesPerSecond => serializer.serialize_unit_variant("UnitType", 5u32, "BytesPerSecond"),
            Self::Milliseconds => serializer.serialize_unit_variant("UnitType", 6u32, "Milliseconds"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The usage data for a usage request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Usage {
    #[doc = "The unit of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<UnitType>,
    #[doc = "A metric name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<MetricName>,
    #[doc = "The quota period used to summarize the usage values."]
    #[serde(rename = "quotaPeriod", default, skip_serializing_if = "Option::is_none")]
    pub quota_period: Option<String>,
    #[doc = "Maximum value for this metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[doc = "Current value for this metric."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<f64>,
    #[doc = "Next reset time for current quota."]
    #[serde(rename = "nextResetTime", default, skip_serializing_if = "Option::is_none")]
    pub next_reset_time: Option<String>,
    #[doc = "Cognitive Services account quota usage status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<usage::Status>,
}
impl Usage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod usage {
    use super::*;
    #[doc = "Cognitive Services account quota usage status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Included,
        Blocked,
        InOverage,
        Unknown,
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
                Self::Included => serializer.serialize_unit_variant("Status", 0u32, "Included"),
                Self::Blocked => serializer.serialize_unit_variant("Status", 1u32, "Blocked"),
                Self::InOverage => serializer.serialize_unit_variant("Status", 2u32, "InOverage"),
                Self::Unknown => serializer.serialize_unit_variant("Status", 3u32, "Unknown"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The response to a list usage request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageListResult {
    #[doc = "The list of usages for Cognitive Service account."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Usage>,
}
impl UsageListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User-assigned managed identity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "Azure Active Directory principal ID associated with this Identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Client App Id associated with this identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The user owned storage for Cognitive Services account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserOwnedStorage {
    #[doc = "Full resource id of a Microsoft.Storage resource."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "identityClientId", default, skip_serializing_if = "Option::is_none")]
    pub identity_client_id: Option<String>,
}
impl UserOwnedStorage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A rule governing the accessibility from a specific virtual network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRule {
    #[doc = "Full resource id of a vnet subnet, such as '/subscriptions/subid/resourceGroups/rg1/providers/Microsoft.Network/virtualNetworks/test-vnet/subnets/subnet1'."]
    pub id: String,
    #[doc = "Gets the state of virtual network rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Ignore missing vnet service endpoint or not."]
    #[serde(rename = "ignoreMissingVnetServiceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing_vnet_service_endpoint: Option<bool>,
}
impl VirtualNetworkRule {
    pub fn new(id: String) -> Self {
        Self {
            id,
            state: None,
            ignore_missing_vnet_service_endpoint: None,
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
