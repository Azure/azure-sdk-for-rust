#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "This object is returned when an error occurs in the Maps API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "If available, a human readable description of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "If available, the component generating the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "If available, a list of additional details about the error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<serde_json::Value>,
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
#[doc = "An Azure resource which represents access to a suite of Maps REST APIs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MapsAccount {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Gets a list of key value pairs that describe the resource. These tags can be used in viewing and grouping this resource (across resource groups). A maximum of 15 tags can be provided for a resource. Each tag must have a key no greater than 128 characters and value no greater than 256 characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The SKU of the Maps Account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Additional Map account properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MapsAccountProperties>,
}
impl MapsAccount {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters used to create a new Maps Account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapsAccountCreateParameters {
    #[doc = "The location of the resource."]
    pub location: String,
    #[doc = "Gets or sets a list of key value pairs that describe the resource. These tags can be used in viewing and grouping this resource (across resource groups). A maximum of 15 tags can be provided for a resource. Each tag must have a key no greater than 128 characters and value no greater than 256 characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The SKU of the Maps Account."]
    pub sku: Sku,
}
impl MapsAccountCreateParameters {
    pub fn new(location: String, sku: Sku) -> Self {
        Self { location, tags: None, sku }
    }
}
#[doc = "The set of keys which can be used to access the Maps REST APIs. Two keys are provided for key rotation without interruption."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MapsAccountKeys {
    #[doc = "The full Azure resource identifier of the Maps Account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The primary key for accessing the Maps REST APIs."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "The secondary key for accessing the Maps REST APIs."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
}
impl MapsAccountKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Additional Map account properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MapsAccountProperties {
    #[doc = "A unique identifier for the maps account"]
    #[serde(rename = "x-ms-client-id", default, skip_serializing_if = "Option::is_none")]
    pub x_ms_client_id: Option<String>,
}
impl MapsAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters used to update an existing Maps Account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MapsAccountUpdateParameters {
    #[doc = "Gets or sets a list of key value pairs that describe the resource. These tags can be used in viewing and grouping this resource (across resource groups). A maximum of 15 tags can be provided for a resource. Each tag must have a key no greater than 128 characters and value no greater than 256 characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The SKU of the Maps Account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl MapsAccountUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of Maps Accounts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MapsAccounts {
    #[doc = "a Maps Account."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MapsAccount>,
}
impl azure_core::Continuable for MapsAccounts {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MapsAccounts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The description of what resources to move between resource groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapsAccountsMoveRequest {
    #[doc = "The name of the destination resource group."]
    #[serde(rename = "targetResourceGroup")]
    pub target_resource_group: String,
    #[doc = "A list of resource names to move from the source resource group."]
    #[serde(rename = "resourceIds")]
    pub resource_ids: Vec<String>,
}
impl MapsAccountsMoveRequest {
    pub fn new(target_resource_group: String, resource_ids: Vec<String>) -> Self {
        Self {
            target_resource_group,
            resource_ids,
        }
    }
}
#[doc = "Whether the operation refers to the primary or secondary key."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapsKeySpecification {
    #[doc = "Whether the operation refers to the primary or secondary key."]
    #[serde(rename = "keyType")]
    pub key_type: maps_key_specification::KeyType,
}
impl MapsKeySpecification {
    pub fn new(key_type: maps_key_specification::KeyType) -> Self {
        Self { key_type }
    }
}
pub mod maps_key_specification {
    use super::*;
    #[doc = "Whether the operation refers to the primary or secondary key."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "KeyType")]
    pub enum KeyType {
        #[serde(rename = "primary")]
        Primary,
        #[serde(rename = "secondary")]
        Secondary,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for KeyType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for KeyType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for KeyType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Primary => serializer.serialize_unit_variant("KeyType", 0u32, "primary"),
                Self::Secondary => serializer.serialize_unit_variant("KeyType", 1u32, "secondary"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The set of operations available for Maps."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MapsOperations {
    #[doc = "An operation available for Maps."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<serde_json::Value>,
}
impl azure_core::Continuable for MapsOperations {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MapsOperations {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "The fully qualified Maps Account resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the Maps Account, which is unique within a Resource Group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SKU of the Maps Account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The name of the SKU, in standard format (such as S0)."]
    pub name: String,
    #[doc = "Gets the sku tier. This is based on the SKU name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
impl Sku {
    pub fn new(name: String) -> Self {
        Self { name, tier: None }
    }
}
