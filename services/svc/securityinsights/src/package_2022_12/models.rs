#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Detailed information about the errors from the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorData {
    #[doc = "Server defined code for the error"]
    pub code: String,
    #[doc = "Error message"]
    pub message: String,
}
impl ErrorData {
    pub fn new(code: String, message: String) -> Self {
        Self { code, message }
    }
}
#[doc = "The definition of an error object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponseBody {
    #[doc = "Detailed information about the errors from the operation."]
    pub error: ErrorData,
}
impl ErrorResponseBody {
    pub fn new(error: ErrorData) -> Self {
        Self { error }
    }
}
#[doc = "Object that stores a list of errors encountered when executing the Upload Indicators operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IndicatorValidationError {
    #[doc = "Index of the indicator in indicators array from request."]
    #[serde(rename = "recordIndex", default, skip_serializing_if = "Option::is_none")]
    pub record_index: Option<i64>,
    #[doc = "List of validation errors for a single indicator."]
    #[serde(
        rename = "validationErrorMessages",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub validation_error_messages: Vec<String>,
}
impl IndicatorValidationError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An object that stores a list of JSON properties from a given JObject."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JsonPropertyBag {}
impl JsonPropertyBag {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schema for request body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UploadIndicatorsRequest {
    #[doc = "Source of the indicators to be uploaded. Source system name cannot be Microsoft Sentinel."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sourcesystem: Option<String>,
    #[doc = "The indicators param is an array of STIX indictors. Indicators need to be in STIX format (STIX Indicator documentation). Indicators need to contain the “ID” field. We only support Stix 2.0 and 2.1 format."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub indicators: Vec<JsonPropertyBag>,
}
impl UploadIndicatorsRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response object containing more details about the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UploadIndicatorsResponse {
    #[doc = "Details of the error. Contains a list of indicator validation errors"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<IndicatorValidationError>,
}
impl UploadIndicatorsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
