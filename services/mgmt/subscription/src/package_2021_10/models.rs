#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The parameters required to accept subscription ownership."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcceptOwnershipRequest {
    #[doc = "Accept subscription ownership request properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AcceptOwnershipRequestProperties>,
}
impl AcceptOwnershipRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Accept subscription ownership request properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcceptOwnershipRequestProperties {
    #[doc = "The friendly name of the subscription."]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "Management group Id for the subscription."]
    #[serde(rename = "managementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub management_group_id: Option<String>,
    #[doc = "Tags for the subscription"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl AcceptOwnershipRequestProperties {
    pub fn new(display_name: String) -> Self {
        Self {
            display_name,
            management_group_id: None,
            tags: None,
        }
    }
}
#[doc = "The accept ownership state of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AcceptOwnershipState")]
pub enum AcceptOwnershipState {
    Pending,
    Completed,
    Expired,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AcceptOwnershipState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AcceptOwnershipState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AcceptOwnershipState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pending => serializer.serialize_unit_variant("AcceptOwnershipState", 0u32, "Pending"),
            Self::Completed => serializer.serialize_unit_variant("AcceptOwnershipState", 1u32, "Completed"),
            Self::Expired => serializer.serialize_unit_variant("AcceptOwnershipState", 2u32, "Expired"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Subscription Accept Ownership Response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AcceptOwnershipStatusResponse {
    #[doc = "Newly created subscription Id."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The accept ownership state of the resource."]
    #[serde(rename = "acceptOwnershipState", default, skip_serializing_if = "Option::is_none")]
    pub accept_ownership_state: Option<AcceptOwnershipState>,
    #[doc = "UPN of the billing owner"]
    #[serde(rename = "billingOwner", default, skip_serializing_if = "Option::is_none")]
    pub billing_owner: Option<String>,
    #[doc = "Tenant Id of the subscription"]
    #[serde(rename = "subscriptionTenantId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_tenant_id: Option<String>,
    #[doc = "The display name of the subscription."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Tags for the subscription"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl AcceptOwnershipStatusResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Billing account policies information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingAccountPoliciesResponse {
    #[doc = "Fully qualified ID for the policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Policy name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Put billing account policies response properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BillingAccountPoliciesResponseProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl BillingAccountPoliciesResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Put billing account policies response properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingAccountPoliciesResponseProperties {
    #[doc = "Service tenant for the billing account."]
    #[serde(rename = "serviceTenants", default, skip_serializing_if = "Vec::is_empty")]
    pub service_tenants: Vec<ServiceTenantResponse>,
    #[doc = "Determine if the transfers are allowed for the billing account"]
    #[serde(rename = "allowTransfers", default, skip_serializing_if = "Option::is_none")]
    pub allow_transfers: Option<bool>,
}
impl BillingAccountPoliciesResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type BillingScope = String;
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
impl azure_core::Continuable for ErrorResponseBody {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponseBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tenant policy information list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetTenantPolicyListResponse {
    #[doc = "The list of tenant policies."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GetTenantPolicyResponse>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GetTenantPolicyListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl GetTenantPolicyListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tenant policy Information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetTenantPolicyResponse {
    #[doc = "Policy Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Policy name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Tenant policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TenantPolicy>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl GetTenantPolicyResponse {
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
#[doc = "REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
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
        #[doc = "Localized friendly description for the operation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
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
#[doc = "Put subscription additional properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PutAliasRequestAdditionalProperties {
    #[doc = "Management group Id for the subscription."]
    #[serde(rename = "managementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub management_group_id: Option<String>,
    #[doc = "Tenant Id of the subscription"]
    #[serde(rename = "subscriptionTenantId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_tenant_id: Option<String>,
    #[doc = "Owner Id of the subscription"]
    #[serde(rename = "subscriptionOwnerId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_owner_id: Option<String>,
    #[doc = "Tags for the subscription"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl PutAliasRequestAdditionalProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Put subscription properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PutAliasRequestProperties {
    #[doc = "The friendly name of the subscription."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The workload type of the subscription. It can be either Production or DevTest."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workload: Option<Workload>,
    #[doc = "Billing scope of the subscription.\nFor CustomerLed and FieldLed - /billingAccounts/{billingAccountName}/billingProfiles/{billingProfileName}/invoiceSections/{invoiceSectionName}\nFor PartnerLed - /billingAccounts/{billingAccountName}/customers/{customerName}\nFor Legacy EA - /billingAccounts/{billingAccountName}/enrollmentAccounts/{enrollmentAccountName}"]
    #[serde(rename = "billingScope", default, skip_serializing_if = "Option::is_none")]
    pub billing_scope: Option<BillingScope>,
    #[doc = "This parameter can be used to create alias for existing subscription Id"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Reseller Id"]
    #[serde(rename = "resellerId", default, skip_serializing_if = "Option::is_none")]
    pub reseller_id: Option<String>,
    #[doc = "Put subscription additional properties."]
    #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<PutAliasRequestAdditionalProperties>,
}
impl PutAliasRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Put tenant policy request properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PutTenantPolicyRequestProperties {
    #[doc = "Blocks the leaving of subscriptions from user's tenant."]
    #[serde(rename = "blockSubscriptionsLeavingTenant", default, skip_serializing_if = "Option::is_none")]
    pub block_subscriptions_leaving_tenant: Option<bool>,
    #[doc = "Blocks the entering of subscriptions into user's tenant."]
    #[serde(rename = "blockSubscriptionsIntoTenant", default, skip_serializing_if = "Option::is_none")]
    pub block_subscriptions_into_tenant: Option<bool>,
    #[doc = "List of user objectIds that are exempted from the set subscription tenant policies for the user's tenant."]
    #[serde(rename = "exemptedPrincipals", default, skip_serializing_if = "Vec::is_empty")]
    pub exempted_principals: Vec<String>,
}
impl PutTenantPolicyRequestProperties {
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
#[doc = "Billing account service tenant."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceTenantResponse {
    #[doc = "Service tenant id."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Service tenant name."]
    #[serde(rename = "tenantName", default, skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
}
impl ServiceTenantResponse {
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
#[doc = "The list of aliases."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionAliasListResult {
    #[doc = "The list of alias."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SubscriptionAliasResponse>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SubscriptionAliasListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subscription Information with the alias."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionAliasResponse {
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
    pub properties: Option<SubscriptionAliasResponseProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl SubscriptionAliasResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Put subscription creation result properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionAliasResponseProperties {
    #[doc = "Newly created subscription Id."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The display name of the subscription."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<subscription_alias_response_properties::ProvisioningState>,
    #[doc = "Url to accept ownership of the subscription."]
    #[serde(rename = "acceptOwnershipUrl", default, skip_serializing_if = "Option::is_none")]
    pub accept_ownership_url: Option<String>,
    #[doc = "The accept ownership state of the resource."]
    #[serde(rename = "acceptOwnershipState", default, skip_serializing_if = "Option::is_none")]
    pub accept_ownership_state: Option<AcceptOwnershipState>,
    #[doc = "Billing scope of the subscription.\nFor CustomerLed and FieldLed - /billingAccounts/{billingAccountName}/billingProfiles/{billingProfileName}/invoiceSections/{invoiceSectionName}\nFor PartnerLed - /billingAccounts/{billingAccountName}/customers/{customerName}\nFor Legacy EA - /billingAccounts/{billingAccountName}/enrollmentAccounts/{enrollmentAccountName}"]
    #[serde(rename = "billingScope", default, skip_serializing_if = "Option::is_none")]
    pub billing_scope: Option<BillingScope>,
    #[doc = "The workload type of the subscription. It can be either Production or DevTest."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workload: Option<Workload>,
    #[doc = "Reseller Id"]
    #[serde(rename = "resellerId", default, skip_serializing_if = "Option::is_none")]
    pub reseller_id: Option<String>,
    #[doc = "Owner Id of the subscription"]
    #[serde(rename = "subscriptionOwnerId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_owner_id: Option<String>,
    #[doc = "The Management Group Id."]
    #[serde(rename = "managementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub management_group_id: Option<String>,
    #[doc = "Created Time"]
    #[serde(rename = "createdTime", default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[doc = "Tags for the subscription"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl SubscriptionAliasResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subscription_alias_response_properties {
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
#[doc = "Tenant policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TenantPolicy {
    #[doc = "Policy Id."]
    #[serde(rename = "policyId", default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[doc = "Blocks the leaving of subscriptions from user's tenant."]
    #[serde(rename = "blockSubscriptionsLeavingTenant", default, skip_serializing_if = "Option::is_none")]
    pub block_subscriptions_leaving_tenant: Option<bool>,
    #[doc = "Blocks the entering of subscriptions into user's tenant."]
    #[serde(rename = "blockSubscriptionsIntoTenant", default, skip_serializing_if = "Option::is_none")]
    pub block_subscriptions_into_tenant: Option<bool>,
    #[doc = "List of user objectIds that are exempted from the set subscription tenant policies for the user's tenant."]
    #[serde(rename = "exemptedPrincipals", default, skip_serializing_if = "Vec::is_empty")]
    pub exempted_principals: Vec<String>,
}
impl TenantPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
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
