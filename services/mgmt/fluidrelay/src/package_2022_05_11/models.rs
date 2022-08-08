#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "All Customer-managed key encryption properties for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomerManagedKeyEncryptionProperties {
    #[doc = "All identity configuration for Customer-managed key settings defining which identity should be used to auth to Key Vault."]
    #[serde(rename = "keyEncryptionKeyIdentity", default, skip_serializing_if = "Option::is_none")]
    pub key_encryption_key_identity: Option<customer_managed_key_encryption_properties::KeyEncryptionKeyIdentity>,
    #[doc = "key encryption key Url, with or without a version. Ex: https://contosovault.vault.azure.net/keys/contosokek/562a4bb76b524a1493a6afe8e536ee78 or https://contosovault.vault.azure.net/keys/contosokek. Key auto rotation is enabled by providing a key uri without version. Otherwise, customer is responsible for rotating the key. The keyEncryptionKeyIdentity(either SystemAssigned or UserAssigned) should have permission to access this key url."]
    #[serde(rename = "keyEncryptionKeyUrl", default, skip_serializing_if = "Option::is_none")]
    pub key_encryption_key_url: Option<String>,
}
impl CustomerManagedKeyEncryptionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod customer_managed_key_encryption_properties {
    use super::*;
    #[doc = "All identity configuration for Customer-managed key settings defining which identity should be used to auth to Key Vault."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct KeyEncryptionKeyIdentity {
        #[doc = "Values can be SystemAssigned or UserAssigned"]
        #[serde(rename = "identityType", default, skip_serializing_if = "Option::is_none")]
        pub identity_type: Option<key_encryption_key_identity::IdentityType>,
        #[doc = "user assigned identity to use for accessing key encryption key Url. Ex: /subscriptions/fa5fc227-a624-475e-b696-cdd604c735bc/resourceGroups/<resource group>/providers/Microsoft.ManagedIdentity/userAssignedIdentities/myId. Mutually exclusive with identityType systemAssignedIdentity."]
        #[serde(rename = "userAssignedIdentityResourceId", default, skip_serializing_if = "Option::is_none")]
        pub user_assigned_identity_resource_id: Option<String>,
    }
    impl KeyEncryptionKeyIdentity {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod key_encryption_key_identity {
        use super::*;
        #[doc = "Values can be SystemAssigned or UserAssigned"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum IdentityType {
            SystemAssigned,
            UserAssigned,
        }
    }
}
#[doc = "All encryption configuration for a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionProperties {
    #[doc = "All Customer-managed key encryption properties for the resource."]
    #[serde(rename = "customerManagedKeyEncryption", default, skip_serializing_if = "Option::is_none")]
    pub customer_managed_key_encryption: Option<CustomerManagedKeyEncryptionProperties>,
}
impl EncryptionProperties {
    pub fn new() -> Self {
        Self::default()
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
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
#[doc = "A FluidRelay Container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FluidRelayContainer {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "The properties of a Fluid Relay Container resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FluidRelayContainerProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl FluidRelayContainer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FluidRelayContainerList {
    #[doc = "A sequence of FluidRelay containers."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<FluidRelayContainer>,
    #[doc = "A link to the next page of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FluidRelayContainerList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl FluidRelayContainerList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a Fluid Relay Container resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FluidRelayContainerProperties {
    #[doc = "The Fluid tenantId for this container"]
    #[serde(rename = "frsTenantId", default, skip_serializing_if = "Option::is_none")]
    pub frs_tenant_id: Option<String>,
    #[doc = "The frsContainerId for this container"]
    #[serde(rename = "frsContainerId", default, skip_serializing_if = "Option::is_none")]
    pub frs_container_id: Option<String>,
    #[doc = "Provision states for FluidRelay RP"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<fluid_relay_container_properties::ProvisioningState>,
    #[doc = "The creation time of this resource"]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "Last time when user access this resource"]
    #[serde(rename = "lastAccessTime", with = "azure_core::date::rfc3339::option")]
    pub last_access_time: Option<time::OffsetDateTime>,
}
impl FluidRelayContainerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod fluid_relay_container_properties {
    use super::*;
    #[doc = "Provision states for FluidRelay RP"]
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
}
#[doc = "The Fluid Relay endpoints for this server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FluidRelayEndpoints {
    #[doc = "The Fluid Relay Orderer endpoints."]
    #[serde(rename = "ordererEndpoints", default, skip_serializing_if = "Vec::is_empty")]
    pub orderer_endpoints: Vec<String>,
    #[doc = "The Fluid Relay storage endpoints."]
    #[serde(rename = "storageEndpoints", default, skip_serializing_if = "Vec::is_empty")]
    pub storage_endpoints: Vec<String>,
}
impl FluidRelayEndpoints {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A FluidRelay Server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FluidRelayServer {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "The properties of a Fluid Relay Service resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FluidRelayServerProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
}
impl FluidRelayServer {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            system_data: None,
            identity: None,
        }
    }
}
#[doc = "The set of available keys for this server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FluidRelayServerKeys {
    #[doc = "The primary key for this server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key1: Option<String>,
    #[doc = "The secondary key for this server"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key2: Option<String>,
}
impl FluidRelayServerKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paged response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FluidRelayServerList {
    #[doc = "A sequence of FluidRelay servers."]
    pub value: Vec<FluidRelayServer>,
    #[doc = "A link to the next page of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for FluidRelayServerList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl FluidRelayServerList {
    pub fn new(value: Vec<FluidRelayServer>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The properties of a Fluid Relay Service resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FluidRelayServerProperties {
    #[doc = "The Fluid tenantId for this server"]
    #[serde(rename = "frsTenantId", default, skip_serializing_if = "Option::is_none")]
    pub frs_tenant_id: Option<String>,
    #[doc = "The Fluid Relay endpoints for this server"]
    #[serde(rename = "fluidRelayEndpoints", default, skip_serializing_if = "Option::is_none")]
    pub fluid_relay_endpoints: Option<FluidRelayEndpoints>,
    #[doc = "Provision states for FluidRelay RP"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<fluid_relay_server_properties::ProvisioningState>,
    #[doc = "All encryption configuration for a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionProperties>,
}
impl FluidRelayServerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod fluid_relay_server_properties {
    use super::*;
    #[doc = "Provision states for FluidRelay RP"]
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
}
#[doc = "The updatable properties of a Fluid Relay server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FluidRelayServerUpdate {
    #[doc = "The properties that can be provided when updating FluidRelayServer resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FluidRelayServerUpdateProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[doc = "The geo-location where the resource lives"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl FluidRelayServerUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties that can be provided when updating FluidRelayServer resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FluidRelayServerUpdateProperties {
    #[doc = "All encryption configuration for a resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<EncryptionProperties>,
}
impl FluidRelayServerUpdateProperties {
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
    #[doc = "The list of user identities associated with the resource."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
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
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
        None,
    }
}
#[doc = "The object that represents the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Service provider: Microsoft.FluidRelay"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Type on which the operation is performed, e.g., 'servers'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Operation type, e.g., read, write, delete, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the operation, e.g., 'Write confluent'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of GET request to list FluidRelay operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of FluidRelay operations supported by the Microsoft.FluidRelay provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationResult>,
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
#[doc = "A FluidRelay REST API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResult {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
impl OperationResult {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "Specifies which key should be generated."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegenerateKeyRequest {
    #[doc = "The key to regenerate."]
    #[serde(rename = "keyName")]
    pub key_name: regenerate_key_request::KeyName,
}
impl RegenerateKeyRequest {
    pub fn new(key_name: regenerate_key_request::KeyName) -> Self {
        Self { key_name }
    }
}
pub mod regenerate_key_request {
    use super::*;
    #[doc = "The key to regenerate."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyName {
        #[serde(rename = "key1")]
        Key1,
        #[serde(rename = "key2")]
        Key2,
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
}
impl Resource {
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
    #[doc = "The timestamp of resource last modification (UTC)"]
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
