#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The Azure resource object (Refer azure generic resource model : https://docs.microsoft.com/en-us/rest/api/resources/resources/get-by-id#genericresource)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppComponent {
    #[doc = "[ReadOnly] Fully qualified resource Id e.g subscriptions/{subId}/resourceGroups/{rg}/providers/Microsoft.LoadTestService/loadtests/{resName}"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Azure resource name"]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "Azure resource type"]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "[ReadOnly] Azure resource display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "[ReadOnly] Resource group name of Azure resource"]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "[ReadOnly] Subscription Id of Azure resource"]
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
#[doc = "App components model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppComponentsMap {
    #[doc = "[ReadOnly] Load test resource id"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "[Required if testRunId is not given] Load test unique identifier"]
    #[serde(rename = "testId", default, skip_serializing_if = "Option::is_none")]
    pub test_id: Option<String>,
    #[doc = "[Required if testId is not given] Load test run unique identifier"]
    #[serde(rename = "testRunId", default, skip_serializing_if = "Option::is_none")]
    pub test_run_id: Option<String>,
    #[doc = "[ReadOnly] AppComponent name"]
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
#[doc = "The definition of an error object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DefaultErrorResponseBody {
    #[doc = "The error response object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponseBody>,
}
impl DefaultErrorResponseBody {
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
#[doc = "The error response object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseBody {
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
    pub details: Vec<ErrorResponseBody>,
}
impl ErrorResponseBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "FileUrl Model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileUrl {
    #[doc = "File url"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "File unique identifier"]
    #[serde(rename = "fileId", default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[doc = "Name of the file"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[doc = "Expiry time of the file"]
    #[serde(rename = "expireTime", default, with = "azure_core::date::rfc3339::option")]
    pub expire_time: Option<time::OffsetDateTime>,
}
impl FileUrl {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileUrlList {
    #[doc = "List of file urls"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FileUrl>,
}
impl FileUrlList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileUrlPath {
    #[doc = "Path of the file"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "url of the file"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Expiry time of the file"]
    #[serde(rename = "expireTime", default, with = "azure_core::date::rfc3339::option")]
    pub expire_time: Option<time::OffsetDateTime>,
}
impl FileUrlPath {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "File validation response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FileValidateResponse {
    #[doc = "File is valid or not"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
    #[doc = "In case file is not valid then error message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
impl FileValidateResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Filters {
    #[doc = "List of request sampler for the test run, for which client metrics can be filtered"]
    #[serde(rename = "requestSamplerValues", default, skip_serializing_if = "Vec::is_empty")]
    pub request_sampler_values: Vec<String>,
    #[doc = "List of errors occurred for the test run, for which client metrics can be filtered"]
    #[serde(rename = "errorFiltersValues", default, skip_serializing_if = "Vec::is_empty")]
    pub error_filters_values: Vec<String>,
}
impl Filters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "[ReadOnly]The input artifacts for the test"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InputTestArtifacts {
    #[doc = "FileUrl Model"]
    #[serde(rename = "configUrl", default, skip_serializing_if = "Option::is_none")]
    pub config_url: Option<FileUrl>,
    #[doc = "FileUrl Model"]
    #[serde(rename = "testScriptUrl", default, skip_serializing_if = "Option::is_none")]
    pub test_script_url: Option<FileUrl>,
    #[doc = "FileUrl Model"]
    #[serde(rename = "inputArtifactsZipFileurl", default, skip_serializing_if = "Option::is_none")]
    pub input_artifacts_zip_fileurl: Option<FileUrl>,
    #[doc = "[ReadOnly]The input artifacts file { name : url } map for the test run"]
    #[serde(rename = "additionalUrls", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_urls: Vec<FileUrl>,
}
impl InputTestArtifacts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The load test configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadTestConfig {
    #[doc = "The number of engine instances to execute load test. Supported values are in range of 1-50. Required for creating a new Test."]
    #[serde(rename = "engineInstances", default, skip_serializing_if = "Option::is_none")]
    pub engine_instances: Option<i32>,
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
#[doc = "The output artifacts for the test run"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputTestArtifacts {
    #[doc = "FileUrl Model"]
    #[serde(rename = "resultUrl")]
    pub result_url: FileUrl,
    #[doc = "FileUrl Model"]
    #[serde(rename = "logsUrl")]
    pub logs_url: FileUrl,
}
impl OutputTestArtifacts {
    pub fn new(result_url: FileUrl, logs_url: FileUrl) -> Self {
        Self { result_url, logs_url }
    }
}
#[doc = "Pass fail criteria for a test"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PassFailCriteria {
    #[doc = "Map of id and pass fail metrics { id  : pass fail metrics }"]
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
    #[doc = "The client metric on which the criteria should be applied. Allowed values - ‘response_time_ms’ , ‘latency’, ‘error’ "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clientmetric: Option<String>,
    #[doc = "The aggregation function to be applied on the client metric. Allowed functions - ‘percentage’ - for error metric , ‘avg’ - for response_time_ms and latency metric "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregate: Option<String>,
    #[doc = "The comparison operator. Supported types ‘>’ "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[doc = "The value to compare with the client metric. Allowed values - ‘error : [0.0 , 100.0] unit- % ’, response_time_ms and latency : any integer value unit- ms"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[doc = "Either ‘stop’ or ‘continue’ after the threshold is met. Default is ‘continue’"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[doc = "[ReadOnly] The actual value of the client metric for the test run "]
    #[serde(rename = "actualValue", default, skip_serializing_if = "Option::is_none")]
    pub actual_value: Option<f64>,
    #[doc = "[ReadOnly] Outcome of the testRun. possible outcome - ‘passed’ , ‘failed’ , ‘undetermined’ "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}
impl PassFailMetric {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Associated metric definition for particular metrics of the azure resource ( Refer : https://docs.microsoft.com/en-us/rest/api/monitor/metric-definitions/list#metricdefinition)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceMetricModel {
    #[doc = "[ReadOnly]  Unique identifier for metric"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource id"]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "Metric name space"]
    pub metricnamespace: String,
    #[doc = "Metric description"]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "Metric name object"]
    pub name: ServerMetricName,
    #[doc = "Metric aggregation"]
    pub aggregation: String,
    #[doc = "Metric unit"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Azure resource type"]
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
    #[doc = "The value of the secret. of type== akv-secret-uri or secret-value"]
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
    #[doc = "Active users time series data"]
    #[serde(rename = "activeUsers", default, skip_serializing_if = "Option::is_none")]
    pub active_users: Option<serde_json::Value>,
    #[doc = "Response time, time series data"]
    #[serde(rename = "responseTime", default, skip_serializing_if = "Option::is_none")]
    pub response_time: Option<serde_json::Value>,
    #[doc = "Throughput time series data"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throughput: Option<serde_json::Value>,
    #[doc = "Errors time series data"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<serde_json::Value>,
}
impl Series {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metric name object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerMetricName {
    #[doc = "Metric name value"]
    pub value: String,
    #[doc = "Metric localized name"]
    #[serde(rename = "localizedValue")]
    pub localized_value: String,
}
impl ServerMetricName {
    pub fn new(value: String, localized_value: String) -> Self {
        Self { value, localized_value }
    }
}
#[doc = "Server metrics config model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerMetricsModel {
    #[doc = "[ReadOnly] Server metrics config name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "[Required if testRunId is not given] Load test unique identifier"]
    #[serde(rename = "testId", default, skip_serializing_if = "Option::is_none")]
    pub test_id: Option<String>,
    #[doc = "[Required if testId is not given] Load test run unique identifier"]
    #[serde(rename = "testRunId", default, skip_serializing_if = "Option::is_none")]
    pub test_run_id: Option<String>,
    #[doc = "Metrics map {metric id : metrics object} (Refer : https://docs.microsoft.com/en-us/rest/api/monitor/metric-definitions/list#metricdefinition for metric id)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<serde_json::Value>,
}
impl ServerMetricsModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Supported azure resource types for app component like Microsoft.LoadTestService/loadtests, Microsoft.ClassicCompute, Microsoft.ClassicStorage etc. (Refer for full list of available resource types in azure : https://docs.microsoft.com/en-us/azure/azure-resource-manager/management/resource-providers-and-types, some of them we are supporting for server side metrics configuration)"]
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
    #[doc = "[ReadOnly]The input artifacts for the test"]
    #[serde(rename = "inputArtifacts")]
    pub input_artifacts: InputTestArtifacts,
    #[doc = "The output artifacts for the test run"]
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
    #[doc = "[ReadOnly] Unique test name as identifier"]
    #[serde(rename = "testId", default, skip_serializing_if = "Option::is_none")]
    pub test_id: Option<String>,
    #[doc = "The test description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Display name of a test "]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "[ReadOnly] Fully qualified resource Id e.g /subscriptions/{subId}/resourceGroups/{rg}/providers/Microsoft.LoadTestService/loadtests/{resName}"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The load test configuration"]
    #[serde(rename = "loadTestConfig", default, skip_serializing_if = "Option::is_none")]
    pub load_test_config: Option<LoadTestConfig>,
    #[doc = "Pass fail criteria for a test"]
    #[serde(rename = "passFailCriteria", default, skip_serializing_if = "Option::is_none")]
    pub pass_fail_criteria: Option<PassFailCriteria>,
    #[doc = "[ReadOnly] The created DateTime(Iso8601Literal format) of the test model"]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "[ReadOnly] The user that created the test model"]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "[ReadOnly] The last Modified DateTime(Iso8601Literal format) of the test model"]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "[ReadOnly] The user that last modified the test model"]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "[ReadOnly]The input artifacts for the test"]
    #[serde(rename = "inputArtifacts", default, skip_serializing_if = "Option::is_none")]
    pub input_artifacts: Option<InputTestArtifacts>,
    #[doc = "Secrets of higher importance , which are stored in a key-vault typically. Allowed secret type: AKV_SECRET_URI"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<serde_json::Value>,
    #[doc = "Secrets of relatively lesser importance , which are defined as a set of <name,value> pairs."]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<serde_json::Value>,
}
impl TestModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestModelResourceList {
    #[doc = "List of Resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TestModel>,
    #[doc = "Continuation token for list of resources in case of paginated results, if applicable"]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
impl TestModelResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Load test run model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRunModel {
    #[doc = "[ReadOnly] Unique test run name as identifier"]
    #[serde(rename = "testRunId", default, skip_serializing_if = "Option::is_none")]
    pub test_run_id: Option<String>,
    #[doc = "Display name of a testRun "]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Associated testId"]
    #[serde(rename = "testId", default, skip_serializing_if = "Option::is_none")]
    pub test_id: Option<String>,
    #[doc = "[ReadOnly] Load test resource Id"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The test run description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "[ReadOnly] The test run status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "[ReadOnly] The test run start DateTime(Iso8601Literal format)"]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
    #[doc = "[ReadOnly] The test run end DateTime(Iso8601Literal format)"]
    #[serde(rename = "endDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_date_time: Option<time::OffsetDateTime>,
    #[doc = "The load test configuration"]
    #[serde(rename = "loadTestConfig", default, skip_serializing_if = "Option::is_none")]
    pub load_test_config: Option<LoadTestConfig>,
    #[doc = "[ReadOnly] Test result for pass/Fail criteria used during the test run. possible outcome - ‘Passed’ , ‘Failed’ , ‘Not Applicable’"]
    #[serde(rename = "testResult", default, skip_serializing_if = "Option::is_none")]
    pub test_result: Option<String>,
    #[doc = "Pass fail criteria for a test"]
    #[serde(rename = "passFailCriteria", default, skip_serializing_if = "Option::is_none")]
    pub pass_fail_criteria: Option<PassFailCriteria>,
    #[serde(rename = "testArtifacts", default, skip_serializing_if = "Option::is_none")]
    pub test_artifacts: Option<TestArtifacts>,
    #[doc = "[ReadOnly] Test run initiated time"]
    #[serde(rename = "executedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub executed_date_time: Option<time::OffsetDateTime>,
    #[doc = "[ReadOnly] No of virtual users, for which test has been run"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vusers: Option<i32>,
    #[doc = "[ReadOnly] Testrun statistics"]
    #[serde(rename = "testRunStatistics", default, skip_serializing_if = "Option::is_none")]
    pub test_run_statistics: Option<serde_json::Value>,
    #[doc = "[ReadOnly] The created DateTime(Iso8601Literal format) of the test run"]
    #[serde(rename = "createdDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_date_time: Option<time::OffsetDateTime>,
    #[doc = "[ReadOnly] The user that created the test run"]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "[ReadOnly] The last updated  DateTime(Iso8601Literal format) of the test run"]
    #[serde(rename = "lastModifiedDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_date_time: Option<time::OffsetDateTime>,
    #[doc = "[ReadOnly] The user that updated the test run"]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "[ReadOnly] Portal url "]
    #[serde(rename = "portalUrl", default, skip_serializing_if = "Option::is_none")]
    pub portal_url: Option<String>,
    #[doc = "Secrets of higher importance , which are stored in a key-vault typically. Allowed secret types: AKV_SECRET_URI/SECRET_VALUE"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<serde_json::Value>,
    #[doc = "Secrets of relatively lesser importance , which are defined as a set of <name,value> pairs."]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<serde_json::Value>,
    #[doc = "[ReadOnly] Test run duration in milliseconds"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
}
impl TestRunModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of Resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRunModelResourceList {
    #[doc = "List of Resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TestRunModel>,
    #[doc = "Continuation token for list of resources in case of paginated results, if applicable"]
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
impl TestRunModelResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "[Readonly] Test run statistics"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestRunStatisticsModel {
    #[doc = "[Readonly] Transaction name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction: Option<String>,
    #[doc = "[Readonly] Sampler count"]
    #[serde(rename = "sampleCount", default, skip_serializing_if = "Option::is_none")]
    pub sample_count: Option<f64>,
    #[doc = "[Readonly] Error count"]
    #[serde(rename = "errorCount", default, skip_serializing_if = "Option::is_none")]
    pub error_count: Option<f64>,
    #[doc = "[Readonly] Error percentage"]
    #[serde(rename = "errorPct", default, skip_serializing_if = "Option::is_none")]
    pub error_pct: Option<f64>,
    #[doc = "[Readonly] Mean response time"]
    #[serde(rename = "meanResTime", default, skip_serializing_if = "Option::is_none")]
    pub mean_res_time: Option<f64>,
    #[doc = "[Readonly] Median response time"]
    #[serde(rename = "medianResTime", default, skip_serializing_if = "Option::is_none")]
    pub median_res_time: Option<f64>,
    #[doc = "[Readonly] Max response time"]
    #[serde(rename = "maxResTime", default, skip_serializing_if = "Option::is_none")]
    pub max_res_time: Option<f64>,
    #[doc = "[Readonly] Minimum response time"]
    #[serde(rename = "minResTime", default, skip_serializing_if = "Option::is_none")]
    pub min_res_time: Option<f64>,
    #[doc = "[Readonly] 90 percentile response time"]
    #[serde(rename = "pct1ResTime", default, skip_serializing_if = "Option::is_none")]
    pub pct1_res_time: Option<f64>,
    #[doc = "[Readonly] 95 percentile response time"]
    #[serde(rename = "pct2ResTime", default, skip_serializing_if = "Option::is_none")]
    pub pct2_res_time: Option<f64>,
    #[doc = "[Readonly] 99 percentile response time"]
    #[serde(rename = "pct3ResTime", default, skip_serializing_if = "Option::is_none")]
    pub pct3_res_time: Option<f64>,
    #[doc = "[Readonly] Throughput"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throughput: Option<f64>,
    #[doc = "[Readonly] Received network bytes"]
    #[serde(rename = "receivedKBytesPerSec", default, skip_serializing_if = "Option::is_none")]
    pub received_k_bytes_per_sec: Option<f64>,
    #[doc = "[Readonly] Send network bytes"]
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
    #[doc = "start DateTime(Iso8601Literal format) for the requested client metrics filter"]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "end DateTime(Iso8601Literal format) for the requested client metrics filter"]
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
    #[doc = "Timestamp(Iso8601Literal format)"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "Value at timestamp"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
impl TimeSeries {
    pub fn new() -> Self {
        Self::default()
    }
}
