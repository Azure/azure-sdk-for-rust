#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The result returned from a database check name availability request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameRequest {
    #[doc = "Resource name."]
    pub name: String,
    #[doc = "The type of resource, for instance Microsoft.DigitalTwins/digitalTwinsInstances."]
    #[serde(rename = "type")]
    pub type_: check_name_request::Type,
}
impl CheckNameRequest {
    pub fn new(name: String, type_: check_name_request::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod check_name_request {
    use super::*;
    #[doc = "The type of resource, for instance Microsoft.DigitalTwins/digitalTwinsInstances."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.DigitalTwins/digitalTwinsInstances")]
        MicrosoftDigitalTwinsDigitalTwinsInstances,
    }
}
#[doc = "The result returned from a check name availability request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameResult {
    #[doc = "Specifies a Boolean value that indicates if the name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The name that was checked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Message indicating an unavailable name due to a conflict, or a description of the naming rules that are violated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Message providing the reason why the given name is invalid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_result::Reason>,
}
impl CheckNameResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_result {
    use super::*;
    #[doc = "Message providing the reason why the given name is invalid."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Reason")]
    pub enum Reason {
        Invalid,
        AlreadyExists,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Reason {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Reason {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Reason {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("Reason", 0u32, "Invalid"),
                Self::AlreadyExists => serializer.serialize_unit_variant("Reason", 1u32, "AlreadyExists"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The description of the DigitalTwins service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DigitalTwinsDescription {
    #[serde(flatten)]
    pub digital_twins_resource: DigitalTwinsResource,
    #[doc = "The properties of a DigitalTwinsInstance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DigitalTwinsProperties>,
}
impl DigitalTwinsDescription {
    pub fn new(digital_twins_resource: DigitalTwinsResource) -> Self {
        Self {
            digital_twins_resource,
            properties: None,
        }
    }
}
#[doc = "A list of DigitalTwins description objects with a next link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DigitalTwinsDescriptionListResult {
    #[doc = "The link used to get the next page of DigitalTwins description objects."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "A list of DigitalTwins description objects."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DigitalTwinsDescription>,
}
impl azure_core::Continuable for DigitalTwinsDescriptionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DigitalTwinsDescriptionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DigitalTwinsInstance endpoint resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DigitalTwinsEndpointResource {
    #[serde(flatten)]
    pub external_resource: ExternalResource,
    #[doc = "Properties related to Digital Twins Endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DigitalTwinsEndpointResourceProperties>,
}
impl DigitalTwinsEndpointResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of DigitalTwinsInstance Endpoints with a next link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DigitalTwinsEndpointResourceListResult {
    #[doc = "The link used to get the next page of DigitalTwinsInstance Endpoints."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "A list of DigitalTwinsInstance Endpoints."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DigitalTwinsEndpointResource>,
}
impl azure_core::Continuable for DigitalTwinsEndpointResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DigitalTwinsEndpointResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties related to Digital Twins Endpoint"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DigitalTwinsEndpointResourceProperties {
    #[doc = "The type of Digital Twins endpoint"]
    #[serde(rename = "endpointType")]
    pub endpoint_type: digital_twins_endpoint_resource_properties::EndpointType,
    #[doc = "The provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<digital_twins_endpoint_resource_properties::ProvisioningState>,
    #[doc = "Time when the Endpoint was added to DigitalTwinsInstance."]
    #[serde(rename = "createdTime", with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DigitalTwinsEndpointResourceProperties {
    pub fn new(endpoint_type: digital_twins_endpoint_resource_properties::EndpointType) -> Self {
        Self {
            endpoint_type,
            provisioning_state: None,
            created_time: None,
            tags: None,
        }
    }
}
pub mod digital_twins_endpoint_resource_properties {
    use super::*;
    #[doc = "The type of Digital Twins endpoint"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EndpointType")]
    pub enum EndpointType {
        EventHub,
        EventGrid,
        ServiceBus,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EndpointType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EndpointType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EndpointType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::EventHub => serializer.serialize_unit_variant("EndpointType", 0u32, "EventHub"),
                Self::EventGrid => serializer.serialize_unit_variant("EndpointType", 1u32, "EventGrid"),
                Self::ServiceBus => serializer.serialize_unit_variant("EndpointType", 2u32, "ServiceBus"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The provisioning state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Provisioning,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Provisioning"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The description of the DigitalTwins service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DigitalTwinsPatchDescription {
    #[doc = "Instance tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DigitalTwinsPatchDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a DigitalTwinsInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DigitalTwinsProperties {
    #[doc = "Time when DigitalTwinsInstance was created."]
    #[serde(rename = "createdTime", with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<time::OffsetDateTime>,
    #[doc = "Time when DigitalTwinsInstance was created."]
    #[serde(rename = "lastUpdatedTime", with = "azure_core::date::rfc3339::option")]
    pub last_updated_time: Option<time::OffsetDateTime>,
    #[doc = "The provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<digital_twins_properties::ProvisioningState>,
    #[doc = "Api endpoint to work with DigitalTwinsInstance."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
}
impl DigitalTwinsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod digital_twins_properties {
    use super::*;
    #[doc = "The provisioning state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Provisioning,
        Deleting,
        Succeeded,
        Failed,
        Canceled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ProvisioningState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ProvisioningState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ProvisioningState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Provisioning"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The common properties of a DigitalTwinsInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DigitalTwinsResource {
    #[doc = "The resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The resource location."]
    pub location: String,
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Information about the SKU of the DigitalTwinsInstance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<DigitalTwinsSkuInfo>,
}
impl DigitalTwinsResource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
            sku: None,
        }
    }
}
#[doc = "Information about the SKU of the DigitalTwinsInstance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DigitalTwinsSkuInfo {
    #[doc = "The name of the SKU."]
    pub name: digital_twins_sku_info::Name,
}
impl DigitalTwinsSkuInfo {
    pub fn new(name: digital_twins_sku_info::Name) -> Self {
        Self { name }
    }
}
pub mod digital_twins_sku_info {
    use super::*;
    #[doc = "The name of the SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        F1,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Name {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Name {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Name {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::F1 => serializer.serialize_unit_variant("Name", 0u32, "F1"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Error definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDefinition {
    #[doc = "Service specific error code which serves as the substatus for the HTTP error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Description of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Internal error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDefinition>,
}
impl ErrorDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
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
#[doc = "properties related to eventgrid."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventGrid {
    #[serde(flatten)]
    pub digital_twins_endpoint_resource_properties: DigitalTwinsEndpointResourceProperties,
    #[doc = "EventGrid Topic Endpoint"]
    #[serde(rename = "TopicEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub topic_endpoint: Option<String>,
    #[doc = "EventGrid secondary accesskey. Will be obfuscated during read"]
    #[serde(rename = "accessKey1")]
    pub access_key1: String,
    #[doc = "EventGrid secondary accesskey. Will be obfuscated during read"]
    #[serde(rename = "accessKey2")]
    pub access_key2: String,
}
impl EventGrid {
    pub fn new(
        digital_twins_endpoint_resource_properties: DigitalTwinsEndpointResourceProperties,
        access_key1: String,
        access_key2: String,
    ) -> Self {
        Self {
            digital_twins_endpoint_resource_properties,
            topic_endpoint: None,
            access_key1,
            access_key2,
        }
    }
}
#[doc = "properties related to eventhub."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHub {
    #[serde(flatten)]
    pub digital_twins_endpoint_resource_properties: DigitalTwinsEndpointResourceProperties,
    #[doc = "PrimaryConnectionString of the endpoint. Will be obfuscated during read"]
    #[serde(rename = "connectionString-PrimaryKey")]
    pub connection_string_primary_key: String,
    #[doc = "SecondaryConnectionString of the endpoint. Will be obfuscated during read"]
    #[serde(rename = "connectionString-SecondaryKey")]
    pub connection_string_secondary_key: String,
}
impl EventHub {
    pub fn new(
        digital_twins_endpoint_resource_properties: DigitalTwinsEndpointResourceProperties,
        connection_string_primary_key: String,
        connection_string_secondary_key: String,
    ) -> Self {
        Self {
            digital_twins_endpoint_resource_properties,
            connection_string_primary_key,
            connection_string_secondary_key,
        }
    }
}
#[doc = "Definition of a Resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExternalResource {
    #[doc = "The resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Extension resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ExternalResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "DigitalTwins service REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{read | write | action | delete}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The object that represents the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Service provider: Microsoft DigitalTwins"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource Type: DigitalTwinsInstances"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Friendly description for the operation,"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of DigitalTwins service operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "The link used to get the next page of DigitalTwins description objects."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "A list of DigitalTwins operations supported by the Microsoft.DigitalTwins resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "properties related to servicebus."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceBus {
    #[serde(flatten)]
    pub digital_twins_endpoint_resource_properties: DigitalTwinsEndpointResourceProperties,
    #[doc = "PrimaryConnectionString of the endpoint. Will be obfuscated during read"]
    #[serde(rename = "primaryConnectionString")]
    pub primary_connection_string: String,
    #[doc = "SecondaryConnectionString of the endpoint. Will be obfuscated during read"]
    #[serde(rename = "secondaryConnectionString")]
    pub secondary_connection_string: String,
}
impl ServiceBus {
    pub fn new(
        digital_twins_endpoint_resource_properties: DigitalTwinsEndpointResourceProperties,
        primary_connection_string: String,
        secondary_connection_string: String,
    ) -> Self {
        Self {
            digital_twins_endpoint_resource_properties,
            primary_connection_string,
            secondary_connection_string,
        }
    }
}
