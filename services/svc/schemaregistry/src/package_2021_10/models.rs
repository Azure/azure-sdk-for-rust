#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An error response returned from Azure Schema Registry service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[doc = "Error response returned from Azure Schema Registry service."]
    pub error: ErrorDetail,
}
impl Error {
    pub fn new(error: ErrorDetail) -> Self {
        Self { error }
    }
}
#[doc = "Error response returned from Azure Schema Registry service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[doc = "Type of error."]
    pub code: String,
    #[doc = "Brief description of error."]
    pub message: String,
    #[doc = "Error message details to help user understand/debug failure."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
}
impl ErrorDetail {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            details: Vec::new(),
        }
    }
}
pub type SchemaGroup = String;
#[doc = "Array received from the registry containing the list of schema groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SchemaGroups {
    #[doc = "Array of schema groups."]
    #[serde(rename = "schemaGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub schema_groups: Vec<SchemaGroup>,
}
impl SchemaGroups {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object received from the registry containing schema identifiers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SchemaId {
    #[doc = "Schema ID that uniquely identifies a schema in the registry namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SchemaId {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type SchemaVersion = i64;
#[doc = "Array received from the registry containing the list of versions for specific schema."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SchemaVersions {
    #[doc = "Array of schema groups."]
    #[serde(rename = "schemaVersions", default, skip_serializing_if = "Vec::is_empty")]
    pub schema_versions: Vec<SchemaVersion>,
}
impl SchemaVersions {
    pub fn new() -> Self {
        Self::default()
    }
}
