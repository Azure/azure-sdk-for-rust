#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "this is the management partner operations error"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "this is the extended error info"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ExtendedErrorInfo>,
    #[doc = "this is the error response code that management partner operations may return"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<ErrorResponseCode>,
    #[doc = "this is the extended error info message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl azure_core::Continuable for Error {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ErrorResponseCode = String;
#[doc = "this is the extended error info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedErrorInfo {
    #[doc = "this is the error response code that management partner operations may return"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<ErrorResponseCode>,
    #[doc = "this is the extended error info message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ExtendedErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "this is the management partner state: Active or Deleted"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagementPartnerState")]
pub enum ManagementPartnerState {
    Active,
    Deleted,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagementPartnerState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagementPartnerState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagementPartnerState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Active => serializer.serialize_unit_variant("ManagementPartnerState", 0u32, "Active"),
            Self::Deleted => serializer.serialize_unit_variant("ManagementPartnerState", 1u32, "Deleted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "this is the management partner operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "the is management partner provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "the is management partner resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "the is management partner operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "the is management partner operation description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "this is the management partner operations list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[doc = "this is the operation response list"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationResponse>,
    #[doc = "Url to get the next page of items."]
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
#[doc = "this is the management partner operations response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResponse {
    #[doc = "this is the operation response name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "this is the management partner operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "the is operation response origin information"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl OperationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "this is the management partner properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartnerProperties {
    #[doc = "This is the partner id"]
    #[serde(rename = "partnerId", default, skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    #[doc = "This is the partner name"]
    #[serde(rename = "partnerName", default, skip_serializing_if = "Option::is_none")]
    pub partner_name: Option<String>,
    #[doc = "This is the tenant id."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "This is the object id."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "This is the version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[doc = "This is the DateTime when the partner was updated."]
    #[serde(rename = "updatedTime", default, with = "azure_core::date::rfc3339::option")]
    pub updated_time: Option<time::OffsetDateTime>,
    #[doc = "This is the DateTime when the partner was created."]
    #[serde(rename = "createdTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "this is the management partner state: Active or Deleted"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ManagementPartnerState>,
}
impl PartnerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "this is the management partner operations response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartnerResponse {
    #[doc = "Type of the partner"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<i64>,
    #[doc = "Identifier of the partner"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the partner"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "this is the management partner properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PartnerProperties>,
    #[doc = "Type of resource. \"Microsoft.ManagementPartner/partners\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl PartnerResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
