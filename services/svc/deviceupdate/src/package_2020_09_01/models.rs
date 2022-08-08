#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
pub type ArrayOfStrings = Vec<String>;
#[doc = "Update compatibility information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Compatibility {
    #[doc = "The manufacturer of device the update is compatible with."]
    #[serde(rename = "deviceManufacturer")]
    pub device_manufacturer: String,
    #[doc = "The model of device the update is compatible with."]
    #[serde(rename = "deviceModel")]
    pub device_model: String,
}
impl Compatibility {
    pub fn new(device_manufacturer: String, device_model: String) -> Self {
        Self {
            device_manufacturer,
            device_model,
        }
    }
}
#[doc = "Deployment metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Deployment {
    #[doc = "Gets or sets the deployment identifier."]
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,
    #[doc = "Supported deployment types."]
    #[serde(rename = "deploymentType")]
    pub deployment_type: DeploymentType,
    #[doc = "Gets or sets the device class identifier."]
    #[serde(rename = "deviceClassId", default, skip_serializing_if = "Option::is_none")]
    pub device_class_id: Option<String>,
    #[doc = "Gets or sets the Deployment start datetime."]
    #[serde(rename = "startDateTime", with = "azure_core::date::rfc3339")]
    pub start_date_time: time::OffsetDateTime,
    #[doc = "Supported deployment group types."]
    #[serde(rename = "deviceGroupType")]
    pub device_group_type: DeviceGroupType,
    #[doc = "Gets or sets the device group definition."]
    #[serde(rename = "deviceGroupDefinition")]
    pub device_group_definition: Vec<String>,
    #[doc = "Update identifier."]
    #[serde(rename = "updateId")]
    pub update_id: UpdateId,
    #[doc = "Boolean flag indicating whether the deployment was canceled."]
    #[serde(rename = "isCanceled", default, skip_serializing_if = "Option::is_none")]
    pub is_canceled: Option<bool>,
    #[doc = "Boolean flag indicating whether the deployment has been retried."]
    #[serde(rename = "isRetried", default, skip_serializing_if = "Option::is_none")]
    pub is_retried: Option<bool>,
    #[doc = "Boolean flag indicating whether the deployment was completed."]
    #[serde(rename = "isCompleted", default, skip_serializing_if = "Option::is_none")]
    pub is_completed: Option<bool>,
}
impl Deployment {
    pub fn new(
        deployment_id: String,
        deployment_type: DeploymentType,
        start_date_time: time::OffsetDateTime,
        device_group_type: DeviceGroupType,
        device_group_definition: Vec<String>,
        update_id: UpdateId,
    ) -> Self {
        Self {
            deployment_id,
            deployment_type,
            device_class_id: None,
            start_date_time,
            device_group_type,
            device_group_definition,
            update_id,
            is_canceled: None,
            is_retried: None,
            is_completed: None,
        }
    }
}
#[doc = "Deployment device status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentDeviceState {
    #[doc = "Device identity."]
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[doc = "The number of times this deployment has been retried on this device."]
    #[serde(rename = "retryCount")]
    pub retry_count: i32,
    #[doc = "Boolean flag indicating whether this device is in a newer deployment and can no longer retry this deployment."]
    #[serde(rename = "movedOnToNewDeployment")]
    pub moved_on_to_new_deployment: bool,
    #[doc = "Deployment state."]
    #[serde(rename = "deviceState")]
    pub device_state: DeviceDeploymentState,
}
impl DeploymentDeviceState {
    pub fn new(device_id: String, retry_count: i32, moved_on_to_new_deployment: bool, device_state: DeviceDeploymentState) -> Self {
        Self {
            device_id,
            retry_count,
            moved_on_to_new_deployment,
            device_state,
        }
    }
}
#[doc = "Deployment device state filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentDeviceStatesFilter {
    #[doc = "Device Identifier."]
    #[serde(rename = "deviceId", default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[doc = "The deployment device state."]
    #[serde(rename = "deviceState", default, skip_serializing_if = "Option::is_none")]
    pub device_state: Option<DeviceState>,
}
impl DeploymentDeviceStatesFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentFilter {
    #[doc = "Update provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Update name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Update version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl DeploymentFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DeploymentState")]
pub enum DeploymentState {
    Active,
    Superseded,
    Canceled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DeploymentState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DeploymentState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DeploymentState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Active => serializer.serialize_unit_variant("DeploymentState", 0u32, "Active"),
            Self::Superseded => serializer.serialize_unit_variant("DeploymentState", 1u32, "Superseded"),
            Self::Canceled => serializer.serialize_unit_variant("DeploymentState", 2u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Deployment status metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentStatus {
    #[doc = "Deployment state."]
    #[serde(rename = "deploymentState")]
    pub deployment_state: DeploymentState,
    #[doc = "Gets or sets the total number of devices in the deployment."]
    #[serde(rename = "totalDevices", default, skip_serializing_if = "Option::is_none")]
    pub total_devices: Option<i32>,
    #[doc = "Gets or sets the number of incompatible devices in the deployment."]
    #[serde(rename = "devicesIncompatibleCount", default, skip_serializing_if = "Option::is_none")]
    pub devices_incompatible_count: Option<i32>,
    #[doc = "Gets or sets the number of devices that are currently in deployment."]
    #[serde(rename = "devicesInProgressCount", default, skip_serializing_if = "Option::is_none")]
    pub devices_in_progress_count: Option<i32>,
    #[doc = "Gets or sets the number of devices that have completed deployment with a failure."]
    #[serde(rename = "devicesCompletedFailedCount", default, skip_serializing_if = "Option::is_none")]
    pub devices_completed_failed_count: Option<i32>,
    #[doc = "Gets or sets the number of devices which have successfully completed deployment."]
    #[serde(rename = "devicesCompletedSucceededCount", default, skip_serializing_if = "Option::is_none")]
    pub devices_completed_succeeded_count: Option<i32>,
    #[doc = "Gets or sets the number of devices which have had their deployment canceled."]
    #[serde(rename = "devicesCanceledCount", default, skip_serializing_if = "Option::is_none")]
    pub devices_canceled_count: Option<i32>,
}
impl DeploymentStatus {
    pub fn new(deployment_state: DeploymentState) -> Self {
        Self {
            deployment_state,
            total_devices: None,
            devices_incompatible_count: None,
            devices_in_progress_count: None,
            devices_completed_failed_count: None,
            devices_completed_succeeded_count: None,
            devices_canceled_count: None,
        }
    }
}
#[doc = "Supported deployment types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DeploymentType")]
pub enum DeploymentType {
    Complete,
    Download,
    Install,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DeploymentType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DeploymentType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DeploymentType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Complete => serializer.serialize_unit_variant("DeploymentType", 0u32, "Complete"),
            Self::Download => serializer.serialize_unit_variant("DeploymentType", 1u32, "Download"),
            Self::Install => serializer.serialize_unit_variant("DeploymentType", 2u32, "Install"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Device metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Device {
    #[doc = "Device identity."]
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[doc = "Device class identity."]
    #[serde(rename = "deviceClassId")]
    pub device_class_id: String,
    #[doc = "Device manufacturer."]
    pub manufacturer: String,
    #[doc = "Device model."]
    pub model: String,
    #[doc = "Device group identity."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "Update identifier."]
    #[serde(rename = "lastAttemptedUpdateId", default, skip_serializing_if = "Option::is_none")]
    pub last_attempted_update_id: Option<UpdateId>,
    #[doc = "Deployment state."]
    #[serde(rename = "deploymentStatus", default, skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<DeviceDeploymentState>,
    #[doc = "Update identifier."]
    #[serde(rename = "installedUpdateId", default, skip_serializing_if = "Option::is_none")]
    pub installed_update_id: Option<UpdateId>,
    #[doc = "Boolean flag indicating whether the latest update is installed on the device"]
    #[serde(rename = "onLatestUpdate")]
    pub on_latest_update: bool,
    #[doc = "The deployment identifier for the last deployment to the device"]
    #[serde(rename = "lastDeploymentId", default, skip_serializing_if = "Option::is_none")]
    pub last_deployment_id: Option<String>,
}
impl Device {
    pub fn new(device_id: String, device_class_id: String, manufacturer: String, model: String, on_latest_update: bool) -> Self {
        Self {
            device_id,
            device_class_id,
            manufacturer,
            model,
            group_id: None,
            last_attempted_update_id: None,
            deployment_status: None,
            installed_update_id: None,
            on_latest_update,
            last_deployment_id: None,
        }
    }
}
#[doc = "Device class metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceClass {
    #[doc = "The device class identifier."]
    #[serde(rename = "deviceClassId")]
    pub device_class_id: String,
    #[doc = "Device manufacturer"]
    pub manufacturer: String,
    #[doc = "Device model."]
    pub model: String,
    #[doc = "Update identifier."]
    #[serde(rename = "bestCompatibleUpdateId")]
    pub best_compatible_update_id: UpdateId,
}
impl DeviceClass {
    pub fn new(device_class_id: String, manufacturer: String, model: String, best_compatible_update_id: UpdateId) -> Self {
        Self {
            device_class_id,
            manufacturer,
            model,
            best_compatible_update_id,
        }
    }
}
#[doc = "Deployment state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DeviceDeploymentState")]
pub enum DeviceDeploymentState {
    Succeeded,
    InProgress,
    Failed,
    Canceled,
    Incompatible,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DeviceDeploymentState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DeviceDeploymentState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DeviceDeploymentState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("DeviceDeploymentState", 0u32, "Succeeded"),
            Self::InProgress => serializer.serialize_unit_variant("DeviceDeploymentState", 1u32, "InProgress"),
            Self::Failed => serializer.serialize_unit_variant("DeviceDeploymentState", 2u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("DeviceDeploymentState", 3u32, "Canceled"),
            Self::Incompatible => serializer.serialize_unit_variant("DeviceDeploymentState", 4u32, "Incompatible"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Operation status filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceFilter {
    #[doc = "Device group identifier."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}
impl DeviceFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Supported deployment group types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DeviceGroupType")]
pub enum DeviceGroupType {
    All,
    Devices,
    DeviceGroupDefinitions,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DeviceGroupType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DeviceGroupType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DeviceGroupType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::All => serializer.serialize_unit_variant("DeviceGroupType", 0u32, "All"),
            Self::Devices => serializer.serialize_unit_variant("DeviceGroupType", 1u32, "Devices"),
            Self::DeviceGroupDefinitions => serializer.serialize_unit_variant("DeviceGroupType", 2u32, "DeviceGroupDefinitions"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The deployment device state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DeviceState")]
pub enum DeviceState {
    NotStarted,
    Incompatible,
    AlreadyInDeployment,
    Canceled,
    InProgress,
    Failed,
    Succeeded,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DeviceState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DeviceState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DeviceState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotStarted => serializer.serialize_unit_variant("DeviceState", 0u32, "NotStarted"),
            Self::Incompatible => serializer.serialize_unit_variant("DeviceState", 1u32, "Incompatible"),
            Self::AlreadyInDeployment => serializer.serialize_unit_variant("DeviceState", 2u32, "AlreadyInDeployment"),
            Self::Canceled => serializer.serialize_unit_variant("DeviceState", 3u32, "Canceled"),
            Self::InProgress => serializer.serialize_unit_variant("DeviceState", 4u32, "InProgress"),
            Self::Failed => serializer.serialize_unit_variant("DeviceState", 5u32, "Failed"),
            Self::Succeeded => serializer.serialize_unit_variant("DeviceState", 6u32, "Succeeded"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Device tag properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceTag {
    #[doc = "Tag name."]
    #[serde(rename = "tagName")]
    pub tag_name: String,
    #[doc = "Number of devices with this tag."]
    #[serde(rename = "deviceCount")]
    pub device_count: i64,
}
impl DeviceTag {
    pub fn new(tag_name: String, device_count: i64) -> Self {
        Self { tag_name, device_count }
    }
}
#[doc = "Error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
    #[doc = "Server defined error code."]
    pub code: String,
    #[doc = "A human-readable representation of the error."]
    pub message: String,
    #[doc = "The target of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "An array of errors that led to the reported error."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<Error>,
    #[doc = "An object containing more specific information than the current object about the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<InnerError>,
    #[doc = "Date and time in UTC when the error occurred."]
    #[serde(rename = "occurredDateTime", with = "azure_core::date::rfc3339::option")]
    pub occurred_date_time: Option<time::OffsetDateTime>,
}
impl Error {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            details: Vec::new(),
            innererror: None,
            occurred_date_time: None,
        }
    }
}
#[doc = "Update file metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct File {
    #[doc = "File identity, generated by server at import time."]
    #[serde(rename = "fileId")]
    pub file_id: String,
    #[doc = "File name."]
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[doc = "File size in number of bytes."]
    #[serde(rename = "sizeInBytes")]
    pub size_in_bytes: i64,
    #[doc = "Mapping of hashing algorithm to base64 encoded hash values."]
    pub hashes: serde_json::Value,
    #[doc = "File MIME type."]
    #[serde(rename = "mimeType", default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[doc = "File ETag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl File {
    pub fn new(file_id: String, file_name: String, size_in_bytes: i64, hashes: serde_json::Value) -> Self {
        Self {
            file_id,
            file_name,
            size_in_bytes,
            hashes,
            mime_type: None,
            etag: None,
        }
    }
}
#[doc = "Metadata describing an update file."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileImportMetadata {
    #[doc = "Update file name as specified inside import manifest."]
    pub filename: String,
    #[doc = "Azure Blob location from which the update file can be downloaded by Device Update for IoT Hub. This is typically a read-only SAS-protected blob URL with an expiration set to at least 4 hours."]
    pub url: String,
}
impl FileImportMetadata {
    pub fn new(filename: String, url: String) -> Self {
        Self { filename, url }
    }
}
#[doc = "Group details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Group {
    #[doc = "Group identity."]
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[doc = "Supported group types."]
    #[serde(rename = "groupType")]
    pub group_type: GroupType,
    #[doc = "IoT Hub tags."]
    pub tags: Vec<String>,
    #[doc = "Date and time when the update was created."]
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[doc = "The number of devices in the group."]
    #[serde(rename = "deviceCount", default, skip_serializing_if = "Option::is_none")]
    pub device_count: Option<i64>,
}
impl Group {
    pub fn new(group_id: String, group_type: GroupType, tags: Vec<String>, created_date_time: String) -> Self {
        Self {
            group_id,
            group_type,
            tags,
            created_date_time,
            device_count: None,
        }
    }
}
#[doc = "Group best updates filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupBestUpdatesFilter {
    #[doc = "Update provider."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Update name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Update version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl GroupBestUpdatesFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Supported group types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "GroupType")]
pub enum GroupType {
    IoTHubTag,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for GroupType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for GroupType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for GroupType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::IoTHubTag => serializer.serialize_unit_variant("GroupType", 0u32, "IoTHubTag"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Metadata describing the import manifest, a document which describes the files and other metadata about an update version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportManifestMetadata {
    #[doc = "Azure Blob location from which the import manifest can be downloaded by Device Update for IoT Hub. This is typically a read-only SAS-protected blob URL with an expiration set to at least 4 hours."]
    pub url: String,
    #[doc = "File size in number of bytes."]
    #[serde(rename = "sizeInBytes")]
    pub size_in_bytes: i64,
    #[doc = "A JSON object containing the hash(es) of the file. At least SHA256 hash is required. This object can be thought of as a set of key-value pairs where the key is the hash algorithm, and the value is the hash of the file calculated using that algorithm."]
    pub hashes: serde_json::Value,
}
impl ImportManifestMetadata {
    pub fn new(url: String, size_in_bytes: i64, hashes: serde_json::Value) -> Self {
        Self {
            url,
            size_in_bytes,
            hashes,
        }
    }
}
#[doc = "Import update input metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportUpdateInput {
    #[doc = "Metadata describing the import manifest, a document which describes the files and other metadata about an update version."]
    #[serde(rename = "importManifest")]
    pub import_manifest: ImportManifestMetadata,
    #[doc = "One or more update file properties like filename and source URL."]
    pub files: Vec<FileImportMetadata>,
}
impl ImportUpdateInput {
    pub fn new(import_manifest: ImportManifestMetadata, files: Vec<FileImportMetadata>) -> Self {
        Self { import_manifest, files }
    }
}
#[doc = "An object containing more specific information than the current object about the error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InnerError {
    #[doc = "A more specific error code than what was provided by the containing error."]
    pub code: String,
    #[doc = "A human-readable representation of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The internal error or exception message."]
    #[serde(rename = "errorDetail", default, skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<String>,
    #[doc = "An object containing more specific information than the current object about the error."]
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Box<Option<InnerError>>,
}
impl InnerError {
    pub fn new(code: String) -> Self {
        Self {
            code,
            message: None,
            error_detail: None,
            inner_error: Box::new(None),
        }
    }
}
#[doc = "Operation metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[doc = "Operation Id."]
    #[serde(rename = "operationId")]
    pub operation_id: String,
    #[doc = "Operation status."]
    pub status: OperationStatus,
    #[doc = "Update identifier."]
    #[serde(rename = "updateId", default, skip_serializing_if = "Option::is_none")]
    pub update_id: Option<UpdateId>,
    #[doc = "Location of the imported update when operation is successful."]
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[doc = "Error details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
    #[doc = "Operation correlation identity that can used by Microsoft Support for troubleshooting."]
    #[serde(rename = "traceId", default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[doc = "Date and time in UTC when the operation status was last updated."]
    #[serde(rename = "lastActionDateTime", with = "azure_core::date::rfc3339")]
    pub last_action_date_time: time::OffsetDateTime,
    #[doc = "Date and time in UTC when the operation was created."]
    #[serde(rename = "createdDateTime", with = "azure_core::date::rfc3339")]
    pub created_date_time: time::OffsetDateTime,
    #[doc = "Operation ETag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl Operation {
    pub fn new(
        operation_id: String,
        status: OperationStatus,
        last_action_date_time: time::OffsetDateTime,
        created_date_time: time::OffsetDateTime,
    ) -> Self {
        Self {
            operation_id,
            status,
            update_id: None,
            resource_location: None,
            error: None,
            trace_id: None,
            last_action_date_time,
            created_date_time,
            etag: None,
        }
    }
}
#[doc = "Operation status filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationFilter {
    #[doc = "Operation status filter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OperationFilterStatus>,
}
impl OperationFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation status filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OperationFilterStatus")]
pub enum OperationFilterStatus {
    Running,
    NotStarted,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OperationFilterStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OperationFilterStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OperationFilterStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Running => serializer.serialize_unit_variant("OperationFilterStatus", 0u32, "Running"),
            Self::NotStarted => serializer.serialize_unit_variant("OperationFilterStatus", 1u32, "NotStarted"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Operation status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OperationStatus")]
pub enum OperationStatus {
    Undefined,
    NotStarted,
    Running,
    Succeeded,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OperationStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OperationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OperationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Undefined => serializer.serialize_unit_variant("OperationStatus", 0u32, "Undefined"),
            Self::NotStarted => serializer.serialize_unit_variant("OperationStatus", 1u32, "NotStarted"),
            Self::Running => serializer.serialize_unit_variant("OperationStatus", 2u32, "Running"),
            Self::Succeeded => serializer.serialize_unit_variant("OperationStatus", 3u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("OperationStatus", 4u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The list of deployment device states."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PageableListOfDeploymentDeviceStates {
    #[doc = "The collection of pageable items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DeploymentDeviceState>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PageableListOfDeploymentDeviceStates {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PageableListOfDeploymentDeviceStates {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of deployments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PageableListOfDeployments {
    #[doc = "The collection of pageable items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Deployment>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PageableListOfDeployments {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PageableListOfDeployments {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of device classes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PageableListOfDeviceClasses {
    #[doc = "The collection of pageable items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DeviceClass>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PageableListOfDeviceClasses {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PageableListOfDeviceClasses {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of device tags."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PageableListOfDeviceTags {
    #[doc = "The collection of pageable items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DeviceTag>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PageableListOfDeviceTags {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PageableListOfDeviceTags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of devices."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PageableListOfDevices {
    #[doc = "The collection of pageable items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Device>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PageableListOfDevices {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PageableListOfDevices {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PageableListOfGroups {
    #[doc = "The collection of pageable items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Group>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PageableListOfGroups {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PageableListOfGroups {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of operations with server paging support."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PageableListOfOperations {
    #[doc = "The collection of pageable items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PageableListOfOperations {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PageableListOfOperations {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of strings with server paging support."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PageableListOfStrings {
    #[doc = "The collection of pageable items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<String>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PageableListOfStrings {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PageableListOfStrings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of updatable devices."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PageableListOfUpdatableDevices {
    #[doc = "The collection of pageable items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UpdatableDevices>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PageableListOfUpdatableDevices {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PageableListOfUpdatableDevices {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of update identities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PageableListOfUpdateIds {
    #[doc = "The collection of pageable items."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UpdateId>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PageableListOfUpdateIds {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl PageableListOfUpdateIds {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Update identifier and the number of devices for which the update is applicable."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatableDevices {
    #[doc = "Update identifier."]
    #[serde(rename = "updateId")]
    pub update_id: UpdateId,
    #[doc = "Total number of devices for which the update is applicable."]
    #[serde(rename = "deviceCount")]
    pub device_count: i64,
}
impl UpdatableDevices {
    pub fn new(update_id: UpdateId, device_count: i64) -> Self {
        Self { update_id, device_count }
    }
}
#[doc = "Update metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Update {
    #[doc = "Update identifier."]
    #[serde(rename = "updateId")]
    pub update_id: UpdateId,
    #[doc = "Update type."]
    #[serde(rename = "updateType")]
    pub update_type: String,
    #[doc = "String interpreted by Device Update client to determine if the update is installed on the device."]
    #[serde(rename = "installedCriteria")]
    pub installed_criteria: String,
    #[doc = "List of update compatibility information."]
    pub compatibility: Vec<Compatibility>,
    #[doc = "Schema version of manifest used to import the update."]
    #[serde(rename = "manifestVersion")]
    pub manifest_version: String,
    #[doc = "Date and time in UTC when the update was imported."]
    #[serde(rename = "importedDateTime", with = "azure_core::date::rfc3339")]
    pub imported_date_time: time::OffsetDateTime,
    #[doc = "Date and time in UTC when the update was created."]
    #[serde(rename = "createdDateTime", with = "azure_core::date::rfc3339")]
    pub created_date_time: time::OffsetDateTime,
    #[doc = "Update ETag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl Update {
    pub fn new(
        update_id: UpdateId,
        update_type: String,
        installed_criteria: String,
        compatibility: Vec<Compatibility>,
        manifest_version: String,
        imported_date_time: time::OffsetDateTime,
        created_date_time: time::OffsetDateTime,
    ) -> Self {
        Self {
            update_id,
            update_type,
            installed_criteria,
            compatibility,
            manifest_version,
            imported_date_time,
            created_date_time,
            etag: None,
        }
    }
}
#[doc = "Update compliance information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateCompliance {
    #[doc = "Total number of devices."]
    #[serde(rename = "totalDeviceCount")]
    pub total_device_count: i64,
    #[doc = "Number of devices on the latest update."]
    #[serde(rename = "onLatestUpdateDeviceCount")]
    pub on_latest_update_device_count: i64,
    #[doc = "Number of devices with a newer update available."]
    #[serde(rename = "newUpdatesAvailableDeviceCount")]
    pub new_updates_available_device_count: i64,
    #[doc = "Number of devices with update in-progress."]
    #[serde(rename = "updatesInProgressDeviceCount")]
    pub updates_in_progress_device_count: i64,
}
impl UpdateCompliance {
    pub fn new(
        total_device_count: i64,
        on_latest_update_device_count: i64,
        new_updates_available_device_count: i64,
        updates_in_progress_device_count: i64,
    ) -> Self {
        Self {
            total_device_count,
            on_latest_update_device_count,
            new_updates_available_device_count,
            updates_in_progress_device_count,
        }
    }
}
#[doc = "Update identifier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateId {
    #[doc = "Update provider."]
    pub provider: String,
    #[doc = "Update name."]
    pub name: String,
    #[doc = "Update version."]
    pub version: String,
}
impl UpdateId {
    pub fn new(provider: String, name: String, version: String) -> Self {
        Self { provider, name, version }
    }
}
