#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Plan level resources and configuration files"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Artifact {
    #[doc = "Artifact name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Artifact uri"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "Artifact type"]
    #[serde(rename = "artifactType", default, skip_serializing_if = "Option::is_none")]
    pub artifact_type: Option<serde_json::Value>,
}
impl Artifact {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Eligibility for Azure consumption commitment benefit (https://docs.microsoft.com/en-us/marketplace/azure-consumption-commitment-benefit), Possible values: Eligible, NotEligible, if no value selected, this filter is ignored. Default: null"]
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
#[doc = "The badge"]
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
#[doc = "PA value showing whether the product is available for purchase through CSP channel, Possible values, OptIn, OptOut, SelectiveOptIn, for more information see, https://docs.microsoft.com/en-us/azure/marketplace/cloud-solution-providers"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CspState")]
pub enum CspState {
    OptIn,
    OptOut,
    Terminated,
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
            Self::Terminated => serializer.serialize_unit_variant("CspState", 2u32, "Terminated"),
            Self::SelectiveOptIn => serializer.serialize_unit_variant("CspState", 3u32, "SelectiveOptIn"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The search error response object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The search error response details object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponseDetails>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The search error response details object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseDetails {
    #[doc = "The error code description. Such as code=InternalError"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message. Such as message=Internal server error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorResponseDetails {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Marketplace registered storefronts"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Gallery")]
pub enum Gallery {
    AppsourceApps,
    AppsourceConsultingServices,
    #[serde(rename = "AMPApps")]
    AmpApps,
    #[serde(rename = "AMPConsultingServices")]
    AmpConsultingServices,
    Azure,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Gallery {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Gallery {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Gallery {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AppsourceApps => serializer.serialize_unit_variant("Gallery", 0u32, "AppsourceApps"),
            Self::AppsourceConsultingServices => serializer.serialize_unit_variant("Gallery", 1u32, "AppsourceConsultingServices"),
            Self::AmpApps => serializer.serialize_unit_variant("Gallery", 2u32, "AMPApps"),
            Self::AmpConsultingServices => serializer.serialize_unit_variant("Gallery", 3u32, "AMPConsultingServices"),
            Self::Azure => serializer.serialize_unit_variant("Gallery", 4u32, "Azure"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Define the search for IndustryCloud, see https://docs.microsoft.com/en-us/industry/"]
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
#[doc = "Lead generation information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LeadGeneration {
    #[doc = "The productId"]
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}
impl LeadGeneration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Links to publisher external references documents"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Link {
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
impl Link {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The product starting price"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketStartPrice {
    #[doc = "The market for which the starting price is calculated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[doc = "The terms pricing model units."]
    #[serde(rename = "termUnits", default, skip_serializing_if = "Option::is_none")]
    pub term_units: Option<String>,
    #[doc = "The meters pricing model units."]
    #[serde(rename = "meterUnits", default, skip_serializing_if = "Option::is_none")]
    pub meter_units: Option<String>,
    #[doc = "Starting (minimal) terms price."]
    #[serde(rename = "minTermPrice", default, skip_serializing_if = "Option::is_none")]
    pub min_term_price: Option<f64>,
    #[doc = "Starting (minimal) meters price."]
    #[serde(rename = "minMeterPrice", default, skip_serializing_if = "Option::is_none")]
    pub min_meter_price: Option<f64>,
    #[doc = "Currency for price."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}
impl MarketStartPrice {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Additional metadata"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanMetadata {
    #[doc = "The VM image generation, see https://docs.microsoft.com/en-us/azure/virtual-machines/generation-2"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,
    #[doc = "Pointing to a planId which holds the alternative stack reference"]
    #[serde(rename = "altStackReference", default, skip_serializing_if = "Option::is_none")]
    pub alt_stack_reference: Option<String>,
    #[doc = "Pointing to a planId that is related to this plan"]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanSummary {
    #[doc = "Plan id"]
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[doc = "A value indicating whether the product is available for purchase through CSP channel"]
    #[serde(rename = "cspState", default, skip_serializing_if = "Option::is_none")]
    pub csp_state: Option<serde_json::Value>,
    #[doc = "Unique plan Id which is obtained by combining uniqueProductId1 and PlanId with no separator in between"]
    #[serde(rename = "uniquePlanId", default, skip_serializing_if = "Option::is_none")]
    pub unique_plan_id: Option<String>,
    #[doc = "Display name of the plan"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The VM image architecture type, x64, ARM"]
    #[serde(rename = "vmArchitectureType", default, skip_serializing_if = "Option::is_none")]
    pub vm_architecture_type: Option<String>,
    #[doc = "Additional metadata"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "What Azure portal view to open when someone wants to create a marketplace item"]
    #[serde(rename = "uiDefinitionUri", default, skip_serializing_if = "Option::is_none")]
    pub ui_definition_uri: Option<String>,
    #[doc = "The list of the pricing types for which this plan is designated"]
    #[serde(
        rename = "pricingTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub pricing_types: Vec<PricingType>,
    #[doc = "A value indicating whether the plan is quantifiable"]
    #[serde(rename = "isQuantifiable", default, skip_serializing_if = "Option::is_none")]
    pub is_quantifiable: Option<bool>,
    #[doc = "VM security types"]
    #[serde(
        rename = "vmSecurityTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vm_security_types: Vec<VmSecurityType>,
    #[doc = "Plan level resources and configuration files"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub artifacts: Vec<Artifact>,
    #[doc = "The summary"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[doc = "The description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The sku id"]
    #[serde(rename = "skuId", default, skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[doc = "The delivery method, deployment model"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ProductType>,
    #[doc = "The display rank"]
    #[serde(rename = "displayRank", default, skip_serializing_if = "Option::is_none")]
    pub display_rank: Option<String>,
    #[doc = "The percentage discount for third party virtual machines software reservations"]
    #[serde(
        rename = "purchaseDurationDiscounts",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub purchase_duration_discounts: Vec<PurchaseDurationDiscount>,
    #[doc = "Lead generation information"]
    #[serde(rename = "leadGeneration", default, skip_serializing_if = "Option::is_none")]
    pub lead_generation: Option<serde_json::Value>,
    #[doc = "Indication that the plan is accessible to restricted audience only"]
    #[serde(rename = "isPrivate", default, skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
}
impl PlanSummary {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProductSummary {
    #[doc = "Display name"]
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
    #[doc = "Publisher id"]
    #[serde(rename = "publisherId", default, skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
    #[doc = "Eligibility for Azure consumption commitment benefit (https://docs.microsoft.com/en-us/marketplace/azure-consumption-commitment-benefit), Possible values: Eligible, NotEligible, if no value selected, this filter is ignored. Default: null"]
    #[serde(rename = "azureBenefit", default, skip_serializing_if = "Option::is_none")]
    pub azure_benefit: Option<serde_json::Value>,
    #[doc = "The following product badges are available: preferredSolution, powerBICertified, AdditionalPurchaseRequirement"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub badges: Vec<Badge>,
    #[doc = "Microsoft or third-party publisher"]
    #[serde(rename = "publisherType", default, skip_serializing_if = "Option::is_none")]
    pub publisher_type: Option<serde_json::Value>,
    #[doc = "Publishing Stage, can be Live or Preview"]
    #[serde(rename = "publishingStage", default, skip_serializing_if = "Option::is_none")]
    pub publishing_stage: Option<serde_json::Value>,
    #[doc = "Product unique identifier"]
    #[serde(rename = "uniqueProductId", default, skip_serializing_if = "Option::is_none")]
    pub unique_product_id: Option<String>,
    #[doc = "The delivery method, deployment model"]
    #[serde(rename = "productType", default, skip_serializing_if = "Option::is_none")]
    pub product_type: Option<serde_json::Value>,
    #[doc = "Array of operating systems to search by, if none selected then filter is ignored, this is relevant for Virtual Machine product type only. Such as operatingSystems=windows,linux"]
    #[serde(
        rename = "operatingSystems",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operating_systems: Vec<String>,
    #[doc = "The list of the pricing types for which this offer is designated"]
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
    #[doc = "Product Summary"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[doc = "The Privacy Policy Uri"]
    #[serde(rename = "privacyPolicyUri", default, skip_serializing_if = "Option::is_none")]
    pub privacy_policy_uri: Option<String>,
    #[doc = "The product version"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The CSP legal terms URI"]
    #[serde(rename = "cspLegalTermsUri", default, skip_serializing_if = "Option::is_none")]
    pub csp_legal_terms_uri: Option<String>,
    #[doc = "The Screenshots"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub screenshots: Vec<String>,
    #[doc = "Array of vm generations to search by, see Azure support for generation 2 VMs - Azure Virtual Machines | Microsoft Docs, Possible values, 1,2. Such as '1'"]
    #[serde(
        rename = "vmImageGenerations",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vm_image_generations: Vec<String>,
    #[doc = "Array of Virtual Machine image architecture types to search by, 1-x64, 2-ARM. Such as '1'"]
    #[serde(
        rename = "vmArchitectureTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vm_architecture_types: Vec<String>,
    #[doc = "Vm security types, Possible values, Trusted, Confidential, None."]
    #[serde(
        rename = "vmSecurityTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub vm_security_types: Vec<VmSecurityType>,
    #[doc = "URI to the small product icon "]
    #[serde(rename = "smallIconUri", default, skip_serializing_if = "Option::is_none")]
    pub small_icon_uri: Option<String>,
    #[doc = "URI to the medium product icon"]
    #[serde(rename = "mediumIconUri", default, skip_serializing_if = "Option::is_none")]
    pub medium_icon_uri: Option<String>,
    #[doc = "URI to the large product icon"]
    #[serde(rename = "largeIconUri", default, skip_serializing_if = "Option::is_none")]
    pub large_icon_uri: Option<String>,
    #[doc = "URI to the wide product icon "]
    #[serde(rename = "wideIconUri", default, skip_serializing_if = "Option::is_none")]
    pub wide_icon_uri: Option<String>,
    #[doc = "The product description text"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Rating buckets, Above1, Above2, Above3, Above4. Above5"]
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
    #[doc = "The product starting price"]
    #[serde(rename = "startingPrice", default, skip_serializing_if = "Option::is_none")]
    pub starting_price: Option<MarketStartPrice>,
    #[doc = "List of linked Add Ins supported by the product"]
    #[serde(
        rename = "linkedAddIns",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub linked_add_ins: Vec<String>,
    #[doc = "List of plans that are available for this product"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub plans: Vec<PlanSummary>,
    #[doc = "The legal terms uri"]
    #[serde(rename = "legalTermsUri", default, skip_serializing_if = "Option::is_none")]
    pub legal_terms_uri: Option<String>,
    #[doc = "The legal terms type"]
    #[serde(rename = "legalTermsType", default, skip_serializing_if = "Option::is_none")]
    pub legal_terms_type: Option<String>,
    #[doc = "The publisher support URL"]
    #[serde(rename = "supportUri", default, skip_serializing_if = "Option::is_none")]
    pub support_uri: Option<String>,
    #[doc = "Links provided by the publisher"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub links: Vec<Link>,
}
impl ProductSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The delivery method, deployment model"]
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
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The publisher type"]
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
#[doc = "The product publishing stage"]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PurchaseDurationDiscount {
    #[doc = "The duration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "The discountPercentage"]
    #[serde(rename = "discountPercentage", default, skip_serializing_if = "Option::is_none")]
    pub discount_percentage: Option<f64>,
}
impl PurchaseDurationDiscount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure portal rating bucket to search by. Such as 'Above1'"]
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
#[doc = "Pointing to a planId that is related to this plan"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RelatedSku {
    #[doc = "The Name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The Generation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,
    #[doc = "The Identity"]
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
    Popularity,
    ApplicableProducts,
    CategoryIds,
    Market,
    LinkedAddIns,
    SupportedProducts,
    HideKeys,
    PublisherId,
    CspStates,
    DisplayName,
    AzureBenefit,
    Badges,
    SmallIconUri,
    MediumIconUri,
    LargeIconUri,
    WideIconUri,
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
    VmImageGenerations,
    VmSecurityTypes,
    VmArchitectureTypes,
    Description,
    RatingBuckets,
    RatingAverage,
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
            Self::Popularity => serializer.serialize_unit_variant("SearchFieldName", 1u32, "Popularity"),
            Self::ApplicableProducts => serializer.serialize_unit_variant("SearchFieldName", 2u32, "ApplicableProducts"),
            Self::CategoryIds => serializer.serialize_unit_variant("SearchFieldName", 3u32, "CategoryIds"),
            Self::Market => serializer.serialize_unit_variant("SearchFieldName", 4u32, "Market"),
            Self::LinkedAddIns => serializer.serialize_unit_variant("SearchFieldName", 5u32, "LinkedAddIns"),
            Self::SupportedProducts => serializer.serialize_unit_variant("SearchFieldName", 6u32, "SupportedProducts"),
            Self::HideKeys => serializer.serialize_unit_variant("SearchFieldName", 7u32, "HideKeys"),
            Self::PublisherId => serializer.serialize_unit_variant("SearchFieldName", 8u32, "PublisherId"),
            Self::CspStates => serializer.serialize_unit_variant("SearchFieldName", 9u32, "CspStates"),
            Self::DisplayName => serializer.serialize_unit_variant("SearchFieldName", 10u32, "DisplayName"),
            Self::AzureBenefit => serializer.serialize_unit_variant("SearchFieldName", 11u32, "AzureBenefit"),
            Self::Badges => serializer.serialize_unit_variant("SearchFieldName", 12u32, "Badges"),
            Self::SmallIconUri => serializer.serialize_unit_variant("SearchFieldName", 13u32, "SmallIconUri"),
            Self::MediumIconUri => serializer.serialize_unit_variant("SearchFieldName", 14u32, "MediumIconUri"),
            Self::LargeIconUri => serializer.serialize_unit_variant("SearchFieldName", 15u32, "LargeIconUri"),
            Self::WideIconUri => serializer.serialize_unit_variant("SearchFieldName", 16u32, "WideIconUri"),
            Self::IndustryCloud => serializer.serialize_unit_variant("SearchFieldName", 17u32, "IndustryCloud"),
            Self::PublisherType => serializer.serialize_unit_variant("SearchFieldName", 18u32, "PublisherType"),
            Self::PublishingState => serializer.serialize_unit_variant("SearchFieldName", 19u32, "PublishingState"),
            Self::Language => serializer.serialize_unit_variant("SearchFieldName", 20u32, "Language"),
            Self::UniqueProductId => serializer.serialize_unit_variant("SearchFieldName", 21u32, "UniqueProductId"),
            Self::ProductType => serializer.serialize_unit_variant("SearchFieldName", 22u32, "ProductType"),
            Self::Plans => serializer.serialize_unit_variant("SearchFieldName", 23u32, "Plans"),
            Self::OperatingSystems => serializer.serialize_unit_variant("SearchFieldName", 24u32, "OperatingSystems"),
            Self::PricingTypes => serializer.serialize_unit_variant("SearchFieldName", 25u32, "PricingTypes"),
            Self::PublisherDisplayName => serializer.serialize_unit_variant("SearchFieldName", 26u32, "PublisherDisplayName"),
            Self::Summary => serializer.serialize_unit_variant("SearchFieldName", 27u32, "Summary"),
            Self::VmImageGenerations => serializer.serialize_unit_variant("SearchFieldName", 28u32, "VmImageGenerations"),
            Self::VmSecurityTypes => serializer.serialize_unit_variant("SearchFieldName", 29u32, "VmSecurityTypes"),
            Self::VmArchitectureTypes => serializer.serialize_unit_variant("SearchFieldName", 30u32, "VmArchitectureTypes"),
            Self::Description => serializer.serialize_unit_variant("SearchFieldName", 31u32, "Description"),
            Self::RatingBuckets => serializer.serialize_unit_variant("SearchFieldName", 32u32, "RatingBuckets"),
            Self::RatingAverage => serializer.serialize_unit_variant("SearchFieldName", 33u32, "RatingAverage"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Search response object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchResponse {
    #[doc = "The search facets"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facets: Option<serde_json::Value>,
    #[doc = "The results"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Vm security type, Possible values: Trusted, Confidential, None."]
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
