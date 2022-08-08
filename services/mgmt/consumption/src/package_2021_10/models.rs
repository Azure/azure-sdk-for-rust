#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The amount with exchange rate."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AmountWithExchangeRate {
    #[serde(flatten)]
    pub amount: Amount,
    #[doc = "The exchange rate."]
    #[serde(rename = "exchangeRate", default, skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<f64>,
    #[doc = "The exchange rate month."]
    #[serde(rename = "exchangeRateMonth", default, skip_serializing_if = "Option::is_none")]
    pub exchange_rate_month: Option<i32>,
}
impl AmountWithExchangeRate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A balance resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Balance {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the balance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BalanceProperties>,
}
impl Balance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the balance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BalanceProperties {
    #[doc = "The ISO currency in which the meter is charged, for example, USD."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The beginning balance for the billing period."]
    #[serde(rename = "beginningBalance", default, skip_serializing_if = "Option::is_none")]
    pub beginning_balance: Option<f64>,
    #[doc = "The ending balance for the billing period (for open periods this will be updated daily)."]
    #[serde(rename = "endingBalance", default, skip_serializing_if = "Option::is_none")]
    pub ending_balance: Option<f64>,
    #[doc = "Total new purchase amount."]
    #[serde(rename = "newPurchases", default, skip_serializing_if = "Option::is_none")]
    pub new_purchases: Option<f64>,
    #[doc = "Total adjustment amount."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adjustments: Option<f64>,
    #[doc = "Total Commitment usage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub utilized: Option<f64>,
    #[doc = "Overage for Azure services."]
    #[serde(rename = "serviceOverage", default, skip_serializing_if = "Option::is_none")]
    pub service_overage: Option<f64>,
    #[doc = "Charges Billed separately."]
    #[serde(rename = "chargesBilledSeparately", default, skip_serializing_if = "Option::is_none")]
    pub charges_billed_separately: Option<f64>,
    #[doc = "serviceOverage + chargesBilledSeparately."]
    #[serde(rename = "totalOverage", default, skip_serializing_if = "Option::is_none")]
    pub total_overage: Option<f64>,
    #[doc = "Azure service commitment + total Overage."]
    #[serde(rename = "totalUsage", default, skip_serializing_if = "Option::is_none")]
    pub total_usage: Option<f64>,
    #[doc = "Total charges for Azure Marketplace."]
    #[serde(rename = "azureMarketplaceServiceCharges", default, skip_serializing_if = "Option::is_none")]
    pub azure_marketplace_service_charges: Option<f64>,
    #[doc = "The billing frequency."]
    #[serde(rename = "billingFrequency", default, skip_serializing_if = "Option::is_none")]
    pub billing_frequency: Option<balance_properties::BillingFrequency>,
    #[doc = "Price is hidden or not."]
    #[serde(rename = "priceHidden", default, skip_serializing_if = "Option::is_none")]
    pub price_hidden: Option<bool>,
    #[doc = "List of new purchases."]
    #[serde(rename = "newPurchasesDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub new_purchases_details: Vec<serde_json::Value>,
    #[doc = "List of Adjustments (Promo credit, SIE credit etc.)."]
    #[serde(rename = "adjustmentDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub adjustment_details: Vec<serde_json::Value>,
}
impl BalanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod balance_properties {
    use super::*;
    #[doc = "The billing frequency."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BillingFrequency")]
    pub enum BillingFrequency {
        Month,
        Quarter,
        Year,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BillingFrequency {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BillingFrequency {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BillingFrequency {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Month => serializer.serialize_unit_variant("BillingFrequency", 0u32, "Month"),
                Self::Quarter => serializer.serialize_unit_variant("BillingFrequency", 1u32, "Quarter"),
                Self::Year => serializer.serialize_unit_variant("BillingFrequency", 2u32, "Year"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A budget resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Budget {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of the budget."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BudgetProperties>,
}
impl Budget {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The comparison expression to be used in the budgets."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BudgetComparisonExpression {
    #[doc = "The name of the column to use in comparison."]
    pub name: String,
    #[doc = "The operator to use for comparison."]
    pub operator: budget_comparison_expression::Operator,
    #[doc = "Array of values to use for comparison"]
    pub values: Vec<String>,
}
impl BudgetComparisonExpression {
    pub fn new(name: String, operator: budget_comparison_expression::Operator, values: Vec<String>) -> Self {
        Self { name, operator, values }
    }
}
pub mod budget_comparison_expression {
    use super::*;
    #[doc = "The operator to use for comparison."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        In,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::In => serializer.serialize_unit_variant("Operator", 0u32, "In"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "May be used to filter budgets by resource group, resource, or meter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BudgetFilter {
    #[doc = "The logical \"AND\" expression. Must have at least 2 items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub and: Vec<BudgetFilterProperties>,
    #[doc = "The comparison expression to be used in the budgets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<BudgetComparisonExpression>,
    #[doc = "The comparison expression to be used in the budgets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BudgetComparisonExpression>,
}
impl BudgetFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Dimensions or Tags to filter a budget by."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BudgetFilterProperties {
    #[doc = "The comparison expression to be used in the budgets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<BudgetComparisonExpression>,
    #[doc = "The comparison expression to be used in the budgets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BudgetComparisonExpression>,
}
impl BudgetFilterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the budget."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BudgetProperties {
    #[doc = "The category of the budget, whether the budget tracks cost or usage."]
    pub category: budget_properties::Category,
    #[doc = "The total amount of cost to track with the budget"]
    pub amount: f64,
    #[doc = "The time covered by a budget. Tracking of the amount will be reset based on the time grain. BillingMonth, BillingQuarter, and BillingAnnual are only supported by WD customers"]
    #[serde(rename = "timeGrain")]
    pub time_grain: budget_properties::TimeGrain,
    #[doc = "The start and end date for a budget."]
    #[serde(rename = "timePeriod")]
    pub time_period: BudgetTimePeriod,
    #[doc = "May be used to filter budgets by resource group, resource, or meter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<BudgetFilter>,
    #[doc = "The current amount of cost which is being tracked for a budget."]
    #[serde(rename = "currentSpend", default, skip_serializing_if = "Option::is_none")]
    pub current_spend: Option<CurrentSpend>,
    #[doc = "Dictionary of notifications associated with the budget. Budget can have up to five notifications."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notifications: Option<serde_json::Value>,
    #[doc = "The forecasted cost which is being tracked for a budget."]
    #[serde(rename = "forecastSpend", default, skip_serializing_if = "Option::is_none")]
    pub forecast_spend: Option<ForecastSpend>,
}
impl BudgetProperties {
    pub fn new(
        category: budget_properties::Category,
        amount: f64,
        time_grain: budget_properties::TimeGrain,
        time_period: BudgetTimePeriod,
    ) -> Self {
        Self {
            category,
            amount,
            time_grain,
            time_period,
            filter: None,
            current_spend: None,
            notifications: None,
            forecast_spend: None,
        }
    }
}
pub mod budget_properties {
    use super::*;
    #[doc = "The category of the budget, whether the budget tracks cost or usage."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Category")]
    pub enum Category {
        Cost,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Category {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Category {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Category {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Cost => serializer.serialize_unit_variant("Category", 0u32, "Cost"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The time covered by a budget. Tracking of the amount will be reset based on the time grain. BillingMonth, BillingQuarter, and BillingAnnual are only supported by WD customers"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TimeGrain")]
    pub enum TimeGrain {
        Monthly,
        Quarterly,
        Annually,
        BillingMonth,
        BillingQuarter,
        BillingAnnual,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TimeGrain {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TimeGrain {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TimeGrain {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Monthly => serializer.serialize_unit_variant("TimeGrain", 0u32, "Monthly"),
                Self::Quarterly => serializer.serialize_unit_variant("TimeGrain", 1u32, "Quarterly"),
                Self::Annually => serializer.serialize_unit_variant("TimeGrain", 2u32, "Annually"),
                Self::BillingMonth => serializer.serialize_unit_variant("TimeGrain", 3u32, "BillingMonth"),
                Self::BillingQuarter => serializer.serialize_unit_variant("TimeGrain", 4u32, "BillingQuarter"),
                Self::BillingAnnual => serializer.serialize_unit_variant("TimeGrain", 5u32, "BillingAnnual"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The start and end date for a budget."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BudgetTimePeriod {
    #[doc = "The start date for the budget."]
    #[serde(rename = "startDate", with = "azure_core::date::rfc3339")]
    pub start_date: time::OffsetDateTime,
    #[doc = "The end date for the budget. If not provided, we default this to 10 years from the start date."]
    #[serde(rename = "endDate", with = "azure_core::date::rfc3339::option")]
    pub end_date: Option<time::OffsetDateTime>,
}
impl BudgetTimePeriod {
    pub fn new(start_date: time::OffsetDateTime) -> Self {
        Self {
            start_date,
            end_date: None,
        }
    }
}
#[doc = "Result of listing budgets. It contains a list of available budgets in the scope provided."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BudgetsListResult {
    #[doc = "The list of budgets."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Budget>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BudgetsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BudgetsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A charge summary resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChargeSummary {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Specifies the kind of charge summary."]
    pub kind: charge_summary::Kind,
}
impl ChargeSummary {
    pub fn new(kind: charge_summary::Kind) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            kind,
        }
    }
}
pub mod charge_summary {
    use super::*;
    #[doc = "Specifies the kind of charge summary."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        #[serde(rename = "legacy")]
        Legacy,
        #[serde(rename = "modern")]
        Modern,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Legacy => serializer.serialize_unit_variant("Kind", 0u32, "legacy"),
                Self::Modern => serializer.serialize_unit_variant("Kind", 1u32, "modern"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Result of listing charge summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChargesListResult {
    #[doc = "The list of charge summary"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ChargeSummary>,
}
impl ChargesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Summary of credit balances."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreditBalanceSummary {
    #[doc = "The amount plus currency ."]
    #[serde(rename = "estimatedBalance", default, skip_serializing_if = "Option::is_none")]
    pub estimated_balance: Option<Amount>,
    #[doc = "The amount plus currency ."]
    #[serde(rename = "currentBalance", default, skip_serializing_if = "Option::is_none")]
    pub current_balance: Option<Amount>,
    #[doc = "The amount with exchange rate."]
    #[serde(rename = "estimatedBalanceInBillingCurrency", default, skip_serializing_if = "Option::is_none")]
    pub estimated_balance_in_billing_currency: Option<AmountWithExchangeRate>,
}
impl CreditBalanceSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A credit summary resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreditSummary {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of the credit summary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CreditSummaryProperties>,
}
impl CreditSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the credit summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreditSummaryProperties {
    #[doc = "Summary of credit balances."]
    #[serde(rename = "balanceSummary", default, skip_serializing_if = "Option::is_none")]
    pub balance_summary: Option<CreditBalanceSummary>,
    #[doc = "The amount plus currency ."]
    #[serde(rename = "pendingCreditAdjustments", default, skip_serializing_if = "Option::is_none")]
    pub pending_credit_adjustments: Option<Amount>,
    #[doc = "The amount plus currency ."]
    #[serde(rename = "expiredCredit", default, skip_serializing_if = "Option::is_none")]
    pub expired_credit: Option<Amount>,
    #[doc = "The amount plus currency ."]
    #[serde(rename = "pendingEligibleCharges", default, skip_serializing_if = "Option::is_none")]
    pub pending_eligible_charges: Option<Amount>,
    #[doc = "The credit currency."]
    #[serde(rename = "creditCurrency", default, skip_serializing_if = "Option::is_none")]
    pub credit_currency: Option<String>,
    #[doc = "The billing currency."]
    #[serde(rename = "billingCurrency", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency: Option<String>,
    #[doc = "The reseller properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reseller: Option<Reseller>,
    #[doc = "The eTag for the resource."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
impl CreditSummaryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The current amount of cost which is being tracked for a budget."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CurrentSpend {
    #[doc = "The total amount of cost which is being tracked by the budget."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[doc = "The unit of measure for the budget amount."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
impl CurrentSpend {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the price sheet download."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DownloadProperties {
    #[doc = "The link (url) to download the pricesheet."]
    #[serde(rename = "downloadUrl", default, skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[doc = "Download link validity."]
    #[serde(rename = "validTill", default, skip_serializing_if = "Option::is_none")]
    pub valid_till: Option<String>,
}
impl DownloadProperties {
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
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicates that the service is not able to process the incoming request. The reason is provided in the error message. \n\nSome Error responses: \n\n * 429 TooManyRequests - Request is throttled. Retry after waiting for the time specified in the \"x-ms-ratelimit-microsoft.consumption-retry-after\" header. \n\n * 503 ServiceUnavailable - Service is temporarily unavailable. Retry after waiting for the time specified in the \"Retry-After\" header."]
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
#[doc = "The event properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventProperties {
    #[doc = "The date of the event."]
    #[serde(rename = "transactionDate", with = "azure_core::date::rfc3339::option")]
    pub transaction_date: Option<time::OffsetDateTime>,
    #[doc = "The description of the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The amount plus currency ."]
    #[serde(rename = "newCredit", default, skip_serializing_if = "Option::is_none")]
    pub new_credit: Option<Amount>,
    #[doc = "The amount plus currency ."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adjustments: Option<Amount>,
    #[doc = "The amount plus currency ."]
    #[serde(rename = "creditExpired", default, skip_serializing_if = "Option::is_none")]
    pub credit_expired: Option<Amount>,
    #[doc = "The amount plus currency ."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub charges: Option<Amount>,
    #[doc = "The amount plus currency ."]
    #[serde(rename = "closedBalance", default, skip_serializing_if = "Option::is_none")]
    pub closed_balance: Option<Amount>,
    #[doc = "Identifies the type of the event."]
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<event_properties::EventType>,
    #[doc = "The number which uniquely identifies the invoice on which the event was billed. This will be empty for unbilled events."]
    #[serde(rename = "invoiceNumber", default, skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<String>,
    #[doc = "The ID that uniquely identifies the billing profile for which the event happened. The property is only available for billing account of type MicrosoftCustomerAgreement. "]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "The display name of the billing profile for which the event happened. The property is only available for billing account of type MicrosoftCustomerAgreement."]
    #[serde(rename = "billingProfileDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_display_name: Option<String>,
    #[doc = "The ID that uniquely identifies the lot for which the event happened."]
    #[serde(rename = "lotId", default, skip_serializing_if = "Option::is_none")]
    pub lot_id: Option<String>,
    #[doc = "Identifies the source of the lot for which the event happened. "]
    #[serde(rename = "lotSource", default, skip_serializing_if = "Option::is_none")]
    pub lot_source: Option<String>,
    #[doc = "The amount plus currency ."]
    #[serde(rename = "canceledCredit", default, skip_serializing_if = "Option::is_none")]
    pub canceled_credit: Option<Amount>,
    #[doc = "The credit currency of the event."]
    #[serde(rename = "creditCurrency", default, skip_serializing_if = "Option::is_none")]
    pub credit_currency: Option<String>,
    #[doc = "The billing currency of the event."]
    #[serde(rename = "billingCurrency", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency: Option<String>,
    #[doc = "The reseller properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reseller: Option<Reseller>,
    #[doc = "The amount with exchange rate."]
    #[serde(rename = "creditExpiredInBillingCurrency", default, skip_serializing_if = "Option::is_none")]
    pub credit_expired_in_billing_currency: Option<AmountWithExchangeRate>,
    #[doc = "The amount with exchange rate."]
    #[serde(rename = "newCreditInBillingCurrency", default, skip_serializing_if = "Option::is_none")]
    pub new_credit_in_billing_currency: Option<AmountWithExchangeRate>,
    #[doc = "The amount with exchange rate."]
    #[serde(rename = "adjustmentsInBillingCurrency", default, skip_serializing_if = "Option::is_none")]
    pub adjustments_in_billing_currency: Option<AmountWithExchangeRate>,
    #[doc = "The amount with exchange rate."]
    #[serde(rename = "chargesInBillingCurrency", default, skip_serializing_if = "Option::is_none")]
    pub charges_in_billing_currency: Option<AmountWithExchangeRate>,
    #[doc = "The amount with exchange rate."]
    #[serde(rename = "closedBalanceInBillingCurrency", default, skip_serializing_if = "Option::is_none")]
    pub closed_balance_in_billing_currency: Option<AmountWithExchangeRate>,
    #[doc = "The eTag for the resource."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
impl EventProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod event_properties {
    use super::*;
    #[doc = "Identifies the type of the event."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EventType")]
    pub enum EventType {
        SettledCharges,
        PendingCharges,
        PendingAdjustments,
        PendingNewCredit,
        PendingExpiredCredit,
        UnKnown,
        NewCredit,
        CreditExpired,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EventType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EventType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EventType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::SettledCharges => serializer.serialize_unit_variant("EventType", 0u32, "SettledCharges"),
                Self::PendingCharges => serializer.serialize_unit_variant("EventType", 1u32, "PendingCharges"),
                Self::PendingAdjustments => serializer.serialize_unit_variant("EventType", 2u32, "PendingAdjustments"),
                Self::PendingNewCredit => serializer.serialize_unit_variant("EventType", 3u32, "PendingNewCredit"),
                Self::PendingExpiredCredit => serializer.serialize_unit_variant("EventType", 4u32, "PendingExpiredCredit"),
                Self::UnKnown => serializer.serialize_unit_variant("EventType", 5u32, "UnKnown"),
                Self::NewCredit => serializer.serialize_unit_variant("EventType", 6u32, "NewCredit"),
                Self::CreditExpired => serializer.serialize_unit_variant("EventType", 7u32, "CreditExpired"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An event summary resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventSummary {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The event properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EventProperties>,
}
impl EventSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of listing event summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Events {
    #[doc = "The list of event summary."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EventSummary>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for Events {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl Events {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The forecasted cost which is being tracked for a budget."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ForecastSpend {
    #[doc = "The forecasted cost for the total time period which is being tracked by the budget. This value is only provided if the budget contains a forecast alert type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[doc = "The unit of measure for the budget amount."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
impl ForecastSpend {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of the error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HighCasedErrorDetails {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl HighCasedErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response indicates that the service is not able to process the incoming request. The reason is provided in the error message. \n\nSome Error responses: \n\n * 429 TooManyRequests - Request is throttled. Retry after waiting for the time specified in the \"x-ms-ratelimit-microsoft.consumption-retry-after\" header. \n\n * 503 ServiceUnavailable - Service is temporarily unavailable. Retry after waiting for the time specified in the \"Retry-After\" header."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HighCasedErrorResponse {
    #[doc = "The details of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<HighCasedErrorDetails>,
}
impl HighCasedErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Legacy charge summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LegacyChargeSummary {
    #[serde(flatten)]
    pub charge_summary: ChargeSummary,
    #[doc = "The properties of legacy charge summary."]
    pub properties: LegacyChargeSummaryProperties,
}
impl LegacyChargeSummary {
    pub fn new(charge_summary: ChargeSummary, properties: LegacyChargeSummaryProperties) -> Self {
        Self {
            charge_summary,
            properties,
        }
    }
}
#[doc = "The properties of legacy charge summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LegacyChargeSummaryProperties {
    #[doc = "The id of the billing period resource that the charge belongs to."]
    #[serde(rename = "billingPeriodId", default, skip_serializing_if = "Option::is_none")]
    pub billing_period_id: Option<String>,
    #[doc = "Usage start date."]
    #[serde(rename = "usageStart", default, skip_serializing_if = "Option::is_none")]
    pub usage_start: Option<String>,
    #[doc = "Usage end date."]
    #[serde(rename = "usageEnd", default, skip_serializing_if = "Option::is_none")]
    pub usage_end: Option<String>,
    #[doc = "Azure Charges."]
    #[serde(rename = "azureCharges", default, skip_serializing_if = "Option::is_none")]
    pub azure_charges: Option<f64>,
    #[doc = "Charges Billed separately."]
    #[serde(rename = "chargesBilledSeparately", default, skip_serializing_if = "Option::is_none")]
    pub charges_billed_separately: Option<f64>,
    #[doc = "Marketplace Charges."]
    #[serde(rename = "marketplaceCharges", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_charges: Option<f64>,
    #[doc = "Currency Code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}
impl LegacyChargeSummaryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Legacy reservation recommendation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LegacyReservationRecommendation {
    #[serde(flatten)]
    pub reservation_recommendation: ReservationRecommendation,
    #[doc = "The properties of the reservation recommendation."]
    pub properties: LegacyReservationRecommendationProperties,
}
impl LegacyReservationRecommendation {
    pub fn new(reservation_recommendation: ReservationRecommendation, properties: LegacyReservationRecommendationProperties) -> Self {
        Self {
            reservation_recommendation,
            properties,
        }
    }
}
#[doc = "The properties of the reservation recommendation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LegacyReservationRecommendationProperties {
    #[doc = "The number of days of usage to look back for recommendation."]
    #[serde(rename = "lookBackPeriod", default, skip_serializing_if = "Option::is_none")]
    pub look_back_period: Option<String>,
    #[doc = "The instance Flexibility Ratio."]
    #[serde(rename = "instanceFlexibilityRatio", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility_ratio: Option<f64>,
    #[doc = "The instance Flexibility Group."]
    #[serde(rename = "instanceFlexibilityGroup", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility_group: Option<String>,
    #[doc = "The normalized Size."]
    #[serde(rename = "normalizedSize", default, skip_serializing_if = "Option::is_none")]
    pub normalized_size: Option<String>,
    #[doc = "The recommended Quantity Normalized."]
    #[serde(rename = "recommendedQuantityNormalized", default, skip_serializing_if = "Option::is_none")]
    pub recommended_quantity_normalized: Option<f64>,
    #[doc = "The meter id (GUID)"]
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[doc = "The azure resource type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "RI recommendations in one or three year terms."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    #[doc = "The total amount of cost without reserved instances."]
    #[serde(rename = "costWithNoReservedInstances", default, skip_serializing_if = "Option::is_none")]
    pub cost_with_no_reserved_instances: Option<f64>,
    #[doc = "Recommended quality for reserved instances."]
    #[serde(rename = "recommendedQuantity", default, skip_serializing_if = "Option::is_none")]
    pub recommended_quantity: Option<f64>,
    #[doc = "The total amount of cost with reserved instances."]
    #[serde(rename = "totalCostWithReservedInstances", default, skip_serializing_if = "Option::is_none")]
    pub total_cost_with_reserved_instances: Option<f64>,
    #[doc = "Total estimated savings with reserved instances."]
    #[serde(rename = "netSavings", default, skip_serializing_if = "Option::is_none")]
    pub net_savings: Option<f64>,
    #[doc = "The usage date for looking back."]
    #[serde(rename = "firstUsageDate", with = "azure_core::date::rfc3339::option")]
    pub first_usage_date: Option<time::OffsetDateTime>,
    #[doc = "Shared or single recommendation."]
    pub scope: String,
    #[doc = "List of sku properties"]
    #[serde(rename = "skuProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub sku_properties: Vec<SkuProperty>,
}
impl LegacyReservationRecommendationProperties {
    pub fn new(scope: String) -> Self {
        Self {
            look_back_period: None,
            instance_flexibility_ratio: None,
            instance_flexibility_group: None,
            normalized_size: None,
            recommended_quantity_normalized: None,
            meter_id: None,
            resource_type: None,
            term: None,
            cost_with_no_reserved_instances: None,
            recommended_quantity: None,
            total_cost_with_reserved_instances: None,
            net_savings: None,
            first_usage_date: None,
            scope,
            sku_properties: Vec::new(),
        }
    }
}
#[doc = "Legacy Reservation transaction resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LegacyReservationTransaction {
    #[serde(flatten)]
    pub reservation_transaction: ReservationTransaction,
    #[doc = "The properties of a legacy reservation transaction."]
    pub properties: LegacyReservationTransactionProperties,
}
impl LegacyReservationTransaction {
    pub fn new(properties: LegacyReservationTransactionProperties) -> Self {
        Self {
            reservation_transaction: ReservationTransaction::default(),
            properties,
        }
    }
}
#[doc = "The properties of a legacy reservation transaction."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LegacyReservationTransactionProperties {
    #[doc = "The date of the transaction"]
    #[serde(rename = "eventDate", with = "azure_core::date::rfc3339::option")]
    pub event_date: Option<time::OffsetDateTime>,
    #[doc = "The reservation order ID is the identifier for a reservation purchase. Each reservation order ID represents a single purchase transaction. A reservation order contains reservations. The reservation order specifies the VM size and region for the reservations."]
    #[serde(rename = "reservationOrderId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order_id: Option<String>,
    #[doc = "The description of the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The type of the transaction (Purchase, Cancel or Refund)."]
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[doc = "The quantity of the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    #[doc = "The charge of the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[doc = "The ISO currency in which the transaction is charged, for example, USD."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The name of the reservation order."]
    #[serde(rename = "reservationOrderName", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order_name: Option<String>,
    #[doc = "The purchasing enrollment."]
    #[serde(rename = "purchasingEnrollment", default, skip_serializing_if = "Option::is_none")]
    pub purchasing_enrollment: Option<String>,
    #[doc = "The subscription guid that makes the transaction."]
    #[serde(rename = "purchasingSubscriptionGuid", default, skip_serializing_if = "Option::is_none")]
    pub purchasing_subscription_guid: Option<String>,
    #[doc = "The subscription name that makes the transaction."]
    #[serde(rename = "purchasingSubscriptionName", default, skip_serializing_if = "Option::is_none")]
    pub purchasing_subscription_name: Option<String>,
    #[doc = "This is the ARM Sku name. It can be used to join with the serviceType field in additional info in usage records."]
    #[serde(rename = "armSkuName", default, skip_serializing_if = "Option::is_none")]
    pub arm_sku_name: Option<String>,
    #[doc = "This is the term of the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    #[doc = "The region of the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "The name of the account that makes the transaction."]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "The email of the account owner that makes the transaction."]
    #[serde(rename = "accountOwnerEmail", default, skip_serializing_if = "Option::is_none")]
    pub account_owner_email: Option<String>,
    #[doc = "The department name."]
    #[serde(rename = "departmentName", default, skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    #[doc = "The cost center of this department if it is a department and a cost center is provided."]
    #[serde(rename = "costCenter", default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[doc = "The current enrollment."]
    #[serde(rename = "currentEnrollment", default, skip_serializing_if = "Option::is_none")]
    pub current_enrollment: Option<String>,
    #[doc = "The billing frequency, which can be either one-time or recurring."]
    #[serde(rename = "billingFrequency", default, skip_serializing_if = "Option::is_none")]
    pub billing_frequency: Option<String>,
    #[doc = "The billing month(yyyyMMdd), on which the event initiated."]
    #[serde(rename = "billingMonth", default, skip_serializing_if = "Option::is_none")]
    pub billing_month: Option<i32>,
    #[doc = "The monetary commitment amount at the enrollment scope."]
    #[serde(rename = "monetaryCommitment", default, skip_serializing_if = "Option::is_none")]
    pub monetary_commitment: Option<f64>,
    #[doc = "The overage amount at the enrollment scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overage: Option<f64>,
}
impl LegacyReservationTransactionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the legacy reservation recommendation for shared scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LegacySharedScopeReservationRecommendationProperties {
    #[serde(flatten)]
    pub legacy_reservation_recommendation_properties: LegacyReservationRecommendationProperties,
}
impl LegacySharedScopeReservationRecommendationProperties {
    pub fn new(legacy_reservation_recommendation_properties: LegacyReservationRecommendationProperties) -> Self {
        Self {
            legacy_reservation_recommendation_properties,
        }
    }
}
#[doc = "The properties of the legacy reservation recommendation for single scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LegacySingleScopeReservationRecommendationProperties {
    #[serde(flatten)]
    pub legacy_reservation_recommendation_properties: LegacyReservationRecommendationProperties,
    #[doc = "Subscription id associated with single scoped recommendation."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}
impl LegacySingleScopeReservationRecommendationProperties {
    pub fn new(legacy_reservation_recommendation_properties: LegacyReservationRecommendationProperties) -> Self {
        Self {
            legacy_reservation_recommendation_properties,
            subscription_id: None,
        }
    }
}
#[doc = "Legacy usage detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LegacyUsageDetail {
    #[serde(flatten)]
    pub usage_detail: UsageDetail,
    #[doc = "The properties of the legacy usage detail."]
    pub properties: LegacyUsageDetailProperties,
}
impl LegacyUsageDetail {
    pub fn new(usage_detail: UsageDetail, properties: LegacyUsageDetailProperties) -> Self {
        Self { usage_detail, properties }
    }
}
#[doc = "The properties of the legacy usage detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LegacyUsageDetailProperties {
    #[doc = "Billing Account identifier."]
    #[serde(rename = "billingAccountId", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_id: Option<String>,
    #[doc = "Billing Account Name."]
    #[serde(rename = "billingAccountName", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_name: Option<String>,
    #[doc = "The billing period start date."]
    #[serde(rename = "billingPeriodStartDate", with = "azure_core::date::rfc3339::option")]
    pub billing_period_start_date: Option<time::OffsetDateTime>,
    #[doc = "The billing period end date."]
    #[serde(rename = "billingPeriodEndDate", with = "azure_core::date::rfc3339::option")]
    pub billing_period_end_date: Option<time::OffsetDateTime>,
    #[doc = "Billing Profile identifier."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "Billing Profile Name."]
    #[serde(rename = "billingProfileName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_name: Option<String>,
    #[doc = "Account Owner Id."]
    #[serde(rename = "accountOwnerId", default, skip_serializing_if = "Option::is_none")]
    pub account_owner_id: Option<String>,
    #[doc = "Account Name."]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "Subscription guid."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Subscription name."]
    #[serde(rename = "subscriptionName", default, skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
    #[doc = "Date for the usage record."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub date: Option<time::OffsetDateTime>,
    #[doc = "Product name for the consumed service or purchase. Not available for Marketplace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[doc = "Part Number of the service used. Can be used to join with the price sheet. Not available for marketplace."]
    #[serde(rename = "partNumber", default, skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
    #[doc = "The meter id (GUID). Not available for marketplace. For reserved instance this represents the primary meter for which the reservation was purchased. For the actual VM Size for which the reservation is purchased see productOrderName."]
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[doc = "The properties of the meter detail."]
    #[serde(rename = "meterDetails", default, skip_serializing_if = "Option::is_none")]
    pub meter_details: Option<MeterDetailsResponse>,
    #[doc = "The usage quantity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    #[doc = "Effective Price that's charged for the usage."]
    #[serde(rename = "effectivePrice", default, skip_serializing_if = "Option::is_none")]
    pub effective_price: Option<f64>,
    #[doc = "The amount of cost before tax."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost: Option<f64>,
    #[doc = "Unit Price is the price applicable to you. (your EA or other contract price)."]
    #[serde(rename = "unitPrice", default, skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<f64>,
    #[doc = "Billing Currency."]
    #[serde(rename = "billingCurrency", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency: Option<String>,
    #[doc = "Resource Location."]
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[doc = "Consumed service name. Name of the azure resource provider that emits the usage or was purchased. This value is not provided for marketplace usage."]
    #[serde(rename = "consumedService", default, skip_serializing_if = "Option::is_none")]
    pub consumed_service: Option<String>,
    #[doc = "Unique identifier of the Azure Resource Manager usage detail resource."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Resource Name."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "Service-specific metadata."]
    #[serde(rename = "serviceInfo1", default, skip_serializing_if = "Option::is_none")]
    pub service_info1: Option<String>,
    #[doc = "Legacy field with optional service-specific metadata."]
    #[serde(rename = "serviceInfo2", default, skip_serializing_if = "Option::is_none")]
    pub service_info2: Option<String>,
    #[doc = "Additional details of this usage item. By default this is not populated, unless it's specified in $expand. Use this field to get usage line item specific details such as the actual VM Size (ServiceType) or the ratio in which the reservation discount is applied."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
    #[doc = "Invoice Section Name."]
    #[serde(rename = "invoiceSection", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section: Option<String>,
    #[doc = "The cost center of this department if it is a department and a cost center is provided."]
    #[serde(rename = "costCenter", default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[doc = "Resource Group Name."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "ARM resource id of the reservation. Only applies to records relevant to reservations."]
    #[serde(rename = "reservationId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    #[doc = "User provided display name of the reservation. Last known name for a particular day is populated in the daily data. Only applies to records relevant to reservations."]
    #[serde(rename = "reservationName", default, skip_serializing_if = "Option::is_none")]
    pub reservation_name: Option<String>,
    #[doc = "Product Order Id. For reservations this is the Reservation Order ID."]
    #[serde(rename = "productOrderId", default, skip_serializing_if = "Option::is_none")]
    pub product_order_id: Option<String>,
    #[doc = "Product Order Name. For reservations this is the SKU that was purchased."]
    #[serde(rename = "productOrderName", default, skip_serializing_if = "Option::is_none")]
    pub product_order_name: Option<String>,
    #[doc = "Offer Id. Ex: MS-AZR-0017P, MS-AZR-0148P."]
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[doc = "Is Azure Credit Eligible."]
    #[serde(rename = "isAzureCreditEligible", default, skip_serializing_if = "Option::is_none")]
    pub is_azure_credit_eligible: Option<bool>,
    #[doc = "Term (in months). 1 month for monthly recurring purchase. 12 months for a 1 year reservation. 36 months for a 3 year reservation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    #[doc = "Publisher Name."]
    #[serde(rename = "publisherName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_name: Option<String>,
    #[doc = "Publisher Type."]
    #[serde(rename = "publisherType", default, skip_serializing_if = "Option::is_none")]
    pub publisher_type: Option<String>,
    #[doc = "Plan Name."]
    #[serde(rename = "planName", default, skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[doc = "Indicates a charge represents credits, usage, a Marketplace purchase, a reservation fee, or a refund."]
    #[serde(rename = "chargeType", default, skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    #[doc = "Indicates how frequently this charge will occur. OneTime for purchases which only happen once, Monthly for fees which recur every month, and UsageBased for charges based on how much a service is used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    #[doc = "Retail price for the resource."]
    #[serde(rename = "payGPrice", default, skip_serializing_if = "Option::is_none")]
    pub pay_g_price: Option<f64>,
    #[doc = "Unique identifier for the applicable benefit."]
    #[serde(rename = "benefitId", default, skip_serializing_if = "Option::is_none")]
    pub benefit_id: Option<String>,
    #[doc = "Name of the applicable benefit."]
    #[serde(rename = "benefitName", default, skip_serializing_if = "Option::is_none")]
    pub benefit_name: Option<String>,
    #[doc = "Identifier that indicates how the meter is priced."]
    #[serde(rename = "pricingModel", default, skip_serializing_if = "Option::is_none")]
    pub pricing_model: Option<legacy_usage_detail_properties::PricingModel>,
}
impl LegacyUsageDetailProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod legacy_usage_detail_properties {
    use super::*;
    #[doc = "Identifier that indicates how the meter is priced."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PricingModel")]
    pub enum PricingModel {
        #[serde(rename = "On Demand")]
        OnDemand,
        Reservation,
        Spot,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PricingModel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PricingModel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PricingModel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::OnDemand => serializer.serialize_unit_variant("PricingModel", 0u32, "On Demand"),
                Self::Reservation => serializer.serialize_unit_variant("PricingModel", 1u32, "Reservation"),
                Self::Spot => serializer.serialize_unit_variant("PricingModel", 2u32, "Spot"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The lot properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LotProperties {
    #[doc = "The amount plus currency ."]
    #[serde(rename = "originalAmount", default, skip_serializing_if = "Option::is_none")]
    pub original_amount: Option<Amount>,
    #[doc = "The amount plus currency ."]
    #[serde(rename = "closedBalance", default, skip_serializing_if = "Option::is_none")]
    pub closed_balance: Option<Amount>,
    #[doc = "The source of the lot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<lot_properties::Source>,
    #[doc = "The date when the lot became effective."]
    #[serde(rename = "startDate", with = "azure_core::date::rfc3339::option")]
    pub start_date: Option<time::OffsetDateTime>,
    #[doc = "The expiration date of a lot."]
    #[serde(rename = "expirationDate", with = "azure_core::date::rfc3339::option")]
    pub expiration_date: Option<time::OffsetDateTime>,
    #[doc = "The po number of the invoice on which the lot was added. This property is not available for ConsumptionCommitment lots."]
    #[serde(rename = "poNumber", default, skip_serializing_if = "Option::is_none")]
    pub po_number: Option<String>,
    #[doc = "The date when the lot was added."]
    #[serde(rename = "purchasedDate", with = "azure_core::date::rfc3339::option")]
    pub purchased_date: Option<time::OffsetDateTime>,
    #[doc = "The status of the lot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<lot_properties::Status>,
    #[doc = "The currency of the lot."]
    #[serde(rename = "creditCurrency", default, skip_serializing_if = "Option::is_none")]
    pub credit_currency: Option<String>,
    #[doc = "The billing currency of the lot."]
    #[serde(rename = "billingCurrency", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency: Option<String>,
    #[doc = "The amount with exchange rate."]
    #[serde(rename = "originalAmountInBillingCurrency", default, skip_serializing_if = "Option::is_none")]
    pub original_amount_in_billing_currency: Option<AmountWithExchangeRate>,
    #[doc = "The amount with exchange rate."]
    #[serde(rename = "closedBalanceInBillingCurrency", default, skip_serializing_if = "Option::is_none")]
    pub closed_balance_in_billing_currency: Option<AmountWithExchangeRate>,
    #[doc = "The reseller properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reseller: Option<Reseller>,
    #[doc = "The eTag for the resource."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
impl LotProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod lot_properties {
    use super::*;
    #[doc = "The source of the lot."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Source")]
    pub enum Source {
        PurchasedCredit,
        PromotionalCredit,
        ConsumptionCommitment,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Source {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Source {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Source {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PurchasedCredit => serializer.serialize_unit_variant("Source", 0u32, "PurchasedCredit"),
                Self::PromotionalCredit => serializer.serialize_unit_variant("Source", 1u32, "PromotionalCredit"),
                Self::ConsumptionCommitment => serializer.serialize_unit_variant("Source", 2u32, "ConsumptionCommitment"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The status of the lot."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        None,
        Active,
        Inactive,
        Expired,
        Complete,
        Canceled,
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
                Self::None => serializer.serialize_unit_variant("Status", 0u32, "None"),
                Self::Active => serializer.serialize_unit_variant("Status", 1u32, "Active"),
                Self::Inactive => serializer.serialize_unit_variant("Status", 2u32, "Inactive"),
                Self::Expired => serializer.serialize_unit_variant("Status", 3u32, "Expired"),
                Self::Complete => serializer.serialize_unit_variant("Status", 4u32, "Complete"),
                Self::Canceled => serializer.serialize_unit_variant("Status", 5u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A lot summary resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LotSummary {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The lot properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LotProperties>,
}
impl LotSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of listing lot summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Lots {
    #[doc = "The list of lot summary."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LotSummary>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for Lots {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl Lots {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the Management Group Aggregated Cost."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroupAggregatedCostProperties {
    #[doc = "The id of the billing period resource that the aggregated cost belongs to."]
    #[serde(rename = "billingPeriodId", default, skip_serializing_if = "Option::is_none")]
    pub billing_period_id: Option<String>,
    #[doc = "The start of the date time range covered by aggregated cost."]
    #[serde(rename = "usageStart", with = "azure_core::date::rfc3339::option")]
    pub usage_start: Option<time::OffsetDateTime>,
    #[doc = "The end of the date time range covered by the aggregated cost."]
    #[serde(rename = "usageEnd", with = "azure_core::date::rfc3339::option")]
    pub usage_end: Option<time::OffsetDateTime>,
    #[doc = "Azure Charges."]
    #[serde(rename = "azureCharges", default, skip_serializing_if = "Option::is_none")]
    pub azure_charges: Option<f64>,
    #[doc = "Marketplace Charges."]
    #[serde(rename = "marketplaceCharges", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_charges: Option<f64>,
    #[doc = "Charges Billed Separately."]
    #[serde(rename = "chargesBilledSeparately", default, skip_serializing_if = "Option::is_none")]
    pub charges_billed_separately: Option<f64>,
    #[doc = "The ISO currency in which the meter is charged, for example, USD."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "Children of a management group"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<ManagementGroupAggregatedCostResult>,
    #[doc = "List of subscription Guids included in the calculation of aggregated cost"]
    #[serde(rename = "includedSubscriptions", default, skip_serializing_if = "Vec::is_empty")]
    pub included_subscriptions: Vec<String>,
    #[doc = "List of subscription Guids excluded from the calculation of aggregated cost"]
    #[serde(rename = "excludedSubscriptions", default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_subscriptions: Vec<String>,
}
impl ManagementGroupAggregatedCostProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A management group aggregated cost resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroupAggregatedCostResult {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the Management Group Aggregated Cost."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagementGroupAggregatedCostProperties>,
}
impl ManagementGroupAggregatedCostResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A marketplace resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Marketplace {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the marketplace usage detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MarketplaceProperties>,
}
impl Marketplace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the marketplace usage detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketplaceProperties {
    #[doc = "The id of the billing period resource that the usage belongs to."]
    #[serde(rename = "billingPeriodId", default, skip_serializing_if = "Option::is_none")]
    pub billing_period_id: Option<String>,
    #[doc = "The start of the date time range covered by the usage detail."]
    #[serde(rename = "usageStart", with = "azure_core::date::rfc3339::option")]
    pub usage_start: Option<time::OffsetDateTime>,
    #[doc = "The end of the date time range covered by the usage detail."]
    #[serde(rename = "usageEnd", with = "azure_core::date::rfc3339::option")]
    pub usage_end: Option<time::OffsetDateTime>,
    #[doc = "The marketplace resource rate."]
    #[serde(rename = "resourceRate", default, skip_serializing_if = "Option::is_none")]
    pub resource_rate: Option<f64>,
    #[doc = "The type of offer."]
    #[serde(rename = "offerName", default, skip_serializing_if = "Option::is_none")]
    pub offer_name: Option<String>,
    #[doc = "The name of resource group."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "Additional information."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
    #[doc = "The order number."]
    #[serde(rename = "orderNumber", default, skip_serializing_if = "Option::is_none")]
    pub order_number: Option<String>,
    #[doc = "The name of the resource instance that the usage is about."]
    #[serde(rename = "instanceName", default, skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[doc = "The uri of the resource instance that the usage is about."]
    #[serde(rename = "instanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[doc = "The ISO currency in which the meter is charged, for example, USD."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The quantity of usage."]
    #[serde(rename = "consumedQuantity", default, skip_serializing_if = "Option::is_none")]
    pub consumed_quantity: Option<f64>,
    #[doc = "The unit of measure."]
    #[serde(rename = "unitOfMeasure", default, skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<String>,
    #[doc = "The amount of cost before tax."]
    #[serde(rename = "pretaxCost", default, skip_serializing_if = "Option::is_none")]
    pub pretax_cost: Option<f64>,
    #[doc = "The estimated usage is subject to change."]
    #[serde(rename = "isEstimated", default, skip_serializing_if = "Option::is_none")]
    pub is_estimated: Option<bool>,
    #[doc = "The meter id (GUID)."]
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[doc = "Subscription guid."]
    #[serde(rename = "subscriptionGuid", default, skip_serializing_if = "Option::is_none")]
    pub subscription_guid: Option<String>,
    #[doc = "Subscription name."]
    #[serde(rename = "subscriptionName", default, skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
    #[doc = "Account name."]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "Department name."]
    #[serde(rename = "departmentName", default, skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    #[doc = "Consumed service name."]
    #[serde(rename = "consumedService", default, skip_serializing_if = "Option::is_none")]
    pub consumed_service: Option<String>,
    #[doc = "The cost center of this department if it is a department and a costcenter exists"]
    #[serde(rename = "costCenter", default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[doc = "Additional details of this usage item. By default this is not populated, unless it's specified in $expand."]
    #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<String>,
    #[doc = "The name of publisher."]
    #[serde(rename = "publisherName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_name: Option<String>,
    #[doc = "The name of plan."]
    #[serde(rename = "planName", default, skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[doc = "Flag indicating whether this is a recurring charge or not."]
    #[serde(rename = "isRecurringCharge", default, skip_serializing_if = "Option::is_none")]
    pub is_recurring_charge: Option<bool>,
}
impl MarketplaceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of listing marketplaces. It contains a list of available marketplaces in reverse chronological order by billing period."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketplacesListResult {
    #[doc = "The list of marketplaces."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Marketplace>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MarketplacesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl MarketplacesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the meter detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MeterDetails {
    #[doc = "The name of the meter, within the given meter category"]
    #[serde(rename = "meterName", default, skip_serializing_if = "Option::is_none")]
    pub meter_name: Option<String>,
    #[doc = "The category of the meter, for example, 'Cloud services', 'Networking', etc.."]
    #[serde(rename = "meterCategory", default, skip_serializing_if = "Option::is_none")]
    pub meter_category: Option<String>,
    #[doc = "The subcategory of the meter, for example, 'A6 Cloud services', 'ExpressRoute (IXP)', etc.."]
    #[serde(rename = "meterSubCategory", default, skip_serializing_if = "Option::is_none")]
    pub meter_sub_category: Option<String>,
    #[doc = "The unit in which the meter consumption is charged, for example, 'Hours', 'GB', etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The location in which the Azure service is available."]
    #[serde(rename = "meterLocation", default, skip_serializing_if = "Option::is_none")]
    pub meter_location: Option<String>,
    #[doc = "The total included quantity associated with the offer."]
    #[serde(rename = "totalIncludedQuantity", default, skip_serializing_if = "Option::is_none")]
    pub total_included_quantity: Option<f64>,
    #[doc = "The pretax listing price."]
    #[serde(rename = "pretaxStandardRate", default, skip_serializing_if = "Option::is_none")]
    pub pretax_standard_rate: Option<f64>,
    #[doc = "The name of the service."]
    #[serde(rename = "serviceName", default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[doc = "The service tier."]
    #[serde(rename = "serviceTier", default, skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,
}
impl MeterDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the meter detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MeterDetailsResponse {
    #[doc = "The name of the meter, within the given meter category"]
    #[serde(rename = "meterName", default, skip_serializing_if = "Option::is_none")]
    pub meter_name: Option<String>,
    #[doc = "The category of the meter, for example, 'Cloud services', 'Networking', etc.."]
    #[serde(rename = "meterCategory", default, skip_serializing_if = "Option::is_none")]
    pub meter_category: Option<String>,
    #[doc = "The subcategory of the meter, for example, 'A6 Cloud services', 'ExpressRoute (IXP)', etc.."]
    #[serde(rename = "meterSubCategory", default, skip_serializing_if = "Option::is_none")]
    pub meter_sub_category: Option<String>,
    #[doc = "The unit in which the meter consumption is charged, for example, 'Hours', 'GB', etc."]
    #[serde(rename = "unitOfMeasure", default, skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<String>,
    #[doc = "The service family."]
    #[serde(rename = "serviceFamily", default, skip_serializing_if = "Option::is_none")]
    pub service_family: Option<String>,
}
impl MeterDetailsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Modern charge summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModernChargeSummary {
    #[serde(flatten)]
    pub charge_summary: ChargeSummary,
    #[doc = "The properties of modern charge summary."]
    pub properties: ModernChargeSummaryProperties,
}
impl ModernChargeSummary {
    pub fn new(charge_summary: ChargeSummary, properties: ModernChargeSummaryProperties) -> Self {
        Self {
            charge_summary,
            properties,
        }
    }
}
#[doc = "The properties of modern charge summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModernChargeSummaryProperties {
    #[doc = "The id of the billing period resource that the charge belongs to."]
    #[serde(rename = "billingPeriodId", default, skip_serializing_if = "Option::is_none")]
    pub billing_period_id: Option<String>,
    #[doc = "Usage start date."]
    #[serde(rename = "usageStart", default, skip_serializing_if = "Option::is_none")]
    pub usage_start: Option<String>,
    #[doc = "Usage end date."]
    #[serde(rename = "usageEnd", default, skip_serializing_if = "Option::is_none")]
    pub usage_end: Option<String>,
    #[doc = "The amount plus currency ."]
    #[serde(rename = "azureCharges", default, skip_serializing_if = "Option::is_none")]
    pub azure_charges: Option<Amount>,
    #[doc = "The amount plus currency ."]
    #[serde(rename = "chargesBilledSeparately", default, skip_serializing_if = "Option::is_none")]
    pub charges_billed_separately: Option<Amount>,
    #[doc = "The amount plus currency ."]
    #[serde(rename = "marketplaceCharges", default, skip_serializing_if = "Option::is_none")]
    pub marketplace_charges: Option<Amount>,
    #[doc = "Billing Account Id"]
    #[serde(rename = "billingAccountId", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_id: Option<String>,
    #[doc = "Billing Profile Id"]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "Invoice Section Id"]
    #[serde(rename = "invoiceSectionId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_id: Option<String>,
    #[doc = "Customer Id"]
    #[serde(rename = "customerId", default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[doc = "Is charge Invoiced"]
    #[serde(rename = "isInvoiced", default, skip_serializing_if = "Option::is_none")]
    pub is_invoiced: Option<bool>,
}
impl ModernChargeSummaryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Modern reservation recommendation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModernReservationRecommendation {
    #[serde(flatten)]
    pub reservation_recommendation: ReservationRecommendation,
    #[doc = "The properties of the reservation recommendation."]
    pub properties: ModernReservationRecommendationProperties,
}
impl ModernReservationRecommendation {
    pub fn new(reservation_recommendation: ReservationRecommendation, properties: ModernReservationRecommendationProperties) -> Self {
        Self {
            reservation_recommendation,
            properties,
        }
    }
}
#[doc = "The properties of the reservation recommendation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModernReservationRecommendationProperties {
    #[doc = "Resource Location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The number of days of usage to look back for recommendation."]
    #[serde(rename = "lookBackPeriod", default, skip_serializing_if = "Option::is_none")]
    pub look_back_period: Option<i32>,
    #[doc = "The instance Flexibility Ratio."]
    #[serde(rename = "instanceFlexibilityRatio", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility_ratio: Option<f64>,
    #[doc = "The instance Flexibility Group."]
    #[serde(rename = "instanceFlexibilityGroup", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility_group: Option<String>,
    #[doc = "The normalized Size."]
    #[serde(rename = "normalizedSize", default, skip_serializing_if = "Option::is_none")]
    pub normalized_size: Option<String>,
    #[doc = "The recommended Quantity Normalized."]
    #[serde(rename = "recommendedQuantityNormalized", default, skip_serializing_if = "Option::is_none")]
    pub recommended_quantity_normalized: Option<f64>,
    #[doc = "The meter id (GUID)"]
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[doc = "RI recommendations in one or three year terms."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    #[doc = "The amount plus currency ."]
    #[serde(rename = "costWithNoReservedInstances", default, skip_serializing_if = "Option::is_none")]
    pub cost_with_no_reserved_instances: Option<Amount>,
    #[doc = "Recommended quality for reserved instances."]
    #[serde(rename = "recommendedQuantity", default, skip_serializing_if = "Option::is_none")]
    pub recommended_quantity: Option<f64>,
    #[doc = "The amount plus currency ."]
    #[serde(rename = "totalCostWithReservedInstances", default, skip_serializing_if = "Option::is_none")]
    pub total_cost_with_reserved_instances: Option<Amount>,
    #[doc = "The amount plus currency ."]
    #[serde(rename = "netSavings", default, skip_serializing_if = "Option::is_none")]
    pub net_savings: Option<Amount>,
    #[doc = "The usage date for looking back."]
    #[serde(rename = "firstUsageDate", with = "azure_core::date::rfc3339::option")]
    pub first_usage_date: Option<time::OffsetDateTime>,
    #[doc = "Shared or single recommendation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "List of sku properties"]
    #[serde(rename = "skuProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub sku_properties: Vec<SkuProperty>,
    #[doc = "This is the ARM Sku name."]
    #[serde(rename = "skuName", default, skip_serializing_if = "Option::is_none")]
    pub sku_name: Option<String>,
}
impl ModernReservationRecommendationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Modern Reservation transaction resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModernReservationTransaction {
    #[serde(flatten)]
    pub reservation_transaction_resource: ReservationTransactionResource,
    #[doc = "The properties of a modern reservation transaction."]
    pub properties: ModernReservationTransactionProperties,
}
impl ModernReservationTransaction {
    pub fn new(properties: ModernReservationTransactionProperties) -> Self {
        Self {
            reservation_transaction_resource: ReservationTransactionResource::default(),
            properties,
        }
    }
}
#[doc = "The properties of a modern reservation transaction."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModernReservationTransactionProperties {
    #[doc = "The charge of the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[doc = "This is the ARM Sku name. It can be used to join with the serviceType field in additional info in usage records."]
    #[serde(rename = "armSkuName", default, skip_serializing_if = "Option::is_none")]
    pub arm_sku_name: Option<String>,
    #[doc = "The billing frequency, which can be either one-time or recurring."]
    #[serde(rename = "billingFrequency", default, skip_serializing_if = "Option::is_none")]
    pub billing_frequency: Option<String>,
    #[doc = "Billing profile Id."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "Billing profile name."]
    #[serde(rename = "billingProfileName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_name: Option<String>,
    #[doc = "The ISO currency in which the transaction is charged, for example, USD."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The description of the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The date of the transaction"]
    #[serde(rename = "eventDate", with = "azure_core::date::rfc3339::option")]
    pub event_date: Option<time::OffsetDateTime>,
    #[doc = "The type of the transaction (Purchase, Cancel or Refund)."]
    #[serde(rename = "eventType", default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[doc = "Invoice Number"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invoice: Option<String>,
    #[doc = "Invoice Id as on the invoice where the specific transaction appears."]
    #[serde(rename = "invoiceId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    #[doc = "Invoice Section Id"]
    #[serde(rename = "invoiceSectionId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_id: Option<String>,
    #[doc = "Invoice Section Name."]
    #[serde(rename = "invoiceSectionName", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_name: Option<String>,
    #[doc = "The subscription guid that makes the transaction."]
    #[serde(rename = "purchasingSubscriptionGuid", default, skip_serializing_if = "Option::is_none")]
    pub purchasing_subscription_guid: Option<String>,
    #[doc = "The subscription name that makes the transaction."]
    #[serde(rename = "purchasingSubscriptionName", default, skip_serializing_if = "Option::is_none")]
    pub purchasing_subscription_name: Option<String>,
    #[doc = "The quantity of the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    #[doc = "The region of the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "The reservation order ID is the identifier for a reservation purchase. Each reservation order ID represents a single purchase transaction. A reservation order contains reservations. The reservation order specifies the VM size and region for the reservations."]
    #[serde(rename = "reservationOrderId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order_id: Option<String>,
    #[doc = "The name of the reservation order."]
    #[serde(rename = "reservationOrderName", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order_name: Option<String>,
    #[doc = "This is the term of the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
}
impl ModernReservationTransactionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of listing reservation recommendations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModernReservationTransactionsListResult {
    #[doc = "The list of reservation recommendations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ModernReservationTransaction>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ModernReservationTransactionsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ModernReservationTransactionsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Modern usage detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModernUsageDetail {
    #[serde(flatten)]
    pub usage_detail: UsageDetail,
    #[doc = "The properties of the usage detail."]
    pub properties: ModernUsageDetailProperties,
}
impl ModernUsageDetail {
    pub fn new(usage_detail: UsageDetail, properties: ModernUsageDetailProperties) -> Self {
        Self { usage_detail, properties }
    }
}
#[doc = "The properties of the usage detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModernUsageDetailProperties {
    #[doc = "Billing Account identifier."]
    #[serde(rename = "billingAccountId", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_id: Option<String>,
    #[doc = "Effective Price that's charged for the usage."]
    #[serde(rename = "effectivePrice", default, skip_serializing_if = "Option::is_none")]
    pub effective_price: Option<f64>,
    #[doc = "Identifier that indicates how the meter is priced"]
    #[serde(rename = "pricingModel", default, skip_serializing_if = "Option::is_none")]
    pub pricing_model: Option<modern_usage_detail_properties::PricingModel>,
    #[doc = "Name of the Billing Account."]
    #[serde(rename = "billingAccountName", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_name: Option<String>,
    #[doc = "Billing Period Start Date as in the invoice."]
    #[serde(rename = "billingPeriodStartDate", with = "azure_core::date::rfc3339::option")]
    pub billing_period_start_date: Option<time::OffsetDateTime>,
    #[doc = "Billing Period End Date as in the invoice."]
    #[serde(rename = "billingPeriodEndDate", with = "azure_core::date::rfc3339::option")]
    pub billing_period_end_date: Option<time::OffsetDateTime>,
    #[doc = "Identifier for the billing profile that groups costs across invoices in the a singular billing currency across across the customers who have onboarded the Microsoft customer agreement and the customers in CSP who have made entitlement purchases like SaaS, Marketplace, RI, etc."]
    #[serde(rename = "billingProfileId", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_id: Option<String>,
    #[doc = "Name of the billing profile that groups costs across invoices in the a singular billing currency across across the customers who have onboarded the Microsoft customer agreement and the customers in CSP who have made entitlement purchases like SaaS, Marketplace, RI, etc."]
    #[serde(rename = "billingProfileName", default, skip_serializing_if = "Option::is_none")]
    pub billing_profile_name: Option<String>,
    #[doc = "Unique Microsoft generated identifier for the Azure Subscription."]
    #[serde(rename = "subscriptionGuid", default, skip_serializing_if = "Option::is_none")]
    pub subscription_guid: Option<String>,
    #[doc = "Name of the Azure Subscription."]
    #[serde(rename = "subscriptionName", default, skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
    #[doc = "Date for the usage record."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub date: Option<time::OffsetDateTime>,
    #[doc = "Name of the product that has accrued charges by consumption or purchase as listed in the invoice. Not available for Marketplace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[doc = "The meter id (GUID). Not available for marketplace. For reserved instance this represents the primary meter for which the reservation was purchased. For the actual VM Size for which the reservation is purchased see productOrderName."]
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[doc = "Identifies the name of the meter against which consumption is measured."]
    #[serde(rename = "meterName", default, skip_serializing_if = "Option::is_none")]
    pub meter_name: Option<String>,
    #[doc = "Identifies the location of the datacenter for certain services that are priced based on datacenter location."]
    #[serde(rename = "meterRegion", default, skip_serializing_if = "Option::is_none")]
    pub meter_region: Option<String>,
    #[doc = "Identifies the top-level service for the usage."]
    #[serde(rename = "meterCategory", default, skip_serializing_if = "Option::is_none")]
    pub meter_category: Option<String>,
    #[doc = "Defines the type or sub-category of Azure service that can affect the rate."]
    #[serde(rename = "meterSubCategory", default, skip_serializing_if = "Option::is_none")]
    pub meter_sub_category: Option<String>,
    #[doc = "List the service family for the product purchased or charged (Example: Storage ; Compute)."]
    #[serde(rename = "serviceFamily", default, skip_serializing_if = "Option::is_none")]
    pub service_family: Option<String>,
    #[doc = "Measure the quantity purchased or consumed.The amount of the meter used during the billing period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    #[doc = "Identifies the Unit that the service is charged in. For example, GB, hours, 10,000 s."]
    #[serde(rename = "unitOfMeasure", default, skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<String>,
    #[doc = "Instance Name."]
    #[serde(rename = "instanceName", default, skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[doc = "Estimated extendedCost or blended cost before tax in USD."]
    #[serde(rename = "costInUSD", default, skip_serializing_if = "Option::is_none")]
    pub cost_in_usd: Option<f64>,
    #[doc = "Unit Price is the price applicable to you. (your EA or other contract price)."]
    #[serde(rename = "unitPrice", default, skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<f64>,
    #[doc = "The currency defining the billed cost."]
    #[serde(rename = "billingCurrencyCode", default, skip_serializing_if = "Option::is_none")]
    pub billing_currency_code: Option<String>,
    #[doc = "Name of the resource location."]
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[doc = "Consumed service name. Name of the azure resource provider that emits the usage or was purchased. This value is not provided for marketplace usage."]
    #[serde(rename = "consumedService", default, skip_serializing_if = "Option::is_none")]
    pub consumed_service: Option<String>,
    #[doc = "Service-specific metadata."]
    #[serde(rename = "serviceInfo1", default, skip_serializing_if = "Option::is_none")]
    pub service_info1: Option<String>,
    #[doc = "Legacy field with optional service-specific metadata."]
    #[serde(rename = "serviceInfo2", default, skip_serializing_if = "Option::is_none")]
    pub service_info2: Option<String>,
    #[doc = "Additional details of this usage item. Use this field to get usage line item specific details such as the actual VM Size (ServiceType) or the ratio in which the reservation discount is applied."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
    #[doc = "Identifier of the project that is being charged in the invoice. Not applicable for Microsoft Customer Agreements onboarded by partners."]
    #[serde(rename = "invoiceSectionId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_id: Option<String>,
    #[doc = "Name of the project that is being charged in the invoice. Not applicable for Microsoft Customer Agreements onboarded by partners."]
    #[serde(rename = "invoiceSectionName", default, skip_serializing_if = "Option::is_none")]
    pub invoice_section_name: Option<String>,
    #[doc = "The cost center of this department if it is a department and a cost center is provided."]
    #[serde(rename = "costCenter", default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[doc = "Name of the Azure resource group used for cohesive lifecycle management of resources."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "ARM resource id of the reservation. Only applies to records relevant to reservations."]
    #[serde(rename = "reservationId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    #[doc = "User provided display name of the reservation. Last known name for a particular day is populated in the daily data. Only applies to records relevant to reservations."]
    #[serde(rename = "reservationName", default, skip_serializing_if = "Option::is_none")]
    pub reservation_name: Option<String>,
    #[doc = "The identifier for the asset or Azure plan name that the subscription belongs to. For example: Azure Plan. For reservations this is the Reservation Order ID."]
    #[serde(rename = "productOrderId", default, skip_serializing_if = "Option::is_none")]
    pub product_order_id: Option<String>,
    #[doc = "Product Order Name. For reservations this is the SKU that was purchased."]
    #[serde(rename = "productOrderName", default, skip_serializing_if = "Option::is_none")]
    pub product_order_name: Option<String>,
    #[doc = "Determines if the cost is eligible to be paid for using Azure credits."]
    #[serde(rename = "isAzureCreditEligible", default, skip_serializing_if = "Option::is_none")]
    pub is_azure_credit_eligible: Option<bool>,
    #[doc = "Term (in months). Displays the term for the validity of the offer. For example. In case of reserved instances it displays 12 months for yearly term of reserved instance. For one time purchases or recurring purchases, the terms displays 1 month; This is not applicable for Azure consumption."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    #[doc = "Name of the publisher of the service including Microsoft or Third Party publishers."]
    #[serde(rename = "publisherName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_name: Option<String>,
    #[doc = "Type of publisher that identifies if the publisher is first party, third party reseller or third party agency."]
    #[serde(rename = "publisherType", default, skip_serializing_if = "Option::is_none")]
    pub publisher_type: Option<String>,
    #[doc = "Indicates a charge represents credits, usage, a Marketplace purchase, a reservation fee, or a refund."]
    #[serde(rename = "chargeType", default, skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<String>,
    #[doc = "Indicates how frequently this charge will occur. OneTime for purchases which only happen once, Monthly for fees which recur every month, and UsageBased for charges based on how much a service is used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    #[doc = "ExtendedCost or blended cost before tax in billed currency."]
    #[serde(rename = "costInBillingCurrency", default, skip_serializing_if = "Option::is_none")]
    pub cost_in_billing_currency: Option<f64>,
    #[doc = "ExtendedCost or blended cost before tax in pricing currency to correlate with prices."]
    #[serde(rename = "costInPricingCurrency", default, skip_serializing_if = "Option::is_none")]
    pub cost_in_pricing_currency: Option<f64>,
    #[doc = "Exchange rate used in conversion from pricing currency to billing currency."]
    #[serde(rename = "exchangeRate", default, skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<String>,
    #[doc = "Date on which exchange rate used in conversion from pricing currency to billing currency."]
    #[serde(rename = "exchangeRateDate", with = "azure_core::date::rfc3339::option")]
    pub exchange_rate_date: Option<time::OffsetDateTime>,
    #[doc = "Invoice ID as on the invoice where the specific transaction appears."]
    #[serde(rename = "invoiceId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    #[doc = "Reference to an original invoice there is a refund (negative cost). This is populated only when there is a refund."]
    #[serde(rename = "previousInvoiceId", default, skip_serializing_if = "Option::is_none")]
    pub previous_invoice_id: Option<String>,
    #[doc = "Pricing Billing Currency."]
    #[serde(rename = "pricingCurrencyCode", default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_code: Option<String>,
    #[doc = "Identifier for the product that has accrued charges by consumption or purchase . This is the concatenated key of productId and SkuId in partner center."]
    #[serde(rename = "productIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub product_identifier: Option<String>,
    #[doc = "Resource Location Normalized."]
    #[serde(rename = "resourceLocationNormalized", default, skip_serializing_if = "Option::is_none")]
    pub resource_location_normalized: Option<String>,
    #[doc = "Start date for the rating period when the service usage was rated for charges. The prices for Azure services are determined for the rating period."]
    #[serde(rename = "servicePeriodStartDate", with = "azure_core::date::rfc3339::option")]
    pub service_period_start_date: Option<time::OffsetDateTime>,
    #[doc = "End date for the period when the service usage was rated for charges. The prices for Azure services are determined based on the rating period."]
    #[serde(rename = "servicePeriodEndDate", with = "azure_core::date::rfc3339::option")]
    pub service_period_end_date: Option<time::OffsetDateTime>,
    #[doc = "Identifier of the customer's AAD tenant."]
    #[serde(rename = "customerTenantId", default, skip_serializing_if = "Option::is_none")]
    pub customer_tenant_id: Option<String>,
    #[doc = "Name of the customer's AAD tenant."]
    #[serde(rename = "customerName", default, skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    #[doc = "Identifier for the partner's AAD tenant."]
    #[serde(rename = "partnerTenantId", default, skip_serializing_if = "Option::is_none")]
    pub partner_tenant_id: Option<String>,
    #[doc = "Name of the partner' AAD tenant."]
    #[serde(rename = "partnerName", default, skip_serializing_if = "Option::is_none")]
    pub partner_name: Option<String>,
    #[doc = "MPNId for the reseller associated with the subscription."]
    #[serde(rename = "resellerMpnId", default, skip_serializing_if = "Option::is_none")]
    pub reseller_mpn_id: Option<String>,
    #[doc = "Reseller Name."]
    #[serde(rename = "resellerName", default, skip_serializing_if = "Option::is_none")]
    pub reseller_name: Option<String>,
    #[doc = "Publisher Id."]
    #[serde(rename = "publisherId", default, skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
    #[doc = "Market Price that's charged for the usage."]
    #[serde(rename = "marketPrice", default, skip_serializing_if = "Option::is_none")]
    pub market_price: Option<f64>,
    #[doc = "Exchange Rate from pricing currency to billing currency."]
    #[serde(rename = "exchangeRatePricingToBilling", default, skip_serializing_if = "Option::is_none")]
    pub exchange_rate_pricing_to_billing: Option<f64>,
    #[doc = "The amount of PayG cost before tax in billing currency."]
    #[serde(rename = "paygCostInBillingCurrency", default, skip_serializing_if = "Option::is_none")]
    pub payg_cost_in_billing_currency: Option<f64>,
    #[doc = "The amount of PayG cost before tax in US Dollar currency."]
    #[serde(rename = "paygCostInUSD", default, skip_serializing_if = "Option::is_none")]
    pub payg_cost_in_usd: Option<f64>,
    #[doc = "Rate of discount applied if there is a partner earned credit (PEC) based on partner admin link access."]
    #[serde(rename = "partnerEarnedCreditRate", default, skip_serializing_if = "Option::is_none")]
    pub partner_earned_credit_rate: Option<f64>,
    #[doc = "Flag to indicate if partner earned credit has been applied or not."]
    #[serde(rename = "partnerEarnedCreditApplied", default, skip_serializing_if = "Option::is_none")]
    pub partner_earned_credit_applied: Option<String>,
    #[doc = "Retail price for the resource."]
    #[serde(rename = "payGPrice", default, skip_serializing_if = "Option::is_none")]
    pub pay_g_price: Option<f64>,
    #[doc = "Unique identifier for the applicable benefit."]
    #[serde(rename = "benefitId", default, skip_serializing_if = "Option::is_none")]
    pub benefit_id: Option<String>,
    #[doc = "Name of the applicable benefit."]
    #[serde(rename = "benefitName", default, skip_serializing_if = "Option::is_none")]
    pub benefit_name: Option<String>,
    #[doc = "Identifier for Product Category or Line Of Business, Ex - Azure, Microsoft 365, AWS e.t.c"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Name for Cost Allocation Rule."]
    #[serde(rename = "costAllocationRuleName", default, skip_serializing_if = "Option::is_none")]
    pub cost_allocation_rule_name: Option<String>,
}
impl ModernUsageDetailProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod modern_usage_detail_properties {
    use super::*;
    #[doc = "Identifier that indicates how the meter is priced"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PricingModel")]
    pub enum PricingModel {
        #[serde(rename = "On Demand")]
        OnDemand,
        Reservation,
        Spot,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PricingModel {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PricingModel {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PricingModel {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::OnDemand => serializer.serialize_unit_variant("PricingModel", 0u32, "On Demand"),
                Self::Reservation => serializer.serialize_unit_variant("PricingModel", 1u32, "Reservation"),
                Self::Spot => serializer.serialize_unit_variant("PricingModel", 2u32, "Spot"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The notification associated with a budget."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    #[doc = "The notification is enabled or not."]
    pub enabled: bool,
    #[doc = "The comparison operator."]
    pub operator: notification::Operator,
    #[doc = "Threshold value associated with a notification. Notification is sent when the cost exceeded the threshold. It is always percent and has to be between 0 and 1000."]
    pub threshold: f64,
    #[doc = "Email addresses to send the budget notification to when the threshold is exceeded. Must have at least one contact email or contact group specified at the Subscription or Resource Group scopes. All other scopes must have at least one contact email specified."]
    #[serde(rename = "contactEmails")]
    pub contact_emails: Vec<String>,
    #[doc = "Contact roles to send the budget notification to when the threshold is exceeded."]
    #[serde(rename = "contactRoles", default, skip_serializing_if = "Vec::is_empty")]
    pub contact_roles: Vec<String>,
    #[doc = "Action groups to send the budget notification to when the threshold is exceeded. Must be provided as a fully qualified Azure resource id. Only supported at Subscription or Resource Group scopes."]
    #[serde(rename = "contactGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub contact_groups: Vec<String>,
    #[doc = "The type of threshold"]
    #[serde(rename = "thresholdType", default, skip_serializing_if = "Option::is_none")]
    pub threshold_type: Option<notification::ThresholdType>,
    #[doc = "Language in which the recipient will receive the notification"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<notification::Locale>,
}
impl Notification {
    pub fn new(enabled: bool, operator: notification::Operator, threshold: f64, contact_emails: Vec<String>) -> Self {
        Self {
            enabled,
            operator,
            threshold,
            contact_emails,
            contact_roles: Vec::new(),
            contact_groups: Vec::new(),
            threshold_type: None,
            locale: None,
        }
    }
}
pub mod notification {
    use super::*;
    #[doc = "The comparison operator."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Operator")]
    pub enum Operator {
        EqualTo,
        GreaterThan,
        GreaterThanOrEqualTo,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Operator {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Operator {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Operator {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::EqualTo => serializer.serialize_unit_variant("Operator", 0u32, "EqualTo"),
                Self::GreaterThan => serializer.serialize_unit_variant("Operator", 1u32, "GreaterThan"),
                Self::GreaterThanOrEqualTo => serializer.serialize_unit_variant("Operator", 2u32, "GreaterThanOrEqualTo"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of threshold"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ThresholdType")]
    pub enum ThresholdType {
        Actual,
        Forecasted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ThresholdType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ThresholdType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ThresholdType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Actual => serializer.serialize_unit_variant("ThresholdType", 0u32, "Actual"),
                Self::Forecasted => serializer.serialize_unit_variant("ThresholdType", 1u32, "Forecasted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for ThresholdType {
        fn default() -> Self {
            Self::Actual
        }
    }
    #[doc = "Language in which the recipient will receive the notification"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Locale")]
    pub enum Locale {
        #[serde(rename = "en-us")]
        EnUs,
        #[serde(rename = "ja-jp")]
        JaJp,
        #[serde(rename = "zh-cn")]
        ZhCn,
        #[serde(rename = "de-de")]
        DeDe,
        #[serde(rename = "es-es")]
        EsEs,
        #[serde(rename = "fr-fr")]
        FrFr,
        #[serde(rename = "it-it")]
        ItIt,
        #[serde(rename = "ko-kr")]
        KoKr,
        #[serde(rename = "pt-br")]
        PtBr,
        #[serde(rename = "ru-ru")]
        RuRu,
        #[serde(rename = "zh-tw")]
        ZhTw,
        #[serde(rename = "cs-cz")]
        CsCz,
        #[serde(rename = "pl-pl")]
        PlPl,
        #[serde(rename = "tr-tr")]
        TrTr,
        #[serde(rename = "da-dk")]
        DaDk,
        #[serde(rename = "en-gb")]
        EnGb,
        #[serde(rename = "hu-hu")]
        HuHu,
        #[serde(rename = "nb-no")]
        NbNo,
        #[serde(rename = "nl-nl")]
        NlNl,
        #[serde(rename = "pt-pt")]
        PtPt,
        #[serde(rename = "sv-se")]
        SvSe,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Locale {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Locale {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Locale {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::EnUs => serializer.serialize_unit_variant("Locale", 0u32, "en-us"),
                Self::JaJp => serializer.serialize_unit_variant("Locale", 1u32, "ja-jp"),
                Self::ZhCn => serializer.serialize_unit_variant("Locale", 2u32, "zh-cn"),
                Self::DeDe => serializer.serialize_unit_variant("Locale", 3u32, "de-de"),
                Self::EsEs => serializer.serialize_unit_variant("Locale", 4u32, "es-es"),
                Self::FrFr => serializer.serialize_unit_variant("Locale", 5u32, "fr-fr"),
                Self::ItIt => serializer.serialize_unit_variant("Locale", 6u32, "it-it"),
                Self::KoKr => serializer.serialize_unit_variant("Locale", 7u32, "ko-kr"),
                Self::PtBr => serializer.serialize_unit_variant("Locale", 8u32, "pt-br"),
                Self::RuRu => serializer.serialize_unit_variant("Locale", 9u32, "ru-ru"),
                Self::ZhTw => serializer.serialize_unit_variant("Locale", 10u32, "zh-tw"),
                Self::CsCz => serializer.serialize_unit_variant("Locale", 11u32, "cs-cz"),
                Self::PlPl => serializer.serialize_unit_variant("Locale", 12u32, "pl-pl"),
                Self::TrTr => serializer.serialize_unit_variant("Locale", 13u32, "tr-tr"),
                Self::DaDk => serializer.serialize_unit_variant("Locale", 14u32, "da-dk"),
                Self::EnGb => serializer.serialize_unit_variant("Locale", 15u32, "en-gb"),
                Self::HuHu => serializer.serialize_unit_variant("Locale", 16u32, "hu-hu"),
                Self::NbNo => serializer.serialize_unit_variant("Locale", 17u32, "nb-no"),
                Self::NlNl => serializer.serialize_unit_variant("Locale", 18u32, "nl-nl"),
                Self::PtPt => serializer.serialize_unit_variant("Locale", 19u32, "pt-pt"),
                Self::SvSe => serializer.serialize_unit_variant("Locale", 20u32, "sv-se"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A Consumption REST API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Operation name: {provider}/{resource}/{operation}."]
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
        #[doc = "Service provider: Microsoft.Consumption."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: UsageDetail, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of listing consumption operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of consumption operations supported by the Microsoft.Consumption resource provider."]
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
#[doc = "price sheet result. It contains the pricesheet associated with billing period"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PriceSheetModel {
    #[doc = "Price sheet"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pricesheets: Vec<PriceSheetProperties>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The properties of the meter detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download: Option<MeterDetails>,
}
impl PriceSheetModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the price sheet."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PriceSheetProperties {
    #[doc = "The id of the billing period resource that the usage belongs to."]
    #[serde(rename = "billingPeriodId", default, skip_serializing_if = "Option::is_none")]
    pub billing_period_id: Option<String>,
    #[doc = "The meter id (GUID)"]
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[doc = "The properties of the meter detail."]
    #[serde(rename = "meterDetails", default, skip_serializing_if = "Option::is_none")]
    pub meter_details: Option<MeterDetails>,
    #[doc = "Unit of measure"]
    #[serde(rename = "unitOfMeasure", default, skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<String>,
    #[doc = "Included quality for an offer"]
    #[serde(rename = "includedQuantity", default, skip_serializing_if = "Option::is_none")]
    pub included_quantity: Option<f64>,
    #[doc = "Part Number"]
    #[serde(rename = "partNumber", default, skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
    #[doc = "Unit Price"]
    #[serde(rename = "unitPrice", default, skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<f64>,
    #[doc = "Currency Code"]
    #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[doc = "Offer Id"]
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
}
impl PriceSheetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An pricesheet resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PriceSheetResult {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "price sheet result. It contains the pricesheet associated with billing period"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PriceSheetModel>,
}
impl PriceSheetResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "eTag of the resource. To handle concurrent update scenario, this field will be used to determine whether the user is updating the latest version or not."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The reseller properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Reseller {
    #[doc = "The reseller property ID."]
    #[serde(rename = "resellerId", default, skip_serializing_if = "Option::is_none")]
    pub reseller_id: Option<String>,
    #[doc = "The reseller property description."]
    #[serde(rename = "resellerDescription", default, skip_serializing_if = "Option::is_none")]
    pub reseller_description: Option<String>,
}
impl Reseller {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "reservation detail resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationDetail {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the reservation detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationDetailProperties>,
}
impl ReservationDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the reservation detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationDetailProperties {
    #[doc = "The reservation order ID is the identifier for a reservation purchase. Each reservation order ID represents a single purchase transaction. A reservation order contains reservations. The reservation order specifies the VM size and region for the reservations."]
    #[serde(rename = "reservationOrderId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order_id: Option<String>,
    #[doc = "The instance Flexibility Ratio."]
    #[serde(rename = "instanceFlexibilityRatio", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility_ratio: Option<String>,
    #[doc = "The instance Flexibility Group."]
    #[serde(rename = "instanceFlexibilityGroup", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility_group: Option<String>,
    #[doc = "The reservation ID is the identifier of a reservation within a reservation order. Each reservation is the grouping for applying the benefit scope and also specifies the number of instances to which the reservation benefit can be applied to."]
    #[serde(rename = "reservationId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    #[doc = "This is the ARM Sku name. It can be used to join with the serviceType field in additional info in usage records."]
    #[serde(rename = "skuName", default, skip_serializing_if = "Option::is_none")]
    pub sku_name: Option<String>,
    #[doc = "This is the total hours reserved for the day. E.g. if reservation for 1 instance was made on 1 PM, this will be 11 hours for that day and 24 hours from subsequent days."]
    #[serde(rename = "reservedHours", default, skip_serializing_if = "Option::is_none")]
    pub reserved_hours: Option<f64>,
    #[doc = "The date on which consumption occurred."]
    #[serde(rename = "usageDate", with = "azure_core::date::rfc3339::option")]
    pub usage_date: Option<time::OffsetDateTime>,
    #[doc = "This is the total hours used by the instance."]
    #[serde(rename = "usedHours", default, skip_serializing_if = "Option::is_none")]
    pub used_hours: Option<f64>,
    #[doc = "This identifier is the name of the resource or the fully qualified Resource ID."]
    #[serde(rename = "instanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[doc = "This is the total count of instances that are reserved for the reservationId."]
    #[serde(rename = "totalReservedQuantity", default, skip_serializing_if = "Option::is_none")]
    pub total_reserved_quantity: Option<f64>,
    #[doc = "The reservation kind."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
impl ReservationDetailProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of listing reservation details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationDetailsListResult {
    #[doc = "The list of reservation details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReservationDetail>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ReservationDetailsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ReservationDetailsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A reservation recommendation resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReservationRecommendation {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub resource_attributes: ResourceAttributes,
    #[doc = "Specifies the kind of reservation recommendation."]
    pub kind: reservation_recommendation::Kind,
}
impl ReservationRecommendation {
    pub fn new(kind: reservation_recommendation::Kind) -> Self {
        Self {
            resource: Resource::default(),
            resource_attributes: ResourceAttributes::default(),
            kind,
        }
    }
}
pub mod reservation_recommendation {
    use super::*;
    #[doc = "Specifies the kind of reservation recommendation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        #[serde(rename = "legacy")]
        Legacy,
        #[serde(rename = "modern")]
        Modern,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Legacy => serializer.serialize_unit_variant("Kind", 0u32, "legacy"),
                Self::Modern => serializer.serialize_unit_variant("Kind", 1u32, "modern"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Details of estimated savings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationRecommendationDetailsCalculatedSavingsProperties {
    #[doc = "The cost without reservation."]
    #[serde(rename = "onDemandCost", default, skip_serializing_if = "Option::is_none")]
    pub on_demand_cost: Option<f64>,
    #[doc = "The difference between total reservation cost and reservation cost."]
    #[serde(rename = "overageCost", default, skip_serializing_if = "Option::is_none")]
    pub overage_cost: Option<f64>,
    #[doc = "The quantity for calculated savings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    #[doc = "The exact cost of the estimated usage using reservation."]
    #[serde(rename = "reservationCost", default, skip_serializing_if = "Option::is_none")]
    pub reservation_cost: Option<f64>,
    #[doc = "The cost of the suggested quantity."]
    #[serde(rename = "totalReservationCost", default, skip_serializing_if = "Option::is_none")]
    pub total_reservation_cost: Option<f64>,
    #[doc = "The number of reserved units used to calculate savings. Always 1 for virtual machines."]
    #[serde(rename = "reservedUnitCount", default, skip_serializing_if = "Option::is_none")]
    pub reserved_unit_count: Option<f64>,
    #[doc = "The amount saved by purchasing the recommended quantity of reservation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub savings: Option<f64>,
}
impl ReservationRecommendationDetailsCalculatedSavingsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reservation recommendation details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationRecommendationDetailsModel {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource Location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource sku"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "The properties of the reservation recommendation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationRecommendationDetailsProperties>,
}
impl ReservationRecommendationDetailsModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the reservation recommendation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationRecommendationDetailsProperties {
    #[doc = "An ISO 4217 currency code identifier for the costs and savings "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "Details of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<ReservationRecommendationDetailsResourceProperties>,
    #[doc = "Resource Group."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "Details of the estimated savings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub savings: Option<ReservationRecommendationDetailsSavingsProperties>,
    #[doc = "Scope of the reservation, ex: Single or Shared."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "Details about historical usage data that has been used for computing the recommendation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ReservationRecommendationDetailsUsageProperties>,
}
impl ReservationRecommendationDetailsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationRecommendationDetailsResourceProperties {
    #[doc = "List of subscriptions for which the reservation is applied."]
    #[serde(rename = "appliedScopes", default, skip_serializing_if = "Vec::is_empty")]
    pub applied_scopes: Vec<String>,
    #[doc = "On demand rate of the resource."]
    #[serde(rename = "onDemandRate", default, skip_serializing_if = "Option::is_none")]
    pub on_demand_rate: Option<f64>,
    #[doc = "Azure product ex: Standard_E8s_v3 etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[doc = "Azure resource region ex:EastUS, WestUS etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "Reservation rate of the resource."]
    #[serde(rename = "reservationRate", default, skip_serializing_if = "Option::is_none")]
    pub reservation_rate: Option<f64>,
    #[doc = "The azure resource type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}
impl ReservationRecommendationDetailsResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the estimated savings."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationRecommendationDetailsSavingsProperties {
    #[doc = "List of calculated savings."]
    #[serde(rename = "calculatedSavings", default, skip_serializing_if = "Vec::is_empty")]
    pub calculated_savings: Vec<ReservationRecommendationDetailsCalculatedSavingsProperties>,
    #[doc = "Number of days of usage to look back used for computing the recommendation."]
    #[serde(rename = "lookBackPeriod", default, skip_serializing_if = "Option::is_none")]
    pub look_back_period: Option<i32>,
    #[doc = "Number of recommended units of the resource."]
    #[serde(rename = "recommendedQuantity", default, skip_serializing_if = "Option::is_none")]
    pub recommended_quantity: Option<f64>,
    #[doc = "Term period of the reservation, ex: P1Y or P3Y."]
    #[serde(rename = "reservationOrderTerm", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order_term: Option<String>,
    #[doc = "Type of savings, ex: instance."]
    #[serde(rename = "savingsType", default, skip_serializing_if = "Option::is_none")]
    pub savings_type: Option<String>,
    #[doc = "Measurement unit ex: hour etc."]
    #[serde(rename = "unitOfMeasure", default, skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<String>,
}
impl ReservationRecommendationDetailsSavingsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details about historical usage data that has been used for computing the recommendation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationRecommendationDetailsUsageProperties {
    #[doc = "The first usage date used for looking back for computing the recommendation."]
    #[serde(rename = "firstConsumptionDate", default, skip_serializing_if = "Option::is_none")]
    pub first_consumption_date: Option<String>,
    #[doc = "The last usage date used for looking back for computing the recommendation."]
    #[serde(rename = "lastConsumptionDate", default, skip_serializing_if = "Option::is_none")]
    pub last_consumption_date: Option<String>,
    #[doc = "What the usage data values represent ex: virtual machine instance."]
    #[serde(rename = "lookBackUnitType", default, skip_serializing_if = "Option::is_none")]
    pub look_back_unit_type: Option<String>,
    #[doc = "The breakdown of historical resource usage.  The values are in the order of usage between the firstConsumptionDate and the lastConsumptionDate."]
    #[serde(rename = "usageData", default, skip_serializing_if = "Vec::is_empty")]
    pub usage_data: Vec<f64>,
    #[doc = "The grain of the values represented in the usage data ex: hourly."]
    #[serde(rename = "usageGrain", default, skip_serializing_if = "Option::is_none")]
    pub usage_grain: Option<String>,
}
impl ReservationRecommendationDetailsUsageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of listing reservation recommendations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationRecommendationsListResult {
    #[doc = "The list of reservation recommendations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReservationRecommendation>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The link (url) to the previous page of results."]
    #[serde(rename = "previousLink", default, skip_serializing_if = "Option::is_none")]
    pub previous_link: Option<String>,
}
impl azure_core::Continuable for ReservationRecommendationsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ReservationRecommendationsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of listing reservation summaries."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationSummariesListResult {
    #[doc = "The list of reservation summaries."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReservationSummary>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ReservationSummariesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ReservationSummariesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "reservation summary resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationSummary {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the reservation summary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationSummaryProperties>,
}
impl ReservationSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the reservation summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationSummaryProperties {
    #[doc = "The reservation order ID is the identifier for a reservation purchase. Each reservation order ID represents a single purchase transaction. A reservation order contains reservations. The reservation order specifies the VM size and region for the reservations."]
    #[serde(rename = "reservationOrderId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order_id: Option<String>,
    #[doc = "The reservation ID is the identifier of a reservation within a reservation order. Each reservation is the grouping for applying the benefit scope and also specifies the number of instances to which the reservation benefit can be applied to."]
    #[serde(rename = "reservationId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    #[doc = "This is the ARM Sku name. It can be used to join with the serviceType field in additional info in usage records."]
    #[serde(rename = "skuName", default, skip_serializing_if = "Option::is_none")]
    pub sku_name: Option<String>,
    #[doc = "This is the total hours reserved. E.g. if reservation for 1 instance was made on 1 PM, this will be 11 hours for that day and 24 hours from subsequent days"]
    #[serde(rename = "reservedHours", default, skip_serializing_if = "Option::is_none")]
    pub reserved_hours: Option<f64>,
    #[doc = "Data corresponding to the utilization record. If the grain of data is monthly, it will be first day of month."]
    #[serde(rename = "usageDate", with = "azure_core::date::rfc3339::option")]
    pub usage_date: Option<time::OffsetDateTime>,
    #[doc = "Total used hours by the reservation"]
    #[serde(rename = "usedHours", default, skip_serializing_if = "Option::is_none")]
    pub used_hours: Option<f64>,
    #[doc = "This is the minimum hourly utilization in the usage time (day or month). E.g. if usage record corresponds to 12/10/2017 and on that for hour 4 and 5, utilization was 10%, this field will return 10% for that day"]
    #[serde(rename = "minUtilizationPercentage", default, skip_serializing_if = "Option::is_none")]
    pub min_utilization_percentage: Option<f64>,
    #[doc = "This is average utilization for the entire time range. (day or month depending on the grain)"]
    #[serde(rename = "avgUtilizationPercentage", default, skip_serializing_if = "Option::is_none")]
    pub avg_utilization_percentage: Option<f64>,
    #[doc = "This is the maximum hourly utilization in the usage time (day or month). E.g. if usage record corresponds to 12/10/2017 and on that for hour 4 and 5, utilization was 100%, this field will return 100% for that day."]
    #[serde(rename = "maxUtilizationPercentage", default, skip_serializing_if = "Option::is_none")]
    pub max_utilization_percentage: Option<f64>,
    #[doc = "The reservation kind."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "This is the purchased quantity for the reservationId."]
    #[serde(rename = "purchasedQuantity", default, skip_serializing_if = "Option::is_none")]
    pub purchased_quantity: Option<f64>,
    #[doc = "This is the remaining quantity for the reservationId."]
    #[serde(rename = "remainingQuantity", default, skip_serializing_if = "Option::is_none")]
    pub remaining_quantity: Option<f64>,
    #[doc = "This is the total count of instances that are reserved for the reservationId."]
    #[serde(rename = "totalReservedQuantity", default, skip_serializing_if = "Option::is_none")]
    pub total_reserved_quantity: Option<f64>,
    #[doc = "This is the used quantity for the reservationId."]
    #[serde(rename = "usedQuantity", default, skip_serializing_if = "Option::is_none")]
    pub used_quantity: Option<f64>,
    #[doc = "This is the utilized percentage for the reservation Id."]
    #[serde(rename = "utilizedPercentage", default, skip_serializing_if = "Option::is_none")]
    pub utilized_percentage: Option<f64>,
}
impl ReservationSummaryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reservation transaction resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationTransaction {
    #[serde(flatten)]
    pub reservation_transaction_resource: ReservationTransactionResource,
    #[doc = "The properties of a legacy reservation transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LegacyReservationTransactionProperties>,
}
impl ReservationTransaction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationTransactionResource {
    #[doc = "Resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}
impl ReservationTransactionResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of listing reservation recommendations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationTransactionsListResult {
    #[doc = "The list of reservation recommendations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReservationTransaction>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ReservationTransactionsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ReservationTransactionsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "The full qualified ARM ID of an event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ID that uniquely identifies an event. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The etag for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceAttributes {
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource sku"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
}
impl ResourceAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Sku property"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuProperty {
    #[doc = "The name of sku property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of sku property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl SkuProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The tag resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tag {
    #[doc = "Tag key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "Tag values."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<String>,
}
impl Tag {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the tag."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagProperties {
    #[doc = "A list of Tag."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The link (url) to the previous page of results."]
    #[serde(rename = "previousLink", default, skip_serializing_if = "Option::is_none")]
    pub previous_link: Option<String>,
}
impl TagProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A resource listing all tags."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsResult {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of the tag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TagProperties>,
}
impl TagsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An usage detail resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageDetail {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Specifies the kind of usage details."]
    pub kind: usage_detail::Kind,
}
impl UsageDetail {
    pub fn new(kind: usage_detail::Kind) -> Self {
        Self {
            resource: Resource::default(),
            kind,
        }
    }
}
pub mod usage_detail {
    use super::*;
    #[doc = "Specifies the kind of usage details."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        #[serde(rename = "legacy")]
        Legacy,
        #[serde(rename = "modern")]
        Modern,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Legacy => serializer.serialize_unit_variant("Kind", 0u32, "legacy"),
                Self::Modern => serializer.serialize_unit_variant("Kind", 1u32, "modern"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Result of listing usage details. It contains a list of available usage details in reverse chronological order by billing period."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageDetailsListResult {
    #[doc = "The list of usage details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UsageDetail>,
    #[doc = "The link (url) to the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UsageDetailsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl UsageDetailsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The amount plus currency ."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Amount {
    #[doc = "Amount currency."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "Amount."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
impl Amount {
    pub fn new() -> Self {
        Self::default()
    }
}
