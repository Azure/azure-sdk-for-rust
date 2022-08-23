#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An Azure resource object (Refer azure generic resource model : https://docs.microsoft.com/en-us/rest/api/resources/resources/get-by-id#genericresource)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppComponent {
    #[doc = "Fully qualified resource Id e.g subscriptions/{subId}/resourceGroups/{rg}/providers/Microsoft.LoadTestService/loadtests/{resName}"]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "Azure resource name"]
    #[serde(rename = "resourceName")]
    pub resource_name: String,
    #[doc = "Azure resource type"]
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
#[doc = "App Components model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppComponentsMap {
    #[doc = "Azure Load Testing resource Id"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "[Required, if testRunId is not given] Load test unique identifier"]
    #[serde(rename = "testId", default, skip_serializing_if = "Option::is_none")]
    pub test_id: Option<String>,
    #[doc = "[Required if testId is not given] Load test run unique identifier"]
    #[serde(rename = "testRunId", default, skip_serializing_if = "Option::is_none")]
    pub test_run_id: Option<String>,
    #[doc = "AppComponent name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "AppComponents Map { resource id (Fully qualified resource Id e.g subscriptions/{subId}/resourceGroups/{rg}/providers/Microsoft.LoadTestService/loadtests/{resName}) : resource object } "]
    pub value: serde_json::Value,
}
impl AppComponentsMap {
    pub fn new(value: serde_json::Value) -> Self {
        Self {
            resource_id: None,
            test_id: None,
            test_run_id: None,
            name: None,
            value,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientMetricsFilters {
    #[doc = "Test run name for which client metrics filters is required."]
    #[serde(rename = "testRunId", default, skip_serializing_if = "Option::is_none")]
    pub test_run_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Filters>,
    #[serde(rename = "timeRange", default, skip_serializing_if = "Option::is_none")]
    pub time_range: Option<TimeRange>,
}
impl ClientMetricsFilters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Client metrics request payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientMetricsRequestModel {
    #[doc = "List of request samplers, maximum supported samplers for queries are 20. In case of empty, it will return metrics for maximum 20 samplers"]
    #[serde(rename = "requestSamplers", default, skip_serializing_if = "Vec::is_empty")]
    pub request_samplers: Vec<String>,
    #[doc = "List of errors, maximum supported errors for queries are 20. In case of empty, by default will return metrics for maximum 20 errors"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
    #[doc = "List of percentiles values for response time, supported values 50,90,99,95. Default value is 50th percentile."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub percentiles: Vec<String>,
    #[doc = "For test duration less than 10 minutes group by time interval can be any one of 5s,10s,1m,5m.\\n\\nFor test duration greater than 10 minutes, group by time interval can be any one of 1m,5m,1h. Default value is 1m."]
    #[serde(rename = "groupByInterval", default, skip_serializing_if = "Option::is_none")]
    pub group_by_interval: Option<String>,
    #[doc = "Start time"]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339")]
    pub start_time: time::OffsetDateTime,
    #[doc = "End time"]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339")]
    pub end_time: time::OffsetDateTime,
}
impl ClientMetricsRequestModel {
    pub fn new(start_time: time::OffsetDateTime, end_time: time::OffsetDateTime) -> Self {
        Self {
            request_samplers: Vec::new(),
            errors: Vec::new(),
            percentiles: Vec::new(),
            group_by_interval: None,
            start_time,
            end_time,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClientMetricsResults {
    #[doc = "Test run name for which client metrics results is required."]
    #[serde(rename = "testRunId", default, skip_serializing_if = "Option::is_none")]
    pub test_run_id: Option<String>,
    #[serde(rename = "timeSeries", default, skip_serializing_if = "Option::is_none")]
    pub time_series: Option<Series>,
}
impl ClientMetricsResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Default server metrics config"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DefaultServerMetricsConfigListModel {
    #[doc = "Default metrics map {resourceType : list of metrics config} (Refer for metrics structure: https://docs.microsoft.com/en-us/rest/api/monitor/metric-definitions/list#metricdefinition)"]
    #[serde(rename = "defaultMetrics", default, skip_serializing_if = "Option::is_none")]
    pub default_metrics: Option<serde_json::Value>,
}
impl DefaultServerMetricsConfigListModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metrics config model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DefaultServerMetricsConfigModel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metricnamespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<LocalizedName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
}
impl DefaultServerMetricsConfigModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error from a REST request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Additional details and inner errors."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<Error>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The definition of an error object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseBody {
    #[doc = "Error from a REST request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}
impl ErrorResponseBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Integer representation of the file type (0 = JMX_FILE, 1 = USER_PROPERTIES, 2 = ADDITIONAL_ARTIFACTS)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FileType {}
#[doc = "FileUrl Model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileUrl {
    #[doc = "File URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "File unique identifier."]
    #[serde(rename = "fileId", default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[doc = "Name of the file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[doc = "Integer representation of the file type (0 = JMX_FILE, 1 = USER_PROPERTIES, 2 = ADDITIONAL_ARTIFACTS)"]
    #[serde(rename = "fileType", default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<FileType>,
    #[doc = "Expiry time of the file"]
    #[serde(rename = "expireTime", default, with = "azure_core::date::rfc3339::option")]
    pub expire_time: Option<time::OffsetDateTime>,
    #[doc = "Validation status of the file"]
    #[serde(rename = "validationStatus", default, skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
}
impl FileUrl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileUrlList {
    #[doc = "List of file URLs."]
    pub value: Vec<FileUrl>,
    #[doc = "Link for the next list of file URLs, if applicable"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl FileUrlList {
    pub fn new(value: Vec<FileUrl>) -> Self {
        Self { value, next_link: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Filters {
    #[doc = "List of request sampler for the test run, for which client metrics can be filtered."]
    #[serde(rename = "requestSamplerValues", default, skip_serializing_if = "Vec::is_empty")]
    pub request_sampler_values: Vec<String>,
    #[doc = "List of errors occurred for the test run, for which client metrics can be filtered."]
    #[serde(rename = "errorFiltersValues", default, skip_serializing_if = "Vec::is_empty")]
    pub error_filters_values: Vec<String>,
}
impl Filters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The input artifacts for the test."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InputTestArtifacts {
    #[doc = "FileUrl Model."]
    #[serde(rename = "configUrl", default, skip_serializing_if = "Option::is_none")]
    pub config_url: Option<FileUrl>,
    #[doc = "FileUrl Model."]
    #[serde(rename = "testScriptUrl", default, skip_serializing_if = "Option::is_none")]
    pub test_script_url: Option<FileUrl>,
    #[doc = "FileUrl Model."]
    #[serde(rename = "userPropUrl", default, skip_serializing_if = "Option::is_none")]
    pub user_prop_url: Option<FileUrl>,
    #[doc = "FileUrl Model."]
    #[serde(rename = "inputArtifactsZipFileurl", default, skip_serializing_if = "Option::is_none")]
    pub input_artifacts_zip_fileurl: Option<FileUrl>,
    #[doc = "The input artifacts file { name : url } map for the test run."]
    #[serde(rename = "additionalUrls", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_urls: Vec<FileUrl>,
}
impl InputTestArtifacts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The load test configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadTestConfig {
    #[doc = "The number of engine instances to execute load test. Supported values are in range of 1-45. Required for creating a new test."]
    #[serde(rename = "engineInstances", default, skip_serializing_if = "Option::is_none")]
    pub engine_instances: Option<i32>,
    #[doc = "Whether all the input CSV files should be split evenly across all engines."]
    #[serde(rename = "splitAllCSVs", default, skip_serializing_if = "Option::is_none")]
    pub split_all_cs_vs: Option<bool>,
}
impl LoadTestConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocalizedName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl LocalizedName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The output artifacts for the test run."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutputTestArtifacts {
    #[doc = "FileUrl Model."]
    #[serde(rename = "resultUrl", default, skip_serializing_if = "Option::is_none")]
    pub result_url: Option<FileUrl>,
    #[doc = "FileUrl Model."]
    #[serde(rename = "logsUrl", default, skip_serializing_if = "Option::is_none")]
    pub logs_url: Option<FileUrl>,
}
impl OutputTestArtifacts {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PassFailMetric {
    #[doc = "The client metric on which the criteria should be applied. Allowed values - ‘response_time_ms’ , ‘latency’, ‘error’, ‘requests’, ‘requests_per_sec’."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clientmetric: Option<String>,
    #[doc = "The aggregation function to be applied on the client metric. Allowed functions - ‘percentage’ - for error metric ,‘avg’, ‘p50’, ‘p90’, ‘p95’, ‘p99’, ‘min’, ‘max’ - for response_time_ms and latency metric, ‘avg’ - for requests_per_sec, ‘count’ - for requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<String>,
    #[doc = "The comparison operator. Supported types ‘>’ "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "Request name for which the Pass fail criteria has to be applied."]
    #[serde(rename = "requestName", default, skip_serializing_if = "Option::is_none")]
    pub request_name: Option<String>,
    #[doc = "The value to compare with the client metric. Allowed values - ‘error : [0.0 , 100.0] unit- % ’, response_time_ms and latency : any integer value unit- ms."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[doc = "Either ‘stop’ or ‘continue’ after the threshold is met. Default is ‘continue’."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[doc = "The actual value of the client metric for the test run."]
    #[serde(rename = "actualValue", default, skip_serializing_if = "Option::is_none")]
    pub actual_value: Option<f64>,
    #[doc = "Outcome of the test run. possible outcome - ‘passed’ , ‘failed’ , ‘undetermined’."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}
impl PassFailMetric {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Associated metric definition for particular metrics of the azure resource ( Refer : https://docs.microsoft.com/en-us/rest/api/monitor/metric-definitions/list#metricdefinition)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceMetricModel {
    #[doc = "Unique identifier for metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource Id."]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "Metric name space."]
    pub metricnamespace: String,
    #[doc = "Metric description."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "Metric name object."]
    pub name: ServerMetricName,
    #[doc = "Metric aggregation."]
    pub aggregation: String,
    #[doc = "Metric unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Azure resource type."]
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}
impl ResourceMetricModel {
    pub fn new(resource_id: String, metricnamespace: String, name: ServerMetricName, aggregation: String, resource_type: String) -> Self {
        Self {
            id: None,
            resource_id,
            metricnamespace,
            display_description: None,
            name,
            aggregation,
            unit: None,
            resource_type,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretMetadata {
    #[doc = "The value of the secret, of type AKV_SECRET_URI or SECRET_VALUE"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Type of secret. eg. AKV_SECRET_URI/SECRET_VALUE"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl SecretMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Series {
    #[doc = "Active users time series data."]
    #[serde(rename = "activeUsers", default, skip_serializing_if = "Option::is_none")]
    pub active_users: Option<serde_json::Value>,
    #[doc = "Response time, time series data."]
    #[serde(rename = "responseTime", default, skip_serializing_if = "Option::is_none")]
    pub response_time: Option<serde_json::Value>,
    #[doc = "Throughput time series data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throughput: Option<serde_json::Value>,
    #[doc = "Errors time series data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<serde_json::Value>,
}
impl Series {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metric name object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerMetricName {
    #[doc = "Metric name value."]
    pub value: String,
    #[doc = "Metric localized name."]
    #[serde(rename = "localizedValue")]
    pub localized_value: String,
}
impl ServerMetricName {
    pub fn new(value: String, localized_value: String) -> Self {
        Self { value, localized_value }
    }
}
#[doc = "Server metrics config model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerMetricsModel {
    #[doc = "Server metrics config name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "[Required, if testRunId is not given] Load test unique identifier"]
    #[serde(rename = "testId", default, skip_serializing_if = "Option::is_none")]
    pub test_id: Option<String>,
    #[doc = "[Required, if testId is not given] Load test run unique identifier"]
    #[serde(rename = "testRunId", default, skip_serializing_if = "Option::is_none")]
    pub test_run_id: Option<String>,
    #[doc = "Metrics map {metric id : metrics object} (Refer : https://docs.microsoft.com/en-us/rest/api/monitor/metric-definitions/list#metricdefinition for metric id)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<serde_json::Value>,
}
impl ServerMetricsModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Supported azure resource types for App Component like Microsoft.LoadTestService/loadtests, Microsoft.ClassicCompute, Microsoft.ClassicStorage etc. (Refer for full list of available resource types in azure : https://docs.microsoft.com/en-us/azure/azure-resource-manager/management/resource-providers-and-types, some of them we are supporting for server side metrics configuration)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedResourceType {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<String>,
}
impl SupportedResourceType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestArtifacts {
    #[doc = "The input artifacts for the test."]
    #[serde(rename = "inputArtifacts")]
    pub input_artifacts: InputTestArtifacts,
    #[doc = "The output artifacts for the test run."]
    #[serde(rename = "outputArtifacts", default, skip_serializing_if = "Option::is_none")]
    pub output_artifacts: Option<OutputTestArtifacts>,
}
impl TestArtifacts {
    pub fn new(input_artifacts: InputTestArtifacts) -> Self {
        Self {
            input_artifacts,
            output_artifacts: None,
        }
    }
}
#[doc = "Load test model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestModel {
    #[doc = "Unique test name as identifier."]
    #[serde(rename = "testId", default, skip_serializing_if = "Option::is_none")]
    pub test_id: Option<String>,
    #[doc = "The test description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Display name of a test."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Fully qualified resource Id e.g /subscriptions/{subId}/resourceGroups/{rg}/providers/Microsoft.LoadTestService/loadtests/{resName}."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The load test configuration."]
    #[serde(rename = "loadTestConfig", default, skip_serializing_if = "Option::is_none")]
    pub load_test_config: Option<LoadTestConfig>,
    #[doc = "Pass fail criteria for a test."]
    #[serde(rename = "passFailCriteria", default, skip_serializing_if = "Option::is_none")]
    pub pass_fail_criteria: Option<PassFailCriteria>,
    #[doc = "The created DateTime(ISO 8601 literal format) of the test model."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "The user that created the test model."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The last Modified DateTime(ISO 8601 literal format) of the test model."]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "The user that last modified the test model."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The input artifacts for the test."]
    #[serde(rename = "inputArtifacts", default, skip_serializing_if = "Option::is_none")]
    pub input_artifacts: Option<InputTestArtifacts>,
    #[doc = "Secrets can be stored in an Azure Key Vault or any other secret store. If the secret is stored in an Azure Key Vault, the value should be the secret identifier and the type should be AKV_SECRET_URI. If the secret is stored elsewhere, the secret value should be provided directly and the type should be SECRET_VALUE."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<serde_json::Value>,
    #[doc = "Environment variables which are defined as a set of <name,value> pairs."]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<serde_json::Value>,
    #[doc = "Subnet ID on which the load test instances should run."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "Type of the managed identity referencing the Key vault."]
    #[serde(rename = "keyvaultReferenceIdentityType", default, skip_serializing_if = "Option::is_none")]
    pub keyvault_reference_identity_type: Option<String>,
    #[doc = "Resource Id of the managed identity referencing the Key vault."]
    #[serde(rename = "keyvaultReferenceIdentityId", default, skip_serializing_if = "Option::is_none")]
    pub keyvault_reference_identity_id: Option<String>,
}
impl TestModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestModelResourceList {
    #[doc = "List of Resources"]
    pub value: Vec<TestModel>,
    #[doc = "Link for the next list of resources in case of paginated results, if applicable"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl TestModelResourceList {
    pub fn new(value: Vec<TestModel>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Load test run model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRunModel {
    #[doc = "Unique test run name as identifier."]
    #[serde(rename = "testRunId", default, skip_serializing_if = "Option::is_none")]
    pub test_run_id: Option<String>,
    #[doc = "Display name of a test run."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Associated test Id."]
    #[serde(rename = "testId", default, skip_serializing_if = "Option::is_none")]
    pub test_id: Option<String>,
    #[doc = "Load test resource Id."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The test run description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The test run status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The test run start DateTime(ISO 8601 literal format)."]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "The test run end DateTime(ISO 8601 literal format)."]
    #[serde(rename = "endDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_date_time: Option<time::OffsetDateTime>,
    #[doc = "The load test configuration."]
    #[serde(rename = "loadTestConfig", default, skip_serializing_if = "Option::is_none")]
    pub load_test_config: Option<LoadTestConfig>,
    #[doc = "Test result for pass/Fail criteria used during the test run. possible outcome - ‘Passed’ , ‘Failed’ , ‘Not Applicable’."]
    #[serde(rename = "testResult", default, skip_serializing_if = "Option::is_none")]
    pub test_result: Option<String>,
    #[doc = "Pass fail criteria for a test."]
    #[serde(rename = "passFailCriteria", default, skip_serializing_if = "Option::is_none")]
    pub pass_fail_criteria: Option<PassFailCriteria>,
    #[serde(rename = "testArtifacts", default, skip_serializing_if = "Option::is_none")]
    pub test_artifacts: Option<TestArtifacts>,
    #[doc = "Test run initiated time"]
    #[serde(rename = "executedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub executed_date_time: Option<time::OffsetDateTime>,
    #[doc = "Number of virtual users, for which test has been run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vusers: Option<i32>,
    #[doc = "Test run statistics"]
    #[serde(rename = "testRunStatistics", default, skip_serializing_if = "Option::is_none")]
    pub test_run_statistics: Option<serde_json::Value>,
    #[doc = "The created DateTime(ISO 8601 literal format) of the test run."]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "The user that created the test run."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The last updated  DateTime(ISO 8601 literal format) of the test run."]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "The user that updated the test run."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "Portal url."]
    #[serde(rename = "portalUrl", default, skip_serializing_if = "Option::is_none")]
    pub portal_url: Option<String>,
    #[doc = "Secrets can be stored in an Azure Key Vault or any other secret store. If the secret is stored in an Azure Key Vault, the value should be the secret identifier and the type should be AKV_SECRET_URI. If the secret is stored elsewhere, the secret value should be provided directly and the type should be SECRET_VALUE."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<serde_json::Value>,
    #[doc = "Environment variables which are defined as a set of <name,value> pairs."]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<serde_json::Value>,
    #[doc = "Test run duration in milliseconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[doc = "Subnet ID on which the load test instances should run."]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}
impl TestRunModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestRunModelResourceList {
    #[doc = "List of Resources"]
    pub value: Vec<TestRunModel>,
    #[doc = "Link for the next list of resources in case of paginated results, if applicable"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl TestRunModelResourceList {
    pub fn new(value: Vec<TestRunModel>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Test run statistics."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRunStatisticsModel {
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
    #[doc = "Sent network bytes."]
    #[serde(rename = "sentKBytesPerSec", default, skip_serializing_if = "Option::is_none")]
    pub sent_k_bytes_per_sec: Option<f64>,
}
impl TestRunStatisticsModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeRange {
    #[doc = "start DateTime(ISO 8601 literal format) for the requested client metrics filter."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "end DateTime(ISO 8601 literal format) for the requested client metrics filter."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
}
impl TimeRange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TimeSeries {
    #[doc = "Timestamp(ISO 8601 literal format)."]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "Value at timestamp."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
impl TimeSeries {
    pub fn new() -> Self {
        Self::default()
    }
}
