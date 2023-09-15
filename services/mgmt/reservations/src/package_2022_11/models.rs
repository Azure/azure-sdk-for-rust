#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Paginated list of applied reservations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppliedReservationList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<String>,
    #[doc = "Url to get the next page of reservations"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl AppliedReservationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response for applied reservations api"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppliedReservations {
    #[doc = "Identifier of the applied reservations"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of resource. \"Microsoft.Capacity/AppliedReservations\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Properties for applied reservations returned"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AppliedReservationsProperties>,
}
impl AppliedReservations {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for applied reservations returned"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppliedReservationsProperties {
    #[doc = "Paginated list of applied reservations"]
    #[serde(rename = "reservationOrderIds", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order_ids: Option<AppliedReservationList>,
}
impl AppliedReservationsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties specific to applied scope type. Not required if not applicable. Required and need to provide tenantId and managementGroupId if AppliedScopeType is ManagementGroup"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppliedScopeProperties {
    #[doc = "Tenant ID where the savings plan should apply benefit."]
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
pub type AppliedScopes = Vec<String>;
#[doc = "The response of available scope api containing scopes and their eligibilities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableScopeProperties {
    #[doc = "The scopes checked by the available scope api."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionScopeProperties>,
}
impl AvailableScopeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Available scope"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableScopeRequest {
    #[doc = "Available scope request properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvailableScopeRequestProperties>,
}
impl AvailableScopeRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Available scope request properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableScopeRequestProperties {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scopes: Vec<String>,
}
impl AvailableScopeRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represent savings plan term in ISO 8601 format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BenefitTerm")]
pub enum BenefitTerm {
    #[serde(rename = "P1Y")]
    P1y,
    #[serde(rename = "P3Y")]
    P3y,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BenefitTerm {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BenefitTerm {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BenefitTerm {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::P1y => serializer.serialize_unit_variant("BenefitTerm", 0u32, "P1Y"),
            Self::P3y => serializer.serialize_unit_variant("BenefitTerm", 1u32, "P3Y"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "billing information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingInformation {
    #[doc = "Pricing information containing the amount and the currency code"]
    #[serde(rename = "billingCurrencyTotalPaidAmount", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_total_paid_amount: Option<Price>,
    #[doc = "Pricing information containing the amount and the currency code"]
    #[serde(rename = "billingCurrencyProratedAmount", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_prorated_amount: Option<Price>,
    #[doc = "Pricing information containing the amount and the currency code"]
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
pub type BillingScopeId = String;
#[doc = "CalculateExchange operation result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CalculateExchangeOperationResultResponse {
    #[doc = "It should match what is used to GET the operation result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "It must match the last segment of the id field, and will typically be a GUID / system generated value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<calculate_exchange_operation_result_response::Status>,
    #[doc = "CalculateExchange response properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CalculateExchangeResponseProperties>,
    #[doc = "Required if status == failed or status == canceled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<OperationResultError>,
}
impl CalculateExchangeOperationResultResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod calculate_exchange_operation_result_response {
    use super::*;
    #[doc = "Status of the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Succeeded,
        Failed,
        Cancelled,
        Pending,
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
                Self::Succeeded => serializer.serialize_unit_variant("Status", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 1u32, "Failed"),
                Self::Cancelled => serializer.serialize_unit_variant("Status", 2u32, "Cancelled"),
                Self::Pending => serializer.serialize_unit_variant("Status", 3u32, "Pending"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Calculate exchange request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CalculateExchangeRequest {
    #[doc = "Calculate exchange request properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CalculateExchangeRequestProperties>,
}
impl CalculateExchangeRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Calculate exchange request properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CalculateExchangeRequestProperties {
    #[doc = "List of reservations that are being purchased in this exchange."]
    #[serde(
        rename = "reservationsToPurchase",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reservations_to_purchase: Vec<PurchaseRequest>,
    #[doc = "List of savings plans that are being purchased in this exchange."]
    #[serde(
        rename = "savingsPlansToPurchase",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub savings_plans_to_purchase: Vec<SavingsPlanPurchaseRequest>,
    #[doc = "List of reservations that are being returned in this exchange."]
    #[serde(
        rename = "reservationsToExchange",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reservations_to_exchange: Vec<ReservationToReturn>,
}
impl CalculateExchangeRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "CalculateExchange response properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CalculateExchangeResponseProperties {
    #[doc = "Exchange session identifier"]
    #[serde(rename = "sessionId", default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[doc = "Pricing information containing the amount and the currency code"]
    #[serde(rename = "netPayable", default, skip_serializing_if = "Option::is_none")]
    pub net_payable: Option<Price>,
    #[doc = "Pricing information containing the amount and the currency code"]
    #[serde(rename = "refundsTotal", default, skip_serializing_if = "Option::is_none")]
    pub refunds_total: Option<Price>,
    #[doc = "Pricing information containing the amount and the currency code"]
    #[serde(rename = "purchasesTotal", default, skip_serializing_if = "Option::is_none")]
    pub purchases_total: Option<Price>,
    #[doc = "Details of the reservations being purchased"]
    #[serde(
        rename = "reservationsToPurchase",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reservations_to_purchase: Vec<ReservationToPurchaseCalculateExchange>,
    #[doc = "Details of the savings plans being purchased"]
    #[serde(
        rename = "savingsPlansToPurchase",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub savings_plans_to_purchase: Vec<SavingsPlanToPurchaseCalculateExchange>,
    #[doc = "Details of the reservations being returned"]
    #[serde(
        rename = "reservationsToExchange",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reservations_to_exchange: Vec<ReservationToExchange>,
    #[doc = "Exchange policy errors"]
    #[serde(rename = "policyResult", default, skip_serializing_if = "Option::is_none")]
    pub policy_result: Option<ExchangePolicyErrors>,
}
impl CalculateExchangeResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of calculate price for reservation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CalculatePriceResponse {
    #[doc = "Properties for calculate price response"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CalculatePriceResponseProperties>,
}
impl CalculatePriceResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for calculate price response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CalculatePriceResponseProperties {
    #[doc = "Currency and amount that customer will be charged in customer's local currency. Tax is not included."]
    #[serde(rename = "billingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_total: Option<calculate_price_response_properties::BillingCurrencyTotal>,
    #[doc = "Net total amount in pricing currency."]
    #[serde(rename = "netTotal", default, skip_serializing_if = "Option::is_none")]
    pub net_total: Option<f64>,
    #[doc = "Tax amount in pricing currency."]
    #[serde(rename = "taxTotal", default, skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<f64>,
    #[doc = "Total amount in pricing currency."]
    #[serde(rename = "grandTotal", default, skip_serializing_if = "Option::is_none")]
    pub grand_total: Option<f64>,
    #[doc = "Whether or not tax is included in grand total"]
    #[serde(rename = "isTaxIncluded", default, skip_serializing_if = "Option::is_none")]
    pub is_tax_included: Option<bool>,
    #[doc = "True if billing is managed by Microsoft Partner. Used only for CSP accounts."]
    #[serde(rename = "isBillingPartnerManaged", default, skip_serializing_if = "Option::is_none")]
    pub is_billing_partner_managed: Option<bool>,
    #[doc = "GUID that represents reservation order that can be placed after calculating price."]
    #[serde(rename = "reservationOrderId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order_id: Option<String>,
    #[doc = "Title of sku that is being purchased."]
    #[serde(rename = "skuTitle", default, skip_serializing_if = "Option::is_none")]
    pub sku_title: Option<String>,
    #[doc = "Description of sku that is being purchased."]
    #[serde(rename = "skuDescription", default, skip_serializing_if = "Option::is_none")]
    pub sku_description: Option<String>,
    #[doc = "Amount that Microsoft uses for record. Used during refund for calculating refund limit. Tax is not included."]
    #[serde(rename = "pricingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<calculate_price_response_properties::PricingCurrencyTotal>,
    #[serde(
        rename = "paymentSchedule",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub payment_schedule: Vec<PaymentDetail>,
}
impl CalculatePriceResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod calculate_price_response_properties {
    use super::*;
    #[doc = "Currency and amount that customer will be charged in customer's local currency. Tax is not included."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct BillingCurrencyTotal {
        #[doc = "The ISO 4217 3-letter currency code for the currency used by this purchase record."]
        #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
        pub currency_code: Option<String>,
        #[doc = "Amount in pricing currency. Tax is not included."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
    }
    impl BillingCurrencyTotal {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Amount that Microsoft uses for record. Used during refund for calculating refund limit. Tax is not included."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct PricingCurrencyTotal {
        #[doc = "The ISO 4217 3-letter currency code for the currency used by this purchase record."]
        #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
        pub currency_code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
    }
    impl PricingCurrencyTotal {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Request containing information needed for calculating refund."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CalculateRefundRequest {
    #[doc = "Fully qualified identifier of the reservation order being returned"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Properties needed for calculate refund including the scope and the reservation to be returned."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CalculateRefundRequestProperties>,
}
impl CalculateRefundRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties needed for calculate refund including the scope and the reservation to be returned."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CalculateRefundRequestProperties {
    #[doc = "The scope of the refund, e.g. Reservation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "Reservation to return"]
    #[serde(rename = "reservationToReturn", default, skip_serializing_if = "Option::is_none")]
    pub reservation_to_return: Option<ReservationToReturn>,
}
impl CalculateRefundRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of calculate refund containing refund information of reservation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CalculateRefundResponse {
    #[doc = "Fully qualified identifier of the reservation being returned"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The refund properties of reservation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RefundResponseProperties>,
}
impl CalculateRefundResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Product details of a type of resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Catalog {
    #[doc = "The type of resource the sku applies to."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The name of sku"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The billing plan options available for this sku."]
    #[serde(rename = "billingPlans", default, skip_serializing_if = "Option::is_none")]
    pub billing_plans: Option<serde_json::Value>,
    #[doc = "Available reservation terms for this resource"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub terms: Vec<ReservationTerm>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub locations: Vec<String>,
    #[serde(
        rename = "skuProperties",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sku_properties: Vec<SkuProperty>,
    #[doc = "Pricing information about the sku"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msrp: Option<catalog::Msrp>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub restrictions: Vec<SkuRestriction>,
    #[doc = "The tier of this sku"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "The size of this sku"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub capabilities: Vec<SkuCapability>,
}
impl Catalog {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod catalog {
    use super::*;
    #[doc = "Pricing information about the sku"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Msrp {
        #[doc = "Pricing information containing the amount and the currency code"]
        #[serde(rename = "p1Y", default, skip_serializing_if = "Option::is_none")]
        pub p1_y: Option<Price>,
        #[doc = "Pricing information containing the amount and the currency code"]
        #[serde(rename = "p3Y", default, skip_serializing_if = "Option::is_none")]
        pub p3_y: Option<Price>,
        #[doc = "Pricing information containing the amount and the currency code"]
        #[serde(rename = "p5Y", default, skip_serializing_if = "Option::is_none")]
        pub p5_y: Option<Price>,
    }
    impl Msrp {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The list of catalogs and pagination information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CatalogsResult {
    #[doc = "The list of catalogs."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Catalog>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The total amount of catalog items."]
    #[serde(rename = "totalItems", default, skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}
impl azure_core::Continuable for CatalogsResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CatalogsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request body for change directory of a reservation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangeDirectoryRequest {
    #[doc = "Tenant id GUID that reservation order is to be transferred to"]
    #[serde(rename = "destinationTenantId", default, skip_serializing_if = "Option::is_none")]
    pub destination_tenant_id: Option<String>,
}
impl ChangeDirectoryRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Change directory response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangeDirectoryResponse {
    #[doc = "Change directory result for reservation order or reservation"]
    #[serde(rename = "reservationOrder", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order: Option<ChangeDirectoryResult>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reservations: Vec<ChangeDirectoryResult>,
}
impl ChangeDirectoryResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Change directory result for reservation order or reservation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChangeDirectoryResult {
    #[doc = "Identifier of the reservation order or reservation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the reservation order or reservation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "True if change directory operation succeeded on this reservation order or reservation"]
    #[serde(rename = "isSucceeded", default, skip_serializing_if = "Option::is_none")]
    pub is_succeeded: Option<bool>,
    #[doc = "Error reason if operation failed. Null otherwise"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
impl ChangeDirectoryResult {
    pub fn new() -> Self {
        Self::default()
    }
}
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
#[doc = "Quota change requests information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateGenericQuotaRequestParameters {
    #[doc = "Quota change requests."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CurrentQuotaLimitBase>,
}
impl CreateGenericQuotaRequestParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Current quota limits."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CurrentQuotaLimit {
    #[doc = "Quota properties."]
    #[serde(rename = "quotaInformation", default, skip_serializing_if = "Option::is_none")]
    pub quota_information: Option<CurrentQuotaLimitBase>,
    #[doc = "Quota request status details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaRequestStatusDetails>,
}
impl CurrentQuotaLimit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quota properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CurrentQuotaLimitBase {
    #[doc = "The quota request ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the quota request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of resource. \"Microsoft.Capacity/ServiceLimits\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Quota properties for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaProperties>,
}
impl CurrentQuotaLimitBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represent the current display state of the reservation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DisplayProvisioningState")]
pub enum DisplayProvisioningState {
    Succeeded,
    Expiring,
    Expired,
    Pending,
    Processing,
    Cancelled,
    Failed,
    Warning,
    NoBenefit,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DisplayProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DisplayProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DisplayProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("DisplayProvisioningState", 0u32, "Succeeded"),
            Self::Expiring => serializer.serialize_unit_variant("DisplayProvisioningState", 1u32, "Expiring"),
            Self::Expired => serializer.serialize_unit_variant("DisplayProvisioningState", 2u32, "Expired"),
            Self::Pending => serializer.serialize_unit_variant("DisplayProvisioningState", 3u32, "Pending"),
            Self::Processing => serializer.serialize_unit_variant("DisplayProvisioningState", 4u32, "Processing"),
            Self::Cancelled => serializer.serialize_unit_variant("DisplayProvisioningState", 5u32, "Cancelled"),
            Self::Failed => serializer.serialize_unit_variant("DisplayProvisioningState", 6u32, "Failed"),
            Self::Warning => serializer.serialize_unit_variant("DisplayProvisioningState", 7u32, "Warning"),
            Self::NoBenefit => serializer.serialize_unit_variant("DisplayProvisioningState", 8u32, "NoBenefit"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Error information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "Extended error information including error code and error message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ExtendedErrorInfo>,
}
impl azure_core::Continuable for Error {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of the error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicates that the service is not able to process the incoming request. The reason is provided in the error message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The details of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
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
#[doc = "Error code describing the reason that service is not able to process the incoming request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ErrorResponseCode")]
pub enum ErrorResponseCode {
    NotSpecified,
    InternalServerError,
    ServerTimeout,
    AuthorizationFailed,
    BadRequest,
    ClientCertificateThumbprintNotSet,
    InvalidRequestContent,
    OperationFailed,
    HttpMethodNotSupported,
    InvalidRequestUri,
    MissingTenantId,
    InvalidTenantId,
    InvalidReservationOrderId,
    InvalidReservationId,
    ReservationIdNotInReservationOrder,
    ReservationOrderNotFound,
    InvalidSubscriptionId,
    InvalidAccessToken,
    InvalidLocationId,
    UnauthenticatedRequestsThrottled,
    InvalidHealthCheckType,
    Forbidden,
    BillingScopeIdCannotBeChanged,
    AppliedScopesNotAssociatedWithCommerceAccount,
    PatchValuesSameAsExisting,
    RoleAssignmentCreationFailed,
    ReservationOrderCreationFailed,
    ReservationOrderNotEnabled,
    CapacityUpdateScopesFailed,
    UnsupportedReservationTerm,
    ReservationOrderIdAlreadyExists,
    RiskCheckFailed,
    CreateQuoteFailed,
    ActivateQuoteFailed,
    NonsupportedAccountId,
    PaymentInstrumentNotFound,
    MissingAppliedScopesForSingle,
    NoValidReservationsToReRate,
    #[serde(rename = "ReRateOnlyAllowedForEA")]
    ReRateOnlyAllowedForEa,
    OperationCannotBePerformedInCurrentState,
    InvalidSingleAppliedScopesCount,
    InvalidFulfillmentRequestParameters,
    NotSupportedCountry,
    InvalidRefundQuantity,
    PurchaseError,
    BillingCustomerInputError,
    BillingPaymentInstrumentSoftError,
    BillingPaymentInstrumentHardError,
    BillingTransientError,
    BillingError,
    FulfillmentConfigurationError,
    FulfillmentOutOfStockError,
    FulfillmentTransientError,
    FulfillmentError,
    CalculatePriceFailed,
    AppliedScopesSameAsExisting,
    SelfServiceRefundNotSupported,
    RefundLimitExceeded,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ErrorResponseCode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ErrorResponseCode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ErrorResponseCode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("ErrorResponseCode", 0u32, "NotSpecified"),
            Self::InternalServerError => serializer.serialize_unit_variant("ErrorResponseCode", 1u32, "InternalServerError"),
            Self::ServerTimeout => serializer.serialize_unit_variant("ErrorResponseCode", 2u32, "ServerTimeout"),
            Self::AuthorizationFailed => serializer.serialize_unit_variant("ErrorResponseCode", 3u32, "AuthorizationFailed"),
            Self::BadRequest => serializer.serialize_unit_variant("ErrorResponseCode", 4u32, "BadRequest"),
            Self::ClientCertificateThumbprintNotSet => {
                serializer.serialize_unit_variant("ErrorResponseCode", 5u32, "ClientCertificateThumbprintNotSet")
            }
            Self::InvalidRequestContent => serializer.serialize_unit_variant("ErrorResponseCode", 6u32, "InvalidRequestContent"),
            Self::OperationFailed => serializer.serialize_unit_variant("ErrorResponseCode", 7u32, "OperationFailed"),
            Self::HttpMethodNotSupported => serializer.serialize_unit_variant("ErrorResponseCode", 8u32, "HttpMethodNotSupported"),
            Self::InvalidRequestUri => serializer.serialize_unit_variant("ErrorResponseCode", 9u32, "InvalidRequestUri"),
            Self::MissingTenantId => serializer.serialize_unit_variant("ErrorResponseCode", 10u32, "MissingTenantId"),
            Self::InvalidTenantId => serializer.serialize_unit_variant("ErrorResponseCode", 11u32, "InvalidTenantId"),
            Self::InvalidReservationOrderId => serializer.serialize_unit_variant("ErrorResponseCode", 12u32, "InvalidReservationOrderId"),
            Self::InvalidReservationId => serializer.serialize_unit_variant("ErrorResponseCode", 13u32, "InvalidReservationId"),
            Self::ReservationIdNotInReservationOrder => {
                serializer.serialize_unit_variant("ErrorResponseCode", 14u32, "ReservationIdNotInReservationOrder")
            }
            Self::ReservationOrderNotFound => serializer.serialize_unit_variant("ErrorResponseCode", 15u32, "ReservationOrderNotFound"),
            Self::InvalidSubscriptionId => serializer.serialize_unit_variant("ErrorResponseCode", 16u32, "InvalidSubscriptionId"),
            Self::InvalidAccessToken => serializer.serialize_unit_variant("ErrorResponseCode", 17u32, "InvalidAccessToken"),
            Self::InvalidLocationId => serializer.serialize_unit_variant("ErrorResponseCode", 18u32, "InvalidLocationId"),
            Self::UnauthenticatedRequestsThrottled => {
                serializer.serialize_unit_variant("ErrorResponseCode", 19u32, "UnauthenticatedRequestsThrottled")
            }
            Self::InvalidHealthCheckType => serializer.serialize_unit_variant("ErrorResponseCode", 20u32, "InvalidHealthCheckType"),
            Self::Forbidden => serializer.serialize_unit_variant("ErrorResponseCode", 21u32, "Forbidden"),
            Self::BillingScopeIdCannotBeChanged => {
                serializer.serialize_unit_variant("ErrorResponseCode", 22u32, "BillingScopeIdCannotBeChanged")
            }
            Self::AppliedScopesNotAssociatedWithCommerceAccount => {
                serializer.serialize_unit_variant("ErrorResponseCode", 23u32, "AppliedScopesNotAssociatedWithCommerceAccount")
            }
            Self::PatchValuesSameAsExisting => serializer.serialize_unit_variant("ErrorResponseCode", 24u32, "PatchValuesSameAsExisting"),
            Self::RoleAssignmentCreationFailed => {
                serializer.serialize_unit_variant("ErrorResponseCode", 25u32, "RoleAssignmentCreationFailed")
            }
            Self::ReservationOrderCreationFailed => {
                serializer.serialize_unit_variant("ErrorResponseCode", 26u32, "ReservationOrderCreationFailed")
            }
            Self::ReservationOrderNotEnabled => serializer.serialize_unit_variant("ErrorResponseCode", 27u32, "ReservationOrderNotEnabled"),
            Self::CapacityUpdateScopesFailed => serializer.serialize_unit_variant("ErrorResponseCode", 28u32, "CapacityUpdateScopesFailed"),
            Self::UnsupportedReservationTerm => serializer.serialize_unit_variant("ErrorResponseCode", 29u32, "UnsupportedReservationTerm"),
            Self::ReservationOrderIdAlreadyExists => {
                serializer.serialize_unit_variant("ErrorResponseCode", 30u32, "ReservationOrderIdAlreadyExists")
            }
            Self::RiskCheckFailed => serializer.serialize_unit_variant("ErrorResponseCode", 31u32, "RiskCheckFailed"),
            Self::CreateQuoteFailed => serializer.serialize_unit_variant("ErrorResponseCode", 32u32, "CreateQuoteFailed"),
            Self::ActivateQuoteFailed => serializer.serialize_unit_variant("ErrorResponseCode", 33u32, "ActivateQuoteFailed"),
            Self::NonsupportedAccountId => serializer.serialize_unit_variant("ErrorResponseCode", 34u32, "NonsupportedAccountId"),
            Self::PaymentInstrumentNotFound => serializer.serialize_unit_variant("ErrorResponseCode", 35u32, "PaymentInstrumentNotFound"),
            Self::MissingAppliedScopesForSingle => {
                serializer.serialize_unit_variant("ErrorResponseCode", 36u32, "MissingAppliedScopesForSingle")
            }
            Self::NoValidReservationsToReRate => {
                serializer.serialize_unit_variant("ErrorResponseCode", 37u32, "NoValidReservationsToReRate")
            }
            Self::ReRateOnlyAllowedForEa => serializer.serialize_unit_variant("ErrorResponseCode", 38u32, "ReRateOnlyAllowedForEA"),
            Self::OperationCannotBePerformedInCurrentState => {
                serializer.serialize_unit_variant("ErrorResponseCode", 39u32, "OperationCannotBePerformedInCurrentState")
            }
            Self::InvalidSingleAppliedScopesCount => {
                serializer.serialize_unit_variant("ErrorResponseCode", 40u32, "InvalidSingleAppliedScopesCount")
            }
            Self::InvalidFulfillmentRequestParameters => {
                serializer.serialize_unit_variant("ErrorResponseCode", 41u32, "InvalidFulfillmentRequestParameters")
            }
            Self::NotSupportedCountry => serializer.serialize_unit_variant("ErrorResponseCode", 42u32, "NotSupportedCountry"),
            Self::InvalidRefundQuantity => serializer.serialize_unit_variant("ErrorResponseCode", 43u32, "InvalidRefundQuantity"),
            Self::PurchaseError => serializer.serialize_unit_variant("ErrorResponseCode", 44u32, "PurchaseError"),
            Self::BillingCustomerInputError => serializer.serialize_unit_variant("ErrorResponseCode", 45u32, "BillingCustomerInputError"),
            Self::BillingPaymentInstrumentSoftError => {
                serializer.serialize_unit_variant("ErrorResponseCode", 46u32, "BillingPaymentInstrumentSoftError")
            }
            Self::BillingPaymentInstrumentHardError => {
                serializer.serialize_unit_variant("ErrorResponseCode", 47u32, "BillingPaymentInstrumentHardError")
            }
            Self::BillingTransientError => serializer.serialize_unit_variant("ErrorResponseCode", 48u32, "BillingTransientError"),
            Self::BillingError => serializer.serialize_unit_variant("ErrorResponseCode", 49u32, "BillingError"),
            Self::FulfillmentConfigurationError => {
                serializer.serialize_unit_variant("ErrorResponseCode", 50u32, "FulfillmentConfigurationError")
            }
            Self::FulfillmentOutOfStockError => serializer.serialize_unit_variant("ErrorResponseCode", 51u32, "FulfillmentOutOfStockError"),
            Self::FulfillmentTransientError => serializer.serialize_unit_variant("ErrorResponseCode", 52u32, "FulfillmentTransientError"),
            Self::FulfillmentError => serializer.serialize_unit_variant("ErrorResponseCode", 53u32, "FulfillmentError"),
            Self::CalculatePriceFailed => serializer.serialize_unit_variant("ErrorResponseCode", 54u32, "CalculatePriceFailed"),
            Self::AppliedScopesSameAsExisting => {
                serializer.serialize_unit_variant("ErrorResponseCode", 55u32, "AppliedScopesSameAsExisting")
            }
            Self::SelfServiceRefundNotSupported => {
                serializer.serialize_unit_variant("ErrorResponseCode", 56u32, "SelfServiceRefundNotSupported")
            }
            Self::RefundLimitExceeded => serializer.serialize_unit_variant("ErrorResponseCode", 57u32, "RefundLimitExceeded"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The API error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExceptionResponse {
    #[doc = "The API error details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ServiceError>,
}
impl azure_core::Continuable for ExceptionResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ExceptionResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Exchange operation result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExchangeOperationResultResponse {
    #[doc = "It should match what is used to GET the operation result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "It must match the last segment of the id field, and will typically be a GUID / system generated value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<exchange_operation_result_response::Status>,
    #[doc = "Exchange response properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExchangeResponseProperties>,
    #[doc = "Required if status == failed or status == canceled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<OperationResultError>,
}
impl ExchangeOperationResultResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod exchange_operation_result_response {
    use super::*;
    #[doc = "Status of the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Succeeded,
        Failed,
        Cancelled,
        PendingRefunds,
        PendingPurchases,
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
                Self::Succeeded => serializer.serialize_unit_variant("Status", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 1u32, "Failed"),
                Self::Cancelled => serializer.serialize_unit_variant("Status", 2u32, "Cancelled"),
                Self::PendingRefunds => serializer.serialize_unit_variant("Status", 3u32, "PendingRefunds"),
                Self::PendingPurchases => serializer.serialize_unit_variant("Status", 4u32, "PendingPurchases"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "error details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExchangePolicyError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ExchangePolicyError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Exchange policy errors"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExchangePolicyErrors {
    #[doc = "Exchange Policy errors"]
    #[serde(
        rename = "policyErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub policy_errors: Vec<ExchangePolicyError>,
}
impl ExchangePolicyErrors {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Exchange request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExchangeRequest {
    #[doc = "Exchange request properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ExchangeRequestProperties>,
}
impl ExchangeRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Exchange request properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExchangeRequestProperties {
    #[doc = "SessionId that was returned by CalculateExchange API."]
    #[serde(rename = "sessionId", default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}
impl ExchangeRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Exchange response properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExchangeResponseProperties {
    #[doc = "Exchange session identifier"]
    #[serde(rename = "sessionId", default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[doc = "Pricing information containing the amount and the currency code"]
    #[serde(rename = "netPayable", default, skip_serializing_if = "Option::is_none")]
    pub net_payable: Option<Price>,
    #[doc = "Pricing information containing the amount and the currency code"]
    #[serde(rename = "refundsTotal", default, skip_serializing_if = "Option::is_none")]
    pub refunds_total: Option<Price>,
    #[doc = "Pricing information containing the amount and the currency code"]
    #[serde(rename = "purchasesTotal", default, skip_serializing_if = "Option::is_none")]
    pub purchases_total: Option<Price>,
    #[doc = "Details of the reservations being purchased"]
    #[serde(
        rename = "reservationsToPurchase",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reservations_to_purchase: Vec<ReservationToPurchaseExchange>,
    #[doc = "Details of the savings plans being purchased"]
    #[serde(
        rename = "savingsPlansToPurchase",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub savings_plans_to_purchase: Vec<SavingsPlanToPurchaseExchange>,
    #[doc = "Details of the reservations being returned"]
    #[serde(
        rename = "reservationsToExchange",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reservations_to_exchange: Vec<ReservationToReturnForExchange>,
    #[doc = "Exchange policy errors"]
    #[serde(rename = "policyResult", default, skip_serializing_if = "Option::is_none")]
    pub policy_result: Option<ExchangePolicyErrors>,
}
impl ExchangeResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Extended error information including error code and error message"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedErrorInfo {
    #[doc = "Error code describing the reason that service is not able to process the incoming request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<ErrorResponseCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ExtendedErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedStatusInfo {
    #[serde(rename = "statusCode", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<ReservationStatusCode>,
    #[doc = "The message giving detailed information about the status code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ExtendedStatusInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Turning this on will apply the reservation discount to other VMs in the same VM size group. Only specify for VirtualMachines reserved resource type."]
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
#[doc = "Location in which the Resources needs to be reserved. It cannot be changed after the resource has been created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Location")]
pub enum Location {
    #[serde(rename = "westus")]
    Westus,
    #[serde(rename = "eastus")]
    Eastus,
    #[serde(rename = "eastus2")]
    Eastus2,
    #[serde(rename = "northcentralus")]
    Northcentralus,
    #[serde(rename = "westus2")]
    Westus2,
    #[serde(rename = "southcentralus")]
    Southcentralus,
    #[serde(rename = "centralus")]
    Centralus,
    #[serde(rename = "westeurope")]
    Westeurope,
    #[serde(rename = "northeurope")]
    Northeurope,
    #[serde(rename = "eastasia")]
    Eastasia,
    #[serde(rename = "southeastasia")]
    Southeastasia,
    #[serde(rename = "japaneast")]
    Japaneast,
    #[serde(rename = "japanwest")]
    Japanwest,
    #[serde(rename = "brazilsouth")]
    Brazilsouth,
    #[serde(rename = "australiaeast")]
    Australiaeast,
    #[serde(rename = "australiasoutheast")]
    Australiasoutheast,
    #[serde(rename = "southindia")]
    Southindia,
    #[serde(rename = "westindia")]
    Westindia,
    #[serde(rename = "centralindia")]
    Centralindia,
    #[serde(rename = "canadacentral")]
    Canadacentral,
    #[serde(rename = "canadaeast")]
    Canadaeast,
    #[serde(rename = "uksouth")]
    Uksouth,
    #[serde(rename = "westcentralus")]
    Westcentralus,
    #[serde(rename = "ukwest")]
    Ukwest,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Location {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Location {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Location {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Westus => serializer.serialize_unit_variant("Location", 0u32, "westus"),
            Self::Eastus => serializer.serialize_unit_variant("Location", 1u32, "eastus"),
            Self::Eastus2 => serializer.serialize_unit_variant("Location", 2u32, "eastus2"),
            Self::Northcentralus => serializer.serialize_unit_variant("Location", 3u32, "northcentralus"),
            Self::Westus2 => serializer.serialize_unit_variant("Location", 4u32, "westus2"),
            Self::Southcentralus => serializer.serialize_unit_variant("Location", 5u32, "southcentralus"),
            Self::Centralus => serializer.serialize_unit_variant("Location", 6u32, "centralus"),
            Self::Westeurope => serializer.serialize_unit_variant("Location", 7u32, "westeurope"),
            Self::Northeurope => serializer.serialize_unit_variant("Location", 8u32, "northeurope"),
            Self::Eastasia => serializer.serialize_unit_variant("Location", 9u32, "eastasia"),
            Self::Southeastasia => serializer.serialize_unit_variant("Location", 10u32, "southeastasia"),
            Self::Japaneast => serializer.serialize_unit_variant("Location", 11u32, "japaneast"),
            Self::Japanwest => serializer.serialize_unit_variant("Location", 12u32, "japanwest"),
            Self::Brazilsouth => serializer.serialize_unit_variant("Location", 13u32, "brazilsouth"),
            Self::Australiaeast => serializer.serialize_unit_variant("Location", 14u32, "australiaeast"),
            Self::Australiasoutheast => serializer.serialize_unit_variant("Location", 15u32, "australiasoutheast"),
            Self::Southindia => serializer.serialize_unit_variant("Location", 16u32, "southindia"),
            Self::Westindia => serializer.serialize_unit_variant("Location", 17u32, "westindia"),
            Self::Centralindia => serializer.serialize_unit_variant("Location", 18u32, "centralindia"),
            Self::Canadacentral => serializer.serialize_unit_variant("Location", 19u32, "canadacentral"),
            Self::Canadaeast => serializer.serialize_unit_variant("Location", 20u32, "canadaeast"),
            Self::Uksouth => serializer.serialize_unit_variant("Location", 21u32, "uksouth"),
            Self::Westcentralus => serializer.serialize_unit_variant("Location", 22u32, "westcentralus"),
            Self::Ukwest => serializer.serialize_unit_variant("Location", 23u32, "ukwest"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type ManagementGroupId = String;
#[doc = "Properties for reservation merge"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MergeProperties {
    #[doc = "Format of the resource id should be /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sources: Vec<String>,
}
impl MergeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The request for reservation merge"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MergeRequest {
    #[doc = "Properties for reservation merge"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MergeProperties>,
}
impl MergeRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about an operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paginated list of operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<OperationResponse>,
    #[doc = "Url to get the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Additional details about an operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response containing operation information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResponse {
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Information about an operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Origin of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Additional details about an operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
}
impl OperationResponse {
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
#[doc = "Status of the individual operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OperationStatus")]
pub enum OperationStatus {
    Succeeded,
    Failed,
    Cancelled,
    Pending,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OperationStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OperationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OperationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("OperationStatus", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("OperationStatus", 1u32, "Failed"),
            Self::Cancelled => serializer.serialize_unit_variant("OperationStatus", 2u32, "Cancelled"),
            Self::Pending => serializer.serialize_unit_variant("OperationStatus", 3u32, "Pending"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The request for reservation patch"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Patch {
    #[doc = "Properties for reservation patch"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PatchProperties>,
}
impl Patch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for reservation patch"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchProperties {
    #[doc = "Type of the Applied Scope."]
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[doc = "List of the subscriptions that the benefit will be applied. Do not specify if AppliedScopeType is Shared. This property will be deprecated and replaced by appliedScopeProperties instead for Single AppliedScopeType."]
    #[serde(rename = "appliedScopes", default, skip_serializing_if = "Option::is_none")]
    pub applied_scopes: Option<AppliedScopes>,
    #[doc = "Properties specific to applied scope type. Not required if not applicable. Required and need to provide tenantId and managementGroupId if AppliedScopeType is ManagementGroup"]
    #[serde(rename = "appliedScopeProperties", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_properties: Option<AppliedScopeProperties>,
    #[doc = "Turning this on will apply the reservation discount to other VMs in the same VM size group. Only specify for VirtualMachines reserved resource type."]
    #[serde(rename = "instanceFlexibility", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility: Option<InstanceFlexibility>,
    #[doc = "Display name of the reservation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Setting this to true will automatically purchase a new reservation on the expiration date time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[serde(rename = "renewProperties", default, skip_serializing_if = "Option::is_none")]
    pub renew_properties: Option<patch_properties::RenewProperties>,
    #[doc = "This is the date-time when the Azure hybrid benefit needs to be reviewed."]
    #[serde(rename = "reviewDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub review_date_time: Option<time::OffsetDateTime>,
}
impl PatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod patch_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct RenewProperties {
        #[doc = "The request for reservation purchase"]
        #[serde(rename = "purchaseProperties", default, skip_serializing_if = "Option::is_none")]
        pub purchase_properties: Option<PurchaseRequest>,
    }
    impl RenewProperties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Information about payment related to a reservation order."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaymentDetail {
    #[doc = "Date when the payment needs to be done."]
    #[serde(rename = "dueDate", default, skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[doc = "Date when the transaction is completed. Is null when it is scheduled."]
    #[serde(rename = "paymentDate", default, skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<String>,
    #[doc = "Pricing information containing the amount and the currency code"]
    #[serde(rename = "pricingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<Price>,
    #[doc = "Pricing information containing the amount and the currency code"]
    #[serde(rename = "billingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_total: Option<Price>,
    #[doc = "Shows the Account that is charged for this payment."]
    #[serde(rename = "billingAccount", default, skip_serializing_if = "Option::is_none")]
    pub billing_account: Option<String>,
    #[doc = "Describes whether the payment is completed, failed, cancelled or scheduled in the future."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PaymentStatus>,
    #[serde(rename = "extendedStatusInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_status_info: Option<ExtendedStatusInfo>,
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
#[doc = "Pricing information containing the amount and the currency code"]
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
#[doc = "Represent the current state of the Reservation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Creating,
    PendingResourceHold,
    ConfirmedResourceHold,
    PendingBilling,
    ConfirmedBilling,
    Created,
    Succeeded,
    Cancelled,
    Expired,
    BillingFailed,
    Failed,
    Split,
    Merged,
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
            Self::PendingResourceHold => serializer.serialize_unit_variant("ProvisioningState", 1u32, "PendingResourceHold"),
            Self::ConfirmedResourceHold => serializer.serialize_unit_variant("ProvisioningState", 2u32, "ConfirmedResourceHold"),
            Self::PendingBilling => serializer.serialize_unit_variant("ProvisioningState", 3u32, "PendingBilling"),
            Self::ConfirmedBilling => serializer.serialize_unit_variant("ProvisioningState", 4u32, "ConfirmedBilling"),
            Self::Created => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Created"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Succeeded"),
            Self::Cancelled => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Cancelled"),
            Self::Expired => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Expired"),
            Self::BillingFailed => serializer.serialize_unit_variant("ProvisioningState", 9u32, "BillingFailed"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 10u32, "Failed"),
            Self::Split => serializer.serialize_unit_variant("ProvisioningState", 11u32, "Split"),
            Self::Merged => serializer.serialize_unit_variant("ProvisioningState", 12u32, "Merged"),
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
#[doc = "The request for reservation purchase"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PurchaseRequest {
    #[doc = "The name of sku"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SkuName>,
    #[doc = "The Azure region where the reserved resource lives."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Properties of reservation purchase request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PurchaseRequestProperties>,
}
impl PurchaseRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of reservation purchase request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PurchaseRequestProperties {
    #[doc = "The type of the resource that is being reserved."]
    #[serde(rename = "reservedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub reserved_resource_type: Option<ReservedResourceType>,
    #[doc = "Subscription that will be charged for purchasing reservation or savings plan"]
    #[serde(rename = "billingScopeId", default, skip_serializing_if = "Option::is_none")]
    pub billing_scope_id: Option<BillingScopeId>,
    #[doc = "Represent the term of reservation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<ReservationTerm>,
    #[doc = "Represent the billing plans."]
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<ReservationBillingPlan>,
    #[doc = "Quantity of the skus that are part of the reservation. Must be greater than zero."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<ReservationQuantity>,
    #[doc = "Friendly name of the reservation"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Type of the Applied Scope."]
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[doc = "List of the subscriptions that the benefit will be applied. Do not specify if AppliedScopeType is Shared. This property will be deprecated and replaced by appliedScopeProperties instead for Single AppliedScopeType."]
    #[serde(rename = "appliedScopes", default, skip_serializing_if = "Option::is_none")]
    pub applied_scopes: Option<AppliedScopes>,
    #[doc = "Properties specific to applied scope type. Not required if not applicable. Required and need to provide tenantId and managementGroupId if AppliedScopeType is ManagementGroup"]
    #[serde(rename = "appliedScopeProperties", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_properties: Option<AppliedScopeProperties>,
    #[doc = "Setting this to true will automatically purchase a new reservation on the expiration date time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[doc = "Properties specific to each reserved resource type. Not required if not applicable."]
    #[serde(rename = "reservedResourceProperties", default, skip_serializing_if = "Option::is_none")]
    pub reserved_resource_properties: Option<purchase_request_properties::ReservedResourceProperties>,
    #[doc = "This is the date-time when the Azure hybrid benefit needs to be reviewed."]
    #[serde(rename = "reviewDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub review_date_time: Option<time::OffsetDateTime>,
}
impl PurchaseRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod purchase_request_properties {
    use super::*;
    #[doc = "Properties specific to each reserved resource type. Not required if not applicable."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ReservedResourceProperties {
        #[doc = "Turning this on will apply the reservation discount to other VMs in the same VM size group. Only specify for VirtualMachines reserved resource type."]
        #[serde(rename = "instanceFlexibility", default, skip_serializing_if = "Option::is_none")]
        pub instance_flexibility: Option<InstanceFlexibility>,
    }
    impl ReservedResourceProperties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Quota limits."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaLimits {
    #[doc = "List of quotas (service limits)."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CurrentQuotaLimitBase>,
    #[doc = "The URI for fetching the next page of quotas (service limits). When no more pages exist, the value is null."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for QuotaLimits {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl QuotaLimits {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quotas (service limits) in the request response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaLimitsResponse {
    #[doc = "List of quotas with the quota request status."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CurrentQuotaLimit>,
    #[doc = "The URI for fetching the next page of quota limits. When no more pages exist, the value is null."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl QuotaLimitsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quota properties for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaProperties {
    #[doc = "Quota properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[doc = "Current usage value for the resource."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i32>,
    #[doc = " The limit units, such as **count** and **bytes**. Use the unit field provided in the response of the GET quota operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Resource name provided by the resource provider. Use this property for quotaRequest parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ResourceName>,
    #[doc = "The resource types."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<ResourceTypesName>,
    #[doc = "The time period over which the quota usage values are summarized. For example, P1D (per one day), PT1M (per one minute), and PT1S (per one second). This parameter is optional because, for some resources such as compute, the time period is irrelevant."]
    #[serde(rename = "quotaPeriod", default, skip_serializing_if = "Option::is_none")]
    pub quota_period: Option<String>,
    #[doc = "Additional properties for the specified resource provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl QuotaProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quota request details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaRequestDetails {
    #[doc = "Quota request ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Quota request name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The details of quota request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaRequestProperties>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl QuotaRequestDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quota request details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaRequestDetailsList {
    #[doc = "The quota requests."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<QuotaRequestDetails>,
    #[doc = "The URI to fetch the next page of quota limits. When there are no more pages, this is null."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for QuotaRequestDetailsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl QuotaRequestDetailsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for the quota submission request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaRequestOneResourceSubmitResponse {
    #[doc = "The quota request ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the quota request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of resource. \"Microsoft.Capacity/ServiceLimits\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The details of quota request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaRequestOneResourceProperties>,
}
impl QuotaRequestOneResourceSubmitResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of quota request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaRequestProperties {
    #[doc = "The quota request status."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<QuotaRequestState>,
    #[doc = "User friendly status message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The time when the quota request was submitted using format: yyyy-MM-ddTHH:mm:ssZ as specified by the ISO 8601 standard."]
    #[serde(rename = "requestSubmitTime", default, with = "azure_core::date::rfc3339::option")]
    pub request_submit_time: Option<time::OffsetDateTime>,
    #[doc = "The quotaRequests."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<SubRequest>,
}
impl QuotaRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The quota request status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "QuotaRequestState")]
pub enum QuotaRequestState {
    Accepted,
    Invalid,
    Succeeded,
    Failed,
    InProgress,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for QuotaRequestState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for QuotaRequestState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for QuotaRequestState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Accepted => serializer.serialize_unit_variant("QuotaRequestState", 0u32, "Accepted"),
            Self::Invalid => serializer.serialize_unit_variant("QuotaRequestState", 1u32, "Invalid"),
            Self::Succeeded => serializer.serialize_unit_variant("QuotaRequestState", 2u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("QuotaRequestState", 3u32, "Failed"),
            Self::InProgress => serializer.serialize_unit_variant("QuotaRequestState", 4u32, "InProgress"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Quota request status details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaRequestStatusDetails {
    #[doc = "The quota request status."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<QuotaRequestState>,
    #[doc = "A user friendly message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl QuotaRequestStatusDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for the quota submission request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaRequestSubmitResponse {
    #[doc = "The quota request ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the quota request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The details of quota request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaRequestProperties>,
    #[doc = "Type of resource. \"Microsoft.Capacity/serviceLimits\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl QuotaRequestSubmitResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response with request ID that the quota request was accepted."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaRequestSubmitResponse201 {
    #[doc = "The quota request ID. Use the requestId parameter to check the request status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Operation ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Quota request status details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaRequestStatusDetails>,
}
impl QuotaRequestSubmitResponse201 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "billing information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RefundBillingInformation {
    #[doc = "Represent the billing plans."]
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<ReservationBillingPlan>,
    #[doc = "The number of completed transactions in this reservation's payment"]
    #[serde(rename = "completedTransactions", default, skip_serializing_if = "Option::is_none")]
    pub completed_transactions: Option<i32>,
    #[doc = "The number of total transactions in this reservation's payment"]
    #[serde(rename = "totalTransactions", default, skip_serializing_if = "Option::is_none")]
    pub total_transactions: Option<i32>,
    #[doc = "Pricing information containing the amount and the currency code"]
    #[serde(rename = "billingCurrencyTotalPaidAmount", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_total_paid_amount: Option<Price>,
    #[doc = "Pricing information containing the amount and the currency code"]
    #[serde(rename = "billingCurrencyProratedAmount", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_prorated_amount: Option<Price>,
    #[doc = "Pricing information containing the amount and the currency code"]
    #[serde(
        rename = "billingCurrencyRemainingCommitmentAmount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub billing_currency_remaining_commitment_amount: Option<Price>,
}
impl RefundBillingInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "error details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RefundPolicyError {
    #[doc = "Error code describing the reason that service is not able to process the incoming request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<ErrorResponseCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl RefundPolicyError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Refund policy result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RefundPolicyResult {
    #[doc = "Refund policy result property"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RefundPolicyResultProperty>,
}
impl RefundPolicyResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Refund policy result property"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RefundPolicyResultProperty {
    #[doc = "Pricing information containing the amount and the currency code"]
    #[serde(rename = "consumedRefundsTotal", default, skip_serializing_if = "Option::is_none")]
    pub consumed_refunds_total: Option<Price>,
    #[doc = "Pricing information containing the amount and the currency code"]
    #[serde(rename = "maxRefundLimit", default, skip_serializing_if = "Option::is_none")]
    pub max_refund_limit: Option<Price>,
    #[doc = "Refund Policy errors"]
    #[serde(
        rename = "policyErrors",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub policy_errors: Vec<RefundPolicyError>,
}
impl RefundPolicyResultProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request containing information needed for returning reservation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RefundRequest {
    #[doc = "Properties needed for refund request including the session id from calculate refund, the scope, the reservation to be returned and the return reason."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RefundRequestProperties>,
}
impl RefundRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties needed for refund request including the session id from calculate refund, the scope, the reservation to be returned and the return reason."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RefundRequestProperties {
    #[doc = "SessionId that was returned by CalculateRefund API."]
    #[serde(rename = "sessionId", default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[doc = "The scope of the refund, e.g. Reservation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "Reservation to return"]
    #[serde(rename = "reservationToReturn", default, skip_serializing_if = "Option::is_none")]
    pub reservation_to_return: Option<ReservationToReturn>,
    #[doc = "The reason of returning the reservation"]
    #[serde(rename = "returnReason", default, skip_serializing_if = "Option::is_none")]
    pub return_reason: Option<String>,
}
impl RefundRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of refund request containing refund information of reservation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RefundResponse {
    #[doc = "Fully qualified identifier of the reservation being returned"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The refund properties of reservation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RefundResponseProperties>,
}
impl RefundResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The refund properties of reservation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RefundResponseProperties {
    #[doc = "Refund session identifier"]
    #[serde(rename = "sessionId", default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[doc = "Quantity to be returned"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[doc = "Pricing information containing the amount and the currency code"]
    #[serde(rename = "billingRefundAmount", default, skip_serializing_if = "Option::is_none")]
    pub billing_refund_amount: Option<Price>,
    #[doc = "Pricing information containing the amount and the currency code"]
    #[serde(rename = "pricingRefundAmount", default, skip_serializing_if = "Option::is_none")]
    pub pricing_refund_amount: Option<Price>,
    #[doc = "Refund policy result"]
    #[serde(rename = "policyResult", default, skip_serializing_if = "Option::is_none")]
    pub policy_result: Option<RefundPolicyResult>,
    #[doc = "billing information"]
    #[serde(rename = "billingInformation", default, skip_serializing_if = "Option::is_none")]
    pub billing_information: Option<RefundBillingInformation>,
}
impl RefundResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type Renew = bool;
#[doc = "The renew properties for a reservation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RenewPropertiesResponse {
    #[doc = "The request for reservation purchase"]
    #[serde(rename = "purchaseProperties", default, skip_serializing_if = "Option::is_none")]
    pub purchase_properties: Option<PurchaseRequest>,
    #[doc = "Amount that Microsoft uses for record. Used during refund for calculating refund limit. Tax is not included. This is locked price 30 days before expiry."]
    #[serde(rename = "pricingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_total: Option<renew_properties_response::PricingCurrencyTotal>,
    #[doc = "Currency and amount that customer will be charged in customer's local currency for renewal purchase. Tax is not included."]
    #[serde(rename = "billingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_total: Option<renew_properties_response::BillingCurrencyTotal>,
}
impl RenewPropertiesResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod renew_properties_response {
    use super::*;
    #[doc = "Amount that Microsoft uses for record. Used during refund for calculating refund limit. Tax is not included. This is locked price 30 days before expiry."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct PricingCurrencyTotal {
        #[doc = "The ISO 4217 3-letter currency code for the currency used by this purchase record."]
        #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
        pub currency_code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
    }
    impl PricingCurrencyTotal {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Currency and amount that customer will be charged in customer's local currency for renewal purchase. Tax is not included."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct BillingCurrencyTotal {
        #[doc = "The ISO 4217 3-letter currency code for the currency used by this purchase record."]
        #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
        pub currency_code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub amount: Option<f64>,
    }
    impl BillingCurrencyTotal {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Represent the billing plans."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReservationBillingPlan")]
pub enum ReservationBillingPlan {
    Upfront,
    Monthly,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReservationBillingPlan {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReservationBillingPlan {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReservationBillingPlan {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Upfront => serializer.serialize_unit_variant("ReservationBillingPlan", 0u32, "Upfront"),
            Self::Monthly => serializer.serialize_unit_variant("ReservationBillingPlan", 1u32, "Monthly"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "List of `Reservation`s"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ReservationResponse>,
    #[doc = "Url to get the next page of reservations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ReservationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ReservationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of reservation merge"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationMergeProperties {
    #[doc = "Reservation resource id Created due to the merge. Format of the resource id is /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(rename = "mergeDestination", default, skip_serializing_if = "Option::is_none")]
    pub merge_destination: Option<String>,
    #[doc = "Resource ids of the source reservation's merged to form this reservation. Format of the resource id is /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(
        rename = "mergeSources",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub merge_sources: Vec<String>,
}
impl ReservationMergeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information describing the type of billing plan for this reservation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationOrderBillingPlanInformation {
    #[doc = "Pricing information containing the amount and the currency code"]
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
impl ReservationOrderBillingPlanInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of `ReservationOrder`s"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationOrderList {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ReservationOrderResponse>,
    #[doc = "Url to get the next page of reservationOrders."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ReservationOrderList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ReservationOrderList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a reservation order."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationOrderProperties {
    #[doc = "Friendly name for user to easily identified the reservation."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "This is the DateTime when the reservation was initially requested for purchase."]
    #[serde(rename = "requestDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub request_date_time: Option<time::OffsetDateTime>,
    #[doc = "This is the DateTime when the reservation was created."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "This is the date when the reservation will expire."]
    #[serde(rename = "expiryDate", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[doc = "This is the date-time when the reservation will expire."]
    #[serde(rename = "expiryDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub expiry_date_time: Option<time::OffsetDateTime>,
    #[doc = "This is the DateTime when the reservation benefit started."]
    #[serde(rename = "benefitStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub benefit_start_time: Option<time::OffsetDateTime>,
    #[doc = "Total Quantity of the skus purchased in the reservation."]
    #[serde(rename = "originalQuantity", default, skip_serializing_if = "Option::is_none")]
    pub original_quantity: Option<i32>,
    #[doc = "Represent the term of reservation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<ReservationTerm>,
    #[doc = "Represent the current state of the Reservation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Represent the billing plans."]
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<ReservationBillingPlan>,
    #[doc = "Information describing the type of billing plan for this reservation."]
    #[serde(rename = "planInformation", default, skip_serializing_if = "Option::is_none")]
    pub plan_information: Option<ReservationOrderBillingPlanInformation>,
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reservations: Vec<ReservationResponse>,
    #[doc = "This is the date-time when the Azure Hybrid Benefit needs to be reviewed."]
    #[serde(rename = "reviewDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub review_date_time: Option<time::OffsetDateTime>,
}
impl ReservationOrderProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of a reservation order being returned."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationOrderResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<i32>,
    #[doc = "Identifier of the reservation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the reservation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Properties of a reservation order."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationOrderProperties>,
    #[doc = "Type of resource. \"Microsoft.Capacity/reservations\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ReservationOrderResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ReservationQuantity = i32;
#[doc = "The definition of the reservation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationResponse {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The Azure region where the reserved resource lives."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<i32>,
    #[doc = "The name of sku"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SkuName>,
    #[doc = "The properties of the reservations"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationsProperties>,
    #[doc = "Resource Provider type to be reserved."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<reservation_response::Kind>,
}
impl ReservationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod reservation_response {
    use super::*;
    #[doc = "Resource Provider type to be reserved."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        #[serde(rename = "Microsoft.Compute")]
        MicrosoftCompute,
    }
}
#[doc = "Properties of reservation split"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationSplitProperties {
    #[doc = "List of destination resource id that are created due to split. Format of the resource id is /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(
        rename = "splitDestinations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub split_destinations: Vec<String>,
    #[doc = "Resource id of the reservation from which this is split. Format of the resource id is /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(rename = "splitSource", default, skip_serializing_if = "Option::is_none")]
    pub split_source: Option<String>,
}
impl ReservationSplitProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReservationStatusCode")]
pub enum ReservationStatusCode {
    None,
    Pending,
    Processing,
    Active,
    PurchaseError,
    PaymentInstrumentError,
    Split,
    Merged,
    Expired,
    Succeeded,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReservationStatusCode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReservationStatusCode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReservationStatusCode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ReservationStatusCode", 0u32, "None"),
            Self::Pending => serializer.serialize_unit_variant("ReservationStatusCode", 1u32, "Pending"),
            Self::Processing => serializer.serialize_unit_variant("ReservationStatusCode", 2u32, "Processing"),
            Self::Active => serializer.serialize_unit_variant("ReservationStatusCode", 3u32, "Active"),
            Self::PurchaseError => serializer.serialize_unit_variant("ReservationStatusCode", 4u32, "PurchaseError"),
            Self::PaymentInstrumentError => serializer.serialize_unit_variant("ReservationStatusCode", 5u32, "PaymentInstrumentError"),
            Self::Split => serializer.serialize_unit_variant("ReservationStatusCode", 6u32, "Split"),
            Self::Merged => serializer.serialize_unit_variant("ReservationStatusCode", 7u32, "Merged"),
            Self::Expired => serializer.serialize_unit_variant("ReservationStatusCode", 8u32, "Expired"),
            Self::Succeeded => serializer.serialize_unit_variant("ReservationStatusCode", 9u32, "Succeeded"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The roll up count summary of reservations in each state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationSummary {
    #[doc = "The number of reservation in Succeeded state"]
    #[serde(rename = "succeededCount", default, skip_serializing_if = "Option::is_none")]
    pub succeeded_count: Option<f64>,
    #[doc = "The number of reservation in Failed state"]
    #[serde(rename = "failedCount", default, skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<f64>,
    #[doc = "The number of reservation in Expiring state"]
    #[serde(rename = "expiringCount", default, skip_serializing_if = "Option::is_none")]
    pub expiring_count: Option<f64>,
    #[doc = "The number of reservation in Expired state"]
    #[serde(rename = "expiredCount", default, skip_serializing_if = "Option::is_none")]
    pub expired_count: Option<f64>,
    #[doc = "The number of reservation in Pending state"]
    #[serde(rename = "pendingCount", default, skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<f64>,
    #[doc = "The number of reservation in Cancelled state"]
    #[serde(rename = "cancelledCount", default, skip_serializing_if = "Option::is_none")]
    pub cancelled_count: Option<f64>,
    #[doc = "The number of reservation in Processing state"]
    #[serde(rename = "processingCount", default, skip_serializing_if = "Option::is_none")]
    pub processing_count: Option<f64>,
    #[doc = "The number of reservation in Warning state"]
    #[serde(rename = "warningCount", default, skip_serializing_if = "Option::is_none")]
    pub warning_count: Option<f64>,
    #[doc = "The number of reservation in NoBenefit state"]
    #[serde(rename = "noBenefitCount", default, skip_serializing_if = "Option::is_none")]
    pub no_benefit_count: Option<f64>,
}
impl ReservationSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of reservation swap"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationSwapProperties {
    #[doc = "Resource id of the source reservation that gets swapped. Format of the resource id is /providers/microsoft.capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(rename = "swapSource", default, skip_serializing_if = "Option::is_none")]
    pub swap_source: Option<String>,
    #[doc = "Reservation resource id that the original resource gets swapped to. Format of the resource id is /providers/microsoft.capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(rename = "swapDestination", default, skip_serializing_if = "Option::is_none")]
    pub swap_destination: Option<String>,
}
impl ReservationSwapProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represent the term of reservation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ReservationTerm")]
pub enum ReservationTerm {
    #[serde(rename = "P1Y")]
    P1y,
    #[serde(rename = "P3Y")]
    P3y,
    #[serde(rename = "P5Y")]
    P5y,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ReservationTerm {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ReservationTerm {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ReservationTerm {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::P1y => serializer.serialize_unit_variant("ReservationTerm", 0u32, "P1Y"),
            Self::P3y => serializer.serialize_unit_variant("ReservationTerm", 1u32, "P3Y"),
            Self::P5y => serializer.serialize_unit_variant("ReservationTerm", 2u32, "P5Y"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Reservation refund details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationToExchange {
    #[doc = "Fully qualified id of the reservation being returned."]
    #[serde(rename = "reservationId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    #[doc = "Quantity to be returned"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[doc = "Pricing information containing the amount and the currency code"]
    #[serde(rename = "billingRefundAmount", default, skip_serializing_if = "Option::is_none")]
    pub billing_refund_amount: Option<Price>,
    #[doc = "billing information"]
    #[serde(rename = "billingInformation", default, skip_serializing_if = "Option::is_none")]
    pub billing_information: Option<BillingInformation>,
}
impl ReservationToExchange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reservation purchase details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationToPurchaseCalculateExchange {
    #[doc = "The request for reservation purchase"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PurchaseRequest>,
    #[doc = "Pricing information containing the amount and the currency code"]
    #[serde(rename = "billingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_total: Option<Price>,
}
impl ReservationToPurchaseCalculateExchange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reservation purchase details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationToPurchaseExchange {
    #[doc = "Fully qualified id of the reservationOrder being purchased"]
    #[serde(rename = "reservationOrderId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order_id: Option<String>,
    #[doc = "Fully qualified id of the reservation being purchased. This value is only guaranteed to be non-null if the purchase is successful."]
    #[serde(rename = "reservationId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    #[doc = "The request for reservation purchase"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PurchaseRequest>,
    #[doc = "Pricing information containing the amount and the currency code"]
    #[serde(rename = "billingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_total: Option<Price>,
    #[doc = "Status of the individual operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OperationStatus>,
}
impl ReservationToPurchaseExchange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reservation to return"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationToReturn {
    #[doc = "Fully qualified identifier of the reservation being returned"]
    #[serde(rename = "reservationId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    #[doc = "Quantity to be returned. Must be greater than zero."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}
impl ReservationToReturn {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reservation refund details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationToReturnForExchange {
    #[doc = "Fully qualified id of the reservation being returned."]
    #[serde(rename = "reservationId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    #[doc = "Quantity to be returned"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[doc = "Pricing information containing the amount and the currency code"]
    #[serde(rename = "billingRefundAmount", default, skip_serializing_if = "Option::is_none")]
    pub billing_refund_amount: Option<Price>,
    #[doc = "billing information"]
    #[serde(rename = "billingInformation", default, skip_serializing_if = "Option::is_none")]
    pub billing_information: Option<BillingInformation>,
    #[doc = "Status of the individual operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OperationStatus>,
}
impl ReservationToReturnForExchange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The aggregate values of reservation utilization"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationUtilizationAggregates {
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
impl ReservationUtilizationAggregates {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of reservations and summary of roll out count of reservations in each state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationsListResult {
    #[doc = "The list of reservations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ReservationResponse>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The roll up count summary of reservations in each state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<ReservationSummary>,
}
impl azure_core::Continuable for ReservationsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ReservationsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the reservations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationsProperties {
    #[doc = "The type of the resource that is being reserved."]
    #[serde(rename = "reservedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub reserved_resource_type: Option<ReservedResourceType>,
    #[doc = "Turning this on will apply the reservation discount to other VMs in the same VM size group. Only specify for VirtualMachines reserved resource type."]
    #[serde(rename = "instanceFlexibility", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility: Option<InstanceFlexibility>,
    #[doc = "Friendly name for user to easily identify the reservation"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "List of the subscriptions that the benefit will be applied. Do not specify if AppliedScopeType is Shared. This property will be deprecated and replaced by appliedScopeProperties instead for Single AppliedScopeType."]
    #[serde(rename = "appliedScopes", default, skip_serializing_if = "Option::is_none")]
    pub applied_scopes: Option<AppliedScopes>,
    #[doc = "Type of the Applied Scope."]
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[doc = "Indicates if the reservation is archived"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[doc = "Capabilities of the reservation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<String>,
    #[doc = "Quantity of the skus that are part of the reservation. Must be greater than zero."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<ReservationQuantity>,
    #[doc = "Represent the current state of the Reservation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "DateTime of the reservation starting when this version is effective from."]
    #[serde(rename = "effectiveDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub effective_date_time: Option<time::OffsetDateTime>,
    #[doc = "This is the DateTime when the reservation benefit started."]
    #[serde(rename = "benefitStartTime", default, with = "azure_core::date::rfc3339::option")]
    pub benefit_start_time: Option<time::OffsetDateTime>,
    #[doc = "DateTime of the last time the reservation was updated."]
    #[serde(rename = "lastUpdatedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated_date_time: Option<time::OffsetDateTime>,
    #[doc = "This is the date when the reservation will expire."]
    #[serde(rename = "expiryDate", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[doc = "This is the date-time when the reservation will expire."]
    #[serde(rename = "expiryDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub expiry_date_time: Option<time::OffsetDateTime>,
    #[doc = "This is the date-time when the Azure Hybrid Benefit needs to be reviewed."]
    #[serde(rename = "reviewDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub review_date_time: Option<time::OffsetDateTime>,
    #[doc = "Description of the sku in english."]
    #[serde(rename = "skuDescription", default, skip_serializing_if = "Option::is_none")]
    pub sku_description: Option<String>,
    #[serde(rename = "extendedStatusInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_status_info: Option<ExtendedStatusInfo>,
    #[doc = "Represent the billing plans."]
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<ReservationBillingPlan>,
    #[doc = "The provisioning state of the reservation for display, e.g. Succeeded"]
    #[serde(rename = "displayProvisioningState", default, skip_serializing_if = "Option::is_none")]
    pub display_provisioning_state: Option<String>,
    #[doc = "The provisioning sub-state of the reservation, e.g. Succeeded"]
    #[serde(rename = "provisioningSubState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_sub_state: Option<String>,
    #[doc = "This is the date when the reservation was purchased."]
    #[serde(rename = "purchaseDate", default, skip_serializing_if = "Option::is_none")]
    pub purchase_date: Option<String>,
    #[doc = "This is the date-time when the reservation was purchased."]
    #[serde(rename = "purchaseDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub purchase_date_time: Option<time::OffsetDateTime>,
    #[doc = "Properties of reservation split"]
    #[serde(rename = "splitProperties", default, skip_serializing_if = "Option::is_none")]
    pub split_properties: Option<ReservationSplitProperties>,
    #[doc = "Properties of reservation merge"]
    #[serde(rename = "mergeProperties", default, skip_serializing_if = "Option::is_none")]
    pub merge_properties: Option<ReservationMergeProperties>,
    #[doc = "Properties of reservation swap"]
    #[serde(rename = "swapProperties", default, skip_serializing_if = "Option::is_none")]
    pub swap_properties: Option<ReservationSwapProperties>,
    #[doc = "Properties specific to applied scope type. Not required if not applicable. Required and need to provide tenantId and managementGroupId if AppliedScopeType is ManagementGroup"]
    #[serde(rename = "appliedScopeProperties", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_properties: Option<AppliedScopeProperties>,
    #[doc = "Subscription that will be charged for purchasing reservation or savings plan"]
    #[serde(rename = "billingScopeId", default, skip_serializing_if = "Option::is_none")]
    pub billing_scope_id: Option<BillingScopeId>,
    #[doc = "Setting this to true will automatically purchase a new reservation on the expiration date time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renew: Option<Renew>,
    #[doc = "Reservation Id of the reservation from which this reservation is renewed. Format of the resource Id is /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}."]
    #[serde(rename = "renewSource", default, skip_serializing_if = "Option::is_none")]
    pub renew_source: Option<String>,
    #[doc = "Reservation Id of the reservation which is purchased because of renew. Format of the resource Id is /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}."]
    #[serde(rename = "renewDestination", default, skip_serializing_if = "Option::is_none")]
    pub renew_destination: Option<String>,
    #[doc = "The renew properties for a reservation."]
    #[serde(rename = "renewProperties", default, skip_serializing_if = "Option::is_none")]
    pub renew_properties: Option<RenewPropertiesResponse>,
    #[doc = "Represent the term of reservation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<ReservationTerm>,
    #[doc = "The applied scope type of the reservation for display, e.g. Shared"]
    #[serde(rename = "userFriendlyAppliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub user_friendly_applied_scope_type: Option<String>,
    #[doc = "The renew state of the reservation for display, e.g. On"]
    #[serde(rename = "userFriendlyRenewState", default, skip_serializing_if = "Option::is_none")]
    pub user_friendly_renew_state: Option<String>,
    #[doc = "Reservation utilization"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub utilization: Option<reservations_properties::Utilization>,
}
impl ReservationsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod reservations_properties {
    use super::*;
    #[doc = "Reservation utilization"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Utilization {
        #[doc = "last 7 day utilization trend for a reservation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub trend: Option<String>,
        #[doc = "The array of aggregates of a reservation's utilization"]
        #[serde(
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub aggregates: Vec<ReservationUtilizationAggregates>,
    }
    impl Utilization {
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
    #[doc = "Fully qualified resource ID for the resource. E.g. \"/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}\""]
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
#[doc = "Resource name provided by the resource provider. Use this property for quotaRequest parameter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceName {
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Resource display localized name."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl ResourceName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResourceTypesName")]
pub enum ResourceTypesName {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "dedicated")]
    Dedicated,
    #[serde(rename = "lowPriority")]
    LowPriority,
    #[serde(rename = "shared")]
    Shared,
    #[serde(rename = "serviceSpecific")]
    ServiceSpecific,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResourceTypesName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResourceTypesName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResourceTypesName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Standard => serializer.serialize_unit_variant("ResourceTypesName", 0u32, "standard"),
            Self::Dedicated => serializer.serialize_unit_variant("ResourceTypesName", 1u32, "dedicated"),
            Self::LowPriority => serializer.serialize_unit_variant("ResourceTypesName", 2u32, "lowPriority"),
            Self::Shared => serializer.serialize_unit_variant("ResourceTypesName", 3u32, "shared"),
            Self::ServiceSpecific => serializer.serialize_unit_variant("ResourceTypesName", 4u32, "serviceSpecific"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Request body for savings plan purchase"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanPurchaseRequest {
    #[doc = "The name of sku"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SkuName>,
    #[doc = "Properties of a savings plan purchase"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SavingsPlanPurchaseRequestProperties>,
}
impl SavingsPlanPurchaseRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a savings plan purchase"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanPurchaseRequestProperties {
    #[doc = "Friendly name of the savings plan"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Subscription that will be charged for purchasing reservation or savings plan"]
    #[serde(rename = "billingScopeId", default, skip_serializing_if = "Option::is_none")]
    pub billing_scope_id: Option<BillingScopeId>,
    #[doc = "Represent savings plan term in ISO 8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<BenefitTerm>,
    #[doc = "Represents the billing plan in ISO 8601 format. Required only for monthly billing plans."]
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<BillingPlan>,
    #[doc = "Type of the Applied Scope."]
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[doc = "Properties specific to applied scope type. Not required if not applicable. Required and need to provide tenantId and managementGroupId if AppliedScopeType is ManagementGroup"]
    #[serde(rename = "appliedScopeProperties", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_properties: Option<AppliedScopeProperties>,
    #[doc = "Commitment towards the benefit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
}
impl SavingsPlanPurchaseRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Savings plan purchase details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanToPurchaseCalculateExchange {
    #[doc = "Request body for savings plan purchase"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SavingsPlanPurchaseRequest>,
    #[doc = "Pricing information containing the amount and the currency code"]
    #[serde(rename = "billingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_total: Option<Price>,
}
impl SavingsPlanToPurchaseCalculateExchange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Savings plan purchase details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SavingsPlanToPurchaseExchange {
    #[doc = "Fully qualified id of the savings plan order being purchased"]
    #[serde(rename = "savingsPlanOrderId", default, skip_serializing_if = "Option::is_none")]
    pub savings_plan_order_id: Option<String>,
    #[doc = "Fully qualified id of the savings plan being purchased. This value is only guaranteed to be non-null if the purchase is successful."]
    #[serde(rename = "savingsPlanId", default, skip_serializing_if = "Option::is_none")]
    pub savings_plan_id: Option<String>,
    #[doc = "Request body for savings plan purchase"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SavingsPlanPurchaseRequest>,
    #[doc = "Pricing information containing the amount and the currency code"]
    #[serde(rename = "billingCurrencyTotal", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_total: Option<Price>,
    #[doc = "Status of the individual operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OperationStatus>,
}
impl SavingsPlanToPurchaseExchange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The scope and whether it is valid."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopeProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}
impl ScopeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The API error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceError {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message text."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The list of error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ServiceErrorDetail>,
}
impl ServiceError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceErrorDetail {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ServiceErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Capability of a sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuCapability {
    #[doc = "An invariant to describe the feature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "An invariant if the feature is measured by quantity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl SkuCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The name of sku"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl SkuName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Property of a sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuProperty {
    #[doc = "An invariant to describe the feature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "An invariant if the feature is measured by quantity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl SkuProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Restriction of a sku."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuRestriction {
    #[doc = "The type of restrictions."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The value of restrictions. If the restriction type is set to location. This would be different locations where the sku is restricted."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<String>,
    #[doc = "The reason for restriction."]
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
}
impl SkuRestriction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for reservation split"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SplitProperties {
    #[doc = "List of the quantities in the new reservations to create."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub quantities: Vec<i32>,
    #[doc = "Resource id of the reservation to be split. Format of the resource id should be /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(rename = "reservationId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
}
impl SplitProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The request for reservation split"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SplitRequest {
    #[doc = "Properties for reservation split"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SplitProperties>,
}
impl SplitRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The sub-request submitted with the quota request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubRequest {
    #[doc = "Quota (resource limit)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[doc = "Resource name provided by the resource provider. Use this property for quotaRequest parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ResourceName>,
    #[doc = "Resource type for which the quota check was made."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = " The limit units, such as **count** and **bytes**. Use the unit field provided in the response of the GET quota operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The quota request status."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<QuotaRequestState>,
    #[doc = "User-friendly status message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Sub request ID for individual request."]
    #[serde(rename = "subRequestId", default, skip_serializing_if = "Option::is_none")]
    pub sub_request_id: Option<String>,
}
impl SubRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type SubscriptionId = String;
#[doc = "The scopes checked by the available scope api."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionScopeProperties {
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub scopes: Vec<ScopeProperties>,
}
impl SubscriptionScopeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type TenantId = String;
#[doc = "The applied scope type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UserFriendlyAppliedScopeType")]
pub enum UserFriendlyAppliedScopeType {
    None,
    Shared,
    Single,
    ResourceGroup,
    ManagementGroup,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UserFriendlyAppliedScopeType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UserFriendlyAppliedScopeType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UserFriendlyAppliedScopeType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("UserFriendlyAppliedScopeType", 0u32, "None"),
            Self::Shared => serializer.serialize_unit_variant("UserFriendlyAppliedScopeType", 1u32, "Shared"),
            Self::Single => serializer.serialize_unit_variant("UserFriendlyAppliedScopeType", 2u32, "Single"),
            Self::ResourceGroup => serializer.serialize_unit_variant("UserFriendlyAppliedScopeType", 3u32, "ResourceGroup"),
            Self::ManagementGroup => serializer.serialize_unit_variant("UserFriendlyAppliedScopeType", 4u32, "ManagementGroup"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The renew state of the reservation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UserFriendlyRenewState")]
pub enum UserFriendlyRenewState {
    On,
    Off,
    Renewed,
    NotRenewed,
    NotApplicable,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UserFriendlyRenewState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UserFriendlyRenewState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UserFriendlyRenewState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::On => serializer.serialize_unit_variant("UserFriendlyRenewState", 0u32, "On"),
            Self::Off => serializer.serialize_unit_variant("UserFriendlyRenewState", 1u32, "Off"),
            Self::Renewed => serializer.serialize_unit_variant("UserFriendlyRenewState", 2u32, "Renewed"),
            Self::NotRenewed => serializer.serialize_unit_variant("UserFriendlyRenewState", 3u32, "NotRenewed"),
            Self::NotApplicable => serializer.serialize_unit_variant("UserFriendlyRenewState", 4u32, "NotApplicable"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The details of quota request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaRequestOneResourceProperties {
    #[doc = "The quota request status."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<QuotaRequestState>,
    #[doc = "User friendly status message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The time when the quota request was submitted using format: yyyy-MM-ddTHH:mm:ssZ as specified by the ISO 8601 standard."]
    #[serde(rename = "requestSubmitTime", default, with = "azure_core::date::rfc3339::option")]
    pub request_submit_time: Option<time::OffsetDateTime>,
    #[doc = "Quota properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CurrentQuotaLimitBase>,
}
impl QuotaRequestOneResourceProperties {
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
