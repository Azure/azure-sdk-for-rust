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
#[serde(remote = "FeatureStatus")]
pub enum FeatureStatus {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for FeatureStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for FeatureStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for FeatureStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::On => serializer.serialize_unit_variant("FeatureStatus", 0u32, "on"),
            Self::Off => serializer.serialize_unit_variant("FeatureStatus", 1u32, "off"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
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
#[doc = "Type of the Incoming Traffic Policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "IncomingTrafficPolicy")]
pub enum IncomingTrafficPolicy {
    AllowAllTraffic,
    AllowVirtualNetworksOnly,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for IncomingTrafficPolicy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for IncomingTrafficPolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for IncomingTrafficPolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::AllowAllTraffic => serializer.serialize_unit_variant("IncomingTrafficPolicy", 0u32, "AllowAllTraffic"),
            Self::AllowVirtualNetworksOnly => serializer.serialize_unit_variant("IncomingTrafficPolicy", 1u32, "AllowVirtualNetworksOnly"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Policy for how namespace and files are recalled during FastDr"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "InitialDownloadPolicy")]
pub enum InitialDownloadPolicy {
    NamespaceOnly,
    NamespaceThenModifiedFiles,
    AvoidTieredFiles,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for InitialDownloadPolicy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for InitialDownloadPolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for InitialDownloadPolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NamespaceOnly => serializer.serialize_unit_variant("InitialDownloadPolicy", 0u32, "NamespaceOnly"),
            Self::NamespaceThenModifiedFiles => {
                serializer.serialize_unit_variant("InitialDownloadPolicy", 1u32, "NamespaceThenModifiedFiles")
            }
            Self::AvoidTieredFiles => serializer.serialize_unit_variant("InitialDownloadPolicy", 2u32, "AvoidTieredFiles"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Policy for enabling follow-the-sun business models: link local cache to cloud behavior to pre-populate before local access."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LocalCacheMode")]
pub enum LocalCacheMode {
    DownloadNewAndModifiedFiles,
    UpdateLocallyCachedFiles,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LocalCacheMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LocalCacheMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LocalCacheMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::DownloadNewAndModifiedFiles => serializer.serialize_unit_variant("LocalCacheMode", 0u32, "DownloadNewAndModifiedFiles"),
            Self::UpdateLocallyCachedFiles => serializer.serialize_unit_variant("LocalCacheMode", 1u32, "UpdateLocallyCachedFiles"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Operation status object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocationOperationStatus {
    #[doc = "Operation resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
    #[doc = "Percent complete."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<i32>,
}
impl LocationOperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of the Operation Direction"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OperationDirection")]
pub enum OperationDirection {
    #[serde(rename = "do")]
    Do,
    #[serde(rename = "undo")]
    Undo,
    #[serde(rename = "cancel")]
    Cancel,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OperationDirection {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OperationDirection {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OperationDirection {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Do => serializer.serialize_unit_variant("OperationDirection", 0u32, "do"),
            Self::Undo => serializer.serialize_unit_variant("OperationDirection", 1u32, "undo"),
            Self::Cancel => serializer.serialize_unit_variant("OperationDirection", 2u32, "cancel"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
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
    #[doc = "Properties of the operations resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OperationProperties>,
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
#[doc = "Properties of the operations resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationProperties {
    #[doc = "Service specification."]
    #[serde(rename = "serviceSpecification", default, skip_serializing_if = "Option::is_none")]
    pub service_specification: Option<OperationResourceServiceSpecification>,
}
impl OperationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation Display Resource object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResourceMetricSpecification {
    #[doc = "Name of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name for the metric."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Display description for the metric."]
    #[serde(rename = "displayDescription", default, skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[doc = "Unit for the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "Aggregation type for the metric."]
    #[serde(rename = "aggregationType", default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[doc = "Supported aggregation types for the metric."]
    #[serde(rename = "supportedAggregationTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_aggregation_types: Vec<String>,
    #[doc = "Fill gaps in the metric with zero."]
    #[serde(rename = "fillGapWithZero", default, skip_serializing_if = "Option::is_none")]
    pub fill_gap_with_zero: Option<bool>,
    #[doc = "Dimensions for the metric specification."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dimensions: Vec<OperationResourceMetricSpecificationDimension>,
}
impl OperationResourceMetricSpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "OperationResourceMetricSpecificationDimension object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResourceMetricSpecificationDimension {
    #[doc = "Name of the dimension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display name of the dimensions."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Indicates metric should be exported for Shoebox."]
    #[serde(rename = "toBeExportedForShoebox", default, skip_serializing_if = "Option::is_none")]
    pub to_be_exported_for_shoebox: Option<bool>,
}
impl OperationResourceMetricSpecificationDimension {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Service specification."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResourceServiceSpecification {
    #[doc = "List of metric specifications."]
    #[serde(rename = "metricSpecifications", default, skip_serializing_if = "Vec::is_empty")]
    pub metric_specifications: Vec<OperationResourceMetricSpecification>,
}
impl OperationResourceServiceSpecification {
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
#[doc = "Type of the ProgressType"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProgressType")]
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
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProgressType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProgressType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProgressType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ProgressType", 0u32, "none"),
            Self::Initialize => serializer.serialize_unit_variant("ProgressType", 1u32, "initialize"),
            Self::Download => serializer.serialize_unit_variant("ProgressType", 2u32, "download"),
            Self::Upload => serializer.serialize_unit_variant("ProgressType", 3u32, "upload"),
            Self::Recall => serializer.serialize_unit_variant("ProgressType", 4u32, "recall"),
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
#[doc = "Type of the registered server agent version status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RegisteredServerAgentVersionStatus")]
pub enum RegisteredServerAgentVersionStatus {
    Ok,
    NearExpiry,
    Expired,
    Blocked,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RegisteredServerAgentVersionStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RegisteredServerAgentVersionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RegisteredServerAgentVersionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Ok => serializer.serialize_unit_variant("RegisteredServerAgentVersionStatus", 0u32, "Ok"),
            Self::NearExpiry => serializer.serialize_unit_variant("RegisteredServerAgentVersionStatus", 1u32, "NearExpiry"),
            Self::Expired => serializer.serialize_unit_variant("RegisteredServerAgentVersionStatus", 2u32, "Expired"),
            Self::Blocked => serializer.serialize_unit_variant("RegisteredServerAgentVersionStatus", 3u32, "Blocked"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
    #[doc = "RegisteredServer Create Properties object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegisteredServerCreateParametersProperties>,
}
impl RegisteredServerCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "RegisteredServer Create Properties object."]
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
    #[doc = "Type of the registered server agent version status"]
    #[serde(rename = "agentVersionStatus", default, skip_serializing_if = "Option::is_none")]
    pub agent_version_status: Option<RegisteredServerAgentVersionStatus>,
    #[doc = "Registered Server Agent Version Expiration Date"]
    #[serde(rename = "agentVersionExpirationDate", with = "azure_core::date::rfc3339::option")]
    pub agent_version_expiration_date: Option<time::OffsetDateTime>,
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
    #[doc = "Telemetry Endpoint Uri"]
    #[serde(rename = "monitoringEndpointUri", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_endpoint_uri: Option<String>,
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
#[serde(remote = "ServerEndpointCloudTieringHealthState")]
pub enum ServerEndpointCloudTieringHealthState {
    Healthy,
    Error,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServerEndpointCloudTieringHealthState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServerEndpointCloudTieringHealthState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServerEndpointCloudTieringHealthState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Healthy => serializer.serialize_unit_variant("ServerEndpointCloudTieringHealthState", 0u32, "Healthy"),
            Self::Error => serializer.serialize_unit_variant("ServerEndpointCloudTieringHealthState", 1u32, "Error"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
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
    #[doc = "Policy for how namespace and files are recalled during FastDr"]
    #[serde(rename = "initialDownloadPolicy", default, skip_serializing_if = "Option::is_none")]
    pub initial_download_policy: Option<InitialDownloadPolicy>,
    #[doc = "Policy for enabling follow-the-sun business models: link local cache to cloud behavior to pre-populate before local access."]
    #[serde(rename = "localCacheMode", default, skip_serializing_if = "Option::is_none")]
    pub local_cache_mode: Option<LocalCacheMode>,
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
#[serde(remote = "ServerEndpointOfflineDataTransferState")]
pub enum ServerEndpointOfflineDataTransferState {
    InProgress,
    Stopping,
    NotRunning,
    Complete,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServerEndpointOfflineDataTransferState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServerEndpointOfflineDataTransferState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServerEndpointOfflineDataTransferState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::InProgress => serializer.serialize_unit_variant("ServerEndpointOfflineDataTransferState", 0u32, "InProgress"),
            Self::Stopping => serializer.serialize_unit_variant("ServerEndpointOfflineDataTransferState", 1u32, "Stopping"),
            Self::NotRunning => serializer.serialize_unit_variant("ServerEndpointOfflineDataTransferState", 2u32, "NotRunning"),
            Self::Complete => serializer.serialize_unit_variant("ServerEndpointOfflineDataTransferState", 3u32, "Complete"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
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
    #[doc = "Policy for how namespace and files are recalled during FastDr"]
    #[serde(rename = "initialDownloadPolicy", default, skip_serializing_if = "Option::is_none")]
    pub initial_download_policy: Option<InitialDownloadPolicy>,
    #[doc = "Policy for enabling follow-the-sun business models: link local cache to cloud behavior to pre-populate before local access."]
    #[serde(rename = "localCacheMode", default, skip_serializing_if = "Option::is_none")]
    pub local_cache_mode: Option<LocalCacheMode>,
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
#[serde(remote = "ServerEndpointSyncActivityState")]
pub enum ServerEndpointSyncActivityState {
    Upload,
    Download,
    UploadAndDownload,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServerEndpointSyncActivityState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServerEndpointSyncActivityState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServerEndpointSyncActivityState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Upload => serializer.serialize_unit_variant("ServerEndpointSyncActivityState", 0u32, "Upload"),
            Self::Download => serializer.serialize_unit_variant("ServerEndpointSyncActivityState", 1u32, "Download"),
            Self::UploadAndDownload => serializer.serialize_unit_variant("ServerEndpointSyncActivityState", 2u32, "UploadAndDownload"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Sync Session status object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointSyncActivityStatus {
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
    #[doc = "Sync mode for the server endpoint."]
    #[serde(rename = "syncMode", default, skip_serializing_if = "Option::is_none")]
    pub sync_mode: Option<ServerEndpointSyncMode>,
}
impl ServerEndpointSyncActivityStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of the sync health state"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServerEndpointSyncHealthState")]
pub enum ServerEndpointSyncHealthState {
    Healthy,
    Error,
    SyncBlockedForRestore,
    SyncBlockedForChangeDetectionPostRestore,
    NoActivity,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServerEndpointSyncHealthState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServerEndpointSyncHealthState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServerEndpointSyncHealthState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Healthy => serializer.serialize_unit_variant("ServerEndpointSyncHealthState", 0u32, "Healthy"),
            Self::Error => serializer.serialize_unit_variant("ServerEndpointSyncHealthState", 1u32, "Error"),
            Self::SyncBlockedForRestore => {
                serializer.serialize_unit_variant("ServerEndpointSyncHealthState", 2u32, "SyncBlockedForRestore")
            }
            Self::SyncBlockedForChangeDetectionPostRestore => {
                serializer.serialize_unit_variant("ServerEndpointSyncHealthState", 3u32, "SyncBlockedForChangeDetectionPostRestore")
            }
            Self::NoActivity => serializer.serialize_unit_variant("ServerEndpointSyncHealthState", 4u32, "NoActivity"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Sync mode for the server endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ServerEndpointSyncMode")]
pub enum ServerEndpointSyncMode {
    Regular,
    NamespaceDownload,
    InitialUpload,
    SnapshotUpload,
    InitialFullDownload,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ServerEndpointSyncMode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ServerEndpointSyncMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ServerEndpointSyncMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Regular => serializer.serialize_unit_variant("ServerEndpointSyncMode", 0u32, "Regular"),
            Self::NamespaceDownload => serializer.serialize_unit_variant("ServerEndpointSyncMode", 1u32, "NamespaceDownload"),
            Self::InitialUpload => serializer.serialize_unit_variant("ServerEndpointSyncMode", 2u32, "InitialUpload"),
            Self::SnapshotUpload => serializer.serialize_unit_variant("ServerEndpointSyncMode", 3u32, "SnapshotUpload"),
            Self::InitialFullDownload => serializer.serialize_unit_variant("ServerEndpointSyncMode", 4u32, "InitialFullDownload"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Sync Session status object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointSyncSessionStatus {
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
    #[doc = "Sync mode for the server endpoint."]
    #[serde(rename = "lastSyncMode", default, skip_serializing_if = "Option::is_none")]
    pub last_sync_mode: Option<ServerEndpointSyncMode>,
}
impl ServerEndpointSyncSessionStatus {
    pub fn new() -> Self {
        Self::default()
    }
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
    pub upload_status: Option<ServerEndpointSyncSessionStatus>,
    #[doc = "Sync Session status object."]
    #[serde(rename = "downloadStatus", default, skip_serializing_if = "Option::is_none")]
    pub download_status: Option<ServerEndpointSyncSessionStatus>,
    #[doc = "Sync Session status object."]
    #[serde(rename = "uploadActivity", default, skip_serializing_if = "Option::is_none")]
    pub upload_activity: Option<ServerEndpointSyncActivityStatus>,
    #[doc = "Sync Session status object."]
    #[serde(rename = "downloadActivity", default, skip_serializing_if = "Option::is_none")]
    pub download_activity: Option<ServerEndpointSyncActivityStatus>,
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
    #[doc = "Policy for enabling follow-the-sun business models: link local cache to cloud behavior to pre-populate before local access."]
    #[serde(rename = "localCacheMode", default, skip_serializing_if = "Option::is_none")]
    pub local_cache_mode: Option<LocalCacheMode>,
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
    #[doc = "Error Details object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<StorageSyncInnerErrorDetails>,
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
    #[doc = "Request URI of the given entry."]
    #[serde(rename = "requestUri", default, skip_serializing_if = "Option::is_none")]
    pub request_uri: Option<String>,
    #[doc = "Exception type of the given entry."]
    #[serde(rename = "exceptionType", default, skip_serializing_if = "Option::is_none")]
    pub exception_type: Option<String>,
    #[doc = "HTTP method of the given entry."]
    #[serde(rename = "httpMethod", default, skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    #[doc = "Hashed message of the given entry."]
    #[serde(rename = "hashedMessage", default, skip_serializing_if = "Option::is_none")]
    pub hashed_message: Option<String>,
    #[doc = "HTTP error code of the given entry."]
    #[serde(rename = "httpErrorCode", default, skip_serializing_if = "Option::is_none")]
    pub http_error_code: Option<String>,
}
impl StorageSyncErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error Details object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncInnerErrorDetails {
    #[doc = "Call stack of the error."]
    #[serde(rename = "callStack", default, skip_serializing_if = "Option::is_none")]
    pub call_stack: Option<String>,
    #[doc = "Error message of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Exception of the inner error."]
    #[serde(rename = "innerException", default, skip_serializing_if = "Option::is_none")]
    pub inner_exception: Option<String>,
    #[doc = "Call stack of the inner error."]
    #[serde(rename = "innerExceptionCallStack", default, skip_serializing_if = "Option::is_none")]
    pub inner_exception_call_stack: Option<String>,
}
impl StorageSyncInnerErrorDetails {
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
    #[doc = "StorageSyncService Properties object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageSyncServiceCreateParametersProperties>,
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
#[doc = "StorageSyncService Properties object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncServiceCreateParametersProperties {
    #[doc = "Type of the Incoming Traffic Policy"]
    #[serde(rename = "incomingTrafficPolicy", default, skip_serializing_if = "Option::is_none")]
    pub incoming_traffic_policy: Option<IncomingTrafficPolicy>,
}
impl StorageSyncServiceCreateParametersProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage Sync Service Properties object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncServiceProperties {
    #[doc = "Type of the Incoming Traffic Policy"]
    #[serde(rename = "incomingTrafficPolicy", default, skip_serializing_if = "Option::is_none")]
    pub incoming_traffic_policy: Option<IncomingTrafficPolicy>,
    #[doc = "Storage Sync service status."]
    #[serde(rename = "storageSyncServiceStatus", default, skip_serializing_if = "Option::is_none")]
    pub storage_sync_service_status: Option<i64>,
    #[doc = "Storage Sync service Uid"]
    #[serde(rename = "storageSyncServiceUid", default, skip_serializing_if = "Option::is_none")]
    pub storage_sync_service_uid: Option<String>,
    #[doc = "StorageSyncService Provisioning State"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "StorageSyncService lastWorkflowId"]
    #[serde(rename = "lastWorkflowId", default, skip_serializing_if = "Option::is_none")]
    pub last_workflow_id: Option<String>,
    #[doc = "Resource Last Operation Name"]
    #[serde(rename = "lastOperationName", default, skip_serializing_if = "Option::is_none")]
    pub last_operation_name: Option<String>,
    #[doc = "List of private endpoint connection associated with the specified storage sync service"]
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
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
    #[doc = "StorageSyncService Properties object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageSyncServiceUpdateProperties>,
}
impl StorageSyncServiceUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "StorageSyncService Properties object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncServiceUpdateProperties {
    #[doc = "Type of the Incoming Traffic Policy"]
    #[serde(rename = "incomingTrafficPolicy", default, skip_serializing_if = "Option::is_none")]
    pub incoming_traffic_policy: Option<IncomingTrafficPolicy>,
}
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
#[serde(remote = "WorkflowStatus")]
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
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for WorkflowStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for WorkflowStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for WorkflowStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Active => serializer.serialize_unit_variant("WorkflowStatus", 0u32, "active"),
            Self::Expired => serializer.serialize_unit_variant("WorkflowStatus", 1u32, "expired"),
            Self::Succeeded => serializer.serialize_unit_variant("WorkflowStatus", 2u32, "succeeded"),
            Self::Aborted => serializer.serialize_unit_variant("WorkflowStatus", 3u32, "aborted"),
            Self::Failed => serializer.serialize_unit_variant("WorkflowStatus", 4u32, "failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
