#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The response to a metrics query that results in a bad request, with optional additional information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdditionalInfoErrorResponse {
    #[doc = "Top level error object that contains all relevant information."]
    pub error: additional_info_error_response::Error,
}
impl AdditionalInfoErrorResponse {
    pub fn new(error: additional_info_error_response::Error) -> Self {
        Self { error }
    }
}
pub mod additional_info_error_response {
    use super::*;
    #[doc = "Top level error object that contains all relevant information."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Error {
        #[doc = "Additional information about the error"]
        #[serde(
            rename = "additionalInfo",
            default,
            deserialize_with = "azure_core::util::deserialize_null_as_default",
            skip_serializing_if = "Vec::is_empty"
        )]
        pub additional_info: Vec<serde_json::Value>,
        #[doc = "Error code"]
        pub code: String,
        #[doc = "Error message indicating why the operation failed."]
        pub message: String,
    }
    impl Error {
        pub fn new(code: String, message: String) -> Self {
            Self {
                additional_info: Vec::new(),
                code,
                message,
            }
        }
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
    #[doc = "The localizable string class."]
    pub name: LocalizableString,
    #[doc = "Description of this metric"]
    #[serde(rename = "displayDescription")]
    pub display_description: String,
    #[doc = "The resource type of the metric resource."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The unit of the metric."]
    pub unit: MetricUnit,
    #[doc = "The time series returned when a data query is performed."]
    pub timeseries: Vec<TimeSeriesElement>,
    #[doc = "'Success' or the error details on query failures for this metric."]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[doc = "Error message encountered querying this specific metric."]
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
impl Metric {
    pub fn new(
        id: String,
        name: LocalizableString,
        display_description: String,
        type_: String,
        unit: MetricUnit,
        timeseries: Vec<TimeSeriesElement>,
    ) -> Self {
        Self {
            id,
            name,
            display_description,
            type_,
            unit,
            timeseries,
            error_code: None,
            error_message: None,
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
#[doc = "The unit of the metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MetricUnit {
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
    #[doc = "The metadata values returned if filter was specified in the call."]
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
