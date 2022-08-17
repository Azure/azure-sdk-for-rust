#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
pub type ApplicationsParam = Vec<String>;
#[doc = "A column in a table."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Column {
    #[doc = "The name of this column."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The data type of this column."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Column {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[doc = "The error's code."]
    pub code: String,
    #[doc = "A human readable error message."]
    pub message: String,
    #[doc = "Indicates which property in the request is responsible for the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Indicates which value in 'target' is responsible for the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Indicates resources which were responsible for the error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
    #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<serde_json::Value>,
}
impl ErrorDetail {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            value: None,
            resources: Vec::new(),
            additional_properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorInfo {
    #[doc = "A machine readable error code."]
    pub code: String,
    #[doc = "A human readable error message."]
    pub message: String,
    #[doc = "error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Box<Option<ErrorInfo>>,
    #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<serde_json::Value>,
}
impl ErrorInfo {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            details: Vec::new(),
            innererror: Box::new(None),
            additional_properties: None,
        }
    }
}
#[doc = "Contains details when the response code indicates an error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorInfo,
}
impl ErrorResponse {
    pub fn new(error: ErrorInfo) -> Self {
        Self { error }
    }
}
pub type EventId = String;
#[doc = "The type of events to query; either a standard event type (`traces`, `customEvents`, `pageViews`, `requests`, `dependencies`, `exceptions`, `availabilityResults`) or `$all` to query across all event types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EventType")]
pub enum EventType {
    #[serde(rename = "$all")]
    All,
    #[serde(rename = "traces")]
    Traces,
    #[serde(rename = "customEvents")]
    CustomEvents,
    #[serde(rename = "pageViews")]
    PageViews,
    #[serde(rename = "browserTimings")]
    BrowserTimings,
    #[serde(rename = "requests")]
    Requests,
    #[serde(rename = "dependencies")]
    Dependencies,
    #[serde(rename = "exceptions")]
    Exceptions,
    #[serde(rename = "availabilityResults")]
    AvailabilityResults,
    #[serde(rename = "performanceCounters")]
    PerformanceCounters,
    #[serde(rename = "customMetrics")]
    CustomMetrics,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EventType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EventType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EventType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::All => serializer.serialize_unit_variant("EventType", 0u32, "$all"),
            Self::Traces => serializer.serialize_unit_variant("EventType", 1u32, "traces"),
            Self::CustomEvents => serializer.serialize_unit_variant("EventType", 2u32, "customEvents"),
            Self::PageViews => serializer.serialize_unit_variant("EventType", 3u32, "pageViews"),
            Self::BrowserTimings => serializer.serialize_unit_variant("EventType", 4u32, "browserTimings"),
            Self::Requests => serializer.serialize_unit_variant("EventType", 5u32, "requests"),
            Self::Dependencies => serializer.serialize_unit_variant("EventType", 6u32, "dependencies"),
            Self::Exceptions => serializer.serialize_unit_variant("EventType", 7u32, "exceptions"),
            Self::AvailabilityResults => serializer.serialize_unit_variant("EventType", 8u32, "availabilityResults"),
            Self::PerformanceCounters => serializer.serialize_unit_variant("EventType", 9u32, "performanceCounters"),
            Self::CustomMetrics => serializer.serialize_unit_variant("EventType", 10u32, "customMetrics"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "AI related application info for an event result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventsAiInfo {
    #[doc = "iKey of the app"]
    #[serde(rename = "iKey", default, skip_serializing_if = "Option::is_none")]
    pub i_key: Option<String>,
    #[doc = "Name of the application"]
    #[serde(rename = "appName", default, skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[doc = "ID of the application"]
    #[serde(rename = "appId", default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[doc = "SDK version of the application"]
    #[serde(rename = "sdkVersion", default, skip_serializing_if = "Option::is_none")]
    pub sdk_version: Option<String>,
}
impl EventsAiInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Application info for an event result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventsApplicationInfo {
    #[doc = "Version of the application"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl EventsApplicationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type EventsApply = String;
#[doc = "The availability result info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventsAvailabilityResultInfo {
    #[doc = "The name of the availability result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates if the availability result was successful"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
    #[doc = "The duration of the availability result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[doc = "The performance bucket of the availability result"]
    #[serde(rename = "performanceBucket", default, skip_serializing_if = "Option::is_none")]
    pub performance_bucket: Option<String>,
    #[doc = "The message of the availability result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The location of the availability result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The ID of the availability result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The size of the availability result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}
impl EventsAvailabilityResultInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An availability result result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsAvailabilityResultResult {
    #[serde(flatten)]
    pub events_result_data: EventsResultData,
    #[doc = "The availability result info"]
    #[serde(rename = "availabilityResult", default, skip_serializing_if = "Option::is_none")]
    pub availability_result: Option<EventsAvailabilityResultInfo>,
}
impl EventsAvailabilityResultResult {
    pub fn new(events_result_data: EventsResultData) -> Self {
        Self {
            events_result_data,
            availability_result: None,
        }
    }
}
#[doc = "The browser timing information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventsBrowserTimingInfo {
    #[doc = "The path of the URL"]
    #[serde(rename = "urlPath", default, skip_serializing_if = "Option::is_none")]
    pub url_path: Option<String>,
    #[doc = "The host of the URL"]
    #[serde(rename = "urlHost", default, skip_serializing_if = "Option::is_none")]
    pub url_host: Option<String>,
    #[doc = "The name of the page"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The url of the page"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "The total duration of the load"]
    #[serde(rename = "totalDuration", default, skip_serializing_if = "Option::is_none")]
    pub total_duration: Option<i64>,
    #[doc = "The performance bucket of the load"]
    #[serde(rename = "performanceBucket", default, skip_serializing_if = "Option::is_none")]
    pub performance_bucket: Option<String>,
    #[doc = "The network duration of the load"]
    #[serde(rename = "networkDuration", default, skip_serializing_if = "Option::is_none")]
    pub network_duration: Option<i64>,
    #[doc = "The send duration of the load"]
    #[serde(rename = "sendDuration", default, skip_serializing_if = "Option::is_none")]
    pub send_duration: Option<i64>,
    #[doc = "The receive duration of the load"]
    #[serde(rename = "receiveDuration", default, skip_serializing_if = "Option::is_none")]
    pub receive_duration: Option<i64>,
    #[doc = "The processing duration of the load"]
    #[serde(rename = "processingDuration", default, skip_serializing_if = "Option::is_none")]
    pub processing_duration: Option<i64>,
}
impl EventsBrowserTimingInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A browser timing result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsBrowserTimingResult {
    #[serde(flatten)]
    pub events_result_data: EventsResultData,
    #[doc = "The browser timing information"]
    #[serde(rename = "browserTiming", default, skip_serializing_if = "Option::is_none")]
    pub browser_timing: Option<EventsBrowserTimingInfo>,
    #[doc = "Client performance information"]
    #[serde(rename = "clientPerformance", default, skip_serializing_if = "Option::is_none")]
    pub client_performance: Option<EventsClientPerformanceInfo>,
}
impl EventsBrowserTimingResult {
    pub fn new(events_result_data: EventsResultData) -> Self {
        Self {
            events_result_data,
            browser_timing: None,
            client_performance: None,
        }
    }
}
#[doc = "Client info for an event result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventsClientInfo {
    #[doc = "Model of the client"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[doc = "Operating system of the client"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[doc = "Type of the client"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Browser of the client"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,
    #[doc = "IP address of the client"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[doc = "City of the client"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "State or province of the client"]
    #[serde(rename = "stateOrProvince", default, skip_serializing_if = "Option::is_none")]
    pub state_or_province: Option<String>,
    #[doc = "Country or region of the client"]
    #[serde(rename = "countryOrRegion", default, skip_serializing_if = "Option::is_none")]
    pub country_or_region: Option<String>,
}
impl EventsClientInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Client performance information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventsClientPerformanceInfo {
    #[doc = "The name of the client performance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl EventsClientPerformanceInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cloud info for an event result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventsCloudInfo {
    #[doc = "Role name of the cloud"]
    #[serde(rename = "roleName", default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[doc = "Role instance of the cloud"]
    #[serde(rename = "roleInstance", default, skip_serializing_if = "Option::is_none")]
    pub role_instance: Option<String>,
}
impl EventsCloudInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type EventsCount = bool;
#[doc = "The custom event information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventsCustomEventInfo {
    #[doc = "The name of the custom event"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl EventsCustomEventInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A custom event result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsCustomEventResult {
    #[serde(flatten)]
    pub events_result_data: EventsResultData,
    #[doc = "The custom event information"]
    #[serde(rename = "customEvent", default, skip_serializing_if = "Option::is_none")]
    pub custom_event: Option<EventsCustomEventInfo>,
}
impl EventsCustomEventResult {
    pub fn new(events_result_data: EventsResultData) -> Self {
        Self {
            events_result_data,
            custom_event: None,
        }
    }
}
#[doc = "The custom metric info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventsCustomMetricInfo {
    #[doc = "The name of the custom metric"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The value of the custom metric"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[doc = "The sum of the custom metric"]
    #[serde(rename = "valueSum", default, skip_serializing_if = "Option::is_none")]
    pub value_sum: Option<f64>,
    #[doc = "The count of the custom metric"]
    #[serde(rename = "valueCount", default, skip_serializing_if = "Option::is_none")]
    pub value_count: Option<i32>,
    #[doc = "The minimum value of the custom metric"]
    #[serde(rename = "valueMin", default, skip_serializing_if = "Option::is_none")]
    pub value_min: Option<f64>,
    #[doc = "The maximum value of the custom metric"]
    #[serde(rename = "valueMax", default, skip_serializing_if = "Option::is_none")]
    pub value_max: Option<f64>,
    #[doc = "The standard deviation of the custom metric"]
    #[serde(rename = "valueStdDev", default, skip_serializing_if = "Option::is_none")]
    pub value_std_dev: Option<f64>,
}
impl EventsCustomMetricInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A custom metric result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsCustomMetricResult {
    #[serde(flatten)]
    pub events_result_data: EventsResultData,
    #[doc = "The custom metric info"]
    #[serde(rename = "customMetric", default, skip_serializing_if = "Option::is_none")]
    pub custom_metric: Option<EventsCustomMetricInfo>,
}
impl EventsCustomMetricResult {
    pub fn new(events_result_data: EventsResultData) -> Self {
        Self {
            events_result_data,
            custom_metric: None,
        }
    }
}
#[doc = "The dependency info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventsDependencyInfo {
    #[doc = "The target of the dependency"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The data of the dependency"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[doc = "Indicates if the dependency was successful"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
    #[doc = "The duration of the dependency"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[doc = "The performance bucket of the dependency"]
    #[serde(rename = "performanceBucket", default, skip_serializing_if = "Option::is_none")]
    pub performance_bucket: Option<String>,
    #[doc = "The result code of the dependency"]
    #[serde(rename = "resultCode", default, skip_serializing_if = "Option::is_none")]
    pub result_code: Option<String>,
    #[doc = "The type of the dependency"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of the dependency"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The ID of the dependency"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl EventsDependencyInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A dependency result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsDependencyResult {
    #[serde(flatten)]
    pub events_result_data: EventsResultData,
    #[doc = "The dependency info"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependency: Option<EventsDependencyInfo>,
}
impl EventsDependencyResult {
    pub fn new(events_result_data: EventsResultData) -> Self {
        Self {
            events_result_data,
            dependency: None,
        }
    }
}
#[doc = "Exception details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventsExceptionDetail {
    #[doc = "The severity level of the exception detail"]
    #[serde(rename = "severityLevel", default, skip_serializing_if = "Option::is_none")]
    pub severity_level: Option<String>,
    #[doc = "The outer ID of the exception detail"]
    #[serde(rename = "outerId", default, skip_serializing_if = "Option::is_none")]
    pub outer_id: Option<String>,
    #[doc = "The message of the exception detail"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The type of the exception detail"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The ID of the exception detail"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The parsed stack"]
    #[serde(rename = "parsedStack", default, skip_serializing_if = "Vec::is_empty")]
    pub parsed_stack: Vec<EventsExceptionDetailsParsedStack>,
}
impl EventsExceptionDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A parsed stack entry"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventsExceptionDetailsParsedStack {
    #[doc = "The assembly of the stack entry"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assembly: Option<String>,
    #[doc = "The method of the stack entry"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[doc = "The level of the stack entry"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    #[doc = "The line of the stack entry"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
}
impl EventsExceptionDetailsParsedStack {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The exception info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventsExceptionInfo {
    #[doc = "The severity level of the exception"]
    #[serde(rename = "severityLevel", default, skip_serializing_if = "Option::is_none")]
    pub severity_level: Option<i64>,
    #[doc = "The problem ID of the exception"]
    #[serde(rename = "problemId", default, skip_serializing_if = "Option::is_none")]
    pub problem_id: Option<String>,
    #[doc = "Indicates where the exception was handled at"]
    #[serde(rename = "handledAt", default, skip_serializing_if = "Option::is_none")]
    pub handled_at: Option<String>,
    #[doc = "The assembly which threw the exception"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assembly: Option<String>,
    #[doc = "The method that threw the exception"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[doc = "The message of the exception"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The type of the exception"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The outer type of the exception"]
    #[serde(rename = "outerType", default, skip_serializing_if = "Option::is_none")]
    pub outer_type: Option<String>,
    #[doc = "The outer method of the exception"]
    #[serde(rename = "outerMethod", default, skip_serializing_if = "Option::is_none")]
    pub outer_method: Option<String>,
    #[doc = "The outer assembly of the exception"]
    #[serde(rename = "outerAssembly", default, skip_serializing_if = "Option::is_none")]
    pub outer_assembly: Option<String>,
    #[doc = "The outer message of the exception"]
    #[serde(rename = "outerMessage", default, skip_serializing_if = "Option::is_none")]
    pub outer_message: Option<String>,
    #[doc = "The inner most type of the exception"]
    #[serde(rename = "innermostType", default, skip_serializing_if = "Option::is_none")]
    pub innermost_type: Option<String>,
    #[doc = "The inner most message of the exception"]
    #[serde(rename = "innermostMessage", default, skip_serializing_if = "Option::is_none")]
    pub innermost_message: Option<String>,
    #[doc = "The inner most method of the exception"]
    #[serde(rename = "innermostMethod", default, skip_serializing_if = "Option::is_none")]
    pub innermost_method: Option<String>,
    #[doc = "The inner most assembly of the exception"]
    #[serde(rename = "innermostAssembly", default, skip_serializing_if = "Option::is_none")]
    pub innermost_assembly: Option<String>,
    #[doc = "The details of the exception"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<EventsExceptionDetail>,
}
impl EventsExceptionInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An exception result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsExceptionResult {
    #[serde(flatten)]
    pub events_result_data: EventsResultData,
    #[doc = "The exception info"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exception: Option<EventsExceptionInfo>,
}
impl EventsExceptionResult {
    pub fn new(events_result_data: EventsResultData) -> Self {
        Self {
            events_result_data,
            exception: None,
        }
    }
}
pub type EventsFilter = String;
pub type EventsFormat = String;
#[doc = "Operation info for an event result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventsOperationInfo {
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "ID of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Parent ID of the operation"]
    #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[doc = "Synthetic source of the operation"]
    #[serde(rename = "syntheticSource", default, skip_serializing_if = "Option::is_none")]
    pub synthetic_source: Option<String>,
}
impl EventsOperationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type EventsOrderBy = String;
#[doc = "The page view information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventsPageViewInfo {
    #[doc = "The name of the page"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The URL of the page"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "The duration of the page view"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[doc = "The performance bucket of the page view"]
    #[serde(rename = "performanceBucket", default, skip_serializing_if = "Option::is_none")]
    pub performance_bucket: Option<String>,
}
impl EventsPageViewInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A page view result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsPageViewResult {
    #[serde(flatten)]
    pub events_result_data: EventsResultData,
    #[doc = "The page view information"]
    #[serde(rename = "pageView", default, skip_serializing_if = "Option::is_none")]
    pub page_view: Option<EventsPageViewInfo>,
}
impl EventsPageViewResult {
    pub fn new(events_result_data: EventsResultData) -> Self {
        Self {
            events_result_data,
            page_view: None,
        }
    }
}
#[doc = "The performance counter info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventsPerformanceCounterInfo {
    #[doc = "The value of the performance counter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[doc = "The name of the performance counter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The category of the performance counter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "The counter of the performance counter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counter: Option<String>,
    #[doc = "The instance name of the performance counter"]
    #[serde(rename = "instanceName", default, skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[doc = "The instance of the performance counter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
}
impl EventsPerformanceCounterInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A performance counter result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsPerformanceCounterResult {
    #[serde(flatten)]
    pub events_result_data: EventsResultData,
    #[doc = "The performance counter info"]
    #[serde(rename = "performanceCounter", default, skip_serializing_if = "Option::is_none")]
    pub performance_counter: Option<EventsPerformanceCounterInfo>,
}
impl EventsPerformanceCounterResult {
    pub fn new(events_result_data: EventsResultData) -> Self {
        Self {
            events_result_data,
            performance_counter: None,
        }
    }
}
#[doc = "The request info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventsRequestInfo {
    #[doc = "The name of the request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The URL of the request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Indicates if the request was successful"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
    #[doc = "The duration of the request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[doc = "The performance bucket of the request"]
    #[serde(rename = "performanceBucket", default, skip_serializing_if = "Option::is_none")]
    pub performance_bucket: Option<String>,
    #[doc = "The result code of the request"]
    #[serde(rename = "resultCode", default, skip_serializing_if = "Option::is_none")]
    pub result_code: Option<String>,
    #[doc = "The source of the request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "The ID of the request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl EventsRequestInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A request result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsRequestResult {
    #[serde(flatten)]
    pub events_result_data: EventsResultData,
    #[doc = "The request info"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<EventsRequestInfo>,
}
impl EventsRequestResult {
    pub fn new(events_result_data: EventsResultData) -> Self {
        Self {
            events_result_data,
            request: None,
        }
    }
}
#[doc = "An event query result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventsResult {
    #[doc = "OData messages for this response."]
    #[serde(rename = "@ai.messages", default, skip_serializing_if = "Vec::is_empty")]
    pub ai_messages: Vec<ErrorInfo>,
    #[doc = "Events query result data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<EventsResultData>,
}
impl EventsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Events query result data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsResultData {
    #[doc = "The unique ID for this event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of events to query; either a standard event type (`traces`, `customEvents`, `pageViews`, `requests`, `dependencies`, `exceptions`, `availabilityResults`) or `$all` to query across all event types."]
    #[serde(rename = "type")]
    pub type_: EventType,
    #[doc = "Count of the event"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Timestamp of the event"]
    #[serde(default, with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "Custom dimensions of the event"]
    #[serde(rename = "customDimensions", default, skip_serializing_if = "Option::is_none")]
    pub custom_dimensions: Option<events_result_data::CustomDimensions>,
    #[doc = "Custom measurements of the event"]
    #[serde(rename = "customMeasurements", default, skip_serializing_if = "Option::is_none")]
    pub custom_measurements: Option<events_result_data::CustomMeasurements>,
    #[doc = "Operation info for an event result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<EventsOperationInfo>,
    #[doc = "Session info for an event result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<EventsSessionInfo>,
    #[doc = "User info for an event result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<EventsUserInfo>,
    #[doc = "Cloud info for an event result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud: Option<EventsCloudInfo>,
    #[doc = "AI related application info for an event result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ai: Option<EventsAiInfo>,
    #[doc = "Application info for an event result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<EventsApplicationInfo>,
    #[doc = "Client info for an event result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<EventsClientInfo>,
}
impl EventsResultData {
    pub fn new(type_: EventType) -> Self {
        Self {
            id: None,
            type_,
            count: None,
            timestamp: None,
            custom_dimensions: None,
            custom_measurements: None,
            operation: None,
            session: None,
            user: None,
            cloud: None,
            ai: None,
            application: None,
            client: None,
        }
    }
}
pub mod events_result_data {
    use super::*;
    #[doc = "Custom dimensions of the event"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct CustomDimensions {
        #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
        pub additional_properties: Option<serde_json::Value>,
    }
    impl CustomDimensions {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Custom measurements of the event"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct CustomMeasurements {
        #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
        pub additional_properties: Option<serde_json::Value>,
    }
    impl CustomMeasurements {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "An events query result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventsResults {
    #[doc = "OData context metadata endpoint for this response"]
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[doc = "OData messages for this response."]
    #[serde(rename = "@ai.messages", default, skip_serializing_if = "Vec::is_empty")]
    pub ai_messages: Vec<ErrorInfo>,
    #[doc = "Contents of the events query result."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EventsResultData>,
}
impl EventsResults {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type EventsSearch = String;
pub type EventsSelect = String;
#[doc = "Session info for an event result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventsSessionInfo {
    #[doc = "ID of the session"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl EventsSessionInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type EventsSkip = i32;
pub type EventsTimespan = String;
pub type EventsTop = i32;
#[doc = "The trace information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventsTraceInfo {
    #[doc = "The trace message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The trace severity level"]
    #[serde(rename = "severityLevel", default, skip_serializing_if = "Option::is_none")]
    pub severity_level: Option<i64>,
}
impl EventsTraceInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A trace result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventsTraceResult {
    #[serde(flatten)]
    pub events_result_data: EventsResultData,
    #[doc = "The trace information"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace: Option<EventsTraceInfo>,
}
impl EventsTraceResult {
    pub fn new(events_result_data: EventsResultData) -> Self {
        Self {
            events_result_data,
            trace: None,
        }
    }
}
#[doc = "User info for an event result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventsUserInfo {
    #[doc = "ID of the user"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Account ID of the user"]
    #[serde(rename = "accountId", default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[doc = "Authenticated ID of the user"]
    #[serde(rename = "authenticatedId", default, skip_serializing_if = "Option::is_none")]
    pub authenticated_id: Option<String>,
}
impl EventsUserInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Application Insights apps that were part of the metadata request and that the user has access to."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataApplication {
    #[doc = "The ID of the Application Insights app."]
    pub id: String,
    #[doc = "The ARM resource ID of the Application Insights app."]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "The name of the Application Insights app."]
    pub name: String,
    #[doc = "The Azure region of the Application Insights app."]
    pub region: String,
    #[doc = "The list of custom tables for the Application Insights app."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tables: Vec<String>,
    #[doc = "The list of stored functions on the Application Insights app"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub functions: Vec<String>,
    #[doc = "The list of table groups on the Application Insights app"]
    #[serde(rename = "tableGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub table_groups: Vec<String>,
}
impl MetadataApplication {
    pub fn new(id: String, resource_id: String, name: String, region: String) -> Self {
        Self {
            id,
            resource_id,
            name,
            region,
            tables: Vec::new(),
            functions: Vec::new(),
            table_groups: Vec::new(),
        }
    }
}
#[doc = "Functions are stored Kusto queries that can be specified as part of queries by using their name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataFunction {
    #[doc = "The ID of the function."]
    pub id: String,
    #[doc = "The name of the function, to be used in queries."]
    pub name: String,
    #[doc = "The parameters/arguments of the function, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
    #[doc = "The display name of the function."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The description of the function."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The KQL body of the function."]
    pub body: String,
}
impl MetadataFunction {
    pub fn new(id: String, name: String, body: String) -> Self {
        Self {
            id,
            name,
            parameters: None,
            display_name: None,
            description: None,
            body,
        }
    }
}
#[doc = "The metadata result for the app, including available tables, etc."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataResults {
    #[doc = "The list of groups of tables on the app."]
    #[serde(rename = "tableGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub table_groups: Vec<MetadataTableGroup>,
    #[doc = "The list of tables and columns that comprise the schema of the app."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tables: Vec<MetadataTable>,
    #[doc = "The list of functions stored on the app."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub functions: Vec<MetadataFunction>,
    #[doc = "The list of Application Insights apps that were referenced in the metadata request."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applications: Vec<MetadataApplication>,
}
impl MetadataResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tables are part of the app schema, and contain a list of columns and a reference to other relevant metadata items."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataTable {
    #[doc = "The ID of the table"]
    pub id: String,
    #[doc = "The name of the table"]
    pub name: String,
    #[doc = "The description of the table"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The column associated with the timespan query parameter for the table"]
    #[serde(rename = "timespanColumn", default, skip_serializing_if = "Option::is_none")]
    pub timespan_column: Option<String>,
    #[doc = "The list of columns defined on the table"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<serde_json::Value>,
}
impl MetadataTable {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            description: None,
            timespan_column: None,
            columns: Vec::new(),
        }
    }
}
#[doc = "The table grouping can be either an Application Insights schema or a Log Analytics solution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataTableGroup {
    #[doc = "The ID of the table group"]
    pub id: String,
    #[doc = "The name of the table group"]
    pub name: String,
    #[doc = "The source of the table group, can be either AI or OMS for Log Analytics workspaces"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "The display name of the table group"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The description of the table group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The list of tables contained in the table group"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tables: Vec<String>,
}
impl MetadataTableGroup {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            source: None,
            display_name: None,
            description: None,
            tables: Vec::new(),
        }
    }
}
#[doc = "ID of the metric. This is either a standard AI metric, or an application-specific custom metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MetricId")]
pub enum MetricId {
    #[serde(rename = "requests/count")]
    RequestsCount,
    #[serde(rename = "requests/duration")]
    RequestsDuration,
    #[serde(rename = "requests/failed")]
    RequestsFailed,
    #[serde(rename = "users/count")]
    UsersCount,
    #[serde(rename = "users/authenticated")]
    UsersAuthenticated,
    #[serde(rename = "pageViews/count")]
    PageViewsCount,
    #[serde(rename = "pageViews/duration")]
    PageViewsDuration,
    #[serde(rename = "client/processingDuration")]
    ClientProcessingDuration,
    #[serde(rename = "client/receiveDuration")]
    ClientReceiveDuration,
    #[serde(rename = "client/networkDuration")]
    ClientNetworkDuration,
    #[serde(rename = "client/sendDuration")]
    ClientSendDuration,
    #[serde(rename = "client/totalDuration")]
    ClientTotalDuration,
    #[serde(rename = "dependencies/count")]
    DependenciesCount,
    #[serde(rename = "dependencies/failed")]
    DependenciesFailed,
    #[serde(rename = "dependencies/duration")]
    DependenciesDuration,
    #[serde(rename = "exceptions/count")]
    ExceptionsCount,
    #[serde(rename = "exceptions/browser")]
    ExceptionsBrowser,
    #[serde(rename = "exceptions/server")]
    ExceptionsServer,
    #[serde(rename = "sessions/count")]
    SessionsCount,
    #[serde(rename = "performanceCounters/requestExecutionTime")]
    PerformanceCountersRequestExecutionTime,
    #[serde(rename = "performanceCounters/requestsPerSecond")]
    PerformanceCountersRequestsPerSecond,
    #[serde(rename = "performanceCounters/requestsInQueue")]
    PerformanceCountersRequestsInQueue,
    #[serde(rename = "performanceCounters/memoryAvailableBytes")]
    PerformanceCountersMemoryAvailableBytes,
    #[serde(rename = "performanceCounters/exceptionsPerSecond")]
    PerformanceCountersExceptionsPerSecond,
    #[serde(rename = "performanceCounters/processCpuPercentage")]
    PerformanceCountersProcessCpuPercentage,
    #[serde(rename = "performanceCounters/processIOBytesPerSecond")]
    PerformanceCountersProcessIoBytesPerSecond,
    #[serde(rename = "performanceCounters/processPrivateBytes")]
    PerformanceCountersProcessPrivateBytes,
    #[serde(rename = "performanceCounters/processorCpuPercentage")]
    PerformanceCountersProcessorCpuPercentage,
    #[serde(rename = "availabilityResults/availabilityPercentage")]
    AvailabilityResultsAvailabilityPercentage,
    #[serde(rename = "availabilityResults/duration")]
    AvailabilityResultsDuration,
    #[serde(rename = "billing/telemetryCount")]
    BillingTelemetryCount,
    #[serde(rename = "customEvents/count")]
    CustomEventsCount,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MetricId {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MetricId {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MetricId {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::RequestsCount => serializer.serialize_unit_variant("MetricId", 0u32, "requests/count"),
            Self::RequestsDuration => serializer.serialize_unit_variant("MetricId", 1u32, "requests/duration"),
            Self::RequestsFailed => serializer.serialize_unit_variant("MetricId", 2u32, "requests/failed"),
            Self::UsersCount => serializer.serialize_unit_variant("MetricId", 3u32, "users/count"),
            Self::UsersAuthenticated => serializer.serialize_unit_variant("MetricId", 4u32, "users/authenticated"),
            Self::PageViewsCount => serializer.serialize_unit_variant("MetricId", 5u32, "pageViews/count"),
            Self::PageViewsDuration => serializer.serialize_unit_variant("MetricId", 6u32, "pageViews/duration"),
            Self::ClientProcessingDuration => serializer.serialize_unit_variant("MetricId", 7u32, "client/processingDuration"),
            Self::ClientReceiveDuration => serializer.serialize_unit_variant("MetricId", 8u32, "client/receiveDuration"),
            Self::ClientNetworkDuration => serializer.serialize_unit_variant("MetricId", 9u32, "client/networkDuration"),
            Self::ClientSendDuration => serializer.serialize_unit_variant("MetricId", 10u32, "client/sendDuration"),
            Self::ClientTotalDuration => serializer.serialize_unit_variant("MetricId", 11u32, "client/totalDuration"),
            Self::DependenciesCount => serializer.serialize_unit_variant("MetricId", 12u32, "dependencies/count"),
            Self::DependenciesFailed => serializer.serialize_unit_variant("MetricId", 13u32, "dependencies/failed"),
            Self::DependenciesDuration => serializer.serialize_unit_variant("MetricId", 14u32, "dependencies/duration"),
            Self::ExceptionsCount => serializer.serialize_unit_variant("MetricId", 15u32, "exceptions/count"),
            Self::ExceptionsBrowser => serializer.serialize_unit_variant("MetricId", 16u32, "exceptions/browser"),
            Self::ExceptionsServer => serializer.serialize_unit_variant("MetricId", 17u32, "exceptions/server"),
            Self::SessionsCount => serializer.serialize_unit_variant("MetricId", 18u32, "sessions/count"),
            Self::PerformanceCountersRequestExecutionTime => {
                serializer.serialize_unit_variant("MetricId", 19u32, "performanceCounters/requestExecutionTime")
            }
            Self::PerformanceCountersRequestsPerSecond => {
                serializer.serialize_unit_variant("MetricId", 20u32, "performanceCounters/requestsPerSecond")
            }
            Self::PerformanceCountersRequestsInQueue => {
                serializer.serialize_unit_variant("MetricId", 21u32, "performanceCounters/requestsInQueue")
            }
            Self::PerformanceCountersMemoryAvailableBytes => {
                serializer.serialize_unit_variant("MetricId", 22u32, "performanceCounters/memoryAvailableBytes")
            }
            Self::PerformanceCountersExceptionsPerSecond => {
                serializer.serialize_unit_variant("MetricId", 23u32, "performanceCounters/exceptionsPerSecond")
            }
            Self::PerformanceCountersProcessCpuPercentage => {
                serializer.serialize_unit_variant("MetricId", 24u32, "performanceCounters/processCpuPercentage")
            }
            Self::PerformanceCountersProcessIoBytesPerSecond => {
                serializer.serialize_unit_variant("MetricId", 25u32, "performanceCounters/processIOBytesPerSecond")
            }
            Self::PerformanceCountersProcessPrivateBytes => {
                serializer.serialize_unit_variant("MetricId", 26u32, "performanceCounters/processPrivateBytes")
            }
            Self::PerformanceCountersProcessorCpuPercentage => {
                serializer.serialize_unit_variant("MetricId", 27u32, "performanceCounters/processorCpuPercentage")
            }
            Self::AvailabilityResultsAvailabilityPercentage => {
                serializer.serialize_unit_variant("MetricId", 28u32, "availabilityResults/availabilityPercentage")
            }
            Self::AvailabilityResultsDuration => serializer.serialize_unit_variant("MetricId", 29u32, "availabilityResults/duration"),
            Self::BillingTelemetryCount => serializer.serialize_unit_variant("MetricId", 30u32, "billing/telemetryCount"),
            Self::CustomEventsCount => serializer.serialize_unit_variant("MetricId", 31u32, "customEvents/count"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type MetricsAggregation = Vec<String>;
pub type MetricsFilter = String;
pub type MetricsInterval = String;
pub type MetricsOrderBy = String;
pub type MetricsPostBody = Vec<MetricsPostBodySchema>;
#[doc = "A metric request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetricsPostBodySchema {
    #[doc = "An identifier for this query.  Must be unique within the post body of the request.  This identifier will be the 'id' property of the response object representing this query."]
    pub id: String,
    #[doc = "The parameters for a single metrics query"]
    pub parameters: metrics_post_body_schema::Parameters,
}
impl MetricsPostBodySchema {
    pub fn new(id: String, parameters: metrics_post_body_schema::Parameters) -> Self {
        Self { id, parameters }
    }
}
pub mod metrics_post_body_schema {
    use super::*;
    #[doc = "The parameters for a single metrics query"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Parameters {
        #[doc = "ID of the metric. This is either a standard AI metric, or an application-specific custom metric."]
        #[serde(rename = "metricId")]
        pub metric_id: MetricId,
        #[doc = "The timespan over which to retrieve metric values. This is an ISO8601 time period value. If timespan is omitted, a default time range of `PT12H` (\"last 12 hours\") is used. The actual timespan that is queried may be adjusted by the server based. In all cases, the actual time span used for the query is included in the response."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub timespan: Option<MetricsTimespan>,
        #[doc = "The aggregation to use when computing the metric values. To retrieve more than one aggregation at a time, separate them with a comma. If no aggregation is specified, then the default aggregation for the metric is used."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub aggregation: Option<MetricsAggregation>,
        #[doc = "The time interval to use when retrieving metric values. This is an ISO8601 duration. If interval is omitted, the metric value is aggregated across the entire timespan. If interval is supplied, the server may adjust the interval to a more appropriate size based on the timespan used for the query. In all cases, the actual interval used for the query is included in the response."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub interval: Option<MetricsInterval>,
        #[doc = "The name of the dimension to segment the metric values by. This dimension must be applicable to the metric you are retrieving. To segment by more than one dimension at a time, separate them with a comma (,). In this case, the metric data will be segmented in the order the dimensions are listed in the parameter."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub segment: Option<MetricsSegment>,
        #[doc = "The number of segments to return.  This value is only valid when segment is specified."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub top: Option<MetricsTop>,
        #[doc = "The aggregation function and direction to sort the segments by.  This value is only valid when segment is specified."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub orderby: Option<MetricsOrderBy>,
        #[doc = "An expression used to filter the results.  This value should be a valid OData filter expression where the keys of each clause should be applicable dimensions for the metric you are retrieving."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub filter: Option<MetricsFilter>,
    }
    impl Parameters {
        pub fn new(metric_id: MetricId) -> Self {
            Self {
                metric_id,
                timespan: None,
                aggregation: None,
                interval: None,
                segment: None,
                top: None,
                orderby: None,
                filter: None,
            }
        }
    }
}
#[doc = "A metric result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricsResult {
    #[doc = "A metric result data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<MetricsResultInfo>,
}
impl MetricsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A metric result data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricsResultInfo {
    #[doc = "Start time of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[doc = "Start time of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[doc = "The interval used to segment the metric data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[doc = "Segmented metric data (if segmented)."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub segments: Vec<MetricsSegmentInfo>,
}
impl MetricsResultInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type MetricsResults = Vec<serde_json::Value>;
pub type MetricsSegment = Vec<String>;
#[doc = "A metric segment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricsSegmentInfo {
    #[doc = "Start time of the metric segment (only when an interval was specified)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[doc = "Start time of the metric segment (only when an interval was specified)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[doc = "Segmented metric data (if further segmented)."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub segments: Vec<MetricsSegmentInfo>,
}
impl MetricsSegmentInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type MetricsTimespan = String;
pub type MetricsTop = i32;
#[doc = "The Analytics query. Learn more about the [Analytics query syntax](https://azure.microsoft.com/documentation/articles/app-insights-analytics-reference/)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryBody {
    #[doc = "The Analytics query. Learn more about the [Analytics query syntax](https://azure.microsoft.com/documentation/articles/app-insights-analytics-reference/)"]
    pub query: QueryParam,
    #[doc = "Optional. The timespan over which to query data. This is an ISO8601 time period value.  This timespan is applied in addition to any that are specified in the query expression."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timespan: Option<QueryTimespan>,
    #[doc = "Application IDs to include in cross-application queries."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applications: Option<ApplicationsParam>,
}
impl QueryBody {
    pub fn new(query: QueryParam) -> Self {
        Self {
            query,
            timespan: None,
            applications: None,
        }
    }
}
pub type QueryParam = String;
#[doc = "Contains the tables, columns & rows resulting from a query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryResults {
    #[doc = "The list of tables, columns and rows."]
    pub tables: Vec<Table>,
}
impl QueryResults {
    pub fn new(tables: Vec<Table>) -> Self {
        Self { tables }
    }
}
pub type QueryTimespan = String;
#[doc = "Contains the columns and rows for one table in a query response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Table {
    #[doc = "The name of the table."]
    pub name: String,
    #[doc = "The list of columns in this table."]
    pub columns: Vec<Column>,
    #[doc = "The resulting rows from this query."]
    pub rows: serde_json::Value,
}
impl Table {
    pub fn new(name: String, columns: Vec<Column>, rows: serde_json::Value) -> Self {
        Self { name, columns, rows }
    }
}
