#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Artifact {
    #[doc = "Artifact name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Artifact uri"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "Artifact type"]
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}
impl Artifact {
    pub fn new(type_: serde_json::Value) -> Self {
        Self {
            name: None,
            uri: None,
            type_,
        }
    }
}
#[doc = "enum type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ArtifactType {
    Template,
    Fragment,
    Custom,
    Metadata,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailabilityEntity {
    #[doc = "List of allowed actions"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<String>,
    #[doc = "Market for the availability collection"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[doc = "A value indicating whether a payment instrument is required"]
    #[serde(rename = "isPIRequired")]
    pub is_pi_required: bool,
    #[doc = "App id of the availability collection"]
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[doc = "Plan Id of the availability collection"]
    #[serde(rename = "planID", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[doc = "Applicable billing meter Id"]
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[doc = "Applicable billing meter information"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meter: Option<serde_json::Value>,
    #[doc = "Pricing audience"]
    #[serde(rename = "pricingAudience")]
    pub pricing_audience: serde_json::Value,
    #[doc = "List of applicable terms"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub terms: Vec<Term>,
    #[doc = "Applicable billing meter information"]
    #[serde(rename = "piFilter", default, skip_serializing_if = "Option::is_none")]
    pub pi_filter: Option<serde_json::Value>,
    #[doc = "A value indicating whether it has been stopped from sell in a market"]
    #[serde(rename = "isStopSell")]
    pub is_stop_sell: bool,
    #[doc = "A value indicating whether it has free trials"]
    #[serde(rename = "hasFreeTrials")]
    pub has_free_trials: bool,
    #[doc = "Asset behaviors"]
    #[serde(rename = "assetBehaviors", default, skip_serializing_if = "Vec::is_empty")]
    pub asset_behaviors: Vec<String>,
    #[doc = "Consumption unit type"]
    #[serde(rename = "consumptionUnitType", default, skip_serializing_if = "Option::is_none")]
    pub consumption_unit_type: Option<String>,
    #[doc = "Display rank"]
    #[serde(rename = "displayRank")]
    pub display_rank: i32,
    #[doc = "Partition key"]
    #[serde(rename = "partitionKey", default, skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<String>,
    #[doc = "A value indicating whether remediation is required to acquire availability"]
    #[serde(rename = "remediationRequired")]
    pub remediation_required: bool,
    #[doc = "Remediation actions for acquiring availability if RemediationRequired is true"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remediations: Vec<Remediation>,
    #[doc = "Availability effectiveness start date - in epoch minutes"]
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<i32>,
    #[doc = "Availability effectiveness end date - in epoch minutes"]
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<i32>,
    #[doc = "Plan availabilities"]
    #[serde(rename = "planAvailabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub plan_availabilities: Vec<AvailabilityEntity>,
}
impl AvailabilityEntity {
    pub fn new(
        is_pi_required: bool,
        pricing_audience: serde_json::Value,
        is_stop_sell: bool,
        has_free_trials: bool,
        display_rank: i32,
        remediation_required: bool,
    ) -> Self {
        Self {
            actions: Vec::new(),
            market: None,
            is_pi_required,
            app_id: None,
            plan_id: None,
            meter_id: None,
            meter: None,
            pricing_audience,
            terms: Vec::new(),
            pi_filter: None,
            is_stop_sell,
            has_free_trials,
            asset_behaviors: Vec::new(),
            consumption_unit_type: None,
            display_rank,
            partition_key: None,
            remediation_required,
            remediations: Vec::new(),
            start_date: None,
            end_date: None,
            plan_availabilities: Vec::new(),
        }
    }
}
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
#[doc = "enum type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CspState {
    OptIn,
    OptOut,
    Terminated,
    SelectiveOptIn,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CatalogItem {
    #[doc = "Language"]
    pub language: String,
    #[doc = "Display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "A value indicating whether standard contract amendments are present for this product"]
    #[serde(rename = "hasStandardContractAmendments")]
    pub has_standard_contract_amendments: bool,
    #[doc = "The publisher's Microsoft Partner Network (MPN) Id"]
    #[serde(rename = "publisherMpnId", default, skip_serializing_if = "Option::is_none")]
    pub publisher_mpn_id: Option<String>,
    #[doc = "Partner Center Seller Id"]
    #[serde(rename = "sellerId", default, skip_serializing_if = "Option::is_none")]
    pub seller_id: Option<String>,
    #[doc = "Publisher id"]
    #[serde(rename = "publisherId", default, skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
    #[doc = "Partner center id"]
    #[serde(rename = "partnerCenterId", default, skip_serializing_if = "Option::is_none")]
    pub partner_center_id: Option<String>,
    #[doc = "Publisher display name"]
    #[serde(rename = "publisherDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_display_name: Option<String>,
    #[doc = "Offer id"]
    #[serde(rename = "offerId")]
    pub offer_id: String,
    #[doc = "Offer legacy id"]
    #[serde(rename = "legacyId")]
    pub legacy_id: String,
    #[doc = "The list of the storefronts for which this offer is designated"]
    #[serde(rename = "determinedStorefronts", default, skip_serializing_if = "Vec::is_empty")]
    pub determined_storefronts: Vec<Store>,
    #[doc = "Summary"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[doc = "Long summary"]
    #[serde(rename = "longSummary", default, skip_serializing_if = "Option::is_none")]
    pub long_summary: Option<String>,
    #[doc = "Description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Offer type"]
    #[serde(rename = "offerType")]
    pub offer_type: serde_json::Value,
    #[doc = "A value indicating whether the item is private"]
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
    #[doc = "A value indicating whether the item is preview"]
    #[serde(rename = "isPreview")]
    pub is_preview: bool,
    #[doc = "A value indicating whether sale of this item is stopped"]
    #[serde(rename = "isStopSell")]
    pub is_stop_sell: bool,
    #[doc = "A value indicating whether this item is eligible for fulfill before charge"]
    #[serde(rename = "fulfillBeforeChargeEligible")]
    pub fulfill_before_charge_eligible: bool,
    #[doc = "Marketing material added by the publisher"]
    #[serde(rename = "marketingMaterial", default, skip_serializing_if = "Option::is_none")]
    pub marketing_material: Option<serde_json::Value>,
    #[doc = "Markets for the item"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub markets: Vec<String>,
    #[doc = "ISV contact details"]
    #[serde(rename = "isvContactDetails", default, skip_serializing_if = "Option::is_none")]
    pub isv_contact_details: Option<serde_json::Value>,
    #[doc = "Big catalog id"]
    #[serde(rename = "bigId")]
    pub big_id: String,
    #[doc = "OCP solution id of the product"]
    #[serde(rename = "ocpSolutionId", default, skip_serializing_if = "Option::is_none")]
    pub ocp_solution_id: Option<String>,
    #[doc = "Legal terms URI"]
    #[serde(rename = "legalTermsUri", default, skip_serializing_if = "Option::is_none")]
    pub legal_terms_uri: Option<String>,
    #[doc = "CSP legal terms URI"]
    #[serde(rename = "cspLegalTermsUri", default, skip_serializing_if = "Option::is_none")]
    pub csp_legal_terms_uri: Option<String>,
    #[doc = "Type of to the legal terms"]
    #[serde(rename = "legalTermsType")]
    pub legal_terms_type: serde_json::Value,
    #[doc = "Uri to the privacy policy of the product"]
    #[serde(rename = "privacyPolicyUri", default, skip_serializing_if = "Option::is_none")]
    pub privacy_policy_uri: Option<String>,
    #[doc = "Help link for the product"]
    #[serde(rename = "helpLink", default, skip_serializing_if = "Option::is_none")]
    pub help_link: Option<String>,
    #[doc = "Support uri of the product"]
    #[serde(rename = "supportUri", default, skip_serializing_if = "Option::is_none")]
    pub support_uri: Option<String>,
    #[doc = "Big Cat submission version"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "User interface definition uri"]
    #[serde(rename = "uiDefinitionUri", default, skip_serializing_if = "Option::is_none")]
    pub ui_definition_uri: Option<String>,
    #[doc = "List of category IDs the marketplace item belongs to"]
    #[serde(rename = "categoryIds", default, skip_serializing_if = "Vec::is_empty")]
    pub category_ids: Vec<String>,
    #[doc = "Market code of a service offer"]
    #[serde(rename = "marketCode", default, skip_serializing_if = "Option::is_none")]
    pub market_code: Option<String>,
    #[doc = "Market states of a service offer"]
    #[serde(rename = "marketStates", default, skip_serializing_if = "Vec::is_empty")]
    pub market_states: Vec<String>,
    #[doc = "List of industry IDs the item belongs to"]
    #[serde(rename = "industryIds", default, skip_serializing_if = "Vec::is_empty")]
    pub industry_ids: Vec<String>,
    #[doc = "List of cloud industry IDs the item belongs to"]
    #[serde(rename = "cloudIndustryCategories", default, skip_serializing_if = "Vec::is_empty")]
    pub cloud_industry_categories: Vec<String>,
    #[doc = "Primary product for the offer"]
    #[serde(rename = "primaryProduct", default, skip_serializing_if = "Option::is_none")]
    pub primary_product: Option<String>,
    #[doc = "It maps to the list of products the publisher provides with which their offer works"]
    #[serde(rename = "supportedProducts", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_products: Vec<String>,
    #[doc = "Product categories the offer belongs to"]
    #[serde(rename = "applicableProducts", default, skip_serializing_if = "Vec::is_empty")]
    pub applicable_products: Vec<String>,
    #[doc = "Service type that applies to the offer"]
    #[serde(rename = "serviceType", default, skip_serializing_if = "Option::is_none")]
    pub service_type: Option<String>,
    #[doc = "Competencies that apply to the offer"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub competencies: Vec<Competency>,
    #[doc = "A value indicating whether the item has prices"]
    #[serde(rename = "hasPrices", default, skip_serializing_if = "Option::is_none")]
    pub has_prices: Option<bool>,
    #[doc = "The duration that applies to the item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<serde_json::Value>,
    #[doc = "The pricing details of each market to the item"]
    #[serde(rename = "marketPricingDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub market_pricing_details: Vec<MarketPricingDetailsItem>,
    #[doc = "The pricing that applies to the item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pricing: Option<serde_json::Value>,
    #[doc = "The states that apply to the item"]
    #[serde(rename = "solutionAreas", default, skip_serializing_if = "Vec::is_empty")]
    pub solution_areas: Vec<String>,
    #[doc = "List of screenshot image URIs provided for the item"]
    #[serde(rename = "screenshotUris", default, skip_serializing_if = "Vec::is_empty")]
    pub screenshot_uris: Vec<String>,
    #[doc = "List of Links provided for the item"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<LinkProperties>,
    #[doc = "List of filters for the item"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<Filter>,
    #[doc = "Dictionary of icon image URIs by icon type"]
    #[serde(rename = "iconFileUris", default, skip_serializing_if = "Option::is_none")]
    pub icon_file_uris: Option<serde_json::Value>,
    #[doc = "List of artifacts"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<Artifact>,
    #[doc = "Custom item metadata"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "List of Images"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<ImageGroup>,
    #[doc = "List of product videos"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub videos: Vec<ProductVideo>,
    #[doc = "Plans available for the offer details"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub plans: Vec<Plan>,
    #[doc = "Resource group name the gallery item belongs to"]
    #[serde(rename = "resourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[doc = "Definition templates"]
    #[serde(rename = "definitionTemplates", default, skip_serializing_if = "Option::is_none")]
    pub definition_templates: Option<serde_json::Value>,
    #[doc = "Properties provided for the item"]
    #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<serde_json::Value>,
    #[doc = "This fields supports setting explicit audience like subscription, tenant or user"]
    #[serde(rename = "restrictedAudience", default, skip_serializing_if = "Option::is_none")]
    pub restricted_audience: Option<serde_json::Value>,
    #[doc = "A value indicating whether the product is third party offer or not"]
    #[serde(rename = "isThirdParty")]
    pub is_third_party: bool,
    #[doc = "This value is used to merge different entities to a single item"]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "List of hide keys provided for the item"]
    #[serde(rename = "hideKeys", default, skip_serializing_if = "Vec::is_empty")]
    pub hide_keys: Vec<String>,
    #[doc = "List of keywords provided for the item"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
    #[doc = "Popularity of the product"]
    pub popularity: f64,
    #[doc = "Pricing details uri of the product"]
    #[serde(rename = "pricingDetailsUri", default, skip_serializing_if = "Option::is_none")]
    pub pricing_details_uri: Option<String>,
    #[doc = "A value indicating whether it has free trials"]
    #[serde(rename = "hasFreeTrials")]
    pub has_free_trials: bool,
    #[doc = "A value indicating whether it has licensed plans"]
    #[serde(rename = "isByol")]
    pub is_byol: bool,
    #[doc = "A value indicating whether it is MACC eligible"]
    #[serde(rename = "isMacc")]
    pub is_macc: bool,
    #[doc = "A value indicating whether it has free plans"]
    #[serde(rename = "hasFreePlans")]
    pub has_free_plans: bool,
    #[doc = "A value indicating whether product is quantifiable"]
    #[serde(rename = "isQuantifiable")]
    pub is_quantifiable: bool,
    #[doc = "Alternative stack reference"]
    #[serde(rename = "altStackReference", default, skip_serializing_if = "Option::is_none")]
    pub alt_stack_reference: Option<String>,
    #[doc = "A value indicating whether an offer has a 'Pay As You Go' plan"]
    #[serde(rename = "hasPaygPlans")]
    pub has_payg_plans: bool,
    #[doc = "A value indicating whether microsoft is acting as a reseller"]
    #[serde(rename = "isReseller")]
    pub is_reseller: bool,
    #[doc = "Expiration policy in seconds"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
    #[doc = "A value indicating whether the offer should excluded from search"]
    #[serde(rename = "isExcludedFromSearch")]
    pub is_excluded_from_search: bool,
    #[doc = "A value indicating what store fronts this offer should surface on"]
    #[serde(rename = "applicableStoreFronts", default, skip_serializing_if = "Option::is_none")]
    pub applicable_store_fronts: Option<serde_json::Value>,
    #[doc = "Offer version specified by the publisher on publishing portal"]
    #[serde(rename = "offerVersion", default, skip_serializing_if = "Option::is_none")]
    pub offer_version: Option<String>,
    #[doc = "A value indicating whether it is a microsoft product"]
    #[serde(rename = "isMicrosoftProduct", default, skip_serializing_if = "Option::is_none")]
    pub is_microsoft_product: Option<bool>,
    #[doc = "Product ownership selling motion"]
    #[serde(rename = "productOwnershipSellingMotion", default, skip_serializing_if = "Option::is_none")]
    pub product_ownership_selling_motion: Option<String>,
    #[doc = "The list of document links provided for the item"]
    #[serde(rename = "documentLinks", default, skip_serializing_if = "Vec::is_empty")]
    pub document_links: Vec<LinkProperties>,
    #[doc = "A value indicating offer's environment"]
    #[serde(rename = "offerEnvironment")]
    pub offer_environment: serde_json::Value,
    #[doc = "List of linked Add Ins provided for the item"]
    #[serde(rename = "linkedAddIns", default, skip_serializing_if = "Vec::is_empty")]
    pub linked_add_ins: Vec<String>,
    #[doc = "A value indicating whether the offer should not be re-ingest during bootstrap session"]
    #[serde(rename = "excludeFromBootstrap")]
    pub exclude_from_bootstrap: bool,
    #[doc = "Notification creation time"]
    #[serde(rename = "hydrationNotificationReceivedAt", default, with = "azure_core::date::rfc3339::option")]
    pub hydration_notification_received_at: Option<time::OffsetDateTime>,
    #[doc = "Last modified date"]
    #[serde(rename = "bigCatLastModifiedDate", default, with = "azure_core::date::rfc3339::option")]
    pub big_cat_last_modified_date: Option<time::OffsetDateTime>,
    #[doc = "Indication to disable sending email on purchase"]
    #[serde(rename = "disableSendEmailOnPurchase")]
    pub disable_send_email_on_purchase: bool,
    #[doc = "Indication to hide from SaaS blade"]
    #[serde(rename = "hideFromSaasBlade")]
    pub hide_from_saas_blade: bool,
    #[doc = "Indication if there is integrated with Microsoft graph API"]
    #[serde(rename = "integratedWithMicrosoftGraphApi")]
    pub integrated_with_microsoft_graph_api: bool,
    #[doc = "Multi tenant AAD app id"]
    #[serde(rename = "multiTenantAadAppId", default, skip_serializing_if = "Option::is_none")]
    pub multi_tenant_aad_app_id: Option<String>,
    #[doc = "License management type"]
    #[serde(rename = "licenseManagementType", default, skip_serializing_if = "Option::is_none")]
    pub license_management_type: Option<String>,
    #[doc = "License model"]
    #[serde(rename = "licenseModel", default, skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[doc = "PBI service principals"]
    #[serde(rename = "pbiServicePrincipalIds", default, skip_serializing_if = "Vec::is_empty")]
    pub pbi_service_principal_ids: Vec<String>,
    #[doc = "Set to true only for offers of OfferType.VirtualMachine to indicate that it was originally of OfferType.CoreVirtualMachine"]
    #[serde(rename = "isCoreVm", default, skip_serializing_if = "Option::is_none")]
    pub is_core_vm: Option<bool>,
    #[doc = "M365 Certification info"]
    #[serde(rename = "m365CertificationInfo", default, skip_serializing_if = "Option::is_none")]
    pub m365_certification_info: Option<serde_json::Value>,
    #[doc = "Download link for offers of type OfferType.PowerBIVisuals"]
    #[serde(rename = "downloadLink", default, skip_serializing_if = "Option::is_none")]
    pub download_link: Option<String>,
    #[doc = "Download sample report link for offers of type OfferType.PowerBIVisuals"]
    #[serde(rename = "downloadSampleLink", default, skip_serializing_if = "Option::is_none")]
    pub download_sample_link: Option<String>,
    #[doc = "Asset ID for offers of type OfferType.PowerBIVisuals"]
    #[serde(rename = "omexAssetId", default, skip_serializing_if = "Option::is_none")]
    pub omex_asset_id: Option<String>,
    #[doc = "Product ID for offers of type OfferType.PowerBIVisuals"]
    #[serde(rename = "mixProductId", default, skip_serializing_if = "Option::is_none")]
    pub mix_product_id: Option<String>,
    #[doc = "Whether an offer has additional purchases required"]
    #[serde(rename = "appFreeType", default, skip_serializing_if = "Option::is_none")]
    pub app_free_type: Option<String>,
    #[doc = "storefront pricings parameters for AppSource and AMP"]
    #[serde(rename = "storeFrontPricings", default, skip_serializing_if = "Option::is_none")]
    pub store_front_pricings: Option<serde_json::Value>,
}
impl CatalogItem {
    pub fn new(
        language: String,
        has_standard_contract_amendments: bool,
        offer_id: String,
        legacy_id: String,
        offer_type: serde_json::Value,
        is_private: bool,
        is_preview: bool,
        is_stop_sell: bool,
        fulfill_before_charge_eligible: bool,
        big_id: String,
        legal_terms_type: serde_json::Value,
        is_third_party: bool,
        popularity: f64,
        has_free_trials: bool,
        is_byol: bool,
        is_macc: bool,
        has_free_plans: bool,
        is_quantifiable: bool,
        has_payg_plans: bool,
        is_reseller: bool,
        is_excluded_from_search: bool,
        offer_environment: serde_json::Value,
        exclude_from_bootstrap: bool,
        disable_send_email_on_purchase: bool,
        hide_from_saas_blade: bool,
        integrated_with_microsoft_graph_api: bool,
    ) -> Self {
        Self {
            language,
            display_name: None,
            has_standard_contract_amendments,
            publisher_mpn_id: None,
            seller_id: None,
            publisher_id: None,
            partner_center_id: None,
            publisher_display_name: None,
            offer_id,
            legacy_id,
            determined_storefronts: Vec::new(),
            summary: None,
            long_summary: None,
            description: None,
            offer_type,
            is_private,
            is_preview,
            is_stop_sell,
            fulfill_before_charge_eligible,
            marketing_material: None,
            markets: Vec::new(),
            isv_contact_details: None,
            big_id,
            ocp_solution_id: None,
            legal_terms_uri: None,
            csp_legal_terms_uri: None,
            legal_terms_type,
            privacy_policy_uri: None,
            help_link: None,
            support_uri: None,
            version: None,
            ui_definition_uri: None,
            category_ids: Vec::new(),
            market_code: None,
            market_states: Vec::new(),
            industry_ids: Vec::new(),
            cloud_industry_categories: Vec::new(),
            primary_product: None,
            supported_products: Vec::new(),
            applicable_products: Vec::new(),
            service_type: None,
            competencies: Vec::new(),
            has_prices: None,
            duration: None,
            market_pricing_details: Vec::new(),
            pricing: None,
            solution_areas: Vec::new(),
            screenshot_uris: Vec::new(),
            links: Vec::new(),
            filters: Vec::new(),
            icon_file_uris: None,
            artifacts: Vec::new(),
            metadata: None,
            images: Vec::new(),
            videos: Vec::new(),
            plans: Vec::new(),
            resource_group_name: None,
            definition_templates: None,
            additional_properties: None,
            restricted_audience: None,
            is_third_party,
            group_id: None,
            hide_keys: Vec::new(),
            keywords: Vec::new(),
            popularity,
            pricing_details_uri: None,
            has_free_trials,
            is_byol,
            is_macc,
            has_free_plans,
            is_quantifiable,
            alt_stack_reference: None,
            has_payg_plans,
            is_reseller,
            ttl: None,
            is_excluded_from_search,
            applicable_store_fronts: None,
            offer_version: None,
            is_microsoft_product: None,
            product_ownership_selling_motion: None,
            document_links: Vec::new(),
            offer_environment,
            linked_add_ins: Vec::new(),
            exclude_from_bootstrap,
            hydration_notification_received_at: None,
            big_cat_last_modified_date: None,
            disable_send_email_on_purchase,
            hide_from_saas_blade,
            integrated_with_microsoft_graph_api,
            multi_tenant_aad_app_id: None,
            license_management_type: None,
            license_model: None,
            pbi_service_principal_ids: Vec::new(),
            is_core_vm: None,
            m365_certification_info: None,
            download_link: None,
            download_sample_link: None,
            omex_asset_id: None,
            mix_product_id: None,
            app_free_type: None,
            store_front_pricings: None,
        }
    }
}
#[doc = "enum type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CertificationType {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Competency {
    #[doc = "Competency name"]
    #[serde(rename = "competencyName", default, skip_serializing_if = "Option::is_none")]
    pub competency_name: Option<String>,
    #[doc = "Competency level"]
    #[serde(rename = "competencyLevel", default, skip_serializing_if = "Option::is_none")]
    pub competency_level: Option<String>,
}
impl Competency {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "enum type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CurrencyDecorator {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DefinitionTemplates {
    #[doc = "UI definition file URI"]
    #[serde(rename = "uiDefinitionFileUri", default, skip_serializing_if = "Option::is_none")]
    pub ui_definition_file_uri: Option<String>,
    #[doc = "Default deployment template id"]
    #[serde(rename = "defaultDeploymentTemplateId", default, skip_serializing_if = "Option::is_none")]
    pub default_deployment_template_id: Option<String>,
    #[doc = "A dictionary containing the deployment template file IDs defined in the package manifest and corresponding URIs"]
    #[serde(rename = "deploymentTemplateFileUris", default, skip_serializing_if = "Option::is_none")]
    pub deployment_template_file_uris: Option<serde_json::Value>,
    #[doc = "a dictionary containing the deployment fragment file IDs defined in the package manifest and corresponding URIs"]
    #[serde(rename = "deploymentFragmentFileUris", default, skip_serializing_if = "Option::is_none")]
    pub deployment_fragment_file_uris: Option<serde_json::Value>,
}
impl DefinitionTemplates {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Duration {
    #[doc = "Duration value"]
    #[serde(rename = "durationValue")]
    pub duration_value: i64,
    #[doc = "Duration unit"]
    #[serde(rename = "durationUnit")]
    pub duration_unit: serde_json::Value,
}
impl Duration {
    pub fn new(duration_value: i64, duration_unit: serde_json::Value) -> Self {
        Self {
            duration_value,
            duration_unit,
        }
    }
}
#[doc = "enum type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EnvironmentInfo {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Filter {
    #[doc = "Filter type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Filter value"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl Filter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "enum type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum IconKind {}
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageGroup {
    #[doc = "context"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[doc = "List of images"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Image>,
}
impl ImageGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IncludedQuantityProperty {
    #[serde(flatten)]
    pub included_quantity_property2: IncludedQuantityProperty2,
}
impl IncludedQuantityProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IncludedQuantityProperty2 {
    #[doc = "Term id"]
    #[serde(rename = "termId", default, skip_serializing_if = "Option::is_none")]
    pub term_id: Option<String>,
    #[doc = "Quantity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
}
impl IncludedQuantityProperty2 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LeadGeneration {
    #[doc = "Product Id"]
    #[serde(rename = "productId", default, skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}
impl LeadGeneration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "enum type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LegalTermsType {
    None,
    #[serde(rename = "EA")]
    Ea,
}
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct M365CertificationInfo {
    #[doc = "M365 Certification Type"]
    #[serde(rename = "m365CertificationType")]
    pub m365_certification_type: serde_json::Value,
    #[doc = "M365 Certification Url"]
    #[serde(rename = "m365CertificationDetailsUrl", default, skip_serializing_if = "Option::is_none")]
    pub m365_certification_details_url: Option<String>,
    #[doc = "M365 Certification Id"]
    #[serde(rename = "m365CertificationId", default, skip_serializing_if = "Option::is_none")]
    pub m365_certification_id: Option<String>,
}
impl M365CertificationInfo {
    pub fn new(m365_certification_type: serde_json::Value) -> Self {
        Self {
            m365_certification_type,
            m365_certification_details_url: None,
            m365_certification_id: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketPricingDetailsItem {
    #[doc = "Pricing that applies to the item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pricing: Option<serde_json::Value>,
    #[doc = "Market code of a service offer"]
    #[serde(rename = "marketCode", default, skip_serializing_if = "Option::is_none")]
    pub market_code: Option<String>,
    #[doc = "Market states of a service offer"]
    #[serde(rename = "marketStates", default, skip_serializing_if = "Vec::is_empty")]
    pub market_states: Vec<String>,
}
impl MarketPricingDetailsItem {
    pub fn new() -> Self {
        Self::default()
    }
}
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
    #[doc = "Type for this meter"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Included quantity properties"]
    #[serde(rename = "includedQuantityProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub included_quantity_properties: Vec<IncludedQuantityProperty>,
}
impl Meter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OfferMetadata {
    #[doc = "Lead generation info"]
    #[serde(rename = "leadGeneration", default, skip_serializing_if = "Option::is_none")]
    pub lead_generation: Option<serde_json::Value>,
    #[doc = "Test Drive info"]
    #[serde(rename = "testDrive", default, skip_serializing_if = "Option::is_none")]
    pub test_drive: Option<serde_json::Value>,
}
impl OfferMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "enum type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OfferType {
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
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenProperty {
    #[doc = "VM generation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl OpenProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperatingSystem {
    #[doc = "Operating system family"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "Operating system type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Operating system name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl OperatingSystem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PiFilter {
    #[doc = "List of exclusion properties"]
    #[serde(rename = "exclusionProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub exclusion_properties: Vec<String>,
    #[doc = "List of inclusion properties"]
    #[serde(rename = "inclusionProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub inclusion_properties: Vec<String>,
}
impl PiFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a feed of entities that includes additional information that OData formats support."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PageResultOfCatalogItem {
    #[doc = "Gets the collection of entities for this feed."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CatalogItem>,
    #[doc = "Gets the link for the next page of items in the feed."]
    #[serde(rename = "nextPageLink", default, skip_serializing_if = "Option::is_none")]
    pub next_page_link: Option<String>,
    #[doc = "Gets the total count of items in the feed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl PageResultOfCatalogItem {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plan {
    #[doc = "Plan id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Display name of the plan"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Summary of the plan"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[doc = "Description of the plan"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Restricted audience"]
    #[serde(rename = "restrictedAudience", default, skip_serializing_if = "Option::is_none")]
    pub restricted_audience: Option<serde_json::Value>,
    #[doc = "Sku id"]
    #[serde(rename = "skuId", default, skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    #[doc = "Plan id"]
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[doc = "Legacy plan Id which is obtained by combining Offer->LegacyId and PlanId with no separator in between"]
    #[serde(rename = "legacyPlanId", default, skip_serializing_if = "Option::is_none")]
    pub legacy_plan_id: Option<String>,
    #[doc = "List of keywords"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
    #[doc = "Offer type"]
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
    #[doc = "Lead generation info"]
    #[serde(rename = "leadGeneration", default, skip_serializing_if = "Option::is_none")]
    pub lead_generation: Option<serde_json::Value>,
    #[doc = "Test Drive info"]
    #[serde(rename = "testDrive", default, skip_serializing_if = "Option::is_none")]
    pub test_drive: Option<serde_json::Value>,
    #[doc = "List of availabilities for this plan"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub availabilities: Vec<AvailabilityEntity>,
    #[doc = "Category IDs for this plan"]
    #[serde(rename = "categoryIds", default, skip_serializing_if = "Vec::is_empty")]
    pub category_ids: Vec<String>,
    #[doc = "Conversion paths for this plan"]
    #[serde(rename = "conversionPaths", default, skip_serializing_if = "Vec::is_empty")]
    pub conversion_paths: Vec<String>,
    #[doc = "Metadata for this plan"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "Operating system info for this plan"]
    #[serde(rename = "operatingSystem", default, skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<serde_json::Value>,
    #[doc = "What blade to be opened when someone wants to create the marketplace item"]
    #[serde(rename = "uiDefinitionUri", default, skip_serializing_if = "Option::is_none")]
    pub ui_definition_uri: Option<String>,
    #[doc = "Files related to the marketplace item"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<Artifact>,
    #[doc = "Version of the marketplace item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Item name"]
    #[serde(rename = "itemName", default, skip_serializing_if = "Option::is_none")]
    pub item_name: Option<String>,
    #[doc = "A value indicating whether the item is private"]
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
    #[doc = "A value indicating whether the plan is hidden"]
    #[serde(rename = "isHidden")]
    pub is_hidden: bool,
    #[doc = "A value indicating whether it has free trials"]
    #[serde(rename = "hasFreeTrials")]
    pub has_free_trials: bool,
    #[doc = "A value indicating whether it has licensed plans"]
    #[serde(rename = "isByol")]
    pub is_byol: bool,
    #[doc = "A value indicating whether it has at least one free availability in any market"]
    #[serde(rename = "isFree")]
    pub is_free: bool,
    #[doc = "A value indicating whether the plan is 'Pay As You Go'"]
    #[serde(rename = "isPayg")]
    pub is_payg: bool,
    #[doc = "A value indicating whether it has been stopped from sell in a market"]
    #[serde(rename = "isStopSell")]
    pub is_stop_sell: bool,
    #[doc = "Alternative stack reference"]
    #[serde(rename = "altStackReference", default, skip_serializing_if = "Option::is_none")]
    pub alt_stack_reference: Option<String>,
    #[doc = "Stack type : Classic, Gen1, Gen2"]
    #[serde(rename = "stackType", default, skip_serializing_if = "Option::is_none")]
    pub stack_type: Option<String>,
    #[doc = "A value indicating whether the product is available for purchase through CSP channel"]
    #[serde(rename = "cspState")]
    pub csp_state: serde_json::Value,
    #[doc = "Resource Provider Namespace"]
    #[serde(rename = "resourceProviderNamespace", default, skip_serializing_if = "Option::is_none")]
    pub resource_provider_namespace: Option<String>,
    #[doc = "Resource Provider type"]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Min quantity"]
    #[serde(rename = "minQuantity", default, skip_serializing_if = "Option::is_none")]
    pub min_quantity: Option<i32>,
    #[doc = "Max quantity"]
    #[serde(rename = "maxQuantity", default, skip_serializing_if = "Option::is_none")]
    pub max_quantity: Option<i32>,
    #[doc = "A value indicating whether the plan is quantifiable"]
    #[serde(rename = "isQuantifiable")]
    pub is_quantifiable: bool,
    #[doc = "Ahe action that can be performed on this plan on the storefronts"]
    #[serde(rename = "callToAction", default, skip_serializing_if = "Option::is_none")]
    pub call_to_action: Option<String>,
    #[doc = "URL to redirect the user to post the performed action on the storefronts"]
    #[serde(rename = "redirectUrl", default, skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[doc = "Service identifier"]
    #[serde(rename = "serviceIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub service_identifier: Option<String>,
    #[doc = "VM security type"]
    #[serde(rename = "vmSecurityType", default, skip_serializing_if = "Option::is_none")]
    pub vm_security_type: Option<serde_json::Value>,
    #[doc = "Display rank"]
    #[serde(rename = "displayRank", default, skip_serializing_if = "Option::is_none")]
    pub display_rank: Option<String>,
}
impl Plan {
    pub fn new(
        type_: serde_json::Value,
        is_private: bool,
        is_hidden: bool,
        has_free_trials: bool,
        is_byol: bool,
        is_free: bool,
        is_payg: bool,
        is_stop_sell: bool,
        csp_state: serde_json::Value,
        is_quantifiable: bool,
    ) -> Self {
        Self {
            id: None,
            display_name: None,
            summary: None,
            description: None,
            restricted_audience: None,
            sku_id: None,
            plan_id: None,
            legacy_plan_id: None,
            keywords: Vec::new(),
            type_,
            lead_generation: None,
            test_drive: None,
            availabilities: Vec::new(),
            category_ids: Vec::new(),
            conversion_paths: Vec::new(),
            metadata: None,
            operating_system: None,
            ui_definition_uri: None,
            artifacts: Vec::new(),
            version: None,
            item_name: None,
            is_private,
            is_hidden,
            has_free_trials,
            is_byol,
            is_free,
            is_payg,
            is_stop_sell,
            alt_stack_reference: None,
            stack_type: None,
            csp_state,
            resource_provider_namespace: None,
            resource_type: None,
            min_quantity: None,
            max_quantity: None,
            is_quantifiable,
            call_to_action: None,
            redirect_url: None,
            service_identifier: None,
            vm_security_type: None,
            display_rank: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanPrice {
    #[doc = "Plan id"]
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[doc = "Currency decorator"]
    #[serde(rename = "currencyDecorator")]
    pub currency_decorator: serde_json::Value,
    #[doc = "Price of the item"]
    pub price: f64,
}
impl PlanPrice {
    pub fn new(currency_decorator: serde_json::Value, price: f64) -> Self {
        Self {
            plan_id: None,
            currency_decorator,
            price,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PreviewImage {
    #[doc = "Caption"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[doc = "Image uri"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "Image purpose"]
    #[serde(rename = "imagePurpose", default, skip_serializing_if = "Option::is_none")]
    pub image_purpose: Option<String>,
}
impl PreviewImage {
    pub fn new() -> Self {
        Self::default()
    }
}
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Pricing {
    #[doc = "Currency code"]
    #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[doc = "Plan prices"]
    #[serde(rename = "planPrices", default, skip_serializing_if = "Vec::is_empty")]
    pub plan_prices: Vec<PlanPrice>,
}
impl Pricing {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "enum type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PricingAudience {
    None,
    DirectCommercial,
    PartnerCommercial,
    Custom,
    IndirectCommercial,
    IndirectGov,
    DirectChk,
    DirectBlue,
}
#[doc = "enum type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PricingOptions {}
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Remediation {
    #[doc = "Remediation identifier"]
    #[serde(rename = "remediationId", default, skip_serializing_if = "Option::is_none")]
    pub remediation_id: Option<String>,
    #[doc = "Remediation type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Remediation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestrictedAudience {
    #[doc = "Subscription based restricted audience"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subscriptions: Vec<String>,
    #[doc = "Tenant based restricted audience"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tenants: Vec<String>,
    #[doc = "User based restricted audience"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<String>,
}
impl RestrictedAudience {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "enum type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServiceDurationUnit {
    Days,
    Hours,
    Weeks,
}
#[doc = "enum type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Store {
    Appsource,
    #[serde(rename = "AMP")]
    Amp,
    Ibiza,
    Cosell,
}
#[doc = "enum type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum StoreFrontOptions {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Term {
    #[doc = "Term description parameters"]
    #[serde(rename = "termDescriptionParameters", default, skip_serializing_if = "Vec::is_empty")]
    pub term_description_parameters: Vec<TermDescriptionParameter>,
    #[doc = "Term id"]
    #[serde(rename = "termId", default, skip_serializing_if = "Option::is_none")]
    pub term_id: Option<String>,
    #[doc = "Term unit"]
    #[serde(rename = "termUnits", default, skip_serializing_if = "Option::is_none")]
    pub term_units: Option<String>,
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
}
impl Term {
    pub fn new() -> Self {
        Self::default()
    }
}
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestDrive {
    #[doc = "Description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "User manual"]
    #[serde(rename = "userManual", default, skip_serializing_if = "Option::is_none")]
    pub user_manual: Option<String>,
    #[doc = "Test Drive Duration"]
    #[serde(rename = "testDriveDuration", default, skip_serializing_if = "Option::is_none")]
    pub test_drive_duration: Option<String>,
    #[doc = "Access Information"]
    #[serde(rename = "accessInformation", default, skip_serializing_if = "Option::is_none")]
    pub access_information: Option<String>,
    #[doc = "Orchestration Type"]
    #[serde(rename = "orchestrationType", default, skip_serializing_if = "Option::is_none")]
    pub orchestration_type: Option<String>,
    #[doc = "Lab identifier"]
    #[serde(rename = "labId", default, skip_serializing_if = "Option::is_none")]
    pub lab_id: Option<String>,
    #[doc = "Demo identifier"]
    #[serde(rename = "demoId", default, skip_serializing_if = "Option::is_none")]
    pub demo_id: Option<String>,
    #[doc = "Walk-through video"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video: Option<serde_json::Value>,
    #[doc = "Power Bi dashboard link"]
    #[serde(rename = "powerBiDashboardLink", default, skip_serializing_if = "Option::is_none")]
    pub power_bi_dashboard_link: Option<String>,
}
impl TestDrive {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UiPricing {
    #[doc = "A value indicating what pricing options are available on the application"]
    #[serde(rename = "pricingOptions")]
    pub pricing_options: serde_json::Value,
    #[doc = "Has prices"]
    #[serde(rename = "hasPrices", default, skip_serializing_if = "Option::is_none")]
    pub has_prices: Option<bool>,
}
impl UiPricing {
    pub fn new(pricing_options: serde_json::Value) -> Self {
        Self {
            pricing_options,
            has_prices: None,
        }
    }
}
#[doc = "enum type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum VmSecurityType {
    None,
    Trusted,
    Confidential,
}
