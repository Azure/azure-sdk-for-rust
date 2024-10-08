#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "API resource model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Api {
    #[doc = "Api identifier."]
    pub name: String,
    #[doc = "The name of the API."]
    pub title: String,
    #[doc = "Short description of the API."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[doc = "The description of the API"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "API kind"]
    pub kind: ApiKind,
    #[doc = "API Lifecycle Stage"]
    #[serde(rename = "lifecycleStage", default, skip_serializing_if = "Option::is_none")]
    pub lifecycle_stage: Option<LifecycleStage>,
    #[doc = "API Terms of Service"]
    #[serde(rename = "termsOfService", default, skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<TermsOfService>,
    #[doc = "API License Model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
    #[doc = "External documentation"]
    #[serde(
        rename = "externalDocumentation",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub external_documentation: Vec<ExternalDocumentation>,
    #[doc = "Points of contact for the API."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub contacts: Vec<Contact>,
    #[doc = "The custom metadata defined for API entities."]
    #[serde(rename = "customProperties", default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<serde_json::Value>,
    #[doc = "Last updated date and time."]
    #[serde(rename = "lastUpdated", default, with = "azure_core::date::rfc3339::option")]
    pub last_updated: Option<::time::OffsetDateTime>,
}
impl Api {
    pub fn new(name: String, title: String, kind: ApiKind) -> Self {
        Self {
            name,
            title,
            summary: None,
            description: None,
            kind,
            lifecycle_stage: None,
            terms_of_service: None,
            license: None,
            external_documentation: Vec::new(),
            contacts: Vec::new(),
            custom_properties: None,
            last_updated: None,
        }
    }
}
#[doc = "API definition resource model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiDefinition {
    #[doc = "Definition identifier."]
    pub name: String,
    #[doc = "The name of the API definition."]
    pub title: String,
    #[doc = "The description of the API definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "API specification"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specification: Option<ApiSpecification>,
}
impl ApiDefinition {
    pub fn new(name: String, title: String) -> Self {
        Self {
            name,
            title,
            description: None,
            specification: None,
        }
    }
}
#[doc = "API deployment resource model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiDeployment {
    #[doc = "Deployment identifier."]
    pub name: String,
    #[doc = "The name of the deployment."]
    pub title: String,
    #[doc = "The description of the deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The name of the deployment environment."]
    pub environment: String,
    #[doc = "Api Deployment Server"]
    pub server: ApiDeploymentServer,
    #[doc = "The custom metadata defined for API deployment entities."]
    #[serde(rename = "customProperties", default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<serde_json::Value>,
    #[doc = "Indicates if this is currently recommended deployment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
}
impl ApiDeployment {
    pub fn new(name: String, title: String, environment: String, server: ApiDeploymentServer) -> Self {
        Self {
            name,
            title,
            description: None,
            environment,
            server,
            custom_properties: None,
            recommended: None,
        }
    }
}
#[doc = "Api Deployment Server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiDeploymentServer {
    #[doc = "Base runtime URIs for this deployment."]
    #[serde(rename = "runtimeUris")]
    pub runtime_uris: Vec<String>,
}
impl ApiDeploymentServer {
    pub fn new(runtime_uris: Vec<String>) -> Self {
        Self { runtime_uris }
    }
}
#[doc = "API kind"]
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
#[doc = "Export specification result model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiSpecExportResult {
    #[doc = "The result of the specification export."]
    pub value: String,
    #[doc = "Export specification result format"]
    pub format: ApiSpecExportResultFormat,
}
impl ApiSpecExportResult {
    pub fn new(value: String, format: ApiSpecExportResultFormat) -> Self {
        Self { value, format }
    }
}
#[doc = "Export specification result format"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ApiSpecExportResultFormat")]
pub enum ApiSpecExportResultFormat {
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
            Self::Link => serializer.serialize_unit_variant("ApiSpecExportResultFormat", 0u32, "link"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "API specification"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiSpecification {
    #[doc = "API specification name, e.g. 'swagger' or 'openapi'."]
    pub name: String,
    #[doc = "API specification version, e.g. '3.0.1'."]
    pub version: String,
}
impl ApiSpecification {
    pub fn new(name: String, version: String) -> Self {
        Self { name, version }
    }
}
#[doc = "API version resource model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiVersion {
    #[doc = "Version identifier."]
    pub name: String,
    #[doc = "Version title."]
    pub title: String,
    #[doc = "API Lifecycle Stage"]
    #[serde(rename = "lifecycleStage")]
    pub lifecycle_stage: LifecycleStage,
}
impl ApiVersion {
    pub fn new(name: String, title: String, lifecycle_stage: LifecycleStage) -> Self {
        Self {
            name,
            title,
            lifecycle_stage,
        }
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
#[doc = "Enum describing allowed operation states."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "AzureCoreFoundationsOperationState")]
pub enum AzureCoreFoundationsOperationState {
    NotStarted,
    Running,
    Succeeded,
    Failed,
    Canceled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for AzureCoreFoundationsOperationState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for AzureCoreFoundationsOperationState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for AzureCoreFoundationsOperationState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotStarted => serializer.serialize_unit_variant("AzureCoreFoundationsOperationState", 0u32, "NotStarted"),
            Self::Running => serializer.serialize_unit_variant("AzureCoreFoundationsOperationState", 1u32, "Running"),
            Self::Succeeded => serializer.serialize_unit_variant("AzureCoreFoundationsOperationState", 2u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("AzureCoreFoundationsOperationState", 3u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("AzureCoreFoundationsOperationState", 4u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "API contact information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    #[doc = "Name of the contact."]
    pub name: String,
    #[doc = "URL for the contact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Email address for the contact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
impl Contact {
    pub fn new(name: String) -> Self {
        Self {
            name,
            url: None,
            email: None,
        }
    }
}
#[doc = "Environment resource model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Environment {
    #[doc = "Environment identifier."]
    pub name: String,
    #[doc = "The name of the environment."]
    pub title: String,
    #[doc = "Environment kind"]
    pub kind: EnvironmentKind,
    #[doc = "Description of the environment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Environment Server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<EnvironmentServer>,
    #[doc = "Environment Onboarding Model"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboarding: Option<EnvironmentOnboardingModel>,
    #[doc = "The custom metadata defined for environment entities."]
    #[serde(rename = "customProperties", default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<serde_json::Value>,
}
impl Environment {
    pub fn new(name: String, title: String, kind: EnvironmentKind) -> Self {
        Self {
            name,
            title,
            kind,
            description: None,
            server: None,
            onboarding: None,
            custom_properties: None,
        }
    }
}
#[doc = "Environment kind"]
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
#[doc = "Environment Onboarding Model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentOnboardingModel {
    #[doc = "Instructions how to onboard to the environment."]
    pub instructions: String,
    #[doc = "Developer portal URIs of the environment."]
    #[serde(rename = "developerPortalUris")]
    pub developer_portal_uris: Vec<String>,
}
impl EnvironmentOnboardingModel {
    pub fn new(instructions: String, developer_portal_uris: Vec<String>) -> Self {
        Self {
            instructions,
            developer_portal_uris,
        }
    }
}
#[doc = "Environment Server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentServer {
    #[doc = "Type of the server that represents the environment."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "URIs of the server's management portal."]
    #[serde(rename = "managementPortalUris")]
    pub management_portal_uris: Vec<String>,
}
impl EnvironmentServer {
    pub fn new(type_: String, management_portal_uris: Vec<String>) -> Self {
        Self {
            type_,
            management_portal_uris,
        }
    }
}
#[doc = "Api External Documentation Model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExternalDocumentation {
    #[doc = "Title of the documentation."]
    pub title: String,
    #[doc = "Description of the documentation."]
    pub description: String,
    #[doc = "URL pointing to the documentation."]
    pub url: String,
}
impl ExternalDocumentation {
    pub fn new(title: String, description: String, url: String) -> Self {
        Self { title, description, url }
    }
}
#[doc = "API License Model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct License {
    #[doc = "Name of the license."]
    pub name: String,
    #[doc = "URL pointing to the license details. The URL field is mutually exclusive of the identifier field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "SPDX license information for the API. The identifier field is mutually exclusive of the URL field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}
impl License {
    pub fn new(name: String) -> Self {
        Self {
            name,
            url: None,
            identifier: None,
        }
    }
}
#[doc = "API Lifecycle Stage"]
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
#[doc = "Paged collection of Api items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedApi {
    #[doc = "The Api items on this page"]
    pub value: Vec<Api>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedApi {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedApi {
    pub fn new(value: Vec<Api>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of API items across workspaces."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedApiAll {
    #[doc = "The Api items on this page"]
    pub value: Vec<Api>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PagedApiAll {
    pub fn new(value: Vec<Api>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of ApiDefinition items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedApiDefinition {
    #[doc = "The ApiDefinition items on this page"]
    pub value: Vec<ApiDefinition>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedApiDefinition {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedApiDefinition {
    pub fn new(value: Vec<ApiDefinition>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of ApiDeployment items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedApiDeployment {
    #[doc = "The ApiDeployment items on this page"]
    pub value: Vec<ApiDeployment>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedApiDeployment {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedApiDeployment {
    pub fn new(value: Vec<ApiDeployment>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of ApiVersion items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedApiVersion {
    #[doc = "The ApiVersion items on this page"]
    pub value: Vec<ApiVersion>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedApiVersion {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedApiVersion {
    pub fn new(value: Vec<ApiVersion>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of Environment items"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedEnvironment {
    #[doc = "The Environment items on this page"]
    pub value: Vec<Environment>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedEnvironment {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedEnvironment {
    pub fn new(value: Vec<Environment>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Paged collection of Environment items across workspaces"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedEnvironmentAll {
    #[doc = "The Environment items on this page"]
    pub value: Vec<Environment>,
    #[doc = "The link to the next page of items"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PagedEnvironmentAll {
    pub fn new(value: Vec<Environment>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "API Terms of Service"]
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
#[doc = "Workspace resource model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    #[doc = "Workspace identifier."]
    pub name: String,
    #[doc = "Workspace title, e.g. 'My workspace'."]
    pub title: String,
    #[doc = "Workspace description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl Workspace {
    pub fn new(name: String, title: String) -> Self {
        Self {
            name,
            title,
            description: None,
        }
    }
}
