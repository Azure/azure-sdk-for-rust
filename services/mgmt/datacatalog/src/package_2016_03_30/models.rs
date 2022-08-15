#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Azure Data Catalog."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdcCatalog {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the data catalog."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdcCatalogProperties>,
}
impl AdcCatalog {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the data catalog."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdcCatalogProperties {
    #[doc = "Azure data catalog SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<adc_catalog_properties::Sku>,
    #[doc = "Azure data catalog units."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub units: Option<i64>,
    #[doc = "Azure data catalog admin list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub admins: Vec<Principals>,
    #[doc = "Azure data catalog user list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<Principals>,
    #[doc = "Azure data catalog provision status."]
    #[serde(rename = "successfullyProvisioned", default, skip_serializing_if = "Option::is_none")]
    pub successfully_provisioned: Option<bool>,
    #[doc = "Automatic unit adjustment enabled or not."]
    #[serde(rename = "enableAutomaticUnitAdjustment", default, skip_serializing_if = "Option::is_none")]
    pub enable_automatic_unit_adjustment: Option<bool>,
}
impl AdcCatalogProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod adc_catalog_properties {
    use super::*;
    #[doc = "Azure data catalog SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Sku")]
    pub enum Sku {
        Free,
        Standard,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Sku {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Sku {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Sku {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Free => serializer.serialize_unit_variant("Sku", 0u32, "Free"),
                Self::Standard => serializer.serialize_unit_variant("Sku", 1u32, "Standard"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The response from the List Azure Data Catalog operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdcCatalogsListResult {
    #[doc = "the list of Azure Data Catalogs."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AdcCatalog>,
}
impl AdcCatalogsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The operation supported by Azure Data Catalog Service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplayInfo {
    #[doc = "The description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The action that users can perform, based on their permission level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Service provider: Azure Data Catalog Service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource on which the operation is performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
impl OperationDisplayInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The operation supported by Azure Data Catalog Service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationEntity {
    #[doc = "Operation name: {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The operation supported by Azure Data Catalog Service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayInfo>,
}
impl OperationEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of Azure data catalog service operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationEntityListResult {
    #[doc = "The list of operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationEntity>,
}
impl OperationEntityListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User principals."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Principals {
    #[doc = "UPN of the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upn: Option<String>,
    #[doc = "Object Id for the user"]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
}
impl Principals {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Resource model definition."]
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
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Resource etag"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
