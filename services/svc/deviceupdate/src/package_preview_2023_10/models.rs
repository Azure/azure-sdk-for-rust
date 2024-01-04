#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
pub type ArrayOfStrings = Vec<String>;
#[doc = "Rollback policy for deployment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudInitiatedRollbackPolicy {
    #[doc = "Update information."]
    pub update: UpdateInfo,
    #[doc = "Failure conditions to initiate rollback policy"]
    pub failure: CloudInitiatedRollbackPolicyFailure,
}
impl CloudInitiatedRollbackPolicy {
    pub fn new(update: UpdateInfo, failure: CloudInitiatedRollbackPolicyFailure) -> Self {
        Self { update, failure }
    }
}
#[doc = "Failure conditions to initiate rollback policy"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CloudInitiatedRollbackPolicyFailure {
    #[doc = "Percentage of devices that failed."]
    #[serde(rename = "devicesFailedPercentage")]
    pub devices_failed_percentage: i64,
    #[doc = "Number of devices that failed."]
    #[serde(rename = "devicesFailedCount")]
    pub devices_failed_count: i64,
}
impl CloudInitiatedRollbackPolicyFailure {
    pub fn new(devices_failed_percentage: i64, devices_failed_count: i64) -> Self {
        Self {
            devices_failed_percentage,
            devices_failed_count,
        }
    }
}
#[doc = "Key-value pairs representing update compatibility information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Compatibility {}
impl Compatibility {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Device Update agent contract model."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContractModel {
    #[doc = "The Device Update agent contract model Id of the device class. This is also used to calculate the device class Id."]
    pub id: String,
    #[doc = "The Device Update agent contract model name of the device class. Intended to be a more readable form of the contract model Id."]
    pub name: String,
}
impl ContractModel {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }
}
#[doc = "The list of counts of each limited resource with both current usage and overall quota."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Counters {
    #[doc = "A counter with both usage and quota information."]
    #[serde(rename = "deviceCount", default, skip_serializing_if = "Option::is_none")]
    pub device_count: Option<UsageQuotaCounter>,
    #[doc = "A counter with both usage and quota information."]
    #[serde(rename = "deviceClassCount", default, skip_serializing_if = "Option::is_none")]
    pub device_class_count: Option<UsageQuotaCounter>,
    #[doc = "A counter with both usage and quota information."]
    #[serde(rename = "deviceGroupCount", default, skip_serializing_if = "Option::is_none")]
    pub device_group_count: Option<UsageQuotaCounter>,
    #[doc = "A counter with both usage and quota information."]
    #[serde(rename = "activeDeploymentCount", default, skip_serializing_if = "Option::is_none")]
    pub active_deployment_count: Option<UsageQuotaCounter>,
    #[doc = "A counter with both usage and quota information."]
    #[serde(rename = "deploymentCount", default, skip_serializing_if = "Option::is_none")]
    pub deployment_count: Option<UsageQuotaCounter>,
}
impl Counters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Deployment {
    #[doc = "The caller-provided deployment identifier. This cannot be longer than 73 characters, must be all lower-case, and cannot contain '&', '^', '[', ']', '{', '}', '|', '<', '>', forward slash, backslash, or double quote. The Updates view in the Azure Portal IoT Hub resource generates a GUID for deploymentId when you create a deployment."]
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,
    #[doc = "The deployment start datetime."]
    #[serde(rename = "startDateTime", with = "azure_core::date::rfc3339")]
    pub start_date_time: time::OffsetDateTime,
    #[doc = "Update information."]
    pub update: UpdateInfo,
    #[doc = "The group identity for the devices the deployment is intended to update."]
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[doc = "The device class subgroups the deployment is compatible with and subgroup deployments have been created for. This is not provided by the caller during CreateOrUpdateDeployment but is automatically determined by Device Update"]
    #[serde(
        rename = "deviceClassSubgroups",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub device_class_subgroups: Vec<String>,
    #[doc = "Boolean flag indicating whether the deployment was canceled."]
    #[serde(rename = "isCanceled", default, skip_serializing_if = "Option::is_none")]
    pub is_canceled: Option<bool>,
    #[doc = "Boolean flag indicating whether the deployment has been retried."]
    #[serde(rename = "isRetried", default, skip_serializing_if = "Option::is_none")]
    pub is_retried: Option<bool>,
    #[doc = "Rollback policy for deployment"]
    #[serde(rename = "rollbackPolicy", default, skip_serializing_if = "Option::is_none")]
    pub rollback_policy: Option<CloudInitiatedRollbackPolicy>,
    #[doc = "Boolean flag indicating whether the deployment is a rollback deployment."]
    #[serde(rename = "isCloudInitiatedRollback", default, skip_serializing_if = "Option::is_none")]
    pub is_cloud_initiated_rollback: Option<bool>,
}
impl Deployment {
    pub fn new(deployment_id: String, start_date_time: time::OffsetDateTime, update: UpdateInfo, group_id: String) -> Self {
        Self {
            deployment_id,
            start_date_time,
            update,
            group_id,
            device_class_subgroups: Vec::new(),
            is_canceled: None,
            is_retried: None,
            rollback_policy: None,
            is_cloud_initiated_rollback: None,
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
    #[doc = "Deployment state."]
    #[serde(rename = "deviceState", default, skip_serializing_if = "Option::is_none")]
    pub device_state: Option<DeviceDeploymentState>,
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
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DeploymentDeviceStatesList {
    pub fn new(value: Vec<DeploymentDeviceState>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Deployment order by."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeploymentOrderBy {
    #[doc = "The deployment start datetime."]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<time::OffsetDateTime>,
}
impl DeploymentOrderBy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Deployment state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DeploymentState")]
pub enum DeploymentState {
    Active,
    ActiveWithSubgroupFailures,
    Failed,
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
            Self::ActiveWithSubgroupFailures => serializer.serialize_unit_variant("DeploymentState", 1u32, "ActiveWithSubgroupFailures"),
            Self::Failed => serializer.serialize_unit_variant("DeploymentState", 2u32, "Failed"),
            Self::Inactive => serializer.serialize_unit_variant("DeploymentState", 3u32, "Inactive"),
            Self::Canceled => serializer.serialize_unit_variant("DeploymentState", 4u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Deployment status metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentStatus {
    #[doc = "The group identity"]
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[doc = "Deployment state."]
    #[serde(rename = "deploymentState")]
    pub deployment_state: DeploymentState,
    #[doc = "Error details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
    #[doc = "The collection of device class subgroup status objects"]
    #[serde(rename = "subgroupStatus")]
    pub subgroup_status: Vec<DeviceClassSubgroupDeploymentStatus>,
}
impl DeploymentStatus {
    pub fn new(group_id: String, deployment_state: DeploymentState, subgroup_status: Vec<DeviceClassSubgroupDeploymentStatus>) -> Self {
        Self {
            group_id,
            deployment_state,
            error: None,
            subgroup_status,
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
        self.next_link.clone().filter(|value| !value.is_empty())
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
    #[doc = "Device group identity."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "Update information."]
    #[serde(rename = "lastAttemptedUpdate", default, skip_serializing_if = "Option::is_none")]
    pub last_attempted_update: Option<UpdateInfo>,
    #[doc = "Deployment state."]
    #[serde(rename = "deploymentStatus", default, skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<DeviceDeploymentState>,
    #[doc = "Update information."]
    #[serde(rename = "installedUpdate", default, skip_serializing_if = "Option::is_none")]
    pub installed_update: Option<UpdateInfo>,
    #[doc = "Boolean flag indicating whether the latest update (the best compatible update for the device's device class and group) is installed on the device"]
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
    pub fn new(device_id: String, device_class_id: String, on_latest_update: bool) -> Self {
        Self {
            device_id,
            module_id: None,
            device_class_id,
            group_id: None,
            last_attempted_update: None,
            deployment_status: None,
            installed_update: None,
            on_latest_update,
            last_deployment_id: None,
            last_install_result: None,
        }
    }
}
#[doc = "Device class metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceClass {
    #[doc = "The device class identifier. This is generated from the model Id and the compat properties reported by the device update agent in the Device Update PnP interface in IoT Hub. It is a hex-encoded SHA1 hash."]
    #[serde(rename = "deviceClassId")]
    pub device_class_id: String,
    #[doc = "The device class friendly name. This can be updated by callers after the device class has been automatically created."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[doc = "The device class properties that are used to calculate the device class Id"]
    #[serde(rename = "deviceClassProperties")]
    pub device_class_properties: DeviceClassProperties,
    #[doc = "Update information."]
    #[serde(rename = "bestCompatibleUpdate", default, skip_serializing_if = "Option::is_none")]
    pub best_compatible_update: Option<UpdateInfo>,
}
impl DeviceClass {
    pub fn new(device_class_id: String, device_class_properties: DeviceClassProperties) -> Self {
        Self {
            device_class_id,
            friendly_name: None,
            device_class_properties,
            best_compatible_update: None,
        }
    }
}
#[doc = "Device class filter. Filters device classes based on device class friendly name"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceClassFilter {
    #[doc = "The friendly name of the device class to use in the filter"]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
impl DeviceClassFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The device class properties that are used to calculate the device class Id"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceClassProperties {
    #[doc = "The Device Update agent contract model."]
    #[serde(rename = "contractModel", default, skip_serializing_if = "Option::is_none")]
    pub contract_model: Option<ContractModel>,
    #[doc = "The compat properties of the device class. This object can be thought of as a set of key-value pairs where the key is the name of the compatibility property and the value is the value of the compatibility property. There will always be at least 1 compat property"]
    #[serde(rename = "compatProperties")]
    pub compat_properties: serde_json::Value,
}
impl DeviceClassProperties {
    pub fn new(compat_properties: serde_json::Value) -> Self {
        Self {
            contract_model: None,
            compat_properties,
        }
    }
}
#[doc = "Device class subgroup details. A device class subgroup is a subset of devices in a group that share the same device class id."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceClassSubgroup {
    #[doc = "Device class subgroup identity. This is generated from the model Id and the compat properties reported by the device update agent in the Device Update PnP interface in IoT Hub. It is a hex-encoded SHA1 hash."]
    #[serde(rename = "deviceClassId")]
    pub device_class_id: String,
    #[doc = "Group identity."]
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[doc = "Date and time when the device class subgroup was created."]
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[doc = "The number of devices in the device class subgroup."]
    #[serde(rename = "deviceCount", default, skip_serializing_if = "Option::is_none")]
    pub device_count: Option<i64>,
    #[doc = "The active deployment Id for the device class subgroup."]
    #[serde(rename = "deploymentId", default, skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
}
impl DeviceClassSubgroup {
    pub fn new(device_class_id: String, group_id: String, created_date_time: String) -> Self {
        Self {
            device_class_id,
            group_id,
            created_date_time,
            device_count: None,
            deployment_id: None,
        }
    }
}
#[doc = "Device class subgroup deployment state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DeviceClassSubgroupDeploymentState")]
pub enum DeviceClassSubgroupDeploymentState {
    Active,
    Failed,
    Inactive,
    Canceled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DeviceClassSubgroupDeploymentState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DeviceClassSubgroupDeploymentState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DeviceClassSubgroupDeploymentState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Active => serializer.serialize_unit_variant("DeviceClassSubgroupDeploymentState", 0u32, "Active"),
            Self::Failed => serializer.serialize_unit_variant("DeviceClassSubgroupDeploymentState", 1u32, "Failed"),
            Self::Inactive => serializer.serialize_unit_variant("DeviceClassSubgroupDeploymentState", 2u32, "Inactive"),
            Self::Canceled => serializer.serialize_unit_variant("DeviceClassSubgroupDeploymentState", 3u32, "Canceled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Device class subgroup deployment status metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceClassSubgroupDeploymentStatus {
    #[doc = "The group identity"]
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[doc = "The device class subgroup identity"]
    #[serde(rename = "deviceClassId")]
    pub device_class_id: String,
    #[doc = "Device class subgroup deployment state."]
    #[serde(rename = "deploymentState")]
    pub deployment_state: DeviceClassSubgroupDeploymentState,
    #[doc = "Error details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
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
impl DeviceClassSubgroupDeploymentStatus {
    pub fn new(group_id: String, device_class_id: String, deployment_state: DeviceClassSubgroupDeploymentState) -> Self {
        Self {
            group_id,
            device_class_id,
            deployment_state,
            error: None,
            total_devices: None,
            devices_in_progress_count: None,
            devices_completed_failed_count: None,
            devices_completed_succeeded_count: None,
            devices_canceled_count: None,
        }
    }
}
#[doc = "Device class subgroups filter. Filters device class subgroups based on device class compat property names and values"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceClassSubgroupFilter {
    #[doc = "The name of the compat property to use in the filter. E.g. compatProperties/manufacturer"]
    #[serde(rename = "compatPropertyName", default, skip_serializing_if = "Option::is_none")]
    pub compat_property_name: Option<String>,
    #[doc = "The value the compat property to use in the filter. E.g. Contoso"]
    #[serde(rename = "compatPropertyValue", default, skip_serializing_if = "Option::is_none")]
    pub compat_property_value: Option<String>,
}
impl DeviceClassSubgroupFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Device class subgroup, update information, and the number of devices for which the update is applicable."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceClassSubgroupUpdatableDevices {
    #[doc = "The group Id"]
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[doc = "The device class subgroup's device class Id"]
    #[serde(rename = "deviceClassId")]
    pub device_class_id: String,
    #[doc = "Update information."]
    pub update: UpdateInfo,
    #[doc = "Total number of devices for which the update is applicable."]
    #[serde(rename = "deviceCount")]
    pub device_count: i64,
}
impl DeviceClassSubgroupUpdatableDevices {
    pub fn new(group_id: String, device_class_id: String, update: UpdateInfo, device_count: i64) -> Self {
        Self {
            group_id,
            device_class_id,
            update,
            device_count,
        }
    }
}
#[doc = "The list of updatable devices for a device class subgroup."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceClassSubgroupUpdatableDevicesList {
    #[doc = "The collection of pageable items."]
    pub value: Vec<DeviceClassSubgroupUpdatableDevices>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeviceClassSubgroupUpdatableDevicesList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DeviceClassSubgroupUpdatableDevicesList {
    pub fn new(value: Vec<DeviceClassSubgroupUpdatableDevices>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The list of device class subgroups within a group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceClassSubgroupsList {
    #[doc = "The collection of pageable items."]
    pub value: Vec<DeviceClassSubgroup>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeviceClassSubgroupsList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DeviceClassSubgroupsList {
    pub fn new(value: Vec<DeviceClassSubgroup>) -> Self {
        Self { value, next_link: None }
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
        self.next_link.clone().filter(|value| !value.is_empty())
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
    Canceled,
    Failed,
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
            Self::Canceled => serializer.serialize_unit_variant("DeviceDeploymentState", 2u32, "Canceled"),
            Self::Failed => serializer.serialize_unit_variant("DeviceDeploymentState", 3u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Device filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceFilter {
    #[doc = "Device group identity."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "Device class identity."]
    #[serde(rename = "deviceClassId", default, skip_serializing_if = "Option::is_none")]
    pub device_class_id: Option<String>,
    #[doc = "Deployment state."]
    #[serde(rename = "deploymentStatus", default, skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<DeviceDeploymentState>,
}
impl DeviceFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Device Health"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceHealth {
    #[doc = "Device id"]
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[doc = "Module id"]
    #[serde(rename = "moduleId", default, skip_serializing_if = "Option::is_none")]
    pub module_id: Option<String>,
    #[doc = "Device health states"]
    pub state: DeviceHealthState,
    #[doc = "Digital twin model Id"]
    #[serde(rename = "digitalTwinModelId", default, skip_serializing_if = "Option::is_none")]
    pub digital_twin_model_id: Option<String>,
    #[doc = "Array of health checks and their results"]
    #[serde(rename = "healthChecks")]
    pub health_checks: Vec<HealthCheck>,
}
impl DeviceHealth {
    pub fn new(device_id: String, state: DeviceHealthState, health_checks: Vec<HealthCheck>) -> Self {
        Self {
            device_id,
            module_id: None,
            state,
            digital_twin_model_id: None,
            health_checks,
        }
    }
}
#[doc = "Device health filter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DeviceHealthFilter {
    #[doc = "Device health states"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<DeviceHealthState>,
    #[doc = "Device Id"]
    #[serde(rename = "deviceId", default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[doc = "Module Id"]
    #[serde(rename = "moduleId", default, skip_serializing_if = "Option::is_none")]
    pub module_id: Option<String>,
}
impl DeviceHealthFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Array of Device Health, with server paging support."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceHealthList {
    #[doc = "The collection of pageable items."]
    pub value: Vec<DeviceHealth>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DeviceHealthList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DeviceHealthList {
    pub fn new(value: Vec<DeviceHealth>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Device health states"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DeviceHealthState")]
pub enum DeviceHealthState {
    #[serde(rename = "healthy")]
    Healthy,
    #[serde(rename = "unhealthy")]
    Unhealthy,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DeviceHealthState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DeviceHealthState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DeviceHealthState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Healthy => serializer.serialize_unit_variant("DeviceHealthState", 0u32, "healthy"),
            Self::Unhealthy => serializer.serialize_unit_variant("DeviceHealthState", 1u32, "unhealthy"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
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
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DeviceOperationsList {
    pub fn new(value: Vec<DeviceOperation>) -> Self {
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
        self.next_link.clone().filter(|value| !value.is_empty())
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    #[doc = "Group identity. This is created from the value of the ADUGroup tag in the Iot Hub's device/module twin or $default for devices with no tag."]
    #[serde(rename = "groupId")]
    pub group_id: String,
    #[doc = "Supported group types."]
    #[serde(rename = "groupType")]
    pub group_type: GroupType,
    #[doc = "Date and time when the update was created."]
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[doc = "The number of devices in the group."]
    #[serde(rename = "deviceCount", default, skip_serializing_if = "Option::is_none")]
    pub device_count: Option<i64>,
    #[doc = "The count of subgroups with new updates available."]
    #[serde(rename = "subgroupsWithNewUpdatesAvailableCount", default, skip_serializing_if = "Option::is_none")]
    pub subgroups_with_new_updates_available_count: Option<i64>,
    #[doc = "The count of subgroups with updates in progress."]
    #[serde(rename = "subgroupsWithUpdatesInProgressCount", default, skip_serializing_if = "Option::is_none")]
    pub subgroups_with_updates_in_progress_count: Option<i64>,
    #[doc = "The count of subgroups with devices on the latest update."]
    #[serde(rename = "subgroupsWithOnLatestUpdateCount", default, skip_serializing_if = "Option::is_none")]
    pub subgroups_with_on_latest_update_count: Option<i64>,
    #[doc = "The active deployment Ids for the group"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub deployments: Vec<String>,
}
impl Group {
    pub fn new(group_id: String, group_type: GroupType, created_date_time: String) -> Self {
        Self {
            group_id,
            group_type,
            created_date_time,
            device_count: None,
            subgroups_with_new_updates_available_count: None,
            subgroups_with_updates_in_progress_count: None,
            subgroups_with_on_latest_update_count: None,
            deployments: Vec::new(),
        }
    }
}
#[doc = "Groups order by."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupOrderBy {
    #[doc = "The group Id."]
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[doc = "The group device count."]
    #[serde(rename = "deviceCount", default, skip_serializing_if = "Option::is_none")]
    pub device_count: Option<String>,
    #[doc = "The group created date."]
    #[serde(rename = "createdDate", default, skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[doc = "The number of subgroups with new updates available"]
    #[serde(rename = "subgroupsWithNewUpdatesAvailableCount", default, skip_serializing_if = "Option::is_none")]
    pub subgroups_with_new_updates_available_count: Option<String>,
    #[doc = "The number of subgroups with updates in progress"]
    #[serde(rename = "subgroupsWithUpdatesInProgressCount", default, skip_serializing_if = "Option::is_none")]
    pub subgroups_with_updates_in_progress_count: Option<String>,
    #[doc = "The number of subgroups with devices on the latest update"]
    #[serde(rename = "subgroupsOnLatestUpdateCount", default, skip_serializing_if = "Option::is_none")]
    pub subgroups_on_latest_update_count: Option<String>,
}
impl GroupOrderBy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Supported group types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "GroupType")]
pub enum GroupType {
    IoTHubTag,
    DefaultNoTag,
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
            Self::DefaultNoTag => serializer.serialize_unit_variant("GroupType", 1u32, "DefaultNoTag"),
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
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl GroupsList {
    pub fn new(value: Vec<Group>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Health check"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthCheck {
    #[doc = "Health check name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Health check result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<HealthCheckResult>,
}
impl HealthCheck {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Health check result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HealthCheckResult")]
pub enum HealthCheckResult {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "userError")]
    UserError,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HealthCheckResult {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HealthCheckResult {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HealthCheckResult {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Success => serializer.serialize_unit_variant("HealthCheckResult", 0u32, "success"),
            Self::UserError => serializer.serialize_unit_variant("HealthCheckResult", 1u32, "userError"),
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    pub inner_error: Option<Box<InnerError>>,
}
impl InnerError {
    pub fn new(code: String) -> Self {
        Self {
            code,
            message: None,
            error_detail: None,
            inner_error: None,
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
    #[serde(
        rename = "stepResults",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
#[doc = "The list of limits of how many of each resource are currently in use and how many are allowed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LimitsResponse {
    #[doc = "The list of counts of each limited resource with both current usage and overall quota."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counters: Option<Counters>,
}
impl LimitsResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Diagnostics request body"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogCollection {
    #[doc = "The log collection id."]
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
impl LogCollection {
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
#[doc = "The list of log collections with detailed status, with server paging support."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogCollectionDetailedStatusList {
    #[doc = "The collection of pageable items."]
    pub value: Vec<LogCollectionOperationDetailedStatus>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl LogCollectionDetailedStatusList {
    pub fn new(value: Vec<LogCollectionOperationDetailedStatus>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The list of log collections with server paging support."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogCollectionList {
    #[doc = "The collection of pageable items."]
    pub value: Vec<LogCollection>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LogCollectionList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl LogCollectionList {
    pub fn new(value: Vec<LogCollection>) -> Self {
        Self { value, next_link: None }
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
    #[serde(
        rename = "deviceStatus",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
            Self::NotStarted => serializer.serialize_unit_variant("OperationStatus", 0u32, "NotStarted"),
            Self::Running => serializer.serialize_unit_variant("OperationStatus", 1u32, "Running"),
            Self::Succeeded => serializer.serialize_unit_variant("OperationStatus", 2u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("OperationStatus", 3u32, "Failed"),
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
#[doc = "Device Class JSON Merge Patch request body"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchBody {
    #[doc = "The device class friendly name. Friendly name can be 1-100 characters, alphanumeric, dot, and dash."]
    #[serde(rename = "friendlyName")]
    pub friendly_name: String,
}
impl PatchBody {
    pub fn new(friendly_name: String) -> Self {
        Self { friendly_name }
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
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    #[doc = "Update information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update: Option<UpdateInfo>,
    #[doc = "Step description."]
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
            update: None,
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
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl StringsList {
    pub fn new(value: Vec<String>) -> Self {
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
    #[serde(
        rename = "referencedBy",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
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
    #[serde(flatten)]
    pub update_file_base: UpdateFileBase,
    #[doc = "File identity, generated by server at import time."]
    #[serde(rename = "fileId")]
    pub file_id: String,
    #[doc = "Optional related files metadata used together DownloadHandler metadata to download payload file."]
    #[serde(
        rename = "relatedFiles",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub related_files: Vec<UpdateFileBase>,
    #[doc = "Download handler for utilizing related files to download payload file."]
    #[serde(rename = "downloadHandler", default, skip_serializing_if = "Option::is_none")]
    pub download_handler: Option<UpdateFileDownloadHandler>,
    #[doc = "File ETag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl UpdateFile {
    pub fn new(update_file_base: UpdateFileBase, file_id: String) -> Self {
        Self {
            update_file_base,
            file_id,
            related_files: Vec::new(),
            download_handler: None,
            etag: None,
        }
    }
}
#[doc = "Update file basic metadata."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateFileBase {
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
    #[doc = "Optional file properties (not consumed by service but pass-through to device)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl UpdateFileBase {
    pub fn new(file_name: String, size_in_bytes: i64, hashes: serde_json::Value) -> Self {
        Self {
            file_name,
            size_in_bytes,
            hashes,
            mime_type: None,
            scan_result: None,
            scan_details: None,
            properties: None,
        }
    }
}
#[doc = "Download handler for utilizing related files to download payload file."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateFileDownloadHandler {
    #[doc = "Download handler identifier."]
    pub id: String,
}
impl UpdateFileDownloadHandler {
    pub fn new(id: String) -> Self {
        Self { id }
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
#[doc = "Update information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateInfo {
    #[doc = "Update identifier."]
    #[serde(rename = "updateId")]
    pub update_id: UpdateId,
    #[doc = "Update description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Friendly update name."]
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
impl UpdateInfo {
    pub fn new(update_id: UpdateId) -> Self {
        Self {
            update_id,
            description: None,
            friendly_name: None,
        }
    }
}
#[doc = "List of update information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateInfoList {
    #[doc = "The collection of pageable items."]
    pub value: Vec<UpdateInfo>,
    #[doc = "The link to the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for UpdateInfoList {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl UpdateInfoList {
    pub fn new(value: Vec<UpdateInfo>) -> Self {
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
        self.next_link.clone().filter(|value| !value.is_empty())
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
    #[doc = "Update information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update: Option<UpdateInfo>,
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
            update: None,
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
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl UpdateOperationsList {
    pub fn new(value: Vec<UpdateOperation>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "A counter with both usage and quota information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageQuotaCounter {
    #[doc = "The current number of the resource that exist"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<i32>,
    #[doc = "The maximum number of the resource that can be created"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quota: Option<i32>,
}
impl UsageQuotaCounter {
    pub fn new() -> Self {
        Self::default()
    }
}
