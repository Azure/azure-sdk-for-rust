#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CommonResourceProperties {
    #[doc = "Resource ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type. Example: \"Microsoft.Quota/quotas\""]
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
    #[doc = "The resource ID."]
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
#[doc = "Resource usage."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CurrentUsagesBase {
    #[doc = "The resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Usage properties for the specified resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UsagesProperties>,
}
impl CurrentUsagesBase {
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
#[doc = "LimitJson abstract class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LimitJsonObject {
    #[doc = "The limit object type."]
    #[serde(rename = "limitObjectType")]
    pub limit_object_type: LimitObjectTypes,
}
impl LimitJsonObject {
    pub fn new(limit_object_type: LimitObjectTypes) -> Self {
        Self { limit_object_type }
    }
}
#[doc = "The resource quota limit value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LimitObject {
    #[serde(flatten)]
    pub limit_json_object: LimitJsonObject,
    #[doc = "The quota/limit value"]
    pub value: i32,
    #[doc = "The quota or usages limit types."]
    #[serde(rename = "limitType", default, skip_serializing_if = "Option::is_none")]
    pub limit_type: Option<LimitTypes>,
}
impl LimitObject {
    pub fn new(limit_json_object: LimitJsonObject, value: i32) -> Self {
        Self {
            limit_json_object,
            value,
            limit_type: None,
        }
    }
}
#[doc = "The limit object type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LimitObjectTypes")]
pub enum LimitObjectTypes {
    LimitValue,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LimitObjectTypes {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LimitObjectTypes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LimitObjectTypes {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::LimitValue => serializer.serialize_unit_variant("LimitObjectTypes", 0u32, "LimitValue"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The quota or usages limit types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LimitTypes")]
pub enum LimitTypes {
    Independent,
    Shared,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LimitTypes {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LimitTypes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LimitTypes {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Independent => serializer.serialize_unit_variant("LimitTypes", 0u32, "Independent"),
            Self::Shared => serializer.serialize_unit_variant("LimitTypes", 1u32, "Shared"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
    #[doc = "The URI used to fetch the next page of quota limits. When there are no more pages, this string is null."]
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
    #[doc = "LimitJson abstract class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<LimitJsonObject>,
    #[doc = " The quota units, such as Count and Bytes. When requesting quota, use the **unit** value returned in the GET response in the request body of your PUT operation."]
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
    #[doc = "Resource type. \"Microsoft.Quota/quotas\"."]
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
    #[doc = "The URI for fetching the next page of quota limits. When there are no more pages, this string is null."]
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
    #[doc = "Error details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ServiceErrorDetail>,
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
    #[doc = "The resource quota limit value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<LimitObject>,
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
    #[doc = "Resource type. \"Microsoft.Quota/quotas\"."]
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
pub type ResourceTypesName = String;
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
    #[doc = "Name of the resource provided by the resource Provider. When requesting quota, use this property name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ResourceName>,
    #[doc = "Resource type for which the quota properties were requested."]
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
    #[doc = "LimitJson abstract class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<LimitJsonObject>,
}
impl SubRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Quota limits."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsagesLimits {
    #[doc = "List of quota limits."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CurrentUsagesBase>,
    #[doc = "The URI used to fetch the next page of quota limits. When there are no more pages, this is null."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UsagesLimits {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl UsagesLimits {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource usages value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsagesObject {
    #[doc = "The usages value."]
    pub value: i32,
    #[doc = "The quota or usages limit types."]
    #[serde(rename = "usagesType", default, skip_serializing_if = "Option::is_none")]
    pub usages_type: Option<UsagesTypes>,
}
impl UsagesObject {
    pub fn new(value: i32) -> Self {
        Self { value, usages_type: None }
    }
}
#[doc = "Usage properties for the specified resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsagesProperties {
    #[doc = "The resource usages value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usages: Option<UsagesObject>,
    #[doc = " The units for the quota usage, such as Count and Bytes. When requesting quota, use the **unit** value returned in the GET response in the request body of your PUT operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Name of the resource provided by the resource Provider. When requesting quota, use this property name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ResourceName>,
    #[doc = "Resource types. For extensibility, it is a string."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<ResourceTypesName>,
    #[doc = "The time period for the summary of the quota usage values. For example:\r\n*P1D (per one day)\n*PT1M (per one minute)\n*PT1S (per one second).\r\nThis parameter is optional because it is not relevant for all resources such as compute."]
    #[serde(rename = "quotaPeriod", default, skip_serializing_if = "Option::is_none")]
    pub quota_period: Option<String>,
    #[doc = "States if quota can be requested for this resource."]
    #[serde(rename = "isQuotaApplicable", default, skip_serializing_if = "Option::is_none")]
    pub is_quota_applicable: Option<bool>,
    #[doc = "Additional properties for the specific resource provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl UsagesProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The quota or usages limit types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UsagesTypes")]
pub enum UsagesTypes {
    Individual,
    Combined,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UsagesTypes {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UsagesTypes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UsagesTypes {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Individual => serializer.serialize_unit_variant("UsagesTypes", 0u32, "Individual"),
            Self::Combined => serializer.serialize_unit_variant("UsagesTypes", 1u32, "Combined"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
    #[doc = "The resource quota limit value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<LimitObject>,
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
    #[doc = "Error details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ServiceErrorDetail>,
    #[doc = "Additional properties for the specific resource provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl QuotaRequestOneResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
