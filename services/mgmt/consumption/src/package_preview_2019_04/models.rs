#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
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
#[doc = "The properties of the budget."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BudgetProperties {
    #[doc = "The category of the budget, whether the budget tracks cost or usage."]
    pub category: budget_properties::Category,
    #[doc = "The total amount of cost to track with the budget"]
    pub amount: f64,
    #[doc = "The time covered by a budget. Tracking of the amount will be reset based on the time grain."]
    #[serde(rename = "timeGrain")]
    pub time_grain: budget_properties::TimeGrain,
    #[doc = "The start and end date for a budget."]
    #[serde(rename = "timePeriod")]
    pub time_period: BudgetTimePeriod,
    #[doc = "May be used to filter budgets by resource group, resource, or meter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Filters>,
    #[doc = "The current amount of cost which is being tracked for a budget."]
    #[serde(rename = "currentSpend", default, skip_serializing_if = "Option::is_none")]
    pub current_spend: Option<CurrentSpend>,
    #[doc = "Dictionary of notifications associated with the budget. Budget can have up to five notifications."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notifications: Option<serde_json::Value>,
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
            filters: None,
            current_spend: None,
            notifications: None,
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
        Usage,
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
                Self::Usage => serializer.serialize_unit_variant("Category", 1u32, "Usage"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The time covered by a budget. Tracking of the amount will be reset based on the time grain."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TimeGrain")]
    pub enum TimeGrain {
        Monthly,
        Quarterly,
        Annually,
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
    #[serde(rename = "endDate", default, with = "azure_core::date::rfc3339::option")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChargeSummary {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the charge summary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ChargeSummaryProperties>,
}
impl ChargeSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the charge summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ChargeSummaryProperties {
    #[doc = "The id of the billing period resource that the charge belongs to."]
    #[serde(rename = "billingPeriodId", default, skip_serializing_if = "Option::is_none")]
    pub billing_period_id: Option<String>,
    #[doc = "Usage start date."]
    #[serde(rename = "usageStart", default, skip_serializing_if = "Option::is_none")]
    pub usage_start: Option<String>,
    #[doc = " Usage end date."]
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
impl ChargeSummaryProperties {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "A secure URL that can be used to download a an entity until the URL expires."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DownloadUrl {
    #[doc = "The URL to the csv file."]
    #[serde(rename = "downloadUrl", default, skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[doc = "The time in UTC at which this download URL will expire."]
    #[serde(rename = "validTill", default, skip_serializing_if = "Option::is_none")]
    pub valid_till: Option<String>,
}
impl DownloadUrl {
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
#[doc = "May be used to filter budgets by resource group, resource, or meter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Filters {
    #[doc = "The list of filters on resource groups, allowed at subscription level only."]
    #[serde(rename = "resourceGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_groups: Vec<String>,
    #[doc = "The list of filters on resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
    #[doc = "The list of filters on meters (GUID), mandatory for budgets of usage category. "]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub meters: Vec<String>,
    #[doc = "The dictionary of filters on tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Filters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A forecast resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Forecast {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the forecast charge."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ForecastProperties>,
}
impl Forecast {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the forecast charge."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ForecastProperties {
    #[doc = "The usage date of the forecast."]
    #[serde(rename = "usageDate", default, skip_serializing_if = "Option::is_none")]
    pub usage_date: Option<String>,
    #[doc = "The granularity of forecast."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grain: Option<forecast_properties::Grain>,
    #[doc = "The amount of charge"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub charge: Option<f64>,
    #[doc = "The ISO currency in which the meter is charged, for example, USD."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The type of the charge. Could be actual or forecast"]
    #[serde(rename = "chargeType", default, skip_serializing_if = "Option::is_none")]
    pub charge_type: Option<forecast_properties::ChargeType>,
    #[doc = "The details about the forecast confidence levels. This is populated only when chargeType is Forecast."]
    #[serde(rename = "confidenceLevels", default, skip_serializing_if = "Vec::is_empty")]
    pub confidence_levels: Vec<serde_json::Value>,
}
impl ForecastProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod forecast_properties {
    use super::*;
    #[doc = "The granularity of forecast."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Grain")]
    pub enum Grain {
        Daily,
        Monthly,
        Yearly,
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
                Self::Daily => serializer.serialize_unit_variant("Grain", 0u32, "Daily"),
                Self::Monthly => serializer.serialize_unit_variant("Grain", 1u32, "Monthly"),
                Self::Yearly => serializer.serialize_unit_variant("Grain", 2u32, "Yearly"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of the charge. Could be actual or forecast"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ChargeType")]
    pub enum ChargeType {
        Actual,
        Forecast,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ChargeType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ChargeType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ChargeType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Actual => serializer.serialize_unit_variant("ChargeType", 0u32, "Actual"),
                Self::Forecast => serializer.serialize_unit_variant("ChargeType", 1u32, "Forecast"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Result of listing forecasts. It contains a list of available forecasts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ForecastsListResult {
    #[doc = "The list of forecasts."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Forecast>,
}
impl azure_core::Continuable for ForecastsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ForecastsListResult {
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
    #[serde(rename = "usageStart", default, with = "azure_core::date::rfc3339::option")]
    pub usage_start: Option<time::OffsetDateTime>,
    #[doc = "The end of the date time range covered by the aggregated cost."]
    #[serde(rename = "usageEnd", default, with = "azure_core::date::rfc3339::option")]
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
#[doc = "An marketplace resource."]
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
    #[serde(rename = "usageStart", default, with = "azure_core::date::rfc3339::option")]
    pub usage_start: Option<time::OffsetDateTime>,
    #[doc = "The end of the date time range covered by the usage detail."]
    #[serde(rename = "usageEnd", default, with = "azure_core::date::rfc3339::option")]
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
#[doc = "The notification associated with a budget."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    #[doc = "The notification is enabled or not."]
    pub enabled: bool,
    #[doc = "The comparison operator."]
    pub operator: notification::Operator,
    #[doc = "Threshold value associated with a notification. Notification is sent when the cost exceeded the threshold. It is always percent and has to be between 0 and 1000."]
    pub threshold: f64,
    #[doc = "Email addresses to send the budget notification to when the threshold is exceeded."]
    #[serde(rename = "contactEmails")]
    pub contact_emails: Vec<String>,
    #[doc = "Contact roles to send the budget notification to when the threshold is exceeded."]
    #[serde(rename = "contactRoles", default, skip_serializing_if = "Vec::is_empty")]
    pub contact_roles: Vec<String>,
    #[doc = "Action groups to send the budget notification to when the threshold is exceeded."]
    #[serde(rename = "contactGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub contact_groups: Vec<String>,
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
}
#[doc = "A Consumption REST API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
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
    #[serde(rename = "usageDate", default, with = "azure_core::date::rfc3339::option")]
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
#[doc = "Reservation recommendation resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationRecommendation {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(flatten)]
    pub resource_attributes: ResourceAttributes,
    #[doc = "The properties of the reservation recommendation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationRecommendationProperties>,
}
impl ReservationRecommendation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the reservation recommendation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationRecommendationProperties {
    #[doc = "The number of days of usage to look back for recommendation."]
    #[serde(rename = "lookBackPeriod", default, skip_serializing_if = "Option::is_none")]
    pub look_back_period: Option<String>,
    #[doc = "The meter id (GUID)"]
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
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
    #[serde(rename = "firstUsageDate", default, with = "azure_core::date::rfc3339::option")]
    pub first_usage_date: Option<time::OffsetDateTime>,
    #[doc = "Shared or single recommendation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
impl ReservationRecommendationProperties {
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
    #[serde(rename = "usageDate", default, with = "azure_core::date::rfc3339::option")]
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
}
impl ReservationSummaryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
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
#[doc = "The tag resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tag {
    #[doc = "Tag key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageDetail {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of the usage detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UsageDetailProperties>,
}
impl UsageDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the usage detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageDetailProperties {
    #[doc = "Billing Account identifier."]
    #[serde(rename = "billingAccountId", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_id: Option<String>,
    #[doc = "Billing Account Name."]
    #[serde(rename = "billingAccountName", default, skip_serializing_if = "Option::is_none")]
    pub billing_account_name: Option<String>,
    #[doc = "The billing period start date."]
    #[serde(rename = "billingPeriodStartDate", default, with = "azure_core::date::rfc3339::option")]
    pub billing_period_start_date: Option<time::OffsetDateTime>,
    #[doc = "The billing period end date."]
    #[serde(rename = "billingPeriodEndDate", default, with = "azure_core::date::rfc3339::option")]
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
    #[serde(default, with = "azure_core::date::rfc3339::option")]
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
    #[doc = "Effective Price that’s charged for the usage."]
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
    #[doc = "Azure resource manager resource identifier."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Resource Name."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "Service Info 1."]
    #[serde(rename = "serviceInfo1", default, skip_serializing_if = "Option::is_none")]
    pub service_info1: Option<String>,
    #[doc = "Service Info 2."]
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
}
impl UsageDetailProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Download response of Usage Details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageDetailsDownloadResponse {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "A secure URL that can be used to download a an entity until the URL expires."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DownloadUrl>,
}
impl UsageDetailsDownloadResponse {
    pub fn new() -> Self {
        Self::default()
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
