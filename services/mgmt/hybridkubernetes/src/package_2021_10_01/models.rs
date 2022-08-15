#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Represents a connected cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectedCluster {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Identity for the connected cluster."]
    pub identity: ConnectedClusterIdentity,
    #[doc = "Properties of the connected cluster."]
    pub properties: ConnectedClusterProperties,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ConnectedCluster {
    pub fn new(tracked_resource: TrackedResource, identity: ConnectedClusterIdentity, properties: ConnectedClusterProperties) -> Self {
        Self {
            tracked_resource,
            identity,
            properties,
            system_data: None,
        }
    }
}
#[doc = "Identity for the connected cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectedClusterIdentity {
    #[doc = "The principal id of connected cluster identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant id associated with the connected cluster. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The type of identity used for the connected cluster. The type 'SystemAssigned, includes a system created identity. The type 'None' means no identity is assigned to the connected cluster."]
    #[serde(rename = "type")]
    pub type_: connected_cluster_identity::Type,
}
impl ConnectedClusterIdentity {
    pub fn new(type_: connected_cluster_identity::Type) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
        }
    }
}
pub mod connected_cluster_identity {
    use super::*;
    #[doc = "The type of identity used for the connected cluster. The type 'SystemAssigned, includes a system created identity. The type 'None' means no identity is assigned to the connected cluster."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        None,
        SystemAssigned,
    }
    impl Default for Type {
        fn default() -> Self {
            Self::SystemAssigned
        }
    }
}
#[doc = "The paginated list of connected Clusters"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectedClusterList {
    #[doc = "The list of connected clusters"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConnectedCluster>,
    #[doc = "The link to fetch the next page of connected cluster"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ConnectedClusterList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ConnectedClusterList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Object containing updates for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectedClusterPatch {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties which can be patched on the connected cluster resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConnectedClusterPatchProperties>,
}
impl ConnectedClusterPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties which can be patched on the connected cluster resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectedClusterPatchProperties {}
impl ConnectedClusterPatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the connected cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectedClusterProperties {
    #[doc = "Base64 encoded public certificate used by the agent to do the initial handshake to the backend services in Azure."]
    #[serde(rename = "agentPublicKeyCertificate")]
    pub agent_public_key_certificate: String,
    #[doc = "The Kubernetes version of the connected cluster resource"]
    #[serde(rename = "kubernetesVersion", default, skip_serializing_if = "Option::is_none")]
    pub kubernetes_version: Option<String>,
    #[doc = "Number of nodes present in the connected cluster resource"]
    #[serde(rename = "totalNodeCount", default, skip_serializing_if = "Option::is_none")]
    pub total_node_count: Option<i64>,
    #[doc = "Number of CPU cores present in the connected cluster resource"]
    #[serde(rename = "totalCoreCount", default, skip_serializing_if = "Option::is_none")]
    pub total_core_count: Option<i32>,
    #[doc = "Version of the agent running on the connected cluster resource"]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "The current deployment state of connectedClusters."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ConnectedClusterProvisioningState>,
    #[doc = "The Kubernetes distribution running on this connected cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distribution: Option<String>,
    #[doc = "The infrastructure on which the Kubernetes cluster represented by this connected cluster is running on."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub infrastructure: Option<String>,
    #[doc = "Connected cluster offering"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offering: Option<String>,
    #[doc = "Expiration time of the managed identity certificate"]
    #[serde(rename = "managedIdentityCertificateExpirationTime", with = "azure_core::date::rfc3339::option")]
    pub managed_identity_certificate_expiration_time: Option<time::OffsetDateTime>,
    #[doc = "Time representing the last instance when heart beat was received from the cluster"]
    #[serde(rename = "lastConnectivityTime", with = "azure_core::date::rfc3339::option")]
    pub last_connectivity_time: Option<time::OffsetDateTime>,
    #[doc = "Represents the connectivity status of the connected cluster."]
    #[serde(rename = "connectivityStatus", default, skip_serializing_if = "Option::is_none")]
    pub connectivity_status: Option<connected_cluster_properties::ConnectivityStatus>,
}
impl ConnectedClusterProperties {
    pub fn new(agent_public_key_certificate: String) -> Self {
        Self {
            agent_public_key_certificate,
            kubernetes_version: None,
            total_node_count: None,
            total_core_count: None,
            agent_version: None,
            provisioning_state: None,
            distribution: None,
            infrastructure: None,
            offering: None,
            managed_identity_certificate_expiration_time: None,
            last_connectivity_time: None,
            connectivity_status: None,
        }
    }
}
pub mod connected_cluster_properties {
    use super::*;
    #[doc = "Represents the connectivity status of the connected cluster."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConnectivityStatus")]
    pub enum ConnectivityStatus {
        Connecting,
        Connected,
        Offline,
        Expired,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConnectivityStatus {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConnectivityStatus {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConnectivityStatus {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Connecting => serializer.serialize_unit_variant("ConnectivityStatus", 0u32, "Connecting"),
                Self::Connected => serializer.serialize_unit_variant("ConnectivityStatus", 1u32, "Connected"),
                Self::Offline => serializer.serialize_unit_variant("ConnectivityStatus", 2u32, "Offline"),
                Self::Expired => serializer.serialize_unit_variant("ConnectivityStatus", 3u32, "Expired"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The current deployment state of connectedClusters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ConnectedClusterProvisioningState")]
pub enum ConnectedClusterProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    Provisioning,
    Updating,
    Deleting,
    Accepted,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ConnectedClusterProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ConnectedClusterProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ConnectedClusterProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("ConnectedClusterProvisioningState", 0u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ConnectedClusterProvisioningState", 1u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ConnectedClusterProvisioningState", 2u32, "Canceled"),
            Self::Provisioning => serializer.serialize_unit_variant("ConnectedClusterProvisioningState", 3u32, "Provisioning"),
            Self::Updating => serializer.serialize_unit_variant("ConnectedClusterProvisioningState", 4u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ConnectedClusterProvisioningState", 5u32, "Deleting"),
            Self::Accepted => serializer.serialize_unit_variant("ConnectedClusterProvisioningState", 6u32, "Accepted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The credential result response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CredentialResult {
    #[doc = "The name of the credential."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Base64-encoded Kubernetes configuration file."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl CredentialResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of credential result response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CredentialResults {
    #[doc = "Contains the REP (rendezvous endpoint) and “Sender” access token."]
    #[serde(rename = "hybridConnectionConfig", default, skip_serializing_if = "Option::is_none")]
    pub hybrid_connection_config: Option<HybridConnectionConfig>,
    #[doc = "Base64-encoded Kubernetes configuration file."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub kubeconfigs: Vec<CredentialResult>,
}
impl CredentialResults {
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
#[doc = "Contains the REP (rendezvous endpoint) and “Sender” access token."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridConnectionConfig {
    #[doc = "Timestamp when this token will be expired."]
    #[serde(rename = "expirationTime", default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<i64>,
    #[doc = "Name of the connection"]
    #[serde(rename = "hybridConnectionName", default, skip_serializing_if = "Option::is_none")]
    pub hybrid_connection_name: Option<String>,
    #[doc = "Name of the relay."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relay: Option<String>,
    #[doc = "Sender access token"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}
impl HybridConnectionConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = ""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListClusterUserCredentialProperties {
    #[doc = "The mode of client authentication."]
    #[serde(rename = "authenticationMethod")]
    pub authentication_method: list_cluster_user_credential_properties::AuthenticationMethod,
    #[doc = "Boolean value to indicate whether the request is for client side proxy or not"]
    #[serde(rename = "clientProxy")]
    pub client_proxy: bool,
}
impl ListClusterUserCredentialProperties {
    pub fn new(authentication_method: list_cluster_user_credential_properties::AuthenticationMethod, client_proxy: bool) -> Self {
        Self {
            authentication_method,
            client_proxy,
        }
    }
}
pub mod list_cluster_user_credential_properties {
    use super::*;
    #[doc = "The mode of client authentication."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AuthenticationMethod")]
    pub enum AuthenticationMethod {
        Token,
        #[serde(rename = "AAD")]
        Aad,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AuthenticationMethod {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AuthenticationMethod {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AuthenticationMethod {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Token => serializer.serialize_unit_variant("AuthenticationMethod", 0u32, "Token"),
                Self::Aad => serializer.serialize_unit_variant("AuthenticationMethod", 1u32, "AAD"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Connected cluster API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {Microsoft.Kubernetes}/{resource}/{operation}"]
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
        #[doc = "Service provider: Microsoft.connectedClusters"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Connected Cluster Resource on which the operation is performed"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The paginated list of connected cluster API operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[doc = "The list of connected cluster API operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "The link to fetch the next page of connected cluster API operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationList {
    pub fn new() -> Self {
        Self::default()
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
