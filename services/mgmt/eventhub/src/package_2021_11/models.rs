#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Namespace/EventHub Connection String"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessKeys {
    #[doc = "Primary connection string of the created namespace AuthorizationRule."]
    #[serde(rename = "primaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub primary_connection_string: Option<String>,
    #[doc = "Secondary connection string of the created namespace AuthorizationRule."]
    #[serde(rename = "secondaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub secondary_connection_string: Option<String>,
    #[doc = "Primary connection string of the alias if GEO DR is enabled"]
    #[serde(rename = "aliasPrimaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub alias_primary_connection_string: Option<String>,
    #[doc = "Secondary  connection string of the alias if GEO DR is enabled"]
    #[serde(rename = "aliasSecondaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub alias_secondary_connection_string: Option<String>,
    #[doc = "A base64-encoded 256-bit primary key for signing and validating the SAS token."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "A base64-encoded 256-bit primary key for signing and validating the SAS token."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[doc = "A string that describes the AuthorizationRule."]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
}
impl AccessKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Single item in List or Get Alias(Disaster Recovery configuration) operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmDisasterRecovery {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties required to the Create Or Update Alias(Disaster Recovery configurations)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<arm_disaster_recovery::Properties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ArmDisasterRecovery {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod arm_disaster_recovery {
    use super::*;
    #[doc = "Properties required to the Create Or Update Alias(Disaster Recovery configurations)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Provisioning state of the Alias(Disaster Recovery configuration) - possible values 'Accepted' or 'Succeeded' or 'Failed'"]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<properties::ProvisioningState>,
        #[doc = "ARM Id of the Primary/Secondary eventhub namespace name, which is part of GEO DR pairing"]
        #[serde(rename = "partnerNamespace", default, skip_serializing_if = "Option::is_none")]
        pub partner_namespace: Option<String>,
        #[doc = "Alternate name specified when alias and namespace names are same."]
        #[serde(rename = "alternateName", default, skip_serializing_if = "Option::is_none")]
        pub alternate_name: Option<String>,
        #[doc = "role of namespace in GEO DR - possible values 'Primary' or 'PrimaryNotReplicating' or 'Secondary'"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub role: Option<properties::Role>,
        #[doc = "Number of entities pending to be replicated."]
        #[serde(rename = "pendingReplicationOperationsCount", default, skip_serializing_if = "Option::is_none")]
        pub pending_replication_operations_count: Option<i64>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Provisioning state of the Alias(Disaster Recovery configuration) - possible values 'Accepted' or 'Succeeded' or 'Failed'"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum ProvisioningState {
            Accepted,
            Succeeded,
            Failed,
        }
        #[doc = "role of namespace in GEO DR - possible values 'Primary' or 'PrimaryNotReplicating' or 'Secondary'"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Role {
            Primary,
            PrimaryNotReplicating,
            Secondary,
        }
    }
}
#[doc = "The result of the List Alias(Disaster Recovery configuration) operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ArmDisasterRecoveryListResult {
    #[doc = "List of Alias(Disaster Recovery configurations)"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ArmDisasterRecovery>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of Alias(Disaster Recovery configuration)"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ArmDisasterRecoveryListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ArmDisasterRecoveryListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Single item in a List or Get AuthorizationRule operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties supplied to create or update AuthorizationRule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<authorization_rule::Properties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl AuthorizationRule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod authorization_rule {
    use super::*;
    #[doc = "Properties supplied to create or update AuthorizationRule"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "The rights associated with the rule."]
        pub rights: Vec<String>,
    }
    impl Properties {
        pub fn new(rights: Vec<String>) -> Self {
            Self { rights }
        }
    }
}
#[doc = "The response from the List namespace operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthorizationRuleListResult {
    #[doc = "Result of the List Authorization Rules operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AuthorizationRule>,
    #[doc = "Link to the next set of results. Not empty if Value contains an incomplete list of Authorization Rules"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AuthorizationRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl AuthorizationRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Pre-provisioned and readily available Event Hubs Cluster count per region."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableCluster {
    #[doc = "Location fo the Available Cluster"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl AvailableCluster {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of the List Available Clusters operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableClustersList {
    #[doc = "The count of readily available and pre-provisioned Event Hubs Clusters per region."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AvailableCluster>,
}
impl AvailableClustersList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties to configure capture description for eventhub"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CaptureDescription {
    #[doc = "A value that indicates whether capture description is enabled. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Enumerates the possible values for the encoding format of capture description. Note: 'AvroDeflate' will be deprecated in New API Version"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<capture_description::Encoding>,
    #[doc = "The time window allows you to set the frequency with which the capture to Azure Blobs will happen, value should between 60 to 900 seconds"]
    #[serde(rename = "intervalInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub interval_in_seconds: Option<i32>,
    #[doc = "The size window defines the amount of data built up in your Event Hub before an capture operation, value should be between 10485760 to 524288000 bytes"]
    #[serde(rename = "sizeLimitInBytes", default, skip_serializing_if = "Option::is_none")]
    pub size_limit_in_bytes: Option<i32>,
    #[doc = "Capture storage details for capture description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<Destination>,
    #[doc = "A value that indicates whether to Skip Empty Archives"]
    #[serde(rename = "skipEmptyArchives", default, skip_serializing_if = "Option::is_none")]
    pub skip_empty_archives: Option<bool>,
}
impl CaptureDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod capture_description {
    use super::*;
    #[doc = "Enumerates the possible values for the encoding format of capture description. Note: 'AvroDeflate' will be deprecated in New API Version"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Encoding {
        Avro,
        AvroDeflate,
    }
}
#[doc = "Parameter supplied to check Namespace name availability operation "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityParameter {
    #[doc = "Name to check the namespace name availability"]
    pub name: String,
}
impl CheckNameAvailabilityParameter {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[doc = "The Result of the CheckNameAvailability operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[doc = "The detailed info regarding the reason associated with the Namespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Value indicating Namespace is availability, true if the Namespace is available; otherwise, false."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "Specifies the reason for the unavailability of the service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<UnavailableReason>,
}
impl CheckNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Single Event Hubs Cluster resource in List or Get operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Cluster {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "SKU parameters particular to a cluster instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ClusterSku>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Event Hubs Cluster properties supplied in responses in List or Get operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<cluster::Properties>,
}
impl Cluster {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cluster {
    use super::*;
    #[doc = "Event Hubs Cluster properties supplied in responses in List or Get operations."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "The UTC time when the Event Hubs Cluster was created."]
        #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
        pub created_at: Option<String>,
        #[doc = "The UTC time when the Event Hubs Cluster was last updated."]
        #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<String>,
        #[doc = "The metric ID of the cluster resource. Provided by the service and not modifiable by the user."]
        #[serde(rename = "metricId", default, skip_serializing_if = "Option::is_none")]
        pub metric_id: Option<String>,
        #[doc = "Status of the Cluster resource"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The response of the List Event Hubs Clusters operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterListResult {
    #[doc = "The Event Hubs Clusters present in the List Event Hubs operation results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Cluster>,
    #[doc = "Link to the next set of results. Empty unless the value parameter contains an incomplete list of Event Hubs Clusters."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ClusterListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ClusterListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Contains all settings for the cluster."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterQuotaConfigurationProperties {
    #[doc = "All possible Cluster settings - a collection of key/value paired settings which apply to quotas and configurations imposed on the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
}
impl ClusterQuotaConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SKU parameters particular to a cluster instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterSku {
    #[doc = "Name of this SKU."]
    pub name: cluster_sku::Name,
    #[doc = "The quantity of Event Hubs Cluster Capacity Units contained in this cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl ClusterSku {
    pub fn new(name: cluster_sku::Name) -> Self {
        Self { name, capacity: None }
    }
}
pub mod cluster_sku {
    use super::*;
    #[doc = "Name of this SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        Dedicated,
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
                Self::Dedicated => serializer.serialize_unit_variant("Name", 0u32, "Dedicated"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "ConnectionState information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionState {
    #[doc = "Status of the connection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<connection_state::Status>,
    #[doc = "Description of the connection state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod connection_state {
    use super::*;
    #[doc = "Status of the connection."]
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
#[doc = "Single item in List or Get Consumer group operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsumerGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Single item in List or Get Consumer group operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<consumer_group::Properties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl ConsumerGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod consumer_group {
    use super::*;
    #[doc = "Single item in List or Get Consumer group operation"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Exact time the message was created."]
        #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        #[doc = "The exact time the message was updated."]
        #[serde(rename = "updatedAt", default, with = "azure_core::date::rfc3339::option")]
        pub updated_at: Option<time::OffsetDateTime>,
        #[doc = "User Metadata is a placeholder to store user-defined string data with maximum length 1024. e.g. it can be used to store descriptive data, such as list of teams and their contact information also user-defined configuration settings can be stored."]
        #[serde(rename = "userMetadata", default, skip_serializing_if = "Option::is_none")]
        pub user_metadata: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The result to the List Consumer Group operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsumerGroupListResult {
    #[doc = "Result of the List Consumer Group operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConsumerGroup>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of Consumer Group"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ConsumerGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl ConsumerGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Capture storage details for capture description"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Destination {
    #[doc = "Name for capture destination"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Properties describing the storage account, blob container and archive name format for capture destination"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<destination::Properties>,
}
impl Destination {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod destination {
    use super::*;
    #[doc = "Properties describing the storage account, blob container and archive name format for capture destination"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Resource id of the storage account to be used to create the blobs"]
        #[serde(rename = "storageAccountResourceId", default, skip_serializing_if = "Option::is_none")]
        pub storage_account_resource_id: Option<String>,
        #[doc = "Blob container Name"]
        #[serde(rename = "blobContainer", default, skip_serializing_if = "Option::is_none")]
        pub blob_container: Option<String>,
        #[doc = "Blob naming convention for archive, e.g. {Namespace}/{EventHub}/{PartitionId}/{Year}/{Month}/{Day}/{Hour}/{Minute}/{Second}. Here all the parameters (Namespace,EventHub .. etc) are mandatory irrespective of order"]
        #[serde(rename = "archiveNameFormat", default, skip_serializing_if = "Option::is_none")]
        pub archive_name_format: Option<String>,
        #[doc = "Subscription Id of Azure Data Lake Store"]
        #[serde(rename = "dataLakeSubscriptionId", default, skip_serializing_if = "Option::is_none")]
        pub data_lake_subscription_id: Option<String>,
        #[doc = "The Azure Data Lake Store name for the captured events"]
        #[serde(rename = "dataLakeAccountName", default, skip_serializing_if = "Option::is_none")]
        pub data_lake_account_name: Option<String>,
        #[doc = "The destination folder path for the captured events"]
        #[serde(rename = "dataLakeFolderPath", default, skip_serializing_if = "Option::is_none")]
        pub data_lake_folder_path: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Single Namespace item in List or Get Operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EhNamespace {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "SKU parameters supplied to the create namespace operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Properties to configure Identity for Bring your Own Keys"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Namespace properties supplied for create namespace operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<eh_namespace::Properties>,
}
impl EhNamespace {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod eh_namespace {
    use super::*;
    #[doc = "Namespace properties supplied for create namespace operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Provisioning state of the Namespace."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<String>,
        #[doc = "Status of the Namespace."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
        #[doc = "The time the Namespace was created."]
        #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        #[doc = "The time the Namespace was updated."]
        #[serde(rename = "updatedAt", default, with = "azure_core::date::rfc3339::option")]
        pub updated_at: Option<time::OffsetDateTime>,
        #[doc = "Endpoint you can use to perform Service Bus operations."]
        #[serde(rename = "serviceBusEndpoint", default, skip_serializing_if = "Option::is_none")]
        pub service_bus_endpoint: Option<String>,
        #[doc = "Cluster ARM ID of the Namespace."]
        #[serde(rename = "clusterArmId", default, skip_serializing_if = "Option::is_none")]
        pub cluster_arm_id: Option<String>,
        #[doc = "Identifier for Azure Insights metrics."]
        #[serde(rename = "metricId", default, skip_serializing_if = "Option::is_none")]
        pub metric_id: Option<String>,
        #[doc = "Value that indicates whether AutoInflate is enabled for eventhub namespace."]
        #[serde(rename = "isAutoInflateEnabled", default, skip_serializing_if = "Option::is_none")]
        pub is_auto_inflate_enabled: Option<bool>,
        #[doc = "Upper limit of throughput units when AutoInflate is enabled, value should be within 0 to 20 throughput units. ( '0' if AutoInflateEnabled = true)"]
        #[serde(rename = "maximumThroughputUnits", default, skip_serializing_if = "Option::is_none")]
        pub maximum_throughput_units: Option<i32>,
        #[doc = "Value that indicates whether Kafka is enabled for eventhub namespace."]
        #[serde(rename = "kafkaEnabled", default, skip_serializing_if = "Option::is_none")]
        pub kafka_enabled: Option<bool>,
        #[doc = "Enabling this property creates a Standard Event Hubs Namespace in regions supported availability zones."]
        #[serde(rename = "zoneRedundant", default, skip_serializing_if = "Option::is_none")]
        pub zone_redundant: Option<bool>,
        #[doc = "Properties to configure Encryption"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub encryption: Option<Encryption>,
        #[doc = "List of private endpoint connections."]
        #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
        pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
        #[doc = "This property disables SAS authentication for the Event Hubs namespace."]
        #[serde(rename = "disableLocalAuth", default, skip_serializing_if = "Option::is_none")]
        pub disable_local_auth: Option<bool>,
        #[doc = "Alternate name specified when alias and namespace names are same."]
        #[serde(rename = "alternateName", default, skip_serializing_if = "Option::is_none")]
        pub alternate_name: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The full ARM ID of an Event Hubs Namespace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EhNamespaceIdContainer {
    #[doc = "id parameter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl EhNamespaceIdContainer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of the List Namespace IDs operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EhNamespaceIdListResult {
    #[doc = "Result of the List Namespace IDs operation"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EhNamespaceIdContainer>,
}
impl EhNamespaceIdListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of the List Namespace operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EhNamespaceListResult {
    #[doc = "Result of the List Namespace operation"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EhNamespace>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of namespaces."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EhNamespaceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EhNamespaceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties to configure Encryption"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Encryption {
    #[doc = "Properties of KeyVault"]
    #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub key_vault_properties: Vec<KeyVaultProperties>,
    #[doc = "Enumerates the possible value of keySource for Encryption"]
    #[serde(rename = "keySource", default, skip_serializing_if = "Option::is_none")]
    pub key_source: Option<encryption::KeySource>,
    #[doc = "Enable Infrastructure Encryption (Double Encryption)"]
    #[serde(rename = "requireInfrastructureEncryption", default, skip_serializing_if = "Option::is_none")]
    pub require_infrastructure_encryption: Option<bool>,
}
impl Encryption {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod encryption {
    use super::*;
    #[doc = "Enumerates the possible value of keySource for Encryption"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeySource {
        #[serde(rename = "Microsoft.KeyVault")]
        MicrosoftKeyVault,
    }
    impl Default for KeySource {
        fn default() -> Self {
            Self::MicrosoftKeyVault
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
#[doc = "Error response indicates Event Hub service is not able to process the incoming request. The reason is provided in the error message."]
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
#[doc = "The result of the List EventHubs operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventHubListResult {
    #[doc = "Result of the List EventHubs operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Eventhub>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of EventHubs."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EventHubListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl EventHubListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Single item in List or Get Event Hub operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Eventhub {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties supplied to the Create Or Update Event Hub operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<eventhub::Properties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Eventhub {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod eventhub {
    use super::*;
    #[doc = "Properties supplied to the Create Or Update Event Hub operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Current number of shards on the Event Hub."]
        #[serde(rename = "partitionIds", default, skip_serializing_if = "Vec::is_empty")]
        pub partition_ids: Vec<String>,
        #[doc = "Exact time the Event Hub was created."]
        #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
        pub created_at: Option<time::OffsetDateTime>,
        #[doc = "The exact time the message was updated."]
        #[serde(rename = "updatedAt", default, with = "azure_core::date::rfc3339::option")]
        pub updated_at: Option<time::OffsetDateTime>,
        #[doc = "Number of days to retain the events for this Event Hub, value should be 1 to 7 days"]
        #[serde(rename = "messageRetentionInDays", default, skip_serializing_if = "Option::is_none")]
        pub message_retention_in_days: Option<i64>,
        #[doc = "Number of partitions created for the Event Hub, allowed values are from 1 to 32 partitions."]
        #[serde(rename = "partitionCount", default, skip_serializing_if = "Option::is_none")]
        pub partition_count: Option<i64>,
        #[doc = "Enumerates the possible values for the status of the Event Hub."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<properties::Status>,
        #[doc = "Properties to configure capture description for eventhub"]
        #[serde(rename = "captureDescription", default, skip_serializing_if = "Option::is_none")]
        pub capture_description: Option<CaptureDescription>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Enumerates the possible values for the status of the Event Hub."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Status {
            Active,
            Disabled,
            Restoring,
            SendDisabled,
            ReceiveDisabled,
            Creating,
            Deleting,
            Renaming,
            Unknown,
        }
    }
}
#[doc = "Properties to configure Identity for Bring your Own Keys"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[doc = "ObjectId from the KeyVault"]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "TenantId from the KeyVault"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Type of managed service identity."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
    #[doc = "Properties for User Assigned Identities"]
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
    #[doc = "Type of managed service identity."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
        None,
    }
}
#[doc = "Properties to configure keyVault Properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultProperties {
    #[doc = "Name of the Key from KeyVault"]
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[doc = "Uri of KeyVault"]
    #[serde(rename = "keyVaultUri", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_uri: Option<String>,
    #[doc = "Key Version"]
    #[serde(rename = "keyVersion", default, skip_serializing_if = "Option::is_none")]
    pub key_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<UserAssignedIdentityProperties>,
}
impl KeyVaultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response from the List namespace operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NwRuleSetIpRules {
    #[doc = "IP Mask"]
    #[serde(rename = "ipMask", default, skip_serializing_if = "Option::is_none")]
    pub ip_mask: Option<String>,
    #[doc = "The IP Filter Action"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<nw_rule_set_ip_rules::Action>,
}
impl NwRuleSetIpRules {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod nw_rule_set_ip_rules {
    use super::*;
    #[doc = "The IP Filter Action"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Action")]
    pub enum Action {
        Allow,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Action {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Action {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Action {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Allow => serializer.serialize_unit_variant("Action", 0u32, "Allow"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The response from the List namespace operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NwRuleSetVirtualNetworkRules {
    #[doc = "Properties supplied for Subnet"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<Subnet>,
    #[doc = "Value that indicates whether to ignore missing Vnet Service Endpoint"]
    #[serde(rename = "ignoreMissingVnetServiceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing_vnet_service_endpoint: Option<bool>,
}
impl NwRuleSetVirtualNetworkRules {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Description of topic resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkRuleSet {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "NetworkRuleSet properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<network_rule_set::Properties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl NetworkRuleSet {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_rule_set {
    use super::*;
    #[doc = "NetworkRuleSet properties"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Value that indicates whether Trusted Service Access is Enabled or not."]
        #[serde(rename = "trustedServiceAccessEnabled", default, skip_serializing_if = "Option::is_none")]
        pub trusted_service_access_enabled: Option<bool>,
        #[doc = "Default Action for Network Rule Set"]
        #[serde(rename = "defaultAction", default, skip_serializing_if = "Option::is_none")]
        pub default_action: Option<properties::DefaultAction>,
        #[doc = "List VirtualNetwork Rules"]
        #[serde(rename = "virtualNetworkRules", default, skip_serializing_if = "Vec::is_empty")]
        pub virtual_network_rules: Vec<NwRuleSetVirtualNetworkRules>,
        #[doc = "List of IpRules"]
        #[serde(rename = "ipRules", default, skip_serializing_if = "Vec::is_empty")]
        pub ip_rules: Vec<NwRuleSetIpRules>,
        #[doc = "This determines if traffic is allowed over public network. By default it is enabled."]
        #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
        pub public_network_access: Option<properties::PublicNetworkAccess>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Default Action for Network Rule Set"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "DefaultAction")]
        pub enum DefaultAction {
            Allow,
            Deny,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for DefaultAction {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for DefaultAction {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for DefaultAction {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Allow => serializer.serialize_unit_variant("DefaultAction", 0u32, "Allow"),
                    Self::Deny => serializer.serialize_unit_variant("DefaultAction", 1u32, "Deny"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        #[doc = "This determines if traffic is allowed over public network. By default it is enabled."]
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
        impl Default for PublicNetworkAccess {
            fn default() -> Self {
                Self::Enabled
            }
        }
    }
}
#[doc = "The response of the List NetworkRuleSet operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkRuleSetListResult {
    #[doc = "Result of the List NetworkRuleSet operation"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NetworkRuleSet>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of NetworkRuleSet."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl NetworkRuleSetListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A Event Hub REST API operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
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
    #[doc = "Properties of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Operation {
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
#[doc = "Result of the request to list Event Hub operations. It contains a list of operations and a URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Event Hub operations supported by the Microsoft.EventHub resource provider."]
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
#[doc = "PrivateEndpoint information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[doc = "The ARM identifier for Private Endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the PrivateEndpointConnection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of the private endpoint connection resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the list of all private endpoint connections operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[doc = "A collection of private endpoint connection resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
    #[doc = "A link for the next page of private endpoint connection resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PrivateEndpointConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PrivateEndpointConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the private endpoint connection resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionProperties {
    #[doc = "PrivateEndpoint information."]
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[doc = "ConnectionState information."]
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<ConnectionState>,
    #[doc = "Provisioning state of the Private Endpoint Connection."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<private_endpoint_connection_properties::ProvisioningState>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod private_endpoint_connection_properties {
    use super::*;
    #[doc = "Provisioning state of the Private Endpoint Connection."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Canceled,
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
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Deleting"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Succeeded"),
                Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Canceled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Information of the private link resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[doc = "Properties of PrivateLinkResource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
    #[doc = "Fully qualified identifier of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the resource"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of PrivateLinkResource"]
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
#[doc = "Result of the List private link resources operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourcesListResult {
    #[doc = "A collection of private link resources"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
    #[doc = "A link for the next page of private link resources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PrivateLinkResourcesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[doc = "Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.EventHub/Namespaces\" or \"Microsoft.EventHub/Namespaces/EventHubs\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The geo-location where the resource lives"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters supplied to the Regenerate Authorization Rule operation, specifies which key needs to be reset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegenerateAccessKeyParameters {
    #[doc = "The access key to regenerate."]
    #[serde(rename = "keyType")]
    pub key_type: regenerate_access_key_parameters::KeyType,
    #[doc = "Optional, if the key value provided, is set for KeyType or autogenerated Key value set for keyType"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
impl RegenerateAccessKeyParameters {
    pub fn new(key_type: regenerate_access_key_parameters::KeyType) -> Self {
        Self { key_type, key: None }
    }
}
pub mod regenerate_access_key_parameters {
    use super::*;
    #[doc = "The access key to regenerate."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "KeyType")]
    pub enum KeyType {
        PrimaryKey,
        SecondaryKey,
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
                Self::PrimaryKey => serializer.serialize_unit_variant("KeyType", 0u32, "PrimaryKey"),
                Self::SecondaryKey => serializer.serialize_unit_variant("KeyType", 1u32, "SecondaryKey"),
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
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Single item in List or Get Schema Group operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SchemaGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = ""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<schema_group::Properties>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl SchemaGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod schema_group {
    use super::*;
    #[doc = ""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[doc = "Exact time the Schema Group was updated"]
        #[serde(rename = "updatedAtUtc", default, with = "azure_core::date::rfc3339::option")]
        pub updated_at_utc: Option<time::OffsetDateTime>,
        #[doc = "Exact time the Schema Group was created."]
        #[serde(rename = "createdAtUtc", default, with = "azure_core::date::rfc3339::option")]
        pub created_at_utc: Option<time::OffsetDateTime>,
        #[doc = "The ETag value."]
        #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
        pub e_tag: Option<String>,
        #[doc = "dictionary object for SchemaGroup group properties"]
        #[serde(rename = "groupProperties", default, skip_serializing_if = "Option::is_none")]
        pub group_properties: Option<serde_json::Value>,
        #[doc = ""]
        #[serde(rename = "schemaCompatibility", default, skip_serializing_if = "Option::is_none")]
        pub schema_compatibility: Option<properties::SchemaCompatibility>,
        #[doc = ""]
        #[serde(rename = "schemaType", default, skip_serializing_if = "Option::is_none")]
        pub schema_type: Option<properties::SchemaType>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod properties {
        use super::*;
        #[doc = ""]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "SchemaCompatibility")]
        pub enum SchemaCompatibility {
            None,
            Backward,
            Forward,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for SchemaCompatibility {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for SchemaCompatibility {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for SchemaCompatibility {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::None => serializer.serialize_unit_variant("SchemaCompatibility", 0u32, "None"),
                    Self::Backward => serializer.serialize_unit_variant("SchemaCompatibility", 1u32, "Backward"),
                    Self::Forward => serializer.serialize_unit_variant("SchemaCompatibility", 2u32, "Forward"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
        #[doc = ""]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "SchemaType")]
        pub enum SchemaType {
            Unknown,
            Avro,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for SchemaType {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for SchemaType {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for SchemaType {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::Unknown => serializer.serialize_unit_variant("SchemaType", 0u32, "Unknown"),
                    Self::Avro => serializer.serialize_unit_variant("SchemaType", 1u32, "Avro"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "The result of the List SchemaGroup operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SchemaGroupListResult {
    #[doc = "Result of the List SchemaGroups operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SchemaGroup>,
    #[doc = "Link to the next set of results. Not empty if Value contains incomplete list of Schema Groups."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SchemaGroupListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl SchemaGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SKU parameters supplied to the create namespace operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "Name of this SKU."]
    pub name: sku::Name,
    #[doc = "The billing tier of this particular SKU."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sku::Tier>,
    #[doc = "The Event Hubs throughput units for Basic or Standard tiers, where value should be 0 to 20 throughput units. The Event Hubs premium units for Premium tier, where value should be 0 to 10 premium units."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl Sku {
    pub fn new(name: sku::Name) -> Self {
        Self {
            name,
            tier: None,
            capacity: None,
        }
    }
}
pub mod sku {
    use super::*;
    #[doc = "Name of this SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Name")]
    pub enum Name {
        Basic,
        Standard,
        Premium,
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
                Self::Basic => serializer.serialize_unit_variant("Name", 0u32, "Basic"),
                Self::Standard => serializer.serialize_unit_variant("Name", 1u32, "Standard"),
                Self::Premium => serializer.serialize_unit_variant("Name", 2u32, "Premium"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The billing tier of this particular SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Tier")]
    pub enum Tier {
        Basic,
        Standard,
        Premium,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Tier {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Tier {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Tier {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Basic => serializer.serialize_unit_variant("Tier", 0u32, "Basic"),
                Self::Standard => serializer.serialize_unit_variant("Tier", 1u32, "Standard"),
                Self::Premium => serializer.serialize_unit_variant("Tier", 2u32, "Premium"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Properties supplied for Subnet"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Subnet {
    #[doc = "Resource ID of Virtual Network Subnet"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl Subnet {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the reason for the unavailability of the service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UnavailableReason {
    None,
    InvalidName,
    SubscriptionIsDisabled,
    NameInUse,
    NameInLockdown,
    TooManyNamespaceInCurrentSubscription,
}
#[doc = "Recognized Dictionary value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "Principal Id of user assigned identity"]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "Client Id of user assigned identity"]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
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
    #[doc = "The type of identity that last modified the resource."]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentityProperties {
    #[doc = "ARM ID of user Identity selected for encryption"]
    #[serde(rename = "userAssignedIdentity", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identity: Option<String>,
}
impl UserAssignedIdentityProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
