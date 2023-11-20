#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Active Directory Principal who’ll get owner access on the new subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdPrincipal {
    #[doc = "Object id of the Principal"]
    #[serde(rename = "objectId")]
    pub object_id: String,
}
impl AdPrincipal {
    pub fn new(object_id: String) -> Self {
        Self { object_id }
    }
}
#[doc = "The ID of the canceled subscription"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CanceledSubscriptionId {
    #[doc = "The ID of the canceled subscription"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl CanceledSubscriptionId {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The ID of the subscriptions that is being enabled"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnabledSubscriptionId {
    #[doc = "The ID of the subscriptions that is being enabled"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl EnabledSubscriptionId {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the format of Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicates that the service is not able to process the incoming request. The reason is provided in the error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseBody {
    #[doc = "Describes the format of Error response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl ErrorResponseBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Location information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Location {
    #[doc = "The fully qualified ID of the location. For example, /subscriptions/00000000-0000-0000-0000-000000000000/locations/westus."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The subscription ID."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The location name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The display name of the location."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The latitude of the location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latitude: Option<String>,
    #[doc = "The longitude of the location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longitude: Option<String>,
}
impl Location {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Location list operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocationListResult {
    #[doc = "An array of locations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Location>,
}
impl azure_core::Continuable for LocationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl LocationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters required to create a new CSP subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModernCspSubscriptionCreationParameters {
    #[doc = "The friendly name of the subscription."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The SKU ID of the Azure plan. Azure plan determines the pricing and service-level agreement of the subscription.  Use 001 for Microsoft Azure Plan and 002 for Microsoft Azure Plan for DevTest."]
    #[serde(rename = "skuId")]
    pub sku_id: String,
    #[doc = "Reseller ID, basically MPN Id."]
    #[serde(rename = "resellerId", default, skip_serializing_if = "Option::is_none")]
    pub reseller_id: Option<String>,
}
impl ModernCspSubscriptionCreationParameters {
    pub fn new(display_name: String, sku_id: String) -> Self {
        Self {
            display_name,
            sku_id,
            reseller_id: None,
        }
    }
}
#[doc = "The parameters required to create a new subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModernSubscriptionCreationParameters {
    #[doc = "The friendly name of the subscription."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The SKU ID of the Azure plan. Azure plan determines the pricing and service-level agreement of the subscription.  Use 001 for Microsoft Azure Plan and 002 for Microsoft Azure Plan for DevTest."]
    #[serde(rename = "skuId")]
    pub sku_id: String,
    #[doc = "If set, the cost center will show up on the Azure usage and charges file."]
    #[serde(rename = "costCenter", default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[doc = "Active Directory Principal who’ll get owner access on the new subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<AdPrincipal>,
    #[doc = "The identifier of the management group to which this subscription will be associated."]
    #[serde(rename = "managementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub management_group_id: Option<String>,
    #[doc = "Additional, untyped parameters to support custom subscription creation scenarios."]
    #[serde(rename = "additionalParameters", default, skip_serializing_if = "Option::is_none")]
    pub additional_parameters: Option<serde_json::Value>,
}
impl ModernSubscriptionCreationParameters {
    pub fn new(display_name: String, sku_id: String) -> Self {
        Self {
            display_name,
            sku_id,
            cost_center: None,
            owner: None,
            management_group_id: None,
            additional_parameters: None,
        }
    }
}
#[doc = "REST API operation"]
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
        #[doc = "Service provider: Microsoft.Subscription"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Profile, endpoint, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of aliases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PutAliasListResult {
    #[doc = "The list of alias."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<PutAliasResponse>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PutAliasListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters required to create a new subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PutAliasRequest {
    #[doc = "Put subscription properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PutAliasRequestProperties>,
}
impl PutAliasRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Put subscription properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PutAliasRequestProperties {
    #[doc = "The friendly name of the subscription."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The workload type of the subscription. It can be either Production or DevTest."]
    pub workload: put_alias_request_properties::Workload,
    #[doc = "Determines whether subscription is fieldLed, partnerLed or LegacyEA"]
    #[serde(rename = "billingScope")]
    pub billing_scope: String,
    #[doc = "This parameter can be used to create alias for existing subscription Id"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}
impl PutAliasRequestProperties {
    pub fn new(display_name: String, workload: put_alias_request_properties::Workload, billing_scope: String) -> Self {
        Self {
            display_name,
            workload,
            billing_scope,
            subscription_id: None,
        }
    }
}
pub mod put_alias_request_properties {
    use super::*;
    #[doc = "The workload type of the subscription. It can be either Production or DevTest."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Workload")]
    pub enum Workload {
        Production,
        DevTest,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Workload {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Workload {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Workload {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Production => serializer.serialize_unit_variant("Workload", 0u32, "Production"),
                Self::DevTest => serializer.serialize_unit_variant("Workload", 1u32, "DevTest"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Subscription Information with the alias."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PutAliasResponse {
    #[doc = "Fully qualified ID for the alias resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Alias ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type, Microsoft.Subscription/aliases."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Put subscription creation result properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PutAliasResponseProperties>,
}
impl PutAliasResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Put subscription creation result properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PutAliasResponseProperties {
    #[doc = "Newly created subscription Id."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<put_alias_response_properties::ProvisioningState>,
}
impl PutAliasResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod put_alias_response_properties {
    use super::*;
    #[doc = "The provisioning state of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Accepted,
        Succeeded,
        Failed,
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
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The ID of the subscriptions that is being renamed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RenamedSubscriptionId {
    #[doc = "The ID of the subscriptions that is being renamed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl RenamedSubscriptionId {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subscription information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Subscription {
    #[doc = "The fully qualified ID for the subscription. For example, /subscriptions/00000000-0000-0000-0000-000000000000."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The subscription ID."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The subscription display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The subscription state. Possible values are Enabled, Warned, PastDue, Disabled, and Deleted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<subscription::State>,
    #[doc = "The tenant ID. For example, 00000000-0000-0000-0000-000000000000."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Tags for the subscription"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Subscription policies."]
    #[serde(rename = "subscriptionPolicies", default, skip_serializing_if = "Option::is_none")]
    pub subscription_policies: Option<SubscriptionPolicies>,
    #[doc = "The authorization source of the request. Valid values are one or more combinations of Legacy, RoleBased, Bypassed, Direct and Management. For example, 'Legacy, RoleBased'."]
    #[serde(rename = "authorizationSource", default, skip_serializing_if = "Option::is_none")]
    pub authorization_source: Option<String>,
}
impl Subscription {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subscription {
    use super::*;
    #[doc = "The subscription state. Possible values are Enabled, Warned, PastDue, Disabled, and Deleted."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Enabled,
        Warned,
        PastDue,
        Disabled,
        Deleted,
    }
}
#[doc = "Subscription Creation Parameters required to create a new Azure subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionCreationParameters {
    #[doc = "The display name of the subscription."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The Management Group Id."]
    #[serde(rename = "managementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub management_group_id: Option<String>,
    #[doc = "The list of principals that should be granted Owner access on the subscription. Principals should be of type User, Service Principal or Security Group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub owners: Vec<AdPrincipal>,
    #[doc = "The offer type of the subscription. For example, MS-AZR-0017P (EnterpriseAgreement) and MS-AZR-0148P (EnterpriseAgreement devTest) are available. Only valid when creating a subscription in a enrollment account scope."]
    #[serde(rename = "offerType", default, skip_serializing_if = "Option::is_none")]
    pub offer_type: Option<subscription_creation_parameters::OfferType>,
    #[doc = "Additional, untyped parameters to support custom subscription creation scenarios."]
    #[serde(rename = "additionalParameters", default, skip_serializing_if = "Option::is_none")]
    pub additional_parameters: Option<serde_json::Value>,
}
impl SubscriptionCreationParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subscription_creation_parameters {
    use super::*;
    #[doc = "The offer type of the subscription. For example, MS-AZR-0017P (EnterpriseAgreement) and MS-AZR-0148P (EnterpriseAgreement devTest) are available. Only valid when creating a subscription in a enrollment account scope."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "OfferType")]
    pub enum OfferType {
        #[serde(rename = "MS-AZR-0017P")]
        MsAzr0017p,
        #[serde(rename = "MS-AZR-0148P")]
        MsAzr0148p,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for OfferType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for OfferType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for OfferType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MsAzr0017p => serializer.serialize_unit_variant("OfferType", 0u32, "MS-AZR-0017P"),
                Self::MsAzr0148p => serializer.serialize_unit_variant("OfferType", 1u32, "MS-AZR-0148P"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The created subscription object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionCreationResult {
    #[doc = "The link to the new subscription. Use this link to check the status of subscription creation operation."]
    #[serde(rename = "subscriptionLink", default, skip_serializing_if = "Option::is_none")]
    pub subscription_link: Option<String>,
}
impl SubscriptionCreationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subscription list operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionListResult {
    #[doc = "An array of subscriptions."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Subscription>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SubscriptionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SubscriptionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The new name of the subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionName {
    #[doc = "New subscription name"]
    #[serde(rename = "subscriptionName", default, skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
}
impl SubscriptionName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subscription policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionPolicies {
    #[doc = "The subscription location placement ID. The ID indicates which regions are visible for a subscription. For example, a subscription with a location placement Id of Public_2014-09-01 has access to Azure public regions."]
    #[serde(rename = "locationPlacementId", default, skip_serializing_if = "Option::is_none")]
    pub location_placement_id: Option<String>,
    #[doc = "The subscription quota ID."]
    #[serde(rename = "quotaId", default, skip_serializing_if = "Option::is_none")]
    pub quota_id: Option<String>,
    #[doc = "The subscription spending limit."]
    #[serde(rename = "spendingLimit", default, skip_serializing_if = "Option::is_none")]
    pub spending_limit: Option<subscription_policies::SpendingLimit>,
}
impl SubscriptionPolicies {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subscription_policies {
    use super::*;
    #[doc = "The subscription spending limit."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SpendingLimit {
        On,
        Off,
        CurrentPeriodOff,
    }
}
#[doc = "Tenant Id information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TenantIdDescription {
    #[doc = "The fully qualified ID of the tenant. For example, /tenants/00000000-0000-0000-0000-000000000000."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The tenant ID. For example, 00000000-0000-0000-0000-000000000000."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The category of the tenant. Possible values are TenantCategoryHome,TenantCategoryProjectedBy,TenantCategoryManagedBy"]
    #[serde(rename = "tenantCategory", default, skip_serializing_if = "Option::is_none")]
    pub tenant_category: Option<String>,
    #[doc = "The country/region name of the address for the tenant."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "The Country/region abbreviation for the tenant."]
    #[serde(rename = "countryCode", default, skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[doc = "The display name of the tenant."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The list of domains for the tenant."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domains: Option<String>,
    #[doc = "The default domain for the tenant."]
    #[serde(rename = "defaultDomain", default, skip_serializing_if = "Option::is_none")]
    pub default_domain: Option<String>,
    #[doc = "The tenant type. Only available for Home tenant category."]
    #[serde(rename = "tenantType", default, skip_serializing_if = "Option::is_none")]
    pub tenant_type: Option<String>,
}
impl TenantIdDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tenant Ids information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantListResult {
    #[doc = "An array of tenants."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TenantIdDescription>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink")]
    pub next_link: String,
}
impl azure_core::Continuable for TenantListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        if self.next_link.is_empty() {
            None
        } else {
            Some(self.next_link.clone())
        }
    }
}
impl TenantListResult {
    pub fn new(next_link: String) -> Self {
        Self {
            value: Vec::new(),
            next_link,
        }
    }
}
