#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommonResourceProperties {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type. \"Microsoft.Quota/quotaLimits\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CommonResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quota change requests information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateGenericQuotaRequestParameters {
    #[doc = "Quota change requests."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CurrentQuotaLimitBase>,
}
impl CreateGenericQuotaRequestParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quota limit."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CurrentQuotaLimitBase {
    #[doc = "The resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Quota properties for the specified resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaProperties>,
}
impl CurrentQuotaLimitBase {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExceptionResponse {
    #[doc = "API error details."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Provider name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Operation description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationResponse>,
    #[doc = "URL to get the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl OperationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quota limits."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaLimits {
    #[doc = "List of quota limits."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CurrentQuotaLimitBase>,
    #[doc = "The URI used to fetch the next page of quota limits. When there are no more pages, this is null."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for QuotaLimits {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl QuotaLimits {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quota limits request response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaLimitsResponse {
    #[doc = "List of quota limits with the quota request status."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CurrentQuotaLimitBase>,
    #[doc = "The URI used to fetch the next page of quota limits. When there are no more pages, this is null."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl QuotaLimitsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quota properties for the specified resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaProperties {
    #[doc = "Quota limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[doc = "Usage information for the current resource."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i32>,
    #[doc = " The quota limit units, such as Count and Bytes. When requesting quota, use the **unit** value returned in the GET response in the request body of your PUT operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Name of the resource provided by the resource Provider. When requesting quota, use this property name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ResourceName>,
    #[doc = "Resource types. For extensibility, it is a string."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<ResourceTypesName>,
    #[doc = "The time period over which the quota usage values are summarized. For example:\r\n*P1D (per one day)\n*PT1M (per one minute)\n*PT1S (per one second).\r\nThis parameter is optional because, for some resources like compute, the period is irrelevant."]
    #[serde(rename = "quotaPeriod", default, skip_serializing_if = "Option::is_none")]
    pub quota_period: Option<String>,
    #[doc = "States if quota can be requested for this resource."]
    #[serde(rename = "isQuotaApplicable", default, skip_serializing_if = "Option::is_none")]
    pub is_quota_applicable: Option<bool>,
    #[doc = "Additional properties for the specific resource provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl QuotaProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of quota requests with details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaRequestDetails {
    #[doc = "Quota request ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Quota request name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type. \"Microsoft.Quota/quotaLimits\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Quota request properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaRequestProperties>,
}
impl QuotaRequestDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quota request information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaRequestDetailsList {
    #[doc = "Quota request details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<QuotaRequestDetails>,
    #[doc = "The URI for fetching the next page of quota limits. When there are no more pages, this is null."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for QuotaRequestDetailsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl QuotaRequestDetailsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quota request response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaRequestOneResourceSubmitResponse {
    #[doc = "Quota request ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the quota request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type. \"Microsoft.Quota/ServiceLimitRequests\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Quota request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaRequestOneResourceProperties>,
}
impl QuotaRequestOneResourceSubmitResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quota request properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaRequestProperties {
    #[doc = "Quota request status."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<QuotaRequestState>,
    #[doc = "User-friendly status message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The quota request submission time. The date conforms to the following format specified by the ISO 8601 standard: yyyy-MM-ddTHH:mm:ssZ"]
    #[serde(rename = "requestSubmitTime", with = "azure_core::date::rfc3339::option")]
    pub request_submit_time: Option<time::OffsetDateTime>,
    #[doc = "Quota request details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SubRequest>,
}
impl QuotaRequestProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quota request status."]
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
    #[doc = "Quota request status."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<QuotaRequestState>,
    #[doc = "User-friendly message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Quota limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[doc = "Usage information for the current resource."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i32>,
    #[doc = " The quota limit units, such as Count and Bytes. When requesting quota, use the **unit** value returned in the GET response in the request body of your PUT operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Name of the resource provided by the resource Provider. When requesting quota, use this property name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ResourceName>,
    #[doc = "Resource types. For extensibility, it is a string."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<ResourceTypesName>,
    #[doc = "The time period over which the quota usage values are summarized. For example:\r\n*P1D (per one day)\n*PT1M (per one minute)\n*PT1S (per one second).\r\nThis parameter is optional because, for some resources like compute, the period is irrelevant."]
    #[serde(rename = "quotaPeriod", default, skip_serializing_if = "Option::is_none")]
    pub quota_period: Option<String>,
    #[doc = "Additional properties for the specific resource provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl QuotaRequestStatusDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quota request response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaRequestSubmitResponse {
    #[doc = "Quota request ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Quota request name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Quota request properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaRequestProperties>,
    #[doc = "Resource type. \"Microsoft.Quota/quotaLimits\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl QuotaRequestSubmitResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The quota request response with the quota request ID."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaRequestSubmitResponse202 {
    #[doc = "The quota request ID. To check the request status, use the **id** value in a [Quota Request Status](https://docs.microsoft.com/en-us/rest/api/reserved-vm-instances/quotarequeststatus/get) GET operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Operation ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Quota request status details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaRequestStatusDetails>,
}
impl QuotaRequestSubmitResponse202 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quota template details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaTemplateDetails {
    #[doc = "Resource type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Quota types information list."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<QuotaTypeInformationDimensionList>,
}
impl QuotaTemplateDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quota templates details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaTemplatesDetails {
    #[doc = "Quota templates information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<QuotaTemplateDetails>,
}
impl QuotaTemplatesDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quota type information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaTypeDimensionInformation {
    #[doc = "Property name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Dimension ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl QuotaTypeDimensionInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quota types information list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaTypeInformationDimensionList {
    #[doc = "Resource type"]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Quota type information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<QuotaTypeDimensionInformation>,
}
impl QuotaTypeInformationDimensionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Name of the resource provided by the resource Provider. When requesting quota, use this property name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceName {
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Resource display name."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl ResourceName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource provider resource dimension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderDimension {
    #[doc = "Resource dimension name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl ResourceProviderDimension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource provider information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderInformation {
    #[doc = "Resource provider name."]
    #[serde(rename = "resourceProviderName", default, skip_serializing_if = "Option::is_none")]
    pub resource_provider_name: Option<String>,
    #[doc = "List of resource provider templates"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ResourceProviderTemplates>,
}
impl ResourceProviderInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource template details for the resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderTemplate {
    #[doc = "Resource type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Resource query details."]
    #[serde(rename = "resourceQuery", default, skip_serializing_if = "Option::is_none")]
    pub resource_query: Option<ResourceQueryDetails>,
    #[doc = "Resource query details."]
    #[serde(rename = "resourceUsagesQuery", default, skip_serializing_if = "Option::is_none")]
    pub resource_usages_query: Option<ResourceQueryDetails>,
    #[doc = "Resource provider dimensions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<ResourceProviderDimension>,
}
impl ResourceProviderTemplate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of resource provider templates"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderTemplates {
    #[doc = "The resource provider templates"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceProviderTemplate>,
}
impl ResourceProviderTemplates {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource query method."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResourceQueryMethod")]
pub enum ResourceQueryMethod {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResourceQueryMethod {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResourceQueryMethod {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResourceQueryMethod {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Get => serializer.serialize_unit_variant("ResourceQueryMethod", 0u32, "GET"),
            Self::Post => serializer.serialize_unit_variant("ResourceQueryMethod", 1u32, "POST"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Resource query types. For extensibility, it is a string."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResourceQueryType")]
pub enum ResourceQueryType {
    #[serde(rename = "ARG")]
    Arg,
    #[serde(rename = "RestAPI")]
    RestApi,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResourceQueryType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResourceQueryType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResourceQueryType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Arg => serializer.serialize_unit_variant("ResourceQueryType", 0u32, "ARG"),
            Self::RestApi => serializer.serialize_unit_variant("ResourceQueryType", 1u32, "RestAPI"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Resource types. For extensibility, it is a string."]
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
#[doc = "API error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceError {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "List of error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ServiceErrorDetail>,
}
impl ServiceError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceErrorDetail {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ServiceErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Request property."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubRequest {
    #[doc = "Resource quota limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[doc = "Name of the resource provided by the resource Provider. When requesting quota, use this property name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ResourceName>,
    #[doc = "Resource type for which the quota check was made."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = " Quota limit units, such as Count and Bytes. When requesting quota, use the **unit** value returned in the GET response in the request body of your PUT operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Quota request status."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<QuotaRequestState>,
    #[doc = "User-friendly status message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Quota request ID."]
    #[serde(rename = "subRequestId", default, skip_serializing_if = "Option::is_none")]
    pub sub_request_id: Option<String>,
}
impl SubRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "quotaBucket provider properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaBucketProperties {
    #[doc = "Property name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl QuotaBucketProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quota request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaRequestOneResourceProperties {
    #[doc = "Quota request status."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<QuotaRequestState>,
    #[doc = "User-friendly status message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Quota request submission time. The date conforms to the following ISO 8601 standard format: yyyy-MM-ddTHH:mm:ssZ."]
    #[serde(rename = "requestSubmitTime", with = "azure_core::date::rfc3339::option")]
    pub request_submit_time: Option<time::OffsetDateTime>,
    #[doc = "Quota limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[doc = "Usage information for the current resource."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i32>,
    #[doc = " The quota limit units, such as Count and Bytes. When requesting quota, use the **unit** value returned in the GET response in the request body of your PUT operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Name of the resource provided by the resource Provider. When requesting quota, use this property name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ResourceName>,
    #[doc = "Resource types. For extensibility, it is a string."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<ResourceTypesName>,
    #[doc = "The time period over which the quota usage values are summarized. For example:\r\n*P1D (per one day)\n*PT1M (per one minute)\n*PT1S (per one second).\r\nThis parameter is optional because, for some resources like compute, the period is irrelevant."]
    #[serde(rename = "quotaPeriod", default, skip_serializing_if = "Option::is_none")]
    pub quota_period: Option<String>,
    #[doc = "States if quota can be requested for this resource."]
    #[serde(rename = "isQuotaApplicable", default, skip_serializing_if = "Option::is_none")]
    pub is_quota_applicable: Option<bool>,
    #[doc = "Additional properties for the specific resource provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl QuotaRequestOneResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource providers list."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProvidersList {
    #[doc = "Resource provider information."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceProviderInformation>,
}
impl ResourceProvidersList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource query details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceQueryDetails {
    #[doc = "Resource query types. For extensibility, it is a string."]
    #[serde(rename = "resourceQueryType", default, skip_serializing_if = "Option::is_none")]
    pub resource_query_type: Option<ResourceQueryType>,
    #[doc = "The resource query method."]
    #[serde(rename = "resourceQueryMethod", default, skip_serializing_if = "Option::is_none")]
    pub resource_query_method: Option<ResourceQueryMethod>,
    #[doc = "Base URI for for resource query."]
    #[serde(rename = "resourceQueryUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_query_uri: Option<String>,
    #[doc = "Template to create the resource query."]
    #[serde(rename = "resourceQueryPostTemplate", default, skip_serializing_if = "Option::is_none")]
    pub resource_query_post_template: Option<String>,
}
impl ResourceQueryDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
