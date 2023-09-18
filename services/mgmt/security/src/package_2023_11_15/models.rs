#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An API collection as represented by Microsoft Defender for APIs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiCollection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Describes the properties of an API collection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApiCollectionProperties>,
}
impl ApiCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Page of a list of API collections as represented by Microsoft Defender for APIs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiCollectionList {
    #[doc = "API collections in this page."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ApiCollection>,
    #[doc = "The URI to fetch the next page."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApiCollectionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ApiCollectionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of an API collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiCollectionProperties {
    #[doc = "Gets the provisioning state of the API collection."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<api_collection_properties::ProvisioningState>,
    #[doc = "The display name of the API collection."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The resource Id of the resource from where this API collection was discovered."]
    #[serde(rename = "discoveredVia", default, skip_serializing_if = "Option::is_none")]
    pub discovered_via: Option<String>,
    #[doc = "The base URI for this API collection. All endpoints of this API collection extend this base URI."]
    #[serde(rename = "baseUrl", default, skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[doc = "The number of API endpoints discovered in this API collection."]
    #[serde(rename = "numberOfApiEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub number_of_api_endpoints: Option<i64>,
    #[doc = "The number of API endpoints in this API collection that have not received any API traffic in the last 30 days."]
    #[serde(rename = "numberOfInactiveApiEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub number_of_inactive_api_endpoints: Option<i64>,
    #[doc = "The number of API endpoints in this API collection that are unauthenticated."]
    #[serde(rename = "numberOfUnauthenticatedApiEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub number_of_unauthenticated_api_endpoints: Option<i64>,
    #[doc = "The number of API endpoints in this API collection for which API traffic from the internet was observed."]
    #[serde(rename = "numberOfExternalApiEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub number_of_external_api_endpoints: Option<i64>,
    #[doc = "The number of API endpoints in this API collection which are exposing sensitive data in their requests and/or responses."]
    #[serde(
        rename = "numberOfApiEndpointsWithSensitiveDataExposed",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_api_endpoints_with_sensitive_data_exposed: Option<i64>,
    #[doc = "The highest priority sensitivity label from Microsoft Purview in this API collection."]
    #[serde(rename = "sensitivityLabel", default, skip_serializing_if = "Option::is_none")]
    pub sensitivity_label: Option<String>,
}
impl ApiCollectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod api_collection_properties {
    use super::*;
    #[doc = "Gets the provisioning state of the API collection."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Canceled,
        InProgress,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
                Self::InProgress => serializer.serialize_unit_variant("ProvisioningState", 3u32, "InProgress"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
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
#[doc = "Describes an Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
