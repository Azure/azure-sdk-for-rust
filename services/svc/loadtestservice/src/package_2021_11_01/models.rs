#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An Azure resource object (Refer azure generic resource model : https://docs.microsoft.com/en-us/rest/api/resources/resources/get-by-id#genericresource)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppComponent {
    #[doc = "fully qualified resource Id e.g subscriptions/{subId}/resourceGroups/{rg}/providers/Microsoft.LoadTestService/loadtests/{resName}"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Azure resource name, required while creating the app component."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "Azure resource type, required while creating the app component."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Azure resource display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Resource group name of the Azure resource"]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "Subscription Id of the Azure resource"]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Kind of Azure resource type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
impl AppComponent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Certificates metadata"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateMetadata {
    #[doc = "The value of the certificate for respective type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Type of certificate"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<certificate_metadata::Type>,
    #[doc = "Name of the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl CertificateMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod certificate_metadata {
    use super::*;
    #[doc = "Type of certificate"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "AKV_CERT_URI")]
        AkvCertUri,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Type {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AkvCertUri => serializer.serialize_unit_variant("Type", 0u32, "AKV_CERT_URI"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Dimension name and values to filter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DimensionFilter {
    #[doc = "The dimension name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The dimension values. Maximum values can be 20."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub values: Vec<String>,
}
impl DimensionFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a metric dimension value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DimensionValue {
    #[doc = "The name of the dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of the dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl DimensionValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metrics dimension values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DimensionValueList {
    #[doc = "The dimension values"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<String>,
    #[doc = "Link for the next set of values in case of paginated results, if applicable"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DimensionValueList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DimensionValueList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error from a REST request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[doc = "The error code."]
    pub code: String,
    #[doc = "The error message."]
    pub message: String,
    #[doc = "The error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Additional details and inner errors."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<Error>,
}
impl Error {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            details: Vec::new(),
        }
    }
}
#[doc = "Error details if there is any failure in load test run"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[doc = "Error details in case test run was not successfully run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The definition of an error object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponseBody {
    #[doc = "Error from a REST request."]
    pub error: Error,
}
impl azure_core::Continuable for ErrorResponseBody {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponseBody {
    pub fn new(error: Error) -> Self {
        Self { error }
    }
}
#[doc = "File info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileInfo {
    #[doc = "File URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Name of the file."]
    #[serde(rename = "fileName", default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[doc = "File type"]
    #[serde(rename = "fileType", default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<file_info::FileType>,
    #[doc = "Expiry time of the file (ISO 8601 literal format)"]
    #[serde(rename = "expireDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub expire_date_time: Option<time::OffsetDateTime>,
    #[doc = "Validation status of the file"]
    #[serde(rename = "validationStatus", default, skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<file_info::ValidationStatus>,
    #[doc = "Validation failure error details"]
    #[serde(rename = "validationFailureDetails", default, skip_serializing_if = "Option::is_none")]
    pub validation_failure_details: Option<String>,
}
impl FileInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod file_info {
    use super::*;
    #[doc = "File type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "FileType")]
    pub enum FileType {
        #[serde(rename = "JMX_FILE")]
        JmxFile,
        #[serde(rename = "USER_PROPERTIES")]
        UserProperties,
        #[serde(rename = "ADDITIONAL_ARTIFACTS")]
        AdditionalArtifacts,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for FileType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for FileType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for FileType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::JmxFile => serializer.serialize_unit_variant("FileType", 0u32, "JMX_FILE"),
                Self::UserProperties => serializer.serialize_unit_variant("FileType", 1u32, "USER_PROPERTIES"),
                Self::AdditionalArtifacts => serializer.serialize_unit_variant("FileType", 2u32, "ADDITIONAL_ARTIFACTS"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Validation status of the file"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ValidationStatus")]
    pub enum ValidationStatus {
        #[serde(rename = "NOT_VALIDATED")]
        NotValidated,
        #[serde(rename = "VALIDATION_SUCCESS")]
        ValidationSuccess,
        #[serde(rename = "VALIDATION_FAILURE")]
        ValidationFailure,
        #[serde(rename = "VALIDATION_INITIATED")]
        ValidationInitiated,
        #[serde(rename = "VALIDATION_NOT_REQUIRED")]
        ValidationNotRequired,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ValidationStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ValidationStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ValidationStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotValidated => serializer.serialize_unit_variant("ValidationStatus", 0u32, "NOT_VALIDATED"),
                Self::ValidationSuccess => serializer.serialize_unit_variant("ValidationStatus", 1u32, "VALIDATION_SUCCESS"),
                Self::ValidationFailure => serializer.serialize_unit_variant("ValidationStatus", 2u32, "VALIDATION_FAILURE"),
                Self::ValidationInitiated => serializer.serialize_unit_variant("ValidationStatus", 3u32, "VALIDATION_INITIATED"),
                Self::ValidationNotRequired => serializer.serialize_unit_variant("ValidationStatus", 4u32, "VALIDATION_NOT_REQUIRED"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Collection of files."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileInfoList {
    #[doc = "List of file info."]
    pub value: Vec<FileInfo>,
    #[doc = "Link for the next list of file URLs, if applicable"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FileInfoList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl FileInfoList {
    pub fn new(value: Vec<FileInfo>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The load test configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadTestConfiguration {
    #[doc = "The number of engine instances to execute load test. Supported values are in range of 1-45. Required for creating a new test."]
    #[serde(rename = "engineInstances", default, skip_serializing_if = "Option::is_none")]
    pub engine_instances: Option<i32>,
    #[doc = "If false, Azure Load Testing copies and processes your input files unmodified across all test engine instances. If true, Azure Load Testing splits the CSV input data evenly across all engine instances. If you provide multiple CSV files, each file will be split evenly."]
    #[serde(rename = "splitAllCSVs", default, skip_serializing_if = "Option::is_none")]
    pub split_all_cs_vs: Option<bool>,
    #[doc = "If true, optionalLoadTestConfig is required and JMX script for the load test is not required to upload."]
    #[serde(rename = "quickStartTest", default, skip_serializing_if = "Option::is_none")]
    pub quick_start_test: Option<bool>,
    #[doc = "Optional load test config"]
    #[serde(rename = "optionalLoadTestConfig", default, skip_serializing_if = "Option::is_none")]
    pub optional_load_test_config: Option<OptionalLoadTestConfig>,
}
impl LoadTestConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metric availability specifies the time grain (aggregation interval or frequency)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAvailability {
    #[doc = "The time grain specifies the aggregation interval for the metric. Expressed as a duration 'PT1M', 'PT1H', etc."]
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<metric_availability::TimeGrain>,
}
impl MetricAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod metric_availability {
    use super::*;
    #[doc = "The time grain specifies the aggregation interval for the metric. Expressed as a duration 'PT1M', 'PT1H', etc."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TimeGrain")]
    pub enum TimeGrain {
        #[serde(rename = "PT5S")]
        Pt5s,
        #[serde(rename = "PT10S")]
        Pt10s,
        #[serde(rename = "PT1M")]
        Pt1m,
        #[serde(rename = "PT5M")]
        Pt5m,
        #[serde(rename = "PT1H")]
        Pt1h,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TimeGrain {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TimeGrain {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TimeGrain {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Pt5s => serializer.serialize_unit_variant("TimeGrain", 0u32, "PT5S"),
                Self::Pt10s => serializer.serialize_unit_variant("TimeGrain", 1u32, "PT10S"),
                Self::Pt1m => serializer.serialize_unit_variant("TimeGrain", 2u32, "PT1M"),
                Self::Pt5m => serializer.serialize_unit_variant("TimeGrain", 3u32, "PT5M"),
                Self::Pt1h => serializer.serialize_unit_variant("TimeGrain", 4u32, "PT1H"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Metric definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricDefinition {
    #[doc = "List of dimensions"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dimensions: Vec<NameAndDesc>,
    #[doc = "The metric description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The metric name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The namespace the metric belongs to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "The primary aggregation type value defining how to use the values for display."]
    #[serde(rename = "primaryAggregationType", default, skip_serializing_if = "Option::is_none")]
    pub primary_aggregation_type: Option<metric_definition::PrimaryAggregationType>,
    #[doc = "The collection of what all aggregation types are supported."]
    #[serde(
        rename = "supportedAggregationTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_aggregation_types: Vec<String>,
    #[doc = "The unit of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<metric_definition::Unit>,
    #[doc = "Metric availability specifies the time grain (aggregation interval or frequency)."]
    #[serde(
        rename = "metricAvailabilities",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metric_availabilities: Vec<MetricAvailability>,
}
impl MetricDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod metric_definition {
    use super::*;
    #[doc = "The primary aggregation type value defining how to use the values for display."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrimaryAggregationType")]
    pub enum PrimaryAggregationType {
        Average,
        Count,
        None,
        Total,
        Percentile90,
        Percentile95,
        Percentile99,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrimaryAggregationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrimaryAggregationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrimaryAggregationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Average => serializer.serialize_unit_variant("PrimaryAggregationType", 0u32, "Average"),
                Self::Count => serializer.serialize_unit_variant("PrimaryAggregationType", 1u32, "Count"),
                Self::None => serializer.serialize_unit_variant("PrimaryAggregationType", 2u32, "None"),
                Self::Total => serializer.serialize_unit_variant("PrimaryAggregationType", 3u32, "Total"),
                Self::Percentile90 => serializer.serialize_unit_variant("PrimaryAggregationType", 4u32, "Percentile90"),
                Self::Percentile95 => serializer.serialize_unit_variant("PrimaryAggregationType", 5u32, "Percentile95"),
                Self::Percentile99 => serializer.serialize_unit_variant("PrimaryAggregationType", 6u32, "Percentile99"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The unit of the metric."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        NotSpecified,
        Percent,
        Count,
        Seconds,
        Milliseconds,
        Bytes,
        BytesPerSecond,
        CountPerSecond,
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
                Self::NotSpecified => serializer.serialize_unit_variant("Unit", 0u32, "NotSpecified"),
                Self::Percent => serializer.serialize_unit_variant("Unit", 1u32, "Percent"),
                Self::Count => serializer.serialize_unit_variant("Unit", 2u32, "Count"),
                Self::Seconds => serializer.serialize_unit_variant("Unit", 3u32, "Seconds"),
                Self::Milliseconds => serializer.serialize_unit_variant("Unit", 4u32, "Milliseconds"),
                Self::Bytes => serializer.serialize_unit_variant("Unit", 5u32, "Bytes"),
                Self::BytesPerSecond => serializer.serialize_unit_variant("Unit", 6u32, "BytesPerSecond"),
                Self::CountPerSecond => serializer.serialize_unit_variant("Unit", 7u32, "CountPerSecond"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Represents collection of metric definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricDefinitionCollection {
    #[doc = "the values for the metric definitions."]
    pub value: Vec<MetricDefinition>,
}
impl MetricDefinitionCollection {
    pub fn new(value: Vec<MetricDefinition>) -> Self {
        Self { value }
    }
}
#[doc = "Metric namespace class specifies the metadata for a metric namespace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricNamespace {
    #[doc = "The namespace description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The metric namespace name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl MetricNamespace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents collection of metric namespaces."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricNamespaceCollection {
    #[doc = "The values for the metric namespaces."]
    pub value: Vec<MetricNamespace>,
}
impl MetricNamespaceCollection {
    pub fn new(value: Vec<MetricNamespace>) -> Self {
        Self { value }
    }
}
#[doc = "Filters to fetch the set of metric"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricRequestPayload {
    #[doc = "Get metrics for specific dimension values. Example: Metric contains dimension like SamplerName, Error. To retrieve all the time series data where SamplerName is equals to HTTPRequest1 or HTTPRequest2, the DimensionFilter value will be {\"SamplerName\", [\"HTTPRequest1\", \"HTTPRequest2\"}"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub filters: Vec<DimensionFilter>,
}
impl MetricRequestPayload {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a metric value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricValue {
    #[doc = "The timestamp for the metric value in ISO 8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[doc = "The metric value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
impl MetricValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a metrics query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Metrics {
    #[doc = "Timeseries data for metric query."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<TimeSeriesElement>,
    #[doc = "Link for the next set of timeseries in case of paginated results, if applicable"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for Metrics {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl Metrics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The name and description"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameAndDesc {
    #[doc = "The description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl NameAndDesc {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Optional load test config"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OptionalLoadTestConfig {
    #[doc = "Test URL. Provide the complete HTTP URL. For example, http://contoso-app.azurewebsites.net/login"]
    #[serde(rename = "endpointUrl", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    #[doc = "No of concurrent virtual users"]
    #[serde(rename = "virtualUsers", default, skip_serializing_if = "Option::is_none")]
    pub virtual_users: Option<i32>,
    #[doc = "Ramp up time"]
    #[serde(rename = "rampUpTime", default, skip_serializing_if = "Option::is_none")]
    pub ramp_up_time: Option<i32>,
    #[doc = "Test run duration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
}
impl OptionalLoadTestConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Pass fail criteria for a test."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PassFailCriteria {
    #[doc = "Map of id and pass fail metrics { id  : pass fail metrics }."]
    #[serde(rename = "passFailMetrics", default, skip_serializing_if = "Option::is_none")]
    pub pass_fail_metrics: Option<serde_json::Value>,
}
impl PassFailCriteria {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Pass fail metric"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PassFailMetric {
    #[doc = "The client metric on which the criteria should be applied."]
    #[serde(rename = "clientMetric", default, skip_serializing_if = "Option::is_none")]
    pub client_metric: Option<pass_fail_metric::ClientMetric>,
    #[doc = "The aggregation function to be applied on the client metric. Allowed functions - ‘percentage’ - for error metric , ‘avg’, ‘p50’, ‘p90’, ‘p95’, ‘p99’, ‘min’, ‘max’ - for response_time_ms and latency metric, ‘avg’ - for requests_per_sec, ‘count’ - for requests"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<pass_fail_metric::Aggregate>,
    #[doc = "The comparison operator. Supported types ‘>’, ‘<’ "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Request name for which the Pass fail criteria has to be applied "]
    #[serde(rename = "requestName", default, skip_serializing_if = "Option::is_none")]
    pub request_name: Option<String>,
    #[doc = "The value to compare with the client metric. Allowed values - ‘error : [0.0 , 100.0] unit- % ’, response_time_ms and latency : any integer value unit- ms."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[doc = "Action taken after the threshold is met. Default is ‘continue’."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<pass_fail_metric::Action>,
    #[doc = "The actual value of the client metric for the test run."]
    #[serde(rename = "actualValue", default, skip_serializing_if = "Option::is_none")]
    pub actual_value: Option<f64>,
    #[doc = "Outcome of the test run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<pass_fail_metric::Result>,
}
impl PassFailMetric {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod pass_fail_metric {
    use super::*;
    #[doc = "The client metric on which the criteria should be applied."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ClientMetric")]
    pub enum ClientMetric {
        #[serde(rename = "response_time_ms")]
        ResponseTimeMs,
        #[serde(rename = "latency")]
        Latency,
        #[serde(rename = "error")]
        Error,
        #[serde(rename = "requests")]
        Requests,
        #[serde(rename = "requests_per_sec")]
        RequestsPerSec,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ClientMetric {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ClientMetric {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ClientMetric {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ResponseTimeMs => serializer.serialize_unit_variant("ClientMetric", 0u32, "response_time_ms"),
                Self::Latency => serializer.serialize_unit_variant("ClientMetric", 1u32, "latency"),
                Self::Error => serializer.serialize_unit_variant("ClientMetric", 2u32, "error"),
                Self::Requests => serializer.serialize_unit_variant("ClientMetric", 3u32, "requests"),
                Self::RequestsPerSec => serializer.serialize_unit_variant("ClientMetric", 4u32, "requests_per_sec"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The aggregation function to be applied on the client metric. Allowed functions - ‘percentage’ - for error metric , ‘avg’, ‘p50’, ‘p90’, ‘p95’, ‘p99’, ‘min’, ‘max’ - for response_time_ms and latency metric, ‘avg’ - for requests_per_sec, ‘count’ - for requests"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Aggregate")]
    pub enum Aggregate {
        #[serde(rename = "count")]
        Count,
        #[serde(rename = "percentage")]
        Percentage,
        #[serde(rename = "avg")]
        Avg,
        #[serde(rename = "p50")]
        P50,
        #[serde(rename = "p90")]
        P90,
        #[serde(rename = "p95")]
        P95,
        #[serde(rename = "p99")]
        P99,
        #[serde(rename = "min")]
        Min,
        #[serde(rename = "max")]
        Max,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Aggregate {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Aggregate {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Aggregate {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Count => serializer.serialize_unit_variant("Aggregate", 0u32, "count"),
                Self::Percentage => serializer.serialize_unit_variant("Aggregate", 1u32, "percentage"),
                Self::Avg => serializer.serialize_unit_variant("Aggregate", 2u32, "avg"),
                Self::P50 => serializer.serialize_unit_variant("Aggregate", 3u32, "p50"),
                Self::P90 => serializer.serialize_unit_variant("Aggregate", 4u32, "p90"),
                Self::P95 => serializer.serialize_unit_variant("Aggregate", 5u32, "p95"),
                Self::P99 => serializer.serialize_unit_variant("Aggregate", 6u32, "p99"),
                Self::Min => serializer.serialize_unit_variant("Aggregate", 7u32, "min"),
                Self::Max => serializer.serialize_unit_variant("Aggregate", 8u32, "max"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Action taken after the threshold is met. Default is ‘continue’."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Action")]
    pub enum Action {
        #[serde(rename = "continue")]
        Continue,
        #[serde(rename = "stop")]
        Stop,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Action {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Action {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Action {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Continue => serializer.serialize_unit_variant("Action", 0u32, "continue"),
                Self::Stop => serializer.serialize_unit_variant("Action", 1u32, "stop"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Action {
        fn default() -> Self {
            Self::Continue
        }
    }
    #[doc = "Outcome of the test run."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Result")]
    pub enum Result {
        #[serde(rename = "passed")]
        Passed,
        #[serde(rename = "undetermined")]
        Undetermined,
        #[serde(rename = "failed")]
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Result {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Result {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Result {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Passed => serializer.serialize_unit_variant("Result", 0u32, "passed"),
                Self::Undetermined => serializer.serialize_unit_variant("Result", 1u32, "undetermined"),
                Self::Failed => serializer.serialize_unit_variant("Result", 2u32, "failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Associated metric definition for particular metrics of the azure resource ( Refer : https://docs.microsoft.com/en-us/rest/api/monitor/metric-definitions/list#metricdefinition)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceMetric {
    #[doc = "Unique name for metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource id."]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "Metric name space."]
    #[serde(rename = "metricNamespace")]
    pub metric_namespace: String,
    #[doc = "Metric description."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "The invariant value of metric name"]
    pub name: String,
    #[doc = "Metric aggregation."]
    pub aggregation: String,
    #[doc = "Metric unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Azure resource type."]
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}
impl ResourceMetric {
    pub fn new(resource_id: String, metric_namespace: String, name: String, aggregation: String, resource_type: String) -> Self {
        Self {
            id: None,
            resource_id,
            metric_namespace,
            display_description: None,
            name,
            aggregation,
            unit: None,
            resource_type,
        }
    }
}
#[doc = "Secret"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Secret {
    #[doc = "The value of the secret for the respective type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Type of secret"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<secret::Type>,
}
impl Secret {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod secret {
    use super::*;
    #[doc = "Type of secret"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "AKV_SECRET_URI")]
        AkvSecretUri,
        #[serde(rename = "SECRET_VALUE")]
        SecretValue,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Type {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AkvSecretUri => serializer.serialize_unit_variant("Type", 0u32, "AKV_SECRET_URI"),
                Self::SecretValue => serializer.serialize_unit_variant("Type", 1u32, "SECRET_VALUE"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Load test model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Test {
    #[doc = "Pass fail criteria for a test."]
    #[serde(rename = "passFailCriteria", default, skip_serializing_if = "Option::is_none")]
    pub pass_fail_criteria: Option<PassFailCriteria>,
    #[doc = "Secrets can be stored in an Azure Key Vault or any other secret store. If the secret is stored in an Azure Key Vault, the value should be the secret identifier and the type should be AKV_SECRET_URI. If the secret is stored elsewhere, the secret value should be provided directly and the type should be SECRET_VALUE."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<serde_json::Value>,
    #[doc = "Certificates metadata"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<CertificateMetadata>,
    #[doc = "Environment variables which are defined as a set of <name,value> pairs."]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<serde_json::Value>,
    #[doc = "The load test configuration."]
    #[serde(rename = "loadTestConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub load_test_configuration: Option<LoadTestConfiguration>,
    #[doc = "The input artifacts for the test."]
    #[serde(rename = "inputArtifacts", default, skip_serializing_if = "Option::is_none")]
    pub input_artifacts: Option<TestInputArtifacts>,
    #[doc = "Unique test name as identifier."]
    #[serde(rename = "testId", default, skip_serializing_if = "Option::is_none")]
    pub test_id: Option<String>,
    #[doc = "The test description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Display name of a test."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Subnet ID on which the load test instances should run."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "Type of the managed identity referencing the Key vault."]
    #[serde(rename = "keyvaultReferenceIdentityType", default, skip_serializing_if = "Option::is_none")]
    pub keyvault_reference_identity_type: Option<String>,
    #[doc = "Resource Id of the managed identity referencing the Key vault."]
    #[serde(rename = "keyvaultReferenceIdentityId", default, skip_serializing_if = "Option::is_none")]
    pub keyvault_reference_identity_id: Option<String>,
    #[doc = "The creation datetime(ISO 8601 literal format)."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "The user that created."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The last Modified datetime(ISO 8601 literal format)."]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "The user that last modified."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
}
impl Test {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test app component"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestAppComponents {
    #[doc = "Azure resource collection { resource id (fully qualified resource Id e.g subscriptions/{subId}/resourceGroups/{rg}/providers/Microsoft.LoadTestService/loadtests/{resName}) : resource object } "]
    pub components: serde_json::Value,
    #[doc = "Test identifier"]
    #[serde(rename = "testId", default, skip_serializing_if = "Option::is_none")]
    pub test_id: Option<String>,
    #[doc = "The creation datetime(ISO 8601 literal format)."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "The user that created."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The last Modified datetime(ISO 8601 literal format)."]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "The user that last modified."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
}
impl TestAppComponents {
    pub fn new(components: serde_json::Value) -> Self {
        Self {
            components,
            test_id: None,
            created_date_time: None,
            created_by: None,
            last_modified_date_time: None,
            last_modified_by: None,
        }
    }
}
#[doc = "The input artifacts for the test."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestInputArtifacts {
    #[doc = "File info"]
    #[serde(rename = "configFileInfo", default, skip_serializing_if = "Option::is_none")]
    pub config_file_info: Option<FileInfo>,
    #[doc = "File info"]
    #[serde(rename = "testScriptFileInfo", default, skip_serializing_if = "Option::is_none")]
    pub test_script_file_info: Option<FileInfo>,
    #[doc = "File info"]
    #[serde(rename = "userPropFileInfo", default, skip_serializing_if = "Option::is_none")]
    pub user_prop_file_info: Option<FileInfo>,
    #[doc = "File info"]
    #[serde(rename = "inputArtifactsZipFileInfo", default, skip_serializing_if = "Option::is_none")]
    pub input_artifacts_zip_file_info: Option<FileInfo>,
    #[doc = "Additional supported files for the test run"]
    #[serde(
        rename = "additionalFileInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_file_info: Vec<FileInfo>,
}
impl TestInputArtifacts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Load test run model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRun {
    #[doc = "Pass fail criteria for a test."]
    #[serde(rename = "passFailCriteria", default, skip_serializing_if = "Option::is_none")]
    pub pass_fail_criteria: Option<PassFailCriteria>,
    #[doc = "Secrets can be stored in an Azure Key Vault or any other secret store. If the secret is stored in an Azure Key Vault, the value should be the secret identifier and the type should be AKV_SECRET_URI. If the secret is stored elsewhere, the secret value should be provided directly and the type should be SECRET_VALUE."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<serde_json::Value>,
    #[doc = "Certificates metadata"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<CertificateMetadata>,
    #[doc = "Environment variables which are defined as a set of <name,value> pairs."]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<serde_json::Value>,
    #[doc = "Error details if there is any failure in load test run"]
    #[serde(
        rename = "errorDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub error_details: Vec<ErrorDetails>,
    #[doc = "Test run statistics."]
    #[serde(rename = "testRunStatistics", default, skip_serializing_if = "Option::is_none")]
    pub test_run_statistics: Option<serde_json::Value>,
    #[doc = "The load test configuration."]
    #[serde(rename = "loadTestConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub load_test_configuration: Option<LoadTestConfiguration>,
    #[doc = "Collection of test run artifacts"]
    #[serde(rename = "testArtifacts", default, skip_serializing_if = "Option::is_none")]
    pub test_artifacts: Option<TestRunArtifacts>,
    #[doc = "Test result for pass/Fail criteria used during the test run."]
    #[serde(rename = "testResult", default, skip_serializing_if = "Option::is_none")]
    pub test_result: Option<test_run::TestResult>,
    #[doc = "Number of virtual users, for which test has been run."]
    #[serde(rename = "virtualUsers", default, skip_serializing_if = "Option::is_none")]
    pub virtual_users: Option<i32>,
    #[doc = "Unique test run name as identifier"]
    #[serde(rename = "testRunId", default, skip_serializing_if = "Option::is_none")]
    pub test_run_id: Option<String>,
    #[doc = "Display name of a testRun."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Associated test Id."]
    #[serde(rename = "testId", default, skip_serializing_if = "Option::is_none")]
    pub test_id: Option<String>,
    #[doc = "The test run description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The test run status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<test_run::Status>,
    #[doc = "The test run start DateTime(ISO 8601 literal format)."]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "The test run end DateTime(ISO 8601 literal format)."]
    #[serde(rename = "endDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_date_time: Option<time::OffsetDateTime>,
    #[doc = "Test run initiated time."]
    #[serde(rename = "executedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub executed_date_time: Option<time::OffsetDateTime>,
    #[doc = "Portal url."]
    #[serde(rename = "portalUrl", default, skip_serializing_if = "Option::is_none")]
    pub portal_url: Option<String>,
    #[doc = "Test run duration in milliseconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[doc = "Subnet ID on which the load test instances should run."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "The creation datetime(ISO 8601 literal format)."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "The user that created."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The last Modified datetime(ISO 8601 literal format)."]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "The user that last modified."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
}
impl TestRun {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod test_run {
    use super::*;
    #[doc = "Test result for pass/Fail criteria used during the test run."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TestResult")]
    pub enum TestResult {
        #[serde(rename = "PASSED")]
        Passed,
        #[serde(rename = "NOT_APPLICABLE")]
        NotApplicable,
        #[serde(rename = "FAILED")]
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TestResult {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TestResult {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TestResult {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Passed => serializer.serialize_unit_variant("TestResult", 0u32, "PASSED"),
                Self::NotApplicable => serializer.serialize_unit_variant("TestResult", 1u32, "NOT_APPLICABLE"),
                Self::Failed => serializer.serialize_unit_variant("TestResult", 2u32, "FAILED"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The test run status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        #[serde(rename = "ACCEPTED")]
        Accepted,
        #[serde(rename = "NOTSTARTED")]
        Notstarted,
        #[serde(rename = "PROVISIONING")]
        Provisioning,
        #[serde(rename = "PROVISIONED")]
        Provisioned,
        #[serde(rename = "CONFIGURING")]
        Configuring,
        #[serde(rename = "CONFIGURED")]
        Configured,
        #[serde(rename = "EXECUTING")]
        Executing,
        #[serde(rename = "EXECUTED")]
        Executed,
        #[serde(rename = "DEPROVISIONING")]
        Deprovisioning,
        #[serde(rename = "DEPROVISIONED")]
        Deprovisioned,
        #[serde(rename = "DONE")]
        Done,
        #[serde(rename = "CANCELLING")]
        Cancelling,
        #[serde(rename = "CANCELLED")]
        Cancelled,
        #[serde(rename = "FAILED")]
        Failed,
        #[serde(rename = "VALIDATION_SUCCESS")]
        ValidationSuccess,
        #[serde(rename = "VALIDATION_FAILURE")]
        ValidationFailure,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Status {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Status {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Status {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Accepted => serializer.serialize_unit_variant("Status", 0u32, "ACCEPTED"),
                Self::Notstarted => serializer.serialize_unit_variant("Status", 1u32, "NOTSTARTED"),
                Self::Provisioning => serializer.serialize_unit_variant("Status", 2u32, "PROVISIONING"),
                Self::Provisioned => serializer.serialize_unit_variant("Status", 3u32, "PROVISIONED"),
                Self::Configuring => serializer.serialize_unit_variant("Status", 4u32, "CONFIGURING"),
                Self::Configured => serializer.serialize_unit_variant("Status", 5u32, "CONFIGURED"),
                Self::Executing => serializer.serialize_unit_variant("Status", 6u32, "EXECUTING"),
                Self::Executed => serializer.serialize_unit_variant("Status", 7u32, "EXECUTED"),
                Self::Deprovisioning => serializer.serialize_unit_variant("Status", 8u32, "DEPROVISIONING"),
                Self::Deprovisioned => serializer.serialize_unit_variant("Status", 9u32, "DEPROVISIONED"),
                Self::Done => serializer.serialize_unit_variant("Status", 10u32, "DONE"),
                Self::Cancelling => serializer.serialize_unit_variant("Status", 11u32, "CANCELLING"),
                Self::Cancelled => serializer.serialize_unit_variant("Status", 12u32, "CANCELLED"),
                Self::Failed => serializer.serialize_unit_variant("Status", 13u32, "FAILED"),
                Self::ValidationSuccess => serializer.serialize_unit_variant("Status", 14u32, "VALIDATION_SUCCESS"),
                Self::ValidationFailure => serializer.serialize_unit_variant("Status", 15u32, "VALIDATION_FAILURE"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Test run app component"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestRunAppComponents {
    #[doc = "Azure resource collection { resource id (fully qualified resource Id e.g subscriptions/{subId}/resourceGroups/{rg}/providers/Microsoft.LoadTestService/loadtests/{resName}) : resource object } "]
    pub components: serde_json::Value,
    #[doc = "Test run identifier"]
    #[serde(rename = "testRunId", default, skip_serializing_if = "Option::is_none")]
    pub test_run_id: Option<String>,
    #[doc = "The creation datetime(ISO 8601 literal format)."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "The user that created."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The last Modified datetime(ISO 8601 literal format)."]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "The user that last modified."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
}
impl TestRunAppComponents {
    pub fn new(components: serde_json::Value) -> Self {
        Self {
            components,
            test_run_id: None,
            created_date_time: None,
            created_by: None,
            last_modified_date_time: None,
            last_modified_by: None,
        }
    }
}
#[doc = "Collection of test run artifacts"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRunArtifacts {
    #[doc = "The input artifacts for the test run."]
    #[serde(rename = "inputArtifacts", default, skip_serializing_if = "Option::is_none")]
    pub input_artifacts: Option<TestRunInputArtifacts>,
    #[doc = "The output artifacts for the test run."]
    #[serde(rename = "outputArtifacts", default, skip_serializing_if = "Option::is_none")]
    pub output_artifacts: Option<TestRunOutputArtifacts>,
}
impl TestRunArtifacts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The input artifacts for the test run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRunInputArtifacts {
    #[doc = "File info"]
    #[serde(rename = "configFileInfo", default, skip_serializing_if = "Option::is_none")]
    pub config_file_info: Option<FileInfo>,
    #[doc = "File info"]
    #[serde(rename = "testScriptFileInfo", default, skip_serializing_if = "Option::is_none")]
    pub test_script_file_info: Option<FileInfo>,
    #[doc = "File info"]
    #[serde(rename = "userPropFileInfo", default, skip_serializing_if = "Option::is_none")]
    pub user_prop_file_info: Option<FileInfo>,
    #[doc = "File info"]
    #[serde(rename = "inputArtifactsZipFileInfo", default, skip_serializing_if = "Option::is_none")]
    pub input_artifacts_zip_file_info: Option<FileInfo>,
    #[doc = "Additional supported files for the test run"]
    #[serde(
        rename = "additionalFileInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_file_info: Vec<FileInfo>,
}
impl TestRunInputArtifacts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The output artifacts for the test run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRunOutputArtifacts {
    #[doc = "File info"]
    #[serde(rename = "resultFileInfo", default, skip_serializing_if = "Option::is_none")]
    pub result_file_info: Option<FileInfo>,
    #[doc = "File info"]
    #[serde(rename = "logsFileInfo", default, skip_serializing_if = "Option::is_none")]
    pub logs_file_info: Option<FileInfo>,
}
impl TestRunOutputArtifacts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test run server metrics configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRunServerMetricConfig {
    #[doc = "Test run identifier"]
    #[serde(rename = "testRunId", default, skip_serializing_if = "Option::is_none")]
    pub test_run_id: Option<String>,
    #[doc = "Azure resource metrics collection {metric id : metrics object} (Refer : https://docs.microsoft.com/en-us/rest/api/monitor/metric-definitions/list#metricdefinition for metric id)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<serde_json::Value>,
    #[doc = "The creation datetime(ISO 8601 literal format)."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "The user that created."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The last Modified datetime(ISO 8601 literal format)."]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "The user that last modified."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
}
impl TestRunServerMetricConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test run statistics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRunStatistics {
    #[doc = "Transaction name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction: Option<String>,
    #[doc = "Sampler count."]
    #[serde(rename = "sampleCount", default, skip_serializing_if = "Option::is_none")]
    pub sample_count: Option<f64>,
    #[doc = "Error count."]
    #[serde(rename = "errorCount", default, skip_serializing_if = "Option::is_none")]
    pub error_count: Option<f64>,
    #[doc = "Error percentage."]
    #[serde(rename = "errorPct", default, skip_serializing_if = "Option::is_none")]
    pub error_pct: Option<f64>,
    #[doc = "Mean response time."]
    #[serde(rename = "meanResTime", default, skip_serializing_if = "Option::is_none")]
    pub mean_res_time: Option<f64>,
    #[doc = "Median response time."]
    #[serde(rename = "medianResTime", default, skip_serializing_if = "Option::is_none")]
    pub median_res_time: Option<f64>,
    #[doc = "Max response time."]
    #[serde(rename = "maxResTime", default, skip_serializing_if = "Option::is_none")]
    pub max_res_time: Option<f64>,
    #[doc = "Minimum response time."]
    #[serde(rename = "minResTime", default, skip_serializing_if = "Option::is_none")]
    pub min_res_time: Option<f64>,
    #[doc = "90 percentile response time."]
    #[serde(rename = "pct1ResTime", default, skip_serializing_if = "Option::is_none")]
    pub pct1_res_time: Option<f64>,
    #[doc = "95 percentile response time."]
    #[serde(rename = "pct2ResTime", default, skip_serializing_if = "Option::is_none")]
    pub pct2_res_time: Option<f64>,
    #[doc = "99 percentile response time."]
    #[serde(rename = "pct3ResTime", default, skip_serializing_if = "Option::is_none")]
    pub pct3_res_time: Option<f64>,
    #[doc = "Throughput."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throughput: Option<f64>,
    #[doc = "Received network bytes."]
    #[serde(rename = "receivedKBytesPerSec", default, skip_serializing_if = "Option::is_none")]
    pub received_k_bytes_per_sec: Option<f64>,
    #[doc = "Send network bytes."]
    #[serde(rename = "sentKBytesPerSec", default, skip_serializing_if = "Option::is_none")]
    pub sent_k_bytes_per_sec: Option<f64>,
}
impl TestRunStatistics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of test runs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestRunsList {
    #[doc = "List of test runs"]
    pub value: Vec<TestRun>,
    #[doc = "Link for the next list of test runs in case of paginated results, if applicable"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TestRunsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TestRunsList {
    pub fn new(value: Vec<TestRun>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Test server metrics configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestServerMetricConfig {
    #[doc = "Test identifier"]
    #[serde(rename = "testId", default, skip_serializing_if = "Option::is_none")]
    pub test_id: Option<String>,
    #[doc = "Azure resource metrics collection {metric id : metrics object} (Refer : https://docs.microsoft.com/en-us/rest/api/monitor/metric-definitions/list#metricdefinition for metric id)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<serde_json::Value>,
    #[doc = "The creation datetime(ISO 8601 literal format)."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "The user that created."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The last Modified datetime(ISO 8601 literal format)."]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "The user that last modified."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
}
impl TestServerMetricConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of tests"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestsList {
    #[doc = "List of tests"]
    pub value: Vec<Test>,
    #[doc = "Link for the next list of tests in case of paginated results, if applicable"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for TestsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl TestsList {
    pub fn new(value: Vec<Test>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The time series returned when a data query is performed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeSeriesElement {
    #[doc = "An array of data points representing the metric values."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data: Vec<MetricValue>,
    #[doc = "The dimension values "]
    #[serde(
        rename = "dimensionValues",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dimension_values: Vec<DimensionValue>,
}
impl TimeSeriesElement {
    pub fn new() -> Self {
        Self::default()
    }
}
