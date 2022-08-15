#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Backup request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupRequest {
    #[doc = "Azure File Share."]
    #[serde(rename = "azureFileShare", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_share: Option<String>,
}
impl BackupRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for a check name availability request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityParameters {
    #[doc = "The name to check for availability"]
    pub name: String,
    #[doc = "The resource type. Must be set to Microsoft.StorageSync/storageSyncServices"]
    #[serde(rename = "type")]
    pub type_: check_name_availability_parameters::Type,
}
impl CheckNameAvailabilityParameters {
    pub fn new(name: String, type_: check_name_availability_parameters::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod check_name_availability_parameters {
    use super::*;
    #[doc = "The resource type. Must be set to Microsoft.StorageSync/storageSyncServices"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.StorageSync/storageSyncServices")]
        MicrosoftStorageSyncStorageSyncServices,
    }
}
#[doc = "The CheckNameAvailability operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[doc = "Gets a boolean value that indicates whether the name is available for you to use. If true, the name is available. If false, the name has already been taken or invalid and cannot be used."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "Gets the reason that a Storage Sync Service name could not be used. The Reason element is only returned if NameAvailable is false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_result::Reason>,
    #[doc = "Gets an error message explaining the Reason value in more detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_result {
    use super::*;
    #[doc = "Gets the reason that a Storage Sync Service name could not be used. The Reason element is only returned if NameAvailable is false."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Invalid,
        AlreadyExists,
    }
}
#[doc = "Cloud Endpoint object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudEndpoint {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "CloudEndpoint Properties object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CloudEndpointProperties>,
}
impl CloudEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Array of CloudEndpoint"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudEndpointArray {
    #[doc = "Collection of CloudEndpoint."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CloudEndpoint>,
}
impl azure_core::Continuable for CloudEndpointArray {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl CloudEndpointArray {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters used when creating a cloud endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudEndpointCreateParameters {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "CloudEndpoint Properties object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CloudEndpointCreateParametersProperties>,
}
impl CloudEndpointCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "CloudEndpoint Properties object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudEndpointCreateParametersProperties {
    #[doc = "Storage Account Resource Id"]
    #[serde(rename = "storageAccountResourceId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_resource_id: Option<String>,
    #[doc = "Azure file share name"]
    #[serde(rename = "azureFileShareName", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_share_name: Option<String>,
    #[doc = "Storage Account Tenant Id"]
    #[serde(rename = "storageAccountTenantId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_tenant_id: Option<String>,
    #[doc = "Friendly Name"]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
impl CloudEndpointCreateParametersProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "CloudEndpoint Properties object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudEndpointProperties {
    #[doc = "Storage Account Resource Id"]
    #[serde(rename = "storageAccountResourceId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_resource_id: Option<String>,
    #[doc = "Azure file share name"]
    #[serde(rename = "azureFileShareName", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_share_name: Option<String>,
    #[doc = "Storage Account Tenant Id"]
    #[serde(rename = "storageAccountTenantId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_tenant_id: Option<String>,
    #[doc = "Partnership Id"]
    #[serde(rename = "partnershipId", default, skip_serializing_if = "Option::is_none")]
    pub partnership_id: Option<String>,
    #[doc = "Friendly Name"]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Backup Enabled"]
    #[serde(rename = "backupEnabled", default, skip_serializing_if = "Option::is_none")]
    pub backup_enabled: Option<String>,
    #[doc = "CloudEndpoint Provisioning State"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "CloudEndpoint lastWorkflowId"]
    #[serde(rename = "lastWorkflowId", default, skip_serializing_if = "Option::is_none")]
    pub last_workflow_id: Option<String>,
    #[doc = "Resource Last Operation Name"]
    #[serde(rename = "lastOperationName", default, skip_serializing_if = "Option::is_none")]
    pub last_operation_name: Option<String>,
}
impl CloudEndpointProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Server endpoint cloud tiering status object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudTieringCachePerformance {
    #[doc = "Last updated timestamp"]
    #[serde(rename = "lastUpdatedTimestamp", with = "azure_core::date::rfc3339::option")]
    pub last_updated_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Count of bytes that were served from the local server"]
    #[serde(rename = "cacheHitBytes", default, skip_serializing_if = "Option::is_none")]
    pub cache_hit_bytes: Option<i64>,
    #[doc = "Count of bytes that were served from the cloud"]
    #[serde(rename = "cacheMissBytes", default, skip_serializing_if = "Option::is_none")]
    pub cache_miss_bytes: Option<i64>,
    #[doc = "Percentage of total bytes (hit + miss) that were served from the local server"]
    #[serde(rename = "cacheHitBytesPercent", default, skip_serializing_if = "Option::is_none")]
    pub cache_hit_bytes_percent: Option<i32>,
}
impl CloudTieringCachePerformance {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Status of the date policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudTieringDatePolicyStatus {
    #[doc = "Last updated timestamp"]
    #[serde(rename = "lastUpdatedTimestamp", with = "azure_core::date::rfc3339::option")]
    pub last_updated_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Most recent access time of tiered files"]
    #[serde(rename = "tieredFilesMostRecentAccessTimestamp", with = "azure_core::date::rfc3339::option")]
    pub tiered_files_most_recent_access_timestamp: Option<time::OffsetDateTime>,
}
impl CloudTieringDatePolicyStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Server endpoint cloud tiering status object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudTieringFilesNotTiering {
    #[doc = "Last updated timestamp"]
    #[serde(rename = "lastUpdatedTimestamp", with = "azure_core::date::rfc3339::option")]
    pub last_updated_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Last cloud tiering result (HResult)"]
    #[serde(rename = "totalFileCount", default, skip_serializing_if = "Option::is_none")]
    pub total_file_count: Option<i64>,
    #[doc = "Array of tiering errors"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<FilesNotTieringError>,
}
impl CloudTieringFilesNotTiering {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Server endpoint cloud tiering status object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudTieringSpaceSavings {
    #[doc = "Last updated timestamp"]
    #[serde(rename = "lastUpdatedTimestamp", with = "azure_core::date::rfc3339::option")]
    pub last_updated_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Volume size"]
    #[serde(rename = "volumeSizeBytes", default, skip_serializing_if = "Option::is_none")]
    pub volume_size_bytes: Option<i64>,
    #[doc = "Total size of content in the azure file share"]
    #[serde(rename = "totalSizeCloudBytes", default, skip_serializing_if = "Option::is_none")]
    pub total_size_cloud_bytes: Option<i64>,
    #[doc = "Cached content size on the server"]
    #[serde(rename = "cachedSizeBytes", default, skip_serializing_if = "Option::is_none")]
    pub cached_size_bytes: Option<i64>,
    #[doc = "Percentage of cached size over total size"]
    #[serde(rename = "spaceSavingsPercent", default, skip_serializing_if = "Option::is_none")]
    pub space_savings_percent: Option<i32>,
    #[doc = "Count of bytes saved on the server"]
    #[serde(rename = "spaceSavingsBytes", default, skip_serializing_if = "Option::is_none")]
    pub space_savings_bytes: Option<i64>,
}
impl CloudTieringSpaceSavings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Status of the volume free space policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudTieringVolumeFreeSpacePolicyStatus {
    #[doc = "Last updated timestamp"]
    #[serde(rename = "lastUpdatedTimestamp", with = "azure_core::date::rfc3339::option")]
    pub last_updated_timestamp: Option<time::OffsetDateTime>,
    #[doc = "In the case where multiple server endpoints are present in a volume, an effective free space policy is applied."]
    #[serde(rename = "effectiveVolumeFreeSpacePolicy", default, skip_serializing_if = "Option::is_none")]
    pub effective_volume_free_space_policy: Option<i32>,
    #[doc = "Current volume free space percentage."]
    #[serde(rename = "currentVolumeFreeSpacePercent", default, skip_serializing_if = "Option::is_none")]
    pub current_volume_free_space_percent: Option<i32>,
}
impl CloudTieringVolumeFreeSpacePolicyStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of the Feature Status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FeatureStatus {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}
#[doc = "Files not tiering error object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct FilesNotTieringError {
    #[doc = "Error code (HResult)"]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[doc = "Count of files with this error"]
    #[serde(rename = "fileCount", default, skip_serializing_if = "Option::is_none")]
    pub file_count: Option<i64>,
}
impl FilesNotTieringError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of the Operation Direction"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OperationDirection {
    #[serde(rename = "do")]
    Do,
    #[serde(rename = "undo")]
    Undo,
    #[serde(rename = "cancel")]
    Cancel,
}
#[doc = "The operation supported by storage sync."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplayInfo {
    #[doc = "The description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The action that users can perform, based on their permission level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Service provider: Microsoft StorageSync."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Resource on which the operation is performed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
impl OperationDisplayInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation Display Resource object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplayResource {
    #[doc = "Operation Display Resource Provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Operation Display Resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Operation Display Resource Operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Operation Display Resource Description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplayResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The operation supported by storage sync."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationEntity {
    #[doc = "Operation name: {provider}/{resource}/{operation}."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The operation supported by storage sync."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayInfo>,
    #[doc = "The origin."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl OperationEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of storage sync operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationEntityListResult {
    #[doc = "The link used to get the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "The list of operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationEntity>,
}
impl azure_core::Continuable for OperationEntityListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl OperationEntityListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation status object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatus {
    #[doc = "Operation Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Operation status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Start time of the operation"]
    #[serde(rename = "startTime", with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<time::OffsetDateTime>,
    #[doc = "End time of the operation"]
    #[serde(rename = "endTime", with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<time::OffsetDateTime>,
    #[doc = "Error type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<StorageSyncApiError>,
}
impl OperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type PhysicalPath = String;
#[doc = "Post Backup Response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PostBackupResponse {
    #[doc = "Post Backup Response Properties object."]
    #[serde(rename = "backupMetadata", default, skip_serializing_if = "Option::is_none")]
    pub backup_metadata: Option<PostBackupResponseProperties>,
}
impl PostBackupResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Post Backup Response Properties object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PostBackupResponseProperties {
    #[doc = "cloud endpoint Name."]
    #[serde(rename = "cloudEndpointName", default, skip_serializing_if = "Option::is_none")]
    pub cloud_endpoint_name: Option<String>,
}
impl PostBackupResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Post Restore Request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PostRestoreRequest {
    #[doc = "Post Restore partition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    #[doc = "Post Restore replica group."]
    #[serde(rename = "replicaGroup", default, skip_serializing_if = "Option::is_none")]
    pub replica_group: Option<String>,
    #[doc = "Post Restore request id."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "Post Restore Azure file share uri."]
    #[serde(rename = "azureFileShareUri", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_share_uri: Option<String>,
    #[doc = "Post Restore Azure status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Post Restore Azure source azure file share uri."]
    #[serde(rename = "sourceAzureFileShareUri", default, skip_serializing_if = "Option::is_none")]
    pub source_azure_file_share_uri: Option<String>,
    #[doc = "Post Restore Azure failed file list."]
    #[serde(rename = "failedFileList", default, skip_serializing_if = "Option::is_none")]
    pub failed_file_list: Option<String>,
    #[doc = "Post Restore restore file spec array."]
    #[serde(rename = "restoreFileSpec", default, skip_serializing_if = "Vec::is_empty")]
    pub restore_file_spec: Vec<RestoreFileSpec>,
}
impl PostRestoreRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Pre Restore request object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PreRestoreRequest {
    #[doc = "Pre Restore partition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    #[doc = "Pre Restore replica group."]
    #[serde(rename = "replicaGroup", default, skip_serializing_if = "Option::is_none")]
    pub replica_group: Option<String>,
    #[doc = "Pre Restore request id."]
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[doc = "Pre Restore Azure file share uri."]
    #[serde(rename = "azureFileShareUri", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_share_uri: Option<String>,
    #[doc = "Pre Restore Azure status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Pre Restore Azure source azure file share uri."]
    #[serde(rename = "sourceAzureFileShareUri", default, skip_serializing_if = "Option::is_none")]
    pub source_azure_file_share_uri: Option<String>,
    #[doc = "Pre Restore backup metadata property bag."]
    #[serde(rename = "backupMetadataPropertyBag", default, skip_serializing_if = "Option::is_none")]
    pub backup_metadata_property_bag: Option<String>,
    #[doc = "Pre Restore restore file spec array."]
    #[serde(rename = "restoreFileSpec", default, skip_serializing_if = "Vec::is_empty")]
    pub restore_file_spec: Vec<RestoreFileSpec>,
    #[doc = "Pre Restore pause wait for sync drain time period in seconds."]
    #[serde(
        rename = "pauseWaitForSyncDrainTimePeriodInSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pause_wait_for_sync_drain_time_period_in_seconds: Option<i64>,
}
impl PreRestoreRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of the ProgressType"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProgressType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "initialize")]
    Initialize,
    #[serde(rename = "download")]
    Download,
    #[serde(rename = "upload")]
    Upload,
    #[serde(rename = "recall")]
    Recall,
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
#[doc = "The parameters used when calling recall action on server endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecallActionParameters {
    #[doc = "Pattern of the files."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[doc = "Recall path."]
    #[serde(rename = "recallPath", default, skip_serializing_if = "Option::is_none")]
    pub recall_path: Option<String>,
}
impl RecallActionParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Registered Server resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegisteredServer {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "RegisteredServer Properties object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegisteredServerProperties>,
}
impl RegisteredServer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Array of RegisteredServer"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegisteredServerArray {
    #[doc = "Collection of Registered Server."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RegisteredServer>,
}
impl azure_core::Continuable for RegisteredServerArray {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl RegisteredServerArray {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters used when creating a registered server."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegisteredServerCreateParameters {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegisteredServerCreateParametersProperties>,
}
impl RegisteredServerCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegisteredServerCreateParametersProperties {
    #[doc = "Registered Server Certificate"]
    #[serde(rename = "serverCertificate", default, skip_serializing_if = "Option::is_none")]
    pub server_certificate: Option<String>,
    #[doc = "Registered Server Agent Version"]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "Registered Server OS Version"]
    #[serde(rename = "serverOSVersion", default, skip_serializing_if = "Option::is_none")]
    pub server_os_version: Option<String>,
    #[doc = "Registered Server last heart beat"]
    #[serde(rename = "lastHeartBeat", default, skip_serializing_if = "Option::is_none")]
    pub last_heart_beat: Option<String>,
    #[doc = "Registered Server serverRole"]
    #[serde(rename = "serverRole", default, skip_serializing_if = "Option::is_none")]
    pub server_role: Option<String>,
    #[doc = "Registered Server clusterId"]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc = "Registered Server clusterName"]
    #[serde(rename = "clusterName", default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[doc = "Registered Server serverId"]
    #[serde(rename = "serverId", default, skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[doc = "Friendly Name"]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
impl RegisteredServerCreateParametersProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "RegisteredServer Properties object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegisteredServerProperties {
    #[doc = "Registered Server Certificate"]
    #[serde(rename = "serverCertificate", default, skip_serializing_if = "Option::is_none")]
    pub server_certificate: Option<String>,
    #[doc = "Registered Server Agent Version"]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "Registered Server OS Version"]
    #[serde(rename = "serverOSVersion", default, skip_serializing_if = "Option::is_none")]
    pub server_os_version: Option<String>,
    #[doc = "Registered Server Management Error Code"]
    #[serde(rename = "serverManagementErrorCode", default, skip_serializing_if = "Option::is_none")]
    pub server_management_error_code: Option<i64>,
    #[doc = "Registered Server last heart beat"]
    #[serde(rename = "lastHeartBeat", default, skip_serializing_if = "Option::is_none")]
    pub last_heart_beat: Option<String>,
    #[doc = "Registered Server Provisioning State"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "Registered Server serverRole"]
    #[serde(rename = "serverRole", default, skip_serializing_if = "Option::is_none")]
    pub server_role: Option<String>,
    #[doc = "Registered Server clusterId"]
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[doc = "Registered Server clusterName"]
    #[serde(rename = "clusterName", default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[doc = "Registered Server serverId"]
    #[serde(rename = "serverId", default, skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[doc = "Registered Server storageSyncServiceUid"]
    #[serde(rename = "storageSyncServiceUid", default, skip_serializing_if = "Option::is_none")]
    pub storage_sync_service_uid: Option<String>,
    #[doc = "Registered Server lastWorkflowId"]
    #[serde(rename = "lastWorkflowId", default, skip_serializing_if = "Option::is_none")]
    pub last_workflow_id: Option<String>,
    #[doc = "Resource Last Operation Name"]
    #[serde(rename = "lastOperationName", default, skip_serializing_if = "Option::is_none")]
    pub last_operation_name: Option<String>,
    #[doc = "Resource discoveryEndpointUri"]
    #[serde(rename = "discoveryEndpointUri", default, skip_serializing_if = "Option::is_none")]
    pub discovery_endpoint_uri: Option<String>,
    #[doc = "Resource Location"]
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[doc = "Service Location"]
    #[serde(rename = "serviceLocation", default, skip_serializing_if = "Option::is_none")]
    pub service_location: Option<String>,
    #[doc = "Friendly Name"]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Management Endpoint Uri"]
    #[serde(rename = "managementEndpointUri", default, skip_serializing_if = "Option::is_none")]
    pub management_endpoint_uri: Option<String>,
    #[doc = "Monitoring Configuration"]
    #[serde(rename = "monitoringConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration: Option<String>,
}
impl RegisteredServerProperties {
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
pub type ResourceId = String;
#[doc = "Resource Move Info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourcesMoveInfo {
    #[doc = "Target resource group."]
    #[serde(rename = "targetResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_group: Option<String>,
    #[doc = "Collection of Resources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<ResourceId>,
}
impl ResourcesMoveInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Restore file spec."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestoreFileSpec {
    #[doc = "Restore file spec path"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Restore file spec isdir"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub isdir: Option<bool>,
}
impl RestoreFileSpec {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Server Endpoint object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpoint {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "ServerEndpoint Properties object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerEndpointProperties>,
}
impl ServerEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Array of ServerEndpoint"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointArray {
    #[doc = "Collection of ServerEndpoint."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerEndpoint>,
}
impl azure_core::Continuable for ServerEndpointArray {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ServerEndpointArray {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of the cloud tiering health state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerEndpointCloudTieringHealthState {
    Healthy,
    Error,
}
#[doc = "Server endpoint cloud tiering status object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointCloudTieringStatus {
    #[doc = "Last updated timestamp"]
    #[serde(rename = "lastUpdatedTimestamp", with = "azure_core::date::rfc3339::option")]
    pub last_updated_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Type of the cloud tiering health state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<ServerEndpointCloudTieringHealthState>,
    #[doc = "The last updated timestamp of health state"]
    #[serde(rename = "healthLastUpdatedTimestamp", with = "azure_core::date::rfc3339::option")]
    pub health_last_updated_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Last cloud tiering result (HResult)"]
    #[serde(rename = "lastCloudTieringResult", default, skip_serializing_if = "Option::is_none")]
    pub last_cloud_tiering_result: Option<i32>,
    #[doc = "Last cloud tiering success timestamp"]
    #[serde(rename = "lastSuccessTimestamp", with = "azure_core::date::rfc3339::option")]
    pub last_success_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Server endpoint cloud tiering status object."]
    #[serde(rename = "spaceSavings", default, skip_serializing_if = "Option::is_none")]
    pub space_savings: Option<CloudTieringSpaceSavings>,
    #[doc = "Server endpoint cloud tiering status object."]
    #[serde(rename = "cachePerformance", default, skip_serializing_if = "Option::is_none")]
    pub cache_performance: Option<CloudTieringCachePerformance>,
    #[doc = "Server endpoint cloud tiering status object."]
    #[serde(rename = "filesNotTiering", default, skip_serializing_if = "Option::is_none")]
    pub files_not_tiering: Option<CloudTieringFilesNotTiering>,
    #[doc = "Status of the volume free space policy"]
    #[serde(rename = "volumeFreeSpacePolicyStatus", default, skip_serializing_if = "Option::is_none")]
    pub volume_free_space_policy_status: Option<CloudTieringVolumeFreeSpacePolicyStatus>,
    #[doc = "Status of the date policy"]
    #[serde(rename = "datePolicyStatus", default, skip_serializing_if = "Option::is_none")]
    pub date_policy_status: Option<CloudTieringDatePolicyStatus>,
}
impl ServerEndpointCloudTieringStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters used when creating a server endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointCreateParameters {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "ServerEndpoint Properties object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerEndpointCreateParametersProperties>,
}
impl ServerEndpointCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ServerEndpoint Properties object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointCreateParametersProperties {
    #[doc = "Server folder used for data synchronization"]
    #[serde(rename = "serverLocalPath", default, skip_serializing_if = "Option::is_none")]
    pub server_local_path: Option<PhysicalPath>,
    #[doc = "Type of the Feature Status"]
    #[serde(rename = "cloudTiering", default, skip_serializing_if = "Option::is_none")]
    pub cloud_tiering: Option<FeatureStatus>,
    #[doc = "Level of free space to be maintained by Cloud Tiering if it is enabled."]
    #[serde(rename = "volumeFreeSpacePercent", default, skip_serializing_if = "Option::is_none")]
    pub volume_free_space_percent: Option<i64>,
    #[doc = "Tier files older than days."]
    #[serde(rename = "tierFilesOlderThanDays", default, skip_serializing_if = "Option::is_none")]
    pub tier_files_older_than_days: Option<i64>,
    #[doc = "Friendly Name"]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Arm resource identifier."]
    #[serde(rename = "serverResourceId", default, skip_serializing_if = "Option::is_none")]
    pub server_resource_id: Option<ResourceId>,
    #[doc = "Type of the Feature Status"]
    #[serde(rename = "offlineDataTransfer", default, skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer: Option<FeatureStatus>,
    #[doc = "Offline data transfer share name"]
    #[serde(rename = "offlineDataTransferShareName", default, skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer_share_name: Option<String>,
}
impl ServerEndpointCreateParametersProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Files not syncing error object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointFilesNotSyncingError {
    #[doc = "Error code (HResult)"]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[doc = "Count of persistent files not syncing with the specified error code"]
    #[serde(rename = "persistentCount", default, skip_serializing_if = "Option::is_none")]
    pub persistent_count: Option<i64>,
    #[doc = "Count of transient files not syncing with the specified error code"]
    #[serde(rename = "transientCount", default, skip_serializing_if = "Option::is_none")]
    pub transient_count: Option<i64>,
}
impl ServerEndpointFilesNotSyncingError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of the Health state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerEndpointOfflineDataTransferState {
    InProgress,
    Stopping,
    NotRunning,
    Complete,
}
#[doc = "ServerEndpoint Properties object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointProperties {
    #[doc = "Server folder used for data synchronization"]
    #[serde(rename = "serverLocalPath", default, skip_serializing_if = "Option::is_none")]
    pub server_local_path: Option<PhysicalPath>,
    #[doc = "Type of the Feature Status"]
    #[serde(rename = "cloudTiering", default, skip_serializing_if = "Option::is_none")]
    pub cloud_tiering: Option<FeatureStatus>,
    #[doc = "Level of free space to be maintained by Cloud Tiering if it is enabled."]
    #[serde(rename = "volumeFreeSpacePercent", default, skip_serializing_if = "Option::is_none")]
    pub volume_free_space_percent: Option<i64>,
    #[doc = "Tier files older than days."]
    #[serde(rename = "tierFilesOlderThanDays", default, skip_serializing_if = "Option::is_none")]
    pub tier_files_older_than_days: Option<i64>,
    #[doc = "Friendly Name"]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Arm resource identifier."]
    #[serde(rename = "serverResourceId", default, skip_serializing_if = "Option::is_none")]
    pub server_resource_id: Option<ResourceId>,
    #[doc = "ServerEndpoint Provisioning State"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "ServerEndpoint lastWorkflowId"]
    #[serde(rename = "lastWorkflowId", default, skip_serializing_if = "Option::is_none")]
    pub last_workflow_id: Option<String>,
    #[doc = "Resource Last Operation Name"]
    #[serde(rename = "lastOperationName", default, skip_serializing_if = "Option::is_none")]
    pub last_operation_name: Option<String>,
    #[doc = "Server Endpoint sync status"]
    #[serde(rename = "syncStatus", default, skip_serializing_if = "Option::is_none")]
    pub sync_status: Option<ServerEndpointSyncStatus>,
    #[doc = "Type of the Feature Status"]
    #[serde(rename = "offlineDataTransfer", default, skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer: Option<FeatureStatus>,
    #[doc = "Offline data transfer storage account resource ID"]
    #[serde(
        rename = "offlineDataTransferStorageAccountResourceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub offline_data_transfer_storage_account_resource_id: Option<String>,
    #[doc = "Offline data transfer storage account tenant ID"]
    #[serde(
        rename = "offlineDataTransferStorageAccountTenantId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub offline_data_transfer_storage_account_tenant_id: Option<String>,
    #[doc = "Offline data transfer share name"]
    #[serde(rename = "offlineDataTransferShareName", default, skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer_share_name: Option<String>,
    #[doc = "Server endpoint cloud tiering status object."]
    #[serde(rename = "cloudTieringStatus", default, skip_serializing_if = "Option::is_none")]
    pub cloud_tiering_status: Option<ServerEndpointCloudTieringStatus>,
    #[doc = "Server endpoint recall status object."]
    #[serde(rename = "recallStatus", default, skip_serializing_if = "Option::is_none")]
    pub recall_status: Option<ServerEndpointRecallStatus>,
}
impl ServerEndpointProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Server endpoint recall error object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointRecallError {
    #[doc = "Error code (HResult)"]
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[doc = "Count of occurences of the error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
impl ServerEndpointRecallError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Server endpoint recall status object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointRecallStatus {
    #[doc = "Last updated timestamp"]
    #[serde(rename = "lastUpdatedTimestamp", with = "azure_core::date::rfc3339::option")]
    pub last_updated_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Total count of recall errors."]
    #[serde(rename = "totalRecallErrorsCount", default, skip_serializing_if = "Option::is_none")]
    pub total_recall_errors_count: Option<i64>,
    #[doc = "Array of recall errors"]
    #[serde(rename = "recallErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub recall_errors: Vec<ServerEndpointRecallError>,
}
impl ServerEndpointRecallStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of the sync activity state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerEndpointSyncActivityState {
    Upload,
    Download,
    UploadAndDownload,
}
#[doc = "Type of the sync health state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerEndpointSyncHealthState {
    Healthy,
    Error,
    SyncBlockedForRestore,
    SyncBlockedForChangeDetectionPostRestore,
    NoActivity,
}
#[doc = "Server Endpoint sync status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointSyncStatus {
    #[doc = "Type of the sync health state"]
    #[serde(rename = "downloadHealth", default, skip_serializing_if = "Option::is_none")]
    pub download_health: Option<ServerEndpointSyncHealthState>,
    #[doc = "Type of the sync health state"]
    #[serde(rename = "uploadHealth", default, skip_serializing_if = "Option::is_none")]
    pub upload_health: Option<ServerEndpointSyncHealthState>,
    #[doc = "Type of the sync health state"]
    #[serde(rename = "combinedHealth", default, skip_serializing_if = "Option::is_none")]
    pub combined_health: Option<ServerEndpointSyncHealthState>,
    #[doc = "Type of the sync activity state"]
    #[serde(rename = "syncActivity", default, skip_serializing_if = "Option::is_none")]
    pub sync_activity: Option<ServerEndpointSyncActivityState>,
    #[doc = "Total count of persistent files not syncing (combined upload + download)."]
    #[serde(rename = "totalPersistentFilesNotSyncingCount", default, skip_serializing_if = "Option::is_none")]
    pub total_persistent_files_not_syncing_count: Option<i64>,
    #[doc = "Last Updated Timestamp"]
    #[serde(rename = "lastUpdatedTimestamp", with = "azure_core::date::rfc3339::option")]
    pub last_updated_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Sync Session status object."]
    #[serde(rename = "uploadStatus", default, skip_serializing_if = "Option::is_none")]
    pub upload_status: Option<SyncSessionStatus>,
    #[doc = "Sync Session status object."]
    #[serde(rename = "downloadStatus", default, skip_serializing_if = "Option::is_none")]
    pub download_status: Option<SyncSessionStatus>,
    #[doc = "Sync Session status object."]
    #[serde(rename = "uploadActivity", default, skip_serializing_if = "Option::is_none")]
    pub upload_activity: Option<SyncActivityStatus>,
    #[doc = "Sync Session status object."]
    #[serde(rename = "downloadActivity", default, skip_serializing_if = "Option::is_none")]
    pub download_activity: Option<SyncActivityStatus>,
    #[doc = "Type of the Health state"]
    #[serde(rename = "offlineDataTransferStatus", default, skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer_status: Option<ServerEndpointOfflineDataTransferState>,
}
impl ServerEndpointSyncStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for updating an Server Endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointUpdateParameters {
    #[doc = "ServerEndpoint Update Properties object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerEndpointUpdateProperties>,
}
impl ServerEndpointUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "ServerEndpoint Update Properties object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointUpdateProperties {
    #[doc = "Type of the Feature Status"]
    #[serde(rename = "cloudTiering", default, skip_serializing_if = "Option::is_none")]
    pub cloud_tiering: Option<FeatureStatus>,
    #[doc = "Level of free space to be maintained by Cloud Tiering if it is enabled."]
    #[serde(rename = "volumeFreeSpacePercent", default, skip_serializing_if = "Option::is_none")]
    pub volume_free_space_percent: Option<i64>,
    #[doc = "Tier files older than days."]
    #[serde(rename = "tierFilesOlderThanDays", default, skip_serializing_if = "Option::is_none")]
    pub tier_files_older_than_days: Option<i64>,
    #[doc = "Type of the Feature Status"]
    #[serde(rename = "offlineDataTransfer", default, skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer: Option<FeatureStatus>,
    #[doc = "Offline data transfer share name"]
    #[serde(rename = "offlineDataTransferShareName", default, skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer_share_name: Option<String>,
}
impl ServerEndpointUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncApiError {
    #[doc = "Error code of the given entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message of the given entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Target of the given error entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Error Details object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<StorageSyncErrorDetails>,
}
impl StorageSyncApiError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncError {
    #[doc = "Error type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<StorageSyncApiError>,
    #[doc = "Error type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<StorageSyncApiError>,
}
impl azure_core::Continuable for StorageSyncError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl StorageSyncError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error Details object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncErrorDetails {
    #[doc = "Error code of the given entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message of the given entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Target of the given entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl StorageSyncErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage Sync Service object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageSyncService {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Storage Sync Service Properties object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageSyncServiceProperties>,
}
impl StorageSyncService {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Array of StorageSyncServices"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncServiceArray {
    #[doc = "Collection of StorageSyncServices."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StorageSyncService>,
}
impl azure_core::Continuable for StorageSyncServiceArray {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl StorageSyncServiceArray {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters used when creating a storage sync service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageSyncServiceCreateParameters {
    #[doc = "Required. Gets or sets the location of the resource. This will be one of the supported and registered Azure Geo Regions (e.g. West US, East US, Southeast Asia, etc.). The geo region of a resource cannot be changed once it is created, but if an identical geo region is specified on update, the request will succeed."]
    pub location: String,
    #[doc = "Gets or sets a list of key value pairs that describe the resource. These tags can be used for viewing and grouping this resource (across resource groups). A maximum of 15 tags can be provided for a resource. Each tag must have a key with a length no greater than 128 characters and a value with a length no greater than 256 characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl StorageSyncServiceCreateParameters {
    pub fn new(location: String) -> Self {
        Self {
            location,
            tags: None,
            properties: None,
        }
    }
}
#[doc = "Storage Sync Service Properties object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncServiceProperties {
    #[doc = "Storage Sync service status."]
    #[serde(rename = "storageSyncServiceStatus", default, skip_serializing_if = "Option::is_none")]
    pub storage_sync_service_status: Option<i64>,
    #[doc = "Storage Sync service Uid"]
    #[serde(rename = "storageSyncServiceUid", default, skip_serializing_if = "Option::is_none")]
    pub storage_sync_service_uid: Option<String>,
}
impl StorageSyncServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Parameters for updating an Storage sync service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncServiceUpdateParameters {
    #[doc = "The user-specified tags associated with the storage sync service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Storage Sync Service Update Properties object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageSyncServiceUpdateProperties>,
}
impl StorageSyncServiceUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage Sync Service Update Properties object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncServiceUpdateProperties {}
impl StorageSyncServiceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subscription State object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionState {
    #[doc = "State of Azure Subscription"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<subscription_state::State>,
    #[doc = "Is Transitioning"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub istransitioning: Option<bool>,
    #[doc = "Subscription State properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionStateProperties>,
}
impl SubscriptionState {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subscription_state {
    use super::*;
    #[doc = "State of Azure Subscription"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "State")]
    pub enum State {
        Registered,
        Unregistered,
        Warned,
        Suspended,
        Deleted,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for State {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for State {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for State {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Registered => serializer.serialize_unit_variant("State", 0u32, "Registered"),
                Self::Unregistered => serializer.serialize_unit_variant("State", 1u32, "Unregistered"),
                Self::Warned => serializer.serialize_unit_variant("State", 2u32, "Warned"),
                Self::Suspended => serializer.serialize_unit_variant("State", 3u32, "Suspended"),
                Self::Deleted => serializer.serialize_unit_variant("State", 4u32, "Deleted"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Subscription State properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionStateProperties {}
impl SubscriptionStateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sync Session status object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncActivityStatus {
    #[doc = "Timestamp when properties were updated"]
    #[serde(with = "azure_core::date::rfc3339::option")]
    pub timestamp: Option<time::OffsetDateTime>,
    #[doc = "Per item error count"]
    #[serde(rename = "perItemErrorCount", default, skip_serializing_if = "Option::is_none")]
    pub per_item_error_count: Option<i64>,
    #[doc = "Applied item count."]
    #[serde(rename = "appliedItemCount", default, skip_serializing_if = "Option::is_none")]
    pub applied_item_count: Option<i64>,
    #[doc = "Total item count (if available)"]
    #[serde(rename = "totalItemCount", default, skip_serializing_if = "Option::is_none")]
    pub total_item_count: Option<i64>,
    #[doc = "Applied bytes"]
    #[serde(rename = "appliedBytes", default, skip_serializing_if = "Option::is_none")]
    pub applied_bytes: Option<i64>,
    #[doc = "Total bytes (if available)"]
    #[serde(rename = "totalBytes", default, skip_serializing_if = "Option::is_none")]
    pub total_bytes: Option<i64>,
}
impl SyncActivityStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sync Group object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "SyncGroup Properties object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SyncGroupProperties>,
}
impl SyncGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Array of SyncGroup"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncGroupArray {
    #[doc = "Collection of SyncGroup."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SyncGroup>,
}
impl azure_core::Continuable for SyncGroupArray {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl SyncGroupArray {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The parameters used when creating a sync group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncGroupCreateParameters {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Sync Group Create Properties object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SyncGroupCreateParametersProperties>,
}
impl SyncGroupCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sync Group Create Properties object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncGroupCreateParametersProperties {}
impl SyncGroupCreateParametersProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SyncGroup Properties object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncGroupProperties {
    #[doc = "Unique Id"]
    #[serde(rename = "uniqueId", default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[doc = "Sync group status"]
    #[serde(rename = "syncGroupStatus", default, skip_serializing_if = "Option::is_none")]
    pub sync_group_status: Option<String>,
}
impl SyncGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Sync Session status object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncSessionStatus {
    #[doc = "Last sync result (HResult)"]
    #[serde(rename = "lastSyncResult", default, skip_serializing_if = "Option::is_none")]
    pub last_sync_result: Option<i32>,
    #[doc = "Last sync timestamp"]
    #[serde(rename = "lastSyncTimestamp", with = "azure_core::date::rfc3339::option")]
    pub last_sync_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Last sync success timestamp"]
    #[serde(rename = "lastSyncSuccessTimestamp", with = "azure_core::date::rfc3339::option")]
    pub last_sync_success_timestamp: Option<time::OffsetDateTime>,
    #[doc = "Last sync per item error count."]
    #[serde(rename = "lastSyncPerItemErrorCount", default, skip_serializing_if = "Option::is_none")]
    pub last_sync_per_item_error_count: Option<i64>,
    #[doc = "Count of persistent files not syncing."]
    #[serde(rename = "persistentFilesNotSyncingCount", default, skip_serializing_if = "Option::is_none")]
    pub persistent_files_not_syncing_count: Option<i64>,
    #[doc = "Count of transient files not syncing."]
    #[serde(rename = "transientFilesNotSyncingCount", default, skip_serializing_if = "Option::is_none")]
    pub transient_files_not_syncing_count: Option<i64>,
    #[doc = "Array of per-item errors coming from the last sync session."]
    #[serde(rename = "filesNotSyncingErrors", default, skip_serializing_if = "Vec::is_empty")]
    pub files_not_syncing_errors: Vec<ServerEndpointFilesNotSyncingError>,
}
impl SyncSessionStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tags object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsObject {}
impl TagsObject {
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
#[doc = "The parameters used when calling trigger change detection action on cloud endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TriggerChangeDetectionParameters {
    #[doc = "Relative path to a directory Azure File share for which change detection is to be performed."]
    #[serde(rename = "directoryPath", default, skip_serializing_if = "Option::is_none")]
    pub directory_path: Option<String>,
    #[doc = "Change Detection Mode. Applies to a directory specified in directoryPath parameter."]
    #[serde(rename = "changeDetectionMode", default, skip_serializing_if = "Option::is_none")]
    pub change_detection_mode: Option<trigger_change_detection_parameters::ChangeDetectionMode>,
    #[doc = "Array of relative paths on the Azure File share to be included in the change detection. Can be files and directories."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<String>,
}
impl TriggerChangeDetectionParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod trigger_change_detection_parameters {
    use super::*;
    #[doc = "Change Detection Mode. Applies to a directory specified in directoryPath parameter."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ChangeDetectionMode")]
    pub enum ChangeDetectionMode {
        Default,
        Recursive,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ChangeDetectionMode {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ChangeDetectionMode {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ChangeDetectionMode {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Default => serializer.serialize_unit_variant("ChangeDetectionMode", 0u32, "Default"),
                Self::Recursive => serializer.serialize_unit_variant("ChangeDetectionMode", 1u32, "Recursive"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Trigger Rollover Request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TriggerRolloverRequest {
    #[doc = "Certificate Data"]
    #[serde(rename = "serverCertificate", default, skip_serializing_if = "Option::is_none")]
    pub server_certificate: Option<String>,
}
impl TriggerRolloverRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workflow resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Workflow {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Workflow Properties object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkflowProperties>,
}
impl Workflow {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Array of Workflow"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowArray {
    #[doc = "Collection of workflow items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workflow>,
}
impl azure_core::Continuable for WorkflowArray {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl WorkflowArray {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workflow Properties object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowProperties {
    #[doc = "last step name"]
    #[serde(rename = "lastStepName", default, skip_serializing_if = "Option::is_none")]
    pub last_step_name: Option<String>,
    #[doc = "Type of the Workflow Status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkflowStatus>,
    #[doc = "Type of the Operation Direction"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<OperationDirection>,
    #[doc = "workflow steps"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steps: Option<String>,
    #[doc = "workflow last operation identifier."]
    #[serde(rename = "lastOperationId", default, skip_serializing_if = "Option::is_none")]
    pub last_operation_id: Option<String>,
}
impl WorkflowProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of the Workflow Status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WorkflowStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "aborted")]
    Aborted,
    #[serde(rename = "failed")]
    Failed,
}
