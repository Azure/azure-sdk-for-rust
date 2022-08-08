#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
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
    #[serde(rename = "dimNames", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(rename = "dimValues", default, skip_serializing_if = "Vec::is_empty")]
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
