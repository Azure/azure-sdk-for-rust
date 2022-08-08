#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Available operations of the service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableOperations {
    #[doc = "Collection of available operation details"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationDetail>,
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AvailableOperations {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AvailableOperations {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of Healthbot operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BotResponseList {
    #[doc = "The link used to get the next page of bot service resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Gets the list of Healthbot results and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<HealthBot>,
}
impl azure_core::Continuable for BotResponseList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BotResponseList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource management error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "The error object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<error::Error>,
}
impl azure_core::Continuable for Error {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod error {
    use super::*;
    #[doc = "The error object."]
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
        #[doc = "The error details."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub details: Vec<Error>,
        #[doc = "The error additional info."]
        #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
        pub additional_info: Vec<ErrorAdditionalInfo>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The resource management error additional info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[doc = "The additional info type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The additional info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "HealthBot resource definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthBot {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The resource model definition representing SKU"]
    pub sku: Sku,
    #[doc = "The properties of a HealthBot. The Health Bot Service is a cloud platform that empowers developers in Healthcare organizations to build and deploy their compliant, AI-powered virtual health assistants and health bots, that help them improve processes and reduce costs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HealthBotProperties>,
}
impl HealthBot {
    pub fn new(tracked_resource: TrackedResource, sku: Sku) -> Self {
        Self {
            tracked_resource,
            sku,
            properties: None,
        }
    }
}
#[doc = "The properties of a HealthBot. The Health Bot Service is a cloud platform that empowers developers in Healthcare organizations to build and deploy their compliant, AI-powered virtual health assistants and health bots, that help them improve processes and reduce costs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthBotProperties {
    #[doc = "The provisioning state of the Healthbot resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The link."]
    #[serde(rename = "botManagementPortalLink", default, skip_serializing_if = "Option::is_none")]
    pub bot_management_portal_link: Option<String>,
}
impl HealthBotProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for updating a HealthBot."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthBotUpdateParameters {
    #[doc = "Tags for a HealthBot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The resource model definition representing SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl HealthBotUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of identity that creates/modifies resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IdentityType")]
pub enum IdentityType {
    User,
    Application,
    ManagedIdentity,
    Key,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::User => serializer.serialize_unit_variant("IdentityType", 0u32, "User"),
            Self::Application => serializer.serialize_unit_variant("IdentityType", 1u32, "Application"),
            Self::ManagedIdentity => serializer.serialize_unit_variant("IdentityType", 2u32, "ManagedIdentity"),
            Self::Key => serializer.serialize_unit_variant("IdentityType", 3u32, "Key"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Operation detail payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDetail {
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Operation display payload"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Origin of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Additional properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl OperationDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation display payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Resource provider of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Localized friendly name for the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Localized friendly description for the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for a ARM tracked top level resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource Id for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Read only system data"]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition representing SKU"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The name of the HealthBot SKU"]
    pub name: sku::Name,
}
impl Sku {
    pub fn new(name: sku::Name) -> Self {
        Self { name }
    }
}
pub mod sku {
    use super::*;
    #[doc = "The name of the HealthBot SKU"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        F0,
        S1,
        C0,
    }
}
#[doc = "Read only system data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that creates/modifies resources"]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<IdentityType>,
    #[doc = "The timestamp of resource creation (UTC)"]
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that creates/modifies resources"]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<IdentityType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for a ARM tracked top level resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives"]
    pub location: String,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
            location,
        }
    }
}
#[doc = "The response returned from validation process"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidationResult {
    #[doc = "The status code of the response validation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl ValidationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
