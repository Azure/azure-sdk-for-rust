#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Parameters body to pass for resource name availability check."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityParameters {
    #[doc = "Resource name."]
    pub name: String,
    #[doc = "Resource type. The only legal value of this property for checking redis cache name availability is 'Microsoft.Cache/redis'."]
    #[serde(rename = "type")]
    pub type_: String,
}
impl CheckNameAvailabilityParameters {
    pub fn new(name: String, type_: String) -> Self {
        Self { name, type_ }
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
#[doc = "Parameters for Redis export operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportRdbParameters {
    #[doc = "File format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[doc = "Prefix to use for exported files."]
    pub prefix: String,
    #[doc = "Container name to export to."]
    pub container: String,
}
impl ExportRdbParameters {
    pub fn new(prefix: String, container: String) -> Self {
        Self {
            format: None,
            prefix,
            container,
        }
    }
}
#[doc = "Parameters for Redis import operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportRdbParameters {
    #[doc = "File format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[doc = "files to import."]
    pub files: Vec<String>,
}
impl ImportRdbParameters {
    pub fn new(files: Vec<String>) -> Self {
        Self { format: None, files }
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
#[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedServiceIdentityType")]
pub enum ManagedServiceIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
    #[serde(rename = "SystemAssigned, UserAssigned")]
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
                serializer.serialize_unit_variant("ManagedServiceIdentityType", 3u32, "SystemAssigned, UserAssigned")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The response of listUpgradeNotifications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationListResponse {
    #[doc = "List of all notifications."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UpgradeNotification>,
    #[doc = "Link for next set of notifications."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NotificationListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl NotificationListResponse {
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
    #[doc = "The object that describes the operation."]
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
    #[doc = "The object that describes the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Friendly name of the resource provider"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Operation type: read, write, delete, listKeys/action, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Resource type on which the operation is performed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Friendly name of the operation"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list REST API operations. It contains a list of operations and a URL nextLink to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the resource provider."]
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
#[doc = "Asynchronous operation status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatus {
    #[serde(flatten)]
    pub operation_status_result: OperationStatusResult,
    #[doc = "Additional properties from RP, only when operation is successful"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl OperationStatus {
    pub fn new(operation_status_result: OperationStatusResult) -> Self {
        Self {
            operation_status_result,
            properties: None,
        }
    }
}
#[doc = "The current status of an async operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatusResult {
    #[doc = "Fully qualified ID for the async operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the async operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Operation status."]
    pub status: String,
    #[doc = "Percent of the operation that is complete."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<f64>,
    #[doc = "The start time of the operation."]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "The end time of the operation."]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "The operations list."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operations: Vec<OperationStatusResult>,
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl OperationStatusResult {
    pub fn new(status: String) -> Self {
        Self {
            id: None,
            name: None,
            status,
            percent_complete: None,
            start_time: None,
            end_time: None,
            operations: Vec::new(),
            error: None,
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
#[doc = "Redis cache access keys."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RedisAccessKeys {
    #[doc = "The current primary key that clients can use to authenticate with Redis cache."]
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[doc = "The current secondary key that clients can use to authenticate with Redis cache."]
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
}
impl RedisAccessKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Create/Update/Get common properties of the redis cache."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RedisCommonProperties {
    #[doc = "All Redis Settings. Few possible keys: rdb-backup-enabled,rdb-storage-connection-string,rdb-backup-frequency,maxmemory-delta,maxmemory-policy,notify-keyspace-events,maxmemory-samples,slowlog-log-slower-than,slowlog-max-len,list-max-ziplist-entries,list-max-ziplist-value,hash-max-ziplist-entries,hash-max-ziplist-value,set-max-intset-entries,zset-max-ziplist-entries,zset-max-ziplist-value etc."]
    #[serde(rename = "redisConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub redis_configuration: Option<redis_common_properties::RedisConfiguration>,
    #[doc = "Redis version. Only major version will be used in PUT/PATCH request with current valid values: (4, 6)"]
    #[serde(rename = "redisVersion", default, skip_serializing_if = "Option::is_none")]
    pub redis_version: Option<String>,
    #[doc = "Specifies whether the non-ssl Redis server port (6379) is enabled."]
    #[serde(rename = "enableNonSslPort", default, skip_serializing_if = "Option::is_none")]
    pub enable_non_ssl_port: Option<bool>,
    #[doc = "The number of replicas to be created per primary."]
    #[serde(rename = "replicasPerMaster", default, skip_serializing_if = "Option::is_none")]
    pub replicas_per_master: Option<i32>,
    #[doc = "The number of replicas to be created per primary."]
    #[serde(rename = "replicasPerPrimary", default, skip_serializing_if = "Option::is_none")]
    pub replicas_per_primary: Option<i32>,
    #[doc = "A dictionary of tenant settings"]
    #[serde(rename = "tenantSettings", default, skip_serializing_if = "Option::is_none")]
    pub tenant_settings: Option<serde_json::Value>,
    #[doc = "The number of shards to be created on a Premium Cluster Cache."]
    #[serde(rename = "shardCount", default, skip_serializing_if = "Option::is_none")]
    pub shard_count: Option<i32>,
    #[doc = "Optional: requires clients to use a specified TLS version (or higher) to connect (e,g, '1.0', '1.1', '1.2')"]
    #[serde(rename = "minimumTlsVersion", default, skip_serializing_if = "Option::is_none")]
    pub minimum_tls_version: Option<redis_common_properties::MinimumTlsVersion>,
    #[doc = "Whether or not public endpoint access is allowed for this cache.  Value is optional but if passed in, must be 'Enabled' or 'Disabled'. If 'Disabled', private endpoints are the exclusive access method. Default value is 'Enabled'"]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<redis_common_properties::PublicNetworkAccess>,
}
impl RedisCommonProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod redis_common_properties {
    use super::*;
    #[doc = "All Redis Settings. Few possible keys: rdb-backup-enabled,rdb-storage-connection-string,rdb-backup-frequency,maxmemory-delta,maxmemory-policy,notify-keyspace-events,maxmemory-samples,slowlog-log-slower-than,slowlog-max-len,list-max-ziplist-entries,list-max-ziplist-value,hash-max-ziplist-entries,hash-max-ziplist-value,set-max-intset-entries,zset-max-ziplist-entries,zset-max-ziplist-value etc."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct RedisConfiguration {
        #[doc = "Specifies whether the rdb backup is enabled"]
        #[serde(rename = "rdb-backup-enabled", default, skip_serializing_if = "Option::is_none")]
        pub rdb_backup_enabled: Option<String>,
        #[doc = "Specifies the frequency for creating rdb backup"]
        #[serde(rename = "rdb-backup-frequency", default, skip_serializing_if = "Option::is_none")]
        pub rdb_backup_frequency: Option<String>,
        #[doc = "Specifies the maximum number of snapshots for rdb backup"]
        #[serde(rename = "rdb-backup-max-snapshot-count", default, skip_serializing_if = "Option::is_none")]
        pub rdb_backup_max_snapshot_count: Option<String>,
        #[doc = "The storage account connection string for storing rdb file"]
        #[serde(rename = "rdb-storage-connection-string", default, skip_serializing_if = "Option::is_none")]
        pub rdb_storage_connection_string: Option<String>,
        #[doc = "Specifies whether the aof backup is enabled"]
        #[serde(rename = "aof-backup-enabled", default, skip_serializing_if = "Option::is_none")]
        pub aof_backup_enabled: Option<String>,
        #[doc = "First storage account connection string"]
        #[serde(rename = "aof-storage-connection-string-0", default, skip_serializing_if = "Option::is_none")]
        pub aof_storage_connection_string_0: Option<String>,
        #[doc = "Second storage account connection string"]
        #[serde(rename = "aof-storage-connection-string-1", default, skip_serializing_if = "Option::is_none")]
        pub aof_storage_connection_string_1: Option<String>,
        #[doc = "Value in megabytes reserved for fragmentation per shard"]
        #[serde(rename = "maxfragmentationmemory-reserved", default, skip_serializing_if = "Option::is_none")]
        pub maxfragmentationmemory_reserved: Option<String>,
        #[doc = "The eviction strategy used when your data won't fit within its memory limit."]
        #[serde(rename = "maxmemory-policy", default, skip_serializing_if = "Option::is_none")]
        pub maxmemory_policy: Option<String>,
        #[doc = "Value in megabytes reserved for non-cache usage per shard e.g. failover."]
        #[serde(rename = "maxmemory-reserved", default, skip_serializing_if = "Option::is_none")]
        pub maxmemory_reserved: Option<String>,
        #[doc = "Value in megabytes reserved for non-cache usage per shard e.g. failover."]
        #[serde(rename = "maxmemory-delta", default, skip_serializing_if = "Option::is_none")]
        pub maxmemory_delta: Option<String>,
        #[doc = "The max clients config"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub maxclients: Option<String>,
        #[doc = "Preferred auth method to communicate to storage account used for data archive, specify SAS or ManagedIdentity, default value is SAS"]
        #[serde(rename = "preferred-data-archive-auth-method", default, skip_serializing_if = "Option::is_none")]
        pub preferred_data_archive_auth_method: Option<String>,
        #[doc = "Preferred auth method to communicate to storage account used for data persistence, specify SAS or ManagedIdentity, default value is SAS"]
        #[serde(
            rename = "preferred-data-persistence-auth-method",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub preferred_data_persistence_auth_method: Option<String>,
        #[doc = "Zonal Configuration"]
        #[serde(rename = "zonal-configuration", default, skip_serializing_if = "Option::is_none")]
        pub zonal_configuration: Option<String>,
        #[doc = "Specifies whether the authentication is disabled. Setting this property is highly discouraged from security point of view."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub authnotrequired: Option<String>,
    }
    impl RedisConfiguration {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "Optional: requires clients to use a specified TLS version (or higher) to connect (e,g, '1.0', '1.1', '1.2')"]
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
    #[doc = "Whether or not public endpoint access is allowed for this cache.  Value is optional but if passed in, must be 'Enabled' or 'Disabled'. If 'Disabled', private endpoints are the exclusive access method. Default value is 'Enabled'"]
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
#[doc = "Parameters supplied to the Create Redis operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisCreateParameters {
    #[doc = "Properties supplied to Create Redis operation."]
    pub properties: RedisCreateProperties,
    #[doc = "A list of availability zones denoting where the resource needs to come from."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
    #[doc = "The geo-location where the resource lives"]
    pub location: String,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl RedisCreateParameters {
    pub fn new(properties: RedisCreateProperties, location: String) -> Self {
        Self {
            properties,
            zones: Vec::new(),
            location,
            tags: None,
            identity: None,
        }
    }
}
#[doc = "Properties supplied to Create Redis operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisCreateProperties {
    #[serde(flatten)]
    pub redis_common_properties: RedisCommonProperties,
    #[doc = "SKU parameters supplied to the create Redis operation."]
    pub sku: Sku,
    #[doc = "The full resource ID of a subnet in a virtual network to deploy the Redis cache in. Example format: /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/Microsoft.{Network|ClassicNetwork}/VirtualNetworks/vnet1/subnets/subnet1"]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "Static IP address. Optionally, may be specified when deploying a Redis cache inside an existing Azure Virtual Network; auto assigned by default."]
    #[serde(rename = "staticIP", default, skip_serializing_if = "Option::is_none")]
    pub static_ip: Option<String>,
}
impl RedisCreateProperties {
    pub fn new(sku: Sku) -> Self {
        Self {
            redis_common_properties: RedisCommonProperties::default(),
            sku,
            subnet_id: None,
            static_ip: None,
        }
    }
}
#[doc = "A firewall rule on a redis cache has a name, and describes a contiguous range of IP addresses permitted to connect"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisFirewallRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Specifies a range of IP addresses permitted to connect to the cache"]
    pub properties: RedisFirewallRuleProperties,
}
impl RedisFirewallRule {
    pub fn new(properties: RedisFirewallRuleProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "Parameters required for creating a firewall rule on redis cache. (Note, you can just use the FirewallRule type instead now.)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisFirewallRuleCreateParameters {
    #[serde(flatten)]
    pub redis_firewall_rule: RedisFirewallRule,
}
impl RedisFirewallRuleCreateParameters {
    pub fn new(redis_firewall_rule: RedisFirewallRule) -> Self {
        Self { redis_firewall_rule }
    }
}
#[doc = "The response of list firewall rules Redis operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RedisFirewallRuleListResult {
    #[doc = "Results of the list firewall rules operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RedisFirewallRule>,
    #[doc = "Link for next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RedisFirewallRuleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RedisFirewallRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies a range of IP addresses permitted to connect to the cache"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisFirewallRuleProperties {
    #[doc = "lowest IP address included in the range"]
    #[serde(rename = "startIP")]
    pub start_ip: String,
    #[doc = "highest IP address included in the range"]
    #[serde(rename = "endIP")]
    pub end_ip: String,
}
impl RedisFirewallRuleProperties {
    pub fn new(start_ip: String, end_ip: String) -> Self {
        Self { start_ip, end_ip }
    }
}
#[doc = "Response to force reboot for Redis cache."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RedisForceRebootResponse {
    #[doc = "Status message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl RedisForceRebootResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of single instance of redis."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RedisInstanceDetails {
    #[doc = "Redis instance SSL port."]
    #[serde(rename = "sslPort", default, skip_serializing_if = "Option::is_none")]
    pub ssl_port: Option<i32>,
    #[doc = "If enableNonSslPort is true, provides Redis instance Non-SSL port."]
    #[serde(rename = "nonSslPort", default, skip_serializing_if = "Option::is_none")]
    pub non_ssl_port: Option<i32>,
    #[doc = "If the Cache uses availability zones, specifies availability zone where this instance is located."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
    #[doc = "If clustering is enabled, the Shard ID of Redis Instance"]
    #[serde(rename = "shardId", default, skip_serializing_if = "Option::is_none")]
    pub shard_id: Option<i32>,
    #[doc = "Specifies whether the instance is a primary node."]
    #[serde(rename = "isMaster", default, skip_serializing_if = "Option::is_none")]
    pub is_master: Option<bool>,
    #[doc = "Specifies whether the instance is a primary node."]
    #[serde(rename = "isPrimary", default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
}
impl RedisInstanceDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Linked server Id"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RedisLinkedServer {
    #[doc = "Linked server Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl RedisLinkedServer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameter required for creating a linked server to redis cache."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisLinkedServerCreateParameters {
    #[doc = "Create properties for a linked server"]
    pub properties: RedisLinkedServerCreateProperties,
}
impl RedisLinkedServerCreateParameters {
    pub fn new(properties: RedisLinkedServerCreateProperties) -> Self {
        Self { properties }
    }
}
#[doc = "Create properties for a linked server"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisLinkedServerCreateProperties {
    #[doc = "Fully qualified resourceId of the linked redis cache."]
    #[serde(rename = "linkedRedisCacheId")]
    pub linked_redis_cache_id: String,
    #[doc = "Location of the linked redis cache."]
    #[serde(rename = "linkedRedisCacheLocation")]
    pub linked_redis_cache_location: String,
    #[doc = "Role of the linked server."]
    #[serde(rename = "serverRole")]
    pub server_role: redis_linked_server_create_properties::ServerRole,
}
impl RedisLinkedServerCreateProperties {
    pub fn new(
        linked_redis_cache_id: String,
        linked_redis_cache_location: String,
        server_role: redis_linked_server_create_properties::ServerRole,
    ) -> Self {
        Self {
            linked_redis_cache_id,
            linked_redis_cache_location,
            server_role,
        }
    }
}
pub mod redis_linked_server_create_properties {
    use super::*;
    #[doc = "Role of the linked server."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ServerRole {
        Primary,
        Secondary,
    }
}
#[doc = "Properties of a linked server to be returned in get/put response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisLinkedServerProperties {
    #[serde(flatten)]
    pub redis_linked_server_create_properties: RedisLinkedServerCreateProperties,
    #[doc = "Terminal state of the link between primary and secondary redis cache."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl RedisLinkedServerProperties {
    pub fn new(redis_linked_server_create_properties: RedisLinkedServerCreateProperties) -> Self {
        Self {
            redis_linked_server_create_properties,
            provisioning_state: None,
        }
    }
}
#[doc = "Response to put/get linked server (with properties) for Redis cache."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RedisLinkedServerWithProperties {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a linked server to be returned in get/put response"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RedisLinkedServerProperties>,
}
impl RedisLinkedServerWithProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of linked servers (with properties) of a Redis cache."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RedisLinkedServerWithPropertiesList {
    #[doc = "List of linked servers (with properties) of a Redis cache."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RedisLinkedServerWithProperties>,
    #[doc = "Link for next set."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RedisLinkedServerWithPropertiesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RedisLinkedServerWithPropertiesList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The response of list Redis operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RedisListResult {
    #[doc = "List of Redis cache instances."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RedisResource>,
    #[doc = "Link for next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RedisListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RedisListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response to put/get patch schedules for Redis cache."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisPatchSchedule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "List of patch schedules for a Redis cache."]
    pub properties: ScheduleEntries,
    #[doc = "The geo-location where the resource lives"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl RedisPatchSchedule {
    pub fn new(properties: ScheduleEntries) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            location: None,
        }
    }
}
#[doc = "The response of list patch schedules Redis operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RedisPatchScheduleListResult {
    #[doc = "Results of the list patch schedules operation."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RedisPatchSchedule>,
    #[doc = "Link for next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for RedisPatchScheduleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl RedisPatchScheduleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the redis cache."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisProperties {
    #[serde(flatten)]
    pub redis_create_properties: RedisCreateProperties,
    #[doc = "Redis instance provisioning status."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<redis_properties::ProvisioningState>,
    #[doc = "Redis host name."]
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[doc = "Redis non-SSL port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[doc = "Redis SSL port."]
    #[serde(rename = "sslPort", default, skip_serializing_if = "Option::is_none")]
    pub ssl_port: Option<i32>,
    #[doc = "Redis cache access keys."]
    #[serde(rename = "accessKeys", default, skip_serializing_if = "Option::is_none")]
    pub access_keys: Option<RedisAccessKeys>,
    #[doc = "List of the linked servers associated with the cache"]
    #[serde(rename = "linkedServers", default, skip_serializing_if = "Vec::is_empty")]
    pub linked_servers: Vec<RedisLinkedServer>,
    #[doc = "List of the Redis instances associated with the cache"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instances: Vec<RedisInstanceDetails>,
    #[doc = "List of private endpoint connection associated with the specified redis cache"]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
}
impl RedisProperties {
    pub fn new(redis_create_properties: RedisCreateProperties) -> Self {
        Self {
            redis_create_properties,
            provisioning_state: None,
            host_name: None,
            port: None,
            ssl_port: None,
            access_keys: None,
            linked_servers: Vec::new(),
            instances: Vec::new(),
            private_endpoint_connections: Vec::new(),
        }
    }
}
pub mod redis_properties {
    use super::*;
    #[doc = "Redis instance provisioning status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ProvisioningState")]
    pub enum ProvisioningState {
        Creating,
        Deleting,
        Disabled,
        Failed,
        Linking,
        Provisioning,
        RecoveringScaleFailure,
        Scaling,
        Succeeded,
        Unlinking,
        Unprovisioning,
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
                Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Creating"),
                Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Deleting"),
                Self::Disabled => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Disabled"),
                Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Failed"),
                Self::Linking => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Linking"),
                Self::Provisioning => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Provisioning"),
                Self::RecoveringScaleFailure => serializer.serialize_unit_variant("ProvisioningState", 6u32, "RecoveringScaleFailure"),
                Self::Scaling => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Scaling"),
                Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Succeeded"),
                Self::Unlinking => serializer.serialize_unit_variant("ProvisioningState", 9u32, "Unlinking"),
                Self::Unprovisioning => serializer.serialize_unit_variant("ProvisioningState", 10u32, "Unprovisioning"),
                Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 11u32, "Updating"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies which Redis node(s) to reboot."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RedisRebootParameters {
    #[doc = "Which Redis node(s) to reboot. Depending on this value data loss is possible."]
    #[serde(rename = "rebootType", default, skip_serializing_if = "Option::is_none")]
    pub reboot_type: Option<redis_reboot_parameters::RebootType>,
    #[doc = "If clustering is enabled, the ID of the shard to be rebooted."]
    #[serde(rename = "shardId", default, skip_serializing_if = "Option::is_none")]
    pub shard_id: Option<i32>,
    #[doc = "A list of redis instances to reboot, specified by per-instance SSL ports or non-SSL ports."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<i64>,
}
impl RedisRebootParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod redis_reboot_parameters {
    use super::*;
    #[doc = "Which Redis node(s) to reboot. Depending on this value data loss is possible."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "RebootType")]
    pub enum RebootType {
        PrimaryNode,
        SecondaryNode,
        AllNodes,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for RebootType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for RebootType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for RebootType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::PrimaryNode => serializer.serialize_unit_variant("RebootType", 0u32, "PrimaryNode"),
                Self::SecondaryNode => serializer.serialize_unit_variant("RebootType", 1u32, "SecondaryNode"),
                Self::AllNodes => serializer.serialize_unit_variant("RebootType", 2u32, "AllNodes"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Specifies which Redis access keys to reset."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisRegenerateKeyParameters {
    #[doc = "The Redis access key to regenerate."]
    #[serde(rename = "keyType")]
    pub key_type: redis_regenerate_key_parameters::KeyType,
}
impl RedisRegenerateKeyParameters {
    pub fn new(key_type: redis_regenerate_key_parameters::KeyType) -> Self {
        Self { key_type }
    }
}
pub mod redis_regenerate_key_parameters {
    use super::*;
    #[doc = "The Redis access key to regenerate."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyType {
        Primary,
        Secondary,
    }
}
#[doc = "A single Redis item in List or Get Operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of the redis cache."]
    pub properties: RedisProperties,
    #[doc = "A list of availability zones denoting where the resource needs to come from."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl RedisResource {
    pub fn new(tracked_resource: TrackedResource, properties: RedisProperties) -> Self {
        Self {
            tracked_resource,
            properties,
            zones: Vec::new(),
            identity: None,
        }
    }
}
#[doc = "Parameters supplied to the Update Redis operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RedisUpdateParameters {
    #[doc = "Patchable properties of the redis cache."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RedisUpdateProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl RedisUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Patchable properties of the redis cache."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RedisUpdateProperties {
    #[serde(flatten)]
    pub redis_common_properties: RedisCommonProperties,
    #[doc = "SKU parameters supplied to the create Redis operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl RedisUpdateProperties {
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
#[doc = "List of patch schedules for a Redis cache."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleEntries {
    #[doc = "List of patch schedules for a Redis cache."]
    #[serde(rename = "scheduleEntries")]
    pub schedule_entries: Vec<ScheduleEntry>,
}
impl ScheduleEntries {
    pub fn new(schedule_entries: Vec<ScheduleEntry>) -> Self {
        Self { schedule_entries }
    }
}
#[doc = "Patch schedule entry for a Premium Redis Cache."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleEntry {
    #[doc = "Day of the week when a cache can be patched."]
    #[serde(rename = "dayOfWeek")]
    pub day_of_week: schedule_entry::DayOfWeek,
    #[doc = "Start hour after which cache patching can start."]
    #[serde(rename = "startHourUtc")]
    pub start_hour_utc: i32,
    #[doc = "ISO8601 timespan specifying how much time cache patching can take. "]
    #[serde(rename = "maintenanceWindow", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<String>,
}
impl ScheduleEntry {
    pub fn new(day_of_week: schedule_entry::DayOfWeek, start_hour_utc: i32) -> Self {
        Self {
            day_of_week,
            start_hour_utc,
            maintenance_window: None,
        }
    }
}
pub mod schedule_entry {
    use super::*;
    #[doc = "Day of the week when a cache can be patched."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DayOfWeek {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
        Everyday,
        Weekend,
    }
}
#[doc = "SKU parameters supplied to the create Redis operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The type of Redis cache to deploy. Valid values: (Basic, Standard, Premium)"]
    pub name: sku::Name,
    #[doc = "The SKU family to use. Valid values: (C, P). (C = Basic/Standard, P = Premium)."]
    pub family: sku::Family,
    #[doc = "The size of the Redis cache to deploy. Valid values: for C (Basic/Standard) family (0, 1, 2, 3, 4, 5, 6), for P (Premium) family (1, 2, 3, 4)."]
    pub capacity: i32,
}
impl Sku {
    pub fn new(name: sku::Name, family: sku::Family, capacity: i32) -> Self {
        Self { name, family, capacity }
    }
}
pub mod sku {
    use super::*;
    #[doc = "The type of Redis cache to deploy. Valid values: (Basic, Standard, Premium)"]
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
    #[doc = "The SKU family to use. Valid values: (C, P). (C = Basic/Standard, P = Premium)."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Family")]
    pub enum Family {
        C,
        P,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Family {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Family {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Family {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::C => serializer.serialize_unit_variant("Family", 0u32, "C"),
                Self::P => serializer.serialize_unit_variant("Family", 1u32, "P"),
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
#[doc = "Properties of upgrade notification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpgradeNotification {
    #[doc = "Name of upgrade notification."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Timestamp when upgrade notification occurred."]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "Details about this upgrade notification"]
    #[serde(rename = "upsellNotification", default, skip_serializing_if = "Option::is_none")]
    pub upsell_notification: Option<serde_json::Value>,
}
impl UpgradeNotification {
    pub fn new() -> Self {
        Self::default()
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
