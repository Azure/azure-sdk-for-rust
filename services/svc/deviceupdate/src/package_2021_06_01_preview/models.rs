#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
pub type ArrayOfStrings = Vec<String>;
#[doc = "Key-value pairs representing update compatibility information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Compatibility {}
impl Compatibility {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Deployment {
    #[doc = "The deployment identifier."]
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,
    #[doc = "The deployment start datetime."]
    #[serde(rename = "startDateTime", with = "azure_core::date::rfc3339")]
    pub start_date_time: time::OffsetDateTime,
    #[doc = "Update identifier."]
    #[serde(rename = "updateId")]
    pub update_id: UpdateId,
    #[doc = "The group identity"]
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[doc = "Boolean flag indicating whether the deployment was canceled."]
    #[serde(rename = "isCanceled", default, skip_serializing_if = "Option::is_none")]
    pub is_canceled: Option<bool>,
    #[doc = "Boolean flag indicating whether the deployment has been retried."]
    #[serde(rename = "isRetried", default, skip_serializing_if = "Option::is_none")]
    pub is_retried: Option<bool>,
}
impl Deployment {
    pub fn new(deployment_id: String, start_date_time: time::OffsetDateTime, update_id: UpdateId, group_id: String) -> Self {
        Self {
            deployment_id,
            start_date_time,
            update_id,
            group_id,
            is_canceled: None,
            is_retried: None,
        }
    }
}
#[doc = "Deployment device status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentDeviceState {
    #[doc = "Device identity."]
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[doc = "Device module identity."]
    #[serde(rename = "moduleId", default, skip_serializing_if = "Option::is_none")]
    pub module_id: Option<String>,
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
            module_id: None,
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
    #[doc = "Device module Identifier."]
    #[serde(rename = "moduleId", default, skip_serializing_if = "Option::is_none")]
    pub module_id: Option<String>,
    #[doc = "The deployment device state."]
    #[serde(rename = "deviceState", default, skip_serializing_if = "Option::is_none")]
    pub device_state: Option<DeviceState>,
}
impl DeploymentDeviceStatesFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of deployment device states."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentDeviceStatesList {
    #[doc = "The collection of pageable items."]
    pub value: Vec<DeploymentDeviceState>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeploymentDeviceStatesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeploymentDeviceStatesList {
    pub fn new(value: Vec<DeploymentDeviceState>) -> Self {
        Self { value, next_link: None }
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
    Inactive,
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
            Self::Inactive => serializer.serialize_unit_variant("DeploymentState", 1u32, "Inactive"),
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
    #[doc = "The total number of devices in the deployment."]
    #[serde(rename = "totalDevices", default, skip_serializing_if = "Option::is_none")]
    pub total_devices: Option<i32>,
    #[doc = "The number of devices that are currently in deployment."]
    #[serde(rename = "devicesInProgressCount", default, skip_serializing_if = "Option::is_none")]
    pub devices_in_progress_count: Option<i32>,
    #[doc = "The number of devices that have completed deployment with a failure."]
    #[serde(rename = "devicesCompletedFailedCount", default, skip_serializing_if = "Option::is_none")]
    pub devices_completed_failed_count: Option<i32>,
    #[doc = "The number of devices which have successfully completed deployment."]
    #[serde(rename = "devicesCompletedSucceededCount", default, skip_serializing_if = "Option::is_none")]
    pub devices_completed_succeeded_count: Option<i32>,
    #[doc = "The number of devices which have had their deployment canceled."]
    #[serde(rename = "devicesCanceledCount", default, skip_serializing_if = "Option::is_none")]
    pub devices_canceled_count: Option<i32>,
}
impl DeploymentStatus {
    pub fn new(deployment_state: DeploymentState) -> Self {
        Self {
            deployment_state,
            total_devices: None,
            devices_in_progress_count: None,
            devices_completed_failed_count: None,
            devices_completed_succeeded_count: None,
            devices_canceled_count: None,
        }
    }
}
#[doc = "The list of deployments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentsList {
    #[doc = "The collection of pageable items."]
    pub value: Vec<Deployment>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeploymentsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeploymentsList {
    pub fn new(value: Vec<Deployment>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Device metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Device {
    #[doc = "Device identity."]
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[doc = "Device module identity."]
    #[serde(rename = "moduleId", default, skip_serializing_if = "Option::is_none")]
    pub module_id: Option<String>,
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
    #[doc = "The install result of an update and any step results under it."]
    #[serde(rename = "lastInstallResult", default, skip_serializing_if = "Option::is_none")]
    pub last_install_result: Option<InstallResult>,
}
impl Device {
    pub fn new(device_id: String, device_class_id: String, manufacturer: String, model: String, on_latest_update: bool) -> Self {
        Self {
            device_id,
            module_id: None,
            device_class_id,
            manufacturer,
            model,
            group_id: None,
            last_attempted_update_id: None,
            deployment_status: None,
            installed_update_id: None,
            on_latest_update,
            last_deployment_id: None,
            last_install_result: None,
        }
    }
}
#[doc = "Device class metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceClass {
    #[doc = "The device class identifier."]
    #[serde(rename = "deviceClassId")]
    pub device_class_id: String,
    #[doc = "The compat properties of the device class. This object can be thought of as a set of key-value pairs where the key is the name of the compatibility property and the value is the value of the compatibility property. There will always be at least 1 compat property"]
    #[serde(rename = "compatProperties")]
    pub compat_properties: serde_json::Value,
    #[doc = "Update identifier."]
    #[serde(rename = "bestCompatibleUpdateId")]
    pub best_compatible_update_id: UpdateId,
}
impl DeviceClass {
    pub fn new(device_class_id: String, compat_properties: serde_json::Value, best_compatible_update_id: UpdateId) -> Self {
        Self {
            device_class_id,
            compat_properties,
            best_compatible_update_id,
        }
    }
}
#[doc = "The list of device classes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceClassesList {
    #[doc = "The collection of pageable items."]
    pub value: Vec<DeviceClass>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeviceClassesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeviceClassesList {
    pub fn new(value: Vec<DeviceClass>) -> Self {
        Self { value, next_link: None }
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
    #[doc = "Device group identity."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}
impl DeviceFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Operation metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceOperation {
    #[doc = "Operation Id."]
    #[serde(rename = "operationId")]
    pub operation_id: String,
    #[doc = "Operation status."]
    pub status: OperationStatus,
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
impl DeviceOperation {
    pub fn new(
        operation_id: String,
        status: OperationStatus,
        last_action_date_time: time::OffsetDateTime,
        created_date_time: time::OffsetDateTime,
    ) -> Self {
        Self {
            operation_id,
            status,
            error: None,
            trace_id: None,
            last_action_date_time,
            created_date_time,
            etag: None,
        }
    }
}
#[doc = "The list of device operations with server paging support."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceOperationsList {
    #[doc = "The collection of pageable items."]
    pub value: Vec<DeviceOperation>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeviceOperationsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeviceOperationsList {
    pub fn new(value: Vec<DeviceOperation>) -> Self {
        Self { value, next_link: None }
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
#[doc = "The list of device tags."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceTagsList {
    #[doc = "The collection of pageable items."]
    pub value: Vec<DeviceTag>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeviceTagsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DeviceTagsList {
    pub fn new(value: Vec<DeviceTag>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Device Update agent id"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceUpdateAgentId {
    #[doc = "Device Id"]
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[doc = "Module Id"]
    #[serde(rename = "moduleId", default, skip_serializing_if = "Option::is_none")]
    pub module_id: Option<String>,
}
impl DeviceUpdateAgentId {
    pub fn new(device_id: String) -> Self {
        Self {
            device_id,
            module_id: None,
        }
    }
}
#[doc = "The list of devices."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevicesList {
    #[doc = "The collection of pageable items."]
    pub value: Vec<Device>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DevicesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl DevicesList {
    pub fn new(value: Vec<Device>) -> Self {
        Self { value, next_link: None }
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
    #[serde(rename = "occurredDateTime", default, with = "azure_core::date::rfc3339::option")]
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
#[doc = "Common error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[doc = "Error details."]
    pub error: Error,
}
impl azure_core::Continuable for ErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponse {
    pub fn new(error: Error) -> Self {
        Self { error }
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
    #[doc = "The deployment Id for the group."]
    #[serde(rename = "deploymentId", default, skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[doc = "The device class Id for the group."]
    #[serde(rename = "deviceClassId", default, skip_serializing_if = "Option::is_none")]
    pub device_class_id: Option<String>,
}
impl Group {
    pub fn new(group_id: String, group_type: GroupType, tags: Vec<String>, created_date_time: String) -> Self {
        Self {
            group_id,
            group_type,
            tags,
            created_date_time,
            device_count: None,
            deployment_id: None,
            device_class_id: None,
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
    DeviceClassIdAndIoTHubTag,
    InvalidDeviceClassIdAndIoTHubTag,
    DefaultDeviceClassId,
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
            Self::DeviceClassIdAndIoTHubTag => serializer.serialize_unit_variant("GroupType", 0u32, "DeviceClassIdAndIoTHubTag"),
            Self::InvalidDeviceClassIdAndIoTHubTag => {
                serializer.serialize_unit_variant("GroupType", 1u32, "InvalidDeviceClassIdAndIoTHubTag")
            }
            Self::DefaultDeviceClassId => serializer.serialize_unit_variant("GroupType", 2u32, "DefaultDeviceClassId"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The list of groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupsList {
    #[doc = "The collection of pageable items."]
    pub value: Vec<Group>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GroupsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl GroupsList {
    pub fn new(value: Vec<Group>) -> Self {
        Self { value, next_link: None }
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
#[doc = "The device import type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ImportType")]
pub enum ImportType {
    Devices,
    Modules,
    All,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ImportType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ImportType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ImportType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Devices => serializer.serialize_unit_variant("ImportType", 0u32, "Devices"),
            Self::Modules => serializer.serialize_unit_variant("ImportType", 1u32, "Modules"),
            Self::All => serializer.serialize_unit_variant("ImportType", 2u32, "All"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type ImportUpdateInput = Vec<ImportUpdateInputItem>;
#[doc = "Import update input item metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportUpdateInputItem {
    #[doc = "Metadata describing the import manifest, a document which describes the files and other metadata about an update version."]
    #[serde(rename = "importManifest")]
    pub import_manifest: ImportManifestMetadata,
    #[doc = "Friendly update name."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "One or more update file properties like filename and source URL."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<FileImportMetadata>,
}
impl ImportUpdateInputItem {
    pub fn new(import_manifest: ImportManifestMetadata) -> Self {
        Self {
            import_manifest,
            friendly_name: None,
            files: Vec::new(),
        }
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
#[doc = "The install result of an update and any step results under it."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstallResult {
    #[doc = "Install result code."]
    #[serde(rename = "resultCode")]
    pub result_code: i64,
    #[doc = "Install extended result code"]
    #[serde(rename = "extendedResultCode")]
    pub extended_result_code: i64,
    #[doc = "A string containing further details about the install result"]
    #[serde(rename = "resultDetails", default, skip_serializing_if = "Option::is_none")]
    pub result_details: Option<String>,
    #[doc = "Array of step results"]
    #[serde(rename = "stepResults", default, skip_serializing_if = "Vec::is_empty")]
    pub step_results: Vec<StepResult>,
}
impl InstallResult {
    pub fn new(result_code: i64, extended_result_code: i64) -> Self {
        Self {
            result_code,
            extended_result_code,
            result_details: None,
            step_results: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Instructions {
    #[doc = "Collection of installation steps."]
    pub steps: Vec<Step>,
}
impl Instructions {
    pub fn new(steps: Vec<Step>) -> Self {
        Self { steps }
    }
}
#[doc = "Diagnostics request body"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogCollectionOperation {
    #[doc = "The diagnostics operation id."]
    #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[doc = "Array of Device Update agent ids"]
    #[serde(rename = "deviceList")]
    pub device_list: Vec<DeviceUpdateAgentId>,
    #[doc = "Description of the diagnostics operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The timestamp when the operation was created."]
    #[serde(rename = "createdDateTime", default, skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[doc = "A timestamp for when the current state was entered."]
    #[serde(rename = "lastActionDateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_action_date_time: Option<String>,
    #[doc = "Operation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OperationStatusWithoutUndefinedOption>,
}
impl LogCollectionOperation {
    pub fn new(device_list: Vec<DeviceUpdateAgentId>) -> Self {
        Self {
            operation_id: None,
            device_list,
            description: None,
            created_date_time: None,
            last_action_date_time: None,
            status: None,
        }
    }
}
#[doc = "Device diagnostics operation detailed status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogCollectionOperationDetailedStatus {
    #[doc = "The device diagnostics operation id."]
    #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[doc = "The timestamp when the operation was created."]
    #[serde(rename = "createdDateTime", default, skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[doc = "A timestamp for when the current state was entered."]
    #[serde(rename = "lastActionDateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_action_date_time: Option<String>,
    #[doc = "Operation status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OperationStatusWithoutUndefinedOption>,
    #[doc = "Status of the devices in the operation"]
    #[serde(rename = "deviceStatus", default, skip_serializing_if = "Vec::is_empty")]
    pub device_status: Vec<LogCollectionOperationDeviceStatus>,
    #[doc = "Device diagnostics operation description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl LogCollectionOperationDetailedStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list of diagnostics operations with detailed status, with server paging support."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogCollectionOperationDetailedStatusList {
    #[doc = "The collection of pageable items."]
    pub value: Vec<LogCollectionOperationDetailedStatus>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl LogCollectionOperationDetailedStatusList {
    pub fn new(value: Vec<LogCollectionOperationDetailedStatus>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Diagnostics operation device status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogCollectionOperationDeviceStatus {
    #[doc = "Device id"]
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[doc = "Module id."]
    #[serde(rename = "moduleId", default, skip_serializing_if = "Option::is_none")]
    pub module_id: Option<String>,
    #[doc = "Operation status."]
    pub status: OperationStatusWithoutUndefinedOption,
    #[doc = "Log upload result code"]
    #[serde(rename = "resultCode", default, skip_serializing_if = "Option::is_none")]
    pub result_code: Option<String>,
    #[doc = "Log upload extended result code"]
    #[serde(rename = "extendedResultCode", default, skip_serializing_if = "Option::is_none")]
    pub extended_result_code: Option<String>,
    #[doc = "Log upload location"]
    #[serde(rename = "logLocation", default, skip_serializing_if = "Option::is_none")]
    pub log_location: Option<String>,
}
impl LogCollectionOperationDeviceStatus {
    pub fn new(device_id: String, status: OperationStatusWithoutUndefinedOption) -> Self {
        Self {
            device_id,
            module_id: None,
            status,
            result_code: None,
            extended_result_code: None,
            log_location: None,
        }
    }
}
#[doc = "The list of diagnostics operations with server paging support."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogCollectionOperationList {
    #[doc = "The collection of pageable items."]
    pub value: Vec<LogCollectionOperation>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LogCollectionOperationList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl LogCollectionOperationList {
    pub fn new(value: Vec<LogCollectionOperation>) -> Self {
        Self { value, next_link: None }
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
#[doc = "Operation status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "OperationStatusWithoutUndefinedOption")]
pub enum OperationStatusWithoutUndefinedOption {
    NotStarted,
    Running,
    Succeeded,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for OperationStatusWithoutUndefinedOption {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for OperationStatusWithoutUndefinedOption {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for OperationStatusWithoutUndefinedOption {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotStarted => serializer.serialize_unit_variant("OperationStatusWithoutUndefinedOption", 0u32, "NotStarted"),
            Self::Running => serializer.serialize_unit_variant("OperationStatusWithoutUndefinedOption", 1u32, "Running"),
            Self::Succeeded => serializer.serialize_unit_variant("OperationStatusWithoutUndefinedOption", 2u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("OperationStatusWithoutUndefinedOption", 3u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Update install instruction step."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Step {
    #[doc = "Step type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<step::Type>,
    #[doc = "Step description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Identity of handler that will execute this step. Required if step type is inline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handler: Option<String>,
    #[doc = "Parameters to be passed to handler during execution."]
    #[serde(rename = "handlerProperties", default, skip_serializing_if = "Option::is_none")]
    pub handler_properties: Option<serde_json::Value>,
    #[doc = "Collection of file names to be passed to handler during execution. Required if step type is inline."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
    #[doc = "Update identifier."]
    #[serde(rename = "updateId", default, skip_serializing_if = "Option::is_none")]
    pub update_id: Option<UpdateId>,
}
impl Step {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod step {
    use super::*;
    #[doc = "Step type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "inline")]
        Inline,
        #[serde(rename = "reference")]
        Reference,
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
                Self::Inline => serializer.serialize_unit_variant("Type", 0u32, "inline"),
                Self::Reference => serializer.serialize_unit_variant("Type", 1u32, "reference"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    impl Default for Type {
        fn default() -> Self {
            Self::Inline
        }
    }
}
#[doc = "The step result under an update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StepResult {
    #[doc = "Update identifier."]
    #[serde(rename = "updateId", default, skip_serializing_if = "Option::is_none")]
    pub update_id: Option<UpdateId>,
    #[doc = "Step description. It might be null for update steps."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Install result code."]
    #[serde(rename = "resultCode")]
    pub result_code: i64,
    #[doc = "Install extended result code"]
    #[serde(rename = "extendedResultCode")]
    pub extended_result_code: i64,
    #[doc = "A string containing further details about the install result"]
    #[serde(rename = "resultDetails", default, skip_serializing_if = "Option::is_none")]
    pub result_details: Option<String>,
}
impl StepResult {
    pub fn new(result_code: i64, extended_result_code: i64) -> Self {
        Self {
            update_id: None,
            description: None,
            result_code,
            extended_result_code,
            result_details: None,
        }
    }
}
#[doc = "The list of strings with server paging support."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringsList {
    #[doc = "The collection of pageable items."]
    pub value: Vec<String>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for StringsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl StringsList {
    pub fn new(value: Vec<String>) -> Self {
        Self { value, next_link: None }
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
#[doc = "The list of updatable devices."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdatableDevicesList {
    #[doc = "The collection of pageable items."]
    pub value: Vec<UpdatableDevices>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UpdatableDevicesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl UpdatableDevicesList {
    pub fn new(value: Vec<UpdatableDevices>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Update metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Update {
    #[doc = "Update identifier."]
    #[serde(rename = "updateId")]
    pub update_id: UpdateId,
    #[doc = "Update description specified by creator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Friendly update name specified by importer."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "Whether the update can be deployed to a device on its own."]
    #[serde(rename = "isDeployable", default, skip_serializing_if = "Option::is_none")]
    pub is_deployable: Option<bool>,
    #[doc = "Update type. Deprecated in latest import manifest schema."]
    #[serde(rename = "updateType", default, skip_serializing_if = "Option::is_none")]
    pub update_type: Option<String>,
    #[doc = "String interpreted by Device Update client to determine if the update is installed on the device. Deprecated in latest import manifest schema."]
    #[serde(rename = "installedCriteria", default, skip_serializing_if = "Option::is_none")]
    pub installed_criteria: Option<String>,
    #[doc = "List of update compatibility information."]
    pub compatibility: Vec<Compatibility>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<Instructions>,
    #[doc = "List of update identities that reference this update."]
    #[serde(rename = "referencedBy", default, skip_serializing_if = "Vec::is_empty")]
    pub referenced_by: Vec<UpdateId>,
    #[doc = "Update aggregate scan result (calculated from payload file scan results)."]
    #[serde(rename = "scanResult", default, skip_serializing_if = "Option::is_none")]
    pub scan_result: Option<String>,
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
        compatibility: Vec<Compatibility>,
        manifest_version: String,
        imported_date_time: time::OffsetDateTime,
        created_date_time: time::OffsetDateTime,
    ) -> Self {
        Self {
            update_id,
            description: None,
            friendly_name: None,
            is_deployable: None,
            update_type: None,
            installed_criteria: None,
            compatibility,
            instructions: None,
            referenced_by: Vec::new(),
            scan_result: None,
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
#[doc = "Update file metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateFile {
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
    #[doc = "Anti-malware scan result."]
    #[serde(rename = "scanResult", default, skip_serializing_if = "Option::is_none")]
    pub scan_result: Option<String>,
    #[doc = "Anti-malware scan details."]
    #[serde(rename = "scanDetails", default, skip_serializing_if = "Option::is_none")]
    pub scan_details: Option<String>,
    #[doc = "File ETag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl UpdateFile {
    pub fn new(file_id: String, file_name: String, size_in_bytes: i64, hashes: serde_json::Value) -> Self {
        Self {
            file_id,
            file_name,
            size_in_bytes,
            hashes,
            mime_type: None,
            scan_result: None,
            scan_details: None,
            etag: None,
        }
    }
}
#[doc = "Update filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateFilter {
    #[doc = "Update isDeployable property."]
    #[serde(rename = "isDeployable", default, skip_serializing_if = "Option::is_none")]
    pub is_deployable: Option<bool>,
}
impl UpdateFilter {
    pub fn new() -> Self {
        Self::default()
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
#[doc = "The list of update identities."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateIdsList {
    #[doc = "The collection of pageable items."]
    pub value: Vec<UpdateId>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UpdateIdsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl UpdateIdsList {
    pub fn new(value: Vec<UpdateId>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The list of updates."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateList {
    #[doc = "The collection of pageable items."]
    pub value: Vec<Update>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UpdateList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl UpdateList {
    pub fn new(value: Vec<Update>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Operation metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateOperation {
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
impl UpdateOperation {
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
#[doc = "The list of operations with server paging support."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateOperationsList {
    #[doc = "The collection of pageable items."]
    pub value: Vec<UpdateOperation>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UpdateOperationsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone()
    }
}
impl UpdateOperationsList {
    pub fn new(value: Vec<UpdateOperation>) -> Self {
        Self { value, next_link: None }
    }
}
