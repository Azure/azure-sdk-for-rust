#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Plan level resources and configuration files"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Artifact {
    #[doc = "Artifact name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Artifact uri"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "The type of the artifact. Possible values:\n- `Custom` - Custom artifact type\n- `Fragment` - Contains link to the nested ARM template\n- `Metadata` - Metadata artifact type\n- `Template` - Contains link to the main ARM template "]
    #[serde(rename = "artifactType")]
    pub artifact_type: serde_json::Value,
}
impl Artifact {
    pub fn new(artifact_type: serde_json::Value) -> Self {
        Self {
            name: None,
            uri: None,
            artifact_type,
        }
    }
}
#[doc = "The type of the artifact"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ArtifactType")]
pub enum ArtifactType {
    Template,
    Fragment,
    Custom,
    Metadata,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ArtifactType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ArtifactType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ArtifactType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Template => serializer.serialize_unit_variant("ArtifactType", 0u32, "Template"),
            Self::Fragment => serializer.serialize_unit_variant("ArtifactType", 1u32, "Fragment"),
            Self::Custom => serializer.serialize_unit_variant("ArtifactType", 2u32, "Custom"),
            Self::Metadata => serializer.serialize_unit_variant("ArtifactType", 3u32, "Metadata"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Availability for a given plan"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Availability {
    #[doc = "The document id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "List of allowed actions"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub actions: Vec<String>,
    #[doc = "Applicable billing meter information"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meter: Option<serde_json::Value>,
    #[doc = "Supported pricing audiences. Possible values:\n- `DirectCommercial` -  Available to commercial direct channel\n- `PartnerCommercial` - Available to Cloud Solution Providers (CSP)"]
    #[serde(rename = "pricingAudience")]
    pub pricing_audience: serde_json::Value,
    #[doc = "List of applicable terms"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub terms: Vec<Term>,
    #[doc = "A value indicating whether it has free trials"]
    #[serde(rename = "hasFreeTrials")]
    pub has_free_trials: bool,
    #[doc = "Consumption unit type"]
    #[serde(rename = "consumptionUnitType", default, skip_serializing_if = "Option::is_none")]
    pub consumption_unit_type: Option<String>,
    #[doc = "Display rank"]
    #[serde(rename = "displayRank")]
    pub display_rank: i32,
}
impl Availability {
    pub fn new(pricing_audience: serde_json::Value, has_free_trials: bool, display_rank: i32) -> Self {
        Self {
            id: None,
            actions: Vec::new(),
            meter: None,
            pricing_audience,
            terms: Vec::new(),
            has_free_trials,
            consumption_unit_type: None,
            display_rank,
        }
    }
}
#[doc = "Eligibility for Azure consumption commitment benefit (https://docs.microsoft.com/en-us/marketplace/azure-consumption-commitment-benefit)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureBenefit")]
pub enum AzureBenefit {
    Eligible,
    NotEligible,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureBenefit {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureBenefit {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureBenefit {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Eligible => serializer.serialize_unit_variant("AzureBenefit", 0u32, "Eligible"),
            Self::NotEligible => serializer.serialize_unit_variant("AzureBenefit", 1u32, "NotEligible"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Badge type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Badge")]
pub enum Badge {
    PreferredSolution,
    #[serde(rename = "PowerBICertified")]
    PowerBiCertified,
    AdditionalPurchaseRequirement,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Badge {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Badge {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Badge {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::PreferredSolution => serializer.serialize_unit_variant("Badge", 0u32, "PreferredSolution"),
            Self::PowerBiCertified => serializer.serialize_unit_variant("Badge", 1u32, "PowerBICertified"),
            Self::AdditionalPurchaseRequirement => serializer.serialize_unit_variant("Badge", 2u32, "AdditionalPurchaseRequirement"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents a billing component model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingComponent {
    #[doc = "Billing Tag"]
    #[serde(rename = "billingTag", default, skip_serializing_if = "Option::is_none")]
    pub billing_tag: Option<String>,
    #[doc = "Custom Meter Ids"]
    #[serde(rename = "customMeterIds", default, skip_serializing_if = "Option::is_none")]
    pub custom_meter_ids: Option<serde_json::Value>,
}
impl BillingComponent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Renew Billing Plan"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BillingPlan {
    #[doc = "Billing period"]
    #[serde(rename = "billingPeriod", default, skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<String>,
    #[doc = "Title"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Price"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<serde_json::Value>,
}
impl BillingPlan {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response page composed of a list of Product Summaries together with a next page link"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CatalogApiResponse {
    #[doc = "The list of returned items"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ProductSummary>,
    #[doc = "Link to the next page where request returns more than a single page of results"]
    #[serde(rename = "nextPageLink", default, skip_serializing_if = "Option::is_none")]
    pub next_page_link: Option<String>,
}
impl CatalogApiResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Value showing whether the plan is available for purchase through CSP channel. For more information see, https://docs.microsoft.com/en-us/azure/marketplace/cloud-solution-providers"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CspState")]
pub enum CspState {
    OptIn,
    OptOut,
    SelectiveOptIn,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CspState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CspState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CspState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::OptIn => serializer.serialize_unit_variant("CspState", 0u32, "OptIn"),
            Self::OptOut => serializer.serialize_unit_variant("CspState", 1u32, "OptOut"),
            Self::SelectiveOptIn => serializer.serialize_unit_variant("CspState", 2u32, "SelectiveOptIn"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The search error response details object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[doc = "The error code description. Such as code=InternalError"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message. Such as message=Internal server error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "List of error response details"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetails>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Catalog API error response object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[doc = "The Catalog API error response details object"]
    pub error: ErrorResponseDetails,
}
impl ErrorResponse {
    pub fn new(error: ErrorResponseDetails) -> Self {
        Self { error }
    }
}
#[doc = "The Catalog API error response details object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponseDetails {
    #[doc = "The error code description. Such as code=InternalError"]
    pub code: String,
    #[doc = "The error message. Such as message=Internal server error"]
    pub message: String,
}
impl ErrorResponseDetails {
    pub fn new(code: String, message: String) -> Self {
        Self { code, message }
    }
}
#[doc = "The facet value"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FacetValue {
    #[doc = "The facet count"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl FacetValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The facets item"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FacetsItem {
    #[doc = "The facet values"]
    #[serde(rename = "facetValues", default, skip_serializing_if = "Option::is_none")]
    pub facet_values: Option<serde_json::Value>,
}
impl FacetsItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The facets response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FacetsResponse {
    #[doc = "The facet items results"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}
impl FacetsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Image item"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Image {
    #[doc = "Image id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Image uri"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "Image type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Image {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Image Group"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageGroup {
    #[doc = "context"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[doc = "List of images"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<Image>,
}
impl ImageGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Included quantity properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IncludedQuantityProperty {
    #[doc = "Term id"]
    #[serde(rename = "termId", default, skip_serializing_if = "Option::is_none")]
    pub term_id: Option<String>,
    #[doc = "Quantity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
}
impl IncludedQuantityProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Industry Cloud"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IndustryCloud")]
pub enum IndustryCloud {
    NotApplicable,
    True,
    False,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IndustryCloud {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IndustryCloud {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IndustryCloud {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotApplicable => serializer.serialize_unit_variant("IndustryCloud", 0u32, "NotApplicable"),
            Self::True => serializer.serialize_unit_variant("IndustryCloud", 1u32, "True"),
            Self::False => serializer.serialize_unit_variant("IndustryCloud", 2u32, "False"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Supported legal terms type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LegalTermsType")]
pub enum LegalTermsType {
    None,
    #[serde(rename = "EA")]
    Ea,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LegalTermsType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LegalTermsType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LegalTermsType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("LegalTermsType", 0u32, "None"),
            Self::Ea => serializer.serialize_unit_variant("LegalTermsType", 1u32, "EA"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents a link item read from the gallery item package"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkProperties {
    #[doc = "Id of the link"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Display name of the link"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "URI of the link"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl LinkProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Retail price of the product's cheapest plan"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketStartPrice {
    #[doc = "The market for which the starting price is calculated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[doc = "The terms pricing model units"]
    #[serde(rename = "termUnits", default, skip_serializing_if = "Option::is_none")]
    pub term_units: Option<String>,
    #[doc = "The meters pricing model units"]
    #[serde(rename = "meterUnits", default, skip_serializing_if = "Option::is_none")]
    pub meter_units: Option<String>,
    #[doc = "Starting (minimal) terms price"]
    #[serde(rename = "minTermPrice", default, skip_serializing_if = "Option::is_none")]
    pub min_term_price: Option<f64>,
    #[doc = "Starting (minimal) meters price"]
    #[serde(rename = "minMeterPrice", default, skip_serializing_if = "Option::is_none")]
    pub min_meter_price: Option<f64>,
    #[doc = "Currency for price"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}
impl MarketStartPrice {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Defines the marketing material from the publisher"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketingMaterial {
    #[doc = "Path to the marketing site"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Path to the learn site"]
    #[serde(rename = "learnUri", default, skip_serializing_if = "Option::is_none")]
    pub learn_uri: Option<String>,
}
impl MarketingMaterial {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Applicable billing meter information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Meter {
    #[doc = "Meter id"]
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[doc = "Compute part number"]
    #[serde(rename = "partNumber", default, skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
    #[doc = "Consumption resource id"]
    #[serde(rename = "consumptionResourceId", default, skip_serializing_if = "Option::is_none")]
    pub consumption_resource_id: Option<String>,
    #[doc = "Price for this meter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<serde_json::Value>,
    #[doc = "Type of this meter"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Included quantity properties"]
    #[serde(
        rename = "includedQuantityProperties",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub included_quantity_properties: Vec<IncludedQuantityProperty>,
}
impl Meter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Summary description of the plan"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanDetails {
    #[doc = "Plan id"]
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[doc = "Unique plan Id which is prefixed by combining uniqueProductId and PlanId with no separator in between"]
    #[serde(rename = "uniquePlanId", default, skip_serializing_if = "Option::is_none")]
    pub unique_plan_id: Option<String>,
    #[doc = "Display name of the plan"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "VM Architecture Type. Applicable to Virtual Machine products only. PossibleValues:\n- `Arm64` - The VM image architecture of the plan is Arm64\n- `X64Gen1` - The VM image architecture of the plan is X64 Generation1\n- `X64Gen2` - The VM image architecture of the plan is X64 Generation2"]
    #[serde(rename = "vmArchitectureType", default, skip_serializing_if = "Option::is_none")]
    pub vm_architecture_type: Option<serde_json::Value>,
    #[doc = "Value showing whether the plan is available for purchase through CSP channel. Possible values: \n- `OptIn` - Plan configured CSP program opt-in for any partner in the CSP Program\n- `Optout` - Plan configured CSP program opt-out\n- `SelectiveOptIn` - Plan configured CSP program opt-in for specific partners in the CSP program ISV selects"]
    #[serde(rename = "cspState", default, skip_serializing_if = "Option::is_none")]
    pub csp_state: Option<serde_json::Value>,
    #[doc = "Additional plan metadata"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "Alternative stack reference"]
    #[serde(rename = "altStackReference", default, skip_serializing_if = "Option::is_none")]
    pub alt_stack_reference: Option<String>,
    #[doc = "Stack type : Classic, Gen1, Gen2"]
    #[serde(rename = "stackType", default, skip_serializing_if = "Option::is_none")]
    pub stack_type: Option<String>,
    #[doc = "The alternative architecture reference"]
    #[serde(rename = "altArchitectureReference", default, skip_serializing_if = "Option::is_none")]
    pub alt_architecture_reference: Option<String>,
    #[doc = "Plan CategoryIds"]
    #[serde(
        rename = "categoryIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub category_ids: Vec<String>,
    #[doc = "Set to true if plan has artifacts that are to be hidden for non authenticated users"]
    #[serde(rename = "hasProtectedArtifacts", default, skip_serializing_if = "Option::is_none")]
    pub has_protected_artifacts: Option<bool>,
    #[doc = "The list of the pricing types for which this plan is designated (Possible values are the same as at product level)"]
    #[serde(
        rename = "pricingTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub pricing_types: Vec<PricingType>,
    #[doc = "VM security type. Applicable to Virtual Machine products only. Possible values:\n- Standard - Basic level of security for the virtual machine\n- Trusted - protects against persistent and advanced attacks on Gen2 virtual machines with configurable features like security boot and virtual Trusted Platform Module(vTPM)\n- Confidential - on top of Trusted Launch, Confidential virtual machines offer higher confidentiality and integrity guaranteed with hardware-based trusted execution environment"]
    #[serde(
        rename = "vmSecuritytypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vm_securitytypes: Vec<VmSecurityType>,
    #[doc = "The plan summary text"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[doc = "Plan description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The SKU id"]
    #[serde(rename = "skuId", default, skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[doc = "Managed Application or Solution template plan type (applicable only to Azure Applications product). Possible values: \n- `SolutionTemplate` - Solution Template plan type (applicable to Azure Applications only)\n- `ManagedApplication` - Managed Application plan type (applicable to Azure Applications only)"]
    #[serde(rename = "planType")]
    pub plan_type: serde_json::Value,
    #[doc = "The order the plan is displayed in the 'Plans' table (as configured by the publisher)"]
    #[serde(rename = "displayRank", default, skip_serializing_if = "Option::is_none")]
    pub display_rank: Option<String>,
    #[doc = "Indication that the plan is accessible to restricted audience only"]
    #[serde(rename = "isPrivate", default, skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    #[doc = "The id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "List of availabilities for this plan"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub availabilities: Vec<Availability>,
    #[doc = "What blade to open when someone wants to create a marketplace item"]
    #[serde(rename = "uiDefinitionUri", default, skip_serializing_if = "Option::is_none")]
    pub ui_definition_uri: Option<String>,
    #[doc = "Files related to the marketplace item"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub artifacts: Vec<Artifact>,
    #[doc = "Version of the marketplace item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "A value indicating whether the plan is hidden"]
    #[serde(rename = "isHidden")]
    pub is_hidden: bool,
    #[doc = "A value indicating whether plan has been stopped from selling"]
    #[serde(rename = "isStopSell")]
    pub is_stop_sell: bool,
    #[doc = "A StopSellInfo object providing stop sell related data"]
    #[serde(rename = "stopSellInfo", default, skip_serializing_if = "Option::is_none")]
    pub stop_sell_info: Option<serde_json::Value>,
    #[doc = "Min quantity"]
    #[serde(rename = "minQuantity", default, skip_serializing_if = "Option::is_none")]
    pub min_quantity: Option<i32>,
    #[doc = "Max quantity"]
    #[serde(rename = "maxQuantity", default, skip_serializing_if = "Option::is_none")]
    pub max_quantity: Option<i32>,
    #[doc = "A value indicating whether the plan is quantifiable"]
    #[serde(rename = "isQuantifiable")]
    pub is_quantifiable: bool,
    #[doc = "Billing components"]
    #[serde(
        rename = "billingComponents",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub billing_components: Vec<BillingComponent>,
    #[doc = "List of purchase duration discounts"]
    #[serde(
        rename = "purchaseDurationDiscounts",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub purchase_duration_discounts: Vec<PurchaseDurationDiscount>,
    #[doc = "A value indicating whether the plan is used as hidden private offer requiring a quote"]
    #[serde(rename = "isHiddenPrivateOffer", default, skip_serializing_if = "Option::is_none")]
    pub is_hidden_private_offer: Option<bool>,
    #[doc = "List of certifications"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub certifications: Vec<LinkProperties>,
    #[doc = "Plan's customer instruction for purchasing"]
    #[serde(rename = "customerInstruction", default, skip_serializing_if = "Option::is_none")]
    pub customer_instruction: Option<String>,
}
impl PlanDetails {
    pub fn new(plan_type: serde_json::Value, is_hidden: bool, is_stop_sell: bool, is_quantifiable: bool) -> Self {
        Self {
            plan_id: None,
            unique_plan_id: None,
            display_name: None,
            vm_architecture_type: None,
            csp_state: None,
            metadata: None,
            alt_stack_reference: None,
            stack_type: None,
            alt_architecture_reference: None,
            category_ids: Vec::new(),
            has_protected_artifacts: None,
            pricing_types: Vec::new(),
            vm_securitytypes: Vec::new(),
            summary: None,
            description: None,
            sku_id: None,
            plan_type,
            display_rank: None,
            is_private: None,
            id: None,
            availabilities: Vec::new(),
            ui_definition_uri: None,
            artifacts: Vec::new(),
            version: None,
            is_hidden,
            is_stop_sell,
            stop_sell_info: None,
            min_quantity: None,
            max_quantity: None,
            is_quantifiable,
            billing_components: Vec::new(),
            purchase_duration_discounts: Vec::new(),
            is_hidden_private_offer: None,
            certifications: Vec::new(),
            customer_instruction: None,
        }
    }
}
#[doc = "Additional metadata"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanMetadata {
    #[doc = "The VM image generation (applicable to Virtual Machine products only).see https://docs.microsoft.com/en-us/azure/virtual-machines/generation-2"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,
    #[doc = "Pointing to a planId which holds the alternative stack reference"]
    #[serde(rename = "altStackReference", default, skip_serializing_if = "Option::is_none")]
    pub alt_stack_reference: Option<String>,
    #[doc = "Pointing to a planId of plan related to this plan"]
    #[serde(
        rename = "relatedSkus",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub related_skus: Vec<PlanSkuRelation>,
}
impl PlanMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Related plan information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanSkuRelation {
    #[doc = "The Sku"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<serde_json::Value>,
    #[doc = "Relation Type"]
    #[serde(rename = "relationType", default, skip_serializing_if = "Option::is_none")]
    pub relation_type: Option<String>,
}
impl PlanSkuRelation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Summary description of the plan"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanSummary {
    #[doc = "Plan id"]
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[doc = "Unique plan Id which is prefixed by combining uniqueProductId and PlanId with no separator in between"]
    #[serde(rename = "uniquePlanId", default, skip_serializing_if = "Option::is_none")]
    pub unique_plan_id: Option<String>,
    #[doc = "Display name of the plan"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "VM Architecture Type. Applicable to Virtual Machine products only. PossibleValues:\n- `Arm64` - The VM image architecture of the plan is Arm64\n- `X64Gen1` - The VM image architecture of the plan is X64 Generation1\n- `X64Gen2` - The VM image architecture of the plan is X64 Generation2"]
    #[serde(rename = "vmArchitectureType", default, skip_serializing_if = "Option::is_none")]
    pub vm_architecture_type: Option<serde_json::Value>,
    #[doc = "A value indicating whether the product is available for purchase through CSP channel"]
    #[serde(rename = "cspState", default, skip_serializing_if = "Option::is_none")]
    pub csp_state: Option<serde_json::Value>,
    #[doc = "Additional plan metadata"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "Alternative stack reference"]
    #[serde(rename = "altStackReference", default, skip_serializing_if = "Option::is_none")]
    pub alt_stack_reference: Option<String>,
    #[doc = "Stack type : Classic, Gen1, Gen2"]
    #[serde(rename = "stackType", default, skip_serializing_if = "Option::is_none")]
    pub stack_type: Option<String>,
    #[doc = "The alternative architecture reference"]
    #[serde(rename = "altArchitectureReference", default, skip_serializing_if = "Option::is_none")]
    pub alt_architecture_reference: Option<String>,
    #[doc = "Plan CategoryIds"]
    #[serde(
        rename = "categoryIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub category_ids: Vec<String>,
    #[doc = "Set to true if plan has artifacts that are to be hidden for non authenticated users"]
    #[serde(rename = "hasProtectedArtifacts", default, skip_serializing_if = "Option::is_none")]
    pub has_protected_artifacts: Option<bool>,
    #[doc = "The list of the pricing types for which this plan is designated (Possible values are the same as at product level)"]
    #[serde(
        rename = "pricingTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub pricing_types: Vec<PricingType>,
    #[doc = "VM security type. Applicable to Virtual Machine products only. Possible values:\n- Standard - Basic level of security for the virtual machine\n- Trusted - protects against persistent and advanced attacks on Gen2 virtual machines with configurable features like security boot and virtual Trusted Platform Module(vTPM)\n- Confidential - on top of Trusted Launch, Confidential virtual machines offer higher confidentiality and integrity guaranteed with hardware-based trusted execution environment"]
    #[serde(
        rename = "vmSecuritytypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vm_securitytypes: Vec<VmSecurityType>,
    #[doc = "The plan summary text"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[doc = "Plan description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The SKU id"]
    #[serde(rename = "skuId", default, skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[doc = "Managed Application or Solution template plan type (applicable only to Azure Applications product). Possible values: \n- `SolutionTemplate` - Solution Template plan type (applicable to Azure Applications only)\n- `ManagedApplication` - Managed Application plan type (applicable to Azure Applications only)"]
    #[serde(rename = "planType")]
    pub plan_type: serde_json::Value,
    #[doc = "The order the plan is displayed in the 'Plans' table (as configured by the publisher)"]
    #[serde(rename = "displayRank", default, skip_serializing_if = "Option::is_none")]
    pub display_rank: Option<String>,
    #[doc = "Indication that the plan is accessible to restricted audience only"]
    #[serde(rename = "isPrivate", default, skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
}
impl PlanSummary {
    pub fn new(plan_type: serde_json::Value) -> Self {
        Self {
            plan_id: None,
            unique_plan_id: None,
            display_name: None,
            vm_architecture_type: None,
            csp_state: None,
            metadata: None,
            alt_stack_reference: None,
            stack_type: None,
            alt_architecture_reference: None,
            category_ids: Vec::new(),
            has_protected_artifacts: None,
            pricing_types: Vec::new(),
            vm_securitytypes: Vec::new(),
            summary: None,
            description: None,
            sku_id: None,
            plan_type,
            display_rank: None,
            is_private: None,
        }
    }
}
#[doc = "Video preview image"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PreviewImage {
    #[doc = "caption"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[doc = "uri"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "imagePurpose"]
    #[serde(rename = "imagePurpose", default, skip_serializing_if = "Option::is_none")]
    pub image_purpose: Option<String>,
}
impl PreviewImage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Pricing details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Price {
    #[doc = "Currency code i.e. 'USD'"]
    #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[doc = "A value indicating whether a payment instrument is required"]
    #[serde(rename = "isPIRequired")]
    pub is_pi_required: bool,
    #[doc = "Retail price for the item"]
    #[serde(rename = "listPrice")]
    pub list_price: f64,
    #[doc = "Manufacturer's suggested retail price for the item"]
    pub msrp: f64,
}
impl Price {
    pub fn new(is_pi_required: bool, list_price: f64, msrp: f64) -> Self {
        Self {
            currency_code: None,
            is_pi_required,
            list_price,
            msrp,
        }
    }
}
#[doc = "Supported pricing audiences"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PricingAudience")]
pub enum PricingAudience {
    DirectCommercial,
    PartnerCommercial,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PricingAudience {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PricingAudience {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PricingAudience {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::DirectCommercial => serializer.serialize_unit_variant("PricingAudience", 0u32, "DirectCommercial"),
            Self::PartnerCommercial => serializer.serialize_unit_variant("PricingAudience", 1u32, "PartnerCommercial"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Pricing type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PricingType")]
pub enum PricingType {
    Free,
    FreeTrial,
    Byol,
    Payg,
    Ri,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PricingType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PricingType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PricingType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Free => serializer.serialize_unit_variant("PricingType", 0u32, "Free"),
            Self::FreeTrial => serializer.serialize_unit_variant("PricingType", 1u32, "FreeTrial"),
            Self::Byol => serializer.serialize_unit_variant("PricingType", 2u32, "Byol"),
            Self::Payg => serializer.serialize_unit_variant("PricingType", 3u32, "Payg"),
            Self::Ri => serializer.serialize_unit_variant("PricingType", 4u32, "Ri"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Summary description of the product"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductDetails {
    #[doc = "Product display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Popularity of the product"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub popularity: Option<f64>,
    #[doc = "Array of product categories, https://docs.microsoft.com/en-us/azure/marketplace/marketplace-categories-industries#categories. Such as 'Productivity'"]
    #[serde(
        rename = "categoryIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub category_ids: Vec<String>,
    #[doc = "List of industry IDs the item belongs to"]
    #[serde(
        rename = "industryIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub industry_ids: Vec<String>,
    #[doc = "Publisher id"]
    #[serde(rename = "publisherId", default, skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
    #[doc = "Eligibility for Azure consumption commitment benefit (https://docs.microsoft.com/en-us/marketplace/azure-consumption-commitment-benefit). Possible values: \n- `Eligible` - Product is eligible for Azure consumption commitment benefit\n- `NotEligible` - Product is not eligible for Azure consumption commitment benefit"]
    #[serde(rename = "azureBenefit", default, skip_serializing_if = "Option::is_none")]
    pub azure_benefit: Option<serde_json::Value>,
    #[doc = "The following product badges are available: preferredSolution, powerBICertified, AdditionalPurchaseRequirement"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub badges: Vec<Badge>,
    #[doc = "A value indicating whether this is a Microsoft or third party product. Possible values:\n- `Microsoft` - Product is published by Microsoft (1st party product)\n- `Third Party` - Product is published by Third Party Vendor"]
    #[serde(rename = "publisherType", default, skip_serializing_if = "Option::is_none")]
    pub publisher_type: Option<serde_json::Value>,
    #[doc = "Product publishing stage. Possible values:\n- `Preview` - Product is available to restricted audience\n- `Public` - Product is publicly available in the Marketplace"]
    #[serde(rename = "publishingStage", default, skip_serializing_if = "Option::is_none")]
    pub publishing_stage: Option<serde_json::Value>,
    #[doc = "Unique product identifier"]
    #[serde(rename = "uniqueProductId", default, skip_serializing_if = "Option::is_none")]
    pub unique_product_id: Option<String>,
    #[doc = "The delivery method or deployment model. Possible values:\n- `SaaS` - The product is a Software solution purchased on pay-as-you-go basis from a cloud service provider\n- `VirtualMachine` -  The product is Image service instances with usage-based pricing\n- `AzureApplication` - Products published as Solution Templates (ST) or Azure Managed Apps (AMA) plans that deploy Azure resources to the customer tenant \n- `DynamicsCE` - All Dynamics products except for DynamicsBC and DynamicsOps\n- `DynamicsBC` - Dynamics 365 business Central \n- `Container` -  The product is delivered as Container\n- `DynamicsOps` - Dynamics 365 Operations Apps\n- `None` - Product type returned for Azure Services, Container Extensions and VM extensions\n- `IotEdgeModules` - Resource is delivered as IoT Edge Module\n- `ContainerApps` - Resource is delivered as Container App\n- `ConsultingServices` Consulting Services (available only in Azure Marketplace as listing offers)\n- `ManagedServices` - The product is delivered as Managed Service\n- `PowerBI` - Power BI products\n- `CosellOnly` - Products available in scope of Co-Sell program only\n- `PowerBIVisuals` - Power BI Visuals products\n- `Office365` - Office 365 products\n- `DevService` - Dev Service\n- `AADApps` - Azure Active Directory Applications\n- `VisualStudioExtension` - Visual Studio Extensions"]
    #[serde(rename = "productType")]
    pub product_type: serde_json::Value,
    #[doc = "Operating system info for this product"]
    #[serde(
        rename = "operatingSystems",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operating_systems: Vec<String>,
    #[doc = "The list of the pricing types for which this offer is designated. Possible Values: \n- `Free` - Product has at least one plan that is free of charge.\n- `FreeTrial` - Product has at least one plan that is free trial.\n- `Byol` - Product has at least one plan that is bring your own license.\n- `Payg` - Product has at least one plan that is Pay as you go, usage based billing model.\n- `Ri` - Product has at least one plan that is Reserved Instance billing model."]
    #[serde(
        rename = "pricingTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub pricing_types: Vec<PricingType>,
    #[doc = "Publisher display name"]
    #[serde(rename = "publisherDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_display_name: Option<String>,
    #[doc = "Product long summary"]
    #[serde(rename = "longSummary", default, skip_serializing_if = "Option::is_none")]
    pub long_summary: Option<String>,
    #[doc = "Product summary"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[doc = "Linked add-in types"]
    #[serde(rename = "linkedAddInsTypes", default, skip_serializing_if = "Option::is_none")]
    pub linked_add_ins_types: Option<serde_json::Value>,
    #[doc = "URI to the small product icon"]
    #[serde(rename = "smallIconUri", default, skip_serializing_if = "Option::is_none")]
    pub small_icon_uri: Option<String>,
    #[doc = "The product description text"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Rating buckets, Above1, Above2, Above3, Above4"]
    #[serde(
        rename = "ratingBuckets",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rating_buckets: Vec<RatingBucket>,
    #[doc = "Average rating for the offer"]
    #[serde(rename = "ratingAverage", default, skip_serializing_if = "Option::is_none")]
    pub rating_average: Option<f64>,
    #[doc = "Total number of ratings for the offer"]
    #[serde(rename = "ratingCount", default, skip_serializing_if = "Option::is_none")]
    pub rating_count: Option<i32>,
    #[doc = "Retail price of the product's cheapest plan"]
    #[serde(rename = "startingPrice", default, skip_serializing_if = "Option::is_none")]
    pub starting_price: Option<serde_json::Value>,
    #[doc = "Plans available for the offer details"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub plans: Vec<PlanDetails>,
    #[doc = "Maps to the list of compatible products that the publisher provides"]
    #[serde(
        rename = "supportedProducts",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_products: Vec<String>,
    #[doc = "Product categories the offer belongs to"]
    #[serde(
        rename = "applicableProducts",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub applicable_products: Vec<String>,
    #[doc = "Latest update date of the product"]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "The product language"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[doc = "A value indicating whether standard contract amendments are present for this product"]
    #[serde(rename = "hasStandardContractAmendments")]
    pub has_standard_contract_amendments: bool,
    #[doc = "The offer id"]
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[doc = "Standard contract amendments for the product"]
    #[serde(rename = "standardContractAmendmentsRevisionId")]
    pub standard_contract_amendments_revision_id: String,
    #[doc = "The universal amendment link for an enterprise contract."]
    #[serde(rename = "universalAmendmentUrl", default, skip_serializing_if = "Option::is_none")]
    pub universal_amendment_url: Option<String>,
    #[doc = "A value indicating whether the item is private"]
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
    #[doc = "A value indicating whether sale of this item is stopped"]
    #[serde(rename = "isStopSell")]
    pub is_stop_sell: bool,
    #[doc = "A StopSellInfo object providing stop sell related data"]
    #[serde(rename = "stopSellInfo", default, skip_serializing_if = "Option::is_none")]
    pub stop_sell_info: Option<serde_json::Value>,
    #[doc = "Marketing material added by the publisher"]
    #[serde(rename = "marketingMaterial", default, skip_serializing_if = "Option::is_none")]
    pub marketing_material: Option<serde_json::Value>,
    #[doc = "Legal terms URI"]
    #[serde(rename = "legalTermsUri", default, skip_serializing_if = "Option::is_none")]
    pub legal_terms_uri: Option<String>,
    #[doc = "CSP legal terms URI"]
    #[serde(rename = "cspLegalTermsUri", default, skip_serializing_if = "Option::is_none")]
    pub csp_legal_terms_uri: Option<String>,
    #[doc = "Supported legal terms type. Possible values:\n- `EA` - Microsoft Enterprise Agreement\n- `None` - None"]
    #[serde(rename = "legalTermsType")]
    pub legal_terms_type: serde_json::Value,
    #[doc = "Uri to the privacy policy of the product"]
    #[serde(rename = "privacyPolicyUri", default, skip_serializing_if = "Option::is_none")]
    pub privacy_policy_uri: Option<String>,
    #[doc = "Support uri of the product"]
    #[serde(rename = "supportUri", default, skip_serializing_if = "Option::is_none")]
    pub support_uri: Option<String>,
    #[doc = "User interface definition uri"]
    #[serde(rename = "uiDefinitionUri", default, skip_serializing_if = "Option::is_none")]
    pub ui_definition_uri: Option<String>,
    #[doc = "List of screenshot image URIs provided for the item"]
    #[serde(
        rename = "screenshotUris",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub screenshot_uris: Vec<String>,
    #[doc = "List of Links provided for the item"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub links: Vec<LinkProperties>,
    #[doc = "List of linked Add Ins provided for the item"]
    #[serde(
        rename = "linkedAddIns",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub linked_add_ins: Vec<String>,
    #[doc = "URI to the medium product icon"]
    #[serde(rename = "mediumIconUri", default, skip_serializing_if = "Option::is_none")]
    pub medium_icon_uri: Option<String>,
    #[doc = "URI to the large product icon"]
    #[serde(rename = "largeIconUri", default, skip_serializing_if = "Option::is_none")]
    pub large_icon_uri: Option<String>,
    #[doc = "URI to the wide product icon"]
    #[serde(rename = "wideIconUri", default, skip_serializing_if = "Option::is_none")]
    pub wide_icon_uri: Option<String>,
    #[doc = "List of Images"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub images: Vec<ImageGroup>,
    #[doc = "List of artifacts"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub artifacts: Vec<Artifact>,
    #[doc = "List of product videos"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub videos: Vec<ProductVideo>,
    #[doc = "Dictionary of additional properties provided for the item"]
    #[serde(rename = "additionalProductProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_product_properties: Option<serde_json::Value>,
    #[doc = "Pricing details uri of the product"]
    #[serde(rename = "pricingDetailsUri", default, skip_serializing_if = "Option::is_none")]
    pub pricing_details_uri: Option<String>,
    #[doc = "A value indicating whether microsoft is acting as a reseller"]
    #[serde(rename = "isReseller")]
    pub is_reseller: bool,
    #[doc = "Product ownership selling motion"]
    #[serde(rename = "productOwnershipSellingMotion", default, skip_serializing_if = "Option::is_none")]
    pub product_ownership_selling_motion: Option<String>,
    #[doc = "Indication to disable sending email on purchase"]
    #[serde(rename = "disableSendEmailOnPurchase")]
    pub disable_send_email_on_purchase: bool,
    #[doc = "Set to true only for offers of ProductType.VirtualMachine to indicate that it was originally of ProductType.CoreVirtualMachine"]
    #[serde(rename = "isCoreVm", default, skip_serializing_if = "Option::is_none")]
    pub is_core_vm: Option<bool>,
}
impl ProductDetails {
    pub fn new(
        product_type: serde_json::Value,
        has_standard_contract_amendments: bool,
        standard_contract_amendments_revision_id: String,
        is_private: bool,
        is_stop_sell: bool,
        legal_terms_type: serde_json::Value,
        is_reseller: bool,
        disable_send_email_on_purchase: bool,
    ) -> Self {
        Self {
            display_name: None,
            popularity: None,
            category_ids: Vec::new(),
            industry_ids: Vec::new(),
            publisher_id: None,
            azure_benefit: None,
            badges: Vec::new(),
            publisher_type: None,
            publishing_stage: None,
            unique_product_id: None,
            product_type,
            operating_systems: Vec::new(),
            pricing_types: Vec::new(),
            publisher_display_name: None,
            long_summary: None,
            summary: None,
            linked_add_ins_types: None,
            small_icon_uri: None,
            description: None,
            rating_buckets: Vec::new(),
            rating_average: None,
            rating_count: None,
            starting_price: None,
            plans: Vec::new(),
            supported_products: Vec::new(),
            applicable_products: Vec::new(),
            last_modified_date_time: None,
            language: None,
            has_standard_contract_amendments,
            offer_id: None,
            standard_contract_amendments_revision_id,
            universal_amendment_url: None,
            is_private,
            is_stop_sell,
            stop_sell_info: None,
            marketing_material: None,
            legal_terms_uri: None,
            csp_legal_terms_uri: None,
            legal_terms_type,
            privacy_policy_uri: None,
            support_uri: None,
            ui_definition_uri: None,
            screenshot_uris: Vec::new(),
            links: Vec::new(),
            linked_add_ins: Vec::new(),
            medium_icon_uri: None,
            large_icon_uri: None,
            wide_icon_uri: None,
            images: Vec::new(),
            artifacts: Vec::new(),
            videos: Vec::new(),
            additional_product_properties: None,
            pricing_details_uri: None,
            is_reseller,
            product_ownership_selling_motion: None,
            disable_send_email_on_purchase,
            is_core_vm: None,
        }
    }
}
#[doc = "Summary description of the product"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProductSummary {
    #[doc = "Product display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Popularity of the product"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub popularity: Option<f64>,
    #[doc = "Array of product categories, https://docs.microsoft.com/en-us/azure/marketplace/marketplace-categories-industries#categories. Such as 'Productivity'"]
    #[serde(
        rename = "categoryIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub category_ids: Vec<String>,
    #[doc = "List of industry IDs the item belongs to"]
    #[serde(
        rename = "industryIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub industry_ids: Vec<String>,
    #[doc = "Publisher id"]
    #[serde(rename = "publisherId", default, skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
    #[doc = "Eligibility for Azure consumption commitment benefit (https://docs.microsoft.com/en-us/marketplace/azure-consumption-commitment-benefit). Possible values: \n- `Eligible` - Product is eligible for Azure consumption commitment benefit\n- `NotEligible` - Product is not eligible for Azure consumption commitment benefit"]
    #[serde(rename = "azureBenefit", default, skip_serializing_if = "Option::is_none")]
    pub azure_benefit: Option<serde_json::Value>,
    #[doc = "The following product badges are available: preferredSolution, powerBICertified, AdditionalPurchaseRequirement"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub badges: Vec<Badge>,
    #[doc = "A value indicating whether this is a Microsoft or third party product. Possible values:\n- `Microsoft` - Product is published by Microsoft (1st party product)\n- `Third Party` - Product is published by Third Party Vendor"]
    #[serde(rename = "publisherType", default, skip_serializing_if = "Option::is_none")]
    pub publisher_type: Option<serde_json::Value>,
    #[doc = "Product publishing stage. Possible values:\n- `Preview` - Product is available to restricted audience\n- `Public` - Product is publicly available in the Marketplace"]
    #[serde(rename = "publishingStage", default, skip_serializing_if = "Option::is_none")]
    pub publishing_stage: Option<serde_json::Value>,
    #[doc = "Unique product identifier"]
    #[serde(rename = "uniqueProductId", default, skip_serializing_if = "Option::is_none")]
    pub unique_product_id: Option<String>,
    #[doc = "The delivery method or deployment model. Possible values:\n- `SaaS` - The product is a Software solution purchased on pay-as-you-go basis from a cloud service provider\n- `VirtualMachine` -  The product is Image service instances with usage-based pricing\n- `AzureApplication` - Products published as Solution Templates (ST) or Azure Managed Apps (AMA) plans that deploy Azure resources to the customer tenant \n- `DynamicsCE` - All Dynamics products except for DynamicsBC and DynamicsOps\n- `DynamicsBC` - Dynamics 365 business Central \n- `Container` -  The product is delivered as Container\n- `DynamicsOps` - Dynamics 365 Operations Apps\n- `None` - Product type returned for Azure Services, Container Extensions and VM extensions\n- `IotEdgeModules` - Resource is delivered as IoT Edge Module\n- `ContainerApps` - Resource is delivered as Container App\n- `ConsultingServices` - Consulting Services (available only in Azure Marketplace as listing offers)\n- `ManagedServices` - The product is delivered as Managed Service\n- `PowerBI` - Power BI products\n- `CosellOnly` - Products available in scope of Co-Sell program only\n- `PowerBIVisuals` - Power BI Visuals products\n- `Office365` - Office 365 products\n- `DevService` - Dev Service\n- `AADApps` - Azure Active Directory Applications\n- `VisualStudioExtension` - Visual Studio Extensions"]
    #[serde(rename = "productType")]
    pub product_type: serde_json::Value,
    #[doc = "Operating system info for this product"]
    #[serde(
        rename = "operatingSystems",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operating_systems: Vec<String>,
    #[doc = "The list of the pricing types for which this offer is designated. Possible Values: \n- `Free` - Product has at least one plan that is free of charge.\n- `FreeTrial` - Product has at least one plan that is free trial.\n- `Byol` - Product has at least one plan that is bring your own license.\n- `Payg` - Product has at least one plan that is Pay as you go, usage based billing model.\n- `Ri` - Product has at least one plan that is Reserved Instance billing model."]
    #[serde(
        rename = "pricingTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub pricing_types: Vec<PricingType>,
    #[doc = "Publisher display name"]
    #[serde(rename = "publisherDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_display_name: Option<String>,
    #[doc = "Product long summary"]
    #[serde(rename = "longSummary", default, skip_serializing_if = "Option::is_none")]
    pub long_summary: Option<String>,
    #[doc = "Product summary"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[doc = "Linked add-in types"]
    #[serde(rename = "linkedAddInsTypes", default, skip_serializing_if = "Option::is_none")]
    pub linked_add_ins_types: Option<serde_json::Value>,
    #[doc = "URI to the small product icon"]
    #[serde(rename = "smallIconUri", default, skip_serializing_if = "Option::is_none")]
    pub small_icon_uri: Option<String>,
    #[doc = "The product description text"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Rating buckets, Above1, Above2, Above3, Above4"]
    #[serde(
        rename = "ratingBuckets",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub rating_buckets: Vec<RatingBucket>,
    #[doc = "Average rating for the offer"]
    #[serde(rename = "ratingAverage", default, skip_serializing_if = "Option::is_none")]
    pub rating_average: Option<f64>,
    #[doc = "Total number of ratings for the offer"]
    #[serde(rename = "ratingCount", default, skip_serializing_if = "Option::is_none")]
    pub rating_count: Option<i32>,
    #[doc = "Retail price of the product's cheapest plan"]
    #[serde(rename = "startingPrice", default, skip_serializing_if = "Option::is_none")]
    pub starting_price: Option<serde_json::Value>,
    #[doc = "List of plans that are available for this product"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub plans: Vec<PlanSummary>,
    #[doc = "Maps to the list of compatible products that the publisher provides"]
    #[serde(
        rename = "supportedProducts",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_products: Vec<String>,
    #[doc = "Product categories the offer belongs to"]
    #[serde(
        rename = "applicableProducts",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub applicable_products: Vec<String>,
    #[doc = "Latest update date of the product"]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<time::OffsetDateTime>,
}
impl ProductSummary {
    pub fn new(product_type: serde_json::Value) -> Self {
        Self {
            display_name: None,
            popularity: None,
            category_ids: Vec::new(),
            industry_ids: Vec::new(),
            publisher_id: None,
            azure_benefit: None,
            badges: Vec::new(),
            publisher_type: None,
            publishing_stage: None,
            unique_product_id: None,
            product_type,
            operating_systems: Vec::new(),
            pricing_types: Vec::new(),
            publisher_display_name: None,
            long_summary: None,
            summary: None,
            linked_add_ins_types: None,
            small_icon_uri: None,
            description: None,
            rating_buckets: Vec::new(),
            rating_average: None,
            rating_count: None,
            starting_price: None,
            plans: Vec::new(),
            supported_products: Vec::new(),
            applicable_products: Vec::new(),
            last_modified_date_time: None,
        }
    }
}
#[doc = "The delivery method or deployment model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProductType")]
pub enum ProductType {
    None,
    DevService,
    ManagedApplication,
    VirtualMachine,
    AzureApplication,
    Container,
    SaaS,
    SolutionTemplate,
    IotEdgeModules,
    ManagedServices,
    ContainerApps,
    VisualStudioExtension,
    DynamicsOps,
    #[serde(rename = "DynamicsCE")]
    DynamicsCe,
    #[serde(rename = "DynamicsBC")]
    DynamicsBc,
    #[serde(rename = "PowerBI")]
    PowerBi,
    ConsultingServices,
    CosellOnly,
    CoreVirtualMachine,
    #[serde(rename = "PowerBIVisuals")]
    PowerBiVisuals,
    Office365,
    #[serde(rename = "AADApps")]
    AadApps,
    AzureServices,
    AppService,
    LogAnalytics,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProductType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProductType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProductType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ProductType", 0u32, "None"),
            Self::DevService => serializer.serialize_unit_variant("ProductType", 1u32, "DevService"),
            Self::ManagedApplication => serializer.serialize_unit_variant("ProductType", 2u32, "ManagedApplication"),
            Self::VirtualMachine => serializer.serialize_unit_variant("ProductType", 3u32, "VirtualMachine"),
            Self::AzureApplication => serializer.serialize_unit_variant("ProductType", 4u32, "AzureApplication"),
            Self::Container => serializer.serialize_unit_variant("ProductType", 5u32, "Container"),
            Self::SaaS => serializer.serialize_unit_variant("ProductType", 6u32, "SaaS"),
            Self::SolutionTemplate => serializer.serialize_unit_variant("ProductType", 7u32, "SolutionTemplate"),
            Self::IotEdgeModules => serializer.serialize_unit_variant("ProductType", 8u32, "IotEdgeModules"),
            Self::ManagedServices => serializer.serialize_unit_variant("ProductType", 9u32, "ManagedServices"),
            Self::ContainerApps => serializer.serialize_unit_variant("ProductType", 10u32, "ContainerApps"),
            Self::VisualStudioExtension => serializer.serialize_unit_variant("ProductType", 11u32, "VisualStudioExtension"),
            Self::DynamicsOps => serializer.serialize_unit_variant("ProductType", 12u32, "DynamicsOps"),
            Self::DynamicsCe => serializer.serialize_unit_variant("ProductType", 13u32, "DynamicsCE"),
            Self::DynamicsBc => serializer.serialize_unit_variant("ProductType", 14u32, "DynamicsBC"),
            Self::PowerBi => serializer.serialize_unit_variant("ProductType", 15u32, "PowerBI"),
            Self::ConsultingServices => serializer.serialize_unit_variant("ProductType", 16u32, "ConsultingServices"),
            Self::CosellOnly => serializer.serialize_unit_variant("ProductType", 17u32, "CosellOnly"),
            Self::CoreVirtualMachine => serializer.serialize_unit_variant("ProductType", 18u32, "CoreVirtualMachine"),
            Self::PowerBiVisuals => serializer.serialize_unit_variant("ProductType", 19u32, "PowerBIVisuals"),
            Self::Office365 => serializer.serialize_unit_variant("ProductType", 20u32, "Office365"),
            Self::AadApps => serializer.serialize_unit_variant("ProductType", 21u32, "AADApps"),
            Self::AzureServices => serializer.serialize_unit_variant("ProductType", 22u32, "AzureServices"),
            Self::AppService => serializer.serialize_unit_variant("ProductType", 23u32, "AppService"),
            Self::LogAnalytics => serializer.serialize_unit_variant("ProductType", 24u32, "LogAnalytics"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Product Video"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductVideo {
    #[doc = "Caption"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[doc = "Video uri"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "Video purpose"]
    #[serde(rename = "videoPurpose", default, skip_serializing_if = "Option::is_none")]
    pub video_purpose: Option<String>,
    #[doc = "Preview image"]
    #[serde(rename = "previewImage", default, skip_serializing_if = "Option::is_none")]
    pub preview_image: Option<serde_json::Value>,
}
impl ProductVideo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Proration policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProrationPolicy {
    #[doc = "Minimum prorated units"]
    #[serde(rename = "minimumProratedUnits", default, skip_serializing_if = "Option::is_none")]
    pub minimum_prorated_units: Option<String>,
}
impl ProrationPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A value indicating whether this is a Microsoft or third party product"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PublisherType")]
pub enum PublisherType {
    Microsoft,
    ThirdParty,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PublisherType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PublisherType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PublisherType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Microsoft => serializer.serialize_unit_variant("PublisherType", 0u32, "Microsoft"),
            Self::ThirdParty => serializer.serialize_unit_variant("PublisherType", 1u32, "ThirdParty"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The product publishing stage. Possible values are: Preview, Public"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PublishingStage")]
pub enum PublishingStage {
    Preview,
    Public,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PublishingStage {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PublishingStage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PublishingStage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Preview => serializer.serialize_unit_variant("PublishingStage", 0u32, "Preview"),
            Self::Public => serializer.serialize_unit_variant("PublishingStage", 1u32, "Public"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The percentage discount for 3rd party virtual machines software reservations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PurchaseDurationDiscount {
    #[doc = "The duration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "The discount percentage"]
    #[serde(rename = "discountPercentage")]
    pub discount_percentage: f64,
}
impl PurchaseDurationDiscount {
    pub fn new(discount_percentage: f64) -> Self {
        Self {
            duration: None,
            discount_percentage,
        }
    }
}
#[doc = "Rating Bucket"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RatingBucket")]
pub enum RatingBucket {
    AboveOne,
    AboveTwo,
    AboveThree,
    AboveFour,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RatingBucket {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RatingBucket {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RatingBucket {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AboveOne => serializer.serialize_unit_variant("RatingBucket", 0u32, "AboveOne"),
            Self::AboveTwo => serializer.serialize_unit_variant("RatingBucket", 1u32, "AboveTwo"),
            Self::AboveThree => serializer.serialize_unit_variant("RatingBucket", 2u32, "AboveThree"),
            Self::AboveFour => serializer.serialize_unit_variant("RatingBucket", 3u32, "AboveFour"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The sku"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RelatedSku {
    #[doc = "The name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The generation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,
    #[doc = "The identity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
}
impl RelatedSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The search field name"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SearchFieldName")]
pub enum SearchFieldName {
    All,
    LastModifiedDateTime,
    Market,
    SupportedProducts,
    HideKeys,
    PublisherId,
    DisplayName,
    AzureBenefit,
    Badges,
    SmallIconUri,
    IndustryCloud,
    PublisherType,
    PublishingState,
    Language,
    UniqueProductId,
    ProductType,
    Plans,
    OperatingSystems,
    PricingTypes,
    PublisherDisplayName,
    Summary,
    LongSummary,
    VmImageGenerations,
    VmSecurityTypes,
    VmArchitectureTypes,
    Description,
    RatingBuckets,
    RatingAverage,
    RatingCount,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SearchFieldName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SearchFieldName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SearchFieldName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::All => serializer.serialize_unit_variant("SearchFieldName", 0u32, "All"),
            Self::LastModifiedDateTime => serializer.serialize_unit_variant("SearchFieldName", 1u32, "LastModifiedDateTime"),
            Self::Market => serializer.serialize_unit_variant("SearchFieldName", 2u32, "Market"),
            Self::SupportedProducts => serializer.serialize_unit_variant("SearchFieldName", 3u32, "SupportedProducts"),
            Self::HideKeys => serializer.serialize_unit_variant("SearchFieldName", 4u32, "HideKeys"),
            Self::PublisherId => serializer.serialize_unit_variant("SearchFieldName", 5u32, "PublisherId"),
            Self::DisplayName => serializer.serialize_unit_variant("SearchFieldName", 6u32, "DisplayName"),
            Self::AzureBenefit => serializer.serialize_unit_variant("SearchFieldName", 7u32, "AzureBenefit"),
            Self::Badges => serializer.serialize_unit_variant("SearchFieldName", 8u32, "Badges"),
            Self::SmallIconUri => serializer.serialize_unit_variant("SearchFieldName", 9u32, "SmallIconUri"),
            Self::IndustryCloud => serializer.serialize_unit_variant("SearchFieldName", 10u32, "IndustryCloud"),
            Self::PublisherType => serializer.serialize_unit_variant("SearchFieldName", 11u32, "PublisherType"),
            Self::PublishingState => serializer.serialize_unit_variant("SearchFieldName", 12u32, "PublishingState"),
            Self::Language => serializer.serialize_unit_variant("SearchFieldName", 13u32, "Language"),
            Self::UniqueProductId => serializer.serialize_unit_variant("SearchFieldName", 14u32, "UniqueProductId"),
            Self::ProductType => serializer.serialize_unit_variant("SearchFieldName", 15u32, "ProductType"),
            Self::Plans => serializer.serialize_unit_variant("SearchFieldName", 16u32, "Plans"),
            Self::OperatingSystems => serializer.serialize_unit_variant("SearchFieldName", 17u32, "OperatingSystems"),
            Self::PricingTypes => serializer.serialize_unit_variant("SearchFieldName", 18u32, "PricingTypes"),
            Self::PublisherDisplayName => serializer.serialize_unit_variant("SearchFieldName", 19u32, "PublisherDisplayName"),
            Self::Summary => serializer.serialize_unit_variant("SearchFieldName", 20u32, "Summary"),
            Self::LongSummary => serializer.serialize_unit_variant("SearchFieldName", 21u32, "LongSummary"),
            Self::VmImageGenerations => serializer.serialize_unit_variant("SearchFieldName", 22u32, "VmImageGenerations"),
            Self::VmSecurityTypes => serializer.serialize_unit_variant("SearchFieldName", 23u32, "VmSecurityTypes"),
            Self::VmArchitectureTypes => serializer.serialize_unit_variant("SearchFieldName", 24u32, "VmArchitectureTypes"),
            Self::Description => serializer.serialize_unit_variant("SearchFieldName", 25u32, "Description"),
            Self::RatingBuckets => serializer.serialize_unit_variant("SearchFieldName", 26u32, "RatingBuckets"),
            Self::RatingAverage => serializer.serialize_unit_variant("SearchFieldName", 27u32, "RatingAverage"),
            Self::RatingCount => serializer.serialize_unit_variant("SearchFieldName", 28u32, "RatingCount"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Search response object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchResponse {
    #[doc = "The search facets"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facets: Option<serde_json::Value>,
    #[doc = "The results"]
    pub results: Vec<ProductSummary>,
    #[doc = "The total count"]
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "The showing results for"]
    #[serde(rename = "showingResultsFor", default, skip_serializing_if = "Option::is_none")]
    pub showing_results_for: Option<String>,
    #[doc = "URL to get the next page of API search"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SearchResponse {
    pub fn new(results: Vec<ProductSummary>) -> Self {
        Self {
            facets: None,
            results,
            total_count: None,
            showing_results_for: None,
            next_link: None,
        }
    }
}
#[doc = "Represents a model for stop sell related information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StopSellInfo {
    #[doc = "A value indicating when the sale of this item is going to be stopped"]
    #[serde(rename = "startDate", default, with = "azure_core::date::rfc3339::option")]
    pub start_date: Option<time::OffsetDateTime>,
    #[doc = "A value indicating the reason for stopping to sell this item. Possible values are: EndOfSupport, SecurityIssue, Other"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<serde_json::Value>,
    #[doc = "A value indicating an ID of an alternative offer provided by the same publisher"]
    #[serde(rename = "alternativeOfferId", default, skip_serializing_if = "Option::is_none")]
    pub alternative_offer_id: Option<String>,
    #[doc = "A value indicating an ID of an alternative plan of the same offer"]
    #[serde(rename = "alternativePlanId", default, skip_serializing_if = "Option::is_none")]
    pub alternative_plan_id: Option<String>,
}
impl StopSellInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Reason for stopping to sell an item"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "StopSellReason")]
pub enum StopSellReason {
    EndOfSupport,
    SecurityIssue,
    Other,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for StopSellReason {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for StopSellReason {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for StopSellReason {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::EndOfSupport => serializer.serialize_unit_variant("StopSellReason", 0u32, "EndOfSupport"),
            Self::SecurityIssue => serializer.serialize_unit_variant("StopSellReason", 1u32, "SecurityIssue"),
            Self::Other => serializer.serialize_unit_variant("StopSellReason", 2u32, "Other"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The type of the suggestion"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SuggestionType")]
pub enum SuggestionType {
    WordSearch,
    Entity,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SuggestionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SuggestionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SuggestionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::WordSearch => serializer.serialize_unit_variant("SuggestionType", 0u32, "WordSearch"),
            Self::Entity => serializer.serialize_unit_variant("SuggestionType", 1u32, "Entity"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The suggestions field name"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SuggestionsFieldName")]
pub enum SuggestionsFieldName {
    DisplayText,
    Id,
    IconUrl,
    ProductType,
    LinkedAddInsTypes,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SuggestionsFieldName {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SuggestionsFieldName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SuggestionsFieldName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::DisplayText => serializer.serialize_unit_variant("SuggestionsFieldName", 0u32, "DisplayText"),
            Self::Id => serializer.serialize_unit_variant("SuggestionsFieldName", 1u32, "Id"),
            Self::IconUrl => serializer.serialize_unit_variant("SuggestionsFieldName", 2u32, "IconUrl"),
            Self::ProductType => serializer.serialize_unit_variant("SuggestionsFieldName", 3u32, "ProductType"),
            Self::LinkedAddInsTypes => serializer.serialize_unit_variant("SuggestionsFieldName", 4u32, "LinkedAddInsTypes"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The suggestion item"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuggestionsItem {
    #[doc = "The suggestion type"]
    #[serde(rename = "suggestionType", default, skip_serializing_if = "Option::is_none")]
    pub suggestion_type: Option<serde_json::Value>,
    #[doc = "The suggestion display text"]
    #[serde(rename = "displayText")]
    pub display_text: String,
    #[doc = "The suggestion item id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The suggestion icon url"]
    #[serde(rename = "iconUrl", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[doc = "The suggestion supported product types"]
    #[serde(rename = "productType", default, skip_serializing_if = "Option::is_none")]
    pub product_type: Option<serde_json::Value>,
    #[doc = "The suggestion linked add-in types"]
    #[serde(rename = "linkedAddInsTypes", default, skip_serializing_if = "Option::is_none")]
    pub linked_add_ins_types: Option<serde_json::Value>,
}
impl SuggestionsItem {
    pub fn new(display_text: String) -> Self {
        Self {
            suggestion_type: None,
            display_text,
            id: None,
            icon_url: None,
            product_type: None,
            linked_add_ins_types: None,
        }
    }
}
#[doc = "The suggestion response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuggestionsResponse {
    #[doc = "The suggestion items results"]
    pub value: Vec<SuggestionsItem>,
    #[doc = "The suggestion count"]
    #[serde(rename = "totalCount")]
    pub total_count: i64,
}
impl SuggestionsResponse {
    pub fn new(value: Vec<SuggestionsItem>, total_count: i64) -> Self {
        Self { value, total_count }
    }
}
#[doc = "Applicable term"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Term {
    #[doc = "Term description parameters"]
    #[serde(
        rename = "termDescriptionParameters",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub term_description_parameters: Vec<TermDescriptionParameter>,
    #[doc = "Term id"]
    #[serde(rename = "termId", default, skip_serializing_if = "Option::is_none")]
    pub term_id: Option<String>,
    #[doc = "Term unit"]
    #[serde(rename = "termUnit", default, skip_serializing_if = "Option::is_none")]
    pub term_unit: Option<String>,
    #[doc = "Proration policy"]
    #[serde(rename = "prorationPolicy", default, skip_serializing_if = "Option::is_none")]
    pub proration_policy: Option<serde_json::Value>,
    #[doc = "Term description"]
    #[serde(rename = "termDescription", default, skip_serializing_if = "Option::is_none")]
    pub term_description: Option<String>,
    #[doc = "Price for the term"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<serde_json::Value>,
    #[doc = "Renew term id"]
    #[serde(rename = "renewTermId", default, skip_serializing_if = "Option::is_none")]
    pub renew_term_id: Option<String>,
    #[doc = "Renew term units"]
    #[serde(rename = "renewTermUnits", default, skip_serializing_if = "Option::is_none")]
    pub renew_term_units: Option<String>,
    #[doc = "Billing Plan"]
    #[serde(rename = "billingPlan", default, skip_serializing_if = "Option::is_none")]
    pub billing_plan: Option<serde_json::Value>,
    #[doc = "Renew Billing Plan"]
    #[serde(rename = "renewToTermBillingPlan", default, skip_serializing_if = "Option::is_none")]
    pub renew_to_term_billing_plan: Option<String>,
    #[doc = "Indicates if autorenew is enabled"]
    #[serde(rename = "isAutorenewable", default, skip_serializing_if = "Option::is_none")]
    pub is_autorenewable: Option<bool>,
}
impl Term {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Term description parameters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TermDescriptionParameter {
    #[doc = "Parameter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    #[doc = "Value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl TermDescriptionParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "VM Architecture Type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VmArchitectureType")]
pub enum VmArchitectureType {
    X64Gen1,
    X64Gen2,
    Arm64,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VmArchitectureType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VmArchitectureType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VmArchitectureType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::X64Gen1 => serializer.serialize_unit_variant("VmArchitectureType", 0u32, "X64Gen1"),
            Self::X64Gen2 => serializer.serialize_unit_variant("VmArchitectureType", 1u32, "X64Gen2"),
            Self::Arm64 => serializer.serialize_unit_variant("VmArchitectureType", 2u32, "Arm64"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "VM Security type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VmSecurityType")]
pub enum VmSecurityType {
    None,
    Trusted,
    Confidential,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VmSecurityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VmSecurityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VmSecurityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("VmSecurityType", 0u32, "None"),
            Self::Trusted => serializer.serialize_unit_variant("VmSecurityType", 1u32, "Trusted"),
            Self::Confidential => serializer.serialize_unit_variant("VmSecurityType", 2u32, "Confidential"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
