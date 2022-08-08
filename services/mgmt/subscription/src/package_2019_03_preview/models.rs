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
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
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
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
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
    #[doc = "Error code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
    #[doc = "Service provider ID, basically MPN Id."]
    #[serde(rename = "serviceProviderId", default, skip_serializing_if = "Option::is_none")]
    pub service_provider_id: Option<String>,
}
impl ModernCspSubscriptionCreationParameters {
    pub fn new(display_name: String, sku_id: String) -> Self {
        Self {
            display_name,
            sku_id,
            reseller_id: None,
            service_provider_id: None,
        }
    }
}
#[doc = "The parameters required to create a new subscription."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModernSubscriptionCreationParameters {
    #[doc = "The friendly name of the subscription."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The ARM ID of the billing profile for which you want to create the subscription."]
    #[serde(rename = "billingProfileId")]
    pub billing_profile_id: String,
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
    pub fn new(display_name: String, billing_profile_id: String, sku_id: String) -> Self {
        Self {
            display_name,
            billing_profile_id,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "The ID of the subscriptions that is being renamed"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RenamedSubscriptionId {
    #[doc = "The ID of the subscriptions that is being renamed"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Subscription>,
    #[doc = "The URL to get the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SubscriptionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
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
#[doc = "status of the subscription POST operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionOperation {
    #[doc = "The operation Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Status of the pending subscription"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Status Detail of the pending subscription"]
    #[serde(rename = "statusDetail", default, skip_serializing_if = "Option::is_none")]
    pub status_detail: Option<String>,
}
impl SubscriptionOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of pending subscription operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionOperationListResult {
    #[doc = "A list of pending SubscriptionOperations"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SubscriptionOperation>,
}
impl SubscriptionOperationListResult {
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
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
