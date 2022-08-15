#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
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
#[doc = "Key-value pairs of instance details in the legacy format."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InfoField {
    #[doc = "Identifies the name of the instance provisioned by the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}
impl InfoField {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Detailed information about the meter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MeterInfo {
    #[doc = "The unique identifier of the resource."]
    #[serde(rename = "MeterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[doc = "The name of the meter, within the given meter category"]
    #[serde(rename = "MeterName", default, skip_serializing_if = "Option::is_none")]
    pub meter_name: Option<String>,
    #[doc = "The category of the meter, e.g., 'Cloud services', 'Networking', etc.."]
    #[serde(rename = "MeterCategory", default, skip_serializing_if = "Option::is_none")]
    pub meter_category: Option<String>,
    #[doc = "The subcategory of the meter, e.g., 'A6 Cloud services', 'ExpressRoute (IXP)', etc.."]
    #[serde(rename = "MeterSubCategory", default, skip_serializing_if = "Option::is_none")]
    pub meter_sub_category: Option<String>,
    #[doc = "The unit in which the meter consumption is charged, e.g., 'Hours', 'GB', etc."]
    #[serde(rename = "Unit", default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Provides additional meter data. 'Third Party' indicates a meter with no discount. Blanks indicate First Party."]
    #[serde(rename = "MeterTags", default, skip_serializing_if = "Vec::is_empty")]
    pub meter_tags: Vec<String>,
    #[doc = "The region in which the Azure service is available."]
    #[serde(rename = "MeterRegion", default, skip_serializing_if = "Option::is_none")]
    pub meter_region: Option<String>,
    #[doc = "The list of key/value pairs for the meter rates, in the format 'key':'value' where key = the meter quantity, and value = the corresponding price"]
    #[serde(rename = "MeterRates", default, skip_serializing_if = "Option::is_none")]
    pub meter_rates: Option<serde_json::Value>,
    #[doc = "Indicates the date from which the meter rate is effective."]
    #[serde(rename = "EffectiveDate", with = "azure_core::date::rfc3339::option")]
    pub effective_date: Option<time::OffsetDateTime>,
    #[doc = "The resource quantity that is included in the offer at no cost. Consumption beyond this quantity will be charged."]
    #[serde(rename = "IncludedQuantity", default, skip_serializing_if = "Option::is_none")]
    pub included_quantity: Option<f32>,
}
impl MeterInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Indicates that a monetary commitment is required for this offer"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonetaryCommitment {
    #[serde(flatten)]
    pub offer_term_info: OfferTermInfo,
    #[doc = "The list of key/value pairs for the tiered meter rates, in the format 'key':'value' where key = price, and value = the corresponding discount percentage. This field is used only by offer terms of type 'Monetary Commitment'."]
    #[serde(rename = "TieredDiscount", default, skip_serializing_if = "Option::is_none")]
    pub tiered_discount: Option<serde_json::Value>,
    #[doc = "An array of meter ids that are excluded from the given offer terms."]
    #[serde(rename = "ExcludedMeterIds", default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_meter_ids: Vec<String>,
}
impl MonetaryCommitment {
    pub fn new(offer_term_info: OfferTermInfo) -> Self {
        Self {
            offer_term_info,
            tiered_discount: None,
            excluded_meter_ids: Vec::new(),
        }
    }
}
#[doc = "Indicates that this is a monetary credit offer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MonetaryCredit {
    #[serde(flatten)]
    pub offer_term_info: OfferTermInfo,
    #[doc = "The amount of credit provided under the terms of the given offer level."]
    #[serde(rename = "Credit", default, skip_serializing_if = "Option::is_none")]
    pub credit: Option<f64>,
    #[doc = "An array of meter ids that are excluded from the given offer terms."]
    #[serde(rename = "ExcludedMeterIds", default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_meter_ids: Vec<String>,
}
impl MonetaryCredit {
    pub fn new(offer_term_info: OfferTermInfo) -> Self {
        Self {
            offer_term_info,
            credit: None,
            excluded_meter_ids: Vec::new(),
        }
    }
}
#[doc = "Describes the offer term."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfferTermInfo {
    #[doc = "Name of the offer term"]
    #[serde(rename = "Name")]
    pub name: offer_term_info::Name,
    #[doc = "Indicates the date from which the offer term is effective."]
    #[serde(rename = "EffectiveDate", with = "azure_core::date::rfc3339::option")]
    pub effective_date: Option<time::OffsetDateTime>,
}
impl OfferTermInfo {
    pub fn new(name: offer_term_info::Name) -> Self {
        Self {
            name,
            effective_date: None,
        }
    }
}
pub mod offer_term_info {
    use super::*;
    #[doc = "Name of the offer term"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "Recurring Charge")]
        RecurringCharge,
        #[serde(rename = "Monetary Commitment")]
        MonetaryCommitment,
        #[serde(rename = "Monetary Credit")]
        MonetaryCredit,
    }
}
#[doc = "Parameters that are used in the odata $filter query parameter for providing RateCard information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RateCardQueryParameters {
    #[doc = "The Offer ID parameter consists of the 'MS-AZR-' prefix, plus the Offer ID number (e.g., MS-AZR-0026P). See https://azure.microsoft.com/en-us/support/legal/offer-details/ for more information on the list of available Offer IDs, country/region availability, and billing currency."]
    #[serde(rename = "OfferDurableId")]
    pub offer_durable_id: String,
    #[doc = "The currency in which the rates need to be provided."]
    #[serde(rename = "Currency")]
    pub currency: String,
    #[doc = "The culture in which the resource metadata needs to be localized."]
    #[serde(rename = "Locale")]
    pub locale: String,
    #[doc = "2 letter ISO code where the offer was purchased."]
    #[serde(rename = "RegionInfo")]
    pub region_info: String,
}
impl RateCardQueryParameters {
    pub fn new(offer_durable_id: String, currency: String, locale: String, region_info: String) -> Self {
        Self {
            offer_durable_id,
            currency,
            locale,
            region_info,
        }
    }
}
#[doc = "Indicates a recurring charge is present for this offer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecurringCharge {
    #[serde(flatten)]
    pub offer_term_info: OfferTermInfo,
    #[doc = "The amount of recurring charge as per the offer term."]
    #[serde(rename = "RecurringCharge", default, skip_serializing_if = "Option::is_none")]
    pub recurring_charge: Option<i64>,
}
impl RecurringCharge {
    pub fn new(offer_term_info: OfferTermInfo) -> Self {
        Self {
            offer_term_info,
            recurring_charge: None,
        }
    }
}
#[doc = "Price and Metadata information for resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceRateCardInfo {
    #[doc = "The currency in which the rates are provided."]
    #[serde(rename = "Currency", default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The culture in which the resource information is localized."]
    #[serde(rename = "Locale", default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[doc = "All rates are pretax, so this will always be returned as 'false'."]
    #[serde(rename = "IsTaxIncluded", default, skip_serializing_if = "Option::is_none")]
    pub is_tax_included: Option<bool>,
    #[doc = "A list of offer terms."]
    #[serde(rename = "OfferTerms", default, skip_serializing_if = "Vec::is_empty")]
    pub offer_terms: Vec<OfferTermInfo>,
    #[doc = "A list of meters."]
    #[serde(rename = "Meters", default, skip_serializing_if = "Vec::is_empty")]
    pub meters: Vec<MeterInfo>,
}
impl ResourceRateCardInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the usageAggregation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageAggregation {
    #[doc = "Unique Id for the usage aggregate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the usage aggregate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the resource being returned."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Describes a sample of the usageAggregation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UsageSample>,
}
impl UsageAggregation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Get UsageAggregates operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageAggregationListResult {
    #[doc = "Gets or sets details for the requested aggregation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UsageAggregation>,
    #[doc = "Gets or sets the link to the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UsageAggregationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl UsageAggregationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a sample of the usageAggregation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageSample {
    #[doc = "The subscription identifier for the Azure user."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Unique ID for the resource that was consumed (aka ResourceID)."]
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[doc = "UTC start time for the usage bucket to which this usage aggregate belongs."]
    #[serde(rename = "usageStartTime", with = "azure_core::date::rfc3339::option")]
    pub usage_start_time: Option<time::OffsetDateTime>,
    #[doc = "UTC end time for the usage bucket to which this usage aggregate belongs."]
    #[serde(rename = "usageEndTime", with = "azure_core::date::rfc3339::option")]
    pub usage_end_time: Option<time::OffsetDateTime>,
    #[doc = "The amount of the resource consumption that occurred in this time frame."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f32>,
    #[doc = "The unit in which the usage for this resource is being counted, e.g. Hours, GB."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Friendly name of the resource being consumed."]
    #[serde(rename = "meterName", default, skip_serializing_if = "Option::is_none")]
    pub meter_name: Option<String>,
    #[doc = "Category of the consumed resource."]
    #[serde(rename = "meterCategory", default, skip_serializing_if = "Option::is_none")]
    pub meter_category: Option<String>,
    #[doc = "Sub-category of the consumed resource."]
    #[serde(rename = "meterSubCategory", default, skip_serializing_if = "Option::is_none")]
    pub meter_sub_category: Option<String>,
    #[doc = "Region of the meterId used for billing purposes"]
    #[serde(rename = "meterRegion", default, skip_serializing_if = "Option::is_none")]
    pub meter_region: Option<String>,
    #[doc = "Key-value pairs of instance details in the legacy format."]
    #[serde(rename = "infoFields", default, skip_serializing_if = "Option::is_none")]
    pub info_fields: Option<InfoField>,
    #[doc = "Key-value pairs of instance details represented as a string."]
    #[serde(rename = "instanceData", default, skip_serializing_if = "Option::is_none")]
    pub instance_data: Option<String>,
}
impl UsageSample {
    pub fn new() -> Self {
        Self::default()
    }
}
