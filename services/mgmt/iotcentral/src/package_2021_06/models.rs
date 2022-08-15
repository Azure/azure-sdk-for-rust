#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The IoT Central application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct App {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The properties of an IoT Central application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AppProperties>,
    #[doc = "Information about the SKU of the IoT Central application."]
    pub sku: AppSkuInfo,
    #[doc = "Managed service identity (either system assigned, or none)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<SystemAssignedServiceIdentity>,
}
impl App {
    pub fn new(resource: Resource, sku: AppSkuInfo) -> Self {
        Self {
            resource,
            properties: None,
            sku,
            identity: None,
        }
    }
}
#[doc = "The properties indicating whether a given IoT Central application name or subdomain is available."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppAvailabilityInfo {
    #[doc = "The value which indicates whether the provided name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason for unavailability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The detailed reason message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl AppAvailabilityInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of IoT Central Applications with a next link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppListResult {
    #[doc = "The link used to get the next page of IoT Central Applications."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "A list of IoT Central Applications."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<App>,
}
impl azure_core::Continuable for AppListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AppListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The description of the IoT Central application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppPatch {
    #[doc = "Instance tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Information about the SKU of the IoT Central application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<AppSkuInfo>,
    #[doc = "The properties of an IoT Central application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AppProperties>,
    #[doc = "Managed service identity (either system assigned, or none)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<SystemAssignedServiceIdentity>,
}
impl AppPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an IoT Central application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppProperties {
    #[doc = "The ID of the application."]
    #[serde(rename = "applicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[doc = "The display name of the application."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The subdomain of the application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<String>,
    #[doc = "The ID of the application template, which is a blueprint that defines the characteristics and behaviors of an application. Optional; if not specified, defaults to a blank blueprint and allows the application to be defined from scratch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[doc = "The current state of the application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<AppState>,
}
impl AppProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the SKU of the IoT Central application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppSkuInfo {
    #[doc = "The name of the SKU."]
    pub name: app_sku_info::Name,
}
impl AppSkuInfo {
    pub fn new(name: app_sku_info::Name) -> Self {
        Self { name }
    }
}
pub mod app_sku_info {
    use super::*;
    #[doc = "The name of the SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        #[serde(rename = "ST0")]
        St0,
        #[serde(rename = "ST1")]
        St1,
        #[serde(rename = "ST2")]
        St2,
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
                Self::St0 => serializer.serialize_unit_variant("Name", 0u32, "ST0"),
                Self::St1 => serializer.serialize_unit_variant("Name", 1u32, "ST1"),
                Self::St2 => serializer.serialize_unit_variant("Name", 2u32, "ST2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The current state of the application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AppState")]
pub enum AppState {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AppState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AppState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AppState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Created => serializer.serialize_unit_variant("AppState", 0u32, "created"),
            Self::Suspended => serializer.serialize_unit_variant("AppState", 1u32, "suspended"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "IoT Central Application Template."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppTemplate {
    #[doc = "The ID of the template."]
    #[serde(rename = "manifestId", default, skip_serializing_if = "Option::is_none")]
    pub manifest_id: Option<String>,
    #[doc = "The version of the template."]
    #[serde(rename = "manifestVersion", default, skip_serializing_if = "Option::is_none")]
    pub manifest_version: Option<String>,
    #[doc = "The name of the template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The title of the template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The order of the template in the templates list."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<f64>,
    #[doc = "The description of the template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The industry of the template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,
    #[doc = "A list of locations that support the template."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<AppTemplateLocations>,
}
impl AppTemplate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "IoT Central Application Template Locations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppTemplateLocations {
    #[doc = "The ID of the location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The display name of the location."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl AppTemplateLocations {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of IoT Central Application Templates with a next link."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppTemplatesResult {
    #[doc = "The link used to get the next page of IoT Central application templates."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "A list of IoT Central Application Templates."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AppTemplate>,
}
impl azure_core::Continuable for AppTemplatesResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AppTemplatesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "Details of error response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl azure_core::Continuable for CloudError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The target of the particular error."]
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
#[doc = "IoT Central REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{read | write | action | delete}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "The intended executor of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Additional descriptions for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The object that represents the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Service provider: Microsoft IoT Central"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource Type: IoT Central"]
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
#[doc = "Input values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationInputs {
    #[doc = "The name of the IoT Central application instance to check."]
    pub name: String,
    #[doc = "The type of the IoT Central resource to query."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl OperationInputs {
    pub fn new(name: String) -> Self {
        Self { name, type_: None }
    }
}
#[doc = "A list of IoT Central operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "The link used to get the next page of IoT Central description objects."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "A list of operations supported by the Microsoft.IoTCentral resource provider."]
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
#[doc = "The common properties of an ARM resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[doc = "The ARM resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The ARM resource name."]
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
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
        }
    }
}
#[doc = "Managed service identity (either system assigned, or none)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemAssignedServiceIdentity {
    #[doc = "The service principal ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Type of managed service identity (either system assigned, or none)."]
    #[serde(rename = "type")]
    pub type_: SystemAssignedServiceIdentityType,
}
impl SystemAssignedServiceIdentity {
    pub fn new(type_: SystemAssignedServiceIdentityType) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
        }
    }
}
#[doc = "Type of managed service identity (either system assigned, or none)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SystemAssignedServiceIdentityType")]
pub enum SystemAssignedServiceIdentityType {
    None,
    SystemAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SystemAssignedServiceIdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SystemAssignedServiceIdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SystemAssignedServiceIdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("SystemAssignedServiceIdentityType", 0u32, "None"),
            Self::SystemAssigned => serializer.serialize_unit_variant("SystemAssignedServiceIdentityType", 1u32, "SystemAssigned"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
