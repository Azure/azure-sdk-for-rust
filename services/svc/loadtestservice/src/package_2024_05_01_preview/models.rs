#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Aggregation type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AggregationType")]
pub enum AggregationType {
    Average,
    Count,
    None,
    Total,
    Percentile75,
    Percentile90,
    Percentile95,
    Percentile96,
    Percentile97,
    Percentile98,
    Percentile99,
    Percentile999,
    Percentile9999,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AggregationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AggregationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AggregationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Average => serializer.serialize_unit_variant("AggregationType", 0u32, "Average"),
            Self::Count => serializer.serialize_unit_variant("AggregationType", 1u32, "Count"),
            Self::None => serializer.serialize_unit_variant("AggregationType", 2u32, "None"),
            Self::Total => serializer.serialize_unit_variant("AggregationType", 3u32, "Total"),
            Self::Percentile75 => serializer.serialize_unit_variant("AggregationType", 4u32, "Percentile75"),
            Self::Percentile90 => serializer.serialize_unit_variant("AggregationType", 5u32, "Percentile90"),
            Self::Percentile95 => serializer.serialize_unit_variant("AggregationType", 6u32, "Percentile95"),
            Self::Percentile96 => serializer.serialize_unit_variant("AggregationType", 7u32, "Percentile96"),
            Self::Percentile97 => serializer.serialize_unit_variant("AggregationType", 8u32, "Percentile97"),
            Self::Percentile98 => serializer.serialize_unit_variant("AggregationType", 9u32, "Percentile98"),
            Self::Percentile99 => serializer.serialize_unit_variant("AggregationType", 10u32, "Percentile99"),
            Self::Percentile999 => serializer.serialize_unit_variant("AggregationType", 11u32, "Percentile999"),
            Self::Percentile9999 => serializer.serialize_unit_variant("AggregationType", 12u32, "Percentile9999"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An Azure resource object (Refer azure generic resource model :https://docs.microsoft.com/en-us/rest/api/resources/resources/get-by-id#genericresource)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppComponent {
    #[doc = "fully qualified resource Id e.g subscriptions/{subId}/resourceGroups/{rg}/providers/Microsoft.LoadTestService/loadtests/{resName}"]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "Azure resource name, required while creating the app component."]
    #[serde(rename = "resourceName")]
    pub resource_name: String,
    #[doc = "Azure resource type, required while creating the app component."]
    #[serde(rename = "resourceType")]
    pub resource_type: String,
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
    pub fn new(resource_id: String, resource_name: String, resource_type: String) -> Self {
        Self {
            resource_id,
            resource_name,
            resource_type,
            display_name: None,
            resource_group: None,
            subscription_id: None,
            kind: None,
        }
    }
}
#[doc = "An Azure resource object (Refer azure generic resource model :https://docs.microsoft.com/en-us/rest/api/resources/resources/get-by-id#genericresource)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppComponentUpdate {
    #[doc = "Azure resource name, required while creating the app component."]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "Azure resource type, required while creating the app component."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Azure resource display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Kind of Azure resource type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
impl AppComponentUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Artifacts container info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactsContainerInfo {
    #[doc = "This is a SAS URI to an Azure Storage Container that contains the test run artifacts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Expiry time of the container (RFC 3339 literal format)"]
    #[serde(rename = "expireDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub expire_date_time: Option<::time::OffsetDateTime>,
}
impl ArtifactsContainerInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Auto stop criteria for a test. This will automatically stop a load test if the error percentage is high for a certain time window."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoStopCriteria {
    #[doc = "Whether auto-stop should be disabled. The default value is false."]
    #[serde(rename = "autoStopDisabled", default, skip_serializing_if = "Option::is_none")]
    pub auto_stop_disabled: Option<bool>,
    #[doc = "Threshold percentage of errors on which test run should be automatically stopped. Allowed values are in range of 0.0-100.0"]
    #[serde(rename = "errorRate", default, skip_serializing_if = "Option::is_none")]
    pub error_rate: Option<f32>,
    #[doc = "Time window during which the error percentage should be evaluated in seconds."]
    #[serde(rename = "errorRateTimeWindowInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub error_rate_time_window_in_seconds: Option<i32>,
}
impl AutoStopCriteria {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCoreFoundationsError {
    #[doc = "One of a server-defined set of error codes."]
    pub code: String,
    #[doc = "A human-readable representation of the error."]
    pub message: String,
    #[doc = "The target of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "An array of details about specific errors that led to this reported error."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<AzureCoreFoundationsError>,
    #[doc = "An object containing more specific information about the error. As per Microsoft One API guidelines - https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<AzureCoreFoundationsInnerError>,
}
impl AzureCoreFoundationsError {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            details: Vec::new(),
            innererror: None,
        }
    }
}
#[doc = "A response containing error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureCoreFoundationsErrorResponse {
    #[doc = "The error object."]
    pub error: AzureCoreFoundationsError,
}
impl azure_core::Continuable for AzureCoreFoundationsErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl AzureCoreFoundationsErrorResponse {
    pub fn new(error: AzureCoreFoundationsError) -> Self {
        Self { error }
    }
}
#[doc = "An object containing more specific information about the error. As per Microsoft One API guidelines - https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureCoreFoundationsInnerError {
    #[doc = "One of a server-defined set of error codes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "An object containing more specific information about the error. As per Microsoft One API guidelines - https://github.com/Microsoft/api-guidelines/blob/vNext/Guidelines.md#7102-error-condition-responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<Box<AzureCoreFoundationsInnerError>>,
}
impl AzureCoreFoundationsInnerError {
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
    #[doc = "Types of certificates supported."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<CertificateType>,
    #[doc = "Name of the certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl CertificateMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Types of certificates supported."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CertificateType")]
pub enum CertificateType {
    #[serde(rename = "AKV_CERT_URI")]
    AkvCertUri,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CertificateType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CertificateType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CertificateType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AkvCertUri => serializer.serialize_unit_variant("CertificateType", 0u32, "AKV_CERT_URI"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
    #[doc = "The dimension name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The dimension value"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<String>,
    #[doc = "Link for the next set of values in case of paginated results, if applicable."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl DimensionValueList {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "File status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FileStatus")]
pub enum FileStatus {
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
impl FromStr for FileStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FileStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FileStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotValidated => serializer.serialize_unit_variant("FileStatus", 0u32, "NOT_VALIDATED"),
            Self::ValidationSuccess => serializer.serialize_unit_variant("FileStatus", 1u32, "VALIDATION_SUCCESS"),
            Self::ValidationFailure => serializer.serialize_unit_variant("FileStatus", 2u32, "VALIDATION_FAILURE"),
            Self::ValidationInitiated => serializer.serialize_unit_variant("FileStatus", 3u32, "VALIDATION_INITIATED"),
            Self::ValidationNotRequired => serializer.serialize_unit_variant("FileStatus", 4u32, "VALIDATION_NOT_REQUIRED"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Types of file supported."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "FileType")]
pub enum FileType {
    #[serde(rename = "JMX_FILE")]
    JmxFile,
    #[serde(rename = "USER_PROPERTIES")]
    UserProperties,
    #[serde(rename = "ADDITIONAL_ARTIFACTS")]
    AdditionalArtifacts,
    #[serde(rename = "ZIPPED_ARTIFACTS")]
    ZippedArtifacts,
    #[serde(rename = "URL_TEST_CONFIG")]
    UrlTestConfig,
    #[serde(rename = "TEST_SCRIPT")]
    TestScript,
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
            Self::ZippedArtifacts => serializer.serialize_unit_variant("FileType", 3u32, "ZIPPED_ARTIFACTS"),
            Self::UrlTestConfig => serializer.serialize_unit_variant("FileType", 4u32, "URL_TEST_CONFIG"),
            Self::TestScript => serializer.serialize_unit_variant("FileType", 5u32, "TEST_SCRIPT"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Resource configuration instance for a Flex Consumption based Azure Function App."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionFlexConsumptionResourceConfiguration {
    #[doc = "Memory size of the instance. Supported values are 2048, 4096."]
    #[serde(rename = "instanceMemoryMB")]
    pub instance_memory_mb: i64,
    #[doc = "HTTP Concurrency for the function app."]
    #[serde(rename = "httpConcurrency")]
    pub http_concurrency: i64,
}
impl FunctionFlexConsumptionResourceConfiguration {
    pub fn new(instance_memory_mb: i64, http_concurrency: i64) -> Self {
        Self {
            instance_memory_mb,
            http_concurrency,
        }
    }
}
#[doc = "Resource configuration instance for a Flex Consumption based Azure Function App."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FunctionFlexConsumptionResourceConfigurationCreateOrUpdate {
    #[doc = "Memory size of the instance. Supported values are 2048, 4096."]
    #[serde(rename = "instanceMemoryMB", default, skip_serializing_if = "Option::is_none")]
    pub instance_memory_mb: Option<i64>,
    #[doc = "HTTP Concurrency for the function app."]
    #[serde(rename = "httpConcurrency", default, skip_serializing_if = "Option::is_none")]
    pub http_concurrency: Option<i64>,
}
impl FunctionFlexConsumptionResourceConfigurationCreateOrUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Configurations for a Function App using Flex Consumption Plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionFlexConsumptionTargetResourceConfigurations {
    #[doc = "A map of configurations for a Function app using Flex Consumption Plan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configurations: Option<serde_json::Value>,
}
impl FunctionFlexConsumptionTargetResourceConfigurations {
    pub fn new() -> Self {
        Self { configurations: None }
    }
}
#[doc = "Configurations for a Function App using Flex Consumption Plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionFlexConsumptionTargetResourceConfigurationsCreateOrUpdate {
    #[doc = "A map of configurations for a Function app using Flex Consumption Plan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configurations: Option<serde_json::Value>,
}
impl FunctionFlexConsumptionTargetResourceConfigurationsCreateOrUpdate {
    pub fn new() -> Self {
        Self { configurations: None }
    }
}
#[doc = "Configurations for the load test."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadTestConfiguration {
    #[doc = "The number of engine instances to execute load test. Supported values are in range of 1-400. Required for creating a new test."]
    #[serde(rename = "engineInstances", default, skip_serializing_if = "Option::is_none")]
    pub engine_instances: Option<i32>,
    #[doc = "If false, Azure Load Testing copies and processes your input files unmodified\nacross all test engine instances. If true, Azure Load Testing splits the CSV\ninput data evenly across all engine instances. If you provide multiple CSV\nfiles, each file will be split evenly."]
    #[serde(rename = "splitAllCSVs", default, skip_serializing_if = "Option::is_none")]
    pub split_all_cs_vs: Option<bool>,
    #[doc = "If true, optionalLoadTestConfig is required and JMX script for the load test is\nnot required to upload."]
    #[serde(rename = "quickStartTest", default, skip_serializing_if = "Option::is_none")]
    pub quick_start_test: Option<bool>,
    #[doc = "Configuration for quick load test"]
    #[serde(rename = "optionalLoadTestConfig", default, skip_serializing_if = "Option::is_none")]
    pub optional_load_test_config: Option<OptionalLoadTestConfig>,
    #[doc = "Region distribution configuration for the load test."]
    #[serde(
        rename = "regionalLoadTestConfig",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub regional_load_test_config: Vec<RegionalConfiguration>,
}
impl LoadTestConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metric availability specifies the time grain (aggregation interval or frequency)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAvailability {
    #[doc = "Time Grain"]
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<TimeGrain>,
}
impl MetricAvailability {
    pub fn new() -> Self {
        Self::default()
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
    #[doc = "Aggregation type."]
    #[serde(rename = "primaryAggregationType", default, skip_serializing_if = "Option::is_none")]
    pub primary_aggregation_type: Option<AggregationType>,
    #[doc = "The collection of what all aggregation types are supported."]
    #[serde(
        rename = "supportedAggregationTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_aggregation_types: Vec<String>,
    #[doc = "Metric unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<MetricUnit>,
    #[doc = "Metric availability specifies the time grain (aggregation interval or\nfrequency)."]
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
#[doc = "Filters to fetch the set of metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricRequestPayload {
    #[doc = "Get metrics for specific dimension values. Example: Metric contains dimension\nlike SamplerName, Error. To retrieve all the time series data where SamplerName\nis equals to HTTPRequest1 or HTTPRequest2, the DimensionFilter value will be\n{\"SamplerName\", [\"HTTPRequest1\", \"HTTPRequest2\"}"]
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
#[doc = "Metric unit."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MetricUnit")]
pub enum MetricUnit {
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
impl FromStr for MetricUnit {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MetricUnit {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MetricUnit {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("MetricUnit", 0u32, "NotSpecified"),
            Self::Percent => serializer.serialize_unit_variant("MetricUnit", 1u32, "Percent"),
            Self::Count => serializer.serialize_unit_variant("MetricUnit", 2u32, "Count"),
            Self::Seconds => serializer.serialize_unit_variant("MetricUnit", 3u32, "Seconds"),
            Self::Milliseconds => serializer.serialize_unit_variant("MetricUnit", 4u32, "Milliseconds"),
            Self::Bytes => serializer.serialize_unit_variant("MetricUnit", 5u32, "Bytes"),
            Self::BytesPerSecond => serializer.serialize_unit_variant("MetricUnit", 6u32, "BytesPerSecond"),
            Self::CountPerSecond => serializer.serialize_unit_variant("MetricUnit", 7u32, "CountPerSecond"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents a metric value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricValue {
    #[doc = "The timestamp for the metric value in RFC 3339 format."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<::time::OffsetDateTime>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metrics {
    #[doc = "The TimeSeriesElement items on this page"]
    pub value: Vec<TimeSeriesElement>,
    #[doc = "The link to the next page of items"]
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
    pub fn new(value: Vec<TimeSeriesElement>) -> Self {
        Self { value, next_link: None }
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
#[doc = "Configuration for quick load test"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OptionalLoadTestConfig {
    #[doc = "Test URL. Provide the complete HTTP URL. For example, https://contoso-app.azurewebsites.net/login"]
    #[serde(rename = "endpointUrl", default, skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    #[doc = "Target throughput (requests per second). This may not be necessarily achieved. The actual throughput will be lower if the application is not capable of handling it."]
    #[serde(rename = "requestsPerSecond", default, skip_serializing_if = "Option::is_none")]
    pub requests_per_second: Option<i32>,
    #[doc = "Maximum response time in milliseconds of the API/endpoint."]
    #[serde(rename = "maxResponseTimeInMs", default, skip_serializing_if = "Option::is_none")]
    pub max_response_time_in_ms: Option<i32>,
    #[doc = "No of concurrent virtual users."]
    #[serde(rename = "virtualUsers", default, skip_serializing_if = "Option::is_none")]
    pub virtual_users: Option<i32>,
    #[doc = "Ramp up time in seconds."]
    #[serde(rename = "rampUpTime", default, skip_serializing_if = "Option::is_none")]
    pub ramp_up_time: Option<i32>,
    #[doc = "Test run duration in seconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
}
impl OptionalLoadTestConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Aggregation functions for pass/fail criteria."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PfAgFunc")]
pub enum PfAgFunc {
    #[serde(rename = "count")]
    Count,
    #[serde(rename = "percentage")]
    Percentage,
    #[serde(rename = "avg")]
    Avg,
    #[serde(rename = "p50")]
    P50,
    #[serde(rename = "p75")]
    P75,
    #[serde(rename = "p90")]
    P90,
    #[serde(rename = "p95")]
    P95,
    #[serde(rename = "p96")]
    P96,
    #[serde(rename = "p97")]
    P97,
    #[serde(rename = "p98")]
    P98,
    #[serde(rename = "p99")]
    P99,
    #[serde(rename = "p99.9")]
    P999,
    #[serde(rename = "p99.99")]
    P9999,
    #[serde(rename = "min")]
    Min,
    #[serde(rename = "max")]
    Max,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PfAgFunc {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PfAgFunc {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PfAgFunc {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Count => serializer.serialize_unit_variant("PfAgFunc", 0u32, "count"),
            Self::Percentage => serializer.serialize_unit_variant("PfAgFunc", 1u32, "percentage"),
            Self::Avg => serializer.serialize_unit_variant("PfAgFunc", 2u32, "avg"),
            Self::P50 => serializer.serialize_unit_variant("PfAgFunc", 3u32, "p50"),
            Self::P75 => serializer.serialize_unit_variant("PfAgFunc", 4u32, "p75"),
            Self::P90 => serializer.serialize_unit_variant("PfAgFunc", 5u32, "p90"),
            Self::P95 => serializer.serialize_unit_variant("PfAgFunc", 6u32, "p95"),
            Self::P96 => serializer.serialize_unit_variant("PfAgFunc", 7u32, "p96"),
            Self::P97 => serializer.serialize_unit_variant("PfAgFunc", 8u32, "p97"),
            Self::P98 => serializer.serialize_unit_variant("PfAgFunc", 9u32, "p98"),
            Self::P99 => serializer.serialize_unit_variant("PfAgFunc", 10u32, "p99"),
            Self::P999 => serializer.serialize_unit_variant("PfAgFunc", 11u32, "p99.9"),
            Self::P9999 => serializer.serialize_unit_variant("PfAgFunc", 12u32, "p99.99"),
            Self::Min => serializer.serialize_unit_variant("PfAgFunc", 13u32, "min"),
            Self::Max => serializer.serialize_unit_variant("PfAgFunc", 14u32, "max"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Metrics for pass/fail criteria."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PfMetrics")]
pub enum PfMetrics {
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
impl FromStr for PfMetrics {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PfMetrics {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PfMetrics {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ResponseTimeMs => serializer.serialize_unit_variant("PfMetrics", 0u32, "response_time_ms"),
            Self::Latency => serializer.serialize_unit_variant("PfMetrics", 1u32, "latency"),
            Self::Error => serializer.serialize_unit_variant("PfMetrics", 2u32, "error"),
            Self::Requests => serializer.serialize_unit_variant("PfMetrics", 3u32, "requests"),
            Self::RequestsPerSec => serializer.serialize_unit_variant("PfMetrics", 4u32, "requests_per_sec"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Pass/fail criteria result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PfResult")]
pub enum PfResult {
    #[serde(rename = "passed")]
    Passed,
    #[serde(rename = "undetermined")]
    Undetermined,
    #[serde(rename = "failed")]
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PfResult {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PfResult {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PfResult {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Passed => serializer.serialize_unit_variant("PfResult", 0u32, "passed"),
            Self::Undetermined => serializer.serialize_unit_variant("PfResult", 1u32, "undetermined"),
            Self::Failed => serializer.serialize_unit_variant("PfResult", 2u32, "failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Test result based on pass/fail criteria."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PfTestResult")]
pub enum PfTestResult {
    #[serde(rename = "PASSED")]
    Passed,
    #[serde(rename = "NOT_APPLICABLE")]
    NotApplicable,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PfTestResult {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PfTestResult {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PfTestResult {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Passed => serializer.serialize_unit_variant("PfTestResult", 0u32, "PASSED"),
            Self::NotApplicable => serializer.serialize_unit_variant("PfTestResult", 1u32, "NOT_APPLICABLE"),
            Self::Failed => serializer.serialize_unit_variant("PfTestResult", 2u32, "FAILED"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Paged collection of Test items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedTest {
    #[doc = "The Test items on this page"]
    pub value: Vec<Test>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedTest {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedTest {
    pub fn new(value: Vec<Test>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of TestFileInfo items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedTestFileInfo {
    #[doc = "The TestFileInfo items on this page"]
    pub value: Vec<TestFileInfo>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedTestFileInfo {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedTestFileInfo {
    pub fn new(value: Vec<TestFileInfo>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of TestProfile items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedTestProfile {
    #[doc = "The TestProfile items on this page"]
    pub value: Vec<TestProfile>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedTestProfile {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedTestProfile {
    pub fn new(value: Vec<TestProfile>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of TestProfileRun items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedTestProfileRun {
    #[doc = "The TestProfileRun items on this page"]
    pub value: Vec<TestProfileRun>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedTestProfileRun {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedTestProfileRun {
    pub fn new(value: Vec<TestProfileRun>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of TestRun items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedTestRun {
    #[doc = "The TestRun items on this page"]
    pub value: Vec<TestRun>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedTestRun {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedTestRun {
    pub fn new(value: Vec<TestRun>) -> Self {
        Self { value, next_link: None }
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
    #[doc = "Metrics for pass/fail criteria."]
    #[serde(rename = "clientMetric", default, skip_serializing_if = "Option::is_none")]
    pub client_metric: Option<PfMetrics>,
    #[doc = "Aggregation functions for pass/fail criteria."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<PfAgFunc>,
    #[doc = "The comparison operator. Supported types ‘>’, ‘<’ "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Request name for which the Pass fail criteria has to be applied "]
    #[serde(rename = "requestName", default, skip_serializing_if = "Option::is_none")]
    pub request_name: Option<String>,
    #[doc = "The value to compare with the client metric. Allowed values - ‘error : [0.0 ,\n100.0] unit- % ’, response_time_ms and latency : any integer value unit- ms."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[doc = "Action taken after the threshold is met. Default is ‘continue’."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<pass_fail_metric::Action>,
    #[doc = "The actual value of the client metric for the test run."]
    #[serde(rename = "actualValue", default, skip_serializing_if = "Option::is_none")]
    pub actual_value: Option<f64>,
    #[doc = "Pass/fail criteria result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<PfResult>,
}
impl PassFailMetric {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod pass_fail_metric {
    use super::*;
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
}
#[doc = "Category of Recommendation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RecommendationCategory")]
pub enum RecommendationCategory {
    ThroughputOptimized,
    CostOptimized,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RecommendationCategory {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RecommendationCategory {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RecommendationCategory {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::ThroughputOptimized => serializer.serialize_unit_variant("RecommendationCategory", 0u32, "ThroughputOptimized"),
            Self::CostOptimized => serializer.serialize_unit_variant("RecommendationCategory", 1u32, "CostOptimized"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Region distribution configuration for the load test."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegionalConfiguration {
    #[doc = "  The number of engine instances to execute load test in specified region. Supported values are in range of 1-400."]
    #[serde(rename = "engineInstances")]
    pub engine_instances: i32,
    #[doc = "Azure region name.\nThe region name should of format accepted by ARM, and should be a region supported by Azure Load Testing. For example, East US should be passed as \"eastus\".\nThe region name must match one of the strings in the \"Name\" column returned from running the \"az account list-locations -o table\" Azure CLI command."]
    pub region: String,
}
impl RegionalConfiguration {
    pub fn new(engine_instances: i32, region: String) -> Self {
        Self { engine_instances, region }
    }
}
#[doc = "Request data collection level for test run"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RequestDataLevel")]
pub enum RequestDataLevel {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "ERRORS")]
    Errors,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RequestDataLevel {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RequestDataLevel {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RequestDataLevel {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("RequestDataLevel", 0u32, "NONE"),
            Self::Errors => serializer.serialize_unit_variant("RequestDataLevel", 1u32, "ERRORS"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Kind of the resource on which test profile is created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResourceKind")]
pub enum ResourceKind {
    FunctionsFlexConsumption,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResourceKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResourceKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResourceKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::FunctionsFlexConsumption => serializer.serialize_unit_variant("ResourceKind", 0u32, "FunctionsFlexConsumption"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Associated metric definition for particular metrics of the azure resource (\nRefer :\nhttps://docs.microsoft.com/en-us/rest/api/monitor/metric-definitions/list#metricdefinition)."]
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
#[doc = "Associated metric definition for particular metrics of the azure resource (\nRefer :\nhttps://docs.microsoft.com/en-us/rest/api/monitor/metric-definitions/list#metricdefinition)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceMetricUpdate {
    #[doc = "Azure resource id."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Metric name space."]
    #[serde(rename = "metricNamespace", default, skip_serializing_if = "Option::is_none")]
    pub metric_namespace: Option<String>,
    #[doc = "Metric description."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "The invariant value of metric name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Metric aggregation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<String>,
    #[doc = "Metric unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Azure resource type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}
impl ResourceMetricUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Secret"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Secret {
    #[doc = "The value of the secret for the respective type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Types of secrets supported."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<SecretType>,
}
impl Secret {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Types of secrets supported."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SecretType")]
pub enum SecretType {
    #[serde(rename = "AKV_SECRET_URI")]
    AkvSecretUri,
    #[serde(rename = "SECRET_VALUE")]
    SecretValue,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SecretType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SecretType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SecretType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AkvSecretUri => serializer.serialize_unit_variant("SecretType", 0u32, "AKV_SECRET_URI"),
            Self::SecretValue => serializer.serialize_unit_variant("SecretType", 1u32, "SECRET_VALUE"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Test run status."]
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
#[doc = "Kind of the resource on which test profile is created."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum TargetResourceConfigurationsUnion {
    FunctionsFlexConsumption(FunctionFlexConsumptionTargetResourceConfigurations),
}
#[doc = "Kind of the resource on which test profile is created."]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum TargetResourceConfigurationsCreateOrUpdateUnion {
    FunctionsFlexConsumption(FunctionFlexConsumptionTargetResourceConfigurationsCreateOrUpdate),
}
#[doc = "Load test model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Test {
    #[doc = "Pass fail criteria for a test."]
    #[serde(rename = "passFailCriteria", default, skip_serializing_if = "Option::is_none")]
    pub pass_fail_criteria: Option<PassFailCriteria>,
    #[doc = "Auto stop criteria for a test. This will automatically stop a load test if the error percentage is high for a certain time window."]
    #[serde(rename = "autoStopCriteria", default, skip_serializing_if = "Option::is_none")]
    pub auto_stop_criteria: Option<AutoStopCriteria>,
    #[doc = "Secrets can be stored in an Azure Key Vault or any other secret store. If the\nsecret is stored in an Azure Key Vault, the value should be the secret\nidentifier and the type should be AKV_SECRET_URI. If the secret is stored\nelsewhere, the secret value should be provided directly and the type should be\nSECRET_VALUE."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<serde_json::Value>,
    #[doc = "Certificates metadata"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<CertificateMetadata>,
    #[doc = "Environment variables which are defined as a set of <name,value> pairs."]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<serde_json::Value>,
    #[doc = "Configurations for the load test."]
    #[serde(rename = "loadTestConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub load_test_configuration: Option<LoadTestConfiguration>,
    #[doc = "Id of the test run to be marked as baseline to view trends of client-side metrics from recent test runs"]
    #[serde(rename = "baselineTestRunId", default, skip_serializing_if = "Option::is_none")]
    pub baseline_test_run_id: Option<String>,
    #[doc = "The input artifacts for the test."]
    #[serde(rename = "inputArtifacts", default, skip_serializing_if = "Option::is_none")]
    pub input_artifacts: Option<TestInputArtifacts>,
    #[doc = "Unique test identifier for the load test, must contain only lower-case alphabetic, numeric, underscore or hyphen characters."]
    #[serde(rename = "testId")]
    pub test_id: String,
    #[doc = "The test description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Display name of a test."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Subnet ID on which the load test instances should run."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "Test kind"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<TestKind>,
    #[doc = "Inject load test engines without deploying public IP for outbound access"]
    #[serde(rename = "publicIPDisabled", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_disabled: Option<bool>,
    #[doc = "Type of the managed identity referencing the Key vault."]
    #[serde(rename = "keyvaultReferenceIdentityType", default, skip_serializing_if = "Option::is_none")]
    pub keyvault_reference_identity_type: Option<String>,
    #[doc = "Resource Id of the managed identity referencing the Key vault."]
    #[serde(rename = "keyvaultReferenceIdentityId", default, skip_serializing_if = "Option::is_none")]
    pub keyvault_reference_identity_id: Option<String>,
    #[doc = "The creation datetime(RFC 3339 literal format)."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<::time::OffsetDateTime>,
    #[doc = "The user that created."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The last Modified datetime(RFC 3339 literal format)."]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<::time::OffsetDateTime>,
    #[doc = "The user that last modified."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
}
impl Test {
    pub fn new(test_id: String) -> Self {
        Self {
            pass_fail_criteria: None,
            auto_stop_criteria: None,
            secrets: None,
            certificate: None,
            environment_variables: None,
            load_test_configuration: None,
            baseline_test_run_id: None,
            input_artifacts: None,
            test_id,
            description: None,
            display_name: None,
            subnet_id: None,
            kind: None,
            public_ip_disabled: None,
            keyvault_reference_identity_type: None,
            keyvault_reference_identity_id: None,
            created_date_time: None,
            created_by: None,
            last_modified_date_time: None,
            last_modified_by: None,
        }
    }
}
#[doc = "Test app components"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestAppComponents {
    #[doc = "Azure resource collection { resource id (fully qualified resource Id e.g\nsubscriptions/{subId}/resourceGroups/{rg}/providers/Microsoft.LoadTestService/loadtests/{resName})\n: resource object } "]
    pub components: serde_json::Value,
    #[doc = "Test identifier"]
    #[serde(rename = "testId", default, skip_serializing_if = "Option::is_none")]
    pub test_id: Option<String>,
    #[doc = "The creation datetime(RFC 3339 literal format)."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<::time::OffsetDateTime>,
    #[doc = "The user that created."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The last Modified datetime(RFC 3339 literal format)."]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<::time::OffsetDateTime>,
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
#[doc = "Test app components"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestAppComponentsUpdate {
    #[doc = "Azure resource collection { resource id (fully qualified resource Id e.g\nsubscriptions/{subId}/resourceGroups/{rg}/providers/Microsoft.LoadTestService/loadtests/{resName})\n: resource object } "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<serde_json::Value>,
}
impl TestAppComponentsUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Load test model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestCreateOrUpdate {
    #[doc = "Pass fail criteria for a test."]
    #[serde(rename = "passFailCriteria", default, skip_serializing_if = "Option::is_none")]
    pub pass_fail_criteria: Option<PassFailCriteria>,
    #[doc = "Auto stop criteria for a test. This will automatically stop a load test if the error percentage is high for a certain time window."]
    #[serde(rename = "autoStopCriteria", default, skip_serializing_if = "Option::is_none")]
    pub auto_stop_criteria: Option<AutoStopCriteria>,
    #[doc = "Secrets can be stored in an Azure Key Vault or any other secret store. If the\nsecret is stored in an Azure Key Vault, the value should be the secret\nidentifier and the type should be AKV_SECRET_URI. If the secret is stored\nelsewhere, the secret value should be provided directly and the type should be\nSECRET_VALUE."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<serde_json::Value>,
    #[doc = "Certificates metadata"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<CertificateMetadata>,
    #[doc = "Environment variables which are defined as a set of <name,value> pairs."]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<serde_json::Value>,
    #[doc = "Configurations for the load test."]
    #[serde(rename = "loadTestConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub load_test_configuration: Option<LoadTestConfiguration>,
    #[doc = "Id of the test run to be marked as baseline to view trends of client-side metrics from recent test runs"]
    #[serde(rename = "baselineTestRunId", default, skip_serializing_if = "Option::is_none")]
    pub baseline_test_run_id: Option<String>,
    #[doc = "The test description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Display name of a test."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Subnet ID on which the load test instances should run."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "Test kind"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<TestKind>,
    #[doc = "Inject load test engines without deploying public IP for outbound access"]
    #[serde(rename = "publicIPDisabled", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_disabled: Option<bool>,
    #[doc = "Type of the managed identity referencing the Key vault."]
    #[serde(rename = "keyvaultReferenceIdentityType", default, skip_serializing_if = "Option::is_none")]
    pub keyvault_reference_identity_type: Option<String>,
    #[doc = "Resource Id of the managed identity referencing the Key vault."]
    #[serde(rename = "keyvaultReferenceIdentityId", default, skip_serializing_if = "Option::is_none")]
    pub keyvault_reference_identity_id: Option<String>,
}
impl TestCreateOrUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test file info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestFileInfo {
    #[doc = "Name of the file."]
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[doc = "File URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Types of file supported."]
    #[serde(rename = "fileType", default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<FileType>,
    #[doc = "Expiry time of the file (RFC 3339 literal format)"]
    #[serde(rename = "expireDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub expire_date_time: Option<::time::OffsetDateTime>,
    #[doc = "File status."]
    #[serde(rename = "validationStatus", default, skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<FileStatus>,
    #[doc = "Validation failure error details"]
    #[serde(rename = "validationFailureDetails", default, skip_serializing_if = "Option::is_none")]
    pub validation_failure_details: Option<String>,
}
impl TestFileInfo {
    pub fn new(file_name: String) -> Self {
        Self {
            file_name,
            url: None,
            file_type: None,
            expire_date_time: None,
            validation_status: None,
            validation_failure_details: None,
        }
    }
}
#[doc = "The input artifacts for the test."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestInputArtifacts {
    #[doc = "Test file info."]
    #[serde(rename = "configFileInfo", default, skip_serializing_if = "Option::is_none")]
    pub config_file_info: Option<TestFileInfo>,
    #[doc = "Test file info."]
    #[serde(rename = "testScriptFileInfo", default, skip_serializing_if = "Option::is_none")]
    pub test_script_file_info: Option<TestFileInfo>,
    #[doc = "Test file info."]
    #[serde(rename = "userPropFileInfo", default, skip_serializing_if = "Option::is_none")]
    pub user_prop_file_info: Option<TestFileInfo>,
    #[doc = "Test file info."]
    #[serde(rename = "inputArtifactsZipFileInfo", default, skip_serializing_if = "Option::is_none")]
    pub input_artifacts_zip_file_info: Option<TestFileInfo>,
    #[doc = "Test file info."]
    #[serde(rename = "urlTestConfigFileInfo", default, skip_serializing_if = "Option::is_none")]
    pub url_test_config_file_info: Option<TestFileInfo>,
    #[doc = "Additional supported files for the test run"]
    #[serde(
        rename = "additionalFileInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_file_info: Vec<TestFileInfo>,
}
impl TestInputArtifacts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test kind"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TestKind")]
pub enum TestKind {
    #[serde(rename = "URL")]
    Url,
    #[serde(rename = "JMX")]
    Jmx,
    Locust,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TestKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TestKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TestKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Url => serializer.serialize_unit_variant("TestKind", 0u32, "URL"),
            Self::Jmx => serializer.serialize_unit_variant("TestKind", 1u32, "JMX"),
            Self::Locust => serializer.serialize_unit_variant("TestKind", 2u32, "Locust"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The Test Profile Model. A Test Profile resource enables you to set up a test profile which contains various configurations for a supported resource type and a load test to execute on that resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestProfile {
    #[doc = "Unique identifier for the test profile, must contain only lower-case alphabetic, numeric, underscore or hyphen characters."]
    #[serde(rename = "testProfileId")]
    pub test_profile_id: String,
    #[doc = "Display name of the test profile."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Description for the test profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Associated test ID for the test profile. This property is required for creating a Test Profile and it's not allowed to be updated."]
    #[serde(rename = "testId", default, skip_serializing_if = "Option::is_none")]
    pub test_id: Option<String>,
    #[doc = "Target resource ID on which the test profile is created. This property is required for creating a Test Profile and it's not allowed to be updated."]
    #[serde(rename = "targetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_id: Option<String>,
    #[doc = "Configurations of a target resource. This varies with the kind of resource."]
    #[serde(rename = "targetResourceConfigurations", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_configurations: Option<TargetResourceConfigurationsUnion>,
    #[doc = "The creation datetime(RFC 3339 literal format)."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<::time::OffsetDateTime>,
    #[doc = "The user that created."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The last Modified datetime(RFC 3339 literal format)."]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<::time::OffsetDateTime>,
    #[doc = "The user that last modified."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
}
impl TestProfile {
    pub fn new(test_profile_id: String) -> Self {
        Self {
            test_profile_id,
            display_name: None,
            description: None,
            test_id: None,
            target_resource_id: None,
            target_resource_configurations: None,
            created_date_time: None,
            created_by: None,
            last_modified_date_time: None,
            last_modified_by: None,
        }
    }
}
#[doc = "The Test Profile Model. A Test Profile resource enables you to set up a test profile which contains various configurations for a supported resource type and a load test to execute on that resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestProfileCreateOrUpdate {
    #[doc = "Display name of the test profile."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Description for the test profile."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Associated test ID for the test profile. This property is required for creating a Test Profile and it's not allowed to be updated."]
    #[serde(rename = "testId", default, skip_serializing_if = "Option::is_none")]
    pub test_id: Option<String>,
    #[doc = "Target resource ID on which the test profile is created. This property is required for creating a Test Profile and it's not allowed to be updated."]
    #[serde(rename = "targetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_id: Option<String>,
    #[doc = "Configurations of a target resource. This varies with the kind of resource."]
    #[serde(rename = "targetResourceConfigurations", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_configurations: Option<TargetResourceConfigurationsCreateOrUpdateUnion>,
}
impl TestProfileCreateOrUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Test Profile Run Model. Test Profile Run resource enables you to instantiate an already created test profile and run load tests to get recommendations on the optimal configuration for the target resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestProfileRun {
    #[doc = "Unique identifier for the test profile run, must contain only lower-case alphabetic, numeric, underscore or hyphen characters."]
    #[serde(rename = "testProfileRunId")]
    pub test_profile_run_id: String,
    #[doc = "Display name for the test profile run."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The test profile run description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Associated test profile ID for the test profile run. This is required to create a test profile run and can't be updated."]
    #[serde(rename = "testProfileId", default, skip_serializing_if = "Option::is_none")]
    pub test_profile_id: Option<String>,
    #[doc = "Target resource ID on which the test profile run is created"]
    #[serde(rename = "targetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_id: Option<String>,
    #[doc = "Configurations of a target resource. This varies with the kind of resource."]
    #[serde(rename = "targetResourceConfigurations", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_configurations: Option<TargetResourceConfigurationsUnion>,
    #[doc = "Test profile run status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<TestProfileRunStatus>,
    #[doc = "Error details if there is any failure in test profile run. These errors are specific to the Test Profile Run."]
    #[serde(
        rename = "errorDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub error_details: Vec<ErrorDetails>,
    #[doc = "The test profile run start DateTime(RFC 3339 literal format)."]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<::time::OffsetDateTime>,
    #[doc = "The test profile run end DateTime(RFC 3339 literal format)."]
    #[serde(rename = "endDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_date_time: Option<::time::OffsetDateTime>,
    #[doc = "Test profile run duration in seconds."]
    #[serde(rename = "durationInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i64>,
    #[doc = "Details of the test runs ran as part of the test profile run.\nKey is the testRunId of the corresponding testRun."]
    #[serde(rename = "testRunDetails", default, skip_serializing_if = "Option::is_none")]
    pub test_run_details: Option<serde_json::Value>,
    #[doc = "Recommendations provided based on a successful test profile run."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub recommendations: Vec<TestProfileRunRecommendation>,
    #[doc = "The creation datetime(RFC 3339 literal format)."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<::time::OffsetDateTime>,
    #[doc = "The user that created."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The last Modified datetime(RFC 3339 literal format)."]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<::time::OffsetDateTime>,
    #[doc = "The user that last modified."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
}
impl TestProfileRun {
    pub fn new(test_profile_run_id: String) -> Self {
        Self {
            test_profile_run_id,
            display_name: None,
            description: None,
            test_profile_id: None,
            target_resource_id: None,
            target_resource_configurations: None,
            status: None,
            error_details: Vec::new(),
            start_date_time: None,
            end_date_time: None,
            duration_in_seconds: None,
            test_run_details: None,
            recommendations: Vec::new(),
            created_date_time: None,
            created_by: None,
            last_modified_date_time: None,
            last_modified_by: None,
        }
    }
}
#[doc = "The Test Profile Run Model. Test Profile Run resource enables you to instantiate an already created test profile and run load tests to get recommendations on the optimal configuration for the target resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestProfileRunCreateOrUpdate {
    #[doc = "Display name for the test profile run."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The test profile run description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Associated test profile ID for the test profile run. This is required to create a test profile run and can't be updated."]
    #[serde(rename = "testProfileId", default, skip_serializing_if = "Option::is_none")]
    pub test_profile_id: Option<String>,
}
impl TestProfileRunCreateOrUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A recommendation object that provides a list of configuration that optimizes its category."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestProfileRunRecommendation {
    #[doc = "Category of Recommendation."]
    pub category: RecommendationCategory,
    #[doc = "List of configurations IDs for which the recommendation is applicable. These are a subset of the provided target resource configurations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub configurations: Vec<String>,
}
impl TestProfileRunRecommendation {
    pub fn new(category: RecommendationCategory) -> Self {
        Self {
            category,
            configurations: Vec::new(),
        }
    }
}
#[doc = "Test profile run status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TestProfileRunStatus")]
pub enum TestProfileRunStatus {
    #[serde(rename = "ACCEPTED")]
    Accepted,
    #[serde(rename = "NOTSTARTED")]
    Notstarted,
    #[serde(rename = "EXECUTING")]
    Executing,
    #[serde(rename = "DONE")]
    Done,
    #[serde(rename = "CANCELLING")]
    Cancelling,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TestProfileRunStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TestProfileRunStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TestProfileRunStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Accepted => serializer.serialize_unit_variant("TestProfileRunStatus", 0u32, "ACCEPTED"),
            Self::Notstarted => serializer.serialize_unit_variant("TestProfileRunStatus", 1u32, "NOTSTARTED"),
            Self::Executing => serializer.serialize_unit_variant("TestProfileRunStatus", 2u32, "EXECUTING"),
            Self::Done => serializer.serialize_unit_variant("TestProfileRunStatus", 3u32, "DONE"),
            Self::Cancelling => serializer.serialize_unit_variant("TestProfileRunStatus", 4u32, "CANCELLING"),
            Self::Cancelled => serializer.serialize_unit_variant("TestProfileRunStatus", 5u32, "CANCELLED"),
            Self::Failed => serializer.serialize_unit_variant("TestProfileRunStatus", 6u32, "FAILED"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Load test run model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestRun {
    #[doc = "Unique test run identifier for the load test run, must contain only lower-case alphabetic, numeric, underscore or hyphen characters."]
    #[serde(rename = "testRunId")]
    pub test_run_id: String,
    #[doc = "Pass fail criteria for a test."]
    #[serde(rename = "passFailCriteria", default, skip_serializing_if = "Option::is_none")]
    pub pass_fail_criteria: Option<PassFailCriteria>,
    #[doc = "Auto stop criteria for a test. This will automatically stop a load test if the error percentage is high for a certain time window."]
    #[serde(rename = "autoStopCriteria", default, skip_serializing_if = "Option::is_none")]
    pub auto_stop_criteria: Option<AutoStopCriteria>,
    #[doc = "Secrets can be stored in an Azure Key Vault or any other secret store. If the\nsecret is stored in an Azure Key Vault, the value should be the secret\nidentifier and the type should be AKV_SECRET_URI. If the secret is stored\nelsewhere, the secret value should be provided directly and the type should be\nSECRET_VALUE."]
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
    #[doc = "Test run statistics. Key is the sampler name and value is the set of statistics for performance metrics like response time, throughput, etc. from the load test run.\nThe sampler name is the same as the name mentioned in the test script.\nSampler name \"Total\" represents the aggregated statistics of all the samplers."]
    #[serde(rename = "testRunStatistics", default, skip_serializing_if = "Option::is_none")]
    pub test_run_statistics: Option<serde_json::Value>,
    #[doc = "Regional statistics. Key is the Azure region name and value is the test run statistics.\nThe region name should of format accepted by ARM, and should be a region supported by Azure Load Testing. For example, East US should be passed as \"eastus\".\nThe region name must match one of the strings in the \"Name\" column returned from running the \"az account list-locations -o table\" Azure CLI command."]
    #[serde(rename = "regionalStatistics", default, skip_serializing_if = "Option::is_none")]
    pub regional_statistics: Option<serde_json::Value>,
    #[doc = "Configurations for the load test."]
    #[serde(rename = "loadTestConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub load_test_configuration: Option<LoadTestConfiguration>,
    #[doc = "Collection of test run artifacts"]
    #[serde(rename = "testArtifacts", default, skip_serializing_if = "Option::is_none")]
    pub test_artifacts: Option<TestRunArtifacts>,
    #[doc = "Test result based on pass/fail criteria."]
    #[serde(rename = "testResult", default, skip_serializing_if = "Option::is_none")]
    pub test_result: Option<PfTestResult>,
    #[doc = "Number of virtual users, for which test has been run."]
    #[serde(rename = "virtualUsers", default, skip_serializing_if = "Option::is_none")]
    pub virtual_users: Option<i32>,
    #[doc = "Display name of a testRun."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Associated test Id."]
    #[serde(rename = "testId", default, skip_serializing_if = "Option::is_none")]
    pub test_id: Option<String>,
    #[doc = "The test run description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Test run status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[doc = "The test run start DateTime(RFC 3339 literal format)."]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<::time::OffsetDateTime>,
    #[doc = "The test run end DateTime(RFC 3339 literal format)."]
    #[serde(rename = "endDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_date_time: Option<::time::OffsetDateTime>,
    #[doc = "Test run initiated time."]
    #[serde(rename = "executedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub executed_date_time: Option<::time::OffsetDateTime>,
    #[doc = "Portal url."]
    #[serde(rename = "portalUrl", default, skip_serializing_if = "Option::is_none")]
    pub portal_url: Option<String>,
    #[doc = "Test run duration in milliseconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[doc = "Subnet ID on which the load test instances should run."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "Test kind"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<TestKind>,
    #[doc = "Request data collection level for test run"]
    #[serde(rename = "requestDataLevel", default, skip_serializing_if = "Option::is_none")]
    pub request_data_level: Option<RequestDataLevel>,
    #[doc = "Enable or disable debug level logging. True if debug logs are enabled for the test run. False otherwise"]
    #[serde(rename = "debugLogsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub debug_logs_enabled: Option<bool>,
    #[doc = "Inject load test engines without deploying public IP for outbound access"]
    #[serde(rename = "publicIPDisabled", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_disabled: Option<bool>,
    #[doc = "The creation datetime(RFC 3339 literal format)."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<::time::OffsetDateTime>,
    #[doc = "The user that created."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The last Modified datetime(RFC 3339 literal format)."]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<::time::OffsetDateTime>,
    #[doc = "The user that last modified."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
}
impl TestRun {
    pub fn new(test_run_id: String) -> Self {
        Self {
            test_run_id,
            pass_fail_criteria: None,
            auto_stop_criteria: None,
            secrets: None,
            certificate: None,
            environment_variables: None,
            error_details: Vec::new(),
            test_run_statistics: None,
            regional_statistics: None,
            load_test_configuration: None,
            test_artifacts: None,
            test_result: None,
            virtual_users: None,
            display_name: None,
            test_id: None,
            description: None,
            status: None,
            start_date_time: None,
            end_date_time: None,
            executed_date_time: None,
            portal_url: None,
            duration: None,
            subnet_id: None,
            kind: None,
            request_data_level: None,
            debug_logs_enabled: None,
            public_ip_disabled: None,
            created_date_time: None,
            created_by: None,
            last_modified_date_time: None,
            last_modified_by: None,
        }
    }
}
#[doc = "Test run app component"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestRunAppComponents {
    #[doc = "Azure resource collection { resource id (fully qualified resource Id e.g\nsubscriptions/{subId}/resourceGroups/{rg}/providers/Microsoft.LoadTestService/loadtests/{resName})\n: resource object } "]
    pub components: serde_json::Value,
    #[doc = "Test run identifier"]
    #[serde(rename = "testRunId", default, skip_serializing_if = "Option::is_none")]
    pub test_run_id: Option<String>,
    #[doc = "The creation datetime(RFC 3339 literal format)."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<::time::OffsetDateTime>,
    #[doc = "The user that created."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The last Modified datetime(RFC 3339 literal format)."]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<::time::OffsetDateTime>,
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
#[doc = "Test run app component"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRunAppComponentsUpdate {
    #[doc = "Azure resource collection { resource id (fully qualified resource Id e.g\nsubscriptions/{subId}/resourceGroups/{rg}/providers/Microsoft.LoadTestService/loadtests/{resName})\n: resource object } "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<serde_json::Value>,
}
impl TestRunAppComponentsUpdate {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Load test run model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRunCreateOrUpdate {
    #[doc = "Pass fail criteria for a test."]
    #[serde(rename = "passFailCriteria", default, skip_serializing_if = "Option::is_none")]
    pub pass_fail_criteria: Option<PassFailCriteria>,
    #[doc = "Auto stop criteria for a test. This will automatically stop a load test if the error percentage is high for a certain time window."]
    #[serde(rename = "autoStopCriteria", default, skip_serializing_if = "Option::is_none")]
    pub auto_stop_criteria: Option<AutoStopCriteria>,
    #[doc = "Secrets can be stored in an Azure Key Vault or any other secret store. If the\nsecret is stored in an Azure Key Vault, the value should be the secret\nidentifier and the type should be AKV_SECRET_URI. If the secret is stored\nelsewhere, the secret value should be provided directly and the type should be\nSECRET_VALUE."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<serde_json::Value>,
    #[doc = "Certificates metadata"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<CertificateMetadata>,
    #[doc = "Environment variables which are defined as a set of <name,value> pairs."]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<serde_json::Value>,
    #[doc = "Configurations for the load test."]
    #[serde(rename = "loadTestConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub load_test_configuration: Option<LoadTestConfiguration>,
    #[doc = "Display name of a testRun."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Associated test Id."]
    #[serde(rename = "testId", default, skip_serializing_if = "Option::is_none")]
    pub test_id: Option<String>,
    #[doc = "The test run description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Request data collection level for test run"]
    #[serde(rename = "requestDataLevel", default, skip_serializing_if = "Option::is_none")]
    pub request_data_level: Option<RequestDataLevel>,
    #[doc = "Enable or disable debug level logging. True if debug logs are enabled for the test run. False otherwise"]
    #[serde(rename = "debugLogsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub debug_logs_enabled: Option<bool>,
}
impl TestRunCreateOrUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of a particular test run for a test profile run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestRunDetail {
    #[doc = "Test run status."]
    pub status: Status,
    #[doc = "ID of the configuration on which the test ran."]
    #[serde(rename = "configurationId")]
    pub configuration_id: String,
    #[doc = "Key value pair of extra properties associated with the test run."]
    pub properties: serde_json::Value,
}
impl TestRunDetail {
    pub fn new(status: Status, configuration_id: String, properties: serde_json::Value) -> Self {
        Self {
            status,
            configuration_id,
            properties,
        }
    }
}
#[doc = "Test run file info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestRunFileInfo {
    #[doc = "Name of the file."]
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[doc = "File URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Types of file supported."]
    #[serde(rename = "fileType", default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<FileType>,
    #[doc = "Expiry time of the file (RFC 3339 literal format)"]
    #[serde(rename = "expireDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub expire_date_time: Option<::time::OffsetDateTime>,
    #[doc = "File status."]
    #[serde(rename = "validationStatus", default, skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<FileStatus>,
    #[doc = "Validation failure error details"]
    #[serde(rename = "validationFailureDetails", default, skip_serializing_if = "Option::is_none")]
    pub validation_failure_details: Option<String>,
}
impl TestRunFileInfo {
    pub fn new(file_name: String) -> Self {
        Self {
            file_name,
            url: None,
            file_type: None,
            expire_date_time: None,
            validation_status: None,
            validation_failure_details: None,
        }
    }
}
#[doc = "The input artifacts for the test run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRunInputArtifacts {
    #[doc = "Test run file info."]
    #[serde(rename = "configFileInfo", default, skip_serializing_if = "Option::is_none")]
    pub config_file_info: Option<TestRunFileInfo>,
    #[doc = "Test run file info."]
    #[serde(rename = "testScriptFileInfo", default, skip_serializing_if = "Option::is_none")]
    pub test_script_file_info: Option<TestRunFileInfo>,
    #[doc = "Test run file info."]
    #[serde(rename = "userPropFileInfo", default, skip_serializing_if = "Option::is_none")]
    pub user_prop_file_info: Option<TestRunFileInfo>,
    #[doc = "Test run file info."]
    #[serde(rename = "inputArtifactsZipFileInfo", default, skip_serializing_if = "Option::is_none")]
    pub input_artifacts_zip_file_info: Option<TestRunFileInfo>,
    #[doc = "Test run file info."]
    #[serde(rename = "urlTestConfigFileInfo", default, skip_serializing_if = "Option::is_none")]
    pub url_test_config_file_info: Option<TestRunFileInfo>,
    #[doc = "Additional supported files for the test run"]
    #[serde(
        rename = "additionalFileInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_file_info: Vec<TestRunFileInfo>,
}
impl TestRunInputArtifacts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The output artifacts for the test run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRunOutputArtifacts {
    #[doc = "Test run file info."]
    #[serde(rename = "resultFileInfo", default, skip_serializing_if = "Option::is_none")]
    pub result_file_info: Option<TestRunFileInfo>,
    #[doc = "Test run file info."]
    #[serde(rename = "logsFileInfo", default, skip_serializing_if = "Option::is_none")]
    pub logs_file_info: Option<TestRunFileInfo>,
    #[doc = "Artifacts container info."]
    #[serde(rename = "artifactsContainerInfo", default, skip_serializing_if = "Option::is_none")]
    pub artifacts_container_info: Option<ArtifactsContainerInfo>,
    #[doc = "Test run file info."]
    #[serde(rename = "reportFileInfo", default, skip_serializing_if = "Option::is_none")]
    pub report_file_info: Option<TestRunFileInfo>,
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
    #[doc = "Azure resource metrics collection {metric id : metrics object} (Refer :\nhttps://docs.microsoft.com/en-us/rest/api/monitor/metric-definitions/list#metricdefinition\nfor metric id)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<serde_json::Value>,
    #[doc = "The creation datetime(RFC 3339 literal format)."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<::time::OffsetDateTime>,
    #[doc = "The user that created."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The last Modified datetime(RFC 3339 literal format)."]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<::time::OffsetDateTime>,
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
    #[doc = "75 percentile response time."]
    #[serde(rename = "pct75ResTime", default, skip_serializing_if = "Option::is_none")]
    pub pct75_res_time: Option<f64>,
    #[doc = "96 percentile response time."]
    #[serde(rename = "pct96ResTime", default, skip_serializing_if = "Option::is_none")]
    pub pct96_res_time: Option<f64>,
    #[doc = "97 percentile response time."]
    #[serde(rename = "pct97ResTime", default, skip_serializing_if = "Option::is_none")]
    pub pct97_res_time: Option<f64>,
    #[doc = "98 percentile response time."]
    #[serde(rename = "pct98ResTime", default, skip_serializing_if = "Option::is_none")]
    pub pct98_res_time: Option<f64>,
    #[doc = "99.9 percentile response time."]
    #[serde(rename = "pct999ResTime", default, skip_serializing_if = "Option::is_none")]
    pub pct999_res_time: Option<f64>,
    #[doc = "99.99 percentile response time."]
    #[serde(rename = "pct9999ResTime", default, skip_serializing_if = "Option::is_none")]
    pub pct9999_res_time: Option<f64>,
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
#[doc = "Test server metrics configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestServerMetricConfig {
    #[doc = "Test identifier"]
    #[serde(rename = "testId", default, skip_serializing_if = "Option::is_none")]
    pub test_id: Option<String>,
    #[doc = "Azure resource metrics collection {metric id : metrics object} (Refer :\nhttps://docs.microsoft.com/en-us/rest/api/monitor/metric-definitions/list#metricdefinition\nfor metric id)."]
    pub metrics: serde_json::Value,
    #[doc = "The creation datetime(RFC 3339 literal format)."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<::time::OffsetDateTime>,
    #[doc = "The user that created."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The last Modified datetime(RFC 3339 literal format)."]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<::time::OffsetDateTime>,
    #[doc = "The user that last modified."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
}
impl TestServerMetricConfig {
    pub fn new(metrics: serde_json::Value) -> Self {
        Self {
            test_id: None,
            metrics,
            created_date_time: None,
            created_by: None,
            last_modified_date_time: None,
            last_modified_by: None,
        }
    }
}
#[doc = "Test server metrics configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestServerMetricConfigUpdate {
    #[doc = "Azure resource metrics collection {metric id : metrics object} (Refer :\nhttps://docs.microsoft.com/en-us/rest/api/monitor/metric-definitions/list#metricdefinition\nfor metric id)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<serde_json::Value>,
}
impl TestServerMetricConfigUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Time Grain"]
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
