#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Appliance ArtifactProfile definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArtifactProfile {
    #[doc = "Endpoint is the URL to upload artifacts to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl ArtifactProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Appliance ArtifactType definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ArtifactType")]
pub enum ArtifactType {
    LogsArtifactType,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ArtifactType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ArtifactType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ArtifactType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::LogsArtifactType => serializer.serialize_unit_variant("ArtifactType", 0u32, "LogsArtifactType"),
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
#[doc = "Contains the REP (rendezvous endpoint) and “Listener” access token from notification service (NS)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridConnectionConfig {
    #[doc = "Timestamp when this token will be expired."]
    #[serde(rename = "expirationTime", default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<i64>,
    #[doc = "Name of the connection"]
    #[serde(rename = "hybridConnectionName", default, skip_serializing_if = "Option::is_none")]
    pub hybrid_connection_name: Option<String>,
    #[doc = "Name of the notification service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relay: Option<String>,
    #[doc = "Listener access token"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
impl HybridConnectionConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[doc = "The principal ID of resource identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The identity type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
}
impl Identity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity {
    use super::*;
    #[doc = "The identity type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        SystemAssigned,
        None,
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
                Self::SystemAssigned => serializer.serialize_unit_variant("Type", 0u32, "SystemAssigned"),
                Self::None => serializer.serialize_unit_variant("Type", 1u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
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
#[doc = "Appliance SSHKey definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SshKey {
    #[doc = "Certificate associated with the public key if the key is signed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[doc = "Certificate creation timestamp (Unix)."]
    #[serde(rename = "creationTimeStamp", default, skip_serializing_if = "Option::is_none")]
    pub creation_time_stamp: Option<i64>,
    #[doc = "Certificate expiration timestamp (Unix)."]
    #[serde(rename = "expirationTimeStamp", default, skip_serializing_if = "Option::is_none")]
    pub expiration_time_stamp: Option<i64>,
    #[doc = "Private Key."]
    #[serde(rename = "privateKey", default, skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[doc = "Public Key."]
    #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
}
impl SshKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Appliance SSHKeyType definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SshKeyType")]
pub enum SshKeyType {
    #[serde(rename = "SSHCustomerUser")]
    SshCustomerUser,
    #[serde(rename = "ManagementCAKey")]
    ManagementCaKey,
    LogsKey,
    ScopedAccessKey,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SshKeyType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SshKeyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SshKeyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::SshCustomerUser => serializer.serialize_unit_variant("SshKeyType", 0u32, "SSHCustomerUser"),
            Self::ManagementCaKey => serializer.serialize_unit_variant("SshKeyType", 1u32, "ManagementCAKey"),
            Self::LogsKey => serializer.serialize_unit_variant("SshKeyType", 2u32, "LogsKey"),
            Self::ScopedAccessKey => serializer.serialize_unit_variant("SshKeyType", 3u32, "ScopedAccessKey"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The SupportedVersion object for appliance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedVersion {
    #[doc = "The SupportedVersionMetadata object for appliance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<SupportedVersionMetadata>,
    #[doc = "The newer version available for upgrade."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl SupportedVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SupportedVersionCatalogVersion object for appliance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedVersionCatalogVersion {
    #[doc = "The SupportedVersionCatalogVersionData object for appliance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<SupportedVersionCatalogVersionData>,
    #[doc = "The catalog version name for the version available for upgrade."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The catalog version namespace for the version available for upgrade."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
impl SupportedVersionCatalogVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SupportedVersionCatalogVersionData object for appliance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedVersionCatalogVersionData {
    #[doc = "The image audience name for the version available for upgrade."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[doc = "The image catalog name for the version available for upgrade."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalog: Option<String>,
    #[doc = "The image offer name for the version available for upgrade."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[doc = "The image version for the version available for upgrade."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl SupportedVersionCatalogVersionData {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SupportedVersionMetadata object for appliance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedVersionMetadata {
    #[doc = "The SupportedVersionCatalogVersion object for appliance."]
    #[serde(rename = "catalogVersion", default, skip_serializing_if = "Option::is_none")]
    pub catalog_version: Option<SupportedVersionCatalogVersion>,
}
impl SupportedVersionMetadata {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "The Upgrade Graph for appliance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpgradeGraph {
    #[doc = "The appliance resource path"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The release train name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The Upgrade Graph Properties for appliance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpgradeGraphProperties>,
}
impl UpgradeGraph {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Upgrade Graph Properties for appliance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpgradeGraphProperties {
    #[doc = "The current appliance version"]
    #[serde(rename = "applianceVersion", default, skip_serializing_if = "Option::is_none")]
    pub appliance_version: Option<String>,
    #[doc = "This contains the current version and supported upgrade versions."]
    #[serde(
        rename = "supportedVersions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub supported_versions: Vec<SupportedVersion>,
}
impl UpgradeGraphProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Appliances definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Appliance {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[doc = "Properties for an appliance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplianceProperties>,
}
impl Appliance {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            identity: None,
            properties: None,
        }
    }
}
#[doc = "Cluster User Credential appliance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplianceCredentialKubeconfig {
    #[doc = "Name which contains the role of the kubeconfig."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<appliance_credential_kubeconfig::Name>,
    #[doc = "Contains the kubeconfig value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl ApplianceCredentialKubeconfig {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod appliance_credential_kubeconfig {
    use super::*;
    #[doc = "Name which contains the role of the kubeconfig."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        #[serde(rename = "clusterUser")]
        ClusterUser,
        #[serde(rename = "clusterCustomerUser")]
        ClusterCustomerUser,
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
                Self::ClusterUser => serializer.serialize_unit_variant("Name", 0u32, "clusterUser"),
                Self::ClusterCustomerUser => serializer.serialize_unit_variant("Name", 1u32, "clusterCustomerUser"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Get Telemetry Config Result appliance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplianceGetTelemetryConfigResult {
    #[doc = "Telemetry instrumentation key."]
    #[serde(rename = "telemetryInstrumentationKey", default, skip_serializing_if = "Option::is_none")]
    pub telemetry_instrumentation_key: Option<String>,
}
impl ApplianceGetTelemetryConfigResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Cluster User Credential appliance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplianceListCredentialResults {
    #[doc = "Contains the REP (rendezvous endpoint) and “Listener” access token from notification service (NS)."]
    #[serde(rename = "hybridConnectionConfig", default, skip_serializing_if = "Option::is_none")]
    pub hybrid_connection_config: Option<HybridConnectionConfig>,
    #[doc = "The list of appliance kubeconfigs."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub kubeconfigs: Vec<ApplianceCredentialKubeconfig>,
}
impl ApplianceListCredentialResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Cluster Keys Results appliance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplianceListKeysResults {
    #[doc = "Map of artifacts that contains a list of ArtifactProfile used to upload artifacts such as logs."]
    #[serde(rename = "artifactProfiles", default, skip_serializing_if = "Option::is_none")]
    pub artifact_profiles: Option<serde_json::Value>,
    #[doc = "The list of appliance kubeconfigs."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub kubeconfigs: Vec<ApplianceCredentialKubeconfig>,
    #[doc = "Map of Customer User Public, Private SSH Keys and Certificate when available."]
    #[serde(rename = "sshKeys", default, skip_serializing_if = "Option::is_none")]
    pub ssh_keys: Option<serde_json::Value>,
}
impl ApplianceListKeysResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Appliances operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplianceListResult {
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of Appliances."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Appliance>,
}
impl azure_core::Continuable for ApplianceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ApplianceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Appliances operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplianceOperation {
    #[doc = "Describes the properties of an Appliances Operation Value Display."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<ApplianceOperationValueDisplay>,
    #[doc = "Is this Operation a data plane operation"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "The name of the compute operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The origin of the compute operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl ApplianceOperation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of an Appliances Operation Value Display."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplianceOperationValueDisplay {
    #[doc = "The description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The display name of the compute operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The resource provider for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "The display name of the resource the operation applies to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
impl ApplianceOperationValueDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Lists of Appliances operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplianceOperationsList {
    #[doc = "Next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of applianceOperation"]
    pub value: Vec<ApplianceOperation>,
}
impl azure_core::Continuable for ApplianceOperationsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ApplianceOperationsList {
    pub fn new(value: Vec<ApplianceOperation>) -> Self {
        Self { next_link: None, value }
    }
}
#[doc = "Properties for an appliance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplianceProperties {
    #[doc = "Represents a supported Fabric/Infra. (AKSEdge etc...)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distro: Option<appliance_properties::Distro>,
    #[doc = "Contains infrastructure information about the Appliance"]
    #[serde(rename = "infrastructureConfig", default, skip_serializing_if = "Option::is_none")]
    pub infrastructure_config: Option<appliance_properties::InfrastructureConfig>,
    #[doc = "The current deployment or provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Certificates pair used to download MSI certificate from HIS. Can only be set once."]
    #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[doc = "Appliance’s health and state of connection to on-prem"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<appliance_properties::Status>,
    #[doc = "Version of the Appliance"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl ApplianceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod appliance_properties {
    use super::*;
    #[doc = "Represents a supported Fabric/Infra. (AKSEdge etc...)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Distro")]
    pub enum Distro {
        #[serde(rename = "AKSEdge")]
        AksEdge,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Distro {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Distro {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Distro {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AksEdge => serializer.serialize_unit_variant("Distro", 0u32, "AKSEdge"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Distro {
        fn default() -> Self {
            Self::AksEdge
        }
    }
    #[doc = "Contains infrastructure information about the Appliance"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct InfrastructureConfig {
        #[doc = "Information about the connected appliance."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<infrastructure_config::Provider>,
    }
    impl InfrastructureConfig {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod infrastructure_config {
        use super::*;
        #[doc = "Information about the connected appliance."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "Provider")]
        pub enum Provider {
            #[serde(rename = "VMWare")]
            VmWare,
            #[serde(rename = "HCI")]
            Hci,
            #[serde(rename = "SCVMM")]
            Scvmm,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for Provider {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for Provider {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for Provider {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::VmWare => serializer.serialize_unit_variant("Provider", 0u32, "VMWare"),
                    Self::Hci => serializer.serialize_unit_variant("Provider", 1u32, "HCI"),
                    Self::Scvmm => serializer.serialize_unit_variant("Provider", 2u32, "SCVMM"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
    #[doc = "Appliance’s health and state of connection to on-prem"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        WaitingForHeartbeat,
        Validating,
        Connecting,
        Connected,
        Running,
        PreparingForUpgrade,
        #[serde(rename = "ETCDSnapshotFailed")]
        EtcdSnapshotFailed,
        UpgradePrerequisitesCompleted,
        #[serde(rename = "ValidatingSFSConnectivity")]
        ValidatingSfsConnectivity,
        ValidatingImageDownload,
        ValidatingImageUpload,
        #[serde(rename = "ValidatingETCDHealth")]
        ValidatingEtcdHealth,
        PreUpgrade,
        #[serde(rename = "UpgradingKVAIO")]
        UpgradingKvaio,
        #[serde(rename = "WaitingForKVAIO")]
        WaitingForKvaio,
        ImagePending,
        ImageProvisioning,
        ImageProvisioned,
        ImageDownloading,
        ImageDownloaded,
        ImageDeprovisioning,
        ImageUnknown,
        UpdatingCloudOperator,
        WaitingForCloudOperator,
        #[serde(rename = "UpdatingCAPI")]
        UpdatingCapi,
        UpdatingCluster,
        PostUpgrade,
        UpgradeComplete,
        UpgradeClusterExtensionFailedToDelete,
        UpgradeFailed,
        Offline,
        None,
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
                Self::WaitingForHeartbeat => serializer.serialize_unit_variant("Status", 0u32, "WaitingForHeartbeat"),
                Self::Validating => serializer.serialize_unit_variant("Status", 1u32, "Validating"),
                Self::Connecting => serializer.serialize_unit_variant("Status", 2u32, "Connecting"),
                Self::Connected => serializer.serialize_unit_variant("Status", 3u32, "Connected"),
                Self::Running => serializer.serialize_unit_variant("Status", 4u32, "Running"),
                Self::PreparingForUpgrade => serializer.serialize_unit_variant("Status", 5u32, "PreparingForUpgrade"),
                Self::EtcdSnapshotFailed => serializer.serialize_unit_variant("Status", 6u32, "ETCDSnapshotFailed"),
                Self::UpgradePrerequisitesCompleted => serializer.serialize_unit_variant("Status", 7u32, "UpgradePrerequisitesCompleted"),
                Self::ValidatingSfsConnectivity => serializer.serialize_unit_variant("Status", 8u32, "ValidatingSFSConnectivity"),
                Self::ValidatingImageDownload => serializer.serialize_unit_variant("Status", 9u32, "ValidatingImageDownload"),
                Self::ValidatingImageUpload => serializer.serialize_unit_variant("Status", 10u32, "ValidatingImageUpload"),
                Self::ValidatingEtcdHealth => serializer.serialize_unit_variant("Status", 11u32, "ValidatingETCDHealth"),
                Self::PreUpgrade => serializer.serialize_unit_variant("Status", 12u32, "PreUpgrade"),
                Self::UpgradingKvaio => serializer.serialize_unit_variant("Status", 13u32, "UpgradingKVAIO"),
                Self::WaitingForKvaio => serializer.serialize_unit_variant("Status", 14u32, "WaitingForKVAIO"),
                Self::ImagePending => serializer.serialize_unit_variant("Status", 15u32, "ImagePending"),
                Self::ImageProvisioning => serializer.serialize_unit_variant("Status", 16u32, "ImageProvisioning"),
                Self::ImageProvisioned => serializer.serialize_unit_variant("Status", 17u32, "ImageProvisioned"),
                Self::ImageDownloading => serializer.serialize_unit_variant("Status", 18u32, "ImageDownloading"),
                Self::ImageDownloaded => serializer.serialize_unit_variant("Status", 19u32, "ImageDownloaded"),
                Self::ImageDeprovisioning => serializer.serialize_unit_variant("Status", 20u32, "ImageDeprovisioning"),
                Self::ImageUnknown => serializer.serialize_unit_variant("Status", 21u32, "ImageUnknown"),
                Self::UpdatingCloudOperator => serializer.serialize_unit_variant("Status", 22u32, "UpdatingCloudOperator"),
                Self::WaitingForCloudOperator => serializer.serialize_unit_variant("Status", 23u32, "WaitingForCloudOperator"),
                Self::UpdatingCapi => serializer.serialize_unit_variant("Status", 24u32, "UpdatingCAPI"),
                Self::UpdatingCluster => serializer.serialize_unit_variant("Status", 25u32, "UpdatingCluster"),
                Self::PostUpgrade => serializer.serialize_unit_variant("Status", 26u32, "PostUpgrade"),
                Self::UpgradeComplete => serializer.serialize_unit_variant("Status", 27u32, "UpgradeComplete"),
                Self::UpgradeClusterExtensionFailedToDelete => {
                    serializer.serialize_unit_variant("Status", 28u32, "UpgradeClusterExtensionFailedToDelete")
                }
                Self::UpgradeFailed => serializer.serialize_unit_variant("Status", 29u32, "UpgradeFailed"),
                Self::Offline => serializer.serialize_unit_variant("Status", 30u32, "Offline"),
                Self::None => serializer.serialize_unit_variant("Status", 31u32, "None"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Appliances patchable resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchableAppliance {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl PatchableAppliance {
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
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
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
