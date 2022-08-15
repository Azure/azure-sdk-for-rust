#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
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
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FacetValueDto {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl FacetValueDto {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FacetsItemDto {
    #[serde(rename = "facetName", default, skip_serializing_if = "Option::is_none")]
    pub facet_name: Option<String>,
    #[serde(rename = "facetValues", default, skip_serializing_if = "Vec::is_empty")]
    pub facet_values: Vec<FacetValueDto>,
}
impl FacetsItemDto {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FacetsResponseDto {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub facets: Vec<FacetsItemDto>,
}
impl FacetsResponseDto {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperatingSystemDto {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl OperatingSystemDto {
    pub fn new() -> Self {
        Self::default()
    }
}
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results (if there are any)."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanDto {
    #[serde(rename = "cspState", default, skip_serializing_if = "Option::is_none")]
    pub csp_state: Option<String>,
    #[serde(rename = "legacyPlanId", default, skip_serializing_if = "Option::is_none")]
    pub legacy_plan_id: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "architectureType", default, skip_serializing_if = "Option::is_none")]
    pub architecture_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PlanMetadataDto>,
    #[serde(rename = "operatingSystem", default, skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<OperatingSystemDto>,
}
impl PlanDto {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanMetadataDto {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,
}
impl PlanMetadataDto {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
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
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateStoreAllow")]
pub enum PrivateStoreAllow {
    AllowedByAdmin,
    AllowedByDefault,
    NonAllowed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateStoreAllow {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateStoreAllow {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateStoreAllow {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AllowedByAdmin => serializer.serialize_unit_variant("PrivateStoreAllow", 0u32, "AllowedByAdmin"),
            Self::AllowedByDefault => serializer.serialize_unit_variant("PrivateStoreAllow", 1u32, "AllowedByDefault"),
            Self::NonAllowed => serializer.serialize_unit_variant("PrivateStoreAllow", 2u32, "NonAllowed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchItemDto {
    #[serde(rename = "ampAppsPopularity", default, skip_serializing_if = "Option::is_none")]
    pub amp_apps_popularity: Option<f64>,
    #[serde(rename = "ampCsPopularity", default, skip_serializing_if = "Option::is_none")]
    pub amp_cs_popularity: Option<f64>,
    #[serde(rename = "applicableProducts", default, skip_serializing_if = "Vec::is_empty")]
    pub applicable_products: Vec<String>,
    #[serde(rename = "appSourceAppsPopularity", default, skip_serializing_if = "Option::is_none")]
    pub app_source_apps_popularity: Option<f64>,
    #[serde(rename = "appSourceCategories", default, skip_serializing_if = "Vec::is_empty")]
    pub app_source_categories: Vec<String>,
    #[serde(rename = "appSourceCsPopularity", default, skip_serializing_if = "Option::is_none")]
    pub app_source_cs_popularity: Option<f64>,
    #[serde(rename = "appSourceIndustries", default, skip_serializing_if = "Vec::is_empty")]
    pub app_source_industries: Vec<String>,
    #[serde(rename = "azureCategories", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_categories: Vec<String>,
    #[serde(rename = "azurePortalCategories", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_portal_categories: Vec<String>,
    #[serde(rename = "azureIndustries", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_industries: Vec<String>,
    #[serde(rename = "bigId", default, skip_serializing_if = "Option::is_none")]
    pub big_id: Option<String>,
    #[serde(rename = "cspStates", default, skip_serializing_if = "Vec::is_empty")]
    pub csp_states: Vec<String>,
    #[serde(rename = "determinedStorefronts", default, skip_serializing_if = "Vec::is_empty")]
    pub determined_storefronts: Vec<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "isAzureBenefitEligible", default, skip_serializing_if = "Option::is_none")]
    pub is_azure_benefit_eligible: Option<bool>,
    #[serde(rename = "isCoreVm", default, skip_serializing_if = "Option::is_none")]
    pub is_core_vm: Option<bool>,
    #[serde(rename = "isPreferredSolution", default, skip_serializing_if = "Option::is_none")]
    pub is_preferred_solution: Option<bool>,
    #[serde(rename = "isAdditionalPurchaseRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_additional_purchase_required: Option<bool>,
    #[serde(rename = "isPowerBICertified", default, skip_serializing_if = "Option::is_none")]
    pub is_power_bi_certified: Option<bool>,
    #[serde(rename = "isIndustryCloud", default, skip_serializing_if = "Option::is_none")]
    pub is_industry_cloud: Option<bool>,
    #[serde(rename = "isMicrosoftProduct", default, skip_serializing_if = "Option::is_none")]
    pub is_microsoft_product: Option<bool>,
    #[serde(rename = "isPreview", default, skip_serializing_if = "Option::is_none")]
    pub is_preview: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "legacyId", default, skip_serializing_if = "Option::is_none")]
    pub legacy_id: Option<String>,
    #[serde(rename = "offerType", default, skip_serializing_if = "Option::is_none")]
    pub offer_type: Option<String>,
    #[serde(rename = "operatingSystems", default, skip_serializing_if = "Vec::is_empty")]
    pub operating_systems: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub plans: Vec<PlanDto>,
    #[serde(rename = "pricingTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub pricing_types: Vec<String>,
    #[serde(rename = "publisherDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "vmGenerations", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_generations: Vec<String>,
    #[serde(rename = "vmImageTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_image_types: Vec<String>,
    #[serde(rename = "vmSecurityTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub vm_security_types: Vec<String>,
    #[serde(rename = "smallIconUri", default, skip_serializing_if = "Option::is_none")]
    pub small_icon_uri: Option<String>,
    #[serde(rename = "mediumIconUri", default, skip_serializing_if = "Option::is_none")]
    pub medium_icon_uri: Option<String>,
    #[serde(rename = "largeIconUri", default, skip_serializing_if = "Option::is_none")]
    pub large_icon_uri: Option<String>,
    #[serde(rename = "wideIconUri", default, skip_serializing_if = "Option::is_none")]
    pub wide_icon_uri: Option<String>,
    #[serde(rename = "heroIconUri", default, skip_serializing_if = "Option::is_none")]
    pub hero_icon_uri: Option<String>,
}
impl SearchItemDto {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchRequest {
    #[serde(rename = "searchQuery", default, skip_serializing_if = "Option::is_none")]
    pub search_query: Option<String>,
    #[serde(rename = "publisherName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub take: Option<i32>,
    #[serde(rename = "isAzureBenefitEligible", default, skip_serializing_if = "Option::is_none")]
    pub is_azure_benefit_eligible: Option<bool>,
    #[serde(rename = "isMicrosoftProduct", default, skip_serializing_if = "Option::is_none")]
    pub is_microsoft_product: Option<bool>,
    #[serde(rename = "isThirdParty", default, skip_serializing_if = "Option::is_none")]
    pub is_third_party: Option<bool>,
    #[serde(rename = "isCoreVm", default, skip_serializing_if = "Option::is_none")]
    pub is_core_vm: Option<bool>,
    #[serde(rename = "isPreferredSolution", default, skip_serializing_if = "Option::is_none")]
    pub is_preferred_solution: Option<bool>,
    #[serde(rename = "isAdditionalPurchaseRequired", default, skip_serializing_if = "Option::is_none")]
    pub is_additional_purchase_required: Option<bool>,
    #[serde(rename = "isPowerBICertified", default, skip_serializing_if = "Option::is_none")]
    pub is_power_bi_certified: Option<bool>,
    #[serde(rename = "isIndustryCloud", default, skip_serializing_if = "Option::is_none")]
    pub is_industry_cloud: Option<bool>,
    pub languages: Vec<String>,
    pub stores: Vec<Store>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub select: Vec<String>,
    #[serde(rename = "sortBy", default, skip_serializing_if = "Vec::is_empty")]
    pub sort_by: Vec<String>,
    #[serde(rename = "offerTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub offer_types: Vec<String>,
    #[serde(rename = "pricingTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub pricing_types: Vec<PricingType>,
    #[serde(rename = "privateStoreAllowances", default, skip_serializing_if = "Vec::is_empty")]
    pub private_store_allowances: Vec<PrivateStoreAllow>,
    #[serde(rename = "appSourceIndustries", default, skip_serializing_if = "Vec::is_empty")]
    pub app_source_industries: Vec<String>,
    #[serde(rename = "operatingSystems", default, skip_serializing_if = "Vec::is_empty")]
    pub operating_systems: Vec<String>,
    #[serde(rename = "appSourceCategories", default, skip_serializing_if = "Vec::is_empty")]
    pub app_source_categories: Vec<String>,
    #[serde(rename = "azureIndustries", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_industries: Vec<String>,
    #[serde(rename = "azureCategories", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_categories: Vec<String>,
    #[serde(rename = "azurePortalCategories", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_portal_categories: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub facets: Vec<String>,
}
impl SearchRequest {
    pub fn new(languages: Vec<String>, stores: Vec<Store>) -> Self {
        Self {
            search_query: None,
            publisher_name: None,
            skip: None,
            take: None,
            is_azure_benefit_eligible: None,
            is_microsoft_product: None,
            is_third_party: None,
            is_core_vm: None,
            is_preferred_solution: None,
            is_additional_purchase_required: None,
            is_power_bi_certified: None,
            is_industry_cloud: None,
            languages,
            stores,
            select: Vec::new(),
            sort_by: Vec::new(),
            offer_types: Vec::new(),
            pricing_types: Vec::new(),
            private_store_allowances: Vec::new(),
            app_source_industries: Vec::new(),
            operating_systems: Vec::new(),
            app_source_categories: Vec::new(),
            azure_industries: Vec::new(),
            azure_categories: Vec::new(),
            azure_portal_categories: Vec::new(),
            facets: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SearchResponseDto {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub facets: Vec<FacetsItemDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub results: Vec<SearchItemDto>,
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}
impl SearchResponseDto {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Store")]
pub enum Store {
    Appsource,
    #[serde(rename = "AMP")]
    Amp,
    Ibiza,
    Cosell,
    DakotaDownstream,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Store {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Store {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Store {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Appsource => serializer.serialize_unit_variant("Store", 0u32, "Appsource"),
            Self::Amp => serializer.serialize_unit_variant("Store", 1u32, "AMP"),
            Self::Ibiza => serializer.serialize_unit_variant("Store", 2u32, "Ibiza"),
            Self::Cosell => serializer.serialize_unit_variant("Store", 3u32, "Cosell"),
            Self::DakotaDownstream => serializer.serialize_unit_variant("Store", 4u32, "DakotaDownstream"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
