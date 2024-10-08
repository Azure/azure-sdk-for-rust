#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The type of action"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ActionType")]
pub enum ActionType {
    Internal,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ActionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ActionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ActionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Internal => serializer.serialize_unit_variant("ActionType", 0u32, "Internal"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "API entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Api {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "API properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApiProperties>,
}
impl Api {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API definition entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiDefinition {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "API definition properties entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApiDefinitionProperties>,
}
impl ApiDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a ApiDefinition list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiDefinitionListResult {
    #[doc = "The ApiDefinition items on this page"]
    pub value: Vec<ApiDefinition>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApiDefinitionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ApiDefinitionListResult {
    pub fn new(value: Vec<ApiDefinition>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "API definition properties entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiDefinitionProperties {
    #[doc = "API definition title."]
    pub title: String,
    #[doc = "API definition description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "API specification details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specification: Option<ApiDefinitionPropertiesSpecification>,
}
impl ApiDefinitionProperties {
    pub fn new(title: String) -> Self {
        Self {
            title,
            description: None,
            specification: None,
        }
    }
}
#[doc = "API specification details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiDefinitionPropertiesSpecification {
    #[doc = "Specification name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specification version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ApiDefinitionPropertiesSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The API specification was successfully imported."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiImportSuccess {}
impl ApiImportSuccess {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kind of the API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ApiKind")]
pub enum ApiKind {
    #[serde(rename = "rest")]
    Rest,
    #[serde(rename = "graphql")]
    Graphql,
    #[serde(rename = "grpc")]
    Grpc,
    #[serde(rename = "soap")]
    Soap,
    #[serde(rename = "webhook")]
    Webhook,
    #[serde(rename = "websocket")]
    Websocket,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ApiKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ApiKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ApiKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Rest => serializer.serialize_unit_variant("ApiKind", 0u32, "rest"),
            Self::Graphql => serializer.serialize_unit_variant("ApiKind", 1u32, "graphql"),
            Self::Grpc => serializer.serialize_unit_variant("ApiKind", 2u32, "grpc"),
            Self::Soap => serializer.serialize_unit_variant("ApiKind", 3u32, "soap"),
            Self::Webhook => serializer.serialize_unit_variant("ApiKind", 4u32, "webhook"),
            Self::Websocket => serializer.serialize_unit_variant("ApiKind", 5u32, "websocket"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The response of a Api list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiListResult {
    #[doc = "The Api items on this page"]
    pub value: Vec<Api>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApiListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ApiListResult {
    pub fn new(value: Vec<Api>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "API properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiProperties {
    #[doc = "API title."]
    pub title: String,
    #[doc = "The kind of the API"]
    pub kind: ApiKind,
    #[doc = "Description of the API."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Short description of the API."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[doc = "The stage of the Api development lifecycle"]
    #[serde(rename = "lifecycleStage", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_stage: Option<LifecycleStage>,
    #[doc = "Terms of service for the API."]
    #[serde(rename = "termsOfService", default, skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<TermsOfService>,
    #[doc = "The set of external documentation"]
    #[serde(
        rename = "externalDocumentation",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub external_documentation: Vec<ExternalDocumentation>,
    #[doc = "The set of contacts"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub contacts: Vec<Contact>,
    #[doc = "The license information for the API."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
    #[doc = "The custom metadata defined for API catalog entities."]
    #[serde(rename = "customProperties", default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<CustomProperties>,
}
impl ApiProperties {
    pub fn new(title: String, kind: ApiKind) -> Self {
        Self {
            title,
            kind,
            description: None,
            summary: None,
            lifecycle_stage: None,
            terms_of_service: None,
            external_documentation: Vec::new(),
            contacts: Vec::new(),
            license: None,
            custom_properties: None,
        }
    }
}
#[doc = "The API specification export result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiSpecExportResult {
    #[doc = "Result format for exported Api spec"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<ApiSpecExportResultFormat>,
    #[doc = "The result of the export operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ApiSpecExportResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result format for exported Api spec"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ApiSpecExportResultFormat")]
pub enum ApiSpecExportResultFormat {
    #[serde(rename = "inline")]
    Inline,
    #[serde(rename = "link")]
    Link,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ApiSpecExportResultFormat {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ApiSpecExportResultFormat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ApiSpecExportResultFormat {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Inline => serializer.serialize_unit_variant("ApiSpecExportResultFormat", 0u32, "inline"),
            Self::Link => serializer.serialize_unit_variant("ApiSpecExportResultFormat", 1u32, "link"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The API specification source entity properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiSpecImportRequest {
    #[doc = "Value of the API specification source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Source format for imported Api spec"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<ApiSpecImportSourceFormat>,
    #[doc = "API specification details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specification: Option<ApiSpecImportRequestSpecification>,
}
impl ApiSpecImportRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API specification details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiSpecImportRequestSpecification {
    #[doc = "Specification name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specification version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ApiSpecImportRequestSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Source format for imported Api spec"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ApiSpecImportSourceFormat")]
pub enum ApiSpecImportSourceFormat {
    #[serde(rename = "inline")]
    Inline,
    #[serde(rename = "link")]
    Link,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ApiSpecImportSourceFormat {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ApiSpecImportSourceFormat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ApiSpecImportSourceFormat {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Inline => serializer.serialize_unit_variant("ApiSpecImportSourceFormat", 0u32, "inline"),
            Self::Link => serializer.serialize_unit_variant("ApiSpecImportSourceFormat", 1u32, "link"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "API version entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiVersion {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "API version properties entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApiVersionProperties>,
}
impl ApiVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a ApiVersion list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiVersionListResult {
    #[doc = "The ApiVersion items on this page"]
    pub value: Vec<ApiVersion>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApiVersionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ApiVersionListResult {
    pub fn new(value: Vec<ApiVersion>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "API version properties entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiVersionProperties {
    #[doc = "API version title."]
    pub title: String,
    #[doc = "The stage of the Api development lifecycle"]
    #[serde(rename = "lifecycleStage")]
    pub lifecycle_stage: LifecycleStage,
}
impl ApiVersionProperties {
    pub fn new(title: String, lifecycle_stage: LifecycleStage) -> Self {
        Self { title, lifecycle_stage }
    }
}
#[doc = "Contact information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Contact {
    #[doc = "Name of the contact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "URL for the contact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Email address of the contact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
impl Contact {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "How the resource was created"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CreatedByType")]
pub enum CreatedByType {
    User,
    Application,
    ManagedIdentity,
    Key,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CreatedByType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CreatedByType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CreatedByType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::User => serializer.serialize_unit_variant("CreatedByType", 0u32, "User"),
            Self::Application => serializer.serialize_unit_variant("CreatedByType", 1u32, "Application"),
            Self::ManagedIdentity => serializer.serialize_unit_variant("CreatedByType", 2u32, "ManagedIdentity"),
            Self::Key => serializer.serialize_unit_variant("CreatedByType", 3u32, "Key"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The custom metadata defined for API catalog entities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomProperties {}
impl CustomProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API deployment entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Deployment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "API deployment entity properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeploymentProperties>,
}
impl Deployment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a Deployment list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentListResult {
    #[doc = "The Deployment items on this page"]
    pub value: Vec<Deployment>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeploymentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DeploymentListResult {
    pub fn new(value: Vec<Deployment>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "API deployment entity properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentProperties {
    #[doc = "API deployment title"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Description of the deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "API center-scoped environment resource ID."]
    #[serde(rename = "environmentId", default, skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[doc = "API center-scoped definition resource ID."]
    #[serde(rename = "definitionId", default, skip_serializing_if = "Option::is_none")]
    pub definition_id: Option<String>,
    #[doc = "State of the Deployment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<DeploymentState>,
    #[doc = "Server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<DeploymentServer>,
    #[doc = "The custom metadata defined for API catalog entities."]
    #[serde(rename = "customProperties", default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<CustomProperties>,
}
impl DeploymentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentServer {
    #[doc = "Base runtime URLs for this deployment."]
    #[serde(
        rename = "runtimeUri",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub runtime_uri: Vec<SmallString>,
}
impl DeploymentServer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "State of the Deployment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DeploymentState")]
pub enum DeploymentState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DeploymentState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DeploymentState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DeploymentState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Active => serializer.serialize_unit_variant("DeploymentState", 0u32, "active"),
            Self::Inactive => serializer.serialize_unit_variant("DeploymentState", 1u32, "inactive"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Environment entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Environment {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Environment properties entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnvironmentProperties>,
}
impl Environment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kind of environment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EnvironmentKind")]
pub enum EnvironmentKind {
    #[serde(rename = "development")]
    Development,
    #[serde(rename = "testing")]
    Testing,
    #[serde(rename = "staging")]
    Staging,
    #[serde(rename = "production")]
    Production,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EnvironmentKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EnvironmentKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EnvironmentKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Development => serializer.serialize_unit_variant("EnvironmentKind", 0u32, "development"),
            Self::Testing => serializer.serialize_unit_variant("EnvironmentKind", 1u32, "testing"),
            Self::Staging => serializer.serialize_unit_variant("EnvironmentKind", 2u32, "staging"),
            Self::Production => serializer.serialize_unit_variant("EnvironmentKind", 3u32, "production"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The response of a Environment list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentListResult {
    #[doc = "The Environment items on this page"]
    pub value: Vec<Environment>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EnvironmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl EnvironmentListResult {
    pub fn new(value: Vec<Environment>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Environment properties entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentProperties {
    #[doc = "Environment title."]
    pub title: String,
    #[doc = "The environment description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The kind of environment"]
    pub kind: EnvironmentKind,
    #[doc = "Server information of the environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<EnvironmentServer>,
    #[doc = "Onboarding information"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboarding: Option<Onboarding>,
    #[doc = "The custom metadata defined for API catalog entities."]
    #[serde(rename = "customProperties", default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<CustomProperties>,
}
impl EnvironmentProperties {
    pub fn new(title: String, kind: EnvironmentKind) -> Self {
        Self {
            title,
            description: None,
            kind,
            server: None,
            onboarding: None,
            custom_properties: None,
        }
    }
}
#[doc = "Server information of the environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentServer {
    #[doc = "The type of environment server"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<EnvironmentServerType>,
    #[doc = "The location of the management portal"]
    #[serde(
        rename = "managementPortalUri",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub management_portal_uri: Vec<String>,
}
impl EnvironmentServer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of environment server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EnvironmentServerType")]
pub enum EnvironmentServerType {
    #[serde(rename = "Azure API Management")]
    AzureApiManagement,
    #[serde(rename = "Azure compute service")]
    AzureComputeService,
    #[serde(rename = "Apigee API Management")]
    ApigeeApiManagement,
    #[serde(rename = "AWS API Gateway")]
    AwsApiGateway,
    #[serde(rename = "Kong API Gateway")]
    KongApiGateway,
    Kubernetes,
    #[serde(rename = "MuleSoft API Management")]
    MuleSoftApiManagement,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EnvironmentServerType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EnvironmentServerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EnvironmentServerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AzureApiManagement => serializer.serialize_unit_variant("EnvironmentServerType", 0u32, "Azure API Management"),
            Self::AzureComputeService => serializer.serialize_unit_variant("EnvironmentServerType", 1u32, "Azure compute service"),
            Self::ApigeeApiManagement => serializer.serialize_unit_variant("EnvironmentServerType", 2u32, "Apigee API Management"),
            Self::AwsApiGateway => serializer.serialize_unit_variant("EnvironmentServerType", 3u32, "AWS API Gateway"),
            Self::KongApiGateway => serializer.serialize_unit_variant("EnvironmentServerType", 4u32, "Kong API Gateway"),
            Self::Kubernetes => serializer.serialize_unit_variant("EnvironmentServerType", 5u32, "Kubernetes"),
            Self::MuleSoftApiManagement => serializer.serialize_unit_variant("EnvironmentServerType", 6u32, "MuleSoft API Management"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
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
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(
        rename = "additionalInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
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
#[doc = "Additional, external documentation for the API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExternalDocumentation {
    #[doc = "Title of the documentation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Description of the documentation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "URL pointing to the documentation."]
    pub url: String,
}
impl ExternalDocumentation {
    pub fn new(url: String) -> Self {
        Self {
            title: None,
            description: None,
            url,
        }
    }
}
#[doc = "The license information for the API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct License {
    #[doc = "Name of the license."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "URL pointing to the license details. The URL field is mutually exclusive of the\nidentifier field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "SPDX license information for the API. The identifier field is mutually\nexclusive of the URL field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}
impl License {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The stage of the Api development lifecycle"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LifecycleStage")]
pub enum LifecycleStage {
    #[serde(rename = "design")]
    Design,
    #[serde(rename = "development")]
    Development,
    #[serde(rename = "testing")]
    Testing,
    #[serde(rename = "preview")]
    Preview,
    #[serde(rename = "production")]
    Production,
    #[serde(rename = "deprecated")]
    Deprecated,
    #[serde(rename = "retired")]
    Retired,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LifecycleStage {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LifecycleStage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LifecycleStage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Design => serializer.serialize_unit_variant("LifecycleStage", 0u32, "design"),
            Self::Development => serializer.serialize_unit_variant("LifecycleStage", 1u32, "development"),
            Self::Testing => serializer.serialize_unit_variant("LifecycleStage", 2u32, "testing"),
            Self::Preview => serializer.serialize_unit_variant("LifecycleStage", 3u32, "preview"),
            Self::Production => serializer.serialize_unit_variant("LifecycleStage", 4u32, "production"),
            Self::Deprecated => serializer.serialize_unit_variant("LifecycleStage", 5u32, "deprecated"),
            Self::Retired => serializer.serialize_unit_variant("LifecycleStage", 6u32, "retired"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Managed service identity (system assigned and/or user assigned identities)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedServiceIdentity {
    #[doc = "The service principal ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
    #[serde(rename = "type")]
    pub type_: ManagedServiceIdentityType,
    #[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedIdentities>,
}
impl ManagedServiceIdentity {
    pub fn new(type_: ManagedServiceIdentityType) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
            user_assigned_identities: None,
        }
    }
}
#[doc = "The type of managed service identity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedServiceIdentityType")]
pub enum ManagedServiceIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
    #[serde(rename = "SystemAssigned,UserAssigned")]
    SystemAssignedUserAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagedServiceIdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagedServiceIdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagedServiceIdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ManagedServiceIdentityType", 0u32, "None"),
            Self::SystemAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 1u32, "SystemAssigned"),
            Self::UserAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 2u32, "UserAssigned"),
            Self::SystemAssignedUserAssigned => {
                serializer.serialize_unit_variant("ManagedServiceIdentityType", 3u32, "SystemAssigned,UserAssigned")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Assignment metadata"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataAssignment {
    #[doc = "Assignment entity for Metadata"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity: Option<MetadataAssignmentEntity>,
    #[doc = "Required assignment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[doc = "Deprecated assignment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
}
impl MetadataAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Assignment entity for Metadata"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MetadataAssignmentEntity")]
pub enum MetadataAssignmentEntity {
    #[serde(rename = "api")]
    Api,
    #[serde(rename = "environment")]
    Environment,
    #[serde(rename = "deployment")]
    Deployment,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MetadataAssignmentEntity {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MetadataAssignmentEntity {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MetadataAssignmentEntity {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Api => serializer.serialize_unit_variant("MetadataAssignmentEntity", 0u32, "api"),
            Self::Environment => serializer.serialize_unit_variant("MetadataAssignmentEntity", 1u32, "environment"),
            Self::Deployment => serializer.serialize_unit_variant("MetadataAssignmentEntity", 2u32, "deployment"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Metadata schema entity. Used to define metadata for the entities in API catalog."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataSchema {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Metadata schema properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MetadataSchemaProperties>,
}
impl MetadataSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The format for schema export"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MetadataSchemaExportFormat")]
pub enum MetadataSchemaExportFormat {
    #[serde(rename = "inline")]
    Inline,
    #[serde(rename = "link")]
    Link,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MetadataSchemaExportFormat {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MetadataSchemaExportFormat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MetadataSchemaExportFormat {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Inline => serializer.serialize_unit_variant("MetadataSchemaExportFormat", 0u32, "inline"),
            Self::Link => serializer.serialize_unit_variant("MetadataSchemaExportFormat", 1u32, "link"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The metadata schema export request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataSchemaExportRequest {
    #[doc = "Assignment entity for Metadata"]
    #[serde(rename = "assignedTo", default, skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<MetadataAssignmentEntity>,
}
impl MetadataSchemaExportRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The metadata schema export result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataSchemaExportResult {
    #[doc = "The format for schema export"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<MetadataSchemaExportFormat>,
    #[doc = "The result of the export operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl MetadataSchemaExportResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a MetadataSchema list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataSchemaListResult {
    #[doc = "The MetadataSchema items on this page"]
    pub value: Vec<MetadataSchema>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for MetadataSchemaListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl MetadataSchemaListResult {
    pub fn new(value: Vec<MetadataSchema>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Metadata schema properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataSchemaProperties {
    #[doc = "The schema defining the type."]
    pub schema: String,
    #[doc = "The assignees"]
    #[serde(
        rename = "assignedTo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub assigned_to: Vec<MetadataAssignment>,
}
impl MetadataSchemaProperties {
    pub fn new(schema: String) -> Self {
        Self {
            schema,
            assigned_to: Vec::new(),
        }
    }
}
#[doc = "Onboarding information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Onboarding {
    #[doc = "Onboarding guide."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[doc = "The location of the development portal"]
    #[serde(
        rename = "developerPortalUri",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub developer_portal_uri: Vec<String>,
}
impl Onboarding {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of a REST API operation, returned from the Resource Provider Operations API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation, as per Resource-Based Access Control (RBAC). Examples: \"Microsoft.Compute/virtualMachines/write\", \"Microsoft.Compute/virtualMachines/capture/action\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Whether the operation applies to data-plane. This is \"true\" for data-plane operations and \"false\" for ARM/control-plane operations."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Localized display information for this particular operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<operation::ActionType>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Localized display information for this particular operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The localized friendly form of the resource provider name, e.g. \"Microsoft Monitoring Insights\" or \"Microsoft Compute\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The localized friendly name of the resource type related to this operation. E.g. \"Virtual Machines\" or \"Job Schedule Collections\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The concise, localized friendly name for the operation; suitable for dropdowns. E.g. \"Create or Update Virtual Machine\", \"Restart Virtual Machine\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The short, localized friendly description of the operation; suitable for tool tips and detailed views."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Origin")]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "user,system")]
        UserSystem,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Origin {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Origin {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Origin {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("Origin", 0u32, "user"),
                Self::System => serializer.serialize_unit_variant("Origin", 1u32, "system"),
                Self::UserSystem => serializer.serialize_unit_variant("Origin", 2u32, "user,system"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActionType")]
    pub enum ActionType {
        Internal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Internal => serializer.serialize_unit_variant("ActionType", 0u32, "Internal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of REST API operations supported by an Azure Resource Provider. It contains an URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the resource provider"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results (if there are any)."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The expected types of origin"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Origin")]
pub enum Origin {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user,system")]
    UserSystem,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Origin {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Origin {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Origin {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::User => serializer.serialize_unit_variant("Origin", 0u32, "user"),
            Self::System => serializer.serialize_unit_variant("Origin", 1u32, "system"),
            Self::UserSystem => serializer.serialize_unit_variant("Origin", 2u32, "user,system"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The provisioning state of the resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
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
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The resource model definition for a Azure Resource Manager proxy resource. It will not have tags and a location"]
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
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. E.g. \"/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.Compute/virtualMachines\" or \"Microsoft.Storage/storageAccounts\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The service entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The properties of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceProperties>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl Service {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            identity: None,
        }
    }
}
#[doc = "The response of a Service list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceListResult {
    #[doc = "The Service items on this page"]
    pub value: Vec<Service>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServiceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ServiceListResult {
    pub fn new(value: Vec<Service>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceProperties {
    #[doc = "The provisioning state of the resource"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type used for update operations of the Service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceUpdate {
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ServiceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type SmallString = String;
#[doc = "Terms of service for the API."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TermsOfService {
    #[doc = "URL pointing to the terms of service."]
    pub url: String,
}
impl TermsOfService {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}
#[doc = "The resource model definition for an Azure Resource Manager tracked top level resource which has 'tags' and a 'location'"]
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
#[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentities {}
impl UserAssignedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User assigned identity properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "The principal ID of the assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client ID of the assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workspace entity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Workspace {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Workspace properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceProperties>,
}
impl Workspace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a Workspace list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceListResult {
    #[doc = "The Workspace items on this page"]
    pub value: Vec<Workspace>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkspaceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl WorkspaceListResult {
    pub fn new(value: Vec<Workspace>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Workspace properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceProperties {
    #[doc = "Workspace title."]
    pub title: String,
    #[doc = "Workspace description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl WorkspaceProperties {
    pub fn new(title: String) -> Self {
        Self { title, description: None }
    }
}
#[doc = "Metadata pertaining to creation and last modification of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<::time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<::time::OffsetDateTime>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_data {
    use super::*;
    #[doc = "The type of identity that created the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreatedByType")]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CreatedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CreatedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CreatedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("CreatedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("CreatedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("CreatedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("CreatedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of identity that last modified the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LastModifiedByType")]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LastModifiedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LastModifiedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LastModifiedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("LastModifiedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("LastModifiedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("LastModifiedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("LastModifiedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
