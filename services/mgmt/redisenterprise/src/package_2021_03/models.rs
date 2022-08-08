#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The secret access keys used for authenticating connections to redis"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessKeys {
    #[doc = "The current primary key that clients can use to authenticate"]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "The current secondary key that clients can use to authenticate"]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
}
impl AccessKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the RedisEnterprise cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "SKU parameters supplied to the create RedisEnterprise operation."]
    pub sku: Sku,
    #[doc = "The Availability Zones where this cluster will be deployed."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
    #[doc = "Properties of RedisEnterprise clusters, as opposed to general resource properties like location, tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterProperties>,
}
impl Cluster {
    pub fn new(tracked_resource: TrackedResource, sku: Sku) -> Self {
        Self {
            tracked_resource,
            sku,
            zones: Vec::new(),
            properties: None,
        }
    }
}
#[doc = "The response of a list-all operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterList {
    #[doc = "List of clusters."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Cluster>,
    #[doc = "The URI to fetch the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ClusterList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ClusterList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of RedisEnterprise clusters, as opposed to general resource properties like location, tags"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterProperties {
    #[doc = "The minimum TLS version for the cluster to support, e.g. '1.2'"]
    #[serde(rename = "minimumTlsVersion", default, skip_serializing_if = "Option::is_none")]
    pub minimum_tls_version: Option<cluster_properties::MinimumTlsVersion>,
    #[doc = "DNS name of the cluster endpoint"]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "Current provisioning status"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Current resource status"]
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<ResourceState>,
    #[doc = "Version of redis the cluster supports, e.g. '6'"]
    #[serde(rename = "redisVersion", default, skip_serializing_if = "Option::is_none")]
    pub redis_version: Option<String>,
    #[doc = "List of private endpoint connections associated with the specified RedisEnterprise cluster"]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
}
impl ClusterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cluster_properties {
    use super::*;
    #[doc = "The minimum TLS version for the cluster to support, e.g. '1.2'"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MinimumTlsVersion")]
    pub enum MinimumTlsVersion {
        #[serde(rename = "1.0")]
        N1_0,
        #[serde(rename = "1.1")]
        N1_1,
        #[serde(rename = "1.2")]
        N1_2,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MinimumTlsVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MinimumTlsVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MinimumTlsVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::N1_0 => serializer.serialize_unit_variant("MinimumTlsVersion", 0u32, "1.0"),
                Self::N1_1 => serializer.serialize_unit_variant("MinimumTlsVersion", 1u32, "1.1"),
                Self::N1_2 => serializer.serialize_unit_variant("MinimumTlsVersion", 2u32, "1.2"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A partial update to the RedisEnterprise cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterUpdate {
    #[doc = "SKU parameters supplied to the create RedisEnterprise operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Properties of RedisEnterprise clusters, as opposed to general resource properties like location, tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ClusterUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a database on the RedisEnterprise cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Database {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of RedisEnterprise databases, as opposed to general resource properties like location, tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseProperties>,
}
impl Database {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of a list-all operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseList {
    #[doc = "List of databases"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Database>,
    #[doc = "The URI to fetch the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DatabaseList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DatabaseList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of RedisEnterprise databases, as opposed to general resource properties like location, tags"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseProperties {
    #[doc = "Specifies whether redis clients can connect using TLS-encrypted or plaintext redis protocols. Default is TLS-encrypted."]
    #[serde(rename = "clientProtocol", default, skip_serializing_if = "Option::is_none")]
    pub client_protocol: Option<database_properties::ClientProtocol>,
    #[doc = "TCP port of the database endpoint. Specified at create time. Defaults to an available port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "Current provisioning status"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Current resource status"]
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<ResourceState>,
    #[doc = "Clustering policy - default is OSSCluster. Specified at create time."]
    #[serde(rename = "clusteringPolicy", default, skip_serializing_if = "Option::is_none")]
    pub clustering_policy: Option<database_properties::ClusteringPolicy>,
    #[doc = "Redis eviction policy - default is VolatileLRU"]
    #[serde(rename = "evictionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub eviction_policy: Option<database_properties::EvictionPolicy>,
    #[doc = "Persistence-related configuration for the RedisEnterprise database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persistence: Option<Persistence>,
    #[doc = "Optional set of redis modules to enable in this database - modules can only be added at creation time."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modules: Vec<Module>,
}
impl DatabaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod database_properties {
    use super::*;
    #[doc = "Specifies whether redis clients can connect using TLS-encrypted or plaintext redis protocols. Default is TLS-encrypted."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ClientProtocol")]
    pub enum ClientProtocol {
        Encrypted,
        Plaintext,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ClientProtocol {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ClientProtocol {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ClientProtocol {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Encrypted => serializer.serialize_unit_variant("ClientProtocol", 0u32, "Encrypted"),
                Self::Plaintext => serializer.serialize_unit_variant("ClientProtocol", 1u32, "Plaintext"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Clustering policy - default is OSSCluster. Specified at create time."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ClusteringPolicy")]
    pub enum ClusteringPolicy {
        EnterpriseCluster,
        #[serde(rename = "OSSCluster")]
        OssCluster,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ClusteringPolicy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ClusteringPolicy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ClusteringPolicy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::EnterpriseCluster => serializer.serialize_unit_variant("ClusteringPolicy", 0u32, "EnterpriseCluster"),
                Self::OssCluster => serializer.serialize_unit_variant("ClusteringPolicy", 1u32, "OSSCluster"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Redis eviction policy - default is VolatileLRU"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "EvictionPolicy")]
    pub enum EvictionPolicy {
        #[serde(rename = "AllKeysLFU")]
        AllKeysLfu,
        #[serde(rename = "AllKeysLRU")]
        AllKeysLru,
        AllKeysRandom,
        #[serde(rename = "VolatileLRU")]
        VolatileLru,
        #[serde(rename = "VolatileLFU")]
        VolatileLfu,
        #[serde(rename = "VolatileTTL")]
        VolatileTtl,
        VolatileRandom,
        NoEviction,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for EvictionPolicy {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for EvictionPolicy {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for EvictionPolicy {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::AllKeysLfu => serializer.serialize_unit_variant("EvictionPolicy", 0u32, "AllKeysLFU"),
                Self::AllKeysLru => serializer.serialize_unit_variant("EvictionPolicy", 1u32, "AllKeysLRU"),
                Self::AllKeysRandom => serializer.serialize_unit_variant("EvictionPolicy", 2u32, "AllKeysRandom"),
                Self::VolatileLru => serializer.serialize_unit_variant("EvictionPolicy", 3u32, "VolatileLRU"),
                Self::VolatileLfu => serializer.serialize_unit_variant("EvictionPolicy", 4u32, "VolatileLFU"),
                Self::VolatileTtl => serializer.serialize_unit_variant("EvictionPolicy", 5u32, "VolatileTTL"),
                Self::VolatileRandom => serializer.serialize_unit_variant("EvictionPolicy", 6u32, "VolatileRandom"),
                Self::NoEviction => serializer.serialize_unit_variant("EvictionPolicy", 7u32, "NoEviction"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A partial update to the RedisEnterprise database"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseUpdate {
    #[doc = "Properties of RedisEnterprise databases, as opposed to general resource properties like location, tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseProperties>,
}
impl DatabaseUpdate {
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
#[doc = "Parameters for a Redis Enterprise export operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportClusterParameters {
    #[doc = "SAS URI for the target directory to export to"]
    #[serde(rename = "sasUri")]
    pub sas_uri: String,
}
impl ExportClusterParameters {
    pub fn new(sas_uri: String) -> Self {
        Self { sas_uri }
    }
}
#[doc = "Parameters for a Redis Enterprise import operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportClusterParameters {
    #[doc = "SAS URI for the target blob to import from"]
    #[serde(rename = "sasUri")]
    pub sas_uri: String,
}
impl ImportClusterParameters {
    pub fn new(sas_uri: String) -> Self {
        Self { sas_uri }
    }
}
#[doc = "Specifies configuration of a redis module"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Module {
    #[doc = "The name of the module, e.g. 'RedisBloom', 'RediSearch', 'RedisTimeSeries'"]
    pub name: String,
    #[doc = "Configuration options for the module, e.g. 'ERROR_RATE 0.00 INITIAL_SIZE 400'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<String>,
    #[doc = "The version of the module, e.g. '1.0'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl Module {
    pub fn new(name: String) -> Self {
        Self {
            name,
            args: None,
            version: None,
        }
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results (if there are any)."]
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
#[doc = "The status of a long-running operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatus {
    #[doc = "The operation's unique id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The operation's name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The start time of the operation."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The end time of the operation."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The current status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl OperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Persistence-related configuration for the RedisEnterprise database"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Persistence {
    #[doc = "Sets whether AOF is enabled."]
    #[serde(rename = "aofEnabled", default, skip_serializing_if = "Option::is_none")]
    pub aof_enabled: Option<bool>,
    #[doc = "Sets whether RDB is enabled."]
    #[serde(rename = "rdbEnabled", default, skip_serializing_if = "Option::is_none")]
    pub rdb_enabled: Option<bool>,
    #[doc = "Sets the frequency at which data is written to disk."]
    #[serde(rename = "aofFrequency", default, skip_serializing_if = "Option::is_none")]
    pub aof_frequency: Option<persistence::AofFrequency>,
    #[doc = "Sets the frequency at which a snapshot of the database is created."]
    #[serde(rename = "rdbFrequency", default, skip_serializing_if = "Option::is_none")]
    pub rdb_frequency: Option<persistence::RdbFrequency>,
}
impl Persistence {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod persistence {
    use super::*;
    #[doc = "Sets the frequency at which data is written to disk."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "AofFrequency")]
    pub enum AofFrequency {
        #[serde(rename = "1s")]
        N1s,
        #[serde(rename = "always")]
        Always,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for AofFrequency {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for AofFrequency {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for AofFrequency {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::N1s => serializer.serialize_unit_variant("AofFrequency", 0u32, "1s"),
                Self::Always => serializer.serialize_unit_variant("AofFrequency", 1u32, "always"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Sets the frequency at which a snapshot of the database is created."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RdbFrequency")]
    pub enum RdbFrequency {
        #[serde(rename = "1h")]
        N1h,
        #[serde(rename = "6h")]
        N6h,
        #[serde(rename = "12h")]
        N12h,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RdbFrequency {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RdbFrequency {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RdbFrequency {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::N1h => serializer.serialize_unit_variant("RdbFrequency", 0u32, "1h"),
                Self::N6h => serializer.serialize_unit_variant("RdbFrequency", 1u32, "6h"),
                Self::N12h => serializer.serialize_unit_variant("RdbFrequency", 2u32, "12h"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Private Endpoint resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "The ARM identifier for Private Endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Private Endpoint Connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the PrivateEndpointConnectProperties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of private endpoint connection associated with the specified storage account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "Array of private endpoint connections"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
}
impl azure_core::Continuable for PrivateEndpointConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl PrivateEndpointConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the PrivateEndpointConnectProperties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "The Private Endpoint resource."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "A collection of information about the state of the connection between service consumer and provider."]
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
            Self::Deleting => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 2u32, "Deleting"),
            Self::Failed => serializer.serialize_unit_variant("PrivateEndpointConnectionProvisioningState", 3u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The private endpoint connection status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PrivateEndpointServiceConnectionStatus")]
pub enum PrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PrivateEndpointServiceConnectionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PrivateEndpointServiceConnectionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PrivateEndpointServiceConnectionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Pending => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 0u32, "Pending"),
            Self::Approved => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 1u32, "Approved"),
            Self::Rejected => serializer.serialize_unit_variant("PrivateEndpointServiceConnectionStatus", 2u32, "Rejected"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a private link resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private link resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceListResult {
    #[doc = "Array of private link resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
}
impl azure_core::Continuable for PrivateLinkResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl PrivateLinkResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[doc = "The private link resource group id."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The private link resource required member names."]
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[doc = "The private link resource Private link DNS zone name."]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A collection of information about the state of the connection between service consumer and provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[doc = "The private endpoint connection status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
    #[doc = "The reason for approval/rejection of the connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A message indicating if changes on the service provider require any updates on the consumer."]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Current provisioning status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Succeeded,
    Failed,
    Canceled,
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
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Canceled"),
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Deleting"),
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
#[doc = "Specifies which access keys to reset to a new random value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegenerateKeyParameters {
    #[doc = "Which access key to regenerate."]
    #[serde(rename = "keyType")]
    pub key_type: regenerate_key_parameters::KeyType,
}
impl RegenerateKeyParameters {
    pub fn new(key_type: regenerate_key_parameters::KeyType) -> Self {
        Self { key_type }
    }
}
pub mod regenerate_key_parameters {
    use super::*;
    #[doc = "Which access key to regenerate."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyType {
        Primary,
        Secondary,
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
#[doc = "Current resource status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ResourceState")]
pub enum ResourceState {
    Running,
    Creating,
    CreateFailed,
    Updating,
    UpdateFailed,
    Deleting,
    DeleteFailed,
    Enabling,
    EnableFailed,
    Disabling,
    DisableFailed,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ResourceState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ResourceState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ResourceState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Running => serializer.serialize_unit_variant("ResourceState", 0u32, "Running"),
            Self::Creating => serializer.serialize_unit_variant("ResourceState", 1u32, "Creating"),
            Self::CreateFailed => serializer.serialize_unit_variant("ResourceState", 2u32, "CreateFailed"),
            Self::Updating => serializer.serialize_unit_variant("ResourceState", 3u32, "Updating"),
            Self::UpdateFailed => serializer.serialize_unit_variant("ResourceState", 4u32, "UpdateFailed"),
            Self::Deleting => serializer.serialize_unit_variant("ResourceState", 5u32, "Deleting"),
            Self::DeleteFailed => serializer.serialize_unit_variant("ResourceState", 6u32, "DeleteFailed"),
            Self::Enabling => serializer.serialize_unit_variant("ResourceState", 7u32, "Enabling"),
            Self::EnableFailed => serializer.serialize_unit_variant("ResourceState", 8u32, "EnableFailed"),
            Self::Disabling => serializer.serialize_unit_variant("ResourceState", 9u32, "Disabling"),
            Self::DisableFailed => serializer.serialize_unit_variant("ResourceState", 10u32, "DisableFailed"),
            Self::Disabled => serializer.serialize_unit_variant("ResourceState", 11u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "SKU parameters supplied to the create RedisEnterprise operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The type of RedisEnterprise cluster to deploy. Possible values: (Enterprise_E10, EnterpriseFlash_F300 etc.)"]
    pub name: sku::Name,
    #[doc = "The size of the RedisEnterprise cluster. Defaults to 2 or 3 depending on SKU. Valid values are (2, 4, 6, ...) for Enterprise SKUs and (3, 9, 15, ...) for Flash SKUs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl Sku {
    pub fn new(name: sku::Name) -> Self {
        Self { name, capacity: None }
    }
}
pub mod sku {
    use super::*;
    #[doc = "The type of RedisEnterprise cluster to deploy. Possible values: (Enterprise_E10, EnterpriseFlash_F300 etc.)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        #[serde(rename = "Enterprise_E10")]
        EnterpriseE10,
        #[serde(rename = "Enterprise_E20")]
        EnterpriseE20,
        #[serde(rename = "Enterprise_E50")]
        EnterpriseE50,
        #[serde(rename = "Enterprise_E100")]
        EnterpriseE100,
        #[serde(rename = "EnterpriseFlash_F300")]
        EnterpriseFlashF300,
        #[serde(rename = "EnterpriseFlash_F700")]
        EnterpriseFlashF700,
        #[serde(rename = "EnterpriseFlash_F1500")]
        EnterpriseFlashF1500,
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
                Self::EnterpriseE10 => serializer.serialize_unit_variant("Name", 0u32, "Enterprise_E10"),
                Self::EnterpriseE20 => serializer.serialize_unit_variant("Name", 1u32, "Enterprise_E20"),
                Self::EnterpriseE50 => serializer.serialize_unit_variant("Name", 2u32, "Enterprise_E50"),
                Self::EnterpriseE100 => serializer.serialize_unit_variant("Name", 3u32, "Enterprise_E100"),
                Self::EnterpriseFlashF300 => serializer.serialize_unit_variant("Name", 4u32, "EnterpriseFlash_F300"),
                Self::EnterpriseFlashF700 => serializer.serialize_unit_variant("Name", 5u32, "EnterpriseFlash_F700"),
                Self::EnterpriseFlashF1500 => serializer.serialize_unit_variant("Name", 6u32, "EnterpriseFlash_F1500"),
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
