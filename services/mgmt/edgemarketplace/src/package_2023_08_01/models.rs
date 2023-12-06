#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Access token request object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessTokenReadRequest {
    #[doc = "The name of the publisher."]
    #[serde(rename = "requestId")]
    pub request_id: String,
}
impl AccessTokenReadRequest {
    pub fn new(request_id: String) -> Self {
        Self { request_id }
    }
}
#[doc = "Access token request object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessTokenRequest {
    #[doc = "The name of the publisher."]
    #[serde(rename = "publisherName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_name: Option<String>,
    #[doc = "The region where the disk will be created."]
    #[serde(rename = "edgeMarketPlaceRegion")]
    pub edge_market_place_region: String,
    #[doc = "The region where the disk will be created."]
    #[serde(rename = "egeMarketPlaceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub ege_market_place_resource_id: Option<String>,
    #[doc = "The hyperv version."]
    #[serde(rename = "hypervGeneration", default, skip_serializing_if = "Option::is_none")]
    pub hyperv_generation: Option<String>,
    #[doc = "The marketplace sku."]
    #[serde(rename = "marketPlaceSku", default, skip_serializing_if = "Option::is_none")]
    pub market_place_sku: Option<String>,
    #[doc = "The marketplace sku version."]
    #[serde(rename = "marketPlaceSkuVersion", default, skip_serializing_if = "Option::is_none")]
    pub market_place_sku_version: Option<String>,
    #[doc = "The device sku."]
    #[serde(rename = "deviceSku", default, skip_serializing_if = "Option::is_none")]
    pub device_sku: Option<String>,
    #[doc = "The device sku version."]
    #[serde(rename = "deviceVersion", default, skip_serializing_if = "Option::is_none")]
    pub device_version: Option<String>,
}
impl AccessTokenRequest {
    pub fn new(edge_market_place_region: String) -> Self {
        Self {
            publisher_name: None,
            edge_market_place_region,
            ege_market_place_resource_id: None,
            hyperv_generation: None,
            market_place_sku: None,
            market_place_sku_version: None,
            device_sku: None,
            device_version: None,
        }
    }
}
#[doc = "The provisioning state of a resource type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureResourceManagerResourceProvisioningState")]
pub enum AzureResourceManagerResourceProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureResourceManagerResourceProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureResourceManagerResourceProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureResourceManagerResourceProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("AzureResourceManagerResourceProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("AzureResourceManagerResourceProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("AzureResourceManagerResourceProvisioningState", 2u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The disk access token"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskAccessToken {
    #[doc = "The disk id."]
    #[serde(rename = "diskId", default, skip_serializing_if = "Option::is_none")]
    pub disk_id: Option<String>,
    #[doc = "The access token creation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The access token."]
    #[serde(rename = "accessToken")]
    pub access_token: String,
}
impl DiskAccessToken {
    pub fn new(access_token: String) -> Self {
        Self {
            disk_id: None,
            status: None,
            access_token,
        }
    }
}
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
#[doc = "Icon files"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IconFileUris {
    #[doc = "uri of small icon"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub small: Option<String>,
    #[doc = "uri of medium icon"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    #[doc = "uri of wide icon"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wide: Option<String>,
    #[doc = "uri of large icon"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub large: Option<String>,
}
impl IconFileUris {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The marketplace sku"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketplaceSku {
    #[doc = "The catalog plan id"]
    #[serde(rename = "catalogPlanId")]
    pub catalog_plan_id: String,
    #[doc = "The marketplace sku id"]
    #[serde(rename = "marketplaceSkuId")]
    pub marketplace_sku_id: String,
    #[doc = "The type of marketplace sku"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The display name of marketplace sku"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The summary"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[doc = "The long summary"]
    #[serde(rename = "longSummary", default, skip_serializing_if = "Option::is_none")]
    pub long_summary: Option<String>,
    #[doc = "The description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The generation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,
    #[doc = "The display rank of the sku"]
    #[serde(rename = "displayRank", default, skip_serializing_if = "Option::is_none")]
    pub display_rank: Option<i32>,
    #[doc = "The sku operating system"]
    #[serde(rename = "operatingSystem", default, skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<SkuOperatingSystem>,
    #[doc = "The marketplace sku version"]
    #[serde(
        rename = "marketplaceSkuVersions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub marketplace_sku_versions: Vec<MarketplaceSkuVersion>,
}
impl MarketplaceSku {
    pub fn new(catalog_plan_id: String, marketplace_sku_id: String) -> Self {
        Self {
            catalog_plan_id,
            marketplace_sku_id,
            type_: None,
            display_name: None,
            summary: None,
            long_summary: None,
            description: None,
            generation: None,
            display_rank: None,
            operating_system: None,
            marketplace_sku_versions: Vec::new(),
        }
    }
}
#[doc = "The marketplace sku version"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarketplaceSkuVersion {
    #[doc = "The name of sku version"]
    pub name: String,
    #[doc = "The size of the image"]
    #[serde(rename = "sizeOnDiskInMb", default, skip_serializing_if = "Option::is_none")]
    pub size_on_disk_in_mb: Option<i32>,
    #[doc = "The size of the download"]
    #[serde(rename = "minimumDownloadSizeInMb", default, skip_serializing_if = "Option::is_none")]
    pub minimum_download_size_in_mb: Option<i32>,
    #[doc = "The stage name"]
    #[serde(rename = "stageName", default, skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
}
impl MarketplaceSkuVersion {
    pub fn new(name: String) -> Self {
        Self {
            name,
            size_on_disk_in_mb: None,
            minimum_download_size_in_mb: None,
            stage_name: None,
        }
    }
}
#[doc = "An offer."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Offer {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The offer properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OfferProperties>,
}
impl Offer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Says if the offer is public/private"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OfferAvailability")]
pub enum OfferAvailability {
    Private,
    Public,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OfferAvailability {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OfferAvailability {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OfferAvailability {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Private => serializer.serialize_unit_variant("OfferAvailability", 0u32, "Private"),
            Self::Public => serializer.serialize_unit_variant("OfferAvailability", 1u32, "Public"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The offer content"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfferContent {
    #[doc = "The display name of the offer"]
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[doc = "The summary"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[doc = "The long summary"]
    #[serde(rename = "longSummary", default, skip_serializing_if = "Option::is_none")]
    pub long_summary: Option<String>,
    #[doc = "The description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The offer id"]
    #[serde(rename = "offerId")]
    pub offer_id: String,
    #[doc = "The offer type"]
    #[serde(rename = "offerType", default, skip_serializing_if = "Option::is_none")]
    pub offer_type: Option<String>,
    #[doc = "The support uri"]
    #[serde(rename = "supportUri", default, skip_serializing_if = "Option::is_none")]
    pub support_uri: Option<String>,
    #[doc = "The popularity of the offer"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub popularity: Option<i32>,
    #[doc = "The offer publisher"]
    #[serde(rename = "offerPublisher", default, skip_serializing_if = "Option::is_none")]
    pub offer_publisher: Option<OfferPublisher>,
    #[doc = "Says if the offer is public/private"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability: Option<OfferAvailability>,
    #[doc = "Offer release type"]
    #[serde(rename = "releaseType", default, skip_serializing_if = "Option::is_none")]
    pub release_type: Option<OfferReleaseType>,
    #[doc = "Icon files"]
    #[serde(rename = "iconFileUris", default, skip_serializing_if = "Option::is_none")]
    pub icon_file_uris: Option<IconFileUris>,
    #[doc = "Terms and conditions"]
    #[serde(rename = "termsAndConditions", default, skip_serializing_if = "Option::is_none")]
    pub terms_and_conditions: Option<TermsAndConditions>,
    #[doc = "The category ids "]
    #[serde(
        rename = "categoryIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub category_ids: Vec<String>,
    #[doc = "The operating systems"]
    #[serde(
        rename = "operatingSystems",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operating_systems: Vec<String>,
}
impl OfferContent {
    pub fn new(display_name: String, offer_id: String) -> Self {
        Self {
            display_name,
            summary: None,
            long_summary: None,
            description: None,
            offer_id,
            offer_type: None,
            support_uri: None,
            popularity: None,
            offer_publisher: None,
            availability: None,
            release_type: None,
            icon_file_uris: None,
            terms_and_conditions: None,
            category_ids: Vec::new(),
            operating_systems: Vec::new(),
        }
    }
}
#[doc = "The response of a Offer list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfferListResult {
    #[doc = "The Offer items on this page"]
    pub value: Vec<Offer>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OfferListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OfferListResult {
    pub fn new(value: Vec<Offer>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The offer properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfferProperties {
    #[doc = "The content version"]
    #[serde(rename = "contentVersion", default, skip_serializing_if = "Option::is_none")]
    pub content_version: Option<String>,
    #[doc = "The content url"]
    #[serde(rename = "contentUrl", default, skip_serializing_if = "Option::is_none")]
    pub content_url: Option<String>,
    #[doc = "The offer content"]
    #[serde(rename = "offerContent")]
    pub offer_content: OfferContent,
    #[doc = "The provisioning state of a resource type."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<AzureResourceManagerResourceProvisioningState>,
    #[doc = "The marketplace skus"]
    #[serde(
        rename = "marketplaceSkus",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub marketplace_skus: Vec<MarketplaceSku>,
}
impl OfferProperties {
    pub fn new(offer_content: OfferContent) -> Self {
        Self {
            content_version: None,
            content_url: None,
            offer_content,
            provisioning_state: None,
            marketplace_skus: Vec::new(),
        }
    }
}
#[doc = "The offer publisher"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfferPublisher {
    #[doc = "The publisher Id"]
    #[serde(rename = "publisherId")]
    pub publisher_id: String,
    #[doc = "The publisher name"]
    #[serde(rename = "publisherDisplayName")]
    pub publisher_display_name: String,
}
impl OfferPublisher {
    pub fn new(publisher_id: String, publisher_display_name: String) -> Self {
        Self {
            publisher_id,
            publisher_display_name,
        }
    }
}
#[doc = "Offer release type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OfferReleaseType")]
pub enum OfferReleaseType {
    Preview,
    #[serde(rename = "GA")]
    Ga,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OfferReleaseType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OfferReleaseType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OfferReleaseType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Preview => serializer.serialize_unit_variant("OfferReleaseType", 0u32, "Preview"),
            Self::Ga => serializer.serialize_unit_variant("OfferReleaseType", 1u32, "GA"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
#[doc = "A publisher who provides offers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Publisher {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Publisher properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PublisherProperties>,
}
impl Publisher {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a Publisher list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublisherListResult {
    #[doc = "The Publisher items on this page"]
    pub value: Vec<Publisher>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PublisherListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PublisherListResult {
    pub fn new(value: Vec<Publisher>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Publisher properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublisherProperties {
    #[doc = "The provisioning state of a resource type."]
    #[serde(rename = "provisioningState")]
    pub provisioning_state: AzureResourceManagerResourceProvisioningState,
}
impl PublisherProperties {
    pub fn new(provisioning_state: AzureResourceManagerResourceProvisioningState) -> Self {
        Self { provisioning_state }
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
#[doc = "The sku operating system"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuOperatingSystem {
    #[doc = "The family of the operating system"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "The type of the operating system"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of the operating system"]
    pub name: String,
}
impl SkuOperatingSystem {
    pub fn new(name: String) -> Self {
        Self {
            family: None,
            type_: None,
            name,
        }
    }
}
#[doc = "Terms and conditions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TermsAndConditions {
    #[doc = "The legal terms and conditions uri"]
    #[serde(rename = "legalTermsUri", default, skip_serializing_if = "Option::is_none")]
    pub legal_terms_uri: Option<String>,
    #[doc = "The type of legal terms"]
    #[serde(rename = "legalTermsType", default, skip_serializing_if = "Option::is_none")]
    pub legal_terms_type: Option<String>,
    #[doc = "The privacy policy uri"]
    #[serde(rename = "privacyPolicyUri", default, skip_serializing_if = "Option::is_none")]
    pub privacy_policy_uri: Option<String>,
}
impl TermsAndConditions {
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
