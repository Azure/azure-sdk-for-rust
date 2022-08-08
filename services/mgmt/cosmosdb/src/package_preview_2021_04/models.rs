#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "The resource model definition for a ARM proxy resource. It will have everything other than required location and tags"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmProxyResource {
    #[doc = "The unique resource identifier of the database account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the database account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of Azure resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ArmProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The core properties of ARM resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmResourceProperties {
    #[doc = "The unique resource identifier of the ARM resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the ARM resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of Azure resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The location of the resource group to which the resource belongs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Tags are a list of key-value pairs that describe the resource. These tags can be used in viewing and grouping this resource (across resource groups). A maximum of 15 tags can be provided for a resource. Each tag must have a key no greater than 128 characters and value no greater than 256 characters. For example, the default experience for a template type is set with \"defaultExperience\": \"Cassandra\". Current \"defaultExperience\" values also include \"Table\", \"Graph\", \"DocumentDB\", and \"MongoDB\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl ArmResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiProperties {
    #[doc = "Describes the ServerVersion of an a MongoDB account."]
    #[serde(rename = "serverVersion", default, skip_serializing_if = "Option::is_none")]
    pub server_version: Option<api_properties::ServerVersion>,
}
impl ApiProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod api_properties {
    use super::*;
    #[doc = "Describes the ServerVersion of an a MongoDB account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ServerVersion")]
    pub enum ServerVersion {
        #[serde(rename = "3.2")]
        N3_2,
        #[serde(rename = "3.6")]
        N3_6,
        #[serde(rename = "4.0")]
        N4_0,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ServerVersion {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ServerVersion {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ServerVersion {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::N3_2 => serializer.serialize_unit_variant("ServerVersion", 0u32, "3.2"),
                Self::N3_6 => serializer.serialize_unit_variant("ServerVersion", 1u32, "3.6"),
                Self::N4_0 => serializer.serialize_unit_variant("ServerVersion", 2u32, "4.0"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Enum to indicate the API type of the restorable database account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ApiType")]
pub enum ApiType {
    #[serde(rename = "MongoDB")]
    MongoDb,
    Gremlin,
    Cassandra,
    Table,
    Sql,
    GremlinV2,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ApiType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ApiType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ApiType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::MongoDb => serializer.serialize_unit_variant("ApiType", 0u32, "MongoDB"),
            Self::Gremlin => serializer.serialize_unit_variant("ApiType", 1u32, "Gremlin"),
            Self::Cassandra => serializer.serialize_unit_variant("ApiType", 2u32, "Cassandra"),
            Self::Table => serializer.serialize_unit_variant("ApiType", 3u32, "Table"),
            Self::Sql => serializer.serialize_unit_variant("ApiType", 4u32, "Sql"),
            Self::GremlinV2 => serializer.serialize_unit_variant("ApiType", 5u32, "GremlinV2"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Cosmos DB resource auto-upgrade policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoUpgradePolicyResource {
    #[doc = "Cosmos DB resource throughput policy"]
    #[serde(rename = "throughputPolicy", default, skip_serializing_if = "Option::is_none")]
    pub throughput_policy: Option<ThroughputPolicyResource>,
}
impl AutoUpgradePolicyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoscaleSettings {
    #[doc = "Represents maximum throughput, the resource can scale up to."]
    #[serde(rename = "maxThroughput", default, skip_serializing_if = "Option::is_none")]
    pub max_throughput: Option<i64>,
}
impl AutoscaleSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cosmos DB provisioned throughput settings object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleSettingsResource {
    #[doc = "Represents maximum throughput container can scale up to."]
    #[serde(rename = "maxThroughput")]
    pub max_throughput: i64,
    #[doc = "Cosmos DB resource auto-upgrade policy"]
    #[serde(rename = "autoUpgradePolicy", default, skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_policy: Option<AutoUpgradePolicyResource>,
    #[doc = "Represents target maximum throughput container can scale up to once offer is no longer in pending state."]
    #[serde(rename = "targetMaxThroughput", default, skip_serializing_if = "Option::is_none")]
    pub target_max_throughput: Option<i64>,
}
impl AutoscaleSettingsResource {
    pub fn new(max_throughput: i64) -> Self {
        Self {
            max_throughput,
            auto_upgrade_policy: None,
            target_max_throughput: None,
        }
    }
}
#[doc = "Backup information of a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupInformation {
    #[doc = "Continuous backup description."]
    #[serde(rename = "continuousBackupInformation", default, skip_serializing_if = "Option::is_none")]
    pub continuous_backup_information: Option<ContinuousBackupInformation>,
}
impl BackupInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The object representing the policy for taking backups on an account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackupPolicy {
    #[doc = "Describes the mode of backups."]
    #[serde(rename = "type")]
    pub type_: BackupPolicyType,
}
impl BackupPolicy {
    pub fn new(type_: BackupPolicyType) -> Self {
        Self { type_ }
    }
}
#[doc = "Describes the mode of backups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BackupPolicyType")]
pub enum BackupPolicyType {
    Periodic,
    Continuous,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BackupPolicyType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BackupPolicyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BackupPolicyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Periodic => serializer.serialize_unit_variant("BackupPolicyType", 0u32, "Periodic"),
            Self::Continuous => serializer.serialize_unit_variant("BackupPolicyType", 1u32, "Continuous"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A restorable backup of a Cassandra cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupResource {
    #[serde(flatten)]
    pub arm_proxy_resource: ArmProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<backup_resource::Properties>,
}
impl BackupResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod backup_resource {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The time this backup was taken, formatted like 2021-01-21T17:35:21"]
        #[serde(with = "azure_core::date::rfc3339::option")]
        pub timestamp: Option<time::OffsetDateTime>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Enum to indicate type of backup storage redundancy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "BackupStorageRedundancy")]
pub enum BackupStorageRedundancy {
    Geo,
    Local,
    Zone,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for BackupStorageRedundancy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for BackupStorageRedundancy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for BackupStorageRedundancy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Geo => serializer.serialize_unit_variant("BackupStorageRedundancy", 0u32, "Geo"),
            Self::Local => serializer.serialize_unit_variant("BackupStorageRedundancy", 1u32, "Local"),
            Self::Zone => serializer.serialize_unit_variant("BackupStorageRedundancy", 2u32, "Zone"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Cosmos DB capability object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Capability {
    #[doc = "Name of the Cosmos DB capability. For example, \"name\": \"EnableCassandra\". Current values also include \"EnableTable\" and \"EnableGremlin\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl Capability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters to create and update Cosmos DB Cassandra keyspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CassandraKeyspaceCreateUpdateParameters {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "Properties to create and update Azure Cosmos DB Cassandra keyspace."]
    pub properties: CassandraKeyspaceCreateUpdateProperties,
}
impl CassandraKeyspaceCreateUpdateParameters {
    pub fn new(properties: CassandraKeyspaceCreateUpdateProperties) -> Self {
        Self {
            arm_resource_properties: ArmResourceProperties::default(),
            properties,
        }
    }
}
#[doc = "Properties to create and update Azure Cosmos DB Cassandra keyspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CassandraKeyspaceCreateUpdateProperties {
    #[doc = "Cosmos DB Cassandra keyspace resource object"]
    pub resource: CassandraKeyspaceResource,
    #[doc = "CreateUpdateOptions are a list of key-value pairs that describe the resource. Supported keys are \"If-Match\", \"If-None-Match\", \"Session-Token\" and \"Throughput\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<CreateUpdateOptions>,
}
impl CassandraKeyspaceCreateUpdateProperties {
    pub fn new(resource: CassandraKeyspaceResource) -> Self {
        Self { resource, options: None }
    }
}
#[doc = "The properties of an Azure Cosmos DB Cassandra keyspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CassandraKeyspaceGetProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}
impl CassandraKeyspaceGetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Cosmos DB Cassandra keyspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CassandraKeyspaceGetResults {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "The properties of an Azure Cosmos DB Cassandra keyspace"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CassandraKeyspaceGetProperties>,
}
impl CassandraKeyspaceGetResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List operation response, that contains the Cassandra keyspaces and their properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CassandraKeyspaceListResult {
    #[doc = "List of Cassandra keyspaces and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CassandraKeyspaceGetResults>,
}
impl azure_core::Continuable for CassandraKeyspaceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl CassandraKeyspaceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cosmos DB Cassandra keyspace resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CassandraKeyspaceResource {
    #[doc = "Name of the Cosmos DB Cassandra keyspace"]
    pub id: String,
}
impl CassandraKeyspaceResource {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "Cosmos DB Cassandra table partition key"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CassandraPartitionKey {
    #[doc = "Name of the Cosmos DB Cassandra table partition key"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl CassandraPartitionKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cosmos DB Cassandra table schema"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CassandraSchema {
    #[doc = "List of Cassandra table columns."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<Column>,
    #[doc = "List of partition key."]
    #[serde(rename = "partitionKeys", default, skip_serializing_if = "Vec::is_empty")]
    pub partition_keys: Vec<CassandraPartitionKey>,
    #[doc = "List of cluster key."]
    #[serde(rename = "clusterKeys", default, skip_serializing_if = "Vec::is_empty")]
    pub cluster_keys: Vec<ClusterKey>,
}
impl CassandraSchema {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters to create and update Cosmos DB Cassandra table."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CassandraTableCreateUpdateParameters {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "Properties to create and update Azure Cosmos DB Cassandra table."]
    pub properties: CassandraTableCreateUpdateProperties,
}
impl CassandraTableCreateUpdateParameters {
    pub fn new(properties: CassandraTableCreateUpdateProperties) -> Self {
        Self {
            arm_resource_properties: ArmResourceProperties::default(),
            properties,
        }
    }
}
#[doc = "Properties to create and update Azure Cosmos DB Cassandra table."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CassandraTableCreateUpdateProperties {
    #[doc = "Cosmos DB Cassandra table resource object"]
    pub resource: CassandraTableResource,
    #[doc = "CreateUpdateOptions are a list of key-value pairs that describe the resource. Supported keys are \"If-Match\", \"If-None-Match\", \"Session-Token\" and \"Throughput\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<CreateUpdateOptions>,
}
impl CassandraTableCreateUpdateProperties {
    pub fn new(resource: CassandraTableResource) -> Self {
        Self { resource, options: None }
    }
}
#[doc = "The properties of an Azure Cosmos DB Cassandra table"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CassandraTableGetProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}
impl CassandraTableGetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Cosmos DB Cassandra table."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CassandraTableGetResults {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "The properties of an Azure Cosmos DB Cassandra table"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CassandraTableGetProperties>,
}
impl CassandraTableGetResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List operation response, that contains the Cassandra tables and their properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CassandraTableListResult {
    #[doc = "List of Cassandra tables and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CassandraTableGetResults>,
}
impl azure_core::Continuable for CassandraTableListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl CassandraTableListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cosmos DB Cassandra table resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CassandraTableResource {
    #[doc = "Name of the Cosmos DB Cassandra table"]
    pub id: String,
    #[doc = "Time to live of the Cosmos DB Cassandra table"]
    #[serde(rename = "defaultTtl", default, skip_serializing_if = "Option::is_none")]
    pub default_ttl: Option<i64>,
    #[doc = "Cosmos DB Cassandra table schema"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<CassandraSchema>,
    #[doc = "Analytical TTL."]
    #[serde(rename = "analyticalStorageTtl", default, skip_serializing_if = "Option::is_none")]
    pub analytical_storage_ttl: Option<i64>,
}
impl CassandraTableResource {
    pub fn new(id: String) -> Self {
        Self {
            id,
            default_ttl: None,
            schema: None,
            analytical_storage_ttl: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Certificate {
    #[doc = "PEM formatted public key."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pem: Option<String>,
}
impl Certificate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "Error Response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
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
#[doc = "Cosmos DB Cassandra table cluster key"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterKey {
    #[doc = "Name of the Cosmos DB Cassandra table cluster key"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Order of the Cosmos DB Cassandra table cluster key, only support \"Asc\" and \"Desc\""]
    #[serde(rename = "orderBy", default, skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
}
impl ClusterKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of all nodes in the cluster (as returned by 'nodetool status')."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterNodeStatus {
    #[doc = "Information about nodes in the cluster (corresponds to what is returned from nodetool info)."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<serde_json::Value>,
}
impl ClusterNodeStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Representation of a managed Cassandra cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterResource {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "Properties of a managed Cassandra cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<cluster_resource::Properties>,
}
impl ClusterResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cluster_resource {
    use super::*;
    #[doc = "Properties of a managed Cassandra cluster."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The status of the resource at the time the operation was called."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<ManagedCassandraProvisioningState>,
        #[doc = "To create an empty cluster, omit this field or set it to null. To restore a backup into a new cluster, set this field to the resource id of the backup."]
        #[serde(rename = "restoreFromBackupId", default, skip_serializing_if = "Option::is_none")]
        pub restore_from_backup_id: Option<String>,
        #[doc = "Resource id of a subnet that this cluster's management service should have its network interface attached to. The subnet must be routable to all subnets that will be delegated to data centers. The resource id must be of the form '/subscriptions/<subscription id>/resourceGroups/<resource group>/providers/Microsoft.Network/virtualNetworks/<virtual network>/subnets/<subnet>'"]
        #[serde(rename = "delegatedManagementSubnetId", default, skip_serializing_if = "Option::is_none")]
        pub delegated_management_subnet_id: Option<String>,
        #[doc = "Which version of Cassandra should this cluster converge to running (e.g., 3.11). When updated, the cluster may take some time to migrate to the new version."]
        #[serde(rename = "cassandraVersion", default, skip_serializing_if = "Option::is_none")]
        pub cassandra_version: Option<String>,
        #[doc = "If you need to set the clusterName property in cassandra.yaml to something besides the resource name of the cluster, set the value to use on this property."]
        #[serde(rename = "clusterNameOverride", default, skip_serializing_if = "Option::is_none")]
        pub cluster_name_override: Option<String>,
        #[doc = "Which authentication method Cassandra should use to authenticate clients. 'None' turns off authentication, so should not be used except in emergencies. 'Cassandra' is the default password based authentication. The default is 'Cassandra'."]
        #[serde(rename = "authenticationMethod", default, skip_serializing_if = "Option::is_none")]
        pub authentication_method: Option<properties::AuthenticationMethod>,
        #[doc = "Initial password for clients connecting as admin to the cluster. Should be changed after cluster creation. Returns null on GET. This field only applies when the authenticationMethod field is 'Cassandra'."]
        #[serde(rename = "initialCassandraAdminPassword", default, skip_serializing_if = "Option::is_none")]
        pub initial_cassandra_admin_password: Option<String>,
        #[doc = "Number of hours to wait between taking a backup of the cluster. To disable backups, set this property to 0."]
        #[serde(rename = "hoursBetweenBackups", default, skip_serializing_if = "Option::is_none")]
        pub hours_between_backups: Option<i32>,
        #[serde(rename = "prometheusEndpoint", default, skip_serializing_if = "Option::is_none")]
        pub prometheus_endpoint: Option<SeedNode>,
        #[doc = "Should automatic repairs run on this cluster? If omitted, this is true, and should stay true unless you are running a hybrid cluster where you are already doing your own repairs."]
        #[serde(rename = "repairEnabled", default, skip_serializing_if = "Option::is_none")]
        pub repair_enabled: Option<bool>,
        #[doc = "List of TLS certificates used to authorize clients connecting to the cluster. All connections are TLS encrypted whether clientCertificates is set or not, but if clientCertificates is set, the managed Cassandra cluster will reject all connections not bearing a TLS client certificate that can be validated from one or more of the public certificates in this property."]
        #[serde(rename = "clientCertificates", default, skip_serializing_if = "Vec::is_empty")]
        pub client_certificates: Vec<Certificate>,
        #[doc = "List of TLS certificates used to authorize gossip from unmanaged data centers. The TLS certificates of all nodes in unmanaged data centers must be verifiable using one of the certificates provided in this property."]
        #[serde(rename = "externalGossipCertificates", default, skip_serializing_if = "Vec::is_empty")]
        pub external_gossip_certificates: Vec<Certificate>,
        #[doc = "List of TLS certificates that unmanaged nodes must trust for gossip with managed nodes. All managed nodes will present TLS client certificates that are verifiable using one of the certificates provided in this property."]
        #[serde(rename = "gossipCertificates", default, skip_serializing_if = "Vec::is_empty")]
        pub gossip_certificates: Vec<Certificate>,
        #[doc = "List of IP addresses of seed nodes in unmanaged data centers. These will be added to the seed node lists of all managed nodes."]
        #[serde(rename = "externalSeedNodes", default, skip_serializing_if = "Vec::is_empty")]
        pub external_seed_nodes: Vec<SeedNode>,
        #[doc = "List of IP addresses of seed nodes in the managed data centers. These should be added to the seed node lists of all unmanaged nodes."]
        #[serde(rename = "seedNodes", default, skip_serializing_if = "Vec::is_empty")]
        pub seed_nodes: Vec<SeedNode>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Which authentication method Cassandra should use to authenticate clients. 'None' turns off authentication, so should not be used except in emergencies. 'Cassandra' is the default password based authentication. The default is 'Cassandra'."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "AuthenticationMethod")]
        pub enum AuthenticationMethod {
            None,
            Cassandra,
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
                    Self::None => serializer.serialize_unit_variant("AuthenticationMethod", 0u32, "None"),
                    Self::Cassandra => serializer.serialize_unit_variant("AuthenticationMethod", 1u32, "Cassandra"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
pub type CollectionName = String;
#[doc = "Cosmos DB Cassandra table column"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Column {
    #[doc = "Name of the Cosmos DB Cassandra table column"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the Cosmos DB Cassandra table column"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Column {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CompositePath {
    #[doc = "The path for which the indexing behavior applies to. Index paths typically start with root and end with wildcard (/path/*)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Sort order for composite paths."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<composite_path::Order>,
}
impl CompositePath {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod composite_path {
    use super::*;
    #[doc = "Sort order for composite paths."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Order")]
    pub enum Order {
        #[serde(rename = "ascending")]
        Ascending,
        #[serde(rename = "descending")]
        Descending,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Order {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Order {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Order {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Ascending => serializer.serialize_unit_variant("Order", 0u32, "ascending"),
                Self::Descending => serializer.serialize_unit_variant("Order", 1u32, "descending"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
pub type CompositePathList = Vec<CompositePath>;
#[doc = "The conflict resolution policy for the container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConflictResolutionPolicy {
    #[doc = "Indicates the conflict resolution mode."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<conflict_resolution_policy::Mode>,
    #[doc = "The conflict resolution path in the case of LastWriterWins mode."]
    #[serde(rename = "conflictResolutionPath", default, skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_path: Option<String>,
    #[doc = "The procedure to resolve conflicts in the case of custom mode."]
    #[serde(rename = "conflictResolutionProcedure", default, skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_procedure: Option<String>,
}
impl ConflictResolutionPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod conflict_resolution_policy {
    use super::*;
    #[doc = "Indicates the conflict resolution mode."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Mode")]
    pub enum Mode {
        LastWriterWins,
        Custom,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Mode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Mode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Mode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::LastWriterWins => serializer.serialize_unit_variant("Mode", 0u32, "LastWriterWins"),
                Self::Custom => serializer.serialize_unit_variant("Mode", 1u32, "Custom"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Mode {
        fn default() -> Self {
            Self::LastWriterWins
        }
    }
}
#[doc = "The cassandra connector offer type for the Cosmos DB C* database account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ConnectorOffer")]
pub enum ConnectorOffer {
    Small,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ConnectorOffer {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ConnectorOffer {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ConnectorOffer {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Small => serializer.serialize_unit_variant("ConnectorOffer", 0u32, "Small"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The consistency policy for the Cosmos DB database account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsistencyPolicy {
    #[doc = "The default consistency level and configuration settings of the Cosmos DB account."]
    #[serde(rename = "defaultConsistencyLevel")]
    pub default_consistency_level: consistency_policy::DefaultConsistencyLevel,
    #[doc = "When used with the Bounded Staleness consistency level, this value represents the number of stale requests tolerated. Accepted range for this value is 1 – 2,147,483,647. Required when defaultConsistencyPolicy is set to 'BoundedStaleness'."]
    #[serde(rename = "maxStalenessPrefix", default, skip_serializing_if = "Option::is_none")]
    pub max_staleness_prefix: Option<i64>,
    #[doc = "When used with the Bounded Staleness consistency level, this value represents the time amount of staleness (in seconds) tolerated. Accepted range for this value is 5 - 86400. Required when defaultConsistencyPolicy is set to 'BoundedStaleness'."]
    #[serde(rename = "maxIntervalInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub max_interval_in_seconds: Option<i32>,
}
impl ConsistencyPolicy {
    pub fn new(default_consistency_level: consistency_policy::DefaultConsistencyLevel) -> Self {
        Self {
            default_consistency_level,
            max_staleness_prefix: None,
            max_interval_in_seconds: None,
        }
    }
}
pub mod consistency_policy {
    use super::*;
    #[doc = "The default consistency level and configuration settings of the Cosmos DB account."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DefaultConsistencyLevel {
        Eventual,
        Session,
        BoundedStaleness,
        Strong,
        ConsistentPrefix,
    }
}
#[doc = "The configuration of the partition key to be used for partitioning data into multiple partitions"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerPartitionKey {
    #[doc = "List of paths using which data within the container can be partitioned"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<Path>,
    #[doc = "Indicates the kind of algorithm used for partitioning. For MultiHash, multiple partition keys (upto three maximum) are supported for container create"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<container_partition_key::Kind>,
    #[doc = "Indicates the version of the partition key definition"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[doc = "Indicates if the container is using a system generated partition key"]
    #[serde(rename = "systemKey", default, skip_serializing_if = "Option::is_none")]
    pub system_key: Option<bool>,
}
impl ContainerPartitionKey {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod container_partition_key {
    use super::*;
    #[doc = "Indicates the kind of algorithm used for partitioning. For MultiHash, multiple partition keys (upto three maximum) are supported for container create"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Hash,
        Range,
        MultiHash,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Hash => serializer.serialize_unit_variant("Kind", 0u32, "Hash"),
                Self::Range => serializer.serialize_unit_variant("Kind", 1u32, "Range"),
                Self::MultiHash => serializer.serialize_unit_variant("Kind", 2u32, "MultiHash"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Kind {
        fn default() -> Self {
            Self::Hash
        }
    }
}
#[doc = "Continuous backup description."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContinuousBackupInformation {
    #[doc = "The latest restorable timestamp for a resource."]
    #[serde(rename = "latestRestorableTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub latest_restorable_timestamp: Option<String>,
}
impl ContinuousBackupInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the regional restorable account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContinuousBackupRestoreLocation {
    #[doc = "The name of the continuous backup restore location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl ContinuousBackupRestoreLocation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The object representing continuous mode backup policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContinuousModeBackupPolicy {
    #[serde(flatten)]
    pub backup_policy: BackupPolicy,
}
impl ContinuousModeBackupPolicy {
    pub fn new(backup_policy: BackupPolicy) -> Self {
        Self { backup_policy }
    }
}
#[doc = "The CORS policy for the Cosmos DB database account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CorsPolicy {
    #[doc = "The origin domains that are permitted to make a request against the service via CORS."]
    #[serde(rename = "allowedOrigins")]
    pub allowed_origins: String,
    #[doc = "The methods (HTTP request verbs) that the origin domain may use for a CORS request."]
    #[serde(rename = "allowedMethods", default, skip_serializing_if = "Option::is_none")]
    pub allowed_methods: Option<String>,
    #[doc = "The request headers that the origin domain may specify on the CORS request."]
    #[serde(rename = "allowedHeaders", default, skip_serializing_if = "Option::is_none")]
    pub allowed_headers: Option<String>,
    #[doc = "The response headers that may be sent in the response to the CORS request and exposed by the browser to the request issuer."]
    #[serde(rename = "exposedHeaders", default, skip_serializing_if = "Option::is_none")]
    pub exposed_headers: Option<String>,
    #[doc = "The maximum amount time that a browser should cache the preflight OPTIONS request."]
    #[serde(rename = "maxAgeInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub max_age_in_seconds: Option<i64>,
}
impl CorsPolicy {
    pub fn new(allowed_origins: String) -> Self {
        Self {
            allowed_origins,
            allowed_methods: None,
            allowed_headers: None,
            exposed_headers: None,
            max_age_in_seconds: None,
        }
    }
}
#[doc = "Enum to indicate the mode of account creation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CreateMode")]
pub enum CreateMode {
    Default,
    Restore,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CreateMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CreateMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CreateMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Default => serializer.serialize_unit_variant("CreateMode", 0u32, "Default"),
            Self::Restore => serializer.serialize_unit_variant("CreateMode", 1u32, "Restore"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
impl Default for CreateMode {
    fn default() -> Self {
        Self::Default
    }
}
#[doc = "CreateUpdateOptions are a list of key-value pairs that describe the resource. Supported keys are \"If-Match\", \"If-None-Match\", \"Session-Token\" and \"Throughput\""]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateUpdateOptions {
    #[doc = "Request Units per second. For example, \"throughput\": 10000."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throughput: Option<i64>,
    #[serde(rename = "autoscaleSettings", default, skip_serializing_if = "Option::is_none")]
    pub autoscale_settings: Option<AutoscaleSettings>,
}
impl CreateUpdateOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A managed Cassandra data center."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataCenterResource {
    #[serde(flatten)]
    pub arm_proxy_resource: ArmProxyResource,
    #[doc = "Properties of a managed Cassandra data center."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<data_center_resource::Properties>,
}
impl DataCenterResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_center_resource {
    use super::*;
    #[doc = "Properties of a managed Cassandra data center."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The status of the resource at the time the operation was called."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<ManagedCassandraProvisioningState>,
        #[doc = "The region this data center should be created in."]
        #[serde(rename = "dataCenterLocation", default, skip_serializing_if = "Option::is_none")]
        pub data_center_location: Option<String>,
        #[doc = "Resource id of a subnet the nodes in this data center should have their network interfaces connected to. The subnet must be in the same region specified in 'dataCenterLocation' and must be able to route to the subnet specified in the cluster's 'delegatedManagementSubnetId' property. This resource id will be of the form '/subscriptions/<subscription id>/resourceGroups/<resource group>/providers/Microsoft.Network/virtualNetworks/<virtual network>/subnets/<subnet>'."]
        #[serde(rename = "delegatedSubnetId", default, skip_serializing_if = "Option::is_none")]
        pub delegated_subnet_id: Option<String>,
        #[doc = "The number of nodes the data center should have. This is the desired number. After it is set, it may take some time for the data center to be scaled to match. To monitor the number of nodes and their status, use the fetchNodeStatus method on the cluster."]
        #[serde(rename = "nodeCount", default, skip_serializing_if = "Option::is_none")]
        pub node_count: Option<i32>,
        #[doc = "IP addresses for seed nodes in this data center. This is for reference. Generally you will want to use the seedNodes property on the cluster, which aggregates the seed nodes from all data centers in the cluster."]
        #[serde(rename = "seedNodes", default, skip_serializing_if = "Vec::is_empty")]
        pub seed_nodes: Vec<SeedNode>,
        #[doc = "A fragment of a cassandra.yaml configuration file to be included in the cassandra.yaml for all nodes in this data center. The fragment should be Base64 encoded, and only a subset of keys are allowed."]
        #[serde(rename = "base64EncodedCassandraYamlFragment", default, skip_serializing_if = "Option::is_none")]
        pub base64_encoded_cassandra_yaml_fragment: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Connection string for the Cosmos DB account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseAccountConnectionString {
    #[doc = "Value of the connection string"]
    #[serde(rename = "connectionString", default, skip_serializing_if = "Option::is_none")]
    pub connection_string: Option<String>,
    #[doc = "Description of the connection string"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl DatabaseAccountConnectionString {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters to create and update Cosmos DB database accounts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseAccountCreateUpdateParameters {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "Indicates the type of database account. This can only be set at database account creation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<database_account_create_update_parameters::Kind>,
    #[doc = "Properties to create and update Azure Cosmos DB database accounts."]
    pub properties: DatabaseAccountCreateUpdateProperties,
}
impl DatabaseAccountCreateUpdateParameters {
    pub fn new(properties: DatabaseAccountCreateUpdateProperties) -> Self {
        Self {
            arm_resource_properties: ArmResourceProperties::default(),
            kind: None,
            properties,
        }
    }
}
pub mod database_account_create_update_parameters {
    use super::*;
    #[doc = "Indicates the type of database account. This can only be set at database account creation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        #[serde(rename = "GlobalDocumentDB")]
        GlobalDocumentDb,
        #[serde(rename = "MongoDB")]
        MongoDb,
        Parse,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::GlobalDocumentDb => serializer.serialize_unit_variant("Kind", 0u32, "GlobalDocumentDB"),
                Self::MongoDb => serializer.serialize_unit_variant("Kind", 1u32, "MongoDB"),
                Self::Parse => serializer.serialize_unit_variant("Kind", 2u32, "Parse"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Kind {
        fn default() -> Self {
            Self::GlobalDocumentDb
        }
    }
}
#[doc = "Properties to create and update Azure Cosmos DB database accounts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseAccountCreateUpdateProperties {
    #[doc = "The consistency policy for the Cosmos DB database account."]
    #[serde(rename = "consistencyPolicy", default, skip_serializing_if = "Option::is_none")]
    pub consistency_policy: Option<ConsistencyPolicy>,
    #[doc = "An array that contains the georeplication locations enabled for the Cosmos DB account."]
    pub locations: Vec<Location>,
    #[doc = "The offer type for the Cosmos DB database account."]
    #[serde(rename = "databaseAccountOfferType")]
    pub database_account_offer_type: DatabaseAccountOfferType,
    #[doc = "Array of IpAddressOrRange objects."]
    #[serde(rename = "ipRules", default, skip_serializing_if = "Option::is_none")]
    pub ip_rules: Option<IpRules>,
    #[doc = "Flag to indicate whether to enable/disable Virtual Network ACL rules."]
    #[serde(rename = "isVirtualNetworkFilterEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_virtual_network_filter_enabled: Option<bool>,
    #[doc = "Enables automatic failover of the write region in the rare event that the region is unavailable due to an outage. Automatic failover will result in a new write region for the account and is chosen based on the failover priorities configured for the account."]
    #[serde(rename = "enableAutomaticFailover", default, skip_serializing_if = "Option::is_none")]
    pub enable_automatic_failover: Option<bool>,
    #[doc = "List of Cosmos DB capabilities for the account"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<Capability>,
    #[doc = "List of Virtual Network ACL rules configured for the Cosmos DB account."]
    #[serde(rename = "virtualNetworkRules", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_network_rules: Vec<VirtualNetworkRule>,
    #[doc = "Enables the account to write in multiple locations"]
    #[serde(rename = "enableMultipleWriteLocations", default, skip_serializing_if = "Option::is_none")]
    pub enable_multiple_write_locations: Option<bool>,
    #[doc = "Enables the cassandra connector on the Cosmos DB C* account"]
    #[serde(rename = "enableCassandraConnector", default, skip_serializing_if = "Option::is_none")]
    pub enable_cassandra_connector: Option<bool>,
    #[doc = "The cassandra connector offer type for the Cosmos DB C* database account."]
    #[serde(rename = "connectorOffer", default, skip_serializing_if = "Option::is_none")]
    pub connector_offer: Option<ConnectorOffer>,
    #[doc = "Disable write operations on metadata resources (databases, containers, throughput) via account keys"]
    #[serde(rename = "disableKeyBasedMetadataWriteAccess", default, skip_serializing_if = "Option::is_none")]
    pub disable_key_based_metadata_write_access: Option<bool>,
    #[doc = "The URI of the key vault"]
    #[serde(rename = "keyVaultKeyUri", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_key_uri: Option<String>,
    #[doc = "The default identity for accessing key vault used in features like customer managed keys. The default identity needs to be explicitly set by the users. It can be \"FirstPartyIdentity\", \"SystemAssignedIdentity\" and more."]
    #[serde(rename = "defaultIdentity", default, skip_serializing_if = "Option::is_none")]
    pub default_identity: Option<String>,
    #[doc = "Whether requests from Public Network are allowed"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccess>,
    #[doc = "Flag to indicate whether Free Tier is enabled."]
    #[serde(rename = "enableFreeTier", default, skip_serializing_if = "Option::is_none")]
    pub enable_free_tier: Option<bool>,
    #[serde(rename = "apiProperties", default, skip_serializing_if = "Option::is_none")]
    pub api_properties: Option<ApiProperties>,
    #[doc = "Flag to indicate whether to enable storage analytics."]
    #[serde(rename = "enableAnalyticalStorage", default, skip_serializing_if = "Option::is_none")]
    pub enable_analytical_storage: Option<bool>,
    #[doc = "Enum to indicate the mode of account creation."]
    #[serde(rename = "createMode")]
    pub create_mode: CreateMode,
    #[doc = "The object representing the policy for taking backups on an account."]
    #[serde(rename = "backupPolicy", default, skip_serializing_if = "Option::is_none")]
    pub backup_policy: Option<BackupPolicy>,
    #[doc = "The CORS policy for the Cosmos DB database account."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cors: Vec<CorsPolicy>,
    #[doc = "Indicates what services are allowed to bypass firewall checks."]
    #[serde(rename = "networkAclBypass", default, skip_serializing_if = "Option::is_none")]
    pub network_acl_bypass: Option<NetworkAclBypass>,
    #[doc = "An array that contains the Resource Ids for Network Acl Bypass for the Cosmos DB account."]
    #[serde(rename = "networkAclBypassResourceIds", default, skip_serializing_if = "Vec::is_empty")]
    pub network_acl_bypass_resource_ids: Vec<String>,
}
impl DatabaseAccountCreateUpdateProperties {
    pub fn new(locations: Vec<Location>, database_account_offer_type: DatabaseAccountOfferType, create_mode: CreateMode) -> Self {
        Self {
            consistency_policy: None,
            locations,
            database_account_offer_type,
            ip_rules: None,
            is_virtual_network_filter_enabled: None,
            enable_automatic_failover: None,
            capabilities: Vec::new(),
            virtual_network_rules: Vec::new(),
            enable_multiple_write_locations: None,
            enable_cassandra_connector: None,
            connector_offer: None,
            disable_key_based_metadata_write_access: None,
            key_vault_key_uri: None,
            default_identity: None,
            public_network_access: None,
            enable_free_tier: None,
            api_properties: None,
            enable_analytical_storage: None,
            create_mode,
            backup_policy: None,
            cors: Vec::new(),
            network_acl_bypass: None,
            network_acl_bypass_resource_ids: Vec::new(),
        }
    }
}
#[doc = "Properties for the database account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseAccountGetProperties {
    #[doc = "The status of the Cosmos DB account at the time the operation was called. The status can be one of following. 'Creating' – the Cosmos DB account is being created. When an account is in Creating state, only properties that are specified as input for the Create Cosmos DB account operation are returned. 'Succeeded' – the Cosmos DB account is active for use. 'Updating' – the Cosmos DB account is being updated. 'Deleting' – the Cosmos DB account is being deleted. 'Failed' – the Cosmos DB account failed creation. 'DeletionFailed' – the Cosmos DB account deletion failed."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The connection endpoint for the Cosmos DB database account."]
    #[serde(rename = "documentEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub document_endpoint: Option<String>,
    #[doc = "The offer type for the Cosmos DB database account."]
    #[serde(rename = "databaseAccountOfferType", default, skip_serializing_if = "Option::is_none")]
    pub database_account_offer_type: Option<DatabaseAccountOfferType>,
    #[doc = "Array of IpAddressOrRange objects."]
    #[serde(rename = "ipRules", default, skip_serializing_if = "Option::is_none")]
    pub ip_rules: Option<IpRules>,
    #[doc = "Flag to indicate whether to enable/disable Virtual Network ACL rules."]
    #[serde(rename = "isVirtualNetworkFilterEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_virtual_network_filter_enabled: Option<bool>,
    #[doc = "Enables automatic failover of the write region in the rare event that the region is unavailable due to an outage. Automatic failover will result in a new write region for the account and is chosen based on the failover priorities configured for the account."]
    #[serde(rename = "enableAutomaticFailover", default, skip_serializing_if = "Option::is_none")]
    pub enable_automatic_failover: Option<bool>,
    #[doc = "The consistency policy for the Cosmos DB database account."]
    #[serde(rename = "consistencyPolicy", default, skip_serializing_if = "Option::is_none")]
    pub consistency_policy: Option<ConsistencyPolicy>,
    #[doc = "List of Cosmos DB capabilities for the account"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<Capability>,
    #[doc = "An array that contains the write location for the Cosmos DB account."]
    #[serde(rename = "writeLocations", default, skip_serializing_if = "Vec::is_empty")]
    pub write_locations: Vec<Location>,
    #[doc = "An array that contains of the read locations enabled for the Cosmos DB account."]
    #[serde(rename = "readLocations", default, skip_serializing_if = "Vec::is_empty")]
    pub read_locations: Vec<Location>,
    #[doc = "An array that contains all of the locations enabled for the Cosmos DB account."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<Location>,
    #[doc = "An array that contains the regions ordered by their failover priorities."]
    #[serde(rename = "failoverPolicies", default, skip_serializing_if = "Vec::is_empty")]
    pub failover_policies: Vec<FailoverPolicy>,
    #[doc = "List of Virtual Network ACL rules configured for the Cosmos DB account."]
    #[serde(rename = "virtualNetworkRules", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_network_rules: Vec<VirtualNetworkRule>,
    #[doc = "List of Private Endpoint Connections configured for the Cosmos DB account."]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[doc = "Enables the account to write in multiple locations"]
    #[serde(rename = "enableMultipleWriteLocations", default, skip_serializing_if = "Option::is_none")]
    pub enable_multiple_write_locations: Option<bool>,
    #[doc = "Enables the cassandra connector on the Cosmos DB C* account"]
    #[serde(rename = "enableCassandraConnector", default, skip_serializing_if = "Option::is_none")]
    pub enable_cassandra_connector: Option<bool>,
    #[doc = "The cassandra connector offer type for the Cosmos DB C* database account."]
    #[serde(rename = "connectorOffer", default, skip_serializing_if = "Option::is_none")]
    pub connector_offer: Option<ConnectorOffer>,
    #[doc = "Disable write operations on metadata resources (databases, containers, throughput) via account keys"]
    #[serde(rename = "disableKeyBasedMetadataWriteAccess", default, skip_serializing_if = "Option::is_none")]
    pub disable_key_based_metadata_write_access: Option<bool>,
    #[doc = "The URI of the key vault"]
    #[serde(rename = "keyVaultKeyUri", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_key_uri: Option<String>,
    #[doc = "The default identity for accessing key vault used in features like customer managed keys. The default identity needs to be explicitly set by the users. It can be \"FirstPartyIdentity\", \"SystemAssignedIdentity\" and more."]
    #[serde(rename = "defaultIdentity", default, skip_serializing_if = "Option::is_none")]
    pub default_identity: Option<String>,
    #[doc = "Whether requests from Public Network are allowed"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccess>,
    #[doc = "Flag to indicate whether Free Tier is enabled."]
    #[serde(rename = "enableFreeTier", default, skip_serializing_if = "Option::is_none")]
    pub enable_free_tier: Option<bool>,
    #[serde(rename = "apiProperties", default, skip_serializing_if = "Option::is_none")]
    pub api_properties: Option<ApiProperties>,
    #[doc = "Flag to indicate whether to enable storage analytics."]
    #[serde(rename = "enableAnalyticalStorage", default, skip_serializing_if = "Option::is_none")]
    pub enable_analytical_storage: Option<bool>,
    #[doc = "A unique identifier assigned to the database account"]
    #[serde(rename = "instanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[doc = "Enum to indicate the mode of account creation."]
    #[serde(rename = "createMode", default, skip_serializing_if = "Option::is_none")]
    pub create_mode: Option<CreateMode>,
    #[doc = "Parameters to indicate the information about the restore."]
    #[serde(rename = "restoreParameters", default, skip_serializing_if = "Option::is_none")]
    pub restore_parameters: Option<RestoreParameters>,
    #[doc = "The object representing the policy for taking backups on an account."]
    #[serde(rename = "backupPolicy", default, skip_serializing_if = "Option::is_none")]
    pub backup_policy: Option<BackupPolicy>,
    #[doc = "The CORS policy for the Cosmos DB database account."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cors: Vec<CorsPolicy>,
    #[doc = "Indicates what services are allowed to bypass firewall checks."]
    #[serde(rename = "networkAclBypass", default, skip_serializing_if = "Option::is_none")]
    pub network_acl_bypass: Option<NetworkAclBypass>,
    #[doc = "An array that contains the Resource Ids for Network Acl Bypass for the Cosmos DB account."]
    #[serde(rename = "networkAclBypassResourceIds", default, skip_serializing_if = "Vec::is_empty")]
    pub network_acl_bypass_resource_ids: Vec<String>,
}
impl DatabaseAccountGetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Cosmos DB database account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseAccountGetResults {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "Indicates the type of database account. This can only be set at database account creation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<database_account_get_results::Kind>,
    #[doc = "Properties for the database account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseAccountGetProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl DatabaseAccountGetResults {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod database_account_get_results {
    use super::*;
    #[doc = "Indicates the type of database account. This can only be set at database account creation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        #[serde(rename = "GlobalDocumentDB")]
        GlobalDocumentDb,
        #[serde(rename = "MongoDB")]
        MongoDb,
        Parse,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::GlobalDocumentDb => serializer.serialize_unit_variant("Kind", 0u32, "GlobalDocumentDB"),
                Self::MongoDb => serializer.serialize_unit_variant("Kind", 1u32, "MongoDB"),
                Self::Parse => serializer.serialize_unit_variant("Kind", 2u32, "Parse"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Kind {
        fn default() -> Self {
            Self::GlobalDocumentDb
        }
    }
}
#[doc = "The connection strings for the given database account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseAccountListConnectionStringsResult {
    #[doc = "An array that contains the connection strings for the Cosmos DB account."]
    #[serde(rename = "connectionStrings", default, skip_serializing_if = "Vec::is_empty")]
    pub connection_strings: Vec<DatabaseAccountConnectionString>,
}
impl DatabaseAccountListConnectionStringsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The access keys for the given database account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseAccountListKeysResult {
    #[serde(flatten)]
    pub database_account_list_read_only_keys_result: DatabaseAccountListReadOnlyKeysResult,
    #[doc = "Base 64 encoded value of the primary read-write key."]
    #[serde(rename = "primaryMasterKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_master_key: Option<String>,
    #[doc = "Base 64 encoded value of the secondary read-write key."]
    #[serde(rename = "secondaryMasterKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_master_key: Option<String>,
}
impl DatabaseAccountListKeysResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The read-only access keys for the given database account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseAccountListReadOnlyKeysResult {
    #[doc = "Base 64 encoded value of the primary read-only key."]
    #[serde(rename = "primaryReadonlyMasterKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_readonly_master_key: Option<String>,
    #[doc = "Base 64 encoded value of the secondary read-only key."]
    #[serde(rename = "secondaryReadonlyMasterKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_readonly_master_key: Option<String>,
}
impl DatabaseAccountListReadOnlyKeysResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The offer type for the Cosmos DB database account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DatabaseAccountOfferType {
    Standard,
}
#[doc = "Parameters to regenerate the keys within the database account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseAccountRegenerateKeyParameters {
    #[doc = "The access key to regenerate."]
    #[serde(rename = "keyKind")]
    pub key_kind: database_account_regenerate_key_parameters::KeyKind,
}
impl DatabaseAccountRegenerateKeyParameters {
    pub fn new(key_kind: database_account_regenerate_key_parameters::KeyKind) -> Self {
        Self { key_kind }
    }
}
pub mod database_account_regenerate_key_parameters {
    use super::*;
    #[doc = "The access key to regenerate."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "KeyKind")]
    pub enum KeyKind {
        #[serde(rename = "primary")]
        Primary,
        #[serde(rename = "secondary")]
        Secondary,
        #[serde(rename = "primaryReadonly")]
        PrimaryReadonly,
        #[serde(rename = "secondaryReadonly")]
        SecondaryReadonly,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for KeyKind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for KeyKind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for KeyKind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Primary => serializer.serialize_unit_variant("KeyKind", 0u32, "primary"),
                Self::Secondary => serializer.serialize_unit_variant("KeyKind", 1u32, "secondary"),
                Self::PrimaryReadonly => serializer.serialize_unit_variant("KeyKind", 2u32, "primaryReadonly"),
                Self::SecondaryReadonly => serializer.serialize_unit_variant("KeyKind", 3u32, "secondaryReadonly"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Parameters for patching Azure Cosmos DB database account properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseAccountUpdateParameters {
    #[doc = "Tags are a list of key-value pairs that describe the resource. These tags can be used in viewing and grouping this resource (across resource groups). A maximum of 15 tags can be provided for a resource. Each tag must have a key no greater than 128 characters and value no greater than 256 characters. For example, the default experience for a template type is set with \"defaultExperience\": \"Cassandra\". Current \"defaultExperience\" values also include \"Table\", \"Graph\", \"DocumentDB\", and \"MongoDB\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[doc = "The location of the resource group to which the resource belongs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Properties to update Azure Cosmos DB database accounts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DatabaseAccountUpdateProperties>,
    #[doc = "Identity for the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl DatabaseAccountUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties to update Azure Cosmos DB database accounts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseAccountUpdateProperties {
    #[doc = "The consistency policy for the Cosmos DB database account."]
    #[serde(rename = "consistencyPolicy", default, skip_serializing_if = "Option::is_none")]
    pub consistency_policy: Option<ConsistencyPolicy>,
    #[doc = "An array that contains the georeplication locations enabled for the Cosmos DB account."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<Location>,
    #[doc = "Array of IpAddressOrRange objects."]
    #[serde(rename = "ipRules", default, skip_serializing_if = "Option::is_none")]
    pub ip_rules: Option<IpRules>,
    #[doc = "Flag to indicate whether to enable/disable Virtual Network ACL rules."]
    #[serde(rename = "isVirtualNetworkFilterEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_virtual_network_filter_enabled: Option<bool>,
    #[doc = "Enables automatic failover of the write region in the rare event that the region is unavailable due to an outage. Automatic failover will result in a new write region for the account and is chosen based on the failover priorities configured for the account."]
    #[serde(rename = "enableAutomaticFailover", default, skip_serializing_if = "Option::is_none")]
    pub enable_automatic_failover: Option<bool>,
    #[doc = "List of Cosmos DB capabilities for the account"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<Capability>,
    #[doc = "List of Virtual Network ACL rules configured for the Cosmos DB account."]
    #[serde(rename = "virtualNetworkRules", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_network_rules: Vec<VirtualNetworkRule>,
    #[doc = "Enables the account to write in multiple locations"]
    #[serde(rename = "enableMultipleWriteLocations", default, skip_serializing_if = "Option::is_none")]
    pub enable_multiple_write_locations: Option<bool>,
    #[doc = "Enables the cassandra connector on the Cosmos DB C* account"]
    #[serde(rename = "enableCassandraConnector", default, skip_serializing_if = "Option::is_none")]
    pub enable_cassandra_connector: Option<bool>,
    #[doc = "The cassandra connector offer type for the Cosmos DB C* database account."]
    #[serde(rename = "connectorOffer", default, skip_serializing_if = "Option::is_none")]
    pub connector_offer: Option<ConnectorOffer>,
    #[doc = "Disable write operations on metadata resources (databases, containers, throughput) via account keys"]
    #[serde(rename = "disableKeyBasedMetadataWriteAccess", default, skip_serializing_if = "Option::is_none")]
    pub disable_key_based_metadata_write_access: Option<bool>,
    #[doc = "The URI of the key vault"]
    #[serde(rename = "keyVaultKeyUri", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_key_uri: Option<String>,
    #[doc = "The default identity for accessing key vault used in features like customer managed keys. The default identity needs to be explicitly set by the users. It can be \"FirstPartyIdentity\", \"SystemAssignedIdentity\" and more."]
    #[serde(rename = "defaultIdentity", default, skip_serializing_if = "Option::is_none")]
    pub default_identity: Option<String>,
    #[doc = "Whether requests from Public Network are allowed"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccess>,
    #[doc = "Flag to indicate whether Free Tier is enabled."]
    #[serde(rename = "enableFreeTier", default, skip_serializing_if = "Option::is_none")]
    pub enable_free_tier: Option<bool>,
    #[serde(rename = "apiProperties", default, skip_serializing_if = "Option::is_none")]
    pub api_properties: Option<ApiProperties>,
    #[doc = "Flag to indicate whether to enable storage analytics."]
    #[serde(rename = "enableAnalyticalStorage", default, skip_serializing_if = "Option::is_none")]
    pub enable_analytical_storage: Option<bool>,
    #[doc = "The object representing the policy for taking backups on an account."]
    #[serde(rename = "backupPolicy", default, skip_serializing_if = "Option::is_none")]
    pub backup_policy: Option<BackupPolicy>,
    #[doc = "The CORS policy for the Cosmos DB database account."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cors: Vec<CorsPolicy>,
    #[doc = "Indicates what services are allowed to bypass firewall checks."]
    #[serde(rename = "networkAclBypass", default, skip_serializing_if = "Option::is_none")]
    pub network_acl_bypass: Option<NetworkAclBypass>,
    #[doc = "An array that contains the Resource Ids for Network Acl Bypass for the Cosmos DB account."]
    #[serde(rename = "networkAclBypassResourceIds", default, skip_serializing_if = "Vec::is_empty")]
    pub network_acl_bypass_resource_ids: Vec<String>,
}
impl DatabaseAccountUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List operation response, that contains the database accounts and their properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseAccountsListResult {
    #[doc = "List of database account and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatabaseAccountGetResults>,
}
impl azure_core::Continuable for DatabaseAccountsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl DatabaseAccountsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specific Databases to restore."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DatabaseRestoreResource {
    #[doc = "The name of the database available for restore."]
    #[serde(rename = "databaseName", default, skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[doc = "The names of the collections available for restore."]
    #[serde(rename = "collectionNames", default, skip_serializing_if = "Vec::is_empty")]
    pub collection_names: Vec<CollectionName>,
}
impl DatabaseRestoreResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for non-restore Azure Cosmos DB database account requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefaultRequestDatabaseAccountCreateUpdateProperties {
    #[serde(flatten)]
    pub database_account_create_update_properties: DatabaseAccountCreateUpdateProperties,
}
impl DefaultRequestDatabaseAccountCreateUpdateProperties {
    pub fn new(database_account_create_update_properties: DatabaseAccountCreateUpdateProperties) -> Self {
        Self {
            database_account_create_update_properties,
        }
    }
}
#[doc = "Error Response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExcludedPath {
    #[doc = "The path for which the indexing behavior applies to. Index paths typically start with root and end with wildcard (/path/*)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl ExcludedPath {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The system generated resource properties associated with SQL databases, SQL containers, Gremlin databases and Gremlin graphs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedResourceProperties {
    #[doc = "A system generated property. A unique identifier."]
    #[serde(rename = "_rid", default, skip_serializing_if = "Option::is_none")]
    pub rid: Option<String>,
    #[doc = "A system generated property that denotes the last updated timestamp of the resource."]
    #[serde(rename = "_ts", default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<f64>,
    #[doc = "A system generated property representing the resource etag required for optimistic concurrency control."]
    #[serde(rename = "_etag", default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl ExtendedResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of new failover policies for the failover priority change."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FailoverPolicies {
    #[doc = "List of failover policies."]
    #[serde(rename = "failoverPolicies")]
    pub failover_policies: Vec<FailoverPolicy>,
}
impl FailoverPolicies {
    pub fn new(failover_policies: Vec<FailoverPolicy>) -> Self {
        Self { failover_policies }
    }
}
#[doc = "The failover policy for a given region of a database account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FailoverPolicy {
    #[doc = "The unique identifier of the region in which the database account replicates to. Example: &lt;accountName&gt;-&lt;locationName&gt;."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the region in which the database account exists."]
    #[serde(rename = "locationName", default, skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    #[doc = "The failover priority of the region. A failover priority of 0 indicates a write region. The maximum value for a failover priority = (total number of regions - 1). Failover priority values must be unique for each of the regions in which the database account exists."]
    #[serde(rename = "failoverPriority", default, skip_serializing_if = "Option::is_none")]
    pub failover_priority: Option<i32>,
}
impl FailoverPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters to create and update Cosmos DB Gremlin database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GremlinDatabaseCreateUpdateParameters {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "Properties to create and update Azure Cosmos DB Gremlin database."]
    pub properties: GremlinDatabaseCreateUpdateProperties,
}
impl GremlinDatabaseCreateUpdateParameters {
    pub fn new(properties: GremlinDatabaseCreateUpdateProperties) -> Self {
        Self {
            arm_resource_properties: ArmResourceProperties::default(),
            properties,
        }
    }
}
#[doc = "Properties to create and update Azure Cosmos DB Gremlin database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GremlinDatabaseCreateUpdateProperties {
    #[doc = "Cosmos DB Gremlin database resource object"]
    pub resource: GremlinDatabaseResource,
    #[doc = "CreateUpdateOptions are a list of key-value pairs that describe the resource. Supported keys are \"If-Match\", \"If-None-Match\", \"Session-Token\" and \"Throughput\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<CreateUpdateOptions>,
}
impl GremlinDatabaseCreateUpdateProperties {
    pub fn new(resource: GremlinDatabaseResource) -> Self {
        Self { resource, options: None }
    }
}
#[doc = "The properties of an Azure Cosmos DB SQL database"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GremlinDatabaseGetProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}
impl GremlinDatabaseGetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Cosmos DB Gremlin database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GremlinDatabaseGetResults {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "The properties of an Azure Cosmos DB SQL database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GremlinDatabaseGetProperties>,
}
impl GremlinDatabaseGetResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List operation response, that contains the Gremlin databases and their properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GremlinDatabaseListResult {
    #[doc = "List of Gremlin databases and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GremlinDatabaseGetResults>,
}
impl azure_core::Continuable for GremlinDatabaseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl GremlinDatabaseListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cosmos DB Gremlin database resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GremlinDatabaseResource {
    #[doc = "Name of the Cosmos DB Gremlin database"]
    pub id: String,
}
impl GremlinDatabaseResource {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "Parameters to create and update Cosmos DB Gremlin graph."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GremlinGraphCreateUpdateParameters {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "Properties to create and update Azure Cosmos DB Gremlin graph."]
    pub properties: GremlinGraphCreateUpdateProperties,
}
impl GremlinGraphCreateUpdateParameters {
    pub fn new(properties: GremlinGraphCreateUpdateProperties) -> Self {
        Self {
            arm_resource_properties: ArmResourceProperties::default(),
            properties,
        }
    }
}
#[doc = "Properties to create and update Azure Cosmos DB Gremlin graph."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GremlinGraphCreateUpdateProperties {
    #[doc = "Cosmos DB Gremlin graph resource object"]
    pub resource: GremlinGraphResource,
    #[doc = "CreateUpdateOptions are a list of key-value pairs that describe the resource. Supported keys are \"If-Match\", \"If-None-Match\", \"Session-Token\" and \"Throughput\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<CreateUpdateOptions>,
}
impl GremlinGraphCreateUpdateProperties {
    pub fn new(resource: GremlinGraphResource) -> Self {
        Self { resource, options: None }
    }
}
#[doc = "The properties of an Azure Cosmos DB Gremlin graph"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GremlinGraphGetProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}
impl GremlinGraphGetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Cosmos DB Gremlin graph."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GremlinGraphGetResults {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "The properties of an Azure Cosmos DB Gremlin graph"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GremlinGraphGetProperties>,
}
impl GremlinGraphGetResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List operation response, that contains the graphs and their properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GremlinGraphListResult {
    #[doc = "List of graphs and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GremlinGraphGetResults>,
}
impl azure_core::Continuable for GremlinGraphListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl GremlinGraphListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cosmos DB Gremlin graph resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GremlinGraphResource {
    #[doc = "Name of the Cosmos DB Gremlin graph"]
    pub id: String,
    #[doc = "Cosmos DB indexing policy"]
    #[serde(rename = "indexingPolicy", default, skip_serializing_if = "Option::is_none")]
    pub indexing_policy: Option<IndexingPolicy>,
    #[doc = "The configuration of the partition key to be used for partitioning data into multiple partitions"]
    #[serde(rename = "partitionKey", default, skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<ContainerPartitionKey>,
    #[doc = "Default time to live"]
    #[serde(rename = "defaultTtl", default, skip_serializing_if = "Option::is_none")]
    pub default_ttl: Option<i64>,
    #[doc = "The unique key policy configuration for specifying uniqueness constraints on documents in the collection in the Azure Cosmos DB service."]
    #[serde(rename = "uniqueKeyPolicy", default, skip_serializing_if = "Option::is_none")]
    pub unique_key_policy: Option<UniqueKeyPolicy>,
    #[doc = "The conflict resolution policy for the container."]
    #[serde(rename = "conflictResolutionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_policy: Option<ConflictResolutionPolicy>,
}
impl GremlinGraphResource {
    pub fn new(id: String) -> Self {
        Self {
            id,
            indexing_policy: None,
            partition_key: None,
            default_ttl: None,
            unique_key_policy: None,
            conflict_resolution_policy: None,
        }
    }
}
pub type IpRules = Vec<IpAddressOrRange>;
#[doc = "The paths that are included in indexing"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IncludedPath {
    #[doc = "The path for which the indexing behavior applies to. Index paths typically start with root and end with wildcard (/path/*)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "List of indexes for this path"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub indexes: Vec<Indexes>,
}
impl IncludedPath {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The indexes for the path."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Indexes {
    #[doc = "The datatype for which the indexing behavior is applied to."]
    #[serde(rename = "dataType", default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<indexes::DataType>,
    #[doc = "The precision of the index. -1 is maximum precision."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<i64>,
    #[doc = "Indicates the type of index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<indexes::Kind>,
}
impl Indexes {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod indexes {
    use super::*;
    #[doc = "The datatype for which the indexing behavior is applied to."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "DataType")]
    pub enum DataType {
        String,
        Number,
        Point,
        Polygon,
        LineString,
        MultiPolygon,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for DataType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for DataType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for DataType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::String => serializer.serialize_unit_variant("DataType", 0u32, "String"),
                Self::Number => serializer.serialize_unit_variant("DataType", 1u32, "Number"),
                Self::Point => serializer.serialize_unit_variant("DataType", 2u32, "Point"),
                Self::Polygon => serializer.serialize_unit_variant("DataType", 3u32, "Polygon"),
                Self::LineString => serializer.serialize_unit_variant("DataType", 4u32, "LineString"),
                Self::MultiPolygon => serializer.serialize_unit_variant("DataType", 5u32, "MultiPolygon"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for DataType {
        fn default() -> Self {
            Self::String
        }
    }
    #[doc = "Indicates the type of index."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Kind")]
    pub enum Kind {
        Hash,
        Range,
        Spatial,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Kind {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Kind {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Kind {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Hash => serializer.serialize_unit_variant("Kind", 0u32, "Hash"),
                Self::Range => serializer.serialize_unit_variant("Kind", 1u32, "Range"),
                Self::Spatial => serializer.serialize_unit_variant("Kind", 2u32, "Spatial"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Kind {
        fn default() -> Self {
            Self::Hash
        }
    }
}
#[doc = "Cosmos DB indexing policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IndexingPolicy {
    #[doc = "Indicates if the indexing policy is automatic"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automatic: Option<bool>,
    #[doc = "Indicates the indexing mode."]
    #[serde(rename = "indexingMode", default, skip_serializing_if = "Option::is_none")]
    pub indexing_mode: Option<indexing_policy::IndexingMode>,
    #[doc = "List of paths to include in the indexing"]
    #[serde(rename = "includedPaths", default, skip_serializing_if = "Vec::is_empty")]
    pub included_paths: Vec<IncludedPath>,
    #[doc = "List of paths to exclude from indexing"]
    #[serde(rename = "excludedPaths", default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_paths: Vec<ExcludedPath>,
    #[doc = "List of composite path list"]
    #[serde(rename = "compositeIndexes", default, skip_serializing_if = "Vec::is_empty")]
    pub composite_indexes: Vec<CompositePathList>,
    #[doc = "List of spatial specifics"]
    #[serde(rename = "spatialIndexes", default, skip_serializing_if = "Vec::is_empty")]
    pub spatial_indexes: Vec<SpatialSpec>,
}
impl IndexingPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod indexing_policy {
    use super::*;
    #[doc = "Indicates the indexing mode."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "IndexingMode")]
    pub enum IndexingMode {
        #[serde(rename = "consistent")]
        Consistent,
        #[serde(rename = "lazy")]
        Lazy,
        #[serde(rename = "none")]
        None,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for IndexingMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for IndexingMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for IndexingMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Consistent => serializer.serialize_unit_variant("IndexingMode", 0u32, "consistent"),
                Self::Lazy => serializer.serialize_unit_variant("IndexingMode", 1u32, "lazy"),
                Self::None => serializer.serialize_unit_variant("IndexingMode", 2u32, "none"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for IndexingMode {
        fn default() -> Self {
            Self::Consistent
        }
    }
}
#[doc = "IpAddressOrRange object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpAddressOrRange {
    #[doc = "A single IPv4 address or a single IPv4 address range in CIDR format. Provided IPs must be well-formatted and cannot be contained in one of the following ranges: 10.0.0.0/8, 100.64.0.0/10, 172.16.0.0/12, 192.168.0.0/16, since these are not enforceable by the IP address filter. Example of valid inputs: “23.40.210.245” or “23.40.210.0/8”."]
    #[serde(rename = "ipAddressOrRange", default, skip_serializing_if = "Option::is_none")]
    pub ip_address_or_range: Option<String>,
}
impl IpAddressOrRange {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type Key = String;
#[doc = "List of restorable backups for a Cassandra cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListBackups {
    #[doc = "Container for array of backups."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<BackupResource>,
}
impl azure_core::Continuable for ListBackups {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ListBackups {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of managed Cassandra clusters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListClusters {
    #[doc = "Container for the array of clusters."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ClusterResource>,
}
impl azure_core::Continuable for ListClusters {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ListClusters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of managed Cassandra data centers and their properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListDataCenters {
    #[doc = "Container for array of data centers."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataCenterResource>,
}
impl azure_core::Continuable for ListDataCenters {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ListDataCenters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A region in which the Azure Cosmos DB database account is deployed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Location {
    #[doc = "The unique identifier of the region within the database account. Example: &lt;accountName&gt;-&lt;locationName&gt;."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the region."]
    #[serde(rename = "locationName", default, skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    #[doc = "The connection endpoint for the specific region. Example: https://&lt;accountName&gt;-&lt;locationName&gt;.documents.azure.com:443/"]
    #[serde(rename = "documentEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub document_endpoint: Option<String>,
    #[doc = "The status of the Cosmos DB account at the time the operation was called. The status can be one of following. 'Creating' – the Cosmos DB account is being created. When an account is in Creating state, only properties that are specified as input for the Create Cosmos DB account operation are returned. 'Succeeded' – the Cosmos DB account is active for use. 'Updating' – the Cosmos DB account is being updated. 'Deleting' – the Cosmos DB account is being deleted. 'Failed' – the Cosmos DB account failed creation. 'DeletionFailed' – the Cosmos DB account deletion failed."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The failover priority of the region. A failover priority of 0 indicates a write region. The maximum value for a failover priority = (total number of regions - 1). Failover priority values must be unique for each of the regions in which the database account exists."]
    #[serde(rename = "failoverPriority", default, skip_serializing_if = "Option::is_none")]
    pub failover_priority: Option<i32>,
    #[doc = "Flag to indicate whether or not this region is an AvailabilityZone region"]
    #[serde(rename = "isZoneRedundant", default, skip_serializing_if = "Option::is_none")]
    pub is_zone_redundant: Option<bool>,
}
impl Location {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cosmos DB location get result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocationGetResult {
    #[serde(flatten)]
    pub arm_proxy_resource: ArmProxyResource,
    #[doc = "Cosmos DB location metadata"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<LocationProperties>,
}
impl LocationGetResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List operation response, that contains Cosmos DB locations and their properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocationListResult {
    #[doc = "List of Cosmos DB locations and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LocationGetResult>,
}
impl azure_core::Continuable for LocationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl LocationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cosmos DB location metadata"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocationProperties {
    #[doc = "The current status of location in Azure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Flag indicating whether the location supports availability zones or not."]
    #[serde(rename = "supportsAvailabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub supports_availability_zone: Option<bool>,
    #[doc = "Flag indicating whether the location is residency sensitive."]
    #[serde(rename = "isResidencyRestricted", default, skip_serializing_if = "Option::is_none")]
    pub is_residency_restricted: Option<bool>,
    #[doc = "The properties of available backup storage redundancies."]
    #[serde(rename = "backupStorageRedundancies", default, skip_serializing_if = "Vec::is_empty")]
    pub backup_storage_redundancies: Vec<BackupStorageRedundancy>,
}
impl LocationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of the resource at the time the operation was called."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedCassandraProvisioningState")]
pub enum ManagedCassandraProvisioningState {
    Creating,
    Updating,
    Deleting,
    Succeeded,
    Failed,
    Canceled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagedCassandraProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagedCassandraProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagedCassandraProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Creating => serializer.serialize_unit_variant("ManagedCassandraProvisioningState", 0u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("ManagedCassandraProvisioningState", 1u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ManagedCassandraProvisioningState", 2u32, "Deleting"),
            Self::Succeeded => serializer.serialize_unit_variant("ManagedCassandraProvisioningState", 3u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ManagedCassandraProvisioningState", 4u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ManagedCassandraProvisioningState", 5u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Identity for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedServiceIdentity {
    #[doc = "The principal id of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant id of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The type of identity used for the resource. The type 'SystemAssigned,UserAssigned' includes both an implicitly created identity and a set of user assigned identities. The type 'None' will remove any identities from the service."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<managed_service_identity::Type>,
    #[doc = "The list of user identities associated with resource. The user identity dictionary key references will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}'."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl ManagedServiceIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod managed_service_identity {
    use super::*;
    #[doc = "The type of identity used for the resource. The type 'SystemAssigned,UserAssigned' includes both an implicitly created identity and a set of user assigned identities. The type 'None' will remove any identities from the service."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned,UserAssigned")]
        SystemAssignedUserAssigned,
        None,
    }
}
#[doc = "Metric data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Metric {
    #[doc = "The start time for the metric (ISO-8601 format)."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time for the metric (ISO-8601 format)."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The time grain to be used to summarize the metric values."]
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[doc = "The unit of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<UnitType>,
    #[doc = "A metric name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<MetricName>,
    #[doc = "The metric values for the specified time window and timestep."]
    #[serde(rename = "metricValues", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_values: Vec<MetricValue>,
}
impl Metric {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The availability of the metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricAvailability {
    #[doc = "The time grain to be used to summarize the metric values."]
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[doc = "The retention for the metric values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention: Option<String>,
}
impl MetricAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The definition of a metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricDefinition {
    #[doc = "The list of metric availabilities for the account."]
    #[serde(rename = "metricAvailabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_availabilities: Vec<MetricAvailability>,
    #[doc = "The primary aggregation type of the metric."]
    #[serde(rename = "primaryAggregationType", default, skip_serializing_if = "Option::is_none")]
    pub primary_aggregation_type: Option<metric_definition::PrimaryAggregationType>,
    #[doc = "The unit of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<UnitType>,
    #[doc = "The resource uri of the database."]
    #[serde(rename = "resourceUri", default, skip_serializing_if = "Option::is_none")]
    pub resource_uri: Option<String>,
    #[doc = "A metric name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<MetricName>,
}
impl MetricDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod metric_definition {
    use super::*;
    #[doc = "The primary aggregation type of the metric."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "PrimaryAggregationType")]
    pub enum PrimaryAggregationType {
        None,
        Average,
        Total,
        Minimum,
        Maximum,
        Last,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for PrimaryAggregationType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for PrimaryAggregationType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for PrimaryAggregationType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::None => serializer.serialize_unit_variant("PrimaryAggregationType", 0u32, "None"),
                Self::Average => serializer.serialize_unit_variant("PrimaryAggregationType", 1u32, "Average"),
                Self::Total => serializer.serialize_unit_variant("PrimaryAggregationType", 2u32, "Total"),
                Self::Minimum => serializer.serialize_unit_variant("PrimaryAggregationType", 3u32, "Minimum"),
                Self::Maximum => serializer.serialize_unit_variant("PrimaryAggregationType", 4u32, "Maximum"),
                Self::Last => serializer.serialize_unit_variant("PrimaryAggregationType", 5u32, "Last"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The response to a list metric definitions request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricDefinitionsListResult {
    #[doc = "The list of metric definitions for the account."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MetricDefinition>,
}
impl azure_core::Continuable for MetricDefinitionsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MetricDefinitionsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a list metrics request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricListResult {
    #[doc = "The list of metrics for the account."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Metric>,
}
impl azure_core::Continuable for MetricListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MetricListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A metric name."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricName {
    #[doc = "The name of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The friendly name of the metric."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl MetricName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents metrics values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricValue {
    #[doc = "The number of values for the metric."]
    #[serde(rename = "_count", default, skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,
    #[doc = "The average value of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average: Option<f64>,
    #[doc = "The max value of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[doc = "The min value of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[doc = "The metric timestamp (ISO-8601 format)."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "The total value of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
}
impl MetricValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters to create and update Cosmos DB MongoDB collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MongoDbCollectionCreateUpdateParameters {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "Properties to create and update Azure Cosmos DB MongoDB collection."]
    pub properties: MongoDbCollectionCreateUpdateProperties,
}
impl MongoDbCollectionCreateUpdateParameters {
    pub fn new(properties: MongoDbCollectionCreateUpdateProperties) -> Self {
        Self {
            arm_resource_properties: ArmResourceProperties::default(),
            properties,
        }
    }
}
#[doc = "Properties to create and update Azure Cosmos DB MongoDB collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MongoDbCollectionCreateUpdateProperties {
    #[doc = "Cosmos DB MongoDB collection resource object"]
    pub resource: MongoDbCollectionResource,
    #[doc = "CreateUpdateOptions are a list of key-value pairs that describe the resource. Supported keys are \"If-Match\", \"If-None-Match\", \"Session-Token\" and \"Throughput\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<CreateUpdateOptions>,
}
impl MongoDbCollectionCreateUpdateProperties {
    pub fn new(resource: MongoDbCollectionResource) -> Self {
        Self { resource, options: None }
    }
}
#[doc = "The properties of an Azure Cosmos DB MongoDB collection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MongoDbCollectionGetProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}
impl MongoDbCollectionGetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Cosmos DB MongoDB collection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MongoDbCollectionGetResults {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "The properties of an Azure Cosmos DB MongoDB collection"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MongoDbCollectionGetProperties>,
}
impl MongoDbCollectionGetResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List operation response, that contains the MongoDB collections and their properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MongoDbCollectionListResult {
    #[doc = "List of MongoDB collections and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MongoDbCollectionGetResults>,
}
impl azure_core::Continuable for MongoDbCollectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MongoDbCollectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cosmos DB MongoDB collection resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MongoDbCollectionResource {
    #[doc = "Name of the Cosmos DB MongoDB collection"]
    pub id: String,
    #[doc = "The shard key and partition kind pair, only support \"Hash\" partition kind"]
    #[serde(rename = "shardKey", default, skip_serializing_if = "Option::is_none")]
    pub shard_key: Option<ShardKeys>,
    #[doc = "List of index keys"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub indexes: Vec<MongoIndex>,
    #[doc = "Analytical TTL."]
    #[serde(rename = "analyticalStorageTtl", default, skip_serializing_if = "Option::is_none")]
    pub analytical_storage_ttl: Option<i64>,
}
impl MongoDbCollectionResource {
    pub fn new(id: String) -> Self {
        Self {
            id,
            shard_key: None,
            indexes: Vec::new(),
            analytical_storage_ttl: None,
        }
    }
}
#[doc = "Parameters to create and update Cosmos DB MongoDB database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MongoDbDatabaseCreateUpdateParameters {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "Properties to create and update Azure Cosmos DB MongoDB database."]
    pub properties: MongoDbDatabaseCreateUpdateProperties,
}
impl MongoDbDatabaseCreateUpdateParameters {
    pub fn new(properties: MongoDbDatabaseCreateUpdateProperties) -> Self {
        Self {
            arm_resource_properties: ArmResourceProperties::default(),
            properties,
        }
    }
}
#[doc = "Properties to create and update Azure Cosmos DB MongoDB database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MongoDbDatabaseCreateUpdateProperties {
    #[doc = "Cosmos DB MongoDB database resource object"]
    pub resource: MongoDbDatabaseResource,
    #[doc = "CreateUpdateOptions are a list of key-value pairs that describe the resource. Supported keys are \"If-Match\", \"If-None-Match\", \"Session-Token\" and \"Throughput\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<CreateUpdateOptions>,
}
impl MongoDbDatabaseCreateUpdateProperties {
    pub fn new(resource: MongoDbDatabaseResource) -> Self {
        Self { resource, options: None }
    }
}
#[doc = "The properties of an Azure Cosmos DB MongoDB database"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MongoDbDatabaseGetProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}
impl MongoDbDatabaseGetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Cosmos DB MongoDB database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MongoDbDatabaseGetResults {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "The properties of an Azure Cosmos DB MongoDB database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MongoDbDatabaseGetProperties>,
}
impl MongoDbDatabaseGetResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List operation response, that contains the MongoDB databases and their properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MongoDbDatabaseListResult {
    #[doc = "List of MongoDB databases and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MongoDbDatabaseGetResults>,
}
impl azure_core::Continuable for MongoDbDatabaseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl MongoDbDatabaseListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cosmos DB MongoDB database resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MongoDbDatabaseResource {
    #[doc = "Name of the Cosmos DB MongoDB database"]
    pub id: String,
}
impl MongoDbDatabaseResource {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "Cosmos DB MongoDB collection index key"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MongoIndex {
    #[doc = "Cosmos DB MongoDB collection resource object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<MongoIndexKeys>,
    #[doc = "Cosmos DB MongoDB collection index options"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<MongoIndexOptions>,
}
impl MongoIndex {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cosmos DB MongoDB collection resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MongoIndexKeys {
    #[doc = "List of keys for each MongoDB collection in the Azure Cosmos DB service"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keys: Vec<Key>,
}
impl MongoIndexKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cosmos DB MongoDB collection index options"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MongoIndexOptions {
    #[doc = "Expire after seconds"]
    #[serde(rename = "expireAfterSeconds", default, skip_serializing_if = "Option::is_none")]
    pub expire_after_seconds: Option<i64>,
    #[doc = "Is unique or not"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,
}
impl MongoIndexOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Indicates what services are allowed to bypass firewall checks."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NetworkAclBypass {
    None,
    AzureServices,
}
#[doc = "A notebook workspace resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotebookWorkspace {
    #[serde(flatten)]
    pub arm_proxy_resource: ArmProxyResource,
    #[doc = "Properties of a notebook workspace resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NotebookWorkspaceProperties>,
}
impl NotebookWorkspace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The connection info for the given notebook workspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotebookWorkspaceConnectionInfoResult {
    #[doc = "Specifies auth token used for connecting to Notebook server (uses token-based auth)."]
    #[serde(rename = "authToken", default, skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[doc = "Specifies the endpoint of Notebook server."]
    #[serde(rename = "notebookServerEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub notebook_server_endpoint: Option<String>,
}
impl NotebookWorkspaceConnectionInfoResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters to create a notebook workspace resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotebookWorkspaceCreateUpdateParameters {
    #[serde(flatten)]
    pub arm_proxy_resource: ArmProxyResource,
}
impl NotebookWorkspaceCreateUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of notebook workspace resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotebookWorkspaceListResult {
    #[doc = "Array of notebook workspace resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NotebookWorkspace>,
}
impl azure_core::Continuable for NotebookWorkspaceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl NotebookWorkspaceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a notebook workspace resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotebookWorkspaceProperties {
    #[doc = "Specifies the endpoint of Notebook server."]
    #[serde(rename = "notebookServerEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub notebook_server_endpoint: Option<String>,
    #[doc = "Status of the notebook workspace. Possible values are: Creating, Online, Deleting, Failed, Updating."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
impl NotebookWorkspaceProperties {
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
        #[serde(rename = "Provider", default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed: Profile, endpoint, etc."]
        #[serde(rename = "Resource", default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation type: Read, write, delete, etc."]
        #[serde(rename = "Operation", default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Description of operation"]
        #[serde(rename = "Description", default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
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
    #[doc = "List of operations supported by the Resource Provider."]
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
#[doc = "Enum to indicate the operation type of the event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OperationType")]
pub enum OperationType {
    Create,
    Replace,
    Delete,
    SystemOperation,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OperationType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OperationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OperationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Create => serializer.serialize_unit_variant("OperationType", 0u32, "Create"),
            Self::Replace => serializer.serialize_unit_variant("OperationType", 1u32, "Replace"),
            Self::Delete => serializer.serialize_unit_variant("OperationType", 2u32, "Delete"),
            Self::SystemOperation => serializer.serialize_unit_variant("OperationType", 3u32, "SystemOperation"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Cosmos DB options resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OptionsResource {
    #[doc = "Value of the Cosmos DB resource throughput or autoscaleSettings. Use the ThroughputSetting resource when retrieving offer details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throughput: Option<i64>,
    #[serde(rename = "autoscaleSettings", default, skip_serializing_if = "Option::is_none")]
    pub autoscale_settings: Option<AutoscaleSettings>,
}
impl OptionsResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The metric values for a single partition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartitionMetric {
    #[serde(flatten)]
    pub metric: Metric,
    #[doc = "The partition id (GUID identifier) of the metric values."]
    #[serde(rename = "partitionId", default, skip_serializing_if = "Option::is_none")]
    pub partition_id: Option<String>,
    #[doc = "The partition key range id (integer identifier) of the metric values."]
    #[serde(rename = "partitionKeyRangeId", default, skip_serializing_if = "Option::is_none")]
    pub partition_key_range_id: Option<String>,
}
impl PartitionMetric {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a list partition metrics request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartitionMetricListResult {
    #[doc = "The list of partition-level metrics for the account."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PartitionMetric>,
}
impl azure_core::Continuable for PartitionMetricListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl PartitionMetricListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The partition level usage data for a usage request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartitionUsage {
    #[serde(flatten)]
    pub usage: Usage,
    #[doc = "The partition id (GUID identifier) of the usages."]
    #[serde(rename = "partitionId", default, skip_serializing_if = "Option::is_none")]
    pub partition_id: Option<String>,
    #[doc = "The partition key range id (integer identifier) of the usages."]
    #[serde(rename = "partitionKeyRangeId", default, skip_serializing_if = "Option::is_none")]
    pub partition_key_range_id: Option<String>,
}
impl PartitionUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a list partition level usage request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PartitionUsagesResult {
    #[doc = "The list of partition-level usages for the database. A usage is a point in time metric"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PartitionUsage>,
}
impl azure_core::Continuable for PartitionUsagesResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl PartitionUsagesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type Path = String;
#[doc = "Percentile Metric data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PercentileMetric {
    #[doc = "The start time for the metric (ISO-8601 format)."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time for the metric (ISO-8601 format)."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The time grain to be used to summarize the metric values."]
    #[serde(rename = "timeGrain", default, skip_serializing_if = "Option::is_none")]
    pub time_grain: Option<String>,
    #[doc = "The unit of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<UnitType>,
    #[doc = "A metric name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<MetricName>,
    #[doc = "The percentile metric values for the specified time window and timestep."]
    #[serde(rename = "metricValues", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_values: Vec<PercentileMetricValue>,
}
impl PercentileMetric {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a list percentile metrics request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PercentileMetricListResult {
    #[doc = "The list of percentile metrics for the account."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PercentileMetric>,
}
impl azure_core::Continuable for PercentileMetricListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl PercentileMetricListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents percentile metrics values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PercentileMetricValue {
    #[serde(flatten)]
    pub metric_value: MetricValue,
    #[doc = "The 10th percentile value for the metric."]
    #[serde(rename = "P10", default, skip_serializing_if = "Option::is_none")]
    pub p10: Option<f64>,
    #[doc = "The 25th percentile value for the metric."]
    #[serde(rename = "P25", default, skip_serializing_if = "Option::is_none")]
    pub p25: Option<f64>,
    #[doc = "The 50th percentile value for the metric."]
    #[serde(rename = "P50", default, skip_serializing_if = "Option::is_none")]
    pub p50: Option<f64>,
    #[doc = "The 75th percentile value for the metric."]
    #[serde(rename = "P75", default, skip_serializing_if = "Option::is_none")]
    pub p75: Option<f64>,
    #[doc = "The 90th percentile value for the metric."]
    #[serde(rename = "P90", default, skip_serializing_if = "Option::is_none")]
    pub p90: Option<f64>,
    #[doc = "The 95th percentile value for the metric."]
    #[serde(rename = "P95", default, skip_serializing_if = "Option::is_none")]
    pub p95: Option<f64>,
    #[doc = "The 99th percentile value for the metric."]
    #[serde(rename = "P99", default, skip_serializing_if = "Option::is_none")]
    pub p99: Option<f64>,
}
impl PercentileMetricValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The object representing periodic mode backup policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeriodicModeBackupPolicy {
    #[serde(flatten)]
    pub backup_policy: BackupPolicy,
    #[doc = "Configuration values for periodic mode backup"]
    #[serde(rename = "periodicModeProperties", default, skip_serializing_if = "Option::is_none")]
    pub periodic_mode_properties: Option<PeriodicModeProperties>,
}
impl PeriodicModeBackupPolicy {
    pub fn new(backup_policy: BackupPolicy) -> Self {
        Self {
            backup_policy,
            periodic_mode_properties: None,
        }
    }
}
#[doc = "Configuration values for periodic mode backup"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PeriodicModeProperties {
    #[doc = "An integer representing the interval in minutes between two backups"]
    #[serde(rename = "backupIntervalInMinutes", default, skip_serializing_if = "Option::is_none")]
    pub backup_interval_in_minutes: Option<i32>,
    #[doc = "An integer representing the time (in hours) that each backup is retained"]
    #[serde(rename = "backupRetentionIntervalInHours", default, skip_serializing_if = "Option::is_none")]
    pub backup_retention_interval_in_hours: Option<i32>,
    #[doc = "Enum to indicate type of backup storage redundancy."]
    #[serde(rename = "backupStorageRedundancy", default, skip_serializing_if = "Option::is_none")]
    pub backup_storage_redundancy: Option<BackupStorageRedundancy>,
}
impl PeriodicModeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The set of data plane operations permitted through this Role Definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Permission {
    #[doc = "An array of data actions that are allowed."]
    #[serde(rename = "dataActions", default, skip_serializing_if = "Vec::is_empty")]
    pub data_actions: Vec<String>,
    #[doc = "An array of data actions that are denied."]
    #[serde(rename = "notDataActions", default, skip_serializing_if = "Vec::is_empty")]
    pub not_data_actions: Vec<String>,
}
impl Permission {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A private endpoint connection"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a private endpoint connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of private endpoint connections"]
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
#[doc = "Properties of a private endpoint connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "Private endpoint which the connection belongs to."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpointProperty>,
    #[doc = "Connection State of the Private Endpoint Connection."]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionStateProperty>,
    #[doc = "Group id of the private endpoint."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "Provisioning state of the private endpoint."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Private endpoint which the connection belongs to."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointProperty {
    #[doc = "Resource id of the private endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpointProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A private link resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub arm_proxy_resource: ArmProxyResource,
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
    #[doc = "The private link resource required zone names."]
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Connection State of the Private Endpoint Connection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionStateProperty {
    #[doc = "The private link service connection status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The private link service connection description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Any action that is required beyond basic workflow (approve/ reject/ disconnect)"]
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionStateProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ProvisioningState = String;
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
#[doc = "Whether requests from Public Network are allowed"]
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
#[doc = "Cosmos DB region to online or offline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegionForOnlineOffline {
    #[doc = "Cosmos DB region, with spaces between words and each word capitalized."]
    pub region: String,
}
impl RegionForOnlineOffline {
    pub fn new(region: String) -> Self {
        Self { region }
    }
}
#[doc = "Specification of the keyspaces and tables to run repair on."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepairPostBody {
    #[doc = "The name of the keyspace that repair should be run on."]
    pub keyspace: String,
    #[doc = "List of tables in the keyspace to repair. If omitted, repair all tables in the keyspace."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tables: Vec<String>,
}
impl RepairPostBody {
    pub fn new(keyspace: String) -> Self {
        Self {
            keyspace,
            tables: Vec::new(),
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
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Azure Cosmos DB restorable database account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableDatabaseAccountGetResult {
    #[doc = "The properties of a restorable database account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RestorableDatabaseAccountProperties>,
    #[doc = "The unique resource identifier of the ARM resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the ARM resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of Azure resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The location of the resource group to which the resource belongs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl RestorableDatabaseAccountGetResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of a restorable database account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableDatabaseAccountProperties {
    #[doc = "The name of the global database account"]
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[doc = "The creation time of the restorable database account (ISO-8601 format)."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "The time at which the restorable database account has been deleted (ISO-8601 format)."]
    #[serde(rename = "deletionTime", with = "azure_core::date::rfc3339::option")]
    pub deletion_time: Option<time::OffsetDateTime>,
    #[doc = "Enum to indicate the API type of the restorable database account."]
    #[serde(rename = "apiType", default, skip_serializing_if = "Option::is_none")]
    pub api_type: Option<ApiType>,
    #[doc = "List of regions where the of the database account can be restored from."]
    #[serde(rename = "restorableLocations", default, skip_serializing_if = "Vec::is_empty")]
    pub restorable_locations: Vec<RestorableLocationResource>,
}
impl RestorableDatabaseAccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List operation response, that contains the restorable database accounts and their properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableDatabaseAccountsListResult {
    #[doc = "List of restorable database accounts and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RestorableDatabaseAccountGetResult>,
}
impl azure_core::Continuable for RestorableDatabaseAccountsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl RestorableDatabaseAccountsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the regional restorable account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableLocationResource {
    #[doc = "The location of the regional restorable account."]
    #[serde(rename = "locationName", default, skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    #[doc = "The instance id of the regional restorable account."]
    #[serde(rename = "regionalDatabaseAccountInstanceId", default, skip_serializing_if = "Option::is_none")]
    pub regional_database_account_instance_id: Option<String>,
    #[doc = "The creation time of the regional restorable database account (ISO-8601 format)."]
    #[serde(rename = "creationTime", with = "azure_core::date::rfc3339::option")]
    pub creation_time: Option<time::OffsetDateTime>,
    #[doc = "The time at which the regional restorable database account has been deleted (ISO-8601 format)."]
    #[serde(rename = "deletionTime", with = "azure_core::date::rfc3339::option")]
    pub deletion_time: Option<time::OffsetDateTime>,
}
impl RestorableLocationResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Cosmos DB MongoDB collection event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableMongodbCollectionGetResult {
    #[doc = "The properties of an Azure Cosmos DB MongoDB collection event"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RestorableMongodbCollectionProperties>,
    #[doc = "The unique resource Identifier of the ARM resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the ARM resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of Azure resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl RestorableMongodbCollectionGetResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an Azure Cosmos DB MongoDB collection event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableMongodbCollectionProperties {
    #[doc = "The resource of an Azure Cosmos DB MongoDB collection event"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<restorable_mongodb_collection_properties::Resource>,
}
impl RestorableMongodbCollectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod restorable_mongodb_collection_properties {
    use super::*;
    #[doc = "The resource of an Azure Cosmos DB MongoDB collection event"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Resource {
        #[doc = "A system generated property. A unique identifier."]
        #[serde(rename = "_rid", default, skip_serializing_if = "Option::is_none")]
        pub rid: Option<String>,
        #[doc = "Enum to indicate the operation type of the event."]
        #[serde(rename = "operationType", default, skip_serializing_if = "Option::is_none")]
        pub operation_type: Option<OperationType>,
        #[doc = "The time when this collection event happened."]
        #[serde(rename = "eventTimestamp", default, skip_serializing_if = "Option::is_none")]
        pub event_timestamp: Option<String>,
        #[doc = "The name of this MongoDB collection."]
        #[serde(rename = "ownerId", default, skip_serializing_if = "Option::is_none")]
        pub owner_id: Option<String>,
        #[doc = "The resource ID of this MongoDB collection."]
        #[serde(rename = "ownerResourceId", default, skip_serializing_if = "Option::is_none")]
        pub owner_resource_id: Option<String>,
    }
    impl Resource {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The List operation response, that contains the MongoDB collection events and their properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableMongodbCollectionsListResult {
    #[doc = "List of MongoDB collection events and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RestorableMongodbCollectionGetResult>,
}
impl azure_core::Continuable for RestorableMongodbCollectionsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl RestorableMongodbCollectionsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Cosmos DB MongoDB database event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableMongodbDatabaseGetResult {
    #[doc = "The properties of an Azure Cosmos DB MongoDB database event"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RestorableMongodbDatabaseProperties>,
    #[doc = "The unique resource Identifier of the ARM resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the ARM resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of Azure resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl RestorableMongodbDatabaseGetResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an Azure Cosmos DB MongoDB database event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableMongodbDatabaseProperties {
    #[doc = "The resource of an Azure Cosmos DB MongoDB database event"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<restorable_mongodb_database_properties::Resource>,
}
impl RestorableMongodbDatabaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod restorable_mongodb_database_properties {
    use super::*;
    #[doc = "The resource of an Azure Cosmos DB MongoDB database event"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Resource {
        #[doc = "A system generated property. A unique identifier."]
        #[serde(rename = "_rid", default, skip_serializing_if = "Option::is_none")]
        pub rid: Option<String>,
        #[doc = "Enum to indicate the operation type of the event."]
        #[serde(rename = "operationType", default, skip_serializing_if = "Option::is_none")]
        pub operation_type: Option<OperationType>,
        #[doc = "The time when this database event happened."]
        #[serde(rename = "eventTimestamp", default, skip_serializing_if = "Option::is_none")]
        pub event_timestamp: Option<String>,
        #[doc = "The name of this MongoDB database."]
        #[serde(rename = "ownerId", default, skip_serializing_if = "Option::is_none")]
        pub owner_id: Option<String>,
        #[doc = "The resource ID of this MongoDB database."]
        #[serde(rename = "ownerResourceId", default, skip_serializing_if = "Option::is_none")]
        pub owner_resource_id: Option<String>,
    }
    impl Resource {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The List operation response, that contains the MongoDB database events and their properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableMongodbDatabasesListResult {
    #[doc = "List of MongoDB database events and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RestorableMongodbDatabaseGetResult>,
}
impl azure_core::Continuable for RestorableMongodbDatabasesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl RestorableMongodbDatabasesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List operation response, that contains the restorable MongoDB resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableMongodbResourcesListResult {
    #[doc = "List of restorable MongoDB resources, including the database and collection names."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatabaseRestoreResource>,
}
impl azure_core::Continuable for RestorableMongodbResourcesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl RestorableMongodbResourcesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Cosmos DB SQL container event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableSqlContainerGetResult {
    #[doc = "The properties of an Azure Cosmos DB SQL container event"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RestorableSqlContainerProperties>,
    #[doc = "The unique resource Identifier of the ARM resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the ARM resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of Azure resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl RestorableSqlContainerGetResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an Azure Cosmos DB SQL container event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableSqlContainerProperties {
    #[doc = "The resource of an Azure Cosmos DB SQL container event"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<restorable_sql_container_properties::Resource>,
}
impl RestorableSqlContainerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod restorable_sql_container_properties {
    use super::*;
    #[doc = "The resource of an Azure Cosmos DB SQL container event"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Resource {
        #[doc = "A system generated property. A unique identifier."]
        #[serde(rename = "_rid", default, skip_serializing_if = "Option::is_none")]
        pub rid: Option<String>,
        #[doc = "Enum to indicate the operation type of the event."]
        #[serde(rename = "operationType", default, skip_serializing_if = "Option::is_none")]
        pub operation_type: Option<OperationType>,
        #[doc = "The when this container event happened."]
        #[serde(rename = "eventTimestamp", default, skip_serializing_if = "Option::is_none")]
        pub event_timestamp: Option<String>,
        #[doc = "The name of this SQL container."]
        #[serde(rename = "ownerId", default, skip_serializing_if = "Option::is_none")]
        pub owner_id: Option<String>,
        #[doc = "The resource ID of this SQL container."]
        #[serde(rename = "ownerResourceId", default, skip_serializing_if = "Option::is_none")]
        pub owner_resource_id: Option<String>,
        #[doc = "Cosmos DB SQL container resource object"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub container: Option<resource::Container>,
    }
    impl Resource {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod resource {
        use super::*;
        #[doc = "Cosmos DB SQL container resource object"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct Container {
            #[serde(flatten)]
            pub sql_container_resource: SqlContainerResource,
            #[serde(flatten)]
            pub extended_resource_properties: ExtendedResourceProperties,
            #[doc = "A system generated property that specifies the addressable path of the container resource."]
            #[serde(rename = "_self", default, skip_serializing_if = "Option::is_none")]
            pub self_: Option<String>,
        }
        impl Container {
            pub fn new(sql_container_resource: SqlContainerResource) -> Self {
                Self {
                    sql_container_resource,
                    extended_resource_properties: ExtendedResourceProperties::default(),
                    self_: None,
                }
            }
        }
    }
}
#[doc = "The List operation response, that contains the SQL container events and their properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableSqlContainersListResult {
    #[doc = "List of SQL container events and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RestorableSqlContainerGetResult>,
}
impl azure_core::Continuable for RestorableSqlContainersListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl RestorableSqlContainersListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Cosmos DB SQL database event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableSqlDatabaseGetResult {
    #[doc = "The properties of an Azure Cosmos DB SQL database event"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RestorableSqlDatabaseProperties>,
    #[doc = "The unique resource Identifier of the ARM resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the ARM resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of Azure resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl RestorableSqlDatabaseGetResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an Azure Cosmos DB SQL database event"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableSqlDatabaseProperties {
    #[doc = "The resource of an Azure Cosmos DB SQL database event"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<restorable_sql_database_properties::Resource>,
}
impl RestorableSqlDatabaseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod restorable_sql_database_properties {
    use super::*;
    #[doc = "The resource of an Azure Cosmos DB SQL database event"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Resource {
        #[doc = "A system generated property. A unique identifier."]
        #[serde(rename = "_rid", default, skip_serializing_if = "Option::is_none")]
        pub rid: Option<String>,
        #[doc = "Enum to indicate the operation type of the event."]
        #[serde(rename = "operationType", default, skip_serializing_if = "Option::is_none")]
        pub operation_type: Option<OperationType>,
        #[doc = "The time when this database event happened."]
        #[serde(rename = "eventTimestamp", default, skip_serializing_if = "Option::is_none")]
        pub event_timestamp: Option<String>,
        #[doc = "The name of the SQL database."]
        #[serde(rename = "ownerId", default, skip_serializing_if = "Option::is_none")]
        pub owner_id: Option<String>,
        #[doc = "The resource ID of the SQL database."]
        #[serde(rename = "ownerResourceId", default, skip_serializing_if = "Option::is_none")]
        pub owner_resource_id: Option<String>,
        #[doc = "Cosmos DB SQL database resource object"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub database: Option<resource::Database>,
    }
    impl Resource {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod resource {
        use super::*;
        #[doc = "Cosmos DB SQL database resource object"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub struct Database {
            #[serde(flatten)]
            pub sql_database_resource: SqlDatabaseResource,
            #[serde(flatten)]
            pub extended_resource_properties: ExtendedResourceProperties,
            #[doc = "A system generated property that specified the addressable path of the collections resource."]
            #[serde(rename = "_colls", default, skip_serializing_if = "Option::is_none")]
            pub colls: Option<String>,
            #[doc = "A system generated property that specifies the addressable path of the users resource."]
            #[serde(rename = "_users", default, skip_serializing_if = "Option::is_none")]
            pub users: Option<String>,
            #[doc = "A system generated property that specifies the addressable path of the database resource."]
            #[serde(rename = "_self", default, skip_serializing_if = "Option::is_none")]
            pub self_: Option<String>,
        }
        impl Database {
            pub fn new(sql_database_resource: SqlDatabaseResource) -> Self {
                Self {
                    sql_database_resource,
                    extended_resource_properties: ExtendedResourceProperties::default(),
                    colls: None,
                    users: None,
                    self_: None,
                }
            }
        }
    }
}
#[doc = "The List operation response, that contains the SQL database events and their properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableSqlDatabasesListResult {
    #[doc = "List of SQL database events and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RestorableSqlDatabaseGetResult>,
}
impl azure_core::Continuable for RestorableSqlDatabasesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl RestorableSqlDatabasesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List operation response, that contains the restorable SQL resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestorableSqlResourcesListResult {
    #[doc = "List of restorable SQL resources, including the database and collection names."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DatabaseRestoreResource>,
}
impl azure_core::Continuable for RestorableSqlResourcesListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl RestorableSqlResourcesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters to indicate the information about the restore."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestoreParameters {
    #[doc = "Describes the mode of the restore."]
    #[serde(rename = "restoreMode", default, skip_serializing_if = "Option::is_none")]
    pub restore_mode: Option<restore_parameters::RestoreMode>,
    #[doc = "The id of the restorable database account from which the restore has to be initiated. For example: /subscriptions/{subscriptionId}/providers/Microsoft.DocumentDB/locations/{location}/restorableDatabaseAccounts/{restorableDatabaseAccountName}"]
    #[serde(rename = "restoreSource", default, skip_serializing_if = "Option::is_none")]
    pub restore_source: Option<String>,
    #[doc = "Time to which the account has to be restored (ISO-8601 format)."]
    #[serde(rename = "restoreTimestampInUtc", with = "azure_core::date::rfc3339::option")]
    pub restore_timestamp_in_utc: Option<time::OffsetDateTime>,
    #[doc = "List of specific databases available for restore."]
    #[serde(rename = "databasesToRestore", default, skip_serializing_if = "Vec::is_empty")]
    pub databases_to_restore: Vec<DatabaseRestoreResource>,
}
impl RestoreParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod restore_parameters {
    use super::*;
    #[doc = "Describes the mode of the restore."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RestoreMode")]
    pub enum RestoreMode {
        PointInTime,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RestoreMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RestoreMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RestoreMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PointInTime => serializer.serialize_unit_variant("RestoreMode", 0u32, "PointInTime"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties to restore Azure Cosmos DB database account."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestoreReqeustDatabaseAccountCreateUpdateProperties {
    #[serde(flatten)]
    pub database_account_create_update_properties: DatabaseAccountCreateUpdateProperties,
    #[doc = "Parameters to indicate the information about the restore."]
    #[serde(rename = "restoreParameters", default, skip_serializing_if = "Option::is_none")]
    pub restore_parameters: Option<RestoreParameters>,
}
impl RestoreReqeustDatabaseAccountCreateUpdateProperties {
    pub fn new(database_account_create_update_properties: DatabaseAccountCreateUpdateProperties) -> Self {
        Self {
            database_account_create_update_properties,
            restore_parameters: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SeedNode {
    #[doc = "IP address of this seed node."]
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}
impl SeedNode {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The shard key and partition kind pair, only support \"Hash\" partition kind"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShardKeys {}
impl ShardKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SpatialSpec {
    #[doc = "The path for which the indexing behavior applies to. Index paths typically start with root and end with wildcard (/path/*)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "List of path's spatial type"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<SpatialType>,
}
impl SpatialSpec {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Indicates the spatial type of index."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SpatialType")]
pub enum SpatialType {
    Point,
    LineString,
    Polygon,
    MultiPolygon,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SpatialType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SpatialType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SpatialType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Point => serializer.serialize_unit_variant("SpatialType", 0u32, "Point"),
            Self::LineString => serializer.serialize_unit_variant("SpatialType", 1u32, "LineString"),
            Self::Polygon => serializer.serialize_unit_variant("SpatialType", 2u32, "Polygon"),
            Self::MultiPolygon => serializer.serialize_unit_variant("SpatialType", 3u32, "MultiPolygon"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Parameters to create and update Cosmos DB container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlContainerCreateUpdateParameters {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "Properties to create and update Azure Cosmos DB container."]
    pub properties: SqlContainerCreateUpdateProperties,
}
impl SqlContainerCreateUpdateParameters {
    pub fn new(properties: SqlContainerCreateUpdateProperties) -> Self {
        Self {
            arm_resource_properties: ArmResourceProperties::default(),
            properties,
        }
    }
}
#[doc = "Properties to create and update Azure Cosmos DB container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlContainerCreateUpdateProperties {
    #[doc = "Cosmos DB SQL container resource object"]
    pub resource: SqlContainerResource,
    #[doc = "CreateUpdateOptions are a list of key-value pairs that describe the resource. Supported keys are \"If-Match\", \"If-None-Match\", \"Session-Token\" and \"Throughput\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<CreateUpdateOptions>,
}
impl SqlContainerCreateUpdateProperties {
    pub fn new(resource: SqlContainerResource) -> Self {
        Self { resource, options: None }
    }
}
#[doc = "The properties of an Azure Cosmos DB container"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlContainerGetProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}
impl SqlContainerGetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Cosmos DB container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlContainerGetResults {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "The properties of an Azure Cosmos DB container"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlContainerGetProperties>,
}
impl SqlContainerGetResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List operation response, that contains the containers and their properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlContainerListResult {
    #[doc = "List of containers and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlContainerGetResults>,
}
impl azure_core::Continuable for SqlContainerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl SqlContainerListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cosmos DB SQL container resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlContainerResource {
    #[doc = "Name of the Cosmos DB SQL container"]
    pub id: String,
    #[doc = "Cosmos DB indexing policy"]
    #[serde(rename = "indexingPolicy", default, skip_serializing_if = "Option::is_none")]
    pub indexing_policy: Option<IndexingPolicy>,
    #[doc = "The configuration of the partition key to be used for partitioning data into multiple partitions"]
    #[serde(rename = "partitionKey", default, skip_serializing_if = "Option::is_none")]
    pub partition_key: Option<ContainerPartitionKey>,
    #[doc = "Default time to live"]
    #[serde(rename = "defaultTtl", default, skip_serializing_if = "Option::is_none")]
    pub default_ttl: Option<i64>,
    #[doc = "The unique key policy configuration for specifying uniqueness constraints on documents in the collection in the Azure Cosmos DB service."]
    #[serde(rename = "uniqueKeyPolicy", default, skip_serializing_if = "Option::is_none")]
    pub unique_key_policy: Option<UniqueKeyPolicy>,
    #[doc = "The conflict resolution policy for the container."]
    #[serde(rename = "conflictResolutionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub conflict_resolution_policy: Option<ConflictResolutionPolicy>,
    #[doc = "Analytical TTL."]
    #[serde(rename = "analyticalStorageTtl", default, skip_serializing_if = "Option::is_none")]
    pub analytical_storage_ttl: Option<i64>,
}
impl SqlContainerResource {
    pub fn new(id: String) -> Self {
        Self {
            id,
            indexing_policy: None,
            partition_key: None,
            default_ttl: None,
            unique_key_policy: None,
            conflict_resolution_policy: None,
            analytical_storage_ttl: None,
        }
    }
}
#[doc = "Parameters to create and update Cosmos DB SQL database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDatabaseCreateUpdateParameters {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "Properties to create and update Azure Cosmos DB SQL database."]
    pub properties: SqlDatabaseCreateUpdateProperties,
}
impl SqlDatabaseCreateUpdateParameters {
    pub fn new(properties: SqlDatabaseCreateUpdateProperties) -> Self {
        Self {
            arm_resource_properties: ArmResourceProperties::default(),
            properties,
        }
    }
}
#[doc = "Properties to create and update Azure Cosmos DB SQL database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDatabaseCreateUpdateProperties {
    #[doc = "Cosmos DB SQL database resource object"]
    pub resource: SqlDatabaseResource,
    #[doc = "CreateUpdateOptions are a list of key-value pairs that describe the resource. Supported keys are \"If-Match\", \"If-None-Match\", \"Session-Token\" and \"Throughput\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<CreateUpdateOptions>,
}
impl SqlDatabaseCreateUpdateProperties {
    pub fn new(resource: SqlDatabaseResource) -> Self {
        Self { resource, options: None }
    }
}
#[doc = "The properties of an Azure Cosmos DB SQL database"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlDatabaseGetProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<sql_database_get_properties::Resource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}
impl SqlDatabaseGetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_database_get_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Resource {
        #[serde(flatten)]
        pub sql_database_resource: SqlDatabaseResource,
        #[serde(flatten)]
        pub extended_resource_properties: ExtendedResourceProperties,
        #[doc = "A system generated property that specified the addressable path of the collections resource."]
        #[serde(rename = "_colls", default, skip_serializing_if = "Option::is_none")]
        pub colls: Option<String>,
        #[doc = "A system generated property that specifies the addressable path of the users resource."]
        #[serde(rename = "_users", default, skip_serializing_if = "Option::is_none")]
        pub users: Option<String>,
    }
    impl Resource {
        pub fn new(sql_database_resource: SqlDatabaseResource) -> Self {
            Self {
                sql_database_resource,
                extended_resource_properties: ExtendedResourceProperties::default(),
                colls: None,
                users: None,
            }
        }
    }
}
#[doc = "An Azure Cosmos DB SQL database."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlDatabaseGetResults {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "The properties of an Azure Cosmos DB SQL database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlDatabaseGetProperties>,
}
impl SqlDatabaseGetResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List operation response, that contains the SQL databases and their properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlDatabaseListResult {
    #[doc = "List of SQL databases and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlDatabaseGetResults>,
}
impl azure_core::Continuable for SqlDatabaseListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl SqlDatabaseListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cosmos DB SQL database resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlDatabaseResource {
    #[doc = "Name of the Cosmos DB SQL database"]
    pub id: String,
}
impl SqlDatabaseResource {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "Parameters to create and update an Azure Cosmos DB SQL Role Assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlRoleAssignmentCreateUpdateParameters {
    #[doc = "Azure Cosmos DB SQL Role Assignment resource object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlRoleAssignmentResource>,
}
impl SqlRoleAssignmentCreateUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Cosmos DB Role Assignment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlRoleAssignmentGetResults {
    #[serde(flatten)]
    pub arm_proxy_resource: ArmProxyResource,
    #[doc = "Azure Cosmos DB SQL Role Assignment resource object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlRoleAssignmentResource>,
}
impl SqlRoleAssignmentGetResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The relevant Role Assignments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlRoleAssignmentListResult {
    #[doc = "List of Role Assignments and their properties"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlRoleAssignmentGetResults>,
}
impl azure_core::Continuable for SqlRoleAssignmentListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl SqlRoleAssignmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Cosmos DB SQL Role Assignment resource object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlRoleAssignmentResource {
    #[doc = "The unique identifier for the associated Role Definition."]
    #[serde(rename = "roleDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[doc = "The data plane resource path for which access is being granted through this Role Assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The unique identifier for the associated AAD principal in the AAD graph to which access is being granted through this Role Assignment. Tenant ID for the principal is inferred using the tenant associated with the subscription."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
}
impl SqlRoleAssignmentResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters to create and update an Azure Cosmos DB SQL Role Definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlRoleDefinitionCreateUpdateParameters {
    #[doc = "Azure Cosmos DB SQL Role Definition resource object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlRoleDefinitionResource>,
}
impl SqlRoleDefinitionCreateUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Cosmos DB SQL Role Definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlRoleDefinitionGetResults {
    #[serde(flatten)]
    pub arm_proxy_resource: ArmProxyResource,
    #[doc = "Azure Cosmos DB SQL Role Definition resource object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlRoleDefinitionResource>,
}
impl SqlRoleDefinitionGetResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The relevant Role Definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlRoleDefinitionListResult {
    #[doc = "List of Role Definitions and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlRoleDefinitionGetResults>,
}
impl azure_core::Continuable for SqlRoleDefinitionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl SqlRoleDefinitionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Azure Cosmos DB SQL Role Definition resource object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlRoleDefinitionResource {
    #[doc = "A user-friendly name for the Role Definition. Must be unique for the database account."]
    #[serde(rename = "roleName", default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[doc = "Indicates whether the Role Definition was built-in or user created."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<sql_role_definition_resource::Type>,
    #[doc = "A set of fully qualified Scopes at or below which Role Assignments may be created using this Role Definition. This will allow application of this Role Definition on the entire database account or any underlying Database / Collection. Must have at least one element. Scopes higher than Database account are not enforceable as assignable Scopes. Note that resources referenced in assignable Scopes need not exist."]
    #[serde(rename = "assignableScopes", default, skip_serializing_if = "Vec::is_empty")]
    pub assignable_scopes: Vec<String>,
    #[doc = "The set of operations allowed through this Role Definition."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<Permission>,
}
impl SqlRoleDefinitionResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_role_definition_resource {
    use super::*;
    #[doc = "Indicates whether the Role Definition was built-in or user created."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        BuiltInRole,
        CustomRole,
    }
}
#[doc = "Parameters to create and update Cosmos DB storedProcedure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlStoredProcedureCreateUpdateParameters {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "Properties to create and update Azure Cosmos DB storedProcedure."]
    pub properties: SqlStoredProcedureCreateUpdateProperties,
}
impl SqlStoredProcedureCreateUpdateParameters {
    pub fn new(properties: SqlStoredProcedureCreateUpdateProperties) -> Self {
        Self {
            arm_resource_properties: ArmResourceProperties::default(),
            properties,
        }
    }
}
#[doc = "Properties to create and update Azure Cosmos DB storedProcedure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlStoredProcedureCreateUpdateProperties {
    #[doc = "Cosmos DB SQL storedProcedure resource object"]
    pub resource: SqlStoredProcedureResource,
    #[doc = "CreateUpdateOptions are a list of key-value pairs that describe the resource. Supported keys are \"If-Match\", \"If-None-Match\", \"Session-Token\" and \"Throughput\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<CreateUpdateOptions>,
}
impl SqlStoredProcedureCreateUpdateProperties {
    pub fn new(resource: SqlStoredProcedureResource) -> Self {
        Self { resource, options: None }
    }
}
#[doc = "The properties of an Azure Cosmos DB StoredProcedure"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlStoredProcedureGetProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<serde_json::Value>,
}
impl SqlStoredProcedureGetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Cosmos DB storedProcedure."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlStoredProcedureGetResults {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "The properties of an Azure Cosmos DB StoredProcedure"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlStoredProcedureGetProperties>,
}
impl SqlStoredProcedureGetResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List operation response, that contains the storedProcedures and their properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlStoredProcedureListResult {
    #[doc = "List of storedProcedures and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlStoredProcedureGetResults>,
}
impl azure_core::Continuable for SqlStoredProcedureListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl SqlStoredProcedureListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cosmos DB SQL storedProcedure resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlStoredProcedureResource {
    #[doc = "Name of the Cosmos DB SQL storedProcedure"]
    pub id: String,
    #[doc = "Body of the Stored Procedure"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}
impl SqlStoredProcedureResource {
    pub fn new(id: String) -> Self {
        Self { id, body: None }
    }
}
#[doc = "Parameters to create and update Cosmos DB trigger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlTriggerCreateUpdateParameters {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "Properties to create and update Azure Cosmos DB trigger."]
    pub properties: SqlTriggerCreateUpdateProperties,
}
impl SqlTriggerCreateUpdateParameters {
    pub fn new(properties: SqlTriggerCreateUpdateProperties) -> Self {
        Self {
            arm_resource_properties: ArmResourceProperties::default(),
            properties,
        }
    }
}
#[doc = "Properties to create and update Azure Cosmos DB trigger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlTriggerCreateUpdateProperties {
    #[doc = "Cosmos DB SQL trigger resource object"]
    pub resource: SqlTriggerResource,
    #[doc = "CreateUpdateOptions are a list of key-value pairs that describe the resource. Supported keys are \"If-Match\", \"If-None-Match\", \"Session-Token\" and \"Throughput\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<CreateUpdateOptions>,
}
impl SqlTriggerCreateUpdateProperties {
    pub fn new(resource: SqlTriggerResource) -> Self {
        Self { resource, options: None }
    }
}
#[doc = "The properties of an Azure Cosmos DB trigger"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlTriggerGetProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<serde_json::Value>,
}
impl SqlTriggerGetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Cosmos DB trigger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlTriggerGetResults {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "The properties of an Azure Cosmos DB trigger"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlTriggerGetProperties>,
}
impl SqlTriggerGetResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List operation response, that contains the triggers and their properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlTriggerListResult {
    #[doc = "List of triggers and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlTriggerGetResults>,
}
impl azure_core::Continuable for SqlTriggerListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl SqlTriggerListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cosmos DB SQL trigger resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlTriggerResource {
    #[doc = "Name of the Cosmos DB SQL trigger"]
    pub id: String,
    #[doc = "Body of the Trigger"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[doc = "Type of the Trigger"]
    #[serde(rename = "triggerType", default, skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<sql_trigger_resource::TriggerType>,
    #[doc = "The operation the trigger is associated with"]
    #[serde(rename = "triggerOperation", default, skip_serializing_if = "Option::is_none")]
    pub trigger_operation: Option<sql_trigger_resource::TriggerOperation>,
}
impl SqlTriggerResource {
    pub fn new(id: String) -> Self {
        Self {
            id,
            body: None,
            trigger_type: None,
            trigger_operation: None,
        }
    }
}
pub mod sql_trigger_resource {
    use super::*;
    #[doc = "Type of the Trigger"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TriggerType")]
    pub enum TriggerType {
        Pre,
        Post,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TriggerType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TriggerType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TriggerType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Pre => serializer.serialize_unit_variant("TriggerType", 0u32, "Pre"),
                Self::Post => serializer.serialize_unit_variant("TriggerType", 1u32, "Post"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The operation the trigger is associated with"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "TriggerOperation")]
    pub enum TriggerOperation {
        All,
        Create,
        Update,
        Delete,
        Replace,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for TriggerOperation {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for TriggerOperation {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for TriggerOperation {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::All => serializer.serialize_unit_variant("TriggerOperation", 0u32, "All"),
                Self::Create => serializer.serialize_unit_variant("TriggerOperation", 1u32, "Create"),
                Self::Update => serializer.serialize_unit_variant("TriggerOperation", 2u32, "Update"),
                Self::Delete => serializer.serialize_unit_variant("TriggerOperation", 3u32, "Delete"),
                Self::Replace => serializer.serialize_unit_variant("TriggerOperation", 4u32, "Replace"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Parameters to create and update Cosmos DB userDefinedFunction."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlUserDefinedFunctionCreateUpdateParameters {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "Properties to create and update Azure Cosmos DB userDefinedFunction."]
    pub properties: SqlUserDefinedFunctionCreateUpdateProperties,
}
impl SqlUserDefinedFunctionCreateUpdateParameters {
    pub fn new(properties: SqlUserDefinedFunctionCreateUpdateProperties) -> Self {
        Self {
            arm_resource_properties: ArmResourceProperties::default(),
            properties,
        }
    }
}
#[doc = "Properties to create and update Azure Cosmos DB userDefinedFunction."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlUserDefinedFunctionCreateUpdateProperties {
    #[doc = "Cosmos DB SQL userDefinedFunction resource object"]
    pub resource: SqlUserDefinedFunctionResource,
    #[doc = "CreateUpdateOptions are a list of key-value pairs that describe the resource. Supported keys are \"If-Match\", \"If-None-Match\", \"Session-Token\" and \"Throughput\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<CreateUpdateOptions>,
}
impl SqlUserDefinedFunctionCreateUpdateProperties {
    pub fn new(resource: SqlUserDefinedFunctionResource) -> Self {
        Self { resource, options: None }
    }
}
#[doc = "The properties of an Azure Cosmos DB userDefinedFunction"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlUserDefinedFunctionGetProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<serde_json::Value>,
}
impl SqlUserDefinedFunctionGetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Cosmos DB userDefinedFunction."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlUserDefinedFunctionGetResults {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "The properties of an Azure Cosmos DB userDefinedFunction"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlUserDefinedFunctionGetProperties>,
}
impl SqlUserDefinedFunctionGetResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List operation response, that contains the userDefinedFunctions and their properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlUserDefinedFunctionListResult {
    #[doc = "List of userDefinedFunctions and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlUserDefinedFunctionGetResults>,
}
impl azure_core::Continuable for SqlUserDefinedFunctionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl SqlUserDefinedFunctionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cosmos DB SQL userDefinedFunction resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlUserDefinedFunctionResource {
    #[doc = "Name of the Cosmos DB SQL userDefinedFunction"]
    pub id: String,
    #[doc = "Body of the User Defined Function"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}
impl SqlUserDefinedFunctionResource {
    pub fn new(id: String) -> Self {
        Self { id, body: None }
    }
}
#[doc = "Parameters to create and update Cosmos DB Table."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableCreateUpdateParameters {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "Properties to create and update Azure Cosmos DB Table."]
    pub properties: TableCreateUpdateProperties,
}
impl TableCreateUpdateParameters {
    pub fn new(properties: TableCreateUpdateProperties) -> Self {
        Self {
            arm_resource_properties: ArmResourceProperties::default(),
            properties,
        }
    }
}
#[doc = "Properties to create and update Azure Cosmos DB Table."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableCreateUpdateProperties {
    #[doc = "Cosmos DB table resource object"]
    pub resource: TableResource,
    #[doc = "CreateUpdateOptions are a list of key-value pairs that describe the resource. Supported keys are \"If-Match\", \"If-None-Match\", \"Session-Token\" and \"Throughput\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<CreateUpdateOptions>,
}
impl TableCreateUpdateProperties {
    pub fn new(resource: TableResource) -> Self {
        Self { resource, options: None }
    }
}
#[doc = "The properties of an Azure Cosmos Table"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableGetProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
}
impl TableGetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Cosmos DB Table."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableGetResults {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "The properties of an Azure Cosmos Table"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TableGetProperties>,
}
impl TableGetResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List operation response, that contains the Table and their properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableListResult {
    #[doc = "List of Table and their properties."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TableGetResults>,
}
impl azure_core::Continuable for TableListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl TableListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cosmos DB table resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableResource {
    #[doc = "Name of the Cosmos DB table"]
    pub id: String,
}
impl TableResource {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[doc = "Tags are a list of key-value pairs that describe the resource. These tags can be used in viewing and grouping this resource (across resource groups). A maximum of 15 tags can be provided for a resource. Each tag must have a key no greater than 128 characters and value no greater than 256 characters. For example, the default experience for a template type is set with \"defaultExperience\": \"Cassandra\". Current \"defaultExperience\" values also include \"Table\", \"Graph\", \"DocumentDB\", and \"MongoDB\"."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tags {}
impl Tags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cosmos DB resource throughput policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ThroughputPolicyResource {
    #[doc = "Determines whether the ThroughputPolicy is active or not"]
    #[serde(rename = "isEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[doc = "Represents the percentage by which throughput can increase every time throughput policy kicks in."]
    #[serde(rename = "incrementPercent", default, skip_serializing_if = "Option::is_none")]
    pub increment_percent: Option<i64>,
}
impl ThroughputPolicyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of an Azure Cosmos DB resource throughput"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ThroughputSettingsGetProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<serde_json::Value>,
}
impl ThroughputSettingsGetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Cosmos DB resource throughput."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ThroughputSettingsGetResults {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "The properties of an Azure Cosmos DB resource throughput"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ThroughputSettingsGetProperties>,
}
impl ThroughputSettingsGetResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Cosmos DB resource throughput object. Either throughput is required or autoscaleSettings is required, but not both."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ThroughputSettingsResource {
    #[doc = "Value of the Cosmos DB resource throughput. Either throughput is required or autoscaleSettings is required, but not both."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throughput: Option<i64>,
    #[doc = "Cosmos DB provisioned throughput settings object"]
    #[serde(rename = "autoscaleSettings", default, skip_serializing_if = "Option::is_none")]
    pub autoscale_settings: Option<AutoscaleSettingsResource>,
    #[doc = "The minimum throughput of the resource"]
    #[serde(rename = "minimumThroughput", default, skip_serializing_if = "Option::is_none")]
    pub minimum_throughput: Option<String>,
    #[doc = "The throughput replace is pending"]
    #[serde(rename = "offerReplacePending", default, skip_serializing_if = "Option::is_none")]
    pub offer_replace_pending: Option<String>,
}
impl ThroughputSettingsResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters to update Cosmos DB resource throughput."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThroughputSettingsUpdateParameters {
    #[serde(flatten)]
    pub arm_resource_properties: ArmResourceProperties,
    #[doc = "Properties to update Azure Cosmos DB resource throughput."]
    pub properties: ThroughputSettingsUpdateProperties,
}
impl ThroughputSettingsUpdateParameters {
    pub fn new(properties: ThroughputSettingsUpdateProperties) -> Self {
        Self {
            arm_resource_properties: ArmResourceProperties::default(),
            properties,
        }
    }
}
#[doc = "Properties to update Azure Cosmos DB resource throughput."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThroughputSettingsUpdateProperties {
    #[doc = "Cosmos DB resource throughput object. Either throughput is required or autoscaleSettings is required, but not both."]
    pub resource: ThroughputSettingsResource,
}
impl ThroughputSettingsUpdateProperties {
    pub fn new(resource: ThroughputSettingsResource) -> Self {
        Self { resource }
    }
}
#[doc = "The unique key on that enforces uniqueness constraint on documents in the collection in the Azure Cosmos DB service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UniqueKey {
    #[doc = "List of paths must be unique for each document in the Azure Cosmos DB service"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<Path>,
}
impl UniqueKey {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The unique key policy configuration for specifying uniqueness constraints on documents in the collection in the Azure Cosmos DB service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UniqueKeyPolicy {
    #[doc = "List of unique keys on that enforces uniqueness constraint on documents in the collection in the Azure Cosmos DB service."]
    #[serde(rename = "uniqueKeys", default, skip_serializing_if = "Vec::is_empty")]
    pub unique_keys: Vec<UniqueKey>,
}
impl UniqueKeyPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The unit of the metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "UnitType")]
pub enum UnitType {
    Count,
    Bytes,
    Seconds,
    Percent,
    CountPerSecond,
    BytesPerSecond,
    Milliseconds,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for UnitType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for UnitType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for UnitType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Count => serializer.serialize_unit_variant("UnitType", 0u32, "Count"),
            Self::Bytes => serializer.serialize_unit_variant("UnitType", 1u32, "Bytes"),
            Self::Seconds => serializer.serialize_unit_variant("UnitType", 2u32, "Seconds"),
            Self::Percent => serializer.serialize_unit_variant("UnitType", 3u32, "Percent"),
            Self::CountPerSecond => serializer.serialize_unit_variant("UnitType", 4u32, "CountPerSecond"),
            Self::BytesPerSecond => serializer.serialize_unit_variant("UnitType", 5u32, "BytesPerSecond"),
            Self::Milliseconds => serializer.serialize_unit_variant("UnitType", 6u32, "Milliseconds"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The usage data for a usage request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Usage {
    #[doc = "The unit of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<UnitType>,
    #[doc = "A metric name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<MetricName>,
    #[doc = "The quota period used to summarize the usage values."]
    #[serde(rename = "quotaPeriod", default, skip_serializing_if = "Option::is_none")]
    pub quota_period: Option<String>,
    #[doc = "Maximum value for this metric"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[doc = "Current value for this metric"]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
}
impl Usage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response to a list usage request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsagesResult {
    #[doc = "The list of usages for the database. A usage is a point in time metric"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Usage>,
}
impl azure_core::Continuable for UsagesResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl UsagesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Virtual Network ACL Rule object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VirtualNetworkRule {
    #[doc = "Resource ID of a subnet, for example: /subscriptions/{subscriptionId}/resourceGroups/{groupName}/providers/Microsoft.Network/virtualNetworks/{virtualNetworkName}/subnets/{subnetName}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Create firewall rule before the virtual network has vnet service endpoint enabled."]
    #[serde(rename = "ignoreMissingVNetServiceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing_v_net_service_endpoint: Option<bool>,
}
impl VirtualNetworkRule {
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
