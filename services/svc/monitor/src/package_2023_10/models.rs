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
#[doc = "The localizable string class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalizableString {
    #[doc = "The invariant value."]
    pub value: String,
    #[doc = "The display name."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl LocalizableString {
    pub fn new(value: String) -> Self {
        Self {
            value,
            localized_value: None,
        }
    }
}
#[doc = "Represents a metric metadata value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataValue {
    #[doc = "The localizable string class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<LocalizableString>,
    #[doc = "The value of the metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl MetadataValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result data of a query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metric {
    #[doc = "The metric Id."]
    pub id: String,
    #[doc = "The resource type of the metric resource."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The localizable string class."]
    pub name: LocalizableString,
    #[doc = "Detailed description of this metric."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "'Success' or the error details on query failures for this metric."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "Error message encountered querying this specific metric."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[doc = "The unit of the metric."]
    pub unit: Unit,
    #[doc = "The time series returned when a data query is performed."]
    pub timeseries: Vec<TimeSeriesElement>,
}
impl Metric {
    pub fn new(id: String, type_: String, name: LocalizableString, unit: Unit, timeseries: Vec<TimeSeriesElement>) -> Self {
        Self {
            id,
            type_,
            name,
            display_description: None,
            error_code: None,
            error_message: None,
            unit,
            timeseries,
        }
    }
}
#[doc = "The metrics result for a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricResultsResponse {
    #[doc = "The collection of metric data responses per resource, per metric."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<serde_json::Value>,
}
impl MetricResultsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a metric value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricValue {
    #[doc = "The timestamp for the metric value in ISO 8601 format."]
    #[serde(rename = "timeStamp", with = "azure_core::date::rfc3339")]
    pub time_stamp: time::OffsetDateTime,
    #[doc = "The average value in the time range."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average: Option<f64>,
    #[doc = "The least value in the time range."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[doc = "The greatest value in the time range."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[doc = "The sum of all of the values in the time range."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[doc = "The number of samples in the time range. Can be used to determine the number of values that contributed to the average value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,
}
impl MetricValue {
    pub fn new(time_stamp: time::OffsetDateTime) -> Self {
        Self {
            time_stamp,
            average: None,
            minimum: None,
            maximum: None,
            total: None,
            count: None,
        }
    }
}
#[doc = "The comma separated list of resource IDs to query metrics for."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceIdList {
    #[doc = "The list of resource IDs to query metrics for."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub resourceids: Vec<String>,
}
impl ResourceIdList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A time series result type. The discriminator value is always TimeSeries in this case."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeSeriesElement {
    #[doc = "The metadata values returned if $filter was specified in the call."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metadatavalues: Vec<MetadataValue>,
    #[doc = "An array of data points representing the metric values.  This is only returned if a result type of data is specified."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data: Vec<MetricValue>,
}
impl TimeSeriesElement {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The unit of the metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Unit")]
pub enum Unit {
    Count,
    Bytes,
    Seconds,
    CountPerSecond,
    BytesPerSecond,
    Percent,
    MilliSeconds,
    ByteSeconds,
    Unspecified,
    Cores,
    MilliCores,
    NanoCores,
    BitsPerSecond,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Unit {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Unit {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Unit {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Count => serializer.serialize_unit_variant("Unit", 0u32, "Count"),
            Self::Bytes => serializer.serialize_unit_variant("Unit", 1u32, "Bytes"),
            Self::Seconds => serializer.serialize_unit_variant("Unit", 2u32, "Seconds"),
            Self::CountPerSecond => serializer.serialize_unit_variant("Unit", 3u32, "CountPerSecond"),
            Self::BytesPerSecond => serializer.serialize_unit_variant("Unit", 4u32, "BytesPerSecond"),
            Self::Percent => serializer.serialize_unit_variant("Unit", 5u32, "Percent"),
            Self::MilliSeconds => serializer.serialize_unit_variant("Unit", 6u32, "MilliSeconds"),
            Self::ByteSeconds => serializer.serialize_unit_variant("Unit", 7u32, "ByteSeconds"),
            Self::Unspecified => serializer.serialize_unit_variant("Unit", 8u32, "Unspecified"),
            Self::Cores => serializer.serialize_unit_variant("Unit", 9u32, "Cores"),
            Self::MilliCores => serializer.serialize_unit_variant("Unit", 10u32, "MilliCores"),
            Self::NanoCores => serializer.serialize_unit_variant("Unit", 11u32, "NanoCores"),
            Self::BitsPerSecond => serializer.serialize_unit_variant("Unit", 12u32, "BitsPerSecond"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
