#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Information about azure databricks accessConnector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccessConnector {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccessConnectorProperties>,
}
impl AccessConnector {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            identity: None,
            properties: None,
        }
    }
}
#[doc = "List of azure databricks accessConnector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessConnectorListResult {
    #[doc = "The array of azure databricks accessConnector."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AccessConnector>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AccessConnectorListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AccessConnectorListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessConnectorProperties {
    #[doc = "Provisioning status of the accessConnector."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<access_connector_properties::ProvisioningState>,
}
impl AccessConnectorProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod access_connector_properties {
    use super::*;
    #[doc = "Provisioning status of the accessConnector."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Deleted,
        Failed,
        Succeeded,
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
                Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Deleted"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Failed"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Succeeded"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "An update to an azure databricks accessConnector."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessConnectorUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<IdentityData>,
}
impl AccessConnectorUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "AddressSpace contains an array of IP address ranges that can be used by subnets of the virtual network."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AddressSpace {
    #[doc = "A list of address blocks reserved for this virtual network in CIDR notation."]
    #[serde(rename = "addressPrefixes", default, skip_serializing_if = "Vec::is_empty")]
    pub address_prefixes: Vec<String>,
}
impl AddressSpace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provides details of the entity that created/updated the workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreatedBy {
    #[doc = "The Object ID that created the workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oid: Option<String>,
    #[doc = "The Personal Object ID corresponding to the object ID above"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub puid: Option<String>,
    #[doc = "The application ID of the application that initiated the creation of the workspace. For example, Azure Portal."]
    #[serde(rename = "applicationId", default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
}
impl CreatedBy {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type CreatedDateTime = time::OffsetDateTime;
#[doc = "The object that contains details of encryption used on the workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Encryption {
    #[doc = "The encryption keySource (provider). Possible values (case-insensitive):  Default, Microsoft.Keyvault"]
    #[serde(rename = "keySource", default, skip_serializing_if = "Option::is_none")]
    pub key_source: Option<encryption::KeySource>,
    #[doc = "The name of KeyVault key."]
    #[serde(rename = "KeyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[doc = "The version of KeyVault key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyversion: Option<String>,
    #[doc = "The Uri of KeyVault."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyvaulturi: Option<String>,
}
impl Encryption {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod encryption {
    use super::*;
    #[doc = "The encryption keySource (provider). Possible values (case-insensitive):  Default, Microsoft.Keyvault"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "KeySource")]
    pub enum KeySource {
        Default,
        #[serde(rename = "Microsoft.Keyvault")]
        MicrosoftKeyvault,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for KeySource {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for KeySource {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for KeySource {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("KeySource", 0u32, "Default"),
                Self::MicrosoftKeyvault => serializer.serialize_unit_variant("KeySource", 1u32, "Microsoft.Keyvault"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for KeySource {
        fn default() -> Self {
            Self::Default
        }
    }
}
#[doc = "Encryption entities for databricks workspace resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionEntitiesDefinition {
    #[doc = "The object that contains details of encryption used on the workspace."]
    #[serde(rename = "managedServices", default, skip_serializing_if = "Option::is_none")]
    pub managed_services: Option<EncryptionV2>,
}
impl EncryptionEntitiesDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The object that contains details of encryption used on the workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionV2 {
    #[doc = "The encryption keySource (provider). Possible values (case-insensitive):  Microsoft.Keyvault"]
    #[serde(rename = "keySource")]
    pub key_source: encryption_v2::KeySource,
    #[doc = "Key Vault input properties for encryption."]
    #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_properties: Option<encryption_v2::KeyVaultProperties>,
}
impl EncryptionV2 {
    pub fn new(key_source: encryption_v2::KeySource) -> Self {
        Self {
            key_source,
            key_vault_properties: None,
        }
    }
}
pub mod encryption_v2 {
    use super::*;
    #[doc = "The encryption keySource (provider). Possible values (case-insensitive):  Microsoft.Keyvault"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "KeySource")]
    pub enum KeySource {
        #[serde(rename = "Microsoft.Keyvault")]
        MicrosoftKeyvault,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for KeySource {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for KeySource {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for KeySource {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::MicrosoftKeyvault => serializer.serialize_unit_variant("KeySource", 0u32, "Microsoft.Keyvault"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Key Vault input properties for encryption."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct KeyVaultProperties {
        #[doc = "The Uri of KeyVault."]
        #[serde(rename = "keyVaultUri")]
        pub key_vault_uri: String,
        #[doc = "The name of KeyVault key."]
        #[serde(rename = "keyName")]
        pub key_name: String,
        #[doc = "The version of KeyVault key."]
        #[serde(rename = "keyVersion")]
        pub key_version: String,
    }
    impl KeyVaultProperties {
        pub fn new(key_vault_uri: String, key_name: String, key_version: String) -> Self {
            Self {
                key_vault_uri,
                key_name,
                key_version,
            }
        }
    }
}
#[doc = "A domain name or IP address the Workspace is reaching at."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointDependency {
    #[doc = "The domain name of the dependency."]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "The Ports used when connecting to domainName."]
    #[serde(rename = "endpointDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint_details: Vec<EndpointDetail>,
}
impl EndpointDependency {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Connect information from the Workspace to a single endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointDetail {
    #[doc = "An IP Address that Domain Name currently resolves to."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "The port an endpoint is connected to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "The time in milliseconds it takes for the connection to be created from the Workspace to this IpAddress at this Port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latency: Option<f64>,
    #[doc = "Whether it is possible to create a connection from the Workspace to this IpAddress at this Port."]
    #[serde(rename = "isAccessible", default, skip_serializing_if = "Option::is_none")]
    pub is_accessible: Option<bool>,
}
impl EndpointDetail {
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
}
impl ErrorDetail {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
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
    #[doc = "Inner error details if they exist."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<String>,
}
impl ErrorInfo {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            details: Vec::new(),
            innererror: None,
        }
    }
}
#[doc = "Contains details when the response code indicates an error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorInfo,
}
impl azure_core::Continuable for ErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponse {
    pub fn new(error: ErrorInfo) -> Self {
        Self { error }
    }
}
#[doc = "The group information for creating a private endpoint on a workspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupIdInformation {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The properties for a group information object"]
    pub properties: GroupIdInformationProperties,
}
impl GroupIdInformation {
    pub fn new(properties: GroupIdInformationProperties) -> Self {
        Self {
            resource: Resource::default(),
            id: None,
            name: None,
            type_: None,
            properties,
        }
    }
}
#[doc = "The properties for a group information object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupIdInformationProperties {
    #[doc = "The group id"]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The required members for a specific group id"]
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "The required DNS zones for a specific group id"]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
impl GroupIdInformationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Identity for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityData {
    #[doc = "The principal ID of resource identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of resource."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The identity type."]
    #[serde(rename = "type")]
    pub type_: identity_data::Type,
}
impl IdentityData {
    pub fn new(type_: identity_data::Type) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
        }
    }
}
pub mod identity_data {
    use super::*;
    #[doc = "The identity type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        None,
        SystemAssigned,
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
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Managed Identity details for storage account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedIdentityConfiguration {
    #[doc = "The objectId of the Managed Identity that is linked to the Managed Storage account."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant Id where the Managed Identity is created."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The type of Identity created. It can be either SystemAssigned or UserAssigned."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ManagedIdentityConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "The object that represents the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft.ResourceProvider"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list Resource Provider operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Resource Provider operations supported by the Resource Provider resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
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
#[doc = "Egress endpoints which Workspace connects to for common purposes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutboundEnvironmentEndpoint {
    #[doc = "The category of endpoints accessed by the Workspace, e.g. azure-storage, azure-mysql, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "The endpoints that Workspace connect to"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<EndpointDependency>,
}
impl OutboundEnvironmentEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type OutboundEnvironmentEndpointCollection = Vec<OutboundEnvironmentEndpoint>;
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PeeringProvisioningState")]
pub enum PeeringProvisioningState {
    Succeeded,
    Updating,
    Deleting,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PeeringProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PeeringProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PeeringProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("PeeringProvisioningState", 0u32, "Succeeded"),
            Self::Updating => serializer.serialize_unit_variant("PeeringProvisioningState", 1u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("PeeringProvisioningState", 2u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("PeeringProvisioningState", 3u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The private endpoint property of a private endpoint connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "The resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The private endpoint connection of a workspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnection {
    #[doc = "The resource identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The properties of a private endpoint connection"]
    pub properties: PrivateEndpointConnectionProperties,
}
impl PrivateEndpointConnection {
    pub fn new(properties: PrivateEndpointConnectionProperties) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            properties,
        }
    }
}
#[doc = "The properties of a private endpoint connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "The private endpoint property of a private endpoint connection"]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "The current state of a private endpoint connection"]
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new(private_link_service_connection_state: PrivateLinkServiceConnectionState) -> Self {
        Self {
            private_endpoint: None,
            private_link_service_connection_state,
            provisioning_state: None,
        }
    }
}
#[doc = "The current provisioning state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointConnectionProvisioningState")]
pub enum PrivateEndpointConnectionProvisioningState {
    Succeeded,
    Creating,
    Updating,
    Deleting,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateEndpointConnectionProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateEndpointConnectionProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateEndpointConnectionProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 0u32, "Succeeded"),
            Self::Creating => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 1u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 2u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 3u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 4u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "List of private link connections."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionsList {
    #[doc = "The list of returned private endpoint connection."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
    #[doc = "The URL to get the next set of endpoint connections."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateEndpointConnectionsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrivateEndpointConnectionsList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The available private link resources for a workspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourcesList {
    #[doc = "The list of available private link resources for a workspace"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GroupIdInformation>,
    #[doc = "The URL to get the next set of private link resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateLinkResourcesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrivateLinkResourcesList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The current state of a private endpoint connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "The status of a private endpoint connection"]
    pub status: private_link_service_connection_state::Status,
    #[doc = "The description for the current state of a private endpoint connection"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Actions required for a private endpoint connection"]
    #[serde(rename = "actionRequired", default, skip_serializing_if = "Option::is_none")]
    pub action_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new(status: private_link_service_connection_state::Status) -> Self {
        Self {
            status,
            description: None,
            action_required: None,
        }
    }
}
pub mod private_link_service_connection_state {
    use super::*;
    #[doc = "The status of a private endpoint connection"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Status")]
    pub enum Status {
        Pending,
        Approved,
        Rejected,
        Disconnected,
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
                Self::Pending => serializer.serialize_unit_variant("Status", 0u32, "Pending"),
                Self::Approved => serializer.serialize_unit_variant("Status", 1u32, "Approved"),
                Self::Rejected => serializer.serialize_unit_variant("Status", 2u32, "Rejected"),
                Self::Disconnected => serializer.serialize_unit_variant("Status", 3u32, "Disconnected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Provisioning status of the workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Accepted,
    Running,
    Ready,
    Creating,
    Created,
    Deleting,
    Deleted,
    Canceled,
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
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Accepted"),
            Self::Running => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Running"),
            Self::Ready => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Ready"),
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Creating"),
            Self::Created => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Created"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
            Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Deleted"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Canceled"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Failed"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 9u32, "Succeeded"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 10u32, "Updating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The core properties of ARM resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource Id for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. Ex- Microsoft.Compute/virtualMachines or Microsoft.Storage/storageAccounts."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SKU for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The SKU name."]
    pub name: String,
    #[doc = "The SKU tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
impl Sku {
    pub fn new(name: String) -> Self {
        Self { name, tier: None }
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
#[doc = "Peerings in a VirtualNetwork resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkPeering {
    #[doc = "Properties of the virtual network peering."]
    pub properties: VirtualNetworkPeeringPropertiesFormat,
    #[doc = "Name of the virtual network peering resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "type of the virtual network peering resource"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl VirtualNetworkPeering {
    pub fn new(properties: VirtualNetworkPeeringPropertiesFormat) -> Self {
        Self {
            properties,
            name: None,
            id: None,
            type_: None,
        }
    }
}
#[doc = "Gets all virtual network peerings under a workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkPeeringList {
    #[doc = "List of virtual network peerings on workspace."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<VirtualNetworkPeering>,
    #[doc = "URL to get the next set of virtual network peering list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for VirtualNetworkPeeringList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl VirtualNetworkPeeringList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the virtual network peering."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkPeeringPropertiesFormat {
    #[doc = "Whether the VMs in the local virtual network space would be able to access the VMs in remote virtual network space."]
    #[serde(rename = "allowVirtualNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub allow_virtual_network_access: Option<bool>,
    #[doc = "Whether the forwarded traffic from the VMs in the local virtual network will be allowed/disallowed in remote virtual network."]
    #[serde(rename = "allowForwardedTraffic", default, skip_serializing_if = "Option::is_none")]
    pub allow_forwarded_traffic: Option<bool>,
    #[doc = "If gateway links can be used in remote virtual networking to link to this virtual network."]
    #[serde(rename = "allowGatewayTransit", default, skip_serializing_if = "Option::is_none")]
    pub allow_gateway_transit: Option<bool>,
    #[doc = "If remote gateways can be used on this virtual network. If the flag is set to true, and allowGatewayTransit on remote peering is also true, virtual network will use gateways of remote virtual network for transit. Only one peering can have this flag set to true. This flag cannot be set if virtual network already has a gateway."]
    #[serde(rename = "useRemoteGateways", default, skip_serializing_if = "Option::is_none")]
    pub use_remote_gateways: Option<bool>,
    #[doc = " The remote virtual network should be in the same region. See here to learn more (https://docs.microsoft.com/en-us/azure/databricks/administration-guide/cloud-configurations/azure/vnet-peering)."]
    #[serde(rename = "databricksVirtualNetwork", default, skip_serializing_if = "Option::is_none")]
    pub databricks_virtual_network: Option<virtual_network_peering_properties_format::DatabricksVirtualNetwork>,
    #[doc = "AddressSpace contains an array of IP address ranges that can be used by subnets of the virtual network."]
    #[serde(rename = "databricksAddressSpace", default, skip_serializing_if = "Option::is_none")]
    pub databricks_address_space: Option<AddressSpace>,
    #[doc = " The remote virtual network should be in the same region. See here to learn more (https://docs.microsoft.com/en-us/azure/databricks/administration-guide/cloud-configurations/azure/vnet-peering)."]
    #[serde(rename = "remoteVirtualNetwork")]
    pub remote_virtual_network: virtual_network_peering_properties_format::RemoteVirtualNetwork,
    #[doc = "AddressSpace contains an array of IP address ranges that can be used by subnets of the virtual network."]
    #[serde(rename = "remoteAddressSpace", default, skip_serializing_if = "Option::is_none")]
    pub remote_address_space: Option<AddressSpace>,
    #[doc = "The status of the virtual network peering."]
    #[serde(rename = "peeringState", default, skip_serializing_if = "Option::is_none")]
    pub peering_state: Option<virtual_network_peering_properties_format::PeeringState>,
    #[doc = "The current provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PeeringProvisioningState>,
}
impl VirtualNetworkPeeringPropertiesFormat {
    pub fn new(remote_virtual_network: virtual_network_peering_properties_format::RemoteVirtualNetwork) -> Self {
        Self {
            allow_virtual_network_access: None,
            allow_forwarded_traffic: None,
            allow_gateway_transit: None,
            use_remote_gateways: None,
            databricks_virtual_network: None,
            databricks_address_space: None,
            remote_virtual_network,
            remote_address_space: None,
            peering_state: None,
            provisioning_state: None,
        }
    }
}
pub mod virtual_network_peering_properties_format {
    use super::*;
    #[doc = " The remote virtual network should be in the same region. See here to learn more (https://docs.microsoft.com/en-us/azure/databricks/administration-guide/cloud-configurations/azure/vnet-peering)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct DatabricksVirtualNetwork {
        #[doc = "The Id of the databricks virtual network."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }
    impl DatabricksVirtualNetwork {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = " The remote virtual network should be in the same region. See here to learn more (https://docs.microsoft.com/en-us/azure/databricks/administration-guide/cloud-configurations/azure/vnet-peering)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct RemoteVirtualNetwork {
        #[doc = "The Id of the remote virtual network."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }
    impl RemoteVirtualNetwork {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The status of the virtual network peering."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PeeringState")]
    pub enum PeeringState {
        Initiated,
        Connected,
        Disconnected,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PeeringState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PeeringState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PeeringState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Initiated => serializer.serialize_unit_variant("PeeringState", 0u32, "Initiated"),
                Self::Connected => serializer.serialize_unit_variant("PeeringState", 1u32, "Connected"),
                Self::Disconnected => serializer.serialize_unit_variant("PeeringState", 2u32, "Disconnected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Information about workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The workspace properties."]
    pub properties: WorkspaceProperties,
    #[doc = "SKU for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Workspace {
    pub fn new(tracked_resource: TrackedResource, properties: WorkspaceProperties) -> Self {
        Self {
            tracked_resource,
            properties,
            sku: None,
            system_data: None,
        }
    }
}
#[doc = "The value which should be used for this field."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceCustomBooleanParameter {
    #[doc = "Provisioning status of the workspace."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<WorkspaceCustomParameterType>,
    #[doc = "The value which should be used for this field."]
    pub value: bool,
}
impl WorkspaceCustomBooleanParameter {
    pub fn new(value: bool) -> Self {
        Self { type_: None, value }
    }
}
#[doc = "The value which should be used for this field."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceCustomObjectParameter {
    #[doc = "Provisioning status of the workspace."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<WorkspaceCustomParameterType>,
    #[doc = "The value which should be used for this field."]
    pub value: serde_json::Value,
}
impl WorkspaceCustomObjectParameter {
    pub fn new(value: serde_json::Value) -> Self {
        Self { type_: None, value }
    }
}
#[doc = "Provisioning status of the workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "WorkspaceCustomParameterType")]
pub enum WorkspaceCustomParameterType {
    Bool,
    Object,
    String,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WorkspaceCustomParameterType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WorkspaceCustomParameterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WorkspaceCustomParameterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Bool => serializer.serialize_unit_variant("WorkspaceCustomParameterType", 0u32, "Bool"),
            Self::Object => serializer.serialize_unit_variant("WorkspaceCustomParameterType", 1u32, "Object"),
            Self::String => serializer.serialize_unit_variant("WorkspaceCustomParameterType", 2u32, "String"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Custom Parameters used for Cluster Creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceCustomParameters {
    #[doc = "The Value."]
    #[serde(rename = "amlWorkspaceId", default, skip_serializing_if = "Option::is_none")]
    pub aml_workspace_id: Option<WorkspaceCustomStringParameter>,
    #[doc = "The Value."]
    #[serde(rename = "customVirtualNetworkId", default, skip_serializing_if = "Option::is_none")]
    pub custom_virtual_network_id: Option<WorkspaceCustomStringParameter>,
    #[doc = "The Value."]
    #[serde(rename = "customPublicSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub custom_public_subnet_name: Option<WorkspaceCustomStringParameter>,
    #[doc = "The Value."]
    #[serde(rename = "customPrivateSubnetName", default, skip_serializing_if = "Option::is_none")]
    pub custom_private_subnet_name: Option<WorkspaceCustomStringParameter>,
    #[doc = "The value which should be used for this field."]
    #[serde(rename = "enableNoPublicIp", default, skip_serializing_if = "Option::is_none")]
    pub enable_no_public_ip: Option<WorkspaceCustomBooleanParameter>,
    #[doc = "The Value."]
    #[serde(rename = "loadBalancerBackendPoolName", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_backend_pool_name: Option<WorkspaceCustomStringParameter>,
    #[doc = "The Value."]
    #[serde(rename = "loadBalancerId", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_id: Option<WorkspaceCustomStringParameter>,
    #[doc = "The Value."]
    #[serde(rename = "natGatewayName", default, skip_serializing_if = "Option::is_none")]
    pub nat_gateway_name: Option<WorkspaceCustomStringParameter>,
    #[doc = "The Value."]
    #[serde(rename = "publicIpName", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_name: Option<WorkspaceCustomStringParameter>,
    #[doc = "The value which should be used for this field."]
    #[serde(rename = "prepareEncryption", default, skip_serializing_if = "Option::is_none")]
    pub prepare_encryption: Option<WorkspaceCustomBooleanParameter>,
    #[doc = "The object that contains details of encryption used on the workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<WorkspaceEncryptionParameter>,
    #[doc = "The value which should be used for this field."]
    #[serde(rename = "requireInfrastructureEncryption", default, skip_serializing_if = "Option::is_none")]
    pub require_infrastructure_encryption: Option<WorkspaceCustomBooleanParameter>,
    #[doc = "The Value."]
    #[serde(rename = "storageAccountName", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_name: Option<WorkspaceCustomStringParameter>,
    #[doc = "The Value."]
    #[serde(rename = "storageAccountSkuName", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_sku_name: Option<WorkspaceCustomStringParameter>,
    #[doc = "The Value."]
    #[serde(rename = "vnetAddressPrefix", default, skip_serializing_if = "Option::is_none")]
    pub vnet_address_prefix: Option<WorkspaceCustomStringParameter>,
    #[doc = "The value which should be used for this field."]
    #[serde(rename = "resourceTags", default, skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<WorkspaceCustomObjectParameter>,
}
impl WorkspaceCustomParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceCustomStringParameter {
    #[doc = "Provisioning status of the workspace."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<WorkspaceCustomParameterType>,
    #[doc = "The value which should be used for this field."]
    pub value: String,
}
impl WorkspaceCustomStringParameter {
    pub fn new(value: String) -> Self {
        Self { type_: None, value }
    }
}
#[doc = "The object that contains details of encryption used on the workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceEncryptionParameter {
    #[doc = "Provisioning status of the workspace."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<WorkspaceCustomParameterType>,
    #[doc = "The object that contains details of encryption used on the workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Encryption>,
}
impl WorkspaceEncryptionParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of workspaces."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceListResult {
    #[doc = "The array of workspaces."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workspace>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for WorkspaceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl WorkspaceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The workspace properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceProperties {
    #[doc = "The managed resource group Id."]
    #[serde(rename = "managedResourceGroupId")]
    pub managed_resource_group_id: String,
    #[doc = "Custom Parameters used for Cluster Creation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<WorkspaceCustomParameters>,
    #[doc = "Provisioning status of the workspace."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The blob URI where the UI definition file is located."]
    #[serde(rename = "uiDefinitionUri", default, skip_serializing_if = "Option::is_none")]
    pub ui_definition_uri: Option<String>,
    #[doc = "The workspace provider authorizations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authorizations: Vec<WorkspaceProviderAuthorization>,
    #[doc = "Provides details of the entity that created/updated the workspace."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<CreatedBy>,
    #[doc = "Provides details of the entity that created/updated the workspace."]
    #[serde(rename = "updatedBy", default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<CreatedBy>,
    #[doc = "The date and time stamp when the workspace was created."]
    #[serde(rename = "createdDateTime", default, skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<CreatedDateTime>,
    #[doc = "The unique identifier of the databricks workspace in databricks control plane."]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "The workspace URL which is of the format 'adb-{workspaceId}.{random}.azuredatabricks.net'"]
    #[serde(rename = "workspaceUrl", default, skip_serializing_if = "Option::is_none")]
    pub workspace_url: Option<String>,
    #[doc = "The Managed Identity details for storage account."]
    #[serde(rename = "storageAccountIdentity", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_identity: Option<ManagedIdentityConfiguration>,
    #[doc = "Encryption properties for databricks workspace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<workspace_properties::Encryption>,
    #[doc = "Private endpoint connections created on the workspace"]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "The network access type for accessing workspace. Set value to disabled to access workspace only via private link."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<workspace_properties::PublicNetworkAccess>,
    #[doc = "Gets or sets a value indicating whether data plane (clusters) to control plane communication happen over private endpoint. Supported values are 'AllRules' and 'NoAzureDatabricksRules'. 'NoAzureServiceRules' value is for internal use only."]
    #[serde(rename = "requiredNsgRules", default, skip_serializing_if = "Option::is_none")]
    pub required_nsg_rules: Option<workspace_properties::RequiredNsgRules>,
}
impl WorkspaceProperties {
    pub fn new(managed_resource_group_id: String) -> Self {
        Self {
            managed_resource_group_id,
            parameters: None,
            provisioning_state: None,
            ui_definition_uri: None,
            authorizations: Vec::new(),
            created_by: None,
            updated_by: None,
            created_date_time: None,
            workspace_id: None,
            workspace_url: None,
            storage_account_identity: None,
            encryption: None,
            private_endpoint_connections: Vec::new(),
            public_network_access: None,
            required_nsg_rules: None,
        }
    }
}
pub mod workspace_properties {
    use super::*;
    #[doc = "Encryption properties for databricks workspace"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Encryption {
        #[doc = "Encryption entities for databricks workspace resource."]
        pub entities: EncryptionEntitiesDefinition,
    }
    impl Encryption {
        pub fn new(entities: EncryptionEntitiesDefinition) -> Self {
            Self { entities }
        }
    }
    #[doc = "The network access type for accessing workspace. Set value to disabled to access workspace only via private link."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PublicNetworkAccess")]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PublicNetworkAccess {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PublicNetworkAccess {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PublicNetworkAccess {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Enabled => serializer.serialize_unit_variant("PublicNetworkAccess", 0u32, "Enabled"),
                Self::Disabled => serializer.serialize_unit_variant("PublicNetworkAccess", 1u32, "Disabled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Gets or sets a value indicating whether data plane (clusters) to control plane communication happen over private endpoint. Supported values are 'AllRules' and 'NoAzureDatabricksRules'. 'NoAzureServiceRules' value is for internal use only."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RequiredNsgRules")]
    pub enum RequiredNsgRules {
        AllRules,
        NoAzureDatabricksRules,
        NoAzureServiceRules,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RequiredNsgRules {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RequiredNsgRules {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RequiredNsgRules {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AllRules => serializer.serialize_unit_variant("RequiredNsgRules", 0u32, "AllRules"),
                Self::NoAzureDatabricksRules => serializer.serialize_unit_variant("RequiredNsgRules", 1u32, "NoAzureDatabricksRules"),
                Self::NoAzureServiceRules => serializer.serialize_unit_variant("RequiredNsgRules", 2u32, "NoAzureServiceRules"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The workspace provider authorization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceProviderAuthorization {
    #[doc = "The provider's principal identifier. This is the identity that the provider will use to call ARM to manage the workspace resources."]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[doc = "The provider's role definition identifier. This role will define all the permissions that the provider must have on the workspace's container resource group. This role definition cannot have permission to delete the resource group."]
    #[serde(rename = "roleDefinitionId")]
    pub role_definition_id: String,
}
impl WorkspaceProviderAuthorization {
    pub fn new(principal_id: String, role_definition_id: String) -> Self {
        Self {
            principal_id,
            role_definition_id,
        }
    }
}
#[doc = "An update to a workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl WorkspaceUpdate {
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
