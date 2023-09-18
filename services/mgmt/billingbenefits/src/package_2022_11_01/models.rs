#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Properties specific to applied scope type. Not required if not applicable."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppliedScopeProperties {
    #[doc = "Tenant ID where the benefit is applied."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<TenantId>,
    #[doc = "Fully-qualified identifier of the management group where the benefit must be applied."]
    #[serde(rename = "managementGroupId", default, skip_serializing_if = "Option::is_none")]
    pub management_group_id: Option<ManagementGroupId>,
    #[doc = "Fully-qualified identifier of the subscription."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<SubscriptionId>,
    #[doc = "Fully-qualified identifier of the resource group."]
    #[serde(rename = "resourceGroupId", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_id: Option<ResourceGroupId>,
    #[doc = "Display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl AppliedScopeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of the Applied Scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AppliedScopeType")]
pub enum AppliedScopeType {
    Single,
    Shared,
    ManagementGroup,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AppliedScopeType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AppliedScopeType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AppliedScopeType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Single => serializer.serialize_unit_variant("AppliedScopeType", 0u32, "Single"),
            Self::Shared => serializer.serialize_unit_variant("AppliedScopeType", 1u32, "Shared"),
            Self::ManagementGroup => serializer.serialize_unit_variant("AppliedScopeType", 2u32, "ManagementGroup"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type BenefitStartTime = time::OffsetDateTime;
pub type BillingAccountId = String;
#[doc = "billing information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingInformation {
    #[serde(rename = "billingCurrencyTotalPaidAmount", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_total_paid_amount: Option<Price>,
    #[serde(rename = "billingCurrencyProratedAmount", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_prorated_amount: Option<Price>,
    #[serde(
        rename = "billingCurrencyRemainingCommitmentAmount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub billing_currency_remaining_commitment_amount: Option<Price>,
}
impl BillingInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the billing plan in ISO 8601 format. Required only for monthly billing plans."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BillingPlan")]
pub enum BillingPlan {
    #[serde(rename = "P1M")]
    P1m,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BillingPlan {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BillingPlan {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BillingPlan {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::P1m => serializer.serialize_unit_variant("BillingPlan", 0u32, "P1M"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Information describing the type of billing plan for this savings plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingPlanInformation {
    #[serde(rename = "pricingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<Price>,
    #[doc = "Date when the billing plan has started."]
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[doc = "For recurring billing plans, indicates the date when next payment will be processed. Null when total is paid off."]
    #[serde(rename = "nextPaymentDueDate", default, skip_serializing_if = "Option::is_none")]
    pub next_payment_due_date: Option<String>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub transactions: Vec<PaymentDetail>,
}
impl BillingPlanInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type BillingProfileId = String;
pub type BillingScopeId = String;
#[doc = "Commitment towards the benefit."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Commitment {
    #[serde(flatten)]
    pub price: Price,
    #[doc = "Commitment grain."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grain: Option<commitment::Grain>,
}
impl Commitment {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod commitment {
    use super::*;
    #[doc = "Commitment grain."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Grain")]
    pub enum Grain {
        Hourly,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Grain {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Grain {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Grain {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Hourly => serializer.serialize_unit_variant("Grain", 0u32, "Hourly"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type CustomerId = String;
pub type DisplayName = String;
pub type EffectiveDateTime = time::OffsetDateTime;
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
pub type ExpiryDateTime = time::OffsetDateTime;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedStatusInfo {
    #[doc = "Status code providing additional information."]
    #[serde(rename = "statusCode", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[doc = "The message giving detailed information about the status code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ExtendedStatusInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Turning this on will apply the reservation discount to other VMs in the same VM size group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "InstanceFlexibility")]
pub enum InstanceFlexibility {
    On,
    Off,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for InstanceFlexibility {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for InstanceFlexibility {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for InstanceFlexibility {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::On => serializer.serialize_unit_variant("InstanceFlexibility", 0u32, "On"),
            Self::Off => serializer.serialize_unit_variant("InstanceFlexibility", 1u32, "Off"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type ManagementGroupId = String;
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
#[doc = "Required if status == failed or status == canceled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResultError {
    #[doc = "Required if status == failed or status == cancelled. If status == failed, provide an invariant error code used for error troubleshooting, aggregation, and analysis."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Required if status == failed. Localized. If status == failed, provide an actionable error message indicating what error occurred, and what the user can do to address the issue."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl OperationResultError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about payment related to a savings plan order."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaymentDetail {
    #[doc = "Date when the payment needs to be done."]
    #[serde(rename = "dueDate", default, skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[doc = "Date when the transaction is completed. Is null when it is scheduled."]
    #[serde(rename = "paymentDate", default, skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<String>,
    #[serde(rename = "pricingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<Price>,
    #[serde(rename = "billingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_total: Option<Price>,
    #[doc = "Describes whether the payment is completed, failed, cancelled or scheduled in the future."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PaymentStatus>,
    #[serde(rename = "extendedStatusInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_status_info: Option<ExtendedStatusInfo>,
    #[doc = "Billing account"]
    #[serde(rename = "billingAccount", default, skip_serializing_if = "Option::is_none")]
    pub billing_account: Option<String>,
}
impl PaymentDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes whether the payment is completed, failed, cancelled or scheduled in the future."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PaymentStatus")]
pub enum PaymentStatus {
    Succeeded,
    Failed,
    Scheduled,
    Cancelled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PaymentStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PaymentStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PaymentStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("PaymentStatus", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("PaymentStatus", 1u32, "Failed"),
            Self::Scheduled => serializer.serialize_unit_variant("PaymentStatus", 2u32, "Scheduled"),
            Self::Cancelled => serializer.serialize_unit_variant("PaymentStatus", 3u32, "Cancelled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Price {
    #[doc = "The ISO 4217 3-letter currency code for the currency used by this purchase record."]
    #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
}
impl Price {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents either billing plan or savings plan term in ISO 8601 format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PricingCurrencyDuration")]
pub enum PricingCurrencyDuration {
    #[serde(rename = "P1M")]
    P1m,
    #[serde(rename = "P1Y")]
    P1y,
    #[serde(rename = "P3Y")]
    P3y,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PricingCurrencyDuration {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PricingCurrencyDuration {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PricingCurrencyDuration {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::P1m => serializer.serialize_unit_variant("PricingCurrencyDuration", 0u32, "P1M"),
            Self::P1y => serializer.serialize_unit_variant("PricingCurrencyDuration", 1u32, "P1Y"),
            Self::P3y => serializer.serialize_unit_variant("PricingCurrencyDuration", 2u32, "P3Y"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PricingCurrencyTotal {
    #[serde(flatten)]
    pub price: Price,
    #[doc = "Represents either billing plan or savings plan term in ISO 8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<PricingCurrencyDuration>,
}
impl PricingCurrencyTotal {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provisioning state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Creating,
    PendingBilling,
    ConfirmedBilling,
    Created,
    Succeeded,
    Cancelled,
    Expired,
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
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
            Self::PendingBilling => serializer.serialize_unit_variant("ProvisioningState", 1u32, "PendingBilling"),
            Self::ConfirmedBilling => serializer.serialize_unit_variant("ProvisioningState", 2u32, "ConfirmedBilling"),
            Self::Created => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Created"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Succeeded"),
            Self::Cancelled => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Cancelled"),
            Self::Expired => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Expired"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type PurchaseDateTime = time::OffsetDateTime;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PurchaseRequest {
    #[doc = "The SKU to be applied for this resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PurchaseRequestProperties>,
}
impl PurchaseRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PurchaseRequestProperties {
    #[doc = "Friendly name of the savings plan"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Subscription that will be charged for purchasing the benefit"]
    #[serde(rename = "billingScopeId", default, skip_serializing_if = "Option::is_none")]
    pub billing_scope_id: Option<BillingScopeId>,
    #[doc = "Represent benefit term in ISO 8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<Term>,
    #[doc = "Represents the billing plan in ISO 8601 format. Required only for monthly billing plans."]
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<BillingPlan>,
    #[doc = "Type of the Applied Scope."]
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[doc = "Commitment towards the benefit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[doc = "DateTime of the savings plan starts providing benefit from."]
    #[serde(rename = "effectiveDateTime", default, skip_serializing_if = "Option::is_none")]
    pub effective_date_time: Option<EffectiveDateTime>,
    #[doc = "Setting this to true will automatically purchase a new benefit on the expiration date time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[doc = "Properties specific to applied scope type. Not required if not applicable."]
    #[serde(rename = "appliedScopeProperties", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_properties: Option<AppliedScopeProperties>,
}
impl PurchaseRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type Renew = bool;
pub type RenewDestination = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RenewProperties {
    #[serde(rename = "purchaseProperties", default, skip_serializing_if = "Option::is_none")]
    pub purchase_properties: Option<PurchaseRequest>,
}
impl RenewProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type RenewSource = String;
#[doc = "Reservation order alias"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationOrderAliasRequest {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The SKU to be applied for this resource"]
    pub sku: Sku,
    #[doc = "The Azure Region where the reservation benefits are applied to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Reservation properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationOrderAliasRequestProperties>,
}
impl ReservationOrderAliasRequest {
    pub fn new(sku: Sku) -> Self {
        Self {
            resource: Resource::default(),
            sku,
            location: None,
            properties: None,
        }
    }
}
#[doc = "Reservation properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationOrderAliasRequestProperties {
    #[doc = "Display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<DisplayName>,
    #[doc = "Subscription that will be charged for purchasing the benefit"]
    #[serde(rename = "billingScopeId", default, skip_serializing_if = "Option::is_none")]
    pub billing_scope_id: Option<BillingScopeId>,
    #[doc = "Represent benefit term in ISO 8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<Term>,
    #[doc = "Represents the billing plan in ISO 8601 format. Required only for monthly billing plans."]
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<BillingPlan>,
    #[doc = "Type of the Applied Scope."]
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[doc = "Properties specific to applied scope type. Not required if not applicable."]
    #[serde(rename = "appliedScopeProperties", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_properties: Option<AppliedScopeProperties>,
    #[doc = "Total Quantity of the SKUs purchased in the Reservation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[doc = "Setting this to true will automatically purchase a new benefit on the expiration date time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[doc = "The type of the resource that is being reserved."]
    #[serde(rename = "reservedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub reserved_resource_type: Option<ReservedResourceType>,
    #[doc = "This is the date-time when the Azure Hybrid Benefit needs to be reviewed."]
    #[serde(rename = "reviewDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub review_date_time: Option<time::OffsetDateTime>,
    #[doc = "Properties specific to each reserved resource type. Not required if not applicable."]
    #[serde(rename = "reservedResourceProperties", default, skip_serializing_if = "Option::is_none")]
    pub reserved_resource_properties: Option<reservation_order_alias_request_properties::ReservedResourceProperties>,
}
impl ReservationOrderAliasRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod reservation_order_alias_request_properties {
    use super::*;
    #[doc = "Properties specific to each reserved resource type. Not required if not applicable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ReservedResourceProperties {
        #[doc = "Turning this on will apply the reservation discount to other VMs in the same VM size group."]
        #[serde(rename = "instanceFlexibility", default, skip_serializing_if = "Option::is_none")]
        pub instance_flexibility: Option<InstanceFlexibility>,
    }
    impl ReservedResourceProperties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Reservation order alias"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationOrderAliasResponse {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The SKU to be applied for this resource"]
    pub sku: Sku,
    #[doc = "The Azure Region where the reserved resource lives."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Reservation properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationOrderAliasResponseProperties>,
}
impl ReservationOrderAliasResponse {
    pub fn new(sku: Sku) -> Self {
        Self {
            resource: Resource::default(),
            sku,
            location: None,
            properties: None,
        }
    }
}
#[doc = "Reservation properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationOrderAliasResponseProperties {
    #[doc = "Display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<DisplayName>,
    #[doc = "Identifier of the reservation order created"]
    #[serde(rename = "reservationOrderId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order_id: Option<String>,
    #[doc = "Provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Subscription that will be charged for purchasing the benefit"]
    #[serde(rename = "billingScopeId", default, skip_serializing_if = "Option::is_none")]
    pub billing_scope_id: Option<BillingScopeId>,
    #[doc = "Represent benefit term in ISO 8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<Term>,
    #[doc = "Represents the billing plan in ISO 8601 format. Required only for monthly billing plans."]
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<BillingPlan>,
    #[doc = "Type of the Applied Scope."]
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[doc = "Properties specific to applied scope type. Not required if not applicable."]
    #[serde(rename = "appliedScopeProperties", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_properties: Option<AppliedScopeProperties>,
    #[doc = "Total Quantity of the SKUs purchased in the Reservation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[doc = "Setting this to true will automatically purchase a new benefit on the expiration date time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[doc = "The type of the resource that is being reserved."]
    #[serde(rename = "reservedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub reserved_resource_type: Option<ReservedResourceType>,
    #[doc = "This is the date-time when the Reservation needs to be reviewed."]
    #[serde(rename = "reviewDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub review_date_time: Option<time::OffsetDateTime>,
    #[doc = "Properties specific to each reserved resource type. Not required if not applicable."]
    #[serde(rename = "reservedResourceProperties", default, skip_serializing_if = "Option::is_none")]
    pub reserved_resource_properties: Option<reservation_order_alias_response_properties::ReservedResourceProperties>,
}
impl ReservationOrderAliasResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod reservation_order_alias_response_properties {
    use super::*;
    #[doc = "Properties specific to each reserved resource type. Not required if not applicable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ReservedResourceProperties {
        #[doc = "Turning this on will apply the reservation discount to other VMs in the same VM size group."]
        #[serde(rename = "instanceFlexibility", default, skip_serializing_if = "Option::is_none")]
        pub instance_flexibility: Option<InstanceFlexibility>,
    }
    impl ReservedResourceProperties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The type of the resource that is being reserved."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReservedResourceType")]
pub enum ReservedResourceType {
    VirtualMachines,
    SqlDatabases,
    SuseLinux,
    CosmosDb,
    RedHat,
    SqlDataWarehouse,
    VMwareCloudSimple,
    RedHatOsa,
    Databricks,
    AppService,
    ManagedDisk,
    BlockBlob,
    RedisCache,
    AzureDataExplorer,
    MySql,
    MariaDb,
    PostgreSql,
    DedicatedHost,
    SapHana,
    SqlAzureHybridBenefit,
    #[serde(rename = "AVS")]
    Avs,
    DataFactory,
    NetAppStorage,
    AzureFiles,
    SqlEdge,
    VirtualMachineSoftware,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReservedResourceType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReservedResourceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReservedResourceType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::VirtualMachines => serializer.serialize_unit_variant("ReservedResourceType", 0u32, "VirtualMachines"),
            Self::SqlDatabases => serializer.serialize_unit_variant("ReservedResourceType", 1u32, "SqlDatabases"),
            Self::SuseLinux => serializer.serialize_unit_variant("ReservedResourceType", 2u32, "SuseLinux"),
            Self::CosmosDb => serializer.serialize_unit_variant("ReservedResourceType", 3u32, "CosmosDb"),
            Self::RedHat => serializer.serialize_unit_variant("ReservedResourceType", 4u32, "RedHat"),
            Self::SqlDataWarehouse => serializer.serialize_unit_variant("ReservedResourceType", 5u32, "SqlDataWarehouse"),
            Self::VMwareCloudSimple => serializer.serialize_unit_variant("ReservedResourceType", 6u32, "VMwareCloudSimple"),
            Self::RedHatOsa => serializer.serialize_unit_variant("ReservedResourceType", 7u32, "RedHatOsa"),
            Self::Databricks => serializer.serialize_unit_variant("ReservedResourceType", 8u32, "Databricks"),
            Self::AppService => serializer.serialize_unit_variant("ReservedResourceType", 9u32, "AppService"),
            Self::ManagedDisk => serializer.serialize_unit_variant("ReservedResourceType", 10u32, "ManagedDisk"),
            Self::BlockBlob => serializer.serialize_unit_variant("ReservedResourceType", 11u32, "BlockBlob"),
            Self::RedisCache => serializer.serialize_unit_variant("ReservedResourceType", 12u32, "RedisCache"),
            Self::AzureDataExplorer => serializer.serialize_unit_variant("ReservedResourceType", 13u32, "AzureDataExplorer"),
            Self::MySql => serializer.serialize_unit_variant("ReservedResourceType", 14u32, "MySql"),
            Self::MariaDb => serializer.serialize_unit_variant("ReservedResourceType", 15u32, "MariaDb"),
            Self::PostgreSql => serializer.serialize_unit_variant("ReservedResourceType", 16u32, "PostgreSql"),
            Self::DedicatedHost => serializer.serialize_unit_variant("ReservedResourceType", 17u32, "DedicatedHost"),
            Self::SapHana => serializer.serialize_unit_variant("ReservedResourceType", 18u32, "SapHana"),
            Self::SqlAzureHybridBenefit => serializer.serialize_unit_variant("ReservedResourceType", 19u32, "SqlAzureHybridBenefit"),
            Self::Avs => serializer.serialize_unit_variant("ReservedResourceType", 20u32, "AVS"),
            Self::DataFactory => serializer.serialize_unit_variant("ReservedResourceType", 21u32, "DataFactory"),
            Self::NetAppStorage => serializer.serialize_unit_variant("ReservedResourceType", 22u32, "NetAppStorage"),
            Self::AzureFiles => serializer.serialize_unit_variant("ReservedResourceType", 23u32, "AzureFiles"),
            Self::SqlEdge => serializer.serialize_unit_variant("ReservedResourceType", 24u32, "SqlEdge"),
            Self::VirtualMachineSoftware => serializer.serialize_unit_variant("ReservedResourceType", 25u32, "VirtualMachineSoftware"),
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
pub type ResourceGroupId = String;
#[doc = "Role assignment entity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentEntity {
    #[doc = "Role assignment entity id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Role assignment entity name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Role assignment entity properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RoleAssignmentEntityProperties>,
}
impl RoleAssignmentEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Role assignment entity properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleAssignmentEntityProperties {
    #[doc = "Principal Id"]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Role definition id"]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "Scope of the role assignment entity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
impl RoleAssignmentEntityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type SavingsPlanId = String;
#[doc = "Savings plan"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavingsPlanModel {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The SKU to be applied for this resource"]
    pub sku: Sku,
    #[doc = "Savings plan properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SavingsPlanModelProperties>,
}
impl SavingsPlanModel {
    pub fn new(sku: Sku) -> Self {
        Self {
            resource: Resource::default(),
            sku,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanModelList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SavingsPlanModel>,
    #[doc = "Url to get the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SavingsPlanModelList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SavingsPlanModelList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanModelListResult {
    #[doc = "The list of savings plans."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SavingsPlanModel>,
    #[doc = "Url to get the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The roll out count summary of the savings plans"]
    #[serde(
        rename = "additionalProperties",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_properties: Vec<SavingsPlanSummary>,
}
impl azure_core::Continuable for SavingsPlanModelListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SavingsPlanModelListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Savings plan properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanModelProperties {
    #[doc = "Display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<DisplayName>,
    #[doc = "Provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The provisioning state of the savings plan for display, e.g. Succeeded"]
    #[serde(rename = "displayProvisioningState", default, skip_serializing_if = "Option::is_none")]
    pub display_provisioning_state: Option<String>,
    #[doc = "Subscription that will be charged for purchasing the benefit"]
    #[serde(rename = "billingScopeId", default, skip_serializing_if = "Option::is_none")]
    pub billing_scope_id: Option<BillingScopeId>,
    #[doc = "Fully-qualified identifier of the billing profile where the savings plan is applied. Present only for Field-led or Customer-led customers."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<BillingProfileId>,
    #[doc = "Fully-qualified identifier of the customer where the savings plan is applied. Present only for Partner-led customers."]
    #[serde(rename = "customerId", default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<CustomerId>,
    #[doc = "Fully-qualified identifier of the billing account where the savings plan is applied. Present only for Enterprise Agreement customers."]
    #[serde(rename = "billingAccountId", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_id: Option<BillingAccountId>,
    #[doc = "Represent benefit term in ISO 8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<Term>,
    #[doc = "Represents the billing plan in ISO 8601 format. Required only for monthly billing plans."]
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<BillingPlan>,
    #[doc = "Type of the Applied Scope."]
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[doc = "The applied scope type of the savings plan for display, e.g. Shared"]
    #[serde(rename = "userFriendlyAppliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub user_friendly_applied_scope_type: Option<String>,
    #[doc = "Properties specific to applied scope type. Not required if not applicable."]
    #[serde(rename = "appliedScopeProperties", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_properties: Option<AppliedScopeProperties>,
    #[doc = "Commitment towards the benefit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[doc = "DateTime of the savings plan starts providing benefit from."]
    #[serde(rename = "effectiveDateTime", default, skip_serializing_if = "Option::is_none")]
    pub effective_date_time: Option<EffectiveDateTime>,
    #[doc = "Expiry date time"]
    #[serde(rename = "expiryDateTime", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date_time: Option<ExpiryDateTime>,
    #[doc = "Date time when the savings plan was purchased"]
    #[serde(rename = "purchaseDateTime", default, skip_serializing_if = "Option::is_none")]
    pub purchase_date_time: Option<PurchaseDateTime>,
    #[doc = "This is the DateTime when the savings plan benefit started."]
    #[serde(rename = "benefitStartTime", default, skip_serializing_if = "Option::is_none")]
    pub benefit_start_time: Option<BenefitStartTime>,
    #[serde(rename = "extendedStatusInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_status_info: Option<ExtendedStatusInfo>,
    #[doc = "Setting this to true will automatically purchase a new benefit on the expiration date time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[doc = "Savings plan utilization"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub utilization: Option<Utilization>,
    #[doc = "SavingsPlan Id of the SavingsPlan from which this SavingsPlan is renewed."]
    #[serde(rename = "renewSource", default, skip_serializing_if = "Option::is_none")]
    pub renew_source: Option<RenewSource>,
    #[doc = "SavingsPlan Id of the SavingsPlan which is purchased because of renew."]
    #[serde(rename = "renewDestination", default, skip_serializing_if = "Option::is_none")]
    pub renew_destination: Option<RenewDestination>,
    #[serde(rename = "renewProperties", default, skip_serializing_if = "Option::is_none")]
    pub renew_properties: Option<RenewProperties>,
}
impl SavingsPlanModelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Savings plan order alias"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavingsPlanOrderAliasModel {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The SKU to be applied for this resource"]
    pub sku: Sku,
    #[doc = "Resource provider kind"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Savings plan properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SavingsPlanOrderAliasProperties>,
}
impl SavingsPlanOrderAliasModel {
    pub fn new(sku: Sku) -> Self {
        Self {
            resource: Resource::default(),
            sku,
            kind: None,
            properties: None,
        }
    }
}
#[doc = "Savings plan properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanOrderAliasProperties {
    #[doc = "Display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<DisplayName>,
    #[doc = "Identifier of the savings plan created"]
    #[serde(rename = "savingsPlanOrderId", default, skip_serializing_if = "Option::is_none")]
    pub savings_plan_order_id: Option<String>,
    #[doc = "Provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Subscription that will be charged for purchasing the benefit"]
    #[serde(rename = "billingScopeId", default, skip_serializing_if = "Option::is_none")]
    pub billing_scope_id: Option<BillingScopeId>,
    #[doc = "Represent benefit term in ISO 8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<Term>,
    #[doc = "Represents the billing plan in ISO 8601 format. Required only for monthly billing plans."]
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<BillingPlan>,
    #[doc = "Type of the Applied Scope."]
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[doc = "Properties specific to applied scope type. Not required if not applicable."]
    #[serde(rename = "appliedScopeProperties", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_properties: Option<AppliedScopeProperties>,
    #[doc = "Commitment towards the benefit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[doc = "Setting this to true will automatically purchase a new benefit on the expiration date time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
}
impl SavingsPlanOrderAliasProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Savings plan order"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SavingsPlanOrderModel {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The SKU to be applied for this resource"]
    pub sku: Sku,
    #[doc = "Savings plan order properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SavingsPlanOrderModelProperties>,
}
impl SavingsPlanOrderModel {
    pub fn new(sku: Sku) -> Self {
        Self {
            resource: Resource::default(),
            sku,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanOrderModelList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SavingsPlanOrderModel>,
    #[doc = "Url to get the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SavingsPlanOrderModelList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SavingsPlanOrderModelList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Savings plan order properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanOrderModelProperties {
    #[doc = "Display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<DisplayName>,
    #[doc = "Provisioning state"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Subscription that will be charged for purchasing the benefit"]
    #[serde(rename = "billingScopeId", default, skip_serializing_if = "Option::is_none")]
    pub billing_scope_id: Option<BillingScopeId>,
    #[doc = "Fully-qualified identifier of the billing profile where the savings plan is applied. Present only for Field-led or Customer-led customers."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<BillingProfileId>,
    #[doc = "Fully-qualified identifier of the customer where the savings plan is applied. Present only for Partner-led customers."]
    #[serde(rename = "customerId", default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<CustomerId>,
    #[doc = "Fully-qualified identifier of the billing account where the savings plan is applied. Present only for Enterprise Agreement customers."]
    #[serde(rename = "billingAccountId", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_id: Option<BillingAccountId>,
    #[doc = "Represent benefit term in ISO 8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<Term>,
    #[doc = "Represents the billing plan in ISO 8601 format. Required only for monthly billing plans."]
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<BillingPlan>,
    #[doc = "Expiry date time"]
    #[serde(rename = "expiryDateTime", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date_time: Option<ExpiryDateTime>,
    #[doc = "This is the DateTime when the savings plan benefit started."]
    #[serde(rename = "benefitStartTime", default, skip_serializing_if = "Option::is_none")]
    pub benefit_start_time: Option<BenefitStartTime>,
    #[doc = "Information describing the type of billing plan for this savings plan."]
    #[serde(rename = "planInformation", default, skip_serializing_if = "Option::is_none")]
    pub plan_information: Option<BillingPlanInformation>,
    #[serde(
        rename = "savingsPlans",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub savings_plans: Vec<SavingsPlanId>,
    #[serde(rename = "extendedStatusInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_status_info: Option<ExtendedStatusInfo>,
}
impl SavingsPlanOrderModelProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanPurchaseValidateRequest {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub benefits: Vec<SavingsPlanOrderAliasModel>,
}
impl SavingsPlanPurchaseValidateRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Savings plans list summary"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanSummary {
    #[doc = "This property has value 'summary'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The roll up count summary of savings plans in each state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<SavingsPlanSummaryCount>,
}
impl SavingsPlanSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The roll up count summary of savings plans in each state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanSummaryCount {
    #[doc = "The number of savings plans in Succeeded state"]
    #[serde(rename = "succeededCount", default, skip_serializing_if = "Option::is_none")]
    pub succeeded_count: Option<f64>,
    #[doc = "The number of savings plans in Failed state"]
    #[serde(rename = "failedCount", default, skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<f64>,
    #[doc = "The number of savings plans in Expiring state"]
    #[serde(rename = "expiringCount", default, skip_serializing_if = "Option::is_none")]
    pub expiring_count: Option<f64>,
    #[doc = "The number of savings plans in Expired state"]
    #[serde(rename = "expiredCount", default, skip_serializing_if = "Option::is_none")]
    pub expired_count: Option<f64>,
    #[doc = "The number of savings plans in Pending state"]
    #[serde(rename = "pendingCount", default, skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<f64>,
    #[doc = "The number of savings plans in Cancelled state"]
    #[serde(rename = "cancelledCount", default, skip_serializing_if = "Option::is_none")]
    pub cancelled_count: Option<f64>,
    #[doc = "The number of savings plans in Processing state"]
    #[serde(rename = "processingCount", default, skip_serializing_if = "Option::is_none")]
    pub processing_count: Option<f64>,
    #[doc = "The number of savings plans in No Benefit state"]
    #[serde(rename = "noBenefitCount", default, skip_serializing_if = "Option::is_none")]
    pub no_benefit_count: Option<f64>,
    #[doc = "The number of savings plans in Warning state"]
    #[serde(rename = "warningCount", default, skip_serializing_if = "Option::is_none")]
    pub warning_count: Option<f64>,
}
impl SavingsPlanSummaryCount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Savings plan patch request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanUpdateRequest {
    #[doc = "Savings plan patch request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SavingsPlanUpdateRequestProperties>,
}
impl SavingsPlanUpdateRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Savings plan patch request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanUpdateRequestProperties {
    #[doc = "Display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<DisplayName>,
    #[doc = "Type of the Applied Scope."]
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[doc = "Properties specific to applied scope type. Not required if not applicable."]
    #[serde(rename = "appliedScopeProperties", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_properties: Option<AppliedScopeProperties>,
    #[doc = "Setting this to true will automatically purchase a new benefit on the expiration date time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[serde(rename = "renewProperties", default, skip_serializing_if = "Option::is_none")]
    pub renew_properties: Option<RenewProperties>,
}
impl SavingsPlanUpdateRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanUpdateValidateRequest {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub benefits: Vec<SavingsPlanUpdateRequestProperties>,
}
impl SavingsPlanUpdateValidateRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Benefit scope response property"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanValidResponseProperty {
    #[doc = "Indicates if the provided input was valid"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
    #[doc = "Failure reason code if the provided input was invalid"]
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    #[doc = "Failure reason if the provided input was invalid"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl SavingsPlanValidResponseProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanValidateResponse {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub benefits: Vec<SavingsPlanValidResponseProperty>,
    #[doc = "Url to get the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SavingsPlanValidateResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SKU to be applied for this resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Sku {
    #[doc = "Name of the SKU to be applied"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl Sku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type SubscriptionId = String;
pub type TenantId = String;
#[doc = "Represent benefit term in ISO 8601 format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Term")]
pub enum Term {
    #[serde(rename = "P1Y")]
    P1y,
    #[serde(rename = "P3Y")]
    P3y,
    #[serde(rename = "P5Y")]
    P5y,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Term {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Term {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Term {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::P1y => serializer.serialize_unit_variant("Term", 0u32, "P1Y"),
            Self::P3y => serializer.serialize_unit_variant("Term", 1u32, "P3Y"),
            Self::P5y => serializer.serialize_unit_variant("Term", 2u32, "P5Y"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Savings plan utilization"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Utilization {
    #[doc = "The number of days trend for a savings plan"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trend: Option<String>,
    #[doc = "The array of aggregates of a savings plan's utilization"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub aggregates: Vec<UtilizationAggregates>,
}
impl Utilization {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The aggregate values of savings plan utilization"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UtilizationAggregates {
    #[doc = "The grain of the aggregate"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grain: Option<f64>,
    #[doc = "The grain unit of the aggregate"]
    #[serde(rename = "grainUnit", default, skip_serializing_if = "Option::is_none")]
    pub grain_unit: Option<String>,
    #[doc = "The aggregate value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[doc = "The aggregate value unit"]
    #[serde(rename = "valueUnit", default, skip_serializing_if = "Option::is_none")]
    pub value_unit: Option<String>,
}
impl UtilizationAggregates {
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
