#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "An error response from the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response from the Batch service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the Batch service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "An identifier for the error. Codes are invariant and are intended to be consumed programmatically."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error, intended to be suitable for display in a user interface."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error. For example, the name of the property in error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "A list of additional details about the error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Returns whether or not Serial Console is disabled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DisableSerialConsoleResult {
    #[doc = "Whether or not Serial Console is disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}
impl DisableSerialConsoleResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Returns whether or not Serial Console is disabled (enabled)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnableSerialConsoleResult {
    #[doc = "Whether or not Serial Console is disabled (enabled)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}
impl EnableSerialConsoleResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error saying that the provided subscription could not be found"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GetSerialConsoleSubscriptionNotFound {
    #[doc = "Error code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Subscription not found message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl GetSerialConsoleSubscriptionNotFound {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for a ARM proxy resource. It will have everything other than required location and tags"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Serial Console operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SerialConsoleOperations {
    #[doc = "A list of Serial Console operations"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<serde_json::Value>,
}
impl SerialConsoleOperations {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Returns whether or not Serial Console is disabled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SerialConsoleStatus {
    #[doc = "Whether or not Serial Console is disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}
impl SerialConsoleStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents the serial port of the parent resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SerialPort {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of the serial port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SerialPortProperties>,
}
impl SerialPort {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Returns a connection string to the serial port of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SerialPortConnectResult {
    #[doc = "Connection string to the serial port of the resource."]
    #[serde(rename = "connectionString", default, skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<String>,
}
impl SerialPortConnectResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list serial ports operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SerialPortListResult {
    #[doc = "The list of serial ports."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SerialPort>,
}
impl SerialPortListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the serial port."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SerialPortProperties {
    #[doc = "Specifies whether the port is enabled for a serial console connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<serial_port_properties::State>,
}
impl SerialPortProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod serial_port_properties {
    use super::*;
    #[doc = "Specifies whether the port is enabled for a serial console connection."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        #[serde(rename = "enabled")]
        Enabled,
        #[serde(rename = "disabled")]
        Disabled,
    }
}
