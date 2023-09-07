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
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The contents of the file to compile."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PortalTenantCompileFile {
    #[doc = "The contents of the file."]
    pub contents: serde_json::Value,
    #[doc = "The contents of the string source."]
    #[serde(rename = "stringSource", default, skip_serializing_if = "Option::is_none")]
    pub string_source: Option<serde_json::Value>,
    #[doc = "The contents of referenced files. The property name is the relative file path and the value is its contents."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<serde_json::Value>,
}
impl PortalTenantCompileFile {
    pub fn new(contents: serde_json::Value) -> Self {
        Self {
            contents,
            string_source: None,
            files: None,
        }
    }
}
#[doc = "The runtime result of source compilation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PortalTenantCompileFileResult {}
impl PortalTenantCompileFileResult {
    pub fn new() -> Self {
        Self::default()
    }
}
