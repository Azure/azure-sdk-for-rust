#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Object that includes an array of Deployment resource name and set them as active."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActiveDeploymentCollection {
    #[doc = "Collection of Deployment name."]
    #[serde(rename = "activeDeploymentNames", default, skip_serializing_if = "Vec::is_empty")]
    pub active_deployment_names: Vec<String>,
}
impl ActiveDeploymentCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Key-value pairs for configurations of add-on."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddonProfile {}
impl AddonProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of custom domain for API portal"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiPortalCustomDomainProperties {
    #[doc = "The thumbprint of bound certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
}
impl ApiPortalCustomDomainProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Custom domain of the API portal"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiPortalCustomDomainResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of custom domain for API portal"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApiPortalCustomDomainProperties>,
}
impl ApiPortalCustomDomainResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of API portal custom domain resources and a possible link for next set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiPortalCustomDomainResourceCollection {
    #[doc = "Collection of API portal custom domain resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApiPortalCustomDomainResource>,
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApiPortalCustomDomainResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ApiPortalCustomDomainResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of instances belong to the API portal"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiPortalInstance {
    #[doc = "Name of the API portal instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Status of the API portal instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl ApiPortalInstance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API portal properties payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiPortalProperties {
    #[doc = "State of the API portal."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<api_portal_properties::ProvisioningState>,
    #[doc = "Indicates whether the API portal exposes endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[doc = "URL of the API portal, exposed when 'public' is true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Indicate if only https is allowed."]
    #[serde(rename = "httpsOnly", default, skip_serializing_if = "Option::is_none")]
    pub https_only: Option<bool>,
    #[doc = "The array of resource Ids of gateway to integrate with API portal."]
    #[serde(rename = "gatewayIds", default, skip_serializing_if = "Vec::is_empty")]
    pub gateway_ids: Vec<String>,
    #[doc = "Collection of OpenAPI source URL locations."]
    #[serde(rename = "sourceUrls", default, skip_serializing_if = "Vec::is_empty")]
    pub source_urls: Vec<String>,
    #[doc = "Single sign-on related configuration"]
    #[serde(rename = "ssoProperties", default, skip_serializing_if = "Option::is_none")]
    pub sso_properties: Option<SsoProperties>,
    #[doc = "Resource requests of the API portal"]
    #[serde(rename = "resourceRequests", default, skip_serializing_if = "Option::is_none")]
    pub resource_requests: Option<ApiPortalResourceRequests>,
    #[doc = "Collection of instances belong to API portal."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instances: Vec<ApiPortalInstance>,
}
impl ApiPortalProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod api_portal_properties {
    use super::*;
    #[doc = "State of the API portal."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Succeeded,
        Failed,
        Deleting,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "API portal resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiPortalResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "API portal properties payload"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApiPortalProperties>,
    #[doc = "Sku of Azure Spring Cloud"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl ApiPortalResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of API portal resources and a possible link for next set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiPortalResourceCollection {
    #[doc = "Collection of API portal resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApiPortalResource>,
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ApiPortalResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ApiPortalResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource requests of the API portal"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiPortalResourceRequests {
    #[doc = "Cpu allocated to each API portal instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[doc = "Memory allocated to each API portal instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
}
impl ApiPortalResourceRequests {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "App resource payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "App resource properties payload"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AppResourceProperties>,
    #[doc = "Managed identity properties retrieved from ARM request headers."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedIdentityProperties>,
    #[doc = "The GEO location of the application, always the same with its parent resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl AppResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of App resources and a possible link for next set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppResourceCollection {
    #[doc = "Collection of App resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AppResource>,
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AppResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AppResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "App resource properties payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppResourceProperties {
    #[doc = "Indicates whether the App exposes public endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[doc = "URL of the App"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Collection of addons"]
    #[serde(rename = "addonConfigs", default, skip_serializing_if = "Option::is_none")]
    pub addon_configs: Option<serde_json::Value>,
    #[doc = "Provisioning state of the App"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<app_resource_properties::ProvisioningState>,
    #[doc = "Fully qualified dns Name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[doc = "Indicate if only https is allowed."]
    #[serde(rename = "httpsOnly", default, skip_serializing_if = "Option::is_none")]
    pub https_only: Option<bool>,
    #[doc = "Temporary disk payload"]
    #[serde(rename = "temporaryDisk", default, skip_serializing_if = "Option::is_none")]
    pub temporary_disk: Option<TemporaryDisk>,
    #[doc = "Persistent disk payload"]
    #[serde(rename = "persistentDisk", default, skip_serializing_if = "Option::is_none")]
    pub persistent_disk: Option<PersistentDisk>,
    #[doc = "Collection of persistent disk resources list and a possible link for next page."]
    #[serde(rename = "customPersistentDisks", default, skip_serializing_if = "Option::is_none")]
    pub custom_persistent_disks: Option<CustomPersistentDiskCollection>,
    #[doc = "Indicate if end to end TLS is enabled."]
    #[serde(rename = "enableEndToEndTLS", default, skip_serializing_if = "Option::is_none")]
    pub enable_end_to_end_tls: Option<bool>,
    #[doc = "Collection of loaded certificate resources list and a possible link for next page."]
    #[serde(rename = "loadedCertificates", default, skip_serializing_if = "Option::is_none")]
    pub loaded_certificates: Option<LoadedCertificateCollection>,
}
impl AppResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod app_resource_properties {
    use super::*;
    #[doc = "Provisioning state of the App"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Succeeded,
        Failed,
        Creating,
        Updating,
        Deleting,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Updating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableRuntimeVersions {
    #[doc = "A list of all supported runtime versions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SupportedRuntimeVersion>,
}
impl AvailableRuntimeVersions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the Azure File volume. Azure File shares are mounted as volumes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileVolume {
    #[serde(flatten)]
    pub custom_persistent_disk_properties: CustomPersistentDiskProperties,
    #[doc = "The share name of the Azure File share."]
    #[serde(rename = "shareName")]
    pub share_name: String,
}
impl AzureFileVolume {
    pub fn new(custom_persistent_disk_properties: CustomPersistentDiskProperties, share_name: String) -> Self {
        Self {
            custom_persistent_disk_properties,
            share_name,
        }
    }
}
#[doc = "Binding resource payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BindingResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Binding resource properties payload"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BindingResourceProperties>,
}
impl BindingResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of Binding resources and a possible link for next set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BindingResourceCollection {
    #[doc = "Collection of Binding resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BindingResource>,
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BindingResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BindingResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Binding resource properties payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BindingResourceProperties {
    #[doc = "The name of the bound resource"]
    #[serde(rename = "resourceName", default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "The standard Azure resource type of the bound resource"]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The Azure resource id of the bound resource"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The key of the bound resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "Binding parameters of the Binding resource"]
    #[serde(rename = "bindingParameters", default, skip_serializing_if = "Option::is_none")]
    pub binding_parameters: Option<serde_json::Value>,
    #[doc = "The generated Spring Boot property file for this binding. The secret will be deducted."]
    #[serde(rename = "generatedProperties", default, skip_serializing_if = "Option::is_none")]
    pub generated_properties: Option<String>,
    #[doc = "Creation time of the Binding resource"]
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[doc = "Update time of the Binding resource"]
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}
impl BindingResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Build resource payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Build {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Build resource properties payload"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BuildProperties>,
}
impl Build {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of Build resources and a possible link for next set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildCollection {
    #[doc = "Collection of Build resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Build>,
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BuildCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BuildCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Build resource properties payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildProperties {
    #[doc = "The relative path of source code"]
    #[serde(rename = "relativePath", default, skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
    #[doc = "The resource id of builder to build the source code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub builder: Option<String>,
    #[doc = "The resource id of agent pool"]
    #[serde(rename = "agentPool", default, skip_serializing_if = "Option::is_none")]
    pub agent_pool: Option<String>,
    #[doc = "Provisioning state of the KPack build result"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<build_properties::ProvisioningState>,
    #[doc = "The environment variables for this build"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<serde_json::Value>,
    #[doc = "The build result triggered by a build"]
    #[serde(rename = "triggeredBuildResult", default, skip_serializing_if = "Option::is_none")]
    pub triggered_build_result: Option<TriggeredBuildResult>,
}
impl BuildProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod build_properties {
    use super::*;
    #[doc = "Provisioning state of the KPack build result"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Succeeded,
        Failed,
        Deleting,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Build result resource payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildResult {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Build result resource properties payload"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BuildResultProperties>,
}
impl BuildResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of Build result resources and a possible link for next set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildResultCollection {
    #[doc = "Collection of Build result resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BuildResult>,
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BuildResultCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BuildResultCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Build result log resource properties payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildResultLog {
    #[doc = "The public download URL of this build result log"]
    #[serde(rename = "blobUrl", default, skip_serializing_if = "Option::is_none")]
    pub blob_url: Option<String>,
}
impl BuildResultLog {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Build result resource properties payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildResultProperties {
    #[doc = "The name of this build result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Provisioning state of the KPack build result"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<build_result_properties::ProvisioningState>,
    #[doc = "The build pod name which can be used to get the build log streaming."]
    #[serde(rename = "buildPodName", default, skip_serializing_if = "Option::is_none")]
    pub build_pod_name: Option<String>,
    #[doc = "All of the build stage (init-container and container) resources in build pod."]
    #[serde(rename = "buildStages", default, skip_serializing_if = "Vec::is_empty")]
    pub build_stages: Vec<BuildStageProperties>,
}
impl BuildResultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod build_result_properties {
    use super::*;
    #[doc = "Provisioning state of the KPack build result"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Queuing,
        Building,
        Succeeded,
        Failed,
        Deleting,
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
                Self::Queuing => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Queuing"),
                Self::Building => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Building"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Reference to a build result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuildResultUserSourceInfo {
    #[serde(flatten)]
    pub user_source_info: UserSourceInfo,
    #[doc = "Resource id of an existing succeeded build result under the same Spring instance."]
    #[serde(rename = "buildResultId", default, skip_serializing_if = "Option::is_none")]
    pub build_result_id: Option<String>,
}
impl BuildResultUserSourceInfo {
    pub fn new(user_source_info: UserSourceInfo) -> Self {
        Self {
            user_source_info,
            build_result_id: None,
        }
    }
}
#[doc = "Build service resource payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildService {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Build service resource properties payload"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BuildServiceProperties>,
}
impl BuildService {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Build service agent pool properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildServiceAgentPoolProperties {
    #[doc = "Provisioning state of the build service agent pool"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Build service agent pool size properties"]
    #[serde(rename = "poolSize", default, skip_serializing_if = "Option::is_none")]
    pub pool_size: Option<BuildServiceAgentPoolSizeProperties>,
}
impl BuildServiceAgentPoolProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The build service agent pool resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildServiceAgentPoolResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Build service agent pool properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BuildServiceAgentPoolProperties>,
}
impl BuildServiceAgentPoolResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of build service agent pool resources and a possible link for next set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildServiceAgentPoolResourceCollection {
    #[doc = "Collection of build service agent pool resource"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BuildServiceAgentPoolResource>,
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BuildServiceAgentPoolResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BuildServiceAgentPoolResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Build service agent pool size properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildServiceAgentPoolSizeProperties {
    #[doc = "The name of build service agent pool size"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The cpu property of build service agent pool size"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[doc = "The memory property of build service agent pool size"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
}
impl BuildServiceAgentPoolSizeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of Build service resources and a possible link for next set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildServiceCollection {
    #[doc = "Collection of Build service resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BuildService>,
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BuildServiceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BuildServiceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Build service resource properties payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildServiceProperties {
    #[doc = "The installed KPack version in this build service."]
    #[serde(rename = "kPackVersion", default, skip_serializing_if = "Option::is_none")]
    pub k_pack_version: Option<String>,
    #[doc = "Provisioning state of the KPack build result"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<build_service_properties::ProvisioningState>,
    #[doc = "The runtime resource configuration of this build service."]
    #[serde(rename = "resourceRequests", default, skip_serializing_if = "Option::is_none")]
    pub resource_requests: Option<build_service_properties::ResourceRequests>,
}
impl BuildServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod build_service_properties {
    use super::*;
    #[doc = "Provisioning state of the KPack build result"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Succeeded,
        Failed,
        Deleting,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The runtime resource configuration of this build service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct ResourceRequests {
        #[doc = "vCPU allocated to the entire build service node pool."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cpu: Option<String>,
        #[doc = "Memory allocated to the entire build service node pool."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub memory: Option<String>,
    }
    impl ResourceRequests {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The build stage (init-container and container) resources in build pod."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildStageProperties {
    #[doc = "The name of this build stage resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The provisioning state of this build stage resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<build_stage_properties::Status>,
}
impl BuildStageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod build_stage_properties {
    use super::*;
    #[doc = "The provisioning state of this build stage resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        NotStarted,
        Running,
        Succeeded,
        Failed,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Status {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Status {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Status {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::NotStarted => serializer.serialize_unit_variant("Status", 0u32, "NotStarted"),
                Self::Running => serializer.serialize_unit_variant("Status", 1u32, "Running"),
                Self::Succeeded => serializer.serialize_unit_variant("Status", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("Status", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "KPack Builder properties payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuilderProperties {
    #[doc = "Builder provision status."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<builder_properties::ProvisioningState>,
    #[doc = "KPack ClusterStack properties payload"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stack: Option<StackProperties>,
    #[doc = "Builder buildpack groups."]
    #[serde(rename = "buildpackGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub buildpack_groups: Vec<BuildpacksGroupProperties>,
}
impl BuilderProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod builder_properties {
    use super::*;
    #[doc = "Builder provision status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Succeeded,
        Failed,
        Deleting,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "KPack Builder resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuilderResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "KPack Builder properties payload"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BuilderProperties>,
}
impl BuilderResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of Builder resources and a possible link for next set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuilderResourceCollection {
    #[doc = "Collection of Builder resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BuilderResource>,
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BuilderResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BuilderResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Buildpack Binding Launch Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildpackBindingLaunchProperties {
    #[doc = "Non-sensitive properties for launchProperties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Sensitive properties for launchProperties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<serde_json::Value>,
}
impl BuildpackBindingLaunchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a buildpack binding"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildpackBindingProperties {
    #[doc = "Buildpack Binding Type"]
    #[serde(rename = "bindingType", default, skip_serializing_if = "Option::is_none")]
    pub binding_type: Option<buildpack_binding_properties::BindingType>,
    #[doc = "State of the Buildpack Binding."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<buildpack_binding_properties::ProvisioningState>,
    #[doc = "Buildpack Binding Launch Properties"]
    #[serde(rename = "launchProperties", default, skip_serializing_if = "Option::is_none")]
    pub launch_properties: Option<BuildpackBindingLaunchProperties>,
}
impl BuildpackBindingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod buildpack_binding_properties {
    use super::*;
    #[doc = "Buildpack Binding Type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "BindingType")]
    pub enum BindingType {
        ApplicationInsights,
        ApacheSkyWalking,
        AppDynamics,
        Dynatrace,
        NewRelic,
        #[serde(rename = "ElasticAPM")]
        ElasticApm,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for BindingType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for BindingType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for BindingType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::ApplicationInsights => serializer.serialize_unit_variant("BindingType", 0u32, "ApplicationInsights"),
                Self::ApacheSkyWalking => serializer.serialize_unit_variant("BindingType", 1u32, "ApacheSkyWalking"),
                Self::AppDynamics => serializer.serialize_unit_variant("BindingType", 2u32, "AppDynamics"),
                Self::Dynatrace => serializer.serialize_unit_variant("BindingType", 3u32, "Dynatrace"),
                Self::NewRelic => serializer.serialize_unit_variant("BindingType", 4u32, "NewRelic"),
                Self::ElasticApm => serializer.serialize_unit_variant("BindingType", 5u32, "ElasticAPM"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "State of the Buildpack Binding."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Succeeded,
        Failed,
        Deleting,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Buildpack Binding Resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildpackBindingResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a buildpack binding"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BuildpackBindingProperties>,
}
impl BuildpackBindingResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of BuildpackBinding resources and a possible link for next set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildpackBindingResourceCollection {
    #[doc = "Collection of BuildpackBinding resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BuildpackBindingResource>,
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for BuildpackBindingResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl BuildpackBindingResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Buildpack properties payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildpackProperties {
    #[doc = "Id of the buildpack"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl BuildpackProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Buildpack group properties of the Builder"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BuildpacksGroupProperties {
    #[doc = "Buildpack group name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Buildpacks in the buildpack group"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub buildpacks: Vec<BuildpackProperties>,
}
impl BuildpacksGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Certificate resource payload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateProperties {
    #[doc = "The type of the certificate source."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The thumbprint of certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "The issuer of certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[doc = "The issue date of certificate."]
    #[serde(rename = "issuedDate", default, skip_serializing_if = "Option::is_none")]
    pub issued_date: Option<String>,
    #[doc = "The expiration date of certificate."]
    #[serde(rename = "expirationDate", default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[doc = "The activate date of certificate."]
    #[serde(rename = "activateDate", default, skip_serializing_if = "Option::is_none")]
    pub activate_date: Option<String>,
    #[doc = "The subject name of certificate."]
    #[serde(rename = "subjectName", default, skip_serializing_if = "Option::is_none")]
    pub subject_name: Option<String>,
    #[doc = "The domain list of certificate."]
    #[serde(rename = "dnsNames", default, skip_serializing_if = "Vec::is_empty")]
    pub dns_names: Vec<String>,
}
impl CertificateProperties {
    pub fn new(type_: String) -> Self {
        Self {
            type_,
            thumbprint: None,
            issuer: None,
            issued_date: None,
            expiration_date: None,
            activate_date: None,
            subject_name: None,
            dns_names: Vec::new(),
        }
    }
}
#[doc = "Certificate resource payload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Certificate resource payload."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CertificateProperties>,
}
impl CertificateResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection compose of certificate resources list and a possible link for next page."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificateResourceCollection {
    #[doc = "The certificate resources list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CertificateResource>,
    #[doc = "The link to next page of certificate list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CertificateResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CertificateResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "An error response from the service."]
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
#[doc = "An error response from the service."]
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
#[doc = "Service properties payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterResourceProperties {
    #[doc = "Provisioning state of the Service"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<cluster_resource_properties::ProvisioningState>,
    #[doc = "Service network profile payload"]
    #[serde(rename = "networkProfile", default, skip_serializing_if = "Option::is_none")]
    pub network_profile: Option<NetworkProfile>,
    #[doc = "Version of the Service"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[doc = "ServiceInstanceEntity GUID which uniquely identifies a created resource"]
    #[serde(rename = "serviceId", default, skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[doc = "Power state of the Service"]
    #[serde(rename = "powerState", default, skip_serializing_if = "Option::is_none")]
    pub power_state: Option<cluster_resource_properties::PowerState>,
    #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub zone_redundant: Option<bool>,
    #[doc = "Fully qualified dns name of the service instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
}
impl ClusterResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cluster_resource_properties {
    use super::*;
    #[doc = "Provisioning state of the Service"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Starting,
        Stopping,
        Deleting,
        Deleted,
        Succeeded,
        Failed,
        Moving,
        Moved,
        MoveFailed,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Starting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Starting"),
                Self::Stopping => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Stopping"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleted"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Failed"),
                Self::Moving => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Moving"),
                Self::Moved => serializer.serialize_unit_variant("ProvisioningState", 9u32, "Moved"),
                Self::MoveFailed => serializer.serialize_unit_variant("ProvisioningState", 10u32, "MoveFailed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Power state of the Service"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PowerState")]
    pub enum PowerState {
        Running,
        Stopped,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PowerState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PowerState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PowerState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Running => serializer.serialize_unit_variant("PowerState", 0u32, "Running"),
                Self::Stopped => serializer.serialize_unit_variant("PowerState", 1u32, "Stopped"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Property of git."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigServerGitProperty {
    #[doc = "Repositories of git."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<GitPatternRepository>,
    #[doc = "URI of the repository"]
    pub uri: String,
    #[doc = "Label of the repository"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "Searching path of the repository"]
    #[serde(rename = "searchPaths", default, skip_serializing_if = "Vec::is_empty")]
    pub search_paths: Vec<String>,
    #[doc = "Username of git repository basic auth."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "Password of git repository basic auth."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "Public sshKey of git repository."]
    #[serde(rename = "hostKey", default, skip_serializing_if = "Option::is_none")]
    pub host_key: Option<String>,
    #[doc = "SshKey algorithm of git repository."]
    #[serde(rename = "hostKeyAlgorithm", default, skip_serializing_if = "Option::is_none")]
    pub host_key_algorithm: Option<String>,
    #[doc = "Private sshKey algorithm of git repository."]
    #[serde(rename = "privateKey", default, skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[doc = "Strict host key checking or not."]
    #[serde(rename = "strictHostKeyChecking", default, skip_serializing_if = "Option::is_none")]
    pub strict_host_key_checking: Option<bool>,
}
impl ConfigServerGitProperty {
    pub fn new(uri: String) -> Self {
        Self {
            repositories: Vec::new(),
            uri,
            label: None,
            search_paths: Vec::new(),
            username: None,
            password: None,
            host_key: None,
            host_key_algorithm: None,
            private_key: None,
            strict_host_key_checking: None,
        }
    }
}
#[doc = "Config server git properties payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigServerProperties {
    #[doc = "State of the config server."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<config_server_properties::ProvisioningState>,
    #[doc = "The error code compose of code and message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
    #[doc = "The settings of config server."]
    #[serde(rename = "configServer", default, skip_serializing_if = "Option::is_none")]
    pub config_server: Option<ConfigServerSettings>,
}
impl ConfigServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod config_server_properties {
    use super::*;
    #[doc = "State of the config server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        NotAvailable,
        Deleted,
        Failed,
        Succeeded,
        Updating,
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
                Self::NotAvailable => serializer.serialize_unit_variant("ProvisioningState", 0u32, "NotAvailable"),
                Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Deleted"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Config Server resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigServerResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Config server git properties payload"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfigServerProperties>,
}
impl ConfigServerResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The settings of config server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigServerSettings {
    #[doc = "Property of git."]
    #[serde(rename = "gitProperty", default, skip_serializing_if = "Option::is_none")]
    pub git_property: Option<ConfigServerGitProperty>,
}
impl ConfigServerSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error record of the config server settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigServerSettingsErrorRecord {
    #[doc = "The name of the config server settings error record"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The uri of the config server settings error record"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "The detail error messages of the record"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<String>,
}
impl ConfigServerSettingsErrorRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Validation result for config server settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigServerSettingsValidateResult {
    #[doc = "Indicate if the config server settings are valid"]
    #[serde(rename = "isValid", default, skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
    #[doc = "The detail validation results"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ConfigServerSettingsErrorRecord>,
}
impl ConfigServerSettingsValidateResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Property of git environment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationServiceGitProperty {
    #[doc = "Repositories of Application Configuration Service git property."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repositories: Option<ConfigurationServiceGitPropertyRepository>,
}
impl ConfigurationServiceGitProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ConfigurationServiceGitPropertyRepository = Vec<ConfigurationServiceGitRepository>;
#[doc = "Validation result for configuration service settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationServiceGitPropertyValidateResult {
    #[doc = "Indicate if the configuration service settings are valid"]
    #[serde(rename = "isValid", default, skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
    #[doc = "The detail validation results"]
    #[serde(rename = "gitReposValidationResult", default, skip_serializing_if = "Vec::is_empty")]
    pub git_repos_validation_result: Vec<ValidationMessages>,
}
impl ConfigurationServiceGitPropertyValidateResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Git repository property payload for Application Configuration Service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationServiceGitRepository {
    #[doc = "Name of the repository"]
    pub name: String,
    #[doc = "Collection of patterns of the repository"]
    pub patterns: Vec<String>,
    #[doc = "URI of the repository"]
    pub uri: String,
    #[doc = "Label of the repository"]
    pub label: String,
    #[doc = "Searching path of the repository"]
    #[serde(rename = "searchPaths", default, skip_serializing_if = "Vec::is_empty")]
    pub search_paths: Vec<String>,
    #[doc = "Username of git repository basic auth."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "Password of git repository basic auth."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "Public sshKey of git repository."]
    #[serde(rename = "hostKey", default, skip_serializing_if = "Option::is_none")]
    pub host_key: Option<String>,
    #[doc = "SshKey algorithm of git repository."]
    #[serde(rename = "hostKeyAlgorithm", default, skip_serializing_if = "Option::is_none")]
    pub host_key_algorithm: Option<String>,
    #[doc = "Private sshKey algorithm of git repository."]
    #[serde(rename = "privateKey", default, skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[doc = "Strict host key checking or not."]
    #[serde(rename = "strictHostKeyChecking", default, skip_serializing_if = "Option::is_none")]
    pub strict_host_key_checking: Option<bool>,
}
impl ConfigurationServiceGitRepository {
    pub fn new(name: String, patterns: Vec<String>, uri: String, label: String) -> Self {
        Self {
            name,
            patterns,
            uri,
            label,
            search_paths: Vec::new(),
            username: None,
            password: None,
            host_key: None,
            host_key_algorithm: None,
            private_key: None,
            strict_host_key_checking: None,
        }
    }
}
#[doc = "Collection of instances belong to the Application Configuration Service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationServiceInstance {
    #[doc = "Name of the Application Configuration Service instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Status of the Application Configuration Service instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl ConfigurationServiceInstance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Application Configuration Service properties payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationServiceProperties {
    #[doc = "State of the Application Configuration Service."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<configuration_service_properties::ProvisioningState>,
    #[doc = "Resource request payload of Application Configuration Service"]
    #[serde(rename = "resourceRequests", default, skip_serializing_if = "Option::is_none")]
    pub resource_requests: Option<ConfigurationServiceResourceRequests>,
    #[doc = "Collection of instances belong to Application Configuration Service."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instances: Vec<ConfigurationServiceInstance>,
    #[doc = "The settings of Application Configuration Service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<ConfigurationServiceSettings>,
}
impl ConfigurationServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod configuration_service_properties {
    use super::*;
    #[doc = "State of the Application Configuration Service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Succeeded,
        Failed,
        Deleting,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Application Configuration Service resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationServiceResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Application Configuration Service properties payload"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfigurationServiceProperties>,
}
impl ConfigurationServiceResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of configuration service resources and a possible link for next set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationServiceResourceCollection {
    #[doc = "Collection of configuration service resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConfigurationServiceResource>,
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ConfigurationServiceResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ConfigurationServiceResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource request payload of Application Configuration Service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationServiceResourceRequests {
    #[doc = "Cpu allocated to each Application Configuration Service instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[doc = "Memory allocated to each Application Configuration Service instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    #[doc = "Instance count of the Application Configuration Service"]
    #[serde(rename = "instanceCount", default, skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i32>,
}
impl ConfigurationServiceResourceRequests {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The settings of Application Configuration Service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationServiceSettings {
    #[doc = "Property of git environment."]
    #[serde(rename = "gitProperty", default, skip_serializing_if = "Option::is_none")]
    pub git_property: Option<ConfigurationServiceGitProperty>,
}
impl ConfigurationServiceSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Validation result for configuration service settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationServiceSettingsValidateResult {
    #[doc = "Validation result for configuration service settings"]
    #[serde(rename = "gitPropertyValidationResult", default, skip_serializing_if = "Option::is_none")]
    pub git_property_validation_result: Option<ConfigurationServiceGitPropertyValidateResult>,
}
impl ConfigurationServiceSettingsValidateResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Container liveness and readiness probe settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerProbeSettings {
    #[doc = "Indicates whether disable the liveness and readiness probe"]
    #[serde(rename = "disableProbe", default, skip_serializing_if = "Option::is_none")]
    pub disable_probe: Option<bool>,
}
impl ContainerProbeSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of certificate imported from key vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentCertificateProperties {
    #[serde(flatten)]
    pub certificate_properties: CertificateProperties,
    #[doc = "The content of uploaded certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
impl ContentCertificateProperties {
    pub fn new(certificate_properties: CertificateProperties) -> Self {
        Self {
            certificate_properties,
            content: None,
        }
    }
}
#[doc = "Custom container payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomContainer {
    #[doc = "The name of the registry that contains the container image"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[doc = "Container image of the custom container. This should be in the form of <repository>:<tag> without the server name of the registry"]
    #[serde(rename = "containerImage", default, skip_serializing_if = "Option::is_none")]
    pub container_image: Option<String>,
    #[doc = "Entrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
    #[doc = "Arguments to the entrypoint. The docker image's CMD is used if this is not provided."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    #[doc = "Credential of the image registry"]
    #[serde(rename = "imageRegistryCredential", default, skip_serializing_if = "Option::is_none")]
    pub image_registry_credential: Option<ImageRegistryCredential>,
    #[doc = "Language framework of the container image uploaded"]
    #[serde(rename = "languageFramework", default, skip_serializing_if = "Option::is_none")]
    pub language_framework: Option<String>,
}
impl CustomContainer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Custom container user source info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomContainerUserSourceInfo {
    #[serde(flatten)]
    pub user_source_info: UserSourceInfo,
    #[doc = "Custom container payload"]
    #[serde(rename = "customContainer", default, skip_serializing_if = "Option::is_none")]
    pub custom_container: Option<CustomContainer>,
}
impl CustomContainerUserSourceInfo {
    pub fn new(user_source_info: UserSourceInfo) -> Self {
        Self {
            user_source_info,
            custom_container: None,
        }
    }
}
#[doc = "Custom domain of app resource payload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomDomainProperties {
    #[doc = "The thumbprint of bound certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[doc = "The app name of domain."]
    #[serde(rename = "appName", default, skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[doc = "The bound certificate name of domain."]
    #[serde(rename = "certName", default, skip_serializing_if = "Option::is_none")]
    pub cert_name: Option<String>,
}
impl CustomDomainProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Custom domain resource payload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomDomainResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Custom domain of app resource payload."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomDomainProperties>,
}
impl CustomDomainResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection compose of a custom domain resources list and a possible link for next page."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomDomainResourceCollection {
    #[doc = "The custom domain resources list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CustomDomainResource>,
    #[doc = "The link to next page of custom domain list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CustomDomainResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl CustomDomainResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Custom domain validate payload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomainValidatePayload {
    #[doc = "Name to be validated"]
    pub name: String,
}
impl CustomDomainValidatePayload {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[doc = "Validation result for custom domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomDomainValidateResult {
    #[doc = "Indicates if domain name is valid."]
    #[serde(rename = "isValid", default, skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
    #[doc = "Message of why domain name is invalid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CustomDomainValidateResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type CustomPersistentDiskCollection = Vec<CustomPersistentDiskResource>;
#[doc = "Custom persistent disk resource payload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomPersistentDiskProperties {
    #[doc = "The type of the underlying resource to mount as a persistent disk."]
    #[serde(rename = "type")]
    pub type_: custom_persistent_disk_properties::Type,
    #[doc = "The mount path of the persistent disk."]
    #[serde(rename = "mountPath")]
    pub mount_path: String,
    #[doc = "Indicates whether the persistent disk is a readOnly one."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "These are the mount options for a persistent disk."]
    #[serde(rename = "mountOptions", default, skip_serializing_if = "Vec::is_empty")]
    pub mount_options: Vec<String>,
}
impl CustomPersistentDiskProperties {
    pub fn new(type_: custom_persistent_disk_properties::Type, mount_path: String) -> Self {
        Self {
            type_,
            mount_path,
            read_only: None,
            mount_options: Vec::new(),
        }
    }
}
pub mod custom_persistent_disk_properties {
    use super::*;
    #[doc = "The type of the underlying resource to mount as a persistent disk."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        AzureFileVolume,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Type {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AzureFileVolume => serializer.serialize_unit_variant("Type", 0u32, "AzureFileVolume"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Custom persistent disk resource payload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomPersistentDiskResource {
    #[doc = "Custom persistent disk resource payload."]
    #[serde(rename = "customPersistentDiskProperties", default, skip_serializing_if = "Option::is_none")]
    pub custom_persistent_disk_properties: Option<CustomPersistentDiskProperties>,
    #[doc = "The resource id of Azure Spring Cloud Storage resource."]
    #[serde(rename = "storageId")]
    pub storage_id: String,
}
impl CustomPersistentDiskResource {
    pub fn new(storage_id: String) -> Self {
        Self {
            custom_persistent_disk_properties: None,
            storage_id,
        }
    }
}
#[doc = "Deployment instance payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentInstance {
    #[doc = "Name of the deployment instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Status of the deployment instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Failed reason of the deployment instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "Discovery status of the deployment instance"]
    #[serde(rename = "discoveryStatus", default, skip_serializing_if = "Option::is_none")]
    pub discovery_status: Option<String>,
    #[doc = "Start time of the deployment instance"]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "Availability zone information of the deployment instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}
impl DeploymentInstance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment resource payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Deployment resource properties payload"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeploymentResourceProperties>,
    #[doc = "Sku of Azure Spring Cloud"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl DeploymentResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of App resources and a possible link for next set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentResourceCollection {
    #[doc = "Collection of Deployment resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DeploymentResource>,
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeploymentResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeploymentResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment resource properties payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentResourceProperties {
    #[doc = "Source information for a deployment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<UserSourceInfo>,
    #[doc = "Deployment settings payload"]
    #[serde(rename = "deploymentSettings", default, skip_serializing_if = "Option::is_none")]
    pub deployment_settings: Option<DeploymentSettings>,
    #[doc = "Provisioning state of the Deployment"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<deployment_resource_properties::ProvisioningState>,
    #[doc = "Status of the Deployment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<deployment_resource_properties::Status>,
    #[doc = "Indicates whether the Deployment is active"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[doc = "Collection of instances belong to the Deployment"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instances: Vec<DeploymentInstance>,
}
impl DeploymentResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod deployment_resource_properties {
    use super::*;
    #[doc = "Provisioning state of the Deployment"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Succeeded,
        Failed,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Status of the Deployment"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Stopped,
        Running,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Status {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Status {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Status {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Stopped => serializer.serialize_unit_variant("Status", 0u32, "Stopped"),
                Self::Running => serializer.serialize_unit_variant("Status", 1u32, "Running"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Deployment settings payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentSettings {
    #[doc = "Deployment resource request payload"]
    #[serde(rename = "resourceRequests", default, skip_serializing_if = "Option::is_none")]
    pub resource_requests: Option<ResourceRequests>,
    #[doc = "Collection of environment variables"]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<serde_json::Value>,
    #[doc = "Collection of addons"]
    #[serde(rename = "addonConfigs", default, skip_serializing_if = "Option::is_none")]
    pub addon_configs: Option<serde_json::Value>,
    #[doc = "Container liveness and readiness probe settings"]
    #[serde(rename = "containerProbeSettings", default, skip_serializing_if = "Option::is_none")]
    pub container_probe_settings: Option<ContainerProbeSettings>,
}
impl DeploymentSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Diagnostic parameters of diagnostic operations"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticParameters {
    #[doc = "App instance name"]
    #[serde(rename = "appInstance", default, skip_serializing_if = "Option::is_none")]
    pub app_instance: Option<String>,
    #[doc = "Your target file path in your own BYOS"]
    #[serde(rename = "filePath", default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[doc = "Duration of your JFR. 1 min can be represented by 1m or 60s."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
}
impl DiagnosticParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error code compose of code and message."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "The code of error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The message of error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API metadata property for Spring Cloud Gateway"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayApiMetadataProperties {
    #[doc = "Title describing the context of the APIs available on the Gateway instance (default: `Spring Cloud Gateway for K8S`)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Detailed description of the APIs available on the Gateway instance (default: `Generated OpenAPI 3 document that describes the API routes configured.`)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Location of additional documentation for the APIs available on the Gateway instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    #[doc = "Version of APIs available on this Gateway instance (default: `unspecified`)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Base URL that API consumers will use to access APIs on the Gateway instance."]
    #[serde(rename = "serverUrl", default, skip_serializing_if = "Option::is_none")]
    pub server_url: Option<String>,
}
impl GatewayApiMetadataProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API route config of the Spring Cloud Gateway"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayApiRoute {
    #[doc = "A title, will be applied to methods in the generated OpenAPI documentation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "A description, will be applied to methods in the generated OpenAPI documentation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Full uri, will override `appName`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "Enable sso validation."]
    #[serde(rename = "ssoEnabled", default, skip_serializing_if = "Option::is_none")]
    pub sso_enabled: Option<bool>,
    #[doc = "Pass currently-authenticated user's identity token to application service, default is 'false'"]
    #[serde(rename = "tokenRelay", default, skip_serializing_if = "Option::is_none")]
    pub token_relay: Option<bool>,
    #[doc = "A number of conditions to evaluate a route for each request. Each predicate may be evaluated against request headers and parameter values. All of the predicates associated with a route must evaluate to true for the route to be matched to the request."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub predicates: Vec<String>,
    #[doc = "To modify the request before sending it to the target endpoint, or the received response."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<String>,
    #[doc = "Route processing order."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[doc = "Classification tags, will be applied to methods in the generated OpenAPI documentation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}
impl GatewayApiRoute {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cross-Origin Resource Sharing property"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayCorsProperties {
    #[doc = "Allowed origins to make cross-site requests. The special value `*` allows all domains."]
    #[serde(rename = "allowedOrigins", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_origins: Vec<String>,
    #[doc = "Allowed HTTP methods on cross-site requests. The special value `*` allows all methods. If not set, `GET` and `HEAD` are allowed by default."]
    #[serde(rename = "allowedMethods", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_methods: Vec<String>,
    #[doc = "Allowed headers in cross-site requests. The special value `*` allows actual requests to send any header."]
    #[serde(rename = "allowedHeaders", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_headers: Vec<String>,
    #[doc = "How long, in seconds, the response from a pre-flight request can be cached by clients."]
    #[serde(rename = "maxAge", default, skip_serializing_if = "Option::is_none")]
    pub max_age: Option<i32>,
    #[doc = "Whether user credentials are supported on cross-site requests. Valid values: `true`, `false`."]
    #[serde(rename = "allowCredentials", default, skip_serializing_if = "Option::is_none")]
    pub allow_credentials: Option<bool>,
    #[doc = "HTTP response headers to expose for cross-site requests."]
    #[serde(rename = "exposedHeaders", default, skip_serializing_if = "Vec::is_empty")]
    pub exposed_headers: Vec<String>,
}
impl GatewayCorsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of custom domain for Spring Cloud Gateway"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayCustomDomainProperties {
    #[doc = "The thumbprint of bound certificate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
}
impl GatewayCustomDomainProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Custom domain of the Spring Cloud Gateway"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayCustomDomainResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of custom domain for Spring Cloud Gateway"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GatewayCustomDomainProperties>,
}
impl GatewayCustomDomainResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of Spring Cloud Gateway custom domain resources and a possible link for next set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayCustomDomainResourceCollection {
    #[doc = "Collection of Spring Cloud Gateway custom domain resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GatewayCustomDomainResource>,
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GatewayCustomDomainResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl GatewayCustomDomainResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of instances belong to the Spring Cloud Gateway"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayInstance {
    #[doc = "Name of the Spring Cloud Gateway instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Status of the Spring Cloud Gateway instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl GatewayInstance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the Spring Cloud Gateway Operator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayOperatorProperties {
    #[doc = "Properties of the Spring Cloud Gateway Operator."]
    #[serde(rename = "resourceRequests", default, skip_serializing_if = "Option::is_none")]
    pub resource_requests: Option<GatewayOperatorResourceRequests>,
    #[doc = "Collection of instances belong to Spring Cloud Gateway operator."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instances: Vec<GatewayInstance>,
}
impl GatewayOperatorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the Spring Cloud Gateway Operator."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayOperatorResourceRequests {
    #[doc = "Cpu allocated to each Spring Cloud Gateway Operator instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[doc = "Memory allocated to each Spring Cloud Gateway Operator instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    #[doc = "Instance count of the Spring Cloud Gateway Operator."]
    #[serde(rename = "instanceCount", default, skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i32>,
}
impl GatewayOperatorResourceRequests {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Spring Cloud Gateway properties payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayProperties {
    #[doc = "State of the Spring Cloud Gateway."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<gateway_properties::ProvisioningState>,
    #[doc = "Indicates whether the Spring Cloud Gateway exposes endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[doc = "URL of the Spring Cloud Gateway, exposed when 'public' is true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Indicate if only https is allowed."]
    #[serde(rename = "httpsOnly", default, skip_serializing_if = "Option::is_none")]
    pub https_only: Option<bool>,
    #[doc = "Single sign-on related configuration"]
    #[serde(rename = "ssoProperties", default, skip_serializing_if = "Option::is_none")]
    pub sso_properties: Option<SsoProperties>,
    #[doc = "API metadata property for Spring Cloud Gateway"]
    #[serde(rename = "apiMetadataProperties", default, skip_serializing_if = "Option::is_none")]
    pub api_metadata_properties: Option<GatewayApiMetadataProperties>,
    #[doc = "Cross-Origin Resource Sharing property"]
    #[serde(rename = "corsProperties", default, skip_serializing_if = "Option::is_none")]
    pub cors_properties: Option<GatewayCorsProperties>,
    #[doc = "Resource request payload of Spring Cloud Gateway."]
    #[serde(rename = "resourceRequests", default, skip_serializing_if = "Option::is_none")]
    pub resource_requests: Option<GatewayResourceRequests>,
    #[doc = "Collection of instances belong to Spring Cloud Gateway."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instances: Vec<GatewayInstance>,
    #[doc = "Properties of the Spring Cloud Gateway Operator."]
    #[serde(rename = "operatorProperties", default, skip_serializing_if = "Option::is_none")]
    pub operator_properties: Option<GatewayOperatorProperties>,
}
impl GatewayProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod gateway_properties {
    use super::*;
    #[doc = "State of the Spring Cloud Gateway."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Succeeded,
        Failed,
        Deleting,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Spring Cloud Gateway resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Spring Cloud Gateway properties payload"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GatewayProperties>,
    #[doc = "Sku of Azure Spring Cloud"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl GatewayResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of gateway resources and a possible link for next set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayResourceCollection {
    #[doc = "Collection of gateway resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GatewayResource>,
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GatewayResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl GatewayResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource request payload of Spring Cloud Gateway."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayResourceRequests {
    #[doc = "Cpu allocated to each Spring Cloud Gateway instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[doc = "Memory allocated to each Spring Cloud Gateway instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
}
impl GatewayResourceRequests {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "API route config of the Spring Cloud Gateway"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayRouteConfigProperties {
    #[doc = "State of the Spring Cloud Gateway route config."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<gateway_route_config_properties::ProvisioningState>,
    #[doc = "The resource Id of the Azure Spring Cloud app, required unless route defines `uri`."]
    #[serde(rename = "appResourceId", default, skip_serializing_if = "Option::is_none")]
    pub app_resource_id: Option<String>,
    #[doc = "Array of API routes, each route contains properties such as `title`, `uri`, `ssoEnabled`, `predicates`, `filters`."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub routes: Vec<GatewayApiRoute>,
}
impl GatewayRouteConfigProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod gateway_route_config_properties {
    use super::*;
    #[doc = "State of the Spring Cloud Gateway route config."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Succeeded,
        Failed,
        Deleting,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Spring Cloud Gateway route config resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayRouteConfigResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "API route config of the Spring Cloud Gateway"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GatewayRouteConfigProperties>,
}
impl GatewayRouteConfigResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of Spring Cloud Gateway route config resources and a possible link for next set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GatewayRouteConfigResourceCollection {
    #[doc = "Collection of Spring Cloud Gateway route config resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GatewayRouteConfigResource>,
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GatewayRouteConfigResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl GatewayRouteConfigResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Git repository property payload for config server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitPatternRepository {
    #[doc = "Name of the repository"]
    pub name: String,
    #[doc = "Collection of pattern of the repository"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pattern: Vec<String>,
    #[doc = "URI of the repository"]
    pub uri: String,
    #[doc = "Label of the repository"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[doc = "Searching path of the repository"]
    #[serde(rename = "searchPaths", default, skip_serializing_if = "Vec::is_empty")]
    pub search_paths: Vec<String>,
    #[doc = "Username of git repository basic auth."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "Password of git repository basic auth."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "Public sshKey of git repository."]
    #[serde(rename = "hostKey", default, skip_serializing_if = "Option::is_none")]
    pub host_key: Option<String>,
    #[doc = "SshKey algorithm of git repository."]
    #[serde(rename = "hostKeyAlgorithm", default, skip_serializing_if = "Option::is_none")]
    pub host_key_algorithm: Option<String>,
    #[doc = "Private sshKey algorithm of git repository."]
    #[serde(rename = "privateKey", default, skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[doc = "Strict host key checking or not."]
    #[serde(rename = "strictHostKeyChecking", default, skip_serializing_if = "Option::is_none")]
    pub strict_host_key_checking: Option<bool>,
}
impl GitPatternRepository {
    pub fn new(name: String, uri: String) -> Self {
        Self {
            name,
            pattern: Vec::new(),
            uri,
            label: None,
            search_paths: Vec::new(),
            username: None,
            password: None,
            host_key: None,
            host_key_algorithm: None,
            private_key: None,
            strict_host_key_checking: None,
        }
    }
}
#[doc = "Credential of the image registry"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageRegistryCredential {
    #[doc = "The username of the image registry credential"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "The password of the image registry credential"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl ImageRegistryCredential {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Uploaded Jar binary for a deployment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JarUploadedUserSourceInfo {
    #[serde(flatten)]
    pub uploaded_user_source_info: UploadedUserSourceInfo,
    #[doc = "Runtime version of the Jar file"]
    #[serde(rename = "runtimeVersion", default, skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
    #[doc = "JVM parameter"]
    #[serde(rename = "jvmOptions", default, skip_serializing_if = "Option::is_none")]
    pub jvm_options: Option<String>,
}
impl JarUploadedUserSourceInfo {
    pub fn new(uploaded_user_source_info: UploadedUserSourceInfo) -> Self {
        Self {
            uploaded_user_source_info,
            runtime_version: None,
            jvm_options: None,
        }
    }
}
#[doc = "Properties of certificate imported from key vault."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultCertificateProperties {
    #[serde(flatten)]
    pub certificate_properties: CertificateProperties,
    #[doc = "The vault uri of user key vault."]
    #[serde(rename = "vaultUri")]
    pub vault_uri: String,
    #[doc = "The certificate name of key vault."]
    #[serde(rename = "keyVaultCertName")]
    pub key_vault_cert_name: String,
    #[doc = "The certificate version of key vault."]
    #[serde(rename = "certVersion", default, skip_serializing_if = "Option::is_none")]
    pub cert_version: Option<String>,
    #[doc = "Optional. If set to true, it will not import private key from key vault."]
    #[serde(rename = "excludePrivateKey", default, skip_serializing_if = "Option::is_none")]
    pub exclude_private_key: Option<bool>,
}
impl KeyVaultCertificateProperties {
    pub fn new(certificate_properties: CertificateProperties, vault_uri: String, key_vault_cert_name: String) -> Self {
        Self {
            certificate_properties,
            vault_uri,
            key_vault_cert_name,
            cert_version: None,
            exclude_private_key: None,
        }
    }
}
#[doc = "Loaded certificate payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadedCertificate {
    #[doc = "Resource Id of loaded certificate"]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "Indicate whether the certificate will be loaded into default trust store, only work for Java runtime."]
    #[serde(rename = "loadTrustStore", default, skip_serializing_if = "Option::is_none")]
    pub load_trust_store: Option<bool>,
}
impl LoadedCertificate {
    pub fn new(resource_id: String) -> Self {
        Self {
            resource_id,
            load_trust_store: None,
        }
    }
}
pub type LoadedCertificateCollection = Vec<LoadedCertificate>;
#[doc = "Log file URL payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogFileUrlResponse {
    #[doc = "URL of the log file"]
    pub url: String,
}
impl LogFileUrlResponse {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}
#[doc = "Specifications of the Log for Azure Monitoring"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogSpecification {
    #[doc = "Name of the log"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized friendly display name of the log"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Blob duration of the log"]
    #[serde(rename = "blobDuration", default, skip_serializing_if = "Option::is_none")]
    pub blob_duration: Option<String>,
}
impl LogSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Managed identity properties retrieved from ARM request headers."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIdentityProperties {
    #[doc = "Type of the managed identity"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<managed_identity_properties::Type>,
    #[doc = "Principal Id of system-assigned managed identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Tenant Id of system-assigned managed identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "User-assigned managed identities in key-value map. The key references will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}'."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedManagedIdentities>,
}
impl ManagedIdentityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_identity_properties {
    use super::*;
    #[doc = "Type of the managed identity"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        None,
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned,UserAssigned")]
        SystemAssignedUserAssigned,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Type {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("Type", 0u32, "None"),
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 1u32, "SystemAssigned"),
                Self::UserAssigned => serializer.serialize_unit_variant("Type", 2u32, "UserAssigned"),
                Self::SystemAssignedUserAssigned => serializer.serialize_unit_variant("Type", 3u32, "SystemAssigned,UserAssigned"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifications of the Dimension of metrics"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricDimension {
    #[doc = "Name of the dimension"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized friendly display name of the dimension"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Whether this dimension should be included for the Shoebox export scenario"]
    #[serde(rename = "toBeExportedForShoebox", default, skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
}
impl MetricDimension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifications of the Metrics for Azure Monitoring"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricSpecification {
    #[doc = "Name of the metric"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Localized friendly display name of the metric"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Localized friendly description of the metric"]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "Unit that makes sense for the metric"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Name of the metric category that the metric belongs to. A metric can only belong to a single category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "Only provide one value for this field. Valid values: Average, Minimum, Maximum, Total, Count."]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[doc = "Supported aggregation types"]
    #[serde(rename = "supportedAggregationTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_aggregation_types: Vec<String>,
    #[doc = "Supported time grain types"]
    #[serde(rename = "supportedTimeGrainTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_time_grain_types: Vec<String>,
    #[doc = "Optional. If set to true, then zero will be returned for time duration where no metric is emitted/published."]
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<bool>,
    #[doc = "Dimensions of the metric"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<MetricDimension>,
    #[doc = "Name of the MDM namespace. Optional."]
    #[serde(rename = "sourceMdmNamespace", default, skip_serializing_if = "Option::is_none")]
    pub source_mdm_namespace: Option<String>,
}
impl MetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Monitoring Setting properties payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoringSettingProperties {
    #[doc = "State of the Monitoring Setting."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<monitoring_setting_properties::ProvisioningState>,
    #[doc = "The error code compose of code and message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
    #[doc = "Indicates whether enable the trace functionality, which will be deprecated since api version 2020-11-01-preview. Please leverage appInsightsInstrumentationKey to indicate if monitoringSettings enabled or not"]
    #[serde(rename = "traceEnabled", default, skip_serializing_if = "Option::is_none")]
    pub trace_enabled: Option<bool>,
    #[doc = "Target application insight instrumentation key, null or whitespace include empty will disable monitoringSettings"]
    #[serde(rename = "appInsightsInstrumentationKey", default, skip_serializing_if = "Option::is_none")]
    pub app_insights_instrumentation_key: Option<String>,
    #[doc = "Indicates the sampling rate of application insight agent, should be in range [0.0, 100.0]"]
    #[serde(rename = "appInsightsSamplingRate", default, skip_serializing_if = "Option::is_none")]
    pub app_insights_sampling_rate: Option<f64>,
    #[doc = "Application Insights agent versions properties payload"]
    #[serde(rename = "appInsightsAgentVersions", default, skip_serializing_if = "Option::is_none")]
    pub app_insights_agent_versions: Option<ApplicationInsightsAgentVersions>,
}
impl MonitoringSettingProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod monitoring_setting_properties {
    use super::*;
    #[doc = "State of the Monitoring Setting."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        NotAvailable,
        Failed,
        Succeeded,
        Updating,
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
                Self::NotAvailable => serializer.serialize_unit_variant("ProvisioningState", 0u32, "NotAvailable"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Monitoring Setting resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MonitoringSettingResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Monitoring Setting properties payload"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MonitoringSettingProperties>,
}
impl MonitoringSettingResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Name availability result payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NameAvailability {
    #[doc = "Indicates whether the name is available"]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "Reason why the name is not available"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "Message why the name is not available"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl NameAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Name availability parameters payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NameAvailabilityParameters {
    #[doc = "Type of the resource to check name availability"]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Name to be checked"]
    pub name: String,
}
impl NameAvailabilityParameters {
    pub fn new(type_: String, name: String) -> Self {
        Self { type_, name }
    }
}
#[doc = "Uploaded Jar binary for a deployment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetCoreZipUploadedUserSourceInfo {
    #[serde(flatten)]
    pub uploaded_user_source_info: UploadedUserSourceInfo,
    #[doc = "The path to the .NET executable relative to zip root"]
    #[serde(rename = "netCoreMainEntryPath", default, skip_serializing_if = "Option::is_none")]
    pub net_core_main_entry_path: Option<String>,
    #[doc = "Runtime version of the .Net file"]
    #[serde(rename = "runtimeVersion", default, skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
}
impl NetCoreZipUploadedUserSourceInfo {
    pub fn new(uploaded_user_source_info: UploadedUserSourceInfo) -> Self {
        Self {
            uploaded_user_source_info,
            net_core_main_entry_path: None,
            runtime_version: None,
        }
    }
}
#[doc = "Service network profile payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkProfile {
    #[doc = "Fully qualified resource Id of the subnet to host Azure Spring Cloud Service Runtime"]
    #[serde(rename = "serviceRuntimeSubnetId", default, skip_serializing_if = "Option::is_none")]
    pub service_runtime_subnet_id: Option<String>,
    #[doc = "Fully qualified resource Id of the subnet to host Azure Spring Cloud Apps"]
    #[serde(rename = "appSubnetId", default, skip_serializing_if = "Option::is_none")]
    pub app_subnet_id: Option<String>,
    #[doc = "Azure Spring Cloud service reserved CIDR"]
    #[serde(rename = "serviceCidr", default, skip_serializing_if = "Option::is_none")]
    pub service_cidr: Option<String>,
    #[doc = "Name of the resource group containing network resources of Azure Spring Cloud Service Runtime"]
    #[serde(rename = "serviceRuntimeNetworkResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub service_runtime_network_resource_group: Option<String>,
    #[doc = "Name of the resource group containing network resources of Azure Spring Cloud Apps"]
    #[serde(rename = "appNetworkResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub app_network_resource_group: Option<String>,
    #[doc = "Desired outbound IP resources for Azure Spring Cloud instance."]
    #[serde(rename = "outboundIPs", default, skip_serializing_if = "Option::is_none")]
    pub outbound_i_ps: Option<network_profile::OutboundIPs>,
    #[doc = "Required inbound or outbound traffics for Azure Spring Cloud instance."]
    #[serde(rename = "requiredTraffics", default, skip_serializing_if = "Vec::is_empty")]
    pub required_traffics: Vec<RequiredTraffic>,
}
impl NetworkProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_profile {
    use super::*;
    #[doc = "Desired outbound IP resources for Azure Spring Cloud instance."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct OutboundIPs {
        #[doc = "A list of public IP addresses."]
        #[serde(rename = "publicIPs", default, skip_serializing_if = "Vec::is_empty")]
        pub public_i_ps: Vec<String>,
    }
    impl OutboundIPs {
        pub fn new() -> Self {
            Self::default()
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
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<operation_detail::ActionType>,
    #[doc = "Origin of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Extra Operation properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
}
impl OperationDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation_detail {
    use super::*;
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
#[doc = "Extra Operation properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {
    #[doc = "Service specification payload"]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<ServiceSpecification>,
}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Persistent disk payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PersistentDisk {
    #[doc = "Size of the persistent disk in GB"]
    #[serde(rename = "sizeInGB", default, skip_serializing_if = "Option::is_none")]
    pub size_in_gb: Option<i32>,
    #[doc = "Size of the used persistent disk in GB"]
    #[serde(rename = "usedInGB", default, skip_serializing_if = "Option::is_none")]
    pub used_in_gb: Option<i32>,
    #[doc = "Mount path of the persistent disk"]
    #[serde(rename = "mountPath", default, skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<String>,
}
impl PersistentDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for a ARM proxy resource. It will have everything other than required location and tags."]
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
#[doc = "Regenerate test key request payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegenerateTestKeyRequestPayload {
    #[doc = "Type of the test key"]
    #[serde(rename = "keyType")]
    pub key_type: regenerate_test_key_request_payload::KeyType,
}
impl RegenerateTestKeyRequestPayload {
    pub fn new(key_type: regenerate_test_key_request_payload::KeyType) -> Self {
        Self { key_type }
    }
}
pub mod regenerate_test_key_request_payload {
    use super::*;
    #[doc = "Type of the test key"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "KeyType")]
    pub enum KeyType {
        Primary,
        Secondary,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for KeyType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for KeyType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for KeyType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Primary => serializer.serialize_unit_variant("KeyType", 0u32, "Primary"),
                Self::Secondary => serializer.serialize_unit_variant("KeyType", 1u32, "Secondary"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Required inbound or outbound traffic for Azure Spring Cloud instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequiredTraffic {
    #[doc = "The protocol of required traffic"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[doc = "The port of required traffic"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "The ip list of required traffic"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ips: Vec<String>,
    #[doc = "The FQDN list of required traffic"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fqdns: Vec<String>,
    #[doc = "The direction of required traffic"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<required_traffic::Direction>,
}
impl RequiredTraffic {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod required_traffic {
    use super::*;
    #[doc = "The direction of required traffic"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Direction")]
    pub enum Direction {
        Inbound,
        Outbound,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Direction {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Direction {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Direction {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Inbound => serializer.serialize_unit_variant("Direction", 0u32, "Inbound"),
                Self::Outbound => serializer.serialize_unit_variant("Direction", 1u32, "Outbound"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The core properties of ARM resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource Id for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
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
#[doc = "Deployment resource request payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceRequests {
    #[doc = "Required CPU. 1 core can be represented by 1 or 1000m. This should be 500m or 1 for Basic tier, and {500m, 1, 2, 3, 4} for Standard tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[doc = "Required memory. 1 GB can be represented by 1Gi or 1024Mi. This should be {512Mi, 1Gi, 2Gi} for Basic tier, and {512Mi, 1Gi, 2Gi, ..., 8Gi} for Standard tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
}
impl ResourceRequests {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an available Azure Spring Cloud SKU."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSku {
    #[doc = "Gets the type of resource the SKU applies to."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Gets the name of SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the tier of SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "The SKU capacity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<SkuCapacity>,
    #[doc = "Gets the set of locations that the SKU is available."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[doc = "Gets a list of locations and availability zones in those locations where the SKU is available."]
    #[serde(rename = "locationInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub location_info: Vec<ResourceSkuLocationInfo>,
    #[doc = "Gets the restrictions because of which SKU cannot be used. This is\r\nempty if there are no restrictions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<ResourceSkuRestrictions>,
}
impl ResourceSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuCapabilities {
    #[doc = "Gets an invariant to describe the feature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets an invariant if the feature is measured by quantity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ResourceSkuCapabilities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of Azure Spring Cloud SKU and a possible link for next set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuCollection {
    #[doc = "Collection of resource SKU"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceSku>,
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ResourceSkuCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ResourceSkuCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Locations and availability zones where the SKU is available"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuLocationInfo {
    #[doc = "Gets location of the SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Gets list of availability zones where the SKU is supported."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
    #[doc = "Gets details of capabilities available to a SKU in specific zones."]
    #[serde(rename = "zoneDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub zone_details: Vec<ResourceSkuZoneDetails>,
}
impl ResourceSkuLocationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about the restriction where the SKU cannot be used"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuRestrictionInfo {
    #[doc = "Gets locations where the SKU is restricted"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[doc = "Gets list of availability zones where the SKU is restricted."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
}
impl ResourceSkuRestrictionInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Restrictions where the SKU cannot be used"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuRestrictions {
    #[doc = "Gets the type of restrictions. Possible values include: 'Location', 'Zone'"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<resource_sku_restrictions::Type>,
    #[doc = "Gets the value of restrictions. If the restriction type is set to\r\nlocation. This would be different locations where the SKU is restricted."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
    #[doc = "Information about the restriction where the SKU cannot be used"]
    #[serde(rename = "restrictionInfo", default, skip_serializing_if = "Option::is_none")]
    pub restriction_info: Option<ResourceSkuRestrictionInfo>,
    #[doc = "Gets the reason for restriction. Possible values include: 'QuotaId', 'NotAvailableForSubscription'"]
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<resource_sku_restrictions::ReasonCode>,
}
impl ResourceSkuRestrictions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_sku_restrictions {
    use super::*;
    #[doc = "Gets the type of restrictions. Possible values include: 'Location', 'Zone'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        Location,
        Zone,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Type {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Location => serializer.serialize_unit_variant("Type", 0u32, "Location"),
                Self::Zone => serializer.serialize_unit_variant("Type", 1u32, "Zone"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets the reason for restriction. Possible values include: 'QuotaId', 'NotAvailableForSubscription'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ReasonCode")]
    pub enum ReasonCode {
        QuotaId,
        NotAvailableForSubscription,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ReasonCode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ReasonCode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ReasonCode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::QuotaId => serializer.serialize_unit_variant("ReasonCode", 0u32, "QuotaId"),
                Self::NotAvailableForSubscription => serializer.serialize_unit_variant("ReasonCode", 1u32, "NotAvailableForSubscription"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Details of capabilities available to a SKU in specific zones"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuZoneDetails {
    #[doc = "Gets the set of zones that the SKU is available in with the\r\nspecified capabilities."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub name: Vec<String>,
    #[doc = "Gets a list of capabilities that are available for the SKU in the\r\nspecified list of zones."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<ResourceSkuCapabilities>,
}
impl ResourceSkuZoneDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource upload definition payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceUploadDefinition {
    #[doc = "Source relative path"]
    #[serde(rename = "relativePath", default, skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
    #[doc = "Upload URL"]
    #[serde(rename = "uploadUrl", default, skip_serializing_if = "Option::is_none")]
    pub upload_url: Option<String>,
}
impl ResourceUploadDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection of instances belong to the Service Registry"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceRegistryInstance {
    #[doc = "Name of the Service Registry instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Status of the Service Registry instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl ServiceRegistryInstance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Service Registry properties payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceRegistryProperties {
    #[doc = "State of the Service Registry."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<service_registry_properties::ProvisioningState>,
    #[doc = "Resource request payload of Service Registry"]
    #[serde(rename = "resourceRequests", default, skip_serializing_if = "Option::is_none")]
    pub resource_requests: Option<ServiceRegistryResourceRequests>,
    #[doc = "Collection of instances belong to Service Registry."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instances: Vec<ServiceRegistryInstance>,
}
impl ServiceRegistryProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod service_registry_properties {
    use super::*;
    #[doc = "State of the Service Registry."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Succeeded,
        Failed,
        Deleting,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Updating"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Deleting"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Service Registry resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceRegistryResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Service Registry properties payload"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServiceRegistryProperties>,
}
impl ServiceRegistryResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of Service Registry resources and a possible link for next set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceRegistryResourceCollection {
    #[doc = "Collection of Service Registry resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServiceRegistryResource>,
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServiceRegistryResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ServiceRegistryResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource request payload of Service Registry"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceRegistryResourceRequests {
    #[doc = "Cpu allocated to each Service Registry instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[doc = "Memory allocated to each Service Registry instance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    #[doc = "Instance count of the Service Registry"]
    #[serde(rename = "instanceCount", default, skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i32>,
}
impl ServiceRegistryResourceRequests {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Service resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Service properties payload"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterResourceProperties>,
    #[doc = "Sku of Azure Spring Cloud"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl ServiceResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of Service resources and a possible link for next set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceResourceList {
    #[doc = "Collection of Service resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServiceResource>,
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ServiceResourceList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ServiceResourceList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Service specification payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceSpecification {
    #[doc = "Specifications of the Log for Azure Monitoring"]
    #[serde(rename = "logSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub log_specifications: Vec<LogSpecification>,
    #[doc = "Specifications of the Metrics for Azure Monitoring"]
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<MetricSpecification>,
}
impl ServiceSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sku of Azure Spring Cloud"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Sku {
    #[doc = "Name of the Sku"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Tier of the Sku"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[doc = "Current capacity of the target resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl Sku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SKU capacity"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuCapacity {
    #[doc = "Gets or sets the minimum."]
    pub minimum: i32,
    #[doc = "Gets or sets the maximum."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,
    #[doc = "Gets or sets the default."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<i32>,
    #[doc = "Gets or sets the type of the scale."]
    #[serde(rename = "scaleType", default, skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<sku_capacity::ScaleType>,
}
impl SkuCapacity {
    pub fn new(minimum: i32) -> Self {
        Self {
            minimum,
            maximum: None,
            default: None,
            scale_type: None,
        }
    }
}
pub mod sku_capacity {
    use super::*;
    #[doc = "Gets or sets the type of the scale."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ScaleType")]
    pub enum ScaleType {
        None,
        Manual,
        Automatic,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ScaleType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ScaleType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ScaleType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("ScaleType", 0u32, "None"),
                Self::Manual => serializer.serialize_unit_variant("ScaleType", 1u32, "Manual"),
                Self::Automatic => serializer.serialize_unit_variant("ScaleType", 2u32, "Automatic"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Uploaded Java source code binary for a deployment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceUploadedUserSourceInfo {
    #[serde(flatten)]
    pub uploaded_user_source_info: UploadedUserSourceInfo,
    #[doc = "Selector for the artifact to be used for the deployment for multi-module projects. This should be\r\nthe relative path to the target module/project."]
    #[serde(rename = "artifactSelector", default, skip_serializing_if = "Option::is_none")]
    pub artifact_selector: Option<String>,
    #[doc = "Runtime version of the source file"]
    #[serde(rename = "runtimeVersion", default, skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
}
impl SourceUploadedUserSourceInfo {
    pub fn new(uploaded_user_source_info: UploadedUserSourceInfo) -> Self {
        Self {
            uploaded_user_source_info,
            artifact_selector: None,
            runtime_version: None,
        }
    }
}
#[doc = "Single sign-on related configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SsoProperties {
    #[doc = "It defines the specific actions applications can be allowed to do on a user's behalf"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scope: Vec<String>,
    #[doc = "The public identifier for the application"]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[doc = "The secret known only to the application and the authorization server"]
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[doc = "The URI of Issuer Identifier"]
    #[serde(rename = "issuerUri", default, skip_serializing_if = "Option::is_none")]
    pub issuer_uri: Option<String>,
}
impl SsoProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "KPack ClusterStack properties payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StackProperties {
    #[doc = "Id of the ClusterStack."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Version of the ClusterStack"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl StackProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "storage resource of type Azure Storage Account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccount {
    #[serde(flatten)]
    pub storage_properties: StorageProperties,
    #[doc = "The account name of the Azure Storage Account."]
    #[serde(rename = "accountName")]
    pub account_name: String,
    #[doc = "The account key of the Azure Storage Account."]
    #[serde(rename = "accountKey")]
    pub account_key: String,
}
impl StorageAccount {
    pub fn new(storage_properties: StorageProperties, account_name: String, account_key: String) -> Self {
        Self {
            storage_properties,
            account_name,
            account_key,
        }
    }
}
#[doc = "Storage resource payload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageProperties {
    #[doc = "The type of the storage."]
    #[serde(rename = "storageType")]
    pub storage_type: storage_properties::StorageType,
}
impl StorageProperties {
    pub fn new(storage_type: storage_properties::StorageType) -> Self {
        Self { storage_type }
    }
}
pub mod storage_properties {
    use super::*;
    #[doc = "The type of the storage."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "StorageType")]
    pub enum StorageType {
        StorageAccount,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for StorageType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for StorageType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for StorageType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::StorageAccount => serializer.serialize_unit_variant("StorageType", 0u32, "StorageAccount"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Storage resource payload."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Storage resource payload."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageProperties>,
}
impl StorageResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Collection compose of storage resources list and a possible link for next page."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageResourceCollection {
    #[doc = "The storage resources list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StorageResource>,
    #[doc = "The link to next page of storage list."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StorageResourceCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl StorageResourceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Supported buildpack resource payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedBuildpackResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Supported buildpack resource properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SupportedBuildpackResourceProperties>,
}
impl SupportedBuildpackResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Supported buildpack resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedBuildpackResourceProperties {
    #[doc = "The id of supported buildpack"]
    #[serde(rename = "buildpackId", default, skip_serializing_if = "Option::is_none")]
    pub buildpack_id: Option<String>,
}
impl SupportedBuildpackResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of supported buildpacks resources and a possible link for next set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedBuildpacksCollection {
    #[doc = "Collection of supported buildpacks resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SupportedBuildpackResource>,
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SupportedBuildpacksCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Supported deployment runtime version descriptor."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedRuntimeVersion {
    #[doc = "The raw value which could be passed to deployment CRUD operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<supported_runtime_version::Value>,
    #[doc = "The platform of this runtime version (possible values: \"Java\" or \".NET\")."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<supported_runtime_version::Platform>,
    #[doc = "The detailed version (major.minor) of the platform."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl SupportedRuntimeVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod supported_runtime_version {
    use super::*;
    #[doc = "The raw value which could be passed to deployment CRUD operations."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Value")]
    pub enum Value {
        #[serde(rename = "Java_8")]
        Java8,
        #[serde(rename = "Java_11")]
        Java11,
        #[serde(rename = "Java_17")]
        Java17,
        #[serde(rename = "NetCore_31")]
        NetCore31,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Value {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Value {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Value {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Java8 => serializer.serialize_unit_variant("Value", 0u32, "Java_8"),
                Self::Java11 => serializer.serialize_unit_variant("Value", 1u32, "Java_11"),
                Self::Java17 => serializer.serialize_unit_variant("Value", 2u32, "Java_17"),
                Self::NetCore31 => serializer.serialize_unit_variant("Value", 3u32, "NetCore_31"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The platform of this runtime version (possible values: \"Java\" or \".NET\")."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Platform")]
    pub enum Platform {
        Java,
        #[serde(rename = ".NET Core")]
        NetCore,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Platform {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Platform {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Platform {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Java => serializer.serialize_unit_variant("Platform", 0u32, "Java"),
                Self::NetCore => serializer.serialize_unit_variant("Platform", 1u32, ".NET Core"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Supported stack resource payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedStackResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Supported stack resource properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SupportedStackResourceProperties>,
}
impl SupportedStackResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Supported stack resource properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedStackResourceProperties {
    #[doc = "The id of supported stack"]
    #[serde(rename = "stackId", default, skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    #[doc = "The version of supported stack"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl SupportedStackResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object that includes an array of supported stacks resources and a possible link for next set"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedStacksCollection {
    #[doc = "Collection of supported stacks resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SupportedStackResource>,
    #[doc = "URL client should use to fetch the next page (per server side paging).\r\nIt's null for now, added for future use."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SupportedStacksCollection {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(rename = "createdAt", with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource modification (UTC)."]
    #[serde(rename = "lastModifiedAt", with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<time::OffsetDateTime>,
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
#[doc = "Temporary disk payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TemporaryDisk {
    #[doc = "Size of the temporary disk in GB"]
    #[serde(rename = "sizeInGB", default, skip_serializing_if = "Option::is_none")]
    pub size_in_gb: Option<i32>,
    #[doc = "Mount path of the temporary disk"]
    #[serde(rename = "mountPath", default, skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<String>,
}
impl TemporaryDisk {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Test keys payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TestKeys {
    #[doc = "Primary key"]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "Secondary key"]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[doc = "Primary test endpoint"]
    #[serde(rename = "primaryTestEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub primary_test_endpoint: Option<String>,
    #[doc = "Secondary test endpoint"]
    #[serde(rename = "secondaryTestEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub secondary_test_endpoint: Option<String>,
    #[doc = "Indicates whether the test endpoint feature enabled or not"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl TestKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for a ARM tracked top level resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The GEO location of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Tags of the service which is a list of key value pairs that describe the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The build result triggered by a build"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TriggeredBuildResult {
    #[doc = "The unique build id of this build result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl TriggeredBuildResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Source with uploaded location"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UploadedUserSourceInfo {
    #[serde(flatten)]
    pub user_source_info: UserSourceInfo,
    #[doc = "Relative path of the storage which stores the source"]
    #[serde(rename = "relativePath", default, skip_serializing_if = "Option::is_none")]
    pub relative_path: Option<String>,
}
impl UploadedUserSourceInfo {
    pub fn new(user_source_info: UserSourceInfo) -> Self {
        Self {
            user_source_info,
            relative_path: None,
        }
    }
}
#[doc = "User-assigned managed identities in key-value map. The key references will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}'."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedManagedIdentities {}
impl UserAssignedManagedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The details of the user-assigned managed identity assigned to an App."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedManagedIdentity {
    #[doc = "Principal Id of user-assigned managed identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Client Id of user-assigned managed identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedManagedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Source information for a deployment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSourceInfo {
    #[doc = "Type of the source uploaded"]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Version of the source"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl UserSourceInfo {
    pub fn new(type_: String) -> Self {
        Self { type_, version: None }
    }
}
#[doc = "Validate messages of the configuration service git repositories"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidationMessages {
    #[doc = "The name of the configuration service git repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Detailed validation messages."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<String>,
}
impl ValidationMessages {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Application Insights agent versions properties payload"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplicationInsightsAgentVersions {
    #[doc = "Indicates the version of application insight java agent"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub java: Option<String>,
}
impl ApplicationInsightsAgentVersions {
    pub fn new() -> Self {
        Self::default()
    }
}
