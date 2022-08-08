#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[doc = "Error code identifying the specific error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A human-readable error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<error_response::Error>,
}
impl azure_core::Continuable for ErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod error_response {
    use super::*;
    #[doc = "Error info."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[doc = "Service-defined error code. This code serves as a sub-status for the HTTP error code specified in the response."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "Human-readable representation of the error."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[doc = "Error details."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub details: Vec<ErrorDetails>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthMonitor {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the monitor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HealthMonitorProperties>,
}
impl HealthMonitor {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthMonitorList {
    #[doc = "Array of health monitors of the virtual machine."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HealthMonitor>,
    #[doc = "Link to next page if the list is too long."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HealthMonitorList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl HealthMonitorList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the monitor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthMonitorProperties {
    #[doc = "Human-readable name of the monitor."]
    #[serde(rename = "monitorName", default, skip_serializing_if = "Option::is_none")]
    pub monitor_name: Option<String>,
    #[doc = "Type of the monitor."]
    #[serde(rename = "monitorType", default, skip_serializing_if = "Option::is_none")]
    pub monitor_type: Option<String>,
    #[doc = "Dynamic monitored object of the monitor."]
    #[serde(rename = "monitoredObject", default, skip_serializing_if = "Option::is_none")]
    pub monitored_object: Option<String>,
    #[doc = "Name of the parent monitor."]
    #[serde(rename = "parentMonitorName", default, skip_serializing_if = "Option::is_none")]
    pub parent_monitor_name: Option<String>,
    #[serde(rename = "previousMonitorState", default, skip_serializing_if = "Option::is_none")]
    pub previous_monitor_state: Option<HealthState>,
    #[serde(rename = "currentMonitorState", default, skip_serializing_if = "Option::is_none")]
    pub current_monitor_state: Option<HealthState>,
    #[doc = "Timestamp of the monitor's last health evaluation."]
    #[serde(rename = "evaluationTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub evaluation_timestamp: Option<String>,
    #[doc = "Timestamp of the monitor's last health state change."]
    #[serde(rename = "currentStateFirstObservedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub current_state_first_observed_timestamp: Option<String>,
    #[doc = "Timestamp of the monitor's last reported health state."]
    #[serde(rename = "lastReportedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub last_reported_timestamp: Option<String>,
    #[doc = "Evidence validating the monitor's current health state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence: Option<serde_json::Value>,
    #[doc = "The configuration settings at the time of the monitor's health evaluation."]
    #[serde(rename = "monitorConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub monitor_configuration: Option<serde_json::Value>,
}
impl HealthMonitorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthMonitorStateChange {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the monitor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HealthMonitorStateChangeProperties>,
}
impl HealthMonitorStateChange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthMonitorStateChangeList {
    #[doc = "Array of health state changes within the specified time window."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HealthMonitorStateChange>,
    #[doc = "Link to next page if the list is too long."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HealthMonitorStateChangeList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl HealthMonitorStateChangeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the monitor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthMonitorStateChangeProperties {
    #[doc = "Human-readable name of the monitor."]
    #[serde(rename = "monitorName", default, skip_serializing_if = "Option::is_none")]
    pub monitor_name: Option<String>,
    #[doc = "Type of the monitor."]
    #[serde(rename = "monitorType", default, skip_serializing_if = "Option::is_none")]
    pub monitor_type: Option<String>,
    #[doc = "Dynamic monitored object of the monitor."]
    #[serde(rename = "monitoredObject", default, skip_serializing_if = "Option::is_none")]
    pub monitored_object: Option<String>,
    #[doc = "Timestamp of the monitor's last health evaluation."]
    #[serde(rename = "evaluationTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub evaluation_timestamp: Option<String>,
    #[doc = "Timestamp of the monitor's last health state change."]
    #[serde(rename = "currentStateFirstObservedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub current_state_first_observed_timestamp: Option<String>,
    #[serde(rename = "previousMonitorState", default, skip_serializing_if = "Option::is_none")]
    pub previous_monitor_state: Option<HealthState>,
    #[serde(rename = "currentMonitorState", default, skip_serializing_if = "Option::is_none")]
    pub current_monitor_state: Option<HealthState>,
    #[doc = "Evidence validating the monitor's current health state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence: Option<serde_json::Value>,
    #[doc = "The configuration settings at the time of the monitor's health evaluation."]
    #[serde(rename = "monitorConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub monitor_configuration: Option<serde_json::Value>,
}
impl HealthMonitorStateChangeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HealthState")]
pub enum HealthState {
    Healthy,
    Critical,
    Warning,
    Unknown,
    Disabled,
    None,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HealthState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HealthState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HealthState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Healthy => serializer.serialize_unit_variant("HealthState", 0u32, "Healthy"),
            Self::Critical => serializer.serialize_unit_variant("HealthState", 1u32, "Critical"),
            Self::Warning => serializer.serialize_unit_variant("HealthState", 2u32, "Warning"),
            Self::Unknown => serializer.serialize_unit_variant("HealthState", 3u32, "Unknown"),
            Self::Disabled => serializer.serialize_unit_variant("HealthState", 4u32, "Disabled"),
            Self::None => serializer.serialize_unit_variant("HealthState", 5u32, "None"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[doc = "The name of the operation being performed on this particular object."]
    pub name: String,
    #[doc = "The localized display information for this particular operation or action."]
    pub display: operation::Display,
    #[doc = "The intended executor of the operation."]
    pub origin: String,
}
impl Operation {
    pub fn new(name: String, display: operation::Display, origin: String) -> Self {
        Self { name, display, origin }
    }
}
pub mod operation {
    use super::*;
    #[doc = "The localized display information for this particular operation or action."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[doc = "Operation resource provider name."]
        pub provider: String,
        #[doc = "Resource on which the operation is performed."]
        pub resource: String,
        #[doc = "Human-readable, friendly name for the operation."]
        pub operation: String,
        #[doc = "Operation description."]
        pub description: String,
    }
    impl Display {
        pub fn new(provider: String, resource: String, operation: String, description: String) -> Self {
            Self {
                provider,
                resource,
                operation,
                description,
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[doc = "Array of available REST API operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "Link to next page if the list is too long."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for the ARM proxy resource, 'microsoft.workloadmonitor/monitors'."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "The resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
