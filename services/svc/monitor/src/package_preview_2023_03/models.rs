#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The response to a metrics query that results in a bad request, with optional additional information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdditionalInfoErrorResponse {
    #[doc = "Top level error object that contains all relevant information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<additional_info_error_response::Error>,
}
impl AdditionalInfoErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod additional_info_error_response {
    use super::*;
    #[doc = "Top level error object that contains all relevant information."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "Error message indicating why the operation failed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiError {
    #[doc = "Gets or sets the azure metrics error code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Gets or sets the azure metrics error message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ApiError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiFailureResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
}
impl ApiFailureResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMetricsBaseData {
    #[doc = "Gets or sets the Metric name"]
    pub metric: String,
    #[doc = "Gets or sets the Metric namespace"]
    pub namespace: String,
    #[doc = "Gets or sets the list of dimension names (optional)"]
    #[serde(
        rename = "dimNames",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dim_names: Vec<String>,
    #[doc = "Gets or sets the list of time series data for the metric (one per unique dimension combination)"]
    pub series: Vec<AzureTimeSeriesData>,
}
impl AzureMetricsBaseData {
    pub fn new(metric: String, namespace: String, series: Vec<AzureTimeSeriesData>) -> Self {
        Self {
            metric,
            namespace,
            dim_names: Vec::new(),
            series,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMetricsData {
    #[serde(rename = "baseData")]
    pub base_data: AzureMetricsBaseData,
}
impl AzureMetricsData {
    pub fn new(base_data: AzureMetricsBaseData) -> Self {
        Self { base_data }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureMetricsDocument {
    #[doc = "Gets or sets Time property (in ISO 8601 format)"]
    pub time: String,
    pub data: AzureMetricsData,
}
impl AzureMetricsDocument {
    pub fn new(time: String, data: AzureMetricsData) -> Self {
        Self { time, data }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMetricsResult {
    #[doc = "Http status code response "]
    #[serde(rename = "statusCode", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
    #[serde(rename = "apiFailureResponse", default, skip_serializing_if = "Option::is_none")]
    pub api_failure_response: Option<ApiFailureResponse>,
}
impl AzureMetricsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureTimeSeriesData {
    #[doc = "Gets or sets dimension values"]
    #[serde(
        rename = "dimValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dim_values: Vec<String>,
    #[doc = "Gets or sets Min value"]
    pub min: f64,
    #[doc = "Gets or sets Max value"]
    pub max: f64,
    #[doc = "Gets or sets Sum value"]
    pub sum: f64,
    #[doc = "Gets or sets Count value"]
    pub count: i32,
}
impl AzureTimeSeriesData {
    pub fn new(min: f64, max: f64, sum: f64, count: i32) -> Self {
        Self {
            dim_values: Vec::new(),
            min,
            max,
            sum,
            count,
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
